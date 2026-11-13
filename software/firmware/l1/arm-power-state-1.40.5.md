# ARM power control and status callers in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
The conditional computed transfer in `powerGetReading` remains unresolved (`ldrls` at
0x40260). Known-edge tables exclude effects reached only through its unknown destination.
Raw undecodable bytes remain in the fact-sheet footer.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/power-state-facts.txt) retain all tool sections.

## `powerInit`

- `powerInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c5cc to 0x3c7fc.
  Established. The branch falls through when `[0x148d04] == 0x0` (`bne` at 0x3c630), reaching a
  four-byte `0x1` store at absolute `0x148d04` (`str` at 0x3c63c). The unnamed call to `0x1b454`
  carries `[sp-40]`, `0x1`, `[0x148d04]`, unknown, with its result assigned to absolute `0x148d00`
  (`bl` at 0x3c668, `str` at 0x3c670). Further four-byte stores include `[sp-44] +0x8`, `+0x10` and
  `+0x14`, retaining `[sp-32]`, `[sp-36]` and `[sp]` (`str` at 0x3c6dc, `str` at 0x3c6e8, `str` at
  0x3c6f4). Power-record roles and allocation semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c5dc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3c5e0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3c5e4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x3c5e8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x3c5ec | load | 4 | `abs 0x3c7d0` | `-` |
| `ldr` at 0x3c5f0 | load | 4 | `abs 0x148d00` | `-` |
| `ldr` at 0x3c5fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c608 | load | 4 | `abs 0x3c7d4` | `-` |
| `str` at 0x3c60c | store | 4 | `sp -0x3c` | `0x108dd8` |
| `ldr` at 0x3c614 | load | 4 | `abs 0x3c7d8` | `-` |
| `ldr` at 0x3c618 | load | 4 | `abs 0x3c7dc` | `-` |
| `ldr` at 0x3c624 | load | 4 | `abs 0x3c7e0` | `-` |
| `ldr` at 0x3c628 | load | 4 | `abs 0x148d04` | `-` |
| `ldr` at 0x3c634 | load | 4 | `abs 0x3c7e0` | `-` |
| `str` at 0x3c63c | store | 4 | `abs 0x148d04` | `0x1` |
| `ldr` at 0x3c640 | load | 4 | `abs 0x3c7e0` | `-` |
| `ldr` at 0x3c644 | load | 4 | `abs 0x148d04` | `-` |
| `str` at 0x3c658 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3c65c | load | 4 | `abs 0x3c7d0` | `-` |
| `ldr` at 0x3c660 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3c670 | store | 4 | `abs 0x148d00` | `ret(0x1b454)` |
| `ldr` at 0x3c674 | load | 4 | `abs 0x148d00` | `-` |
| `ldr` at 0x3c680 | load | 4 | `abs 0x3c7e4` | `-` |
| `str` at 0x3c684 | store | 4 | `sp -0x3c` | `0x108e10` |
| `ldr` at 0x3c688 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3c68c | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0x3c694 | load | 4 | `abs 0x3c7d8` | `-` |
| `ldr` at 0x3c698 | load | 4 | `abs 0x3c7e8` | `-` |
| `ldr` at 0x3c6a4 | load | 4 | `abs 0x3c7d0` | `-` |
| `ldr` at 0x3c6a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c6bc | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3c6c4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3c6c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c6cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c6d0 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0x3c6d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c6d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c6dc | store | 4 | `[sp-44] +0x8` | `[sp-32]` |
| `ldr` at 0x3c6e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c6e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c6e8 | store | 4 | `[sp-44] +0x10` | `[sp-36]` |
| `ldr` at 0x3c6ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c6f0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x3c6f4 | store | 4 | `[sp-44] +0x14` | `[sp]` |
| `ldr` at 0x3c6f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c6fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c700 | load | 4 | `[sp-44] +0x8` | `-` |
| `str` at 0x3c710 | store | 4 | `[sp-44] +0x4` | `ret(0x1b454)` |
| `ldr` at 0x3c714 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3c720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c728 | load | 4 | `abs 0x3c7ec` | `-` |
| `ldr` at 0x3c72c | load | 4 | `abs 0x3c7f0` | `-` |
| `str` at 0x3c758 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3c75c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c764 | load | 4 | `abs 0x3c7ec` | `-` |
| `ldr` at 0x3c768 | load | 4 | `abs 0x3c7f0` | `-` |
| `ldr` at 0x3c774 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3c784 | load | 4 | `abs 0x3c7f4` | `-` |
| `str` at 0x3c788 | store | 4 | `(unknown)` | `0x108e5c` |
| `str` at 0x3c78c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x3c790 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c794 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x3c79c | load | 4 | `abs 0x3c7d8` | `-` |
| `ldr` at 0x3c7a0 | load | 4 | `abs 0x3c7f8` | `-` |
| `ldr` at 0x3c7ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c7b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c7b4 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3c7bc | load | 4 | `[sp-44] +0x8` | `-` |
| `ldmdb` at 0x3c7cc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3c7cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c7cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c7cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c620 | `(unknown callee) 0x21eb8` | `0x9` | `0x108df8` | `0x1a9` | `0x0` |
| `bl` at 0x3c668 | `(unknown callee) 0x1b454` | `[sp-40]` | `0x1` | `[0x148d04]` | `?` |
| `bl` at 0x3c6a0 | `(unknown callee) 0x21eb8` | `0x9` | `0x108df8` | `0x1b3` | `0x0` |
| `bl` at 0x3c708 | `(unknown callee) 0x1b454` | `[[sp-44]+8]` | `0x1` | `[sp-44]` | `[sp-44]` |
| `bl` at 0x3c730 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3c73c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3c76c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3c77c | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3c7a8 | `(unknown callee) 0x21eb8` | `0x9` | `0x108df8` | `0x1c1` | `0x0` |
| `bl` at 0x3c7c0 | `(unknown callee) 0x1b4cc` | `[[sp-44]+4]` | `0x1` | `[[sp-44]+8]` | `[sp-44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3c5f8 | `[0x148d00] == 0x0` | `[0x148d00] != 0x0` |
| `bne` at 0x3c604 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x3c630 | `[0x148d04] != 0x0` | `[0x148d04] == 0x0` |
| `bne` at 0x3c67c | `[0x148d00] != 0x0` | `[0x148d00] == 0x0` |
| `bne` at 0x3c71c | `[[sp-44]+4] != 0x0` | `[[sp-44]+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c7cc | `r0` | `0x0` |

## `powerNotPresent`

- `powerNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c7fc to 0x3c920.
  Established. A signed-negative `r1` takes the branch to `0x3c85c` (`blt` at 0x3c840). A later
  branch reaches that destination when `r1 >= [[sp-28]+8] (signed, 32-bit)` (`bge` at 0x3c854). The
  other path writes byte `0x0` at `([[sp-28]+4] + r1) +0x0` (`strb` at 0x3c900). The exit value is
  unknown (`ldmdb` at 0x3c904). The stored byte has no established hardware-presence interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c80c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c810 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3c814 | load | 4 | `abs 0x3c908` | `-` |
| `ldr` at 0x3c818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c82c | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3c834 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c84c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3c85c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c864 | load | 4 | `abs 0x3c90c` | `-` |
| `ldr` at 0x3c868 | load | 4 | `abs 0x3c910` | `-` |
| `str` at 0x3c894 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3c898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c8a0 | load | 4 | `abs 0x3c90c` | `-` |
| `ldr` at 0x3c8a4 | load | 4 | `abs 0x3c910` | `-` |
| `ldr` at 0x3c8b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c8c0 | load | 4 | `abs 0x3c914` | `-` |
| `str` at 0x3c8c4 | store | 4 | `(unknown)` | `0x108ea0` |
| `str` at 0x3c8c8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x3c8cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c8d0 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x3c8d8 | load | 4 | `abs 0x3c918` | `-` |
| `ldr` at 0x3c8dc | load | 4 | `abs 0x3c91c` | `-` |
| `ldr` at 0x3c8ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c8f0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3c8f4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3c900 | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x0` |
| `ldmdb` at 0x3c904 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c904 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c904 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c86c | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3c878 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3c8a8 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3c8b8 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3c8e4 | `(unknown callee) 0x21eb8` | `0x6` | `0x108df8` | `0x1ce` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3c840 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3c854 | `r1 >= [[sp-28]+8] (signed, 32-bit)` | `r1 < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c904 | `r0` | `?` |

## `powerPresent`

- `powerPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c920 to 0x3ca44.
  Established. Two branches select `0x3c980` for signed-negative `r1` or for `r1 >= [[sp-28]+8]
  (signed, 32-bit)` (`blt` at 0x3c964, `bge` at 0x3c978). The store path instead assigns byte `0x1`
  at `([[sp-28]+4] + r1) +0x0` (`strb` at 0x3ca24). Included unnamed calls are `0x4f9a4` with `r0`,
  `0x0`, `0x108e50`, `0x108e58`, and `0x21eb8` with `0x6`, `0x108df8`, `0x1de`, `0x0` (`bl` at
  0x3c990, `bl` at 0x3ca08). Physical presence detection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c930 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c934 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3c938 | load | 4 | `abs 0x3ca2c` | `-` |
| `ldr` at 0x3c93c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c950 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3c958 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c95c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c968 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c96c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c970 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3c980 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c988 | load | 4 | `abs 0x3ca30` | `-` |
| `ldr` at 0x3c98c | load | 4 | `abs 0x3ca34` | `-` |
| `str` at 0x3c9b8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3c9bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c9c4 | load | 4 | `abs 0x3ca30` | `-` |
| `ldr` at 0x3c9c8 | load | 4 | `abs 0x3ca34` | `-` |
| `ldr` at 0x3c9d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c9e4 | load | 4 | `abs 0x3ca38` | `-` |
| `str` at 0x3c9e8 | store | 4 | `(unknown)` | `0x108ecc` |
| `str` at 0x3c9ec | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x3c9f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c9f4 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x3c9fc | load | 4 | `abs 0x3ca3c` | `-` |
| `ldr` at 0x3ca00 | load | 4 | `abs 0x3ca40` | `-` |
| `ldr` at 0x3ca10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ca14 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3ca18 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3ca24 | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x1` |
| `ldmdb` at 0x3ca28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ca28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ca28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c990 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3c99c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3c9cc | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x108e50` | `0x108e58` |
| `bl` at 0x3c9dc | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3ca08 | `(unknown callee) 0x21eb8` | `0x6` | `0x108df8` | `0x1de` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3c964 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3c978 | `r1 >= [[sp-28]+8] (signed, 32-bit)` | `r1 < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ca28 | `r0` | `?` |

## `powerReset`

- `powerReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ca44 to 0x3cc10.
  Established. The byte store at `[sp-24] +0xf` initially has unknown value (`strb` at 0x3ca8c).
  Later assignments at that same printed target include `([sp-36]:1 & 0xff)` and `0x5` (`strb` at
  0x3cb8c, `strb` at 0x3cbb0). The call to unnamed `0x40d80` has candidates `[sp-20]`, `[sp-28]`,
  `[sp-29]:1`, `[sp-29]:1` (`bl` at 0x3cbc4). A zero result takes the branch to `0x3cbf0` (`beq` at
  0x3cbd8). The returned expression is `[sp-40]` (`ldmdb` at 0x3cc08). Reset and margin-state
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ca54 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3ca58 | load | 4 | `abs 0x3cc0c` | `-` |
| `ldr` at 0x3ca5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ca70 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3ca78 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x3ca80 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3ca84 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3ca8c | store | 1 | `[sp-24] +0xf` | `?` |
| `str` at 0x3ca94 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3ca98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ca9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3caa0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3cab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cab4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3cac8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x3cad0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3cad4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cad8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x3cadc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3cae4 | load | 1 | `([[sp-24]+4] + [sp-48]) +0x0` | `-` |
| `ldr` at 0x3caf0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3caf4 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldr` at 0x3cb00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cb04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3cb10 | store | 4 | `sp -0x24` | `ret(0x4128c)` |
| `ldr` at 0x3cb14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cb18 | store | 4 | `sp -0x34` | `ret(0x4128c)` |
| `ldr` at 0x3cb1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3cb28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3cb34 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3cb40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cb44 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3cb50 | store | 1 | `sp -0x1d` | `(ret(0x4121c) & 0xff)` |
| `str` at 0x3cb5c | store | 4 | `sp -0x24` | `0x2` |
| `ldr` at 0x3cb60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cb64 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3cb70 | store | 1 | `sp -0x1d` | `(ret(0x4089c) & 0xff)` |
| `ldr` at 0x3cb74 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0x3cb78 | load | 1 | `[sp-24] +0xf` | `-` |
| `ldr` at 0x3cb84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3cb88 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x3cb8c | store | 1 | `[sp-24] +0xf` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x3cb94 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0x3cb98 | load | 1 | `[sp-24] +0xf` | `-` |
| `ldr` at 0x3cb9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3cba8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3cbb0 | store | 1 | `[sp-24] +0xf` | `0x5` |
| `ldrb` at 0x3cbb4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3cbb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cbbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3cbcc | store | 4 | `sp -0x2c` | `ret(0x40d80)` |
| `ldr` at 0x3cbd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cbdc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cbe8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3cbec | store | 4 | `sp -0x28` | `ret(0x40d80)` |
| `ldr` at 0x3cbf0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3cbf8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x3cc00 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x3cc08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3cc08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3cc08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3cb08 | `(unknown callee) 0x4128c` | `[sp-20]` | `[sp-28]` | `[[sp-24]+4]` | `[[sp-28]+6]:1` |
| `bl` at 0x3cb48 | `(unknown callee) 0x4121c` | `[sp-20]` | `[sp-28]` | `?` | `ret(0x4128c)` |
| `bl` at 0x3cb68 | `(unknown callee) 0x4089c` | `[sp-20]` | `[sp-28]` | `?` | `?` |
| `bl` at 0x3cbc4 | `(unknown callee) 0x40d80` | `[sp-20]` | `[sp-28]` | `[sp-29]:1` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3caa8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3caec | `[([[sp-24]+4] + [sp-48])]:1 == 0x0` | `[([[sp-24]+4] + [sp-48])]:1 != 0x0` |
| `beq` at 0x3cafc | `[[sp-28]+6]:1 == 0xff` | `[[sp-28]+6]:1 != 0xff` |
| `beq` at 0x3cb24 | `ret(0x4128c) == 0x0` | `ret(0x4128c) != 0x0` |
| `blt` at 0x3cb30 | `ret(0x4128c) < 0x0 (signed, 32-bit)` | `ret(0x4128c) >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x3cb3c | `ret(0x4128c) > 0x4 (signed, 32-bit)` | `ret(0x4128c) <= 0x4 (signed, 32-bit)` |
| `bne` at 0x3cb80 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3cba4 | `? == [sp-36]` | `? != [sp-36]` |
| `beq` at 0x3cbd8 | `ret(0x40d80) == 0x0` | `ret(0x40d80) != 0x0` |
| `bne` at 0x3cbe4 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3cc08 | `r0` | `[sp-40]` |

## `powerGetCfg`

- `powerGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3cc10 to 0x3cc70.
  Established. A four-byte load reads absolute `0x148d00` (`ldr` at 0x3cc40). Another four-byte load
  at `[sp-28] +0x0` supplies the four-byte store of `[[sp-28]]` at `r1 +0x0` (`ldr` at 0x3cc54,
  `str` at 0x3cc58). The exit carries `[[sp-28]+8]` (`ldmdb` at 0x3cc68). The configuration role and
  count unit are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3cc20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3cc24 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3cc28 | load | 4 | `abs 0x3cc6c` | `-` |
| `ldr` at 0x3cc2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cc40 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3cc48 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3cc4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cc50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cc54 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3cc58 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x3cc5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cc60 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0x3cc68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3cc68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3cc68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3cc68 | `r0` | `[[sp-28]+8]` |

## `powerIsPresent`

- `powerIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3cc70 to 0x3cd00.
  Established. Signed-negative `r1` takes the branch to `0x3ccd0` (`blt` at 0x3ccb4). A second
  branch selects the same destination when `r1 >= [[sp-28]+8] (signed, 32-bit)` (`bge` at 0x3ccc8).
  That destination writes four-byte zero at `sp -0x20` (`str` at 0x3ccd4). The other path reads a
  byte at `([[sp-28]+4] + r1) +0x0` and stores the expression as four bytes at `sp -0x20` (`ldrb` at
  0x3ccec, `str` at 0x3ccf0). The final return is unknown (`ldmdb` at 0x3ccf8). Presence semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3cc80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3cc84 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3cc88 | load | 4 | `abs 0x3ccfc` | `-` |
| `ldr` at 0x3cc8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cca0 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3cca8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3ccac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ccb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ccbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ccc0 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x3ccd4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3ccdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3cce0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3cce4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3ccec | load | 1 | `([[sp-28]+4] + r1) +0x0` | `-` |
| `str` at 0x3ccf0 | store | 4 | `sp -0x20` | `[([[sp-28]+4] + r1)]:1` |
| `ldr` at 0x3ccf4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x3ccf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ccf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ccf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3ccb4 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3ccc8 | `r1 >= [[sp-28]+8] (signed, 32-bit)` | `r1 < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ccf8 | `r0` | `?` |

## `powerUpSlot`

- `powerUpSlot`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3cd00 to 0x3d008.
  Established. Byte assignments include `0x0` at `[sp-44] +0xc`, `0x1` at `[sp-44] +0xe`, and `0x1`
  at `[sp-44] +0xd` (`strb` at 0x3cd90, `strb` at 0x3cdb0, `strb` at 0x3cf70). The branch taken when
  `[[sp-44]+16] == 0x0` skips an indirect-call arm (`beq` at 0x3cf0c). That arm contains an
  unknown-target call with candidates `[sp-20]`, unknown, unknown, `[[sp-44]+16]` (`mov` at
  0x3cf28). The later byte store clears `[sp-44] +0xe` (`strb` at 0x3cfd0). Power-sequence and
  callback effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3cd10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3cd14 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3cd18 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3cd20 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3cd24 | load | 4 | `abs 0x3cff8` | `-` |
| `ldr` at 0x3cd28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cd3c | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3cd44 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3cd48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3cd50 | store | 4 | `sp -0x34` | `ret(0x4ea90)` |
| `ldr` at 0x3cd54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cd60 | load | 4 | `abs 0x3cffc` | `-` |
| `str` at 0x3cd64 | store | 4 | `sp -0x38` | `0x108ef8` |
| `ldr` at 0x3cd6c | load | 4 | `abs 0x3d000` | `-` |
| `str` at 0x3cd70 | store | 4 | `sp -0x38` | `0x108efc` |
| `ldr` at 0x3cd74 | load | 4 | `abs 0x3d004` | `-` |
| `ldr` at 0x3cd78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cd7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3cd80 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3cd88 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3cd90 | store | 1 | `[sp-44] +0xc` | `0x0` |
| `str` at 0x3cda0 | store | 4 | `sp -0x20` | `ret(0x4e5ec)` |
| `ldr` at 0x3cda8 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3cdb0 | store | 1 | `[sp-44] +0xe` | `0x1` |
| `str` at 0x3cdb8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3cdbc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cdc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cdc4 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x3cdd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cdd8 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3cddc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3cde4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3cdf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cdf4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ce08 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrb` at 0x3ce10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3ce1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ce20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ce34 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x3ce3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3ce4c | store | 4 | `sp -0x24` | `ret(0x40af8)` |
| `ldr` at 0x3ce50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ce5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ce6c | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3ce74 | store | 1 | `[sp-44] +0xe` | `0x0` |
| `ldr` at 0x3ce78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ce7c | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x3ce84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ce88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ce9c | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrh` at 0x3cea4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3ceb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ceb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cec8 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrh` at 0x3ced0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3cee8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3cef0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3cf00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cf04 | load | 4 | `[sp-44] +0x10` | `-` |
| `str` at 0x3cf14 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x3cf18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cf1c | load | 4 | `[sp-44] +0x10` | `-` |
| `ldr` at 0x3cf20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cf38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3cf48 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x3cf4c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3cf50 | store | 4 | `sp -0x24` | `[sp-60]` |
| `ldr` at 0x3cf5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cf60 | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x3cf68 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3cf70 | store | 1 | `[sp-44] +0xd` | `0x1` |
| `ldr` at 0x3cf74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3cf8c | store | 4 | `sp -0x24` | `ret(0x4e54c)` |
| `ldr` at 0x3cf90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3cf9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3cfa8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cfac | store | 4 | `sp -0x20` | `ret(0x4e54c)` |
| `ldr` at 0x3cfb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cfb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cfc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cfc8 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3cfd0 | store | 1 | `[sp-44] +0xe` | `0x0` |
| `ldr` at 0x3cfd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cfe8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3cfec | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x3cff0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x3cff4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3cff4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3cff4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3cd4c | `(unknown callee) 0x4ea90` | `r2` | `0x148d00` | `?` | `?` |
| `bl` at 0x3cd84 | `(unknown callee) 0x2079c` | `0x108f04` | `[sp-20]` | `[sp-56]` | `ret(0x4ea90)` |
| `bl` at 0x3cd98 | `(unknown callee) 0x4e5ec` | `0x5` | `?` | `[sp-44]` | `0x0` |
| `bl` at 0x3cda4 | `(unknown callee) 0x82c60` | `ret(0x4e5ec)` | `?` | `?` | `ret(0x4e5ec)` |
| `bl` at 0x3ce44 | `(unknown callee) 0x40af8` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3ce64 | `(unknown callee) 0x3d688` | `[sp-20]` | `0x5` | `?` | `ret(0x40af8)` |
| `bl` at 0x3ce68 | `(unknown callee) 0x82c78` | `ret(0x3d688)` | `?` | `?` | `?` |
| `bl` at 0x3ced8 | `(unknown callee) 0x412fc` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x3cee4 | `(unknown callee) 0x412fc` | `0x19` | `[sp-44]` | `?` | `?` |
| `bl` at 0x3cefc | `(unknown callee) 0x412fc` | `0x64` | `?` | `?` | `[[sp-44]+8]` |
| `mov` at 0x3cf28 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[[sp-44]+16]` |
| `bl` at 0x3cf64 | `(unknown callee) 0x82c78` | `?` | `?` | `?` | `?` |
| `bl` at 0x3cf84 | `(unknown callee) 0x4e54c` | `0x5` | `?` | `[sp-44]` | `[sp-24]` |
| `bl` at 0x3cfb4 | `(unknown callee) 0x5d810` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3cfbc | `(unknown callee) 0x5d994` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3cfc4 | `(unknown callee) 0x5d034` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3cfe4 | `(unknown callee) 0x82d60` | `0x0` | `?` | `[sp-44]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3cd5c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `blt` at 0x3cdcc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3cdec | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3ce18 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x3ce58 | `ret(0x40af8) == 0x0` | `ret(0x40af8) != 0x0` |
| `beq` at 0x3ceac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3cf0c | `[[sp-44]+16] == 0x0` | `[[sp-44]+16] != 0x0` |
| `beq` at 0x3cf34 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3cf40 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3cf58 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `beq` at 0x3cf7c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x3cf98 | `ret(0x4e54c) == 0x0` | `ret(0x4e54c) != 0x0` |
| `bne` at 0x3cfa4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3cfdc | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3cff4 | `r0` | `?` |

## `powerDownSlot`

- `powerDownSlot`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d688 to 0x3d890.
  Established. The branch takes the `r1 == 0x5` arm to `0x3d6f4` (`beq` at 0x3d6d4). A subsequent
  four-byte load reads `[sp-120] +0x14` (`ldr` at 0x3d710). A zero value takes the branch past an
  unknown-target call (`beq` at 0x3d718, `mov` at 0x3d734). The indirect call has candidates
  `[sp-20]`, unknown, unknown, `[[sp-120]+20]` (`mov` at 0x3d734). Included direct calls are unnamed
  `0xd72bc` and `0x40cac` (`bl` at 0x3d80c, `bl` at 0x3d830). Shutdown behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d698 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3d69c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3d6a4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x3d6a8 | load | 4 | `abs 0x3d888` | `-` |
| `ldr` at 0x3d6ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d6c0 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3d6c8 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0x3d6cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d6d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d6e4 | load | 4 | `abs 0x3d88c` | `-` |
| `ldr` at 0x3d6e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d700 | store | 4 | `sp -0x1c` | `ret(0x4e5ec)` |
| `ldr` at 0x3d704 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d70c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d710 | load | 4 | `[sp-120] +0x14` | `-` |
| `str` at 0x3d720 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x3d724 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d728 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x3d72c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d744 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3d754 | store | 4 | `sp -0x80` | `0x1` |
| `ldr` at 0x3d758 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x3d75c | store | 4 | `sp -0x20` | `[sp-128]` |
| `ldr` at 0x3d768 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d76c | store | 4 | `sp -0x1c` | `[sp-128]` |
| `ldr` at 0x3d770 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d774 | load | 4 | `[sp-120] +0x8` | `-` |
| `str` at 0x3d77c | store | 4 | `sp -0x24` | `[[sp-120]+8]-1` |
| `ldr` at 0x3d780 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d790 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d794 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d7a8 | load | 4 | `[sp-120] +0x0` | `-` |
| `str` at 0x3d7b0 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x3d7b4 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3d7b8 | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x3d7c4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d7c8 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3d7cc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3d7d4 | load | 1 | `([[sp-120]+4] + [sp-36]) +0x0` | `-` |
| `ldr` at 0x3d7e0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d7e4 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3d7e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3d7f0 | load | 1 | `([[sp-120]+4] + [sp-36]) +0x0` | `-` |
| `ldr` at 0x3d7fc | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3d800 | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x3d804 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d824 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x3d828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d82c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x3d840 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3d850 | store | 4 | `sp -0x84` | `0x1` |
| `ldr` at 0x3d854 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x3d858 | store | 4 | `sp -0x20` | `[sp-132]` |
| `ldr` at 0x3d864 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d868 | store | 4 | `sp -0x1c` | `[sp-132]` |
| `ldr` at 0x3d86c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3d874 | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `ldr` at 0x3d87c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3d884 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d884 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d884 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d6dc | `(unknown callee) 0x4ea90` | `r1` | `0x148d00` | `?` | `r1` |
| `bl` at 0x3d6f0 | `(unknown callee) 0x2079c` | `0x109008` | `[sp-20]` | `ret(0x4ea90)` | `ret(0x4ea90)` |
| `bl` at 0x3d6f8 | `(unknown callee) 0x4e5ec` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x3d708 | `(unknown callee) 0x5d1e8` | `[sp-20]` | `?` | `?` | `ret(0x4e5ec)` |
| `mov` at 0x3d734 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[[sp-120]+20]` |
| `bl` at 0x3d80c | `(unknown callee) 0xd72bc` | `[sp-20]` | `[[sp-124]+2]:1` | `[[sp-120]+4]` | `[[sp-124]+2]:1` |
| `bl` at 0x3d830 | `(unknown callee) 0x40cac` | `[sp-20]` | `[sp-124]` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d6d4 | `r1 == 0x5` | `r1 != 0x5` |
| `beq` at 0x3d718 | `[[sp-120]+20] == 0x0` | `[[sp-120]+20] != 0x0` |
| `beq` at 0x3d740 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3d74c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x3d764 | `[sp-128] == 0x0` | `[sp-128] != 0x0` |
| `bge` at 0x3d788 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3d7c0 | `[[sp-124]+2]:1 == 0xff` | `[[sp-124]+2]:1 != 0xff` |
| `bne` at 0x3d7dc | `[([[sp-120]+4] + [sp-36])]:1 != 0x0` | `[([[sp-120]+4] + [sp-36])]:1 == 0x0` |
| `bne` at 0x3d7f8 | `[([[sp-120]+4] + [sp-36])]:1 != 0x0` | `[([[sp-120]+4] + [sp-36])]:1 == 0x0` |
| `bne` at 0x3d818 | `ret(0xd72bc) != 0x0` | `ret(0xd72bc) == 0x0` |
| `beq` at 0x3d83c | `ret(0x40cac) == 0x0` | `ret(0x40cac) != 0x0` |
| `bne` at 0x3d848 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x3d860 | `[sp-132] == 0x0` | `[sp-132] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d884 | `r0` | `[sp-28]` |

## `powerStatusSlot`

- `powerStatusSlot`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dd74 to 0x3df3c.
  Established. The unnamed call to `0x3df3c` has candidates `[sp-20]` followed by three copies of
  `[[sp-48]+1]:1` (`bl` at 0x3de54). A positive signed result takes the branch to `0x3de68` (`bgt`
  at 0x3de60). A subsequent call to `0xd75c0` carries `[sp-20]`, `[[sp-48]+2]:1`, unknown,
  `[[sp-48]+2]:1` (`bl` at 0x3de84). The branch takes the unequal arm when `[sp-36] != ret(0x40aa8)`
  (`bne` at 0x3dec0). The hardware meaning of the accumulated status is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3dd84 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3dd88 | load | 4 | `abs 0x3df38` | `-` |
| `ldr` at 0x3dd8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3dda0 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3dda8 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x3ddb0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x3ddb8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x3ddc0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3ddc4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ddc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ddcc | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x3dddc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3dde0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ddf4 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x3ddfc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3de00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3de04 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3de08 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3de10 | load | 1 | `([[sp-44]+4] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x3de1c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3de20 | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x3de2c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3de30 | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x3de34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3de38 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x3de44 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3de48 | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x3de4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3de68 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3de70 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x3de74 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3de78 | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x3de7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3de8c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3de90 | load | 1 | `[sp-48] +0x3` | `-` |
| `str` at 0x3dea8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3deac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3deb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3dec4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3decc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x3ded0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ded8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x3dee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3deec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3def0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3df00 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x3df08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3df18 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x3df24 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x3df28 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3df2c | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x3df30 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3df34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3df34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3df34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3de54 | `(unknown callee) 0x3df3c` | `[sp-20]` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` |
| `bl` at 0x3de84 | `(unknown callee) 0xd75c0` | `[sp-20]` | `[[sp-48]+2]:1` | `?` | `[[sp-48]+2]:1` |
| `bl` at 0x3deb0 | `(unknown callee) 0x40aa8` | `[sp-48]` | `?` | `ret(0xd75c0)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3ddd4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3de18 | `[([[sp-44]+4] + [sp-24])]:1 == 0x0` | `[([[sp-44]+4] + [sp-24])]:1 != 0x0` |
| `beq` at 0x3de28 | `[[sp-48]+2]:1 == 0xff` | `[[sp-48]+2]:1 != 0xff` |
| `bge` at 0x3de40 | `[[sp-48]+1]:1 >= [[sp-44]+8] (signed, 32-bit)` | `[[sp-48]+1]:1 < [[sp-44]+8] (signed, 32-bit)` |
| `bgt` at 0x3de60 | `ret(0x3df3c) > 0x0 (signed, 32-bit)` | `ret(0x3df3c) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x3dec0 | `[sp-36] != ret(0x40aa8)` | `[sp-36] == ret(0x40aa8)` |
| `beq` at 0x3dee8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3def8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3df10 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3df34 | `r0` | `[sp-52]` |

## `powerStatusAll`

- `powerStatusAll`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dce4 to 0x3dd74.
  Established. The first unnamed call to `0x3dd74` has candidates `0x0`, `r1`, `r2`, `0x0` (`bl` at
  0x3dd00). A later call to the same target has unknown first three candidates and `[0x148d04]` as
  its fourth (`bl` at 0x3dd30). The equality branch takes `ret(0x3dd74) == [sp-24]` to `0x3dd50`
  (`beq` at 0x3dd40); its fall-through writes an unknown four-byte value at `sp -0x1c` (`str` at
  0x3dd48). The merged result is unknown (`ldmdb` at 0x3dd6c). A complete hardware-status
  aggregation is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3dcf8 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x3dd08 | store | 4 | `sp -0x18` | `ret(0x3dd74)` |
| `str` at 0x3dd10 | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x3dd14 | load | 4 | `abs 0x3dd70` | `-` |
| `ldr` at 0x3dd18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3dd1c | load | 4 | `abs 0x148d04` | `-` |
| `ldr` at 0x3dd2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3dd38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dd48 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3dd50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3dd58 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x3dd60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dd64 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3dd68 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3dd6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dd6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dd6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3dd00 | `(unknown callee) 0x3dd74` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3dd30 | `(unknown callee) 0x3dd74` | `?` | `?` | `?` | `[0x148d04]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3dd24 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3dd40 | `ret(0x3dd74) == [sp-24]` | `ret(0x3dd74) != [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dd6c | `r0` | `?` |

## `powerStatus`

- `powerStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3df3c to 0x3e05c.
  Established. After the unnamed call to `0x3cc70`, a nonzero result takes the branch to `0x3df78`
  (`bl` at 0x3df5c, `bne` at 0x3df68); its fall-through stores four-byte `0xfffff9b9` at `sp -0x24`
  (`str` at 0x3df70). Another branch takes `[[sp-28]+2]:1 != 0xff` to `0x3dfd4` (`bne` at 0x3dfc4);
  its fall-through assigns four-byte `0xfffff9b8` at `sp -0x24` (`str` at 0x3dfcc). Calls include
  unnamed `0xd75c0` and `0x40aa8` (`bl` at 0x3dfe4, `bl` at 0x3e004). Status polarity remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3df4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3df50 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3df54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3df58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3df6c | load | 4 | `abs 0x3e050` | `-` |
| `str` at 0x3df70 | store | 4 | `sp -0x24` | `0xfffff9b9` |
| `ldr` at 0x3df78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3df8c | load | 4 | `abs 0x3e054` | `-` |
| `ldr` at 0x3df90 | load | 4 | `abs 0x148d00` | `-` |
| `ldr` at 0x3df98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3dfac | load | 4 | `(unknown)` | `-` |
| `str` at 0x3dfb4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3dfb8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3dfbc | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x3dfc8 | load | 4 | `abs 0x3e058` | `-` |
| `str` at 0x3dfcc | store | 4 | `sp -0x24` | `0xfffff9b8` |
| `ldr` at 0x3dfd4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3dfd8 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x3dfdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3dfec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3dff0 | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x3dffc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3e000 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3e014 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3e024 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3e02c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3e03c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3e040 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3e044 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x3e048 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x3e04c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e04c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e04c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3df5c | `(unknown callee) 0x3cc70` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3dfe4 | `(unknown callee) 0xd75c0` | `[sp-20]` | `[[sp-28]+2]:1` | `?` | `[[sp-28]+2]:1` |
| `bl` at 0x3e004 | `(unknown callee) 0x40aa8` | `[sp-28]` | `?` | `ret(0xd75c0)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3df68 | `ret(0x3cc70) != 0x0` | `ret(0x3cc70) == 0x0` |
| `bne` at 0x3dfc4 | `[[sp-28]+2]:1 != 0xff` | `[[sp-28]+2]:1 == 0xff` |
| `beq` at 0x3e010 | `ret(0x40aa8) == 0x0` | `ret(0x40aa8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e04c | `r0` | `?` |

## `powerState`

- `powerState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e05c to 0x3e2b8.
  Established. The unnamed call to `0x400fc` has candidates `[sp-32]`, `ret(0x408ec)`, `0x0`,
  `sp-52` (`bl` at 0x3e1b0). Later branches take their arms for signed-positive `ret(0xfbb54)` and
  signed-negative `ret(0xfbb5c)` (`bgt` at 0x3e220, `blt` at 0x3e27c). Four-byte stores at `sp
  -0x3c` include `0x1` and `0x2` in separate arms (`str` at 0x3e288, `str` at 0x3e294). State labels
  and numerical comparison units are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e06c | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x3e070 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x3e074 | load | 4 | `abs 0x3e2b0` | `-` |
| `ldr` at 0x3e078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3e08c | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x3e094 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3e098 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3e09c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3e0b0 | load | 4 | `abs 0x3e2b4` | `-` |
| `str` at 0x3e0b4 | store | 4 | `sp -0x3c` | `0xfffff9b9` |
| `ldr` at 0x3e0bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3e0c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3e0d4 | load | 4 | `[sp-56] +0x0` | `-` |
| `str` at 0x3e0dc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3e0e0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3e0e4 | load | 1 | `[sp-44] +0x2` | `-` |
| `ldr` at 0x3e0f0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3e0f4 | load | 1 | `[sp-44] +0x2` | `-` |
| `ldr` at 0x3e0f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3e108 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3e10c | load | 1 | `[sp-44] +0x3` | `-` |
| `str` at 0x3e118 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3e11c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3e130 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3e140 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3e154 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3e168 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x3e170 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3e174 | load | 1 | `[sp-44] +0x4` | `-` |
| `str` at 0x3e184 | store | 4 | `sp -0x3c` | `0x2` |
| `ldr` at 0x3e18c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3e190 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3e1a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3e1b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3e1b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3e1c4 | store | 4 | `sp -0x30` | `ret(0x40754)` |
| `ldr` at 0x3e1c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3e1d8 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x3e1f4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3e1f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3e224 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3e234 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x3e250 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3e250 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3e288 | store | 4 | `sp -0x3c` | `0x1` |
| `str` at 0x3e294 | store | 4 | `sp -0x3c` | `0x2` |
| `ldr` at 0x3e298 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0x3e29c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3e29c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3e29c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3e29c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e29c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e29c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e0a0 | `(unknown callee) 0x3cc70` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x3e100 | `(unknown callee) 0xd75c0` | `[sp-32]` | `[[sp-44]+2]:1` | `?` | `[[sp-44]+2]:1` |
| `bl` at 0x3e120 | `(unknown callee) 0x40aa8` | `[sp-44]` | `?` | `ret(0xd75c0)` | `?` |
| `bl` at 0x3e144 | `(unknown callee) 0x40aa8` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x3e194 | `(unknown callee) 0x408ec` | `[sp-32]` | `[sp-44]` | `?` | `[[sp-44]+4]:1` |
| `bl` at 0x3e1b0 | `(unknown callee) 0x400fc` | `[sp-32]` | `ret(0x408ec)` | `0x0` | `sp-52` |
| `bl` at 0x3e1bc | `(unknown callee) 0x40754` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x3e1cc | `(unknown callee) 0xfb644` | `[sp-52]` | `?` | `?` | `ret(0x40754)` |
| `bl` at 0x3e1dc | `(unknown callee) 0xfb644` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x3e1f8 | `(unknown callee) 0xfb690` | `ret(0xfb644)` | `?` | `?` | `?` |
| `bl` at 0x3e214 | `(unknown callee) 0xfbb54` | `ret(0xfb644)` | `?` | `ret(0xfb690)` | `?` |
| `bl` at 0x3e228 | `(unknown callee) 0xfb644` | `[sp-52]` | `?` | `?` | `ret(0xfbb54)` |
| `bl` at 0x3e238 | `(unknown callee) 0xfb644` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x3e254 | `(unknown callee) 0xfb690` | `ret(0xfb644)` | `?` | `?` | `?` |
| `bl` at 0x3e270 | `(unknown callee) 0xfbb5c` | `ret(0xfb644)` | `?` | `ret(0xfb690)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3e0ac | `ret(0x3cc70) != 0x0` | `ret(0x3cc70) == 0x0` |
| `beq` at 0x3e0ec | `[[sp-44]+2]:1 == 0xff` | `[[sp-44]+2]:1 != 0xff` |
| `beq` at 0x3e12c | `ret(0x40aa8) == 0x0` | `ret(0x40aa8) != 0x0` |
| `bne` at 0x3e138 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x3e150 | `ret(0x40aa8) != 0x0` | `ret(0x40aa8) == 0x0` |
| `bne` at 0x3e15c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x3e17c | `[[sp-44]+4]:1 != 0xff` | `[[sp-44]+4]:1 == 0xff` |
| `bgt` at 0x3e220 | `ret(0xfbb54) > 0x0 (signed, 32-bit)` | `ret(0xfbb54) <= 0x0 (signed, 32-bit)` |
| `blt` at 0x3e27c | `ret(0xfbb5c) < 0x0 (signed, 32-bit)` | `ret(0xfbb5c) >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e29c | `r0` | `?` |

## `powerCycle`

- `powerCycle`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dc5c to 0x3dcc4.
  Established. The unnamed call to `0x3d890` carries `r0`, `r1`, `r2`, `r3` (`bl` at 0x3dc74); its
  zero result takes the branch to `0x3dc98` (`beq` at 0x3dc88). That arm includes unnamed `0x412fc`
  with `0xc8`, unknown, unknown, `ret(0x3d890)`, then `0x3d008` with `0x1`, `[sp-20]`, unknown,
  unknown (`bl` at 0x3dc9c, `bl` at 0x3dca8). The final return is unknown (`ldmdb` at 0x3dcc0). A
  delay unit or power-cycle effect is not established by these call targets.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3dc6c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3dc70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3dc7c | store | 4 | `sp -0x18` | `ret(0x3d890)` |
| `ldr` at 0x3dc80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3dc8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dc90 | store | 4 | `sp -0x1c` | `ret(0x3d890)` |
| `ldr` at 0x3dca4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3dcb0 | store | 4 | `sp -0x18` | `ret(0x3d008)` |
| `ldr` at 0x3dcb4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dcb8 | store | 4 | `sp -0x1c` | `ret(0x3d008)` |
| `ldr` at 0x3dcbc | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3dcc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dcc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dcc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3dc74 | `(unknown callee) 0x3d890` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3dc9c | `(unknown callee) 0x412fc` | `0xc8` | `?` | `?` | `ret(0x3d890)` |
| `bl` at 0x3dca8 | `(unknown callee) 0x3d008` | `0x1` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3dc88 | `ret(0x3d890) == 0x0` | `ret(0x3d890) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dcc0 | `r0` | `?` |

## `powerGetName`

- `powerGetName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x403f4 to 0x404e4.
  Established. Three four-byte loads have unresolved effective addresses (`ldr` at 0x40450, `ldr` at
  0x4047c, `ldr` at 0x404cc). The branch takes its arm for `? == 0x0` (`beq` at 0x40458). Both later
  four-byte stores at `sp -0x20` have unknown values (`str` at 0x40480, `str` at 0x404d0). The exit
  reports `[sp-32]` (`ldmdb` at 0x404d8). Name-table structure and string-pointer semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40404 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40408 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4040c | load | 4 | `abs 0x404dc` | `-` |
| `ldr` at 0x40410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40424 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x4042c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x40430 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40448 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x40450 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4045c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40474 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x4047c | load | 4 | `(unknown)` | `-` |
| `str` at 0x40480 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x40488 | load | 4 | `abs 0x404e0` | `-` |
| `ldr` at 0x4048c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x404a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x404ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x404cc | load | 4 | `(unknown)` | `-` |
| `str` at 0x404d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x404d4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x404d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x404d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x404d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x40458 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x404d8 | `r0` | `[sp-32]` |

## `powerGetReading`

- `powerGetReading`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x400fc to 0x403f4.
  Established. The unnamed call to `0xcd8e0` has candidates `[sp-28]`, `[ret(0x409a4)+4]:1`,
  `[ret(0x409a4)+5]:1`, `sp-46` (`bl` at 0x40174); its zero result takes the branch to `0x40198`
  (`beq` at 0x40188). The halfword store at `[sp-36] +0x0` retains `([sp-46]:2 & 0xffff)` (`strh` at
  0x401ac). A four-byte store at `[sp-60] +0x0` retains `ret(0xfbffc)` (`str` at 0x4022c). The
  conditional computed transfer remains unresolved, with predicate `? <= 0x5 (unsigned, 32-bit)`
  (`ldrls` at 0x40260). Selector-only conversion effects and reading units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4010c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x40110 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x40114 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x40118 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x4011c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40120 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4012c | store | 4 | `sp -0x2c` | `ret(0x409a4)` |
| `ldr` at 0x40130 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4013c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x40148 | store | 4 | `sp -0x38` | `ret(0x40a40)` |
| `ldr` at 0x40150 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x40154 | load | 1 | `ret(0x409a4) +0x4` | `-` |
| `ldr` at 0x40158 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x4015c | load | 1 | `ret(0x409a4) +0x5` | `-` |
| `ldr` at 0x40164 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4017c | store | 4 | `sp -0x34` | `ret(0xcd8e0)` |
| `ldr` at 0x40180 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4018c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x40190 | store | 4 | `sp -0x38` | `ret(0xcd8e0)` |
| `ldr` at 0x40198 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x401a4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x401a8 | load | 2 | `sp -0x2e` | `-` |
| `strh` at 0x401ac | store | 2 | `[sp-36] +0x0` | `([sp-46]:2 & 0xffff)` |
| `ldr` at 0x401b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x401bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x401c0 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldrsh` at 0x401c4 | load | 2 | `sp -0x2e` | `-` |
| `str` at 0x401d0 | store | 4 | `sp -0x40` | `ret(0xfbfb0)` |
| `ldrsh` at 0x401d4 | load | 2 | `sp -0x2e` | `-` |
| `ldr` at 0x401e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x401e4 | load | 4 | `abs 0x403e0` | `-` |
| `str` at 0x401f0 | store | 4 | `sp -0x40` | `ret(0xfbde8)` |
| `ldr` at 0x401f4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x401f8 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x401fc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x40200 | load | 1 | `[sp-44] +0x5` | `-` |
| `ldr` at 0x40204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40218 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x40228 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x4022c | store | 4 | `[sp-60] +0x0` | `ret(0xfbffc)` |
| `ldr` at 0x40230 | load | 4 | `abs 0x403e4` | `-` |
| `ldr` at 0x40234 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x40238 | load | 1 | `[sp-44] +0x0` | `-` |
| `ldrb` at 0x40258 | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x40260 | load | 4 | `(unknown)` | `-` |
| `str` at 0x403bc | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x403c0 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x403c4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x403c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x403c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x403c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x403c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40124 | `(unknown callee) 0x409a4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x40140 | `(unknown callee) 0x40a40` | `[sp-28]` | `?` | `?` | `ret(0x409a4)` |
| `bl` at 0x40174 | `(unknown callee) 0xcd8e0` | `[sp-28]` | `[ret(0x409a4)+4]:1` | `[ret(0x409a4)+5]:1` | `sp-46` |
| `bl` at 0x401cc | `(unknown callee) 0xfbfb0` | `?` | `?` | `[sp-40]` | `?` |
| `bl` at 0x401e8 | `(unknown callee) 0xfbde8` | `ret(0xfbfb0)` | `0x47800000` | `?` | `?` |
| `bl` at 0x40210 | `(unknown callee) 0xce2d0` | `[sp-28]` | `[[sp-44]+4]:1` | `[[sp-44]+5]:1` | `[[sp-44]+5]:1` |
| `bl` at 0x40220 | `(unknown callee) 0xfbffc` | `[sp-64]` | `ret(0xce2d0)` | `?` | `ret(0xce2d0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x40138 | `ret(0x409a4) != 0x0` | `ret(0x409a4) == 0x0` |
| `beq` at 0x40188 | `ret(0xcd8e0) == 0x0` | `ret(0xcd8e0) != 0x0` |
| `beq` at 0x401a0 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x401b8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bge` at 0x401dc | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `ldrls` at 0x40260 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x403c4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x40260).
