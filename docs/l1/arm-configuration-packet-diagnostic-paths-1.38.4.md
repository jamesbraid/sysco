# ARM configuration, packet and diagnostic paths in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
The CramFS begins at package offset `0xb8c48` and contains 3035136 bytes.
CramFS SHA-256: `d701825754e87751a85b25cbcff25dba772be94107aedfb4cce9c5ef15c8c1ae`.
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
L1CfgInit, L1CfgQuery, B2BInit and B2BReqRsp have unresolved computed selectors. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/configuration-packet-diagnostic-paths-facts.txt) retain all tool sections.

## `L1CfgInit`

- `L1CfgInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a8f4 to 0x6b1e0.
  Established. A word at absolute `0x148650` is read and incremented through the stored expression
  `[0x148650]+1` (`ldr` at 0x6a948, `str` at 0x6a950). A later store places `ret(0x1b1e4)` at
  absolute `0x148668` (`str` at 0x6a960). The branch is taken when `[0x148650] == 0x0` and falls
  through for a nonzero value (`beq` at 0x6a920). Configuration roles beyond these state changes are
  unknown, and the computed selector limits coverage to known edges (`ldrls` at 0x6ad80).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a910 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6a914 | load | 4 | `abs 0x6b178` | `-` |
| `ldr` at 0x6a918 | load | 4 | `abs 0x148650` | `-` |
| `ldr` at 0x6a924 | load | 4 | `abs 0x6b17c` | `-` |
| `str` at 0x6a928 | store | 4 | `sp -0x40` | `0x10f0b8` |
| `ldr` at 0x6a930 | load | 4 | `abs 0x6b180` | `-` |
| `ldr` at 0x6a940 | load | 4 | `abs 0x6b178` | `-` |
| `ldr` at 0x6a944 | load | 4 | `abs 0x6b178` | `-` |
| `ldr` at 0x6a948 | load | 4 | `abs 0x148650` | `-` |
| `str` at 0x6a950 | store | 4 | `abs 0x148650` | `[0x148650]+1` |
| `ldr` at 0x6a954 | load | 4 | `abs 0x6b184` | `-` |
| `str` at 0x6a960 | store | 4 | `abs 0x148668` | `ret(0x1b1e4)` |
| `str` at 0x6a97c | store | 4 | `sp -0x20` | `ret(0x1b22c)` |
| `ldr` at 0x6a980 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6a9ac | load | 4 | `abs 0x6b188` | `-` |
| `ldr` at 0x6a9b8 | load | 4 | `abs 0x6b18c` | `-` |
| `str` at 0x6a9c0 | store | 4 | `abs 0x148654` | `0x100` |
| `ldr` at 0x6a9c4 | load | 4 | `abs 0x6b18c` | `-` |
| `str` at 0x6a9cc | store | 4 | `abs 0x148654` | `0x1` |
| `ldr` at 0x6a9d0 | load | 4 | `abs 0x6b18c` | `-` |
| `str` at 0x6a9d8 | store | 4 | `abs 0x148654` | `0x10` |
| `ldr` at 0x6a9dc | load | 4 | `abs 0x6b18c` | `-` |
| `ldr` at 0x6a9e0 | load | 4 | `abs 0x6b190` | `-` |
| `str` at 0x6a9e4 | store | 4 | `abs 0x148654` | `0x3ff` |
| `ldr` at 0x6a9e8 | load | 4 | `abs 0x6b18c` | `-` |
| `str` at 0x6a9f0 | store | 4 | `abs 0x148654` | `0x0` |
| `ldr` at 0x6a9f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6a9f8 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x6a9fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6aa08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6aa14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6aa24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6aa34 | load | 4 | `abs 0x6b194` | `-` |
| `str` at 0x6aa3c | store | 4 | `abs 0x14286c` | `0xc` |
| `ldr` at 0x6aa40 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6aa48 | store | 4 | `abs 0x1489bc` | `0x10` |
| `ldr` at 0x6aa4c | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6aa54 | store | 4 | `abs 0x1489b8` | `0x0` |
| `ldr` at 0x6aa58 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aa5c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aa60 | load | 4 | `abs 0x1489bc` | `-` |
| `str` at 0x6aa80 | store | 4 | `abs 0x1489c0` | `ret(0x1b124)` |
| `ldr` at 0x6aa84 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aa88 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aa8c | load | 4 | `abs 0x1489bc` | `-` |
| `ldr` at 0x6aaa0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6aab0 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6aab8 | store | 4 | `abs 0x1489b0` | `0xd` |
| `ldr` at 0x6aabc | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aac0 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aac4 | load | 4 | `abs 0x1489b0` | `-` |
| `str` at 0x6aaec | store | 4 | `abs 0x1489b4` | `ret(0x1b124)` |
| `ldr` at 0x6aaf0 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aaf4 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aaf8 | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x6ab14 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ab24 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ab2c | store | 4 | `abs 0x14892c` | `0x1` |
| `ldr` at 0x6ab30 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ab38 | store | 4 | `abs 0x14898c` | `0x2` |
| `ldr` at 0x6ab3c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6ab40 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6ab44 | store | 4 | `abs 0x14893c` | `[sp-36]` |
| `ldr` at 0x6ab48 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ab54 | store | 4 | `abs 0x148934` | `ret(0x1f1dc)` |
| `ldr` at 0x6ab58 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ab64 | store | 4 | `abs 0x148938` | `ret(0x1f1dc)` |
| `ldr` at 0x6ab68 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ab74 | store | 4 | `abs 0x148954` | `ret(0x1f1dc)` |
| `ldr` at 0x6ab78 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6ab7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6ab84 | store | 4 | `abs 0x14899c` | `ret(0x74b98)` |
| `ldr` at 0x6ab88 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ab94 | store | 4 | `abs 0x148968` | `ret(0x1f1dc)` |
| `ldr` at 0x6ab98 | load | 4 | `abs 0x148968` | `-` |
| `ldr` at 0x6aba4 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6abac | store | 4 | `abs 0x148968` | `0x0` |
| `ldr` at 0x6abc8 | load | 4 | `abs 0x6b19c` | `-` |
| `ldr` at 0x6abd4 | load | 4 | `abs 0x6b1a0` | `-` |
| `ldr` at 0x6abd8 | load | 4 | `abs 0x6b188` | `-` |
| `ldr` at 0x6abe0 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6abec | store | 4 | `abs 0x148940` | `ret(0x5ea88)` |
| `ldr` at 0x6abf0 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6abf8 | store | 4 | `abs 0x148944` | `0x0` |
| `ldr` at 0x6abfc | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ac04 | store | 4 | `abs 0x148958` | `0x0` |
| `ldr` at 0x6ac08 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ac10 | store | 4 | `abs 0x148960` | `ret(0x3d8e0)` |
| `ldr` at 0x6ac14 | load | 4 | `abs 0x6b198` | `-` |
| `str` at 0x6ac1c | store | 4 | `abs 0x148964` | `ret(0x3d8c0)` |
| `ldr` at 0x6ac20 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac28 | store | 4 | `abs 0x1489d8` | `0x1` |
| `ldr` at 0x6ac2c | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac34 | store | 4 | `abs 0x148a04` | `0x2` |
| `ldr` at 0x6ac38 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac40 | store | 4 | `abs 0x148a20` | `0x3` |
| `ldr` at 0x6ac44 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac4c | store | 4 | `abs 0x148a24` | `0x4` |
| `ldr` at 0x6ac50 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac58 | store | 4 | `abs 0x148a28` | `0x5` |
| `ldr` at 0x6ac5c | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac64 | store | 4 | `abs 0x148a2c` | `0x6` |
| `ldr` at 0x6ac68 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac70 | store | 4 | `abs 0x148a30` | `0x7` |
| `ldr` at 0x6ac74 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac7c | store | 4 | `abs 0x148a34` | `0x8` |
| `ldr` at 0x6ac80 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac88 | store | 4 | `abs 0x1489d4` | `0x9` |
| `ldr` at 0x6ac8c | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6ac94 | store | 4 | `abs 0x1489dc` | `0xa` |
| `ldr` at 0x6ac98 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6aca0 | store | 4 | `abs 0x148a08` | `0xb` |
| `ldr` at 0x6aca4 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6acac | store | 4 | `abs 0x148a0c` | `0xc` |
| `ldr` at 0x6acb0 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6acb8 | store | 4 | `abs 0x1489e0` | `0x9` |
| `ldr` at 0x6acbc | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6acc4 | store | 4 | `abs 0x1489e4` | `0xa` |
| `ldr` at 0x6acc8 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6acd0 | store | 4 | `abs 0x148a00` | `0xb` |
| `ldr` at 0x6acd4 | load | 4 | `abs 0x6b1a4` | `-` |
| `str` at 0x6acdc | store | 4 | `abs 0x1489f4` | `0xc` |
| `ldr` at 0x6ace0 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ace8 | store | 4 | `abs 0x148ad4` | `0x2` |
| `ldr` at 0x6acec | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6acf4 | store | 4 | `abs 0x148ad8` | `0xd` |
| `ldr` at 0x6acf8 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad00 | store | 4 | `abs 0x148adc` | `0x14` |
| `ldr` at 0x6ad04 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad0c | store | 4 | `abs 0x148ae0` | `0x15` |
| `ldr` at 0x6ad10 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad18 | store | 4 | `abs 0x148ae4` | `0x16` |
| `ldr` at 0x6ad1c | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad24 | store | 4 | `abs 0x148ae8` | `0x17` |
| `ldr` at 0x6ad28 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad30 | store | 4 | `abs 0x148aec` | `0x18` |
| `ldr` at 0x6ad34 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad3c | store | 4 | `abs 0x148af0` | `0x19` |
| `ldr` at 0x6ad40 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad48 | store | 4 | `abs 0x148af4` | `0x1` |
| `ldr` at 0x6ad4c | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad54 | store | 4 | `abs 0x148af8` | `0x3` |
| `ldr` at 0x6ad58 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad60 | store | 4 | `abs 0x148afc` | `0xe` |
| `ldr` at 0x6ad64 | load | 4 | `abs 0x6b1a8` | `-` |
| `str` at 0x6ad6c | store | 4 | `abs 0x148b00` | `0xf` |
| `ldr` at 0x6ad70 | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6ad74 | load | 4 | `abs 0x14893c` | `-` |
| `ldrls` at 0x6ad80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6af58 | load | 4 | `abs 0x6b1ac` | `-` |
| `ldr` at 0x6af5c | load | 4 | `abs 0x6b1b0` | `-` |
| `ldr` at 0x6af6c | load | 4 | `abs 0x6b1b4` | `-` |
| `ldr` at 0x6af7c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6af80 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6af8c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6af90 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6af9c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6afa0 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6afac | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6afb0 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6afbc | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6afc0 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6afcc | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6afd0 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6afdc | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6afe0 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6afec | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6aff0 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6affc | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6b000 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6b00c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6b010 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6b01c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6b020 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6b02c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6b030 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6b03c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6b040 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6b04c | load | 4 | `abs 0x6b198` | `-` |
| `ldr` at 0x6b050 | load | 4 | `abs 0x14893c` | `-` |
| `str` at 0x6b064 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x6b06c | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x6b074 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x6b07c | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x6b084 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x6b088 | load | 4 | `abs 0x6b1b8` | `-` |
| `ldr` at 0x6b08c | load | 4 | `abs 0x6b1bc` | `-` |
| `ldr` at 0x6b090 | load | 4 | `abs 0x6b1c0` | `-` |
| `str` at 0x6b0a4 | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0x6b0ac | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x6b0b4 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x6b0bc | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x6b0c4 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x6b0c8 | load | 4 | `abs 0x6b1b8` | `-` |
| `ldr` at 0x6b0cc | load | 4 | `abs 0x6b1bc` | `-` |
| `ldr` at 0x6b0d0 | load | 4 | `abs 0x6b1c4` | `-` |
| `ldr` at 0x6b0dc | load | 4 | `abs 0x6b1b4` | `-` |
| `ldr` at 0x6b0e0 | load | 4 | `abs 0x6b1c8` | `-` |
| `ldr` at 0x6b0e8 | load | 4 | `abs 0x6b1cc` | `-` |
| `str` at 0x6b0f4 | store | 4 | `sp -0x1c` | `ret(0xb4178)` |
| `ldr` at 0x6b0f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6b104 | load | 4 | `abs 0x6b1d0` | `-` |
| `str` at 0x6b108 | store | 4 | `sp -0x40` | `0x10f104` |
| `ldr` at 0x6b10c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6b110 | store | 4 | `sp -0x3c` | `ret(0xb4178)` |
| `ldr` at 0x6b118 | load | 4 | `abs 0x6b180` | `-` |
| `ldr` at 0x6b11c | load | 4 | `abs 0x6b1d4` | `-` |
| `ldr` at 0x6b128 | load | 4 | `abs 0x6b1b4` | `-` |
| `ldr` at 0x6b134 | load | 4 | `abs 0x6b1cc` | `-` |
| `str` at 0x6b140 | store | 4 | `sp -0x1c` | `ret(0xb4604)` |
| `ldr` at 0x6b144 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6b150 | load | 4 | `abs 0x6b1d8` | `-` |
| `str` at 0x6b154 | store | 4 | `sp -0x40` | `0x10f130` |
| `ldr` at 0x6b158 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6b15c | store | 4 | `sp -0x3c` | `ret(0xb4604)` |
| `ldr` at 0x6b164 | load | 4 | `abs 0x6b180` | `-` |
| `ldr` at 0x6b168 | load | 4 | `abs 0x6b1dc` | `-` |
| `ldmdb` at 0x6b174 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6b174 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b174 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b174 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a904 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x6a93c | `(unknown callee) 0x21e28` | `0x9` | `0x10f0d4` | `0x6d` | `0x0` |
| `bl` at 0x6a95c | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `0x148650` | `[0x148650]+1` |
| `bl` at 0x6a968 | `(unknown callee) 0x1f1dc` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x6a974 | `(unknown callee) 0x1b22c` | `ret(0x1f1dc)` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x6a98c | `(unknown callee) 0x4f2f0` | `ret(0x1b22c)` | `?` | `?` | `ret(0x1b22c)` |
| `bl` at 0x6a99c | `(unknown callee) 0x1d38c` | `0xf` | `ret(0x4f2f0)` | `?` | `ret(0x4f2f0)` |
| `bl` at 0x6a9a4 | `(unknown callee) 0x1f1dc` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x6a9b4 | `(unknown callee) 0x1b508` | `0x2b1b48` | `ret(0x1f1dc)` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x6aa78 | `(unknown callee) 0x1b124` | `?` | `?` | `[0x1489bc]` | `?` |
| `bl` at 0x6aaac | `(unknown callee) 0x1b43c` | `[0x1489c0]` | `0x0` | `?` | `?` |
| `bl` at 0x6aae4 | `(unknown callee) 0x1b124` | `?` | `?` | `[0x1489b0]` | `?` |
| `bl` at 0x6ab20 | `(unknown callee) 0x1b43c` | `[0x1489b4]` | `0x0` | `?` | `?` |
| `bl` at 0x6ab50 | `(unknown callee) 0x1f1dc` | `0x1` | `?` | `0x14892c` | `[sp-36]` |
| `bl` at 0x6ab60 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6ab70 | `(unknown callee) 0x1f1dc` | `0x98` | `?` | `?` | `?` |
| `bl` at 0x6ab80 | `(unknown callee) 0x74b98` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x6ab90 | `(unknown callee) 0x1f1dc` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x6abb8 | `(unknown callee) 0x1d38c` | `0xe` | `0x0` | `0x14892c` | `0x0` |
| `bl` at 0x6abc0 | `(unknown callee) 0x1f1dc` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x6abd0 | `(unknown callee) 0x1b508` | `0x14896c` | `ret(0x1f1dc)` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x6abdc | `(unknown callee) 0x1b508` | `0x148978` | `0x2b1b48` | `?` | `?` |
| `bl` at 0x6abe8 | `(unknown callee) 0x5ea88` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6ac0c | `(unknown callee) 0x3d8e0` | `ret(0x5ea88)` | `?` | `0x14892c` | `0x0` |
| `bl` at 0x6ac18 | `(unknown callee) 0x3d8c0` | `ret(0x3d8e0)` | `?` | `?` | `?` |
| `bl` at 0x6af54 | `(unknown callee) 0x6b1e0` | `ret(0x3d8c0)` | `?` | `0x148ad0` | `[0x14893c]-48` |
| `bl` at 0x6af68 | `(unknown callee) 0x22920` | `0x1488cc` | `0x10f0ec` | `0x1` | `0x9` |
| `bl` at 0x6af78 | `(unknown callee) 0xb553c` | `0x148924` | `0xe` | `0x0` | `?` |
| `bl` at 0x6b098 | `(unknown callee) 0x223f0` | `0x148920` | `0x10f0f8` | `0x6b538` | `0x0` |
| `bl` at 0x6b0d8 | `(unknown callee) 0x223f0` | `0x148920` | `0x10f0f8` | `0x6b320` | `0x0` |
| `bl` at 0x6b0ec | `(unknown callee) 0xb4178` | `0x148924` | `0xffff00e` | `0x3` | `0x729ec` |
| `bl` at 0x6b124 | `(unknown callee) 0x21e28` | `0x9` | `0x10f0d4` | `0x129` | `0x0` |
| `bl` at 0x6b138 | `(unknown callee) 0xb4604` | `0x148924` | `?` | `0x0` | `0x729ec` |
| `bl` at 0x6b170 | `(unknown callee) 0x21e28` | `0x9` | `0x10f0d4` | `0x12e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a920 | `[0x148650] == 0x0` | `[0x148650] != 0x0` |
| `beq` at 0x6a988 | `ret(0x1b22c) == 0x6` | `ret(0x1b22c) != 0x6` |
| `beq` at 0x6aa04 | `[sp-36] == 0x46` | `[sp-36] != 0x46` |
| `bgt` at 0x6aa10 | `[sp-36] > 0x46 (signed, 32-bit)` | `[sp-36] <= 0x46 (signed, 32-bit)` |
| `beq` at 0x6aa1c | `[sp-36] == 0x42` | `[sp-36] != 0x42` |
| `beq` at 0x6aa2c | `[sp-36] == 0x48` | `[sp-36] != 0x48` |
| `ble` at 0x6aba0 | `[0x148968] <= 0xfe (signed, 32-bit)` | `[0x148968] > 0xfe (signed, 32-bit)` |
| `ldrls` at 0x6ad80 | `[0x14893c]-48 <= 0x4c (unsigned, 32-bit)` | `[0x14893c]-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0x6af88 | `[0x14893c] == 0x35` | `[0x14893c] != 0x35` |
| `beq` at 0x6af98 | `[0x14893c] == 0x4f` | `[0x14893c] != 0x4f` |
| `beq` at 0x6afa8 | `[0x14893c] == 0x42` | `[0x14893c] != 0x42` |
| `beq` at 0x6afb8 | `[0x14893c] == 0x46` | `[0x14893c] != 0x46` |
| `beq` at 0x6afc8 | `[0x14893c] == 0x48` | `[0x14893c] != 0x48` |
| `beq` at 0x6afd8 | `[0x14893c] == 0x4a` | `[0x14893c] != 0x4a` |
| `beq` at 0x6afe8 | `[0x14893c] == 0x4b` | `[0x14893c] != 0x4b` |
| `beq` at 0x6aff8 | `[0x14893c] == 0x21` | `[0x14893c] != 0x21` |
| `beq` at 0x6b008 | `[0x14893c] == 0x5e` | `[0x14893c] != 0x5e` |
| `beq` at 0x6b018 | `[0x14893c] == 0x2b` | `[0x14893c] != 0x2b` |
| `beq` at 0x6b028 | `[0x14893c] == 0x4c` | `[0x14893c] != 0x4c` |
| `beq` at 0x6b038 | `[0x14893c] == 0x5a` | `[0x14893c] != 0x5a` |
| `beq` at 0x6b048 | `[0x14893c] == 0x7c` | `[0x14893c] != 0x7c` |
| `beq` at 0x6b058 | `[0x14893c] == 0x4d` | `[0x14893c] != 0x4d` |
| `beq` at 0x6b100 | `ret(0xb4178) == 0x0` | `ret(0xb4178) != 0x0` |
| `beq` at 0x6b14c | `ret(0xb4604) == 0x0` | `ret(0xb4604) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b174 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6ad80).

## `L1CfgRingTask`

- `L1CfgRingTask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b538 to 0x6c0d0.
  Established. The opening call targets `0x76cdc` with candidate r0 `0x1f4` and the other candidates
  still at entry r1 through r3 (`bl` at 0x6b554). A word store clears absolute `0x148680` (`str` at
  0x6b598). The comparison takes its branch for signed `ret(0x1b1e4) >= [0x148680]` and
  falls through for less (`bge` at 0x6b580). The recorded exit carries `ret(0xb6f64)` (`ldmdb` at
  0x6c094). Ring-state and timing units are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b548 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6b54c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6b558 | load | 4 | `abs 0x6c098` | `-` |
| `ldr` at 0x6b55c | load | 4 | `abs 0x148680` | `-` |
| `ldr` at 0x6b574 | load | 4 | `abs 0x6c098` | `-` |
| `ldr` at 0x6b578 | load | 4 | `abs 0x148680` | `-` |
| `ldr` at 0x6b588 | load | 4 | `abs 0x6c09c` | `-` |
| `ldr` at 0x6b590 | load | 4 | `abs 0x6c098` | `-` |
| `str` at 0x6b598 | store | 4 | `abs 0x148680` | `0x0` |
| `ldr` at 0x6b59c | load | 4 | `abs 0x6c0a0` | `-` |
| `ldr` at 0x6b5a0 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6b5b0 | load | 4 | `abs 0x6c0a4` | `-` |
| `ldr` at 0x6b5b4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6b5b8 | load | 4 | `abs 0x6c0a8` | `-` |
| `ldr` at 0x6b5c4 | load | 4 | `abs 0x6c0ac` | `-` |
| `ldr` at 0x6b5c8 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6b5d0 | store | 4 | `sp -0x44` | `[0x1489b4]+156` |
| `ldr` at 0x6b5d4 | load | 4 | `abs 0x6c0ac` | `-` |
| `ldr` at 0x6b5d8 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6b5e0 | store | 4 | `sp -0x48` | `[0x1489b4]+312` |
| `ldr` at 0x6b60c | load | 4 | `abs 0x6c0b0` | `-` |
| `str` at 0x6b614 | store | 4 | `abs 0x148658` | `?` |
| `ldr` at 0x6b630 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b634 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6b640 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b644 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b650 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b654 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b660 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b664 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b670 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b674 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b680 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b684 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b690 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b694 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b6a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b6a4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b6b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b6b4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b6c0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b6c4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b6d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b6d4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b6e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b6e4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b6f0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b6f4 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b700 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b704 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b710 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b714 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b734 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b738 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6b744 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b748 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b754 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b758 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b764 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b768 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b774 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b778 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b784 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b788 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b794 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b798 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b7a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b7a8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b7b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b7b8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b7c4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b7c8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b7d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b7d8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b7e4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b7e8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b7f4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b7f8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b804 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b808 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b814 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b818 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6b828 | load | 4 | `abs 0x6c0ac` | `-` |
| `ldr` at 0x6b82c | load | 4 | `abs 0x14893c` | `-` |
| `str` at 0x6b864 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x6b868 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b86c | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6b878 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b87c | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x6b88c | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6b894 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b898 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6b8a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b8a8 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x6b8b8 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6b8bc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6b8c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b8cc | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6b8d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b8dc | load | 4 | `[sp-68] +0x78` | `-` |
| `str` at 0x6b8ec | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6b8f4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b8f8 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6b904 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6b908 | load | 4 | `[sp-72] +0x78` | `-` |
| `str` at 0x6b918 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6b91c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6b928 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b92c | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6b938 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b93c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b948 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b94c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b958 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b95c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b968 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b96c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b978 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b97c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b988 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b98c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b998 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b99c | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b9a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b9ac | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b9b8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b9bc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b9c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b9cc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b9d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b9dc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b9e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b9ec | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6b9f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6b9fc | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6ba08 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6ba0c | load | 4 | `[sp-68] +0x10` | `-` |
| `str` at 0x6ba1c | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6ba24 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba28 | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6ba34 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba38 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6ba44 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba48 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6ba54 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba58 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6ba64 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba68 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6ba74 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba78 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6ba84 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba88 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6ba94 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6ba98 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6baa4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6baa8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bab4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bab8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bac4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bac8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bad4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bad8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bae4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bae8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6baf4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6baf8 | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bb04 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bb08 | load | 4 | `[sp-72] +0x10` | `-` |
| `str` at 0x6bb18 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x6bb1c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6bb80 | load | 4 | `abs 0x6c0ac` | `-` |
| `str` at 0x6bb90 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x6bb94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6bb98 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x6bba0 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x6bba4 | load | 4 | `abs 0x6c0b4` | `-` |
| `str` at 0x6bbbc | store | 4 | `sp -0x1c` | `ret(0xb6f64)` |
| `ldr` at 0x6bbc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bbd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bbd4 | load | 4 | `abs 0x6c0b8` | `-` |
| `ldr` at 0x6bbd8 | load | 4 | `abs 0x148dd4` | `-` |
| `str` at 0x6bbdc | store | 4 | `ret(0xb6f64) +0x18` | `[0x148dd4]` |
| `ldr` at 0x6bbe0 | load | 4 | `abs 0x6c0bc` | `-` |
| `ldr` at 0x6bbe4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6bbf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6bc04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6bc0c | load | 4 | `abs 0x6c0b0` | `-` |
| `str` at 0x6bc14 | store | 4 | `abs 0x148658` | `?` |
| `ldr` at 0x6bc1c | load | 4 | `abs 0x6c0b0` | `-` |
| `ldr` at 0x6bc20 | load | 4 | `abs 0x148658` | `-` |
| `ldr` at 0x6bc54 | load | 4 | `abs 0x6c0b0` | `-` |
| `str` at 0x6bc5c | store | 4 | `abs 0x148658` | `0x0` |
| `ldr` at 0x6bc60 | load | 4 | `abs 0x6c0c0` | `-` |
| `ldr` at 0x6bc70 | load | 4 | `abs 0x6c0c0` | `-` |
| `ldr` at 0x6bc74 | load | 4 | `abs 0x6c0ac` | `-` |
| `ldr` at 0x6bc94 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bc98 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldr` at 0x6bca4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bca8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bcb4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bcb8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bcc4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bcc8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bcd4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bcd8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bce4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bce8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bcf4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bcf8 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd04 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd08 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd14 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd18 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd24 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd28 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd34 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd38 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd44 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd48 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd58 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd64 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd68 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bd74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x6bd78 | load | 4 | `[sp-68] +0x10` | `-` |
| `ldr` at 0x6bde0 | load | 4 | `abs 0x6c0c0` | `-` |
| `str` at 0x6bdf0 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x6bdf4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6bdf8 | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x6be00 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x6be04 | load | 4 | `abs 0x6c0b4` | `-` |
| `str` at 0x6be1c | store | 4 | `sp -0x1c` | `ret(0xb6f64)` |
| `ldr` at 0x6be20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6be2c | load | 4 | `abs 0x6c0c4` | `-` |
| `ldr` at 0x6be38 | load | 4 | `abs 0x6c0a0` | `-` |
| `ldr` at 0x6be3c | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6be4c | load | 4 | `abs 0x6c0a4` | `-` |
| `ldr` at 0x6be50 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6be54 | load | 4 | `abs 0x6c0c8` | `-` |
| `ldr` at 0x6be5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6be60 | load | 4 | `abs 0x6c0b8` | `-` |
| `ldr` at 0x6be64 | load | 4 | `abs 0x148dd4` | `-` |
| `str` at 0x6be68 | store | 4 | `(unknown)` | `[0x148dd4]` |
| `ldr` at 0x6be6c | load | 4 | `abs 0x6c0bc` | `-` |
| `ldr` at 0x6be70 | load | 4 | `abs 0x2b16d4` | `-` |
| `ldr` at 0x6be7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6be98 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6be9c | load | 4 | `[sp-72] +0x4` | `-` |
| `ldr` at 0x6bea8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6beac | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6beb8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bebc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bec8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6becc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bed8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bedc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bee8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6beec | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bef8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6befc | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf08 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf0c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf18 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf1c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf28 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf2c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf38 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf3c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf48 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf4c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf58 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf5c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf68 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf6c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bf78 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6bf7c | load | 4 | `[sp-72] +0x10` | `-` |
| `ldr` at 0x6bfe4 | load | 4 | `abs 0x6c0c0` | `-` |
| `str` at 0x6bff4 | store | 4 | `sp -0x58` | `0x2` |
| `ldr` at 0x6bff8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6bffc | store | 4 | `sp -0x54` | `[sp-64]` |
| `str` at 0x6c004 | store | 4 | `sp -0x50` | `sp-60` |
| `ldr` at 0x6c008 | load | 4 | `abs 0x6c0b4` | `-` |
| `str` at 0x6c020 | store | 4 | `sp -0x1c` | `ret(0xb6f64)` |
| `ldr` at 0x6c024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c030 | load | 4 | `abs 0x6c0c4` | `-` |
| `ldr` at 0x6c03c | load | 4 | `abs 0x6c0a0` | `-` |
| `ldr` at 0x6c040 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6c050 | load | 4 | `abs 0x6c0a4` | `-` |
| `ldr` at 0x6c054 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6c058 | load | 4 | `abs 0x6c0cc` | `-` |
| `ldr` at 0x6c060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6c064 | load | 4 | `abs 0x6c0b8` | `-` |
| `ldr` at 0x6c068 | load | 4 | `abs 0x148dd4` | `-` |
| `str` at 0x6c06c | store | 4 | `(unknown)` | `[0x148dd4]` |
| `ldr` at 0x6c070 | load | 4 | `abs 0x6c0bc` | `-` |
| `ldr` at 0x6c074 | load | 4 | `abs 0x2b16d8` | `-` |
| `ldr` at 0x6c080 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6c094 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6c094 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6c094 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b554 | `(unknown callee) 0x76cdc` | `0x1f4` | `r1` | `r2` | `r3` |
| `bl` at 0x6b56c | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[0x148680]` |
| `bl` at 0x6b58c | `(unknown callee) 0x6c0d0` | `0x10f150` | `?` | `[0x148680]` | `ret(0x1b1e4)` |
| `bl` at 0x6b5bc | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f160` | `0x148680` | `0x144dc8` |
| `bl` at 0x6b5c0 | `(unknown callee) 0x6d51c` | `?` | `?` | `?` | `?` |
| `bl` at 0x6b5e8 | `(unknown callee) 0x79dcc` | `0x1` | `?` | `?` | `[0x1489b4]+312` |
| `bl` at 0x6b5fc | `(unknown callee) 0x79dcc` | `0x2` | `?` | `?` | `ret(0x79dcc)` |
| `bl` at 0x6b620 | `(unknown callee) 0x79dcc` | `0x1` | `?` | `?` | `ret(0x79dcc)` |
| `bl` at 0x6b724 | `(unknown callee) 0x79dcc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6b83c | `(unknown callee) 0x79dcc` | `0x1` | `?` | `?` | `[0x14893c]` |
| `bl` at 0x6b850 | `(unknown callee) 0x79dcc` | `0x2` | `?` | `?` | `ret(0x79dcc)` |
| `bl` at 0x6bb3c | `(unknown callee) 0x6907c` | `sp-64` | `sp-60` | `0x3` | `sp-64` |
| `bl` at 0x6bb54 | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6bb6c | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6bb88 | `(unknown callee) 0x71a6c` | `sp-64` | `sp-60` | `0x14892c` | `0x1` |
| `bl` at 0x6bbb4 | `(unknown callee) 0xb6f64` | `0x148924` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6bc08 | `(unknown callee) 0xb686c` | `?` | `[sp-28]` | `0x50` | `?` |
| `bl` at 0x6bc30 | `(unknown callee) 0x79dcc` | `0x1` | `?` | `?` | `[0x148658]` |
| `bl` at 0x6bc44 | `(unknown callee) 0x79dcc` | `0x2` | `?` | `?` | `ret(0x79dcc)` |
| `bl` at 0x6bc6c | `(unknown callee) 0x1b43c` | `0x148684` | `0x0` | `0x230` | `?` |
| `bl` at 0x6bc7c | `(unknown callee) 0x1b304` | `0x148684` | `0x14892c` | `0x70` | `?` |
| `bl` at 0x6bc84 | `(unknown callee) 0x79dcc` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6bd9c | `(unknown callee) 0x6907c` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x6bdb4 | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6bdcc | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6bde8 | `(unknown callee) 0x71a6c` | `sp-64` | `sp-60` | `0x148684` | `0x5` |
| `bl` at 0x6be14 | `(unknown callee) 0xb6f64` | `0x148924` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6be30 | `(unknown callee) 0x20240` | `0x10f174` | `?` | `?` | `ret(0xb6f64)` |
| `bl` at 0x6be58 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f1a0` | `?` | `0x144dc8` |
| `bl` at 0x6be80 | `(unknown callee) 0xb686c` | `[0x2b16d4]+20` | `[sp-28]` | `?` | `[0x2b16d4]+20` |
| `bl` at 0x6be88 | `(unknown callee) 0x79dcc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x6bfa0 | `(unknown callee) 0x6907c` | `sp-64` | `sp-60` | `0x2` | `sp-64` |
| `bl` at 0x6bfb8 | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6bfd0 | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6bfec | `(unknown callee) 0x71a6c` | `sp-64` | `sp-60` | `0x148684` | `0x5` |
| `bl` at 0x6c018 | `(unknown callee) 0xb6f64` | `0x148924` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6c034 | `(unknown callee) 0x20240` | `0x10f174` | `?` | `?` | `ret(0xb6f64)` |
| `bl` at 0x6c05c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f1b8` | `?` | `0x144dc8` |
| `bl` at 0x6c084 | `(unknown callee) 0xb686c` | `[0x2b16d8]+20` | `[sp-28]` | `?` | `[0x2b16d8]+20` |
| `bl` at 0x6c08c | `(unknown callee) 0x76cdc` | `0x1f4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6b564 | `[0x148680] == 0x0` | `[0x148680] != 0x0` |
| `bge` at 0x6b580 | `ret(0x1b1e4) >= [0x148680] (signed, 32-bit)` | `ret(0x1b1e4) < [0x148680] (signed, 32-bit)` |
| `beq` at 0x6b5ac | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6b5f4 | `ret(0x79dcc) != 0x0` | `ret(0x79dcc) == 0x0` |
| `bne` at 0x6b608 | `ret(0x79dcc) != 0x0` | `ret(0x79dcc) == 0x0` |
| `beq` at 0x6b62c | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x6b63c | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6b64c | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x6b65c | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x6b66c | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x6b67c | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x6b68c | `[[sp-68]+16] == 0x48` | `[[sp-68]+16] != 0x48` |
| `beq` at 0x6b69c | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x6b6ac | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x6b6bc | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x6b6cc | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x6b6dc | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x6b6ec | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x6b6fc | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x6b70c | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `bne` at 0x6b71c | `[[sp-68]+16] != 0x4d` | `[[sp-68]+16] == 0x4d` |
| `beq` at 0x6b730 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x6b740 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6b750 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x6b760 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x6b770 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x6b780 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x6b790 | `[[sp-72]+16] == 0x48` | `[[sp-72]+16] != 0x48` |
| `beq` at 0x6b7a0 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x6b7b0 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x6b7c0 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x6b7d0 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x6b7e0 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x6b7f0 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x6b800 | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x6b810 | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `bne` at 0x6b820 | `[[sp-72]+16] != 0x4d` | `[[sp-72]+16] == 0x4d` |
| `bne` at 0x6b834 | `[0x14893c] != 0x42` | `[0x14893c] == 0x42` |
| `beq` at 0x6b848 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x6b85c | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x6b874 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `bne` at 0x6b884 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6b8a0 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `bne` at 0x6b8b0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6b8c4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6b8d4 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6b8e4 | `[[sp-68]+120] == 0x0` | `[[sp-68]+120] != 0x0` |
| `beq` at 0x6b900 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6b910 | `[[sp-72]+120] == 0x0` | `[[sp-72]+120] != 0x0` |
| `bne` at 0x6b924 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6b934 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6b944 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x6b954 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x6b964 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x6b974 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x6b984 | `[[sp-68]+16] == 0x48` | `[[sp-68]+16] != 0x48` |
| `beq` at 0x6b994 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x6b9a4 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x6b9b4 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x6b9c4 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x6b9d4 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x6b9e4 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x6b9f4 | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x6ba04 | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `beq` at 0x6ba14 | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `beq` at 0x6ba30 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6ba40 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x6ba50 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x6ba60 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x6ba70 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x6ba80 | `[[sp-72]+16] == 0x48` | `[[sp-72]+16] != 0x48` |
| `beq` at 0x6ba90 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x6baa0 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x6bab0 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x6bac0 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x6bad0 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x6bae0 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x6baf0 | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x6bb00 | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `beq` at 0x6bb10 | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `beq` at 0x6bb24 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x6bbc8 | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |
| `blt` at 0x6bc28 | `[0x148658] < 0x0 (signed, 32-bit)` | `[0x148658] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x6bc3c | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `bne` at 0x6bc50 | `ret(0x79dcc) != 0x0` | `ret(0x79dcc) == 0x0` |
| `beq` at 0x6bc90 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x6bca0 | `[[sp-68]+4] == 0x0` | `[[sp-68]+4] != 0x0` |
| `beq` at 0x6bcb0 | `[[sp-68]+16] == 0x35` | `[[sp-68]+16] != 0x35` |
| `beq` at 0x6bcc0 | `[[sp-68]+16] == 0x4f` | `[[sp-68]+16] != 0x4f` |
| `beq` at 0x6bcd0 | `[[sp-68]+16] == 0x42` | `[[sp-68]+16] != 0x42` |
| `beq` at 0x6bce0 | `[[sp-68]+16] == 0x46` | `[[sp-68]+16] != 0x46` |
| `beq` at 0x6bcf0 | `[[sp-68]+16] == 0x48` | `[[sp-68]+16] != 0x48` |
| `beq` at 0x6bd00 | `[[sp-68]+16] == 0x4a` | `[[sp-68]+16] != 0x4a` |
| `beq` at 0x6bd10 | `[[sp-68]+16] == 0x4b` | `[[sp-68]+16] != 0x4b` |
| `beq` at 0x6bd20 | `[[sp-68]+16] == 0x21` | `[[sp-68]+16] != 0x21` |
| `beq` at 0x6bd30 | `[[sp-68]+16] == 0x5e` | `[[sp-68]+16] != 0x5e` |
| `beq` at 0x6bd40 | `[[sp-68]+16] == 0x2b` | `[[sp-68]+16] != 0x2b` |
| `beq` at 0x6bd50 | `[[sp-68]+16] == 0x4c` | `[[sp-68]+16] != 0x4c` |
| `beq` at 0x6bd60 | `[[sp-68]+16] == 0x5a` | `[[sp-68]+16] != 0x5a` |
| `beq` at 0x6bd70 | `[[sp-68]+16] == 0x7c` | `[[sp-68]+16] != 0x7c` |
| `beq` at 0x6bd80 | `[[sp-68]+16] == 0x4d` | `[[sp-68]+16] != 0x4d` |
| `bne` at 0x6be28 | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |
| `beq` at 0x6be48 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6be94 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x6bea4 | `[[sp-72]+4] == 0x0` | `[[sp-72]+4] != 0x0` |
| `beq` at 0x6beb4 | `[[sp-72]+16] == 0x35` | `[[sp-72]+16] != 0x35` |
| `beq` at 0x6bec4 | `[[sp-72]+16] == 0x4f` | `[[sp-72]+16] != 0x4f` |
| `beq` at 0x6bed4 | `[[sp-72]+16] == 0x42` | `[[sp-72]+16] != 0x42` |
| `beq` at 0x6bee4 | `[[sp-72]+16] == 0x46` | `[[sp-72]+16] != 0x46` |
| `beq` at 0x6bef4 | `[[sp-72]+16] == 0x48` | `[[sp-72]+16] != 0x48` |
| `beq` at 0x6bf04 | `[[sp-72]+16] == 0x4a` | `[[sp-72]+16] != 0x4a` |
| `beq` at 0x6bf14 | `[[sp-72]+16] == 0x4b` | `[[sp-72]+16] != 0x4b` |
| `beq` at 0x6bf24 | `[[sp-72]+16] == 0x21` | `[[sp-72]+16] != 0x21` |
| `beq` at 0x6bf34 | `[[sp-72]+16] == 0x5e` | `[[sp-72]+16] != 0x5e` |
| `beq` at 0x6bf44 | `[[sp-72]+16] == 0x2b` | `[[sp-72]+16] != 0x2b` |
| `beq` at 0x6bf54 | `[[sp-72]+16] == 0x4c` | `[[sp-72]+16] != 0x4c` |
| `beq` at 0x6bf64 | `[[sp-72]+16] == 0x5a` | `[[sp-72]+16] != 0x5a` |
| `beq` at 0x6bf74 | `[[sp-72]+16] == 0x7c` | `[[sp-72]+16] != 0x7c` |
| `beq` at 0x6bf84 | `[[sp-72]+16] == 0x4d` | `[[sp-72]+16] != 0x4d` |
| `bne` at 0x6c02c | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |
| `beq` at 0x6c04c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6c094 | `r0` | `ret(0xb6f64)` |

