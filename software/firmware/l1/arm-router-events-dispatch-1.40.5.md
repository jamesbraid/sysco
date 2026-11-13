# ARM router events and dispatch in 1.40.5

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
L1CfgQuery, B2BInit and B2BReqRsp have unresolved computed transfers. Tables cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/router-events-dispatch-facts.txt) retain all tool sections.

## `EvtInit`

- `EvtInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6963c to 0x69714.
  Established. A four-byte absolute assignment at `0x149378` writes `0x1` (`str` at 0x696ec). The
  earlier branch takes `[0x149378] != 0x0` to `0x696f0` (`bne` at 0x69694). The unnamed `0xb4a70`
  call has `0x14937c`, `0xffff001`, `0x1`, `0x69714` (`bl` at 0x696a8). Event registration behavior
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6964c | load | 4 | `abs 0x696f4` | `-` |
| `ldr` at 0x6965c | load | 4 | `abs 0x696f8` | `-` |
| `ldr` at 0x69660 | load | 4 | `abs 0x14356c` | `-` |
| `ldr` at 0x6966c | load | 4 | `abs 0x696fc` | `-` |
| `str` at 0x69670 | store | 4 | `sp -0x18` | `0x69840` |
| `str` at 0x6967c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x69680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69688 | load | 4 | `abs 0x69700` | `-` |
| `ldr` at 0x6968c | load | 4 | `abs 0x149378` | `-` |
| `ldr` at 0x69698 | load | 4 | `abs 0x696f4` | `-` |
| `ldr` at 0x6969c | load | 4 | `abs 0x69704` | `-` |
| `ldr` at 0x696a4 | load | 4 | `abs 0x69708` | `-` |
| `str` at 0x696b0 | store | 4 | `sp -0x14` | `ret(0xb4a70)` |
| `ldr` at 0x696b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x696c0 | load | 4 | `abs 0x6970c` | `-` |
| `str` at 0x696c4 | store | 4 | `sp -0x20` | `0x10f9e4` |
| `ldr` at 0x696c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x696cc | store | 4 | `sp -0x1c` | `ret(0xb4a70)` |
| `ldr` at 0x696d4 | load | 4 | `abs 0x69710` | `-` |
| `ldr` at 0x696e4 | load | 4 | `abs 0x69700` | `-` |
| `str` at 0x696ec | store | 4 | `abs 0x149378` | `0x1` |
| `ldmdb` at 0x696f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x696f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x696f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69658 | `(unknown callee) 0xb5e34` | `0x14937c` | `0x1` | `0x0` | `r3` |
| `bl` at 0x69684 | `(unknown callee) 0x24548` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x696a8 | `(unknown callee) 0xb4a70` | `0x14937c` | `0xffff001` | `0x1` | `0x69714` |
| `bl` at 0x696e0 | `(unknown callee) 0x21eb8` | `0x9` | `0x10fa10` | `0x43` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x69668 | `[0x14356c] == 0x0` | `[0x14356c] != 0x0` |
| `bne` at 0x69694 | `[0x149378] != 0x0` | `[0x149378] == 0x0` |
| `beq` at 0x696bc | `ret(0xb4a70) == 0x0` | `ret(0xb4a70) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x696f0 | `r0` | `?` |

## `L1CfgQuery`

- `L1CfgQuery`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x73334 to 0x74f0c.
  Established. The unresolved computed transfer takes unsigned `[[sp-60]+56] <= 0x5` and falls
  through for `[[sp-60]+56] > 0x5` (`ldrls` at 0x73820). Included four-byte absolute assignments at
  `0x1493f0` and `0x1493f8` retain `ret(0x1b274)` and `0x0` respectively (`str` at 0x74138, `str` at
  0x74144). The unnamed `0x1d41c` call has `0x9a`, `0x1`, unknown, unknown (`bl` at 0x74854).
  Configuration selectors and output record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x73344 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x73348 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x7334c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7336c | load | 4 | `abs 0x74280` | `-` |
