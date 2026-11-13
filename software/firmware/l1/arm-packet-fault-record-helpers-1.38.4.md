# ARM packet and fault record helpers in 1.38.4

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
FRUStatus and FruFmtMultiRec have unresolved computed selectors. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/packet-fault-record-helpers-facts.txt) retain all tool sections.

## `pkr`

- `pkr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60ff0 to 0x61070.
  Established. The indirect call has entry r0 through r3 as candidates (`mov` at 0x6102c). The
  recorded return carries `[sp-36]` (`ldmdb` at 0x61064); its target and the hardware read effect
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61000 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x61004 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x61008 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x6100c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x61010 | load | 4 | `abs 0x61068` | `-` |
| `ldr` at 0x61014 | load | 4 | `abs 0x2b0940` | `-` |
| `ldr` at 0x61018 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6101c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61020 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x61024 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61034 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x61038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6103c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61040 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x61044 | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x61048 | load | 4 | `abs 0x6106c` | `-` |
| `ldr` at 0x6104c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x61050 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x61054 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x6105c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x61064 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x61064 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x61064 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x6102c | `(unknown callee) (unknown target)` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x61058 | `(unknown callee) 0x2070c` | `0x10c8d4` | `[sp-24]` | `[[sp-28]]` | `[[sp-32]]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x61064 | `r0` | `[sp-36]` |

## `pkw`