## `L1CfgOptimize`

- `L1CfgOptimize`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6d51c to 0x6e7f0.
  Established. The initial decision branches on `[0x148680] == 0x0`, with nonzero falling through
  (`beq` at 0x6d538). Another word read uses absolute `0x14867c` (`ldr` at 0x6d570); its zero
  comparison takes the branch and nonzero falls through (`beq` at 0x6d578). One call targets
  `0x24028` with candidate registers `[0x144dc8]`, `0x10f3b8`, `0x2b1628` and `0x144dc8` (`bl` at
  0x6d560). The body does not settle what physical configuration is being optimized.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6d52c | load | 4 | `abs 0x6e4f8` | `-` |
| `ldr` at 0x6d530 | load | 4 | `abs 0x148680` | `-` |
| `ldr` at 0x6d53c | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6d540 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d550 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6d554 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6d558 | load | 4 | `abs 0x6e504` | `-` |
| `ldr` at 0x6d55c | load | 4 | `abs 0x6e508` | `-` |
| `ldr` at 0x6d56c | load | 4 | `abs 0x6e50c` | `-` |
| `ldr` at 0x6d570 | load | 4 | `abs 0x14867c` | `-` |
| `ldr` at 0x6d588 | load | 4 | `abs 0x6e50c` | `-` |
| `ldr` at 0x6d58c | load | 4 | `abs 0x14867c` | `-` |
| `str` at 0x6d594 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6d598 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d5a4 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6d5a8 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d5b8 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6d5bc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6d5c0 | load | 4 | `abs 0x6e510` | `-` |
| `ldr` at 0x6d5c4 | load | 4 | `abs 0x6e508` | `-` |
| `ldr` at 0x6d5cc | load | 4 | `abs 0x6e4f8` | `-` |
| `ldr` at 0x6d5d0 | load | 4 | `abs 0x6e50c` | `-` |
| `ldr` at 0x6d5d4 | load | 4 | `abs 0x14867c` | `-` |
| `str` at 0x6d5dc | store | 4 | `abs 0x148680` | `[0x14867c]+5` |
| `ldr` at 0x6d5f4 | load | 4 | `abs 0x6e50c` | `-` |
| `str` at 0x6d5f8 | store | 4 | `abs 0x14867c` | `ret(0x1b1e4)` |
| `ldr` at 0x6d5fc | load | 4 | `abs 0x6e514` | `-` |
| `ldr` at 0x6d600 | load | 4 | `abs 0x1488bc` | `-` |
| `ldr` at 0x6d60c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d610 | load | 4 | `abs 0x6e514` | `-` |
| `ldr` at 0x6d614 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6d618 | load | 4 | `abs 0x1488bc` | `-` |
| `ldr` at 0x6d628 | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6d62c | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6d638 | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6d63c | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6d644 | load | 4 | `abs 0x6e514` | `-` |
| `ldr` at 0x6d648 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d64c | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x6d668 | store | 4 | `abs 0x1488bc` | `?` |
| `ldr` at 0x6d66c | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6d670 | load | 4 | `abs 0x6e514` | `-` |
| `ldr` at 0x6d674 | load | 4 | `abs 0x1488bc` | `-` |
| `str` at 0x6d694 | store | 4 | `abs 0x1488b8` | `ret(0x1b124)` |
| `ldr` at 0x6d698 | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6d69c | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6d6ac | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6d6b0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d6b4 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d6b8 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6d6cc | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6d6d0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6d6dc | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d6e0 | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x6d6e4 | store | 4 | `sp -0x7c` | `[0x1489b8]` |
| `ldr` at 0x6d6e8 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6d6ec | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d6fc | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6d700 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6d704 | load | 4 | `abs 0x6e51c` | `-` |
| `str` at 0x6d714 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x6d718 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d71c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d720 | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x6d730 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d734 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d750 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d758 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d764 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6d768 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d778 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6d77c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6d780 | load | 4 | `abs 0x6e520` | `-` |
| `ldr` at 0x6d784 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d790 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6d794 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d7a4 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6d7a8 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d7ac | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d7c8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d7d0 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6d7d4 | load | 4 | `abs 0x6e524` | `-` |
| `ldr` at 0x6d7d8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d7dc | load | 4 | `(unknown)` | `-` |
| `str` at 0x6d7e8 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x6d7ec | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d7f0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d80c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d814 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6d818 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d828 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d82c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d848 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d850 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6d864 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6d86c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6d870 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d874 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d878 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d87c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d880 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6d884 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6d888 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x6d88c | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x6d898 | store | 4 | `sp -0x9c` | `ret(0x6f784)` |
| `ldr` at 0x6d89c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d8a8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d8ac | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0x6d8b4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d8b8 | load | 4 | `ret(0x6f784) +0x14` | `-` |
| `ldr` at 0x6d8c4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d8c8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6d8cc | store | 4 | `ret(0x6f784) +0x10` | `[sp-128]` |
| `ldr` at 0x6d8d0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d8d4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d8d8 | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x6d8e0 | store | 4 | `[sp-156] +0x14` | `[[sp-152]+20]+1` |
| `ldr` at 0x6d8e4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d8e8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d8ec | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x6d8f0 | store | 4 | `[sp-156] +0xc` | `[[sp-152]+12]` |
| `ldr` at 0x6d8f4 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6d8f8 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d908 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6d90c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d910 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d914 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d918 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x6d920 | store | 4 | `sp -0xc0` | `[[sp-156]+16]+64` |
| `ldr` at 0x6d924 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d928 | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x6d92c | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x6d930 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d934 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x6d938 | store | 4 | `sp -0xb8` | `[[sp-156]+20]` |
| `ldr` at 0x6d93c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6d940 | load | 4 | `abs 0x6e528` | `-` |
| `ldr` at 0x6d944 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6d948 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6d954 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d958 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6d95c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6d978 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d980 | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0x6d984 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d990 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d994 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d998 | load | 4 | `[sp-152] +0x4` | `-` |
| `str` at 0x6d99c | store | 4 | `sp -0xc0` | `[[sp-152]+4]` |
| `ldr` at 0x6d9a0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d9a4 | load | 4 | `[sp-152] +0x8` | `-` |
| `str` at 0x6d9a8 | store | 4 | `sp -0xbc` | `[[sp-152]+8]` |
| `ldr` at 0x6d9ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d9b0 | load | 4 | `[sp-152] +0xc` | `-` |
| `str` at 0x6d9b4 | store | 4 | `sp -0xb8` | `[[sp-152]+12]` |
| `ldr` at 0x6d9b8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6d9bc | load | 4 | `[sp-152] +0x14` | `-` |
| `str` at 0x6d9c4 | store | 4 | `sp -0xb4` | `[[sp-152]+20]+1` |
| `ldr` at 0x6d9c8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6d9cc | store | 4 | `sp -0xb0` | `[sp-128]` |
| `ldr` at 0x6d9d0 | load | 4 | `abs 0x6e52c` | `-` |
| `ldr` at 0x6d9d4 | load | 4 | `abs 0x6e530` | `-` |
| `ldr` at 0x6d9d8 | load | 4 | `abs 0x6e534` | `-` |
| `ldr` at 0x6d9dc | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x6d9e8 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6d9ec | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d9fc | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6da00 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6da04 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6da08 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6da10 | store | 4 | `sp -0xc0` | `[sp-128]+64` |
| `ldr` at 0x6da14 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6da18 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6da34 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6da3c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6da44 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6da48 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6da4c | load | 4 | `[sp-152] +0x10` | `-` |
| `str` at 0x6da54 | store | 4 | `sp -0xb8` | `[[sp-152]+16]+64` |
| `ldr` at 0x6da58 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6da5c | load | 4 | `abs 0x6e538` | `-` |
| `ldr` at 0x6da60 | load | 4 | `[sp-152] +0x0` | `-` |
| `ldr` at 0x6da64 | load | 4 | `[sp-152] +0x4` | `-` |
| `ldr` at 0x6da6c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x6da74 | store | 4 | `sp -0x84` | `[sp-132]+1` |
| `ldr` at 0x6da7c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6da84 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `str` at 0x6da90 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x6da94 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6da98 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6da9c | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6daac | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dab0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6dac4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6dacc | load | 4 | `abs 0x6e53c` | `-` |
| `ldr` at 0x6dad0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6dad4 | load | 4 | `abs 0x14286c` | `-` |
| `ldr` at 0x6dae0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dae4 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dae8 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6daec | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6db00 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6db08 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6db0c | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6db10 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6db14 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x6db28 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6db30 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6db34 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6db38 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6db3c | load | 4 | `abs 0x6e530` | `-` |
| `ldr` at 0x6db40 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x6db44 | load | 4 | `abs 0x6e540` | `-` |
| `ldr` at 0x6db4c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x6db54 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x6db5c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6db60 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6db64 | load | 4 | `abs 0x148934` | `-` |
| `ldr` at 0x6db68 | load | 4 | `abs 0x148938` | `-` |
| `str` at 0x6db74 | store | 4 | `sp -0x90` | `ret(0x748c4)` |
| `ldr` at 0x6db78 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6db7c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6db84 | store | 4 | `ret(0x748c4) +0xc` | `0x0` |
| `str` at 0x6db8c | store | 4 | `abs 0x1489a0` | `0x0` |
| `ldr` at 0x6db90 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6db94 | load | 4 | `abs 0x148984` | `-` |
| `ldr` at 0x6dba0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dba4 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6dbac | store | 4 | `[sp-144] +0xc` | `?` |
| `str` at 0x6dbb4 | store | 4 | `abs 0x1489a0` | `?` |
| `ldr` at 0x6dbb8 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dbbc | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dbc0 | load | 4 | `abs 0x148934` | `-` |
| `str` at 0x6dbc4 | store | 4 | `abs 0x1489a4` | `[0x148934]` |
| `ldr` at 0x6dbc8 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dbcc | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dbd0 | load | 4 | `abs 0x148938` | `-` |
| `str` at 0x6dbd4 | store | 4 | `abs 0x1489a8` | `[0x148938]` |
| `ldr` at 0x6dbdc | load | 4 | `abs 0x6e580` | `-` |
| `str` at 0x6dbe4 | store | 4 | `abs 0x148984` | `0x0` |
| `str` at 0x6dbf0 | store | 4 | `sp -0x94` | `ret(0x6f9f8)` |
| `ldr` at 0x6dbf4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc00 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc04 | load | 4 | `ret(0x6f9f8) +0xc` | `-` |
| `ldr` at 0x6dc10 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc14 | load | 4 | `[sp-148] +0xc` | `-` |
| `ldr` at 0x6dc20 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc24 | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x6dc30 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc34 | load | 4 | `[sp-148] +0x8` | `-` |
| `ldr` at 0x6dc44 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dc48 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc4c | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6dc50 | store | 4 | `abs 0x148984` | `[[sp-148]+16]` |
| `ldr` at 0x6dc54 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x6dc58 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc5c | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6dc60 | store | 4 | `[sp-144] +0xc` | `[[sp-148]+16]` |
| `ldr` at 0x6dc64 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dc68 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc6c | load | 4 | `[sp-148] +0x10` | `-` |
| `str` at 0x6dc70 | store | 4 | `abs 0x1489a0` | `[[sp-148]+16]` |
| `ldr` at 0x6dc74 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dc78 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc7c | load | 4 | `[sp-148] +0x0` | `-` |
| `str` at 0x6dc80 | store | 4 | `abs 0x1489a4` | `[[sp-148]]` |
| `ldr` at 0x6dc84 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dc88 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc8c | load | 4 | `[sp-148] +0x4` | `-` |
| `str` at 0x6dc90 | store | 4 | `abs 0x1489a8` | `[[sp-148]+4]` |
| `ldr` at 0x6dc94 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dc98 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6dc9c | load | 4 | `[sp-148] +0x14` | `-` |
| `str` at 0x6dca0 | store | 4 | `abs 0x1489ac` | `[[sp-148]+20]` |
| `ldr` at 0x6dca4 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6dca8 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6dcb8 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6dcbc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6dcc0 | load | 4 | `abs 0x6e544` | `-` |
| `str` at 0x6dccc | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x6dcd0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dcd4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6dcd8 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6dce4 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dce8 | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6dcec | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dcf0 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6dd04 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6dd08 | load | 4 | `abs 0x1488b8` | `-` |
| `str` at 0x6dd28 | store | 4 | `sp -0xa4` | `0x1` |
| `ldr` at 0x6dd2c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6dd30 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6dd34 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6dd38 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6dd48 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6dd4c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6dd50 | load | 4 | `abs 0x6e548` | `-` |
| `ldr` at 0x6dd54 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6dd5c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6dd68 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6dd6c | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6dd7c | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6dd80 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6dd84 | load | 4 | `abs 0x6e54c` | `-` |
| `str` at 0x6dd90 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x6dd94 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dd98 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6dd9c | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6dda8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6ddb0 | store | 4 | `sp -0x8c` | `0x1` |
| `ldr` at 0x6ddb8 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6ddc0 | load | 4 | `abs 0x6e550` | `-` |
| `ldr` at 0x6ddc4 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6ddc8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6dddc | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6dde0 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6ddf0 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6ddf8 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6ddfc | load | 4 | `abs 0x6e554` | `-` |
| `str` at 0x6de08 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6de0c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6de10 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6de14 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6de24 | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6de28 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6de2c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6de40 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6de48 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6de4c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6de60 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6de68 | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6de6c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6de70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6de74 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6de80 | store | 4 | `sp -0x9c` | `ret(0x6f784)` |
| `ldr` at 0x6de84 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6de90 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6de98 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6dea0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dea4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6deb8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6dec0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6dec4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6ded8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6dee0 | load | 4 | `abs 0x6e558` | `-` |
| `ldr` at 0x6dee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6dee8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6defc | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6df00 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6df10 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6df18 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6df1c | load | 4 | `abs 0x6e554` | `-` |
| `ldr` at 0x6df24 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6df2c | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x6df38 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6df3c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6df40 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6df50 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6df54 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6df58 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6df6c | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6df70 | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6df78 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6df8c | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6df90 | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6df98 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6df9c | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6dfa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6dfa4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6dfb0 | store | 4 | `sp -0x9c` | `ret(0x6f784)` |
| `ldr` at 0x6dfb4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6dfc0 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6dfc8 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6dfd0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6dfe4 | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6dfe8 | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6dff0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e004 | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6e008 | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6e014 | load | 4 | `abs 0x6e55c` | `-` |
| `ldr` at 0x6e018 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e01c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e030 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6e034 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e044 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6e04c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e050 | load | 4 | `abs 0x6e554` | `-` |
| `ldr` at 0x6e058 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e060 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `str` at 0x6e06c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6e070 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e074 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e078 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6e088 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e08c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e0a0 | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x6e0a8 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x6e0ac | load | 4 | `abs 0x6e518` | `-` |
| `ldr` at 0x6e0b0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e0b4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e0b8 | load | 4 | `abs 0x1488b8` | `-` |
| `ldr` at 0x6e0bc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6e0c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e0c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e0d0 | store | 4 | `sp -0x98` | `ret(0x6f784)` |
| `ldr` at 0x6e0d4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e0e0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e0e4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e0e8 | load | 4 | `[sp-156] +0x14` | `-` |
| `ldr` at 0x6e0ec | load | 4 | `ret(0x6f784) +0x14` | `-` |
| `ldr` at 0x6e0f8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6e100 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6e108 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e10c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e110 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e114 | load | 4 | `ret(0x6f784) +0x14` | `-` |
| `str` at 0x6e118 | store | 4 | `sp -0xc0` | `[ret(0x6f784)+20]` |
| `ldr` at 0x6e11c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e120 | load | 4 | `[sp-156] +0x14` | `-` |
| `str` at 0x6e124 | store | 4 | `sp -0xbc` | `[[sp-156]+20]` |
| `ldr` at 0x6e12c | load | 4 | `abs 0x6e560` | `-` |
| `ldr` at 0x6e130 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6e134 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6e148 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6e14c | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e15c | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6e164 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e168 | load | 4 | `abs 0x6e554` | `-` |
| `ldr` at 0x6e170 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e17c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e180 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e184 | load | 4 | `[sp-156] +0x10` | `-` |
| `ldr` at 0x6e188 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e194 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6e19c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6e1a4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e1a8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e1ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e1b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e1b4 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6e1b8 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e1bc | load | 4 | `[sp-156] +0x10` | `-` |
| `str` at 0x6e1c0 | store | 4 | `sp -0xbc` | `[[sp-156]+16]` |
| `ldr` at 0x6e1c8 | load | 4 | `abs 0x6e564` | `-` |
| `ldr` at 0x6e1cc | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6e1d0 | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6e1e4 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6e1e8 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e1f8 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6e200 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e204 | load | 4 | `abs 0x6e554` | `-` |
| `ldr` at 0x6e20c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e218 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e21c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e220 | load | 4 | `[sp-156] +0xc` | `-` |
| `ldr` at 0x6e224 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e230 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6e238 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6e240 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e244 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e248 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e24c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e250 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6e254 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e258 | load | 4 | `[sp-156] +0xc` | `-` |
| `str` at 0x6e25c | store | 4 | `sp -0xbc` | `[[sp-156]+12]` |
| `ldr` at 0x6e264 | load | 4 | `abs 0x6e568` | `-` |
| `ldr` at 0x6e268 | load | 4 | `[sp-156] +0x0` | `-` |
| `ldr` at 0x6e26c | load | 4 | `[sp-156] +0x4` | `-` |
| `ldr` at 0x6e280 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6e284 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e294 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6e29c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e2a0 | load | 4 | `abs 0x6e554` | `-` |
| `ldr` at 0x6e2a8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e2b0 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x6e2c0 | load | 4 | `abs 0x6e56c` | `-` |
| `ldr` at 0x6e2c4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6e2d8 | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6e2dc | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e2ec | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6e2f4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e2f8 | load | 4 | `abs 0x6e554` | `-` |
| `ldr` at 0x6e300 | load | 4 | `abs 0x6e500` | `-` |
| `ldr` at 0x6e304 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e308 | load | 4 | `abs 0x6e54c` | `-` |
| `str` at 0x6e314 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x6e318 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e31c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e320 | load | 4 | `abs 0x1489c4` | `-` |
| `str` at 0x6e334 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6e338 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e33c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e340 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e344 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e358 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x6e360 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e364 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e378 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x6e380 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6e384 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6e388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e38c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e398 | store | 4 | `sp -0x9c` | `ret(0x6f784)` |
| `ldr` at 0x6e39c | load | 4 | `abs 0x6e4fc` | `-` |
| `ldr` at 0x6e3a0 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e3b0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e3c0 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e3c8 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6e3cc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e3d8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e3dc | store | 4 | `sp -0xa8` | `[sp-160]` |
| `ldr` at 0x6e3e0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e3e4 | load | 4 | `abs 0x148984` | `-` |
| `ldr` at 0x6e3f0 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e3f4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e3f8 | load | 4 | `ret(0x6f784) +0x10` | `-` |
| `ldr` at 0x6e414 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6e41c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e428 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x6e430 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x6e434 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x6e438 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6e43c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e440 | store | 4 | `sp -0xac` | `[sp-160]` |
| `ldr` at 0x6e444 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6e454 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e458 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e45c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e478 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6e480 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e48c | load | 4 | `sp -0xac` | `-` |
| `str` at 0x6e494 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x6e498 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x6e49c | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6e4a0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e4ac | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e4b0 | load | 4 | `abs 0x148984` | `-` |
| `ldr` at 0x6e4bc | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e4c0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e4c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e4e0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6e4e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e588 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6e598 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e59c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e5a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e5bc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6e5c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e5d0 | load | 4 | `abs 0x6e574` | `-` |
| `ldr` at 0x6e5d4 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e5e4 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e5e8 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e5ec | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e600 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x6e608 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e60c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e620 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x6e628 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e62c | store | 4 | `sp -0xc0` | `[sp-160]` |
| `ldr` at 0x6e630 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e634 | load | 4 | `abs 0x6e570` | `-` |
| `ldr` at 0x6e638 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e63c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e644 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e648 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e65c | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x6e664 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e668 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e67c | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x6e688 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e68c | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x6e690 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6e6b4 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e6b8 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e6bc | load | 4 | `abs 0x1489c4` | `-` |
| `str` at 0x6e6c4 | store | 4 | `abs 0x1489c4` | `[0x1489c4]-1` |
| `ldr` at 0x6e6c8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x6e6d0 | store | 4 | `sp -0x88` | `[sp-136]-1` |
| `ldr` at 0x6e6d4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x6e6dc | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x6e6e8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6e6f4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6e700 | load | 4 | `abs 0x6e574` | `-` |
| `ldr` at 0x6e704 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e714 | load | 4 | `abs 0x6e578` | `-` |
| `ldr` at 0x6e718 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e71c | load | 4 | `abs 0x6e57c` | `-` |
| `str` at 0x6e728 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x6e72c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e730 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e734 | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x6e744 | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e748 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e764 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6e76c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e77c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e780 | load | 4 | `abs 0x14899c` | `-` |
| `ldr` at 0x6e78c | load | 4 | `abs 0x6e580` | `-` |
| `ldr` at 0x6e790 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e7ac | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6e7b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e7c0 | load | 4 | `abs 0x6e584` | `-` |
| `ldr` at 0x6e7c4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e7c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e7d4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6e7dc | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6e7e4 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldmdb` at 0x6e7ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6e7ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6e7ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6e7ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6d560 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f3b8` | `0x2b1628` | `0x144dc8` |
| `bl` at 0x6d564 | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `?` |
| `bl` at 0x6d580 | `(unknown callee) 0x1b1e4` | `0x0` | `r1` | `r2` | `[0x14867c]` |
| `bl` at 0x6d5c8 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f3d8` | `0x2b1628` | `0x144dc8` |
| `bl` at 0x6d5e0 | `(unknown callee) 0x6c168` | `?` | `?` | `0x148680` | `[0x14867c]+5` |
| `bl` at 0x6d5ec | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x6d640 | `(unknown callee) 0x1b400` | `[0x1488b8]` | `?` | `?` | `0x1488b8` |
| `bl` at 0x6d68c | `(unknown callee) 0x1b124` | `?` | `0x1488bc` | `[0x1488bc]` | `?` |
| `bl` at 0x6d6a8 | `(unknown callee) 0x1b43c` | `[0x1488b8]` | `0x0` | `0x4` | `0x1488b8` |
| `bl` at 0x6d6d8 | `(unknown callee) 0x1b304` | `[0x1488b8]` | `[0x1489c0]` | `?` | `?` |
| `bl` at 0x6d708 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f3f4` | `?` | `0x144dc8` |
| `bl` at 0x6d70c | `(unknown callee) 0x6b1e0` | `?` | `?` | `?` | `?` |
| `bl` at 0x6d788 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f408` | `[sp-128]` | `0x144dc8` |
| `bl` at 0x6d7e0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f428` | `[sp-128]` | `?` |
| `bl` at 0x6d890 | `(unknown callee) 0x6f784` | `[0x1489c0]` | `[0x1489b8]` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x6d94c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f448` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6d9e0 | `(unknown callee) 0x70e7c` | `0x1489c0` | `0x1489b8` | `0x1489bc` | `[[sp-152]]` |
| `bl` at 0x6da68 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f478` | `[[sp-152]]` | `[[sp-152]+4]` |
| `bl` at 0x6db48 | `(unknown callee) 0x71544` | `[0x1489c0]` | `0x1489b8` | `[0x1489cc]` | `0x1489c4` |
| `bl` at 0x6db6c | `(unknown callee) 0x748c4` | `[0x148934]` | `[0x148938]` | `0x14892c` | `0x14892c` |
| `bl` at 0x6dbe8 | `(unknown callee) 0x6f9f8` | `ret(0x748c4)` | `0x14892c` | `0x14892c` | `0x0` |
| `bl` at 0x6dcc4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f4a0` | `?` | `0x144dc8` |
| `bl` at 0x6dd10 | `(unknown callee) 0x1b274` | `[0x1489c0]` | `[0x1488b8]` | `?` | `?` |
| `bl` at 0x6dd58 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f4b4` | `?` | `0x144dc8` |
| `bl` at 0x6dd88 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f4cc` | `?` | `0x144dc8` |
| `bl` at 0x6ddcc | `(unknown callee) 0x1b49c` | `sp-120` | `0x10f4f0` | `[0x1489b8]` | `[sp-124]` |
| `bl` at 0x6ddd8 | `(unknown callee) 0x6d240` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6de00 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f514` | `sp-120` | `0x144dc8` |
| `bl` at 0x6de78 | `(unknown callee) 0x6f784` | `[0x1488b8]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x6deec | `(unknown callee) 0x1b49c` | `sp-120` | `0x10f51c` | `?` | `?` |
| `bl` at 0x6def8 | `(unknown callee) 0x6d240` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6df20 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f514` | `sp-120` | `0x144dc8` |
| `bl` at 0x6dfa8 | `(unknown callee) 0x6f784` | `[0x1489c0]` | `[0x1489b8]` | `?` | `?` |
| `bl` at 0x6e020 | `(unknown callee) 0x1b49c` | `sp-120` | `0x10f52c` | `?` | `?` |
| `bl` at 0x6e02c | `(unknown callee) 0x6d240` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e054 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f514` | `sp-120` | `0x144dc8` |
| `bl` at 0x6e0c8 | `(unknown callee) 0x6f784` | `[0x1488b8]` | `[sp-124]` | `?` | `?` |
| `bl` at 0x6e138 | `(unknown callee) 0x1b49c` | `sp-120` | `0x10f540` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6e144 | `(unknown callee) 0x6d240` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e16c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f514` | `sp-120` | `0x144dc8` |
| `bl` at 0x6e1d4 | `(unknown callee) 0x1b49c` | `sp-120` | `0x10f558` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6e1e0 | `(unknown callee) 0x6d240` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e208 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f514` | `sp-120` | `0x144dc8` |
| `bl` at 0x6e270 | `(unknown callee) 0x1b49c` | `sp-120` | `0x10f574` | `[[sp-156]]` | `[[sp-156]+4]` |
| `bl` at 0x6e27c | `(unknown callee) 0x6d240` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e2a4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f514` | `sp-120` | `0x144dc8` |
| `bl` at 0x6e2c8 | `(unknown callee) 0x1b49c` | `sp-120` | `0x10f590` | `?` | `sp-120` |
| `bl` at 0x6e2d4 | `(unknown callee) 0x6d240` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x6e2fc | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f514` | `sp-120` | `0x144dc8` |
| `bl` at 0x6e30c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f4cc` | `?` | `0x144dc8` |
| `bl` at 0x6e390 | `(unknown callee) 0x6f784` | `[0x1489c0]` | `[0x1489b8]` | `?` | `?` |
| `bl` at 0x6e640 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f5a0` | `?` | `?` |
| `bl` at 0x6e6b0 | `(unknown callee) 0x1b304` | `?` | `?` | `?` | `?` |
| `bl` at 0x6e6e4 | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `[0x1489c4]` |
| `bl` at 0x6e720 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f5c0` | `?` | `0x144dc8` |
| `bl` at 0x6e7cc | `(unknown callee) 0x71ea8` | `?` | `0x14892c` | `0x148ad0` | `[sp-128]` |
| `bl` at 0x6e7d8 | `(unknown callee) 0x6c734` | `[sp-128]` | `0x14892c` | `?` | `[0x14899c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6d538 | `[0x148680] == 0x0` | `[0x148680] != 0x0` |
| `beq` at 0x6d54c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d578 | `[0x14867c] == 0x0` | `[0x14867c] != 0x0` |
| `bgt` at 0x6d5a0 | `[sp-152] > 0x4 (signed, 32-bit)` | `[sp-152] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x6d5b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d608 | `[0x1488bc] == 0x0` | `[0x1488bc] != 0x0` |
| `bgt` at 0x6d620 | `[0x1489b8] > [0x1488bc] (signed, 32-bit)` | `[0x1489b8] <= [0x1488bc] (signed, 32-bit)` |
| `beq` at 0x6d634 | `[0x1488b8] == 0x0` | `[0x1488b8] != 0x0` |
| `beq` at 0x6d6f8 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6d728 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6d760 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6d774 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d7a0 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6d820 | `[sp-132] < ? (signed, 32-bit)` | `[sp-132] >= ? (signed, 32-bit)` |
| `beq` at 0x6d8a4 | `ret(0x6f784) == 0x0` | `ret(0x6f784) != 0x0` |
| `bhs` at 0x6d8c0 | `[[sp-152]+20]+1 >= [ret(0x6f784)+20] (unsigned, 32-bit)` | `[[sp-152]+20]+1 < [ret(0x6f784)+20] (unsigned, 32-bit)` |
| `beq` at 0x6d904 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d98c | `[[sp-152]+16] == ?` | `[[sp-152]+16] != ?` |
| `beq` at 0x6d9f8 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6daa4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x6dadc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x6db9c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6dbfc | `ret(0x6f9f8) == 0x0` | `ret(0x6f9f8) != 0x0` |
| `beq` at 0x6dc0c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6dc1c | `[[sp-148]+12] == 0x0` | `[[sp-148]+12] != 0x0` |
| `beq` at 0x6dc2c | `[[sp-148]+8] == 0x52` | `[[sp-148]+8] != 0x52` |
| `beq` at 0x6dc3c | `[[sp-148]+8] == 0x44` | `[[sp-148]+8] != 0x44` |
| `beq` at 0x6dcb4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6dce0 | `[sp-124] != [0x1489b8]` | `[sp-124] == [0x1489b8]` |
| `bne` at 0x6dd1c | `ret(0x1b274) != 0x0` | `ret(0x1b274) == 0x0` |
| `beq` at 0x6dd44 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6dd64 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6dd78 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6dda4 | `[sp-124] == [0x1489b8]` | `[sp-124] != [0x1489b8]` |
| `beq` at 0x6ddec | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6de1c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6de8c | `ret(0x6f784) != 0x0` | `ret(0x6f784) == 0x0` |
| `beq` at 0x6df0c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6df48 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6dfbc | `ret(0x6f784) != 0x0` | `ret(0x6f784) == 0x0` |
| `beq` at 0x6e040 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e080 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x6e0dc | `ret(0x6f784) == 0x0` | `ret(0x6f784) != 0x0` |
| `beq` at 0x6e0f4 | `[[sp-156]+20] == [ret(0x6f784)+20]` | `[[sp-156]+20] != [ret(0x6f784)+20]` |
| `beq` at 0x6e158 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e178 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6e190 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6e1f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e214 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6e22c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x6e290 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e2e8 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e328 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x6e3ac | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e3d4 | `[sp-160] != 0x0` | `[sp-160] == 0x0` |
| `bne` at 0x6e3ec | `[0x148984] != 0x0` | `[0x148984] == 0x0` |
| `bne` at 0x6e424 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e450 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e488 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e4a8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x6e4b8 | `[0x148984] != 0x0` | `[0x148984] == 0x0` |
| `bne` at 0x6e4f0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e594 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e5cc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e5e0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6e6f0 | `[sp-140] == 0x0` | `[sp-140] != 0x0` |
| `beq` at 0x6e6fc | `[sp-140] == 0x2` | `[sp-140] != 0x2` |
| `beq` at 0x6e710 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6e73c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6e774 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e788 | `[0x14899c] == 0x0` | `[0x14899c] != 0x0` |
| `beq` at 0x6e7bc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6e7ec | `r0` | `?` |

