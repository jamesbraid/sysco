# ARM capture and image synchronization paths in 1.40.5

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
`SdrDecode` and `SerNumLearn` have unresolved computed transfers. Their access, call and return
lists cover known edges only. Unknown data values and instruction effects remain explicit
in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/capture-image-sync-facts.txt) retain all tool sections.

## `IP57_Int`

- `IP57_Int`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x614b0 to 0x61724.
  Established. The unnamed `0x5db5c` call carries `0x0`, `[sp-24]:1`, `[sp-25]:1`, `[sp-26]:1` (`bl` at 0x61528). The unnamed `0xd7530` call carries `0x0`, `[sp-24]:1`, `sp-26`, `[sp-24]:1` (`bl` at 0x61550); its zero result takes `0x615b0`, while nonzero falls through (`beq` at 0x61564).
  Interrupt signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x614c4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x614c8 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x614d0 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0x614d8 | store | 1 | `sp -0x16` | `(r1 & 0xff)` |
| `strb` at 0x614e0 | store | 1 | `sp -0x17` | `(r2 & 0xff)` |
| `strb` at 0x614e8 | store | 1 | `sp -0x18` | `(r3 & 0xff)` |
| `strb` at 0x614f0 | store | 1 | `sp -0x19` | `([sp] & 0xff)` |
| `strb` at 0x614f8 | store | 1 | `sp -0x1a` | `([sp+4] & 0xff)` |
| `ldrb` at 0x614fc | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x61504 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x6150c | store | 1 | `sp -0x1b` | `?` |
| `ldrb` at 0x61510 | load | 1 | `sp -0x18` | `-` |
| `ldrb` at 0x61514 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x61518 | load | 1 | `sp -0x1a` | `-` |
| `ldrsb` at 0x6152c | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x61540 | load | 1 | `sp -0x18` | `-` |
| `str` at 0x61558 | store | 4 | `sp -0x20` | `ret(0xd7530)` |
| `ldr` at 0x6155c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x61568 | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x6157c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61588 | load | 4 | `abs 0x61704` | `-` |
| `str` at 0x6158c | store | 4 | `sp -0x38` | `0x10d198` |
| `str` at 0x61590 | store | 4 | `sp -0x34` | `ret(0xd7eec)` |
| `str` at 0x61594 | store | 4 | `sp -0x30` | `ret(0x21a38)` |
| `ldr` at 0x6159c | load | 4 | `abs 0x61708` | `-` |
| `ldr` at 0x615a0 | load | 4 | `abs 0x6170c` | `-` |
| `ldrb` at 0x615b0 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x615c4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x615cc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x615d4 | load | 4 | `abs 0x61710` | `-` |
| `ldrb` at 0x615e8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x61600 | store | 4 | `sp -0x20` | `ret(0x537cc)` |
| `ldr` at 0x61604 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x61610 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x61624 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61630 | load | 4 | `abs 0x61714` | `-` |
| `str` at 0x61634 | store | 4 | `sp -0x38` | `0x10d200` |
| `str` at 0x61638 | store | 4 | `sp -0x34` | `ret(0x53874)` |
| `str` at 0x6163c | store | 4 | `sp -0x30` | `ret(0x21a38)` |
| `ldr` at 0x61644 | load | 4 | `abs 0x61708` | `-` |
| `ldr` at 0x61648 | load | 4 | `abs 0x61718` | `-` |
| `ldr` at 0x61658 | load | 4 | `abs 0x6171c` | `-` |
| `str` at 0x6165c | store | 4 | `sp -0x38` | `0x10d224` |
| `ldrb` at 0x61660 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x61668 | store | 4 | `sp -0x34` | `[sp-21]:1+48` |
| `ldrb` at 0x6166c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x61674 | store | 4 | `sp -0x30` | `[sp-22]:1+65` |
| `ldr` at 0x61678 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6167c | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x61680 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x6169c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x6169c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x616b8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x616b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x616d8 | store | 4 | `sp -0x28` | `ret(0xfbbf8)` |
| `ldr` at 0x616e0 | load | 4 | `abs 0x61708` | `-` |
| `ldr` at 0x616e4 | load | 4 | `abs 0x61720` | `-` |
| `ldmdb` at 0x616f0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x616f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x616f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x616f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x61528 | `(unknown callee) 0x5db5c` | `0x0` | `[sp-24]:1` | `[sp-25]:1` | `[sp-26]:1` |
| `bl` at 0x6153c | `(unknown callee) 0x61854` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x61550 | `(unknown callee) 0xd7530` | `0x0` | `[sp-24]:1` | `sp-26` | `[sp-24]:1` |
| `bl` at 0x61574 | `(unknown callee) 0xd7eec` | `0x0` | `[sp-24]:1` | `?` | `[sp-24]:1` |
| `bl` at 0x61580 | `(unknown callee) 0x21a38` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x615a8 | `(unknown callee) 0x21eb8` | `0x3` | `0x10d08c` | `0x494` | `0x0` |
| `bl` at 0x615e0 | `(unknown callee) 0x2079c` | `0x10d1d4` | `[sp-21]:1+48` | `[sp-22]:1+65` | `[sp-22]:1+65` |
| `bl` at 0x615f8 | `(unknown callee) 0x537cc` | `0x0` | `[sp-23]:1` | `sp-36` | `[sp-23]:1` |
| `bl` at 0x6161c | `(unknown callee) 0x53874` | `0x0` | `[sp-23]:1` | `?` | `[sp-23]:1` |
| `bl` at 0x61628 | `(unknown callee) 0x21a38` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x61650 | `(unknown callee) 0x21eb8` | `0x3` | `0x10d08c` | `0x49e` | `0x0` |
| `bl` at 0x61684 | `(unknown callee) 0xfb618` | `[sp-36]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x616a0 | `(unknown callee) 0xfb690` | `ret(0xfb618)` | `?` | `?` | `?` |
| `bl` at 0x616bc | `(unknown callee) 0xfb30c` | `ret(0xfb690)` | `?` | `?` | `?` |
| `bl` at 0x616d0 | `(unknown callee) 0xfbbf8` | `ret(0xfb30c)` | `?` | `?` | `ret(0xfb30c)` |
| `bl` at 0x616ec | `(unknown callee) 0x21eb8` | `0x1` | `0x10d08c` | `0x4a1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x61534 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `beq` at 0x61564 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `bne` at 0x615c0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6160c | `ret(0x537cc) == 0x0` | `ret(0x537cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x616f0 | `r0` | `?` |

## `SdrDecode`

- `SdrDecode`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad860 to 0xad914.
  Established. The byte read uses `r0 +0x3` (`ldrb` at 0xad880). The word assignment at `sp -0x1c`
  writes `[r0+3]:1` (`str` at 0xad884). The selector takes unsigned `[r0+3]:1-1 <= 0x11` to an
  unresolved destination, while larger values fall through (`ldrls` at 0xad894). Sensor record
  decoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad870 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xad874 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xad878 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xad87c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xad880 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xad884 | store | 4 | `sp -0x1c` | `[r0+3]:1` |
| `ldr` at 0xad888 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0xad894 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xad910 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad910 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad910 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xad894 | `[r0+3]:1-1 <= 0x11 (unsigned, 32-bit)` | `[r0+3]:1-1 > 0x11 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad910 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xad894).

