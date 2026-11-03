# ARM Linux 2.6 router packet paths in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
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
The indirect calls (`mov` at 0xfd640, `mov` at 0xfdae0) have unknown targets.
Their continuations are retained. No unresolved computed transfers were reported.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-router-packets-facts.txt) retain all tool sections.

## `IRouterInit`

- `IRouterInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfb4f8 to 0xfb5cc.
  Established. Separate word assignments write `0x1` and `0x2` to `0x1b1004` (`str` at 0xfb528, `str` at 0xfb538).
  The unnamed `0xfb5cc` call has candidate r0/r1/r2/r3 values `0x31baac`, `0x1`, `0x3` and unknown (`bl` at 0xfb574).
  One unnamed `0x22b84` call has candidate r0/r1/r2/r3 values `0x1b1028`, `0x18810c`, `0x1` and `0x9` (`bl` at 0xfb588).
  Router initialization fields and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfb508 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfb510 | store | 4 | `sp -0x18` | `0x1` |
| `ldrb` at 0xfb514 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xfb520 | load | 4 | `abs 0xfb5b0` | `-` |
| `str` at 0xfb528 | store | 4 | `abs 0x1b1004` | `0x1` |
| `ldr` at 0xfb530 | load | 4 | `abs 0xfb5b0` | `-` |
| `str` at 0xfb538 | store | 4 | `abs 0x1b1004` | `0x2` |
| `str` at 0xfb540 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xfb544 | load | 4 | `abs 0xfb5b4` | `-` |
| `ldr` at 0xfb548 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfb564 | store | 4 | `abs 0x31bab4` | `?` |
| `ldr` at 0xfb568 | load | 4 | `abs 0xfb5b8` | `-` |
| `ldr` at 0xfb578 | load | 4 | `abs 0xfb5bc` | `-` |
| `ldr` at 0xfb57c | load | 4 | `abs 0xfb5c0` | `-` |
| `ldr` at 0xfb58c | load | 4 | `abs 0xfb5c4` | `-` |
| `ldr` at 0xfb590 | load | 4 | `abs 0xfb5c8` | `-` |
| `ldr` at 0xfb5a0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xfb5ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfb5ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfb5ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfb574 | `(unknown callee) 0xfb5cc` | `0x31baac` | `0x1` | `0x3` | `?` |
| `bl` at 0xfb588 | `(unknown callee) 0x22b84` | `0x1b1028` | `0x18810c` | `0x1` | `0x9` |
| `bl` at 0xfb59c | `(unknown callee) 0x22b84` | `0x1b107c` | `0x188118` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfb51c | `[sp-21]:1 == 0x0` | `[sp-21]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfb5ac | `r0` | `[sp-24]` |

## `IRouterInitID`

- `IRouterInitID`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfb5cc to 0xfb6b8.
  Established. A byte assignment writes `([sp-28] & 0xff)` to `[sp-20] +0x4` (`strb` at 0xfb694).
  A halfword assignment clears `[sp-20] +0x6` (`strh` at 0xfb6a0).
  An earlier word assignment to entry `r0 +0x0` has unknown value (`str` at 0xfb614).
  Identifier field meanings and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfb5dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfb5e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfb5e4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xfb5e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb5ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb5f0 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xfb5fc | load | 4 | `abs 0xfb6b4` | `-` |
| `ldr` at 0xfb600 | load | 4 | `abs 0x31bab4` | `-` |
| `str` at 0xfb614 | store | 4 | `r0 +0x0` | `?` |
| `ldr` at 0xfb618 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfb61c | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0xfb620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb624 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xfb630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb638 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xfb640 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfb650 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xfb654 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xfb65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb660 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb664 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xfb66c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfb678 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xfb67c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xfb680 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfb684 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfb688 | store | 4 | `[sp-32] +0x0` | `[sp-36]` |
| `ldr` at 0xfb68c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb690 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xfb694 | store | 1 | `[sp-20] +0x4` | `([sp-28] & 0xff)` |
| `ldr` at 0xfb698 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xfb6a0 | store | 2 | `[sp-20] +0x6` | `0x0` |
| `ldm` at 0xfb6b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfb6b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfb6b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xfb62c | `[[sp-20]] >= 0x0 (signed, 32-bit)` | `[[sp-20]] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfb6b0 | `r0` | `0x0` |

## `IRouterSetAuthority`

- `IRouterSetAuthority`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfb6b8 to 0xfb6ec.
  Established. A byte assignment writes `(r1 & 0xff)` to entry `r0 +0x4` (`strb` at 0xfb6d8).
  The integer return candidate is `0x0` (`ldm` at 0xfb6e8).
  Authority encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfb6c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfb6cc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xfb6d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb6d4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xfb6d8 | store | 1 | `r0 +0x4` | `(r1 & 0xff)` |
| `ldm` at 0xfb6e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfb6e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfb6e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfb6e8 | `r0` | `0x0` |

## `IRouterCreateQueue`

- `IRouterCreateQueue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfb6ec to 0xfb734.
  Established. A word assignment places `0x1` at `sp -0x20` (`str` at 0xfb70c).
  The unnamed `0x231fc` call has candidate r0/r1/r2/r3 values entry `r0`, entry `r1`, entry `r2` and `0x7` (`bl` at 0xfb720).
  The integer return candidate is `ret(0x231fc)` (`ldm` at 0xfb730).
  Queue ownership and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfb6fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfb700 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfb704 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfb70c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xfb710 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb714 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfb718 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xfb730 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfb730 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfb730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfb720 | `(unknown callee) 0x231fc` | `r0` | `r1` | `r2` | `0x7` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfb730 | `r0` | `ret(0x231fc)` |

## `IRouterGetPacket`

- `IRouterGetPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfb734 to 0xfb794.
  Established. The unnamed `0x23680` call has candidate r0/r1/r2/r3 values entry `r1`, entry `r0`, entry `r2` and entry `r3` (`bl` at 0xfb75c);
  its zero result takes `0xfb780`, while nonzero falls through (`beq` at 0xfb770).
  The taken path clears the word at `sp -0x28` (`str` at 0xfb784).
  Packet retrieval and hardware ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfb744 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfb748 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfb74c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xfb750 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfb754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb758 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfb764 | store | 4 | `sp -0x24` | `ret(0x23680)` |
| `ldr` at 0xfb768 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfb774 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfb778 | store | 4 | `sp -0x28` | `ret(0x23680)` |
| `str` at 0xfb784 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xfb788 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xfb790 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfb790 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfb790 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfb75c | `(unknown callee) 0x23680` | `r1` | `r0` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfb770 | `ret(0x23680) == 0x0` | `ret(0x23680) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfb790 | `r0` | `[sp-40]` |

