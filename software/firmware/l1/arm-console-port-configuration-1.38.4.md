# ARM console and port configuration in 1.38.4

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
Two computed selectors in UtilPortStr remain unresolved. Tables and return values cover only known edges, with unresolved sites listed below that entry. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/console-port-configuration-facts.txt) retain all tool sections.

## `promIceEnabled`

- `promIceEnabled`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44d64 to 0x44e78.
  Established. The call to `0x45180` has `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)` (`bl` at
  0x44d8c); its nonzero result takes the bypass and zero falls through (`bne` at 0x44da0). The next
  result test takes its bypass for `ret(0x44d04) == 0x0` and falls through for nonzero (`beq` at
  0x44db8). PROM-ICE meaning and final r0 remain unknown (`ldmdb` at 0x44e6c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44d78 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x44d7c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x44d94 | store | 4 | `sp -0x1c` | `ret(0x45180)` |
| `ldr` at 0x44d98 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x44da4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x44dbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x44dc0 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x44dc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x44dc8 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0x44de0 | store | 4 | `sp -0x1c` | `ret(0xd6b1c)` |
| `ldr` at 0x44de4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x44de8 | store | 4 | `sp -0x24` | `ret(0xd6b1c)` |
| `ldr` at 0x44dec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x44df8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x44e0c | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x44e18 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x44e20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x44e2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x44e38 | load | 4 | `abs 0x44e70` | `-` |
| `str` at 0x44e3c | store | 4 | `sp -0x30` | `0x109164` |
| `ldrb` at 0x44e40 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x44e44 | store | 4 | `sp -0x2c` | `[sp-17]:1` |
| `str` at 0x44e48 | store | 4 | `sp -0x28` | `ret(0x219a8)` |
| `ldr` at 0x44e50 | load | 4 | `abs 0x44e74` | `-` |
| `str` at 0x44e64 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x44e68 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x44e6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44e6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44e6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44d8c | `(unknown callee) 0x45180` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x44dac | `(unknown callee) 0x44d04` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x44dd8 | `(unknown callee) 0xd6b1c` | `0x0` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `[[sp-24]+1]:1` |
| `bl` at 0x44e30 | `(unknown callee) 0x219a8` | `?` | `?` | `?` | `?` |
| `bl` at 0x44e5c | `(unknown callee) 0x21e28` | `0x1` | `0x109110` | `0xa9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x44da0 | `ret(0x45180) != 0x0` | `ret(0x45180) == 0x0` |
| `beq` at 0x44db8 | `ret(0x44d04) == 0x0` | `ret(0x44d04) != 0x0` |
| `beq` at 0x44df4 | `ret(0xd6b1c) == 0x0` | `ret(0xd6b1c) != 0x0` |
| `beq` at 0x44e00 | `ret(0xd6b1c) == 0x1` | `ret(0xd6b1c) != 0x1` |
| `beq` at 0x44e28 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44e6c | `r0` | `?` |

## `SMPInit`

- `SMPInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x66314 to 0x667ec.
  Established. A word store advances absolute `0x14852c` to `[0x14852c]+1` (`str` at 0x66364). Other
  word stores write `0x0` at `0x2b0d0c` and `0x3` at `0x2b0d10` (`str` at 0x66370, `str` at
  0x6637c). The opening global test takes its bypass for zero and falls through for nonzero (`beq`
  at 0x66334). The first call to `0x21e28` carries `0x9`, `0x10e990`, `0x62`, `0x0` (`bl` at
  0x66350). Protocol initialization ownership and exit r0 remain unestablished (`ldmdb` at 0x66750).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x66324 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x66328 | load | 4 | `abs 0x66754` | `-` |
| `ldr` at 0x6632c | load | 4 | `abs 0x14852c` | `-` |
| `ldr` at 0x66338 | load | 4 | `abs 0x66758` | `-` |
| `str` at 0x6633c | store | 4 | `sp -0x40` | `0x10e970` |
| `ldr` at 0x66344 | load | 4 | `abs 0x6675c` | `-` |
| `ldr` at 0x66354 | load | 4 | `abs 0x66754` | `-` |
| `ldr` at 0x66358 | load | 4 | `abs 0x66754` | `-` |
| `ldr` at 0x6635c | load | 4 | `abs 0x14852c` | `-` |
| `str` at 0x66364 | store | 4 | `abs 0x14852c` | `[0x14852c]+1` |
| `ldr` at 0x66368 | load | 4 | `abs 0x66760` | `-` |
| `str` at 0x66370 | store | 4 | `abs 0x2b0d0c` | `0x0` |
| `ldr` at 0x66374 | load | 4 | `abs 0x66760` | `-` |
| `str` at 0x6637c | store | 4 | `abs 0x2b0d10` | `0x3` |
| `str` at 0x66384 | store | 4 | `sp -0x24` | `0x14` |
| `ldr` at 0x66388 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x6638c | load | 4 | `abs 0x66764` | `-` |
| `str` at 0x66390 | store | 4 | `abs 0x2b0d64` | `0x2b0d14` |
| `ldr` at 0x66394 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x66398 | load | 4 | `abs 0x2b0d64` | `-` |
| `strb` at 0x663a0 | store | 1 | `[0x2b0d64] +0x0` | `0x0` |
| `ldr` at 0x663a4 | load | 4 | `abs 0x66768` | `-` |
| `str` at 0x663b4 | store | 4 | `abs 0x148564` | `?` |
| `ldr` at 0x663b8 | load | 4 | `abs 0x66760` | `-` |
| `str` at 0x663c0 | store | 4 | `abs 0x2b13c8` | `0x0` |
| `ldr` at 0x663c4 | load | 4 | `abs 0x66760` | `-` |
| `str` at 0x663cc | store | 4 | `abs 0x2b13cc` | `0x0` |
| `ldr` at 0x663d0 | load | 4 | `abs 0x6676c` | `-` |
| `ldr` at 0x663d4 | load | 4 | `abs 0x66770` | `-` |
| `str` at 0x663e4 | store | 4 | `sp -0x1c` | `ret(0xb5654)` |
| `ldr` at 0x663e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x663f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66400 | load | 4 | `abs 0x66774` | `-` |
| `str` at 0x66404 | store | 4 | `sp -0x40` | `0x10e9b0` |
| `str` at 0x66408 | store | 4 | `sp -0x3c` | `ret(0x21674)` |
| `ldr` at 0x66410 | load | 4 | `abs 0x6675c` | `-` |
| `ldr` at 0x66420 | load | 4 | `abs 0x66778` | `-` |
| `ldr` at 0x66424 | load | 4 | `abs 0x6677c` | `-` |
| `str` at 0x66434 | store | 4 | `sp -0x1c` | `ret(0xb5654)` |
| `ldr` at 0x66438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66450 | load | 4 | `abs 0x66780` | `-` |
| `str` at 0x66454 | store | 4 | `sp -0x40` | `0x10e9dc` |
| `str` at 0x66458 | store | 4 | `sp -0x3c` | `ret(0x21674)` |
| `ldr` at 0x66460 | load | 4 | `abs 0x6675c` | `-` |
| `ldr` at 0x66470 | load | 4 | `abs 0x66784` | `-` |
| `ldr` at 0x66474 | load | 4 | `abs 0x66788` | `-` |
| `str` at 0x66484 | store | 4 | `sp -0x1c` | `ret(0xb5654)` |
| `ldr` at 0x66488 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66494 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x664a0 | load | 4 | `abs 0x6678c` | `-` |
| `str` at 0x664a4 | store | 4 | `sp -0x40` | `0x10ea0c` |
| `str` at 0x664a8 | store | 4 | `sp -0x3c` | `ret(0x21674)` |
| `ldr` at 0x664b0 | load | 4 | `abs 0x6675c` | `-` |
| `ldr` at 0x664c0 | load | 4 | `abs 0x66790` | `-` |
| `ldr` at 0x664c4 | load | 4 | `abs 0x66794` | `-` |
| `ldr` at 0x664d4 | load | 4 | `abs 0x66798` | `-` |
| `ldr` at 0x664d8 | load | 4 | `abs 0x6679c` | `-` |
| `str` at 0x664ec | store | 4 | `sp -0x40` | `0x100` |
| `str` at 0x664f4 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x664fc | store | 4 | `sp -0x38` | `0x1` |
| `str` at 0x66504 | store | 4 | `sp -0x34` | `0x10` |
| `str` at 0x6650c | store | 4 | `sp -0x30` | `0x410` |
| `ldr` at 0x66510 | load | 4 | `abs 0x667a0` | `-` |
| `str` at 0x66514 | store | 4 | `sp -0x2c` | `0x68858` |
| `str` at 0x6651c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x66520 | load | 4 | `abs 0x667a4` | `-` |
| `ldr` at 0x66524 | load | 4 | `abs 0x667a8` | `-` |
| `ldr` at 0x6652c | load | 4 | `abs 0x667ac` | `-` |
| `ldr` at 0x66534 | load | 4 | `abs 0x66760` | `-` |
| `str` at 0x66550 | store | 4 | `sp -0x20` | `ret(0x74908)` |
| `ldr` at 0x66554 | load | 4 | `abs 0x667b0` | `-` |
| `ldr` at 0x66558 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6655c | load | 4 | `ret(0x74908) +0x8` | `-` |
| `str` at 0x66560 | store | 4 | `abs 0x1485f0` | `[ret(0x74908)+8]` |
| `ldr` at 0x66564 | load | 4 | `abs 0x667b4` | `-` |
| `ldr` at 0x66568 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6656c | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0x66570 | store | 4 | `abs 0x1485f4` | `[[sp-32]+12]` |
| `ldr` at 0x66574 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x66578 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x6657c | load | 4 | `abs 0x2b0a80` | `-` |
| `str` at 0x66594 | store | 4 | `abs 0x2b13d0` | `?` |
| `ldr` at 0x66598 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x6659c | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x665a0 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x665a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x665ac | load | 4 | `[sp-32] +0x14` | `-` |
| `str` at 0x665bc | store | 4 | `abs 0x2b13d0` | `?` |
| `ldr` at 0x665c0 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x665c4 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x665c8 | load | 4 | `abs 0x2b0a80` | `-` |
| `str` at 0x665e0 | store | 4 | `abs 0x2b13d8` | `?` |
| `ldr` at 0x665e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x665f8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x665fc | load | 4 | `abs 0x667b8` | `-` |
| `str` at 0x66600 | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x66608 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x66610 | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x66618 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x6661c | load | 4 | `abs 0x667bc` | `-` |
| `ldr` at 0x66620 | load | 4 | `abs 0x667c0` | `-` |
| `ldr` at 0x66624 | load | 4 | `abs 0x667c4` | `-` |
| `str` at 0x66634 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x66638 | load | 4 | `abs 0x667b8` | `-` |
| `str` at 0x6663c | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x66644 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x6664c | store | 4 | `sp -0x34` | `0x28` |
| `str` at 0x66654 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x66658 | load | 4 | `abs 0x667c8` | `-` |
| `ldr` at 0x6665c | load | 4 | `abs 0x667cc` | `-` |
| `ldr` at 0x66660 | load | 4 | `abs 0x667d0` | `-` |
| `str` at 0x66670 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x66674 | load | 4 | `abs 0x667b8` | `-` |
| `str` at 0x66678 | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x66680 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x66688 | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x66690 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x66694 | load | 4 | `abs 0x667d4` | `-` |
| `ldr` at 0x66698 | load | 4 | `abs 0x667d8` | `-` |
| `ldr` at 0x6669c | load | 4 | `abs 0x667dc` | `-` |
| `ldr` at 0x666a8 | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x666ac | load | 4 | `abs 0x667e0` | `-` |
| `ldr` at 0x666b4 | load | 4 | `abs 0x66784` | `-` |
| `str` at 0x666c0 | store | 4 | `sp -0x1c` | `ret(0xb4220)` |
| `ldr` at 0x666c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x666d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x666dc | load | 4 | `abs 0x667e4` | `-` |
| `str` at 0x666e0 | store | 4 | `sp -0x40` | `0x10ea6c` |
| `str` at 0x666e4 | store | 4 | `sp -0x3c` | `ret(0x21674)` |
| `ldr` at 0x666ec | load | 4 | `abs 0x6675c` | `-` |
| `ldr` at 0x666fc | load | 4 | `abs 0x66760` | `-` |
| `ldr` at 0x66700 | load | 4 | `abs 0x667e0` | `-` |
| `ldr` at 0x66708 | load | 4 | `abs 0x6676c` | `-` |
| `str` at 0x66714 | store | 4 | `sp -0x1c` | `ret(0xb4220)` |
| `ldr` at 0x66718 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66724 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66730 | load | 4 | `abs 0x667e8` | `-` |
| `str` at 0x66734 | store | 4 | `sp -0x40` | `0x10ea90` |
| `str` at 0x66738 | store | 4 | `sp -0x3c` | `ret(0x21674)` |
| `ldr` at 0x66740 | load | 4 | `abs 0x6675c` | `-` |
| `ldmdb` at 0x66750 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x66750 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x66750 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x66750 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x66350 | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x62` | `0x0` |
| `bl` at 0x663a8 | `(unknown callee) 0x4f090` | `?` | `?` | `[0x2b0d64]` | `0x0` |
| `bl` at 0x663dc | `(unknown callee) 0xb5654` | `0x2b0a90` | `0x10e9a4` | `0x19` | `0x0` |
| `bl` at 0x663f8 | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0x6641c | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x80` | `0x0` |
| `bl` at 0x6642c | `(unknown callee) 0xb5654` | `0x2b0b28` | `0x10e9d0` | `0x19` | `?` |
| `bl` at 0x66448 | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0x6646c | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x83` | `0x0` |
| `bl` at 0x6647c | `(unknown callee) 0xb5654` | `0x2b0bc0` | `0x10ea00` | `0x19` | `?` |
| `bl` at 0x66498 | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0x664bc | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x86` | `0x0` |
| `bl` at 0x664d0 | `(unknown callee) 0x22920` | `0x2b0c58` | `0x10ea30` | `0x1` | `0x9` |
| `bl` at 0x664e4 | `(unknown callee) 0x22920` | `0x2b0cac` | `0x10ea3c` | `0x1` | `0x9` |
| `bl` at 0x66530 | `(unknown callee) 0xba79c` | `0x2b13dc` | `0x2b1418` | `0x100` | `0x2b1518` |
| `bl` at 0x66540 | `(unknown callee) 0xb553c` | `0x2b0a80` | `0x2` | `0x0` | `?` |
| `bl` at 0x66548 | `(unknown callee) 0x74908` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x665e4 | `(unknown callee) 0x68ae4` | `ret(0x74908)` | `0x2b0a80` | `0x2b0a80` | `?` |
| `bl` at 0x6662c | `(unknown callee) 0x223f0` | `0x2b0d00` | `0x10ea48` | `0x668c8` | `0x0` |
| `bl` at 0x66668 | `(unknown callee) 0x223f0` | `0x2b0d04` | `0x10ea54` | `0x66da8` | `0x0` |
| `bl` at 0x666a4 | `(unknown callee) 0x223f0` | `0x2b0d08` | `0x10ea60` | `0x67acc` | `0x0` |
| `bl` at 0x666b8 | `(unknown callee) 0xb4220` | `0x2b0a80` | `0xffff002` | `0x1` | `0x2b0bc0` |
| `bl` at 0x666d4 | `(unknown callee) 0x21674` | `ret(0xb4220)` | `?` | `?` | `ret(0xb4220)` |
| `bl` at 0x666f8 | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0xd0` | `0x0` |
| `bl` at 0x6670c | `(unknown callee) 0xb4220` | `0x2b0a80` | `0xffff002` | `0x2` | `0x2b0a90` |
| `bl` at 0x66728 | `(unknown callee) 0x21674` | `ret(0xb4220)` | `?` | `?` | `ret(0xb4220)` |
| `bl` at 0x6674c | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0xdc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x66334 | `[0x14852c] == 0x0` | `[0x14852c] != 0x0` |
| `beq` at 0x663f0 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `beq` at 0x66440 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `beq` at 0x66490 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `bne` at 0x665f0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x666cc | `ret(0xb4220) == 0x0` | `ret(0xb4220) != 0x0` |
| `beq` at 0x66720 | `ret(0xb4220) == 0x0` | `ret(0xb4220) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x66750 | `r0` | `?` |

## `SMPLock`

- `SMPLock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x667ec to 0x6687c.
  Established. The call to `0x23ec4` receives entry r0 through r3 (`bl` at 0x66804); its nonzero
  result takes the continuation and zero falls through (`bne` at 0x66818). A later word store writes
  `ret(0x23ec4)` at absolute `0x148530` (`str` at 0x6684c). Lock ownership and the joined return
  value remain unknown (`ldmdb` at 0x6686c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x667fc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x66800 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6680c | store | 4 | `sp -0x18` | `ret(0x23ec4)` |
| `ldr` at 0x66810 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x66820 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x66828 | load | 4 | `abs 0x66870` | `-` |
| `ldr` at 0x6682c | load | 4 | `abs 0x2b0a8c` | `-` |
| `ldr` at 0x66838 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6683c | store | 4 | `sp -0x1c` | `ret(0x23ec4)` |
| `ldr` at 0x66844 | load | 4 | `abs 0x66874` | `-` |
| `ldr` at 0x66848 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6684c | store | 4 | `abs 0x148530` | `ret(0x23ec4)` |
| `ldr` at 0x66850 | load | 4 | `abs 0x66874` | `-` |
| `ldr` at 0x66854 | load | 4 | `abs 0x148530` | `-` |
| `ldr` at 0x66858 | load | 4 | `abs 0x66878` | `-` |
| `ldr` at 0x66860 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x66864 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x66868 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6686c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6686c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6686c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x66804 | `(unknown callee) 0x23ec4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x6685c | `(unknown callee) 0xca450` | `[0x148530]` | `0x148568` | `0x148530` | `0x148530` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x66818 | `ret(0x23ec4) != 0x0` | `ret(0x23ec4) == 0x0` |
| `bne` at 0x66834 | `[0x2b0a8c] != 0x0` | `[0x2b0a8c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6686c | `r0` | `?` |

## `SMPUnlock`

- `SMPUnlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6687c to 0x668c8.
  Established. The initial global test takes its continuation for `[0x148530] != 0x0` and falls
  through on zero (`bne` at 0x66894). The call to `0xca48c` has `[0x148530]`, `0x148568`, entry r2,
  `0x148530` (`bl` at 0x668a8). A later word store clears absolute `0x148530` (`str` at 0x668b4).
  The call sequence’s purpose and r0 at exit remain unresolved (`ldmdb` at 0x668bc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x66888 | load | 4 | `abs 0x668c0` | `-` |
| `ldr` at 0x6688c | load | 4 | `abs 0x148530` | `-` |
| `ldr` at 0x6689c | load | 4 | `abs 0x668c0` | `-` |
| `ldr` at 0x668a0 | load | 4 | `abs 0x148530` | `-` |
| `ldr` at 0x668a4 | load | 4 | `abs 0x668c4` | `-` |
| `ldr` at 0x668ac | load | 4 | `abs 0x668c0` | `-` |
| `str` at 0x668b4 | store | 4 | `abs 0x148530` | `0x0` |
| `ldmdb` at 0x668bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x668bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x668bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x668a8 | `(unknown callee) 0xca48c` | `[0x148530]` | `0x148568` | `r2` | `0x148530` |
| `bl` at 0x668b8 | `(unknown callee) 0x23f10` | `ret(0xca48c)` | `?` | `0x148530` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x66894 | `[0x148530] != 0x0` | `[0x148530] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x668bc | `r0` | `?` |

## `SMPLibrary`

- `SMPLibrary`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67200 to 0x67acc.
  Established. A word store writes `0x51` at absolute `0x2b1620` (`str` at 0x67290). Other word
  stores write `ret(0x1b124)` at `[sp-32] +0x0` and `[[sp-32]]` at absolute `0x2b1618` (`str` at
  0x672a0, `str` at 0x672a8). The opening call to `0x22a38` carries `0x2b0cac`, unknown, entry r2,
  `0x0` (`bl` at 0x67230); its zero result takes the bypass and nonzero falls through (`beq` at
  0x67244). Console command interpretation and final r0 remain unidentified (`ldmdb` at 0x67a58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x67210 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x67214 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x6721c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x67224 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x67228 | load | 4 | `abs 0x67a5c` | `-` |
| `str` at 0x67238 | store | 4 | `sp -0x28` | `ret(0x22a38)` |
| `ldr` at 0x6723c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67248 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67254 | load | 4 | `abs 0x67a60` | `-` |
| `str` at 0x67258 | store | 4 | `sp -0x90` | `0x10ebf0` |
| `str` at 0x6725c | store | 4 | `sp -0x8c` | `ret(0x21674)` |
| `ldr` at 0x67264 | load | 4 | `abs 0x67a64` | `-` |
| `ldr` at 0x67268 | load | 4 | `abs 0x67a68` | `-` |
| `ldr` at 0x67274 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67280 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67284 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67288 | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x67290 | store | 4 | `abs 0x2b1620` | `0x51` |
| `str` at 0x672a0 | store | 4 | `[sp-32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x672a4 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x672a8 | store | 4 | `abs 0x2b1618` | `[[sp-32]]` |
| `ldr` at 0x672ac | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x672b0 | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x672b8 | store | 4 | `abs 0x2b161c` | `0x0` |
| `ldr` at 0x672bc | load | 4 | `abs 0x2b1618` | `-` |
| `str` at 0x672c4 | store | 4 | `[0x2b1618] +0x0` | `0x0` |
| `ldr` at 0x672cc | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x672d4 | store | 4 | `abs 0x2b1618` | `0x0` |
| `ldr` at 0x672d8 | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x672e0 | store | 4 | `abs 0x2b161c` | `0x0` |
| `ldr` at 0x672e4 | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x672ec | store | 4 | `abs 0x2b1620` | `0x0` |
| `ldr` at 0x672f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x672f4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67300 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67304 | load | 4 | `abs 0x2b0d0c` | `-` |
| `ldr` at 0x67310 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67314 | load | 4 | `abs 0x67a70` | `-` |
| `ldr` at 0x6732c | load | 4 | `abs 0x67a5c` | `-` |
| `str` at 0x67338 | store | 4 | `sp -0x28` | `ret(0x22dc0)` |
| `ldr` at 0x6733c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67348 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67354 | load | 4 | `abs 0x67a74` | `-` |
| `str` at 0x67358 | store | 4 | `sp -0x90` | `0x10ec18` |
| `str` at 0x6735c | store | 4 | `sp -0x8c` | `ret(0x21674)` |
| `ldr` at 0x67364 | load | 4 | `abs 0x67a64` | `-` |
| `ldr` at 0x67368 | load | 4 | `abs 0x67a78` | `-` |
| `str` at 0x67378 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x67380 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67384 | load | 4 | `abs 0x2b0a88` | `-` |
| `ldr` at 0x67388 | load | 4 | `abs 0x67a7c` | `-` |
| `ldr` at 0x67390 | load | 4 | `abs 0x67a7c` | `-` |
| `strb` at 0x67398 | store | 1 | `abs 0x1485e9` | `0x1` |
| `ldr` at 0x6739c | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x673a0 | load | 4 | `abs 0x2b0a88` | `-` |
| `ldr` at 0x673a4 | load | 4 | `abs 0x67a7c` | `-` |
| `ldr` at 0x673ac | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x673b4 | store | 4 | `abs 0x2b0d0c` | `0x1` |
| `ldr` at 0x673b8 | load | 4 | `abs 0x67a5c` | `-` |
| `str` at 0x673c4 | store | 4 | `sp -0x28` | `ret(0x22dc0)` |
| `ldr` at 0x673c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x673d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x673e0 | load | 4 | `abs 0x67a74` | `-` |
| `str` at 0x673e4 | store | 4 | `sp -0x90` | `0x10ec18` |
| `str` at 0x673e8 | store | 4 | `sp -0x8c` | `ret(0x21674)` |
| `ldr` at 0x673f0 | load | 4 | `abs 0x67a64` | `-` |
| `ldr` at 0x673f4 | load | 4 | `abs 0x67a80` | `-` |
| `str` at 0x67404 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x6740c | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67410 | load | 4 | `abs 0x2b0d10` | `-` |
| `str` at 0x67414 | store | 4 | `sp -0x88` | `[0x2b0d10]` |
| `ldr` at 0x67418 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67424 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67430 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x67440 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67444 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67450 | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x67458 | store | 4 | `abs 0x2b0d10` | `0x2` |
| `ldr` at 0x6745c | load | 4 | `abs 0x67a84` | `-` |
| `ldr` at 0x67464 | load | 4 | `abs 0x67a88` | `-` |
| `ldr` at 0x6746c | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67470 | load | 4 | `abs 0x67a8c` | `-` |
| `str` at 0x67474 | store | 4 | `abs 0x2b0d64` | `0x2b0d14` |
| `ldr` at 0x6747c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x67488 | store | 4 | `sp -0x24` | `ret(0x68308)` |
| `ldr` at 0x67490 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67494 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x674a0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x674a4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x674b0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x674b4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x674c0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x674c4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x674d0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x674d4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x674e0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x674e4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x674f0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x674f4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67500 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67504 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67510 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67514 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67520 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67524 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67530 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67534 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67540 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67544 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67550 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67554 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67560 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67564 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67570 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67574 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67580 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67584 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x67590 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x67594 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x675a0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x675a4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x675b0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x675b4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x675c0 | load | 4 | `abs 0x67a90` | `-` |
| `ldr` at 0x675c4 | load | 4 | `abs 0x148564` | `-` |
| `ldr` at 0x675d0 | load | 4 | `abs 0x67a94` | `-` |
| `ldr` at 0x675ec | load | 4 | `abs 0x67a98` | `-` |
| `ldr` at 0x675f4 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x675f8 | load | 4 | `abs 0x2b0d10` | `-` |
| `ldr` at 0x67604 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67608 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x6761c | load | 4 | `abs 0x67a9c` | `-` |
| `ldr` at 0x67630 | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x67638 | store | 4 | `abs 0x2b0d10` | `0x1` |
| `ldr` at 0x67640 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67644 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67650 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67654 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67664 | load | 4 | `abs 0x67a8c` | `-` |
| `ldr` at 0x67678 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x6767c | load | 4 | `abs 0x2b13c8` | `-` |
| `ldr` at 0x67690 | load | 4 | `abs 0x67aa0` | `-` |
| `ldr` at 0x6769c | load | 4 | `abs 0x67a8c` | `-` |
| `ldr` at 0x676a4 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x676a8 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x676ac | load | 4 | `abs 0x2b13c8` | `-` |
| `ldr` at 0x676b4 | load | 4 | `abs 0x67aa4` | `-` |
| `str` at 0x676dc | store | 4 | `abs 0x2b13c8` | `?` |
| `ldr` at 0x676e0 | load | 4 | `abs 0x67aa8` | `-` |
| `ldr` at 0x676e4 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x676e8 | load | 4 | `abs 0x2b13c8` | `-` |
| `str` at 0x676ec | store | 4 | `abs 0x14853c` | `[0x2b13c8]` |
| `ldr` at 0x676f0 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x676f4 | load | 4 | `abs 0x2b13cc` | `-` |
| `ldr` at 0x67700 | load | 4 | `abs 0x67aac` | `-` |
| `ldr` at 0x67704 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67708 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x6770c | load | 4 | `abs 0x2b13cc` | `-` |
| `str` at 0x67714 | store | 4 | `abs 0x2b13cc` | `[0x2b13cc]+1` |
| `str` at 0x67718 | store | 4 | `abs 0x148540` | `[0x2b13cc]+1` |
| `ldr` at 0x67720 | load | 4 | `abs 0x67aac` | `-` |
| `str` at 0x67728 | store | 4 | `abs 0x148540` | `0x14` |
| `ldr` at 0x6772c | load | 4 | `abs 0x67ab0` | `-` |
| `str` at 0x6773c | store | 4 | `sp -0x24` | `ret(0x68430)` |
| `ldr` at 0x67740 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67744 | load | 4 | `abs 0x67a8c` | `-` |
| `str` at 0x67748 | store | 4 | `abs 0x2b0d64` | `0x2b0d14` |
| `ldr` at 0x6774c | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67750 | load | 4 | `abs 0x2b0d64` | `-` |
| `strb` at 0x67758 | store | 1 | `[0x2b0d64] +0x0` | `0x0` |
| `ldr` at 0x6775c | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67760 | load | 4 | `abs 0x2b0d10` | `-` |
| `ldr` at 0x6777c | load | 4 | `abs 0x67a98` | `-` |
| `ldr` at 0x67784 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67788 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x6779c | load | 4 | `abs 0x67ab4` | `-` |
| `ldr` at 0x677a4 | load | 4 | `abs 0x67a6c` | `-` |
| `str` at 0x677ac | store | 4 | `abs 0x2b0d10` | `0x1` |
| `ldr` at 0x677c0 | load | 4 | `abs 0x67a88` | `-` |
| `ldr` at 0x677cc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x677d0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x677dc | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x677e0 | load | 4 | `abs 0x2b0d64` | `-` |
| `ldr` at 0x677e4 | load | 4 | `abs 0x67a8c` | `-` |
| `ldr` at 0x677f0 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x677f4 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x677f8 | load | 4 | `abs 0x2b0d64` | `-` |
| `str` at 0x67804 | store | 4 | `abs 0x2b0d64` | `[0x2b0d64]-1` |
| `strb` at 0x6780c | store | 1 | `[0x2b0d64] -0x1` | `0x0` |
| `ldr` at 0x67810 | load | 4 | `abs 0x67ab8` | `-` |
| `ldr` at 0x6781c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67820 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x6782c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67830 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x67840 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x67844 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x67854 | store | 4 | `sp -0x7c` | `?` |
| `str` at 0x6785c | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x67868 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x6786c | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67870 | load | 4 | `abs 0x2b13cc` | `-` |
| `str` at 0x67874 | store | 4 | `sp -0x80` | `[0x2b13cc]` |
| `ldr` at 0x67878 | load | 4 | `abs 0x67aac` | `-` |
| `ldr` at 0x6787c | load | 4 | `abs 0x148540` | `-` |
| `ldr` at 0x67880 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x67894 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67898 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x6789c | load | 4 | `abs 0x2b0d64` | `-` |
| `str` at 0x678a8 | store | 4 | `abs 0x2b0d64` | `[0x2b0d64]-1` |
| `ldr` at 0x678ac | load | 4 | `abs 0x67abc` | `-` |
| `ldr` at 0x678bc | load | 4 | `abs 0x67ab8` | `-` |
| `ldr` at 0x678c8 | load | 4 | `abs 0x67aa8` | `-` |
| `ldr` at 0x678cc | load | 4 | `abs 0x67aa8` | `-` |
| `ldr` at 0x678d0 | load | 4 | `abs 0x14853c` | `-` |
| `ldr` at 0x678d4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x678e0 | load | 4 | `abs 0x67aa4` | `-` |
| `str` at 0x67908 | store | 4 | `abs 0x14853c` | `?` |
| `ldr` at 0x6790c | load | 4 | `abs 0x67aac` | `-` |
| `ldr` at 0x67910 | load | 4 | `abs 0x67aac` | `-` |
| `ldr` at 0x67914 | load | 4 | `abs 0x148540` | `-` |
| `ldr` at 0x67918 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x67920 | store | 4 | `abs 0x148540` | `([0x148540] + [sp-124])` |
| `ldr` at 0x67924 | load | 4 | `abs 0x67aa8` | `-` |
| `ldr` at 0x67928 | load | 4 | `abs 0x14853c` | `-` |
| `ldr` at 0x6793c | load | 4 | `abs 0x67aa0` | `-` |
| `ldr` at 0x67944 | load | 4 | `abs 0x67a8c` | `-` |
| `ldr` at 0x67950 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67954 | load | 4 | `abs 0x67a8c` | `-` |
| `ldr` at 0x67964 | load | 4 | `abs 0x67a8c` | `-` |
| `str` at 0x6796c | store | 4 | `abs 0x2b0d64` | `ret(0x1b22c)+0x2b0d14` |
| `ldr` at 0x67970 | load | 4 | `abs 0x67a8c` | `-` |
| `ldr` at 0x6797c | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67980 | load | 4 | `abs 0x2b0d64` | `-` |
| `ldr` at 0x67984 | load | 4 | `abs 0x67ac0` | `-` |
| `ldr` at 0x67990 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x67998 | load | 4 | `abs 0x2b0d64` | `-` |
| `ldr` at 0x6799c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x679a4 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x679a8 | store | 1 | `[0x2b0d64] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x679b0 | store | 4 | `abs 0x2b0d64` | `[0x2b0d64]+1` |
| `ldr` at 0x679b4 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x679b8 | load | 4 | `abs 0x2b0d64` | `-` |
| `strb` at 0x679c0 | store | 1 | `[0x2b0d64] +0x0` | `0x0` |
| `ldr` at 0x679c8 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x679d0 | load | 4 | `abs 0x2b0d64` | `-` |
| `ldr` at 0x679d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x679dc | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x679e0 | store | 1 | `(unknown)` | `([[sp-28]]:1 & 0xff)` |
| `ldr` at 0x679e4 | load | 4 | `abs 0x67a6c` | `-` |
| `ldr` at 0x679e8 | load | 4 | `abs 0x2b0d64` | `-` |
| `strb` at 0x679f0 | store | 1 | `[0x2b0d64] +0x0` | `0x0` |
| `ldr` at 0x679f4 | load | 4 | `abs 0x67ac4` | `-` |
| `ldr` at 0x679fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67a04 | load | 4 | `abs 0x67a5c` | `-` |
| `str` at 0x67a10 | store | 4 | `sp -0x28` | `ret(0x22dc0)` |
| `ldr` at 0x67a14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67a20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x67a2c | load | 4 | `abs 0x67a74` | `-` |
| `str` at 0x67a30 | store | 4 | `sp -0x90` | `0x10ec18` |
| `str` at 0x67a34 | store | 4 | `sp -0x8c` | `ret(0x21674)` |
| `ldr` at 0x67a3c | load | 4 | `abs 0x67a64` | `-` |
| `ldr` at 0x67a40 | load | 4 | `abs 0x67ac8` | `-` |
| `ldr` at 0x67a4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x67a50 | store | 4 | `sp -0x84` | `[sp-36]` |
| `ldr` at 0x67a54 | load | 4 | `sp -0x84` | `-` |
| `ldmdb` at 0x67a58 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x67a58 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x67a58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x67a58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x67a58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67230 | `(unknown callee) 0x22a38` | `0x2b0cac` | `?` | `r2` | `0x0` |
| `bl` at 0x6724c | `(unknown callee) 0x21674` | `ret(0x22a38)` | `?` | `?` | `ret(0x22a38)` |
| `bl` at 0x67270 | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x286` | `0x0` |
| `bl` at 0x67298 | `(unknown callee) 0x1b124` | `0x144` | `?` | `0x2b0a80` | `0x51` |
| `bl` at 0x6731c | `(unknown callee) 0x68680` | `0x2b0a80` | `0x10ec14` | `0x1` | `[0x2b0d0c]` |
| `bl` at 0x67330 | `(unknown callee) 0x22dc0` | `0x2b0cac` | `?` | `?` | `ret(0x68680)` |
| `bl` at 0x6734c | `(unknown callee) 0x21674` | `ret(0x22dc0)` | `?` | `?` | `ret(0x22dc0)` |
| `bl` at 0x67370 | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x29e` | `0x0` |
| `bl` at 0x6738c | `(unknown callee) 0xca450` | `[0x2b0a88]` | `0x1485d4` | `?` | `0x2b0a80` |
| `bl` at 0x673a8 | `(unknown callee) 0xca48c` | `[0x2b0a88]` | `0x1485d4` | `0x1485d4` | `0x2b0a80` |
| `bl` at 0x673bc | `(unknown callee) 0x22dc0` | `0x2b0cac` | `?` | `0x2b0a80` | `0x1` |
| `bl` at 0x673d8 | `(unknown callee) 0x21674` | `ret(0x22dc0)` | `?` | `?` | `ret(0x22dc0)` |
| `bl` at 0x673fc | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x2b3` | `0x0` |
| `bl` at 0x67460 | `(unknown callee) 0x688b8` | `0x10ec3c` | `?` | `0x2b0a80` | `0x2` |
| `bl` at 0x67468 | `(unknown callee) 0x688b8` | `0x2b0d68` | `?` | `?` | `?` |
| `bl` at 0x67480 | `(unknown callee) 0x68308` | `[sp-28]` | `?` | `[0x2b0d10]` | `[[sp-28]]:1` |
| `bl` at 0x675d4 | `(unknown callee) 0x688b8` | `0x10ec60` | `?` | `[0x2b0d10]` | `[0x148564]` |
| `bl` at 0x675dc | `(unknown callee) 0x3d8e0` | `?` | `?` | `[0x2b0d10]` | `[0x148564]` |
| `bl` at 0x675f0 | `(unknown callee) 0x688b8` | `0x10ec84` | `?` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x6760c | `(unknown callee) 0x697f4` | `[0x2b13d0]` | `?` | `?` | `0x2b0a80` |
| `bl` at 0x67620 | `(unknown callee) 0x1b49c` | `sp-120` | `0x10ecb8` | `ret(0x697f4)` | `sp-120` |
| `bl` at 0x6762c | `(unknown callee) 0x688b8` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x67668 | `(unknown callee) 0x1b22c` | `0x2b0d14` | `?` | `[0x2b0d10]` | `[[sp-28]]:1` |
| `bl` at 0x676a0 | `(unknown callee) 0x1b508` | `?` | `0x2b0d14` | `?` | `?` |
| `bl` at 0x67730 | `(unknown callee) 0x688b8` | `0x10ecf0` | `?` | `?` | `?` |
| `bl` at 0x67734 | `(unknown callee) 0x68430` | `ret(0x688b8)` | `?` | `?` | `?` |
| `bl` at 0x6776c | `(unknown callee) 0x3d8e0` | `ret(0x68430)` | `?` | `[0x2b0d64]` | `[0x2b0d10]` |
| `bl` at 0x67780 | `(unknown callee) 0x688b8` | `0x10ec84` | `?` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x6778c | `(unknown callee) 0x697f4` | `[0x2b13d0]` | `?` | `?` | `0x2b0a80` |
| `bl` at 0x677a0 | `(unknown callee) 0x1b49c` | `sp-120` | `0x10ecf4` | `ret(0x697f4)` | `sp-120` |
| `bl` at 0x677b8 | `(unknown callee) 0x688b8` | `sp-120` | `?` | `0x2b0a80` | `sp-120` |
| `bl` at 0x677c4 | `(unknown callee) 0x688b8` | `0x2b0d68` | `?` | `[0x2b0d64]` | `[0x2b0d10]` |
| `bl` at 0x67814 | `(unknown callee) 0x688b8` | `0x10ed30` | `0x2b0a80` | `[0x2b0d64]-1` | `0x0` |
| `bl` at 0x678c0 | `(unknown callee) 0x688b8` | `0x10ed30` | `0x2b0a80` | `[0x2b0d64]-1` | `0x2b0d13` |
| `bl` at 0x6794c | `(unknown callee) 0x1b508` | `0x2b0d14` | `?` | `?` | `?` |
| `bl` at 0x67958 | `(unknown callee) 0x1b22c` | `0x2b0d14` | `?` | `?` | `?` |
| `bl` at 0x67974 | `(unknown callee) 0x688b8` | `0x2b0d14` | `?` | `ret(0x1b22c)` | `ret(0x1b22c)+0x2b0d14` |
| `bl` at 0x679f8 | `(unknown callee) 0x688b8` | `0x10ed34` | `?` | `[0x2b0d64]` | `0x0` |
| `bl` at 0x67a00 | `(unknown callee) 0x688b8` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x67a08 | `(unknown callee) 0x22dc0` | `0x2b0cac` | `?` | `?` | `?` |
| `bl` at 0x67a24 | `(unknown callee) 0x21674` | `ret(0x22dc0)` | `?` | `?` | `ret(0x22dc0)` |
| `bl` at 0x67a48 | `(unknown callee) 0x21e28` | `0x9` | `0x10e990` | `0x3a9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x67244 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0x6727c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x672fc | `[[sp-28]]:1 != 0x18` | `[[sp-28]]:1 == 0x18` |
| `bne` at 0x6730c | `[0x2b0d0c] != 0x0` | `[0x2b0d0c] == 0x0` |
| `beq` at 0x67328 | `ret(0x68680) == 0x1` | `ret(0x68680) != 0x1` |
| `beq` at 0x67344 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `beq` at 0x673d0 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `beq` at 0x67420 | `[0x2b0d10] == 0x1` | `[0x2b0d10] != 0x1` |
| `blo` at 0x6742c | `[0x2b0d10] < 0x1 (unsigned, 32-bit)` | `[0x2b0d10] >= 0x1 (unsigned, 32-bit)` |
| `bhi` at 0x67438 | `[0x2b0d10] > 0x3 (unsigned, 32-bit)` | `[0x2b0d10] <= 0x3 (unsigned, 32-bit)` |
| `bne` at 0x6744c | `[[sp-28]]:1 != 0x14` | `[[sp-28]]:1 == 0x14` |
| `bne` at 0x6749c | `[[sp-28]]:1 != 0x4` | `[[sp-28]]:1 == 0x4` |
| `beq` at 0x674ac | `[0x148564] == 0x43` | `[0x148564] != 0x43` |
| `beq` at 0x674bc | `[0x148564] == 0x33` | `[0x148564] != 0x33` |
| `beq` at 0x674cc | `[0x148564] == 0x35` | `[0x148564] != 0x35` |
| `beq` at 0x674dc | `[0x148564] == 0x34` | `[0x148564] != 0x34` |
| `beq` at 0x674ec | `[0x148564] == 0x41` | `[0x148564] != 0x41` |
| `beq` at 0x674fc | `[0x148564] == 0x36` | `[0x148564] != 0x36` |
| `beq` at 0x6750c | `[0x148564] == 0x39` | `[0x148564] != 0x39` |
| `beq` at 0x6751c | `[0x148564] == 0x37` | `[0x148564] != 0x37` |
| `beq` at 0x6752c | `[0x148564] == 0x38` | `[0x148564] != 0x38` |
| `beq` at 0x6753c | `[0x148564] == 0x30` | `[0x148564] != 0x30` |
| `beq` at 0x6754c | `[0x148564] == 0x51` | `[0x148564] != 0x51` |
| `beq` at 0x6755c | `[0x148564] == 0x40` | `[0x148564] != 0x40` |
| `beq` at 0x6756c | `[0x148564] == 0x4f` | `[0x148564] != 0x4f` |
| `beq` at 0x6757c | `[0x148564] == 0x42` | `[0x148564] != 0x42` |
| `beq` at 0x6758c | `[0x148564] == 0x46` | `[0x148564] != 0x46` |
| `beq` at 0x6759c | `[0x148564] == 0x48` | `[0x148564] != 0x48` |
| `beq` at 0x675ac | `[0x148564] == 0x4c` | `[0x148564] != 0x4c` |
| `beq` at 0x675bc | `[0x148564] == 0x4d` | `[0x148564] != 0x4d` |
| `beq` at 0x675cc | `[0x148564] == 0x7c` | `[0x148564] != 0x7c` |
| `beq` at 0x675e8 | `ret(0x3d8e0) == 0x1` | `ret(0x3d8e0) != 0x1` |
| `bne` at 0x67600 | `[0x2b0d10] != 0x3` | `[0x2b0d10] == 0x3` |
| `beq` at 0x6764c | `[[sp-28]]:1 == 0xd` | `[[sp-28]]:1 != 0xd` |
| `beq` at 0x6765c | `[[sp-28]]:1 == 0xa` | `[[sp-28]]:1 != 0xa` |
| `beq` at 0x67674 | `ret(0x1b22c) == 0x0` | `ret(0x1b22c) != 0x0` |
| `bgt` at 0x676fc | `[0x2b13cc] > 0x13 (signed, 32-bit)` | `[0x2b13cc] <= 0x13 (signed, 32-bit)` |
| `bne` at 0x67768 | `[0x2b0d10] != 0x2` | `[0x2b0d10] == 0x2` |
| `beq` at 0x67778 | `ret(0x3d8e0) == 0x1` | `ret(0x3d8e0) != 0x1` |
| `bne` at 0x677d8 | `[[sp-28]]:1 != 0x8` | `[[sp-28]]:1 == 0x8` |
| `bls` at 0x677ec | `[0x2b0d64] <= 0x2b0d14 (unsigned, 32-bit)` | `[0x2b0d64] > 0x2b0d14 (unsigned, 32-bit)` |
| `beq` at 0x67828 | `[[sp-28]]:1 == 0x10` | `[[sp-28]]:1 != 0x10` |
| `beq` at 0x67838 | `[[sp-28]]:1 == 0xe` | `[[sp-28]]:1 != 0xe` |
| `bne` at 0x6784c | `[[sp-28]]:1 != 0x10` | `[[sp-28]]:1 == 0x10` |
| `bne` at 0x67888 | `[0x148540] != [sp-128]` | `[0x148540] == [sp-128]` |
| `bhi` at 0x678b4 | `[0x2b0d64]-1 > 0x2b0d13 (unsigned, 32-bit)` | `[0x2b0d64]-1 <= 0x2b0d13 (unsigned, 32-bit)` |
| `bhs` at 0x6798c | `[0x2b0d64] >= 0x2b0d63 (unsigned, 32-bit)` | `[0x2b0d64] < 0x2b0d63 (unsigned, 32-bit)` |
| `beq` at 0x67a1c | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x67a58 | `r0` | `?` |

## `SMPConsoleSubchannelDefault`

- `SMPConsoleSubchannelDefault`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67ce0 to 0x67d30.
  Established. The global zero test takes its bypass and nonzero falls through (`beq` at 0x67d00).
  The call to `0x67d30` has entry r0, entry r1, entry r2, `[0x142868]` (`bl` at 0x67d08).
  Fall-through later writes `0x1` at absolute `0x142868` with a word store (`str` at 0x67d1c).
  Default-subchannel meaning and the common r0 are unknown (`ldmdb` at 0x67d28).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x67cf0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x67cf4 | load | 4 | `abs 0x67d2c` | `-` |
| `ldr` at 0x67cf8 | load | 4 | `abs 0x142868` | `-` |
| `ldr` at 0x67d04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67d10 | store | 4 | `sp -0x14` | `ret(0x67d30)` |
| `ldr` at 0x67d14 | load | 4 | `abs 0x67d2c` | `-` |
| `str` at 0x67d1c | store | 4 | `abs 0x142868` | `0x1` |
| `ldr` at 0x67d20 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x67d28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x67d28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x67d28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67d08 | `(unknown callee) 0x67d30` | `r0` | `r1` | `r2` | `[0x142868]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x67d00 | `[0x142868] == 0x0` | `[0x142868] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x67d28 | `r0` | `?` |

## `SMPConsoleSubchannel`

- `SMPConsoleSubchannel`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67d30 to 0x67ee8.
  Established. A word store clears absolute `0x142868` (`str` at 0x67d4c). Another word store writes
  `[sp-20]` at `ret(0x74908) +0x14` (`str` at 0x67d7c). The deciding equality test takes its bypass
  for `[sp-20] == [ret(0x74908)+20]` and falls through on inequality (`beq` at 0x67d70). A later
  word store carries `[sp-32]` at absolute `0x2b13d0` (`str` at 0x67e74). The exit reports `[sp-20]`
  (`ldmdb` at 0x67ed4). Subchannel encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x67d40 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x67d44 | load | 4 | `abs 0x67ed8` | `-` |
| `str` at 0x67d4c | store | 4 | `abs 0x142868` | `0x0` |
| `str` at 0x67d5c | store | 4 | `sp -0x18` | `ret(0x74908)` |
| `ldr` at 0x67d60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67d64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x67d68 | load | 4 | `ret(0x74908) +0x14` | `-` |
| `ldr` at 0x67d74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67d78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67d7c | store | 4 | `ret(0x74908) +0x14` | `[sp-20]` |
| `ldr` at 0x67d90 | load | 4 | `abs 0x67edc` | `-` |
| `ldr` at 0x67d94 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x67da4 | load | 4 | `abs 0x67edc` | `-` |
| `ldr` at 0x67da8 | load | 4 | `abs 0x2b13d0` | `-` |
| `str` at 0x67dc4 | store | 4 | `sp -0x1c` | `ret(0x74708)` |
| `ldr` at 0x67dc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67dd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67dd8 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x67de0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67de4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x67de8 | load | 4 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x67dec | load | 4 | `ret(0x74708) +0x3c` | `-` |
| `ldr` at 0x67df8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67dfc | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x67e04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67e08 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x67e18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x67e1c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x67e38 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x67e3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67e44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67e54 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x67e58 | load | 4 | `abs 0x67edc` | `-` |
| `ldr` at 0x67e5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67e60 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x67e6c | load | 4 | `abs 0x67edc` | `-` |
| `ldr` at 0x67e70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x67e74 | store | 4 | `abs 0x2b13d0` | `[sp-32]` |
| `ldr` at 0x67e78 | load | 4 | `abs 0x67ee0` | `-` |
| `ldr` at 0x67e7c | load | 4 | `abs 0x148518` | `-` |
| `ldr` at 0x67e88 | load | 4 | `abs 0x67edc` | `-` |
| `ldr` at 0x67e8c | load | 4 | `abs 0x2b0d10` | `-` |
| `ldr` at 0x67e98 | load | 4 | `abs 0x67edc` | `-` |
| `ldr` at 0x67e9c | load | 4 | `abs 0x2b0d10` | `-` |
| `ldr` at 0x67eac | load | 4 | `abs 0x67edc` | `-` |
| `ldr` at 0x67eb0 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x67ebc | load | 4 | `abs 0x67ee4` | `-` |
| `ldr` at 0x67ec8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x67ecc | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x67ed0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x67ed4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x67ed4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x67ed4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67d54 | `(unknown callee) 0x74908` | `0x0` | `r1` | `0x142868` | `0x0` |
| `bl` at 0x67d88 | `(unknown callee) 0x5e8d4` | `0x1` | `[sp-20]` | `[sp-20]` | `ret(0x74908)` |
| `bl` at 0x67d8c | `(unknown callee) 0x74f24` | `ret(0x5e8d4)` | `?` | `?` | `?` |
| `bl` at 0x67dbc | `(unknown callee) 0x74708` | `?` | `?` | `?` | `?` |
| `bl` at 0x67eb4 | `(unknown callee) 0x697f4` | `[0x2b13d0]` | `[[sp-24]+60]` | `0x2b0a80` | `0x2b0a80` |
| `bl` at 0x67ec4 | `(unknown callee) 0x23f2c` | `0x10ed78` | `ret(0x697f4)` | `?` | `ret(0x697f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x67d70 | `[sp-20] == [ret(0x74908)+20]` | `[sp-20] != [ret(0x74908)+20]` |
| `bne` at 0x67dd0 | `ret(0x74708) != 0x0` | `ret(0x74708) == 0x0` |
| `beq` at 0x67df4 | `[[sp-24]+60] == [ret(0x74708)+60]` | `[[sp-24]+60] != [ret(0x74708)+60]` |
| `beq` at 0x67e68 | `[sp-32] == [0x2b13d0]` | `[sp-32] != [0x2b13d0]` |
| `beq` at 0x67e84 | `[0x148518] == 0x0` | `[0x148518] != 0x0` |
| `beq` at 0x67e94 | `[0x2b0d10] == 0x1` | `[0x2b0d10] != 0x1` |
| `beq` at 0x67ea4 | `[0x2b0d10] == 0x2` | `[0x2b0d10] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x67ed4 | `r0` | `[sp-20]` |

## `SMPSystemConsole`

- `SMPSystemConsole`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x67ee8 to 0x68018.
  Established. The entry-r0 test takes its bypass on zero and falls through on nonzero (`beq` at
  0x67f0c). The next guard takes its continuation for `r1 != 0x0` and falls through on zero (`bne`
  at 0x67f18). A word store writes `[sp-32]` at absolute `0x2b13d0` (`str` at 0x67fa4).
  Console-address encoding and the joined return value remain unidentified (`ldmdb` at 0x68008).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x67ef8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x67efc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x67f00 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x67f04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x67f10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x67f20 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x67f34 | store | 4 | `sp -0x24` | `ret(0x74908)` |
| `ldr` at 0x67f38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x67f48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x67f64 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x67f68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67f70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x67f80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x67f88 | load | 4 | `abs 0x6800c` | `-` |
| `ldr` at 0x67f8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x67f90 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x67f9c | load | 4 | `abs 0x6800c` | `-` |
| `ldr` at 0x67fa0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x67fa4 | store | 4 | `abs 0x2b13d0` | `[sp-32]` |
| `ldr` at 0x67fa8 | load | 4 | `abs 0x68010` | `-` |
| `ldr` at 0x67fac | load | 4 | `abs 0x148518` | `-` |
| `ldr` at 0x67fb8 | load | 4 | `abs 0x6800c` | `-` |
| `ldr` at 0x67fbc | load | 4 | `abs 0x2b0d10` | `-` |
| `ldr` at 0x67fc8 | load | 4 | `abs 0x6800c` | `-` |
| `ldr` at 0x67fcc | load | 4 | `abs 0x2b0d10` | `-` |
| `ldr` at 0x67fdc | load | 4 | `abs 0x6800c` | `-` |
| `ldr` at 0x67fe0 | load | 4 | `abs 0x2b13d0` | `-` |
| `ldr` at 0x67fec | load | 4 | `abs 0x68014` | `-` |
| `ldr` at 0x67ff8 | load | 4 | `abs 0x6800c` | `-` |
| `ldr` at 0x67ffc | load | 4 | `abs 0x2b13d0` | `-` |
| `str` at 0x68000 | store | 4 | `sp -0x28` | `[0x2b13d0]` |
| `ldr` at 0x68004 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x68008 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x68008 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x68008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x67f2c | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r1` |
| `bl` at 0x67f84 | `(unknown callee) 0x74f04` | `ret(0x74908)` | `?` | `?` | `?` |
| `bl` at 0x67fe4 | `(unknown callee) 0x697f4` | `[0x2b13d0]` | `?` | `0x2b0a80` | `0x2b0a80` |
| `bl` at 0x67ff4 | `(unknown callee) 0x23f2c` | `0x10eda0` | `ret(0x697f4)` | `?` | `ret(0x697f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x67f0c | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x67f18 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x67f98 | `[sp-32] == [0x2b13d0]` | `[sp-32] != [0x2b13d0]` |
| `beq` at 0x67fb4 | `[0x148518] == 0x0` | `[0x148518] != 0x0` |
| `beq` at 0x67fc4 | `[0x2b0d10] == 0x1` | `[0x2b0d10] != 0x1` |
| `beq` at 0x67fd4 | `[0x2b0d10] == 0x2` | `[0x2b0d10] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x68008 | `r0` | `?` |

## `UtilPortStr`

- `UtilPortStr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69e4c to 0x6a08c.
  Established. The first equality branch takes its continuation for `r0 == 0x40` and falls through
  otherwise (`beq` at 0x69e6c). Two computed selectors remain unresolved (`ldrls` at 0x69e80,
  `ldrls` at 0x69f60). The first selector takes an unresolved edge for unsigned `r1-1 <= 0xb` and
  falls through for greater (`ldrls` at 0x69e80). The known-edge exit reports `0x10f04c` (`ldmdb` at
  0x6a034). Port-name mappings inside excluded arms remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69e5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69e60 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x69e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69e74 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x69e80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69f48 | load | 4 | `abs 0x6a068` | `-` |
| `str` at 0x69f4c | store | 4 | `sp -0x1c` | `0x10f04c` |
| `ldr` at 0x69f54 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x69f60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6a028 | load | 4 | `abs 0x6a068` | `-` |
| `str` at 0x6a02c | store | 4 | `sp -0x1c` | `0x10f04c` |
| `ldr` at 0x6a030 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6a034 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6a034 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6a034 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x69e6c | `r0 == 0x40` | `r0 != 0x40` |
| `ldrls` at 0x69e80 | `r1-1 <= 0xb (unsigned, 32-bit)` | `r1-1 > 0xb (unsigned, 32-bit)` |
| `ldrls` at 0x69f60 | `r1-1 <= 0xb (unsigned, 32-bit)` | `r1-1 > 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6a034 | `r0` | `0x10f04c` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x69e80).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x69f60).

## `L1CfgPortEnable`

- `L1CfgPortEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e9f0 to 0x6eac8.
  Established. Two word stores at unknown targets write `0x1` and `0x0` (`str` at 0x6ea60, `str` at
  0x6ea90). A later word store advances absolute `0x14865c` to `[0x14865c]+1` (`str` at 0x6eaa4).
  The call to `0x6c0d0` has `0x10f618` and entry r1, r2, r3 (`bl` at 0x6ea08). The exit carries
  `ret(0x6c168)` (`ldmdb` at 0x6eaac). Port-configuration ownership remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ea00 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6ea04 | load | 4 | `abs 0x6eab0` | `-` |
| `ldr` at 0x6ea0c | load | 4 | `abs 0x6eab4` | `-` |
| `ldr` at 0x6ea10 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6ea20 | load | 4 | `abs 0x6eab8` | `-` |
| `ldr` at 0x6ea24 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6ea28 | load | 4 | `abs 0x6eabc` | `-` |
| `ldr` at 0x6ea2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ea34 | load | 4 | `abs 0x6eac0` | `-` |
| `ldr` at 0x6ea38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ea54 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6ea60 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x6ea64 | load | 4 | `abs 0x6eac0` | `-` |
| `ldr` at 0x6ea68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ea84 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6ea90 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6ea94 | load | 4 | `abs 0x6eac4` | `-` |
| `ldr` at 0x6ea98 | load | 4 | `abs 0x6eac4` | `-` |
| `ldr` at 0x6ea9c | load | 4 | `abs 0x14865c` | `-` |
| `str` at 0x6eaa4 | store | 4 | `abs 0x14865c` | `[0x14865c]+1` |
| `ldmdb` at 0x6eaac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6eaac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6eaac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ea08 | `(unknown callee) 0x6c0d0` | `0x10f618` | `r1` | `r2` | `r3` |
| `bl` at 0x6ea30 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f628` | `[sp-20]` | `0x144dc8` |
| `bl` at 0x6eaa8 | `(unknown callee) 0x6c168` | `?` | `0x14892c` | `0x14865c` | `[0x14865c]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6ea1c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6eaac | `r0` | `ret(0x6c168)` |

## `L1CfgPortConfigure`

- `L1CfgPortConfigure`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6eac8 to 0x6f364.
  Established. A word store at an unknown effective address writes `0x1` (`str` at 0x6ec34). Two
  later word stores advance absolute `0x148660` and `0x148664` to their loaded expressions plus one
  (`str` at 0x6ec48, `str` at 0x6ec6c). The first call to `0xb7944` has entry r0, `0x0`, entry r2,
  unknown (`bl` at 0x6eb0c). The first branch has unknown operand, takes its continuation on nonzero
  and falls through on zero (`bne` at 0x6eaf0). Configuration-field meanings and final r0 remain
  unresolved (`ldmdb` at 0x6f32c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ead8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x6eadc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x6eae0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6eae4 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x6eaf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6eaf8 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x6eafc | store | 4 | `sp -0xa8` | `[r0+40]` |
| `ldr` at 0x6eb04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6eb10 | store | 4 | `sp -0xa8` | `ret(0xb7944)` |
| `ldr` at 0x6eb14 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x6eb18 | store | 4 | `sp -0x2c` | `[sp-168]` |
| `ldr` at 0x6eb1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6eb20 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x6eb30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6eb34 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x6eb38 | store | 4 | `sp -0xac` | `[[sp-24]+48]` |
| `ldr` at 0x6eb40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6eb4c | store | 4 | `sp -0xac` | `ret(0xb7944)` |
| `ldr` at 0x6eb50 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x6eb54 | store | 4 | `sp -0xa0` | `[sp-172]` |
| `ldr` at 0x6eb58 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6eb64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6eb68 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x6eb6c | store | 4 | `sp -0xb0` | `[[sp-24]+32]:1` |
| `ldr` at 0x6eb70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6eb7c | load | 4 | `abs 0x6f330` | `-` |
| `str` at 0x6eb80 | store | 4 | `sp -0xb4` | `0x10f640` |
| `ldr` at 0x6eb88 | load | 4 | `abs 0x6f334` | `-` |
| `str` at 0x6eb8c | store | 4 | `sp -0xb4` | `0x10f648` |
| `ldr` at 0x6eb90 | load | 4 | `abs 0x6f338` | `-` |
| `ldr` at 0x6eb94 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6eb98 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6ebac | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6ebb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6ebbc | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x6ebc0 | store | 4 | `sp -0x24` | `[[sp-24]+12]` |
| `str` at 0x6ebc8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6ebcc | load | 4 | `abs 0x6f33c` | `-` |
| `ldr` at 0x6ebd4 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ebd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ebf4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ebfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ec08 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ec0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ec28 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6ec34 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x6ec38 | load | 4 | `abs 0x6f344` | `-` |
| `ldr` at 0x6ec3c | load | 4 | `abs 0x6f344` | `-` |
| `ldr` at 0x6ec40 | load | 4 | `abs 0x148660` | `-` |
| `str` at 0x6ec48 | store | 4 | `abs 0x148660` | `[0x148660]+1` |
| `ldr` at 0x6ec4c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6ec5c | load | 4 | `abs 0x6f348` | `-` |
| `ldr` at 0x6ec60 | load | 4 | `abs 0x6f348` | `-` |
| `ldr` at 0x6ec64 | load | 4 | `abs 0x148664` | `-` |
| `str` at 0x6ec6c | store | 4 | `abs 0x148664` | `[0x148664]+1` |
| `ldr` at 0x6ec70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6ec74 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0x6ec80 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ec84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6eca0 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6ecac | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x6ecb0 | load | 4 | `abs 0x6f34c` | `-` |
| `ldr` at 0x6ecb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6ecb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6ecbc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6ecc4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x6ecc8 | load | 2 | `[sp-24] +0x22` | `-` |
| `ldr` at 0x6ecd4 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ecd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ecf4 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6ed00 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x6ed04 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x6ed08 | load | 2 | `[sp-24] +0x22` | `-` |
| `str` at 0x6ed0c | store | 4 | `sp -0x28` | `[[sp-24]+34]:2` |
| `ldr` at 0x6ed10 | load | 4 | `abs 0x6f350` | `-` |
| `ldr` at 0x6ed14 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6ed24 | load | 4 | `abs 0x6f354` | `-` |
| `str` at 0x6ed28 | store | 4 | `sp -0xb8` | `0x144dc8` |
| `ldr` at 0x6ed2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6ed38 | load | 4 | `abs 0x6f358` | `-` |
| `str` at 0x6ed3c | store | 4 | `sp -0xbc` | `0x10f684` |
| `ldr` at 0x6ed44 | load | 4 | `abs 0x6f35c` | `-` |
| `str` at 0x6ed48 | store | 4 | `sp -0xbc` | `0x10f68c` |
| `ldr` at 0x6ed4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6ed54 | store | 4 | `sp -0xc0` | `[sp-36]+64` |
| `ldr` at 0x6ed58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6ed64 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6ed6c | store | 4 | `sp -0xc4` | `[sp-40]+64` |
| `str` at 0x6ed78 | store | 4 | `sp -0xc4` | `0x3f` |
| `ldr` at 0x6ed7c | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x6ed80 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x6ed84 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x6ed88 | store | 4 | `sp -0xd8` | `[sp-44]` |
| `ldr` at 0x6ed8c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x6ed90 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6ed94 | load | 4 | `abs 0x6f360` | `-` |
| `ldr` at 0x6ed98 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x6ed9c | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x6eda4 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6eda8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6edc4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ede4 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ede8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ee04 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ee0c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x6ee10 | store | 4 | `(unknown)` | `[sp-148]` |
| `ldr` at 0x6ee14 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ee18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ee34 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ee3c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6ee40 | store | 4 | `(unknown)` | `[sp-144]` |
| `ldr` at 0x6ee44 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ee48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ee64 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ee6c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6ee70 | store | 4 | `(unknown)` | `[sp-140]` |
| `ldr` at 0x6ee74 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ee78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ee94 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ee9c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6eea4 | store | 4 | `(unknown)` | `ret(0x74b98)` |
| `ldr` at 0x6eea8 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6eeac | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6eeb8 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6eebc | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6eec8 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6eecc | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6eedc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6eee8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6eef4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x6ef04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6ef10 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ef14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ef30 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ef38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6ef40 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x6ef4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x6ef50 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ef54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6ef70 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ef78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6ef7c | store | 4 | `(unknown)` | `[sp-40]` |
| `ldr` at 0x6ef80 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6ef84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6efa4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldrb` at 0x6efb0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6efbc | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6efc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6efdc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6efe8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6f000 | store | 4 | `sp -0xa4` | `ret(0xae97c)` |
| `ldr` at 0x6f008 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f00c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f028 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f034 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6f04c | store | 4 | `sp -0xa4` | `ret(0xae97c)` |
| `ldr` at 0x6f050 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f054 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f070 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f078 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f084 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f088 | load | 4 | `abs 0x14899c` | `-` |
| `ldr` at 0x6f098 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f09c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f0b8 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6f0c4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6f0c8 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f0cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f0e8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f0f4 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f0f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f114 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f120 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f124 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f140 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f14c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6f150 | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x6f154 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6f158 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `ldr` at 0x6f15c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x6f160 | store | 4 | `sp -0xd4` | `[sp-68]` |
| `str` at 0x6f168 | store | 4 | `sp -0xd0` | `0x0` |
| `str` at 0x6f170 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x6f178 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6f184 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6f188 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6f19c | store | 4 | `sp -0x20` | `0x2` |
| `ldr` at 0x6f1a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6f1a4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x6f1a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f1b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f1bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x6f1d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6f1e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f1e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f1f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6f1fc | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x6f204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f208 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6f210 | store | 4 | `sp -0xc8` | `ret(0xb7944)` |
| `ldr` at 0x6f21c | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x6f228 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6f238 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f23c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f258 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f264 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f268 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f284 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f290 | load | 4 | `abs 0x6f340` | `-` |
| `ldr` at 0x6f294 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f2b0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f2bc | load | 4 | `sp -0x90` | `-` |
| `str` at 0x6f2c0 | store | 4 | `sp -0xdc` | `[sp-144]` |
| `ldr` at 0x6f2c4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6f2c8 | store | 4 | `sp -0xd8` | `[sp-140]` |
| `str` at 0x6f2d0 | store | 4 | `sp -0xd4` | `0x0` |
| `str` at 0x6f2d8 | store | 4 | `sp -0xd0` | `0x1` |
| `str` at 0x6f2e0 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x6f2e8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6f2f4 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x6f2f8 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x6f308 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6f310 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x6f324 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x6f32c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6f32c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6f32c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6f32c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6eb0c | `(unknown callee) 0xb7944` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x6eb48 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x6eb9c | `(unknown callee) 0x23f2c` | `0x10f654` | `[[sp-24]+32]:1` | `[sp-180]` | `?` |
| `bl` at 0x6ebb4 | `(unknown callee) 0x71cbc` | `sp-156` | `[sp-172]` | `0x70` | `sp-156` |
| `bl` at 0x6ebd0 | `(unknown callee) 0x6c0d0` | `0x10f670` | `?` | `?` | `0x0` |
| `bl` at 0x6eda0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f694` | `[sp-188]` | `[sp-36]+64` |
| `bl` at 0x6ede0 | `(unknown callee) 0x1b304` | `?` | `sp-156` | `0x70` | `?` |
| `bl` at 0x6eea0 | `(unknown callee) 0x74b98` | `[sp-140]` | `0x14892c` | `?` | `[0x1489b4]` |
| `bl` at 0x6eff8 | `(unknown callee) 0xae97c` | `?` | `[sp-36]+16384` | `?` | `[sp-36]+16384` |
| `bl` at 0x6f044 | `(unknown callee) 0xae97c` | `?` | `[sp-36]+8192` | `?` | `[sp-36]+8192` |
| `bl` at 0x6f17c | `(unknown callee) 0x70e7c` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x6f194 | `(unknown callee) 0x711f8` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x6f20c | `(unknown callee) 0xb7944` | `[sp-24]` | `?` | `0x24` | `?` |
| `bl` at 0x6f224 | `(unknown callee) 0x71cbc` | `sp-156` | `?` | `0x70` | `sp-156` |
| `bl` at 0x6f2ec | `(unknown callee) 0x70e7c` | `?` | `?` | `?` | `[sp-148]` |
| `bl` at 0x6f304 | `(unknown callee) 0x711f8` | `[sp-148]` | `[sp-144]` | `sp-156` | `0x0` |
| `bl` at 0x6f318 | `(unknown callee) 0x6d51c` | `?` | `?` | `[[sp-24]+32]:1` | `?` |
| `bl` at 0x6f320 | `(unknown callee) 0x6c168` | `ret(0xae97c)` | `0x14892c` | `?` | `[0x14899c]` |
| `bl` at 0x6f328 | `(unknown callee) 0x6c734` | `[sp-36]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6eaf0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6eb2c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6eb60 | `[sp-172] != 0x0` | `[sp-172] == 0x0` |
| `beq` at 0x6eb78 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x6ec04 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6ec54 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `bne` at 0x6ec7c | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |
| `beq` at 0x6ecd0 | `[[sp-24]+34]:2 == 0x0` | `[[sp-24]+34]:2 != 0x0` |
| `beq` at 0x6ed20 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6ed34 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x6ed60 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x6eeb4 | `[0x14893c] == 0x52` | `[0x14893c] != 0x52` |
| `beq` at 0x6eec4 | `[0x14893c] == 0x44` | `[0x14893c] != 0x44` |
| `beq` at 0x6eed4 | `[0x14893c] == 0x40` | `[0x14893c] != 0x40` |
| `beq` at 0x6eee4 | `[sp-140] == 0x52` | `[sp-140] != 0x52` |
| `beq` at 0x6eef0 | `[sp-140] == 0x44` | `[sp-140] != 0x44` |
| `beq` at 0x6eefc | `[sp-140] == 0x40` | `[sp-140] != 0x40` |
| `beq` at 0x6ef0c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x6efb8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6f080 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6f090 | `[0x14899c] != 0x0` | `[0x14899c] == 0x0` |
| `blt` at 0x6f1b0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6f1dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6f230 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6f32c | `r0` | `?` |

## `L1CfgPortDisable`

- `L1CfgPortDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f364 to 0x6f784.
  Established. Word stores write `[0x148660]-1`, `[0x148664]-1` and `[0x14865c]-1` at those
  respective absolute addresses (`str` at 0x6f41c, `str` at 0x6f468, `str` at 0x6f47c). The opening
  call to `0x6c0d0` carries `0x10f6c0` and entry r1 through r3 (`bl` at 0x6f37c). The first zero
  test has unknown operand, takes its bypass on zero and falls through on nonzero (`beq` at
  0x6f390). Port-disable propagation and exit r0 remain unknown (`ldmdb` at 0x6f754).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f374 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6f378 | load | 4 | `abs 0x6f758` | `-` |
| `ldr` at 0x6f380 | load | 4 | `abs 0x6f75c` | `-` |
| `ldr` at 0x6f384 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6f394 | load | 4 | `abs 0x6f760` | `-` |
| `ldr` at 0x6f398 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6f39c | load | 4 | `abs 0x6f764` | `-` |
| `ldr` at 0x6f3a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f3a8 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f3ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f3c8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f3d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6f3d4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x6f3d8 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f3dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f3f8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f400 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f40c | load | 4 | `abs 0x6f76c` | `-` |
| `ldr` at 0x6f410 | load | 4 | `abs 0x6f76c` | `-` |
| `ldr` at 0x6f414 | load | 4 | `abs 0x148660` | `-` |
| `str` at 0x6f41c | store | 4 | `abs 0x148660` | `[0x148660]-1` |
| `ldr` at 0x6f420 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f440 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f448 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f458 | load | 4 | `abs 0x6f770` | `-` |
| `ldr` at 0x6f45c | load | 4 | `abs 0x6f770` | `-` |
| `ldr` at 0x6f460 | load | 4 | `abs 0x148664` | `-` |
| `str` at 0x6f468 | store | 4 | `abs 0x148664` | `[0x148664]-1` |
| `ldr` at 0x6f46c | load | 4 | `abs 0x6f774` | `-` |
| `ldr` at 0x6f470 | load | 4 | `abs 0x6f774` | `-` |
| `ldr` at 0x6f474 | load | 4 | `abs 0x14865c` | `-` |
| `str` at 0x6f47c | store | 4 | `abs 0x14865c` | `[0x14865c]-1` |
| `ldr` at 0x6f480 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f4a0 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6f4ac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6f4b0 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f4b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f4d0 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6f4dc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6f4e0 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f4e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f500 | load | 4 | `abs 0x1489b4` | `-` |
| `str` at 0x6f50c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x6f510 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f514 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f530 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6f54c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f564 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f568 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f574 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f578 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f584 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f588 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f594 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f598 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f5a4 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f5a8 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f5b4 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f5b8 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f5c4 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f5c8 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f5d4 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f5d8 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f5e4 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f5e8 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f5f4 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f5f8 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f604 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f608 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f614 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f618 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f624 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f628 | load | 4 | `abs 0x14893c` | `-` |
| `ldr` at 0x6f634 | load | 4 | `abs 0x6f768` | `-` |
| `ldr` at 0x6f638 | load | 4 | `abs 0x14893c` | `-` |
| `str` at 0x6f694 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x6f698 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x6f69c | store | 4 | `sp -0x48` | `[sp-64]` |
| `str` at 0x6f6a4 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0x6f6a8 | load | 4 | `abs 0x6f778` | `-` |
| `str` at 0x6f6c0 | store | 4 | `sp -0x1c` | `ret(0xb6f64)` |
| `ldr` at 0x6f6c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f6d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f6d8 | load | 4 | `abs 0x6f77c` | `-` |
| `ldr` at 0x6f6dc | load | 4 | `abs 0x148dd4` | `-` |
| `str` at 0x6f6e0 | store | 4 | `ret(0xb6f64) +0x18` | `[0x148dd4]` |
| `ldr` at 0x6f6e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f704 | load | 4 | `abs 0x6f780` | `-` |
| `ldr` at 0x6f708 | load | 4 | `abs 0x2b16d8` | `-` |
| `ldr` at 0x6f714 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f71c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f73c | load | 4 | `abs 0x6f780` | `-` |
| `ldr` at 0x6f740 | load | 4 | `abs 0x2b16d4` | `-` |
| `ldr` at 0x6f74c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x6f754 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6f754 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6f754 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6f37c | `(unknown callee) 0x6c0d0` | `0x10f6c0` | `r1` | `r2` | `r3` |
| `bl` at 0x6f3a4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f6d4` | `[sp-20]` | `0x144dc8` |
| `bl` at 0x6f548 | `(unknown callee) 0x1b43c` | `?` | `0x0` | `0x70` | `?` |
| `bl` at 0x6f558 | `(unknown callee) 0x6d51c` | `ret(0x1b43c)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x6f560 | `(unknown callee) 0x6c168` | `ret(0x1b43c)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x6f65c | `(unknown callee) 0x6907c` | `sp-64` | `sp-60` | `0x1` | `sp-64` |
| `bl` at 0x6f674 | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6f68c | `(unknown callee) 0x690d8` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x6f6b8 | `(unknown callee) 0xb6f64` | `0x148924` | `0x0` | `0x64` | `0x2` |
| `bl` at 0x6f6f4 | `(unknown callee) 0x79dcc` | `0x2` | `?` | `ret(0xb6f64)` | `[sp-20]` |
| `bl` at 0x6f718 | `(unknown callee) 0xb686c` | `[0x2b16d8]+20` | `[sp-28]` | `?` | `[0x2b16d8]+20` |
| `bl` at 0x6f72c | `(unknown callee) 0x79dcc` | `0x1` | `?` | `?` | `[sp-20]` |
| `bl` at 0x6f750 | `(unknown callee) 0xb686c` | `[0x2b16d4]+20` | `[sp-28]` | `?` | `[0x2b16d4]+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6f390 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6f408 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6f450 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x6f554 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x6f570 | `[0x14893c] == 0x35` | `[0x14893c] != 0x35` |
| `beq` at 0x6f580 | `[0x14893c] == 0x4f` | `[0x14893c] != 0x4f` |
| `beq` at 0x6f590 | `[0x14893c] == 0x42` | `[0x14893c] != 0x42` |
| `beq` at 0x6f5a0 | `[0x14893c] == 0x46` | `[0x14893c] != 0x46` |
| `beq` at 0x6f5b0 | `[0x14893c] == 0x48` | `[0x14893c] != 0x48` |
| `beq` at 0x6f5c0 | `[0x14893c] == 0x4a` | `[0x14893c] != 0x4a` |
| `beq` at 0x6f5d0 | `[0x14893c] == 0x4b` | `[0x14893c] != 0x4b` |
| `beq` at 0x6f5e0 | `[0x14893c] == 0x21` | `[0x14893c] != 0x21` |
| `beq` at 0x6f5f0 | `[0x14893c] == 0x5e` | `[0x14893c] != 0x5e` |
| `beq` at 0x6f600 | `[0x14893c] == 0x2b` | `[0x14893c] != 0x2b` |
| `beq` at 0x6f610 | `[0x14893c] == 0x4c` | `[0x14893c] != 0x4c` |
| `beq` at 0x6f620 | `[0x14893c] == 0x5a` | `[0x14893c] != 0x5a` |
| `beq` at 0x6f630 | `[0x14893c] == 0x7c` | `[0x14893c] != 0x7c` |
| `beq` at 0x6f640 | `[0x14893c] == 0x4d` | `[0x14893c] != 0x4d` |
| `bne` at 0x6f6cc | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |
| `bne` at 0x6f6ec | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0x6f700 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |
| `bne` at 0x6f724 | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `beq` at 0x6f738 | `ret(0x79dcc) == 0x0` | `ret(0x79dcc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6f754 | `r0` | `?` |

## `L1CfgGetPort`

- `L1CfgGetPort`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74b30 to 0x74b78.
  Established. A word load reads absolute `0x1489b4` (`ldr` at 0x74b64). The
  resulting r0 at exit remains unresolved (`ldmdb` at 0x74b70). There are no calls or conditional
  branches in the facts. Port-record geometry is unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74b40 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x74b44 | load | 4 | `abs 0x74b74` | `-` |
| `ldr` at 0x74b48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74b64 | load | 4 | `abs 0x1489b4` | `-` |
| `ldmdb` at 0x74b70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74b70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74b70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74b70 | `r0` | `?` |

## `L1CfgEventPartition`

- `L1CfgEventPartition`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75014 to 0x751c4.
  Established. A word store writes `[sp-20]` at `ret(0x74908) +0x3c` (`str` at 0x75054); its
  prior comparison takes the bypass for `[sp-20] == [ret(0x74908)+60]` and falls through on
  inequality (`beq` at 0x75048). The opening call to `0x74908` has `0x0` and entry r1 through r3
  (`bl` at 0x7502c). Partition-event encoding and the common r0 remain unknown (`ldmdb` at 0x751c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75024 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75034 | store | 4 | `sp -0x18` | `ret(0x74908)` |
| `ldr` at 0x75038 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7503c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75040 | load | 4 | `ret(0x74908) +0x3c` | `-` |
| `ldr` at 0x7504c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75050 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75054 | store | 4 | `ret(0x74908) +0x3c` | `[sp-20]` |
| `ldr` at 0x7506c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75070 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7507c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75080 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7508c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75090 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7509c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x750a0 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x750ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x750b0 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x750bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x750c0 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x750cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x750d0 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x750dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x750e0 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x750ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x750f0 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x750fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75100 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7510c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75110 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7511c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75120 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7512c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75130 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7513c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75140 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7514c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75150 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7515c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75160 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7516c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75170 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7517c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75180 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x7518c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75190 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldmdb` at 0x751c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x751c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x751c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7502c | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x75058 | `(unknown callee) 0x69af8` | `ret(0x74908)` | `?` | `ret(0x74908)` | `[sp-20]` |
| `bl` at 0x75068 | `(unknown callee) 0x74c78` | `0x6` | `0x0` | `0x0` | `?` |
| `bl` at 0x751a4 | `(unknown callee) 0x71ea8` | `0x1` | `?` | `?` | `[[sp-24]+16]` |
| `bl` at 0x751ac | `(unknown callee) 0x71ea8` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x751b4 | `(unknown callee) 0x71ea8` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x751bc | `(unknown callee) 0x71ea8` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75048 | `[sp-20] == [ret(0x74908)+60]` | `[sp-20] != [ret(0x74908)+60]` |
| `beq` at 0x75078 | `[[sp-24]+16] == 0x43` | `[[sp-24]+16] != 0x43` |
| `beq` at 0x75088 | `[[sp-24]+16] == 0x33` | `[[sp-24]+16] != 0x33` |
| `beq` at 0x75098 | `[[sp-24]+16] == 0x35` | `[[sp-24]+16] != 0x35` |
| `beq` at 0x750a8 | `[[sp-24]+16] == 0x34` | `[[sp-24]+16] != 0x34` |
| `beq` at 0x750b8 | `[[sp-24]+16] == 0x41` | `[[sp-24]+16] != 0x41` |
| `beq` at 0x750c8 | `[[sp-24]+16] == 0x36` | `[[sp-24]+16] != 0x36` |
| `beq` at 0x750d8 | `[[sp-24]+16] == 0x39` | `[[sp-24]+16] != 0x39` |
| `beq` at 0x750e8 | `[[sp-24]+16] == 0x37` | `[[sp-24]+16] != 0x37` |
| `beq` at 0x750f8 | `[[sp-24]+16] == 0x38` | `[[sp-24]+16] != 0x38` |
| `beq` at 0x75108 | `[[sp-24]+16] == 0x30` | `[[sp-24]+16] != 0x30` |
| `beq` at 0x75118 | `[[sp-24]+16] == 0x51` | `[[sp-24]+16] != 0x51` |
| `beq` at 0x75128 | `[[sp-24]+16] == 0x40` | `[[sp-24]+16] != 0x40` |
| `beq` at 0x75138 | `[[sp-24]+16] == 0x4f` | `[[sp-24]+16] != 0x4f` |
| `beq` at 0x75148 | `[[sp-24]+16] == 0x42` | `[[sp-24]+16] != 0x42` |
| `beq` at 0x75158 | `[[sp-24]+16] == 0x46` | `[[sp-24]+16] != 0x46` |
| `beq` at 0x75168 | `[[sp-24]+16] == 0x48` | `[[sp-24]+16] != 0x48` |
| `beq` at 0x75178 | `[[sp-24]+16] == 0x4c` | `[[sp-24]+16] != 0x4c` |
| `beq` at 0x75188 | `[[sp-24]+16] == 0x4d` | `[[sp-24]+16] != 0x4d` |
| `beq` at 0x75198 | `[[sp-24]+16] == 0x7c` | `[[sp-24]+16] != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x751c0 | `r0` | `?` |

## `SysComEventEnabled`

- `SysComEventEnabled`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x859cc to 0x85a0c.
  Established. The deciding branch retains only `Z == 0` for taken and its negation for fall-through
  (`bne` at 0x859e8). No calls occur in the facts. The exit r0 expression is `[sp-20]` (`ldmdb` at
  0x85a04). System-event enablement semantics remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x859dc | load | 4 | `abs 0x85a08` | `-` |
| `ldr` at 0x859e0 | load | 4 | `abs 0x2b172c` | `-` |
| `str` at 0x859f0 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x859fc | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x85a00 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x85a04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x85a04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x85a04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x859e8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x85a04 | `r0` | `[sp-20]` |
