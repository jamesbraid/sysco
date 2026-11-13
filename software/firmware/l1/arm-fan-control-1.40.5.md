# ARM fan control callers in 1.40.5

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
The conditional computed transfer in `fanGetReading` remains unresolved (`ldrls` at 0x47268). Known-edge tables exclude effects reached only through its unknown destination. Raw undecodable bytes remain in the fact-sheet footer.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/fan-control-facts.txt) retain all tool sections.

## `fanCtrlInit`

- `fanCtrlInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x458a4 to 0x459cc.
  Established. A four-byte assignment writes `0x1` to absolute `0x148d7c` (`str` at 0x45928). The
  unnamed call to `0x1b454` has candidates `[sp-44]`, `0x1`, `0x148d7c`, unknown (`bl` at 0x45948);
  its result is stored at absolute `0x148d78` (`str` at 0x45950). Later stores at `[sp-36] +0x0` and
  `+0x4` retain an unknown four-byte value and byte `([sp-32]:1 & 0xff)` respectively (`str` at
  0x459a4, `strb` at 0x459b0). Fan-controller identities and allocation effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x458b4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x458b8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x458bc | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x458c0 | load | 4 | `abs 0x459b8` | `-` |
| `ldr` at 0x458c4 | load | 4 | `abs 0x148d78` | `-` |
| `ldr` at 0x458d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x458dc | load | 4 | `abs 0x459bc` | `-` |
| `str` at 0x458e0 | store | 4 | `sp -0x38` | `0x109d60` |
| `ldr` at 0x458e4 | load | 4 | `abs 0x459b8` | `-` |
| `ldr` at 0x458e8 | load | 4 | `abs 0x148d78` | `-` |
| `str` at 0x458ec | store | 4 | `sp -0x34` | `[0x148d78]` |
| `ldr` at 0x458f0 | load | 4 | `abs 0x459c0` | `-` |
| `ldr` at 0x458f4 | load | 4 | `abs 0x148d7c` | `-` |
| `str` at 0x458f8 | store | 4 | `sp -0x30` | `[0x148d7c]` |
| `ldr` at 0x45900 | load | 4 | `abs 0x459c4` | `-` |
| `ldr` at 0x45910 | load | 4 | `abs 0x459c0` | `-` |
| `ldr` at 0x45914 | load | 4 | `abs 0x148d7c` | `-` |
| `ldr` at 0x45920 | load | 4 | `abs 0x459c0` | `-` |
| `str` at 0x45928 | store | 4 | `abs 0x148d7c` | `0x1` |
| `ldr` at 0x4592c | load | 4 | `abs 0x459c0` | `-` |
| `ldr` at 0x45930 | load | 4 | `abs 0x148d7c` | `-` |
| `str` at 0x45938 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x4593c | load | 4 | `abs 0x459b8` | `-` |
| `ldr` at 0x45940 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x45950 | store | 4 | `abs 0x148d78` | `ret(0x1b454)` |
| `ldr` at 0x45954 | load | 4 | `abs 0x148d78` | `-` |
| `ldr` at 0x45960 | load | 4 | `abs 0x459c8` | `-` |
| `str` at 0x45964 | store | 4 | `sp -0x38` | `0x109d94` |
| `ldr` at 0x45968 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4596c | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x45974 | load | 4 | `abs 0x459c4` | `-` |
| `ldr` at 0x45984 | load | 4 | `abs 0x459b8` | `-` |
| `ldr` at 0x45988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45990 | load | 4 | `abs 0x148d78` | `-` |
| `str` at 0x45998 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4599c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x459a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x459a4 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x459a8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x459ac | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x459b0 | store | 1 | `[sp-36] +0x4` | `([sp-32]:1 & 0xff)` |
| `ldmdb` at 0x459b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x459b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x459b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x459b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4590c | `(unknown callee) 0x21eb8` | `0x9` | `0x109cc4` | `0x81` | `0x0` |
| `bl` at 0x45948 | `(unknown callee) 0x1b454` | `[sp-44]` | `0x1` | `0x148d7c` | `?` |
| `bl` at 0x45980 | `(unknown callee) 0x21eb8` | `0x9` | `0x109cc4` | `0x8d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x458cc | `[0x148d78] == 0x0` | `[0x148d78] != 0x0` |
| `bne` at 0x458d8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x4591c | `[0x148d7c] != 0x0` | `[0x148d7c] == 0x0` |
| `bne` at 0x4595c | `[0x148d78] != 0x0` | `[0x148d78] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x459b4 | `r0` | `?` |

## `fanTachInit`

- `fanTachInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45618 to 0x458a4.
  Established. The branch falls through when `[0x148d74] == 0x0` (`bne` at 0x45690), reaching a
  four-byte `0x1` store at absolute `0x148d74` (`str` at 0x4569c). Two unnamed calls to `0x1b454`
  produce stores at absolute `0x148d70` and `[sp-36] +0x8` (`bl` at 0x456c8, `bl` at 0x45754, `str`
  at 0x456d0, `str` at 0x4575c). The latter call has candidates `[[sp-36]+4]:1`, `0x1`, `[sp-36]`,
  `[[sp-36]+4]:1` (`bl` at 0x45754). The loop predicate retains unknown operands (`blt` at 0x4582c).
  Tachometer record roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45628 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x4562c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x45630 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x45634 | load | 4 | `abs 0x45884` | `-` |
| `ldr` at 0x45638 | load | 4 | `abs 0x148d70` | `-` |
| `ldr` at 0x45644 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45650 | load | 4 | `abs 0x45888` | `-` |
| `str` at 0x45654 | store | 4 | `sp -0x38` | `0x109c90` |
| `ldr` at 0x45658 | load | 4 | `abs 0x45884` | `-` |
| `ldr` at 0x4565c | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x45660 | store | 4 | `sp -0x34` | `[0x148d70]` |
| `ldr` at 0x45664 | load | 4 | `abs 0x4588c` | `-` |
| `ldr` at 0x45668 | load | 4 | `abs 0x148d74` | `-` |
| `str` at 0x4566c | store | 4 | `sp -0x30` | `[0x148d74]` |
| `ldr` at 0x45674 | load | 4 | `abs 0x45890` | `-` |
| `ldr` at 0x45684 | load | 4 | `abs 0x4588c` | `-` |
| `ldr` at 0x45688 | load | 4 | `abs 0x148d74` | `-` |
| `ldr` at 0x45694 | load | 4 | `abs 0x4588c` | `-` |
| `str` at 0x4569c | store | 4 | `abs 0x148d74` | `0x1` |
| `ldr` at 0x456a0 | load | 4 | `abs 0x4588c` | `-` |
| `ldr` at 0x456a4 | load | 4 | `abs 0x148d74` | `-` |
| `str` at 0x456b8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x456bc | load | 4 | `abs 0x45884` | `-` |
| `ldr` at 0x456c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x456d0 | store | 4 | `abs 0x148d70` | `ret(0x1b454)` |
| `ldr` at 0x456d4 | load | 4 | `abs 0x148d70` | `-` |
| `ldr` at 0x456e0 | load | 4 | `abs 0x45894` | `-` |
| `str` at 0x456e4 | store | 4 | `sp -0x38` | `0x109cd8` |
| `ldr` at 0x456e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x456ec | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x456f4 | load | 4 | `abs 0x45890` | `-` |
| `ldr` at 0x45704 | load | 4 | `abs 0x45884` | `-` |
| `ldr` at 0x45708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4571c | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x45724 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x45728 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4572c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45730 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x45734 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x45738 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x4573c | store | 1 | `[sp-36] +0x4` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0x45740 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45744 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x45748 | load | 1 | `[sp-36] +0x4` | `-` |
| `str` at 0x4575c | store | 4 | `[sp-36] +0x8` | `ret(0x1b454)` |
| `ldr` at 0x45760 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x4576c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45774 | load | 4 | `abs 0x45898` | `-` |
| `ldr` at 0x45778 | load | 4 | `abs 0x4589c` | `-` |
| `str` at 0x457a4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x457a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x457b0 | load | 4 | `abs 0x45898` | `-` |
| `ldr` at 0x457b4 | load | 4 | `abs 0x4589c` | `-` |
| `ldr` at 0x457c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x457d0 | load | 4 | `abs 0x458a0` | `-` |
| `str` at 0x457d4 | store | 4 | `(unknown)` | `0x109d20` |
| `str` at 0x457d8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x457dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x457e0 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x457e8 | load | 4 | `abs 0x45890` | `-` |
| `ldr` at 0x457f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x457fc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x45800 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x45804 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x45818 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4581c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x45820 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x45824 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x45834 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x45838 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4584c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x45854 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x45860 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x45864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45870 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x45878 | store | 4 | `sp -0x28` | `?` |
| `ldmdb` at 0x45880 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x45880 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45880 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45880 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45680 | `(unknown callee) 0x21eb8` | `0x9` | `0x109cc4` | `0x4f` | `0x0` |
| `bl` at 0x456c8 | `(unknown callee) 0x1b454` | `[sp-44]` | `0x1` | `[0x148d74]` | `?` |
| `bl` at 0x45700 | `(unknown callee) 0x21eb8` | `0x9` | `0x109cc4` | `0x5b` | `0x0` |
| `bl` at 0x45754 | `(unknown callee) 0x1b454` | `[[sp-36]+4]:1` | `0x1` | `[sp-36]` | `[[sp-36]+4]:1` |
| `bl` at 0x4577c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x45788 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x457b8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x457c8 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x457f4 | `(unknown callee) 0x21eb8` | `0x9` | `0x109cc4` | `0x66` | `0x0` |
| `bl` at 0x45810 | `(unknown callee) 0x1b4cc` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]:1` | `[[sp-36]+4]:1` |
| `bl` at 0x4586c | `(unknown callee) 0x459cc` | `[sp-24]` | `[sp-40]:1` | `?` | `[sp-40]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x45640 | `[0x148d70] == 0x0` | `[0x148d70] != 0x0` |
| `bne` at 0x4564c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x45690 | `[0x148d74] != 0x0` | `[0x148d74] == 0x0` |
| `bne` at 0x456dc | `[0x148d70] != 0x0` | `[0x148d70] == 0x0` |
| `bne` at 0x45768 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `blt` at 0x4582c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x4585c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45880 | `r0` | `?` |