- `pkw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61070 to 0x610e8.
  Established. The first call is indirect and receives entry r0 through r3 as candidates (`mov` at
  0x610ac). The exit retains `[sp-36]` (`ldmdb` at 0x610dc). The callee target and write protocol
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61080 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x61084 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x61088 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x6108c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x61090 | load | 4 | `abs 0x610e0` | `-` |
| `ldr` at 0x61094 | load | 4 | `abs 0x2b0920` | `-` |
| `ldr` at 0x61098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6109c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x610a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x610a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x610b4 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x610b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x610bc | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x610c0 | load | 4 | `abs 0x610e4` | `-` |
| `ldr` at 0x610c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x610c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x610cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x610d4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x610dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x610dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x610dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x610ac | `(unknown callee) (unknown target)` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x610d0 | `(unknown callee) 0x2070c` | `0x10c8e4` | `[sp-24]` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x610dc | `r0` | `[sp-36]` |

## `PktGenRead`

- `PktGenRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63a3c to 0x63b08.
  Established. The call to `0x621d0` has `0x0`, `[sp-20]`, `[sp-24]` and `[sp-28]` as candidates
  (`bl` at 0x63ab4). Nonzero takes the branch and zero falls through (`bne` at 0x63ac8). The final
  r0 expression and packet-generator read contract remain unknown (`ldmdb` at 0x63b04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63a4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x63a50 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x63a54 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x63a58 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x63a68 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x63a6c | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x63a7c | load | 4 | `sp +0x4` | `-` |
| `str` at 0x63a80 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x63a84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63a88 | store | 4 | `sp -0x44` | `[sp-32]` |
| `str` at 0x63a90 | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0x63a98 | store | 4 | `sp -0x3c` | `sp-44` |
| `str` at 0x63aa0 | store | 4 | `sp -0x38` | `sp-52` |
| `ldr` at 0x63aa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63aac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x63ab0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x63abc | store | 4 | `sp -0x24` | `ret(0x621d0)` |
| `ldr` at 0x63ac0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x63acc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x63adc | load | 4 | `(unknown)` | `-` |
| `str` at 0x63ae0 | store | 4 | `[sp+8] +0x0` | `?` |
| `ldr` at 0x63ae4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x63af4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x63af8 | store | 4 | `[sp+12] +0x0` | `?` |
| `ldr` at 0x63afc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x63b04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63b04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63b04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63ab4 | `(unknown callee) 0x621d0` | `0x0` | `[sp-20]` | `[sp-24]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x63ac8 | `ret(0x621d0) != 0x0` | `ret(0x621d0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63b04 | `r0` | `?` |

## `PktGenWrite`

- `PktGenWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63b08 to 0x63bc0.
  Established. The only call targets `0x621d0` with `0x1`, `[sp-20]`, `[sp-24]` and `[sp-28]` as
  candidates (`bl` at 0x63ba8). The exit carries `ret(0x621d0)` (`ldmdb` at 0x63bbc). The command
  selector's write meaning remains unproved in this wrapper.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63b18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x63b1c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x63b20 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x63b24 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x63b34 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x63b38 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x63b48 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x63b4c | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0x63b5c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x63b60 | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0x63b70 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x63b74 | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0x63b78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63b7c | store | 4 | `sp -0x44` | `[sp-32]` |
| `str` at 0x63b84 | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0x63b8c | store | 4 | `sp -0x3c` | `sp-44` |
| `str` at 0x63b94 | store | 4 | `sp -0x38` | `sp-52` |
| `ldr` at 0x63b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x63ba0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x63ba4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x63bb0 | store | 4 | `sp -0x24` | `ret(0x621d0)` |
| `ldr` at 0x63bb4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x63bbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63bbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63bbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63ba8 | `(unknown callee) 0x621d0` | `0x1` | `[sp-20]` | `[sp-24]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63bbc | `r0` | `ret(0x621d0)` |

## `TRIGSyndToBit`

- `TRIGSyndToBit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0504 to 0xc057c.
  Established. Word stores set `sp -0x18` to `0x7f` and clear `sp -0x1c` (`str` at 0xc051c, `str` at
  0xc0524). The loop decision has unknown operands (`bls` at 0xc0530). The return expression is
  unknown (`ldmdb` at 0xc0574). Syndrome-to-bit interpretation remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0514 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc051c | store | 4 | `sp -0x18` | `0x7f` |
| `str` at 0xc0524 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc0528 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0538 | load | 4 | `abs 0xc0578` | `-` |
| `ldr` at 0xc053c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0544 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0550 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0554 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc055c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0564 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc056c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc0574 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0574 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0574 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc0530 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc054c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0574 | `r0` | `?` |

## `TRIGBitToSide`

- `TRIGBitToSide`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc057c to 0xc0634.
  Established. A word store clears `sp -0x18` (`str` at 0xc0594). The first branch takes `r0 <= 0xb
  (unsigned, 32-bit)` and falls through above the bound (`bls` at 0xc05a0). A later word store
  writes `0x1` at `sp -0x18` (`str` at 0xc0624). The side mapping and merged return remain unknown
  (`ldmdb` at 0xc0630).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc058c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0594 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc0598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc05f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0624 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xc0628 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc0630 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0630 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0630 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc05a0 | `r0 <= 0xb (unsigned, 32-bit)` | `r0 > 0xb (unsigned, 32-bit)` |
| `bls` at 0xc05ac | `r0 <= 0xf (unsigned, 32-bit)` | `r0 > 0xf (unsigned, 32-bit)` |
| `bls` at 0xc05b8 | `r0 <= 0x17 (unsigned, 32-bit)` | `r0 > 0x17 (unsigned, 32-bit)` |
| `bls` at 0xc05c4 | `r0 <= 0x19 (unsigned, 32-bit)` | `r0 > 0x19 (unsigned, 32-bit)` |
| `bls` at 0xc05d0 | `r0 <= 0x1d (unsigned, 32-bit)` | `r0 > 0x1d (unsigned, 32-bit)` |
| `bls` at 0xc05dc | `r0 <= 0x2f (unsigned, 32-bit)` | `r0 > 0x2f (unsigned, 32-bit)` |
| `bls` at 0xc05e8 | `r0 <= 0x37 (unsigned, 32-bit)` | `r0 > 0x37 (unsigned, 32-bit)` |
| `bls` at 0xc05f4 | `r0 <= 0x39 (unsigned, 32-bit)` | `r0 > 0x39 (unsigned, 32-bit)` |
| `bls` at 0xc0600 | `r0 <= 0x3d (unsigned, 32-bit)` | `r0 > 0x3d (unsigned, 32-bit)` |
| `bls` at 0xc060c | `r0 <= 0x3f (unsigned, 32-bit)` | `r0 > 0x3f (unsigned, 32-bit)` |
| `bhi` at 0xc0618 | `r0 > 0x3f (unsigned, 32-bit)` | `r0 <= 0x3f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0630 | `r0` | `?` |

## `TRIGFindAddrIndex`

- `TRIGFindAddrIndex`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0634 to 0xc0710.
  Established. Four word saves retain entry r0 through r3 at `sp -0x14`, `sp -0x18`, `sp -0x1c` and
  `sp -0x20` (`str` at 0xc0644, `str` at 0xc0648, `str` at 0xc064c, `str` at 0xc0650). The loop
  comparison has unknown operands (`bls` at 0xc0674). The returned index and record geometry remain
  unknown (`ldmdb` at 0xc070c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0644 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0648 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc064c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc0650 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc0654 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc065c | store | 4 | `sp -0x28` | `r2+1` |
| `str` at 0xc0664 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc0668 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc066c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc067c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0690 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0698 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc06ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc06b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc06cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc06d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc06e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc06ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc06f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc06f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc06fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc0704 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xc070c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc070c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc070c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc0674 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc06b4 | `? != r0` | `? == r0` |
| `bne` at 0xc06e8 | `? != r1` | `? == r1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc070c | `r0` | `?` |

## `FRUEventCreate`

- `FRUEventCreate`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc08d8 to 0xc0a7c.
  Established. The first call to `0x1b124` receives `0xac`, entry r1, entry r2 and entry r3 as
  candidates (`bl` at 0xc08fc); its nonzero result takes the branch and zero falls through (`bne` at
  0xc0910). The merged r0 value and event-record contract remain unknown (`ldmdb` at 0xc0a78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc08e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc08ec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc08f0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc08f4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xc0904 | store | 4 | `sp -0x24` | `ret(0x1b124)` |
| `ldr` at 0xc0908 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0918 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc091c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0928 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc092c | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc0930 | store | 1 | `[sp-36] +0x8` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc0934 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0938 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xc093c | store | 1 | `[sp-36] +0x9` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xc0940 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0944 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xc0948 | store | 1 | `[sp-36] +0xa` | `([sp]:1 & 0xff)` |
| `ldr` at 0xc094c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0950 | load | 1 | `sp +0x4` | `-` |
| `strb` at 0xc0954 | store | 1 | `[sp-36] +0xb` | `([sp+4]:1 & 0xff)` |
| `ldr` at 0xc0958 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc095c | load | 1 | `sp +0x8` | `-` |
| `strb` at 0xc0960 | store | 1 | `[sp-36] +0xc` | `([sp+8]:1 & 0xff)` |
| `ldr` at 0xc0964 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0968 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc096c | store | 4 | `[sp-36] +0x10` | `[sp+12]` |
| `ldr` at 0xc0970 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0974 | load | 1 | `sp +0x10` | `-` |
| `strb` at 0xc0978 | store | 1 | `[sp-36] +0xd` | `([sp+16]:1 & 0xff)` |
| `ldr` at 0xc097c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc0980 | load | 2 | `sp +0x14` | `-` |
| `strh` at 0xc0984 | store | 2 | `[sp-36] +0x18` | `([sp+20]:2 & 0xffff)` |
| `ldr` at 0xc0988 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc098c | load | 2 | `sp +0x18` | `-` |
| `strh` at 0xc0990 | store | 2 | `[sp-36] +0x1a` | `([sp+24]:2 & 0xffff)` |
| `ldr` at 0xc0994 | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xc099c | store | 2 | `[sp-36] +0x1c` | `0x0` |
| `ldr` at 0xc09a0 | load | 4 | `sp -0x24` | `-` |
| `strh` at 0xc09a8 | store | 2 | `[sp-36] +0x1e` | `0x0` |
| `ldr` at 0xc09ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc09b0 | load | 1 | `sp +0x1c` | `-` |
| `strb` at 0xc09b4 | store | 1 | `[sp-36] +0x14` | `([sp+28]:1 & 0xff)` |
| `ldr` at 0xc09b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc09bc | load | 1 | `sp +0x20` | `-` |
| `strb` at 0xc09c0 | store | 1 | `[sp-36] +0x20` | `([sp+32]:1 & 0xff)` |
| `ldr` at 0xc09c4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc09c8 | load | 1 | `sp +0x24` | `-` |
| `strb` at 0xc09cc | store | 1 | `[sp-36] +0x15` | `([sp+36]:1 & 0xff)` |
| `ldr` at 0xc09d0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc09d4 | load | 1 | `sp +0x28` | `-` |
| `strb` at 0xc09d8 | store | 1 | `[sp-36] +0x21` | `([sp+40]:1 & 0xff)` |
| `ldr` at 0xc09dc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc09e0 | load | 1 | `sp +0x2c` | `-` |
| `strb` at 0xc09e4 | store | 1 | `[sp-36] +0x22` | `([sp+44]:1 & 0xff)` |
| `ldr` at 0xc09e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc09ec | load | 1 | `sp +0x30` | `-` |
| `strb` at 0xc09f0 | store | 1 | `[sp-36] +0x23` | `([sp+48]:1 & 0xff)` |
| `ldr` at 0xc09f4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc09f8 | load | 1 | `sp +0x34` | `-` |
| `strb` at 0xc09fc | store | 1 | `[sp-36] +0x24` | `([sp+52]:1 & 0xff)` |
| `ldr` at 0xc0a00 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0a04 | load | 1 | `sp +0x38` | `-` |
| `strb` at 0xc0a08 | store | 1 | `[sp-36] +0x26` | `([sp+56]:1 & 0xff)` |
| `ldr` at 0xc0a0c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc0a10 | load | 1 | `sp +0x3c` | `-` |
| `strb` at 0xc0a14 | store | 1 | `[sp-36] +0x25` | `([sp+60]:1 & 0xff)` |
| `ldr` at 0xc0a18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0a24 | load | 4 | `sp +0x40` | `-` |
| `ldr` at 0xc0a30 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc0a38 | store | 1 | `[sp-36] +0x67` | `0x0` |
| `ldr` at 0xc0a3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0a48 | load | 4 | `sp +0x44` | `-` |
| `ldr` at 0xc0a54 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xc0a5c | store | 1 | `[sp-36] +0xa7` | `0x0` |
| `ldr` at 0xc0a60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0a68 | store | 4 | `[sp-36] +0xa8` | `0x0` |
| `ldr` at 0xc0a6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0a70 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc0a78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0a78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0a78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc08fc | `(unknown callee) 0x1b124` | `0xac` | `r1` | `r2` | `r3` |
| `bl` at 0xc0924 | `(unknown callee) 0x1b304` | `ret(0x1b124)` | `[sp-24]` | `0x8` | `ret(0x1b124)` |
| `bl` at 0xc0a2c | `(unknown callee) 0x1b454` | `[sp-36]+40` | `[sp+64]` | `0x40` | `[sp-36]+40` |
| `bl` at 0xc0a50 | `(unknown callee) 0x1b454` | `[sp-36]+104` | `[sp+68]` | `0x40` | `[sp-36]+104` |
| `bl` at 0xc0a74 | `(unknown callee) 0xc0788` | `[sp-20]` | `[sp-36]` | `[sp-36]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0910 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0a78 | `r0` | `?` |

## `FRUEventRank`

- `FRUEventRank`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0a7c to 0xc0d74.
  Established. Word stores clear `sp -0x20` and `sp -0x24` (`str` at 0xc0a94, `str` at 0xc0a9c). The
  first branch takes `Z == 0` and falls through on `not (Z == 0)` (`bne` at 0xc0ac0). The call to
  `0xbd340` has unknown, `0x3f`, `0x20` and unknown as candidates (`bl` at 0xc0ad4). Ranking
  semantics and the final value remain unknown (`ldmdb` at 0xc0d70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0a8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0a94 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xc0a9c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xc0aa4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc0aa8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0aac | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc0ab0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0ab4 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xc0ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ac8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ae4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b00 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc0b04 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc0b10 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc0b14 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc0b20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc0b28 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xc0b2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b30 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xc0b34 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xc0b3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0b40 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc0b44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0b8c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc0b90 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc0b9c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc0ba0 | load | 1 | `[sp-20] +0x26` | `-` |
| `ldr` at 0xc0bac | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0bb0 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xc0bb4 | store | 2 | `[sp-20] +0x1c` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xc0bb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc0bc0 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0xc0bc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0bc8 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `str` at 0xc0bd0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc0bd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0be4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0c00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0c1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0c20 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xc0c2c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc0c30 | load | 1 | `[sp-28] +0x26` | `-` |
| `ldr` at 0xc0c3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0c40 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0c44 | load | 2 | `[sp-28] +0x18` | `-` |
| `ldrh` at 0xc0c48 | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xc0c54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0c58 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0c5c | load | 2 | `[sp-28] +0x1a` | `-` |
| `ldrh` at 0xc0c60 | load | 2 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xc0c70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0c78 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc0c7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0c80 | load | 4 | `[sp-28] +0xa8` | `-` |
| `str` at 0xc0c84 | store | 4 | `sp -0x1c` | `[[sp-28]+168]` |
| `ldr` at 0xc0c8c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0c90 | load | 2 | `sp -0x24` | `-` |
| `strh` at 0xc0c94 | store | 2 | `[sp-20] +0x1e` | `([sp-36]:2 & 0xffff)` |
| `ldr` at 0xc0c98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0c9c | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xc0ca0 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldr` at 0xc0ca8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0cac | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc0cb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0cc4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc0cc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0ccc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc0cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0ce0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0ce4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0ce8 | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xc0cec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0cf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0cfc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0d00 | load | 2 | `(unknown)` | `-` |
| `ldrh` at 0xc0d04 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0d14 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0d18 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0d24 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc0d28 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0d2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc0d38 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc0d3c | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0d40 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc0d44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0d48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0d4c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc0d54 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xc0d58 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xc0d5c | store | 2 | `(unknown)` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xc0d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0d64 | load | 4 | `[sp-20] +0xa8` | `-` |
| `str` at 0xc0d68 | store | 4 | `sp -0x14` | `[[sp-20]+168]` |
| `ldmdb` at 0xc0d70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0d70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0d70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0ad4 | `(unknown callee) 0xbd340` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xc0af0 | `(unknown callee) 0xbd340` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xbd340)` |
| `bl` at 0xc0b60 | `(unknown callee) 0xbd340` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xc0b7c | `(unknown callee) 0xbd340` | `[sp-20]` | `0x1f` | `0x0` | `ret(0xbd340)` |
| `bl` at 0xc0bf0 | `(unknown callee) 0xbd340` | `?` | `0x3f` | `0x20` | `?` |
| `bl` at 0xc0c0c | `(unknown callee) 0xbd340` | `[sp-28]` | `0x1f` | `0x0` | `ret(0xbd340)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0ac0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc0ae0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0afc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0b0c | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xc0b1c | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `bne` at 0xc0b4c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc0b6c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0b88 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0b98 | `[[sp-20]+38]:1 == 0x4` | `[[sp-20]+38]:1 != 0x4` |
| `beq` at 0xc0ba8 | `[[sp-20]+38]:1 == 0x8` | `[[sp-20]+38]:1 != 0x8` |
| `bne` at 0xc0bdc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc0bfc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0c18 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0c28 | `[[sp-28]+38]:1 == 0x4` | `[[sp-28]+38]:1 != 0x4` |
| `beq` at 0xc0c38 | `[[sp-28]+38]:1 == 0x8` | `[[sp-28]+38]:1 != 0x8` |
| `beq` at 0xc0c50 | `[[sp-28]+24]:2 == [[sp-20]+24]:2` | `[[sp-28]+24]:2 != [[sp-20]+24]:2` |
| `beq` at 0xc0c68 | `[[sp-28]+26]:2 == [[sp-20]+24]:2` | `[[sp-28]+26]:2 != [[sp-20]+24]:2` |
| `bne` at 0xc0cb8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc0cd8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc0cf4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0d0c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0d20 | `Z == 1` | `not (Z == 1)` |
| `bls` at 0xc0d34 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0d70 | `r0` | `?` |

## `FRUEventPrint`

- `FRUEventPrint`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0d74 to 0xc1138.
  Established. The entry-r0 guard takes nonzero and falls through on zero (`bne` at 0xc0dac). The
  opening call to `0x1b49c` carries `sp-164`, `0x132674`, entry r2 and `sp-164` as candidates (`bl`
  at 0xc0dc8). The final r0 expression and fault-event output format remain unknown (`ldmdb` at
  0xc10fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0d84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0d88 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc0d8c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc0d90 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xc0d98 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0xc0d9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc0da0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xc0da4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0db0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0dc4 | load | 4 | `abs 0xc1100` | `-` |
| `ldr` at 0xc0de4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0de8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0dfc | load | 4 | `abs 0xc1104` | `-` |
| `str` at 0xc0e00 | store | 4 | `sp -0xb8` | `0x1326a0` |
| `ldr` at 0xc0e04 | load | 4 | `abs 0xc1108` | `-` |
| `str` at 0xc0e08 | store | 4 | `sp -0xb4` | `0x1326a8` |
| `ldr` at 0xc0e0c | load | 4 | `abs 0xc110c` | `-` |
| `str` at 0xc0e10 | store | 4 | `sp -0xb0` | `0x1326b0` |
| `ldr` at 0xc0e14 | load | 4 | `abs 0xc1110` | `-` |
| `str` at 0xc0e18 | store | 4 | `sp -0xac` | `0x1326b8` |
| `ldr` at 0xc0e20 | load | 4 | `abs 0xc1114` | `-` |
| `ldr` at 0xc0e24 | load | 4 | `abs 0xc1118` | `-` |
| `ldr` at 0xc0e28 | load | 4 | `abs 0xc111c` | `-` |
| `ldr` at 0xc0e48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0e60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0e6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0e80 | load | 4 | `abs 0xc1120` | `-` |
| `ldr` at 0xc0ea0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0ea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0eb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ed4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0ed8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0ee4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0ee8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0eec | load | 4 | `abs 0xc1124` | `-` |
| `ldr` at 0xc0efc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0f04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0f0c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f10 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f14 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xc0f18 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f1c | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f20 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xc0f24 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f28 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f2c | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xc0f30 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f34 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f38 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xc0f40 | load | 4 | `abs 0xc1128` | `-` |
| `ldr` at 0xc0f58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0f60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0f68 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f6c | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f70 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xc0f74 | load | 4 | `abs 0xc112c` | `-` |
| `str` at 0xc0f78 | store | 4 | `sp -0xb4` | `0x132754` |
| `ldr` at 0xc0f7c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f80 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f84 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xc0f88 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f8c | load | 2 | `(unknown)` | `-` |
| `str` at 0xc0f90 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xc0f98 | load | 4 | `abs 0xc1130` | `-` |
| `ldr` at 0xc0fc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0fc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0fd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0fe4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0fe8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0fec | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0ff0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0ff8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc1004 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1008 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc100c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc1010 | load | 2 | `(unknown)` | `-` |
| `str` at 0xc1018 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xc101c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc1020 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc1024 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc102c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc1038 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc104c | load | 4 | `abs 0xc1134` | `-` |
| `ldr` at 0xc106c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc1070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc1084 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc1094 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc1098 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc109c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc10ac | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc10b0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc10b4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc10b8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc10c0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xc10cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc10d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc10d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc10dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc10e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc10e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc10ec | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xc10f4 | store | 4 | `sp -0xa8` | `?` |
| `ldmdb` at 0xc10fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc10fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc10fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0dc8 | `(unknown callee) 0x1b49c` | `sp-164` | `0x132674` | `r2` | `sp-164` |
| `bl` at 0xc0dd4 | `(unknown callee) 0x1b22c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc0df4 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b22c)` | `sp-164` |
| `bl` at 0xc0e2c | `(unknown callee) 0x1b49c` | `sp-164` | `0x1326bc` | `0x1326e0` | `0x1326e4` |
| `bl` at 0xc0e38 | `(unknown callee) 0x1b22c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc0e58 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b22c)` | `sp-164` |
| `bl` at 0xc0e84 | `(unknown callee) 0x1b49c` | `sp-164` | `0x1326ec` | `r2` | `sp-164` |
| `bl` at 0xc0e90 | `(unknown callee) 0x1b22c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc0eb0 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b22c)` | `sp-164` |
| `bl` at 0xc0f4c | `(unknown callee) 0x1b49c` | `sp-164` | `0x132730` | `?` | `?` |
| `bl` at 0xc0fa4 | `(unknown callee) 0x1b49c` | `sp-164` | `0x13275c` | `?` | `?` |
| `bl` at 0xc0fb0 | `(unknown callee) 0x1b22c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc0fd0 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b22c)` | `sp-164` |
| `bl` at 0xc1050 | `(unknown callee) 0x1b49c` | `sp-164` | `0x132780` | `?` | `sp-164` |
| `bl` at 0xc105c | `(unknown callee) 0x1b22c` | `sp-164` | `?` | `?` | `sp-164` |
| `mov` at 0xc107c | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-164` | `ret(0x1b22c)` | `sp-164` |
| `bl` at 0xc10d8 | `(unknown callee) 0xc1138` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc0dac | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xc0db8 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xc0e68 | `r0 != 0x2` | `r0 == 0x2` |
| `beq` at 0xc0e74 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xc0ec0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc0ed0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc0ee0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc0ef4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc0fe0 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xc1000 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc1034 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc1040 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc108c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc10a4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc10c8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc10fc | `r0` | `?` |

