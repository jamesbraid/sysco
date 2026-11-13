# ARM front-panel and startup paths in 1.40.5

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
`vramSet`, `vramGet` and `l1ioInit` have unresolved computed transfers. Their access, call and
return lists cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/front-panel-startup-facts.txt) retain all tool sections.

## `frontpanelLEDInit`

- `frontpanelLEDInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52efc to 0x52f84.
  Established. The absolute word at `0x1490b4` receives an unknown value (`str` at 0x52f60). A
  separate word assignment at `0x1490b8` writes `[sp-24]` (`str` at 0x52f6c). The initial zero test
  takes `[0x1490b4] == 0x0` to `0x52f58`, and nonzero falls through (`beq` at 0x52f20). LED
  configuration meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52f0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52f10 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x52f14 | load | 4 | `abs 0x52f74` | `-` |
| `ldr` at 0x52f18 | load | 4 | `abs 0x1490b4` | `-` |
| `ldr` at 0x52f24 | load | 4 | `abs 0x52f78` | `-` |
| `str` at 0x52f28 | store | 4 | `sp -0x24` | `0x10bcc0` |
| `ldr` at 0x52f2c | load | 4 | `abs 0x52f74` | `-` |
| `ldr` at 0x52f30 | load | 4 | `abs 0x1490b4` | `-` |
| `str` at 0x52f34 | store | 4 | `sp -0x20` | `[0x1490b4]` |
| `ldr` at 0x52f38 | load | 4 | `abs 0x52f7c` | `-` |
| `ldr` at 0x52f3c | load | 4 | `abs 0x1490b8` | `-` |
| `str` at 0x52f40 | store | 4 | `sp -0x1c` | `[0x1490b8]` |
| `ldr` at 0x52f48 | load | 4 | `abs 0x52f80` | `-` |
| `ldr` at 0x52f58 | load | 4 | `abs 0x52f74` | `-` |
| `ldr` at 0x52f5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52f60 | store | 4 | `abs 0x1490b4` | `?` |
| `ldr` at 0x52f64 | load | 4 | `abs 0x52f7c` | `-` |
| `ldr` at 0x52f68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x52f6c | store | 4 | `abs 0x1490b8` | `[sp-24]` |
| `ldmdb` at 0x52f70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52f70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52f70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52f54 | `(unknown callee) 0x21eb8` | `0x9` | `0x10bcf4` | `0x3a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x52f20 | `[0x1490b4] == 0x0` | `[0x1490b4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52f70 | `r0` | `?` |

## `frontpanelLEDOn`

- `frontpanelLEDOn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52f84 to 0x52ff0.
  Established. The branch takes `r0 != 0x4` to `0x52fb4`, and falls through for `0x4` (`bne` at
  0x52fa4). The absolute word at `0x1490a8` is cleared (`str` at 0x52fbc). The unnamed `0x53280`
  call carries `[sp-20]` followed by three unknown candidates (`bl` at 0x52fd8). Hardware LED
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52f94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52f98 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x52f9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52fac | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x52fb4 | load | 4 | `abs 0x52fec` | `-` |
| `str` at 0x52fbc | store | 4 | `abs 0x1490a8` | `0x0` |
| `ldr` at 0x52fc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x52fd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x52fe0 | store | 4 | `sp -0x1c` | `ret(0x53280)` |
| `ldr` at 0x52fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x52fe8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52fe8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52fe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52fd0 | `(unknown callee) 0x52ff0` | `0xff` | `r1` | `0x0` | `[sp-24]` |
| `bl` at 0x52fd8 | `(unknown callee) 0x53280` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52fa4 | `r0 != 0x4` | `r0 == 0x4` |
| `bne` at 0x52fc8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52fe8 | `r0` | `?` |

## `frontpanelLEDOff`

- `frontpanelLEDOff`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52ff0 to 0x53044.
  Established. The absolute word at `0x1490a8` receives `0x0` (`str` at 0x53024). The unnamed
  `0x5338c` call carries `[sp-20]`, `r1`, `0x0`, `0x1490a8` (`bl` at 0x5302c); its result is
  assigned as a word at `sp -0x18` (`str` at 0x53034). The LED state interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53000 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x53004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53014 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5301c | load | 4 | `abs 0x53040` | `-` |
