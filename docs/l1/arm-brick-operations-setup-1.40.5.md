# ARM brick-operation setup in 1.40.5

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
`brickInit` and `brickInitBrickOps` have unresolved computed transfers. Their access, call and return lists cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/brick-operations-setup-facts.txt) retain all tool sections.

## `brickInit`

- `brickInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ebf0 to 0x4f4d0.
  Established. Included absolute assignments write the four-byte value `0x1` at `0x2af240` (`str` at
  0x4ec18) and byte value `0x2` at `0x148f7d` (`strb` at 0x4ec24). The early branch takes
  `ret(0xd247c) != 0x0` to `0x4eea4` and falls through when zero (`bne` at 0x4ec4c). The unnamed
  `0x1b9a0` call carries `0x2af280`, `[0x1434a8]:1`, an unknown third candidate, `[0x1434a8]:1`
  (`bl` at 0x4f464). Board identification beyond the unresolved computed edge remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ec04 | store | 4 | `sp -0x94` | `0x1` |
| `ldr` at 0x4ec10 | load | 4 | `abs 0x4f46c` | `-` |
| `str` at 0x4ec18 | store | 4 | `abs 0x2af240` | `0x1` |
| `ldr` at 0x4ec1c | load | 4 | `abs 0x4f470` | `-` |
| `strb` at 0x4ec24 | store | 1 | `abs 0x148f7d` | `0x2` |
| `strb` at 0x4ec2c | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x4ec50 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4ec54 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4ec60 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ec64 | load | 4 | `abs 0x4f474` | `-` |
| `strb` at 0x4ec80 | store | 1 | `sp -0x8e` | `?` |
| `str` at 0x4ec88 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x4ec90 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ec94 | load | 4 | `abs 0x4f478` | `-` |
| `ldr` at 0x4ecac | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ecb4 | store | 1 | `abs 0x1434a8` | `0x4c` |
| `ldr` at 0x4ecb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4ecbc | load | 4 | `abs 0x4f480` | `-` |
| `ldr` at 0x4ecd4 | load | 4 | `abs 0x4f484` | `-` |
| `strb` at 0x4ecdc | store | 1 | `abs 0x148f7c` | `0x7` |
| `ldr` at 0x4ece4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ece8 | load | 4 | `abs 0x4f488` | `-` |
| `ldr` at 0x4ed2c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4ed30 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4ed3c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ed40 | load | 4 | `abs 0x4f48c` | `-` |
| `ldr` at 0x4ed58 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ed60 | store | 1 | `abs 0x1434a8` | `0x5a` |
| `ldr` at 0x4ed68 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ed6c | load | 4 | `abs 0x4f490` | `-` |
| `ldr` at 0x4ed84 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ed8c | store | 1 | `abs 0x1434a8` | `0x7c` |
| `ldr` at 0x4ed94 | load | 4 | `abs 0x4f494` | `-` |
| `ldr` at 0x4ed98 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4edc4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4edc8 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4edd4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4edd8 | load | 4 | `abs 0x4f48c` | `-` |
| `ldr` at 0x4edf0 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4edf8 | store | 1 | `abs 0x1434a8` | `0x5a` |
| `ldr` at 0x4ee00 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ee04 | load | 4 | `abs 0x4f490` | `-` |
| `ldr` at 0x4ee1c | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ee24 | store | 1 | `abs 0x1434a8` | `0x7c` |
| `ldr` at 0x4ee2c | load | 4 | `abs 0x4f494` | `-` |
| `ldr` at 0x4ee30 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ee3c | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ee44 | store | 1 | `abs 0x1434a8` | `0x5a` |
| `ldr` at 0x4ee4c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ee50 | load | 4 | `abs 0x4f498` | `-` |
| `strb` at 0x4ee6c | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x4ee74 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ee78 | load | 4 | `abs 0x4f49c` | `-` |
| `strb` at 0x4ee94 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x4eea4 | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4eea8 | load | 1 | `abs 0x1434a8` | `-` |
| `ldrb` at 0x4eeb4 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4eeb8 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4eec0 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4eec4 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4eee4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4eee8 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `strb` at 0x4eef8 | store | 1 | `sp -0x95` | `?` |
| `ldr` at 0x4eefc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ef00 | load | 4 | `abs 0x4f4a0` | `-` |
| `ldr` at 0x4ef18 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ef20 | store | 1 | `abs 0x1434a8` | `0x44` |
| `ldr` at 0x4ef28 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ef2c | load | 4 | `abs 0x4f4a4` | `-` |
| `ldr` at 0x4ef44 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ef4c | store | 1 | `abs 0x1434a8` | `0x40` |
| `ldr` at 0x4ef54 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ef58 | load | 4 | `abs 0x4f4a8` | `-` |
| `ldr` at 0x4ef70 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4ef78 | store | 1 | `abs 0x1434a8` | `0x42` |
| `ldr` at 0x4ef80 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4ef84 | load | 4 | `abs 0x4f4ac` | `-` |
| `ldr` at 0x4ef9c | load | 4 | `abs 0x4f47c` | `-` |
| `str` at 0x4efa0 | store | 4 | `sp -0x9c` | `0x1434a8` |
| `ldr` at 0x4efb0 | load | 4 | `abs 0x4f4b0` | `-` |
| `strb` at 0x4efc8 | store | 1 | `sp -0x9d` | `0x48` |
| `strb` at 0x4efd4 | store | 1 | `sp -0x9d` | `0x46` |
| `ldrb` at 0x4efd8 | load | 1 | `sp -0x9d` | `-` |
| `ldr` at 0x4efdc | load | 4 | `sp -0x9c` | `-` |
| `strb` at 0x4efe0 | store | 1 | `[sp-156] +0x0` | `([sp-157]:1 & 0xff)` |
| `ldr` at 0x4efe8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4efec | load | 4 | `abs 0x4f4b4` | `-` |
| `ldr` at 0x4f004 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4f00c | store | 1 | `abs 0x1434a8` | `0x53` |
| `ldr` at 0x4f014 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4f018 | load | 4 | `abs 0x4f4b8` | `-` |
| `ldr` at 0x4f030 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4f038 | store | 1 | `abs 0x1434a8` | `0x4d` |
| `ldr` at 0x4f048 | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4f04c | load | 1 | `abs 0x1434a8` | `-` |
| `ldrb` at 0x4f058 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4f05c | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4f064 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4f068 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4f088 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4f08c | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4f098 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4f09c | load | 4 | `abs 0x4f4bc` | `-` |
| `ldr` at 0x4f0b4 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4f0bc | store | 1 | `abs 0x1434a8` | `0x5e` |
| `ldr` at 0x4f0c4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4f0c8 | load | 4 | `abs 0x4f4c0` | `-` |
| `ldr` at 0x4f0e0 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4f0e8 | store | 1 | `abs 0x1434a8` | `0x35` |
| `ldr` at 0x4f0f8 | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4f0fc | load | 1 | `abs 0x1434a8` | `-` |
| `ldrb` at 0x4f108 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4f10c | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4f114 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4f118 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4f138 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4f13c | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4f148 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4f14c | load | 4 | `abs 0x4f4c4` | `-` |
| `ldr` at 0x4f164 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4f16c | store | 1 | `abs 0x1434a8` | `0x4a` |
| `ldr` at 0x4f17c | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4f180 | load | 1 | `abs 0x1434a8` | `-` |
| `ldrb` at 0x4f18c | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4f190 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `strb` at 0x4f198 | store | 1 | `sp -0x8e` | `?` |
| `ldrb` at 0x4f19c | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4f1bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4f1c0 | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4f1cc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4f1d0 | load | 4 | `abs 0x4f4c4` | `-` |
| `ldr` at 0x4f1e8 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4f1f0 | store | 1 | `abs 0x1434a8` | `0x4a` |
| `ldr` at 0x4f200 | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4f204 | load | 1 | `abs 0x1434a8` | `-` |
| `ldrb` at 0x4f210 | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x4f214 | store | 1 | `sp -0x8d` | `([sp-142]:1 & 0xff)` |
| `ldrb` at 0x4f218 | load | 1 | `sp -0x8d` | `-` |
| `ldr` at 0x4f238 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4f23c | store | 4 | `sp -0x8c` | `[sp-60]` |
| `ldr` at 0x4f248 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x4f24c | load | 4 | `abs 0x4f4c8` | `-` |
| `ldr` at 0x4f264 | load | 4 | `abs 0x4f47c` | `-` |
| `strb` at 0x4f26c | store | 1 | `abs 0x1434a8` | `0x2b` |
| `ldr` at 0x4f27c | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4f280 | load | 1 | `abs 0x1434a8` | `-` |
| `ldr` at 0x4f28c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x4f298 | load | 4 | `abs 0x4f46c` | `-` |
| `str` at 0x4f2a0 | store | 4 | `abs 0x2af240` | `0x2` |
| `strb` at 0x4f2b0 | store | 1 | `sp -0x95` | `(ret(0x1f26c) & 0xff)` |
| `ldrb` at 0x4f2b4 | load | 1 | `sp -0x95` | `-` |
| `ldrls` at 0x4f2c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f440 | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4f444 | load | 1 | `abs 0x1434a8` | `-` |
| `ldr` at 0x4f454 | load | 4 | `abs 0x4f47c` | `-` |
| `ldrb` at 0x4f458 | load | 1 | `abs 0x1434a8` | `-` |
| `ldr` at 0x4f45c | load | 4 | `abs 0x4f4cc` | `-` |
| `ldmdb` at 0x4f468 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f468 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f468 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4ec0c | `(unknown callee) 0xc7ea4` | `0x6` | `r1` | `r2` | `0x1` |
| `bl` at 0x4ec40 | `(unknown callee) 0xd247c` | `0x0` | `0xff` | `sp-136` | `sp-136` |
| `bl` at 0x4ec6c | `(unknown callee) 0x1b2e0` | `[sp-60]` | `0x10b130` | `0x9` | `[sp-60]` |
| `bl` at 0x4ec9c | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b13c` | `0xd` | `ret(0x1b2e0)` |
| `bl` at 0x4ecc4 | `(unknown callee) 0x1b2e0` | `[sp-52]` | `0x10b14c` | `0xc` | `0x4c` |
| `bl` at 0x4ecf0 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b15c` | `0xd` | `ret(0x1b2e0)` |
| `bl` at 0x4ed08 | `(unknown callee) 0xd25a8` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4ed1c | `(unknown callee) 0xd247c` | `0x0` | `0xfe` | `sp-136` | `sp-136` |
| `bl` at 0x4ed48 | `(unknown callee) 0x1b2e0` | `[sp-60]` | `0x10b16c` | `0xd` | `[sp-60]` |
| `bl` at 0x4ed74 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b17c` | `0x4` | `ret(0x1b2e0)` |
| `bl` at 0x4ed9c | `(unknown callee) 0x2079c` | `0x10b184` | `[sp-140]` | `?` | `ret(0x1b2e0)` |
| `bl` at 0x4edb4 | `(unknown callee) 0xd247c` | `0x0` | `0xfd` | `sp-136` | `sp-136` |
| `bl` at 0x4ede0 | `(unknown callee) 0x1b2e0` | `[sp-60]` | `0x10b16c` | `0xd` | `[sp-60]` |
| `bl` at 0x4ee0c | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b17c` | `0x4` | `ret(0x1b2e0)` |
| `bl` at 0x4ee34 | `(unknown callee) 0x2079c` | `0x10b184` | `[sp-140]` | `?` | `ret(0x1b2e0)` |
| `bl` at 0x4ee58 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b1a0` | `0x8` | `ret(0x1b2e0)` |
| `bl` at 0x4ee80 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b1ac` | `0xd` | `ret(0x1b2e0)` |
| `bl` at 0x4eea0 | `(unknown callee) 0xd25a8` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4eed4 | `(unknown callee) 0xd247c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4ef08 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b1bc` | `0x7` | `?` |
| `bl` at 0x4ef34 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b1c4` | `0xa` | `ret(0x1b2e0)` |
| `bl` at 0x4ef60 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b1d0` | `0xd` | `ret(0x1b2e0)` |
| `bl` at 0x4ef8c | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b1e0` | `0xe` | `ret(0x1b2e0)` |
| `bl` at 0x4efb4 | `(unknown callee) 0x4f9e0` | `sp-149` | `0x1` | `0x10b1f0` | `sp-149` |
| `bl` at 0x4eff4 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b1f8` | `0x8` | `ret(0x1b2e0)` |
| `bl` at 0x4f020 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b204` | `0x4` | `ret(0x1b2e0)` |
| `bl` at 0x4f044 | `(unknown callee) 0xd25a8` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4f078 | `(unknown callee) 0xd247c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4f0a4 | `(unknown callee) 0x1b2e0` | `[sp-60]` | `0x10b20c` | `0x7` | `[sp-60]` |
| `bl` at 0x4f0d0 | `(unknown callee) 0x1b2e0` | `[sp-140]` | `0x10b214` | `0x4` | `ret(0x1b2e0)` |
| `bl` at 0x4f0f4 | `(unknown callee) 0xd25a8` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4f128 | `(unknown callee) 0xd247c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4f154 | `(unknown callee) 0x1b2e0` | `[sp-60]` | `0x10b21c` | `0xa` | `[sp-60]` |
| `bl` at 0x4f178 | `(unknown callee) 0xd25a8` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4f1ac | `(unknown callee) 0xd247c` | `0x0` | `[sp-141]:1` | `sp-136` | `[sp-141]:1` |
| `bl` at 0x4f1d8 | `(unknown callee) 0x1b2e0` | `[sp-60]` | `0x10b21c` | `0xa` | `[sp-60]` |
| `bl` at 0x4f1fc | `(unknown callee) 0xd25a8` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4f228 | `(unknown callee) 0xd247c` | `0x0` | `([sp-142]:1 & 0xff)` | `sp-136` | `([sp-142]:1 & 0xff)` |
| `bl` at 0x4f254 | `(unknown callee) 0x1b2e0` | `[sp-60]` | `0x10b228` | `0xc` | `[sp-60]` |
| `bl` at 0x4f278 | `(unknown callee) 0xd25a8` | `sp-136` | `?` | `?` | `sp-136` |
| `bl` at 0x4f2a8 | `(unknown callee) 0x1f26c` | `0x9` | `?` | `0x2af240` | `0x2` |
| `bl` at 0x4f450 | `(unknown callee) 0x1d41c` | `0x9` | `[0x1434a8]:1` | `?` | `[0x1434a8]:1` |
| `bl` at 0x4f464 | `(unknown callee) 0x1b9a0` | `0x2af280` | `[0x1434a8]:1` | `?` | `[0x1434a8]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4ec4c | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4ec5c | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4ec78 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4eca8 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ecd0 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ecfc | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ed28 | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4ed38 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4ed54 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ed80 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4edc0 | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4edd0 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4edec | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ee18 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ee64 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ee8c | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4eeb0 | `[0x1434a8]:1 != 0x3f` | `[0x1434a8]:1 == 0x3f` |
| `bne` at 0x4eee0 | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4eef0 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4ef14 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ef40 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ef6c | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4ef98 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `beq` at 0x4efc0 | `ret(0x4f9e0) == 0x0` | `ret(0x4f9e0) != 0x0` |
| `bne` at 0x4f000 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4f02c | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4f054 | `[0x1434a8]:1 != 0x3f` | `[0x1434a8]:1 == 0x3f` |
| `bne` at 0x4f084 | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4f094 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4f0b0 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4f0dc | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4f104 | `[0x1434a8]:1 != 0x3f` | `[0x1434a8]:1 == 0x3f` |
| `bne` at 0x4f134 | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4f144 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4f160 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4f188 | `[0x1434a8]:1 != 0x3f` | `[0x1434a8]:1 == 0x3f` |
| `bne` at 0x4f1b8 | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4f1c8 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4f1e4 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4f20c | `[0x1434a8]:1 != 0x3f` | `[0x1434a8]:1 == 0x3f` |
| `bne` at 0x4f234 | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x4f244 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bne` at 0x4f260 | `ret(0x1b2e0) != 0x0` | `ret(0x1b2e0) == 0x0` |
| `bne` at 0x4f288 | `[0x1434a8]:1 != 0x3f` | `[0x1434a8]:1 == 0x3f` |
| `beq` at 0x4f294 | `[sp-148] == 0x0` | `[sp-148] != 0x0` |
| `ldrls` at 0x4f2c4 | `? <= 0x59 (unsigned, 32-bit)` | `? > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f468 | `r0` | `ret(0x1b9a0)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4f2c4).

