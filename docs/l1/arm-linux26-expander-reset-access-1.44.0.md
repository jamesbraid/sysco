# ARM Linux 2.6 I/O expander reset and access paths in 1.44.0

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
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers in ioExpReset, ioExpPreConfig and ioExpSoftResetInt leave unresolved
edges. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-expander-reset-access-facts.txt) retain all tool sections.

## `ioExpInit`

- `ioExpInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1247d4 to 0x124a84.
  Established. Word stores write ret(0x6e140) to absolute 0x314fd8 and ret(0x204d8) to absolute 0x314fd4 (`str` at 0x124864, `str` at 0x124898).
  The later call to unnamed 0x204d8 has candidate r0/r1/r2/r3 values [[sp-36]+4]/12/[sp-36]/[sp-36], and its result is stored at [sp-36] +8 (`bl` at 0x124918, `str` at 0x124920).
  Byte stores write 0 and 1 at unknown effective addresses on separate paths, leaving those software fields unidentified (`strb` at 0x124a24, `strb` at 0x124a50).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1247e4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x1247e8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x1247ec | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x1247f0 | load | 4 | `abs 0x124a6c` | `-` |
| `ldr` at 0x1247f4 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x124808 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x124814 | load | 4 | `abs 0x124a70` | `-` |
| `str` at 0x124818 | store | 4 | `sp -0x40` | `0x18fb28` |
| `ldr` at 0x12481c | load | 4 | `abs 0x124a6c` | `-` |
| `ldr` at 0x124820 | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x124824 | store | 4 | `sp -0x3c` | `[0x314fd4]` |
| `ldr` at 0x124828 | load | 4 | `abs 0x124a74` | `-` |
| `ldr` at 0x12482c | load | 4 | `abs 0x314fd8` | `-` |
| `str` at 0x124830 | store | 4 | `sp -0x38` | `[0x314fd8]` |
| `ldr` at 0x124838 | load | 4 | `abs 0x124a78` | `-` |
| `ldr` at 0x124848 | load | 4 | `abs 0x124a74` | `-` |
| `ldr` at 0x12484c | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x124860 | load | 4 | `abs 0x124a74` | `-` |
| `str` at 0x124864 | store | 4 | `abs 0x314fd8` | `ret(0x6e140)` |
| `ldr` at 0x124868 | load | 4 | `abs 0x124a74` | `-` |
| `ldr` at 0x12486c | load | 4 | `abs 0x314fd8` | `-` |
| `str` at 0x124880 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x124884 | load | 4 | `abs 0x124a6c` | `-` |
| `ldr` at 0x124888 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x124898 | store | 4 | `abs 0x314fd4` | `ret(0x204d8)` |
| `ldr` at 0x12489c | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x1248a8 | load | 4 | `abs 0x124a7c` | `-` |
| `str` at 0x1248ac | store | 4 | `sp -0x40` | `0x18fb74` |
| `ldr` at 0x1248b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1248b4 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x1248bc | load | 4 | `abs 0x124a78` | `-` |
| `ldr` at 0x1248cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1248e0 | load | 4 | `abs 0x124a6c` | `-` |
| `ldr` at 0x1248e4 | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x1248ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x1248f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1248f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1248f8 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x1248fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x124900 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x124904 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0x124908 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12490c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x124910 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0x124920 | store | 4 | `[sp-36] +0x8` | `ret(0x204d8)` |
| `ldr` at 0x124924 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x124930 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x124944 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x124948 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x124958 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x12495c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x124984 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x124988 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12498c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x124990 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12499c | load | 4 | `abs 0x124a80` | `-` |
| `str` at 0x1249a0 | store | 4 | `(unknown)` | `0x18fbac` |
| `str` at 0x1249a4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x1249a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1249ac | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x1249b4 | load | 4 | `abs 0x124a78` | `-` |
| `str` at 0x1249c8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x1249cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1249d0 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x1249d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1249e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1249e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1249ec | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x1249f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x124a00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x124a04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x124a18 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0x124a24 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x124a2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x124a30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x124a44 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0x124a50 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x124a54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x124a5c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0x124a68 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x124a68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x124a68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x124a68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x124800 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x314fd4]` |
| `bl` at 0x124844 | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x67` | `0x0` |
| `bl` at 0x124858 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x314fd8]` |
| `bl` at 0x124890 | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `[0x314fd8]` | `?` |
| `bl` at 0x1248c8 | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x72` | `0x0` |
| `bl` at 0x124918 | `(unknown callee) 0x204d8` | `[[sp-36]+4]` | `0xc` | `[sp-36]` | `[sp-36]` |
| `bl` at 0x124940 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x12494c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x124994 | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x1249c0 | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x7e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1247fc | `[0x314fd4] == 0x0` | `[0x314fd4] != 0x0` |
| `bne` at 0x124810 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x124854 | `[0x314fd8] != 0x0` | `[0x314fd8] == 0x0` |
| `bne` at 0x1248a4 | `[0x314fd4] != 0x0` | `[0x314fd4] == 0x0` |
| `bne` at 0x12492c | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `ble` at 0x1249dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x1249fc | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x124a68 | `r0` | `?` |

## `ioExpRemove`

- `ioExpRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x124a84 to 0x124b4c.
  Established. Word stores write 0 at [sp-24] +4, [sp-24] and [sp-24] +8 (`str` at 0x124b14, `str` at 0x124b20, `str` at 0x124b38).
  The sole call to unnamed 0x20514 has candidate r0/r1/r2/r3 values [[sp-24]+8]/entry r1/[sp-24]/[sp-24] (`bl` at 0x124b2c).
  A prior computed word load has unknown effective address, so the cleared object has no established physical identity (`ldr` at 0x124ac8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x124a94 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x124a98 | load | 4 | `abs 0x124b44` | `-` |
| `ldr` at 0x124a9c | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x124aa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124abc | load | 4 | `abs 0x124b48` | `-` |
| `ldr` at 0x124ac0 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x124ac8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x124ad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124aec | load | 4 | `abs 0x124b48` | `-` |
| `ldr` at 0x124af0 | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x124af8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x124afc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x124b00 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x124b0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x124b14 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x124b18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x124b20 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x124b24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x124b28 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x124b30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x124b38 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldm` at 0x124b40 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x124b40 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x124b40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x124b2c | `(unknown callee) 0x20514` | `[[sp-24]+8]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x124aa4 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x124ad0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x124b08 | `[[sp-24]+4] == 0x0` | `[[sp-24]+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x124b40 | `r0` | `?` |

## `ioExpPresent`

- `ioExpPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x124b4c to 0x124cb8.
  Established. A signed comparison branches to 0x124c0c when (entry r1 & 0xff) is greater than or equal to [[sp-28]+4], and falls through otherwise (`bge` at 0x124bdc);
  its fall-through reaches a byte store of 1 at an unknown effective address (`strb` at 0x124c04).
  The other path calls unnamed 0x6e1b0 with candidate r0/r1/r2/r3 values entry r0/1/0/0 (`bl` at 0x124c1c).
  Neither the loaded word nor the byte destination establishes a physical presence indication.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x124b5c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x124b64 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x124b68 | load | 4 | `abs 0x124ca8` | `-` |
| `ldr` at 0x124b6c | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x124b78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124b8c | load | 4 | `abs 0x124cac` | `-` |
| `ldr` at 0x124b90 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x124b98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x124ba8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124bbc | load | 4 | `abs 0x124cac` | `-` |
| `ldr` at 0x124bc0 | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x124bc8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x124bcc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124bd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x124bd4 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x124be0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x124be4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124bf8 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0x124c04 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x124c0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x124c20 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x124c24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x124c34 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x124c38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x124c60 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x124c64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x124c68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x124c6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x124c78 | load | 4 | `abs 0x124cb0` | `-` |
| `str` at 0x124c7c | store | 4 | `(unknown)` | `0x18fbe4` |
| `str` at 0x124c80 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x124c84 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x124c88 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x124c90 | load | 4 | `abs 0x124cb4` | `-` |
| `ldm` at 0x124ca4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x124ca4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x124ca4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x124c1c | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x124c28 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x124c70 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x124c9c | `(unknown callee) 0x22070` | `0x6` | `0x18fb58` | `0xa9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x124b74 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x124ba0 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x124bdc | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x124ca4 | `r0` | `?` |

## `ioExpNotPresent`

- `ioExpNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x124cb8 to 0x124e24.
  Established. Word loads read absolute 0x314fd8 and 0x314fd4, and another word load reads [sp-28] +4 (`ldr` at 0x124cd8, `ldr` at 0x124cfc, `ldr` at 0x124d40).
  A signed comparison branches to 0x124d78 when (entry r1 & 0xff) is at least [[sp-28]+4] (`bge` at 0x124d48);
  its fall-through reaches a byte store of 0 at an unknown effective address (`strb` at 0x124d70).
  The call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 6/0x18fb58/0xba/0 (`bl` at 0x124e08).
  The modified software field remains Unknown in meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x124cc8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x124cd0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x124cd4 | load | 4 | `abs 0x124e14` | `-` |
| `ldr` at 0x124cd8 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x124ce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124cf8 | load | 4 | `abs 0x124e18` | `-` |
| `ldr` at 0x124cfc | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x124d04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x124d14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124d28 | load | 4 | `abs 0x124e18` | `-` |
| `ldr` at 0x124d2c | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x124d34 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x124d38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124d3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x124d40 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x124d4c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x124d50 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124d64 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0x124d70 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x124d78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x124d8c | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x124d90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x124da0 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x124da4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x124dcc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x124dd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x124dd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x124dd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x124de4 | load | 4 | `abs 0x124e1c` | `-` |
| `str` at 0x124de8 | store | 4 | `(unknown)` | `0x18fc08` |
| `str` at 0x124dec | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x124df0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x124df4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x124dfc | load | 4 | `abs 0x124e20` | `-` |
| `ldm` at 0x124e10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x124e10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x124e10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x124d88 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x124d94 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x124ddc | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x124e08 | `(unknown callee) 0x22070` | `0x6` | `0x18fb58` | `0xba` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x124ce0 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x124d0c | `? != 0x0` | `? == 0x0` |
| `bge` at 0x124d48 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x124e10 | `r0` | `?` |

## `ioExpReset`

- `ioExpReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x124e24 to 0x124ff4.
  Established. Known-edge byte loads read ret(0x126748), [sp-28] +6 and [sp-28] +4 (`ldrb` at 0x124e74, `ldrb` at 0x124e8c, `ldrb` at 0x124e90).
  Later byte stores write ([[sp-32]+3]:1 & 0xff), 5 and 0 at [sp-32] +4, +5 and +3 (`strb` at 0x124f9c, `strb` at 0x124fa8, `strb` at 0x124fb4).
  A call to unnamed 0x127058 has candidate r0/r1/r2/r3 values [sp-20]/[sp-21]:1/[[sp-28]+7]:1/1 (`bl` at 0x124fe0).
  The computed transfer is unresolved (`ldrls` at 0x124ea8), so omitted effects are unknown and the loaded bytes have no established register meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x124e34 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x124e3c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x124e40 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124e44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x124e54 | store | 4 | `sp -0x1c` | `ret(0x126630)` |
| `ldrb` at 0x124e58 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124e5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x124e6c | store | 4 | `sp -0x20` | `ret(0x126748)` |
| `ldr` at 0x124e70 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x124e74 | load | 1 | `ret(0x126748) +0x0` | `-` |
| `ldr` at 0x124e80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x124e84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x124e88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x124e8c | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0x124e90 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0x124e98 | store | 1 | `ret(0x126748) +0x1` | `?` |
| `ldr` at 0x124e9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x124ea0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x124ea8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x124f64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x124f70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x124f78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x124f7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x124f8c | store | 4 | `sp -0x24` | `ret(0x126e4c)` |
| `ldr` at 0x124f90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x124f94 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x124f98 | load | 1 | `[sp-32] +0x3` | `-` |
| `strb` at 0x124f9c | store | 1 | `[sp-32] +0x4` | `([[sp-32]+3]:1 & 0xff)` |
| `ldr` at 0x124fa0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x124fa8 | store | 1 | `[sp-32] +0x5` | `0x5` |
| `ldr` at 0x124fac | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x124fb4 | store | 1 | `[sp-32] +0x3` | `0x0` |
| `ldr` at 0x124fb8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x124fc4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x124fc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x124fcc | load | 1 | `[sp-28] +0x7` | `-` |
| `ldr` at 0x124fd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x124fe8 | store | 4 | `sp -0x24` | `ret(0x127058)` |
| `ldm` at 0x124ff0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x124ff0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x124ff0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x124e4c | `(unknown callee) 0x126630` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x124e64 | `(unknown callee) 0x126748` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x124f84 | `(unknown callee) 0x126e4c` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |
| `bl` at 0x124fe0 | `(unknown callee) 0x127058` | `[sp-20]` | `[sp-21]:1` | `[[sp-28]+7]:1` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x124e7c | `[ret(0x126748)]:1 == 0x0` | `[ret(0x126748)]:1 != 0x0` |
| `ldrls` at 0x124ea8 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0x124f6c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x124fc0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x124ff0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x124ea8).

## `ioExpPreConfig`

- `ioExpPreConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x124ff4 to 0x125150.
  Established. A word store initializes sp-0x24 to 0, and byte loads read [sp-28] +6 and +4 (`str` at 0x125014, `ldrb` at 0x125064, `ldrb` at 0x125068).
  A byte store writes an unknown value at ret(0x126748) +1 (`strb` at 0x125070).
  The call to unnamed 0x126e4c has candidate r0/r1/r2/r3 values [sp-20]/[sp-28]/[sp-32]+2/[sp-32]+2 (`bl` at 0x12510c).
  The computed transfer remains unresolved (`ldrls` at 0x125084), leaving omitted effects and the software-field meanings unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125004 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x12500c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x125014 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0x125018 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x12501c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12502c | store | 4 | `sp -0x1c` | `ret(0x126630)` |
| `ldrb` at 0x125030 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125034 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125044 | store | 4 | `sp -0x20` | `ret(0x126748)` |
| `ldr` at 0x125048 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x12504c | load | 1 | `ret(0x126748) +0x0` | `-` |
| `ldr` at 0x125058 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12505c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x125060 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125064 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0x125068 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0x125070 | store | 1 | `ret(0x126748) +0x1` | `?` |
| `ldr` at 0x125074 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125078 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x125084 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1250dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1250f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x125100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125104 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x125114 | store | 4 | `sp -0x24` | `ret(0x126e4c)` |
| `ldr` at 0x125118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x125124 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x125128 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12512c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125130 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0x125134 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0x12513c | store | 1 | `[sp-32] +0x1` | `?` |
| `ldm` at 0x125144 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125144 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125144 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125024 | `(unknown callee) 0x126630` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x12503c | `(unknown callee) 0x126748` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x1250e8 | `(unknown callee) 0x20fd8` | `ret(0x126748)` | `[[sp-28]+6]:1` | `[sp-28]` | `[sp-36]` |
| `bl` at 0x12510c | `(unknown callee) 0x126e4c` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x125054 | `[ret(0x126748)]:1 == 0x0` | `[ret(0x126748)]:1 != 0x0` |
| `ldrls` at 0x125084 | `[[sp-28]]:1-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x1250e4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x1250f4 | `ret(0x20fd8) == 0x0` | `ret(0x20fd8) != 0x0` |
| `bne` at 0x125120 | `ret(0x126e4c) != 0x0` | `ret(0x126e4c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125144 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x125084).

## `ioExpHardReset`

- `ioExpHardReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125150 to 0x12521c.
  Established. Word loads read absolute 0x314fd8 and 0x314fd4, followed later by a word load at [sp-28] +4 (`ldr` at 0x125168, `ldr` at 0x12518c, `ldr` at 0x1251e0).
  Calls to unnamed 0x127c40 and 0x124e24 have candidate r0 values entry r0 and [sp-20], respectively (`bl` at 0x1251cc, `bl` at 0x1251f8).
  A byte store writes ([sp-21]:1+1 & 0xff) at sp-0x15 before a backward transfer to 0x1251d8 (`strb` at 0x125204, `b` at 0x125208).
  The calls do not identify which physical reset operations, if any, occur.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125160 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x125164 | load | 4 | `abs 0x125214` | `-` |
| `ldr` at 0x125168 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x125174 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125188 | load | 4 | `abs 0x125218` | `-` |
| `ldr` at 0x12518c | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x125194 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1251a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1251b8 | load | 4 | `abs 0x125218` | `-` |
| `ldr` at 0x1251bc | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x1251c4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1251c8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x1251d4 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x1251d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1251dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1251e0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x1251ec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1251f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1251fc | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x125204 | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `ldm` at 0x125210 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125210 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125210 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1251cc | `(unknown callee) 0x127c40` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x1251f8 | `(unknown callee) 0x124e24` | `[sp-20]` | `?` | `[[sp-28]+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x125170 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x12519c | `? != 0x0` | `? == 0x0` |
| `bge` at 0x1251e8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125210 | `r0` | `?` |

## `ioExpSoftResetData`

- `ioExpSoftResetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12521c to 0x125468.
  Established. One path stores 0xfffffc11 at sp-0x3c, while the final loop-exit path stores 0 there (`str` at 0x125270, `str` at 0x125440).
  The call to unnamed 0x126e4c has candidate r0/r1/r2/r3 values [sp-24]/[sp-28]/ret(0x126748)+2/ret(0x126748)+2 (`bl` at 0x12535c).
  A later byte store writes an unknown value at [sp-32] +1, and the loop updates sp-0x21 to ([sp-33]:1+1 & 0xff) (`strb` at 0x125428, `strb` at 0x125434).
  The byte's meaning and the unnamed callee's physical effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12522c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x125230 | load | 4 | `abs 0x125450` | `-` |
| `ldr` at 0x125234 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x125240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x125254 | load | 4 | `abs 0x125454` | `-` |
| `ldr` at 0x125258 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x125260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x12526c | load | 4 | `abs 0x125458` | `-` |
| `str` at 0x125270 | store | 4 | `sp -0x3c` | `0xfffffc11` |
| `ldr` at 0x125278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12528c | load | 4 | `abs 0x125454` | `-` |
| `ldr` at 0x125290 | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x125298 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x1252a4 | store | 4 | `sp -0x28` | `ret(0x10a620)` |
| `ldr` at 0x1252a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1252b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1252c0 | load | 4 | `abs 0x12545c` | `-` |
| `str` at 0x1252c4 | store | 4 | `sp -0x44` | `0x18fc78` |
| `str` at 0x1252c8 | store | 4 | `sp -0x40` | `ret(0x219a4)` |
| `ldr` at 0x1252d0 | load | 4 | `abs 0x125460` | `-` |
| `str` at 0x1252e4 | store | 4 | `sp -0x3c` | `?` |
| `strb` at 0x1252f0 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x1252f4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x1252f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1252fc | load | 4 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0x125308 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x12530c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12531c | store | 4 | `sp -0x1c` | `ret(0x126630)` |
| `ldrb` at 0x125320 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x125324 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x125334 | store | 4 | `sp -0x20` | `ret(0x126748)` |
| `ldr` at 0x125338 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x12533c | load | 1 | `ret(0x126748) +0x0` | `-` |
| `ldr` at 0x125348 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x125350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x125354 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x125364 | store | 4 | `sp -0x28` | `ret(0x126e4c)` |
| `ldr` at 0x125368 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x125374 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x125388 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x12538c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12539c | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x1253a0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1253c8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1253cc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1253d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1253d4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1253e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1253ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1253f0 | load | 4 | `abs 0x125464` | `-` |
| `ldr` at 0x1253f8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x125408 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x125410 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x125414 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x125418 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x12541c | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0x125420 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0x125428 | store | 1 | `[sp-32] +0x1` | `?` |
| `ldrb` at 0x12542c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x125434 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `str` at 0x125440 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x125444 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x12544c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12544c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12544c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12544c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12529c | `(unknown callee) 0x10a620` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x1252b8 | `(unknown callee) 0x219a4` | `ret(0x10a620)` | `?` | `?` | `ret(0x10a620)` |
| `bl` at 0x1252dc | `(unknown callee) 0x22070` | `0x1` | `0x18fb58` | `0x170` | `0x0` |
| `bl` at 0x125314 | `(unknown callee) 0x126630` | `[sp-24]` | `?` | `[[sp-44]+4]` | `?` |
| `bl` at 0x12532c | `(unknown callee) 0x126748` | `[sp-24]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0x12535c | `(unknown callee) 0x126e4c` | `[sp-24]` | `[sp-28]` | `ret(0x126748)+2` | `ret(0x126748)+2` |
| `bl` at 0x125384 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x125390 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1253d8 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x1253e4 | `(unknown callee) 0x219a4` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x125400 | `(unknown callee) 0x29908` | `0x18fca4` | `ret(0x203f4)` | `[[sp-28]+12]` | `ret(0x219a4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12523c | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x125268 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x1252b0 | `ret(0x10a620) == 0x0` | `ret(0x10a620) != 0x0` |
| `bge` at 0x125304 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x125344 | `[ret(0x126748)]:1 == 0x0` | `[ret(0x126748)]:1 != 0x0` |
| `beq` at 0x125370 | `ret(0x126e4c) == 0x0` | `ret(0x126e4c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12544c | `r0` | `?` |

## `ioExpSoftResetInt`

- `ioExpSoftResetInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125468 to 0x12576c.
  Established. Known-edge byte stores write ([ret(0x126748)+3]:1 & 0xff) at ret(0x126748) +4 and 5 at [sp-28] +5 (`strb` at 0x1255dc, `strb` at 0x1255e8).
  The unknown-target call has candidate r0/r1/r2/r3 values [sp-20]/[sp-24]/[sp-28]+3/[sp-28]+3 (`mov` at 0x125624).
  Another call to unnamed 0x125cf8 has candidate r2=0xff with r0=[sp-20] and r1/r3=[sp-29]:1 (`bl` at 0x125724).
  The unresolved computed transfer leaves omitted effects unknown (`ldrls` at 0x125554), and none of these bytes is established as an interrupt register.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125478 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x12547c | load | 4 | `abs 0x12574c` | `-` |
| `ldr` at 0x125480 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x12548c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1254a0 | load | 4 | `abs 0x125750` | `-` |
| `ldr` at 0x1254a4 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x1254ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1254b8 | load | 4 | `abs 0x125754` | `-` |
| `str` at 0x1254bc | store | 4 | `sp -0x38` | `0xfffffc11` |
| `ldr` at 0x1254c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1254d8 | load | 4 | `abs 0x125750` | `-` |
| `ldr` at 0x1254dc | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x1254e4 | store | 4 | `sp -0x28` | `?` |
| `strb` at 0x1254ec | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x1254f0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x1254f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1254f8 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0x125504 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x125508 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125518 | store | 4 | `sp -0x18` | `ret(0x126630)` |
| `ldrb` at 0x12551c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x125520 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125530 | store | 4 | `sp -0x1c` | `ret(0x126748)` |
| `ldr` at 0x125534 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125538 | load | 1 | `ret(0x126748) +0x0` | `-` |
| `ldr` at 0x125544 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x125548 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0x125554 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1255cc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x1255d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1255d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1255d8 | load | 1 | `ret(0x126748) +0x3` | `-` |
| `strb` at 0x1255dc | store | 1 | `ret(0x126748) +0x4` | `([ret(0x126748)+3]:1 & 0xff)` |
| `ldr` at 0x1255e0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x1255e8 | store | 1 | `[sp-28] +0x5` | `0x5` |
| `ldr` at 0x1255ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1255f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1255f4 | load | 1 | `[sp-24] +0x7` | `-` |
| `strb` at 0x1255f8 | store | 1 | `[sp-28] +0x3` | `([[sp-24]+7]:1 & 0xff)` |
| `ldr` at 0x1255fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x125608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x125610 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x125614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125618 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x125638 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x125640 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125644 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0x125648 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12564c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x125660 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125674 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x125678 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x125688 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x12568c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1256b4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1256b8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1256bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1256c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1256cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1256d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1256d4 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0x1256d8 | load | 4 | `abs 0x125768` | `-` |
| `ldr` at 0x1256e0 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x1256ec | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1256f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1256f8 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0x125704 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x125708 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0x125714 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x125718 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x125728 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x125730 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0x12573c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x125740 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x125748 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125748 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125748 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125510 | `(unknown callee) 0x126630` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0x125528 | `(unknown callee) 0x126748` | `[sp-20]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `mov` at 0x125624 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-24]` | `[sp-28]+3` | `[sp-28]+3` |
| `bl` at 0x125670 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x12567c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1256c4 | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x1256e4 | `(unknown callee) 0x29908` | `0x18fccc` | `ret(0x203f4)` | `[[sp-24]+12]` | `[[sp-28]+3]:1` |
| `bl` at 0x125724 | `(unknown callee) 0x125cf8` | `[sp-20]` | `[sp-29]:1` | `0xff` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x125488 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x1254b4 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x125500 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x125540 | `[ret(0x126748)]:1 == 0x0` | `[ret(0x126748)]:1 != 0x0` |
| `ldrls` at 0x125554 | `[[sp-24]]:1-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-24]]:1-1 > 0x3 (unsigned, 32-bit)` |
| `beq` at 0x125604 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x125630 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x12565c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x125700 | `[[sp-28]+3]:1 == 0x0` | `[[sp-28]+3]:1 != 0x0` |
| `bne` at 0x125710 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125748 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x125554).

## `ioExpGetCfg`

- `ioExpGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12576c to 0x12586c.
  Established. Word loads read [sp-28] and [sp-28] +4 (`ldr` at 0x125840, `ldr` at 0x12584c).
  A word store writes [[sp-28]] at entry r1, while other paths store 0 or 0xfffffc11 at sp-0x20 (`str` at 0x125844, `str` at 0x1257c4, `str` at 0x12580c).
  No calls are emitted and the recorded r0 return is unknown, so the returned word's configuration meaning is unestablished (`ldm` at 0x12585c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12577c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x125780 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x125784 | load | 4 | `abs 0x125860` | `-` |
