# ARM port setup and panel LED paths in 1.38.4

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/port-panel-led-facts.txt) retain all tool sections.

## `nl4r_init_ports`

- `nl4r_init_ports`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2898c to 0x28d94.
  Established. The global test takes its bypass for `[0x2ad25c] == 0x0` and falls through for
  nonzero (`beq` at 0x289b4). The opening call to `0x23f2c` carries `0x10156c`, entry r0, entry r2,
  `[0x2ad25c]` (`bl` at 0x289c0). The first loop branch retains only `ls flags condition (operands
  unknown)` and its negation for fall-through (`bls` at 0x289e4). Port-field meanings and r0 at the
  common exit remain unresolved (`ldmdb` at 0x28d6c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2899c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x289a0 | load | 4 | `abs 0x28d70` | `-` |
| `str` at 0x289a4 | store | 4 | `sp -0x28` | `0x101560` |
| `ldr` at 0x289a8 | load | 4 | `abs 0x28d74` | `-` |
| `ldr` at 0x289ac | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x289b8 | load | 4 | `abs 0x28d78` | `-` |
| `ldr` at 0x289bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x289c8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x289d0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x289d8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x289dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x289ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x289f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28a08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28a10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28a18 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x28a20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28a24 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x28a28 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x28a2c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x28a30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28a38 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x28a40 | load | 4 | `abs 0x28d7c` | `-` |
| `ldr` at 0x28a44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x28a50 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28a54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28a68 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x28a6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28a8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28a90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28aa8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x28aac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28acc | load | 4 | `abs 0x28d74` | `-` |
| `ldr` at 0x28ad0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x28adc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x28ae0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x28ae8 | load | 1 | `([sp-40] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x28aec | load | 4 | `abs 0x28d80` | `-` |
| `ldr` at 0x28af4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28afc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28b04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28b1c | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x28b20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28b2c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28b30 | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `ldr` at 0x28b38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28b44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28b58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x28b68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28b70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x28b78 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x28b7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28b84 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x28b90 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28b94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28ba4 | load | 4 | `abs 0x28d74` | `-` |
| `ldr` at 0x28ba8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x28bb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x28bb8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x28bc0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x28bc4 | load | 4 | `abs 0x28d84` | `-` |
| `ldr` at 0x28bd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28bd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28bf0 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x28bf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28c00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28c04 | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `ldr` at 0x28c0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28c14 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x28c20 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x28c24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28c34 | load | 4 | `abs 0x28d74` | `-` |
| `ldr` at 0x28c38 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x28c44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x28c48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x28c50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x28c54 | load | 4 | `abs 0x28d88` | `-` |
| `ldr` at 0x28c60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x28c68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28c80 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x28c84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28c90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28c94 | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `ldr` at 0x28c9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x28ca4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x28cac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x28cb0 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0x28cb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28cc8 | load | 4 | `abs 0x28d74` | `-` |
| `ldr` at 0x28ccc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x28cd8 | load | 4 | `abs 0x28d8c` | `-` |
| `ldr` at 0x28ce0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28cf8 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x28cfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28d08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28d0c | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `ldr` at 0x28d14 | load | 4 | `abs 0x28d74` | `-` |
| `ldr` at 0x28d18 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x28d24 | load | 4 | `abs 0x28d90` | `-` |
| `ldr` at 0x28d2c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x28d44 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x28d48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x28d54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x28d58 | store | 4 | `sp -0x2c` | `ret(0xec7d4)` |
| `str` at 0x28d64 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x28d68 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x28d6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x28d6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x28d6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x289c0 | `(unknown callee) 0x23f2c` | `0x10156c` | `r0` | `r2` | `[0x2ad25c]` |
| `bl` at 0x289f4 | `(unknown callee) 0x29480` | `?` | `?` | `?` | `?` |
| `bl` at 0x28a48 | `(unknown callee) 0x2070c` | `0x101590` | `?` | `?` | `?` |
| `bl` at 0x28a88 | `(unknown callee) 0xebed0` | `[sp-20]` | `?` | `0x1` | `sp-32` |
| `bl` at 0x28a94 | `(unknown callee) 0x29480` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x28ac8 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+10` | `0x2` | `sp-32` |
| `bl` at 0x28af8 | `(unknown callee) 0x23f2c` | `0x10159c` | `[([sp-40] + [sp-24])]:1` | `[sp-32]` | `[([sp-40] + [sp-24])]:1` |
| `bl` at 0x28b14 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `[sp-24]+10` | `0x0` |
| `bl` at 0x28b54 | `(unknown callee) 0xeb9a0` | `[sp-20]` | `[sp-24]+10` | `0x0` | `sp-32` |
| `bl` at 0x28bcc | `(unknown callee) 0x23f2c` | `0x1015c8` | `?` | `[sp-40]` | `?` |
| `bl` at 0x28be8 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x28c5c | `(unknown callee) 0x23f2c` | `0x1015ec` | `?` | `[sp-40]` | `?` |
| `bl` at 0x28c78 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `?` | `0x0` |
| `bl` at 0x28cc4 | `(unknown callee) 0xebed0` | `[sp-20]` | `0x5` | `0x0` | `sp-32` |
| `bl` at 0x28cdc | `(unknown callee) 0x23f2c` | `0x101610` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x28cf0 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `0x5` | `0x0` |
| `bl` at 0x28d28 | `(unknown callee) 0x23f2c` | `0x101630` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x28d3c | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x2` | `0x9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x289b4 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bls` at 0x289e4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x28a00 | `ret(0x29480) == 0x0` | `ret(0x29480) != 0x0` |
| `bls` at 0x28a5c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x28ad8 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28b28 | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |
| `bne` at 0x28b60 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bls` at 0x28b9c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x28bb0 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28bfc | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |
| `bls` at 0x28c2c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x28c40 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28c8c | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |
| `bls` at 0x28cd4 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28d04 | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |
| `bls` at 0x28d20 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x28d50 | `ret(0xec7d4) >= 0x0 (signed, 32-bit)` | `ret(0xec7d4) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x28d6c | `r0` | `?` |

## `TIOTest_SetSystemAceMode`

- `TIOTest_SetSystemAceMode`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2cdbc to 0x2ce84.
  Established. The opening call to `0x1f1dc` has `0x99`, entry r1, entry r2, entry r3 (`bl` at
  0x2cdd0); its zero result takes the bypass and nonzero falls through (`beq` at 0x2cde8). The exit
  carries `[sp-28]` (`ldmdb` at 0x2ce80). Mode selection, polarity and physical effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2cdd4 | store | 4 | `sp -0x14` | `ret(0x1f1dc)` |
| `str` at 0x2cddc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x2cde0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2cdf0 | store | 4 | `sp -0x20` | `0x80` |
| `str` at 0x2cdfc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2ce0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2ce18 | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x2ce1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ce2c | store | 4 | `sp -0x24` | `0x10` |
| `str` at 0x2ce38 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0x2ce48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2ce54 | store | 4 | `sp -0x18` | `ret(0xd6a1c)` |
| `ldr` at 0x2ce58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ce64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2ce70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2ce74 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x2ce78 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2ce80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2ce80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2ce80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2cdd0 | `(unknown callee) 0x1f1dc` | `0x99` | `r1` | `r2` | `r3` |
| `bl` at 0x2ce10 | `(unknown callee) 0xd6a1c` | `0x0` | `0xb` | `0x80` | `[sp-32]` |
| `bl` at 0x2ce4c | `(unknown callee) 0xd6a1c` | `0x0` | `0xc` | `0x38` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2cde8 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x2ce24 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `beq` at 0x2ce60 | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `bne` at 0x2ce6c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2ce80 | `r0` | `[sp-28]` |

## `opus_ip65_lbi_tap`

- `opus_ip65_lbi_tap`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2fc40 to 0x2fcb4.
  Established. The two deciding branches take their bypasses for entry `r1 == 0x0` and `r2 == 0x0`,
  respectively (`beq` at 0x2fc6c, `beq` at 0x2fc84). The single call to `0xd6a1c` has `0x0`, `0xa`,
  `0x50`, unknown (`bl` at 0x2fca4). The exit propagates that call result (`ldmdb` at 0x2fcb0). TAP
  signal meanings remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2fc50 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2fc54 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2fc58 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x2fc60 | store | 1 | `sp -0x1d` | `0x0` |
| `ldr` at 0x2fc64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2fc70 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x2fc78 | store | 1 | `sp -0x1d` | `0x10` |
| `ldr` at 0x2fc7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2fc88 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x2fc90 | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0x2fc94 | load | 1 | `sp -0x1d` | `-` |
| `ldmdb` at 0x2fcb0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2fcb0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2fcb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2fca4 | `(unknown callee) 0xd6a1c` | `0x0` | `0xa` | `0x50` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2fc6c | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0x2fc84 | `r2 == 0x0` | `r2 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2fcb0 | `r0` | `ret(0xd6a1c)` |

## `LBIReadLEDS`

- `LBIReadLEDS`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x399dc to 0x39d08.
  Established. A byte store has both unknown effective target and unknown value (`strb` at 0x39c78).
  The next byte store writes an unknown value at `[sp-24] +0x0` (`strb` at 0x39ca4). The result test
  takes its continuation for `ret(0xeac50) == 0x0` and falls through for nonzero (`beq` at 0x39a28).
  LED encoding and the merged r0 remain unknown (`ldmdb` at 0x39ce4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x399ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x399f0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x39a00 | store | 4 | `sp -0x38` | `0x28` |
| `ldr` at 0x39a04 | load | 4 | `abs 0x39ce8` | `-` |
| `str` at 0x39a08 | store | 4 | `sp -0x34` | `0x107910` |
| `ldr` at 0x39a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39a30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39a3c | load | 4 | `abs 0x39cec` | `-` |
| `str` at 0x39a40 | store | 4 | `sp -0x38` | `0x107920` |
| `str` at 0x39a44 | store | 4 | `sp -0x34` | `ret(0xea118)` |
| `ldr` at 0x39a4c | load | 4 | `abs 0x39cf0` | `-` |
| `ldr` at 0x39a50 | load | 4 | `abs 0x39cf4` | `-` |
| `ldr` at 0x39a5c | load | 4 | `abs 0x39cf8` | `-` |
| `str` at 0x39a60 | store | 4 | `sp -0x30` | `0xfffff059` |
| `str` at 0x39a78 | store | 4 | `sp -0x38` | `0x20` |
| `ldr` at 0x39a90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39a94 | load | 4 | `abs 0x39cfc` | `-` |
| `ldr` at 0x39aa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39aa8 | load | 4 | `abs 0x39cfc` | `-` |
| `str` at 0x39ab4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39ab8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39abc | load | 4 | `abs 0x39d00` | `-` |
| `ldr` at 0x39ac8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39ad0 | load | 4 | `abs 0x39d00` | `-` |
| `str` at 0x39adc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39ae0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39ae4 | load | 4 | `abs 0x39d04` | `-` |
| `ldr` at 0x39af0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39af8 | load | 4 | `abs 0x39d04` | `-` |
| `str` at 0x39b04 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39b08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39b18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39b2c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39b30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39b40 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39b54 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x39b64 | store | 4 | `sp -0x38` | `0x8` |
| `ldr` at 0x39b7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39b80 | load | 4 | `abs 0x39cfc` | `-` |
| `ldr` at 0x39b8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39b94 | load | 4 | `abs 0x39cfc` | `-` |
| `str` at 0x39ba0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39ba4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39ba8 | load | 4 | `abs 0x39d00` | `-` |
| `ldr` at 0x39bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39bbc | load | 4 | `abs 0x39d00` | `-` |
| `str` at 0x39bc8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39bcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39bd0 | load | 4 | `abs 0x39d04` | `-` |
| `ldr` at 0x39bdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39be4 | load | 4 | `abs 0x39d04` | `-` |
| `str` at 0x39bf0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39bf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39c04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x39c18 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39c1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39c2c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x39c40 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x39c48 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x39c4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39c5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39c60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39c68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39c70 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x39c78 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x39c7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39c80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39c84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39c90 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x39c9c | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x39ca4 | store | 1 | `[sp-24] +0x0` | `?` |
| `ldr` at 0x39ca8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39cac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39cb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39cb8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x39cc4 | load | 1 | `[sp-24] +0x1` | `-` |
| `strb` at 0x39ccc | store | 1 | `[sp-24] +0x1` | `?` |
| `ldr` at 0x39cd0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39cd8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x39ce0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x39ce4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39ce4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39ce4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x399f4 | `(unknown callee) 0xe335c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x39a1c | `(unknown callee) 0xeac50` | `[sp-20]` | `0x19` | `0x0` | `sp-32` |
| `bl` at 0x39a2c | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x39a34 | `(unknown callee) 0xea118` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x39a58 | `(unknown callee) 0x21e28` | `0x3` | `0x107714` | `0x1dd` | `0x0` |
| `bl` at 0x39a68 | `(unknown callee) 0xe33c8` | `ret(0xeac50)` | `?` | `?` | `ret(0xeac50)` |
| `bl` at 0x39a8c | `(unknown callee) 0xe2174` | `0x0` | `sp-32` | `0x0` | `sp-40` |
| `bl` at 0x39b78 | `(unknown callee) 0xe2174` | `0x20` | `sp-32` | `0x0` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x39a28 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `ble` at 0x39c54 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39ce4 | `r0` | `?` |

## `displayLedOn`

- `displayLedOn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51aa0 to 0x51bac.
  Established. The first call to `0xd69d4` uses `0x0`, `[0x1427a0]:1`, `0x40`, `[0x1427a0]:1` (`bl`
  at 0x51b14). The entry global-byte test takes its continuation on nonzero and falls through on
  zero (`bne` at 0x51ac0). The following selector takes its branch for `r0 == 0x1` and falls through
  otherwise (`beq` at 0x51ae0). Color mapping and r0 at exit remain unidentified (`ldmdb` at
  0x51b94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51ab0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x51ab4 | load | 4 | `abs 0x51b98` | `-` |
| `ldrb` at 0x51ab8 | load | 1 | `abs 0x1482a0` | `-` |
| `ldr` at 0x51ac4 | load | 4 | `abs 0x51b9c` | `-` |
| `str` at 0x51ac8 | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x51ad0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51ad4 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x51ad8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51ae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51af0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51b00 | load | 4 | `abs 0x51ba0` | `-` |
| `ldrb` at 0x51b04 | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51b1c | store | 4 | `sp -0x18` | `ret(0xd69d4)` |
| `ldr` at 0x51b24 | load | 4 | `abs 0x51ba0` | `-` |
| `ldrb` at 0x51b28 | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51b40 | store | 4 | `sp -0x18` | `ret(0xd69d4)` |
| `ldr` at 0x51b48 | load | 4 | `abs 0x51ba0` | `-` |
| `ldrb` at 0x51b4c | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51b64 | store | 4 | `sp -0x18` | `ret(0xd69d4)` |
| `ldr` at 0x51b6c | load | 4 | `abs 0x51ba4` | `-` |
| `str` at 0x51b70 | store | 4 | `sp -0x24` | `0x10b000` |
| `ldr` at 0x51b74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51b78 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x51b80 | load | 4 | `abs 0x51ba8` | `-` |
| `ldr` at 0x51b90 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x51b94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51b94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51b94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51b14 | `(unknown callee) 0xd69d4` | `0x0` | `[0x1427a0]:1` | `0x40` | `[0x1427a0]:1` |
| `bl` at 0x51b38 | `(unknown callee) 0xd69d4` | `0x0` | `[0x1427a0]:1` | `0x10` | `[0x1427a0]:1` |
| `bl` at 0x51b5c | `(unknown callee) 0xd69d4` | `0x0` | `[0x1427a0]:1` | `0x20` | `[0x1427a0]:1` |
| `bl` at 0x51b8c | `(unknown callee) 0x21e28` | `0x9` | `0x10af4c` | `0x1d8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51ac0 | `[0x1482a0]:1 != 0x0` | `[0x1482a0]:1 == 0x0` |
| `beq` at 0x51ae0 | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x51aec | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x51af8 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51b94 | `r0` | `?` |

## `displayLedOff`

- `displayLedOff`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51bac to 0x51cbc.
  Established. The call to `0xd6988` carries `0x0`, `[0x1427a0]:1`, `0x40`, `[0x1427a0]:1` at its
  first site (`bl` at 0x51c20). The global-byte guard takes the continuation for nonzero and falls
  through on zero (`bne` at 0x51bcc). The selector comparison takes its branch for `r0 == 0x1` and
  falls through on inequality (`beq` at 0x51bec). Physical LED polarity and the common r0 value are
  unresolved (`ldmdb` at 0x51ca0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51bbc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x51bc0 | load | 4 | `abs 0x51ca4` | `-` |
| `ldrb` at 0x51bc4 | load | 1 | `abs 0x1482a0` | `-` |
| `ldr` at 0x51bd0 | load | 4 | `abs 0x51ca8` | `-` |
| `str` at 0x51bd4 | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x51bdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51be0 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x51be4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51bf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51c0c | load | 4 | `abs 0x51cac` | `-` |
| `ldrb` at 0x51c10 | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51c28 | store | 4 | `sp -0x18` | `ret(0xd6988)` |
| `ldr` at 0x51c30 | load | 4 | `abs 0x51cac` | `-` |
| `ldrb` at 0x51c34 | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51c4c | store | 4 | `sp -0x18` | `ret(0xd6988)` |
| `ldr` at 0x51c54 | load | 4 | `abs 0x51cac` | `-` |
| `ldrb` at 0x51c58 | load | 1 | `abs 0x1427a0` | `-` |
| `str` at 0x51c70 | store | 4 | `sp -0x18` | `ret(0xd6988)` |
| `ldr` at 0x51c78 | load | 4 | `abs 0x51cb0` | `-` |
| `str` at 0x51c7c | store | 4 | `sp -0x24` | `0x10b000` |
| `ldr` at 0x51c80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51c84 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x51c8c | load | 4 | `abs 0x51cb4` | `-` |
| `ldr` at 0x51c90 | load | 4 | `abs 0x51cb8` | `-` |
| `ldr` at 0x51c9c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x51ca0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51ca0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51ca0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51c20 | `(unknown callee) 0xd6988` | `0x0` | `[0x1427a0]:1` | `0x40` | `[0x1427a0]:1` |
| `bl` at 0x51c44 | `(unknown callee) 0xd6988` | `0x0` | `[0x1427a0]:1` | `0x10` | `[0x1427a0]:1` |
| `bl` at 0x51c68 | `(unknown callee) 0xd6988` | `0x0` | `[0x1427a0]:1` | `0x20` | `[0x1427a0]:1` |
| `bl` at 0x51c98 | `(unknown callee) 0x21e28` | `0x9` | `0x10af4c` | `0x1f5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51bcc | `[0x1482a0]:1 != 0x0` | `[0x1482a0]:1 == 0x0` |
| `beq` at 0x51bec | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x51bf8 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x51c04 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51ca0 | `r0` | `?` |

## `displayLedStatus`

- `displayLedStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x51cbc to 0x51db0.
  Established. The first byte comparison takes its branch for `[0x1427a0]:1 != 0xff` and falls
  through on equality (`bne` at 0x51cdc). The call to `0x21e28` has `0x9`, `0x10af4c`, `0x218`,
  `0x0` (`bl` at 0x51d60). Final r0 is unknown (`ldmdb` at 0x51d9c). Status-bit and color
  interpretation cannot be established from these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51ccc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x51cd0 | load | 4 | `abs 0x51da0` | `-` |
| `ldrb` at 0x51cd4 | load | 1 | `abs 0x1427a0` | `-` |
| `ldr` at 0x51ce0 | load | 4 | `abs 0x51da4` | `-` |
| `str` at 0x51ce4 | store | 4 | `sp -0x1c` | `0xfffff8f8` |
| `ldr` at 0x51cec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51cf0 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x51cf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51d00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51d0c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x51d20 | store | 1 | `sp -0x15` | `0x40` |
| `strb` at 0x51d2c | store | 1 | `sp -0x15` | `0x10` |
| `strb` at 0x51d38 | store | 1 | `sp -0x15` | `0x20` |
| `ldr` at 0x51d40 | load | 4 | `abs 0x51da8` | `-` |
| `str` at 0x51d44 | store | 4 | `sp -0x28` | `0x10b000` |
| `ldr` at 0x51d48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x51d4c | store | 4 | `sp -0x24` | `r0` |
| `ldr` at 0x51d54 | load | 4 | `abs 0x51dac` | `-` |
| `ldr` at 0x51d64 | load | 4 | `abs 0x51da0` | `-` |
| `ldrb` at 0x51d68 | load | 1 | `abs 0x1427a0` | `-` |
| `ldrb` at 0x51d7c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x51d94 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x51d98 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x51d9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51d9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51d9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51d60 | `(unknown callee) 0x21e28` | `0x9` | `0x10af4c` | `0x218` | `0x0` |
| `bl` at 0x51d74 | `(unknown callee) 0xd6c2c` | `0x0` | `[0x1427a0]:1` | `?` | `[0x1427a0]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x51cdc | `[0x1427a0]:1 != 0xff` | `[0x1427a0]:1 == 0xff` |
| `beq` at 0x51cfc | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x51d08 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x51d14 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51d9c | `r0` | `?` |

## `frontpanelLEDInit`

- `frontpanelLEDInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x526e8 to 0x52770.
  Established. Word stores write unknown at absolute `0x148334` and `[sp-24]` at `0x148338` (`str`
  at 0x5274c, `str` at 0x52758). The initial zero test takes its bypass and falls through for
  nonzero (`beq` at 0x5270c). The call to `0x21e28` carries `0x9`, `0x10b1e8`, `0x3a`, `0x0` (`bl`
  at 0x52740). Panel-record ownership and exit r0 remain unproved (`ldmdb` at 0x5275c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x526f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x526fc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x52700 | load | 4 | `abs 0x52760` | `-` |
| `ldr` at 0x52704 | load | 4 | `abs 0x148334` | `-` |
| `ldr` at 0x52710 | load | 4 | `abs 0x52764` | `-` |
| `str` at 0x52714 | store | 4 | `sp -0x24` | `0x10b1b4` |
| `ldr` at 0x52718 | load | 4 | `abs 0x52760` | `-` |
| `ldr` at 0x5271c | load | 4 | `abs 0x148334` | `-` |
| `str` at 0x52720 | store | 4 | `sp -0x20` | `[0x148334]` |
| `ldr` at 0x52724 | load | 4 | `abs 0x52768` | `-` |
| `ldr` at 0x52728 | load | 4 | `abs 0x148338` | `-` |
| `str` at 0x5272c | store | 4 | `sp -0x1c` | `[0x148338]` |
| `ldr` at 0x52734 | load | 4 | `abs 0x5276c` | `-` |
| `ldr` at 0x52744 | load | 4 | `abs 0x52760` | `-` |
| `ldr` at 0x52748 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5274c | store | 4 | `abs 0x148334` | `?` |
| `ldr` at 0x52750 | load | 4 | `abs 0x52768` | `-` |
| `ldr` at 0x52754 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52758 | store | 4 | `abs 0x148338` | `[sp-24]` |
| `ldmdb` at 0x5275c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5275c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5275c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52740 | `(unknown callee) 0x21e28` | `0x9` | `0x10b1e8` | `0x3a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5270c | `[0x148334] == 0x0` | `[0x148334] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5275c | `r0` | `?` |

## `frontpanelLEDOn`

- `frontpanelLEDOn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52770 to 0x527dc.
  Established. The entry-r0 comparison takes its continuation for `r0 != 0x4` and falls through on
  equality (`bne` at 0x52790). A word store clears absolute `0x148328` (`str` at 0x527a8). The call
  to `0x527dc` has `0xff`, entry r1, `0x0`, `[sp-24]` (`bl` at 0x527bc). LED-mask meaning and the
  merged return are unknown (`ldmdb` at 0x527d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52780 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52784 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x52788 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52798 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x527a0 | load | 4 | `abs 0x527d8` | `-` |
| `str` at 0x527a8 | store | 4 | `abs 0x148328` | `0x0` |
| `ldr` at 0x527ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x527c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x527cc | store | 4 | `sp -0x1c` | `ret(0x52a6c)` |
| `ldr` at 0x527d0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x527d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x527d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x527d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x527bc | `(unknown callee) 0x527dc` | `0xff` | `r1` | `0x0` | `[sp-24]` |
| `bl` at 0x527c4 | `(unknown callee) 0x52a6c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52790 | `r0 != 0x4` | `r0 == 0x4` |
| `bne` at 0x527b4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x527d4 | `r0` | `?` |

## `frontpanelLEDOff`

- `frontpanelLEDOff`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x527dc to 0x52830.
  Established. The first test takes the continuation for `r0 != 0x4` and falls through for equality
  (`bne` at 0x527f8). That continuation clears absolute `0x148328` with a word store (`str` at
  0x52810). The call to `0x52b78` carries `[sp-20]`, entry r1, `0x0`, `0x148328` (`bl` at 0x52818).
  Panel-state interpretation and final r0 remain unresolved (`ldmdb` at 0x52828).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x527ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x527f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52800 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x52808 | load | 4 | `abs 0x5282c` | `-` |
| `str` at 0x52810 | store | 4 | `abs 0x148328` | `0x0` |
| `ldr` at 0x52814 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52820 | store | 4 | `sp -0x18` | `ret(0x52b78)` |
| `ldr` at 0x52824 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x52828 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52828 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52828 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52818 | `(unknown callee) 0x52b78` | `[sp-20]` | `r1` | `0x0` | `0x148328` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x527f8 | `r0 != 0x4` | `r0 == 0x4` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52828 | `r0` | `?` |

## `fp_led_on_raw`

- `fp_led_on_raw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52a6c to 0x52b78.
  Established. The byte store at absolute `0x14833e` has unknown value (`strb` at 0x52b4c). The call
  to `0xd6a1c` uses `0x0` and three unknown candidates (`bl` at 0x52b18). The loop branch retains
  only `lt flags condition (operands unknown)` for taken and its negation for fall-through (`blt` at
  0x52a98). The exit reports `[sp-24]` (`ldmdb` at 0x52b68). Output-pin encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52a7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52a84 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x52a88 | load | 4 | `abs 0x52b6c` | `-` |
| `ldr` at 0x52a8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52a90 | load | 4 | `abs 0x148338` | `-` |
| `ldr` at 0x52aa0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52aa8 | load | 4 | `abs 0x52b70` | `-` |
| `ldr` at 0x52aac | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52ab4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x52ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52ac8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52ad0 | load | 4 | `abs 0x52b70` | `-` |
| `ldr` at 0x52ad4 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52adc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x52ae0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52ae8 | load | 4 | `abs 0x52b70` | `-` |
| `ldr` at 0x52aec | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52af4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x52af8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52b00 | load | 4 | `abs 0x52b70` | `-` |
| `ldr` at 0x52b04 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52b0c | load | 1 | `(unknown)` | `-` |
| `str` at 0x52b20 | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x52b24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52b30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x52b34 | store | 4 | `sp -0x18` | `ret(0xd6a1c)` |
| `ldr` at 0x52b38 | load | 4 | `abs 0x52b74` | `-` |
| `ldr` at 0x52b3c | load | 4 | `abs 0x52b74` | `-` |
| `ldrb` at 0x52b40 | load | 1 | `abs 0x14833e` | `-` |
| `ldrb` at 0x52b44 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x52b4c | store | 1 | `abs 0x14833e` | `?` |
| `ldr` at 0x52b50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x52b58 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x52b60 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x52b68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52b68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52b68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52b18 | `(unknown callee) 0xd6a1c` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x52a98 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x52ac4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52b2c | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52b68 | `r0` | `[sp-24]` |

## `fp_led_off_raw`

- `fp_led_off_raw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52b78 to 0x52c74.
  Established. A byte store writes an unknown value at absolute `0x14833e` (`strb` at 0x52c48). The
  call to `0xd6a1c` has `0x0`, unknown, unknown, `0x0` (`bl` at 0x52c10). The first loop condition
  remains `lt flags condition (operands unknown)` for taken and its negation for fall-through (`blt`
  at 0x52ba4). R0 exits as `[sp-24]` (`ldmdb` at 0x52c64). Clear-mask polarity is unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52b88 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52b90 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x52b94 | load | 4 | `abs 0x52c68` | `-` |
| `ldr` at 0x52b98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52b9c | load | 4 | `abs 0x148338` | `-` |
| `ldr` at 0x52bac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52bb4 | load | 4 | `abs 0x52c6c` | `-` |
| `ldr` at 0x52bb8 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52bc0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x52bc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52bd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52bdc | load | 4 | `abs 0x52c6c` | `-` |
| `ldr` at 0x52be0 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52be8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x52bec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52bf4 | load | 4 | `abs 0x52c6c` | `-` |
| `ldr` at 0x52bf8 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52c00 | load | 1 | `(unknown)` | `-` |
| `str` at 0x52c18 | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x52c1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52c28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x52c2c | store | 4 | `sp -0x18` | `ret(0xd6a1c)` |
| `ldr` at 0x52c30 | load | 4 | `abs 0x52c70` | `-` |
| `ldr` at 0x52c34 | load | 4 | `abs 0x52c70` | `-` |
| `ldrb` at 0x52c38 | load | 1 | `sp -0x14` | `-` |
| `ldrb` at 0x52c40 | load | 1 | `abs 0x14833e` | `-` |
| `strb` at 0x52c48 | store | 1 | `abs 0x14833e` | `?` |
| `ldr` at 0x52c4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x52c54 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x52c5c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x52c64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52c64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52c64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52c10 | `(unknown callee) 0xd6a1c` | `0x0` | `?` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x52ba4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x52bd0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x52c24 | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52c64 | `r0` | `[sp-24]` |

## `pwrStatusLEDInit`

- `pwrStatusLEDInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ec0c to 0x5ec98.
  Established. Word stores write unknown at absolute `0x148414` and `[sp-24]` at `0x148418` (`str`
  at 0x5ec70, `str` at 0x5ec7c). The zero test takes the bypass and nonzero falls through (`beq` at
  0x5ec30). The opening call to `0x21e28` uses `0x9`, `0x10c408`, `0x2c`, `0x0` (`bl` at 0x5ec64).
  The exit carries `ret(0x5ec98)` (`ldmdb` at 0x5ec84). Power-status record fields remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ec1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5ec20 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5ec24 | load | 4 | `abs 0x5ec88` | `-` |
| `ldr` at 0x5ec28 | load | 4 | `abs 0x148414` | `-` |
| `ldr` at 0x5ec34 | load | 4 | `abs 0x5ec8c` | `-` |
| `str` at 0x5ec38 | store | 4 | `sp -0x24` | `0x10c3e0` |
| `ldr` at 0x5ec3c | load | 4 | `abs 0x5ec88` | `-` |
| `ldr` at 0x5ec40 | load | 4 | `abs 0x148414` | `-` |
| `str` at 0x5ec44 | store | 4 | `sp -0x20` | `[0x148414]` |
| `ldr` at 0x5ec48 | load | 4 | `abs 0x5ec90` | `-` |
| `ldr` at 0x5ec4c | load | 4 | `abs 0x148418` | `-` |
| `str` at 0x5ec50 | store | 4 | `sp -0x1c` | `[0x148418]` |
| `ldr` at 0x5ec58 | load | 4 | `abs 0x5ec94` | `-` |
| `ldr` at 0x5ec68 | load | 4 | `abs 0x5ec88` | `-` |
| `ldr` at 0x5ec6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5ec70 | store | 4 | `abs 0x148414` | `?` |
| `ldr` at 0x5ec74 | load | 4 | `abs 0x5ec90` | `-` |
| `ldr` at 0x5ec78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5ec7c | store | 4 | `abs 0x148418` | `[sp-24]` |
| `ldmdb` at 0x5ec84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ec84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ec84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ec64 | `(unknown callee) 0x21e28` | `0x9` | `0x10c408` | `0x2c` | `0x0` |
| `bl` at 0x5ec80 | `(unknown callee) 0x5ec98` | `?` | `?` | `0x148418` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ec30 | `[0x148414] == 0x0` | `[0x148414] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ec84 | `r0` | `ret(0x5ec98)` |

## `pwrStatusLEDUpdate`

- `pwrStatusLEDUpdate`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ec98 to 0x5eed8.
  Established. The call to `0x3dc58` has `0x0` and three unknown candidates (`bl` at 0x5ed24). The
  outer loop branch retains an unknown-operands `lt` predicate and its negation for fall-through
  (`blt` at 0x5ecc8). The next loop condition similarly retains only an unknown-operands `le`
  predicate (`ble` at 0x5ed14). Power-state aggregation and the exit r0 are unresolved (`ldmdb` at
  0x5eec8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ecac | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x5ecb4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x5ecb8 | load | 4 | `abs 0x5eecc` | `-` |
| `ldr` at 0x5ecbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ecc0 | load | 4 | `abs 0x148418` | `-` |
| `str` at 0x5ecd4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x5ecd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ece0 | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ece4 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ecec | load | 1 | `(unknown)` | `-` |
| `str` at 0x5ecf0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5ecf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ecfc | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ed00 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ed08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ed0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5ed20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5ed2c | store | 4 | `sp -0x20` | `ret(0x3dc58)` |
| `ldr` at 0x5ed30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ed3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ed40 | load | 4 | `abs 0x5eed4` | `-` |
| `ldr` at 0x5ed4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5ed54 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x5ed58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5ed60 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x5ed68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ed74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ed7c | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ed80 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ed88 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ed94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ed9c | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5eda0 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5eda8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5edac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5edb4 | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5edb8 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5edc0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5edd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5eddc | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ede0 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ede8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5edec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5edf4 | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5edf8 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ee00 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ee10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ee1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ee24 | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ee28 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ee30 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ee3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ee44 | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ee48 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ee50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ee54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ee5c | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ee60 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ee68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ee7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ee84 | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5ee88 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5ee90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ee94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ee9c | load | 4 | `abs 0x5eed0` | `-` |
| `ldr` at 0x5eea0 | load | 4 | `abs 0x148414` | `-` |
| `ldrb` at 0x5eea8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5eeb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5eec0 | store | 4 | `sp -0x14` | `?` |
| `ldmdb` at 0x5eec8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5eec8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5eec8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ed24 | `(unknown callee) 0x3dc58` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5edcc | `(unknown callee) 0xd6988` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5ee0c | `(unknown callee) 0xd69d4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5ee74 | `(unknown callee) 0xd69d4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5eeb4 | `(unknown callee) 0xd6988` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5ecc8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x5ed14 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x5ed38 | `ret(0x3dc58) == 0x2` | `ret(0x3dc58) != 0x2` |
| `beq` at 0x5ed48 | `ret(0x3dc58) == 0xfffff9b9` | `ret(0x3dc58) != 0xfffff9b9` |
| `bne` at 0x5ed70 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5ed90 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5ee18 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5ee38 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5eec8 | `r0` | `?` |

## `TIOSysComInt`

- `TIOSysComInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53d84 to 0x53e88.
  Established. The first loop branch retains `lt flags condition (operands unknown)` for taken and
  its negation for fall-through (`blt` at 0x53dcc). The call to `0x21e28` has `0x1`, `0x10b2c8`,
  `0x241`, `0x0` (`bl` at 0x53df0). A later branch takes `Z == 0` and falls through on its negation
  (`bne` at 0x53e40). Interrupt matching and r0 at exit remain unknown (`ldmdb` at 0x53e70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53d94 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x53da0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x53da8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x53db0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x53db8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x53dbc | load | 4 | `abs 0x53e74` | `-` |
| `ldrb` at 0x53dc0 | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x53dc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53dd0 | load | 4 | `abs 0x53e78` | `-` |
| `str` at 0x53dd4 | store | 4 | `sp -0x28` | `0x10b390` |
| `ldrb` at 0x53dd8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x53ddc | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x53de4 | load | 4 | `abs 0x53e7c` | `-` |
| `ldr` at 0x53de8 | load | 4 | `abs 0x53e80` | `-` |
| `ldr` at 0x53df8 | load | 4 | `abs 0x53e84` | `-` |
| `ldr` at 0x53e00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53e1c | load | 4 | `abs 0x148350` | `-` |
| `str` at 0x53e24 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x53e2c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x53e30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53e34 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x53e38 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x53e44 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x53e48 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x53e60 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53e64 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x53e70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53e70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53e70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53df0 | `(unknown callee) 0x21e28` | `0x1` | `0x10b2c8` | `0x241` | `0x0` |
| `bl` at 0x53e6c | `(unknown callee) 0x837e4` | `?` | `?` | `(r3 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x53dcc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x53e40 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x53e5c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53e70 | `r0` | `?` |
