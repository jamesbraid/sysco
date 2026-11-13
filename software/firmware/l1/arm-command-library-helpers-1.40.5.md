# ARM command libraries and protocol helpers in 1.40.5

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
`smpcmd_mgn` and `smpcmd_tst` have unresolved computed transfers. Their access, call and return lists cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/command-library-helpers-facts.txt) retain all tool sections.

## `SMPLibrary`

- `SMPLibrary`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67a8c to 0x68358.
  Established. Included absolute four-byte assignments write `0x51` at `0x2b23a0` (`str` at 0x67b1c)
  and `[[sp-32]]` at `0x2b2398` (`str` at 0x67b34). A byte assignment at `0x149369` writes `0x1`
  (`strb` at 0x67c24). The unnamed `0xcade4` call carries `[0x2b1808]`, `0x149354`, an unknown third
  candidate, `0x2b1800` (`bl` at 0x67c18). Console state meanings and callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x67a9c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x67aa0 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x67aa8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x67ab0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x67ab4 | load | 4 | `abs 0x682e8` | `-` |
| `str` at 0x67ac4 | store | 4 | `sp -0x28` | `ret(0x22ac8)` |
| `ldr` at 0x67ac8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67ad4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67ae0 | load | 4 | `abs 0x682ec` | `-` |
| `str` at 0x67ae4 | store | 4 | `sp -0x90` | `0x10f740` |
| `str` at 0x67ae8 | store | 4 | `sp -0x8c` | `ret(0x21704)` |
| `ldr` at 0x67af0 | load | 4 | `abs 0x682f0` | `-` |
| `ldr` at 0x67af4 | load | 4 | `abs 0x682f4` | `-` |
| `ldr` at 0x67b00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67b0c | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67b10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67b14 | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67b1c | store | 4 | `abs 0x2b23a0` | `0x51` |
| `str` at 0x67b2c | store | 4 | `[sp-32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x67b30 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x67b34 | store | 4 | `abs 0x2b2398` | `[[sp-32]]` |
| `ldr` at 0x67b38 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67b3c | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67b44 | store | 4 | `abs 0x2b239c` | `0x0` |
| `ldr` at 0x67b48 | load | 4 | `abs 0x2b2398` | `-` |
| `str` at 0x67b50 | store | 4 | `[0x2b2398] +0x0` | `0x0` |
| `ldr` at 0x67b58 | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67b60 | store | 4 | `abs 0x2b2398` | `0x0` |
| `ldr` at 0x67b64 | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67b6c | store | 4 | `abs 0x2b239c` | `0x0` |
| `ldr` at 0x67b70 | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67b78 | store | 4 | `abs 0x2b23a0` | `0x0` |
| `ldr` at 0x67b7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67b80 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67b8c | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67b90 | load | 4 | `abs 0x2b1a8c` | `-` |
| `ldr` at 0x67b9c | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67ba0 | load | 4 | `abs 0x682fc` | `-` |
| `ldr` at 0x67bb8 | load | 4 | `abs 0x682e8` | `-` |
| `str` at 0x67bc4 | store | 4 | `sp -0x28` | `ret(0x22e50)` |
| `ldr` at 0x67bc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67bd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67be0 | load | 4 | `abs 0x68300` | `-` |
| `str` at 0x67be4 | store | 4 | `sp -0x90` | `0x10f768` |
| `str` at 0x67be8 | store | 4 | `sp -0x8c` | `ret(0x21704)` |
| `ldr` at 0x67bf0 | load | 4 | `abs 0x682f0` | `-` |
| `ldr` at 0x67bf4 | load | 4 | `abs 0x68304` | `-` |
| `str` at 0x67c04 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x67c0c | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67c10 | load | 4 | `abs 0x2b1808` | `-` |
| `ldr` at 0x67c14 | load | 4 | `abs 0x68308` | `-` |
| `ldr` at 0x67c1c | load | 4 | `abs 0x68308` | `-` |
| `strb` at 0x67c24 | store | 1 | `abs 0x149369` | `0x1` |
| `ldr` at 0x67c28 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67c2c | load | 4 | `abs 0x2b1808` | `-` |
| `ldr` at 0x67c30 | load | 4 | `abs 0x68308` | `-` |
| `ldr` at 0x67c38 | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67c40 | store | 4 | `abs 0x2b1a8c` | `0x1` |
| `ldr` at 0x67c44 | load | 4 | `abs 0x682e8` | `-` |
| `str` at 0x67c50 | store | 4 | `sp -0x28` | `ret(0x22e50)` |
| `ldr` at 0x67c54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67c60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67c6c | load | 4 | `abs 0x68300` | `-` |
| `str` at 0x67c70 | store | 4 | `sp -0x90` | `0x10f768` |
| `str` at 0x67c74 | store | 4 | `sp -0x8c` | `ret(0x21704)` |
| `ldr` at 0x67c7c | load | 4 | `abs 0x682f0` | `-` |
| `ldr` at 0x67c80 | load | 4 | `abs 0x6830c` | `-` |
| `str` at 0x67c90 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x67c98 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67c9c | load | 4 | `abs 0x2b1a90` | `-` |
| `str` at 0x67ca0 | store | 4 | `sp -0x88` | `[0x2b1a90]` |
| `ldr` at 0x67ca4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67cb0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67cbc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67ccc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67cd0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67cdc | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67ce4 | store | 4 | `abs 0x2b1a90` | `0x2` |
| `ldr` at 0x67ce8 | load | 4 | `abs 0x68310` | `-` |
| `ldr` at 0x67cf0 | load | 4 | `abs 0x68314` | `-` |
| `ldr` at 0x67cf8 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67cfc | load | 4 | `abs 0x68318` | `-` |
| `str` at 0x67d00 | store | 4 | `abs 0x2b1ae4` | `0x2b1a94` |
| `ldr` at 0x67d08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x67d14 | store | 4 | `sp -0x24` | `ret(0x68b94)` |
| `ldr` at 0x67d1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67d20 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67d2c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67d30 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67d3c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67d40 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67d4c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67d50 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67d5c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67d60 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67d6c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67d70 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67d7c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67d80 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67d8c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67d90 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67d9c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67da0 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67dac | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67db0 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67dbc | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67dc0 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67dcc | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67dd0 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67ddc | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67de0 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67dec | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67df0 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67dfc | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67e00 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67e0c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67e10 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67e1c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67e20 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67e2c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67e30 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67e3c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67e40 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67e4c | load | 4 | `abs 0x6831c` | `-` |
| `ldr` at 0x67e50 | load | 4 | `abs 0x1492e4` | `-` |
| `ldr` at 0x67e5c | load | 4 | `abs 0x68320` | `-` |
| `ldr` at 0x67e78 | load | 4 | `abs 0x68324` | `-` |
| `ldr` at 0x67e80 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67e84 | load | 4 | `abs 0x2b1a90` | `-` |
| `ldr` at 0x67e90 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67e94 | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x67ea8 | load | 4 | `abs 0x68328` | `-` |
| `ldr` at 0x67ebc | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x67ec4 | store | 4 | `abs 0x2b1a90` | `0x1` |
| `ldr` at 0x67ecc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67ed0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67edc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67ee0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67ef0 | load | 4 | `abs 0x68318` | `-` |
| `ldr` at 0x67f04 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67f08 | load | 4 | `abs 0x2b2148` | `-` |
| `ldr` at 0x67f1c | load | 4 | `abs 0x6832c` | `-` |
| `ldr` at 0x67f28 | load | 4 | `abs 0x68318` | `-` |
| `ldr` at 0x67f30 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67f34 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67f38 | load | 4 | `abs 0x2b2148` | `-` |
| `ldr` at 0x67f40 | load | 4 | `abs 0x68330` | `-` |
| `str` at 0x67f68 | store | 4 | `abs 0x2b2148` | `?` |
| `ldr` at 0x67f6c | load | 4 | `abs 0x68334` | `-` |
| `ldr` at 0x67f70 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67f74 | load | 4 | `abs 0x2b2148` | `-` |
| `str` at 0x67f78 | store | 4 | `abs 0x1492bc` | `[0x2b2148]` |
| `ldr` at 0x67f7c | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67f80 | load | 4 | `abs 0x2b214c` | `-` |
| `ldr` at 0x67f8c | load | 4 | `abs 0x68338` | `-` |
| `ldr` at 0x67f90 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67f94 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67f98 | load | 4 | `abs 0x2b214c` | `-` |
| `str` at 0x67fa0 | store | 4 | `abs 0x2b214c` | `[0x2b214c]+1` |
| `str` at 0x67fa4 | store | 4 | `abs 0x1492c0` | `[0x2b214c]+1` |
| `ldr` at 0x67fac | load | 4 | `abs 0x68338` | `-` |
| `str` at 0x67fb4 | store | 4 | `abs 0x1492c0` | `0x14` |
| `ldr` at 0x67fb8 | load | 4 | `abs 0x6833c` | `-` |
| `str` at 0x67fc8 | store | 4 | `sp -0x24` | `ret(0x68cbc)` |
| `ldr` at 0x67fcc | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67fd0 | load | 4 | `abs 0x68318` | `-` |
| `str` at 0x67fd4 | store | 4 | `abs 0x2b1ae4` | `0x2b1a94` |
| `ldr` at 0x67fd8 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67fdc | load | 4 | `abs 0x2b1ae4` | `-` |
| `strb` at 0x67fe4 | store | 1 | `[0x2b1ae4] +0x0` | `0x0` |
| `ldr` at 0x67fe8 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x67fec | load | 4 | `abs 0x2b1a90` | `-` |
| `ldr` at 0x68008 | load | 4 | `abs 0x68324` | `-` |
| `ldr` at 0x68010 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68014 | load | 4 | `abs 0x2b2150` | `-` |
| `ldr` at 0x68028 | load | 4 | `abs 0x68340` | `-` |
| `ldr` at 0x68030 | load | 4 | `abs 0x682f8` | `-` |
| `str` at 0x68038 | store | 4 | `abs 0x2b1a90` | `0x1` |
| `ldr` at 0x6804c | load | 4 | `abs 0x68314` | `-` |
| `ldr` at 0x68058 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x6805c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x68068 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x6806c | load | 4 | `abs 0x2b1ae4` | `-` |
| `ldr` at 0x68070 | load | 4 | `abs 0x68318` | `-` |
| `ldr` at 0x6807c | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68080 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68084 | load | 4 | `abs 0x2b1ae4` | `-` |
| `str` at 0x68090 | store | 4 | `abs 0x2b1ae4` | `[0x2b1ae4]-1` |
| `strb` at 0x68098 | store | 1 | `[0x2b1ae4] -0x1` | `0x0` |
| `ldr` at 0x6809c | load | 4 | `abs 0x68344` | `-` |
| `ldr` at 0x680a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x680ac | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x680b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x680bc | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x680cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x680d0 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x680e0 | store | 4 | `sp -0x7c` | `?` |
| `str` at 0x680e8 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x680f4 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x680f8 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x680fc | load | 4 | `abs 0x2b214c` | `-` |
| `str` at 0x68100 | store | 4 | `sp -0x80` | `[0x2b214c]` |
| `ldr` at 0x68104 | load | 4 | `abs 0x68338` | `-` |
| `ldr` at 0x68108 | load | 4 | `abs 0x1492c0` | `-` |
| `ldr` at 0x6810c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x68120 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68124 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68128 | load | 4 | `abs 0x2b1ae4` | `-` |
| `str` at 0x68134 | store | 4 | `abs 0x2b1ae4` | `[0x2b1ae4]-1` |
| `ldr` at 0x68138 | load | 4 | `abs 0x68348` | `-` |
| `ldr` at 0x68148 | load | 4 | `abs 0x68344` | `-` |
| `ldr` at 0x68154 | load | 4 | `abs 0x68334` | `-` |
| `ldr` at 0x68158 | load | 4 | `abs 0x68334` | `-` |
| `ldr` at 0x6815c | load | 4 | `abs 0x1492bc` | `-` |
| `ldr` at 0x68160 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6816c | load | 4 | `abs 0x68330` | `-` |
| `str` at 0x68194 | store | 4 | `abs 0x1492bc` | `?` |
| `ldr` at 0x68198 | load | 4 | `abs 0x68338` | `-` |
| `ldr` at 0x6819c | load | 4 | `abs 0x68338` | `-` |
| `ldr` at 0x681a0 | load | 4 | `abs 0x1492c0` | `-` |
| `ldr` at 0x681a4 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x681ac | store | 4 | `abs 0x1492c0` | `([0x1492c0] + [sp-124])` |
| `ldr` at 0x681b0 | load | 4 | `abs 0x68334` | `-` |
| `ldr` at 0x681b4 | load | 4 | `abs 0x1492bc` | `-` |
| `ldr` at 0x681c8 | load | 4 | `abs 0x6832c` | `-` |
| `ldr` at 0x681d0 | load | 4 | `abs 0x68318` | `-` |
| `ldr` at 0x681dc | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x681e0 | load | 4 | `abs 0x68318` | `-` |
| `ldr` at 0x681f0 | load | 4 | `abs 0x68318` | `-` |
| `str` at 0x681f8 | store | 4 | `abs 0x2b1ae4` | `ret(0x1b2bc)+0x2b1a94` |
| `ldr` at 0x681fc | load | 4 | `abs 0x68318` | `-` |
| `ldr` at 0x68208 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x6820c | load | 4 | `abs 0x2b1ae4` | `-` |
| `ldr` at 0x68210 | load | 4 | `abs 0x6834c` | `-` |
| `ldr` at 0x6821c | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68224 | load | 4 | `abs 0x2b1ae4` | `-` |
| `ldr` at 0x68228 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x68230 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x68234 | store | 1 | `[0x2b1ae4] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x6823c | store | 4 | `abs 0x2b1ae4` | `[0x2b1ae4]+1` |
| `ldr` at 0x68240 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68244 | load | 4 | `abs 0x2b1ae4` | `-` |
| `strb` at 0x6824c | store | 1 | `[0x2b1ae4] +0x0` | `0x0` |
| `ldr` at 0x68254 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x6825c | load | 4 | `abs 0x2b1ae4` | `-` |
| `ldr` at 0x68264 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x68268 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x6826c | store | 1 | `(unknown)` | `([[sp-28]]:1 & 0xff)` |
| `ldr` at 0x68270 | load | 4 | `abs 0x682f8` | `-` |
| `ldr` at 0x68274 | load | 4 | `abs 0x2b1ae4` | `-` |
| `strb` at 0x6827c | store | 1 | `[0x2b1ae4] +0x0` | `0x0` |
| `ldr` at 0x68280 | load | 4 | `abs 0x68350` | `-` |
| `ldr` at 0x68288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x68290 | load | 4 | `abs 0x682e8` | `-` |
| `str` at 0x6829c | store | 4 | `sp -0x28` | `ret(0x22e50)` |
| `ldr` at 0x682a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x682ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x682b8 | load | 4 | `abs 0x68300` | `-` |
| `str` at 0x682bc | store | 4 | `sp -0x90` | `0x10f768` |
| `str` at 0x682c0 | store | 4 | `sp -0x8c` | `ret(0x21704)` |
| `ldr` at 0x682c8 | load | 4 | `abs 0x682f0` | `-` |
| `ldr` at 0x682cc | load | 4 | `abs 0x68354` | `-` |
| `ldr` at 0x682d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x682dc | store | 4 | `sp -0x84` | `[sp-36]` |
| `ldr` at 0x682e0 | load | 4 | `sp -0x84` | `-` |
| `ldmdb` at 0x682e4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x682e4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x682e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x682e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x682e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67abc | `(unknown callee) 0x22ac8` | `0x2b1a2c` | `?` | `r2` | `0x0` |
| `bl` at 0x67ad8 | `(unknown callee) 0x21704` | `ret(0x22ac8)` | `?` | `?` | `ret(0x22ac8)` |
| `bl` at 0x67afc | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x286` | `0x0` |
| `bl` at 0x67b24 | `(unknown callee) 0x1b1b4` | `0x144` | `?` | `0x2b1800` | `0x51` |
| `bl` at 0x67ba8 | `(unknown callee) 0x68f0c` | `0x2b1800` | `0x10f764` | `0x1` | `[0x2b1a8c]` |
| `bl` at 0x67bbc | `(unknown callee) 0x22e50` | `0x2b1a2c` | `?` | `?` | `ret(0x68f0c)` |
| `bl` at 0x67bd8 | `(unknown callee) 0x21704` | `ret(0x22e50)` | `?` | `?` | `ret(0x22e50)` |
| `bl` at 0x67bfc | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x29e` | `0x0` |
| `bl` at 0x67c18 | `(unknown callee) 0xcade4` | `[0x2b1808]` | `0x149354` | `?` | `0x2b1800` |
| `bl` at 0x67c34 | `(unknown callee) 0xcae20` | `[0x2b1808]` | `0x149354` | `0x149354` | `0x2b1800` |
| `bl` at 0x67c48 | `(unknown callee) 0x22e50` | `0x2b1a2c` | `?` | `0x2b1800` | `0x1` |
| `bl` at 0x67c64 | `(unknown callee) 0x21704` | `ret(0x22e50)` | `?` | `?` | `ret(0x22e50)` |
| `bl` at 0x67c88 | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x2b3` | `0x0` |
| `bl` at 0x67cec | `(unknown callee) 0x69144` | `0x10f78c` | `?` | `0x2b1800` | `0x2` |
| `bl` at 0x67cf4 | `(unknown callee) 0x69144` | `0x2b1ae8` | `?` | `?` | `?` |
| `bl` at 0x67d0c | `(unknown callee) 0x68b94` | `[sp-28]` | `?` | `[0x2b1a90]` | `[[sp-28]]:1` |
| `bl` at 0x67e60 | `(unknown callee) 0x69144` | `0x10f7b0` | `?` | `[0x2b1a90]` | `[0x1492e4]` |
| `bl` at 0x67e68 | `(unknown callee) 0x3dce4` | `?` | `?` | `[0x2b1a90]` | `[0x1492e4]` |
| `bl` at 0x67e7c | `(unknown callee) 0x69144` | `0x10f7d4` | `?` | `?` | `ret(0x3dce4)` |
| `bl` at 0x67e98 | `(unknown callee) 0x6a080` | `[0x2b2150]` | `?` | `?` | `0x2b1800` |
| `bl` at 0x67eac | `(unknown callee) 0x1b52c` | `sp-120` | `0x10f808` | `ret(0x6a080)` | `sp-120` |
| `bl` at 0x67eb8 | `(unknown callee) 0x69144` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x67ef4 | `(unknown callee) 0x1b2bc` | `0x2b1a94` | `?` | `[0x2b1a90]` | `[[sp-28]]:1` |
| `bl` at 0x67f2c | `(unknown callee) 0x1b598` | `?` | `0x2b1a94` | `?` | `?` |
| `bl` at 0x67fbc | `(unknown callee) 0x69144` | `0x10f840` | `?` | `?` | `?` |
| `bl` at 0x67fc0 | `(unknown callee) 0x68cbc` | `ret(0x69144)` | `?` | `?` | `?` |
| `bl` at 0x67ff8 | `(unknown callee) 0x3dce4` | `ret(0x68cbc)` | `?` | `[0x2b1ae4]` | `[0x2b1a90]` |
| `bl` at 0x6800c | `(unknown callee) 0x69144` | `0x10f7d4` | `?` | `?` | `ret(0x3dce4)` |
| `bl` at 0x68018 | `(unknown callee) 0x6a080` | `[0x2b2150]` | `?` | `?` | `0x2b1800` |
| `bl` at 0x6802c | `(unknown callee) 0x1b52c` | `sp-120` | `0x10f844` | `ret(0x6a080)` | `sp-120` |
| `bl` at 0x68044 | `(unknown callee) 0x69144` | `sp-120` | `?` | `0x2b1800` | `sp-120` |
| `bl` at 0x68050 | `(unknown callee) 0x69144` | `0x2b1ae8` | `?` | `[0x2b1ae4]` | `[0x2b1a90]` |
| `bl` at 0x680a0 | `(unknown callee) 0x69144` | `0x10f880` | `0x2b1800` | `[0x2b1ae4]-1` | `0x0` |
| `bl` at 0x6814c | `(unknown callee) 0x69144` | `0x10f880` | `0x2b1800` | `[0x2b1ae4]-1` | `0x2b1a93` |
| `bl` at 0x681d8 | `(unknown callee) 0x1b598` | `0x2b1a94` | `?` | `?` | `?` |
| `bl` at 0x681e4 | `(unknown callee) 0x1b2bc` | `0x2b1a94` | `?` | `?` | `?` |
| `bl` at 0x68200 | `(unknown callee) 0x69144` | `0x2b1a94` | `?` | `ret(0x1b2bc)` | `ret(0x1b2bc)+0x2b1a94` |
| `bl` at 0x68284 | `(unknown callee) 0x69144` | `0x10f884` | `?` | `[0x2b1ae4]` | `0x0` |
| `bl` at 0x6828c | `(unknown callee) 0x69144` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x68294 | `(unknown callee) 0x22e50` | `0x2b1a2c` | `?` | `?` | `?` |
| `bl` at 0x682b0 | `(unknown callee) 0x21704` | `ret(0x22e50)` | `?` | `?` | `ret(0x22e50)` |
| `bl` at 0x682d4 | `(unknown callee) 0x21eb8` | `0x9` | `0x10f4e0` | `0x3a9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x67ad0 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0x67b08 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x67b88 | `[[sp-28]]:1 != 0x18` | `[[sp-28]]:1 == 0x18` |
| `bne` at 0x67b98 | `[0x2b1a8c] != 0x0` | `[0x2b1a8c] == 0x0` |
| `beq` at 0x67bb4 | `ret(0x68f0c) == 0x1` | `ret(0x68f0c) != 0x1` |
| `beq` at 0x67bd0 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `beq` at 0x67c5c | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `beq` at 0x67cac | `[0x2b1a90] == 0x1` | `[0x2b1a90] != 0x1` |
| `blo` at 0x67cb8 | `[0x2b1a90] < 0x1 (unsigned, 32-bit)` | `[0x2b1a90] >= 0x1 (unsigned, 32-bit)` |
| `bhi` at 0x67cc4 | `[0x2b1a90] > 0x3 (unsigned, 32-bit)` | `[0x2b1a90] <= 0x3 (unsigned, 32-bit)` |
| `bne` at 0x67cd8 | `[[sp-28]]:1 != 0x14` | `[[sp-28]]:1 == 0x14` |
| `bne` at 0x67d28 | `[[sp-28]]:1 != 0x4` | `[[sp-28]]:1 == 0x4` |
| `beq` at 0x67d38 | `[0x1492e4] == 0x43` | `[0x1492e4] != 0x43` |
| `beq` at 0x67d48 | `[0x1492e4] == 0x33` | `[0x1492e4] != 0x33` |
| `beq` at 0x67d58 | `[0x1492e4] == 0x35` | `[0x1492e4] != 0x35` |
| `beq` at 0x67d68 | `[0x1492e4] == 0x34` | `[0x1492e4] != 0x34` |
| `beq` at 0x67d78 | `[0x1492e4] == 0x41` | `[0x1492e4] != 0x41` |
| `beq` at 0x67d88 | `[0x1492e4] == 0x36` | `[0x1492e4] != 0x36` |
| `beq` at 0x67d98 | `[0x1492e4] == 0x39` | `[0x1492e4] != 0x39` |
| `beq` at 0x67da8 | `[0x1492e4] == 0x37` | `[0x1492e4] != 0x37` |
| `beq` at 0x67db8 | `[0x1492e4] == 0x38` | `[0x1492e4] != 0x38` |
| `beq` at 0x67dc8 | `[0x1492e4] == 0x30` | `[0x1492e4] != 0x30` |
| `beq` at 0x67dd8 | `[0x1492e4] == 0x51` | `[0x1492e4] != 0x51` |
| `beq` at 0x67de8 | `[0x1492e4] == 0x40` | `[0x1492e4] != 0x40` |
| `beq` at 0x67df8 | `[0x1492e4] == 0x4f` | `[0x1492e4] != 0x4f` |
| `beq` at 0x67e08 | `[0x1492e4] == 0x42` | `[0x1492e4] != 0x42` |
| `beq` at 0x67e18 | `[0x1492e4] == 0x46` | `[0x1492e4] != 0x46` |
| `beq` at 0x67e28 | `[0x1492e4] == 0x48` | `[0x1492e4] != 0x48` |
| `beq` at 0x67e38 | `[0x1492e4] == 0x4c` | `[0x1492e4] != 0x4c` |
| `beq` at 0x67e48 | `[0x1492e4] == 0x4d` | `[0x1492e4] != 0x4d` |
| `beq` at 0x67e58 | `[0x1492e4] == 0x7c` | `[0x1492e4] != 0x7c` |
| `beq` at 0x67e74 | `ret(0x3dce4) == 0x1` | `ret(0x3dce4) != 0x1` |
| `bne` at 0x67e8c | `[0x2b1a90] != 0x3` | `[0x2b1a90] == 0x3` |
| `beq` at 0x67ed8 | `[[sp-28]]:1 == 0xd` | `[[sp-28]]:1 != 0xd` |
| `beq` at 0x67ee8 | `[[sp-28]]:1 == 0xa` | `[[sp-28]]:1 != 0xa` |
| `beq` at 0x67f00 | `ret(0x1b2bc) == 0x0` | `ret(0x1b2bc) != 0x0` |
| `bgt` at 0x67f88 | `[0x2b214c] > 0x13 (signed, 32-bit)` | `[0x2b214c] <= 0x13 (signed, 32-bit)` |
| `bne` at 0x67ff4 | `[0x2b1a90] != 0x2` | `[0x2b1a90] == 0x2` |
| `beq` at 0x68004 | `ret(0x3dce4) == 0x1` | `ret(0x3dce4) != 0x1` |
| `bne` at 0x68064 | `[[sp-28]]:1 != 0x8` | `[[sp-28]]:1 == 0x8` |
| `bls` at 0x68078 | `[0x2b1ae4] <= 0x2b1a94 (unsigned, 32-bit)` | `[0x2b1ae4] > 0x2b1a94 (unsigned, 32-bit)` |
| `beq` at 0x680b4 | `[[sp-28]]:1 == 0x10` | `[[sp-28]]:1 != 0x10` |
| `beq` at 0x680c4 | `[[sp-28]]:1 == 0xe` | `[[sp-28]]:1 != 0xe` |
| `bne` at 0x680d8 | `[[sp-28]]:1 != 0x10` | `[[sp-28]]:1 == 0x10` |
| `bne` at 0x68114 | `[0x1492c0] != [sp-128]` | `[0x1492c0] == [sp-128]` |
| `bhi` at 0x68140 | `[0x2b1ae4]-1 > 0x2b1a93 (unsigned, 32-bit)` | `[0x2b1ae4]-1 <= 0x2b1a93 (unsigned, 32-bit)` |
| `bhs` at 0x68218 | `[0x2b1ae4] >= 0x2b1ae3 (unsigned, 32-bit)` | `[0x2b1ae4] < 0x2b1ae3 (unsigned, 32-bit)` |
| `beq` at 0x682a8 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x682e4 | `r0` | `?` |