## `L1CfgProcessRequest`

- `L1CfgProcessRequest`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x720ec to 0x727a8.
  Established. This request path reads a byte at `r0 +0x24` and a word at `r0 +0x28` (`ldrb` at
  0x7210c, `ldr` at 0x72120); its call to `0xb7944` has candidate registers entry r0, `0x0`, entry
  r2 and unknown (`bl` at 0x72134). A later comparison takes the branch for signed `[sp-116] > 0xf`
  and falls through for at most that value (`bgt` at 0x7216c). Request-code meanings remain unknown
  in this reading.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x720fc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x72104 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x72108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7210c | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x7211c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72120 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x72124 | store | 4 | `sp -0x8c` | `[r0+40]` |
| `ldr` at 0x7212c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72138 | store | 4 | `sp -0x8c` | `ret(0xb7944)` |
| `ldr` at 0x7213c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x72140 | store | 4 | `sp -0x74` | `[sp-140]` |
| `ldr` at 0x72154 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x7215c | load | 4 | `sp -0x74` | `-` |
| `str` at 0x72160 | store | 4 | `sp -0x98` | `[sp-116]` |
| `ldr` at 0x72164 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72170 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x7217c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72188 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x72194 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x721a4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x721b4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x721c0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x721cc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x721dc | load | 4 | `abs 0x72768` | `-` |
| `str` at 0x721e8 | store | 4 | `abs 0x148670` | `ret(0x1b1e4)` |
| `ldr` at 0x721ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x721f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x721fc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x72208 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7220c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x7221c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72220 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x72224 | store | 4 | `sp -0x90` | `[[sp-24]+48]` |
| `ldr` at 0x7222c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72238 | store | 4 | `sp -0x90` | `ret(0xb7944)` |
| `ldr` at 0x7223c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x72240 | store | 4 | `sp -0x78` | `[sp-144]` |
| `ldr` at 0x72244 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x72254 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x72258 | load | 4 | `abs 0x7276c` | `-` |
| `ldr` at 0x7225c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x72270 | load | 4 | `abs 0x7276c` | `-` |
| `ldr` at 0x72274 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x72280 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x72284 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x722a0 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x722a4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x722b8 | load | 4 | `abs 0x72770` | `-` |
| `ldr` at 0x722c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x722c4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x722d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x722d4 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x722e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x722e8 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x722ec | store | 4 | `sp -0x94` | `[[sp-24]+56]` |
| `ldr` at 0x722f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72300 | store | 4 | `sp -0x94` | `ret(0xb7944)` |
| `ldr` at 0x72304 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x72308 | store | 4 | `sp -0x7c` | `[sp-148]` |
| `ldr` at 0x7230c | load | 4 | `abs 0x72774` | `-` |
| `ldr` at 0x72310 | load | 4 | `abs 0x72778` | `-` |
| `ldr` at 0x72314 | load | 4 | `abs 0x1488c0` | `-` |
| `ldr` at 0x72318 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x72324 | load | 4 | `abs 0x7277c` | `-` |
| `ldr` at 0x72328 | load | 4 | `abs 0x1488c4` | `-` |
| `ldr` at 0x72334 | load | 4 | `abs 0x7277c` | `-` |
| `ldr` at 0x72338 | load | 4 | `abs 0x1488c4` | `-` |
| `ldr` at 0x72340 | load | 4 | `abs 0x72780` | `-` |
| `ldr` at 0x72344 | load | 4 | `abs 0x1488c8` | `-` |
| `ldr` at 0x72350 | load | 4 | `abs 0x72780` | `-` |
| `ldr` at 0x72354 | load | 4 | `abs 0x1488c8` | `-` |
| `ldr` at 0x7235c | load | 4 | `abs 0x72774` | `-` |
| `ldr` at 0x72360 | load | 4 | `abs 0x72778` | `-` |
| `ldr` at 0x72364 | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x72368 | store | 4 | `abs 0x1488c0` | `[0x1489b8]` |
| `ldr` at 0x7236c | load | 4 | `abs 0x7277c` | `-` |
| `ldr` at 0x72370 | load | 4 | `abs 0x72774` | `-` |
| `ldr` at 0x72374 | load | 4 | `abs 0x1488c0` | `-` |
| `str` at 0x72380 | store | 4 | `abs 0x1488c4` | `ret(0x1b124)` |
| `ldr` at 0x72384 | load | 4 | `abs 0x72780` | `-` |
| `ldr` at 0x72388 | load | 4 | `abs 0x72774` | `-` |
| `ldr` at 0x7238c | load | 4 | `abs 0x1488c0` | `-` |
| `str` at 0x723ac | store | 4 | `abs 0x1488c8` | `ret(0x1b124)` |
| `ldr` at 0x723c8 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x723d8 | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x723e0 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x723e8 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x723ec | load | 4 | `abs 0x72778` | `-` |
| `ldr` at 0x723f0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x723f4 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x72404 | load | 4 | `abs 0x72778` | `-` |
| `ldr` at 0x72408 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7241c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72424 | load | 4 | `abs 0x72778` | `-` |
| `ldr` at 0x72428 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7243c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72444 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72448 | load | 4 | `(unknown)` | `-` |
| `str` at 0x72454 | store | 4 | `sp -0x78` | `ret(0x74708)` |
| `ldr` at 0x72458 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x72464 | load | 4 | `abs 0x7277c` | `-` |
| `ldr` at 0x72468 | load | 4 | `abs 0x1488c4` | `-` |
| `ldr` at 0x7246c | load | 4 | `sp -0x88` | `-` |
| `ldrb` at 0x72474 | load | 1 | `sp -0x80` | `-` |
| `strb` at 0x72478 | store | 1 | `([0x1488c4] + [sp-136]) +0x0` | `([sp-128]:1 & 0xff)` |
| `ldr` at 0x7247c | load | 4 | `abs 0x72780` | `-` |
| `ldr` at 0x72480 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72494 | load | 4 | `abs 0x1488c8` | `-` |
| `ldr` at 0x724a0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x724ac | load | 4 | `sp -0x88` | `-` |
| `str` at 0x724b4 | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0x724b8 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x724c0 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldr` at 0x724d8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x724f0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72518 | load | 4 | `abs 0x7277c` | `-` |
| `ldr` at 0x72524 | load | 4 | `abs 0x1488c4` | `-` |
| `ldr` at 0x72528 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72538 | load | 4 | `abs 0x72780` | `-` |
| `ldr` at 0x72544 | load | 4 | `abs 0x1488c8` | `-` |
| `ldr` at 0x72548 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x72564 | load | 4 | `abs 0x72778` | `-` |
| `ldr` at 0x72580 | load | 4 | `abs 0x72784` | `-` |
| `ldr` at 0x7259c | load | 4 | `abs 0x72784` | `-` |
| `ldr` at 0x725b8 | load | 4 | `abs 0x72784` | `-` |
| `str` at 0x725cc | store | 4 | `sp -0xa4` | `sp-96` |
| `ldr` at 0x725d0 | load | 4 | `abs 0x72788` | `-` |
| `ldr` at 0x725d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x725dc | load | 4 | `sp -0x64` | `-` |
| `str` at 0x725e8 | store | 4 | `sp -0x68` | `ret(0xb6bdc)` |
| `ldr` at 0x725f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7260c | load | 4 | `abs 0x72778` | `-` |
| `ldr` at 0x72628 | load | 4 | `abs 0x72784` | `-` |
| `ldr` at 0x72634 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72638 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x7263c | store | 4 | `sp -0x20` | `[[sp-24]+28]` |
| `ldr` at 0x72640 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72644 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x72648 | store | 4 | `sp -0x70` | `[[sp-24]+12]` |
| `ldr` at 0x7264c | load | 4 | `abs 0x7278c` | `-` |
| `ldr` at 0x72650 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x72660 | load | 4 | `abs 0x72790` | `-` |
| `ldr` at 0x72664 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x72668 | load | 4 | `abs 0x72794` | `-` |
| `ldr` at 0x7266c | load | 4 | `sp -0x70` | `-` |
| `str` at 0x7267c | store | 4 | `sp -0xa4` | `0x2` |
| `ldr` at 0x72680 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x72684 | store | 4 | `sp -0xa0` | `[sp-100]` |
| `str` at 0x7268c | store | 4 | `sp -0x9c` | `sp-96` |
| `ldr` at 0x72690 | load | 4 | `abs 0x72788` | `-` |
| `ldr` at 0x72698 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x726a8 | store | 4 | `sp -0x1c` | `ret(0xb6f64)` |
| `ldr` at 0x726ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x726b8 | load | 4 | `abs 0x72790` | `-` |
| `ldr` at 0x726bc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x726c0 | load | 4 | `abs 0x72798` | `-` |
| `ldr` at 0x726cc | load | 4 | `abs 0x7278c` | `-` |
| `ldr` at 0x726d0 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x726e0 | load | 4 | `abs 0x72790` | `-` |
| `ldr` at 0x726e4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x726e8 | load | 4 | `abs 0x7279c` | `-` |
| `ldr` at 0x726ec | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x726f4 | load | 4 | `abs 0x727a0` | `-` |
| `ldr` at 0x726f8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x7270c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72718 | load | 4 | `abs 0x727a0` | `-` |
| `ldr` at 0x7271c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x72730 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7273c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72750 | load | 4 | `abs 0x727a4` | `-` |
| `ldr` at 0x72754 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x7275c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x72764 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x72764 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x72764 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x72764 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72134 | `(unknown callee) 0xb7944` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x72158 | `(unknown callee) 0x6907c` | `sp-100` | `sp-96` | `[sp-140]` | `sp-100` |
| `bl` at 0x721e4 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[sp-116]` |
| `bl` at 0x721f0 | `(unknown callee) 0x6c1f0` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x72234 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x72260 | `(unknown callee) 0x1aff8` | `0x148990` | `[sp-144]` | `?` | `0x0` |
| `bl` at 0x72278 | `(unknown callee) 0x1b508` | `0x148990` | `[sp-120]` | `?` | `ret(0x1aff8)` |
| `bl` at 0x7228c | `(unknown callee) 0xae97c` | `[sp-120]` | `0x8000` | `?` | `?` |
| `bl` at 0x722b4 | `(unknown callee) 0x751c4` | `0x1` | `?` | `?` | `[sp-124]` |
| `bl` at 0x722bc | `(unknown callee) 0x6c0d0` | `0x10f858` | `?` | `?` | `?` |
| `bl` at 0x722fc | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x7233c | `(unknown callee) 0x1b400` | `[0x1488c4]` | `[0x1488c0]` | `0x14892c` | `0x1488c4` |
| `bl` at 0x72358 | `(unknown callee) 0x1b400` | `[0x1488c8]` | `?` | `?` | `0x1488c8` |
| `bl` at 0x72378 | `(unknown callee) 0x1b124` | `[0x1488c0]` | `?` | `0x1488c0` | `0x1488c0` |
| `bl` at 0x723a4 | `(unknown callee) 0x1b124` | `?` | `?` | `[0x1488c0]` | `?` |
| `bl` at 0x723c4 | `(unknown callee) 0x6907c` | `sp-100` | `sp-96` | `0x7` | `sp-100` |
| `bl` at 0x7244c | `(unknown callee) 0x74708` | `?` | `?` | `?` | `?` |
| `bl` at 0x724a8 | `(unknown callee) 0x1b304` | `?` | `[sp-120]` | `0x70` | `?` |
| `bl` at 0x724dc | `(unknown callee) 0x690d8` | `sp-100` | `sp-96` | `[sp-124]` | `sp-100` |
| `bl` at 0x724f4 | `(unknown callee) 0x690d8` | `sp-100` | `sp-96` | `[sp-136]` | `sp-100` |
| `bl` at 0x7250c | `(unknown callee) 0x690d8` | `sp-100` | `sp-96` | `0x70` | `sp-100` |
| `bl` at 0x7252c | `(unknown callee) 0x69250` | `sp-100` | `sp-96` | `[0x1488c4]` | `[sp-136]` |
| `bl` at 0x7254c | `(unknown callee) 0x71a6c` | `sp-100` | `sp-96` | `[0x1488c8]` | `[sp-136]` |
| `bl` at 0x7256c | `(unknown callee) 0x71a6c` | `sp-100` | `sp-96` | `0x14892c` | `0x1` |
| `bl` at 0x72588 | `(unknown callee) 0x71a6c` | `sp-100` | `sp-96` | `0x148bd0` | `0x1` |
| `bl` at 0x725a4 | `(unknown callee) 0x71a6c` | `sp-100` | `sp-96` | `0x148bd0` | `0x1` |
| `bl` at 0x725c0 | `(unknown callee) 0x71a6c` | `sp-100` | `sp-96` | `0x148bd0` | `0x1` |
| `bl` at 0x725c4 | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `?` |
| `bl` at 0x725e0 | `(unknown callee) 0xb6bdc` | `0x148924` | `0x0` | `[sp-24]` | `[sp-100]` |
| `bl` at 0x725f8 | `(unknown callee) 0x6eac8` | `[sp-24]` | `0x1` | `?` | `[sp-116]` |
| `bl` at 0x72614 | `(unknown callee) 0x71a6c` | `sp-100` | `sp-96` | `0x14892c` | `0x1` |
| `bl` at 0x72630 | `(unknown callee) 0x71a6c` | `sp-100` | `sp-96` | `0x148bd0` | `0x1` |
| `bl` at 0x72670 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f870` | `[[sp-24]+12]` | `0x144dc8` |
| `bl` at 0x726a0 | `(unknown callee) 0xb6f64` | `0x148924` | `sp-32` | `?` | `0x1` |
| `bl` at 0x726c4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f890` | `?` | `0x144dc8` |
| `bl` at 0x726f0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f8c8` | `[sp-112]` | `0x144dc8` |
| `bl` at 0x72740 | `(unknown callee) 0xb686c` | `?` | `?` | `0x50` | `?` |
| `bl` at 0x72758 | `(unknown callee) 0x2070c` | `0x10f8ec` | `[sp-112]` | `?` | `ret(0xb686c)` |
| `bl` at 0x72760 | `(unknown callee) 0xb56f4` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72118 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x7216c | `[sp-116] > 0xf (signed, 32-bit)` | `[sp-116] <= 0xf (signed, 32-bit)` |
| `bge` at 0x72178 | `[sp-116] >= 0xc (signed, 32-bit)` | `[sp-116] < 0xc (signed, 32-bit)` |
| `beq` at 0x72184 | `[sp-116] == 0x6` | `[sp-116] != 0x6` |
| `bgt` at 0x72190 | `[sp-116] > 0x6 (signed, 32-bit)` | `[sp-116] <= 0x6 (signed, 32-bit)` |
| `blt` at 0x7219c | `[sp-116] < 0x1 (signed, 32-bit)` | `[sp-116] >= 0x1 (signed, 32-bit)` |
| `beq` at 0x721ac | `[sp-116] == 0x9` | `[sp-116] != 0x9` |
| `blt` at 0x721bc | `[sp-116] < 0x14 (signed, 32-bit)` | `[sp-116] >= 0x14 (signed, 32-bit)` |
| `ble` at 0x721c8 | `[sp-116] <= 0x19 (signed, 32-bit)` | `[sp-116] > 0x19 (signed, 32-bit)` |
| `beq` at 0x721d4 | `[sp-116] == 0x3e8` | `[sp-116] != 0x3e8` |
| `bls` at 0x72204 | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x72218 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7224c | `[sp-144] == 0x0` | `[sp-144] != 0x0` |
| `beq` at 0x7226c | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `beq` at 0x72298 | `ret(0xae97c) == 0x0` | `ret(0xae97c) != 0x0` |
| `beq` at 0x722ac | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `bls` at 0x722cc | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x722e0 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x72320 | `[0x1488c0] >= [0x1489b8] (signed, 32-bit)` | `[0x1488c0] < [0x1489b8] (signed, 32-bit)` |
| `beq` at 0x72330 | `[0x1488c4] == 0x0` | `[0x1488c4] != 0x0` |
| `beq` at 0x7234c | `[0x1488c8] == 0x0` | `[0x1488c8] != 0x0` |
| `bne` at 0x723d0 | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `blt` at 0x723fc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x72460 | `ret(0x74708) == 0x0` | `ret(0x74708) != 0x0` |
| `beq` at 0x7265c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x726b4 | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |
| `beq` at 0x726dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x72714 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7274c | `ret(0xb686c) == 0x0` | `ret(0xb686c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x72764 | `r0` | `?` |

## `L1CfgQuery`

- `L1CfgQuery`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72aa8 to 0x74680.
  Established. The first branch takes the zero-r1 arm and falls through when entry r1 is nonzero
  (`beq` at 0x72ac8). On a known path, a word at absolute `0x1489b8` is read (`ldr` at 0x72af0), and
  the call result `ret(0x1b124)` is stored at `[sp-24] +0x0` (`str` at 0x72b14). The computed
  selector leaves unanalysed edges (`ldrls` at 0x72f94). The recorded r0 exit value is unknown
  (`ldmdb` at 0x7467c). Output ownership and query-selector meanings are unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72ab8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x72abc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x72ac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72acc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72ae0 | load | 4 | `abs 0x739f4` | `-` |
| `ldr` at 0x72ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72aec | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72af0 | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x72b14 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x72b18 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x72b1c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x72b24 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x72b28 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72b2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72b30 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x72b40 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72b44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72b58 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72b60 | load | 4 | `(unknown)` | `-` |
| `str` at 0x72b64 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x72b68 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72b6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72b80 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72b88 | load | 4 | `(unknown)` | `-` |
| `str` at 0x72b8c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x72b90 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72b94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72ba8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72bb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72bbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72bd0 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72bd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72be8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72bf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72c08 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72c0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72c20 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72c28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72c34 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72c38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72c4c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72c54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72c58 | load | 4 | `abs 0x739fc` | `-` |
| `ldr` at 0x72c5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72c68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72c6c | load | 4 | `abs 0x73a00` | `-` |
| `ldr` at 0x72c74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72c84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72c8c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x72c90 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72c94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72ca8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72cb0 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72cb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72cc8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72cd0 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72cd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72ce8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72cf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72cf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72cf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72cfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72d04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72d14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72d1c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x72d20 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72d24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72d38 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72d40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72d4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72d50 | load | 4 | `abs 0x73a04` | `-` |
| `ldr` at 0x72d5c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72d60 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72d64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72d78 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72d80 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x72d84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72d90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72d94 | load | 4 | `abs 0x73a08` | `-` |
| `ldr` at 0x72da0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72db0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72db8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x72dbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72dcc | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72dd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72de4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72dec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72df0 | load | 4 | `abs 0x73a0c` | `-` |
| `ldr` at 0x72df4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72dfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72e0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72e14 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x72e18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72e28 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72e2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72e40 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72e48 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72e4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72e60 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72e68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72e6c | load | 4 | `(unknown)` | `-` |
| `str` at 0x72e78 | store | 4 | `sp -0x3c` | `ret(0x74708)` |
| `ldr` at 0x72e7c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72e80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72e94 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72e9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72ea8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72eac | load | 4 | `abs 0x73a10` | `-` |
| `ldr` at 0x72eb8 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72ebc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72ed0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72ed8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72ee4 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x72ee8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72efc | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x72f04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72f08 | load | 4 | `abs 0x73a14` | `-` |
| `ldr` at 0x72f0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x72f18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72f1c | load | 4 | `abs 0x73a18` | `-` |
| `ldr` at 0x72f24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72f34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72f3c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x72f40 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x72f4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72f50 | load | 4 | `abs 0x73a1c` | `-` |
| `ldr` at 0x72f5c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x72f60 | load | 4 | `[sp-60] +0x34` | `-` |
| `ldr` at 0x72f6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72f70 | load | 4 | `abs 0x73a20` | `-` |
| `ldr` at 0x72f7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72f80 | load | 4 | `abs 0x73a24` | `-` |
| `ldr` at 0x72f88 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x72f8c | load | 4 | `[sp-60] +0x38` | `-` |
| `ldrls` at 0x72f94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73014 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73018 | load | 4 | `abs 0x73a40` | `-` |
| `ldr` at 0x73020 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73024 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x73034 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73038 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x73048 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7304c | load | 4 | `abs 0x73a44` | `-` |
| `ldr` at 0x73058 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x7305c | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x7306c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73070 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x73080 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73084 | load | 4 | `abs 0x73a48` | `-` |
| `ldr` at 0x73090 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73094 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x730a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x730a8 | load | 4 | `abs 0x73a4c` | `-` |
| `ldr` at 0x730b4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x730b8 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x730c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x730cc | load | 4 | `abs 0x73a50` | `-` |
| `ldr` at 0x730d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x730dc | load | 4 | `abs 0x73a54` | `-` |
| `ldr` at 0x730e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x730e8 | load | 4 | `abs 0x73adc` | `-` |
| `ldr` at 0x730f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73100 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73108 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x7310c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x73114 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7311c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7312c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73134 | load | 4 | `abs 0x73a58` | `-` |
| `ldr` at 0x73138 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x73140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73150 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73158 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x7315c | load | 4 | `abs 0x73a5c` | `-` |
| `ldr` at 0x73160 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73164 | load | 4 | `abs 0x73a60` | `-` |
| `ldr` at 0x73168 | load | 4 | `abs 0x14865c` | `-` |
| `ldr` at 0x73170 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73180 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73188 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x7318c | load | 4 | `abs 0x73a64` | `-` |
| `ldr` at 0x73190 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73194 | load | 4 | `abs 0x73a68` | `-` |
| `ldr` at 0x73198 | load | 4 | `abs 0x148660` | `-` |
| `ldr` at 0x731a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x731b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x731b8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x731bc | load | 4 | `abs 0x73a6c` | `-` |
| `ldr` at 0x731c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x731c4 | load | 4 | `abs 0x73a70` | `-` |
| `ldr` at 0x731c8 | load | 4 | `abs 0x148664` | `-` |
| `ldr` at 0x731d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x731e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x731e8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x731f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73204 | load | 4 | `abs 0x73a74` | `-` |
| `ldr` at 0x7320c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73210 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73214 | load | 4 | `abs 0x1489b0` | `-` |
| `str` at 0x73240 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x73244 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x73248 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x7324c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73250 | load | 4 | `abs 0x73a78` | `-` |
| `ldr` at 0x73258 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73268 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73270 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73274 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73278 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x7327c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73280 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73284 | load | 4 | `abs 0x148934` | `-` |
| `ldr` at 0x73288 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x7328c | load | 4 | `abs 0x148938` | `-` |
| `ldr` at 0x73294 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73298 | load | 4 | `abs 0x73adc` | `-` |
| `ldr` at 0x732a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x732b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x732b8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `str` at 0x732c0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x732c4 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x732c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x732cc | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x732dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x732f4 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x732f8 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x732fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73308 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7330c | load | 4 | `abs 0x73a7c` | `-` |
| `ldr` at 0x73310 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73318 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73328 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73330 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73334 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73338 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73354 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x7335c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73368 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7336c | load | 4 | `abs 0x73a80` | `-` |
| `ldr` at 0x73378 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x7337c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73398 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x733a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x733ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x733b0 | load | 4 | `abs 0x73a84` | `-` |
| `ldr` at 0x733bc | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x733c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x733dc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x733e4 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x733e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73404 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x7340c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73410 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7342c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x73434 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73438 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7343c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73440 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73448 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x7344c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73468 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x73470 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7347c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73480 | load | 4 | `abs 0x73a88` | `-` |
| `ldr` at 0x7348c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73490 | load | 4 | `abs 0x73a8c` | `-` |
| `ldr` at 0x73498 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x7349c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x734b8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x734c0 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x734c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x734e0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x734e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x734ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x734f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73504 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73508 | load | 4 | `abs 0x73a8c` | `-` |
| `ldr` at 0x73510 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73520 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73528 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x7352c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73530 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7354c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x73554 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73558 | load | 4 | `abs 0x73a90` | `-` |
| `ldr` at 0x7355c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73574 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7357c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73584 | load | 4 | `abs 0x73adc` | `-` |
| `ldr` at 0x7358c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7359c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x735a4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x735a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x735b0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x735c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x735d0 | load | 4 | `abs 0x73a94` | `-` |
| `ldr` at 0x735d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x735dc | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x735e0 | load | 4 | `abs 0x1489b8` | `-` |
| `str` at 0x73600 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x73604 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x73608 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x7360c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73610 | load | 4 | `abs 0x73a78` | `-` |
| `ldr` at 0x73618 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73628 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73630 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73634 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73638 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x7363c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73640 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73644 | load | 4 | `abs 0x148934` | `-` |
| `ldr` at 0x73648 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x7364c | load | 4 | `abs 0x148938` | `-` |
| `ldr` at 0x73654 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73658 | load | 4 | `abs 0x73adc` | `-` |
| `ldr` at 0x73660 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73670 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73678 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `str` at 0x73680 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x73684 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73688 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7368c | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x7369c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x736a0 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x736a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x736b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x736b4 | load | 4 | `abs 0x73af4` | `-` |
| `ldr` at 0x736b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x736c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x736d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x736d8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `str` at 0x736e0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x736e4 | load | 4 | `abs 0x73a98` | `-` |
| `ldr` at 0x736e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x736ec | load | 4 | `abs 0x14286c` | `-` |
| `str` at 0x73700 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x73704 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7370c | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x7371c | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73720 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73734 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7373c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73740 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73750 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73754 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73768 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x73770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73774 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73784 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7378c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73790 | load | 4 | `abs 0x73a9c` | `-` |
| `ldr` at 0x73798 | load | 4 | `abs 0x73aa0` | `-` |
| `ldr` at 0x737a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x737b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x737b8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x737bc | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x737c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x737d4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x737dc | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x737e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x737f4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x737fc | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x73800 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73814 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7381c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73820 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73824 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73828 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73830 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73834 | load | 4 | `abs 0x73adc` | `-` |
| `ldr` at 0x7383c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7384c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73854 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73858 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x73860 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x73868 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x73870 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x73878 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x73880 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x73890 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73894 | load | 4 | `abs 0x73aa4` | `-` |
| `ldr` at 0x738a0 | load | 4 | `abs 0x73ab8` | `-` |
| `str` at 0x738ac | store | 4 | `abs 0x148670` | `ret(0x1b1e4)` |
| `ldr` at 0x738b0 | load | 4 | `abs 0x73ac0` | `-` |
| `str` at 0x738b8 | store | 4 | `abs 0x148678` | `0x0` |
| `ldr` at 0x738bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x738cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x738dc | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x738e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x738e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x738f4 | load | 4 | `abs 0x73ab4` | `-` |
| `ldr` at 0x738f8 | load | 4 | `abs 0x148668` | `-` |
| `str` at 0x73900 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x73904 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73908 | load | 4 | `abs 0x73ac4` | `-` |
| `ldr` at 0x73920 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73924 | load | 4 | `abs 0x73ac4` | `-` |
| `ldr` at 0x73958 | load | 4 | `abs 0x73ac8` | `-` |
| `ldr` at 0x73970 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73974 | load | 4 | `abs 0x73ac8` | `-` |
| `str` at 0x739a0 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x739a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x739a8 | load | 4 | `abs 0x73aa8` | `-` |
| `ldr` at 0x739b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x739c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x739d0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x739d4 | load | 4 | `abs 0x739f8` | `-` |
| `ldr` at 0x739d8 | load | 4 | `abs 0x14899c` | `-` |
| `ldr` at 0x739e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x739e8 | load | 4 | `abs 0x73aac` | `-` |
| `ldr` at 0x73b34 | load | 4 | `abs 0x73ab0` | `-` |
| `ldr` at 0x73b38 | load | 4 | `abs 0x73ab4` | `-` |
| `ldr` at 0x73b3c | load | 4 | `abs 0x14866c` | `-` |
| `ldr` at 0x73b40 | load | 4 | `abs 0x148668` | `-` |
| `str` at 0x73b48 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x73b58 | load | 4 | `abs 0x73ab8` | `-` |
| `ldr` at 0x73b5c | load | 4 | `abs 0x148670` | `-` |
| `str` at 0x73b64 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x73b68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73b6c | load | 4 | `abs 0x73ac4` | `-` |
| `ldr` at 0x73b84 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73b88 | load | 4 | `abs 0x73ac4` | `-` |
| `ldr` at 0x73bbc | load | 4 | `abs 0x73ac8` | `-` |
| `ldr` at 0x73bd4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73bd8 | load | 4 | `abs 0x73ac8` | `-` |
| `str` at 0x73c04 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x73c08 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x73c0c | load | 4 | `abs 0x73ac4` | `-` |
| `str` at 0x73c24 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x73c28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x73c2c | load | 4 | `abs 0x73ac4` | `-` |
| `ldr` at 0x73c60 | load | 4 | `abs 0x73ac8` | `-` |
| `str` at 0x73c78 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x73c7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x73c80 | load | 4 | `abs 0x73ac8` | `-` |
| `str` at 0x73cac | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x73cb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73cb4 | load | 4 | `abs 0x73abc` | `-` |
| `ldr` at 0x73cc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73cd4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73cdc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73cf4 | load | 4 | `abs 0x73ac0` | `-` |
| `ldr` at 0x73cf8 | load | 4 | `abs 0x148678` | `-` |
| `ldr` at 0x73d10 | load | 4 | `abs 0x73ac0` | `-` |
| `ldr` at 0x73d14 | load | 4 | `abs 0x148678` | `-` |
| `str` at 0x73d1c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x73d20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73d24 | load | 4 | `abs 0x73ac4` | `-` |
| `ldr` at 0x73d3c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73d40 | load | 4 | `abs 0x73ac4` | `-` |
| `ldr` at 0x73d74 | load | 4 | `abs 0x73ac8` | `-` |
| `ldr` at 0x73d8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73d90 | load | 4 | `abs 0x73ac8` | `-` |
| `str` at 0x73dbc | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x73dc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73dc4 | load | 4 | `abs 0x73acc` | `-` |
| `ldr` at 0x73dd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73ddc | load | 4 | `abs 0x73ad0` | `-` |
| `ldr` at 0x73de8 | load | 4 | `abs 0x73ad4` | `-` |
| `ldr` at 0x73dec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73df0 | load | 4 | `abs 0x73ad8` | `-` |
| `ldr` at 0x73df4 | load | 4 | `abs 0x142874` | `-` |
| `ldr` at 0x73dfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e00 | load | 4 | `abs 0x73adc` | `-` |
| `ldr` at 0x73e0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73e1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73e2c | store | 4 | `(unknown)` | `ret(0x1b124)` |
| `ldr` at 0x73e30 | load | 4 | `(unknown)` | `-` |
| `str` at 0x73e34 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x73e38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e3c | load | 4 | `abs 0x73ae0` | `-` |
| `ldr` at 0x73e40 | load | 4 | `abs 0x73ae4` | `-` |
| `ldr` at 0x73e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73e5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73e78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73e88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73e8c | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x73e90 | load | 4 | `abs 0x1489c4` | `-` |
| `str` at 0x73eb0 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x73eb4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x73eb8 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x73ec0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x73ec4 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x73ec8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x73ecc | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x73edc | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x73ee0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x73ef4 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x73efc | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x73f00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x73f14 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x73f1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f20 | load | 4 | `abs 0x73ae8` | `-` |
| `ldr` at 0x73f24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73f28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73f30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73f48 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73f4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x73f54 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x73f5c | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x73f60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f64 | load | 4 | `abs 0x73aec` | `-` |
| `ldr` at 0x73f68 | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x73f70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73f88 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x73f90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73fb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73fd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73fe4 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x73fec | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x73ff0 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x73ff4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73ff8 | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x74008 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x7400c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74028 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74030 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x74034 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7403c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x74040 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x74048 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x74050 | load | 4 | `abs 0x73af0` | `-` |
| `ldr` at 0x74058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7405c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x7407c | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x74080 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x74084 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x7408c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x74090 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74094 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74098 | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x740a8 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x740ac | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x740b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x740bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x740c0 | load | 4 | `abs 0x73af4` | `-` |
| `ldr` at 0x740c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x740cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x740dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x740e4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `str` at 0x740ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x740f0 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x740f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74110 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7411c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7412c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74130 | load | 4 | `abs 0x73af8` | `-` |
| `ldr` at 0x74138 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74148 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74150 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x74154 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74158 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74174 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x7417c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74190 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74198 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x7419c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x741b8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x741c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x741d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x741dc | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x741e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x741fc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74204 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74218 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74220 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74224 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74228 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7422c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74234 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74244 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7424c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x74250 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74254 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74270 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74278 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7428c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74294 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x742a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x742a4 | load | 4 | `abs 0x73afc` | `-` |
| `ldr` at 0x742b0 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x742b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x742d0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x742d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x742ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x742f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74300 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74304 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74320 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74328 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7433c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74344 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74348 | load | 4 | `abs 0x73b00` | `-` |
| `ldr` at 0x7434c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74358 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x7435c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74378 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74380 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74384 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x743a0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x743a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x743bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x743c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x743c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x743d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x743d8 | load | 4 | `abs 0x73b04` | `-` |
| `ldr` at 0x743e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x743f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x743fc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x74400 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74404 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74420 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74428 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7443c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74444 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74450 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74454 | load | 4 | `abs 0x73b08` | `-` |
| `ldr` at 0x74460 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74464 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74480 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x74488 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7449c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x744a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x744b0 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x744b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x744d0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x744d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x744ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x744f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x744f8 | load | 4 | `abs 0x73b0c` | `-` |
| `ldr` at 0x744fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74508 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7450c | load | 4 | `abs 0x73b10` | `-` |
| `ldr` at 0x74514 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74524 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7452c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b22c))` |
| `ldr` at 0x74530 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x74538 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x74540 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x74548 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x74558 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74568 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x74578 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x7457c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x74580 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x74584 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74588 | load | 4 | `abs 0x1489a0` | `-` |
| `ldr` at 0x74594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74598 | load | 4 | `abs 0x73b14` | `-` |
| `ldr` at 0x745a4 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x745a8 | load | 4 | `abs 0x1489a0` | `-` |
| `ldr` at 0x745b4 | load | 4 | `abs 0x73b18` | `-` |
| `ldr` at 0x745bc | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x745c0 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x745c4 | load | 4 | `abs 0x1489a4` | `-` |
| `ldr` at 0x745c8 | load | 4 | `abs 0x1489a8` | `-` |
| `str` at 0x745d4 | store | 4 | `sp -0x3c` | `ret(0x748c4)` |
| `ldr` at 0x745d8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x745e4 | load | 4 | `abs 0x73b28` | `-` |
| `str` at 0x745e8 | store | 4 | `sp -0x40` | `0x14892c` |
| `ldr` at 0x745ec | load | 4 | `abs 0x73b28` | `-` |
| `str` at 0x745f0 | store | 4 | `sp -0x44` | `0x14892c` |
| `ldr` at 0x745f4 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x745f8 | load | 4 | `abs 0x1489ac` | `-` |
| `str` at 0x745fc | store | 4 | `sp -0x58` | `[0x1489ac]` |
| `ldr` at 0x74600 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x74604 | load | 4 | `ret(0x748c4) +0xc` | `-` |
| `ldr` at 0x74610 | load | 4 | `abs 0x73b1c` | `-` |
| `str` at 0x74614 | store | 4 | `sp -0x48` | `0x10fc34` |
| `ldr` at 0x7461c | load | 4 | `abs 0x73b20` | `-` |
| `str` at 0x74620 | store | 4 | `sp -0x48` | `0x10fc38` |
| `ldr` at 0x74624 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x74628 | store | 4 | `sp -0x54` | `[sp-72]` |
| `ldr` at 0x7462c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74630 | load | 4 | `abs 0x73b24` | `-` |
| `ldr` at 0x74634 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x74638 | load | 4 | `[sp-64] +0x78` | `-` |
| `ldr` at 0x7463c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x74640 | load | 4 | `[sp-68] +0x7c` | `-` |
| `ldr` at 0x7464c | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74650 | load | 4 | `abs 0x73b28` | `-` |
| `ldr` at 0x74654 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74658 | load | 4 | `abs 0x73b2c` | `-` |
| `ldr` at 0x7465c | load | 4 | `abs 0x1489a4` | `-` |
| `ldr` at 0x74660 | load | 4 | `abs 0x1489a8` | `-` |
| `ldr` at 0x74670 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74674 | load | 4 | `abs 0x73b30` | `-` |
| `ldmdb` at 0x7467c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7467c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7467c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7467c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72ae4 | `(unknown callee) 0x6c0d0` | `0x10f93c` | `r1` | `r2` | `?` |
| `bl` at 0x72b0c | `(unknown callee) 0x1b124` | `?` | `?` | `[0x1489b8]` | `?` |
| `bl` at 0x72bf4 | `(unknown callee) 0x79e74` | `?` | `0x14892c` | `?` | `?` |
| `bl` at 0x72c60 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10f948` | `?` | `?` |
| `bl` at 0x72c70 | `(unknown callee) 0x1b508` | `[sp-32]` | `0x10f950` | `?` | `?` |
| `bl` at 0x72c78 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x72d00 | `(unknown callee) 0x69d3c` | `([sp-32] + ret(0x1b22c))` | `?` | `?` | `?` |
| `bl` at 0x72d08 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x72d54 | `(unknown callee) 0x1b508` | `([sp-32] + ret(0x1b22c))` | `0x10f958` | `?` | `?` |
| `bl` at 0x72d88 | `(unknown callee) 0x69e4c` | `[0x14893c]` | `?` | `?` | `?` |
| `bl` at 0x72d9c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10f960` | `ret(0x69e4c)` | `ret(0x69e4c)` |
| `bl` at 0x72da4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x72df8 | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x10f964` | `?` | `?` |
| `bl` at 0x72e00 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x72e70 | `(unknown callee) 0x74708` | `?` | `?` | `?` | `?` |
| `bl` at 0x72eb0 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f968` | `?` | `?` |
| `bl` at 0x72f10 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10f974` | `?` | `?` |
| `bl` at 0x72f20 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f980` | `?` | `?` |
| `bl` at 0x72f28 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x72f54 | `(unknown callee) 0x1b2d4` | `([sp-32] + ret(0x1b22c))` | `0x10f98c` | `ret(0x1b22c)` | `[sp-60]` |
| `bl` at 0x72f74 | `(unknown callee) 0x1b2d4` | `([sp-32] + ret(0x1b22c))` | `0x10f998` | `ret(0x1b22c)` | `[[sp-60]+52]` |
| `bl` at 0x72f84 | `(unknown callee) 0x1b2d4` | `([sp-32] + ret(0x1b22c))` | `0x10f9a0` | `ret(0x1b22c)` | `[[sp-60]+52]` |
| `bl` at 0x7301c | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f9d8` | `?` | `[[sp-60]+56]` |
| `bl` at 0x73050 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f9e0` | `?` | `?` |
| `bl` at 0x73088 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f9ec` | `?` | `?` |
| `bl` at 0x730ac | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f9f8` | `?` | `?` |
| `bl` at 0x730d0 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10fa04` | `?` | `?` |
| `bl` at 0x730e0 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10fa10` | `?` | `?` |
| `bl` at 0x730ec | `(unknown callee) 0x1b2d4` | `?` | `0x10f2ac` | `?` | `?` |
| `bl` at 0x730f4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7313c | `(unknown callee) 0x1b49c` | `?` | `0x10fa1c` | `[0x1489b8]` | `0x14892c` |
| `bl` at 0x73144 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7316c | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x10fa28` | `[0x14865c]` | `0x14865c` |
| `bl` at 0x73174 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7319c | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x10fa40` | `[0x148660]` | `0x148660` |
| `bl` at 0x731a4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x731cc | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x10fa58` | `[0x148664]` | `0x148664` |
| `bl` at 0x731d4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x731ec | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `?` |
| `bl` at 0x73208 | `(unknown callee) 0x6c0d0` | `0x10fa70` | `r1` | `r2` | `?` |
| `bl` at 0x73238 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0x73254 | `(unknown callee) 0x1b49c` | `[[sp-24]]` | `0x10fa7c` | `?` | `[[sp-24]]` |
| `bl` at 0x7325c | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73290 | `(unknown callee) 0x69d3c` | `([sp-32] + ret(0x1b22c))` | `[0x148934]` | `[0x14893c]` | `[0x148938]` |
| `bl` at 0x7329c | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f2ac` | `?` | `?` |
| `bl` at 0x732a4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x732e0 | `(unknown callee) 0x79e74` | `?` | `?` | `?` | `[0x1489b0]` |
| `bl` at 0x73300 | `(unknown callee) 0x69e4c` | `[0x14893c]` | `[sp-48]` | `?` | `0x14892c` |
| `bl` at 0x73314 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fa88` | `[sp-48]` | `ret(0x69e4c)` |
| `bl` at 0x7331c | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73370 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10fa94` | `?` | `?` |
| `bl` at 0x733b4 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10faa4` | `?` | `?` |
| `bl` at 0x73444 | `(unknown callee) 0x69d3c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73484 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10fac0` | `?` | `?` |
| `bl` at 0x73494 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10fac8` | `?` | `?` |
| `bl` at 0x734f0 | `(unknown callee) 0x69e4c` | `?` | `?` | `?` | `?` |
| `bl` at 0x73500 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `ret(0x69e4c)` | `?` | `ret(0x69e4c)` |
| `bl` at 0x7350c | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10fac8` | `?` | `?` |
| `bl` at 0x73514 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73560 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10facc` | `?` | `?` |
| `bl` at 0x73568 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73588 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f2ac` | `?` | `?` |
| `bl` at 0x73590 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x735b8 | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `[0x1489b0]` |
| `bl` at 0x735d4 | `(unknown callee) 0x6c0d0` | `0x10fad8` | `r1` | `r2` | `?` |
| `bl` at 0x735f8 | `(unknown callee) 0x1b124` | `?` | `?` | `[0x1489b8]` | `?` |
| `bl` at 0x73614 | `(unknown callee) 0x1b49c` | `[[sp-24]]` | `0x10fa7c` | `?` | `[[sp-24]]` |
| `bl` at 0x7361c | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73650 | `(unknown callee) 0x69d3c` | `([sp-32] + ret(0x1b22c))` | `[0x148934]` | `[0x14893c]` | `[0x148938]` |
| `bl` at 0x7365c | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f2ac` | `?` | `?` |
| `bl` at 0x73664 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x736a8 | `(unknown callee) 0x69e4c` | `[0x14893c]` | `?` | `?` | `0x14892c` |
| `bl` at 0x736bc | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fbcc` | `[sp-48]` | `ret(0x69e4c)` |
| `bl` at 0x736c4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7379c | `(unknown callee) 0x1b49c` | `([sp-32] + ret(0x1b22c))` | `0x10fae4` | `?` | `0x10fae8` |
| `bl` at 0x737a4 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7382c | `(unknown callee) 0x69d3c` | `([sp-32] + ret(0x1b22c))` | `?` | `?` | `?` |
| `bl` at 0x73838 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f2ac` | `?` | `?` |
| `bl` at 0x73840 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73888 | `(unknown callee) 0x6c168` | `ret(0x1b22c)` | `?` | `?` | `[0x1489b0]` |
| `bl` at 0x738a8 | `(unknown callee) 0x1b1e4` | `0x0` | `r1` | `r1` | `0x1010` |
| `bl` at 0x738d4 | `(unknown callee) 0x1b124` | `0xc8` | `?` | `?` | `?` |
| `bl` at 0x738ec | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x739b4 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10faec` | `?` | `?` |
| `bl` at 0x739bc | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x739ec | `(unknown callee) 0x1b508` | `([sp-32] + ret(0x1b22c))` | `0x10fb04` | `ret(0x1b22c)` | `[0x14899c]` |
| `bl` at 0x73b50 | `(unknown callee) 0x1b1e4` | `0x0` | `[0x14866c]` | `0x148668` | `?` |
| `bl` at 0x73cc0 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fb1c` | `?` | `?` |
| `bl` at 0x73cc8 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73ce4 | `(unknown callee) 0x1f1dc` | `0x9a` | `?` | `ret(0x1b22c)` | `([sp-32] + ret(0x1b22c))` |
| `bl` at 0x73d08 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[0x148678]` |
| `bl` at 0x73dd0 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fb48` | `?` | `?` |
| `bl` at 0x73de0 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fb60` | `?` | `[0x148678]` |
| `bl` at 0x73df8 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fb74` | `[0x142874]` | `0x142874` |
| `bl` at 0x73e04 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x10f2ac` | `?` | `?` |
| `bl` at 0x73e24 | `(unknown callee) 0x1b124` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x73e44 | `(unknown callee) 0x1b49c` | `?` | `0x10fb90` | `0x2b1628` | `?` |
| `bl` at 0x73e58 | `(unknown callee) 0x6d2f0` | `?` | `?` | `?` | `?` |
| `bl` at 0x73e70 | `(unknown callee) 0x6d374` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x73ea8 | `(unknown callee) 0x1b124` | `?` | `?` | `[0x1489c4]` | `?` |
| `bl` at 0x73f2c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fba8` | `?` | `?` |
| `bl` at 0x73f34 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73f6c | `(unknown callee) 0x1b49c` | `?` | `0x10fbb4` | `[0x1489c4]` | `0x14892c` |
| `bl` at 0x73f74 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73fa8 | `(unknown callee) 0x1d38c` | `0x9a` | `0x0` | `?` | `?` |
| `bl` at 0x73fc8 | `(unknown callee) 0x1d38c` | `0x9a` | `0x1` | `?` | `?` |
| `bl` at 0x74054 | `(unknown callee) 0x6c0d0` | `0x10fbc0` | `?` | `?` | `[0x1489b0]` |
| `bl` at 0x74074 | `(unknown callee) 0x1b124` | `?` | `?` | `[sp-60]` | `?` |
| `bl` at 0x740b4 | `(unknown callee) 0x69e4c` | `[0x14893c]` | `?` | `?` | `0x14892c` |
| `bl` at 0x740c8 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fbcc` | `[sp-48]` | `ret(0x69e4c)` |
| `bl` at 0x740d0 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74134 | `(unknown callee) 0x1b508` | `[sp-32]` | `0x10fbd8` | `[sp-36]` | `?` |
| `bl` at 0x7413c | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74230 | `(unknown callee) 0x69d3c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74238 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x742a8 | `(unknown callee) 0x1b508` | `[sp-32]` | `0x10fbe0` | `?` | `?` |
| `bl` at 0x74350 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fbe8` | `?` | `?` |
| `bl` at 0x743cc | `(unknown callee) 0x69e4c` | `?` | `?` | `?` | `?` |
| `bl` at 0x743e0 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fbf4` | `ret(0x69e4c)` | `ret(0x69e4c)` |
| `bl` at 0x743e8 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74458 | `(unknown callee) 0x1b508` | `[sp-32]` | `0x10fc00` | `?` | `?` |
| `bl` at 0x74500 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fc08` | `?` | `?` |
| `bl` at 0x74510 | `(unknown callee) 0x1b508` | `[sp-32]` | `0x10fc10` | `?` | `?` |
| `bl` at 0x74518 | `(unknown callee) 0x1b22c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74550 | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `[0x1489b0]` |
| `bl` at 0x74570 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x7459c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fc1c` | `?` | `[0x1489a0]` |
| `bl` at 0x745b8 | `(unknown callee) 0x6c0d0` | `0x10fc28` | `?` | `?` | `[0x1489a0]` |
| `bl` at 0x745cc | `(unknown callee) 0x748c4` | `[0x1489a4]` | `[0x1489a8]` | `0x14892c` | `0x14892c` |
| `bl` at 0x74644 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fc44` | `[[sp-64]+120]` | `[[sp-68]+124]` |
| `bl` at 0x74664 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fc64` | `[0x1489a4]` | `[0x1489a8]` |
| `bl` at 0x74668 | `(unknown callee) 0x6c168` | `ret(0x1b49c)` | `?` | `?` | `?` |
| `bl` at 0x74678 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x10fc7c` | `?` | `[0x1489a0]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x72ac8 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x72ad8 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x72b38 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x72bb8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x72bc8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x72c00 | `ret(0x79e74) != 0x0` | `ret(0x79e74) == 0x0` |
| `beq` at 0x72c30 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x72d48 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x72dc8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x72e24 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x72ea4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x72ee0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x72f48 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `beq` at 0x72f68 | `[[sp-60]+52] == 0x0` | `[[sp-60]+52] != 0x0` |
| `ldrls` at 0x72f94 | `[[sp-60]+56] <= 0x5 (unsigned, 32-bit)` | `[[sp-60]+56] > 0x5 (unsigned, 32-bit)` |
| `beq` at 0x73030 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x73044 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73068 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7307c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x730a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x730c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x73128 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x73200 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x732d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x732ec | `ret(0x79e74) != 0x0` | `ret(0x79e74) == 0x0` |
| `bne` at 0x73364 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x733a8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73478 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x735cc | `? == 0x0` | `? != 0x0` |
| `blt` at 0x73694 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x736f4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x73714 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x73748 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x7377c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x7389c | `r1 != 0x1010` | `r1 == 0x1010` |
| `beq` at 0x738c8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x739e0 | `[0x14899c] == 0x0` | `[0x14899c] != 0x0` |
| `beq` at 0x73cf0 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x73d00 | `[0x148678] == 0x0` | `[0x148678] != 0x0` |
| `beq` at 0x73e18 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x73e54 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x73e68 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x73e84 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x73ed4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x73f9c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x73fbc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x73fdc | `? == 0x0` | `? != 0x0` |
| `blt` at 0x74000 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x740a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x74124 | `[sp-36] < ? (signed, 32-bit)` | `[sp-36] >= ? (signed, 32-bit)` |
| `bne` at 0x7429c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x742fc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7444c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x744ac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x74564 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x74590 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x745b0 | `[0x1489a0] == 0x0` | `[0x1489a0] != 0x0` |
| `beq` at 0x745e0 | `ret(0x748c4) == 0x0` | `ret(0x748c4) != 0x0` |
| `bne` at 0x7460c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7467c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x72f94).

