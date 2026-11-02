# ARM port state and status entry paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
A computed transfer in portStatus leaves unresolved edges; its tables cover known edges only. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/port-state-facts.txt) retain all tool sections.

## `portInit`

- `portInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e5ec to 0x3e808.
  Established. A word at `0x119df4` receives an unknown value (`str` at 0x3e650), while the word at
  `0x119df8` receives `[sp-28]` (`str` at 0x3e65c). The result of `0x18920` is stored at `0x119df0`
  (`str` at 0x3e690). The following branch is taken when `[0x119df0] != 0`, with zero falling
  through (`bne` at 0x3e69c). Subsequent byte stores include two (`strb` at 0x3e794), zero (`strb`
  at 0x3e7b4) and zero (`strb` at 0x3e7d0), each at an unknown effective target. Port identity and
  hardware initialization are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3e5fc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3e600 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x3e604 | load | 4 | `abs 0x3e7e8` | `-` |
| `ldr` at 0x3e608 | load | 4 | `abs 0x119df4` | `-` |
| `ldr` at 0x3e614 | load | 4 | `abs 0x3e7ec` | `-` |
| `str` at 0x3e618 | store | 4 | `sp -0x2c` | `0xe1d78` |
| `ldr` at 0x3e61c | load | 4 | `abs 0x3e7e8` | `-` |
| `ldr` at 0x3e620 | load | 4 | `abs 0x119df4` | `-` |
| `str` at 0x3e624 | store | 4 | `sp -0x28` | `[0x119df4]` |
| `ldr` at 0x3e628 | load | 4 | `abs 0x3e7f0` | `-` |
| `ldr` at 0x3e62c | load | 4 | `abs 0x119df8` | `-` |
| `str` at 0x3e630 | store | 4 | `sp -0x24` | `[0x119df8]` |
| `ldr` at 0x3e638 | load | 4 | `abs 0x3e7f4` | `-` |
| `ldr` at 0x3e648 | load | 4 | `abs 0x3e7e8` | `-` |
| `ldr` at 0x3e64c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3e650 | store | 4 | `abs 0x119df4` | `?` |
| `ldr` at 0x3e654 | load | 4 | `abs 0x3e7f0` | `-` |
| `ldr` at 0x3e658 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3e65c | store | 4 | `abs 0x119df8` | `[sp-28]` |
| `ldr` at 0x3e660 | load | 4 | `abs 0x3e7f8` | `-` |
| `ldr` at 0x3e664 | load | 4 | `abs 0x3e7fc` | `-` |
| `ldr` at 0x3e674 | load | 4 | `abs 0x3e800` | `-` |
| `ldr` at 0x3e678 | load | 4 | `abs 0x3e7f0` | `-` |
| `ldr` at 0x3e67c | load | 4 | `abs 0x119df8` | `-` |
| `str` at 0x3e690 | store | 4 | `abs 0x119df0` | `ret(0x18920)` |
| `ldr` at 0x3e694 | load | 4 | `abs 0x119df0` | `-` |
| `ldr` at 0x3e6a0 | load | 4 | `abs 0x3e804` | `-` |
| `str` at 0x3e6a4 | store | 4 | `sp -0x2c` | `0xe1dc4` |
| `ldr` at 0x3e6ac | load | 4 | `abs 0x3e7f4` | `-` |
| `strb` at 0x3e6c0 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x3e6c4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3e6c8 | load | 4 | `abs 0x3e7f0` | `-` |
| `ldr` at 0x3e6cc | load | 4 | `abs 0x119df8` | `-` |
| `ldrb` at 0x3e6dc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3e6f0 | load | 4 | `abs 0x3e7e8` | `-` |
| `ldr` at 0x3e6f4 | load | 4 | `abs 0x119df4` | `-` |
| `ldrb` at 0x3e6fc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3e714 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3e728 | load | 4 | `abs 0x3e7e8` | `-` |
| `ldr` at 0x3e72c | load | 4 | `abs 0x119df4` | `-` |
| `ldrb` at 0x3e734 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3e740 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3e754 | load | 4 | `abs 0x3e7e8` | `-` |
| `ldr` at 0x3e758 | load | 4 | `abs 0x119df4` | `-` |
| `ldrb` at 0x3e760 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3e77c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3e784 | load | 4 | `abs 0x3e800` | `-` |
| `ldr` at 0x3e788 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3e794 | store | 1 | `(unknown)` | `0x2` |
| `ldrb` at 0x3e79c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3e7a4 | load | 4 | `abs 0x3e800` | `-` |
| `ldr` at 0x3e7a8 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3e7b4 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x3e7b8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3e7c0 | load | 4 | `abs 0x3e800` | `-` |
| `ldr` at 0x3e7c4 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3e7d0 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x3e7d4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x3e7dc | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldmdb` at 0x3e7e4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3e7e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e7e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e7e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e644 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0x64` | `0x0` |
| `bl` at 0x3e670 | `(unknown callee) 0x1f9e8` | `0x119dfc` | `0xe1db8` | `0x1` | `0x9` |
| `bl` at 0x3e688 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x3e6b8 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0x78` | `0x0` |
| `bl` at 0x3e704 | `(unknown callee) 0xb78b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x3e768 | `(unknown callee) 0xb78b4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3e610 | `[0x119df4] == 0x0` | `[0x119df4] != 0x0` |
| `bne` at 0x3e69c | `[0x119df0] != 0x0` | `[0x119df0] == 0x0` |
| `blt` at 0x3e6d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3e710 | `ret(0xb78b4) == 0x0` | `ret(0xb78b4) != 0x0` |
| `beq` at 0x3e73c | `? == 0xff` | `? != 0xff` |
| `bne` at 0x3e774 | `ret(0xb78b4) != 0x0` | `ret(0xb78b4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e7e4 | `r0` | `?` |

## `portEnable`

- `portEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e808 to 0x3e8a8.
  Established. The first comparison takes its branch when `(r0 & 0xff) < [0x119df8]` signed, and
  falls through otherwise (`blt` at 0x3e830). A later equality branch skips the final byte store
  when an unknown value is zero (`beq` at 0x3e874); its nonzero fall-through writes two to an
  unknown effective target (`strb` at 0x3e890). Whether that byte enables a physical connection
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e81c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3e820 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e824 | load | 4 | `abs 0x3e898` | `-` |
| `ldr` at 0x3e828 | load | 4 | `abs 0x119df8` | `-` |
| `ldr` at 0x3e834 | load | 4 | `abs 0x3e89c` | `-` |
| `str` at 0x3e838 | store | 4 | `sp -0x1c` | `0xe1df8` |
| `ldrb` at 0x3e83c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3e840 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x3e848 | load | 4 | `abs 0x3e8a0` | `-` |
| `ldrb` at 0x3e858 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e860 | load | 4 | `abs 0x3e8a4` | `-` |
| `ldr` at 0x3e864 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3e86c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3e878 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e880 | load | 4 | `abs 0x3e8a4` | `-` |
| `ldr` at 0x3e884 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3e890 | store | 1 | `(unknown)` | `0x2` |
| `ldmdb` at 0x3e894 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e894 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e894 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e854 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0x8d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3e830 | `(r0 & 0xff) < [0x119df8] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119df8] (signed, 32-bit)` |
| `beq` at 0x3e874 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e894 | `r0` | `?` |