| `ldr` at 0x73374 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73378 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x7337c | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x733a0 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x733a4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x733a8 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x733b0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x733b4 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x733b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x733bc | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x733cc | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x733d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x733e4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x733ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x733f0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x733f4 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x733f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7340c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x73414 | load | 4 | `(unknown)` | `-` |
| `str` at 0x73418 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x7341c | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73420 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73434 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7343c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73448 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7345c | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73474 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7347c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73494 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73498 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x734ac | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x734b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x734c0 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x734c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x734d8 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x734e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x734e4 | load | 4 | `abs 0x74288` | `-` |
| `ldr` at 0x734e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x734f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x734f8 | load | 4 | `abs 0x7428c` | `-` |
| `ldr` at 0x73500 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73510 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73518 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x7351c | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73520 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73534 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7353c | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73540 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73554 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7355c | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73560 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73574 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7357c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73580 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73584 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73588 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73590 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x735a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x735a8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x735ac | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x735b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x735c4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x735cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x735d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x735dc | load | 4 | `abs 0x74290` | `-` |
| `ldr` at 0x735e8 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x735ec | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x735f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73604 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x7360c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x73610 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7361c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73620 | load | 4 | `abs 0x74294` | `-` |
| `ldr` at 0x7362c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7363c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73644 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73658 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x7365c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73670 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x73678 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7367c | load | 4 | `abs 0x74298` | `-` |
| `ldr` at 0x73680 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73688 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73698 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x736a0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x736a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x736b4 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x736b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x736cc | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x736d4 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x736d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x736ec | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x736f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x736f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x73704 | store | 4 | `sp -0x3c` | `ret(0x74f94)` |
| `ldr` at 0x73708 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x7370c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73720 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x73728 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73734 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73738 | load | 4 | `abs 0x7429c` | `-` |
| `ldr` at 0x73744 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73748 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7375c | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x73764 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73770 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73774 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73788 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x73790 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73794 | load | 4 | `abs 0x742a0` | `-` |
| `ldr` at 0x73798 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x737a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x737a8 | load | 4 | `abs 0x742a4` | `-` |
| `ldr` at 0x737b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x737c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x737c8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x737cc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x737d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x737dc | load | 4 | `abs 0x742a8` | `-` |
| `ldr` at 0x737e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x737ec | load | 4 | `[sp-60] +0x34` | `-` |
| `ldr` at 0x737f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x737fc | load | 4 | `abs 0x742ac` | `-` |
| `ldr` at 0x73808 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7380c | load | 4 | `abs 0x742b0` | `-` |
| `ldr` at 0x73814 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73818 | load | 4 | `[sp-60] +0x38` | `-` |
| `ldrls` at 0x73820 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x738a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x738a4 | load | 4 | `abs 0x742cc` | `-` |
| `ldr` at 0x738ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x738b0 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x738c0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x738c4 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x738d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x738d8 | load | 4 | `abs 0x742d0` | `-` |
| `ldr` at 0x738e4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x738e8 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x738f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x738fc | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x7390c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73910 | load | 4 | `abs 0x742d4` | `-` |
| `ldr` at 0x7391c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73920 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x73930 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73934 | load | 4 | `abs 0x742d8` | `-` |
| `ldr` at 0x73940 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73944 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0x73954 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73958 | load | 4 | `abs 0x742dc` | `-` |
| `ldr` at 0x73964 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73968 | load | 4 | `abs 0x742e0` | `-` |
| `ldr` at 0x73970 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73974 | load | 4 | `abs 0x74368` | `-` |
| `ldr` at 0x7397c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7398c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73994 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73998 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x739a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x739a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x739b8 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x739bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x739c0 | load | 4 | `abs 0x742e4` | `-` |
| `ldr` at 0x739c4 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x739cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x739dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x739e4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x739e8 | load | 4 | `abs 0x742e8` | `-` |
| `ldr` at 0x739ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x739f0 | load | 4 | `abs 0x742ec` | `-` |
| `ldr` at 0x739f4 | load | 4 | `abs 0x1493dc` | `-` |
| `ldr` at 0x739fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73a0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73a14 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73a18 | load | 4 | `abs 0x742f0` | `-` |
| `ldr` at 0x73a1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73a20 | load | 4 | `abs 0x742f4` | `-` |
| `ldr` at 0x73a24 | load | 4 | `abs 0x1493e0` | `-` |
| `ldr` at 0x73a2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73a3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73a44 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73a48 | load | 4 | `abs 0x742f8` | `-` |
| `ldr` at 0x73a4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73a50 | load | 4 | `abs 0x742fc` | `-` |
| `ldr` at 0x73a54 | load | 4 | `abs 0x1493e4` | `-` |
| `ldr` at 0x73a5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73a6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73a74 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73a80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73a90 | load | 4 | `abs 0x74300` | `-` |
| `ldr` at 0x73a98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73a9c | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73aa0 | load | 4 | `abs 0x149730` | `-` |
| `str` at 0x73acc | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x73ad0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x73ad4 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x73ad8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73adc | load | 4 | `abs 0x74304` | `-` |
| `ldr` at 0x73ae4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73af4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73afc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73b00 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73b04 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73b08 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73b0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73b10 | load | 4 | `abs 0x1496b4` | `-` |
| `ldr` at 0x73b14 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x73b18 | load | 4 | `abs 0x1496b8` | `-` |
| `ldr` at 0x73b20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73b24 | load | 4 | `abs 0x74368` | `-` |
| `ldr` at 0x73b2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73b3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73b44 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `str` at 0x73b4c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x73b50 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73b54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73b58 | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x73b68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73b80 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73b84 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x73b88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73b94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73b98 | load | 4 | `abs 0x74308` | `-` |
| `ldr` at 0x73b9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73ba4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73bb4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73bbc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73bc0 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73bc4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73be0 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73be8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73bf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73bf8 | load | 4 | `abs 0x7430c` | `-` |
| `ldr` at 0x73c04 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73c08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73c24 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73c2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73c38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73c3c | load | 4 | `abs 0x74310` | `-` |
| `ldr` at 0x73c48 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73c4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73c68 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73c70 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73c74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73c90 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73c98 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73c9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73cb8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73cc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73cc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73cc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73ccc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73cd4 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73cd8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73cf4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73cfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73d08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73d0c | load | 4 | `abs 0x74314` | `-` |
| `ldr` at 0x73d18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73d1c | load | 4 | `abs 0x74318` | `-` |
| `ldr` at 0x73d24 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73d28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73d44 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73d4c | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73d50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73d6c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73d74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73d78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73d84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73d90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73d94 | load | 4 | `abs 0x74318` | `-` |
| `ldr` at 0x73d9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73dac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73db4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73db8 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73dbc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73dd8 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x73de0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73de4 | load | 4 | `abs 0x7431c` | `-` |
| `ldr` at 0x73de8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73df0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73e08 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73e0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e10 | load | 4 | `abs 0x74368` | `-` |
| `ldr` at 0x73e18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73e30 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73e34 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x73e3c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x73e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73e5c | load | 4 | `abs 0x74320` | `-` |
| `ldr` at 0x73e64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73e68 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73e6c | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x73e8c | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x73e90 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x73e94 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x73e98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e9c | load | 4 | `abs 0x74304` | `-` |
| `ldr` at 0x73ea4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73eb4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73ebc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x73ec0 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73ec4 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73ec8 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73ecc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73ed0 | load | 4 | `abs 0x1496b4` | `-` |
| `ldr` at 0x73ed4 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x73ed8 | load | 4 | `abs 0x1496b8` | `-` |
| `ldr` at 0x73ee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73ee4 | load | 4 | `abs 0x74368` | `-` |
| `ldr` at 0x73eec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73efc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73f04 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `str` at 0x73f0c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x73f10 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73f14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73f18 | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x73f28 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73f2c | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x73f30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73f3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f40 | load | 4 | `abs 0x74380` | `-` |
| `ldr` at 0x73f44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73f4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73f64 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `str` at 0x73f6c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x73f70 | load | 4 | `abs 0x74324` | `-` |
| `ldr` at 0x73f74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73f78 | load | 4 | `abs 0x143578` | `-` |
| `str` at 0x73f8c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x73f90 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73f94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73f98 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x73fa8 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73fac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73fc0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x73fc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73fcc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73fdc | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x73fe0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73ff4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x73ffc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74000 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x74010 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x74018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7401c | load | 4 | `abs 0x74328` | `-` |
| `ldr` at 0x74024 | load | 4 | `abs 0x7432c` | `-` |
| `ldr` at 0x7402c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7403c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74044 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x74048 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x7404c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74060 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x74068 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x7406c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74080 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x74088 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x7408c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x740a0 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x740a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x740ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x740b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x740b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x740bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x740c0 | load | 4 | `abs 0x74368` | `-` |
| `ldr` at 0x740c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x740d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x740e0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x740e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x740ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x740f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x740fc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x74104 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7410c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x7411c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74120 | load | 4 | `abs 0x74330` | `-` |
| `ldr` at 0x7412c | load | 4 | `abs 0x74344` | `-` |
| `str` at 0x74138 | store | 4 | `abs 0x1493f0` | `ret(0x1b274)` |
| `ldr` at 0x7413c | load | 4 | `abs 0x7434c` | `-` |
| `str` at 0x74144 | store | 4 | `abs 0x1493f8` | `0x0` |
| `ldr` at 0x74148 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74158 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x74168 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x7416c | load | 4 | `(unknown)` | `-` |
| `str` at 0x74170 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x74180 | load | 4 | `abs 0x74340` | `-` |
| `ldr` at 0x74184 | load | 4 | `abs 0x1493e8` | `-` |
| `str` at 0x7418c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x74190 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x74194 | load | 4 | `abs 0x74350` | `-` |
| `ldr` at 0x741ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x741b0 | load | 4 | `abs 0x74350` | `-` |
| `ldr` at 0x741e4 | load | 4 | `abs 0x74354` | `-` |
| `ldr` at 0x741fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x74200 | load | 4 | `abs 0x74354` | `-` |
| `str` at 0x7422c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x74230 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74234 | load | 4 | `abs 0x74334` | `-` |
| `ldr` at 0x74244 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74254 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7425c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x74260 | load | 4 | `abs 0x74284` | `-` |
| `ldr` at 0x74264 | load | 4 | `abs 0x14971c` | `-` |
| `ldr` at 0x74270 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74274 | load | 4 | `abs 0x74338` | `-` |
| `ldr` at 0x743c0 | load | 4 | `abs 0x7433c` | `-` |
| `ldr` at 0x743c4 | load | 4 | `abs 0x74340` | `-` |
| `ldr` at 0x743c8 | load | 4 | `abs 0x1493ec` | `-` |
| `ldr` at 0x743cc | load | 4 | `abs 0x1493e8` | `-` |
| `str` at 0x743d4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x743e4 | load | 4 | `abs 0x74344` | `-` |
| `ldr` at 0x743e8 | load | 4 | `abs 0x1493f0` | `-` |
| `str` at 0x743f0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x743f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x743f8 | load | 4 | `abs 0x74350` | `-` |
| `ldr` at 0x74410 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x74414 | load | 4 | `abs 0x74350` | `-` |
| `ldr` at 0x74448 | load | 4 | `abs 0x74354` | `-` |
| `ldr` at 0x74460 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x74464 | load | 4 | `abs 0x74354` | `-` |
| `str` at 0x74490 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x74494 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x74498 | load | 4 | `abs 0x74350` | `-` |
| `str` at 0x744b0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x744b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x744b8 | load | 4 | `abs 0x74350` | `-` |
| `ldr` at 0x744ec | load | 4 | `abs 0x74354` | `-` |
| `str` at 0x74504 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x74508 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x7450c | load | 4 | `abs 0x74354` | `-` |
| `str` at 0x74538 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x7453c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74540 | load | 4 | `abs 0x74348` | `-` |
| `ldr` at 0x74550 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74560 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74568 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x74580 | load | 4 | `abs 0x7434c` | `-` |
| `ldr` at 0x74584 | load | 4 | `abs 0x1493f8` | `-` |
| `ldr` at 0x7459c | load | 4 | `abs 0x7434c` | `-` |
| `ldr` at 0x745a0 | load | 4 | `abs 0x1493f8` | `-` |
| `str` at 0x745a8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x745ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x745b0 | load | 4 | `abs 0x74350` | `-` |
| `ldr` at 0x745c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x745cc | load | 4 | `abs 0x74350` | `-` |
| `ldr` at 0x74600 | load | 4 | `abs 0x74354` | `-` |
| `ldr` at 0x74618 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x7461c | load | 4 | `abs 0x74354` | `-` |
| `str` at 0x74648 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x7464c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74650 | load | 4 | `abs 0x74358` | `-` |
| `ldr` at 0x74664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74668 | load | 4 | `abs 0x7435c` | `-` |
| `ldr` at 0x74674 | load | 4 | `abs 0x74360` | `-` |
| `ldr` at 0x74678 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7467c | load | 4 | `abs 0x74364` | `-` |
| `ldr` at 0x74680 | load | 4 | `abs 0x143580` | `-` |
| `ldr` at 0x74688 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7468c | load | 4 | `abs 0x74368` | `-` |
| `ldr` at 0x74698 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x746a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x746b8 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0x746bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x746c0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x746c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x746c8 | load | 4 | `abs 0x7436c` | `-` |
| `ldr` at 0x746cc | load | 4 | `abs 0x74370` | `-` |
| `ldr` at 0x746d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x746e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x746f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74714 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74718 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x7471c | load | 4 | `abs 0x149744` | `-` |
| `str` at 0x7473c | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x74740 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x74744 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x7474c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x74750 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74754 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x74758 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x74768 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x7476c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x74780 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x74788 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x7478c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x747a0 | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x747a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x747ac | load | 4 | `abs 0x74374` | `-` |
| `ldr` at 0x747b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x747b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x747bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x747cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x747d4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x747d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x747e0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x747e8 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x747ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x747f0 | load | 4 | `abs 0x74378` | `-` |
| `ldr` at 0x747f4 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x747fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7480c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74814 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x7481c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7483c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7485c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x74870 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x74878 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x7487c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74880 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74884 | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x74894 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74898 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x748b4 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x748bc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x748c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x748c8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x748cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x748d4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x748dc | load | 4 | `abs 0x7437c` | `-` |
| `ldr` at 0x748e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x748e8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x74908 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x7490c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x74910 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0x74918 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x7491c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74920 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74924 | load | 4 | `abs 0x149730` | `-` |
| `ldr` at 0x74934 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74938 | load | 4 | `abs 0x1496bc` | `-` |
| `ldr` at 0x7493c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74948 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7494c | load | 4 | `abs 0x74380` | `-` |
| `ldr` at 0x74950 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74958 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74968 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74970 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `str` at 0x74978 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7497c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74980 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7499c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x749a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x749a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x749b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x749bc | load | 4 | `abs 0x74384` | `-` |
| `ldr` at 0x749c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x749d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x749dc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x749e0 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x749e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74a00 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74a08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74a1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74a24 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74a28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74a44 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74a4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74a60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74a68 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74a6c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74a88 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74a90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74aa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74aac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74ab0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74ab4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74ab8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74ac0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74ad0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74ad8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x74adc | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74ae0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74afc | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74b04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74b18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74b20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74b2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74b30 | load | 4 | `abs 0x74388` | `-` |
| `ldr` at 0x74b3c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74b40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74b5c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74b64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74b78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74b80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74b8c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74b90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74bac | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74bc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74bd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74bd4 | load | 4 | `abs 0x7438c` | `-` |
| `ldr` at 0x74bd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74be4 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74be8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74c04 | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74c0c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74c10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74c2c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74c34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74c48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74c50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74c54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74c60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74c64 | load | 4 | `abs 0x74390` | `-` |
| `ldr` at 0x74c70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74c80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74c88 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x74c8c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74c90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74cac | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74cb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74cc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74cd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74cdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74ce0 | load | 4 | `abs 0x74394` | `-` |
| `ldr` at 0x74cec | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74cf0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74d0c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74d14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74d28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74d30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74d3c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74d40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x74d5c | load | 4 | `abs 0x149734` | `-` |
| `ldr` at 0x74d64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74d78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74d80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74d84 | load | 4 | `abs 0x74398` | `-` |
| `ldr` at 0x74d88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74d94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74d98 | load | 4 | `abs 0x7439c` | `-` |
| `ldr` at 0x74da0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74db0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x74db8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x1b2bc))` |
| `ldr` at 0x74dbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x74dc4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x74dcc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x74dd4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x74de4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74df4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x74e04 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x74e08 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x74e0c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0x74e10 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74e14 | load | 4 | `abs 0x149720` | `-` |
| `ldr` at 0x74e20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74e24 | load | 4 | `abs 0x743a0` | `-` |
| `ldr` at 0x74e30 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74e34 | load | 4 | `abs 0x149720` | `-` |
| `ldr` at 0x74e40 | load | 4 | `abs 0x743a4` | `-` |
| `ldr` at 0x74e48 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74e4c | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74e50 | load | 4 | `abs 0x149724` | `-` |
| `ldr` at 0x74e54 | load | 4 | `abs 0x149728` | `-` |
| `str` at 0x74e60 | store | 4 | `sp -0x3c` | `ret(0x75150)` |
| `ldr` at 0x74e64 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x74e70 | load | 4 | `abs 0x743b4` | `-` |
| `str` at 0x74e74 | store | 4 | `sp -0x40` | `0x1496ac` |
| `ldr` at 0x74e78 | load | 4 | `abs 0x743b4` | `-` |
| `str` at 0x74e7c | store | 4 | `sp -0x44` | `0x1496ac` |
| `ldr` at 0x74e80 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74e84 | load | 4 | `abs 0x14972c` | `-` |
| `str` at 0x74e88 | store | 4 | `sp -0x58` | `[0x14972c]` |
| `ldr` at 0x74e8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x74e90 | load | 4 | `ret(0x75150) +0xc` | `-` |
| `ldr` at 0x74e9c | load | 4 | `abs 0x743a8` | `-` |
| `str` at 0x74ea0 | store | 4 | `sp -0x48` | `0x110784` |
| `ldr` at 0x74ea8 | load | 4 | `abs 0x743ac` | `-` |
| `str` at 0x74eac | store | 4 | `sp -0x48` | `0x110788` |
| `ldr` at 0x74eb0 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x74eb4 | store | 4 | `sp -0x54` | `[sp-72]` |
| `ldr` at 0x74eb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74ebc | load | 4 | `abs 0x743b0` | `-` |
| `ldr` at 0x74ec0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x74ec4 | load | 4 | `[sp-64] +0x78` | `-` |
| `ldr` at 0x74ec8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x74ecc | load | 4 | `[sp-68] +0x7c` | `-` |
| `ldr` at 0x74ed8 | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74edc | load | 4 | `abs 0x743b4` | `-` |
| `ldr` at 0x74ee0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74ee4 | load | 4 | `abs 0x743b8` | `-` |
| `ldr` at 0x74ee8 | load | 4 | `abs 0x149724` | `-` |
| `ldr` at 0x74eec | load | 4 | `abs 0x149728` | `-` |
| `ldr` at 0x74efc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74f00 | load | 4 | `abs 0x743bc` | `-` |
| `ldmdb` at 0x74f08 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x74f08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74f08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74f08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x73370 | `(unknown callee) 0x6c95c` | `0x11048c` | `r1` | `r2` | `?` |
| `bl` at 0x73398 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[0x149738]` | `?` |
| `bl` at 0x73480 | `(unknown callee) 0x7a700` | `?` | `0x1496ac` | `?` | `?` |
| `bl` at 0x734ec | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x110498` | `?` | `?` |
| `bl` at 0x734fc | `(unknown callee) 0x1b598` | `[sp-32]` | `0x1104a0` | `?` | `?` |
| `bl` at 0x73504 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7358c | `(unknown callee) 0x6a5c8` | `([sp-32] + ret(0x1b2bc))` | `?` | `?` | `?` |
| `bl` at 0x73594 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x735e0 | `(unknown callee) 0x1b598` | `([sp-32] + ret(0x1b2bc))` | `0x1104a8` | `?` | `?` |
| `bl` at 0x73614 | `(unknown callee) 0x6a6d8` | `[0x1496bc]` | `?` | `?` | `?` |
| `bl` at 0x73628 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1104b0` | `ret(0x6a6d8)` | `ret(0x6a6d8)` |
| `bl` at 0x73630 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73684 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x1104b4` | `?` | `?` |
| `bl` at 0x7368c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x736fc | `(unknown callee) 0x74f94` | `?` | `?` | `?` | `?` |
| `bl` at 0x7373c | `(unknown callee) 0x1b364` | `[sp-32]` | `0x1104b8` | `?` | `?` |
| `bl` at 0x7379c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1104c4` | `?` | `?` |
| `bl` at 0x737ac | `(unknown callee) 0x1b364` | `[sp-32]` | `0x1104d0` | `?` | `?` |
| `bl` at 0x737b4 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x737e0 | `(unknown callee) 0x1b364` | `([sp-32] + ret(0x1b2bc))` | `0x1104dc` | `ret(0x1b2bc)` | `[sp-60]` |
| `bl` at 0x73800 | `(unknown callee) 0x1b364` | `([sp-32] + ret(0x1b2bc))` | `0x1104e8` | `ret(0x1b2bc)` | `[[sp-60]+52]` |
| `bl` at 0x73810 | `(unknown callee) 0x1b364` | `([sp-32] + ret(0x1b2bc))` | `0x1104f0` | `ret(0x1b2bc)` | `[[sp-60]+52]` |
| `bl` at 0x738a8 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110528` | `?` | `[[sp-60]+56]` |
| `bl` at 0x738dc | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110530` | `?` | `?` |
| `bl` at 0x73914 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x11053c` | `?` | `?` |
| `bl` at 0x73938 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110548` | `?` | `?` |
| `bl` at 0x7395c | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110554` | `?` | `?` |
| `bl` at 0x7396c | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110560` | `?` | `?` |
| `bl` at 0x73978 | `(unknown callee) 0x1b364` | `?` | `0x10fdfc` | `?` | `?` |
| `bl` at 0x73980 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x739c8 | `(unknown callee) 0x1b52c` | `?` | `0x11056c` | `[0x149738]` | `0x1496ac` |
| `bl` at 0x739d0 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x739f8 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x110578` | `[0x1493dc]` | `0x1493dc` |
| `bl` at 0x73a00 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73a28 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x110590` | `[0x1493e0]` | `0x1493e0` |
| `bl` at 0x73a30 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73a58 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x1105a8` | `[0x1493e4]` | `0x1493e4` |
| `bl` at 0x73a60 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73a78 | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `?` |
| `bl` at 0x73a94 | `(unknown callee) 0x6c95c` | `0x1105c0` | `r1` | `r2` | `?` |
| `bl` at 0x73ac4 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x73ae0 | `(unknown callee) 0x1b52c` | `[[sp-24]]` | `0x1105cc` | `?` | `[[sp-24]]` |
| `bl` at 0x73ae8 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73b1c | `(unknown callee) 0x6a5c8` | `([sp-32] + ret(0x1b2bc))` | `[0x1496b4]` | `[0x1496bc]` | `[0x1496b8]` |
| `bl` at 0x73b28 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x10fdfc` | `?` | `?` |
| `bl` at 0x73b30 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73b6c | `(unknown callee) 0x7a700` | `?` | `?` | `?` | `[0x149730]` |
| `bl` at 0x73b8c | `(unknown callee) 0x6a6d8` | `[0x1496bc]` | `[sp-48]` | `?` | `0x1496ac` |
| `bl` at 0x73ba0 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1105d8` | `[sp-48]` | `ret(0x6a6d8)` |
| `bl` at 0x73ba8 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73bfc | `(unknown callee) 0x1b364` | `[sp-32]` | `0x1105e4` | `?` | `?` |
| `bl` at 0x73c40 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x1105f4` | `?` | `?` |
| `bl` at 0x73cd0 | `(unknown callee) 0x6a5c8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73d10 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110610` | `?` | `?` |
| `bl` at 0x73d20 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110618` | `?` | `?` |
| `bl` at 0x73d7c | `(unknown callee) 0x6a6d8` | `?` | `?` | `?` | `?` |
| `bl` at 0x73d8c | `(unknown callee) 0x1b364` | `[sp-32]` | `ret(0x6a6d8)` | `?` | `ret(0x6a6d8)` |
| `bl` at 0x73d98 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x110618` | `?` | `?` |
| `bl` at 0x73da0 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73dec | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x11061c` | `?` | `?` |
| `bl` at 0x73df4 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73e14 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x10fdfc` | `?` | `?` |
| `bl` at 0x73e1c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73e44 | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `[0x149730]` |
| `bl` at 0x73e60 | `(unknown callee) 0x6c95c` | `0x110628` | `r1` | `r2` | `?` |
| `bl` at 0x73e84 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[0x149738]` | `?` |
| `bl` at 0x73ea0 | `(unknown callee) 0x1b52c` | `[[sp-24]]` | `0x1105cc` | `?` | `[[sp-24]]` |
| `bl` at 0x73ea8 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73edc | `(unknown callee) 0x6a5c8` | `([sp-32] + ret(0x1b2bc))` | `[0x1496b4]` | `[0x1496bc]` | `[0x1496b8]` |
| `bl` at 0x73ee8 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x10fdfc` | `?` | `?` |
| `bl` at 0x73ef0 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x73f34 | `(unknown callee) 0x6a6d8` | `[0x1496bc]` | `?` | `?` | `0x1496ac` |
| `bl` at 0x73f48 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x11071c` | `[sp-48]` | `ret(0x6a6d8)` |
| `bl` at 0x73f50 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74028 | `(unknown callee) 0x1b52c` | `([sp-32] + ret(0x1b2bc))` | `0x110634` | `?` | `0x110638` |
| `bl` at 0x74030 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x740b8 | `(unknown callee) 0x6a5c8` | `([sp-32] + ret(0x1b2bc))` | `?` | `?` | `?` |
| `bl` at 0x740c4 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x10fdfc` | `?` | `?` |
| `bl` at 0x740cc | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74114 | `(unknown callee) 0x6c9f4` | `ret(0x1b2bc)` | `?` | `?` | `[0x149730]` |
| `bl` at 0x74134 | `(unknown callee) 0x1b274` | `0x0` | `r1` | `r1` | `0x1010` |
| `bl` at 0x74160 | `(unknown callee) 0x1b1b4` | `0xc8` | `?` | `?` | `?` |
| `bl` at 0x74178 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x74240 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x11063c` | `?` | `?` |
| `bl` at 0x74248 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74278 | `(unknown callee) 0x1b598` | `([sp-32] + ret(0x1b2bc))` | `0x110654` | `ret(0x1b2bc)` | `[0x14971c]` |
| `bl` at 0x743dc | `(unknown callee) 0x1b274` | `0x0` | `[0x1493ec]` | `0x1493e8` | `?` |
| `bl` at 0x7454c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x11066c` | `?` | `?` |
| `bl` at 0x74554 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74570 | `(unknown callee) 0x1f26c` | `0x9a` | `?` | `ret(0x1b2bc)` | `([sp-32] + ret(0x1b2bc))` |
| `bl` at 0x74594 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[0x1493f8]` |
| `bl` at 0x7465c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x110698` | `?` | `?` |
| `bl` at 0x7466c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1106b0` | `?` | `[0x1493f8]` |
| `bl` at 0x74684 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1106c4` | `[0x143580]` | `0x143580` |
| `bl` at 0x74690 | `(unknown callee) 0x1b364` | `[sp-32]` | `0x10fdfc` | `?` | `?` |
| `bl` at 0x746b0 | `(unknown callee) 0x1b1b4` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x746d0 | `(unknown callee) 0x1b52c` | `?` | `0x1106e0` | `0x2b23a8` | `?` |
| `bl` at 0x746e4 | `(unknown callee) 0x6db7c` | `?` | `?` | `?` | `?` |
| `bl` at 0x746fc | `(unknown callee) 0x6dc00` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x74734 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[0x149744]` | `?` |
| `bl` at 0x747b8 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1106f8` | `?` | `?` |
| `bl` at 0x747c0 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x747f8 | `(unknown callee) 0x1b52c` | `?` | `0x110704` | `[0x149744]` | `0x1496ac` |
| `bl` at 0x74800 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74834 | `(unknown callee) 0x1d41c` | `0x9a` | `0x0` | `?` | `?` |
| `bl` at 0x74854 | `(unknown callee) 0x1d41c` | `0x9a` | `0x1` | `?` | `?` |
| `bl` at 0x748e0 | `(unknown callee) 0x6c95c` | `0x110710` | `?` | `?` | `[0x149730]` |
| `bl` at 0x74900 | `(unknown callee) 0x1b1b4` | `?` | `?` | `[sp-60]` | `?` |
| `bl` at 0x74940 | `(unknown callee) 0x6a6d8` | `[0x1496bc]` | `?` | `?` | `0x1496ac` |
| `bl` at 0x74954 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x11071c` | `[sp-48]` | `ret(0x6a6d8)` |
| `bl` at 0x7495c | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x749c0 | `(unknown callee) 0x1b598` | `[sp-32]` | `0x110728` | `[sp-36]` | `?` |
| `bl` at 0x749c8 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74abc | `(unknown callee) 0x6a5c8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74ac4 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74b34 | `(unknown callee) 0x1b598` | `[sp-32]` | `0x110730` | `?` | `?` |
| `bl` at 0x74bdc | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x110738` | `?` | `?` |
| `bl` at 0x74c58 | `(unknown callee) 0x6a6d8` | `?` | `?` | `?` | `?` |
| `bl` at 0x74c6c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x110744` | `ret(0x6a6d8)` | `ret(0x6a6d8)` |
| `bl` at 0x74c74 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74ce4 | `(unknown callee) 0x1b598` | `[sp-32]` | `0x110750` | `?` | `?` |
| `bl` at 0x74d8c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x110758` | `?` | `?` |
| `bl` at 0x74d9c | `(unknown callee) 0x1b598` | `[sp-32]` | `0x110760` | `?` | `?` |
| `bl` at 0x74da4 | `(unknown callee) 0x1b2bc` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x74ddc | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `[0x149730]` |
| `bl` at 0x74dfc | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x74e28 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x11076c` | `?` | `[0x149720]` |
| `bl` at 0x74e44 | `(unknown callee) 0x6c95c` | `0x110778` | `?` | `?` | `[0x149720]` |
| `bl` at 0x74e58 | `(unknown callee) 0x75150` | `[0x149724]` | `[0x149728]` | `0x1496ac` | `0x1496ac` |
| `bl` at 0x74ed0 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x110794` | `[[sp-64]+120]` | `[[sp-68]+124]` |
| `bl` at 0x74ef0 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1107b4` | `[0x149724]` | `[0x149728]` |
| `bl` at 0x74ef4 | `(unknown callee) 0x6c9f4` | `ret(0x1b52c)` | `?` | `?` | `?` |
| `bl` at 0x74f04 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x1107cc` | `?` | `[0x149720]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x73354 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x73364 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x733c4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x73444 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x73454 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7348c | `ret(0x7a700) != 0x0` | `ret(0x7a700) == 0x0` |
| `beq` at 0x734bc | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x735d4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73654 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x736b0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x73730 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x7376c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x737d4 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `beq` at 0x737f4 | `[[sp-60]+52] == 0x0` | `[[sp-60]+52] != 0x0` |
| `ldrls` at 0x73820 | `[[sp-60]+56] <= 0x5 (unsigned, 32-bit)` | `[[sp-60]+56] > 0x5 (unsigned, 32-bit)` |
| `beq` at 0x738bc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x738d0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x738f4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x73908 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7392c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x73950 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x739b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x73a8c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x73b60 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x73b78 | `ret(0x7a700) != 0x0` | `ret(0x7a700) == 0x0` |
| `bne` at 0x73bf0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73c34 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73d04 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x73e58 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x73f20 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x73f80 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x73fa0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x73fd4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x74008 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x74128 | `r1 != 0x1010` | `r1 == 0x1010` |
| `beq` at 0x74154 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7426c | `[0x14971c] == 0x0` | `[0x14971c] != 0x0` |
| `beq` at 0x7457c | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x7458c | `[0x1493f8] == 0x0` | `[0x1493f8] != 0x0` |
| `beq` at 0x746a4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x746e0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x746f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x74710 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x74760 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x74828 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x74848 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x74868 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x7488c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x7492c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x749b0 | `[sp-36] < ? (signed, 32-bit)` | `[sp-36] >= ? (signed, 32-bit)` |
| `bne` at 0x74b28 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x74b88 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x74cd8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x74d38 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x74df0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x74e1c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x74e3c | `[0x149720] == 0x0` | `[0x149720] != 0x0` |
| `beq` at 0x74e6c | `ret(0x75150) == 0x0` | `ret(0x75150) != 0x0` |
| `bne` at 0x74e98 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74f08 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x73820).

