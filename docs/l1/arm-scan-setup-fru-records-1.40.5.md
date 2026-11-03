# ARM scan setup and FRU record helpers in 1.40.5

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/scan-setup-fru-records-facts.txt) retain all tool sections.

## `setup_tdrdata_tio`

- `setup_tdrdata_tio`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf28c0 to 0xf2f80.
  Established. The unnamed `0x1b1b4` call carries `0x44` and three unknown candidates (`bl` at 0xf28ec); its result is assigned to the word at `[sp-24] +0x28` (`str` at 0xf28f4). The unnamed
  `0xf1164` call at `0xf2960` carries `0x13899c`, `0x7c`, `0x145584`, unknown (`bl` at 0xf2960). TIO
  scan-data setup policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf28d0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf28d8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf28dc | load | 4 | `abs 0xf2f04` | `-` |
| `ldr` at 0xf28e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf28f4 | store | 4 | `[sp-24] +0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xf28f8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2908 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xf2914 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf2918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2928 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf292c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2934 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2940 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf2944 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf294c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf2954 | load | 4 | `abs 0xf2f08` | `-` |
| `ldr` at 0xf295c | load | 4 | `abs 0xf2f0c` | `-` |
| `ldr` at 0xf2968 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2970 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf297c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2984 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf298c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2990 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf29a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf29a8 | load | 4 | `abs 0xf2f10` | `-` |
| `ldr` at 0xf29b0 | load | 4 | `abs 0xf2f14` | `-` |
| `ldr` at 0xf29bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf29c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf29d0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf29d8 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf29e0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf29e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf29f4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf29fc | load | 4 | `abs 0xf2f18` | `-` |
| `ldr` at 0xf2a04 | load | 4 | `abs 0xf2f1c` | `-` |
| `ldr` at 0xf2a10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2a18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2a24 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2a2c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2a34 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2a38 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2a48 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2a50 | load | 4 | `abs 0xf2f20` | `-` |
| `ldr` at 0xf2a58 | load | 4 | `abs 0xf2f24` | `-` |
| `ldr` at 0xf2a64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2a6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2a78 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2a80 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2a88 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2a8c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2a9c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2aa4 | load | 4 | `abs 0xf2f28` | `-` |
| `ldr` at 0xf2aac | load | 4 | `abs 0xf2f2c` | `-` |
| `ldr` at 0xf2ab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2ac0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2acc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2ad4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2adc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2ae0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2af0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2af8 | load | 4 | `abs 0xf2f30` | `-` |
| `ldr` at 0xf2b00 | load | 4 | `abs 0xf2f34` | `-` |
| `ldr` at 0xf2b0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2b14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2b20 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2b28 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2b30 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2b34 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2b44 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2b4c | load | 4 | `abs 0xf2f38` | `-` |
| `ldr` at 0xf2b54 | load | 4 | `abs 0xf2f3c` | `-` |
| `ldr` at 0xf2b60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2b68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2b74 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2b7c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2b84 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2b88 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2b98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2ba0 | load | 4 | `abs 0xf2f40` | `-` |
| `ldr` at 0xf2ba8 | load | 4 | `abs 0xf2f44` | `-` |
| `ldr` at 0xf2bb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2bbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2bc8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2bd0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2bd8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2bdc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2bec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2bf4 | load | 4 | `abs 0xf2f48` | `-` |
| `ldr` at 0xf2bfc | load | 4 | `abs 0xf2f4c` | `-` |
| `ldr` at 0xf2c08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2c10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2c1c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2c24 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2c2c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2c30 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2c40 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2c48 | load | 4 | `abs 0xf2f50` | `-` |
| `ldr` at 0xf2c50 | load | 4 | `abs 0xf2f54` | `-` |
| `ldr` at 0xf2c5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2c64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2c70 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2c78 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2c80 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2c84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2c94 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2c9c | load | 4 | `abs 0xf2f58` | `-` |
| `ldr` at 0xf2ca4 | load | 4 | `abs 0xf2f54` | `-` |
| `ldr` at 0xf2cb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2cb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2cc4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2ccc | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2cd4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2cd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2ce8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2cf0 | load | 4 | `abs 0xf2f5c` | `-` |
| `ldr` at 0xf2cf8 | load | 4 | `abs 0xf2f60` | `-` |
| `ldr` at 0xf2d04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2d0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2d18 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2d20 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2d28 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2d2c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2d3c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2d44 | load | 4 | `abs 0xf2f64` | `-` |
| `ldr` at 0xf2d4c | load | 4 | `abs 0xf2f60` | `-` |
| `ldr` at 0xf2d58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2d60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2d6c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2d74 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2d7c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2d80 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2d90 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2d98 | load | 4 | `abs 0xf2f68` | `-` |
| `ldr` at 0xf2da0 | load | 4 | `abs 0xf2f6c` | `-` |
| `ldr` at 0xf2dac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2db4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2dc0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2dc8 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2dd0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2dd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2de4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2dec | load | 4 | `abs 0xf2f70` | `-` |
| `ldr` at 0xf2df4 | load | 4 | `abs 0xf2f6c` | `-` |
| `ldr` at 0xf2e00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2e08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2e14 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2e1c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2e24 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2e28 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2e38 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2e40 | load | 4 | `abs 0xf2f74` | `-` |
| `ldr` at 0xf2e48 | load | 4 | `abs 0xf2f78` | `-` |
| `ldr` at 0xf2e54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2e5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2e68 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2e70 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2e78 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2e7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2e8c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2e94 | load | 4 | `abs 0xf2f7c` | `-` |
| `ldr` at 0xf2e9c | load | 4 | `abs 0xf2f78` | `-` |
| `ldr` at 0xf2ea8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2eb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2ebc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2ec4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2ecc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2ed0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2ee0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2ee8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2eec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf2ef0 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf2ef8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf2efc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf2f00 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf2f00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf2f00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf2f00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf28e0 | `(unknown callee) 0x23fbc` | `0x138984` | `r1` | `r2` | `0x0` |
| `bl` at 0xf28ec | `(unknown callee) 0x1b1b4` | `0x44` | `?` | `?` | `?` |
| `bl` at 0xf2960 | `(unknown callee) 0xf1164` | `0x13899c` | `0x7c` | `0x145584` | `?` |
| `bl` at 0xf29b4 | `(unknown callee) 0xf1164` | `0x1389a8` | `0x7d` | `0x14559c` | `?` |
| `bl` at 0xf2a08 | `(unknown callee) 0xf1164` | `0x1389b4` | `0x7f` | `0x1455b4` | `?` |
| `bl` at 0xf2a5c | `(unknown callee) 0xf1164` | `0x1389bc` | `0x7e` | `0x143db0` | `?` |
| `bl` at 0xf2ab0 | `(unknown callee) 0xf1164` | `0x1389c4` | `0x7` | `0x143e7c` | `?` |
| `bl` at 0xf2b04 | `(unknown callee) 0xf1164` | `0x1389cc` | `0x5` | `0x143eb8` | `?` |
| `bl` at 0xf2b58 | `(unknown callee) 0xf1164` | `0x1389d8` | `0x6` | `0x14409c` | `?` |
| `bl` at 0xf2bac | `(unknown callee) 0xf1164` | `0x1389e0` | `0x12` | `0x1455cc` | `?` |
| `bl` at 0xf2c00 | `(unknown callee) 0xf1164` | `0x1389f0` | `0x11` | `0x1455e4` | `?` |
| `bl` at 0xf2c54 | `(unknown callee) 0xf1164` | `0x138a00` | `0xa` | `0x14417c` | `?` |
| `bl` at 0xf2ca8 | `(unknown callee) 0xf1164` | `0x138a0c` | `0xb` | `0x14417c` | `?` |
| `bl` at 0xf2cfc | `(unknown callee) 0xf1164` | `0x138a18` | `0x1a` | `0x1456d0` | `?` |
| `bl` at 0xf2d50 | `(unknown callee) 0xf1164` | `0x138a28` | `0x1b` | `0x1456d0` | `?` |
| `bl` at 0xf2da4 | `(unknown callee) 0xf1164` | `0x138a38` | `0x60` | `0x14421c` | `?` |
| `bl` at 0xf2df8 | `(unknown callee) 0xf1164` | `0x138a48` | `0x62` | `0x14421c` | `?` |
| `bl` at 0xf2e4c | `(unknown callee) 0xf1164` | `0x138a58` | `0x61` | `0x145748` | `?` |
| `bl` at 0xf2ea0 | `(unknown callee) 0xf1164` | `0x138a68` | `0x63` | `0x145748` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf2900 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf2920 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf2998 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf29ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2a40 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2a94 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2ae8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2b3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2b90 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2be4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2c38 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2c8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2ce0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2d34 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2d88 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2ddc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2e30 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2e84 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2ed8 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf2f00 | `r0` | `[sp-36]` |

## `setup_tdrdata_nl4r`

- `setup_tdrdata_nl4r`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf2f80 to 0xf36e8.
  Established. The word at `sp -0x44` receives `0x139510` (`str` at 0xf2fa0). The unnamed `0x1b1b4`
  call carries `0xa8` and three unknown candidates (`bl` at 0xf2fb4). The later unnamed `0xf1164`
  call at `0xf33bc` carries `sp-64`, `[sp-36]:2`, `0x144970`, `sp-64` (`bl` at 0xf33bc). NL4R scan
  record construction remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf2f90 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf2f98 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf2f9c | load | 4 | `abs 0xf3670` | `-` |
| `str` at 0xf2fa0 | store | 4 | `sp -0x44` | `0x139510` |
| `ldr` at 0xf2fa4 | load | 4 | `abs 0xf3674` | `-` |
| `ldr` at 0xf2fac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf2fbc | store | 4 | `[sp-24] +0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xf2fc0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2fd0 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf2fdc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf2fe0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2ff0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2ffc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3008 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf300c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf3014 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf301c | load | 4 | `abs 0xf3678` | `-` |
| `ldr` at 0xf3024 | load | 4 | `abs 0xf367c` | `-` |
| `ldr` at 0xf3030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3038 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3044 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf304c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3054 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3058 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3068 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf3070 | load | 4 | `abs 0xf3680` | `-` |
| `ldr` at 0xf3078 | load | 4 | `abs 0xf3684` | `-` |
| `ldr` at 0xf3084 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf308c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3098 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf30a0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf30a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf30ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xf30bc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf30c4 | load | 4 | `abs 0xf3688` | `-` |
| `ldr` at 0xf30cc | load | 4 | `abs 0xf368c` | `-` |
| `ldr` at 0xf30d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf30e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf30ec | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf30f4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf30fc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3100 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3110 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf3118 | load | 4 | `abs 0xf3690` | `-` |
| `ldr` at 0xf3120 | load | 4 | `abs 0xf3694` | `-` |
| `ldr` at 0xf312c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3134 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3140 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3148 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3150 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3154 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3164 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf316c | load | 4 | `abs 0xf3698` | `-` |
| `ldr` at 0xf3174 | load | 4 | `abs 0xf369c` | `-` |
| `ldr` at 0xf3180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3188 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3194 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf319c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf31a4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf31a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf31b8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf31c0 | load | 4 | `abs 0xf36a0` | `-` |
| `ldr` at 0xf31c8 | load | 4 | `abs 0xf36a4` | `-` |
| `ldr` at 0xf31d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf31dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf31e8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf31f0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf31f8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf31fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf320c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf3214 | load | 4 | `abs 0xf36a8` | `-` |
| `ldr` at 0xf321c | load | 4 | `abs 0xf36ac` | `-` |
| `ldr` at 0xf3228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3230 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf323c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3244 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf324c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3250 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3260 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf3268 | load | 4 | `abs 0xf36b0` | `-` |
| `ldr` at 0xf3270 | load | 4 | `abs 0xf36b4` | `-` |
| `ldr` at 0xf327c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3284 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3290 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3298 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf32a0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf32a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf32b4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf32bc | load | 4 | `abs 0xf36b8` | `-` |
| `ldr` at 0xf32c4 | load | 4 | `abs 0xf36bc` | `-` |
| `ldr` at 0xf32d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf32d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf32e4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf32ec | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf32f4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf32f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3308 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf3310 | load | 4 | `abs 0xf36c0` | `-` |
| `ldr` at 0xf3318 | load | 4 | `abs 0xf36c4` | `-` |
| `ldr` at 0xf3324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf332c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3338 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3340 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3348 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf334c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf335c | store | 4 | `sp -0x48` | `?` |
| `str` at 0xf3368 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf3370 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xf3374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3388 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf338c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf3394 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf339c | load | 4 | `abs 0xf36c8` | `-` |
| `ldrh` at 0xf33ac | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf33b8 | load | 4 | `abs 0xf36cc` | `-` |
| `ldr` at 0xf33c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf33cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf33d8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf33e0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf33e8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf33ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xf33fc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf3404 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf340c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf3410 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf3418 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xf3424 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf342c | store | 4 | `sp -0x24` | `0x11` |
| `ldr` at 0xf3430 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3444 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf3448 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf3450 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf3458 | load | 4 | `abs 0xf36d0` | `-` |
| `ldrh` at 0xf3468 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf3474 | load | 4 | `abs 0xf36d4` | `-` |
| `ldr` at 0xf3480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3494 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf349c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf34a4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf34a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf34b8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf34c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf34c8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf34cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf34d4 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xf34e0 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf34e8 | store | 4 | `sp -0x24` | `0x60` |
| `ldr` at 0xf34ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3500 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf3504 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf350c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf3514 | load | 4 | `abs 0xf36d8` | `-` |
| `ldrh` at 0xf3524 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf3530 | load | 4 | `abs 0xf36dc` | `-` |
| `ldr` at 0xf353c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3544 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3550 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3558 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3560 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3564 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3574 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf357c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf3584 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf3588 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf3590 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `str` at 0xf359c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xf35a4 | store | 4 | `sp -0x24` | `0x61` |
| `ldr` at 0xf35a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf35bc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf35c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf35c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf35d0 | load | 4 | `abs 0xf36e0` | `-` |
| `ldrh` at 0xf35e0 | load | 2 | `sp -0x24` | `-` |
| `ldr` at 0xf35ec | load | 4 | `abs 0xf36e4` | `-` |
| `ldr` at 0xf35f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3600 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf360c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3614 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf361c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3620 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3630 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xf3638 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf3640 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf3644 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf364c | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `ldr` at 0xf3654 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3658 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf365c | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf3664 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xf3668 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xf366c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf366c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf366c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf366c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf2fa8 | `(unknown callee) 0x23fbc` | `0x13951c` | `r1` | `r2` | `0x139510` |
| `bl` at 0xf2fb4 | `(unknown callee) 0x1b1b4` | `0xa8` | `?` | `?` | `?` |
| `bl` at 0xf3028 | `(unknown callee) 0xf1164` | `0x139534` | `0x7c` | `0x145584` | `?` |
| `bl` at 0xf307c | `(unknown callee) 0xf1164` | `0x139540` | `0x7d` | `0x14559c` | `?` |
| `bl` at 0xf30d0 | `(unknown callee) 0xf1164` | `0x13954c` | `0x7f` | `0x1455b4` | `?` |
| `bl` at 0xf3124 | `(unknown callee) 0xf1164` | `0x139554` | `0x7e` | `0x144240` | `?` |
| `bl` at 0xf3178 | `(unknown callee) 0xf1164` | `0x13955c` | `0x7` | `0x1442ac` | `?` |
| `bl` at 0xf31cc | `(unknown callee) 0xf1164` | `0x139564` | `0x5` | `0x1442e8` | `?` |
| `bl` at 0xf3220 | `(unknown callee) 0xf1164` | `0x139570` | `0x6` | `0x144774` | `?` |
| `bl` at 0xf3274 | `(unknown callee) 0xf1164` | `0x139578` | `0xd` | `0x1455cc` | `?` |
| `bl` at 0xf32c8 | `(unknown callee) 0xf1164` | `0x139588` | `0xc` | `0x1455e4` | `?` |
| `bl` at 0xf331c | `(unknown callee) 0xf1164` | `0x139598` | `0x4b` | `0x145764` | `?` |
| `bl` at 0xf33a4 | `(unknown callee) 0x1b52c` | `sp-64` | `0x1395a4` | `?` | `?` |
| `bl` at 0xf33bc | `(unknown callee) 0xf1164` | `sp-64` | `[sp-36]:2` | `0x144970` | `sp-64` |
| `bl` at 0xf3460 | `(unknown callee) 0x1b52c` | `sp-64` | `0x1395ac` | `?` | `?` |
| `bl` at 0xf3478 | `(unknown callee) 0xf1164` | `sp-64` | `[sp-36]:2` | `0x1456d0` | `sp-64` |
| `bl` at 0xf351c | `(unknown callee) 0x1b52c` | `sp-64` | `0x1395b8` | `?` | `?` |
| `bl` at 0xf3534 | `(unknown callee) 0xf1164` | `sp-64` | `[sp-36]:2` | `0x144a4c` | `sp-64` |
| `bl` at 0xf35d8 | `(unknown callee) 0x1b52c` | `sp-64` | `0x1395c8` | `?` | `?` |
| `bl` at 0xf35f0 | `(unknown callee) 0xf1164` | `sp-64` | `[sp-36]:2` | `0x145748` | `sp-64` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf2fc8 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf2fe8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf3060 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf30b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3108 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf315c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf31b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3204 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3258 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf32ac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3300 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3354 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf337c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf33f4 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf3438 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf34b0 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf34f4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf356c | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf35b0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf3628 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf366c | `r0` | `[sp-72]` |