## `fanTachNotPresent`

- `fanTachNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x459cc to 0x45a2c.
  Established. The unnamed call to `0x47650` carries `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl`
  at 0x459f4). A nonzero result takes the branch to a byte-zero assignment at `([sp-21]:1 +
  [ret(0x47650)+8]) +0x0` (`bne` at 0x45a08, `strb` at 0x45a24). The exit retains `ret(0x47650)`
  (`ldmdb` at 0x45a28). No absent-tachometer meaning is established by that stored zero.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x459dc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x459e4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x459e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x459ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x459fc | store | 4 | `sp -0x1c` | `ret(0x47650)` |
| `ldr` at 0x45a00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45a10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45a14 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45a18 | load | 4 | `ret(0x47650) +0x8` | `-` |
| `strb` at 0x45a24 | store | 1 | `([sp-21]:1 + [ret(0x47650)+8]) +0x0` | `0x0` |
| `ldmdb` at 0x45a28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45a28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45a28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x459f4 | `(unknown callee) 0x47650` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x45a08 | `ret(0x47650) != 0x0` | `ret(0x47650) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45a28 | `r0` | `ret(0x47650)` |

## `fanTachPresent`

- `fanTachPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45a2c to 0x45a8c.
  Established. The visible unnamed call to `0x47650` has candidates `r0`, `(r1 & 0xff)`, `r2`, `(r1
  & 0xff)` (`bl` at 0x45a54); its nonzero result takes the store arm (`bne` at 0x45a68). The
  one-byte store writes `0x0` at `([sp-21]:1 + [ret(0x47650)+8]) +0x0` (`strb` at 0x45a84). The
  return carries `ret(0x47650)` (`ldmdb` at 0x45a88). The routine's name does not establish a
  presence flag value.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45a3c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x45a44 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x45a48 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45a4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45a5c | store | 4 | `sp -0x1c` | `ret(0x47650)` |
| `ldr` at 0x45a60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45a70 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45a74 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45a78 | load | 4 | `ret(0x47650) +0x8` | `-` |
| `strb` at 0x45a84 | store | 1 | `([sp-21]:1 + [ret(0x47650)+8]) +0x0` | `0x0` |
| `ldmdb` at 0x45a88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45a88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45a88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45a54 | `(unknown callee) 0x47650` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x45a68 | `ret(0x47650) != 0x0` | `ret(0x47650) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45a88 | `r0` | `ret(0x47650)` |

## `fanTachIsPresent`

- `fanTachIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45a8c to 0x45af8.
  Established. The unnamed call to `0x47650` has the candidate prefix `r0`, `(r1 & 0xff)`, `r2`,
  `(r1 & 0xff)` (`bl` at 0x45ab4); its nonzero result takes a byte-reading arm (`bne` at 0x45ac8).
  That arm reads `([sp-21]:1 + [ret(0x47650)+8]) +0x0` and stores the resulting byte expression as
  four bytes at `sp -0x20` (`ldrb` at 0x45ae8, `str` at 0x45aec). The other arm stores four-byte
  zero there (`str` at 0x45ad0). The merged return is unknown (`ldmdb` at 0x45af4). Presence
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45a9c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x45aa4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x45aa8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45aac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45abc | store | 4 | `sp -0x1c` | `ret(0x47650)` |
| `ldr` at 0x45ac0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45ad0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x45ad8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45adc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45ae0 | load | 4 | `ret(0x47650) +0x8` | `-` |
| `ldrb` at 0x45ae8 | load | 1 | `([sp-21]:1 + [ret(0x47650)+8]) +0x0` | `-` |
| `str` at 0x45aec | store | 4 | `sp -0x20` | `[([sp-21]:1 + [ret(0x47650)+8])]:1` |
| `ldr` at 0x45af0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x45af4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45af4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45af4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45ab4 | `(unknown callee) 0x47650` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x45ac8 | `ret(0x47650) != 0x0` | `ret(0x47650) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45af4 | `r0` | `?` |

## `fanGetTachCfg`

- `fanGetTachCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46174 to 0x461e0.
  Established. The four-byte load reads absolute `0x148d70` (`ldr` at 0x461a4). A zero `r1` takes
  the branch past the optional store (`beq` at 0x461b8). On the fall-through, four-byte `[[sp-28]]`
  is stored at `r1 +0x0` (`str` at 0x461c8). A later byte load at `[sp-28] +0x4` supplies return
  expression `[[sp-28]+4]:1` (`ldrb` at 0x461d0, `ldmdb` at 0x461d8). Configuration-record meaning
  and a count unit are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46184 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x46188 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4618c | load | 4 | `abs 0x461dc` | `-` |
| `ldr` at 0x46190 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x461a4 | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x461ac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x461b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x461bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x461c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x461c4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x461c8 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x461cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x461d0 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0x461d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x461d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x461d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x461b8 | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x461d8 | `r0` | `[[sp-28]+4]:1` |

## `fanReset`

- `fanReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45af8 to 0x46174.
  Established. The first unnamed call to `0xcd15c` has candidates `r0`, `sp-76`, unknown, `sp-76`
  (`bl` at 0x45b54). A later unnamed call to `0xd73b0` carries `[sp-28]`, `[[sp-32]+3]:1`,
  `[[sp-32]+5]:1`, `[sp-37]:1` (`bl` at 0x45c7c); its zero result takes the branch to the iteration
  tail (`beq` at 0x45c90). Other visible unnamed calls include `0xda618` with `[sp-28]`,
  `[sp-60]:1`, `[sp-64]:1`, `[[sp-32]+4]:1`, and `0xda6d0` with `[sp-28]`, `[sp-80]:1`, `[sp-84]:1`,
  `[sp-88]` (`bl` at 0x45e10, `bl` at 0x4612c). The known exit reports zero (`ldmdb` at 0x46148).
  Fan-reset operations and byte-field meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45b08 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x45b0c | load | 4 | `abs 0x4614c` | `-` |