| `str` at 0x53024 | store | 4 | `abs 0x1490a8` | `0x0` |
| `ldr` at 0x53028 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53034 | store | 4 | `sp -0x18` | `ret(0x5338c)` |
| `ldr` at 0x53038 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5303c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5303c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5303c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5302c | `(unknown callee) 0x5338c` | `[sp-20]` | `r1` | `0x0` | `0x1490a8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5300c | `r0 != 0x4` | `r0 == 0x4` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5303c | `r0` | `?` |

## `frontpanelLEDBlink`

- `frontpanelLEDBlink`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53044 to 0x5312c.
  Established. The absolute four-byte assignment at `0x1490a8` writes `0x1` (`str` at 0x5307c). The
  byte at absolute `0x1490bc` receives `([sp-20]:1 & 0xff)` (`strb` at 0x53088). A later signed test
  takes `[sp-24] > 0x0` to `0x530d0`, and falls through at or below zero (`bgt` at 0x530c4).
  Blink timing and hardware meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53054 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x53058 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5305c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x53060 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53074 | load | 4 | `abs 0x53114` | `-` |
| `str` at 0x5307c | store | 4 | `abs 0x1490a8` | `0x1` |
| `ldr` at 0x53080 | load | 4 | `abs 0x53118` | `-` |
| `ldrb` at 0x53084 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x53088 | store | 1 | `abs 0x1490bc` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x5308c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5309c | load | 4 | `abs 0x5311c` | `-` |
| `ldr` at 0x530a0 | load | 4 | `abs 0x53120` | `-` |
| `ldr` at 0x530a4 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x530a8 | load | 1 | `[0x1490b4] +0x2` | `-` |
| `strb` at 0x530ac | store | 1 | `abs 0x1490bd` | `([[0x1490b4]+2]:1 & 0xff)` |
| `ldr` at 0x530b0 | load | 4 | `abs 0x53124` | `-` |
| `strb` at 0x530b8 | store | 1 | `abs 0x1490ad` | `0x1` |
| `ldr` at 0x530bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x530cc | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x530d0 | load | 4 | `abs 0x53128` | `-` |
| `ldr` at 0x530d4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x530dc | store | 1 | `abs 0x1490ac` | `?` |
| `str` at 0x530e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x530ec | load | 4 | `abs 0x53124` | `-` |
| `strb` at 0x530f4 | store | 1 | `abs 0x1490ad` | `0x0` |
| `ldr` at 0x530f8 | load | 4 | `abs 0x5311c` | `-` |
| `ldrb` at 0x530fc | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x53100 | store | 1 | `abs 0x1490bd` | `([sp-20]:1 & 0xff)` |
| `str` at 0x53108 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5310c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x53110 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53110 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53110 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53070 | `(unknown callee) 0x52ff0` | `0xff` | `r1` | `r2` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x53068 | `r2 != 0x0` | `r2 == 0x0` |
| `beq` at 0x53098 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x530c4 | `[sp-24] > 0x0 (signed, 32-bit)` | `[sp-24] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53110 | `r0` | `0x0` |

## `frontpanelLEDBlinkTask`

- `frontpanelLEDBlinkTask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5312c to 0x53280.
  Established. The absolute word at `0x1490b0` is assigned `[0x1490b0]-1` (`str` at 0x531f0). The
  unnamed `0x5338c` call carries `[0x1490bd]:1`, `[0x1490be]:1`, `0x1490bd`, `[0x1490bd]:1` (`bl` at
  0x53220). A later word assignment at `0x1434fc` writes `0x3` (`str` at 0x53240). Blink scheduling
  and LED effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x53138 | load | 4 | `abs 0x53260` | `-` |
| `ldr` at 0x5313c | load | 4 | `abs 0x1490a8` | `-` |
| `ldr` at 0x5314c | load | 4 | `abs 0x53264` | `-` |
| `ldr` at 0x53150 | load | 4 | `abs 0x1434fc` | `-` |
| `ldr` at 0x5315c | load | 4 | `abs 0x53268` | `-` |
| `ldrb` at 0x53160 | load | 1 | `abs 0x1490ad` | `-` |
| `ldr` at 0x5316c | load | 4 | `abs 0x5326c` | `-` |
| `ldr` at 0x53170 | load | 4 | `abs 0x1490b0` | `-` |
| `ldr` at 0x5317c | load | 4 | `abs 0x53270` | `-` |
| `ldr` at 0x53180 | load | 4 | `abs 0x53274` | `-` |
| `ldr` at 0x53184 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x53188 | load | 1 | `abs 0x1490bd` | `-` |
| `ldrb` at 0x5318c | load | 1 | `[0x1490b4] +0x2` | `-` |
| `ldr` at 0x53198 | load | 4 | `abs 0x53270` | `-` |
| `ldr` at 0x531a0 | load | 4 | `abs 0x53274` | `-` |
| `ldr` at 0x531a4 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x531ac | load | 1 | `[0x1490b4] +0x6` | `-` |
| `strb` at 0x531b0 | store | 1 | `abs 0x1490bd` | `([[0x1490b4]+6]:1 & 0xff)` |
| `ldr` at 0x531b8 | load | 4 | `abs 0x53270` | `-` |
| `ldr` at 0x531bc | load | 4 | `abs 0x53274` | `-` |
| `ldr` at 0x531c0 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x531c4 | load | 1 | `[0x1490b4] +0x2` | `-` |
| `strb` at 0x531c8 | store | 1 | `abs 0x1490bd` | `([[0x1490b4]+2]:1 & 0xff)` |
| `ldr` at 0x531cc | load | 4 | `abs 0x5326c` | `-` |
| `ldr` at 0x531d0 | load | 4 | `abs 0x53278` | `-` |
| `ldrb` at 0x531d4 | load | 1 | `abs 0x1490ac` | `-` |
| `str` at 0x531d8 | store | 4 | `abs 0x1490b0` | `[0x1490ac]:1` |
| `ldr` at 0x531e0 | load | 4 | `abs 0x5326c` | `-` |
| `ldr` at 0x531e4 | load | 4 | `abs 0x5326c` | `-` |
| `ldr` at 0x531e8 | load | 4 | `abs 0x1490b0` | `-` |
| `str` at 0x531f0 | store | 4 | `abs 0x1490b0` | `[0x1490b0]-1` |
| `ldr` at 0x531f4 | load | 4 | `abs 0x5327c` | `-` |
| `ldr` at 0x531f8 | load | 4 | `abs 0x53270` | `-` |
| `ldrb` at 0x531fc | load | 1 | `abs 0x1490be` | `-` |
| `ldrb` at 0x53200 | load | 1 | `abs 0x1490bd` | `-` |
| `ldr` at 0x53214 | load | 4 | `abs 0x53270` | `-` |
| `ldrb` at 0x53218 | load | 1 | `abs 0x1490bd` | `-` |
| `ldr` at 0x53228 | load | 4 | `abs 0x53270` | `-` |
| `ldrb` at 0x5322c | load | 1 | `abs 0x1490bd` | `-` |
| `ldr` at 0x53238 | load | 4 | `abs 0x53264` | `-` |
| `str` at 0x53240 | store | 4 | `abs 0x1434fc` | `0x3` |
| `ldr` at 0x53248 | load | 4 | `abs 0x53264` | `-` |
| `ldr` at 0x5324c | load | 4 | `abs 0x53264` | `-` |
| `ldr` at 0x53250 | load | 4 | `abs 0x1434fc` | `-` |
| `str` at 0x53258 | store | 4 | `abs 0x1434fc` | `[0x1434fc]-1` |
| `ldmdb` at 0x5325c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5325c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5325c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53220 | `(unknown callee) 0x5338c` | `[0x1490bd]:1` | `[0x1490be]:1` | `0x1490bd` | `[0x1490bd]:1` |
| `bl` at 0x53234 | `(unknown callee) 0x53280` | `[0x1490bd]:1` | `[0x1490be]:1` | `0x1490bd` | `[0x1490bd]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x53144 | `[0x1490a8] != 0x0` | `[0x1490a8] == 0x0` |
| `bge` at 0x53158 | `[0x1434fc] >= 0x0 (signed, 32-bit)` | `[0x1434fc] < 0x0 (signed, 32-bit)` |
| `beq` at 0x53168 | `[0x1490ad]:1 == 0x0` | `[0x1490ad]:1 != 0x0` |
| `bgt` at 0x53178 | `[0x1490b0] > 0x1 (signed, 32-bit)` | `[0x1490b0] <= 0x1 (signed, 32-bit)` |
| `bne` at 0x53194 | `[0x1490bd]:1 != [[0x1490b4]+2]:1` | `[0x1490bd]:1 == [[0x1490b4]+2]:1` |
| `beq` at 0x53210 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5325c | `r0` | `?` |