## `portDisable`

- `portDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e8a8 to 0x3e948.
  Established. Entry `r0` is saved as the byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x3e8bc). The
  signed less-than branch compares that truncated value with `[0x119df8]` (`blt` at 0x3e8d0).
  Another branch is taken for an unknown zero value and falls through otherwise (`beq` at 0x3e914).
  The fall-through byte store contains one at an unknown target (`strb` at 0x3e930). Disable
  semantics cannot be established from that value alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e8bc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3e8c0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e8c4 | load | 4 | `abs 0x3e938` | `-` |
| `ldr` at 0x3e8c8 | load | 4 | `abs 0x119df8` | `-` |
| `ldr` at 0x3e8d4 | load | 4 | `abs 0x3e93c` | `-` |
| `str` at 0x3e8d8 | store | 4 | `sp -0x1c` | `0xe1e24` |
| `ldrb` at 0x3e8dc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3e8e0 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x3e8e8 | load | 4 | `abs 0x3e940` | `-` |
| `ldrb` at 0x3e8f8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e900 | load | 4 | `abs 0x3e944` | `-` |
| `ldr` at 0x3e904 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3e90c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3e918 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e920 | load | 4 | `abs 0x3e944` | `-` |
| `ldr` at 0x3e924 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3e930 | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x3e934 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e934 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e934 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e8f4 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0x96` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3e8d0 | `(r0 & 0xff) < [0x119df8] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119df8] (signed, 32-bit)` |
| `beq` at 0x3e914 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e934 | `r0` | `?` |

## `portIsEnabled`

- `portIsEnabled`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e948 to 0x3e9d4.
  Established. The entry byte at `sp -0x11` contains `(r0 & 0xff)` (`strb` at 0x3e95c). The
  comparison takes its branch when that value is signed less than `[0x119df8]` (`blt` at 0x3e970).
  The call to `0x1eed0` has candidates `(0x9, 0xe1da4, 0x9f, 0x0)` (`bl` at 0x3e994). The return
  value is unknown (`ldmdb` at 0x3e9c0), so an enabled-state result is not established by these
  facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e95c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3e960 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e964 | load | 4 | `abs 0x3e9c4` | `-` |
| `ldr` at 0x3e968 | load | 4 | `abs 0x119df8` | `-` |
| `ldr` at 0x3e974 | load | 4 | `abs 0x3e9c8` | `-` |
| `str` at 0x3e978 | store | 4 | `sp -0x1c` | `0xe1e54` |
| `ldrb` at 0x3e97c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3e980 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x3e988 | load | 4 | `abs 0x3e9cc` | `-` |
| `ldrb` at 0x3e998 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e9a0 | load | 4 | `abs 0x3e9d0` | `-` |
| `ldr` at 0x3e9a4 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3e9ac | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x3e9c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e9c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e9c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e994 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0x9f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3e970 | `(r0 & 0xff) < [0x119df8] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119df8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e9c0 | `r0` | `?` |

## `portPresent`

- `portPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e9d4 to 0x3ea54.
  Established. The signed less-than comparison takes its branch for `(r0 & 0xff) < [0x119df8]`, and
  falls through for greater or equal (`blt` at 0x3e9fc). The later byte store writes two at an
  unknown effective target (`strb` at 0x3ea3c). A call to `0x1eed0` carries third candidate `0xab`
  (`bl` at 0x3ea20). Presence detection and the stored-byte meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e9e8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3e9ec | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3e9f0 | load | 4 | `abs 0x3ea44` | `-` |
| `ldr` at 0x3e9f4 | load | 4 | `abs 0x119df8` | `-` |
| `ldr` at 0x3ea00 | load | 4 | `abs 0x3ea48` | `-` |
| `str` at 0x3ea04 | store | 4 | `sp -0x1c` | `0xe1e84` |
| `ldrb` at 0x3ea08 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3ea0c | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x3ea14 | load | 4 | `abs 0x3ea4c` | `-` |
| `ldrb` at 0x3ea24 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3ea2c | load | 4 | `abs 0x3ea50` | `-` |
| `ldr` at 0x3ea30 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3ea3c | store | 1 | `(unknown)` | `0x2` |
| `ldmdb` at 0x3ea40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ea40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ea40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ea20 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0xab` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3e9fc | `(r0 & 0xff) < [0x119df8] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119df8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ea40 | `r0` | `?` |

## `portNotPresent`

- `portNotPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ea54 to 0x3ead4.
  Established. The first byte save truncates entry `r0` at `sp -0x11` (`strb` at 0x3ea68). The
  signed less-than branch skips the following call when `(r0 & 0xff) < [0x119df8]` (`blt` at
  0x3ea7c). The fall-through calls `0x1eed0` with third candidate `0xb3` (`bl` at 0x3eaa0). The
  final explicit byte update clears an unknown effective target (`strb` at 0x3eabc); its
  relationship to physical absence is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3ea68 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3ea6c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3ea70 | load | 4 | `abs 0x3eac4` | `-` |
| `ldr` at 0x3ea74 | load | 4 | `abs 0x119df8` | `-` |
| `ldr` at 0x3ea80 | load | 4 | `abs 0x3eac8` | `-` |
| `str` at 0x3ea84 | store | 4 | `sp -0x1c` | `0xe1eb4` |
| `ldrb` at 0x3ea88 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3ea8c | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x3ea94 | load | 4 | `abs 0x3eacc` | `-` |
| `ldrb` at 0x3eaa4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3eaac | load | 4 | `abs 0x3ead0` | `-` |
| `ldr` at 0x3eab0 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3eabc | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x3eac0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3eac0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3eac0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3eaa0 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0xb3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3ea7c | `(r0 & 0xff) < [0x119df8] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119df8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3eac0 | `r0` | `?` |

## `portIsPresent`

- `portIsPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ead4 to 0x3eb60.
  Established. The signed comparison branches for `(r0 & 0xff) < [0x119df8]` (`blt` at 0x3eafc). On
  its fall-through, `0x1eed0` receives candidates `(0x9, 0xe1da4, 0xbb, 0x0)` (`bl` at 0x3eb20). The
  exit leaves `r0` unknown (`ldmdb` at 0x3eb4c). A physical-present predicate cannot be assigned to
  that unknown value.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3eae8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x3eaec | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3eaf0 | load | 4 | `abs 0x3eb50` | `-` |
| `ldr` at 0x3eaf4 | load | 4 | `abs 0x119df8` | `-` |
| `ldr` at 0x3eb00 | load | 4 | `abs 0x3eb54` | `-` |
| `str` at 0x3eb04 | store | 4 | `sp -0x1c` | `0xe1ee4` |
| `ldrb` at 0x3eb08 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3eb0c | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x3eb14 | load | 4 | `abs 0x3eb58` | `-` |
| `ldrb` at 0x3eb24 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3eb2c | load | 4 | `abs 0x3eb5c` | `-` |
| `ldr` at 0x3eb30 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3eb38 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x3eb4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3eb4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3eb4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3eb20 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0xbb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3eafc | `(r0 & 0xff) < [0x119df8] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119df8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3eb4c | `r0` | `?` |

