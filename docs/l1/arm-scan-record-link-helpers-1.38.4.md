# ARM scan records and link-reading helpers in 1.38.4

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
Computed transfers in two link-reading routines have unresolved destinations. Their tables and returns cover only known edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/scan-record-link-helpers-facts.txt) retain all tool sections.

## `idcode_fill`

- `idcode_fill`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe886c to 0xe8980.
  Established. The first non-stack word store writes `[sp-28]` at entry `r1 +0x0` (`str` at 0xe88b4). Later
  word stores write `[sp-32]`, `[sp-36]` and `[sp-40]` at `[sp-24] +0x0`, respectively (`str` at
  0xe88f8, `str` at 0xe893c, `str` at 0xe8970). The exit reports r0 `[[sp-24]]` (`ldmdb` at
  0xe8974). Calls and conditional branches are absent from the facts. IDCODE field meanings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe887c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe8880 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe8884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8894 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xe8898 | store | 4 | `sp -0x1c` | `[r1]` |
| `ldr` at 0xe88a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe88ac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe88b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe88b4 | store | 4 | `r1 +0x0` | `[sp-28]` |
| `ldr` at 0xe88b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe88bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe88cc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe88d0 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0xe88d4 | load | 4 | `abs 0xe8978` | `-` |
| `ldr` at 0xe88e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe88f0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe88f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe88f8 | store | 4 | `[sp-24] +0x0` | `[sp-32]` |
| `ldr` at 0xe88fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8900 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8910 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe8914 | store | 4 | `sp -0x24` | `[[sp-24]]` |
| `ldr` at 0xe8918 | load | 4 | `abs 0xe897c` | `-` |
| `ldr` at 0xe8924 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8934 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe8938 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe893c | store | 4 | `[sp-24] +0x0` | `[sp-36]` |
| `ldr` at 0xe8940 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe894c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe8950 | store | 4 | `sp -0x28` | `[[sp-24]]` |
| `ldr` at 0xe8958 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe8968 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe896c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe8970 | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `ldmdb` at 0xe8974 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8974 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8974 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8974 | `r0` | `[[sp-24]]` |

## `scanpktInit`

