# ARM protocol, discovery and argument helpers in 1.38.4

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
Accesses and return values cover known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/protocol-discovery-argument-helpers-facts.txt) retain all tool sections.

## `syscom_process_inbound_request`

- `syscom_process_inbound_request`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83ac4 to 0x83ea4.
  Established. A word store sets `sp -0x80` to `0x1` (`str` at 0x83ae4). The opening call to
  `0x8260c` carries entry r0, `0x2`, entry r2 and `0x0` as candidates (`bl` at 0x83af8); its zero
  result takes the branch and nonzero falls through (`beq` at 0x83b04). Request decoding semantics
  and the merged return remain unknown (`ldmdb` at 0x83e90).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x83ad4 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x83ad8 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x83adc | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x83ae4 | store | 4 | `sp -0x80` | `0x1` |
| `str` at 0x83aec | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x83af0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83b08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83b0c | load | 4 | `abs 0x83e94` | `-` |
| `ldr` at 0x83b10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83b14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x83b20 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83b28 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x83b34 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83b3c | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x83b4c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83b54 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x83b64 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83b6c | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x83b74 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x83b7c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83b84 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x83b90 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83b98 | load | 1 | `[sp-32] +0x6` | `-` |
| `str` at 0x83ba0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x83ba4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x83bb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83bbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83bc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x83bd8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83be0 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x83bec | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83bf4 | load | 1 | `[sp-32] +0x6` | `-` |
| `ldr` at 0x83c14 | load | 4 | `abs 0x83e98` | `-` |
| `ldr` at 0x83c18 | load | 4 | `abs 0x2b1720` | `-` |
| `str` at 0x83c1c | store | 4 | `(unknown)` | `[0x2b1720]` |
| `ldr` at 0x83c20 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83c24 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x83c2c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x83c48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x83c50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83c54 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x83c6c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x83c70 | load | 4 | `abs 0x83e9c` | `-` |
| `ldr` at 0x83c74 | load | 4 | `abs 0x149710` | `-` |
| `ldr` at 0x83c80 | load | 4 | `abs 0x83e9c` | `-` |
| `ldr` at 0x83c84 | load | 4 | `abs 0x149710` | `-` |
| `ldr` at 0x83c88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83c8c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x83c90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83c94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x83ca0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x83cb0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83cb8 | load | 1 | `[sp-32] +0x7` | `-` |
| `str` at 0x83cbc | store | 4 | `sp -0x94` | `[[sp-32]+7]:1` |
| `ldr` at 0x83cc0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x83cc8 | store | 4 | `sp -0x20` | `[sp-32]+8` |
| `ldr` at 0x83ccc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x83cd4 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `ldr` at 0x83ce4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83ce8 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x83cec | store | 4 | `sp -0x98` | `[[sp-32]]:1` |
| `ldr` at 0x83cf0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x83cfc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x83d18 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83d20 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x83d2c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83d34 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x83d44 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83d4c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x83d5c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83d64 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x83d78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x83d80 | store | 4 | `sp -0x20` | `[sp-32]+5` |
| `ldr` at 0x83d88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83da0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x83dac | store | 4 | `sp -0xa8` | `0x1` |
| `ldr` at 0x83dc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83dd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x83ddc | store | 4 | `sp -0x20` | `(ret(0x1b22c) + [sp-32])+1` |
| `ldr` at 0x83dec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83df4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83df8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x83e0c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83e10 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x83e18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x83e20 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x83e30 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x83e34 | store | 4 | `sp -0xa8` | `[sp-128]` |
| `str` at 0x83e3c | store | 4 | `sp -0xa4` | `sp-124` |
| `str` at 0x83e44 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x83e4c | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x83e64 | store | 4 | `sp -0x84` | `ret(0xb6a9c)` |
| `ldr` at 0x83e68 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x83e74 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x83e80 | load | 4 | `abs 0x83ea0` | `-` |
| `ldr` at 0x83e84 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x83e90 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x83e90 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x83e90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x83e90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x83e90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83af8 | `(unknown callee) 0x8260c` | `r0` | `0x2` | `r2` | `0x0` |
| `bl` at 0x83b18 | `(unknown callee) 0x826a4` | `[sp-28]` | `0x111d0c` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x83bc4 | `(unknown callee) 0x85a38` | `[sp-28]` | `[sp-32]` | `[sp-36]` | `[sp-140]-12289` |
| `bl` at 0x83c04 | `(unknown callee) 0x6907c` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x83c58 | `(unknown callee) 0x837a4` | `[sp-28]` | `[sp-144]` | `sp-24` | `?` |
| `mov` at 0x83c9c | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-144]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x83d74 | `(unknown callee) 0x690d8` | `sp-128` | `sp-124` | `?` | `?` |
| `bl` at 0x83d8c | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0x83dbc | `(unknown callee) 0x692f8` | `sp-128` | `sp-124` | `[sp-32]+1` | `ret(0x1b22c)+1` |
| `bl` at 0x83dc4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x83e08 | `(unknown callee) 0x6917c` | `sp-128` | `sp-124` | `[sp-32]+1` | `?` |
| `bl` at 0x83e5c | `(unknown callee) 0xb6a9c` | `sp-44` | `sp-136` | `0x0` | `0x0` |
| `bl` at 0x83e78 | `(unknown callee) 0x21674` | `ret(0xb6a9c)` | `?` | `?` | `ret(0xb6a9c)` |
| `bl` at 0x83e8c | `(unknown callee) 0x2070c` | `0x111d1c` | `[sp-28]` | `ret(0x21674)` | `ret(0x21674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x83b04 | `ret(0x8260c) == 0x0` | `ret(0x8260c) != 0x0` |
| `bhi` at 0x83bb4 | `[sp-140]-12289 > 0x2 (unsigned, 32-bit)` | `[sp-140]-12289 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x83c7c | `[0x149710] == 0x0` | `[0x149710] != 0x0` |
| `bls` at 0x83ca8 | `[sp-36] <= 0x7 (unsigned, 32-bit)` | `[sp-36] > 0x7 (unsigned, 32-bit)` |
| `bne` at 0x83cdc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x83cf8 | `[[sp-32]]:1 == 0x0` | `[[sp-32]]:1 != 0x0` |
| `beq` at 0x83d04 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0x83e70 | `ret(0xb6a9c) == 0x0` | `ret(0xb6a9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x83e90 | `r0` | `?` |

## `syscom_process_inbound_event`

- `syscom_process_inbound_event`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83ea4 to 0x841cc.
  Established. A word store saves `[0x2ae5b4]` at `sp -0x74` (`str` at 0x83ec8). The first branch
  takes `[0x2ae5b4] != 0x0` and falls through on zero (`bne` at 0x83ed8). The call to `0x1b304` has
  four unknown candidates (`bl` at 0x83f4c). Event field meanings and final r0 value remain unknown
  (`ldmdb` at 0x841b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x83eb4 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x83eb8 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x83ebc | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x83ec0 | load | 4 | `abs 0x841b8` | `-` |
| `ldr` at 0x83ec4 | load | 4 | `abs 0x2ae5b4` | `-` |
| `str` at 0x83ec8 | store | 4 | `sp -0x74` | `[0x2ae5b4]` |
| `ldr` at 0x83ecc | load | 4 | `abs 0x841b8` | `-` |
| `ldr` at 0x83ed0 | load | 4 | `abs 0x2ae5b4` | `-` |
| `str` at 0x83ee0 | store | 4 | `sp -0x74` | `0x1` |
| `ldr` at 0x83ee4 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x83ee8 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x83eec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x83ef4 | store | 4 | `sp -0x68` | `r1+1` |
| `ldrb` at 0x83ef8 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x83f00 | store | 1 | `sp -0x69` | `([sp-36]:1-1 & 0xff)` |
| `str` at 0x83f08 | store | 4 | `sp -0x70` | `0x0` |
| `ldrb` at 0x83f0c | load | 1 | `sp -0x69` | `-` |
| `strb` at 0x83f14 | store | 1 | `sp -0x69` | `?` |
| `ldr` at 0x83f28 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x83f2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x83f38 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x83f40 | load | 1 | `sp -0x69` | `-` |
| `ldr` at 0x83f44 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x83f50 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x83f58 | store | 4 | `sp -0x70` | `[sp-112]+1` |
| `ldr` at 0x83f60 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x83f68 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x83f70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x83f74 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x83f7c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x83f88 | load | 4 | `abs 0x841bc` | `-` |
| `ldr` at 0x83f8c | load | 4 | `abs 0x2b1728` | `-` |
| `ldr` at 0x83f94 | load | 4 | `[0x2b1728] +0x3c` | `-` |
| `ldr` at 0x83fa4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83fac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x83fb8 | store | 4 | `sp -0x78` | `0x1` |
| `ldr` at 0x83fcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83fe8 | load | 4 | `abs 0x841c0` | `-` |
| `ldr` at 0x83fec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x83ff0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x83ff8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x83ffc | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x84004 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x84008 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84014 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8401c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84020 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x8402c | store | 4 | `sp -0x60` | `ret(0x837a4)` |
| `str` at 0x84034 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x84038 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8403c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x8404c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x84068 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x84078 | store | 4 | `sp -0x60` | `ret(0x837a4)` |
| `ldr` at 0x84080 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x84088 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x84090 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x840a4 | load | 4 | `abs 0x841bc` | `-` |
| `ldr` at 0x840a8 | load | 4 | `abs 0x2b1720` | `-` |
| `str` at 0x840ac | store | 4 | `(unknown)` | `[0x2b1720]` |
| `ldr` at 0x840c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x840d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x840d4 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x840ec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x840f0 | load | 4 | `abs 0x841c4` | `-` |
| `ldr` at 0x840f4 | load | 4 | `abs 0x14971c` | `-` |
| `ldr` at 0x84100 | load | 4 | `abs 0x841c4` | `-` |
| `ldr` at 0x84104 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8410c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x84114 | load | 4 | `abs 0x14971c` | `-` |
| `ldr` at 0x84118 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84148 | load | 4 | `abs 0x841bc` | `-` |
| `ldr` at 0x8414c | load | 4 | `abs 0x2b1738` | `-` |
| `str` at 0x84150 | store | 4 | `sp -0x78` | `[0x2b1738]` |
| `ldr` at 0x8415c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x8416c | store | 4 | `sp -0x58` | `ret(0xb6d54)` |
| `ldr` at 0x84170 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8417c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x84188 | load | 4 | `abs 0x841c8` | `-` |
| `ldr` at 0x8418c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x84198 | load | 4 | `abs 0x841bc` | `-` |
| `ldr` at 0x8419c | load | 4 | `abs 0x2b1790` | `-` |
| `ldr` at 0x841a8 | load | 4 | `abs 0x841bc` | `-` |
| `ldr` at 0x841ac | load | 4 | `abs 0x2b1798` | `-` |
| `ldmdb` at 0x841b4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x841b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x841b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x841b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x841b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83f4c | `(unknown callee) 0x1b304` | `?` | `?` | `?` | `?` |
| `bl` at 0x83f98 | `(unknown callee) 0x6907c` | `sp-80` | `sp-76` | `[[0x2b1728]+60]` | `[0x2b1728]` |
| `bl` at 0x83fc8 | `(unknown callee) 0x692f8` | `sp-80` | `sp-76` | `[sp-32]+1` | `[sp-36]-1` |
| `bl` at 0x83fd4 | `(unknown callee) 0x8260c` | `[sp-28]` | `0x4` | `?` | `?` |
| `bl` at 0x83ff4 | `(unknown callee) 0x826a4` | `[sp-28]` | `0x111d48` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x84018 | `(unknown callee) 0x5e8d4` | `[sp-28]+2` | `[sp-92]` | `?` | `[sp-28]+2` |
| `bl` at 0x84024 | `(unknown callee) 0x837a4` | `[sp-28]` | `[sp-92]` | `?` | `?` |
| `bl` at 0x84058 | `(unknown callee) 0x5ea88` | `?` | `?` | `?` | `?` |
| `bl` at 0x84070 | `(unknown callee) 0x837a4` | `[sp-84]` | `0x4` | `?` | `ret(0x5ea88)` |
| `bl` at 0x84094 | `(unknown callee) 0x67d30` | `?` | `?` | `?` | `?` |
| `bl` at 0x840d8 | `(unknown callee) 0x837a4` | `[sp-28]` | `[sp-92]` | `sp-24` | `?` |
| `mov` at 0x84124 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-32]+1` | `[sp-36]-1` | `[sp-36]-1` |
| `bl` at 0x8412c | `(unknown callee) 0x1f1dc` | `0x81` | `?` | `?` | `?` |
| `bl` at 0x84164 | `(unknown callee) 0xb6d54` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x84180 | `(unknown callee) 0x21674` | `ret(0xb6d54)` | `?` | `?` | `ret(0xb6d54)` |
| `bl` at 0x84194 | `(unknown callee) 0x2070c` | `0x111d58` | `[sp-28]` | `ret(0x21674)` | `ret(0x21674)` |
| `bl` at 0x841b0 | `(unknown callee) 0x87540` | `[0x2b1798]` | `?` | `?` | `0x2b1720` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x83ed8 | `[0x2ae5b4] != 0x0` | `[0x2ae5b4] == 0x0` |
| `bne` at 0x83f20 | `? != 0xff` | `? == 0xff` |
| `bne` at 0x83f34 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x83fe0 | `ret(0x8260c) == 0x0` | `ret(0x8260c) != 0x0` |
| `blt` at 0x84044 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x84064 | `ret(0x5ea88) != 0x4` | `ret(0x5ea88) == 0x4` |
| `beq` at 0x840fc | `[0x14971c] == 0x0` | `[0x14971c] != 0x0` |
| `beq` at 0x84138 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x84178 | `ret(0xb6d54) == 0x0` | `ret(0xb6d54) != 0x0` |
| `beq` at 0x841a4 | `[0x2b1790] == 0x0` | `[0x2b1790] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x841b4 | `r0` | `?` |

## `syscom_process_outbound_request`

- `syscom_process_outbound_request`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84518 to 0x84994.
  Established. The first call to `0x3d8e0` receives entry r0 through r3 as candidates (`bl` at
  0x8452c). Equality with `0x1` takes the following branch and inequality falls through (`beq` at
  0x84538). The return records `ret(0xb6bdc)` (`ldmdb` at 0x8496c). Request transport selection and
  acknowledgement meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x84528 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x84554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84558 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8456c | load | 4 | `abs 0x84970` | `-` |
| `str` at 0x84598 | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x845b0 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x845b4 | load | 4 | `abs 0x84974` | `-` |
| `ldr` at 0x845bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x845c0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x845cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x845d0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x845dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x845e0 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x845e8 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x845ec | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x845f4 | store | 4 | `sp -0x8c` | `?` |
| `str` at 0x84600 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x84604 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x84608 | store | 4 | `sp -0x84` | `[sp-140]` |
| `ldr` at 0x8460c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x84618 | store | 4 | `sp -0x88` | `ret(0x83734)` |
| `ldr` at 0x8461c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x84628 | store | 4 | `sp -0x84` | `ret(0x8376c)` |
| `ldr` at 0x8462c | load | 4 | `abs 0x84974` | `-` |
| `ldr` at 0x84630 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x84654 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8465c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84668 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8466c | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x8467c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x84680 | load | 1 | `[sp-20] +0x30` | `-` |
| `strb` at 0x84684 | store | 1 | `sp -0x8d` | `([[sp-20]+48]:1 & 0xff)` |
| `ldr` at 0x8468c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x8469c | store | 1 | `sp -0x8d` | `(ret(0xb7944) & 0xff)` |
| `ldrb` at 0x846a0 | load | 1 | `sp -0x8d` | `-` |
| `strb` at 0x846a4 | store | 1 | `sp -0x7c` | `?` |
| `strb` at 0x846ac | store | 1 | `sp -0x7b` | `0x0` |
| `ldr` at 0x846b0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x846cc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x846d0 | load | 4 | `abs 0x84978` | `-` |
| `ldr` at 0x846e0 | load | 4 | `abs 0x84974` | `-` |
| `ldr` at 0x846e4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x84708 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x84714 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84718 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x84730 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x84734 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x84744 | store | 4 | `sp -0x80` | `0x0` |
| `ldrb` at 0x8474c | load | 1 | `sp -0x84` | `-` |
| `strb` at 0x84750 | store | 1 | `sp -0x7c` | `([sp-132]:1 & 0xff)` |
| `ldr` at 0x84754 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x84758 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x84768 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8476c | load | 1 | `[sp-20] +0x30` | `-` |
| `strb` at 0x84770 | store | 1 | `sp -0x8e` | `([[sp-20]+48]:1 & 0xff)` |
| `ldr` at 0x84778 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x84788 | store | 1 | `sp -0x8e` | `(ret(0xb7944) & 0xff)` |
| `ldrb` at 0x8478c | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x84790 | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0x84794 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x847b0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x847b4 | load | 4 | `abs 0x8497c` | `-` |
| `ldr` at 0x847c4 | load | 4 | `abs 0x84974` | `-` |
| `ldr` at 0x847c8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x847ec | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x847f8 | load | 4 | `abs 0x84974` | `-` |
| `ldr` at 0x847fc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x84820 | load | 4 | `abs 0x2b1734` | `-` |
| `str` at 0x84830 | store | 4 | `sp -0x94` | `0x2` |
| `ldr` at 0x84838 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8483c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8484c | store | 4 | `sp -0x80` | `ret(0xba8cc)` |
| `ldr` at 0x84850 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x84860 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x84874 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x8487c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x84888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8488c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x848a0 | load | 4 | `abs 0x84980` | `-` |
| `ldr` at 0x848a8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x848ac | load | 4 | `abs 0x84984` | `-` |
| `ldr` at 0x848c0 | load | 4 | `abs 0x84988` | `-` |
| `ldr` at 0x848d4 | load | 4 | `abs 0x8498c` | `-` |
| `ldr` at 0x848dc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x848e0 | load | 4 | `abs 0x84984` | `-` |
| `ldr` at 0x848ec | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x84910 | load | 4 | `abs 0x84990` | `-` |
| `str` at 0x8493c | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x84954 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x84958 | load | 4 | `abs 0x84974` | `-` |
| `ldr` at 0x84960 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84964 | load | 4 | `sp -0x78` | `-` |
| `ldmdb` at 0x8496c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8496c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8496c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8452c | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x84550 | `(unknown callee) 0x6907c` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x8455c | `(unknown callee) 0x694c4` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x84570 | `(unknown callee) 0x1b49c` | `sp-100` | `0x111dbc` | `ret(0x694c4)` | `sp-100` |
| `bl` at 0x8457c | `(unknown callee) 0x1b22c` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x845a8 | `(unknown callee) 0x692f8` | `sp-120` | `sp-116` | `sp-100` | `ret(0x1b22c)` |
| `bl` at 0x845c4 | `(unknown callee) 0xb6bdc` | `0x2b1720` | `0x0` | `[sp-20]` | `[sp-120]` |
| `bl` at 0x84610 | `(unknown callee) 0x83734` | `[sp-140]` | `?` | `?` | `[sp-140]` |
| `bl` at 0x84620 | `(unknown callee) 0x8376c` | `[sp-132]` | `?` | `?` | `ret(0x83734)` |
| `bl` at 0x84694 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x846b8 | `(unknown callee) 0x8260c` | `[sp-136]` | `0x8` | `?` | `0x0` |
| `bl` at 0x846dc | `(unknown callee) 0x826a4` | `[sp-136]` | `0x111de0` | `sp-124` | `0x1` |
| `mov` at 0x84728 | `(unknown callee) (unknown target)` | `[sp-136]` | `sp-124` | `0x1` | `?` |
| `bl` at 0x84780 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x8479c | `(unknown callee) 0x8260c` | `[sp-136]` | `0x10` | `?` | `?` |
| `bl` at 0x847c0 | `(unknown callee) 0x826a4` | `[sp-136]` | `0x111df0` | `sp-124` | `0x2` |
| `bl` at 0x84844 | `(unknown callee) 0xba8cc` | `?` | `?` | `[sp-136]` | `sp-124` |
| `bl` at 0x84878 | `(unknown callee) 0x6907c` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x84890 | `(unknown callee) 0x697f4` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x848a4 | `(unknown callee) 0x1b49c` | `sp-100` | `0x111e04` | `ret(0x697f4)` | `sp-100` |
| `bl` at 0x848c4 | `(unknown callee) 0x1b2d4` | `sp-100` | `0x111e18` | `[sp-128]` | `sp-100` |
| `bl` at 0x848d8 | `(unknown callee) 0x1b2d4` | `sp-100` | `0x111e30` | `[sp-128]` | `sp-100` |
| `bl` at 0x848f0 | `(unknown callee) 0x21674` | `[sp-128]` | `?` | `[sp-128]` | `0xfffff69e` |
| `bl` at 0x84904 | `(unknown callee) 0x1b2d4` | `sp-100` | `ret(0x21674)` | `ret(0x21674)` | `sp-100` |
| `bl` at 0x84914 | `(unknown callee) 0x1b2d4` | `sp-100` | `0x111e3c` | `?` | `sp-100` |
| `bl` at 0x84920 | `(unknown callee) 0x1b22c` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x8494c | `(unknown callee) 0x692f8` | `sp-120` | `sp-116` | `sp-100` | `ret(0x1b22c)` |
| `bl` at 0x84968 | `(unknown callee) 0xb6bdc` | `0x2b1720` | `0x0` | `[sp-20]` | `[sp-120]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x84538 | `ret(0x3d8e0) == 0x1` | `ret(0x3d8e0) != 0x1` |
| `blt` at 0x845d8 | `[[sp-20]+24] < 0x0 (signed, 32-bit)` | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x84664 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x84678 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x846c4 | `ret(0x8260c) == 0x0` | `ret(0x8260c) != 0x0` |
| `bne` at 0x8473c | `ret(unknown) != 0x1` | `ret(unknown) == 0x1` |
| `bne` at 0x84764 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x847a8 | `ret(0x8260c) == 0x0` | `ret(0x8260c) != 0x0` |
| `bne` at 0x84858 | `ret(0xba8cc) != 0x2` | `ret(0xba8cc) == 0x2` |
| `beq` at 0x84884 | `[sp-128] == 0x0` | `[sp-128] != 0x0` |
| `bne` at 0x848b4 | `[sp-128] != 0xfffff69e` | `[sp-128] == 0xfffff69e` |
| `beq` at 0x848e8 | `[sp-128] == 0xfffff69e` | `[sp-128] != 0xfffff69e` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8496c | `r0` | `ret(0xb6bdc)` |