## `fp_led_on_raw`

- `fp_led_on_raw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53280 to 0x5338c.
  Established. The unnamed `0xd73b0` call carries `0x0` and three unknown candidates (`bl` at
  0x5332c); its zero result takes `0x5334c`, and nonzero falls through (`beq` at 0x53340). The byte
  assignment at absolute `0x1490be` has an unknown value (`strb` at 0x53360). Hardware on-state
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53290 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x53298 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5329c | load | 4 | `abs 0x53380` | `-` |
| `ldr` at 0x532a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x532a4 | load | 4 | `abs 0x1490b8` | `-` |
| `ldr` at 0x532b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x532bc | load | 4 | `abs 0x53384` | `-` |
| `ldr` at 0x532c0 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x532c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x532cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x532dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x532e4 | load | 4 | `abs 0x53384` | `-` |
| `ldr` at 0x532e8 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x532f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x532f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x532fc | load | 4 | `abs 0x53384` | `-` |
| `ldr` at 0x53300 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x53308 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5330c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53314 | load | 4 | `abs 0x53384` | `-` |
| `ldr` at 0x53318 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x53320 | load | 1 | `(unknown)` | `-` |
| `str` at 0x53334 | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x53338 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53344 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53348 | store | 4 | `sp -0x18` | `ret(0xd73b0)` |
| `ldr` at 0x5334c | load | 4 | `abs 0x53388` | `-` |
| `ldr` at 0x53350 | load | 4 | `abs 0x53388` | `-` |
| `ldrb` at 0x53354 | load | 1 | `abs 0x1490be` | `-` |
| `ldrb` at 0x53358 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x53360 | store | 1 | `abs 0x1490be` | `?` |
| `ldr` at 0x53364 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5336c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x53374 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5337c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5337c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5337c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5332c | `(unknown callee) 0xd73b0` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x532ac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x532d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x53340 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5337c | `r0` | `[sp-24]` |

## `fp_led_off_raw`

- `fp_led_off_raw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5338c to 0x53488.
  Established. The unnamed `0xd73b0` call carries `0x0`, unknown, unknown, `0x0` (`bl` at 0x53424);
  its nonzero result falls through, while zero takes `0x53444` (`beq` at 0x53438). The absolute byte
  at `0x1490be` receives an unknown value (`strb` at 0x5345c). Hardware off-state effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5339c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x533a4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x533a8 | load | 4 | `abs 0x5347c` | `-` |