## `brickInitBrickOps`

- `brickInitBrickOps`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1b9a0 to 0x1bbdc.
  Established. The computed branch takes unsigned `r1-43 <= 0x51` to an unresolved target and falls
  through above that bound (`ldrls` at 0x1b9c4). The known path includes an unnamed `0x24618` call
  carrying `r0`, `r1`, `r2`, `r1-43` (`bl` at 0x1bbb8). The return reports `ret(0xafea4)` (`ldmdb`
  at 0x1bbd8). Selector-specific setup remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1b9b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1b9b4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x1b9b8 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x1b9c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1bbb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1bbb4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1bbd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1bbd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1bbd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1bbb8 | `(unknown callee) 0x24618` | `r0` | `r1` | `r2` | `r1-43` |
| `bl` at 0x1bbbc | `(unknown callee) 0xdaf84` | `ret(0x24618)` | `?` | `?` | `?` |
| `bl` at 0x1bbc0 | `(unknown callee) 0xe8190` | `ret(0xdaf84)` | `?` | `?` | `?` |
| `bl` at 0x1bbc4 | `(unknown callee) 0xf407c` | `ret(0xe8190)` | `?` | `?` | `?` |
| `bl` at 0x1bbc8 | `(unknown callee) 0xe9998` | `ret(0xf407c)` | `?` | `?` | `?` |
| `bl` at 0x1bbcc | `(unknown callee) 0xa38f0` | `ret(0xe9998)` | `?` | `?` | `?` |
| `bl` at 0x1bbd0 | `(unknown callee) 0xa7038` | `ret(0xa38f0)` | `?` | `?` | `?` |
| `bl` at 0x1bbd4 | `(unknown callee) 0xafea4` | `ret(0xa7038)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x1b9c4 | `r1-43 <= 0x51 (unsigned, 32-bit)` | `r1-43 > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1bbd8 | `r0` | `ret(0xafea4)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1b9c4).

## `Unknown_BrickOpsInit`

- `Unknown_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24618 to 0x246f4.
  Established. A four-byte assignment at `r0 +0x4` writes `0x24590` (`str` at 0x2463c). The byte
  assignment at `[sp-20] +0xc` writes `0x4e` (`strb` at 0x24648). Included four-byte writes at
  `[sp-20] +0xa0` and `[sp-20] +0xac` retain `0xff` (`str` at 0x24660, `str` at 0x24684). Record
  fields and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24628 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2462c | load | 4 | `abs 0x246ec` | `-` |
