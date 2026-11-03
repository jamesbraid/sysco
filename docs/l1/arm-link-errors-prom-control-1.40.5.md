# ARM link errors and PROM control paths in 1.40.5

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
No unresolved computed transfers are reported. Unknown data values and instruction effects remain
explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/link-errors-prom-control-facts.txt) retain all tool sections.

## `LinkErrReadStatus`

- `LinkErrReadStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x365bc to 0x367e4.
  Established. The byte assignment through `r1 +0x0` writes `0x1` (`strb` at 0x365f0). The unnamed
  `0xeae04` call at `0x36628` carries `[sp-36]`, `0x1000`, `sp-48`, `sp-52` (`bl` at 0x36628); its
  zero result takes `0x36650`, while nonzero falls through (`beq` at 0x3663c). Link-status meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x365d0 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x365d4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x365dc | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x365e4 | store | 4 | `sp -0x24` | `0x600` |
| `ldr` at 0x365e8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x365f0 | store | 1 | `r1 +0x0` | `0x1` |
| `ldr` at 0x365f4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x365fc | store | 1 | `[sp-24] +0x1` | `0x0` |
| `ldr` at 0x36600 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x36608 | store | 1 | `[sp-24] +0x2` | `0x0` |
| `ldr` at 0x36618 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36630 | store | 4 | `sp -0x1c` | `ret(0xeae04)` |
| `ldr` at 0x36634 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36644 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36648 | store | 4 | `sp -0x40` | `[sp-28]` |
| `ldr` at 0x36654 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x36664 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x36674 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x36678 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x36680 | load | 4 | `abs 0x367d8` | `-` |
| `str` at 0x36688 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x36698 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3669c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x366b0 | store | 4 | `sp -0x1c` | `ret(0xeae04)` |
| `ldr` at 0x366b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x366c8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x366d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x366dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x366f0 | store | 4 | `sp -0x1c` | `ret(0xeae04)` |
| `ldr` at 0x366f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36708 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x36714 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x36728 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x36730 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x36738 | store | 1 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x3673c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3674c | load | 4 | `abs 0x367dc` | `-` |
| `ldrb` at 0x36750 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x36754 | load | 4 | `abs 0x367dc` | `-` |
| `ldrb` at 0x36758 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3675c | load | 4 | `(unknown)` | `-` |
| `str` at 0x36764 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x36768 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x36770 | store | 1 | `[sp-24] +0x1` | `0x1` |
| `str` at 0x36778 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x3677c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3678c | load | 4 | `abs 0x367e0` | `-` |
| `ldrb` at 0x36790 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x36794 | load | 4 | `abs 0x367e0` | `-` |
| `ldrb` at 0x36798 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3679c | load | 4 | `(unknown)` | `-` |
| `str` at 0x367a4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x367a8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x367b0 | store | 1 | `[sp-24] +0x2` | `0x1` |
| `str` at 0x367b8 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x367bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x367c0 | store | 4 | `sp -0x40` | `[sp-32]` |
| `str` at 0x367cc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x367d0 | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0x367d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x367d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x367d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3660c | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `[sp-24]` | `0x0` |
| `bl` at 0x36628 | `(unknown callee) 0xeae04` | `[sp-36]` | `0x1000` | `sp-48` | `sp-52` |
| `bl` at 0x36640 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x36650 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x3668c | `(unknown callee) 0xe3cf8` | `ret(0xe3d64)` | `?` | `?` | `?` |
| `bl` at 0x366a8 | `(unknown callee) 0xeae04` | `[sp-36]` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0x366c0 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x366e8 | `(unknown callee) 0xeae04` | `[sp-36]` | `[sp-44]` | `sp-56` | `sp-60` |
| `bl` at 0x36700 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x36710 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3663c | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x36660 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x366bc | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x366fc | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `bne` at 0x36720 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x36748 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x36788 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x367d4 | `r0` | `?` |

## `LinkErrReadBER`

- `LinkErrReadBER`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x367e4 to 0x36d10.
  Established. The unnamed `0xeae04` call at `0x36848` carries `[sp-36]`, `0x1000`, `sp-76`, `sp-80`
  (`bl` at 0x36848); its zero result takes `0x36870`, while nonzero falls through (`beq` at
  0x3685c). The word assignment at `[sp-28] +0xc` writes `[sp-104]` (`str` at 0x36a68). Error-rate
  units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x367f8 | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x367fc | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `str` at 0x36804 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x36808 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x36810 | store | 1 | `r1 +0x0` | `0x1` |
| `str` at 0x36818 | store | 4 | `sp -0x24` | `0x600` |
| `str` at 0x36820 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x36828 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x36838 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36850 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x36854 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36864 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36868 | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x36874 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x36884 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3688c | load | 4 | `abs 0x36cf8` | `-` |
| `str` at 0x36894 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x36898 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x368a0 | load | 4 | `abs 0x36cfc` | `-` |
| `str` at 0x368a8 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x368ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x368b4 | load | 4 | `abs 0x36d00` | `-` |
| `str` at 0x368bc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x368cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x368d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x368e4 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x368e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x368f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x368fc | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x3690c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36910 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x36924 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x36928 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36938 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3693c | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x3694c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36950 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x36964 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x36968 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x36978 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3697c | store | 4 | `sp -0x80` | `[sp-32]` |
| `ldr` at 0x36984 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36988 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3699c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x369b0 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x369b8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x369c0 | store | 1 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x369c4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x369d8 | store | 4 | `sp -0x84` | `0x1` |
| `str` at 0x369e4 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x369e8 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x369ec | store | 4 | `sp -0x38` | `[sp-132]` |
| `ldr` at 0x369f0 | load | 4 | `abs 0x36d04` | `-` |
| `ldr` at 0x369f4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x369fc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x36a00 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x36a0c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x36a10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x36a18 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x36a24 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x36a28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x36a34 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36a3c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x36a40 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36a48 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x36a4c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x36a54 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x36a5c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x36a60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36a64 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36a68 | store | 4 | `[sp-28] +0xc` | `[sp-104]` |
| `ldr` at 0x36a6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36a70 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x36a74 | store | 4 | `[sp-28] +0x8` | `[sp-100]` |
| `ldr` at 0x36a78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36a7c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x36a80 | store | 4 | `[sp-28] +0x4` | `[sp-60]` |
| `ldr` at 0x36a84 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36a88 | store | 4 | `sp -0x6c` | `[sp-104]` |
| `ldr` at 0x36a8c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x36a90 | store | 4 | `sp -0x70` | `[sp-100]` |
| `ldr` at 0x36a94 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x36a98 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x36a9c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x36aa8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x36ab8 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x36ac0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x36ac4 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x36acc | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x36ad0 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x36ad8 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x36adc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x36ae8 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x36af0 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x36af4 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x36afc | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x36b04 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x36b10 | load | 4 | `sp -0x70` | `-` |
| `ldm` at 0x36b2c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36b2c | load | 4 | `(unknown)` | `-` |
| `stm` at 0x36b30 | store | 4 | `(unknown)` | `?` |
| `stm` at 0x36b30 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x36b34 | load | 4 | `abs 0x36d08` | `-` |
| `ldr` at 0x36b40 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x36b58 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x36b5c | load | 4 | `sp -0x74` | `-` |
| `stmib` at 0x36b68 | store | 4 | `sp -0x90` | `ret(0xfb618)` |
| `stmib` at 0x36b68 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x36b6c | load | 4 | `sp -0x74` | `-` |
| `ldmib` at 0x36b7c | load | 4 | `sp -0x90` | `-` |
| `ldmib` at 0x36b7c | load | 4 | `sp -0x8c` | `-` |
| `ldm` at 0x36b84 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36b84 | load | 4 | `(unknown)` | `-` |
| `stmib` at 0x36b98 | store | 4 | `sp -0x90` | `ret(0xfb30c)` |
| `stmib` at 0x36b98 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x36b9c | load | 4 | `sp -0x6c` | `-` |
| `stmda` at 0x36ba8 | store | 4 | `sp -0x98` | `ret(0xfb618)` |
| `stmda` at 0x36ba8 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x36bac | load | 4 | `sp -0x6c` | `-` |
| `ldmda` at 0x36bbc | load | 4 | `sp -0x98` | `-` |
| `ldmda` at 0x36bbc | load | 4 | `sp -0x94` | `-` |
| `ldm` at 0x36bc4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36bc4 | load | 4 | `(unknown)` | `-` |
| `stmda` at 0x36bd8 | store | 4 | `sp -0x98` | `ret(0xfb30c)` |
| `stmda` at 0x36bd8 | store | 4 | `sp -0x94` | `?` |
| `ldmib` at 0x36be0 | load | 4 | `sp -0x90` | `-` |
| `ldmib` at 0x36be0 | load | 4 | `sp -0x8c` | `-` |
| `ldmda` at 0x36be8 | load | 4 | `sp -0x98` | `-` |
| `ldmda` at 0x36be8 | load | 4 | `sp -0x94` | `-` |
| `ldm` at 0x36c04 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36c04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36c14 | load | 4 | `sp -0x88` | `-` |
| `stm` at 0x36c18 | store | 4 | `[sp-136] +0x0` | `ret(0xfb8e8)` |
| `stm` at 0x36c18 | store | 4 | `[sp-136] +0x4` | `?` |
| `ldm` at 0x36c30 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36c30 | load | 4 | `(unknown)` | `-` |
| `stm` at 0x36c34 | store | 4 | `(unknown)` | `?` |
| `stm` at 0x36c34 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x36c38 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x36c50 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36c50 | load | 4 | `(unknown)` | `-` |
| `stm` at 0x36c54 | store | 4 | `[sp-28] +0x10` | `?` |
| `stm` at 0x36c54 | store | 4 | `[sp-28] +0x14` | `?` |
| `ldr` at 0x36c58 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x36c70 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36c70 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36c78 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36c78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36c9c | load | 4 | `abs 0x36d0c` | `-` |
| `ldm` at 0x36ca0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x36ca0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x36cac | store | 4 | `sp -0x80` | `0x1` |
| `ldr` at 0x36cb4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x36cb8 | store | 4 | `sp -0x80` | `[sp-52]` |
| `str` at 0x36cc4 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x36cc8 | load | 4 | `sp -0x80` | `-` |
| `ldmdb` at 0x36ccc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x36ccc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36ccc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36ccc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3682c | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `r1` | `?` |
| `bl` at 0x36848 | `(unknown callee) 0xeae04` | `[sp-36]` | `0x1000` | `sp-76` | `sp-80` |
| `bl` at 0x36860 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x36870 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x368c0 | `(unknown callee) 0xe3cf8` | `ret(0xe3d64)` | `?` | `?` | `?` |
| `bl` at 0x368dc | `(unknown callee) 0xeae04` | `[sp-36]` | `[sp-40]` | `sp-84` | `sp-88` |
| `bl` at 0x368f4 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x3691c | `(unknown callee) 0xeae04` | `[sp-36]` | `[sp-44]` | `sp-92` | `sp-96` |
| `bl` at 0x36934 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x3695c | `(unknown callee) 0xeae04` | `[sp-36]` | `[sp-48]` | `sp-100` | `sp-104` |
| `bl` at 0x36974 | `(unknown callee) 0xe3d64` | `ret(0xeae04)` | `?` | `?` | `ret(0xeae04)` |
| `bl` at 0x36994 | `(unknown callee) 0xeaf08` | `[sp-36]` | `[sp-48]` | `0x0` | `0x0` |
| `bl` at 0x36998 | `(unknown callee) 0xe3d64` | `ret(0xeaf08)` | `?` | `?` | `?` |
| `bl` at 0x36b38 | `(unknown callee) 0x2079c` | `0x107594` | `sp-20` | `?` | `?` |
| `bl` at 0x36b60 | `(unknown callee) 0xfb618` | `[sp-116]` | `?` | `?` | `?` |
| `bl` at 0x36b88 | `(unknown callee) 0xfb30c` | `ret(0xfb618)` | `[sp-140]` | `?` | `?` |
| `bl` at 0x36ba0 | `(unknown callee) 0xfb618` | `[sp-108]` | `?` | `?` | `?` |
| `bl` at 0x36bc8 | `(unknown callee) 0xfb30c` | `ret(0xfb618)` | `[sp-148]` | `?` | `?` |
| `bl` at 0x36bec | `(unknown callee) 0xfb8e8` | `[sp-144]` | `[sp-140]` | `?` | `[sp-148]` |
| `bl` at 0x36c08 | `(unknown callee) 0xfb8e8` | `ret(0xfb8e8)` | `?` | `?` | `?` |
| `bl` at 0x36c7c | `(unknown callee) 0xfbb54` | `?` | `?` | `?` | `?` |
| `bl` at 0x36ca4 | `(unknown callee) 0x2079c` | `0x1075ac` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3685c | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x36880 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x368f0 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x36930 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x36970 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `bne` at 0x369a8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x369d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x36a30 | `[sp-132] == 0x0` | `[sp-132] != 0x0` |
| `bls` at 0x36aa4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x36ab0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x36ae4 | `[sp-72] == 0x0` | `[sp-72] != 0x0` |
| `bne` at 0x36b0c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x36b18 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x36b48 | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x36b74 | `[sp-116] >= 0x0 (signed, 32-bit)` | `[sp-116] < 0x0 (signed, 32-bit)` |
| `bge` at 0x36bb4 | `[sp-108] >= 0x0 (signed, 32-bit)` | `[sp-108] < 0x0 (signed, 32-bit)` |
| `beq` at 0x36c60 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bgt` at 0x36c88 | `ret(0xfbb54) > 0x0 (signed, 32-bit)` | `ret(0xfbb54) <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36ccc | `r0` | `?` |

## `LinkErrMonitor`

- `LinkErrMonitor`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36d10 to 0x3704c.
  Established. The unnamed `0x7aad8` call at `0x36ebc` carries `0x1`, `0x8100`, `0x0`, `sp-268`
  (`bl` at 0x36ebc). The later calls at `0x36eec` and `0x36fc8` carry `0x1`, `0x8101`, `0x0`,
  `sp-268` and `0x1`, `0x8102`, `0x0`, `sp-268` (`bl` at 0x36eec, `bl` at 0x36fc8). Notification
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36d20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x36d24 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x36d2c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x36d50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36d5c | load | 4 | `abs 0x37010` | `-` |
| `str` at 0x36d68 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x36d6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36d7c | load | 4 | `abs 0x37014` | `-` |
| `ldr` at 0x36d80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36d88 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x36d8c | load | 4 | `abs 0x37018` | `-` |
| `ldr` at 0x36d90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36d98 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x36d9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36da4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x36dac | load | 4 | `abs 0x3701c` | `-` |
| `str` at 0x36db4 | store | 4 | `abs 0x148c88` | `0x0` |
| `ldr` at 0x36db8 | load | 4 | `abs 0x37020` | `-` |
| `str` at 0x36dc0 | store | 4 | `abs 0x148c8c` | `0x0` |
| `ldr` at 0x36dc4 | load | 4 | `abs 0x37024` | `-` |
| `str` at 0x36dcc | store | 4 | `abs 0x148c90` | `0x0` |
| `str` at 0x36dd4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x36dd8 | load | 4 | `abs 0x37028` | `-` |
| `str` at 0x36de4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x36de8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x36df8 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x36e10 | store | 4 | `sp -0x20` | `ret(0x365bc)` |
| `ldr` at 0x36e14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36e24 | store | 4 | `sp -0x110` | `sp-268` |
| `ldr` at 0x36e28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36e30 | store | 4 | `sp -0x114` | `[sp-28]+65` |
| `ldrb` at 0x36e34 | load | 1 | `sp -0x43` | `-` |
| `ldr` at 0x36e40 | load | 4 | `abs 0x3702c` | `-` |
| `str` at 0x36e44 | store | 4 | `sp -0x118` | `0x1075f4` |
| `ldr` at 0x36e4c | load | 4 | `abs 0x37030` | `-` |
| `str` at 0x36e50 | store | 4 | `sp -0x118` | `0x1075f8` |
| `ldrb` at 0x36e54 | load | 1 | `sp -0x42` | `-` |
| `ldr` at 0x36e60 | load | 4 | `abs 0x3702c` | `-` |
| `str` at 0x36e64 | store | 4 | `sp -0x11c` | `0x1075f4` |
| `ldr` at 0x36e6c | load | 4 | `abs 0x37030` | `-` |
| `str` at 0x36e70 | store | 4 | `sp -0x11c` | `0x1075f8` |
| `ldr` at 0x36e74 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x36e78 | store | 4 | `sp -0x124` | `[sp-284]` |
| `ldr` at 0x36e7c | load | 4 | `sp -0x110` | `-` |
| `ldr` at 0x36e80 | load | 4 | `abs 0x37034` | `-` |
| `ldr` at 0x36e84 | load | 4 | `sp -0x114` | `-` |
| `ldr` at 0x36e88 | load | 4 | `sp -0x118` | `-` |
| `ldrb` at 0x36e90 | load | 1 | `sp -0x43` | `-` |
| `str` at 0x36ea4 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x36eac | store | 4 | `sp -0x120` | `0x0` |
| `ldrb` at 0x36ec0 | load | 1 | `sp -0x42` | `-` |
| `str` at 0x36ed4 | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x36edc | store | 4 | `sp -0x120` | `0x0` |
| `ldr` at 0x36ee4 | load | 4 | `abs 0x37038` | `-` |
| `ldr` at 0x36efc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36f04 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x36f10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36f14 | load | 4 | `abs 0x3703c` | `-` |
| `str` at 0x36f20 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x36f24 | load | 4 | `abs 0x14346c` | `-` |
| `str` at 0x36f34 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x36f3c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x36f40 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x36f50 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x36f74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x36f88 | load | 4 | `(unknown)` | `-` |
| `str` at 0x36f8c | store | 4 | `sp -0x124` | `?` |
| `ldr` at 0x36f90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36f98 | load | 4 | `abs 0x37040` | `-` |
| `str` at 0x36fac | store | 4 | `sp -0x124` | `0x1` |
| `str` at 0x36fb4 | store | 4 | `sp -0x120` | `0x0` |
| `ldr` at 0x36fbc | load | 4 | `abs 0x37044` | `-` |
| `ldr` at 0x36fd8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x36fe0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x36ffc | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x37000 | load | 4 | `abs 0x37048` | `-` |
| `ldr` at 0x37004 | load | 4 | `abs 0x143468` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36d30 | `(unknown callee) 0x37834` | `?` | `?` | `?` | `?` |
| `bl` at 0x36d40 | `(unknown callee) 0x3dce4` | `ret(0x37834)` | `?` | `?` | `ret(0x37834)` |
| `bl` at 0x36d60 | `(unknown callee) 0x202d0` | `0x1075d8` | `?` | `?` | `[sp-36]` |
| `bl` at 0x36ddc | `(unknown callee) 0x37898` | `0x1770` | `?` | `0x148c90` | `0x0` |
| `bl` at 0x36e08 | `(unknown callee) 0x365bc` | `[sp-28]:1` | `sp-68` | `sp-68` | `[sp-28]:1` |
| `bl` at 0x36e8c | `(unknown callee) 0x1b52c` | `sp-268` | `0x1075fc` | `[sp-28]+65` | `[sp-280]` |
| `bl` at 0x36ebc | `(unknown callee) 0x7aad8` | `0x1` | `0x8100` | `0x0` | `sp-268` |
| `bl` at 0x36eec | `(unknown callee) 0x7aad8` | `0x1` | `0x8101` | `0x0` | `sp-268` |
| `bl` at 0x36ef8 | `(unknown callee) 0x202d0` | `sp-268` | `?` | `?` | `sp-268` |
| `bl` at 0x36f60 | `(unknown callee) 0x367e4` | `[sp-28]:1` | `sp-64` | `sp-64` | `[sp-28]:1` |
| `bl` at 0x36fa0 | `(unknown callee) 0x1b52c` | `sp-268` | `0x107630` | `[sp-28]+65` | `?` |
| `bl` at 0x36fc8 | `(unknown callee) 0x7aad8` | `0x1` | `0x8102` | `0x0` | `sp-268` |
| `bl` at 0x36fd4 | `(unknown callee) 0x202d0` | `sp-268` | `?` | `?` | `sp-268` |
| `bl` at 0x36fe8 | `(unknown callee) 0x3dce4` | `?` | `?` | `?` | `?` |
| `bl` at 0x37008 | `(unknown callee) 0x37898` | `[0x143468]` | `?` | `?` | `0x143468` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36d3c | `ret(0x37834) == 0x0` | `ret(0x37834) != 0x0` |
| `beq` at 0x36d4c | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `beq` at 0x36d58 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `ble` at 0x36d74 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x36df0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x36e1c | `ret(0x365bc) == 0x0` | `ret(0x365bc) != 0x0` |
| `beq` at 0x36e3c | `[sp-67]:1 == 0x0` | `[sp-67]:1 != 0x0` |
| `beq` at 0x36e5c | `[sp-66]:1 == 0x0` | `[sp-66]:1 != 0x0` |
| `beq` at 0x36e98 | `[sp-67]:1 == 0x0` | `[sp-67]:1 != 0x0` |
| `beq` at 0x36ec8 | `[sp-66]:1 == 0x0` | `[sp-66]:1 != 0x0` |
| `blt` at 0x36f2c | `[sp-40] < [0x14346c] (signed, 32-bit)` | `[sp-40] >= [0x14346c] (signed, 32-bit)` |
| `ble` at 0x36f48 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x36f6c | `ret(0x367e4) != 0x1` | `ret(0x367e4) == 0x1` |
| `bne` at 0x36ff4 | `ret(0x3dce4) != 0x0` | `ret(0x3dce4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `LinkErrQuery`

- `LinkErrQuery`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3704c to 0x37748.
  Established. The unnamed `0x1b1b4` call carries `0x5dc`, `r1`, `r2`, `r3` (`bl` at 0x37068); its
  result is assigned as a word through `r0 +0x0` (`str` at 0x37070). The absolute word at `0x143464`
  is read (`ldr` at 0x37100). Query field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3705c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x37060 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x37064 | load | 4 | `abs 0x376c8` | `-` |
| `str` at 0x37070 | store | 4 | `r0 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x37074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x37078 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x3707c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x37080 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x37088 | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0x3708c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3709c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x370a4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x370bc | load | 4 | `abs 0x376cc` | `-` |
| `str` at 0x370c0 | store | 4 | `sp -0x48` | `0x10764c` |
| `ldr` at 0x370c8 | load | 4 | `abs 0x376d0` | `-` |
| `str` at 0x370cc | store | 4 | `sp -0x48` | `0x107654` |
| `ldr` at 0x370d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x370d4 | load | 4 | `abs 0x376d4` | `-` |
| `ldr` at 0x370d8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x370e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x370f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x370f8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x370fc | load | 4 | `abs 0x376d8` | `-` |
| `ldr` at 0x37100 | load | 4 | `abs 0x143464` | `-` |
| `ldr` at 0x3710c | load | 4 | `abs 0x376dc` | `-` |
| `str` at 0x37110 | store | 4 | `sp -0x4c` | `0x10767c` |
| `ldr` at 0x37118 | load | 4 | `abs 0x376e0` | `-` |
| `str` at 0x3711c | store | 4 | `sp -0x4c` | `0x107684` |
| `ldr` at 0x37120 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37124 | load | 4 | `abs 0x376e4` | `-` |
| `ldr` at 0x37128 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x37130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37140 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37148 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x3714c | load | 4 | `abs 0x376e8` | `-` |
| `ldr` at 0x37150 | load | 4 | `abs 0x143468` | `-` |
| `str` at 0x37170 | store | 4 | `sp -0x24` | `ret(0xfbfb0)` |
| `ldr` at 0x37174 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37178 | load | 4 | `abs 0x376ec` | `-` |
| `ldr` at 0x37180 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37190 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37198 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x3719c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x371a0 | load | 4 | `abs 0x376f0` | `-` |
| `ldr` at 0x371b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x371bc | load | 4 | `abs 0x376f0` | `-` |
| `ldr` at 0x371d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x371dc | load | 4 | `abs 0x376f4` | `-` |
| `ldr` at 0x371f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x371f4 | load | 4 | `abs 0x376f8` | `-` |
| `ldr` at 0x3720c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37210 | load | 4 | `abs 0x376f8` | `-` |
| `ldr` at 0x3722c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37230 | load | 4 | `abs 0x376fc` | `-` |
| `ldr` at 0x37244 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37254 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37258 | load | 4 | `abs 0x37700` | `-` |
| `ldr` at 0x37268 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37278 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37280 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x37284 | load | 4 | `abs 0x376e8` | `-` |
| `ldr` at 0x37288 | load | 4 | `abs 0x37704` | `-` |
| `ldr` at 0x3728c | load | 4 | `abs 0x143468` | `-` |
| `ldr` at 0x37290 | load | 4 | `abs 0x14346c` | `-` |
| `str` at 0x372b4 | store | 4 | `sp -0x28` | `ret(0xfbfb0)` |
| `ldr` at 0x372b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x372bc | load | 4 | `abs 0x37708` | `-` |
| `ldr` at 0x372c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x372d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x372dc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x372e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x372e4 | load | 4 | `abs 0x376f0` | `-` |
| `ldr` at 0x372fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37300 | load | 4 | `abs 0x376f0` | `-` |
| `ldr` at 0x3731c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37320 | load | 4 | `abs 0x376f4` | `-` |
| `ldr` at 0x37334 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37338 | load | 4 | `abs 0x376f8` | `-` |
| `ldr` at 0x37350 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37354 | load | 4 | `abs 0x376f8` | `-` |
| `ldr` at 0x37370 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37374 | load | 4 | `abs 0x376fc` | `-` |
| `ldr` at 0x37388 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3738c | load | 4 | `abs 0x3770c` | `-` |
| `ldr` at 0x373a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x373a8 | load | 4 | `abs 0x376f0` | `-` |
| `ldr` at 0x373c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x373c8 | load | 4 | `abs 0x376f4` | `-` |
| `ldr` at 0x373dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x373ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x373f0 | load | 4 | `abs 0x37700` | `-` |
| `ldr` at 0x37400 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37410 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37418 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x3741c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37420 | load | 4 | `abs 0x37710` | `-` |
| `ldr` at 0x37428 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37438 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37440 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x37444 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37448 | load | 4 | `abs 0x37714` | `-` |
| `ldr` at 0x37450 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37460 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37468 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x3746c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37470 | load | 4 | `abs 0x37718` | `-` |
| `ldr` at 0x37478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37488 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37490 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x37494 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37498 | load | 4 | `abs 0x3771c` | `-` |
| `ldr` at 0x374a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x374b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x374b8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `str` at 0x374c0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x374c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x374d4 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x374e8 | load | 1 | `sp -0x44` | `-` |
| `ldr` at 0x374f4 | load | 4 | `abs 0x37720` | `-` |
| `str` at 0x374f8 | store | 4 | `sp -0x5c` | `0x1077b4` |
| `ldr` at 0x374fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37500 | load | 4 | `abs 0x37724` | `-` |
| `ldr` at 0x37504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37508 | load | 4 | `abs 0x37720` | `-` |
| `ldrb` at 0x37514 | load | 1 | `sp -0x43` | `-` |
| `ldrb` at 0x37518 | load | 1 | `sp -0x42` | `-` |
| `str` at 0x3751c | store | 4 | `sp -0x5c` | `[sp-66]:1` |
| `ldr` at 0x37520 | load | 4 | `abs 0x37728` | `-` |
| `ldr` at 0x37524 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37528 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3752c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x37530 | load | 4 | `abs 0x3772c` | `-` |
| `ldr` at 0x37534 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37538 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3753c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x37540 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37544 | load | 4 | `abs 0x37730` | `-` |
| `ldr` at 0x37548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37554 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37564 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3756c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x37570 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37578 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x37580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37584 | load | 4 | `abs 0x37734` | `-` |
| `ldr` at 0x3758c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3759c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x375a4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x375a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x375ac | load | 4 | `abs 0x37738` | `-` |
| `ldr` at 0x375b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x375c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x375cc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x375d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x375d4 | load | 4 | `abs 0x3773c` | `-` |
| `ldr` at 0x375dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x375ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x375f4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `str` at 0x375fc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x37600 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x37610 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x37624 | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x37630 | load | 4 | `abs 0x37720` | `-` |
| `str` at 0x37634 | store | 4 | `sp -0x5c` | `0x1077b4` |
| `ldr` at 0x37638 | load | 4 | `abs 0x37720` | `-` |
| `str` at 0x3763c | store | 4 | `sp -0x58` | `0x1077b4` |
| `ldr` at 0x37640 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37644 | load | 4 | `abs 0x37740` | `-` |
| `ldr` at 0x37648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3764c | load | 4 | `abs 0x37720` | `-` |
| `ldr` at 0x37658 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3765c | store | 4 | `sp -0x5c` | `[sp-52]` |
| `ldr` at 0x37660 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x37664 | store | 4 | `sp -0x58` | `[sp-60]` |
| `ldm` at 0x3767c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3767c | load | 4 | `(unknown)` | `-` |
| `stm` at 0x37680 | store | 4 | `sp -0x54` | `?` |
| `stm` at 0x37680 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x37684 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37688 | load | 4 | `abs 0x37744` | `-` |
| `ldr` at 0x3768c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37690 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x37698 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x376a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x376b0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x376b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x376bc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x376c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x376c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x376c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x376c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37068 | `(unknown callee) 0x1b1b4` | `0x5dc` | `r1` | `r2` | `r3` |
| `bl` at 0x37090 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `[[sp-24]]` | `0x0` |
| `bl` at 0x370ac | `(unknown callee) 0x1f26c` | `0x9b` | `?` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x370dc | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x107660` | `[sp-72]` | `?` |
| `bl` at 0x370e4 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3712c | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x10768c` | `[sp-76]` | `?` |
| `bl` at 0x37134 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37168 | `(unknown callee) 0xfbfb0` | `?` | `?` | `[0x143468]` | `?` |
| `bl` at 0x3717c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076ac` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x37184 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x371a4 | `(unknown callee) 0xfc32c` | `[sp-36]` | `0x476a6000` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x371c0 | `(unknown callee) 0xfc18c` | `[sp-36]` | `0x476a6000` | `?` | `ret(0xfc32c)` |
| `bl` at 0x371cc | `(unknown callee) 0xfb644` | `ret(0xfc18c)` | `?` | `?` | `ret(0xfc18c)` |
| `bl` at 0x371e8 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076d0` | `ret(0xfb644)` | `?` |
| `bl` at 0x371f8 | `(unknown callee) 0xfc32c` | `[sp-36]` | `0x447a0000` | `?` | `ret(0xfc32c)` |
| `bl` at 0x37214 | `(unknown callee) 0xfc18c` | `[sp-36]` | `0x447a0000` | `?` | `ret(0xfc32c)` |
| `bl` at 0x37220 | `(unknown callee) 0xfb644` | `ret(0xfc18c)` | `?` | `?` | `ret(0xfc18c)` |
| `bl` at 0x3723c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076e0` | `ret(0xfb644)` | `?` |
| `bl` at 0x37248 | `(unknown callee) 0xfb644` | `[sp-36]` | `?` | `?` | `ret(0xfc32c)` |
| `bl` at 0x37264 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076f0` | `ret(0xfb644)` | `?` |
| `bl` at 0x3726c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x372ac | `(unknown callee) 0xfbfb0` | `?` | `[0x143468]` | `?` | `?` |
| `bl` at 0x372c0 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076fc` | `?` | `ret(0xfbfb0)` |
| `bl` at 0x372c8 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x372e8 | `(unknown callee) 0xfc32c` | `[sp-40]` | `0x476a6000` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x37304 | `(unknown callee) 0xfc18c` | `[sp-40]` | `0x476a6000` | `?` | `ret(0xfc32c)` |
| `bl` at 0x37310 | `(unknown callee) 0xfb644` | `ret(0xfc18c)` | `?` | `?` | `ret(0xfc18c)` |
| `bl` at 0x3732c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076d0` | `ret(0xfb644)` | `?` |
| `bl` at 0x3733c | `(unknown callee) 0xfc32c` | `[sp-40]` | `0x447a0000` | `?` | `ret(0xfc32c)` |
| `bl` at 0x37358 | `(unknown callee) 0xfc18c` | `[sp-40]` | `0x447a0000` | `?` | `ret(0xfc32c)` |
| `bl` at 0x37364 | `(unknown callee) 0xfb644` | `ret(0xfc18c)` | `?` | `?` | `ret(0xfc18c)` |
| `bl` at 0x37380 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076e0` | `ret(0xfb644)` | `?` |
| `bl` at 0x37390 | `(unknown callee) 0xfc33c` | `[sp-40]` | `0x0` | `?` | `ret(0xfc32c)` |
| `bl` at 0x373ac | `(unknown callee) 0xfc18c` | `[sp-36]` | `0x476a6000` | `?` | `ret(0xfc33c)` |
| `bl` at 0x373b8 | `(unknown callee) 0xfb644` | `ret(0xfc18c)` | `?` | `?` | `ret(0xfc18c)` |
| `bl` at 0x373d4 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076d0` | `ret(0xfb644)` | `?` |
| `bl` at 0x373e0 | `(unknown callee) 0xfb644` | `[sp-40]` | `?` | `?` | `ret(0xfc33c)` |
| `bl` at 0x373fc | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1076f0` | `ret(0xfb644)` | `?` |
| `bl` at 0x37404 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37424 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x10771c` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x3742c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3744c | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x107738` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x37454 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37474 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x107764` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x3747c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3749c | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x10778c` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x374a4 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x374e4 | `(unknown callee) 0x365bc` | `[sp-28]:1` | `sp-68` | `sp-68` | `[sp-28]:1` |
| `bl` at 0x3750c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1077c0` | `[sp-28]` | `0x1077b4` |
| `bl` at 0x37550 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1077d8` | `[sp-28]` | `[sp-67]:1` |
| `bl` at 0x37558 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37588 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x1077f8` | `ret(0x1b2bc)` | `?` |
| `bl` at 0x37590 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x375b0 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x107818` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x375b8 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x375d8 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x107850` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x375e0 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37620 | `(unknown callee) 0x367e4` | `[sp-28]:1` | `sp-64` | `sp-64` | `[sp-28]:1` |
| `bl` at 0x37650 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x107888` | `[sp-28]` | `0x1077b4` |
| `bl` at 0x37694 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1078a0` | `[sp-28]` | `[sp-56]` |
| `bl` at 0x3769c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x370b8 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x37108 | `[0x143464] == 0x0` | `[0x143464] != 0x0` |
| `bge` at 0x371b0 | `ret(0xfc32c) >= 0x0 (signed, 32-bit)` | `ret(0xfc32c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x37204 | `ret(0xfc32c) >= 0x0 (signed, 32-bit)` | `ret(0xfc32c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x372f4 | `ret(0xfc32c) >= 0x0 (signed, 32-bit)` | `ret(0xfc32c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x37348 | `ret(0xfc32c) >= 0x0 (signed, 32-bit)` | `ret(0xfc32c) < 0x0 (signed, 32-bit)` |
| `beq` at 0x3739c | `ret(0xfc33c) == 0x0` | `ret(0xfc33c) != 0x0` |
| `ble` at 0x374cc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x374f0 | `[sp-68]:1 == 0x0` | `[sp-68]:1 != 0x0` |
| `ble` at 0x37608 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x3762c | `[sp-64]:1 == 0x0` | `[sp-64]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x376c4 | `r0` | `ret(0x1b2bc)` |

## `LinkErrTaskInit`

- `LinkErrTaskInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37748 to 0x377c0.
  Established. The unnamed `0x229b0` call carries `0x148c94`, `0x1078c8`, `0x0`, `0x9` (`bl` at
  0x37768). The unnamed `0x22480` call carries `0x148ce8`, `0x1078d4`, `0x36d10`, `0x1` (`bl` at
  0x377a4). Task setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x37758 | load | 4 | `abs 0x377ac` | `-` |
| `ldr` at 0x3775c | load | 4 | `abs 0x377b0` | `-` |
| `str` at 0x37770 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x37778 | store | 4 | `sp -0x20` | `0x800` |
| `str` at 0x37780 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x37788 | store | 4 | `sp -0x18` | `0xa` |
| `str` at 0x37790 | store | 4 | `sp -0x14` | `0x9` |
| `ldr` at 0x37794 | load | 4 | `abs 0x377b4` | `-` |
| `ldr` at 0x37798 | load | 4 | `abs 0x377b8` | `-` |
| `ldr` at 0x3779c | load | 4 | `abs 0x377bc` | `-` |
| `ldmdb` at 0x377a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x377a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x377a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37768 | `(unknown callee) 0x229b0` | `0x148c94` | `0x1078c8` | `0x0` | `0x9` |
| `bl` at 0x377a4 | `(unknown callee) 0x22480` | `0x148ce8` | `0x1078d4` | `0x36d10` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x377a8 | `r0` | `ret(0x22480)` |

## `LinkErrChangeFreq`

- `LinkErrChangeFreq`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x377c0 to 0x37834.
  Established. The absolute word assignment at `0x143468` writes `r0` (`str` at 0x377dc). The signed
  condition `[0x143468] <= 0x176f` takes `0x37804`, while greater values fall through (`ble` at
  0x377f0). The unnamed `0xfb0a8` call carries `0x1770`, `[0x143468]`, `[0x143468]`, `0x143468`
  (`bl` at 0x37814). Frequency units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x377d0 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x377d4 | load | 4 | `abs 0x37824` | `-` |
| `ldr` at 0x377d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x377dc | store | 4 | `abs 0x143468` | `r0` |
| `ldr` at 0x377e0 | load | 4 | `abs 0x37824` | `-` |
| `ldr` at 0x377e4 | load | 4 | `abs 0x143468` | `-` |
| `ldr` at 0x377e8 | load | 4 | `abs 0x37828` | `-` |
| `ldr` at 0x377f4 | load | 4 | `abs 0x3782c` | `-` |
| `str` at 0x377fc | store | 4 | `abs 0x14346c` | `0x0` |
| `ldr` at 0x37804 | load | 4 | `abs 0x3782c` | `-` |
| `ldr` at 0x37808 | load | 4 | `abs 0x37824` | `-` |
| `ldr` at 0x3780c | load | 4 | `abs 0x37830` | `-` |
| `ldr` at 0x37810 | load | 4 | `abs 0x143468` | `-` |
| `str` at 0x3781c | store | 4 | `abs 0x14346c` | `ret(0xfb0a8)` |
| `ldmdb` at 0x37820 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x37820 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x37820 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x37820 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37814 | `(unknown callee) 0xfb0a8` | `0x1770` | `[0x143468]` | `[0x143468]` | `0x143468` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x377f0 | `[0x143468] <= 0x176f (signed, 32-bit)` | `[0x143468] > 0x176f (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x37820 | `r0` | `?` |

## `LinkErrActive`

- `LinkErrActive`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37834 to 0x37898.
  Established. The unnamed `0x1f26c` call carries `0x9b`, `r1`, `r2`, `r3` (`bl` at 0x37848). The
  absolute byte at `0x143464` is read (`ldrb` at 0x37858). The zero-byte condition `([0x143464]:1 & 0xff) == 0x0` takes `0x37884`, while nonzero falls through (`beq` at 0x37874). Monitor activation
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x37850 | store | 1 | `sp -0x11` | `(ret(0x1f26c) & 0xff)` |
| `ldr` at 0x37854 | load | 4 | `abs 0x37894` | `-` |
| `ldrb` at 0x37858 | load | 1 | `abs 0x143464` | `-` |
| `strb` at 0x3785c | store | 1 | `sp -0x12` | `([0x143464]:1 & 0xff)` |
| `ldrb` at 0x37860 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x3786c | load | 1 | `sp -0x12` | `-` |
| `str` at 0x3787c | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x37888 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3788c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x37890 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x37890 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x37890 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37848 | `(unknown callee) 0x1f26c` | `0x9b` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x37868 | `(ret(0x1f26c) & 0xff) == 0x0` | `(ret(0x1f26c) & 0xff) != 0x0` |
| `beq` at 0x37874 | `([0x143464]:1 & 0xff) == 0x0` | `([0x143464]:1 & 0xff) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x37890 | `r0` | `?` |

## `promIceInit`

- `promIceInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44e60 to 0x44f2c.
  Established. The absolute word at `0x2b14c0` receives `ret(0x1b454)` (`str` at 0x44eb8). The
  unnamed `0x1b4cc` call carries `[0x2b14c0]`, `0x1`, `[sp-28]`, `0x2b14c0` (`bl` at 0x44ef4). The
  absolute word assignment at `0x2b14a0` writes `[sp-24]` (`str` at 0x44f00). PROM setup
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x44e70 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x44e74 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x44e78 | load | 4 | `abs 0x44f14` | `-` |
| `ldr` at 0x44e7c | load | 4 | `abs 0x2b14a0` | `-` |
| `ldr` at 0x44e88 | load | 4 | `abs 0x44f18` | `-` |
| `str` at 0x44e8c | store | 4 | `sp -0x20` | `0x109bf8` |
| `ldr` at 0x44e94 | load | 4 | `abs 0x44f1c` | `-` |
| `ldr` at 0x44ea4 | load | 4 | `abs 0x44f20` | `-` |
| `ldr` at 0x44ea8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x44eb8 | store | 4 | `abs 0x2b14c0` | `ret(0x1b454)` |
| `ldr` at 0x44ebc | load | 4 | `abs 0x2b14c0` | `-` |
| `ldr` at 0x44ec8 | load | 4 | `abs 0x44f24` | `-` |
| `str` at 0x44ecc | store | 4 | `sp -0x20` | `0x109c34` |
| `ldr` at 0x44ed4 | load | 4 | `abs 0x44f1c` | `-` |
| `ldr` at 0x44ee4 | load | 4 | `abs 0x44f20` | `-` |
| `ldr` at 0x44ee8 | load | 4 | `abs 0x2b14c0` | `-` |
| `ldr` at 0x44ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x44ef8 | load | 4 | `abs 0x44f14` | `-` |
| `ldr` at 0x44efc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x44f00 | store | 4 | `abs 0x2b14a0` | `[sp-24]` |
| `ldr` at 0x44f04 | load | 4 | `abs 0x44f28` | `-` |
| `ldr` at 0x44f08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x44f0c | store | 4 | `abs 0x2b1480` | `[sp-28]` |
| `ldmdb` at 0x44f10 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x44f10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44f10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44f10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44ea0 | `(unknown callee) 0x21eb8` | `0x9` | `0x109c1c` | `0x39` | `0x0` |
| `bl` at 0x44eb0 | `(unknown callee) 0x1b454` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x44ee0 | `(unknown callee) 0x21eb8` | `0x9` | `0x109c1c` | `0x40` | `0x0` |
| `bl` at 0x44ef4 | `(unknown callee) 0x1b4cc` | `[0x2b14c0]` | `0x1` | `[sp-28]` | `0x2b14c0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x44e84 | `[0x2b14a0] == 0x0` | `[0x2b14a0] != 0x0` |
| `bne` at 0x44ec4 | `[0x2b14c0] != 0x0` | `[0x2b14c0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44f10 | `r0` | `ret(0x1b4cc)` |

## `promIceNotPresent`

- `promIceNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44f2c to 0x44f7c.
  Established. The signed condition `(r0 & 0xff) >= [0x2b1480]` takes `0x44f70`, while smaller
  values fall through (`bge` at 0x44f54). The byte assignment at `((r0 & 0xff) + [0x2b14c0]) +0x0`
  writes `0x0` (`strb` at 0x44f6c). Presence-state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44f40 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x44f44 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x44f48 | load | 4 | `abs 0x44f74` | `-` |
| `ldr` at 0x44f4c | load | 4 | `abs 0x2b1480` | `-` |
| `ldrb` at 0x44f58 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x44f5c | load | 4 | `abs 0x44f78` | `-` |
| `ldr` at 0x44f60 | load | 4 | `abs 0x2b14c0` | `-` |
| `strb` at 0x44f6c | store | 1 | `((r0 & 0xff) + [0x2b14c0]) +0x0` | `0x0` |
| `ldmdb` at 0x44f70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44f70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44f70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x44f54 | `(r0 & 0xff) >= [0x2b1480] (signed, 32-bit)` | `(r0 & 0xff) < [0x2b1480] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44f70 | `r0` | `r0` |

## `promIceGetCfg`

- `promIceGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x450ac to 0x450e8.
  Established. The absolute word at `0x2b14a0` is read (`ldr` at 0x450c8); its expression
  `[0x2b14a0]` is assigned through `r0 +0x0` (`str` at 0x450cc). The return reports `[0x2b1480]`
  (`ldmdb` at 0x450dc). Configuration contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x450bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x450c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x450c4 | load | 4 | `abs 0x450e0` | `-` |
| `ldr` at 0x450c8 | load | 4 | `abs 0x2b14a0` | `-` |
| `str` at 0x450cc | store | 4 | `r0 +0x0` | `[0x2b14a0]` |
| `ldr` at 0x450d0 | load | 4 | `abs 0x450e4` | `-` |
| `ldr` at 0x450d4 | load | 4 | `abs 0x2b1480` | `-` |
| `ldmdb` at 0x450dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x450dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x450dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x450dc | `r0` | `[0x2b1480]` |

## `promIceCount`

- `promIceCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x450e8 to 0x45108.
  Established. The four-byte load reads `0x2b1480` (`ldr` at 0x450f8). The reported return value is
  `[0x2b1480]` (`ldmdb` at 0x45100). Count interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x450f4 | load | 4 | `abs 0x45104` | `-` |
| `ldr` at 0x450f8 | load | 4 | `abs 0x2b1480` | `-` |
| `ldmdb` at 0x45100 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45100 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45100 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45100 | `r0` | `[0x2b1480]` |

## `promIcePresent`

- `promIcePresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45108 to 0x45168.
  Established. The byte read uses `((r0 & 0xff) + [0x2b14c0]) +0x0` (`ldrb` at 0x45144); its
  expression `[((r0 & 0xff) + [0x2b14c0])]:1` is stored as a word at `sp -0x18` (`str` at 0x45148).
  Presence flags remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4511c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x45120 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x45124 | load | 4 | `abs 0x45160` | `-` |
| `ldr` at 0x45128 | load | 4 | `abs 0x2b1480` | `-` |
| `ldrb` at 0x45134 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x45138 | load | 4 | `abs 0x45164` | `-` |
| `ldr` at 0x4513c | load | 4 | `abs 0x2b14c0` | `-` |
| `ldrb` at 0x45144 | load | 1 | `((r0 & 0xff) + [0x2b14c0]) +0x0` | `-` |
| `str` at 0x45148 | store | 4 | `sp -0x18` | `[((r0 & 0xff) + [0x2b14c0])]:1` |
| `str` at 0x45154 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x45158 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4515c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4515c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4515c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x45130 | `(r0 & 0xff) >= [0x2b1480] (signed, 32-bit)` | `(r0 & 0xff) < [0x2b1480] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4515c | `r0` | `?` |

## `promIceEnabled`

- `promIceEnabled`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45168 to 0x4527c.
  Established. The unnamed `0xd74b0` call carries `0x0`, `[[sp-24]]:1`, `[[sp-24]+1]:1`,
  `[[sp-24]+1]:1` (`bl` at 0x451dc); its zero result takes `0x4520c`, while nonzero falls through
  (`beq` at 0x451f8). The word assignment at `sp -0x20` writes `0x1` (`str` at 0x45210). Enable
  polarity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4517c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x45180 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x45198 | store | 4 | `sp -0x1c` | `ret(0x45584)` |
| `ldr` at 0x4519c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x451a8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x451c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x451c4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x451c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x451cc | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0x451e4 | store | 4 | `sp -0x1c` | `ret(0xd74b0)` |
| `ldr` at 0x451e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x451ec | store | 4 | `sp -0x24` | `ret(0xd74b0)` |
| `ldr` at 0x451f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x451fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x45210 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x4521c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x45224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45230 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4523c | load | 4 | `abs 0x45274` | `-` |
| `str` at 0x45240 | store | 4 | `sp -0x30` | `0x109c70` |
| `ldrb` at 0x45244 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x45248 | store | 4 | `sp -0x2c` | `[sp-17]:1` |
| `str` at 0x4524c | store | 4 | `sp -0x28` | `ret(0x21a38)` |
| `ldr` at 0x45254 | load | 4 | `abs 0x45278` | `-` |
| `str` at 0x45268 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4526c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x45270 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45270 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45190 | `(unknown callee) 0x45584` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x451b0 | `(unknown callee) 0x45108` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x451dc | `(unknown callee) 0xd74b0` | `0x0` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `[[sp-24]+1]:1` |
| `bl` at 0x45234 | `(unknown callee) 0x21a38` | `?` | `?` | `?` | `?` |
| `bl` at 0x45260 | `(unknown callee) 0x21eb8` | `0x1` | `0x109c1c` | `0xa9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x451a4 | `ret(0x45584) != 0x0` | `ret(0x45584) == 0x0` |
| `beq` at 0x451bc | `ret(0x45108) == 0x0` | `ret(0x45108) != 0x0` |
| `beq` at 0x451f8 | `ret(0xd74b0) == 0x0` | `ret(0xd74b0) != 0x0` |
| `beq` at 0x45204 | `ret(0xd74b0) == 0x1` | `ret(0xd74b0) != 0x1` |
| `beq` at 0x4522c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45270 | `r0` | `?` |

## `promIceEnable`

- `promIceEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4527c to 0x45350.
  Established. The non-`0xff` entry-byte condition `(r0 & 0xff) != 0xff` takes `0x45330`, while
  equality falls through (`bne` at 0x452a4). The unnamed `0x45424` call at `0x45338` carries `(r0 & 0xff)`, `r1`, `r2`, `(r0 & 0xff)` (`bl` at 0x45338). PROM enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x45290 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x45298 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x4529c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x452ac | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x452b0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x452b4 | load | 4 | `abs 0x4534c` | `-` |
| `ldr` at 0x452b8 | load | 4 | `abs 0x2b1480` | `-` |
| `ldrb` at 0x452c8 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x452e0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x452f0 | store | 4 | `sp -0x1c` | `ret(0x45424)` |
| `ldr` at 0x452f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45300 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4530c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45310 | store | 4 | `sp -0x18` | `ret(0x45424)` |
| `ldrb` at 0x45314 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x4531c | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x45324 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45328 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x45330 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x45340 | store | 4 | `sp -0x24` | `ret(0x45424)` |
| `ldr` at 0x45344 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x45348 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45348 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45348 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x452d0 | `(unknown callee) 0x45108` | `?` | `?` | `[0x2b1480]` | `?` |
| `bl` at 0x452e8 | `(unknown callee) 0x45424` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x45338 | `(unknown callee) 0x45424` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x452a4 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x452c0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x452dc | `ret(0x45108) == 0x0` | `ret(0x45108) != 0x0` |
| `beq` at 0x452fc | `ret(0x45424) == 0x0` | `ret(0x45424) != 0x0` |
| `bne` at 0x45308 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45348 | `r0` | `?` |

## `promIceDisable`

- `promIceDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45350 to 0x45424.
  Established. The unnamed `0x454d4` call at `0x4540c` carries `(r0 & 0xff)`, `r1`, `r2`, `(r0 & 0xff)` (`bl` at 0x4540c). The byte assignment at `sp -0x1d` writes `([sp-29]:1+1 & 0xff)` (`strb`
  at 0x453f0). PROM disable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x45364 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4536c | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x45370 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x45380 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x45384 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x45388 | load | 4 | `abs 0x45420` | `-` |
| `ldr` at 0x4538c | load | 4 | `abs 0x2b1480` | `-` |
| `ldrb` at 0x4539c | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x453b4 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x453c4 | store | 4 | `sp -0x1c` | `ret(0x454d4)` |
| `ldr` at 0x453c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x453d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x453e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x453e4 | store | 4 | `sp -0x18` | `ret(0x454d4)` |
| `ldrb` at 0x453e8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x453f0 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x453f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x453fc | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x45404 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x45414 | store | 4 | `sp -0x24` | `ret(0x454d4)` |
| `ldr` at 0x45418 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4541c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4541c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4541c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x453a4 | `(unknown callee) 0x45108` | `?` | `?` | `[0x2b1480]` | `?` |
| `bl` at 0x453bc | `(unknown callee) 0x454d4` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x4540c | `(unknown callee) 0x454d4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x45378 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x45394 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x453b0 | `ret(0x45108) == 0x0` | `ret(0x45108) != 0x0` |
| `beq` at 0x453d0 | `ret(0x454d4) == 0x0` | `ret(0x454d4) != 0x0` |
| `bne` at 0x453dc | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4541c | `r0` | `?` |