| `ldr` at 0x533ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x533b0 | load | 4 | `abs 0x1490b8` | `-` |
| `ldr` at 0x533c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x533c8 | load | 4 | `abs 0x53480` | `-` |
| `ldr` at 0x533cc | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x533d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x533d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x533e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x533f0 | load | 4 | `abs 0x53480` | `-` |
| `ldr` at 0x533f4 | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x533fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x53400 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53408 | load | 4 | `abs 0x53480` | `-` |
| `ldr` at 0x5340c | load | 4 | `abs 0x1490b4` | `-` |
| `ldrb` at 0x53414 | load | 1 | `(unknown)` | `-` |
| `str` at 0x5342c | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x53430 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5343c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53440 | store | 4 | `sp -0x18` | `ret(0xd73b0)` |
| `ldr` at 0x53444 | load | 4 | `abs 0x53484` | `-` |
| `ldr` at 0x53448 | load | 4 | `abs 0x53484` | `-` |
| `ldrb` at 0x5344c | load | 1 | `sp -0x14` | `-` |
| `ldrb` at 0x53454 | load | 1 | `abs 0x1490be` | `-` |
| `strb` at 0x5345c | store | 1 | `abs 0x1490be` | `?` |
| `ldr` at 0x53460 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x53468 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x53470 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x53478 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53478 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53424 | `(unknown callee) 0xd73b0` | `0x0` | `?` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x533b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x533e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x53438 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53478 | `r0` | `[sp-24]` |

## `t1_displayInit`

- `t1_displayInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbac0 to 0xdbb64.
  Established. Included byte assignments retain `0xc` at `r0 +0x2` (`strb` at 0xdbadc) and `0x20` at
  `[sp-20] +0x5` (`strb` at 0xdbb00). Four-byte assignments write `0xdbb64` at `[sp-20] +0xc` (`str`
  at 0xdbb18) and `[sp-20]` at absolute `0x2adfb4` (`str` at 0xdbb48). Device field roles and stored
  pointer consumers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbad0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdbad4 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdbadc | store | 1 | `r0 +0x2` | `0xc` |
| `ldr` at 0xdbae0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdbae8 | store | 1 | `[sp-20] +0x3` | `0x0` |
| `ldr` at 0xdbaec | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdbaf4 | store | 1 | `[sp-20] +0x4` | `0x0` |
| `ldr` at 0xdbaf8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xdbb00 | store | 1 | `[sp-20] +0x5` | `0x20` |
| `ldr` at 0xdbb04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbb08 | load | 4 | `abs 0xdbb50` | `-` |
| `str` at 0xdbb0c | store | 4 | `[sp-20] +0x8` | `0x2b44c9` |
| `ldr` at 0xdbb10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbb14 | load | 4 | `abs 0xdbb54` | `-` |
| `str` at 0xdbb18 | store | 4 | `[sp-20] +0xc` | `0xdbb64` |
| `ldr` at 0xdbb1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbb20 | load | 4 | `abs 0xdbb58` | `-` |
| `str` at 0xdbb24 | store | 4 | `[sp-20] +0x10` | `0xdbce0` |
| `ldr` at 0xdbb28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdbb30 | store | 4 | `[sp-20] +0x14` | `0x0` |
| `ldr` at 0xdbb34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbb38 | load | 4 | `abs 0xdbb5c` | `-` |
| `str` at 0xdbb3c | store | 4 | `[sp-20] +0x18` | `0xdbe9c` |
| `ldr` at 0xdbb40 | load | 4 | `abs 0xdbb60` | `-` |
| `ldr` at 0xdbb44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdbb48 | store | 4 | `abs 0x2adfb4` | `[sp-20]` |
| `ldmdb` at 0xdbb4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdbb4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdbb4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdbb4c | `r0` | `r0` |

## `pcf2119_Init_stub`

- `pcf2119_Init_stub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8dd8 to 0xc8df0.
  Established. A four-byte stack assignment retains entry `r0` at `sp -0x14` (`str` at 0xc8de8). The
  epilogue returns with entry `r0` preserved (`ldmdb` at 0xc8dec). No display operation is
  established by this body, whose role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8de8 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xc8dec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8dec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8dec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8dec | `r0` | `r0` |

## `vramInit`