| `str` at 0x24630 | store | 4 | `sp -0x18` | `0x1430ec` |
| `ldr` at 0x24634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24638 | load | 4 | `abs 0x246f0` | `-` |
| `str` at 0x2463c | store | 4 | `r0 +0x4` | `0x24590` |
| `ldr` at 0x24640 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x24648 | store | 1 | `[sp-20] +0xc` | `0x4e` |
| `ldr` at 0x2464c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24654 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x24658 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24660 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x24664 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2466c | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x24670 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24678 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2467c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24684 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x24688 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24690 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x24694 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2469c | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x246a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x246a8 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x246ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x246b4 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x246b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x246c0 | store | 4 | `[sp-20] +0xc4` | `0x0` |
| `ldr` at 0x246c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x246cc | store | 4 | `[sp-20] +0xc8` | `0x0` |
| `ldr` at 0x246d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x246d8 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x246dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x246e4 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldmdb` at 0x246e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x246e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x246e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x246e8 | `r0` | `r0` |

## `Athena_BrickOpsInit`

- `Athena_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d16c to 0x2d2f0.
  Established. Included four-byte assignments retain `0x104328` at `[sp-20] +0x14` (`str` at
  0x2d1f0), `0x53f1c` at `[sp-20] +0xc4` (`str` at 0x2d22c), and `0x54128` at `[sp-20] +0xc8` (`str`
  at 0x2d238). A separate word at `[sp-20] +0x68` receives `0x2d` (`str` at 0x2d274). Pointer roles
  and the meaning of these configured values remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d17c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2d180 | load | 4 | `abs 0x2d2d0` | `-` |
