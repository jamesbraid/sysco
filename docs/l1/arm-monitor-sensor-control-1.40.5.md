# ARM monitor sensor and control callers in 1.40.5

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
Computed transfers remain unresolved (`ldrls` at 0xccf40, 0xcd2a8, 0xcd364, 0xcd554, 0xcd990, 0xcdb54).
Known-edge tables exclude effects reached only through unknown destinations. Numeric callee effects
remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/monitor-sensor-control-facts.txt) retain all tool sections.

## `monitorInit`

- `monitorInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc980 to 0xccbc4.
  Established. The branch takes `[0x2adf80] != 0x0` past a four-byte `0x1` assignment at absolute
  `0x2adf80` (`bne` at 0xcc9f8, `str` at 0xcca04). The unnamed call to `0x1b454` carries `[sp-44]`,
  `0x1`, `0x2adf80`, unknown, and its result is assigned as four bytes at absolute `0x2adf7c` (`bl`
  at 0xcca24, `str` at 0xcca2c). Another call to that target has `[[sp-36]+8]`, `0x8`, `[sp-36]`,
  `[sp-36]` (`bl` at 0xccaa0). A later byte store writes `0x1` at an unresolved effective address
  (`strb` at 0xccb80). Allocation and monitor-record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc990 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xcc994 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xcc998 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xcc99c | load | 4 | `abs 0xccb98` | `-` |
| `ldr` at 0xcc9a0 | load | 4 | `abs 0x2adf7c` | `-` |
| `ldr` at 0xcc9ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc9b8 | load | 4 | `abs 0xccb9c` | `-` |
| `str` at 0xcc9bc | store | 4 | `sp -0x38` | `0x13489c` |
| `ldr` at 0xcc9c0 | load | 4 | `abs 0xccb98` | `-` |
| `ldr` at 0xcc9c4 | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xcc9c8 | store | 4 | `sp -0x34` | `[0x2adf7c]` |
| `ldr` at 0xcc9cc | load | 4 | `abs 0xccba0` | `-` |
| `ldr` at 0xcc9d0 | load | 4 | `abs 0x2adf80` | `-` |
| `str` at 0xcc9d4 | store | 4 | `sp -0x30` | `[0x2adf80]` |
| `ldr` at 0xcc9dc | load | 4 | `abs 0xccba4` | `-` |
| `ldr` at 0xcc9e0 | load | 4 | `abs 0xccba8` | `-` |
| `ldr` at 0xcc9ec | load | 4 | `abs 0xccba0` | `-` |
| `ldr` at 0xcc9f0 | load | 4 | `abs 0x2adf80` | `-` |
| `ldr` at 0xcc9fc | load | 4 | `abs 0xccba0` | `-` |
| `str` at 0xcca04 | store | 4 | `abs 0x2adf80` | `0x1` |
| `ldr` at 0xcca08 | load | 4 | `abs 0xccba0` | `-` |
| `ldr` at 0xcca0c | load | 4 | `abs 0x2adf80` | `-` |
| `str` at 0xcca14 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcca18 | load | 4 | `abs 0xccb98` | `-` |
| `ldr` at 0xcca1c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcca2c | store | 4 | `abs 0x2adf7c` | `ret(0x1b454)` |
| `ldr` at 0xcca30 | load | 4 | `abs 0x2adf7c` | `-` |
| `ldr` at 0xcca3c | load | 4 | `abs 0xccbac` | `-` |
| `str` at 0xcca40 | store | 4 | `sp -0x38` | `0x1348e8` |
| `ldr` at 0xcca44 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcca48 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xcca50 | load | 4 | `abs 0xccba4` | `-` |
| `ldr` at 0xcca54 | load | 4 | `abs 0xccbb0` | `-` |
| `ldr` at 0xcca60 | load | 4 | `abs 0xccb98` | `-` |
| `ldr` at 0xcca64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcca6c | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xcca74 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcca78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcca7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcca80 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xcca84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcca88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcca8c | store | 4 | `[sp-36] +0x8` | `[sp-32]` |
| `ldr` at 0xcca90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcca94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcca98 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xccaa8 | store | 4 | `[sp-36] +0x4` | `ret(0x1b454)` |
| `ldr` at 0xccaac | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xccab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccac0 | load | 4 | `abs 0xccbb4` | `-` |
| `ldr` at 0xccac4 | load | 4 | `abs 0xccbb8` | `-` |
| `str` at 0xccaf0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xccaf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccafc | load | 4 | `abs 0xccbb4` | `-` |
| `ldr` at 0xccb00 | load | 4 | `abs 0xccbb8` | `-` |
| `ldr` at 0xccb0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xccb1c | load | 4 | `abs 0xccbbc` | `-` |
| `str` at 0xccb20 | store | 4 | `(unknown)` | `0x134930` |
| `str` at 0xccb24 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xccb28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xccb2c | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xccb30 | store | 4 | `(unknown)` | `[[sp-36]+8]` |
| `ldr` at 0xccb38 | load | 4 | `abs 0xccba4` | `-` |
| `ldr` at 0xccb3c | load | 4 | `abs 0xccbc0` | `-` |
| `str` at 0xccb4c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xccb50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xccb54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xccb58 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xccb68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xccb6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xccb74 | load | 4 | `[sp-36] +0x4` | `-` |
| `strb` at 0xccb80 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xccb84 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xccb8c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0xccb94 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xccb94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xccb94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xccb94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc9e8 | `(unknown callee) 0x21eb8` | `0x9` | `0x1348cc` | `0x10f` | `0x0` |
| `bl` at 0xcca24 | `(unknown callee) 0x1b454` | `[sp-44]` | `0x1` | `0x2adf80` | `?` |
| `bl` at 0xcca5c | `(unknown callee) 0x21eb8` | `0x9` | `0x1348cc` | `0x11b` | `0x0` |
| `bl` at 0xccaa0 | `(unknown callee) 0x1b454` | `[[sp-36]+8]` | `0x8` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xccac8 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xccad4 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xccb04 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xccb14 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xccb44 | `(unknown callee) 0x21eb8` | `0x9` | `0x1348cc` | `0x126` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcc9a8 | `[0x2adf7c] == 0x0` | `[0x2adf7c] != 0x0` |
| `bne` at 0xcc9b4 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xcc9f8 | `[0x2adf80] != 0x0` | `[0x2adf80] == 0x0` |
| `bne` at 0xcca38 | `[0x2adf7c] != 0x0` | `[0x2adf7c] == 0x0` |
| `bne` at 0xccab4 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `blt` at 0xccb60 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xccb94 | `r0` | `?` |

## `monitorSetFlags`

- `monitorSetFlags`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xccbc4 to 0xccc0c.
  Established. The four-byte absolute load reads `0x2adf7c` (`ldr` at 0xccbec). A byte store at
  `[sp-28] +0xc` retains `((r1 & 0xff) & 0xff)` (`strb` at 0xccc00). The exit carries entry `r0`
  (`ldmdb` at 0xccc04). Flag definitions and device effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xccbd4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xccbdc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xccbe0 | load | 4 | `abs 0xccc08` | `-` |
| `ldr` at 0xccbe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccbec | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xccbf4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xccbf8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xccbfc | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xccc00 | store | 1 | `[sp-28] +0xc` | `((r1 & 0xff) & 0xff)` |
| `ldmdb` at 0xccc04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xccc04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xccc04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xccc04 | `r0` | `r0` |

## `monitorNotPresent`

- `monitorNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xccc0c to 0xccd18.
  Established. The branch takes `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` to `0xccc74` (`bge` at
  0xccc50). The fall-through writes byte `0x0` at an unresolved effective address (`strb` at
  0xccc6c). Included unnamed calls to `0x4f9a4` carry `r0`, `0x0`, `0x134924`, `0x13492c`, then
  `[sp-20]`, `0x0`, `0x134924`, `0x13492c` (`bl` at 0xccc84, `bl` at 0xcccc0). Physical-absence
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xccc1c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xccc24 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xccc28 | load | 4 | `abs 0xccd04` | `-` |
| `ldr` at 0xccc2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccc34 | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xccc3c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xccc40 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccc44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xccc48 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xccc54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xccc58 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccc60 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xccc6c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xccc74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccc7c | load | 4 | `abs 0xccd08` | `-` |
| `ldr` at 0xccc80 | load | 4 | `abs 0xccd0c` | `-` |
| `str` at 0xcccac | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcccb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcccb8 | load | 4 | `abs 0xccd08` | `-` |
| `ldr` at 0xcccbc | load | 4 | `abs 0xccd0c` | `-` |
| `ldr` at 0xcccc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcccd8 | load | 4 | `abs 0xccd10` | `-` |
| `str` at 0xcccdc | store | 4 | `(unknown)` | `0x134968` |
| `str` at 0xccce0 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xccce4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xccce8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xcccf0 | load | 4 | `abs 0xccd14` | `-` |
| `ldmdb` at 0xccd00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xccd00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xccd00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccc84 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xccc90 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xcccc0 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xcccd0 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xcccfc | `(unknown callee) 0x21eb8` | `0x6` | `0x1348cc` | `0x140` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xccc50 | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xccd00 | `r0` | `?` |

