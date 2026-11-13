# ARM fan and power state paths in 1.38.4

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/fan-power-state-facts.txt) retain all tool sections.

## `fanCtrlInit`

- `fanCtrlInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x454a0 to 0x455c8.
  Established. On the zero arm of the absolute-word test, a word store writes `0x1` at `0x148048`
  (`bne` at 0x45518, `str` at 0x45524). A call to `0x1b3c4` carries `[sp-44]`, `0x1`, `0x148048`,
  unknown, and its result is stored as a word at `0x148044` (`bl` at 0x45544, `str` at 0x4554c). The
  final two explicit stores put an unknown word at `[sp-36] +0x0` and byte `([sp-32]:1 & 0xff)` at
  `[sp-36] +0x4` (`str` at 0x455a0, `strb` at 0x455ac). Fan-controller initialization and the final
  r0 value are not established (`ldmdb` at 0x455b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x454b0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x454b4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x454b8 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x454bc | load | 4 | `abs 0x455b4` | `-` |
| `ldr` at 0x454c0 | load | 4 | `abs 0x148044` | `-` |
| `ldr` at 0x454cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x454d8 | load | 4 | `abs 0x455b8` | `-` |
| `str` at 0x454dc | store | 4 | `sp -0x38` | `0x109254` |
| `ldr` at 0x454e0 | load | 4 | `abs 0x455b4` | `-` |
| `ldr` at 0x454e4 | load | 4 | `abs 0x148044` | `-` |
| `str` at 0x454e8 | store | 4 | `sp -0x34` | `[0x148044]` |
| `ldr` at 0x454ec | load | 4 | `abs 0x455bc` | `-` |
| `ldr` at 0x454f0 | load | 4 | `abs 0x148048` | `-` |
| `str` at 0x454f4 | store | 4 | `sp -0x30` | `[0x148048]` |
| `ldr` at 0x454fc | load | 4 | `abs 0x455c0` | `-` |
| `ldr` at 0x4550c | load | 4 | `abs 0x455bc` | `-` |
| `ldr` at 0x45510 | load | 4 | `abs 0x148048` | `-` |
| `ldr` at 0x4551c | load | 4 | `abs 0x455bc` | `-` |
| `str` at 0x45524 | store | 4 | `abs 0x148048` | `0x1` |
| `ldr` at 0x45528 | load | 4 | `abs 0x455bc` | `-` |
| `ldr` at 0x4552c | load | 4 | `abs 0x148048` | `-` |
| `str` at 0x45534 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x45538 | load | 4 | `abs 0x455b4` | `-` |
| `ldr` at 0x4553c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4554c | store | 4 | `abs 0x148044` | `ret(0x1b3c4)` |
| `ldr` at 0x45550 | load | 4 | `abs 0x148044` | `-` |
| `ldr` at 0x4555c | load | 4 | `abs 0x455c4` | `-` |
| `str` at 0x45560 | store | 4 | `sp -0x38` | `0x109288` |
| `ldr` at 0x45564 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x45568 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0x45570 | load | 4 | `abs 0x455c0` | `-` |
| `ldr` at 0x45580 | load | 4 | `abs 0x455b4` | `-` |
| `ldr` at 0x45584 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4558c | load | 4 | `abs 0x148044` | `-` |
| `str` at 0x45594 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x45598 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4559c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x455a0 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x455a4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x455a8 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x455ac | store | 1 | `[sp-36] +0x4` | `([sp-32]:1 & 0xff)` |
| `ldmdb` at 0x455b0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x455b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x455b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x455b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45508 | `(unknown callee) 0x21e28` | `0x9` | `0x1091b8` | `0x81` | `0x0` |
| `bl` at 0x45544 | `(unknown callee) 0x1b3c4` | `[sp-44]` | `0x1` | `0x148048` | `?` |
| `bl` at 0x4557c | `(unknown callee) 0x21e28` | `0x9` | `0x1091b8` | `0x8d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x454c8 | `[0x148044] == 0x0` | `[0x148044] != 0x0` |
| `bne` at 0x454d4 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x45518 | `[0x148048] != 0x0` | `[0x148048] == 0x0` |
| `bne` at 0x45558 | `[0x148044] != 0x0` | `[0x148044] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x455b0 | `r0` | `?` |

## `fanTachNotPresent`

- `fanTachNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x455c8 to 0x45628.
  Established. The call to `0x4724c` has entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl` at
  0x455f0); its nonzero result takes the continuation and zero falls through to the exit (`bne` at
  0x45604). The continuation clears a byte at `([sp-21]:1 + [ret(0x4724c)+8]) +0x0` (`strb` at
  0x45620). The returned r0 value is `ret(0x4724c)`, while the physical presence interpretation is
  unknown (`ldmdb` at 0x45624).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x455d8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x455e0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x455e4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x455e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x455f8 | store | 4 | `sp -0x1c` | `ret(0x4724c)` |
| `ldr` at 0x455fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4560c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45610 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45614 | load | 4 | `ret(0x4724c) +0x8` | `-` |
| `strb` at 0x45620 | store | 1 | `([sp-21]:1 + [ret(0x4724c)+8]) +0x0` | `0x0` |
| `ldmdb` at 0x45624 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45624 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45624 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x455f0 | `(unknown callee) 0x4724c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x45604 | `ret(0x4724c) != 0x0` | `ret(0x4724c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45624 | `r0` | `ret(0x4724c)` |

## `fanTachPresent`

- `fanTachPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45628 to 0x45688.
  Established. A call to `0x4724c` carries entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl` at
  0x45650). Nonzero takes the continuing edge and zero falls through to the return (`bne` at
  0x45664). The reached byte store writes `0x0` at `([sp-21]:1 + [ret(0x4724c)+8]) +0x0` (`strb` at
  0x45680). The routine name does not establish a physical presence state, and the r0 return retains
  `ret(0x4724c)` (`ldmdb` at 0x45684).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45638 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x45640 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x45644 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45648 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x45658 | store | 4 | `sp -0x1c` | `ret(0x4724c)` |
| `ldr` at 0x4565c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4566c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45670 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x45674 | load | 4 | `ret(0x4724c) +0x8` | `-` |
| `strb` at 0x45680 | store | 1 | `([sp-21]:1 + [ret(0x4724c)+8]) +0x0` | `0x0` |
| `ldmdb` at 0x45684 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45684 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45684 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45650 | `(unknown callee) 0x4724c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x45664 | `ret(0x4724c) != 0x0` | `ret(0x4724c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45684 | `r0` | `ret(0x4724c)` |

## `fanTachIsPresent`

- `fanTachIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45688 to 0x456f4.
  Established. The first call targets `0x4724c` with entry r0, `(r1 & 0xff)`, entry r2, `(r1 &
  0xff)` (`bl` at 0x456b0); its nonzero result takes the read arm, while zero falls through to store
  word `0x0` at `sp -0x20` (`bne` at 0x456c4, `str` at 0x456cc). The read arm loads a byte at
  `([sp-21]:1 + [ret(0x4724c)+8]) +0x0`, saving `[([sp-21]:1 + [ret(0x4724c)+8])]:1` as a word at
  that stack target (`ldrb` at 0x456e4, `str` at 0x456e8). The merged r0 return and physical
  tachometer-presence criterion remain unknown (`ldmdb` at 0x456f0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45698 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x456a0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x456a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x456a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x456b8 | store | 4 | `sp -0x1c` | `ret(0x4724c)` |
| `ldr` at 0x456bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x456cc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x456d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x456d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x456dc | load | 4 | `ret(0x4724c) +0x8` | `-` |
| `ldrb` at 0x456e4 | load | 1 | `([sp-21]:1 + [ret(0x4724c)+8]) +0x0` | `-` |
| `str` at 0x456e8 | store | 4 | `sp -0x20` | `[([sp-21]:1 + [ret(0x4724c)+8])]:1` |
| `ldr` at 0x456ec | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x456f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x456f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x456f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x456b0 | `(unknown callee) 0x4724c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x456c4 | `ret(0x4724c) != 0x0` | `ret(0x4724c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x456f0 | `r0` | `?` |

## `fanGetTachCfg`

- `fanGetTachCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45d70 to 0x45ddc.
  Established. The branch takes its bypass for entry `r1 == 0x0` and falls through on nonzero (`beq`
  at 0x45db4). The nonzero arm writes word `[[sp-28]]` at entry `r1 +0x0` (`str` at 0x45dc4). A
  later byte read addresses `[sp-28] +0x4` and supplies the r0 return `[[sp-28]+4]:1` (`ldrb` at
  0x45dcc, `ldmdb` at 0x45dd4). No calls are reported, and the configuration-field meanings are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45d80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x45d84 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x45d88 | load | 4 | `abs 0x45dd8` | `-` |
| `ldr` at 0x45d8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x45da0 | load | 4 | `abs 0x14803c` | `-` |
| `str` at 0x45da8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x45dac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45db8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45dbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45dc0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x45dc4 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x45dc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x45dcc | load | 1 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0x45dd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45dd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45dd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x45db4 | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45dd4 | `r0` | `[[sp-28]+4]:1` |

## `fanLocation`

- `fanLocation`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46f9c to 0x47004.
  Established. The sole call reaches `0x47004` with entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)`
  (`bl` at 0x46fc4). A subsequent word read addresses `ret(0x47004) +0x4` (`ldr` at 0x46fe0). The
  nonzero result takes the bypass and zero falls through to store word `0x1093e0` at `sp -0x20`
  (`bne` at 0x46fe8, `str` at 0x46ff0). The location representation and joined r0 return are
  unproven (`ldmdb` at 0x46ffc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46fac | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x46fb4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x46fb8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46fbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46fcc | store | 4 | `sp -0x1c` | `ret(0x47004)` |
| `ldr` at 0x46fd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46fd4 | load | 4 | `ret(0x47004) +0x4` | `-` |
| `str` at 0x46fd8 | store | 4 | `sp -0x20` | `[ret(0x47004)+4]` |
| `ldr` at 0x46fdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x46fe0 | load | 4 | `ret(0x47004) +0x4` | `-` |
| `ldr` at 0x46fec | load | 4 | `abs 0x47000` | `-` |
| `str` at 0x46ff0 | store | 4 | `sp -0x20` | `0x1093e0` |
| `ldr` at 0x46ff4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x46ffc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46ffc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46ffc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46fc4 | `(unknown callee) 0x47004` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x46fe8 | `[ret(0x47004)+4] != 0x0` | `[ret(0x47004)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46ffc | `r0` | `?` |

## `fanGetSpeed`

- `fanGetSpeed`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x46c64 to 0x46df0.
  Established. The call to `0x45688` has `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1` (`bl` at
  0x46ca8); its nonzero result takes the continuation and zero falls through to store word
  `0xfffffa85` at `sp -0x2c` (`bne` at 0x46cb4, `str` at 0x46cbc). The selector takes separate call
  arms for `[ret(0x47130)]:1 == 0x1` and `== 0x2`, falling through on inequality (`beq` at 0x46d0c,
  `beq` at 0x46cf4). Those arms include `0xd6c2c` with `[sp-20]`, `[ret(0x47130)+3]:1`, unknown,
  `[ret(0x47130)+3]:1` and `0xcd71c` with `[sp-20]`, `[ret(0x47130)+3]:1`, `[ret(0x47130)+4]:1`,
  `[sp-28]`, respectively (`bl` at 0x46d50, `bl` at 0x46da8). Physical speed units and the merged
  return remain unknown (`ldmdb` at 0x46de0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x46c74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x46c7c | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x46c80 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x46c84 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46c88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46c98 | store | 4 | `sp -0x20` | `ret(0x47004)` |
| `ldrb` at 0x46c9c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x46ca0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46cb8 | load | 4 | `abs 0x46de4` | `-` |
| `str` at 0x46cbc | store | 4 | `sp -0x2c` | `0xfffffa85` |
| `ldr` at 0x46cc4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x46cc8 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x46ccc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x46cdc | store | 4 | `sp -0x24` | `ret(0x47130)` |
| `ldr` at 0x46ce0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x46ce4 | load | 1 | `ret(0x47130) +0x0` | `-` |
| `str` at 0x46ce8 | store | 4 | `sp -0x30` | `[ret(0x47130)]:1` |
| `ldr` at 0x46cec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46cf8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46d04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46d14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x46d24 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x46d28 | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x46d34 | load | 4 | `abs 0x46de8` | `-` |
| `str` at 0x46d38 | store | 4 | `sp -0x2c` | `0xfffffa87` |
| `ldr` at 0x46d40 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x46d44 | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x46d48 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x46d58 | store | 1 | `sp -0x25` | `(ret(0xd6c2c) & 0xff)` |
| `ldr` at 0x46d5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x46d60 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x46d68 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x46d6c | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x46d74 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x46d78 | load | 1 | `[sp-36] +0x4` | `-` |
| `strb` at 0x46d80 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x46d88 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x46d8c | load | 1 | `ret(0x47130) +0x3` | `-` |
| `ldr` at 0x46d90 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x46d94 | load | 1 | `ret(0x47130) +0x4` | `-` |
| `ldr` at 0x46d98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x46da4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x46db0 | store | 4 | `sp -0x2c` | `ret(0xcd71c)` |
| `ldr` at 0x46db8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x46dc0 | store | 1 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x46dc8 | load | 4 | `abs 0x46dec` | `-` |
| `str` at 0x46dcc | store | 4 | `sp -0x2c` | `0xfffffa86` |
| `str` at 0x46dd8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x46ddc | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x46de0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x46de0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x46de0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x46c90 | `(unknown callee) 0x47004` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x46ca8 | `(unknown callee) 0x45688` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x46cd4 | `(unknown callee) 0x47130` | `[sp-20]` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0x46d50 | `(unknown callee) 0xd6c2c` | `[sp-20]` | `[ret(0x47130)+3]:1` | `?` | `[ret(0x47130)+3]:1` |
| `bl` at 0x46da8 | `(unknown callee) 0xcd71c` | `[sp-20]` | `[ret(0x47130)+3]:1` | `[ret(0x47130)+4]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x46cb4 | `ret(0x45688) != 0x0` | `ret(0x45688) == 0x0` |
| `beq` at 0x46cf4 | `[ret(0x47130)]:1 == 0x2` | `[ret(0x47130)]:1 != 0x2` |
| `bgt` at 0x46d00 | `[ret(0x47130)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x47130)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x46d0c | `[ret(0x47130)]:1 == 0x1` | `[ret(0x47130)]:1 != 0x1` |
| `beq` at 0x46d1c | `[ret(0x47130)]:1 == 0x63` | `[ret(0x47130)]:1 != 0x63` |
| `bne` at 0x46d30 | `[ret(0x47130)+3]:1 != 0xff` | `[ret(0x47130)+3]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x46de0 | `r0` | `?` |

## `vsc055_FanPWMInit`

- `vsc055_FanPWMInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9c84 to 0xd9d3c.
  Established. A byte store initializes `sp -0x29` to `0x0`, while a later byte store at the same
  target has an unknown value (`strb` at 0xd9cb0, `strb` at 0xd9cdc). The call to `0xd687c` carries
  entry r0, `sp-32`, entry r2, `sp-32` (`bl` at 0xd9cc0). The final call reaches `0xc7f7c` with
  unknown, `[sp-22]:1`, unknown, `[sp-41]:1` and supplies the r0 return (`bl` at 0xd9d24, `ldmdb` at
  0xd9d38). No conditional branches are reported, and the PWM configuration and physical output
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9c94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9c98 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xd9ca0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd9ca8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd9cb0 | store | 1 | `sp -0x29` | `0x0` |
| `ldr` at 0xd9cb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd9cc8 | store | 4 | `sp -0x24` | `ret(0xd687c)` |
| `ldr` at 0xd9ccc | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xd9cd4 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xd9cdc | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xd9ce0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9ce4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd9cf8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd9d04 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xd9d10 | load | 1 | `sp -0x29` | `-` |
| `str` at 0xd9d2c | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xd9d30 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd9d38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9d38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9d38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9cc0 | `(unknown callee) 0xd687c` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0xd9cec | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-21]:1` | `[sp-41]:1` | `[sp-21]:1` |
| `bl` at 0xd9d24 | `(unknown callee) 0xc7f7c` | `?` | `[sp-22]:1` | `?` | `[sp-41]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9d38 | `r0` | `ret(0xc7f7c)` |

## `vsc055_FanMonitorInit`

- `vsc055_FanMonitorInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9d3c to 0xd9e4c.
  Established. Two calls to `0xc7f7c` use unknown, `[sp-22]:1`, `[sp-41]:1`, `0xff` and later
  unknown, `[sp-22]:1`, `[sp-41]:1`, `[sp-42]:1`, respectively (`bl` at 0xd9dc4, `bl` at 0xd9e34).
  Between them, byte stores at `sp -0x2a` and `sp -0x29` retain unknown values (`strb` at 0xd9ddc,
  `strb` at 0xd9dec, `strb` at 0xd9dfc). The second call result is returned in r0 (`ldmdb` at
  0xd9e48). Monitoring thresholds, timing and hardware effects are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9d4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9d50 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xd9d58 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd9d60 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd9d68 | store | 1 | `sp -0x2a` | `0x0` |
| `ldr` at 0xd9d70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd9d80 | store | 4 | `sp -0x24` | `ret(0xd687c)` |
| `ldr` at 0xd9d84 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xd9d90 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xd9d94 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9d98 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd9dac | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd9db0 | load | 1 | `sp -0x29` | `-` |
| `str` at 0xd9dcc | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldrb` at 0xd9dd0 | load | 1 | `sp -0x2a` | `-` |
| `strb` at 0xd9ddc | store | 1 | `sp -0x2a` | `?` |
| `ldrb` at 0xd9de0 | load | 1 | `sp -0x2a` | `-` |
| `ldrb` at 0xd9de4 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xd9dec | store | 1 | `sp -0x2a` | `?` |
| `ldr` at 0xd9df0 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xd9dfc | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xd9e00 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9e04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd9e18 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd9e1c | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xd9e20 | load | 1 | `sp -0x2a` | `-` |
| `str` at 0xd9e3c | store | 4 | `sp -0x28` | `ret(0xc7f7c)` |
| `ldr` at 0xd9e40 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd9e48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9e48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9e48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9d78 | `(unknown callee) 0xd687c` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0xd9da0 | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd9dc4 | `(unknown callee) 0xc7f7c` | `?` | `[sp-22]:1` | `[sp-41]:1` | `0xff` |
| `bl` at 0xd9e0c | `(unknown callee) 0xc7784` | `[sp-20]` | `[sp-21]:1` | `[sp-42]:1` | `[sp-21]:1` |
| `bl` at 0xd9e34 | `(unknown callee) 0xc7f7c` | `?` | `[sp-22]:1` | `[sp-41]:1` | `[sp-42]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9e48 | `r0` | `ret(0xc7f7c)` |

## `powerGetCfg`

- `powerGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c80c to 0x3c86c.
  Established. A word store writes `[[sp-28]]` at entry `r1 +0x0` (`str` at 0x3c854). The following
  word read addresses `[sp-28] +0x8`, and the r0 return is `[[sp-28]+8]` (`ldr` at 0x3c85c, `ldmdb`
  at 0x3c864). The facts report no calls or conditional branches. Power-configuration object
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c81c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c820 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3c824 | load | 4 | `abs 0x3c868` | `-` |
| `ldr` at 0x3c828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c83c | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3c844 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c84c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c850 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3c854 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x3c858 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c85c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0x3c864 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c864 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c864 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c864 | `r0` | `[[sp-28]+8]` |

## `powerIsPresent`

- `powerIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c86c to 0x3c8fc.
  Established. Two signed bounds take the fallback for entry `r1 < 0x0` and entry `r1 >=
  [[sp-28]+8]`, respectively, falling through for the opposite relation (`blt` at 0x3c8b0, `bge` at
  0x3c8c4). The fallback stores word `0x0` at `sp -0x20` (`str` at 0x3c8d0). The other path reads a
  byte at `([[sp-28]+4] + r1) +0x0` and saves `[([[sp-28]+4] + r1)]:1` as a word at that stack
  target (`ldrb` at 0x3c8e8, `str` at 0x3c8ec). The merged r0 value and physical power-presence
  criterion remain unknown (`ldmdb` at 0x3c8f4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c87c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c880 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3c884 | load | 4 | `abs 0x3c8f8` | `-` |
| `ldr` at 0x3c888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c89c | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3c8a4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c8a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c8b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c8b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c8bc | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x3c8d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3c8d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c8dc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3c8e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c8e8 | load | 1 | `([[sp-28]+4] + r1) +0x0` | `-` |
| `str` at 0x3c8ec | store | 4 | `sp -0x20` | `[([[sp-28]+4] + r1)]:1` |
| `ldr` at 0x3c8f0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x3c8f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c8f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c8f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3c8b0 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3c8c4 | `r1 >= [[sp-28]+8] (signed, 32-bit)` | `r1 < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c8f4 | `r0` | `?` |

## `powerStatusAll`

- `powerStatusAll`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d8e0 to 0x3d970.
  Established. The initial call to `0x3d970` carries `0x0`, entry r1, entry r2, `0x0` (`bl` at
  0x3d8fc). A later call to that target has unknown, unknown, unknown, `[0x147fd0]` (`bl` at
  0x3d92c); its equality test takes the continuing edge for `ret(0x3d970) == [sp-24]` and falls
  through on inequality (`beq` at 0x3d93c). The joined return and aggregate power-status meaning
  remain unproven (`ldmdb` at 0x3d968).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d8f4 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x3d904 | store | 4 | `sp -0x18` | `ret(0x3d970)` |
| `str` at 0x3d90c | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x3d910 | load | 4 | `abs 0x3d96c` | `-` |
| `ldr` at 0x3d914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d918 | load | 4 | `abs 0x147fd0` | `-` |
| `ldr` at 0x3d928 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d934 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d944 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3d94c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d954 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x3d95c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d960 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3d964 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3d968 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d968 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d968 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d8fc | `(unknown callee) 0x3d970` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3d92c | `(unknown callee) 0x3d970` | `?` | `?` | `?` | `[0x147fd0]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3d920 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3d93c | `ret(0x3d970) == [sp-24]` | `ret(0x3d970) != [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d968 | `r0` | `?` |

## `powerStatus`

- `powerStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3db38 to 0x3dc58.
  Established. A nonzero result from the call to `0x3c86c` takes the continuation, while zero falls
  through to store word `0xfffff9b9` at `sp -0x24` (`bl` at 0x3db58, `bne` at 0x3db64, `str` at
  0x3db6c). The later byte test takes its continuation for `[[sp-28]+2]:1 != 0xff` and falls through
  on equality (`bne` at 0x3dbc0). The reached call to `0xd6c2c` carries `[sp-20]`, `[[sp-28]+2]:1`,
  unknown, `[[sp-28]+2]:1` (`bl` at 0x3dbe0). Status polarity, hardware interpretation and the
  merged r0 return are unknown (`ldmdb` at 0x3dc48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3db48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3db4c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3db50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3db54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3db68 | load | 4 | `abs 0x3dc4c` | `-` |
| `str` at 0x3db6c | store | 4 | `sp -0x24` | `0xfffff9b9` |
| `ldr` at 0x3db74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3db88 | load | 4 | `abs 0x3dc50` | `-` |
| `ldr` at 0x3db8c | load | 4 | `abs 0x147fcc` | `-` |
| `ldr` at 0x3db94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3dba8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3dbb0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3dbb4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3dbb8 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x3dbc4 | load | 4 | `abs 0x3dc54` | `-` |
| `str` at 0x3dbc8 | store | 4 | `sp -0x24` | `0xfffff9b8` |
| `ldr` at 0x3dbd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3dbd4 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x3dbd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3dbe8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3dbec | load | 1 | `[sp-28] +0x3` | `-` |
| `str` at 0x3dbf8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3dbfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3dc10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3dc20 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3dc28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3dc38 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3dc3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3dc40 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x3dc44 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x3dc48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dc48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dc48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3db58 | `(unknown callee) 0x3c86c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3dbe0 | `(unknown callee) 0xd6c2c` | `[sp-20]` | `[[sp-28]+2]:1` | `?` | `[[sp-28]+2]:1` |
| `bl` at 0x3dc00 | `(unknown callee) 0x406a4` | `[sp-28]` | `?` | `ret(0xd6c2c)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3db64 | `ret(0x3c86c) != 0x0` | `ret(0x3c86c) == 0x0` |
| `bne` at 0x3dbc0 | `[[sp-28]+2]:1 != 0xff` | `[[sp-28]+2]:1 == 0xff` |
| `beq` at 0x3dc0c | `ret(0x406a4) == 0x0` | `ret(0x406a4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dc48 | `r0` | `?` |

## `powerCycle`

- `powerCycle`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d858 to 0x3d8c0.
  Established. The first call targets `0x3d48c` with entry r0 through r3 (`bl` at 0x3d870); its zero
  result takes the continuing edge and nonzero falls through to save `ret(0x3d48c)` as a word at `sp
  -0x1c` (`beq` at 0x3d884, `str` at 0x3d88c). On continuation, `0x40ef8` has `0xc8`, unknown,
  unknown, `ret(0x3d48c)`, followed by `0x3cc04` with `0x1`, `[sp-20]`, unknown, unknown (`bl` at
  0x3d898, `bl` at 0x3d8a4). The interval units, physical power sequence and final r0 result remain
  unknown (`ldmdb` at 0x3d8bc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d868 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d86c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d878 | store | 4 | `sp -0x18` | `ret(0x3d48c)` |
| `ldr` at 0x3d87c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d888 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d88c | store | 4 | `sp -0x1c` | `ret(0x3d48c)` |
| `ldr` at 0x3d8a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d8ac | store | 4 | `sp -0x18` | `ret(0x3cc04)` |
| `ldr` at 0x3d8b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d8b4 | store | 4 | `sp -0x1c` | `ret(0x3cc04)` |
| `ldr` at 0x3d8b8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3d8bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d8bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d8bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d870 | `(unknown callee) 0x3d48c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3d898 | `(unknown callee) 0x40ef8` | `0xc8` | `?` | `?` | `ret(0x3d48c)` |
| `bl` at 0x3d8a4 | `(unknown callee) 0x3cc04` | `0x1` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d884 | `ret(0x3d48c) == 0x0` | `ret(0x3d48c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d8bc | `r0` | `?` |

## `powerResetDone`

- `powerResetDone`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x407dc to 0x40820.
  Established. The absolute word at `0x147fcc` is read before an indexed byte store whose effective
  target is unknown (`ldr` at 0x40808, `strb` at 0x40814). That byte store writes `0x0` (`strb` at
  0x40814). Entry r0 survives as the reported return value (`ldmdb` at 0x40818). Calls and
  conditional branches are absent, and a completed physical reset is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x407ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x407f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40804 | load | 4 | `abs 0x4081c` | `-` |
| `ldr` at 0x40808 | load | 4 | `abs 0x147fcc` | `-` |
| `strb` at 0x40814 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x40818 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40818 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40818 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40818 | `r0` | `r0` |