## `B2BInit`

- `B2BInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x76d04 to 0x778e4.
  Established. Three consecutive word stores replace absolute `0x148dd8` with `0xfff`, then `0x1`,
  then `0x0` (`str` at 0x76d5c, `str` at 0x76d68, `str` at 0x76d74). The call to `0x1b43c` carries
  `0x2b1680`, `0x0`, `0x84` and unknown as candidates (`bl` at 0x76d94); its computed selector has
  unresolved edges (`ldrls` at 0x76db4). Only the known-edge exit records `ret(0x24028)` (`ldmdb` at
  0x7785c). B2B initialization effects after that selector remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x76d14 | load | 4 | `abs 0x77860` | `-` |
| `ldr` at 0x76d18 | load | 4 | `abs 0x148dd0` | `-` |
| `ldr` at 0x76d24 | load | 4 | `abs 0x77864` | `-` |
| `str` at 0x76d28 | store | 4 | `sp -0x64` | `0x10ff48` |
| `ldr` at 0x76d30 | load | 4 | `abs 0x77868` | `-` |
| `ldr` at 0x76d40 | load | 4 | `abs 0x77860` | `-` |
| `ldr` at 0x76d44 | load | 4 | `abs 0x77860` | `-` |
| `ldr` at 0x76d48 | load | 4 | `abs 0x148dd0` | `-` |
| `str` at 0x76d50 | store | 4 | `abs 0x148dd0` | `[0x148dd0]+1` |
| `ldr` at 0x76d54 | load | 4 | `abs 0x7786c` | `-` |
| `ldr` at 0x76d58 | load | 4 | `abs 0x77870` | `-` |
| `str` at 0x76d5c | store | 4 | `abs 0x148dd8` | `0xfff` |
| `ldr` at 0x76d60 | load | 4 | `abs 0x7786c` | `-` |
| `str` at 0x76d68 | store | 4 | `abs 0x148dd8` | `0x1` |
| `ldr` at 0x76d6c | load | 4 | `abs 0x7786c` | `-` |
| `str` at 0x76d74 | store | 4 | `abs 0x148dd8` | `0x0` |
| `str` at 0x76d84 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x76d88 | load | 4 | `abs 0x77874` | `-` |
| `ldr` at 0x76d98 | load | 4 | `abs 0x77878` | `-` |
| `ldr` at 0x76da8 | load | 4 | `sp -0x34` | `-` |
| `ldrls` at 0x76db4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x77094 | load | 4 | `abs 0x7787c` | `-` |
| `ldr` at 0x77098 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x7709c | load | 4 | `abs 0x77880` | `-` |
| `ldmdb` at 0x7785c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x7785c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7785c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7785c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7785c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x76d3c | `(unknown callee) 0x21e28` | `0x9` | `0x10ff64` | `0x68` | `0x0` |
| `bl` at 0x76d78 | `(unknown callee) 0x4f090` | `?` | `?` | `0x148dd8` | `0x0` |
| `bl` at 0x76d94 | `(unknown callee) 0x1b43c` | `0x2b1680` | `0x0` | `0x84` | `?` |
| `bl` at 0x76da4 | `(unknown callee) 0x1b43c` | `0x2b169c` | `?` | `0x34` | `?` |
| `bl` at 0x770a0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10ff7c` | `?` | `0x144dc8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x76d20 | `[0x148dd0] == 0x0` | `[0x148dd0] != 0x0` |
| `ldrls` at 0x76db4 | `[sp-52]-43 <= 0x51 (unsigned, 32-bit)` | `[sp-52]-43 > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7785c | `r0` | `ret(0x24028)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x76db4).

## `B2BEventMesh`

- `B2BEventMesh`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78438 to 0x78ebc.
  Established. Word reads include `r0 +0xc` and `r0 +0x1c` (`ldr` at 0x78458, `ldr` at 0x78464). The
  signed comparison takes the branch when `[r0+28] >= 0x0` and falls through for a negative value
  (`bge` at 0x7846c). A later word store writes entry-stack expression `[sp-20]` to absolute
  `0x149afc` (`str` at 0x78724), while another clears `[sp-20] +0x18` (`str` at 0x78784). Mesh-event
  field meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78448 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x78450 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x78454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78458 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x7845c | store | 4 | `sp -0x20` | `[r0+12]` |
| `ldr` at 0x78460 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78464 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78470 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78474 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x7847c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x78480 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x78488 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x7848c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x7849c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x784a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x784a8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x784b0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x784b4 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x784c4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x784c8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x784cc | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0x784d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x784d4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x784e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x784e4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x784ec | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x784f0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x784f8 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x784fc | load | 4 | `sp -0x54` | `-` |
| `str` at 0x7850c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78514 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78518 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78520 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78524 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x78534 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78538 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x7853c | store | 4 | `sp -0x28` | `[sp-84]` |
| `ldr` at 0x78540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78544 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78554 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x7855c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78560 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x78568 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x7856c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x7857c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78584 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78588 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78590 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78594 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x785a4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x785a8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x785ac | store | 4 | `sp -0x2c` | `[sp-88]` |
| `ldr` at 0x785b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x785b4 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x785c4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x785c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x785cc | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x785d8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x785dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x785e0 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x785ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x785f0 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78604 | store | 4 | `sp -0x5c` | `0xe` |
| `str` at 0x78610 | store | 4 | `sp -0x5c` | `0xf` |
| `ldr` at 0x78618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7861c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78624 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x78628 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x78630 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x78634 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x78638 | store | 4 | `sp -0x38` | `[sp-92]` |
| `ldr` at 0x7863c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78640 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78650 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x78654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78658 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78664 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x7866c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x78670 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78674 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78688 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78698 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x786b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x786b4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x786c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x786c8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x786d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x786d8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x786f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x786f4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78708 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x7870c | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78710 | load | 4 | `abs 0x148dd8` | `-` |
| `str` at 0x78718 | store | 4 | `abs 0x148dd8` | `?` |
| `ldr` at 0x7871c | load | 4 | `abs 0x78e58` | `-` |
| `ldr` at 0x78720 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x78724 | store | 4 | `abs 0x149afc` | `[sp-20]` |
| `ldr` at 0x78728 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x7872c | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x7873c | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78740 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78744 | load | 4 | `abs 0x78e60` | `-` |
| `ldr` at 0x78748 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x7874c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x78754 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7877c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x78784 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `str` at 0x7878c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x78794 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x78798 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x7879c | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x787ac | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x787b0 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x787b4 | load | 4 | `abs 0x78e64` | `-` |
| `ldr` at 0x787b8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x787bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x787c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x787d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x787dc | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0x787e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x787e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x787f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x78800 | load | 4 | `abs 0x78e68` | `-` |
| `ldr` at 0x78804 | load | 4 | `abs 0x148de0` | `-` |
| `ldr` at 0x78808 | load | 4 | `[0x148de0] +0x58` | `-` |
| `ldr` at 0x78814 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x78820 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78824 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78834 | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78838 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x7883c | load | 4 | `abs 0x78e6c` | `-` |
| `ldr` at 0x78844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7886c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78878 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x7887c | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x7888c | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78890 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78894 | load | 4 | `abs 0x78e70` | `-` |
| `ldr` at 0x7889c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x788a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x788b8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x788d0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x788d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x788f0 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x788f4 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78904 | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78908 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7890c | store | 4 | `sp -0x7c` | `[sp-32]` |
| `ldr` at 0x78910 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78914 | load | 4 | `abs 0x78e74` | `-` |
| `ldr` at 0x78918 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7891c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x78924 | load | 4 | `abs 0x78e78` | `-` |
| `ldr` at 0x78928 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7892c | load | 4 | `abs 0x148de4` | `-` |
| `ldr` at 0x78938 | load | 4 | `abs 0x78e7c` | `-` |
| `ldr` at 0x7893c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x78940 | load | 4 | `abs 0x148de8` | `-` |
| `ldr` at 0x7894c | load | 4 | `abs 0x78e80` | `-` |
| `str` at 0x78950 | store | 4 | `sp -0x4c` | `0x2b0970` |
| `ldr` at 0x78954 | load | 4 | `abs 0x78e68` | `-` |
| `ldr` at 0x78958 | load | 4 | `abs 0x148de0` | `-` |
| `ldr` at 0x7895c | load | 4 | `[0x148de0] +0x58` | `-` |
| `ldr` at 0x78984 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78990 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78994 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x789a4 | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x789a8 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x789ac | load | 4 | `abs 0x78e84` | `-` |
| `ldr` at 0x789b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x789c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x789d0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x789e8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x789ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78a10 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x78a1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78a20 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x78a38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78a3c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x78a50 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78a54 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78a64 | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78a6c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x78a7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78a80 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x78a8c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x78a90 | store | 4 | `sp -0x7c` | `[sp-72]` |
| `ldr` at 0x78a94 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78a98 | load | 4 | `abs 0x78e88` | `-` |
| `ldr` at 0x78aa4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x78ab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78abc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78ac8 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78acc | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78adc | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78ae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78ae4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x78af4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78af8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x78b04 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78b08 | load | 4 | `abs 0x78e8c` | `-` |
| `ldr` at 0x78b10 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x78b14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x78b20 | store | 4 | `sp -0x1c` | `ret(0xb686c)` |
| `ldr` at 0x78b24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78b30 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x78b3c | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78b40 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78b5c | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78b60 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x78b64 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x78b68 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78b6c | load | 4 | `abs 0x78e90` | `-` |
| `ldr` at 0x78b7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78b88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78b94 | load | 4 | `abs 0x78e94` | `-` |
| `ldr` at 0x78b9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78ba0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78bac | store | 4 | `sp -0x18` | `ret(0x748c4)` |
| `ldr` at 0x78bb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x78bc0 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78bc4 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78bd4 | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78bd8 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78bdc | load | 4 | `abs 0x78e98` | `-` |
| `ldr` at 0x78be0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78be4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x78bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78bf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x78bfc | load | 4 | `ret(0x748c4) +0x10` | `-` |
| `str` at 0x78c00 | store | 4 | `sp -0x20` | `[ret(0x748c4)+16]` |
| `ldr` at 0x78c08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78c1c | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78c20 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78c30 | load | 4 | `abs 0x78e5c` | `-` |
| `str` at 0x78c34 | store | 4 | `sp -0x60` | `0x144dc8` |
| `ldr` at 0x78c38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78c44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x78c50 | load | 4 | `abs 0x78e9c` | `-` |
| `str` at 0x78c54 | store | 4 | `sp -0x64` | `0x110350` |
| `ldr` at 0x78c5c | load | 4 | `abs 0x78ea0` | `-` |
| `str` at 0x78c60 | store | 4 | `sp -0x64` | `0x110354` |
| `ldr` at 0x78c68 | load | 4 | `abs 0x78ea4` | `-` |
| `str` at 0x78c6c | store | 4 | `sp -0x64` | `0x110358` |
| `ldr` at 0x78c70 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x78c74 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x78c78 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78c7c | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x78c80 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x78c84 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x78c88 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x78c8c | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x78c90 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x78c94 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78c98 | load | 4 | `abs 0x78ea8` | `-` |
| `ldr` at 0x78c9c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x78ca0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x78ca8 | load | 4 | `abs 0x78eac` | `-` |
| `ldr` at 0x78cac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78cc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x78ccc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x78cd8 | store | 4 | `sp -0x1c` | `ret(0xb686c)` |
| `ldr` at 0x78cdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78ce8 | load | 4 | `abs 0x78eac` | `-` |
| `ldr` at 0x78cec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78d00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x78d0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78d18 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78d1c | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78d2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x78d38 | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78d3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x78d40 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x78d44 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78d48 | load | 4 | `abs 0x78eb0` | `-` |
| `ldr` at 0x78d58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78d64 | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78d68 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78d78 | load | 4 | `abs 0x78e5c` | `-` |
| `ldr` at 0x78d7c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x78d80 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x78d84 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78d88 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x78d8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x78d90 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x78d94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x78d98 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x78d9c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78da0 | load | 4 | `abs 0x78eb4` | `-` |
| `ldr` at 0x78da4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78da8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x78db0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78dbc | load | 4 | `abs 0x78e54` | `-` |
| `ldr` at 0x78dc0 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78dd0 | load | 4 | `abs 0x78e5c` | `-` |
| `str` at 0x78dd4 | store | 4 | `sp -0x68` | `0x144dc8` |
| `ldr` at 0x78dd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78de4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x78df0 | load | 4 | `abs 0x78e9c` | `-` |
| `str` at 0x78df4 | store | 4 | `sp -0x6c` | `0x110350` |
| `ldr` at 0x78dfc | load | 4 | `abs 0x78ea0` | `-` |
| `str` at 0x78e00 | store | 4 | `sp -0x6c` | `0x110354` |
| `ldr` at 0x78e08 | load | 4 | `abs 0x78ea4` | `-` |
| `str` at 0x78e0c | store | 4 | `sp -0x6c` | `0x110358` |
| `ldr` at 0x78e10 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x78e14 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x78e18 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78e1c | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x78e20 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x78e24 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x78e28 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x78e2c | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x78e30 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x78e34 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78e38 | load | 4 | `abs 0x78eb8` | `-` |
| `ldr` at 0x78e3c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x78e40 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x78e48 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x78e50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x78e50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x78e50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78750 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1101a0` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x78768 | `(unknown callee) 0x74c08` | `?` | `?` | `?` | `?` |
| `bl` at 0x787c0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1101c0` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x787d4 | `(unknown callee) 0x72878` | `[sp-20]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x78840 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1101ec` | `?` | `0x144dc8` |
| `bl` at 0x78848 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x78868 | `(unknown callee) 0x74ab8` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x78898 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110204` | `?` | `0x144dc8` |
| `bl` at 0x788a0 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x788ec | `(unknown callee) 0x1b304` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x78920 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110230` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x78980 | `(unknown callee) 0x74ab8` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x789b0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x11024c` | `?` | `0x144dc8` |
| `bl` at 0x789b8 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x78a04 | `(unknown callee) 0x1b304` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x78aa0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110280` | `?` | `?` |
| `bl` at 0x78ab4 | `(unknown callee) 0x729ec` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x78ac0 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x78b0c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1102b8` | `?` | `?` |
| `bl` at 0x78b18 | `(unknown callee) 0xb686c` | `[sp-76]` | `[sp-20]` | `?` | `?` |
| `bl` at 0x78b34 | `(unknown callee) 0x82878` | `[sp-76]` | `?` | `?` | `ret(0xb686c)` |
| `bl` at 0x78b54 | `(unknown callee) 0x21674` | `ret(0xb686c)` | `?` | `?` | `?` |
| `bl` at 0x78b78 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1102e0` | `[[sp-20]+22]:1` | `ret(0x21674)` |
| `bl` at 0x78b80 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x78b98 | `(unknown callee) 0x6c0d0` | `0x11030c` | `?` | `?` | `[sp-32]` |
| `bl` at 0x78ba4 | `(unknown callee) 0x748c4` | `[sp-48]` | `[sp-52]` | `?` | `?` |
| `bl` at 0x78bbc | `(unknown callee) 0x6c168` | `ret(0x748c4)` | `?` | `?` | `ret(0x748c4)` |
| `bl` at 0x78be8 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x11031c` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x78bf0 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x78c04 | `(unknown callee) 0x6c168` | `ret(0x748c4)` | `?` | `?` | `[ret(0x748c4)+16]` |
| `bl` at 0x78c0c | `(unknown callee) 0x79dcc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x78ca4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x11035c` | `?` | `[sp-56]` |
| `bl` at 0x78cd0 | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x78d10 | `(unknown callee) 0x82878` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x78d30 | `(unknown callee) 0x21674` | `ret(0xb686c)` | `?` | `?` | `?` |
| `bl` at 0x78d54 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x11038c` | `[[sp-20]+22]:1` | `ret(0x21674)` |
| `bl` at 0x78d5c | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x78dac | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1103b8` | `[sp-32]` | `[sp-56]` |
| `bl` at 0x78db4 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x78e44 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1103ec` | `?` | `[sp-56]` |
| `bl` at 0x78e4c | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7846c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x784dc | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x7854c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x785e8 | `[r0+24] >= 0x0 (signed, 32-bit)` | `[r0+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x785fc | `? != 0x2` | `? == 0x2` |
| `beq` at 0x78680 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x78690 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x786a8 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x786c0 | `? == 0x5` | `? != 0x5` |
| `bge` at 0x786d0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x786e8 | `? == 0x4` | `? != 0x4` |
| `beq` at 0x78700 | `? == 0x4` | `? != 0x4` |
| `beq` at 0x78738 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x78760 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x78774 | `ret(0x74c08) != 0x0` | `ret(0x74c08) == 0x0` |
| `beq` at 0x787a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x787cc | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0x787f0 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0x787fc | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `bne` at 0x78810 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7881c | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x78830 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x78874 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x78888 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x78900 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x78934 | `[sp-48] != [0x148de4]` | `[sp-48] == [0x148de4]` |
| `bne` at 0x78948 | `[sp-52] != [0x148de8]` | `[sp-52] == [0x148de8]` |
| `beq` at 0x78964 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x7898c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x789a0 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x78a18 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x78a30 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x78a48 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x78a60 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x78aac | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x78ad8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x78b2c | `ret(0xb686c) != 0x0` | `ret(0xb686c) == 0x0` |
| `beq` at 0x78b4c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x78b90 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x78bb8 | `ret(0x748c4) != 0x0` | `ret(0x748c4) == 0x0` |
| `beq` at 0x78bd0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x78c18 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x78c2c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x78c40 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x78c4c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x78ce4 | `ret(0xb686c) != 0x0` | `ret(0xb686c) == 0x0` |
| `beq` at 0x78d28 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x78d74 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x78dcc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x78de0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x78dec | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x78e50 | `r0` | `?` |

## `B2BEventRing`

- `B2BEventRing`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78ebc to 0x79760.
  Established. The ring-event body reads words at `r0 +0x18` and `r0 +0x1c` (`ldr` at 0x78ee8, `ldr`
  at 0x78f14); its first signed decision takes the nonnegative arm and falls through for a negative
  `[r0+28]` (`bge` at 0x78f48). A later store writes `[0x148dd4]` through an unknown effective
  address (`str` at 0x78ff4). The call to `0xb3a34` has candidate r1 `0x1` and three unknown
  candidates (`bl` at 0x7904c). The destination of that store and the protocol role are unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78ecc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x78ed4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x78ed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78edc | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x78ee0 | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x78ee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78ee8 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78ef8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x78efc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f00 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78f0c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x78f10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f14 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78f24 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x78f28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f2c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78f38 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x78f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f40 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f50 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78f58 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x78f5c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x78f64 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x78f68 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x78f78 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x78f80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f84 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78f8c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x78f90 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x78fa0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x78fa4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x78fa8 | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x78fac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x78fb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x78fc4 | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x78fc8 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x78fd8 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x78fdc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x78fe0 | load | 4 | `abs 0x7972c` | `-` |
| `ldr` at 0x78fe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78fec | load | 4 | `abs 0x79730` | `-` |
| `ldr` at 0x78ff0 | load | 4 | `abs 0x148dd4` | `-` |
| `str` at 0x78ff4 | store | 4 | `(unknown)` | `[0x148dd4]` |
| `ldr` at 0x78ffc | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x79000 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79010 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x79014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79018 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x79028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7902c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x79038 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x7903c | load | 4 | `abs 0x79734` | `-` |
| `ldr` at 0x79044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79050 | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x79054 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7905c | load | 4 | `abs 0x7973c` | `-` |
| `ldr` at 0x79060 | load | 4 | `abs 0x148de0` | `-` |
| `ldr` at 0x79064 | load | 4 | `[0x148de0] +0x58` | `-` |
| `ldr` at 0x79070 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7907c | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x79080 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79090 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x79094 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x79098 | load | 4 | `abs 0x79740` | `-` |
| `ldr` at 0x790a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x790ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x790bc | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x790c0 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x790d0 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x790d4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x790d8 | load | 4 | `abs 0x79744` | `-` |
| `ldr` at 0x790dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x790e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x790f4 | store | 4 | `sp -0x44` | `0x2` |
| `str` at 0x79100 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x79104 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x79108 | store | 4 | `sp -0x1c` | `[sp-68]` |
| `ldr` at 0x7910c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79118 | store | 4 | `sp -0x38` | `ret(0x74b30)` |
| `ldr` at 0x7911c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79130 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79134 | load | 4 | `[sp-56] +0x4` | `-` |
| `ldr` at 0x79140 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79144 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79150 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79154 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79160 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79164 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79170 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79174 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79180 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79184 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79190 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79194 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x791a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x791a4 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x791b0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x791b4 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x791c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x791c4 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x791d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x791d4 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x791e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x791e4 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x791f0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x791f4 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79200 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79204 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79210 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79214 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79224 | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x79228 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7923c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79248 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79254 | store | 4 | `sp -0x20` | `ret(0xb686c)` |
| `ldr` at 0x79258 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79274 | load | 4 | `abs 0x79748` | `-` |
| `ldr` at 0x79278 | load | 4 | `abs 0x148658` | `-` |
| `ldr` at 0x79284 | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x79288 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79298 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x7929c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x792a0 | load | 4 | `abs 0x7974c` | `-` |
| `ldr` at 0x792a8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x792b8 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x792bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x792cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x792e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x792ec | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x792f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79304 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79324 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79330 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79338 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x79340 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79350 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x79354 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79364 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79378 | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x7937c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79390 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7939c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x793b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x793bc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x793c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x793d4 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x793d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x793e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x793fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79408 | store | 4 | `sp -0x38` | `ret(0x74b30)` |
| `ldr` at 0x7940c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79410 | load | 4 | `ret(0x74b30) +0x4` | `-` |
| `ldr` at 0x7941c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79420 | load | 4 | `ret(0x74b30) +0x78` | `-` |
| `ldr` at 0x7942c | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x79430 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79440 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x79444 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x79448 | load | 4 | `abs 0x79750` | `-` |
| `ldr` at 0x7944c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79454 | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x79458 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7946c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79478 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79484 | store | 4 | `sp -0x20` | `ret(0xb686c)` |
| `ldr` at 0x79488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79498 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x794a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x794ac | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x794b8 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x794bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x794cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x794e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x794ec | store | 4 | `sp -0x38` | `ret(0x74b30)` |
| `ldr` at 0x794f0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x794f4 | load | 4 | `ret(0x74b30) +0x4` | `-` |
| `ldr` at 0x79500 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79504 | load | 4 | `ret(0x74b30) +0x10` | `-` |
| `ldr` at 0x79510 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79514 | load | 4 | `ret(0x74b30) +0x10` | `-` |
| `ldr` at 0x79520 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79524 | load | 4 | `ret(0x74b30) +0x10` | `-` |
| `ldr` at 0x79530 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79534 | load | 4 | `ret(0x74b30) +0x10` | `-` |
| `ldr` at 0x79544 | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x79548 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79558 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x7955c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x79560 | load | 4 | `abs 0x79754` | `-` |
| `ldr` at 0x79564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7956c | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x79570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79584 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79590 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7959c | store | 4 | `sp -0x20` | `ret(0xb686c)` |
| `ldr` at 0x795a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x795b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x795bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x795c4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x795cc | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x795d0 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x795e0 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x795e4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x795e8 | load | 4 | `abs 0x79758` | `-` |
| `str` at 0x795f4 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x795f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79620 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79638 | store | 4 | `sp -0x38` | `ret(0x74b30)` |
| `str` at 0x79648 | store | 4 | `sp -0x3c` | `ret(0x74b30)` |
| `ldr` at 0x7964c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x79650 | load | 4 | `ret(0x74b30) +0x4` | `-` |
| `ldr` at 0x7965c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79660 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x79664 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0x79668 | load | 4 | `ret(0x74b30) +0x8` | `-` |
| `ldr` at 0x79674 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79678 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x7967c | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0x79680 | load | 4 | `ret(0x74b30) +0xc` | `-` |
| `ldr` at 0x79690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7969c | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x796a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x796b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x796c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x796d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x796e0 | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x796e4 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x796f4 | load | 4 | `abs 0x79728` | `-` |
| `ldr` at 0x796f8 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x796fc | load | 4 | `abs 0x7975c` | `-` |
| `ldr` at 0x79700 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79708 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79710 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x79718 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x79720 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x79720 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x79720 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78fe4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x11041c` | `r2` | `0x144dc8` |
| `bl` at 0x79040 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110434` | `?` | `?` |
| `bl` at 0x7904c | `(unknown callee) 0xb3a34` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x79058 | `(unknown callee) 0xb6798` | `0x2b1680` | `[sp-20]` | `?` | `?` |
| `bl` at 0x7909c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1101ec` | `?` | `0x144dc8` |
| `bl` at 0x790a4 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x790e0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110458` | `[sp-28]` | `0x144dc8` |
| `bl` at 0x79110 | `(unknown callee) 0x74b30` | `[sp-68]` | `?` | `?` | `[sp-68]` |
| `bl` at 0x79120 | `(unknown callee) 0x79dcc` | `[sp-28]` | `?` | `?` | `ret(0x74b30)` |
| `bl` at 0x7924c | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x7926c | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x792a4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110474` | `?` | `0x144dc8` |
| `bl` at 0x792d0 | `(unknown callee) 0x79dcc` | `?` | `?` | `?` | `?` |
| `bl` at 0x792e8 | `(unknown callee) 0xb3a34` | `[sp-20]` | `0x1` | `?` | `ret(0x79dcc)` |
| `bl` at 0x79314 | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x7932c | `(unknown callee) 0xb3a34` | `[sp-20]` | `?` | `?` | `ret(0xb686c)` |
| `bl` at 0x79344 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79368 | `(unknown callee) 0x79dcc` | `?` | `?` | `?` | `?` |
| `bl` at 0x793a0 | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x793c8 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x793ec | `(unknown callee) 0x79dcc` | `?` | `?` | `?` | `?` |
| `bl` at 0x79400 | `(unknown callee) 0x74b30` | `[sp-28]` | `?` | `?` | `ret(0x79dcc)` |
| `bl` at 0x79450 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1104a0` | `[sp-28]` | `0x144dc8` |
| `bl` at 0x7947c | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x7949c | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `ret(0xb686c)` |
| `bl` at 0x794d0 | `(unknown callee) 0x79dcc` | `?` | `?` | `?` | `?` |
| `bl` at 0x794e4 | `(unknown callee) 0x74b30` | `[sp-28]` | `?` | `?` | `ret(0x79dcc)` |
| `bl` at 0x79568 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1104c4` | `[sp-28]` | `0x144dc8` |
| `bl` at 0x79594 | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x795b4 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `ret(0xb686c)` |
| `bl` at 0x795ec | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1104e8` | `?` | `0x144dc8` |
| `bl` at 0x7960c | `(unknown callee) 0x79dcc` | `?` | `?` | `?` | `?` |
| `bl` at 0x79630 | `(unknown callee) 0x74b30` | `0x1` | `?` | `?` | `[sp-28]` |
| `bl` at 0x79640 | `(unknown callee) 0x74b30` | `0x2` | `?` | `?` | `ret(0x74b30)` |
| `bl` at 0x79698 | `(unknown callee) 0xb3a34` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x796c4 | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x796dc | `(unknown callee) 0xb3a34` | `[sp-20]` | `?` | `?` | `ret(0xb686c)` |
| `bl` at 0x79704 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110504` | `[sp-28]` | `0x144dc8` |
| `bl` at 0x7971c | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x78f48 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x78fb4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x78fc0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x78fd4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7900c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7906c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x79078 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x7908c | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x790b8 | `[sp-28]-1 > 0x1 (unsigned, 32-bit)` | `[sp-28]-1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x790cc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x790ec | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `beq` at 0x7912c | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x7913c | `[[sp-56]+4] == 0x0` | `[[sp-56]+4] != 0x0` |
| `beq` at 0x7914c | `[[sp-56]+16] == 0x35` | `[[sp-56]+16] != 0x35` |
| `beq` at 0x7915c | `[[sp-56]+16] == 0x4f` | `[[sp-56]+16] != 0x4f` |
| `beq` at 0x7916c | `[[sp-56]+16] == 0x42` | `[[sp-56]+16] != 0x42` |
| `beq` at 0x7917c | `[[sp-56]+16] == 0x46` | `[[sp-56]+16] != 0x46` |
| `beq` at 0x7918c | `[[sp-56]+16] == 0x48` | `[[sp-56]+16] != 0x48` |
| `beq` at 0x7919c | `[[sp-56]+16] == 0x4a` | `[[sp-56]+16] != 0x4a` |
| `beq` at 0x791ac | `[[sp-56]+16] == 0x4b` | `[[sp-56]+16] != 0x4b` |
| `beq` at 0x791bc | `[[sp-56]+16] == 0x21` | `[[sp-56]+16] != 0x21` |
| `beq` at 0x791cc | `[[sp-56]+16] == 0x5e` | `[[sp-56]+16] != 0x5e` |
| `beq` at 0x791dc | `[[sp-56]+16] == 0x2b` | `[[sp-56]+16] != 0x2b` |
| `beq` at 0x791ec | `[[sp-56]+16] == 0x4c` | `[[sp-56]+16] != 0x4c` |
| `beq` at 0x791fc | `[[sp-56]+16] == 0x5a` | `[[sp-56]+16] != 0x5a` |
| `beq` at 0x7920c | `[[sp-56]+16] == 0x7c` | `[[sp-56]+16] != 0x7c` |
| `beq` at 0x7921c | `[[sp-56]+16] == 0x4d` | `[[sp-56]+16] != 0x4d` |
| `bne` at 0x79260 | `ret(0xb686c) != 0x0` | `ret(0xb686c) == 0x0` |
| `ble` at 0x79280 | `[0x148658] <= 0x0 (signed, 32-bit)` | `[0x148658] > 0x0 (signed, 32-bit)` |
| `beq` at 0x79294 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x792b0 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `ble` at 0x792c4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x792dc | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x79320 | `ret(0xb686c) == 0x0` | `ret(0xb686c) != 0x0` |
| `ble` at 0x7935c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79374 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `bne` at 0x793ac | `ret(0xb686c) != 0x0` | `ret(0xb686c) == 0x0` |
| `ble` at 0x793e0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x793f8 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x79418 | `[ret(0x74b30)+4] == 0x0` | `[ret(0x74b30)+4] != 0x0` |
| `bne` at 0x79428 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x7943c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79490 | `ret(0xb686c) != 0x0` | `ret(0xb686c) == 0x0` |
| `ble` at 0x794c4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x794dc | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `beq` at 0x794fc | `[ret(0x74b30)+4] == 0x0` | `[ret(0x74b30)+4] != 0x0` |
| `beq` at 0x7950c | `[ret(0x74b30)+16] == 0x52` | `[ret(0x74b30)+16] != 0x52` |
| `beq` at 0x7951c | `[ret(0x74b30)+16] == 0x44` | `[ret(0x74b30)+16] != 0x44` |
| `beq` at 0x7952c | `[ret(0x74b30)+16] == 0x36` | `[ret(0x74b30)+16] != 0x36` |
| `beq` at 0x7953c | `[ret(0x74b30)+16] == 0x40` | `[ret(0x74b30)+16] != 0x40` |
| `beq` at 0x79554 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x795a8 | `ret(0xb686c) != 0x0` | `ret(0xb686c) == 0x0` |
| `beq` at 0x795dc | `? == 0x0` | `? != 0x0` |
| `ble` at 0x79600 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x79618 | `ret(0x79dcc) != 0x0` | `ret(0x79dcc) == 0x0` |
| `bne` at 0x79628 | `[sp-28] != 0x2` | `[sp-28] == 0x2` |
| `beq` at 0x79658 | `[ret(0x74b30)+4] == 0x0` | `[ret(0x74b30)+4] != 0x0` |
| `bne` at 0x79670 | `[[sp-56]+8] != [ret(0x74b30)+8]` | `[[sp-56]+8] == [ret(0x74b30)+8]` |
| `bne` at 0x79688 | `[[sp-56]+12] != [ret(0x74b30)+12]` | `[[sp-56]+12] == [ret(0x74b30)+12]` |
| `beq` at 0x796d0 | `ret(0xb686c) == 0x0` | `ret(0xb686c) != 0x0` |
| `beq` at 0x796f0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x79720 | `r0` | `?` |

## `B2BReqRsp`

- `B2BReqRsp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79760 to 0x79dcc.
  Established. A word read uses `[sp-20] +0x18` (`ldr` at 0x79780). The first signed branch is taken
  for `[[sp-20]+24] >= 0x0`, with negative values falling through (`bge` at 0x797b4). Known edges
  include a call to `0x74758` with `[sp-36]`, `[sp-40]` and two unknown candidates (`bl` at
  0x79b5c). The computed transfer has unresolved edges (`ldrls` at 0x7985c), so request/response
  dispatch coverage is incomplete.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79770 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x79778 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7977c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79780 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x79790 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x79794 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79798 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x797a4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x797a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797ac | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x797b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797bc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x797c4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x797c8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x797d0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x797d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797dc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x797e4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x797e8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x797ec | store | 4 | `sp -0x2c` | `[sp-56]` |
| `ldr` at 0x797f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797f4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x79800 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79804 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x79818 | store | 4 | `sp -0x3c` | `0xe` |
| `str` at 0x79824 | store | 4 | `sp -0x3c` | `0xf` |
| `ldr` at 0x7982c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79830 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x79838 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x7983c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x79844 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x79848 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x7984c | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0x79850 | load | 4 | `sp -0x30` | `-` |
| `ldrls` at 0x7985c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79b10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79b1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x79b28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79b34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x79b40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79b4c | load | 4 | `abs 0x79da0` | `-` |
| `ldr` at 0x79b54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x79b58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79b70 | store | 4 | `sp -0x34` | `ret(0x74850)` |
| `ldr` at 0x79b74 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x79b80 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x79b84 | load | 4 | `ret(0x74850) +0x10` | `-` |
| `str` at 0x79b88 | store | 4 | `sp -0x20` | `[ret(0x74850)+16]` |
| `ldr` at 0x79b8c | load | 4 | `abs 0x79d94` | `-` |
| `ldr` at 0x79b90 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79ba0 | load | 4 | `abs 0x79d98` | `-` |
| `ldr` at 0x79ba4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x79ba8 | store | 4 | `sp -0x48` | `[ret(0x74850)+16]` |
| `ldr` at 0x79bac | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x79bb0 | load | 4 | `abs 0x79da4` | `-` |
| `ldr` at 0x79bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x79bb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79bc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79bd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79bdc | store | 4 | `sp -0x20` | `ret(0x79ebc)` |
| `ldr` at 0x79be0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79bf0 | load | 4 | `abs 0x79d94` | `-` |
| `ldr` at 0x79bf4 | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79c04 | load | 4 | `abs 0x79d98` | `-` |
| `ldr` at 0x79c08 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x79c0c | store | 4 | `sp -0x48` | `ret(0x79ebc)` |
| `ldr` at 0x79c10 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x79c14 | load | 4 | `abs 0x79da8` | `-` |
| `ldr` at 0x79c18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x79c1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79c24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79c38 | load | 4 | `abs 0x79d94` | `-` |
| `ldr` at 0x79c3c | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79c4c | load | 4 | `abs 0x79d98` | `-` |
| `ldr` at 0x79c50 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x79c54 | load | 4 | `abs 0x79dac` | `-` |
| `ldr` at 0x79c58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79c60 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79c64 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x79c74 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x79c78 | load | 4 | `abs 0x79db0` | `-` |
| `ldr` at 0x79c7c | load | 4 | `abs 0x79db4` | `-` |
| `ldr` at 0x79c80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79c98 | load | 4 | `abs 0x79db0` | `-` |
| `ldr` at 0x79c9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79cb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79cbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79cc8 | store | 4 | `sp -0x18` | `ret(0xb686c)` |
| `ldr` at 0x79ccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79cd8 | load | 4 | `abs 0x79d94` | `-` |
| `ldr` at 0x79cdc | load | 4 | `abs 0x148dd8` | `-` |
| `ldr` at 0x79cec | load | 4 | `abs 0x79d98` | `-` |
| `ldr` at 0x79cf0 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x79cf4 | load | 4 | `abs 0x79db8` | `-` |
| `ldr` at 0x79cf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79cfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79d04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79d08 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x79d18 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x79d1c | load | 4 | `abs 0x79db0` | `-` |
| `ldr` at 0x79d20 | load | 4 | `abs 0x79dbc` | `-` |
| `ldr` at 0x79d24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79d30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x79d38 | store | 4 | `sp -0x40` | `ret(0x21674)` |
| `ldr` at 0x79d3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x79d40 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x79d4c | load | 4 | `abs 0x79dc0` | `-` |
| `str` at 0x79d50 | store | 4 | `sp -0x44` | `0x110604` |
| `ldr` at 0x79d58 | load | 4 | `abs 0x79dc4` | `-` |
| `str` at 0x79d5c | store | 4 | `sp -0x44` | `0x110608` |
| `ldr` at 0x79d60 | load | 4 | `abs 0x79dc8` | `-` |
| `ldr` at 0x79d64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79d68 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x79d6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x79d74 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x79d7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x79d7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x79d7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79b50 | `(unknown callee) 0x6c0d0` | `0x110554` | `r1` | `r2` | `[sp-40]` |
| `bl` at 0x79b5c | `(unknown callee) 0x74758` | `[sp-36]` | `[sp-40]` | `?` | `?` |
| `bl` at 0x79b68 | `(unknown callee) 0x74850` | `ret(0x74758)` | `?` | `?` | `ret(0x74758)` |
| `bl` at 0x79bbc | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110560` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x79bc0 | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `?` |
| `bl` at 0x79bd4 | `(unknown callee) 0x79ebc` | `[sp-20]` | `?` | `?` | `[sp-32]` |
| `bl` at 0x79c20 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x110584` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x79c28 | `(unknown callee) 0x79dcc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x79c5c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1105c0` | `[sp-32]` | `0x144dc8` |
| `bl` at 0x79c88 | `(unknown callee) 0xb6bdc` | `0x2b1680` | `0xfffffda0` | `[sp-20]` | `0x0` |
| `bl` at 0x79c90 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79cc0 | `(unknown callee) 0xb686c` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x79d00 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1105dc` | `[sp-32]` | `ret(0xb686c)` |
| `bl` at 0x79d2c | `(unknown callee) 0xb6bdc` | `0x2b1680` | `0xfffffd9e` | `[sp-20]` | `0x0` |
| `bl` at 0x79d34 | `(unknown callee) 0x21674` | `?` | `?` | `?` | `?` |
| `bl` at 0x79d70 | `(unknown callee) 0x2070c` | `0x11060c` | `[sp-32]` | `[sp-68]` | `ret(0x21674)` |
| `bl` at 0x79d78 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x797b4 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bge` at 0x797fc | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x79810 | `? != 0x2` | `? == 0x2` |
| `ldrls` at 0x7985c | `[sp-60]-1 <= 0xe (unsigned, 32-bit)` | `[sp-60]-1 > 0xe (unsigned, 32-bit)` |
| `bne` at 0x79b18 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x79b24 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x79b30 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x79b3c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x79b48 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x79b7c | `ret(0x74850) == 0x0` | `ret(0x74850) != 0x0` |
| `beq` at 0x79b9c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79bcc | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0x79be8 | `ret(0x79ebc) > 0x0 (signed, 32-bit)` | `ret(0x79ebc) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x79c00 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79c34 | `ret(0x79dcc) != 0x0` | `ret(0x79dcc) == 0x0` |
| `beq` at 0x79c48 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79c6c | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `beq` at 0x79cd4 | `ret(0xb686c) == 0x0` | `ret(0xb686c) != 0x0` |
| `beq` at 0x79ce8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79d10 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `bne` at 0x79d48 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x79d7c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7985c).