## `B2BInit`

- `B2BInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x77590 to 0x78170.
  Established. Included four-byte absolute assignments at `0x149b58` write `0xfff`, `0x1` and `0x0`
  (`str` at 0x775e8, `str` at 0x775f4, `str` at 0x77600). The unresolved computed transfer takes
  unsigned `[sp-52]-43 <= 0x51` and falls through for `[sp-52]-43 > 0x51` (`ldrls` at 0x77640). The
  unnamed `0x1b4cc` call carries `0x2b2400`, `0x0`, `0x84`, unknown (`bl` at 0x77620).
  Initialization selectors and board records remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x775a0 | load | 4 | `abs 0x780ec` | `-` |
| `ldr` at 0x775a4 | load | 4 | `abs 0x149b50` | `-` |
| `ldr` at 0x775b0 | load | 4 | `abs 0x780f0` | `-` |
| `str` at 0x775b4 | store | 4 | `sp -0x64` | `0x110a98` |
| `ldr` at 0x775bc | load | 4 | `abs 0x780f4` | `-` |
| `ldr` at 0x775cc | load | 4 | `abs 0x780ec` | `-` |
| `ldr` at 0x775d0 | load | 4 | `abs 0x780ec` | `-` |
| `ldr` at 0x775d4 | load | 4 | `abs 0x149b50` | `-` |
| `str` at 0x775dc | store | 4 | `abs 0x149b50` | `[0x149b50]+1` |
| `ldr` at 0x775e0 | load | 4 | `abs 0x780f8` | `-` |
| `ldr` at 0x775e4 | load | 4 | `abs 0x780fc` | `-` |
| `str` at 0x775e8 | store | 4 | `abs 0x149b58` | `0xfff` |
| `ldr` at 0x775ec | load | 4 | `abs 0x780f8` | `-` |
| `str` at 0x775f4 | store | 4 | `abs 0x149b58` | `0x1` |
| `ldr` at 0x775f8 | load | 4 | `abs 0x780f8` | `-` |
| `str` at 0x77600 | store | 4 | `abs 0x149b58` | `0x0` |
| `str` at 0x77610 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x77614 | load | 4 | `abs 0x78100` | `-` |
| `ldr` at 0x77624 | load | 4 | `abs 0x78104` | `-` |
| `ldr` at 0x77634 | load | 4 | `sp -0x34` | `-` |
| `ldrls` at 0x77640 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x77920 | load | 4 | `abs 0x78108` | `-` |
| `ldr` at 0x77924 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x77928 | load | 4 | `abs 0x7810c` | `-` |
| `ldmdb` at 0x780e8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x780e8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x780e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x780e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x780e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x775c8 | `(unknown callee) 0x21eb8` | `0x9` | `0x110ab4` | `0x68` | `0x0` |
| `bl` at 0x77604 | `(unknown callee) 0x4f4d0` | `?` | `?` | `0x149b58` | `0x0` |
| `bl` at 0x77620 | `(unknown callee) 0x1b4cc` | `0x2b2400` | `0x0` | `0x84` | `?` |
| `bl` at 0x77630 | `(unknown callee) 0x1b4cc` | `0x2b241c` | `?` | `0x34` | `?` |
| `bl` at 0x7792c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110acc` | `?` | `0x145ae8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x775ac | `[0x149b50] == 0x0` | `[0x149b50] != 0x0` |
| `ldrls` at 0x77640 | `[sp-52]-43 <= 0x51 (unsigned, 32-bit)` | `[sp-52]-43 > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x780e8 | `r0` | `ret(0x240b8)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x77640).

## `B2BEventMesh`

- `B2BEventMesh`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78cc4 to 0x79748.
  Established. A four-byte absolute assignment at `0x14a878` retains `[sp-20]` (`str` at 0x78fb0). A
  later four-byte assignment at `[sp-20] +0x18` writes `0x0` (`str` at 0x79010). The unnamed
  `0x1b394` call carries `[sp-20]+24`, `sp-68`, `0x4`, `[sp-20]+24` (`bl` at 0x79178). The branch
  takes `[sp-48] != [0x149b64]` to `0x79414` and falls through for equality (`bne` at 0x791c0). Mesh
  event fields and routing policy remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78cd4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x78cdc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x78ce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78ce4 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x78ce8 | store | 4 | `sp -0x20` | `[r0+12]` |
| `ldr` at 0x78cec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78cf0 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78d00 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78d08 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x78d0c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x78d14 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x78d18 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x78d28 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x78d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78d34 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78d3c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x78d40 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x78d50 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x78d54 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x78d58 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0x78d5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78d60 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78d6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78d70 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78d78 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78d7c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x78d84 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78d88 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x78d98 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78da0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78da4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78dac | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78db0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x78dc0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x78dc4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x78dc8 | store | 4 | `sp -0x28` | `[sp-84]` |
| `ldr` at 0x78dcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78dd0 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78de0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78de8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78dec | load | 4 | `sp -0x58` | `-` |
| `str` at 0x78df4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78df8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x78e08 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78e14 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78e1c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78e20 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x78e30 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x78e34 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x78e38 | store | 4 | `sp -0x2c` | `[sp-88]` |
| `ldr` at 0x78e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78e40 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78e50 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x78e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78e58 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78e64 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x78e68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78e6c | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x78e78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78e7c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78e90 | store | 4 | `sp -0x5c` | `0xe` |
| `str` at 0x78e9c | store | 4 | `sp -0x5c` | `0xf` |
| `ldr` at 0x78ea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78ea8 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x78eb0 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x78eb4 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x78ebc | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x78ec0 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x78ec4 | store | 4 | `sp -0x38` | `[sp-92]` |
| `ldr` at 0x78ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78ecc | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78edc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x78ee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78ee4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x78ef0 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x78ef8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x78efc | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x78f00 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x78f10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f14 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78f20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f24 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f40 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78f50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f54 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f64 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78f7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78f80 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x78f94 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x78f98 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x78f9c | load | 4 | `abs 0x149b58` | `-` |
| `str` at 0x78fa4 | store | 4 | `abs 0x149b58` | `?` |
| `ldr` at 0x78fa8 | load | 4 | `abs 0x796e4` | `-` |
| `ldr` at 0x78fac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x78fb0 | store | 4 | `abs 0x14a878` | `[sp-20]` |
| `ldr` at 0x78fb4 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x78fb8 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x78fc8 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x78fcc | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x78fd0 | load | 4 | `abs 0x796ec` | `-` |
| `ldr` at 0x78fd4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x78fd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x78fe0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x78ff0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79008 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79010 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `str` at 0x79018 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x79020 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x79024 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x79028 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79038 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x7903c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79040 | load | 4 | `abs 0x796f0` | `-` |
| `ldr` at 0x79044 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x79048 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x79050 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7905c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79068 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0x79070 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x79074 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79080 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7908c | load | 4 | `abs 0x796f4` | `-` |
| `ldr` at 0x79090 | load | 4 | `abs 0x149b60` | `-` |
| `ldr` at 0x79094 | load | 4 | `[0x149b60] +0x58` | `-` |
| `ldr` at 0x790a0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x790ac | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x790b0 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x790c0 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x790c4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x790c8 | load | 4 | `abs 0x796f8` | `-` |
| `ldr` at 0x790d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x790f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79104 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x79108 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79118 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x7911c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79120 | load | 4 | `abs 0x796fc` | `-` |
| `ldr` at 0x79128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79134 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79144 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7915c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x79160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7917c | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x79180 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79190 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x79194 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x79198 | store | 4 | `sp -0x7c` | `[sp-32]` |
| `ldr` at 0x7919c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x791a0 | load | 4 | `abs 0x79700` | `-` |
| `ldr` at 0x791a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x791a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x791b0 | load | 4 | `abs 0x79704` | `-` |
| `ldr` at 0x791b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x791b8 | load | 4 | `abs 0x149b64` | `-` |
| `ldr` at 0x791c4 | load | 4 | `abs 0x79708` | `-` |
| `ldr` at 0x791c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x791cc | load | 4 | `abs 0x149b68` | `-` |
| `ldr` at 0x791d8 | load | 4 | `abs 0x7970c` | `-` |
| `str` at 0x791dc | store | 4 | `sp -0x4c` | `0x2b16f0` |
| `ldr` at 0x791e0 | load | 4 | `abs 0x796f4` | `-` |
| `ldr` at 0x791e4 | load | 4 | `abs 0x149b60` | `-` |
| `ldr` at 0x791e8 | load | 4 | `[0x149b60] +0x58` | `-` |
| `ldr` at 0x79210 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7921c | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x79220 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79230 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x79234 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79238 | load | 4 | `abs 0x79710` | `-` |
| `ldr` at 0x79240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7924c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7925c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x79274 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x79278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7929c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x792a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x792ac | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x792c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x792c8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x792dc | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x792e0 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x792f0 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x792f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x792f8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x79308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7930c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x79318 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x7931c | store | 4 | `sp -0x7c` | `[sp-72]` |
| `ldr` at 0x79320 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79324 | load | 4 | `abs 0x79714` | `-` |
| `ldr` at 0x79330 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x7933c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79354 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x79358 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79368 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x7936c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79370 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x79380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79384 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x79390 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79394 | load | 4 | `abs 0x79718` | `-` |
| `ldr` at 0x7939c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x793a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x793ac | store | 4 | `sp -0x1c` | `ret(0xb7164)` |
| `ldr` at 0x793b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x793bc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x793c8 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x793cc | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x793dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x793e8 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x793ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x793f0 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x793f4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x793f8 | load | 4 | `abs 0x7971c` | `-` |
| `ldr` at 0x79408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79414 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79420 | load | 4 | `abs 0x79720` | `-` |
| `ldr` at 0x79428 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7942c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x79438 | store | 4 | `sp -0x18` | `ret(0x75150)` |
| `ldr` at 0x7943c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7944c | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x79450 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79460 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x79464 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79468 | load | 4 | `abs 0x79724` | `-` |
| `ldr` at 0x7946c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79470 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x79478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79484 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79488 | load | 4 | `ret(0x75150) +0x10` | `-` |
| `str` at 0x7948c | store | 4 | `sp -0x20` | `[ret(0x75150)+16]` |
| `ldr` at 0x79494 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x794a8 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x794ac | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x794bc | load | 4 | `abs 0x796e8` | `-` |
| `str` at 0x794c0 | store | 4 | `sp -0x60` | `0x145ae8` |
| `ldr` at 0x794c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x794d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x794dc | load | 4 | `abs 0x79728` | `-` |
| `str` at 0x794e0 | store | 4 | `sp -0x64` | `0x110ea0` |
| `ldr` at 0x794e8 | load | 4 | `abs 0x7972c` | `-` |
| `str` at 0x794ec | store | 4 | `sp -0x64` | `0x110ea4` |
| `ldr` at 0x794f4 | load | 4 | `abs 0x79730` | `-` |
| `str` at 0x794f8 | store | 4 | `sp -0x64` | `0x110ea8` |
| `ldr` at 0x794fc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x79500 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x79504 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x79508 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x7950c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x79510 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x79514 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x79518 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x7951c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x79520 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79524 | load | 4 | `abs 0x79734` | `-` |
| `ldr` at 0x79528 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x7952c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79534 | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x79538 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7954c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79558 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79564 | store | 4 | `sp -0x1c` | `ret(0xb7164)` |
| `ldr` at 0x79568 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79574 | load | 4 | `abs 0x79738` | `-` |
| `ldr` at 0x79578 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7958c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x795a4 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x795a8 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x795b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x795c4 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x795c8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x795cc | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x795d0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x795d4 | load | 4 | `abs 0x7973c` | `-` |
| `ldr` at 0x795e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x795f0 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x795f4 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79604 | load | 4 | `abs 0x796e8` | `-` |
| `ldr` at 0x79608 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7960c | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x79610 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x79614 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x79618 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x7961c | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x79620 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x79624 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x79628 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7962c | load | 4 | `abs 0x79740` | `-` |
| `ldr` at 0x79630 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79634 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x7963c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79648 | load | 4 | `abs 0x796e0` | `-` |
| `ldr` at 0x7964c | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7965c | load | 4 | `abs 0x796e8` | `-` |
| `str` at 0x79660 | store | 4 | `sp -0x68` | `0x145ae8` |
| `ldr` at 0x79664 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x79670 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7967c | load | 4 | `abs 0x79728` | `-` |
| `str` at 0x79680 | store | 4 | `sp -0x6c` | `0x110ea0` |
| `ldr` at 0x79688 | load | 4 | `abs 0x7972c` | `-` |
| `str` at 0x7968c | store | 4 | `sp -0x6c` | `0x110ea4` |
| `ldr` at 0x79694 | load | 4 | `abs 0x79730` | `-` |
| `str` at 0x79698 | store | 4 | `sp -0x6c` | `0x110ea8` |
| `ldr` at 0x7969c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x796a0 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0x796a4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x796a8 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x796ac | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x796b0 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0x796b4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x796b8 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0x796bc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x796c0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x796c4 | load | 4 | `abs 0x79744` | `-` |
| `ldr` at 0x796c8 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x796cc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x796d4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x796dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x796dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x796dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78fdc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110cf0` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x78ff4 | `(unknown callee) 0x75494` | `?` | `?` | `?` | `?` |
| `bl` at 0x7904c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110d10` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x79060 | `(unknown callee) 0x73104` | `[sp-20]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x790cc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110d3c` | `?` | `0x145ae8` |
| `bl` at 0x790d4 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x790f4 | `(unknown callee) 0x75344` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x79124 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110d54` | `?` | `0x145ae8` |
| `bl` at 0x7912c | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79178 | `(unknown callee) 0x1b394` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x791ac | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110d80` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x7920c | `(unknown callee) 0x75344` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0x7923c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110d9c` | `?` | `0x145ae8` |
| `bl` at 0x79244 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79290 | `(unknown callee) 0x1b394` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0x7932c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110dd0` | `?` | `?` |
| `bl` at 0x79340 | `(unknown callee) 0x73278` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x7934c | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0x79398 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110e08` | `?` | `?` |
| `bl` at 0x793a4 | `(unknown callee) 0xb7164` | `[sp-76]` | `[sp-20]` | `?` | `?` |
| `bl` at 0x793c0 | `(unknown callee) 0x83128` | `[sp-76]` | `?` | `?` | `ret(0xb7164)` |
| `bl` at 0x793e0 | `(unknown callee) 0x21704` | `ret(0xb7164)` | `?` | `?` | `?` |
| `bl` at 0x79404 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110e30` | `[[sp-20]+22]:1` | `ret(0x21704)` |
| `bl` at 0x7940c | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79424 | `(unknown callee) 0x6c95c` | `0x110e5c` | `?` | `?` | `[sp-32]` |
| `bl` at 0x79430 | `(unknown callee) 0x75150` | `[sp-48]` | `[sp-52]` | `?` | `?` |
| `bl` at 0x79448 | `(unknown callee) 0x6c9f4` | `ret(0x75150)` | `?` | `?` | `ret(0x75150)` |
| `bl` at 0x79474 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110e6c` | `[sp-48]` | `[sp-52]` |
| `bl` at 0x7947c | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79490 | `(unknown callee) 0x6c9f4` | `ret(0x75150)` | `?` | `?` | `[ret(0x75150)+16]` |
| `bl` at 0x79498 | `(unknown callee) 0x7a658` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x79530 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110eac` | `?` | `[sp-56]` |
| `bl` at 0x7955c | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x7959c | `(unknown callee) 0x83128` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x795bc | `(unknown callee) 0x21704` | `ret(0xb7164)` | `?` | `?` | `?` |
| `bl` at 0x795e0 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110edc` | `[[sp-20]+22]:1` | `ret(0x21704)` |
| `bl` at 0x795e8 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79638 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110f08` | `[sp-32]` | `[sp-56]` |
| `bl` at 0x79640 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x796d0 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110f3c` | `?` | `[sp-56]` |
| `bl` at 0x796d8 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x78cf8 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x78d68 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x78dd8 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0x78e74 | `[r0+24] >= 0x0 (signed, 32-bit)` | `[r0+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x78e88 | `? != 0x2` | `? == 0x2` |
| `beq` at 0x78f0c | `? == 0x0` | `? != 0x0` |
| `bge` at 0x78f1c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x78f34 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x78f4c | `? == 0x5` | `? != 0x5` |
| `bge` at 0x78f5c | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x78f74 | `? == 0x4` | `? != 0x4` |
| `beq` at 0x78f8c | `? == 0x4` | `? != 0x4` |
| `beq` at 0x78fc4 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x78fec | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x79000 | `ret(0x75494) != 0x0` | `ret(0x75494) == 0x0` |
| `beq` at 0x79034 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x79058 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0x7907c | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0x79088 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `bne` at 0x7909c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x790a8 | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x790bc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79100 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x79114 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7918c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x791c0 | `[sp-48] != [0x149b64]` | `[sp-48] == [0x149b64]` |
| `bne` at 0x791d4 | `[sp-52] != [0x149b68]` | `[sp-52] == [0x149b68]` |
| `beq` at 0x791f0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x79218 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x7922c | `? == 0x0` | `? != 0x0` |
| `bge` at 0x792a4 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x792bc | `? == 0xe` | `? != 0xe` |
| `beq` at 0x792d4 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x792ec | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79338 | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0x79364 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x793b8 | `ret(0xb7164) != 0x0` | `ret(0xb7164) == 0x0` |
| `beq` at 0x793d8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7941c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x79444 | `ret(0x75150) != 0x0` | `ret(0x75150) == 0x0` |
| `beq` at 0x7945c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x794a4 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x794b8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x794cc | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x794d8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x79570 | `ret(0xb7164) != 0x0` | `ret(0xb7164) == 0x0` |
| `beq` at 0x795b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x79600 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x79658 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7966c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x79678 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x796dc | `r0` | `?` |

## `B2BEventRing`

- `B2BEventRing`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79748 to 0x79fec.
  Established. An included four-byte store with unknown effective target retains `[0x149b54]` (`str`
  at 0x79880). The branch takes unsigned `[sp-28]-1 > 0x1` to `0x79b00` and falls through for
  `[sp-28]-1 <= 0x1` (`bhi` at 0x79944). The unnamed `0xb7090` call has `0x2b2400`, `[sp-20]`,
  unknown, unknown (`bl` at 0x798e4). Ring distribution and record identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79758 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x79760 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x79764 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79768 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0x7976c | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0x79770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79774 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x79784 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x79788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7978c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x79798 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7979c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797a0 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x797b0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x797b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797b8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x797c4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x797c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797cc | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x797d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x797dc | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x797e4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x797e8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x797f0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x797f4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x79804 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x7980c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79810 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x79818 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x7981c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x7982c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x79830 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x79834 | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x79838 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x79844 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79850 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x79854 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79864 | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79868 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7986c | load | 4 | `abs 0x79fb8` | `-` |
| `ldr` at 0x79874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79878 | load | 4 | `abs 0x79fbc` | `-` |
| `ldr` at 0x7987c | load | 4 | `abs 0x149b54` | `-` |
| `str` at 0x79880 | store | 4 | `(unknown)` | `[0x149b54]` |
| `ldr` at 0x79888 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x7988c | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7989c | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x798a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x798a4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x798b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x798b8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x798c4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x798c8 | load | 4 | `abs 0x79fc0` | `-` |
| `ldr` at 0x798d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x798dc | load | 4 | `abs 0x79fc4` | `-` |
| `ldr` at 0x798e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x798e8 | load | 4 | `abs 0x79fc8` | `-` |
| `ldr` at 0x798ec | load | 4 | `abs 0x149b60` | `-` |
| `ldr` at 0x798f0 | load | 4 | `[0x149b60] +0x58` | `-` |
| `ldr` at 0x798fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x79908 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x7990c | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7991c | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79920 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79924 | load | 4 | `abs 0x79fcc` | `-` |
| `ldr` at 0x7992c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79938 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79948 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x7994c | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7995c | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79960 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79964 | load | 4 | `abs 0x79fd0` | `-` |
| `ldr` at 0x79968 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79970 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79980 | store | 4 | `sp -0x44` | `0x2` |
| `str` at 0x7998c | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0x79990 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x79994 | store | 4 | `sp -0x1c` | `[sp-68]` |
| `ldr` at 0x79998 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x799a4 | store | 4 | `sp -0x38` | `ret(0x753bc)` |
| `ldr` at 0x799a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x799bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x799c0 | load | 4 | `[sp-56] +0x4` | `-` |
| `ldr` at 0x799cc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x799d0 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x799dc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x799e0 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x799ec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x799f0 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x799fc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a00 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a0c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a10 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a1c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a20 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a2c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a30 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a3c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a40 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a4c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a50 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a5c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a60 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a6c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a70 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a80 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a8c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79a90 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79a9c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79aa0 | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0x79ab0 | load | 4 | `abs 0x79fc4` | `-` |
| `ldr` at 0x79ab4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79ac8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79ad4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79ae0 | store | 4 | `sp -0x20` | `ret(0xb7164)` |
| `ldr` at 0x79ae4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79af4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79b00 | load | 4 | `abs 0x79fd4` | `-` |
| `ldr` at 0x79b04 | load | 4 | `abs 0x1493d8` | `-` |
| `ldr` at 0x79b10 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x79b14 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79b24 | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79b28 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79b2c | load | 4 | `abs 0x79fd8` | `-` |
| `ldr` at 0x79b34 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x79b44 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x79b48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79b58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79b6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79b78 | load | 4 | `abs 0x79fc4` | `-` |
| `ldr` at 0x79b7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79b90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79bb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79bbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79bc4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x79bcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79bdc | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x79be0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79bf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79c04 | load | 4 | `abs 0x79fc4` | `-` |
| `ldr` at 0x79c08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79c1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79c40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79c48 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x79c50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79c60 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x79c64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79c74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79c88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79c94 | store | 4 | `sp -0x38` | `ret(0x753bc)` |
| `ldr` at 0x79c98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79c9c | load | 4 | `ret(0x753bc) +0x4` | `-` |
| `ldr` at 0x79ca8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79cac | load | 4 | `ret(0x753bc) +0x78` | `-` |
| `ldr` at 0x79cb8 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x79cbc | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79ccc | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79cd0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79cd4 | load | 4 | `abs 0x79fdc` | `-` |
| `ldr` at 0x79cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79ce0 | load | 4 | `abs 0x79fc4` | `-` |
| `ldr` at 0x79ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79cf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79d04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79d10 | store | 4 | `sp -0x20` | `ret(0xb7164)` |
| `ldr` at 0x79d14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79d24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79d30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79d38 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x79d44 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x79d48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79d58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79d6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79d78 | store | 4 | `sp -0x38` | `ret(0x753bc)` |
| `ldr` at 0x79d7c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79d80 | load | 4 | `ret(0x753bc) +0x4` | `-` |
| `ldr` at 0x79d8c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79d90 | load | 4 | `ret(0x753bc) +0x10` | `-` |
| `ldr` at 0x79d9c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79da0 | load | 4 | `ret(0x753bc) +0x10` | `-` |
| `ldr` at 0x79dac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79db0 | load | 4 | `ret(0x753bc) +0x10` | `-` |
| `ldr` at 0x79dbc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79dc0 | load | 4 | `ret(0x753bc) +0x10` | `-` |
| `ldr` at 0x79dd0 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x79dd4 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79de4 | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79de8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79dec | load | 4 | `abs 0x79fe0` | `-` |
| `ldr` at 0x79df0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79df8 | load | 4 | `abs 0x79fc4` | `-` |
| `ldr` at 0x79dfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79e10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79e1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79e28 | store | 4 | `sp -0x20` | `ret(0xb7164)` |
| `ldr` at 0x79e2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79e50 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x79e58 | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x79e5c | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79e6c | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79e70 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79e74 | load | 4 | `abs 0x79fe4` | `-` |
| `str` at 0x79e80 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x79e84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79e94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79eac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79ec4 | store | 4 | `sp -0x38` | `ret(0x753bc)` |
| `str` at 0x79ed4 | store | 4 | `sp -0x3c` | `ret(0x753bc)` |
| `ldr` at 0x79ed8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x79edc | load | 4 | `ret(0x753bc) +0x4` | `-` |
| `ldr` at 0x79ee8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79eec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x79ef0 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0x79ef4 | load | 4 | `ret(0x753bc) +0x8` | `-` |
| `ldr` at 0x79f00 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x79f04 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x79f08 | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0x79f0c | load | 4 | `ret(0x753bc) +0xc` | `-` |
| `ldr` at 0x79f1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79f28 | load | 4 | `abs 0x79fc4` | `-` |
| `ldr` at 0x79f2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79f40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79f6c | load | 4 | `abs 0x79fb0` | `-` |
| `ldr` at 0x79f70 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x79f80 | load | 4 | `abs 0x79fb4` | `-` |
| `ldr` at 0x79f84 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x79f88 | load | 4 | `abs 0x79fe8` | `-` |
| `ldr` at 0x79f8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79f94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79f9c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x79fa4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x79fac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x79fac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x79fac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79870 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110f6c` | `r2` | `0x145ae8` |
| `bl` at 0x798cc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110f84` | `?` | `?` |
| `bl` at 0x798d8 | `(unknown callee) 0xb432c` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x798e4 | `(unknown callee) 0xb7090` | `0x2b2400` | `[sp-20]` | `?` | `?` |
| `bl` at 0x79928 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110d3c` | `?` | `0x145ae8` |
| `bl` at 0x79930 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x7996c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110fa8` | `[sp-28]` | `0x145ae8` |
| `bl` at 0x7999c | `(unknown callee) 0x753bc` | `[sp-68]` | `?` | `?` | `[sp-68]` |
| `bl` at 0x799ac | `(unknown callee) 0x7a658` | `[sp-28]` | `?` | `?` | `ret(0x753bc)` |
| `bl` at 0x79ad8 | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x79af8 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79b30 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110fc4` | `?` | `0x145ae8` |
| `bl` at 0x79b5c | `(unknown callee) 0x7a658` | `?` | `?` | `?` | `?` |
| `bl` at 0x79b74 | `(unknown callee) 0xb432c` | `[sp-20]` | `0x1` | `?` | `ret(0x7a658)` |
| `bl` at 0x79ba0 | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x79bb8 | `(unknown callee) 0xb432c` | `[sp-20]` | `?` | `?` | `ret(0xb7164)` |
| `bl` at 0x79bd0 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79bf4 | `(unknown callee) 0x7a658` | `?` | `?` | `?` | `?` |
| `bl` at 0x79c2c | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x79c54 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x79c78 | `(unknown callee) 0x7a658` | `?` | `?` | `?` | `?` |
| `bl` at 0x79c8c | `(unknown callee) 0x753bc` | `[sp-28]` | `?` | `?` | `ret(0x7a658)` |
| `bl` at 0x79cdc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110ff0` | `[sp-28]` | `0x145ae8` |
| `bl` at 0x79d08 | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x79d28 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `ret(0xb7164)` |
| `bl` at 0x79d5c | `(unknown callee) 0x7a658` | `?` | `?` | `?` | `?` |
| `bl` at 0x79d70 | `(unknown callee) 0x753bc` | `[sp-28]` | `?` | `?` | `ret(0x7a658)` |
| `bl` at 0x79df4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x111014` | `[sp-28]` | `0x145ae8` |
| `bl` at 0x79e20 | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x79e40 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `ret(0xb7164)` |
| `bl` at 0x79e78 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x111038` | `?` | `0x145ae8` |
| `bl` at 0x79e98 | `(unknown callee) 0x7a658` | `?` | `?` | `?` | `?` |
| `bl` at 0x79ebc | `(unknown callee) 0x753bc` | `0x1` | `?` | `?` | `[sp-28]` |
| `bl` at 0x79ecc | `(unknown callee) 0x753bc` | `0x2` | `?` | `?` | `ret(0x753bc)` |
| `bl` at 0x79f24 | `(unknown callee) 0xb432c` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x79f50 | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x79f68 | `(unknown callee) 0xb432c` | `[sp-20]` | `?` | `?` | `ret(0xb7164)` |
| `bl` at 0x79f90 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x111054` | `[sp-28]` | `0x145ae8` |
| `bl` at 0x79fa8 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x797d4 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x79840 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x7984c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x79860 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x79898 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x798f8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x79904 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x79918 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x79944 | `[sp-28]-1 > 0x1 (unsigned, 32-bit)` | `[sp-28]-1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x79958 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79978 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `beq` at 0x799b8 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x799c8 | `[[sp-56]+4] == 0x0` | `[[sp-56]+4] != 0x0` |
| `beq` at 0x799d8 | `[[sp-56]+16] == 0x35` | `[[sp-56]+16] != 0x35` |
| `beq` at 0x799e8 | `[[sp-56]+16] == 0x4f` | `[[sp-56]+16] != 0x4f` |
| `beq` at 0x799f8 | `[[sp-56]+16] == 0x42` | `[[sp-56]+16] != 0x42` |
| `beq` at 0x79a08 | `[[sp-56]+16] == 0x46` | `[[sp-56]+16] != 0x46` |
| `beq` at 0x79a18 | `[[sp-56]+16] == 0x48` | `[[sp-56]+16] != 0x48` |
| `beq` at 0x79a28 | `[[sp-56]+16] == 0x4a` | `[[sp-56]+16] != 0x4a` |
| `beq` at 0x79a38 | `[[sp-56]+16] == 0x4b` | `[[sp-56]+16] != 0x4b` |
| `beq` at 0x79a48 | `[[sp-56]+16] == 0x21` | `[[sp-56]+16] != 0x21` |
| `beq` at 0x79a58 | `[[sp-56]+16] == 0x5e` | `[[sp-56]+16] != 0x5e` |
| `beq` at 0x79a68 | `[[sp-56]+16] == 0x2b` | `[[sp-56]+16] != 0x2b` |
| `beq` at 0x79a78 | `[[sp-56]+16] == 0x4c` | `[[sp-56]+16] != 0x4c` |
| `beq` at 0x79a88 | `[[sp-56]+16] == 0x5a` | `[[sp-56]+16] != 0x5a` |
| `beq` at 0x79a98 | `[[sp-56]+16] == 0x7c` | `[[sp-56]+16] != 0x7c` |
| `beq` at 0x79aa8 | `[[sp-56]+16] == 0x4d` | `[[sp-56]+16] != 0x4d` |
| `bne` at 0x79aec | `ret(0xb7164) != 0x0` | `ret(0xb7164) == 0x0` |
| `ble` at 0x79b0c | `[0x1493d8] <= 0x0 (signed, 32-bit)` | `[0x1493d8] > 0x0 (signed, 32-bit)` |
| `beq` at 0x79b20 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x79b3c | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `ble` at 0x79b50 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79b68 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x79bac | `ret(0xb7164) == 0x0` | `ret(0xb7164) != 0x0` |
| `ble` at 0x79be8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79c00 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `bne` at 0x79c38 | `ret(0xb7164) != 0x0` | `ret(0xb7164) == 0x0` |
| `ble` at 0x79c6c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79c84 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x79ca4 | `[ret(0x753bc)+4] == 0x0` | `[ret(0x753bc)+4] != 0x0` |
| `bne` at 0x79cb4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x79cc8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79d1c | `ret(0xb7164) != 0x0` | `ret(0xb7164) == 0x0` |
| `ble` at 0x79d50 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79d68 | `ret(0x7a658) == 0x0` | `ret(0x7a658) != 0x0` |
| `beq` at 0x79d88 | `[ret(0x753bc)+4] == 0x0` | `[ret(0x753bc)+4] != 0x0` |
| `beq` at 0x79d98 | `[ret(0x753bc)+16] == 0x52` | `[ret(0x753bc)+16] != 0x52` |
| `beq` at 0x79da8 | `[ret(0x753bc)+16] == 0x44` | `[ret(0x753bc)+16] != 0x44` |
| `beq` at 0x79db8 | `[ret(0x753bc)+16] == 0x36` | `[ret(0x753bc)+16] != 0x36` |
| `beq` at 0x79dc8 | `[ret(0x753bc)+16] == 0x40` | `[ret(0x753bc)+16] != 0x40` |
| `beq` at 0x79de0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x79e34 | `ret(0xb7164) != 0x0` | `ret(0xb7164) == 0x0` |
| `beq` at 0x79e68 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x79e8c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x79ea4 | `ret(0x7a658) != 0x0` | `ret(0x7a658) == 0x0` |
| `bne` at 0x79eb4 | `[sp-28] != 0x2` | `[sp-28] == 0x2` |
| `beq` at 0x79ee4 | `[ret(0x753bc)+4] == 0x0` | `[ret(0x753bc)+4] != 0x0` |
| `bne` at 0x79efc | `[[sp-56]+8] != [ret(0x753bc)+8]` | `[[sp-56]+8] == [ret(0x753bc)+8]` |
| `bne` at 0x79f14 | `[[sp-56]+12] != [ret(0x753bc)+12]` | `[[sp-56]+12] == [ret(0x753bc)+12]` |
| `beq` at 0x79f5c | `ret(0xb7164) == 0x0` | `ret(0xb7164) != 0x0` |
| `beq` at 0x79f7c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x79fac | `r0` | `?` |

## `B2BReqRsp`

- `B2BReqRsp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79fec to 0x7a658.
  Established. The unresolved computed transfer takes unsigned `[sp-60]-1 <= 0xe` and falls through
  for `[sp-60]-1 > 0xe` (`ldrls` at 0x7a0e8). A known-edge four-byte read targets `ret(0x750dc) +0x10` (`ldr` at 0x7a410). The unnamed `0xb7164` call has unknown, `[sp-20]`, `0x50`, unknown
  (`bl` at 0x7a54c); its zero result takes the branch to `0x7a608` (`beq` at 0x7a560). Request
  selection and response fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79ffc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7a004 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7a008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a00c | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x7a01c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7a020 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a024 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x7a030 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7a034 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a038 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x7a044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a048 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x7a050 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x7a054 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x7a05c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x7a064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a068 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x7a070 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x7a074 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x7a078 | store | 4 | `sp -0x2c` | `[sp-56]` |
