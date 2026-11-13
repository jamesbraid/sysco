# ARM Linux 2.6 SHUB configuration routines in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-shub-configuration-facts.txt) retain all tool sections.

## `SHUB_Init`

- `SHUB_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x647a8 to 0x64c58.
  Established. A byte store writes the low eight bits of entry r1 to absolute address 0x1a5ca8 (`strb` at 0x647d8).
  A word store writes [sp-20] to absolute address 0x1a5cac (`str` at 0x647e4).
  A later halfword store places 0x320 at absolute address 0x31a68c (`strh` at 0x64b74).
  The result from unnamed 0x13ba30 is stored as a word at entry sp minus 0x2c (`bl` at 0x64bbc, `str` at 0x64bc4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x647b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x647bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x647c0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x647c4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x647c8 | load | 4 | `sp +0x10` | `-` |
| `strb` at 0x647cc | store | 1 | `sp -0x21` | `([sp+16] & 0xff)` |
| `ldr` at 0x647d0 | load | 4 | `abs 0x64c30` | `-` |
| `ldr` at 0x647d4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x647d8 | store | 1 | `abs 0x1a5ca8` | `(r1 & 0xff)` |
| `ldr` at 0x647dc | load | 4 | `abs 0x64c34` | `-` |
| `ldr` at 0x647e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x647e4 | store | 4 | `abs 0x1a5cac` | `[sp-20]` |
| `ldr` at 0x647e8 | load | 4 | `abs 0x64c38` | `-` |
| `ldr` at 0x647ec | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x647f0 | store | 1 | `abs 0x1a5cb0` | `([sp-28] & 0xff)` |
| `ldr` at 0x647f4 | load | 4 | `abs 0x64c3c` | `-` |
| `ldr` at 0x647f8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x647fc | store | 4 | `abs 0x1a5cbc` | `[sp+4]` |
| `ldr` at 0x64800 | load | 4 | `abs 0x64c40` | `-` |
| `ldr` at 0x64804 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x64808 | store | 4 | `abs 0x1a5cc0` | `[sp+8]` |
| `ldr` at 0x6480c | load | 4 | `abs 0x64c44` | `-` |
| `ldr` at 0x64810 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x64814 | store | 4 | `abs 0x1a5cc4` | `[sp+12]` |
| `ldr` at 0x64818 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6481c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x64828 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6482c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64838 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x6483c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64858 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6485c | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64860 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x6487c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x64880 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64884 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x648a0 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x648a4 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x648a8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x648c0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x648c4 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0x648c8 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x648cc | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x648f0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x648f4 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x648f8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x6491c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x64920 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64924 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64948 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x6494c | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64950 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64974 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x64978 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x6497c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x649a0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x649a4 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x649a8 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x649cc | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x649d0 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x649d4 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x649f8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x649fc | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64a00 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64a24 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x64a28 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64a2c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64a50 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x64a54 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64a58 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x64a7c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x64a80 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64a84 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64aa8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x64aac | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64ab0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64ad4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x64ad8 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64adc | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x64b00 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x64b04 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64b08 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x64b2c | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x64b30 | load | 4 | `abs 0x64c48` | `-` |
| `ldr` at 0x64b34 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x64b58 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x64b5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x64b64 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x64b6c | load | 4 | `abs 0x64c4c` | `-` |
| `strh` at 0x64b74 | store | 2 | `abs 0x31a68c` | `0x320` |
| `str` at 0x64b80 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x64b84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64b88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64b94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64ba4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64bac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64bb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64bb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x64bc4 | store | 4 | `sp -0x2c` | `ret(0x13ba30)` |
| `ldr` at 0x64bc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x64bd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x64be0 | load | 4 | `abs 0x64c50` | `-` |
| `str` at 0x64be4 | store | 4 | `sp -0x38` | `0x1631f0` |
| `ldr` at 0x64be8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64bf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x64bfc | store | 4 | `sp -0x34` | `?` |
| `str` at 0x64c00 | store | 4 | `sp -0x30` | `ret(0x219a4)` |
| `ldr` at 0x64c08 | load | 4 | `abs 0x64c54` | `-` |
| `ldr` at 0x64c18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x64c20 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0x64c2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64c2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64c2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64820 | `(unknown callee) 0x64c58` | `[sp-32]` | `[sp]` | `0x1a5cc4` | `[sp+12]` |
| `bl` at 0x64b78 | `(unknown callee) 0x6af40` | `?` | `?` | `0x31a68c` | `0x320` |
| `bl` at 0x64bbc | `(unknown callee) 0x13ba30` | `?` | `?` | `?` | `?` |
| `bl` at 0x64bd8 | `(unknown callee) 0x219a4` | `ret(0x13ba30)` | `?` | `?` | `ret(0x13ba30)` |
| `bl` at 0x64c14 | `(unknown callee) 0x22070` | `0x9` | `0x16320c` | `0xad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x64834 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0x64b90 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x64bd0 | `ret(0x13ba30) == 0x0` | `ret(0x13ba30) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64c2c | `r0` | `?` |

## `SHUB_Set_MemCfg`

- `SHUB_Set_MemCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64c58 to 0x64ce0.
  Established. Word and byte stores write entry r0 and the low eight bits of [sp-24] to absolute addresses 0x1a5cb4 and 0x1a5cb8 respectively (`str` at 0x64c90, `strb` at 0x64c9c).
  The first bne is taken when the separate word load from 0x1a5cb4 is not equal to [sp-28] (`ldr` at 0x64ca4, `bne` at 0x64cb0).
  The call to unnamed 0x6518c has printed r0-r3 candidates entry r0, entry r1, unknown and unknown (`bl` at 0x64ccc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64c68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64c6c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x64c70 | load | 4 | `abs 0x64cd8` | `-` |
| `ldr` at 0x64c74 | load | 4 | `abs 0x1a5cb4` | `-` |
| `str` at 0x64c78 | store | 4 | `sp -0x1c` | `[0x1a5cb4]` |
| `ldr` at 0x64c7c | load | 4 | `abs 0x64cdc` | `-` |
| `ldrb` at 0x64c80 | load | 1 | `abs 0x1a5cb8` | `-` |
| `str` at 0x64c84 | store | 4 | `sp -0x20` | `[0x1a5cb8]:1` |
| `ldr` at 0x64c88 | load | 4 | `abs 0x64cd8` | `-` |
| `ldr` at 0x64c8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x64c90 | store | 4 | `abs 0x1a5cb4` | `r0` |
| `ldr` at 0x64c94 | load | 4 | `abs 0x64cdc` | `-` |
| `ldr` at 0x64c98 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x64c9c | store | 1 | `abs 0x1a5cb8` | `([sp-24] & 0xff)` |
| `ldr` at 0x64ca0 | load | 4 | `abs 0x64cd8` | `-` |
| `ldr` at 0x64ca4 | load | 4 | `abs 0x1a5cb4` | `-` |
| `ldr` at 0x64ca8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64cb4 | load | 4 | `abs 0x64cdc` | `-` |
| `ldrb` at 0x64cb8 | load | 1 | `abs 0x1a5cb8` | `-` |
| `ldr` at 0x64cbc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x64cd4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64cd4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64cd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64ccc | `(unknown callee) 0x6518c` | `r0` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64cb0 | `[0x1a5cb4] != [sp-28]` | `[0x1a5cb4] == [sp-28]` |
| `bne` at 0x64cc4 | `[0x1a5cb8]:1 != [sp-32]` | `[0x1a5cb8]:1 == [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64cd4 | `r0` | `?` |

## `SHUB_MMR_Write`

- `SHUB_MMR_Write`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64ce0 to 0x64d58.
  Established. The call to unnamed 0x13d2b8 has printed r0-r3 candidates unknown, entry r1, entry r2 and entry r3 (`bl` at 0x64d24).
  The bne is taken when that call result is nonzero and falls through when it is zero (`bne` at 0x64d30).
  Separate paths store 1 and 0 as words at entry sp minus 0x24 (`str` at 0x64d38, `str` at 0x64d44).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64cf0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64cf4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x64cf8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x64cfc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x64d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64d08 | load | 4 | `abs 0x64d54` | `-` |
| `ldr` at 0x64d0c | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x64d14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64d1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64d20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64d38 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x64d44 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x64d48 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x64d50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64d50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64d50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64d24 | `(unknown callee) 0x13d2b8` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64d30 | `ret(0x13d2b8) != 0x0` | `ret(0x13d2b8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64d50 | `r0` | `[sp-36]` |

## `SHUB_MMR_Read`

- `SHUB_MMR_Read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64d58 to 0x64dd0.
  Established. A computed word load has an unknown effective address (`ldr` at 0x64d8c).
  The routine calls unnamed 0x13d1a0 with printed r0-r3 candidates unknown, entry r1, entry r2 and entry r3 (`bl` at 0x64d9c);
  its bne takes the branch when that call result is nonzero and falls through when it is zero (`bne` at 0x64da8).
  The recorded return value is [sp-36] in r0 (`ldm` at 0x64dc8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64d68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64d6c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x64d70 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x64d74 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x64d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64d80 | load | 4 | `abs 0x64dcc` | `-` |
| `ldr` at 0x64d84 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x64d8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x64d90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64d94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64d98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64db0 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x64dbc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x64dc0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x64dc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64dc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64dc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64d9c | `(unknown callee) 0x13d1a0` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64da8 | `ret(0x13d1a0) != 0x0` | `ret(0x13d1a0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64dc8 | `r0` | `[sp-36]` |

## `SHUB_NICfg_Override`

- `SHUB_NICfg_Override`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64dd0 to 0x64e20.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x64de0, `str` at 0x64de4).
  The indexed word load retains an unknown effective address (`ldr` at 0x64e0c).
  The return record leaves r0 unknown after the two predicated moves have merged effects (`moveq` at 0x64df4, `movne` at 0x64df8, `ldm` at 0x64e18).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64de0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64de4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x64de8 | load | 4 | `abs 0x64e1c` | `-` |
| `ldr` at 0x64dec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64e00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64e0c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x64e18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64e18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64e18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64e18 | `r0` | `?` |

## `SHUB_Get_NICfg`

- `SHUB_Get_NICfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64e20 to 0x64e7c.
  Established. Entry r0 and r1 are saved as words at entry sp minus 0x14 and 0x18 respectively (`str` at 0x64e30, `str` at 0x64e34).
  The halfword load has an unknown effective address (`ldrh` at 0x64e68).
  The return record retains an unknown value in r0 (`ldm` at 0x64e74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64e30 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64e34 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x64e38 | load | 4 | `abs 0x64e78` | `-` |
| `ldr` at 0x64e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64e50 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x64e68 | load | 2 | `(unknown)` | `-` |
| `ldm` at 0x64e74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64e74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64e74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64e74 | `r0` | `?` |

## `SHUB_Set_NICfg`

- `SHUB_Set_NICfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64e7c to 0x64f7c.
  Established. A halfword store preserves the low 16 bits of entry r2 at entry sp minus 0x1a (`strh` at 0x64e98).
  Mutually exclusive paths write unknown halfword values to the address held in [sp-36] (`strh` at 0x64f1c, `strh` at 0x64f3c).
  A separate path stores the low 16 bits of [sp-26]:2 at that address (`strh` at 0x64f4c).
  The later word store writes 1 to an unknown effective target (`str` at 0x64f68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64e8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64e90 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x64e94 | store | 4 | `sp -0x20` | `r3` |
| `strh` at 0x64e98 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldr` at 0x64e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64eb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x64ebc | load | 4 | `abs 0x64f74` | `-` |
| `str` at 0x64ec4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x64ec8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64ecc | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0x64ed0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64edc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64ee8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64ef8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x64f08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64f0c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x64f10 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldrh` at 0x64f14 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x64f1c | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x64f24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x64f28 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x64f2c | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0x64f34 | load | 2 | `[sp-36] +0x0` | `-` |
| `strh` at 0x64f3c | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x64f44 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x64f48 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x64f4c | store | 2 | `[sp-36] +0x0` | `([sp-26]:2 & 0xffff)` |
| `ldr` at 0x64f50 | load | 4 | `abs 0x64f78` | `-` |
| `ldr` at 0x64f54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x64f5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x64f68 | store | 4 | `(unknown)` | `0x1` |
| `ldm` at 0x64f70 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x64f70 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x64f70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64ed8 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |
| `bgt` at 0x64ee4 | `[sp-32] > 0x2 (signed, 32-bit)` | `[sp-32] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x64ef0 | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `beq` at 0x64f00 | `[sp-32] == 0x3` | `[sp-32] != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x64f70 | `r0` | `r0` |

## `SHUB_Set_MCI`

- `SHUB_Set_MCI`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64f7c to 0x65064.
  Established. A word store writes 1 to absolute address 0x1a5cc8 (`str` at 0x64fa8).
  Included computed stores write the low eight bits of [sp-17]:1 as a byte and the low 16 bits of [sp-20]:2 as a halfword to unknown effective targets (`strb` at 0x64fe8, `strh` at 0x65014).
  A later byte store writes the low eight bits of [sp-21]:1 to an unknown effective target (`strb` at 0x65040).
  An addition increments [sp-28] by 1, and a word store writes the result at entry sp minus 0x1c (`add` at 0x65048, `str` at 0x6504c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64f90 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0x64f94 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0x64f9c | store | 1 | `sp -0x15` | `(r2 & 0xff)` |
| `ldr` at 0x64fa0 | load | 4 | `abs 0x6505c` | `-` |
| `str` at 0x64fa8 | store | 4 | `abs 0x1a5cc8` | `0x1` |
| `str` at 0x64fb0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x64fb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x64fc0 | load | 4 | `abs 0x65060` | `-` |
| `ldr` at 0x64fc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x64fe4 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x64fe8 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x64fec | load | 4 | `abs 0x65060` | `-` |
| `ldr` at 0x64ff0 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x65010 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x65014 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x65018 | load | 4 | `abs 0x65060` | `-` |
| `ldr` at 0x6501c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x6503c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x65040 | store | 1 | `(unknown)` | `([sp-21]:1 & 0xff)` |
| `ldr` at 0x65044 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6504c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0x65058 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65058 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65058 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x64fbc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65058 | `r0` | `?` |

## `SHUB_Reset_MCI`

- `SHUB_Reset_MCI`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65064 to 0x65084.
  Established. The explicit word store writes 0 to absolute address 0x1a5cc8 (`str` at 0x65078).
  Included implicit saves, with PC omitted, place entry r11, sp and lr at entry sp minus 0x10, 0xc and 8 respectively (`push` at 0x65068).
  The return record preserves entry r0 (`ldm` at 0x6507c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65070 | load | 4 | `abs 0x65080` | `-` |
| `str` at 0x65078 | store | 4 | `abs 0x1a5cc8` | `0x0` |
| `ldm` at 0x6507c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6507c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6507c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6507c | `r0` | `r0` |

## `SHUB_Set_WRT90`

- `SHUB_Set_WRT90`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65084 to 0x6516c.
  Established. The initial byte stores preserve the low eight bits of entry r0 and r1 at entry sp minus 0x11 and 0x12 respectively (`strb` at 0x65098, `strb` at 0x650a0).
  A halfword store preserves the low 16 bits of entry r2 at entry sp minus 0x14 (`strh` at 0x650a4).
  The word store at absolute address 0x1a5ccc writes 1 (`str` at 0x650b0).
  Computed byte stores and a halfword store have unknown effective targets (`strb` at 0x650f0, `strb` at 0x6511c, `strh` at 0x65148).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x65098 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x650a0 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0x650a4 | store | 2 | `sp -0x14` | `(r2 & 0xffff)` |
| `ldr` at 0x650a8 | load | 4 | `abs 0x65164` | `-` |
| `str` at 0x650b0 | store | 4 | `abs 0x1a5ccc` | `0x1` |
| `str` at 0x650b8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x650bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x650c8 | load | 4 | `abs 0x65168` | `-` |
| `ldr` at 0x650cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x650ec | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x650f0 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x650f4 | load | 4 | `abs 0x65168` | `-` |
| `ldr` at 0x650f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x65118 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x6511c | store | 1 | `(unknown)` | `([sp-18]:1 & 0xff)` |
| `ldr` at 0x65120 | load | 4 | `abs 0x65168` | `-` |
| `ldr` at 0x65124 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x65144 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x65148 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x6514c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x65154 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x65160 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65160 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65160 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x650c4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65160 | `r0` | `?` |

## `SHUB_Reset_WRT90`

- `SHUB_Reset_WRT90`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6516c to 0x6518c.
  Established. Zero is stored as a word at absolute address 0x1a5ccc (`str` at 0x65180).
  The final ldm reads words at entry sp minus 0x10, 0xc and 8 (`ldm` at 0x65184);
  its return record keeps entry r0 in r0 (`ldm` at 0x65184).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65178 | load | 4 | `abs 0x65188` | `-` |
| `str` at 0x65180 | store | 4 | `abs 0x1a5ccc` | `0x0` |
| `ldm` at 0x65184 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65184 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65184 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65184 | `r0` | `r0` |

## `SHUB_Set_FSBCfg`

- `SHUB_Set_FSBCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6531c to 0x654d0.
  Established. A word store writes 1 to absolute address 0x1a5cd0 (`str` at 0x6537c).
  Word loads read entry sp, entry sp plus 4 and entry sp plus 8 (`ldr` at 0x65330, `ldr` at 0x65334, `ldr` at 0x65338).
  Their low eight bits are stored as bytes at entry sp minus 0x1d, 0x1e and 0x1f respectively (`strb` at 0x65360, `strb` at 0x65368, `strb` at 0x65370).
  Seven later byte-store instructions retain unknown effective targets (`strb` at 0x653b4, `strb` at 0x653d8, `strb` at 0x65404, `strb` at 0x65430, `strb` at 0x6545c, `strb` at 0x65488, `strb` at 0x654ac).
  An addition increments [sp-36] by 1, and a word store writes the result at entry sp minus 0x24 (`add` at 0x654b4, `str` at 0x654b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65330 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x65334 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x65338 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x65340 | store | 1 | `sp -0x19` | `(r0 & 0xff)` |
| `strb` at 0x65348 | store | 1 | `sp -0x1a` | `(r1 & 0xff)` |
| `strb` at 0x65350 | store | 1 | `sp -0x1b` | `(r2 & 0xff)` |
| `strb` at 0x65358 | store | 1 | `sp -0x1c` | `(r3 & 0xff)` |
| `strb` at 0x65360 | store | 1 | `sp -0x1d` | `([sp] & 0xff)` |
| `strb` at 0x65368 | store | 1 | `sp -0x1e` | `([sp+4] & 0xff)` |
| `strb` at 0x65370 | store | 1 | `sp -0x1f` | `([sp+8] & 0xff)` |
| `ldr` at 0x65374 | load | 4 | `abs 0x654c8` | `-` |
| `str` at 0x6537c | store | 4 | `abs 0x1a5cd0` | `0x1` |
| `str` at 0x65384 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x65388 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x65394 | load | 4 | `abs 0x654cc` | `-` |
| `ldr` at 0x65398 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x653b0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x653b4 | store | 1 | `(unknown)` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0x653b8 | load | 4 | `abs 0x654cc` | `-` |
| `ldr` at 0x653bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x653d4 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x653d8 | store | 1 | `(unknown)` | `([sp-26]:1 & 0xff)` |
| `ldr` at 0x653dc | load | 4 | `abs 0x654cc` | `-` |
| `ldr` at 0x653e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x65400 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x65404 | store | 1 | `(unknown)` | `([sp-27]:1 & 0xff)` |
| `ldr` at 0x65408 | load | 4 | `abs 0x654cc` | `-` |
| `ldr` at 0x6540c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x6542c | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x65430 | store | 1 | `(unknown)` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x65434 | load | 4 | `abs 0x654cc` | `-` |
| `ldr` at 0x65438 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x65458 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x6545c | store | 1 | `(unknown)` | `([sp-29]:1 & 0xff)` |
| `ldr` at 0x65460 | load | 4 | `abs 0x654cc` | `-` |
| `ldr` at 0x65464 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x65484 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x65488 | store | 1 | `(unknown)` | `([sp-30]:1 & 0xff)` |
| `ldr` at 0x6548c | load | 4 | `abs 0x654cc` | `-` |
| `ldr` at 0x65490 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x654a8 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x654ac | store | 1 | `(unknown)` | `([sp-31]:1 & 0xff)` |
| `ldr` at 0x654b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x654b8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0x654c4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x654c4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x654c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x654c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x654c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x65390 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x654c4 | `r0` | `?` |

## `SHUB_Reset_FSBCfg`

- `SHUB_Reset_FSBCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x654d0 to 0x654f0.
  Established. A word store places 0 at absolute address 0x1a5cd0 (`str` at 0x654e4).
  Included implicit word saves, with PC omitted, place entry r11, sp and lr at entry sp minus 0x10, 0xc and 8 (`push` at 0x654d4).
  The ldm reads words at those three locations and the return record preserves entry r0 (`ldm` at 0x654e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x654dc | load | 4 | `abs 0x654ec` | `-` |
| `str` at 0x654e4 | store | 4 | `abs 0x1a5cd0` | `0x0` |
| `ldm` at 0x654e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x654e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x654e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x654e8 | `r0` | `r0` |

## `SHUB_Target`

- `SHUB_Target`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x654f0 to 0x6552c.
  Established. A byte store keeps the low eight bits of entry r0 at entry sp minus 0x11 (`strb` at 0x65504).
  A separate word load reads absolute address 0x1a5cac (`ldr` at 0x65514).
  The following computed word load has an unknown effective address (`ldr` at 0x6551c).
  The returned r0 value remains unknown (`ldm` at 0x65524).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x65504 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x65508 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x65510 | load | 4 | `abs 0x65528` | `-` |
| `ldr` at 0x65514 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x6551c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x65524 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x65524 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65524 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65524 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65524 | `r0` | `?` |

## `SHUB_Allow_Resets`

- `SHUB_Allow_Resets`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6552c to 0x657a0.
  Established. One call to unnamed 0x64ce0 has printed r0-r3 candidates unknown, 0x50000010, 0 and [sp-40] (`bl` at 0x65594).
  A later call to unnamed 0x64d58 has candidates [sp-17]:1, 0x50000010, sp-44 and sp-48 (`bl` at 0x65604).
  Four word stores write 0xfffff893 at entry sp minus 0x20 on their respective paths (`str` at 0x655a8, `str` at 0x655d8, `str` at 0x65618, `str` at 0x6565c).
  A separate word load from absolute address 0x315034 supplies the zero test: beq is taken when that loaded word is zero and falls through otherwise (`ldr` at 0x656dc, `beq` at 0x656e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x65540 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x65544 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x6554c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x65554 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x65558 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x65568 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x65570 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x65574 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x65580 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x65588 | load | 4 | `abs 0x65780` | `-` |
| `ldr` at 0x65590 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x655a4 | load | 4 | `abs 0x65784` | `-` |
| `str` at 0x655a8 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldrb` at 0x655b0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x655b8 | load | 4 | `abs 0x65788` | `-` |
| `ldr` at 0x655c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x655d4 | load | 4 | `abs 0x65784` | `-` |
| `str` at 0x655d8 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x655e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x655ec | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x655fc | load | 4 | `abs 0x65780` | `-` |
| `ldr` at 0x65614 | load | 4 | `abs 0x65784` | `-` |
| `str` at 0x65618 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x65620 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x65630 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x65640 | load | 4 | `abs 0x65788` | `-` |
| `ldr` at 0x65658 | load | 4 | `abs 0x65784` | `-` |
| `str` at 0x6565c | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x65664 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x65678 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x65680 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x65688 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x65690 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6569c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x656a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x656b4 | load | 4 | `abs 0x6578c` | `-` |
| `str` at 0x656b8 | store | 4 | `sp -0x38` | `0x163220` |
| `ldr` at 0x656c0 | load | 4 | `abs 0x65790` | `-` |
| `str` at 0x656c4 | store | 4 | `sp -0x38` | `0x16322c` |
| `ldr` at 0x656c8 | load | 4 | `abs 0x65794` | `-` |
| `ldr` at 0x656cc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x656d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x656d8 | load | 4 | `abs 0x65798` | `-` |
| `ldr` at 0x656dc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x656e8 | load | 4 | `abs 0x65794` | `-` |
| `str` at 0x656ec | store | 4 | `sp -0x4c` | `0x163234` |
| `ldr` at 0x656f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x656fc | load | 4 | `abs 0x6578c` | `-` |
| `str` at 0x65700 | store | 4 | `sp -0x3c` | `0x163220` |
| `ldr` at 0x65708 | load | 4 | `abs 0x65790` | `-` |
| `str` at 0x6570c | store | 4 | `sp -0x3c` | `0x16322c` |
| `ldr` at 0x65710 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x65714 | store | 4 | `sp -0x48` | `[sp-60]` |
| `ldr` at 0x65718 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6571c | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0x65724 | load | 4 | `abs 0x6579c` | `-` |
| `ldr` at 0x65734 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x65744 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x6574c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6575c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x65764 | load | 4 | `abs 0x65784` | `-` |
| `str` at 0x65768 | store | 4 | `sp -0x40` | `0xfffff893` |
| `ldr` at 0x6576c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x65770 | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x65774 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x6577c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6577c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6577c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x65594 | `(unknown callee) 0x64ce0` | `?` | `0x50000010` | `0x0` | `[sp-40]` |
| `bl` at 0x655c4 | `(unknown callee) 0x64ce0` | `[sp-17]:1` | `0x50002010` | `0x0` | `[sp-40]` |
| `bl` at 0x65604 | `(unknown callee) 0x64d58` | `[sp-17]:1` | `0x50000010` | `sp-44` | `sp-48` |
| `bl` at 0x65648 | `(unknown callee) 0x64d58` | `[sp-17]:1` | `0x50002010` | `sp-44` | `sp-48` |
| `bl` at 0x656d4 | `(unknown callee) 0x29908` | `0x163234` | `[sp-56]` | `?` | `?` |
| `bl` at 0x65730 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x1b0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x6557c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x655a0 | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `bne` at 0x655d0 | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `bne` at 0x655e8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x65610 | `ret(0x64d58) != 0x0` | `ret(0x64d58) == 0x0` |
| `bne` at 0x65628 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `bne` at 0x65654 | `ret(0x64d58) != 0x0` | `ret(0x64d58) == 0x0` |
| `bne` at 0x6566c | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `ble` at 0x65698 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x656a4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x656b0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x656e4 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x656f8 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x6573c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x65754 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6577c | `r0` | `?` |