## `monitorPresent`

- `monitorPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xccd18 to 0xcce28.
  Established. The branch takes `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` to `0xccd80` (`bge` at
  0xccd5c); its fall-through writes byte `0x1` to an unresolved effective address (`strb` at
  0xccd78). A later unnamed call to `0x21eb8` carries `0x6`, `0x1348cc`, `0x14f`, `0x0` (`bl` at
  0xcce08). Presence-detection effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xccd28 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xccd30 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xccd34 | load | 4 | `abs 0xcce10` | `-` |
| `ldr` at 0xccd38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccd40 | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xccd48 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xccd4c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccd50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xccd54 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xccd60 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xccd64 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccd6c | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xccd78 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xccd80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccd88 | load | 4 | `abs 0xcce14` | `-` |
| `ldr` at 0xccd8c | load | 4 | `abs 0xcce18` | `-` |
| `str` at 0xccdb8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xccdbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xccdc4 | load | 4 | `abs 0xcce14` | `-` |
| `ldr` at 0xccdc8 | load | 4 | `abs 0xcce18` | `-` |
| `ldr` at 0xccdd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xccde4 | load | 4 | `abs 0xcce1c` | `-` |
| `str` at 0xccde8 | store | 4 | `(unknown)` | `0x134990` |
| `str` at 0xccdec | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xccdf0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xccdf4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xccdfc | load | 4 | `abs 0xcce20` | `-` |
| `ldr` at 0xcce00 | load | 4 | `abs 0xcce24` | `-` |
| `ldmdb` at 0xcce0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcce0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcce0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccd90 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xccd9c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xccdcc | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xccddc | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xcce08 | `(unknown callee) 0x21eb8` | `0x6` | `0x1348cc` | `0x14f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xccd5c | `(r1 & 0xff) >= [[sp-28]+8] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcce0c | `r0` | `?` |

## `monitorIsPresent`

- `monitorIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcce28 to 0xcce6c.
  Established. The unnamed call to `0xcebcc` has candidates `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)`
  (`bl` at 0xcce50). A byte read at `ret(0xcebcc) +0x0` supplies return expression
  `[ret(0xcebcc)]:1` (`ldrb` at 0xcce60, `ldmdb` at 0xcce68). A device-presence interpretation of
  that byte is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcce38 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcce40 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xcce44 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcce48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcce58 | store | 4 | `sp -0x1c` | `ret(0xcebcc)` |
