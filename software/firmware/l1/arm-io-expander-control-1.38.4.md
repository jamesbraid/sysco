# ARM I/O-expander control paths in 1.38.4

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
Reset, preconfiguration and interrupt-restoration selectors have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/io-expander-control-facts.txt) retain all tool sections.

## `ioExpInit`

- `ioExpInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5b60 to 0xd5e04.
  Established. On the zero arm of the absolute-word test, a store writes word `0x1` at `0x2ad220`
  (`bne` at 0xd5bd8, `str` at 0xd5be4). The call to `0x1b3c4` has `[sp-44]`, `0x1`, `[0x2ad220]`,
  unknown, with its result stored as a word at `0x2ad21c` (`bl` at 0xd5c10, `str` at 0xd5c18). Later
  byte stores write `0x0` and `0x1` at unknown targets (`strb` at 0xd5da0, `strb` at 0xd5dcc). Their
  selector branch only retains `Z == 0`, following its edge when true and falling through otherwise
  (`bne` at 0xd5d78). Device initialization, indexed layout and the final r0 value remain unknown
  (`ldmdb` at 0xd5de0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5b70 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd5b74 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd5b78 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd5b7c | load | 4 | `abs 0xd5de4` | `-` |
| `ldr` at 0xd5b80 | load | 4 | `abs 0x2ad21c` | `-` |
| `ldr` at 0xd5b8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5b98 | load | 4 | `abs 0xd5de8` | `-` |
| `str` at 0xd5b9c | store | 4 | `sp -0x38` | `0x1346dc` |
| `ldr` at 0xd5ba0 | load | 4 | `abs 0xd5de4` | `-` |
| `ldr` at 0xd5ba4 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd5ba8 | store | 4 | `sp -0x34` | `[0x2ad21c]` |
| `ldr` at 0xd5bac | load | 4 | `abs 0xd5dec` | `-` |
| `ldr` at 0xd5bb0 | load | 4 | `abs 0x2ad220` | `-` |
| `str` at 0xd5bb4 | store | 4 | `sp -0x30` | `[0x2ad220]` |
| `ldr` at 0xd5bbc | load | 4 | `abs 0xd5df0` | `-` |
| `ldr` at 0xd5bcc | load | 4 | `abs 0xd5dec` | `-` |
| `ldr` at 0xd5bd0 | load | 4 | `abs 0x2ad220` | `-` |
| `ldr` at 0xd5bdc | load | 4 | `abs 0xd5dec` | `-` |
| `str` at 0xd5be4 | store | 4 | `abs 0x2ad220` | `0x1` |
| `ldr` at 0xd5be8 | load | 4 | `abs 0xd5dec` | `-` |
| `ldr` at 0xd5bec | load | 4 | `abs 0x2ad220` | `-` |
| `str` at 0xd5c00 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd5c04 | load | 4 | `abs 0xd5de4` | `-` |
| `ldr` at 0xd5c08 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd5c18 | store | 4 | `abs 0x2ad21c` | `ret(0x1b3c4)` |
| `ldr` at 0xd5c1c | load | 4 | `abs 0x2ad21c` | `-` |
| `ldr` at 0xd5c28 | load | 4 | `abs 0xd5df4` | `-` |
| `str` at 0xd5c2c | store | 4 | `sp -0x38` | `0x134728` |
| `ldr` at 0xd5c30 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd5c34 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xd5c3c | load | 4 | `abs 0xd5df0` | `-` |
| `ldr` at 0xd5c4c | load | 4 | `abs 0xd5de4` | `-` |
| `ldr` at 0xd5c50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5c64 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd5c6c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd5c70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5c74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5c78 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xd5c7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5c80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5c84 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xd5c88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5c8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5c90 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xd5ca0 | store | 4 | `[sp-36] +0x8` | `ret(0x1b3c4)` |
| `ldr` at 0xd5ca4 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xd5cb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5cb8 | load | 4 | `abs 0xd5df8` | `-` |
| `ldr` at 0xd5cbc | load | 4 | `abs 0xd5dfc` | `-` |
| `str` at 0xd5ce8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd5cec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5cf4 | load | 4 | `abs 0xd5df8` | `-` |
| `ldr` at 0xd5cf8 | load | 4 | `abs 0xd5dfc` | `-` |
| `ldr` at 0xd5d04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd5d14 | load | 4 | `abs 0xd5e00` | `-` |
| `str` at 0xd5d18 | store | 4 | `(unknown)` | `0x13476c` |
| `str` at 0xd5d1c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xd5d20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5d24 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd5d2c | load | 4 | `abs 0xd5df0` | `-` |
| `str` at 0xd5d40 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd5d44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5d48 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5d4c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xd5d5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5d60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5d68 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0xd5d70 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd5d7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5d80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5d94 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd5da0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xd5da8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5dac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5dc0 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd5dcc | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xd5dd0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd5dd8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0xd5de0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd5de0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5de0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5de0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5bc8 | `(unknown callee) 0x21e28` | `0x9` | `0x13470c` | `0x54` | `0x0` |
| `bl` at 0xd5c10 | `(unknown callee) 0x1b3c4` | `[sp-44]` | `0x1` | `[0x2ad220]` | `?` |
| `bl` at 0xd5c48 | `(unknown callee) 0x21e28` | `0x9` | `0x13470c` | `0x60` | `0x0` |
| `bl` at 0xd5c98 | `(unknown callee) 0x1b3c4` | `[[sp-36]+4]` | `0xc` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xd5cc0 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd5ccc | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd5cfc | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd5d0c | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd5d38 | `(unknown callee) 0x21e28` | `0x9` | `0x13470c` | `0x6c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd5b88 | `[0x2ad21c] == 0x0` | `[0x2ad21c] != 0x0` |
| `bne` at 0xd5b94 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd5bd8 | `[0x2ad220] != 0x0` | `[0x2ad220] == 0x0` |
| `bne` at 0xd5c24 | `[0x2ad21c] != 0x0` | `[0x2ad21c] == 0x0` |
| `bne` at 0xd5cac | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `blt` at 0xd5d54 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd5d78 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5de0 | `r0` | `?` |