| `ldr` at 0x45b10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45b24 | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x45b2c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x45b30 | load | 4 | `abs 0x46150` | `-` |
| `ldr` at 0x45b34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45b3c | load | 4 | `abs 0x148d78` | `-` |
| `str` at 0x45b44 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x45b4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45b5c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x45b60 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0x45b64 | load | 1 | `[sp-56] +0x4` | `-` |
| `ldr` at 0x45b68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45b78 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x45b7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45b90 | load | 4 | `[sp-56] +0x0` | `-` |
| `str` at 0x45b98 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x45b9c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45ba0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x45ba4 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x45ba8 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x45bb4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x45bc0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x45bd0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x45be0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45be4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45bf4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45bf8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45c38 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45c3c | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x45c44 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45c4c | load | 1 | `[sp-32] +0x5` | `-` |
| `strb` at 0x45c54 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x45c58 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45c5c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45c60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45c64 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0x45c68 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x45c6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45c84 | store | 4 | `sp -0x2c` | `ret(0xd73b0)` |
| `ldr` at 0x45c88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45c94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45c9c | load | 4 | `abs 0x46154` | `-` |
| `ldr` at 0x45ca0 | load | 4 | `abs 0x46158` | `-` |
| `str` at 0x45ccc | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x45cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45cd8 | load | 4 | `abs 0x46154` | `-` |
| `ldr` at 0x45cdc | load | 4 | `abs 0x46158` | `-` |
| `ldr` at 0x45ce8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x45cf8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45cfc | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45d00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45d10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45d1c | load | 4 | `abs 0x4615c` | `-` |
| `str` at 0x45d20 | store | 4 | `(unknown)` | `0x109dd0` |
| `str` at 0x45d24 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x45d28 | store | 4 | `(unknown)` | `ret(0xd7eec)` |
| `str` at 0x45d2c | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x45d34 | load | 4 | `abs 0x46160` | `-` |
| `ldr` at 0x45d48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45d4c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45d50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45d6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45d70 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45d84 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45d8c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45d98 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45d9c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45db0 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45db8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x45dbc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x45dc0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45dc4 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45dd8 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45de0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x45de4 | store | 4 | `sp -0x40` | `?` |
| `ldrb` at 0x45de8 | load | 1 | `sp -0x3c` | `-` |
| `ldrb` at 0x45dec | load | 1 | `sp -0x40` | `-` |
| `ldr` at 0x45df0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45df4 | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0x45dfc | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x45e00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45e14 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45e18 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45e1c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45e20 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x45e24 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45e28 | load | 1 | `[sp-32] +0x7` | `-` |
| `ldr` at 0x45e2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x45e38 | store | 4 | `sp -0x2c` | `ret(0xcde10)` |
| `ldr` at 0x45e3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45e48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45e50 | load | 4 | `abs 0x46154` | `-` |
| `ldr` at 0x45e54 | load | 4 | `abs 0x46158` | `-` |
| `str` at 0x45e80 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x45e84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45e8c | load | 4 | `abs 0x46154` | `-` |
| `ldr` at 0x45e90 | load | 4 | `abs 0x46158` | `-` |
| `ldr` at 0x45e9c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x45eac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45eb0 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x45eb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45ec4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x45ed0 | load | 4 | `abs 0x46164` | `-` |
| `str` at 0x45ed4 | store | 4 | `(unknown)` | `0x109e04` |
| `str` at 0x45ed8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0x45edc | store | 4 | `(unknown)` | `ret(0xce8f8)` |
| `str` at 0x45ee0 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0x45ee8 | load | 4 | `abs 0x46160` | `-` |
| `ldr` at 0x45efc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45f04 | load | 4 | `abs 0x46168` | `-` |
| `ldr` at 0x45f08 | load | 4 | `abs 0x4616c` | `-` |
| `str` at 0x45f34 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x45f38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45f40 | load | 4 | `abs 0x46168` | `-` |
| `ldr` at 0x45f44 | load | 4 | `abs 0x4616c` | `-` |
| `ldr` at 0x45f50 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x45f60 | load | 4 | `abs 0x46170` | `-` |
| `str` at 0x45f64 | store | 4 | `(unknown)` | `0x109e3c` |
| `str` at 0x45f68 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x45f6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x45f70 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x45f74 | store | 4 | `(unknown)` | `[[sp-32]]:1` |
| `ldr` at 0x45f7c | load | 4 | `abs 0x46160` | `-` |
| `ldr` at 0x45f8c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x45f94 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x45fa0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x45fa4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x45fa8 | load | 1 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x45fac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45fbc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x45fc0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x45fd4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x45fdc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x45ff0 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x45ff8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x46008 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4600c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46020 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x46028 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4603c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x46044 | load | 1 | `(unknown)` | `-` |
| `str` at 0x46048 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x4604c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46050 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46064 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x4606c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x46080 | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0x46088 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4608c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x46090 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46094 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x460a8 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x460b0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x460b4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x460b8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x460bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x460d0 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x460d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x460ec | load | 4 | `sp -0x4c` | `-` |
| `ldrh` at 0x460f4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x460f8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x46108 | store | 4 | `sp -0x5c` | `?` |
| `ldrb` at 0x4610c | load | 1 | `sp -0x50` | `-` |
| `ldrb` at 0x46110 | load | 1 | `sp -0x54` | `-` |
| `ldr` at 0x46114 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x46118 | store | 4 | `(unknown)` | `[sp-92]` |
| `ldr` at 0x4611c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46128 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x46130 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x46138 | store | 4 | `sp -0x30` | `?` |
| `ldmdb` at 0x46148 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x46148 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x46148 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46148 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45b54 | `(unknown callee) 0xcd15c` | `r0` | `sp-76` | `?` | `sp-76` |
| `bl` at 0x45c04 | `(unknown callee) 0xd72bc` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x45c1c | `(unknown callee) 0x1f26c` | `0x4` | `?` | `?` | `ret(0xd72bc)` |
| `bl` at 0x45c30 | `(unknown callee) 0x1f26c` | `0x5` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x45c7c | `(unknown callee) 0xd73b0` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+5]:1` | `[sp-37]:1` |
| `bl` at 0x45ca4 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x45cb0 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x45ce0 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x45cf0 | `(unknown callee) 0x1b598` | `[sp-80]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x45d08 | `(unknown callee) 0xd7eec` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x45d14 | `(unknown callee) 0x21a38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x45d40 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0xec` | `0x0` |
| `bl` at 0x45d58 | `(unknown callee) 0xcce28` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x45e10 | `(unknown callee) 0xda618` | `[sp-28]` | `[sp-60]:1` | `[sp-64]:1` | `[[sp-32]+4]:1` |
| `bl` at 0x45e30 | `(unknown callee) 0xcde10` | `[sp-28]` | `[[sp-32]+3]:1` | `[[sp-32]+4]:1` | `[[sp-32]+7]:1` |
| `bl` at 0x45e58 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x45e64 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x45e94 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x45ea4 | `(unknown callee) 0x1b598` | `[sp-80]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x45ebc | `(unknown callee) 0xce8f8` | `[sp-28]` | `[[sp-32]+3]:1` | `?` | `[[sp-32]+3]:1` |
| `bl` at 0x45ec8 | `(unknown callee) 0x21a38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x45ef4 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x104` | `0x0` |
| `bl` at 0x45f0c | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x109e30` | `0x109e38` |
| `bl` at 0x45f18 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x45f48 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x109e30` | `0x109e38` |
| `bl` at 0x45f58 | `(unknown callee) 0x1b598` | `[sp-80]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x45f88 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x110` | `0x0` |
| `bl` at 0x4612c | `(unknown callee) 0xda6d0` | `[sp-28]` | `[sp-80]:1` | `[sp-84]:1` | `[sp-88]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x45b70 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x45bb0 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x45bbc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x45bc8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x45bd8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x45bec | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x45c10 | `ret(0xd72bc) != 0x0` | `ret(0xd72bc) == 0x0` |
| `beq` at 0x45c28 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x45c90 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `bne` at 0x45d64 | `ret(0xcce28) != 0x0` | `ret(0xcce28) == 0x0` |
| `bne` at 0x45d94 | `? != 0x6` | `? == 0x6` |
| `beq` at 0x45e44 | `ret(0xcde10) == 0x0` | `ret(0xcde10) != 0x0` |
| `blt` at 0x45fb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x46000 | `? == 0x6` | `? != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46148 | `r0` | `0x0` |

## `fanOn`

- `fanOn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x461e0 to 0x465bc.
  Established. The first unnamed call to `0x47898` has candidates `r0`, `(r1 & 0xff)`, `sp-29`,
  `sp-30` (`bl` at 0x46238); its zero result takes the branch to `0x4625c` (`beq` at 0x4624c). Later
  calls include `0xd7368` with `[sp-20]`, `[ret(0x47534)+1]:1`, `[ret(0x47534)+2]:1`,
  `[ret(0x47534)+2]:1`, and `0xcde10` with `[sp-20]`, `[ret(0x47534)+3]:1`, `[ret(0x47534)+4]:1`,
  `[ret(0x47534)+7]:1` (`bl` at 0x46338, `bl` at 0x46414). A four-byte store places `0xfffffa86` at
  `sp -0x24` (`str` at 0x46568). The merged return is unknown (`ldmdb` at 0x46588). Fan activation
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x461f0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x461f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x461fc | load | 4 | `abs 0x4658c` | `-` |
| `ldr` at 0x46200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46214 | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x4621c | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x46220 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4622c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46240 | store | 4 | `sp -0x24` | `ret(0x47898)` |
| `ldr` at 0x46244 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46250 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46254 | store | 4 | `sp -0x34` | `ret(0x47898)` |
| `ldrb` at 0x4625c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x46260 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x46264 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46268 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x46278 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x4627c | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46280 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46288 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x46298 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x4629c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x462ac | load | 1 | `ret(0x47408) +0x0` | `-` |
| `ldr` at 0x462b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x462c0 | store | 4 | `sp -0x1c` | `ret(0x47534)` |
| `ldr` at 0x462c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x462c8 | load | 1 | `ret(0x47534) +0x0` | `-` |
| `str` at 0x462cc | store | 4 | `sp -0x38` | `[ret(0x47534)]:1` |
| `ldr` at 0x462d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x462dc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x462e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x462f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x46308 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4630c | load | 1 | `ret(0x47534) +0x1` | `-` |
| `ldr` at 0x4631c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46320 | load | 1 | `ret(0x47534) +0x1` | `-` |
| `ldr` at 0x46324 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46328 | load | 1 | `ret(0x47534) +0x2` | `-` |
| `ldr` at 0x4632c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46340 | store | 4 | `sp -0x28` | `ret(0xd7368)` |
| `ldr` at 0x46344 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46350 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46358 | load | 4 | `abs 0x46590` | `-` |
| `ldr` at 0x4635c | load | 4 | `abs 0x46594` | `-` |
| `str` at 0x46388 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4638c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46394 | load | 4 | `abs 0x46590` | `-` |
| `ldr` at 0x46398 | load | 4 | `abs 0x46594` | `-` |
| `ldr` at 0x463a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x463b4 | load | 4 | `abs 0x46598` | `-` |
| `str` at 0x463b8 | store | 4 | `(unknown)` | `0x109e60` |
| `str` at 0x463bc | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x463c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x463c4 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x463c8 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x463d0 | load | 4 | `abs 0x4659c` | `-` |
| `ldr` at 0x463d4 | load | 4 | `abs 0x465a0` | `-` |
| `ldr` at 0x463e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x463ec | load | 4 | `sp -0x28` | `-` |
| `str` at 0x463f0 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x463f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x463fc | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x46400 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46404 | load | 1 | `ret(0x47534) +0x4` | `-` |
| `ldr` at 0x46408 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4640c | load | 1 | `ret(0x47534) +0x7` | `-` |
| `ldr` at 0x46410 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4641c | store | 4 | `sp -0x28` | `ret(0xcde10)` |
| `ldr` at 0x46420 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4642c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46434 | load | 4 | `abs 0x46590` | `-` |
| `ldr` at 0x46438 | load | 4 | `abs 0x46594` | `-` |
| `str` at 0x46464 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46470 | load | 4 | `abs 0x46590` | `-` |
| `ldr` at 0x46474 | load | 4 | `abs 0x46594` | `-` |
| `ldr` at 0x46480 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46490 | load | 4 | `abs 0x46598` | `-` |
| `str` at 0x46494 | store | 4 | `(unknown)` | `0x109e60` |
| `str` at 0x46498 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x4649c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x464a0 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x464a4 | store | 4 | `(unknown)` | `[[sp-28]+4]:1` |
| `ldr` at 0x464ac | load | 4 | `abs 0x4659c` | `-` |
| `ldr` at 0x464b0 | load | 4 | `abs 0x465a4` | `-` |
| `ldr` at 0x464bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x464c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x464cc | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x464d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x464dc | load | 4 | `abs 0x465a8` | `-` |
| `ldr` at 0x464e0 | load | 4 | `abs 0x465ac` | `-` |
| `str` at 0x4650c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46518 | load | 4 | `abs 0x465a8` | `-` |
| `ldr` at 0x4651c | load | 4 | `abs 0x465ac` | `-` |
| `ldr` at 0x46528 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46538 | load | 4 | `abs 0x465b0` | `-` |
| `str` at 0x4653c | store | 4 | `(unknown)` | `0x109e84` |
| `str` at 0x46540 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x46544 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46548 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x4654c | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x46554 | load | 4 | `abs 0x4659c` | `-` |
| `ldr` at 0x46558 | load | 4 | `abs 0x465b4` | `-` |
| `ldr` at 0x46564 | load | 4 | `abs 0x465b8` | `-` |
| `str` at 0x46568 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x4656c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x46574 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x4657c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46580 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x46584 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x46588 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46588 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46588 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46238 | `(unknown callee) 0x47898` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x462a4 | `(unknown callee) 0x47408` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x462b8 | `(unknown callee) 0x47534` | `[sp-20]` | `[ret(0x47408)]:1` | `?` | `[ret(0x47408)]:1` |
| `bl` at 0x46338 | `(unknown callee) 0xd7368` | `[sp-20]` | `[ret(0x47534)+1]:1` | `[ret(0x47534)+2]:1` | `[ret(0x47534)+2]:1` |
| `bl` at 0x46360 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x4636c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x4639c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x463ac | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x463dc | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x16d` | `0x0` |
| `bl` at 0x46414 | `(unknown callee) 0xcde10` | `[sp-20]` | `[ret(0x47534)+3]:1` | `[ret(0x47534)+4]:1` | `[ret(0x47534)+7]:1` |
| `bl` at 0x4643c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x46448 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46478 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x46488 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x464b8 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x179` | `0x0` |
| `bl` at 0x464e4 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109e30` | `0x109e38` |
| `bl` at 0x464f0 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46520 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109e30` | `0x109e38` |
| `bl` at 0x46530 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46560 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x186` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4624c | `ret(0x47898) == 0x0` | `ret(0x47898) != 0x0` |
| `bls` at 0x46270 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x46290 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x462d8 | `[ret(0x47534)]:1 == 0x2` | `[ret(0x47534)]:1 != 0x2` |
| `bgt` at 0x462e4 | `[ret(0x47534)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47534)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x462f0 | `[ret(0x47534)]:1 == 0x1` | `[ret(0x47534)]:1 != 0x1` |
| `beq` at 0x46300 | `[ret(0x47534)]:1 == 0x63` | `[ret(0x47534)]:1 != 0x63` |
| `bne` at 0x46314 | `[ret(0x47534)+1]:1 != 0xff` | `[ret(0x47534)+1]:1 == 0xff` |
| `beq` at 0x4634c | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `bne` at 0x463e8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x46428 | `ret(0xcde10) == 0x0` | `ret(0xcde10) != 0x0` |
| `bne` at 0x464c4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46588 | `r0` | `?` |