| `ldr` at 0xcce5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcce60 | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldmdb` at 0xcce68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcce68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcce68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcce50 | `(unknown callee) 0xcebcc` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcce68 | `r0` | `[ret(0xcebcc)]:1` |

## `monitorReset`

- `monitorReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcce6c to 0xcd15c.
  Established. The unnamed call to `0xcebcc` uses `[sp-24]`, `[sp-37]:1`, unknown, `[sp-37]:1` (`bl`
  at 0xccedc). Byte-zero assignments target an unresolved effective address and `[sp-36] +0x0`
  (`strb` at 0xccf10, `strb` at 0xccf1c). The conditional computed transfer has unresolved
  destination and is taken for `[[sp-32]]:1-1 <= 0x5 (unsigned, 32-bit)` (`ldrls` at 0xccf40). A
  later branch takes `[sp-44] == 0x0` to `0xcd128` (`beq` at 0xcd084). Reset-register identities and
  selector-only effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcce7c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xcce80 | load | 4 | `abs 0xcd13c` | `-` |
| `ldr` at 0xcce84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcce8c | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xcce94 | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0xcce9c | store | 1 | `sp -0x25` | `0x0` |
| `ldrb` at 0xccea0 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xccea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xccea8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xcceb8 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xccebc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xccecc | store | 4 | `sp -0x20` | `ret(0xceaac)` |
| `ldrb` at 0xcced0 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xcced4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xccee4 | store | 4 | `sp -0x24` | `ret(0xcebcc)` |
| `ldr` at 0xccee8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcceec | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xccef8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xccefc | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xccf04 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xccf10 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xccf14 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xccf1c | store | 1 | `[sp-36] +0x0` | `0x0` |
| `ldr` at 0xccf20 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xccf24 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xccf30 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xccf34 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0xccf40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xccfec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xccff4 | load | 4 | `abs 0xcd140` | `-` |
| `ldr` at 0xccff8 | load | 4 | `abs 0xcd144` | `-` |
| `str` at 0xcd024 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcd028 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd030 | load | 4 | `abs 0xcd140` | `-` |
| `ldr` at 0xcd034 | load | 4 | `abs 0xcd144` | `-` |
| `ldr` at 0xcd040 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcd050 | load | 4 | `abs 0xcd148` | `-` |
| `str` at 0xcd054 | store | 4 | `(unknown)` | `0x1349b8` |
| `str` at 0xcd058 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xcd05c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd060 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcd064 | store | 4 | `(unknown)` | `[[sp-32]]:1` |
| `ldr` at 0xcd06c | load | 4 | `abs 0xcd14c` | `-` |
| `ldr` at 0xcd070 | load | 4 | `abs 0xcd150` | `-` |
| `ldr` at 0xcd07c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd088 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd090 | load | 4 | `abs 0xcd140` | `-` |
| `ldr` at 0xcd094 | load | 4 | `abs 0xcd144` | `-` |
| `str` at 0xcd0c0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcd0c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd0cc | load | 4 | `abs 0xcd140` | `-` |
| `ldr` at 0xcd0d0 | load | 4 | `abs 0xcd144` | `-` |
| `ldr` at 0xcd0dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcd0ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd0f8 | load | 4 | `abs 0xcd154` | `-` |
| `str` at 0xcd0fc | store | 4 | `(unknown)` | `0x1349d0` |
| `str` at 0xcd100 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xcd104 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd108 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0xcd10c | store | 4 | `(unknown)` | `[[sp-32]+8]` |
| `str` at 0xcd110 | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0xcd118 | load | 4 | `abs 0xcd14c` | `-` |
| `ldr` at 0xcd11c | load | 4 | `abs 0xcd158` | `-` |
| `ldrb` at 0xcd128 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0xcd130 | store | 1 | `sp -0x25` | `([sp-37]:1+1 & 0xff)` |
| `ldmdb` at 0xcd138 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcd138 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd138 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd138 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccec4 | `(unknown callee) 0xceaac` | `?` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0xccedc | `(unknown callee) 0xcebcc` | `[sp-24]` | `[sp-37]:1` | `?` | `[sp-37]:1` |
| `bl` at 0xccffc | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xcd008 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xcd038 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xcd048 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xcd078 | `(unknown callee) 0x21eb8` | `0x9` | `0x1348cc` | `0x195` | `0x0` |
| `bl` at 0xcd098 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xcd0a4 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xcd0d4 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xcd0e4 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xcd0f0 | `(unknown callee) 0x21a38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xcd124 | `(unknown callee) 0x21eb8` | `0x1` | `0x1348cc` | `0x19b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcceb0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xccef4 | `[[sp-32]]:1 != 0x63` | `[[sp-32]]:1 == 0x63` |
| `beq` at 0xccf2c | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xccf40 | `[[sp-32]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xcd084 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd138 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xccf40).

## `monitorGetCfg`

- `monitorGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd15c to 0xcd1b0.
  Established. The four-byte load reads absolute `0x2adf7c` (`ldr` at 0xcd180). A later four-byte
  store assigns `[[sp-28]]` at `r1 +0x0` (`str` at 0xcd198). The four-byte read at `[sp-28] +0x8`
  supplies `[[sp-28]+8]` to the exit (`ldr` at 0xcd1a0, `ldmdb` at 0xcd1a8). Configuration-record
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd16c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd170 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcd174 | load | 4 | `abs 0xcd1ac` | `-` |
| `ldr` at 0xcd178 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd180 | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xcd188 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xcd18c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd190 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcd194 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xcd198 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xcd19c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcd1a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0xcd1a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd1a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd1a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd1a8 | `r0` | `[[sp-28]+8]` |