## `ioExpPresent`

- `ioExpPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5e04 to 0xd5f28.
  Established. The signed guard takes its alternate path for `(r1 & 0xff) >= [[sp-28]+4]` and falls
  through for less (`bge` at 0xd5e54). The fall-through arm stores byte `0x1` at an unknown
  effective target (`strb` at 0xd5e7c). The other path includes a call to `0x21e28` with `0x6`,
  `0x13470c`, `0x82`, `0x0` (`bl` at 0xd5f0c). Physical presence semantics and the joined r0 return
  remain unproven (`ldmdb` at 0xd5f10).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5e14 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd5e1c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd5e20 | load | 4 | `abs 0xd5f14` | `-` |
| `ldr` at 0xd5e24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5e38 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd5e40 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd5e44 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5e48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5e4c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd5e58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd5e5c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5e70 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd5e7c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xd5e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5e8c | load | 4 | `abs 0xd5f18` | `-` |
| `ldr` at 0xd5e90 | load | 4 | `abs 0xd5f1c` | `-` |
| `str` at 0xd5ebc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd5ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5ec8 | load | 4 | `abs 0xd5f18` | `-` |
| `ldr` at 0xd5ecc | load | 4 | `abs 0xd5f1c` | `-` |
| `ldr` at 0xd5ed8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd5ee8 | load | 4 | `abs 0xd5f20` | `-` |
| `str` at 0xd5eec | store | 4 | `(unknown)` | `0x1347a4` |
| `str` at 0xd5ef0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd5ef4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd5ef8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd5f00 | load | 4 | `abs 0xd5f24` | `-` |
| `ldmdb` at 0xd5f10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5f10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5f10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5e94 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd5ea0 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd5ed0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd5ee0 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd5f0c | `(unknown callee) 0x21e28` | `0x6` | `0x13470c` | `0x82` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd5e54 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5f10 | `r0` | `?` |

## `ioExpNotPresent`

- `ioExpNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5f28 to 0xd604c.
  Established. The first bound takes the alternate path for `(r1 & 0xff) >= [[sp-28]+4]` signed and
  falls through for less (`bge` at 0xd5f78); its fall-through byte store writes `0x0` at an unknown
  target (`strb` at 0xd5fa0). The alternate path reaches `0x21e28` with `0x6`, `0x13470c`, `0x91`,
  `0x0` (`bl` at 0xd6030). Device absence, referenced byte layout and the returned r0 value are
  unknown (`ldmdb` at 0xd6034).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5f38 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd5f40 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd5f44 | load | 4 | `abs 0xd6038` | `-` |