## `portReset`

- `portReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3eb60 to 0x3ecc4.
  Established. The loop begins with a byte clear at `sp -0x15` (`strb` at 0x3eb74), and its test
  retains an `lt` flags condition with unknown operands (`blt` at 0x3eb88). A call to `0x40ff0` has
  second candidate two (`bl` at 0x3ec20); its result differing from one takes the branch past the
  following byte write (`bne` at 0x3ec2c). Equality falls through to a store of three at an unknown
  target (`strb` at 0x3ec48). Another byte write also stores three at an unknown target (`strb` at
  0x3ec10). The reset signal and its timing remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3eb74 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x3eb78 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3eb7c | load | 4 | `abs 0x3ecb0` | `-` |
| `ldr` at 0x3eb80 | load | 4 | `abs 0x119df8` | `-` |
| `ldrb` at 0x3eb90 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3eb98 | load | 4 | `abs 0x3ecb4` | `-` |
| `ldr` at 0x3eb9c | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3eba4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3ebb0 | load | 4 | `abs 0x3ecb8` | `-` |
| `ldrb` at 0x3ebb4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3ebc8 | load | 4 | `abs 0x119df4` | `-` |
| `str` at 0x3ebd0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3ebd4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x3ebd8 | load | 2 | `(unknown)` | `-` |
| `ldrb` at 0x3ebf8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3ec00 | load | 4 | `abs 0x3ecb4` | `-` |
| `ldr` at 0x3ec04 | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3ec10 | store | 1 | `(unknown)` | `0x3` |
| `ldr` at 0x3ec18 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ec30 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3ec38 | load | 4 | `abs 0x3ecb4` | `-` |
| `ldr` at 0x3ec3c | load | 4 | `abs 0x119df0` | `-` |
| `strb` at 0x3ec48 | store | 1 | `(unknown)` | `0x3` |
| `ldr` at 0x3ec4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ec58 | store | 4 | `sp -0x14` | `ret(0x3fed0)` |
| `ldr` at 0x3ec5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ec68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ec74 | load | 4 | `abs 0x3ecbc` | `-` |
| `str` at 0x3ec78 | store | 4 | `sp -0x28` | `0xe1f14` |
| `ldrb` at 0x3ec7c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3ec80 | store | 4 | `sp -0x24` | `[sp-21]:1` |
| `str` at 0x3ec84 | store | 4 | `sp -0x20` | `ret(0x1ea50)` |
| `ldr` at 0x3ec8c | load | 4 | `abs 0x3ecc0` | `-` |
| `ldrb` at 0x3ec9c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x3eca4 | store | 1 | `sp -0x15` | `?` |
| `ldmdb` at 0x3ecac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ecac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ecac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ebe8 | `(unknown callee) 0x3a8c0` | `?` | `0x119df4` | `?` | `?` |
| `bl` at 0x3ec20 | `(unknown callee) 0x40ff0` | `?` | `0x2` | `?` | `?` |
| `bl` at 0x3ec50 | `(unknown callee) 0x3fed0` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x3ec6c | `(unknown callee) 0x1ea50` | `ret(0x3fed0)` | `?` | `?` | `ret(0x3fed0)` |
| `bl` at 0x3ec98 | `(unknown callee) 0x1eed0` | `0x3` | `0xe1da4` | `0xe6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3eb88 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x3ebac | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x3ebe4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3ebf4 | `ret(0x3a8c0) == 0x0` | `ret(0x3a8c0) != 0x0` |
| `bne` at 0x3ec2c | `ret(0x40ff0) != 0x1` | `ret(0x40ff0) == 0x1` |
| `beq` at 0x3ec64 | `ret(0x3fed0) == 0x0` | `ret(0x3fed0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ecac | `r0` | `?` |

## `portGetCfg`

- `portGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ecc4 to 0x3ed00.
  Established. The word `[0x119df4]` is stored at entry `r0 +0x0` (`str` at 0x3ece4). The exit
  returns `[0x119df8]` in `r0` (`ldmdb` at 0x3ecf4). No call or conditional branch is recorded. The
  configuration structure and count meaning are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ecd4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3ecd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ecdc | load | 4 | `abs 0x3ecf8` | `-` |
| `ldr` at 0x3ece0 | load | 4 | `abs 0x119df4` | `-` |
| `str` at 0x3ece4 | store | 4 | `r0 +0x0` | `[0x119df4]` |
| `ldr` at 0x3ece8 | load | 4 | `abs 0x3ecfc` | `-` |
| `ldr` at 0x3ecec | load | 4 | `abs 0x119df8` | `-` |
| `ldmdb` at 0x3ecf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ecf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ecf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ecf4 | `r0` | `[0x119df8]` |

## `portUpInternal`

- `portUpInternal`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ed00 to 0x3efcc.
  Established. The first branch is taken when `(r0 & 0xff) != 0xff`, and falls through on equality
  (`bne` at 0x3ed34). A zero result from `0x40650` falls through (`bne` at 0x3ed64) to a call to
  `0x406c0` (`bl` at 0x3ed68). Later calls to `0x40898` (`bl` at 0x3ef1c) and `0x3fed0` (`bl` at
  0x3ef6c) each have unknown first candidates and second candidate `0x119df4`. The loop test retains
  an `lt` flags condition with unknown operands (`blt` at 0x3edf8). Link activation and selector
  roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3ed18 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x3ed20 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x3ed28 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x3ed2c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3ed3c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3ed40 | load | 4 | `abs 0x3efb8` | `-` |
