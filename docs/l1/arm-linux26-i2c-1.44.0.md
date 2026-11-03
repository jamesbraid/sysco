# ARM Linux 2.6 I2C paths in 1.44.0

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
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-i2c-facts.txt) retain all tool sections.

## `i2cInit`

- `i2cInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a314 to 0x10a4ec.
  Established. A word assignment places `0x56000000` at `sp -0x24` (`str` at 0x10a3bc).
  Separate word assignments store `ret(0x2016c)` and later `[0x31beec]+36` at `0x31beec` (`str` at 0x10a3d4, `str` at 0x10a42c).
  An unnamed `0x2016c` call carries `0x0`, `0x100`, `0x3` and `0x1` (`bl` at 0x10a3d0).
  The hardware mapping and initialization contract remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10a324 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x10a328 | load | 4 | `abs 0x10a4c4` | `-` |
| `ldr` at 0x10a32c | load | 4 | `abs 0x1a04bc` | `-` |
| `ldr` at 0x10a338 | load | 4 | `abs 0x10a4c8` | `-` |
| `str` at 0x10a33c | store | 4 | `sp -0x28` | `0x18c7d4` |
| `ldr` at 0x10a344 | load | 4 | `abs 0x10a4cc` | `-` |
| `ldr` at 0x10a354 | load | 4 | `abs 0x10a4d0` | `-` |
| `ldr` at 0x10a358 | load | 4 | `abs 0x10a4d4` | `-` |
| `str` at 0x10a364 | store | 4 | `sp -0x1c` | `ret(0x2046c)` |
| `ldr` at 0x10a368 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10a380 | load | 4 | `abs 0x10a4d8` | `-` |
| `str` at 0x10a384 | store | 4 | `sp -0x28` | `0x18c800` |
| `ldr` at 0x10a388 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10a38c | store | 4 | `sp -0x24` | `[ret(0x20184)]` |
| `ldr` at 0x10a394 | load | 4 | `abs 0x10a4cc` | `-` |
| `ldr` at 0x10a3ac | load | 4 | `abs 0x10a4dc` | `-` |
| `ldr` at 0x10a3b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10a3b4 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x10a3bc | store | 4 | `sp -0x24` | `0x56000000` |
| `str` at 0x10a3d4 | store | 4 | `abs 0x31beec` | `ret(0x2016c)` |
| `ldr` at 0x10a3d8 | load | 4 | `abs 0x31beec` | `-` |
| `ldr` at 0x10a3f0 | load | 4 | `abs 0x10a4e0` | `-` |
| `str` at 0x10a3f4 | store | 4 | `sp -0x28` | `0x18c820` |
| `ldr` at 0x10a3f8 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10a3fc | store | 4 | `sp -0x24` | `[ret(0x20184)]` |
| `ldr` at 0x10a404 | load | 4 | `abs 0x10a4cc` | `-` |
| `ldr` at 0x10a41c | load | 4 | `abs 0x10a4dc` | `-` |
| `ldr` at 0x10a420 | load | 4 | `abs 0x10a4dc` | `-` |
| `ldr` at 0x10a424 | load | 4 | `abs 0x31beec` | `-` |
| `str` at 0x10a42c | store | 4 | `abs 0x31beec` | `[0x31beec]+36` |
| `ldr` at 0x10a430 | load | 4 | `abs 0x10a4e4` | `-` |
| `ldr` at 0x10a440 | load | 4 | `abs 0x10a4c4` | `-` |
| `str` at 0x10a444 | store | 4 | `abs 0x1a04bc` | `ret(0x2046c)` |
| `ldr` at 0x10a448 | load | 4 | `abs 0x1a04bc` | `-` |
| `ldr` at 0x10a45c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10a468 | load | 4 | `abs 0x10a4e8` | `-` |
| `str` at 0x10a46c | store | 4 | `sp -0x28` | `0x18c858` |
| `str` at 0x10a470 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x10a478 | load | 4 | `abs 0x10a4cc` | `-` |
| `strb` at 0x10a48c | store | 1 | `sp -0x1d` | `0x1` |
| `ldrb` at 0x10a490 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x10a49c | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x10a4ac | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x10a4b4 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldm` at 0x10a4c0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10a4c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a4c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a4c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10a350 | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x59` | `0x0` |
| `bl` at 0x10a35c | `(unknown callee) 0x2046c` | `0x18c7f4` | `0x1002` | `?` | `?` |
| `bl` at 0x10a374 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0x10a3a0 | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x5f` | `0x0` |
| `bl` at 0x10a3a8 | `(unknown callee) 0x204b4` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x10a3d0 | `(unknown callee) 0x2016c` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0x10a3e4 | `(unknown callee) 0x20184` | `ret(0x2016c)` | `?` | `?` | `[0x31beec]` |
| `bl` at 0x10a410 | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x67` | `0x0` |
| `bl` at 0x10a418 | `(unknown callee) 0x204b4` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x10a438 | `(unknown callee) 0x2046c` | `0x18c84c` | `0x2` | `0x31beec` | `[0x31beec]+36` |
| `bl` at 0x10a454 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `ret(0x2046c)` | `[0x1a04bc]` |
| `bl` at 0x10a460 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x10a484 | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x71` | `0x0` |
| `bl` at 0x10a4a4 | `(unknown callee) 0x1243c0` | `?` | `?` | `?` | `?` |
| `bl` at 0x10a4a8 | `(unknown callee) 0x10ab4c` | `ret(0x1243c0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x10a334 | `[0x1a04bc] < 0x0 (signed, 32-bit)` | `[0x1a04bc] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x10a370 | `ret(0x2046c) >= 0x0 (signed, 32-bit)` | `ret(0x2046c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x10a3e0 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x10a450 | `[0x1a04bc] >= 0x0 (signed, 32-bit)` | `[0x1a04bc] < 0x0 (signed, 32-bit)` |
| `bhi` at 0x10a498 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a4c0 | `r0` | `?` |

## `i2cBus`

- `i2cBus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a4ec to 0x10a5a8.
  Established. A byte assignment preserves `(r1 & 0xff)` at `sp -0x15` (`strb` at 0x10a504).
  Inequality `ret(0x6e190) != [sp-20]` takes `0x10a528`, while equality falls through (`bne` at 0x10a518).
  Included word reads address `0x314694` (`ldr` at 0x10a52c, `ldr` at 0x10a570).
  The bus-number encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10a4fc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x10a504 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x10a510 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x10a51c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x10a520 | store | 4 | `sp -0x1c` | `[sp-21]:1` |
