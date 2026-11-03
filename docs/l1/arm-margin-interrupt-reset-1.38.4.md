# ARM multi-device margin, interrupt and reset paths in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
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
The multi-device margin initializer has an unresolved computed transfer. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/margin-interrupt-reset-facts.txt) retain all tool sections.

## `power_margin_multi_init`

- `power_margin_multi_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ea10 to 0x3f22c.
  Established. An initial word store clears `[sp+4] +0x0` (`str` at 0x3ea38). Later word stores
  write `[sp-28]`, `[sp-32]`, and `[sp-36]` at `[sp-24] +0x0`, `+0x4`, and `+0x8`, respectively
  (`str` at 0x3ea54, `str` at 0x3ea60, `str` at 0x3ea6c). The computed transfer takes an unresolved
  edge for `[[[sp-24]+12]+8]-1 <= 0x4` unsigned and falls through above that bound (`ldrls` at
  0x3eb94). A known-edge call to `0x4097c` has `[sp-28]`, `[[sp-24]+12]`, `[[sp-24]+38]:1`,
  `[[sp-24]+38]:1` (`bl` at 0x3f1d0). Excluded selector effects, physical margin changes and the
  joined r0 value remain unknown (`ldmdb` at 0x3f200).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ea20 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3ea24 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3ea28 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x3ea2c | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x3ea30 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x3ea38 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `ldr` at 0x3ea3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ea4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ea50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ea54 | store | 4 | `[sp-24] +0x0` | `[sp-28]` |
| `ldr` at 0x3ea58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ea5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ea60 | store | 4 | `[sp-24] +0x4` | `[sp-32]` |
| `ldr` at 0x3ea64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ea68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ea6c | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0x3ea70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ea74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ea7c | store | 4 | `[sp-24] +0x18` | `0x4` |
| `str` at 0x3ea84 | store | 4 | `[sp-24] +0x14` | `0x4` |
| `ldr` at 0x3ea88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ea8c | load | 4 | `abs 0x3f204` | `-` |
| `str` at 0x3ea90 | store | 4 | `[sp-24] +0x1c` | `0x0` |
| `ldr` at 0x3ea94 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3ea9c | store | 1 | `[sp-24] +0x25` | `0x0` |
| `ldr` at 0x3eaa0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3eaa8 | store | 1 | `[sp-24] +0x27` | `0x0` |
| `ldr` at 0x3eaac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3eab4 | store | 4 | `[sp-24] +0x30` | `0x1` |
| `ldr` at 0x3eab8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3eac0 | store | 1 | `[sp-24] +0x2c` | `0x2` |
| `ldr` at 0x3eac4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3eacc | store | 1 | `[sp-24] +0x2d` | `0x6` |
| `ldr` at 0x3ead0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ead4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ead8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3eae0 | store | 4 | `[sp-24] +0xc` | `ret(0x405a0)` |
| `ldr` at 0x3eae4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eaf0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3eafc | store | 4 | `sp -0x40` | `ret(0x4063c)` |
| `ldr` at 0x3eb04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3eb10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eb14 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eb18 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3eb24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eb28 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eb2c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3eb38 | load | 4 | `abs 0x3f208` | `-` |
| `str` at 0x3eb3c | store | 4 | `sp -0x40` | `0xfffff9b8` |
| `ldr` at 0x3eb44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eb48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3eb4c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x3eb54 | store | 4 | `sp -0x34` | `ret(0x40e88)` |
| `ldr` at 0x3eb58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eb5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3eb60 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eb6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eb70 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldrb` at 0x3eb74 | load | 1 | `[[sp-24]+12] +0x6` | `-` |
| `ldr` at 0x3eb80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eb84 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eb88 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldrls` at 0x3eb94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3edc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3edcc | load | 4 | `abs 0x3f210` | `-` |
| `ldr` at 0x3edd0 | load | 4 | `abs 0x3f214` | `-` |
| `str` at 0x3edfc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3ee00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ee08 | load | 4 | `abs 0x3f210` | `-` |
| `ldr` at 0x3ee0c | load | 4 | `abs 0x3f214` | `-` |
| `ldr` at 0x3ee18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3ee28 | load | 4 | `abs 0x3f218` | `-` |
| `str` at 0x3ee2c | store | 4 | `(unknown)` | `0x1085d0` |
| `str` at 0x3ee30 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x3ee34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee38 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3ee3c | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `str` at 0x3ee40 | store | 4 | `(unknown)` | `[[[sp-24]+12]+8]` |
| `ldr` at 0x3ee48 | load | 4 | `abs 0x3f21c` | `-` |
| `ldr` at 0x3ee4c | load | 4 | `abs 0x3f220` | `-` |
| `ldr` at 0x3ee5c | load | 4 | `abs 0x3f224` | `-` |
| `str` at 0x3ee60 | store | 4 | `sp -0x40` | `0xfffff9be` |
| `ldr` at 0x3ee68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ee74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ee80 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3ee88 | store | 1 | `[sp-24] +0x26` | `(ret(0x40498) & 0xff)` |
| `ldr` at 0x3ee8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ee90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ee94 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eea4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3eea8 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3eeac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3eeb0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3eebc | load | 4 | `sp +0x4` | `-` |
| `str` at 0x3eec4 | store | 4 | `[sp+4] +0x0` | `0x1` |
| `ldr` at 0x3eec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3eecc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3eed0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3eed4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3eed8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x3eee8 | store | 4 | `sp -0x40` | `ret(0x4097c)` |
| `ldr` at 0x3eef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3eef4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ef00 | store | 4 | `sp -0x2c` | `ret(0x3db38)` |
| `ldr` at 0x3ef04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ef10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ef14 | load | 4 | `abs 0x3f208` | `-` |
| `ldr` at 0x3ef20 | load | 4 | `abs 0x3f228` | `-` |
| `str` at 0x3ef24 | store | 4 | `sp -0x40` | `0xfffff9bd` |
| `ldr` at 0x3ef2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ef34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ef3c | store | 4 | `[sp-24] +0x20` | `ret(0x40350)` |
| `ldr` at 0x3ef40 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ef44 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0x3ef48 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3ef54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3ef60 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3ef70 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3ef80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ef8c | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3ef94 | store | 1 | `[sp-24] +0x26` | `(ret(0x40470) & 0xff)` |
| `ldr` at 0x3ef98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ef9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3efa0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3efa8 | store | 4 | `[sp-24] +0x20` | `ret(0x401f0)` |
| `ldr` at 0x3efb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3efb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3efb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3efbc | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3efc4 | store | 1 | `[sp-24] +0x26` | `(ret(0x40498) & 0xff)` |
| `ldr` at 0x3efcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3efd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3efd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3efd8 | load | 4 | `[sp-24] +0xc` | `-` |
| `strb` at 0x3efe0 | store | 1 | `[sp-24] +0x26` | `(ret(0x404c0) & 0xff)` |
| `ldr` at 0x3efe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3efe8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3efec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3eff4 | store | 4 | `[sp-24] +0x20` | `ret(0x402a0)` |
| `ldr` at 0x3effc | load | 4 | `abs 0x3f20c` | `-` |
| `str` at 0x3f000 | store | 4 | `sp -0x40` | `0xfffff9bc` |
| `ldr` at 0x3f008 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f00c | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f01c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3f020 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3f02c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f030 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x3f034 | load | 4 | `[[sp-24]+12] +0x8` | `-` |
| `ldr` at 0x3f044 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f048 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f04c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f050 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x3f068 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3f06c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f070 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f078 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x3f090 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x3f094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f098 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x3f0a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f0a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f0ac | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x3f0b0 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3f0bc | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x3f0c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f0c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f0c8 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f0cc | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x3f0d8 | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x3f0e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f0e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f0e8 | load | 1 | `[sp-24] +0x25` | `-` |
| `ldr` at 0x3f0ec | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3f0f8 | store | 1 | `[sp-24] +0x29` | `?` |
| `ldr` at 0x3f0fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f100 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f104 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f108 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0x3f114 | store | 1 | `[sp-24] +0x28` | `?` |
| `ldr` at 0x3f11c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f120 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f124 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f128 | load | 1 | `[sp-24] +0x25` | `-` |
| `str` at 0x3f130 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x3f134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f138 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f13c | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x3f140 | store | 1 | `[sp-24] +0x29` | `([[sp-24]+37]:1 & 0xff)` |
| `ldr` at 0x3f144 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f148 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f14c | load | 1 | `[sp-24] +0x24` | `-` |
| `strb` at 0x3f150 | store | 1 | `[sp-24] +0x28` | `([[sp-24]+36]:1 & 0xff)` |
| `ldr` at 0x3f154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f158 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f15c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f160 | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x3f164 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f168 | load | 1 | `[sp-24] +0x25` | `-` |
| `ldrb` at 0x3f180 | load | 1 | `[sp-24] +0x25` | `-` |
| `strb` at 0x3f188 | store | 1 | `[sp-24] +0x26` | `?` |
| `ldr` at 0x3f18c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3f194 | store | 1 | `[sp-24] +0x2d` | `0x2` |
| `ldr` at 0x3f198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f19c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f1a0 | load | 1 | `[sp-24] +0x29` | `-` |
| `strb` at 0x3f1a4 | store | 1 | `[sp-24] +0x2b` | `([[sp-24]+41]:1 & 0xff)` |
| `ldr` at 0x3f1a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f1ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f1b0 | load | 1 | `[sp-24] +0x28` | `-` |
| `strb` at 0x3f1b4 | store | 1 | `[sp-24] +0x2a` | `([[sp-24]+40]:1 & 0xff)` |
| `ldr` at 0x3f1b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f1bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3f1c0 | load | 1 | `[sp-24] +0x26` | `-` |
| `ldr` at 0x3f1c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f1c8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x3f1d8 | store | 4 | `sp -0x2c` | `ret(0x4097c)` |
| `ldr` at 0x3f1dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f1e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3f1ec | store | 4 | `sp -0x40` | `ret(0x4097c)` |
| `str` at 0x3f1f8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x3f1fc | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0x3f200 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3f200 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3f200 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3f200 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ea48 | `(unknown callee) 0x1b43c` | `[sp-24]` | `0x0` | `0x4` | `0x0` |
| `bl` at 0x3eadc | `(unknown callee) 0x405a0` | `[sp-28]` | `[sp-32]` | `[sp-24]` | `0x6` |
| `bl` at 0x3eaf4 | `(unknown callee) 0x4063c` | `[sp-28]` | `?` | `?` | `[[sp-24]+12]` |
| `bl` at 0x3eb50 | `(unknown callee) 0x40e88` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3eb68 | `(unknown callee) 0x40ec0` | `[sp-28]` | `[[sp-24]+12]` | `0x5` | `[sp-24]` |
| `bl` at 0x3edd4 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3ede0 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x3ee10 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3ee20 | `(unknown callee) 0x1b508` | `[sp-56]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x3ee54 | `(unknown callee) 0x21e28` | `0x9` | `0x1082ec` | `0x6fd` | `0x0` |
| `bl` at 0x3ee84 | `(unknown callee) 0x40498` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3ee9c | `(unknown callee) 0x40ec0` | `[sp-28]` | `[[sp-24]+12]` | `0x0` | `[sp-24]` |
| `bl` at 0x3eeb8 | `(unknown callee) 0x40e50` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x3eee0 | `(unknown callee) 0x4097c` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |
| `bl` at 0x3eef8 | `(unknown callee) 0x3db38` | `[sp-28]` | `[sp-32]` | `?` | `[sp-36]` |
| `bl` at 0x3ef38 | `(unknown callee) 0x40350` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x3ef90 | `(unknown callee) 0x40470` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3efa4 | `(unknown callee) 0x401f0` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x3efc0 | `(unknown callee) 0x40498` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3efdc | `(unknown callee) 0x404c0` | `[sp-28]` | `[[sp-24]+12]` | `?` | `[sp-24]` |
| `bl` at 0x3eff0 | `(unknown callee) 0x402a0` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x3f1d0 | `(unknown callee) 0x4097c` | `[sp-28]` | `[[sp-24]+12]` | `[[sp-24]+38]:1` | `[[sp-24]+38]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3eaec | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `bne` at 0x3eb0c | `[sp-36] != 0x6` | `[sp-36] == 0x6` |
| `beq` at 0x3eb20 | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x3eb34 | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `beq` at 0x3eb7c | `[[[sp-24]+12]+6]:1 == 0xff` | `[[[sp-24]+12]+6]:1 != 0xff` |
| `ldrls` at 0x3eb94 | `[[[sp-24]+12]+8]-1 <= 0x4 (unsigned, 32-bit)` | `[[[sp-24]+12]+8]-1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0x3ee70 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x3ef0c | `ret(0x3db38) == 0x1` | `ret(0x3db38) != 0x1` |
| `beq` at 0x3ef1c | `ret(0x3db38) == 0xfffff9b8` | `ret(0x3db38) != 0xfffff9b8` |
| `beq` at 0x3ef50 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bgt` at 0x3ef5c | `[sp-36] > 0x2 (signed, 32-bit)` | `[sp-36] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x3ef68 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0x3ef78 | `[sp-36] == 0x3` | `[sp-36] != 0x3` |
| `beq` at 0x3f014 | `[[sp-24]+38]:1 == 0x0` | `[[sp-24]+38]:1 != 0x0` |
| `beq` at 0x3f028 | `[[[sp-24]+12]+8] == 0x1` | `[[[sp-24]+12]+8] != 0x1` |
| `beq` at 0x3f03c | `[[[sp-24]+12]+8] == 0x2` | `[[[sp-24]+12]+8] != 0x2` |
| `ble` at 0x3f0a0 | `[[sp-24]+48] <= 0x0 (signed, 32-bit)` | `[[sp-24]+48] > 0x0 (signed, 32-bit)` |
| `beq` at 0x3f1e4 | `ret(0x4097c) == 0x0` | `ret(0x4097c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3f200 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3eb94).

## `power_margin_multi`

- `power_margin_multi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f22c to 0x3fc64.
  Established. The call to `0x3fcf8` carries unknown, unknown, `0x0`, unknown (`bl` at 0x3f2a4); its
  zero result takes the continuation and nonzero falls through to save that result as a word at `sp
  -0x2c` (`beq` at 0x3f2b8, `str` at 0x3f2c0). Later byte stores include `([[sp-28]+40]:1 & 0xff)`
  at `[sp-28] +0x26`, `([[sp-28]+38]:1 & 0xff)` at `[sp-28] +0x29`, and `0x2` at `[sp-28] +0x2d`
  (`strb` at 0x3f530, `strb` at 0x3f888, `strb` at 0x3fbb0). The final call to `0x4097c` uses
  `[[sp-28]]`, `[[sp-28]+12]`, `[[sp-28]+38]:1`, `[[sp-28]+38]:1` (`bl` at 0x3fbd0). The search
  policy, physical voltage units and returned r0 value remain unproven (`ldmdb` at 0x3fc20).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f23c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3f240 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0x3f244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f250 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f254 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0x3f260 | load | 4 | `abs 0x3fc24` | `-` |
