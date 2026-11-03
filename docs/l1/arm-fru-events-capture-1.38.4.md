# ARM FRU events and capture paths in 1.38.4

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
register locations r0 through r3 with unknown arity. Explicit stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
The computed selector in L1FRUSHUBDeadlock remains unresolved. Tables and return values cover only known edges, with its unresolved site listed below the entry. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/fru-events-capture-facts.txt) retain all tool sections.

## `L1FRU_2m_Dump`

- `L1FRU_2m_Dump`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6cf8 to 0xa6d24.
  Established. The only call reaches `0xc44f8` with entry r0, `0x0`, entry r1, entry r3 (`bl` at
  0xa6d1c). The exit propagates `ret(0xc44f8)` (`ldmdb` at 0xa6d20). No deciding branches are
  reported. Dump-output format and field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6d08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa6d0c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa6d10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6d18 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xa6d20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6d20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6d20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6d1c | `(unknown callee) 0xc44f8` | `r0` | `0x0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6d20 | `r0` | `ret(0xc44f8)` |

## `L1FRU_2m_Events`

- `L1FRU_2m_Events`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6d24 to 0xa77b0.
  Established. Three byte stores carry `([[sp-104]+1]:1 & 0xff)`, `([[sp-104]+2]:1 & 0xff)` and
  `([[sp-104]+3]:1 & 0xff)` at `[sp-104] +0x0`, `+0x1` and `+0x2`, respectively (`strb` at 0xa70ec,
  `strb` at 0xa710c, `strb` at 0xa712c). The initial call to `0x74908` has `0x0` and entry r1
  through r3 (`bl` at 0xa6d38). The outer loop retains only `lt flags condition (operands unknown)`
  and its negation for fall-through (`blt` at 0xa6dd0). Event-record encoding and final r0 remain
  unresolved (`ldmdb` at 0xa77a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6d40 | store | 4 | `sp -0x98` | `ret(0x74908)` |
| `ldr` at 0xa6d44 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa6d48 | load | 4 | `ret(0x74908) +0x8` | `-` |
| `str` at 0xa6d4c | store | 4 | `sp -0x1c` | `[ret(0x74908)+8]` |
| `ldr` at 0xa6d50 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa6d54 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0xa6d58 | store | 4 | `sp -0x74` | `[ret(0x74908)+12]` |
| `ldr` at 0xa6d5c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa6d60 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `str` at 0xa6d64 | store | 4 | `sp -0x78` | `[ret(0x74908)+16]` |
| `ldr` at 0xa6d68 | load | 4 | `abs 0xa77a4` | `-` |
| `strb` at 0xa6d74 | store | 1 | `sp -0x60` | `?` |
| `strb` at 0xa6d7c | store | 1 | `sp -0x5f` | `?` |
| `strb` at 0xa6d84 | store | 1 | `sp -0x5e` | `?` |
| `strb` at 0xa6d8c | store | 1 | `sp -0x5d` | `?` |
| `strb` at 0xa6d94 | store | 1 | `sp -0x5c` | `?` |
| `strb` at 0xa6d9c | store | 1 | `sp -0x5b` | `?` |
| `strb` at 0xa6da4 | store | 1 | `sp -0x5a` | `?` |
| `strb` at 0xa6dac | store | 1 | `sp -0x59` | `?` |
| `ldr` at 0xa6db0 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa6db4 | load | 4 | `abs 0x2b1af4` | `-` |
| `str` at 0xa6db8 | store | 4 | `sp -0x80` | `[0x2b1af4]` |
| `str` at 0xa6dc0 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xa6dc4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa6dc8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa6dd8 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa6ddc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa6df0 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6df8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6dfc | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xa6e00 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa6e04 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa6e18 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6e20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6e24 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xa6e28 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa6e2c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa6e40 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6e48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6e4c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xa6e50 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa6e54 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa6e68 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6e70 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6e74 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xa6e78 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa6e7c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa6e90 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6e98 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6e9c | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xa6ea0 | load | 4 | `abs 0xa77ac` | `-` |
| `str` at 0xa6ea4 | store | 4 | `sp -0x2c` | `0xffff` |
| `str` at 0xa6eac | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xa6eb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6eb4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa6ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6ed8 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa6ee0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa6efc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6f04 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa6f0c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa6f10 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6f24 | store | 4 | `sp -0x2c` | `ret(0xbd340)` |
| `ldr` at 0xa6f2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6f40 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa6f48 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa6f64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6f6c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa6f74 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa6f78 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6f8c | store | 4 | `sp -0x2c` | `ret(0xbd340)` |
| `ldr` at 0xa6f90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa6f98 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0xa6fa4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa6fa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6fac | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa6fbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6fc4 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa6fcc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa6fdc | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa6fe4 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa6fe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6ff0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa6ff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa700c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7014 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7020 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa7030 | store | 4 | `sp -0x50` | `ret(0xbd340)` |
| `ldr` at 0xa7034 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa7040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7048 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa7050 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa706c | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa7074 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa708c | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0xa7090 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7094 | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0xa7098 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa709c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa70a0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa70a4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa70b0 | store | 4 | `sp -0x4c` | `ret(0xbd684)` |
| `ldr` at 0xa70b4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa70b8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa70c4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa70cc | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa70d4 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xa70d8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa70e0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa70e8 | load | 1 | `[sp-104] +0x1` | `-` |
| `strb` at 0xa70ec | store | 1 | `[sp-104] +0x0` | `([[sp-104]+1]:1 & 0xff)` |
| `ldr` at 0xa70f4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7100 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7108 | load | 1 | `[sp-104] +0x2` | `-` |
| `strb` at 0xa710c | store | 1 | `[sp-104] +0x1` | `([[sp-104]+2]:1 & 0xff)` |
| `ldr` at 0xa7114 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7120 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7128 | load | 1 | `[sp-104] +0x3` | `-` |
| `strb` at 0xa712c | store | 1 | `[sp-104] +0x2` | `([[sp-104]+3]:1 & 0xff)` |
| `ldr` at 0xa7134 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7140 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7148 | load | 1 | `[sp-104] +0x4` | `-` |
| `strb` at 0xa714c | store | 1 | `[sp-104] +0x3` | `([[sp-104]+4]:1 & 0xff)` |
| `ldr` at 0xa7154 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7160 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7168 | load | 1 | `[sp-104] +0x5` | `-` |
| `strb` at 0xa716c | store | 1 | `[sp-104] +0x4` | `([[sp-104]+5]:1 & 0xff)` |
| `ldr` at 0xa7174 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7180 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa7188 | load | 1 | `[sp-104] +0x6` | `-` |
| `strb` at 0xa718c | store | 1 | `[sp-104] +0x5` | `([[sp-104]+6]:1 & 0xff)` |
| `ldr` at 0xa7194 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa71a0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xa71a8 | load | 1 | `[sp-104] +0x7` | `-` |
| `strb` at 0xa71ac | store | 1 | `[sp-104] +0x6` | `([[sp-104]+7]:1 & 0xff)` |
| `ldr` at 0xa71b4 | load | 4 | `sp -0x68` | `-` |
| `strb` at 0xa71c0 | store | 1 | `[sp-104] +0x7` | `0x0` |
| `ldr` at 0xa71c4 | load | 4 | `abs 0xa77ac` | `-` |
| `str` at 0xa71c8 | store | 4 | `sp -0x28` | `0xffff` |
| `str` at 0xa71d0 | store | 4 | `sp -0x30` | `0x3` |
| `str` at 0xa71d8 | store | 4 | `sp -0x34` | `0x7` |
| `str` at 0xa71e0 | store | 4 | `sp -0x38` | `0x3` |
| `str` at 0xa71e8 | store | 4 | `sp -0x3c` | `0x7f` |
| `str` at 0xa71f0 | store | 4 | `sp -0x40` | `0x7f` |
| `str` at 0xa71f8 | store | 4 | `sp -0x44` | `0x7f` |
| `str` at 0xa7200 | store | 4 | `sp -0x48` | `0x7f` |
| `ldr` at 0xa7204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa720c | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7214 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa7228 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa722c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7234 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa723c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa7258 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa7260 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa7274 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7280 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7288 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa72a4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa72ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xa72b0 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xa72b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa72bc | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa72c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa72c8 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xa72cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa72d4 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa72dc | load | 1 | `(unknown)` | `-` |
| `str` at 0xa72e8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa72ec | load | 4 | `sp -0x78` | `-` |
| `str` at 0xa72f0 | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0xa72f4 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa72f8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa730c | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa7314 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7318 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0xa731c | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa7320 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa7334 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa733c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7340 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0xa7344 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7348 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0xa734c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7350 | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0xa7354 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa7358 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0xa735c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa7360 | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0xa7364 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa7368 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0xa736c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa7370 | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0xa7374 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa7378 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0xa737c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa7380 | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0xa7384 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa7388 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0xa738c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa7390 | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0xa7394 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa7398 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xa739c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa73a0 | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0xa73a4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa73a8 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0xa73ac | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa73b0 | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0xa73b4 | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa73b8 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0xa73bc | load | 4 | `abs 0xa77a4` | `-` |
| `ldr` at 0xa73c0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa73c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa73c8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xa73d4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa73e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa73e8 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa73f0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7408 | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0xa740c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7410 | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0xa7414 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7418 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa741c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa7420 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa742c | store | 4 | `sp -0x4c` | `ret(0xbd684)` |
| `ldr` at 0xa7430 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7434 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0xa7440 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7448 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7450 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xa7454 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7458 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa745c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7460 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7468 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa746c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7470 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa747c | store | 4 | `sp -0x28` | `ret(0xbf4a4)` |
| `ldr` at 0xa7480 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7484 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7488 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa748c | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7494 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa749c | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa74a8 | store | 4 | `sp -0x30` | `ret(0xbf4a4)` |
| `ldr` at 0xa74ac | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa74b0 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa74b4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa74b8 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa74c0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa74c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa74c8 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa74d4 | store | 4 | `sp -0x34` | `ret(0xbf4a4)` |
| `ldr` at 0xa74d8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa74dc | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa74e0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa74e4 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa74ec | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa74f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa74f4 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7500 | store | 4 | `sp -0x38` | `ret(0xbf4a4)` |
| `ldr` at 0xa7504 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7508 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa750c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7510 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7518 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa751c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7520 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa752c | store | 4 | `sp -0x3c` | `ret(0xbf4a4)` |
| `ldr` at 0xa7530 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7534 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7538 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa753c | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7544 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7548 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa754c | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7558 | store | 4 | `sp -0x40` | `ret(0xbf4a4)` |
| `ldr` at 0xa755c | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa7560 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7564 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7568 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa7570 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7574 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7578 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa7584 | store | 4 | `sp -0x44` | `ret(0xbf4a4)` |
| `ldr` at 0xa7588 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa758c | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0xa7590 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa7594 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0xa759c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa75a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa75a4 | load | 4 | `sp -0x84` | `-` |
| `str` at 0xa75b0 | store | 4 | `sp -0x48` | `ret(0xbf4a4)` |
| `ldr` at 0xa75b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa75bc | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa75c4 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa75d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa75dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa75e4 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa75ec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa7608 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0xa7610 | load | 2 | `(unknown)` | `-` |
| `str` at 0xa7624 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7630 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0xa7638 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xa7654 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa765c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7660 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xa7664 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa766c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xa7674 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7678 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0xa767c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7684 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0xa768c | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7698 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa769c | load | 4 | `sp -0x78` | `-` |
| `str` at 0xa76a0 | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0xa76a4 | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa76a8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa76bc | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa76c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa76c8 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0xa76cc | load | 4 | `abs 0xa77a8` | `-` |
| `ldr` at 0xa76d0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa76e4 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa76ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xa76f0 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0xa76f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa76f8 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0xa76fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7700 | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0xa7704 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa7708 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0xa770c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa7710 | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0xa7714 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa7718 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0xa771c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa7720 | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0xa7724 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa7728 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0xa772c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa7730 | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0xa7734 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa7738 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0xa773c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa7740 | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0xa7744 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa7748 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xa774c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa7750 | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0xa7754 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa7758 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0xa775c | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa7760 | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0xa7764 | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa7768 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0xa776c | load | 4 | `abs 0xa77a4` | `-` |
| `ldr` at 0xa7770 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa7774 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7778 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xa7780 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7788 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xa7790 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0xa7798 | store | 4 | `sp -0x7c` | `[sp-124]+1` |
| `ldmdb` at 0xa77a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa77a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa77a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6d38 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa6d6c | `(unknown callee) 0xc0710` | `0x2b1afc` | `?` | `?` | `[ret(0x74908)+16]` |
| `bl` at 0xa6f1c | `(unknown callee) 0xbd340` | `?` | `0x2a` | `0x20` | `?` |
| `bl` at 0xa6f84 | `(unknown callee) 0xbd340` | `?` | `0x15` | `0x8` | `?` |
| `bl` at 0xa7028 | `(unknown callee) 0xbd340` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0xa70a8 | `(unknown callee) 0xbd684` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0xa73cc | `(unknown callee) 0xc08d8` | `0x2b1afc` | `[sp-104]` | `[sp-28]` | `[sp-116]` |
| `bl` at 0xa7424 | `(unknown callee) 0xbd684` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0xa7474 | `(unknown callee) 0xbf4a4` | `0x0` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa74a0 | `(unknown callee) 0xbf4a4` | `0x1` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa74cc | `(unknown callee) 0xbf4a4` | `0x2` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa74f8 | `(unknown callee) 0xbf4a4` | `0x7` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7524 | `(unknown callee) 0xbf4a4` | `0x3` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa7550 | `(unknown callee) 0xbf4a4` | `0x4` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa757c | `(unknown callee) 0xbf4a4` | `0x5` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa75a8 | `(unknown callee) 0xbf4a4` | `0x6` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0xa777c | `(unknown callee) 0xc08d8` | `0x2b1afc` | `[sp-104]` | `[sp-28]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xa6dd0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xa6ebc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa6ef8 | `? != 0x2` | `? == 0x2` |
| `bne` at 0xa6f60 | `? != 0x34` | `? == 0x34` |
| `blt` at 0xa6fb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa703c | `ret(0xbd340) == 0x0` | `ret(0xbd340) != 0x0` |
| `ble` at 0xa7060 | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xa7084 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bhi` at 0xa70c0 | `ret(0xbd684) > [sp-132] (unsigned, 32-bit)` | `ret(0xbd684) <= [sp-132] (unsigned, 32-bit)` |
| `beq` at 0xa73dc | `ret(0xbd340) == 0x0` | `ret(0xbd340) != 0x0` |
| `beq` at 0xa7400 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xa743c | `ret(0xbd684) > [sp-132] (unsigned, 32-bit)` | `ret(0xbd684) <= [sp-132] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa77a0 | `r0` | `?` |