## `PktGenMemOp`

- `PktGenMemOp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x610e8 to 0x621d0.
  Established. Entry r0 equal to `0x1` takes the initial branch (`beq` at 0x6113c). Other values fall through to a word
  store of `0xfffff377` at `sp -0xc0` (`beq` at 0x6113c, `str` at 0x61144). An indirect call has
  candidates `[sp-28]`, `[[sp-124]]`, `sp-76` and `sp-80`, with unknown target (`mov` at 0x61370).
  Later stores include halfword `0x123` at absolute `0x148468` and word `[sp-180]` at absolute
  `0x142b64` (`strh` at 0x61708, `str` at 0x61fdc). Their device interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x610f8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x610fc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x61100 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x61104 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x6110c | store | 4 | `sp -0x7c` | `0x0` |
| `str` at 0x61114 | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x6111c | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x61124 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x61128 | load | 4 | `abs 0x62044` | `-` |
| `ldr` at 0x6112c | load | 4 | `abs 0x142b64` | `-` |
| `str` at 0x61130 | store | 4 | `sp -0xb4` | `[0x142b64]` |
| `ldr` at 0x61134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61140 | load | 4 | `abs 0x62048` | `-` |
| `str` at 0x61144 | store | 4 | `sp -0xc0` | `0xfffff377` |
| `ldr` at 0x6114c | load | 4 | `abs 0x6204c` | `-` |
| `str` at 0x61150 | store | 4 | `sp -0x2c` | `0xea468` |
| `ldr` at 0x61154 | load | 4 | `abs 0x62050` | `-` |
| `str` at 0x61158 | store | 4 | `sp -0x28` | `0xea56c` |
| `ldr` at 0x6115c | load | 4 | `abs 0x62054` | `-` |
| `ldr` at 0x61160 | load | 4 | `abs 0x148448` | `-` |
| `ldr` at 0x6116c | load | 4 | `abs 0x62058` | `-` |
| `ldr` at 0x61170 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x61174 | store | 4 | `abs 0x2b0940` | `0xea468` |
| `ldr` at 0x61178 | load | 4 | `abs 0x6205c` | `-` |
| `ldr` at 0x6117c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x61180 | store | 4 | `abs 0x2b0920` | `[sp-40]` |
| `ldr` at 0x61184 | load | 4 | `abs 0x62060` | `-` |
| `str` at 0x61188 | store | 4 | `sp -0x2c` | `0x60ff0` |
| `ldr` at 0x6118c | load | 4 | `abs 0x62064` | `-` |
| `str` at 0x61190 | store | 4 | `sp -0x28` | `0x61070` |
| `ldr` at 0x61194 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x611a0 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x611a4 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x611a8 | store | 4 | `sp -0xa8` | `[sp-196]` |
| `ldr` at 0x611ac | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x611b8 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x611c4 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x611d4 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x611e0 | load | 4 | `abs 0x62068` | `-` |
| `ldr` at 0x611e8 | load | 4 | `abs 0x62048` | `-` |
| `str` at 0x611ec | store | 4 | `sp -0xc0` | `0xfffff377` |
| `ldr` at 0x611f4 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x611f8 | store | 4 | `sp -0xc8` | `[sp-196]` |
| `ldr` at 0x611fc | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61208 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61214 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61224 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x61234 | load | 4 | `abs 0x6206c` | `-` |
| `str` at 0x61238 | store | 4 | `sp -0x7c` | `0x10c79c` |
| `str` at 0x61240 | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x61244 | load | 4 | `abs 0x62070` | `-` |
| `str` at 0x61248 | store | 4 | `sp -0x84` | `0x10c7f4` |
| `str` at 0x61250 | store | 4 | `sp -0x88` | `0xb` |
| `ldr` at 0x61254 | load | 4 | `abs 0x62074` | `-` |
| `str` at 0x61258 | store | 4 | `sp -0x8c` | `0x10c820` |
| `str` at 0x61260 | store | 4 | `sp -0x90` | `0x6` |
| `ldr` at 0x61264 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x6126c | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x61274 | load | 4 | `abs 0x62078` | `-` |
| `str` at 0x61278 | store | 4 | `sp -0x7c` | `0x10c700` |
| `str` at 0x61280 | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x61284 | load | 4 | `abs 0x6207c` | `-` |
| `str` at 0x61288 | store | 4 | `sp -0x84` | `0x10c758` |
| `str` at 0x61290 | store | 4 | `sp -0x88` | `0xb` |
| `str` at 0x61298 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x612a0 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x612a4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x612ac | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x612b4 | load | 4 | `abs 0x62080` | `-` |
| `str` at 0x612b8 | store | 4 | `sp -0x7c` | `0x10c838` |
| `str` at 0x612c0 | store | 4 | `sp -0x80` | `0x16` |
| `ldr` at 0x612c4 | load | 4 | `abs 0x62084` | `-` |
| `str` at 0x612c8 | store | 4 | `sp -0x84` | `0x10c890` |
| `str` at 0x612d0 | store | 4 | `sp -0x88` | `0xb` |
| `ldr` at 0x612d4 | load | 4 | `abs 0x62088` | `-` |
| `str` at 0x612d8 | store | 4 | `sp -0x8c` | `0x10c8bc` |
| `str` at 0x612e0 | store | 4 | `sp -0x90` | `0x6` |
| `ldr` at 0x612e4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x612ec | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x612f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x612fc | store | 4 | `sp -0x30` | `ret(0xeb27c)` |
| `ldr` at 0x61300 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6130c | load | 4 | `abs 0x6208c` | `-` |
| `ldr` at 0x61310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61318 | load | 4 | `abs 0x62090` | `-` |
| `str` at 0x6131c | store | 4 | `sp -0xc0` | `0xfffff375` |
| `ldr` at 0x61324 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x61338 | load | 4 | `abs 0x62094` | `-` |
| `ldr` at 0x6133c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61344 | load | 4 | `abs 0x62098` | `-` |
| `str` at 0x61348 | store | 4 | `sp -0xc0` | `0xfffff374` |
| `ldr` at 0x61350 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x6135c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61364 | load | 4 | `[sp-124] +0x0` | `-` |
| `ldr` at 0x61380 | load | 4 | `abs 0x6209c` | `-` |
| `str` at 0x6138c | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x61394 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x613a0 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x613a8 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x613b0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x613c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x613c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x613c8 | load | 4 | `[sp-132] +0x20` | `-` |
| `str` at 0x613e8 | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x613f0 | load | 4 | `abs 0x62044` | `-` |
| `str` at 0x613f8 | store | 4 | `abs 0x142b64` | `0x0` |
| `str` at 0x61400 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x61408 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x61410 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61418 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6141c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61420 | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x61424 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61428 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61440 | load | 4 | `abs 0x620a0` | `-` |
| `ldr` at 0x61448 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x6144c | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61454 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61458 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6145c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61460 | load | 4 | `[sp-132] +0x0` | `-` |
| `ldr` at 0x61464 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61468 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61480 | load | 4 | `abs 0x620a4` | `-` |
| `ldr` at 0x61488 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x6148c | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61498 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x614a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x614a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x614a8 | load | 4 | `[sp-132] +0x1c` | `-` |
| `ldr` at 0x614ac | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x614b0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x614c8 | load | 4 | `abs 0x620a8` | `-` |
| `ldr` at 0x614d0 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x614d4 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x614e0 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x614e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x614ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x614f0 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x614f4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x614f8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61510 | load | 4 | `abs 0x620ac` | `-` |
| `ldr` at 0x61518 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x6151c | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61524 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x61530 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x61534 | store | 4 | `sp -0x54` | `[sp-68]` |
| `ldr` at 0x61538 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x61540 | store | 4 | `sp -0x58` | `?` |
| `str` at 0x61548 | store | 4 | `sp -0x54` | `?` |
| `str` at 0x61550 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61558 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x61568 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0x6156c | load | 4 | `abs 0x620b0` | `-` |
| `str` at 0x61570 | store | 4 | `sp -0x58` | `0x7ffffd` |
| `str` at 0x6157c | store | 4 | `sp -0x54` | `?` |
| `str` at 0x61584 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6158c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61594 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6159c | load | 4 | `[sp-132] +0x20` | `-` |
| `ldr` at 0x615a0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x615a4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x615bc | load | 4 | `abs 0x620b4` | `-` |
| `ldr` at 0x615c4 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x615c8 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x615d4 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x615d8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x615dc | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x615ec | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x615f4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x61604 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6160c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61628 | load | 4 | `abs 0x620b8` | `-` |
| `str` at 0x61634 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x6163c | load | 4 | `abs 0x62128` | `-` |
| `ldr` at 0x61640 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61644 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x61648 | store | 4 | `(unknown)` | `[sp-84]` |
| `ldr` at 0x6164c | load | 4 | `abs 0x62128` | `-` |
| `ldr` at 0x61650 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61664 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61668 | store | 4 | `(unknown)` | `[sp-88]` |
| `ldr` at 0x6166c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61670 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x61678 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6167c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x61684 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6168c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61690 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61694 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61698 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6169c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x616b4 | load | 4 | `abs 0x620bc` | `-` |
| `ldr` at 0x616bc | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x616c0 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x616c8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x616d0 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x616d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x616e4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x616e8 | store | 4 | `sp -0x20` | `[sp-52]` |
| `ldr` at 0x616ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x616fc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x61700 | load | 4 | `abs 0x620c0` | `-` |
| `ldr` at 0x61704 | load | 4 | `abs 0x620c4` | `-` |
| `strh` at 0x61708 | store | 2 | `abs 0x148468` | `0x123` |
| `ldr` at 0x6170c | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x61710 | load | 1 | `sp -0x38` | `-` |
| `strb` at 0x61714 | store | 1 | `abs 0x148460` | `([sp-56]:1 & 0xff)` |
| `ldr` at 0x61718 | load | 4 | `abs 0x620c0` | `-` |
| `ldrh` at 0x6171c | load | 2 | `sp -0x34` | `-` |
| `strh` at 0x61720 | store | 2 | `abs 0x14845e` | `([sp-52]:2 & 0xffff)` |
| `ldr` at 0x61724 | load | 4 | `abs 0x620c0` | `-` |
| `ldrh` at 0x61728 | load | 2 | `sp +0x0` | `-` |
| `strh` at 0x61730 | store | 2 | `abs 0x14845c` | `?` |
| `ldr` at 0x61734 | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x61738 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x6173c | store | 1 | `abs 0x148457` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x61740 | load | 4 | `abs 0x620c0` | `-` |
| `ldrh` at 0x61744 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x61748 | store | 2 | `abs 0x148454` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0x6174c | load | 4 | `abs 0x620c0` | `-` |
| `str` at 0x61750 | store | 4 | `sp -0xcc` | `0x148450` |
| `ldr` at 0x61754 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x61764 | store | 1 | `sp -0xcd` | `0x4` |
| `strb` at 0x61770 | store | 1 | `sp -0xcd` | `0x4c` |
| `ldrb` at 0x61774 | load | 1 | `sp -0xcd` | `-` |
| `ldr` at 0x61778 | load | 4 | `sp -0xcc` | `-` |
| `strb` at 0x6177c | store | 1 | `abs 0x148456` | `([sp-205]:1 & 0xff)` |
| `ldr` at 0x61780 | load | 4 | `abs 0x620c0` | `-` |
| `strb` at 0x61788 | store | 1 | `abs 0x148452` | `0x0` |
| `ldr` at 0x6178c | load | 4 | `abs 0x620c0` | `-` |
| `strb` at 0x61794 | store | 1 | `abs 0x148451` | `0x0` |
| `ldr` at 0x61798 | load | 4 | `abs 0x620c0` | `-` |
| `strb` at 0x617a0 | store | 1 | `abs 0x148450` | `0x0` |
| `str` at 0x617a8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x617ac | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x617b0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x617c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x617c8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x617d0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x617d4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x617dc | load | 4 | `sp +0x8` | `-` |
| `str` at 0x617e4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x617e8 | load | 4 | `abs 0x620c0` | `-` |
| `ldr` at 0x617ec | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x617f8 | store | 1 | `abs 0x148461` | `?` |
| `ldr` at 0x617fc | load | 4 | `abs 0x620c0` | `-` |
| `ldr` at 0x61800 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x6180c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x6181c | store | 4 | `abs 0x148458` | `?` |
| `ldr` at 0x61820 | load | 4 | `abs 0x620c0` | `-` |
| `ldr` at 0x61824 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x61834 | store | 4 | `abs 0x148464` | `?` |
| `ldr` at 0x61838 | load | 4 | `abs 0x620c0` | `-` |
| `ldr` at 0x6183c | load | 4 | `abs 0x148458` | `-` |
| `ldr` at 0x61848 | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x6184c | load | 1 | `abs 0x148457` | `-` |
| `str` at 0x61858 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x6185c | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x61860 | load | 1 | `abs 0x148456` | `-` |
| `ldr` at 0x61868 | load | 4 | `abs 0x620c0` | `-` |
| `ldrh` at 0x6186c | load | 2 | `abs 0x148454` | `-` |
| `ldr` at 0x61880 | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x61884 | load | 1 | `abs 0x148452` | `-` |
| `ldr` at 0x61894 | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x61898 | load | 1 | `abs 0x148451` | `-` |
| `ldr` at 0x618a4 | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x618a8 | load | 1 | `abs 0x148450` | `-` |
| `str` at 0x618b4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x618bc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x618c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x618c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x618cc | load | 4 | `[sp-124] +0x8` | `-` |
| `ldr` at 0x618d0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x618d4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x618ec | load | 4 | `abs 0x620c8` | `-` |
| `ldr` at 0x618f4 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x618f8 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61900 | load | 4 | `abs 0x620c0` | `-` |
| `ldrh` at 0x61904 | load | 2 | `abs 0x148468` | `-` |
| `ldr` at 0x61914 | load | 4 | `abs 0x620c0` | `-` |
| `ldr` at 0x61918 | load | 4 | `abs 0x148464` | `-` |
| `ldr` at 0x6192c | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x61930 | load | 1 | `abs 0x148461` | `-` |
| `str` at 0x61940 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x61944 | load | 4 | `abs 0x620c0` | `-` |
| `ldrb` at 0x61948 | load | 1 | `abs 0x148460` | `-` |
| `ldr` at 0x61954 | load | 4 | `abs 0x620c0` | `-` |
| `ldrh` at 0x61958 | load | 2 | `abs 0x14845e` | `-` |
| `ldr` at 0x6196c | load | 4 | `abs 0x620c0` | `-` |
| `ldrh` at 0x61970 | load | 2 | `abs 0x14845c` | `-` |
| `str` at 0x61980 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x61988 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x61990 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61994 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61998 | load | 4 | `[sp-124] +0xc` | `-` |
| `ldr` at 0x6199c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x619a0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x619b8 | load | 4 | `abs 0x620cc` | `-` |
| `ldr` at 0x619c0 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x619c4 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x619d0 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0x619d4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x619dc | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x619e0 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x619f0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x619fc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x61a04 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x61a0c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x61a18 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x61a24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61a28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61a34 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x61a40 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x61a4c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x61a58 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x61a64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61a68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61a78 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x61a84 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x61a90 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x61a98 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x61aa0 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x61aac | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x61ab8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61abc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ac0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61ac4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61ac8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x61ae0 | load | 4 | `abs 0x620d0` | `-` |
| `ldr` at 0x61ae4 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x61aec | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x61af0 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61af8 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x61b00 | store | 4 | `sp -0xac` | `[sp-172]+1` |
| `ldr` at 0x61b04 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x61b0c | store | 4 | `sp -0xb0` | `[sp-176]+2` |
| `str` at 0x61b18 | store | 4 | `sp -0x54` | `0x40000000` |
| `str` at 0x61b20 | store | 4 | `sp -0x58` | `0x8` |
| `ldr` at 0x61b28 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x61b30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61b34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61b38 | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0x61b3c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61b40 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61b58 | load | 4 | `abs 0x620d4` | `-` |
| `ldr` at 0x61b60 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x61b64 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `str` at 0x61b70 | store | 4 | `sp -0x9c` | `0xa` |
| `str` at 0x61b78 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x61b7c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x61b88 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x61b90 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x61ba4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x61bb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61bb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61bbc | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0x61bd8 | load | 4 | `abs 0x620d8` | `-` |
| `str` at 0x61be4 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x61bec | load | 4 | `sp -0x54` | `-` |
| `str` at 0x61bf8 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x61c00 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x61c0c | load | 4 | `abs 0x620dc` | `-` |
| `ldr` at 0x61c14 | load | 4 | `abs 0x620e0` | `-` |
| `str` at 0x61c18 | store | 4 | `sp -0xa4` | `0xfffff37e` |
| `str` at 0x61c24 | store | 4 | `sp -0x9c` | `0xa` |
| `str` at 0x61c2c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x61c30 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x61c3c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x61c44 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x61c58 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61c68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61c6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61c70 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x61c8c | load | 4 | `abs 0x620e4` | `-` |
| `str` at 0x61c98 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x61ca0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x61cac | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x61cb4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x61cc0 | load | 4 | `abs 0x620e8` | `-` |
| `ldr` at 0x61cc8 | load | 4 | `abs 0x620ec` | `-` |
| `str` at 0x61ccc | store | 4 | `sp -0xa4` | `0xfffff37d` |
| `ldr` at 0x61cd8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61ce8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61cec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61cf0 | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x61d0c | load | 4 | `abs 0x620f0` | `-` |
| `str` at 0x61d18 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x61d20 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x61d30 | load | 4 | `abs 0x620f4` | `-` |
| `ldr` at 0x61d38 | load | 4 | `abs 0x620f8` | `-` |
| `str` at 0x61d3c | store | 4 | `sp -0xa4` | `0xfffff37c` |
| `ldr` at 0x61d44 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x61d54 | load | 4 | `abs 0x620fc` | `-` |
| `ldr` at 0x61d5c | load | 4 | `abs 0x62100` | `-` |
| `str` at 0x61d60 | store | 4 | `sp -0xa4` | `0xfffff37b` |
| `ldr` at 0x61d68 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x61d78 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x61d80 | load | 4 | `abs 0x62104` | `-` |
| `ldr` at 0x61d8c | load | 4 | `abs 0x62108` | `-` |
| `str` at 0x61d90 | store | 4 | `sp -0xa4` | `0xfffff37a` |
| `str` at 0x61d9c | store | 4 | `sp -0x9c` | `0x1` |
| `str` at 0x61da4 | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x61dac | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x61db4 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x61dbc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61dcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61dd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61dd4 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x61df0 | load | 4 | `abs 0x620e4` | `-` |
| `str` at 0x61dfc | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x61e08 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61e10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61e14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e18 | load | 4 | `[sp-132] +0x4` | `-` |
| `ldr` at 0x61e1c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61e20 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61e38 | load | 4 | `abs 0x6210c` | `-` |
| `ldr` at 0x61e40 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x61e44 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61e4c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61e50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61e54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61e58 | load | 4 | `[sp-132] +0x0` | `-` |
| `ldr` at 0x61e5c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x61e60 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x61e78 | load | 4 | `abs 0x62110` | `-` |
| `ldr` at 0x61e80 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x61e84 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61e90 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61e98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61e9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ea0 | load | 4 | `[sp-132] +0x28` | `-` |
| `ldr` at 0x61ec0 | load | 4 | `abs 0x62114` | `-` |
| `ldr` at 0x61ec8 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x61ecc | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61ed8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61ee0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61ee4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ee8 | load | 4 | `[sp-132] +0x24` | `-` |
| `ldr` at 0x61f08 | load | 4 | `abs 0x62118` | `-` |
| `ldr` at 0x61f10 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x61f14 | store | 4 | `sp -0xa4` | `0xfffff380` |
| `ldr` at 0x61f20 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61f30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x61f34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61f38 | load | 4 | `[sp-132] +0x18` | `-` |
| `ldr` at 0x61f54 | load | 4 | `abs 0x620e4` | `-` |
| `str` at 0x61f60 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x61f68 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x61f80 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x61f84 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x61f90 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x61f98 | store | 4 | `sp -0x9c` | `[sp-156]-1` |
| `ldr` at 0x61fa4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x61fb0 | load | 4 | `abs 0x6211c` | `-` |
| `ldr` at 0x61fb8 | load | 4 | `abs 0x62120` | `-` |
| `str` at 0x61fbc | store | 4 | `sp -0xa4` | `0xfffff378` |
| `ldr` at 0x61fc4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x61fcc | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x61fd4 | load | 4 | `abs 0x62044` | `-` |
| `ldr` at 0x61fd8 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x61fdc | store | 4 | `abs 0x142b64` | `[sp-180]` |
| `ldr` at 0x61fe4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x61fec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x61ff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61ff4 | load | 4 | `[sp-132] +0x20` | `-` |
| `ldr` at 0x61ff8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x61ffc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62014 | load | 4 | `abs 0x62124` | `-` |
| `ldr` at 0x6201c | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x62020 | store | 4 | `sp -0xc0` | `0xfffff380` |
| `str` at 0x6202c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x62030 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x62034 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x62134 | load | 4 | `abs 0x62128` | `-` |
| `ldr` at 0x62138 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6213c | load | 4 | `(unknown)` | `-` |
| `str` at 0x62140 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x62144 | load | 4 | `abs 0x62128` | `-` |
| `ldr` at 0x62148 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6215c | load | 4 | `(unknown)` | `-` |
| `str` at 0x62160 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x62164 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6216c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x62174 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62178 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6217c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62180 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x62184 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x6219c | load | 4 | `abs 0x6212c` | `-` |
| `ldr` at 0x621a4 | load | 4 | `abs 0x62130` | `-` |
| `str` at 0x621a8 | store | 4 | `sp -0xc0` | `0xfffff380` |
| `ldr` at 0x621b0 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x621b8 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x621c0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x621c4 | store | 4 | `sp -0xc0` | `[sp-164]` |
| `ldr` at 0x621c8 | load | 4 | `sp -0xc0` | `-` |
| `ldmdb` at 0x621cc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x621cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x621cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x621cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x611e4 | `(unknown callee) 0x2070c` | `0x10c8f4` | `r1` | `[sp-196]` | `[sp-196]` |
| `bl` at 0x612f4 | `(unknown callee) 0xeb27c` | `?` | `r1` | `[sp-196]` | `?` |
| `bl` at 0x61314 | `(unknown callee) 0x2070c` | `0x10c908` | `[sp-28]` | `?` | `ret(0xeb27c)` |
| `bl` at 0x61328 | `(unknown callee) 0xe9080` | `ret(0xeb27c)` | `?` | `?` | `ret(0xeb27c)` |
| `bl` at 0x61340 | `(unknown callee) 0x2070c` | `0x10c928` | `[sp-28]` | `?` | `ret(0xe9080)` |
| `mov` at 0x61370 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]]` | `sp-76` | `sp-80` |
| `bl` at 0x61384 | `(unknown callee) 0x2070c` | `0x10c948` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x613d4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `sp-68` | `sp-72` |
| `mov` at 0x61430 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `0x0` | `0x0` |
| `bl` at 0x61444 | `(unknown callee) 0x2070c` | `0x10c968` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61470 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61484 | `(unknown callee) 0x2070c` | `0x10c988` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x614b8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+28]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x614cc | `(unknown callee) 0x2070c` | `0x10c9a8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61500 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61514 | `(unknown callee) 0x2070c` | `0x10c9c8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x615ac | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x615c0 | `(unknown callee) 0x2070c` | `0x10c9e4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61618 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `sp-84` | `sp-88` |
| `bl` at 0x6162c | `(unknown callee) 0x2070c` | `0x10ca04` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x616a4 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x616b8 | `(unknown callee) 0x2070c` | `0x10ca2c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x618dc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+8]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x618f0 | `(unknown callee) 0x2070c` | `0x10ca54` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x619a8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+12]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x619bc | `(unknown callee) 0x2070c` | `0x10ca74` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61ad0 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x61ae8 | `(unknown callee) 0x2070c` | `0x10ca94` | `[sp-172]` | `?` | `ret(unknown)` |
| `mov` at 0x61b48 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+4]` | `0x40000000` | `0x8` |
| `bl` at 0x61b5c | `(unknown callee) 0x2070c` | `0x10cab4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61bc8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-124]+4]` | `sp-84` | `sp-88` |
| `bl` at 0x61bdc | `(unknown callee) 0x2070c` | `0x10cad8` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x61c10 | `(unknown callee) 0x2070c` | `0x10caf0` | `?` | `?` | `?` |
| `mov` at 0x61c7c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-84` | `sp-88` |
| `bl` at 0x61c90 | `(unknown callee) 0x2070c` | `0x10cb04` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x61cc4 | `(unknown callee) 0x2070c` | `0x10cb1c` | `?` | `?` | `?` |
| `mov` at 0x61cfc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `sp-92` | `sp-96` |
| `bl` at 0x61d10 | `(unknown callee) 0x20240` | `0x10cb30` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x61d34 | `(unknown callee) 0x20240` | `0x10cb44` | `?` | `?` | `?` |
| `bl` at 0x61d58 | `(unknown callee) 0x20240` | `0x10cb54` | `?` | `?` | `?` |
| `bl` at 0x61d88 | `(unknown callee) 0x2070c` | `0x10cb68` | `?` | `?` | `?` |
| `mov` at 0x61de0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-184` | `sp-188` |
| `bl` at 0x61df4 | `(unknown callee) 0x2070c` | `0x10cb04` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61e28 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+4]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61e3c | `(unknown callee) 0x2070c` | `0x10cb80` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61e68 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]]` | `[sp-84]` | `[sp-88]` |
| `bl` at 0x61e7c | `(unknown callee) 0x2070c` | `0x10cb94` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61eb0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+40]` | `?` | `?` |
| `bl` at 0x61ec4 | `(unknown callee) 0x2070c` | `0x10cbac` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61ef8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+36]` | `?` | `?` |
| `bl` at 0x61f0c | `(unknown callee) 0x2070c` | `0x10cbc8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x61f44 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+24]` | `sp-184` | `sp-188` |
| `bl` at 0x61f58 | `(unknown callee) 0x2070c` | `0x10cb04` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x61fb4 | `(unknown callee) 0x2070c` | `0x10cbe0` | `?` | `?` | `[sp-152]` |
| `mov` at 0x62004 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-132]+32]` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x62018 | `(unknown callee) 0x2070c` | `0x10cbf4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6218c | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x621a0 | `(unknown callee) 0x2070c` | `0x10cc18` | `?` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6113c | `r0 == 0x1` | `r0 != 0x1` |
| `beq` at 0x61168 | `[0x148448] == 0x0` | `[0x148448] != 0x0` |
| `beq` at 0x611b4 | `[sp-196] == 0x4` | `[sp-196] != 0x4` |
| `bgt` at 0x611c0 | `[sp-196] > 0x4 (signed, 32-bit)` | `[sp-196] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x611cc | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `beq` at 0x611dc | `[sp-196] == 0x7` | `[sp-196] != 0x7` |
| `beq` at 0x61204 | `[sp-196] == 0x4` | `[sp-196] != 0x4` |
| `bgt` at 0x61210 | `[sp-196] > 0x4 (signed, 32-bit)` | `[sp-196] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x6121c | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `beq` at 0x6122c | `[sp-196] == 0x7` | `[sp-196] != 0x7` |
| `bne` at 0x61308 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `beq` at 0x61334 | `ret(0xe9080) == 0x0` | `ret(0xe9080) != 0x0` |
| `beq` at 0x6137c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x613e0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6143c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6147c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x614c4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6150c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x6152c | `[sp-168] != 0x2` | `[sp-168] == 0x2` |
| `bne` at 0x61560 | `[sp-168] != 0x4` | `[sp-168] == 0x4` |
| `beq` at 0x615b8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x615e4 | `C == 0` | `not (C == 0)` |
| `beq` at 0x61624 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x616b0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x616e0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x616f4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6175c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x617b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x618e8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x619b4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x619e8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `ble` at 0x619f8 | `[sp-160] <= 0x0 (signed, 32-bit)` | `[sp-160] > 0x0 (signed, 32-bit)` |
| `bne` at 0x61a30 | `? != ?` | `? == ?` |
| `bne` at 0x61a70 | `? != ?` | `? == ?` |
| `beq` at 0x61adc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61b54 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x61b84 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x61b98 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x61bd4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x61c08 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x61c38 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x61c4c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x61c88 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x61cbc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x61d08 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61d2c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61d50 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x61d74 | `? == 0xe0` | `? != 0xe0` |
| `beq` at 0x61dec | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61e34 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61e74 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61ebc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61f04 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x61f50 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x61f8c | `[sp-152] != 0x0` | `[sp-152] == 0x0` |
| `bne` at 0x61fa0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x61fac | `[sp-152] != 0x0` | `[sp-152] == 0x0` |
| `beq` at 0x62010 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x6203c | `C == 0` | `not (C == 0)` |
| `beq` at 0x62198 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x621cc | `r0` | `?` |