| `str` at 0x2d184 | store | 4 | `sp -0x18` | `0x1432fa` |
| `ldr` at 0x2d188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d18c | load | 4 | `abs 0x2d2d4` | `-` |
| `str` at 0x2d190 | store | 4 | `r0 +0x0` | `0x2ce08` |
| `ldr` at 0x2d194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d198 | load | 4 | `abs 0x2d2d8` | `-` |
| `str` at 0x2d19c | store | 4 | `[sp-20] +0x4` | `0x2cbf8` |
| `ldr` at 0x2d1a0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2d1a8 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2d1ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1b4 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2d1b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1c0 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2d1c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1cc | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2d1d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1d8 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2d1dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1e4 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2d1e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d1ec | load | 4 | `abs 0x2d2dc` | `-` |
| `str` at 0x2d1f0 | store | 4 | `[sp-20] +0x14` | `0x104328` |
| `ldr` at 0x2d1f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1fc | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2d200 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d208 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2d20c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d214 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2d218 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d220 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2d224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d228 | load | 4 | `abs 0x2d2e0` | `-` |
| `str` at 0x2d22c | store | 4 | `[sp-20] +0xc4` | `0x53f1c` |
| `ldr` at 0x2d230 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d234 | load | 4 | `abs 0x2d2e4` | `-` |
| `str` at 0x2d238 | store | 4 | `[sp-20] +0xc8` | `0x54128` |
| `ldr` at 0x2d23c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d244 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2d248 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d250 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2d254 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d258 | load | 4 | `abs 0x2d2e8` | `-` |
| `str` at 0x2d25c | store | 4 | `[sp-20] +0xec` | `0x2c62c` |
| `ldr` at 0x2d260 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d264 | load | 4 | `abs 0x2d2ec` | `-` |
| `str` at 0x2d268 | store | 4 | `[sp-20] +0xf0` | `0x5442c` |
| `ldr` at 0x2d26c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d274 | store | 4 | `[sp-20] +0x68` | `0x2d` |
| `ldr` at 0x2d278 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d280 | store | 4 | `[sp-20] +0x6c` | `0x32` |
| `ldr` at 0x2d284 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d28c | store | 4 | `[sp-20] +0x70` | `0x37` |
| `ldr` at 0x2d290 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d298 | store | 4 | `[sp-20] +0x74` | `0x3c` |
| `ldr` at 0x2d29c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d2a4 | store | 4 | `[sp-20] +0x78` | `0x2b` |
| `ldr` at 0x2d2a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d2b0 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `ldr` at 0x2d2b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d2bc | store | 4 | `[sp-20] +0x1c` | `0x0` |
| `ldr` at 0x2d2c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d2c8 | store | 4 | `[sp-20] +0x20` | `0x0` |
| `ldmdb` at 0x2d2cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d2cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d2cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d2cc | `r0` | `r0` |

## `IP63_BrickOpsInit`

- `IP63_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x386c0 to 0x38850.
  Established. Two included four-byte assignments write `0x1` at `[sp-20] +0xb4` (`str` at 0x38780)
  and `[sp-20] +0x88` (`str` at 0x3878c). Another word at `[sp-20] +0xb0` receives `0x10eee4` (`str`
  at 0x387c8). The byte at `[sp-20] +0xc` is cleared (`strb` at 0x386fc). Board field meanings and
  pointer consumers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x386d0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x386d4 | load | 4 | `abs 0x38818` | `-` |
| `str` at 0x386d8 | store | 4 | `sp -0x18` | `0x143494` |
| `ldr` at 0x386dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x386e0 | load | 4 | `abs 0x3881c` | `-` |
| `str` at 0x386e4 | store | 4 | `r0 +0x0` | `0x37fb8` |
| `ldr` at 0x386e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x386ec | load | 4 | `abs 0x38820` | `-` |
| `str` at 0x386f0 | store | 4 | `[sp-20] +0x4` | `0x37dbc` |
| `ldr` at 0x386f4 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x386fc | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x38700 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38708 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x3870c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38714 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x38718 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38720 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x38724 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3872c | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x38730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38738 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x3873c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38744 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x38748 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38750 | store | 4 | `[sp-20] +0x6c` | `0x23` |
| `ldr` at 0x38754 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3875c | store | 4 | `[sp-20] +0x70` | `0x28` |
| `ldr` at 0x38760 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38768 | store | 4 | `[sp-20] +0x74` | `0x2d` |
| `ldr` at 0x3876c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38774 | store | 4 | `[sp-20] +0x78` | `0x1b` |
| `ldr` at 0x38778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38780 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x38784 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3878c | store | 4 | `[sp-20] +0x88` | `0x1` |
| `ldr` at 0x38790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38794 | load | 4 | `abs 0x38824` | `-` |
| `str` at 0x38798 | store | 4 | `[sp-20] +0x90` | `0x38538` |
| `ldr` at 0x3879c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387a0 | load | 4 | `abs 0x38828` | `-` |
| `str` at 0x387a4 | store | 4 | `[sp-20] +0x94` | `0x3850c` |
| `ldr` at 0x387a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387ac | load | 4 | `abs 0x3882c` | `-` |
| `str` at 0x387b0 | store | 4 | `[sp-20] +0x98` | `0x384e0` |
| `ldr` at 0x387b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387b8 | load | 4 | `abs 0x38830` | `-` |
| `str` at 0x387bc | store | 4 | `[sp-20] +0x9c` | `0x3842c` |
| `ldr` at 0x387c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387c4 | load | 4 | `abs 0x38834` | `-` |
| `str` at 0x387c8 | store | 4 | `[sp-20] +0xb0` | `0x10eee4` |
| `ldr` at 0x387cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387d0 | load | 4 | `abs 0x38838` | `-` |
| `str` at 0x387d4 | store | 4 | `[sp-20] +0xc4` | `0x383dc` |
| `ldr` at 0x387d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387dc | load | 4 | `abs 0x3883c` | `-` |
| `str` at 0x387e0 | store | 4 | `[sp-20] +0xc8` | `0x38404` |
| `ldr` at 0x387e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387e8 | load | 4 | `abs 0x38840` | `-` |
| `str` at 0x387ec | store | 4 | `[sp-20] +0xd0` | `0x38670` |
| `ldr` at 0x387f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x387f4 | load | 4 | `abs 0x38844` | `-` |
| `str` at 0x387f8 | store | 4 | `[sp-20] +0xd4` | `0x38698` |
| `ldr` at 0x387fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38800 | load | 4 | `abs 0x38848` | `-` |
| `str` at 0x38804 | store | 4 | `[sp-20] +0xec` | `0x38178` |
| `ldr` at 0x38808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3880c | load | 4 | `abs 0x3884c` | `-` |
| `str` at 0x38810 | store | 4 | `[sp-20] +0xf0` | `0x3b4f8` |
| `ldmdb` at 0x38814 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38814 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38814 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38814 | `r0` | `r0` |

## `NL4S_BrickOpsInit`

- `NL4S_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29b3c to 0x29b68.
  Established. A four-byte stack assignment at `sp -0x18` retains `0x1431f2` (`str` at 0x29b54). The
  unnamed `0x299bc` call carries `r0`, `r1`, `r2`, `0x1431f2` (`bl` at 0x29b5c); its result appears
  as `ret(0x299bc)` at the return (`ldmdb` at 0x29b60). Board setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29b4c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x29b50 | load | 4 | `abs 0x29b64` | `-` |