| `ldr` at 0x7a07c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a080 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x7a08c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a090 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x7a0a4 | store | 4 | `sp -0x3c` | `0xe` |
| `str` at 0x7a0b0 | store | 4 | `sp -0x3c` | `0xf` |
| `ldr` at 0x7a0b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a0bc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x7a0c4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x7a0c8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x7a0d0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x7a0d4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x7a0d8 | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0x7a0dc | load | 4 | `sp -0x30` | `-` |
| `ldrls` at 0x7a0e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a39c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a3a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a3b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a3c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a3cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a3d8 | load | 4 | `abs 0x7a62c` | `-` |
| `ldr` at 0x7a3e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a3e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7a3fc | store | 4 | `sp -0x34` | `ret(0x750dc)` |
| `ldr` at 0x7a400 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7a40c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7a410 | load | 4 | `ret(0x750dc) +0x10` | `-` |
| `str` at 0x7a414 | store | 4 | `sp -0x20` | `[ret(0x750dc)+16]` |
| `ldr` at 0x7a418 | load | 4 | `abs 0x7a620` | `-` |
| `ldr` at 0x7a41c | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7a42c | load | 4 | `abs 0x7a624` | `-` |
| `ldr` at 0x7a430 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7a434 | store | 4 | `sp -0x48` | `[ret(0x750dc)+16]` |
| `ldr` at 0x7a438 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7a43c | load | 4 | `abs 0x7a630` | `-` |
| `ldr` at 0x7a440 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a444 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a450 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a45c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a468 | store | 4 | `sp -0x20` | `ret(0x7a748)` |
| `ldr` at 0x7a46c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a47c | load | 4 | `abs 0x7a620` | `-` |
| `ldr` at 0x7a480 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7a490 | load | 4 | `abs 0x7a624` | `-` |
| `ldr` at 0x7a494 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7a498 | store | 4 | `sp -0x48` | `ret(0x7a748)` |
| `ldr` at 0x7a49c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7a4a0 | load | 4 | `abs 0x7a634` | `-` |
| `ldr` at 0x7a4a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a4a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a4b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a4c4 | load | 4 | `abs 0x7a620` | `-` |
| `ldr` at 0x7a4c8 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7a4d8 | load | 4 | `abs 0x7a624` | `-` |
| `ldr` at 0x7a4dc | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7a4e0 | load | 4 | `abs 0x7a638` | `-` |
| `ldr` at 0x7a4e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a4ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7a4f0 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x7a500 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x7a504 | load | 4 | `abs 0x7a63c` | `-` |
| `ldr` at 0x7a508 | load | 4 | `abs 0x7a640` | `-` |
| `ldr` at 0x7a50c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a524 | load | 4 | `abs 0x7a63c` | `-` |
| `ldr` at 0x7a528 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a53c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a548 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a554 | store | 4 | `sp -0x18` | `ret(0xb7164)` |
| `ldr` at 0x7a558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a564 | load | 4 | `abs 0x7a620` | `-` |
| `ldr` at 0x7a568 | load | 4 | `abs 0x149b58` | `-` |
| `ldr` at 0x7a578 | load | 4 | `abs 0x7a624` | `-` |
| `ldr` at 0x7a57c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x7a580 | load | 4 | `abs 0x7a644` | `-` |
| `ldr` at 0x7a584 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a588 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a590 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7a594 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0x7a5a4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x7a5a8 | load | 4 | `abs 0x7a63c` | `-` |
| `ldr` at 0x7a5ac | load | 4 | `abs 0x7a648` | `-` |
| `ldr` at 0x7a5b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a5bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7a5c4 | store | 4 | `sp -0x40` | `ret(0x21704)` |
| `ldr` at 0x7a5c8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7a5cc | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0x7a5d8 | load | 4 | `abs 0x7a64c` | `-` |
| `str` at 0x7a5dc | store | 4 | `sp -0x44` | `0x111154` |
| `ldr` at 0x7a5e4 | load | 4 | `abs 0x7a650` | `-` |
| `str` at 0x7a5e8 | store | 4 | `sp -0x44` | `0x111158` |
| `ldr` at 0x7a5ec | load | 4 | `abs 0x7a654` | `-` |
| `ldr` at 0x7a5f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a5f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x7a5f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x7a600 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7a608 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7a608 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7a608 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a3dc | `(unknown callee) 0x6c95c` | `0x1110a4` | `r1` | `r2` | `[sp-40]` |
| `bl` at 0x7a3e8 | `(unknown callee) 0x74fe4` | `[sp-36]` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7a3f4 | `(unknown callee) 0x750dc` | `ret(0x74fe4)` | `?` | `?` | `ret(0x74fe4)` |
| `bl` at 0x7a448 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1110b0` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x7a44c | `(unknown callee) 0x6c9f4` | `?` | `?` | `?` | `?` |
| `bl` at 0x7a460 | `(unknown callee) 0x7a748` | `[sp-20]` | `?` | `?` | `[sp-32]` |
| `bl` at 0x7a4ac | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1110d4` | `[sp-36]` | `[sp-40]` |
| `bl` at 0x7a4b4 | `(unknown callee) 0x7a658` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7a4e8 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x111110` | `[sp-32]` | `0x145ae8` |
| `bl` at 0x7a514 | `(unknown callee) 0xb74d4` | `0x2b2400` | `0xfffffda0` | `[sp-20]` | `0x0` |
| `bl` at 0x7a51c | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x7a54c | `(unknown callee) 0xb7164` | `?` | `[sp-20]` | `0x50` | `?` |
| `bl` at 0x7a58c | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x11112c` | `[sp-32]` | `ret(0xb7164)` |
| `bl` at 0x7a5b8 | `(unknown callee) 0xb74d4` | `0x2b2400` | `0xfffffd9e` | `[sp-20]` | `0x0` |
| `bl` at 0x7a5c0 | `(unknown callee) 0x21704` | `?` | `?` | `?` | `?` |
| `bl` at 0x7a5fc | `(unknown callee) 0x2079c` | `0x11115c` | `[sp-32]` | `[sp-68]` | `ret(0x21704)` |
| `bl` at 0x7a604 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7a040 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bge` at 0x7a088 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x7a09c | `? != 0x2` | `? == 0x2` |
| `ldrls` at 0x7a0e8 | `[sp-60]-1 <= 0xe (unsigned, 32-bit)` | `[sp-60]-1 > 0xe (unsigned, 32-bit)` |
| `bne` at 0x7a3a4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x7a3b0 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x7a3bc | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x7a3c8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x7a3d4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x7a408 | `ret(0x750dc) == 0x0` | `ret(0x750dc) != 0x0` |
| `beq` at 0x7a428 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7a458 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0x7a474 | `ret(0x7a748) > 0x0 (signed, 32-bit)` | `ret(0x7a748) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x7a48c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7a4c0 | `ret(0x7a658) != 0x0` | `ret(0x7a658) == 0x0` |
| `beq` at 0x7a4d4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7a4f8 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `beq` at 0x7a560 | `ret(0xb7164) == 0x0` | `ret(0xb7164) != 0x0` |
| `beq` at 0x7a574 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7a59c | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `bne` at 0x7a5d4 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7a608 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7a0e8).