| `str` at 0x3f264 | store | 4 | `sp -0x40` | `0x10875c` |
| `ldr` at 0x3f26c | load | 4 | `abs 0x3fc28` | `-` |
| `ldr` at 0x3f270 | load | 4 | `abs 0x3fc2c` | `-` |
| `ldr` at 0x3f27c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f280 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3f284 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3f288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f28c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f290 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f298 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f29c | load | 4 | `(unknown)` | `-` |
| `str` at 0x3f2ac | store | 4 | `sp -0x24` | `ret(0x3fcf8)` |
| `ldr` at 0x3f2b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f2bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3f2c0 | store | 4 | `sp -0x2c` | `ret(0x3fcf8)` |
| `ldr` at 0x3f2c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f2d0 | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0x3f2d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f2d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f2dc | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f2e0 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f2f8 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f2fc | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f308 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f30c | load | 4 | `abs 0x3fc34` | `-` |
| `ldr` at 0x3f310 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f318 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f320 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f328 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f32c | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f330 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f348 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f34c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f350 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f368 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f36c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f380 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f384 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f390 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f394 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f3a4 | load | 4 | `abs 0x3fc38` | `-` |
| `ldr` at 0x3f3a8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f3b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f3c4 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3f3c8 | load | 1 | `[sp-28] +0x18` | `-` |
| `strb` at 0x3f3d0 | store | 1 | `[sp-28] +0x26` | `?` |
| `ldr` at 0x3f3d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f3dc | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f3e8 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f3ec | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f404 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f408 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f41c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f420 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f424 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f428 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3f438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f43c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f440 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f444 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f458 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f45c | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f468 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f46c | load | 4 | `abs 0x3fc3c` | `-` |
| `ldr` at 0x3f470 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f478 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f480 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f488 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f48c | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f490 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f4a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f4ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f4b0 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f4c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f4cc | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f4d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f4d4 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f4dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f4e0 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3f4ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f4f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f4f4 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldrb` at 0x3f4f8 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0x3f504 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f508 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f518 | load | 4 | `abs 0x3fc40` | `-` |
| `ldr` at 0x3f51c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f524 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f528 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f52c | load | 1 | `[sp-28] +0x28` | `-` |
| `strb` at 0x3f530 | store | 1 | `[sp-28] +0x26` | `([[sp-28]+40]:1 & 0xff)` |
| `ldr` at 0x3f534 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f53c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f548 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f54c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f564 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f568 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f56c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f570 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f578 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f57c | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3f588 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f58c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f590 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldrb` at 0x3f594 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0x3f5a0 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f5a4 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f5b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f5b4 | load | 4 | `abs 0x3fc44` | `-` |
| `ldr` at 0x3f5b8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f5c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f5c8 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0x3f5cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3f5d0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x3f5dc | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f5e0 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f5ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f5f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f5f4 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f5f8 | load | 4 | `abs 0x3fc48` | `-` |
| `ldr` at 0x3f5fc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f604 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f60c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f610 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f614 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3f618 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x3f628 | store | 4 | `sp -0x24` | `ret(0x4097c)` |
| `ldr` at 0x3f62c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f638 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f640 | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0x3f644 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3f648 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x3f650 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f65c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3f660 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3f664 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3f66c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f670 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f674 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f678 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f67c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3f680 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x3f690 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3f698 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f69c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6a0 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f6a4 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3f6b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6b8 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3f6cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6d0 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3f6e4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3f6ec | store | 1 | `[sp-28] +0x2c` | `0x1` |
| `ldr` at 0x3f6f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f6f8 | load | 4 | `[sp-28] +0x18` | `-` |
| `str` at 0x3f6fc | store | 4 | `[sp-28] +0x14` | `[[sp-28]+24]` |
| `ldr` at 0x3f700 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f708 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x3f70c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x3f724 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f728 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f72c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3f730 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0x3f73c | load | 4 | `abs 0x3fc4c` | `-` |
| `str` at 0x3f740 | store | 4 | `sp -0x2c` | `0xfffff9ba` |
| `ldr` at 0x3f748 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f74c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f750 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3f754 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3f760 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f764 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f768 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f76c | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3f770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f774 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f78c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f790 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldrb` at 0x3f794 | load | 1 | `[sp-28] +0x2a` | `-` |
| `ldr` at 0x3f7a0 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f7a4 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f7b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f7b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f7b8 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f7bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f7c0 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3f7c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f7c8 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0x3f7cc | store | 4 | `sp -0x40` | `[[sp-28]+48]` |
| `ldr` at 0x3f7d0 | load | 4 | `abs 0x3fc50` | `-` |
| `ldr` at 0x3f7d4 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f7e4 | load | 4 | `abs 0x3fc54` | `-` |
| `str` at 0x3f7e8 | store | 4 | `sp -0x2c` | `0xfffff9b7` |
| `ldr` at 0x3f7f0 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f7f4 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f800 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f804 | load | 4 | `abs 0x3fc58` | `-` |
| `ldr` at 0x3f808 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3f810 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f814 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f818 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f81c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3f820 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3f824 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3f82c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f830 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f834 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f838 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f83c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3f840 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3f84c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f854 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `str` at 0x3f85c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3f864 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f868 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3f87c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f880 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f884 | load | 1 | `[sp-28] +0x26` | `-` |
| `strb` at 0x3f888 | store | 1 | `[sp-28] +0x29` | `([[sp-28]+38]:1 & 0xff)` |
| `ldr` at 0x3f88c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f890 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x3f894 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f898 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3f89c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f8a0 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3f8b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f8bc | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3f8d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f8dc | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3f8e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f8e4 | load | 1 | `[sp-28] +0x29` | `-` |
| `str` at 0x3f8f8 | store | 4 | `sp -0x34` | `ret(0x1b490)` |
| `ldr` at 0x3f8fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f900 | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3f904 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3f914 | store | 4 | `sp -0x34` | `ret(0xfa70c)` |
| `str` at 0x3f920 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x3f924 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3f928 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3f92c | store | 4 | `[sp-48] +0x18` | `?` |
| `ldr` at 0x3f934 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f938 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f93c | load | 1 | `[sp-28] +0x2c` | `-` |
| `str` at 0x3f940 | store | 4 | `[sp-28] +0x18` | `[[sp-28]+44]:1` |
| `ldr` at 0x3f948 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f94c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f950 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3f954 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0x3f960 | load | 4 | `abs 0x3fc4c` | `-` |
| `str` at 0x3f964 | store | 4 | `sp -0x2c` | `0xfffff9ba` |
| `ldr` at 0x3f96c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f970 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f974 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3f978 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3f984 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f988 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f98c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f990 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3f994 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f998 | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x3f9ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f9b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f9b4 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldrb` at 0x3f9b8 | load | 1 | `[sp-28] +0x2b` | `-` |
| `ldr` at 0x3f9c4 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3f9c8 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3f9d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f9d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f9dc | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3f9e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f9e4 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3f9e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f9ec | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0x3f9f0 | store | 4 | `sp -0x40` | `[[sp-28]+48]` |
| `ldr` at 0x3f9f4 | load | 4 | `abs 0x3fc5c` | `-` |
| `ldr` at 0x3f9f8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3fa08 | load | 4 | `abs 0x3fc54` | `-` |
| `str` at 0x3fa0c | store | 4 | `sp -0x2c` | `0xfffff9b7` |
| `ldr` at 0x3fa14 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3fa18 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3fa24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa28 | load | 4 | `abs 0x3fc58` | `-` |
| `ldr` at 0x3fa2c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3fa34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa40 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fa44 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3fa48 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3fa50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fa5c | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fa60 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fa64 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x3fa70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3fa78 | store | 4 | `[sp-32] +0x0` | `0x1` |
| `str` at 0x3fa80 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3fa88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fa8c | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x3faa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3faa4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3faa8 | load | 1 | `[sp-28] +0x26` | `-` |
| `strb` at 0x3faac | store | 1 | `[sp-28] +0x28` | `([[sp-28]+38]:1 & 0xff)` |
| `ldr` at 0x3fab0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3fab4 | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0x3fab8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fabc | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fac0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fac4 | load | 1 | `[sp-28] +0x29` | `-` |
| `ldr` at 0x3fadc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fae0 | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3fafc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb00 | load | 1 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x3fb04 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb08 | load | 1 | `[sp-28] +0x29` | `-` |
| `str` at 0x3fb1c | store | 4 | `sp -0x3c` | `ret(0x1b490)` |
| `ldr` at 0x3fb20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb24 | load | 1 | `[sp-28] +0x2d` | `-` |
| `ldr` at 0x3fb28 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3fb3c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x3fb48 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x3fb4c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3fb50 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3fb54 | store | 4 | `[sp-56] +0x18` | `[sp-60]` |
| `ldr` at 0x3fb5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb60 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb64 | load | 1 | `[sp-28] +0x2c` | `-` |
| `str` at 0x3fb6c | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0x3fb70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb7c | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0x3fb80 | load | 4 | `[sp-28] +0x30` | `-` |
| `str` at 0x3fb88 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0x3fb8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fb94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fb98 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldrb` at 0x3fb9c | load | 1 | `[sp-28] +0x18` | `-` |
| `strb` at 0x3fba4 | store | 1 | `[sp-28] +0x26` | `(([[sp-28]+38]:1 + [[sp-28]+24]:1) & 0xff)` |
| `ldr` at 0x3fba8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3fbb0 | store | 1 | `[sp-28] +0x2d` | `0x2` |
| `ldr` at 0x3fbb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fbb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fbbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fbc0 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0x3fbc4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3fbc8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x3fbd8 | store | 4 | `sp -0x24` | `ret(0x4097c)` |
| `ldr` at 0x3fbdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3fbe8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3fbec | store | 4 | `sp -0x2c` | `ret(0x4097c)` |
| `ldr` at 0x3fbf4 | load | 4 | `abs 0x3fc30` | `-` |
| `ldr` at 0x3fbf8 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3fc04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fc08 | load | 4 | `abs 0x3fc60` | `-` |
| `ldr` at 0x3fc0c | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0x3fc18 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3fc1c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3fc20 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3fc20 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3fc20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3fc20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3fc20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f278 | `(unknown callee) 0x21e28` | `0x9` | `0x1082ec` | `0x763` | `0x0` |
| `bl` at 0x3f2a4 | `(unknown callee) 0x3fcf8` | `?` | `?` | `0x0` | `?` |
| `bl` at 0x3f2e4 | `(unknown callee) 0xfb9a0` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f314 | `(unknown callee) 0x23f2c` | `0x108798` | `[[sp-28]+16]` | `?` | `[sp-28]` |
| `bl` at 0x3f334 | `(unknown callee) 0xfb998` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f354 | `(unknown callee) 0xfb990` | `[sp-40]` | `[[sp-28]+32]` | `?` | `[sp-28]` |
| `bl` at 0x3f370 | `(unknown callee) 0x1b490` | `[[sp-28]+24]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3f398 | `(unknown callee) 0xfaca8` | `[sp-40]` | `?` | `?` | `[0x148520]` |
| `bl` at 0x3f3b4 | `(unknown callee) 0x23f2c` | `0x1087b8` | `[[sp-28]+16]` | `ret(0xfaca8)` | `?` |
| `bl` at 0x3f3f0 | `(unknown callee) 0xfb990` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f40c | `(unknown callee) 0x1b490` | `[[sp-28]+24]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3f474 | `(unknown callee) 0x23f2c` | `0x1087f0` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f494 | `(unknown callee) 0xfb998` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f4b4 | `(unknown callee) 0xfb998` | `[sp-40]` | `[[sp-28]+32]` | `?` | `[sp-28]` |
| `bl` at 0x3f520 | `(unknown callee) 0x23f2c` | `0x108828` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f550 | `(unknown callee) 0xfb990` | `[sp-40]` | `[[sp-28]+32]` | `?` | `[sp-28]` |
| `bl` at 0x3f5bc | `(unknown callee) 0x23f2c` | `0x108854` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f600 | `(unknown callee) 0x23f2c` | `0x108890` | `[[sp-28]+16]` | `[[sp-28]+38]:1` | `[sp-28]` |
| `bl` at 0x3f620 | `(unknown callee) 0x4097c` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3f668 | `(unknown callee) 0x40ec0` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0x3f688 | `(unknown callee) 0x40e50` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3f6bc | `(unknown callee) 0x1b490` | `[[sp-28]+24]` | `[[sp-28]+24]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f6d4 | `(unknown callee) 0x1b490` | `[[sp-28]+20]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x3f710 | `(unknown callee) 0xfb998` | `[[sp-28]+28]` | `[[sp-28]+32]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f7e0 | `(unknown callee) 0x23f2c` | `0x1088b0` | `[[sp-28]+16]` | `[[sp-28]+38]:1` | `[[sp-28]+40]:1` |
| `bl` at 0x3f80c | `(unknown callee) 0x23f2c` | `0x1088e0` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3f828 | `(unknown callee) 0x40ec0` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0x3f848 | `(unknown callee) 0x40e50` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3f86c | `(unknown callee) 0x1b490` | `[[sp-28]+20]` | `[[sp-28]+38]:1` | `[[sp-28]+40]:1` | `[sp-28]` |
| `bl` at 0x3f8ac | `(unknown callee) 0x1b490` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3f8c8 | `(unknown callee) 0xfa70c` | `ret(0x1b490)` | `[[sp-28]+45]:1` | `ret(0x1b490)` | `[[sp-28]+45]:1` |
| `bl` at 0x3f8f0 | `(unknown callee) 0x1b490` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3f90c | `(unknown callee) 0xfa70c` | `ret(0x1b490)` | `[[sp-28]+45]:1` | `?` | `[[sp-28]+45]:1` |
| `bl` at 0x3fa04 | `(unknown callee) 0x23f2c` | `0x108908` | `[[sp-28]+16]` | `[[sp-28]+38]:1` | `[[sp-28]+41]:1` |
| `bl` at 0x3fa30 | `(unknown callee) 0x23f2c` | `0x1088e0` | `[[sp-28]+16]` | `[sp-28]` | `[sp-28]` |
| `bl` at 0x3fa4c | `(unknown callee) 0x40ec0` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0x3fa6c | `(unknown callee) 0x40e50` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3fa90 | `(unknown callee) 0x1b490` | `[[sp-28]+20]` | `[[sp-28]+38]:1` | `[[sp-28]+41]:1` | `[sp-28]` |
| `bl` at 0x3fad0 | `(unknown callee) 0x1b490` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3faec | `(unknown callee) 0xfa70c` | `ret(0x1b490)` | `[[sp-28]+45]:1` | `ret(0x1b490)` | `[[sp-28]+45]:1` |
| `bl` at 0x3fb14 | `(unknown callee) 0x1b490` | `?` | `?` | `[[sp-28]+40]:1` | `?` |
| `bl` at 0x3fb30 | `(unknown callee) 0xfa70c` | `ret(0x1b490)` | `[[sp-28]+45]:1` | `?` | `[[sp-28]+45]:1` |
| `bl` at 0x3fbd0 | `(unknown callee) 0x4097c` | `[[sp-28]]` | `[[sp-28]+12]` | `[[sp-28]+38]:1` | `[[sp-28]+38]:1` |
| `bl` at 0x3fc10 | `(unknown callee) 0x23f2c` | `0x108938` | `[[sp-28]+16]` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3f24c | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x3f25c | `[r0+12] != 0x0` | `[r0+12] == 0x0` |
| `beq` at 0x3f2b8 | `ret(0x3fcf8) == 0x0` | `ret(0x3fcf8) != 0x0` |
| `beq` at 0x3f2f0 | `ret(0xfb9a0) == 0x0` | `ret(0xfb9a0) != 0x0` |
| `beq` at 0x3f304 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `blt` at 0x3f340 | `ret(0xfb998) < 0x0 (signed, 32-bit)` | `ret(0xfb998) >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3f360 | `ret(0xfb990) >= 0x0 (signed, 32-bit)` | `ret(0xfb990) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3f37c | `ret(0x1b490) != 0x1` | `ret(0x1b490) == 0x1` |
| `beq` at 0x3f38c | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `bge` at 0x3f3fc | `ret(0xfb990) >= 0x0 (signed, 32-bit)` | `ret(0xfb990) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3f418 | `ret(0x1b490) != 0x1` | `ret(0x1b490) == 0x1` |
| `bne` at 0x3f434 | `([[sp-28]+24] + [[sp-28]+20]) != 0x0` | `([[sp-28]+24] + [[sp-28]+20]) == 0x0` |
| `bgt` at 0x3f450 | `? > 0x0 (signed, 32-bit)` | `? <= 0x0 (signed, 32-bit)` |
| `beq` at 0x3f464 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `blt` at 0x3f4a0 | `ret(0xfb998) < 0x0 (signed, 32-bit)` | `ret(0xfb998) >= 0x0 (signed, 32-bit)` |
| `blt` at 0x3f4c0 | `ret(0xfb998) < 0x0 (signed, 32-bit)` | `ret(0xfb998) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x3f4e8 | `([[sp-28]+38]:1 + [[sp-28]+48]) != [[sp-28]+40]:1` | `([[sp-28]+38]:1 + [[sp-28]+48]) == [[sp-28]+40]:1` |
| `beq` at 0x3f500 | `[[sp-28]+40]:1 == [[sp-28]+36]:1` | `[[sp-28]+40]:1 != [[sp-28]+36]:1` |
| `beq` at 0x3f510 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `bgt` at 0x3f55c | `ret(0xfb990) > 0x0 (signed, 32-bit)` | `ret(0xfb990) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x3f584 | `? != [[sp-28]+41]:1` | `? == [[sp-28]+41]:1` |
| `beq` at 0x3f59c | `[[sp-28]+41]:1 == [[sp-28]+37]:1` | `[[sp-28]+41]:1 != [[sp-28]+37]:1` |
| `beq` at 0x3f5ac | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `beq` at 0x3f5d8 | `[[sp-32]] == 0x0` | `[[sp-32]] != 0x0` |
| `beq` at 0x3f5e8 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `beq` at 0x3f634 | `ret(0x4097c) == 0x0` | `ret(0x4097c) != 0x0` |
| `bge` at 0x3f6b0 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bne` at 0x3f6c8 | `ret(0x1b490) != 0x2` | `ret(0x1b490) == 0x2` |
| `bne` at 0x3f6e0 | `ret(0x1b490) != 0x2` | `ret(0x1b490) == 0x2` |
| `blt` at 0x3f71c | `ret(0xfb998) < 0x0 (signed, 32-bit)` | `ret(0xfb998) >= 0x0 (signed, 32-bit)` |
| `bne` at 0x3f738 | `[[sp-28]+38]:1 != [[sp-28]+36]:1` | `[[sp-28]+38]:1 == [[sp-28]+36]:1` |
| `beq` at 0x3f75c | `[[sp-28]+38]:1 == [[sp-28]+40]:1` | `[[sp-28]+38]:1 != [[sp-28]+40]:1` |
| `beq` at 0x3f780 | `[[sp-28]+38]:1 == ([[sp-28]+40]:1 + [[sp-28]+48])` | `[[sp-28]+38]:1 != ([[sp-28]+40]:1 + [[sp-28]+48])` |
| `bne` at 0x3f79c | `[[sp-28]+40]:1 != [[sp-28]+42]:1` | `[[sp-28]+40]:1 == [[sp-28]+42]:1` |
| `beq` at 0x3f7ac | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `beq` at 0x3f7fc | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `ble` at 0x3f878 | `ret(0x1b490) <= 0x3 (signed, 32-bit)` | `ret(0x1b490) > 0x3 (signed, 32-bit)` |
| `ble` at 0x3f8d4 | `ret(0xfa70c) <= 0x1 (signed, 32-bit)` | `ret(0xfa70c) > 0x1 (signed, 32-bit)` |
| `bne` at 0x3f95c | `[[sp-28]+38]:1 != [[sp-28]+37]:1` | `[[sp-28]+38]:1 == [[sp-28]+37]:1` |
| `beq` at 0x3f980 | `[[sp-28]+38]:1 == [[sp-28]+41]:1` | `[[sp-28]+38]:1 != [[sp-28]+41]:1` |
| `beq` at 0x3f9a4 | `[[sp-28]+38]:1 == ?` | `[[sp-28]+38]:1 != ?` |
| `bne` at 0x3f9c0 | `[[sp-28]+41]:1 != [[sp-28]+43]:1` | `[[sp-28]+41]:1 == [[sp-28]+43]:1` |
| `beq` at 0x3f9d0 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `beq` at 0x3fa20 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `ble` at 0x3fa9c | `ret(0x1b490) <= 0x3 (signed, 32-bit)` | `ret(0x1b490) > 0x3 (signed, 32-bit)` |
| `ble` at 0x3faf8 | `ret(0xfa70c) <= 0x1 (signed, 32-bit)` | `ret(0xfa70c) > 0x1 (signed, 32-bit)` |
| `beq` at 0x3fbe4 | `ret(0x4097c) == 0x0` | `ret(0x4097c) != 0x0` |
| `beq` at 0x3fc00 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3fc20 | `r0` | `?` |

