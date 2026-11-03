# ARM router and scan dispatch in 1.38.4

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
Computed transfers in the two dispatchers remain unresolved. Tables and return values cover only known edges, and the excluded transfer sites are listed with each entry. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/router-scan-dispatch-facts.txt) retain all tool sections.

## `L1ReqIRouter`

- `L1ReqIRouter`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x882a0 to 0x8d224.
  Established. The opening call to `0x4f090` carries entry r0, r1, r2 and `0x0` (`bl` at 0x882c8).
  The next call uses `0x2b17c0` and three unknown candidates at `0x22a38` (`bl` at 0x882e0); its
  zero result takes the branch, while nonzero falls through (`beq` at 0x882f4). Two word stores
  initially clear `sp -0x29c` and `sp -0x2a0` (`str` at 0x882bc, `str` at 0x882c4). The signed
  comparison takes its branch for `[[sp-24]+28] < 0x0` and falls through for greater or equal (`blt`
  at 0x88398). The recognized exit carries `ret(0xb56f4)` on known edges (`ldmdb` at 0x8d1e0).
  Request-field interpretation, call arity and effects beyond the unresolved selectors remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x882b4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x882bc | store | 4 | `sp -0x29c` | `0x0` |
| `str` at 0x882c4 | store | 4 | `sp -0x2a0` | `0x0` |
| `str` at 0x882d4 | store | 4 | `sp -0x2a4` | `?` |
| `ldr` at 0x882d8 | load | 4 | `abs 0x892a8` | `-` |
| `str` at 0x882e8 | store | 4 | `sp -0x2a0` | `ret(0x22a38)` |
| `ldr` at 0x882ec | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x882f8 | load | 4 | `abs 0x892ac` | `-` |
| `str` at 0x882fc | store | 4 | `sp -0x5dc` | `0x1120e8` |
| `ldr` at 0x88300 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x88304 | store | 4 | `sp -0x5d8` | `ret(0x22a38)` |
| `ldr` at 0x8830c | load | 4 | `abs 0x892b0` | `-` |
| `ldr` at 0x88310 | load | 4 | `abs 0x892b4` | `-` |
| `ldr` at 0x8831c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x88320 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x88348 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8834c | load | 1 | `[sp-24] +0x24` | `-` |
| `ldr` at 0x8835c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88360 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x88364 | store | 4 | `sp -0x4b0` | `[[sp-24]+40]` |
| `ldr` at 0x8836c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x88378 | store | 4 | `sp -0x4b0` | `ret(0xb7944)` |
| `ldr` at 0x8837c | load | 4 | `sp -0x4b0` | `-` |
| `str` at 0x88380 | store | 4 | `sp -0x2a8` | `[sp-1200]` |
| `str` at 0x88388 | store | 4 | `sp -0x4b4` | `?` |
| `ldr` at 0x8838c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88390 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x8839c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x883a0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x883a8 | store | 4 | `sp -0x4b8` | `?` |
| `ldr` at 0x883ac | load | 4 | `sp -0x4b8` | `-` |
| `str` at 0x883b4 | store | 4 | `sp -0x4b8` | `?` |
| `str` at 0x883c0 | store | 4 | `sp -0x4b8` | `?` |
| `ldr` at 0x883c4 | load | 4 | `sp -0x4b8` | `-` |
| `ldr` at 0x883d4 | load | 4 | `sp -0x4b4` | `-` |
| `strb` at 0x883d8 | store | 1 | `(sp-20 + [sp-1204]) +0x0` | `(ret(0x83734) & 0xff)` |
| `ldrb` at 0x883e4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x883f8 | store | 4 | `sp -0x2b0` | `ret(0x5ea88)` |
| `ldr` at 0x883fc | load | 4 | `abs 0x892b8` | `-` |
| `ldr` at 0x88400 | load | 4 | `abs 0x148514` | `-` |
| `ldr` at 0x8840c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88410 | load | 4 | `abs 0x892bc` | `-` |
| `ldr` at 0x88414 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x88418 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x88438 | load | 4 | `sp -0x2a8` | `-` |
| `str` at 0x8843c | store | 4 | `sp -0x5cc` | `[sp-680]` |
| `ldr` at 0x88440 | load | 4 | `abs 0x892c0` | `-` |
| `ldr` at 0x88444 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88450 | load | 4 | `abs 0x892c0` | `-` |
| `ldr` at 0x88454 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88460 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x8846c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88478 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88484 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88490 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x8849c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x884a8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x884b8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x884c4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x884d4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x884e0 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x884ec | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x884f8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88508 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88514 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88524 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88530 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x8853c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88548 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88554 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88560 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88570 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x8857c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x8858c | load | 4 | `abs 0x892c4` | `-` |
| `ldr` at 0x88590 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x8859c | load | 4 | `abs 0x892c4` | `-` |
| `ldr` at 0x885a0 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x885ac | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x885b8 | load | 4 | `abs 0x892c8` | `-` |
| `ldr` at 0x885bc | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x885cc | load | 4 | `abs 0x892cc` | `-` |
| `ldr` at 0x885d0 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x885e0 | load | 4 | `abs 0x892d0` | `-` |
| `ldr` at 0x885e4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x885f0 | load | 4 | `abs 0x892d0` | `-` |
| `ldr` at 0x885f4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88600 | load | 4 | `abs 0x892d4` | `-` |
| `ldr` at 0x88604 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88610 | load | 4 | `abs 0x892d4` | `-` |
| `ldr` at 0x88614 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88620 | load | 4 | `abs 0x892d8` | `-` |
| `ldr` at 0x88624 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88630 | load | 4 | `abs 0x892d8` | `-` |
| `ldr` at 0x88634 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88640 | load | 4 | `abs 0x892dc` | `-` |
| `ldr` at 0x88644 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88650 | load | 4 | `abs 0x892e0` | `-` |
| `ldr` at 0x88654 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88664 | load | 4 | `abs 0x892e4` | `-` |
| `ldr` at 0x88668 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88674 | load | 4 | `abs 0x892e8` | `-` |
| `ldr` at 0x88678 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88688 | load | 4 | `abs 0x892ec` | `-` |
| `ldr` at 0x8868c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88698 | load | 4 | `abs 0x892ec` | `-` |
| `ldr` at 0x8869c | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x886a8 | load | 4 | `abs 0x892f0` | `-` |
| `ldr` at 0x886ac | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x886b8 | load | 4 | `abs 0x892f4` | `-` |
| `ldr` at 0x886bc | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x886cc | load | 4 | `abs 0x892f8` | `-` |
| `ldr` at 0x886d0 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x886dc | load | 4 | `abs 0x892fc` | `-` |
| `ldr` at 0x886e0 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x886f0 | load | 4 | `abs 0x89300` | `-` |
| `ldr` at 0x886f4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88700 | load | 4 | `abs 0x89300` | `-` |
| `ldr` at 0x88704 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88710 | load | 4 | `abs 0x89304` | `-` |
| `ldr` at 0x88714 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88720 | load | 4 | `abs 0x89304` | `-` |
| `ldr` at 0x88724 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88730 | load | 4 | `abs 0x89308` | `-` |
| `ldr` at 0x88734 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88740 | load | 4 | `abs 0x8930c` | `-` |
| `ldr` at 0x88744 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88754 | load | 4 | `abs 0x89310` | `-` |
| `ldr` at 0x88758 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88764 | load | 4 | `abs 0x89314` | `-` |
| `ldr` at 0x88768 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88778 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88784 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x88790 | load | 4 | `abs 0x89318` | `-` |
| `ldr` at 0x88794 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x887a0 | load | 4 | `abs 0x8931c` | `-` |
| `ldr` at 0x887a4 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x887b4 | load | 4 | `abs 0x89320` | `-` |
| `ldr` at 0x887b8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x887c4 | load | 4 | `abs 0x89320` | `-` |
| `ldr` at 0x887c8 | load | 4 | `sp -0x5cc` | `-` |
| `ldr` at 0x887d4 | load | 4 | `abs 0x89324` | `-` |
| `ldr` at 0x887d8 | load | 4 | `sp -0x5cc` | `-` |
| `str` at 0x887f4 | store | 4 | `sp -0x2b8` | `ret(0x1f1dc)` |
| `ldr` at 0x88820 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8884c | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x88878 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88880 | load | 4 | `sp -0x2a8` | `-` |
| `ldr` at 0x8888c | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x888a0 | store | 4 | `sp -0x2c0` | `?` |
| `ldr` at 0x888a4 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x888a8 | store | 4 | `sp -0x2c4` | `[sp-676]` |
| `ldr` at 0x888ac | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x888b0 | store | 4 | `sp -0x4bc` | `[sp-676]` |
| `ldr` at 0x888b4 | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x888c0 | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x888cc | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x888dc | load | 4 | `sp -0x4bc` | `-` |
| `ldr` at 0x888ec | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0x888fc | store | 4 | `sp -0x2c4` | `0x23` |
| `str` at 0x88908 | store | 4 | `sp -0x2c4` | `0x2c` |
| `ldrb` at 0x88914 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x88918 | load | 4 | `abs 0x89328` | `-` |
| `ldr` at 0x88920 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0x88938 | load | 4 | `sp -0x2c4` | `-` |
| `ldr` at 0x8895c | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x88970 | store | 4 | `sp -0x2c0` | `?` |
| `str` at 0x88980 | store | 4 | `sp -0x2b4` | `ret(0x74908)` |
| `ldr` at 0x88984 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88988 | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x88990 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88994 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0x8899c | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x889a0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x889a4 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `ldr` at 0x889b0 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x889c0 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x889c4 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `str` at 0x889c8 | store | 4 | `sp -0x4c0` | `[ret(0x74908)+16]` |
| `str` at 0x889d4 | store | 4 | `sp -0x4c0` | `0x23` |
| `ldr` at 0x889d8 | load | 4 | `sp -0x4c0` | `-` |
| `str` at 0x889dc | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x889f0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88a08 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x88a1c | store | 4 | `sp -0x2b4` | `ret(0x74908)` |
| `ldr` at 0x88a20 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88a24 | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x88a2c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88a30 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0x88a38 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x88a3c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88a40 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `ldr` at 0x88a4c | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x88a5c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88a60 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `str` at 0x88a64 | store | 4 | `sp -0x4c4` | `[ret(0x74908)+16]` |
| `str` at 0x88a70 | store | 4 | `sp -0x4c4` | `0x23` |
| `ldr` at 0x88a74 | load | 4 | `sp -0x4c4` | `-` |
| `str` at 0x88a78 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x88a8c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88aa4 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x88ab8 | store | 4 | `sp -0x2b4` | `ret(0x74908)` |
| `ldr` at 0x88abc | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88ac0 | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x88ac8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88acc | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0x88ad4 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x88ad8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88adc | load | 4 | `ret(0x74908) +0x10` | `-` |
| `ldr` at 0x88ae8 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x88af8 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88afc | load | 4 | `ret(0x74908) +0x10` | `-` |
| `str` at 0x88b00 | store | 4 | `sp -0x4c8` | `[ret(0x74908)+16]` |
| `str` at 0x88b0c | store | 4 | `sp -0x4c8` | `0x23` |
| `ldr` at 0x88b10 | load | 4 | `sp -0x4c8` | `-` |
| `str` at 0x88b14 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x88b28 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88b40 | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x88b54 | store | 4 | `sp -0x2b4` | `ret(0x74908)` |
| `ldr` at 0x88b58 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88b5c | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x88b64 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88b68 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0x88b70 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x88b74 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88b78 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `ldr` at 0x88b84 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x88b94 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88b98 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `str` at 0x88b9c | store | 4 | `sp -0x4cc` | `[ret(0x74908)+16]` |
| `str` at 0x88ba8 | store | 4 | `sp -0x4cc` | `0x23` |
| `ldr` at 0x88bac | load | 4 | `sp -0x4cc` | `-` |
| `str` at 0x88bb0 | store | 4 | `sp -0x2c4` | `?` |
| `ldr` at 0x88bc4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88bdc | load | 4 | `sp -0x2c4` | `-` |
| `str` at 0x88c0c | store | 4 | `sp -0x2b4` | `ret(0x74908)` |
| `ldr` at 0x88c10 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88c14 | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x88c1c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88c20 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0x88c28 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x88c3c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88c4c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88c58 | load | 4 | `[sp-692] +0x10` | `-` |
| `str` at 0x88c6c | store | 4 | `sp -0x2b4` | `ret(0x74908)` |
| `ldr` at 0x88c70 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88c74 | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x88c7c | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88c80 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `str` at 0x88c88 | store | 4 | `sp -0x2b8` | `?` |
| `ldr` at 0x88c9c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88cac | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x88cb8 | load | 4 | `[sp-692] +0x10` | `-` |
| `ldr` at 0x88cc4 | load | 4 | `sp -0x2a4` | `-` |
| `ldrb` at 0x88cd8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x88ce4 | store | 4 | `sp -0x2bc` | `ret(0x4f148)` |
| `str` at 0x88cf0 | store | 4 | `sp -0x2bc` | `ret(0x4f2f0)` |
| `ldr` at 0x88cf4 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x88d34 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x88d50 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x88d58 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x88d84 | load | 4 | `abs 0x8932c` | `-` |
| `str` at 0x88d94 | store | 4 | `sp -0x2b8` | `ret(0x1b430)` |
| `ldr` at 0x88da8 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88db0 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x88dcc | store | 4 | `sp -0x2b8` | `ret(0x1b430)` |
| `ldr` at 0x88de0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88de8 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x88e00 | store | 4 | `sp -0x2b8` | `ret(0x1b430)` |
| `ldr` at 0x88e14 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x88e20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x88e24 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x88e4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x88e50 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x88e60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88e64 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x88e68 | store | 4 | `sp -0x4d0` | `[[sp-24]+48]` |
| `ldr` at 0x88e70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x88e7c | store | 4 | `sp -0x4d0` | `ret(0xb7944)` |
| `ldr` at 0x88e80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88e84 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x88e90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x88e94 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x88e9c | store | 4 | `sp -0x4d4` | `?` |
| `ldr` at 0x88ea0 | load | 4 | `sp -0x4d4` | `-` |
| `str` at 0x88ea8 | store | 4 | `sp -0x4d4` | `?` |
| `str` at 0x88eb4 | store | 4 | `sp -0x4d4` | `?` |
| `ldr` at 0x88eb8 | load | 4 | `sp -0x4d4` | `-` |
| `ldr` at 0x88ebc | load | 4 | `sp -0x4d0` | `-` |
| `str` at 0x88ec8 | store | 4 | `sp -0x2cc` | `ret(0x87824)` |
| `str` at 0x88ee0 | store | 4 | `sp -0x2d0` | `ret(0xd1dfc)` |
| `ldr` at 0x88ee4 | load | 4 | `abs 0x89334` | `-` |
| `ldrb` at 0x88eec | load | 1 | `sp -0x2cc` | `-` |
| `ldr` at 0x88f08 | load | 4 | `abs 0x89330` | `-` |
| `ldr` at 0x88f2c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x88f30 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x88f78 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x88f8c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x88f9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x88fa4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x88fb8 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x88fc0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x88fc4 | store | 4 | `sp -0x4d8` | `?` |
| `ldr` at 0x88fc8 | load | 4 | `sp -0x4d8` | `-` |
| `ldr` at 0x88fd4 | load | 4 | `sp -0x4d8` | `-` |
| `ldr` at 0x88fec | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89000 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89010 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89020 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89034 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89054 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89068 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89078 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x890a0 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x890b4 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x890c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x890d4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x890e8 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x890f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89108 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8911c | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x8912c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8913c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89150 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89160 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x89174 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89188 | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x89198 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x891a8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x891bc | load | 4 | `sp -0x2c8` | `-` |
| `ldr` at 0x891cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x891d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x891dc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x89204 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89208 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x89218 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8921c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x89220 | store | 4 | `sp -0x4dc` | `[[sp-24]+48]` |
| `ldr` at 0x89228 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x89234 | store | 4 | `sp -0x4dc` | `ret(0xb7944)` |
| `ldr` at 0x89238 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8923c | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x89248 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8924c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x89254 | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x89258 | load | 4 | `sp -0x4e0` | `-` |
| `str` at 0x89260 | store | 4 | `sp -0x4e0` | `?` |
| `str` at 0x8926c | store | 4 | `sp -0x4e0` | `?` |
| `ldr` at 0x89270 | load | 4 | `sp -0x4e0` | `-` |
| `ldr` at 0x89274 | load | 4 | `sp -0x4dc` | `-` |
| `str` at 0x89280 | store | 4 | `sp -0x2cc` | `ret(0x87824)` |
| `ldr` at 0x89284 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89288 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x89298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8929c | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x892a0 | store | 4 | `sp -0x4e4` | `[[sp-24]+56]` |
| `ldr` at 0x8934c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x89358 | store | 4 | `sp -0x4e4` | `ret(0xb7944)` |
| `ldr` at 0x8935c | load | 4 | `sp -0x4e4` | `-` |
| `str` at 0x89360 | store | 4 | `sp -0x2d4` | `[sp-1252]` |
| `ldr` at 0x89364 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x89368 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x89378 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8937c | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x89380 | store | 4 | `sp -0x4e8` | `[[sp-24]+64]` |
| `ldr` at 0x89388 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x89394 | store | 4 | `sp -0x4e8` | `ret(0xb7944)` |
| `ldr` at 0x89398 | load | 4 | `sp -0x4e8` | `-` |
| `str` at 0x8939c | store | 4 | `sp -0x2d8` | `[sp-1256]` |
| `ldr` at 0x893a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x893a4 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x893b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x893b8 | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x893bc | store | 4 | `sp -0x4ec` | `[[sp-24]+72]` |
| `ldr` at 0x893c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x893d0 | store | 4 | `sp -0x4ec` | `ret(0xb7944)` |
| `ldr` at 0x893d4 | load | 4 | `sp -0x4ec` | `-` |
| `str` at 0x893d8 | store | 4 | `sp -0x2dc` | `[sp-1260]` |
| `str` at 0x893e0 | store | 4 | `sp -0x2e0` | `0x0` |
| `str` at 0x893e8 | store | 4 | `sp -0x2e4` | `0x0` |
| `str` at 0x89400 | store | 4 | `sp -0x2e8` | `ret(0xd1dfc)` |
| `ldr` at 0x89404 | load | 4 | `abs 0x89334` | `-` |
| `ldrb` at 0x8940c | load | 1 | `sp -0x2cc` | `-` |
| `ldr` at 0x89444 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x89448 | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x89470 | load | 4 | `sp -0x2d4` | `-` |
| `ldrls` at 0x89478 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x898b4 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x898c0 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x898c4 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x898ec | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x898f0 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x898f8 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x89904 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x89908 | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x89910 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0x89918 | store | 4 | `sp -0x2e4` | `0x0` |
| `ldr` at 0x89920 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x89924 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8992c | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x89934 | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0x89960 | load | 4 | `sp -0x2e4` | `-` |
| `ldr` at 0x89968 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8997c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x89980 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x89994 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x899a0 | load | 4 | `abs 0x89344` | `-` |
| `ldr` at 0x899a4 | load | 4 | `abs 0x2ae60c` | `-` |
| `ldr` at 0x899b0 | load | 4 | `abs 0x89344` | `-` |
| `ldr` at 0x899b4 | load | 4 | `abs 0x2ae60c` | `-` |
| `str` at 0x899c8 | store | 4 | `sp -0x2bc` | `ret(unknown)` |
| `str` at 0x899d8 | store | 4 | `sp -0x2bc` | `ret(0x1f1dc)` |
| `ldr` at 0x899dc | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x89a24 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x89a40 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x89a48 | load | 4 | `sp -0x2bc` | `-` |
| `str` at 0x89a64 | store | 4 | `sp -0x2b8` | `ret(0x1f1dc)` |
| `ldr` at 0x89a90 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x89aa8 | store | 4 | `sp -0x2b8` | `ret(0x1f1dc)` |
| `ldr` at 0x89ad4 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x89af8 | load | 4 | `sp -0x2a4` | `-` |
| `ldrls` at 0x89b04 | load | 4 | `(unknown)` | `-` |
| `str` at 0x89d5c | store | 4 | `sp -0x2b8` | `ret(0x1f1dc)` |
| `ldr` at 0x89d70 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x89d84 | store | 4 | `sp -0x2b8` | `ret(0x1f1dc)` |
| `ldr` at 0x89d98 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x89da4 | load | 4 | `sp -0x2a4` | `-` |
| `ldrb` at 0x89dd8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x89df8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x89e04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89e10 | load | 4 | `abs 0x8933c` | `-` |
| `ldrb` at 0x89e4c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89e60 | load | 4 | `sp -0x2a4` | `-` |
| `str` at 0x89e64 | store | 4 | `sp -0x4f0` | `[sp-676]` |
| `ldr` at 0x89e68 | load | 4 | `sp -0x4f0` | `-` |
| `ldr` at 0x89e74 | load | 4 | `sp -0x4f0` | `-` |
| `ldrb` at 0x89eac | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x89ee4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x89ef0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89efc | load | 4 | `abs 0x89340` | `-` |
| `ldrb` at 0x89f38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x89fa4 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x89fec | load | 4 | `sp -0x2e8` | `-` |
| `ldr` at 0x8a004 | load | 4 | `sp -0x2e4` | `-` |
| `str` at 0x8a018 | store | 4 | `sp -0x2dc` | `ret(0x5d81c)` |
| `ldr` at 0x8a01c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a044 | load | 4 | `abs 0x89344` | `-` |
| `ldr` at 0x8a048 | load | 4 | `abs 0x2ae5b4` | `-` |
| `str` at 0x8a04c | store | 4 | `sp -0x4f4` | `[0x2ae5b4]` |
| `ldr` at 0x8a050 | load | 4 | `abs 0x89344` | `-` |
| `ldr` at 0x8a054 | load | 4 | `abs 0x2ae5b4` | `-` |
| `str` at 0x8a064 | store | 4 | `sp -0x4f4` | `0x1` |
| `ldr` at 0x8a068 | load | 4 | `sp -0x4f4` | `-` |
| `str` at 0x8a06c | store | 4 | `sp -0x2d4` | `?` |
| `ldr` at 0x8a070 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x8a084 | store | 4 | `sp -0x2d0` | `ret(0x1b124)` |
| `ldr` at 0x8a088 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8a090 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8a0a0 | load | 4 | `sp -0x2d4` | `-` |
| `str` at 0x8a0b4 | store | 4 | `sp -0x2cc` | `ret(0x1b124)` |
| `ldr` at 0x8a0b8 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8a0c0 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x8a0d4 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x8a0d8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a0dc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a0ec | load | 4 | `abs 0x89344` | `-` |
| `ldr` at 0x8a0f4 | load | 4 | `abs 0x2ae58c` | `-` |
| `str` at 0x8a0fc | store | 4 | `sp -0x2f0` | `?` |
| `ldr` at 0x8a100 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a104 | load | 4 | `sp -0x2f0` | `-` |
| `str` at 0x8a110 | store | 4 | `sp -0x2f4` | `ret(0xfa70c)` |
| `ldr` at 0x8a114 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a11c | load | 4 | `sp -0x2f0` | `-` |
| `ldr` at 0x8a128 | load | 4 | `abs 0x89344` | `-` |
| `ldr` at 0x8a12c | load | 4 | `abs 0x2ae58c` | `-` |
| `str` at 0x8a134 | store | 4 | `sp -0x2f8` | `?` |
| `ldr` at 0x8a138 | load | 4 | `abs 0x89348` | `-` |
| `ldrb` at 0x8a140 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a158 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8a160 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8a168 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8a170 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8a17c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8a184 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8a18c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8a190 | load | 4 | `abs 0x89348` | `-` |
| `ldrb` at 0x8a198 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a1b0 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8a1b8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8a1c0 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8a1c8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8a1d4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8a1dc | load | 4 | `(unknown)` | `-` |
| `str` at 0x8a1e4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8a1e8 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8a1f0 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x8a220 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8a238 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8a244 | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x8a248 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8a24c | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8a264 | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8a26c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8a27c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8a28c | load | 4 | `sp -0x2d8` | `-` |
| `ldr` at 0x8a294 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x8a2a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8a2ac | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8a2b4 | store | 4 | `sp -0x2d8` | `[sp-728]+1` |
| `ldr` at 0x8a2bc | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8a2c4 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x8a2d4 | store | 4 | `sp -0x2f8` | `sp-868` |
| `str` at 0x8a300 | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x8a304 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8a314 | load | 4 | `sp -0x318` | `-` |
| `str` at 0x8a320 | store | 4 | `sp -0x318` | `?` |
| `ldr` at 0x8a324 | load | 4 | `sp -0x310` | `-` |
| `str` at 0x8a32c | store | 4 | `sp -0x310` | `?` |
| `ldr` at 0x8a330 | load | 4 | `sp -0x310` | `-` |
| `str` at 0x8a338 | store | 4 | `sp -0x310` | `?` |
| `str` at 0x8a340 | store | 4 | `sp -0x30c` | `0xc5` |
| `ldr` at 0x8a344 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8a350 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8a354 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a358 | load | 4 | `abs 0x8b2fc` | `-` |
| `ldr` at 0x8a364 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a36c | load | 4 | `abs 0x8b2fc` | `-` |
| `str` at 0x8a378 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8a37c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a380 | load | 4 | `abs 0x8b300` | `-` |
| `ldr` at 0x8a38c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a394 | load | 4 | `abs 0x8b300` | `-` |
| `str` at 0x8a3a0 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8a3a4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a3a8 | load | 4 | `abs 0x8b304` | `-` |
| `ldr` at 0x8a3b4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a3bc | load | 4 | `abs 0x8b304` | `-` |
| `str` at 0x8a3c8 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8a3cc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a3dc | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8a3f0 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8a3f4 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8a404 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8a418 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x8a41c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8a420 | store | 4 | `sp -0x308` | `[sp-732]` |
| `str` at 0x8a430 | store | 4 | `sp -0x2a0` | `ret(0xe3430)` |
| `ldr` at 0x8a434 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8a440 | load | 4 | `sp -0x300` | `-` |
| `str` at 0x8a444 | store | 4 | `sp -0x324` | `[sp-768]` |
| `ldr` at 0x8a448 | load | 4 | `sp -0x2fc` | `-` |
| `str` at 0x8a44c | store | 4 | `sp -0x320` | `[sp-764]` |
| `ldr` at 0x8a450 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x8a454 | store | 4 | `sp -0x31c` | `[sp-760]` |
| `str` at 0x8a45c | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x8a460 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a470 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a47c | load | 4 | `abs 0x8b308` | `-` |
| `ldr` at 0x8a490 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a4a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8a4b4 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8a4bc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8a4c4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8a4cc | load | 4 | `sp -0x2e0` | `-` |
| `ldrb` at 0x8a4dc | load | 1 | `(unknown)` | `-` |
| `strb` at 0x8a4e4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8a4e8 | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8a4f0 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x8a520 | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x8a52c | store | 4 | `sp -0x2a0` | `0x0` |
| `ldr` at 0x8a534 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x8a53c | store | 4 | `sp -0x2f8` | `[sp-760]+8` |
| `ldr` at 0x8a540 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8a548 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x8a550 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8a598 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a59c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8a5c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a5c8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8a5d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a5dc | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8a5e0 | store | 4 | `sp -0x4f8` | `[[sp-24]+48]` |
| `ldr` at 0x8a5e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8a5f4 | store | 4 | `sp -0x4f8` | `ret(0xb7944)` |
| `ldr` at 0x8a5fc | load | 4 | `sp -0x4f8` | `-` |
| `ldr` at 0x8a620 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a624 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8a64c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a650 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0x8a65c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a660 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x8a668 | store | 4 | `sp -0x4fc` | `?` |
| `ldr` at 0x8a66c | load | 4 | `sp -0x4fc` | `-` |
| `str` at 0x8a674 | store | 4 | `sp -0x4fc` | `?` |
| `str` at 0x8a680 | store | 4 | `sp -0x4fc` | `0x60` |
| `ldr` at 0x8a684 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a688 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8a698 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a69c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8a6a0 | load | 4 | `sp -0x4fc` | `-` |
| `str` at 0x8a6a8 | store | 4 | `sp -0x500` | `?` |
| `ldr` at 0x8a6b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a6c0 | load | 4 | `sp -0x4fc` | `-` |
| `str` at 0x8a6c8 | store | 4 | `sp -0x500` | `?` |
| `ldr` at 0x8a6cc | load | 4 | `sp -0x500` | `-` |
| `str` at 0x8a6fc | store | 4 | `sp -0x2b4` | `ret(0x74908)` |
| `ldr` at 0x8a700 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8a704 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8a708 | load | 4 | `sp -0x2b4` | `-` |
| `ldr` at 0x8a70c | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x8a710 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `ldr` at 0x8a714 | load | 4 | `ret(0x74908) +0x14` | `-` |
| `ldr` at 0x8a738 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a73c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8a764 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a768 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8a794 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a798 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8a7a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a7ac | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8a7b0 | store | 4 | `sp -0x504` | `[[sp-24]+48]` |
| `ldr` at 0x8a7b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8a7c4 | store | 4 | `sp -0x504` | `ret(0xb7944)` |
| `ldr` at 0x8a7cc | load | 4 | `sp -0x504` | `-` |
| `ldr` at 0x8a7f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a7f8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8a820 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a824 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8a834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a838 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8a83c | store | 4 | `sp -0x508` | `[[sp-24]+48]` |
| `ldr` at 0x8a844 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8a850 | store | 4 | `sp -0x508` | `ret(0xb7944)` |
| `ldr` at 0x8a858 | load | 4 | `sp -0x508` | `-` |
| `ldr` at 0x8a890 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a894 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8a8bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a8c0 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8a8d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a8d4 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8a8d8 | store | 4 | `sp -0x50c` | `[[sp-24]+48]` |
| `ldr` at 0x8a8e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8a8ec | store | 4 | `sp -0x50c` | `ret(0xb7944)` |
| `ldr` at 0x8a8f4 | load | 4 | `sp -0x50c` | `-` |
| `ldr` at 0x8a918 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a91c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8a944 | load | 4 | `abs 0x8b30c` | `-` |
| `str` at 0x8a948 | store | 4 | `sp -0x510` | `0x2b1720` |
| `ldr` at 0x8a94c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a950 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8a960 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8a964 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8a968 | store | 4 | `sp -0x514` | `[[sp-24]+48]` |
| `ldr` at 0x8a970 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8a97c | store | 4 | `sp -0x514` | `ret(0xb7944)` |
| `ldrb` at 0x8a988 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8a990 | load | 4 | `sp -0x514` | `-` |
| `ldr` at 0x8a998 | load | 4 | `sp -0x510` | `-` |
| `str` at 0x8a99c | store | 4 | `[sp-1296] +0xc` | `ret(0x837a4)` |
| `ldr` at 0x8a9a0 | load | 4 | `abs 0x8b30c` | `-` |
| `ldr` at 0x8a9a8 | load | 4 | `abs 0x2b172c` | `-` |
| `ldr` at 0x8a9cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a9d0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8a9f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8a9fc | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8aa0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8aa10 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8aa18 | store | 4 | `sp -0x528` | `?` |
| `ldr` at 0x8aa20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8aa2c | store | 4 | `sp -0x528` | `ret(0xb7944)` |
| `ldr` at 0x8aa30 | load | 4 | `sp -0x528` | `-` |
| `str` at 0x8aa38 | store | 4 | `sp -0x528` | `?` |
| `ldr` at 0x8aa3c | load | 4 | `sp -0x528` | `-` |
| `ldr` at 0x8aa48 | load | 4 | `sp -0x528` | `-` |
| `ldr` at 0x8aa54 | load | 4 | `sp -0x528` | `-` |
| `ldr` at 0x8aa64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8aa68 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8aa78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8aa7c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8aa80 | store | 4 | `sp -0x518` | `[[sp-24]+48]` |
| `ldr` at 0x8aa88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8aa94 | store | 4 | `sp -0x518` | `ret(0xb7944)` |
| `ldr` at 0x8aa98 | load | 4 | `sp -0x518` | `-` |
| `ldr` at 0x8aaa8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8aaac | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8aabc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8aac0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8aac4 | store | 4 | `sp -0x51c` | `[[sp-24]+48]` |
| `ldr` at 0x8aacc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8aad8 | store | 4 | `sp -0x51c` | `ret(0xb7944)` |
| `ldr` at 0x8aadc | load | 4 | `sp -0x51c` | `-` |
| `ldr` at 0x8aae8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8aaec | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8aafc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ab00 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8ab04 | store | 4 | `sp -0x520` | `[[sp-24]+48]` |
| `ldr` at 0x8ab0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ab18 | store | 4 | `sp -0x520` | `ret(0xb7944)` |
| `ldr` at 0x8ab1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ab20 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8ab30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ab34 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8ab3c | store | 4 | `sp -0x524` | `?` |
| `ldr` at 0x8ab44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ab50 | store | 4 | `sp -0x524` | `ret(0xb7944)` |
| `ldr` at 0x8ab54 | load | 4 | `sp -0x524` | `-` |
| `str` at 0x8ab5c | store | 4 | `sp -0x524` | `?` |
| `ldr` at 0x8ab60 | load | 4 | `sp -0x520` | `-` |
| `ldr` at 0x8ab64 | load | 4 | `sp -0x524` | `-` |
| `ldr` at 0x8abb0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8abb4 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8abdc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8abe0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8abf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8abf4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8abfc | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8ac00 | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8ac08 | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8ac10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ac1c | store | 4 | `sp -0x530` | `ret(0xb7944)` |
| `ldr` at 0x8ac20 | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8ac28 | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8ac2c | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8ac34 | store | 4 | `sp -0x530` | `?` |
| `ldr` at 0x8ac38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ac3c | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8ac4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ac50 | load | 4 | `[sp-24] +0x40` | `-` |
| `ldr` at 0x8ac60 | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8ac68 | store | 4 | `sp -0x52c` | `?` |
| `ldr` at 0x8ac70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ac8c | load | 4 | `sp -0x530` | `-` |
| `str` at 0x8ac94 | store | 4 | `sp -0x52c` | `?` |
| `ldr` at 0x8ac98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ac9c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8acac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8acb0 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8acb8 | load | 4 | `sp -0x52c` | `-` |
| `str` at 0x8acc0 | store | 4 | `sp -0x534` | `?` |
| `ldr` at 0x8acc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8acdc | load | 4 | `sp -0x52c` | `-` |
| `str` at 0x8ace4 | store | 4 | `sp -0x534` | `?` |
| `ldr` at 0x8ace8 | load | 4 | `sp -0x534` | `-` |
| `str` at 0x8acec | store | 4 | `sp -0x2b0` | `[sp-1332]` |
| `ldrb` at 0x8acf8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8ad04 | load | 4 | `sp -0x2b0` | `-` |
| `ldrb` at 0x8ad14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8ad18 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x8ad20 | load | 4 | `sp -0x2b0` | `-` |
| `ldr` at 0x8ad30 | load | 4 | `sp -0x2b0` | `-` |
| `str` at 0x8ad38 | store | 4 | `sp -0x5dc` | `?` |
| `ldr` at 0x8ad3c | load | 4 | `abs 0x8b310` | `-` |
| `ldr` at 0x8ad48 | load | 4 | `abs 0x8b30c` | `-` |
| `ldr` at 0x8ad4c | load | 4 | `abs 0x2b172c` | `-` |
| `ldr` at 0x8ad58 | load | 4 | `abs 0x8b30c` | `-` |
| `ldr` at 0x8ad5c | load | 4 | `abs 0x2b172c` | `-` |
| `ldrb` at 0x8ad70 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8ad84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ad88 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldrb` at 0x8adb8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8adbc | store | 4 | `sp -0x538` | `?` |
| `ldr` at 0x8adc0 | load | 4 | `sp -0x538` | `-` |
| `str` at 0x8adc8 | store | 4 | `sp -0x538` | `?` |
| `ldr` at 0x8adcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8add0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8ade0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ade4 | load | 4 | `[sp-24] +0x38` | `-` |
| `ldr` at 0x8ade8 | load | 4 | `sp -0x538` | `-` |
| `str` at 0x8adf4 | store | 4 | `sp -0x53c` | `?` |
| `ldr` at 0x8adfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ae0c | load | 4 | `sp -0x538` | `-` |
| `str` at 0x8ae18 | store | 4 | `sp -0x53c` | `?` |
| `ldr` at 0x8ae1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ae20 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8ae30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ae34 | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8ae38 | store | 4 | `sp -0x540` | `[[sp-24]+48]:1` |
| `ldr` at 0x8ae40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ae54 | store | 4 | `sp -0x540` | `?` |
| `ldr` at 0x8ae58 | load | 4 | `sp -0x53c` | `-` |
| `ldr` at 0x8ae5c | load | 4 | `sp -0x540` | `-` |
| `str` at 0x8ae70 | store | 4 | `sp -0x2b8` | `ret(0x4e10c)` |
| `ldr` at 0x8ae74 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8aea8 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8aeb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8aeb8 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8aec4 | load | 4 | `abs 0x8b314` | `-` |
| `ldr` at 0x8aec8 | load | 4 | `abs 0x1497d4` | `-` |
| `ldr` at 0x8aef0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8aef4 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8af04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8af08 | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8af0c | store | 4 | `sp -0x544` | `[[sp-24]+48]:1` |
| `ldr` at 0x8af14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8af28 | store | 4 | `sp -0x544` | `?` |
| `ldr` at 0x8af2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8af30 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8af40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8af44 | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x8af48 | store | 4 | `sp -0x548` | `[[sp-24]+56]:1` |
| `ldr` at 0x8af50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8af64 | store | 4 | `sp -0x548` | `?` |
| `ldr` at 0x8af68 | load | 4 | `abs 0x8b314` | `-` |
| `ldr` at 0x8af6c | load | 4 | `abs 0x1497d4` | `-` |
| `ldr` at 0x8af74 | load | 4 | `sp -0x544` | `-` |
| `ldr` at 0x8af78 | load | 4 | `sp -0x548` | `-` |
| `str` at 0x8af84 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x8af88 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8afbc | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8afc8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8afcc | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8afd8 | load | 4 | `abs 0x8b318` | `-` |
| `ldr` at 0x8afdc | load | 4 | `abs 0x1497d8` | `-` |
| `ldr` at 0x8b004 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b008 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b018 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b01c | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b020 | store | 4 | `sp -0x54c` | `[[sp-24]+48]:1` |
| `ldr` at 0x8b028 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b03c | store | 4 | `sp -0x54c` | `?` |
| `ldr` at 0x8b040 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b044 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b054 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b058 | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x8b05c | store | 4 | `sp -0x550` | `[[sp-24]+56]:1` |
| `ldr` at 0x8b064 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b078 | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0x8b07c | load | 4 | `abs 0x8b318` | `-` |
| `ldr` at 0x8b080 | load | 4 | `abs 0x1497d8` | `-` |
| `ldr` at 0x8b088 | load | 4 | `sp -0x54c` | `-` |
| `ldr` at 0x8b08c | load | 4 | `sp -0x550` | `-` |
| `str` at 0x8b098 | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x8b09c | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b0d0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b0dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b0e0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8b0ec | load | 4 | `abs 0x8b31c` | `-` |
| `ldr` at 0x8b0f0 | load | 4 | `abs 0x1497dc` | `-` |
| `ldr` at 0x8b118 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b11c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8b12c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b130 | load | 1 | `[sp-24] +0x30` | `-` |
| `str` at 0x8b134 | store | 4 | `sp -0x554` | `[[sp-24]+48]:1` |
| `ldr` at 0x8b13c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b150 | store | 4 | `sp -0x554` | `?` |
| `ldr` at 0x8b154 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b158 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8b168 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8b16c | load | 1 | `[sp-24] +0x38` | `-` |
| `str` at 0x8b170 | store | 4 | `sp -0x558` | `[[sp-24]+56]:1` |
| `ldr` at 0x8b178 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b18c | store | 4 | `sp -0x558` | `?` |
| `ldr` at 0x8b190 | load | 4 | `abs 0x8b31c` | `-` |
| `ldr` at 0x8b194 | load | 4 | `abs 0x1497dc` | `-` |
| `ldr` at 0x8b19c | load | 4 | `sp -0x554` | `-` |
| `ldr` at 0x8b1a0 | load | 4 | `sp -0x558` | `-` |
| `str` at 0x8b1ac | store | 4 | `sp -0x2b8` | `ret(unknown)` |
| `ldr` at 0x8b1b0 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b1e4 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8b1f8 | store | 4 | `sp -0x2b8` | `ret(0x3d48c)` |
| `ldr` at 0x8b1fc | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b230 | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8b244 | store | 4 | `sp -0x2b8` | `ret(0x3d858)` |
| `ldr` at 0x8b248 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8b27c | load | 4 | `sp -0x2b8` | `-` |
| `str` at 0x8b29c | store | 4 | `sp -0x2f4` | `ret(0xd1dfc)` |
| `ldr` at 0x8b2a0 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8b2e4 | store | 4 | `sp -0x2e0` | `0x0` |
| `ldr` at 0x8b2e8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8b2ec | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8b394 | load | 4 | `abs 0x8b358` | `-` |
| `ldrb` at 0x8b39c | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8b3b8 | load | 4 | `abs 0x8b320` | `-` |
| `ldr` at 0x8b3bc | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8b3e0 | store | 4 | `sp -0x2d0` | `(sp-996 + ret(0x1b22c))` |
| `str` at 0x8b3e8 | store | 4 | `sp -0x2d8` | `0x1` |
| `ldr` at 0x8b3ec | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b3f0 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x8b3fc | load | 4 | `abs 0x8b358` | `-` |
| `ldrb` at 0x8b404 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8b428 | load | 4 | `abs 0x8b324` | `-` |
| `ldr` at 0x8b434 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b438 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x8b444 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b448 | load | 4 | `[sp-760] +0x1c` | `-` |
| `ldr` at 0x8b454 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b458 | load | 4 | `abs 0x8b328` | `-` |
| `ldr` at 0x8b468 | load | 4 | `abs 0x8b358` | `-` |
| `ldrb` at 0x8b470 | load | 1 | `sp -0x2e0` | `-` |
| `str` at 0x8b490 | store | 4 | `sp -0x2d4` | `ret(0x8d224)` |
| `ldr` at 0x8b494 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8b4a0 | load | 4 | `abs 0x8b32c` | `-` |
| `str` at 0x8b4a4 | store | 4 | `sp -0x55c` | `0x1121d4` |
| `ldr` at 0x8b4ac | load | 4 | `abs 0x8b330` | `-` |
| `str` at 0x8b4b0 | store | 4 | `sp -0x55c` | `0x1121dc` |
| `ldr` at 0x8b4b4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b4b8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b4bc | load | 4 | `[sp-760] +0x50` | `-` |
| `str` at 0x8b4c0 | store | 4 | `sp -0x5dc` | `[[sp-760]+80]` |
| `ldr` at 0x8b4c4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b4c8 | load | 4 | `[sp-760] +0x54` | `-` |
| `str` at 0x8b4cc | store | 4 | `sp -0x5d8` | `[[sp-760]+84]` |
| `ldr` at 0x8b4d0 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b4d4 | load | 4 | `[sp-760] +0x58` | `-` |
| `ldrb` at 0x8b4d8 | load | 1 | `[[sp-760]+88] +0x0` | `-` |
| `ldr` at 0x8b4e8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b4ec | load | 4 | `[sp-760] +0x58` | `-` |
| `str` at 0x8b4f4 | store | 4 | `sp -0x5d4` | `?` |
| `ldr` at 0x8b4f8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b4fc | load | 4 | `[sp-760] +0x5c` | `-` |
| `str` at 0x8b500 | store | 4 | `sp -0x5d0` | `[[sp-760]+92]` |
| `ldr` at 0x8b504 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b508 | load | 4 | `abs 0x8b334` | `-` |
| `ldr` at 0x8b50c | load | 4 | `sp -0x55c` | `-` |
| `ldr` at 0x8b510 | load | 4 | `[sp-760] +0x4c` | `-` |
| `ldr` at 0x8b51c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b520 | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x8b52c | load | 4 | `abs 0x8b358` | `-` |
| `ldrb` at 0x8b534 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8b558 | load | 4 | `abs 0x8b338` | `-` |
| `ldr` at 0x8b564 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b568 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8b574 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b578 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8b584 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b588 | load | 4 | `abs 0x8b33c` | `-` |
| `ldr` at 0x8b598 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b59c | load | 4 | `abs 0x8b340` | `-` |
| `ldr` at 0x8b5a4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b5b4 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8b5bc | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x1b22c))` |
| `str` at 0x8b5c4 | store | 4 | `sp -0x2dc` | `0x40` |
| `ldr` at 0x8b5c8 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8b5d8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b5dc | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x8b5e0 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8b5e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8b5ec | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b5f0 | load | 4 | `abs 0x8b344` | `-` |
| `ldr` at 0x8b5fc | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8b604 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8b608 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8b610 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `str` at 0x8b61c | store | 4 | `sp -0x2dc` | `0x5f` |
| `ldr` at 0x8b620 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8b630 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b634 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x8b638 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8b640 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8b644 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b648 | load | 4 | `abs 0x8b344` | `-` |
| `ldr` at 0x8b654 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8b65c | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8b660 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8b668 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x8b674 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8b680 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8b688 | store | 4 | `sp -0x2d0` | `?` |
| `str` at 0x8b690 | store | 4 | `sp -0x2dc` | `0x49` |
| `ldr` at 0x8b694 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8b6a8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b6ac | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b6b0 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldr` at 0x8b6b4 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8b6c0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x8b6c4 | store | 1 | `(unknown)` | `?` |
| `str` at 0x8b6cc | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8b6d0 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8b6d8 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x8b6e4 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8b6f0 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8b6f8 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8b6fc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b704 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b70c | load | 1 | `[[sp-760]+12] +0x40` | `-` |
| `ldr` at 0x8b718 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b720 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b728 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x8b740 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b748 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b750 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x8b768 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b770 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b778 | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `ldr` at 0x8b77c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b784 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b78c | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `ldr` at 0x8b790 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b794 | load | 4 | `abs 0x8b348` | `-` |
| `ldr` at 0x8b7a0 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8b7a8 | store | 4 | `sp -0x2d0` | `[sp-720]+4` |
| `ldr` at 0x8b7b4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b7b8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b7c0 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b7cc | load | 1 | `[[sp-760]+12] +0x5b` | `-` |
| `strb` at 0x8b7d0 | store | 1 | `[sp-720] +0x0` | `([[[sp-760]+12]+91]:1 & 0xff)` |
| `str` at 0x8b7d8 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0x8b7e0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b7e4 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b7ec | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b7f8 | load | 1 | `[[sp-760]+12] +0x5c` | `-` |
| `strb` at 0x8b7fc | store | 1 | `[sp-720] +0x1` | `([[[sp-760]+12]+92]:1 & 0xff)` |
| `str` at 0x8b804 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8b80c | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8b818 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8b820 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8b824 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b82c | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b834 | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x8b840 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b848 | load | 4 | `[sp-760] +0xc` | `-` |
| `ldrb` at 0x8b850 | load | 1 | `[[sp-760]+12] +0x9` | `-` |
| `ldr` at 0x8b858 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b85c | load | 4 | `abs 0x8b34c` | `-` |
| `ldr` at 0x8b868 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b878 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8b880 | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x1b22c))` |
| `ldr` at 0x8b884 | load | 4 | `abs 0x8b350` | `-` |
| `ldr` at 0x8b888 | load | 4 | `abs 0x1497e8` | `-` |
| `ldr` at 0x8b894 | load | 4 | `abs 0x8b350` | `-` |
| `ldr` at 0x8b898 | load | 4 | `abs 0x1497e8` | `-` |
| `ldr` at 0x8b89c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b8a8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b8b8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b8bc | load | 4 | `abs 0x8b354` | `-` |
| `ldr` at 0x8b8c8 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b8cc | load | 4 | `[sp-760] +0x0` | `-` |
| `ldr` at 0x8b8d8 | load | 4 | `abs 0x8b358` | `-` |
| `ldrb` at 0x8b8e0 | load | 1 | `sp -0x2e0` | `-` |
| `ldr` at 0x8b904 | load | 4 | `abs 0x8b35c` | `-` |
| `ldr` at 0x8b910 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b914 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8b920 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b924 | load | 4 | `[sp-760] +0x4` | `-` |
| `ldr` at 0x8b930 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b934 | load | 4 | `abs 0x8b360` | `-` |
| `ldr` at 0x8b944 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b948 | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x8b954 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b958 | load | 4 | `[sp-760] +0x20` | `-` |
| `ldr` at 0x8b964 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b968 | load | 4 | `abs 0x8b364` | `-` |
| `ldr` at 0x8b978 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b97c | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x8b988 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b98c | load | 4 | `[sp-760] +0x2c` | `-` |
| `ldr` at 0x8b998 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b99c | load | 4 | `abs 0x8b368` | `-` |
| `ldr` at 0x8b9ac | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b9b0 | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x8b9bc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8b9c0 | load | 4 | `[sp-760] +0x40` | `-` |
| `ldr` at 0x8b9cc | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b9d0 | load | 4 | `abs 0x8b36c` | `-` |
| `ldr` at 0x8b9e0 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b9e4 | load | 4 | `abs 0x8b370` | `-` |
| `ldr` at 0x8b9ec | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8b9fc | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8ba04 | store | 4 | `sp -0x2d0` | `([sp-720] + ret(0x1b22c))` |
| `str` at 0x8ba0c | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0x8ba10 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8ba24 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8ba28 | store | 4 | `sp -0x564` | `?` |
| `str` at 0x8ba30 | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8ba34 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8ba38 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x8ba3c | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8ba44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8ba5c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8ba60 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x8ba64 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8ba6c | load | 1 | `([[sp-760]+68] + [sp-732]) +0x0` | `-` |
| `strb` at 0x8ba74 | store | 1 | `sp -0x55d` | `([([[sp-760]+68] + [sp-732])]:1 & 0xff)` |
| `strb` at 0x8ba84 | store | 1 | `sp -0x55d` | `0x3f` |
| `ldrb` at 0x8ba8c | load | 1 | `sp -0x55d` | `-` |
| `ldr` at 0x8ba90 | load | 4 | `sp -0x564` | `-` |
| `strb` at 0x8ba94 | store | 1 | `[sp-1380] +0x0` | `?` |
| `ldr` at 0x8ba98 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8baa0 | store | 4 | `sp -0x2dc` | `[sp-732]+1` |
| `ldr` at 0x8baac | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8bab8 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8bac0 | store | 4 | `sp -0x2d0` | `?` |
| `str` at 0x8bac8 | store | 4 | `sp -0x2dc` | `0x7` |
| `ldr` at 0x8bacc | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8badc | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bae0 | load | 4 | `[sp-760] +0x44` | `-` |
| `ldr` at 0x8bae8 | load | 4 | `sp -0x2dc` | `-` |
| `ldrb` at 0x8baf4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8baf8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bafc | load | 4 | `abs 0x8b374` | `-` |
| `ldr` at 0x8bb08 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0x8bb10 | store | 4 | `sp -0x2d0` | `[sp-720]+2` |
| `ldr` at 0x8bb14 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8bb1c | store | 4 | `sp -0x2dc` | `[sp-732]-1` |
| `ldr` at 0x8bb28 | load | 4 | `sp -0x2d0` | `-` |
| `strb` at 0x8bb34 | store | 1 | `(unknown)` | `0x2c` |
| `str` at 0x8bb3c | store | 4 | `sp -0x2d0` | `?` |
| `ldr` at 0x8bb40 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bb44 | load | 4 | `[sp-760] +0x24` | `-` |
| `ldrb` at 0x8bb48 | load | 1 | `[[sp-760]+36] +0x3` | `-` |
| `ldr` at 0x8bb4c | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bb50 | load | 4 | `sp -0x2f8` | `-` |
| `ldr` at 0x8bb54 | load | 4 | `[sp-760] +0x34` | `-` |
| `str` at 0x8bb58 | store | 4 | `sp -0x5dc` | `[[sp-760]+52]` |
| `ldr` at 0x8bb5c | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bb60 | load | 4 | `abs 0x8b378` | `-` |
| `ldr` at 0x8bb64 | load | 4 | `[sp-760] +0x28` | `-` |
| `ldr` at 0x8bb78 | load | 4 | `abs 0x8b37c` | `-` |
| `str` at 0x8bb84 | store | 4 | `sp -0x2d8` | `0x0` |
| `ldr` at 0x8bb88 | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8bbb8 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8bbcc | load | 4 | `sp -0x2e0` | `-` |
| `str` at 0x8bbd4 | store | 4 | `sp -0x2e0` | `[sp-736]+1` |
| `ldr` at 0x8bbd8 | load | 4 | `sp -0x2f8` | `-` |
| `str` at 0x8bbe0 | store | 4 | `sp -0x2f8` | `[sp-760]+120` |
| `ldr` at 0x8bbe8 | load | 4 | `abs 0x8b380` | `-` |
| `ldr` at 0x8bbf0 | load | 4 | `abs 0x8b384` | `-` |
| `ldr` at 0x8bbf8 | load | 4 | `abs 0x2b36b0` | `-` |
| `ldr` at 0x8bc04 | load | 4 | `abs 0x8b388` | `-` |
| `ldr` at 0x8bc08 | load | 4 | `abs 0x8b38c` | `-` |
| `ldrb` at 0x8bc0c | load | 1 | `abs 0x112334` | `-` |
| `strb` at 0x8bc14 | store | 1 | `sp -0x4ac` | `([0x112334]:1 & 0xff)` |
| `ldr` at 0x8bc30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8bc34 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8bc44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8bc48 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8bc4c | store | 4 | `sp -0x568` | `[[sp-24]+48]` |
| `ldr` at 0x8bc54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8bc60 | store | 4 | `sp -0x568` | `ret(0xb7944)` |
| `ldr` at 0x8bc64 | load | 4 | `sp -0x568` | `-` |
| `str` at 0x8bc68 | store | 4 | `sp -0x2f4` | `[sp-1384]` |
| `ldr` at 0x8bc6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8bc70 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8bc80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8bc84 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8bc88 | store | 4 | `sp -0x56c` | `[[sp-24]+56]` |
| `ldr` at 0x8bc90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8bc9c | store | 4 | `sp -0x56c` | `ret(0xb7944)` |
| `ldr` at 0x8bca0 | load | 4 | `sp -0x56c` | `-` |
| `str` at 0x8bca4 | store | 4 | `sp -0x2dc` | `[sp-1388]` |
| `ldr` at 0x8bca8 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8bcac | store | 4 | `sp -0x580` | `[sp-756]` |
| `ldr` at 0x8bcb0 | load | 4 | `sp -0x580` | `-` |
| `ldr` at 0x8bcbc | load | 4 | `sp -0x580` | `-` |
| `ldr` at 0x8bccc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8bcd0 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8bce0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8bce4 | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x8bce8 | store | 4 | `sp -0x570` | `[[sp-24]+64]` |
| `ldr` at 0x8bcf0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8bcfc | store | 4 | `sp -0x570` | `ret(0xb7944)` |
| `ldr` at 0x8bd00 | load | 4 | `sp -0x570` | `-` |
| `str` at 0x8bd04 | store | 4 | `sp -0x2d8` | `[sp-1392]` |
| `ldr` at 0x8bd08 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0x8bd20 | load | 4 | `sp -0x2d8` | `-` |
| `str` at 0x8bd34 | store | 4 | `sp -0x2e0` | `ret(0x39684)` |
| `ldr` at 0x8bd38 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8bd5c | load | 4 | `abs 0x8b38c` | `-` |
| `ldrb` at 0x8bd64 | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8bd9c | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8bddc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8bde4 | load | 4 | `abs 0x8b38c` | `-` |
| `ldrb` at 0x8bdec | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8be24 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8be3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8be40 | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8be50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8be54 | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x8be58 | store | 4 | `sp -0x574` | `[[sp-24]+64]` |
| `ldr` at 0x8be60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8be6c | store | 4 | `sp -0x574` | `ret(0xb7944)` |
| `ldr` at 0x8be70 | load | 4 | `sp -0x574` | `-` |
| `str` at 0x8be74 | store | 4 | `sp -0x2d4` | `[sp-1396]` |
| `ldr` at 0x8be78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8be7c | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x8be8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8be90 | load | 4 | `[sp-24] +0x48` | `-` |
| `str` at 0x8be94 | store | 4 | `sp -0x578` | `[[sp-24]+72]` |
| `ldr` at 0x8be9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8bea8 | store | 4 | `sp -0x578` | `ret(0xb7944)` |
| `ldr` at 0x8beac | load | 4 | `sp -0x578` | `-` |
| `str` at 0x8beb0 | store | 4 | `sp -0x2d0` | `[sp-1400]` |
| `ldr` at 0x8beb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8beb8 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x8bec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8becc | load | 4 | `[sp-24] +0x50` | `-` |
| `str` at 0x8bed0 | store | 4 | `sp -0x57c` | `[[sp-24]+80]` |
| `ldr` at 0x8bed8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8bee4 | store | 4 | `sp -0x57c` | `ret(0xb7944)` |
| `ldr` at 0x8bee8 | load | 4 | `sp -0x57c` | `-` |
| `str` at 0x8beec | store | 4 | `sp -0x2cc` | `[sp-1404]` |
| `ldr` at 0x8bef0 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0x8bf00 | store | 4 | `sp -0x5dc` | `0x0` |
| `str` at 0x8bf0c | store | 4 | `sp -0x5d8` | `sp-1196` |
| `ldr` at 0x8bf14 | load | 4 | `sp -0x2d4` | `-` |
| `ldr` at 0x8bf18 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x8bf1c | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x8bf28 | store | 4 | `sp -0x2e0` | `ret(0x3987c)` |
| `ldr` at 0x8bf2c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8bf50 | load | 4 | `abs 0x8b38c` | `-` |
| `ldrb` at 0x8bf58 | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8bf90 | store | 4 | `sp -0x5dc` | `0x1` |
| `ldr` at 0x8bfd0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8bfd8 | load | 4 | `abs 0x8b38c` | `-` |
| `ldrb` at 0x8bfe0 | load | 1 | `sp -0x4ac` | `-` |
| `str` at 0x8c018 | store | 4 | `sp -0x5dc` | `0x1` |
| `str` at 0x8c03c | store | 4 | `sp -0x2b8` | `ret(0x1f1dc)` |
| `ldr` at 0x8c07c | load | 4 | `abs 0x8b390` | `-` |
| `ldr` at 0x8c0ac | load | 4 | `abs 0x8b390` | `-` |
| `ldr` at 0x8c0c8 | load | 4 | `sp -0x2b8` | `-` |
| `ldr` at 0x8c0d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c0d8 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8c104 | store | 4 | `sp -0x584` | `ret(0xdadb8)` |
| `ldr` at 0x8c108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c10c | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c11c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c120 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8c124 | load | 4 | `sp -0x584` | `-` |
| `ldr` at 0x8c134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c144 | load | 4 | `sp -0x584` | `-` |
| `ldr` at 0x8c170 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c174 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c184 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c188 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8c18c | store | 4 | `sp -0x588` | `[[sp-24]+48]` |
| `ldr` at 0x8c194 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c1a0 | store | 4 | `sp -0x588` | `ret(0xb7944)` |
| `ldr` at 0x8c1a4 | load | 4 | `sp -0x588` | `-` |
| `str` at 0x8c1b0 | store | 4 | `sp -0x2a0` | `ret(0xdaec8)` |
| `ldr` at 0x8c1b4 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c1e8 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c210 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c214 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8c240 | store | 4 | `sp -0x58c` | `ret(0xdadb8)` |
| `ldr` at 0x8c244 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c248 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c258 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c25c | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8c260 | load | 4 | `sp -0x58c` | `-` |
| `ldr` at 0x8c270 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c280 | load | 4 | `sp -0x58c` | `-` |
| `ldr` at 0x8c2ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c2b0 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c2c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c2c4 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8c2c8 | store | 4 | `sp -0x590` | `[[sp-24]+48]` |
| `ldr` at 0x8c2d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c2dc | store | 4 | `sp -0x590` | `ret(0xb7944)` |
| `ldr` at 0x8c2e8 | load | 4 | `sp -0x590` | `-` |
| `str` at 0x8c2f8 | store | 4 | `sp -0x2a0` | `ret(0xdaf80)` |
| `ldr` at 0x8c2fc | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c330 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c370 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c374 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8c380 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c384 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c398 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8c39c | store | 4 | `sp -0x594` | `[[sp-24]+48]` |
| `ldr` at 0x8c3a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c3b0 | store | 4 | `sp -0x594` | `ret(0xb7944)` |
| `ldr` at 0x8c3b4 | load | 4 | `sp -0x594` | `-` |
| `str` at 0x8c3b8 | store | 4 | `sp -0x2e0` | `[sp-1428]` |
| `ldr` at 0x8c3bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c3c0 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8c40c | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x8c410 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c414 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c424 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c428 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c43c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c44c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c450 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c464 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8c468 | store | 4 | `sp -0x598` | `?` |
| `ldr` at 0x8c470 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c478 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c48c | store | 4 | `sp -0x598` | `?` |
| `ldr` at 0x8c49c | load | 4 | `sp -0x598` | `-` |
| `str` at 0x8c4ac | store | 4 | `sp -0x2a0` | `ret(0xd6b9c)` |
| `ldr` at 0x8c4b0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c4e4 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c4fc | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c518 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c528 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8c530 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x8c538 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c53c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8c564 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c568 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c57c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8c580 | store | 4 | `sp -0x59c` | `[[sp-24]+48]` |
| `ldr` at 0x8c588 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c594 | store | 4 | `sp -0x59c` | `ret(0xb7944)` |
| `ldr` at 0x8c598 | load | 4 | `sp -0x59c` | `-` |
| `str` at 0x8c59c | store | 4 | `sp -0x2e0` | `[sp-1436]` |
| `ldr` at 0x8c5a0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c5b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c5b8 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0x8c5fc | store | 4 | `sp -0x2f4` | `0x0` |
| `ldr` at 0x8c600 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c604 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8c614 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c618 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c638 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c648 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c64c | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c66c | load | 1 | `(unknown)` | `-` |
| `str` at 0x8c670 | store | 4 | `sp -0x5a0` | `?` |
| `ldr` at 0x8c678 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c68c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c6a0 | store | 4 | `sp -0x5a0` | `?` |
| `ldr` at 0x8c6a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c6a8 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c6c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c6d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c6dc | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c6fc | load | 1 | `(unknown)` | `-` |
| `str` at 0x8c700 | store | 4 | `sp -0x5a4` | `?` |
| `ldr` at 0x8c708 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c71c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c730 | store | 4 | `sp -0x5a4` | `?` |
| `ldr` at 0x8c734 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c738 | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c758 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c768 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c76c | load | 4 | `sp -0x2f4` | `-` |
| `ldrb` at 0x8c78c | load | 1 | `(unknown)` | `-` |
| `str` at 0x8c790 | store | 4 | `sp -0x5a8` | `?` |
| `ldr` at 0x8c798 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c7ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c7c0 | store | 4 | `sp -0x5a8` | `?` |
| `ldr` at 0x8c7c8 | load | 4 | `sp -0x5a0` | `-` |
| `ldr` at 0x8c7cc | load | 4 | `sp -0x5a4` | `-` |
| `ldr` at 0x8c7d0 | load | 4 | `sp -0x5a8` | `-` |
| `str` at 0x8c7dc | store | 4 | `sp -0x2a0` | `ret(0xd6a1c)` |
| `ldr` at 0x8c7e0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c814 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8c82c | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c838 | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8c840 | store | 4 | `sp -0x2f4` | `[sp-756]+1` |
| `ldr` at 0x8c848 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c84c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8c874 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c880 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c88c | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c898 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c8a4 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c8b0 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c8bc | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c8c8 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c8d4 | load | 4 | `sp -0x2a4` | `-` |
| `ldr` at 0x8c8e4 | load | 4 | `abs 0x8d1e4` | `-` |
| `ldr` at 0x8c8e8 | load | 4 | `abs 0x1497e8` | `-` |
| `ldr` at 0x8c8f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c8f8 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8c908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c90c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0x8c910 | store | 4 | `sp -0x5ac` | `[[sp-24]+48]` |
| `ldr` at 0x8c918 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c924 | store | 4 | `sp -0x5ac` | `ret(0xb7944)` |
| `ldr` at 0x8c928 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8c92c | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8c93c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c940 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0x8c944 | store | 4 | `sp -0x5b0` | `[[sp-24]+56]` |
| `ldr` at 0x8c94c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c958 | store | 4 | `sp -0x5b0` | `ret(0xb7944)` |
| `ldr` at 0x8c95c | load | 4 | `abs 0x8d1e4` | `-` |
| `ldr` at 0x8c960 | load | 4 | `abs 0x1497e8` | `-` |
| `ldr` at 0x8c968 | load | 4 | `sp -0x5ac` | `-` |
| `ldr` at 0x8c96c | load | 4 | `sp -0x5b0` | `-` |
| `str` at 0x8c980 | store | 4 | `sp -0x2f4` | `ret(unknown)` |
| `ldr` at 0x8c984 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c9b8 | load | 4 | `sp -0x2f4` | `-` |
| `ldr` at 0x8c9ec | load | 4 | `sp -0x2f4` | `-` |
| `str` at 0x8ca38 | store | 4 | `sp -0x2a0` | `ret(0x4dbcc)` |
| `ldr` at 0x8ca4c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8ca58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ca5c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0x8ca84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8ca88 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0x8ca98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ca9c | load | 4 | `[sp-24] +0x38` | `-` |
| `ldr` at 0x8caa0 | load | 4 | `abs 0x8d1e8` | `-` |
| `str` at 0x8caac | store | 4 | `sp -0x5b4` | `?` |
| `ldr` at 0x8cab0 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8cab8 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8cac4 | load | 4 | `sp -0x5b4` | `-` |
| `str` at 0x8cad0 | store | 4 | `sp -0x5b4` | `?` |
| `ldr` at 0x8cad8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cae4 | store | 4 | `sp -0x5b4` | `ret(0xb7944)` |
| `ldr` at 0x8cae8 | load | 4 | `abs 0x8d1e8` | `-` |
| `ldr` at 0x8caec | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8cb0c | load | 4 | `sp -0x5b4` | `-` |
| `str` at 0x8cb14 | store | 4 | `sp -0x5b4` | `?` |
| `ldr` at 0x8cb18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cb1c | load | 1 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x8cb2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cb30 | load | 4 | `[sp-24] +0x40` | `-` |
| `str` at 0x8cb38 | store | 4 | `sp -0x5b8` | `?` |
| `ldr` at 0x8cb40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cb4c | store | 4 | `sp -0x5b8` | `ret(0xb7944)` |
| `ldr` at 0x8cb50 | load | 4 | `sp -0x5b8` | `-` |
| `str` at 0x8cb58 | store | 4 | `sp -0x5b8` | `?` |
| `ldr` at 0x8cb5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cb60 | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0x8cb70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cb74 | load | 4 | `[sp-24] +0x48` | `-` |
| `ldr` at 0x8cb78 | load | 4 | `abs 0x8d1ec` | `-` |
| `str` at 0x8cb84 | store | 4 | `sp -0x5bc` | `?` |
| `ldr` at 0x8cb88 | load | 4 | `sp -0x5bc` | `-` |
| `ldr` at 0x8cb90 | load | 4 | `sp -0x5bc` | `-` |
| `str` at 0x8cba0 | store | 4 | `sp -0x5bc` | `?` |
| `ldr` at 0x8cba8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cbb4 | store | 4 | `sp -0x5bc` | `ret(0xb7944)` |
| `ldr` at 0x8cbb8 | load | 4 | `abs 0x8d1ec` | `-` |
| `ldr` at 0x8cbbc | load | 4 | `sp -0x5bc` | `-` |
| `ldr` at 0x8cbd8 | load | 4 | `sp -0x5bc` | `-` |
| `str` at 0x8cbe0 | store | 4 | `sp -0x5bc` | `?` |
| `ldr` at 0x8cbe4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cbe8 | load | 1 | `[sp-24] +0x4c` | `-` |
| `ldr` at 0x8cbf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cbfc | load | 4 | `[sp-24] +0x50` | `-` |
| `ldr` at 0x8cc00 | load | 4 | `abs 0x8d1f0` | `-` |
| `str` at 0x8cc0c | store | 4 | `sp -0x5c0` | `?` |
| `ldr` at 0x8cc10 | load | 4 | `sp -0x5c0` | `-` |
| `ldr` at 0x8cc18 | load | 4 | `sp -0x5c0` | `-` |
| `str` at 0x8cc28 | store | 4 | `sp -0x5c0` | `?` |
| `ldr` at 0x8cc30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cc3c | store | 4 | `sp -0x5c0` | `ret(0xb7944)` |
| `ldr` at 0x8cc40 | load | 4 | `abs 0x8d1f0` | `-` |
| `ldr` at 0x8cc44 | load | 4 | `sp -0x5c0` | `-` |
| `ldr` at 0x8cc60 | load | 4 | `sp -0x5c0` | `-` |
| `str` at 0x8cc68 | store | 4 | `sp -0x5c0` | `?` |
| `ldr` at 0x8cc6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cc70 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0x8cc80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cc84 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0x8cc88 | load | 4 | `abs 0x8d1f4` | `-` |
| `str` at 0x8cc94 | store | 4 | `sp -0x5c4` | `?` |
| `ldr` at 0x8cc98 | load | 4 | `sp -0x5c4` | `-` |
| `ldr` at 0x8cca0 | load | 4 | `sp -0x5c4` | `-` |
| `ldr` at 0x8ccac | load | 4 | `sp -0x5c4` | `-` |
| `str` at 0x8ccc4 | store | 4 | `sp -0x5c4` | `?` |
| `ldr` at 0x8cccc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ccd8 | store | 4 | `sp -0x5c4` | `ret(0xb7944)` |
| `ldr` at 0x8ccdc | load | 4 | `abs 0x8d1f4` | `-` |
| `ldr` at 0x8cce0 | load | 4 | `sp -0x5c4` | `-` |
| `ldr` at 0x8cd0c | load | 4 | `sp -0x5c4` | `-` |
| `str` at 0x8cd14 | store | 4 | `sp -0x5c4` | `?` |
| `ldr` at 0x8cd18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cd1c | load | 1 | `[sp-24] +0x54` | `-` |
| `ldr` at 0x8cd2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cd30 | load | 4 | `[sp-24] +0x58` | `-` |
| `ldr` at 0x8cd34 | load | 4 | `abs 0x8d1f0` | `-` |
| `str` at 0x8cd40 | store | 4 | `sp -0x5c8` | `?` |
| `ldr` at 0x8cd44 | load | 4 | `sp -0x5c8` | `-` |
| `ldr` at 0x8cd4c | load | 4 | `sp -0x5c8` | `-` |
| `str` at 0x8cd5c | store | 4 | `sp -0x5c8` | `?` |
| `ldr` at 0x8cd64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cd70 | store | 4 | `sp -0x5c8` | `ret(0xb7944)` |
| `ldr` at 0x8cd74 | load | 4 | `abs 0x8d1f0` | `-` |
| `ldr` at 0x8cd78 | load | 4 | `sp -0x5c8` | `-` |
| `ldr` at 0x8cd94 | load | 4 | `sp -0x5c8` | `-` |
| `str` at 0x8cd9c | store | 4 | `sp -0x5c8` | `?` |
| `ldr` at 0x8cda8 | load | 4 | `sp -0x5bc` | `-` |
| `str` at 0x8cdac | store | 4 | `sp -0x5dc` | `[sp-1468]` |
| `ldr` at 0x8cdb0 | load | 4 | `sp -0x5c0` | `-` |
| `str` at 0x8cdb4 | store | 4 | `sp -0x5d8` | `[sp-1472]` |
| `ldr` at 0x8cdb8 | load | 4 | `sp -0x5c4` | `-` |
| `str` at 0x8cdbc | store | 4 | `sp -0x5d4` | `[sp-1476]` |
| `ldr` at 0x8cdc0 | load | 4 | `sp -0x5c8` | `-` |
| `str` at 0x8cdc4 | store | 4 | `sp -0x5d0` | `[sp-1480]` |
| `ldr` at 0x8cdcc | load | 4 | `abs 0x8d1f8` | `-` |
| `ldr` at 0x8cdd0 | load | 4 | `sp -0x5b4` | `-` |
| `ldr` at 0x8cdd4 | load | 4 | `sp -0x5b8` | `-` |
| `ldr` at 0x8ce00 | load | 4 | `abs 0x8d1fc` | `-` |
| `str` at 0x8ce4c | store | 4 | `sp -0x2f4` | `ret(0x1b1e4)` |
| `str` at 0x8ce60 | store | 4 | `sp -0x2e0` | `ret(0x1b3dc)` |
| `ldr` at 0x8ce6c | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ce70 | load | 4 | `ret(0x1b3dc) +0x14` | `-` |
| `ldr` at 0x8ce90 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ce94 | load | 4 | `[sp-736] +0x10` | `-` |
| `ldr` at 0x8ceb0 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8cebc | load | 4 | `[sp-736] +0xc` | `-` |
| `ldr` at 0x8cecc | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8ced8 | load | 4 | `[sp-736] +0x8` | `-` |
| `ldr` at 0x8cee8 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8cef4 | load | 4 | `[sp-736] +0x4` | `-` |
| `ldr` at 0x8cf04 | load | 4 | `sp -0x2e0` | `-` |
| `ldr` at 0x8cf10 | load | 4 | `[sp-736] +0x0` | `-` |
| `ldr` at 0x8cf1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8cf20 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldrb` at 0x8cf50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8cf54 | load | 4 | `abs 0x8d200` | `-` |
| `ldr` at 0x8cf58 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0x8cf64 | load | 4 | `abs 0x8d200` | `-` |
| `ldrb` at 0x8cf70 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8cf98 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8cfa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8cfac | load | 4 | `abs 0x8d200` | `-` |
| `ldrb` at 0x8cfb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8cfe0 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8cfe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d01c | load | 4 | `abs 0x8d200` | `-` |
| `ldrb` at 0x8d028 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d050 | load | 4 | `abs 0x2b1734` | `-` |
| `ldr` at 0x8d058 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d068 | store | 4 | `sp -0x2a0` | `ret(0x54e58)` |
| `ldr` at 0x8d06c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d0a0 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d0d4 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d0e0 | load | 4 | `abs 0x8d204` | `-` |
| `ldr` at 0x8d0e4 | load | 4 | `abs 0x1497d0` | `-` |
| `ldr` at 0x8d0f0 | load | 4 | `abs 0x8d204` | `-` |
| `ldr` at 0x8d0fc | load | 4 | `abs 0x1497d0` | `-` |
| `ldr` at 0x8d100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d130 | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x8d140 | store | 4 | `sp -0x5dc` | `sp-664` |
| `ldr` at 0x8d144 | load | 4 | `abs 0x8d208` | `-` |
| `ldr` at 0x8d14c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d150 | load | 4 | `sp -0x29c` | `-` |
| `str` at 0x8d15c | store | 4 | `sp -0x2a0` | `ret(0xb6bdc)` |
| `ldr` at 0x8d160 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d16c | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d178 | load | 4 | `abs 0x8d20c` | `-` |
| `str` at 0x8d17c | store | 4 | `sp -0x5dc` | `0x112470` |
| `str` at 0x8d180 | store | 4 | `sp -0x5d8` | `ret(0x21674)` |
| `ldr` at 0x8d188 | load | 4 | `abs 0x8d210` | `-` |
| `ldr` at 0x8d18c | load | 4 | `abs 0x8d214` | `-` |
| `ldr` at 0x8d198 | load | 4 | `abs 0x8d218` | `-` |
| `str` at 0x8d1a4 | store | 4 | `sp -0x2a0` | `ret(0x22dc0)` |
| `ldr` at 0x8d1a8 | load | 4 | `sp -0x2a0` | `-` |
| `ldr` at 0x8d1b4 | load | 4 | `abs 0x8d21c` | `-` |
| `str` at 0x8d1b8 | store | 4 | `sp -0x5dc` | `0x11248c` |
| `ldr` at 0x8d1bc | load | 4 | `sp -0x2a0` | `-` |
| `str` at 0x8d1c0 | store | 4 | `sp -0x5d8` | `ret(0x22dc0)` |
| `ldr` at 0x8d1c8 | load | 4 | `abs 0x8d210` | `-` |
| `ldr` at 0x8d1cc | load | 4 | `abs 0x8d220` | `-` |
| `ldr` at 0x8d1d8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8d1e0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8d1e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8d1e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8d1e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x882c8 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x882e0 | `(unknown callee) 0x22a38` | `0x2b17c0` | `?` | `?` | `?` |
| `bl` at 0x88318 | `(unknown callee) 0x21e28` | `0x9` | `0x1120a0` | `0x1f1` | `0x0` |
| `bl` at 0x88340 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x88374 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x883c8 | `(unknown callee) 0x83734` | `[sp-1208]` | `?` | `?` | `?` |
| `bl` at 0x883f0 | `(unknown callee) 0x5ea88` | `?` | `[sp-1204]` | `sp-20` | `?` |
| `bl` at 0x8841c | `(unknown callee) 0x2070c` | `0x11210c` | `[[sp-24]+28]` | `[sp-680]` | `[sp-24]` |
| `bl` at 0x88434 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x887ec | `(unknown callee) 0x1f1dc` | `0x3` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x8880c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x88824 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88830 | `(unknown callee) 0x1f1dc` | `0x1` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x88840 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x88864 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8887c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88924 | `(unknown callee) 0x2070c` | `0x112120` | `?` | `[sp-708]` | `?` |
| `bl` at 0x8893c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x88958 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x88978 | `(unknown callee) 0x74908` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x889f4 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88a0c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x88a14 | `(unknown callee) 0x74908` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x88a90 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88aa8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x88ab0 | `(unknown callee) 0x74908` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x88b2c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88b44 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x88b4c | `(unknown callee) 0x74908` | `0x4` | `?` | `?` | `?` |
| `bl` at 0x88bc8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88be0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-708]` | `sp-668` |
| `bl` at 0x88bfc | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x88c04 | `(unknown callee) 0x74908` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x88c40 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88c5c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x88c64 | `(unknown callee) 0x74908` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x88ca0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x88cbc | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[[sp-692]+16]` | `sp-668` |
| `bl` at 0x88ce0 | `(unknown callee) 0x4f148` | `?` | `sp-20` | `[sp-680]` | `?` |
| `bl` at 0x88cec | `(unknown callee) 0x4f2f0` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-676]` |
| `bl` at 0x88d14 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x88d30 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x88d38 | `(unknown callee) 0x1b22c` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x88d60 | `(unknown callee) 0x692f8` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x1b22c)` |
| `bl` at 0x88d7c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x88d90 | `(unknown callee) 0x1b430` | `0x112140` | `sp-700` | `0xa` | `sp-700` |
| `bl` at 0x88dac | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `ret(0x1b430)` | `sp-668` |
| `bl` at 0x88dc8 | `(unknown callee) 0x1b430` | `[sp-700]+1` | `sp-700` | `0xa` | `[sp-700]+1` |
| `bl` at 0x88de4 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `ret(0x1b430)` | `sp-668` |
| `bl` at 0x88dfc | `(unknown callee) 0x1b430` | `[sp-700]+1` | `0x0` | `0xa` | `[sp-700]+1` |
| `bl` at 0x88e18 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `ret(0x1b430)` | `sp-668` |
| `bl` at 0x88e44 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x88e78 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x88ec0 | `(unknown callee) 0x87824` | `[sp-1236]` | `[sp-1232]` | `?` | `?` |
| `bl` at 0x88ed8 | `(unknown callee) 0xd1dfc` | `0x0` | `sp-712` | `?` | `sp-712` |
| `bl` at 0x88ef8 | `(unknown callee) 0xd1e50` | `0x0` | `[sp-716]:1` | `?` | `[sp-716]:1` |
| `bl` at 0x88f0c | `(unknown callee) 0x20240` | `0x112148` | `?` | `?` | `ret(0xd1e50)` |
| `bl` at 0x88f24 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x88f50 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x88f6c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x88fa0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89014 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89048 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8907c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89094 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `0x8` | `sp-668` |
| `bl` at 0x890c8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x890fc | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89130 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89164 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8919c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x891d0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x891fc | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89230 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x89278 | `(unknown callee) 0x87824` | `[sp-1248]` | `[sp-1244]` | `?` | `?` |
| `bl` at 0x89354 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x89390 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x893cc | `(unknown callee) 0xb7944` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x893f8 | `(unknown callee) 0xd1dfc` | `0x0` | `sp-720` | `[sp-1260]` | `sp-720` |
| `bl` at 0x89418 | `(unknown callee) 0xd1e50` | `0x0` | `[sp-716]:1` | `?` | `[sp-716]:1` |
| `bl` at 0x8943c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89468 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x898b0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x898e4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8994c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89964 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x89998 | `(unknown callee) 0x6917c` | `sp-668` | `sp-664` | `([sp-736] + [sp-728])` | `[sp-732]` |
| `mov` at 0x899c0 | `(unknown callee) (unknown target)` | `0x0` | `[sp-680]` | `[sp-680]` | `[0x2ae60c]` |
| `bl` at 0x899d4 | `(unknown callee) 0x1f1dc` | `0xd` | `[sp-680]` | `[sp-680]` | `[0x2ae60c]` |
| `bl` at 0x899e0 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0x89a04 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x89a20 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89a28 | `(unknown callee) 0x1b22c` | `[sp-700]` | `?` | `?` | `?` |
| `bl` at 0x89a50 | `(unknown callee) 0x692f8` | `sp-668` | `sp-664` | `[sp-700]` | `ret(0x1b22c)` |
| `bl` at 0x89a5c | `(unknown callee) 0x1f1dc` | `0xe` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x89a7c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89a94 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x89aa0 | `(unknown callee) 0x1f1dc` | `0x12` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x89ac0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89ad8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x89af4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89d54 | `(unknown callee) 0x1f1dc` | `0x2a` | `?` | `?` | `[sp-676]-48` |
| `bl` at 0x89d74 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `ret(0x1f1dc)` | `sp-668` |
| `bl` at 0x89d7c | `(unknown callee) 0x1f1dc` | `0x2b` | `?` | `?` | `?` |
| `bl` at 0x89d9c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `ret(0x1f1dc)` | `sp-668` |
| `bl` at 0x89dc4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89dec | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89e1c | `(unknown callee) 0x2070c` | `0x11215c` | `?` | `?` | `?` |
| `bl` at 0x89e38 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89e58 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89e98 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89ec0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89ed8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `0x800` | `sp-668` |
| `bl` at 0x89f08 | `(unknown callee) 0x2070c` | `0x112174` | `?` | `?` | `?` |
| `bl` at 0x89f24 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89f44 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x89f5c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `0x200` | `sp-668` |
| `bl` at 0x89f78 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89f90 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89fa8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-676]` | `sp-668` |
| `bl` at 0x89fc0 | `(unknown callee) 0x4e60c` | `sp-744` | `sp-740` | `sp-740` | `sp-744` |
| `bl` at 0x89fd8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x89ff0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-744]` | `sp-668` |
| `bl` at 0x8a008 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-740]` | `sp-668` |
| `bl` at 0x8a010 | `(unknown callee) 0x5d81c` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x8a03c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a07c | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0x8a09c | `(unknown callee) 0x1b43c` | `ret(0x1b124)` | `0x0` | `?` | `?` |
| `bl` at 0x8a0ac | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0x8a0cc | `(unknown callee) 0x1b43c` | `ret(0x1b124)` | `0x0` | `?` | `?` |
| `bl` at 0x8a108 | `(unknown callee) 0xfa70c` | `[sp-736]` | `[sp-752]` | `0x4` | `?` |
| `bl` at 0x8a120 | `(unknown callee) 0xfa87c` | `[sp-736]` | `[sp-752]` | `?` | `[sp-736]` |
| `bl` at 0x8a148 | `(unknown callee) 0x5d83c` | `[sp-736]:1` | `?` | `ret(0xfa87c)` | `[sp-736]:1` |
| `bl` at 0x8a1a0 | `(unknown callee) 0x5d8c0` | `[sp-736]:1` | `sp-20` | `?` | `[sp-736]:1` |
| `bl` at 0x8a20c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a224 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-724]` | `sp-668` |
| `bl` at 0x8a23c | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-732]` | `sp-668` |
| `bl` at 0x8a280 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8a2a8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8a2c0 | `(unknown callee) 0x1b400` | `[sp-720]` | `?` | `?` | `[sp-724]` |
| `bl` at 0x8a2c8 | `(unknown callee) 0x1b400` | `[sp-716]` | `?` | `?` | `?` |
| `bl` at 0x8a2e8 | `(unknown callee) 0x1b43c` | `sp-868` | `0x0` | `0x40` | `sp-868` |
| `bl` at 0x8a2f8 | `(unknown callee) 0xdc2f0` | `0x0` | `sp-792` | `?` | `sp-792` |
| `bl` at 0x8a42c | `(unknown callee) 0xe3430` | `sp-792` | `?` | `?` | `sp-792` |
| `bl` at 0x8a50c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a524 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8a570 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a590 | `(unknown callee) 0x6917c` | `sp-668` | `sp-664` | `sp-868` | `0x40` |
| `bl` at 0x8a5bc | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a5f0 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8a600 | `(unknown callee) 0x1d38c` | `0x3` | `[sp-1272]` | `?` | `?` |
| `bl` at 0x8a618 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a644 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a6b8 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8a6d0 | `(unknown callee) 0x67d30` | `[sp-1280]` | `?` | `?` | `?` |
| `bl` at 0x8a6e8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a6f4 | `(unknown callee) 0x74908` | `0x0` | `[sp-680]` | `[sp-680]` | `0x1003` |
| `bl` at 0x8a718 | `(unknown callee) 0x67ee8` | `[ret(0x74908)+8]` | `[ret(0x74908)+12]` | `[ret(0x74908)+20]` | `ret(0x74908)` |
| `bl` at 0x8a730 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a75c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a78c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a7c0 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8a7d4 | `(unknown callee) 0x51580` | `0x32` | `[sp-1284]` | `0x0` | `?` |
| `bl` at 0x8a7ec | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a818 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a84c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8a85c | `(unknown callee) 0x1d38c` | `0xe` | `[sp-1288]` | `?` | `?` |
| `bl` at 0x8a864 | `(unknown callee) 0x1f1dc` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x8a870 | `(unknown callee) 0x75014` | `ret(0x1f1dc)` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x8a888 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a8b4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a8e8 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8a8f8 | `(unknown callee) 0x1d38c` | `0x12` | `[sp-1292]` | `?` | `?` |
| `bl` at 0x8a910 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a93c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8a978 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `0x2b1720` | `?` |
| `bl` at 0x8a994 | `(unknown callee) 0x837a4` | `?` | `[sp-1300]` | `?` | `?` |
| `bl` at 0x8a9ac | `(unknown callee) 0x1d38c` | `0x13` | `[0x2b172c]` | `?` | `0x2b1720` |
| `bl` at 0x8a9c4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8a9f0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8aa28 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8aa90 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8aaa0 | `(unknown callee) 0x52770` | `[sp-1304]` | `0x0` | `?` | `?` |
| `bl` at 0x8aad4 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8aae0 | `(unknown callee) 0x527dc` | `[sp-1308]` | `?` | `?` | `?` |
| `bl` at 0x8ab14 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8ab4c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x8ab6c | `(unknown callee) 0x52830` | `[sp-1312]` | `[sp-1316]` | `0x0` | `?` |
| `bl` at 0x8ab88 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8aba0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8aba8 | `(unknown callee) 0x85a0c` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `0x100a` |
| `bl` at 0x8abd4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ac18 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8ac78 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x8acd0 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8ad08 | `(unknown callee) 0x5e8d4` | `?` | `[sp-688]` | `?` | `?` |
| `bl` at 0x8ad44 | `(unknown callee) 0x2070c` | `0x11218c` | `?` | `?` | `?` |
| `bl` at 0x8ad60 | `(unknown callee) 0x83734` | `[0x2b172c]` | `?` | `?` | `0x2b1720` |
| `bl` at 0x8ad7c | `(unknown callee) 0x85a0c` | `sp-20` | `?` | `ret(0x83734)` | `?` |
| `bl` at 0x8ada8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ae04 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8ae48 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8ae60 | `(unknown callee) 0x5dcac` | `[sp-1340]` | `?` | `?` | `?` |
| `bl` at 0x8ae6c | `(unknown callee) 0x4e10c` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2001` |
| `bl` at 0x8ae94 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8aeac | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8aee8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8af1c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8af58 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8af80 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[0x1497d4]` |
| `bl` at 0x8afa8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8afc0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8affc | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b030 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8b06c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8b094 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[0x1497d8]` |
| `bl` at 0x8b0bc | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b0d4 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8b110 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b144 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8b180 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8b1a8 | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[0x1497dc]` |
| `bl` at 0x8b1d0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b1e8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8b1f4 | `(unknown callee) 0x3d48c` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2005` |
| `bl` at 0x8b21c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b234 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8b240 | `(unknown callee) 0x3d858` | `0x3` | `[sp-680]` | `[sp-680]` | `0x2006` |
| `bl` at 0x8b268 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b280 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8b294 | `(unknown callee) 0xd1dfc` | `0x0` | `sp-760` | `[sp-680]` | `sp-760` |
| `bl` at 0x8b2c0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8b2dc | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8b3a8 | `(unknown callee) 0xd2280` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8b3c4 | `(unknown callee) 0x1b49c` | `sp-996` | `0x1121a8` | `[sp-736]` | `ret(0xd2280)` |
| `bl` at 0x8b3d4 | `(unknown callee) 0x1b22c` | `sp-996` | `?` | `?` | `sp-996` |
| `bl` at 0x8b410 | `(unknown callee) 0xd1e50` | `0x0` | `[sp-736]:1` | `sp-20` | `[sp-736]:1` |
| `bl` at 0x8b42c | `(unknown callee) 0x1b2d4` | `sp-996` | `0x1121b0` | `?` | `sp-996` |
| `bl` at 0x8b44c | `(unknown callee) 0xd22e8` | `[[sp-760]+28]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8b460 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x1121c4` | `ret(0xd22e8)` | `ret(0xd22e8)` |
| `bl` at 0x8b47c | `(unknown callee) 0xd2280` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8b488 | `(unknown callee) 0x8d224` | `ret(0xd2280)` | `?` | `?` | `ret(0xd2280)` |
| `bl` at 0x8b514 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x1121e8` | `[sp-1372]` | `[[sp-760]+76]` |
| `bl` at 0x8b540 | `(unknown callee) 0xd1e50` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8b55c | `(unknown callee) 0x1b2d4` | `sp-996` | `0x112200` | `?` | `sp-996` |
| `bl` at 0x8b57c | `(unknown callee) 0xd22e8` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8b590 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x112214` | `ret(0xd22e8)` | `ret(0xd22e8)` |
| `bl` at 0x8b5a0 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x112224` | `?` | `[[sp-760]+4]` |
| `bl` at 0x8b5a8 | `(unknown callee) 0x1b22c` | `[sp-720]` | `?` | `?` | `?` |
| `bl` at 0x8b5f8 | `(unknown callee) 0x1b49c` | `?` | `0x112230` | `?` | `?` |
| `bl` at 0x8b650 | `(unknown callee) 0x1b49c` | `?` | `0x112230` | `?` | `?` |
| `bl` at 0x8b730 | `(unknown callee) 0x1afbc` | `[[[sp-760]+12]+91]:1` | `?` | `0x5b` | `[[[sp-760]+12]+91]:1` |
| `bl` at 0x8b758 | `(unknown callee) 0x1afbc` | `[[[sp-760]+12]+92]:1` | `?` | `0x5c` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x8b79c | `(unknown callee) 0x1b49c` | `?` | `0x112238` | `[[[sp-760]+12]+91]:1` | `[[[sp-760]+12]+92]:1` |
| `bl` at 0x8b864 | `(unknown callee) 0x1b49c` | `?` | `0x112244` | `?` | `?` |
| `bl` at 0x8b86c | `(unknown callee) 0x1b22c` | `[sp-720]` | `?` | `?` | `?` |
| `mov` at 0x8b8b0 | `(unknown callee) (unknown target)` | `[sp-760]` | `0x0` | `0x0` | `([sp-720] + ret(0x1b22c))` |
| `bl` at 0x8b8c0 | `(unknown callee) 0x1b508` | `([sp-720] + ret(0x1b22c))` | `0x112250` | `ret(0x1b22c)` | `[0x1497e8]` |
| `bl` at 0x8b8ec | `(unknown callee) 0xd1e50` | `0x0` | `[sp-736]:1` | `?` | `[sp-736]:1` |
| `bl` at 0x8b908 | `(unknown callee) 0x1b2d4` | `sp-996` | `0x112254` | `?` | `sp-996` |
| `bl` at 0x8b928 | `(unknown callee) 0xd22e8` | `[[sp-760]+4]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8b93c | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x112268` | `ret(0xd22e8)` | `ret(0xd22e8)` |
| `bl` at 0x8b95c | `(unknown callee) 0xd22e8` | `[[sp-760]+32]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8b970 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x112278` | `ret(0xd22e8)` | `ret(0xd22e8)` |
| `bl` at 0x8b990 | `(unknown callee) 0xd22e8` | `[[sp-760]+44]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8b9a4 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x11229c` | `ret(0xd22e8)` | `ret(0xd22e8)` |
| `bl` at 0x8b9c4 | `(unknown callee) 0xd22e8` | `[[sp-760]+64]` | `?` | `?` | `[sp-760]` |
| `bl` at 0x8b9d8 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x1122c0` | `ret(0xd22e8)` | `ret(0xd22e8)` |
| `bl` at 0x8b9e8 | `(unknown callee) 0x1b49c` | `[sp-720]` | `0x1122e8` | `?` | `[[sp-760]+64]` |
| `bl` at 0x8b9f0 | `(unknown callee) 0x1b22c` | `[sp-720]` | `?` | `?` | `?` |
| `bl` at 0x8ba4c | `(unknown callee) 0x1afbc` | `?` | `?` | `[[sp-760]+68]` | `?` |
| `bl` at 0x8bb04 | `(unknown callee) 0x1b49c` | `?` | `0x1122f4` | `?` | `?` |
| `bl` at 0x8bb68 | `(unknown callee) 0x1b49c` | `?` | `0x1122fc` | `[[[sp-760]+36]+3]:1` | `[[sp-760]+40]` |
| `bl` at 0x8bb7c | `(unknown callee) 0x1b2d4` | `sp-996` | `0x112310` | `?` | `sp-996` |
| `bl` at 0x8bb9c | `(unknown callee) 0x1b22c` | `sp-996` | `?` | `?` | `sp-996` |
| `bl` at 0x8bbc8 | `(unknown callee) 0x692f8` | `sp-668` | `sp-664` | `sp-996` | `ret(0x1b22c)` |
| `bl` at 0x8bbec | `(unknown callee) 0x20240` | `0x112318` | `[sp-680]` | `[sp-680]` | `0x4002` |
| `bl` at 0x8bbfc | `(unknown callee) 0x20cdc` | `0x1` | `[0x2b36b0]` | `?` | `0x2b36a0` |
| `bl` at 0x8bc2c | `(unknown callee) 0x1b43c` | `sp-1195` | `0x0` | `0xc7` | `sp-1195` |
| `bl` at 0x8bc5c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8bc98 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0x8bcf8 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x3` | `[sp-1388]` | `?` |
| `bl` at 0x8bd2c | `(unknown callee) 0x39684` | `?` | `[sp-1392]` | `sp-1196` | `0x0` |
| `bl` at 0x8bd58 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8bd7c | `(unknown callee) 0x1b22c` | `sp-1196` | `?` | `?` | `sp-1196` |
| `bl` at 0x8bdac | `(unknown callee) 0x692f8` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b22c)` |
| `bl` at 0x8bdc8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8bde0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-736]` | `sp-668` |
| `bl` at 0x8be04 | `(unknown callee) 0x1b22c` | `sp-1196` | `sp-20` | `?` | `sp-1196` |
| `bl` at 0x8be34 | `(unknown callee) 0x692f8` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b22c)` |
| `bl` at 0x8be68 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x3` | `[sp-1388]` | `?` |
| `bl` at 0x8bea4 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x4` | `[sp-1396]` | `?` |
| `bl` at 0x8bee0 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x8bf20 | `(unknown callee) 0x3987c` | `?` | `[sp-724]` | `[sp-720]` | `[sp-1404]` |
| `bl` at 0x8bf4c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8bf70 | `(unknown callee) 0x1b22c` | `sp-1196` | `sp-20` | `?` | `sp-1196` |
| `bl` at 0x8bfa0 | `(unknown callee) 0x692f8` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b22c)` |
| `bl` at 0x8bfbc | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8bfd4 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-736]` | `sp-668` |
| `bl` at 0x8bff8 | `(unknown callee) 0x1b22c` | `sp-1196` | `?` | `sp-20` | `sp-1196` |
| `bl` at 0x8c028 | `(unknown callee) 0x692f8` | `sp-668` | `sp-664` | `sp-1196` | `ret(0x1b22c)` |
| `bl` at 0x8c034 | `(unknown callee) 0x1f1dc` | `0x7e` | `[sp-680]` | `[sp-680]` | `[sp-680]` |
| `bl` at 0x8c054 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c05c | `(unknown callee) 0x1f1dc` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x8c080 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `0xffff` | `sp-668` |
| `bl` at 0x8c08c | `(unknown callee) 0x1f1dc` | `0x80` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x8c0b0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `0xffff` | `sp-668` |
| `bl` at 0x8c0cc | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-696]` | `sp-668` |
| `bl` at 0x8c0f8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c100 | `(unknown callee) 0xdadb8` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[[sp-24]+32]:1` |
| `bl` at 0x8c13c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8c168 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c19c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8c1a8 | `(unknown callee) 0xdaec8` | `[sp-1416]` | `?` | `?` | `?` |
| `bl` at 0x8c1d4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c1ec | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8c208 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c234 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c23c | `(unknown callee) 0xdadb8` | `[sp-680]` | `[sp-680]` | `[sp-680]` | `[[sp-24]+32]:1` |
| `bl` at 0x8c278 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8c2a4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c2d8 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8c2f0 | `(unknown callee) 0xdaf80` | `[sp-1424]` | `sp-745` | `?` | `sp-745` |
| `bl` at 0x8c31c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c334 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8c350 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c368 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c3ac | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8c3e8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c404 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c480 | `(unknown callee) 0xb7944` | `[sp-24]` | `?` | `0x34` | `?` |
| `bl` at 0x8c4a4 | `(unknown callee) 0xd6b9c` | `0x0` | `[sp-1432]` | `sp-745` | `sp-745` |
| `bl` at 0x8c4d0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c4e8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8c500 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8c524 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `?` | `?` |
| `bl` at 0x8c55c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c590 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8c5d8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c5f4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c694 | `(unknown callee) 0xb7944` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x8c724 | `(unknown callee) 0xb7944` | `[sp-24]` | `?` | `[sp-756]` | `?` |
| `bl` at 0x8c7b4 | `(unknown callee) 0xb7944` | `[sp-24]` | `?` | `[sp-756]` | `?` |
| `bl` at 0x8c7d4 | `(unknown callee) 0xd6a1c` | `0x0` | `[sp-1440]` | `[sp-1444]` | `[sp-1448]` |
| `bl` at 0x8c800 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c818 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8c830 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8c86c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c920 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `[sp-680]` | `?` |
| `bl` at 0x8c954 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `?` | `?` |
| `mov` at 0x8c978 | `(unknown callee) (unknown target)` | `0x0` | `[sp-1452]` | `[sp-1456]` | `0x0` |
| `bl` at 0x8c9a4 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8c9bc | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8c9d8 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8c9f0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-756]` | `sp-668` |
| `bl` at 0x8ca0c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8ca28 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8ca30 | `(unknown callee) 0x4dbcc` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x8ca50 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `ret(0x4dbcc)` | `sp-668` |
| `bl` at 0x8ca7c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8cae0 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x2` | `[sp-680]` | `?` |
| `bl` at 0x8cb48 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x3` | `?` | `?` |
| `bl` at 0x8cbb0 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x4` | `?` | `?` |
| `bl` at 0x8cc38 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x5` | `?` | `?` |
| `bl` at 0x8ccd4 | `(unknown callee) 0xb7944` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0x8cd6c | `(unknown callee) 0xb7944` | `[sp-24]` | `0x6` | `?` | `?` |
| `bl` at 0x8cdd8 | `(unknown callee) 0x1b49c` | `sp-1196` | `0x1123fc` | `[sp-1460]` | `[sp-1464]` |
| `bl` at 0x8cde8 | `(unknown callee) 0x1b0dc` | `sp-1196` | `?` | `?` | `sp-1196` |
| `bl` at 0x8ce08 | `(unknown callee) 0x2070c` | `0x11244c` | `sp-1196` | `?` | `sp-1196` |
| `bl` at 0x8ce20 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8ce3c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8ce44 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x8ce58 | `(unknown callee) 0x1b3dc` | `sp-756` | `?` | `?` | `sp-756` |
| `bl` at 0x8ce84 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[ret(0x1b3dc)+20]+1900` | `[ret(0x1b3dc)+20]+1900` |
| `bl` at 0x8cea4 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[[sp-736]+16]+1` | `[[sp-736]+16]+1` |
| `bl` at 0x8cec0 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[[sp-736]+12]` | `sp-668` |
| `bl` at 0x8cedc | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[[sp-736]+8]` | `sp-668` |
| `bl` at 0x8cef8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[[sp-736]+4]` | `sp-668` |
| `bl` at 0x8cf14 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[[sp-736]]` | `sp-668` |
| `bl` at 0x8cf40 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d014 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d060 | `(unknown callee) 0x54e58` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x8d08c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d0a4 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `bl` at 0x8d0c0 | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `0x0` | `sp-668` |
| `bl` at 0x8d0d8 | `(unknown callee) 0x690d8` | `sp-668` | `sp-664` | `[sp-672]` | `sp-668` |
| `mov` at 0x8d110 | `(unknown callee) (unknown target)` | `[sp-24]` | `sp-668` | `sp-664` | `[0x1497d0]` |
| `bl` at 0x8d12c | `(unknown callee) 0x6907c` | `sp-668` | `sp-664` | `?` | `sp-668` |
| `bl` at 0x8d154 | `(unknown callee) 0xb6bdc` | `0x2b1814` | `0x0` | `[sp-24]` | `[sp-668]` |
| `bl` at 0x8d170 | `(unknown callee) 0x21674` | `ret(0xb6bdc)` | `?` | `?` | `ret(0xb6bdc)` |
| `bl` at 0x8d194 | `(unknown callee) 0x21e28` | `0x6` | `0x1120a0` | `0x752` | `0x0` |
| `bl` at 0x8d19c | `(unknown callee) 0x22dc0` | `0x2b17c0` | `?` | `?` | `?` |
| `bl` at 0x8d1d4 | `(unknown callee) 0x21e28` | `0x9` | `0x1120a0` | `0x75b` | `0x0` |
| `bl` at 0x8d1dc | `(unknown callee) 0xb56f4` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x882f4 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `bne` at 0x88328 | `[[sp-24]+32]:1 != 0x0` | `[[sp-24]+32]:1 == 0x0` |
| `bne` at 0x88358 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x88398 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `beq` at 0x88408 | `[0x148514] == 0x0` | `[0x148514] != 0x0` |
| `beq` at 0x8844c | `[sp-680] == 0x1006` | `[sp-680] != 0x1006` |
| `bhi` at 0x8845c | `[sp-680] > 0x1006 (unsigned, 32-bit)` | `[sp-680] <= 0x1006 (unsigned, 32-bit)` |
| `beq` at 0x88468 | `[sp-680] == 0xc` | `[sp-680] != 0xc` |
| `bhi` at 0x88474 | `[sp-680] > 0xc (unsigned, 32-bit)` | `[sp-680] <= 0xc (unsigned, 32-bit)` |
| `beq` at 0x88480 | `[sp-680] == 0x6` | `[sp-680] != 0x6` |
| `bhi` at 0x8848c | `[sp-680] > 0x6 (unsigned, 32-bit)` | `[sp-680] <= 0x6 (unsigned, 32-bit)` |
| `bhi` at 0x88498 | `[sp-680] > 0x3 (unsigned, 32-bit)` | `[sp-680] <= 0x3 (unsigned, 32-bit)` |
| `bhs` at 0x884a4 | `[sp-680] >= 0x2 (unsigned, 32-bit)` | `[sp-680] < 0x2 (unsigned, 32-bit)` |
| `beq` at 0x884b0 | `[sp-680] == 0x1` | `[sp-680] != 0x1` |
| `beq` at 0x884c0 | `[sp-680] == 0x4` | `[sp-680] != 0x4` |
| `beq` at 0x884cc | `[sp-680] == 0x5` | `[sp-680] != 0x5` |
| `beq` at 0x884dc | `[sp-680] == 0x9` | `[sp-680] != 0x9` |
| `bhi` at 0x884e8 | `[sp-680] > 0x9 (unsigned, 32-bit)` | `[sp-680] <= 0x9 (unsigned, 32-bit)` |
| `beq` at 0x884f4 | `[sp-680] == 0x7` | `[sp-680] != 0x7` |
| `beq` at 0x88500 | `[sp-680] == 0x8` | `[sp-680] != 0x8` |
| `beq` at 0x88510 | `[sp-680] == 0xa` | `[sp-680] != 0xa` |
| `beq` at 0x8851c | `[sp-680] == 0xb` | `[sp-680] != 0xb` |
| `beq` at 0x8852c | `[sp-680] == 0x12` | `[sp-680] != 0x12` |
| `bhi` at 0x88538 | `[sp-680] > 0x12 (unsigned, 32-bit)` | `[sp-680] <= 0x12 (unsigned, 32-bit)` |
| `beq` at 0x88544 | `[sp-680] == 0xf` | `[sp-680] != 0xf` |
| `bhi` at 0x88550 | `[sp-680] > 0xf (unsigned, 32-bit)` | `[sp-680] <= 0xf (unsigned, 32-bit)` |
| `beq` at 0x8855c | `[sp-680] == 0xd` | `[sp-680] != 0xd` |
| `beq` at 0x88568 | `[sp-680] == 0xe` | `[sp-680] != 0xe` |
| `beq` at 0x88578 | `[sp-680] == 0x10` | `[sp-680] != 0x10` |
| `beq` at 0x88584 | `[sp-680] == 0x11` | `[sp-680] != 0x11` |
| `beq` at 0x88598 | `[sp-680] == 0x1002` | `[sp-680] != 0x1002` |
| `bhi` at 0x885a8 | `[sp-680] > 0x1002 (unsigned, 32-bit)` | `[sp-680] <= 0x1002 (unsigned, 32-bit)` |
| `beq` at 0x885b4 | `[sp-680] == 0x13` | `[sp-680] != 0x13` |
| `beq` at 0x885c4 | `[sp-680] == 0x1001` | `[sp-680] != 0x1001` |
| `beq` at 0x885d8 | `[sp-680] == 0x1003` | `[sp-680] != 0x1003` |
| `beq` at 0x885ec | `[sp-680] == 0x2006` | `[sp-680] != 0x2006` |
| `bhi` at 0x885fc | `[sp-680] > 0x2006 (unsigned, 32-bit)` | `[sp-680] <= 0x2006 (unsigned, 32-bit)` |
| `beq` at 0x8860c | `[sp-680] == 0x100c` | `[sp-680] != 0x100c` |
| `bhi` at 0x8861c | `[sp-680] > 0x100c (unsigned, 32-bit)` | `[sp-680] <= 0x100c (unsigned, 32-bit)` |
| `beq` at 0x8862c | `[sp-680] == 0x1009` | `[sp-680] != 0x1009` |
| `bhi` at 0x8863c | `[sp-680] > 0x1009 (unsigned, 32-bit)` | `[sp-680] <= 0x1009 (unsigned, 32-bit)` |
| `beq` at 0x8864c | `[sp-680] == 0x1007` | `[sp-680] != 0x1007` |
| `beq` at 0x8865c | `[sp-680] == 0x1008` | `[sp-680] != 0x1008` |
| `beq` at 0x88670 | `[sp-680] == 0x100a` | `[sp-680] != 0x100a` |
| `beq` at 0x88680 | `[sp-680] == 0x100b` | `[sp-680] != 0x100b` |
| `beq` at 0x88694 | `[sp-680] == 0x2003` | `[sp-680] != 0x2003` |
| `bhi` at 0x886a4 | `[sp-680] > 0x2003 (unsigned, 32-bit)` | `[sp-680] <= 0x2003 (unsigned, 32-bit)` |
| `beq` at 0x886b4 | `[sp-680] == 0x2001` | `[sp-680] != 0x2001` |
| `beq` at 0x886c4 | `[sp-680] == 0x2002` | `[sp-680] != 0x2002` |
| `beq` at 0x886d8 | `[sp-680] == 0x2004` | `[sp-680] != 0x2004` |
| `beq` at 0x886e8 | `[sp-680] == 0x2005` | `[sp-680] != 0x2005` |
| `beq` at 0x886fc | `[sp-680] == 0x200e` | `[sp-680] != 0x200e` |
| `bhi` at 0x8870c | `[sp-680] > 0x200e (unsigned, 32-bit)` | `[sp-680] <= 0x200e (unsigned, 32-bit)` |
| `beq` at 0x8871c | `[sp-680] == 0x200b` | `[sp-680] != 0x200b` |
| `bhi` at 0x8872c | `[sp-680] > 0x200b (unsigned, 32-bit)` | `[sp-680] <= 0x200b (unsigned, 32-bit)` |
| `beq` at 0x8873c | `[sp-680] == 0x2009` | `[sp-680] != 0x2009` |
| `beq` at 0x8874c | `[sp-680] == 0x200a` | `[sp-680] != 0x200a` |
| `beq` at 0x88760 | `[sp-680] == 0x200c` | `[sp-680] != 0x200c` |
| `beq` at 0x88770 | `[sp-680] == 0x200d` | `[sp-680] != 0x200d` |
| `beq` at 0x88780 | `[sp-680] == 0x4000` | `[sp-680] != 0x4000` |
| `bhi` at 0x8878c | `[sp-680] > 0x4000 (unsigned, 32-bit)` | `[sp-680] <= 0x4000 (unsigned, 32-bit)` |
| `beq` at 0x8879c | `[sp-680] == 0x200f` | `[sp-680] != 0x200f` |
| `beq` at 0x887ac | `[sp-680] == 0x3001` | `[sp-680] != 0x3001` |
| `beq` at 0x887c0 | `[sp-680] == 0x4002` | `[sp-680] != 0x4002` |
| `blo` at 0x887d0 | `[sp-680] < 0x4002 (unsigned, 32-bit)` | `[sp-680] >= 0x4002 (unsigned, 32-bit)` |
| `beq` at 0x887e0 | `[sp-680] == 0x4003` | `[sp-680] != 0x4003` |
| `bne` at 0x88888 | `[sp-680] != 0x3` | `[sp-680] == 0x3` |
| `beq` at 0x888bc | `[sp-676] == 0x5a` | `[sp-676] != 0x5a` |
| `bgt` at 0x888c8 | `[sp-676] > 0x5a (signed, 32-bit)` | `[sp-676] <= 0x5a (signed, 32-bit)` |
| `beq` at 0x888d4 | `[sp-676] == 0x3d` | `[sp-676] != 0x3d` |
| `beq` at 0x888e4 | `[sp-676] == 0x7c` | `[sp-676] != 0x7c` |
| `bne` at 0x888f4 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x889ac | `[ret(0x74908)+16] != 0x3d` | `[ret(0x74908)+16] == 0x3d` |
| `bne` at 0x889b8 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x88a48 | `[ret(0x74908)+16] != 0x3d` | `[ret(0x74908)+16] == 0x3d` |
| `bne` at 0x88a54 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x88ae4 | `[ret(0x74908)+16] != 0x3d` | `[ret(0x74908)+16] == 0x3d` |
| `bne` at 0x88af0 | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x88b80 | `[ret(0x74908)+16] != 0x3d` | `[ret(0x74908)+16] == 0x3d` |
| `bne` at 0x88b8c | `[sp-704] != 0x0` | `[sp-704] == 0x0` |
| `bne` at 0x88ccc | `[sp-676] != 0x40` | `[sp-676] == 0x40` |
| `bne` at 0x88cfc | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x88e2c | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x88e5c | `? != 0x0` | `? == 0x0` |
| `blt` at 0x88e8c | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x88f04 | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `blt` at 0x88f38 | `[sp-716] < [sp-720] (signed, 32-bit)` | `[sp-716] >= [sp-720] (signed, 32-bit)` |
| `beq` at 0x88fd0 | `[sp-1240] == 0x1` | `[sp-1240] != 0x1` |
| `beq` at 0x88fdc | `[sp-1240] == 0x2` | `[sp-1240] != 0x2` |
| `bhi` at 0x891e4 | `[[sp-24]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x89214 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x89244 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x89294 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89374 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x893b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x89424 | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `blt` at 0x89450 | `[sp-716] < [sp-744] (signed, 32-bit)` | `[sp-716] >= [sp-744] (signed, 32-bit)` |
| `ldrls` at 0x89478 | `[sp-724] <= 0x4 (unsigned, 32-bit)` | `[sp-724] > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x898bc | `[sp-736] == 0x0` | `[sp-736] != 0x0` |
| `blo` at 0x898cc | `[sp-728] < [sp-740] (unsigned, 32-bit)` | `[sp-728] >= [sp-740] (unsigned, 32-bit)` |
| `bls` at 0x89900 | `([sp-728] + [sp-732]) <= [sp-740] (unsigned, 32-bit)` | `([sp-728] + [sp-732]) > [sp-740] (unsigned, 32-bit)` |
| `beq` at 0x89970 | `[sp-732] == 0x0` | `[sp-732] != 0x0` |
| `beq` at 0x899ac | `[0x2ae60c] == 0x0` | `[0x2ae60c] != 0x0` |
| `bne` at 0x899ec | `ret(0x1b22c) != 0x0` | `ret(0x1b22c) == 0x0` |
| `ldrls` at 0x89b04 | `[sp-676]-48 <= 0x4c (unsigned, 32-bit)` | `[sp-676]-48 > 0x4c (unsigned, 32-bit)` |
| `bne` at 0x89dac | `[sp-676] != 0x5a` | `[sp-676] == 0x5a` |
| `beq` at 0x89e70 | `[sp-676] == 0x5a` | `[sp-676] != 0x5a` |
| `beq` at 0x89e7c | `[sp-676] == 0x7c` | `[sp-676] != 0x7c` |
| `bne` at 0x8a024 | `ret(0x5d81c) != 0x0` | `ret(0x5d81c) == 0x0` |
| `bne` at 0x8a05c | `[0x2ae5b4] != 0x0` | `[0x2ae5b4] == 0x0` |
| `blt` at 0x8a0e4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8a154 | `ret(0x5d83c) == 0x0` | `ret(0x5d83c) != 0x0` |
| `beq` at 0x8a1ac | `ret(0x5d8c0) == 0x0` | `ret(0x5d8c0) != 0x0` |
| `blt` at 0x8a254 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x8a30c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8a43c | `ret(0xe3430) == 0x0` | `ret(0xe3430) != 0x0` |
| `bls` at 0x8a468 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x8a4b0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8a558 | `[sp-672] == 0x0` | `[sp-672] != 0x0` |
| `bhi` at 0x8a5a4 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8a5d4 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8a62c | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x8a658 | `[[sp-24]+28] < 0x0 (signed, 32-bit)` | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x8a694 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8a744 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x8a774 | `? == 0x2` | `? != 0x2` |
| `bne` at 0x8a7a4 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8a800 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8a830 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8a89c | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8a8cc | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8a924 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8a95c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x8a9d8 | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8aa08 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8aa44 | `[sp-1320] == 0x1` | `[sp-1320] != 0x1` |
| `blo` at 0x8aa50 | `[sp-1320] < 0x1 (unsigned, 32-bit)` | `[sp-1320] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x8aa5c | `[sp-1320] == 0x2` | `[sp-1320] != 0x2` |
| `bne` at 0x8aa74 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8aab8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8aaf8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ab2c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8abbc | `[[sp-24]+32]:1 == 0x4` | `[[sp-24]+32]:1 != 0x4` |
| `bne` at 0x8abec | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ac48 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8aca8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8ad54 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x8ad78 | `ret(0x83734) != ?` | `ret(0x83734) == ?` |
| `beq` at 0x8ad90 | `[[sp-24]+32]:1 == 0x3` | `[[sp-24]+32]:1 != 0x3` |
| `bne` at 0x8addc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ae2c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8ae7c | `ret(0x4e10c) == 0x0` | `ret(0x4e10c) != 0x0` |
| `bls` at 0x8aec0 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8aed0 | `[0x1497d4] != 0x0` | `[0x1497d4] == 0x0` |
| `bne` at 0x8af00 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8af3c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8af90 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x8afd4 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8afe4 | `[0x1497d8] != 0x0` | `[0x1497d8] == 0x0` |
| `bne` at 0x8b014 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b050 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b0a4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bls` at 0x8b0e8 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x8b0f8 | `[0x1497dc] != 0x0` | `[0x1497dc] == 0x0` |
| `bne` at 0x8b128 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8b164 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8b1b8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x8b204 | `ret(0x3d48c) == 0x0` | `ret(0x3d48c) != 0x0` |
| `beq` at 0x8b250 | `ret(0x3d858) == 0x0` | `ret(0x3d858) != 0x0` |
| `bne` at 0x8b2a8 | `ret(0xd1dfc) != 0x0` | `ret(0xd1dfc) == 0x0` |
| `blt` at 0x8b2f4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8b3f8 | `[[sp-760]] != 0x1` | `[[sp-760]] == 0x1` |
| `bne` at 0x8b41c | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `beq` at 0x8b440 | `[[sp-760]+28] == 0x0` | `[[sp-760]+28] != 0x0` |
| `beq` at 0x8b49c | `ret(0x8d224) == 0x0` | `ret(0x8d224) != 0x0` |
| `bne` at 0x8b528 | `[[sp-760]] != 0x2` | `[[sp-760]] == 0x2` |
| `bne` at 0x8b54c | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `beq` at 0x8b570 | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `ble` at 0x8b5d0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x8b628 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x8b69c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8b714 | `[[[sp-760]+12]+64]:1 == 0x2c` | `[[[sp-760]+12]+64]:1 != 0x2c` |
| `beq` at 0x8b73c | `ret(0x1afbc) == 0x0` | `ret(0x1afbc) != 0x0` |
| `bne` at 0x8b764 | `ret(0x1afbc) != 0x0` | `ret(0x1afbc) == 0x0` |
| `beq` at 0x8b890 | `[0x1497e8] == 0x0` | `[0x1497e8] != 0x0` |
| `bne` at 0x8b8d4 | `[[sp-760]] != 0x3` | `[[sp-760]] == 0x3` |
| `bne` at 0x8b8f8 | `ret(0xd1e50) != 0x0` | `ret(0xd1e50) == 0x0` |
| `beq` at 0x8b91c | `[[sp-760]+4] == 0x0` | `[[sp-760]+4] != 0x0` |
| `beq` at 0x8b950 | `[[sp-760]+32] == 0x0` | `[[sp-760]+32] != 0x0` |
| `beq` at 0x8b984 | `[[sp-760]+44] == 0x0` | `[[sp-760]+44] != 0x0` |
| `beq` at 0x8b9b8 | `[[sp-760]+64] == 0x0` | `[[sp-760]+64] != 0x0` |
| `ble` at 0x8ba18 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8ba58 | `ret(0x1afbc) == 0x0` | `ret(0x1afbc) != 0x0` |
| `bge` at 0x8bad4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x8bb90 | `[sp-728] == 0x0` | `[sp-728] != 0x0` |
| `bne` at 0x8bc40 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8bc7c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8bcb8 | `[sp-756] == 0x1` | `[sp-756] != 0x1` |
| `beq` at 0x8bcc4 | `[sp-756] == 0x2` | `[sp-756] != 0x2` |
| `bne` at 0x8bcdc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8bd40 | `ret(0x39684) != 0x0` | `ret(0x39684) == 0x0` |
| `beq` at 0x8bd6c | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `beq` at 0x8bdf4 | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `bne` at 0x8be4c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8be88 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8bec4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8bf34 | `ret(0x3987c) != 0x0` | `ret(0x3987c) == 0x0` |
| `beq` at 0x8bf60 | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `beq` at 0x8bfe8 | `[sp-1196]:1 == 0x0` | `[sp-1196]:1 != 0x0` |
| `beq` at 0x8c068 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x8c098 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `bhi` at 0x8c0e0 | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8c118 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x8c12c | `[[sp-24]+48] >= ret(0xdadb8) (unsigned, 32-bit)` | `[[sp-24]+48] < ret(0xdadb8) (unsigned, 32-bit)` |
| `bhs` at 0x8c14c | `ret(0xb7944) >= [sp-1412] (unsigned, 32-bit)` | `ret(0xb7944) < [sp-1412] (unsigned, 32-bit)` |
| `bne` at 0x8c180 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8c1bc | `ret(0xdaec8) == 0x0` | `ret(0xdaec8) != 0x0` |
| `bne` at 0x8c21c | `[[sp-24]+32]:1 != 0x0` | `[[sp-24]+32]:1 == 0x0` |
| `bne` at 0x8c254 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x8c268 | `[[sp-24]+48] >= ret(0xdadb8) (unsigned, 32-bit)` | `[[sp-24]+48] < ret(0xdadb8) (unsigned, 32-bit)` |
| `bhs` at 0x8c288 | `ret(0xb7944) >= [sp-1420] (unsigned, 32-bit)` | `ret(0xb7944) < [sp-1420] (unsigned, 32-bit)` |
| `bne` at 0x8c2bc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8c304 | `ret(0xdaf80) == 0x0` | `ret(0xdaf80) != 0x0` |
| `bls` at 0x8c37c | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x8c390 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c3cc | `[sp-1428] != [[sp-24]+32]:1-2` | `[sp-1428] == [[sp-24]+32]:1-2` |
| `blt` at 0x8c41c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8c448 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8c4b8 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `bhi` at 0x8c544 | `[[sp-24]+32]:1 > 0x4 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x4 (unsigned, 32-bit)` |
| `bne` at 0x8c574 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8c5c0 | `? == [[sp-24]+32]:1` | `? != [[sp-24]+32]:1` |
| `blt` at 0x8c60c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8c644 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c6d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c764 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8c7e8 | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `bhi` at 0x8c854 | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0x8c87c | `[sp-676] == 0x35` | `[sp-676] != 0x35` |
| `beq` at 0x8c888 | `[sp-676] == 0x40` | `[sp-676] != 0x40` |
| `beq` at 0x8c894 | `[sp-676] == 0x4f` | `[sp-676] != 0x4f` |
| `beq` at 0x8c8a0 | `[sp-676] == 0x42` | `[sp-676] != 0x42` |
| `beq` at 0x8c8ac | `[sp-676] == 0x46` | `[sp-676] != 0x46` |
| `beq` at 0x8c8b8 | `[sp-676] == 0x48` | `[sp-676] != 0x48` |
| `beq` at 0x8c8c4 | `[sp-676] == 0x4c` | `[sp-676] != 0x4c` |
| `beq` at 0x8c8d0 | `[sp-676] == 0x4d` | `[sp-676] != 0x4d` |
| `beq` at 0x8c8dc | `[sp-676] == 0x7c` | `[sp-676] != 0x7c` |
| `beq` at 0x8c8f0 | `[0x1497e8] == 0x0` | `[0x1497e8] != 0x0` |
| `bne` at 0x8c904 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8c938 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8c98c | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `bhi` at 0x8ca64 | `[[sp-24]+32]:1 > 0x6 (unsigned, 32-bit)` | `[[sp-24]+32]:1 <= 0x6 (unsigned, 32-bit)` |
| `bne` at 0x8ca94 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8cb28 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8cb6c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8cbf4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8cc7c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8cd28 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8cdf4 | `ret(0x1b0dc) == 0x0` | `ret(0x1b0dc) != 0x0` |
| `beq` at 0x8cf28 | `[[sp-24]+32]:1 == 0x1` | `[[sp-24]+32]:1 != 0x1` |
| `bge` at 0x8cf60 | `? >= [0x2b1730] (signed, 32-bit)` | `? < [0x2b1730] (signed, 32-bit)` |
| `bne` at 0x8cfa8 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x8cff8 | `? != 0x4` | `? == 0x4` |
| `bge` at 0x8d074 | `ret(0x54e58) >= 0x0 (signed, 32-bit)` | `ret(0x54e58) < 0x0 (signed, 32-bit)` |
| `beq` at 0x8d0ec | `[0x1497d0] == 0x0` | `[0x1497d0] != 0x0` |
| `beq` at 0x8d138 | `[sp-668] == 0x0` | `[sp-668] != 0x0` |
| `beq` at 0x8d168 | `ret(0xb6bdc) == 0x0` | `ret(0xb6bdc) != 0x0` |
| `beq` at 0x8d1b0 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8d1e0 | `r0` | `ret(0xb56f4)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x89478).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x89b04).

## `sn1sc_process_scancmd`

- `sn1sc_process_scancmd`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3430 to 0xe4d9c.
  Established. A byte store clears absolute `0x2b37a8` (`strb` at 0xe3498). The following global
  test takes its continuation for `[0x2ad258] != 0x0` and falls through on zero (`bne` at 0xe34a8).
  An indirect call has unknown target and candidates unknown, unknown, `[0x142b50]`, unknown (`mov`
  at 0xe352c). The result test takes its branch on zero and falls through on nonzero (`beq` at
  0xe3538). A known-edge call to `0x1b508` carries `0x2b37a8`, `0x135760`, unknown, unknown (`bl` at
  0xe4cd4). The return has unresolved r0 (`ldmdb` at 0xe4d98). Scan-command decoding and behavior of
  the computed selector remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3440 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xe3444 | load | 4 | `abs 0xe43e8` | `-` |
| `ldr` at 0xe3448 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe3458 | load | 4 | `abs 0xe43b8` | `-` |
| `ldr` at 0xe3460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe346c | load | 4 | `abs 0xe43e8` | `-` |
| `ldr` at 0xe3470 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe3480 | load | 4 | `abs 0xe43bc` | `-` |
| `ldr` at 0xe3488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3490 | load | 4 | `abs 0xe43e4` | `-` |
| `strb` at 0xe3498 | store | 1 | `abs 0x2b37a8` | `0x0` |
| `ldr` at 0xe349c | load | 4 | `abs 0xe43c0` | `-` |
| `ldr` at 0xe34a0 | load | 4 | `abs 0x2ad258` | `-` |
| `str` at 0xe34b0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe34b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe34b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe34c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe34c8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe34d4 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0xe34dc | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe34e8 | load | 4 | `abs 0xe43c4` | `-` |
| `ldr` at 0xe34f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe34f8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe3508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3510 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe351c | load | 4 | `abs 0x142b50` | `-` |
| `ldr` at 0xe3540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3548 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe3558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3560 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe356c | load | 4 | `abs 0xe43e4` | `-` |
| `ldr` at 0xe3570 | load | 4 | `abs 0xe43c8` | `-` |
| `ldr` at 0xe3580 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3584 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3588 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe3590 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0xe3598 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe35a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe35a4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xe35b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe4ccc | load | 4 | `abs 0xe43e4` | `-` |
| `ldr` at 0xe4cd0 | load | 4 | `abs 0xe43dc` | `-` |
| `str` at 0xe4cdc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe4ce4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4ce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4cf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4cf8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe4d04 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xe4d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe4d14 | load | 4 | `abs 0xe43e8` | `-` |
| `ldr` at 0xe4d18 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe4d24 | load | 4 | `abs 0xe43e4` | `-` |
| `ldrb` at 0xe4d28 | load | 1 | `abs 0x2b37a8` | `-` |
| `ldr` at 0xe4d34 | load | 4 | `abs 0xe43e0` | `-` |
| `ldr` at 0xe4d38 | load | 4 | `abs 0xe43e4` | `-` |
| `ldr` at 0xe4d40 | load | 4 | `abs 0xe43e8` | `-` |
| `ldr` at 0xe4d44 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe4d54 | load | 4 | `abs 0xe43ec` | `-` |
| `ldr` at 0xe4d5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4d68 | load | 4 | `abs 0xe43f0` | `-` |
| `ldr` at 0xe4d6c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe4d7c | load | 4 | `abs 0xe43f4` | `-` |
| `ldr` at 0xe4d84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4d8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe4d90 | store | 4 | `sp -0x5c` | `[sp-28]` |
| `ldr` at 0xe4d94 | load | 4 | `sp -0x5c` | `-` |
| `ldmdb` at 0xe4d98 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe4d98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe4d98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe4d98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3464 | `(unknown callee) 0xdc3bc` | `0x0` | `0x1356d4` | `0x1` | `r0` |
| `bl` at 0xe348c | `(unknown callee) 0xdc3bc` | `0x0` | `0x1356f8` | `0x0` | `r0` |
| `bl` at 0xe34e4 | `(unknown callee) 0xe335c` | `?` | `?` | `0x2b37a8` | `[0x2ad258]` |
| `mov` at 0xe352c | `(unknown callee) (unknown target)` | `?` | `?` | `[0x142b50]` | `?` |
| `bl` at 0xe3578 | `(unknown callee) 0x1b49c` | `0x2b37a8` | `0x13571c` | `?` | `?` |
| `bl` at 0xe357c | `(unknown callee) 0xe33c8` | `ret(0x1b49c)` | `?` | `?` | `?` |
| `bl` at 0xe4cd4 | `(unknown callee) 0x1b508` | `0x2b37a8` | `0x135760` | `?` | `?` |
| `bl` at 0xe4ce0 | `(unknown callee) 0xe33c8` | `ret(0x1b508)` | `?` | `?` | `0x0` |
| `bl` at 0xe4d3c | `(unknown callee) 0x23f2c` | `0x13578c` | `0x2b37a8` | `?` | `[0x2b37a8]:1` |
| `bl` at 0xe4d60 | `(unknown callee) 0xdc3bc` | `0x0` | `0x13579c` | `0x1` | `[sp-24]` |
| `bl` at 0xe4d88 | `(unknown callee) 0xdc3bc` | `0x0` | `0x1357c4` | `0x0` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe3450 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xe3478 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xe34a8 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |
| `beq` at 0xe3538 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `ldrls` at 0xe35b8 | `? <= 0x14 (unsigned, 32-bit)` | `? > 0x14 (unsigned, 32-bit)` |
| `bne` at 0xe4d10 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xe4d20 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0xe4d30 | `[0x2b37a8]:1 == 0x0` | `[0x2b37a8]:1 != 0x0` |
| `bls` at 0xe4d4c | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xe4d74 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe4d98 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xe35b8).

## `SHUB_ScanInitConfig`

- `SHUB_ScanInitConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b288 to 0x4bd0c.
  Established. A halfword store writes `0x320` at absolute `0x2b07e0` (`strh` at 0x4b414). The
  earlier halfword test takes the path to that store for `[0x2b07e0]:2 == 0x0` and falls through on
  nonzero (`beq` at 0x4b3cc). The first call to `0xeac50` carries `[sp-168]`, `0x7c`, `0x0`, `0x0`
  (`bl` at 0x4b380); its zero result takes the continuation and nonzero falls through (`beq` at
  0x4b38c). The call to `0x49860` has `[sp-20]`, unknown, `[0x14805c]:1`, `sp-172` (`bl` at
  0x4b464). Initialization fields, physical scan effects and r0 at the merged exit remain
  unidentified (`ldmdb` at 0x4bc48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4b298 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4b29c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4b2a8 | store | 4 | `sp -0xa4` | `0x0` |
| `str` at 0x4b2b4 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x4b2c0 | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x4b2cc | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x4b2d8 | store | 4 | `sp -0x94` | `0x0` |
| `str` at 0x4b2e4 | store | 4 | `sp -0x90` | `0x0` |
| `str` at 0x4b2f0 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x4b2fc | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x4b300 | load | 4 | `abs 0x4bc4c` | `-` |
| `str` at 0x4b304 | store | 4 | `sp -0xa4` | `0x7ffffd28` |
| `ldr` at 0x4b308 | load | 4 | `abs 0x4bc50` | `-` |
| `str` at 0x4b30c | store | 4 | `sp -0xa0` | `0x602800` |
| `ldr` at 0x4b310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b318 | load | 4 | `abs 0x4bc54` | `-` |
| `ldr` at 0x4b31c | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x4b324 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4b328 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x4b32c | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b330 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b33c | load | 4 | `abs 0x4bc5c` | `-` |
| `str` at 0x4b340 | store | 4 | `sp -0xc4` | `0x109bfc` |
| `ldr` at 0x4b344 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b348 | store | 4 | `sp -0xc0` | `r0` |
| `ldr` at 0x4b350 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b354 | load | 4 | `abs 0x4bc64` | `-` |
| `str` at 0x4b364 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x4b368 | load | 4 | `abs 0x4bc68` | `-` |
| `str` at 0x4b36c | store | 4 | `sp -0xc0` | `0x109c18` |
| `ldr` at 0x4b370 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4b390 | load | 4 | `abs 0x4bc6c` | `-` |
| `str` at 0x4b394 | store | 4 | `sp -0xc4` | `0x109c24` |
| `ldr` at 0x4b398 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b39c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b3a4 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b3a8 | load | 4 | `abs 0x4bc70` | `-` |
| `ldr` at 0x4b3b4 | load | 4 | `abs 0x4bc74` | `-` |
| `str` at 0x4b3b8 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4b3c0 | load | 4 | `abs 0x4bc78` | `-` |
| `ldrh` at 0x4b3c4 | load | 2 | `abs 0x2b07e0` | `-` |
| `ldr` at 0x4b3d0 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b3d4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b3e0 | load | 4 | `abs 0x4bc7c` | `-` |
| `str` at 0x4b3e4 | store | 4 | `sp -0xc4` | `0x109c54` |
| `ldr` at 0x4b3e8 | load | 4 | `abs 0x4bc78` | `-` |
| `ldrh` at 0x4b3ec | load | 2 | `abs 0x2b07e0` | `-` |
| `str` at 0x4b3f0 | store | 4 | `sp -0xc0` | `[0x2b07e0]:2` |
| `ldr` at 0x4b3f8 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b3fc | load | 4 | `abs 0x4bc80` | `-` |
| `ldr` at 0x4b40c | load | 4 | `abs 0x4bc78` | `-` |
| `strh` at 0x4b414 | store | 2 | `abs 0x2b07e0` | `0x320` |
| `ldr` at 0x4b418 | load | 4 | `abs 0x4bc84` | `-` |
| `ldrb` at 0x4b41c | load | 1 | `abs 0x14805c` | `-` |
| `str` at 0x4b42c | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0x4b430 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b438 | load | 4 | `abs 0x4bc54` | `-` |
| `ldr` at 0x4b43c | load | 4 | `abs 0x148058` | `-` |
| `ldrb` at 0x4b444 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b448 | load | 4 | `abs 0x4bc84` | `-` |
| `ldrb` at 0x4b44c | load | 1 | `abs 0x14805c` | `-` |
| `ldr` at 0x4b454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b468 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b46c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b48c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b498 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b49c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b4bc | load | 1 | `(unknown)` | `-` |
| `str` at 0x4b4c0 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x4b4c4 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b4c8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b4d4 | load | 4 | `abs 0x4bc8c` | `-` |
| `str` at 0x4b4d8 | store | 4 | `sp -0xc4` | `0x109c6c` |
| `ldr` at 0x4b4dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b4e0 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b4e4 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x4b4e8 | store | 4 | `sp -0xbc` | `[sp-176]` |
| `ldr` at 0x4b4f0 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b4f4 | load | 4 | `abs 0x4bc90` | `-` |
| `ldr` at 0x4b504 | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x4b508 | load | 1 | `[sp-172] +0x6` | `-` |
| `str` at 0x4b50c | store | 4 | `sp -0xb0` | `[[sp-172]+6]:1` |
| `ldr` at 0x4b510 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b514 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b520 | load | 4 | `abs 0x4bc94` | `-` |
| `str` at 0x4b524 | store | 4 | `sp -0xc4` | `0x109c90` |
| `ldr` at 0x4b528 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b52c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b530 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x4b534 | store | 4 | `sp -0xbc` | `[[sp-172]+6]:1` |
| `ldr` at 0x4b53c | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b540 | load | 4 | `abs 0x4bc98` | `-` |
| `ldr` at 0x4b54c | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x4b550 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x4b554 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x4b560 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x4b56c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x4b57c | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b580 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b5a0 | load | 1 | `sp -0xb0` | `-` |
| `strb` at 0x4b5a4 | store | 1 | `(unknown)` | `([sp-176]:1 & 0xff)` |
| `ldr` at 0x4b5ac | load | 4 | `abs 0x4bc9c` | `-` |
| `str` at 0x4b5b0 | store | 4 | `sp -0xc4` | `0x109ca8` |
| `ldr` at 0x4b5b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b5b8 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b5c0 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b5c4 | load | 4 | `abs 0x4bca0` | `-` |
| `ldr` at 0x4b5d0 | load | 4 | `abs 0x4bc74` | `-` |
| `str` at 0x4b5d4 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4b5dc | load | 4 | `abs 0x4bca4` | `-` |
| `ldr` at 0x4b5e0 | load | 4 | `abs 0x148074` | `-` |
| `ldr` at 0x4b5ec | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b5f0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b5fc | load | 4 | `abs 0x4bca8` | `-` |
| `str` at 0x4b600 | store | 4 | `sp -0xc4` | `0x109cfc` |
| `ldr` at 0x4b604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b608 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b60c | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b610 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b630 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4b634 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4b63c | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b640 | load | 4 | `abs 0x4bcac` | `-` |
| `ldr` at 0x4b650 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b674 | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x4b678 | load | 1 | `[sp-172] +0xc` | `-` |
| `strb` at 0x4b67c | store | 1 | `(unknown)` | `([[sp-172]+12]:1 & 0xff)` |
| `ldr` at 0x4b680 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b684 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b690 | load | 4 | `abs 0x4bcb0` | `-` |
| `str` at 0x4b694 | store | 4 | `sp -0xc4` | `0x109d20` |
| `ldr` at 0x4b698 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b69c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b6a0 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b6a4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b6c4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4b6c8 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4b6d0 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b6d4 | load | 4 | `abs 0x4bcb4` | `-` |
| `ldr` at 0x4b6e0 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b6e4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4b704 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4b710 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b714 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b720 | load | 4 | `abs 0x4bcb8` | `-` |
| `str` at 0x4b724 | store | 4 | `sp -0xc4` | `0x109d44` |
| `ldr` at 0x4b728 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b72c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b730 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b734 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4b754 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4b758 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4b760 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b764 | load | 4 | `abs 0x4bcbc` | `-` |
| `ldr` at 0x4b774 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b778 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b798 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x4b79c | load | 2 | `[sp-172] +0x8` | `-` |
| `strh` at 0x4b7a0 | store | 2 | `(unknown)` | `([[sp-172]+8]:2 & 0xffff)` |
| `ldr` at 0x4b7a4 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b7a8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b7b4 | load | 4 | `abs 0x4bcc0` | `-` |
| `str` at 0x4b7b8 | store | 4 | `sp -0xc4` | `0x109d70` |
| `ldr` at 0x4b7bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b7c0 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b7c4 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b7c8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4b7e8 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4b7ec | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4b7f4 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b7f8 | load | 4 | `abs 0x4bcc4` | `-` |
| `ldr` at 0x4b804 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b808 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4b828 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4b834 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b838 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b844 | load | 4 | `abs 0x4bcc8` | `-` |
| `str` at 0x4b848 | store | 4 | `sp -0xc4` | `0x109d90` |
| `ldr` at 0x4b84c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b850 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b854 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b858 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4b878 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4b87c | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4b884 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b888 | load | 4 | `abs 0x4bccc` | `-` |
| `ldr` at 0x4b898 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b89c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b8bc | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x4b8c0 | load | 2 | `[sp-172] +0xa` | `-` |
| `strh` at 0x4b8c4 | store | 2 | `(unknown)` | `([[sp-172]+10]:2 & 0xffff)` |
| `ldr` at 0x4b8c8 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4b8cc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4b8d8 | load | 4 | `abs 0x4bcd0` | `-` |
| `str` at 0x4b8dc | store | 4 | `sp -0xc4` | `0x109dbc` |
| `ldr` at 0x4b8e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b8e4 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b8e8 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b8ec | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4b90c | load | 2 | `(unknown)` | `-` |
| `str` at 0x4b910 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4b918 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b92c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b940 | store | 4 | `sp -0xc4` | `0x38` |
| `ldr` at 0x4b944 | load | 4 | `abs 0x4bcd4` | `-` |
| `str` at 0x4b948 | store | 4 | `sp -0xc0` | `0x109ddc` |
| `ldr` at 0x4b94c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4b968 | load | 4 | `abs 0x4bcd8` | `-` |
| `str` at 0x4b96c | store | 4 | `sp -0xc4` | `0x109de4` |
| `ldr` at 0x4b970 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b974 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b97c | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4b980 | load | 4 | `abs 0x4bcdc` | `-` |
| `ldr` at 0x4b98c | load | 4 | `abs 0x4bc74` | `-` |
| `str` at 0x4b990 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4b998 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4b9d0 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x4b9d8 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4b9dc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4b9fc | load | 2 | `(unknown)` | `-` |
| `str` at 0x4ba58 | store | 4 | `sp -0xc4` | `0xf2` |
| `ldr` at 0x4ba5c | load | 4 | `abs 0x4bce0` | `-` |
| `str` at 0x4ba60 | store | 4 | `sp -0xc0` | `0x109e0c` |
| `ldr` at 0x4ba64 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4ba80 | load | 4 | `abs 0x4bce4` | `-` |
| `str` at 0x4ba84 | store | 4 | `sp -0xc4` | `0x109e1c` |
| `ldr` at 0x4ba88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4ba8c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4ba94 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4ba98 | load | 4 | `abs 0x4bce8` | `-` |
| `ldr` at 0x4baa4 | load | 4 | `abs 0x4bc74` | `-` |
| `str` at 0x4baa8 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4bab0 | load | 4 | `abs 0x4bc88` | `-` |
| `ldr` at 0x4bab4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bad4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4bb30 | store | 4 | `sp -0xc4` | `0xf2` |
| `ldr` at 0x4bb34 | load | 4 | `abs 0x4bcec` | `-` |
| `str` at 0x4bb38 | store | 4 | `sp -0xc0` | `0x109e4c` |
| `ldr` at 0x4bb3c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4bb58 | load | 4 | `abs 0x4bcf0` | `-` |
| `str` at 0x4bb5c | store | 4 | `sp -0xc4` | `0x109e5c` |
| `ldr` at 0x4bb60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bb64 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bb6c | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4bb70 | load | 4 | `abs 0x4bcf4` | `-` |
| `ldr` at 0x4bb7c | load | 4 | `abs 0x4bc74` | `-` |
| `str` at 0x4bb80 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `str` at 0x4bb8c | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x4bb90 | load | 4 | `abs 0x4bcf8` | `-` |
| `str` at 0x4bb94 | store | 4 | `sp -0xc0` | `0x109e8c` |
| `ldr` at 0x4bb98 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4bbb8 | load | 4 | `abs 0x4bcfc` | `-` |
| `str` at 0x4bbbc | store | 4 | `sp -0xc4` | `0x109e98` |
| `ldr` at 0x4bbc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bbc4 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bbcc | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4bbd0 | load | 4 | `abs 0x4bd00` | `-` |
| `ldr` at 0x4bbdc | load | 4 | `abs 0x4bc74` | `-` |
| `str` at 0x4bbe0 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4bbe8 | load | 4 | `abs 0x4bc58` | `-` |
| `ldr` at 0x4bbec | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x4bbf8 | load | 4 | `abs 0x4bd04` | `-` |
| `str` at 0x4bbfc | store | 4 | `sp -0xc4` | `0x109ec4` |
| `ldr` at 0x4bc04 | load | 4 | `abs 0x4bc60` | `-` |
| `ldr` at 0x4bc08 | load | 4 | `abs 0x4bd08` | `-` |
| `str` at 0x4bc40 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x4bc44 | load | 4 | `sp -0xb4` | `-` |
| `ldmdb` at 0x4bc48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4bc48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4bc48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4b35c | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x4bd` | `0x0` |
| `bl` at 0x4b380 | `(unknown callee) 0xeac50` | `[sp-168]` | `0x7c` | `0x0` | `0x0` |
| `bl` at 0x4b3b0 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x4c5` | `0x0` |
| `bl` at 0x4b404 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x4cc` | `0x0` |
| `bl` at 0x4b464 | `(unknown callee) 0x49860` | `[sp-20]` | `?` | `[0x14805c]:1` | `sp-172` |
| `bl` at 0x4b4fc | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x4ea` | `0x0` |
| `bl` at 0x4b548 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x4ed` | `0x0` |
| `bl` at 0x4b5cc | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x4f9` | `0x0` |
| `bl` at 0x4b648 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x503` | `0x0` |
| `bl` at 0x4b6dc | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x506` | `0x0` |
| `bl` at 0x4b76c | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x50f` | `0x0` |
| `bl` at 0x4b800 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x513` | `0x0` |
| `bl` at 0x4b890 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x51c` | `0x0` |
| `bl` at 0x4b924 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x520` | `0x0` |
| `bl` at 0x4b934 | `(unknown callee) 0x4abf4` | `[sp-20]` | `sp-132` | `?` | `sp-132` |
| `bl` at 0x4b958 | `(unknown callee) 0xeac50` | `[sp-168]` | `0x7e` | `sp-132` | `0x0` |
| `bl` at 0x4b988 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x52d` | `0x0` |
| `bl` at 0x4b9b4 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x4b9c8 | `(unknown callee) 0xdcb88` | `0x0` | `0x0` | `0x20` | `sp-124` |
| `bl` at 0x4ba18 | `(unknown callee) 0xe1c00` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4ba28 | `(unknown callee) 0xe1c00` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4ba3c | `(unknown callee) 0xe1c74` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4ba4c | `(unknown callee) 0xe1c74` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4ba70 | `(unknown callee) 0xeac50` | `[sp-168]` | `0x1a` | `sp-164` | `0x0` |
| `bl` at 0x4baa0 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x54c` | `0x0` |
| `bl` at 0x4baf0 | `(unknown callee) 0xe1c00` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4bb00 | `(unknown callee) 0xe1c00` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4bb14 | `(unknown callee) 0xe1c74` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4bb24 | `(unknown callee) 0xe1c74` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4bb48 | `(unknown callee) 0xeac50` | `[sp-168]` | `0x1b` | `sp-164` | `0x0` |
| `bl` at 0x4bb78 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x559` | `0x0` |
| `bl` at 0x4bba8 | `(unknown callee) 0xeac50` | `[sp-168]` | `0x7d` | `0x0` | `0x0` |
| `bl` at 0x4bbd8 | `(unknown callee) 0x21e28` | `0x3` | `0x109480` | `0x563` | `0x0` |
| `bl` at 0x4bc10 | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x566` | `0x0` |
| `bl` at 0x4bc24 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x4bc38 | `(unknown callee) 0xdcb88` | `0x0` | `0x0` | `0x20` | `sp-124` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4b338 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b38c | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4b3cc | `[0x2b07e0]:2 == 0x0` | `[0x2b07e0]:2 != 0x0` |
| `beq` at 0x4b3dc | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b424 | `[0x14805c]:1 == 0x0` | `[0x14805c]:1 != 0x0` |
| `beq` at 0x4b494 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4b4d0 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b51c | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `blt` at 0x4b55c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x4b568 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4b574 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x4b5e8 | `[0x148074] == 0x0` | `[0x148074] != 0x0` |
| `beq` at 0x4b5f8 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b68c | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b70c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4b71c | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b7b0 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b830 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4b840 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b8d4 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x4b964 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `bne` at 0x4b9a0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x4ba08 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4ba7c | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4bae0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4bb54 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4bbb4 | `ret(0xeac50) == 0x0` | `ret(0xeac50) != 0x0` |
| `beq` at 0x4bbf4 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4bc48 | `r0` | `?` |
