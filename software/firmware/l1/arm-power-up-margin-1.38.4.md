# ARM power-up and margin paths in 1.38.4

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
Power-up, margin-read and margin-name selectors have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/power-up-margin-facts.txt) retain all tool sections.

## `powerUp`

- `powerUp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3cc04 to 0x3d284.
  Established. The first computed transfer takes an unresolved edge for `ret(0x503e0)+1704 <= 0x4`
  unsigned and falls through above that bound (`ldrls` at 0x3ccbc). The later computed transfer does
  the same for `[sp-53]:1-48 <= 0x4c`, falling through for greater (`ldrls` at 0x3cfb4). On known
  paths, byte stores clear `[sp-72] +0xc`, set `[sp-72] +0xe` to `0x1`, and set `[sp-72] +0xd` to
  `0x1` (`strb` at 0x3cdb4, `strb` at 0x3cdd4, `strb` at 0x3cf98). The call to `0x41e80` has `0x0`,
  `0xff`, `0x3`, unknown (`bl` at 0x3cf60). Excluded dispatch actions, physical power-up timing and
  the joined r0 value remain unknown (`ldmdb` at 0x3d240).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3cc14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3cc18 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3cc20 | store | 4 | `sp -0x20` | `0x0` |
| `strb` at 0x3cc2c | store | 1 | `sp -0x35` | `(ret(0x4f090) & 0xff)` |
| `ldr` at 0x3cc30 | load | 4 | `abs 0x3d244` | `-` |
| `ldr` at 0x3cc34 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3cc38 | store | 4 | `sp -0x48` | `[0x147fcc]` |
| `ldr` at 0x3cc40 | load | 4 | `abs 0x3d248` | `-` |
| `ldr` at 0x3cc44 | load | 4 | `abs 0x2ae604` | `-` |
| `ldr` at 0x3cc50 | load | 4 | `abs 0x3d248` | `-` |
| `ldr` at 0x3cc54 | load | 4 | `abs 0x2ae604` | `-` |
| `ldr` at 0x3cc6c | load | 4 | `abs 0x3d24c` | `-` |
| `str` at 0x3cc70 | store | 4 | `sp -0x64` | `0x10840c` |
| `ldr` at 0x3cc78 | load | 4 | `abs 0x3d250` | `-` |
| `ldr` at 0x3cc7c | load | 4 | `abs 0x3d254` | `-` |
| `str` at 0x3cc8c | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x3cc9c | store | 4 | `sp -0x28` | `ret(0x503e0)` |
| `ldr` at 0x3cca0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ccac | load | 4 | `sp -0x28` | `-` |
| `ldrls` at 0x3ccbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3cd1c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x3cd28 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3cd2c | store | 4 | `sp -0x50` | `[sp-40]` |
| `ldr` at 0x3cd34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3cd3c | store | 4 | `sp -0x54` | `ret(0x4e650)` |
| `ldr` at 0x3cd40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cd4c | load | 4 | `abs 0x3d26c` | `-` |
| `str` at 0x3cd50 | store | 4 | `sp -0x58` | `0x1083ec` |
| `ldr` at 0x3cd58 | load | 4 | `abs 0x3d270` | `-` |
| `str` at 0x3cd5c | store | 4 | `sp -0x58` | `0x1083f0` |
| `ldr` at 0x3cd60 | load | 4 | `abs 0x3d274` | `-` |
| `ldr` at 0x3cd64 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x3cd68 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x3cd7c | store | 4 | `sp -0x24` | `ret(0x51aa0)` |
| `str` at 0x3cd90 | store | 4 | `sp -0x24` | `ret(0x52770)` |
| `ldr` at 0x3cd98 | load | 4 | `abs 0x3d278` | `-` |
| `str` at 0x3cda8 | store | 4 | `sp -0x24` | `ret(0x51580)` |
| `ldr` at 0x3cdac | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3cdb4 | store | 1 | `[sp-72] +0xc` | `0x0` |
| `str` at 0x3cdc4 | store | 4 | `sp -0x20` | `ret(0x4e1ac)` |
| `ldr` at 0x3cdcc | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3cdd4 | store | 1 | `[sp-72] +0xe` | `0x1` |
| `str` at 0x3cddc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3cde0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3cde4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cde8 | load | 4 | `[sp-72] +0x8` | `-` |
| `ldr` at 0x3cdf8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3cdfc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ce10 | load | 4 | `[sp-72] +0x0` | `-` |
| `str` at 0x3ce18 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x3ce1c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ce20 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x3ce24 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3ce2c | load | 1 | `([[sp-72]+4] + [sp-44]) +0x0` | `-` |
| `ldr` at 0x3ce38 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x3ce3c | load | 1 | `[sp-76] +0x2` | `-` |
| `ldr` at 0x3ce4c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x3ce58 | store | 4 | `sp -0x24` | `ret(0x406f4)` |
| `ldr` at 0x3ce5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ce74 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3ce7c | store | 1 | `[sp-72] +0xe` | `0x0` |
| `ldr` at 0x3ce80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ce84 | store | 4 | `sp -0x50` | `[sp-36]` |
| `ldr` at 0x3ce8c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3ce90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cea4 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrh` at 0x3ceac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3ceb8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3cebc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ced0 | load | 4 | `[sp-72] +0x0` | `-` |
| `ldrh` at 0x3ced8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3cef0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3cef8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x3cf08 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3cf0c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3cf18 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3cf1c | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x3cf30 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x3cf34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3cf40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3cf4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cf50 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `str` at 0x3cf68 | store | 4 | `sp -0x24` | `ret(0x41e80)` |
| `ldr` at 0x3cf6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3cf78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3cf84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cf88 | store | 4 | `sp -0x20` | `ret(0x41e80)` |
| `ldr` at 0x3cf90 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3cf98 | store | 1 | `[sp-72] +0xd` | `0x1` |
| `ldr` at 0x3cf9c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3cfa8 | load | 1 | `sp -0x35` | `-` |
| `ldrls` at 0x3cfb4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3d104 | store | 4 | `sp -0x24` | `ret(0x4e10c)` |
| `ldr` at 0x3d108 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d114 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d120 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3d124 | store | 4 | `sp -0x20` | `ret(0x4e10c)` |
| `ldr` at 0x3d128 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d140 | load | 4 | `abs 0x3d27c` | `-` |
| `str` at 0x3d150 | store | 4 | `sp -0x24` | `ret(0x51580)` |
| `ldr` at 0x3d15c | load | 4 | `abs 0x3d280` | `-` |
| `str` at 0x3d16c | store | 4 | `sp -0x24` | `ret(0x51580)` |
| `ldrb` at 0x3d188 | load | 1 | `sp -0x35` | `-` |
| `str` at 0x3d18c | store | 4 | `sp -0x5c` | `[sp-53]:1` |
| `ldr` at 0x3d190 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x3d19c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x3d1b8 | store | 4 | `sp -0x24` | `ret(0x4e10c)` |
| `ldr` at 0x3d1bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d1d4 | store | 4 | `sp -0x24` | `ret(0x4e1ac)` |
| `ldr` at 0x3d1d8 | load | 4 | `abs 0x3d248` | `-` |
| `ldr` at 0x3d1dc | load | 4 | `abs 0x2ae584` | `-` |
| `ldr` at 0x3d1e8 | load | 4 | `abs 0x3d248` | `-` |
| `ldr` at 0x3d1ec | load | 4 | `abs 0x2ae584` | `-` |
| `ldr` at 0x3d1f8 | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x3d200 | store | 1 | `[sp-72] +0xe` | `0x0` |
| `ldr` at 0x3d204 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d218 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d21c | store | 4 | `sp -0x60` | `[sp-32]` |
| `ldr` at 0x3d220 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d22c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3d230 | store | 4 | `sp -0x60` | `[sp-40]` |
| `ldr` at 0x3d234 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x3d238 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3d23c | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0x3d240 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d240 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d240 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3cc24 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3cc3c | `(unknown callee) 0x51a6c` | `ret(0x4f090)` | `?` | `?` | `[0x147fcc]` |
| `mov` at 0x3cc5c | `(unknown callee) (unknown target)` | `ret(0x51a6c)` | `?` | `?` | `[0x2ae604]` |
| `bl` at 0x3cc84 | `(unknown callee) 0x21e28` | `0x6` | `0x1082ec` | `0x2d5` | `0x0` |
| `bl` at 0x3cc94 | `(unknown callee) 0x503e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x3cd14 | `(unknown callee) 0x51aa0` | `0x2` | `?` | `?` | `ret(0x503e0)+1704` |
| `bl` at 0x3cd24 | `(unknown callee) 0x51580` | `0x32` | `[sp-76]` | `0x2` | `?` |
| `bl` at 0x3cd38 | `(unknown callee) 0x4e650` | `[sp-24]` | `?` | `?` | `ret(0x503e0)` |
| `bl` at 0x3cd6c | `(unknown callee) 0x2070c` | `0x1084c4` | `[sp-88]` | `ret(0x4e650)` | `?` |
| `bl` at 0x3cd74 | `(unknown callee) 0x51aa0` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3cd88 | `(unknown callee) 0x52770` | `0x1` | `0x0` | `?` | `ret(0x51aa0)` |
| `bl` at 0x3cda0 | `(unknown callee) 0x51580` | `0x32` | `0x1084d4` | `0x1` | `ret(0x52770)` |
| `bl` at 0x3cdbc | `(unknown callee) 0x4e1ac` | `0x5` | `?` | `[sp-72]` | `0x0` |
| `bl` at 0x3cdc8 | `(unknown callee) 0x823b0` | `ret(0x4e1ac)` | `?` | `?` | `ret(0x4e1ac)` |
| `bl` at 0x3ce50 | `(unknown callee) 0x406f4` | `0x0` | `[sp-76]` | `[[sp-72]+4]` | `[[sp-76]+2]:1` |
| `bl` at 0x3ce6c | `(unknown callee) 0x3d48c` | `0x5` | `?` | `?` | `ret(0x406f4)` |
| `bl` at 0x3ce70 | `(unknown callee) 0x823c8` | `ret(0x3d48c)` | `?` | `?` | `?` |
| `bl` at 0x3cee0 | `(unknown callee) 0x40ef8` | `?` | `[sp-72]` | `?` | `?` |
| `bl` at 0x3ceec | `(unknown callee) 0x40ef8` | `0x19` | `[sp-72]` | `?` | `?` |
| `bl` at 0x3cf04 | `(unknown callee) 0x40ef8` | `0x64` | `?` | `?` | `[[sp-72]+8]` |
| `mov` at 0x3cf28 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[[sp-72]+16]` |
| `bl` at 0x3cf60 | `(unknown callee) 0x41e80` | `0x0` | `0xff` | `0x3` | `?` |
| `bl` at 0x3cf8c | `(unknown callee) 0x823c8` | `ret(0x41e80)` | `?` | `?` | `?` |
| `bl` at 0x3d0fc | `(unknown callee) 0x4e10c` | `0x5` | `?` | `[sp-72]` | `[sp-53]:1-48` |
| `bl` at 0x3d138 | `(unknown callee) 0x74f64` | `0x1` | `?` | `?` | `[sp-32]` |
| `bl` at 0x3d148 | `(unknown callee) 0x51580` | `0x32` | `0x1084e0` | `0x0` | `?` |
| `bl` at 0x3d164 | `(unknown callee) 0x51580` | `0x32` | `0x1084ec` | `0x2` | `[sp-32]` |
| `bl` at 0x3d174 | `(unknown callee) 0x5cf84` | `0x0` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x3d17c | `(unknown callee) 0x5d108` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d184 | `(unknown callee) 0x5c7a8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d1b0 | `(unknown callee) 0x4e10c` | `0x5` | `?` | `?` | `[sp-53]:1` |
| `bl` at 0x3d1cc | `(unknown callee) 0x4e1ac` | `0x5` | `?` | `?` | `[sp-20]` |
| `mov` at 0x3d1f4 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x2ae584]` |
| `bl` at 0x3d214 | `(unknown callee) 0x824b0` | `0x0` | `?` | `[sp-72]` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3cc4c | `[0x2ae604] == 0x0` | `[0x2ae604] != 0x0` |
| `beq` at 0x3cc68 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x3cca8 | `ret(0x503e0) == 0x0` | `ret(0x503e0) != 0x0` |
| `ldrls` at 0x3ccbc | `ret(0x503e0)+1704 <= 0x4 (unsigned, 32-bit)` | `ret(0x503e0)+1704 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x3cd48 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `blt` at 0x3cdf0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3ce34 | `[([[sp-72]+4] + [sp-44])]:1 == 0x0` | `[([[sp-72]+4] + [sp-44])]:1 != 0x0` |
| `beq` at 0x3ce44 | `[[sp-76]+2]:1 == 0xff` | `[[sp-76]+2]:1 != 0xff` |
| `beq` at 0x3ce64 | `ret(0x406f4) == 0x0` | `ret(0x406f4) != 0x0` |
| `beq` at 0x3ceb4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3cf14 | `[[sp-72]+16] == 0x0` | `[[sp-72]+16] != 0x0` |
| `beq` at 0x3cf3c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3cf48 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3cf74 | `ret(0x41e80) == 0x0` | `ret(0x41e80) != 0x0` |
| `bne` at 0x3cf80 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3cfa4 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `ldrls` at 0x3cfb4 | `[sp-53]:1-48 <= 0x4c (unsigned, 32-bit)` | `[sp-53]:1-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0x3d110 | `ret(0x4e10c) == 0x0` | `ret(0x4e10c) != 0x0` |
| `bne` at 0x3d11c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x3d130 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3d198 | `[sp-53]:1 == 0x23` | `[sp-53]:1 != 0x23` |
| `beq` at 0x3d1a4 | `[sp-53]:1 == 0x3d` | `[sp-53]:1 != 0x3d` |
| `bne` at 0x3d1c4 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x3d1e4 | `[0x2ae584] == 0x0` | `[0x2ae584] != 0x0` |
| `beq` at 0x3d20c | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `bne` at 0x3d228 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d240 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3ccbc).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3cfb4).