## `fanOff`

- `fanOff`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x465bc to 0x46990.
  Established. The call to unnamed `0x47898` carries `r0`, `(r1 & 0xff)`, `sp-29`, `sp-30` (`bl` at
  0x46614). The branch takes its zero-result arm to `0x46638` (`beq` at 0x46628). Included later
  calls are `0xd731c` with `[sp-20]`, `[ret(0x47534)+1]:1`, `[ret(0x47534)+2]:1`,
  `[ret(0x47534)+2]:1`, and `0xcde10` with `[sp-20]`, `[ret(0x47534)+3]:1`, `[ret(0x47534)+4]:1`,
  `0x0` (`bl` at 0x46714, `bl` at 0x467f4). Four bytes of `0xfffffa86` are assigned at `sp -0x24`
  (`str` at 0x46948). Hardware shutdown semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x465cc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x465d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x465d8 | load | 4 | `abs 0x4696c` | `-` |
| `ldr` at 0x465dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x465f0 | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x465f8 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x465fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46608 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4661c | store | 4 | `sp -0x24` | `ret(0x47898)` |
| `ldr` at 0x46620 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4662c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46630 | store | 4 | `sp -0x34` | `ret(0x47898)` |
| `ldrb` at 0x46638 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x4663c | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x46640 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46644 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x46654 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46658 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x4665c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46664 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x46674 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46678 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46688 | load | 1 | `ret(0x47408) +0x0` | `-` |
| `ldr` at 0x4668c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4669c | store | 4 | `sp -0x1c` | `ret(0x47534)` |
| `ldr` at 0x466a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x466a4 | load | 1 | `ret(0x47534) +0x0` | `-` |
| `str` at 0x466a8 | store | 4 | `sp -0x38` | `[ret(0x47534)]:1` |
| `ldr` at 0x466ac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x466b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x466c4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x466d4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x466e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x466e8 | load | 1 | `ret(0x47534) +0x1` | `-` |
| `ldr` at 0x466f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x466fc | load | 1 | `ret(0x47534) +0x1` | `-` |
| `ldr` at 0x46700 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46704 | load | 1 | `ret(0x47534) +0x2` | `-` |
| `ldr` at 0x46708 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4671c | store | 4 | `sp -0x28` | `ret(0xd731c)` |
| `ldr` at 0x46720 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4672c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46734 | load | 4 | `abs 0x46970` | `-` |
| `ldr` at 0x46738 | load | 4 | `abs 0x46974` | `-` |
| `str` at 0x46764 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46770 | load | 4 | `abs 0x46970` | `-` |
| `ldr` at 0x46774 | load | 4 | `abs 0x46974` | `-` |
| `ldr` at 0x46780 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46790 | load | 4 | `abs 0x46978` | `-` |
| `str` at 0x46794 | store | 4 | `(unknown)` | `0x109ea8` |
| `str` at 0x46798 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x4679c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x467a0 | load | 1 | `[sp-28] +0x1` | `-` |
| `str` at 0x467a4 | store | 4 | `(unknown)` | `[[sp-28]+1]:1` |
| `ldr` at 0x467ac | load | 4 | `abs 0x4697c` | `-` |
| `ldr` at 0x467b0 | load | 4 | `abs 0x46980` | `-` |
| `ldr` at 0x467bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x467c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x467cc | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x467d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x467d8 | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x467dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x467e0 | load | 1 | `ret(0x47534) +0x4` | `-` |
| `ldr` at 0x467e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x467fc | store | 4 | `sp -0x28` | `ret(0xcde10)` |
| `ldr` at 0x46800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4680c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46814 | load | 4 | `abs 0x46970` | `-` |
| `ldr` at 0x46818 | load | 4 | `abs 0x46974` | `-` |
| `str` at 0x46844 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46850 | load | 4 | `abs 0x46970` | `-` |
| `ldr` at 0x46854 | load | 4 | `abs 0x46974` | `-` |
| `ldr` at 0x46860 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46870 | load | 4 | `abs 0x46978` | `-` |
| `str` at 0x46874 | store | 4 | `(unknown)` | `0x109ea8` |
| `str` at 0x46878 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x4687c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46880 | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x46884 | store | 4 | `(unknown)` | `[[sp-28]+3]:1` |
| `ldr` at 0x4688c | load | 4 | `abs 0x4697c` | `-` |
| `ldr` at 0x46890 | load | 4 | `abs 0x46984` | `-` |
| `ldr` at 0x4689c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x468a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x468ac | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x468b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x468bc | load | 4 | `abs 0x46970` | `-` |
| `ldr` at 0x468c0 | load | 4 | `abs 0x46974` | `-` |
| `str` at 0x468ec | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x468f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x468f8 | load | 4 | `abs 0x46970` | `-` |
| `ldr` at 0x468fc | load | 4 | `abs 0x46974` | `-` |
| `ldr` at 0x46908 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46918 | load | 4 | `abs 0x46988` | `-` |
| `str` at 0x4691c | store | 4 | `(unknown)` | `0x109ecc` |
| `str` at 0x46920 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x46924 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46928 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x4692c | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x46934 | load | 4 | `abs 0x4697c` | `-` |
| `ldr` at 0x46944 | load | 4 | `abs 0x4698c` | `-` |
| `str` at 0x46948 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x4694c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x46954 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x4695c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46960 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x46964 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x46968 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46968 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46968 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46614 | `(unknown callee) 0x47898` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x46680 | `(unknown callee) 0x47408` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x46694 | `(unknown callee) 0x47534` | `[sp-20]` | `[ret(0x47408)]:1` | `?` | `[ret(0x47408)]:1` |
| `bl` at 0x46714 | `(unknown callee) 0xd731c` | `[sp-20]` | `[ret(0x47534)+1]:1` | `[ret(0x47534)+2]:1` | `[ret(0x47534)+2]:1` |
| `bl` at 0x4673c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x46748 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46778 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x46788 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x467b8 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x1af` | `0x0` |
| `bl` at 0x467f4 | `(unknown callee) 0xcde10` | `[sp-20]` | `[ret(0x47534)+3]:1` | `[ret(0x47534)+4]:1` | `0x0` |
| `bl` at 0x4681c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x46828 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46858 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x46868 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46898 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x1bb` | `0x0` |
| `bl` at 0x468c4 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x468d0 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46900 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x109d14` | `0x109d1c` |
| `bl` at 0x46910 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x46940 | `(unknown callee) 0x21eb8` | `0x1` | `0x109cc4` | `0x1c8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46628 | `ret(0x47898) == 0x0` | `ret(0x47898) != 0x0` |
| `bls` at 0x4664c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x4666c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x466b4 | `[ret(0x47534)]:1 == 0x2` | `[ret(0x47534)]:1 != 0x2` |
| `bgt` at 0x466c0 | `[ret(0x47534)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47534)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x466cc | `[ret(0x47534)]:1 == 0x1` | `[ret(0x47534)]:1 != 0x1` |
| `beq` at 0x466dc | `[ret(0x47534)]:1 == 0x63` | `[ret(0x47534)]:1 != 0x63` |
| `bne` at 0x466f0 | `[ret(0x47534)+1]:1 != 0xff` | `[ret(0x47534)+1]:1 == 0xff` |
| `beq` at 0x46728 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |
| `bne` at 0x467c4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x46808 | `ret(0xcde10) == 0x0` | `ret(0xcde10) != 0x0` |
| `bne` at 0x468a4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46968 | `r0` | `?` |

## `fanStatus`

- `fanStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46990 to 0x46bc0.
  Established. An unnamed call to `0x3dd74` precedes a branch taken when its result is nonzero (`bl`
  at 0x469d4, `bne` at 0x469e0). The zero-result fall-through stores four-byte `0x0` at `sp -0x30`
  (`str` at 0x469e8). A later call to `0xce0b0` has candidates `[sp-20]`, `[ret(0x47534)+3]:1`,
  `[ret(0x47534)+4]:1`, `sp-37` (`bl` at 0x46b54). The caller then reads one byte from `sp -0x25`
  (`ldrb` at 0x46b58). A fan-status interpretation of that byte is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x469a0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x469a8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x469ac | load | 4 | `abs 0x46bbc` | `-` |