## `FRUStatus`

- `FRUStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc3f0c to 0xc3fe8.
  Established. The selector takes `r0 <= 0x8 (unsigned, 32-bit)` and falls through above the bound
  (`ldrls` at 0xc3f28). Taken edges remain unresolved. The known fall-through word store writes
  `0x132a64` at `sp -0x18`, and that value reaches the known-edge exit (`str` at 0xc3fb8, `ldmdb` at
  0xc3fc0). Selected status names remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc3f1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc3f20 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc3f28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc3fb4 | load | 4 | `abs 0xc3fe4` | `-` |
| `str` at 0xc3fb8 | store | 4 | `sp -0x18` | `0x132a64` |
| `ldr` at 0xc3fbc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc3fc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc3fc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc3fc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc3f28 | `r0 <= 0x8 (unsigned, 32-bit)` | `r0 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc3fc0 | `r0` | `0x132a64` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc3f28).

## `_FRUDump`

- `_FRUDump`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc3fe8 to 0xc44f8.
  Established. The opening call to `0xc3f0c` receives `[sp+12]`, entry r1, entry r2 and entry r3 as
  candidates (`bl` at 0xc400c). The guard takes `[sp+12] == 0x0` and falls through on nonzero (`beq`
  at 0xc4068). The return carries `ret(0xc3edc)` (`ldmdb` at 0xc44d0). Fault-record output semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc3ff8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc3ffc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc4000 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc4004 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc4008 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc401c | load | 4 | `abs 0xc44d4` | `-` |