## `powerMargin`

- `powerMargin`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e368 to 0x3ea10.
  Established. An absolute word store writes `0x5` at `0x142790` (`str` at 0x3e508). A later call to
  `0x3ea10` carries `[sp-68]`, `[sp-20]`, `[sp-40]`, `[sp-28]` (`bl` at 0x3e5ec); its zero result
  takes the continuing edge and nonzero falls through to another arm (`beq` at 0x3e600). Word
  clears include `[sp-68] +0xc` and `[sp-72] +0xc` (`str` at 0x3e648, `str` at 0x3e720, `str` at
  0x3e8f4, `str` at 0x3e930). Near the end, the zero arm of the test for `[sp-56]` writes word
  `[sp-28]` at `0x142790` (`bne` at 0x3e9a4, `str` at 0x3e9b0). Physical margin changes and the
  merged return are unproven (`ldmdb` at 0x3e9c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e378 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3e37c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3e380 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3e384 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x3e38c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x3e390 | load | 4 | `abs 0x3e9cc` | `-` |
| `ldr` at 0x3e394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e3a8 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3e3b0 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x3e3b8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x3e3c0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3e3c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e3c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e3cc | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x3e3dc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e3e0 | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x3e3e4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3e3ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e3f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e3fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e410 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrb` at 0x3e418 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e424 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e434 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e444 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3e44c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x3e450 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3e458 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3e460 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3e470 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x3e478 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3e4a0 | store | 4 | `sp -0x34` | `ret(0x1b124)` |
| `ldr` at 0x3e4a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3e4b0 | load | 4 | `abs 0x3e9d0` | `-` |
| `str` at 0x3e4b4 | store | 4 | `sp -0x64` | `0x1085ec` |
| `ldr` at 0x3e4b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3e4bc | store | 4 | `sp -0x60` | `[sp-20]` |
| `ldr` at 0x3e4c4 | load | 4 | `abs 0x3e9d4` | `-` |
| `ldr` at 0x3e4c8 | load | 4 | `abs 0x3e9d8` | `-` |
| `ldr` at 0x3e4d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3e4f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3e500 | load | 4 | `abs 0x3e9dc` | `-` |
| `str` at 0x3e508 | store | 4 | `abs 0x142790` | `0x5` |
| `str` at 0x3e518 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x3e520 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3e524 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e528 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e52c | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x3e53c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e540 | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x3e544 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3e54c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e558 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e55c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e570 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrb` at 0x3e578 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3e584 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e590 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e594 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e5a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3e5c0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3e5c8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x3e5cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3e5d0 | store | 4 | `sp -0x64` | `[sp-32]` |
| `str` at 0x3e5d8 | store | 4 | `sp -0x60` | `sp-64` |
| `ldr` at 0x3e5dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e5e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e5e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e5e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e5f4 | store | 4 | `sp -0x24` | `ret(0x3ea10)` |
| `ldr` at 0x3e5f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3e604 | load | 4 | `abs 0x3e9e0` | `-` |
| `ldr` at 0x3e608 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3e614 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3e618 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e62c | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x3e634 | load | 4 | `abs 0x3e9e4` | `-` |
| `ldr` at 0x3e638 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3e640 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3e648 | store | 4 | `[sp-68] +0xc` | `0x0` |
| `ldr` at 0x3e64c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3e658 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3e65c | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x3e664 | load | 4 | `abs 0x3e9e0` | `-` |
| `ldr` at 0x3e668 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3e674 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e678 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3e67c | load | 1 | `[sp-68] +0x25` | `-` |
| `ldr` at 0x3e680 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3e684 | load | 1 | `[sp-68] +0x24` | `-` |
| `ldr` at 0x3e688 | load | 4 | `abs 0x3e9e8` | `-` |
| `ldr` at 0x3e68c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x3e694 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e698 | load | 4 | `[sp-68] +0x20` | `-` |
| `ldr` at 0x3e6a8 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3e6ac | load | 1 | `[sp-68] +0x29` | `-` |
| `ldr` at 0x3e6b0 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3e6b4 | load | 1 | `[sp-68] +0x26` | `-` |
| `str` at 0x3e6b8 | store | 4 | `sp -0x64` | `[[sp-68]+38]:1` |
| `ldr` at 0x3e6bc | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3e6c0 | load | 1 | `[sp-68] +0x28` | `-` |
| `str` at 0x3e6c4 | store | 4 | `sp -0x60` | `[[sp-68]+40]:1` |
| `ldr` at 0x3e6c8 | load | 4 | `abs 0x3e9ec` | `-` |
| `ldr` at 0x3e6d4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e6d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e6dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e6e0 | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3e6e4 | load | 1 | `[sp-68] +0x2c` | `-` |
| `str` at 0x3e6e8 | store | 4 | `sp -0x64` | `[[sp-68]+44]:1` |
| `ldr` at 0x3e6ec | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x3e6f0 | load | 1 | `[sp-68] +0x2d` | `-` |
| `str` at 0x3e6f4 | store | 4 | `sp -0x60` | `[[sp-68]+45]:1` |
| `ldr` at 0x3e6f8 | load | 4 | `abs 0x3e9f0` | `-` |
| `ldr` at 0x3e6fc | load | 4 | `[sp-68] +0x18` | `-` |
| `ldr` at 0x3e700 | load | 4 | `[sp-68] +0x14` | `-` |
| `ldr` at 0x3e704 | load | 4 | `[sp-68] +0x30` | `-` |
| `ldr` at 0x3e70c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3e718 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3e720 | store | 4 | `[sp-68] +0xc` | `0x0` |
| `ldr` at 0x3e724 | load | 4 | `abs 0x3e9e0` | `-` |
| `ldr` at 0x3e728 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3e734 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e738 | load | 4 | `abs 0x3e9f4` | `-` |
| `ldr` at 0x3e73c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x3e744 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3e74c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x3e750 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3e758 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x3e764 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3e768 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3e77c | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x3e78c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3e790 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3e794 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3e7a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3e7c0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3e7c8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x3e7cc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e7d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3e7e4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3e7f4 | store | 4 | `sp -0x24` | `ret(0x3f22c)` |
| `ldr` at 0x3e7f8 | load | 4 | `abs 0x3e9e0` | `-` |
| `ldr` at 0x3e7fc | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3e808 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e80c | load | 4 | `abs 0x3e9f8` | `-` |
| `ldr` at 0x3e810 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3e818 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e81c | load | 4 | `[sp-72] +0x1c` | `-` |
| `stm` at 0x3e828 | store | 4 | `sp -0x54` | `ret(0xfaca8)` |
| `stm` at 0x3e828 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3e82c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e830 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e834 | load | 4 | `[sp-72] +0x18` | `-` |
| `ldr` at 0x3e838 | load | 4 | `[sp-72] +0x30` | `-` |
| `ldr` at 0x3e848 | load | 4 | `abs 0x3e9fc` | `-` |
| `str` at 0x3e84c | store | 4 | `sp -0x58` | `0x1086e8` |
| `ldr` at 0x3e854 | load | 4 | `abs 0x3ea00` | `-` |
| `str` at 0x3e858 | store | 4 | `sp -0x58` | `0x1086f0` |
| `ldr` at 0x3e85c | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3e860 | load | 1 | `[sp-72] +0x29` | `-` |
| `str` at 0x3e864 | store | 4 | `sp -0x64` | `[[sp-72]+41]:1` |
| `ldr` at 0x3e868 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3e86c | load | 1 | `[sp-72] +0x26` | `-` |
| `str` at 0x3e870 | store | 4 | `sp -0x60` | `[[sp-72]+38]:1` |
| `ldr` at 0x3e874 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3e878 | load | 1 | `[sp-72] +0x28` | `-` |
| `str` at 0x3e87c | store | 4 | `sp -0x5c` | `[[sp-72]+40]:1` |
| `ldr` at 0x3e880 | load | 4 | `abs 0x3ea04` | `-` |
| `ldm` at 0x3e888 | load | 4 | `sp -0x54` | `-` |
| `ldm` at 0x3e888 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3e88c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x3e894 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e898 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e89c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e8a0 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3e8a4 | load | 1 | `[sp-72] +0x2c` | `-` |
| `str` at 0x3e8a8 | store | 4 | `sp -0x64` | `[[sp-72]+44]:1` |
| `ldr` at 0x3e8ac | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0x3e8b0 | load | 1 | `[sp-72] +0x2d` | `-` |
| `str` at 0x3e8b4 | store | 4 | `sp -0x60` | `[[sp-72]+45]:1` |
| `ldr` at 0x3e8b8 | load | 4 | `abs 0x3e9f0` | `-` |
| `ldr` at 0x3e8bc | load | 4 | `[sp-72] +0x18` | `-` |
| `ldr` at 0x3e8c0 | load | 4 | `[sp-72] +0x14` | `-` |
| `ldr` at 0x3e8c4 | load | 4 | `[sp-72] +0x30` | `-` |
| `ldr` at 0x3e8cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3e8d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3e8e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3e8e8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3e8ec | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3e8f4 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x3e8f8 | load | 4 | `abs 0x3e9e0` | `-` |
| `ldr` at 0x3e8fc | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3e908 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e90c | load | 4 | `abs 0x3ea08` | `-` |
| `ldr` at 0x3e910 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3e91c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3e928 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3e930 | store | 4 | `[sp-72] +0xc` | `0x0` |
| `ldr` at 0x3e934 | load | 4 | `abs 0x3e9e0` | `-` |
| `ldr` at 0x3e938 | load | 4 | `abs 0x148520` | `-` |
| `ldr` at 0x3e944 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x3e948 | load | 4 | `abs 0x3ea0c` | `-` |
| `ldr` at 0x3e94c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x3e958 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3e960 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x3e964 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3e96c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3e974 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3e984 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3e98c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x3e994 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3e99c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3e9a8 | load | 4 | `abs 0x3e9dc` | `-` |
| `ldr` at 0x3e9ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e9b0 | store | 4 | `abs 0x142790` | `[sp-28]` |
| `ldr` at 0x3e9bc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3e9c0 | store | 4 | `sp -0x4c` | `[sp-56]` |
| `ldr` at 0x3e9c4 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x3e9c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e9c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e9c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e498 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0x3e4d0 | `(unknown callee) 0x21e28` | `0x9` | `0x1082ec` | `0x608` | `0x0` |
| `bl` at 0x3e4fc | `(unknown callee) 0x1b43c` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x3e510 | `(unknown callee) 0x74fbc` | `0x5` | `?` | `0x142790` | `0x5` |
| `bl` at 0x3e5ec | `(unknown callee) 0x3ea10` | `[sp-68]` | `[sp-20]` | `[sp-40]` | `[sp-28]` |
| `bl` at 0x3e63c | `(unknown callee) 0x23f2c` | `0x108610` | `?` | `?` | `?` |
| `bl` at 0x3e690 | `(unknown callee) 0x23f2c` | `0x108624` | `[[sp-68]+16]` | `[[sp-68]+37]:1` | `[[sp-68]+36]:1` |
| `bl` at 0x3e69c | `(unknown callee) 0xfaca8` | `[[sp-68]+32]` | `?` | `?` | `[sp-68]` |
| `bl` at 0x3e6d0 | `(unknown callee) 0x23f2c` | `0x108648` | `ret(0xfaca8)` | `?` | `[[sp-68]+41]:1` |
| `bl` at 0x3e708 | `(unknown callee) 0x23f2c` | `0x108684` | `[[sp-68]+24]` | `[[sp-68]+20]` | `[[sp-68]+48]` |
| `bl` at 0x3e740 | `(unknown callee) 0x23f2c` | `0x1086cc` | `[[sp-68]+16]` | `[sp-68]` | `[sp-68]` |
| `bl` at 0x3e784 | `(unknown callee) 0x40ef8` | `0x6e` | `?` | `?` | `0x0` |
| `bl` at 0x3e7ec | `(unknown callee) 0x3f22c` | `?` | `sp-64` | `?` | `sp-64` |
| `bl` at 0x3e814 | `(unknown callee) 0x23f2c` | `0x1086dc` | `[[sp-72]+16]` | `?` | `[sp-72]` |
| `bl` at 0x3e820 | `(unknown callee) 0xfaca8` | `[[sp-72]+28]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x3e890 | `(unknown callee) 0x23f2c` | `0x1086f8` | `[sp-84]` | `[sp-80]` | `[sp-88]` |
| `bl` at 0x3e8c8 | `(unknown callee) 0x23f2c` | `0x108684` | `[[sp-72]+24]` | `[[sp-72]+20]` | `[[sp-72]+48]` |
| `bl` at 0x3e914 | `(unknown callee) 0x23f2c` | `0x108738` | `[[sp-72]+16]` | `[sp-72]` | `[sp-72]` |
| `bl` at 0x3e950 | `(unknown callee) 0x23f2c` | `0x10874c` | `[[sp-72]+16]` | `[sp-72]` | `[sp-72]` |
| `bl` at 0x3e998 | `(unknown callee) 0x1b400` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x3e9b8 | `(unknown callee) 0x74fbc` | `[sp-28]` | `?` | `0x142790` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3e3d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3e3f4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3e420 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x3e42c | `[sp-24] == 0x21` | `[sp-24] != 0x21` |
| `beq` at 0x3e43c | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `bne` at 0x3e468 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3e4ac | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `blt` at 0x3e534 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3e554 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3e580 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x3e58c | `[sp-24] == 0x21` | `[sp-24] != 0x21` |
| `beq` at 0x3e59c | `[sp-24] == [sp-40]` | `[sp-24] != [sp-40]` |
| `beq` at 0x3e600 | `ret(0x3ea10) == 0x0` | `ret(0x3ea10) != 0x0` |
| `beq` at 0x3e610 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `bne` at 0x3e654 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x3e670 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `beq` at 0x3e714 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0x3e730 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `ble` at 0x3e770 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x3e79c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3e7d8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3e804 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `ble` at 0x3e844 | `? <= 0x0 (signed, 32-bit)` | `? > 0x0 (signed, 32-bit)` |
| `beq` at 0x3e8d4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3e8e0 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |
| `beq` at 0x3e904 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `beq` at 0x3e924 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0x3e940 | `[0x148520] == 0x0` | `[0x148520] != 0x0` |
| `bne` at 0x3e97c | `[sp-68] != 0x0` | `[sp-68] == 0x0` |
| `bne` at 0x3e9a4 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e9c8 | `r0` | `?` |

## `powerGetMargin`

- `powerGetMargin`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dfc0 to 0x3e304.
  Established. The first call targets `0x405a0` with entry r0 through r3 (`bl` at 0x3dfe4); its
  nonzero result takes the continuation and zero falls through to `0x4063c` (`bne` at 0x3dff8,
  `bl` at 0x3e000). The computed transfer takes an unresolved edge for `[ret(0x405a0)+8]-1 <= 0x4`
  unsigned and falls through above it (`ldrls` at 0x3e030). The known fall-through route includes a
  call to `0x21e28` with `0x9`, `0x1082ec`, `0x5cf`, `0x0` (`bl` at 0x3e2c8). Excluded output
  operations, margin encoding and the joined r0 return remain unknown (`ldmdb` at 0x3e2e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3dfd0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3dfd4 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x3dfd8 | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x3dfdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3dfe0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3dfec | store | 4 | `sp -0x28` | `ret(0x405a0)` |
| `ldr` at 0x3dff0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3dffc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e008 | store | 4 | `sp -0x34` | `ret(0x4063c)` |
| `ldr` at 0x3e010 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3e014 | load | 1 | `ret(0x405a0) +0x6` | `-` |
| `ldr` at 0x3e020 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e024 | load | 4 | `ret(0x405a0) +0x8` | `-` |
| `ldrls` at 0x3e030 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3e23c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e244 | load | 4 | `abs 0x3e2ec` | `-` |
| `ldr` at 0x3e248 | load | 4 | `abs 0x3e2f0` | `-` |
| `str` at 0x3e274 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3e278 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e280 | load | 4 | `abs 0x3e2ec` | `-` |
| `ldr` at 0x3e284 | load | 4 | `abs 0x3e2f0` | `-` |
| `ldr` at 0x3e290 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3e2a0 | load | 4 | `abs 0x3e2f4` | `-` |
| `str` at 0x3e2a4 | store | 4 | `(unknown)` | `0x1085d0` |
| `str` at 0x3e2a8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x3e2ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e2b0 | load | 4 | `[sp-40] +0x8` | `-` |
| `str` at 0x3e2b4 | store | 4 | `(unknown)` | `[[sp-40]+8]` |
| `ldr` at 0x3e2bc | load | 4 | `abs 0x3e2f8` | `-` |
| `ldr` at 0x3e2c0 | load | 4 | `abs 0x3e2fc` | `-` |
| `ldr` at 0x3e2d0 | load | 4 | `abs 0x3e300` | `-` |
| `str` at 0x3e2d4 | store | 4 | `sp -0x34` | `0xfffff9be` |
| `str` at 0x3e2e0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3e2e4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3e2e8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3e2e8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3e2e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e2e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e2e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3dfe4 | `(unknown callee) 0x405a0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3e000 | `(unknown callee) 0x4063c` | `[sp-28]` | `?` | `?` | `ret(0x405a0)` |
| `bl` at 0x3e24c | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3e258 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x3e288 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3e298 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x3e2c8 | `(unknown callee) 0x21e28` | `0x9` | `0x1082ec` | `0x5cf` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3dff8 | `ret(0x405a0) != 0x0` | `ret(0x405a0) == 0x0` |
| `beq` at 0x3e01c | `[ret(0x405a0)+6]:1 == 0xff` | `[ret(0x405a0)+6]:1 != 0xff` |
| `ldrls` at 0x3e030 | `[ret(0x405a0)+8]-1 <= 0x4 (unsigned, 32-bit)` | `[ret(0x405a0)+8]-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e2e8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3e030).

## `powerMarginAll`

- `powerMarginAll`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e304 to 0x3e368.
  Established. The call to `0x3e368` carries `0x0`, `0x21`, entry r0, `0x0` (`bl` at 0x3e328); its
  nonzero result takes the bypass and zero falls through to write word `[sp-20]` at absolute
  `0x142790` (`bne` at 0x3e33c, `str` at 0x3e348). That zero arm calls `0x74fbc` with `[sp-20]`,
  unknown, `0x142790`, `[sp-20]` (`bl` at 0x3e350). The aggregate margin behavior and returned r0
  value are unknown (`ldmdb` at 0x3e35c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e314 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3e320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3e324 | load | 4 | `abs 0x3e360` | `-` |
| `str` at 0x3e330 | store | 4 | `sp -0x18` | `ret(0x3e368)` |
| `ldr` at 0x3e334 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e340 | load | 4 | `abs 0x3e364` | `-` |
| `ldr` at 0x3e344 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3e348 | store | 4 | `abs 0x142790` | `[sp-20]` |
| `ldr` at 0x3e354 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3e35c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e35c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e35c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e328 | `(unknown callee) 0x3e368` | `0x0` | `0x21` | `r0` | `0x0` |
| `bl` at 0x3e350 | `(unknown callee) 0x74fbc` | `[sp-20]` | `?` | `0x142790` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3e33c | `ret(0x3e368) != 0x0` | `ret(0x3e368) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e35c | `r0` | `?` |

## `powerMarginStatusAll`

- `powerMarginStatusAll`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d8c0 to 0x3d8e0.
  Established. A word read addresses absolute `0x142790` (`ldr` at 0x3d8d0); its value moves to r0
  and is returned as `[0x142790]` (`mov` at 0x3d8d4, `ldmdb` at 0x3d8d8). No explicit stores, calls
  or conditional branches are reported. The status encoding and relationship to physical margins
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3d8cc | load | 4 | `abs 0x3d8dc` | `-` |
| `ldr` at 0x3d8d0 | load | 4 | `abs 0x142790` | `-` |
| `ldmdb` at 0x3d8d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d8d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d8d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d8d8 | `r0` | `[0x142790]` |

## `powerGetName`

- `powerGetName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3fff0 to 0x400e0.
  Established. The first indexed word read has an unknown effective target (`ldr` at 0x4004c); its
  zero test takes the alternate route for `? == 0x0` and falls through on inequality (`beq` at
  0x40054). The two alternative word stores at `sp -0x20` both have unknown values (`str` at
  0x4007c, `str` at 0x400cc). The r0 return is `[sp-32]`, while the name object and referenced table
  remain unidentified (`ldmdb` at 0x400d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40000 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40004 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x40008 | load | 4 | `abs 0x400d8` | `-` |
| `ldr` at 0x4000c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40020 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x40028 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4002c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40044 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x4004c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x40058 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4005c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40070 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x40078 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4007c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x40084 | load | 4 | `abs 0x400dc` | `-` |
| `ldr` at 0x40088 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4008c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x400a0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x400a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x400c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x400cc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x400d0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x400d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x400d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x400d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x40054 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x400d4 | `r0` | `[sp-32]` |

## `powerMarginName`

- `powerMarginName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x400e0 to 0x401f0.
  Established. The first call reaches `0x405a0` with entry r0 through r3 (`bl` at 0x40100). The
  later call to `0x40e88` has `[sp-20]`, `ret(0x405a0)`, unknown, `[ret(0x405a0)+6]:1` (`bl` at
  0x4013c); its computed transfer takes an unresolved edge for `ret(0x40e88) <= 0x5` unsigned and
  falls through for greater (`ldrls` at 0x40148). On known paths, word stores at `sp -0x20` include
  `0x108948`, `0x108978`, and `0x108980` (`str` at 0x4011c, `str` at 0x401b4, `str` at 0x401c0).
  Excluded names, their interpretation and the joined r0 value remain unknown (`ldmdb` at 0x401c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x400f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x400f4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x400f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x400fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x40108 | store | 4 | `sp -0x1c` | `ret(0x405a0)` |
| `ldr` at 0x4010c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40118 | load | 4 | `abs 0x401cc` | `-` |
| `str` at 0x4011c | store | 4 | `sp -0x20` | `0x108948` |
| `ldr` at 0x40124 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x40128 | load | 1 | `ret(0x405a0) +0x6` | `-` |
| `ldr` at 0x40134 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40138 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x40148 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x401b0 | load | 4 | `abs 0x401e8` | `-` |
| `str` at 0x401b4 | store | 4 | `sp -0x20` | `0x108978` |
| `ldr` at 0x401bc | load | 4 | `abs 0x401ec` | `-` |
| `str` at 0x401c0 | store | 4 | `sp -0x20` | `0x108980` |
| `ldr` at 0x401c4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x401c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x401c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x401c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40100 | `(unknown callee) 0x405a0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4013c | `(unknown callee) 0x40e88` | `[sp-20]` | `ret(0x405a0)` | `?` | `[ret(0x405a0)+6]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x40114 | `ret(0x405a0) != 0x0` | `ret(0x405a0) == 0x0` |
| `beq` at 0x40130 | `[ret(0x405a0)+6]:1 == 0xff` | `[ret(0x405a0)+6]:1 != 0xff` |
| `ldrls` at 0x40148 | `ret(0x40e88) <= 0x5 (unsigned, 32-bit)` | `ret(0x40e88) > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x401c8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x40148).

## `powerLowMargin`

- `powerLowMargin`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40470 to 0x40498.
  Established. A byte read addresses entry `r1 +0xf` (`ldrb` at 0x4048c). The loaded value supplies
  the r0 return `[r1+15]:1` (`ldmdb` at 0x40494). The two explicit stores retain entry r0 and r1 as
  words at `sp -0x14` and `sp -0x18` (`str` at 0x40480, `str` at 0x40484). Calls and conditional
  branches are absent, and the physical low-margin setting is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40480 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40484 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x40488 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4048c | load | 1 | `r1 +0xf` | `-` |
| `ldmdb` at 0x40494 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40494 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40494 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40494 | `r0` | `[r1+15]:1` |

## `powerNormalMargin`

- `powerNormalMargin`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40498 to 0x404c0.
  Established. Entry r1 is saved as a word at `sp -0x18` before a byte load from entry `r1 +0xd`
  (`str` at 0x404ac, `ldrb` at 0x404b4). That byte reaches the r0 return as `[r1+13]:1` (`ldmdb` at
  0x404bc). There are no calls or conditional branches in the facts. The nominal-margin field
  meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x404a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x404ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x404b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x404b4 | load | 1 | `r1 +0xd` | `-` |
| `ldmdb` at 0x404bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x404bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x404bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x404bc | `r0` | `[r1+13]:1` |

## `powerHighMargin`

- `powerHighMargin`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x404c0 to 0x404e8.
  Established. The explicit byte load reads entry `r1 +0xe` (`ldrb` at 0x404dc). The r0 return
  preserves the loaded value `[r1+14]:1` (`ldmdb` at 0x404e4). An earlier word store saves entry r0
  at `sp -0x14` (`str` at 0x404d0). No calls are reported, and the physical high-margin value and
  units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x404d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x404d4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x404d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x404dc | load | 1 | `r1 +0xe` | `-` |
| `ldmdb` at 0x404e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x404e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x404e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x404e4 | `r0` | `[r1+14]:1` |

## `powerGetIndexOfConfig`

- `powerGetIndexOfConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x404e8 to 0x405a0.
  Established. The loop test retains a less-than flags predicate with unknown operands, following its
  edge when true and falling through otherwise (`blt` at 0x4053c). The inner comparison takes its
  continuing edge for `? != [sp-24]` and falls through on equality (`bne` at 0x4056c). The equality
  arm stores word `[sp-28]` at `sp -0x24`, while the later fallback stores an unknown word there
  (`str` at 0x40574, `str` at 0x40590). The return reports `[sp-36]` (`ldmdb` at 0x40598). The lookup index meaning is
  unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x404f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x404fc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x40500 | load | 4 | `abs 0x4059c` | `-` |
| `ldr` at 0x40504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40518 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x40520 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x40528 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4052c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x40530 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40534 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x40544 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x40548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4055c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x40564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40570 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40574 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x4057c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40584 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x40590 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x40594 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x40598 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40598 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x4053c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x4056c | `? != [sp-24]` | `? == [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40598 | `r0` | `[sp-36]` |

## `powerResetStart`

- `powerResetStart`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x407c4 to 0x407dc.
  Established. The only explicit store saves entry r0 as a word at `sp -0x14` (`str` at 0x407d4).
  The body restores frame registers at the return, where r0 remains the entry r0 (`ldmdb` at
  0x407d8). No calls or conditional branches are reported, and a physical reset-start action is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x407d4 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0x407d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x407d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x407d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x407d8 | `r0` | `r0` |

## `powerUpInProgress`

- `powerUpInProgress`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40820 to 0x40864.
  Established. The absolute word at `0x147fcc` is loaded before an indexed byte read whose target is
  unknown (`ldr` at 0x4084c, `ldrb` at 0x40854). The reported r0 return is unknown (`ldmdb` at
  0x4085c). The only explicit store saves entry r0 as a word at `sp -0x14` (`str` at 0x40830).
  Physical power-up progress and the loaded byte interpretation remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40830 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x40834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40848 | load | 4 | `abs 0x40860` | `-` |
| `ldr` at 0x4084c | load | 4 | `abs 0x147fcc` | `-` |
| `ldrb` at 0x40854 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x4085c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4085c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4085c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4085c | `r0` | `?` |

## `powerUpFirstReset`

- `powerUpFirstReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40864 to 0x408a8.
  Established. A word read addresses absolute `0x147fcc` (`ldr` at 0x40890). The subsequent indexed
  byte load has an unknown effective target and supplies an unknown r0 return (`ldrb` at 0x40898,
  `ldmdb` at 0x408a0). Calls and conditional branches are absent. The first-reset state and its
  encoding are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40874 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x40878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4088c | load | 4 | `abs 0x408a4` | `-` |
| `ldr` at 0x40890 | load | 4 | `abs 0x147fcc` | `-` |
| `ldrb` at 0x40898 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x408a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x408a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x408a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x408a0 | `r0` | `?` |

## `powerSetMarginState`

- `powerSetMarginState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40ec0 to 0x40ef8.
  Established. The byte load addresses entry `r1 +0x10` (`ldrb` at 0x40ee0). The only call targets
  `0x1d38c` with `[r1+16]:1+20`, entry r2, entry r2, `[r1+16]:1+20` (`bl` at 0x40ef0); its result
  reaches the r0 return (`ldmdb` at 0x40ef4). Physical margin-state changes and the callee argument
  count remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40ed0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40ed4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x40ed8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x40edc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x40ee0 | load | 1 | `r1 +0x10` | `-` |
| `ldr` at 0x40eec | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x40ef4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40ef4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40ef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40ef0 | `(unknown callee) 0x1d38c` | `[r1+16]:1+20` | `r2` | `r2` | `[r1+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40ef4 | `r0` | `ret(0x1d38c)` |