| `ldr` at 0x10a528 | load | 4 | `abs 0x10a59c` | `-` |
| `ldr` at 0x10a52c | load | 4 | `abs 0x314694` | `-` |
| `ldr` at 0x10a538 | load | 4 | `abs 0x10a5a0` | `-` |
| `str` at 0x10a53c | store | 4 | `sp -0x20` | `0x18c874` |
| `ldr` at 0x10a544 | load | 4 | `abs 0x10a5a4` | `-` |
| `ldrb` at 0x10a55c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x10a564 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10a56c | load | 4 | `abs 0x10a59c` | `-` |
| `ldr` at 0x10a570 | load | 4 | `abs 0x314694` | `-` |
| `ldr` at 0x10a578 | load | 4 | `(unknown)` | `-` |
| `str` at 0x10a58c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x10a590 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x10a598 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a598 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10a508 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `r1` |
| `bl` at 0x10a550 | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x94` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x10a518 | `ret(0x6e190) != [sp-20]` | `ret(0x6e190) == [sp-20]` |
| `bne` at 0x10a534 | `[0x314694] != 0x0` | `[0x314694] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a598 | `r0` | `?` |

## `i2cLock`

- `i2cLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a87c to 0x10a9c0.
  Established. A word assignment writes `ret(0x20370)` to `0x31468c` (`str` at 0x10a8f0).
  The unknown-target call carries `[sp-17]:1`, unknown, `[sp-17]:1` and `[0x1a04c0]` (`mov` at 0x10a948).
  On a separate path, a byte assignment writes `([sp-17]:1 & 0xff)` to `0x314690` (`strb` at 0x10a984).
  Lock ownership and callback hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10a890 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10a894 | load | 4 | `abs 0x10a99c` | `-` |
| `str` at 0x10a8a0 | store | 4 | `sp -0x18` | `ret(0x202d4)` |
| `ldr` at 0x10a8a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10a8b8 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10a8c4 | load | 4 | `abs 0x10a9a0` | `-` |
| `str` at 0x10a8c8 | store | 4 | `sp -0x24` | `0x18c92c` |
| `str` at 0x10a8cc | store | 4 | `sp -0x20` | `ret(0x219a4)` |
| `ldr` at 0x10a8d4 | load | 4 | `abs 0x10a9a4` | `-` |
| `ldr` at 0x10a8d8 | load | 4 | `abs 0x10a9a8` | `-` |
| `ldr` at 0x10a8ec | load | 4 | `abs 0x10a9ac` | `-` |
| `str` at 0x10a8f0 | store | 4 | `abs 0x31468c` | `ret(0x20370)` |
| `ldrb` at 0x10a8f4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10a900 | load | 4 | `abs 0x10a9b0` | `-` |
| `ldr` at 0x10a904 | load | 4 | `abs 0x1a04c0` | `-` |
| `ldr` at 0x10a910 | load | 4 | `abs 0x10a9b4` | `-` |
| `str` at 0x10a914 | store | 4 | `sp -0x24` | `0x18c948` |
| `ldrb` at 0x10a918 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10a91c | store | 4 | `sp -0x20` | `[sp-17]:1` |
| `ldr` at 0x10a924 | load | 4 | `abs 0x10a9a4` | `-` |
| `ldr` at 0x10a928 | load | 4 | `abs 0x10a9b8` | `-` |
| `ldr` at 0x10a934 | load | 4 | `abs 0x10a9b0` | `-` |
| `ldrb` at 0x10a938 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10a93c | load | 4 | `abs 0x1a04c0` | `-` |
| `str` at 0x10a950 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0x10a954 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10a96c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10a974 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x10a97c | load | 4 | `abs 0x10a9bc` | `-` |
| `ldrb` at 0x10a980 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x10a984 | store | 1 | `abs 0x314690` | `([sp-17]:1 & 0xff)` |
| `str` at 0x10a98c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x10a990 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x10a998 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a998 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a998 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10a898 | `(unknown callee) 0x202d4` | `0x314670` | `r1` | `r2` | `r0` |
| `bl` at 0x10a8b0 | `(unknown callee) 0x20184` | `ret(0x202d4)` | `?` | `?` | `ret(0x202d4)` |
| `bl` at 0x10a8bc | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x10a8e0 | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x162` | `0x0` |
| `bl` at 0x10a8e4 | `(unknown callee) 0x20370` | `?` | `?` | `?` | `?` |
| `bl` at 0x10a930 | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x16b` | `0x0` |
| `mov` at 0x10a948 | `(unknown callee) (unknown target)` | `[sp-17]:1` | `?` | `[sp-17]:1` | `[0x1a04c0]` |
| `bl` at 0x10a960 | `(unknown callee) 0x10a9c0` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x10a964 | `(unknown callee) 0x20184` | `ret(0x10a9c0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10a8ac | `ret(0x202d4) == 0x0` | `ret(0x202d4) != 0x0` |
| `beq` at 0x10a8fc | `[sp-17]:1 == 0x0` | `[sp-17]:1 != 0x0` |
| `bne` at 0x10a90c | `[0x1a04c0] != 0x0` | `[0x1a04c0] == 0x0` |
| `beq` at 0x10a95c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a998 | `r0` | `?` |

## `i2cUnlock`

- `i2cUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a9c0 to 0x10aab8.
  Established. A word assignment clears `0x31468c`, and a later byte assignment clears `0x314690` (`str` at 0x10a9d8, `strb` at 0x10aa3c).
  The unknown-target call carries `[0x314690]:1`, entry `r1`, `[0x314688]` and `[0x314690]:1` (`mov` at 0x10aa14).
  A zero `ret(0x2058c)` takes `0x10aa90`, while nonzero falls through (`beq` at 0x10aa58).
  The release protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x10a9d0 | load | 4 | `abs 0x10aa98` | `-` |
| `str` at 0x10a9d8 | store | 4 | `abs 0x31468c` | `0x0` |
| `ldr` at 0x10a9dc | load | 4 | `abs 0x10aa9c` | `-` |
| `ldrb` at 0x10a9e0 | load | 1 | `abs 0x314690` | `-` |
| `ldr` at 0x10a9ec | load | 4 | `abs 0x10aaa0` | `-` |
| `ldr` at 0x10a9f0 | load | 4 | `abs 0x314688` | `-` |
| `ldr` at 0x10a9fc | load | 4 | `abs 0x10aaa0` | `-` |
| `ldr` at 0x10aa00 | load | 4 | `abs 0x10aa9c` | `-` |
| `ldrb` at 0x10aa04 | load | 1 | `abs 0x314690` | `-` |
| `ldr` at 0x10aa08 | load | 4 | `abs 0x314688` | `-` |
| `str` at 0x10aa1c | store | 4 | `sp -0x14` | `ret(unknown)` |
| `ldr` at 0x10aa20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10aa2c | load | 4 | `abs 0x10aaa4` | `-` |
| `ldr` at 0x10aa34 | load | 4 | `abs 0x10aa9c` | `-` |
| `strb` at 0x10aa3c | store | 1 | `abs 0x314690` | `0x0` |
| `ldr` at 0x10aa40 | load | 4 | `abs 0x10aaa8` | `-` |
| `str` at 0x10aa4c | store | 4 | `sp -0x14` | `ret(0x2058c)` |
| `ldr` at 0x10aa50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10aa64 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10aa70 | load | 4 | `abs 0x10aaac` | `-` |
| `str` at 0x10aa74 | store | 4 | `sp -0x1c` | `0x18c978` |
| `str` at 0x10aa78 | store | 4 | `sp -0x18` | `ret(0x219a4)` |
| `ldr` at 0x10aa80 | load | 4 | `abs 0x10aab0` | `-` |
| `ldr` at 0x10aa84 | load | 4 | `abs 0x10aab4` | `-` |
| `ldm` at 0x10aa94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10aa94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10aa94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x10aa14 | `(unknown callee) (unknown target)` | `[0x314690]:1` | `r1` | `[0x314688]` | `[0x314690]:1` |
| `bl` at 0x10aa30 | `(unknown callee) 0x293f8` | `0x18c960` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x10aa44 | `(unknown callee) 0x2058c` | `0x314670` | `?` | `?` | `?` |
| `bl` at 0x10aa5c | `(unknown callee) 0x20184` | `ret(0x2058c)` | `?` | `?` | `ret(0x2058c)` |
| `bl` at 0x10aa68 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x10aa8c | `(unknown callee) 0x22070` | `0x9` | `0x18c7ec` | `0x195` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10a9e8 | `[0x314690]:1 == 0x0` | `[0x314690]:1 != 0x0` |
| `beq` at 0x10a9f8 | `[0x314688] == 0x0` | `[0x314688] != 0x0` |
| `beq` at 0x10aa28 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x10aa58 | `ret(0x2058c) == 0x0` | `ret(0x2058c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10aa94 | `r0` | `?` |

## `i2cSelectBus`

- `i2cSelectBus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10aab8 to 0x10ab4c.
  Established. A word read addresses `0x1a04c0` (`ldr` at 0x10aad4).
  Zero there takes `0x10ab04`, while nonzero falls through (`beq` at 0x10aadc).
  The unknown-target call carries `(r0 & 0xff)`, entry `r1`, `(r0 & 0xff)` and `[0x1a04c0]` (`mov` at 0x10aaf4).
  Physical bus-selection behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10aacc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10aad0 | load | 4 | `abs 0x10ab3c` | `-` |
| `ldr` at 0x10aad4 | load | 4 | `abs 0x1a04c0` | `-` |
| `ldr` at 0x10aae0 | load | 4 | `abs 0x10ab3c` | `-` |
| `ldrb` at 0x10aae4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10aae8 | load | 4 | `abs 0x1a04c0` | `-` |
| `str` at 0x10aafc | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0x10ab04 | load | 4 | `abs 0x10ab40` | `-` |
| `str` at 0x10ab08 | store | 4 | `sp -0x20` | `0x18c994` |
| `ldrb` at 0x10ab0c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10ab10 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x10ab18 | load | 4 | `abs 0x10ab44` | `-` |
| `ldr` at 0x10ab1c | load | 4 | `abs 0x10ab48` | `-` |
| `str` at 0x10ab2c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x10ab30 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10ab38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10ab38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10ab38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x10aaf4 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x1a04c0]` |
| `bl` at 0x10ab24 | `(unknown callee) 0x22070` | `0x1` | `0x18c7ec` | `0x1a7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10aadc | `[0x1a04c0] == 0x0` | `[0x1a04c0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10ab38 | `r0` | `?` |

## `i2cReset`

- `i2cReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ab4c to 0x10abd4.
  Established. The unnamed `0x20520` call carries `[0x1a04bc]`, `0x782`, `0x0` and `0x1a04bc` (`bl` at 0x10ab80);
  its signed nonnegative result takes `0x10abb0`, while negative falls through (`bge` at 0x10ab8c).
  A word assignment updates `sp -0x14` to `[sp-20]+1` (`str` at 0x10abb8).
  The hardware reset sequence behind the call remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10ab60 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x10ab64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10ab70 | load | 4 | `abs 0x10abc4` | `-` |
| `ldr` at 0x10ab74 | load | 4 | `abs 0x1a04bc` | `-` |
| `ldr` at 0x10ab78 | load | 4 | `abs 0x10abc8` | `-` |
| `ldr` at 0x10ab9c | load | 4 | `abs 0x10abcc` | `-` |
| `ldr` at 0x10aba0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x10aba4 | load | 4 | `abs 0x10abd0` | `-` |
| `ldr` at 0x10aba8 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10abb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10abb8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0x10abc0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10abc0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10abc0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10abc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ab80 | `(unknown callee) 0x20520` | `[0x1a04bc]` | `0x782` | `0x0` | `0x1a04bc` |
| `bl` at 0x10ab90 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10abac | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x18c9b0` | `[ret(0x20184)]` | `0x1a288c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x10ab6c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0x10ab8c | `ret(0x20520) >= 0x0 (signed, 32-bit)` | `ret(0x20520) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10abc0 | `r0` | `?` |

## `i2cWrite`

- `i2cWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10abd4 to 0x10ac24.
  Established. A halfword assignment stores `(r2 & 0xffff)` at `sp -0x1a` (`strh` at 0x10abf0).
  The unnamed `0x10af88` call carries `0x0`, `(r0 & 0xff)`, unknown and `[sp-24]` (`bl` at 0x10ac10);
  its result is the integer return candidate (`ldm` at 0x10ac20).
  Wire-write effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10abe8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x10abec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0x10abf0 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0x10abf4 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x10abf8 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0x10abfc | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0x10ac0c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10ac20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10ac20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10ac20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ac10 | `(unknown callee) 0x10af88` | `0x0` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10ac20 | `r0` | `ret(0x10af88)` |

## `i2cRead`

- `i2cRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ac84 to 0x10acd4.
  Established. The outgoing stack word at `sp -0x20` receives `(r2 & 0xffff)` (`str` at 0x10acac).
  An unnamed `0x10af88` call carries `0x1`, `(r0 & 0xff)`, unknown and `[sp-24]` (`bl` at 0x10acc0).
  The return record preserves `ret(0x10af88)` (`ldm` at 0x10acd0).
  Receive-buffer and wire semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10ac98 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x10ac9c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0x10aca0 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0x10aca4 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x10aca8 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0x10acac | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0x10acbc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10acd0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10acd0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10acd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10acc0 | `(unknown callee) 0x10af88` | `0x1` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10acd0 | `r0` | `ret(0x10af88)` |

## `i2cWriteOffset`

- `i2cWriteOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ac24 to 0x10ac84.
  Established. Included byte assignments write `(r0 & 0xff)` and `(r1 & 0xff)` to `sp -0x11` and `sp -0x12` (`strb` at 0x10ac40, `strb` at 0x10ac48).
  A halfword assignment stores `(r3 & 0xffff)` at `sp -0x1a` (`strh` at 0x10ac4c).
  The unnamed `0x10af88` call carries `0x0`, `(r0 & 0xff)`, `(r1 & 0xff)` and entry `r2` (`bl` at 0x10ac70).
  Offset-write protocol ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10ac34 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0x10ac40 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x10ac48 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0x10ac4c | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0x10ac50 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10ac54 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0x10ac58 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0x10ac5c | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0x10ac6c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10ac80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10ac80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10ac80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ac70 | `(unknown callee) 0x10af88` | `0x0` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10ac80 | `r0` | `ret(0x10af88)` |

## `i2cReadOffset`

- `i2cReadOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10acd4 to 0x10ad34.
  Established. A local word assignment preserves entry `r2` at `sp -0x18` (`str` at 0x10ace4).
  The unnamed `0x10af88` call carries `0x1`, `(r0 & 0xff)`, `(r1 & 0xff)` and entry `r2` (`bl` at 0x10ad20);
  its outgoing stack word at `sp -0x20` contains `(r3 & 0xffff)` (`str` at 0x10ad0c).
  Offset-read units and device behavior remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10ace4 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0x10acf0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x10acf8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0x10acfc | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0x10ad00 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10ad04 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0x10ad08 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0x10ad0c | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0x10ad1c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10ad30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10ad30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10ad30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ad20 | `(unknown callee) 0x10af88` | `0x1` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10ad30 | `r0` | `ret(0x10af88)` |

## `i2cWriteByte`

- `i2cWriteByte`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ad34 to 0x10adc4.
  Established. Zero `ret(0x10a87c)` takes `0x10ad88`, while nonzero falls through (`beq` at 0x10ad78).
  The unnamed `0x10abd4` call carries `[sp-18]:1`, `sp-19`, `0x1` and `[sp-18]:1` (`bl` at 0x10ad9c).
  A subsequent unnamed `0x10a9c0` call carries `0x0`, unknown, unknown and `ret(0x10abd4)` (`bl` at 0x10adac).
  Single-byte transfer hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10ad48 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x10ad50 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x10ad58 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x10ad5c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10ad6c | store | 4 | `sp -0x18` | `ret(0x10a87c)` |
| `ldr` at 0x10ad70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10ad7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10ad80 | store | 4 | `sp -0x1c` | `ret(0x10a87c)` |
| `ldrb` at 0x10ad88 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x10ada4 | store | 4 | `sp -0x18` | `ret(0x10abd4)` |
| `ldr` at 0x10adb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10adb4 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x10adb8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x10adc0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10adc0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10adc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ad64 | `(unknown callee) 0x10a87c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x10ad9c | `(unknown callee) 0x10abd4` | `[sp-18]:1` | `sp-19` | `0x1` | `[sp-18]:1` |
| `bl` at 0x10adac | `(unknown callee) 0x10a9c0` | `0x0` | `?` | `?` | `ret(0x10abd4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10ad78 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10adc0 | `r0` | `?` |

## `i2cReadByte`

- `i2cReadByte`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ae68 to 0x10aef0.
  Established. The unnamed `0x10ac84` call carries `[sp-18]:1`, `[sp-24]`, `0x1` and `[sp-18]:1` (`bl` at 0x10aec8);
  its result is saved to `sp -0x1c` before a separate call (`str` at 0x10aed0).
  That unnamed `0x10a9c0` call carries `0x0`, unknown, unknown and `ret(0x10ac84)` (`bl` at 0x10aed8).
  Byte-read completion policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10ae7c | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0x10ae80 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x10ae88 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x10ae8c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10ae9c | store | 4 | `sp -0x1c` | `ret(0x10a87c)` |
| `ldr` at 0x10aea0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10aeac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10aeb0 | store | 4 | `sp -0x20` | `ret(0x10a87c)` |
| `ldrb` at 0x10aeb8 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x10aec0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10aed0 | store | 4 | `sp -0x1c` | `ret(0x10ac84)` |
| `ldr` at 0x10aedc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10aee0 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0x10aee4 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x10aeec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10aeec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10aeec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ae94 | `(unknown callee) 0x10a87c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x10aec8 | `(unknown callee) 0x10ac84` | `[sp-18]:1` | `[sp-24]` | `0x1` | `[sp-18]:1` |
| `bl` at 0x10aed8 | `(unknown callee) 0x10a9c0` | `0x0` | `?` | `?` | `ret(0x10ac84)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10aea8 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10aeec | `r0` | `?` |

## `i2cWriteByteOffset`

- `i2cWriteByteOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10adc4 to 0x10ae68.
  Established. A byte assignment stores `(r3 & 0xff)` at `sp -0x14` (`strb` at 0x10adf4).
  The unnamed `0x10ac24` call carries `[sp-18]:1`, `[sp-19]:1`, `sp-20` and `0x1` (`bl` at 0x10ae40).
  The branch on `ret(0x10a87c) == 0` takes `0x10ae24`, while nonzero falls through (`beq` at 0x10ae14).
  Address-offset and payload interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10addc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x10ade4 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x10adec | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0x10adf4 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `ldrb` at 0x10adf8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10ae08 | store | 4 | `sp -0x18` | `ret(0x10a87c)` |
| `ldr` at 0x10ae0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10ae18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10ae1c | store | 4 | `sp -0x1c` | `ret(0x10a87c)` |
| `ldrb` at 0x10ae24 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x10ae28 | load | 1 | `sp -0x13` | `-` |
| `str` at 0x10ae48 | store | 4 | `sp -0x18` | `ret(0x10ac24)` |
| `ldr` at 0x10ae54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10ae58 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x10ae5c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x10ae64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10ae64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10ae64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ae00 | `(unknown callee) 0x10a87c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x10ae40 | `(unknown callee) 0x10ac24` | `[sp-18]:1` | `[sp-19]:1` | `sp-20` | `0x1` |
| `bl` at 0x10ae50 | `(unknown callee) 0x10a9c0` | `0x0` | `?` | `?` | `ret(0x10ac24)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10ae14 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10ae64 | `r0` | `?` |

## `i2cReadByteOffset`

- `i2cReadByteOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10aef0 to 0x10af88.
  Established. The fourth entry register is preserved as a word at `sp -0x18` (`str` at 0x10af00).
  An unnamed `0x10acd4` call carries `[sp-18]:1`, `[sp-19]:1`, `[sp-24]` and `0x1` (`bl` at 0x10af60).
  A later word assignment places `[sp-28]` at `sp -0x20` (`str` at 0x10af78).
  Hardware byte-offset read behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10af00 | store | 4 | `sp -0x18` | `r3` |
| `strb` at 0x10af08 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x10af10 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x10af18 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x10af1c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10af2c | store | 4 | `sp -0x1c` | `ret(0x10a87c)` |
| `ldr` at 0x10af30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10af3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10af40 | store | 4 | `sp -0x20` | `ret(0x10a87c)` |
| `ldrb` at 0x10af48 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x10af4c | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x10af58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10af68 | store | 4 | `sp -0x1c` | `ret(0x10acd4)` |
| `ldr` at 0x10af74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10af78 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0x10af7c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x10af84 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10af84 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10af84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10af24 | `(unknown callee) 0x10a87c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x10af60 | `(unknown callee) 0x10acd4` | `[sp-18]:1` | `[sp-19]:1` | `[sp-24]` | `0x1` |
| `bl` at 0x10af70 | `(unknown callee) 0x10a9c0` | `0x0` | `?` | `?` | `ret(0x10acd4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10af38 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10af84 | `r0` | `?` |

## `i2cSetSwitch`

- `i2cSetSwitch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a7d8 to 0x10a840.
  Established. A word read addresses `0x31beec` (`ldr` at 0x10a81c).
  A word assignment writes `(r0 & 0xff)` to `[0x31beec] +0x0` (`str` at 0x10a824).
  The integer return candidate is zero (`ldm` at 0x10a834).
  Switch-register identity and bus effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10a7ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x10a7f4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x10a7f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10a7fc | load | 4 | `abs 0x10a838` | `-` |
| `ldr` at 0x10a808 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10a810 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x10a818 | load | 4 | `abs 0x10a83c` | `-` |
| `ldr` at 0x10a81c | load | 4 | `abs 0x31beec` | `-` |
| `ldrb` at 0x10a820 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10a824 | store | 4 | `[0x31beec] +0x0` | `(r0 & 0xff)` |
| `ldm` at 0x10a834 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a834 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x10a804 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a834 | `r0` | `0x0` |