| `ldr` at 0x3ed44 | load | 4 | `abs 0x119df8` | `-` |
| `str` at 0x3ed48 | store | 4 | `sp -0x2c` | `[0x119df8]` |
| `ldrb` at 0x3ed50 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3ed70 | store | 4 | `sp -0x30` | `ret(0x406c0)` |
| `ldrb` at 0x3ed78 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3ed80 | load | 4 | `abs 0x3efbc` | `-` |
| `ldr` at 0x3ed84 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3ed8c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3ed98 | load | 4 | `abs 0x3efc0` | `-` |
| `str` at 0x3ed9c | store | 4 | `sp -0x30` | `0xfffffb4e` |
| `ldrb` at 0x3eda4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3edac | load | 4 | `abs 0x3efbc` | `-` |
| `ldr` at 0x3edb0 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3edb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3edc4 | load | 4 | `abs 0x3efc4` | `-` |
| `str` at 0x3edc8 | store | 4 | `sp -0x30` | `0xfffffb4d` |
| `ldrb` at 0x3edd0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3edd4 | store | 4 | `sp -0x28` | `[sp-17]:1` |
| `ldrb` at 0x3edd8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3ede0 | store | 4 | `sp -0x2c` | `[sp-17]:1+1` |
| `ldr` at 0x3ede4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3ede8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3edec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3edf0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3ee00 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x3ee0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ee20 | load | 4 | `abs 0x3efc8` | `-` |
| `ldr` at 0x3ee24 | load | 4 | `abs 0x119df4` | `-` |
| `ldrh` at 0x3ee2c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3ee3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ee50 | load | 4 | `abs 0x3efc8` | `-` |
| `ldr` at 0x3ee54 | load | 4 | `abs 0x119df4` | `-` |
| `ldr` at 0x3ee5c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x3ee6c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x3ee78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ee8c | load | 4 | `abs 0x3efc8` | `-` |
| `ldr` at 0x3ee90 | load | 4 | `abs 0x119df4` | `-` |
| `ldrh` at 0x3ee98 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3eea8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3eebc | load | 4 | `abs 0x3efc8` | `-` |
| `ldr` at 0x3eec0 | load | 4 | `abs 0x119df4` | `-` |
| `ldr` at 0x3eec8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3eed8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3eee0 | load | 4 | `abs 0x3efbc` | `-` |
| `ldr` at 0x3eee4 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3eeec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3eef8 | load | 4 | `abs 0x3efc8` | `-` |
| `ldr` at 0x3eefc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ef10 | load | 4 | `abs 0x119df4` | `-` |
| `str` at 0x3ef24 | store | 4 | `sp -0x20` | `ret(0x40898)` |
| `ldr` at 0x3ef28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ef34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ef40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ef44 | store | 4 | `sp -0x1c` | `ret(0x40898)` |
| `ldr` at 0x3ef48 | load | 4 | `abs 0x3efc8` | `-` |
| `ldr` at 0x3ef4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ef60 | load | 4 | `abs 0x119df4` | `-` |
| `str` at 0x3ef74 | store | 4 | `sp -0x20` | `ret(0x3fed0)` |
| `ldr` at 0x3ef78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ef84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ef90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ef94 | store | 4 | `sp -0x1c` | `ret(0x3fed0)` |
| `ldr` at 0x3ef98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3efa0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3efa8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3efac | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3efb0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x3efb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3efb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3efb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ed58 | `(unknown callee) 0x40650` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0x3ed68 | `(unknown callee) 0x406c0` | `ret(0x40650)` | `?` | `?` | `ret(0x40650)` |
| `bl` at 0x3ef1c | `(unknown callee) 0x40898` | `?` | `0x119df4` | `?` | `?` |
| `bl` at 0x3ef6c | `(unknown callee) 0x3fed0` | `?` | `0x119df4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3ed34 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `bne` at 0x3ed64 | `ret(0x40650) != 0x0` | `ret(0x40650) == 0x0` |
| `bne` at 0x3ed94 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x3edc0 | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x3edf8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3ee08 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3ee38 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x3ee64 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3ee74 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3eea4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x3eed0 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x3eef4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x3ef30 | `ret(0x40898) == 0x0` | `ret(0x40898) != 0x0` |
| `bne` at 0x3ef3c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x3ef80 | `ret(0x3fed0) == 0x0` | `ret(0x3fed0) != 0x0` |
| `bne` at 0x3ef8c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3efb4 | `r0` | `?` |

## `portDownInternal`

- `portDownInternal`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3efcc to 0x3f248.
  Established. Entry `r0` and `r1` are truncated into stack bytes (`strb` at 0x3efe4, `strb` at
  0x3efec). The first branch takes the non-`0xff` arm for `(r0 & 0xff)` and falls through on
  equality (`bne` at 0x3f000). Later the call to `0x40ac0` has candidates `(unknown, 0x119df4,
  unknown, unknown)` (`bl` at 0x3f1e8). A zero result takes its following branch, while nonzero
  falls through (`beq` at 0x3f1fc). The physical link-down operation and return meaning remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3efe4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x3efec | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x3eff4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x3eff8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3f008 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3f00c | load | 4 | `abs 0x3f234` | `-` |
| `ldr` at 0x3f010 | load | 4 | `abs 0x119df8` | `-` |
| `str` at 0x3f014 | store | 4 | `sp -0x2c` | `[0x119df8]` |
| `ldrb` at 0x3f01c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3f03c | store | 4 | `sp -0x30` | `ret(0x406c0)` |
| `ldrb` at 0x3f044 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3f04c | load | 4 | `abs 0x3f238` | `-` |
| `ldr` at 0x3f050 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3f058 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f064 | load | 4 | `abs 0x3f23c` | `-` |
| `str` at 0x3f068 | store | 4 | `sp -0x30` | `0xfffffb4e` |
| `ldrb` at 0x3f070 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3f078 | load | 4 | `abs 0x3f238` | `-` |
| `ldr` at 0x3f07c | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3f084 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f090 | load | 4 | `abs 0x3f240` | `-` |
| `str` at 0x3f094 | store | 4 | `sp -0x30` | `0xfffffb4d` |
| `ldrb` at 0x3f09c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3f0a0 | store | 4 | `sp -0x28` | `[sp-17]:1` |
| `ldrb` at 0x3f0a4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3f0ac | store | 4 | `sp -0x2c` | `[sp-17]:1+1` |
| `ldr` at 0x3f0b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3f0b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3f0b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f0bc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3f0cc | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x3f0d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f0ec | load | 4 | `abs 0x3f244` | `-` |
| `ldr` at 0x3f0f0 | load | 4 | `abs 0x119df4` | `-` |
| `ldrh` at 0x3f0f8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3f108 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f11c | load | 4 | `abs 0x3f244` | `-` |
| `ldr` at 0x3f120 | load | 4 | `abs 0x119df4` | `-` |
| `ldr` at 0x3f128 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x3f138 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x3f144 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f158 | load | 4 | `abs 0x3f244` | `-` |
| `ldr` at 0x3f15c | load | 4 | `abs 0x119df4` | `-` |
| `ldrh` at 0x3f164 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3f174 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f188 | load | 4 | `abs 0x3f244` | `-` |
| `ldr` at 0x3f18c | load | 4 | `abs 0x119df4` | `-` |
| `ldr` at 0x3f194 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f1a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f1ac | load | 4 | `abs 0x3f238` | `-` |
| `ldr` at 0x3f1b0 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3f1b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f1c4 | load | 4 | `abs 0x3f244` | `-` |
| `ldr` at 0x3f1c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f1dc | load | 4 | `abs 0x119df4` | `-` |
| `str` at 0x3f1f0 | store | 4 | `sp -0x20` | `ret(0x40ac0)` |
| `ldr` at 0x3f1f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3f200 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f20c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f210 | store | 4 | `sp -0x1c` | `ret(0x40ac0)` |
| `ldr` at 0x3f214 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3f21c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3f224 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f228 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3f22c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x3f230 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3f230 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3f230 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f024 | `(unknown callee) 0x40650` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0x3f034 | `(unknown callee) 0x406c0` | `ret(0x40650)` | `?` | `?` | `ret(0x40650)` |
| `bl` at 0x3f1e8 | `(unknown callee) 0x40ac0` | `?` | `0x119df4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3f000 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `bne` at 0x3f030 | `ret(0x40650) != 0x0` | `ret(0x40650) == 0x0` |
| `bne` at 0x3f060 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x3f08c | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `blt` at 0x3f0c4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3f0d4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3f104 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x3f130 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3f140 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x3f170 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x3f19c | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x3f1c0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x3f1fc | `ret(0x40ac0) == 0x0` | `ret(0x40ac0) != 0x0` |
| `bne` at 0x3f208 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3f230 | `r0` | `?` |