| `str` at 0x29b54 | store | 4 | `sp -0x18` | `0x1431f2` |
| `ldr` at 0x29b58 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x29b60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29b60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29b60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29b5c | `(unknown callee) 0x299bc` | `r0` | `r1` | `r2` | `0x1431f2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29b60 | `r0` | `ret(0x299bc)` |

## `NL4R_BrickOpsInit`

- `NL4R_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29b00 to 0x29b3c.
  Established. The included four-byte assignment at `r0 +0x0` writes `0x2988c` (`str` at 0x29b24).
  The unnamed `0x299bc` call uses `[sp-20]`, `r1`, `r0`, `0x2988c` (`bl` at 0x29b2c). Field roles
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29b10 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x29b14 | load | 4 | `abs 0x29b34` | `-` |
| `str` at 0x29b18 | store | 4 | `sp -0x18` | `0x1431ec` |
| `ldr` at 0x29b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29b20 | load | 4 | `abs 0x29b38` | `-` |
| `str` at 0x29b24 | store | 4 | `r0 +0x0` | `0x2988c` |
| `ldr` at 0x29b28 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x29b30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29b30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29b30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29b2c | `(unknown callee) 0x299bc` | `[sp-20]` | `r1` | `r0` | `0x2988c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29b30 | `r0` | `ret(0x299bc)` |

## `PA_BrickOpsInit`

- `PA_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a5b8 to 0x2a728.
  Established. The four-byte value `0x102cec` is assigned at `[sp-20] +0x14` (`str` at 0x2a63c). A
  later word at `[sp-20] +0x7c` receives `0x29da0` (`str` at 0x2a6fc). Included words at `[sp-20] +0x68` and `[sp-20] +0x78` retain `0x1e` and `0x1b` (`str` at 0x2a648, `str` at 0x2a678). These
  field roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a5c8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2a5cc | load | 4 | `abs 0x2a704` | `-` |
| `str` at 0x2a5d0 | store | 4 | `sp -0x18` | `0x143228` |
| `ldr` at 0x2a5d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a5d8 | load | 4 | `abs 0x2a708` | `-` |
| `str` at 0x2a5dc | store | 4 | `r0 +0x0` | `0x2a368` |
| `ldr` at 0x2a5e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a5e4 | load | 4 | `abs 0x2a70c` | `-` |
| `str` at 0x2a5e8 | store | 4 | `[sp-20] +0x4` | `0x2a16c` |
| `ldr` at 0x2a5ec | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2a5f4 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2a5f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a600 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2a604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a60c | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2a610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a618 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2a61c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a624 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2a628 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a630 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2a634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a638 | load | 4 | `abs 0x2a710` | `-` |
| `str` at 0x2a63c | store | 4 | `[sp-20] +0x14` | `0x102cec` |
| `ldr` at 0x2a640 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a648 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x2a64c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a654 | store | 4 | `[sp-20] +0x6c` | `0x23` |
| `ldr` at 0x2a658 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a660 | store | 4 | `[sp-20] +0x70` | `0x28` |
| `ldr` at 0x2a664 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a66c | store | 4 | `[sp-20] +0x74` | `0x2d` |
| `ldr` at 0x2a670 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a678 | store | 4 | `[sp-20] +0x78` | `0x1b` |
| `ldr` at 0x2a67c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a684 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2a688 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a690 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2a694 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a69c | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2a6a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a6a8 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2a6ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a6b0 | load | 4 | `abs 0x2a714` | `-` |
| `str` at 0x2a6b4 | store | 4 | `[sp-20] +0xc4` | `0x53f1c` |
| `ldr` at 0x2a6b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a6bc | load | 4 | `abs 0x2a718` | `-` |
| `str` at 0x2a6c0 | store | 4 | `[sp-20] +0xc8` | `0x54128` |
| `ldr` at 0x2a6c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a6cc | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2a6d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a6d8 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2a6dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a6e0 | load | 4 | `abs 0x2a71c` | `-` |
| `str` at 0x2a6e4 | store | 4 | `[sp-20] +0xec` | `0x2a3ac` |
| `ldr` at 0x2a6e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a6ec | load | 4 | `abs 0x2a720` | `-` |
| `str` at 0x2a6f0 | store | 4 | `[sp-20] +0xf0` | `0x5442c` |
| `ldr` at 0x2a6f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a6f8 | load | 4 | `abs 0x2a724` | `-` |
| `str` at 0x2a6fc | store | 4 | `[sp-20] +0x7c` | `0x29da0` |
| `ldmdb` at 0x2a700 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a700 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a700 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a700 | `r0` | `r0` |

## `Daytona_BrickOpsInit`

- `Daytona_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3645c to 0x365bc.
  Established. Included four-byte writes retain `0x35e0c` at `r0 +0x0` (`str` at 0x36480) and
  `0x35be4` at `[sp-20] +0x4` (`str` at 0x3648c). A separate word at `[sp-20] +0x14` receives
  `0x107590` (`str` at 0x364e0). The later assignment at `[sp-20] +0xec` writes `0x35e70` (`str` at
  0x36588). Setup record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3646c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x36470 | load | 4 | `abs 0x3659c` | `-` |
| `str` at 0x36474 | store | 4 | `sp -0x18` | `0x14345c` |
| `ldr` at 0x36478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3647c | load | 4 | `abs 0x365a0` | `-` |
| `str` at 0x36480 | store | 4 | `r0 +0x0` | `0x35e0c` |
| `ldr` at 0x36484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36488 | load | 4 | `abs 0x365a4` | `-` |
| `str` at 0x3648c | store | 4 | `[sp-20] +0x4` | `0x35be4` |
| `ldr` at 0x36490 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x36498 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x3649c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x364a4 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x364a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x364b0 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x364b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x364bc | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x364c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x364c8 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x364cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x364d4 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x364d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x364dc | load | 4 | `abs 0x365a8` | `-` |
| `str` at 0x364e0 | store | 4 | `[sp-20] +0x14` | `0x107590` |
| `ldr` at 0x364e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x364ec | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x364f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x364f8 | store | 4 | `[sp-20] +0x6c` | `0x23` |
| `ldr` at 0x364fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36504 | store | 4 | `[sp-20] +0x70` | `0x28` |
| `ldr` at 0x36508 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36510 | store | 4 | `[sp-20] +0x74` | `0x2d` |
| `ldr` at 0x36514 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3651c | store | 4 | `[sp-20] +0x78` | `0x1b` |
| `ldr` at 0x36520 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36528 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x3652c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36534 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x36538 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36540 | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x36544 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3654c | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x36550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36554 | load | 4 | `abs 0x365ac` | `-` |
| `str` at 0x36558 | store | 4 | `[sp-20] +0xc4` | `0x53f1c` |
| `ldr` at 0x3655c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36560 | load | 4 | `abs 0x365b0` | `-` |
| `str` at 0x36564 | store | 4 | `[sp-20] +0xc8` | `0x54128` |
| `ldr` at 0x36568 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36570 | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x36574 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3657c | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x36580 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36584 | load | 4 | `abs 0x365b4` | `-` |
| `str` at 0x36588 | store | 4 | `[sp-20] +0xec` | `0x35e70` |
| `ldr` at 0x3658c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36590 | load | 4 | `abs 0x365b8` | `-` |
| `str` at 0x36594 | store | 4 | `[sp-20] +0xf0` | `0x5442c` |
| `ldmdb` at 0x36598 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36598 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36598 | `r0` | `r0` |

## `Vortix_BrickOpsInit`

- `Vortix_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25da4 to 0x25fac.
  Established. The first unnamed `0x1b394` call carries `sp-96`, `0x100fb4`, `0x4c`, `0x100fb4`
  (`bl` at 0x25dd0). The second carries `[sp-20]+24`, `sp-96`, `0x4c`, `[sp-20]+24` (`bl` at
  0x25e9c). Included four-byte assignments write `0x1` at `[sp-20] +0x8c` (`str` at 0x25eb4) and
  `0x8` at `[sp-20] +0x88` (`str` at 0x25ec0). The record roles and callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x25db4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x25db8 | load | 4 | `abs 0x25f64` | `-` |