## `setup_tdrdata_psic`

- `setup_tdrdata_psic`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7680 to 0xf7c9c.
  Established. The word at `[sp-24] +0x28` receives `ret(0x1b1b4)` (`str` at 0xf76b4); its nonzero
  value takes `0xf76d0`, while zero falls through (`bne` at 0xf76c0). The unnamed `0xf1164` call at
  `0xf7720` carries `0x13a88c`, `0x8`, `0x14532c`, unknown (`bl` at 0xf7720). PSIC record setup
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf7690 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf7698 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf769c | load | 4 | `abs 0xf7c3c` | `-` |
| `ldr` at 0xf76a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf76b4 | store | 4 | `[sp-24] +0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xf76b8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf76c8 | store | 4 | `sp -0x70` | `?` |
| `str` at 0xf76d4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf76d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf76e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf76ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf76f4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7700 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf7704 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf770c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf7714 | load | 4 | `abs 0xf7c40` | `-` |
| `ldr` at 0xf771c | load | 4 | `abs 0xf7c44` | `-` |
| `ldr` at 0xf7728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7730 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf773c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7744 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf774c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7750 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7760 | store | 4 | `sp -0x70` | `?` |
| `str` at 0xf776c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf7770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf7780 | load | 4 | `abs 0xf7c48` | `-` |
| `ldm` at 0xf778c | load | 4 | `abs 0x13a84c` | `-` |
| `ldm` at 0xf778c | load | 4 | `abs 0x13a850` | `-` |
| `ldm` at 0xf778c | load | 4 | `abs 0x13a854` | `-` |
| `ldm` at 0xf778c | load | 4 | `abs 0x13a858` | `-` |
| `stm` at 0xf7790 | store | 4 | `sp -0x58` | `[0x13a84c]` |
| `stm` at 0xf7790 | store | 4 | `sp -0x54` | `[0x13a850]` |
| `stm` at 0xf7790 | store | 4 | `sp -0x50` | `[0x13a854]` |
| `stm` at 0xf7790 | store | 4 | `sp -0x4c` | `[0x13a858]` |
| `ldr` at 0xf7794 | load | 4 | `abs 0x13a85c` | `-` |
| `str` at 0xf7798 | store | 4 | `sp -0x48` | `[0x13a85c]` |
| `ldr` at 0xf779c | load | 4 | `abs 0xf7c4c` | `-` |
| `ldm` at 0xf77a8 | load | 4 | `abs 0x13a860` | `-` |
| `ldm` at 0xf77a8 | load | 4 | `abs 0x13a864` | `-` |
| `ldm` at 0xf77a8 | load | 4 | `abs 0x13a868` | `-` |
| `ldm` at 0xf77a8 | load | 4 | `abs 0x13a86c` | `-` |
| `stm` at 0xf77ac | store | 4 | `sp -0x6c` | `[0x13a860]` |
| `stm` at 0xf77ac | store | 4 | `sp -0x68` | `[0x13a864]` |
| `stm` at 0xf77ac | store | 4 | `sp -0x64` | `[0x13a868]` |
| `stm` at 0xf77ac | store | 4 | `sp -0x60` | `[0x13a86c]` |
| `ldr` at 0xf77b0 | load | 4 | `abs 0x13a870` | `-` |
| `str` at 0xf77b4 | store | 4 | `sp -0x5c` | `[0x13a870]` |
| `str` at 0xf77bc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf77c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf77d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf77f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf77f4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xf7800 | load | 2 | `sp -0x20` | `-` |
| `ldrh` at 0xf7808 | load | 2 | `sp -0x1c` | `-` |
| `ldr` at 0xf781c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf7834 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf7840 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7848 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7854 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf785c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7864 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7868 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7878 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7880 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf7888 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf7890 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf7898 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xf78a0 | load | 4 | `abs 0xf7c50` | `-` |
| `ldr` at 0xf78a8 | load | 4 | `abs 0xf7c54` | `-` |
| `ldr` at 0xf78b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf78bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf78c8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf78d0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf78d8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf78dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf78ec | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf78f4 | load | 4 | `abs 0xf7c58` | `-` |
| `ldr` at 0xf78fc | load | 4 | `abs 0xf7c5c` | `-` |
| `ldr` at 0xf7908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7910 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf791c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7924 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf792c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7930 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7940 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7948 | load | 4 | `abs 0xf7c60` | `-` |
| `ldr` at 0xf7950 | load | 4 | `abs 0xf7c5c` | `-` |
| `ldr` at 0xf795c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7964 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7970 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7978 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7980 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7984 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7994 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf799c | load | 4 | `abs 0xf7c64` | `-` |
| `ldr` at 0xf79a4 | load | 4 | `abs 0xf7c68` | `-` |
| `ldr` at 0xf79b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf79b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf79c4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf79cc | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf79d4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf79d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf79e8 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf79f0 | load | 4 | `abs 0xf7c6c` | `-` |
| `ldr` at 0xf79f8 | load | 4 | `abs 0xf7c68` | `-` |
| `ldr` at 0xf7a04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7a0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7a18 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7a20 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7a28 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7a2c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7a3c | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7a44 | load | 4 | `abs 0xf7c70` | `-` |
| `ldr` at 0xf7a4c | load | 4 | `abs 0xf7c74` | `-` |
| `ldr` at 0xf7a58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7a60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7a6c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7a74 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7a7c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7a80 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7a90 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7a98 | load | 4 | `abs 0xf7c78` | `-` |
| `ldr` at 0xf7aa0 | load | 4 | `abs 0xf7c74` | `-` |
| `ldr` at 0xf7aac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7ab4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7ac0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7ac8 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7ad0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7ad4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7ae4 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7aec | load | 4 | `abs 0xf7c7c` | `-` |
| `ldr` at 0xf7af4 | load | 4 | `abs 0xf7c80` | `-` |
| `ldr` at 0xf7b00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7b08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7b14 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7b1c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7b24 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7b28 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7b38 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7b40 | load | 4 | `abs 0xf7c84` | `-` |
| `ldr` at 0xf7b48 | load | 4 | `abs 0xf7c80` | `-` |
| `ldr` at 0xf7b54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7b5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7b68 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7b70 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7b78 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7b7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7b8c | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7b94 | load | 4 | `abs 0xf7c88` | `-` |
| `ldr` at 0xf7b9c | load | 4 | `abs 0xf7c8c` | `-` |
| `ldr` at 0xf7ba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7bb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7bbc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf7bc4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf7bcc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf7bd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7be0 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xf7be8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf7bf4 | load | 4 | `abs 0xf7c90` | `-` |
| `str` at 0xf7bf8 | store | 4 | `sp -0x7c` | `0x13a910` |
| `str` at 0xf7c00 | store | 4 | `sp -0x78` | `0x5b` |
| `ldr` at 0xf7c04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf7c08 | store | 4 | `sp -0x74` | `[sp-36]+1` |
| `ldr` at 0xf7c10 | load | 4 | `abs 0xf7c94` | `-` |
| `ldr` at 0xf7c14 | load | 4 | `abs 0xf7c98` | `-` |
| `ldr` at 0xf7c20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf7c24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf7c28 | store | 4 | `[sp-24] +0x24` | `?` |
| `str` at 0xf7c30 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xf7c34 | load | 4 | `sp -0x70` | `-` |
| `ldmdb` at 0xf7c38 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf7c38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf7c38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf7c38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf76a0 | `(unknown callee) 0x23fbc` | `0x13a874` | `r1` | `r2` | `0x0` |
| `bl` at 0xf76ac | `(unknown callee) 0x1b1b4` | `0x16c` | `?` | `?` | `?` |
| `bl` at 0xf7720 | `(unknown callee) 0xf1164` | `0x13a88c` | `0x8` | `0x14532c` | `?` |
| `bl` at 0xf77f8 | `(unknown callee) 0x1b52c` | `sp-68` | `?` | `[sp-32]` | `?` |
| `bl` at 0xf7838 | `(unknown callee) 0xf1164` | `sp-68` | `?` | `?` | `?` |
| `bl` at 0xf78ac | `(unknown callee) 0xf1164` | `0x13a898` | `0x10` | `0x145344` | `?` |
| `bl` at 0xf7900 | `(unknown callee) 0xf1164` | `0x13a8a4` | `0x11` | `0x14535c` | `?` |
| `bl` at 0xf7954 | `(unknown callee) 0xf1164` | `0x13a8b0` | `0x12` | `0x14535c` | `?` |
| `bl` at 0xf79a8 | `(unknown callee) 0xf1164` | `0x13a8bc` | `0x13` | `0x145374` | `?` |
| `bl` at 0xf79fc | `(unknown callee) 0xf1164` | `0x13a8c8` | `0x14` | `0x145374` | `?` |
| `bl` at 0xf7a50 | `(unknown callee) 0xf1164` | `0x13a8d4` | `0x15` | `0x1453e0` | `?` |
| `bl` at 0xf7aa4 | `(unknown callee) 0xf1164` | `0x13a8e0` | `0x16` | `0x1453e0` | `?` |
| `bl` at 0xf7af8 | `(unknown callee) 0xf1164` | `0x13a8ec` | `0x17` | `0x1453f8` | `?` |
| `bl` at 0xf7b4c | `(unknown callee) 0xf1164` | `0x13a8f8` | `0x18` | `0x1453f8` | `?` |
| `bl` at 0xf7ba0 | `(unknown callee) 0xf1164` | `0x13a904` | `0x19` | `0x145524` | `?` |
| `bl` at 0xf7c1c | `(unknown callee) 0x21eb8` | `0x9` | `0x13a954` | `0x153` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf76c0 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf76e0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf7758 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xf7778 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xf77c8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf7870 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf78e4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7938 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf798c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf79e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7a34 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7a88 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7adc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7b30 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7b84 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf7bd8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xf7bf0 | `[sp-36]+1 == 0x5b` | `[sp-36]+1 != 0x5b` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf7c38 | `r0` | `[sp-112]` |

## `FRUPackData`

- `FRUPackData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc25e4 to 0xc330c.
  Established. The word through `[sp+32] +0x0` receives `0x1` (`str` at 0xc260c). Included byte
  assignments through `[[sp+28]] +0x0` and `[[sp+28]] +0x1` write `0x6` and `0x8` respectively
  (`strb` at 0xc2930, `strb` at 0xc294c). The unnamed `0x1b394` call at `0xc317c` carries
  `[sp-40]+4`, `[sp+16]`, `[sp-52]`, `[sp-40]+4` (`bl` at 0xc317c). FRU packing format meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc25f4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc25f8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc25fc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xc2600 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xc2604 | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc260c | store | 4 | `[sp+32] +0x0` | `0x1` |