## `L1FRU_2m_Examine`

- `L1FRU_2m_Examine`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7900 to 0xa7fa8.
  Established. A word store writes `[0x149980]` at `[sp-24] +0x0`, followed by a word store clearing
  absolute `0x149980` (`str` at 0xa7f4c, `str` at 0xa7f58). The first global test takes its branch
  for `[0x2b1ae8] == 0x0` and falls through for nonzero (`beq` at 0xa7960). The next comparison
  takes its bypass for `[0x2b1af0] != 0x1` and falls through on equality (`bne` at 0xa7974). The
  exit reports `ret(0xc0710)` (`ldmdb` at 0xa7f64). Record ranking and temporal units are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7910 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7914 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa791c | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0xa7924 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xa792c | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xa7934 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xa793c | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xa794c | store | 4 | `sp -0xac` | `ret(0x74908)` |
| `ldr` at 0xa7954 | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa7958 | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa7968 | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa796c | load | 4 | `abs 0x2b1af0` | `-` |
| `ldr` at 0xa7978 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa7980 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xa7988 | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa798c | load | 4 | `abs 0x2b1af0` | `-` |
| `ldr` at 0xa7998 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa79a0 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0xa79a4 | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa79a8 | load | 4 | `abs 0x2b1aec` | `-` |
| `str` at 0xa79ac | store | 4 | `sp -0x48` | `[0x2b1aec]` |
| `ldr` at 0xa79b0 | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa79b4 | load | 4 | `abs 0x2b1aec` | `-` |
| `str` at 0xa79b8 | store | 4 | `sp -0x4c` | `[0x2b1aec]` |
| `ldr` at 0xa79bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa79c4 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0xa79cc | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa79d0 | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa79e0 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xa79e4 | load | 4 | `sp -0xac` | `-` |
| `str` at 0xa79e8 | store | 4 | `sp -0xcc` | `ret(0xc3f0c)` |
| `ldr` at 0xa79f0 | load | 4 | `abs 0xa7f6c` | `-` |
| `ldr` at 0xa79f4 | load | 4 | `[sp-172] +0x8` | `-` |
| `ldr` at 0xa79f8 | load | 4 | `[sp-172] +0xc` | `-` |
| `str` at 0xa7a18 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa7a1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7a2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7a30 | load | 4 | `[sp-20] +0x414` | `-` |
| `ldr` at 0xa7a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7a44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7a48 | load | 4 | `abs 0xa7f70` | `-` |
| `ldr` at 0xa7a58 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7a5c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa7a60 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa7a64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7a74 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa7a78 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7a8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa7a90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7aa4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7aa8 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xa7ab0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa7abc | store | 4 | `sp -0xb4` | `ret(0xb7944)` |
| `str` at 0xa7ad0 | store | 4 | `sp -0xcc` | `sp-48` |
| `str` at 0xa7ad8 | store | 4 | `sp -0xc8` | `sp-52` |
| `str` at 0xa7ae0 | store | 4 | `sp -0xc4` | `sp-56` |
| `str` at 0xa7ae8 | store | 4 | `sp -0xc0` | `sp-60` |
| `str` at 0xa7af0 | store | 4 | `sp -0xbc` | `sp-64` |
| `str` at 0xa7af8 | store | 4 | `sp -0xb8` | `sp-168` |
| `ldr` at 0xa7afc | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0xa7b10 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7b1c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7b2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7b30 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0xa7b40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7b44 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xa7b50 | store | 4 | `sp -0xcc` | `ret(0xc3f0c)` |
| `ldr` at 0xa7b54 | load | 4 | `abs 0xa7f6c` | `-` |
| `ldr` at 0xa7b70 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7b80 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa7b88 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0xa7b8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7b90 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7b9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa7ba0 | store | 4 | `sp -0x48` | `[sp-40]` |
| `ldr` at 0xa7ba4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7ba8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7bb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa7bb8 | store | 4 | `sp -0x4c` | `[sp-40]` |
| `ldr` at 0xa7bbc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa7bc8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa7bd0 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xa7bd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa7be4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa7bec | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0xa7bf0 | load | 4 | `abs 0xa7f74` | `-` |
| `ldr` at 0xa7bf4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xa7bfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7c04 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa7c0c | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa7c10 | load | 4 | `abs 0x2b1afc` | `-` |
| `ldr` at 0xa7c18 | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa7c20 | load | 4 | `abs 0xa7f78` | `-` |
| `ldr` at 0xa7c28 | load | 4 | `abs 0x2b1afc` | `-` |
| `ldr` at 0xa7c30 | load | 4 | `abs 0xa7f68` | `-` |
| `ldr` at 0xa7c38 | load | 4 | `abs 0xa7f78` | `-` |
| `ldr` at 0xa7c40 | load | 4 | `abs 0x2b1afc` | `-` |
| `ldr` at 0xa7c48 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa7c5c | store | 4 | `sp -0xb0` | `ret(0x1b1e4)` |
| `ldr` at 0xa7c60 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa7c6c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa7c7c | load | 4 | `abs 0xa7f7c` | `-` |
| `ldr` at 0xa7c8c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa7c9c | load | 4 | `abs 0xa7f80` | `-` |
| `ldr` at 0xa7cb0 | load | 4 | `abs 0xa7f84` | `-` |
| `ldr` at 0xa7cd8 | load | 4 | `abs 0xa7f88` | `-` |
| `ldr` at 0xa7d14 | load | 4 | `abs 0xa7f8c` | `-` |
| `ldr` at 0xa7d38 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7d3c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7d44 | load | 4 | `abs 0xa7f90` | `-` |
| `ldr` at 0xa7d5c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7d60 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7d68 | load | 4 | `abs 0xa7f94` | `-` |
| `ldr` at 0xa7d80 | load | 4 | `abs 0xa7f94` | `-` |
| `ldr` at 0xa7dac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7db0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7db8 | load | 4 | `abs 0xa7f94` | `-` |
| `str` at 0xa7de4 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0xa7de8 | load | 4 | `abs 0xa7f98` | `-` |
| `ldr` at 0xa7e0c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7e10 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa7e1c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7e20 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa7e34 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0xa7e38 | store | 4 | `sp -0xcc` | `[sp-176]` |
| `ldr` at 0xa7e40 | load | 4 | `abs 0xa7f9c` | `-` |
| `ldr` at 0xa7e44 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7e48 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7e6c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa7e70 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7e78 | load | 4 | `abs 0xa7f90` | `-` |
| `ldr` at 0xa7e90 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa7e94 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7e9c | load | 4 | `abs 0xa7f94` | `-` |
| `ldr` at 0xa7eb4 | load | 4 | `abs 0xa7f94` | `-` |
| `ldr` at 0xa7ee0 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa7ee4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa7eec | load | 4 | `abs 0xa7f94` | `-` |
| `str` at 0xa7f18 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0xa7f1c | load | 4 | `abs 0xa7fa0` | `-` |
| `ldr` at 0xa7f40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7f44 | load | 4 | `abs 0xa7fa4` | `-` |
| `ldr` at 0xa7f48 | load | 4 | `abs 0x149980` | `-` |
| `str` at 0xa7f4c | store | 4 | `[sp-24] +0x0` | `[0x149980]` |
| `ldr` at 0xa7f50 | load | 4 | `abs 0xa7fa4` | `-` |
| `str` at 0xa7f58 | store | 4 | `abs 0x149980` | `0x0` |
| `ldr` at 0xa7f5c | load | 4 | `abs 0xa7f74` | `-` |
| `ldmdb` at 0xa7f64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa7f64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa7f64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7944 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0xa7950 | `(unknown callee) 0xa77b0` | `ret(0x74908)` | `?` | `?` | `ret(0x74908)` |
| `bl` at 0xa79d4 | `(unknown callee) 0xc3f0c` | `[0x2b1ae8]` | `?` | `?` | `0x2b1ae0` |
| `bl` at 0xa79fc | `(unknown callee) 0x1b49c` | `sp-164` | `0x126650` | `[[sp-172]+8]` | `[[sp-172]+12]` |
| `bl` at 0xa7a10 | `(unknown callee) 0xa77f8` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa7ab8 | `(unknown callee) 0xb7944` | `?` | `0x1` | `0x418` | `?` |
| `bl` at 0xa7b0c | `(unknown callee) 0xc3994` | `[sp-180]` | `sp-36` | `sp-40` | `sp-44` |
| `bl` at 0xa7b20 | `(unknown callee) 0xc3f0c` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0xa7b58 | `(unknown callee) 0x1b49c` | `sp-164` | `0x126650` | `?` | `?` |
| `bl` at 0xa7b6c | `(unknown callee) 0xa77f8` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa7bf8 | `(unknown callee) 0xc087c` | `0x2b1afc` | `[sp-168]` | `[sp-40]` | `?` |
| `bl` at 0xa7c14 | `(unknown callee) 0xc0a7c` | `[0x2b1afc]` | `?` | `?` | `0x2b1ae0` |
| `bl` at 0xa7c2c | `(unknown callee) 0xc0d74` | `0x0` | `0xa77f8` | `0x0` | `[0x2b1afc]` |
| `bl` at 0xa7c44 | `(unknown callee) 0xc0d74` | `0x2` | `0xa77f8` | `0x0` | `[0x2b1afc]` |
| `bl` at 0xa7c58 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[sp-68]` |
| `bl` at 0xa7c84 | `(unknown callee) 0xa77f8` | `0x0` | `0x126668` | `?` | `[sp-84]` |
| `bl` at 0xa7ca4 | `(unknown callee) 0xa77f8` | `0x0` | `0x1266a0` | `?` | `[sp-80]` |
| `bl` at 0xa7cb8 | `(unknown callee) 0xa77f8` | `0x0` | `0x1266c8` | `?` | `?` |
| `bl` at 0xa7cc4 | `(unknown callee) 0x1afe0` | `sp-72` | `?` | `?` | `sp-72` |
| `bl` at 0xa7ce0 | `(unknown callee) 0x1b13c` | `sp-164` | `0x3c` | `0x1266f0` | `ret(0x1afe0)` |
| `bl` at 0xa7cf4 | `(unknown callee) 0xa77f8` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa7d00 | `(unknown callee) 0x1afe0` | `sp-76` | `?` | `?` | `sp-76` |
| `bl` at 0xa7d1c | `(unknown callee) 0x1b13c` | `sp-164` | `0x3c` | `0x126710` | `ret(0x1afe0)` |
| `bl` at 0xa7d30 | `(unknown callee) 0xa77f8` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa7df4 | `(unknown callee) 0x1b49c` | `sp-164` | `0x126730` | `?` | `?` |
| `bl` at 0xa7e08 | `(unknown callee) 0xa77f8` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa7e4c | `(unknown callee) 0x1b49c` | `sp-164` | `0x12674c` | `[sp-72]` | `[sp-76]` |
| `bl` at 0xa7e60 | `(unknown callee) 0xa77f8` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa7f28 | `(unknown callee) 0x1b49c` | `sp-164` | `0x126770` | `?` | `?` |
| `bl` at 0xa7f3c | `(unknown callee) 0xa77f8` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa7f60 | `(unknown callee) 0xc0710` | `0x2b1afc` | `?` | `0x149980` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa7960 | `[0x2b1ae8] == 0x0` | `[0x2b1ae8] != 0x0` |
| `bne` at 0xa7974 | `[0x2b1af0] != 0x1` | `[0x2b1af0] == 0x1` |
| `bne` at 0xa7994 | `[0x2b1af0] != 0x2` | `[0x2b1af0] == 0x2` |
| `beq` at 0xa7a24 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `blt` at 0xa7a38 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa7a6c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa7a84 | `? == 0x1` | `? != 0x1` |
| `bne` at 0xa7a9c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa7b18 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `beq` at 0xa7b78 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bge` at 0xa7b98 | `[sp-40] >= [sp-72] (signed, 32-bit)` | `[sp-40] < [sp-72] (signed, 32-bit)` |
| `ble` at 0xa7bb0 | `[sp-40] <= [sp-76] (signed, 32-bit)` | `[sp-40] > [sp-76] (signed, 32-bit)` |
| `bne` at 0xa7bc4 | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `bne` at 0xa7be0 | `[sp-44] != 0x2` | `[sp-44] == 0x2` |
| `beq` at 0xa7c50 | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `beq` at 0xa7c68 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `beq` at 0xa7c74 | `[sp-84] == 0x0` | `[sp-84] != 0x0` |
| `beq` at 0xa7c94 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `bgt` at 0xa7e18 | `[sp-76] > [sp-176] (signed, 32-bit)` | `[sp-76] <= [sp-176] (signed, 32-bit)` |
| `bgt` at 0xa7e28 | `[sp-72] > [sp-176] (signed, 32-bit)` | `[sp-72] <= [sp-176] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa7f64 | `r0` | `ret(0xc0710)` |

## `L1FRU_2m_Attach`

- `L1FRU_2m_Attach`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7fa8 to 0xa8148.
  Established. The first call to `0x1f1dc` carries `0x1`, entry r1, entry r2, entry r3 (`bl` at
  0xa7fc8). The selector takes its branch for `[sp-20] != 0x1` and falls through on equality (`bne`
  at 0xa7ff4). Another selector takes its bypass for `[sp-20] != 0x2` and falls through on equality
  (`bne` at 0xa80b4). The exit carries `[sp-52]` (`ldmdb` at 0xa8138). Attachment-record format and
  ownership remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7fb8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7fbc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa7fc0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa7fcc | store | 4 | `sp -0x28` | `ret(0x1f1dc)` |
| `str` at 0xa7fd8 | store | 4 | `sp -0x2c` | `ret(0x1f1dc)` |
| `str` at 0xa7fe8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xa7fec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7ff8 | load | 4 | `abs 0xa813c` | `-` |
| `ldr` at 0xa7ffc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8004 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa8008 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa800c | load | 4 | `abs 0x2b1aec` | `-` |
| `str` at 0xa8010 | store | 4 | `sp -0x58` | `[0x2b1aec]` |
| `ldr` at 0xa8014 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa8018 | load | 4 | `abs 0x2b1af0` | `-` |
| `str` at 0xa801c | store | 4 | `sp -0x54` | `[0x2b1af0]` |
| `ldr` at 0xa8020 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa8024 | load | 4 | `abs 0x2b1af4` | `-` |
| `str` at 0xa8028 | store | 4 | `sp -0x50` | `[0x2b1af4]` |
| `ldr` at 0xa802c | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa8030 | load | 4 | `abs 0x2b1af8` | `-` |
| `str` at 0xa8034 | store | 4 | `sp -0x4c` | `[0x2b1af8]` |
| `ldr` at 0xa8038 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa803c | load | 4 | `abs 0x2b1b00` | `-` |
| `str` at 0xa8040 | store | 4 | `sp -0x48` | `[0x2b1b00]` |
| `ldr` at 0xa8044 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa8048 | load | 4 | `abs 0x2b1b04` | `-` |
| `str` at 0xa804c | store | 4 | `sp -0x44` | `[0x2b1b04]` |
| `ldr` at 0xa8050 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa8054 | load | 4 | `abs 0x2b1b08` | `-` |
| `str` at 0xa8058 | store | 4 | `sp -0x40` | `[0x2b1b08]` |
| `str` at 0xa8060 | store | 4 | `sp -0x3c` | `sp-32` |
| `str` at 0xa8068 | store | 4 | `sp -0x38` | `sp-36` |
| `ldr` at 0xa806c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa8070 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8074 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa8078 | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa8084 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa8088 | store | 4 | `sp -0x58` | `[sp-36]` |
| `ldr` at 0xa8094 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8098 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa80a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa80a4 | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0xa80ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa80b8 | load | 4 | `abs 0xa8144` | `-` |
| `ldr` at 0xa80bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa80c4 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa80c8 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa80cc | load | 4 | `abs 0x2b1aec` | `-` |
| `str` at 0xa80d0 | store | 4 | `sp -0x58` | `[0x2b1aec]` |
| `ldr` at 0xa80d4 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa80d8 | load | 4 | `abs 0x2b1af0` | `-` |
| `str` at 0xa80dc | store | 4 | `sp -0x54` | `[0x2b1af0]` |
| `ldr` at 0xa80e0 | load | 4 | `abs 0xa8140` | `-` |
| `ldr` at 0xa80e4 | load | 4 | `abs 0x2b1afc` | `-` |
| `str` at 0xa80e8 | store | 4 | `sp -0x50` | `[0x2b1afc]` |
| `str` at 0xa80f0 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xa80f8 | store | 4 | `sp -0x48` | `sp-36` |
| `ldr` at 0xa80fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa8100 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8104 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa8108 | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa8118 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa811c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8120 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8128 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa812c | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0xa8134 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xa8138 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8138 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8138 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7fc8 | `(unknown callee) 0x1f1dc` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xa7fd4 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xa7fdc | `(unknown callee) 0x4f090` | `ret(0x1f1dc)` | `?` | `?` | `?` |
| `bl` at 0xa8000 | `(unknown callee) 0x2070c` | `0x12678c` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0xa807c | `(unknown callee) 0xc1cec` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x2b1ae8]` |
| `bl` at 0xa809c | `(unknown callee) 0x69418` | `0x3` | `sp-24` | `[sp-28]` | `[sp-32]` |
| `bl` at 0xa80c0 | `(unknown callee) 0x2070c` | `0x126798` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0xa810c | `(unknown callee) 0xc2a14` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x2b1ae8]` |
| `bl` at 0xa8124 | `(unknown callee) 0x6917c` | `sp-24` | `[sp-28]` | `[sp-32]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa7ff4 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xa80b4 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8138 | `r0` | `[sp-52]` |