## `portStatus`

- `portStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f248 to 0x3f6e4.
  Established. The first call to `0xb7af8` has candidates `([[sp-32]]:1, [sp-28], unknown,
  [[sp-32]]:1)` (`bl` at 0x3f2d0); its zero result takes the following branch, while nonzero falls
  through (`beq` at 0x3f2e4). Explicit writes at `[sp-28] +0x0` include byte values `([sp-57]:1 &
  0xff)` (`strb` at 0x3f34c) and `([sp-37]:1 & 0xff)` (`strb` at 0x3f418), plus word values
  `[sp-64]` (`str` at 0x3f4b4) and `[sp-68]` (`str` at 0x3f4f4). A later computed transfer uses an
  unsigned unknown-value comparison with `0x33` (`ldrls` at 0x3f568); its taken edges and the status
  encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f25c | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x3f260 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0x3f268 | store | 1 | `sp -0x25` | `0x0` |
| `ldrb` at 0x3f26c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3f27c | store | 4 | `sp -0x20` | `ret(0x40650)` |
| `ldr` at 0x3f280 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f294 | store | 4 | `sp -0x34` | `ret(0x406c0)` |
| `ldrb` at 0x3f29c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3f2b4 | load | 4 | `abs 0x3f6dc` | `-` |
| `str` at 0x3f2b8 | store | 4 | `sp -0x34` | `0xfffffb4e` |
| `ldr` at 0x3f2c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f2c4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x3f2cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f2d8 | store | 4 | `sp -0x24` | `ret(0xb7af8)` |
| `ldr` at 0x3f2dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f2e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3f2ec | store | 4 | `sp -0x34` | `ret(0xb7af8)` |
| `ldr` at 0x3f2f4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3f2f8 | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3f308 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3f30c | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0x3f310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f314 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f318 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3f31c | load | 1 | `[sp-32] +0x1` | `-` |
| `strb` at 0x3f334 | store | 1 | `sp -0x39` | `0x4` |
| `strb` at 0x3f340 | store | 1 | `sp -0x39` | `0x0` |
| `ldrb` at 0x3f344 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0x3f348 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0x3f34c | store | 1 | `[sp-28] +0x0` | `([sp-57]:1 & 0xff)` |
| `ldr` at 0x3f354 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f358 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x3f364 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f368 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f36c | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x3f374 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x3f378 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f37c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f380 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x3f388 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x3f38c | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3f390 | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3f3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f3a4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3f3b0 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3f3b8 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3f3bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f3c0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3f3cc | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3f3d4 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3f3d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f3dc | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3f3e8 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3f3f0 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3f3f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f3f8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x3f404 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3f40c | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3f410 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f414 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0x3f418 | store | 1 | `[sp-28] +0x0` | `([sp-37]:1 & 0xff)` |
| `ldr` at 0x3f41c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f420 | load | 1 | `[sp-32] +0x2` | `-` |
| `str` at 0x3f438 | store | 4 | `sp -0x24` | `ret(0xb7af8)` |
| `ldr` at 0x3f43c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3f448 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3f44c | store | 4 | `sp -0x34` | `ret(0xb7af8)` |
| `ldr` at 0x3f454 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3f458 | load | 1 | `sp -0x25` | `-` |
| `ldrb` at 0x3f45c | load | 1 | `[sp-32] +0x3` | `-` |
| `strb` at 0x3f464 | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0x3f468 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3f46c | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3f47c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f480 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x3f490 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3f494 | store | 4 | `sp -0x40` | `[[sp-28]]` |
| `ldr` at 0x3f49c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3f4ac | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3f4b0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3f4b4 | store | 4 | `[sp-28] +0x0` | `[sp-64]` |
| `ldr` at 0x3f4bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f4c0 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0x3f4d0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x3f4d4 | store | 4 | `sp -0x44` | `[[sp-28]]` |
| `ldr` at 0x3f4dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3f4ec | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x3f4f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x3f4f4 | store | 4 | `[sp-28] +0x0` | `[sp-68]` |
| `ldr` at 0x3f4f8 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3f4fc | load | 2 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x3f50c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f510 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3f524 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f528 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x3f53c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f540 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x3f568 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f664 | load | 4 | `abs 0x3f6e0` | `-` |
| `ldr` at 0x3f670 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f680 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x3f69c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3f69c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f6b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3f6b8 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x3f6c0 | store | 1 | `[sp-28] +0x0` | `?` |
| `str` at 0x3f6c8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3f6cc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3f6d0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3f6d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3f6d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3f6d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f274 | `(unknown callee) 0x40650` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x3f28c | `(unknown callee) 0x406c0` | `ret(0x40650)` | `?` | `?` | `ret(0x40650)` |
| `bl` at 0x3f2a4 | `(unknown callee) 0x3ead4` | `[sp-21]:1` | `?` | `?` | `[sp-21]:1` |
| `bl` at 0x3f2d0 | `(unknown callee) 0xb7af8` | `[[sp-32]]:1` | `[sp-28]` | `?` | `[[sp-32]]:1` |
| `bl` at 0x3f430 | `(unknown callee) 0xb7af8` | `[[sp-32]+2]:1` | `sp-37` | `sp-37` | `[[sp-32]+2]:1` |
| `bl` at 0x3f554 | `(unknown callee) 0x2b31c` | `[[sp-28]]` | `[sp-28]` | `?` | `?` |
| `bl` at 0x3f668 | `(unknown callee) 0x1d6d8` | `0xe1f38` | `?` | `?` | `?` |
| `bl` at 0x3f67c | `(unknown callee) 0x3cd7c` | `[sp-48]` | `0x0` | `sp-44` | `sp-44` |
| `bl` at 0x3f684 | `(unknown callee) 0xd5c38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x3f6a0 | `(unknown callee) 0xd6150` | `ret(0xd5c38)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3f288 | `ret(0x40650) != 0x0` | `ret(0x40650) == 0x0` |
| `bne` at 0x3f2b0 | `ret(0x3ead4) != 0x0` | `ret(0x3ead4) == 0x0` |
| `beq` at 0x3f2e4 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `beq` at 0x3f304 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3f32c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3f360 | `[[sp-32]+1]:1 == 0x0` | `[[sp-32]+1]:1 != 0x0` |
| `beq` at 0x3f39c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3f444 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `beq` at 0x3f478 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3f508 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3f520 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3f538 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x3f550 | `? != 0x0` | `? == 0x0` |
| `ldrls` at 0x3f568 | `? <= 0x33 (unsigned, 32-bit)` | `? > 0x33 (unsigned, 32-bit)` |
| `blt` at 0x3f6ac | `ret(0xd6150) < 0x0 (signed, 32-bit)` | `ret(0xd6150) >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3f6d0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3f568).