## `IRouterFreePacket`

- `IRouterFreePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfb794 to 0xfbb58.
  Established. A word assignment clears `0x1b1020` (`str` at 0xfba68).
  Another writes `[0x1b1018]-1` to `0x1b1018` (`str` at 0xfba90).
  A halfword assignment writes `([sp-40]:2 & 0xffff)` to `[sp-20] +0x4` (`strh` at 0xfb964).
  The unnamed `0x20514` call has candidate r0/r1/r2/r3 values `[[sp-20]+8]`, unknown, `[[sp-20]+32]:1` and `[sp-20]` (`bl` at 0xfba04).
  Packet lifetime fields and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfb7a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xfb7a8 | load | 4 | `abs 0xfbb18` | `-` |
| `str` at 0xfb7b8 | store | 4 | `sp -0x20` | `ret(0x22ca0)` |
| `ldr` at 0xfb7bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfb7c8 | load | 4 | `abs 0xfbb1c` | `-` |
| `str` at 0xfb7cc | store | 4 | `sp -0x34` | `0x188068` |
| `ldr` at 0xfb7d0 | load | 4 | `abs 0xfbb18` | `-` |
| `str` at 0xfb7d4 | store | 4 | `sp -0x30` | `0x1b107c` |
| `ldr` at 0xfb7d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfb7dc | store | 4 | `sp -0x2c` | `ret(0x22ca0)` |
| `ldr` at 0xfb7e4 | load | 4 | `abs 0xfbb20` | `-` |
| `ldr` at 0xfb7f4 | load | 4 | `abs 0xfbb24` | `-` |
| `ldr` at 0xfb7f8 | load | 4 | `abs 0x1b1008` | `-` |
| `ldr` at 0xfb804 | load | 4 | `abs 0xfbb28` | `-` |
| `ldr` at 0xfb808 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfb80c | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xfb820 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xfb824 | load | 4 | `abs 0xfbb2c` | `-` |
| `ldr` at 0xfb828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb830 | load | 4 | `abs 0xfbb28` | `-` |
| `ldr` at 0xfb834 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xfb83c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfb840 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xfb860 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xfb864 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0xfb868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb86c | load | 4 | `abs 0xfbb30` | `-` |
| `ldr` at 0xfb874 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xfb880 | load | 4 | `abs 0xfbb18` | `-` |
| `str` at 0xfb88c | store | 4 | `sp -0x20` | `ret(0x2302c)` |
| `ldr` at 0xfb890 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfb89c | load | 4 | `abs 0xfbb34` | `-` |
| `str` at 0xfb8a0 | store | 4 | `sp -0x34` | `0x1880c0` |
| `ldr` at 0xfb8a4 | load | 4 | `abs 0xfbb18` | `-` |
| `str` at 0xfb8a8 | store | 4 | `sp -0x30` | `0x1b107c` |
| `ldr` at 0xfb8ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfb8b0 | store | 4 | `sp -0x2c` | `ret(0x2302c)` |
| `ldr` at 0xfb8b8 | load | 4 | `abs 0xfbb20` | `-` |
| `ldr` at 0xfb8bc | load | 4 | `abs 0xfbb38` | `-` |
| `str` at 0xfb8cc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xfb8d4 | load | 4 | `abs 0xfbb3c` | `-` |
| `ldr` at 0xfb8d8 | load | 4 | `abs 0x1b1020` | `-` |
| `ldr` at 0xfb8e4 | load | 4 | `abs 0xfbb3c` | `-` |
| `ldr` at 0xfb8e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb8ec | load | 4 | `abs 0x1b1020` | `-` |
| `ldr` at 0xfb8f8 | load | 4 | `abs 0xfbb28` | `-` |
| `ldr` at 0xfb8fc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfb900 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xfb914 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xfb918 | load | 4 | `abs 0xfbb40` | `-` |
| `ldr` at 0xfb91c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb928 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfb92c | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xfb938 | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xfb93c | store | 2 | `sp -0x28` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xfb940 | load | 4 | `abs 0xfbb44` | `-` |
| `ldrh` at 0xfb94c | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xfb95c | store | 2 | `sp -0x28` | `?` |
| `ldrh` at 0xfb960 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xfb964 | store | 2 | `[sp-20] +0x4` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xfb984 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb988 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xfb998 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xfb99c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xfb9a0 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xfb9a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfb9b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb9b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfb9bc | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xfb9c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfb9d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfb9d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfb9dc | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xfb9e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfb9ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfb9f4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xfb9fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfba00 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xfba08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfba0c | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0xfba10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfba1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfba20 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xfba24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfba28 | load | 4 | `(unknown)` | `-` |
| `str` at 0xfba2c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xfba30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfba3c | load | 4 | `abs 0xfbb3c` | `-` |
| `ldr` at 0xfba40 | load | 4 | `abs 0x1b1020` | `-` |
| `ldr` at 0xfba4c | load | 4 | `abs 0xfbb3c` | `-` |
| `ldr` at 0xfba50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfba54 | load | 4 | `abs 0x1b1020` | `-` |
| `ldr` at 0xfba60 | load | 4 | `abs 0xfbb3c` | `-` |
| `str` at 0xfba68 | store | 4 | `abs 0x1b1020` | `0x0` |
| `ldr` at 0xfba6c | load | 4 | `abs 0xfbb28` | `-` |
| `ldr` at 0xfba70 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xfba74 | load | 4 | `abs 0xfbb48` | `-` |
| `ldr` at 0xfba78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfba80 | load | 4 | `abs 0xfbb4c` | `-` |
| `ldr` at 0xfba84 | load | 4 | `abs 0xfbb4c` | `-` |
| `ldr` at 0xfba88 | load | 4 | `abs 0x1b1018` | `-` |
| `str` at 0xfba90 | store | 4 | `abs 0x1b1018` | `[0x1b1018]-1` |
| `ldr` at 0xfba94 | load | 4 | `abs 0xfbb24` | `-` |
| `ldr` at 0xfba98 | load | 4 | `abs 0x1b1008` | `-` |
| `ldr` at 0xfbaa4 | load | 4 | `abs 0xfbb28` | `-` |
| `ldr` at 0xfbaa8 | load | 4 | `abs 0xfbb4c` | `-` |
| `ldr` at 0xfbaac | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xfbab0 | load | 4 | `abs 0xfbb50` | `-` |
| `ldr` at 0xfbab4 | load | 4 | `abs 0x1b1018` | `-` |
| `ldr` at 0xfbabc | load | 4 | `abs 0xfbb18` | `-` |
| `str` at 0xfbac8 | store | 4 | `sp -0x20` | `ret(0x2302c)` |
| `ldr` at 0xfbacc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfbad8 | load | 4 | `abs 0xfbb34` | `-` |
| `str` at 0xfbadc | store | 4 | `sp -0x34` | `0x1880c0` |
| `ldr` at 0xfbae0 | load | 4 | `abs 0xfbb18` | `-` |
| `str` at 0xfbae4 | store | 4 | `sp -0x30` | `0x1b107c` |
| `ldr` at 0xfbae8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfbaec | store | 4 | `sp -0x2c` | `ret(0x2302c)` |
| `ldr` at 0xfbaf4 | load | 4 | `abs 0xfbb20` | `-` |
| `ldr` at 0xfbaf8 | load | 4 | `abs 0xfbb54` | `-` |
| `str` at 0xfbb08 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xfbb0c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xfbb14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfbb14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfbb14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfb7b0 | `(unknown callee) 0x22ca0` | `0x1b107c` | `?` | `r2` | `r3` |
| `bl` at 0xfb7f0 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x3e4` | `0x0` |
| `bl` at 0xfb82c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x188124` | `[sp-20]` | `?` |
| `bl` at 0xfb838 | `(unknown callee) 0x200dc` | `[0x1a288c]` | `?` | `?` | `0x1a288c` |
| `bl` at 0xfb858 | `(unknown callee) 0x228ec` | `?` | `?` | `?` | `?` |
| `bl` at 0xfb87c | `(unknown callee) 0x29908` | `0x188134` | `[[sp-20]+22]:1` | `[[sp-20]+28]` | `ret(0x228ec)` |
| `bl` at 0xfb884 | `(unknown callee) 0x2302c` | `0x1b107c` | `?` | `?` | `?` |
| `bl` at 0xfb8c4 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x3f1` | `0x0` |
| `bl` at 0xfb920 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x188160` | `[sp-20]` | `?` |
| `bl` at 0xfb9e8 | `(unknown callee) 0x20514` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0xfba04 | `(unknown callee) 0x20514` | `[[sp-20]+8]` | `?` | `[[sp-20]+32]:1` | `[sp-20]` |
| `bl` at 0xfba34 | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |
| `bl` at 0xfba7c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x18816c` | `[sp-20]` | `0x1a288c` |
| `bl` at 0xfbab8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x188178` | `[0x1b1018]` | `0x1a288c` |
| `bl` at 0xfbac0 | `(unknown callee) 0x2302c` | `0x1b107c` | `?` | `?` | `?` |
| `bl` at 0xfbb00 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x421` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfb7c4 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0xfb800 | `[0x1b1008] == 0x0` | `[0x1b1008] != 0x0` |
| `bne` at 0xfb854 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xfb898 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |
| `beq` at 0xfb8e0 | `[0x1b1020] == 0x0` | `[0x1b1020] != 0x0` |
| `bne` at 0xfb8f4 | `[sp-20] != [0x1b1020]` | `[sp-20] == [0x1b1020]` |
| `bne` at 0xfb980 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xfb990 | `[[sp-20]+8] == 0x0` | `[[sp-20]+8] != 0x0` |
| `ble` at 0xfb9ac | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xfb9cc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfba18 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfba48 | `[0x1b1020] == 0x0` | `[0x1b1020] != 0x0` |
| `bne` at 0xfba5c | `[sp-20] != [0x1b1020]` | `[sp-20] == [0x1b1020]` |
| `beq` at 0xfbaa0 | `[0x1b1008] == 0x0` | `[0x1b1008] != 0x0` |
| `beq` at 0xfbad4 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfbb14 | `r0` | `0x0` |

