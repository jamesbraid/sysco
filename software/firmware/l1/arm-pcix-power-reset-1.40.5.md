# ARM PCI-X power and reset callers in 1.40.5

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
The fact sheets have no unresolved computed jumps. Indirect call targets and numeric callee
effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/pcix-power-reset-facts.txt) retain all tool sections.

## `pcixInit`

- `pcixInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57d08 to 0x58180.
  Established. The four-byte assignment writes `0x1` at absolute `0x1490e0` (`str` at 0x57d90). The
  unnamed call to `0x1b454` carries `[sp-52]`, `0x1`, `[0x1490e0]`, unknown, and its result is
  stored as four bytes at absolute `0x1490dc` (`bl` at 0x57dbc, `str` at 0x57dc4). Later four-byte
  stores include `[sp-48] +0xc`, `+0x10` and `+0x18`, retaining `[sp]`, `[sp+4]` and `[sp+8]`
  respectively (`str` at 0x57e48, `str` at 0x57e54, `str` at 0x57e60). Two unknown-target calls have
  candidates `[sp-24]`, `[sp-48]`, unknown, `[[sp-52]]`, and `[sp-24]` followed by three unknown
  candidates (`mov` at 0x5801c, `mov` at 0x5808c). Initialization and callback effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57d18 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x57d1c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x57d20 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x57d24 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x57d28 | load | 4 | `abs 0x58158` | `-` |
| `ldr` at 0x57d2c | load | 4 | `abs 0x1490dc` | `-` |
| `ldr` at 0x57d38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57d44 | load | 4 | `abs 0x5815c` | `-` |
| `str` at 0x57d48 | store | 4 | `sp -0x48` | `0x10c53c` |
| `ldr` at 0x57d4c | load | 4 | `abs 0x58158` | `-` |
| `ldr` at 0x57d50 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x57d54 | store | 4 | `sp -0x44` | `[0x1490dc]` |
| `ldr` at 0x57d58 | load | 4 | `abs 0x58160` | `-` |
| `ldr` at 0x57d5c | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x57d60 | store | 4 | `sp -0x40` | `[0x1490e0]` |
| `ldr` at 0x57d68 | load | 4 | `abs 0x58164` | `-` |
| `ldr` at 0x57d78 | load | 4 | `abs 0x58160` | `-` |
| `ldr` at 0x57d7c | load | 4 | `abs 0x1490e0` | `-` |
| `ldr` at 0x57d88 | load | 4 | `abs 0x58160` | `-` |
| `str` at 0x57d90 | store | 4 | `abs 0x1490e0` | `0x1` |
| `ldr` at 0x57d94 | load | 4 | `abs 0x58160` | `-` |
| `ldr` at 0x57d98 | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x57dac | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x57db0 | load | 4 | `abs 0x58158` | `-` |
| `ldr` at 0x57db4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x57dc4 | store | 4 | `abs 0x1490dc` | `ret(0x1b454)` |
| `ldr` at 0x57dc8 | load | 4 | `abs 0x1490dc` | `-` |
| `ldr` at 0x57dd4 | load | 4 | `abs 0x58168` | `-` |
| `str` at 0x57dd8 | store | 4 | `sp -0x48` | `0x10c580` |
| `ldr` at 0x57ddc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x57de0 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0x57de8 | load | 4 | `abs 0x58164` | `-` |
| `ldr` at 0x57df8 | load | 4 | `abs 0x58158` | `-` |
| `ldr` at 0x57dfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e10 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x57e18 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x57e1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57e24 | store | 4 | `[sp-48] +0x0` | `[sp-28]` |
| `ldr` at 0x57e28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57e30 | store | 4 | `[sp-48] +0x4` | `[sp-32]` |
| `ldr` at 0x57e34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57e3c | store | 4 | `[sp-48] +0x8` | `[sp-36]` |
| `ldr` at 0x57e40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e44 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x57e48 | store | 4 | `[sp-48] +0xc` | `[sp]` |
| `ldr` at 0x57e4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e50 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x57e54 | store | 4 | `[sp-48] +0x10` | `[sp+4]` |
| `ldr` at 0x57e58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e5c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x57e60 | store | 4 | `[sp-48] +0x18` | `[sp+8]` |
| `ldr` at 0x57e64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57e6c | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x57e7c | store | 4 | `[sp-48] +0x14` | `ret(0x1b454)` |
| `ldr` at 0x57e80 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57e94 | load | 4 | `abs 0x5816c` | `-` |
| `ldr` at 0x57e98 | load | 4 | `abs 0x58170` | `-` |
| `str` at 0x57ec4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x57ec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ed0 | load | 4 | `abs 0x5816c` | `-` |
| `ldr` at 0x57ed4 | load | 4 | `abs 0x58170` | `-` |
| `ldr` at 0x57ee0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x57ef0 | load | 4 | `abs 0x58174` | `-` |
| `str` at 0x57ef4 | store | 4 | `(unknown)` | `0x10c5c4` |
| `str` at 0x57ef8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x57efc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57f00 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x57f04 | store | 4 | `(unknown)` | `[[sp-48]+8]` |
| `ldr` at 0x57f0c | load | 4 | `abs 0x58164` | `-` |
| `ldr` at 0x57f1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57f20 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57f24 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57f2c | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x57f38 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x57f3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57f40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57f44 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0x57f54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57f58 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x57f6c | load | 4 | `abs 0x58178` | `-` |
| `str` at 0x57f74 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x57f78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57f7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57f84 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57f8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57f90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57fa4 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x57fac | load | 1 | `(unknown)` | `-` |
| `strb` at 0x57fb0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x57fb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57fb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57fc0 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57fc8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57fcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57fe0 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x57fe8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x57fec | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x57ff0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57ff4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57ffc | load | 4 | `[sp-48] +0x14` | `-` |
| `strb` at 0x58008 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x5800c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x58010 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x58014 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5802c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x58030 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x5803c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x58040 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58044 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58058 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldr` at 0x58060 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58064 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5806c | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x58078 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x5807c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x58094 | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x58098 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x580a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x580ac | load | 4 | `abs 0x5816c` | `-` |
| `ldr` at 0x580b0 | load | 4 | `abs 0x58170` | `-` |
| `str` at 0x580dc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x580e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x580e8 | load | 4 | `abs 0x5816c` | `-` |
| `ldr` at 0x580ec | load | 4 | `abs 0x58170` | `-` |
| `ldr` at 0x580f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x58108 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58114 | load | 4 | `abs 0x5817c` | `-` |
| `str` at 0x58118 | store | 4 | `(unknown)` | `0x10c600` |
| `str` at 0x5811c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x58120 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58124 | store | 4 | `(unknown)` | `[sp-44]` |
| `str` at 0x58128 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x58130 | load | 4 | `abs 0x58164` | `-` |
| `ldr` at 0x58140 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58148 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x58154 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x58154 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58154 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58154 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x57d74 | `(unknown callee) 0x21eb8` | `0x9` | `0x10c56c` | `0xa5` | `0x0` |
| `bl` at 0x57dbc | `(unknown callee) 0x1b454` | `[sp-52]` | `0x1` | `[0x1490e0]` | `?` |
| `bl` at 0x57df4 | `(unknown callee) 0x21eb8` | `0x9` | `0x10c56c` | `0xb1` | `0x0` |
| `bl` at 0x57e74 | `(unknown callee) 0x1b454` | `[[sp-48]+8]` | `0x8` | `[sp-48]` | `[sp-48]` |
| `bl` at 0x57e9c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x57ea8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x57ed8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x57ee8 | `(unknown callee) 0x1b598` | `[sp-52]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x57f18 | `(unknown callee) 0x21eb8` | `0x9` | `0x10c56c` | `0xc3` | `0x0` |
| `bl` at 0x57f30 | `(unknown callee) 0x1b4cc` | `[[sp-48]+20]` | `0x1` | `[[sp-48]+8]` | `[sp-48]` |
| `mov` at 0x5801c | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-48]` | `?` | `[[sp-52]]` |
| `mov` at 0x5808c | `(unknown callee) (unknown target)` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x580b4 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x580c0 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x580f0 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x58100 | `(unknown callee) 0x1b598` | `[sp-56]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x5810c | `(unknown callee) 0x21a38` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x5813c | `(unknown callee) 0x21eb8` | `0x1` | `0x10c56c` | `0xd0` | `0x0` |
| `bl` at 0x58150 | `(unknown callee) 0xa5430` | `?` | `?` | `?` | `[[sp-48]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x57d34 | `[0x1490dc] == 0x0` | `[0x1490dc] != 0x0` |
| `bne` at 0x57d40 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x57d84 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `bne` at 0x57dd0 | `[0x1490dc] != 0x0` | `[0x1490dc] == 0x0` |
| `bne` at 0x57e88 | `[[sp-48]+20] != 0x0` | `[[sp-48]+20] == 0x0` |
| `blt` at 0x57f4c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58028 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x58038 | `[[sp-52]+8] == 0x0` | `[[sp-52]+8] != 0x0` |
| `beq` at 0x580a0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58154 | `r0` | `ret(0xa5430)` |

## `pcixASICInit`

- `pcixASICInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58180 to 0x5827c.
  Established. Five four-byte stores have unresolved effective addresses (`str` at 0x581c0, `str` at
  0x581ec, `str` at 0x58218, `str` at 0x58244, `str` at 0x58270). Their retained values are `r1`,
  `[sp-28]`, `[sp-32]`, `[sp]` and `[sp+4]`, respectively (`str` at 0x581c0, `str` at 0x581ec, `str`
  at 0x58218, `str` at 0x58244, `str` at 0x58270). The exit carries `0x1490e4` (`ldmdb` at 0x58274).
  ASIC-configuration field meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58190 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58194 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x58198 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x5819c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x581a0 | load | 4 | `abs 0x58278` | `-` |
| `ldr` at 0x581a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x581bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x581c0 | store | 4 | `(unknown)` | `r1` |
| `ldr` at 0x581c4 | load | 4 | `abs 0x58278` | `-` |
| `ldr` at 0x581c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x581e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x581ec | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x581f0 | load | 4 | `abs 0x58278` | `-` |
| `ldr` at 0x581f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58214 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x58218 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x5821c | load | 4 | `abs 0x58278` | `-` |
| `ldr` at 0x58220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58240 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x58244 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x58248 | load | 4 | `abs 0x58278` | `-` |
| `ldr` at 0x5824c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5826c | load | 4 | `sp +0x4` | `-` |
| `str` at 0x58270 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldmdb` at 0x58274 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58274 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58274 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58274 | `r0` | `0x1490e4` |

## `pcixGetSlotCfg`

- `pcixGetSlotCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5827c to 0x5830c.
  Established. The branch takes `[0x1490e0] != 0x0` to `0x582b0` (`bne` at 0x582a0). A later zero
  `[sp-24]` takes the branch past an optional four-byte store (`beq` at 0x582dc). That store writes
  `[[sp-28]+12]` at `[sp-24] +0x0` (`str` at 0x582ec). The following four-byte read targets
  `[sp-28] +0x10` (`ldr` at 0x582f4). Slot-configuration semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5828c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58290 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58294 | load | 4 | `abs 0x58304` | `-` |
| `ldr` at 0x58298 | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x582a8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x582b0 | load | 4 | `abs 0x58308` | `-` |
| `ldr` at 0x582b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x582c8 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x582d0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x582d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x582e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x582e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x582e8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x582ec | store | 4 | `[sp-24] +0x0` | `[[sp-28]+12]` |
| `ldr` at 0x582f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x582f4 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0x582f8 | store | 4 | `sp -0x20` | `[[sp-28]+16]` |
| `ldr` at 0x582fc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x58300 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58300 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58300 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x582a0 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x582dc | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58300 | `r0` | `?` |

## `pcixGetBusCfg`

- `pcixGetBusCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5830c to 0x5839c.
  Established. The branch takes `[0x1490e0] != 0x0` to `0x58340` (`bne` at 0x58330). A zero
  `[sp-24]` takes a later branch past the four-byte store of `[[sp-28]+4]` at `[sp-24] +0x0` (`beq`
  at 0x5836c, `str` at 0x5837c). Another four-byte read targets `[sp-28] +0x8` (`ldr` at 0x58384).
  Bus-configuration meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5831c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58320 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x58324 | load | 4 | `abs 0x58394` | `-` |
| `ldr` at 0x58328 | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x58338 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x58340 | load | 4 | `abs 0x58398` | `-` |
| `ldr` at 0x58344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58358 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x58360 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x58364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58378 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x5837c | store | 4 | `[sp-24] +0x0` | `[[sp-28]+4]` |
| `ldr` at 0x58380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58384 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x58388 | store | 4 | `sp -0x20` | `[[sp-28]+8]` |
| `ldr` at 0x5838c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x58390 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58390 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58330 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x5836c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58390 | `r0` | `?` |

## `pcixUp`

- `pcixUp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5839c to 0x586a4.
  Established. The branch takes `[0x1490e0] != 0x0` to `0x583fc` (`bne` at 0x583cc). Later byte
  stores at unresolved effective addresses include `0x1` and `0x0` in separate paths (`strb` at
  0x58464, `strb` at 0x584c0). The unnamed call to `0x5a648` carries `[sp-20]`, `ret(0x5ba20)`,
  unknown, `ret(0x5ba20)` (`bl` at 0x58618); its signed-nonnegative result takes the branch to
  `0x58658` (`bge` at 0x5862c). A subsequent call to unnamed `0x5b3bc` uses `[sp-20]`, `[sp-36]`,
  unknown, `ret(0x5a648)` (`bl` at 0x58660). PCI-X enable effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x583ac | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x583b4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x583bc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x583c0 | load | 4 | `abs 0x58698` | `-` |
| `ldr` at 0x583c4 | load | 4 | `abs 0x1490e0` | `-` |
| `ldrb` at 0x583d0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x583dc | load | 4 | `abs 0x5869c` | `-` |
| `str` at 0x583e0 | store | 4 | `sp -0x34` | `0xfffff703` |
| `str` at 0x583ec | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x583f0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x583f4 | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x583fc | load | 4 | `abs 0x586a0` | `-` |
| `ldr` at 0x58400 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58414 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5841c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58420 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x58430 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x58434 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58438 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5843c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5844c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58450 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58458 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58464 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x58468 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58470 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x58478 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58484 | store | 4 | `sp -0x28` | `ret(0x591dc)` |
| `str` at 0x5848c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x58490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58494 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58498 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x584a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x584ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x584b4 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x584c0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x584c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x584cc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x584d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x584d8 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x584e0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x584ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x584f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x584f8 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58504 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x58508 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5850c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5851c | store | 4 | `sp -0x20` | `ret(0x5b944)` |
| `ldr` at 0x58520 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5852c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58530 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58538 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58544 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x58548 | load | 4 | `abs 0x5869c` | `-` |
| `str` at 0x5854c | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x58554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58558 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x58564 | store | 4 | `sp -0x28` | `ret(0x5a52c)` |
| `ldr` at 0x58568 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5856c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58574 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58580 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x58584 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58588 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x58590 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58594 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5859c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x585a8 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x585ac | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x585b0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x585b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x585c4 | store | 4 | `sp -0x24` | `ret(0x5ba20)` |
| `ldr` at 0x585c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x585d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x585d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x585e0 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x585ec | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x585f0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x585f4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x585f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58608 | store | 4 | `sp -0x30` | `ret(0x5bc00)` |
| `ldr` at 0x58610 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58614 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58620 | store | 4 | `sp -0x28` | `ret(0x5a648)` |
| `ldr` at 0x58624 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58630 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58634 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5863c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58648 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x5864c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58650 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x58658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5865c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58668 | store | 4 | `sp -0x28` | `ret(0x5b3bc)` |
| `ldr` at 0x5866c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58670 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58678 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58684 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x58688 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5868c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x58690 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x58694 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58694 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58694 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5847c | `(unknown callee) 0x591dc` | `[sp-20]` | `?` | `?` | `[[sp-28]+8]` |
| `bl` at 0x58514 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x5855c | `(unknown callee) 0x5a52c` | `[sp-20]` | `ret(0x5b944)` | `?` | `ret(0x5b944)` |
| `bl` at 0x585bc | `(unknown callee) 0x5ba20` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58600 | `(unknown callee) 0x5bc00` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58618 | `(unknown callee) 0x5a648` | `[sp-20]` | `ret(0x5ba20)` | `?` | `ret(0x5ba20)` |
| `bl` at 0x58660 | `(unknown callee) 0x5b3bc` | `[sp-20]` | `[sp-36]` | `?` | `ret(0x5a648)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x583cc | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x583d8 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x58428 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x58444 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x584a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x584e8 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58528 | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `bne` at 0x585d0 | `ret(0x5ba20) != 0x0` | `ret(0x5ba20) == 0x0` |
| `bge` at 0x5862c | `ret(0x5a648) >= 0x0 (signed, 32-bit)` | `ret(0x5a648) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58694 | `r0` | `?` |

## `pcixDown`

- `pcixDown`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x586a4 to 0x58964.
  Established. A nonzero `[0x1490e0]` takes the branch to `0x58704` (`bne` at 0x586d4). Included
  byte assignments at unresolved effective addresses write `0x1` and `0x0` in different paths
  (`strb` at 0x5876c, `strb` at 0x587c8). The unnamed call to `0x5abfc` carries `[sp-20]`,
  `ret(0x5b944)`, unknown, `ret(0x5b944)` (`bl` at 0x58864). Another call to unnamed `0x5acd8` has
  `[sp-20]`, `ret(0x5ba20)`, unknown, `ret(0x5ba20)` (`bl` at 0x58920). Disable and power effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x586b4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x586bc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x586c4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x586c8 | load | 4 | `abs 0x58958` | `-` |
| `ldr` at 0x586cc | load | 4 | `abs 0x1490e0` | `-` |
| `ldrb` at 0x586d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x586e4 | load | 4 | `abs 0x5895c` | `-` |
| `str` at 0x586e8 | store | 4 | `sp -0x34` | `0xfffff703` |
| `str` at 0x586f4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x586f8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x586fc | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x58704 | load | 4 | `abs 0x58960` | `-` |
| `ldr` at 0x58708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5871c | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x58724 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58728 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x58738 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x5873c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58740 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58744 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x58754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58758 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58760 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x5876c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x58770 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58778 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x58780 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5878c | store | 4 | `sp -0x28` | `ret(0x592d4)` |
| `str` at 0x58794 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x58798 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5879c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x587a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x587b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x587b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x587bc | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x587c8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x587cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x587d4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x587dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x587e0 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x587e8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x587f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x587f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58800 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x5880c | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x58810 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58814 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58824 | store | 4 | `sp -0x20` | `ret(0x5b944)` |
| `ldr` at 0x58828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58834 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58838 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58840 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x5884c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x58850 | load | 4 | `abs 0x5895c` | `-` |
| `str` at 0x58854 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x5885c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58860 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5886c | store | 4 | `sp -0x28` | `ret(0x5abfc)` |
| `ldr` at 0x58870 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58874 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5887c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58888 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x5888c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58890 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x58898 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5889c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x588a4 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x588b0 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x588b4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x588b8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x588bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x588cc | store | 4 | `sp -0x24` | `ret(0x5ba20)` |
| `ldr` at 0x588d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x588dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x588e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x588e8 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x588f4 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x588f8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x588fc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58900 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58910 | store | 4 | `sp -0x30` | `ret(0x5bc00)` |
| `ldr` at 0x58918 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5891c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58928 | store | 4 | `sp -0x28` | `ret(0x5acd8)` |
| `ldr` at 0x5892c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58930 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58938 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58944 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x58948 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5894c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x58950 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x58954 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58954 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58954 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58784 | `(unknown callee) 0x592d4` | `[sp-20]` | `?` | `?` | `[[sp-28]+8]` |
| `bl` at 0x5881c | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58864 | `(unknown callee) 0x5abfc` | `[sp-20]` | `ret(0x5b944)` | `?` | `ret(0x5b944)` |
| `bl` at 0x588c4 | `(unknown callee) 0x5ba20` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58908 | `(unknown callee) 0x5bc00` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58920 | `(unknown callee) 0x5acd8` | `[sp-20]` | `ret(0x5ba20)` | `?` | `ret(0x5ba20)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x586d4 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x586e0 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x58730 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x5874c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x587a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x587f0 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58830 | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `bne` at 0x588d8 | `ret(0x5ba20) != 0x0` | `ret(0x5ba20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58954 | `r0` | `?` |

## `pcixReset`

- `pcixReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58964 to 0x589e8.
  Established. The unnamed call to `0x593cc` carries `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`,
  `(r1 & 0xff)` (`bl` at 0x58998); its zero result takes the branch to `0x589bc` (`beq` at 0x589ac). That
  arm includes unnamed `0x5cb28` with `0x2`, unknown, unknown, `ret(0x593cc)`, then `0x596a0` with
  `[sp-20]`, `[sp-21]:1`, `[sp-22]:1`, `[sp-21]:1` (`bl` at 0x589c0, `bl` at 0x589d4). Reset effects
  and delay units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58974 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5897c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x58984 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x58988 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5898c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58990 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x589a0 | store | 4 | `sp -0x1c` | `ret(0x593cc)` |
| `ldr` at 0x589a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x589b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x589b4 | store | 4 | `sp -0x20` | `ret(0x593cc)` |
| `ldrb` at 0x589c4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x589c8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x589cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x589dc | store | 4 | `sp -0x20` | `ret(0x596a0)` |
| `ldr` at 0x589e0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x589e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x589e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x589e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58998 | `(unknown callee) 0x593cc` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x589c0 | `(unknown callee) 0x5cb28` | `0x2` | `?` | `?` | `ret(0x593cc)` |
| `bl` at 0x589d4 | `(unknown callee) 0x596a0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x589ac | `ret(0x593cc) == 0x0` | `ret(0x593cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x589e4 | `r0` | `?` |

## `pcixLedOn`

- `pcixLedOn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x589e8 to 0x58c50.
  Established. Two unnamed calls to `0x5c894` have candidates `[sp-20]`, unknown, `0x1`, unknown
  (`bl` at 0x58ac0, `bl` at 0x58b94). The first zero result takes the branch to `0x58aec` (`beq` at
  0x58ad4). A later call to the same target carries `[sp-20]`, `ret(0x5ba20)`, `0x1`, `ret(0x5ba20)`
  (`bl` at 0x58c30). A four-byte assignment at `sp -0x28` retains `ret(0x5c894)` (`str` at 0x58ae8).
  LED register meanings and callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x589f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x58a00 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x58a08 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x58a10 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58a14 | load | 4 | `abs 0x58c44` | `-` |
| `ldr` at 0x58a18 | load | 4 | `abs 0x1490e0` | `-` |
| `ldrb` at 0x58a24 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58a30 | load | 4 | `abs 0x58c48` | `-` |
| `str` at 0x58a34 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x58a40 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x58a44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x58a48 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x58a50 | load | 4 | `abs 0x58c4c` | `-` |
| `ldr` at 0x58a54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58a68 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x58a70 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58a74 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x58a84 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x58a88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58a8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58a90 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x58aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58aa4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58aac | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58ab4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58ac8 | store | 4 | `sp -0x2c` | `ret(0x5c894)` |
| `ldr` at 0x58acc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58ad8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58ae4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58ae8 | store | 4 | `sp -0x28` | `ret(0x5c894)` |
| `ldr` at 0x58aec | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58af4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58afc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58b00 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58b08 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x58b14 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58b18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58b28 | store | 4 | `sp -0x20` | `ret(0x5b944)` |
| `ldr` at 0x58b2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58b38 | load | 4 | `abs 0x58c48` | `-` |
| `str` at 0x58b3c | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x58b44 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58b48 | load | 1 | `ret(0x5b944) +0x1` | `-` |
| `str` at 0x58b4c | store | 4 | `sp -0x30` | `[ret(0x5b944)+1]:1` |
| `ldr` at 0x58b50 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58b54 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58b58 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58b5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58b64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58b74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58b78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58b80 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58b88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58b9c | store | 4 | `sp -0x2c` | `ret(0x5c894)` |
| `ldr` at 0x58ba0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58bac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58bb8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58bbc | store | 4 | `sp -0x28` | `ret(0x5c894)` |
| `ldr` at 0x58bc0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58bc8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58bd0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58bd4 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58bdc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58be0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58be4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58bf4 | store | 4 | `sp -0x24` | `ret(0x5ba20)` |
| `ldr` at 0x58bf8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x58c04 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58c08 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58c0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58c1c | store | 4 | `sp -0x34` | `ret(0x5bc00)` |
| `ldr` at 0x58c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58c28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58c38 | store | 4 | `sp -0x34` | `ret(0x5c894)` |
| `ldr` at 0x58c3c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x58c40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58c40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58c40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58ac0 | `(unknown callee) 0x5c894` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x58b20 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58b94 | `(unknown callee) 0x5c894` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x58bec | `(unknown callee) 0x5ba20` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58c14 | `(unknown callee) 0x5bc00` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58c30 | `(unknown callee) 0x5c894` | `[sp-20]` | `ret(0x5ba20)` | `0x1` | `ret(0x5ba20)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58a20 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x58a2c | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x58a7c | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x58a98 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58ad4 | `ret(0x5c894) == 0x0` | `ret(0x5c894) != 0x0` |
| `bne` at 0x58ae0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58b10 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58b34 | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `blt` at 0x58b6c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58ba8 | `ret(0x5c894) == 0x0` | `ret(0x5c894) != 0x0` |
| `bne` at 0x58bb4 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58c00 | `ret(0x5ba20) != 0x0` | `ret(0x5ba20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58c40 | `r0` | `?` |

## `pcixLedOff`

- `pcixLedOff`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58c50 to 0x58eb8.
  Established. The unnamed calls to `0x5c894` include `[sp-20]`, unknown, `0x0`, unknown, and
  `[sp-20]`, unknown, `0x1`, unknown (`bl` at 0x58d28, `bl` at 0x58dfc). The first zero result takes
  the branch to `0x58d54` (`beq` at 0x58d3c). The later call to that target carries `[sp-20]`,
  `ret(0x5ba20)`, `0x0`, `ret(0x5ba20)` (`bl` at 0x58e98). The body does not establish that every
  call uses a zero third candidate. LED-off effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58c60 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x58c68 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x58c70 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x58c78 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58c7c | load | 4 | `abs 0x58eac` | `-` |
| `ldr` at 0x58c80 | load | 4 | `abs 0x1490e0` | `-` |
| `ldrb` at 0x58c8c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58c98 | load | 4 | `abs 0x58eb0` | `-` |
| `str` at 0x58c9c | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x58ca8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x58cac | load | 4 | `sp -0x38` | `-` |
| `str` at 0x58cb0 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x58cb8 | load | 4 | `abs 0x58eb4` | `-` |
| `ldr` at 0x58cbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58cd0 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x58cd8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58cdc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x58cec | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x58cf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58cf4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58cf8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x58d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58d0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58d14 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58d1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58d30 | store | 4 | `sp -0x2c` | `ret(0x5c894)` |
| `ldr` at 0x58d34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58d40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58d4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58d50 | store | 4 | `sp -0x28` | `ret(0x5c894)` |
| `ldr` at 0x58d54 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58d5c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58d64 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58d68 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58d70 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x58d7c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58d80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58d90 | store | 4 | `sp -0x20` | `ret(0x5b944)` |
| `ldr` at 0x58d94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58da0 | load | 4 | `abs 0x58eb0` | `-` |
| `str` at 0x58da4 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x58dac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58db0 | load | 1 | `ret(0x5b944) +0x1` | `-` |
| `str` at 0x58db4 | store | 4 | `sp -0x30` | `[ret(0x5b944)+1]:1` |
| `ldr` at 0x58db8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58dbc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58dc0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58dc4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58dcc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58ddc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58de0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58de8 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58df0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58e04 | store | 4 | `sp -0x2c` | `ret(0x5c894)` |
| `ldr` at 0x58e08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58e14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58e20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58e24 | store | 4 | `sp -0x28` | `ret(0x5c894)` |
| `ldr` at 0x58e28 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58e30 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58e38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58e3c | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58e44 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58e48 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58e4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58e5c | store | 4 | `sp -0x24` | `ret(0x5ba20)` |
| `ldr` at 0x58e60 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x58e6c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58e70 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58e74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58e84 | store | 4 | `sp -0x34` | `ret(0x5bc00)` |
| `ldr` at 0x58e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58e90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58ea0 | store | 4 | `sp -0x34` | `ret(0x5c894)` |
| `ldr` at 0x58ea4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x58ea8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58ea8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58ea8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58d28 | `(unknown callee) 0x5c894` | `[sp-20]` | `?` | `0x0` | `?` |
| `bl` at 0x58d88 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58dfc | `(unknown callee) 0x5c894` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x58e54 | `(unknown callee) 0x5ba20` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58e7c | `(unknown callee) 0x5bc00` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58e98 | `(unknown callee) 0x5c894` | `[sp-20]` | `ret(0x5ba20)` | `0x0` | `ret(0x5ba20)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58c88 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x58c94 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x58ce4 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x58d00 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58d3c | `ret(0x5c894) == 0x0` | `ret(0x5c894) != 0x0` |
| `bne` at 0x58d48 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58d78 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58d9c | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `blt` at 0x58dd4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58e10 | `ret(0x5c894) == 0x0` | `ret(0x5c894) != 0x0` |
| `bne` at 0x58e1c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58e68 | `ret(0x5ba20) != 0x0` | `ret(0x5ba20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58ea8 | `r0` | `?` |

## `pcixStatus`

- `pcixStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58eb8 to 0x59028.
  Established. A nonzero `[0x1490e0]` takes the branch to `0x58efc` (`bne` at 0x58eec). The other
  arm stores four-byte `0xfffff703` at `sp -0x30` (`str` at 0x58ef4). The unnamed call to `0x5bf68`
  has candidates `[sp-20]`, `ret(0x5ba20)`, `[sp-28]`, `ret(0x5ba20)` (`bl` at 0x58f74); its zero
  result takes the branch to `0x58f98` (`beq` at 0x58f88). The later unknown-target call carries
  `[sp-20]`, `ret(0x5b944)`, `[sp]`, unknown (`mov` at 0x59004). Status-field and callback semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58ec8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58ecc | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x58ed4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x58edc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x58ee0 | load | 4 | `abs 0x59018` | `-` |
| `ldr` at 0x58ee4 | load | 4 | `abs 0x1490e0` | `-` |
| `ldr` at 0x58ef0 | load | 4 | `abs 0x5901c` | `-` |
| `str` at 0x58ef4 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x58efc | load | 4 | `abs 0x59020` | `-` |
| `ldr` at 0x58f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58f14 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x58f1c | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x58f20 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58f24 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58f28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58f38 | store | 4 | `sp -0x24` | `ret(0x5ba20)` |
| `ldr` at 0x58f3c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x58f48 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58f4c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58f50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58f60 | store | 4 | `sp -0x30` | `ret(0x5bc00)` |
| `ldr` at 0x58f68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58f6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58f70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58f7c | store | 4 | `sp -0x2c` | `ret(0x5bf68)` |
| `ldr` at 0x58f80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58f8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58f90 | store | 4 | `sp -0x30` | `ret(0x5bf68)` |
| `ldrb` at 0x58f98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58f9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58fac | store | 4 | `sp -0x28` | `ret(0x5b944)` |
| `ldr` at 0x58fb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58fbc | load | 4 | `abs 0x5901c` | `-` |
| `str` at 0x58fc0 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x58fc8 | load | 4 | `abs 0x59024` | `-` |
| `ldr` at 0x58fcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58fd0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x58ff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x58ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58ff8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58ffc | load | 4 | `sp +0x0` | `-` |
| `str` at 0x5900c | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0x59010 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x59014 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59014 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59014 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58f30 | `(unknown callee) 0x5ba20` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58f58 | `(unknown callee) 0x5bc00` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58f74 | `(unknown callee) 0x5bf68` | `[sp-20]` | `ret(0x5ba20)` | `[sp-28]` | `ret(0x5ba20)` |
| `bl` at 0x58fa4 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `mov` at 0x59004 | `(unknown callee) (unknown target)` | `[sp-20]` | `ret(0x5b944)` | `[sp]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58eec | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `bne` at 0x58f44 | `ret(0x5ba20) != 0x0` | `ret(0x5ba20) == 0x0` |
| `beq` at 0x58f88 | `ret(0x5bf68) == 0x0` | `ret(0x5bf68) != 0x0` |
| `bne` at 0x58fb8 | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59014 | `r0` | `?` |

## `pcixBusInReset`

- `pcixBusInReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59028 to 0x591dc.
  Established. The unnamed call to `0x5b944` has candidates `[sp-20]`, `[sp-21]:1`, unknown,
  `[sp-21]:1` (`bl` at 0x59090); its nonzero result takes the branch to `0x590b4` (`bne` at
  0x590a4). A later call to unnamed `0x5bf68` carries `[sp-20]`, `[sp-36]`, `sp-44`, `sp-44` (`bl`
  at 0x5911c). The zero-result branch reaches `0x59140` (`beq` at 0x59130). Subsequent byte reads
  have unresolved effective addresses (`ldrb` at 0x5914c, `ldrb` at 0x5916c, `ldrb` at 0x5918c).
  Bus-reset status semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59038 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x59040 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x59044 | load | 4 | `abs 0x591cc` | `-` |
| `ldr` at 0x59048 | load | 4 | `abs 0x1490e0` | `-` |
| `ldr` at 0x59054 | load | 4 | `abs 0x591d0` | `-` |
| `str` at 0x59058 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x59060 | load | 4 | `abs 0x591d4` | `-` |
| `ldr` at 0x59064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59078 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x59080 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x59084 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59088 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59098 | store | 4 | `sp -0x20` | `ret(0x5b944)` |
| `ldr` at 0x5909c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x590a8 | load | 4 | `abs 0x591d0` | `-` |
| `str` at 0x590ac | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x590b8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x590bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x590c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x590c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x590d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x590d8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x590dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x590e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x590ec | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x590f4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x59100 | load | 4 | `abs 0x591d8` | `-` |
| `str` at 0x59104 | store | 4 | `sp -0x38` | `0xfffff702` |
| `ldr` at 0x59110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59114 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59124 | store | 4 | `sp -0x30` | `ret(0x5bf68)` |
| `ldr` at 0x59128 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59134 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59138 | store | 4 | `sp -0x38` | `ret(0x5bf68)` |
| `ldrb` at 0x5914c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5916c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5918c | load | 1 | `(unknown)` | `-` |
| `str` at 0x591a4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x591ac | load | 4 | `sp -0x34` | `-` |
| `str` at 0x591b4 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0x591c0 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x591c4 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x591c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x591c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x591c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59090 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x5911c | `(unknown callee) 0x5bf68` | `[sp-20]` | `[sp-36]` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59050 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `bne` at 0x590a4 | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `blt` at 0x590cc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x590fc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x59130 | `ret(0x5bf68) == 0x0` | `ret(0x5bf68) != 0x0` |
| `beq` at 0x5915c | `? == 0x3` | `? != 0x3` |
| `bne` at 0x5917c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5919c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x591c8 | `r0` | `?` |

## `pcixPowerUp`

- `pcixPowerUp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x591dc to 0x592d4.
  Established. The branch takes `[0x1490e0] != 0x0` to `0x59214` (`bne` at 0x59204). The zero
  fall-through stores four-byte `0x0` at `sp -0x28` (`str` at 0x5920c). The unnamed call to
  `0x5a52c` carries `[sp-20]` followed by three unknown candidates (`bl` at 0x59280); its zero
  result takes the branch to `0x592ac` (`beq` at 0x59294). A later four-byte assignment at
  `sp -0x1c` retains `ret(0x5a52c)` (`str` at 0x592a8). Power-up effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x591ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x591f4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x591f8 | load | 4 | `abs 0x592cc` | `-` |
| `ldr` at 0x591fc | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x5920c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x59214 | load | 4 | `abs 0x592d0` | `-` |
| `ldr` at 0x59218 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5922c | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x59234 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x5923c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x59240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59244 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59248 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x59258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5925c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59270 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x59278 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59288 | store | 4 | `sp -0x20` | `ret(0x5a52c)` |
| `ldr` at 0x5928c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59298 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x592a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x592a8 | store | 4 | `sp -0x1c` | `ret(0x5a52c)` |
| `ldr` at 0x592ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x592b4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x592bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x592c0 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x592c4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x592c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x592c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x592c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59280 | `(unknown callee) 0x5a52c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59204 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `blt` at 0x59250 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x59294 | `ret(0x5a52c) == 0x0` | `ret(0x5a52c) != 0x0` |
| `bne` at 0x592a0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x592c8 | `r0` | `?` |

## `pcixPowerDown`

- `pcixPowerDown`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x592d4 to 0x593cc.
  Established. A nonzero `[0x1490e0]` takes the branch to `0x5930c` (`bne` at 0x592fc). The other
  arm assigns four-byte `0x0` at `sp -0x28` (`str` at 0x59304). The unnamed call to `0x5abfc` has
  candidates `[sp-20]`, unknown, unknown, unknown (`bl` at 0x59378); its zero result takes the
  branch to `0x593a4` (`beq` at 0x5938c). A later four-byte store at `sp -0x1c` retains
  `ret(0x5abfc)` (`str` at 0x593a0). Physical power-down behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x592e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x592ec | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x592f0 | load | 4 | `abs 0x593c4` | `-` |
| `ldr` at 0x592f4 | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x59304 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5930c | load | 4 | `abs 0x593c8` | `-` |
| `ldr` at 0x59310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59324 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5932c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x59334 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x59338 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5933c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59340 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x59350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59354 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59368 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x59370 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59380 | store | 4 | `sp -0x20` | `ret(0x5abfc)` |
| `ldr` at 0x59384 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59390 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5939c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x593a0 | store | 4 | `sp -0x1c` | `ret(0x5abfc)` |
| `ldr` at 0x593a4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x593ac | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x593b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x593b8 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x593bc | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x593c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x593c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x593c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59378 | `(unknown callee) 0x5abfc` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x592fc | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `blt` at 0x59348 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5938c | `ret(0x5abfc) == 0x0` | `ret(0x5abfc) != 0x0` |
| `bne` at 0x59398 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x593c0 | `r0` | `?` |

## `pcixResetHold`

- `pcixResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x593cc to 0x596a0.
  Established. The unnamed calls to `0x5ade4` include `[sp-20]`, unknown, `0x1`, unknown in two
  paths (`bl` at 0x594a4, `bl` at 0x595e4). The first zero result takes the branch to `0x594d0`
  (`beq` at 0x594b8). A later call to that target carries `[sp-20]`, `ret(0x5ba20)`, `0x0`,
  `ret(0x5ba20)` (`bl` at 0x59680). A byte assignment at an unresolved effective address has unknown
  value (`strb` at 0x59538). Reset-hold signaling is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x593dc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x593e4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x593ec | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x593f4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x593f8 | load | 4 | `abs 0x59694` | `-` |
| `ldr` at 0x593fc | load | 4 | `abs 0x1490e0` | `-` |
| `ldrb` at 0x59408 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59414 | load | 4 | `abs 0x59698` | `-` |
| `str` at 0x59418 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x59424 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x59428 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x5942c | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x59434 | load | 4 | `abs 0x5969c` | `-` |
| `ldr` at 0x59438 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5944c | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x59454 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x59458 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x59468 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x5946c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59470 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59474 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x59484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59488 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59490 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x59498 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x594ac | store | 4 | `sp -0x2c` | `ret(0x5ade4)` |
| `ldr` at 0x594b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x594bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x594c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x594cc | store | 4 | `sp -0x28` | `ret(0x5ade4)` |
| `ldr` at 0x594d0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x594d8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0x594e4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x594e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x594ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x594f0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x59500 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59504 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5950c | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x59514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59518 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5952c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x59534 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x59538 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x5953c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59544 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x5954c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x59550 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x59558 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x59564 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59568 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59578 | store | 4 | `sp -0x20` | `ret(0x5b944)` |
| `ldr` at 0x5957c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59588 | load | 4 | `abs 0x59698` | `-` |
| `str` at 0x5958c | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x59594 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59598 | load | 1 | `ret(0x5b944) +0x1` | `-` |
| `str` at 0x5959c | store | 4 | `sp -0x30` | `[ret(0x5b944)+1]:1` |
| `ldr` at 0x595a0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x595a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x595a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x595ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x595b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x595c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x595c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x595d0 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x595d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x595ec | store | 4 | `sp -0x2c` | `ret(0x5ade4)` |
| `ldr` at 0x595f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x595fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59608 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5960c | store | 4 | `sp -0x28` | `ret(0x5ade4)` |
| `ldr` at 0x59610 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59618 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x59620 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x59624 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x5962c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59630 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59634 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59644 | store | 4 | `sp -0x24` | `ret(0x5ba20)` |
| `ldr` at 0x59648 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x59654 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59658 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5965c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5966c | store | 4 | `sp -0x34` | `ret(0x5bc00)` |
| `ldr` at 0x59674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59678 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59688 | store | 4 | `sp -0x34` | `ret(0x5ade4)` |
| `ldr` at 0x5968c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x59690 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59690 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x594a4 | `(unknown callee) 0x5ade4` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x59570 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x595e4 | `(unknown callee) 0x5ade4` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x5963c | `(unknown callee) 0x5ba20` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59664 | `(unknown callee) 0x5bc00` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59680 | `(unknown callee) 0x5ade4` | `[sp-20]` | `ret(0x5ba20)` | `0x0` | `ret(0x5ba20)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59404 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x59410 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x59460 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x5947c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x594b8 | `ret(0x5ade4) == 0x0` | `ret(0x5ade4) != 0x0` |
| `bne` at 0x594c4 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `blt` at 0x594f8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x59560 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x59584 | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `blt` at 0x595bc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x595f8 | `ret(0x5ade4) == 0x0` | `ret(0x5ade4) != 0x0` |
| `bne` at 0x59604 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x59650 | `ret(0x5ba20) != 0x0` | `ret(0x5ba20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59690 | `r0` | `?` |

## `pcixResetRelease`

- `pcixResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x596a0 to 0x598a8.
  Established. The unnamed call to `0x5b208` has candidates `[sp-20]` followed by three unknown
  values (`bl` at 0x59780); its zero result takes the branch to `0x597ac` (`beq` at 0x59794).
  Another call to that target carries `[sp-20]`, `ret(0x5b944)`, unknown, `ret(0x5b944)` (`bl` at
  0x5980c). Included later calls to unnamed `0x5ba20` and `0x5b3bc` carry `[sp-20]`, `[sp-21]:1`,
  `[sp-22]:1`, `[sp-21]:1`, and `[sp-20]`, `ret(0x5ba20)`, unknown, `ret(0x5ba20)` respectively
  (`bl` at 0x59848, `bl` at 0x59888). Reset-release effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x596b0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x596b8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x596c0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x596c8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x596cc | load | 4 | `abs 0x5989c` | `-` |
| `ldr` at 0x596d0 | load | 4 | `abs 0x1490e0` | `-` |
| `ldrb` at 0x596dc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x596e8 | load | 4 | `abs 0x598a0` | `-` |
| `str` at 0x596ec | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x596f8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x596fc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x59700 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x59708 | load | 4 | `abs 0x598a4` | `-` |
| `ldr` at 0x5970c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59720 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x59728 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x5972c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x5973c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x59740 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59744 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59748 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x59758 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5975c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59770 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x59778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59788 | store | 4 | `sp -0x2c` | `ret(0x5b208)` |
| `ldr` at 0x5978c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59798 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x597a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x597a8 | store | 4 | `sp -0x28` | `ret(0x5b208)` |
| `ldr` at 0x597ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x597b4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x597bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x597c0 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x597c8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x597d4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x597d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x597e8 | store | 4 | `sp -0x24` | `ret(0x5b944)` |
| `ldr` at 0x597ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x597f8 | load | 4 | `abs 0x598a0` | `-` |
| `str` at 0x597fc | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x59804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59808 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59814 | store | 4 | `sp -0x2c` | `ret(0x5b208)` |
| `ldr` at 0x59818 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59824 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x59828 | store | 4 | `sp -0x28` | `ret(0x5b208)` |
| `ldr` at 0x5982c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x59830 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x59838 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5983c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59840 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59850 | store | 4 | `sp -0x20` | `ret(0x5ba20)` |
| `ldr` at 0x59854 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59860 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59864 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59868 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59878 | store | 4 | `sp -0x34` | `ret(0x5bc00)` |
| `ldr` at 0x59880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59884 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59890 | store | 4 | `sp -0x34` | `ret(0x5b3bc)` |
| `ldr` at 0x59894 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x59898 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59898 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59780 | `(unknown callee) 0x5b208` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x597e0 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x5980c | `(unknown callee) 0x5b208` | `[sp-20]` | `ret(0x5b944)` | `?` | `ret(0x5b944)` |
| `bl` at 0x59848 | `(unknown callee) 0x5ba20` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59870 | `(unknown callee) 0x5bc00` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59888 | `(unknown callee) 0x5b3bc` | `[sp-20]` | `ret(0x5ba20)` | `?` | `ret(0x5ba20)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x596d8 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x596e4 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x59734 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x59750 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x59794 | `ret(0x5b208) == 0x0` | `ret(0x5b208) != 0x0` |
| `bne` at 0x597a0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x597d0 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x597f4 | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `beq` at 0x59820 | `ret(0x5b208) == 0x0` | `ret(0x5b208) != 0x0` |
| `bne` at 0x5985c | `ret(0x5ba20) != 0x0` | `ret(0x5ba20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59898 | `r0` | `?` |