## `monitorCount`

- `monitorCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd1b0 to 0xcd1f0.
  Established. A four-byte read targets absolute `0x2adf7c` (`ldr` at 0xcd1d0). Another four-byte
  read targets `[sp-24] +0x8` and supplies the return expression `[[sp-24]+8]` (`ldr` at 0xcd1e0,
  `ldmdb` at 0xcd1e8). The counted object is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd1c0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcd1c4 | load | 4 | `abs 0xcd1ec` | `-` |
| `ldr` at 0xcd1c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd1d0 | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xcd1d8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xcd1dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcd1e0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0xcd1e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd1e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd1e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd1e8 | `r0` | `[[sp-24]+8]` |

## `monitorSetVoltageLimits`

- `monitorSetVoltageLimits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd1f0 to 0xcd4bc.
  Established. The unnamed call to `0xcebcc` carries `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1`
  (`bl` at 0xcd250). The branch takes `[ret(0xcebcc)]:1 != 0x0` to `0xcd278` (`bne` at 0xcd268). The zero
  fall-through assigns four-byte `0xfffffc11` at `sp -0x2c` (`str` at 0xcd270). Conditional computed
  transfers are taken for `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` and `<= 0x8 (unsigned, 32-bit)`,
  with unresolved destinations (`ldrls` at 0xcd2a8, `ldrls` at 0xcd364). Voltage-limit units and
  selector-only effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd200 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcd208 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xcd210 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd218 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcd220 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xcd228 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0xcd22c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd230 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd240 | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldrb` at 0xcd244 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd248 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd258 | store | 4 | `sp -0x20` | `ret(0xcebcc)` |
| `ldr` at 0xcd25c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd260 | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xcd26c | load | 4 | `abs 0xcd4b4` | `-` |
| `str` at 0xcd270 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xcd278 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd27c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xcd280 | store | 4 | `sp -0x30` | `[[sp-28]]:1` |
| `ldr` at 0xcd284 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcd290 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xcd2a0 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xcd2a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcd354 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xcd35c | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xcd364 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcd3f8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcd400 | load | 4 | `abs 0xcd4b8` | `-` |
| `str` at 0xcd404 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xcd4ac | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xcd4b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd4b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd4b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd238 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcd250 | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd268 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `beq` at 0xcd28c | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xcd298 | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `ldrls` at 0xcd2a8 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `ldrls` at 0xcd364 | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd4b0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcd2a8).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcd364).

## `monitorSetTempLimits`

- `monitorSetTempLimits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd4bc to 0xcd7a4.
  Established. The byte read at `ret(0xcebcc) +0x0` precedes the branch taken for `[ret(0xcebcc)]:1
  != 0x0` (`ldrb` at 0xcd52c, `bne` at 0xcd534). The other arm writes four-byte `0xfffffc11` at `sp -0x2c` (`str` at 0xcd53c). The conditional computed transfer is taken for `[[sp-28]]:1-1 <= 0x4
  (unsigned, 32-bit)` and has unresolved destination (`ldrls` at 0xcd554). A later known-edge
  assignment writes four-byte `0xfffff830` at `sp -0x2c` (`str` at 0xcd6ec). Temperature-limit and
  selector-only meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd4cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcd4d4 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xcd4dc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd4e4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcd4ec | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xcd4f4 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0xcd4f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd4fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd50c | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldrb` at 0xcd510 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd514 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd524 | store | 4 | `sp -0x20` | `ret(0xcebcc)` |
| `ldr` at 0xcd528 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd52c | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xcd538 | load | 4 | `abs 0xcd79c` | `-` |
| `str` at 0xcd53c | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xcd544 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd548 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xcd554 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcd6e8 | load | 4 | `abs 0xcd7a0` | `-` |
| `str` at 0xcd6ec | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xcd794 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xcd798 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd798 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd798 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd504 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcd51c | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd534 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `ldrls` at 0xcd554 | `[[sp-28]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd798 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcd554).