## `IRouterSendPacket`

- `IRouterSendPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfca84 to 0xfcb5c.
  Established. A byte read addresses entry `r1 +0x16` (`ldrb` at 0xfcaa0).
  The signed condition `[r1+22]:1 <= 0x1` takes `0xfcad0`, while higher falls through (`ble` at 0xfcabc).
  The unnamed `0xfbf94` call has candidate r0/r1/r2/r3 values entry `r0`, entry `r1`, `0x1` and `[r1+22]:1` (`bl` at 0xfcadc).
  Packet type encoding and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfca94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfca98 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xfca9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xfcaa0 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xfcaa4 | store | 4 | `sp -0x20` | `[r1+22]:1` |
| `ldr` at 0xfcaa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfcab4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfcac0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfcad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcad4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfcae4 | store | 4 | `sp -0x1c` | `ret(0xfbf94)` |
| `ldr` at 0xfcae8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfcaf4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xfcaf8 | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0xfcb08 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xfcb0c | load | 4 | `abs 0xfcb58` | `-` |
| `ldr` at 0xfcb10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfcb14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfcb24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcb28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfcb34 | store | 4 | `sp -0x1c` | `ret(0xfc678)` |
| `str` at 0xfcb3c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xfcb40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfcb48 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xfcb54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfcb54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfcb54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfcadc | `(unknown callee) 0xfbf94` | `r0` | `r1` | `0x1` | `[r1+22]:1` |
| `bl` at 0xfcb1c | `(unknown callee) 0xfd0dc` | `0x31baac` | `ret(0xfbf94)` | `[sp-24]` | `0x0` |
| `bl` at 0xfcb2c | `(unknown callee) 0xfc678` | `r0` | `r1` | `r2` | `[r1+22]:1` |
| `bl` at 0xfcb44 | `(unknown callee) 0xfb794` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xfcab0 | `[r1+22]:1 < 0x0 (signed, 32-bit)` | `[r1+22]:1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0xfcabc | `[r1+22]:1 <= 0x1 (signed, 32-bit)` | `[r1+22]:1 > 0x1 (signed, 32-bit)` |
| `beq` at 0xfcac8 | `[r1+22]:1 == 0x2` | `[r1+22]:1 != 0x2` |
| `beq` at 0xfcaf0 | `ret(0xfbf94) == 0x0` | `ret(0xfbf94) != 0x0` |
| `bne` at 0xfcb00 | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfcb54 | `r0` | `[sp-28]` |

## `IRouterReadPacket`