- `vramInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ed64 to 0x5efe8.
  Established. The absolute word at `0x149170` receives `[0x149170]+1` (`str` at 0x5edb8). The
  unnamed `0x1b130` call carries `0x0`, `0x8000`, `0x3`, `0x1` (`bl` at 0x5ee60). Included absolute
  word assignments retain `[sp-28]` at `0x2b1640` (`str` at 0x5eed8) and `[0x2b1640]` at `0x149114`
  (`str` at 0x5eef8). Storage mapping and initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ed78 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5ed7c | load | 4 | `abs 0x5efb8` | `-` |
| `ldr` at 0x5ed80 | load | 4 | `abs 0x149170` | `-` |
| `ldr` at 0x5ed8c | load | 4 | `abs 0x5efbc` | `-` |
| `str` at 0x5ed90 | store | 4 | `sp -0x34` | `0x10ce30` |
| `ldr` at 0x5ed98 | load | 4 | `abs 0x5efc0` | `-` |
| `ldr` at 0x5eda8 | load | 4 | `abs 0x5efb8` | `-` |
| `ldr` at 0x5edac | load | 4 | `abs 0x5efb8` | `-` |
| `ldr` at 0x5edb0 | load | 4 | `abs 0x149170` | `-` |
| `str` at 0x5edb8 | store | 4 | `abs 0x149170` | `[0x149170]+1` |
| `ldr` at 0x5edbc | load | 4 | `abs 0x5efc4` | `-` |
| `ldr` at 0x5edcc | load | 4 | `abs 0x5efc8` | `-` |
| `ldr` at 0x5edd0 | load | 4 | `abs 0x5efcc` | `-` |
| `str` at 0x5eddc | store | 4 | `sp -0x18` | `ret(0x1b400)` |
| `ldr` at 0x5ede0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5edf4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0x5ee0c | load | 4 | `abs 0x5efd0` | `-` |
| `str` at 0x5ee10 | store | 4 | `sp -0x34` | `0x10ce70` |
| `ldr` at 0x5ee14 | load | 4 | `abs 0x5efd4` | `-` |
| `str` at 0x5ee18 | store | 4 | `sp -0x30` | `0x10ce94` |
| `str` at 0x5ee1c | store | 4 | `sp -0x2c` | `ret(0x21704)` |
| `ldr` at 0x5ee20 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0x5ee24 | store | 4 | `sp -0x28` | `[ret(0x1b148)]` |
| `ldr` at 0x5ee2c | load | 4 | `abs 0x5efc0` | `-` |
| `ldr` at 0x5ee40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5ee44 | store | 4 | `sp -0x34` | `ret(0x1b400)` |
| `str` at 0x5ee4c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x5ee64 | store | 4 | `sp -0x1c` | `ret(0x1b130)` |
| `ldr` at 0x5ee68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ee7c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0x5ee94 | load | 4 | `abs 0x5efd8` | `-` |
| `str` at 0x5ee98 | store | 4 | `sp -0x34` | `0x10cea0` |
| `ldr` at 0x5ee9c | load | 4 | `abs 0x5efd4` | `-` |
| `str` at 0x5eea0 | store | 4 | `sp -0x30` | `0x10ce94` |
| `str` at 0x5eea4 | store | 4 | `sp -0x2c` | `ret(0x21704)` |
| `ldr` at 0x5eea8 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0x5eeac | store | 4 | `sp -0x28` | `[ret(0x1b148)]` |
| `ldr` at 0x5eeb4 | load | 4 | `abs 0x5efc0` | `-` |
| `ldr` at 0x5eec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5eed0 | load | 4 | `abs 0x5efdc` | `-` |
| `ldr` at 0x5eed4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5eed8 | store | 4 | `abs 0x2b1640` | `[sp-28]` |
| `ldr` at 0x5eedc | load | 4 | `abs 0x5efe0` | `-` |
| `ldr` at 0x5eee0 | load | 4 | `abs 0x149114` | `-` |
| `ldr` at 0x5eeec | load | 4 | `abs 0x5efe0` | `-` |
| `ldr` at 0x5eef0 | load | 4 | `abs 0x5efdc` | `-` |
| `ldr` at 0x5eef4 | load | 4 | `abs 0x2b1640` | `-` |
| `str` at 0x5eef8 | store | 4 | `abs 0x149114` | `[0x2b1640]` |
| `ldr` at 0x5ef18 | load | 4 | `abs 0x5efe0` | `-` |
| `ldr` at 0x5ef1c | load | 4 | `abs 0x149114` | `-` |
| `ldr` at 0x5ef24 | load | 4 | `abs 0x5efe0` | `-` |
| `ldr` at 0x5ef2c | load | 4 | `abs 0x149114` | `-` |
| `ldr` at 0x5ef48 | load | 4 | `abs 0x5efe4` | `-` |
| `ldrb` at 0x5ef4c | load | 1 | `abs 0x149118` | `-` |
| `ldr` at 0x5ef58 | load | 4 | `abs 0x5efe4` | `-` |
| `strb` at 0x5ef60 | store | 1 | `abs 0x149118` | `0x0` |
| `ldr` at 0x5ef64 | load | 4 | `abs 0x5efc4` | `-` |
| `ldr` at 0x5ef68 | load | 4 | `abs 0x5efe4` | `-` |
| `ldrb` at 0x5ef6c | load | 1 | `abs 0x149118` | `-` |
| `ldr` at 0x5ef74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ef84 | load | 4 | `abs 0x5efc4` | `-` |
| `ldr` at 0x5ef88 | load | 4 | `abs 0x5efe4` | `-` |
| `ldrb` at 0x5ef8c | load | 1 | `abs 0x149118` | `-` |
| `ldr` at 0x5ef94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5ef9c | load | 4 | `abs 0x5efe4` | `-` |
| `ldr` at 0x5efa0 | load | 4 | `abs 0x5efe4` | `-` |
| `ldrb` at 0x5efa4 | load | 1 | `abs 0x149118` | `-` |
| `strb` at 0x5efac | store | 1 | `abs 0x149118` | `([0x149118]:1+1 & 0xff)` |
| `ldmdb` at 0x5efb4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5efb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5efb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5efb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5eda4 | `(unknown callee) 0x21eb8` | `0x9` | `0x10ce50` | `0x71` | `0x0` |
| `bl` at 0x5edc8 | `(unknown callee) 0x1b4cc` | `0x149174` | `0x0` | `0x20` | `[0x149170]+1` |
| `bl` at 0x5edd4 | `(unknown callee) 0x1b400` | `0x10ce64` | `0x1002` | `?` | `?` |
| `bl` at 0x5edec | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0x5edf8 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0x5ee00 | `(unknown callee) 0x1b148` | `ret(0x21704)` | `?` | `?` | `?` |
| `bl` at 0x5ee38 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ce50` | `0x84` | `0x0` |
| `bl` at 0x5ee60 | `(unknown callee) 0x1b130` | `0x0` | `0x8000` | `0x3` | `0x1` |
| `bl` at 0x5ee74 | `(unknown callee) 0x1b148` | `ret(0x1b130)` | `?` | `?` | `ret(0x1b130)` |
| `bl` at 0x5ee80 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0x5ee88 | `(unknown callee) 0x1b148` | `ret(0x21704)` | `?` | `?` | `?` |
| `bl` at 0x5eec0 | `(unknown callee) 0x21eb8` | `0x3` | `0x10ce50` | `0x8a` | `0x0` |
| `bl` at 0x5eec8 | `(unknown callee) 0x1b094` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5eefc | `(unknown callee) 0x20cf0` | `ret(0x1b130)` | `?` | `?` | `?` |
| `bl` at 0x5ef14 | `(unknown callee) 0x5efe8` | `?` | `?` | `?` | `?` |
| `bl` at 0x5ef34 | `(unknown callee) 0x1b394` | `[0x149114]+128` | `[0x149114]` | `0x80` | `[0x149114]+128` |
| `bl` at 0x5ef38 | `(unknown callee) 0x20cf0` | `ret(0x1b394)` | `?` | `?` | `?` |
| `bl` at 0x5ef98 | `(unknown callee) 0x202d0` | `?` | `?` | `0x149174` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ed88 | `[0x149170] == 0x0` | `[0x149170] != 0x0` |
| `bge` at 0x5ede8 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `bne` at 0x5ee70 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5eee8 | `[0x149114] != 0x0` | `[0x149114] == 0x0` |
| `beq` at 0x5ef44 | `ret(0x20cf0) == 0x0` | `ret(0x20cf0) != 0x0` |
| `beq` at 0x5ef54 | `[0x149118]:1 == 0x0` | `[0x149118]:1 != 0x0` |
| `bne` at 0x5ef7c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5efb4 | `r0` | `?` |