## `SMPCmdLibrary`

- `SMPCmdLibrary`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x997ac to 0x9aab0.
  Established. The unnamed `0x75194` call carries `0x0`, `r1`, `r2`, `0x0` (`bl` at 0x997f0).
  Included word reads use `ret(0x75194) +0x8` (`ldr` at 0x99800) and `ret(0x75194) +0xc` (`ldr` at
  0x99814). A later halfword read uses absolute `0x2b25a6` (`ldrh` at 0x999e8). The indirect call
  has an unknown target and candidates `sp-2664`, unknown, `sp-2664`, unknown (`mov` at 0x9a9c8).
  Command effects and field encodings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x997c0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x997c4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x997c8 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x997d0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x997d8 | store | 4 | `sp -0xa6c` | `0x0` |
| `str` at 0x997e0 | store | 4 | `sp -0xa70` | `0x0` |
| `str` at 0x997e8 | store | 4 | `sp -0xa74` | `0x0` |
| `str` at 0x997f8 | store | 4 | `sp -0xa88` | `ret(0x75194)` |
| `ldr` at 0x997fc | load | 4 | `sp -0xa88` | `-` |
| `ldr` at 0x99800 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x99810 | load | 4 | `sp -0xa88` | `-` |
| `ldr` at 0x99814 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `str` at 0x99830 | store | 4 | `sp -0xa8c` | `?` |
| `ldr` at 0x99834 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x99844 | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x99848 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x9984c | store | 4 | `sp -0xa78` | `[[sp-28]]` |
| `ldr` at 0x99850 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99858 | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x99860 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9986c | store | 4 | `[sp-2680] +0x4` | `0x0` |
| `ldr` at 0x99880 | load | 4 | `abs 0x9a724` | `-` |
| `ldr` at 0x99888 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9988c | store | 4 | `sp -0xa68` | `[sp-32]` |
| `str` at 0x99894 | store | 4 | `sp -0xac8` | `0x0` |
| `str` at 0x998a0 | store | 4 | `sp -0xac4` | `sp-2664` |
| `ldr` at 0x998a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x998ac | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x998bc | store | 4 | `sp -0x2c` | `ret(0xbc0fc)` |
| `ldr` at 0x998c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x998cc | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x998d8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x998dc | store | 4 | `sp -0xa98` | `ret(0xbc0fc)` |
| `ldr` at 0x998e4 | load | 4 | `abs 0x9a778` | `-` |
| `str` at 0x998f4 | store | 4 | `sp -0x24` | `ret(0x22ac8)` |
| `ldr` at 0x998f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99904 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99908 | load | 4 | `[sp-2680] +0x0` | `-` |
| `ldr` at 0x9990c | load | 4 | `abs 0x9a728` | `-` |
| `str` at 0x99918 | store | 4 | `sp -0xa98` | `?` |
| `ldr` at 0x99920 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9992c | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x99944 | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x99948 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x9994c | store | 4 | `sp -0xa78` | `[[sp-28]]` |
| `str` at 0x99954 | store | 4 | `sp -0xa7c` | `0x0` |
| `ldr` at 0x99958 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99960 | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x99964 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x9996c | store | 4 | `sp -0x450` | `[sp-2656]-1` |
| `ldr` at 0x99970 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99980 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99984 | load | 4 | `abs 0x9a72c` | `-` |
| `ldr` at 0x99998 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x999a4 | load | 4 | `abs 0x9a730` | `-` |
| `str` at 0x999a8 | store | 4 | `sp -0xa90` | `0xffff003` |
| `ldr` at 0x999ac | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x999b0 | load | 4 | `abs 0x9a76c` | `-` |
| `ldr` at 0x999c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x999c8 | store | 4 | `sp -0xa94` | `?` |
| `ldr` at 0x999cc | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x999d0 | load | 4 | `abs 0x9a734` | `-` |
| `ldr` at 0x999e4 | load | 4 | `abs 0x9a738` | `-` |
| `ldrh` at 0x999e8 | load | 2 | `abs 0x2b25a6` | `-` |
| `str` at 0x999ec | store | 4 | `(unknown)` | `[0x2b25a6]:2` |
| `ldr` at 0x999f0 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x999f4 | load | 4 | `abs 0x9a73c` | `-` |
| `str` at 0x99a0c | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x99a28 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x99a44 | store | 4 | `sp -0xac8` | `0x1` |
| `ldr` at 0x99a4c | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x99a64 | store | 4 | `sp -0xac8` | `0x2` |
| `ldr` at 0x99a68 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x99a6c | store | 4 | `sp -0xac4` | `[sp-76]` |
| `str` at 0x99a74 | store | 4 | `sp -0xac0` | `sp-72` |
| `ldr` at 0x99a78 | load | 4 | `abs 0x9a738` | `-` |
| `str` at 0x99a90 | store | 4 | `sp -0x30` | `ret(0xb785c)` |
| `ldr` at 0x99a94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99aa4 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99ab8 | load | 4 | `abs 0x9a740` | `-` |
| `ldr` at 0x99abc | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99ad0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99af8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99b00 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99b08 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99b1c | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x99b20 | load | 4 | `abs 0x9a738` | `-` |
| `ldr` at 0x99b24 | load | 4 | `abs 0x2b25a0` | `-` |
| `ldr` at 0x99b30 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99b38 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99b40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99b48 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99b50 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99b58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99b5c | load | 4 | `abs 0x9a744` | `-` |
| `ldr` at 0x99b64 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99b6c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99b74 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99b7c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99b84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99b94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99b9c | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99bb4 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99bbc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99bc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99bc8 | load | 4 | `abs 0x9a770` | `-` |
| `ldr` at 0x99bd0 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99bd8 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99be0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99bec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99bf0 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x99bf8 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x99bfc | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99c00 | load | 4 | `abs 0x9a73c` | `-` |
| `str` at 0x99c18 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99c20 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99c24 | load | 4 | `abs 0x9a76c` | `-` |
| `ldr` at 0x99c38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99c48 | load | 4 | `abs 0x9a738` | `-` |
| `ldr` at 0x99c4c | load | 4 | `abs 0x2b25a0` | `-` |
| `str` at 0x99c68 | store | 4 | `sp -0xa94` | `0x0` |
| `str` at 0x99c74 | store | 4 | `sp -0xac8` | `0x0` |
| `str` at 0x99c80 | store | 4 | `sp -0xac4` | `sp-2664` |
| `ldr` at 0x99c88 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x99c98 | store | 4 | `sp -0x2c` | `ret(0xbc0fc)` |
| `ldr` at 0x99c9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x99ca8 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x99cb0 | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x99cb4 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99cc0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99cc8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99cdc | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x99ce0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x99cec | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99cf0 | load | 4 | `abs 0x9a76c` | `-` |
| `ldr` at 0x99d04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99d10 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99d18 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99d20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99d28 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99d30 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99d38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99d3c | load | 4 | `abs 0x9a748` | `-` |
| `ldr` at 0x99d44 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99d4c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99d54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99d58 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99d60 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99d68 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99d70 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99d7c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99d84 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99d88 | load | 4 | `abs 0x9a76c` | `-` |
| `ldr` at 0x99d9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99da8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99db0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99db8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99dc8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99dd4 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99de0 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99de4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99de8 | load | 4 | `abs 0x9a74c` | `-` |
| `ldr` at 0x99df0 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99e00 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99e08 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99e10 | store | 4 | `(unknown)` | `ret(0x1b034)` |
| `ldr` at 0x99e14 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x99e1c | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x99e24 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99e30 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99e34 | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x99e3c | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x99e44 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x99e48 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99e4c | load | 4 | `abs 0x9a73c` | `-` |
| `str` at 0x99e64 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99e6c | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99e70 | load | 4 | `abs 0x9a73c` | `-` |
| `str` at 0x99e88 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x99ea4 | load | 4 | `sp -0xa64` | `-` |
| `str` at 0x99ec0 | store | 4 | `sp -0xac8` | `0x1` |
| `ldr` at 0x99ec8 | load | 4 | `sp -0xa64` | `-` |
| `ldr` at 0x99edc | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99ef4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x99ef8 | store | 4 | `sp -0xac8` | `[sp-76]` |
| `str` at 0x99f00 | store | 4 | `sp -0xac4` | `sp-72` |
| `ldr` at 0x99f0c | load | 4 | `sp -0x450` | `-` |
| `str` at 0x99f18 | store | 4 | `sp -0xac0` | `?` |
| `str` at 0x99f20 | store | 4 | `sp -0xabc` | `0x0` |
| `ldr` at 0x99f24 | load | 4 | `abs 0x9a738` | `-` |
| `str` at 0x99f38 | store | 4 | `sp -0x24` | `ret(0xb7394)` |
| `ldr` at 0x99f3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99f4c | load | 4 | `sp -0x65c` | `-` |
| `ldr` at 0x99f58 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x99f5c | load | 4 | `abs 0x9a73c` | `-` |
| `str` at 0x99f74 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x99f78 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x99f80 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x99f88 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99f90 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x99fa4 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x99fa8 | load | 4 | `abs 0x9a738` | `-` |
| `ldr` at 0x99fac | load | 4 | `abs 0x2b25a0` | `-` |
| `ldr` at 0x99fb8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99fc0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99fc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99fd0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99fd8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99fe0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99fe4 | load | 4 | `abs 0x9a750` | `-` |
| `ldr` at 0x99fec | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x99ff4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x99ffc | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a004 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a00c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a01c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a024 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a028 | load | 4 | `abs 0x9a76c` | `-` |
| `ldr` at 0x9a03c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a054 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a05c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a064 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a068 | load | 4 | `abs 0x9a770` | `-` |
| `ldr` at 0x9a070 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a078 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a080 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a08c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a090 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a094 | load | 4 | `abs 0x9a73c` | `-` |
| `str` at 0x9a0ac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a0b0 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x9a0b8 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x9a0bc | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x9a0c4 | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x9a0c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a0d4 | load | 4 | `abs 0x9a754` | `-` |
| `ldr` at 0x9a0e0 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x9a0e8 | store | 4 | `sp -0x450` | `[sp-1104]-1` |
| `ldr` at 0x9a0f0 | load | 4 | `sp -0xa64` | `-` |
| `ldr` at 0x9a0f8 | load | 4 | `abs 0x9a758` | `-` |
| `str` at 0x9a0fc | store | 4 | `sp -0x28` | `0x44c` |
| `ldr` at 0x9a100 | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x9a104 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x9a118 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x9a124 | load | 4 | `abs 0x9a75c` | `-` |
| `ldr` at 0x9a128 | load | 4 | `abs 0x9a758` | `-` |
| `str` at 0x9a134 | store | 4 | `sp -0x24` | `ret(0xb5f90)` |
| `ldr` at 0x9a138 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9a13c | store | 4 | `sp -0xaa0` | `ret(0xb5f90)` |
| `ldr` at 0x9a140 | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x9a14c | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x9a158 | load | 4 | `sp -0xaa0` | `-` |
| `ldr` at 0x9a168 | load | 4 | `sp -0xaa0` | `-` |
| `str` at 0x9a17c | store | 4 | `sp -0x450` | `0x0` |
| `ldr` at 0x9a180 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a184 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x9a194 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a198 | load | 4 | `abs 0x9a73c` | `-` |
| `ldr` at 0x9a1ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a1b8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a1c0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a1d4 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x9a1d8 | load | 4 | `abs 0x9a738` | `-` |
| `ldr` at 0x9a1dc | load | 4 | `abs 0x2b25a0` | `-` |
| `ldr` at 0x9a1e8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a1f0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a1f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a200 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a208 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a210 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a214 | load | 4 | `abs 0x9a760` | `-` |
| `ldr` at 0x9a21c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a224 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a22c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a234 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a23c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a24c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a254 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a258 | load | 4 | `abs 0x9a76c` | `-` |
| `ldr` at 0x9a26c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a284 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a28c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a294 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a298 | load | 4 | `abs 0x9a770` | `-` |
| `ldr` at 0x9a2a0 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a2a8 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a2b0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a2bc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a2c0 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x9a2c8 | store | 4 | `sp -0x450` | `?` |
| `ldr` at 0x9a2d0 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x9a2d4 | store | 4 | `sp -0xa6c` | `[sp-2656]` |
| `ldr` at 0x9a2dc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a2e0 | load | 1 | `[sp-48] +0x13` | `-` |
| `ldr` at 0x9a2ec | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a2f8 | store | 4 | `sp -0xa94` | `ret(0xb823c)` |
| `ldr` at 0x9a2fc | load | 4 | `sp -0xa94` | `-` |
| `ldr` at 0x9a300 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9a30c | load | 4 | `sp -0xa94` | `-` |
| `str` at 0x9a310 | store | 4 | `sp -0x28` | `ret(0xb823c)` |
| `ldr` at 0x9a314 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a320 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a324 | load | 1 | `[sp-48] +0x12` | `-` |
| `str` at 0x9a328 | store | 4 | `sp -0xa80` | `[[sp-48]+18]:1` |
| `ldr` at 0x9a32c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a330 | load | 1 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x9a334 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a338 | load | 1 | `[sp-48] +0x21` | `-` |
| `str` at 0x9a344 | store | 4 | `sp -0xa84` | `?` |
| `str` at 0x9a34c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x9a354 | store | 4 | `sp -0x450` | `0x0` |
| `ldr` at 0x9a358 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a364 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a368 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x9a378 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a37c | load | 4 | `abs 0x9a734` | `-` |
| `ldr` at 0x9a390 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a394 | store | 4 | `sp -0xa9c` | `?` |
| `ldr` at 0x9a398 | load | 4 | `sp -0xa80` | `-` |
| `ldr` at 0x9a3a4 | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x9a3ac | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x9a3bc | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x9a3c8 | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x9a3d8 | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a3dc | load | 4 | `abs 0x9a73c` | `-` |
| `ldr` at 0x9a3f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a3fc | load | 4 | `sp -0x450` | `-` |
| `ldr` at 0x9a400 | load | 4 | `abs 0x9a73c` | `-` |
| `str` at 0x9a418 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x9a420 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x9a424 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x9a42c | store | 4 | `sp -0x450` | `?` |
| `ldr` at 0x9a434 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a440 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a44c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x9a454 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a460 | load | 4 | `abs 0x9a764` | `-` |
| `ldr` at 0x9a468 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9a470 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a47c | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x9a488 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x9a48c | load | 4 | `abs 0x9a774` | `-` |
| `ldr` at 0x9a4a0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a4a4 | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0x9a4a8 | load | 4 | `sp -0x654` | `-` |
| `str` at 0x9a4b0 | store | 4 | `sp -0x654` | `[sp-1620]+1` |
| `ldr` at 0x9a4b4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a4b8 | load | 1 | `[sp-48] +0x13` | `-` |
| `ldr` at 0x9a4c8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a4cc | load | 1 | `[sp-48] +0x24` | `-` |
| `ldr` at 0x9a4dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a4e0 | load | 4 | `[sp-48] +0x28` | `-` |
| `str` at 0x9a4e4 | store | 4 | `sp -0xaa4` | `[[sp-48]+40]` |
| `ldr` at 0x9a4ec | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a4f8 | store | 4 | `sp -0xaa4` | `ret(0xb823c)` |
| `ldr` at 0x9a4fc | load | 4 | `sp -0xaa4` | `-` |
| `str` at 0x9a500 | store | 4 | `sp -0x2c` | `[sp-2724]` |
| `ldr` at 0x9a50c | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x9a514 | store | 4 | `sp -0xa74` | `[sp-2676]+1` |
| `ldr` at 0x9a518 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a51c | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x9a528 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a530 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a544 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x9a548 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9a554 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a558 | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x9a564 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a56c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a574 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a57c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a584 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a58c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a590 | load | 4 | `abs 0x9a748` | `-` |
| `ldr` at 0x9a598 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a59c | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x9a5ac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a5b0 | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x9a5b4 | store | 4 | `sp -0xaa8` | `[[sp-48]+48]` |
| `ldr` at 0x9a5bc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a5c8 | store | 4 | `sp -0xaa8` | `ret(0xb823c)` |
| `ldr` at 0x9a5cc | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a5d4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a5dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a5e0 | load | 4 | `sp -0xaa8` | `-` |
| `ldr` at 0x9a5e8 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a5f0 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a5f8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a604 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a60c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a610 | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x9a61c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a620 | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x9a62c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a634 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a63c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a64c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a658 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a664 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a668 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a66c | load | 4 | `abs 0x9a74c` | `-` |
| `ldr` at 0x9a674 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a678 | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x9a688 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a68c | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x9a690 | store | 4 | `sp -0xaac` | `[[sp-48]+48]` |
| `ldr` at 0x9a698 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a6a4 | store | 4 | `sp -0xaac` | `ret(0xb823c)` |
| `ldr` at 0x9a6a8 | load | 4 | `sp -0xaac` | `-` |
| `ldr` at 0x9a6b8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a6c0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a6c8 | store | 4 | `(unknown)` | `ret(0x1b034)` |
| `ldr` at 0x9a6cc | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a6d4 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a6dc | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a6e8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a6f0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a6f4 | load | 1 | `[sp-48] +0x20` | `-` |
| `ldr` at 0x9a700 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a704 | load | 1 | `[sp-48] +0x2c` | `-` |
| `ldr` at 0x9a714 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a718 | load | 4 | `[sp-48] +0x30` | `-` |
| `str` at 0x9a71c | store | 4 | `sp -0xab0` | `[[sp-48]+48]` |
| `ldr` at 0x9a788 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a794 | store | 4 | `sp -0xab0` | `ret(0xb823c)` |
| `ldr` at 0x9a798 | load | 4 | `sp -0xab0` | `-` |
| `ldr` at 0x9a7b0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a7bc | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a7c4 | store | 4 | `(unknown)` | `ret(0x1b034)` |
| `str` at 0x9a7cc | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a7d0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x9a7d4 | load | 1 | `[sp-48] +0x34` | `-` |
| `ldr` at 0x9a7e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a7e8 | load | 4 | `[sp-48] +0x38` | `-` |
| `str` at 0x9a7ec | store | 4 | `sp -0xab4` | `[[sp-48]+56]` |
| `ldr` at 0x9a7f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a800 | store | 4 | `sp -0xab4` | `ret(0xb823c)` |
| `ldr` at 0x9a804 | load | 4 | `sp -0xab4` | `-` |
| `ldr` at 0x9a81c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a828 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a830 | store | 4 | `(unknown)` | `ret(0x1b034)` |
| `str` at 0x9a838 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a83c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a844 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a850 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a858 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a860 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a874 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x9a878 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9a87c | load | 4 | `[sp-48] +0x1c` | `-` |
| `ldr` at 0x9a888 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a890 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a8a0 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a8a8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a8b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a8b4 | load | 4 | `abs 0x9a768` | `-` |
| `ldr` at 0x9a8bc | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a8c4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a8cc | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a8d4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a8dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a8ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a8f4 | load | 4 | `sp -0x450` | `-` |
| `str` at 0x9a8fc | store | 4 | `sp -0x450` | `[sp-1104]-1` |
| `ldr` at 0x9a900 | load | 4 | `abs 0x9a76c` | `-` |
| `ldr` at 0x9a914 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a92c | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a934 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a93c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a940 | load | 4 | `abs 0x9a770` | `-` |
| `ldr` at 0x9a948 | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a950 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a958 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a964 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9a968 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x9a974 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9a980 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x9a984 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x9a98c | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x9a994 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x9a9a8 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x9a9b0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x9a9b8 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x9a9cc | load | 4 | `sp -0xa7c` | `-` |
| `str` at 0x9a9d4 | store | 4 | `sp -0xa7c` | `[sp-2684]+1` |
| `ldr` at 0x9a9dc | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x9a9e8 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x9a9f0 | store | 4 | `sp -0xa6c` | `0x0` |
| `ldr` at 0x9a9f4 | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x9a9f8 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x9aa08 | load | 4 | `sp -0xa6c` | `-` |
| `ldr` at 0x9aa0c | load | 4 | `abs 0x9a774` | `-` |
| `ldr` at 0x9aa20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9aa28 | load | 4 | `sp -0xa6c` | `-` |
| `str` at 0x9aa30 | store | 4 | `sp -0xa6c` | `[sp-2668]+1` |
| `ldr` at 0x9aa38 | load | 4 | `abs 0x9a778` | `-` |
| `str` at 0x9aa44 | store | 4 | `sp -0x24` | `ret(0x22e50)` |
| `ldr` at 0x9aa48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9aa54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9aa60 | load | 4 | `abs 0x9a77c` | `-` |
| `str` at 0x9aa64 | store | 4 | `sp -0xac8` | `0x11e5c0` |
| `str` at 0x9aa68 | store | 4 | `sp -0xac4` | `ret(0x21704)` |
| `ldr` at 0x9aa70 | load | 4 | `abs 0x9a780` | `-` |
| `ldr` at 0x9aa74 | load | 4 | `abs 0x9a784` | `-` |
| `ldr` at 0x9aa80 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x9aa90 | store | 4 | `sp -0xab8` | `?` |
| `str` at 0x9aa9c | store | 4 | `sp -0xab8` | `0x0` |
| `ldr` at 0x9aaa0 | load | 4 | `sp -0xab8` | `-` |
| `str` at 0x9aaa4 | store | 4 | `sp -0xa98` | `[sp-2744]` |
| `ldr` at 0x9aaa8 | load | 4 | `sp -0xa98` | `-` |
| `ldmdb` at 0x9aaac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x9aaac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9aaac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9aaac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x997f0 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x9983c | `(unknown callee) 0x1b1b4` | `0x8` | `?` | `?` | `?` |
| `bl` at 0x99884 | `(unknown callee) 0x1b4cc` | `sp-2664` | `0x0` | `0x618` | `sp-2664` |
| `bl` at 0x998b4 | `(unknown callee) 0xbc0fc` | `0x1` | `[sp-24]` | `[sp-2680]` | `0x0` |
| `bl` at 0x998ec | `(unknown callee) 0x22ac8` | `0x2b26dc` | `0x0` | `?` | `[sp-2656]` |
| `bl` at 0x99910 | `(unknown callee) 0x1b598` | `[[sp-2680]]` | `0x11e4b0` | `?` | `[sp-2680]` |
| `bl` at 0x99924 | `(unknown callee) 0x1b490` | `[sp-2680]` | `?` | `?` | `ret(0x22ac8)` |
| `bl` at 0x9993c | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x99a24 | `(unknown callee) 0x69908` | `sp-76` | `sp-72` | `0x0` | `sp-76` |
| `bl` at 0x99a2c | `(unknown callee) 0x1b2bc` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x99a54 | `(unknown callee) 0x69b84` | `sp-76` | `sp-72` | `[sp-2660]` | `ret(0x1b2bc)` |
| `bl` at 0x99a88 | `(unknown callee) 0xb785c` | `0x2b25a0` | `sp-2704` | `0x0` | `0x0` |
| `bl` at 0x99aa8 | `(unknown callee) 0x7a658` | `[sp-2708]` | `?` | `?` | `ret(0xb785c)` |
| `bl` at 0x99ae8 | `(unknown callee) 0x23b30` | `?` | `sp-48` | `0x64` | `?` |
| `bl` at 0x99afc | `(unknown callee) 0xb5fec` | `[sp-48]` | `?` | `?` | `ret(0x23b30)` |
| `bl` at 0x99b14 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x99b28 | `(unknown callee) 0x69d50` | `[0x2b25a0]` | `?` | `?` | `0x2b25a0` |
| `bl` at 0x99b44 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x99b60 | `(unknown callee) 0x1b364` | `?` | `0x11e4d8` | `?` | `?` |
| `bl` at 0x99b88 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0x99ba0 | `(unknown callee) 0x69d50` | `[sp-2708]` | `?` | `ret(0x1b2bc)` | `?` |
| `bl` at 0x99bb0 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `ret(0x69d50)` |
| `bl` at 0x99bcc | `(unknown callee) 0x1b364` | `?` | `0x11be48` | `?` | `?` |
| `bl` at 0x99c90 | `(unknown callee) 0xbc0fc` | `0x1` | `[sp-2660]` | `sp-2708` | `0x0` |
| `bl` at 0x99cd4 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x99d08 | `(unknown callee) 0x69d50` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x99d24 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x99d40 | `(unknown callee) 0x1b364` | `?` | `0x11e500` | `?` | `?` |
| `bl` at 0x99d5c | `(unknown callee) 0x1b364` | `?` | `[sp-2708]` | `?` | `?` |
| `bl` at 0x99da0 | `(unknown callee) 0x69d50` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x99dbc | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x99dec | `(unknown callee) 0x1b364` | `?` | `0x11e504` | `[sp-2684]+1` | `?` |
| `bl` at 0x99df4 | `(unknown callee) 0x1b034` | `[sp-2708]` | `?` | `?` | `?` |
| `bl` at 0x99e38 | `(unknown callee) 0x1b490` | `[sp-2708]` | `?` | `?` | `0x0` |
| `bl` at 0x99ea0 | `(unknown callee) 0x69908` | `sp-76` | `sp-72` | `0x0` | `sp-76` |
| `bl` at 0x99ea8 | `(unknown callee) 0x1b2bc` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x99ed0 | `(unknown callee) 0x69b84` | `sp-76` | `sp-72` | `[sp-2660]` | `ret(0x1b2bc)` |
| `bl` at 0x99f30 | `(unknown callee) 0xb7394` | `0x2b25a0` | `?` | `sp-2700` | `0x0` |
| `bl` at 0x99f9c | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x99fb0 | `(unknown callee) 0x69d50` | `[0x2b25a0]` | `?` | `?` | `0x2b25a0` |
| `bl` at 0x99fcc | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x99fe8 | `(unknown callee) 0x1b364` | `?` | `0x11e508` | `?` | `?` |
| `bl` at 0x9a010 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a040 | `(unknown callee) 0x69d50` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x9a050 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `ret(0x69d50)` |
| `bl` at 0x9a06c | `(unknown callee) 0x1b364` | `?` | `0x11be48` | `?` | `?` |
| `bl` at 0x9a0cc | `(unknown callee) 0x21704` | `[sp-36]` | `sp-20` | `?` | `[sp-2676]+1` |
| `bl` at 0x9a0dc | `(unknown callee) 0x2079c` | `0x11e530` | `ret(0x21704)` | `?` | `ret(0x21704)` |
| `bl` at 0x9a0f4 | `(unknown callee) 0x1b490` | `[sp-2660]` | `?` | `?` | `?` |
| `bl` at 0x9a12c | `(unknown callee) 0xb5f90` | `sp-48` | `0x2b2640` | `0x44c` | `sp-48` |
| `bl` at 0x9a1cc | `(unknown callee) 0x1b1b4` | `0x50` | `sp-20` | `?` | `[sp-2680]` |
| `bl` at 0x9a1e0 | `(unknown callee) 0x69d50` | `[0x2b25a0]` | `?` | `?` | `0x2b25a0` |
| `bl` at 0x9a1fc | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x9a218 | `(unknown callee) 0x1b364` | `?` | `0x11e554` | `?` | `?` |
| `bl` at 0x9a240 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a270 | `(unknown callee) 0x69d50` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x9a280 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `ret(0x69d50)` |
| `bl` at 0x9a29c | `(unknown callee) 0x1b364` | `?` | `0x11be48` | `?` | `?` |
| `bl` at 0x9a2f4 | `(unknown callee) 0xb823c` | `[sp-48]` | `0x0` | `ret(0xb5f90)` | `[[sp-48]+19]:1` |
| `bl` at 0x9a318 | `(unknown callee) 0xb5fec` | `[sp-48]` | `?` | `ret(0xb823c)` | `?` |
| `bl` at 0x9a444 | `(unknown callee) 0xb5fec` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x9a458 | `(unknown callee) 0x21704` | `[sp-36]` | `ret(0xb5f90)` | `ret(0xb5f90)` | `ret(0xb5f90)` |
| `bl` at 0x9a46c | `(unknown callee) 0x2079c` | `0x11e570` | `ret(0x21704)` | `[sp-36]` | `ret(0x21704)` |
| `bl` at 0x9a4f4 | `(unknown callee) 0xb823c` | `[sp-48]` | `0x0` | `?` | `?` |
| `bl` at 0x9a53c | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x9a55c | `(unknown callee) 0x69d50` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x9a578 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x9a594 | `(unknown callee) 0x1b364` | `?` | `0x11e500` | `?` | `?` |
| `bl` at 0x9a5c4 | `(unknown callee) 0xb823c` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x9a5e4 | `(unknown callee) 0x1b364` | `?` | `[sp-2728]` | `?` | `?` |
| `bl` at 0x9a624 | `(unknown callee) 0x69d50` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x9a640 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x9a670 | `(unknown callee) 0x1b364` | `?` | `0x11e504` | `[sp-2684]+1` | `?` |
| `bl` at 0x9a6a0 | `(unknown callee) 0xb823c` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x9a6ac | `(unknown callee) 0x1b034` | `[sp-2732]` | `?` | `?` | `?` |
| `bl` at 0x9a790 | `(unknown callee) 0xb823c` | `[sp-48]` | `0x1` | `?` | `?` |
| `bl` at 0x9a79c | `(unknown callee) 0x1b034` | `[sp-2736]` | `?` | `?` | `?` |
| `bl` at 0x9a7fc | `(unknown callee) 0xb823c` | `[sp-48]` | `0x2` | `[sp-2684]+1` | `?` |
| `bl` at 0x9a808 | `(unknown callee) 0x1b034` | `[sp-2740]` | `?` | `?` | `?` |
| `bl` at 0x9a86c | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `[sp-2680]` |
| `bl` at 0x9a880 | `(unknown callee) 0x69d50` | `[[sp-48]+28]` | `?` | `?` | `[sp-48]` |
| `bl` at 0x9a89c | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `?` |
| `bl` at 0x9a8b8 | `(unknown callee) 0x1b364` | `?` | `0x11e598` | `?` | `?` |
| `bl` at 0x9a8e0 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a918 | `(unknown callee) 0x69d50` | `?` | `sp-20` | `0xfffff5bc` | `?` |
| `bl` at 0x9a928 | `(unknown callee) 0x1b598` | `?` | `ret(0x69d50)` | `?` | `ret(0x69d50)` |
| `bl` at 0x9a944 | `(unknown callee) 0x1b364` | `?` | `0x11be48` | `?` | `?` |
| `bl` at 0x9a978 | `(unknown callee) 0xb5fec` | `[sp-48]` | `?` | `?` | `[sp-1624]` |
| `mov` at 0x9a9c8 | `(unknown callee) (unknown target)` | `sp-2664` | `?` | `sp-2664` | `?` |
| `bl` at 0x9aa24 | `(unknown callee) 0xb5fec` | `?` | `sp-20` | `0xfffff9c4` | `?` |
| `bl` at 0x9aa3c | `(unknown callee) 0x22e50` | `0x2b26dc` | `?` | `?` | `?` |
| `bl` at 0x9aa58 | `(unknown callee) 0x21704` | `ret(0x22e50)` | `?` | `?` | `ret(0x22e50)` |
| `bl` at 0x9aa7c | `(unknown callee) 0x21eb8` | `0x9` | `0x11e3d4` | `0x1995` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x998c8 | `ret(0xbc0fc) != 0x0` | `ret(0xbc0fc) == 0x0` |
| `bne` at 0x998d4 | `[sp-2656] != 0x0` | `[sp-2656] == 0x0` |
| `beq` at 0x99900 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `bge` at 0x99978 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x999a0 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x99a9c | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |
| `beq` at 0x99ab4 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x99af4 | `ret(0x23b30) == 0x0` | `ret(0x23b30) != 0x0` |
| `bne` at 0x99c60 | `? != ?` | `? == ?` |
| `beq` at 0x99ca4 | `ret(0xbc0fc) == 0x0` | `ret(0xbc0fc) != 0x0` |
| `beq` at 0x99cbc | `[sp-2708] == 0x0` | `[sp-2708] != 0x0` |
| `beq` at 0x99ce8 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x99f44 | `ret(0xb7394) == 0x0` | `ret(0xb7394) != 0x0` |
| `bne` at 0x99f54 | `[sp-1628] != 0x1` | `[sp-1628] == 0x1` |
| `blt` at 0x9a10c | `[sp-2668] < [sp-2656] (signed, 32-bit)` | `[sp-2668] >= [sp-2656] (signed, 32-bit)` |
| `beq` at 0x9a148 | `ret(0xb5f90) == 0x0` | `ret(0xb5f90) != 0x0` |
| `bgt` at 0x9a154 | `ret(0xb5f90) > 0x0 (signed, 32-bit)` | `ret(0xb5f90) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x9a160 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9a170 | `ret(0xb5f90) == 0x6e` | `ret(0xb5f90) != 0x6e` |
| `blt` at 0x9a18c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x9a1b4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9a2e8 | `[[sp-48]+19]:1 != 0x13` | `[[sp-48]+19]:1 == 0x13` |
| `ble` at 0x9a308 | `ret(0xb823c) <= [sp-40] (signed, 32-bit)` | `ret(0xb823c) > [sp-40] (signed, 32-bit)` |
| `bne` at 0x9a360 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x9a370 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x9a3a0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9a3b4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9a3d0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9a3f8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9a43c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9a478 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0x9a484 | `[sp-1624] == 0x0` | `[sp-1624] != 0x0` |
| `beq` at 0x9a4c0 | `[[sp-48]+19]:1 == 0x0` | `[[sp-48]+19]:1 != 0x0` |
| `bne` at 0x9a4d8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9a508 | `[sp-2724] == 0x0` | `[sp-2724] != 0x0` |
| `bls` at 0x9a524 | `[[sp-48]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-48]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x9a550 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bne` at 0x9a5a8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9a618 | `[[sp-48]+32]:1 != 0x2` | `[[sp-48]+32]:1 == 0x2` |
| `bne` at 0x9a684 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9a6fc | `[[sp-48]+32]:1 != 0x3` | `[[sp-48]+32]:1 == 0x3` |
| `bne` at 0x9a710 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9a7e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x9a970 | `[sp-1624] != 0x0` | `[sp-1624] == 0x0` |
| `beq` at 0x9a99c | `[sp-1624] == 0x0` | `[sp-1624] != 0x0` |
| `blt` at 0x9aa00 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x9aa50 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `beq` at 0x9aa88 | `[sp-2676] == 0x0` | `[sp-2676] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9aaac | `r0` | `?` |