## `PktGenOp`

- `PktGenOp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x621d0 to 0x63a3c.
  Established. The initial word read tests absolute `0x148448` (`ldr` at 0x62224). Zero takes the branch and nonzero
  falls through (`ldr` at 0x62224, `beq` at 0x6222c). The following known stores include word
  `0xea468` at absolute `0x2b0940` and word `[sp-40]` at absolute `0x2b0920` (`str` at 0x62238,
  `str` at 0x62244). A later halfword store places `0x123` at absolute `0x148468` (`strh` at
  0x62a64). Packet-operation register semantics and the return value remain unknown (`ldmdb` at
  0x63a38).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x621e0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x621e4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x621e8 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x621ec | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x621f4 | store | 4 | `sp -0x70` | `0x0` |
| `str` at 0x621fc | store | 4 | `sp -0x78` | `0x0` |
| `str` at 0x62204 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x6220c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x62210 | load | 4 | `abs 0x63138` | `-` |
| `str` at 0x62214 | store | 4 | `sp -0x2c` | `0xea468` |
| `ldr` at 0x62218 | load | 4 | `abs 0x6313c` | `-` |
| `str` at 0x6221c | store | 4 | `sp -0x28` | `0xea56c` |
| `ldr` at 0x62220 | load | 4 | `abs 0x63140` | `-` |
| `ldr` at 0x62224 | load | 4 | `abs 0x148448` | `-` |
| `ldr` at 0x62230 | load | 4 | `abs 0x63144` | `-` |
| `ldr` at 0x62234 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x62238 | store | 4 | `abs 0x2b0940` | `0xea468` |
| `ldr` at 0x6223c | load | 4 | `abs 0x63148` | `-` |
| `ldr` at 0x62240 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x62244 | store | 4 | `abs 0x2b0920` | `[sp-40]` |
| `ldr` at 0x62248 | load | 4 | `abs 0x6314c` | `-` |
| `str` at 0x6224c | store | 4 | `sp -0x2c` | `0x60ff0` |
| `ldr` at 0x62250 | load | 4 | `abs 0x63150` | `-` |
| `str` at 0x62254 | store | 4 | `sp -0x28` | `0x61070` |
| `ldr` at 0x62258 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x62264 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x62268 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x6226c | store | 4 | `sp -0x9c` | `[sp-188]` |
| `ldr` at 0x62270 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x6227c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x62288 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x62298 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x622a4 | load | 4 | `abs 0x63154` | `-` |
| `ldr` at 0x622ac | load | 4 | `abs 0x63158` | `-` |
| `str` at 0x622b0 | store | 4 | `sp -0xb8` | `0xfffff377` |
| `ldr` at 0x622b8 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x622bc | store | 4 | `sp -0xc0` | `[sp-188]` |
| `ldr` at 0x622c0 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x622cc | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x622d8 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x622e8 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x622f8 | load | 4 | `abs 0x6315c` | `-` |
| `str` at 0x622fc | store | 4 | `sp -0x70` | `0x10c79c` |
| `str` at 0x62304 | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x62308 | load | 4 | `abs 0x63160` | `-` |
| `str` at 0x6230c | store | 4 | `sp -0x78` | `0x10c7f4` |
| `str` at 0x62314 | store | 4 | `sp -0x7c` | `0xb` |
| `ldr` at 0x62318 | load | 4 | `abs 0x63164` | `-` |
| `str` at 0x6231c | store | 4 | `sp -0x80` | `0x10c820` |
| `str` at 0x62324 | store | 4 | `sp -0x84` | `0x6` |
| `str` at 0x6232c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x62334 | load | 4 | `abs 0x63168` | `-` |
| `str` at 0x62338 | store | 4 | `sp -0x70` | `0x10c700` |
| `str` at 0x62340 | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x62344 | load | 4 | `abs 0x6316c` | `-` |
| `str` at 0x62348 | store | 4 | `sp -0x78` | `0x10c758` |
| `str` at 0x62350 | store | 4 | `sp -0x7c` | `0xb` |
| `str` at 0x62358 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x62360 | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0x62368 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x62370 | load | 4 | `abs 0x63170` | `-` |
| `str` at 0x62374 | store | 4 | `sp -0x70` | `0x10c838` |
| `str` at 0x6237c | store | 4 | `sp -0x74` | `0x16` |
| `ldr` at 0x62380 | load | 4 | `abs 0x63174` | `-` |
| `str` at 0x62384 | store | 4 | `sp -0x78` | `0x10c890` |
| `str` at 0x6238c | store | 4 | `sp -0x7c` | `0xb` |
| `ldr` at 0x62390 | load | 4 | `abs 0x63178` | `-` |
| `str` at 0x62394 | store | 4 | `sp -0x80` | `0x10c8bc` |
| `str` at 0x6239c | store | 4 | `sp -0x84` | `0x6` |
| `ldr` at 0x623a0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x623a8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x623ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x623b8 | store | 4 | `sp -0x98` | `ret(0xe9028)` |
| `ldr` at 0x623bc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x623c8 | load | 4 | `abs 0x6317c` | `-` |
| `ldr` at 0x623cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x623d4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x623d8 | load | 4 | `abs 0x63180` | `-` |
| `ldr` at 0x623e4 | load | 4 | `abs 0x63184` | `-` |
| `str` at 0x623e8 | store | 4 | `sp -0xc4` | `0xfffff375` |
| `ldr` at 0x623f0 | load | 4 | `abs 0x63188` | `-` |
| `str` at 0x623f4 | store | 4 | `sp -0xc4` | `0xfffff374` |
| `ldr` at 0x623f8 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x623fc | store | 4 | `sp -0xb8` | `[sp-196]` |
| `ldr` at 0x62404 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62410 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62418 | load | 4 | `[sp-112] +0x0` | `-` |
| `ldr` at 0x62434 | load | 4 | `abs 0x6318c` | `-` |
| `str` at 0x62440 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x62448 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x62454 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x6245c | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x62464 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62474 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62478 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6247c | load | 4 | `[sp-120] +0x20` | `-` |
| `str` at 0x6249c | store | 4 | `sp -0xb8` | `?` |
| `str` at 0x624a8 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x624b0 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x624b8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x624c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x624c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x624c8 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x624cc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x624d0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x624e8 | load | 4 | `abs 0x63190` | `-` |
| `ldr` at 0x624f0 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x624f4 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62500 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62508 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6250c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62510 | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x62514 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62518 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62530 | load | 4 | `abs 0x63194` | `-` |
| `ldr` at 0x62538 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x6253c | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62548 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62550 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62554 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62558 | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x6255c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62560 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62578 | load | 4 | `abs 0x63198` | `-` |
| `ldr` at 0x62580 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62584 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62590 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62598 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6259c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x625a0 | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x625a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x625a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x625c0 | load | 4 | `abs 0x6319c` | `-` |
| `ldr` at 0x625c8 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x625cc | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x625d8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x625e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x625e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x625e8 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x625ec | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x625f0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62608 | load | 4 | `abs 0x631a0` | `-` |
| `ldr` at 0x62610 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62614 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x6261c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62620 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62624 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62628 | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0x6262c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62630 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62648 | load | 4 | `abs 0x631a4` | `-` |
| `ldr` at 0x62650 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62654 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62660 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x62668 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6266c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62670 | load | 4 | `[sp-120] +0x1c` | `-` |
| `ldr` at 0x62674 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62678 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62690 | load | 4 | `abs 0x631a8` | `-` |
| `ldr` at 0x62698 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x6269c | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x626a8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x626b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x626b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x626b8 | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x626bc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x626c0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x626d8 | load | 4 | `abs 0x631ac` | `-` |
| `ldr` at 0x626e0 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x626e4 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x626ec | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x626f8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x626fc | store | 4 | `sp -0x48` | `[sp-56]` |
| `ldr` at 0x62700 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x62708 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x62710 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x62720 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x62724 | load | 4 | `abs 0x631b0` | `-` |
| `str` at 0x62728 | store | 4 | `sp -0x4c` | `0x7ffffd` |
| `str` at 0x62734 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x62738 | load | 4 | `abs 0x631b4` | `-` |
| `str` at 0x6273c | store | 4 | `sp -0x4c` | `0x1fffffd` |
| `ldr` at 0x62744 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6274c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62750 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62754 | load | 4 | `[sp-120] +0x20` | `-` |
| `ldr` at 0x62758 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x6275c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62774 | load | 4 | `abs 0x631b8` | `-` |
| `ldr` at 0x6277c | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62780 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `str` at 0x6278c | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x62790 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x62794 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x627a4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x627ac | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x627bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x627c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x627c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x627e0 | load | 4 | `abs 0x631bc` | `-` |
| `str` at 0x627ec | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x627f4 | load | 4 | `abs 0x63270` | `-` |
| `ldr` at 0x627f8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x627fc | load | 4 | `sp -0x48` | `-` |
| `str` at 0x62800 | store | 4 | `(unknown)` | `[sp-72]` |
| `ldr` at 0x62804 | load | 4 | `abs 0x63270` | `-` |
| `ldr` at 0x62808 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6281c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x62820 | store | 4 | `(unknown)` | `[sp-76]` |
| `ldr` at 0x62824 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62828 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x62830 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x62834 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6283c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x62844 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62848 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6284c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62850 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62854 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6286c | load | 4 | `abs 0x631c0` | `-` |
| `ldr` at 0x62874 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62878 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62880 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x62888 | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `str` at 0x62894 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x62898 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6289c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x628b8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x628c0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x628c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x628cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x628dc | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x628e4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x628ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x628f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x628f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62900 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62910 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6291c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x62920 | store | 4 | `sp -0x20` | `[sp-48]` |
| `ldr` at 0x62924 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62930 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x62934 | store | 4 | `sp -0xc8` | `[sp-156]` |
| `ldr` at 0x62938 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x62944 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x62950 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x62960 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x6297c | load | 4 | `(unknown)` | `-` |
| `str` at 0x62990 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x6299c | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0x629a4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x629b0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x629bc | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x629d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x629e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x629e8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x629ec | load | 4 | `abs 0x63140` | `-` |
| `ldr` at 0x629f0 | load | 4 | `abs 0x148448` | `-` |
| `ldr` at 0x629fc | load | 4 | `abs 0x631c4` | `-` |
| `ldr` at 0x62a00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x62a08 | load | 4 | `abs 0x631c8` | `-` |
| `ldr` at 0x62a0c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x62a14 | load | 4 | `abs 0x631cc` | `-` |
| `ldr` at 0x62a18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x62a20 | load | 4 | `abs 0x631d0` | `-` |
| `ldr` at 0x62a24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x62a38 | load | 4 | `abs 0x631d4` | `-` |
| `ldr` at 0x62a3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62a50 | load | 4 | `abs 0x631d8` | `-` |
| `ldr` at 0x62a54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62a5c | load | 4 | `abs 0x631dc` | `-` |
| `ldr` at 0x62a60 | load | 4 | `abs 0x631e0` | `-` |
| `strh` at 0x62a64 | store | 2 | `abs 0x148468` | `0x123` |
| `ldr` at 0x62a68 | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62a6c | load | 1 | `sp -0x34` | `-` |
| `strb` at 0x62a70 | store | 1 | `abs 0x148460` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0x62a74 | load | 4 | `abs 0x631dc` | `-` |
| `ldrh` at 0x62a78 | load | 2 | `sp -0x30` | `-` |
| `strh` at 0x62a7c | store | 2 | `abs 0x14845e` | `([sp-48]:2 & 0xffff)` |
| `ldr` at 0x62a80 | load | 4 | `abs 0x631dc` | `-` |
| `ldrh` at 0x62a84 | load | 2 | `sp +0x0` | `-` |
| `strh` at 0x62a8c | store | 2 | `abs 0x14845c` | `?` |
| `ldr` at 0x62a90 | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62a94 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x62a98 | store | 1 | `abs 0x148457` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x62a9c | load | 4 | `abs 0x631dc` | `-` |
| `ldrh` at 0x62aa0 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x62aa4 | store | 2 | `abs 0x148454` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0x62aa8 | load | 4 | `abs 0x631dc` | `-` |
| `str` at 0x62aac | store | 4 | `sp -0xcc` | `0x148450` |
| `ldr` at 0x62ab0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x62ac0 | store | 1 | `sp -0xcd` | `0x11` |
| `strb` at 0x62acc | store | 1 | `sp -0xcd` | `0x31` |
| `ldrb` at 0x62ad0 | load | 1 | `sp -0xcd` | `-` |
| `ldr` at 0x62ad4 | load | 4 | `sp -0xcc` | `-` |
| `strb` at 0x62ad8 | store | 1 | `abs 0x148456` | `([sp-205]:1 & 0xff)` |
| `ldr` at 0x62adc | load | 4 | `abs 0x631dc` | `-` |
| `strb` at 0x62ae4 | store | 1 | `abs 0x148452` | `0x0` |
| `ldr` at 0x62ae8 | load | 4 | `abs 0x631dc` | `-` |
| `strb` at 0x62af0 | store | 1 | `abs 0x148451` | `0x0` |
| `ldr` at 0x62af4 | load | 4 | `abs 0x631dc` | `-` |
| `strb` at 0x62afc | store | 1 | `abs 0x148450` | `0x0` |
| `ldr` at 0x62b00 | load | 4 | `abs 0x631dc` | `-` |
| `ldr` at 0x62b10 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x62b1c | store | 1 | `abs 0x148461` | `?` |
| `ldr` at 0x62b20 | load | 4 | `abs 0x631dc` | `-` |
| `ldr` at 0x62b30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62b48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x62b58 | store | 4 | `abs 0x148458` | `?` |
| `ldr` at 0x62b5c | load | 4 | `abs 0x631dc` | `-` |
| `ldr` at 0x62b6c | load | 4 | `(unknown)` | `-` |
| `str` at 0x62b7c | store | 4 | `abs 0x148464` | `?` |
| `ldr` at 0x62b80 | load | 4 | `abs 0x631dc` | `-` |
| `ldr` at 0x62b84 | load | 4 | `abs 0x148458` | `-` |
| `ldr` at 0x62b90 | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62b94 | load | 1 | `abs 0x148457` | `-` |
| `str` at 0x62ba0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x62ba4 | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62ba8 | load | 1 | `abs 0x148456` | `-` |
| `ldr` at 0x62bb0 | load | 4 | `abs 0x631dc` | `-` |
| `ldrh` at 0x62bb4 | load | 2 | `abs 0x148454` | `-` |
| `ldr` at 0x62bc8 | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62bcc | load | 1 | `abs 0x148452` | `-` |
| `ldr` at 0x62bdc | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62be0 | load | 1 | `abs 0x148451` | `-` |
| `ldr` at 0x62bec | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62bf0 | load | 1 | `abs 0x148450` | `-` |
| `str` at 0x62bfc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x62c04 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62c0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62c10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62c14 | load | 4 | `[sp-112] +0x8` | `-` |
| `ldr` at 0x62c18 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62c1c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62c34 | load | 4 | `abs 0x631e4` | `-` |
| `ldr` at 0x62c3c | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62c40 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62c48 | load | 4 | `abs 0x631dc` | `-` |
| `ldrh` at 0x62c4c | load | 2 | `abs 0x148468` | `-` |
| `ldr` at 0x62c5c | load | 4 | `abs 0x631dc` | `-` |
| `ldr` at 0x62c60 | load | 4 | `abs 0x148464` | `-` |
| `ldr` at 0x62c74 | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62c78 | load | 1 | `abs 0x148461` | `-` |
| `str` at 0x62c88 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x62c8c | load | 4 | `abs 0x631dc` | `-` |
| `ldrb` at 0x62c90 | load | 1 | `abs 0x148460` | `-` |
| `ldr` at 0x62c9c | load | 4 | `abs 0x631dc` | `-` |
| `ldrh` at 0x62ca0 | load | 2 | `abs 0x14845e` | `-` |
| `ldr` at 0x62cb4 | load | 4 | `abs 0x631dc` | `-` |
| `ldrh` at 0x62cb8 | load | 2 | `abs 0x14845c` | `-` |
| `str` at 0x62cc8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x62cd0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62cd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62cdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62ce0 | load | 4 | `[sp-112] +0xc` | `-` |
| `ldr` at 0x62ce4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62ce8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62d00 | load | 4 | `abs 0x631e8` | `-` |
| `ldr` at 0x62d08 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62d0c | store | 4 | `sp -0x98` | `0xfffff380` |
| `str` at 0x62d18 | store | 4 | `sp -0x48` | `0x40000000` |
| `str` at 0x62d20 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x62d24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x62d30 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x62d38 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x62d40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62d54 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62d5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62d60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62d64 | load | 4 | `[sp-112] +0x10` | `-` |
| `ldr` at 0x62d84 | load | 4 | `abs 0x631ec` | `-` |
| `ldr` at 0x62d8c | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62d90 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62d9c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62da4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x62dac | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x62db4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x62dbc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x62dc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62dc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62dcc | load | 4 | `[sp-112] +0x14` | `-` |
| `ldr` at 0x62dd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62dd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62dec | load | 4 | `abs 0x631f0` | `-` |
| `ldr` at 0x62df4 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62df8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62e04 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62e0c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x62e14 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x62e1c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x62e24 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x62e2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62e30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62e34 | load | 4 | `[sp-112] +0x10` | `-` |
| `ldr` at 0x62e38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62e3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x62e54 | load | 4 | `abs 0x631ec` | `-` |
| `ldr` at 0x62e5c | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62e60 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62e6c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62e74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62e78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62e7c | load | 4 | `[sp-112] +0x14` | `-` |
| `ldr` at 0x62e9c | load | 4 | `abs 0x631f0` | `-` |
| `ldr` at 0x62ea4 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62ea8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x62eb0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x62eb8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x62ec0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62ec8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x62ecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62ed0 | load | 4 | `[sp-112] +0x4` | `-` |
| `ldr` at 0x62ed4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x62ed8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x62ef0 | load | 4 | `abs 0x631f4` | `-` |
| `ldr` at 0x62ef8 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x62efc | store | 4 | `sp -0x98` | `0xfffff380` |
| `str` at 0x62f08 | store | 4 | `sp -0x90` | `0x14` |
| `str` at 0x62f10 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x62f14 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x62f20 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x62f28 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x62f3c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x62f4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x62f50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x62f54 | load | 4 | `[sp-112] +0x4` | `-` |
| `ldr` at 0x62f70 | load | 4 | `abs 0x631f8` | `-` |
| `str` at 0x62f7c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x62f84 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x62f90 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x62f94 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x62fac | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x62fb8 | load | 4 | `abs 0x631fc` | `-` |
| `ldr` at 0x62fc0 | load | 4 | `abs 0x63200` | `-` |
| `str` at 0x62fc4 | store | 4 | `sp -0x98` | `0xfffff37e` |
| `str` at 0x62fd0 | store | 4 | `sp -0x90` | `0x14` |
| `str` at 0x62fd8 | store | 4 | `sp -0x8c` | `0x0` |
| `ldr` at 0x62fdc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x62fe8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x62ff0 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x63004 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63014 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6301c | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x63038 | load | 4 | `abs 0x63250` | `-` |
| `str` at 0x63044 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6304c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x63058 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6305c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63074 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63080 | load | 4 | `abs 0x63204` | `-` |
| `ldr` at 0x63088 | load | 4 | `abs 0x63208` | `-` |
| `str` at 0x6308c | store | 4 | `sp -0x98` | `0xfffff37d` |
| `ldr` at 0x63094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x630a4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x630b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x630b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x630bc | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x630d8 | load | 4 | `abs 0x6320c` | `-` |
| `str` at 0x630e4 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x630f0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63100 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63108 | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x63124 | load | 4 | `abs 0x6320c` | `-` |
| `str` at 0x63130 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63280 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63290 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63298 | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x632b4 | load | 4 | `abs 0x6320c` | `-` |
| `str` at 0x632c0 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x632cc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x632dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x632e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x632e4 | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x63300 | load | 4 | `abs 0x6320c` | `-` |
| `str` at 0x6330c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63314 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x63324 | load | 4 | `abs 0x63210` | `-` |
| `ldr` at 0x6332c | load | 4 | `abs 0x63214` | `-` |
| `str` at 0x63330 | store | 4 | `sp -0x98` | `0xfffff37c` |
| `ldr` at 0x63338 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x63348 | load | 4 | `abs 0x63218` | `-` |
| `ldr` at 0x63350 | load | 4 | `abs 0x6321c` | `-` |
| `str` at 0x63354 | store | 4 | `sp -0x98` | `0xfffff37b` |
| `ldr` at 0x6335c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x6336c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x63374 | load | 4 | `abs 0x63220` | `-` |
| `ldr` at 0x63380 | load | 4 | `abs 0x63224` | `-` |
| `str` at 0x63384 | store | 4 | `sp -0x98` | `0xfffff37a` |
| `ldr` at 0x6338c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x63398 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x6339c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x633a4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x633a8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x633b0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x633b8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x633bc | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x633c4 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x633d4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x633d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x633f0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x633f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63418 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6341c | store | 4 | `sp -0xd4` | `?` |
| `ldr` at 0x63420 | load | 4 | `abs 0x63228` | `-` |
| `ldr` at 0x63424 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63428 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x6342c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x63434 | load | 4 | `abs 0x6322c` | `-` |
| `str` at 0x63438 | store | 4 | `sp -0x98` | `0xfffff379` |
| `ldr` at 0x6344c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6345c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63464 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x6346c | load | 4 | `sp -0x60` | `-` |
| `str` at 0x63470 | store | 4 | `(unknown)` | `[sp-96]` |
| `ldr` at 0x63474 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6347c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63484 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x63488 | store | 4 | `(unknown)` | `[sp-100]` |
| `ldr` at 0x63490 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63498 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x634a0 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x634a4 | store | 4 | `(unknown)` | `[sp-104]` |
| `ldr` at 0x634a8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x634b0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x634b8 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x634bc | store | 4 | `(unknown)` | `[sp-108]` |
| `str` at 0x634c4 | store | 4 | `sp -0x90` | `0x1` |
| `str` at 0x634cc | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x634d4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x634dc | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x634e4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x634f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x634f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x634fc | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x63518 | load | 4 | `abs 0x63250` | `-` |
| `str` at 0x63524 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63530 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63538 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6353c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63540 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x63544 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63548 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63560 | load | 4 | `abs 0x63230` | `-` |
| `ldr` at 0x63568 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x6356c | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63578 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63580 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63584 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63588 | load | 4 | `[sp-120] +0x8` | `-` |
| `ldr` at 0x6358c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63590 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x635a8 | load | 4 | `abs 0x63234` | `-` |
| `ldr` at 0x635b0 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x635b4 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x635c0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x635c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x635cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x635d0 | load | 4 | `[sp-120] +0xc` | `-` |
| `ldr` at 0x635d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x635d8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x635f0 | load | 4 | `abs 0x63238` | `-` |
| `ldr` at 0x635f8 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x635fc | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63608 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63610 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63614 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63618 | load | 4 | `[sp-120] +0x10` | `-` |
| `ldr` at 0x6361c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63620 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63638 | load | 4 | `abs 0x6323c` | `-` |
| `ldr` at 0x63640 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x63644 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63650 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63658 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6365c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63660 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x63664 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x63668 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63680 | load | 4 | `abs 0x63240` | `-` |
| `ldr` at 0x63688 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x6368c | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x63694 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63698 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6369c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x636a0 | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0x636a4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x636a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x636c0 | load | 4 | `abs 0x63244` | `-` |
| `ldr` at 0x636c8 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x636cc | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x636d8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x636e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x636ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x636f0 | load | 4 | `[sp-120] +0x1c` | `-` |
| `ldr` at 0x6370c | load | 4 | `abs 0x63250` | `-` |
| `str` at 0x63718 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x63724 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6372c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63730 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63734 | load | 4 | `[sp-120] +0x28` | `-` |
| `ldr` at 0x63738 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6373c | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x63754 | load | 4 | `abs 0x63248` | `-` |
| `ldr` at 0x6375c | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x63760 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x6376c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63774 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6377c | load | 4 | `[sp-120] +0x24` | `-` |
| `ldr` at 0x63780 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x63784 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x6379c | load | 4 | `abs 0x6324c` | `-` |
| `ldr` at 0x637a4 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x637a8 | store | 4 | `sp -0x98` | `0xfffff380` |
| `ldr` at 0x637b4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x637c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x637c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x637cc | load | 4 | `[sp-120] +0x18` | `-` |
| `ldr` at 0x637e8 | load | 4 | `abs 0x63250` | `-` |
| `str` at 0x637f4 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x637fc | load | 4 | `sp -0xac` | `-` |
| `str` at 0x63814 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x63818 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63824 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6382c | store | 4 | `sp -0x90` | `[sp-144]-1` |
| `ldr` at 0x63838 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x63844 | load | 4 | `abs 0x63254` | `-` |
| `ldr` at 0x6384c | load | 4 | `abs 0x63258` | `-` |
| `str` at 0x63850 | store | 4 | `sp -0x98` | `0xfffff378` |
| `ldr` at 0x63858 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x63860 | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `ldr` at 0x63868 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6387c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x63880 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63884 | load | 4 | `abs 0x63260` | `-` |
| `ldr` at 0x638a4 | load | 4 | `abs 0x6325c` | `-` |
| `ldr` at 0x638ac | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x638b0 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x638b8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x638c0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x638c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x638c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x638cc | load | 4 | `abs 0x63260` | `-` |
| `ldr` at 0x638d0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x638d4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x638ec | load | 4 | `abs 0x63268` | `-` |
| `ldr` at 0x638f4 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x638f8 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x63900 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x63904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63908 | load | 4 | `abs 0x63264` | `-` |
| `ldr` at 0x63928 | load | 4 | `abs 0x63268` | `-` |
| `ldr` at 0x63930 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x63934 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x63940 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x63948 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6394c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x63950 | load | 4 | `[sp-120] +0x20` | `-` |
| `ldr` at 0x63954 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x63958 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x63970 | load | 4 | `abs 0x6326c` | `-` |
| `ldr` at 0x63978 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x6397c | store | 4 | `sp -0xb8` | `0xfffff380` |
| `str` at 0x63988 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x6398c | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x63990 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x639a0 | load | 4 | `abs 0x63270` | `-` |
| `ldr` at 0x639a4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x639a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x639ac | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x639b0 | load | 4 | `abs 0x63270` | `-` |
| `ldr` at 0x639b4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x639c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x639cc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x639d0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x639d8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x639e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x639e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x639e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x639ec | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x639f0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x63a08 | load | 4 | `abs 0x63274` | `-` |
| `ldr` at 0x63a10 | load | 4 | `abs 0x63278` | `-` |
| `str` at 0x63a14 | store | 4 | `sp -0xb8` | `0xfffff380` |
| `ldr` at 0x63a1c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x63a24 | store | 4 | `sp -0x94` | `[sp-148]+1` |
| `ldr` at 0x63a2c | load | 4 | `sp -0x98` | `-` |
| `str` at 0x63a30 | store | 4 | `sp -0xb8` | `[sp-152]` |
| `ldr` at 0x63a34 | load | 4 | `sp -0xb8` | `-` |
| `ldmdb` at 0x63a38 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x63a38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63a38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63a38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x622a8 | `(unknown callee) 0x2070c` | `0x10c8f4` | `[sp-188]` | `[sp-188]` | `?` |
| `bl` at 0x623b0 | `(unknown callee) 0xe9028` | `?` | `[sp-188]` | `[sp-188]` | `?` |
| `bl` at 0x623d0 | `(unknown callee) 0x2070c` | `0x10c928` | `[sp-28]` | `?` | `ret(0xe9028)` |
| `mov` at 0x62424 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]]` | `sp-64` | `sp-68` |
| `bl` at 0x62438 | `(unknown callee) 0x2070c` | `0x10c948` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62488 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `sp-56` | `sp-60` |
| `mov` at 0x624d8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `0x0` | `0x0` |
| `bl` at 0x624ec | `(unknown callee) 0x2070c` | `0x10c968` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62520 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62534 | `(unknown callee) 0x2070c` | `0x10cc44` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62568 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x6257c | `(unknown callee) 0x2070c` | `0x10cc64` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x625b0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x625c4 | `(unknown callee) 0x2070c` | `0x10cc84` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x625f8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+20]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x6260c | `(unknown callee) 0x2070c` | `0x10cca4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62638 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x6264c | `(unknown callee) 0x2070c` | `0x10c988` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62680 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+28]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62694 | `(unknown callee) 0x2070c` | `0x10c9a8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x626c8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x626dc | `(unknown callee) 0x2070c` | `0x10c9c8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62764 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `?` | `[sp-76]` |
| `bl` at 0x62778 | `(unknown callee) 0x2070c` | `0x10c9e4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x627d0 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `sp-72` | `sp-76` |
| `bl` at 0x627e4 | `(unknown callee) 0x2070c` | `0x10ca04` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6285c | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62870 | `(unknown callee) 0x2070c` | `0x10ca2c` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x62a04 | `(unknown callee) 0x2070c` | `0x10ccc4` | `[sp-48]` | `?` | `[0x148448]` |
| `bl` at 0x62a10 | `(unknown callee) 0x2070c` | `0x10ccd0` | `[sp-52]` | `?` | `?` |
| `bl` at 0x62a1c | `(unknown callee) 0x2070c` | `0x10ccdc` | `[sp-32]` | `?` | `?` |
| `bl` at 0x62a28 | `(unknown callee) 0x2070c` | `0x10cce8` | `[sp-36]` | `?` | `?` |
| `bl` at 0x62a40 | `(unknown callee) 0x2070c` | `0x10ccf4` | `?` | `?` | `?` |
| `bl` at 0x62a58 | `(unknown callee) 0x2070c` | `0x10cd00` | `?` | `?` | `?` |
| `mov` at 0x62c24 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62c38 | `(unknown callee) 0x2070c` | `0x10ca54` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62cf0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x62d04 | `(unknown callee) 0x2070c` | `0x10ca74` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62d74 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+16]` | `0x0` | `0x0` |
| `bl` at 0x62d88 | `(unknown callee) 0x2070c` | `0x10cd0c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62ddc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+20]` | `?` | `?` |
| `bl` at 0x62df0 | `(unknown callee) 0x2070c` | `0x10cd30` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62e44 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+16]` | `?` | `?` |
| `bl` at 0x62e58 | `(unknown callee) 0x2070c` | `0x10cd0c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62e8c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+20]` | `0x0` | `0x0` |
| `bl` at 0x62ea0 | `(unknown callee) 0x2070c` | `0x10cd30` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62ee0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+4]` | `?` | `?` |
| `bl` at 0x62ef4 | `(unknown callee) 0x2070c` | `0x10cab4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x62f60 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-112]+4]` | `sp-72` | `sp-76` |
| `bl` at 0x62f74 | `(unknown callee) 0x2070c` | `0x10cad8` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x62fa4 | `(unknown callee) 0x63bc0` | `0xa` | `?` | `?` | `[sp-140]` |
| `bl` at 0x62fbc | `(unknown callee) 0x2070c` | `0x10caf0` | `?` | `?` | `?` |
| `mov` at 0x63028 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-72` | `sp-76` |
| `bl` at 0x6303c | `(unknown callee) 0x2070c` | `0x10cb04` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x6306c | `(unknown callee) 0x63bc0` | `0xa` | `?` | `?` | `[sp-140]` |
| `bl` at 0x63084 | `(unknown callee) 0x2070c` | `0x10cb1c` | `?` | `?` | `?` |
| `mov` at 0x630c8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `sp-80` | `sp-84` |
| `bl` at 0x630dc | `(unknown callee) 0x20240` | `0x10cb30` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63114 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `sp-88` | `sp-92` |
| `bl` at 0x63128 | `(unknown callee) 0x20240` | `0x10cb30` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x632a4 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `sp-96` | `sp-100` |
| `bl` at 0x632b8 | `(unknown callee) 0x20240` | `0x10cb30` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x632f0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `sp-104` | `sp-108` |
| `bl` at 0x63304 | `(unknown callee) 0x20240` | `0x10cb30` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x63328 | `(unknown callee) 0x20240` | `0x10cb44` | `?` | `?` | `?` |
| `bl` at 0x6334c | `(unknown callee) 0x20240` | `0x10cb54` | `?` | `?` | `?` |
| `bl` at 0x6337c | `(unknown callee) 0x2070c` | `0x10cb68` | `?` | `?` | `?` |
| `bl` at 0x63430 | `(unknown callee) 0x2070c` | `0x10cd54` | `[sp-72]` | `[sp-76]` | `?` |
| `mov` at 0x63508 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-168` | `sp-172` |
| `bl` at 0x6351c | `(unknown callee) 0x2070c` | `0x10cb04` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63550 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+4]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63564 | `(unknown callee) 0x2070c` | `0x10cb80` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63598 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+8]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x635ac | `(unknown callee) 0x2070c` | `0x10cd78` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x635e0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+12]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x635f4 | `(unknown callee) 0x2070c` | `0x10cd8c` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63628 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+16]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x6363c | `(unknown callee) 0x2070c` | `0x10cda0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63670 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+20]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x63684 | `(unknown callee) 0x2070c` | `0x10cdb4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x636b0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]]` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x636c4 | `(unknown callee) 0x2070c` | `0x10cb94` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x636fc | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+28]` | `sp-176` | `sp-180` |
| `bl` at 0x63710 | `(unknown callee) 0x2070c` | `0x10cb04` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63744 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+40]` | `[sp-176]` | `[sp-180]` |
| `bl` at 0x63758 | `(unknown callee) 0x2070c` | `0x10cbac` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x6378c | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+36]` | `[sp-168]` | `[sp-172]` |
| `bl` at 0x637a0 | `(unknown callee) 0x2070c` | `0x10cbc8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x637d8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+24]` | `sp-168` | `sp-172` |
| `bl` at 0x637ec | `(unknown callee) 0x2070c` | `0x10cb04` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x63848 | `(unknown callee) 0x2070c` | `0x10cbe0` | `?` | `?` | `[sp-140]` |
| `mov` at 0x63894 | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90010000` | `sp-72` | `sp-76` |
| `bl` at 0x638a8 | `(unknown callee) 0x2070c` | `0x10cdc8` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x638dc | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90010000` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x638f0 | `(unknown callee) 0x2070c` | `0x10cde0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63918 | `(unknown callee) (unknown target)` | `[sp-28]` | `0x90071100` | `0x0` | `0x0` |
| `bl` at 0x6392c | `(unknown callee) 0x2070c` | `0x10cde0` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x63960 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-120]+32]` | `[sp-56]` | `[sp-60]` |
| `bl` at 0x63974 | `(unknown callee) 0x2070c` | `0x10cbf4` | `?` | `?` | `ret(unknown)` |
| `mov` at 0x639f8 | `(unknown callee) (unknown target)` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x63a0c | `(unknown callee) 0x2070c` | `0x10cc18` | `?` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6222c | `[0x148448] == 0x0` | `[0x148448] != 0x0` |
| `beq` at 0x62278 | `[sp-188] == 0x4` | `[sp-188] != 0x4` |
| `bgt` at 0x62284 | `[sp-188] > 0x4 (signed, 32-bit)` | `[sp-188] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x62290 | `[sp-188] == 0x2` | `[sp-188] != 0x2` |
| `beq` at 0x622a0 | `[sp-188] == 0x7` | `[sp-188] != 0x7` |
| `beq` at 0x622c8 | `[sp-188] == 0x4` | `[sp-188] != 0x4` |
| `bgt` at 0x622d4 | `[sp-188] > 0x4 (signed, 32-bit)` | `[sp-188] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x622e0 | `[sp-188] == 0x2` | `[sp-188] != 0x2` |
| `beq` at 0x622f0 | `[sp-188] == 0x7` | `[sp-188] != 0x7` |
| `beq` at 0x623c4 | `ret(0xe9028) == 0x0` | `ret(0xe9028) != 0x0` |
| `bne` at 0x623e0 | `[sp-152] != 0xfffff7c6` | `[sp-152] == 0xfffff7c6` |
| `beq` at 0x62430 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62494 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x624e4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6252c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62574 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x625bc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62604 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62644 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6268c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x626d4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x626f4 | `[sp-156] != 0x2` | `[sp-156] == 0x2` |
| `bne` at 0x62718 | `[sp-156] != 0x4` | `[sp-156] == 0x4` |
| `beq` at 0x62770 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x6279c | `C == 0` | `not (C == 0)` |
| `beq` at 0x627dc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62868 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blt` at 0x628a4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x628fc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x62908 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x62918 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6292c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x62940 | `[sp-156] == 0x4` | `[sp-156] != 0x4` |
| `bgt` at 0x6294c | `[sp-156] > 0x4 (signed, 32-bit)` | `[sp-156] <= 0x4 (signed, 32-bit)` |
| `beq` at 0x62958 | `[sp-156] == 0x2` | `[sp-156] != 0x2` |
| `beq` at 0x62968 | `[sp-156] == 0x7` | `[sp-156] != 0x7` |
| `beq` at 0x62988 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x629f8 | `[0x148448] == 0x0` | `[0x148448] != 0x0` |
| `bne` at 0x62ab8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x62c30 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62cfc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62d2c | `[sp-24] != 0x1` | `[sp-24] == 0x1` |
| `beq` at 0x62d4c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x62d80 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62de8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62e50 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62e98 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x62eec | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62f1c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x62f30 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x62f6c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x62f9c | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x62fb4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x62fe4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x62ff8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x63034 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x63064 | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x6307c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6309c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x630d4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63120 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x632b0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x632fc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63320 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63344 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63368 | `? == 0xb4` | `? != 0xb4` |
| `beq` at 0x633e0 | `[sp-88] == ?` | `[sp-88] != ?` |
| `beq` at 0x633fc | `[sp-92] == ?` | `[sp-92] != ?` |
| `beq` at 0x63458 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x63514 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6355c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x635a4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x635ec | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63634 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6367c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x636bc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63708 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63750 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63798 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x637e4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x63820 | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x63834 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x63840 | `[sp-140] != 0x0` | `[sp-140] == 0x0` |
| `bne` at 0x63870 | `[sp-156] != 0x4` | `[sp-156] == 0x4` |
| `beq` at 0x638a0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x638e8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x63924 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x6396c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `blo` at 0x63998 | `C == 0` | `not (C == 0)` |
| `beq` at 0x63a04 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63a38 | `r0` | `?` |