## `L1FRU_2m_PortStatus`

- `L1FRU_2m_PortStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8148 to 0xa8168.
  Established. The sole call to `0x42a24` uses `0x0`, `0x2b1b00`, entry r2, entry r3 (`bl` at
  0xa815c); its result reaches r0 at exit (`ldmdb` at 0xa8160). No conditional branches occur in the
  facts. Port-status reply fields remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa8158 | load | 4 | `abs 0xa8164` | `-` |
| `ldmdb` at 0xa8160 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8160 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8160 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa815c | `(unknown callee) 0x42a24` | `0x0` | `0x2b1b00` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8160 | `r0` | `ret(0x42a24)` |

## `L1FRU_2m_MgnStatus`

- `L1FRU_2m_MgnStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8188 to 0xa81a8.
  Established. The call to `0x945c0` has `0x2b1b08`, `0x1`, entry r2, entry r3 (`bl` at 0xa819c).
  The return reports `ret(0x945c0)` (`ldmdb` at 0xa81a0). No deciding branches are reported.
  Margin-status format remains unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa8194 | load | 4 | `abs 0xa81a4` | `-` |
| `ldmdb` at 0xa81a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa81a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa81a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa819c | `(unknown callee) 0x945c0` | `0x2b1b08` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa81a0 | `r0` | `ret(0x945c0)` |