## `IcmbMsgGetData`

- `IcmbMsgGetData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf1a0 to 0xaf1f8.
  Established. The unnamed `0xaf000` call carries `r0`, `r1`, `r2`, `r3` (`bl` at 0xaf1b8); its
  nonzero result takes `0xaf1e0`, while zero falls through (`bne` at 0xaf1cc). The word assignments
  at `sp -0x1c` write `[sp-20]+2` and `[sp-20]+3` (`str` at 0xaf1d8, `str` at 0xaf1e8). Message data
  contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf1b0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf1b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf1c0 | store | 4 | `sp -0x18` | `ret(0xaf000)` |
| `ldr` at 0xaf1c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf1d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf1d8 | store | 4 | `sp -0x1c` | `[sp-20]+2` |
| `ldr` at 0xaf1e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf1e8 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xaf1ec | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xaf1f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf1f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf1f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf1b8 | `(unknown callee) 0xaf000` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf1cc | `ret(0xaf000) != 0x0` | `ret(0xaf000) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf1f4 | `r0` | `[sp-28]` |

## `SerNumLearn`

- `SerNumLearn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf274 to 0xaf7ac.
  Established. The absolute word assignment at `0x14a7c0` writes `0x0` (`str` at 0xaf380). The
  unnamed `0x1d41c` call carries `0xd`, `[sp-20]`, unknown, `ret(0xaf7ac)` (`bl` at 0xaf654). The
  selector takes unsigned `[sp-140]-48 <= 0x4c` to an unresolved destination, while larger values
  fall through (`ldrls` at 0xaf3c0). Serial-number learning policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf284 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaf288 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaf294 | store | 4 | `sp -0x1c` | `ret(0x1f26c)` |
| `str` at 0xaf2a4 | store | 4 | `sp -0x8c` | `?` |
| `str` at 0xaf2b0 | store | 4 | `sp -0x90` | `ret(0x1f26c)` |
| `str` at 0xaf2bc | store | 4 | `sp -0x94` | `ret(0x1f26c)` |
| `ldr` at 0xaf2c0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xaf2cc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xaf2d8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xaf2e4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xaf2f4 | load | 4 | `abs 0xaf770` | `-` |
| `ldr` at 0xaf2f8 | load | 4 | `abs 0x148fd0` | `-` |
| `str` at 0xaf308 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaf310 | load | 4 | `abs 0xaf770` | `-` |
| `ldr` at 0xaf314 | load | 4 | `abs 0x148fd0` | `-` |
| `ldr` at 0xaf320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf324 | load | 4 | `abs 0xaf774` | `-` |
| `str` at 0xaf334 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaf33c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf344 | load | 4 | `abs 0xaf778` | `-` |
| `ldr` at 0xaf348 | load | 4 | `abs 0x14a7c0` | `-` |
| `ldr` at 0xaf358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf364 | load | 4 | `abs 0xaf77c` | `-` |
| `ldr` at 0xaf378 | load | 4 | `abs 0xaf778` | `-` |
| `str` at 0xaf380 | store | 4 | `abs 0x14a7c0` | `0x0` |
| `str` at 0xaf388 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaf390 | load | 4 | `abs 0xaf780` | `-` |
| `ldr` at 0xaf394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf39c | load | 4 | `abs 0xaf778` | `-` |
| `ldr` at 0xaf3a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaf3a4 | store | 4 | `abs 0x14a7c0` | `[sp-24]` |
| `ldr` at 0xaf3ac | load | 4 | `abs 0xaf780` | `-` |
| `str` at 0xaf3b0 | store | 4 | `sp -0x14` | `0x2b28c8` |
| `ldr` at 0xaf3b4 | load | 4 | `sp -0x8c` | `-` |
| `ldrls` at 0xaf3c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf5f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf5fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf610 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf614 | load | 4 | `abs 0xaf78c` | `-` |
| `ldr` at 0xaf628 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xaf62c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xaf638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf660 | load | 4 | `abs 0xaf790` | `-` |
| `ldr` at 0xaf664 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf670 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaf678 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0xaf67c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaf688 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaf694 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaf6a4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaf6b0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xaf6c8 | load | 4 | `abs 0xaf794` | `-` |
| `ldr` at 0xaf6dc | load | 4 | `abs 0xaf798` | `-` |
| `ldr` at 0xaf6f0 | load | 4 | `abs 0xaf79c` | `-` |
| `ldr` at 0xaf704 | load | 4 | `abs 0xaf7a0` | `-` |
| `ldr` at 0xaf71c | load | 4 | `abs 0xaf7a4` | `-` |
| `ldr` at 0xaf728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf72c | load | 4 | `abs 0xaf774` | `-` |
| `str` at 0xaf748 | store | 4 | `sp -0x98` | `0x1` |
| `ldr` at 0xaf750 | load | 4 | `abs 0xaf7a8` | `-` |
| `ldr` at 0xaf758 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf764 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xaf768 | load | 4 | `sp -0x98` | `-` |
| `ldmdb` at 0xaf76c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf76c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf76c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf290 | `(unknown callee) 0x1f26c` | `0xd` | `r1` | `r2` | `r3` |
| `bl` at 0xaf298 | `(unknown callee) 0x4f4d0` | `ret(0x1f26c)` | `?` | `?` | `?` |
| `bl` at 0xaf2ac | `(unknown callee) 0x1f26c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xaf2b8 | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xaf368 | `(unknown callee) 0x1b088` | `[sp-20]+1` | `0x127ea8` | `?` | `[sp-20]+1` |
| `bl` at 0xaf398 | `(unknown callee) 0x1b598` | `0x2b28c8` | `[sp-20]` | `?` | `ret(0x1b088)` |
| `bl` at 0xaf600 | `(unknown callee) 0x1b088` | `[sp-28]` | `?` | `?` | `[sp-140]-48` |
| `bl` at 0xaf618 | `(unknown callee) 0x1b088` | `[sp-28]` | `0x127f60` | `?` | `ret(0x1b088)` |
| `bl` at 0xaf63c | `(unknown callee) 0xaf7ac` | `[sp-20]` | `?` | `?` | `[[sp-28]]:1` |
| `bl` at 0xaf654 | `(unknown callee) 0x1d41c` | `0xd` | `[sp-20]` | `?` | `ret(0xaf7ac)` |
| `bl` at 0xaf66c | `(unknown callee) 0x1b52c` | `sp-128` | `0x127f6c` | `[sp-24]` | `[sp-20]` |
| `bl` at 0xaf6cc | `(unknown callee) 0x1b364` | `sp-128` | `0x127fac` | `?` | `sp-128` |
| `bl` at 0xaf6e0 | `(unknown callee) 0x1b364` | `sp-128` | `0x127fb4` | `?` | `sp-128` |
| `bl` at 0xaf6f4 | `(unknown callee) 0x1b364` | `sp-128` | `0x127fc0` | `?` | `sp-128` |
| `bl` at 0xaf708 | `(unknown callee) 0x1b364` | `sp-128` | `0x127fcc` | `?` | `sp-128` |
| `bl` at 0xaf714 | `(unknown callee) 0x202d0` | `sp-128` | `?` | `?` | `sp-128` |
| `bl` at 0xaf724 | `(unknown callee) 0x23fbc` | `0x127fd8` | `sp-128` | `?` | `sp-128` |
| `bl` at 0xaf740 | `(unknown callee) 0x75a50` | `?` | `?` | `[sp-24]` | `?` |
| `bl` at 0xaf754 | `(unknown callee) 0x202d0` | `0x127fdc` | `?` | `?` | `ret(0xaf7ac)` |
| `bl` at 0xaf75c | `(unknown callee) 0x202d0` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaf2c8 | `[sp-140] == 0x52` | `[sp-140] != 0x52` |
| `beq` at 0xaf2d4 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0xaf2e0 | `[sp-140] == 0x36` | `[sp-140] != 0x36` |
| `beq` at 0xaf2ec | `[sp-140] == 0x40` | `[sp-140] != 0x40` |
| `bne` at 0xaf300 | `[0x148fd0] != 0x2` | `[0x148fd0] == 0x2` |
| `bne` at 0xaf31c | `[0x148fd0] != 0x1` | `[0x148fd0] == 0x1` |
| `bgt` at 0xaf32c | `[sp-24] > 0x3fff (signed, 32-bit)` | `[sp-24] <= 0x3fff (signed, 32-bit)` |
| `blt` at 0xaf354 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `bne` at 0xaf374 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `ldrls` at 0xaf3c0 | `[sp-140]-48 <= 0x4c (unsigned, 32-bit)` | `[sp-140]-48 > 0x4c (unsigned, 32-bit)` |
| `beq` at 0xaf60c | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xaf624 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0xaf634 | `[[sp-28]]:1 == 0x0` | `[[sp-28]]:1 != 0x0` |
| `beq` at 0xaf648 | `ret(0xaf7ac) == 0x0` | `ret(0xaf7ac) != 0x0` |
| `beq` at 0xaf684 | `[sp-156] == 0x2000` | `[sp-156] != 0x2000` |
| `bgt` at 0xaf690 | `[sp-156] > 0x2000 (signed, 32-bit)` | `[sp-156] <= 0x2000 (signed, 32-bit)` |
| `beq` at 0xaf69c | `[sp-156] == 0x1000` | `[sp-156] != 0x1000` |
| `beq` at 0xaf6ac | `[sp-156] == 0x4000` | `[sp-156] != 0x4000` |
| `beq` at 0xaf6b8 | `[sp-156] == 0x8000` | `[sp-156] != 0x8000` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf76c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xaf3c0).