## `smpcmd_intr_count`

- `smpcmd_intr_count`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb26a4 to 0xb29e8.
  Established. The unnamed `0xcd1b0` call has candidates `0x0`, `r1`, `r2`, `r3` (`bl` at 0xb26c0).
  A word assignment at `[sp-32] +0x0` writes `ret(0x1b1b4)` (`str` at 0xb271c). A later branch takes
  `ret(0xcce28) != 0x0` to `0xb2844` and falls through when zero (`bne` at 0xb2818). Interrupt-count
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb26b4 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xb26b8 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xb26c8 | store | 4 | `sp -0x24` | `ret(0xcd1b0)` |
| `str` at 0xb26d8 | store | 4 | `sp -0x28` | `ret(0xd7270)` |
| `str` at 0xb26e0 | store | 4 | `sp -0x2c` | `ret(0xc924c)` |
| `ldr` at 0xb26e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb26e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb26ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb26f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb271c | store | 4 | `[sp-32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xb2720 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xb2724 | store | 4 | `sp -0x30` | `[[sp-32]]` |
| `ldr` at 0xb2728 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb272c | load | 4 | `abs 0xb29d0` | `-` |
| `ldr` at 0xb2734 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2744 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb274c | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b2bc))` |
| `str` at 0xb2754 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb2758 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb275c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb276c | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb277c | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb2790 | load | 1 | `sp -0x34` | `-` |
| `str` at 0xb27a0 | store | 4 | `sp -0x38` | `ret(0xc926c)` |
| `ldr` at 0xb27a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb27a8 | load | 4 | `abs 0xb29d4` | `-` |
| `ldr` at 0xb27b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb27c8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb27d0 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b2bc))` |
| `ldr` at 0xb27d4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb27dc | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0xb27e8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb27ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb27f0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb2800 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb281c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb2830 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2834 | load | 4 | `abs 0xb29d8` | `-` |
| `ldr` at 0xb2844 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb2850 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb2870 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb2884 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb2898 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb289c | load | 4 | `abs 0xb29dc` | `-` |
| `ldr` at 0xb28a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb28b8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb28c0 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b2bc))` |
| `ldr` at 0xb28c4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb28cc | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0xb28d8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb28dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb28e0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb28f0 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb290c | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb2920 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2924 | load | 4 | `abs 0xb29e0` | `-` |
| `ldr` at 0xb2934 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb2940 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb2960 | load | 1 | `sp -0x34` | `-` |
| `ldrb` at 0xb2974 | load | 1 | `sp -0x34` | `-` |
| `ldr` at 0xb2988 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb298c | load | 4 | `abs 0xb29e4` | `-` |
| `ldr` at 0xb2998 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb29a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb29b0 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x1b2bc))` |
| `ldr` at 0xb29b4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb29bc | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldmdb` at 0xb29cc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb29cc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb29cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb29cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb29cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb26c0 | `(unknown callee) 0xcd1b0` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xb26d0 | `(unknown callee) 0xd7270` | `0x0` | `?` | `?` | `ret(0xcd1b0)` |
| `bl` at 0xb26dc | `(unknown callee) 0xc924c` | `ret(0xd7270)` | `?` | `?` | `ret(0xd7270)` |
| `bl` at 0xb2714 | `(unknown callee) 0x1b1b4` | `?` | `?` | `(([sp-36] + [sp-40]) + ret(0xc924c))` | `?` |
| `bl` at 0xb2730 | `(unknown callee) 0x1b598` | `[[sp-32]]` | `0x128504` | `?` | `[[sp-32]]` |
| `bl` at 0xb2738 | `(unknown callee) 0x1b2bc` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb2774 | `(unknown callee) 0xc92f8` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0xb2784 | `(unknown callee) 0xc9384` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0xb2798 | `(unknown callee) 0xc926c` | `[sp-52]:1` | `?` | `?` | `[sp-52]:1` |
| `bl` at 0xb27b4 | `(unknown callee) 0x1b52c` | `[sp-48]` | `0x128514` | `ret(0xc92f8)` | `?` |
| `bl` at 0xb27bc | `(unknown callee) 0x1b2bc` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb280c | `(unknown callee) 0xcce28` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2828 | `(unknown callee) 0xce8f8` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb283c | `(unknown callee) 0x1b52c` | `[sp-48]` | `0x12852c` | `ret(0xce8f8)` | `ret(0xce8f8)` |
| `bl` at 0xb285c | `(unknown callee) 0xce510` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb287c | `(unknown callee) 0xce8f8` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2890 | `(unknown callee) 0xce510` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb28a4 | `(unknown callee) 0x1b52c` | `[sp-48]` | `0x12854c` | `ret(0xce8f8)` | `ret(0xce510)` |
| `bl` at 0xb28ac | `(unknown callee) 0x1b2bc` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb28fc | `(unknown callee) 0xd72bc` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2918 | `(unknown callee) 0xd7eec` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb292c | `(unknown callee) 0x1b52c` | `[sp-48]` | `0x128564` | `ret(0xd7eec)` | `ret(0xd7eec)` |
| `bl` at 0xb294c | `(unknown callee) 0xd7604` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb296c | `(unknown callee) 0xd7eec` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2980 | `(unknown callee) 0xd7604` | `0x0` | `[sp-52]:1` | `?` | `[sp-52]:1` |
| `bl` at 0xb2994 | `(unknown callee) 0x1b52c` | `[sp-48]` | `0x128584` | `ret(0xd7eec)` | `ret(0xd7604)` |
| `bl` at 0xb299c | `(unknown callee) 0x1b2bc` | `[sp-48]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb2764 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb27f8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb2818 | `ret(0xcce28) != 0x0` | `ret(0xcce28) == 0x0` |
| `bne` at 0xb284c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xb2868 | `ret(0xce510) != 0x0` | `ret(0xce510) == 0x0` |
| `blt` at 0xb28e8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb2908 | `ret(0xd72bc) != 0x0` | `ret(0xd72bc) == 0x0` |
| `bne` at 0xb293c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xb2958 | `ret(0xd7604) != 0x0` | `ret(0xd7604) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb29cc | `r0` | `0x0` |