| `ldr` at 0x125788 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x125794 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1257a8 | load | 4 | `abs 0x125864` | `-` |
| `ldr` at 0x1257ac | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x1257b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1257c4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1257cc | load | 4 | `abs 0x125860` | `-` |
| `ldr` at 0x1257d0 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x1257dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1257f0 | load | 4 | `abs 0x125864` | `-` |
| `ldr` at 0x1257f4 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x1257fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x125808 | load | 4 | `abs 0x125868` | `-` |
| `str` at 0x12580c | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0x125814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125828 | load | 4 | `abs 0x125864` | `-` |
| `ldr` at 0x12582c | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x125834 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x125838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12583c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x125840 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x125844 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x125848 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12584c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x125850 | store | 4 | `sp -0x20` | `[[sp-28]+4]` |
| `ldr` at 0x125854 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x12585c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12585c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12585c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x125790 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x1257bc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x1257d8 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x125804 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12585c | `r0` | `?` |

## `ioExpCount`

- `ioExpCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12586c to 0x125958.
  Established. The branch takes the path to 0x1258bc when [0x314fd8] is 0, reaching a word store of 0 at sp-0x1c (`beq` at 0x12588c, `str` at 0x1258c0).
  Another path stores 0xfffffc11 there, while the last path loads [sp-24] +4 and saves that word at sp-0x1c (`str` at 0x125908, `ldr` at 0x125938, `str` at 0x12593c).
  No calls are emitted and the recorded r0 return is unknown, leaving any count interpretation Unknown (`ldm` at 0x125948).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12587c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x125880 | load | 4 | `abs 0x12594c` | `-` |
| `ldr` at 0x125884 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x125890 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1258a4 | load | 4 | `abs 0x125950` | `-` |
| `ldr` at 0x1258a8 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x1258b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1258c0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x1258c8 | load | 4 | `abs 0x12594c` | `-` |
| `ldr` at 0x1258cc | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x1258d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1258ec | load | 4 | `abs 0x125950` | `-` |
| `ldr` at 0x1258f0 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x1258f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x125904 | load | 4 | `abs 0x125954` | `-` |
| `str` at 0x125908 | store | 4 | `sp -0x1c` | `0xfffffc11` |
| `ldr` at 0x125910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125924 | load | 4 | `abs 0x125950` | `-` |
| `ldr` at 0x125928 | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x125930 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x125934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x125938 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0x12593c | store | 4 | `sp -0x1c` | `[[sp-24]+4]` |
| `ldr` at 0x125940 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x125948 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125948 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125948 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12588c | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x1258b8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x1258d4 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x125900 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125948 | `r0` | `?` |

## `ioExpIsPresent`

- `ioExpIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125958 to 0x1259bc.
  Established. The sole call to unnamed 0x126748 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/entry r2/(entry r1 & 0xff) (`bl` at 0x125980).
  A nonzero call result takes the branch to 0x1259a4, where a byte load reads ret(0x126748), while the zero-result path reaches a store of 0 at sp-0x20 (`bne` at 0x125994, `ldrb` at 0x1259a8, `str` at 0x12599c).
  The recorded r0 return is [sp-32], whose relationship to physical presence remains Unknown (`ldm` at 0x1259b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125968 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125970 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x125974 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125978 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125988 | store | 4 | `sp -0x1c` | `ret(0x126748)` |
| `ldr` at 0x12598c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x12599c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1259a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1259a8 | load | 1 | `ret(0x126748) +0x0` | `-` |
| `str` at 0x1259ac | store | 4 | `sp -0x20` | `[ret(0x126748)]:1` |
| `ldr` at 0x1259b0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x1259b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1259b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1259b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125980 | `(unknown callee) 0x126748` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x125994 | `ret(0x126748) != 0x0` | `ret(0x126748) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1259b8 | `r0` | `[sp-32]` |

## `ioExpSetBit`

- `ioExpSetBit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1259bc to 0x125a0c.
  Established. Byte stores save (entry r1 & 0xff) and (entry r2 & 0xff) at sp-0x15 and sp-0x16 (`strb` at 0x1259d4, `strb` at 0x1259dc).
  The sole call to unnamed 0x125a58 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/(entry r2 & 0xff)/(entry r2 & 0xff) (`bl` at 0x1259f8);
  its result is the recorded r0 return, and this wrapper alone establishes no physical bit assignment (`ldm` at 0x125a08).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1259cc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1259d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x1259dc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x1259e0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x1259e4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x1259e8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x1259ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x125a08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125a08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125a08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1259f8 | `(unknown callee) 0x125a58` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125a08 | `r0` | `ret(0x125a58)` |

## `ioExpClearBit`

- `ioExpClearBit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125a0c to 0x125a58.
  Established. A word store saves entry r0 at sp-0x14 and byte stores save masked entry r1/r2 at sp-0x15 and sp-0x16 (`str` at 0x125a1c, `strb` at 0x125a24, `strb` at 0x125a2c).
  The call to unnamed 0x125a58 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/(entry r2 & 0xff)/0 (`bl` at 0x125a44).
  The recorded r0 return is that call result, with the affected byte or bit identity still Unknown (`ldm` at 0x125a54).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125a1c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125a24 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125a2c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x125a30 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x125a34 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x125a38 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x125a54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125a54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125a54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125a44 | `(unknown callee) 0x125a58` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125a54 | `r0` | `ret(0x125a58)` |

## `ioExpWrite`

- `ioExpWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125a58 to 0x125b08.
  Established. The call to unnamed 0x126748 has candidate r0/r1 values [sp-20]/[sp-21]:1, and a byte load then reads its result address (`bl` at 0x125aac, `ldrb` at 0x125abc).
  A nonzero loaded byte takes the branch to 0x125ad4, while the other path stores 0xfffffc11 at sp-0x28 (`bne` at 0x125ac4, `str` at 0x125acc).
  The later call to unnamed 0x12686c has candidate r0/r1/r2/r3 values [sp-20]/[sp-28]/ret(0x126748)/[sp-22]:1 (`bl` at 0x125aec).
  The addressed software objects and the call's physical write behavior remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125a68 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125a74 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125a7c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x125a84 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x125a88 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125a8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125a9c | store | 4 | `sp -0x1c` | `ret(0x126630)` |
| `ldrb` at 0x125aa0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125aa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125ab4 | store | 4 | `sp -0x20` | `ret(0x126748)` |
| `ldr` at 0x125ab8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x125abc | load | 1 | `ret(0x126748) +0x0` | `-` |
| `ldr` at 0x125ac8 | load | 4 | `abs 0x125b04` | `-` |
| `str` at 0x125acc | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldrb` at 0x125ad4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x125ad8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x125adc | store | 4 | `sp -0x2c` | `[sp-23]:1` |
| `ldr` at 0x125ae0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125ae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x125ae8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x125af4 | store | 4 | `sp -0x28` | `ret(0x12686c)` |
| `ldr` at 0x125af8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x125b00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125b00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125b00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125a94 | `(unknown callee) 0x126630` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x125aac | `(unknown callee) 0x126748` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x125aec | `(unknown callee) 0x12686c` | `[sp-20]` | `[sp-28]` | `ret(0x126748)` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x125ac4 | `[ret(0x126748)]:1 != 0x0` | `[ret(0x126748)]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125b00 | `r0` | `?` |
