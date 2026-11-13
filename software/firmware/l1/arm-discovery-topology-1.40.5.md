# ARM discovery and topology entry paths in 1.40.5

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
L1CfgInit has an unresolved computed transfer. Tables cover known edges only.
Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/discovery-topology-facts.txt) retain all tool sections.

## `L1CfgInit`

- `L1CfgInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b180 to 0x6ba6c.
  Established. Included four-byte absolute stores write `0x10` at `0x14973c`, `0x0` at `0x149738`
  and `ret(0x1b1b4)` at `0x149740` (`str` at 0x6b2d4, `str` at 0x6b2e0, `str` at 0x6b30c). The
  unresolved computed transfer takes unsigned `[0x1496bc]-48 <= 0x4c` and falls through for
  `[0x1496bc]-48 > 0x4c` (`ldrls` at 0x6b60c). On known edges the later unnamed `0xb4a70` call
  carries `0x1496a4`, `0xffff00e`, `0x3`, `0x73278` (`bl` at 0x6b978). Configuration selector and
  registration meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b19c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6b1a0 | load | 4 | `abs 0x6ba04` | `-` |
| `ldr` at 0x6b1a4 | load | 4 | `abs 0x1493d0` | `-` |
| `ldr` at 0x6b1b0 | load | 4 | `abs 0x6ba08` | `-` |
| `str` at 0x6b1b4 | store | 4 | `sp -0x40` | `0x10fc08` |
| `ldr` at 0x6b1bc | load | 4 | `abs 0x6ba0c` | `-` |
| `ldr` at 0x6b1cc | load | 4 | `abs 0x6ba04` | `-` |
| `ldr` at 0x6b1d0 | load | 4 | `abs 0x6ba04` | `-` |
| `ldr` at 0x6b1d4 | load | 4 | `abs 0x1493d0` | `-` |
| `str` at 0x6b1dc | store | 4 | `abs 0x1493d0` | `[0x1493d0]+1` |
| `ldr` at 0x6b1e0 | load | 4 | `abs 0x6ba10` | `-` |
| `str` at 0x6b1ec | store | 4 | `abs 0x1493e8` | `ret(0x1b274)` |
| `str` at 0x6b208 | store | 4 | `sp -0x20` | `ret(0x1b2bc)` |
| `ldr` at 0x6b20c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6b238 | load | 4 | `abs 0x6ba14` | `-` |
| `ldr` at 0x6b244 | load | 4 | `abs 0x6ba18` | `-` |
| `str` at 0x6b24c | store | 4 | `abs 0x1493d4` | `0x100` |
| `ldr` at 0x6b250 | load | 4 | `abs 0x6ba18` | `-` |
| `str` at 0x6b258 | store | 4 | `abs 0x1493d4` | `0x1` |
| `ldr` at 0x6b25c | load | 4 | `abs 0x6ba18` | `-` |
| `str` at 0x6b264 | store | 4 | `abs 0x1493d4` | `0x10` |
| `ldr` at 0x6b268 | load | 4 | `abs 0x6ba18` | `-` |
| `ldr` at 0x6b26c | load | 4 | `abs 0x6ba1c` | `-` |
| `str` at 0x6b270 | store | 4 | `abs 0x1493d4` | `0x3ff` |
| `ldr` at 0x6b274 | load | 4 | `abs 0x6ba18` | `-` |
| `str` at 0x6b27c | store | 4 | `abs 0x1493d4` | `0x0` |
| `ldr` at 0x6b280 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6b284 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x6b288 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b294 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b2a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b2b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6b2c0 | load | 4 | `abs 0x6ba20` | `-` |
| `str` at 0x6b2c8 | store | 4 | `abs 0x143578` | `0xc` |
| `ldr` at 0x6b2cc | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b2d4 | store | 4 | `abs 0x14973c` | `0x10` |
| `ldr` at 0x6b2d8 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b2e0 | store | 4 | `abs 0x149738` | `0x0` |
| `ldr` at 0x6b2e4 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b2e8 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b2ec | load | 4 | `abs 0x14973c` | `-` |
| `str` at 0x6b30c | store | 4 | `abs 0x149740` | `ret(0x1b1b4)` |
| `ldr` at 0x6b310 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b314 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b318 | load | 4 | `abs 0x14973c` | `-` |
| `ldr` at 0x6b32c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6b33c | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b344 | store | 4 | `abs 0x149730` | `0xd` |
| `ldr` at 0x6b348 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b34c | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b350 | load | 4 | `abs 0x149730` | `-` |
| `str` at 0x6b378 | store | 4 | `abs 0x149734` | `ret(0x1b1b4)` |
| `ldr` at 0x6b37c | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b380 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b384 | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x6b3a0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6b3b0 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b3b8 | store | 4 | `abs 0x1496ac` | `0x1` |
| `ldr` at 0x6b3bc | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b3c4 | store | 4 | `abs 0x14970c` | `0x2` |
| `ldr` at 0x6b3c8 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b3cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6b3d0 | store | 4 | `abs 0x1496bc` | `[sp-36]` |
| `ldr` at 0x6b3d4 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b3e0 | store | 4 | `abs 0x1496b4` | `ret(0x1f26c)` |
| `ldr` at 0x6b3e4 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b3f0 | store | 4 | `abs 0x1496b8` | `ret(0x1f26c)` |
| `ldr` at 0x6b3f4 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b400 | store | 4 | `abs 0x1496d4` | `ret(0x1f26c)` |
| `ldr` at 0x6b404 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b408 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6b410 | store | 4 | `abs 0x14971c` | `ret(0x75424)` |
| `ldr` at 0x6b414 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b420 | store | 4 | `abs 0x1496e8` | `ret(0x1f26c)` |
| `ldr` at 0x6b424 | load | 4 | `abs 0x1496e8` | `-` |
| `ldr` at 0x6b430 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b438 | store | 4 | `abs 0x1496e8` | `0x0` |
| `ldr` at 0x6b454 | load | 4 | `abs 0x6ba28` | `-` |
| `ldr` at 0x6b460 | load | 4 | `abs 0x6ba2c` | `-` |
| `ldr` at 0x6b464 | load | 4 | `abs 0x6ba14` | `-` |
| `ldr` at 0x6b46c | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b478 | store | 4 | `abs 0x1496c0` | `ret(0x5f314)` |
| `ldr` at 0x6b47c | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b484 | store | 4 | `abs 0x1496c4` | `0x0` |
| `ldr` at 0x6b488 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b490 | store | 4 | `abs 0x1496d8` | `0x0` |
| `ldr` at 0x6b494 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b49c | store | 4 | `abs 0x1496e0` | `ret(0x3dce4)` |
| `ldr` at 0x6b4a0 | load | 4 | `abs 0x6ba24` | `-` |
| `str` at 0x6b4a8 | store | 4 | `abs 0x1496e4` | `ret(0x3dcc4)` |
| `ldr` at 0x6b4ac | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b4b4 | store | 4 | `abs 0x149758` | `0x1` |
| `ldr` at 0x6b4b8 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b4c0 | store | 4 | `abs 0x149784` | `0x2` |
| `ldr` at 0x6b4c4 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b4cc | store | 4 | `abs 0x1497a0` | `0x3` |
| `ldr` at 0x6b4d0 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b4d8 | store | 4 | `abs 0x1497a4` | `0x4` |
| `ldr` at 0x6b4dc | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b4e4 | store | 4 | `abs 0x1497a8` | `0x5` |
| `ldr` at 0x6b4e8 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b4f0 | store | 4 | `abs 0x1497ac` | `0x6` |
| `ldr` at 0x6b4f4 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b4fc | store | 4 | `abs 0x1497b0` | `0x7` |
| `ldr` at 0x6b500 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b508 | store | 4 | `abs 0x1497b4` | `0x8` |
| `ldr` at 0x6b50c | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b514 | store | 4 | `abs 0x149754` | `0x9` |
| `ldr` at 0x6b518 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b520 | store | 4 | `abs 0x14975c` | `0xa` |
| `ldr` at 0x6b524 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b52c | store | 4 | `abs 0x149788` | `0xb` |
| `ldr` at 0x6b530 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b538 | store | 4 | `abs 0x14978c` | `0xc` |
| `ldr` at 0x6b53c | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b544 | store | 4 | `abs 0x149760` | `0x9` |
| `ldr` at 0x6b548 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b550 | store | 4 | `abs 0x149764` | `0xa` |
| `ldr` at 0x6b554 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b55c | store | 4 | `abs 0x149780` | `0xb` |
| `ldr` at 0x6b560 | load | 4 | `abs 0x6ba30` | `-` |
| `str` at 0x6b568 | store | 4 | `abs 0x149774` | `0xc` |
| `ldr` at 0x6b56c | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b574 | store | 4 | `abs 0x149854` | `0x2` |
| `ldr` at 0x6b578 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b580 | store | 4 | `abs 0x149858` | `0xd` |
| `ldr` at 0x6b584 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b58c | store | 4 | `abs 0x14985c` | `0x14` |
| `ldr` at 0x6b590 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b598 | store | 4 | `abs 0x149860` | `0x15` |
| `ldr` at 0x6b59c | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5a4 | store | 4 | `abs 0x149864` | `0x16` |
| `ldr` at 0x6b5a8 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5b0 | store | 4 | `abs 0x149868` | `0x17` |
| `ldr` at 0x6b5b4 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5bc | store | 4 | `abs 0x14986c` | `0x18` |
| `ldr` at 0x6b5c0 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5c8 | store | 4 | `abs 0x149870` | `0x19` |
| `ldr` at 0x6b5cc | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5d4 | store | 4 | `abs 0x149874` | `0x1` |
| `ldr` at 0x6b5d8 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5e0 | store | 4 | `abs 0x149878` | `0x3` |
| `ldr` at 0x6b5e4 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5ec | store | 4 | `abs 0x14987c` | `0xe` |
| `ldr` at 0x6b5f0 | load | 4 | `abs 0x6ba34` | `-` |
| `str` at 0x6b5f8 | store | 4 | `abs 0x149880` | `0xf` |
| `ldr` at 0x6b5fc | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b600 | load | 4 | `abs 0x1496bc` | `-` |
| `ldrls` at 0x6b60c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6b7e4 | load | 4 | `abs 0x6ba38` | `-` |
| `ldr` at 0x6b7e8 | load | 4 | `abs 0x6ba3c` | `-` |
| `ldr` at 0x6b7f8 | load | 4 | `abs 0x6ba40` | `-` |
| `ldr` at 0x6b808 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b80c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b818 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b81c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b828 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b82c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b838 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b83c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b848 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b84c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b858 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b85c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b868 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b86c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b878 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b87c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b888 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b88c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b898 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b89c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b8a8 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b8ac | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b8b8 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b8bc | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b8c8 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b8cc | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6b8d8 | load | 4 | `abs 0x6ba24` | `-` |
| `ldr` at 0x6b8dc | load | 4 | `abs 0x1496bc` | `-` |
| `str` at 0x6b8f0 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x6b8f8 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x6b900 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x6b908 | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x6b910 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x6b914 | load | 4 | `abs 0x6ba44` | `-` |
| `ldr` at 0x6b918 | load | 4 | `abs 0x6ba48` | `-` |
| `ldr` at 0x6b91c | load | 4 | `abs 0x6ba4c` | `-` |
| `str` at 0x6b930 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x6b938 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x6b940 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x6b948 | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x6b950 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x6b954 | load | 4 | `abs 0x6ba44` | `-` |
| `ldr` at 0x6b958 | load | 4 | `abs 0x6ba48` | `-` |
| `ldr` at 0x6b95c | load | 4 | `abs 0x6ba50` | `-` |
| `ldr` at 0x6b968 | load | 4 | `abs 0x6ba40` | `-` |
| `ldr` at 0x6b96c | load | 4 | `abs 0x6ba54` | `-` |
| `ldr` at 0x6b974 | load | 4 | `abs 0x6ba58` | `-` |
| `str` at 0x6b980 | store | 4 | `sp -0x1c` | `ret(0xb4a70)` |
| `ldr` at 0x6b984 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6b990 | load | 4 | `abs 0x6ba5c` | `-` |
| `str` at 0x6b994 | store | 4 | `sp -0x40` | `0x10fc54` |
| `ldr` at 0x6b998 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6b99c | store | 4 | `sp -0x3c` | `ret(0xb4a70)` |
| `ldr` at 0x6b9a4 | load | 4 | `abs 0x6ba0c` | `-` |
| `ldr` at 0x6b9a8 | load | 4 | `abs 0x6ba60` | `-` |
| `ldr` at 0x6b9b4 | load | 4 | `abs 0x6ba40` | `-` |
| `ldr` at 0x6b9c0 | load | 4 | `abs 0x6ba58` | `-` |
| `str` at 0x6b9cc | store | 4 | `sp -0x1c` | `ret(0xb4efc)` |
| `ldr` at 0x6b9d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6b9dc | load | 4 | `abs 0x6ba64` | `-` |
| `str` at 0x6b9e0 | store | 4 | `sp -0x40` | `0x10fc80` |
| `ldr` at 0x6b9e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6b9e8 | store | 4 | `sp -0x3c` | `ret(0xb4efc)` |
| `ldr` at 0x6b9f0 | load | 4 | `abs 0x6ba0c` | `-` |
| `ldr` at 0x6b9f4 | load | 4 | `abs 0x6ba68` | `-` |
| `ldmdb` at 0x6ba00 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6ba00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ba00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ba00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b190 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x6b1c8 | `(unknown callee) 0x21eb8` | `0x9` | `0x10fc24` | `0x6d` | `0x0` |
| `bl` at 0x6b1e8 | `(unknown callee) 0x1b274` | `0x0` | `?` | `0x1493d0` | `[0x1493d0]+1` |
| `bl` at 0x6b1f4 | `(unknown callee) 0x1f26c` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x6b200 | `(unknown callee) 0x1b2bc` | `ret(0x1f26c)` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x6b218 | `(unknown callee) 0x4f730` | `ret(0x1b2bc)` | `?` | `?` | `ret(0x1b2bc)` |
| `bl` at 0x6b228 | `(unknown callee) 0x1d41c` | `0xf` | `ret(0x4f730)` | `?` | `ret(0x4f730)` |
| `bl` at 0x6b230 | `(unknown callee) 0x1f26c` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x6b240 | `(unknown callee) 0x1b598` | `0x2b28c8` | `ret(0x1f26c)` | `?` | `ret(0x1f26c)` |
| `bl` at 0x6b304 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[0x14973c]` | `?` |
| `bl` at 0x6b338 | `(unknown callee) 0x1b4cc` | `[0x149740]` | `0x0` | `?` | `?` |
| `bl` at 0x6b370 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[0x149730]` | `?` |
| `bl` at 0x6b3ac | `(unknown callee) 0x1b4cc` | `[0x149734]` | `0x0` | `?` | `?` |
| `bl` at 0x6b3dc | `(unknown callee) 0x1f26c` | `0x1` | `?` | `0x1496ac` | `[sp-36]` |
| `bl` at 0x6b3ec | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6b3fc | `(unknown callee) 0x1f26c` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x6b40c | `(unknown callee) 0x75424` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x6b41c | `(unknown callee) 0x1f26c` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x6b444 | `(unknown callee) 0x1d41c` | `0xe` | `0x0` | `0x1496ac` | `0x0` |
| `bl` at 0x6b44c | `(unknown callee) 0x1f26c` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x6b45c | `(unknown callee) 0x1b598` | `0x1496ec` | `ret(0x1f26c)` | `?` | `ret(0x1f26c)` |
| `bl` at 0x6b468 | `(unknown callee) 0x1b598` | `0x1496f8` | `0x2b28c8` | `?` | `?` |
| `bl` at 0x6b474 | `(unknown callee) 0x5f314` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6b498 | `(unknown callee) 0x3dce4` | `ret(0x5f314)` | `?` | `0x1496ac` | `0x0` |
| `bl` at 0x6b4a4 | `(unknown callee) 0x3dcc4` | `ret(0x3dce4)` | `?` | `?` | `?` |
| `bl` at 0x6b7e0 | `(unknown callee) 0x6ba6c` | `ret(0x3dcc4)` | `?` | `0x149850` | `[0x1496bc]-48` |
| `bl` at 0x6b7f4 | `(unknown callee) 0x229b0` | `0x14964c` | `0x10fc3c` | `0x1` | `0x9` |
| `bl` at 0x6b804 | `(unknown callee) 0xb5e34` | `0x1496a4` | `0xe` | `0x0` | `?` |
| `bl` at 0x6b924 | `(unknown callee) 0x22480` | `0x1496a0` | `0x10fc48` | `0x6bdc4` | `0x0` |
| `bl` at 0x6b964 | `(unknown callee) 0x22480` | `0x1496a0` | `0x10fc48` | `0x6bbac` | `0x0` |
| `bl` at 0x6b978 | `(unknown callee) 0xb4a70` | `0x1496a4` | `0xffff00e` | `0x3` | `0x73278` |
| `bl` at 0x6b9b0 | `(unknown callee) 0x21eb8` | `0x9` | `0x10fc24` | `0x129` | `0x0` |
| `bl` at 0x6b9c4 | `(unknown callee) 0xb4efc` | `0x1496a4` | `?` | `0x0` | `0x73278` |
| `bl` at 0x6b9fc | `(unknown callee) 0x21eb8` | `0x9` | `0x10fc24` | `0x12e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6b1ac | `[0x1493d0] == 0x0` | `[0x1493d0] != 0x0` |
| `beq` at 0x6b214 | `ret(0x1b2bc) == 0x6` | `ret(0x1b2bc) != 0x6` |
| `beq` at 0x6b290 | `[sp-36] == 0x46` | `[sp-36] != 0x46` |
| `bgt` at 0x6b29c | `[sp-36] > 0x46 (signed, 32-bit)` | `[sp-36] <= 0x46 (signed, 32-bit)` |
| `beq` at 0x6b2a8 | `[sp-36] == 0x42` | `[sp-36] != 0x42` |
| `beq` at 0x6b2b8 | `[sp-36] == 0x48` | `[sp-36] != 0x48` |
| `ble` at 0x6b42c | `[0x1496e8] <= 0xfe (signed, 32-bit)` | `[0x1496e8] > 0xfe (signed, 32-bit)` |
| `ldrls` at 0x6b60c | `[0x1496bc]-48 <= 0x4c (unsigned, 32-bit)` | `[0x1496bc]-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0x6b814 | `[0x1496bc] == 0x35` | `[0x1496bc] != 0x35` |
| `beq` at 0x6b824 | `[0x1496bc] == 0x4f` | `[0x1496bc] != 0x4f` |
| `beq` at 0x6b834 | `[0x1496bc] == 0x42` | `[0x1496bc] != 0x42` |
| `beq` at 0x6b844 | `[0x1496bc] == 0x46` | `[0x1496bc] != 0x46` |
| `beq` at 0x6b854 | `[0x1496bc] == 0x48` | `[0x1496bc] != 0x48` |
| `beq` at 0x6b864 | `[0x1496bc] == 0x4a` | `[0x1496bc] != 0x4a` |
| `beq` at 0x6b874 | `[0x1496bc] == 0x4b` | `[0x1496bc] != 0x4b` |
| `beq` at 0x6b884 | `[0x1496bc] == 0x21` | `[0x1496bc] != 0x21` |
| `beq` at 0x6b894 | `[0x1496bc] == 0x5e` | `[0x1496bc] != 0x5e` |
| `beq` at 0x6b8a4 | `[0x1496bc] == 0x2b` | `[0x1496bc] != 0x2b` |
| `beq` at 0x6b8b4 | `[0x1496bc] == 0x4c` | `[0x1496bc] != 0x4c` |
| `beq` at 0x6b8c4 | `[0x1496bc] == 0x5a` | `[0x1496bc] != 0x5a` |
| `beq` at 0x6b8d4 | `[0x1496bc] == 0x7c` | `[0x1496bc] != 0x7c` |
| `beq` at 0x6b8e4 | `[0x1496bc] == 0x4d` | `[0x1496bc] != 0x4d` |
| `beq` at 0x6b98c | `ret(0xb4a70) == 0x0` | `ret(0xb4a70) != 0x0` |
| `beq` at 0x6b9d8 | `ret(0xb4efc) == 0x0` | `ret(0xb4efc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ba00 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6b60c).