## `smpcmd_mgn`

- `smpcmd_mgn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x960f8 to 0x96604.
  Established. The word at `[sp-24] +0x70` is loaded (`ldr` at 0x96130), followed by a word read at
  `[[sp-24]+112] +0x0` (`ldr` at 0x96134). The unnamed `0x8dbc0` call carries `[[[sp-24]+112]]`,
  `[sp-28]`, an unknown third candidate, `[[sp-24]+112]` (`bl` at 0x9613c); its zero result takes
  `0x96158`, while nonzero falls through (`beq` at 0x96148). Effects beyond the unresolved computed
  transfer remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x96108 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x9610c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x96114 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x9611c | store | 4 | `sp -0x24` | `0x0` |
| `strb` at 0x96128 | store | 1 | `sp -0x36` | `(ret(0x4f4d0) & 0xff)` |
| `ldr` at 0x9612c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x96130 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x96134 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x96138 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x96150 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x96168 | store | 4 | `sp -0x2c` | `ret(0x3cc10)` |
| `ldr` at 0x9616c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x96178 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x96188 | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x9618c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x96190 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x96194 | load | 4 | `abs 0x965ec` | `-` |
| `str` at 0x961a0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x961a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x961b0 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x961b8 | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `ldr` at 0x961c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x961cc | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x961d4 | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `str` at 0x961d8 | store | 4 | `sp -0x24` | `[[[sp-24]+112]+8]` |
| `str` at 0x961e0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x961ec | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x961f4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x961f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x961fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9620c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x96220 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x96228 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x96238 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x96244 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0x96248 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9624c | load | 4 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x96258 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9625c | store | 4 | `sp -0x24` | `[sp-68]` |
| `ldr` at 0x96264 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9626c | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x96274 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x96278 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x96284 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x96294 | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x96298 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9629c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x962a0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x962a4 | load | 4 | `abs 0x965f0` | `-` |
| `ldr` at 0x962a8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x962b4 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x962c0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x962c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x962d4 | store | 4 | `sp -0x34` | `ret(0x403f4)` |
| `ldr` at 0x962d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x962e0 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x962e8 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldrls` at 0x962f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9657c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x965d4 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x965d8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x965d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x965d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x965d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x96120 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x9613c | `(unknown callee) 0x8dbc0` | `[[[sp-24]+112]]` | `[sp-28]` | `?` | `[[sp-24]+112]` |
| `bl` at 0x96164 | `(unknown callee) 0x3cc10` | `0x0` | `sp-40` | `?` | `sp-40` |
| `bl` at 0x96180 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `ret(0x3cc10)` |
| `bl` at 0x96198 | `(unknown callee) 0x1b598` | `[[sp-28]]` | `0x11d0d8` | `?` | `[sp-28]` |
| `bl` at 0x9628c | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `[sp-68]` | `[sp-44]` |
| `bl` at 0x962ac | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x11d3c4` | `[[sp-24]+40]` | `[sp-28]` |
| `bl` at 0x962cc | `(unknown callee) 0x403f4` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x96148 | `ret(0x8dbc0) == 0x0` | `ret(0x8dbc0) != 0x0` |
| `bne` at 0x96174 | `ret(0x3cc10) != 0x0` | `ret(0x3cc10) == 0x0` |
| `bne` at 0x961c0 | `[[[sp-24]+112]+8] != 0x21` | `[[[sp-24]+112]+8] == 0x21` |
| `blo` at 0x96204 | `[sp-68] < [sp-44] (unsigned, 32-bit)` | `[sp-68] >= [sp-44] (unsigned, 32-bit)` |
| `bne` at 0x96230 | `? != 0xff` | `? == 0xff` |
| `bne` at 0x96254 | `[sp-72]+1 != [[sp-24]+40]` | `[sp-72]+1 == [[sp-24]+40]` |
| `bne` at 0x96280 | `[sp-68] != [sp-44]` | `[sp-68] == [sp-44]` |
| `ldrls` at 0x962f0 | `[[[sp-24]+112]+4] <= 0x3 (unsigned, 32-bit)` | `[[[sp-24]+112]+4] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x965d8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x962f0).