- `scanpktInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8980 to 0xe8b0c.
  Established. The global-state branch takes its work arm for `[0x2ad27c] == 0x0` and falls through
  on nonzero (`beq` at 0xe899c). The call to `0xb5654` has `0x2b398c`, `0x13603c`, `0x14`,
  `[0x2ad27c]+1` (`bl` at 0xe89e0). Later `0x223f0` receives `0x2b3988`, `0x136070`, `0xe8b0c`,
  `0x0` (`bl` at 0xe8a70). Queue ownership, task roles and r0 at exit are unresolved (`ldmdb` at
  0xe8adc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe8990 | load | 4 | `abs 0xe8ae0` | `-` |
| `ldr` at 0xe8994 | load | 4 | `abs 0x2ad27c` | `-` |
| `ldr` at 0xe89a0 | load | 4 | `abs 0xe8ae4` | `-` |
| `str` at 0xe89a4 | store | 4 | `sp -0x2c` | `0x135ff8` |
| `ldr` at 0xe89ac | load | 4 | `abs 0xe8ae8` | `-` |
| `ldr` at 0xe89c0 | load | 4 | `abs 0xe8ae0` | `-` |
| `ldr` at 0xe89c4 | load | 4 | `abs 0xe8ae0` | `-` |
| `ldr` at 0xe89c8 | load | 4 | `abs 0x2ad27c` | `-` |
| `str` at 0xe89d0 | store | 4 | `abs 0x2ad27c` | `[0x2ad27c]+1` |
| `ldr` at 0xe89d4 | load | 4 | `abs 0xe8aec` | `-` |
| `ldr` at 0xe89d8 | load | 4 | `abs 0xe8af0` | `-` |
| `str` at 0xe89e8 | store | 4 | `sp -0x14` | `ret(0xb5654)` |
| `ldr` at 0xe89ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe89f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8a04 | load | 4 | `abs 0xe8af4` | `-` |
| `str` at 0xe8a08 | store | 4 | `sp -0x2c` | `0x136048` |
| `str` at 0xe8a0c | store | 4 | `sp -0x28` | `ret(0x21674)` |
| `ldr` at 0xe8a14 | load | 4 | `abs 0xe8ae8` | `-` |
| `ldr` at 0xe8a28 | load | 4 | `abs 0xe8af8` | `-` |
| `str` at 0xe8a3c | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xe8a44 | store | 4 | `sp -0x28` | `0x800` |
| `str` at 0xe8a4c | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0xe8a54 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0xe8a5c | store | 4 | `sp -0x1c` | `0x4` |
| `ldr` at 0xe8a60 | load | 4 | `abs 0xe8afc` | `-` |
| `ldr` at 0xe8a64 | load | 4 | `abs 0xe8b00` | `-` |
| `ldr` at 0xe8a68 | load | 4 | `abs 0xe8b04` | `-` |
| `ldr` at 0xe8a84 | load | 4 | `abs 0xe8af8` | `-` |
| `ldr` at 0xe8a88 | load | 4 | `abs 0xe8af8` | `-` |
| `ldr` at 0xe8a8c | load | 4 | `abs 0x2b3980` | `-` |
| `ldr` at 0xe8a94 | load | 4 | `abs 0xe8aec` | `-` |
| `str` at 0xe8aa0 | store | 4 | `sp -0x14` | `ret(0xb4220)` |
| `ldr` at 0xe8aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8ab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8abc | load | 4 | `abs 0xe8b08` | `-` |
| `str` at 0xe8ac0 | store | 4 | `sp -0x2c` | `0x13607c` |
| `str` at 0xe8ac4 | store | 4 | `sp -0x28` | `ret(0x21674)` |
| `ldr` at 0xe8acc | load | 4 | `abs 0xe8ae8` | `-` |
| `ldmdb` at 0xe8adc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8adc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8adc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe89b8 | `(unknown callee) 0x21e28` | `0x4` | `0x136024` | `0x42` | `0x0` |
| `bl` at 0xe89e0 | `(unknown callee) 0xb5654` | `0x2b398c` | `0x13603c` | `0x14` | `[0x2ad27c]+1` |
| `bl` at 0xe89fc | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0xe8a20 | `(unknown callee) 0x21e28` | `0x4` | `0x136024` | `0x5a` | `0x0` |
| `bl` at 0xe8a34 | `(unknown callee) 0xb553c` | `0x2b3980` | `0xf` | `0x0` | `ret(0xb5654)` |
| `bl` at 0xe8a70 | `(unknown callee) 0x223f0` | `0x2b3988` | `0x136070` | `0xe8b0c` | `0x0` |
| `bl` at 0xe8a98 | `(unknown callee) 0xb4220` | `0x2b3980` | `[0x2b3980]` | `0x1` | `0x2b398c` |
| `bl` at 0xe8ab4 | `(unknown callee) 0x21674` | `ret(0xb4220)` | `?` | `?` | `ret(0xb4220)` |
| `bl` at 0xe8ad8 | `(unknown callee) 0x21e28` | `0x4` | `0x136024` | `0x74` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe899c | `[0x2ad27c] == 0x0` | `[0x2ad27c] != 0x0` |
| `beq` at 0xe89f4 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `beq` at 0xe8a7c | `ret(0x223f0) == 0x0` | `ret(0x223f0) != 0x0` |
| `beq` at 0xe8aac | `ret(0xb4220) == 0x0` | `ret(0xb4220) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8adc | `r0` | `?` |

## `lbi_tapmux_init`