## `L1CfgMeshTask`

- `L1CfgMeshTask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6bbac to 0x6bdc4.
  Established. The unnamed `0x6c95c` call has `0x10fca0`, unknown, `[0x149400]`, `ret(0x1b274)`
  (`bl` at 0x6bc18). A following four-byte absolute assignment at `0x149400` writes `0x0` (`str` at
  0x6bc24). Separate included four-byte stores at absolute `0x143580` write `0x5` and `0xf` (`str`
  at 0x6bc58, `str` at 0x6bd78). Mesh timing and state meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6bbbc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6bbc0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6bbc8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x6bbd0 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x6bbd8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6bbe4 | load | 4 | `abs 0x6bd9c` | `-` |
| `ldr` at 0x6bbe8 | load | 4 | `abs 0x149400` | `-` |
| `ldr` at 0x6bc00 | load | 4 | `abs 0x6bd9c` | `-` |
| `ldr` at 0x6bc04 | load | 4 | `abs 0x149400` | `-` |
| `ldr` at 0x6bc14 | load | 4 | `abs 0x6bda0` | `-` |
| `ldr` at 0x6bc1c | load | 4 | `abs 0x6bd9c` | `-` |
| `str` at 0x6bc24 | store | 4 | `abs 0x149400` | `0x0` |
| `ldr` at 0x6bc28 | load | 4 | `abs 0x6bda4` | `-` |
| `ldr` at 0x6bc2c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6bc3c | load | 4 | `abs 0x6bda8` | `-` |
| `ldr` at 0x6bc40 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6bc44 | load | 4 | `abs 0x6bdac` | `-` |
| `ldr` at 0x6bc50 | load | 4 | `abs 0x6bdb0` | `-` |
| `str` at 0x6bc58 | store | 4 | `abs 0x143580` | `0x5` |
| `ldr` at 0x6bc60 | load | 4 | `abs 0x6bdb4` | `-` |
| `ldr` at 0x6bc64 | load | 4 | `abs 0x1493f0` | `-` |
| `ldr` at 0x6bc7c | load | 4 | `abs 0x6bdb4` | `-` |
| `ldr` at 0x6bc80 | load | 4 | `abs 0x1493f0` | `-` |
| `str` at 0x6bc88 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6bc8c | load | 4 | `abs 0x6bdb8` | `-` |
| `ldr` at 0x6bc90 | load | 4 | `abs 0x1493f4` | `-` |
| `ldr` at 0x6bca8 | load | 4 | `abs 0x6bdb8` | `-` |
| `ldr` at 0x6bcac | load | 4 | `abs 0x1493f4` | `-` |
| `str` at 0x6bcb4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6bcb8 | load | 4 | `abs 0x6bdbc` | `-` |
| `ldr` at 0x6bcbc | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6bcc8 | load | 4 | `abs 0x6bdb0` | `-` |
| `ldr` at 0x6bccc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bcd0 | load | 4 | `abs 0x143580` | `-` |
| `str` at 0x6bce4 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x6bcec | load | 4 | `abs 0x6bdc0` | `-` |
| `ldr` at 0x6bcf0 | load | 4 | `abs 0x1493ec` | `-` |
| `ldr` at 0x6bcfc | load | 4 | `abs 0x6bdc0` | `-` |
| `ldr` at 0x6bd00 | load | 4 | `abs 0x6bdb4` | `-` |
| `ldr` at 0x6bd04 | load | 4 | `abs 0x1493f0` | `-` |
| `str` at 0x6bd08 | store | 4 | `abs 0x1493ec` | `[0x1493f0]` |
| `str` at 0x6bd14 | store | 4 | `sp -0x28` | `ret(0x1f26c)` |
| `ldr` at 0x6bd18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6bd1c | store | 4 | `sp -0x2c` | `ret(0x1f26c)` |
| `ldr` at 0x6bd20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6bd2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6bd3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6bd48 | load | 4 | `abs 0x6bdb0` | `-` |
| `ldr` at 0x6bd4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6bd50 | load | 4 | `abs 0x143580` | `-` |
| `ldr` at 0x6bd64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bd70 | load | 4 | `abs 0x6bdb0` | `-` |
| `str` at 0x6bd78 | store | 4 | `abs 0x143580` | `0xf` |
| `ldr` at 0x6bd80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6bd94 | store | 4 | `sp -0x24` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6bbe0 | `(unknown callee) 0x22780` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x6bbf8 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[0x149400]` |
| `bl` at 0x6bc18 | `(unknown callee) 0x6c95c` | `0x10fca0` | `?` | `[0x149400]` | `ret(0x1b274)` |
| `bl` at 0x6bc48 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fcb0` | `0x149400` | `0x145ae8` |
| `bl` at 0x6bc4c | `(unknown callee) 0x6dda8` | `?` | `?` | `?` | `?` |
| `bl` at 0x6bc74 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[0x1493f0]` |
| `bl` at 0x6bca0 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[0x1493f4]` |
| `bl` at 0x6bd10 | `(unknown callee) 0x1f26c` | `0x9a` | `?` | `?` | `?` |
| `bl` at 0x6bd60 | `(unknown callee) 0x6d804` | `ret(0x1f26c)` | `?` | `?` | `?` |
| `bl` at 0x6bd8c | `(unknown callee) 0x6d804` | `ret(0x1f26c)` | `?` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6bbf0 | `[0x149400] == 0x0` | `[0x149400] != 0x0` |
| `bge` at 0x6bc0c | `ret(0x1b274) >= [0x149400] (signed, 32-bit)` | `ret(0x1b274) < [0x149400] (signed, 32-bit)` |
| `beq` at 0x6bc38 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6bc6c | `[0x1493f0] == 0x0` | `[0x1493f0] != 0x0` |
| `beq` at 0x6bc98 | `[0x1493f4] == 0x0` | `[0x1493f4] != 0x0` |
| `beq` at 0x6bcc4 | `[0x149738] == 0x1` | `[0x149738] != 0x1` |
| `blt` at 0x6bcd8 | `[sp-28] < [0x143580] (signed, 32-bit)` | `[sp-28] >= [0x143580] (signed, 32-bit)` |
| `bne` at 0x6bcf8 | `[0x1493ec] != 0x0` | `[0x1493ec] == 0x0` |
| `beq` at 0x6bd28 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x6bd34 | `ret(0x1f26c) == 0x1` | `ret(0x1f26c) != 0x1` |
| `beq` at 0x6bd44 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bge` at 0x6bd58 | `[sp-32] >= [0x143580] (signed, 32-bit)` | `[sp-32] < [0x143580] (signed, 32-bit)` |
| `ble` at 0x6bd6c | `[sp-28] <= 0x3c (signed, 32-bit)` | `[sp-28] > 0x3c (signed, 32-bit)` |
| `beq` at 0x6bd88 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `L1CfgRingTask`