- `IRouterReadPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfd578 to 0xfd9e4.
  Established. A call to an unknown target has candidate r0/r1/r2/r3 values `[sp-28]`, `sp-44`, `[sp-64]` and `sp-44` (`mov` at 0xfd640);
  its signed positive result takes `0xfd770`, while nonpositive falls through (`bgt` at 0xfd654).
  A word assignment clears `[sp-44] +0x0` (`str` at 0xfd934).
  Another writes `[sp-40]` to `[sp-32] +0x0` (`str` at 0xfd994).
  Packet assembly fields and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfd588 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfd58c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfd590 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfd594 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xfd59c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xfd5a4 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xfd5ac | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xfd5b4 | store | 4 | `sp -0x3c` | `0x410` |
| `ldr` at 0xfd5b8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xfd5c0 | store | 4 | `sp -0x40` | `0x400` |
| `ldr` at 0xfd5c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xfd5d0 | store | 4 | `sp -0x2c` | `ret(0x201fc)` |
| `ldr` at 0xfd5d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd5e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd5ec | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd5f0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xfd5f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd5f8 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xfd5fc | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xfd600 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfd60c | load | 4 | `abs 0xfd9b4` | `-` |
| `str` at 0xfd610 | store | 4 | `sp -0x60` | `0xfffffda4` |
| `ldr` at 0xfd618 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd62c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfd630 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfd638 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xfd648 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xfd64c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfd658 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd65c | store | 4 | `sp -0x64` | `ret(unknown)` |
| `ldr` at 0xfd660 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xfd66c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xfd680 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xfd688 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd68c | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xfd694 | load | 4 | `abs 0xfd9b8` | `-` |
| `str` at 0xfd698 | store | 4 | `sp -0x44` | `0xfffffd9c` |
| `ldr` at 0xfd69c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd6a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd6ac | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xfd6b0 | store | 4 | `sp -0x48` | `[[sp-40]]` |
| `ldr` at 0xfd6b4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xfd6c0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xfd6c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xfd6c8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xfd6cc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xfd6d4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xfd6d8 | store | 4 | `sp -0x48` | `[sp-76]` |
| `ldr` at 0xfd6e0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd6e8 | store | 4 | `[sp-40] +0x0` | `0x0` |
| `ldr` at 0xfd6ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd6f8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xfd6fc | load | 1 | `[sp-40] +0x16` | `-` |
| `str` at 0xfd70c | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xfd710 | load | 4 | `abs 0xfd9bc` | `-` |
| `ldr` at 0xfd714 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xfd718 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd724 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd730 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd738 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd744 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd748 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xfd74c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd750 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xfd754 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xfd758 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfd764 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xfd768 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xfd770 | load | 4 | `abs 0xfd9c0` | `-` |
| `ldr` at 0xfd774 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd778 | load | 4 | `abs 0x1b1004` | `-` |
| `ldr` at 0xfd780 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd78c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xfd790 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xfd7a4 | load | 4 | `abs 0xfd9c4` | `-` |
| `ldr` at 0xfd7a8 | load | 4 | `abs 0x1b100c` | `-` |
| `ldr` at 0xfd7c0 | load | 4 | `abs 0xfd9c8` | `-` |
| `ldr` at 0xfd7c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd7cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xfd7d8 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xfd7e0 | load | 4 | `abs 0xfd9cc` | `-` |
| `ldr` at 0xfd7e4 | load | 4 | `abs 0x1b1008` | `-` |
| `ldr` at 0xfd7f0 | load | 4 | `abs 0xfd9d0` | `-` |
| `ldr` at 0xfd7f4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xfd7f8 | load | 4 | `abs 0xfd9d4` | `-` |
| `ldr` at 0xfd7fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd804 | load | 4 | `abs 0xfd9d0` | `-` |
| `ldr` at 0xfd808 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xfd810 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xfd814 | store | 4 | `sp -0x28` | `[sp-44]` |
| `ldr` at 0xfd81c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xfd820 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xfd828 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xfd838 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xfd83c | load | 4 | `abs 0xfd9c4` | `-` |
| `ldr` at 0xfd840 | load | 4 | `abs 0x1b100c` | `-` |
| `ldr` at 0xfd858 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd864 | load | 4 | `abs 0xfd9c4` | `-` |
| `ldr` at 0xfd868 | load | 4 | `abs 0x1b100c` | `-` |
| `ldr` at 0xfd87c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xfd888 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xfd890 | load | 4 | `abs 0xfd9d8` | `-` |
| `ldr` at 0xfd8a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd8ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd8b0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xfd8b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd8b8 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xfd8bc | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xfd8c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfd8cc | load | 4 | `abs 0xfd9c4` | `-` |
| `ldr` at 0xfd8d0 | load | 4 | `abs 0x1b100c` | `-` |
| `ldr` at 0xfd8dc | load | 4 | `abs 0xfd9dc` | `-` |
| `ldr` at 0xfd8e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd8e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd8f0 | load | 4 | `abs 0xfd9e0` | `-` |
| `str` at 0xfd8f4 | store | 4 | `sp -0x60` | `0xfffffd9a` |
| `ldr` at 0xfd8fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfd900 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xfd904 | store | 4 | `[sp-48] +0x0` | `[sp-44]` |
| `ldr` at 0xfd908 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xfd90c | load | 2 | `[sp-44] +0x4` | `-` |
| `strh` at 0xfd910 | store | 2 | `sp -0x68` | `([[sp-44]+4]:2 & 0xffff)` |
| `ldrh` at 0xfd914 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xfd920 | store | 2 | `sp -0x68` | `?` |
| `ldrh` at 0xfd924 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xfd928 | store | 2 | `[sp-44] +0x4` | `([sp-104]:2 & 0xffff)` |
| `ldr` at 0xfd92c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xfd934 | store | 4 | `[sp-44] +0x0` | `0x0` |
| `ldr` at 0xfd938 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xfd93c | load | 1 | `[sp-44] +0x15` | `-` |
| `str` at 0xfd948 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xfd94c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xfd950 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0xfd954 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xfd964 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xfd968 | load | 2 | `[sp-40] +0x4` | `-` |
| `strh` at 0xfd96c | store | 2 | `sp -0x6c` | `([[sp-40]+4]:2 & 0xffff)` |
| `ldrh` at 0xfd970 | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xfd980 | store | 2 | `sp -0x6c` | `?` |
| `ldrh` at 0xfd984 | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xfd988 | store | 2 | `[sp-40] +0x4` | `([sp-108]:2 & 0xffff)` |
| `ldr` at 0xfd98c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfd990 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd994 | store | 4 | `[sp-32] +0x0` | `[sp-40]` |
| `str` at 0xfd9a4 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xfd9a8 | load | 4 | `sp -0x60` | `-` |
| `ldm` at 0xfd9b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfd9b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfd9b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfd5c8 | `(unknown callee) 0x201fc` | `0x410` | `?` | `?` | `0x400` |
| `bl` at 0xfd604 | `(unknown callee) 0x20514` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xfd624 | `(unknown callee) 0x20550` | `ret(0x201fc)` | `0x0` | `0x2c` | `ret(0x201fc)` |
| `mov` at 0xfd640 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-44` | `[sp-64]` | `sp-44` |
| `bl` at 0xfd6d0 | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |
| `bl` at 0xfd720 | `(unknown callee) 0xfd0dc` | `0x31baac` | `?` | `[sp-40]` | `0x0` |
| `bl` at 0xfd734 | `(unknown callee) 0x20514` | `[sp-44]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xfd75c | `(unknown callee) 0x20514` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xfd77c | `(unknown callee) 0xfdbb0` | `[sp-44]` | `[0x1b1004]` | `?` | `0x1b1004` |
| `bl` at 0xfd7bc | `(unknown callee) 0xfe66c` | `0x0` | `0x0` | `?` | `[0x1b100c]` |
| `bl` at 0xfd7c8 | `(unknown callee) 0xfe66c` | `0x188260` | `[sp-44]` | `?` | `?` |
| `bl` at 0xfd7d0 | `(unknown callee) 0x20514` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xfd800 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x188268` | `[sp-44]` | `0x1a288c` |
| `bl` at 0xfd80c | `(unknown callee) 0x200dc` | `[0x1a288c]` | `?` | `?` | `0x1a288c` |
| `bl` at 0xfd854 | `(unknown callee) 0xfe66c` | `0x0` | `0x0` | `?` | `[0x1b100c]` |
| `bl` at 0xfd898 | `(unknown callee) 0x205bc` | `sp-92` | `0x188274` | `?` | `?` |
| `bl` at 0xfd8a8 | `(unknown callee) 0xfe66c` | `sp-92` | `[sp-40]` | `?` | `sp-92` |
| `bl` at 0xfd8c4 | `(unknown callee) 0x20514` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xfd8e4 | `(unknown callee) 0xfe66c` | `0x18827c` | `[sp-44]` | `?` | `[0x1b100c]` |
| `bl` at 0xfd8ec | `(unknown callee) 0x20514` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xfd99c | `(unknown callee) 0xfaa44` | `0x1` | `([[sp-40]+4]:2 & 0xffff)` | `[sp-32]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xfd5dc | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0xfd5e8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bgt` at 0xfd654 | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xfd668 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfd674 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0xfd6a4 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xfd6bc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfd6f4 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xfd704 | `[[sp-40]+22]:1 != 0x0` | `[[sp-40]+22]:1 == 0x0` |
| `beq` at 0xfd72c | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0xfd740 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xfd788 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xfd7a0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xfd7b0 | `[0x1b100c] == 0x0` | `[0x1b100c] != 0x0` |
| `beq` at 0xfd7ec | `[0x1b1008] == 0x0` | `[0x1b1008] != 0x0` |
| `beq` at 0xfd830 | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0xfd848 | `[0x1b100c] == 0x0` | `[0x1b100c] != 0x0` |
| `beq` at 0xfd860 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xfd870 | `[0x1b100c] == 0x0` | `[0x1b100c] != 0x0` |
| `beq` at 0xfd8d8 | `[0x1b100c] == 0x0` | `[0x1b100c] != 0x0` |
| `beq` at 0xfd95c | `[sp-56] == 0x0` | `[sp-56] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfd9b0 | `r0` | `?` |

## `IRouterWritePacket`

- `IRouterWritePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfd9e4 to 0xfdbb0.
  Established. A call to an unknown target has candidate r0/r1/r2/r3 values `[sp-28]`, `[sp-52]+16`, `[sp-40]` and `[sp-52]+16` (`mov` at 0xfdae0).
  Equality `ret(unknown) == [sp-40]` takes `0xfdb70`, while inequality falls through (`beq` at 0xfdaf8).
  The unnamed `0x203f4` call has candidate r0/r1/r2/r3 values `ret(0x201fc)`, `[sp-36]`, `[sp-40]+16` and `[sp-40]+16` (`bl` at 0xfdab4).
  Packet serialization fields and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfd9f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfd9f8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfd9fc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfda00 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xfda04 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfda08 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xfda0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfda18 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xfda1c | load | 2 | `(unknown)` | `-` |