## `vramSet`

- `vramSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f160 to 0x5f314.
  Established. The computed transfer takes unsigned `r0-1 <= 0x30` to an unresolved target, and
  falls through above that bound (`ldrls` at 0x5f184). Included four-byte stack assignments retain
  `r0` at `sp -0x18` (`str` at 0x5f170) and `r1` at `sp -0x1c` (`str` at 0x5f174). The known return
  preserves entry `r0` (`ldmdb` at 0x5f30c). Selector-specific state writes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f170 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5f174 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x5f178 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x5f184 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x5f30c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5f30c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f30c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f30c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x5f184 | `r0-1 <= 0x30 (unsigned, 32-bit)` | `r0-1 > 0x30 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f30c | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5f184).

## `vramGet`

- `vramGet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f314 to 0x5f498.
  Established. The selector takes unsigned `r0-1 <= 0x30` to an unresolved destination, and falls
  through above that bound (`ldrls` at 0x5f334). The known fallback clears the word at `sp -0x18`
  (`str` at 0x5f488) and reports `0x0` at the return (`ldmdb` at 0x5f490). Selector-specific reads
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f324 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5f328 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x5f334 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5f488 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5f48c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5f490 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f490 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f490 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x5f334 | `r0-1 <= 0x30 (unsigned, 32-bit)` | `r0-1 > 0x30 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f490 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5f334).

## `l1ioInit`