| `ldr` at 0x469b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x469c4 | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x469cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x469d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x469e8 | store | 4 | `sp -0x30` | `0x0` |
| `ldrb` at 0x469f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x469fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46a10 | store | 4 | `sp -0x24` | `ret(0x47898)` |
| `ldr` at 0x46a14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46a20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46a24 | store | 4 | `sp -0x30` | `ret(0x47898)` |
| `ldrb` at 0x46a2c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x46a30 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x46a34 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46a38 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x46a48 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46a4c | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46a50 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46a58 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x46a68 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46a6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46a7c | load | 1 | `ret(0x47408) +0x0` | `-` |
| `ldr` at 0x46a80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46a90 | store | 4 | `sp -0x1c` | `ret(0x47534)` |
| `ldr` at 0x46a94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46a98 | load | 1 | `ret(0x47534) +0x0` | `-` |
| `str` at 0x46a9c | store | 4 | `sp -0x34` | `[ret(0x47534)]:1` |
| `ldr` at 0x46aa0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46aac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46ab8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46ac8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46ad8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46adc | load | 1 | `ret(0x47534) +0x1` | `-` |
| `str` at 0x46aec | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x46af4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46af8 | load | 1 | `ret(0x47534) +0x1` | `-` |
| `ldr` at 0x46afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46b0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46b10 | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0x46b28 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x46b30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46b34 | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x46b38 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46b3c | load | 1 | `ret(0x47534) +0x4` | `-` |
| `ldr` at 0x46b44 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46b58 | load | 1 | `sp -0x25` | `-` |
| `str` at 0x46b68 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x46b74 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x46b80 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x46b84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46b90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46b94 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldrb` at 0x46b9c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x46ba4 | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x46bac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46bb0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46bb4 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x46bb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46bb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46bb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x469d4 | `(unknown callee) 0x3dd74` | `r0` | `0x148d70` | `?` | `?` |
| `bl` at 0x46a08 | `(unknown callee) 0x47898` | `[sp-20]` | `[sp-21]:1` | `sp-29` | `sp-30` |
| `bl` at 0x46a74 | `(unknown callee) 0x47408` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x46a88 | `(unknown callee) 0x47534` | `[sp-20]` | `[ret(0x47408)]:1` | `?` | `[ret(0x47408)]:1` |
| `bl` at 0x46b04 | `(unknown callee) 0xd75c0` | `[sp-20]` | `[ret(0x47534)+1]:1` | `?` | `[ret(0x47534)+1]:1` |
| `bl` at 0x46b54 | `(unknown callee) 0xce0b0` | `[sp-20]` | `[ret(0x47534)+3]:1` | `[ret(0x47534)+4]:1` | `sp-37` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x469e0 | `ret(0x3dd74) != 0x0` | `ret(0x3dd74) == 0x0` |
| `beq` at 0x46a1c | `ret(0x47898) == 0x0` | `ret(0x47898) != 0x0` |
| `bls` at 0x46a40 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x46a60 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x46aa8 | `[ret(0x47534)]:1 == 0x2` | `[ret(0x47534)]:1 != 0x2` |
| `bgt` at 0x46ab4 | `[ret(0x47534)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47534)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x46ac0 | `[ret(0x47534)]:1 == 0x1` | `[ret(0x47534)]:1 != 0x1` |
| `beq` at 0x46ad0 | `[ret(0x47534)]:1 == 0x63` | `[ret(0x47534)]:1 != 0x63` |
| `bne` at 0x46ae4 | `[ret(0x47534)+1]:1 != 0xff` | `[ret(0x47534)+1]:1 == 0xff` |
| `beq` at 0x46b8c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46bb8 | `r0` | `?` |