## `L1FRUSHUBCapture`

- `L1FRUSHUBCapture`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa81a8 to 0xa8488.
  Established. The word store at absolute `0x2b1ae8` writes `0x1` (`str` at 0xa81d4). Two later byte
  stores have unknown targets and values (`strb` at 0xa8358, `strb` at 0xa8378). The entry test
  takes its continuation for `[r0+24] != 0x0` and falls through on zero (`bne` at 0xa81c8). The
  first call to `0xe335c` carries entry r0, r1, r2, `[r0+24]` (`bl` at 0xa81dc). Capture-register
  meaning and final r0 remain unknown (`ldmdb` at 0xa847c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa81b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa81bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa81c0 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0xa81cc | load | 4 | `abs 0xa8480` | `-` |
| `str` at 0xa81d4 | store | 4 | `abs 0x2b1ae8` | `0x1` |
| `ldr` at 0xa81e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa81e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa81e8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xa81f0 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8204 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa8208 | load | 4 | `abs 0xa8480` | `-` |
| `ldr` at 0xa820c | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa8218 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa821c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8220 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xa8234 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xa8238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa823c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8250 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xa8258 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa825c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa8260 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8264 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8278 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0xa8280 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa8288 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa828c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa8298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa829c | load | 4 | `[sp-20] +0x4` | `-` |
| `str` at 0xa82ac | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xa82b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa82b4 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0xa82bc | store | 4 | `sp -0x3c` | `sp-48` |
| `str` at 0xa82c4 | store | 4 | `sp -0x38` | `sp-52` |
| `str` at 0xa82ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa82f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa82f8 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa8310 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa832c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa8330 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa833c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8340 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8348 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8350 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa8358 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa835c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8360 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8368 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8370 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa8378 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa837c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8380 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8388 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8390 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa8398 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa839c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa83a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa83a8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa83b0 | load | 1 | `sp -0x30` | `-` |
| `strb` at 0xa83b4 | store | 1 | `(unknown)` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0xa83b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa83bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa83c4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa83cc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa83d4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa83d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa83dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa83e4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa83ec | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa83f4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa83f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa83fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8404 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa840c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa8414 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa8418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa841c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8424 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa842c | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa8430 | store | 1 | `(unknown)` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xa8438 | load | 4 | `abs 0xa8480` | `-` |
| `str` at 0xa8440 | store | 4 | `abs 0x2b1ae8` | `0x4` |
| `ldr` at 0xa8444 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa8448 | store | 4 | `sp -0x44` | `[sp-44]` |
| `ldr` at 0xa844c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa8450 | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0xa8454 | load | 4 | `abs 0xa8484` | `-` |
| `ldr` at 0xa8458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa845c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8460 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8468 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa8470 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xa847c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa847c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa847c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa81dc | `(unknown callee) 0xe335c` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0xa81fc | `(unknown callee) 0x1b43c` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0xa82a0 | `(unknown callee) 0x48298` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0xa82d8 | `(unknown callee) 0x63a3c` | `ret(0x48298)` | `?` | `?` | `?` |
| `bl` at 0xa82fc | `(unknown callee) 0x48298` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0xa8318 | `(unknown callee) 0xea468` | `ret(0x48298)` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0xa8464 | `(unknown callee) 0x2070c` | `0x1267a8` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0xa8478 | `(unknown callee) 0xe33c8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa81c8 | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0xa8214 | `[0x2b1ae8] != 0x0` | `[0x2b1ae8] == 0x0` |
| `blt` at 0xa8228 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa8294 | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `beq` at 0xa8338 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa847c | `r0` | `?` |

## `L1FRUSHUBDeadlock`

- `L1FRUSHUBDeadlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8734 to 0xa9094.
  Established. A word store writes `0x1e` at absolute `0x2b1b20` (`str` at 0xa8790). Other word
  stores write unknown at `0x2b1b40` and `ret(0x1b124)` at `0x1499b8` (`str` at 0xa87b0, `str` at
  0xa87c8). The initial global test takes its bypass for `[0x1499b4] != 0x0` and falls through for
  zero (`bne` at 0xa8784). The computed selector remains unresolved (`ldrls` at 0xa8a60). The
  known-edge exit reports r0 `0x0` (`ldmdb` at 0xa9064). Deadlock criteria and iteration units
  remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8744 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa8748 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa874c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xa8754 | store | 4 | `sp -0x48` | `0x1e` |
| `str` at 0xa875c | store | 4 | `sp -0x4c` | `0x5` |
| `str` at 0xa8764 | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0xa876c | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xa8774 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0xa8778 | load | 4 | `abs 0xa9068` | `-` |
| `ldr` at 0xa877c | load | 4 | `abs 0x1499b4` | `-` |
| `ldr` at 0xa8788 | load | 4 | `abs 0xa906c` | `-` |
| `str` at 0xa8790 | store | 4 | `abs 0x2b1b20` | `0x1e` |
| `ldr` at 0xa8794 | load | 4 | `abs 0xa9070` | `-` |
| `ldr` at 0xa8798 | load | 4 | `abs 0xa906c` | `-` |
| `ldr` at 0xa879c | load | 4 | `abs 0x2b1b20` | `-` |
| `str` at 0xa87b0 | store | 4 | `abs 0x2b1b40` | `?` |
| `ldr` at 0xa87b4 | load | 4 | `abs 0xa9074` | `-` |
| `ldr` at 0xa87b8 | load | 4 | `abs 0xa9070` | `-` |
| `ldr` at 0xa87bc | load | 4 | `abs 0x2b1b40` | `-` |
| `str` at 0xa87c8 | store | 4 | `abs 0x1499b8` | `ret(0x1b124)` |
| `ldr` at 0xa87cc | load | 4 | `abs 0xa9068` | `-` |
| `str` at 0xa87d4 | store | 4 | `abs 0x1499b4` | `0x1` |
| `ldr` at 0xa87ec | load | 4 | `abs 0xa9074` | `-` |
| `ldr` at 0xa87f0 | load | 4 | `abs 0xa9070` | `-` |
| `ldr` at 0xa87f4 | load | 4 | `abs 0x1499b8` | `-` |
| `ldr` at 0xa87fc | load | 4 | `abs 0x2b1b40` | `-` |
| `str` at 0xa8808 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xa8810 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xa8814 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8818 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa8828 | load | 4 | `abs 0xa9078` | `-` |
| `ldr` at 0xa882c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8840 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8844 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa884c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa8850 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa8858 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa8860 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8864 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa8870 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8874 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa8894 | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xa8898 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa889c | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0xa88a0 | load | 4 | `sp -0x60` | `-` |
| `strb` at 0xa88a8 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xa88b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa88b4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa88c0 | store | 4 | `sp -0x70` | `ret(0x1b22c)` |
| `ldr` at 0xa88c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa88c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa88cc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa88e0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa88e8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa88f8 | store | 4 | `[sp-28] +0x0` | `ret(0x1b2c8)` |
| `ldr` at 0xa88fc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa8900 | store | 4 | `sp -0x60` | `[[sp-28]]` |
| `ldr` at 0xa8904 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa8908 | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa8910 | store | 4 | `sp -0x60` | `([[sp-28]] + [sp-112])` |
| `str` at 0xa891c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xa8924 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xa8928 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa892c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa893c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa894c | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xa8954 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa895c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xa896c | store | 4 | `sp -0x70` | `0x2` |
| `str` at 0xa8978 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0xa8980 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xa8984 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8990 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8994 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa89a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa89b0 | load | 4 | `abs 0xa9078` | `-` |
| `ldr` at 0xa89b4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa89c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa89d4 | store | 4 | `sp -0x24` | `ret(0xea468)` |
| `ldr` at 0xa89d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa89e8 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0xa89f0 | store | 1 | `sp -0x80` | `?` |
| `ldr` at 0xa89f4 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0xa89fc | store | 1 | `sp -0x7f` | `?` |
| `ldr` at 0xa8a00 | load | 4 | `sp -0x74` | `-` |
| `strb` at 0xa8a08 | store | 1 | `sp -0x7e` | `?` |
| `ldrb` at 0xa8a0c | load | 1 | `sp -0x74` | `-` |
| `strb` at 0xa8a10 | store | 1 | `sp -0x7d` | `([sp-116]:1 & 0xff)` |
| `ldr` at 0xa8a14 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0xa8a1c | store | 1 | `sp -0x7c` | `?` |
| `ldr` at 0xa8a20 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0xa8a28 | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0xa8a2c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0xa8a34 | store | 1 | `sp -0x7a` | `?` |
| `ldrb` at 0xa8a38 | load | 1 | `sp -0x78` | `-` |
| `strb` at 0xa8a3c | store | 1 | `sp -0x79` | `([sp-120]:1 & 0xff)` |
| `ldr` at 0xa8a40 | load | 4 | `abs 0xa9078` | `-` |
| `ldr` at 0xa8a44 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8a58 | load | 4 | `(unknown)` | `-` |
| `ldrls` at 0xa8a60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8cb0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa8cb8 | store | 4 | `sp -0x58` | `[sp-88]+1` |
| `ldr` at 0xa8cc0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa8cc8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8ce4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa8cec | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa8cf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8d04 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa8d08 | load | 4 | `abs 0xa907c` | `-` |
| `ldr` at 0xa8d0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8d14 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa8d24 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa8d2c | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b22c))` |
| `str` at 0xa8d34 | store | 4 | `sp -0xc4` | `0x0` |
| `str` at 0xa8d40 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xa8d44 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8d48 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa8d58 | load | 4 | `abs 0xa9078` | `-` |
| `ldr` at 0xa8d5c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8d70 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0xa8d74 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa8d7c | store | 4 | `sp -0x6c` | `?` |
| `str` at 0xa8d84 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xa8d88 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8d98 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8da8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8dc0 | load | 4 | `abs 0xa9074` | `-` |
| `ldr` at 0xa8dc4 | load | 4 | `abs 0x1499b8` | `-` |
| `ldr` at 0xa8dcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8ddc | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa8de4 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa8de8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8df8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8e10 | load | 4 | `abs 0xa9074` | `-` |
| `ldr` at 0xa8e14 | load | 4 | `abs 0x1499b8` | `-` |
| `ldr` at 0xa8e1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8e2c | load | 4 | `sp -0x68` | `-` |
| `str` at 0xa8e34 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xa8e38 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa8e40 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa8e48 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa8e50 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa8e58 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa8e64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8e70 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa8e74 | store | 4 | `sp -0xcc` | `[sp-108]` |
| `ldr` at 0xa8e78 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa8e7c | load | 4 | `abs 0xa9080` | `-` |
| `ldr` at 0xa8e80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8e84 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa8e8c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa8e9c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa8ea4 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b22c))` |
| `ldr` at 0xa8eac | load | 4 | `sp -0x68` | `-` |
| `str` at 0xa8eb0 | store | 4 | `sp -0xcc` | `[sp-104]` |
| `ldr` at 0xa8eb4 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa8eb8 | store | 4 | `sp -0xc8` | `[sp-108]` |
| `ldr` at 0xa8ebc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa8ec0 | load | 4 | `abs 0xa9084` | `-` |
| `ldr` at 0xa8ec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8ec8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa8ed0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa8ee0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa8ee8 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b22c))` |
| `str` at 0xa8ef0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xa8ef4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8f00 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8f04 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa8f18 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xa8f1c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8f2c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8f3c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8f54 | load | 4 | `abs 0xa9074` | `-` |
| `ldr` at 0xa8f58 | load | 4 | `abs 0x1499b8` | `-` |
| `ldr` at 0xa8f60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8f74 | load | 4 | `abs 0xa9078` | `-` |
| `ldr` at 0xa8f78 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8f8c | load | 4 | `abs 0xa9088` | `-` |
| `ldr` at 0xa8f90 | load | 4 | `abs 0xa9078` | `-` |
| `ldr` at 0xa8f94 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8fa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8fb8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8fc0 | load | 4 | `abs 0xa908c` | `-` |
| `ldr` at 0xa8fc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8fc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8fd4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa8fe4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xa8ffc | load | 4 | `abs 0xa9074` | `-` |
| `ldr` at 0xa9000 | load | 4 | `abs 0x1499b8` | `-` |
| `ldr` at 0xa9008 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa900c | load | 4 | `abs 0xa9090` | `-` |
| `ldr` at 0xa9014 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa901c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa902c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa9034 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x1b22c))` |
| `ldr` at 0xa9038 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa9040 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa9048 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa9050 | store | 4 | `sp -0x58` | `?` |
| `str` at 0xa905c | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0xa9060 | load | 4 | `sp -0xc4` | `-` |
| `ldmdb` at 0xa9064 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa9064 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9064 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9064 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa87c0 | `(unknown callee) 0x1b124` | `[0x2b1b40]` | `0x2b1b40` | `[0x2b1b20]` | `0x2b1b40` |
| `bl` at 0xa87e8 | `(unknown callee) 0x1b43c` | `sp-68` | `0x0` | `0x20` | `sp-68` |
| `bl` at 0xa8800 | `(unknown callee) 0x1b43c` | `[0x1499b8]` | `0x0` | `[0x2b1b40]` | `0x1499b8` |
| `bl` at 0xa888c | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xa88b8 | `(unknown callee) 0x1b22c` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0xa88f0 | `(unknown callee) 0x1b2c8` | `[[sp-28]]` | `?` | `?` | `?` |
| `bl` at 0xa8914 | `(unknown callee) 0xe335c` | `?` | `?` | `[[sp-28]]` | `?` |
| `bl` at 0xa89a8 | `(unknown callee) 0x48298` | `[sp-24]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xa89cc | `(unknown callee) 0xea468` | `ret(0x48298)` | `?` | `sp-116` | `sp-120` |
| `bl` at 0xa8cd4 | `(unknown callee) 0xe33c8` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0xa8cdc | `(unknown callee) 0xa9294` | `0x64` | `?` | `?` | `?` |
| `bl` at 0xa8ce0 | `(unknown callee) 0xe335c` | `ret(0xa9294)` | `?` | `?` | `?` |
| `bl` at 0xa8cf4 | `(unknown callee) 0xe33c8` | `?` | `?` | `?` | `[sp-76]` |
| `bl` at 0xa8d10 | `(unknown callee) 0x1b49c` | `[sp-96]` | `0x126e28` | `[sp-24]` | `[sp-36]` |
| `bl` at 0xa8d18 | `(unknown callee) 0x1b22c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xa8e88 | `(unknown callee) 0x1b49c` | `[sp-96]` | `0x126e4c` | `[sp-24]` | `[sp-104]` |
| `bl` at 0xa8e90 | `(unknown callee) 0x1b22c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xa8ecc | `(unknown callee) 0x1b49c` | `[sp-96]` | `0x126e74` | `[sp-24]` | `[sp-100]` |
| `bl` at 0xa8ed4 | `(unknown callee) 0x1b22c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xa8fcc | `(unknown callee) 0x1b49c` | `sp-192` | `0x126ea0` | `?` | `?` |
| `bl` at 0xa9018 | `(unknown callee) 0x1b49c` | `[sp-96]` | `0x126ea8` | `sp-192` | `?` |
| `bl` at 0xa9020 | `(unknown callee) 0x1b22c` | `[sp-96]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa8784 | `[0x1499b4] != 0x0` | `[0x1499b4] == 0x0` |
| `blt` at 0xa8820 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa886c | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `blt` at 0xa8934 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa8944 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa8964 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa898c | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xa899c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa89e0 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `ldrls` at 0xa8a60 | `? <= 0x8 (unsigned, 32-bit)` | `? > 0x8 (unsigned, 32-bit)` |
| `bge` at 0xa8cd0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xa8d00 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `blt` at 0xa8d50 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xa8d90 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa8e60 | `[sp-100] != 0x0` | `[sp-100] == 0x0` |
| `beq` at 0xa8e6c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0xa8efc | `[sp-32] != 0x2` | `[sp-32] == 0x2` |
| `blt` at 0xa8f0c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xa8f24 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa8f68 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9064 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa8a60).

## `L1FRUSHUBInit`

- `L1FRUSHUBInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9094 to 0xa9294.
  Established. Word stores at unknown effective targets carry `0x1`, `[sp-32]` and `[0x1429b4]`
  (`str` at 0xa9108, `str` at 0xa9130, `str` at 0xa915c). The call to `0x1b124` has `[sp-28]`,
  `0x2b1ae0`, unknown, `[0x2b1af8]` (`bl` at 0xa9210). The opening global test takes its
  continuation for `[0x2b1af8] != 0x0` and falls through on zero (`bne` at 0xa90c0). SHUB record
  geometry and r0 at exit remain unproved (`ldmdb` at 0xa927c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa90a4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa90a8 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa90ac | load | 4 | `abs 0x2b1af4` | `-` |
| `str` at 0xa90b0 | store | 4 | `sp -0x24` | `[0x2b1af4]` |
| `ldr` at 0xa90b4 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa90b8 | load | 4 | `abs 0x2b1af8` | `-` |
| `str` at 0xa90cc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa90d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa90d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa90e4 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa90e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa90fc | load | 4 | `abs 0x2b1af8` | `-` |
| `str` at 0xa9108 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xa910c | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa9110 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9124 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa912c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9130 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xa9134 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa9138 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa914c | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa9154 | load | 4 | `abs 0xa9284` | `-` |
| `ldr` at 0xa9158 | load | 4 | `abs 0x1429b4` | `-` |
| `str` at 0xa915c | store | 4 | `(unknown)` | `[0x1429b4]` |
| `ldr` at 0xa9160 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa9164 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9178 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa9180 | load | 4 | `abs 0xa9288` | `-` |
| `str` at 0xa9184 | store | 4 | `(unknown)` | `0x12b5f0` |
| `ldr` at 0xa9188 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa918c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa91a0 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa91a8 | load | 4 | `abs 0xa928c` | `-` |
| `ldr` at 0xa91ac | load | 4 | `abs 0x1429b8` | `-` |
| `str` at 0xa91b0 | store | 4 | `(unknown)` | `[0x1429b8]` |
| `ldr` at 0xa91b4 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa91b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa91cc | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa91d4 | load | 4 | `abs 0xa9290` | `-` |
| `str` at 0xa91d8 | store | 4 | `(unknown)` | `0x12eb00` |
| `ldr` at 0xa91dc | load | 4 | `abs 0xa9284` | `-` |
| `ldr` at 0xa91e0 | load | 4 | `abs 0x1429b4` | `-` |
| `str` at 0xa91e8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa91ec | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa91f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9204 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa920c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa9218 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xa921c | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa9220 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9234 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa923c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa924c | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa9250 | load | 4 | `abs 0xa9280` | `-` |
| `ldr` at 0xa9254 | load | 4 | `abs 0x2b1af4` | `-` |
| `str` at 0xa925c | store | 4 | `abs 0x2b1af4` | `[0x2b1af4]+1` |
| `ldr` at 0xa9260 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9268 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xa926c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9274 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xa927c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa927c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa927c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa927c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9210 | `(unknown callee) 0x1b124` | `[sp-28]` | `0x2b1ae0` | `?` | `[0x2b1af8]` |
| `bl` at 0xa9248 | `(unknown callee) 0x1b43c` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa90c0 | `[0x2b1af8] != 0x0` | `[0x2b1af8] == 0x0` |
| `blt` at 0xa90dc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa927c | `r0` | `?` |

## `L1FRUNL4RCapture`

- `L1FRUNL4RCapture`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa92bc to 0xa9534.
  Established. The absolute word store at `0x2b1ae8` carries `0x1` (`str` at 0xa92e8). Two byte
  stores at unknown effective targets have unknown values (`strb` at 0xa9404, `strb` at 0xa9424).
  The first test takes its continuation for `[r0+24] != 0x0` and falls through on zero (`bne` at
  0xa92dc). The opening call to `0xe335c` uses entry r0, entry r1, entry r2, `[r0+24]` (`bl` at
  0xa92f0). NL4R capture fields and the common r0 remain unknown (`ldmdb` at 0xa9528).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa92cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa92d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa92d4 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0xa92e0 | load | 4 | `abs 0xa952c` | `-` |
| `str` at 0xa92e8 | store | 4 | `abs 0x2b1ae8` | `0x1` |
| `ldr` at 0xa92f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa92f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa92fc | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xa9304 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa9318 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa931c | load | 4 | `abs 0xa952c` | `-` |
| `ldr` at 0xa9320 | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa932c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9334 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xa9348 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xa934c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9364 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xa936c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa9370 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa9374 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9378 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa938c | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0xa9394 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa939c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa93a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa93a4 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa93bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa93d8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa93dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa93e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa93ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa93f4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa93fc | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa9404 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa940c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9414 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa941c | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa9424 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa942c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9434 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa943c | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa9444 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa944c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9454 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa945c | load | 1 | `sp -0x30` | `-` |
| `strb` at 0xa9460 | store | 1 | `(unknown)` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0xa9464 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9470 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9478 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa9480 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9490 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9498 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa94a0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa94a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa94a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa94b0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa94b8 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa94c0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa94c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa94c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa94d0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa94d8 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa94dc | store | 1 | `(unknown)` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xa94e4 | load | 4 | `abs 0xa952c` | `-` |
| `str` at 0xa94ec | store | 4 | `abs 0x2b1ae8` | `0x4` |
| `ldr` at 0xa94f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa94f4 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0xa94f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa94fc | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xa9500 | load | 4 | `abs 0xa9530` | `-` |
| `ldr` at 0xa9504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa950c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9514 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa951c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xa9528 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9528 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9528 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa92f0 | `(unknown callee) 0xe335c` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0xa9310 | `(unknown callee) 0x1b43c` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0xa93a8 | `(unknown callee) 0x27940` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0xa93c4 | `(unknown callee) 0xea468` | `ret(0x27940)` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0xa9510 | `(unknown callee) 0x2070c` | `0x126eb8` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0xa9524 | `(unknown callee) 0xe33c8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa92dc | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0xa9328 | `[0x2b1ae8] != 0x0` | `[0x2b1ae8] == 0x0` |
| `blt` at 0xa933c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa93e4 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9528 | `r0` | `?` |

## `L1FRUNL4RInit`

- `L1FRUNL4RInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9534 to 0xa9734.
  Established. Three word stores at unknown targets write `0x2`, `[sp-32]` and `[0x1429bc]`,
  respectively (`str` at 0xa95a8, `str` at 0xa95d0, `str` at 0xa95fc). The first global test takes
  its continuation for `[0x2b1af8] != 0x0` and falls through on zero (`bne` at 0xa9560). The call to
  `0x1b124` has `[sp-28]`, `0x2b1ae0`, unknown, `[0x2b1af8]` (`bl` at 0xa96b0).
  Initialization-record ownership and final r0 remain unidentified (`ldmdb` at 0xa971c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9544 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa9548 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa954c | load | 4 | `abs 0x2b1af4` | `-` |
| `str` at 0xa9550 | store | 4 | `sp -0x24` | `[0x2b1af4]` |
| `ldr` at 0xa9554 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa9558 | load | 4 | `abs 0x2b1af8` | `-` |
| `str` at 0xa956c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa9570 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9574 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9584 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa9588 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa959c | load | 4 | `abs 0x2b1af8` | `-` |
| `str` at 0xa95a8 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0xa95ac | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa95b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa95c4 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa95cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa95d0 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xa95d4 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa95d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa95ec | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa95f4 | load | 4 | `abs 0xa9724` | `-` |
| `ldr` at 0xa95f8 | load | 4 | `abs 0x1429bc` | `-` |
| `str` at 0xa95fc | store | 4 | `(unknown)` | `[0x1429bc]` |
| `ldr` at 0xa9600 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa9604 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9618 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa9620 | load | 4 | `abs 0xa9728` | `-` |
| `str` at 0xa9624 | store | 4 | `(unknown)` | `0x130df8` |
| `ldr` at 0xa9628 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa962c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9640 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa9648 | load | 4 | `abs 0xa972c` | `-` |
| `ldr` at 0xa964c | load | 4 | `abs 0x1429c0` | `-` |
| `str` at 0xa9650 | store | 4 | `(unknown)` | `[0x1429c0]` |
| `ldr` at 0xa9654 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa9658 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa966c | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa9674 | load | 4 | `abs 0xa9730` | `-` |
| `str` at 0xa9678 | store | 4 | `(unknown)` | `0x1319a0` |
| `ldr` at 0xa967c | load | 4 | `abs 0xa9724` | `-` |
| `ldr` at 0xa9680 | load | 4 | `abs 0x1429bc` | `-` |
| `str` at 0xa9688 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa968c | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa9690 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa96a4 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa96ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa96b8 | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0xa96bc | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa96c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa96d4 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa96dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa96e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa96ec | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa96f0 | load | 4 | `abs 0xa9720` | `-` |
| `ldr` at 0xa96f4 | load | 4 | `abs 0x2b1af4` | `-` |
| `str` at 0xa96fc | store | 4 | `abs 0x2b1af4` | `[0x2b1af4]+1` |
| `ldr` at 0xa9700 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9708 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xa970c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9714 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xa971c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa971c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa971c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa971c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa96b0 | `(unknown callee) 0x1b124` | `[sp-28]` | `0x2b1ae0` | `?` | `[0x2b1af8]` |
| `bl` at 0xa96e8 | `(unknown callee) 0x1b43c` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa9560 | `[0x2b1af8] != 0x0` | `[0x2b1af8] == 0x0` |
| `blt` at 0xa957c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa971c | `r0` | `?` |

## `FRUInit`

- `FRUInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbd330 to 0xbd340.
  Established. The recognized exit retains entry r0 (`ldmdb` at 0xbd33c). Memory accesses consist of
  stack saves (`push` at 0xbd334) and restores (`ldmdb` at 0xbd33c). There are no calls or conditional branches in the facts. An
  initialization role is not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0xbd33c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbd33c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbd33c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbd33c | `r0` | `r0` |

## `FRUEventInit`

- `FRUEventInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0710 to 0xc0788.
  Established. The initial test takes its bypass for `[r0] == 0x0` and falls through for nonzero
  (`beq` at 0xc0730). The call to `0x1b400` has all four candidates unknown (`bl` at 0xc0768). A
  later word store clears an unknown effective address (`str` at 0xc0780). Event-list ownership and
  the joined return value remain unresolved (`ldmdb` at 0xc0784).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0720 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc0724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0728 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc0734 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0738 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xc073c | store | 4 | `sp -0x18` | `[r0]` |
| `str` at 0xc0744 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc0748 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc075c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0760 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc0764 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc076c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0770 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0xc0778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0780 | store | 4 | `(unknown)` | `0x0` |
| `ldmdb` at 0xc0784 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0784 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0768 | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc0730 | `[r0] == 0x0` | `[r0] != 0x0` |
| `bne` at 0xc0750 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0784 | `r0` | `?` |

## `FRUEventAdd`

- `FRUEventAdd`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0788 to 0xc087c.
  Established. A word store writes entry r1 at `r0 +0x0` (`str` at 0xc07b8). Other word stores carry
  `[[sp-20]]` at `[sp-24] +0xa8` and `[sp-24]` at `[sp-20] +0x0` (`str` at 0xc07ec, `str` at
  0xc07f8). The call to `0x1b274` has entry r1, `[r0]`, `0x8`, entry r0 (`bl` at 0xc07d0); its
  signed-positive result takes the continuation and zero or negative falls through (`bgt` at
  0xc07dc). Sorting keys and r0 at exit remain unknown (`ldmdb` at 0xc0878).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0798 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc079c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc07a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc07a4 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc07b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc07b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc07b8 | store | 4 | `r0 +0x0` | `r1` |
| `ldr` at 0xc07c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc07c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc07c8 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xc07e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc07e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc07e8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc07ec | store | 4 | `[sp-24] +0xa8` | `[[sp-20]]` |
| `ldr` at 0xc07f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc07f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc07f8 | store | 4 | `[sp-20] +0x0` | `[sp-24]` |
| `ldr` at 0xc0800 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0804 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xc0808 | store | 4 | `sp -0x1c` | `[[sp-20]]` |
| `ldr` at 0xc080c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0810 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xc0814 | load | 4 | `[[sp-20]] +0xa8` | `-` |
| `str` at 0xc0818 | store | 4 | `sp -0x20` | `[[[sp-20]]+168]` |
| `ldr` at 0xc081c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc082c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0848 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc084c | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xc0850 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0854 | load | 4 | `[sp-32] +0xa8` | `-` |
| `str` at 0xc0858 | store | 4 | `sp -0x20` | `[[sp-32]+168]` |
| `ldr` at 0xc0860 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0864 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc0868 | store | 4 | `[sp-24] +0xa8` | `?` |
| `ldr` at 0xc086c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0870 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0874 | store | 4 | `[sp-28] +0xa8` | `[sp-24]` |
| `ldmdb` at 0xc0878 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0878 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0878 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc07d0 | `(unknown callee) 0x1b274` | `r1` | `[r0]` | `0x8` | `r0` |
| `bl` at 0xc0834 | `(unknown callee) 0x1b274` | `[sp-24]` | `?` | `0x8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc07ac | `[r0] != 0x0` | `[r0] == 0x0` |
| `bgt` at 0xc07dc | `ret(0x1b274) > 0x0 (signed, 32-bit)` | `ret(0x1b274) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xc0824 | `Z == 1` | `not (Z == 1)` |
| `bge` at 0xc0840 | `ret(0x1b274) >= 0x0 (signed, 32-bit)` | `ret(0x1b274) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0878 | `r0` | `?` |

## `FRUEventMerge`

- `FRUEventMerge`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc087c to 0xc08d8.
  Established. A word store clears an unknown effective target (`str` at 0xc08c0). The call to
  `0xc0788` receives `[sp-20]`, `[sp-28]`, unknown, `0x0` (`bl` at 0xc08cc). The loop branch retains
  only `Z == 0` and its negation for fall-through (`bne` at 0xc089c). Record-chain ownership and the
  exit r0 remain unestablished (`ldmdb` at 0xc08d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc088c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0890 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc0894 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc08a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc08a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc08ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc08b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc08b4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc08b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc08c0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc08c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc08c8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc08d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc08d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc08d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc08cc | `(unknown callee) 0xc0788` | `[sp-20]` | `[sp-28]` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc089c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc08d4 | `r0` | `?` |