| `ldr` at 0x25dd4 | load | 4 | `abs 0x25f68` | `-` |
| `str` at 0x25dd8 | store | 4 | `sp -0x64` | `0x143158` |
| `ldr` at 0x25ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25de0 | load | 4 | `abs 0x25f6c` | `-` |
| `str` at 0x25de4 | store | 4 | `[sp-20] +0x0` | `0x25c98` |
| `ldr` at 0x25de8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25dec | load | 4 | `abs 0x25f70` | `-` |
| `str` at 0x25df0 | store | 4 | `[sp-20] +0x4` | `0x25988` |
| `ldr` at 0x25df4 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x25dfc | store | 1 | `[sp-20] +0xc` | `0x4e` |
| `ldr` at 0x25e00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e08 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x25e0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e14 | store | 4 | `[sp-20] +0xa0` | `0x19` |
| `ldr` at 0x25e18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e20 | store | 4 | `[sp-20] +0xa4` | `0x22` |
| `ldr` at 0x25e24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e2c | store | 4 | `[sp-20] +0xa8` | `0x2b` |
| `ldr` at 0x25e30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e38 | store | 4 | `[sp-20] +0xac` | `0x34` |
| `ldr` at 0x25e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25e40 | load | 4 | `abs 0x25f74` | `-` |
| `str` at 0x25e44 | store | 4 | `[sp-20] +0xb0` | `0x10eee4` |
| `ldr` at 0x25e48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e50 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x25e54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e5c | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x25e60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e68 | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x25e6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e74 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x25e78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25e80 | store | 4 | `[sp-20] +0x78` | `0x19` |
| `ldr` at 0x25e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ea4 | load | 4 | `abs 0x25f78` | `-` |
| `str` at 0x25ea8 | store | 4 | `[sp-20] +0x7c` | `0x25004` |
| `ldr` at 0x25eac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25eb4 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x25eb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25ec0 | store | 4 | `[sp-20] +0x88` | `0x8` |
| `ldr` at 0x25ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ec8 | load | 4 | `abs 0x25f7c` | `-` |
| `str` at 0x25ecc | store | 4 | `[sp-20] +0x90` | `0x60688` |
| `ldr` at 0x25ed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ed4 | load | 4 | `abs 0x25f80` | `-` |
| `str` at 0x25ed8 | store | 4 | `[sp-20] +0x94` | `0x60770` |
| `ldr` at 0x25edc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ee0 | load | 4 | `abs 0x25f84` | `-` |
| `str` at 0x25ee4 | store | 4 | `[sp-20] +0x98` | `0x60700` |
| `ldr` at 0x25ee8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x25ef0 | store | 4 | `[sp-20] +0xb4` | `0x4` |
| `ldr` at 0x25ef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25ef8 | load | 4 | `abs 0x25f88` | `-` |
| `str` at 0x25efc | store | 4 | `[sp-20] +0xb8` | `0x24ccc` |
| `ldr` at 0x25f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f04 | load | 4 | `abs 0x25f8c` | `-` |
| `str` at 0x25f08 | store | 4 | `[sp-20] +0xbc` | `0x24d34` |
| `ldr` at 0x25f0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f10 | load | 4 | `abs 0x25f90` | `-` |
| `str` at 0x25f14 | store | 4 | `[sp-20] +0xc0` | `0x24d9c` |
| `ldr` at 0x25f18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f1c | load | 4 | `abs 0x25f94` | `-` |
| `str` at 0x25f20 | store | 4 | `[sp-20] +0xc4` | `0x24a9c` |
| `ldr` at 0x25f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f28 | load | 4 | `abs 0x25f98` | `-` |
| `str` at 0x25f2c | store | 4 | `[sp-20] +0xc8` | `0x24b4c` |
| `ldr` at 0x25f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f34 | load | 4 | `abs 0x25f9c` | `-` |
| `str` at 0x25f38 | store | 4 | `[sp-20] +0xd0` | `0x24c20` |
| `ldr` at 0x25f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f40 | load | 4 | `abs 0x25fa0` | `-` |
| `str` at 0x25f44 | store | 4 | `[sp-20] +0xd4` | `0x24c98` |
| `ldr` at 0x25f48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f4c | load | 4 | `abs 0x25fa4` | `-` |
| `str` at 0x25f50 | store | 4 | `[sp-20] +0xec` | `0x253c4` |
| `ldr` at 0x25f54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25f58 | load | 4 | `abs 0x25fa8` | `-` |
| `str` at 0x25f5c | store | 4 | `[sp-20] +0xf0` | `0x25664` |
| `ldmdb` at 0x25f60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x25f60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x25f60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25dd0 | `(unknown callee) 0x1b394` | `sp-96` | `0x100fb4` | `0x4c` | `0x100fb4` |
| `bl` at 0x25e9c | `(unknown callee) 0x1b394` | `[sp-20]+24` | `sp-96` | `0x4c` | `[sp-20]+24` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x25f60 | `r0` | `ret(0x1b394)` |

## `Dorado_BrickOpsInit`

