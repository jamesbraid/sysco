# ARM PIO, FRU and controller communication in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
Undecodable byte ranges remain visible as fact-sheet warnings. Values and predicates stay unknown where the analysis cannot follow an effect.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/pio-fru-controller-communication-facts.txt) retain all tool sections.

## `FruFmtMultiRec`

- `FruFmtMultiRec`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x98568 to 0x98ca4.
  Established. A byte of zero is stored at `r3 +0x0` (`strb` at 0x98594). The computed transfer is
  taken when `[sp-28] <= 0x5 (unsigned, 32-bit)` and falls through when `[sp-28] > 0x5 (unsigned,
  32-bit)` (`ldrls` at 0x985a0); its taken target is unresolved. The known fall-through route
  returns entry `r0` (`ldmdb` at 0x98c44). The record-formatting role remains unknown beyond these
  known edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x98578 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x9857c | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x98580 | store | 4 | `sp -0x28` | `r3` |
| `strb` at 0x98588 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0x9858c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x98594 | store | 1 | `r3 +0x0` | `0x0` |
| `ldr` at 0x98598 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x985a0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x98c44 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x98c44 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x98c44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x98c44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x98c44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x985a0 | `[sp-28] <= 0x5 (unsigned, 32-bit)` | `[sp-28] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x98c44 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x985a0).

## `PIOWrite_Common`

- `PIOWrite_Common`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc21c to 0xcc9c0.
  Established. The early word reads have unknown effective targets (`ldr` at 0xcc260, `ldr` at
  0xcc28c). A call to `0xbc308` has r0–r3 candidates `0x1`, `0x0`, `[sp+16]`, and unknown (`bl` at
  0xcc318). Another call to `0xca57c` has candidates `[sp-76]`, `[sp-84]`, `[sp-80]`, and `sp-56`
  (`bl` at 0xcc3a4). The branch is taken when `ret(0xbc8b4) != 0x0` and falls through on equality
  (`bne` at 0xcc3e4). The PIO hardware mapping is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc22c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcc230 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcc234 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcc238 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcc23c | load | 4 | `abs 0xcc988` | `-` |
| `ldr` at 0xcc240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc260 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcc264 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xcc268 | load | 4 | `abs 0xcc988` | `-` |
| `ldr` at 0xcc26c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc28c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcc290 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xcc294 | load | 4 | `abs 0xcc988` | `-` |
| `ldr` at 0xcc298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc2b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc2c0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcc2c8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcc2cc | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xcc2d0 | load | 4 | `abs 0xcc988` | `-` |
| `ldr` at 0xcc2d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc2f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc2fc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcc304 | load | 2 | `(unknown)` | `-` |
| `str` at 0xcc308 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xcc314 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcc31c | load | 4 | `abs 0xcc988` | `-` |
| `ldr` at 0xcc320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc340 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc348 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcc350 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcc360 | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0xcc368 | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0xcc374 | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0xcc37c | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0xcc380 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcc384 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xcc38c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc390 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xcc394 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcc398 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcc39c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xcc3a8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xcc3ac | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xcc3b0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc3b4 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc3b8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc3bc | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcc3c0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcc3c4 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xcc3ec | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xcc3f4 | load | 4 | `abs 0xcc98c` | `-` |
| `ldr` at 0xcc3f8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcc404 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc408 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcc40c | load | 4 | `abs 0xcc990` | `-` |
| `ldr` at 0xcc410 | load | 4 | `abs 0xcc994` | `-` |
| `ldr` at 0xcc414 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcc418 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcc420 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xcc424 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xcc428 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xcc42c | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xcc44c | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xcc450 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc454 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc458 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcc45c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc46c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcc470 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xcc474 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc478 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc47c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc480 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcc484 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcc488 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xcc4b0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xcc4b8 | load | 4 | `abs 0xcc98c` | `-` |
| `ldr` at 0xcc4bc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcc4c8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc4cc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcc4d0 | load | 4 | `abs 0xcc998` | `-` |
| `ldr` at 0xcc4d4 | load | 4 | `abs 0xcc994` | `-` |
| `ldr` at 0xcc4d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc4dc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcc4e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcc4e8 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0xcc4f0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc4f4 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xcc4f8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcc4fc | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcc500 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xcc50c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xcc510 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xcc514 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc518 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc51c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc520 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcc524 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcc528 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xcc550 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xcc558 | load | 4 | `abs 0xcc98c` | `-` |
| `ldr` at 0xcc55c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcc568 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc56c | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcc570 | load | 4 | `abs 0xcc99c` | `-` |
| `ldr` at 0xcc574 | load | 4 | `abs 0xcc994` | `-` |
| `ldr` at 0xcc578 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcc57c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcc584 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcc590 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcc594 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcc59c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcc5a0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcc5a8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcc5ac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc5b0 | load | 4 | `abs 0xcc9a0` | `-` |
| `ldr` at 0xcc5bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc5c4 | load | 4 | `abs 0xcc9a0` | `-` |
| `str` at 0xcc5d0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcc5d4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc5d8 | load | 4 | `abs 0xcc9a4` | `-` |
| `ldr` at 0xcc5e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc5ec | load | 4 | `abs 0xcc9a4` | `-` |
| `str` at 0xcc5f8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcc5fc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc600 | load | 4 | `abs 0xcc9a8` | `-` |
| `ldr` at 0xcc60c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc614 | load | 4 | `abs 0xcc9a8` | `-` |
| `str` at 0xcc620 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcc624 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc634 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcc648 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xcc64c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xcc65c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcc670 | store | 4 | `sp -0x38` | `?` |
| `str` at 0xcc67c | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xcc680 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc684 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc688 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcc68c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc69c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcc6a0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xcc6a4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc6a8 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc6ac | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc6b0 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcc6b4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcc6b8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xcc6e0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xcc6e8 | load | 4 | `abs 0xcc98c` | `-` |
| `ldr` at 0xcc6ec | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcc6f8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc6fc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcc700 | load | 4 | `abs 0xcc9ac` | `-` |
| `ldr` at 0xcc704 | load | 4 | `abs 0xcc994` | `-` |
| `ldr` at 0xcc708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc70c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcc714 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcc724 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xcc72c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcc730 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0xcc738 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc73c | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0xcc740 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcc744 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcc748 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xcc754 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xcc758 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0xcc75c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc760 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc764 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc768 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcc76c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcc770 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xcc798 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xcc7a0 | load | 4 | `abs 0xcc98c` | `-` |
| `ldr` at 0xcc7a4 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcc7b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc7b4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcc7b8 | load | 4 | `abs 0xcc990` | `-` |
| `ldr` at 0xcc7bc | load | 4 | `abs 0xcc994` | `-` |
| `ldr` at 0xcc7c0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcc7c4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc7d0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xcc7d4 | load | 4 | `abs 0xcc9b0` | `-` |
| `str` at 0xcc7dc | store | 4 | `abs 0x27ecb8` | `0x0` |
| `ldr` at 0xcc7e0 | load | 4 | `abs 0xcc9b0` | `-` |
| `ldr` at 0xcc7e4 | load | 4 | `abs 0x27ecb8` | `-` |
| `ldr` at 0xcc7f4 | load | 4 | `abs 0xcc9b0` | `-` |
| `ldr` at 0xcc7f8 | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcc814 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcc818 | load | 4 | `abs 0xcc9b0` | `-` |
| `ldr` at 0xcc81c | load | 4 | `abs 0xcc9b0` | `-` |
| `ldr` at 0xcc820 | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcc828 | store | 4 | `abs 0x27ecb8` | `[0x27ecb8]+1` |
| `str` at 0xcc838 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0xcc83c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc840 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc844 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcc848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc854 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc860 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcc86c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcc870 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xcc874 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcc878 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0xcc87c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc880 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcc884 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcc888 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0xcc8b0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xcc8b8 | load | 4 | `abs 0xcc98c` | `-` |
| `ldr` at 0xcc8bc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcc8c8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcc8cc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcc8d0 | load | 4 | `abs 0xcc9b4` | `-` |
| `ldr` at 0xcc8d4 | load | 4 | `abs 0xcc994` | `-` |
| `ldr` at 0xcc8d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc8dc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcc8e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc8f0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xcc904 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc910 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xcc924 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcc928 | load | 4 | `abs 0xcc9b8` | `-` |
| `str` at 0xcc934 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xcc940 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xcc944 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcc94c | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xcc954 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcc960 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcc964 | load | 4 | `abs 0xcc9bc` | `-` |
| `str` at 0xcc970 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xcc97c | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xcc980 | load | 4 | `sp -0x58` | `-` |
| `ldmdb` at 0xcc984 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc984 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc984 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc318 | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xcc3a4 | `(unknown callee) 0xca57c` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xcc3d8 | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xcc41c | `(unknown callee) 0xca8c0` | `0x10b20c` | `0x10b21c` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xcc440 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xcc468 | `(unknown callee) 0xca7e0` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xcc49c | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcc4e0 | `(unknown callee) 0xca8c0` | `0x10b220` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcc508 | `(unknown callee) 0xca57c` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xcc53c | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcc580 | `(unknown callee) 0xca8c0` | `0x10b230` | `0x10b21c` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xcc698 | `(unknown callee) 0xca7e0` | `[sp-72]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xcc6cc | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcc710 | `(unknown callee) 0xca8c0` | `0x10b240` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcc750 | `(unknown callee) 0xca57c` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0xcc784 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcc7c8 | `(unknown callee) 0xca8c0` | `0x10b20c` | `0x10b21c` | `[sp-76]` | `[sp+8]` |
| `bl` at 0xcc850 | `(unknown callee) 0xca7e0` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xcc89c | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcc8e0 | `(unknown callee) 0xca8c0` | `0x10b250` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcc8f4 | `(unknown callee) 0xc162c` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xcc914 | `(unknown callee) 0xc162c` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xcc92c | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b268` | `?` | `ret(0xc162c)` |
| `bl` at 0xcc968 | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b29c` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcc358 | `? != 0x6` | `? == 0x6` |
| `bne` at 0xcc3e4 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcc400 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcc4a8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcc4c4 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcc548 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcc564 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcc6d8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcc6f4 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcc71c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xcc790 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcc7ac | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xcc7ec | `[0x27ecb8] <= 0x2 (unsigned, 32-bit)` | `[0x27ecb8] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xcc85c | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xcc868 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xcc8a8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcc8c4 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xcc900 | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `beq` at 0xcc920 | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `bne` at 0xcc95c | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc984 | `r0` | `?` |

## `PIORead_Common`

- `PIORead_Common`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc9c0 to 0xcd054.
  Established. The call to `0xbc308` carries candidates `0x1`, `0x0`, `[sp+16]`, and unknown (`bl`
  at 0xccabc). The call to `0xca57c` instead carries `[sp-92]`, `[sp-100]`, `[sp-96]`, and `sp-56`
  (`bl` at 0xccb60). A branch takes the nonzero `ret(0xbc8b4)` edge and falls through for zero
  (`bne` at 0xccba0). The return value is unknown (`ldmdb` at 0xcd01c), as is the physical read
  interface.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc9d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcc9d4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcc9d8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcc9dc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcc9e0 | load | 4 | `abs 0xcd020` | `-` |
| `ldr` at 0xcc9e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcca04 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcca08 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xcca0c | load | 4 | `abs 0xcd020` | `-` |
| `ldr` at 0xcca10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcca30 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcca34 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xcca38 | load | 4 | `abs 0xcd020` | `-` |
| `ldr` at 0xcca3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcca5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcca64 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcca6c | load | 1 | `(unknown)` | `-` |
| `str` at 0xcca70 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xcca74 | load | 4 | `abs 0xcd020` | `-` |
| `ldr` at 0xcca78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcca98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccaa0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xccaa8 | load | 2 | `(unknown)` | `-` |
| `str` at 0xccaac | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xccab8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xccac0 | load | 4 | `abs 0xcd020` | `-` |
| `ldr` at 0xccac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccaec | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xccaf4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xccb04 | store | 4 | `sp -0x50` | `0xd0000000` |
| `str` at 0xccb0c | store | 4 | `sp -0x54` | `0x50000000` |
| `str` at 0xccb18 | store | 4 | `sp -0x50` | `0x8000000` |
| `str` at 0xccb20 | store | 4 | `sp -0x54` | `0xf0000000` |
| `ldr` at 0xccb24 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xccb28 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xccb30 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xccb38 | store | 4 | `[sp] +0x0` | `0x0` |
| `ldr` at 0xccb3c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xccb40 | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0xccb48 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xccb4c | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xccb50 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xccb54 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xccb58 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xccb64 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xccb68 | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xccb6c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xccb70 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xccb74 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xccb78 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xccb7c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xccb80 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xccba8 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xccbb0 | load | 4 | `abs 0xcd024` | `-` |
| `ldr` at 0xccbb4 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xccbc0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xccbc4 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xccbc8 | load | 4 | `abs 0xcd028` | `-` |
| `ldr` at 0xccbcc | load | 4 | `abs 0xcd02c` | `-` |
| `ldr` at 0xccbd0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xccbd4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xccbdc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xccbe8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xccbec | load | 4 | `sp -0x38` | `-` |
| `str` at 0xccbf4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xccbf8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccbfc | load | 4 | `abs 0xcd030` | `-` |
| `ldr` at 0xccc08 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccc10 | load | 4 | `abs 0xcd030` | `-` |
| `str` at 0xccc1c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xccc20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccc24 | load | 4 | `abs 0xcd034` | `-` |
| `ldr` at 0xccc30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccc38 | load | 4 | `abs 0xcd034` | `-` |
| `str` at 0xccc44 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xccc48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccc4c | load | 4 | `abs 0xcd038` | `-` |
| `ldr` at 0xccc58 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccc60 | load | 4 | `abs 0xcd038` | `-` |
| `str` at 0xccc6c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xccc70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccc80 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xccc94 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xccc98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xccca8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcccbc | store | 4 | `sp -0x38` | `?` |
| `str` at 0xcccc8 | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xccccc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcccd0 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xcccd4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xcccd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccce8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcccec | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xcccf0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcccf4 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xcccf8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcccfc | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xccd00 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xccd04 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xccd2c | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xccd34 | load | 4 | `abs 0xcd024` | `-` |
| `ldr` at 0xccd38 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xccd44 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xccd48 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xccd4c | load | 4 | `abs 0xcd03c` | `-` |
| `ldr` at 0xccd50 | load | 4 | `abs 0xcd02c` | `-` |
| `ldr` at 0xccd54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xccd58 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xccd60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xccd70 | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xccd78 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xccd7c | store | 4 | `sp -0x38` | `[sp-80]` |
| `ldr` at 0xccd84 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xccd88 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0xccd8c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xccd90 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xccd94 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xccda0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xccda4 | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0xccda8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xccdac | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xccdb0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xccdb4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xccdb8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xccdbc | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xccde4 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xccdec | load | 4 | `abs 0xcd024` | `-` |
| `ldr` at 0xccdf0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xccdfc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcce00 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xcce04 | load | 4 | `abs 0xcd040` | `-` |
| `ldr` at 0xcce08 | load | 4 | `abs 0xcd02c` | `-` |
| `ldr` at 0xcce0c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xcce10 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcce1c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xcce20 | load | 4 | `abs 0xcd044` | `-` |
| `str` at 0xcce28 | store | 4 | `abs 0x27ecb8` | `0x0` |
| `ldr` at 0xcce2c | load | 4 | `abs 0xcd044` | `-` |
| `ldr` at 0xcce30 | load | 4 | `abs 0x27ecb8` | `-` |
| `ldr` at 0xcce40 | load | 4 | `abs 0xcd044` | `-` |
| `ldr` at 0xcce44 | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcce60 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcce64 | load | 4 | `abs 0xcd044` | `-` |
| `ldr` at 0xcce68 | load | 4 | `abs 0xcd044` | `-` |
| `ldr` at 0xcce6c | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcce74 | store | 4 | `abs 0x27ecb8` | `[0x27ecb8]+1` |
| `str` at 0xcce84 | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0xcce88 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcce8c | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xcce90 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xcce94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccea0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcceac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcceb8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xccebc | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0xccec0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xccec4 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0xccec8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xccecc | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcced0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcced4 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0xccefc | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xccf04 | load | 4 | `abs 0xcd024` | `-` |
| `ldr` at 0xccf08 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xccf14 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xccf18 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0xccf1c | load | 4 | `abs 0xcd048` | `-` |
| `ldr` at 0xccf20 | load | 4 | `abs 0xcd02c` | `-` |
| `ldr` at 0xccf24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xccf28 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xccf30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccf3c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xccf50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccf5c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xccf70 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xccf74 | load | 4 | `abs 0xcd04c` | `-` |
| `str` at 0xccf80 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xccf90 | store | 4 | `sp -0x78` | `0x42` |
| `ldr` at 0xccf94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccf98 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xccfbc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xccfc0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xccfc4 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xccfc8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xccfcc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xccfd0 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xccfd8 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xccfdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xccfe4 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xccfec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xccff8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xccffc | load | 4 | `abs 0xcd050` | `-` |
| `str` at 0xcd008 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xcd014 | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0xcd018 | load | 4 | `sp -0x68` | `-` |
| `ldmdb` at 0xcd01c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd01c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd01c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccabc | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xccb60 | `(unknown callee) 0xca57c` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xccb94 | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xccbd8 | `(unknown callee) 0xca8c0` | `0x10b2d4` | `0x10b21c` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xccce4 | `(unknown callee) 0xca7e0` | `[sp-88]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0xccd18 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xccd5c | `(unknown callee) 0xca8c0` | `0x10b2e4` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xccd9c | `(unknown callee) 0xca57c` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0xccdd0 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcce14 | `(unknown callee) 0xca8c0` | `0x10b2f4` | `0x10b21c` | `[sp-92]` | `[sp+8]` |
| `bl` at 0xcce9c | `(unknown callee) 0xca7e0` | `[sp-88]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0xccee8 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xccf2c | `(unknown callee) 0xca8c0` | `0x10b304` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xccf40 | `(unknown callee) 0xc162c` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0xccf60 | `(unknown callee) 0xc162c` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0xccf78 | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b268` | `?` | `ret(0xc162c)` |
| `bl` at 0xccfa4 | `(unknown callee) 0xc1ab4` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xccfb8 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xcd000 | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b31c` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xccafc | `? != 0x6` | `? == 0x6` |
| `bne` at 0xccba0 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xccbbc | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xccd24 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xccd40 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xccd68 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xccddc | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xccdf8 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xcce38 | `[0x27ecb8] <= 0x2 (unsigned, 32-bit)` | `[0x27ecb8] > 0x2 (unsigned, 32-bit)` |
| `ble` at 0xccea8 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xcceb4 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xccef4 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xccf10 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xccf4c | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `beq` at 0xccf6c | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `bne` at 0xccff4 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd01c | `r0` | `?` |

## `PIOWrite_PIC`

- `PIOWrite_PIC`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd054 to 0xcd76c.
  Established. A call to `0xca57c` exposes candidates `[sp-68]`, `[sp-76]`, `[sp-72]`, and `sp-56`
  (`bl` at 0xcd178). The branch on `ret(0xbc8b4)` is taken for nonzero and falls through for zero
  (`bne` at 0xcd1b8). A later branch takes the unsigned `[0x27eb98] <= 0x2` edge and falls through
  for values greater than `0x2` (`bls` at 0xcd1d4). The PIC destination and hardware role are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd064 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd068 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcd06c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcd070 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcd074 | load | 4 | `abs 0xcd73c` | `-` |
| `ldr` at 0xcd078 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd098 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcd09c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xcd0a0 | load | 4 | `abs 0xcd73c` | `-` |
| `ldr` at 0xcd0a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd0c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcd0c8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xcd0cc | load | 4 | `abs 0xcd73c` | `-` |
| `ldr` at 0xcd0d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd0f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd0f8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcd100 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcd104 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xcd108 | load | 4 | `abs 0xcd73c` | `-` |
| `ldr` at 0xcd10c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd12c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd134 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcd13c | load | 2 | `(unknown)` | `-` |
| `str` at 0xcd140 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xcd14c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd158 | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xcd160 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd164 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xcd168 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd16c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcd170 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcd17c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcd180 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xcd184 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd188 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd18c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd190 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xcd194 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd198 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xcd1c0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xcd1c8 | load | 4 | `abs 0xcd740` | `-` |
| `ldr` at 0xcd1cc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd1d8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd1dc | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcd1e0 | load | 4 | `abs 0xcd744` | `-` |
| `ldr` at 0xcd1e4 | load | 4 | `abs 0xcd748` | `-` |
| `ldr` at 0xcd1e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd1ec | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcd1f4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xcd1f8 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0xcd1fc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xcd200 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xcd220 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xcd224 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd228 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd22c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcd230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd240 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcd244 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xcd248 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd24c | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd250 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd254 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xcd258 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd25c | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xcd284 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xcd28c | load | 4 | `abs 0xcd740` | `-` |
| `ldr` at 0xcd290 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd29c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd2a0 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcd2a4 | load | 4 | `abs 0xcd74c` | `-` |
| `ldr` at 0xcd2a8 | load | 4 | `abs 0xcd748` | `-` |
| `ldr` at 0xcd2ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcd2b0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd2bc | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xcd2c4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd2c8 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xcd2cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd2d0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcd2d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcd2e0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcd2e4 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xcd2e8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd2ec | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd2f0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd2f4 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xcd2f8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd2fc | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xcd324 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xcd32c | load | 4 | `abs 0xcd740` | `-` |
| `ldr` at 0xcd330 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd33c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd340 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcd344 | load | 4 | `abs 0xcd750` | `-` |
| `ldr` at 0xcd348 | load | 4 | `abs 0xcd748` | `-` |
| `ldr` at 0xcd34c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd350 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd35c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xcd360 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xcd368 | store | 4 | `sp -0x38` | `0x1000000` |
| `ldr` at 0xcd36c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcd370 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xcd390 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xcd394 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd398 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd39c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcd3a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd3b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcd3b4 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xcd3b8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd3bc | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd3c0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd3c4 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xcd3c8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd3cc | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xcd3f4 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xcd3fc | load | 4 | `abs 0xcd740` | `-` |
| `ldr` at 0xcd400 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd40c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd410 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcd414 | load | 4 | `abs 0xcd754` | `-` |
| `ldr` at 0xcd418 | load | 4 | `abs 0xcd748` | `-` |
| `ldr` at 0xcd41c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcd420 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd42c | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xcd434 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd438 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xcd43c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd440 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcd444 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcd450 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcd454 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xcd458 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd45c | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd460 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd464 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xcd468 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd46c | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xcd494 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xcd49c | load | 4 | `abs 0xcd740` | `-` |
| `ldr` at 0xcd4a0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd4ac | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd4b0 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcd4b4 | load | 4 | `abs 0xcd758` | `-` |
| `ldr` at 0xcd4b8 | load | 4 | `abs 0xcd748` | `-` |
| `ldr` at 0xcd4bc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd4c0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcd4c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd4d8 | store | 4 | `sp -0x50` | `0x1` |
| `str` at 0xcd4e4 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xcd4ec | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd4f0 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0xcd4f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd4f8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcd4fc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xcd508 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcd50c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xcd510 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd514 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd518 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd51c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xcd520 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd524 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xcd54c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xcd554 | load | 4 | `abs 0xcd740` | `-` |
| `ldr` at 0xcd558 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd564 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd568 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcd56c | load | 4 | `abs 0xcd750` | `-` |
| `ldr` at 0xcd570 | load | 4 | `abs 0xcd748` | `-` |
| `ldr` at 0xcd574 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcd578 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd584 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xcd588 | load | 4 | `abs 0xcd75c` | `-` |
| `str` at 0xcd590 | store | 4 | `abs 0x27ecb8` | `0x0` |
| `ldr` at 0xcd594 | load | 4 | `abs 0xcd75c` | `-` |
| `ldr` at 0xcd598 | load | 4 | `abs 0x27ecb8` | `-` |
| `ldr` at 0xcd5a8 | load | 4 | `abs 0xcd75c` | `-` |
| `ldr` at 0xcd5ac | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcd5c8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcd5cc | load | 4 | `abs 0xcd75c` | `-` |
| `ldr` at 0xcd5d0 | load | 4 | `abs 0xcd75c` | `-` |
| `ldr` at 0xcd5d4 | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcd5dc | store | 4 | `abs 0x27ecb8` | `[0x27ecb8]+1` |
| `str` at 0xcd5ec | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0xcd5f0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd5f4 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd5f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcd5fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcd614 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcd620 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcd624 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0xcd628 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd62c | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0xcd630 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd634 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0xcd638 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd63c | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0xcd664 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xcd66c | load | 4 | `abs 0xcd740` | `-` |
| `ldr` at 0xcd670 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd67c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd680 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0xcd684 | load | 4 | `abs 0xcd760` | `-` |
| `ldr` at 0xcd688 | load | 4 | `abs 0xcd748` | `-` |
| `ldr` at 0xcd68c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcd690 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcd698 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd6a4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xcd6b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd6c4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xcd6d8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcd6dc | load | 4 | `abs 0xcd764` | `-` |
| `str` at 0xcd6e8 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xcd6f4 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xcd6f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd700 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xcd708 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xcd714 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcd718 | load | 4 | `abs 0xcd768` | `-` |
| `str` at 0xcd724 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xcd730 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0xcd734 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xcd738 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd738 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd738 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd150 | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xcd178 | `(unknown callee) 0xca57c` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xcd1ac | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xcd1f0 | `(unknown callee) 0xca8c0` | `0x10b354` | `0x10b21c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xcd214 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xcd23c | `(unknown callee) 0xca7e0` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xcd270 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcd2b4 | `(unknown callee) 0xca8c0` | `0x10b368` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcd2dc | `(unknown callee) 0xca57c` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xcd310 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcd354 | `(unknown callee) 0xca8c0` | `0x10b37c` | `0x10b21c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xcd384 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xcd3ac | `(unknown callee) 0xca7e0` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xcd3e0 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcd424 | `(unknown callee) 0xca8c0` | `0x10b390` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcd44c | `(unknown callee) 0xca57c` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xcd480 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcd4c4 | `(unknown callee) 0xca8c0` | `0x10b3a4` | `0x10b21c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xcd504 | `(unknown callee) 0xca57c` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0xcd538 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcd57c | `(unknown callee) 0xca8c0` | `0x10b37c` | `0x10b21c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xcd604 | `(unknown callee) 0xca7e0` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xcd650 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcd694 | `(unknown callee) 0xca8c0` | `0x10b3b8` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcd6a8 | `(unknown callee) 0xc162c` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xcd6c8 | `(unknown callee) 0xc162c` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xcd6e0 | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b3d4` | `?` | `ret(0xc162c)` |
| `bl` at 0xcd71c | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b40c` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd1b8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd1d4 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcd27c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd298 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcd31c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd338 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcd3ec | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd408 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcd48c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd4a8 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcd4d0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xcd544 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd560 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xcd5a0 | `[0x27ecb8] <= 0x1 (unsigned, 32-bit)` | `[0x27ecb8] > 0x1 (unsigned, 32-bit)` |
| `ble` at 0xcd610 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xcd61c | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0xcd65c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd678 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xcd6b4 | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `beq` at 0xcd6d4 | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `bne` at 0xcd710 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd738 | `r0` | `?` |

## `PIORead_PIC`

- `PIORead_PIC`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd76c to 0xcdecc.
  Established. The first call has target `0xbc308` and candidates `0x1`, `0x0`, `[sp+16]`, and
  unknown (`bl` at 0xcd868). The `0xca57c` call uses `[sp-84]`, `[sp-92]`, `[sp-88]`, and `sp-56` as
  candidates (`bl` at 0xcd8a8). The nonzero `ret(0xbc8b4)` branch is taken, while zero falls through
  (`bne` at 0xcd8e8). A separate unsigned comparison takes `[0x27eb98] <= 0x2` and falls through for
  greater values (`bls` at 0xcd904). The underlying device read operation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd77c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd780 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcd784 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcd788 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xcd78c | load | 4 | `abs 0xcde9c` | `-` |
| `ldr` at 0xcd790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd7b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcd7b4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xcd7b8 | load | 4 | `abs 0xcde9c` | `-` |
| `ldr` at 0xcd7bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd7dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xcd7e0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xcd7e4 | load | 4 | `abs 0xcde9c` | `-` |
| `ldr` at 0xcd7e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd808 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd810 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xcd818 | load | 1 | `(unknown)` | `-` |
| `str` at 0xcd81c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xcd820 | load | 4 | `abs 0xcde9c` | `-` |
| `ldr` at 0xcd824 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd84c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xcd854 | load | 2 | `(unknown)` | `-` |
| `str` at 0xcd858 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xcd864 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcd86c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xcd870 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xcd878 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0xcd880 | store | 4 | `[sp] +0x0` | `0x0` |
| `str` at 0xcd888 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xcd890 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd894 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xcd898 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcd89c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xcd8a0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xcd8ac | load | 4 | `sp -0x54` | `-` |
| `str` at 0xcd8b0 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xcd8b4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd8b8 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcd8bc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd8c0 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcd8c4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd8c8 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xcd8f0 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcd8f8 | load | 4 | `abs 0xcdea0` | `-` |
| `ldr` at 0xcd8fc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd908 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd90c | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcd910 | load | 4 | `abs 0xcdea4` | `-` |
| `ldr` at 0xcd914 | load | 4 | `abs 0xcdea8` | `-` |
| `ldr` at 0xcd918 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcd91c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd928 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xcd92c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcd930 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0xcd950 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xcd954 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd958 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcd95c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xcd960 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd970 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcd974 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xcd978 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd97c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcd980 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd984 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcd988 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcd98c | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xcd9b4 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcd9bc | load | 4 | `abs 0xcdea0` | `-` |
| `ldr` at 0xcd9c0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcd9cc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcd9d0 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcd9d4 | load | 4 | `abs 0xcdeac` | `-` |
| `ldr` at 0xcd9d8 | load | 4 | `abs 0xcdea8` | `-` |
| `ldr` at 0xcd9dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcd9e0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd9ec | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0xcd9f4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcd9f8 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xcd9fc | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcda00 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xcda04 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xcda10 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xcda14 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xcda18 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcda1c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcda20 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcda24 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcda28 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcda2c | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xcda54 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcda5c | load | 4 | `abs 0xcdea0` | `-` |
| `ldr` at 0xcda60 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcda6c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcda70 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcda74 | load | 4 | `abs 0xcdeb0` | `-` |
| `ldr` at 0xcda78 | load | 4 | `abs 0xcdea8` | `-` |
| `ldr` at 0xcda7c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcda80 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcda88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcda98 | store | 4 | `sp -0x60` | `0x1` |
| `str` at 0xcdaa4 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0xcdaac | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdab0 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xcdab4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcdab8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xcdabc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xcdac8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xcdacc | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xcdad0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdad4 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcdad8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcdadc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcdae0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcdae4 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xcdb0c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcdb14 | load | 4 | `abs 0xcdea0` | `-` |
| `ldr` at 0xcdb18 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcdb24 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcdb28 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcdb2c | load | 4 | `abs 0xcdea4` | `-` |
| `ldr` at 0xcdb30 | load | 4 | `abs 0xcdea8` | `-` |
| `ldr` at 0xcdb34 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcdb38 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdb44 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xcdb48 | load | 4 | `abs 0xcdeb4` | `-` |
| `str` at 0xcdb50 | store | 4 | `abs 0x27ecb8` | `0x0` |
| `ldr` at 0xcdb54 | load | 4 | `abs 0xcdeb4` | `-` |
| `ldr` at 0xcdb58 | load | 4 | `abs 0x27ecb8` | `-` |
| `ldr` at 0xcdb68 | load | 4 | `abs 0xcdeb4` | `-` |
| `ldr` at 0xcdb6c | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcdb88 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcdb8c | load | 4 | `abs 0xcdeb4` | `-` |
| `ldr` at 0xcdb90 | load | 4 | `abs 0xcdeb4` | `-` |
| `ldr` at 0xcdb94 | load | 4 | `abs 0x27ecb8` | `-` |
| `str` at 0xcdb9c | store | 4 | `abs 0x27ecb8` | `[0x27ecb8]+1` |
| `str` at 0xcdbac | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xcdbb0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdbb4 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcdbb8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xcdbbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdbc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcdbd4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcdbe0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcdbe4 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xcdbe8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdbec | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcdbf0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcdbf4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcdbf8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcdbfc | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xcdc24 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcdc2c | load | 4 | `abs 0xcdea0` | `-` |
| `ldr` at 0xcdc30 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcdc3c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcdc40 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcdc44 | load | 4 | `abs 0xcdeb8` | `-` |
| `ldr` at 0xcdc48 | load | 4 | `abs 0xcdea8` | `-` |
| `ldr` at 0xcdc4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcdc50 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xcdc58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdc64 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xcdc78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdc84 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xcdc98 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcdc9c | load | 4 | `abs 0xcdebc` | `-` |
| `str` at 0xcdca8 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xcdcb4 | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0xcdcbc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdcc0 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0xcdcc4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcdcc8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xcdccc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xcdcd8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xcdcdc | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0xcdce0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdce4 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcdce8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcdcec | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcdcf0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcdcf4 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xcdd1c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcdd24 | load | 4 | `abs 0xcdea0` | `-` |
| `ldr` at 0xcdd28 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcdd34 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcdd38 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcdd3c | load | 4 | `abs 0xcdec0` | `-` |
| `ldr` at 0xcdd40 | load | 4 | `abs 0xcdea8` | `-` |
| `ldr` at 0xcdd44 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xcdd48 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdd6c | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0xcdd70 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdd74 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcdd78 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xcdd7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcdd8c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcdd90 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0xcdd94 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcdd98 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0xcdd9c | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcdda0 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0xcdda4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xcdda8 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0xcddd0 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcddd8 | load | 4 | `abs 0xcdea0` | `-` |
| `ldr` at 0xcdddc | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xcdde8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xcddec | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0xcddf0 | load | 4 | `abs 0xcdec4` | `-` |
| `ldr` at 0xcddf4 | load | 4 | `abs 0xcdea8` | `-` |
| `ldr` at 0xcddf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcddfc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xcde0c | store | 4 | `sp -0x70` | `0x40` |
| `ldr` at 0xcde10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcde14 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xcde38 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xcde3c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xcde40 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0xcde44 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xcde48 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xcde4c | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0xcde54 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xcde58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcde60 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xcde68 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xcde74 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xcde78 | load | 4 | `abs 0xcdec8` | `-` |
| `str` at 0xcde84 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0xcde90 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0xcde94 | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xcde98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcde98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcde98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd868 | `(unknown callee) 0xbc308` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xcd8a8 | `(unknown callee) 0xca57c` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xcd8dc | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xcd920 | `(unknown callee) 0xca8c0` | `0x10b448` | `0x10b21c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xcd944 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xcd96c | `(unknown callee) 0xca7e0` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xcd9a0 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcd9e4 | `(unknown callee) 0xca8c0` | `0x10b45c` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcda0c | `(unknown callee) 0xca57c` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xcda40 | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xcda84 | `(unknown callee) 0xca8c0` | `0x10b470` | `0x10b21c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xcdac4 | `(unknown callee) 0xca57c` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xcdaf8 | `(unknown callee) 0xbc8b4` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcdb3c | `(unknown callee) 0xca8c0` | `0x10b448` | `0x10b21c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xcdbc4 | `(unknown callee) 0xca7e0` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xcdc10 | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcdc54 | `(unknown callee) 0xca8c0` | `0x10b484` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcdc68 | `(unknown callee) 0xc162c` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0xcdc88 | `(unknown callee) 0xc162c` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0xcdca0 | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b3d4` | `?` | `ret(0xc162c)` |
| `bl` at 0xcdcd4 | `(unknown callee) 0xca57c` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0xcdd08 | `(unknown callee) 0xbc8b4` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0xcdd4c | `(unknown callee) 0xca8c0` | `0x10b4a0` | `0x10b21c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0xcdd60 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0xcdd88 | `(unknown callee) 0xca7e0` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0xcddbc | `(unknown callee) 0xbc8b4` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0xcde00 | `(unknown callee) 0xca8c0` | `0x10b4b4` | `0x10b21c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0xcde20 | `(unknown callee) 0xc1ab4` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0xcde34 | `(unknown callee) 0xc1c9c` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0xcde7c | `(unknown callee) 0x18cd4` | `[sp+16]` | `0x10b4c8` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd8e8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd904 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcd9ac | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcd9c8 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcda4c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcda68 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcda90 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xcdb04 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcdb20 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xcdb60 | `[0x27ecb8] <= 0x1 (unsigned, 32-bit)` | `[0x27ecb8] > 0x1 (unsigned, 32-bit)` |
| `ble` at 0xcdbd0 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0xcdbdc | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0xcdc1c | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcdc38 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xcdc74 | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `beq` at 0xcdc94 | `ret(0xc162c) == 0x0` | `ret(0xc162c) != 0x0` |
| `bne` at 0xcdd14 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcdd30 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcddc8 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `bls` at 0xcdde4 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xcde70 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcde98 | `r0` | `?` |

## `SMPCmdLibrary`

- `SMPCmdLibrary`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81090 to 0x82334.
  Established. Calling `0x5e7a8` presents candidates `0x0`, entry `r1`, entry `r2`, and `0x0` (`bl`
  at 0x810d0). Word loads address `ret(0x5e7a8) +0x8` (`ldr` at 0x810e0) and `ret(0x5e7a8) +0xc`
  (`ldr` at 0x810f4). Another call targets `0x18920` with candidates `0x8`, unknown, unknown, and
  unknown (`bl` at 0x8111c). A word store writes `ret(0x18920)` at `[sp-28] +0x0` (`str` at
  0x81124). The command-library role is unknown beyond the recorded operations.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x810a0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x810a4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x810a8 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x810b0 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x810b8 | store | 4 | `sp -0xa68` | `0x0` |
| `str` at 0x810c0 | store | 4 | `sp -0xa6c` | `0x0` |
| `str` at 0x810c8 | store | 4 | `sp -0xa70` | `0x0` |
| `str` at 0x810d8 | store | 4 | `sp -0xa84` | `ret(0x5e7a8)` |
| `ldr` at 0x810dc | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x810e0 | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `ldr` at 0x810f0 | load | 4 | `sp -0xa84` | `-` |
| `ldr` at 0x810f4 | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x81110 | store | 4 | `sp -0xa88` | `?` |
| `ldr` at 0x81114 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x81124 | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x81128 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x8112c | store | 4 | `sp -0xa74` | `[[sp-28]]` |
| `ldr` at 0x81130 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81138 | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x81140 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x8114c | store | 4 | `[sp-2676] +0x4` | `0x0` |
| `ldr` at 0x8115c | load | 4 | `abs 0x81fac` | `-` |
| `ldr` at 0x81164 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x81168 | store | 4 | `sp -0xa64` | `[sp-32]` |
| `str` at 0x81170 | store | 4 | `sp -0xac4` | `0x0` |
| `str` at 0x81178 | store | 4 | `sp -0xac0` | `sp-2660` |
| `ldr` at 0x81180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x81184 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81194 | store | 4 | `sp -0x28` | `ret(0x9fdbc)` |
| `ldr` at 0x81198 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x811a4 | load | 4 | `sp -0xa5c` | `-` |
| `ldr` at 0x811b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x811b4 | store | 4 | `sp -0xa94` | `ret(0x9fdbc)` |
| `ldr` at 0x811bc | load | 4 | `abs 0x82000` | `-` |
| `str` at 0x811cc | store | 4 | `sp -0x24` | `ret(0x1fb00)` |
| `ldr` at 0x811d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x811dc | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x811e0 | load | 4 | `[sp-2676] +0x0` | `-` |
| `ldr` at 0x811e4 | load | 4 | `abs 0x81fb0` | `-` |
| `str` at 0x811f0 | store | 4 | `sp -0xa94` | `?` |
| `ldr` at 0x811f8 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x81204 | load | 4 | `sp -0xa5c` | `-` |
| `str` at 0x8121c | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x81220 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x81224 | store | 4 | `sp -0xa74` | `[[sp-28]]` |
| `str` at 0x8122c | store | 4 | `sp -0xa78` | `0x0` |
| `ldr` at 0x81230 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81238 | store | 4 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x8123c | load | 4 | `sp -0xa5c` | `-` |
| `str` at 0x81244 | store | 4 | `sp -0x44c` | `[sp-2652]-1` |
| `ldr` at 0x81248 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81258 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x8125c | load | 4 | `abs 0x81fb4` | `-` |
| `ldr` at 0x81270 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8127c | load | 4 | `abs 0x81fb8` | `-` |
| `str` at 0x81280 | store | 4 | `sp -0xa8c` | `0xffff003` |
| `ldr` at 0x81284 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81288 | load | 4 | `abs 0x81ff4` | `-` |
| `ldr` at 0x8129c | load | 4 | `(unknown)` | `-` |
| `str` at 0x812a0 | store | 4 | `sp -0xa90` | `?` |
| `ldr` at 0x812a4 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x812a8 | load | 4 | `abs 0x81fbc` | `-` |
| `ldr` at 0x812bc | load | 4 | `abs 0x81fc0` | `-` |
| `ldrh` at 0x812c0 | load | 2 | `abs 0x282046` | `-` |
| `str` at 0x812c4 | store | 4 | `(unknown)` | `[0x282046]:2` |
| `ldr` at 0x812c8 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x812cc | load | 4 | `abs 0x81fc4` | `-` |
| `str` at 0x812e4 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x81300 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x8131c | store | 4 | `sp -0xac4` | `0x1` |
| `ldr` at 0x81324 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x8133c | store | 4 | `sp -0xac4` | `0x2` |
| `ldr` at 0x81340 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x81344 | store | 4 | `sp -0xac0` | `[sp-72]` |
| `str` at 0x8134c | store | 4 | `sp -0xabc` | `sp-68` |
| `ldr` at 0x81350 | load | 4 | `abs 0x81fc0` | `-` |
| `str` at 0x81368 | store | 4 | `sp -0x2c` | `ret(0x9c10c)` |
| `ldr` at 0x8136c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8137c | load | 4 | `sp -0xa90` | `-` |
| `ldr` at 0x81390 | load | 4 | `abs 0x81fc8` | `-` |
| `ldr` at 0x81394 | load | 4 | `sp -0xa90` | `-` |
| `ldr` at 0x813a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x813d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x813d8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x813e0 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x813f4 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x813f8 | load | 4 | `abs 0x81fc0` | `-` |
| `ldr` at 0x813fc | load | 4 | `abs 0x282040` | `-` |
| `ldr` at 0x81408 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81410 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81418 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81420 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81428 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81430 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81434 | load | 4 | `abs 0x81fcc` | `-` |
| `ldr` at 0x8143c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81444 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x8144c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81454 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x8145c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8146c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81474 | load | 4 | `sp -0xa90` | `-` |
| `ldr` at 0x8148c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81494 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x8149c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x814a0 | load | 4 | `abs 0x81ff8` | `-` |
| `ldr` at 0x814a8 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x814b0 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x814b8 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x814c4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x814c8 | load | 4 | `sp -0xa68` | `-` |
| `str` at 0x814d0 | store | 4 | `sp -0xa68` | `[sp-2664]+1` |
| `ldr` at 0x814d4 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x814d8 | load | 4 | `abs 0x81fc4` | `-` |
| `str` at 0x814f0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x814f8 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x814fc | load | 4 | `abs 0x81ff4` | `-` |
| `ldr` at 0x81510 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81520 | load | 4 | `abs 0x81fc0` | `-` |
| `ldr` at 0x81524 | load | 4 | `abs 0x282040` | `-` |
| `str` at 0x81540 | store | 4 | `sp -0xa90` | `0x0` |
| `str` at 0x81550 | store | 4 | `sp -0xac4` | `0x0` |
| `str` at 0x81558 | store | 4 | `sp -0xac0` | `sp-2660` |
| `ldr` at 0x81560 | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x81570 | store | 4 | `sp -0x28` | `ret(0x9fdbc)` |
| `ldr` at 0x81574 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81580 | load | 4 | `sp -0xa70` | `-` |
| `str` at 0x81588 | store | 4 | `sp -0xa70` | `[sp-2672]+1` |
| `ldr` at 0x8158c | load | 4 | `sp -0xa90` | `-` |
| `ldr` at 0x81598 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x815a0 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x815b4 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x815b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x815c4 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x815c8 | load | 4 | `abs 0x81ff4` | `-` |
| `ldr` at 0x815dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x815e8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x815f0 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x815f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81600 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81608 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81610 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81614 | load | 4 | `abs 0x81fd0` | `-` |
| `ldr` at 0x8161c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81624 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x8162c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81630 | load | 4 | `sp -0xa90` | `-` |
| `ldr` at 0x81638 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x81640 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x81648 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81654 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8165c | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81660 | load | 4 | `abs 0x81ff4` | `-` |
| `ldr` at 0x81674 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81680 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81688 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81690 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x816a0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x816ac | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x816b8 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x816bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x816c0 | load | 4 | `abs 0x81fd4` | `-` |
| `ldr` at 0x816c8 | load | 4 | `sp -0xa90` | `-` |
| `ldr` at 0x816d8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x816e0 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x816e8 | store | 4 | `(unknown)` | `ret(0x18788)` |
| `ldr` at 0x816ec | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x816f4 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x816fc | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81708 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8170c | load | 4 | `sp -0xa90` | `-` |
| `ldr` at 0x81714 | load | 4 | `sp -0xa68` | `-` |
| `str` at 0x8171c | store | 4 | `sp -0xa68` | `[sp-2664]+1` |
| `ldr` at 0x81720 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81724 | load | 4 | `abs 0x81fc4` | `-` |
| `str` at 0x8173c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x81744 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81748 | load | 4 | `abs 0x81fc4` | `-` |
| `str` at 0x81760 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x8177c | load | 4 | `sp -0xa60` | `-` |
| `str` at 0x81798 | store | 4 | `sp -0xac4` | `0x1` |
| `ldr` at 0x817a0 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x817b0 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x817c8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x817cc | store | 4 | `sp -0xac4` | `[sp-72]` |
| `str` at 0x817d4 | store | 4 | `sp -0xac0` | `sp-68` |
| `ldr` at 0x817e0 | load | 4 | `sp -0x44c` | `-` |
| `str` at 0x817ec | store | 4 | `sp -0xabc` | `?` |
| `str` at 0x817f4 | store | 4 | `sp -0xab8` | `0x0` |
| `ldr` at 0x817f8 | load | 4 | `abs 0x81fc0` | `-` |
| `str` at 0x8180c | store | 4 | `sp -0x24` | `ret(0x9bc5c)` |
| `ldr` at 0x81810 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x81820 | load | 4 | `sp -0x658` | `-` |
| `ldr` at 0x8182c | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81830 | load | 4 | `abs 0x81fc4` | `-` |
| `str` at 0x81848 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x8184c | load | 4 | `sp -0xa68` | `-` |
| `str` at 0x81854 | store | 4 | `sp -0xa68` | `[sp-2664]+1` |
| `ldr` at 0x8185c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81864 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81878 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x8187c | load | 4 | `abs 0x81fc0` | `-` |
| `ldr` at 0x81880 | load | 4 | `abs 0x282040` | `-` |
| `ldr` at 0x8188c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81894 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x8189c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x818a4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x818ac | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x818b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x818b8 | load | 4 | `abs 0x81fd8` | `-` |
| `ldr` at 0x818c0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x818c8 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x818d0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x818d8 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x818e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x818f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x818f8 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x818fc | load | 4 | `abs 0x81ff4` | `-` |
| `ldr` at 0x81910 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81928 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81930 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81938 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8193c | load | 4 | `abs 0x81ff8` | `-` |
| `ldr` at 0x81944 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x8194c | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x81954 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81960 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x81964 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81968 | load | 4 | `abs 0x81fc4` | `-` |
| `str` at 0x81980 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x81984 | load | 4 | `sp -0xa68` | `-` |
| `str` at 0x8198c | store | 4 | `sp -0xa68` | `[sp-2664]+1` |
| `ldr` at 0x81990 | load | 4 | `sp -0xa70` | `-` |
| `str` at 0x81998 | store | 4 | `sp -0xa70` | `[sp-2672]+1` |
| `ldr` at 0x8199c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x819a8 | load | 4 | `abs 0x81fdc` | `-` |
| `ldr` at 0x819b4 | load | 4 | `sp -0x44c` | `-` |
| `str` at 0x819bc | store | 4 | `sp -0x44c` | `[sp-1100]-1` |
| `ldr` at 0x819c4 | load | 4 | `sp -0xa60` | `-` |
| `ldr` at 0x819cc | load | 4 | `sp -0xa68` | `-` |
| `ldr` at 0x819d0 | load | 4 | `sp -0xa5c` | `-` |
| `str` at 0x819e4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x819f0 | load | 4 | `abs 0x81fe0` | `-` |
| `ldr` at 0x819f4 | load | 4 | `abs 0x81fe4` | `-` |
| `str` at 0x81a00 | store | 4 | `sp -0x24` | `ret(0x9a93c)` |
| `ldr` at 0x81a04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x81a08 | store | 4 | `sp -0xa9c` | `ret(0x9a93c)` |
| `ldr` at 0x81a0c | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x81a18 | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x81a24 | load | 4 | `sp -0xa9c` | `-` |
| `ldr` at 0x81a34 | load | 4 | `sp -0xa9c` | `-` |
| `str` at 0x81a48 | store | 4 | `sp -0x44c` | `0x0` |
| `ldr` at 0x81a4c | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81a50 | load | 4 | `sp -0xa5c` | `-` |
| `ldr` at 0x81a60 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81a64 | load | 4 | `abs 0x81fc4` | `-` |
| `ldr` at 0x81a78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81a84 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81a8c | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81aa0 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x81aa4 | load | 4 | `abs 0x81fc0` | `-` |
| `ldr` at 0x81aa8 | load | 4 | `abs 0x282040` | `-` |
| `ldr` at 0x81ab4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81abc | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81ac4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81acc | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81ad4 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81adc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81ae0 | load | 4 | `abs 0x81fe8` | `-` |
| `ldr` at 0x81ae8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81af0 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81af8 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81b00 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81b08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81b18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81b20 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81b24 | load | 4 | `abs 0x81ff4` | `-` |
| `ldr` at 0x81b38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81b50 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81b58 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81b60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81b64 | load | 4 | `abs 0x81ff8` | `-` |
| `ldr` at 0x81b6c | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x81b74 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x81b7c | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81b88 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x81b8c | load | 4 | `sp -0x44c` | `-` |
| `str` at 0x81b94 | store | 4 | `sp -0x44c` | `?` |
| `ldr` at 0x81b9c | load | 4 | `sp -0xa5c` | `-` |
| `str` at 0x81ba0 | store | 4 | `sp -0xa68` | `[sp-2652]` |
| `ldr` at 0x81ba8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81bac | load | 1 | `[sp-44] +0x12` | `-` |
| `str` at 0x81bb0 | store | 4 | `sp -0xa7c` | `[[sp-44]+18]:1` |
| `ldr` at 0x81bb4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81bb8 | load | 1 | `[sp-44] +0x14` | `-` |
| `ldr` at 0x81bbc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81bc0 | load | 1 | `[sp-44] +0x21` | `-` |
| `str` at 0x81bcc | store | 4 | `sp -0xa80` | `?` |
| `str` at 0x81bd4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x81bdc | store | 4 | `sp -0x44c` | `0x0` |
| `ldr` at 0x81be0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x81bec | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81bf0 | load | 4 | `sp -0xa5c` | `-` |
| `ldr` at 0x81c00 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81c04 | load | 4 | `abs 0x81fbc` | `-` |
| `ldr` at 0x81c18 | load | 4 | `(unknown)` | `-` |
| `str` at 0x81c1c | store | 4 | `sp -0xa98` | `?` |
| `ldr` at 0x81c20 | load | 4 | `sp -0xa7c` | `-` |
| `ldr` at 0x81c2c | load | 4 | `sp -0xa80` | `-` |
| `ldr` at 0x81c34 | load | 4 | `sp -0xa98` | `-` |
| `ldr` at 0x81c44 | load | 4 | `sp -0xa80` | `-` |
| `ldr` at 0x81c50 | load | 4 | `sp -0xa98` | `-` |
| `ldr` at 0x81c60 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81c64 | load | 4 | `abs 0x81fc4` | `-` |
| `ldr` at 0x81c78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81c84 | load | 4 | `sp -0x44c` | `-` |
| `ldr` at 0x81c88 | load | 4 | `abs 0x81fc4` | `-` |
| `str` at 0x81ca0 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x81ca8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x81cac | load | 4 | `sp -0x44c` | `-` |
| `str` at 0x81cb4 | store | 4 | `sp -0x44c` | `?` |
| `ldr` at 0x81cbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x81cc8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x81cd4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x81cdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x81ce8 | load | 4 | `abs 0x81fec` | `-` |
| `ldr` at 0x81cf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x81cf8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81d04 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x81d10 | load | 4 | `sp -0x650` | `-` |
| `ldr` at 0x81d14 | load | 4 | `abs 0x81ffc` | `-` |
| `ldr` at 0x81d28 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x81d2c | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0x81d30 | load | 4 | `sp -0x650` | `-` |
| `str` at 0x81d38 | store | 4 | `sp -0x650` | `[sp-1616]+1` |
| `ldr` at 0x81d3c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81d40 | load | 1 | `[sp-44] +0x13` | `-` |
| `ldr` at 0x81d50 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81d54 | load | 1 | `[sp-44] +0x24` | `-` |
| `ldr` at 0x81d64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81d68 | load | 4 | `[sp-44] +0x28` | `-` |
| `str` at 0x81d6c | store | 4 | `sp -0xaa0` | `[[sp-44]+40]` |
| `ldr` at 0x81d74 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x81d80 | store | 4 | `sp -0xaa0` | `ret(0x9ca84)` |
| `ldr` at 0x81d84 | load | 4 | `sp -0xaa0` | `-` |
| `str` at 0x81d88 | store | 4 | `sp -0x28` | `[sp-2720]` |
| `ldr` at 0x81d94 | load | 4 | `sp -0xa70` | `-` |
| `str` at 0x81d9c | store | 4 | `sp -0xa70` | `[sp-2672]+1` |
| `ldr` at 0x81da0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81da4 | load | 1 | `[sp-44] +0x20` | `-` |
| `ldr` at 0x81db0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81db8 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81dcc | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x81dd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81ddc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81de0 | load | 4 | `[sp-44] +0x1c` | `-` |
| `ldr` at 0x81dec | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81df4 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81dfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81e04 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81e0c | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81e14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81e18 | load | 4 | `abs 0x81fd0` | `-` |
| `ldr` at 0x81e20 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81e24 | load | 1 | `[sp-44] +0x2c` | `-` |
| `ldr` at 0x81e34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81e38 | load | 4 | `[sp-44] +0x30` | `-` |
| `str` at 0x81e3c | store | 4 | `sp -0xaa4` | `[[sp-44]+48]` |
| `ldr` at 0x81e44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x81e50 | store | 4 | `sp -0xaa4` | `ret(0x9ca84)` |
| `ldr` at 0x81e54 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81e5c | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81e64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81e68 | load | 4 | `sp -0xaa4` | `-` |
| `ldr` at 0x81e70 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x81e78 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x81e80 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81e8c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x81e94 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81e98 | load | 1 | `[sp-44] +0x20` | `-` |
| `ldr` at 0x81ea4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81ea8 | load | 4 | `[sp-44] +0x1c` | `-` |
| `ldr` at 0x81eb4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81ebc | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x81ec4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81ed4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81ee0 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81eec | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x81ef0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81ef4 | load | 4 | `abs 0x81fd4` | `-` |
| `ldr` at 0x81efc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81f00 | load | 1 | `[sp-44] +0x2c` | `-` |
| `ldr` at 0x81f10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81f14 | load | 4 | `[sp-44] +0x30` | `-` |
| `str` at 0x81f18 | store | 4 | `sp -0xaa8` | `[[sp-44]+48]` |
| `ldr` at 0x81f20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x81f2c | store | 4 | `sp -0xaa8` | `ret(0x9ca84)` |
| `ldr` at 0x81f30 | load | 4 | `sp -0xaa8` | `-` |
| `ldr` at 0x81f40 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x81f48 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81f50 | store | 4 | `(unknown)` | `ret(0x18788)` |
| `ldr` at 0x81f54 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x81f5c | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x81f64 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x81f70 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x81f78 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81f7c | load | 1 | `[sp-44] +0x20` | `-` |
| `ldr` at 0x81f88 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x81f8c | load | 1 | `[sp-44] +0x2c` | `-` |
| `ldr` at 0x81f9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81fa0 | load | 4 | `[sp-44] +0x30` | `-` |
| `str` at 0x81fa4 | store | 4 | `sp -0xaac` | `[[sp-44]+48]` |
| `ldr` at 0x82010 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8201c | store | 4 | `sp -0xaac` | `ret(0x9ca84)` |
| `ldr` at 0x82020 | load | 4 | `sp -0xaac` | `-` |
| `ldr` at 0x82038 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x82044 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x8204c | store | 4 | `(unknown)` | `ret(0x18788)` |
| `str` at 0x82054 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x82058 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x8205c | load | 1 | `[sp-44] +0x34` | `-` |
| `ldr` at 0x8206c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x82070 | load | 4 | `[sp-44] +0x38` | `-` |
| `str` at 0x82074 | store | 4 | `sp -0xab0` | `[[sp-44]+56]` |
| `ldr` at 0x8207c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x82088 | store | 4 | `sp -0xab0` | `ret(0x9ca84)` |
| `ldr` at 0x8208c | load | 4 | `sp -0xab0` | `-` |
| `ldr` at 0x820a4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x820b0 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x820b8 | store | 4 | `(unknown)` | `ret(0x18788)` |
| `str` at 0x820c0 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x820c4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x820cc | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x820d8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x820e0 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x820e8 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x820fc | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x82100 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x82104 | load | 4 | `[sp-44] +0x1c` | `-` |
| `ldr` at 0x82110 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x82118 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x82120 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x82128 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x82130 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x82138 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8213c | load | 4 | `abs 0x81ff0` | `-` |
| `ldr` at 0x82144 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8214c | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x82154 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x8215c | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x82164 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x82174 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8217c | load | 4 | `sp -0x44c` | `-` |
| `str` at 0x82184 | store | 4 | `sp -0x44c` | `[sp-1100]-1` |
| `ldr` at 0x82188 | load | 4 | `abs 0x81ff4` | `-` |
| `ldr` at 0x8219c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x821b4 | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x821bc | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x821c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x821c8 | load | 4 | `abs 0x81ff8` | `-` |
| `ldr` at 0x821d0 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x821d8 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x821e0 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x821ec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x821f0 | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x821fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x82208 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x8220c | load | 4 | `sp -0xa68` | `-` |
| `str` at 0x82214 | store | 4 | `sp -0xa68` | `[sp-2664]+1` |
| `ldr` at 0x8221c | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x8222c | load | 4 | `sp -0xa78` | `-` |
| `ldr` at 0x82234 | load | 4 | `sp -0xa74` | `-` |
| `ldr` at 0x8223c | load | 4 | `sp -0x654` | `-` |
| `ldr` at 0x82250 | load | 4 | `sp -0xa78` | `-` |
| `str` at 0x82258 | store | 4 | `sp -0xa78` | `[sp-2680]+1` |
| `ldr` at 0x82260 | load | 4 | `sp -0xa74` | `-` |
| `str` at 0x8226c | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x82274 | store | 4 | `sp -0xa68` | `0x0` |
| `ldr` at 0x82278 | load | 4 | `sp -0xa68` | `-` |
| `ldr` at 0x8227c | load | 4 | `sp -0x650` | `-` |
| `ldr` at 0x8228c | load | 4 | `sp -0xa68` | `-` |
| `ldr` at 0x82290 | load | 4 | `abs 0x81ffc` | `-` |
| `ldr` at 0x822a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x822ac | load | 4 | `sp -0xa68` | `-` |
| `str` at 0x822b4 | store | 4 | `sp -0xa68` | `[sp-2664]+1` |
| `ldr` at 0x822bc | load | 4 | `abs 0x82000` | `-` |
| `str` at 0x822c8 | store | 4 | `sp -0x24` | `ret(0x1fe64)` |
| `ldr` at 0x822cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x822d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x822e4 | load | 4 | `abs 0x82004` | `-` |
| `str` at 0x822e8 | store | 4 | `sp -0xac4` | `0xf46c0` |
| `str` at 0x822ec | store | 4 | `sp -0xac0` | `ret(0x1e71c)` |
| `ldr` at 0x822f4 | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x822f8 | load | 4 | `abs 0x8200c` | `-` |
| `ldr` at 0x82304 | load | 4 | `sp -0xa70` | `-` |
| `str` at 0x82314 | store | 4 | `sp -0xab4` | `?` |
| `str` at 0x82320 | store | 4 | `sp -0xab4` | `0x0` |
| `ldr` at 0x82324 | load | 4 | `sp -0xab4` | `-` |
| `str` at 0x82328 | store | 4 | `sp -0xa94` | `[sp-2740]` |
| `ldr` at 0x8232c | load | 4 | `sp -0xa94` | `-` |
| `ldmdb` at 0x82330 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82330 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82330 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x810d0 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x8111c | `(unknown callee) 0x18920` | `0x8` | `?` | `?` | `?` |
| `bl` at 0x81160 | `(unknown callee) 0x18c20` | `sp-2660` | `0x0` | `0x618` | `sp-2660` |
| `bl` at 0x8118c | `(unknown callee) 0x9fdbc` | `0x1` | `[sp-24]` | `[sp-2676]` | `0x0` |
| `bl` at 0x811c4 | `(unknown callee) 0x1fb00` | `0x282174` | `0x0` | `?` | `[sp-2652]` |
| `bl` at 0x811e8 | `(unknown callee) 0x18cd4` | `[[sp-2676]]` | `0xf45b0` | `?` | `[sp-2676]` |
| `bl` at 0x811fc | `(unknown callee) 0x18be4` | `[sp-2676]` | `?` | `?` | `ret(0x1fb00)` |
| `bl` at 0x81214 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x812fc | `(unknown callee) 0x541d4` | `sp-72` | `sp-68` | `0x0` | `sp-72` |
| `bl` at 0x81304 | `(unknown callee) 0x18a1c` | `[sp-2656]` | `?` | `?` | `?` |
| `bl` at 0x8132c | `(unknown callee) 0x54450` | `sp-72` | `sp-68` | `[sp-2656]` | `ret(0x18a1c)` |
| `bl` at 0x81360 | `(unknown callee) 0x9c10c` | `0x282040` | `sp-2700` | `0x0` | `0x0` |
| `bl` at 0x81380 | `(unknown callee) 0x63278` | `[sp-2704]` | `?` | `?` | `ret(0x9c10c)` |
| `bl` at 0x813c0 | `(unknown callee) 0x20adc` | `?` | `sp-44` | `0x64` | `?` |
| `bl` at 0x813d4 | `(unknown callee) 0x9a998` | `[sp-44]` | `?` | `?` | `ret(0x20adc)` |
| `bl` at 0x813ec | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `[sp-2676]` |
| `bl` at 0x81400 | `(unknown callee) 0x5461c` | `[0x282040]` | `?` | `?` | `0x282040` |
| `bl` at 0x8141c | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x81438 | `(unknown callee) 0x18ab8` | `?` | `0xf45d8` | `?` | `?` |
| `bl` at 0x81460 | `(unknown callee) 0x18a1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x81478 | `(unknown callee) 0x5461c` | `[sp-2704]` | `?` | `ret(0x18a1c)` | `?` |
| `bl` at 0x81488 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `ret(0x5461c)` |
| `bl` at 0x814a4 | `(unknown callee) 0x18ab8` | `?` | `0xf1e44` | `?` | `?` |
| `bl` at 0x81568 | `(unknown callee) 0x9fdbc` | `0x1` | `[sp-2656]` | `sp-2704` | `0x0` |
| `bl` at 0x815ac | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `[sp-2676]` |
| `bl` at 0x815e0 | `(unknown callee) 0x5461c` | `?` | `sp-20` | `0xfffff5c0` | `?` |
| `bl` at 0x815fc | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x81618 | `(unknown callee) 0x18ab8` | `?` | `0xf4600` | `?` | `?` |
| `bl` at 0x81634 | `(unknown callee) 0x18ab8` | `?` | `[sp-2704]` | `?` | `?` |
| `bl` at 0x81678 | `(unknown callee) 0x5461c` | `?` | `sp-20` | `0xfffff5c0` | `?` |
| `bl` at 0x81694 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x816c4 | `(unknown callee) 0x18ab8` | `?` | `0xf4604` | `[sp-2680]+1` | `?` |
| `bl` at 0x816cc | `(unknown callee) 0x18788` | `[sp-2704]` | `?` | `?` | `?` |
| `bl` at 0x81710 | `(unknown callee) 0x18be4` | `[sp-2704]` | `?` | `?` | `0x0` |
| `bl` at 0x81778 | `(unknown callee) 0x541d4` | `sp-72` | `sp-68` | `0x0` | `sp-72` |
| `bl` at 0x81780 | `(unknown callee) 0x18a1c` | `[sp-2656]` | `?` | `?` | `?` |
| `bl` at 0x817a8 | `(unknown callee) 0x54450` | `sp-72` | `sp-68` | `[sp-2656]` | `ret(0x18a1c)` |
| `bl` at 0x81804 | `(unknown callee) 0x9bc5c` | `0x282040` | `?` | `sp-2696` | `0x0` |
| `bl` at 0x81870 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `[sp-2676]` |
| `bl` at 0x81884 | `(unknown callee) 0x5461c` | `[0x282040]` | `?` | `?` | `0x282040` |
| `bl` at 0x818a0 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x818bc | `(unknown callee) 0x18ab8` | `?` | `0xf4608` | `?` | `?` |
| `bl` at 0x818e4 | `(unknown callee) 0x18a1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x81914 | `(unknown callee) 0x5461c` | `?` | `sp-20` | `0xfffff5c0` | `?` |
| `bl` at 0x81924 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `ret(0x5461c)` |
| `bl` at 0x81940 | `(unknown callee) 0x18ab8` | `?` | `0xf1e44` | `?` | `?` |
| `bl` at 0x819a0 | `(unknown callee) 0x1e71c` | `[sp-36]` | `sp-20` | `?` | `[sp-2672]+1` |
| `bl` at 0x819b0 | `(unknown callee) 0x1dba4` | `0xf4630` | `ret(0x1e71c)` | `?` | `ret(0x1e71c)` |
| `bl` at 0x819c8 | `(unknown callee) 0x18be4` | `[sp-2656]` | `?` | `?` | `?` |
| `bl` at 0x819f8 | `(unknown callee) 0x9a93c` | `sp-44` | `0x2820dc` | `0x44c` | `sp-44` |
| `bl` at 0x81a98 | `(unknown callee) 0x18920` | `0x50` | `sp-20` | `?` | `[sp-2676]` |
| `bl` at 0x81aac | `(unknown callee) 0x5461c` | `[0x282040]` | `?` | `?` | `0x282040` |
| `bl` at 0x81ac8 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x81ae4 | `(unknown callee) 0x18ab8` | `?` | `0xf4654` | `?` | `?` |
| `bl` at 0x81b0c | `(unknown callee) 0x18a1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x81b3c | `(unknown callee) 0x5461c` | `?` | `sp-20` | `0xfffff5c0` | `?` |
| `bl` at 0x81b4c | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `ret(0x5461c)` |
| `bl` at 0x81b68 | `(unknown callee) 0x18ab8` | `?` | `0xf1e44` | `?` | `?` |
| `bl` at 0x81ccc | `(unknown callee) 0x9a998` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x81ce0 | `(unknown callee) 0x1e71c` | `[sp-36]` | `ret(0x9a93c)` | `ret(0x9a93c)` | `ret(0x9a93c)` |
| `bl` at 0x81cf4 | `(unknown callee) 0x1dba4` | `0xf4670` | `ret(0x1e71c)` | `[sp-36]` | `ret(0x1e71c)` |
| `bl` at 0x81d7c | `(unknown callee) 0x9ca84` | `[sp-44]` | `0x0` | `?` | `?` |
| `bl` at 0x81dc4 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `[sp-2676]` |
| `bl` at 0x81de4 | `(unknown callee) 0x5461c` | `[[sp-44]+28]` | `?` | `?` | `[sp-44]` |
| `bl` at 0x81e00 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x81e1c | `(unknown callee) 0x18ab8` | `?` | `0xf4600` | `?` | `?` |
| `bl` at 0x81e4c | `(unknown callee) 0x9ca84` | `[sp-44]` | `0x1` | `?` | `?` |
| `bl` at 0x81e6c | `(unknown callee) 0x18ab8` | `?` | `[sp-2724]` | `?` | `?` |
| `bl` at 0x81eac | `(unknown callee) 0x5461c` | `[[sp-44]+28]` | `?` | `?` | `[sp-44]` |
| `bl` at 0x81ec8 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x81ef8 | `(unknown callee) 0x18ab8` | `?` | `0xf4604` | `[sp-2680]+1` | `?` |
| `bl` at 0x81f28 | `(unknown callee) 0x9ca84` | `[sp-44]` | `0x1` | `?` | `?` |
| `bl` at 0x81f34 | `(unknown callee) 0x18788` | `[sp-2728]` | `?` | `?` | `?` |
| `bl` at 0x82018 | `(unknown callee) 0x9ca84` | `[sp-44]` | `0x1` | `?` | `?` |
| `bl` at 0x82024 | `(unknown callee) 0x18788` | `[sp-2732]` | `?` | `?` | `?` |
| `bl` at 0x82084 | `(unknown callee) 0x9ca84` | `[sp-44]` | `0x2` | `[sp-2680]+1` | `?` |
| `bl` at 0x82090 | `(unknown callee) 0x18788` | `[sp-2736]` | `?` | `?` | `?` |
| `bl` at 0x820f4 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `[sp-2676]` |
| `bl` at 0x82108 | `(unknown callee) 0x5461c` | `[[sp-44]+28]` | `?` | `?` | `[sp-44]` |
| `bl` at 0x82124 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `?` |
| `bl` at 0x82140 | `(unknown callee) 0x18ab8` | `?` | `0xf4698` | `?` | `?` |
| `bl` at 0x82168 | `(unknown callee) 0x18a1c` | `?` | `?` | `?` | `?` |
| `bl` at 0x821a0 | `(unknown callee) 0x5461c` | `?` | `sp-20` | `0xfffff5c0` | `?` |
| `bl` at 0x821b0 | `(unknown callee) 0x18cd4` | `?` | `ret(0x5461c)` | `?` | `ret(0x5461c)` |
| `bl` at 0x821cc | `(unknown callee) 0x18ab8` | `?` | `0xf1e44` | `?` | `?` |
| `bl` at 0x82200 | `(unknown callee) 0x9a998` | `[sp-44]` | `?` | `?` | `[sp-1620]` |
| `mov` at 0x8224c | `(unknown callee) (unknown target)` | `sp-2660` | `?` | `[sp-1620]` | `?` |
| `bl` at 0x822a8 | `(unknown callee) 0x9a998` | `?` | `sp-20` | `0xfffff9c8` | `?` |
| `bl` at 0x822c0 | `(unknown callee) 0x1fe64` | `0x282174` | `?` | `?` | `?` |
| `bl` at 0x822dc | `(unknown callee) 0x1e71c` | `ret(0x1fe64)` | `?` | `?` | `ret(0x1fe64)` |
| `bl` at 0x82300 | `(unknown callee) 0x1eed0` | `0x9` | `0xf44d4` | `0x19ec` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x811a0 | `ret(0x9fdbc) != 0x0` | `ret(0x9fdbc) == 0x0` |
| `bne` at 0x811ac | `[sp-2652] != 0x0` | `[sp-2652] == 0x0` |
| `beq` at 0x811d8 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `bge` at 0x81250 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x81278 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x81374 | `ret(0x9c10c) != 0x0` | `ret(0x9c10c) == 0x0` |
| `beq` at 0x8138c | `ret(0x63278) == 0x0` | `ret(0x63278) != 0x0` |
| `beq` at 0x813cc | `ret(0x20adc) == 0x0` | `ret(0x20adc) != 0x0` |
| `bne` at 0x81538 | `? != ?` | `? == ?` |
| `beq` at 0x8157c | `ret(0x9fdbc) == 0x0` | `ret(0x9fdbc) != 0x0` |
| `beq` at 0x81594 | `[sp-2704] == 0x0` | `[sp-2704] != 0x0` |
| `beq` at 0x815c0 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x81818 | `ret(0x9bc5c) == 0x0` | `ret(0x9bc5c) != 0x0` |
| `bne` at 0x81828 | `[sp-1624] != 0x1` | `[sp-1624] == 0x1` |
| `blt` at 0x819d8 | `[sp-2664] < [sp-2652] (signed, 32-bit)` | `[sp-2664] >= [sp-2652] (signed, 32-bit)` |
| `beq` at 0x81a14 | `ret(0x9a93c) == 0x0` | `ret(0x9a93c) != 0x0` |
| `bgt` at 0x81a20 | `ret(0x9a93c) > 0x0 (signed, 32-bit)` | `ret(0x9a93c) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x81a2c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x81a3c | `ret(0x9a93c) == 0x6e` | `ret(0x9a93c) != 0x6e` |
| `blt` at 0x81a58 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x81a80 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x81be8 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x81bf8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x81c28 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x81c3c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x81c58 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x81c80 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x81cc4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x81d00 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x81d0c | `[sp-1620] == 0x0` | `[sp-1620] != 0x0` |
| `beq` at 0x81d48 | `[[sp-44]+19]:1 == 0x0` | `[[sp-44]+19]:1 != 0x0` |
| `bne` at 0x81d60 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x81d90 | `[sp-2720] == 0x0` | `[sp-2720] != 0x0` |
| `bls` at 0x81dac | `[[sp-44]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-44]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x81dd8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x81e30 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x81ea0 | `[[sp-44]+32]:1 != 0x2` | `[[sp-44]+32]:1 == 0x2` |
| `bne` at 0x81f0c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x81f84 | `[[sp-44]+32]:1 != 0x3` | `[[sp-44]+32]:1 == 0x3` |
| `bne` at 0x81f98 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x82068 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x821f8 | `[sp-1620] != 0x0` | `[sp-1620] == 0x0` |
| `beq` at 0x82224 | `[sp-1620] == 0x0` | `[sp-1620] != 0x0` |
| `blt` at 0x82284 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x822d4 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |
| `beq` at 0x8230c | `[sp-2672] == 0x0` | `[sp-2672] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82330 | `r0` | `?` |

## `FRUEventGuess`

- `FRUEventGuess`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4f38 to 0xa5abc.
  Established. A byte load uses `[sp-28] +0xd` (`ldrb` at 0xa4f68). The following branch is taken
  when `[[sp-28]+13]:1 != 0x0` and falls through for zero (`bne` at 0xa4f70). The earlier call to
  `0x18cd4` has candidates `sp-284`, `0x107470`, entry `r2`, and `sp-284` (`bl` at 0xa4f60). No
  event identity or device role is established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4f48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4f4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa4f50 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xa4f5c | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa4f64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa4f68 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa4f7c | load | 4 | `abs 0xa5a64` | `-` |
| `ldr` at 0xa4f84 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa4f88 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa4fa0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa4fbc | load | 4 | `abs 0xa5a68` | `-` |
| `ldr` at 0xa4fcc | load | 4 | `abs 0xa5a6c` | `-` |
| `ldr` at 0xa4fd4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa4fd8 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa4ff0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa500c | load | 4 | `abs 0xa5a68` | `-` |
| `ldr` at 0xa501c | load | 4 | `abs 0xa5a70` | `-` |
| `ldr` at 0xa5024 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5028 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa5040 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa505c | load | 4 | `abs 0xa5a68` | `-` |
| `ldr` at 0xa506c | load | 4 | `abs 0xa5a74` | `-` |
| `ldr` at 0xa5074 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5078 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa5090 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa50ac | load | 4 | `abs 0xa5a68` | `-` |
| `ldr` at 0xa50bc | load | 4 | `abs 0xa5a78` | `-` |
| `ldr` at 0xa50c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa50c8 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa50e0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa50fc | load | 4 | `abs 0xa5a68` | `-` |
| `ldr` at 0xa510c | load | 4 | `abs 0xa5a7c` | `-` |
| `ldr` at 0xa5114 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5118 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa5130 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa514c | load | 4 | `abs 0xa5a68` | `-` |
| `ldr` at 0xa515c | load | 4 | `abs 0xa5a80` | `-` |
| `ldr` at 0xa516c | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa5174 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa5178 | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldr` at 0xa517c | load | 4 | `abs 0xa5a84` | `-` |
| `ldr` at 0xa5188 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa518c | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa519c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa51a0 | load | 2 | `[sp-28] +0x1a` | `-` |
| `str` at 0xa51ac | store | 4 | `sp -0x3a4` | `?` |
| `ldr` at 0xa51b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa51b4 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xa51c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa51c4 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xa51d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa51d4 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xa51e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa51e4 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xa51f8 | load | 4 | `abs 0xa5a88` | `-` |
| `ldr` at 0xa5208 | load | 4 | `abs 0xa5a8c` | `-` |
| `ldr` at 0xa520c | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xa5218 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa521c | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xa5228 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa522c | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xa5238 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa523c | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xa5248 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa524c | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xa5260 | load | 4 | `abs 0xa5a90` | `-` |
| `ldr` at 0xa5270 | load | 4 | `abs 0xa5a8c` | `-` |
| `ldr` at 0xa5274 | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xa5288 | load | 4 | `abs 0xa5a8c` | `-` |
| `ldr` at 0xa528c | load | 4 | `sp -0x3a4` | `-` |
| `ldr` at 0xa5294 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5298 | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa52b0 | load | 4 | `abs 0xa5a94` | `-` |
| `ldr` at 0xa52c0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa52d0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa52e0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa52f0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa52f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa52fc | load | 1 | `[sp-28] +0xd` | `-` |
| `ldr` at 0xa5314 | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5324 | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5334 | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5344 | load | 4 | `abs 0xa5a98` | `-` |
| `str` at 0xa5350 | store | 4 | `sp -0x3a8` | `0x8` |
| `ldr` at 0xa5354 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5358 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xa5364 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5368 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xa5374 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5378 | load | 1 | `[sp-28] +0x20` | `-` |
| `ldr` at 0xa5380 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5384 | load | 1 | `[sp-28] +0x14` | `-` |
| `str` at 0xa538c | store | 4 | `sp -0x3a8` | `?` |
| `str` at 0xa5394 | store | 4 | `sp -0x3ac` | `0x3` |
| `str` at 0xa539c | store | 4 | `sp -0x3b0` | `0x3` |
| `str` at 0xa53a4 | store | 4 | `sp -0x3b4` | `0x3` |
| `str` at 0xa53ac | store | 4 | `sp -0x3b8` | `0x3` |
| `ldr` at 0xa53b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa53b4 | load | 1 | `[sp-28] +0x15` | `-` |
| `ldr` at 0xa53c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa53c4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xa53d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa53d4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xa53e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa53e8 | load | 1 | `[sp-28] +0x15` | `-` |
| `str` at 0xa53ec | store | 4 | `sp -0x3ac` | `[[sp-28]+21]:1` |
| `ldr` at 0xa53f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa53f4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xa5400 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5404 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xa5414 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5418 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xa5424 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5428 | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xa5438 | store | 4 | `sp -0x3ac` | `ret(0xa437c)` |
| `ldr` at 0xa543c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5440 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xa544c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5450 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xa545c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5460 | load | 1 | `[sp-28] +0x22` | `-` |
| `str` at 0xa5470 | store | 4 | `sp -0x3b0` | `ret(0xa437c)` |
| `ldr` at 0xa5474 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5478 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xa5484 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5488 | load | 1 | `[sp-28] +0x23` | `-` |
| `str` at 0xa5498 | store | 4 | `sp -0x3b4` | `ret(0xa437c)` |
| `ldr` at 0xa549c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa54a0 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xa54ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa54b0 | load | 1 | `[sp-28] +0x24` | `-` |
| `str` at 0xa54c0 | store | 4 | `sp -0x3b8` | `ret(0xa437c)` |
| `ldr` at 0xa54c4 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa54d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa54d4 | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xa54e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa54e4 | load | 1 | `[sp-28] +0x21` | `-` |
| `ldr` at 0xa54f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa54f4 | load | 1 | `[sp-28] +0x22` | `-` |
| `ldr` at 0xa5500 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5504 | load | 1 | `[sp-28] +0x23` | `-` |
| `ldr` at 0xa5510 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5514 | load | 1 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xa5524 | load | 4 | `abs 0xa5a9c` | `-` |
| `ldr` at 0xa5528 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xa5530 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa5534 | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa5538 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa553c | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xa5540 | load | 1 | `[[sp-28]+20]:1 +0xfd9c4` | `-` |
| `str` at 0xa5544 | store | 4 | `sp -0x3c0` | `[[[sp-28]+20]:1+1038788]:1` |
| `ldr` at 0xa554c | load | 4 | `abs 0xa5aa4` | `-` |
| `ldr` at 0xa5550 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa5564 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5568 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xa556c | load | 4 | `abs 0xa5a9c` | `-` |
| `ldr` at 0xa5570 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xa5578 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa557c | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xa5580 | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa5584 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5588 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xa558c | load | 1 | `[[sp-28]+20]:1 +0xfd9c4` | `-` |
| `str` at 0xa5590 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1038788]:1` |
| `ldr` at 0xa5598 | load | 4 | `abs 0xa5aa8` | `-` |
| `ldr` at 0xa559c | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa55ac | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa55b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa55bc | load | 1 | `[sp-28] +0x25` | `-` |
| `ldr` at 0xa55c8 | load | 4 | `sp -0x3ac` | `-` |
| `ldr` at 0xa55d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa55dc | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xa55e0 | load | 4 | `abs 0xa5a9c` | `-` |
| `ldr` at 0xa55e4 | load | 4 | `sp -0x3ac` | `-` |
| `ldrb` at 0xa55ec | load | 1 | `(unknown)` | `-` |
| `str` at 0xa55f0 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xa55f4 | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa55f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa55fc | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xa5600 | load | 1 | `[[sp-28]+20]:1 +0xfd9c4` | `-` |
| `str` at 0xa5604 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1038788]:1` |
| `ldr` at 0xa560c | load | 4 | `abs 0xa5aa8` | `-` |
| `ldr` at 0xa5610 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa561c | load | 4 | `sp -0x3b0` | `-` |
| `ldr` at 0xa562c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5630 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xa5634 | load | 4 | `abs 0xa5a9c` | `-` |
| `ldr` at 0xa5638 | load | 4 | `sp -0x3b0` | `-` |
| `ldrb` at 0xa5640 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa5644 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xa5648 | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa564c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5650 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xa5654 | load | 1 | `[[sp-28]+20]:1 +0xfd9c4` | `-` |
| `str` at 0xa5658 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1038788]:1` |
| `ldr` at 0xa5660 | load | 4 | `abs 0xa5aa8` | `-` |
| `ldr` at 0xa5664 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa5670 | load | 4 | `sp -0x3b4` | `-` |
| `ldr` at 0xa5680 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5684 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xa5688 | load | 4 | `abs 0xa5a9c` | `-` |
| `ldr` at 0xa568c | load | 4 | `sp -0x3b4` | `-` |
| `ldrb` at 0xa5694 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa5698 | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xa569c | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa56a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa56a4 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xa56a8 | load | 1 | `[[sp-28]+20]:1 +0xfd9c4` | `-` |
| `str` at 0xa56ac | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1038788]:1` |
| `ldr` at 0xa56b4 | load | 4 | `abs 0xa5aa8` | `-` |
| `ldr` at 0xa56b8 | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa56c4 | load | 4 | `sp -0x3b8` | `-` |
| `ldr` at 0xa56d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa56d8 | load | 1 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xa56dc | load | 4 | `abs 0xa5a9c` | `-` |
| `ldr` at 0xa56e0 | load | 4 | `sp -0x3b8` | `-` |
| `ldrb` at 0xa56e8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa56ec | store | 4 | `sp -0x3c0` | `?` |
| `ldr` at 0xa56f0 | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa56f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa56f8 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0xa56fc | load | 1 | `[[sp-28]+20]:1 +0xfd9c4` | `-` |
| `str` at 0xa5700 | store | 4 | `sp -0x3bc` | `[[[sp-28]+20]:1+1038788]:1` |
| `ldr` at 0xa5708 | load | 4 | `abs 0xa5aa8` | `-` |
| `ldr` at 0xa570c | load | 4 | `sp -0x3a8` | `-` |
| `ldr` at 0xa5718 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa571c | load | 2 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0xa5720 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa5724 | load | 2 | `[sp-28] +0x1e` | `-` |
| `str` at 0xa572c | store | 4 | `sp -0x3a0` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` |
| `ldr` at 0xa5734 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa5738 | load | 2 | `[sp-28] +0x18` | `-` |
| `str` at 0xa5740 | store | 4 | `sp -0x3c0` | `sp-284` |
| `ldr` at 0xa5748 | load | 4 | `abs 0xa5aac` | `-` |
| `ldr` at 0xa574c | load | 4 | `sp -0x3a0` | `-` |
| `ldr` at 0xa5770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5788 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xa578c | load | 2 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xa5790 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa5794 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xa5798 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa579c | load | 1 | `[sp-28] +0x9` | `-` |
| `str` at 0xa57a0 | store | 4 | `sp -0x3c0` | `[[sp-28]+9]:1` |
| `ldr` at 0xa57a8 | load | 4 | `abs 0xa5ab0` | `-` |
| `ldr` at 0xa57d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa57d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa57ec | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa580c | load | 4 | `abs 0xa5ab4` | `-` |
| `ldr` at 0xa582c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5848 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa5864 | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5884 | load | 4 | `abs 0xa5ab4` | `-` |
| `ldr` at 0xa58a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa58a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa58c0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa58dc | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa58fc | load | 4 | `abs 0xa5ab4` | `-` |
| `ldr` at 0xa591c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5938 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa5954 | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5974 | load | 4 | `abs 0xa5ab4` | `-` |
| `ldr` at 0xa5994 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5998 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa59b0 | load | 4 | `abs 0xa5a60` | `-` |
| `ldr` at 0xa59cc | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa59ec | load | 4 | `abs 0xa5ab4` | `-` |
| `ldr` at 0xa5a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5a10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5a28 | load | 4 | `abs 0xa5ab8` | `-` |
| `ldr` at 0xa5a48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5a4c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xa5a5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa5a5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa5a5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4f60 | `(unknown callee) 0x18cd4` | `sp-284` | `0x107470` | `r2` | `sp-284` |
| `bl` at 0xa4f80 | `(unknown callee) 0x18ab8` | `sp-284` | `0x107474` | `?` | `sp-284` |
| `bl` at 0xa4fa4 | `(unknown callee) 0x187e8` | `sp-284` | `0x107470` | `?` | `sp-284` |
| `bl` at 0xa4fc0 | `(unknown callee) 0x18ab8` | `sp-284` | `0x10747c` | `?` | `sp-284` |
| `bl` at 0xa4fd0 | `(unknown callee) 0x18ab8` | `sp-284` | `0x107480` | `?` | `sp-284` |
| `bl` at 0xa4ff4 | `(unknown callee) 0x187e8` | `sp-284` | `0x107470` | `?` | `sp-284` |
| `bl` at 0xa5010 | `(unknown callee) 0x18ab8` | `sp-284` | `0x10747c` | `?` | `sp-284` |
| `bl` at 0xa5020 | `(unknown callee) 0x18ab8` | `sp-284` | `0x107494` | `?` | `sp-284` |
| `bl` at 0xa5044 | `(unknown callee) 0x187e8` | `sp-284` | `0x107470` | `?` | `sp-284` |
| `bl` at 0xa5060 | `(unknown callee) 0x18ab8` | `sp-284` | `0x10747c` | `?` | `sp-284` |
| `bl` at 0xa5070 | `(unknown callee) 0x18ab8` | `sp-284` | `0x1074a0` | `?` | `sp-284` |
| `bl` at 0xa5094 | `(unknown callee) 0x187e8` | `sp-284` | `0x107470` | `?` | `sp-284` |
| `bl` at 0xa50b0 | `(unknown callee) 0x18ab8` | `sp-284` | `0x10747c` | `?` | `sp-284` |
| `bl` at 0xa50c0 | `(unknown callee) 0x18ab8` | `sp-284` | `0x1074b4` | `?` | `sp-284` |
| `bl` at 0xa50e4 | `(unknown callee) 0x187e8` | `sp-284` | `0x107470` | `?` | `sp-284` |
| `bl` at 0xa5100 | `(unknown callee) 0x18ab8` | `sp-284` | `0x10747c` | `?` | `sp-284` |
| `bl` at 0xa5110 | `(unknown callee) 0x18ab8` | `sp-284` | `0x1074bc` | `?` | `sp-284` |
| `bl` at 0xa5134 | `(unknown callee) 0x187e8` | `sp-284` | `0x107470` | `?` | `sp-284` |
| `bl` at 0xa5150 | `(unknown callee) 0x18ab8` | `sp-284` | `0x10747c` | `?` | `sp-284` |
| `bl` at 0xa5160 | `(unknown callee) 0x18ab8` | `sp-284` | `0x1074e0` | `?` | `sp-284` |
| `bl` at 0xa5170 | `(unknown callee) 0x18cd4` | `sp-156` | `0x107470` | `?` | `sp-156` |
| `bl` at 0xa51fc | `(unknown callee) 0x18c74` | `sp-284` | `0x1074ec` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xa5210 | `(unknown callee) 0x18c74` | `sp-156` | `0x10751c` | `[sp-932]` | `sp-156` |
| `bl` at 0xa5264 | `(unknown callee) 0x18c74` | `sp-284` | `0x10754c` | `[[sp-28]+26]:2` | `sp-284` |
| `bl` at 0xa5278 | `(unknown callee) 0x18c74` | `sp-156` | `0x10751c` | `[sp-932]` | `sp-156` |
| `bl` at 0xa5290 | `(unknown callee) 0x18c74` | `sp-156` | `0x10751c` | `[sp-932]` | `sp-156` |
| `bl` at 0xa52b4 | `(unknown callee) 0x18c74` | `sp-284` | `0x107574` | `?` | `sp-284` |
| `bl` at 0xa52c4 | `(unknown callee) 0x18cd4` | `sp-412` | `0x107470` | `?` | `sp-412` |
| `bl` at 0xa52d4 | `(unknown callee) 0x18cd4` | `sp-540` | `0x107470` | `?` | `sp-540` |
| `bl` at 0xa52e4 | `(unknown callee) 0x18cd4` | `sp-668` | `0x107470` | `?` | `sp-668` |
| `bl` at 0xa52f4 | `(unknown callee) 0x18cd4` | `sp-796` | `0x107470` | `?` | `sp-796` |
| `bl` at 0xa5318 | `(unknown callee) 0x18ab8` | `sp-412` | `0x1075ac` | `?` | `sp-412` |
| `bl` at 0xa5328 | `(unknown callee) 0x18ab8` | `sp-540` | `0x1075ac` | `?` | `sp-540` |
| `bl` at 0xa5338 | `(unknown callee) 0x18ab8` | `sp-668` | `0x1075ac` | `?` | `sp-668` |
| `bl` at 0xa5348 | `(unknown callee) 0x18ab8` | `sp-796` | `0x1075ac` | `?` | `sp-796` |
| `bl` at 0xa5430 | `(unknown callee) 0xa437c` | `[[sp-28]+33]:1` | `?` | `?` | `[[sp-28]+33]:1` |
| `bl` at 0xa5468 | `(unknown callee) 0xa437c` | `[[sp-28]+34]:1` | `?` | `?` | `[[sp-28]+34]:1` |
| `bl` at 0xa5490 | `(unknown callee) 0xa437c` | `[[sp-28]+35]:1` | `?` | `?` | `[[sp-28]+35]:1` |
| `bl` at 0xa54b8 | `(unknown callee) 0xa437c` | `[[sp-28]+36]:1` | `?` | `?` | `[[sp-28]+36]:1` |
| `bl` at 0xa5558 | `(unknown callee) 0x18c74` | `sp-412` | `0x1075b0` | `?` | `?` |
| `bl` at 0xa55a4 | `(unknown callee) 0x18c74` | `sp-412` | `0x1075e0` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xa5618 | `(unknown callee) 0x18c74` | `sp-412` | `0x1075e0` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xa566c | `(unknown callee) 0x18c74` | `sp-540` | `0x1075e0` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xa56c0 | `(unknown callee) 0x18c74` | `sp-668` | `0x1075e0` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xa5714 | `(unknown callee) 0x18c74` | `sp-796` | `0x1075e0` | `?` | `[[sp-28]+12]:1` |
| `bl` at 0xa5754 | `(unknown callee) 0x18c74` | `sp-924` | `0x107610` | `([[sp-28]+28]:2 + [[sp-28]+30]:2)` | `[[sp-28]+24]:2` |
| `bl` at 0xa5760 | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa5780 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |
| `bl` at 0xa57b4 | `(unknown callee) 0x18c74` | `sp-924` | `0x107628` | `[[sp-28]+24]:2` | `[[sp-28]+8]:1` |
| `bl` at 0xa57c0 | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa57e0 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |
| `bl` at 0xa57f0 | `(unknown callee) 0x187e8` | `sp-156` | `0x107470` | `?` | `sp-156` |
| `bl` at 0xa5810 | `(unknown callee) 0x18c74` | `sp-924` | `0x107658` | `sp-156` | `sp-924` |
| `bl` at 0xa581c | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa583c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |
| `bl` at 0xa584c | `(unknown callee) 0x187e8` | `sp-412` | `0x107470` | `?` | `sp-412` |
| `bl` at 0xa5868 | `(unknown callee) 0x187e8` | `sp-412` | `0x1075ac` | `?` | `sp-412` |
| `bl` at 0xa5888 | `(unknown callee) 0x18c74` | `sp-924` | `0x107658` | `sp-412` | `sp-924` |
| `bl` at 0xa5894 | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa58b4 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |
| `bl` at 0xa58c4 | `(unknown callee) 0x187e8` | `sp-540` | `0x107470` | `?` | `sp-540` |
| `bl` at 0xa58e0 | `(unknown callee) 0x187e8` | `sp-540` | `0x1075ac` | `?` | `sp-540` |
| `bl` at 0xa5900 | `(unknown callee) 0x18c74` | `sp-924` | `0x107658` | `sp-540` | `sp-924` |
| `bl` at 0xa590c | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa592c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |
| `bl` at 0xa593c | `(unknown callee) 0x187e8` | `sp-668` | `0x107470` | `?` | `sp-668` |
| `bl` at 0xa5958 | `(unknown callee) 0x187e8` | `sp-668` | `0x1075ac` | `?` | `sp-668` |
| `bl` at 0xa5978 | `(unknown callee) 0x18c74` | `sp-924` | `0x107658` | `sp-668` | `sp-924` |
| `bl` at 0xa5984 | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa59a4 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |
| `bl` at 0xa59b4 | `(unknown callee) 0x187e8` | `sp-796` | `0x107470` | `?` | `sp-796` |
| `bl` at 0xa59d0 | `(unknown callee) 0x187e8` | `sp-796` | `0x1075ac` | `?` | `sp-796` |
| `bl` at 0xa59f0 | `(unknown callee) 0x18c74` | `sp-924` | `0x107658` | `sp-796` | `sp-924` |
| `bl` at 0xa59fc | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa5a1c | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |
| `bl` at 0xa5a2c | `(unknown callee) 0x18c74` | `sp-924` | `0x107660` | `?` | `sp-924` |
| `bl` at 0xa5a38 | `(unknown callee) 0x18a1c` | `sp-924` | `?` | `?` | `sp-924` |
| `mov` at 0xa5a58 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-924` | `ret(0x18a1c)` | `sp-924` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa4f70 | `[[sp-28]+13]:1 != 0x0` | `[[sp-28]+13]:1 == 0x0` |
| `beq` at 0xa4f94 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa4fb0 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa4fe4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa5000 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5034 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa5050 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5084 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa50a0 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa50d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa50f0 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5124 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa5140 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5184 | `[[sp-28]+26]:2 == 0xffff` | `[[sp-28]+26]:2 != 0xffff` |
| `beq` at 0xa5198 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa51bc | `[[sp-28]+33]:1 != 0x77` | `[[sp-28]+33]:1 == 0x77` |
| `bne` at 0xa51cc | `[[sp-28]+34]:1 != 0x77` | `[[sp-28]+34]:1 == 0x77` |
| `bne` at 0xa51dc | `[[sp-28]+35]:1 != 0x77` | `[[sp-28]+35]:1 == 0x77` |
| `bne` at 0xa51ec | `[[sp-28]+36]:1 != 0x77` | `[[sp-28]+36]:1 == 0x77` |
| `bne` at 0xa5224 | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xa5234 | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xa5244 | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xa5254 | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `beq` at 0xa52a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa5308 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xa5360 | `[[sp-28]+32]:1 > 0x6 (unsigned, 32-bit)` | `[[sp-28]+32]:1 <= 0x6 (unsigned, 32-bit)` |
| `bhi` at 0xa5370 | `[[sp-28]+20]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+20]:1 <= 0x2 (unsigned, 32-bit)` |
| `bhi` at 0xa53bc | `[[sp-28]+21]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-28]+21]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xa53cc | `[[sp-28]+37]:1 == 0x6` | `[[sp-28]+37]:1 != 0x6` |
| `beq` at 0xa53dc | `[[sp-28]+37]:1 == 0x8` | `[[sp-28]+37]:1 != 0x8` |
| `beq` at 0xa53fc | `[[sp-28]+37]:1 == 0xa` | `[[sp-28]+37]:1 != 0xa` |
| `beq` at 0xa540c | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bhi` at 0xa5420 | `[[sp-28]+33]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+33]:1 <= 0x7e (unsigned, 32-bit)` |
| `bne` at 0xa5448 | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bhi` at 0xa5458 | `[[sp-28]+34]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+34]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xa5480 | `[[sp-28]+35]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+35]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xa54a8 | `[[sp-28]+36]:1 > 0x7e (unsigned, 32-bit)` | `[[sp-28]+36]:1 <= 0x7e (unsigned, 32-bit)` |
| `bhi` at 0xa54cc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xa54dc | `[[sp-28]+37]:1 == 0x16` | `[[sp-28]+37]:1 != 0x16` |
| `bne` at 0xa54ec | `[[sp-28]+33]:1 != 0x78` | `[[sp-28]+33]:1 == 0x78` |
| `bne` at 0xa54fc | `[[sp-28]+34]:1 != 0x78` | `[[sp-28]+34]:1 == 0x78` |
| `bne` at 0xa550c | `[[sp-28]+35]:1 != 0x78` | `[[sp-28]+35]:1 == 0x78` |
| `bne` at 0xa551c | `[[sp-28]+36]:1 != 0x78` | `[[sp-28]+36]:1 == 0x78` |
| `bhi` at 0xa55b4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xa55c4 | `[[sp-28]+37]:1 != 0x16` | `[[sp-28]+37]:1 == 0x16` |
| `bhi` at 0xa55d0 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xa5624 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xa5678 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xa56cc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xa57fc | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5858 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5874 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa58d0 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa58ec | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5948 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa5964 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa59c0 | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |
| `beq` at 0xa59dc | `ret(0x187e8) == 0x0` | `ret(0x187e8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa5a5c | `r0` | `ret(unknown)` |

## `FRUPackData`

- `FRUPackData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5aec to 0xa6814.
  Established. A word store places `0x1` at `[sp+32] +0x0` (`str` at 0xa5b14). Another writes
  `[[sp+32]]+1` to `[sp+32] +0x0` (`str` at 0xa5b28). The branch takes `[sp+16] == 0x0` and falls
  through for a nonzero value (`beq` at 0xa5ce8). The call to `0x18a1c` presents candidates
  `[sp+16]`, unknown, `[sp+32]`, and `[sp+16]` (`bl` at 0xa5cf0). The returned r0 is `ret(0x18ae8)`
  (`ldmdb` at 0xa6810). The external meaning of the packed data is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa5afc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa5b00 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa5b04 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xa5b08 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xa5b0c | load | 4 | `sp +0x20` | `-` |
| `str` at 0xa5b14 | store | 4 | `[sp+32] +0x0` | `0x1` |
| `ldr` at 0xa5b18 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b1c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b20 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5b28 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xa5b2c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b30 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b34 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5b3c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xa5b40 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b44 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b48 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5b50 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+1` |
| `ldr` at 0xa5b54 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b58 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b5c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5b64 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5b68 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b6c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b70 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5b78 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5b7c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b80 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b84 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5b8c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5b90 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b94 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5b98 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5ba0 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5ba4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5ba8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5bac | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5bb4 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5bb8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5bbc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5bc0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5bc8 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5bcc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5bd0 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5bd4 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5bdc | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `str` at 0xa5be4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xa5be8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5bec | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa5bfc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c00 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c04 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5c0c | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5c10 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c14 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c18 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5c20 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5c24 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c28 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c2c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5c34 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5c38 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c3c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5c54 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa5c5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa5c64 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5c6c | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xa5c70 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c74 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5c78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5c8c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa5c94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa5c9c | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5ca4 | store | 4 | `[sp+32] +0x0` | `?` |
| `ldr` at 0xa5ca8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa5cb0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xa5cb8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5cbc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5cc0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5cc8 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5ccc | load | 4 | `sp +0x20` | `-` |
| `str` at 0xa5cd0 | store | 4 | `sp -0x38` | `[sp+32]` |
| `ldr` at 0xa5cd4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5cd8 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5cdc | store | 4 | `sp -0x3c` | `[[sp+32]]` |
| `ldr` at 0xa5ce0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa5cec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa5cf8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa5d00 | store | 4 | `sp -0x40` | `([sp-60] + ret(0x18a1c))` |
| `ldr` at 0xa5d04 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa5d0c | store | 4 | `sp -0x40` | `([sp-60] + ret(0x18a1c))+1` |
| `ldr` at 0xa5d14 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa5d18 | store | 4 | `sp -0x40` | `[[sp+32]]` |
| `ldr` at 0xa5d1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa5d20 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa5d24 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xa5d28 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5d2c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5d30 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5d38 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5d3c | load | 4 | `sp +0x20` | `-` |
| `str` at 0xa5d40 | store | 4 | `sp -0x44` | `[sp+32]` |
| `ldr` at 0xa5d44 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5d48 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5d4c | store | 4 | `sp -0x48` | `[[sp+32]]` |
| `ldr` at 0xa5d50 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa5d5c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa5d68 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa5d70 | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x18a1c))` |
| `ldr` at 0xa5d74 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xa5d7c | store | 4 | `sp -0x4c` | `([sp-72] + ret(0x18a1c))+1` |
| `ldr` at 0xa5d84 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa5d88 | store | 4 | `sp -0x4c` | `[[sp+32]]` |
| `ldr` at 0xa5d8c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa5d90 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa5d94 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xa5d98 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5d9c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5da0 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5da8 | store | 4 | `[sp+32] +0x0` | `[[sp+32]]+4` |
| `ldr` at 0xa5dac | load | 4 | `sp +0x20` | `-` |
| `str` at 0xa5db0 | store | 4 | `sp -0x50` | `[sp+32]` |
| `ldr` at 0xa5db4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5db8 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5dbc | store | 4 | `sp -0x54` | `[[sp+32]]` |
| `ldr` at 0xa5dc0 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa5dcc | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa5dd8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa5de0 | store | 4 | `sp -0x58` | `([sp-84] + ret(0x18a1c))` |
| `ldr` at 0xa5de4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa5dec | store | 4 | `sp -0x58` | `([sp-84] + ret(0x18a1c))+1` |
| `ldr` at 0xa5df4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa5df8 | store | 4 | `sp -0x58` | `[[sp+32]]` |
| `ldr` at 0xa5dfc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa5e00 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa5e04 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xa5e08 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xa5e0c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa5e10 | load | 4 | `[sp+32] +0x0` | `-` |
| `str` at 0xa5e1c | store | 4 | `[sp+28] +0x0` | `ret(0x18920)` |
| `ldr` at 0xa5e20 | load | 4 | `[sp+28] +0x0` | `-` |
| `str` at 0xa5e24 | store | 4 | `sp -0x28` | `[[sp+28]]` |
| `ldr` at 0xa5e2c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xa5e38 | store | 1 | `[[sp+28]] +0x0` | `0x6` |
| `str` at 0xa5e40 | store | 4 | `sp -0x28` | `[[sp+28]]+1` |
| `ldr` at 0xa5e48 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xa5e54 | store | 1 | `[[sp+28]] +0x1` | `0x8` |
| `str` at 0xa5e5c | store | 4 | `sp -0x28` | `[[sp+28]]+2` |
| `ldr` at 0xa5e64 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xa5e70 | store | 1 | `[[sp+28]] +0x2` | `0x0` |
| `str` at 0xa5e78 | store | 4 | `sp -0x28` | `[[sp+28]]+3` |
| `ldr` at 0xa5e80 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xa5e8c | store | 1 | `[[sp+28]] +0x3` | `0x0` |
| `str` at 0xa5e94 | store | 4 | `sp -0x28` | `[[sp+28]]+4` |
| `ldr` at 0xa5e98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5e9c | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa5ea4 | store | 1 | `[[sp+28]] +0x4` | `?` |
| `ldr` at 0xa5eac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5eb4 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa5ebc | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa5ec4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5ecc | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa5ed4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa5edc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa5ee4 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xa5ee8 | store | 1 | `[sp-40] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xa5eec | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa5ef4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa5ef8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5efc | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xa5f04 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa5f0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5f14 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xa5f1c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa5f24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5f2c | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xa5f34 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa5f3c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa5f44 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xa5f48 | store | 1 | `[sp-40] +0x3` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xa5f4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa5f54 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa5f58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5f5c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa5f64 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa5f6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5f74 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa5f7c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa5f84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5f8c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa5f94 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa5f9c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa5fa4 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xa5fa8 | store | 1 | `[sp-40] +0x3` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xa5fac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa5fb4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa5fb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5fbc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xa5fc4 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa5fcc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5fd4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xa5fdc | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa5fe4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5fec | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xa5ff4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa5ffc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa6004 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xa6008 | store | 1 | `[sp-40] +0x3` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xa600c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6014 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa6018 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa601c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa6024 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa602c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6034 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa603c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa6044 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa604c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa6054 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa605c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa6064 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xa6068 | store | 1 | `[sp-40] +0x3` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xa606c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6074 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa6078 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa607c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa6084 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa608c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6094 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa609c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa60a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa60ac | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa60b4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa60bc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa60c4 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xa60c8 | store | 1 | `[sp-40] +0x3` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xa60cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa60d4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa60d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa60dc | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xa60e4 | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa60ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa60f4 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xa60fc | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa6104 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa610c | load | 4 | `sp +0x8` | `-` |
| `strb` at 0xa6114 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa611c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa6124 | load | 1 | `sp +0x8` | `-` |
| `strb` at 0xa6128 | store | 1 | `[sp-40] +0x3` | `([sp+8]:1 & 0xff)` |
| `ldr` at 0xa612c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6134 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xa613c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xa6140 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6144 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa6154 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6158 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa616c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6174 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa617c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa6184 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa618c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa61a0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa61a8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa61b0 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa61b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa61c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa61d4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa61dc | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa61e4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa61ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa61f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6208 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xa6210 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xa6214 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xa6218 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6220 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa6224 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6228 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa623c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6244 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa624c | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa6254 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa625c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6270 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6278 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa6280 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa6288 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6290 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa62a4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa62ac | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa62b4 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa62bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa62c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa62d8 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xa62e0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xa62e4 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xa62e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa62f0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa62f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa62f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa630c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6314 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa631c | store | 1 | `[sp-40] +0x4` | `?` |
| `ldr` at 0xa6324 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa632c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6340 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6348 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa6350 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa6358 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6360 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6374 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa637c | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa6384 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa638c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6394 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa63a8 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xa63b0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xa63b4 | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xa63b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa63c0 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `str` at 0xa63c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xa63cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa63e0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa63e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa63ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa63fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6400 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6414 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa641c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6430 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa6438 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa6440 | store | 1 | `[sp-40] +0x0` | `?` |
| `ldr` at 0xa6448 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6450 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6464 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa646c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6480 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa6488 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa6490 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa6498 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa64a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa64b4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa64bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa64d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa64d8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xa64e0 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa64e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa64f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6504 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa650c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6520 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xa6528 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xa652c | store | 1 | `[sp-40] +0x3` | `?` |
| `ldr` at 0xa6530 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6538 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa653c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6544 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xa654c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6560 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6568 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa657c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6584 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa658c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6590 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa659c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa65b0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa65b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa65c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa65c8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa65cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa65d4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xa65dc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa65e8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xa65f4 | store | 4 | `sp -0x5c` | `ret(0x18a1c)` |
| `ldr` at 0xa65f8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa6600 | store | 4 | `sp -0x5c` | `ret(0x18a1c)+1` |
| `str` at 0xa660c | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xa6610 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa6614 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa6618 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa661c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa6624 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa662c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6634 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa663c | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa6644 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa664c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa6654 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa665c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa6664 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa6668 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xa666c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6674 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa6678 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa667c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6680 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa6688 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa668c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa6694 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xa6698 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa66a4 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xa66b0 | store | 4 | `sp -0x60` | `ret(0x18a1c)` |
| `ldr` at 0xa66b4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa66bc | store | 4 | `sp -0x60` | `ret(0x18a1c)+1` |
| `str` at 0xa66c8 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xa66cc | load | 4 | `sp -0x60` | `-` |
| `str` at 0xa66d0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa66d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa66d8 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa66e0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa66e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa66f0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa66f8 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa6700 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6708 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa6710 | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa6718 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa6720 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa6724 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xa6728 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6730 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa6734 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6738 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa673c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa6744 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6748 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa6750 | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldr` at 0xa6754 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa6760 | load | 4 | `sp +0x18` | `-` |
| `str` at 0xa676c | store | 4 | `sp -0x64` | `ret(0x18a1c)` |
| `ldr` at 0xa6770 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6778 | store | 4 | `sp -0x64` | `ret(0x18a1c)+1` |
| `str` at 0xa6784 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xa6788 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa678c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa6790 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6794 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa679c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa67a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa67ac | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa67b4 | store | 1 | `[sp-40] +0x1` | `?` |
| `ldr` at 0xa67bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa67c4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa67cc | store | 1 | `[sp-40] +0x2` | `?` |
| `ldr` at 0xa67d4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xa67dc | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa67e0 | store | 1 | `[sp-40] +0x3` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xa67e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa67ec | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0xa67f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa67f4 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa67f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa6800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6804 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa680c | store | 4 | `sp -0x28` | `([sp-40] + [sp-52])` |
| `ldmdb` at 0xa6810 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa6810 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6810 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6810 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5cf0 | `(unknown callee) 0x18a1c` | `[sp+16]` | `?` | `[sp+32]` | `[sp+16]` |
| `bl` at 0xa5d60 | `(unknown callee) 0x18a1c` | `[sp+20]` | `?` | `[sp+32]` | `[sp+20]` |
| `bl` at 0xa5dd0 | `(unknown callee) 0x18a1c` | `[sp+24]` | `?` | `[sp+32]` | `[sp+24]` |
| `bl` at 0xa5e14 | `(unknown callee) 0x18920` | `[[sp+32]]` | `?` | `?` | `[sp+32]` |
| `bl` at 0xa6598 | `(unknown callee) 0x18ae8` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xa65ec | `(unknown callee) 0x18a1c` | `[sp+16]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xa6684 | `(unknown callee) 0x18ae8` | `[sp-40]+4` | `[sp+16]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xa66a8 | `(unknown callee) 0x18a1c` | `[sp+20]` | `?` | `[sp-40]` | `[sp+20]` |
| `bl` at 0xa6740 | `(unknown callee) 0x18ae8` | `[sp-40]+4` | `[sp+20]` | `[sp-52]` | `[sp-40]+4` |
| `bl` at 0xa6764 | `(unknown callee) 0x18a1c` | `[sp+24]` | `?` | `[sp-40]` | `[sp+24]` |
| `bl` at 0xa67fc | `(unknown callee) 0x18ae8` | `[sp-40]+4` | `[sp+24]` | `[sp-52]` | `[sp-40]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xa5bf4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa5ce8 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xa5d58 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xa5dc8 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `blt` at 0xa614c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xa63f4 | `[sp-44] < ? (signed, 32-bit)` | `[sp-44] >= ? (signed, 32-bit)` |
| `beq` at 0xa65e4 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0xa66a0 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0xa675c | `[sp+24] == 0x0` | `[sp+24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6810 | `r0` | `ret(0x18ae8)` |

## `FRUPackEvents`

- `FRUPackEvents`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6814 to 0xa6ce4.
  Established. A word store writes `0x1` at `[sp+16] +0x0` (`str` at 0xa6880). Calling `0x18920`
  uses candidates `[[sp+16]]`, `[sp+16]`, unknown, and `[sp+16]` (`bl` at 0xa6978). The later
  `0x18ae8` call has candidates unknown, unknown, `0xa8`, and unknown (`bl` at 0xa6cc0). The branch
  is taken when `Z == 0` and falls through when `not (Z == 0)` (`bne` at 0xa6cac); its compared
  operands and the event-packing role are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6824 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa6828 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa682c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xa6830 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xa6838 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xa683c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xa6840 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xa6844 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6854 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa685c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xa6860 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6864 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6868 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xa6874 | store | 4 | `sp -0x34` | `0xa8` |
| `ldr` at 0xa6878 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xa6880 | store | 4 | `[sp+16] +0x0` | `0x1` |
| `ldr` at 0xa6884 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6888 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa688c | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa6894 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xa6898 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa689c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68a0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa68a8 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xa68ac | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68b0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68b4 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa68bc | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+1` |
| `ldr` at 0xa68c0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68c4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68c8 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa68d0 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xa68d4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68d8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68dc | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa68e4 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xa68e8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68ec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa68f0 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa68f8 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xa68fc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6900 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6904 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa690c | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xa6910 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6914 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6918 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa6920 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xa6924 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6928 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa692c | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa6934 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xa6938 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa693c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6940 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa6948 | store | 4 | `[sp+16] +0x0` | `[[sp+16]]+4` |
| `ldr` at 0xa694c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6950 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6954 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa6958 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6960 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa6968 | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xa696c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6970 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa6974 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa6980 | store | 4 | `[sp+12] +0x0` | `ret(0x18920)` |
| `ldr` at 0xa6984 | load | 4 | `[sp+12] +0x0` | `-` |
| `str` at 0xa6988 | store | 4 | `sp -0x2c` | `[[sp+12]]` |
| `ldr` at 0xa6990 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xa699c | store | 1 | `[[sp+12]] +0x0` | `0x6` |
| `str` at 0xa69a4 | store | 4 | `sp -0x2c` | `[[sp+12]]+1` |
| `ldr` at 0xa69ac | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa69b0 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa69b8 | store | 1 | `[[sp+12]] +0x1` | `([sp-52]:1 & 0xff)` |
| `str` at 0xa69c0 | store | 4 | `sp -0x2c` | `[[sp+12]]+2` |
| `ldr` at 0xa69c8 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xa69d4 | store | 1 | `[[sp+12]] +0x2` | `0x0` |
| `str` at 0xa69dc | store | 4 | `sp -0x2c` | `[[sp+12]]+3` |
| `ldr` at 0xa69e4 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xa69f0 | store | 1 | `[[sp+12]] +0x3` | `0x0` |
| `str` at 0xa69f8 | store | 4 | `sp -0x2c` | `[[sp+12]]+4` |
| `ldr` at 0xa69fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6a00 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa6a08 | store | 1 | `[[sp+12]] +0x4` | `?` |
| `ldr` at 0xa6a10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6a18 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa6a20 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xa6a28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6a30 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xa6a38 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xa6a40 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6a48 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xa6a4c | store | 1 | `[sp-44] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xa6a50 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6a58 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xa6a5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6a60 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xa6a68 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xa6a70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6a78 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xa6a80 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xa6a88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6a90 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0xa6a98 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xa6aa0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6aa8 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xa6aac | store | 1 | `[sp-44] +0x3` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xa6ab0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6ab8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xa6abc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6ac0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa6ac8 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xa6ad0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6ad8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa6ae0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xa6ae8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6af0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xa6af8 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xa6b00 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6b08 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xa6b0c | store | 1 | `[sp-44] +0x3` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xa6b10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6b18 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xa6b1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6b20 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xa6b28 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xa6b30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6b38 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xa6b40 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xa6b48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6b50 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xa6b58 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xa6b60 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6b68 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xa6b6c | store | 1 | `[sp-44] +0x3` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xa6b70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6b78 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xa6b7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6b80 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa6b88 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xa6b90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6b98 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa6ba0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xa6ba8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6bb0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xa6bb8 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xa6bc0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6bc8 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xa6bcc | store | 1 | `[sp-44] +0x3` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xa6bd0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6bd8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xa6bdc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6be0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa6be8 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xa6bf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6bf8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa6c00 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xa6c08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6c10 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xa6c18 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xa6c20 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6c28 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xa6c2c | store | 1 | `[sp-44] +0x3` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xa6c30 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6c38 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xa6c3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6c40 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa6c48 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0xa6c50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6c58 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa6c60 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0xa6c68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6c70 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa6c78 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0xa6c80 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xa6c88 | load | 1 | `sp -0x30` | `-` |
| `strb` at 0xa6c8c | store | 1 | `[sp-44] +0x3` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0xa6c90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6c98 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0xa6c9c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xa6ca0 | store | 4 | `sp -0x28` | `[sp+8]` |
| `ldr` at 0xa6ca4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6cb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa6cb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6cc4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa6ccc | store | 4 | `sp -0x2c` | `[sp-44]+168` |
| `ldr` at 0xa6cd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa6cd4 | load | 4 | `[sp-40] +0xa8` | `-` |
| `str` at 0xa6cd8 | store | 4 | `sp -0x28` | `[[sp-40]+168]` |
| `ldmdb` at 0xa6ce0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa6ce0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6ce0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6ce0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6978 | `(unknown callee) 0x18920` | `[[sp+16]]` | `[sp+16]` | `?` | `[sp+16]` |
| `bl` at 0xa6cc0 | `(unknown callee) 0x18ae8` | `?` | `?` | `0xa8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa684c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa6cac | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6ce0 | `r0` | `?` |