## `fanSetSpeedRaw`

- `fanSetSpeedRaw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46bc0 to 0x46e60.
  Established. The branch taken when `[ret(0x47534)+3]:1 != 0xff` reaches `0x46d1c` (`bne` at
  0x46d0c); its fall-through writes four-byte `0xfffffa87` at `sp -0x30` (`str` at 0x46d14). A byte
  store at `sp -0x20` has unknown value (`strb` at 0x46d24). Calls include unnamed `0xd73b0` with
  `[sp-20]`, `[[sp-28]+3]:1`, `[[sp-28]+5]:1`, `[sp-32]:1`, and `0xcde10` with `[sp-20]`,
  `[ret(0x47534)+3]:1`, `[ret(0x47534)+4]:1`, `[sp-22]:1` (`bl` at 0x46d8c, `bl` at 0x46de0). Speed
  units and the physical output are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46bd0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x46bd8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x46be0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x46be4 | load | 4 | `abs 0x46e54` | `-` |
| `ldr` at 0x46be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46bfc | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x46c04 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x46c08 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46c14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46c28 | store | 4 | `sp -0x24` | `ret(0x47898)` |
| `ldr` at 0x46c2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46c38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46c3c | store | 4 | `sp -0x30` | `ret(0x47898)` |
| `ldrb` at 0x46c44 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x46c48 | store | 1 | `sp -0x1f` | `([sp-29]:1 & 0xff)` |
| `ldrb` at 0x46c4c | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46c50 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x46c60 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46c64 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46c68 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46c70 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x46c7c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46c80 | load | 1 | `sp -0x1f` | `-` |
| `ldrb` at 0x46c84 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0x46c90 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46c94 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46ca4 | load | 1 | `ret(0x47408) +0x0` | `-` |
| `ldr` at 0x46ca8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46cb8 | store | 4 | `sp -0x1c` | `ret(0x47534)` |
| `ldr` at 0x46cbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46cc0 | load | 1 | `ret(0x47534) +0x0` | `-` |
| `str` at 0x46cc4 | store | 4 | `sp -0x34` | `[ret(0x47534)]:1` |
| `ldr` at 0x46cc8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46cd4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46ce0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46cf0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46d00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46d04 | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x46d10 | load | 4 | `abs 0x46e58` | `-` |
| `str` at 0x46d14 | store | 4 | `sp -0x30` | `0xfffffa87` |
| `ldrb` at 0x46d1c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x46d24 | store | 1 | `sp -0x20` | `?` |
| `ldrb` at 0x46d28 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x46d44 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46d48 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46d4c | load | 1 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x46d54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46d5c | load | 1 | `[sp-28] +0x5` | `-` |
| `strb` at 0x46d64 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x46d68 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46d6c | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0x46d70 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46d74 | load | 1 | `[sp-28] +0x5` | `-` |
| `ldrb` at 0x46d78 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46d7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46d94 | store | 4 | `sp -0x28` | `ret(0xd73b0)` |
| `ldr` at 0x46d98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46da4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46db0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x46db4 | store | 4 | `sp -0x24` | `ret(0xd73b0)` |
| `ldr` at 0x46dbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46dc0 | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x46dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46dc8 | load | 1 | `ret(0x47534) +0x4` | `-` |
| `ldrb` at 0x46dcc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x46dd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46de8 | store | 4 | `sp -0x28` | `ret(0xcde10)` |
| `ldr` at 0x46dec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x46df8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46e04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x46e08 | store | 4 | `sp -0x24` | `ret(0xcde10)` |
| `ldrb` at 0x46e0c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x46e2c | load | 4 | `abs 0x46e5c` | `-` |
| `str` at 0x46e30 | store | 4 | `sp -0x24` | `0xfffffa86` |
| `ldrb` at 0x46e34 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x46e3c | store | 1 | `sp -0x1f` | `?` |
| `ldr` at 0x46e44 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46e48 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x46e4c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x46e50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46e50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46e50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46c20 | `(unknown callee) 0x47898` | `r0` | `(r1 & 0xff)` | `sp-29` | `sp-30` |
| `bl` at 0x46c9c | `(unknown callee) 0x47408` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x46cb0 | `(unknown callee) 0x47534` | `[sp-20]` | `[ret(0x47408)]:1` | `?` | `[ret(0x47408)]:1` |
| `bl` at 0x46d34 | `(unknown callee) 0x1d41c` | `0x5` | `[sp-32]:1` | `?` | `[sp-32]:1` |
| `bl` at 0x46d40 | `(unknown callee) 0x1d41c` | `0x4` | `0x1` | `?` | `?` |
| `bl` at 0x46d8c | `(unknown callee) 0xd73b0` | `[sp-20]` | `[[sp-28]+3]:1` | `[[sp-28]+5]:1` | `[sp-32]:1` |
| `bl` at 0x46de0 | `(unknown callee) 0xcde10` | `[sp-20]` | `[ret(0x47534)+3]:1` | `[ret(0x47534)+4]:1` | `[sp-22]:1` |
| `bl` at 0x46e18 | `(unknown callee) 0x1d41c` | `0x5` | `[sp-22]:1` | `?` | `[sp-22]:1` |
| `bl` at 0x46e24 | `(unknown callee) 0x1d41c` | `0x4` | `0x1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46c34 | `ret(0x47898) == 0x0` | `ret(0x47898) != 0x0` |
| `bls` at 0x46c58 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x46c78 | `Z == 1` | `not (Z == 1)` |
| `bhs` at 0x46c8c | `C == 1` | `not (C == 1)` |
| `beq` at 0x46cd0 | `[ret(0x47534)]:1 == 0x2` | `[ret(0x47534)]:1 != 0x2` |
| `bgt` at 0x46cdc | `[ret(0x47534)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47534)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x46ce8 | `[ret(0x47534)]:1 == 0x1` | `[ret(0x47534)]:1 != 0x1` |
| `beq` at 0x46cf8 | `[ret(0x47534)]:1 == 0x63` | `[ret(0x47534)]:1 != 0x63` |
| `bne` at 0x46d0c | `[ret(0x47534)+3]:1 != 0xff` | `[ret(0x47534)+3]:1 == 0xff` |
| `beq` at 0x46da0 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `bne` at 0x46dac | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x46df4 | `ret(0xcde10) == 0x0` | `ret(0xcde10) != 0x0` |
| `bne` at 0x46e00 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46e50 | `r0` | `?` |

## `fanSetSpeed`

- `fanSetSpeed`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46e60 to 0x47068.
  Established. Three byte stores to `sp -0x1d` retain `([ret(0x47534)+7]:1 & 0xff)`,
  `([ret(0x47534)+8]:1 & 0xff)` and `([ret(0x47534)+9]:1 & 0xff)` in separate arms (`strb` at
  0x46fd4, `strb` at 0x46fe4, `strb` at 0x46ff4). Their deciding branches are taken for `[sp-22]:1
  == 0x0`, `== 0x1` and `== 0x2`, respectively (`beq` at 0x46fb4, `beq` at 0x46f9c, `beq` at
  0x46fc4). The unnamed call to `0x46bc0` has candidates `[sp-20]`, `[sp-32]:1`, unknown,
  `[sp-32]:1` (`bl` at 0x47014); its zero-result branch goes to `0x47040` (`beq` at 0x47028). The
  meanings of the selected bytes are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46e70 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x46e78 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x46e80 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x46e84 | load | 4 | `abs 0x47060` | `-` |
| `ldr` at 0x46e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46e9c | load | 4 | `abs 0x148d70` | `-` |
| `str` at 0x46ea4 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x46ea8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46eb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46ec8 | store | 4 | `sp -0x24` | `ret(0x47898)` |
| `ldr` at 0x46ecc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x46ed8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x46edc | store | 4 | `sp -0x30` | `ret(0x47898)` |
| `ldrb` at 0x46ee4 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x46ee8 | store | 1 | `sp -0x20` | `([sp-30]:1 & 0xff)` |
| `ldrb` at 0x46eec | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x46ef0 | load | 1 | `sp -0x1f` | `-` |
| `ldr` at 0x46f00 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46f04 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x46f08 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x46f14 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x46f18 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46f1c | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrb` at 0x46f24 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x46f34 | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x46f38 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x46f48 | load | 1 | `ret(0x47408) +0x0` | `-` |
| `ldr` at 0x46f4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46f5c | store | 4 | `sp -0x1c` | `ret(0x47534)` |
| `ldr` at 0x46f60 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46f64 | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldrb` at 0x46f70 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46f80 | load | 4 | `abs 0x47064` | `-` |
| `str` at 0x46f84 | store | 4 | `sp -0x30` | `0xfffffa87` |
| `ldrb` at 0x46f8c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x46f90 | store | 4 | `sp -0x34` | `[sp-22]:1` |
| `ldr` at 0x46f94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46fa0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46fac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46fbc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x46fcc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46fd0 | load | 1 | `ret(0x47534) +0x7` | `-` |
| `strb` at 0x46fd4 | store | 1 | `sp -0x1d` | `([ret(0x47534)+7]:1 & 0xff)` |
| `ldr` at 0x46fdc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46fe0 | load | 1 | `ret(0x47534) +0x8` | `-` |
| `strb` at 0x46fe4 | store | 1 | `sp -0x1d` | `([ret(0x47534)+8]:1 & 0xff)` |
| `ldr` at 0x46fec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46ff0 | load | 1 | `ret(0x47534) +0x9` | `-` |
| `strb` at 0x46ff4 | store | 1 | `sp -0x1d` | `([ret(0x47534)+9]:1 & 0xff)` |
| `ldrb` at 0x46ffc | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x47000 | store | 1 | `sp -0x1d` | `([sp-22]:1 & 0xff)` |
| `ldrb` at 0x47004 | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x47008 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4700c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4701c | store | 4 | `sp -0x28` | `ret(0x46bc0)` |
| `ldr` at 0x47020 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4702c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x47038 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4703c | store | 4 | `sp -0x24` | `ret(0x46bc0)` |
| `ldrb` at 0x47040 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x47048 | store | 1 | `sp -0x20` | `?` |
| `ldr` at 0x47050 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x47054 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x47058 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x4705c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4705c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4705c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46ec0 | `(unknown callee) 0x47898` | `r0` | `(r1 & 0xff)` | `sp-30` | `sp-31` |
| `bl` at 0x46f40 | `(unknown callee) 0x47408` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x46f54 | `(unknown callee) 0x47534` | `[sp-20]` | `[ret(0x47408)]:1` | `?` | `[ret(0x47408)]:1` |
| `bl` at 0x47014 | `(unknown callee) 0x46bc0` | `[sp-20]` | `[sp-32]:1` | `?` | `[sp-32]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x46ed4 | `ret(0x47898) == 0x0` | `ret(0x47898) != 0x0` |
| `bls` at 0x46ef8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bhs` at 0x46f10 | `C == 1` | `not (C == 1)` |
| `bne` at 0x46f2c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x46f6c | `[ret(0x47534)+3]:1 != 0xff` | `[ret(0x47534)+3]:1 == 0xff` |
| `bne` at 0x46f78 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `beq` at 0x46f9c | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0x46fa8 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x46fb4 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0x46fc4 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `beq` at 0x47028 | `ret(0x46bc0) == 0x0` | `ret(0x46bc0) != 0x0` |
| `bne` at 0x47034 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4705c | `r0` | `?` |

## `fanGetSpeed`

- `fanGetSpeed`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47068 to 0x471f4.
  Established. The unnamed call to `0x45a8c` is followed by a branch taken for a nonzero result
  (`bl` at 0x470ac, `bne` at 0x470b8). The fall-through stores four-byte `0xfffffa85` at `sp -0x2c`
  (`str` at 0x470c0). Later explicit byte stores at `[sp-28] +0x0` have unknown value in one arm and
  `0x0` in another (`strb` at 0x47184, `strb` at 0x471c4). Another arm calls unnamed `0xce0b0` with
  `[sp-20]`, `[ret(0x47534)+3]:1`, `[ret(0x47534)+4]:1`, `[sp-28]` (`bl` at 0x471ac). No speed unit
  is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47078 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47080 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x47084 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x47088 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4708c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4709c | store | 4 | `sp -0x20` | `ret(0x47408)` |
| `ldrb` at 0x470a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x470a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x470bc | load | 4 | `abs 0x471e8` | `-` |
| `str` at 0x470c0 | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x470c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x470cc | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x470d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x470e0 | store | 4 | `sp -0x24` | `ret(0x47534)` |
| `ldr` at 0x470e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x470e8 | load | 1 | `ret(0x47534) +0x0` | `-` |
| `str` at 0x470ec | store | 4 | `sp -0x30` | `[ret(0x47534)]:1` |
| `ldr` at 0x470f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x470fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x47108 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x47118 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x47128 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4712c | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x47138 | load | 4 | `abs 0x471ec` | `-` |
| `str` at 0x4713c | store | 4 | `sp -0x2c` | `0xfffffa87` |
| `ldr` at 0x47144 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x47148 | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x4714c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x4715c | store | 1 | `sp -0x25` | `(ret(0xd75c0) & 0xff)` |
| `ldr` at 0x47160 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x47164 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x4716c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x47170 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x47178 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4717c | load | 1 | `[sp-36] +0x4` | `-` |
| `strb` at 0x47184 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x4718c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x47190 | load | 1 | `ret(0x47534) +0x3` | `-` |
| `ldr` at 0x47194 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x47198 | load | 1 | `ret(0x47534) +0x4` | `-` |
| `ldr` at 0x4719c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x471a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x471b4 | store | 4 | `sp -0x2c` | `ret(0xce0b0)` |
| `ldr` at 0x471bc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x471c4 | store | 1 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x471cc | load | 4 | `abs 0x471f0` | `-` |
| `str` at 0x471d0 | store | 4 | `sp -0x2c` | `0xfffffa86` |
| `str` at 0x471dc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x471e0 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x471e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x471e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x471e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47094 | `(unknown callee) 0x47408` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x470ac | `(unknown callee) 0x45a8c` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x470d8 | `(unknown callee) 0x47534` | `[sp-20]` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0x47154 | `(unknown callee) 0xd75c0` | `[sp-20]` | `[ret(0x47534)+3]:1` | `?` | `[ret(0x47534)+3]:1` |
| `bl` at 0x471ac | `(unknown callee) 0xce0b0` | `[sp-20]` | `[ret(0x47534)+3]:1` | `[ret(0x47534)+4]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x470b8 | `ret(0x45a8c) != 0x0` | `ret(0x45a8c) == 0x0` |
| `beq` at 0x470f8 | `[ret(0x47534)]:1 == 0x2` | `[ret(0x47534)]:1 != 0x2` |
| `bgt` at 0x47104 | `[ret(0x47534)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47534)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x47110 | `[ret(0x47534)]:1 == 0x1` | `[ret(0x47534)]:1 != 0x1` |
| `beq` at 0x47120 | `[ret(0x47534)]:1 == 0x63` | `[ret(0x47534)]:1 != 0x63` |
| `bne` at 0x47134 | `[ret(0x47534)+3]:1 != 0xff` | `[ret(0x47534)+3]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x471e4 | `r0` | `?` |