## `syscom_process_outbound_event`

- `syscom_process_outbound_event`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84994 to 0x85074.
  Established. The opening branch takes `Z == 0` and falls through on `not (Z == 0)` (`bne` at
  0x849bc). The call to `0x83734` has `[0x2b172c]`, entry r1, entry r2 and `0x2b1720` as candidates
  (`bl` at 0x849cc). The final r0 expression is unknown (`ldmdb` at 0x85068). Outbound-event
  encoding remains unproved beyond the listed accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x849a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x849ac | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x849b0 | load | 4 | `abs 0x8506c` | `-` |
| `ldr` at 0x849b4 | load | 4 | `abs 0x2b172c` | `-` |
| `ldr` at 0x849c4 | load | 4 | `abs 0x8506c` | `-` |
| `ldr` at 0x849c8 | load | 4 | `abs 0x2b172c` | `-` |
| `str` at 0x849d4 | store | 4 | `sp -0xb0` | `ret(0x83734)` |
| `ldr` at 0x849d8 | load | 4 | `abs 0x8506c` | `-` |
| `ldr` at 0x849dc | load | 4 | `abs 0x2b172c` | `-` |
| `str` at 0x849e8 | store | 4 | `sp -0xac` | `ret(0x8376c)` |
| `ldr` at 0x849ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x849f0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x849fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84a00 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x84a18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84a1c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x84a2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84a30 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x84a3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84a40 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x84a58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84a5c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldrb` at 0x84a6c | load | 1 | `sp -0xac` | `-` |
| `strb` at 0x84a74 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x84a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84a7c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x84a80 | store | 4 | `sp -0xa8` | `[[sp-20]+28]` |
| `ldr` at 0x84a84 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84a8c | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x84a90 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84a98 | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x84a9c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84aa4 | store | 1 | `sp -0x91` | `?` |
| `ldrb` at 0x84aa8 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x84aac | store | 1 | `sp -0x90` | `([sp-168]:1 & 0xff)` |
| `ldr` at 0x84ab0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x84ab4 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x84ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84ac8 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x84acc | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0x84ad4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x84ae0 | store | 4 | `sp -0xb4` | `ret(0xb7944)` |
| `ldr` at 0x84ae4 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x84ae8 | store | 4 | `sp -0xa8` | `[sp-180]` |
| `ldr` at 0x84aec | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84af4 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x84af8 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84b00 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x84b04 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84b0c | store | 1 | `sp -0x8d` | `?` |
| `ldrb` at 0x84b10 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x84b14 | store | 1 | `sp -0x8c` | `([sp-168]:1 & 0xff)` |
| `strb` at 0x84b1c | store | 1 | `sp -0x8b` | `0x0` |
| `str` at 0x84b24 | store | 4 | `sp -0x98` | `0xa` |
| `str` at 0x84b2c | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x84b30 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x84b3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x84b40 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x84b44 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x84b54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84b58 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x84b6c | load | 1 | `(unknown)` | `-` |
| `str` at 0x84b74 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x84b78 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x84b84 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x84b90 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x84ba0 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x84bb0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x84bc4 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x84bd0 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x84bec | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x84bf4 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x84bf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84bfc | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x84c10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x84c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84c24 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x84c38 | load | 4 | `(unknown)` | `-` |
| `str` at 0x84c3c | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x84c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84c48 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x84c50 | store | 4 | `sp -0xb8` | `ret(0xb7944)` |
| `ldr` at 0x84c54 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x84c58 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x84c60 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84c78 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84c80 | store | 1 | `(unknown)` | `?` |
| `str` at 0x84c88 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x84c90 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84ca8 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84cb0 | store | 1 | `(unknown)` | `?` |
| `str` at 0x84cb8 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x84cc0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84cd8 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x84ce0 | store | 1 | `(unknown)` | `?` |
| `str` at 0x84ce8 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x84cf0 | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x84d08 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x84d0c | store | 1 | `(unknown)` | `([sp-168]:1 & 0xff)` |
| `str` at 0x84d14 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x84d1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84d20 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x84d34 | load | 2 | `(unknown)` | `-` |
| `str` at 0x84d38 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x84d3c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84d40 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x84d54 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x84d60 | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x84d78 | load | 1 | `sp -0xa4` | `-` |
| `strb` at 0x84d84 | store | 1 | `(unknown)` | `?` |
| `str` at 0x84d8c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x84d90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84d94 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x84da8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x84db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84dbc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x84dd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x84dd4 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x84ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84de0 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x84de8 | store | 4 | `sp -0xbc` | `ret(0xb7944)` |
| `ldr` at 0x84df0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84dfc | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x84e00 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x84e08 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84e0c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x84e14 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x84e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84e20 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x84e34 | load | 2 | `(unknown)` | `-` |
| `str` at 0x84e38 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x84e3c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84e40 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x84e54 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x84e60 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x84e7c | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x84e84 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x84e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84e8c | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x84ea0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x84eb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84eb4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x84ec8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x84ecc | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x84ed4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84ed8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x84ee0 | store | 4 | `sp -0xc0` | `ret(0xb7944)` |
| `ldr` at 0x84ee8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84ef4 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x84efc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84f00 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x84f08 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x84f0c | load | 1 | `sp -0x8b` | `-` |
| `strb` at 0x84f14 | store | 1 | `sp -0x8b` | `?` |
| `ldr` at 0x84f18 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x84f20 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x84f28 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x84f44 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x84f48 | load | 4 | `abs 0x85070` | `-` |
| `ldr` at 0x84f50 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84f58 | load | 4 | `abs 0x8506c` | `-` |
| `ldr` at 0x84f5c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x84f80 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x84f88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84f94 | load | 4 | `abs 0x8506c` | `-` |
| `ldr` at 0x84f98 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x84fbc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x84fc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84fcc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x84fd4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x84fe4 | load | 4 | `abs 0x8506c` | `-` |
| `ldr` at 0x84fe8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8500c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x85018 | load | 4 | `abs 0x8506c` | `-` |
| `ldr` at 0x8501c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x85040 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8504c | load | 4 | `sp -0x98` | `-` |
| `str` at 0x85050 | store | 4 | `sp -0xc8` | `[sp-152]` |
| `ldr` at 0x85058 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8505c | load | 4 | `sp -0xb0` | `-` |
| `ldmdb` at 0x85068 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x85068 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x85068 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x849cc | `(unknown callee) 0x83734` | `[0x2b172c]` | `r1` | `r2` | `0x2b1720` |
| `bl` at 0x849e0 | `(unknown callee) 0x8376c` | `[0x2b172c]` | `?` | `?` | `0x2b1720` |
| `bl` at 0x84adc | `(unknown callee) 0xb7944` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x84c4c | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x84de4 | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x84e04 | `(unknown callee) 0x1b304` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x84edc | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x84ef8 | `(unknown callee) 0x1b508` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x84f30 | `(unknown callee) 0x8260c` | `[sp-176]` | `0x20` | `?` | `?` |
| `bl` at 0x84f54 | `(unknown callee) 0x826a4` | `[sp-176]` | `0x111e40` | `sp-148` | `[sp-152]` |
| `mov` at 0x84fdc | `(unknown callee) (unknown target)` | `[sp-176]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x85064 | `(unknown callee) 0xba8cc` | `?` | `?` | `[sp-176]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x849bc | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x849f8 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x84a10 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x84a28 | `? == 0x5` | `? != 0x5` |
| `bge` at 0x84a38 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x84a50 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x84a68 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x84ac0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x84b38 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `blo` at 0x84b4c | `C == 0` | `not (C == 0)` |
| `beq` at 0x84b80 | `[sp-196] == 0x1` | `[sp-196] != 0x1` |
| `bgt` at 0x84b8c | `[sp-196] > 0x1 (signed, 32-bit)` | `[sp-196] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x84b98 | `[sp-196] == 0x0` | `[sp-196] != 0x0` |
| `beq` at 0x84ba8 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `bls` at 0x84bbc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x84c1c | `? != 0x0` | `? == 0x0` |
| `bls` at 0x84d4c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x84db4 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x84e4c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x84eac | `? != 0x0` | `? == 0x0` |
| `beq` at 0x84f3c | `ret(0x8260c) == 0x0` | `ret(0x8260c) != 0x0` |
| `bne` at 0x84f90 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x85068 | `r0` | `?` |