## `SerNumValid`

- `SerNumValid`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf7ac to 0xafa2c.
  Established. The absolute byte at `0x2af28c` is read (`ldrb` at 0xaf7c4). Nonzero takes `0xaf7dc`,
  while zero falls through (`bne` at 0xaf7cc). The unnamed `0x1b01c` call carries `sp-28`,
  `[[sp-20]]:1`, `sp-28`, `[[sp-20]]:1` (`bl` at 0xaf81c). Serial-number validity policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf7bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf7c0 | load | 4 | `abs 0xafa20` | `-` |
| `ldrb` at 0xaf7c4 | load | 1 | `abs 0x2af28c` | `-` |
| `str` at 0xaf7d4 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xaf7dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf7f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf7fc | load | 4 | `abs 0xafa20` | `-` |
| `ldrb` at 0xaf800 | load | 1 | `abs 0x2af28c` | `-` |
| `strb` at 0xaf804 | store | 1 | `sp -0x1c` | `([0x2af28c]:1 & 0xff)` |
| `ldr` at 0xaf80c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf810 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xaf830 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf844 | load | 4 | `abs 0xafa24` | `-` |
| `ldr` at 0xaf858 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf864 | load | 4 | `abs 0xafa28` | `-` |
| `str` at 0xaf87c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf888 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf890 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xaf8a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf8a8 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldr` at 0xaf8c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf8cc | load | 1 | `[sp-20] +0x2` | `-` |
| `ldr` at 0xaf8d4 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xaf8dc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf8fc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf904 | load | 1 | `[sp-20] +0x3` | `-` |
| `ldr` at 0xaf90c | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xaf914 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf934 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf93c | load | 1 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xaf944 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xaf94c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf96c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf974 | load | 1 | `[sp-20] +0x5` | `-` |
| `ldr` at 0xaf97c | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xaf984 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf9a4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf9ac | load | 1 | `[sp-20] +0x6` | `-` |
| `ldr` at 0xaf9b4 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xaf9bc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xaf9dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xaf9e4 | load | 1 | `[sp-20] +0x7` | `-` |
| `ldr` at 0xaf9ec | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xaf9f4 | load | 2 | `(unknown)` | `-` |
| `str` at 0xafa08 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xafa14 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xafa18 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xafa1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xafa1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xafa1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf7e0 | `(unknown callee) 0x1b2bc` | `r0` | `r1` | `r2` | `[0x2af28c]:1` |
| `bl` at 0xaf81c | `(unknown callee) 0x1b01c` | `sp-28` | `[[sp-20]]:1` | `sp-28` | `[[sp-20]]:1` |
| `bl` at 0xaf848 | `(unknown callee) 0x1b088` | `[sp-20]+1` | `0x128004` | `?` | `[sp-20]+1` |
| `bl` at 0xaf868 | `(unknown callee) 0x1b088` | `[sp-20]+1` | `0x127eb0` | `?` | `[sp-20]+1` |
| `bl` at 0xaf8b4 | `(unknown callee) 0x1b55c` | `ret(0x1b088)` | `?` | `0x1` | `[[sp-20]+1]:1` |
| `bl` at 0xaf8ec | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0xaf924 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0xaf95c | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0xaf994 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0xaf9cc | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf7cc | `[0x2af28c]:1 != 0x0` | `[0x2af28c]:1 == 0x0` |
| `beq` at 0xaf7ec | `ret(0x1b2bc) == 0x8` | `ret(0x1b2bc) != 0x8` |
| `bne` at 0xaf828 | `ret(0x1b01c) != 0x0` | `ret(0x1b01c) == 0x0` |
| `beq` at 0xaf854 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `bne` at 0xaf874 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bls` at 0xaf898 | `[[sp-20]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-20]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0xaf8b0 | `[[sp-20]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-20]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0xaf8e8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf920 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf958 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf990 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaf9c8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xafa00 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xafa1c | `r0` | `?` |