## `TRIGTimeStamp`

- `TRIGTimeStamp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbd684 to 0xbe2e8.
  Established. The timestamp path calls `0xc0634` with `0x7`, `0x22`, entry r2 and entry r3 as
  candidates (`bl` at 0xbd724). Another call targets `0xbe2e8` with `0x7`, `0x11`, `[sp-20]` and
  `[sp-24]` (`bl` at 0xbd758). The recorded r0 exit value is `[sp-40]` (`ldmdb` at
  0xbe2e4). Timestamp encoding and units are not established by these operands.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbd694 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbd698 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbd69c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbd6a0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbd6a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd6ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd6b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd6c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd6d0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd6d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd6ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd6f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd6fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd71c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbd720 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd72c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbd730 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd734 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbd738 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd73c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbd740 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd744 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbd750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd754 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd768 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd770 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbd778 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd780 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd788 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd7a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbd7a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd7b4 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbd7b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd7bc | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbd7c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd7c4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbd7c8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd7cc | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbd7d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd7dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd7f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd7f8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbd800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd808 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd810 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd824 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd82c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd834 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd850 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd858 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd874 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbd87c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbd89c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbd8a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd8ac | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbd8b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd8b4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbd8b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd8bc | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbd8c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd8c4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbd8d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd8d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd8e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd8f0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbd8f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd8f8 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbd8fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd900 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbd904 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd908 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbd914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd918 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd92c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd934 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbd938 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd93c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbd940 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd944 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbd948 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd94c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbd958 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd95c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd970 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd978 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbd97c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd980 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbd984 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd988 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbd98c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd990 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbd99c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd9a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd9b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd9bc | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbd9c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbd9c4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbd9c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbd9cc | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbd9d0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbd9d4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbd9e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbd9e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbd9f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbda00 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbda04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbda08 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbda0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbda10 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbda14 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbda18 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbda24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbda28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbda3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbda44 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbda48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbda4c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbda50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbda54 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbda58 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbda5c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbda68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbda6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbda80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbda88 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbda8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbda90 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbda94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbda98 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbda9c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbdaa0 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbdaac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdac4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdacc | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbdad4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdadc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdae4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdaf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdb00 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdb08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdb1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdb24 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdb2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdb40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdb48 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdb50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdb70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdb74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdb80 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdb84 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdb88 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbdb8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdb90 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbdb94 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbdb98 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbdba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdbbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdbc4 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbdbcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdbd4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdbdc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdbf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdbfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdc08 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdc10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdc18 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdc20 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdc3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdc40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdc4c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdc54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdc5c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdc64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdc80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdc84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdc90 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdc98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdca0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdca8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdcc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdcc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdcd4 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdcdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdce4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdcec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdd08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdd0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdd18 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdd20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdd28 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdd30 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdd4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdd50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdd5c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdd64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdd6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdd74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdd90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdd94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdda0 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdda8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbddb0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbddb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbddd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbddd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdde4 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbddec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbddf4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbddfc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbde18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbde1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbde28 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbde30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbde38 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbde40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbde5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbde60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbde6c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbde70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbde74 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbde78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbde7c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbde80 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbde84 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbde90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbde94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdea8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdeb0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbdeb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdec0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdec8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdee4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdee8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdef4 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdefc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdf04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbdf0c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbdf28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbdf2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdf38 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbdf3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdf40 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbdf44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdf48 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbdf4c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbdf50 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbdf5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdf60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdf74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdf7c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbdf80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdf84 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbdf88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdf8c | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbdf90 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbdf94 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbdfa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdfa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdfb8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdfc0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbdfc4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbdfc8 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbdfcc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbdfd0 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbdfd4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbdfd8 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbdfe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbdfe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbdffc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe004 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe00c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe014 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe01c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe03c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe048 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbe04c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe050 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe054 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe058 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe05c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe060 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe06c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe070 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe084 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe08c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe09c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe0a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe0c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe0c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe0d0 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbe0d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe0e0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe0e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe108 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe114 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbe118 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe11c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe120 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe124 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe128 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe12c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe13c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe150 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe158 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe15c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe160 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe164 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe168 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe16c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe170 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe17c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe194 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe19c | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe1a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1a4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe1a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe1ac | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe1b0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe1b4 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe1c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe1c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe1d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1e0 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe1e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe1f0 | store | 4 | `sp -0x28` | `r2+1` |
| `ldr` at 0xbe1f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbe1f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe204 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbe20c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe214 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbe218 | load | 4 | `sp -0x24` | `-` |
| `ldrsb` at 0xbe21c | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbe228 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe230 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe234 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe23c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe244 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe260 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xbe268 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe284 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe288 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe294 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbe298 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe29c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xbe2a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbe2a4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0xbe2a8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbe2ac | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xbe2b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe2bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe2d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbe2d8 | store | 4 | `sp -0x28` | `[sp-28]+1` |
| `ldr` at 0xbe2dc | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xbe2e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbe2e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbe2e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbd724 | `(unknown callee) 0xc0634` | `0x7` | `0x22` | `r2` | `r3` |
| `bl` at 0xbd758 | `(unknown callee) 0xbe2e8` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbd7ac | `(unknown callee) 0xc0634` | `0x6` | `0x22` | `r2` | `r3` |
| `bl` at 0xbd7e0 | `(unknown callee) 0xbe2e8` | `0x6` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbd8a4 | `(unknown callee) 0xc0634` | `0x8` | `0x22` | `r2` | `r3` |
| `bl` at 0xbd8d8 | `(unknown callee) 0xbe2e8` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbd91c | `(unknown callee) 0xbeeec` | `0xa` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbd960 | `(unknown callee) 0xbeeec` | `0xb` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbd9a4 | `(unknown callee) 0xbeeec` | `0xc` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbd9e8 | `(unknown callee) 0xbeeec` | `0xd` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbda2c | `(unknown callee) 0xbeeec` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbda70 | `(unknown callee) 0xbeeec` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbdab4 | `(unknown callee) 0xbeeec` | `0x9` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbdb78 | `(unknown callee) 0xc0634` | `0x5` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdbac | `(unknown callee) 0xbe2e8` | `0x5` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbdc00 | `(unknown callee) 0xc0634` | `0x10` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdc44 | `(unknown callee) 0xc0634` | `0x14` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdc88 | `(unknown callee) 0xc0634` | `0x16` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdccc | `(unknown callee) 0xc0634` | `0x18` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdd10 | `(unknown callee) 0xc0634` | `0x1a` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdd54 | `(unknown callee) 0xc0634` | `0x15` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdd98 | `(unknown callee) 0xc0634` | `0x17` | `0x22` | `r2` | `r3` |
| `bl` at 0xbdddc | `(unknown callee) 0xc0634` | `0x19` | `0x22` | `r2` | `r3` |
| `bl` at 0xbde20 | `(unknown callee) 0xc0634` | `0x1b` | `0x22` | `r2` | `r3` |
| `bl` at 0xbde64 | `(unknown callee) 0xc0634` | `0x7` | `0x23` | `r2` | `r3` |
| `bl` at 0xbde98 | `(unknown callee) 0xbe2e8` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbdeec | `(unknown callee) 0xc0634` | `0x6` | `0x23` | `r2` | `r3` |
| `bl` at 0xbdf30 | `(unknown callee) 0xc0634` | `0x8` | `0x23` | `r2` | `r3` |
| `bl` at 0xbdf64 | `(unknown callee) 0xbe2e8` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbdfa8 | `(unknown callee) 0xbeeec` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbdfec | `(unknown callee) 0xbeeec` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe040 | `(unknown callee) 0xc0634` | `0x7` | `0x24` | `r2` | `r3` |
| `bl` at 0xbe074 | `(unknown callee) 0xbe2e8` | `0x7` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe0c8 | `(unknown callee) 0xc0634` | `0x6` | `0x24` | `r2` | `r3` |
| `bl` at 0xbe10c | `(unknown callee) 0xc0634` | `0x8` | `0x24` | `r2` | `r3` |
| `bl` at 0xbe140 | `(unknown callee) 0xbe2e8` | `0x8` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe184 | `(unknown callee) 0xbeeec` | `0xe` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe1c8 | `(unknown callee) 0xbeeec` | `0xf` | `0x11` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xbe28c | `(unknown callee) 0xc0634` | `0x4` | `0x22` | `[sp-28]` | `[sp-32]` |
| `bl` at 0xbe2c0 | `(unknown callee) 0xbe2e8` | `0x4` | `0x11` | `[sp-20]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbd6c4 | `? == 0xc` | `? != 0xc` |
| `beq` at 0xbd6e8 | `? == 0xb` | `? != 0xb` |
| `beq` at 0xbd70c | `? == 0x9` | `? != 0x9` |
| `bne` at 0xbd764 | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `bne` at 0xbd798 | `? != 0x7` | `? == 0x7` |
| `bne` at 0xbd7ec | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `beq` at 0xbd820 | `? == 0x17` | `? != 0x17` |
| `beq` at 0xbd844 | `? == 0x19` | `? != 0x19` |
| `beq` at 0xbd868 | `? == 0x1a` | `? != 0x1a` |
| `beq` at 0xbd88c | `? == 0x18` | `? != 0x18` |
| `bne` at 0xbd8e4 | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `bne` at 0xbd928 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbd96c | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbd9b0 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbd9f4 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbda38 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbda7c | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbdac0 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `beq` at 0xbdaf4 | `? == 0x2` | `? != 0x2` |
| `beq` at 0xbdb18 | `? == 0x3` | `? != 0x3` |
| `beq` at 0xbdb3c | `? == 0x4` | `? != 0x4` |
| `beq` at 0xbdb60 | `? == 0x5` | `? != 0x5` |
| `bne` at 0xbdbb8 | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `bne` at 0xbdbec | `? != 0x1` | `? == 0x1` |
| `bne` at 0xbdc30 | `? != 0xe` | `? == 0xe` |
| `bne` at 0xbdc74 | `? != 0x10` | `? == 0x10` |
| `bne` at 0xbdcb8 | `? != 0x12` | `? == 0x12` |
| `bne` at 0xbdcfc | `? != 0x14` | `? == 0x14` |
| `bne` at 0xbdd40 | `? != 0xf` | `? == 0xf` |
| `bne` at 0xbdd84 | `? != 0x11` | `? == 0x11` |
| `bne` at 0xbddc8 | `? != 0x13` | `? == 0x13` |
| `bne` at 0xbde0c | `? != 0x15` | `? == 0x15` |
| `bne` at 0xbde50 | `? != 0xa` | `? == 0xa` |
| `bne` at 0xbdea4 | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `bne` at 0xbded8 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xbdf1c | `? != 0x16` | `? == 0x16` |
| `bne` at 0xbdf70 | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `bne` at 0xbdfb4 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbdff8 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbe02c | `? != 0xd` | `? == 0xd` |
| `bne` at 0xbe080 | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `bne` at 0xbe0b4 | `? != 0x8` | `? == 0x8` |
| `bne` at 0xbe0f8 | `? != 0x1b` | `? == 0x1b` |
| `bne` at 0xbe14c | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |
| `bne` at 0xbe190 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bne` at 0xbe1d4 | `ret(0xbeeec) != 0x0` | `ret(0xbeeec) == 0x0` |
| `bhi` at 0xbe200 | `[sp-40] > [sp-28] (unsigned, 32-bit)` | `[sp-40] <= [sp-28] (unsigned, 32-bit)` |
| `blt` at 0xbe224 | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `ble` at 0xbe254 | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xbe278 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bne` at 0xbe2cc | `ret(0xbe2e8) != 0x0` | `ret(0xbe2e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbe2e4 | `r0` | `[sp-40]` |

## `TRIGSetInFIRST`

- `TRIGSetInFIRST`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbe2e8 to 0xbeeec.
  Established. An initial halfword read has an unknown effective address (`ldrh` at 0xbe320). After
  two comparisons with unknown operands, the call to `0xc0634` carries entry r0, `0x12`, `[sp]` and
  `[sp+4]` as candidates (`bne` at 0xbe360, `bne` at 0xbe3ac, `bl` at 0xbe3c0). The subsequent
  unsigned comparison branches when `ret(0xc0634) > [sp]` and falls through otherwise (`bhi` at
  0xbe3d8). A later call to `0xbd340` has `[sp-44]`, `0x0`, `0x0` and unknown as candidates (`bl` at
  0xbe4f4). The FIRST field interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbe2f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbe2fc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbe300 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbe304 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xbe30c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xbe310 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe318 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbe320 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe33c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbe344 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe364 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe36c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbe374 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe390 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xbe398 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe3a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbe3b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbe3b8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe3bc | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbe3c8 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbe3cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe3d0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe3dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe3e4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbe3ec | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbe3f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe3f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe400 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe40c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe41c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe428 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe438 | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe43c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe444 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbe44c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe460 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe468 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbe470 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe48c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe490 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbe49c | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbe4a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe4a4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe4b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe4b8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbe4c0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbe4c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe4cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbe4d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe4e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe4fc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe500 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe508 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbe510 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe52c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbe534 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe550 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe554 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbe560 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbe564 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe568 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe574 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbe580 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe588 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbe590 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbe594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe59c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe5a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe5b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe5b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe5c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe5cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe5dc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe5e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe5ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbe5f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe608 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe61c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe620 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe628 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbe630 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe64c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbe654 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe670 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe674 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbe680 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbe684 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe688 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe694 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbe6a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe6a8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbe6b0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbe6b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe6bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe6c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe6d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe6d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe6e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe6ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe6fc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe704 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe70c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbe714 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe728 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe73c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe740 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe748 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbe750 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe76c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbe774 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe790 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe794 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbe7a0 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbe7a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe7a8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe7b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbe7c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe7c8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbe7d0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbe7d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe7dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe7e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe7f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe7f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe800 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe80c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe81c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe824 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe82c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbe834 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe848 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe85c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe860 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe868 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbe870 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe88c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbe894 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe8b0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe8b4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbe8c0 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbe8c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe8c8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe8d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbe8e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe8e8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbe8f0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbe8f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe8fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe904 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe910 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbe920 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbe92c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe93c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe944 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe94c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbe954 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbe968 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbe97c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbe980 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbe988 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbe990 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe9ac | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbe9b4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbe9d0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe9d4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbe9e0 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbe9e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbe9e8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbe9f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbea00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbea08 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbea10 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbea14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbea1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbea24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbea30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbea38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbea40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbea4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbea5c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbea64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbea6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbea74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbea88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbea9c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbeaa0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeaa8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbeab0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbeacc | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbead4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbeaf0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbeaf4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbeb00 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbeb04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbeb08 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbeb14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbeb20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbeb28 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbeb30 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbeb34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeb3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeb44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbeb50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeb58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeb60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbeb6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbeb7c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbeb84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeb8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbeb94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbeba8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbebbc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbebc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbebc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbebd0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbebec | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbebf4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbec10 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbec14 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbec20 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbec24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbec28 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbec34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbec40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbec48 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbec50 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbec54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbec5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbec64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbec70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbec78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbec80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbec8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbec9c | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbeca4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbecac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbecb4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbecc8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbecdc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbece0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbece8 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbecf0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbed0c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbed14 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbed30 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbed34 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbed40 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbed44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbed48 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbed54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbed60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbed68 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbed70 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbed74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbed7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbed84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbed90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbed98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeda0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbedac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbedbc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbedc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbedcc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xbedd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbede8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbedfc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbee00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbee08 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbee10 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbee2c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbee34 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbee50 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbee54 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbee60 | store | 4 | `sp -0x24` | `ret(0xc0634)` |
| `ldr` at 0xbee64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbee68 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbee74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbee80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbee88 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbee90 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbee94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbee9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeea4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbeeb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbeeb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbeec0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbeecc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbeedc | store | 4 | `sp -0x28` | `ret(0xbd340)` |
| `ldr` at 0xbeee0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xbeee8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbeee8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbeee8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbe3c0 | `(unknown callee) 0xc0634` | `r0` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbe430 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbe494 | `(unknown callee) 0xc0634` | `0x4` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbe4f4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x0` | `0x0` | `?` |
| `bl` at 0xbe558 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbe5d4 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbe614 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x1` | `0x1` | `?` |
| `bl` at 0xbe678 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbe6f4 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbe734 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x2` | `0x2` | `?` |
| `bl` at 0xbe798 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbe814 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbe854 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x3` | `0x3` | `?` |
| `bl` at 0xbe8b8 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbe934 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbe974 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x4` | `0x4` | `?` |
| `bl` at 0xbe9d8 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbea54 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbea94 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x5` | `0x5` | `?` |
| `bl` at 0xbeaf8 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbeb74 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbebb4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x6` | `0x6` | `?` |
| `bl` at 0xbec18 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbec94 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbecd4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x7` | `0x7` | `?` |
| `bl` at 0xbed38 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbedb4 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xbedf4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x8` | `0x8` | `?` |
| `bl` at 0xbee58 | `(unknown callee) 0xc0634` | `?` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbeed4 | `(unknown callee) 0xbd340` | `[sp-44]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbe360 | `? != r0` | `? == r0` |
| `bne` at 0xbe3ac | `? != r1` | `? == r1` |
| `bhi` at 0xbe3d8 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `ble` at 0xbe45c | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0xbe480 | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bhi` at 0xbe4ac | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbe4e4 | `? != 0x1d` | `? == 0x1d` |
| `bhi` at 0xbe570 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbe57c | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbe604 | `? != 0x1e` | `? == 0x1e` |
| `bhi` at 0xbe690 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbe69c | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbe724 | `? != 0x1f` | `? == 0x1f` |
| `bhi` at 0xbe7b0 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbe7bc | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbe844 | `? != 0x20` | `? == 0x20` |
| `bhi` at 0xbe8d0 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbe8dc | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbe964 | `? != 0x21` | `? == 0x21` |
| `bhi` at 0xbe9f0 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbe9fc | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbea84 | `? != 0x22` | `? == 0x22` |
| `bhi` at 0xbeb10 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbeb1c | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbeba4 | `? != 0x23` | `? == 0x23` |
| `bhi` at 0xbec30 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbec3c | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbecc4 | `? != 0x24` | `? == 0x24` |
| `bhi` at 0xbed50 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbed5c | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xbede4 | `? != 0x1c` | `? == 0x1c` |
| `bhi` at 0xbee70 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `bne` at 0xbee7c | `[sp-40] != 0x1` | `[sp-40] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbeee8 | `r0` | `?` |

## `TRIGSetInXNFIRST`

- `TRIGSetInXNFIRST`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbeeec to 0xbf4a4.
  Established. This body saves `0x12fcd0` at `sp -0x28` (`str` at 0xbef10); its call to `0xc0634`
  supplies entry r0, `0x12`, `[sp]` and `[sp+4]` as candidates (`bl` at 0xbefcc). The unsigned
  result comparison takes the branch for `ret(0xc0634) > [sp]` and falls through for at most `[sp]`
  (`bhi` at 0xbefe4). A later call to `0xbd340` has `[sp-52]`, `[sp-36]`, `[sp-36]` and unknown as
  candidates (`bl` at 0xbf488). The lookup table and XNFIRST semantics remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbeefc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbef00 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbef04 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbef08 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbef0c | load | 4 | `abs 0xbf4a0` | `-` |
| `str` at 0xbef10 | store | 4 | `sp -0x28` | `0x12fcd0` |
| `str` at 0xbef18 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xbef1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbef24 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbef2c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbef48 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbef50 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbef64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbef70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbef78 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbef80 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbef9c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xbefa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbefb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbefbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbefc4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbefc8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbefd4 | store | 4 | `sp -0x2c` | `ret(0xc0634)` |
| `ldr` at 0xbefd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbefdc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbefe8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbeff0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbeff8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xbeffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf004 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf00c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf020 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf028 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf034 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xbf044 | store | 4 | `sp -0x30` | `ret(0xbd340)` |
| `ldr` at 0xbf048 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbf058 | store | 4 | `sp -0x24` | `0x80` |
| `ldr` at 0xbf05c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf064 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf06c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf088 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf090 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf0ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf0b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf0bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf0d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf0e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf0e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf0e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf0f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf0f8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf114 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf11c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf138 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf140 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf148 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf164 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf16c | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf170 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf174 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf17c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf184 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf1a0 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf1a8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf1c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf1cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf1d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf1f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf1f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf1fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf200 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf208 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf210 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf22c | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf234 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf250 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf27c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf284 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf288 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf28c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf294 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf29c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf2b8 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf2c0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf2dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf2e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf2ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf308 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf310 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf314 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf318 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf320 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf328 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf344 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf34c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf368 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf378 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf394 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf39c | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf3a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf3a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf3ac | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xbf3b4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf3d0 | load | 4 | `sp +0x4` | `-` |
| `ldrh` at 0xbf3d8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf3f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf3fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf404 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf420 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf428 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbf42c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbf430 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf444 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf448 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf454 | store | 4 | `sp -0x2c` | `ret(0xc0634)` |
| `ldr` at 0xbf458 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf45c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbf468 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf470 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbf478 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xbf47c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbf480 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbf484 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbf490 | store | 4 | `sp -0x30` | `ret(0xbd340)` |
| `ldr` at 0xbf494 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xbf49c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbf49c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbf49c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbefcc | `(unknown callee) 0xc0634` | `r0` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf03c | `(unknown callee) 0xbd340` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0xbf44c | `(unknown callee) 0xc0634` | `0x8` | `0x12` | `[sp]` | `[sp+4]` |
| `bl` at 0xbf488 | `(unknown callee) 0xbd340` | `[sp-52]` | `[sp-36]` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbef6c | `? != r0` | `? == r0` |
| `bne` at 0xbefb8 | `? != r1` | `? == r1` |
| `bhi` at 0xbefe4 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |
| `beq` at 0xbf050 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbf0a8 | `? != 0xa` | `? == 0xa` |
| `bne` at 0xbf134 | `? != 0xb` | `? == 0xb` |
| `bne` at 0xbf1c0 | `? != 0xc` | `? == 0xc` |
| `bne` at 0xbf24c | `? != 0xd` | `? == 0xd` |
| `bne` at 0xbf2d8 | `? != 0xe` | `? == 0xe` |
| `bne` at 0xbf364 | `? != 0xf` | `? == 0xf` |
| `bne` at 0xbf3f0 | `? != 0x9` | `? == 0x9` |
| `bhi` at 0xbf438 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xbf464 | `ret(0xc0634) > [sp] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbf49c | `r0` | `?` |