| `ldr` at 0xc2610 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2614 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2618 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2620 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xc2624 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2628 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc262c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2634 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xc2638 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc263c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2640 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2648 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xc264c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2650 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2654 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc265c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc2660 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2664 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2668 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2670 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc2674 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2678 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc267c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2684 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc2688 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc268c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2690 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2698 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc269c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26a0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26a4 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc26ac | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc26b0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26b4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26b8 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc26c0 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc26c4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26c8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26cc | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc26d4 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `str` at 0xc26dc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc26e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc26e4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc26f4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26f8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc26fc | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2704 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc2708 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc270c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2710 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2718 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc271c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2720 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2724 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc272c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc2730 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2734 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2738 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc274c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2754 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc275c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2764 | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xc2768 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc276c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2770 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2784 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc278c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc2794 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc279c | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xc27a0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc27a8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xc27b0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc27b4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc27b8 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc27c0 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc27c4 | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc27c8 | store | 4 | `sp -0x38` | `[sp+32]` |
| `ldr` at 0xc27cc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc27d0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc27d4 | store | 4 | `sp -0x3c` | `[[sp+32]]` |
| `ldr` at 0xc27d8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc27e4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc27f0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc27f8 | store | 4 | `sp -0x40` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xc27fc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc2804 | store | 4 | `sp -0x40` | `([sp-60] + ret(0x1b2bc))+1` |
| `ldr` at 0xc280c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc2810 | store | 4 | `sp -0x40` | `[[sp+32]]` |
| `ldr` at 0xc2814 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc2818 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc281c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc2820 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2824 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2828 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2830 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc2834 | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc2838 | store | 4 | `sp -0x44` | `[sp+32]` |
| `ldr` at 0xc283c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2840 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2844 | store | 4 | `sp -0x48` | `[[sp+32]]` |
| `ldr` at 0xc2848 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc2854 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc2860 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc2868 | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x1b2bc))` |
| `ldr` at 0xc286c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xc2874 | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x1b2bc))+1` |
| `ldr` at 0xc287c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc2880 | store | 4 | `sp -0x4c` | `[[sp+32]]` |
| `ldr` at 0xc2884 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc2888 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc288c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc2890 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2894 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2898 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc28a0 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xc28a4 | load | 4 | `sp +0x20` | `-` |
| `str` at 0xc28a8 | store | 4 | `sp -0x50` | `[sp+32]` |
| `ldr` at 0xc28ac | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc28b0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc28b4 | store | 4 | `sp -0x54` | `[[sp+32]]` |
| `ldr` at 0xc28b8 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc28c4 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc28d0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xc28d8 | store | 4 | `sp -0x58` | `([sp-84] + ret(0x1b2bc))` |
| `ldr` at 0xc28dc | load | 4 | `sp -0x58` | `-` |
| `str` at 0xc28e4 | store | 4 | `sp -0x58` | `([sp-84] + ret(0x1b2bc))+1` |
| `ldr` at 0xc28ec | load | 4 | `sp -0x54` | `-` |
| `str` at 0xc28f0 | store | 4 | `sp -0x58` | `[[sp+32]]` |
| `ldr` at 0xc28f4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xc28f8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xc28fc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc2900 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc2904 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc2908 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xc2914 | store | 4 | `[sp+28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xc2918 | load | 4 | `[sp+28] +0x0` | `-` |
| `str` at 0xc291c | store | 4 | `sp -0x28` | `[[sp+28]]` |
| `ldr` at 0xc2924 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc2930 | store | 1 | `[[sp+28]] +0x0` | `0x6` |
| `str` at 0xc2938 | store | 4 | `sp -0x28` | `[[sp+28]]+1` |
| `ldr` at 0xc2940 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc294c | store | 1 | `[[sp+28]] +0x1` | `0x8` |
| `str` at 0xc2954 | store | 4 | `sp -0x28` | `[[sp+28]]+2` |
| `ldr` at 0xc295c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc2968 | store | 1 | `[[sp+28]] +0x2` | `0x0` |
| `str` at 0xc2970 | store | 4 | `sp -0x28` | `[[sp+28]]+3` |
| `ldr` at 0xc2978 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xc2984 | store | 1 | `[[sp+28]] +0x3` | `0x0` |
| `str` at 0xc298c | store | 4 | `sp -0x28` | `[[sp+28]]+4` |
| `ldr` at 0xc2990 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2994 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc299c | store | 1 | `[[sp+28]] +0x4` | `?` |
| `ldr` at 0xc29a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc29ac | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc29b4 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc29bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc29c4 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc29cc | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc29d4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc29dc | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xc29e0 | store | 1 | `[sp-40] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xc29e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc29ec | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc29f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc29f4 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc29fc | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2a04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a0c | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc2a14 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2a1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a24 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc2a2c | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2a34 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2a3c | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xc2a40 | store | 1 | `[sp-40] +0x3` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xc2a44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2a4c | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2a50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a54 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2a5c | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2a64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a6c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2a74 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2a7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2a84 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc2a8c | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2a94 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2a9c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xc2aa0 | store | 1 | `[sp-40] +0x3` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xc2aa4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2aac | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2ab0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2ab4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc2abc | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2ac4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2acc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc2ad4 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2adc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2ae4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc2aec | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2af4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2afc | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc2b00 | store | 1 | `[sp-40] +0x3` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc2b04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2b0c | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2b10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2b14 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2b1c | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2b24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2b2c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2b34 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2b3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2b44 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc2b4c | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2b54 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2b5c | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xc2b60 | store | 1 | `[sp-40] +0x3` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xc2b64 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2b6c | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2b70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2b74 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc2b7c | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2b84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2b8c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc2b94 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2b9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2ba4 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc2bac | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2bb4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2bbc | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xc2bc0 | store | 1 | `[sp-40] +0x3` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xc2bc4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2bcc | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2bd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2bd4 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xc2bdc | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2be4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2bec | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xc2bf4 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2bfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2c04 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xc2c0c | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2c14 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc2c1c | load | 1 | `sp +0x8` | `-` |
| `strb` at 0xc2c20 | store | 1 | `[sp-40] +0x3` | `([sp+8]:1 & 0xff)` |
| `ldr` at 0xc2c24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2c2c | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xc2c34 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc2c38 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2c3c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc2c4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2c50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2c64 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2c6c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2c74 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc2c7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2c84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2c98 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2ca0 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2ca8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2cb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2cb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2ccc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2cd4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2cdc | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2ce4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2cec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2d00 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xc2d08 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc2d0c | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc2d10 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2d18 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2d1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2d20 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2d34 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2d3c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2d44 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2d4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2d54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2d68 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2d70 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2d78 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2d80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2d88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2d9c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2da4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2dac | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2db4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2dbc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2dd0 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xc2dd8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc2ddc | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc2de0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2de8 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc2dec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2df0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2e04 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2e0c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2e14 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xc2e1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2e24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2e38 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2e40 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2e48 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2e50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2e58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2e6c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2e74 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2e7c | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2e84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2e8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2ea0 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xc2ea8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc2eac | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc2eb0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc2eb8 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xc2ec0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc2ec4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2ed8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2ee0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2ee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc2ef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2ef8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2f0c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2f14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2f28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc2f30 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2f38 | store | 1 | `[sp-40] +0x0` | `?` |
| `ldr` at 0xc2f40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2f48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2f5c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2f64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2f78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc2f80 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2f88 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc2f90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2f98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2fac | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc2fb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc2fc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc2fd0 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xc2fd8 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc2fe0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc2fe8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc2ffc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc3004 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3018 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc3020 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc3024 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xc3028 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc3030 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc3034 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc303c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc3044 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc3058 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc3060 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc3074 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc307c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3084 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3088 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3094 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc30a8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc30b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc30b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc30c0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc30c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc30cc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xc30d4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc30e0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc30ec | store | 4 | `sp -0x5c` | `ret(0x1b2bc)` |
| `ldr` at 0xc30f0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xc30f8 | store | 4 | `sp -0x5c` | `ret(0x1b2bc)+1` |
| `str` at 0xc3104 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xc3108 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xc310c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc3110 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3114 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc311c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc3124 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc312c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc3134 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc313c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3144 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc314c | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc3154 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc315c | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc3160 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xc3164 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc316c | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc3170 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3174 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3178 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc3180 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3184 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc318c | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xc3190 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc319c | load | 4 | `sp +0x14` | `-` |
| `str` at 0xc31a8 | store | 4 | `sp -0x60` | `ret(0x1b2bc)` |
| `ldr` at 0xc31ac | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc31b4 | store | 4 | `sp -0x60` | `ret(0x1b2bc)+1` |
| `str` at 0xc31c0 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xc31c4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc31c8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc31cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc31d0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc31d8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc31e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc31e8 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc31f0 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc31f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3200 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc3208 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc3210 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc3218 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc321c | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xc3220 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc3228 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc322c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3230 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc3234 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc323c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3240 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc3248 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xc324c | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc3258 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xc3264 | store | 4 | `sp -0x64` | `ret(0x1b2bc)` |
| `ldr` at 0xc3268 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xc3270 | store | 4 | `sp -0x64` | `ret(0x1b2bc)+1` |
| `str` at 0xc327c | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xc3280 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xc3284 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc3288 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc328c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc3294 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc329c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc32a4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc32ac | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xc32b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc32bc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xc32c4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xc32cc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xc32d4 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc32d8 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xc32dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc32e4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xc32e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc32ec | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc32f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc32f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc32fc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc3304 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldmdb` at 0xc3308 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc3308 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc3308 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc3308 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc27e8 | `(unknown callee) 0x1b2bc` | `[sp+16]` | `?` | `[sp+32]` | `[sp+16]` |
| `bl` at 0xc2858 | `(unknown callee) 0x1b2bc` | `[sp+20]` | `?` | `[sp+32]` | `[sp+20]` |
| `bl` at 0xc28c8 | `(unknown callee) 0x1b2bc` | `[sp+24]` | `?` | `[sp+32]` | `[sp+24]` |
| `bl` at 0xc290c | `(unknown callee) 0x1b1b4` | `[[sp+32]]` | `?` | `?` | `[sp+32]` |
| `bl` at 0xc3090 | `(unknown callee) 0x1b394` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xc30e4 | `(unknown callee) 0x1b2bc` | `[sp+16]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xc317c | `(unknown callee) 0x1b394` | `[sp-40]+4` | `[sp+16]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xc31a0 | `(unknown callee) 0x1b2bc` | `[sp+20]` | `?` | `[sp-40]` | `[sp+20]` |
| `bl` at 0xc3238 | `(unknown callee) 0x1b394` | `[sp-40]+4` | `[sp+20]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xc325c | `(unknown callee) 0x1b2bc` | `[sp+24]` | `?` | `[sp-40]` | `[sp+24]` |
| `bl` at 0xc32f4 | `(unknown callee) 0x1b394` | `[sp-40]+4` | `[sp+24]` | `[sp-52]` | `[sp-40]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc26ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc27e0 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xc2850 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xc28c0 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `blt` at 0xc2c44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xc2eec | `[sp-44] < ? (signed, 32-bit)` | `[sp-44] >= ? (signed, 32-bit)` |
| `beq` at 0xc30dc | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xc3198 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xc3254 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc3308 | `r0` | `ret(0x1b394)` |

## `FRUPackEvents`

- `FRUPackEvents`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc330c to 0xc37dc.
  Established. The word through `[sp+16] +0x0` receives `0x1` (`str` at 0xc3378). The byte through
  `[[sp+12]] +0x0` receives `0x6` (`strb` at 0xc3494). The unnamed `0x1b394` call has unknown,
  unknown, `0xa8`, unknown candidates (`bl` at 0xc37b8). Packed event field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc331c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc3320 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc3324 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xc3328 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xc3330 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc3334 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xc3338 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xc333c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc334c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc3354 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc3358 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc335c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3360 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xc336c | store | 4 | `sp -0x34` | `0xa8` |
| `ldr` at 0xc3370 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc3378 | store | 4 | `[sp+16] +0x0` | `0x1` |
| `ldr` at 0xc337c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3380 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3384 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc338c | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xc3390 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3394 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3398 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc33a0 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xc33a4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33a8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33ac | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc33b4 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xc33b8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33bc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33c0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc33c8 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc33cc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33d0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33d4 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc33dc | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc33e0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33e4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33e8 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc33f0 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc33f4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33f8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc33fc | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc3404 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc3408 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc340c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3410 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc3418 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc341c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3420 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3424 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc342c | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc3430 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3434 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3438 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc3440 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xc3444 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3448 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc344c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc3450 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc3458 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc3460 | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xc3464 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc3468 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc346c | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc3478 | store | 4 | `[sp+12] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xc347c | load | 4 | `[sp+12] +0x0` | `-` |
| `str` at 0xc3480 | store | 4 | `sp -0x2c` | `[[sp+12]]` |
| `ldr` at 0xc3488 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xc3494 | store | 1 | `[[sp+12]] +0x0` | `0x6` |
| `str` at 0xc349c | store | 4 | `sp -0x2c` | `[[sp+12]]+1` |
| `ldr` at 0xc34a4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc34a8 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xc34b0 | store | 1 | `[[sp+12]] +0x1` | `([sp-52]:1 & 0xff)` |
| `str` at 0xc34b8 | store | 4 | `sp -0x2c` | `[[sp+12]]+2` |
| `ldr` at 0xc34c0 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xc34cc | store | 1 | `[[sp+12]] +0x2` | `0x0` |
| `str` at 0xc34d4 | store | 4 | `sp -0x2c` | `[[sp+12]]+3` |
| `ldr` at 0xc34dc | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xc34e8 | store | 1 | `[[sp+12]] +0x3` | `0x0` |
| `str` at 0xc34f0 | store | 4 | `sp -0x2c` | `[[sp+12]]+4` |
| `ldr` at 0xc34f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc34f8 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc3500 | store | 1 | `[[sp+12]] +0x4` | `?` |
| `ldr` at 0xc3508 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3510 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc3518 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc3520 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3528 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc3530 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc3538 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc3540 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xc3544 | store | 1 | `[sp-44] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xc3548 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3550 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc3554 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3558 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc3560 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc3568 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3570 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc3578 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc3580 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3588 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xc3590 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc3598 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc35a0 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xc35a4 | store | 1 | `[sp-44] +0x3` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xc35a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc35b0 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc35b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc35b8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc35c0 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc35c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc35d0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc35d8 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc35e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc35e8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc35f0 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc35f8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc3600 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xc3604 | store | 1 | `[sp-44] +0x3` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xc3608 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3610 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc3614 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3618 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc3620 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc3628 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3630 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc3638 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc3640 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3648 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc3650 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc3658 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc3660 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc3664 | store | 1 | `[sp-44] +0x3` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc3668 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3670 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc3674 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3678 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc3680 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc3688 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3690 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc3698 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc36a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc36a8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc36b0 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc36b8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc36c0 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xc36c4 | store | 1 | `[sp-44] +0x3` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xc36c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc36d0 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc36d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc36d8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc36e0 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc36e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc36f0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc36f8 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc3700 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3708 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc3710 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc3718 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc3720 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xc3724 | store | 1 | `[sp-44] +0x3` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xc3728 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3730 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc3734 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3738 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xc3740 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xc3748 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3750 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xc3758 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xc3760 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3768 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xc3770 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xc3778 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xc3780 | load | 1 | `sp -0x30` | `-` |
| `strb` at 0xc3784 | store | 1 | `[sp-44] +0x3` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0xc3788 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3790 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xc3794 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xc3798 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xc379c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc37ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc37b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc37bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc37c4 | store | 4 | `sp -0x2c` | `[sp-44]+168` |
| `ldr` at 0xc37c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc37cc | load | 4 | `[sp-40] +0xa8` | `-` |
| `str` at 0xc37d0 | store | 4 | `sp -0x28` | `[[sp-40]+168]` |
| `ldmdb` at 0xc37d8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc37d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc37d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc37d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc3470 | `(unknown callee) 0x1b1b4` | `[[sp+16]]` | `[sp+16]` | `?` | `[sp+16]` |
| `bl` at 0xc37b8 | `(unknown callee) 0x1b394` | `?` | `?` | `0xa8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc3344 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc37a4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc37d8 | `r0` | `?` |