## `EnvFanInt`

- `EnvFanInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7d944 to 0x7dc00.
  Established. A byte store writes `([[sp-40]+1]:1+1 & 0xff)` at `[sp-40] +0x1` (`strb` at 0x7db08).
  Later stores put byte `0x1` and byte `0x0` at `[sp-40] +0x2` (`strb` at 0x7dba0, `strb` at
  0x7dbc0). The intervening call to `0x22dc0` has `0x148e80`, unknown, `[sp-40]`, `0x1` (`bl` at 0x7dba8); its zero
  result takes the bypass of the clear and nonzero falls through to it (`bl` at 0x7dba8, `beq` at
  0x7dbb4, `strb` at 0x7dbc0). Physical interrupt acknowledgement and the merged return remain
  unknown (`ldmdb` at 0x7dbd0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7d954 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7d95c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7d964 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7d96c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7d970 | load | 4 | `abs 0x7dbd4` | `-` |
| `ldr` at 0x7d974 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7d978 | load | 4 | `abs 0x1490f8` | `-` |
| `ldr` at 0x7d988 | load | 4 | `abs 0x7dbd4` | `-` |
| `ldr` at 0x7d98c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7d9a0 | load | 4 | `abs 0x149104` | `-` |
| `str` at 0x7d9a8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7d9ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7d9b0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7d9b4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7d9c0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7d9c4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7d9c8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7d9d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7d9e0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7d9e8 | load | 4 | `abs 0x7dbd4` | `-` |
| `ldr` at 0x7d9ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7d9f0 | load | 4 | `abs 0x1490f8` | `-` |
| `ldr` at 0x7d9fc | load | 4 | `abs 0x7dbd8` | `-` |
| `str` at 0x7da00 | store | 4 | `sp -0x3c` | `0x110ab0` |
| `ldrb` at 0x7da04 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7da08 | store | 4 | `sp -0x38` | `(r1 & 0xff)` |
| `ldrb` at 0x7da0c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7da10 | store | 4 | `sp -0x34` | `(r2 & 0xff)` |
| `ldr` at 0x7da18 | load | 4 | `abs 0x7dbdc` | `-` |
| `ldr` at 0x7da1c | load | 4 | `abs 0x7dbe0` | `-` |
| `str` at 0x7da2c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x7da34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7da48 | load | 4 | `abs 0x7dbe4` | `-` |
| `str` at 0x7da50 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7da54 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7da58 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7da64 | load | 4 | `abs 0x7dbd4` | `-` |
| `ldr` at 0x7da68 | load | 4 | `abs 0x1490e8` | `-` |
| `ldr` at 0x7da74 | load | 4 | `abs 0x7dbd4` | `-` |
| `ldr` at 0x7da78 | load | 4 | `abs 0x1490ec` | `-` |
| `ldrb` at 0x7da84 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x7da98 | load | 4 | `abs 0x7dbe8` | `-` |
| `str` at 0x7da9c | store | 4 | `sp -0x3c` | `0x110ae4` |
| `str` at 0x7daa0 | store | 4 | `sp -0x38` | `ret(0x46f9c)` |
| `ldrb` at 0x7daa4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7daa8 | store | 4 | `sp -0x34` | `[sp-21]:1` |
| `ldrb` at 0x7daac | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7dab0 | store | 4 | `sp -0x30` | `[sp-22]:1` |
| `ldr` at 0x7dab8 | load | 4 | `abs 0x7dbdc` | `-` |
| `ldr` at 0x7dabc | load | 4 | `abs 0x7dbec` | `-` |
| `str` at 0x7dacc | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x7daec | store | 4 | `sp -0x20` | `0x1e` |
| `str` at 0x7daf8 | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0x7dafc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7db00 | load | 1 | `[sp-40] +0x1` | `-` |
| `strb` at 0x7db08 | store | 1 | `[sp-40] +0x1` | `([[sp-40]+1]:1+1 & 0xff)` |
| `ldr` at 0x7db10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7db20 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x7db28 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7db2c | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x7db30 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7db34 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0x7db48 | load | 4 | `abs 0x7dbf0` | `-` |
| `ldr` at 0x7db4c | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x7db5c | load | 4 | `abs 0x7dbf4` | `-` |
| `ldr` at 0x7db60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7db68 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7db6c | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x7db78 | load | 4 | `abs 0x7dbf0` | `-` |
| `ldr` at 0x7db7c | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x7db8c | load | 4 | `abs 0x7dbf8` | `-` |
| `ldr` at 0x7db90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7db98 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x7dba0 | store | 1 | `[sp-40] +0x2` | `0x1` |
| `ldr` at 0x7dba4 | load | 4 | `abs 0x7dbfc` | `-` |
| `ldr` at 0x7dbb8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x7dbc0 | store | 1 | `[sp-40] +0x2` | `0x0` |
| `str` at 0x7dbc8 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x7dbcc | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x7dbd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7dbd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7dbd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7da24 | `(unknown callee) 0x21e28` | `0x1` | `0x1106f4` | `0x556` | `0x0` |
| `bl` at 0x7da90 | `(unknown callee) 0x46f9c` | `0x0` | `[sp-28]:1` | `?` | `[sp-28]:1` |
| `bl` at 0x7dac4 | `(unknown callee) 0x21e28` | `0x1` | `0x1106f4` | `0x563` | `0x0` |
| `bl` at 0x7dad4 | `(unknown callee) 0x4f090` | `r0` | `?` | `?` | `?` |
| `bl` at 0x7db44 | `(unknown callee) 0xcdbc0` | `0x0` | `[[sp-36]+1]:1` | `[[sp-36]+2]:1` | `[[sp-36]+2]:1` |
| `bl` at 0x7db64 | `(unknown callee) 0x23f2c` | `0x110b14` | `[sp-28]` | `?` | `?` |
| `bl` at 0x7db94 | `(unknown callee) 0x23f2c` | `0x110b28` | `[sp-28]` | `?` | `?` |
| `bl` at 0x7dba8 | `(unknown callee) 0x22dc0` | `0x148e80` | `?` | `[sp-40]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x7d980 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7d9bc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7d9d0 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x7d9f8 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7da60 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7da70 | `[0x1490e8] != 0x0` | `[0x1490e8] == 0x0` |
| `bne` at 0x7da80 | `[0x1490ec] != 0x0` | `[0x1490ec] == 0x0` |
| `bne` at 0x7dae4 | `? != 0x47` | `? == 0x47` |
| `bge` at 0x7db18 | `? >= [sp-32] (signed, 32-bit)` | `? < [sp-32] (signed, 32-bit)` |
| `beq` at 0x7db58 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7db74 | `[[sp-40]+2]:1 != 0x0` | `[[sp-40]+2]:1 == 0x0` |
| `beq` at 0x7db88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7dbb4 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7dbd0 | `r0` | `?` |