## `smpcmd_tst`

- `smpcmd_tst`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1f48 to 0xb24d4.
  Established. The unnamed `0x1b1b4` call carries `0x50`, `r1`, `r2`, `0x1` (`bl` at 0xb1f70); its
  result is written as a word at `r1 +0x0` (`str` at 0xb1f78). The byte at `[[sp-28]] +0x0` is
  cleared (`strb` at 0xb1f88). The computed transfer takes unsigned `[[[sp-24]+112]] <= 0x8` to an
  unresolved destination and falls through above that bound (`ldrls` at 0xb1f9c). Test effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1f58 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb1f5c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb1f64 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xb1f68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1f78 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xb1f7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1f80 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xb1f88 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xb1f8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1f90 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xb1f94 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldrls` at 0xb1f9c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb247c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb248c | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0xb2490 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb2490 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb2490 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb2490 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1f70 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb1f9c | `[[[sp-24]+112]] <= 0x8 (unsigned, 32-bit)` | `[[[sp-24]+112]] > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb2490 | `r0` | `[sp-76]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb1f9c).

## `SMPCmd_2m_Init`

- `SMPCmd_2m_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa38f0 to 0xa3938.
  Established. The unnamed `0xbc05c` call carries `0x1209a4`, `0xba`, `r2`, `r3` (`bl` at 0xa3904).
  Four-byte absolute assignments write `0xa0cf4` at `0x14a560` (`str` at 0xa3910) and `0xa12f0` at
  `0x14a564` (`str` at 0xa391c). The configured pointer roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa38fc | load | 4 | `abs 0xa3924` | `-` |