## `fanGetReading`

- `fanGetReading`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x471f4 to 0x473a0.
  Established. An unnamed call to `0x45a8c` precedes the branch taken for its nonzero result (`bl`
  at 0x47238, `bne` at 0x47244). The conditional computed transfer has an unknown destination and is
  taken when `? <= 0x7 (unsigned, 32-bit)` (`ldrls` at 0x47268). Known fall-through effects
  include the call to `0xcdcc8` with `[sp-20]`, `[[sp-40]+1]:1`, `[[sp-40]+2]:1`, `[sp-28]` (`bl` at
  0x472dc). A signed-negative `[[sp-28]]` takes the branch to `0x4731c` (`blt` at 0x47304). The
  four-byte store at `[sp-48] +0x0` retains `ret(0xfc430)` (`str` at 0x47378). Conversion semantics
  and reading units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47204 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4720c | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x47210 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x47214 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x47218 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x47228 | store | 4 | `sp -0x28` | `ret(0x47408)` |
| `ldrb` at 0x4722c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x47230 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47248 | load | 4 | `abs 0x4738c` | `-` |
| `str` at 0x4724c | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x47254 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x47258 | load | 1 | `[sp-40] +0x8` | `-` |
| `ldrls` at 0x47268 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4729c | load | 4 | `abs 0x47394` | `-` |
| `str` at 0x472a0 | store | 4 | `sp -0x24` | `0x3f800000` |
| `ldr` at 0x472bc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x472c0 | load | 1 | `[sp-40] +0x1` | `-` |
| `ldr` at 0x472c4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x472c8 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x472cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x472d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x472e4 | store | 4 | `sp -0x20` | `ret(0xcdcc8)` |
| `ldr` at 0x472e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x472ec | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x472f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x472f4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x472f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x472fc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x47308 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4730c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x47314 | store | 4 | `sp -0x38` | `ret(0xfbfb0)` |
| `ldr` at 0x4731c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x47320 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x47328 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4732c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x47340 | store | 4 | `sp -0x38` | `ret(0xfbfb0)` |
| `ldr` at 0x47344 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x47348 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x47354 | store | 4 | `sp -0x38` | `ret(0xfbde8)` |
| `ldr` at 0x47358 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4735c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x47374 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x47378 | store | 4 | `[sp-48] +0x0` | `ret(0xfc430)` |
| `ldr` at 0x4737c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47380 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x47384 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x47388 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47388 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47388 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47220 | `(unknown callee) 0x47408` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x47238 | `(unknown callee) 0x45a8c` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x472dc | `(unknown callee) 0xcdcc8` | `[sp-20]` | `[[sp-40]+1]:1` | `[[sp-40]+2]:1` | `[sp-28]` |
| `bl` at 0x47310 | `(unknown callee) 0xfbfb0` | `[[sp-28]]` | `[sp-28]` | `[sp-28]` | `[[sp-28]]` |
| `bl` at 0x4733c | `(unknown callee) 0xfbfb0` | `?` | `[sp-28]` | `?` | `?` |
| `bl` at 0x4734c | `(unknown callee) 0xfbde8` | `ret(0xfbfb0)` | `ret(0xfbfb0)` | `?` | `?` |
| `bl` at 0x47360 | `(unknown callee) 0xfc18c` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x4736c | `(unknown callee) 0xfc430` | `ret(0xfc18c)` | `?` | `?` | `ret(0xfc18c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47244 | `ret(0x45a8c) != 0x0` | `ret(0x45a8c) == 0x0` |
| `ldrls` at 0x47268 | `? <= 0x7 (unsigned, 32-bit)` | `? > 0x7 (unsigned, 32-bit)` |
| `blt` at 0x47304 | `[[sp-28]] < 0x0 (signed, 32-bit)` | `[[sp-28]] >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47388 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x47268).

