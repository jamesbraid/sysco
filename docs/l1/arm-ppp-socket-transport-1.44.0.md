# ARM PPP, packet and socket transport in 1.44.0

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
The computed transfers in `IRFmtAddr` (`ldrls` at 0xab350) and `SocketMakeInetConnection` (`ldrls` at 0xabfb0)
leave unresolved selector edges. Access and call tables cover only known edges.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/ppp-socket-transport-facts.txt) retain all tool sections.

## `PPPRead`

- `PPPRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae30c to 0xaec8c.
  Established. Included word reads address `r0 +0x28` and `r0 +0x2c` (`ldr` at 0xae390, `ldr` at 0xae39c).
  Equality `[sp-45]:1 == 0x7e` takes `0xae55c`, while inequality falls through (`beq` at 0xae554).
  The unnamed `0xadad0` call carries `0xffff`, `([[sp-36]] + [sp-96])`, `[sp-76]`, `sp-52` (`bl` at 0xae6d0).
  An included byte assignment writes `0x55` to `([sp-64] + [sp-56]) +0x0` (`strb` at 0xaebb0).
  PPP receive record field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae31c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xae320 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xae324 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xae328 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xae330 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xae338 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xae340 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xae348 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xae350 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0xae358 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0xae360 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xae368 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xae370 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xae378 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xae380 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xae388 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xae38c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae390 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xae394 | store | 4 | `sp -0x60` | `[r0+40]` |
| `ldr` at 0xae398 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae39c | load | 4 | `r0 +0x2c` | `-` |
| `str` at 0xae3a0 | store | 4 | `sp -0x64` | `[r0+44]` |
| `ldr` at 0xae3a4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xae3b4 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xae3b8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xae3c0 | store | 4 | `sp +0x0` | `?` |
| `ldr` at 0xae3c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae3c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xae3d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae3d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xae3dc | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae3e4 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xae3ec | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xae3f4 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xae3fc | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xae400 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xae40c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae410 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xae41c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae420 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xae424 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae430 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xae43c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xae444 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xae448 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xae44c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xae458 | store | 4 | `sp -0x38` | `ret(0x195d4)` |
| `ldr` at 0xae45c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae464 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xae468 | load | 1 | `(unknown)` | `-` |
| `str` at 0xae470 | store | 4 | `(unknown)` | `?` |
| `strb` at 0xae474 | store | 1 | `sp -0x2d` | `?` |
| `ldr` at 0xae478 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae47c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae480 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xae488 | store | 4 | `[sp-24] +0xc` | `[[sp-24]+12]-1` |
| `ldr` at 0xae490 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae498 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xae4a0 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xae4a4 | store | 1 | `([sp-64] + [sp-56]) +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xae4ac | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xae4b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae4b4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xae4c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xae4c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae4d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae4d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae4dc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xae4e0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xae4e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae4ec | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xae4f0 | store | 4 | `sp -0x44` | `[[sp-24]+12]` |
| `str` at 0xae4fc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xae500 | load | 4 | `sp -0x5c` | `-` |
| `ldrb` at 0xae50c | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0xae51c | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xae520 | store | 4 | `sp -0x70` | `[sp-45]:1` |
| `ldr` at 0xae524 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xae530 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xae53c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xae54c | load | 4 | `sp -0x70` | `-` |
| `str` at 0xae560 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xae564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae568 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0xae574 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xae580 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae584 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae588 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae594 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xae598 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xae59c | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xae5a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae5a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xae5ac | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xae5b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae5b4 | load | 4 | `abs 0xaec6c` | `-` |
| `ldr` at 0xae5bc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae5c8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae5d4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae5e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae5e4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae5e8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae5f4 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xae5f8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xae5fc | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xae600 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae604 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xae60c | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xae610 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae614 | load | 4 | `abs 0xaec70` | `-` |
| `ldr` at 0xae61c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae628 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae634 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xae640 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xae648 | store | 4 | `sp -0x48` | `[sp-72]-1` |
| `ldrb` at 0xae64c | load | 1 | `[sp-72] -0x1` | `-` |
| `strb` at 0xae650 | store | 1 | `sp -0x33` | `([[sp-72]-1]:1 & 0xff)` |
| `ldr` at 0xae654 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xae65c | store | 4 | `sp -0x48` | `[sp-72]-2` |
| `ldrb` at 0xae660 | load | 1 | `[sp-72] -0x2` | `-` |
| `strb` at 0xae664 | store | 1 | `sp -0x34` | `([[sp-72]-2]:1 & 0xff)` |
| `ldr` at 0xae668 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xae670 | store | 4 | `sp -0x4c` | `[sp-76]-2` |
| `ldr` at 0xae678 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0xae684 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xae688 | store | 1 | `sp -0x33` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xae68c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xae694 | store | 4 | `sp -0x48` | `[sp-72]-1` |
| `ldrb` at 0xae698 | load | 1 | `[sp-72] -0x1` | `-` |
| `strb` at 0xae69c | store | 1 | `sp -0x34` | `([[sp-72]-1]:1 & 0xff)` |
| `ldr` at 0xae6a0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xae6a8 | store | 4 | `sp -0x4c` | `[sp-76]-1` |
| `ldr` at 0xae6ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae6b0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae6b4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xae6c0 | load | 4 | `abs 0xaec74` | `-` |
| `ldr` at 0xae6c8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae6e0 | load | 4 | `abs 0xaec78` | `-` |
| `str` at 0xae6f0 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xae6f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae6fc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae700 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae70c | store | 4 | `sp -0x7c` | `sp-52` |
| `ldr` at 0xae710 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xae714 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xae718 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae71c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xae724 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xae728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae72c | load | 4 | `abs 0xaec7c` | `-` |
| `ldr` at 0xae734 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae73c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xae748 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xae750 | store | 4 | `[sp-24] +0x30` | `0x1` |
| `ldr` at 0xae754 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae758 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae75c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae764 | store | 4 | `sp -0x48` | `([[sp-36]] + [sp-96])` |
| `str` at 0xae76c | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xae774 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xae77c | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xae784 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xae78c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae790 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xae7a0 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xae7a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae7ac | load | 4 | `[sp-24] +0x24` | `-` |
| `ldr` at 0xae7b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xae7c0 | store | 4 | `[sp-24] +0x30` | `0x0` |
| `ldr` at 0xae7c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae7c8 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae7cc | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae7d8 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xae7dc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xae7e0 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xae7e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae7e8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xae7f0 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xae7f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae7f8 | load | 4 | `abs 0xaec80` | `-` |
| `ldr` at 0xae800 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae808 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xae814 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xae81c | load | 4 | `abs 0xaec80` | `-` |
| `str` at 0xae820 | store | 4 | `sp -0x6c` | `0xfffffd40` |
| `ldr` at 0xae828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae82c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0xae838 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xae84c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xae850 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xae854 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xae858 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae85c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xae864 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xae868 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xae880 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xae888 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0xae894 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xae89c | store | 1 | `sp -0x2d` | `?` |
| `str` at 0xae8a4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xae8a8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xae8ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae8b8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xae8c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae8c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae8cc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae8d0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xae8dc | store | 4 | `[sp-36] +0x0` | `ret(0x195d4)` |
| `ldr` at 0xae8e0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae8ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xae8f4 | store | 4 | `[sp-24] +0x30` | `0x0` |
| `str` at 0xae8fc | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xae900 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xae904 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xae908 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae90c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xae914 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xae918 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae91c | load | 4 | `abs 0xaec84` | `-` |
| `ldr` at 0xae92c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xae938 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xae940 | load | 4 | `abs 0xaec84` | `-` |
| `str` at 0xae944 | store | 4 | `sp -0x6c` | `0xfffffd3e` |
| `ldr` at 0xae94c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae950 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae954 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xae95c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae964 | store | 4 | `sp -0x48` | `(([[sp-36]] + [sp]) + [sp-96])` |
| `ldr` at 0xae968 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xae96c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae974 | store | 4 | `sp +0x0` | `?` |
| `str` at 0xae97c | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xae984 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xae990 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae994 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xae998 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xae9a4 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xae9a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xae9ac | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xae9b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae9b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xae9bc | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xae9c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae9cc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xae9d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xae9dc | store | 4 | `[sp-24] +0x30` | `0x0` |
| `str` at 0xae9e4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xae9ec | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xae9f4 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xae9fc | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xaea04 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xaea0c | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xaea18 | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0xaea20 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xaea24 | store | 1 | `[sp-80] +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xaea2c | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xaea30 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xaea38 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0xaea44 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xaea4c | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xaea50 | store | 1 | `[sp-72] +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xaea58 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0xaea5c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xaea64 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0xaea6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaea78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaea84 | load | 4 | `abs 0xaec88` | `-` |
| `ldr` at 0xaea88 | load | 4 | `abs 0x133ee4` | `-` |
| `ldr` at 0xaea94 | load | 4 | `abs 0xaec88` | `-` |
| `ldr` at 0xaea98 | load | 4 | `abs 0x133ee4` | `-` |
| `ldr` at 0xaeaa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeaa4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaeaa8 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xaeaac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeab4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeab8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaeabc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xaeac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeac4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaeac8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaead0 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xaeadc | store | 4 | `(unknown)` | `ret(unknown)` |
| `ldr` at 0xaeae0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaeaec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeaf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaeaf4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xaeaf8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xaeb04 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xaeb08 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xaeb0c | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xaeb10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xaeb14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xaeb1c | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xaeb20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeb24 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xaeb2c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xaeb34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaeb3c | store | 4 | `[sp-24] +0x30` | `0x0` |
| `ldr` at 0xaeb40 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xaeb4c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xaeb54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaeb58 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xaeb5c | store | 4 | `sp -0x6c` | `[[sp-24]+12]` |
| `ldr` at 0xaeb64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xaeb6c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xaeb78 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xaeb80 | store | 4 | `sp -0x3c` | `[sp-60]+256` |
| `ldr` at 0xaeb84 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xaeb88 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xaeb94 | store | 4 | `sp -0x38` | `ret(0x195d4)` |
| `ldr` at 0xaeb9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xaeba4 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xaebb0 | store | 1 | `([sp-64] + [sp-56]) +0x0` | `0x55` |
| `str` at 0xaebb8 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xaebc0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xaebc8 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xaebd4 | store | 1 | `([sp-64]+1 + [sp-56]) +0x0` | `?` |
| `str` at 0xaebdc | store | 4 | `sp -0x40` | `[sp-64]+2` |
| `str` at 0xaebe4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xaebe8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xaebf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaebf8 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xaec08 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xaec10 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xaec18 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xaec20 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xaec28 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xaec30 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xaec34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaec44 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xaec50 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xaec58 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xaec5c | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xaec60 | load | 4 | `sp -0x6c` | `-` |
| `ldm` at 0xaec68 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xaec68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaec68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaec68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae450 | `(unknown callee) 0x195d4` | `?` | `?` | `?` | `?` |
| `bl` at 0xae4e4 | `(unknown callee) 0x19610` | `([sp-56] + [sp-64]+1)+1` | `[[sp-24]+8]` | `[[sp-24]+12]` | `([sp-56] + [sp-64]+1)+1` |
| `bl` at 0xae5c0 | `(unknown callee) 0xadc40` | `[sp-24]` | `0xfffffd41` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xae620 | `(unknown callee) 0xadc40` | `[sp-24]` | `0xfffffd42` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xae6d0 | `(unknown callee) 0xadad0` | `0xffff` | `([[sp-36]] + [sp-96])` | `[sp-76]` | `sp-52` |
| `bl` at 0xae738 | `(unknown callee) 0xadc40` | `[sp-24]` | `0xfffffd44` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xae804 | `(unknown callee) 0xadc40` | `[sp-24]` | `0xfffffd40` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xae818 | `(unknown callee) 0x1970c` | `[sp-56]` | `?` | `?` | `[sp-56]` |
| `bl` at 0xae878 | `(unknown callee) 0xadc40` | `[sp-24]` | `?` | `sp-45` | `0x1` |
| `bl` at 0xae8d4 | `(unknown callee) 0x195d4` | `[[sp-36]]` | `[sp-100]` | `[sp]` | `[sp-36]` |
| `bl` at 0xae928 | `(unknown callee) 0xadc40` | `[sp-24]` | `0xfffffd3e` | `0x0` | `0x0` |
| `bl` at 0xae93c | `(unknown callee) 0x1970c` | `[sp-56]` | `?` | `?` | `[sp-56]` |
| `bl` at 0xae9d0 | `(unknown callee) 0xadc40` | `[sp-24]` | `?` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xaea9c | `(unknown callee) 0x20974` | `[0x133ee4]` | `?` | `?` | `0x133ee4` |
| `mov` at 0xaead8 | `(unknown callee) (unknown target)` | `[sp-32]` | `?` | `[[sp-24]+4]` | `?` |
| `bl` at 0xaeb30 | `(unknown callee) 0xadc40` | `[sp-24]` | `[[sp-24]+12]` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xaeb50 | `(unknown callee) 0x1970c` | `[sp-56]` | `?` | `[sp-24]` | `[sp-56]` |
| `bl` at 0xaeb8c | `(unknown callee) 0x195d4` | `[sp-56]` | `[sp-60]+256` | `[sp-64]+3` | `[sp-60]+256` |
| `bl` at 0xaec54 | `(unknown callee) 0x1970c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xae3ac | `[sp] >= 0x0 (signed, 32-bit)` | `[sp] < 0x0 (signed, 32-bit)` |
| `beq` at 0xae3d0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xae408 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xae418 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blo` at 0xae438 | `C == 0` | `not (C == 0)` |
| `beq` at 0xae4bc | `[[sp-24]+12] == 0x0` | `[[sp-24]+12] != 0x0` |
| `beq` at 0xae508 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0xae514 | `[sp-45]:1 == 0x7e` | `[sp-45]:1 != 0x7e` |
| `beq` at 0xae52c | `[sp-45]:1 == 0x7d` | `[sp-45]:1 != 0x7d` |
| `bgt` at 0xae538 | `[sp-45]:1 > 0x7d (signed, 32-bit)` | `[sp-45]:1 <= 0x7d (signed, 32-bit)` |
| `beq` at 0xae544 | `[sp-45]:1 == 0xd` | `[sp-45]:1 != 0xd` |
| `beq` at 0xae554 | `[sp-45]:1 == 0x7e` | `[sp-45]:1 != 0x7e` |
| `beq` at 0xae570 | `[[sp-24]+48] == 0x0` | `[[sp-24]+48] != 0x0` |
| `beq` at 0xae57c | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `beq` at 0xae5d0 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `bhi` at 0xae5dc | `[sp-76] > 0x2 (unsigned, 32-bit)` | `[sp-76] <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xae630 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `bne` at 0xae63c | `[sp-84] != 0x0` | `[sp-84] == 0x0` |
| `bne` at 0xae680 | `[sp-84] != 0x1` | `[sp-84] == 0x1` |
| `bne` at 0xae6e8 | `? != 0xf0b8` | `? == 0xf0b8` |
| `bne` at 0xae744 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xae798 | `[[sp-24]+48] == 0x0` | `[[sp-24]+48] != 0x0` |
| `beq` at 0xae7b4 | `[[sp-24]+36] == 0x0` | `[[sp-24]+36] != 0x0` |
| `beq` at 0xae810 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bne` at 0xae834 | `[[sp-24]+48] != 0x0` | `[[sp-24]+48] == 0x0` |
| `bne` at 0xae840 | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `beq` at 0xae890 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `bhi` at 0xae8b4 | `[sp] > [sp-76] (unsigned, 32-bit)` | `[sp] <= [sp-76] (unsigned, 32-bit)` |
| `beq` at 0xae8c0 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `bne` at 0xae8e8 | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `beq` at 0xae934 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bls` at 0xae98c | `[sp-84] <= 0x1 (unsigned, 32-bit)` | `[sp-84] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xaea74 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xaea80 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xaea90 | `[0x133ee4] == 0x0` | `[0x133ee4] != 0x0` |
| `bge` at 0xaeae8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xaeb48 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `blo` at 0xaeb74 | `[sp-64]+3 < [sp-60] (unsigned, 32-bit)` | `[sp-64]+3 >= [sp-60] (unsigned, 32-bit)` |
| `beq` at 0xaebf0 | `[sp-104] == 0x0` | `[sp-104] != 0x0` |
| `bne` at 0xaec00 | `[[sp-24]+48] != 0x0` | `[[sp-24]+48] == 0x0` |
| `bne` at 0xaec3c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xaec4c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaec68 | `r0` | `?` |