## `FRUUnpackData`

- `FRUUnpackData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc37dc to 0xc428c.
  Established. The word through `[sp+28] +0x0` receives `0x0` (`str` at 0xc3828). Equality
  `[[sp-28]] == 0x6` takes `0xc386c`, while inequality falls through (`beq` at 0xc3858). The unnamed
  `0x1b394` call at `0xc412c` carries `[[sp+24]]`, `[sp-24]`, `[sp-64]`, `[sp+24]` (`bl` at 0xc412c). FRU unpacked record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc37ec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc37f0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc37f4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xc37f8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xc37fc | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc3804 | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xc3808 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xc3810 | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xc3814 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xc381c | store | 4 | `[sp+24] +0x0` | `0x0` |
| `ldr` at 0xc3820 | load | 4 | `sp +0x1c` | `-` |
| `str` at 0xc3828 | store | 4 | `[sp+28] +0x0` | `0x0` |
| `ldr` at 0xc382c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3834 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc383c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xc3840 | store | 4 | `[sp-28] +0x0` | `[[sp-24]]:1` |
| `str` at 0xc3848 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xc384c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc3850 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc385c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3864 | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xc3870 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3878 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xc387c | store | 4 | `sp -0x30` | `[[sp-24]+1]:1` |
| `str` at 0xc3884 | store | 4 | `sp -0x18` | `[sp-24]+2` |
| `ldr` at 0xc388c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3894 | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0xc3898 | store | 4 | `sp -0x34` | `[[sp-24]+2]:1` |
| `str` at 0xc38a0 | store | 4 | `sp -0x18` | `[sp-24]+3` |
| `ldr` at 0xc38a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc38b0 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc38b4 | store | 4 | `sp -0x38` | `[[sp-24]+3]:1` |
| `str` at 0xc38bc | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc38c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc38c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc38c8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc38d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc38dc | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc38ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc38f4 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3904 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc390c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3914 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xc3918 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3920 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3924 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc3928 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc392c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3938 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3940 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3950 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3958 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3968 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3970 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3978 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xc397c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3984 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3988 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xc398c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3990 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc399c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc39a4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc39b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc39bc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc39cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc39d4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc39dc | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xc39e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc39e8 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc39ec | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xc39f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc39f4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3a00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3a08 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3a18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3a20 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3a30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3a38 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3a40 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xc3a44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3a4c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3a50 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc3a54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3a58 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3a64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3a6c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3a7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3a84 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3a94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3a9c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3aa4 | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xc3aa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3ab0 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3ab4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc3ab8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3abc | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3ac8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3ad0 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3ae0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3ae8 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3af8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3b00 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3b08 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xc3b0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3b14 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3b18 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3b1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3b20 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3b2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3b34 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3b44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3b4c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3b5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3b64 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3b6c | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xc3b70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3b78 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3b7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc3b88 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3b90 | store | 4 | `[sp+12] +0x0` | `0x6` |
| `ldr` at 0xc3b98 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc3b9c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3ba0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc3bc0 | store | 4 | `[sp+20] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xc3bc4 | load | 4 | `[sp+20] +0x0` | `-` |
| `str` at 0xc3bc8 | store | 4 | `sp -0x28` | `[[sp+20]]` |
| `ldr` at 0xc3bcc | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc3bd0 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xc3bdc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3be4 | store | 4 | `[sp+12] +0x0` | `0x8` |
| `str` at 0xc3bf0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xc3bf4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc3bf8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3bfc | load | 4 | `[sp+16] +0x0` | `-` |
| `ldr` at 0xc3c0c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3c20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3c28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3c2c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc3c38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3c40 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xc3c50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3c58 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xc3c68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3c70 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc3c78 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc3c7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3c84 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3c88 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3c9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3ca4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3ca8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3cb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3cbc | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3ccc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3cd4 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3ce4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3cec | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3cf4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc3cf8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3d00 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3d04 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3d18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3d20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3d24 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc3d30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3d38 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xc3d48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3d50 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xc3d60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3d68 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xc3d70 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc3d74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3d7c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3d80 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3d94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3d9c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3db0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3db8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3dcc | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xc3dd0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3de4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3dec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3df8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3e00 | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xc3e08 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3e1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3e24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3e38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3e40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3e48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3e5c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc3e60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3e74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3e7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3e80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3e90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3e94 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc3ea0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3ea8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xc3eb8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3ec0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xc3ed0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc3ed8 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc3ee0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc3ee4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc3eec | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc3ef0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3f04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3f0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc3f14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3f1c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc3f20 | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xc3f24 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc3f2c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xc3f34 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3f48 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3f50 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3f64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3f6c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc3f80 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xc3f84 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3f98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3fa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3fac | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc3fb4 | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xc3fbc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3fd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3fd8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc3fec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc3ff4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3ffc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc400c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc4020 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4028 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc403c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc404c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc405c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc4070 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4078 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4080 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc4088 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc408c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc4094 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xc409c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc40a0 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc40ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc40b4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xc40c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc40cc | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xc40dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc40e4 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xc40ec | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc40f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc40f8 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xc40fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4108 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc410c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc4118 | store | 4 | `[sp+24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xc411c | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc4120 | load | 4 | `[sp+24] +0x0` | `-` |
| `ldr` at 0xc4124 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4128 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4134 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc413c | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xc4140 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc4144 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4150 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc4158 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4168 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc4170 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4180 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc4188 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4190 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc4194 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc419c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc41a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc41ac | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc41b0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc41bc | store | 4 | `[sp+28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xc41c0 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc41c4 | load | 4 | `[sp+28] +0x0` | `-` |
| `ldr` at 0xc41c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc41cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc41d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc41d8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc41e0 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xc41e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc41e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc41f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc41fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc420c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc4214 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc4224 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc422c | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4234 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc4238 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc4240 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc4244 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4250 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc4254 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc4260 | store | 4 | `[sp+32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xc4264 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc4268 | load | 4 | `[sp+32] +0x0` | `-` |
| `ldr` at 0xc426c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4270 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc427c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc4284 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldmdb` at 0xc4288 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc4288 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4288 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4288 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc3bb8 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[[sp+16]]` | `?` |
| `bl` at 0xc3dc4 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xc3e54 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xc3f78 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xc4008 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xc4058 | `(unknown callee) 0x1b394` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0xc4110 | `(unknown callee) 0x1b1b4` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xc412c | `(unknown callee) 0x1b394` | `[[sp+24]]` | `[sp-24]` | `[sp-64]` | `[sp+24]` |
| `bl` at 0xc41b4 | `(unknown callee) 0x1b1b4` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xc41d0 | `(unknown callee) 0x1b394` | `[[sp+28]]` | `[sp-24]` | `[sp-64]` | `[sp+28]` |
| `bl` at 0xc4258 | `(unknown callee) 0x1b1b4` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xc4274 | `(unknown callee) 0x1b394` | `[[sp+32]]` | `[sp-24]` | `[sp-64]` | `[sp+32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc3858 | `[[sp-28]] == 0x6` | `[[sp-28]] != 0x6` |
| `beq` at 0xc3b84 | `[sp-48] == 0x8` | `[sp-48] != 0x8` |
| `bne` at 0xc3bd8 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |
| `blt` at 0xc3c04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc3df4 | `? != 0x0` | `? == 0x0` |
| `blt` at 0xc3e88 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc3fa8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xc4104 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xc41a8 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xc424c | `[sp-64] == 0x0` | `[sp-64] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4288 | `r0` | `?` |

## `FRUUnpackEvents`

- `FRUUnpackEvents`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc428c to 0xc47d4.
  Established. The words through `[sp+16] +0x0` and `[sp+20] +0x0` each receive `0x0` (`str` at 0xc42b4, `str` at 0xc42c0). Equality `[[sp-24]] == 0x6` takes `0xc4304`, while inequality falls
  through (`beq` at 0xc42f0). The unnamed `0x1b394` call carries `ret(0x1b1b4)`, `[sp-20]`,
  `[sp-56]`, `ret(0x1b1b4)` (`bl` at 0xc466c). Event unpacking policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc429c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc42a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc42a4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc42a8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc42ac | load | 4 | `sp +0x10` | `-` |
| `str` at 0xc42b4 | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xc42b8 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xc42c0 | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xc42c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc42cc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc42d4 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xc42d8 | store | 4 | `[sp-24] +0x0` | `[[sp-20]]:1` |
| `str` at 0xc42e0 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xc42e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc42e8 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xc42f4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc42fc | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xc4308 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4310 | load | 1 | `[sp-20] +0x1` | `-` |
| `str` at 0xc4314 | store | 4 | `sp -0x38` | `[[sp-20]+1]:1` |
| `str` at 0xc431c | store | 4 | `sp -0x14` | `[sp-20]+2` |
| `ldr` at 0xc4324 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc432c | load | 1 | `[sp-20] +0x2` | `-` |
| `str` at 0xc4330 | store | 4 | `sp -0x30` | `[[sp-20]+2]:1` |
| `str` at 0xc4338 | store | 4 | `sp -0x14` | `[sp-20]+3` |
| `ldr` at 0xc4340 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4348 | load | 1 | `[sp-20] +0x3` | `-` |
| `str` at 0xc434c | store | 4 | `sp -0x34` | `[[sp-20]+3]:1` |
| `str` at 0xc4354 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc4358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc435c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4360 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc436c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4374 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc4384 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc438c | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc439c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc43a4 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc43ac | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xc43b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc43b8 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc43bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc43c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc43c4 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc43d0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc43d8 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc43e8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc43f0 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc4400 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4408 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc4410 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xc4414 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc441c | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc4420 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xc4424 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4428 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc4434 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc443c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc444c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4454 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc4464 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc446c | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc4474 | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xc4478 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc4480 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc4484 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xc4488 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc448c | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc4498 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc44a0 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc44b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc44b8 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc44c8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc44d0 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc44d8 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xc44dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc44e4 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc44e8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc44ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc44f0 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc44fc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4504 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc4514 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc451c | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc452c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4534 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc453c | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xc4540 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc4548 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc454c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc4550 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4554 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc4560 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4568 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc4578 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4580 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc4590 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4598 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc45a0 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xc45a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc45ac | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc45b0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc45b4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc45b8 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xc45c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc45cc | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xc45dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc45e4 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xc45f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc45fc | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xc4604 | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xc4608 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc4610 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xc4614 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc4620 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc4628 | store | 4 | `[sp+12] +0x0` | `0x7` |
| `str` at 0xc4634 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc4638 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc463c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc4640 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xc465c | store | 4 | `sp -0x24` | `ret(0x1b1b4)` |
| `ldr` at 0xc4660 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4664 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4668 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc4670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4674 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc467c | store | 4 | `sp -0x14` | `([sp-20] + [sp-56])` |
| `ldr` at 0xc4680 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc4688 | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xc468c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4690 | load | 4 | `[sp-36] +0x10` | `-` |
| `str` at 0xc4694 | store | 4 | `sp -0x3c` | `[[sp-36]+16]` |
| `ldr` at 0xc4698 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc46a4 | store | 4 | `sp -0x3c` | `ret(0x1aff8)` |
| `ldr` at 0xc46a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc46ac | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc46b0 | store | 4 | `[sp-36] +0x10` | `ret(0x1aff8)` |
| `ldr` at 0xc46b4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc46b8 | load | 2 | `[sp-36] +0x18` | `-` |
| `str` at 0xc46bc | store | 4 | `sp -0x3c` | `[[sp-36]+24]:2` |
| `ldrh` at 0xc46c0 | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc46d8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc46dc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc46e0 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc46e4 | store | 2 | `[sp-36] +0x18` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc46e8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc46ec | load | 2 | `[sp-36] +0x1a` | `-` |
| `str` at 0xc46f0 | store | 4 | `sp -0x3c` | `[[sp-36]+26]:2` |
| `ldrh` at 0xc46f4 | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc470c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc4710 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc4714 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc4718 | store | 2 | `[sp-36] +0x1a` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc471c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc4720 | load | 2 | `[sp-36] +0x1c` | `-` |
| `str` at 0xc4724 | store | 4 | `sp -0x3c` | `[[sp-36]+28]:2` |
| `ldrh` at 0xc4728 | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc4740 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc4744 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc4748 | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc474c | store | 2 | `[sp-36] +0x1c` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc4750 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc4754 | load | 2 | `[sp-36] +0x1e` | `-` |
| `str` at 0xc4758 | store | 4 | `sp -0x3c` | `[[sp-36]+30]:2` |
| `ldrh` at 0xc475c | load | 2 | `sp -0x3c` | `-` |
| `str` at 0xc4774 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc4778 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc477c | load | 2 | `sp -0x3c` | `-` |
| `strh` at 0xc4780 | store | 2 | `[sp-36] +0x1e` | `([sp-60]:2 & 0xffff)` |
| `ldr` at 0xc4784 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4788 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xc4794 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4798 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc479c | store | 4 | `[sp+20] +0x0` | `[sp-36]` |
| `ldr` at 0xc47a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc47a4 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xc47ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc47b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc47b4 | store | 4 | `[sp-40] +0xa8` | `[sp-36]` |
| `ldr` at 0xc47b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc47bc | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xc47c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc47c8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0xc47d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc47d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc47d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4654 | `(unknown callee) 0x1b1b4` | `0xac` | `?` | `?` | `[[sp+16]]` |
| `bl` at 0xc466c | `(unknown callee) 0x1b394` | `ret(0x1b1b4)` | `[sp-20]` | `[sp-56]` | `ret(0x1b1b4)` |
| `bl` at 0xc469c | `(unknown callee) 0x1aff8` | `[[sp-36]+16]` | `?` | `[sp-36]` | `[[sp-36]+16]` |
| `bl` at 0xc46c8 | `(unknown callee) 0x1b298` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |
| `bl` at 0xc46fc | `(unknown callee) 0x1b298` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |
| `bl` at 0xc4730 | `(unknown callee) 0x1b298` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |
| `bl` at 0xc4764 | `(unknown callee) 0x1b298` | `[sp-60]:2` | `?` | `[sp-36]` | `[sp-60]:2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc42f0 | `[[sp-24]] == 0x6` | `[[sp-24]] != 0x6` |
| `beq` at 0xc461c | `[sp-56] == 0xa8` | `[sp-56] != 0xa8` |
| `blt` at 0xc4648 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc4790 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc47d0 | `r0` | `?` |

## `pkw`

- `pkw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x618fc to 0x61974.
  Established. The absolute word at `0x2b16a0` is read (`ldr` at 0x61920). The indirect call at
  `0x61938` has an unknown target and entry candidates `r0`, `r1`, `r2`, `r3` (`mov` at 0x61938).
  The unnamed `0x2079c` call carries `0x10d434`, `[sp-24]`, `[sp-28]`, `[sp-32]` (`bl` at 0x6195c).
  Write callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6190c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x61910 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x61914 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x61918 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x6191c | load | 4 | `abs 0x6196c` | `-` |
| `ldr` at 0x61920 | load | 4 | `abs 0x2b16a0` | `-` |
| `ldr` at 0x61924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61928 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6192c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61930 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61940 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x61944 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x61948 | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x6194c | load | 4 | `abs 0x61970` | `-` |
| `ldr` at 0x61950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61958 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61960 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x61968 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x61968 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x61968 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x61938 | `(unknown callee) (unknown target)` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x6195c | `(unknown callee) 0x2079c` | `0x10d434` | `[sp-24]` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x61968 | `r0` | `[sp-36]` |

## `idcode_fill`

- `idcode_fill`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9208 to 0xe931c.
  Established. The word assignment through `r1 +0x0` writes `[sp-28]` (`str` at 0xe9250). Included
  later word assignments through `[sp-24] +0x0` write `[sp-32]`, `[sp-36]` and `[sp-40]` (`str` at 0xe9294, `str` at 0xe92d8, `str` at 0xe930c). ID code field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9218 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe921c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe9220 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe9230 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xe9234 | store | 4 | `sp -0x1c` | `[r1]` |
| `ldr` at 0xe923c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9248 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe924c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9250 | store | 4 | `r1 +0x0` | `[sp-28]` |
| `ldr` at 0xe9254 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe9268 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe926c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0xe9270 | load | 4 | `abs 0xe9314` | `-` |
| `ldr` at 0xe927c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe928c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe9290 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9294 | store | 4 | `[sp-24] +0x0` | `[sp-32]` |
| `ldr` at 0xe9298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe929c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe92ac | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe92b0 | store | 4 | `sp -0x24` | `[[sp-24]]` |
| `ldr` at 0xe92b4 | load | 4 | `abs 0xe9318` | `-` |
| `ldr` at 0xe92c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe92d0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe92d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe92d8 | store | 4 | `[sp-24] +0x0` | `[sp-36]` |
| `ldr` at 0xe92dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe92e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe92e8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe92ec | store | 4 | `sp -0x28` | `[[sp-24]]` |
| `ldr` at 0xe92f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe9304 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe9308 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe930c | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `ldmdb` at 0xe9310 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9310 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9310 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9310 | `r0` | `[[sp-24]]` |

## `memory_access_function`

- `memory_access_function`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5738 to 0xe5a00.
  Established. The absolute word at `0x2adfd8` is read (`ldr` at 0xe57d4); its unsigned value at
  most `0x1` takes `0xe581c`, while larger values fall through (`bls` at 0xe57dc). The unnamed
  `0x61974` call carries `0x1`, `[sp-24]`, unknown, unknown (`bl` at 0xe5984). Memory operation
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5748 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe5750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5758 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xe576c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5774 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xe577c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe5780 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe5788 | store | 4 | `sp -0x38` | `[sp-56]-2` |
| `ldr` at 0xe5794 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe579c | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xe57a4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe57a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe57b0 | store | 4 | `sp -0x38` | `[sp-56]-1` |
| `ldr` at 0xe57b4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe57b8 | store | 4 | `sp -0x20` | `[sp-56]` |
| `ldr` at 0xe57bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe57c0 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xe57cc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe57d0 | load | 4 | `abs 0xe59dc` | `-` |
| `ldr` at 0xe57d4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe57e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe57ec | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xe57fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5804 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xe5810 | load | 4 | `abs 0xe59e0` | `-` |
| `ldr` at 0xe5820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5828 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5840 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe585c | store | 4 | `sp -0x18` | `ret(0xebe18)` |
| `ldr` at 0xe5860 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe586c | load | 4 | `abs 0xe59e4` | `-` |
| `str` at 0xe5878 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe5880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe588c | load | 4 | `abs 0xe59e8` | `-` |
| `str` at 0xe5898 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe58a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe58ac | load | 4 | `abs 0xe59ec` | `-` |
| `str` at 0xe58b8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe58c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe58d0 | load | 4 | `abs 0xe59f0` | `-` |
| `str` at 0xe58dc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe58e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe58f0 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xe58f4 | store | 4 | `sp -0x28` | `[[sp-20]+16]` |
| `ldr` at 0xe58fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5904 | load | 4 | `[sp-20] +0x14` | `-` |
| `str` at 0xe5908 | store | 4 | `sp -0x2c` | `[[sp-20]+20]` |
| `ldr` at 0xe590c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5914 | store | 4 | `sp -0x30` | `[sp-20]+24` |
| `ldr` at 0xe5918 | load | 4 | `abs 0xe59dc` | `-` |
| `ldr` at 0xe591c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe5928 | load | 4 | `abs 0xe59f4` | `-` |
| `ldr` at 0xe592c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5930 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe5934 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe5940 | store | 4 | `sp -0x4c` | `0xff` |
| `ldr` at 0xe5944 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5948 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe594c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe5950 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe5954 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe5968 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe596c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe5970 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe5978 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe598c | store | 4 | `sp -0x1c` | `ret(0x61974)` |
| `ldr` at 0xe5990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe599c | load | 4 | `abs 0xe59f8` | `-` |
| `ldr` at 0xe59a0 | load | 4 | `abs 0xe59fc` | `-` |
| `ldr` at 0xe59a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe59b0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe59bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe59c8 | store | 4 | `[sp-20] +0xc` | `0x0` |
| `str` at 0xe59d0 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xe59d4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xe59d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe59d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe59d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5818 | `(unknown callee) 0x23fbc` | `0x136370` | `?` | `?` | `?` |
| `bl` at 0xe5854 | `(unknown callee) 0xebe18` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xe5870 | `(unknown callee) 0x2079c` | `0x136398` | `?` | `?` | `ret(0xebe18)` |
| `bl` at 0xe5890 | `(unknown callee) 0x2079c` | `0x1363c4` | `?` | `?` | `[sp-36]` |
| `bl` at 0xe58b0 | `(unknown callee) 0x2079c` | `0x1363dc` | `?` | `?` | `[sp-32]` |
| `bl` at 0xe58d4 | `(unknown callee) 0x2079c` | `0x1363f8` | `?` | `?` | `?` |
| `bl` at 0xe5938 | `(unknown callee) 0x23fbc` | `0x136418` | `[sp-24]` | `[[sp-20]+16]` | `[[sp-20]+20]` |
| `bl` at 0xe5984 | `(unknown callee) 0x61974` | `0x1` | `[sp-24]` | `?` | `?` |
| `bl` at 0xe59a8 | `(unknown callee) 0x1b52c` | `0x2b4528` | `0x136448` | `ret(0x61974)` | `ret(0x61974)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe5764 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xe57dc | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xe5868 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xe5888 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0xe58a8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xe58cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe5924 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0xe5998 | `ret(0x61974) == 0x0` | `ret(0x61974) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe59d8 | `r0` | `?` |

## `JTAG_Cfg_display_stub`

- `JTAG_Cfg_display_stub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8554 to 0xe8564.
  Established. The return reports entry `r0` (`ldmdb` at 0xe8560); its explicit word reads use `sp -0x10`, `sp -0xc` and `sp -0x8` (`ldmdb` at 0xe8560). Display configuration effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0xe8560 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8560 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8560 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8560 | `r0` | `r0` |

## `sernumGetKey`

- `sernumGetKey`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4fad4 to 0x4fc44.
  Established. The unnamed `0xd2790` call carries `0x0`, `sp-28`, unknown, `sp-28` (`bl` at 0x4fb08). Included word reads use `[sp-28] +0x6c`, `[sp-28] +0x70` and `[sp-28] +0x74` (`ldr` at 0x4fb60, `ldr` at 0x4fb54, `ldr` at 0x4fb6c). The word at `sp -0x50` receives `0x2` on an included
  path (`str` at 0x4fc1c). Serial key meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4fae4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4fae8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4faf8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x4fb20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4fb28 | store | 4 | `sp -0x1c` | `[sp-28]+480` |
| `ldr` at 0x4fb2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fb30 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x4fb3c | load | 4 | `abs 0x4fc34` | `-` |
| `str` at 0x4fb48 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x4fb50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fb54 | load | 4 | `[sp-28] +0x70` | `-` |
| `str` at 0x4fb58 | store | 4 | `sp -0x3c` | `[[sp-28]+112]` |
| `ldr` at 0x4fb5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fb60 | load | 4 | `[sp-28] +0x6c` | `-` |
| `str` at 0x4fb64 | store | 4 | `sp -0x38` | `[[sp-28]+108]` |
| `ldr` at 0x4fb68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4fb6c | load | 4 | `[sp-28] +0x74` | `-` |
| `str` at 0x4fb70 | store | 4 | `sp -0x40` | `[[sp-28]+116]` |
| `ldr` at 0x4fb74 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x4fb78 | store | 4 | `sp -0x2c` | `[[sp-28]+108]` |
| `ldr` at 0x4fb7c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4fb80 | store | 4 | `sp -0x28` | `[[sp-28]+112]` |
| `ldr` at 0x4fb84 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x4fb88 | store | 4 | `sp -0x20` | `[[sp-28]+116]` |
| `ldr` at 0x4fb8c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x4fb98 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x4fb98 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x4fb98 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x4fb98 | load | 4 | `sp -0x20` | `-` |
| `stm` at 0x4fb9c | store | 4 | `[sp-20] +0x0` | `[[sp-28]+108]` |
| `stm` at 0x4fb9c | store | 4 | `[sp-20] +0x4` | `[[sp-28]+112]` |
| `stm` at 0x4fb9c | store | 4 | `[sp-20] +0x8` | `[sp-36]` |
| `stm` at 0x4fb9c | store | 4 | `[sp-20] +0xc` | `[[sp-28]+116]` |
| `ldr` at 0x4fba0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4fba4 | load | 4 | `abs 0x4fc38` | `-` |
| `ldr` at 0x4fbb0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4fbb4 | load | 4 | `abs 0x4fc38` | `-` |
| `ldr` at 0x4fbc0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x4fbc4 | load | 4 | `abs 0x4fc38` | `-` |
| `ldr` at 0x4fbd0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4fbdc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x4fbe0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4fbf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4fbfc | load | 4 | `abs 0x4fc3c` | `-` |
| `ldr` at 0x4fc04 | load | 4 | `abs 0x4fc40` | `-` |
| `ldr` at 0x4fc08 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4fc0c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x4fc10 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x4fc1c | store | 4 | `sp -0x50` | `0x2` |
| `str` at 0x4fc28 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0x4fc2c | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0x4fc30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4fc30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4fc30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4faec | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4fb08 | `(unknown callee) 0xd2790` | `0x0` | `sp-28` | `?` | `sp-28` |
| `bl` at 0x4fb0c | `(unknown callee) 0x4f4d0` | `ret(0xd2790)` | `?` | `?` | `?` |
| `bl` at 0x4fb40 | `(unknown callee) 0x202d0` | `0x10b668` | `?` | `?` | `[[sp-28]+28]` |
| `bl` at 0x4fc00 | `(unknown callee) 0x23fbc` | `0x10b69c` | `[[sp-28]+112]` | `?` | `[sp-24]` |
| `bl` at 0x4fc14 | `(unknown callee) 0x2079c` | `0x10b6cc` | `[sp-56]` | `[sp-60]` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4fb1c | `? != 0x5e` | `? == 0x5e` |
| `beq` at 0x4fb38 | `[[sp-28]+28] == 0x0` | `[[sp-28]+28] != 0x0` |
| `bhi` at 0x4fbac | `[sp-60] > 0xea60 (unsigned, 32-bit)` | `[sp-60] <= 0xea60 (unsigned, 32-bit)` |
| `bhi` at 0x4fbbc | `[sp-56] > 0xea60 (unsigned, 32-bit)` | `[sp-56] <= 0xea60 (unsigned, 32-bit)` |
| `bhi` at 0x4fbcc | `[sp-64] > 0xea60 (unsigned, 32-bit)` | `[sp-64] <= 0xea60 (unsigned, 32-bit)` |
| `bne` at 0x4fbd8 | `[sp-56] != 0x2` | `[sp-56] == 0x2` |
| `bhi` at 0x4fbe8 | `[sp-64] > [sp-60] (unsigned, 32-bit)` | `[sp-64] <= [sp-60] (unsigned, 32-bit)` |
| `beq` at 0x4fbf8 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4fc30 | `r0` | `?` |

## `sernumUnscrambleExtra`

- `sernumUnscrambleExtra`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5038c to 0x504d0.
  Established. The unnamed `0x5062c` call at `0x503d4` carries `[sp-24]`, `0x1`, `0x1`,
  `ret(0x5062c)` (`bl` at 0x503d4). A byte assignment at an unknown target writes `(ret(0x504d0) & 0xff)` (`strb` at 0x50490). Another byte assignment at an unknown target writes `0x0` (`strb` at 0x504c0). Extra serial-data decoding policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5039c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x503a0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x503a4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x503ac | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x503b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x503c4 | store | 4 | `sp -0x24` | `ret(0x5062c)` |
| `ldr` at 0x503c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x503dc | store | 4 | `sp -0x30` | `ret(0x5062c)` |
| `ldr` at 0x503e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x503f0 | store | 4 | `sp -0x2c` | `ret(0x1b2bc)-1` |
| `ldr` at 0x503f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50408 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x50414 | store | 4 | `sp -0x28` | `ret(0x5062c)` |
| `ldr` at 0x5041c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x50428 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5043c | store | 4 | `sp -0x28` | `ret(0xfb218)` |
| `str` at 0x50448 | store | 4 | `sp -0x34` | `0x2` |
| `str` at 0x50450 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x50454 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x50458 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5046c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x50474 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50478 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x50480 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x50490 | store | 1 | `(unknown)` | `(ret(0x504d0) & 0xff)` |
| `ldr` at 0x50494 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x5049c | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x504a0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x504a8 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x504b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x504b4 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x504c0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x504c4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x504cc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x504cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x504cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x504cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x503bc | `(unknown callee) 0x5062c` | `r0` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x503d4 | `(unknown callee) 0x5062c` | `[sp-24]` | `0x1` | `0x1` | `ret(0x5062c)` |
| `bl` at 0x503e4 | `(unknown callee) 0x1b2bc` | `[sp-24]` | `?` | `?` | `ret(0x5062c)` |
| `bl` at 0x5040c | `(unknown callee) 0x5062c` | `[sp-24]` | `0x2` | `ret(0x1b2bc)-1` | `[sp-36]` |
| `bl` at 0x50420 | `(unknown callee) 0x505c4` | `0xa` | `[sp-48]` | `?` | `ret(0x5062c)` |
| `bl` at 0x50434 | `(unknown callee) 0xfb218` | `[sp-40]` | `ret(0x505c4)` | `ret(0x505c4)` | `[sp-40]` |
| `bl` at 0x50488 | `(unknown callee) 0x504d0` | `?` | `?` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x503fc | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `ble` at 0x50460 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x504cc | `r0` | `[sp-40]` |

## `brickSlotLabel`

- `brickSlotLabel`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f9a4 to 0x4f9e0.
  Established. The absolute byte at `0x148f84` receives `0x0` (`strb` at 0x4f9cc). The return
  reports `0x148f84` (`ldmdb` at 0x4f9d8). Slot label interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f9b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4f9b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4f9bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x4f9c0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x4f9c4 | load | 4 | `abs 0x4f9dc` | `-` |
| `strb` at 0x4f9cc | store | 1 | `abs 0x148f84` | `0x0` |
| `ldr` at 0x4f9d0 | load | 4 | `abs 0x4f9dc` | `-` |
| `ldmdb` at 0x4f9d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f9d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f9d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f9d8 | `r0` | `0x148f84` |

## `pkr`

- `pkr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6187c to 0x618fc.
  Established. The absolute word at `0x2b16c0` is read (`ldr` at 0x618a0). The indirect call at
  `0x618b8` has an unknown target and entry candidates `r0`, `r1`, `r2`, `r3` (`mov` at 0x618b8).
  The unnamed `0x2079c` call carries `0x10d424`, `[sp-24]`, `[[sp-28]]`, `[[sp-32]]` (`bl` at 0x618e4). Access callback semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6188c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x61890 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x61894 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x61898 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x6189c | load | 4 | `abs 0x618f4` | `-` |
| `ldr` at 0x618a0 | load | 4 | `abs 0x2b16c0` | `-` |
| `ldr` at 0x618a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x618a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x618ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x618b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x618c0 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x618c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x618c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x618cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x618d0 | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x618d4 | load | 4 | `abs 0x618f8` | `-` |
| `ldr` at 0x618d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x618dc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x618e0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x618e8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x618f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x618f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x618f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x618b8 | `(unknown callee) (unknown target)` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x618e4 | `(unknown callee) 0x2079c` | `0x10d424` | `[sp-24]` | `[[sp-28]]` | `[[sp-32]]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x618f0 | `r0` | `[sp-36]` |