- `Dorado_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2efd8 to 0x2f1e4.
  Established. The byte at `[sp-20] +0xc` receives `0x52` (`strb` at 0x2f014). Included four-byte
  assignments retain `0x13` at `[sp-20] +0xa0` (`str` at 0x2f02c), `0x32e9c` at `[sp-20] +0xfc`
  (`str` at 0x2f0bc), and `0x32718` at `[sp-20] +0x80` (`str` at 0x2f0c8). A separate word at
  `[sp-20] +0x88` is set to `0x2` (`str` at 0x2f0e0). Device roles and record layout meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2efe8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2efec | load | 4 | `abs 0x2f190` | `-` |
| `str` at 0x2eff0 | store | 4 | `sp -0x18` | `0x14332c` |
| `ldr` at 0x2eff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2eff8 | load | 4 | `abs 0x2f194` | `-` |
| `str` at 0x2effc | store | 4 | `r0 +0x0` | `0x2ef80` |
| `ldr` at 0x2f000 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f004 | load | 4 | `abs 0x2f198` | `-` |
| `str` at 0x2f008 | store | 4 | `[sp-20] +0x4` | `0x2ecfc` |
| `ldr` at 0x2f00c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2f014 | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x2f018 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f020 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2f024 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f02c | store | 4 | `[sp-20] +0xa0` | `0x13` |
| `ldr` at 0x2f030 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f038 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2f03c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f044 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2f048 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f050 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2f054 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f058 | load | 4 | `abs 0x2f19c` | `-` |
| `str` at 0x2f05c | store | 4 | `[sp-20] +0x14` | `0x1050bc` |
| `ldr` at 0x2f060 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f064 | load | 4 | `abs 0x2f1a0` | `-` |
| `str` at 0x2f068 | store | 4 | `[sp-20] +0xb0` | `0x10eee4` |
| `ldr` at 0x2f06c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f070 | load | 4 | `abs 0x2f1a4` | `-` |
| `str` at 0x2f074 | store | 4 | `[sp-20] +0x7c` | `0x2da88` |
| `ldr` at 0x2f078 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f080 | store | 4 | `[sp-20] +0x68` | `0x47` |
| `ldr` at 0x2f084 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f08c | store | 4 | `[sp-20] +0x6c` | `0x48` |
| `ldr` at 0x2f090 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f098 | store | 4 | `[sp-20] +0x70` | `0x49` |
| `ldr` at 0x2f09c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f0a4 | store | 4 | `[sp-20] +0x74` | `0x4b` |
| `ldr` at 0x2f0a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f0b0 | store | 4 | `[sp-20] +0x78` | `0x45` |
| `ldr` at 0x2f0b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f0b8 | load | 4 | `abs 0x2f1a8` | `-` |
| `str` at 0x2f0bc | store | 4 | `[sp-20] +0xfc` | `0x32e9c` |
| `ldr` at 0x2f0c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f0c4 | load | 4 | `abs 0x2f1ac` | `-` |
| `str` at 0x2f0c8 | store | 4 | `[sp-20] +0x80` | `0x32718` |
| `ldr` at 0x2f0cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f0d4 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x2f0d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f0e0 | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x2f0e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f0e8 | load | 4 | `abs 0x2f1b0` | `-` |
| `str` at 0x2f0ec | store | 4 | `[sp-20] +0x90` | `0x60688` |
| `ldr` at 0x2f0f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f0f4 | load | 4 | `abs 0x2f1b4` | `-` |
| `str` at 0x2f0f8 | store | 4 | `[sp-20] +0x94` | `0x60770` |
| `ldr` at 0x2f0fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f100 | load | 4 | `abs 0x2f1b8` | `-` |
| `str` at 0x2f104 | store | 4 | `[sp-20] +0x98` | `0x60700` |
| `ldr` at 0x2f108 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2f110 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x2f114 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f118 | load | 4 | `abs 0x2f1bc` | `-` |
| `str` at 0x2f11c | store | 4 | `[sp-20] +0xb8` | `0x2d8b8` |
| `ldr` at 0x2f120 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f124 | load | 4 | `abs 0x2f1c0` | `-` |
| `str` at 0x2f128 | store | 4 | `[sp-20] +0xbc` | `0x2d908` |
| `ldr` at 0x2f12c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f130 | load | 4 | `abs 0x2f1c4` | `-` |
| `str` at 0x2f134 | store | 4 | `[sp-20] +0xc0` | `0x2d958` |
| `ldr` at 0x2f138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f13c | load | 4 | `abs 0x2f1c8` | `-` |
| `str` at 0x2f140 | store | 4 | `[sp-20] +0xc4` | `0x2d570` |
| `ldr` at 0x2f144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f148 | load | 4 | `abs 0x2f1cc` | `-` |
| `str` at 0x2f14c | store | 4 | `[sp-20] +0xc8` | `0x2d618` |
| `ldr` at 0x2f150 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f154 | load | 4 | `abs 0x2f1d0` | `-` |
| `str` at 0x2f158 | store | 4 | `[sp-20] +0xcc` | `0x2d6bc` |
| `ldr` at 0x2f15c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f160 | load | 4 | `abs 0x2f1d4` | `-` |
| `str` at 0x2f164 | store | 4 | `[sp-20] +0xd0` | `0x2d818` |
| `ldr` at 0x2f168 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f16c | load | 4 | `abs 0x2f1d8` | `-` |
| `str` at 0x2f170 | store | 4 | `[sp-20] +0xd4` | `0x2d868` |
| `ldr` at 0x2f174 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f178 | load | 4 | `abs 0x2f1dc` | `-` |
| `str` at 0x2f17c | store | 4 | `[sp-20] +0xec` | `0x2e550` |
| `ldr` at 0x2f180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2f184 | load | 4 | `abs 0x2f1e0` | `-` |
| `str` at 0x2f188 | store | 4 | `[sp-20] +0xf0` | `0x2df80` |
| `ldmdb` at 0x2f18c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2f18c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2f18c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2f18c | `r0` | `r0` |

## `Gemini_BrickOpsInit`

- `Gemini_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x276ec to 0x27898.
  Established. Included four-byte writes assign `0xf` at `[sp-20] +0xa0` (`str` at 0x27740),
  `0x101c54` at `[sp-20] +0x14` (`str` at 0x27770), and `0x266b4` at `[sp-20] +0x7c` (`str` at
  0x27788). The word at `[sp-20] +0xb4` is set to `0x1` (`str` at 0x2780c). The assigned values do
  not establish the record field roles, which remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x276fc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x27700 | load | 4 | `abs 0x2785c` | `-` |
| `str` at 0x27704 | store | 4 | `sp -0x18` | `0x1431cc` |
| `ldr` at 0x27708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2770c | load | 4 | `abs 0x27860` | `-` |
| `str` at 0x27710 | store | 4 | `r0 +0x0` | `0x27694` |
| `ldr` at 0x27714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27718 | load | 4 | `abs 0x27864` | `-` |
| `str` at 0x2771c | store | 4 | `[sp-20] +0x4` | `0x27438` |
| `ldr` at 0x27720 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x27728 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2772c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27734 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x27738 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27740 | store | 4 | `[sp-20] +0xa0` | `0xf` |
| `ldr` at 0x27744 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2774c | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x27750 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27758 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2775c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27764 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x27768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2776c | load | 4 | `abs 0x27868` | `-` |
| `str` at 0x27770 | store | 4 | `[sp-20] +0x14` | `0x101c54` |
| `ldr` at 0x27774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27778 | load | 4 | `abs 0x2786c` | `-` |
| `str` at 0x2777c | store | 4 | `[sp-20] +0xb0` | `0x10eee4` |
| `ldr` at 0x27780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27784 | load | 4 | `abs 0x27870` | `-` |
| `str` at 0x27788 | store | 4 | `[sp-20] +0x7c` | `0x266b4` |
| `ldr` at 0x2778c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27794 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x27798 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x277a0 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x277a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x277ac | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x277b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x277b8 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x277bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x277c4 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x277c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x277d0 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x277d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x277dc | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x277e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x277e4 | load | 4 | `abs 0x27874` | `-` |
| `str` at 0x277e8 | store | 4 | `[sp-20] +0x90` | `0x60688` |
| `ldr` at 0x277ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x277f0 | load | 4 | `abs 0x27878` | `-` |
| `str` at 0x277f4 | store | 4 | `[sp-20] +0x94` | `0x60770` |
| `ldr` at 0x277f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x277fc | load | 4 | `abs 0x2787c` | `-` |
| `str` at 0x27800 | store | 4 | `[sp-20] +0x98` | `0x60700` |
| `ldr` at 0x27804 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2780c | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x27810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27814 | load | 4 | `abs 0x27880` | `-` |
| `str` at 0x27818 | store | 4 | `[sp-20] +0xc4` | `0x263c0` |
| `ldr` at 0x2781c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27820 | load | 4 | `abs 0x27884` | `-` |
| `str` at 0x27824 | store | 4 | `[sp-20] +0xc8` | `0x26470` |
| `ldr` at 0x27828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2782c | load | 4 | `abs 0x27888` | `-` |
| `str` at 0x27830 | store | 4 | `[sp-20] +0xd0` | `0x26534` |
| `ldr` at 0x27834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27838 | load | 4 | `abs 0x2788c` | `-` |
| `str` at 0x2783c | store | 4 | `[sp-20] +0xd4` | `0x26584` |
| `ldr` at 0x27840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27844 | load | 4 | `abs 0x27890` | `-` |
| `str` at 0x27848 | store | 4 | `[sp-20] +0xec` | `0x26c84` |
| `ldr` at 0x2784c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27850 | load | 4 | `abs 0x27894` | `-` |
| `str` at 0x27854 | store | 4 | `[sp-20] +0xf0` | `0x26b98` |
| `ldmdb` at 0x27858 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27858 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27858 | `r0` | `r0` |