## `PPPWrite`

- `PPPWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaddfc to 0xae30c.
  Established. Included byte stores write `0x7e` and `0x7d` to `[[sp-20]+24] +0x0` (`strb` at 0xade48, `strb` at 0xadf30).
  The unsigned condition `[[sp-20]+28]+5 < [[sp-20]+20]` takes `0xade68`, while greater or equal falls through (`blo` at 0xae004).
  An indirect call with unknown target carries `[sp-28]`, `[[sp-20]+16]`, `[[sp-20]+28]`, `[sp-20]` (`mov` at 0xae298).
  PPP output callback ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xade0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xade10 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xade14 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xade18 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xade1c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xade20 | store | 4 | `sp -0x30` | `[sp]` |
| `ldr` at 0xade24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xade28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xade2c | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xade30 | store | 4 | `r0 +0x18` | `[r0+16]` |
| `ldr` at 0xade34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xade3c | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xade48 | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `str` at 0xade50 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xade54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xade5c | store | 4 | `[sp-20] +0x1c` | `0x1` |
| `strh` at 0xade64 | store | 2 | `sp -0x26` | `?` |
| `ldr` at 0xade68 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xade78 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xade7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xade80 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xade84 | store | 4 | `sp -0x3c` | `[[sp-32]]:1` |
| `ldr` at 0xade88 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xade94 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xadea0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xadeac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xadebc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xaded0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaded4 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0xadee4 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xadeec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadef0 | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0xadf00 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xadf0c | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xadf10 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xadf1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadf24 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xadf30 | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0xadf38 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xadf3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadf44 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xadf48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xadf4c | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0xadf58 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xadf60 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xadf64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadf68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadf6c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xadf74 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0xadf7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadf84 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xadf88 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xadf90 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0xadf94 | store | 1 | `[[sp-20]+24] +0x0` | `([[sp-32]]:1 & 0xff)` |
| `str` at 0xadf9c | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xadfa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadfa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadfa8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xadfb0 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0xadfb4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xadfb8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrh` at 0xadfbc | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xadfd0 | store | 2 | `sp -0x26` | `(ret(0xadbe8) & 0xffff)` |
| `ldr` at 0xadfd4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xadfdc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xadfe0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xadfe8 | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0xadfec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadff0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xadff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadffc | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xae008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae00c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae010 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xae018 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xae01c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xae02c | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xae030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae034 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae038 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xae044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae048 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae050 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xae058 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xae060 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xae064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae068 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae06c | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xae070 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xae078 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xae07c | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xae084 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae08c | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xae090 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+16]` |
| `ldr` at 0xae094 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae09c | store | 4 | `[sp-20] +0x1c` | `0x0` |
| `ldrh` at 0xae0a4 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xae0ac | store | 2 | `sp -0x26` | `?` |
| `ldrh` at 0xae0b0 | load | 2 | `sp -0x26` | `-` |
| `strb` at 0xae0b4 | store | 1 | `sp -0x2c` | `([sp-38]:2 & 0xff)` |
| `ldrh` at 0xae0b8 | load | 2 | `sp -0x26` | `-` |
| `strb` at 0xae0c0 | store | 1 | `sp -0x2b` | `?` |
| `strh` at 0xae0c8 | store | 2 | `sp -0x26` | `0x0` |
| `ldrh` at 0xae0cc | load | 2 | `sp -0x26` | `-` |
| `str` at 0xae0dc | store | 4 | `sp -0x34` | `0x0` |
| `ldrh` at 0xae0e0 | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xae0f4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xae0f8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xae0fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae108 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae114 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae120 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae130 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xae144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae148 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0xae158 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xae160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae164 | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0xae174 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xae180 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xae184 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xae190 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae198 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xae1a4 | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0xae1ac | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xae1b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae1b8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0xae1bc | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xae1d0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xae1dc | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xae1e4 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xae1e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae1ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae1f0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xae1f8 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0xae200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae208 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0xae20c | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xae224 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xae228 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xae230 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xae234 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae23c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xae244 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldrh` at 0xae248 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xae250 | store | 2 | `sp -0x26` | `([sp-38]:2+1 & 0xffff)` |
| `ldr` at 0xae258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae25c | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xae264 | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `ldr` at 0xae268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae26c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae270 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xae278 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0xae27c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae284 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xae28c | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xae290 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xae2a0 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xae2a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae2a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae2ac | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xae2b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae2bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae2c4 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xae2cc | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xae2d4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xae2d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae2dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xae2e0 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xae2e4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xae2ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xae2f0 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xae2f8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xae2fc | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0xae300 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xae308 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xae308 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xae308 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xadfc8 | `(unknown callee) 0xadbe8` | `[[sp-32]]:1` | `[sp-38]:2` | `[sp-38]:2` | `[[sp-32]]:1` |
| `mov` at 0xae024 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0xae074 | `(unknown callee) 0xadc40` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |
| `mov` at 0xae298 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0xae2e8 | `(unknown callee) 0xadc40` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xade70 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0xade90 | `[[sp-32]]:1 == 0x13` | `[[sp-32]]:1 != 0x13` |
| `bgt` at 0xade9c | `[[sp-32]]:1 > 0x13 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x13 (signed, 32-bit)` |
| `beq` at 0xadea8 | `[[sp-32]]:1 == 0xd` | `[[sp-32]]:1 != 0xd` |
| `beq` at 0xadeb4 | `[[sp-32]]:1 == 0x11` | `[[sp-32]]:1 != 0x11` |
| `bhi` at 0xadec8 | `[[sp-32]]:1-125 > 0x1 (unsigned, 32-bit)` | `[[sp-32]]:1-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xadedc | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0xadef8 | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0xadf18 | `Z == 1` | `not (Z == 1)` |
| `blo` at 0xae004 | `[[sp-20]+28]+5 < [[sp-20]+20] (unsigned, 32-bit)` | `[[sp-20]+28]+5 >= [[sp-20]+20] (unsigned, 32-bit)` |
| `beq` at 0xae040 | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |
| `bhi` at 0xae0d4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xae104 | `[sp-64] == 0x13` | `[sp-64] != 0x13` |
| `bgt` at 0xae110 | `[sp-64] > 0x13 (signed, 32-bit)` | `[sp-64] <= 0x13 (signed, 32-bit)` |
| `beq` at 0xae11c | `[sp-64] == 0xd` | `[sp-64] != 0xd` |
| `beq` at 0xae128 | `[sp-64] == 0x11` | `[sp-64] != 0x11` |
| `bhi` at 0xae13c | `[sp-64]-125 > 0x1 (unsigned, 32-bit)` | `[sp-64]-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xae150 | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0xae16c | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0xae18c | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `beq` at 0xae2b4 | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xae308 | `r0` | `?` |