| `ldr` at 0xc4028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc402c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4040 | load | 4 | `abs 0xc44d8` | `-` |
| `ldr` at 0xc4044 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4050 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4054 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4060 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc4088 | load | 4 | `abs 0xc44dc` | `-` |
| `ldr` at 0xc4094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4098 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc40a8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc40ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc40b0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xc40c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc40d0 | store | 4 | `sp -0x9c` | `ret(0xc4b30)` |
| `ldr` at 0xc40d8 | load | 4 | `abs 0xc44e0` | `-` |
| `ldr` at 0xc40dc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xc40e0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc40ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc40f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc40fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4110 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4118 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc411c | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xc4120 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc412c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc4138 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc4148 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xc415c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4170 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc417c | load | 4 | `abs 0xc44e4` | `-` |
| `ldr` at 0xc4180 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc41a4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc41b0 | load | 4 | `abs 0xc44e8` | `-` |
| `ldr` at 0xc41b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc41c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc41d8 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc41e4 | load | 4 | `abs 0xc44ec` | `-` |
| `ldr` at 0xc41e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc41f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc41f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc4208 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc420c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4220 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4228 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc422c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4240 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4254 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc425c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4264 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc426c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4280 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4288 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4290 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4298 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc429c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc42b0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc42b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc42c0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc42c8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc42cc | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0xc42d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc42e4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc42ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc42f4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc42fc | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4300 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0xc4304 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4318 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4320 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4328 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4330 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4334 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0xc4338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc434c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4354 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc435c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4364 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4368 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xc436c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc4380 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc4388 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4390 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4398 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc439c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xc43a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc43b4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc43bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc43c4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc43cc | load | 1 | `(unknown)` | `-` |
| `str` at 0xc43d0 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xc43d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc43e8 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0xc43f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc43f8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xc4400 | load | 1 | `(unknown)` | `-` |
| `str` at 0xc4404 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0xc4408 | load | 4 | `abs 0xc44f0` | `-` |
| `ldr` at 0xc440c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc441c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc442c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4434 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xc443c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc4444 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc444c | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc4458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc445c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4460 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0xc4468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc446c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4470 | load | 4 | `abs 0xc44f4` | `-` |
| `ldr` at 0xc4478 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc4484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc448c | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0xc4494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc449c | load | 4 | `abs 0xc44f4` | `-` |
| `ldr` at 0xc44a4 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc44b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc44b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc44b8 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0xc44c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc44c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc44c8 | load | 4 | `abs 0xc44f4` | `-` |
| `ldmdb` at 0xc44d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc44d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc44d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc400c | `(unknown callee) 0xc3f0c` | `[sp+12]` | `r1` | `r2` | `r3` |
| `bl` at 0xc4020 | `(unknown callee) 0x1b49c` | `sp-124` | `0x132a78` | `ret(0xc3f0c)` | `sp-124` |
| `bl` at 0xc4034 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4048 | `(unknown callee) 0x1b49c` | `sp-124` | `0x132a88` | `[sp-28]` | `sp-124` |
| `bl` at 0xc405c | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4078 | `(unknown callee) 0x1b334` | `sp-32` | `?` | `?` | `sp-32` |
| `bl` at 0xc408c | `(unknown callee) 0x1b49c` | `sp-124` | `0x132a9c` | `ret(0x1b334)` | `sp-124` |
| `bl` at 0xc40a0 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc40c4 | `(unknown callee) 0xc4b30` | `[sp]` | `?` | `?` | `[sp+16]` |
| `bl` at 0xc40e4 | `(unknown callee) 0x1b49c` | `sp-124` | `0x132aac` | `[sp+4]` | `[sp+8]` |
| `bl` at 0xc40f8 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4184 | `(unknown callee) 0x1b49c` | `sp-124` | `0x132abc` | `?` | `?` |
| `bl` at 0xc41b8 | `(unknown callee) 0x1b49c` | `sp-124` | `0x132ac8` | `?` | `?` |
| `bl` at 0xc41ec | `(unknown callee) 0x1b49c` | `sp-124` | `0x132ad4` | `?` | `?` |
| `bl` at 0xc4200 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4414 | `(unknown callee) 0x1b49c` | `sp-124` | `0x132adc` | `?` | `?` |
| `bl` at 0xc4428 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `sp-124` | `sp-124` |
| `bl` at 0xc4464 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `[sp+24]` | `[sp+24]` |
| `bl` at 0xc4474 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `0x132998` | `?` |
| `bl` at 0xc4490 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `[sp+28]` | `[sp+28]` |
| `bl` at 0xc44a0 | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `0x132998` | `?` |
| `bl` at 0xc44bc | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `[sp+32]` | `[sp+32]` |
| `bl` at 0xc44cc | `(unknown callee) 0xc3edc` | `[sp-20]` | `[sp-24]` | `0x132998` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc4068 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `blt` at 0xc40b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc4128 | `[sp-128] == 0x2` | `[sp-128] != 0x2` |
| `bgt` at 0xc4134 | `[sp-128] > 0x2 (signed, 32-bit)` | `[sp-128] <= 0x2 (signed, 32-bit)` |
| `beq` at 0xc4140 | `[sp-128] == 0x1` | `[sp-128] != 0x1` |
| `beq` at 0xc4150 | `[sp-128] == 0x3` | `[sp-128] != 0x3` |
| `blt` at 0xc4234 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc4454 | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `beq` at 0xc4480 | `[sp+28] == 0x0` | `[sp+28] != 0x0` |
| `beq` at 0xc44ac | `[sp+32] == 0x0` | `[sp+32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc44d0 | `r0` | `ret(0xc3edc)` |

## `FRUDump`

- `FRUDump`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc44f8 to 0xc4714.
  Established. The first call targets `0xc2ee4` with entry r2, `sp-32`, `sp-36` and `sp-40` as
  candidates (`bl` at 0xc4590). The first branch takes `[sp-64] == 0x0` and falls through on nonzero
  (`beq` at 0xc4600). The final r0 expression and decoded record layout remain unknown (`ldmdb` at
  0xc4710).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4508 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc450c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc4510 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc4518 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xc4520 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xc4528 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xc453c | store | 4 | `sp -0xc4` | `sp-44` |
| `str` at 0xc4544 | store | 4 | `sp -0xc0` | `sp-48` |
| `str` at 0xc454c | store | 4 | `sp -0xbc` | `sp-52` |
| `str` at 0xc4554 | store | 4 | `sp -0xb8` | `sp-56` |
| `str` at 0xc455c | store | 4 | `sp -0xb4` | `sp-60` |
| `str` at 0xc4564 | store | 4 | `sp -0xb0` | `sp-64` |
| `str` at 0xc456c | store | 4 | `sp -0xac` | `sp-72` |
| `str` at 0xc4574 | store | 4 | `sp -0xa8` | `sp-76` |
| `str` at 0xc457c | store | 4 | `sp -0xa4` | `sp-80` |
| `ldr` at 0xc4580 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4594 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc4598 | store | 4 | `sp -0xc4` | `[sp-44]` |
| `ldr` at 0xc459c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc45a0 | store | 4 | `sp -0xc0` | `[sp-48]` |
| `ldr` at 0xc45a4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xc45a8 | store | 4 | `sp -0xbc` | `[sp-52]` |
| `ldr` at 0xc45ac | load | 4 | `sp -0x38` | `-` |
| `str` at 0xc45b0 | store | 4 | `sp -0xb8` | `[sp-56]` |
| `ldr` at 0xc45b4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc45b8 | store | 4 | `sp -0xb4` | `[sp-60]` |
| `ldr` at 0xc45bc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc45c0 | store | 4 | `sp -0xb0` | `[sp-64]` |
| `ldr` at 0xc45c4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc45c8 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0xc45cc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xc45d0 | store | 4 | `sp -0xa8` | `[sp-76]` |
| `ldr` at 0xc45d4 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xc45d8 | store | 4 | `sp -0xa4` | `[sp-80]` |
| `ldr` at 0xc45dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc45e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc45e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc45e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc45f4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xc45f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4604 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4608 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xc4618 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc462c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4634 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4640 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4654 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc465c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4664 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc4678 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc4680 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc468c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xc46a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc46a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc46b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xc46b8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xc46c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc46cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xc46d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc46e0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xc46e8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc46f4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xc46fc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xc4708 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xc4710 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4710 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4710 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4590 | `(unknown callee) 0xc2ee4` | `r2` | `sp-32` | `sp-36` | `sp-40` |
| `bl` at 0xc45ec | `(unknown callee) 0xc3fe8` | `[sp-20]` | `[sp-24]` | `[sp-32]` | `[sp-36]` |
| `bl` at 0xc4660 | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |
| `bl` at 0xc46ac | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |
| `bl` at 0xc46d0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `[sp-64]` |
| `bl` at 0xc46e4 | `(unknown callee) 0x1b400` | `[sp-72]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xc46f8 | `(unknown callee) 0x1b400` | `[sp-76]` | `?` | `?` | `[sp-76]` |
| `bl` at 0xc470c | `(unknown callee) 0x1b400` | `[sp-80]` | `?` | `?` | `[sp-80]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc4600 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `blt` at 0xc4610 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xc463c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc4688 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc46c8 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `beq` at 0xc46dc | `[sp-72] == 0x0` | `[sp-72] != 0x0` |
| `beq` at 0xc46f0 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `beq` at 0xc4704 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4710 | `r0` | `?` |

## `FRUDumpBuff`

- `FRUDumpBuff`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4714 to 0xc4760.
  Established. The opening call to `0xc53dc` has entry r0 through r3 as candidates (`bl` at
  0xc4728). A word store saves `ret(0xc53dc)` at `sp -0x18` (`str` at 0xc4730). The exit carries
  `ret(0xc542c)` (`ldmdb` at 0xc4758). The buffer ownership contract remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4724 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc4730 | store | 4 | `sp -0x18` | `ret(0xc53dc)` |
| `ldr` at 0xc4734 | load | 4 | `abs 0xc475c` | `-` |
| `ldr` at 0xc4738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc473c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4744 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc4758 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4758 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4758 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4728 | `(unknown callee) 0xc53dc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc4740 | `(unknown callee) 0xc44f8` | `0xc5498` | `ret(0xc53dc)` | `[sp-20]` | `ret(0xc53dc)` |
| `bl` at 0xc474c | `(unknown callee) 0xc542c` | `[sp-24]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4758 | `r0` | `ret(0xc542c)` |

## `FruFmtMultiRec`

- `FruFmtMultiRec`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3268 to 0xb39a4.
  Established. The selector takes `[sp-28] <= 0x5 (unsigned, 32-bit)` and falls through above that
  bound (`ldrls` at 0xb32a0); its taken edges remain unresolved. Entry r0 is preserved at the
  known-edge exit (`ldmdb` at 0xb3944). The selected multi-record formatting and field units remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb3278 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xb327c | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xb3280 | store | 4 | `sp -0x28` | `r3` |
| `strb` at 0xb3288 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xb328c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xb3294 | store | 1 | `r3 +0x0` | `0x0` |
| `ldr` at 0xb3298 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0xb32a0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xb3944 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb3944 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb3944 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb3944 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb3944 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb32a0 | `[sp-28] <= 0x5 (unsigned, 32-bit)` | `[sp-28] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb3944 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb32a0).