## `EnvPowerInt`

- `EnvPowerInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7d6e0 to 0x7d944.
  Established. A byte store writes `0x1` at an unknown effective target (`strb` at 0x7d8e4). The
  following call to `0x22dc0` carries `0x148e2c`, unknown, unknown, `0x1` (`bl` at 0x7d8ec); its
  zero result takes the continuing edge and nonzero falls through to clear byte `[sp-36] +0x1`
  (`beq` at 0x7d8f8, `strb` at 0x7d904). The routine returns an unknown r0 value, and the physical
  power interrupt source is unproven (`ldmdb` at 0x7d914).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7d6f0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7d6f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7d700 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7d708 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7d70c | load | 4 | `abs 0x7d918` | `-` |
| `ldr` at 0x7d710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7d714 | load | 4 | `abs 0x1490f4` | `-` |
| `ldr` at 0x7d724 | load | 4 | `abs 0x7d918` | `-` |
| `ldr` at 0x7d728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7d73c | load | 4 | `abs 0x149100` | `-` |
| `str` at 0x7d744 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7d748 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7d74c | load | 1 | `[sp-32] +0x4` | `-` |
| `ldrb` at 0x7d750 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7d75c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7d760 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0x7d764 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7d774 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7d77c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x7d784 | load | 4 | `abs 0x7d918` | `-` |
| `ldr` at 0x7d788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7d78c | load | 4 | `abs 0x1490f4` | `-` |
| `ldr` at 0x7d798 | load | 4 | `abs 0x7d91c` | `-` |
| `str` at 0x7d79c | store | 4 | `sp -0x38` | `0x110a18` |
| `ldrb` at 0x7d7a0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7d7a4 | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x7d7a8 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7d7ac | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x7d7b4 | load | 4 | `abs 0x7d920` | `-` |
| `ldr` at 0x7d7b8 | load | 4 | `abs 0x7d924` | `-` |
| `str` at 0x7d7c8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7d7d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7d7d8 | load | 4 | `abs 0x7d928` | `-` |
| `str` at 0x7d7e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7d7e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7d7e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7d7f4 | load | 4 | `abs 0x7d918` | `-` |
| `ldr` at 0x7d7f8 | load | 4 | `abs 0x1490e8` | `-` |
| `ldr` at 0x7d804 | load | 4 | `abs 0x7d918` | `-` |
| `ldr` at 0x7d808 | load | 4 | `abs 0x1490ec` | `-` |
| `ldr` at 0x7d818 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7d834 | load | 4 | `abs 0x7d92c` | `-` |
| `str` at 0x7d838 | store | 4 | `sp -0x38` | `0x110a50` |
| `str` at 0x7d83c | store | 4 | `sp -0x34` | `ret(0x3fff0)` |
| `ldrb` at 0x7d840 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7d844 | store | 4 | `sp -0x30` | `[sp-21]:1` |
| `ldrb` at 0x7d848 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x7d84c | store | 4 | `sp -0x2c` | `[sp-22]:1` |
| `ldr` at 0x7d854 | load | 4 | `abs 0x7d920` | `-` |
| `ldr` at 0x7d858 | load | 4 | `abs 0x7d930` | `-` |
| `str` at 0x7d868 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7d870 | load | 4 | `abs 0x7d934` | `-` |
| `ldr` at 0x7d874 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x7d888 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7d8a4 | load | 4 | `abs 0x7d938` | `-` |
| `ldr` at 0x7d8b0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7d8b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7d8c0 | load | 4 | `abs 0x7d934` | `-` |
| `ldr` at 0x7d8c4 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x7d8d4 | load | 4 | `abs 0x7d93c` | `-` |
| `ldr` at 0x7d8dc | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x7d8e4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x7d8e8 | load | 4 | `abs 0x7d940` | `-` |
| `ldr` at 0x7d8fc | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x7d904 | store | 1 | `[sp-36] +0x1` | `0x0` |
| `str` at 0x7d90c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x7d910 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x7d914 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7d914 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7d914 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7d7c0 | `(unknown callee) 0x21e28` | `0x1` | `0x1106f4` | `0x517` | `0x0` |
| `bl` at 0x7d81c | `(unknown callee) 0x404e8` | `0x0` | `[sp-32]` | `?` | `[0x1490ec]` |
| `bl` at 0x7d82c | `(unknown callee) 0x3fff0` | `0x0` | `ret(0x404e8)` | `?` | `ret(0x404e8)` |
| `bl` at 0x7d860 | `(unknown callee) 0x21e28` | `0x1` | `0x1106f4` | `0x524` | `0x0` |
| `bl` at 0x7d88c | `(unknown callee) 0x404e8` | `0x0` | `[sp-32]` | `?` | `?` |
| `bl` at 0x7d89c | `(unknown callee) 0x3fff0` | `0x0` | `ret(0x404e8)` | `?` | `ret(0x404e8)` |
| `bl` at 0x7d8ac | `(unknown callee) 0x23f2c` | `0x110a84` | `ret(0x3fff0)` | `?` | `ret(0x3fff0)` |
| `bl` at 0x7d8d8 | `(unknown callee) 0x23f2c` | `0x110a98` | `?` | `?` | `?` |
| `bl` at 0x7d8ec | `(unknown callee) 0x22dc0` | `0x148e2c` | `?` | `?` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x7d71c | `C == 0` | `not (C == 0)` |
| `bne` at 0x7d758 | `[[sp-32]+4]:1 != [sp-21]:1` | `[[sp-32]+4]:1 == [sp-21]:1` |
| `bne` at 0x7d76c | `[[sp-32]+5]:1 != [sp-22]:1` | `[[sp-32]+5]:1 == [sp-22]:1` |
| `blo` at 0x7d794 | `C == 0` | `not (C == 0)` |
| `bne` at 0x7d7f0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7d800 | `[0x1490e8] != 0x0` | `[0x1490e8] == 0x0` |
| `bne` at 0x7d810 | `[0x1490ec] != 0x0` | `[0x1490ec] == 0x0` |
| `beq` at 0x7d880 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7d8bc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x7d8d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7d8f8 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7d914 | `r0` | `?` |

## `monitorVSC055Int`

- `monitorVSC055Int`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1398 to 0xd1584.
  Established. The branch takes its continuing edge for `[sp-33]:1 != 0xff` and falls through on
  equality to `0x4f564` (`bne` at 0xd1490, `bl` at 0xd14a4). The final call reaches `0x7d944`
  with `[sp-20]`, `[sp-33]:1`, `[sp-34]:1`, `[sp-33]:1` (`bl` at 0xd155c). The r0 return is `0x1`
  (`ldmdb` at 0xd1568). Channel mapping and the physical interrupt source remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd13a8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd13b4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd13bc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd13c4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xd13c8 | load | 4 | `abs 0xd156c` | `-` |
| `ldr` at 0xd13cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd13d4 | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xd13dc | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0xd13e4 | store | 1 | `sp -0x21` | `?` |
| `strb` at 0xd13ec | store | 1 | `sp -0x22` | `?` |
| `str` at 0xd13f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd13f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd13fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1400 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd1410 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1414 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1428 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xd1430 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd1434 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1444 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1458 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xd1460 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd1464 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd1470 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xd1474 | store | 1 | `sp -0x21` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xd1478 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1480 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd1488 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd1494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd149c | load | 4 | `abs 0xd1570` | `-` |
| `ldr` at 0xd14a0 | load | 4 | `abs 0xd1574` | `-` |
| `str` at 0xd14cc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd14d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd14d8 | load | 4 | `abs 0xd1570` | `-` |
| `ldr` at 0xd14dc | load | 4 | `abs 0xd1574` | `-` |
| `ldr` at 0xd14e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd14f8 | load | 4 | `abs 0xd1578` | `-` |
| `str` at 0xd14fc | store | 4 | `(unknown)` | `0x134064` |
| `str` at 0xd1500 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd1504 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd1508 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xd150c | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd1510 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xd1514 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd1518 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xd1520 | load | 4 | `abs 0xd157c` | `-` |
| `ldr` at 0xd1524 | load | 4 | `abs 0xd1580` | `-` |
| `ldrb` at 0xd1530 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xd1548 | store | 1 | `sp -0x22` | `?` |
| `ldrb` at 0xd154c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd1550 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xd1554 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd1568 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1568 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1568 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd14a4 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xd14b0 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd14e0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xd14f0 | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd152c | `(unknown callee) 0x21e28` | `0x1` | `0x133d7c` | `0xbc9` | `0x0` |
| `bl` at 0xd155c | `(unknown callee) 0x7d944` | `[sp-20]` | `[sp-33]:1` | `[sp-34]:1` | `[sp-33]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd1408 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd143c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd146c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd1490 | `[sp-33]:1 != 0xff` | `[sp-33]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1568 | `r0` | `0x1` |

## `mic74_Reset`

- `mic74_Reset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9e74 to 0xda13c.
  Established. Calls to `0xc7f7c` include unknown, `[sp-26]:1`, `0x4`, `0x0` and later `[sp-40]`,
  `[sp-26]:1`, `0x0`, unknown (`bl` at 0xd9ed0, `bl` at 0xd9f40). Further calls to that target
  include unknown, `[sp-26]:1`, `0x2`, `[[sp-24]+5]:1` and unknown, `[sp-26]:1`, `0x5`,
  `[[sp-24]+6]:1` (`bl` at 0xd9f98, `bl` at 0xd9ff0). The last call reaches `0xc80a0` with unknown,
  `[sp-26]:1`, `0x3`, `sp-27` (`bl` at 0xda104). The r0 return is `[sp-32]`, while device-reset
  effects and register meanings remain unknown (`ldmdb` at 0xda138).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9e84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9e88 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd9e8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9e90 | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0xd9e94 | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0xd9e98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9e9c | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0xd9ea0 | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0xd9ea4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd9ea8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd9ebc | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xd9ed8 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldrb` at 0xd9edc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd9ee0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd9ef4 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xd9ef8 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xd9efc | store | 4 | `sp -0x2c` | `[sp-26]:1` |
| `str` at 0xd9f04 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd9f08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9f0c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xd9f18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9f1c | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xd9f2c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd9f30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd9f34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9f3c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd9f48 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xd9f4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd9f58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9f64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd9f68 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldrb` at 0xd9f6c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd9f70 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd9f84 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd9f88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9f8c | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0xd9fa0 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xd9fa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd9fb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9fbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd9fc0 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldrb` at 0xd9fc4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd9fc8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd9fdc | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd9fe0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9fe4 | load | 1 | `[sp-24] +0x6` | `-` |
| `str` at 0xd9ff8 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xd9ffc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda008 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda014 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda018 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldrb` at 0xda01c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda020 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda034 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xda038 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda03c | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xda058 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xda05c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda068 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda074 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda078 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldrb` at 0xda07c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda080 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda094 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xda0b0 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xda0b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda0c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda0cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda0d0 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldrb` at 0xda0d4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda0d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda0ec | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xda10c | store | 4 | `sp -0x24` | `ret(0xc80a0)` |
| `ldr` at 0xda110 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda11c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda128 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda12c | store | 4 | `sp -0x20` | `ret(0xc80a0)` |
| `ldr` at 0xda130 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xda138 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda138 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda138 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9eb0 | `(unknown callee) 0xc7784` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0xd9ed0 | `(unknown callee) 0xc7f7c` | `?` | `[sp-26]:1` | `0x4` | `0x0` |
| `bl` at 0xd9ee8 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xd9f40 | `(unknown callee) 0xc7f7c` | `[sp-40]` | `[sp-26]:1` | `0x0` | `?` |
| `bl` at 0xd9f78 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xd9f98 | `(unknown callee) 0xc7f7c` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0xd9fd0 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xd9ff0 | `(unknown callee) 0xc7f7c` | `?` | `[sp-26]:1` | `0x5` | `[[sp-24]+6]:1` |
| `bl` at 0xda028 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda050 | `(unknown callee) 0xc7f7c` | `?` | `[sp-26]:1` | `0x1` | `?` |
| `bl` at 0xda088 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda0a8 | `(unknown callee) 0xc7f7c` | `?` | `[sp-26]:1` | `0x6` | `0x0` |
| `bl` at 0xda0e0 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda104 | `(unknown callee) 0xc80a0` | `?` | `[sp-26]:1` | `0x3` | `sp-27` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd9f14 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |
| `beq` at 0xd9f24 | `[[sp-24]+4]:1 == 0x0` | `[[sp-24]+4]:1 != 0x0` |
| `beq` at 0xd9f54 | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `bne` at 0xd9f60 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xd9fac | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `bne` at 0xd9fb8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xda004 | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `bne` at 0xda010 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xda064 | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `bne` at 0xda070 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xda0bc | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `bne` at 0xda0c8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xda118 | `ret(0xc80a0) == 0x0` | `ret(0xc80a0) != 0x0` |
| `bne` at 0xda124 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda138 | `r0` | `[sp-32]` |