## `SMPCmdIRouter`

- `SMPCmdIRouter`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9aab0 to 0x9adc4.
  Established. A four-byte absolute assignment at `0x14a568` writes `0x0` (`str` at 0x9ad2c). The
  indirect calls have unknown targets and candidates `0x1`, `[sp-32]`, `sp-2088`, `sp-2088`, then
  `0x2`, `[sp-32]`, `sp-2088`, `sp-2088` respectively (`mov` at 0x9acc4, `mov` at 0x9ad18). The
  unnamed `0xb74d4` call carries `0x2b25a0`, `0x0`, `[sp-2104]`, `[sp-32]` (`bl` at 0x9ad50).
  Command dispatch fields and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9aac4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9aac8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9aadc | load | 4 | `abs 0x9ad9c` | `-` |
| `str` at 0x9aaec | store | 4 | `sp -0x1c` | `ret(0xb5f90)` |
| `ldr` at 0x9aaf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9aafc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ab08 | load | 4 | `abs 0x9ada0` | `-` |
| `str` at 0x9ab0c | store | 4 | `sp -0x848` | `0x11e5e0` |
| `str` at 0x9ab10 | store | 4 | `sp -0x844` | `ret(0x21704)` |
| `ldr` at 0x9ab18 | load | 4 | `abs 0x9ada4` | `-` |
| `ldr` at 0x9ab1c | load | 4 | `abs 0x9ada8` | `-` |
| `ldr` at 0x9ab28 | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x9ab2c | load | 1 | `[sp-2104] +0x20` | `-` |
| `str` at 0x9ab3c | store | 4 | `sp -0x834` | `0x0` |
| `ldr` at 0x9ab44 | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x9ab48 | load | 1 | `[sp-2104] +0x2c` | `-` |
| `ldr` at 0x9ab58 | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x9ab5c | load | 4 | `[sp-2104] +0x30` | `-` |
| `str` at 0x9ab60 | store | 4 | `sp -0x840` | `[[sp-2104]+48]` |
| `ldr` at 0x9ab68 | load | 4 | `sp -0x838` | `-` |
| `str` at 0x9ab74 | store | 4 | `sp -0x840` | `ret(0xb823c)` |
| `ldr` at 0x9ab78 | load | 4 | `sp -0x840` | `-` |
| `str` at 0x9ab7c | store | 4 | `sp -0x834` | `[sp-2112]` |
| `ldr` at 0x9ab8c | load | 4 | `sp -0x834` | `-` |
| `ldr` at 0x9ab94 | load | 4 | `sp -0x838` | `-` |
| `str` at 0x9aba0 | store | 4 | `sp -0x1c` | `ret(0x997ac)` |
| `ldr` at 0x9abbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9abc4 | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x9abd4 | store | 4 | `sp -0x830` | `0x0` |
| `ldr` at 0x9abd8 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9abe0 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9abe8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9abf8 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9ac00 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9ac08 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9ac0c | store | 4 | `sp -0x83c` | `?` |
| `ldr` at 0x9ac10 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9ac18 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9ac20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ac40 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9ac48 | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x9ac54 | store | 4 | `sp -0x848` | `0x0` |
| `ldr` at 0x9ac58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ac64 | load | 4 | `sp -0x830` | `-` |
| `str` at 0x9ac6c | store | 4 | `sp -0x830` | `[sp-2096]+1` |
| `ldr` at 0x9ac74 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9ac7c | load | 4 | `abs 0x9adac` | `-` |
| `ldr` at 0x9ac80 | load | 4 | `abs 0x14a72c` | `-` |
| `ldr` at 0x9ac8c | load | 4 | `abs 0x9adb0` | `-` |
| `ldr` at 0x9ac90 | load | 4 | `abs 0x14a568` | `-` |
| `ldr` at 0x9aca0 | load | 4 | `abs 0x9adac` | `-` |
| `ldr` at 0x9acb0 | load | 4 | `abs 0x14a72c` | `-` |
| `ldr` at 0x9acb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9accc | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x9acd0 | load | 4 | `abs 0x9adac` | `-` |
| `ldr` at 0x9acd4 | load | 4 | `abs 0x14a72c` | `-` |
| `ldr` at 0x9ace0 | load | 4 | `abs 0x9adb0` | `-` |
| `ldr` at 0x9ace4 | load | 4 | `abs 0x14a568` | `-` |
| `ldr` at 0x9acf4 | load | 4 | `abs 0x9adac` | `-` |
| `ldr` at 0x9ad04 | load | 4 | `abs 0x14a72c` | `-` |
| `ldr` at 0x9ad0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ad20 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x9ad24 | load | 4 | `abs 0x9adb0` | `-` |
| `str` at 0x9ad2c | store | 4 | `abs 0x14a568` | `0x0` |
| `str` at 0x9ad3c | store | 4 | `sp -0x848` | `sp-2088` |
| `ldr` at 0x9ad40 | load | 4 | `abs 0x9adb4` | `-` |
| `ldr` at 0x9ad48 | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x9ad4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9ad58 | store | 4 | `sp -0x1c` | `ret(0xb74d4)` |
| `ldr` at 0x9ad5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ad68 | load | 4 | `abs 0x9adb8` | `-` |
| `ldr` at 0x9ad6c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x9ad70 | load | 4 | `abs 0x9adbc` | `-` |
| `ldr` at 0x9ad78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ad84 | load | 4 | `abs 0x9adc0` | `-` |
| `ldr` at 0x9ad90 | load | 4 | `sp -0x838` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9aae4 | `(unknown callee) 0xb5f90` | `sp-2104` | `0x2b25a8` | `?` | `sp-2104` |
| `bl` at 0x9ab00 | `(unknown callee) 0x21704` | `ret(0xb5f90)` | `?` | `?` | `ret(0xb5f90)` |
| `bl` at 0x9ab24 | `(unknown callee) 0x21eb8` | `0x9` | `0x11e3d4` | `0x19b5` | `0x0` |
| `bl` at 0x9ab70 | `(unknown callee) 0xb823c` | `[sp-2104]` | `0x1` | `?` | `?` |
| `bl` at 0x9ab98 | `(unknown callee) 0x997ac` | `?` | `sp-2092` | `[sp-2104]` | `sp-2092` |
| `bl` at 0x9abc0 | `(unknown callee) 0x69908` | `sp-32` | `sp-2088` | `ret(0x997ac)` | `sp-2088` |
| `bl` at 0x9ac24 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0x9ac60 | `(unknown callee) 0x69b84` | `sp-32` | `sp-2088` | `?` | `ret(0x1b2bc)` |
| `bl` at 0x9ac78 | `(unknown callee) 0x1b490` | `[sp-2092]` | `?` | `?` | `?` |
| `mov` at 0x9acc4 | `(unknown callee) (unknown target)` | `0x1` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `mov` at 0x9ad18 | `(unknown callee) (unknown target)` | `0x2` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `bl` at 0x9ad50 | `(unknown callee) 0xb74d4` | `0x2b25a0` | `0x0` | `[sp-2104]` | `[sp-32]` |
| `bl` at 0x9ad74 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x11e604` | `?` | `0x145ae8` |
| `bl` at 0x9ad7c | `(unknown callee) 0x21704` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x9ad8c | `(unknown callee) 0x2079c` | `0x11e624` | `ret(0x21704)` | `?` | `ret(0x21704)` |
| `bl` at 0x9ad94 | `(unknown callee) 0xb5fec` | `[sp-2104]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9aaf8 | `ret(0xb5f90) == 0x0` | `ret(0xb5f90) != 0x0` |
| `beq` at 0x9ab34 | `[[sp-2104]+32]:1 == 0x2` | `[[sp-2104]+32]:1 != 0x2` |
| `bne` at 0x9ab54 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9abcc | `[sp-2092] == 0x0` | `[sp-2092] != 0x0` |
| `bne` at 0x9abf0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9ac88 | `[0x14a72c] == 0x0` | `[0x14a72c] != 0x0` |
| `beq` at 0x9ac9c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9acdc | `[0x14a72c] == 0x0` | `[0x14a72c] != 0x0` |
| `beq` at 0x9acf0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9ad64 | `ret(0xb74d4) == 0x0` | `ret(0xb74d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `LpbkInit`

- `LpbkInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3938 to 0xa3a0c.
  Established. A four-byte absolute assignment at `0x14a684` writes `[0x14a684]+1` (`str` at
  0xa39e8). The unnamed `0xb4a70` call carries `0x2b2740`, `0xffff012`, `0x1`, `0xa3a0c` (`bl` at
  0xa3994); its zero result takes the branch to `0xa39d8` (`beq` at 0xa39a8). Loopback endpoint
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa3948 | load | 4 | `abs 0xa39f0` | `-` |
| `ldr` at 0xa394c | load | 4 | `abs 0x14a684` | `-` |
| `ldr` at 0xa3958 | load | 4 | `abs 0xa39f4` | `-` |
| `str` at 0xa395c | store | 4 | `sp -0x1c` | `0x125870` |
| `ldr` at 0xa3964 | load | 4 | `abs 0xa39f8` | `-` |
| `ldr` at 0xa3974 | load | 4 | `abs 0xa39fc` | `-` |
| `ldr` at 0xa3984 | load | 4 | `abs 0xa39fc` | `-` |
| `ldr` at 0xa3988 | load | 4 | `abs 0xa3a00` | `-` |
| `ldr` at 0xa3990 | load | 4 | `abs 0xa3a04` | `-` |
| `str` at 0xa399c | store | 4 | `sp -0x14` | `ret(0xb4a70)` |
| `ldr` at 0xa39a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa39ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa39b8 | load | 4 | `abs 0xa3a08` | `-` |
| `str` at 0xa39bc | store | 4 | `sp -0x1c` | `0x1258ac` |
| `str` at 0xa39c0 | store | 4 | `sp -0x18` | `ret(0x21704)` |
| `ldr` at 0xa39c8 | load | 4 | `abs 0xa39f8` | `-` |
| `ldr` at 0xa39d8 | load | 4 | `abs 0xa39f0` | `-` |
| `ldr` at 0xa39dc | load | 4 | `abs 0xa39f0` | `-` |
| `ldr` at 0xa39e0 | load | 4 | `abs 0x14a684` | `-` |
| `str` at 0xa39e8 | store | 4 | `abs 0x14a684` | `[0x14a684]+1` |
| `ldmdb` at 0xa39ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa39ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa39ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3970 | `(unknown callee) 0x21eb8` | `0x9` | `0x125894` | `0x2a` | `0x0` |
| `bl` at 0xa3980 | `(unknown callee) 0xb5e34` | `0x2b2740` | `0x12` | `0x0` | `?` |
| `bl` at 0xa3994 | `(unknown callee) 0xb4a70` | `0x2b2740` | `0xffff012` | `0x1` | `0xa3a0c` |
| `bl` at 0xa39b0 | `(unknown callee) 0x21704` | `ret(0xb4a70)` | `?` | `?` | `ret(0xb4a70)` |
| `bl` at 0xa39d4 | `(unknown callee) 0x21eb8` | `0x9` | `0x125894` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa3954 | `[0x14a684] == 0x0` | `[0x14a684] != 0x0` |
| `beq` at 0xa39a8 | `ret(0xb4a70) == 0x0` | `ret(0xb4a70) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa39ec | `r0` | `?` |

## `LpbkIRouter`

- `LpbkIRouter`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3a0c to 0xa3e70.
  Established. Included byte stores with unknown effective targets write `0x0`, `0x1` and `0x2`
  (`strb` at 0xa3ad0, `strb` at 0xa3bb8, `strb` at 0xa3cb8). The unnamed `0xb785c` call has
  `0x2b2740`, `sp-24`, `0x0`, `0x1` (`bl` at 0xa3e2c); its nonzero result takes the branch to
  `0xa3e50` (`bne` at 0xa3e40). A later halfword assignment at `[sp-92] +0x2` retains
  `(ret(0x1b2bc)+1 & 0xffff)` (`strh` at 0xa3dec). Loopback record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3a1c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa3a20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3a24 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa3a28 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0xa3a30 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa3a34 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa3a38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3a3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3a48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3a58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3a5c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3a70 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa3a78 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa3a7c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa3a88 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa3a94 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa3aa4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa3ab4 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa3ad0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa3ad4 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa3af0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa3af4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3b0c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa3b10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3b14 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3b28 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3b3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3b50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3b54 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xa3b5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3b60 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3b68 | store | 4 | `sp -0x48` | `ret(0xb823c)` |
| `ldr` at 0xa3b6c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa3b70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa3b74 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xa3b78 | load | 4 | `sp -0x40` | `-` |
| `strh` at 0xa3b94 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xa3b9c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa3bb8 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xa3bbc | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa3bd8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa3bdc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3bf4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xa3bf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3bfc | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3c10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3c24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3c38 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3c3c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa3c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3c48 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3c50 | store | 4 | `sp -0x50` | `ret(0xb823c)` |
| `ldr` at 0xa3c54 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa3c58 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xa3c5c | store | 4 | `[sp-76] +0x0` | `?` |
| `ldr` at 0xa3c60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3c78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3c7c | load | 4 | `sp -0x40` | `-` |
| `ldrh` at 0xa3c90 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xa3c94 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xa3c9c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa3cb8 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xa3cbc | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa3cd8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa3cdc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3cf4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa3cf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3cfc | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3d10 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3d24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3d38 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3d3c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa3d44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3d48 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3d50 | store | 4 | `sp -0x58` | `ret(0xb823c)` |
| `ldr` at 0xa3d54 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa3d58 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa3d5c | store | 4 | `[sp-84] +0x0` | `?` |
| `ldr` at 0xa3d60 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3d78 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa3d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3d80 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3d94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3da4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3da8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3dbc | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3dc0 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xa3dc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3dcc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3dd4 | store | 4 | `sp -0x60` | `ret(0xb823c)` |
| `ldr` at 0xa3dd8 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa3de8 | load | 4 | `sp -0x5c` | `-` |
| `strh` at 0xa3dec | store | 2 | `[sp-92] +0x2` | `(ret(0x1b2bc)+1 & 0xffff)` |
| `ldr` at 0xa3df0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3df8 | store | 4 | `sp -0x40` | `?` |
| `str` at 0xa3e08 | store | 4 | `sp -0x70` | `0x2` |
| `ldr` at 0xa3e0c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3e10 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0xa3e18 | store | 4 | `sp -0x68` | `sp-60` |
| `ldr` at 0xa3e1c | load | 4 | `abs 0xa3e68` | `-` |
| `str` at 0xa3e34 | store | 4 | `sp -0x1c` | `ret(0xb785c)` |
| `ldr` at 0xa3e38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3e44 | load | 4 | `abs 0xa3e6c` | `-` |
| `ldr` at 0xa3e50 | load | 4 | `abs 0xa3e68` | `-` |
| `ldr` at 0xa3e54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3e5c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa3e64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa3e64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa3e64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3b64 | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa3c4c | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa3d4c | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa3dd0 | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa3ddc | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0xa3e2c | `(unknown callee) 0xb785c` | `0x2b2740` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0xa3e48 | `(unknown callee) 0x202d0` | `0x1258dc` | `?` | `?` | `ret(0xb785c)` |
| `bl` at 0xa3e58 | `(unknown callee) 0xb7090` | `0x2b2740` | `ret(0xb785c)` | `?` | `ret(0xb785c)` |
| `bl` at 0xa3e60 | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa3a44 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xa3a50 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xa3a84 | `[sp-100] == 0x1` | `[sp-100] != 0x1` |
| `bgt` at 0xa3a90 | `[sp-100] > 0x1 (signed, 32-bit)` | `[sp-100] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xa3a9c | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0xa3aac | `[sp-100] == 0x2` | `[sp-100] != 0x2` |
| `bne` at 0xa3b34 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa3c1c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa3d1c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa3da0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa3e40 | `ret(0xb785c) != 0x0` | `ret(0xb785c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa3e64 | `r0` | `ret(0xb5fec)` |

## `IcmbPktBuild`

- `IcmbPktBuild`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaedc0 to 0xaee98.
  Established. A byte assignment at `[sp] +0x4` writes `0x0` (`strb` at 0xaee24). The unnamed
  `0x1b394` call has `[sp]+5`, `[sp-24]`, `[sp-25]:1`, `[sp]+5` (`bl` at 0xaee3c). A later unnamed
  `0xaefbc` call carries `([sp-26]:1 + [sp])-2`, unknown, unknown, unknown (`bl` at 0xaee88). Packet
  encoding and checksum meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaedd0 | store | 4 | `sp -0x18` | `r2` |
| `strh` at 0xaedd4 | store | 2 | `sp -0x12` | `(r0 & 0xffff)` |
| `strh` at 0xaedd8 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0xaeddc | store | 1 | `sp -0x19` | `(r3 & 0xff)` |
| `ldrb` at 0xaede0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xaede8 | store | 1 | `sp -0x1a` | `((r3 & 0xff)+7 & 0xff)` |
| `ldrh` at 0xaedec | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xaedf0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xaedfc | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xaee04 | load | 2 | `sp -0x12` | `-` |
| `ldr` at 0xaee18 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xaee24 | store | 1 | `[sp] +0x4` | `0x0` |
| `ldr` at 0xaee28 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xaee30 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xaee38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaee40 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xaee44 | load | 4 | `sp +0x0` | `-` |
| `strh` at 0xaee54 | store | 2 | `sp -0x1c` | `(ret(0xaed2c) & 0xffff)` |
| `ldrb` at 0xaee58 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xaee5c | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xaee70 | load | 2 | `sp -0x1c` | `-` |
| `ldrb` at 0xaee8c | load | 1 | `sp -0x1a` | `-` |
| `ldmdb` at 0xaee94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaee94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaee94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaedf8 | `(unknown callee) 0xaefbc` | `[sp]` | `(r1 & 0xffff)` | `r2` | `(r1 & 0xffff)` |
| `bl` at 0xaee10 | `(unknown callee) 0xaefbc` | `[sp]+2` | `[sp-18]:2` | `[sp-18]:2` | `[sp]+2` |
| `bl` at 0xaee3c | `(unknown callee) 0x1b394` | `[sp]+5` | `[sp-24]` | `[sp-25]:1` | `[sp]+5` |
| `bl` at 0xaee4c | `(unknown callee) 0xaed2c` | `[sp]` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0xaee88 | `(unknown callee) 0xaefbc` | `([sp-26]:1 + [sp])-2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaee94 | `r0` | `[sp-26]:1` |