## `SysComCaptureOut`

- `SysComCaptureOut`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb093c to 0xb0de4.
  Established. The absolute word read uses `0x2b24b4` (`ldr` at 0xb0984). The unnamed `0x1b394` call
  has unknown first and fourth candidates with `[sp-28]` and `[sp-32]` between them (`bl` at 0xb0cd4). The halfword store at `0xb0dd0` has an unknown target and writes `([sp-40]:2 & 0xffff)`
  (`strh` at 0xb0dd0). Capture record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb094c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb0950 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb0954 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb0958 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb095c | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0960 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0984 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb098c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb09b8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb09c0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb09c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb09c8 | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb09cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb09f0 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb09f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0a24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0a2c | load | 4 | `abs 0xb0ddc` | `-` |
| `ldrh` at 0xb0a30 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb0a34 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb0a38 | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0a3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0a60 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0a68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0a6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0a7c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0a88 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb0a8c | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0a90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0ab4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0abc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0ae8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0af0 | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0af4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0b18 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0b20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0b4c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb0b54 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xb0b74 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb0b78 | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0b7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0ba0 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0ba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0bd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0bdc | load | 4 | `abs 0xb0ddc` | `-` |
| `ldr` at 0xb0be0 | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0c08 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0c3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0c44 | load | 4 | `abs 0xb0ddc` | `-` |
| `ldrh` at 0xb0c48 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb0c50 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xb0c54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0c80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0c8c | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0cb4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0cbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0ccc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0cd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0cd8 | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0cdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0d00 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0d08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0d34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0d40 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb0d4c | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xb0d50 | store | 1 | `(unknown)` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xb0d54 | load | 4 | `abs 0xb0dd8` | `-` |
| `ldr` at 0xb0d58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0d7c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0d84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0db0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0db8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0dbc | load | 4 | `abs 0xb0de0` | `-` |
| `ldrh` at 0xb0dcc | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb0dd0 | store | 2 | `(unknown)` | `([sp-40]:2 & 0xffff)` |
| `ldmdb` at 0xb0dd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0dd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0dd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0cd4 | `(unknown callee) 0x1b394` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb0a74 | `[sp-24] < ? (signed, 32-bit)` | `[sp-24] >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0dd4 | `r0` | `0x2b24a0` |

## `SysComCaptureDump`

- `SysComCaptureDump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1168 to 0xb17b0.
  Established. The signed condition `r0 < [0x2b24b0]` takes `0xb11cc`, while greater-or-equal falls
  through (`blt` at 0xb1194). The unnamed `0x1b1b4` call at `0xb11a0` carries `0x50`, `r1`, `r0`,
  `[0x2b24b0]` (`bl` at 0xb11a0). The word assignment through `r2 +0x0` writes `ret(0x1b1b4)` (`str` at 0xb11a8). Capture dump contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1178 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb117c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb1180 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xb1184 | load | 4 | `abs 0xb1780` | `-` |