## `syscom_process_outbound_response`

- `syscom_process_outbound_response`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x85074 to 0x857e4.
  Established. Two word stores clear `sp -0x98` and `sp -0x9c` (`str` at 0x8508c, `str` at 0x85094).
  The signed branch takes `[r0+24] < 0x0 (signed, 32-bit)` and falls through on nonnegative values
  (`blt` at 0x850a4). The first call to `0x83734` receives `[sp-184]`, entry r1, entry r2 and
  unknown as candidates (`bl` at 0x850dc). Response encoding and final r0 value remain unknown
  (`ldmdb` at 0x857d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x85084 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8508c | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x85094 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x85098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8509c | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x850a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x850ac | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x850b4 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x850b8 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x850c0 | store | 4 | `sp -0xb8` | `?` |
| `str` at 0x850cc | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x850d0 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x850d4 | store | 4 | `sp -0xb0` | `[sp-184]` |
| `ldr` at 0x850d8 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x850e4 | store | 4 | `sp -0xb4` | `ret(0x83734)` |
| `ldr` at 0x850e8 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x850f4 | store | 4 | `sp -0xb0` | `ret(0x8376c)` |
| `ldr` at 0x850f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x850fc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x85108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8510c | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x85114 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x85118 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x85120 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x85128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8512c | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x85134 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x85138 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x8513c | store | 4 | `sp -0xac` | `[sp-188]` |
| `ldr` at 0x85140 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x8514c | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85150 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8515c | load | 1 | `sp -0xb0` | `-` |
| `strb` at 0x85164 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x85168 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8516c | load | 1 | `[sp-20] +0x13` | `-` |
| `ldr` at 0x8517c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85180 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x85190 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85194 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x85198 | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0x851a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x851ac | store | 4 | `sp -0xc0` | `ret(0xb7944)` |
| `ldr` at 0x851b0 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x851b4 | store | 4 | `sp -0xa8` | `[sp-192]` |
| `ldr` at 0x851b8 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x851c0 | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x851c4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x851cc | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x851d0 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x851d8 | store | 1 | `sp -0x91` | `?` |
| `ldrb` at 0x851dc | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x851e0 | store | 1 | `sp -0x90` | `([sp-168]:1 & 0xff)` |
| `strb` at 0x851e8 | store | 1 | `sp -0x8f` | `0x0` |
| `str` at 0x851f0 | store | 4 | `sp -0x98` | `0x6` |
| `strb` at 0x851fc | store | 1 | `sp -0x93` | `?` |
| `strb` at 0x85204 | store | 1 | `sp -0x92` | `?` |
| `strb` at 0x8520c | store | 1 | `sp -0x91` | `?` |
| `strb` at 0x85214 | store | 1 | `sp -0x90` | `?` |
| `ldr` at 0x85218 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8521c | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85224 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x85228 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8522c | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85234 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x85238 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8523c | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85244 | store | 1 | `sp -0x8d` | `?` |
| `ldr` at 0x85248 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8524c | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85250 | store | 1 | `sp -0x8c` | `([[sp-20]+19]:1 & 0xff)` |
| `str` at 0x85258 | store | 4 | `sp -0x98` | `0x9` |
| `str` at 0x85260 | store | 4 | `sp -0x9c` | `0x1` |
| `str` at 0x85268 | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x8526c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x85278 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8527c | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x85280 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85290 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85294 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x852a8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x852b0 | store | 4 | `sp -0xd0` | `?` |
| `ldr` at 0x852b4 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x852c0 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x852cc | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x852dc | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x852ec | load | 4 | `sp -0x98` | `-` |
| `str` at 0x85300 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x8530c | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x85328 | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x85330 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x85334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85338 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x8534c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8535c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85360 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85374 | load | 4 | `(unknown)` | `-` |
| `str` at 0x85378 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x85380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85384 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8538c | store | 4 | `sp -0xc4` | `ret(0xb7944)` |
| `ldr` at 0x85390 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x85394 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x8539c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x853b4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x853bc | store | 1 | `(unknown)` | `?` |
| `str` at 0x853c4 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x853cc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x853e4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x853ec | store | 1 | `(unknown)` | `?` |
| `str` at 0x853f4 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x853fc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85414 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x8541c | store | 1 | `(unknown)` | `?` |
| `str` at 0x85424 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x8542c | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x85444 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x85448 | store | 1 | `(unknown)` | `([sp-168]:1 & 0xff)` |
| `str` at 0x85450 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x85458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8545c | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x85470 | load | 2 | `(unknown)` | `-` |
| `str` at 0x85474 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x85478 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8547c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85490 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x8549c | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x854b4 | load | 1 | `sp -0xa4` | `-` |
| `strb` at 0x854c0 | store | 1 | `(unknown)` | `?` |
| `str` at 0x854c8 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x854cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x854d0 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x854e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x854f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x854f8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x8550c | load | 4 | `(unknown)` | `-` |
| `str` at 0x85510 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x85518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8551c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85524 | store | 4 | `sp -0xc8` | `ret(0xb7944)` |
| `ldr` at 0x8552c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85538 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x8553c | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x85544 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85548 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85550 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x85558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8555c | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x85570 | load | 2 | `(unknown)` | `-` |
| `str` at 0x85574 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x85578 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8557c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85590 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x8559c | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x855b8 | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x855c0 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x855c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x855c8 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x855dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x855ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x855f0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85604 | load | 4 | `(unknown)` | `-` |
| `str` at 0x85608 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x85610 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85614 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8561c | store | 4 | `sp -0xcc` | `ret(0xb7944)` |
| `ldr` at 0x85624 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85630 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x85638 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8563c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85644 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x85648 | load | 1 | `sp -0x8f` | `-` |
| `strb` at 0x85650 | store | 1 | `sp -0x8f` | `([sp-143]:1+1 & 0xff)` |
| `ldr` at 0x85654 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8565c | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x85664 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85680 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85684 | load | 4 | `abs 0x857d8` | `-` |
| `ldr` at 0x8568c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85694 | load | 4 | `abs 0x857dc` | `-` |
| `ldr` at 0x85698 | load | 4 | `abs 0x149714` | `-` |
| `ldr` at 0x856a4 | load | 4 | `abs 0x857dc` | `-` |
| `ldr` at 0x856ac | load | 4 | `abs 0x149714` | `-` |
| `ldr` at 0x856b0 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x856b4 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x856b8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x856c4 | load | 4 | `abs 0x857e0` | `-` |
| `ldr` at 0x856c8 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x856ec | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x856f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x85700 | load | 4 | `abs 0x857e0` | `-` |
| `ldr` at 0x85704 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85728 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x85734 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x85738 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85740 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85750 | load | 4 | `abs 0x857e0` | `-` |
| `ldr` at 0x85754 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85778 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x85784 | load | 4 | `abs 0x857e0` | `-` |
| `ldr` at 0x85788 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x857ac | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x857b8 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x857bc | store | 4 | `sp -0xd4` | `[sp-152]` |
| `ldr` at 0x857c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x857c8 | load | 4 | `sp -0xb4` | `-` |
| `ldmdb` at 0x857d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x857d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x857d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x850dc | `(unknown callee) 0x83734` | `[sp-184]` | `r1` | `r2` | `?` |
| `bl` at 0x850ec | `(unknown callee) 0x8376c` | `[sp-176]` | `?` | `?` | `ret(0x83734)` |
| `bl` at 0x85154 | `(unknown callee) 0x867a8` | `[sp-180]` | `[sp-20]` | `?` | `[sp-188]` |
| `bl` at 0x851a8 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x85388 | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x85520 | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x85540 | `(unknown callee) 0x1b304` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x85618 | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x85634 | `(unknown callee) 0x1b508` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x8566c | `(unknown callee) 0x8260c` | `[sp-180]` | `0x40` | `?` | `?` |
| `bl` at 0x85690 | `(unknown callee) 0x826a4` | `[sp-180]` | `0x111e50` | `sp-148` | `[sp-152]` |
| `mov` at 0x856c0 | `(unknown callee) (unknown target)` | `[sp-180]` | `[sp-176]` | `sp-148` | `[sp-152]` |
| `mov` at 0x85748 | `(unknown callee) (unknown target)` | `[sp-180]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x857d0 | `(unknown callee) 0xba8cc` | `?` | `?` | `[sp-180]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x850a4 | `[r0+24] < 0x0 (signed, 32-bit)` | `[r0+24] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x85104 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x85148 | `[sp-188] != 0x15` | `[sp-188] == 0x15` |
| `beq` at 0x85174 | `[[sp-20]+19]:1 == 0x0` | `[[sp-20]+19]:1 != 0x0` |
| `bne` at 0x8518c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x85274 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `blo` at 0x85288 | `C == 0` | `not (C == 0)` |
| `beq` at 0x852bc | `[sp-208] == 0x1` | `[sp-208] != 0x1` |
| `bgt` at 0x852c8 | `[sp-208] > 0x1 (signed, 32-bit)` | `[sp-208] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x852d4 | `[sp-208] == 0x0` | `[sp-208] != 0x0` |
| `beq` at 0x852e4 | `[sp-208] == 0x2` | `[sp-208] != 0x2` |
| `bls` at 0x852f8 | `[sp-152]+5 <= 0x7f (unsigned, 32-bit)` | `[sp-152]+5 > 0x7f (unsigned, 32-bit)` |
| `bne` at 0x85358 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x85488 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x854f0 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x85588 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x855e8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x85678 | `ret(0x8260c) == 0x0` | `ret(0x8260c) != 0x0` |
| `beq` at 0x856a0 | `[0x149714] == 0x0` | `[0x149714] != 0x0` |
| `bne` at 0x856fc | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x857d4 | `r0` | `?` |

## `init`

- `init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82a78 to 0x832e8.
  Established. Word stores write entry r0 at absolute `0x2b1730` and `ret(0x1b124)` at `0x2b1734`
  (`str` at 0x82a98, `str` at 0x82ad0). The call to `0x1b124` has unknown, `[sp-24]`, unknown and
  unknown as candidates (`bl` at 0x82ac8). The loop decision retains unknown operands (`blt` at
  0x82b24). Communication-record geometry and the common return remain unknown (`ldmdb` at 0x832ac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82a88 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x82a8c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x82a90 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82a94 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82a98 | store | 4 | `abs 0x2b1730` | `r0` |
| `ldr` at 0x82a9c | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82aa0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82ad0 | store | 4 | `abs 0x2b1734` | `ret(0x1b124)` |
| `ldr` at 0x82ad4 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82ad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x82afc | load | 4 | `abs 0x2b1734` | `-` |
| `str` at 0x82b10 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x82b14 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82b18 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82b1c | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0x82b2c | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82b30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82b54 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82b5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82b70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82b78 | load | 4 | `(unknown)` | `-` |
| `str` at 0x82b7c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x82b80 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x82b84 | store | 4 | `(unknown)` | `[sp-88]` |
| `ldr` at 0x82b88 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x82b94 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x82ba4 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82ba8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82bcc | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82bd4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82be8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82bf0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x82bfc | store | 4 | `(unknown)` | `ret(0xc9b90)` |
| `ldr` at 0x82c00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x82c0c | load | 4 | `abs 0x832b4` | `-` |
| `str` at 0x82c10 | store | 4 | `sp -0x78` | `0x111bdc` |
| `ldr` at 0x82c14 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x82c18 | store | 4 | `sp -0x74` | `[sp-52]` |
| `ldr` at 0x82c20 | load | 4 | `abs 0x832b8` | `-` |
| `ldr` at 0x82c24 | load | 4 | `abs 0x832bc` | `-` |
| `ldr` at 0x82c34 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82c38 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82c5c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82c68 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x82c78 | store | 1 | `sp -0x3f` | `0x1` |
| `strb` at 0x82c80 | store | 1 | `sp -0x3c` | `0x1` |
| `ldr` at 0x82c84 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82c88 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82cac | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82cb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x82cc4 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82cc8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82cec | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82cf4 | load | 4 | `abs 0x832c0` | `-` |
| `str` at 0x82cf8 | store | 4 | `(unknown)` | `0x86da0` |
| `ldr` at 0x82cfc | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82d00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82d24 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82d2c | load | 4 | `abs 0x832c4` | `-` |
| `str` at 0x82d30 | store | 4 | `(unknown)` | `0x86ea4` |
| `ldr` at 0x82d38 | load | 4 | `abs 0x832c8` | `-` |
| `ldr` at 0x82d3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82d44 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82d48 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82d6c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82d74 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82d88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82d90 | load | 4 | `(unknown)` | `-` |
| `str` at 0x82d94 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x82d98 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82d9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82dc0 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82dc8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82ddc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82de4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x82de8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x82dec | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82df0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82e14 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82e1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82e30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82e38 | load | 4 | `(unknown)` | `-` |
| `str` at 0x82e3c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x82e40 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82e44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82e68 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82e70 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82e84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82e8c | load | 4 | `(unknown)` | `-` |
| `str` at 0x82e90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x82e9c | load | 4 | `abs 0x832cc` | `-` |
| `ldr` at 0x82ea0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82ea8 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82eac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82ed0 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82efc | load | 4 | `abs 0x832d0` | `-` |
| `ldr` at 0x82f00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82f08 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82f0c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82f30 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82f54 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82f58 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82f7c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82f84 | load | 4 | `abs 0x832d4` | `-` |
| `str` at 0x82f88 | store | 4 | `(unknown)` | `0x86fd0` |
| `ldr` at 0x82f8c | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82f90 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82fb4 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82fbc | load | 4 | `abs 0x832d8` | `-` |
| `str` at 0x82fc0 | store | 4 | `(unknown)` | `0x87248` |
| `ldr` at 0x82fc4 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x82fc8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x82fec | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x82ff4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x82ff8 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0x82ffc | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x83000 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83024 | load | 4 | `abs 0x2b1734` | `-` |
| `str` at 0x83030 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x83034 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x83038 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8305c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x83064 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83078 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83080 | load | 4 | `(unknown)` | `-` |
| `str` at 0x83084 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x83090 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x83094 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x830b8 | load | 4 | `abs 0x2b1734` | `-` |
| `str` at 0x830d8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x830dc | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x830e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83104 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8310c | load | 4 | `(unknown)` | `-` |
| `str` at 0x83110 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x83114 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x83120 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x83130 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x83134 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83158 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x83164 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x83168 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8318c | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x83198 | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x8319c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x831c0 | load | 4 | `abs 0x2b1734` | `-` |
| `str` at 0x831d0 | store | 4 | `sp -0x78` | `0x100` |
| `str` at 0x831d8 | store | 4 | `sp -0x74` | `0x0` |
| `str` at 0x831e0 | store | 4 | `sp -0x70` | `0x0` |
| `str` at 0x831e8 | store | 4 | `sp -0x6c` | `0x0` |
| `str` at 0x831f0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x831f4 | load | 4 | `abs 0x832dc` | `-` |
| `str` at 0x831f8 | store | 4 | `sp -0x64` | `0x86bfc` |
| `ldr` at 0x831fc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x83200 | store | 4 | `sp -0x60` | `[sp-52]` |
| `ldr` at 0x83220 | load | 4 | `abs 0x832e0` | `-` |
| `ldr` at 0x83224 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8322c | load | 4 | `abs 0x832b0` | `-` |
| `ldr` at 0x83230 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83254 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x83264 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x83268 | store | 4 | `sp -0x78` | `[sp-52]` |
| `str` at 0x83270 | store | 4 | `sp -0x74` | `0xb20` |
| `str` at 0x83278 | store | 4 | `sp -0x70` | `0x19` |
| `str` at 0x83280 | store | 4 | `sp -0x6c` | `0xa` |
| `str` at 0x83288 | store | 4 | `sp -0x68` | `0x9` |
| `ldr` at 0x83290 | load | 4 | `abs 0x832e4` | `-` |
| `ldr` at 0x8329c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x832a4 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldmdb` at 0x832ac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x832ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x832ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x832ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82ac8 | `(unknown callee) 0x1b124` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0x82b08 | `(unknown callee) 0x1b43c` | `[0x2b1734]` | `0x0` | `?` | `?` |
| `bl` at 0x82bf8 | `(unknown callee) 0xc9b90` | `?` | `0x2` | `?` | `?` |
| `bl` at 0x82c2c | `(unknown callee) 0x21e28` | `0x1` | `0x111830` | `0x166` | `0x0` |
| `bl` at 0x82c70 | `(unknown callee) 0xca450` | `?` | `sp-84` | `sp-84` | `?` |
| `bl` at 0x82cc0 | `(unknown callee) 0xca48c` | `?` | `sp-84` | `sp-84` | `?` |
| `bl` at 0x82d40 | `(unknown callee) 0x2070c` | `0x111bf8` | `[sp-52]` | `?` | `[sp-88]` |
| `bl` at 0x82ea4 | `(unknown callee) 0x1b49c` | `sp-44` | `0x111c08` | `[sp-52]` | `sp-44` |
| `bl` at 0x82ef0 | `(unknown callee) 0x22920` | `?` | `sp-44` | `0x0` | `0x9` |
| `bl` at 0x82f04 | `(unknown callee) 0x1b49c` | `sp-44` | `0x111c14` | `[sp-52]` | `sp-44` |
| `bl` at 0x82f50 | `(unknown callee) 0x22920` | `?` | `sp-44` | `0x0` | `0x9` |
| `bl` at 0x830c4 | `(unknown callee) 0x1f1dc` | `0x8` | `[sp-52]` | `?` | `[0x2b1734]` |
| `bl` at 0x83214 | `(unknown callee) 0xba79c` | `?` | `?` | `0x100` | `?` |
| `bl` at 0x83228 | `(unknown callee) 0x1b49c` | `sp-44` | `0x111c20` | `[sp-52]` | `sp-44` |
| `bl` at 0x83298 | `(unknown callee) 0x223f0` | `?` | `sp-44` | `0x842a8` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x82b24 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x82b90 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `beq` at 0x82b9c | `[sp-88] == 0x1` | `[sp-88] != 0x1` |
| `bne` at 0x82c08 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8308c | `? != 0x2` | `? == 0x2` |
| `beq` at 0x8311c | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0x83128 | `[sp-92] == 0x1` | `[sp-92] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x832ac | `r0` | `?` |

## `L1CfgDiscoveryResponse`

- `L1CfgDiscoveryResponse`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c1f0 to 0x6c734.
  Established. The opening call to `0x6c0d0` has `0x10f238`, entry r1, entry r2 and `[r0+12]` as
  candidates (`bl` at 0x6c214). The first decision has an unknown operand (`bne` at 0x6c254). The
  recorded return carries `ret(0x6d51c)` (`ldmdb` at 0x6c70c). Discovery response-field semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6c200 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x6c204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6c208 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x6c20c | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x6c210 | load | 4 | `abs 0x6c710` | `-` |
| `ldr` at 0x6c218 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c21c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c238 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6c240 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6c244 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6c248 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x6c258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6c25c | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x6c260 | store | 4 | `sp -0x28` | `[[sp-24]+72]` |
| `ldr` at 0x6c268 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6c274 | store | 4 | `sp -0x28` | `ret(0xb7944)` |
| `ldr` at 0x6c278 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6c27c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6c280 | store | 4 | `[sp-36] +0x90` | `[sp-40]` |
| `ldr` at 0x6c284 | load | 4 | `abs 0x6c718` | `-` |
| `ldr` at 0x6c288 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6c2a0 | load | 4 | `abs 0x6c71c` | `-` |
| `ldr` at 0x6c2a4 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c2a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c2c4 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6c2cc | store | 4 | `sp -0x34` | `ret(0x1b1e4)` |
| `ldr` at 0x6c2d0 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6c2d4 | load | 4 | `abs 0x6c720` | `-` |
| `ldr` at 0x6c2d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c2dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c2e4 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c2e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c304 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c30c | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c32c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c334 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c338 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c344 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c348 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c364 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c36c | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c38c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c394 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6c3b0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x6c3b4 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c3b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c3d4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c3dc | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c3fc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c404 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c408 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c424 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c42c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c440 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6c450 | store | 4 | `(unknown)` | `ret(0x1b2c8)` |
| `ldr` at 0x6c454 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6c458 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x6c468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6c46c | load | 4 | `[sp-24] +0x50` | `-` |
| `str` at 0x6c470 | store | 4 | `sp -0x2c` | `[[sp-24]+80]` |
| `ldr` at 0x6c478 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6c484 | store | 4 | `sp -0x2c` | `ret(0xb7944)` |
| `ldr` at 0x6c488 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c48c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c4a8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c4b0 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c4b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c4d0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c4d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c4ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c4f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6c4fc | load | 4 | `abs 0x6c718` | `-` |
| `ldr` at 0x6c500 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6c510 | load | 4 | `abs 0x6c724` | `-` |
| `ldr` at 0x6c514 | load | 4 | `abs 0x142870` | `-` |
| `ldr` at 0x6c518 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c52c | load | 4 | `abs 0x6c71c` | `-` |
| `ldr` at 0x6c530 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c534 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c550 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c558 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6c55c | load | 4 | `abs 0x6c728` | `-` |
| `ldr` at 0x6c560 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c564 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6c570 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x6c574 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c594 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c59c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6c5a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c5b0 | load | 4 | `abs 0x6c71c` | `-` |
| `ldr` at 0x6c5b4 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c5b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c5d4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c5dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6c5f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c5f8 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c5fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c618 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c620 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6c634 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c63c | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c640 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c65c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6c678 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c680 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6c684 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x6c688 | load | 4 | `abs 0x6c714` | `-` |
| `ldr` at 0x6c68c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c6a8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c6b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6c6c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c6cc | load | 4 | `(unknown)` | `-` |
| `str` at 0x6c6d0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x6c6d4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6c6d8 | load | 4 | `abs 0x6c72c` | `-` |
| `ldr` at 0x6c6dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c6e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c6e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6c6f0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x6c6f8 | load | 4 | `abs 0x6c71c` | `-` |
| `ldr` at 0x6c6fc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6c700 | load | 4 | `abs 0x6c730` | `-` |
| `ldmdb` at 0x6c70c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6c70c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6c70c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6c70c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6c214 | `(unknown callee) 0x6c0d0` | `0x10f238` | `r1` | `r2` | `[r0+12]` |
| `bl` at 0x6c270 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x6c29c | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `[sp-36]` | `?` |
| `bl` at 0x6c2e0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f250` | `[sp-28]` | `?` |
| `bl` at 0x6c448 | `(unknown callee) 0x1b2c8` | `?` | `?` | `?` | `?` |
| `bl` at 0x6c480 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x6c4f8 | `(unknown callee) 0x1b304` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x6c568 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f288` | `[sp-28]` | `?` |
| `bl` at 0x6c6e4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f298` | `?` | `?` |
| `bl` at 0x6c704 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f2ac` | `[sp-32]` | `0x144dc8` |
| `bl` at 0x6c708 | `(unknown callee) 0x6d51c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6c254 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6c294 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x6c340 | `? <= ? (signed, 32-bit)` | `? > ? (signed, 32-bit)` |
| `bne` at 0x6c464 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6c50c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6c528 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6c5a8 | `[sp-32] < ? (signed, 32-bit)` | `[sp-32] >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6c70c | `r0` | `ret(0x6d51c)` |

## `L1CfgDiscoveryRequest`

- `L1CfgDiscoveryRequest`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c734 to 0x6cf78.
  Established. A word store clears `sp -0x78` (`str` at 0x6c74c). The first guard retains an unknown
  operand (`beq` at 0x6c780). The call to `0x24028` receives `[0x144dc8]`, `0x10f2b0`, `[sp-20]` and
  `[0x1489b8]` as candidates (`bl` at 0x6c7b4). Discovery-request encoding and final r0 value remain
  unknown (`ldmdb` at 0x6cf3c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6c744 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6c74c | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x6c750 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6c754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6c770 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6c778 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6c788 | load | 4 | `abs 0x6cf44` | `-` |
| `ldr` at 0x6c78c | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6c79c | load | 4 | `abs 0x6cf48` | `-` |
| `ldr` at 0x6c7a0 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6c7a4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6c7a8 | load | 4 | `abs 0x6cf4c` | `-` |
| `ldr` at 0x6c7ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6c7b0 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6c7b8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c7d4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c7d8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c7f4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c7f8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6c814 | store | 4 | `(unknown)` | `0x3e8` |
| `ldr` at 0x6c818 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6c834 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6c838 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6c840 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6c844 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c860 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c864 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c880 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c884 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6c89c | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6c8a0 | load | 4 | `abs 0x148934` | `-` |
| `str` at 0x6c8a4 | store | 4 | `(unknown)` | `[0x148934]` |
| `ldr` at 0x6c8a8 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6c8c4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6c8c8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6c8d0 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6c8d4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c8f0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c8f4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c910 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c914 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6c92c | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6c930 | load | 4 | `abs 0x148938` | `-` |
| `str` at 0x6c934 | store | 4 | `(unknown)` | `[0x148938]` |
| `ldr` at 0x6c938 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6c954 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6c958 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6c960 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6c964 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c980 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c984 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6c9a0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6c9a4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6c9bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6c9c0 | store | 4 | `(unknown)` | `[sp-20]` |
| `ldr` at 0x6c9c4 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6c9e0 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6c9e4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6c9ec | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6c9f0 | load | 4 | `abs 0x6cf50` | `-` |
| `ldr` at 0x6c9f8 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6c9fc | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6ca08 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6ca0c | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6ca1c | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6ca20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ca3c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ca44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ca50 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6ca54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ca70 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ca78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ca88 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6caa4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6caa8 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6cac4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6cac8 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6cae4 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x6cae8 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6cb04 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6cb08 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6cb10 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `str` at 0x6cb20 | store | 4 | `sp -0x18` | `ret(0x1b124)` |
| `ldr` at 0x6cb24 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cb28 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cb2c | load | 4 | `abs 0x148934` | `-` |
| `ldr` at 0x6cb30 | load | 4 | `abs 0x148938` | `-` |
| `str` at 0x6cb3c | store | 4 | `sp -0x7c` | `ret(0x748c4)` |
| `ldr` at 0x6cb40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6cb44 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6cb50 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6cb6c | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x6cb70 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6cb8c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6cb90 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6cba8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6cbac | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x6cbb0 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6cbcc | store | 2 | `(unknown)` | `0x18` |
| `ldr` at 0x6cbd0 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6cbd8 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6cbe0 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6cbfc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6cc00 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6cc1c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6cc20 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6cc38 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cc3c | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x6cc40 | store | 4 | `(unknown)` | `[0x1489b8]` |
| `ldr` at 0x6cc44 | load | 4 | `sp -0x78` | `-` |
| `strh` at 0x6cc60 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x6cc64 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6cc6c | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6cc70 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cc74 | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x6cc94 | store | 4 | `sp -0x18` | `ret(0x1b124)` |
| `ldr` at 0x6cc98 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cc9c | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cca0 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6ccb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6ccb8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6ccc4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6cce0 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x6cce4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x6cd00 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6cd04 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6cd1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6cd20 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x6cd24 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6cd3c | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cd40 | load | 4 | `abs 0x1489b8` | `-` |
| `strh` at 0x6cd54 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x6cd58 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6cd60 | store | 4 | `sp -0x78` | `[sp-120]+1` |
| `ldr` at 0x6cd68 | load | 4 | `abs 0x6cf54` | `-` |
| `str` at 0x6cd6c | store | 4 | `sp -0x1c` | `0xffff00e` |
| `ldr` at 0x6cd70 | load | 4 | `abs 0x6cf44` | `-` |
| `ldr` at 0x6cd74 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6cd84 | load | 4 | `abs 0x6cf58` | `-` |
| `ldr` at 0x6cd88 | load | 4 | `abs 0x142870` | `-` |
| `ldr` at 0x6cd8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cda0 | load | 4 | `abs 0x6cf48` | `-` |
| `ldr` at 0x6cda4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6cda8 | load | 4 | `abs 0x6cf5c` | `-` |
| `ldr` at 0x6cdac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6cdb8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6cdbc | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cdc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6cdc4 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6cdd4 | load | 4 | `abs 0x6cf48` | `-` |
| `ldr` at 0x6cdd8 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cddc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6cdf0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6cdf8 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6cdfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ce10 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6ce18 | load | 4 | `abs 0x6cf40` | `-` |
| `ldr` at 0x6ce1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ce30 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6ce38 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6ce3c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x6ce40 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6ce44 | load | 4 | `abs 0x6cf60` | `-` |
| `ldr` at 0x6ce48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ce4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ce54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6ce5c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x6ce64 | load | 4 | `abs 0x6cf48` | `-` |
| `ldr` at 0x6ce68 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6ce6c | load | 4 | `abs 0x6cf64` | `-` |
| `str` at 0x6ce7c | store | 4 | `sp -0x88` | `0x2` |
| `ldr` at 0x6ce80 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x6ce84 | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0x6ce8c | store | 4 | `sp -0x80` | `sp-116` |
| `ldr` at 0x6ce90 | load | 4 | `abs 0x6cf68` | `-` |
| `str` at 0x6cea8 | store | 4 | `sp -0x20` | `ret(0xb6f64)` |
| `ldr` at 0x6ceac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6ceb8 | load | 4 | `abs 0x6cf48` | `-` |
| `ldr` at 0x6cebc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6cec0 | load | 4 | `abs 0x6cf6c` | `-` |
| `ldr` at 0x6cecc | load | 4 | `abs 0x6cf70` | `-` |
| `ldr` at 0x6ced0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cef0 | load | 4 | `abs 0x6cf70` | `-` |
| `ldr` at 0x6cef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cf08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cf14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6cf28 | load | 4 | `abs 0x6cf74` | `-` |
| `ldr` at 0x6cf2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cf34 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x6cf3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6cf3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6cf3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6c7b4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f2b0` | `[sp-20]` | `[0x1489b8]` |
| `bl` at 0x6c9f4 | `(unknown callee) 0x6c0d0` | `0x10f2e0` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x6cb18 | `(unknown callee) 0x1b124` | `0x18` | `sp-16` | `?` | `[sp-120]+1` |
| `bl` at 0x6cb34 | `(unknown callee) 0x748c4` | `[0x148934]` | `[0x148938]` | `0x14892c` | `0x14892c` |
| `bl` at 0x6cb4c | `(unknown callee) 0x1b304` | `[sp-24]` | `ret(0x748c4)` | `0x18` | `ret(0x748c4)` |
| `bl` at 0x6cc8c | `(unknown callee) 0x1b124` | `?` | `sp-16` | `[0x1489b8]` | `?` |
| `bl` at 0x6ccc0 | `(unknown callee) 0x1b304` | `ret(0x1b124)` | `[0x1489c0]` | `?` | `?` |
| `bl` at 0x6cd64 | `(unknown callee) 0x6c168` | `ret(0x1b304)` | `?` | `?` | `[sp-120]+1` |
| `bl` at 0x6cdb0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f2f8` | `[sp-20]` | `0x144dc8` |
| `bl` at 0x6ce50 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f304` | `?` | `?` |
| `bl` at 0x6ce70 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f2ac` | `?` | `0x144dc8` |
| `bl` at 0x6cea0 | `(unknown callee) 0xb6f64` | `0x148924` | `sp-28` | `0x0` | `0x0` |
| `bl` at 0x6cec4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f314` | `?` | `0x144dc8` |
| `bl` at 0x6cf18 | `(unknown callee) 0xb686c` | `?` | `ret(0xb6f64)` | `0x50` | `?` |
| `bl` at 0x6cf30 | `(unknown callee) 0x2070c` | `0x10f348` | `[sp-20]` | `?` | `ret(0xb686c)` |
| `bl` at 0x6cf38 | `(unknown callee) 0xb56f4` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6c780 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6c798 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6ca04 | `[0x14893c] == 0x52` | `[0x14893c] != 0x52` |
| `beq` at 0x6ca14 | `[0x14893c] == 0x44` | `[0x14893c] != 0x44` |
| `beq` at 0x6ca4c | `? == 0x52` | `? != 0x52` |
| `beq` at 0x6ca80 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x6cd80 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6cd9c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6cdcc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6ceb4 | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |
| `beq` at 0x6ceec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6cf24 | `ret(0xb686c) == 0x0` | `ret(0xb686c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6cf3c | `r0` | `?` |

## `ARG__START`

- `ARG__START`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6907c to 0x690d8.
  Established. The first byte store clears `r1 +0x0` (`strb` at 0x690a0). Entry r0 is
  retained at the recorded return (`ldmdb` at 0x690d4). The argument record's protocol meaning
  remains unknown beyond the access table.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6908c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69090 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x69094 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x69098 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x690a0 | store | 1 | `r1 +0x0` | `0x0` |
| `ldr` at 0x690a4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x690ac | store | 1 | `[sp-24] +0x1` | `0x0` |
| `ldr` at 0x690b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x690b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x690b8 | store | 4 | `[sp-24] +0x4` | `[sp-28]` |
| `ldr` at 0x690bc | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x690c4 | store | 2 | `[sp-24] +0x2` | `0x0` |
| `ldr` at 0x690c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x690d0 | store | 4 | `[sp-20] +0x0` | `0x1` |
| `ldmdb` at 0x690d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x690d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x690d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x690d4 | `r0` | `r0` |

## `ARG__IMMEDIATE`

- `ARG__IMMEDIATE`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x690d8 to 0x6917c.
  Established. A byte store clears an unknown effective target (`strb` at 0x6910c). The exit
  preserves entry r0 (`ldmdb` at 0x69178). The immediate-argument encoding and calculated record
  geometry remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x690e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x690ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x690f0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x690f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x690f8 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x69100 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x6910c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x69110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69114 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x6911c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x69128 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6912c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69130 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x69138 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69140 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x69144 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x69148 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6914c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x69154 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x69160 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x69164 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69168 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6916c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x69174 | store | 4 | `[sp-20] +0x0` | `[[sp-20]]+1` |
| `ldmdb` at 0x69178 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x69178 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x69178 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x69178 | `r0` | `r0` |

## `ARG__INDIRECT`

- `ARG__INDIRECT`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6917c to 0x69250.
  Established. The call to `0x1b124` carries entry r3, entry r1, entry r2 and entry r3 as candidates
  (`bl` at 0x691a0); its zero result takes the branch and nonzero falls through (`beq` at 0x691b4).
  The final r0 value is unknown (`ldmdb` at 0x6924c). The indirect-argument ownership contract
  remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6918c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69190 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x69194 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x69198 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x6919c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x691a8 | store | 4 | `sp -0x24` | `ret(0x1b124)` |
| `ldr` at 0x691ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x691b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x691bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x691c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x691c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x691cc | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x691d4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x691e0 | store | 1 | `(unknown)` | `0x9` |
| `ldr` at 0x691e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x691e8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x691f0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x691fc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x69200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69204 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x6920c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69214 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x69218 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x6921c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69220 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x69228 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x69230 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x69234 | store | 2 | `(unknown)` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0x69238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6923c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69240 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x69248 | store | 4 | `[sp-20] +0x0` | `[[sp-20]]+1` |
| `ldmdb` at 0x6924c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6924c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6924c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x691a0 | `(unknown callee) 0x1b124` | `r3` | `r1` | `r2` | `r3` |
| `bl` at 0x691c4 | `(unknown callee) 0x1b304` | `ret(0x1b124)` | `[sp-28]` | `[sp-32]` | `ret(0x1b124)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x691b4 | `ret(0x1b124) == 0x0` | `ret(0x1b124) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6924c | `r0` | `?` |

## `ARG__INDIRECT_NOFREE`

- `ARG__INDIRECT_NOFREE`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69250 to 0x692f8.
  Established. Four word stores save entry r0 through r3 at `sp -0x14`, `sp -0x18`, `sp -0x1c` and
  `sp -0x20` (`str` at 0x69260, `str` at 0x69264, `str` at 0x69268, `str` at 0x6926c). The recorded
  return preserves entry r0 (`ldmdb` at 0x692f4). The argument ownership and field encodings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69260 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69264 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x69268 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x6926c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x69270 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69274 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x6927c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x69288 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x6928c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69290 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x69298 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x692a4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x692a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x692ac | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x692b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x692bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x692c0 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x692c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x692c8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x692d0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x692d8 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x692dc | store | 2 | `(unknown)` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0x692e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x692e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x692e8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x692f0 | store | 4 | `[sp-20] +0x0` | `[[sp-20]]+1` |
| `ldmdb` at 0x692f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x692f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x692f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x692f4 | `r0` | `r0` |

## `ARG__STRING`

- `ARG__STRING`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x692f8 to 0x69418.
  Established. The first branch takes `[sp] == 0x0` and falls through on nonzero (`beq` at 0x69320).
  The call to `0x1b124` receives `r3+1`, entry r1, entry r2 and `r3+1` as candidates (`bl` at
  0x69330). The final r0 expression is unknown (`ldmdb` at 0x69414). String ownership and length
  semantics remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69308 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x6930c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x69310 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x69314 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x69318 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x69324 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x69338 | store | 4 | `sp -0x28` | `ret(0x1b124)` |
| `ldr` at 0x6933c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x69348 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6934c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69350 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x69358 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6935c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x69368 | store | 1 | `([sp-40] + [sp-36]) +0x0` | `0x0` |
| `ldr` at 0x69370 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69374 | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0x69378 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x69384 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69390 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x6939c | store | 1 | `(unknown)` | `0xa` |
| `ldr` at 0x693a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x693a4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x693ac | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x693b8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x693bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x693c0 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x693c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x693d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x693d4 | store | 4 | `(unknown)` | `[sp-40]` |
| `ldr` at 0x693d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x693dc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x693e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x693ec | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x693fc | store | 2 | `(unknown)` | `(ret(0x1b22c)+1 & 0xffff)` |
| `ldr` at 0x69400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69404 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69408 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x69410 | store | 4 | `[sp-24] +0x0` | `[[sp-24]]+1` |
| `ldmdb` at 0x69414 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x69414 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x69414 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x69414 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69330 | `(unknown callee) 0x1b124` | `r3+1` | `r1` | `r2` | `r3+1` |
| `bl` at 0x69354 | `(unknown callee) 0x1b454` | `ret(0x1b124)` | `[sp-32]` | `[sp-36]` | `ret(0x1b124)` |
| `bl` at 0x693f0 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x69320 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0x69344 | `ret(0x1b124) == 0x0` | `ret(0x1b124) != 0x0` |
| `beq` at 0x69380 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x69414 | `r0` | `?` |

## `ARG__USERDEF`

- `ARG__USERDEF`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69418 to 0x694c4.
  Established. The opening word stores save entry r0 through r3 at `sp -0x14`, `sp -0x18`, `sp
  -0x1c` and `sp -0x20` (`str` at 0x69428, `str` at 0x6942c, `str` at 0x69430, `str` at 0x69434).
  The exit retains entry r0 (`ldmdb` at 0x694c0). User-defined argument encoding remains unknown
  beyond the explicit operands.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69428 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6942c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x69430 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x69434 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x69438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6943c | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x69444 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x6944c | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x69454 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x69458 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6945c | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x69464 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x69470 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x69474 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69478 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x69480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69488 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6948c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x69490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69494 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x6949c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x694a4 | load | 2 | `sp +0x0` | `-` |
| `strh` at 0x694a8 | store | 2 | `(unknown)` | `([sp]:2 & 0xffff)` |
| `ldr` at 0x694ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x694b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x694b4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x694bc | store | 4 | `[sp-24] +0x0` | `[[sp-24]]+1` |
| `ldmdb` at 0x694c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x694c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x694c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x694c0 | `r0` | `r0` |

## `IRPktSize`

- `IRPktSize`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb837c to 0xb83d8.
  Established. A word store clears `sp -0x18` (`str` at 0xb8394). The loop branch takes `Z == 0` and
  falls through on `not (Z == 0)` (`bne` at 0xb83a0). The return expression remains unknown (`ldmdb`
  at 0xb83d4). The packet-size interpretation and list geometry are unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb838c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8394 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb8398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb83a8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb83ac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb83b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb83b8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb83bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb83c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb83c4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xb83cc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb83d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb83d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb83d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb83a0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb83d4 | `r0` | `?` |