## `IcmbPktDecode`

- `IcmbPktDecode`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaee98 to 0xaef64.
  Established. The branch takes `([sp-36]:2 + (ret(0xaef64) & 0xffff)) == 0xffff` to `0xaef18` and
  falls through for inequality (`beq` at 0xaef08). A halfword assignment at `[sp-28] +0x0` retains
  `(ret(0xaef64) & 0xffff)` (`strh` at 0xaef30). A later four-byte assignment at `[sp-32] +0x0`
  writes `[sp-20]+5` (`str` at 0xaef4c). Decode validation and field meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaeea8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaeeac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaeeb0 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xaeeb8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xaeebc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xaeec0 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xaeed0 | store | 2 | `sp -0x24` | `(ret(0xaed2c) & 0xffff)` |
| `ldrb` at 0xaeed4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xaeed8 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xaeef0 | store | 2 | `sp -0x26` | `(ret(0xaef64) & 0xffff)` |
| `ldrh` at 0xaeef4 | load | 2 | `sp -0x24` | `-` |
| `ldrh` at 0xaeef8 | load | 2 | `sp -0x26` | `-` |
| `ldr` at 0xaef00 | load | 4 | `abs 0xaef60` | `-` |
| `str` at 0xaef10 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xaef18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaef2c | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xaef30 | store | 2 | `[sp-28] +0x0` | `(ret(0xaef64) & 0xffff)` |
| `ldrb` at 0xaef34 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xaef3c | store | 1 | `sp -0x21` | `([sp-21]:1-7 & 0xff)` |
| `ldr` at 0xaef40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaef44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaef4c | store | 4 | `[sp-32] +0x0` | `[sp-20]+5` |
| `ldrb` at 0xaef50 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xaef54 | store | 4 | `sp -0x2c` | `[sp-33]:1` |
| `ldr` at 0xaef58 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xaef5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaef5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaef5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaeec8 | `(unknown callee) 0xaed2c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xaeee8 | `(unknown callee) 0xaef64` | `([sp-21]:1 + [sp-20])-2` | `?` | `[sp-21]:1` | `([sp-21]:1 + [sp-20])-2` |
| `bl` at 0xaef24 | `(unknown callee) 0xaef64` | `[sp-20]+2` | `?` | `([sp-36]:2 + (ret(0xaef64) & 0xffff))` | `[sp-20]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaef08 | `([sp-36]:2 + (ret(0xaef64) & 0xffff)) == 0xffff` | `([sp-36]:2 + (ret(0xaef64) & 0xffff)) != 0xffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaef5c | `r0` | `?` |

## `IRouterWritePacket`

- `IRouterWritePacket`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7da0 to 0xb7f6c.
  Established. The indirect call has unknown target and candidates `[sp-28]`, `[sp-52]+16`,
  `[sp-40]`, `[sp-52]+16` (`mov` at 0xb7ea0). A call result equal to `[sp-40]` takes the branch to
  `0xb7f30` (`beq` at 0xb7eb8). Inequality falls through (`beq` at 0xb7eb8). The unnamed `0xb7f6c` call carries
  `[sp-52]`, `0x1`, unknown, unknown (`bl` at 0xb7e80). Write callback effects and framing meaning
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7db0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7db4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7db8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb7dbc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb7dc0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7dc4 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb7dc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7dd8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb7ddc | load | 2 | `(unknown)` | `-` |
| `str` at 0xb7de0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb7de4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb7de8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb7df8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb7dfc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7e10 | store | 4 | `sp -0x34` | `ret(0x1b1b4)` |
| `ldr` at 0xb7e14 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7e20 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb7e24 | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xb7e34 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb7e38 | load | 4 | `abs 0xb7f60` | `-` |
| `ldr` at 0xb7e3c | load | 4 | `abs 0xb7f64` | `-` |
| `ldr` at 0xb7e40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7e4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7e54 | load | 4 | `abs 0xb7f64` | `-` |
| `str` at 0xb7e58 | store | 4 | `sp -0x38` | `0xfffffda4` |
| `ldr` at 0xb7e60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7e68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7e78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7e84 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7e90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7e98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7ea8 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0xb7eac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7eb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7ebc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7ec8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb7ecc | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xb7edc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb7ee0 | load | 4 | `abs 0xb7f60` | `-` |
| `ldr` at 0xb7ee8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7ef4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7efc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7f04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7f10 | load | 4 | `abs 0xb7f68` | `-` |
| `str` at 0xb7f14 | store | 4 | `sp -0x3c` | `0xfffffd92` |
| `str` at 0xb7f20 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb7f24 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7f28 | store | 4 | `sp -0x38` | `[sp-60]` |
| `ldr` at 0xb7f30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7f38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7f3c | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xb7f40 | store | 4 | `sp -0x24` | `[[sp-36]]` |
| `ldr` at 0xb7f48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7f54 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb7f58 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xb7f5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7f5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7f5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7e08 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xb7e48 | `(unknown callee) 0xb74d4` | `0x2b2a60` | `0xfffffda4` | `[sp-32]` | `0x0` |
| `bl` at 0xb7e50 | `(unknown callee) 0xb5fec` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xb7e74 | `(unknown callee) 0x1b394` | `ret(0x1b1b4)` | `[sp-36]` | `[sp-40]+16` | `[sp-40]+16` |
| `bl` at 0xb7e80 | `(unknown callee) 0xb7f6c` | `[sp-52]` | `0x1` | `?` | `?` |
| `mov` at 0xb7ea0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-52]+16` | `[sp-40]` | `[sp-52]+16` |
| `bl` at 0xb7ef0 | `(unknown callee) 0xb74d4` | `0x2b2a60` | `?` | `[sp-32]` | `0x0` |
| `bl` at 0xb7ef8 | `(unknown callee) 0x1b490` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0xb7f00 | `(unknown callee) 0xb5fec` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xb7f34 | `(unknown callee) 0x1b490` | `[sp-52]` | `?` | `ret(unknown)` | `[sp-40]` |
| `bl` at 0xb7f4c | `(unknown callee) 0xb5fec` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb7dd0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb7e1c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xb7e2c | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `beq` at 0xb7eb8 | `ret(unknown) == [sp-40]` | `ret(unknown) != [sp-40]` |
| `bge` at 0xb7ec4 | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `bne` at 0xb7ed4 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `bne` at 0xb7f0c | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7f5c | `r0` | `?` |