## `portState`

- `portState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f754 to 0x3f818.
  Established. Four words `[0xe1fa0]`, `[0xe1fa4]`, `[0xe1fa8]` and `[0xe1fac]` are stored at stack
  offsets `-0x2c`, `-0x28`, `-0x24` and `-0x20`, respectively (`stm` at 0x3f77c). Two further words
  `[0xe1fb0]` and `[0xe1fb4]` reach `sp -0x1c` and `sp -0x18` (`stm` at 0x3f784). The branch for
  `(r0 & 0xff) >= [0x119df8]` signed takes the path that stores `0xe1f54` at `sp -0x30` (`bge` at
  0x3f798, `str` at 0x3f7c4). The merged return remains unknown (`ldmdb` at 0x3f804). No state-label
  interpretation is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3f768 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x3f76c | load | 4 | `abs 0x3f808` | `-` |
| `ldm` at 0x3f778 | load | 4 | `abs 0xe1fa0` | `-` |
| `ldm` at 0x3f778 | load | 4 | `abs 0xe1fa4` | `-` |
| `ldm` at 0x3f778 | load | 4 | `abs 0xe1fa8` | `-` |
| `ldm` at 0x3f778 | load | 4 | `abs 0xe1fac` | `-` |
| `stm` at 0x3f77c | store | 4 | `sp -0x2c` | `[0xe1fa0]` |
| `stm` at 0x3f77c | store | 4 | `sp -0x28` | `[0xe1fa4]` |
| `stm` at 0x3f77c | store | 4 | `sp -0x24` | `[0xe1fa8]` |
| `stm` at 0x3f77c | store | 4 | `sp -0x20` | `[0xe1fac]` |
| `ldm` at 0x3f780 | load | 4 | `abs 0xe1fb0` | `-` |
| `ldm` at 0x3f780 | load | 4 | `abs 0xe1fb4` | `-` |
| `stm` at 0x3f784 | store | 4 | `sp -0x1c` | `[0xe1fb0]` |
| `stm` at 0x3f784 | store | 4 | `sp -0x18` | `[0xe1fb4]` |
| `ldrb` at 0x3f788 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3f78c | load | 4 | `abs 0x3f80c` | `-` |
| `ldr` at 0x3f790 | load | 4 | `abs 0x119df8` | `-` |
| `ldrb` at 0x3f79c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3f7a4 | load | 4 | `abs 0x3f810` | `-` |
| `ldr` at 0x3f7a8 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3f7b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f7c0 | load | 4 | `abs 0x3f814` | `-` |
| `str` at 0x3f7c4 | store | 4 | `sp -0x30` | `0xe1f54` |
| `ldrb` at 0x3f7cc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3f7d4 | load | 4 | `abs 0x3f810` | `-` |
| `ldr` at 0x3f7d8 | load | 4 | `abs 0x119df0` | `-` |
| `ldrb` at 0x3f7e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3f7f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3f7fc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3f800 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x3f804 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3f804 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3f804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x3f798 | `(r0 & 0xff) >= [0x119df8] (signed, 32-bit)` | `(r0 & 0xff) < [0x119df8] (signed, 32-bit)` |
| `bhi` at 0x3f7b8 | `? > 0x5 (unsigned, 32-bit)` | `? <= 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3f804 | `r0` | `?` |

## `portQuery`

- `portQuery`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3f818 to 0x3fc24.
  Established. The first call to `0x3ecc4` uses `sp-32` as its first candidate (`bl` at 0x3f83c). A
  nonzero result takes the branch past the next call sequence, while zero falls through (`bne` at
  0x3f850). Two word stores of `ret(0x18920)` target `[sp-24] +0x0` (`str` at 0x3f864, `str` at
  0x3f89c). Within the loop, `0x3f248` receives candidates `([sp-40]:1, sp-36, sp-36, [sp-40]:1)`
  (`bl` at 0x3f8fc). The exit reports `ret(0x18a1c)` (`ldmdb` at 0x3fbf0). Query field meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3f828 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3f830 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x3f844 | store | 4 | `sp -0x2c` | `ret(0x3ecc4)` |