## `PPPInit`

- `PPPInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadcc4 to 0xadd98.
  Established. The unnamed `0x19748` call carries entry `r0`, `0x0`, `0x3c`, entry `r3` (`bl` at 0xadcf0).
  Included word assignments write `[sp-24]` to `[sp-20] +0x0`, `[sp]` to `[sp-20] +0x14` and `[sp+24]` to `[sp-20] +0x38` (`str` at 0xadcfc, `str` at 0xadd20, `str` at 0xadd80).
  The unnamed `0xadd98` call carries `[sp-20]`, `0x3`, `[sp-20]`, `[sp+24]` (`bl` at 0xadd8c).
  PPP configuration field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadcd4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xadcd8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xadcdc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xadce0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xadce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadcf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadcf8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xadcfc | store | 4 | `[sp-20] +0x0` | `[sp-24]` |
| `ldr` at 0xadd00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xadd08 | store | 4 | `[sp-20] +0x4` | `[sp-28]` |
| `ldr` at 0xadd0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xadd14 | store | 4 | `[sp-20] +0x10` | `[sp-32]` |
| `ldr` at 0xadd18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd1c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xadd20 | store | 4 | `[sp-20] +0x14` | `[sp]` |
| `ldr` at 0xadd24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd28 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xadd38 | store | 4 | `[sp-20] +0x20` | `?` |
| `ldr` at 0xadd3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd40 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xadd50 | store | 4 | `[sp-20] +0x24` | `?` |
| `ldr` at 0xadd54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd58 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xadd5c | store | 4 | `[sp-20] +0x28` | `[sp+12]` |
| `ldr` at 0xadd60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd64 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xadd68 | store | 4 | `[sp-20] +0x2c` | `[sp+16]` |
| `ldr` at 0xadd6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd70 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xadd74 | store | 4 | `[sp-20] +0x34` | `[sp+20]` |
| `ldr` at 0xadd78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xadd7c | load | 4 | `sp +0x18` | `-` |
| `str` at 0xadd80 | store | 4 | `[sp-20] +0x38` | `[sp+24]` |
| `ldr` at 0xadd84 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xadd94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xadd94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xadd94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xadcf0 | `(unknown callee) 0x19748` | `r0` | `0x0` | `0x3c` | `r3` |
| `bl` at 0xadd8c | `(unknown callee) 0xadd98` | `[sp-20]` | `0x3` | `[sp-20]` | `[sp+24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xadd94 | `r0` | `ret(0xadd98)` |

## `IRArg`

- `IRArg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaadf4 to 0xab1a0.
  Established. A byte read addresses entry `r0 +0x20` (`ldrb` at 0xaae10).
  The signed comparison `[r0+32]:1 > r1` takes `0xaae2c`, while less than or equal falls through (`bgt` at 0xaae1c).
  A word assignment writes `ret(0x1943c)` to `[sp-24] +0x8` (`str` at 0xaaf98).
  The unnamed `0x19610` call carries `[sp-36]`, `[sp-40]`, `[sp-44]` and an unknown fourth candidate (`bl` at 0xab0dc).
  The hardware role of the argument storage remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaae04 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xaae08 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xaae0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaae10 | load | 1 | `r0 +0x20` | `-` |
| `ldr` at 0xaae14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaae24 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xaae2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaae30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaae44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xaae54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaae58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaae6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xaae70 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xaae78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaae7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaae90 | load | 1 | `(unknown)` | `-` |
| `str` at 0xaae94 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xaae98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaae9c | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xaaea0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaeac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xaaeb4 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0xaaec0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaec4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaaec8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaaed0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaaee0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xaaee8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaeec | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xaaf00 | load | 2 | `(unknown)` | `-` |
| `str` at 0xaaf04 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xaaf08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaaf20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaaf24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaaf2c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xaaf30 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xaaf34 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaaf38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaaf40 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaaf4c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xaaf50 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaaf54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaaf60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaaf64 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xaaf6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf70 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaaf7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaf80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaaf84 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0xaaf98 | store | 4 | `[sp-24] +0x8` | `ret(0x1943c)` |
| `ldr` at 0xaaf9c | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaafac | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xaafb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaafb8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaafbc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xaafc4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaafd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaafd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaafe0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaafe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaaff4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaaff8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab000 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab008 | load | 4 | `(unknown)` | `-` |
| `str` at 0xab00c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xab014 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab020 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab028 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab02c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xab040 | load | 2 | `(unknown)` | `-` |
| `str` at 0xab050 | store | 4 | `(unknown)` | `ret(0x1943c)` |
| `ldr` at 0xab054 | load | 4 | `(unknown)` | `-` |
| `str` at 0xab064 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xab06c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab078 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab080 | load | 4 | `(unknown)` | `-` |
| `str` at 0xab084 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xab088 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab08c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab0a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xab0a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xab0ac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xab0b0 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xab0b4 | load | 2 | `[sp-32] +0x10` | `-` |
| `ldr` at 0xab0b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab0c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xab0cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xab0d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xab0d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab0d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xab0e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xab0e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab0ec | store | 4 | `sp -0x24` | `([sp-36] + [sp-44])` |
| `ldr` at 0xab0f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xab0f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab0fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xab100 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xab10c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab110 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xab114 | store | 4 | `sp -0x20` | `[[sp-32]]` |
| `ldr` at 0xab118 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xab120 | store | 4 | `sp -0x28` | `[[sp-32]]+36` |
| `ldr` at 0xab124 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xab128 | load | 2 | `[[sp-32]] +0x10` | `-` |
| `ldr` at 0xab130 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xab13c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xab140 | load | 2 | `[[sp-32]] +0x10` | `-` |
| `str` at 0xab144 | store | 4 | `sp -0x3c` | `[[[sp-32]]+16]:2` |
| `ldr` at 0xab148 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xab150 | store | 4 | `sp -0x3c` | `[[[sp-32]]+16]:2-20` |
| `ldr` at 0xab158 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xab15c | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0xab160 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xab164 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xab168 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xab178 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab17c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab184 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xab18c | load | 4 | `(unknown)` | `-` |
| `str` at 0xab190 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xab194 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xab19c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xab19c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab19c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab19c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaaf90 | `(unknown callee) 0x1943c` | `?` | `0x28` | `[sp-52]` | `?` |
| `bl` at 0xaafd0 | `(unknown callee) 0x19748` | `[[sp-24]+8]` | `0x0` | `?` | `?` |
| `bl` at 0xab048 | `(unknown callee) 0x1943c` | `?` | `0x24` | `[sp-24]` | `?` |
| `bl` at 0xab0dc | `(unknown callee) 0x19610` | `[sp-36]` | `[sp-40]` | `[sp-44]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xaae1c | `[r0+32]:1 > r1 (signed, 32-bit)` | `[r0+32]:1 <= r1 (signed, 32-bit)` |
| `bne` at 0xaae50 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xaaea8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xaaebc | `[sp-48]-1 == 0x0` | `[sp-48]-1 != 0x0` |
| `bne` at 0xaaed8 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xaaf5c | `[sp-52] > [sp-44] (unsigned, 32-bit)` | `[sp-52] <= [sp-44] (unsigned, 32-bit)` |
| `bne` at 0xaaf78 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |
| `bne` at 0xaafa4 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |
| `beq` at 0xaaff0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xab05c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xab108 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `blo` at 0xab138 | `[sp-52] < [[[sp-32]]+16]:2-20 (unsigned, 32-bit)` | `[sp-52] >= [[[sp-32]]+16]:2-20 (unsigned, 32-bit)` |
| `beq` at 0xab170 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab19c | `r0` | `?` |

## `IRFmtAddr`

- `IRFmtAddr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab23c to 0xab514.
  Established. Nonnegative entry `r0` takes `0xab290`, while negative falls through (`bge` at 0xab264).
  Included word assignments set `sp -0x38` to `0xe` and `0xf` on separate paths (`str` at 0xab27c, `str` at 0xab288).
  The unnamed `0x197a8` call carries `[sp-24]`, `0x1140d4`, `[sp-32]` and `[sp-32]` (`bl` at 0xab47c).
  The computed transfer is unresolved, limiting access, call and return records to known edges (`ldrls` at 0xab350).
  The hardware address encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab24c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab250 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xab254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab258 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xab25c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab268 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab27c | store | 4 | `sp -0x38` | `0xe` |
| `str` at 0xab288 | store | 4 | `sp -0x38` | `0xf` |
| `ldr` at 0xab290 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab298 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xab29c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xab2a4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xab2a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xab2ac | store | 4 | `sp -0x20` | `[sp-56]` |
| `ldr` at 0xab2b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab2c0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xab2c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab2d0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xab2d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab2e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab2e8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xab2ec | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xab2f4 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xab300 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xab304 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xab308 | store | 4 | `sp -0x2c` | `[sp-60]` |
| `ldr` at 0xab30c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab318 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab320 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xab324 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xab32c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xab334 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab33c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xab340 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xab344 | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xab348 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xab350 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xab470 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab474 | load | 4 | `abs 0xab50c` | `-` |
| `ldr` at 0xab478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab48c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab494 | store | 4 | `sp -0x18` | `([sp-24] + ret(0x19538))` |
| `ldr` at 0xab498 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab49c | store | 4 | `sp -0x4c` | `[sp-44]` |
| `ldr` at 0xab4a0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xab4a4 | store | 4 | `sp -0x48` | `[sp-48]` |
| `ldr` at 0xab4a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab4ac | load | 4 | `abs 0xab510` | `-` |
| `ldr` at 0xab4b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xab4b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab4bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab4c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab4d0 | store | 4 | `sp -0x18` | `([sp-24] + ret(0x19538))` |
| `ldr` at 0xab4d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab4d8 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xab4dc | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xab4e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab4e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab4e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab47c | `(unknown callee) 0x197a8` | `[sp-24]` | `0x1140d4` | `[sp-32]` | `[sp-32]` |
| `bl` at 0xab484 | `(unknown callee) 0x19538` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0xab4b8 | `(unknown callee) 0x197a8` | `([sp-24] + ret(0x19538))` | `0x1140dc` | `[sp-36]` | `[sp-40]` |
| `bl` at 0xab4c0 | `(unknown callee) 0x19538` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xab264 | `r0 >= 0x0 (signed, 32-bit)` | `r0 < 0x0 (signed, 32-bit)` |
| `bne` at 0xab274 | `? != 0x2` | `? == 0x2` |
| `blt` at 0xab2dc | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0xab314 | `[sp-20] >= 0x0 (signed, 32-bit)` | `[sp-20] < 0x0 (signed, 32-bit)` |
| `ldrls` at 0xab350 | `[sp-32] <= 0xf (unsigned, 32-bit)` | `[sp-32] > 0xf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab4e4 | `r0` | `[sp-28]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xab350).

## `IRFmtClas`

- `IRFmtClas`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab1a0 to 0xab23c.
  Established. Equality entry `r0 == 0x1` takes `0xab200`, while inequality falls through (`beq` at 0xab1c4).
  Distinct paths assign `0x114094`, `0x114098`, `0x11409c` and `0x1140a0` to `sp -0x18` (`str` at 0xab1f8, `str` at 0xab204, `str` at 0xab210, `str` at 0xab21c).
  The hardware interpretation of the selected class remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab1b0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xab1b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab1b8 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xab1bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab1c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab1d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab1e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab1f4 | load | 4 | `abs 0xab22c` | `-` |
| `str` at 0xab1f8 | store | 4 | `sp -0x18` | `0x114094` |
| `ldr` at 0xab200 | load | 4 | `abs 0xab230` | `-` |
| `str` at 0xab204 | store | 4 | `sp -0x18` | `0x114098` |
| `ldr` at 0xab20c | load | 4 | `abs 0xab234` | `-` |
| `str` at 0xab210 | store | 4 | `sp -0x18` | `0x11409c` |
| `ldr` at 0xab218 | load | 4 | `abs 0xab238` | `-` |
| `str` at 0xab21c | store | 4 | `sp -0x18` | `0x1140a0` |
| `ldr` at 0xab220 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xab228 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab228 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab228 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xab1c4 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0xab1d0 | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xab1dc | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xab1ec | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab228 | `r0` | `?` |

## `IRFmtError`

- `IRFmtError`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab814 to 0xab870.
  Established. Zero entry `r0` takes `0xab854`, while nonzero falls through (`beq` at 0xab830).
  That zero path assigns `0x1142f8` to `sp -0x18` (`str` at 0xab858).
  The unnamed `0x1f8e0` call carries unknown, entry `r1`, unknown and entry `r0` (`bl` at 0xab844).
  Error-number ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab824 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xab828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab838 | load | 4 | `abs 0xab868` | `-` |
| `str` at 0xab84c | store | 4 | `sp -0x18` | `ret(0x1f8e0)` |
| `ldr` at 0xab854 | load | 4 | `abs 0xab86c` | `-` |
| `str` at 0xab858 | store | 4 | `sp -0x18` | `0x1142f8` |
| `ldr` at 0xab85c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xab864 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xab864 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xab864 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab844 | `(unknown callee) 0x1f8e0` | `?` | `r1` | `?` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xab830 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xab864 | `r0` | `?` |

## `SocketMakeInetConnection`

- `SocketMakeInetConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabe70 to 0xac074.
  Established. The unnamed `0xac658` call carries entry `r0`, `sp-48`, entry `r2` and `sp-48` (`bl` at 0xabe98).
  Included halfword assignments write `0x2` to `sp -0x2c` and `(ret(0x19730) & 0xffff)` to `sp -0x2a` (`strh` at 0xabf54, `strh` at 0xabf70).
  A zero `ret(0x19754)` takes `0xac04c`, while nonzero falls through (`beq` at 0xabf98).
  The unresolved computed transfer limits the records to known edges (`ldrls` at 0xabfb0).
  Hardware transport responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabe80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabe84 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xabe88 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xabe90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabeac | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xabeb0 | load | 4 | `abs 0xac060` | `-` |
| `str` at 0xabeb4 | store | 4 | `sp -0x40` | `0x11430c` |
| `ldr` at 0xabeb8 | load | 4 | `abs 0xac064` | `-` |
| `ldr` at 0xabebc | load | 4 | `abs 0xac060` | `-` |
| `ldr` at 0xabec4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabed0 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xabeec | store | 4 | `sp -0x34` | `ret(0x197c0)` |
| `ldr` at 0xabef0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xabf04 | load | 4 | `abs 0xac068` | `-` |
| `ldr` at 0xabf08 | load | 4 | `abs 0xac06c` | `-` |
| `ldr` at 0xabf0c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabf10 | load | 4 | `abs 0xac060` | `-` |
| `str` at 0xabf1c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xabf24 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xabf28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabf34 | store | 4 | `sp -0x38` | `ret(0xabde0)` |
| `ldr` at 0xabf38 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xabf44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xabf48 | store | 4 | `sp -0x3c` | `ret(0xabde0)` |
| `strh` at 0xabf54 | store | 2 | `sp -0x2c` | `0x2` |
| `ldr` at 0xabf58 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xabf70 | store | 2 | `sp -0x2a` | `(ret(0x19730) & 0xffff)` |
| `ldr` at 0xabf74 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xabf78 | store | 4 | `sp -0x28` | `[sp-48]` |
| `ldr` at 0xabf80 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xabfa4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldrls` at 0xabfb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xac014 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac018 | store | 4 | `sp -0x44` | `[sp-24]` |
| `ldr` at 0xac01c | load | 4 | `abs 0xac060` | `-` |
| `str` at 0xac020 | store | 4 | `sp -0x40` | `0x11430c` |
| `ldr` at 0xac024 | load | 4 | `abs 0xac068` | `-` |
| `ldr` at 0xac028 | load | 4 | `abs 0xac070` | `-` |
| `ldr` at 0xac02c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac038 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xac044 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xac04c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xac050 | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0xac054 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0xac05c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac05c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac05c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabe98 | `(unknown callee) 0xac658` | `r0` | `sp-48` | `r2` | `sp-48` |
| `bl` at 0xabec8 | `(unknown callee) 0x1f5a0` | `0xfffffe6a` | `0x11430c` | `0x0` | `[sp-20]` |
| `bl` at 0xabee4 | `(unknown callee) 0x197c0` | `0x2` | `0x1` | `0x0` | `ret(0xac658)` |
| `bl` at 0xabefc | `(unknown callee) 0x193d0` | `ret(0x197c0)` | `?` | `?` | `ret(0x197c0)` |
| `bl` at 0xabf14 | `(unknown callee) 0x1f538` | `0xfffffe6c` | `0x114304` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xabf2c | `(unknown callee) 0xabde0` | `ret(0x197c0)` | `[sp-28]` | `?` | `ret(0x197c0)` |
| `bl` at 0xabf68 | `(unknown callee) 0x19730` | `?` | `?` | `?` | `?` |
| `bl` at 0xabf8c | `(unknown callee) 0x19754` | `[sp-52]` | `sp-44` | `0x10` | `sp-44` |
| `bl` at 0xabf9c | `(unknown callee) 0x193d0` | `ret(0x19754)` | `?` | `?` | `ret(0x19754)` |
| `bl` at 0xac008 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]-101` |
| `bl` at 0xac034 | `(unknown callee) 0x1f5a0` | `0xfffffe6c` | `0x11435c` | `[ret(0x193d0)]` | `[sp-20]` |
| `bl` at 0xac03c | `(unknown callee) 0x1931c` | `[sp-52]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xabea4 | `ret(0xac658) == 0x0` | `ret(0xac658) != 0x0` |
| `bge` at 0xabef8 | `ret(0x197c0) >= 0x0 (signed, 32-bit)` | `ret(0x197c0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xabf40 | `ret(0xabde0) == 0x0` | `ret(0xabde0) != 0x0` |
| `beq` at 0xabf98 | `ret(0x19754) == 0x0` | `ret(0x19754) != 0x0` |
| `ldrls` at 0xabfb0 | `[ret(0x193d0)]-101 <= 0xe (unsigned, 32-bit)` | `[ret(0x193d0)]-101 > 0xe (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac05c | `r0` | `[sp-60]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xabfb0).

## `SocketMakeUnixConnection`

- `SocketMakeUnixConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac074 to 0xac230.
  Established. The unnamed `0x19604` call carries `ret(0x197c0)`, `0x4`, `0x800` and `[sp-24]` (`bl` at 0xac0f0).
  Another unnamed `0x19754` call carries `[sp-140]`, `sp-136`, `ret(0xacaf0)` and `sp-136` (`bl` at 0xac158).
  Equality `[ret(0x193d0)] == 0x73` takes `0xac204`, while inequality falls through (`beq` at 0xac1a8).
  The device boundary behind the connection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac084 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac088 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac0a0 | store | 4 | `sp -0x8c` | `ret(0x197c0)` |
| `ldr` at 0xac0a4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xac0bc | load | 4 | `abs 0xac218` | `-` |
| `ldr` at 0xac0c0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac0c4 | load | 4 | `abs 0xac21c` | `-` |
| `str` at 0xac0d0 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xac0d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac0e4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xac10c | load | 4 | `abs 0xac220` | `-` |
| `ldr` at 0xac110 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac114 | load | 4 | `abs 0xac224` | `-` |
| `ldr` at 0xac11c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xac128 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xac138 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac144 | store | 4 | `sp -0x90` | `ret(0xacaf0)` |
| `ldr` at 0xac14c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xac154 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xac170 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xac174 | store | 4 | `sp -0x98` | `[ret(0x193d0)]` |
| `ldr` at 0xac178 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xac184 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xac190 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xac1a0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xac1b0 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xac1bc | store | 4 | `sp -0x94` | `?` |
| `str` at 0xac1d8 | store | 4 | `sp -0x9c` | `sp-134` |
| `ldr` at 0xac1e0 | load | 4 | `abs 0xac228` | `-` |
| `ldr` at 0xac1e4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac1e8 | load | 4 | `abs 0xac22c` | `-` |
| `ldr` at 0xac1f0 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xac1fc | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xac204 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xac208 | store | 4 | `sp -0x94` | `[sp-140]` |
| `ldr` at 0xac20c | load | 4 | `sp -0x94` | `-` |
| `ldm` at 0xac214 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac214 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac214 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac098 | `(unknown callee) 0x197c0` | `0x1` | `0x1` | `0x0` | `r3` |
| `bl` at 0xac0b0 | `(unknown callee) 0x193d0` | `ret(0x197c0)` | `?` | `?` | `ret(0x197c0)` |
| `bl` at 0xac0c8 | `(unknown callee) 0x1f538` | `?` | `0x114304` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xac0f0 | `(unknown callee) 0x19604` | `ret(0x197c0)` | `0x4` | `0x800` | `[sp-24]` |
| `bl` at 0xac100 | `(unknown callee) 0x193d0` | `ret(0x19604)` | `?` | `?` | `ret(0x19604)` |
| `bl` at 0xac118 | `(unknown callee) 0x1f538` | `?` | `0x114348` | `[ret(0x193d0)]` | `0x114350` |
| `bl` at 0xac120 | `(unknown callee) 0x1931c` | `[sp-140]` | `?` | `?` | `?` |
| `bl` at 0xac13c | `(unknown callee) 0xacaf0` | `sp-136` | `[sp-20]` | `?` | `sp-136` |
| `bl` at 0xac158 | `(unknown callee) 0x19754` | `[sp-140]` | `sp-136` | `ret(0xacaf0)` | `sp-136` |
| `bl` at 0xac168 | `(unknown callee) 0x193d0` | `ret(0x19754)` | `?` | `?` | `ret(0x19754)` |
| `bl` at 0xac1b4 | `(unknown callee) 0x1931c` | `[sp-140]` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xac1c4 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xac1ec | `(unknown callee) 0x1f538` | `?` | `0x11435c` | `[ret(0x193d0)]` | `0x11433c` |
| `bl` at 0xac1f4 | `(unknown callee) 0x1931c` | `[sp-140]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xac0ac | `ret(0x197c0) >= 0x0 (signed, 32-bit)` | `ret(0x197c0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xac0e0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xac0fc | `ret(0x19604) == 0x0` | `ret(0x19604) != 0x0` |
| `beq` at 0xac164 | `ret(0x19754) == 0x0` | `ret(0x19754) != 0x0` |
| `bgt` at 0xac180 | `[ret(0x193d0)] > 0x6f (signed, 32-bit)` | `[ret(0x193d0)] <= 0x6f (signed, 32-bit)` |
| `bge` at 0xac18c | `[ret(0x193d0)] >= 0x6e (signed, 32-bit)` | `[ret(0x193d0)] < 0x6e (signed, 32-bit)` |
| `beq` at 0xac198 | `[ret(0x193d0)] == 0x2` | `[ret(0x193d0)] != 0x2` |
| `beq` at 0xac1a8 | `[ret(0x193d0)] == 0x73` | `[ret(0x193d0)] != 0x73` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac214 | `r0` | `[sp-148]` |

## `SocketExportInetConnection`

- `SocketExportInetConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab8cc to 0xabaa8.
  Established. A halfword assignment sets `sp -0x24` to `0x2`, and a word assignment clears `sp -0x20` (`strh` at 0xab9a4, `str` at 0xab9c8).
  The unnamed `0x194cc` call carries `ret(0x197c0)`, `0x1`, `0x2` and `sp-44` (`bl` at 0xab94c).
  A signed nonnegative `ret(0x19430)` takes `0xaba78`, while negative falls through (`bge` at 0xaba44).
  The hardware significance of the exported endpoint remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab8dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab8e4 | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xab8fc | store | 4 | `sp -0x28` | `ret(0x197c0)` |
| `ldr` at 0xab900 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab914 | load | 4 | `abs 0xaba8c` | `-` |
| `ldr` at 0xab918 | load | 4 | `abs 0xaba90` | `-` |
| `ldr` at 0xab91c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xab920 | load | 4 | `abs 0xaba94` | `-` |
| `str` at 0xab92c | store | 4 | `sp -0x30` | `?` |
| `str` at 0xab93c | store | 4 | `sp -0x38` | `0x4` |
| `ldr` at 0xab940 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab964 | load | 4 | `abs 0xaba8c` | `-` |
| `ldr` at 0xab968 | load | 4 | `abs 0xaba98` | `-` |
| `ldr` at 0xab96c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xab970 | load | 4 | `abs 0xaba9c` | `-` |
| `ldr` at 0xab978 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xab984 | store | 4 | `sp -0x30` | `?` |
| `strh` at 0xab9a4 | store | 2 | `sp -0x24` | `0x2` |
| `ldr` at 0xab9a8 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xab9c0 | store | 2 | `sp -0x22` | `(ret(0x19730) & 0xffff)` |
| `str` at 0xab9c8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xab9d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab9f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab9fc | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xaba00 | load | 4 | `abs 0xaba94` | `-` |
| `str` at 0xaba04 | store | 4 | `sp -0x34` | `0x11430c` |
| `ldr` at 0xaba08 | load | 4 | `abs 0xaba8c` | `-` |
| `ldr` at 0xaba0c | load | 4 | `abs 0xabaa0` | `-` |
| `ldr` at 0xaba10 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xaba14 | load | 4 | `abs 0xaba94` | `-` |
| `ldr` at 0xaba1c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaba28 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xaba30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaba50 | load | 4 | `abs 0xaba8c` | `-` |
| `ldr` at 0xaba54 | load | 4 | `abs 0xabaa4` | `-` |
| `ldr` at 0xaba58 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xaba5c | load | 4 | `abs 0xaba94` | `-` |
| `ldr` at 0xaba64 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaba70 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xaba78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaba7c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xaba80 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xaba88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaba88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaba88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab8f4 | `(unknown callee) 0x197c0` | `0x2` | `0x1` | `0x0` | `0x1` |
| `bl` at 0xab90c | `(unknown callee) 0x193d0` | `ret(0x197c0)` | `?` | `?` | `ret(0x197c0)` |
| `bl` at 0xab924 | `(unknown callee) 0x1f538` | `0xfffffe70` | `0x114304` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xab94c | `(unknown callee) 0x194cc` | `ret(0x197c0)` | `0x1` | `0x2` | `sp-44` |
| `bl` at 0xab95c | `(unknown callee) 0x193d0` | `ret(0x194cc)` | `?` | `?` | `ret(0x194cc)` |
| `bl` at 0xab974 | `(unknown callee) 0x1f538` | `0xfffffe70` | `0x114310` | `[ret(0x193d0)]` | `0x11431c` |
| `bl` at 0xab97c | `(unknown callee) 0x1931c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xab99c | `(unknown callee) 0x19748` | `sp-36` | `0x0` | `0x10` | `sp-36` |
| `bl` at 0xab9b8 | `(unknown callee) 0x19730` | `?` | `?` | `?` | `?` |
| `bl` at 0xab9dc | `(unknown callee) 0x195ec` | `[sp-40]` | `sp-36` | `0x10` | `sp-36` |
| `bl` at 0xab9ec | `(unknown callee) 0x193d0` | `ret(0x195ec)` | `?` | `?` | `ret(0x195ec)` |
| `bl` at 0xaba18 | `(unknown callee) 0x1f5a0` | `0xfffffe70` | `0x11432c` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xaba20 | `(unknown callee) 0x1931c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xaba38 | `(unknown callee) 0x19430` | `[sp-40]` | `0x5` | `?` | `ret(0x195ec)` |
| `bl` at 0xaba48 | `(unknown callee) 0x193d0` | `ret(0x19430)` | `?` | `?` | `ret(0x19430)` |
| `bl` at 0xaba60 | `(unknown callee) 0x1f538` | `0xfffffe70` | `0x114334` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xaba68 | `(unknown callee) 0x1931c` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xab908 | `ret(0x197c0) >= 0x0 (signed, 32-bit)` | `ret(0x197c0) < 0x0 (signed, 32-bit)` |
| `bge` at 0xab958 | `ret(0x194cc) >= 0x0 (signed, 32-bit)` | `ret(0x194cc) < 0x0 (signed, 32-bit)` |
| `bge` at 0xab9e8 | `ret(0x195ec) >= 0x0 (signed, 32-bit)` | `ret(0x195ec) < 0x0 (signed, 32-bit)` |
| `bge` at 0xaba44 | `ret(0x19430) >= 0x0 (signed, 32-bit)` | `ret(0x19430) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaba88 | `r0` | `[sp-48]` |

## `SocketExportUnixConnection`

- `SocketExportUnixConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabaa8 to 0xabc00.
  Established. The unnamed `0xacaf0` call carries `sp-132`, `[sp-20]`, unknown and `sp-132` (`bl` at 0xabb14).
  A following unnamed `0x19358` call carries `sp-130`, unknown, unknown and `sp-130` (`bl` at 0xabb2c).
  The signed condition `ret(0x195ec) >= 0` takes `0xabb90`, while negative falls through (`bge` at 0xabb4c).
  Endpoint naming and hardware ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabab8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabad0 | store | 4 | `sp -0x88` | `ret(0x197c0)` |
| `ldr` at 0xabad4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xabaec | load | 4 | `abs 0xabbec` | `-` |
| `ldr` at 0xabaf0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabaf4 | load | 4 | `abs 0xabbf0` | `-` |
| `str` at 0xabb00 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xabb10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabb1c | store | 4 | `sp -0x8c` | `ret(0xacaf0)` |
| `ldr` at 0xabb34 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xabb3c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xabb64 | store | 4 | `sp -0x94` | `sp-130` |
| `ldr` at 0xabb6c | load | 4 | `abs 0xabbf4` | `-` |
| `ldr` at 0xabb70 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabb74 | load | 4 | `abs 0xabbf8` | `-` |
| `ldr` at 0xabb7c | load | 4 | `sp -0x88` | `-` |
| `str` at 0xabb88 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xabb90 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xabbb4 | load | 4 | `abs 0xabbfc` | `-` |
| `ldr` at 0xabbb8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabbbc | load | 4 | `abs 0xabbf0` | `-` |
| `ldr` at 0xabbc4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xabbd0 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xabbd8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xabbdc | store | 4 | `sp -0x90` | `[sp-136]` |
| `ldr` at 0xabbe0 | load | 4 | `sp -0x90` | `-` |
| `ldm` at 0xabbe8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xabbe8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xabbe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabac8 | `(unknown callee) 0x197c0` | `0x1` | `0x1` | `0x0` | `r3` |
| `bl` at 0xabae0 | `(unknown callee) 0x193d0` | `ret(0x197c0)` | `?` | `?` | `ret(0x197c0)` |
| `bl` at 0xabaf8 | `(unknown callee) 0x1f538` | `?` | `0x114304` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xabb14 | `(unknown callee) 0xacaf0` | `sp-132` | `[sp-20]` | `?` | `sp-132` |
| `bl` at 0xabb2c | `(unknown callee) 0x19358` | `sp-130` | `?` | `?` | `sp-130` |
| `bl` at 0xabb40 | `(unknown callee) 0x195ec` | `[sp-136]` | `sp-132` | `[sp-140]` | `sp-132` |
| `bl` at 0xabb50 | `(unknown callee) 0x193d0` | `ret(0x195ec)` | `?` | `?` | `ret(0x195ec)` |
| `bl` at 0xabb78 | `(unknown callee) 0x1f538` | `?` | `0x11432c` | `[ret(0x193d0)]` | `0x11433c` |
| `bl` at 0xabb80 | `(unknown callee) 0x1931c` | `[sp-136]` | `?` | `?` | `?` |
| `bl` at 0xabb98 | `(unknown callee) 0x19430` | `[sp-136]` | `0x5` | `?` | `ret(0x195ec)` |
| `bl` at 0xabba8 | `(unknown callee) 0x193d0` | `ret(0x19430)` | `?` | `?` | `ret(0x19430)` |
| `bl` at 0xabbc0 | `(unknown callee) 0x1f538` | `?` | `0x114334` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xabbc8 | `(unknown callee) 0x1931c` | `[sp-136]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xabadc | `ret(0x197c0) >= 0x0 (signed, 32-bit)` | `ret(0x197c0) < 0x0 (signed, 32-bit)` |
| `bge` at 0xabb4c | `ret(0x195ec) >= 0x0 (signed, 32-bit)` | `ret(0x195ec) < 0x0 (signed, 32-bit)` |
| `bge` at 0xabba4 | `ret(0x19430) >= 0x0 (signed, 32-bit)` | `ret(0x19430) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xabbe8 | `r0` | `[sp-144]` |

## `SocketAcceptInetConnection`

- `SocketAcceptInetConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabc00 to 0xabcf0.
  Established. A word assignment initializes `sp -0x30` to `0x10` (`str` at 0xabc1c).
  The unnamed `0x193dc` call carries entry `r0`, `sp-44`, `sp-48` and `sp-44` (`bl` at 0xabc30).
  Zero `[sp-24]` takes `0xabcc8`, while nonzero falls through (`beq` at 0xabc78).
  The hardware meaning of the returned connection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabc10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabc14 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xabc1c | store | 4 | `sp -0x30` | `0x10` |
| `ldr` at 0xabc28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabc38 | store | 4 | `sp -0x1c` | `ret(0x193dc)` |
| `ldr` at 0xabc3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabc50 | load | 4 | `abs 0xabcdc` | `-` |
| `ldr` at 0xabc54 | load | 4 | `abs 0xabce0` | `-` |
| `ldr` at 0xabc58 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabc5c | load | 4 | `abs 0xabce4` | `-` |
| `str` at 0xabc68 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xabc70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabc7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabca0 | load | 4 | `abs 0xabcdc` | `-` |
| `ldr` at 0xabca4 | load | 4 | `abs 0xabce8` | `-` |
| `ldr` at 0xabca8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabcac | load | 4 | `abs 0xabcec` | `-` |
| `ldr` at 0xabcb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabcc0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xabcc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabccc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xabcd0 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xabcd8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xabcd8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xabcd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabc30 | `(unknown callee) 0x193dc` | `r0` | `sp-44` | `sp-48` | `sp-44` |
| `bl` at 0xabc48 | `(unknown callee) 0x193d0` | `ret(0x193dc)` | `?` | `?` | `ret(0x193dc)` |
| `bl` at 0xabc60 | `(unknown callee) 0x1f538` | `0xfffffe6e` | `0x114340` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xabc88 | `(unknown callee) 0x19604` | `ret(0x193dc)` | `0x4` | `0x800` | `[sp-24]` |
| `bl` at 0xabc98 | `(unknown callee) 0x193d0` | `ret(0x19604)` | `?` | `?` | `ret(0x19604)` |
| `bl` at 0xabcb0 | `(unknown callee) 0x1f538` | `0xfffffe6e` | `0x114348` | `[ret(0x193d0)]` | `0x114350` |
| `bl` at 0xabcb8 | `(unknown callee) 0x1931c` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xabc44 | `ret(0x193dc) >= 0x0 (signed, 32-bit)` | `ret(0x193dc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xabc78 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xabc94 | `ret(0x19604) == 0x0` | `ret(0x19604) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xabcd8 | `r0` | `?` |

## `SocketAcceptUnixConnection`

- `SocketAcceptUnixConnection`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabcf0 to 0xabde0.
  Established. The local word at `sp -0x90` is initialized to `0x70` (`str` at 0xabd0c).
  An unnamed `0x193dc` call carries entry `r0`, `sp-140`, `sp-144` and `sp-140` (`bl` at 0xabd20).
  Nonnegative `ret(0x193dc)` takes `0xabd60`, while negative falls through (`bge` at 0xabd34).
  The hardware responsibilities of this acceptance path remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabd00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabd04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xabd0c | store | 4 | `sp -0x90` | `0x70` |
| `ldr` at 0xabd18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabd28 | store | 4 | `sp -0x1c` | `ret(0x193dc)` |
| `ldr` at 0xabd2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabd40 | load | 4 | `abs 0xabdcc` | `-` |
| `ldr` at 0xabd44 | load | 4 | `abs 0xabdd0` | `-` |
| `ldr` at 0xabd48 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabd4c | load | 4 | `abs 0xabdd4` | `-` |
| `str` at 0xabd58 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xabd60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabd6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabd90 | load | 4 | `abs 0xabdcc` | `-` |
| `ldr` at 0xabd94 | load | 4 | `abs 0xabdd8` | `-` |
| `ldr` at 0xabd98 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabd9c | load | 4 | `abs 0xabddc` | `-` |
| `ldr` at 0xabda4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabdb0 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xabdb8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabdbc | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xabdc0 | load | 4 | `sp -0x94` | `-` |
| `ldm` at 0xabdc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xabdc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xabdc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabd20 | `(unknown callee) 0x193dc` | `r0` | `sp-140` | `sp-144` | `sp-140` |
| `bl` at 0xabd38 | `(unknown callee) 0x193d0` | `ret(0x193dc)` | `?` | `?` | `ret(0x193dc)` |
| `bl` at 0xabd50 | `(unknown callee) 0x1f538` | `0xfffffe6d` | `0x114340` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xabd78 | `(unknown callee) 0x19604` | `ret(0x193dc)` | `0x4` | `0x800` | `[sp-24]` |
| `bl` at 0xabd88 | `(unknown callee) 0x193d0` | `ret(0x19604)` | `?` | `?` | `ret(0x19604)` |
| `bl` at 0xabda0 | `(unknown callee) 0x1f538` | `0xfffffe6d` | `0x114348` | `[ret(0x193d0)]` | `0x114350` |
| `bl` at 0xabda8 | `(unknown callee) 0x1931c` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xabd34 | `ret(0x193dc) >= 0x0 (signed, 32-bit)` | `ret(0x193dc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xabd68 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xabd84 | `ret(0x19604) == 0x0` | `ret(0x19604) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xabdc8 | `r0` | `?` |

## `SocketRead`

- `SocketRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacb64 to 0xacc50.
  Established. A word assignment clears `ret(0x193d0) +0x0` (`str` at 0xacba4).
  The unnamed `0x197fc` call carries `[sp-32]`, `[sp-36]`, `[sp-20]` and `0x0` (`bl` at 0xacbb4).
  Inequality `[ret(0x193d0)] != 0xb` takes `0xacbe8`, while equality falls through (`bne` at 0xacbd0).
  Hardware receive semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xacb74 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0xacb78 | store | 4 | `sp -0x24` | `r1` |
| `str` at 0xacb7c | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0xacb80 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xacb84 | store | 4 | `sp -0x14` | `r2` |
| `ldr` at 0xacb88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xacba4 | store | 4 | `ret(0x193d0) +0x0` | `0x0` |
| `ldr` at 0xacba8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacbac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xacbb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xacbbc | store | 4 | `sp -0x18` | `ret(0x197fc)` |
| `ldr` at 0xacbc8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xacbd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacbd8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xacbe0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xacbe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xacbf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacbf8 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xacc00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xacc10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacc14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacc1c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xacc20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xacc24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xacc2c | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])` |
| `ldr` at 0xacc34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacc38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xacc40 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xacc44 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xacc4c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xacc4c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xacc4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xacb94 | `(unknown callee) 0x193d0` | `?` | `?` | `?` | `?` |
| `bl` at 0xacbb4 | `(unknown callee) 0x197fc` | `[sp-32]` | `[sp-36]` | `[sp-20]` | `0x0` |
| `bl` at 0xacbc0 | `(unknown callee) 0x193d0` | `ret(0x197fc)` | `?` | `?` | `ret(0x197fc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xacb90 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xacbd0 | `[ret(0x193d0)] != 0xb` | `[ret(0x193d0)] == 0xb` |
| `bge` at 0xacbf0 | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xacc08 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xacc4c | `r0` | `[sp-28]` |

## `SocketWrite`

- `SocketWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacc50 to 0xacd00.
  Established. The signed condition `ret(0x192e0) >= 0` takes `0xaccb0`, while negative falls through (`bge` at 0xacca0).
  On the negative path, a word assignment stores `ret(0x192e0)` to `sp -0x1c` (`str` at 0xacca8).
  A later word assignment updates `sp -0x24` with `([sp-36] + [sp-24])` (`str` at 0xaccdc).
  Hardware send completion semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xacc60 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0xacc64 | store | 4 | `sp -0x24` | `r1` |
| `str` at 0xacc68 | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0xacc6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xacc70 | store | 4 | `sp -0x14` | `r2` |
| `ldr` at 0xacc74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacc80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacc84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xacc88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xacc94 | store | 4 | `sp -0x18` | `ret(0x192e0)` |
| `ldr` at 0xacc98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xacca4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacca8 | store | 4 | `sp -0x1c` | `ret(0x192e0)` |
| `ldr` at 0xaccb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaccc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaccc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacccc | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xaccd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaccd4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaccdc | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])` |
| `ldr` at 0xacce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacce8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaccf0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xaccf4 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xaccfc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaccfc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaccfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xacc8c | `(unknown callee) 0x192e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xacc7c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xacca0 | `ret(0x192e0) >= 0x0 (signed, 32-bit)` | `ret(0x192e0) < 0x0 (signed, 32-bit)` |
| `bne` at 0xaccb8 | `ret(0x192e0) != 0x0` | `ret(0x192e0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaccfc | `r0` | `[sp-28]` |