## `IRouterSetFrameByteOrder`

- `IRouterSetFrameByteOrder`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7f6c to 0xb823c.
  Established. The branch takes `r1 != 0x2` to `0xb80f8` and falls through for equality (`bne` at
  0xb7fb0). Included halfword assignments at `r0 +0x10` retain `(ret(0x1b298) & 0xffff)` and
  `(ret(0x1b4b4) & 0xffff)` on separate routes (`strh` at 0xb7fcc, `strh` at 0xb8110). Byte stores
  at `[sp-24] +0x6` retain `([sp-36]:1 & 0xff)` and `([sp-40]:1 & 0xff)` respectively (`strb` at
  0xb80f0, `strb` at 0xb8234). Byte-order markers and indexed record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7f7c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb7f80 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb7f84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7f88 | load | 1 | `r0 +0x6` | `-` |
| `ldr` at 0xb7f98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7fa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7fb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7fb8 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb7fbc | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0xb7fcc | store | 2 | `r0 +0x10` | `(ret(0x1b298) & 0xffff)` |
| `ldr` at 0xb7fd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7fd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7fd8 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0xb7fe4 | store | 4 | `[sp-24] +0x18` | `ret(0x1aff8)` |
| `ldr` at 0xb7fe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7fec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7ff0 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xb7ffc | store | 4 | `[sp-24] +0x1c` | `ret(0x1aff8)` |
| `ldr` at 0xb8000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8004 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8008 | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0xb8018 | store | 2 | `[sp-24] +0x22` | `(ret(0x1b298) & 0xffff)` |
| `str` at 0xb8020 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb8024 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8028 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xb802c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb803c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8040 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8054 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8058 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb806c | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb807c | store | 2 | `(unknown)` | `(ret(0x1b298) & 0xffff)` |
| `ldr` at 0xb8080 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8084 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8098 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb809c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb80b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb80bc | store | 4 | `(unknown)` | `ret(0x1aff8)` |
| `ldr` at 0xb80c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb80c8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xb80d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb80d4 | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xb80d8 | store | 1 | `sp -0x24` | `([[sp-24]+6]:1 & 0xff)` |
| `ldrb` at 0xb80dc | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xb80e8 | store | 1 | `sp -0x24` | `?` |
| `ldrb` at 0xb80ec | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xb80f0 | store | 1 | `[sp-24] +0x6` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xb80f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb80fc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8100 | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0xb8110 | store | 2 | `r0 +0x10` | `(ret(0x1b4b4) & 0xffff)` |
| `ldr` at 0xb8114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8118 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb811c | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0xb8128 | store | 4 | `[sp-24] +0x18` | `ret(0x1b100)` |
| `ldr` at 0xb812c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8134 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xb8140 | store | 4 | `[sp-24] +0x1c` | `ret(0x1b100)` |
| `ldr` at 0xb8144 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8148 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb814c | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0xb815c | store | 2 | `[sp-24] +0x22` | `(ret(0x1b4b4) & 0xffff)` |
| `str` at 0xb8164 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb8168 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb816c | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xb8170 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8184 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb819c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb81b0 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb81c0 | store | 2 | `(unknown)` | `(ret(0x1b4b4) & 0xffff)` |
| `ldr` at 0xb81c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb81c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb81dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb81e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb81f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8200 | store | 4 | `(unknown)` | `ret(0x1b100)` |
| `ldr` at 0xb8204 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb820c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xb8214 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8218 | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xb821c | store | 1 | `sp -0x28` | `([[sp-24]+6]:1 & 0xff)` |
| `ldrb` at 0xb8220 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xb822c | store | 1 | `sp -0x28` | `?` |
| `ldrb` at 0xb8230 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xb8234 | store | 1 | `[sp-24] +0x6` | `([sp-40]:1 & 0xff)` |
| `ldmdb` at 0xb8238 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb8238 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8238 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8238 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7fc4 | `(unknown callee) 0x1b298` | `[r0+16]:2` | `r1` | `?` | `[r0+16]:2` |
| `bl` at 0xb7fdc | `(unknown callee) 0x1aff8` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb7ff4 | `(unknown callee) 0x1aff8` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb8010 | `(unknown callee) 0x1b298` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0xb8074 | `(unknown callee) 0x1b298` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0xb80b4 | `(unknown callee) 0x1aff8` | `?` | `[sp-24]` | `0x28` | `?` |
| `bl` at 0xb8108 | `(unknown callee) 0x1b4b4` | `[r0+16]:2` | `r1` | `?` | `[r0+16]:2` |
| `bl` at 0xb8120 | `(unknown callee) 0x1b100` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb8138 | `(unknown callee) 0x1b100` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb8154 | `(unknown callee) 0x1b4b4` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0xb81b8 | `(unknown callee) 0x1b4b4` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0xb81f8 | `(unknown callee) 0x1b100` | `?` | `[sp-24]` | `0x28` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb7fa0 | `? != r1` | `? == r1` |
| `bne` at 0xb7fb0 | `r1 != 0x2` | `r1 == 0x2` |
| `blt` at 0xb8034 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb8178 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8238 | `r0` | `?` |