| `ldr` at 0xd5f48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5f5c | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd5f64 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd5f68 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5f6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5f70 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd5f7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd5f80 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5f94 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd5fa0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xd5fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5fb0 | load | 4 | `abs 0xd603c` | `-` |
| `ldr` at 0xd5fb4 | load | 4 | `abs 0xd6040` | `-` |
| `str` at 0xd5fe0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd5fe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5fec | load | 4 | `abs 0xd603c` | `-` |
| `ldr` at 0xd5ff0 | load | 4 | `abs 0xd6040` | `-` |
| `ldr` at 0xd5ffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd600c | load | 4 | `abs 0xd6044` | `-` |
| `str` at 0xd6010 | store | 4 | `(unknown)` | `0x1347c8` |
| `str` at 0xd6014 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd6018 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd601c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd6024 | load | 4 | `abs 0xd6048` | `-` |
| `ldmdb` at 0xd6034 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6034 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6034 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5fb8 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd5fc4 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd5ff4 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd6004 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd6030 | `(unknown callee) 0x21e28` | `0x6` | `0x13470c` | `0x91` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd5f78 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6034 | `r0` | `?` |

## `ioExpReset`

- `ioExpReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd604c to 0xd621c.
  Established. The computed transfer takes an unresolved edge for `[[sp-28]]:1 <= 0x4` unsigned and
  falls through above that bound (`ldrls` at 0xd60d0). On known paths, byte stores write
  `([[sp-32]+3]:1 & 0xff)` at `[sp-32] +0x4`, `0x5` at `[sp-32] +0x5`, and `0x0` at `[sp-32] +0x3`
  (`strb` at 0xd61c8, `strb` at 0xd61d4, `strb` at 0xd61e0). A later call to `0xd7e6c` carries
  `[sp-20]`, `[sp-21]:1`, `[[sp-28]+7]:1`, `0x1` (`bl` at 0xd620c). Excluded reset cases, hardware
  effects and the joined r0 value remain unknown (`ldmdb` at 0xd6218).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd605c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6064 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd6068 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd606c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd607c | store | 4 | `sp -0x1c` | `ret(0xd75c0)` |
| `ldrb` at 0xd6080 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6084 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6094 | store | 4 | `sp -0x20` | `ret(0xd76d4)` |
| `ldr` at 0xd6098 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd609c | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `ldr` at 0xd60a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd60ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd60b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd60b4 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0xd60b8 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xd60c0 | store | 1 | `ret(0xd76d4) +0x1` | `?` |
| `ldr` at 0xd60c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd60c8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd60d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd6190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd619c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd61a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd61a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd61b8 | store | 4 | `sp -0x24` | `ret(0xd7c60)` |
| `ldr` at 0xd61bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd61c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd61c4 | load | 1 | `[sp-32] +0x3` | `-` |
| `strb` at 0xd61c8 | store | 1 | `[sp-32] +0x4` | `([[sp-32]+3]:1 & 0xff)` |
| `ldr` at 0xd61cc | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xd61d4 | store | 1 | `[sp-32] +0x5` | `0x5` |
| `ldr` at 0xd61d8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xd61e0 | store | 1 | `[sp-32] +0x3` | `0x0` |
| `ldr` at 0xd61e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd61f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd61f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd61f8 | load | 1 | `[sp-28] +0x7` | `-` |
| `ldr` at 0xd61fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6214 | store | 4 | `sp -0x24` | `ret(0xd7e6c)` |
| `ldmdb` at 0xd6218 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6218 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6218 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6074 | `(unknown callee) 0xd75c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd608c | `(unknown callee) 0xd76d4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd61b0 | `(unknown callee) 0xd7c60` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |
| `bl` at 0xd620c | `(unknown callee) 0xd7e6c` | `[sp-20]` | `[sp-21]:1` | `[[sp-28]+7]:1` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd60a4 | `[ret(0xd76d4)]:1 == 0x0` | `[ret(0xd76d4)]:1 != 0x0` |
| `ldrls` at 0xd60d0 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0xd6198 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0xd61ec | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6218 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd60d0).

## `ioExpPreConfig`

- `ioExpPreConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd621c to 0xd6340.
  Established. The call to `0xd76d4` has `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1` (`bl` at
  0xd6264); its zero byte test takes the bypass for `[ret(0xd76d4)]:1 == 0x0` and falls through
  otherwise (`beq` at 0xd627c). The computed transfer takes an unresolved edge for `[[sp-28]]:1 <=
  0x4` unsigned and falls through above it (`ldrls` at 0xd62a8). The later known-edge call to
  `0xd7c60` uses `[sp-20]`, `[sp-28]`, `[sp-32]+2`, `[sp-32]+2` (`bl` at 0xd6324). Excluded
  configuration actions and the merged return are unproven (`ldmdb` at 0xd6330).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd622c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6234 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0xd623c | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xd6240 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6244 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6254 | store | 4 | `sp -0x1c` | `ret(0xd75c0)` |