| `ldr` at 0xb1188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb118c | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xb1198 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb11a8 | store | 4 | `r2 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xb11ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb11b0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xb11b4 | load | 4 | `abs 0xb1784` | `-` |
| `ldr` at 0xb11b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb11c4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb11cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb11d8 | load | 4 | `abs 0xb1780` | `-` |
| `ldr` at 0xb11dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1200 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb1208 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb120c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb1214 | load | 4 | `abs 0xb1780` | `-` |
| `ldr` at 0xb1218 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb123c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb1244 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1248 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb124c | load | 4 | `abs 0xb1780` | `-` |
| `ldr` at 0xb1250 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1274 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb127c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1280 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb1284 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1290 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb12a0 | store | 4 | `[sp-32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xb12a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb12a8 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xb12ac | load | 4 | `abs 0xb1788` | `-` |
| `str` at 0xb12b8 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0xb12c4 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xb12cc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb12d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb12d4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb12e8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb12ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb12fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1328 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1334 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb1340 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1350 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb135c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb1360 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1368 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1370 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb1378 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb1380 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1384 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb1398 | store | 4 | `[sp-32] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xb139c | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xb13a0 | store | 4 | `sp -0x40` | `[[sp-32]]` |
| `str` at 0xb13a8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb13ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb13b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb13c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb13ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb13f4 | load | 4 | `abs 0xb178c` | `-` |
| `ldr` at 0xb13f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb13fc | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xb1400 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb142c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1434 | load | 4 | `abs 0xb1790` | `-` |
| `ldr` at 0xb1438 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb143c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xb1440 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb1444 | load | 4 | `abs 0xb1794` | `-` |
| `ldr` at 0xb1448 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb144c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1454 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb1464 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb146c | store | 4 | `sp -0x40` | `([sp-64] + ret(0x1b2bc))` |
| `ldr` at 0xb1470 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb149c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb14a4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb14ac | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xb14b4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb14b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb14c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb14f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1500 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb150c | load | 1 | `(unknown)` | `-` |
| `str` at 0xb1510 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb1514 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb1520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb154c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1554 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb1558 | load | 4 | `abs 0xb1798` | `-` |
| `ldrh` at 0xb1568 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb156c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb1570 | load | 4 | `abs 0xb179c` | `-` |
| `ldr` at 0xb157c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb158c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb1594 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x1b2bc))` |
| `str` at 0xb159c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xb15a0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb15a4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb15b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb15e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb15e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb15f4 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xb15fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1614 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1640 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1648 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb1654 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xb165c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1660 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb1664 | load | 4 | `abs 0xb17a0` | `-` |
| `ldr` at 0xb1674 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb16a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb16a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb16b4 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xb16bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb16c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb16c4 | load | 4 | `abs 0xb17a4` | `-` |
| `ldr` at 0xb16d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb16d8 | store | 4 | `sp -0x40` | `[sp-64]+3` |
| `ldr` at 0xb16dc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xb16e4 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0xb16ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb16f0 | load | 4 | `abs 0xb17a8` | `-` |
| `ldr` at 0xb16f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb1708 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb1710 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x1b2bc))` |
| `ldr` at 0xb1714 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1734 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1738 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb1740 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xb1748 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb1750 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xb1758 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb175c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb1760 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xb176c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xb1770 | load | 4 | `abs 0xb17ac` | `-` |
| `ldr` at 0xb1778 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0xb177c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb177c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb177c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb177c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb11a0 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r0` | `[0x2b24b0]` |
| `bl` at 0xb11bc | `(unknown callee) 0x1b52c` | `[[sp-32]]` | `0x12813c` | `[sp-24]` | `[sp-32]` |
| `bl` at 0xb1298 | `(unknown callee) 0x1b1b4` | `0x50` | `[sp-24]` | `?` | `[sp-36]` |
| `bl` at 0xb12b0 | `(unknown callee) 0x1b598` | `[[sp-32]]` | `0x12815c` | `?` | `[sp-32]` |
| `bl` at 0xb1390 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xb1450 | `(unknown callee) 0x1b52c` | `?` | `0x128190` | `[sp-24]` | `?` |
| `bl` at 0xb1458 | `(unknown callee) 0x1b2bc` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xb1578 | `(unknown callee) 0x1b52c` | `?` | `0x1281b8` | `?` | `?` |
| `bl` at 0xb1580 | `(unknown callee) 0x1b2bc` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xb1604 | `(unknown callee) 0x1b04c` | `?` | `?` | `?` | `?` |
| `bl` at 0xb166c | `(unknown callee) 0x1b52c` | `[sp-64]` | `0x1281c0` | `?` | `?` |
| `bl` at 0xb16cc | `(unknown callee) 0x1b52c` | `[sp-64]` | `0x1281c8` | `?` | `?` |
| `bl` at 0xb16f4 | `(unknown callee) 0x1b598` | `?` | `0x1281d0` | `?` | `[sp-68]` |
| `bl` at 0xb16fc | `(unknown callee) 0x1b2bc` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xb1774 | `(unknown callee) 0x1b364` | `?` | `0x1281d4` | `?` | `[[sp-32]]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb1194 | `r0 < [0x2b24b0] (signed, 32-bit)` | `r0 >= [0x2b24b0] (signed, 32-bit)` |
| `bne` at 0xb11d4 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xb128c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `blt` at 0xb12dc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xb12f4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0xb13b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xb14c0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xb151c | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `blt` at 0xb15ac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb1610 | `ret(0x1b04c) == 0x0` | `ret(0x1b04c) != 0x0` |
| `bne` at 0xb1768 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb177c | `r0` | `[sp-76]` |

## `SysComSimStart`

- `SysComSimStart`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0000 to 0xb00e0.
  Established. The unnamed `0x229b0` call carries `0x14a7d4`, `0x128068`, `0x0`, `0x9` (`bl` at 0xb0030). The unnamed `0x22480` call carries `0x14a7d0`, `0x128074`, `0xb0100`, `0x0` (`bl` at 0xb006c). The absolute word assignment at `0x14a7c8` writes `0x1` (`str` at 0xb0078).
  Simulation-start effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb0010 | load | 4 | `abs 0xb00c0` | `-` |
| `ldr` at 0xb0014 | load | 4 | `abs 0x14a7c8` | `-` |
| `ldr` at 0xb0020 | load | 4 | `abs 0xb00c4` | `-` |
| `ldr` at 0xb0024 | load | 4 | `abs 0xb00c8` | `-` |
| `str` at 0xb0038 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xb0040 | store | 4 | `sp -0x24` | `0x800` |
| `str` at 0xb0048 | store | 4 | `sp -0x20` | `0x19` |
| `str` at 0xb0050 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0xb0058 | store | 4 | `sp -0x18` | `0x9` |
| `ldr` at 0xb005c | load | 4 | `abs 0xb00cc` | `-` |
| `ldr` at 0xb0060 | load | 4 | `abs 0xb00d0` | `-` |
| `ldr` at 0xb0064 | load | 4 | `abs 0xb00d4` | `-` |
| `ldr` at 0xb0070 | load | 4 | `abs 0xb00c0` | `-` |
| `str` at 0xb0078 | store | 4 | `abs 0x14a7c8` | `0x1` |
| `ldr` at 0xb007c | load | 4 | `abs 0xb00c4` | `-` |
| `str` at 0xb0088 | store | 4 | `sp -0x14` | `ret(0x22e50)` |
| `ldr` at 0xb008c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0098 | load | 4 | `abs 0xb00d8` | `-` |
| `str` at 0xb009c | store | 4 | `sp -0x28` | `0x128080` |
| `ldr` at 0xb00a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb00a4 | store | 4 | `sp -0x24` | `ret(0x22e50)` |
| `ldr` at 0xb00ac | load | 4 | `abs 0xb00dc` | `-` |
| `ldmdb` at 0xb00bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb00bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb00bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0030 | `(unknown callee) 0x229b0` | `0x14a7d4` | `0x128068` | `0x0` | `0x9` |
| `bl` at 0xb006c | `(unknown callee) 0x22480` | `0x14a7d0` | `0x128074` | `0xb0100` | `0x0` |
| `bl` at 0xb0080 | `(unknown callee) 0x22e50` | `0x14a7d4` | `?` | `?` | `?` |
| `bl` at 0xb00b8 | `(unknown callee) 0x21eb8` | `0x9` | `0x1280ac` | `0xbd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb001c | `[0x14a7c8] != 0x0` | `[0x14a7c8] == 0x0` |
| `beq` at 0xb0094 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb00bc | `r0` | `?` |