## `mic74_PreReset`

- `mic74_PreReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda13c to 0xda210.
  Established. Two calls to `0xc7f7c` carry unknown, `[sp-26]:1`, `0x2`, `[[sp-24]+5]:1` and then
  unknown, `[sp-26]:1`, `0x1`, unknown (`bl` at 0xda198, `bl` at 0xda1d8). The second zero result
  takes the continuation and nonzero falls through to test `[sp-32]` (`beq` at 0xda1ec, `bne` at
  0xda1f8). That second test takes its bypass for `[sp-32] != 0x0` and falls through on equality to
  save `ret(0xc7f7c)` as a word at `sp -0x20` (`bne` at 0xda1f8, `str` at 0xda200). The returned
  value is `[sp-32]` (`ldmdb` at 0xda20c). Physical preparation for reset remains unproven (`ldmdb` at 0xda20c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda14c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xda150 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xda154 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda158 | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0xda15c | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0xda160 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda164 | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0xda168 | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0xda16c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda170 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda184 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xda188 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda18c | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0xda1a0 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldrb` at 0xda1a4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda1a8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda1bc | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xda1c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda1c4 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xda1e0 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xda1e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda1f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda1fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda200 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldr` at 0xda204 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xda20c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda20c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda20c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda178 | `(unknown callee) 0xc7784` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0xda198 | `(unknown callee) 0xc7f7c` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0xda1b0 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda1d8 | `(unknown callee) 0xc7f7c` | `?` | `[sp-26]:1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xda1ec | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `bne` at 0xda1f8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda20c | `r0` | `[sp-32]` |