| `str` at 0xfda20 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xfda24 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xfda28 | load | 1 | `(unknown)` | `-` |
| `str` at 0xfda38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xfda3c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfda50 | store | 4 | `sp -0x34` | `ret(0x201fc)` |
| `ldr` at 0xfda54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xfda60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xfda64 | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xfda74 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xfda78 | load | 4 | `abs 0xfdba4` | `-` |
| `ldr` at 0xfda7c | load | 4 | `abs 0xfdba8` | `-` |
| `ldr` at 0xfda80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfda8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfda94 | load | 4 | `abs 0xfdba8` | `-` |
| `str` at 0xfda98 | store | 4 | `sp -0x38` | `0xfffffda4` |
| `ldr` at 0xfdaa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfdaa8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xfdaac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfdab8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xfdac4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xfdacc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfdad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfdad8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfdae8 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0xfdaec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfdaf0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfdafc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfdb08 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xfdb0c | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xfdb1c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xfdb20 | load | 4 | `abs 0xfdba4` | `-` |
| `ldr` at 0xfdb28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfdb34 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xfdb3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfdb44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfdb50 | load | 4 | `abs 0xfdbac` | `-` |
| `str` at 0xfdb54 | store | 4 | `sp -0x3c` | `0xfffffd92` |
| `str` at 0xfdb60 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xfdb64 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xfdb68 | store | 4 | `sp -0x38` | `[sp-60]` |
| `ldr` at 0xfdb70 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xfdb78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfdb7c | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xfdb80 | store | 4 | `sp -0x24` | `[[sp-36]]` |
| `ldr` at 0xfdb88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfdb94 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xfdb98 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xfdba0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfdba0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfdba0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfda48 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xfda88 | `(unknown callee) 0xfd0dc` | `0x31baac` | `0xfffffda4` | `[sp-32]` | `0x0` |
| `bl` at 0xfda90 | `(unknown callee) 0xfb794` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xfdab4 | `(unknown callee) 0x203f4` | `ret(0x201fc)` | `[sp-36]` | `[sp-40]+16` | `[sp-40]+16` |
| `bl` at 0xfdac0 | `(unknown callee) 0xfdbb0` | `[sp-52]` | `0x1` | `?` | `?` |
| `mov` at 0xfdae0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-52]+16` | `[sp-40]` | `[sp-52]+16` |
| `bl` at 0xfdb30 | `(unknown callee) 0xfd0dc` | `0x31baac` | `?` | `[sp-32]` | `0x0` |
| `bl` at 0xfdb38 | `(unknown callee) 0x20514` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0xfdb40 | `(unknown callee) 0xfb794` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xfdb74 | `(unknown callee) 0x20514` | `[sp-52]` | `?` | `ret(unknown)` | `[sp-40]` |
| `bl` at 0xfdb8c | `(unknown callee) 0xfb794` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfda14 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xfda5c | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0xfda6c | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `beq` at 0xfdaf8 | `ret(unknown) == [sp-40]` | `ret(unknown) != [sp-40]` |
| `bge` at 0xfdb04 | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `bne` at 0xfdb14 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `bne` at 0xfdb4c | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfdba0 | `r0` | `?` |

## `IRouterRequest`

- `IRouterRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfcf98 to 0xfd0dc.
  Established. A halfword read addresses entry `r0 +0x6`, and a word assignment writes `[r0+6]:2` to `[sp+12] +0x0` (`ldrh` at 0xfcfcc, `str` at 0xfcfd0).
  The unnamed `0xfac9c` call has candidate r0/r1/r2/r3 values `[sp-48]`, `0x0`, `0x0` and `[sp-32]` (`bl` at 0xfd084);
  its zero result takes `0xfd0a8`, while nonzero falls through (`beq` at 0xfd098).
  Request field meanings and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfcfa8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfcfac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfcfb0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfcfb4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xfcfb8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xfcfc4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xfcfc8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfcfcc | load | 2 | `r0 +0x6` | `-` |