## `SysComSimStop`

- `SysComSimStop`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb00e0 to 0xb0100.
  Established. The absolute word assignment at `0x14a488` writes `0x0` (`str` at 0xb00f4). The
  return reports entry `r0` (`ldmdb` at 0xb00f8). Simulation-stop effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb00ec | load | 4 | `abs 0xb00fc` | `-` |
| `str` at 0xb00f4 | store | 4 | `abs 0x14a488` | `0x0` |
| `ldmdb` at 0xb00f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb00f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb00f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb00f8 | `r0` | `r0` |

## `imageSyncInit`

- `imageSyncInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb6a0 to 0xdb710.
  Established. The absolute word at `0x2adfb0` is read (`ldr` at 0xdb6bc); its zero value takes
  `0xdb6e4`, while nonzero falls through (`beq` at 0xdb6c4). The word assignment at `0x2b44c0` has
  an unknown value (`str` at 0xdb6ec). Image synchronization setup remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb6b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdb6b4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdb6b8 | load | 4 | `abs 0xdb700` | `-` |
| `ldr` at 0xdb6bc | load | 4 | `abs 0x2adfb0` | `-` |
| `ldr` at 0xdb6c8 | load | 4 | `abs 0xdb704` | `-` |
| `str` at 0xdb6cc | store | 4 | `sp -0x20` | `0x1359d4` |
| `ldr` at 0xdb6d4 | load | 4 | `abs 0xdb708` | `-` |
| `ldr` at 0xdb6e4 | load | 4 | `abs 0xdb70c` | `-` |
| `ldr` at 0xdb6e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdb6ec | store | 4 | `abs 0x2b44c0` | `?` |
| `ldr` at 0xdb6f0 | load | 4 | `abs 0xdb700` | `-` |
| `ldr` at 0xdb6f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdb6f8 | store | 4 | `abs 0x2adfb0` | `[sp-24]` |
| `ldmdb` at 0xdb6fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb6fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb6fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb6e0 | `(unknown callee) 0x21eb8` | `0x9` | `0x1359f4` | `0x35` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdb6c4 | `[0x2adfb0] == 0x0` | `[0x2adfb0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb6fc | `r0` | `?` |

## `imageSyncGetCfg`

- `imageSyncGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb710 to 0xdb74c.
  Established. The absolute word read uses `0x2b44c0` (`ldr` at 0xdb72c); its expression
  `[0x2b44c0]` is assigned through `r0 +0x0` (`str` at 0xdb730). The reported return value is
  `[0x2adfb0]` (`ldmdb` at 0xdb740). Configuration contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb720 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdb724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdb728 | load | 4 | `abs 0xdb744` | `-` |
| `ldr` at 0xdb72c | load | 4 | `abs 0x2b44c0` | `-` |
| `str` at 0xdb730 | store | 4 | `r0 +0x0` | `[0x2b44c0]` |
| `ldr` at 0xdb734 | load | 4 | `abs 0xdb748` | `-` |
| `ldr` at 0xdb738 | load | 4 | `abs 0x2adfb0` | `-` |
| `ldmdb` at 0xdb740 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb740 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb740 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb740 | `r0` | `[0x2adfb0]` |

## `imageSyncCount`

- `imageSyncCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb74c to 0xdb76c.
  Established. The word at `0x2adfb0` is read (`ldr` at 0xdb75c). The return reports `[0x2adfb0]`
  (`ldmdb` at 0xdb764). Synchronization count meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xdb758 | load | 4 | `abs 0xdb768` | `-` |
| `ldr` at 0xdb75c | load | 4 | `abs 0x2adfb0` | `-` |
| `ldmdb` at 0xdb764 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb764 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb764 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb764 | `r0` | `[0x2adfb0]` |

## `imageSyncClear`

- `imageSyncClear`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb85c to 0xdb914.
  Established. The unnamed `0xc87e8` call at `0xdb8bc` carries `[ret(0xdba10)]:1`,
  `[ret(0xdba10)+1]:1`, `0x1`, `[ret(0xdba10)+1]:1` (`bl` at 0xdb8bc); its zero result takes
  `0xdb8e0`, while nonzero falls through (`beq` at 0xdb8d0). Clear effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdb870 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdb874 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdb884 | store | 4 | `sp -0x18` | `ret(0xdba10)` |
| `ldr` at 0xdb888 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb894 | load | 4 | `abs 0xdb910` | `-` |
| `str` at 0xdb898 | store | 4 | `sp -0x20` | `0xfffffc10` |
| `ldr` at 0xdb8a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb8a4 | load | 1 | `ret(0xdba10) +0x0` | `-` |
| `ldr` at 0xdb8a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb8ac | load | 1 | `ret(0xdba10) +0x1` | `-` |
| `str` at 0xdb8c4 | store | 4 | `sp -0x1c` | `ret(0xc87e8)` |
| `ldr` at 0xdb8c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb8d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdb8d8 | store | 4 | `sp -0x20` | `ret(0xc87e8)` |
| `ldr` at 0xdb8e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb8e4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xdb8e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb8ec | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xdb904 | store | 4 | `sp -0x20` | `ret(0xc87e8)` |
| `ldr` at 0xdb908 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdb90c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb90c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb90c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb87c | `(unknown callee) 0xdba10` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdb8bc | `(unknown callee) 0xc87e8` | `[ret(0xdba10)]:1` | `[ret(0xdba10)+1]:1` | `0x1` | `[ret(0xdba10)+1]:1` |
| `bl` at 0xdb8fc | `(unknown callee) 0xc87e8` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `0x0` | `[[sp-24]+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb890 | `ret(0xdba10) != 0x0` | `ret(0xdba10) == 0x0` |
| `beq` at 0xdb8d0 | `ret(0xc87e8) == 0x0` | `ret(0xc87e8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb90c | `r0` | `?` |

## `imageSyncRead`

- `imageSyncRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb914 to 0xdb990.
  Established. The unnamed `0xc8914` call carries `[ret(0xdba10)]:1`, `[ret(0xdba10)+1]:1`,
  `[sp-24]`, `[ret(0xdba10)+1]:1` (`bl` at 0xdb978). The word assignment at `sp -0x20` writes
  `0xfffffc10` (`str` at 0xdb954). Read-result meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb928 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xdb92c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdb930 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdb940 | store | 4 | `sp -0x1c` | `ret(0xdba10)` |
| `ldr` at 0xdb944 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb950 | load | 4 | `abs 0xdb98c` | `-` |
| `str` at 0xdb954 | store | 4 | `sp -0x20` | `0xfffffc10` |
| `ldr` at 0xdb95c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdb960 | load | 1 | `ret(0xdba10) +0x0` | `-` |
| `ldr` at 0xdb964 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdb968 | load | 1 | `ret(0xdba10) +0x1` | `-` |
| `ldr` at 0xdb974 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdb980 | store | 4 | `sp -0x20` | `ret(0xc8914)` |
| `ldr` at 0xdb984 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdb988 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb988 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb988 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb938 | `(unknown callee) 0xdba10` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdb978 | `(unknown callee) 0xc8914` | `[ret(0xdba10)]:1` | `[ret(0xdba10)+1]:1` | `[sp-24]` | `[ret(0xdba10)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb94c | `ret(0xdba10) != 0x0` | `ret(0xdba10) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb988 | `r0` | `?` |

## `imageSyncName`

- `imageSyncName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb990 to 0xdba10.
  Established. The word read at `ret(0xdba10) +0x4` supplies `[ret(0xdba10)+4]` (`ldr` at 0xdb9d8).
  The word assignment at `sp -0x20` writes `0x135a1c` (`str` at 0xdb9f4). Synchronization name
  contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdb9a4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdb9a8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdb9b8 | store | 4 | `sp -0x18` | `ret(0xdba10)` |
| `ldr` at 0xdb9bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb9c8 | load | 4 | `abs 0xdba08` | `-` |
| `str` at 0xdb9cc | store | 4 | `sp -0x1c` | `0x135a14` |
| `ldr` at 0xdb9d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb9d8 | load | 4 | `ret(0xdba10) +0x4` | `-` |
| `str` at 0xdb9dc | store | 4 | `sp -0x20` | `[ret(0xdba10)+4]` |
| `ldr` at 0xdb9e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb9e4 | load | 4 | `ret(0xdba10) +0x4` | `-` |
| `ldr` at 0xdb9f0 | load | 4 | `abs 0xdba0c` | `-` |
| `str` at 0xdb9f4 | store | 4 | `sp -0x20` | `0x135a1c` |
| `ldr` at 0xdb9f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdb9fc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdba00 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdba04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdba04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdba04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb9b0 | `(unknown callee) 0xdba10` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb9c4 | `ret(0xdba10) != 0x0` | `ret(0xdba10) == 0x0` |
| `bne` at 0xdb9ec | `[ret(0xdba10)+4] != 0x0` | `[ret(0xdba10)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdba04 | `r0` | `?` |