## `mic74_IntSet`

- `mic74_IntSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda210 to 0xda278.
  Established. A byte load reads entry `r1 +0x1` before the call to `0xc7784` (`ldrb` at 0xda234,
  `bl` at 0xda240). The final call targets `0xc7f7c` with unknown, `[[sp-24]+2]:1`, `0x4`,
  `[sp-25]:1` (`bl` at 0xda268); its result supplies the r0 return (`ldmdb` at 0xda274). The facts
  report no conditional branches, and the physical interrupt-setting operation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda220 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xda224 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xda22c | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xda230 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda234 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xda238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda24c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda250 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xda254 | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0xda274 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda274 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda274 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda240 | `(unknown callee) 0xc7784` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xda268 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda274 | `r0` | `ret(0xc7f7c)` |

## `mic74_IntGet`

- `mic74_IntGet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda278 to 0xda2d8.
  Established. The entry r0, r1 and r2 words are retained at `sp -0x14`, `sp -0x18`, and `sp -0x1c`
  (`str` at 0xda288, `str` at 0xda28c, `str` at 0xda290). The call to `0xc80a0` has unknown,
  `[[sp-24]+2]:1`, `0x4`, `[sp-28]` and supplies r0 at return (`bl` at 0xda2c8, `ldmdb` at 0xda2d4).
  No conditional branches are reported. Interrupt status representation and callee side effects
  remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda288 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xda28c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xda290 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xda294 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda298 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xda29c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xda2b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda2b4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xda2c4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xda2d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda2d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda2d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda2a4 | `(unknown callee) 0xc7784` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xda2c8 | `(unknown callee) 0xc80a0` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda2d4 | `r0` | `ret(0xc80a0)` |