- `L1CfgRingTask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6bdc4 to 0x6c95c.
  Established. Two included four-byte stack assignments at `sp -0x44` and `sp -0x48` retain
  `[0x149734]+156` and `[0x149734]+312` (`str` at 0x6be5c, `str` at 0x6be6c). The unnamed `0xb785c`
  call carries `0x1496a4`, `0x0`, `0x64`, `0x2` (`bl` at 0x6c440); its nonzero result takes the
  branch to `0x6c45c` (`bne` at 0x6c454). That route includes a four-byte assignment of `[0x149b54]`
  at `ret(0xb785c) +0x18` (`str` at 0x6c468). Ring selection, timing and record meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6bdd4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6bdd8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6bde4 | load | 4 | `abs 0x6c924` | `-` |
| `ldr` at 0x6bde8 | load | 4 | `abs 0x149400` | `-` |
| `ldr` at 0x6be00 | load | 4 | `abs 0x6c924` | `-` |
| `ldr` at 0x6be04 | load | 4 | `abs 0x149400` | `-` |
| `ldr` at 0x6be14 | load | 4 | `abs 0x6c928` | `-` |
| `ldr` at 0x6be1c | load | 4 | `abs 0x6c924` | `-` |
| `str` at 0x6be24 | store | 4 | `abs 0x149400` | `0x0` |
| `ldr` at 0x6be28 | load | 4 | `abs 0x6c92c` | `-` |
| `ldr` at 0x6be2c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6be3c | load | 4 | `abs 0x6c930` | `-` |
| `ldr` at 0x6be40 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6be44 | load | 4 | `abs 0x6c934` | `-` |
| `ldr` at 0x6be50 | load | 4 | `abs 0x6c938` | `-` |
| `ldr` at 0x6be54 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6be5c | store | 4 | `sp -0x44` | `[0x149734]+156` |
| `ldr` at 0x6be60 | load | 4 | `abs 0x6c938` | `-` |
| `ldr` at 0x6be64 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6be6c | store | 4 | `sp -0x48` | `[0x149734]+312` |
| `ldr` at 0x6be98 | load | 4 | `abs 0x6c93c` | `-` |
| `str` at 0x6bea0 | store | 4 | `abs 0x1493d8` | `?` |
| `ldr` at 0x6bebc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bec0 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6becc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bed0 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bedc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bee0 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6beec | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bef0 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6befc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf00 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf0c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf10 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf1c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf20 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf2c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf30 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf3c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf40 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf4c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf50 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf5c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf60 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf6c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf70 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf7c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf80 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf8c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bf90 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bf9c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bfa0 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bfc0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bfc4 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6bfd0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bfd4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bfe0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bfe4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bff0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bff4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c000 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c004 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c010 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c014 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c020 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c024 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c030 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c034 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c040 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c044 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c050 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c054 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c060 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c064 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c070 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c074 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c080 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c084 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c090 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c094 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c0a0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c0a4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c0b4 | load | 4 | `abs 0x6c938` | `-` |
| `ldr` at 0x6c0b8 | load | 4 | `abs 0x1496bc` | `-` |
| `str` at 0x6c0f0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x6c0f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c0f8 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6c104 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c108 | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x6c118 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6c120 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c124 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6c130 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c134 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x6c144 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6c148 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6c154 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c158 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6c164 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c168 | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x6c178 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6c180 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c184 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6c190 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c194 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x6c1a4 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6c1a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6c1b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c1b8 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6c1c4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c1c8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c1d4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c1d8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c1e4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c1e8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c1f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c1f8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c204 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c208 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c214 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c218 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c224 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c228 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c234 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c238 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c244 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c248 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c254 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c258 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c264 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c268 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c274 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c278 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c284 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c288 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c294 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c298 | load | 4 | `[sp-68] +0x10` | `-` |
| `str` at 0x6c2a8 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6c2b0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c2b4 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6c2c0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c2c4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c2d0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c2d4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c2e0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c2e4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c2f0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c2f4 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c300 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c304 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c310 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c314 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c320 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c324 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c330 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c334 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c340 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c344 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c350 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c354 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c360 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c364 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c370 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c374 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c380 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c384 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c390 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c394 | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x6c3a4 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6c3a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6c40c | load | 4 | `abs 0x6c938` | `-` |
| `str` at 0x6c41c | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x6c420 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6c424 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x6c42c | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x6c430 | load | 4 | `abs 0x6c940` | `-` |
| `str` at 0x6c448 | store | 4 | `sp -0x1c` | `ret(0xb785c)` |
| `ldr` at 0x6c44c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c45c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c460 | load | 4 | `abs 0x6c944` | `-` |
| `ldr` at 0x6c464 | load | 4 | `abs 0x149b54` | `-` |
| `str` at 0x6c468 | store | 4 | `ret(0xb785c) +0x18` | `[0x149b54]` |
| `ldr` at 0x6c46c | load | 4 | `abs 0x6c948` | `-` |
| `ldr` at 0x6c470 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6c484 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c498 | load | 4 | `abs 0x6c93c` | `-` |
| `str` at 0x6c4a0 | store | 4 | `abs 0x1493d8` | `?` |
| `ldr` at 0x6c4a8 | load | 4 | `abs 0x6c93c` | `-` |
| `ldr` at 0x6c4ac | load | 4 | `abs 0x1493d8` | `-` |
| `ldr` at 0x6c4e0 | load | 4 | `abs 0x6c93c` | `-` |
| `str` at 0x6c4e8 | store | 4 | `abs 0x1493d8` | `0x0` |
| `ldr` at 0x6c4ec | load | 4 | `abs 0x6c94c` | `-` |
| `ldr` at 0x6c4fc | load | 4 | `abs 0x6c94c` | `-` |
| `ldr` at 0x6c500 | load | 4 | `abs 0x6c938` | `-` |
| `ldr` at 0x6c520 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c524 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6c530 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c534 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c540 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c544 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c550 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c554 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c560 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c564 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c570 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c574 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c580 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c584 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c590 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c594 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c5a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c5a4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c5b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c5b4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c5c0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c5c4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c5d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c5d4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c5e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c5e4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c5f0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c5f4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c600 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6c604 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6c66c | load | 4 | `abs 0x6c94c` | `-` |
| `str` at 0x6c67c | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x6c680 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6c684 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x6c68c | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x6c690 | load | 4 | `abs 0x6c940` | `-` |
| `str` at 0x6c6a8 | store | 4 | `sp -0x1c` | `ret(0xb785c)` |
| `ldr` at 0x6c6ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c6b8 | load | 4 | `abs 0x6c950` | `-` |
| `ldr` at 0x6c6c4 | load | 4 | `abs 0x6c92c` | `-` |
| `ldr` at 0x6c6c8 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6c6d8 | load | 4 | `abs 0x6c930` | `-` |
| `ldr` at 0x6c6dc | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6c6e0 | load | 4 | `abs 0x6c954` | `-` |
| `ldr` at 0x6c6e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c6ec | load | 4 | `abs 0x6c944` | `-` |
| `ldr` at 0x6c6f0 | load | 4 | `abs 0x149b54` | `-` |
| `str` at 0x6c6f4 | store | 4 | `(unknown)` | `[0x149b54]` |
| `ldr` at 0x6c6f8 | load | 4 | `abs 0x6c948` | `-` |
| `ldr` at 0x6c6fc | load | 4 | `abs 0x2b2454` | `-` |
| `ldr` at 0x6c708 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c724 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c728 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6c734 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c738 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c744 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c748 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c754 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c758 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c764 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c768 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c774 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c778 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c784 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c788 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c794 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c798 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c7a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c7a8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c7b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c7b8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c7c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c7c8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c7d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c7d8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c7e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c7e8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c7f4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c7f8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c804 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6c808 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6c870 | load | 4 | `abs 0x6c94c` | `-` |
| `str` at 0x6c880 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x6c884 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6c888 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x6c890 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x6c894 | load | 4 | `abs 0x6c940` | `-` |
| `str` at 0x6c8ac | store | 4 | `sp -0x1c` | `ret(0xb785c)` |
| `ldr` at 0x6c8b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c8bc | load | 4 | `abs 0x6c950` | `-` |
| `ldr` at 0x6c8c8 | load | 4 | `abs 0x6c92c` | `-` |
| `ldr` at 0x6c8cc | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6c8dc | load | 4 | `abs 0x6c930` | `-` |
| `ldr` at 0x6c8e0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6c8e4 | load | 4 | `abs 0x6c958` | `-` |
| `ldr` at 0x6c8ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c8f0 | load | 4 | `abs 0x6c944` | `-` |
| `ldr` at 0x6c8f4 | load | 4 | `abs 0x149b54` | `-` |
| `str` at 0x6c8f8 | store | 4 | `(unknown)` | `[0x149b54]` |
| `ldr` at 0x6c8fc | load | 4 | `abs 0x6c948` | `-` |
| `ldr` at 0x6c900 | load | 4 | `abs 0x2b2458` | `-` |
| `ldr` at 0x6c90c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6c920 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6c920 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6c920 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6bde0 | `(unknown callee) 0x77568` | `0x1f4` | `r1` | `r2` | `r3` |
| `bl` at 0x6bdf8 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[0x149400]` |
| `bl` at 0x6be18 | `(unknown callee) 0x6c95c` | `0x10fca0` | `?` | `[0x149400]` | `ret(0x1b274)` |
| `bl` at 0x6be48 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fcb0` | `0x149400` | `0x145ae8` |
| `bl` at 0x6be4c | `(unknown callee) 0x6dda8` | `?` | `?` | `?` | `?` |
| `bl` at 0x6be74 | `(unknown callee) 0x7a658` | `0x1` | `?` | `?` | `[0x149734]+312` |
| `bl` at 0x6be88 | `(unknown callee) 0x7a658` | `0x2` | `?` | `?` | `ret(0x7a658)` |
| `bl` at 0x6beac | `(unknown callee) 0x7a658` | `0x1` | `?` | `?` | `ret(0x7a658)` |
| `bl` at 0x6bfb0 | `(unknown callee) 0x7a658` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6c0c8 | `(unknown callee) 0x7a658` | `0x1` | `?` | `?` | `[0x1496bc]` |
| `bl` at 0x6c0dc | `(unknown callee) 0x7a658` | `0x2` | `?` | `?` | `ret(0x7a658)` |
| `bl` at 0x6c3c8 | `(unknown callee) 0x69908` | `sp-64` | `sp-60` | `0x3` | `sp-64` |
| `bl` at 0x6c3e0 | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6c3f8 | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6c414 | `(unknown callee) 0x722f8` | `sp-64` | `sp-60` | `0x1496ac` | `0x1` |
| `bl` at 0x6c440 | `(unknown callee) 0xb785c` | `0x1496a4` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6c494 | `(unknown callee) 0xb7164` | `?` | `[sp-28]` | `0x50` | `?` |
| `bl` at 0x6c4bc | `(unknown callee) 0x7a658` | `0x1` | `?` | `?` | `[0x1493d8]` |
| `bl` at 0x6c4d0 | `(unknown callee) 0x7a658` | `0x2` | `?` | `?` | `ret(0x7a658)` |
| `bl` at 0x6c4f8 | `(unknown callee) 0x1b4cc` | `0x149404` | `0x0` | `0x230` | `?` |
| `bl` at 0x6c508 | `(unknown callee) 0x1b394` | `0x149404` | `0x1496ac` | `0x70` | `?` |
| `bl` at 0x6c510 | `(unknown callee) 0x7a658` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6c628 | `(unknown callee) 0x69908` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x6c640 | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6c658 | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6c674 | `(unknown callee) 0x722f8` | `sp-64` | `sp-60` | `0x149404` | `0x5` |
| `bl` at 0x6c6a0 | `(unknown callee) 0xb785c` | `0x1496a4` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6c6bc | `(unknown callee) 0x202d0` | `0x10fcc4` | `?` | `?` | `ret(0xb785c)` |
| `bl` at 0x6c6e4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fcf0` | `?` | `0x145ae8` |
| `bl` at 0x6c70c | `(unknown callee) 0xb7164` | `[0x2b2454]+20` | `[sp-28]` | `?` | `[0x2b2454]+20` |
| `bl` at 0x6c714 | `(unknown callee) 0x7a658` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6c82c | `(unknown callee) 0x69908` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x6c844 | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6c85c | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6c878 | `(unknown callee) 0x722f8` | `sp-64` | `sp-60` | `0x149404` | `0x5` |
| `bl` at 0x6c8a4 | `(unknown callee) 0xb785c` | `0x1496a4` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6c8c0 | `(unknown callee) 0x202d0` | `0x10fcc4` | `?` | `?` | `ret(0xb785c)` |
| `bl` at 0x6c8e8 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fd08` | `?` | `0x145ae8` |
| `bl` at 0x6c910 | `(unknown callee) 0xb7164` | `[0x2b2458]+20` | `[sp-28]` | `?` | `[0x2b2458]+20` |
| `bl` at 0x6c918 | `(unknown callee) 0x77568` | `0x1f4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6bdf0 | `[0x149400] == 0x0` | `[0x149400] != 0x0` |
| `bge` at 0x6be0c | `ret(0x1b274) >= [0x149400] (signed, 32-bit)` | `ret(0x1b274) < [0x149400] (signed, 32-bit)` |
| `beq` at 0x6be38 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6be80 | `ret(0x7a658) != 0x0` | `ret(0x7a658) == 0x0` |
| `bne` at 0x6be94 | `ret(0x7a658) != 0x0` | `ret(0x7a658) == 0x0` |
| `beq` at 0x6beb8 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x6bec8 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6bed8 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x6bee8 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x6bef8 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x6bf08 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x6bf18 | `[[sp-68]+16] == 0x48` | `[[sp-68]+16] != 0x48` |
| `beq` at 0x6bf28 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x6bf38 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x6bf48 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x6bf58 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x6bf68 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x6bf78 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x6bf88 | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x6bf98 | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `bne` at 0x6bfa8 | `[[sp-68]+16] != 0x4d` | `[[sp-68]+16] == 0x4d` |
| `beq` at 0x6bfbc | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x6bfcc | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6bfdc | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x6bfec | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x6bffc | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x6c00c | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x6c01c | `[[sp-72]+16] == 0x48` | `[[sp-72]+16] != 0x48` |
| `beq` at 0x6c02c | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x6c03c | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x6c04c | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x6c05c | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x6c06c | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x6c07c | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x6c08c | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x6c09c | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `bne` at 0x6c0ac | `[[sp-72]+16] != 0x4d` | `[[sp-72]+16] == 0x4d` |
| `bne` at 0x6c0c0 | `[0x1496bc] != 0x42` | `[0x1496bc] == 0x42` |
| `beq` at 0x6c0d4 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x6c0e8 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x6c100 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `bne` at 0x6c110 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6c12c | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `bne` at 0x6c13c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6c150 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6c160 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6c170 | `[[sp-68]+120] == 0x0` | `[[sp-68]+120] != 0x0` |
| `beq` at 0x6c18c | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6c19c | `[[sp-72]+120] == 0x0` | `[[sp-72]+120] != 0x0` |
| `bne` at 0x6c1b0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6c1c0 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6c1d0 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x6c1e0 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x6c1f0 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x6c200 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x6c210 | `[[sp-68]+16] == 0x48` | `[[sp-68]+16] != 0x48` |
| `beq` at 0x6c220 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x6c230 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x6c240 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x6c250 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x6c260 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x6c270 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x6c280 | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x6c290 | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `beq` at 0x6c2a0 | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `beq` at 0x6c2bc | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6c2cc | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x6c2dc | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x6c2ec | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x6c2fc | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x6c30c | `[[sp-72]+16] == 0x48` | `[[sp-72]+16] != 0x48` |
| `beq` at 0x6c31c | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x6c32c | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x6c33c | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x6c34c | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x6c35c | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x6c36c | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x6c37c | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x6c38c | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `beq` at 0x6c39c | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `beq` at 0x6c3b0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x6c454 | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |
| `blt` at 0x6c4b4 | `[0x1493d8] < 0x0 (signed, 32-bit)` | `[0x1493d8] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x6c4c8 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `bne` at 0x6c4dc | `ret(0x7a658) != 0x0` | `ret(0x7a658) == 0x0` |
| `beq` at 0x6c51c | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x6c52c | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6c53c | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x6c54c | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x6c55c | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x6c56c | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x6c57c | `[[sp-68]+16] == 0x48` | `[[sp-68]+16] != 0x48` |
| `beq` at 0x6c58c | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x6c59c | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x6c5ac | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x6c5bc | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x6c5cc | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x6c5dc | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x6c5ec | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x6c5fc | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `beq` at 0x6c60c | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `bne` at 0x6c6b4 | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |
| `beq` at 0x6c6d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6c720 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x6c730 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6c740 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x6c750 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x6c760 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x6c770 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x6c780 | `[[sp-72]+16] == 0x48` | `[[sp-72]+16] != 0x48` |
| `beq` at 0x6c790 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x6c7a0 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x6c7b0 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x6c7c0 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x6c7d0 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x6c7e0 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x6c7f0 | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x6c800 | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `beq` at 0x6c810 | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `bne` at 0x6c8b8 | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |
| `beq` at 0x6c8d8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6c920 | `r0` | `ret(0xb785c)` |

## `L1CfgDiscoveryResponse`

- `L1CfgDiscoveryResponse`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ca7c to 0x6cfc0.
  Established. The unnamed `0xb823c` calls carry `[sp-24]`, `0x4`, unknown, unknown, and `[sp-24]`,
  `0x5`, unknown, unknown (`bl` at 0x6cafc, `bl` at 0x6cd0c). A four-byte assignment at
  `[sp-36] +0x90` retains `[sp-40]` (`str` at 0x6cb0c). Another four-byte store with unknown effective target
  writes `ret(0x1b358)` (`str` at 0x6ccdc). Discovery response fields and allocation size remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ca8c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x6ca90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6ca94 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x6ca98 | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x6ca9c | load | 4 | `abs 0x6cf9c` | `-` |
| `ldr` at 0x6caa4 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6caa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cac4 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6cacc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6cad0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6cad4 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x6cae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6cae8 | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x6caec | store | 4 | `sp -0x28` | `[[sp-24]+72]` |
| `ldr` at 0x6caf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6cb00 | store | 4 | `sp -0x28` | `ret(0xb823c)` |
| `ldr` at 0x6cb04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6cb08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6cb0c | store | 4 | `[sp-36] +0x90` | `[sp-40]` |
| `ldr` at 0x6cb10 | load | 4 | `abs 0x6cfa4` | `-` |
| `ldr` at 0x6cb14 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6cb2c | load | 4 | `abs 0x6cfa8` | `-` |
| `ldr` at 0x6cb30 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cb34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cb50 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6cb58 | store | 4 | `sp -0x34` | `ret(0x1b274)` |
| `ldr` at 0x6cb5c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6cb60 | load | 4 | `abs 0x6cfac` | `-` |
| `ldr` at 0x6cb64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cb68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cb70 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cb74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cb90 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cb98 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cb9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cbb8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cbc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cbc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cbd0 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cbd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cbf0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cbf8 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cbfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cc18 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cc20 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6cc3c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x6cc40 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cc44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cc60 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cc68 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cc6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cc88 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cc90 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cc94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ccb0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6ccb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cccc | load | 4 | `(unknown)` | `-` |
| `str` at 0x6ccdc | store | 4 | `(unknown)` | `ret(0x1b358)` |
| `ldr` at 0x6cce0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6cce4 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x6ccf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6ccf8 | load | 4 | `[sp-24] +0x50` | `-` |
| `str` at 0x6ccfc | store | 4 | `sp -0x2c` | `[[sp-24]+80]` |
| `ldr` at 0x6cd04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6cd10 | store | 4 | `sp -0x2c` | `ret(0xb823c)` |
| `ldr` at 0x6cd14 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cd18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cd34 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cd3c | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cd40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cd5c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cd64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cd78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cd7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6cd88 | load | 4 | `abs 0x6cfa4` | `-` |
| `ldr` at 0x6cd8c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6cd9c | load | 4 | `abs 0x6cfb0` | `-` |
| `ldr` at 0x6cda0 | load | 4 | `abs 0x14357c` | `-` |
| `ldr` at 0x6cda4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cdb8 | load | 4 | `abs 0x6cfa8` | `-` |
| `ldr` at 0x6cdbc | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cdc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cddc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cde4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6cde8 | load | 4 | `abs 0x6cfb4` | `-` |
| `ldr` at 0x6cdec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cdf0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6cdfc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x6ce00 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6ce04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ce20 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6ce28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6ce2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ce3c | load | 4 | `abs 0x6cfa8` | `-` |
| `ldr` at 0x6ce40 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6ce44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ce60 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6ce68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6ce7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ce84 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6ce88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cea4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6ceac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cec0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cec8 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cee8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cef0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cf04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cf0c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6cf10 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x6cf14 | load | 4 | `abs 0x6cfa0` | `-` |
| `ldr` at 0x6cf18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6cf34 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6cf3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cf50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cf58 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6cf5c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x6cf60 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6cf64 | load | 4 | `abs 0x6cfb8` | `-` |
| `ldr` at 0x6cf68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cf6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cf74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6cf7c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x6cf84 | load | 4 | `abs 0x6cfa8` | `-` |
| `ldr` at 0x6cf88 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6cf8c | load | 4 | `abs 0x6cfbc` | `-` |
| `ldmdb` at 0x6cf98 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6cf98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6cf98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6cf98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6caa0 | `(unknown callee) 0x6c95c` | `0x10fd88` | `r1` | `r2` | `[r0+12]` |
| `bl` at 0x6cafc | `(unknown callee) 0xb823c` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x6cb28 | `(unknown callee) 0x1b274` | `0x0` | `?` | `[sp-36]` | `?` |
| `bl` at 0x6cb6c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fda0` | `[sp-28]` | `?` |
| `bl` at 0x6ccd4 | `(unknown callee) 0x1b358` | `?` | `?` | `?` | `?` |
| `bl` at 0x6cd0c | `(unknown callee) 0xb823c` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x6cd84 | `(unknown callee) 0x1b394` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x6cdf4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fdd8` | `[sp-28]` | `?` |
| `bl` at 0x6cf70 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fde8` | `?` | `?` |
| `bl` at 0x6cf90 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fdfc` | `[sp-32]` | `0x145ae8` |
| `bl` at 0x6cf94 | `(unknown callee) 0x6dda8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6cae0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6cb20 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x6cbcc | `? <= ? (signed, 32-bit)` | `? > ? (signed, 32-bit)` |
| `bne` at 0x6ccf0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6cd98 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6cdb4 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6ce34 | `[sp-32] < ? (signed, 32-bit)` | `[sp-32] >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6cf98 | `r0` | `ret(0x6dda8)` |

## `L1CfgDiscoveryRequest`

- `L1CfgDiscoveryRequest`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6cfc0 to 0x6d804.
  Established. Included stores with unknown effective targets write bytes `0x0` and a four-byte
  `0x3e8` (`strb` at 0x6d060, `strb` at 0x6d080, `str` at 0x6d0a0). The unnamed `0x1b394` call
  carries `[sp-24]`, `ret(0x75150)`, `0x18`, `ret(0x75150)` (`bl` at 0x6d3d8). The later unnamed
  `0xb785c` call has `0x1496a4`, `sp-28`, `0x0`, `0x0` (`bl` at 0x6d72c). Discovery request layout
  and transmission meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6cfd0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6cfd8 | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x6cfdc | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6cfe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cffc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d004 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d014 | load | 4 | `abs 0x6d7d0` | `-` |
| `ldr` at 0x6d018 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6d028 | load | 4 | `abs 0x6d7d4` | `-` |
| `ldr` at 0x6d02c | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d030 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6d034 | load | 4 | `abs 0x6d7d8` | `-` |
| `ldr` at 0x6d038 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d03c | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6d044 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d060 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d064 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d080 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d084 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d0a0 | store | 4 | `(unknown)` | `0x3e8` |
| `ldr` at 0x6d0a4 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6d0c0 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6d0c4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d0cc | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6d0d0 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d0ec | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d0f0 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d10c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d110 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d128 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d12c | load | 4 | `abs 0x1496b4` | `-` |
| `str` at 0x6d130 | store | 4 | `(unknown)` | `[0x1496b4]` |
| `ldr` at 0x6d134 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6d150 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6d154 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d15c | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6d160 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d17c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d180 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d19c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d1a0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d1b8 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d1bc | load | 4 | `abs 0x1496b8` | `-` |
| `str` at 0x6d1c0 | store | 4 | `(unknown)` | `[0x1496b8]` |
| `ldr` at 0x6d1c4 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6d1e0 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6d1e4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d1ec | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6d1f0 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d20c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d210 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d22c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d230 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d248 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6d24c | store | 4 | `(unknown)` | `[sp-20]` |
| `ldr` at 0x6d250 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6d26c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6d270 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d278 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6d27c | load | 4 | `abs 0x6d7dc` | `-` |
| `ldr` at 0x6d284 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d288 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6d294 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d298 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6d2a8 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d2ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d2c8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d2d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d2dc | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d2e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d2fc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d304 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d314 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d330 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d334 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d350 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d354 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d370 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x6d374 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6d390 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6d394 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d39c | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `str` at 0x6d3ac | store | 4 | `sp -0x18` | `ret(0x1b1b4)` |
| `ldr` at 0x6d3b0 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d3b4 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d3b8 | load | 4 | `abs 0x1496b4` | `-` |
| `ldr` at 0x6d3bc | load | 4 | `abs 0x1496b8` | `-` |
| `str` at 0x6d3c8 | store | 4 | `sp -0x7c` | `ret(0x75150)` |
| `ldr` at 0x6d3cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6d3d0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6d3dc | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d3f8 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x6d3fc | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d418 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d41c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d434 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6d438 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x6d43c | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6d458 | store | 2 | `(unknown)` | `0x18` |
| `ldr` at 0x6d45c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d464 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6d46c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d488 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d48c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d4a8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d4ac | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d4c4 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d4c8 | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x6d4cc | store | 4 | `(unknown)` | `[0x149738]` |
| `ldr` at 0x6d4d0 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6d4ec | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6d4f0 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d4f8 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6d4fc | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d500 | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x6d520 | store | 4 | `sp -0x18` | `ret(0x1b1b4)` |
| `ldr` at 0x6d524 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d528 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d52c | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6d540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6d544 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6d550 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d56c | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x6d570 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6d58c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6d590 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d5a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6d5ac | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x6d5b0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d5c8 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d5cc | load | 4 | `abs 0x149738` | `-` |
| `strh` at 0x6d5e0 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x6d5e4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d5ec | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6d5f4 | load | 4 | `abs 0x6d7e0` | `-` |
| `str` at 0x6d5f8 | store | 4 | `sp -0x1c` | `0xffff00e` |
| `ldr` at 0x6d5fc | load | 4 | `abs 0x6d7d0` | `-` |
| `ldr` at 0x6d600 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6d610 | load | 4 | `abs 0x6d7e4` | `-` |
| `ldr` at 0x6d614 | load | 4 | `abs 0x14357c` | `-` |
| `ldr` at 0x6d618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d62c | load | 4 | `abs 0x6d7d4` | `-` |
| `ldr` at 0x6d630 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6d634 | load | 4 | `abs 0x6d7e8` | `-` |
| `ldr` at 0x6d638 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6d644 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6d648 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d64c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6d650 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6d660 | load | 4 | `abs 0x6d7d4` | `-` |
| `ldr` at 0x6d664 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d668 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6d67c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6d684 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d688 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6d69c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6d6a4 | load | 4 | `abs 0x6d7cc` | `-` |
| `ldr` at 0x6d6a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6d6bc | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6d6c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6d6c8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x6d6cc | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6d6d0 | load | 4 | `abs 0x6d7ec` | `-` |
| `ldr` at 0x6d6d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d6d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d6e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6d6e8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x6d6f0 | load | 4 | `abs 0x6d7d4` | `-` |
| `ldr` at 0x6d6f4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6d6f8 | load | 4 | `abs 0x6d7f0` | `-` |
| `str` at 0x6d708 | store | 4 | `sp -0x88` | `0x2` |
| `ldr` at 0x6d70c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6d710 | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0x6d718 | store | 4 | `sp -0x80` | `sp-116` |
| `ldr` at 0x6d71c | load | 4 | `abs 0x6d7f4` | `-` |
| `str` at 0x6d734 | store | 4 | `sp -0x20` | `ret(0xb785c)` |
| `ldr` at 0x6d738 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d744 | load | 4 | `abs 0x6d7d4` | `-` |
| `ldr` at 0x6d748 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6d74c | load | 4 | `abs 0x6d7f8` | `-` |
| `ldr` at 0x6d758 | load | 4 | `abs 0x6d7fc` | `-` |
| `ldr` at 0x6d75c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d77c | load | 4 | `abs 0x6d7fc` | `-` |
| `ldr` at 0x6d780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d794 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d7a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6d7b4 | load | 4 | `abs 0x6d800` | `-` |
| `ldr` at 0x6d7b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d7c0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x6d7c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6d7c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6d7c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6d040 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fe00` | `[sp-20]` | `[0x149738]` |
| `bl` at 0x6d280 | `(unknown callee) 0x6c95c` | `0x10fe30` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x6d3a4 | `(unknown callee) 0x1b1b4` | `0x18` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x6d3c0 | `(unknown callee) 0x75150` | `[0x1496b4]` | `[0x1496b8]` | `0x1496ac` | `0x1496ac` |
| `bl` at 0x6d3d8 | `(unknown callee) 0x1b394` | `[sp-24]` | `ret(0x75150)` | `0x18` | `ret(0x75150)` |
| `bl` at 0x6d518 | `(unknown callee) 0x1b1b4` | `?` | `sp-16` | `[0x149738]` | `?` |
| `bl` at 0x6d54c | `(unknown callee) 0x1b394` | `ret(0x1b1b4)` | `[0x149740]` | `?` | `?` |
| `bl` at 0x6d5f0 | `(unknown callee) 0x6c9f4` | `ret(0x1b394)` | `?` | `?` | `[sp-120]+1` |
| `bl` at 0x6d63c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fe48` | `[sp-20]` | `0x145ae8` |
| `bl` at 0x6d6dc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fe54` | `?` | `?` |
| `bl` at 0x6d6fc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fdfc` | `?` | `0x145ae8` |
| `bl` at 0x6d72c | `(unknown callee) 0xb785c` | `0x1496a4` | `sp-28` | `0x0` | `0x0` |
| `bl` at 0x6d750 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fe64` | `?` | `0x145ae8` |
| `bl` at 0x6d7a4 | `(unknown callee) 0xb7164` | `?` | `ret(0xb785c)` | `0x50` | `?` |
| `bl` at 0x6d7bc | `(unknown callee) 0x2079c` | `0x10fe98` | `[sp-20]` | `?` | `ret(0xb7164)` |
| `bl` at 0x6d7c4 | `(unknown callee) 0xb5fec` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6d00c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d024 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d290 | `[0x1496bc] == 0x52` | `[0x1496bc] != 0x52` |
| `beq` at 0x6d2a0 | `[0x1496bc] == 0x44` | `[0x1496bc] != 0x44` |
| `beq` at 0x6d2d8 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x6d30c | `? == 0x44` | `? != 0x44` |
| `beq` at 0x6d60c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d628 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6d658 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6d740 | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |
| `beq` at 0x6d778 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d7b0 | `ret(0xb7164) == 0x0` | `ret(0xb7164) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6d7c8 | `r0` | `?` |

## `L1CfgUpdate`

- `L1CfgUpdate`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6d804 to 0x6dacc.
  Established. Included byte stores at `sp -0x94` and `sp -0x93` and a halfword store at `sp -0x92`
  write `0x0` (`strb` at 0x6d828, `strb` at 0x6d830, `strh` at 0x6d840). The unnamed `0x722f8` call
  has `sp-152`, `sp-148`, `0x1496ac`, `0x1` (`bl` at 0x6d868). A later unnamed `0xb764c` call
  carries `0x1496a4`, `0x3e9`, `[sp-152]`, `sp-148` (`bl` at 0x6daa4). Update-record encoding and
  transmission effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6d818 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x6d81c | load | 4 | `abs 0x6daac` | `-` |
| `strb` at 0x6d828 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x6d830 | store | 1 | `sp -0x93` | `0x0` |
| `str` at 0x6d838 | store | 4 | `sp -0x90` | `0x1` |
| `strh` at 0x6d840 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x6d844 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6d84c | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x6d860 | load | 4 | `abs 0x6dab0` | `-` |
| `str` at 0x6d870 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x6d874 | load | 4 | `abs 0x6dab0` | `-` |
| `ldr` at 0x6d878 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d87c | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x6d88c | load | 4 | `abs 0x6dab0` | `-` |
| `ldr` at 0x6d890 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d8ac | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d8b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d8c0 | load | 4 | `abs 0x6dab0` | `-` |
| `ldr` at 0x6d8c4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d8e0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d8e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6d8f8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6d8fc | load | 4 | `abs 0x6dab0` | `-` |
| `ldr` at 0x6d900 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d91c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d924 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6d928 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d938 | load | 4 | `abs 0x6dab0` | `-` |
| `ldr` at 0x6d93c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d958 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d960 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6d974 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d97c | load | 4 | `abs 0x6dab0` | `-` |
| `ldr` at 0x6d980 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d99c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6d9a4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6d9b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d9c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d9c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6d9cc | store | 4 | `sp -0xa4` | `ret(0x74f94)` |
| `ldr` at 0x6d9d0 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x6d9ec | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x6d9f8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6da00 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x6da08 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x6da10 | store | 4 | `sp -0x9c` | `[sp-156]+1` |
| `ldr` at 0x6da18 | load | 4 | `abs 0x6dab4` | `-` |
| `str` at 0x6da24 | store | 4 | `abs 0x1493f4` | `ret(0x1b274)` |
| `strb` at 0x6da30 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x6da38 | store | 1 | `sp -0x93` | `0x0` |
| `ldr` at 0x6da3c | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6da44 | store | 4 | `sp -0x90` | `[sp-152]-1` |
| `strh` at 0x6da4c | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x6da50 | load | 4 | `abs 0x6dab8` | `-` |
| `ldr` at 0x6da54 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6da70 | load | 4 | `abs 0x6dabc` | `-` |
| `ldr` at 0x6da74 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6da78 | load | 4 | `abs 0x6dac0` | `-` |
| `ldr` at 0x6da7c | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6da90 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0x6da94 | load | 4 | `abs 0x6dac4` | `-` |
| `ldr` at 0x6da98 | load | 4 | `abs 0x6dac8` | `-` |
| `ldr` at 0x6da9c | load | 4 | `sp -0x98` | `-` |
| `ldmdb` at 0x6daa8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6daa8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6daa8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6daa8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6d820 | `(unknown callee) 0x6c95c` | `0x10fec8` | `r1` | `r2` | `0x0` |
| `bl` at 0x6d868 | `(unknown callee) 0x722f8` | `sp-152` | `sp-148` | `0x1496ac` | `0x1` |
| `bl` at 0x6d9c8 | `(unknown callee) 0x74f94` | `?` | `?` | `?` | `?` |
| `bl` at 0x6d9f4 | `(unknown callee) 0x722f8` | `sp-152` | `sp-148` | `ret(0x74f94)` | `0x1` |
| `bl` at 0x6da20 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[0x149730]` |
| `bl` at 0x6da28 | `(unknown callee) 0x6c9f4` | `ret(0x1b274)` | `?` | `?` | `?` |
| `bl` at 0x6da68 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x6da84 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fed4` | `[sp-152]` | `ret(0x1b274)` |
| `bl` at 0x6daa4 | `(unknown callee) 0xb764c` | `0x1496a4` | `0x3e9` | `[sp-152]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x6d884 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x6d8bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d8f0 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6d930 | `[sp-160] < ? (signed, 32-bit)` | `[sp-160] >= ? (signed, 32-bit)` |
| `beq` at 0x6d9d8 | `ret(0x74f94) == 0x0` | `ret(0x74f94) != 0x0` |
| `beq` at 0x6da60 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6daa8 | `r0` | `ret(0xb764c)` |