## `FRUUnpackData`

- `FRUUnpackData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6ce4 to 0xa7794.
  Established. Word stores clear `[sp+16] +0x0` (`str` at 0xa6d0c), `[sp+20] +0x0` (`str` at
  0xa6d18), `[sp+24] +0x0` (`str` at 0xa6d24), and `[sp+28] +0x0` (`str` at 0xa6d30). The first
  branch takes `[[sp-28]] == 0x6` and falls through for inequality (`beq` at 0xa6d60). A later
  branch takes `[sp-48] == 0x8` and falls through for inequality (`beq` at 0xa708c). The call to
  `0x18920` has candidates unknown, unknown, `[[sp+16]]`, and unknown (`bl` at 0xa70c0). The
  physical interpretation of the decoded data remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6cf4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa6cf8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa6cfc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xa6d00 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xa6d04 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xa6d0c | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xa6d10 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xa6d18 | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xa6d1c | load | 4 | `sp +0x18` | `-` |
| `str` at 0xa6d24 | store | 4 | `[sp+24] +0x0` | `0x0` |
| `ldr` at 0xa6d28 | load | 4 | `sp +0x1c` | `-` |
| `str` at 0xa6d30 | store | 4 | `[sp+28] +0x0` | `0x0` |
| `ldr` at 0xa6d34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6d3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6d44 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xa6d48 | store | 4 | `[sp-28] +0x0` | `[[sp-24]]:1` |
| `str` at 0xa6d50 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xa6d54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6d58 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa6d64 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa6d6c | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xa6d78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6d80 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xa6d84 | store | 4 | `sp -0x30` | `[[sp-24]+1]:1` |
| `str` at 0xa6d8c | store | 4 | `sp -0x18` | `[sp-24]+2` |
| `ldr` at 0xa6d94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6d9c | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0xa6da0 | store | 4 | `sp -0x34` | `[[sp-24]+2]:1` |
| `str` at 0xa6da8 | store | 4 | `sp -0x18` | `[sp-24]+3` |
| `ldr` at 0xa6db0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6db8 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xa6dbc | store | 4 | `sp -0x38` | `[[sp-24]+3]:1` |
| `str` at 0xa6dc4 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa6dc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa6dcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6dd0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa6ddc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6de4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa6df4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6dfc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa6e0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6e14 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa6e1c | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xa6e20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa6e28 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa6e2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa6e30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6e34 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa6e40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6e48 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa6e58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6e60 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa6e70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6e78 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa6e80 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xa6e84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa6e8c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa6e90 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa6e94 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6e98 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa6ea4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6eac | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa6ebc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6ec4 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa6ed4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6edc | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa6ee4 | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xa6ee8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa6ef0 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa6ef4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa6ef8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6efc | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa6f08 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6f10 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa6f20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6f28 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa6f38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6f40 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa6f48 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xa6f4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa6f54 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa6f58 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa6f5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6f60 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa6f6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6f74 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa6f84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6f8c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa6f9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6fa4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa6fac | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xa6fb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa6fb8 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa6fbc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa6fc0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6fc4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa6fd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6fd8 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa6fe8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa6ff0 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa7000 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7008 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa7010 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xa7014 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa701c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa7020 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa7024 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7028 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa7034 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa703c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa704c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7054 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa7064 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa706c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa7074 | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xa7078 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7080 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa7084 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7090 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa7098 | store | 4 | `[sp+12] +0x0` | `0x6` |
| `ldr` at 0xa70a0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa70a4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa70a8 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa70c8 | store | 4 | `[sp+20] +0x0` | `ret(0x18920)` |
| `ldr` at 0xa70cc | load | 4 | `[sp+20] +0x0` | `-` |
| `str` at 0xa70d0 | store | 4 | `sp -0x28` | `[[sp+20]]` |
| `ldr` at 0xa70d4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa70d8 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xa70e4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa70ec | store | 4 | `[sp+12] +0x0` | `0x8` |
| `str` at 0xa70f8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xa70fc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa7100 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7104 | load | 4 | `[sp+16] +0x0` | `-` |
| `ldr` at 0xa7114 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7128 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7130 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7134 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xa7140 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7148 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xa7158 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7160 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xa7170 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7178 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xa7180 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xa7184 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa718c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa7190 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa71a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa71ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa71b0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa71bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa71c4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa71d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa71dc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa71ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa71f4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa71fc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xa7200 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7208 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa720c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7220 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7228 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa722c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xa7238 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7240 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0xa7250 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7258 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xa7268 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7270 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0xa7278 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xa727c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7284 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa7288 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa729c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa72a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa72b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa72c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa72d4 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0xa72d8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa72ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa72f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7300 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa7308 | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xa7310 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7324 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa732c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7340 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7348 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7350 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7364 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xa7368 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa737c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7384 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa7388 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7398 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa739c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xa73a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa73b0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xa73c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa73c8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xa73d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa73e0 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xa73e8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa73ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa73f4 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa73f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa740c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7414 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa741c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7424 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa7428 | store | 4 | `(unknown)` | `[sp-68]` |
| `ldr` at 0xa742c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa7434 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xa743c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7450 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7458 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa746c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7474 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7488 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0xa748c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa74a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa74a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa74b4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa74bc | store | 4 | `[sp+12] +0x0` | `0x8` |
| `ldr` at 0xa74c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa74d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa74e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa74f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa74fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7504 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7514 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7528 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7530 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7544 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa754c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7554 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7564 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7578 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7580 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa7588 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7590 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa7594 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa759c | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xa75a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa75a8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xa75b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa75bc | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xa75cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa75d4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xa75e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa75ec | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xa75f4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa75f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7600 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0xa7604 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa7610 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa7614 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa7620 | store | 4 | `[sp+24] +0x0` | `ret(0x18920)` |
| `ldr` at 0xa7624 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xa7628 | load | 4 | `[sp+24] +0x0` | `-` |
| `ldr` at 0xa762c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7630 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa7638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa763c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa7644 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xa7648 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa764c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7658 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7660 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7670 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7678 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7688 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7690 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa7698 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa769c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa76a4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa76a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa76b4 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xa76b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa76c4 | store | 4 | `[sp+28] +0x0` | `ret(0x18920)` |
| `ldr` at 0xa76c8 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xa76cc | load | 4 | `[sp+28] +0x0` | `-` |
| `ldr` at 0xa76d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa76d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa76dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa76e0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa76e8 | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldr` at 0xa76ec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa76f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa76fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7704 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa7714 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa771c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa772c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa7734 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa773c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa7740 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa7748 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa774c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa7758 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa775c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa7768 | store | 4 | `[sp+32] +0x0` | `ret(0x18920)` |
| `ldr` at 0xa776c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xa7770 | load | 4 | `[sp+32] +0x0` | `-` |
| `ldr` at 0xa7774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7778 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa7780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7784 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa778c | store | 4 | `sp -0x18` | `([sp-24] + [sp-64])` |
| `ldmdb` at 0xa7790 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa7790 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa7790 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa7790 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa70c0 | `(unknown callee) 0x18920` | `?` | `?` | `[[sp+16]]` | `?` |
| `bl` at 0xa72cc | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xa735c | `(unknown callee) 0x18c20` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xa7480 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0xa7510 | `(unknown callee) 0x18c20` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xa7560 | `(unknown callee) 0x18ae8` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0xa7618 | `(unknown callee) 0x18920` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xa7634 | `(unknown callee) 0x18ae8` | `[[sp+24]]` | `[sp-24]` | `[sp-64]` | `[sp+24]` |
| `bl` at 0xa76bc | `(unknown callee) 0x18920` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xa76d8 | `(unknown callee) 0x18ae8` | `[[sp+28]]` | `[sp-24]` | `[sp-64]` | `[sp+28]` |
| `bl` at 0xa7760 | `(unknown callee) 0x18920` | `[sp-64]` | `?` | `0x3` | `[sp-64]` |
| `bl` at 0xa777c | `(unknown callee) 0x18ae8` | `[[sp+32]]` | `[sp-24]` | `[sp-64]` | `[sp+32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa6d60 | `[[sp-28]] == 0x6` | `[[sp-28]] != 0x6` |
| `beq` at 0xa708c | `[sp-48] == 0x8` | `[sp-48] != 0x8` |
| `bne` at 0xa70e0 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |
| `blt` at 0xa710c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa72fc | `? != 0x0` | `? == 0x0` |
| `blt` at 0xa7390 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa74b0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa760c | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xa76b0 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xa7754 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa7790 | `r0` | `?` |

## `FRUUnpackEvents`

- `FRUUnpackEvents`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7794 to 0xa7be4.
  Established. Word stores zero `[sp+16] +0x0` (`str` at 0xa77bc) and `[sp+20] +0x0` (`str` at
  0xa77c8). A byte read at `[sp-20] +0x0` (`ldrb` at 0xa77dc) supplies `[[sp-20]]:1` for a word
  store at `[sp-24] +0x0` (`str` at 0xa77e0). The branch takes `[sp-56] == 0xa8` and falls through
  on inequality (`beq` at 0xa7b24). The `0x18920` call has candidates `0xac`, unknown, unknown, and
  `[[sp+16]]` (`bl` at 0xa7b5c). The event format and hardware consequences are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa77a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa77a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa77ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa77b0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa77b4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xa77bc | store | 4 | `[sp+16] +0x0` | `0x0` |
| `ldr` at 0xa77c0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xa77c8 | store | 4 | `[sp+20] +0x0` | `0x0` |
| `ldr` at 0xa77cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa77d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa77dc | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xa77e0 | store | 4 | `[sp-24] +0x0` | `[[sp-20]]:1` |
| `str` at 0xa77e8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xa77ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa77f0 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xa77fc | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa7804 | store | 4 | `[sp+12] +0x0` | `0x5` |
| `ldr` at 0xa7810 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7818 | load | 1 | `[sp-20] +0x1` | `-` |
| `str` at 0xa781c | store | 4 | `sp -0x38` | `[[sp-20]+1]:1` |
| `str` at 0xa7824 | store | 4 | `sp -0x14` | `[sp-20]+2` |
| `ldr` at 0xa782c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7834 | load | 1 | `[sp-20] +0x2` | `-` |
| `str` at 0xa7838 | store | 4 | `sp -0x30` | `[[sp-20]+2]:1` |
| `str` at 0xa7840 | store | 4 | `sp -0x14` | `[sp-20]+3` |
| `ldr` at 0xa7848 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7850 | load | 1 | `[sp-20] +0x3` | `-` |
| `str` at 0xa7854 | store | 4 | `sp -0x34` | `[[sp-20]+3]:1` |
| `str` at 0xa785c | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa7860 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7864 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7868 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa7874 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa787c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa788c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7894 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa78a4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa78ac | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xa78b4 | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xa78b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa78c0 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa78c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa78c8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa78cc | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa78d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa78e0 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa78f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa78f8 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa7908 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7910 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xa7918 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xa791c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7924 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa7928 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa792c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7930 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa793c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7944 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa7954 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa795c | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa796c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7974 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xa797c | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xa7980 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7988 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa798c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa7990 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7994 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa79a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa79a8 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa79b8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa79c0 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa79d0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa79d8 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xa79e0 | store | 4 | `[sp+4] +0x0` | `?` |
| `ldr` at 0xa79e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa79ec | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa79f0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa79f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa79f8 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa7a04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7a0c | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa7a1c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7a24 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa7a34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7a3c | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xa7a44 | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0xa7a48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7a50 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa7a54 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xa7a58 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7a5c | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa7a68 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7a70 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa7a80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7a88 | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa7a98 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7aa0 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xa7aa8 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0xa7aac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7ab4 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa7ab8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa7abc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7ac0 | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa7acc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7ad4 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xa7ae4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7aec | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xa7afc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa7b04 | load | 1 | `[sp-20] +0x7` | `-` |
| `str` at 0xa7b0c | store | 4 | `[sp+16] +0x0` | `?` |
| `ldr` at 0xa7b10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7b18 | store | 4 | `sp -0x14` | `[sp-20]+4` |
| `ldr` at 0xa7b1c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa7b28 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa7b30 | store | 4 | `[sp+12] +0x0` | `0x7` |
| `str` at 0xa7b3c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xa7b40 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xa7b44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa7b48 | load | 4 | `[sp+16] +0x0` | `-` |
| `str` at 0xa7b64 | store | 4 | `sp -0x24` | `ret(0x18920)` |
| `ldr` at 0xa7b68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7b6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa7b78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b7c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa7b84 | store | 4 | `sp -0x14` | `([sp-20] + [sp-56])` |
| `ldr` at 0xa7b88 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa7b90 | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xa7b94 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7b98 | load | 4 | `[sp+20] +0x0` | `-` |
| `ldr` at 0xa7ba4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xa7ba8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa7bac | store | 4 | `[sp+20] +0x0` | `[sp-36]` |
| `ldr` at 0xa7bb0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa7bb4 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xa7bbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa7bc0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa7bc4 | store | 4 | `[sp-40] +0xa8` | `[sp-36]` |
| `ldr` at 0xa7bc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa7bcc | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xa7bd0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa7bd8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0xa7be0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa7be0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa7be0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7b5c | `(unknown callee) 0x18920` | `0xac` | `?` | `?` | `[[sp+16]]` |
| `bl` at 0xa7b74 | `(unknown callee) 0x18ae8` | `ret(0x18920)` | `[sp-20]` | `[sp-56]` | `ret(0x18920)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa77f8 | `[[sp-24]] == 0x6` | `[[sp-24]] != 0x6` |
| `beq` at 0xa7b24 | `[sp-56] == 0xa8` | `[sp-56] != 0xa8` |
| `blt` at 0xa7b50 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa7ba0 | `[[sp+20]] != 0x0` | `[[sp+20]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa7be0 | `r0` | `?` |

## `PPPWrite`

- `PPPWrite`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ee84 to 0x9f398.
  Established. A word read from `r0 +0x10` (`ldr` at 0x9eeb4) is followed by a word store of
  `[r0+16]` at `r0 +0x18` (`str` at 0x9eeb8). A byte store places `0x7e` at `[[sp-20]+24] +0x0`
  (`strb` at 0x9eed0). The indirect call has an unknown target and candidates `[sp-28]`,
  `[[sp-20]+16]`, `[[sp-20]+28]`, and `[sp-20]` (`mov` at 0x9f0b0). The branch takes `[sp] != 0x0`
  and falls through for zero (`bne` at 0x9eef8). The transport hardware and callback identity remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ee94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9ee98 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9ee9c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9eea0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9eea4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x9eea8 | store | 4 | `sp -0x30` | `[sp]` |
| `ldr` at 0x9eeac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eeb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eeb4 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0x9eeb8 | store | 4 | `r0 +0x18` | `[r0+16]` |
| `ldr` at 0x9eebc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eec4 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0x9eed0 | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `str` at 0x9eed8 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0x9eedc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9eee4 | store | 4 | `[sp-20] +0x1c` | `0x1` |
| `strh` at 0x9eeec | store | 2 | `sp -0x26` | `?` |
| `ldr` at 0x9eef0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x9ef04 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x9ef08 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9ef0c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x9ef10 | store | 4 | `sp -0x3c` | `[[sp-32]]:1` |
| `ldr` at 0x9ef14 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9ef20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9ef2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9ef38 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9ef48 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x9ef5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ef60 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0x9ef70 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x9ef78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ef7c | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0x9ef8c | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0x9ef98 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x9ef9c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9efa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9efb0 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0x9efbc | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0x9efc4 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0x9efc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9efd0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9efd4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9efd8 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x9efe4 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0x9efec | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0x9eff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eff8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x9f000 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0x9f008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f010 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x9f014 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9f01c | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0x9f020 | store | 1 | `[[sp-20]+24] +0x0` | `([[sp-32]]:1 & 0xff)` |
| `str` at 0x9f028 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0x9f02c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f034 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x9f03c | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0x9f040 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9f044 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrh` at 0x9f048 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0x9f05c | store | 2 | `sp -0x26` | `(ret(0x9ec7c) & 0xffff)` |
| `ldr` at 0x9f060 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9f068 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x9f06c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x9f074 | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0x9f078 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f07c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9f084 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f088 | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0x9f094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f09c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f0a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f0a4 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0x9f0a8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x9f0b8 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x9f0bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f0c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f0c4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9f0d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f0d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f0dc | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x9f0e4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x9f0ec | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x9f0f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f0f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f0f8 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0x9f0fc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9f104 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9f108 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x9f110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f114 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f118 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0x9f11c | store | 4 | `[sp-20] +0x18` | `[[sp-20]+16]` |
| `ldr` at 0x9f120 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f128 | store | 4 | `[sp-20] +0x1c` | `0x0` |
| `ldrh` at 0x9f130 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0x9f138 | store | 2 | `sp -0x26` | `?` |
| `ldrb` at 0x9f13c | load | 1 | `sp -0x26` | `-` |
| `strb` at 0x9f140 | store | 1 | `sp -0x2c` | `([sp-38]:1 & 0xff)` |
| `ldrh` at 0x9f144 | load | 2 | `sp -0x26` | `-` |
| `strb` at 0x9f14c | store | 1 | `sp -0x2b` | `?` |
| `strh` at 0x9f154 | store | 2 | `sp -0x26` | `0x0` |
| `ldrh` at 0x9f158 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x9f16c | store | 4 | `sp -0x34` | `0x0` |
| `ldrh` at 0x9f170 | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0x9f184 | load | 1 | `(unknown)` | `-` |
| `str` at 0x9f188 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x9f18c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9f198 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9f1a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9f1b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9f1c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x9f1d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f1d8 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0x9f1e8 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x9f1f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f1f4 | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0x9f204 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0x9f210 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x9f214 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9f220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f228 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0x9f234 | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0x9f23c | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0x9f240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f248 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0x9f24c | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0x9f260 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x9f26c | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0x9f274 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0x9f278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f27c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f280 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x9f288 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0x9f290 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f298 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0x9f29c | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0x9f2b4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x9f2b8 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0x9f2c0 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0x9f2c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f2c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f2cc | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x9f2d4 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldrh` at 0x9f2d8 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0x9f2e0 | store | 2 | `sp -0x26` | `([sp-38]:2+1 & 0xffff)` |
| `ldr` at 0x9f2e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f2ec | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0x9f2f4 | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `ldr` at 0x9f2f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f2fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f300 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x9f308 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0x9f30c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f314 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f318 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f31c | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0x9f320 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x9f330 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x9f334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f33c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9f348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f34c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f354 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x9f35c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x9f364 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x9f368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f36c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9f370 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0x9f374 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x9f37c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9f380 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x9f388 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9f38c | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0x9f390 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x9f394 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9f394 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9f394 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9f054 | `(unknown callee) 0x9ec7c` | `[[sp-32]]:1` | `[sp-38]:2` | `[sp-38]:2` | `[[sp-32]]:1` |
| `mov` at 0x9f0b0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0x9f100 | `(unknown callee) 0x9ecd4` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |
| `mov` at 0x9f328 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0x9f378 | `(unknown callee) 0x9ecd4` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9eef8 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0x9ef1c | `[[sp-32]]:1 == 0x13` | `[[sp-32]]:1 != 0x13` |
| `bgt` at 0x9ef28 | `[[sp-32]]:1 > 0x13 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x13 (signed, 32-bit)` |
| `beq` at 0x9ef34 | `[[sp-32]]:1 == 0xd` | `[[sp-32]]:1 != 0xd` |
| `beq` at 0x9ef40 | `[[sp-32]]:1 == 0x11` | `[[sp-32]]:1 != 0x11` |
| `bhi` at 0x9ef54 | `[[sp-32]]:1-125 > 0x1 (unsigned, 32-bit)` | `[[sp-32]]:1-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x9ef68 | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0x9ef84 | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0x9efa4 | `Z == 1` | `not (Z == 1)` |
| `blo` at 0x9f090 | `[[sp-20]+28]+5 < [[sp-20]+20] (unsigned, 32-bit)` | `[[sp-20]+28]+5 >= [[sp-20]+20] (unsigned, 32-bit)` |
| `beq` at 0x9f0cc | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |
| `bls` at 0x9f160 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x9f194 | `[sp-64] == 0x13` | `[sp-64] != 0x13` |
| `bgt` at 0x9f1a0 | `[sp-64] > 0x13 (signed, 32-bit)` | `[sp-64] <= 0x13 (signed, 32-bit)` |
| `beq` at 0x9f1ac | `[sp-64] == 0xd` | `[sp-64] != 0xd` |
| `beq` at 0x9f1b8 | `[sp-64] == 0x11` | `[sp-64] != 0x11` |
| `bhi` at 0x9f1cc | `[sp-64]-125 > 0x1 (unsigned, 32-bit)` | `[sp-64]-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x9f1e0 | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0x9f1fc | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0x9f21c | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `beq` at 0x9f344 | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9f394 | `r0` | `?` |

## `SocketExportInetConnection`

- `SocketExportInetConnection`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d51c to 0x9d6ec.
  Established. The call to `0x18c8c` carries candidates `0x2`, `0x1`, `0x0`, and `0x1` (`bl` at
  0x9d544). A signed branch takes `ret(0x18c8c) >= 0x0` and falls through for a negative result
  (`bge` at 0x9d558). A word load addresses `ret(0x188b4) +0x0` (`ldr` at 0x9d56c). The exit returns
  `[sp-48]` in r0 (`ldmdb` at 0x9d6cc). The exported connection semantics are unknown from these
  operations.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d52c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9d534 | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0x9d54c | store | 4 | `sp -0x28` | `ret(0x18c8c)` |
| `ldr` at 0x9d550 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9d564 | load | 4 | `abs 0x9d6d0` | `-` |
| `ldr` at 0x9d568 | load | 4 | `abs 0x9d6d4` | `-` |
| `ldr` at 0x9d56c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d570 | load | 4 | `abs 0x9d6d8` | `-` |
| `str` at 0x9d57c | store | 4 | `sp -0x30` | `?` |
| `str` at 0x9d58c | store | 4 | `sp -0x38` | `0x4` |
| `ldr` at 0x9d590 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9d5b4 | load | 4 | `abs 0x9d6d0` | `-` |
| `ldr` at 0x9d5b8 | load | 4 | `abs 0x9d6dc` | `-` |
| `ldr` at 0x9d5bc | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d5c0 | load | 4 | `abs 0x9d6e0` | `-` |
| `ldr` at 0x9d5c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9d5d4 | store | 4 | `sp -0x30` | `?` |
| `strh` at 0x9d5f4 | store | 2 | `sp -0x24` | `0x2` |
| `ldrh` at 0x9d5f8 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x9d608 | store | 2 | `sp -0x22` | `(ret(0x18c08) & 0xffff)` |
| `str` at 0x9d610 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x9d618 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9d640 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9d644 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0x9d648 | load | 4 | `abs 0x9d6d8` | `-` |
| `str` at 0x9d64c | store | 4 | `sp -0x34` | `0xfd2d0` |
| `ldr` at 0x9d650 | load | 4 | `abs 0x9d6d0` | `-` |
| `ldr` at 0x9d654 | load | 4 | `abs 0x9d6e4` | `-` |
| `ldr` at 0x9d658 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d65c | load | 4 | `abs 0x9d6d8` | `-` |
| `ldr` at 0x9d664 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9d670 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x9d678 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9d698 | load | 4 | `abs 0x9d6d0` | `-` |
| `ldr` at 0x9d69c | load | 4 | `abs 0x9d6e8` | `-` |
| `ldr` at 0x9d6a0 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d6a4 | load | 4 | `abs 0x9d6d8` | `-` |
| `ldr` at 0x9d6ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9d6b8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x9d6c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9d6c4 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x9d6c8 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x9d6cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9d6cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9d6cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d544 | `(unknown callee) 0x18c8c` | `0x2` | `0x1` | `0x0` | `0x1` |
| `bl` at 0x9d55c | `(unknown callee) 0x188b4` | `ret(0x18c8c)` | `?` | `?` | `ret(0x18c8c)` |
| `bl` at 0x9d574 | `(unknown callee) 0x1e384` | `0xfffffe70` | `0xfd2c8` | `[ret(0x188b4)]` | `0xfd2d0` |
| `bl` at 0x9d59c | `(unknown callee) 0x189a4` | `ret(0x18c8c)` | `0x1` | `0x2` | `sp-44` |
| `bl` at 0x9d5ac | `(unknown callee) 0x188b4` | `ret(0x189a4)` | `?` | `?` | `ret(0x189a4)` |
| `bl` at 0x9d5c4 | `(unknown callee) 0x1e384` | `0xfffffe70` | `0xfd2d4` | `[ret(0x188b4)]` | `0xfd2e0` |
| `bl` at 0x9d5cc | `(unknown callee) 0x18800` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x9d5ec | `(unknown callee) 0x18c20` | `sp-36` | `0x0` | `0x10` | `sp-36` |
| `bl` at 0x9d600 | `(unknown callee) 0x18c08` | `[sp-20]:2` | `?` | `?` | `[sp-20]:2` |
| `bl` at 0x9d624 | `(unknown callee) 0x18ac4` | `[sp-40]` | `sp-36` | `0x10` | `sp-36` |
| `bl` at 0x9d634 | `(unknown callee) 0x188b4` | `ret(0x18ac4)` | `?` | `?` | `ret(0x18ac4)` |
| `bl` at 0x9d660 | `(unknown callee) 0x1e3e8` | `0xfffffe70` | `0xfd2f0` | `[ret(0x188b4)]` | `0xfd2d0` |
| `bl` at 0x9d668 | `(unknown callee) 0x18800` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x9d680 | `(unknown callee) 0x18914` | `[sp-40]` | `0x5` | `?` | `ret(0x18ac4)` |
| `bl` at 0x9d690 | `(unknown callee) 0x188b4` | `ret(0x18914)` | `?` | `?` | `ret(0x18914)` |
| `bl` at 0x9d6a8 | `(unknown callee) 0x1e384` | `0xfffffe70` | `0xfd2f8` | `[ret(0x188b4)]` | `0xfd2d0` |
| `bl` at 0x9d6b0 | `(unknown callee) 0x18800` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x9d558 | `ret(0x18c8c) >= 0x0 (signed, 32-bit)` | `ret(0x18c8c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x9d5a8 | `ret(0x189a4) >= 0x0 (signed, 32-bit)` | `ret(0x189a4) < 0x0 (signed, 32-bit)` |
| `bge` at 0x9d630 | `ret(0x18ac4) >= 0x0 (signed, 32-bit)` | `ret(0x18ac4) < 0x0 (signed, 32-bit)` |
| `bge` at 0x9d68c | `ret(0x18914) >= 0x0 (signed, 32-bit)` | `ret(0x18914) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9d6cc | `r0` | `[sp-48]` |

## `SocketExportUnixConnection`

- `SocketExportUnixConnection`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d6ec to 0x9d840.
  Established. Calling `0x18c8c` presents candidates `0x1`, `0x1`, `0x0`, and entry `r3` (`bl` at
  0x9d70c); its result controls a signed branch taken for `ret(0x18c8c) >= 0x0`, falling through for
  negative values (`bge` at 0x9d720). A word read uses `ret(0x188b4) +0x0` (`ldr` at 0x9d734). The
  return reports `[sp-144]` in r0 (`ldmdb` at 0x9d828). The Unix connection role remains unknown
  without callee identities.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d6fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9d714 | store | 4 | `sp -0x88` | `ret(0x18c8c)` |
| `ldr` at 0x9d718 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d730 | load | 4 | `abs 0x9d82c` | `-` |
| `ldr` at 0x9d734 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d738 | load | 4 | `abs 0x9d830` | `-` |
| `str` at 0x9d744 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x9d754 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9d760 | store | 4 | `sp -0x8c` | `ret(0x9e6e0)` |
| `ldr` at 0x9d778 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d780 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x9d7a8 | store | 4 | `sp -0x94` | `sp-130` |
| `ldr` at 0x9d7b0 | load | 4 | `abs 0x9d834` | `-` |
| `ldr` at 0x9d7b4 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d7b8 | load | 4 | `abs 0x9d838` | `-` |
| `ldr` at 0x9d7c0 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9d7cc | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x9d7d4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x9d7f8 | load | 4 | `abs 0x9d83c` | `-` |
| `ldr` at 0x9d7fc | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d800 | load | 4 | `abs 0x9d830` | `-` |
| `ldr` at 0x9d808 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9d814 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x9d81c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x9d820 | store | 4 | `sp -0x90` | `[sp-136]` |
| `ldr` at 0x9d824 | load | 4 | `sp -0x90` | `-` |
| `ldmdb` at 0x9d828 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9d828 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9d828 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d70c | `(unknown callee) 0x18c8c` | `0x1` | `0x1` | `0x0` | `r3` |
| `bl` at 0x9d724 | `(unknown callee) 0x188b4` | `ret(0x18c8c)` | `?` | `?` | `ret(0x18c8c)` |
| `bl` at 0x9d73c | `(unknown callee) 0x1e384` | `?` | `0xfd2c8` | `[ret(0x188b4)]` | `0xfd2d0` |
| `bl` at 0x9d758 | `(unknown callee) 0x9e6e0` | `sp-132` | `[sp-20]` | `?` | `sp-132` |
| `bl` at 0x9d770 | `(unknown callee) 0x1883c` | `sp-130` | `?` | `?` | `sp-130` |
| `bl` at 0x9d784 | `(unknown callee) 0x18ac4` | `[sp-136]` | `sp-132` | `[sp-140]` | `sp-132` |
| `bl` at 0x9d794 | `(unknown callee) 0x188b4` | `ret(0x18ac4)` | `?` | `?` | `ret(0x18ac4)` |
| `bl` at 0x9d7bc | `(unknown callee) 0x1e384` | `?` | `0xfd2f0` | `[ret(0x188b4)]` | `0xfd300` |
| `bl` at 0x9d7c4 | `(unknown callee) 0x18800` | `[sp-136]` | `?` | `?` | `?` |
| `bl` at 0x9d7dc | `(unknown callee) 0x18914` | `[sp-136]` | `0x5` | `?` | `ret(0x18ac4)` |
| `bl` at 0x9d7ec | `(unknown callee) 0x188b4` | `ret(0x18914)` | `?` | `?` | `ret(0x18914)` |
| `bl` at 0x9d804 | `(unknown callee) 0x1e384` | `?` | `0xfd2f8` | `[ret(0x188b4)]` | `0xfd2d0` |
| `bl` at 0x9d80c | `(unknown callee) 0x18800` | `[sp-136]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x9d720 | `ret(0x18c8c) >= 0x0 (signed, 32-bit)` | `ret(0x18c8c) < 0x0 (signed, 32-bit)` |
| `bge` at 0x9d790 | `ret(0x18ac4) >= 0x0 (signed, 32-bit)` | `ret(0x18ac4) < 0x0 (signed, 32-bit)` |
| `bge` at 0x9d7e8 | `ret(0x18914) >= 0x0 (signed, 32-bit)` | `ret(0x18914) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9d828 | `r0` | `[sp-144]` |

## `SocketAcceptInetConnection`

- `SocketAcceptInetConnection`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9d840 to 0x9d92c.
  Established. The call to `0x188c0` uses candidates entry `r0`, `sp-44`, `sp-48`, and `sp-44` (`bl`
  at 0x9d870). The signed result branch takes `ret(0x188c0) >= 0x0` and falls through for negative
  values (`bge` at 0x9d884). A second branch takes `[sp-24] == 0x0` and falls through on inequality
  (`beq` at 0x9d8b8). The word read at `ret(0x188b4) +0x0` has an unknown value (`ldr` at 0x9d898).
  The accepted-connection semantics and external device effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9d850 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9d854 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9d85c | store | 4 | `sp -0x30` | `0x10` |
| `ldr` at 0x9d868 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9d878 | store | 4 | `sp -0x1c` | `ret(0x188c0)` |
| `ldr` at 0x9d87c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9d890 | load | 4 | `abs 0x9d918` | `-` |
| `ldr` at 0x9d894 | load | 4 | `abs 0x9d91c` | `-` |
| `ldr` at 0x9d898 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d89c | load | 4 | `abs 0x9d920` | `-` |
| `str` at 0x9d8a8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x9d8b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9d8bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9d8e0 | load | 4 | `abs 0x9d918` | `-` |
| `ldr` at 0x9d8e4 | load | 4 | `abs 0x9d924` | `-` |
| `ldr` at 0x9d8e8 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0x9d8ec | load | 4 | `abs 0x9d928` | `-` |
| `ldr` at 0x9d8f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9d900 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x9d908 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9d90c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x9d910 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x9d914 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9d914 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9d914 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9d870 | `(unknown callee) 0x188c0` | `r0` | `sp-44` | `sp-48` | `sp-44` |
| `bl` at 0x9d888 | `(unknown callee) 0x188b4` | `ret(0x188c0)` | `?` | `?` | `ret(0x188c0)` |
| `bl` at 0x9d8a0 | `(unknown callee) 0x1e384` | `0xfffffe6e` | `0xfd304` | `[ret(0x188b4)]` | `0xfd2d0` |
| `bl` at 0x9d8c8 | `(unknown callee) 0x18adc` | `ret(0x188c0)` | `0x4` | `0x800` | `[sp-24]` |
| `bl` at 0x9d8d8 | `(unknown callee) 0x188b4` | `ret(0x18adc)` | `?` | `?` | `ret(0x18adc)` |
| `bl` at 0x9d8f0 | `(unknown callee) 0x1e384` | `0xfffffe6e` | `0xfd30c` | `[ret(0x188b4)]` | `0xfd314` |
| `bl` at 0x9d8f8 | `(unknown callee) 0x18800` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x9d884 | `ret(0x188c0) >= 0x0 (signed, 32-bit)` | `ret(0x188c0) < 0x0 (signed, 32-bit)` |
| `beq` at 0x9d8b8 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x9d8d4 | `ret(0x18adc) == 0x0` | `ret(0x18adc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9d914 | `r0` | `?` |