| `ldr` at 0xa3908 | load | 4 | `abs 0xa3928` | `-` |
| `ldr` at 0xa390c | load | 4 | `abs 0xa392c` | `-` |
| `str` at 0xa3910 | store | 4 | `abs 0x14a560` | `0xa0cf4` |
| `ldr` at 0xa3914 | load | 4 | `abs 0xa3930` | `-` |
| `ldr` at 0xa3918 | load | 4 | `abs 0xa3934` | `-` |
| `str` at 0xa391c | store | 4 | `abs 0x14a564` | `0xa12f0` |
| `ldmdb` at 0xa3920 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa3920 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa3920 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3904 | `(unknown callee) 0xbc05c` | `0x1209a4` | `0xba` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa3920 | `r0` | `ret(0xbc05c)` |

## `IRPktSize`

- `IRPktSize`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8c74 to 0xb8cd0.
  Established. A four-byte stack assignment clears `sp -0x18` (`str` at 0xb8c8c). The loop branch
  takes `Z == 0` to `0xb8ca0` and falls through when `not (Z == 0)` (`bne` at 0xb8c98); its
  halfword read has an unknown effective address (`ldrh` at 0xb8ca4), and the return value remains
  unknown (`ldmdb` at 0xb8ccc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8c84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8c8c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb8c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8ca0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb8ca4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb8ca8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8cb0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb8cb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8cb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8cbc | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xb8cc4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb8ccc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8ccc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8ccc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb8c98 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8ccc | `r0` | `?` |

## `IRFmtPkt`

- `IRFmtPkt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8928 to 0xb89f4.
  Established. The first unnamed `0xb8654` call uses `[r1+24]`, `sp-60`, `sp-60`, `r1` (`bl` at
  0xb8954). A later byte read uses `[sp-24] +0x20` (`ldrb` at 0xb89a0), and a halfword read uses
  `[sp-24] +0x22` (`ldrh` at 0xb89b8). The return reports `[sp-28]` (`ldmdb` at 0xb89ec). Packet
  field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8938 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb893c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8940 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb8944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb894c | load | 4 | `r1 +0x18` | `-` |
| `ldr` at 0xb8958 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8960 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0xb896c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8970 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xb8980 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8984 | load | 1 | `[sp-24] +0x14` | `-` |
| `ldr` at 0xb8988 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb898c | load | 1 | `[sp-24] +0x21` | `-` |
| `str` at 0xb8998 | store | 4 | `sp -0x74` | `ret(0xb85bc)` |
| `ldr` at 0xb899c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb89a0 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0xb89a4 | store | 4 | `sp -0x70` | `[[sp-24]+32]:1` |
| `ldr` at 0xb89a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb89ac | load | 1 | `[sp-24] +0x13` | `-` |
| `str` at 0xb89b0 | store | 4 | `sp -0x6c` | `[[sp-24]+19]:1` |
| `ldr` at 0xb89b4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb89b8 | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0xb89bc | store | 4 | `sp -0x68` | `[[sp-24]+34]:2` |
| `str` at 0xb89c4 | store | 4 | `sp -0x64` | `sp-60` |
| `str` at 0xb89cc | store | 4 | `sp -0x60` | `sp-92` |
| `ldr` at 0xb89d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb89d4 | load | 4 | `abs 0xb89f0` | `-` |
| `ldr` at 0xb89d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb89e4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb89ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb89ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb89ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8954 | `(unknown callee) 0xb8654` | `[r1+24]` | `sp-60` | `sp-60` | `r1` |
| `bl` at 0xb8968 | `(unknown callee) 0xb8654` | `[[sp-24]+28]` | `sp-92` | `sp-92` | `[sp-24]` |
| `bl` at 0xb8978 | `(unknown callee) 0xb85bc` | `[[sp-24]+22]:1` | `?` | `?` | `[[sp-24]+22]:1` |
| `bl` at 0xb89e0 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x128ec4` | `[sp-20]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb89ec | `r0` | `[sp-28]` |

## `SocketExportUnixConnection`

- `SocketExportUnixConnection`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8ea0 to 0xb8ff4.
  Established. The unnamed `0x1b544` call carries `0x1`, `0x1`, `0x0`, `r3` (`bl` at 0xb8ec0); its
  signed nonnegative result takes the branch to `0xb8f00`, while a negative result falls through
  (`bge` at 0xb8ed4). A later word stack assignment writes `sp-130` at `sp -0x94` (`str` at
  0xb8f5c). Callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8eb0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8ec8 | store | 4 | `sp -0x88` | `ret(0x1b544)` |
| `ldr` at 0xb8ecc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xb8ee4 | load | 4 | `abs 0xb8fe0` | `-` |
| `ldr` at 0xb8ee8 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xb8eec | load | 4 | `abs 0xb8fe4` | `-` |
| `str` at 0xb8ef8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xb8f08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8f14 | store | 4 | `sp -0x8c` | `ret(0xb9e98)` |
| `ldr` at 0xb8f2c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xb8f34 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xb8f5c | store | 4 | `sp -0x94` | `sp-130` |
| `ldr` at 0xb8f64 | load | 4 | `abs 0xb8fe8` | `-` |
| `ldr` at 0xb8f68 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xb8f6c | load | 4 | `abs 0xb8fec` | `-` |
| `ldr` at 0xb8f74 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xb8f80 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xb8f88 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xb8fac | load | 4 | `abs 0xb8ff0` | `-` |
| `ldr` at 0xb8fb0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xb8fb4 | load | 4 | `abs 0xb8fe4` | `-` |
| `ldr` at 0xb8fbc | load | 4 | `sp -0x88` | `-` |
| `str` at 0xb8fc8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xb8fd0 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xb8fd4 | store | 4 | `sp -0x90` | `[sp-136]` |
| `ldr` at 0xb8fd8 | load | 4 | `sp -0x90` | `-` |
| `ldmdb` at 0xb8fdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8fdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8fdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8ec0 | `(unknown callee) 0x1b544` | `0x1` | `0x1` | `0x0` | `r3` |
| `bl` at 0xb8ed8 | `(unknown callee) 0x1b148` | `ret(0x1b544)` | `?` | `?` | `ret(0x1b544)` |
| `bl` at 0xb8ef0 | `(unknown callee) 0x2136c` | `?` | `0x1290d8` | `[ret(0x1b148)]` | `0x1290e0` |
| `bl` at 0xb8f0c | `(unknown callee) 0xb9e98` | `sp-132` | `[sp-20]` | `?` | `sp-132` |
| `bl` at 0xb8f24 | `(unknown callee) 0x1b0d0` | `sp-130` | `?` | `?` | `sp-130` |
| `bl` at 0xb8f38 | `(unknown callee) 0x1b370` | `[sp-136]` | `sp-132` | `[sp-140]` | `sp-132` |
| `bl` at 0xb8f48 | `(unknown callee) 0x1b148` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xb8f70 | `(unknown callee) 0x2136c` | `?` | `0x129100` | `[ret(0x1b148)]` | `0x129110` |
| `bl` at 0xb8f78 | `(unknown callee) 0x1b094` | `[sp-136]` | `?` | `?` | `?` |
| `bl` at 0xb8f90 | `(unknown callee) 0x1b1a8` | `[sp-136]` | `0x5` | `?` | `ret(0x1b370)` |
| `bl` at 0xb8fa0 | `(unknown callee) 0x1b148` | `ret(0x1b1a8)` | `?` | `?` | `ret(0x1b1a8)` |
| `bl` at 0xb8fb8 | `(unknown callee) 0x2136c` | `?` | `0x129108` | `[ret(0x1b148)]` | `0x1290e0` |
| `bl` at 0xb8fc0 | `(unknown callee) 0x1b094` | `[sp-136]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb8ed4 | `ret(0x1b544) >= 0x0 (signed, 32-bit)` | `ret(0x1b544) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb8f44 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb8f9c | `ret(0x1b1a8) >= 0x0 (signed, 32-bit)` | `ret(0x1b1a8) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8fdc | `r0` | `[sp-144]` |

## `SocketExportInetConnection`

- `SocketExportInetConnection`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8cd0 to 0xb8ea0.
  Established. The unnamed `0x1b544` call carries `0x2`, `0x1`, `0x0`, `0x1` (`bl` at 0xb8cf8). A
  halfword stack assignment writes `0x2` at `sp -0x24` (`strh` at 0xb8da8). A separate halfword at
  `sp -0x22` receives `(ret(0x1b4b4) & 0xffff)` (`strh` at 0xb8dbc). Connection effects and record
  fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8ce0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8ce8 | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xb8d00 | store | 4 | `sp -0x28` | `ret(0x1b544)` |
| `ldr` at 0xb8d04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8d18 | load | 4 | `abs 0xb8e84` | `-` |
| `ldr` at 0xb8d1c | load | 4 | `abs 0xb8e88` | `-` |
| `ldr` at 0xb8d20 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xb8d24 | load | 4 | `abs 0xb8e8c` | `-` |
| `str` at 0xb8d30 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xb8d40 | store | 4 | `sp -0x38` | `0x4` |
| `ldr` at 0xb8d44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8d68 | load | 4 | `abs 0xb8e84` | `-` |
| `ldr` at 0xb8d6c | load | 4 | `abs 0xb8e90` | `-` |
| `ldr` at 0xb8d70 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xb8d74 | load | 4 | `abs 0xb8e94` | `-` |
| `ldr` at 0xb8d7c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8d88 | store | 4 | `sp -0x30` | `?` |
| `strh` at 0xb8da8 | store | 2 | `sp -0x24` | `0x2` |
| `ldrh` at 0xb8dac | load | 2 | `sp -0x14` | `-` |
| `strh` at 0xb8dbc | store | 2 | `sp -0x22` | `(ret(0x1b4b4) & 0xffff)` |
| `str` at 0xb8dc4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb8dcc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8df4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8df8 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xb8dfc | load | 4 | `abs 0xb8e8c` | `-` |
| `str` at 0xb8e00 | store | 4 | `sp -0x34` | `0x1290e0` |
| `ldr` at 0xb8e04 | load | 4 | `abs 0xb8e84` | `-` |
| `ldr` at 0xb8e08 | load | 4 | `abs 0xb8e98` | `-` |
| `ldr` at 0xb8e0c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xb8e10 | load | 4 | `abs 0xb8e8c` | `-` |
| `ldr` at 0xb8e18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8e24 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb8e2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8e4c | load | 4 | `abs 0xb8e84` | `-` |
| `ldr` at 0xb8e50 | load | 4 | `abs 0xb8e9c` | `-` |
| `ldr` at 0xb8e54 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xb8e58 | load | 4 | `abs 0xb8e8c` | `-` |
| `ldr` at 0xb8e60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8e6c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb8e74 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8e78 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb8e7c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xb8e80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8e80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8e80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8cf8 | `(unknown callee) 0x1b544` | `0x2` | `0x1` | `0x0` | `0x1` |
| `bl` at 0xb8d10 | `(unknown callee) 0x1b148` | `ret(0x1b544)` | `?` | `?` | `ret(0x1b544)` |
| `bl` at 0xb8d28 | `(unknown callee) 0x2136c` | `0xfffffe70` | `0x1290d8` | `[ret(0x1b148)]` | `0x1290e0` |
| `bl` at 0xb8d50 | `(unknown callee) 0x1b250` | `ret(0x1b544)` | `0x1` | `0x2` | `sp-44` |
| `bl` at 0xb8d60 | `(unknown callee) 0x1b148` | `ret(0x1b250)` | `?` | `?` | `ret(0x1b250)` |
| `bl` at 0xb8d78 | `(unknown callee) 0x2136c` | `0xfffffe70` | `0x1290e4` | `[ret(0x1b148)]` | `0x1290f0` |
| `bl` at 0xb8d80 | `(unknown callee) 0x1b094` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb8da0 | `(unknown callee) 0x1b4cc` | `sp-36` | `0x0` | `0x10` | `sp-36` |
| `bl` at 0xb8db4 | `(unknown callee) 0x1b4b4` | `[sp-20]:2` | `?` | `?` | `[sp-20]:2` |
| `bl` at 0xb8dd8 | `(unknown callee) 0x1b370` | `[sp-40]` | `sp-36` | `0x10` | `sp-36` |
| `bl` at 0xb8de8 | `(unknown callee) 0x1b148` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xb8e14 | `(unknown callee) 0x213d0` | `0xfffffe70` | `0x129100` | `[ret(0x1b148)]` | `0x1290e0` |
| `bl` at 0xb8e1c | `(unknown callee) 0x1b094` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb8e34 | `(unknown callee) 0x1b1a8` | `[sp-40]` | `0x5` | `?` | `ret(0x1b370)` |
| `bl` at 0xb8e44 | `(unknown callee) 0x1b148` | `ret(0x1b1a8)` | `?` | `?` | `ret(0x1b1a8)` |
| `bl` at 0xb8e5c | `(unknown callee) 0x2136c` | `0xfffffe70` | `0x129108` | `[ret(0x1b148)]` | `0x1290e0` |
| `bl` at 0xb8e64 | `(unknown callee) 0x1b094` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb8d0c | `ret(0x1b544) >= 0x0 (signed, 32-bit)` | `ret(0x1b544) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb8d5c | `ret(0x1b250) >= 0x0 (signed, 32-bit)` | `ret(0x1b250) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb8de4 | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bge` at 0xb8e40 | `ret(0x1b1a8) >= 0x0 (signed, 32-bit)` | `ret(0x1b1a8) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8e80 | `r0` | `[sp-48]` |

## `charioPort`

- `charioPort`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x23e04 to 0x23e24.
  Established. The absolute word at `0x1462a4` is loaded (`ldr` at 0x23e14). The return reports
  `[0x1462a4]` (`ldmdb` at 0x23e1c). Port identity and consumers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x23e10 | load | 4 | `abs 0x23e20` | `-` |
| `ldr` at 0x23e14 | load | 4 | `abs 0x1462a4` | `-` |
| `ldmdb` at 0x23e1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x23e1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x23e1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x23e1c | `r0` | `[0x1462a4]` |

## `exceptionResetReport`

- `exceptionResetReport`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20e34 to 0x20e44.
  Established. The frame epilogue returns with entry `r0` preserved (`ldmdb` at 0x20e40). No report
  effect is established by this body, whose role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x20e40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20e40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20e40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20e40 | `r0` | `r0` |

## `JTAG_Cfg_set_ports_stub`

- `JTAG_Cfg_set_ports_stub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8530 to 0xe8554.
  Established. Two four-byte stack assignments retain `r0` at `sp -0x14` (`str` at 0xe8540) and `r1`
  at `sp -0x18` (`str` at 0xe8544). The return reports `0x0` (`ldmdb` at 0xe8550). This body
  establishes no external port access.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8540 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe8544 | store | 4 | `sp -0x18` | `r1` |
| `ldmdb` at 0xe8550 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8550 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8550 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8550 | `r0` | `0x0` |

## `brickGetType`

- `brickGetType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f4d0 to 0x4f4f0.
  Established. The absolute byte at `0x1434a8` is read (`ldrb` at 0x4f4e0). The return value is
  `[0x1434a8]:1` (`ldmdb` at 0x4f4e8). The byte encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4f4dc | load | 4 | `abs 0x4f4ec` | `-` |
| `ldrb` at 0x4f4e0 | load | 1 | `abs 0x1434a8` | `-` |
| `ldmdb` at 0x4f4e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f4e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f4e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f4e8 | `r0` | `[0x1434a8]:1` |

## `brickGetSubType`

- `brickGetSubType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f51c to 0x4f53c.
  Established. A byte read uses absolute `0x148f7c` (`ldrb` at 0x4f52c). The return reports
  `[0x148f7c]:1` (`ldmdb` at 0x4f534). Subtype interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4f528 | load | 4 | `abs 0x4f538` | `-` |
| `ldrb` at 0x4f52c | load | 1 | `abs 0x148f7c` | `-` |
| `ldmdb` at 0x4f534 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f534 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f534 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f534 | `r0` | `[0x148f7c]:1` |