- `lbi_tapmux_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf5668 to 0xf5790.
  Established. Word stores write `0x2` at `0x2ad450` and `ret(0x1b3c4)` at `0x2ad44c` (`str` at
  0xf56f8, `str` at 0xf5720). The call producing that result uses `[sp-36]`, `0x1`, `0x2ad450`,
  unknown (`bl` at 0xf5718). A later word store has unknown target and value (`str` at 0xf576c); its
  return supplies r0 `0x0` (`ldmdb` at 0xf5778). Mux identity and physical selection remain
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf5678 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf567c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xf5680 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf568c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf5690 | load | 4 | `abs 0xf577c` | `-` |
| `ldr` at 0xf5694 | load | 4 | `abs 0x2ad44c` | `-` |
| `ldr` at 0xf56a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf56ac | load | 4 | `abs 0xf5780` | `-` |
| `str` at 0xf56b0 | store | 4 | `sp -0x30` | `0x1398e8` |
| `ldr` at 0xf56b4 | load | 4 | `abs 0xf577c` | `-` |
| `ldr` at 0xf56b8 | load | 4 | `abs 0x2ad44c` | `-` |
| `str` at 0xf56bc | store | 4 | `sp -0x2c` | `[0x2ad44c]` |
| `ldr` at 0xf56c0 | load | 4 | `abs 0xf5784` | `-` |
| `ldr` at 0xf56c4 | load | 4 | `abs 0x2ad450` | `-` |
| `str` at 0xf56c8 | store | 4 | `sp -0x28` | `[0x2ad450]` |
| `ldr` at 0xf56d0 | load | 4 | `abs 0xf5788` | `-` |
| `ldr` at 0xf56e0 | load | 4 | `abs 0xf5784` | `-` |
| `ldr` at 0xf56e4 | load | 4 | `abs 0x2ad450` | `-` |
| `ldr` at 0xf56f0 | load | 4 | `abs 0xf5784` | `-` |
| `str` at 0xf56f8 | store | 4 | `abs 0x2ad450` | `0x2` |
| `ldr` at 0xf56fc | load | 4 | `abs 0xf5784` | `-` |
| `ldr` at 0xf5700 | load | 4 | `abs 0x2ad450` | `-` |
| `str` at 0xf5708 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf570c | load | 4 | `abs 0xf577c` | `-` |
| `ldr` at 0xf5710 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf5720 | store | 4 | `abs 0x2ad44c` | `ret(0x1b3c4)` |
| `ldr` at 0xf5724 | load | 4 | `abs 0x2ad44c` | `-` |
| `ldr` at 0xf5730 | load | 4 | `abs 0xf578c` | `-` |
| `str` at 0xf5734 | store | 4 | `sp -0x30` | `0x139938` |
| `ldr` at 0xf5738 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf573c | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xf5744 | load | 4 | `abs 0xf5788` | `-` |
| `ldr` at 0xf5754 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf575c | load | 4 | `abs 0xf577c` | `-` |
| `ldr` at 0xf5760 | load | 4 | `abs 0x2ad44c` | `-` |
| `ldr` at 0xf5768 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf576c | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xf5778 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf5778 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf5778 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf5778 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf56dc | `(unknown callee) 0x21e28` | `0x9` | `0x139920` | `0x52` | `0x0` |
| `bl` at 0xf5718 | `(unknown callee) 0x1b3c4` | `[sp-36]` | `0x1` | `0x2ad450` | `?` |
| `bl` at 0xf5750 | `(unknown callee) 0x21e28` | `0x9` | `0x139920` | `0x5e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf569c | `[0x2ad44c] == 0x0` | `[0x2ad44c] != 0x0` |
| `bne` at 0xf56a8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0xf56ec | `[0x2ad450] != 0x0` | `[0x2ad450] == 0x0` |
| `bne` at 0xf572c | `[0x2ad44c] != 0x0` | `[0x2ad44c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf5778 | `r0` | `0x0` |

## `lbi_jtag_verify`

- `lbi_jtag_verify`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf5830 to 0xf5bb4.
  Established. The call to `0xdcf54` carries `0x1`, `0x1`, `0x1`, `[sp-56]` (`bl` at 0xf5948); its
  nonzero result takes the continuation and zero falls through (`bne` at 0xf5954). Later tests take
  their equality arms for unknown values equal to `0x180005` and `0xa800ffea`, respectively, and
  fall through on inequality (`beq` at 0xf5b14, `beq` at 0xf5b58). The exit reports r0 `[sp-60]`
  (`ldmdb` at 0xf5b90). Hardware identity and verification policy are not established by the
  resolved values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf5840 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf5844 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf5848 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf584c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf5858 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xf5860 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xf5864 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf586c | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xf5874 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf5878 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf5890 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf589c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf58a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf58b8 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf58c4 | store | 4 | `sp -0x24` | `0x40` |
| `ldr` at 0xf58d4 | load | 4 | `abs 0xf5b94` | `-` |
| `str` at 0xf58d8 | store | 4 | `(unknown)` | `0xa800ffea` |
| `ldr` at 0xf58e8 | load | 4 | `abs 0xf5b98` | `-` |
| `str` at 0xf58ec | store | 4 | `(unknown)` | `0xabcdef01` |
| `str` at 0xf5900 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xf5914 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf5918 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf591c | store | 4 | `sp -0x4c` | `[sp-36]` |
| `str` at 0xf5924 | store | 4 | `sp -0x48` | `sp-44` |
| `str` at 0xf592c | store | 4 | `sp -0x44` | `sp-52` |
| `ldr` at 0xf5930 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf5934 | store | 4 | `sp -0x40` | `[sp-28]` |
| `ldr` at 0xf5944 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf595c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf597c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5980 | load | 4 | `abs 0xf5b9c` | `-` |
| `ldr` at 0xf5998 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf59a0 | load | 4 | `abs 0xf5b9c` | `-` |
| `str` at 0xf59ac | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf59c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf59cc | load | 4 | `abs 0xf5ba0` | `-` |
| `ldr` at 0xf59e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf59ec | load | 4 | `abs 0xf5ba0` | `-` |
| `str` at 0xf59f8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5a14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5a18 | load | 4 | `abs 0xf5ba4` | `-` |
| `ldr` at 0xf5a30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5a38 | load | 4 | `abs 0xf5ba4` | `-` |
| `str` at 0xf5a44 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5a60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5a7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5a90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5aac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5ac8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5adc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf5ae0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf5af0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5af4 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xf5b04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5b0c | load | 4 | `abs 0xf5ba8` | `-` |
| `ldr` at 0xf5b24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5b28 | load | 4 | `abs 0xf5bac` | `-` |
| `ldr` at 0xf5b2c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5b38 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf5b4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf5b50 | load | 4 | `abs 0xf5b94` | `-` |
| `ldr` at 0xf5b68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf5b6c | load | 4 | `abs 0xf5bb0` | `-` |
| `ldr` at 0xf5b70 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf5b7c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xf5b88 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xf5b8c | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xf5b90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf5b90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf5b90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf587c | `(unknown callee) 0xdc9a8` | `0x0` | `[sp-56]` | `[sp-28]` | `0x0` |
| `bl` at 0xf58a4 | `(unknown callee) 0xdca64` | `0x1` | `[sp-56]` | `[sp-28]` | `ret(0xdc9a8)` |
| `bl` at 0xf5948 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-56]` |
| `bl` at 0xf5b30 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x139978` | `?` | `?` |
| `bl` at 0xf5b74 | `(unknown callee) 0x1b49c` | `[sp-28]` | `0x1399a4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf5888 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xf58b0 | `ret(0xdca64) != 0x0` | `ret(0xdca64) == 0x0` |
| `bne` at 0xf5954 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xf5b14 | `? == 0x180005` | `? != 0x180005` |
| `beq` at 0xf5b58 | `? == 0xa800ffea` | `? != 0xa800ffea` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf5b90 | `r0` | `[sp-60]` |