## `fanLocation`

- `fanLocation`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x473a0 to 0x47408.
  Established. Following the unnamed call to `0x47408`, two four-byte loads read `ret(0x47408) +0x4`
  (`bl` at 0x473c8, `ldr` at 0x473d8, `ldr` at 0x473e4). The branch is taken when
  `[ret(0x47408)+4] != 0x0`, reaching `0x473f8` (`bne` at 0x473ec). The zero fall-through
  writes four-byte `0x109eec` at `sp -0x20`
  (`str` at 0x473f4). The final merged return is unknown (`ldmdb` at 0x47400). A location-string
  role for the loaded value is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x473b0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x473b8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x473bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x473c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x473d0 | store | 4 | `sp -0x1c` | `ret(0x47408)` |
| `ldr` at 0x473d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x473d8 | load | 4 | `ret(0x47408) +0x4` | `-` |
| `str` at 0x473dc | store | 4 | `sp -0x20` | `[ret(0x47408)+4]` |
| `ldr` at 0x473e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x473e4 | load | 4 | `ret(0x47408) +0x4` | `-` |
| `ldr` at 0x473f0 | load | 4 | `abs 0x47404` | `-` |
| `str` at 0x473f4 | store | 4 | `sp -0x20` | `0x109eec` |
| `ldr` at 0x473f8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x47400 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47400 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47400 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x473c8 | `(unknown callee) 0x47408` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x473ec | `[ret(0x47408)+4] != 0x0` | `[ret(0x47408)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47400 | `r0` | `?` |