## `monitorSetTachLimit`

- `monitorSetTachLimit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd7a4 to 0xcd8e0.
  Established. A nonzero `[ret(0xcebcc)]:1` takes the branch to `0xcd818` (`bne` at 0xcd808).
  Included unnamed calls target `0xcfa88`, `0xd06a4` and `0xd1c10`, each with candidates `[sp-20]`,
  `[sp-32]`, `[sp-22]:1`, `[sp-28]` (`bl` at 0xcd870, `bl` at 0xcd894, `bl` at 0xcd8b8). Their
  deciding equality branches are taken for `[[sp-32]]:1 == 0x1`, `== 0x2` and `== 0x6`, respectively
  (`beq` at 0xcd844, `beq` at 0xcd82c, `beq` at 0xcd854). Tachometer-limit units and callee effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd7b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd7b8 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcd7c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd7c8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xcd7cc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd7d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd7e0 | store | 4 | `sp -0x20` | `ret(0xceaac)` |
| `ldrb` at 0xcd7e4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd7e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd7f8 | store | 4 | `sp -0x24` | `ret(0xcebcc)` |
| `ldr` at 0xcd7fc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcd800 | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xcd80c | load | 4 | `abs 0xcd8d8` | `-` |
| `str` at 0xcd810 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xcd818 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd81c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcd820 | store | 4 | `sp -0x2c` | `[[sp-32]]:1` |
| `ldr` at 0xcd824 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd830 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd83c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcd84c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xcd85c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd860 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd864 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd86c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd878 | store | 4 | `sp -0x28` | `ret(0xcfa88)` |
| `ldrb` at 0xcd880 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd884 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd888 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd890 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd89c | store | 4 | `sp -0x28` | `ret(0xd06a4)` |
| `ldrb` at 0xcd8a4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd8a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd8ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd8b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd8c0 | store | 4 | `sp -0x28` | `ret(0xd1c10)` |
| `ldr` at 0xcd8c8 | load | 4 | `abs 0xcd8dc` | `-` |
| `str` at 0xcd8cc | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xcd8d0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcd8d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd8d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd8d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd7d8 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcd7f0 | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcd870 | `(unknown callee) 0xcfa88` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xcd894 | `(unknown callee) 0xd06a4` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xcd8b8 | `(unknown callee) 0xd1c10` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd808 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `beq` at 0xcd82c | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xcd838 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xcd844 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xcd854 | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd8d4 | `r0` | `?` |

## `monitorReadVoltage`

- `monitorReadVoltage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd8e0 to 0xcdac4.
  Established. The halfword store writes `0x0` at `r3 +0x0` (`strh` at 0xcd910). The unnamed call to
  `0xceaac` has candidates `[sp-20]`, `[sp-21]:1`, `r3`, `[sp-21]:1` (`bl` at 0xcd920). The
  conditional computed transfer is taken for `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` and has
  unresolved destination (`ldrls` at 0xcd990). Another arm calls unnamed `0xd02d4` with `[sp-20]`,
  `[sp-32]`, `[sp-22]:1`, `[sp-28]` (`bl` at 0xcda28). Voltage units and selector-only effects are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd8f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd8f4 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcd8fc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd904 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xcd908 | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xcd910 | store | 2 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xcd914 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd918 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd928 | store | 4 | `sp -0x20` | `ret(0xceaac)` |