## `JChip_read_dmin_dmax`

- `JChip_read_dmin_dmax`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf3720 to 0xf39c4.
  Established. The computed transfer takes its arm for an unknown unsigned value at most `0x5` and
  falls through for greater (`ldrls` at 0xf3778); its taken destination is unresolved, so the facts
  exclude those edges (`ldrls` at 0xf3778). The known-edge call reaches `0x23f2c` with `0x139700`,
  entry r0, entry r1, entry r2 (`bl` at 0xf3760). The reported known-edge return has r0 `0xfffff7c0`
  (`ldmdb` at 0xf39a8). Link timing reads and the other possible results remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf3730 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf3734 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf3738 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf373c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf3740 | load | 4 | `abs 0xf39ac` | `-` |
| `ldr` at 0xf3744 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xf3750 | load | 4 | `abs 0xf39b0` | `-` |
| `ldr` at 0xf3754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf3758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf375c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3764 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xf3778 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf3838 | load | 4 | `abs 0xf39b4` | `-` |
| `str` at 0xf383c | store | 4 | `sp -0x34` | `0xfffff7c0` |
| `ldr` at 0xf39a4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xf39a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf39a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf39a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf3760 | `(unknown callee) 0x23f2c` | `0x139700` | `r0` | `r1` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xf374c | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `ldrls` at 0xf3778 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf39a8 | `r0` | `0xfffff7c0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf3778).

## `JChip_read_link_state`

- `JChip_read_link_state`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf39c4 to 0xf3d60.
  Established. The selector transfer takes its arm for an unknown unsigned value at most `0x5` and
  falls through for greater (`ldrls` at 0xf3a1c); its computed destinations are unresolved and
  excluded from the fact sections (`ldrls` at 0xf3a1c). The known-edge call to `0x23f2c` has
  `0x139700`, entry r0, entry r1, entry r2 (`bl` at 0xf3a04). The known-edge exit reports r0
  `0xfffff7c0` (`ldmdb` at 0xf3d40). Other link-state paths and hardware effects are not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf39d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf39d8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf39dc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf39e0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf39e4 | load | 4 | `abs 0xf3d44` | `-` |
| `ldr` at 0xf39e8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xf39f4 | load | 4 | `abs 0xf3d48` | `-` |
| `ldr` at 0xf39f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf39fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3a00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3a08 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xf3a1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf3b20 | load | 4 | `abs 0xf3d4c` | `-` |
| `str` at 0xf3b24 | store | 4 | `sp -0x44` | `0xfffff7c0` |
| `ldr` at 0xf3d3c | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xf3d40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf3d40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf3d40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf3a04 | `(unknown callee) 0x23f2c` | `0x139700` | `r0` | `r1` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xf39f0 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `ldrls` at 0xf3a1c | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf3d40 | `r0` | `0xfffff7c0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf3a1c).