- `l1ioInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x652a8 to 0x65b94.
  Established. The unnamed `0x1b400` call carries `0x10d984`, `0x802`, unknown, `ret(0x20cf0)` (`bl`
  at 0x65328); its signed nonnegative result takes `0x65368`, while a negative result falls through
  (`bge` at 0x6533c). The selector takes unsigned `? <= 0x51` to an unresolved destination, and
  falls through above that bound (`ldrls` at 0x65390). Controller setup effects beyond known edges
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x652b8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x652c0 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x652c8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x652d0 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x652d8 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x652e0 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x652e8 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x652f0 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x652f8 | store | 4 | `sp -0x64` | `0x0` |
| `str` at 0x65300 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x65304 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65320 | load | 4 | `abs 0x65b18` | `-` |
| `ldr` at 0x65324 | load | 4 | `abs 0x65b1c` | `-` |
| `str` at 0x65330 | store | 4 | `sp -0x6c` | `ret(0x1b400)` |
| `ldr` at 0x65334 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x65348 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0x65354 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x6535c | load | 4 | `abs 0x65b20` | `-` |
| `ldr` at 0x65368 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x65374 | store | 4 | `sp -0x70` | `0x0` |
| `ldrls` at 0x65390 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6557c | load | 4 | `abs 0x65b48` | `-` |
| `str` at 0x65580 | store | 4 | `sp -0x80` | `0x10db00` |
| `ldr` at 0x65584 | load | 4 | `abs 0x65b4c` | `-` |
| `str` at 0x65588 | store | 4 | `sp -0x7c` | `0x10db38` |
| `ldr` at 0x65590 | load | 4 | `abs 0x65b50` | `-` |
| `str` at 0x655a4 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x65b10 | load | 4 | `sp -0x70` | `-` |
| `ldmdb` at 0x65b14 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x65b14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65b14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65b14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x65310 | `(unknown callee) 0x20cf0` | `r0` | `r1` | `r2` | `[sp-24]` |
| `bl` at 0x65328 | `(unknown callee) 0x1b400` | `0x10d984` | `0x802` | `?` | `ret(0x20cf0)` |
| `bl` at 0x65340 | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0x65358 | `(unknown callee) 0x1b094` | `[sp-108]` | `?` | `?` | `[ret(0x1b148)]` |
| `bl` at 0x65360 | `(unknown callee) 0x2079c` | `0x10d994` | `?` | `?` | `?` |
| `bl` at 0x6536c | `(unknown callee) 0x1b094` | `?` | `?` | `?` | `?` |
| `bl` at 0x6537c | `(unknown callee) 0x4f4d0` | `?` | `?` | `?` | `?` |
| `bl` at 0x6559c | `(unknown callee) 0x21eb8` | `0x4` | `0x10db44` | `0xa3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6530c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x6531c | `ret(0x20cf0) == 0x0` | `ret(0x20cf0) != 0x0` |
| `bge` at 0x6533c | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `bne` at 0x65350 | `[ret(0x1b148)] != 0x13` | `[ret(0x1b148)] == 0x13` |
| `ldrls` at 0x65390 | `? <= 0x51 (unsigned, 32-bit)` | `? > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65b14 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x65390).

## `main_task`

- `main_task`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65c18 to 0x65df0.
  Established. The unnamed `0xca358` call carries `0x1`, `0x2`, `0x9600`, unknown (`bl` at 0x65c94).
  An indirect call has an unknown target and candidates `ret(0xc9170)`, unknown, unknown,
  `[0x14a710]` (`mov` at 0x65d50). The later zero result of unnamed `0x20cf0` takes the return path,
  and nonzero falls through (`beq` at 0x65db8). Startup effects and callee roles beyond the printed
  candidates remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x65c28 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x65c2c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x65c34 | store | 4 | `sp -0x20` | `0x2` |
| `str` at 0x65c3c | store | 4 | `sp -0x24` | `0x6` |
| `str` at 0x65c44 | store | 4 | `sp -0x30` | `0x1e` |
| `ldr` at 0x65c4c | load | 4 | `abs 0x65dd0` | `-` |
| `ldr` at 0x65c50 | load | 4 | `abs 0x65dd4` | `-` |
| `ldr` at 0x65c60 | load | 4 | `abs 0x65dd8` | `-` |
| `ldr` at 0x65c64 | load | 4 | `abs 0x2af280` | `-` |
| `ldr` at 0x65c70 | load | 4 | `abs 0x65dd8` | `-` |
| `ldr` at 0x65c74 | load | 4 | `abs 0x2af280` | `-` |
| `ldr` at 0x65cb8 | load | 4 | `abs 0x65dd8` | `-` |
| `ldr` at 0x65cbc | load | 4 | `abs 0x2af284` | `-` |
| `ldr` at 0x65d1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65d34 | load | 4 | `abs 0x65ddc` | `-` |
| `ldr` at 0x65d38 | load | 4 | `abs 0x14a710` | `-` |
| `ldr` at 0x65d44 | load | 4 | `abs 0x65ddc` | `-` |
| `ldr` at 0x65d48 | load | 4 | `abs 0x14a710` | `-` |
| `ldr` at 0x65d78 | load | 4 | `abs 0x65de0` | `-` |
| `ldr` at 0x65d88 | load | 4 | `abs 0x65de4` | `-` |
| `ldr` at 0x65da0 | load | 4 | `abs 0x65de8` | `-` |
| `ldr` at 0x65dc0 | load | 4 | `abs 0x65dec` | `-` |
| `ldmdb` at 0x65dcc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x65dcc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65dcc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65dcc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x65c48 | `(unknown callee) 0x1bcf4` | `r0` | `r1` | `r2` | `0x1e` |
| `bl` at 0x65c54 | `(unknown callee) 0x2079c` | `0x10dcd4` | `0x10dce4` | `?` | `?` |
| `bl` at 0x65c58 | `(unknown callee) 0x5ed64` | `ret(0x2079c)` | `?` | `?` | `?` |
| `bl` at 0x65c5c | `(unknown callee) 0x4ebf0` | `ret(0x5ed64)` | `?` | `?` | `?` |
| `mov` at 0x65c7c | `(unknown callee) (unknown target)` | `ret(0x4ebf0)` | `?` | `?` | `[0x2af280]` |
| `bl` at 0x65c84 | `(unknown callee) 0x652a8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x65c94 | `(unknown callee) 0xca358` | `0x1` | `0x2` | `0x9600` | `?` |
| `bl` at 0x65c9c | `(unknown callee) 0x23bd8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x65ca8 | `(unknown callee) 0x21e3c` | `0x0` | `0x0` | `?` | `?` |
| `bl` at 0x65cb0 | `(unknown callee) 0xcb804` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x65cb4 | `(unknown callee) 0xe3ac8` | `ret(0xcb804)` | `?` | `?` | `?` |
| `mov` at 0x65cc4 | `(unknown callee) (unknown target)` | `ret(0xe3ac8)` | `?` | `?` | `[0x2af284]` |
| `bl` at 0x65cc8 | `(unknown callee) 0x5116c` | `ret(unknown)` | `?` | `?` | `?` |
| `bl` at 0x65ccc | `(unknown callee) 0x20e34` | `ret(0x5116c)` | `?` | `?` | `?` |
| `bl` at 0x65cd4 | `(unknown callee) 0x1f26c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x65cec | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x65d00 | `(unknown callee) 0xb5d64` | `?` | `?` | `?` | `?` |
| `bl` at 0x65d04 | `(unknown callee) 0x88a60` | `ret(0xb5d64)` | `?` | `?` | `?` |
| `bl` at 0x65d08 | `(unknown callee) 0x99514` | `ret(0x88a60)` | `?` | `?` | `?` |
| `bl` at 0x65d0c | `(unknown callee) 0xa3938` | `ret(0x99514)` | `?` | `?` | `?` |
| `bl` at 0x65d10 | `(unknown callee) 0xb1998` | `ret(0xa3938)` | `?` | `?` | `?` |
| `bl` at 0x65d14 | `(unknown callee) 0x6963c` | `ret(0xb1998)` | `?` | `?` | `?` |
| `bl` at 0x65d18 | `(unknown callee) 0x6b180` | `ret(0x6963c)` | `?` | `?` | `?` |
| `bl` at 0x65d20 | `(unknown callee) 0x66ba0` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x65d24 | `(unknown callee) 0x7b1b0` | `ret(0x66ba0)` | `?` | `?` | `?` |
| `bl` at 0x65d28 | `(unknown callee) 0xe931c` | `ret(0x7b1b0)` | `?` | `?` | `?` |
| `bl` at 0x65d2c | `(unknown callee) 0xa5940` | `ret(0xe931c)` | `?` | `?` | `?` |
| `bl` at 0x65d30 | `(unknown callee) 0xc9170` | `ret(0xa5940)` | `?` | `?` | `?` |
| `mov` at 0x65d50 | `(unknown callee) (unknown target)` | `ret(0xc9170)` | `?` | `?` | `[0x14a710]` |
| `bl` at 0x65d54 | `(unknown callee) 0x65df0` | `?` | `?` | `?` | `?` |
| `bl` at 0x65d58 | `(unknown callee) 0x77590` | `ret(0x65df0)` | `?` | `?` | `?` |
| `bl` at 0x65d5c | `(unknown callee) 0x83b98` | `ret(0x77590)` | `?` | `?` | `?` |
| `bl` at 0x65d60 | `(unknown callee) 0x4f4d0` | `ret(0x83b98)` | `?` | `?` | `?` |
| `bl` at 0x65d80 | `(unknown callee) 0x51b28` | `0x31` | `0x10dcec` | `0x2` | `?` |
| `bl` at 0x65d90 | `(unknown callee) 0x51b28` | `0x32` | `0x10dcfc` | `0x2` | `?` |
| `bl` at 0x65d98 | `(unknown callee) 0x6a384` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0x65da8 | `(unknown callee) 0x51b28` | `0x32` | `0x10dd0c` | `0x0` | `?` |
| `bl` at 0x65dac | `(unknown callee) 0x20cf0` | `ret(0x51b28)` | `?` | `?` | `?` |
| `bl` at 0x65dc8 | `(unknown callee) 0x51b28` | `0x32` | `0x10dd18` | `0x1` | `ret(0x20cf0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x65c6c | `[0x2af280] == 0x0` | `[0x2af280] != 0x0` |
| `beq` at 0x65d40 | `[0x14a710] == 0x0` | `[0x14a710] != 0x0` |
| `bne` at 0x65d70 | `? != 0x3f` | `? == 0x3f` |
| `beq` at 0x65db8 | `ret(0x20cf0) == 0x0` | `ret(0x20cf0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65dcc | `r0` | `?` |

## `networkInit`

- `networkInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x246f4 to 0x2474c.
  Established. The unnamed `0x1f26c` call carries `0x34`, `r1`, `r2`, `r3` (`bl` at 0x24704); its
  zero result takes `0x24730`, and nonzero falls through (`beq` at 0x24710). The absolute word at
  `0x1430f4` is cleared (`str` at 0x2473c). Network initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24714 | load | 4 | `abs 0x24744` | `-` |
| `ldr` at 0x24734 | load | 4 | `abs 0x24748` | `-` |
| `str` at 0x2473c | store | 4 | `abs 0x1430f4` | `0x0` |
| `ldmdb` at 0x24740 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24740 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24740 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24704 | `(unknown callee) 0x1f26c` | `0x34` | `r1` | `r2` | `r3` |
| `bl` at 0x24718 | `(unknown callee) 0x2079c` | `0xff468` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x24724 | `(unknown callee) 0x1d41c` | `0x34` | `0x0` | `?` | `?` |
| `bl` at 0x24728 | `(unknown callee) 0x2474c` | `ret(0x1d41c)` | `?` | `?` | `?` |
| `bl` at 0x24730 | `(unknown callee) 0x2474c` | `ret(0x1f26c)` | `?` | `?` | `ret(0x1f26c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24710 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24740 | `r0` | `ret(0x2474c)` |