| `ldrb` at 0xcd92c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd930 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd940 | store | 4 | `sp -0x24` | `ret(0xcebcc)` |
| `ldr` at 0xcd944 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcd948 | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xcd954 | load | 4 | `abs 0xcdabc` | `-` |
| `str` at 0xcd958 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xcd960 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd964 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcd968 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0xcd96c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcd978 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xcd988 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xcd990 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcda0c | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0xcda14 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcda18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcda1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcda24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcda30 | store | 4 | `sp -0x30` | `ret(0xd02d4)` |
| `ldr` at 0xcda38 | load | 4 | `abs 0xcdac0` | `-` |
| `str` at 0xcda3c | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0xcdab4 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xcdab8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdab8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdab8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd920 | `(unknown callee) 0xceaac` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xcd938 | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcda28 | `(unknown callee) 0xd02d4` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd950 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `beq` at 0xcd974 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xcd980 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `ldrls` at 0xcd990 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdab8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcd990).

## `monitorReadTemp`

- `monitorReadTemp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdac4 to 0xcdcc8.
  Established. A byte-zero assignment targets `r3 +0x0` (`strb` at 0xcdaf4). The unnamed call to
  `0xceaac` carries `[sp-20]`, `[sp-21]:1`, `r3`, `[sp-21]:1` (`bl` at 0xcdb04). A conditional
  computed transfer is taken for `[[sp-32]]:1-1 <= 0x4 (unsigned, 32-bit)` with unresolved
  destination (`ldrls` at 0xcdb54). Included known-edge four-byte stores at `sp -0x2c` retain
  `0xfffffc11` and `0xfffff830` in separate paths (`str` at 0xcdb3c, `str` at 0xcdc70). Temperature
  units and selector-only effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdad4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcdad8 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcdae0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcdae8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xcdaec | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xcdaf4 | store | 1 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xcdaf8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdafc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdb0c | store | 4 | `sp -0x20` | `ret(0xceaac)` |
| `ldrb` at 0xcdb10 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdb14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdb24 | store | 4 | `sp -0x24` | `ret(0xcebcc)` |
| `ldr` at 0xcdb28 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcdb2c | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xcdb38 | load | 4 | `abs 0xcdcc0` | `-` |
| `str` at 0xcdb3c | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xcdb44 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcdb48 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0xcdb54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcdc6c | load | 4 | `abs 0xcdcc4` | `-` |
| `str` at 0xcdc70 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xcdcb8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xcdcbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdcbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdcbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdb04 | `(unknown callee) 0xceaac` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xcdb1c | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcdb34 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `ldrls` at 0xcdb54 | `[[sp-32]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdcbc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcdb54).