## `mic74_Int`

- `mic74_Int`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda2d8 to 0xda548.
  Established. The loop branch retains a less-or-equal flags predicate with unknown operands, following
  its edge when true and falling through otherwise (`ble` at 0xda310). Within the loop, a call to
  `0xc80a0` carries unknown, `[[sp-28]+2]:1`, `0x3`, `sp-34` (`bl` at 0xda350); its zero result
  takes the continuing edge and nonzero falls through to the other arm (`beq` at 0xda364). A
  later call to `0xd7168` has `[sp-24]`, `[sp-28]`, `[sp-32]`, `[sp-33]:1`, and a word store saves
  `([sp-40] + ret(0xd7168))` at `sp -0x28` (`bl` at 0xda44c, `str` at 0xda45c). The return reports
  `[sp-40]`, while physical interrupt acknowledgement and retry timing remain unknown (`ldmdb` at
  0xda530).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda2e8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xda2ec | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xda2f0 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xda2f4 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `str` at 0xda2fc | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xda304 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xda308 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xda318 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xda31c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xda320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda334 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xda338 | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0xda358 | store | 4 | `sp -0x2c` | `ret(0xc80a0)` |
| `ldr` at 0xda35c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xda368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda370 | load | 4 | `abs 0xda534` | `-` |
| `ldr` at 0xda374 | load | 4 | `abs 0xda538` | `-` |
| `str` at 0xda3a0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xda3a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda3ac | load | 4 | `abs 0xda534` | `-` |
| `ldr` at 0xda3b0 | load | 4 | `abs 0xda538` | `-` |
| `ldr` at 0xda3bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xda3cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xda3d8 | load | 4 | `abs 0xda53c` | `-` |
| `str` at 0xda3dc | store | 4 | `(unknown)` | `0x134d90` |
| `str` at 0xda3e0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xda3e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xda3e8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xda3ec | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xda3f0 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0xda3f8 | load | 4 | `abs 0xda540` | `-` |
| `ldr` at 0xda408 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xda40c | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xda414 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xda418 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xda41c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0xda430 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xda434 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xda438 | store | 4 | `sp -0x4c` | `[sp-34]:1` |
| `ldr` at 0xda43c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xda444 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda454 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xda45c | store | 4 | `sp -0x28` | `([sp-40] + ret(0xd7168))` |
| `ldr` at 0xda464 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xda468 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xda470 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xda478 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xda480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda488 | load | 4 | `abs 0xda534` | `-` |
| `ldr` at 0xda48c | load | 4 | `abs 0xda538` | `-` |
| `str` at 0xda4b8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xda4bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda4c4 | load | 4 | `abs 0xda534` | `-` |
| `ldr` at 0xda4c8 | load | 4 | `abs 0xda538` | `-` |
| `ldr` at 0xda4d4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xda4e4 | load | 4 | `abs 0xda544` | `-` |
| `str` at 0xda4e8 | store | 4 | `(unknown)` | `0x134dd4` |
| `str` at 0xda4ec | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xda4f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xda4f4 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xda4f8 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0xda4fc | load | 1 | `sp -0x22` | `-` |
| `str` at 0xda500 | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0xda504 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xda508 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0xda50c | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0xda514 | load | 4 | `abs 0xda540` | `-` |
| `ldr` at 0xda524 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xda528 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xda52c | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xda530 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xda530 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda530 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda530 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda328 | `(unknown callee) 0xc7784` | `?` | `?` | `?` | `?` |
| `bl` at 0xda350 | `(unknown callee) 0xc80a0` | `?` | `[[sp-28]+2]:1` | `0x3` | `sp-34` |
| `bl` at 0xda378 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134d84` | `0x134d8c` |
| `bl` at 0xda384 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xda3b4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134d84` | `0x134d8c` |
| `bl` at 0xda3c4 | `(unknown callee) 0x1b508` | `[sp-52]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xda3d0 | `(unknown callee) 0x219a8` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xda404 | `(unknown callee) 0x21e28` | `0x3` | `0x134db8` | `0x87` | `0x0` |
| `bl` at 0xda44c | `(unknown callee) 0xd7168` | `[sp-24]` | `[sp-28]` | `[sp-32]` | `[sp-33]:1` |
| `bl` at 0xda490 | `(unknown callee) 0x4f564` | `?` | `0x0` | `0x134d84` | `0x134d8c` |
| `bl` at 0xda49c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xda4cc | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134d84` | `0x134d8c` |
| `bl` at 0xda4dc | `(unknown callee) 0x1b508` | `[sp-52]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xda520 | `(unknown callee) 0x21e28` | `0x3` | `0x134db8` | `0x92` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xda310 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xda364 | `ret(0xc80a0) == 0x0` | `ret(0xc80a0) != 0x0` |
| `beq` at 0xda42c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda530 | `r0` | `[sp-40]` |

## `vsc055_PortReset`

- `vsc055_PortReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8c50 to 0xd8d9c.
  Established. The first call to `0xc7f7c` has unknown, `[[sp-24]+2]:1`, `[[sp-24]+3]:1`,
  `[sp-30]:1` (`bl` at 0xd8d10). The second call to that target uses unknown, `[[sp-24]+2]:1`,
  unknown, `[sp-29]:1` (`bl` at 0xd8d60); its zero result takes the continuation and nonzero falls
  through to a test for `[sp-36] != 0x0` (`beq` at 0xd8d74, `bne` at 0xd8d80). The r0 return is
  `[sp-36]` (`ldmdb` at 0xd8d94). Physical port-reset ordering and values remain unproven (`ldmdb` at 0xd8d94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8c60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd8c64 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd8c68 | load | 4 | `abs 0xd8d98` | `-` |
| `ldr` at 0xd8c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8c80 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd8c88 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd8c8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8c90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8c94 | load | 1 | `r1 +0x5` | `-` |
| `ldrb` at 0xd8c98 | load | 1 | `r1 +0x6` | `-` |
| `strb` at 0xd8ca0 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xd8ca4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8ca8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8cac | load | 1 | `r1 +0x5` | `-` |
| `ldr` at 0xd8cb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8cbc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldrb` at 0xd8cc4 | load | 1 | `r1 +0x4` | `-` |
| `strb` at 0xd8ccc | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xd8cd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8cd4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd8cd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8cec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8cf0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd8cf4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8cf8 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd8cfc | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xd8d18 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xd8d1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8d20 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd8d24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8d38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8d3c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd8d40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8d44 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd8d50 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd8d68 | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xd8d6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8d78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8d84 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8d88 | store | 4 | `sp -0x24` | `ret(0xc7f7c)` |
| `ldr` at 0xd8d8c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd8d94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd8d94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd8d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8ce0 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-24]+1]:1` | `[r1+4]:1` | `[[sp-24]+1]:1` |
| `bl` at 0xd8d10 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `[sp-30]:1` |
| `bl` at 0xd8d2c | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd8d60 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd8d74 | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `bne` at 0xd8d80 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd8d94 | `r0` | `[sp-36]` |

## `vsc055_PortSoftReset`

- `vsc055_PortSoftReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8d9c to 0xd9108.
  Established. The initial call to `0xc80a0` carries unknown, `[[sp-24]+2]:1`, unknown, `sp-27`
  (`bl` at 0xd8e00); its zero result takes the continuation and nonzero falls through to save
  `ret(0xc80a0)` as a word at `sp -0x28` (`beq` at 0xd8e14, `str` at 0xd8e1c). The later call to
  `0xc7f7c` at `0xd9050` has unknown, `[[sp-24]+2]:1`, unknown, `[sp-28]:1` (`bl` at 0xd9050). The
  final call to the same target uses unknown, `[[sp-24]+2]:1`, `[[sp-24]+3]:1`, unknown (`bl` at
  0xd90c8). Soft-reset criteria and the joined return remain unknown (`ldmdb` at 0xd90f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8dac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd8db0 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xd8db8 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xd8dbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8dc0 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xd8dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8dd8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8ddc | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd8de0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8de4 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xd8e08 | store | 4 | `sp -0x20` | `ret(0xc80a0)` |
| `ldr` at 0xd8e0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd8e18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd8e1c | store | 4 | `sp -0x28` | `ret(0xc80a0)` |
| `ldr` at 0xd8e24 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8e28 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd8e2c | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0xd8e44 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0xd8e48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8e4c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd8e54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8e58 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd8e60 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xd8e70 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd8e78 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0xd8e7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8e80 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd8e84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8e88 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd8e90 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xd8e9c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd8eac | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd8eb4 | store | 1 | `sp -0x1a` | `?` |
| `ldrb` at 0xd8eb8 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd8ec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8ec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8ecc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8ed0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd8ed4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd8ee4 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd8eec | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0xd8ef4 | store | 1 | `sp -0x1c` | `?` |
| `ldr` at 0xd8ef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8f00 | load | 4 | `abs 0xd90fc` | `-` |
| `ldr` at 0xd8f04 | load | 4 | `abs 0xd9100` | `-` |
| `str` at 0xd8f30 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd8f34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8f3c | load | 4 | `abs 0xd90fc` | `-` |
| `ldr` at 0xd8f40 | load | 4 | `abs 0xd9100` | `-` |
| `ldr` at 0xd8f4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8f5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8f60 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xd8f64 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0xd8f68 | store | 4 | `(unknown)` | `[sp-27]:1` |
| `ldrb` at 0xd8f6c | load | 1 | `sp -0x1c` | `-` |
| `str` at 0xd8f70 | store | 4 | `(unknown)` | `[sp-28]:1` |
| `ldrb` at 0xd8f74 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xd8f78 | store | 4 | `(unknown)` | `[sp-25]:1` |
| `ldr` at 0xd8f7c | load | 4 | `abs 0xd9104` | `-` |
| `ldr` at 0xd8f84 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xd8f90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8f94 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd8f98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8fac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8fb0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd8fb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8fb8 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xd8fbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd8fc0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd8fc4 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xd8fd0 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xd8ff0 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldr` at 0xd8ff4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9000 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9004 | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xd900c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9010 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd9014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9028 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd902c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd9030 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9034 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd9040 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0xd9058 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldr` at 0xd905c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9068 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd906c | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xd9074 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9078 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd907c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9090 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9094 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd9098 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd909c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xd90a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd90a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd90a8 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd90ac | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd90b4 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xd90d0 | store | 4 | `sp -0x20` | `ret(0xc7f7c)` |
| `ldr` at 0xd90d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd90e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd90e4 | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `str` at 0xd90f0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd90f4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd90f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd90f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd90f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8dcc | `(unknown callee) 0xc7784` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xd8e00 | `(unknown callee) 0xc80a0` | `?` | `[[sp-24]+2]:1` | `?` | `sp-27` |
| `bl` at 0xd8f08 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8f14 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8f44 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134bcc` | `0x134bd4` |
| `bl` at 0xd8f54 | `(unknown callee) 0x1b508` | `[sp-36]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd8f8c | `(unknown callee) 0x2070c` | `0x134cac` | `ret(0x1b508)` | `[[sp-24]+12]` | `[sp-26]:1` |
| `bl` at 0xd8fa0 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd8fe8 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |
| `bl` at 0xd901c | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd9050 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-28]:1` |
| `bl` at 0xd9084 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd90c8 | `(unknown callee) 0xc7f7c` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd8e14 | `ret(0xc80a0) == 0x0` | `ret(0xc80a0) != 0x0` |
| `beq` at 0xd8e6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd8ea8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd8ec0 | `[sp-26]:1 == 0x0` | `[sp-26]:1 != 0x0` |
| `beq` at 0xd8ffc | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `beq` at 0xd9064 | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |
| `beq` at 0xd90dc | `ret(0xc7f7c) == 0x0` | `ret(0xc7f7c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd90f8 | `r0` | `?` |

## `vsc055_GetCfg`

- `vsc055_GetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9108 to 0xd9174.
  Established. The branch takes its bypass for entry `r1 == 0x0` and falls through on nonzero (`beq`
  at 0xd914c). That nonzero arm stores word `[[sp-28]]` at entry `r1 +0x0` (`str` at 0xd915c). The
  later word read addresses `[sp-28] +0x4` and supplies the r0 return `[[sp-28]+4]` (`ldr` at
  0xd9164, `ldmdb` at 0xd916c). No calls are reported, and configuration-field meanings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9118 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd911c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd9120 | load | 4 | `abs 0xd9170` | `-` |
| `ldr` at 0xd9124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9138 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd9140 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd9144 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9150 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9154 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9158 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd915c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd9160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9164 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd916c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd916c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd916c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd914c | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd916c | `r0` | `[[sp-28]+4]` |

## `vsc055_Count`

- `vsc055_Count`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9174 to 0xd91c0.
  Established. A word read addresses absolute `0x142b10` (`ldr` at 0xd91a0). The later word load
  addresses `[sp-24] +0x4`, and r0 returns `[[sp-24]+4]` (`ldr` at 0xd91b0, `ldmdb` at 0xd91b8).
  Calls and conditional branches are absent. The referenced object and count units are unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9184 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd9188 | load | 4 | `abs 0xd91bc` | `-` |
| `ldr` at 0xd918c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd91a0 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd91a8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd91ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd91b0 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0xd91b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd91b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd91b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd91b8 | `r0` | `[[sp-24]+4]` |

## `vsc055_Present`

- `vsc055_Present`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd91c0 to 0xd9244.
  Established. The signed bound takes its continuing edge for `(r1 & 0xff) < [[sp-28]+4]` and falls
  through for greater-or-equal (`blt` at 0xd9210). The fall-through arm stores word `0x0` at `sp
  -0x20` (`str` at 0xd9218). The continuing arm loads a byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0`
  and saves its value as a word at that stack target (`ldrb` at 0xd9230, `str` at 0xd9234). The
  physical presence criterion and merged r0 return remain unproven (`ldmdb` at 0xd923c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd91d0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd91d8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd91dc | load | 4 | `abs 0xd9240` | `-` |
| `ldr` at 0xd91e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd91f4 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd91fc | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd9200 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9208 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd9218 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd9220 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd9224 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9228 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd9230 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xd9234 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xd9238 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd923c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd923c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd923c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd9210 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd923c | `r0` | `?` |