## `tdrfld_create`

- `tdrfld_create`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xefca4 to 0xefd1c.
  Established. The call to `0x1b124` carries `0xc`, entry r1, entry r2, entry r3 (`bl` at 0xefcbc).
  The subsequent call to `0xefd1c` has `ret(0x1b124)`, `[sp-20]`, unknown, `ret(0x1b124)` (`bl` at
  0xefce8); its nonzero result takes the continuation and zero falls through (`bne` at 0xefcf4).
  Field-record ownership and the joined r0 value remain unknown (`ldmdb` at 0xefd18).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xefcb4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xefcc4 | store | 4 | `sp -0x18` | `ret(0x1b124)` |
| `ldr` at 0xefcc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xefcd8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xefce0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefcf8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xefd04 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xefd0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xefd10 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xefd14 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xefd18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xefd18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xefd18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xefcbc | `(unknown callee) 0x1b124` | `0xc` | `r1` | `r2` | `r3` |
| `bl` at 0xefce8 | `(unknown callee) 0xefd1c` | `ret(0x1b124)` | `[sp-20]` | `?` | `ret(0x1b124)` |
| `bl` at 0xefcfc | `(unknown callee) 0x1b400` | `[sp-24]` | `?` | `?` | `ret(0xefd1c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xefcd0 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xefcf4 | `ret(0xefd1c) != 0x0` | `ret(0xefd1c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xefd18 | `r0` | `?` |

## `tdrfld_init`

- `tdrfld_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xefd1c to 0xefdac.
  Established. Entry r1 takes the bypass on zero and falls through on nonzero (`beq` at 0xefd3c).
  Later halfword stores clear `[sp-24] +0x4` and `[sp-24] +0x6`, and a byte store clears `[sp-24]
  +0x8` (`strh` at 0xefd80, `strh` at 0xefd8c, `strb` at 0xefd98). The call to `0x1afa4` has entry
  r1, entry r1, entry r2, entry r1 (`bl` at 0xefd48). Field layout meaning and the common r0 result
  are unproved (`ldmdb` at 0xefda8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xefd2c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xefd30 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xefd34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefd40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefd44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xefd4c | store | 4 | `r0 +0x0` | `ret(0x1afa4)` |
| `ldr` at 0xefd50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefd54 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xefd64 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xefd6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xefd74 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xefd78 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xefd80 | store | 2 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0xefd84 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xefd8c | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xefd90 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xefd98 | store | 1 | `[sp-24] +0x8` | `0x0` |
| `str` at 0xefda0 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xefda4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xefda8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xefda8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xefda8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xefda8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xefd48 | `(unknown callee) 0x1afa4` | `r1` | `r1` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xefd3c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xefd5c | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xefda8 | `r0` | `?` |

## `tdrdef_create`

- `tdrdef_create`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xefec0 to 0xeff40.
  Established. The call to `0xeff40` uses `ret(0x1b124)`, `[sp-20]`, `[sp-24]`, `ret(0x1b124)` (`bl`
  at 0xeff0c). A nonzero result takes its continuation and zero falls through (`bne` at 0xeff18).
  The latter arm includes a call to `0x1b400` with `[sp-28]`, unknown, unknown, `ret(0xeff40)` (`bl`
  at 0xeff20). Definition ownership and the merged return remain unidentified (`ldmdb` at 0xeff3c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xefed0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xefed4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xefee4 | store | 4 | `sp -0x1c` | `ret(0x1b124)` |
| `ldr` at 0xefee8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xefef8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xeff00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeff04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeff08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeff1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeff28 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xeff30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeff34 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xeff38 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xeff3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeff3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeff3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xefedc | `(unknown callee) 0x1b124` | `0xc` | `r1` | `r2` | `r3` |
| `bl` at 0xeff0c | `(unknown callee) 0xeff40` | `ret(0x1b124)` | `[sp-20]` | `[sp-24]` | `ret(0x1b124)` |
| `bl` at 0xeff20 | `(unknown callee) 0x1b400` | `[sp-28]` | `?` | `?` | `ret(0xeff40)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xefef0 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xeff18 | `ret(0xeff40) != 0x0` | `ret(0xeff40) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeff3c | `r0` | `?` |

## `tdrdef_init`

- `tdrdef_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeff40 to 0xf0054.
  Established. The call to `0x1b43c` uses `[[sp-24]+4]`, `0x0`, `[sp-36]`, `[sp-24]` (`bl` at
  0xeffb0). A later call to the same target changes the first candidate to `[[sp-24]+8]`, retaining
  `0x0`, `[sp-36]`, `[sp-24]` (`bl` at 0xf0034). The second result test takes its continuation for
  `[[sp-24]+8] != 0x0` and falls through on zero (`bne` at 0xf0008). Definition dimensions and final
  r0 are unresolved (`ldmdb` at 0xf0050).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeff50 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xeff54 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xeff58 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xeff5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeff70 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xeff74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeff78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeff84 | store | 4 | `r0 +0x4` | `ret(0x1b124)` |
| `ldr` at 0xeff88 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xeff98 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xeffa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeffa4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xeffac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeffb4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xeffb8 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xeffbc | store | 2 | `[sp-24] +0x2` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xeffc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeffd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeffe8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xeffec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefff0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xefffc | store | 4 | `[sp-24] +0x8` | `ret(0x1b124)` |
| `ldr` at 0xf0000 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf000c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0010 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xf001c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf0024 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0028 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf0030 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0038 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xf003c | load | 2 | `sp -0x1c` | `-` |
| `strh` at 0xf0040 | store | 2 | `[sp-24] +0x0` | `([sp-28]:2 & 0xffff)` |
| `str` at 0xf0048 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xf004c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xf0050 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf0050 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0050 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeff7c | `(unknown callee) 0x1b124` | `[sp-36]` | `r1` | `r2` | `?` |
| `bl` at 0xeffb0 | `(unknown callee) 0x1b43c` | `[[sp-24]+4]` | `0x0` | `[sp-36]` | `[sp-24]` |
| `bl` at 0xefff4 | `(unknown callee) 0x1b124` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xf0014 | `(unknown callee) 0x1b400` | `[[sp-24]+4]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf0034 | `(unknown callee) 0x1b43c` | `[[sp-24]+8]` | `0x0` | `[sp-36]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeff90 | `[r0+4] != 0x0` | `[r0+4] == 0x0` |
| `bne` at 0xf0008 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0050 | `r0` | `?` |

## `tdrdef_find_field`

- `tdrdef_find_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf072c to 0xf07c8.
  Established. The only call targets `0x1aff8` with four unknown candidates (`bl` at 0xf078c); its
  nonzero result takes the next branch and zero falls through (`bne` at 0xf0798). The loop test is
  resolved only as taken for `C == 0`, falling through otherwise (`blo` at 0xf075c). Matching rules
  and r0 at exit remain unknown (`ldmdb` at 0xf07c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf073c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0740 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0748 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf074c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xf0750 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0764 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0768 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf077c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0784 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0788 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf079c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf07a0 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xf07a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf07b0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf07b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf07bc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf07c0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xf07c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf07c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf07c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf078c | `(unknown callee) 0x1aff8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf075c | `C == 0` | `not (C == 0)` |
| `bne` at 0xf0798 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf07c4 | `r0` | `?` |

## `tdrdata_create`

- `tdrdata_create`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf07c8 to 0xf0858.
  Established. The call to `0xf0858` has `ret(0x1b124)`, `[sp-20]`, `[sp-22]:2`, `[sp-28]` (`bl` at
  0xf0824). The following result test takes its continuation on nonzero and falls through on zero
  (`bne` at 0xf0830). The zero arm calls `0x1b400` with `[sp-32]`, unknown, unknown, `ret(0xf0858)`
  (`bl` at 0xf0838). Record ownership and the common r0 result are not established (`ldmdb` at
  0xf0854).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf07d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf07e0 | store | 4 | `sp -0x1c` | `r2` |
| `strh` at 0xf07e4 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `str` at 0xf07f4 | store | 4 | `sp -0x20` | `ret(0x1b124)` |
| `ldr` at 0xf07f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0808 | store | 4 | `sp -0x24` | `0x0` |
| `ldrh` at 0xf0810 | load | 2 | `sp -0x16` | `-` |
| `ldr` at 0xf0814 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0820 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0834 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0840 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf0848 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf084c | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xf0850 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf0854 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0854 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf07ec | `(unknown callee) 0x1b124` | `0x14` | `r1` | `r2` | `r1` |
| `bl` at 0xf0824 | `(unknown callee) 0xf0858` | `ret(0x1b124)` | `[sp-20]` | `[sp-22]:2` | `[sp-28]` |
| `bl` at 0xf0838 | `(unknown callee) 0x1b400` | `[sp-32]` | `?` | `?` | `ret(0xf0858)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf0800 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xf0830 | `ret(0xf0858) != 0x0` | `ret(0xf0858) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0854 | `r0` | `?` |

## `tdrdata_init`

- `tdrdata_init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0858 to 0xf09f8.
  Established. The halfword store writes `([sp-30]:2 & 0xffff)` at `[sp-24] +0x4`, and the next
  halfword store writes `0x0` at `[sp-24] +0x6` (`strh` at 0xf08c4, `strh` at 0xf08d0). Word stores
  retain separate `ret(0x1b124)` results at `[sp-24] +0x10` and `[sp-24] +0xc` (`str` at 0xf0934,
  `str` at 0xf0984). A final call to `0xf13fc` uses `[sp-24]` and three unknown candidates (`bl` at
  0xf09e4). Array meanings and the joined r0 remain unknown (`ldmdb` at 0xf09f4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0868 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf086c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xf0870 | store | 4 | `sp -0x24` | `r3` |
| `strh` at 0xf0874 | store | 2 | `sp -0x1e` | `(r2 & 0xffff)` |
| `ldr` at 0xf0878 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0888 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf0890 | store | 4 | `r0 +0x0` | `ret(0x1afa4)` |
| `ldr` at 0xf0894 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0898 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf08a8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf08b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf08b8 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xf08bc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xf08c0 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xf08c4 | store | 2 | `[sp-24] +0x4` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0xf08c8 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xf08d0 | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xf08d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf08d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf08dc | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0xf08e0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf08e4 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xf0904 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf0908 | load | 2 | `[sp-36] +0x0` | `-` |
| `str` at 0xf0920 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf0924 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0928 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf0934 | store | 4 | `[sp-24] +0x10` | `ret(0x1b124)` |
| `ldr` at 0xf0938 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xf0944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0948 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xf0954 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0958 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf0964 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf096c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0970 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf0974 | load | 2 | `[sp-36] +0x2` | `-` |
| `str` at 0xf0984 | store | 4 | `[sp-24] +0xc` | `ret(0x1b124)` |
| `ldr` at 0xf0988 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf0994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0998 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xf09a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf09a8 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xf09b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf09b4 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0xf09c0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf09c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf09cc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf09d0 | load | 2 | `[sp-36] +0x2` | `-` |
| `ldr` at 0xf09d4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf09e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf09ec | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xf09f0 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xf09f4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf09f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf09f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf09f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf088c | `(unknown callee) 0x1afa4` | `r1` | `r1` | `r2` | `r1` |
| `bl` at 0xf092c | `(unknown callee) 0x1b124` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xf095c | `(unknown callee) 0x1b400` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf097c | `(unknown callee) 0x1b124` | `[[sp-36]+2]:2` | `?` | `?` | `[[sp-36]+2]:2` |
| `bl` at 0xf09ac | `(unknown callee) 0x1b400` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf09b8 | `(unknown callee) 0x1b400` | `[[sp-24]+16]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf09dc | `(unknown callee) 0x1b43c` | `[[sp-24]+12]` | `0x0` | `[[sp-36]+2]:2` | `[sp-24]` |
| `bl` at 0xf09e4 | `(unknown callee) 0xf13fc` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf0880 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xf08a0 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0xf0940 | `[[sp-24]+16] != 0x0` | `[[sp-24]+16] == 0x0` |
| `beq` at 0xf0950 | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |
| `bne` at 0xf0990 | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `beq` at 0xf09a0 | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf09f4 | `r0` | `?` |

## `tdrdata_add_overrides`

- `tdrdata_add_overrides`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1714 to 0xf1820.
  Established. The byte read has unknown effective address (`ldrb` at 0xf1764). The following guard
  takes its bypass when an unknown value equals `0x0` and falls through on nonzero (`beq` at
  0xf1770). The call to `0xe2174` has four unknown candidates (`bl` at 0xf1808). Override-bit format
  and the r0 result cannot be established here (`ldmdb` at 0xf181c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1724 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf1728 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf172c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf1734 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf1738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf173c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1740 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1744 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1758 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf175c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xf1764 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf1774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1778 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf177c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1790 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1798 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf179c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf17a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf17a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf17a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf17bc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf17c4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf17c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf17d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf17d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf17d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf17ec | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf17f4 | load | 2 | `(unknown)` | `-` |
| `str` at 0xf17f8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf17fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1804 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf180c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1814 | store | 4 | `sp -0x20` | `?` |
| `ldmdb` at 0xf181c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf181c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf181c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1808 | `(unknown callee) 0xe2174` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf174c | `C == 0` | `not (C == 0)` |
| `beq` at 0xf1770 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf181c | `r0` | `?` |

## `setup_tdrdata_lbi`

- `setup_tdrdata_lbi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7300 to 0xf7328.
  Established. The sole call reaches `0xf6ce4` with entry r0, entry r1, entry r2, entry r3 (`bl` at
  0xf7318); its result reaches the return as `ret(0xf6ce4)` (`ldmdb` at 0xf7324). No conditional
  branch appears in the facts. The wrapper alone does not establish data-register setup semantics.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf7310 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xf7314 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf7324 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf7324 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf7324 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf7318 | `(unknown callee) 0xf6ce4` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf7324 | `r0` | `ret(0xf6ce4)` |