| `ldrb` at 0xd6258 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd625c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd626c | store | 4 | `sp -0x20` | `ret(0xd76d4)` |
| `ldr` at 0xd6270 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd6274 | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `ldr` at 0xd6280 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6284 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6288 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd628c | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0xd6290 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xd6298 | store | 1 | `ret(0xd76d4) +0x1` | `?` |
| `ldr` at 0xd629c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd62a0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd62a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd6304 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6310 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6318 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd631c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd632c | store | 4 | `sp -0x24` | `ret(0xd7c60)` |
| `ldmdb` at 0xd6330 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6330 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd624c | `(unknown callee) 0xd75c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd6264 | `(unknown callee) 0xd76d4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd6324 | `(unknown callee) 0xd7c60` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd627c | `[ret(0xd76d4)]:1 == 0x0` | `[ret(0xd76d4)]:1 != 0x0` |
| `ldrls` at 0xd62a8 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0xd630c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6330 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd62a8).

## `ioExpHardReset`

- `ioExpHardReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6340 to 0xd63c8.
  Established. The opening call targets `0xd88d8` with entry r0, `0x2ad21c`, unknown, unknown (`bl`
  at 0xd637c). A byte store initializes `sp -0x19` to `0x0` (`strb` at 0xd6384). A later store
  there writes `([sp-25]:1+1 & 0xff)` (`strb` at 0xd63b8). The loop call to `0xd604c` has
  `[sp-20]`, unknown, `[[sp-24]+4]`, unknown (`bl` at 0xd63ac). Physical hard-reset ordering and the
  r0 return remain unknown (`ldmdb` at 0xd63c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6350 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd6354 | load | 4 | `abs 0xd63c4` | `-` |
| `ldr` at 0xd6358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd636c | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd6374 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd6378 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xd6384 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xd6388 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd638c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6390 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd63a0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd63a4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd63b0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd63b8 | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0xd63c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd63c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd63c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd637c | `(unknown callee) 0xd88d8` | `r0` | `0x2ad21c` | `?` | `?` |
| `bl` at 0xd63ac | `(unknown callee) 0xd604c` | `[sp-20]` | `?` | `[[sp-24]+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd6398 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd63c0 | `r0` | `?` |

## `ioExpSoftResetData`

- `ioExpSoftResetData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd63c8 to 0xd65c8.
  Established. The first call reaches `0xc77fc` with entry r0, `0x2ad21c`, unknown, unknown (`bl` at
  0xd6400); its zero result takes the continuation and nonzero falls through to `0x21674` (`beq`
  at 0xd6414, `bl` at 0xd641c). On continuation, the call to `0xd7c60` has `[sp-24]`, `[sp-28]`,
  `ret(0xd76d4)+2`, `ret(0xd76d4)+2` (`bl` at 0xd64c4). A later byte store writes an unknown value
  at `[sp-32] +0x1` (`strb` at 0xd6588). Physical restart state and the merged r0 value are unproven
  (`ldmdb` at 0xd65a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd63d8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xd63dc | load | 4 | `abs 0xd65ac` | `-` |
| `ldr` at 0xd63e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd63f4 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd63fc | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xd6408 | store | 4 | `sp -0x28` | `ret(0xc77fc)` |
| `ldr` at 0xd640c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6418 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6424 | load | 4 | `abs 0xd65b0` | `-` |
| `str` at 0xd6428 | store | 4 | `sp -0x3c` | `0x134838` |
| `str` at 0xd642c | store | 4 | `sp -0x38` | `ret(0x21674)` |
| `ldr` at 0xd6434 | load | 4 | `abs 0xd65b4` | `-` |
| `ldr` at 0xd6438 | load | 4 | `abs 0xd65b8` | `-` |
| `str` at 0xd6448 | store | 4 | `sp -0x34` | `?` |
| `strb` at 0xd6454 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xd6458 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd645c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd6460 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0xd6470 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd6474 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd6484 | store | 4 | `sp -0x1c` | `ret(0xd75c0)` |
| `ldrb` at 0xd6488 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd648c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd649c | store | 4 | `sp -0x20` | `ret(0xd76d4)` |
| `ldr` at 0xd64a0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd64a4 | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `ldr` at 0xd64b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd64b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd64bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd64cc | store | 4 | `sp -0x28` | `ret(0xd7c60)` |
| `ldr` at 0xd64d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd64dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd64e4 | load | 4 | `abs 0xd65bc` | `-` |
| `ldr` at 0xd64e8 | load | 4 | `abs 0xd65c0` | `-` |
| `str` at 0xd6514 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd6518 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6520 | load | 4 | `abs 0xd65bc` | `-` |
| `ldr` at 0xd6524 | load | 4 | `abs 0xd65c0` | `-` |
| `ldr` at 0xd6530 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd6540 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd654c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6550 | load | 4 | `abs 0xd65c4` | `-` |
| `ldr` at 0xd6558 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd6568 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd6570 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6574 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6578 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd657c | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xd6580 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xd6588 | store | 1 | `[sp-32] +0x1` | `?` |
| `ldrb` at 0xd658c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd6594 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `str` at 0xd65a0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd65a4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xd65a8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd65a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd65a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd65a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6400 | `(unknown callee) 0xc77fc` | `r0` | `0x2ad21c` | `?` | `?` |
| `bl` at 0xd641c | `(unknown callee) 0x21674` | `ret(0xc77fc)` | `?` | `?` | `ret(0xc77fc)` |
| `bl` at 0xd6440 | `(unknown callee) 0x21e28` | `0x1` | `0x13470c` | `0x13d` | `0x0` |
| `bl` at 0xd647c | `(unknown callee) 0xd75c0` | `[sp-24]` | `?` | `[[sp-44]+4]` | `?` |
| `bl` at 0xd6494 | `(unknown callee) 0xd76d4` | `[sp-24]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xd64c4 | `(unknown callee) 0xd7c60` | `[sp-24]` | `[sp-28]` | `ret(0xd76d4)+2` | `ret(0xd76d4)+2` |
| `bl` at 0xd64ec | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd64f8 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd6528 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd6538 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd6544 | `(unknown callee) 0x21674` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd6560 | `(unknown callee) 0x2070c` | `0x134864` | `ret(0x1b508)` | `[[sp-28]+12]` | `ret(0x21674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd6414 | `ret(0xc77fc) == 0x0` | `ret(0xc77fc) != 0x0` |
| `blt` at 0xd6468 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd64ac | `[ret(0xd76d4)]:1 == 0x0` | `[ret(0xd76d4)]:1 != 0x0` |
| `beq` at 0xd64d8 | `ret(0xd7c60) == 0x0` | `ret(0xd7c60) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd65a8 | `r0` | `?` |

## `ioExpSoftResetInt`

- `ioExpSoftResetInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd65c8 to 0xd687c.
  Established. The computed transfer takes an unresolved edge for `[[sp-24]]:1-1 <= 0x3` unsigned
  and falls through for greater (`ldrls` at 0xd6670). The known default route stores byte
  `([ret(0xd76d4)+3]:1 & 0xff)` at `ret(0xd76d4) +0x4`, byte `0x5` at `[sp-28] +0x5`, and byte
  `([[sp-24]+7]:1 & 0xff)` at `[sp-28] +0x3` (`strb` at 0xd66f8, `strb` at 0xd6704, `strb` at
  0xd6714). The indirect call has unknown target with `[sp-20]`, `[sp-24]`, `[sp-28]+3`, `[sp-28]+3`
  (`mov` at 0xd6740); its return address is `0xd6744` (`mov` at 0xd673c). Excluded interrupt
  restoration, physical reset effects and the joined r0 return remain unknown (`ldmdb` at 0xd6858).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd65d8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd65dc | load | 4 | `abs 0xd685c` | `-` |
| `ldr` at 0xd65e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd65f4 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd65fc | store | 4 | `sp -0x28` | `?` |
| `strb` at 0xd6604 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd6608 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd660c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6610 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0xd6620 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd6624 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6634 | store | 4 | `sp -0x18` | `ret(0xd75c0)` |
| `ldrb` at 0xd6638 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd663c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd664c | store | 4 | `sp -0x1c` | `ret(0xd76d4)` |
| `ldr` at 0xd6650 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6654 | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `ldr` at 0xd6660 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd6664 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xd6670 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd66e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd66ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd66f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd66f4 | load | 1 | `ret(0xd76d4) +0x3` | `-` |
| `strb` at 0xd66f8 | store | 1 | `ret(0xd76d4) +0x4` | `([ret(0xd76d4)+3]:1 & 0xff)` |
| `ldr` at 0xd66fc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xd6704 | store | 1 | `[sp-28] +0x5` | `0x5` |
| `ldr` at 0xd6708 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd670c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd6710 | load | 1 | `[sp-24] +0x7` | `-` |
| `strb` at 0xd6714 | store | 1 | `[sp-28] +0x3` | `([[sp-24]+7]:1 & 0xff)` |
| `ldr` at 0xd6718 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6724 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd672c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6734 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd6754 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd675c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6760 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xd6764 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd6768 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd677c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6784 | load | 4 | `abs 0xd6870` | `-` |
| `ldr` at 0xd6788 | load | 4 | `abs 0xd6874` | `-` |
| `str` at 0xd67b4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd67b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd67c0 | load | 4 | `abs 0xd6870` | `-` |
| `ldr` at 0xd67c4 | load | 4 | `abs 0xd6874` | `-` |
| `ldr` at 0xd67d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd67e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd67e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd67e8 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xd67ec | load | 4 | `abs 0xd6878` | `-` |
| `ldr` at 0xd67f4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd6800 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd6808 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd680c | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xd6818 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd681c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xd6828 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd682c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd683c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd6844 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0xd6850 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd6854 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd6858 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6858 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd662c | `(unknown callee) 0xd75c0` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0xd6644 | `(unknown callee) 0xd76d4` | `[sp-20]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `mov` at 0xd6740 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-24]` | `[sp-28]+3` | `[sp-28]+3` |
| `bl` at 0xd678c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd6798 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd67c8 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134760` | `0x134768` |
| `bl` at 0xd67d8 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd67f8 | `(unknown callee) 0x2070c` | `0x13488c` | `ret(0x1b508)` | `[[sp-24]+12]` | `[[sp-28]+3]:1` |
| `bl` at 0xd6838 | `(unknown callee) 0xd6cf8` | `[sp-20]` | `[sp-29]:1` | `0xff` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd6618 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd665c | `[ret(0xd76d4)]:1 == 0x0` | `[ret(0xd76d4)]:1 != 0x0` |
| `ldrls` at 0xd6670 | `[[sp-24]]:1-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-24]]:1-1 > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xd6720 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xd674c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0xd6778 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd6814 | `[[sp-28]+3]:1 == 0x0` | `[[sp-28]+3]:1 != 0x0` |
| `bne` at 0xd6824 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6858 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd6670).

## `ioExpGetCfg`

- `ioExpGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd687c to 0xd68dc.
  Established. A word store writes `[[sp-28]]` at entry `r1 +0x0` (`str` at 0xd68c4). The following
  word read addresses `[sp-28] +0x4` and supplies the r0 return `[[sp-28]+4]` (`ldr` at 0xd68cc,
  `ldmdb` at 0xd68d4). The facts report no calls or conditional branches. Configuration-field
  meanings and the output object remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd688c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd6890 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd6894 | load | 4 | `abs 0xd68d8` | `-` |