## `IRArg`

- `IRArg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb823c to 0xb85bc.
  Established. The branch takes signed `r1 < [r0+32]:1` to `0xb8274` and falls through for greater
  or equal values (`blt` at 0xb8264). An included four-byte assignment at `[sp-24] +0x8` retains
  `ret(0x1b1b4)` (`str` at 0xb83e4). The later unnamed `0x1b394` call has `[sp-36]`, `[sp-40]`,
  `[sp-44]`, unknown (`bl` at 0xb8528). Argument record layout and allocation units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb824c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb8250 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb8254 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8258 | load | 1 | `r0 +0x20` | `-` |
| `ldr` at 0xb825c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb826c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb8274 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8278 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb828c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb829c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb82a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb82b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb82b8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb82c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb82c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb82d8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb82dc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xb82e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb82e4 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xb82e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb82f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb82fc | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0xb830c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8310 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8314 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb831c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb832c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb8334 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8338 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb834c | load | 2 | `(unknown)` | `-` |
| `str` at 0xb8350 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb8354 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb836c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8370 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8378 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb837c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb8380 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xb8384 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb838c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8398 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb839c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb83a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb83ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb83b0 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xb83b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb83bc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb83c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb83cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb83d0 | load | 1 | `[sp-24] +0x20` | `-` |
| `str` at 0xb83e4 | store | 4 | `[sp-24] +0x8` | `ret(0x1b1b4)` |
| `ldr` at 0xb83e8 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xb83f8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb8400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8404 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8408 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xb8410 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb8420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8424 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb842c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb8434 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8440 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb844c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb8454 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8458 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb8460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8464 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb846c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb8474 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8478 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb848c | load | 2 | `(unknown)` | `-` |
| `str` at 0xb849c | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xb84a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb84b0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb84b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb84bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb84c4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb84cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xb84d0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb84d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb84d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb84ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb84f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb84f8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb84fc | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb8500 | load | 2 | `[sp-32] +0x10` | `-` |
| `ldr` at 0xb8504 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb850c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8518 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb851c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8524 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb852c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8530 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb8538 | store | 4 | `sp -0x24` | `([sp-36] + [sp-44])` |
| `ldr` at 0xb853c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb8540 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb8548 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb854c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb8558 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb855c | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xb8560 | store | 4 | `sp -0x20` | `[[sp-32]]` |
| `ldr` at 0xb8564 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb856c | store | 4 | `sp -0x28` | `[[sp-32]]+36` |
| `ldr` at 0xb8570 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb8574 | load | 2 | `[[sp-32]] +0x10` | `-` |
| `ldr` at 0xb857c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb8588 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb858c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb8598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb859c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb85a4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xb85ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xb85b0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb85b4 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xb85b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb85b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb85b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb85b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb83dc | `(unknown callee) 0x1b1b4` | `?` | `r0` | `[sp-52]` | `?` |
| `bl` at 0xb841c | `(unknown callee) 0x1b4cc` | `[[sp-24]+8]` | `0x0` | `?` | `?` |
| `bl` at 0xb8494 | `(unknown callee) 0x1b1b4` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0xb8528 | `(unknown callee) 0x1b394` | `[sp-36]` | `[sp-40]` | `[sp-44]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb8264 | `r1 < [r0+32]:1 (signed, 32-bit)` | `r1 >= [r0+32]:1 (signed, 32-bit)` |
| `bne` at 0xb8298 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb82f0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb8304 | `[sp-48]-1 != 0x0` | `[sp-48]-1 == 0x0` |
| `bne` at 0xb8324 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb83a8 | `[sp-52] > [sp-44] (unsigned, 32-bit)` | `[sp-52] <= [sp-44] (unsigned, 32-bit)` |
| `bne` at 0xb83c4 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |
| `bne` at 0xb83f0 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |
| `beq` at 0xb843c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb84a8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb8554 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `bne` at 0xb8594 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb85b8 | `r0` | `?` |

## `routerExitServiceMode`

- `routerExitServiceMode`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7d78 to 0xc7ea4.
  Established. A four-byte absolute assignment at `0x148fd0` writes `0x2` (`str` at 0xc7dec). The
  earlier branch takes `[sp-20] != 0x1` to `0xc7de4` and falls through for equality (`bne` at
  0xc7dcc). Included unnamed `0x4257c` calls carry `0x0`, `0x6`, `0x2`, unknown and `0x0`, `0x7`,
  `0x2`, unknown respectively (`bl` at 0xc7e78, `bl` at 0xc7e88). Service mode and port meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7d88 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc7d8c | load | 4 | `abs 0xc7e98` | `-` |
| `ldm` at 0xc7d94 | load | 4 | `abs 0x133a30` | `-` |
| `ldm` at 0xc7d94 | load | 4 | `abs 0x133a34` | `-` |
| `ldm` at 0xc7d94 | load | 4 | `abs 0x133a38` | `-` |
| `stm` at 0xc7d98 | store | 4 | `sp -0x20` | `[0x133a30]` |
| `stm` at 0xc7d98 | store | 4 | `sp -0x1c` | `[0x133a34]` |
| `strb` at 0xc7d9c | store | 1 | `sp -0x18` | `([0x133a38] & 0xff)` |
| `ldr` at 0xc7dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc7de4 | load | 4 | `abs 0xc7e9c` | `-` |
| `str` at 0xc7dec | store | 4 | `abs 0x148fd0` | `0x2` |
| `ldr` at 0xc7e8c | load | 4 | `abs 0xc7ea0` | `-` |
| `ldmdb` at 0xc7e94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7e94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7e94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc7da8 | `(unknown callee) 0x1d41c` | `0x50` | `0x0` | `[0x133a38]` | `0x133a30` |
| `bl` at 0xc7db4 | `(unknown callee) 0x1d41c` | `0x3e` | `0x0` | `?` | `?` |
| `bl` at 0xc7dc0 | `(unknown callee) 0x1d41c` | `0x3f` | `0x0` | `?` | `?` |
| `bl` at 0xc7ddc | `(unknown callee) 0x1d41c` | `0xd` | `sp-32` | `?` | `sp-32` |
| `bl` at 0xc7dfc | `(unknown callee) 0x1d41c` | `0xd` | `sp-32` | `0x148fd0` | `sp-32` |
| `bl` at 0xc7e08 | `(unknown callee) 0x1d41c` | `0x3d` | `0x2` | `?` | `?` |
| `bl` at 0xc7e14 | `(unknown callee) 0x1d41c` | `0x7b` | `0x1` | `?` | `?` |
| `bl` at 0xc7e18 | `(unknown callee) 0x4f4d0` | `ret(0x1d41c)` | `?` | `?` | `?` |
| `bl` at 0xc7e2c | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0xc7e40 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0xc7e54 | `(unknown callee) 0x4f4d0` | `ret(0x4f4d0)` | `?` | `?` | `?` |
| `bl` at 0xc7e78 | `(unknown callee) 0x4257c` | `0x0` | `0x6` | `0x2` | `?` |
| `bl` at 0xc7e88 | `(unknown callee) 0x4257c` | `0x0` | `0x7` | `0x2` | `?` |
| `bl` at 0xc7e90 | `(unknown callee) 0x202d0` | `0x133a3c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc7dcc | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0xc7e28 | `? == 0x52` | `? != 0x52` |
| `beq` at 0xc7e3c | `? == 0x44` | `? != 0x44` |
| `beq` at 0xc7e50 | `? == 0x36` | `? != 0x36` |
| `beq` at 0xc7e64 | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7e94 | `r0` | `ret(0x202d0)` |