| `str` at 0xfcfd0 | store | 4 | `[sp+12] +0x0` | `[r0+6]:2` |
| `str` at 0xfcfd8 | store | 4 | `sp -0x30` | `sp-40` |
| `ldr` at 0xfcfdc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xfcfe0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xfcfe4 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xfcfe8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xfcfec | store | 4 | `sp -0x58` | `[sp]` |
| `str` at 0xfcff4 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xfcff8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfcffc | load | 2 | `(unknown)` | `-` |
| `strh` at 0xfd004 | store | 2 | `(unknown)` | `?` |
| `str` at 0xfd010 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xfd014 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfd018 | store | 4 | `sp -0x4c` | `[sp-24]` |
| `ldr` at 0xfd01c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfd028 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfd02c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xfd038 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfd03c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xfd044 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfd048 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0xfd04c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xfd050 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xfd054 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xfd058 | store | 4 | `sp -0x44` | `[sp+4]` |
| `ldr` at 0xfd05c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xfd060 | store | 4 | `sp -0x40` | `[sp+8]` |
| `ldr` at 0xfd064 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xfd068 | store | 4 | `sp -0x3c` | `[sp+16]` |
| `str` at 0xfd070 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xfd074 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfd080 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfd08c | store | 4 | `sp -0x24` | `ret(0xfac9c)` |
| `ldr` at 0xfd090 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfd09c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd0a0 | store | 4 | `sp -0x2c` | `ret(0xfac9c)` |
| `ldr` at 0xfd0a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfd0ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd0bc | store | 4 | `sp -0x24` | `ret(0xfbf94)` |
| `ldr` at 0xfd0c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd0c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd0cc | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xfd0d0 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xfd0d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfd0d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfd0d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfd084 | `(unknown callee) 0xfac9c` | `[sp-48]` | `0x0` | `0x0` | `[sp-32]` |
| `bl` at 0xfd0b4 | `(unknown callee) 0xfbf94` | `[sp-20]` | `[sp-40]` | `0x1` | `ret(0xfac9c)` |
| `bl` at 0xfd0c4 | `(unknown callee) 0xfb794` | `[sp-40]` | `?` | `?` | `ret(0xfbf94)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfcfc0 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `beq` at 0xfd024 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0xfd034 | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |
| `beq` at 0xfd098 | `ret(0xfac9c) == 0x0` | `ret(0xfac9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfd0d8 | `r0` | `?` |

## `IRouterResponse`

- `IRouterResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfd0dc to 0xfd284.
  Established. A byte read addresses entry `r2 +0x17`, and a halfword read addresses entry `r2 +0x22` (`ldrb` at 0xfd108, `ldrh` at 0xfd15c).
  The unnamed `0xfac9c` call has candidate r0/r1/r2/r3 values `[sp-48]`, `0x1`, `0x0` and `[sp-52]` (`bl` at 0xfd228);
  its zero result takes `0xfd24c`, while nonzero falls through (`beq` at 0xfd23c).
  Response field meanings and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfd0ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfd0f0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfd0f4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfd0f8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xfd100 | store | 4 | `sp -0x30` | `sp-36` |
| `ldr` at 0xfd104 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xfd108 | load | 1 | `r2 +0x17` | `-` |
| `str` at 0xfd118 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xfd11c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfd128 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xfd12c | load | 1 | `r0 +0x4` | `-` |
| `str` at 0xfd130 | store | 4 | `sp -0x38` | `[r0+4]:1` |
| `ldr` at 0xfd138 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xfd13c | load | 1 | `r2 +0x17` | `-` |
| `str` at 0xfd140 | store | 4 | `sp -0x38` | `[r2+23]:1` |
| `ldr` at 0xfd144 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xfd14c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xfd150 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xfd154 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xfd158 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xfd15c | load | 2 | `r2 +0x22` | `-` |
| `str` at 0xfd160 | store | 4 | `sp -0x60` | `[r2+34]:2` |
| `ldr` at 0xfd164 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfd170 | load | 4 | `abs 0xfd280` | `-` |
| `str` at 0xfd1a0 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xfd1a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xfd1a8 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xfd1ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xfd1b0 | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xfd1bc | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xfd1c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfd1c8 | store | 4 | `sp -0x54` | `[sp-28]+28` |
| `ldr` at 0xfd1cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfd1d0 | store | 4 | `sp -0x3c` | `[sp-20]` |
| `ldr` at 0xfd1d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfd1e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfd1e8 | store | 4 | `sp -0x3c` | `[sp-28]+24` |
| `ldr` at 0xfd1ec | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xfd1f0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xfd1f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfd1f8 | store | 4 | `sp -0x4c` | `[sp-32]` |
| `ldr` at 0xfd1fc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xfd200 | store | 4 | `sp -0x48` | `[sp]` |
| `ldr` at 0xfd204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfd208 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xfd20c | store | 4 | `sp -0x44` | `[[sp-28]+12]` |
| `str` at 0xfd214 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xfd218 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfd224 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xfd230 | store | 4 | `sp -0x28` | `ret(0xfac9c)` |
| `ldr` at 0xfd234 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd240 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd244 | store | 4 | `sp -0x2c` | `ret(0xfac9c)` |
| `ldr` at 0xfd24c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfd250 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd260 | store | 4 | `sp -0x28` | `ret(0xfbf94)` |
| `ldr` at 0xfd264 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfd26c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd270 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xfd274 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xfd27c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfd27c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfd27c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfd228 | `(unknown callee) 0xfac9c` | `[sp-48]` | `0x1` | `0x0` | `[sp-52]` |
| `bl` at 0xfd258 | `(unknown callee) 0xfbf94` | `[sp-20]` | `[sp-36]` | `0x1` | `ret(0xfac9c)` |
| `bl` at 0xfd268 | `(unknown callee) 0xfb794` | `[sp-36]` | `?` | `?` | `ret(0xfbf94)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfd124 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xfd1dc | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0xfd23c | `ret(0xfac9c) == 0x0` | `ret(0xfac9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfd27c | `r0` | `?` |

## `IRouterEvent`

- `IRouterEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfd284 to 0xfd370.
  Established. A halfword assignment writes `([r0+6]:2+1 & 0xffff)` to entry `r0 +0x6` (`strh` at 0xfd2d0).
  The unnamed `0xfac9c` call has candidate r0/r1/r2/r3 values `sp-36`, `0x2`, `0x0` and `[sp-24]` (`bl` at 0xfd31c).
  The unnamed `0xfc678` call has candidate r0/r1/r2/r3 values `[sp-20]`, `[sp-36]`, unknown and `ret(0xfac9c)` (`bl` at 0xfd348).
  Event field meanings and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfd294 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfd298 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfd29c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfd2a0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xfd2a8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xfd2ac | load | 1 | `r0 +0x4` | `-` |
| `str` at 0xfd2b0 | store | 4 | `sp -0x54` | `[r0+4]:1` |
| `ldr` at 0xfd2b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfd2b8 | store | 4 | `sp -0x50` | `r2` |
| `str` at 0xfd2c0 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xfd2c4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfd2c8 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xfd2d0 | store | 2 | `r0 +0x6` | `([r0+6]:2+1 & 0xffff)` |
| `str` at 0xfd2dc | store | 4 | `sp -0x48` | `?` |
| `str` at 0xfd2e4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xfd2e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfd2ec | store | 4 | `sp -0x40` | `[sp-20]` |
| `ldr` at 0xfd2f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfd2f4 | store | 4 | `sp -0x3c` | `[sp-32]` |
| `ldr` at 0xfd2f8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xfd2fc | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0xfd304 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xfd308 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xfd30c | store | 4 | `sp -0x30` | `[sp+4]` |
| `ldr` at 0xfd318 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfd324 | store | 4 | `sp -0x28` | `ret(0xfac9c)` |
| `ldr` at 0xfd328 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd334 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd338 | store | 4 | `sp -0x2c` | `ret(0xfac9c)` |
| `ldr` at 0xfd340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfd344 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd350 | store | 4 | `sp -0x28` | `ret(0xfc678)` |
| `ldr` at 0xfd354 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfd35c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfd360 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xfd364 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xfd36c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfd36c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfd36c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfd31c | `(unknown callee) 0xfac9c` | `sp-36` | `0x2` | `0x0` | `[sp-24]` |
| `bl` at 0xfd348 | `(unknown callee) 0xfc678` | `[sp-20]` | `[sp-36]` | `?` | `ret(0xfac9c)` |
| `bl` at 0xfd358 | `(unknown callee) 0xfb794` | `[sp-36]` | `?` | `?` | `ret(0xfc678)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfd330 | `ret(0xfac9c) == 0x0` | `ret(0xfac9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfd36c | `r0` | `?` |

## `IRouterCommand`

- `IRouterCommand`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfd370 to 0xfd4a8.
  Established. Byte reads address `[[sp+8]]` offsets `+0x14`, `+0x21` and `+0x12` (`ldrb` at 0xfd424, `ldrb` at 0xfd430, `ldrb` at 0xfd448).
  The unnamed `0xfb734` call has candidate r0/r1/r2/r3 values `[sp+8]`, `[sp+12]`, `[sp+16]` and `ret(0xfcf98)` (`bl` at 0xfd3f8).
  A word assignment places `0xfffffd96` at `sp -0x30` (`str` at 0xfd46c).
  Command correlation fields and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfd380 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfd384 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfd388 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfd38c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xfd390 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfd394 | store | 4 | `sp -0x44` | `r3` |
| `ldr` at 0xfd398 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xfd39c | store | 4 | `sp -0x40` | `[sp]` |
| `ldr` at 0xfd3a0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xfd3a4 | store | 4 | `sp -0x3c` | `[sp+4]` |
| `str` at 0xfd3ac | store | 4 | `sp -0x38` | `sp-40` |
| `str` at 0xfd3b4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xfd3b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfd3bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfd3c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfd3d0 | store | 4 | `sp -0x24` | `ret(0xfcf98)` |
| `ldr` at 0xfd3d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfd3e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd3e4 | store | 4 | `sp -0x30` | `ret(0xfcf98)` |
| `ldr` at 0xfd3ec | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xfd3f0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xfd3f4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xfd400 | store | 4 | `sp -0x24` | `ret(0xfb734)` |
| `ldr` at 0xfd404 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfd410 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfd414 | store | 4 | `sp -0x30` | `ret(0xfb734)` |
| `ldr` at 0xfd41c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xfd420 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xfd424 | load | 1 | `[[sp+8]] +0x14` | `-` |
| `ldr` at 0xfd428 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xfd42c | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xfd430 | load | 1 | `[[sp+8]] +0x21` | `-` |
| `str` at 0xfd43c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xfd440 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xfd444 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xfd448 | load | 1 | `[[sp+8]] +0x12` | `-` |
| `ldr` at 0xfd454 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd45c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd468 | load | 4 | `abs 0xfd4a4` | `-` |
| `str` at 0xfd46c | store | 4 | `sp -0x30` | `0xfffffd96` |
| `ldr` at 0xfd474 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd478 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfd484 | load | 4 | `abs 0xfd4a4` | `-` |
| `str` at 0xfd488 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `str` at 0xfd494 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xfd498 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xfd4a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfd4a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfd4a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfd3c8 | `(unknown callee) 0xfcf98` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xfd3f8 | `(unknown callee) 0xfb734` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `ret(0xfcf98)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfd3dc | `ret(0xfcf98) == 0x0` | `ret(0xfcf98) != 0x0` |
| `beq` at 0xfd40c | `ret(0xfb734) == 0x0` | `ret(0xfb734) != 0x0` |
| `bne` at 0xfd450 | `[[[sp+8]]+18]:1 != 0x0` | `[[[sp+8]]+18]:1 == 0x0` |
| `beq` at 0xfd464 | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0xfd480 | `[sp-40] == [sp-44]` | `[sp-40] != [sp-44]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfd4a0 | `r0` | `?` |

## `IRouterMakePacket`

- `IRouterMakePacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfd4a8 to 0xfd578.
  Established. A halfword assignment writes `([r0+6]:2+1 & 0xffff)` to entry `r0 +0x6` (`strh` at 0xfd4fc).
  The unnamed `0xfac9c` call has candidate r0/r1/r2/r3 values `sp-36`, `[sp]`, `[sp-40]` and `[sp-28]` (`bl` at 0xfd548).
  One word assignment clears `sp -0x24` (`str` at 0xfd55c).
  Packet construction fields and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfd4b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfd4bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfd4c0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfd4c4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xfd4cc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xfd4d4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xfd4d8 | load | 1 | `r0 +0x4` | `-` |
| `str` at 0xfd4dc | store | 4 | `sp -0x50` | `[r0+4]:1` |
| `ldr` at 0xfd4e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfd4e4 | store | 4 | `sp -0x4c` | `r3` |
| `str` at 0xfd4ec | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xfd4f0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfd4f4 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xfd4fc | store | 2 | `r0 +0x6` | `([r0+6]:2+1 & 0xffff)` |
| `str` at 0xfd508 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xfd50c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfd510 | store | 4 | `sp -0x40` | `[sp-24]` |
| `ldr` at 0xfd514 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfd518 | store | 4 | `sp -0x3c` | `[sp-20]` |
| `ldr` at 0xfd51c | load | 4 | `sp +0x8` | `-` |
| `str` at 0xfd520 | store | 4 | `sp -0x38` | `[sp+8]` |
| `ldr` at 0xfd524 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xfd528 | store | 4 | `sp -0x34` | `[sp+12]` |
| `str` at 0xfd530 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xfd538 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xfd53c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xfd540 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfd544 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfd55c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xfd564 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xfd568 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xfd574 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfd574 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfd574 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfd548 | `(unknown callee) 0xfac9c` | `sp-36` | `[sp]` | `[sp-40]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfd554 | `ret(0xfac9c) == 0x0` | `ret(0xfac9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfd574 | `r0` | `[sp-36]` |

## `IRouterPacketUseCount`

- `IRouterPacketUseCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf9a70 to 0xf9be0.
  Established. A halfword assignment writes `([sp-32]:2 & 0xffff)` to `[sp-20] +0x4` (`strh` at 0xf9b20).
  An included word read addresses `0x1b1020` (`ldr` at 0xf9b28).
  The unnamed `0x2302c` call has candidate r0/r1/r2/r3 values `0x1b107c`, unknown, unknown and unknown (`bl` at 0xf9b74).
  Packet count encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf9a80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf9a84 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xf9a88 | load | 4 | `abs 0xf9bc0` | `-` |
| `str` at 0xf9a98 | store | 4 | `sp -0x1c` | `ret(0x22ca0)` |
| `ldr` at 0xf9a9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf9aa8 | load | 4 | `abs 0xf9bc4` | `-` |
| `str` at 0xf9aac | store | 4 | `sp -0x2c` | `0x188068` |
| `ldr` at 0xf9ab0 | load | 4 | `abs 0xf9bc0` | `-` |
| `str` at 0xf9ab4 | store | 4 | `sp -0x28` | `0x1b107c` |
| `ldr` at 0xf9ab8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf9abc | store | 4 | `sp -0x24` | `ret(0x22ca0)` |
| `ldr` at 0xf9ac4 | load | 4 | `abs 0xf9bc8` | `-` |
| `ldr` at 0xf9ad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf9ad8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xf9adc | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xf9ae4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xf9af4 | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xf9af8 | store | 2 | `sp -0x20` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xf9afc | load | 4 | `abs 0xf9bcc` | `-` |
| `ldrh` at 0xf9b08 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xf9b18 | store | 2 | `sp -0x20` | `?` |
| `ldrh` at 0xf9b1c | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xf9b20 | store | 2 | `[sp-20] +0x4` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xf9b24 | load | 4 | `abs 0xf9bd0` | `-` |
| `ldr` at 0xf9b28 | load | 4 | `abs 0x1b1020` | `-` |
| `ldr` at 0xf9b34 | load | 4 | `abs 0xf9bd0` | `-` |
| `ldr` at 0xf9b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf9b3c | load | 4 | `abs 0x1b1020` | `-` |
| `ldr` at 0xf9b48 | load | 4 | `abs 0xf9bd4` | `-` |
| `ldr` at 0xf9b4c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xf9b50 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xf9b60 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xf9b64 | load | 4 | `abs 0xf9bd8` | `-` |
| `ldr` at 0xf9b68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf9b70 | load | 4 | `abs 0xf9bc0` | `-` |
| `str` at 0xf9b7c | store | 4 | `sp -0x1c` | `ret(0x2302c)` |
| `ldr` at 0xf9b80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf9b8c | load | 4 | `abs 0xf9bdc` | `-` |
| `str` at 0xf9b90 | store | 4 | `sp -0x2c` | `0x1880c0` |
| `ldr` at 0xf9b94 | load | 4 | `abs 0xf9bc0` | `-` |
| `str` at 0xf9b98 | store | 4 | `sp -0x28` | `0x1b107c` |
| `ldr` at 0xf9b9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf9ba0 | store | 4 | `sp -0x24` | `ret(0x2302c)` |
| `ldr` at 0xf9ba8 | load | 4 | `abs 0xf9bc8` | `-` |
| `ldm` at 0xf9bbc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf9bbc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf9bbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf9a90 | `(unknown callee) 0x22ca0` | `0x1b107c` | `?` | `r2` | `r3` |
| `bl` at 0xf9ad0 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x5b` | `0x0` |
| `bl` at 0xf9b6c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x1880b4` | `[sp-20]` | `?` |
| `bl` at 0xf9b74 | `(unknown callee) 0x2302c` | `0x1b107c` | `?` | `?` | `?` |
| `bl` at 0xf9bb4 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x63` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf9aa4 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0xf9b30 | `[0x1b1020] == 0x0` | `[0x1b1020] != 0x0` |
| `bne` at 0xf9b44 | `[sp-20] != [0x1b1020]` | `[sp-20] == [0x1b1020]` |
| `beq` at 0xf9b88 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf9bbc | `r0` | `?` |