| `ldr` at 0xd6898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd68ac | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd68b4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd68b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd68bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd68c0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd68c4 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd68c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd68cc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd68d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd68d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd68d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd68d4 | `r0` | `[[sp-28]+4]` |

## `ioExpCount`

- `ioExpCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd68dc to 0xd6928.
  Established. The absolute word at `0x2ad21c` is loaded before the final word read (`ldr` at
  0xd6908). That read addresses `[sp-24] +0x4`, and its value reaches r0 as `[[sp-24]+4]` (`ldr` at
  0xd6918, `ldmdb` at 0xd6920). No calls or conditional branches are reported. Count units and the
  referenced record remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd68ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd68f0 | load | 4 | `abs 0xd6924` | `-` |
| `ldr` at 0xd68f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6908 | load | 4 | `abs 0x2ad21c` | `-` |
| `str` at 0xd6910 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd6914 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6918 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0xd6920 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6920 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6920 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6920 | `r0` | `[[sp-24]+4]` |

## `ioExpIsPresent`

- `ioExpIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6928 to 0xd6988.
  Established. The only call reaches `0xd76d4` with entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)`
  (`bl` at 0xd6950); its nonzero result takes the read arm and zero falls through to store word
  `0x0` at `sp -0x20` (`bne` at 0xd6964, `str` at 0xd696c). The read arm loads a byte at
  `ret(0xd76d4) +0x0`, then saves `[ret(0xd76d4)]:1` as a word at the same stack target (`ldrb` at
  0xd6978, `str` at 0xd697c). The r0 return is `[sp-32]` (`ldmdb` at 0xd6984). Physical presence interpretation remains
  unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6938 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6940 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd6944 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6948 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6958 | store | 4 | `sp -0x1c` | `ret(0xd76d4)` |
| `ldr` at 0xd695c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd696c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd6974 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6978 | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `str` at 0xd697c | store | 4 | `sp -0x20` | `[ret(0xd76d4)]:1` |
| `ldr` at 0xd6980 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd6984 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6984 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6984 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6950 | `(unknown callee) 0xd76d4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd6964 | `ret(0xd76d4) != 0x0` | `ret(0xd76d4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6984 | `r0` | `[sp-32]` |

## `ioExpSetBit`

- `ioExpSetBit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6988 to 0xd69d4.
  Established. The call to `0xd6a1c` carries entry r0, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r2 & 0xff)`
  (`bl` at 0xd69c4); its result supplies r0 at the return (`ldmdb` at 0xd69d0). The byte stores
  retain `(r1 & 0xff)` and `(r2 & 0xff)` at `sp -0x15` and `sp -0x16`, respectively (`strb` at
  0xd69a0, `strb` at 0xd69a8). Calls other than that target are absent, and physical pin-setting
  behavior is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6998 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd69a0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd69a8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd69ac | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd69b0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd69b4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd69b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd69d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd69d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd69d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd69c4 | `(unknown callee) 0xd6a1c` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd69d0 | `r0` | `ret(0xd6a1c)` |

## `ioExpClearBit`

- `ioExpClearBit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd69d4 to 0xd6a1c.
  Established. A call to `0xd6a1c` has entry r0, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0` (`bl` at
  0xd6a0c). The returned r0 value is `ret(0xd6a1c)` (`ldmdb` at 0xd6a18). An earlier word store
  saves entry r0 at `sp -0x14` (`str` at 0xd69e4). No conditional branches are reported, and
  physical pin-clearing semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd69e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd69ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd69f4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd69f8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd69fc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd6a00 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd6a18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6a18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6a18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6a0c | `(unknown callee) 0xd6a1c` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6a18 | `r0` | `ret(0xd6a1c)` |

## `ioExpWrite`

- `ioExpWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6a1c to 0xd6b1c.
  Established. The byte test takes the continuation for `[ret(0xd76d4)]:1 != 0x0` and falls through
  on equality to store word `0xfffffc11` at `sp -0x28` (`bne` at 0xd6a88, `str` at 0xd6a90). A later
  call to `0xd77f4` carries `[sp-20]`, `[sp-28]`, `[sp-32]`, `[sp-22]:1` (`bl` at 0xd6af8). Before
  that call, a word store writes `[sp-23]:1` at `sp -0x2c` (`str` at 0xd6ae4). The following call to
  `0xc7b90` has `ret(0xd77f4)`, unknown, unknown, `ret(0xd77f4)` (`bl` at 0xd6b04). Physical write
  behavior, argument count and the merged return remain unknown (`ldmdb` at 0xd6b14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6a2c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6a38 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd6a40 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd6a48 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0xd6a4c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6a50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6a60 | store | 4 | `sp -0x1c` | `ret(0xd75c0)` |
| `ldrb` at 0xd6a64 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6a68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6a78 | store | 4 | `sp -0x20` | `ret(0xd76d4)` |
| `ldr` at 0xd6a7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd6a80 | load | 1 | `ret(0xd76d4) +0x0` | `-` |
| `ldr` at 0xd6a8c | load | 4 | `abs 0xd6b18` | `-` |
| `str` at 0xd6a90 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd6a98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6a9c | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xd6aa0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6ac0 | store | 4 | `sp -0x24` | `ret(0xc7a50)` |
| `ldr` at 0xd6ac4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6ad0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd6ad4 | store | 4 | `sp -0x28` | `ret(0xc7a50)` |
| `ldrb` at 0xd6adc | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd6ae0 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd6ae4 | store | 4 | `sp -0x2c` | `[sp-23]:1` |
| `ldr` at 0xd6ae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6aec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6af0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd6b00 | store | 4 | `sp -0x24` | `ret(0xd77f4)` |
| `ldr` at 0xd6b08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd6b0c | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xd6b10 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd6b14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6b14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6b14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6a58 | `(unknown callee) 0xd75c0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd6a70 | `(unknown callee) 0xd76d4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd6aa8 | `(unknown callee) 0xc7784` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xd6ab8 | `(unknown callee) 0xc7a50` | `?` | `?` | `?` | `?` |
| `bl` at 0xd6af8 | `(unknown callee) 0xd77f4` | `[sp-20]` | `[sp-28]` | `[sp-32]` | `[sp-22]:1` |
| `bl` at 0xd6b04 | `(unknown callee) 0xc7b90` | `ret(0xd77f4)` | `?` | `?` | `ret(0xd77f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd6a88 | `[ret(0xd76d4)]:1 != 0x0` | `[ret(0xd76d4)]:1 == 0x0` |
| `beq` at 0xd6acc | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6b14 | `r0` | `?` |

## `ioExpGetBit`

- `ioExpGetBit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6b1c to 0xd6b9c.
  Established. The call to `0xd6b9c` has entry r0, `(r1 & 0xff)`, `sp-29`, `(r1 & 0xff)` (`bl` at
  0xd6b50); its zero result takes the continuation and nonzero falls through to store word
  `ret(0xd6b9c)` at `sp -0x24` (`beq` at 0xd6b64, `str` at 0xd6b6c). The other arm stores an unknown
  word at that same stack target (`str` at 0xd6b90). The joined r0 value and physical input-bit
  interpretation remain unproven (`ldmdb` at 0xd6b98).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6b2c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6b34 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd6b3c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd6b40 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6b48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6b58 | store | 4 | `sp -0x1c` | `ret(0xd6b9c)` |
| `ldr` at 0xd6b5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6b68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd6b6c | store | 4 | `sp -0x24` | `ret(0xd6b9c)` |
| `ldrb` at 0xd6b74 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd6b78 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd6b90 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd6b94 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd6b98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6b98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6b98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6b50 | `(unknown callee) 0xd6b9c` | `r0` | `(r1 & 0xff)` | `sp-29` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd6b64 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6b98 | `r0` | `?` |
