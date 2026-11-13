# ARM monitor, panel and link-control paths in 1.38.4

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
The temperature-limit and temperature-read selectors have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/monitor-panel-link-control-facts.txt) retain all tool sections.

## `monitorSetTempLimits`

- `monitorSetTempLimits`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xccb28 to 0xcce10.
  Established. The byte test takes the selector path for `[ret(0xce238)]:1 != 0x0` and falls through
  on equality to store word `0xfffffc11` at `sp -0x2c` (`bne` at 0xccba0, `str` at 0xccba8). The
  computed transfer takes an unresolved edge for `[[sp-28]]:1-1 <= 0x4` unsigned and falls through
  above that bound (`ldrls` at 0xccbc0); its known default stores word `0xfffff830` at the same
  stack target (`str` at 0xccd58). Excluded limit-setting cases, threshold units and the joined r0
  return remain unknown (`ldmdb` at 0xcce04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xccb38 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xccb40 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xccb48 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xccb50 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xccb58 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xccb60 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0xccb64 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccb68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xccb78 | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldrb` at 0xccb7c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccb80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xccb90 | store | 4 | `sp -0x20` | `ret(0xce238)` |
| `ldr` at 0xccb94 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xccb98 | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xccba4 | load | 4 | `abs 0xcce08` | `-` |
| `str` at 0xccba8 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xccbb0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xccbb4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xccbc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xccd54 | load | 4 | `abs 0xcce0c` | `-` |
| `str` at 0xccd58 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xcce00 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xcce04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcce04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcce04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccb70 | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xccb88 | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xccba0 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `ldrls` at 0xccbc0 | `[[sp-28]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcce04 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xccbc0).

## `monitorReadTemp`

- `monitorReadTemp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd130 to 0xcd334.
  Established. The opening byte store writes `0x0` at entry `r3 +0x0` (`strb` at 0xcd160). The later
  call to `0xce238` has `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1` (`bl` at 0xcd188). The computed
  transfer takes an unresolved edge for `[[sp-32]]:1-1 <= 0x4` unsigned and falls through for
  greater (`ldrls` at 0xcd1c0). The known default stores word `0xfffff830` at `sp -0x2c` (`str` at
  0xcd2dc). Excluded reads, temperature units and the merged return remain unproved (`ldmdb` at
  0xcd328).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd140 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcd144 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xcd14c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd154 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xcd158 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xcd160 | store | 1 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xcd164 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd168 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd178 | store | 4 | `sp -0x20` | `ret(0xce118)` |
| `ldrb` at 0xcd17c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd180 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd190 | store | 4 | `sp -0x24` | `ret(0xce238)` |
| `ldr` at 0xcd194 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcd198 | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xcd1a4 | load | 4 | `abs 0xcd32c` | `-` |
| `str` at 0xcd1a8 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xcd1b0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcd1b4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0xcd1c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcd2d8 | load | 4 | `abs 0xcd330` | `-` |
| `str` at 0xcd2dc | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xcd324 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xcd328 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd328 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd328 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd170 | `(unknown callee) 0xce118` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xcd188 | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcd1a0 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `ldrls` at 0xcd1c0 | `[[sp-32]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd328 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcd1c0).

## `frontpanelLEDBlink`

- `frontpanelLEDBlink`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52830 to 0x52918.
  Established. A word store writes `0x1` at `0x148328`, and a byte store writes `([sp-20]:1 & 0xff)`
  at `0x14833c` (`str` at 0x52868, `strb` at 0x52874). Separate arms write byte `0x1` or byte `0x0`
  at `0x14832d` (`strb` at 0x528a4, `strb` at 0x528e0). The byte store at `0x14832c` has an unknown
  value (`strb` at 0x528c8). The r0 return is `0x0` (`ldmdb` at 0x528fc). Physical LED behavior,
  selected colors and timing units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x52840 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x52844 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x52848 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x5284c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52860 | load | 4 | `abs 0x52900` | `-` |
| `str` at 0x52868 | store | 4 | `abs 0x148328` | `0x1` |
| `ldr` at 0x5286c | load | 4 | `abs 0x52904` | `-` |
| `ldrb` at 0x52870 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x52874 | store | 1 | `abs 0x14833c` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x52878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x52888 | load | 4 | `abs 0x52908` | `-` |
| `ldr` at 0x5288c | load | 4 | `abs 0x5290c` | `-` |
| `ldr` at 0x52890 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52894 | load | 1 | `[0x148334] +0x2` | `-` |
| `strb` at 0x52898 | store | 1 | `abs 0x14833d` | `([[0x148334]+2]:1 & 0xff)` |
| `ldr` at 0x5289c | load | 4 | `abs 0x52910` | `-` |
| `strb` at 0x528a4 | store | 1 | `abs 0x14832d` | `0x1` |
| `ldr` at 0x528a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x528b8 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x528bc | load | 4 | `abs 0x52914` | `-` |
| `ldr` at 0x528c0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x528c8 | store | 1 | `abs 0x14832c` | `?` |
| `str` at 0x528d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x528d8 | load | 4 | `abs 0x52910` | `-` |
| `strb` at 0x528e0 | store | 1 | `abs 0x14832d` | `0x0` |
| `ldr` at 0x528e4 | load | 4 | `abs 0x52908` | `-` |
| `ldrb` at 0x528e8 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x528ec | store | 1 | `abs 0x14833d` | `([sp-20]:1 & 0xff)` |
| `str` at 0x528f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x528f8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x528fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x528fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x528fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5285c | `(unknown callee) 0x527dc` | `0xff` | `r1` | `r2` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52854 | `r2 != 0x0` | `r2 == 0x0` |
| `beq` at 0x52884 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x528b0 | `[sp-24] > 0x0 (signed, 32-bit)` | `[sp-24] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x528fc | `r0` | `0x0` |

## `frontpanelLEDBlinkTask`

- `frontpanelLEDBlinkTask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x52918 to 0x52a6c.
  Established. The signed branch takes the decrement arm for `[0x1427f0] >= 0x0` and falls through
  for less (`bge` at 0x52944). That arm stores word `[0x1427f0]-1` at `0x1427f0` (`str` at 0x52a44).
  Other paths include byte stores of `([[0x148334]+6]:1 & 0xff)` and `([[0x148334]+2]:1 & 0xff)` at
  `0x14833d` (`strb` at 0x5299c, `strb` at 0x529b4). The call to `0x52b78` has `[0x14833d]:1`,
  `[0x14833e]:1`, `0x14833d`, `[0x14833d]:1` (`bl` at 0x52a0c). Physical blinking and the joined r0
  value remain unidentified (`ldmdb` at 0x52a48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x52924 | load | 4 | `abs 0x52a4c` | `-` |
| `ldr` at 0x52928 | load | 4 | `abs 0x148328` | `-` |
| `ldr` at 0x52938 | load | 4 | `abs 0x52a50` | `-` |
| `ldr` at 0x5293c | load | 4 | `abs 0x1427f0` | `-` |
| `ldr` at 0x52948 | load | 4 | `abs 0x52a54` | `-` |
| `ldrb` at 0x5294c | load | 1 | `abs 0x14832d` | `-` |
| `ldr` at 0x52958 | load | 4 | `abs 0x52a58` | `-` |
| `ldr` at 0x5295c | load | 4 | `abs 0x148330` | `-` |
| `ldr` at 0x52968 | load | 4 | `abs 0x52a5c` | `-` |
| `ldr` at 0x5296c | load | 4 | `abs 0x52a60` | `-` |
| `ldr` at 0x52970 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52974 | load | 1 | `abs 0x14833d` | `-` |
| `ldrb` at 0x52978 | load | 1 | `[0x148334] +0x2` | `-` |
| `ldr` at 0x52984 | load | 4 | `abs 0x52a5c` | `-` |
| `ldr` at 0x5298c | load | 4 | `abs 0x52a60` | `-` |
| `ldr` at 0x52990 | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x52998 | load | 1 | `[0x148334] +0x6` | `-` |
| `strb` at 0x5299c | store | 1 | `abs 0x14833d` | `([[0x148334]+6]:1 & 0xff)` |
| `ldr` at 0x529a4 | load | 4 | `abs 0x52a5c` | `-` |
| `ldr` at 0x529a8 | load | 4 | `abs 0x52a60` | `-` |
| `ldr` at 0x529ac | load | 4 | `abs 0x148334` | `-` |
| `ldrb` at 0x529b0 | load | 1 | `[0x148334] +0x2` | `-` |
| `strb` at 0x529b4 | store | 1 | `abs 0x14833d` | `([[0x148334]+2]:1 & 0xff)` |
| `ldr` at 0x529b8 | load | 4 | `abs 0x52a58` | `-` |
| `ldr` at 0x529bc | load | 4 | `abs 0x52a64` | `-` |
| `ldrb` at 0x529c0 | load | 1 | `abs 0x14832c` | `-` |
| `str` at 0x529c4 | store | 4 | `abs 0x148330` | `[0x14832c]:1` |
| `ldr` at 0x529cc | load | 4 | `abs 0x52a58` | `-` |
| `ldr` at 0x529d0 | load | 4 | `abs 0x52a58` | `-` |
| `ldr` at 0x529d4 | load | 4 | `abs 0x148330` | `-` |
| `str` at 0x529dc | store | 4 | `abs 0x148330` | `[0x148330]-1` |
| `ldr` at 0x529e0 | load | 4 | `abs 0x52a68` | `-` |
| `ldr` at 0x529e4 | load | 4 | `abs 0x52a5c` | `-` |
| `ldrb` at 0x529e8 | load | 1 | `abs 0x14833e` | `-` |
| `ldrb` at 0x529ec | load | 1 | `abs 0x14833d` | `-` |
| `ldr` at 0x52a00 | load | 4 | `abs 0x52a5c` | `-` |
| `ldrb` at 0x52a04 | load | 1 | `abs 0x14833d` | `-` |
| `ldr` at 0x52a14 | load | 4 | `abs 0x52a5c` | `-` |
| `ldrb` at 0x52a18 | load | 1 | `abs 0x14833d` | `-` |
| `ldr` at 0x52a24 | load | 4 | `abs 0x52a50` | `-` |
| `str` at 0x52a2c | store | 4 | `abs 0x1427f0` | `0x3` |
| `ldr` at 0x52a34 | load | 4 | `abs 0x52a50` | `-` |
| `ldr` at 0x52a38 | load | 4 | `abs 0x52a50` | `-` |
| `ldr` at 0x52a3c | load | 4 | `abs 0x1427f0` | `-` |
| `str` at 0x52a44 | store | 4 | `abs 0x1427f0` | `[0x1427f0]-1` |
| `ldmdb` at 0x52a48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52a48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52a48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52a0c | `(unknown callee) 0x52b78` | `[0x14833d]:1` | `[0x14833e]:1` | `0x14833d` | `[0x14833d]:1` |
| `bl` at 0x52a20 | `(unknown callee) 0x52a6c` | `[0x14833d]:1` | `[0x14833e]:1` | `0x14833d` | `[0x14833d]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x52930 | `[0x148328] != 0x0` | `[0x148328] == 0x0` |
| `bge` at 0x52944 | `[0x1427f0] >= 0x0 (signed, 32-bit)` | `[0x1427f0] < 0x0 (signed, 32-bit)` |
| `beq` at 0x52954 | `[0x14832d]:1 == 0x0` | `[0x14832d]:1 != 0x0` |
| `bgt` at 0x52964 | `[0x148330] > 0x1 (signed, 32-bit)` | `[0x148330] <= 0x1 (signed, 32-bit)` |
| `bne` at 0x52980 | `[0x14833d]:1 != [[0x148334]+2]:1` | `[0x14833d]:1 == [[0x148334]+2]:1` |
| `beq` at 0x529fc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52a48 | `r0` | `?` |

## `vortix_set_link_enable`

- `vortix_set_link_enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x255d4 to 0x256c0.
  Established. The absolute-byte test takes the later selection path for `[0x147ea4]:1 != 0x0` and
  falls through on equality (`bne` at 0x25654). The two calls target `0x291fc` and `0x4da4c`,
  respectively, each with entry r0, entry r1, entry r2 and an unknown fourth candidate (`bl` at
  0x25684, `bl` at 0x256a0). Separate word stores save their results at `sp -0x20` (`str` at
  0x2568c, `str` at 0x256a8). Link-enable semantics and the joined r0 return remain unknown (`ldmdb`
  at 0x256b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x255e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x255e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x255ec | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x255f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2562c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x25640 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x25648 | load | 4 | `abs 0x256bc` | `-` |
| `ldrb` at 0x2564c | load | 1 | `abs 0x147ea4` | `-` |
| `str` at 0x2565c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x25664 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2567c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x25680 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2568c | store | 4 | `sp -0x20` | `ret(0x291fc)` |
| `ldr` at 0x25694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x25698 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2569c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x256a8 | store | 4 | `sp -0x20` | `ret(0x4da4c)` |
| `ldr` at 0x256ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x256b0 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x256b4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x256b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x256b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x256b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x25684 | `(unknown callee) 0x291fc` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x256a0 | `(unknown callee) 0x4da4c` | `r0` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x25600 | `? == 0x6` | `? != 0x6` |
| `bne` at 0x25614 | `? != 0x2` | `? == 0x2` |
| `bne` at 0x25628 | `? != 0x2` | `? == 0x2` |
| `bgt` at 0x25634 | `r1 > 0x1 (signed, 32-bit)` | `r1 <= 0x1 (signed, 32-bit)` |
| `bne` at 0x25654 | `[0x147ea4]:1 != 0x0` | `[0x147ea4]:1 == 0x0` |
| `bne` at 0x25674 | `? != 0x6` | `? == 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x256b8 | `r0` | `?` |

## `NL4R_set_link_enable`

- `NL4R_set_link_enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27f18 to 0x27f9c.
  Established. The absolute-byte guard takes the call arm for `[0x147eb9]:1 != 0x0` and falls
  through on equality to save word `0x0` at `sp -0x24` (`bne` at 0x27f60, `str` at 0x27f68). The
  call to `0x291fc` has entry r0, entry r1, entry r2, `[0x147eb9]:1` (`bl` at 0x27f7c); its result
  is saved as a word at `sp -0x20` (`str` at 0x27f84). Physical link selection and the merged r0
  value are unproved (`ldmdb` at 0x27f94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27f28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27f2c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x27f30 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x27f34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27f4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x27f54 | load | 4 | `abs 0x27f98` | `-` |
| `ldrb` at 0x27f58 | load | 1 | `abs 0x147eb9` | `-` |
| `str` at 0x27f68 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x27f70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27f74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27f78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27f84 | store | 4 | `sp -0x20` | `ret(0x291fc)` |
| `ldr` at 0x27f88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x27f8c | store | 4 | `sp -0x24` | `ret(0x291fc)` |
| `ldr` at 0x27f90 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x27f94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27f94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27f94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27f7c | `(unknown callee) 0x291fc` | `r0` | `r1` | `r2` | `[0x147eb9]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x27f44 | `? == 0x6` | `? != 0x6` |
| `bne` at 0x27f60 | `[0x147eb9]:1 != 0x0` | `[0x147eb9]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27f94 | `r0` | `?` |

## `nl4r_set_link_enable`

- `nl4r_set_link_enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x291fc to 0x29480.
  Established. The first call to `0xec7d4` carries `[sp-20]`, `0x3`, `[sp-24]+10`, `0x0` (`bl` at
  0x292ec); its signed-result branch takes the continuation for `[sp-36] >= 0x0` and falls through
  for less (`bge` at 0x29304). Two later calls to the same target each have `[sp-20]`, `0x3`,
  `[sp-24]+18`, `0x0` (`bl` at 0x293c0, `bl` at 0x2942c). Calls to `0xebed0` include `[sp-20]`,
  `[sp-24]+18`, `0x2`, `sp-32` (`bl` at 0x29378). The external transaction protocol and the joined
  r0 value remain unknown (`ldmdb` at 0x29460).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2920c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29210 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x29214 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x29218 | load | 4 | `abs 0x29464` | `-` |
| `ldr` at 0x2921c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x29228 | load | 4 | `abs 0x29468` | `-` |
| `ldr` at 0x2922c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29234 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2923c | load | 4 | `abs 0x2946c` | `-` |
| `ldr` at 0x29240 | load | 4 | `abs 0x14850c` | `-` |
| `ldr` at 0x2924c | load | 4 | `abs 0x29468` | `-` |
| `str` at 0x29250 | store | 4 | `sp -0x44` | `0x101680` |
| `ldr` at 0x29254 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29258 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x2925c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29260 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x29264 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29268 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x29270 | load | 4 | `abs 0x29470` | `-` |
| `ldr` at 0x29274 | load | 4 | `abs 0x29474` | `-` |
| `ldr` at 0x29280 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29284 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x29288 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x292a8 | load | 4 | `abs 0x29464` | `-` |
| `ldr` at 0x292ac | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x292b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x292c0 | load | 4 | `abs 0x29478` | `-` |
| `ldr` at 0x292c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x292d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x292dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x292f4 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x292fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x29308 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2930c | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x29314 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x29324 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x29330 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x29334 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29340 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29358 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2935c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2937c | load | 4 | `abs 0x29464` | `-` |
| `ldr` at 0x29380 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x2938c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29394 | load | 4 | `abs 0x2947c` | `-` |
| `ldr` at 0x2939c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x293a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x293b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x293c8 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x293d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x293dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x293e0 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x293ec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x293f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x293fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x29414 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2941c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x29434 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x2943c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x29448 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2944c | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x29458 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x2945c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x29460 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29460 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29460 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x29238 | `(unknown callee) 0x23f2c` | `0x101680` | `r0` | `r1` | `r2` |
| `bl` at 0x2927c | `(unknown callee) 0x21e28` | `0x6` | `0x1012e4` | `0x343` | `0x0` |
| `bl` at 0x292a4 | `(unknown callee) 0xebed0` | `?` | `?` | `0x1` | `sp-32` |
| `bl` at 0x292cc | `(unknown callee) 0x23f2c` | `0x1016b8` | `[sp-24]+65` | `[sp-28]` | `[sp-24]+65` |
| `bl` at 0x292d0 | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x292ec | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x3` | `[sp-24]+10` | `0x0` |
| `bl` at 0x292f8 | `(unknown callee) 0xe33c8` | `ret(0xec7d4)` | `?` | `?` | `ret(0xec7d4)` |
| `bl` at 0x29350 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+18` | `0x0` | `sp-32` |
| `bl` at 0x29378 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+18` | `0x2` | `sp-32` |
| `bl` at 0x293a0 | `(unknown callee) 0x23f2c` | `0x1016ec` | `[sp-24]+65` | `[sp-28]` | `[sp-24]+65` |
| `bl` at 0x293a4 | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x293c0 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x3` | `[sp-24]+18` | `0x0` |
| `bl` at 0x293cc | `(unknown callee) 0xe33c8` | `ret(0xec7d4)` | `?` | `?` | `ret(0xec7d4)` |
| `bl` at 0x2940c | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+18` | `0x0` | `sp-32` |
| `bl` at 0x29410 | `(unknown callee) 0xe335c` | `ret(0xebed0)` | `?` | `?` | `?` |
| `bl` at 0x2942c | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x3` | `[sp-24]+18` | `0x0` |
| `bl` at 0x29438 | `(unknown callee) 0xe33c8` | `ret(0xec7d4)` | `?` | `?` | `ret(0xec7d4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x29224 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x29248 | `[0x14850c] == 0x0` | `[0x14850c] != 0x0` |
| `bls` at 0x292b4 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x29304 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bne` at 0x2931c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bls` at 0x29388 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x293d8 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bge` at 0x29444 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29460 | `r0` | `?` |

## `SHUB2_SetLinkEnable`

- `SHUB2_SetLinkEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b0f4 to 0x3b1dc.
  Established. The absolute-word guard takes its alternate path for `[0x147fc0] == 0x0` and falls
  through otherwise (`beq` at 0x3b144). A subsequent word test takes the call path for
  `[[0x147fbc]+12] != 0x0` and falls through on equality (`bne` at 0x3b198). The call to `0x3bfa4`
  has entry r0, entry r1, entry r2, `[[0x147fbc]+12]` (`bl` at 0x3b1b4). Device and link meanings,
  and the r0 return, remain unknown (`ldmdb` at 0x3b1cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b104 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3b108 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3b10c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x3b110 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3b128 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x3b134 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3b138 | load | 4 | `abs 0x3b1d0` | `-` |
| `ldr` at 0x3b13c | load | 4 | `abs 0x147fc0` | `-` |
| `ldr` at 0x3b148 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b150 | load | 4 | `abs 0x3b1d4` | `-` |
| `ldr` at 0x3b154 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3b15c | load | 4 | `[0x147fbc] +0x8` | `-` |
| `ldr` at 0x3b168 | load | 4 | `abs 0x3b1d8` | `-` |
| `str` at 0x3b16c | store | 4 | `sp -0x2c` | `0xffffeffc` |
| `ldr` at 0x3b174 | load | 4 | `abs 0x3b1d4` | `-` |
| `ldr` at 0x3b178 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b180 | load | 4 | `abs 0x147fbc` | `-` |
| `str` at 0x3b188 | store | 4 | `sp -0x28` | `[0x147fbc]` |
| `ldr` at 0x3b18c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b190 | load | 4 | `[0x147fbc] +0xc` | `-` |
| `str` at 0x3b1a0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3b1a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b1ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b1b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b1bc | store | 4 | `sp -0x20` | `ret(0x3bfa4)` |
| `ldr` at 0x3b1c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3b1c4 | store | 4 | `sp -0x2c` | `ret(0x3bfa4)` |
| `ldr` at 0x3b1c8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3b1cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b1cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b1cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b1b4 | `(unknown callee) 0x3bfa4` | `r0` | `r1` | `r2` | `[[0x147fbc]+12]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3b120 | `? == 0x7` | `? != 0x7` |
| `beq` at 0x3b144 | `[0x147fc0] == 0x0` | `[0x147fc0] != 0x0` |
| `bne` at 0x3b164 | `[[0x147fbc]+8] != 0x0` | `[[0x147fbc]+8] == 0x0` |
| `bne` at 0x3b198 | `[[0x147fbc]+12] != 0x0` | `[[0x147fbc]+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b1cc | `r0` | `?` |

## `shub2_set_link_enable`

- `shub2_set_link_enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3bfa4 to 0x3c1c8.
  Established. The first call to `0xec7d4` uses `[sp-20]`, `0x3`, `[sp-24]+9`, `0x0` (`bl` at
  0x3c048). A later pair uses `[sp-20]`, `0x3`, `[sp-24]+11`, `0x0` (`bl` at 0x3c114, `bl` at
  0x3c180). The call to `0xebed0` between them carries `[sp-20]`, `[sp-24]+11`, `0x0`, `sp-32` (`bl`
  at 0x3c160). The final signed test takes the continuation for `[sp-36] >= 0x0` and falls through
  for less (`bge` at 0x3c198). Physical link control and the merged r0 result remain unproven
  (`ldmdb` at 0x3c1b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3bfb4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3bfb8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3bfbc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x3bfc0 | load | 4 | `abs 0x3c1b8` | `-` |
| `ldr` at 0x3bfc4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x3bfd0 | load | 4 | `abs 0x3c1bc` | `-` |
| `ldr` at 0x3bfd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3bfd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bfdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3bfe4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3bfe8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3bfec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3bff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c00c | load | 4 | `abs 0x3c1b8` | `-` |
| `ldr` at 0x3c010 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x3c01c | load | 4 | `abs 0x3c1c0` | `-` |
| `ldr` at 0x3c020 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c038 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c050 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x3c058 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c064 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c068 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x3c070 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c080 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x3c08c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x3c090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c09c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c0b4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3c0b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c0c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c0d8 | load | 4 | `abs 0x3c1b8` | `-` |
| `ldr` at 0x3c0dc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x3c0e8 | load | 4 | `abs 0x3c1c4` | `-` |
| `ldr` at 0x3c0ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c0f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c0fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c104 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c11c | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x3c124 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c130 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c134 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x3c140 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3c144 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c150 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c168 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c170 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c188 | store | 4 | `sp -0x24` | `ret(0xec7d4)` |
| `ldr` at 0x3c190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c19c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c1a0 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x3c1ac | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3c1b0 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3c1b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c1b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c1b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3bfe0 | `(unknown callee) 0x23f2c` | `0x107fb4` | `r0` | `r1` | `r2` |
| `bl` at 0x3c008 | `(unknown callee) 0xebed0` | `?` | `?` | `0x19` | `sp-32` |
| `bl` at 0x3c028 | `(unknown callee) 0x23f2c` | `0x107fec` | `[sp-24]` | `[sp-28]` | `[0x2ad25c]` |
| `bl` at 0x3c02c | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3c048 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x3` | `[sp-24]+9` | `0x0` |
| `bl` at 0x3c054 | `(unknown callee) 0xe33c8` | `ret(0xec7d4)` | `?` | `?` | `ret(0xec7d4)` |
| `bl` at 0x3c0ac | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+11` | `0x0` | `sp-32` |
| `bl` at 0x3c0d4 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+11` | `0x2` | `sp-32` |
| `bl` at 0x3c0f4 | `(unknown callee) 0x23f2c` | `0x108024` | `[sp-24]` | `[sp-28]` | `[0x2ad25c]` |
| `bl` at 0x3c0f8 | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3c114 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x3c120 | `(unknown callee) 0xe33c8` | `ret(0xec7d4)` | `?` | `?` | `ret(0xec7d4)` |
| `bl` at 0x3c160 | `(unknown callee) 0xebed0` | `[sp-20]` | `[sp-24]+11` | `0x0` | `sp-32` |
| `bl` at 0x3c164 | `(unknown callee) 0xe335c` | `ret(0xebed0)` | `?` | `?` | `?` |
| `bl` at 0x3c180 | `(unknown callee) 0xec7d4` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x3c18c | `(unknown callee) 0xe33c8` | `ret(0xec7d4)` | `?` | `?` | `ret(0xec7d4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3bfcc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bls` at 0x3c018 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x3c060 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bne` at 0x3c078 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bls` at 0x3c0e4 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x3c12c | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bge` at 0x3c198 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c1b4 | `r0` | `?` |

## `shub_set_link_enable`

- `shub_set_link_enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4da4c to 0x4db2c.
  Established. The call to `0xea56c` has `[sp-20]`, `0x10060300`, `0x0`, `0x1` (`bl` at 0x4dafc).
  The following call to `0xe33c8` carries `ret(0xea56c)` and three unknown candidates (`bl` at
  0x4db00). Both result arms store word `0x0` at `sp -0x20`, and the return has r0 `0x0` (`str` at
  0x4dae0, `str` at 0x4db08, `ldmdb` at 0x4db10). Hardware addressing and link-enable effects are
  not established by these numeric callees.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4da5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4da60 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4da64 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x4da68 | load | 4 | `abs 0x4db14` | `-` |
| `ldr` at 0x4da6c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4da78 | load | 4 | `abs 0x4db18` | `-` |
| `ldr` at 0x4da7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4da80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4da84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4da8c | load | 4 | `abs 0x4db1c` | `-` |
| `ldr` at 0x4da90 | load | 4 | `abs 0x14850c` | `-` |
| `ldr` at 0x4da9c | load | 4 | `abs 0x4db18` | `-` |
| `str` at 0x4daa0 | store | 4 | `sp -0x30` | `0x10a404` |
| `ldr` at 0x4daa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4daa8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x4daac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4dab0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4dab4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dab8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4dac0 | load | 4 | `abs 0x4db20` | `-` |
| `ldr` at 0x4dac4 | load | 4 | `abs 0x4db24` | `-` |
| `ldr` at 0x4dad0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4dae0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4daec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4daf0 | load | 4 | `abs 0x4db28` | `-` |
| `str` at 0x4db08 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4db0c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x4db10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4db10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4db10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4da88 | `(unknown callee) 0x23f2c` | `0x10a404` | `r0` | `r1` | `r2` |
| `bl` at 0x4dacc | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x7eb` | `0x0` |
| `bl` at 0x4dae8 | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4dafc | `(unknown callee) 0xea56c` | `[sp-20]` | `0x10060300` | `0x0` | `0x1` |
| `bl` at 0x4db00 | `(unknown callee) 0xe33c8` | `ret(0xea56c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4da74 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4da98 | `[0x14850c] == 0x0` | `[0x14850c] != 0x0` |
| `bne` at 0x4dad8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4db10 | `r0` | `0x0` |

## `TIOSetLinkEnable`

- `TIOSetLinkEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53c18 to 0x53c9c.
  Established. The absolute-byte branch takes the call path for `[0x148354]:1 != 0x0` and falls
  through on equality (`bne` at 0x53c60). The call reaches `0x53c9c` with entry r0, entry r1, entry
  r2, `[0x148354]:1` (`bl` at 0x53c7c); its result is saved as a word at `sp -0x20` and then at `sp
  -0x24` (`str` at 0x53c84, `str` at 0x53c8c). Physical port identity and the joined r0 value remain
  unknown (`ldmdb` at 0x53c94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53c28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x53c2c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x53c30 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x53c34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53c4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x53c54 | load | 4 | `abs 0x53c98` | `-` |
| `ldrb` at 0x53c58 | load | 1 | `abs 0x148354` | `-` |
| `str` at 0x53c68 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x53c70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53c74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53c78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53c84 | store | 4 | `sp -0x20` | `ret(0x53c9c)` |
| `ldr` at 0x53c88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x53c8c | store | 4 | `sp -0x24` | `ret(0x53c9c)` |
| `ldr` at 0x53c90 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x53c94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53c94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53c94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53c7c | `(unknown callee) 0x53c9c` | `r0` | `r1` | `r2` | `[0x148354]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53c44 | `? == 0x4` | `? != 0x4` |
| `bne` at 0x53c60 | `[0x148354]:1 != 0x0` | `[0x148354]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53c94 | `r0` | `?` |

## `tio_set_link_enable`

- `tio_set_link_enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53c9c to 0x53d84.
  Established. One word store places `0x90060780` at `sp -0x20` before the later calls (`str` at
  0x53d3c). The call to `0xea56c` has `[sp-20]`, `[sp-32]`, `0x0`, `0x1` (`bl` at 0x53d54); its
  following call to `0xe33c8` uses `ret(0xea56c)` and three unknown candidates (`bl` at 0x53d58).
  The final r0 value is `0x0` (`ldmdb` at 0x53d68). Register meanings and physical link changes
  remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53cac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x53cb0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x53cb4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x53cb8 | load | 4 | `abs 0x53d6c` | `-` |
| `ldr` at 0x53cbc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x53cc8 | load | 4 | `abs 0x53d70` | `-` |
| `ldr` at 0x53ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53cd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53cd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53cdc | load | 4 | `abs 0x53d74` | `-` |
| `ldr` at 0x53ce0 | load | 4 | `abs 0x14850c` | `-` |
| `ldr` at 0x53cec | load | 4 | `abs 0x53d70` | `-` |
| `str` at 0x53cf0 | store | 4 | `sp -0x34` | `0x10b358` |
| `ldr` at 0x53cf4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53cf8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x53cfc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53d00 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x53d04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53d08 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x53d10 | load | 4 | `abs 0x53d78` | `-` |
| `ldr` at 0x53d14 | load | 4 | `abs 0x53d7c` | `-` |
| `ldr` at 0x53d20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53d30 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x53d38 | load | 4 | `abs 0x53d80` | `-` |
| `str` at 0x53d3c | store | 4 | `sp -0x20` | `0x90060780` |
| `ldr` at 0x53d44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53d48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x53d60 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x53d64 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x53d68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53d68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53d68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53cd8 | `(unknown callee) 0x23f2c` | `0x10b358` | `r0` | `r1` | `r2` |
| `bl` at 0x53d1c | `(unknown callee) 0x21e28` | `0x7` | `0x10b2c8` | `0x216` | `0x0` |
| `bl` at 0x53d40 | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `0x90060780` |
| `bl` at 0x53d54 | `(unknown callee) 0xea56c` | `[sp-20]` | `[sp-32]` | `0x0` | `0x1` |
| `bl` at 0x53d58 | `(unknown callee) 0xe33c8` | `ret(0xea56c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53cc4 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x53ce8 | `[0x14850c] == 0x0` | `[0x14850c] != 0x0` |
| `bne` at 0x53d28 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53d68 | `r0` | `0x0` |

## `LinkErrTaskInit`

- `LinkErrTaskInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37344 to 0x373bc.
  Established. Five word stores place `0x0`, `0x800`, `0xa`, `0xa`, `0x9` at `sp -0x24`, `sp -0x20`,
  `sp -0x1c`, `sp -0x18`, `sp -0x14`, respectively (`str` at 0x3736c, `str` at 0x37374, `str` at
  0x3737c, `str` at 0x37384, `str` at 0x3738c). The later call to `0x223f0` has `0x147fb4`,
  `0x106dc8`, `0x3690c`, `0x1` (`bl` at 0x373a0); its result supplies r0 at the return (`ldmdb` at
  0x373a4). Task creation semantics, stack argument count and monitoring intervals remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x37354 | load | 4 | `abs 0x373a8` | `-` |
| `ldr` at 0x37358 | load | 4 | `abs 0x373ac` | `-` |
| `str` at 0x3736c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x37374 | store | 4 | `sp -0x20` | `0x800` |
| `str` at 0x3737c | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x37384 | store | 4 | `sp -0x18` | `0xa` |
| `str` at 0x3738c | store | 4 | `sp -0x14` | `0x9` |
| `ldr` at 0x37390 | load | 4 | `abs 0x373b0` | `-` |
| `ldr` at 0x37394 | load | 4 | `abs 0x373b4` | `-` |
| `ldr` at 0x37398 | load | 4 | `abs 0x373b8` | `-` |
| `ldmdb` at 0x373a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x373a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x373a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37364 | `(unknown callee) 0x22920` | `0x147f60` | `0x106dbc` | `0x0` | `0x9` |
| `bl` at 0x373a0 | `(unknown callee) 0x223f0` | `0x147fb4` | `0x106dc8` | `0x3690c` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x373a4 | `r0` | `ret(0x223f0)` |

## `LinkErrChangeFreq`

- `LinkErrChangeFreq`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x373bc to 0x37430.
  Established. The opening word store writes entry r0 at `0x14275c` (`str` at 0x373d8). The signed
  bound takes the call arm for `[0x14275c] <= 0x176f` and falls through for greater (`ble` at
  0x373ec). The call to `0xfa70c` has `0x1770`, `[0x14275c]`, `[0x14275c]`, `0x14275c`, with its
  result stored as a word at `0x142760` (`bl` at 0x37410, `str` at 0x37418). The other arm stores
  word `0x0` there (`str` at 0x373f8). Frequency units and the joined r0 return remain unknown
  (`ldmdb` at 0x3741c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x373cc | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x373d0 | load | 4 | `abs 0x37420` | `-` |
| `ldr` at 0x373d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x373d8 | store | 4 | `abs 0x14275c` | `r0` |
| `ldr` at 0x373dc | load | 4 | `abs 0x37420` | `-` |
| `ldr` at 0x373e0 | load | 4 | `abs 0x14275c` | `-` |
| `ldr` at 0x373e4 | load | 4 | `abs 0x37424` | `-` |
| `ldr` at 0x373f0 | load | 4 | `abs 0x37428` | `-` |
| `str` at 0x373f8 | store | 4 | `abs 0x142760` | `0x0` |
| `ldr` at 0x37400 | load | 4 | `abs 0x37428` | `-` |
| `ldr` at 0x37404 | load | 4 | `abs 0x37420` | `-` |
| `ldr` at 0x37408 | load | 4 | `abs 0x3742c` | `-` |
| `ldr` at 0x3740c | load | 4 | `abs 0x14275c` | `-` |
| `str` at 0x37418 | store | 4 | `abs 0x142760` | `ret(0xfa70c)` |
| `ldmdb` at 0x3741c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3741c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3741c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3741c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37410 | `(unknown callee) 0xfa70c` | `0x1770` | `[0x14275c]` | `[0x14275c]` | `0x14275c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x373ec | `[0x14275c] <= 0x176f (signed, 32-bit)` | `[0x14275c] > 0x176f (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3741c | `r0` | `?` |

## `LinkErrActive`

- `LinkErrActive`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37430 to 0x37494.
  Established. The only call reaches `0x1f1dc` with `0x9b`, entry r1, entry r2, entry r3 (`bl` at
  0x37444). The first byte test takes the zero-result arm for `(ret(0x1f1dc) & 0xff) == 0x0` and
  falls through otherwise (`beq` at 0x37464). The next test takes that arm for `([0x142758]:1 &
  0xff) == 0x0`, also falling through otherwise (`beq` at 0x37470). Separate result stores write
  word `0x1` and word `0x0` at `sp -0x18` (`str` at 0x37478, `str` at 0x37484). The joined return
  and physical activity interpretation remain unknown (`ldmdb` at 0x3748c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3744c | store | 1 | `sp -0x11` | `(ret(0x1f1dc) & 0xff)` |
| `ldr` at 0x37450 | load | 4 | `abs 0x37490` | `-` |
| `ldrb` at 0x37454 | load | 1 | `abs 0x142758` | `-` |
| `strb` at 0x37458 | store | 1 | `sp -0x12` | `([0x142758]:1 & 0xff)` |
| `ldrb` at 0x3745c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x37468 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x37478 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x37484 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x37488 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3748c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3748c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3748c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37444 | `(unknown callee) 0x1f1dc` | `0x9b` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x37464 | `(ret(0x1f1dc) & 0xff) == 0x0` | `(ret(0x1f1dc) & 0xff) != 0x0` |
| `beq` at 0x37470 | `([0x142758]:1 & 0xff) == 0x0` | `([0x142758]:1 & 0xff) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3748c | `r0` | `?` |