## `L1CfgOptimize`

- `L1CfgOptimize`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6dda8 to 0x6f07c.
  Established. Included four-byte absolute assignments at `0x149400` and `0x1493fc` retain
  `[0x1493fc]+5` and `ret(0x1b274)` (`str` at 0x6de68, `str` at 0x6de84). The unnamed `0x70284` call
  carries `ret(0x75150)`, `0x1496ac`, `0x1496ac`, `0x0` (`bl` at 0x6e474); its zero result takes the
  branch to `0x6e530` (`beq` at 0x6e488). A later four-byte absolute store at `0x149744` writes
  `[0x149744]-1` (`str` at 0x6ef50). Topology optimization and record ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6ddb8 | load | 4 | `abs 0x6ed84` | `-` |
| `ldr` at 0x6ddbc | load | 4 | `abs 0x149400` | `-` |
| `ldr` at 0x6ddc8 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6ddcc | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6dddc | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6dde0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6dde4 | load | 4 | `abs 0x6ed90` | `-` |
| `ldr` at 0x6dde8 | load | 4 | `abs 0x6ed94` | `-` |
| `ldr` at 0x6ddf8 | load | 4 | `abs 0x6ed98` | `-` |
| `ldr` at 0x6ddfc | load | 4 | `abs 0x1493fc` | `-` |
| `ldr` at 0x6de14 | load | 4 | `abs 0x6ed98` | `-` |
| `ldr` at 0x6de18 | load | 4 | `abs 0x1493fc` | `-` |
| `str` at 0x6de20 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6de24 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6de30 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6de34 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6de44 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6de48 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6de4c | load | 4 | `abs 0x6ed9c` | `-` |
| `ldr` at 0x6de50 | load | 4 | `abs 0x6ed94` | `-` |
| `ldr` at 0x6de58 | load | 4 | `abs 0x6ed84` | `-` |
| `ldr` at 0x6de5c | load | 4 | `abs 0x6ed98` | `-` |
| `ldr` at 0x6de60 | load | 4 | `abs 0x1493fc` | `-` |
| `str` at 0x6de68 | store | 4 | `abs 0x149400` | `[0x1493fc]+5` |
| `ldr` at 0x6de80 | load | 4 | `abs 0x6ed98` | `-` |
| `str` at 0x6de84 | store | 4 | `abs 0x1493fc` | `ret(0x1b274)` |
| `ldr` at 0x6de88 | load | 4 | `abs 0x6eda0` | `-` |
| `ldr` at 0x6de8c | load | 4 | `abs 0x14963c` | `-` |
| `ldr` at 0x6de98 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6de9c | load | 4 | `abs 0x6eda0` | `-` |
| `ldr` at 0x6dea0 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6dea4 | load | 4 | `abs 0x14963c` | `-` |
| `ldr` at 0x6deb4 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6deb8 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6dec4 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6dec8 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6ded0 | load | 4 | `abs 0x6eda0` | `-` |
| `ldr` at 0x6ded4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ded8 | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x6def4 | store | 4 | `abs 0x14963c` | `?` |
| `ldr` at 0x6def8 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6defc | load | 4 | `abs 0x6eda0` | `-` |
| `ldr` at 0x6df00 | load | 4 | `abs 0x14963c` | `-` |
| `str` at 0x6df20 | store | 4 | `abs 0x149638` | `ret(0x1b1b4)` |
| `ldr` at 0x6df24 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6df28 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6df38 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6df3c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6df40 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6df44 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6df58 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6df5c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6df68 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6df6c | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x6df70 | store | 4 | `sp -0x7c` | `[0x149738]` |
| `ldr` at 0x6df74 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6df78 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6df88 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6df8c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6df90 | load | 4 | `abs 0x6eda8` | `-` |
| `str` at 0x6dfa0 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x6dfa4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6dfa8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6dfac | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x6dfbc | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6dfc0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6dfdc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6dfe4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6dff0 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6dff4 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e004 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e008 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e00c | load | 4 | `abs 0x6edac` | `-` |
| `ldr` at 0x6e010 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e01c | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e020 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e030 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e034 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e038 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e054 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6e05c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e060 | load | 4 | `abs 0x6edb0` | `-` |
| `ldr` at 0x6e064 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e068 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e074 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x6e078 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e07c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e098 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6e0a0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6e0a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e0b4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e0b8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e0d4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6e0dc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6e0f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e0f8 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6e0fc | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e100 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e104 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e108 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e10c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e110 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e114 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x6e118 | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x6e124 | store | 4 | `sp -0x9c` | `ret(0x70010)` |
| `ldr` at 0x6e128 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e134 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e138 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x6e140 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e144 | load | 4 | `ret(0x70010) +0x14` | `-` |
| `ldr` at 0x6e150 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e154 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6e158 | store | 4 | `ret(0x70010) +0x10` | `[sp-128]` |
| `ldr` at 0x6e15c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e160 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e164 | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x6e16c | store | 4 | `[sp-156] +0x14` | `[[sp-152]+20]+1` |
| `ldr` at 0x6e170 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e174 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e178 | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x6e17c | store | 4 | `[sp-156] +0xc` | `[[sp-152]+12]` |
| `ldr` at 0x6e180 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e184 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e194 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e198 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e19c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e1a0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e1a4 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x6e1ac | store | 4 | `sp -0xc0` | `[[sp-156]+16]+64` |
| `ldr` at 0x6e1b0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e1b4 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x6e1b8 | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x6e1bc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e1c0 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x6e1c4 | store | 4 | `sp -0xb8` | `[[sp-156]+20]` |
| `ldr` at 0x6e1c8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e1cc | load | 4 | `abs 0x6edb4` | `-` |
| `ldr` at 0x6e1d0 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6e1d4 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6e1e0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e1e4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e1e8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e204 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6e20c | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x6e210 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e21c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e220 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e224 | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x6e228 | store | 4 | `sp -0xc0` | `[[sp-152]+4]` |
| `ldr` at 0x6e22c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e230 | load | 4 | `[sp-152] +0x8` | `-` |
| `str` at 0x6e234 | store | 4 | `sp -0xbc` | `[[sp-152]+8]` |
| `ldr` at 0x6e238 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e23c | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x6e240 | store | 4 | `sp -0xb8` | `[[sp-152]+12]` |
| `ldr` at 0x6e244 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e248 | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x6e250 | store | 4 | `sp -0xb4` | `[[sp-152]+20]+1` |
| `ldr` at 0x6e254 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6e258 | store | 4 | `sp -0xb0` | `[sp-128]` |
| `ldr` at 0x6e25c | load | 4 | `abs 0x6edb8` | `-` |
| `ldr` at 0x6e260 | load | 4 | `abs 0x6edbc` | `-` |
| `ldr` at 0x6e264 | load | 4 | `abs 0x6edc0` | `-` |
| `ldr` at 0x6e268 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x6e274 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e278 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e288 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e28c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e290 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e294 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6e29c | store | 4 | `sp -0xc0` | `[sp-128]+64` |
| `ldr` at 0x6e2a0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e2a4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e2c0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6e2c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e2d0 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6e2d4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e2d8 | load | 4 | `[sp-152] +0x10` | `-` |
| `str` at 0x6e2e0 | store | 4 | `sp -0xb8` | `[[sp-152]+16]+64` |
| `ldr` at 0x6e2e4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e2e8 | load | 4 | `abs 0x6edc4` | `-` |
| `ldr` at 0x6e2ec | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x6e2f0 | load | 4 | `[sp-152] +0x4` | `-` |
| `ldr` at 0x6e2f8 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x6e300 | store | 4 | `sp -0x84` | `[sp-132]+1` |
| `ldr` at 0x6e308 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6e310 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `str` at 0x6e31c | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x6e320 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e324 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6e328 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e338 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e33c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6e350 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e358 | load | 4 | `abs 0x6edc8` | `-` |
| `ldr` at 0x6e35c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e360 | load | 4 | `abs 0x143578` | `-` |
| `ldr` at 0x6e36c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e370 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e374 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e378 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6e38c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e394 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e398 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6e39c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e3a0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6e3b4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e3bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e3c0 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6e3c4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e3c8 | load | 4 | `abs 0x6edbc` | `-` |
| `ldr` at 0x6e3cc | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x6e3d0 | load | 4 | `abs 0x6edcc` | `-` |
| `ldr` at 0x6e3d8 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x6e3e0 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x6e3e8 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e3ec | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e3f0 | load | 4 | `abs 0x1496b4` | `-` |
| `ldr` at 0x6e3f4 | load | 4 | `abs 0x1496b8` | `-` |
| `str` at 0x6e400 | store | 4 | `sp -0x90` | `ret(0x75150)` |
| `ldr` at 0x6e404 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e408 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6e410 | store | 4 | `ret(0x75150) +0xc` | `0x0` |
| `str` at 0x6e418 | store | 4 | `abs 0x149720` | `0x0` |
| `ldr` at 0x6e41c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e420 | load | 4 | `abs 0x149704` | `-` |
| `ldr` at 0x6e42c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e430 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6e438 | store | 4 | `[sp-144] +0xc` | `?` |
| `str` at 0x6e440 | store | 4 | `abs 0x149720` | `?` |
| `ldr` at 0x6e444 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e448 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e44c | load | 4 | `abs 0x1496b4` | `-` |
| `str` at 0x6e450 | store | 4 | `abs 0x149724` | `[0x1496b4]` |
| `ldr` at 0x6e454 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e458 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e45c | load | 4 | `abs 0x1496b8` | `-` |
| `str` at 0x6e460 | store | 4 | `abs 0x149728` | `[0x1496b8]` |
| `ldr` at 0x6e468 | load | 4 | `abs 0x6ee0c` | `-` |
| `str` at 0x6e470 | store | 4 | `abs 0x149704` | `0x0` |
| `str` at 0x6e47c | store | 4 | `sp -0x94` | `ret(0x70284)` |
| `ldr` at 0x6e480 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e48c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e490 | load | 4 | `ret(0x70284) +0xc` | `-` |
| `ldr` at 0x6e49c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e4a0 | load | 4 | `[sp-148] +0xc` | `-` |
| `ldr` at 0x6e4ac | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e4b0 | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x6e4bc | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e4c0 | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x6e4d0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e4d4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e4d8 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6e4dc | store | 4 | `abs 0x149704` | `[[sp-148]+16]` |
| `ldr` at 0x6e4e0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x6e4e4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e4e8 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6e4ec | store | 4 | `[sp-144] +0xc` | `[[sp-148]+16]` |
| `ldr` at 0x6e4f0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e4f4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e4f8 | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6e4fc | store | 4 | `abs 0x149720` | `[[sp-148]+16]` |
| `ldr` at 0x6e500 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e504 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e508 | load | 4 | `[sp-148] +0x0` | `-` |
| `str` at 0x6e50c | store | 4 | `abs 0x149724` | `[[sp-148]]` |
| `ldr` at 0x6e510 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e514 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e518 | load | 4 | `[sp-148] +0x4` | `-` |
| `str` at 0x6e51c | store | 4 | `abs 0x149728` | `[[sp-148]+4]` |
| `ldr` at 0x6e520 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e524 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6e528 | load | 4 | `[sp-148] +0x14` | `-` |
| `str` at 0x6e52c | store | 4 | `abs 0x14972c` | `[[sp-148]+20]` |
| `ldr` at 0x6e530 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e534 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e544 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e548 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e54c | load | 4 | `abs 0x6edd0` | `-` |
| `str` at 0x6e558 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x6e55c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e560 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6e564 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e570 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e574 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6e578 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e57c | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e590 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e594 | load | 4 | `abs 0x149638` | `-` |
| `str` at 0x6e5b4 | store | 4 | `sp -0xa4` | `0x1` |
| `ldr` at 0x6e5b8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6e5bc | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6e5c0 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e5c4 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e5d4 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e5d8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e5dc | load | 4 | `abs 0x6edd4` | `-` |
| `ldr` at 0x6e5e0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6e5e8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6e5f4 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e5f8 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e608 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e60c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e610 | load | 4 | `abs 0x6edd8` | `-` |
| `str` at 0x6e61c | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x6e620 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e624 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6e628 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e634 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6e63c | store | 4 | `sp -0x8c` | `0x1` |
| `ldr` at 0x6e644 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e64c | load | 4 | `abs 0x6eddc` | `-` |
| `ldr` at 0x6e650 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e654 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6e668 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e66c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e67c | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e684 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e688 | load | 4 | `abs 0x6ede0` | `-` |
| `str` at 0x6e694 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6e698 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e69c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e6a0 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e6b0 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6e6b4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e6b8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e6cc | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e6d4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e6d8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e6ec | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e6f4 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6e6f8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6e6fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e700 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e70c | store | 4 | `sp -0x9c` | `ret(0x70010)` |
| `ldr` at 0x6e710 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e71c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6e724 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6e72c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e730 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e744 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e74c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e750 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e764 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e76c | load | 4 | `abs 0x6ede4` | `-` |
| `ldr` at 0x6e770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e774 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e788 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e78c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e79c | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e7a4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e7a8 | load | 4 | `abs 0x6ede0` | `-` |
| `ldr` at 0x6e7b0 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e7b8 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x6e7c4 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6e7c8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e7cc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6e7dc | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e7e0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e7e4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e7f8 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6e7fc | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6e804 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e818 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6e81c | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6e824 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6e828 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e82c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e830 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e83c | store | 4 | `sp -0x9c` | `ret(0x70010)` |
| `ldr` at 0x6e840 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e84c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6e854 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6e85c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e870 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6e874 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6e87c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e890 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6e894 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6e8a0 | load | 4 | `abs 0x6ede8` | `-` |
| `ldr` at 0x6e8a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e8a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e8bc | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e8c0 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e8d0 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e8d8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e8dc | load | 4 | `abs 0x6ede0` | `-` |
| `ldr` at 0x6e8e4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e8ec | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x6e8f8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6e8fc | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e900 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e904 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6e914 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6e918 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e92c | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x6e934 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x6e938 | load | 4 | `abs 0x6eda4` | `-` |
| `ldr` at 0x6e93c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e940 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e944 | load | 4 | `abs 0x149638` | `-` |
| `ldr` at 0x6e948 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6e94c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e950 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e95c | store | 4 | `sp -0x98` | `ret(0x70010)` |
| `ldr` at 0x6e960 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e96c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e970 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e974 | load | 4 | `[sp-156] +0x14` | `-` |
| `ldr` at 0x6e978 | load | 4 | `ret(0x70010) +0x14` | `-` |
| `ldr` at 0x6e984 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6e98c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6e994 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e998 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e99c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e9a0 | load | 4 | `ret(0x70010) +0x14` | `-` |
| `str` at 0x6e9a4 | store | 4 | `sp -0xc0` | `[ret(0x70010)+20]` |
| `ldr` at 0x6e9a8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e9ac | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x6e9b0 | store | 4 | `sp -0xbc` | `[[sp-156]+20]` |
| `ldr` at 0x6e9b8 | load | 4 | `abs 0x6edec` | `-` |
| `ldr` at 0x6e9bc | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6e9c0 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6e9d4 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6e9d8 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6e9e8 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6e9f0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6e9f4 | load | 4 | `abs 0x6ede0` | `-` |
| `ldr` at 0x6e9fc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6ea08 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ea0c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6ea10 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x6ea14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ea20 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6ea28 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6ea30 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ea34 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ea38 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6ea3c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6ea40 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6ea44 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ea48 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x6ea4c | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x6ea54 | load | 4 | `abs 0x6edf0` | `-` |
| `ldr` at 0x6ea58 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6ea5c | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6ea70 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6ea74 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6ea84 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6ea8c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6ea90 | load | 4 | `abs 0x6ede0` | `-` |
| `ldr` at 0x6ea98 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6eaa4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6eaa8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6eaac | load | 4 | `[sp-156] +0xc` | `-` |
| `ldr` at 0x6eab0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6eabc | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6eac4 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6eacc | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ead0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ead4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6ead8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6eadc | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6eae0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6eae4 | load | 4 | `[sp-156] +0xc` | `-` |
| `str` at 0x6eae8 | store | 4 | `sp -0xbc` | `[[sp-156]+12]` |
| `ldr` at 0x6eaf0 | load | 4 | `abs 0x6edf4` | `-` |
| `ldr` at 0x6eaf4 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6eaf8 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6eb0c | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6eb10 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6eb20 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6eb28 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6eb2c | load | 4 | `abs 0x6ede0` | `-` |
| `ldr` at 0x6eb34 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6eb3c | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x6eb4c | load | 4 | `abs 0x6edf8` | `-` |
| `ldr` at 0x6eb50 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6eb64 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6eb68 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6eb78 | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6eb80 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6eb84 | load | 4 | `abs 0x6ede0` | `-` |
| `ldr` at 0x6eb8c | load | 4 | `abs 0x6ed8c` | `-` |
| `ldr` at 0x6eb90 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6eb94 | load | 4 | `abs 0x6edd8` | `-` |
| `str` at 0x6eba0 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x6eba4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6eba8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6ebac | load | 4 | `abs 0x149744` | `-` |
| `str` at 0x6ebc0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6ebc4 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ebc8 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ebcc | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ebd0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6ebe4 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x6ebec | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ebf0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6ec04 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x6ec0c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6ec10 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x6ec14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ec18 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6ec24 | store | 4 | `sp -0x9c` | `ret(0x70010)` |
| `ldr` at 0x6ec28 | load | 4 | `abs 0x6ed88` | `-` |
| `ldr` at 0x6ec2c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6ec3c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ec4c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6ec54 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6ec58 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6ec64 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6ec68 | store | 4 | `sp -0xa8` | `[sp-160]` |
| `ldr` at 0x6ec6c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ec70 | load | 4 | `abs 0x149704` | `-` |
| `ldr` at 0x6ec7c | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ec80 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ec84 | load | 4 | `ret(0x70010) +0x10` | `-` |
| `ldr` at 0x6eca0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6eca8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ecb4 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x6ecbc | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x6ecc0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x6ecc4 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6ecc8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6eccc | store | 4 | `sp -0xac` | `[sp-160]` |
| `ldr` at 0x6ecd0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6ece0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ece4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ece8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ed04 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6ed0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ed18 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x6ed20 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x6ed24 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x6ed28 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6ed2c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ed38 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ed3c | load | 4 | `abs 0x149704` | `-` |
| `ldr` at 0x6ed48 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ed4c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ed50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ed6c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6ed74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ee14 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6ee24 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ee28 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6ee2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ee48 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6ee50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ee5c | load | 4 | `abs 0x6ee00` | `-` |
| `ldr` at 0x6ee60 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6ee70 | load | 4 | `abs 0x6ee04` | `-` |
| `ldr` at 0x6ee74 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ee78 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6ee8c | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x6ee94 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ee98 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6eeac | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x6eeb4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6eeb8 | store | 4 | `sp -0xc0` | `[sp-160]` |
| `ldr` at 0x6eebc | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6eec0 | load | 4 | `abs 0x6edfc` | `-` |
| `ldr` at 0x6eec4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6eec8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6eed0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6eed4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6eee8 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x6eef0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6eef4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6ef08 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x6ef14 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ef18 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x6ef1c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6ef40 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ef44 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6ef48 | load | 4 | `abs 0x149744` | `-` |
| `str` at 0x6ef50 | store | 4 | `abs 0x149744` | `[0x149744]-1` |
| `ldr` at 0x6ef54 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x6ef5c | store | 4 | `sp -0x88` | `[sp-136]-1` |
| `ldr` at 0x6ef60 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x6ef68 | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x6ef74 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6ef80 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6ef8c | load | 4 | `abs 0x6ee00` | `-` |
| `ldr` at 0x6ef90 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6efa0 | load | 4 | `abs 0x6ee04` | `-` |
| `ldr` at 0x6efa4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6efa8 | load | 4 | `abs 0x6ee08` | `-` |
| `str` at 0x6efb4 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x6efb8 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6efbc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6efc0 | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x6efd0 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6efd4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6eff0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6eff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f008 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6f00c | load | 4 | `abs 0x14971c` | `-` |
| `ldr` at 0x6f018 | load | 4 | `abs 0x6ee0c` | `-` |
| `ldr` at 0x6f01c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6f038 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f040 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f04c | load | 4 | `abs 0x6ee10` | `-` |
| `ldr` at 0x6f050 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6f054 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f060 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6f068 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6f070 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldmdb` at 0x6f078 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6f078 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6f078 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6f078 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ddec | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10ff08` | `0x2b23a8` | `0x145ae8` |
| `bl` at 0x6ddf0 | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `?` |
| `bl` at 0x6de0c | `(unknown callee) 0x1b274` | `0x0` | `r1` | `r2` | `[0x1493fc]` |
| `bl` at 0x6de54 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10ff28` | `0x2b23a8` | `0x145ae8` |
| `bl` at 0x6de6c | `(unknown callee) 0x6c9f4` | `?` | `?` | `0x149400` | `[0x1493fc]+5` |
| `bl` at 0x6de78 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x6decc | `(unknown callee) 0x1b490` | `[0x149638]` | `?` | `?` | `0x149638` |
| `bl` at 0x6df18 | `(unknown callee) 0x1b1b4` | `?` | `0x14963c` | `[0x14963c]` | `?` |
| `bl` at 0x6df34 | `(unknown callee) 0x1b4cc` | `[0x149638]` | `0x0` | `0x4` | `0x149638` |
| `bl` at 0x6df64 | `(unknown callee) 0x1b394` | `[0x149638]` | `[0x149740]` | `?` | `?` |
| `bl` at 0x6df94 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10ff44` | `?` | `0x145ae8` |
| `bl` at 0x6df98 | `(unknown callee) 0x6ba6c` | `?` | `?` | `?` | `?` |
| `bl` at 0x6e014 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10ff58` | `[sp-128]` | `0x145ae8` |
| `bl` at 0x6e06c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10ff78` | `[sp-128]` | `?` |
| `bl` at 0x6e11c | `(unknown callee) 0x70010` | `[0x149740]` | `[0x149738]` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x6e1d8 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10ff98` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6e26c | `(unknown callee) 0x71708` | `0x149740` | `0x149738` | `0x14973c` | `[[sp-152]]` |
| `bl` at 0x6e2f4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10ffc8` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x6e3d4 | `(unknown callee) 0x71dd0` | `[0x149740]` | `0x149738` | `[0x14974c]` | `0x149744` |
| `bl` at 0x6e3f8 | `(unknown callee) 0x75150` | `[0x1496b4]` | `[0x1496b8]` | `0x1496ac` | `0x1496ac` |
| `bl` at 0x6e474 | `(unknown callee) 0x70284` | `ret(0x75150)` | `0x1496ac` | `0x1496ac` | `0x0` |
| `bl` at 0x6e550 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x10fff0` | `?` | `0x145ae8` |
| `bl` at 0x6e59c | `(unknown callee) 0x1b304` | `[0x149740]` | `[0x149638]` | `?` | `?` |
| `bl` at 0x6e5e4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110004` | `?` | `0x145ae8` |
| `bl` at 0x6e614 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x11001c` | `?` | `0x145ae8` |
| `bl` at 0x6e658 | `(unknown callee) 0x1b52c` | `sp-120` | `0x110040` | `[0x149738]` | `[sp-124]` |
| `bl` at 0x6e664 | `(unknown callee) 0x6dacc` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e68c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110064` | `sp-120` | `0x145ae8` |
| `bl` at 0x6e704 | `(unknown callee) 0x70010` | `[0x149638]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x6e778 | `(unknown callee) 0x1b52c` | `sp-120` | `0x11006c` | `?` | `?` |
| `bl` at 0x6e784 | `(unknown callee) 0x6dacc` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e7ac | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110064` | `sp-120` | `0x145ae8` |
| `bl` at 0x6e834 | `(unknown callee) 0x70010` | `[0x149740]` | `[0x149738]` | `?` | `?` |
| `bl` at 0x6e8ac | `(unknown callee) 0x1b52c` | `sp-120` | `0x11007c` | `?` | `?` |
| `bl` at 0x6e8b8 | `(unknown callee) 0x6dacc` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e8e0 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110064` | `sp-120` | `0x145ae8` |
| `bl` at 0x6e954 | `(unknown callee) 0x70010` | `[0x149638]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x6e9c4 | `(unknown callee) 0x1b52c` | `sp-120` | `0x110090` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6e9d0 | `(unknown callee) 0x6dacc` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e9f8 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110064` | `sp-120` | `0x145ae8` |
| `bl` at 0x6ea60 | `(unknown callee) 0x1b52c` | `sp-120` | `0x1100a8` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6ea6c | `(unknown callee) 0x6dacc` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6ea94 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110064` | `sp-120` | `0x145ae8` |
| `bl` at 0x6eafc | `(unknown callee) 0x1b52c` | `sp-120` | `0x1100c4` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6eb08 | `(unknown callee) 0x6dacc` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6eb30 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110064` | `sp-120` | `0x145ae8` |
| `bl` at 0x6eb54 | `(unknown callee) 0x1b52c` | `sp-120` | `0x1100e0` | `?` | `sp-120` |
| `bl` at 0x6eb60 | `(unknown callee) 0x6dacc` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6eb88 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110064` | `sp-120` | `0x145ae8` |
| `bl` at 0x6eb98 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x11001c` | `?` | `0x145ae8` |
| `bl` at 0x6ec1c | `(unknown callee) 0x70010` | `[0x149740]` | `[0x149738]` | `?` | `?` |
| `bl` at 0x6eecc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1100f0` | `?` | `?` |
| `bl` at 0x6ef3c | `(unknown callee) 0x1b394` | `?` | `?` | `?` | `?` |
| `bl` at 0x6ef70 | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `[0x149744]` |
| `bl` at 0x6efac | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110110` | `?` | `0x145ae8` |
| `bl` at 0x6f058 | `(unknown callee) 0x72734` | `?` | `0x1496ac` | `0x149850` | `[sp-128]` |
| `bl` at 0x6f064 | `(unknown callee) 0x6cfc0` | `[sp-128]` | `0x1496ac` | `?` | `[0x14971c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6ddc4 | `[0x149400] == 0x0` | `[0x149400] != 0x0` |
| `beq` at 0x6ddd8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6de04 | `[0x1493fc] == 0x0` | `[0x1493fc] != 0x0` |
| `bgt` at 0x6de2c | `[sp-152] > 0x4 (signed, 32-bit)` | `[sp-152] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x6de40 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6de94 | `[0x14963c] == 0x0` | `[0x14963c] != 0x0` |
| `bgt` at 0x6deac | `[0x149738] > [0x14963c] (signed, 32-bit)` | `[0x149738] <= [0x14963c] (signed, 32-bit)` |
| `beq` at 0x6dec0 | `[0x149638] == 0x0` | `[0x149638] != 0x0` |
| `beq` at 0x6df84 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6dfb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6dfec | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e000 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e02c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e0ac | `[sp-132] < ? (signed, 32-bit)` | `[sp-132] >= ? (signed, 32-bit)` |
| `beq` at 0x6e130 | `ret(0x70010) == 0x0` | `ret(0x70010) != 0x0` |
| `bhs` at 0x6e14c | `[[sp-152]+20]+1 >= [ret(0x70010)+20] (unsigned, 32-bit)` | `[[sp-152]+20]+1 < [ret(0x70010)+20] (unsigned, 32-bit)` |
| `beq` at 0x6e190 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e218 | `[[sp-152]+16] == ?` | `[[sp-152]+16] != ?` |
| `beq` at 0x6e284 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e330 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x6e368 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x6e428 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6e488 | `ret(0x70284) == 0x0` | `ret(0x70284) != 0x0` |
| `beq` at 0x6e498 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6e4a8 | `[[sp-148]+12] == 0x0` | `[[sp-148]+12] != 0x0` |
| `beq` at 0x6e4b8 | `[[sp-148]+8] == 0x52` | `[[sp-148]+8] != 0x52` |
| `beq` at 0x6e4c8 | `[[sp-148]+8] == 0x44` | `[[sp-148]+8] != 0x44` |
| `beq` at 0x6e540 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e56c | `[sp-124] != [0x149738]` | `[sp-124] == [0x149738]` |
| `bne` at 0x6e5a8 | `ret(0x1b304) != 0x0` | `ret(0x1b304) == 0x0` |
| `beq` at 0x6e5d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e5f0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6e604 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e630 | `[sp-124] == [0x149738]` | `[sp-124] != [0x149738]` |
| `beq` at 0x6e678 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e6a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6e718 | `ret(0x70010) != 0x0` | `ret(0x70010) == 0x0` |
| `beq` at 0x6e798 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e7d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6e848 | `ret(0x70010) != 0x0` | `ret(0x70010) == 0x0` |
| `beq` at 0x6e8cc | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e90c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x6e968 | `ret(0x70010) == 0x0` | `ret(0x70010) != 0x0` |
| `beq` at 0x6e980 | `[[sp-156]+20] == [ret(0x70010)+20]` | `[[sp-156]+20] != [ret(0x70010)+20]` |
| `beq` at 0x6e9e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6ea04 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6ea1c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6ea80 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6eaa0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6eab8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6eb1c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6eb74 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6ebb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x6ec38 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6ec60 | `[sp-160] != 0x0` | `[sp-160] == 0x0` |
| `bne` at 0x6ec78 | `[0x149704] != 0x0` | `[0x149704] == 0x0` |
| `bne` at 0x6ecb0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6ecdc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6ed14 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6ed34 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x6ed44 | `[0x149704] != 0x0` | `[0x149704] == 0x0` |
| `bne` at 0x6ed7c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6ee20 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6ee58 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6ee6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6ef7c | `[sp-140] == 0x0` | `[sp-140] != 0x0` |
| `beq` at 0x6ef88 | `[sp-140] == 0x2` | `[sp-140] != 0x2` |
| `beq` at 0x6ef9c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6efc8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6f000 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6f014 | `[0x14971c] == 0x0` | `[0x14971c] != 0x0` |
| `beq` at 0x6f048 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6f078 | `r0` | `?` |

## `L1CfgRingEvent`

- `L1CfgRingEvent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f07c to 0x6f27c.
  Established. The branch takes unsigned `[sp-48]-1 > 0x2` to `0x6f254` (`bhi` at 0x6f100). An
  included four-byte assignment at `[sp-20] +0x30` writes `[[sp-20]+48]+1` (`str` at 0x6f1a8). The
  later unnamed `0xb823c` call carries `[sp-20]`, `0x1`, `[sp-32]`, unknown (`bl` at 0x6f208).
  Ring-event fields and effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f08c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6f090 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6f094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f098 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6f0a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6f0ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f0b0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6f0bc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6f0c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6f0c4 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x6f0d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f0d8 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x6f0dc | store | 4 | `sp -0x30` | `[r0+40]` |
| `ldr` at 0x6f0e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6f0f0 | store | 4 | `sp -0x30` | `ret(0xb823c)` |
| `ldr` at 0x6f0f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6f104 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6f108 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x6f118 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f11c | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x6f12c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f148 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6f14c | store | 4 | `[sp-20] +0x38` | `[sp-24]` |
| `ldr` at 0x6f150 | load | 4 | `abs 0x6f264` | `-` |
| `ldr` at 0x6f154 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6f164 | load | 4 | `abs 0x6f268` | `-` |
| `ldr` at 0x6f168 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f16c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6f17c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f180 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6f18c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6f190 | load | 4 | `abs 0x6f26c` | `-` |
| `ldr` at 0x6f198 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f19c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f1a0 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x6f1a8 | store | 4 | `[sp-20] +0x30` | `[[sp-20]+48]+1` |
| `ldr` at 0x6f1ac | load | 4 | `abs 0x6f270` | `-` |
| `ldr` at 0x6f1b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f1b4 | load | 4 | `abs 0x1496b4` | `-` |
| `ldr` at 0x6f1c0 | load | 4 | `abs 0x6f270` | `-` |
| `ldr` at 0x6f1c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f1c8 | load | 4 | `abs 0x1496b8` | `-` |
| `ldr` at 0x6f1d4 | load | 4 | `abs 0x6f274` | `-` |
| `str` at 0x6f1d8 | store | 4 | `sp -0x28` | `0x1493d8` |
| `ldr` at 0x6f1dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6f1e0 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x6f1f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f1f4 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x6f1f8 | store | 4 | `sp -0x2c` | `[[sp-20]+48]` |
| `ldr` at 0x6f200 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6f20c | store | 4 | `sp -0x2c` | `ret(0xb823c)` |
| `ldr` at 0x6f210 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6f214 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6f218 | store | 4 | `[sp-40] +0x0` | `[sp-44]` |
| `ldr` at 0x6f21c | load | 4 | `abs 0x6f264` | `-` |
| `ldr` at 0x6f220 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6f230 | load | 4 | `abs 0x6f268` | `-` |
| `ldr` at 0x6f234 | load | 4 | `abs 0x6f274` | `-` |
| `ldr` at 0x6f238 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6f23c | load | 4 | `abs 0x6f278` | `-` |
| `ldr` at 0x6f240 | load | 4 | `abs 0x1493d8` | `-` |
| `str` at 0x6f24c | store | 4 | `sp -0x24` | `?` |
| `str` at 0x6f258 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6f25c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x6f260 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6f260 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6f260 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6f0ec | `(unknown callee) 0xb823c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x6f134 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `[sp-48]` | `?` |
| `bl` at 0x6f194 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110124` | `?` | `?` |
| `bl` at 0x6f208 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0x6f244 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x11014c` | `[0x1493d8]` | `0x145ae8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6f0d0 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x6f100 | `[sp-48]-1 > 0x2 (unsigned, 32-bit)` | `[sp-48]-1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x6f114 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6f124 | `[[sp-20]+48] != 0x0` | `[[sp-20]+48] == 0x0` |
| `bne` at 0x6f140 | `ret(0xb823c) != 0x0` | `ret(0xb823c) == 0x0` |
| `beq` at 0x6f160 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6f1bc | `[sp-28] != [0x1496b4]` | `[sp-28] == [0x1496b4]` |
| `bne` at 0x6f1d0 | `[sp-32] != [0x1496b8]` | `[sp-32] == [0x1496b8]` |
| `bne` at 0x6f1ec | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6f22c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6f260 | `r0` | `?` |

## `L1CfgPortConfigure`

- `L1CfgPortConfigure`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f354 to 0x6fbf0.
  Established. Two included four-byte absolute stores write `[0x1493e0]+1` at `0x1493e0` and
  `[0x1493e4]+1` at `0x1493e4` (`str` at 0x6f4d4, `str` at 0x6f4f8). The unnamed `0x72548` call has
  `sp-156`, `[sp-172]`, `0x70`, `sp-156` (`bl` at 0x6f440). A later unnamed `0x71a84` call uses
  `[sp-148]`, `[sp-144]`, `sp-156`, `0x0` (`bl` at 0x6fa20). Port configuration and unpacked field
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f364 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x6f368 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x6f36c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6f370 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x6f380 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f384 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x6f388 | store | 4 | `sp -0xa8` | `[r0+40]` |
| `ldr` at 0x6f390 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6f39c | store | 4 | `sp -0xa8` | `ret(0xb823c)` |
| `ldr` at 0x6f3a0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x6f3a4 | store | 4 | `sp -0x2c` | `[sp-168]` |
| `ldr` at 0x6f3a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6f3ac | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x6f3bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f3c0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x6f3c4 | store | 4 | `sp -0xac` | `[[sp-24]+48]` |
| `ldr` at 0x6f3cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6f3d8 | store | 4 | `sp -0xac` | `ret(0xb823c)` |
| `ldr` at 0x6f3dc | load | 4 | `sp -0xac` | `-` |
| `str` at 0x6f3e0 | store | 4 | `sp -0xa0` | `[sp-172]` |
| `ldr` at 0x6f3e4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6f3f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6f3f4 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x6f3f8 | store | 4 | `sp -0xb0` | `[[sp-24]+32]:1` |
| `ldr` at 0x6f3fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f408 | load | 4 | `abs 0x6fbbc` | `-` |
| `str` at 0x6f40c | store | 4 | `sp -0xb4` | `0x110190` |
| `ldr` at 0x6f414 | load | 4 | `abs 0x6fbc0` | `-` |
| `str` at 0x6f418 | store | 4 | `sp -0xb4` | `0x110198` |
| `ldr` at 0x6f41c | load | 4 | `abs 0x6fbc4` | `-` |
| `ldr` at 0x6f420 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6f424 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6f438 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6f444 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f448 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x6f44c | store | 4 | `sp -0x24` | `[[sp-24]+12]` |
| `str` at 0x6f454 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6f458 | load | 4 | `abs 0x6fbc8` | `-` |
| `ldr` at 0x6f460 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f464 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f480 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f488 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f494 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f498 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f4b4 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6f4c0 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x6f4c4 | load | 4 | `abs 0x6fbd0` | `-` |
| `ldr` at 0x6f4c8 | load | 4 | `abs 0x6fbd0` | `-` |
| `ldr` at 0x6f4cc | load | 4 | `abs 0x1493e0` | `-` |
| `str` at 0x6f4d4 | store | 4 | `abs 0x1493e0` | `[0x1493e0]+1` |
| `ldr` at 0x6f4d8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6f4e8 | load | 4 | `abs 0x6fbd4` | `-` |
| `ldr` at 0x6f4ec | load | 4 | `abs 0x6fbd4` | `-` |
| `ldr` at 0x6f4f0 | load | 4 | `abs 0x1493e4` | `-` |
| `str` at 0x6f4f8 | store | 4 | `abs 0x1493e4` | `[0x1493e4]+1` |
| `ldr` at 0x6f4fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6f500 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0x6f50c | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f510 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f52c | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6f538 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x6f53c | load | 4 | `abs 0x6fbd8` | `-` |
| `ldr` at 0x6f540 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6f544 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6f548 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6f550 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x6f554 | load | 2 | `[sp-24] +0x22` | `-` |
| `ldr` at 0x6f560 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f564 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f580 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6f58c | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x6f590 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x6f594 | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0x6f598 | store | 4 | `sp -0x28` | `[[sp-24]+34]:2` |
| `ldr` at 0x6f59c | load | 4 | `abs 0x6fbdc` | `-` |
| `ldr` at 0x6f5a0 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6f5b0 | load | 4 | `abs 0x6fbe0` | `-` |
| `str` at 0x6f5b4 | store | 4 | `sp -0xb8` | `0x145ae8` |
| `ldr` at 0x6f5b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f5c4 | load | 4 | `abs 0x6fbe4` | `-` |
| `str` at 0x6f5c8 | store | 4 | `sp -0xbc` | `0x1101d4` |
| `ldr` at 0x6f5d0 | load | 4 | `abs 0x6fbe8` | `-` |
| `str` at 0x6f5d4 | store | 4 | `sp -0xbc` | `0x1101dc` |
| `ldr` at 0x6f5d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6f5e0 | store | 4 | `sp -0xc0` | `[sp-36]+64` |
| `ldr` at 0x6f5e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f5f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6f5f8 | store | 4 | `sp -0xc4` | `[sp-40]+64` |
| `str` at 0x6f604 | store | 4 | `sp -0xc4` | `0x3f` |
| `ldr` at 0x6f608 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x6f60c | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x6f610 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x6f614 | store | 4 | `sp -0xd8` | `[sp-44]` |
| `ldr` at 0x6f618 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x6f61c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6f620 | load | 4 | `abs 0x6fbec` | `-` |
| `ldr` at 0x6f624 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x6f628 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x6f630 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f634 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f650 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f670 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f674 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f690 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f698 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x6f69c | store | 4 | `(unknown)` | `[sp-148]` |
| `ldr` at 0x6f6a0 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f6a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f6c0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f6c8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6f6cc | store | 4 | `(unknown)` | `[sp-144]` |
| `ldr` at 0x6f6d0 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f6d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f6f0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f6f8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6f6fc | store | 4 | `(unknown)` | `[sp-140]` |
| `ldr` at 0x6f700 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f704 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f720 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f728 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6f730 | store | 4 | `(unknown)` | `ret(0x75424)` |
| `ldr` at 0x6f734 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f738 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6f744 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f748 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6f754 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f758 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6f768 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6f774 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6f780 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6f790 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f79c | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f7a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f7bc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f7c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f7cc | load | 4 | `sp -0x74` | `-` |
| `str` at 0x6f7d8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x6f7dc | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f7e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f7fc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f804 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6f808 | store | 4 | `(unknown)` | `[sp-40]` |
| `ldr` at 0x6f80c | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f810 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f830 | load | 4 | `abs 0x149734` | `-` |
| `ldrb` at 0x6f83c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6f848 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f84c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f868 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f874 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6f88c | store | 4 | `sp -0xa4` | `ret(0xaf274)` |
| `ldr` at 0x6f894 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f898 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f8b4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f8c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6f8d8 | store | 4 | `sp -0xa4` | `ret(0xaf274)` |
| `ldr` at 0x6f8dc | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f8e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f8fc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f904 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f910 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f914 | load | 4 | `abs 0x14971c` | `-` |
| `ldr` at 0x6f924 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f928 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f944 | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6f950 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6f954 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f958 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f974 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f980 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f984 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f9a0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f9ac | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6f9b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f9cc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6f9d8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6f9dc | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x6f9e0 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6f9e4 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `ldr` at 0x6f9e8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x6f9ec | store | 4 | `sp -0xd4` | `[sp-68]` |
| `str` at 0x6f9f4 | store | 4 | `sp -0xd0` | `0x0` |
| `str` at 0x6f9fc | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x6fa04 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6fa10 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6fa14 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6fa28 | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x6fa2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6fa30 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x6fa34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6fa44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6fa48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6fa5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6fa6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6fa70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6fa84 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6fa88 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x6fa90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6fa94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6fa9c | store | 4 | `sp -0xc8` | `ret(0xb823c)` |
| `ldr` at 0x6faa8 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x6fab4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6fac4 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6fac8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6fae4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6faf0 | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6faf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6fb10 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6fb1c | load | 4 | `abs 0x6fbcc` | `-` |
| `ldr` at 0x6fb20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6fb3c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6fb48 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6fb4c | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x6fb50 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6fb54 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `str` at 0x6fb5c | store | 4 | `sp -0xd4` | `0x0` |
| `str` at 0x6fb64 | store | 4 | `sp -0xd0` | `0x1` |
| `str` at 0x6fb6c | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x6fb74 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6fb80 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6fb84 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x6fb94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6fb9c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x6fbb0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x6fbb8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6fbb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6fbb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6fbb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6f398 | `(unknown callee) 0xb823c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x6f3d4 | `(unknown callee) 0xb823c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x6f428 | `(unknown callee) 0x23fbc` | `0x1101a4` | `[[sp-24]+32]:1` | `[sp-180]` | `?` |
| `bl` at 0x6f440 | `(unknown callee) 0x72548` | `sp-156` | `[sp-172]` | `0x70` | `sp-156` |
| `bl` at 0x6f45c | `(unknown callee) 0x6c95c` | `0x1101c0` | `?` | `?` | `0x0` |
| `bl` at 0x6f62c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1101e4` | `[sp-188]` | `[sp-36]+64` |
| `bl` at 0x6f66c | `(unknown callee) 0x1b394` | `?` | `sp-156` | `0x70` | `?` |
| `bl` at 0x6f72c | `(unknown callee) 0x75424` | `[sp-140]` | `0x1496ac` | `?` | `[0x149734]` |
| `bl` at 0x6f884 | `(unknown callee) 0xaf274` | `?` | `[sp-36]+16384` | `?` | `[sp-36]+16384` |
| `bl` at 0x6f8d0 | `(unknown callee) 0xaf274` | `?` | `[sp-36]+8192` | `?` | `[sp-36]+8192` |
| `bl` at 0x6fa08 | `(unknown callee) 0x71708` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x6fa20 | `(unknown callee) 0x71a84` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x6fa98 | `(unknown callee) 0xb823c` | `[sp-24]` | `?` | `0x24` | `?` |
| `bl` at 0x6fab0 | `(unknown callee) 0x72548` | `sp-156` | `?` | `0x70` | `sp-156` |
| `bl` at 0x6fb78 | `(unknown callee) 0x71708` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x6fb90 | `(unknown callee) 0x71a84` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x6fba4 | `(unknown callee) 0x6dda8` | `?` | `?` | `[[sp-24]+32]:1` | `?` |
| `bl` at 0x6fbac | `(unknown callee) 0x6c9f4` | `ret(0xaf274)` | `0x1496ac` | `?` | `[0x14971c]` |
| `bl` at 0x6fbb4 | `(unknown callee) 0x6cfc0` | `[sp-36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6f37c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6f3b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6f3ec | `[sp-172] != 0x0` | `[sp-172] == 0x0` |
| `beq` at 0x6f404 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x6f490 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6f4e0 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `bne` at 0x6f508 | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |
| `beq` at 0x6f55c | `[[sp-24]+34]:2 == 0x0` | `[[sp-24]+34]:2 != 0x0` |
| `beq` at 0x6f5ac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6f5c0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x6f5ec | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x6f740 | `[0x1496bc] == 0x52` | `[0x1496bc] != 0x52` |
| `beq` at 0x6f750 | `[0x1496bc] == 0x44` | `[0x1496bc] != 0x44` |
| `beq` at 0x6f760 | `[0x1496bc] == 0x40` | `[0x1496bc] != 0x40` |
| `beq` at 0x6f770 | `[sp-140] == 0x52` | `[sp-140] != 0x52` |
| `beq` at 0x6f77c | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0x6f788 | `[sp-140] == 0x40` | `[sp-140] != 0x40` |
| `beq` at 0x6f798 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x6f844 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6f90c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6f91c | `[0x14971c] != 0x0` | `[0x14971c] == 0x0` |
| `blt` at 0x6fa3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6fa68 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6fabc | `[sp-156] != 0x0` | `[sp-156] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6fbb8 | `r0` | `?` |

## `L1CfgPortDisable`

- `L1CfgPortDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6fbf0 to 0x70010.
  Established. Included four-byte absolute stores write `[0x1493e0]-1` at `0x1493e0`, `[0x1493e4]-1`
  at `0x1493e4` and `[0x1493dc]-1` at `0x1493dc` (`str` at 0x6fca8, `str` at 0x6fcf4, `str` at
  0x6fd08). The unnamed `0xb785c` call carries `0x1496a4`, `0x0`, `0x64`, `0x2` (`bl` at 0x6ff44);
  its nonzero result takes the branch to `0x6ff60`, followed by a four-byte assignment of
  `[0x149b54]` at `ret(0xb785c) +0x18` (`bne` at 0x6ff58, `str` at 0x6ff6c). Port-disable and
  message effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6fc00 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6fc04 | load | 4 | `abs 0x6ffe4` | `-` |
| `ldr` at 0x6fc0c | load | 4 | `abs 0x6ffe8` | `-` |
| `ldr` at 0x6fc10 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x6fc20 | load | 4 | `abs 0x6ffec` | `-` |
| `ldr` at 0x6fc24 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x6fc28 | load | 4 | `abs 0x6fff0` | `-` |
| `ldr` at 0x6fc2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fc34 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fc38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fc54 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6fc5c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6fc60 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x6fc64 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fc68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fc84 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6fc8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fc98 | load | 4 | `abs 0x6fff8` | `-` |
| `ldr` at 0x6fc9c | load | 4 | `abs 0x6fff8` | `-` |
| `ldr` at 0x6fca0 | load | 4 | `abs 0x1493e0` | `-` |
| `str` at 0x6fca8 | store | 4 | `abs 0x1493e0` | `[0x1493e0]-1` |
| `ldr` at 0x6fcac | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fcb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fccc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6fcd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fce4 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fce8 | load | 4 | `abs 0x6fffc` | `-` |
| `ldr` at 0x6fcec | load | 4 | `abs 0x1493e4` | `-` |
| `str` at 0x6fcf4 | store | 4 | `abs 0x1493e4` | `[0x1493e4]-1` |
| `ldr` at 0x6fcf8 | load | 4 | `abs 0x70000` | `-` |
| `ldr` at 0x6fcfc | load | 4 | `abs 0x70000` | `-` |
| `ldr` at 0x6fd00 | load | 4 | `abs 0x1493dc` | `-` |
| `str` at 0x6fd08 | store | 4 | `abs 0x1493dc` | `[0x1493dc]-1` |
| `ldr` at 0x6fd0c | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fd10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fd2c | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6fd38 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6fd3c | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fd40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fd5c | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6fd68 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6fd6c | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fd70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fd8c | load | 4 | `abs 0x149734` | `-` |
| `str` at 0x6fd98 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6fd9c | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fda0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fdbc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x6fdd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6fdf0 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fdf4 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe00 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe04 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe10 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe14 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe20 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe24 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe30 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe34 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe40 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe44 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe50 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe54 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe60 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe64 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe70 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe74 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe80 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe84 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fe90 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fe94 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fea0 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fea4 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6feb0 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6feb4 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x6fec0 | load | 4 | `abs 0x6fff4` | `-` |
| `ldr` at 0x6fec4 | load | 4 | `abs 0x1496bc` | `-` |
| `str` at 0x6ff20 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6ff24 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6ff28 | store | 4 | `sp -0x48` | `[sp-64]` |
| `str` at 0x6ff30 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0x6ff34 | load | 4 | `abs 0x70004` | `-` |
| `str` at 0x6ff4c | store | 4 | `sp -0x1c` | `ret(0xb785c)` |
| `ldr` at 0x6ff50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ff60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ff64 | load | 4 | `abs 0x70008` | `-` |
| `ldr` at 0x6ff68 | load | 4 | `abs 0x149b54` | `-` |
| `str` at 0x6ff6c | store | 4 | `ret(0xb785c) +0x18` | `[0x149b54]` |
| `ldr` at 0x6ff70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ff90 | load | 4 | `abs 0x7000c` | `-` |
| `ldr` at 0x6ff94 | load | 4 | `abs 0x2b2458` | `-` |
| `ldr` at 0x6ffa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ffa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ffc8 | load | 4 | `abs 0x7000c` | `-` |
| `ldr` at 0x6ffcc | load | 4 | `abs 0x2b2454` | `-` |
| `ldr` at 0x6ffd8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6ffe0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ffe0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ffe0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6fc08 | `(unknown callee) 0x6c95c` | `0x110210` | `r1` | `r2` | `r3` |
| `bl` at 0x6fc30 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110224` | `[sp-20]` | `0x145ae8` |
| `bl` at 0x6fdd4 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `0x70` | `?` |
| `bl` at 0x6fde4 | `(unknown callee) 0x6dda8` | `ret(0x1b4cc)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x6fdec | `(unknown callee) 0x6c9f4` | `ret(0x1b4cc)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x6fee8 | `(unknown callee) 0x69908` | `sp-64` | `sp-60` | `0x1` | `sp-64` |
| `bl` at 0x6ff00 | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6ff18 | `(unknown callee) 0x69964` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6ff44 | `(unknown callee) 0xb785c` | `0x1496a4` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6ff80 | `(unknown callee) 0x7a658` | `0x2` | `?` | `ret(0xb785c)` | `[sp-20]` |
| `bl` at 0x6ffa4 | `(unknown callee) 0xb7164` | `[0x2b2458]+20` | `[sp-28]` | `?` | `[0x2b2458]+20` |
| `bl` at 0x6ffb8 | `(unknown callee) 0x7a658` | `0x1` | `?` | `?` | `[sp-20]` |
| `bl` at 0x6ffdc | `(unknown callee) 0xb7164` | `[0x2b2454]+20` | `[sp-28]` | `?` | `[0x2b2454]+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6fc1c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fc94 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6fcdc | `? == 0x44` | `? != 0x44` |
| `beq` at 0x6fde0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x6fdfc | `[0x1496bc] == 0x35` | `[0x1496bc] != 0x35` |
| `beq` at 0x6fe0c | `[0x1496bc] == 0x4f` | `[0x1496bc] != 0x4f` |
| `beq` at 0x6fe1c | `[0x1496bc] == 0x42` | `[0x1496bc] != 0x42` |
| `beq` at 0x6fe2c | `[0x1496bc] == 0x46` | `[0x1496bc] != 0x46` |
| `beq` at 0x6fe3c | `[0x1496bc] == 0x48` | `[0x1496bc] != 0x48` |
| `beq` at 0x6fe4c | `[0x1496bc] == 0x4a` | `[0x1496bc] != 0x4a` |
| `beq` at 0x6fe5c | `[0x1496bc] == 0x4b` | `[0x1496bc] != 0x4b` |
| `beq` at 0x6fe6c | `[0x1496bc] == 0x21` | `[0x1496bc] != 0x21` |
| `beq` at 0x6fe7c | `[0x1496bc] == 0x5e` | `[0x1496bc] != 0x5e` |
| `beq` at 0x6fe8c | `[0x1496bc] == 0x2b` | `[0x1496bc] != 0x2b` |
| `beq` at 0x6fe9c | `[0x1496bc] == 0x4c` | `[0x1496bc] != 0x4c` |
| `beq` at 0x6feac | `[0x1496bc] == 0x5a` | `[0x1496bc] != 0x5a` |
| `beq` at 0x6febc | `[0x1496bc] == 0x7c` | `[0x1496bc] != 0x7c` |
| `beq` at 0x6fecc | `[0x1496bc] == 0x4d` | `[0x1496bc] != 0x4d` |
| `bne` at 0x6ff58 | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |
| `bne` at 0x6ff78 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0x6ff8c | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `bne` at 0x6ffb0 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `beq` at 0x6ffc4 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ffe0 | `r0` | `?` |

## `L1CfgFindBrick`

- `L1CfgFindBrick`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70010 to 0x7012c.
  Established. Two four-byte reads have unknown effective targets (`ldr` at 0x70088, `ldr` at
  0x700b4). Included four-byte stack stores at `sp -0x28` and `sp -0x2c` write `0x1` and `0x0`
  respectively (`str` at 0x700c8, `str` at 0x7011c). Zero `r0` takes the branch to `0x700dc` (`beq`
  at 0x70054). Brick search representation and flags-only comparison operands remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x70020 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x70024 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x70028 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x7002c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x70034 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x7003c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x70040 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7004c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70058 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7005c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7006c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x70080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70088 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7008c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70098 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x700ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x700b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x700b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x700c8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x700cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x700d4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x700dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x700e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x700fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x70104 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x70108 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x70110 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x7011c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x70120 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x70128 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x70128 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x70128 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x70048 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x70054 | `r0 == 0x0` | `r0 != 0x0` |
| `blt` at 0x70064 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x70094 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x700c0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x700e4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x70128 | `r0` | `?` |

## `L1CfgFindState`

- `L1CfgFindState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7012c to 0x70284.
  Established. Two branches take `r2 != [0x1496b4]` and `r3 != [0x1496b8]` to `0x70180` (`bne` at
  0x7015c, `bne` at 0x70170). The second fall-through includes four-byte `0x1496ac` at `sp -0x2c`
  (`str` at 0x70178). Later four-byte reads have unknown effective targets (`ldr` at 0x701d8, `ldr`
  at 0x70204). State-record layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7013c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x70140 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x70144 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x70148 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x7014c | load | 4 | `abs 0x70280` | `-` |
| `ldr` at 0x70150 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70154 | load | 4 | `abs 0x1496b4` | `-` |
| `ldr` at 0x70160 | load | 4 | `abs 0x70280` | `-` |
| `ldr` at 0x70164 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x70168 | load | 4 | `abs 0x1496b8` | `-` |
| `ldr` at 0x70174 | load | 4 | `abs 0x70280` | `-` |
| `str` at 0x70178 | store | 4 | `sp -0x2c` | `0x1496ac` |
| `str` at 0x70184 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x7018c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x70190 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7019c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x701a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x701ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x701bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x701d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x701d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x701dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x701e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x701fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70204 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70208 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x70218 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x7021c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x70224 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x7022c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x70238 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7024c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x70254 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x70258 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x70260 | store | 4 | `sp -0x30` | `[sp-48]-112` |
| `str` at 0x7026c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x70270 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x70274 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x70278 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x7027c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7027c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7027c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x7015c | `r2 != [0x1496b4]` | `r2 == [0x1496b4]` |
| `bne` at 0x70170 | `r3 != [0x1496b8]` | `r3 == [0x1496b8]` |
| `bne` at 0x70198 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x701a4 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x701b4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x701e4 | `? != [sp-28]` | `? == [sp-28]` |
| `bne` at 0x70210 | `? != [sp-32]` | `? == [sp-32]` |
| `beq` at 0x70234 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7027c | `r0` | `?` |

## `L1CfgFindL2`

- `L1CfgFindL2`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x70284 to 0x71708.
  Established. The unnamed `0x4f4d0` call carries the four entry register candidates `r0`, `r1`,
  `r2`, `r3` (`bl` at 0x70294). Included four-byte absolute reads target `0x149738` and `0x149740`
  (`ldr` at 0x702bc, `ldr` at 0x702e4). The branch takes unsigned `? >= [[sp-28]+20]` to `0x70388`
  (`bhs` at 0x7035c). A later four-byte stack assignment at `sp -0x14` retains `[sp-20]+1` (`str` at
  0x70cbc). L2 lookup ordering, comparison operands and the result interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x702a0 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x702a8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x702b0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x702b4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x702b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x702bc | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x702cc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x702d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x702e4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x702ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x702f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70304 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7031c | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x70324 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7032c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70344 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7034c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70350 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70354 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x70360 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70378 | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x70380 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70388 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7038c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x703a0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x703a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x703ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x703b0 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x703bc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x703c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x703c4 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x703e0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x703e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x703f4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x703f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x703fc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x70418 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70420 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70430 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70434 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70438 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7044c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70454 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70470 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70478 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70484 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70488 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7048c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x704a0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x704a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x704c4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x704cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x704d8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x704dc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x704e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x704f4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x704fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70518 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70520 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7052c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70530 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70548 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70550 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7056c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70574 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70580 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70584 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70588 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7059c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x705a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x705c0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x705c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x705d4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x705d8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x705dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x705f0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x705f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70614 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x7061c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70628 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7062c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70644 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7064c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70668 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7067c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70680 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70698 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x706a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x706bc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x706c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x706d0 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x706d4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x706d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x706ec | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x706f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70710 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70718 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70724 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70728 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7072c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70740 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70748 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70764 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x7076c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70778 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7077c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70794 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7079c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x707b8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x707c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x707cc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x707d0 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x707d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x707e8 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x707f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7080c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70814 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70820 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70824 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7083c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70844 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70860 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70868 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70874 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70878 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7087c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70890 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x708b4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x708bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x708c8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x708cc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x708d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x708e4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x708ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70908 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70910 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7091c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70920 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70938 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70940 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7095c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70964 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70970 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70974 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7098c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70994 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x709b0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x709b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x709c4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x709c8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x709cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x709e0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x709e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a04 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70a0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a18 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70a1c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70a20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70a34 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70a3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a58 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70a60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70a74 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70a80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70a84 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70a90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70a94 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70aa4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ab4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70ac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ac4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ad4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70ae0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ae4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70af0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70af4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b04 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b14 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b24 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b34 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b44 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b54 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b64 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b74 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b84 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70b90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70b94 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70ba0 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70ba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70bb8 | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x70bc0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70bc8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70bcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70be0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70be8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70bec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70bf0 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x70bfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70c08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70c18 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70c1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70c30 | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x70c38 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70c40 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70c58 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70c60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70c68 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x70c74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70c80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70c90 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70c94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70ca8 | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x70cb0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70cb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x70cbc | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x70cc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70cd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x70cd4 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x70ce0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x70ce4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70ce8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70cec | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x70cfc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70d14 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70d1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70d28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70d34 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70d38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70d4c | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x70d54 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70d5c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70d74 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70d7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70d80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70d84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70d90 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70d94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70da8 | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x70db0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70db8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70dbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70dd0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70dd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ddc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70de0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70dec | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70df0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70df4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70e10 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70e18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70e24 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70e28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70e2c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x70e48 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70e50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70e64 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70e68 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70e6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70e80 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70e88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70ea4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70eac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70eb8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70ebc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70ed4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70edc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70ef8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70f00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70f0c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70f10 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70f14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70f28 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70f30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70f4c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70f54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70f60 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70f64 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70f68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70f7c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70f84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70fa0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70fa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70fb4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70fb8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x70fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70fd0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x70fd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70ff4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x70ffc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71008 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7100c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71024 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7102c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71048 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x71050 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7105c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71060 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71078 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71080 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7109c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x710a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x710b0 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x710b4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x710b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x710cc | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x710d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x710f0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x710f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71104 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71108 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7110c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71120 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71144 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x7114c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71158 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7115c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71174 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7117c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71198 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x711a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x711ac | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x711b0 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x711b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x711c8 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x711d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x711ec | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x711f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71200 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71204 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71208 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7121c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71224 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71240 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x71248 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71254 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71258 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x7125c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71270 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71278 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71294 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x7129c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x712a8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x712ac | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x712b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x712c4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x712cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x712e8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x712f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x712fc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71300 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71318 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71320 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7133c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x71344 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71350 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71354 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7136c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71374 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71390 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x71398 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x713a4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x713a8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x713ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x713c0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x713c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x713e4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x713ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x713f8 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x713fc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71400 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71414 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7141c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71438 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x71440 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7144c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71450 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71468 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71470 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7148c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x71494 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x714a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x714a8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x714b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x714b8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x714c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x714c8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x714d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x714d8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x714e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x714e8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x714f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x714f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71508 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71518 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71524 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71528 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71534 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71538 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71548 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71554 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71558 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71568 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71574 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71578 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71584 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71588 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x71594 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71598 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x715a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x715a8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x715b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x715b8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x715c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x715c8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x715d4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x715d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x715ec | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x715f4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x715fc | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71614 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7161c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71624 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x71630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7163c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7164c | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71664 | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x7166c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x71674 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x71678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7168c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71694 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71698 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7169c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x716a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x716b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x716c4 | load | 4 | `abs 0x70e60` | `-` |
| `ldr` at 0x716c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x716dc | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x716e4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x716e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x716f0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x716f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x716fc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x71700 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x71704 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x71704 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x71704 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x70294 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x702c4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x702f4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x70300 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bhs` at 0x7035c | `? >= [[sp-28]+20] (unsigned, 32-bit)` | `? < [[sp-28]+20] (unsigned, 32-bit)` |
| `bne` at 0x703b8 | `? != [[sp-28]+20]` | `? == [[sp-28]+20]` |
| `beq` at 0x703f0 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x70428 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x70480 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x704d4 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x70528 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x7057c | `? == 0x34` | `? != 0x34` |
| `beq` at 0x705d0 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x70624 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x70678 | `? == 0x39` | `? != 0x39` |
| `beq` at 0x706cc | `? == 0x37` | `? != 0x37` |
| `beq` at 0x70720 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x70774 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x707c8 | `? == 0x51` | `? != 0x51` |
| `beq` at 0x7081c | `? == 0x40` | `? != 0x40` |
| `beq` at 0x70870 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x708c4 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x70918 | `? == 0x46` | `? != 0x46` |
| `beq` at 0x7096c | `? == 0x48` | `? != 0x48` |
| `beq` at 0x709c0 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x70a14 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x70a68 | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x70a7c | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x70a8c | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x70a9c | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x70aac | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x70abc | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x70acc | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x70adc | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x70aec | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x70afc | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x70b0c | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x70b1c | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x70b2c | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x70b3c | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x70b4c | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x70b5c | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x70b6c | `[[sp-28]+8] == 0x48` | `[[sp-28]+8] != 0x48` |
| `beq` at 0x70b7c | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x70b8c | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x70b9c | `[[sp-28]+8] == 0x7c` | `[[sp-28]+8] != 0x7c` |
| `bhs` at 0x70bf8 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x70c04 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x70c10 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `bhs` at 0x70c70 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x70c7c | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x70c88 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `beq` at 0x70ccc | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x70cf4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x70d24 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x70d30 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0x70d8c | `C == 1` | `not (C == 1)` |
| `bne` at 0x70de8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x70e20 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x70e58 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x70eb4 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x70f08 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x70f5c | `? == 0x35` | `? != 0x35` |
| `beq` at 0x70fb0 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x71004 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x71058 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x710ac | `? == 0x39` | `? != 0x39` |
| `beq` at 0x71100 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x71154 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x711a8 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x711fc | `? == 0x51` | `? != 0x51` |
| `beq` at 0x71250 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x712a4 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x712f8 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x7134c | `? == 0x46` | `? != 0x46` |
| `beq` at 0x713a0 | `? == 0x48` | `? != 0x48` |
| `beq` at 0x713f4 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x71448 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x7149c | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x714b0 | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x714c0 | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x714d0 | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x714e0 | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x714f0 | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x71500 | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x71510 | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x71520 | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x71530 | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x71540 | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x71550 | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x71560 | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x71570 | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x71580 | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x71590 | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x715a0 | `[[sp-28]+8] == 0x48` | `[[sp-28]+8] != 0x48` |
| `beq` at 0x715b0 | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x715c0 | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x715d0 | `[[sp-28]+8] == 0x7c` | `[[sp-28]+8] != 0x7c` |
| `bhs` at 0x7162c | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x71638 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x71644 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `bhs` at 0x716a4 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x716b0 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x716bc | `[sp-24] == 0x44` | `[sp-24] != 0x44` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x71704 | `r0` | `?` |

## `L1CfgAddBrick`

- `L1CfgAddBrick`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71708 to 0x71a84.
  Established. The unnamed `0x70010` call carries `[r0]`, `[r1]`, `r3`, `[sp]` (`bl` at 0x71740);
  its nonzero-result fall-through includes a four-byte assignment of `[sp+8]` at `ret(0x70010) +0xc`
  (`beq` at 0x71754, `str` at 0x71760). An included later absolute four-byte store at `0x1493cc`
  writes `[[sp-28]]` (`str` at 0x71a6c). Brick insertion and record ordering remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x71718 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7171c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x71720 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x71724 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x71728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7172c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71730 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x71734 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x71738 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7173c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x71748 | store | 4 | `sp -0x30` | `ret(0x70010)` |
| `ldr` at 0x7174c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x71758 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7175c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x71760 | store | 4 | `ret(0x70010) +0xc` | `[sp+8]` |
| `ldr` at 0x71768 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7176c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x71774 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71778 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x71784 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71788 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7178c | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x71794 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]+16` |
| `ldr` at 0x71798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7179c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x717a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x717a4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x717b8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x717c8 | store | 4 | `[sp-24] +0x0` | `ret(0x1b358)` |
| `ldr` at 0x717cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x717d0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x717d4 | store | 4 | `sp -0x2c` | `[[sp-24]]` |
| `str` at 0x717dc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x717e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x717e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x717e8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x717f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71800 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x71808 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7181c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71824 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7182c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71840 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71848 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71858 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x71860 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x71868 | load | 4 | `abs 0x71a74` | `-` |
| `ldr` at 0x7186c | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x7187c | load | 4 | `abs 0x71a78` | `-` |
| `ldr` at 0x71880 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x71884 | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0x71888 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x7188c | store | 4 | `sp -0x40` | `[sp+16]` |
| `ldr` at 0x71890 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x71894 | store | 4 | `sp -0x3c` | `[sp+12]` |
| `ldr` at 0x71898 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7189c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x718a0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x718a4 | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0x718a8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x718ac | load | 4 | `abs 0x71a7c` | `-` |
| `ldr` at 0x718b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x718b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x718bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x718c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x718d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x718d4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x718ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x718fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71900 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71904 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x71910 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71924 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71930 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71944 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7194c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71950 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x71954 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71974 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71988 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71990 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x71994 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x71998 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x719ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x719b4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x719b8 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x719bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x719d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x719d8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x719dc | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x719e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x719f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x719fc | load | 4 | `sp +0x8` | `-` |
| `str` at 0x71a00 | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0x71a04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71a18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71a20 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x71a24 | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0x71a28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71a3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71a44 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x71a48 | store | 4 | `(unknown)` | `[sp+16]` |
| `ldr` at 0x71a4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71a50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71a54 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x71a5c | store | 4 | `[sp-28] +0x0` | `[[sp-28]]+1` |
| `ldr` at 0x71a60 | load | 4 | `abs 0x71a80` | `-` |
| `ldr` at 0x71a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71a68 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x71a6c | store | 4 | `abs 0x1493cc` | `[[sp-28]]` |
| `ldmdb` at 0x71a70 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x71a70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x71a70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x71a70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x71740 | `(unknown callee) 0x70010` | `[r0]` | `[r1]` | `r3` | `[sp]` |
| `bl` at 0x717c0 | `(unknown callee) 0x1b358` | `[[sp-24]]` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0x718b8 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x11023c` | `[[sp-24]]` | `[sp-36]` |
| `bl` at 0x718f4 | `(unknown callee) 0x1b1fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x71970 | `(unknown callee) 0x1b1fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x71754 | `ret(0x70010) == 0x0` | `ret(0x70010) != 0x0` |
| `ble` at 0x71780 | `[[sp-28]]+1 <= [[sp-32]] (signed, 32-bit)` | `[[sp-28]]+1 > [[sp-32]] (signed, 32-bit)` |
| `blt` at 0x717f0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blo` at 0x71854 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |
| `beq` at 0x71878 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x718c4 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x7190c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x71a70 | `r0` | `?` |

## `L1CfgAddState`

- `L1CfgAddState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71a84 to 0x71dd0.
  Established. Included four-byte absolute assignments write `[0x149748]+16` at `0x149748` and
  `ret(0x1b358)` at `0x14974c` (`str` at 0x71bac, `str` at 0x71be0). A later four-byte absolute
  store at `0x149744` writes `[0x149744]+1` (`str` at 0x71db0). The two unnamed `0x1b394` calls each
  have unknown, `[sp-32]`, `0x70`, unknown (`bl` at 0x71b78, `bl` at 0x71d9c). State insertion and
  record size remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x71a94 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x71a98 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x71a9c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x71aa0 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x71aa8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x71aac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71ab8 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71abc | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71ac0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x71ac4 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x71ac8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71acc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71ae0 | load | 4 | `abs 0x71dbc` | `-` |
| `ldr` at 0x71ae4 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x71af4 | load | 4 | `abs 0x71dc0` | `-` |
| `ldr` at 0x71af8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x71afc | load | 4 | `abs 0x71dc4` | `-` |
| `ldr` at 0x71b00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71b04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71b10 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71b14 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71b18 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71b1c | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x71b20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71b24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x71b30 | store | 4 | `sp -0x28` | `ret(0x7012c)` |
| `ldr` at 0x71b34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71b40 | load | 4 | `abs 0x71dbc` | `-` |
| `ldr` at 0x71b44 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x71b54 | load | 4 | `abs 0x71dc0` | `-` |
| `ldr` at 0x71b58 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x71b5c | load | 4 | `abs 0x71dc8` | `-` |
| `ldr` at 0x71b60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71b64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71b6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x71b70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71b80 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71b84 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x71b8c | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71b90 | load | 4 | `abs 0x149748` | `-` |
| `ldr` at 0x71b9c | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71ba0 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71ba4 | load | 4 | `abs 0x149748` | `-` |
| `str` at 0x71bac | store | 4 | `abs 0x149748` | `[0x149748]+16` |
| `ldr` at 0x71bb0 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71bb4 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71bb8 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71bbc | load | 4 | `abs 0x149748` | `-` |
| `ldr` at 0x71bd0 | load | 4 | `abs 0x14974c` | `-` |
| `str` at 0x71be0 | store | 4 | `abs 0x14974c` | `ret(0x1b358)` |
| `str` at 0x71be8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x71bec | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71bf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71bf4 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x71c04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71c0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71c14 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71c18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71c2c | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71c34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71c3c | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71c40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71c54 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71c5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71c6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x71c74 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x71c7c | load | 4 | `abs 0x71dbc` | `-` |
| `ldr` at 0x71c80 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x71c90 | load | 4 | `abs 0x71dc0` | `-` |
| `ldr` at 0x71c94 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x71c98 | load | 4 | `abs 0x71dcc` | `-` |
| `ldr` at 0x71c9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71ca0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71ca8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71cb4 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71cb8 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71cc0 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71cc4 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71cc8 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x71ce0 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71cf0 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71cf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71cf8 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x71d04 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71d08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71d1c | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71d28 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71d2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71d40 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71d48 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71d4c | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x71d50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71d70 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71d74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71d88 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x71d94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71da0 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71da4 | load | 4 | `abs 0x71db8` | `-` |
| `ldr` at 0x71da8 | load | 4 | `abs 0x149744` | `-` |
| `str` at 0x71db0 | store | 4 | `abs 0x149744` | `[0x149744]+1` |
| `ldmdb` at 0x71db4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x71db4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x71db4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x71db4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x71ad0 | `(unknown callee) 0x70010` | `[0x149740]` | `[0x149738]` | `r0` | `r1` |
| `bl` at 0x71b08 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110274` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x71b28 | `(unknown callee) 0x7012c` | `[0x14974c]` | `[0x149744]` | `?` | `?` |
| `bl` at 0x71b68 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110294` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x71b78 | `(unknown callee) 0x1b394` | `?` | `[sp-32]` | `0x70` | `?` |
| `bl` at 0x71bd8 | `(unknown callee) 0x1b358` | `[0x14974c]` | `?` | `[0x149748]` | `?` |
| `bl` at 0x71ca4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1102ac` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x71ce8 | `(unknown callee) 0x1b1fc` | `[0x14974c]+112` | `[0x14974c]` | `?` | `?` |
| `bl` at 0x71d6c | `(unknown callee) 0x1b1fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x71d9c | `(unknown callee) 0x1b394` | `?` | `[sp-32]` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x71ab4 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0x71adc | `ret(0x70010) != 0x0` | `ret(0x70010) == 0x0` |
| `beq` at 0x71af0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x71b3c | `ret(0x7012c) == 0x0` | `ret(0x7012c) != 0x0` |
| `beq` at 0x71b50 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x71b98 | `[0x149744]+1 <= [0x149748] (signed, 32-bit)` | `[0x149744]+1 > [0x149748] (signed, 32-bit)` |
| `blt` at 0x71bfc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x71c68 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `beq` at 0x71c8c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x71cb0 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x71d00 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x71db4 | `r0` | `?` |