| `ldr` at 0x3f848 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3f854 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3f864 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x3f868 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f86c | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x3f870 | load | 4 | `abs 0x3fbf4` | `-` |
| `ldr` at 0x3f878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3f87c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3f89c | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x3f8a0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x3f8a4 | store | 4 | `sp -0x30` | `[[sp-24]]` |
| `ldr` at 0x3f8a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f8ac | load | 4 | `abs 0x3fbf8` | `-` |
| `ldr` at 0x3f8b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f8c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3f8cc | store | 4 | `sp -0x30` | `([sp-48] + ret(0x18a1c))` |
| `str` at 0x3f8d4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3f8d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f8dc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3f8ec | load | 1 | `sp -0x28` | `-` |
| `str` at 0x3f904 | store | 4 | `sp -0x1c` | `ret(0x3f248)` |
| `ldr` at 0x3f908 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f914 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f918 | load | 4 | `abs 0x3fbfc` | `-` |
| `ldr` at 0x3f924 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f938 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3f940 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f944 | load | 4 | `abs 0x3fc00` | `-` |
| `ldr` at 0x3f948 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3f960 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f974 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3f97c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3f980 | load | 4 | `abs 0x3fc04` | `-` |
| `ldr` at 0x3f984 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3f994 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f9a8 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3f9b0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3f9c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3f9d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3f9dc | store | 4 | `sp -0x34` | `?` |
| `ldrb` at 0x3f9e0 | load | 1 | `sp -0x24` | `-` |
| `str` at 0x3f9e4 | store | 4 | `sp -0x38` | `[sp-36]:1` |
| `ldrb` at 0x3f9e8 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x3f9fc | load | 4 | `abs 0x3fc08` | `-` |
| `str` at 0x3fa00 | store | 4 | `sp -0x3c` | `0xe207c` |
| `ldr` at 0x3fa08 | load | 4 | `abs 0x3fc0c` | `-` |
| `str` at 0x3fa0c | store | 4 | `sp -0x3c` | `0xe2080` |
| `ldr` at 0x3fa10 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3fa14 | store | 4 | `sp -0x70` | `[sp-60]` |
| `ldr` at 0x3fa18 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3fa1c | load | 4 | `abs 0x3fc10` | `-` |
| `ldr` at 0x3fa20 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3fa24 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x3fa28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3fa34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3fa48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3fa50 | store | 4 | `sp -0x40` | `?` |
| `ldrb` at 0x3fa54 | load | 1 | `sp -0x24` | `-` |
| `str` at 0x3fa58 | store | 4 | `sp -0x44` | `[sp-36]:1` |
| `ldrb` at 0x3fa5c | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x3fa70 | load | 4 | `abs 0x3fc14` | `-` |
| `str` at 0x3fa74 | store | 4 | `sp -0x48` | `0xe20b4` |
| `ldr` at 0x3fa7c | load | 4 | `abs 0x3fc18` | `-` |
| `str` at 0x3fa80 | store | 4 | `sp -0x48` | `0xe20bc` |
| `ldr` at 0x3fa84 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x3fa88 | store | 4 | `sp -0x70` | `[sp-72]` |
| `ldrb` at 0x3fa8c | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x3faa0 | load | 4 | `abs 0x3fc14` | `-` |
| `str` at 0x3faa4 | store | 4 | `sp -0x4c` | `0xe20b4` |
| `ldr` at 0x3faac | load | 4 | `abs 0x3fc18` | `-` |
| `str` at 0x3fab0 | store | 4 | `sp -0x4c` | `0xe20bc` |
| `ldr` at 0x3fab4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x3fab8 | store | 4 | `sp -0x6c` | `[sp-76]` |
| `ldrb` at 0x3fabc | load | 1 | `sp -0x24` | `-` |
| `ldrb` at 0x3fad0 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x3fae4 | load | 4 | `abs 0x3fc1c` | `-` |
| `str` at 0x3fae8 | store | 4 | `sp -0x54` | `0xe20c4` |
| `ldr` at 0x3faf0 | load | 4 | `abs 0x3fc0c` | `-` |
| `str` at 0x3faf4 | store | 4 | `sp -0x54` | `0xe2080` |
| `ldr` at 0x3faf8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x3fafc | store | 4 | `sp -0x50` | `[sp-84]` |
| `ldr` at 0x3fb04 | load | 4 | `abs 0x3fc08` | `-` |
| `str` at 0x3fb08 | store | 4 | `sp -0x50` | `0xe207c` |
| `ldr` at 0x3fb0c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x3fb10 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x3fb14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3fb28 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x3fb30 | load | 2 | `(unknown)` | `-` |
| `ldrb` at 0x3fb40 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x3fb54 | load | 4 | `abs 0x3fc0c` | `-` |
| `str` at 0x3fb58 | store | 4 | `sp -0x5c` | `0xe2080` |
| `ldr` at 0x3fb60 | load | 4 | `abs 0x3fc08` | `-` |
| `str` at 0x3fb64 | store | 4 | `sp -0x5c` | `0xe207c` |
| `ldr` at 0x3fb68 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x3fb6c | store | 4 | `sp -0x58` | `[sp-92]` |
| `ldrb` at 0x3fb74 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x3fb88 | load | 4 | `abs 0x3fc08` | `-` |
| `str` at 0x3fb8c | store | 4 | `sp -0x60` | `0xe207c` |
| `ldr` at 0x3fb94 | load | 4 | `abs 0x3fc0c` | `-` |
| `str` at 0x3fb98 | store | 4 | `sp -0x60` | `0xe2080` |
| `ldr` at 0x3fb9c | load | 4 | `sp -0x60` | `-` |
| `str` at 0x3fba0 | store | 4 | `sp -0x58` | `[sp-96]` |
| `ldr` at 0x3fba4 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x3fba8 | store | 4 | `sp -0x64` | `[sp-88]` |
| `ldr` at 0x3fbac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3fbb0 | load | 4 | `abs 0x3fc20` | `-` |
| `ldr` at 0x3fbb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3fbb8 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldr` at 0x3fbbc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x3fbc4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3fbd4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3fbdc | store | 4 | `sp -0x30` | `([sp-48] + ret(0x18a1c))` |
| `ldr` at 0x3fbe0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3fbe8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0x3fbf0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3fbf0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3fbf0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3fbf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3f83c | `(unknown callee) 0x3ecc4` | `sp-32` | `r1` | `r2` | `sp-32` |
| `bl` at 0x3f85c | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `ret(0x3ecc4)` |
| `bl` at 0x3f874 | `(unknown callee) 0x18cd4` | `[[sp-24]]` | `0xe1fb8` | `?` | `[sp-24]` |
| `bl` at 0x3f894 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x3f8b0 | `(unknown callee) 0x18c74` | `[[sp-24]]` | `0xe1fd0` | `?` | `[[sp-24]]` |
| `bl` at 0x3f8b8 | `(unknown callee) 0x18a1c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x3f8fc | `(unknown callee) 0x3f248` | `[sp-40]:1` | `sp-36` | `sp-36` | `[sp-40]:1` |
| `bl` at 0x3f94c | `(unknown callee) 0x18c74` | `[sp-48]` | `0xe2048` | `?` | `?` |
| `bl` at 0x3f958 | `(unknown callee) 0x1e71c` | `ret(0x3f248)` | `?` | `ret(0x3f248)` | `0xfffffb4e` |
| `bl` at 0x3f98c | `(unknown callee) 0x18c74` | `[sp-48]` | `0xe205c` | `?` | `ret(0x1e71c)` |
| `bl` at 0x3fa2c | `(unknown callee) 0x18c74` | `[sp-48]` | `0xe2084` | `[[sp-52]+8]` | `[sp-36]:1` |
| `bl` at 0x3fbc0 | `(unknown callee) 0x18c74` | `[sp-48]` | `0xe20c8` | `[[sp-64]+8]` | `[sp-36]:1` |
| `bl` at 0x3fbc8 | `(unknown callee) 0x18a1c` | `[sp-48]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3f850 | `ret(0x3ecc4) != 0x0` | `ret(0x3ecc4) == 0x0` |
| `blt` at 0x3f8e4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3f910 | `ret(0x3f248) == 0x0` | `ret(0x3f248) != 0x0` |
| `bne` at 0x3f920 | `ret(0x3f248) != 0xfffffb4e` | `ret(0x3f248) == 0xfffffb4e` |
| `beq` at 0x3f9bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3f9f8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3fa6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3fa9c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x3facc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x3fae0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3fb3c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3fb50 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3fb84 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3fbf0 | `r0` | `ret(0x18a1c)` |

## `portInt`

- `portInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3fc24 to 0x3fed0.
  Established. The three entry values are truncated into bytes `(r0 & 0xff)` (`strb` at 0x3fc38),
  `(r1 & 0xff)` (`strb` at 0x3fc40) and `(r2 & 0xff)` (`strb` at 0x3fc48). Calls to `0x4006c`
  include candidates `([sp-24], unknown, unknown, 0x1)` (`bl` at 0x3fd8c) and `(unknown, unknown,
  unknown, 0x1)` (`bl` at 0x3fe10). A later call to `0x1fe64` has first candidate `0x119dfc` (`bl`
  at 0x3fe64); its zero result takes the branch past the final call, while nonzero falls through
  (`beq` at 0x3fe78). Interrupt source, acknowledgement and byte-field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3fc38 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x3fc40 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x3fc48 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldr` at 0x3fc4c | load | 4 | `abs 0x3fea4` | `-` |
| `str` at 0x3fc5c | store | 4 | `sp -0x24` | `ret(0x1fb00)` |
| `ldr` at 0x3fc60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3fc6c | load | 4 | `abs 0x3fea8` | `-` |
| `str` at 0x3fc70 | store | 4 | `sp -0x34` | `0xe20e8` |
| `ldr` at 0x3fc74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3fc78 | store | 4 | `sp -0x30` | `ret(0x1fb00)` |
| `ldr` at 0x3fc80 | load | 4 | `abs 0x3feac` | `-` |
| `ldrb` at 0x3fc90 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3fca0 | store | 4 | `sp -0x18` | `ret(0x40708)` |
| `ldrb` at 0x3fca4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3fcb4 | store | 4 | `sp -0x1c` | `ret(0x407d0)` |
| `ldr` at 0x3fcb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3fcc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fcd0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x3fce0 | load | 4 | `abs 0x3feb0` | `-` |
| `str` at 0x3fce4 | store | 4 | `sp -0x34` | `0xe210c` |
| `str` at 0x3fce8 | store | 4 | `sp -0x30` | `ret(0xb8268)` |
| `ldrb` at 0x3fcec | load | 1 | `sp -0x12` | `-` |
| `str` at 0x3fcf0 | store | 4 | `sp -0x2c` | `[sp-18]:1` |
| `ldr` at 0x3fcf8 | load | 4 | `abs 0x3feac` | `-` |
| `ldr` at 0x3fcfc | load | 4 | `abs 0x3feb4` | `-` |
| `ldrb` at 0x3fd08 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x3fd0c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x3fd1c | load | 4 | `abs 0x3feb8` | `-` |
| `ldr` at 0x3fd20 | load | 4 | `abs 0x119f84` | `-` |
| `ldr` at 0x3fd2c | load | 4 | `abs 0x3febc` | `-` |
| `str` at 0x3fd30 | store | 4 | `sp -0x34` | `0xe2148` |
| `ldrb` at 0x3fd34 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x3fd38 | store | 4 | `sp -0x30` | `[sp-17]:1` |
| `ldrb` at 0x3fd3c | load | 1 | `sp -0x12` | `-` |
| `str` at 0x3fd40 | store | 4 | `sp -0x2c` | `[sp-18]:1` |
| `ldrb` at 0x3fd44 | load | 1 | `sp -0x13` | `-` |
| `str` at 0x3fd48 | store | 4 | `sp -0x28` | `[sp-19]:1` |
| `ldr` at 0x3fd50 | load | 4 | `abs 0x3feac` | `-` |
| `ldr` at 0x3fd60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3fd6c | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x3fd74 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x3fd7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3fd94 | store | 4 | `sp -0x20` | `ret(0x4006c)` |
| `ldr` at 0x3fd98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3fda4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3fdb0 | load | 4 | `abs 0x3fec0` | `-` |
| `str` at 0x3fdb4 | store | 4 | `sp -0x34` | `0xe2178` |
| `ldr` at 0x3fdb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3fdbc | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x3fdc0 | store | 4 | `sp -0x30` | `[[sp-24]+8]` |
| `str` at 0x3fdc4 | store | 4 | `sp -0x2c` | `ret(0x1ea50)` |
| `ldr` at 0x3fdcc | load | 4 | `abs 0x3feac` | `-` |
| `ldr` at 0x3fddc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3fde8 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x3fdf4 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x3fe00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3fe18 | store | 4 | `sp -0x20` | `ret(0x4006c)` |
| `ldr` at 0x3fe1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3fe28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3fe34 | load | 4 | `abs 0x3fec0` | `-` |
| `str` at 0x3fe38 | store | 4 | `sp -0x34` | `0xe2178` |
| `ldr` at 0x3fe3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe40 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x3fe44 | store | 4 | `sp -0x30` | `[[sp-28]+8]` |
| `str` at 0x3fe48 | store | 4 | `sp -0x2c` | `ret(0x1ea50)` |
| `ldr` at 0x3fe50 | load | 4 | `abs 0x3feac` | `-` |
| `ldr` at 0x3fe54 | load | 4 | `abs 0x3fec4` | `-` |
| `ldr` at 0x3fe60 | load | 4 | `abs 0x3fea4` | `-` |
| `str` at 0x3fe6c | store | 4 | `sp -0x24` | `ret(0x1fe64)` |
| `ldr` at 0x3fe70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3fe7c | load | 4 | `abs 0x3fec8` | `-` |
| `str` at 0x3fe80 | store | 4 | `sp -0x34` | `0xe2194` |
| `ldr` at 0x3fe84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3fe88 | store | 4 | `sp -0x30` | `ret(0x1fe64)` |
| `ldr` at 0x3fe90 | load | 4 | `abs 0x3feac` | `-` |
| `ldr` at 0x3fe94 | load | 4 | `abs 0x3fecc` | `-` |
| `ldmdb` at 0x3fea0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3fea0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3fea0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3fc54 | `(unknown callee) 0x1fb00` | `0x119dfc` | `?` | `r2` | `r2` |
| `bl` at 0x3fc8c | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0x2d4` | `0x0` |
| `bl` at 0x3fc98 | `(unknown callee) 0x40708` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x3fcac | `(unknown callee) 0x407d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x3fcd8 | `(unknown callee) 0xb8268` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x3fd04 | `(unknown callee) 0x1eed0` | `0x1` | `0xe1da4` | `0x2dd` | `0x0` |
| `bl` at 0x3fd18 | `(unknown callee) 0xb7c2c` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |
| `bl` at 0x3fd5c | `(unknown callee) 0x1eed0` | `0x6` | `0xe1da4` | `0x2e4` | `0x0` |
| `bl` at 0x3fd8c | `(unknown callee) 0x4006c` | `[sp-24]` | `?` | `?` | `0x1` |
| `bl` at 0x3fda8 | `(unknown callee) 0x1ea50` | `ret(0x4006c)` | `?` | `?` | `ret(0x4006c)` |
| `bl` at 0x3fdd8 | `(unknown callee) 0x1eed0` | `0x3` | `0xe1da4` | `0x2ec` | `0x0` |
| `bl` at 0x3fe10 | `(unknown callee) 0x4006c` | `?` | `?` | `?` | `0x1` |
| `bl` at 0x3fe2c | `(unknown callee) 0x1ea50` | `ret(0x4006c)` | `?` | `?` | `ret(0x4006c)` |
| `bl` at 0x3fe5c | `(unknown callee) 0x1eed0` | `0x3` | `0xe1da4` | `0x2f2` | `0x0` |
| `bl` at 0x3fe64 | `(unknown callee) 0x1fe64` | `0x119dfc` | `?` | `?` | `?` |
| `bl` at 0x3fe9c | `(unknown callee) 0x1eed0` | `0x9` | `0xe1da4` | `0x2fa` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3fc68 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `bne` at 0x3fcc0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x3fccc | `ret(0x407d0) != 0x0` | `ret(0x407d0) == 0x0` |
| `beq` at 0x3fd28 | `[0x119f84] == 0x0` | `[0x119f84] != 0x0` |
| `beq` at 0x3fd68 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x3fda0 | `ret(0x4006c) == 0x0` | `ret(0x4006c) != 0x0` |
| `beq` at 0x3fde4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3fe24 | `ret(0x4006c) == 0x0` | `ret(0x4006c) != 0x0` |
| `beq` at 0x3fe78 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3fea0 | `r0` | `?` |