## `IP67_BrickOpsInit`

- `IP67_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x341f0 to 0x343e4.
  Established. The unnamed `0x1b394` call carries `0x2af298`, `0x105f20`, `0x10`, `0x337d0` (`bl` at
  0x342a8). A byte assignment at `[sp-20] +0xc` writes `0x54` (`strb` at 0x3422c). Included
  four-byte assignments retain `0x9` at `[sp-20] +0xa0` (`str` at 0x34244) and `0xa` at `[sp-20] +0xa4` (`str` at 0x34250). Callee effects and board field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34200 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x34204 | load | 4 | `abs 0x34394` | `-` |
| `str` at 0x34208 | store | 4 | `sp -0x18` | `0x143418` |
| `ldr` at 0x3420c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34210 | load | 4 | `abs 0x34398` | `-` |
| `str` at 0x34214 | store | 4 | `r0 +0x0` | `0x33fb8` |
| `ldr` at 0x34218 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3421c | load | 4 | `abs 0x3439c` | `-` |
| `str` at 0x34220 | store | 4 | `[sp-20] +0x4` | `0x34000` |
| `ldr` at 0x34224 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x3422c | store | 1 | `[sp-20] +0xc` | `0x54` |
| `ldr` at 0x34230 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34238 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x3423c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34244 | store | 4 | `[sp-20] +0xa0` | `0x9` |
| `ldr` at 0x34248 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34250 | store | 4 | `[sp-20] +0xa4` | `0xa` |
| `ldr` at 0x34254 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3425c | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x34260 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34268 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x3426c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34270 | load | 4 | `abs 0x343a0` | `-` |
| `str` at 0x34274 | store | 4 | `[sp-20] +0x14` | `0x10661c` |
| `ldr` at 0x34278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3427c | load | 4 | `abs 0x343a4` | `-` |
| `str` at 0x34280 | store | 4 | `[sp-20] +0xb0` | `0x10eee4` |
| `ldr` at 0x34284 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34288 | load | 4 | `abs 0x343a8` | `-` |
| `str` at 0x3428c | store | 4 | `[sp-20] +0x84` | `0x5f524` |
| `ldr` at 0x34290 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34294 | load | 4 | `abs 0x343ac` | `-` |
| `str` at 0x34298 | store | 4 | `[sp-20] +0x7c` | `0x337d0` |
| `ldr` at 0x3429c | load | 4 | `abs 0x343b0` | `-` |
| `ldr` at 0x342a0 | load | 4 | `abs 0x343b4` | `-` |
| `ldr` at 0x342ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x342b4 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x342b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x342c0 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x342c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x342cc | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x342d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x342d8 | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x342dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x342e4 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x342e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342ec | load | 4 | `abs 0x343b8` | `-` |
| `str` at 0x342f0 | store | 4 | `[sp-20] +0xfc` | `0x32e9c` |
| `ldr` at 0x342f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x342f8 | load | 4 | `abs 0x343bc` | `-` |
| `str` at 0x342fc | store | 4 | `[sp-20] +0x80` | `0x32718` |
| `ldr` at 0x34300 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34308 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x3430c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34314 | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x34318 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3431c | load | 4 | `abs 0x343c0` | `-` |
| `str` at 0x34320 | store | 4 | `[sp-20] +0x90` | `0x60688` |
| `ldr` at 0x34324 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34328 | load | 4 | `abs 0x343c4` | `-` |
| `str` at 0x3432c | store | 4 | `[sp-20] +0x94` | `0x60770` |
| `ldr` at 0x34330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34334 | load | 4 | `abs 0x343c8` | `-` |
| `str` at 0x34338 | store | 4 | `[sp-20] +0x98` | `0x60700` |
| `ldr` at 0x3433c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34344 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x34348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3434c | load | 4 | `abs 0x343cc` | `-` |
| `str` at 0x34350 | store | 4 | `[sp-20] +0xc4` | `0x334a0` |
| `ldr` at 0x34354 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34358 | load | 4 | `abs 0x343d0` | `-` |
| `str` at 0x3435c | store | 4 | `[sp-20] +0xc8` | `0x33528` |
| `ldr` at 0x34360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34364 | load | 4 | `abs 0x343d4` | `-` |
| `str` at 0x34368 | store | 4 | `[sp-20] +0xd0` | `0x3364c` |
| `ldr` at 0x3436c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34370 | load | 4 | `abs 0x343d8` | `-` |
| `str` at 0x34374 | store | 4 | `[sp-20] +0xd4` | `0x3369c` |
| `ldr` at 0x34378 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3437c | load | 4 | `abs 0x343dc` | `-` |
| `str` at 0x34380 | store | 4 | `[sp-20] +0xec` | `0x33c58` |
| `ldr` at 0x34384 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34388 | load | 4 | `abs 0x343e0` | `-` |
| `str` at 0x3438c | store | 4 | `[sp-20] +0xf0` | `0x33b2c` |
| `ldmdb` at 0x34390 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34390 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x342a8 | `(unknown callee) 0x1b394` | `0x2af298` | `0x105f20` | `0x10` | `0x337d0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34390 | `r0` | `ret(0x1b394)` |

## `brickSetType`

- `brickSetType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f4f0 to 0x4f51c.
  Established. The byte stack assignment at `sp -0x11` retains `(r0 & 0xff)` (`strb` at 0x4f504).
  The later byte write at absolute `0x1434a8` has value `((r0 & 0xff) & 0xff)` (`strb` at 0x4f510).
  The reported return preserves `r0` (`ldmdb` at 0x4f514). Type encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4f504 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x4f508 | load | 4 | `abs 0x4f518` | `-` |
| `ldrb` at 0x4f50c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4f510 | store | 1 | `abs 0x1434a8` | `((r0 & 0xff) & 0xff)` |
| `ldmdb` at 0x4f514 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f514 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f514 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f514 | `r0` | `r0` |

## `brickSetSubType`

- `brickSetSubType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f53c to 0x4f568.
  Established. A byte write at absolute `0x148f7c` retains `((r0 & 0xff) & 0xff)` (`strb` at
  0x4f55c); its source byte is read from `sp -0x11` (`ldrb` at 0x4f558). The return value is the
  entry `r0` (`ldmdb` at 0x4f560). Subtype encoding and consumers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4f550 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x4f554 | load | 4 | `abs 0x4f564` | `-` |
| `ldrb` at 0x4f558 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4f55c | store | 1 | `abs 0x148f7c` | `((r0 & 0xff) & 0xff)` |
| `ldmdb` at 0x4f560 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f560 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f560 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f560 | `r0` | `r0` |