## `monitorReadTach`

- `monitorReadTach`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdcc8 to 0xcde10.
  Established. The four-byte assignment writes `0x0` at `r3 +0x0` (`str` at 0xcdcf8). The branch
  taken for `[ret(0xcebcc)]:1 != 0x0` reaches `0xcdd48` (`bne` at 0xcdd38). Included later unnamed
  calls target `0xcf95c`, `0xd0500` and `0xd1b1c`, all with candidates `[sp-20]`, `[sp-32]`,
  `[sp-22]:1`, `[sp-28]` (`bl` at 0xcdda0, `bl` at 0xcddc4, `bl` at 0xcdde8). The final merged
  return is unknown (`ldmdb` at 0xcde04). Tachometer read effects and units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdcd8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcdcdc | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcdce4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcdcec | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xcdcf0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcdcf8 | store | 4 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xcdcfc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdd00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdd10 | store | 4 | `sp -0x20` | `ret(0xceaac)` |
| `ldrb` at 0xcdd14 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdd18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdd28 | store | 4 | `sp -0x24` | `ret(0xcebcc)` |
| `ldr` at 0xcdd2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcdd30 | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xcdd3c | load | 4 | `abs 0xcde08` | `-` |
| `str` at 0xcdd40 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xcdd48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcdd4c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcdd50 | store | 4 | `sp -0x2c` | `[[sp-32]]:1` |
| `ldr` at 0xcdd54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcdd60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcdd6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcdd7c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xcdd8c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcdd90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcdd94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdd9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcdda8 | store | 4 | `sp -0x28` | `ret(0xcf95c)` |
| `ldrb` at 0xcddb0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcddb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcddb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcddc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcddcc | store | 4 | `sp -0x28` | `ret(0xd0500)` |
| `ldrb` at 0xcddd4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcddd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcdddc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcdde4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcddf0 | store | 4 | `sp -0x28` | `ret(0xd1b1c)` |
| `ldr` at 0xcddf8 | load | 4 | `abs 0xcde0c` | `-` |
| `str` at 0xcddfc | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xcde00 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcde04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcde04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcde04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdd08 | `(unknown callee) 0xceaac` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xcdd20 | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcdda0 | `(unknown callee) 0xcf95c` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xcddc4 | `(unknown callee) 0xd0500` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |
| `bl` at 0xcdde8 | `(unknown callee) 0xd1b1c` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcdd38 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `beq` at 0xcdd5c | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xcdd68 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xcdd74 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xcdd84 | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcde04 | `r0` | `?` |

## `monitorWriteOutput`