## `TRIGReadDetails`

- `TRIGReadDetails`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbf4a4 to 0xc0504.
  Established. The initial branch falls through when entry r0 is zero, reaching a word store of
  `0xffff` at `sp -0x24` (`bne` at 0xbf4d4, `str` at 0xbf4dc). Nonzero takes the branch past that store (`bne` at 0xbf4d4, `str` at
  0xbf4dc). Calls include `0xc0634` with candidates `0x7`, `0x15`, `[sp-32]` and `[sp]` (`bl` at
  0xbf604), then on a known path `0xbd340` with `[sp-44]`, `0x2a`, `0x20` and unknown (`bl` at
  0xbf640). The detail-selector meaning and final r0 value are unknown (`ldmdb` at 0xc0500).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbf4b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbf4b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbf4bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbf4c0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xbf4c8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbf4cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf4d8 | load | 4 | `abs 0xc0370` | `-` |
| `str` at 0xbf4dc | store | 4 | `sp -0x24` | `0xffff` |
| `ldr` at 0xbf4e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbf4f0 | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0xbf4f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbf504 | store | 4 | `sp -0x24` | `0x7` |
| `ldr` at 0xbf508 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbf518 | store | 4 | `sp -0x24` | `0x3` |
| `ldr` at 0xbf51c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf528 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf540 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbf554 | store | 4 | `sp -0x24` | `0x7f` |
| `ldr` at 0xbf558 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf560 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbf568 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf57c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbf58c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xbf590 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf5a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf5a4 | load | 4 | `abs 0xc0374` | `-` |
| `ldr` at 0xbf5b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf5c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbf5c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbf5cc | load | 4 | `abs 0xc0378` | `-` |
| `str` at 0xbf5e4 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xbf5e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbf5fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf600 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbf60c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbf610 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf614 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf620 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf628 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf630 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf634 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf648 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbf658 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf65c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbf668 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbf66c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf670 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf67c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf684 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf68c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf690 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf6a4 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbf6a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf6b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbf6b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf6cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf6d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbf6dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf6f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf708 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf70c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbf718 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbf71c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf720 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf72c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf734 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf73c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf740 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbf75c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf770 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbf774 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf77c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbf784 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf7ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf7b0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbf7bc | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbf7c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf7c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf7d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf7d8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf7e0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf7e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf7f8 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbf7fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf804 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbf80c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf834 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf838 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbf844 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbf848 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf84c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf858 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf860 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf868 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf86c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf880 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbf884 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf88c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbf894 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbf8a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbf8bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf8c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbf8cc | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbf8d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf8d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbf8e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbf8e8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbf8f0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbf8f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbf908 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbf90c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf914 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xbf91c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf93c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf944 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xbf94c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf968 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xbf970 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf998 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xbf9a0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf9bc | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xbf9c4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbf9e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbf9ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbf9f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbfa10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfa14 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbfa20 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbfa24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfa28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfa34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfa3c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbfa44 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbfa48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfa64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfa70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfa84 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfa88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfa94 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfaa8 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfaac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfab8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfacc | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfadc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfafc | store | 4 | `sp -0x24` | `ret(0xc0504)` |
| `ldr` at 0xbfb0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfb10 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbfb1c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbfb20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfb24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfb30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfb38 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbfb40 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbfb44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfb60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfb6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfb80 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfb84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfba0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfbb0 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfbb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfbc4 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfbc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfbd8 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfbdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfbec | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfbf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfc00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfc14 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfc18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfc24 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfc38 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfc3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfc48 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfc68 | store | 4 | `sp -0x24` | `ret(0xc0504)` |
| `ldr` at 0xbfc70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfc78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbfc80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbfc9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfca0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbfcac | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbfcb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfcb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfcc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfcc8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbfcd0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbfcd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfcf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfcfc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfd10 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfd14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfd20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfd34 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfd38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfd44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfd58 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfd68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfd6c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbfd78 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbfd7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfd80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbfd8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbfd94 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbfd9c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbfda0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfdbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfdc8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfddc | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfde0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbfdfc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfe0c | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfe10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfe20 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfe24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfe34 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfe38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfe48 | store | 4 | `sp -0x24` | `0x77` |
| `ldr` at 0xbfe50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfe5c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfe70 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfe74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfe80 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbfe94 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfe98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfea8 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xbfeac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfebc | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xbfec0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfed0 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xbfed4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbfee4 | store | 4 | `sp -0x24` | `0x78` |
| `ldr` at 0xbfeec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbfef4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xbfefc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbff18 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xbff20 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbff40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbff48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbff50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xbff6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbff70 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbff7c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xbff80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbff84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbff90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbff98 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbffa0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xbffa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbffb0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbffc8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbffcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbffd8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbffec | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xbfff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbfffc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0010 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc0018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0020 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc0028 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc0044 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0048 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0054 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc0058 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc005c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0068 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0070 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0078 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc007c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0088 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc00a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc00a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc00b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc00c4 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc00c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc00d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc00e8 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc00f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc00f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc0100 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc011c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0120 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc012c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc0130 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0134 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0140 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc014c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0154 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc015c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0160 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0174 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc0178 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0180 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc018c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0190 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc019c | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc01a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc01a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc01b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc01b8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc01c0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc01c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc01d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc01e0 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xbd340))` |
| `ldr` at 0xc01ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc01f0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc01fc | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc0200 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0204 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0210 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0218 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0220 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0230 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0244 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc0248 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0254 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0268 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc0274 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0278 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0284 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc0288 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc028c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0298 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc02a0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc02a8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc02ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc02b8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc02d8 | store | 4 | `sp -0x24` | `ret(0xc0504)` |
| `ldr` at 0xc02dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc02e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0308 | store | 4 | `sp -0x24` | `ret(0xc0504)` |
| `ldr` at 0xc030c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0318 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0338 | store | 4 | `sp -0x24` | `ret(0xc0504)` |
| `ldr` at 0xc033c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0348 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0368 | store | 4 | `sp -0x24` | `ret(0xc0504)` |
| `ldr` at 0xc037c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0384 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc038c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc03a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc03ac | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc03b8 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc03bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc03c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc03cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc03d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc03e0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc03e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc03ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc0400 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc0404 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc040c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc0418 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc041c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0428 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc042c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0430 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc043c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0444 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc044c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc0450 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc0464 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc046c | store | 4 | `sp -0x24` | `([sp-36] + ret(0xbd340))` |
| `ldr` at 0xc0478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc047c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc0488 | store | 4 | `sp -0x28` | `ret(0xc0634)` |
| `ldr` at 0xc048c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0490 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc049c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc04a4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc04ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc04b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc04bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc04d0 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc04d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc04e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc04f4 | store | 4 | `sp -0x24` | `ret(0xbd340)` |
| `ldr` at 0xc04f8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc0500 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0500 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0500 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbf5a8 | `(unknown callee) 0x1aff8` | `?` | `0x12ceb8` | `?` | `?` |
| `bl` at 0xbf5d0 | `(unknown callee) 0x1aff8` | `?` | `0x12cec4` | `?` | `?` |
| `bl` at 0xbf604 | `(unknown callee) 0xc0634` | `0x7` | `0x15` | `[sp-32]` | `[sp]` |
| `bl` at 0xbf640 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x2a` | `0x20` | `?` |
| `bl` at 0xbf660 | `(unknown callee) 0xc0634` | `0x7` | `0x14` | `[sp-32]` | `[sp]` |
| `bl` at 0xbf69c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x30` | `0x26` | `?` |
| `bl` at 0xbf710 | `(unknown callee) 0xc0634` | `0x5` | `0x14` | `?` | `[sp]` |
| `bl` at 0xbf74c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x3f` | `0x3f` | `?` |
| `bl` at 0xbf768 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x25` | `0x18` | `ret(0xbd340)` |
| `bl` at 0xbf7b4 | `(unknown callee) 0xc0634` | `0xa` | `0x15` | `?` | `[sp]` |
| `bl` at 0xbf7f0 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xbf83c | `(unknown callee) 0xc0634` | `0xc` | `0x15` | `?` | `[sp]` |
| `bl` at 0xbf878 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xbf8c4 | `(unknown callee) 0xc0634` | `0x9` | `0x15` | `?` | `[sp]` |
| `bl` at 0xbf900 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x1a` | `0x10` | `?` |
| `bl` at 0xbfa18 | `(unknown callee) 0xc0634` | `0x7` | `0x1a` | `?` | `[sp]` |
| `bl` at 0xbfa54 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x37` | `0x30` | `?` |
| `bl` at 0xbfa7c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x2d` | `0x23` | `[sp-20]` |
| `bl` at 0xbfaa0 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x4` | `0x4` | `[sp-20]` |
| `bl` at 0xbfac4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x20` | `0x1f` | `[sp-20]` |
| `bl` at 0xbfae8 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x37` | `0x30` | `[sp-20]` |
| `bl` at 0xbfaf4 | `(unknown callee) 0xc0504` | `ret(0xbd340)` | `?` | `?` | `ret(0xbd340)` |
| `bl` at 0xbfb14 | `(unknown callee) 0xc0634` | `0x7` | `0x25` | `[sp-32]` | `[sp]` |
| `bl` at 0xbfb50 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x3f` | `0x38` | `?` |
| `bl` at 0xbfb78 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x30` | `0x26` | `[sp-20]` |
| `bl` at 0xbfb90 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x25` | `0x20` | `?` |
| `bl` at 0xbfc0c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xbfc30 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xbfc54 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x3a` | `0x33` | `[sp-20]` |
| `bl` at 0xbfc60 | `(unknown callee) 0xc0504` | `ret(0xbd340)` | `?` | `?` | `ret(0xbd340)` |
| `bl` at 0xbfca4 | `(unknown callee) 0xc0634` | `0x7` | `0x1e` | `?` | `[sp]` |
| `bl` at 0xbfce0 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x37` | `0x30` | `?` |
| `bl` at 0xbfd08 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x2d` | `0x23` | `[sp-20]` |
| `bl` at 0xbfd2c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x4` | `0x4` | `[sp-20]` |
| `bl` at 0xbfd50 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x20` | `0x1f` | `[sp-20]` |
| `bl` at 0xbfd70 | `(unknown callee) 0xc0634` | `0x7` | `0x25` | `[sp-32]` | `[sp]` |
| `bl` at 0xbfdac | `(unknown callee) 0xbd340` | `[sp-44]` | `0x3f` | `0x38` | `?` |
| `bl` at 0xbfdd4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x30` | `0x26` | `[sp-20]` |
| `bl` at 0xbfdec | `(unknown callee) 0xbd340` | `[sp-44]` | `0x25` | `0x20` | `?` |
| `bl` at 0xbfe68 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xbfe8c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xbff74 | `(unknown callee) 0xc0634` | `0x6` | `0x1a` | `?` | `[sp]` |
| `bl` at 0xbffbc | `(unknown callee) 0xbd340` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xbffe4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xc0008 | `(unknown callee) 0xbd340` | `[sp-44]` | `0xb` | `0xb` | `[sp-20]` |
| `bl` at 0xc004c | `(unknown callee) 0xc0634` | `0x6` | `0x1e` | `?` | `[sp]` |
| `bl` at 0xc0094 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x7` | `0x7` | `[sp-20]` |
| `bl` at 0xc00bc | `(unknown callee) 0xbd340` | `[sp-44]` | `0x23` | `0x22` | `[sp-20]` |
| `bl` at 0xc00e0 | `(unknown callee) 0xbd340` | `[sp-44]` | `0xb` | `0xb` | `[sp-20]` |
| `bl` at 0xc0124 | `(unknown callee) 0xc0634` | `0x8` | `0x1d` | `?` | `[sp]` |
| `bl` at 0xc016c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x4` | `0x0` | `?` |
| `bl` at 0xc0194 | `(unknown callee) 0xc0634` | `0x8` | `0x1c` | `[sp-32]` | `[sp]` |
| `bl` at 0xc01d0 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x3f` | `0x39` | `?` |
| `bl` at 0xc01f4 | `(unknown callee) 0xc0634` | `0x8` | `0x1c` | `[sp-32]` | `[sp]` |
| `bl` at 0xc023c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x1a` | `0x1a` | `[sp-20]` |
| `bl` at 0xc0260 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x36` | `0x35` | `[sp-20]` |
| `bl` at 0xc027c | `(unknown callee) 0xc0634` | `0x8` | `0x1a` | `[sp-32]` | `[sp]` |
| `bl` at 0xc02c4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x7` | `0x0` | `[sp-20]` |
| `bl` at 0xc02d0 | `(unknown callee) 0xc0504` | `ret(0xbd340)` | `?` | `?` | `ret(0xbd340)` |
| `bl` at 0xc02f4 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x17` | `0x10` | `[sp-20]` |
| `bl` at 0xc0300 | `(unknown callee) 0xc0504` | `ret(0xbd340)` | `?` | `?` | `ret(0xbd340)` |
| `bl` at 0xc0324 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x27` | `0x20` | `[sp-20]` |
| `bl` at 0xc0330 | `(unknown callee) 0xc0504` | `ret(0xbd340)` | `?` | `?` | `ret(0xbd340)` |
| `bl` at 0xc0354 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x37` | `0x30` | `[sp-20]` |
| `bl` at 0xc0360 | `(unknown callee) 0xc0504` | `ret(0xbd340)` | `?` | `?` | `ret(0xbd340)` |
| `bl` at 0xc03b0 | `(unknown callee) 0xc0634` | `0x8` | `0x21` | `?` | `[sp]` |
| `bl` at 0xc03f8 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x4` | `0x0` | `?` |
| `bl` at 0xc0420 | `(unknown callee) 0xc0634` | `0x8` | `0x20` | `[sp-32]` | `[sp]` |
| `bl` at 0xc045c | `(unknown callee) 0xbd340` | `[sp-44]` | `0x3f` | `0x39` | `?` |
| `bl` at 0xc0480 | `(unknown callee) 0xc0634` | `0x8` | `0x20` | `[sp-32]` | `[sp]` |
| `bl` at 0xc04c8 | `(unknown callee) 0xbd340` | `[sp-44]` | `0x1a` | `0x1a` | `[sp-20]` |
| `bl` at 0xc04ec | `(unknown callee) 0xbd340` | `[sp-44]` | `0x36` | `0x35` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbf4d4 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xbf4e8 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xbf4fc | `r0 != 0x2` | `r0 == 0x2` |
| `bne` at 0xbf510 | `r0 != 0x7` | `r0 == 0x7` |
| `beq` at 0xbf524 | `r0 == 0x3` | `r0 != 0x3` |
| `beq` at 0xbf530 | `r0 == 0x4` | `r0 != 0x4` |
| `beq` at 0xbf53c | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0xbf548 | `r0 == 0x6` | `r0 != 0x6` |
| `bne` at 0xbf578 | `? != 0xb` | `? == 0xb` |
| `bne` at 0xbf584 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xbf5b4 | `ret(0x1aff8) == 0x0` | `ret(0x1aff8) != 0x0` |
| `bne` at 0xbf5dc | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xbf5f0 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xbf61c | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xbf678 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xbf6c8 | `? == 0x5` | `? != 0x5` |
| `beq` at 0xbf6ec | `? == 0x4` | `? != 0x4` |
| `bne` at 0xbf6fc | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xbf728 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xbf758 | `ret(0xbd340) == 0x0` | `ret(0xbd340) != 0x0` |
| `bne` at 0xbf794 | `? != 0x19` | `? == 0x19` |
| `bne` at 0xbf7a0 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xbf7cc | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xbf81c | `? != 0x1a` | `? == 0x1a` |
| `bne` at 0xbf828 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xbf854 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xbf8a4 | `? != 0x18` | `? == 0x18` |
| `bne` at 0xbf8b0 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xbf8dc | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xbf938 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbf988 | `? != 0x4` | `? == 0x4` |
| `bne` at 0xbf9dc | `? != 0x8` | `? == 0x8` |
| `bne` at 0xbfa04 | `? != 0xa` | `? == 0xa` |
| `bhi` at 0xbfa30 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xbfa60 | `ret(0xbd340) == 0xff` | `ret(0xbd340) != 0xff` |
| `bne` at 0xbfa6c | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xbfa90 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xbfab4 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xbfad8 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bhi` at 0xbfb2c | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xbfb5c | `ret(0xbd340) != 0xaa` | `ret(0xbd340) == 0xaa` |
| `bne` at 0xbfb68 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xbfb9c | `ret(0xbd340) != 0x8` | `ret(0xbd340) == 0x8` |
| `bne` at 0xbfba8 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xbfbbc | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xbfbd0 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xbfbe4 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xbfbfc | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xbfc20 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xbfc44 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xbfc90 | `? != 0xd` | `? == 0xd` |
| `bhi` at 0xbfcbc | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `beq` at 0xbfcec | `ret(0xbd340) == 0xff` | `ret(0xbd340) != 0xff` |
| `bne` at 0xbfcf8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xbfd1c | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xbfd40 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bhi` at 0xbfd88 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xbfdb8 | `ret(0xbd340) != 0xaa` | `ret(0xbd340) == 0xaa` |
| `bne` at 0xbfdc4 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0xbfdf8 | `ret(0xbd340) != 0x8` | `ret(0xbd340) == 0x8` |
| `bne` at 0xbfe04 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xbfe18 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xbfe2c | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xbfe40 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xbfe58 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xbfe7c | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xbfea0 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xbfeb4 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xbfec8 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xbfedc | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xbff38 | `? != 0x1c` | `? == 0x1c` |
| `bne` at 0xbff60 | `? != 0x6` | `? == 0x6` |
| `bhi` at 0xbff8c | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xbffac | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xbffd4 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xbfff8 | `[sp-20] != 0x7` | `[sp-20] == 0x7` |
| `bne` at 0xc0038 | `? != 0x8` | `? == 0x8` |
| `bhi` at 0xc0064 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0084 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc00ac | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bne` at 0xc00d0 | `[sp-20] != 0x7` | `[sp-20] == 0x7` |
| `bne` at 0xc0110 | `? != 0x16` | `? == 0x16` |
| `bhi` at 0xc013c | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc0148 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhi` at 0xc01ac | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xc020c | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc022c | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc0250 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `bhi` at 0xc0294 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc02b4 | `[sp-20] != 0x3` | `[sp-20] == 0x3` |
| `bne` at 0xc02e4 | `[sp-20] != 0x4` | `[sp-20] == 0x4` |
| `bne` at 0xc0314 | `[sp-20] != 0x5` | `[sp-20] == 0x5` |
| `bne` at 0xc0344 | `[sp-20] != 0x6` | `[sp-20] == 0x6` |
| `bne` at 0xc039c | `? != 0x1b` | `? == 0x1b` |
| `bhi` at 0xc03c8 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc03d4 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhi` at 0xc0438 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bhi` at 0xc0498 | `ret(0xc0634) > [sp-32] (unsigned, 32-bit)` | `ret(0xc0634) <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xc04b8 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xc04dc | `[sp-20] != 0x2` | `[sp-20] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0500 | `r0` | `?` |