- `monitorWriteOutput`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcde10 to 0xce0b0.
  Established. The unnamed call to `0xc8874` carries unknown, `[[sp-28]+2]:1`, `[sp-34]:1`, `0xe2`
  (`bl` at 0xcdfb0); its zero result takes the branch to `0xce058` (`beq` at 0xcdfc4). Another arm
  calls unnamed `0xc8998` with unknown, `[[sp-28]+2]:1`, `[sp-33]:1`, `sp-35` (`bl` at 0xce020). The
  later call to `0xc8874` carries unknown, `[[sp-28]+2]:1`, `[sp-33]:1`, `[sp-23]:1` (`bl` at
  0xce090). Device-output register meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcde20 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcde2c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcde34 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcde3c | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0xcde40 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcde44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcde54 | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldrb` at 0xcde58 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcde5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcde6c | store | 4 | `sp -0x20` | `ret(0xcebcc)` |
| `ldr` at 0xcde70 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcde74 | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xcde80 | load | 4 | `abs 0xce0a4` | `-` |
| `str` at 0xcde84 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xcde8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcde90 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xcde94 | store | 4 | `sp -0x34` | `[[sp-28]]:1` |
| `ldr` at 0xcde98 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcdea4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcdeb0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcdec0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xcded0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcdedc | load | 4 | `abs 0xce0a8` | `-` |
| `str` at 0xcdee0 | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `strb` at 0xcdeec | store | 1 | `sp -0x21` | `0x19` |
| `ldrb` at 0xcdef4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xcdef8 | store | 4 | `sp -0x30` | `[sp-22]:1` |
| `ldr` at 0xcdefc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcdf08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcdf14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcdf24 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xcdf38 | store | 1 | `sp -0x21` | `0x30` |
| `strb` at 0xcdf40 | store | 1 | `sp -0x22` | `0x5c` |
| `strb` at 0xcdf4c | store | 1 | `sp -0x21` | `0x31` |
| `strb` at 0xcdf54 | store | 1 | `sp -0x22` | `0x5d` |
| `strb` at 0xcdf60 | store | 1 | `sp -0x21` | `0x32` |
| `strb` at 0xcdf68 | store | 1 | `sp -0x22` | `0x5e` |
| `ldr` at 0xcdf70 | load | 4 | `abs 0xce0a8` | `-` |
| `str` at 0xcdf74 | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `ldr` at 0xcdf7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcdf80 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xcdf84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcdf98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcdf9c | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xcdfa0 | load | 1 | `sp -0x22` | `-` |
| `str` at 0xcdfb8 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xcdfbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcdfc8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xcdfcc | store | 4 | `sp -0x2c` | `ret(0xc8874)` |
| `ldrb` at 0xcdfdc | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xcdfe4 | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xcdfe8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcdfec | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xcdff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce004 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce008 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xce00c | load | 1 | `sp -0x21` | `-` |
| `str` at 0xce028 | store | 4 | `sp -0x28` | `ret(0xc8998)` |
| `ldrb` at 0xce02c | load | 1 | `sp -0x23` | `-` |
| `strb` at 0xce034 | store | 1 | `sp -0x23` | `?` |
| `ldrb` at 0xce038 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xce03c | load | 1 | `sp -0x23` | `-` |
| `strb` at 0xce044 | store | 1 | `sp -0x17` | `?` |
| `ldr` at 0xce04c | load | 4 | `abs 0xce0ac` | `-` |
| `str` at 0xce050 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xce058 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce05c | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xce060 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce074 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce078 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xce07c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xce080 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xce098 | store | 4 | `sp -0x2c` | `ret(0xc8874)` |
| `ldr` at 0xce09c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xce0a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce0a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce0a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcde4c | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcde64 | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcdf8c | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xcdfb0 | `(unknown callee) 0xc8874` | `?` | `[[sp-28]+2]:1` | `[sp-34]:1` | `0xe2` |
| `bl` at 0xcdff8 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xce020 | `(unknown callee) 0xc8998` | `?` | `[[sp-28]+2]:1` | `[sp-33]:1` | `sp-35` |
| `bl` at 0xce068 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0xce090 | `(unknown callee) 0xc8874` | `?` | `[[sp-28]+2]:1` | `[sp-33]:1` | `[sp-23]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcde7c | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `beq` at 0xcdea0 | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `bgt` at 0xcdeac | `[[sp-28]]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xcdeb8 | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xcdec8 | `[[sp-28]]:1 == 0x6` | `[[sp-28]]:1 != 0x6` |
| `beq` at 0xcded8 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xcdf04 | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0xcdf10 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xcdf1c | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xcdf2c | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `beq` at 0xcdfc4 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce0a0 | `r0` | `?` |
