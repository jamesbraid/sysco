# ARM L1 power entry paths in 1.34.2

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
values, and brackets denote loaded values. Repeated loads with the same expression are separate
observations and need not return the same value. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. An unresolved computed transfer can leave additional effects and return paths unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/power-facts.txt) retain all tool sections.

## `powerInit`

- `powerInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x395a0 to 0x396c4.
  Established. The word read at 0x281120 decides `beq`, whose taken zero path skips the first call
  to 0x1eed0 (`ldr` at 0x395dc, `beq` at 0x395e4, `bl` at 0x39600). The call to 0x18ba8 supplies the
  word stored at 0x2810e0 (`bl` at 0x39610, `str` at 0x39618). Later word stores write `[sp-24]`,
  `[sp-28]`, `[sp-32]` and `[sp-36]` at 0x281120, 0x281100, 0x281140 and 0x281160 respectively
  (`str` at 0x39668, `str` at 0x39674, `str` at 0x39680, `str` at 0x3968c). The globals' device
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x395b0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x395b4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x395b8 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x395bc | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x395c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x395d4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x395d8 | load | 4 | `abs 0x3969c` | `-` |
| `ldr` at 0x395dc | load | 4 | `abs 0x281120` | `-` |
| `ldr` at 0x395e8 | load | 4 | `abs 0x396a0` | `-` |
| `str` at 0x395ec | store | 4 | `sp -0x30` | `0xe1564` |
| `ldr` at 0x395f4 | load | 4 | `abs 0x396a4` | `-` |
| `ldr` at 0x395f8 | load | 4 | `abs 0x396a8` | `-` |
| `ldr` at 0x39604 | load | 4 | `abs 0x396ac` | `-` |
| `ldr` at 0x39608 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39618 | store | 4 | `abs 0x2810e0` | `ret(0x18ba8)` |
| `ldr` at 0x3961c | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x39628 | load | 4 | `abs 0x396b0` | `-` |
| `str` at 0x3962c | store | 4 | `sp -0x30` | `0xe159c` |
| `ldr` at 0x39630 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39634 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x3963c | load | 4 | `abs 0x396a4` | `-` |
| `ldr` at 0x39640 | load | 4 | `abs 0x396b4` | `-` |
| `ldr` at 0x3964c | load | 4 | `abs 0x396ac` | `-` |
| `ldr` at 0x39650 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x39658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39660 | load | 4 | `abs 0x3969c` | `-` |
| `ldr` at 0x39664 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x39668 | store | 4 | `abs 0x281120` | `[sp-24]` |
| `ldr` at 0x3966c | load | 4 | `abs 0x396b8` | `-` |
| `ldr` at 0x39670 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39674 | store | 4 | `abs 0x281100` | `[sp-28]` |
| `ldr` at 0x39678 | load | 4 | `abs 0x396bc` | `-` |
| `ldr` at 0x3967c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x39680 | store | 4 | `abs 0x281140` | `[sp-32]` |
| `ldr` at 0x39684 | load | 4 | `abs 0x396c0` | `-` |
| `ldr` at 0x39688 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3968c | store | 4 | `abs 0x281160` | `[sp-36]` |
| `ldmdb` at 0x39698 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x39698 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39698 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39698 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39600 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x11f` | `0x0` |
| `bl` at 0x39610 | `(unknown callee) 0x18ba8` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x39648 | `(unknown callee) 0x1eed0` | `0x9` | `0xe1584` | `0x126` | `0x0` |
| `bl` at 0x3965c | `(unknown callee) 0x18c20` | `[0x2810e0]` | `0x1` | `[sp-28]` | `0x2810e0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x395e4 | `[0x281120] == 0x0` | `[0x281120] != 0x0` |
| `bne` at 0x39624 | `[0x2810e0] != 0x0` | `[0x2810e0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39698 | `r0` | `0x0` |

## `powerDisable`

- `powerDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x396c4 to 0x39754.
  Established. `blt` takes the negative-entry-r0 path to 0x1eed0 (`blt` at 0x396e0, `bl` at
  0x3971c). If it falls through, `bge` takes the same call path for `r0 >= [0x281100]` signed (`bge`
  at 0x396f4). Falling through both branches reaches a byte store of 0x0 at `([0x2810e0] + r0) +0x0`
  (`strb` at 0x39738). This indexed store does not establish a physical power-disable operation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x396d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x396d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x396e4 | load | 4 | `abs 0x39740` | `-` |
| `ldr` at 0x396e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x396ec | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x396fc | load | 4 | `abs 0x39744` | `-` |
| `str` at 0x39700 | store | 4 | `sp -0x1c` | `0xe15dc` |
| `ldr` at 0x39704 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x39708 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x39710 | load | 4 | `abs 0x39748` | `-` |
| `ldr` at 0x39714 | load | 4 | `abs 0x3974c` | `-` |
| `ldr` at 0x39724 | load | 4 | `abs 0x39750` | `-` |
| `ldr` at 0x39728 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x3972c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x39738 | store | 1 | `([0x2810e0] + r0) +0x0` | `0x0` |
| `ldmdb` at 0x3973c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3973c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3973c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3971c | `(unknown callee) 0x1eed0` | `0x6` | `0xe1584` | `0x13f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x396e0 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x396f4 | `r0 >= [0x281100] (signed, 32-bit)` | `r0 < [0x281100] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3973c | `r0` | `?` |

## `powerEnable`

- `powerEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39754 to 0x397e0.
  Established. The first `blt` takes the negative-entry-r0 path to the call to 0x1eed0 (`blt` at
  0x39770, `bl` at 0x397ac). The following `bge` also takes that call path for `r0 >= [0x281100]`
  signed (`bge` at 0x39784). The alternative indexed-byte path writes 0x1 at `([0x2810e0] + r0)
  +0x0` (`strb` at 0x397c8). The receiving byte's hardware role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39764 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x39768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39774 | load | 4 | `abs 0x397d0` | `-` |
| `ldr` at 0x39778 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3977c | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x3978c | load | 4 | `abs 0x397d4` | `-` |
| `str` at 0x39790 | store | 4 | `sp -0x1c` | `0xe1604` |
| `ldr` at 0x39794 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x39798 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x397a0 | load | 4 | `abs 0x397d8` | `-` |
| `ldr` at 0x397b4 | load | 4 | `abs 0x397dc` | `-` |
| `ldr` at 0x397b8 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x397bc | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x397c8 | store | 1 | `([0x2810e0] + r0) +0x0` | `0x1` |
| `ldmdb` at 0x397cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x397cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x397cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x397ac | `(unknown callee) 0x1eed0` | `0x6` | `0xe1584` | `0x14c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x39770 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x39784 | `r0 >= [0x281100] (signed, 32-bit)` | `r0 < [0x281100] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x397cc | `r0` | `?` |

## `powerReset`

- `powerReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x397e0 to 0x39994.
  Established. The initial word store at 0x1155e0 has an unknown value (`str` at 0x39800). The
  loop's `blt` has unknown operands (`blt` at 0x3981c). A later `bne` retains only the predicate Z
  == 0, with fall-through reaching a word store of `[sp-28]` at 0x1155e0 (`bne` at 0x398fc, `str` at
  0x39908). On its taken path, `beq` compares a fresh word load from that global with `[sp-28]`. Unequal fall-through writes 0x5 there (`ldr` at 0x39914, `beq` at 0x39920, `str` at 0x3992c).
  Calls include 0x1c79c, 0x3d8ac and 0x3dea4, but their physical reset roles remain unknown (`bl` at
  0x39884, `bl` at 0x398e4, `bl` at 0x3993c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x397f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x397f8 | load | 4 | `abs 0x39984` | `-` |
| `str` at 0x39800 | store | 4 | `abs 0x1155e0` | `?` |
| `str` at 0x39808 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3980c | load | 4 | `abs 0x39988` | `-` |
| `ldr` at 0x39810 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39814 | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x39824 | load | 4 | `abs 0x3998c` | `-` |
| `ldr` at 0x39828 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3983c | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x39844 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x39848 | load | 4 | `abs 0x39990` | `-` |
| `ldr` at 0x3984c | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x39850 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x39858 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39864 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x39868 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x39874 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x39878 | load | 1 | `(unknown)` | `-` |
| `str` at 0x39888 | store | 4 | `sp -0x1c` | `ret(0x1c79c)` |
| `ldr` at 0x3988c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39890 | store | 4 | `sp -0x2c` | `ret(0x1c79c)` |
| `ldr` at 0x39894 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x398a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x398ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x398b8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x398bc | load | 1 | `[sp-20] +0x10` | `-` |
| `strb` at 0x398d0 | store | 1 | `sp -0x15` | `(ret(0x1c79c) & 0xff)` |
| `str` at 0x398dc | store | 4 | `sp -0x1c` | `0x2` |
| `ldr` at 0x398e0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x398ec | store | 1 | `sp -0x15` | `(ret(0x3d8ac) & 0xff)` |
| `ldr` at 0x398f0 | load | 4 | `abs 0x39984` | `-` |
| `ldr` at 0x398f4 | load | 4 | `abs 0x1155e0` | `-` |
| `ldr` at 0x39900 | load | 4 | `abs 0x39984` | `-` |
| `ldr` at 0x39904 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39908 | store | 4 | `abs 0x1155e0` | `[sp-28]` |
| `ldr` at 0x39910 | load | 4 | `abs 0x39984` | `-` |
| `ldr` at 0x39914 | load | 4 | `abs 0x1155e0` | `-` |
| `ldr` at 0x39918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39924 | load | 4 | `abs 0x39984` | `-` |
| `str` at 0x3992c | store | 4 | `abs 0x1155e0` | `0x5` |
| `ldrb` at 0x39930 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x39934 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x39944 | store | 4 | `sp -0x24` | `ret(0x3dea4)` |
| `ldr` at 0x39948 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39954 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39960 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x39964 | store | 4 | `sp -0x20` | `ret(0x3dea4)` |
| `ldr` at 0x39968 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39970 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39978 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x39980 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39980 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39980 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39884 | `(unknown callee) 0x1c79c` | `?` | `0x281120` | `[0x2810e0]` | `?` |
| `bl` at 0x398c8 | `(unknown callee) 0x1c79c` | `[[sp-20]+16]:1+130` | `?` | `?` | `[[sp-20]+16]:1+130` |
| `bl` at 0x398e4 | `(unknown callee) 0x3d8ac` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3993c | `(unknown callee) 0x3dea4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3981c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x39860 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x39870 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3989c | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `blt` at 0x398a8 | `ret(0x1c79c) < 0x0 (signed, 32-bit)` | `ret(0x1c79c) >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x398b4 | `ret(0x1c79c) > 0x4 (signed, 32-bit)` | `ret(0x1c79c) <= 0x4 (signed, 32-bit)` |
| `bne` at 0x398fc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x39920 | `[0x1155e0] == [sp-28]` | `[0x1155e0] != [sp-28]` |
| `beq` at 0x39950 | `ret(0x3dea4) == 0x0` | `ret(0x3dea4) != 0x0` |
| `bne` at 0x3995c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39980 | `r0` | `[sp-32]` |

## `powerPresent`

- `powerPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x399d0 to 0x39a3c.
  Established. The negative-entry-r0 path taken by `blt`, or the signed `r0 >= [0x281100]` path
  taken by `bge`, reaches a zero word save at sp -0x18 (`blt` at 0x399ec, `bge` at 0x39a00, `str` at
  0x39a0c). The other path loads a byte from `([0x2810e0] + r0) +0x0` and saves its expression as a
  word at that stack slot (`ldrb` at 0x39a24, `str` at 0x39a28). The merged return value is unknown
  (`ldmdb` at 0x39a30). No physical presence test is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x399e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x399e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x399f0 | load | 4 | `abs 0x39a34` | `-` |
| `ldr` at 0x399f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x399f8 | load | 4 | `abs 0x281100` | `-` |
| `str` at 0x39a0c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x39a14 | load | 4 | `abs 0x39a38` | `-` |
| `ldr` at 0x39a18 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x39a1c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x39a24 | load | 1 | `([0x2810e0] + r0) +0x0` | `-` |
| `str` at 0x39a28 | store | 4 | `sp -0x18` | `[([0x2810e0] + r0)]:1` |
| `ldr` at 0x39a2c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x39a30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39a30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39a30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x399ec | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x39a00 | `r0 >= [0x281100] (signed, 32-bit)` | `r0 < [0x281100] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39a30 | `r0` | `?` |

## `powerCycle`

- `powerCycle`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a838 to 0x3a8a0.
  Established. The first call targets 0x3a458 with entry r0 as its first candidate (`bl` at
  0x3a850). `beq` takes the zero-result path to 0x3e294 with r0 candidate 0xc8, followed by 0x39a3c
  with r0 candidate 0x1 and r1 candidate `[sp-20]` (`beq` at 0x3a864, `bl` at 0x3a878, `bl` at
  0x3a884). Nonzero fall-through copies the first call result to sp -0x1c and jumps to the final
  join (`str` at 0x3a86c, `b` at 0x3a870). The physical cycle sequence and timing unit are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a848 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3a84c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3a858 | store | 4 | `sp -0x18` | `ret(0x3a458)` |
| `ldr` at 0x3a85c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a868 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3a86c | store | 4 | `sp -0x1c` | `ret(0x3a458)` |
| `ldr` at 0x3a880 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3a88c | store | 4 | `sp -0x18` | `ret(0x39a3c)` |
| `ldr` at 0x3a890 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3a894 | store | 4 | `sp -0x1c` | `ret(0x39a3c)` |
| `ldr` at 0x3a898 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3a89c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3a89c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3a89c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a850 | `(unknown callee) 0x3a458` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3a878 | `(unknown callee) 0x3e294` | `0xc8` | `?` | `?` | `ret(0x3a458)` |
| `bl` at 0x3a884 | `(unknown callee) 0x39a3c` | `0x1` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3a864 | `ret(0x3a458) == 0x0` | `ret(0x3a458) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3a89c | `r0` | `?` |

## `powerMarginStatusAll`

- `powerMarginStatusAll`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a8a0 to 0x3a8c0.
  Established. A word load from 0x1155e0 supplies the return expression `[0x1155e0]` (`ldr` at
  0x3a8b0, `ldmdb` at 0x3a8b8). The body alone does not establish the word's margin-status encoding.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3a8ac | load | 4 | `abs 0x3a8bc` | `-` |
| `ldr` at 0x3a8b0 | load | 4 | `abs 0x1155e0` | `-` |
| `ldmdb` at 0x3a8b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3a8b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3a8b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3a8b8 | `r0` | `[0x1155e0]` |

## `powerStatusAll`

- `powerStatusAll`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a8c0 to 0x3aa60.
  Established. Three word stores initialize sp -0x14, sp -0x18 and sp -0x1c to 0x0 (`str` at
  0x3a8d4, `str` at 0x3a8dc, `str` at 0x3a8e4). The loop's `blt` has unknown operands (`blt` at
  0x3a8f8). A later `bgt` takes the positive-result path after the call to 0x3aa60 (`bl` at 0x3a974,
  `bgt` at 0x3a980). The call to 0x3dba8 is followed by `bne`, whose equal fall-through reaches the
  word store of `[sp-24]+1` at sp -0x18 (`bl` at 0x3a9cc, `bne` at 0x3a9dc, `str` at 0x3a9e8). The
  return retains `[sp-44]`, and the physical status aggregation remains unknown (`ldmdb` at
  0x3aa50).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a8d4 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x3a8dc | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x3a8e4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x3a8e8 | load | 4 | `abs 0x3aa54` | `-` |
| `ldr` at 0x3a8ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a8f0 | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x3a900 | load | 4 | `abs 0x3aa58` | `-` |
| `ldr` at 0x3a904 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a918 | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x3a920 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3a924 | load | 4 | `abs 0x3aa5c` | `-` |
| `ldr` at 0x3a928 | load | 4 | `abs 0x2810e0` | `-` |
| `ldr` at 0x3a92c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3a934 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a940 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3a944 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a950 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3a954 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a958 | load | 4 | `abs 0x3aa54` | `-` |
| `ldr` at 0x3a95c | load | 4 | `abs 0x281100` | `-` |
| `ldr` at 0x3a968 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3a96c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a988 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a990 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3a994 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3a998 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a9a8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3a9ac | load | 1 | `[sp-40] +0x3` | `-` |
| `str` at 0x3a9c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3a9c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3a9d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3a9e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3a9e8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x3a9ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3a9f4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x3a9fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3aa08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3aa0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3aa1c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x3aa24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3aa34 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x3aa40 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3aa44 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3aa48 | store | 4 | `sp -0x2c` | `[sp-48]` |
| `ldr` at 0x3aa4c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3aa50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3aa50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3aa50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a974 | `(unknown callee) 0x3aa60` | `?` | `0x281120` | `?` | `?` |
| `bl` at 0x3a9a0 | `(unknown callee) 0xb7b78` | `?` | `?` | `?` | `?` |
| `bl` at 0x3a9cc | `(unknown callee) 0x3dba8` | `[sp-40]` | `?` | `ret(0xb7b78)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3a8f8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3a93c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3a94c | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x3a964 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0x3a980 | `ret(0x3aa60) > 0x0 (signed, 32-bit)` | `ret(0x3aa60) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x3a9dc | `[sp-32] != ret(0x3dba8)` | `[sp-32] == ret(0x3dba8)` |
| `beq` at 0x3aa04 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3aa14 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3aa2c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3aa50 | `r0` | `[sp-44]` |

## `powerStatus`

- `powerStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3aa60 to 0x3ab58.
  Established. The first `bne` takes the nonzero-result path after 0x399d0. Zero fall-through saves
  0xfffff9b9 at sp -0x20 (`bl` at 0x3aa78, `bne` at 0x3aa84, `str` at 0x3aa8c). A later `bne` takes
  the path to 0xb7b78 when `[[sp-24]+2]:1 != 0xff`. Equality falls through to a save of
  0xfffff9b8 (`bne` at 0x3aac4, `bl` at 0x3aae0, `str` at 0x3aacc). The call to 0x3dba8 has r0
  candidate `[sp-24]` (`bl` at 0x3ab00); its following `beq` takes the zero-result path into a
  separate result-producing arm, whose stored value is unknown (`beq` at 0x3ab0c, `str` at 0x3ab38).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3aa70 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3aa74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aa88 | load | 4 | `abs 0x3ab4c` | `-` |
| `str` at 0x3aa8c | store | 4 | `sp -0x20` | `0xfffff9b9` |
| `ldr` at 0x3aa94 | load | 4 | `abs 0x3ab50` | `-` |
| `ldr` at 0x3aa98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aaac | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x3aab4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3aab8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3aabc | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x3aac8 | load | 4 | `abs 0x3ab54` | `-` |
| `str` at 0x3aacc | store | 4 | `sp -0x20` | `0xfffff9b8` |
| `ldr` at 0x3aad4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3aad8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x3aae8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3aaec | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x3aaf8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3aafc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ab10 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ab20 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3ab28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ab38 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3ab3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ab40 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x3ab44 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x3ab48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ab48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ab48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3aa78 | `(unknown callee) 0x399d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3aae0 | `(unknown callee) 0xb7b78` | `[[sp-24]+2]:1` | `0x281120` | `?` | `[[sp-24]+2]:1` |
| `bl` at 0x3ab00 | `(unknown callee) 0x3dba8` | `[sp-24]` | `?` | `ret(0xb7b78)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3aa84 | `ret(0x399d0) != 0x0` | `ret(0x399d0) == 0x0` |
| `bne` at 0x3aac4 | `[[sp-24]+2]:1 != 0xff` | `[[sp-24]+2]:1 == 0xff` |
| `beq` at 0x3ab0c | `ret(0x3dba8) == 0x0` | `ret(0x3dba8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ab48 | `r0` | `?` |

## `powerTempMargin`

- `powerTempMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b134 to 0x3b1c4.
  Established. The call to 0x3dae8 precedes `bne`, whose taken nonzero-result path reaches the
  increment store `[0x119dd8]+1` at 0x119dd8 (`bl` at 0x3b154, `bne` at 0x3b168, `str` at 0x3b18c).
  The next `bne` skips the call to 0x6bab8 when that incremented expression is not equal to 0x1 (`bne`
  at 0x3b194, `bl` at 0x3b19c); its equal fall-through calls 0x6bab8 with first candidate 0x1 (`bl`
  at 0x3b19c). The later call to 0x3dea4 uses r0 candidate `[sp-28]` and r1 candidate `[sp-21]:1`
  (`bl` at 0x3b1ac). The margin and counter meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b144 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b14c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3b150 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3b15c | store | 4 | `sp -0x1c` | `ret(0x3dae8)` |
| `ldr` at 0x3b160 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b174 | store | 4 | `sp -0x20` | `ret(0x3db60)` |
| `ldr` at 0x3b17c | load | 4 | `abs 0x3b1c0` | `-` |
| `ldr` at 0x3b180 | load | 4 | `abs 0x3b1c0` | `-` |
| `ldr` at 0x3b184 | load | 4 | `abs 0x119dd8` | `-` |
| `str` at 0x3b18c | store | 4 | `abs 0x119dd8` | `[0x119dd8]+1` |
| `ldrb` at 0x3b1a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b1a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b1b4 | store | 4 | `sp -0x20` | `ret(0x3dea4)` |
| `ldr` at 0x3b1b8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x3b1bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b1bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b1bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b154 | `(unknown callee) 0x3dae8` | `r0` | `r1` | `r2` | `r1` |
| `bl` at 0x3b16c | `(unknown callee) 0x3db60` | `ret(0x3dae8)` | `?` | `?` | `ret(0x3dae8)` |
| `bl` at 0x3b19c | `(unknown callee) 0x6bab8` | `0x1` | `?` | `0x119dd8` | `[0x119dd8]+1` |
| `bl` at 0x3b1ac | `(unknown callee) 0x3dea4` | `[sp-28]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3b168 | `ret(0x3dae8) != 0x0` | `ret(0x3dae8) == 0x0` |
| `bne` at 0x3b194 | `[0x119dd8]+1 != 0x1` | `[0x119dd8]+1 == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b1bc | `r0` | `?` |

## `powerRestoreMargin`

- `powerRestoreMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b1c4 to 0x3b2bc.
  Established. After the call to 0x1c79c, `bhi` takes the unsigned `ret(0x1c79c)-1 > 0x3` path
  to 0x3d8ac. Fall-through reaches another call to 0x1c79c (`bl` at 0x3b214, `bhi` at 0x3b224,
  `bl` at 0x3b24c, `bl` at 0x3b238). The resulting byte saves at sp -0x19 contain the masked result
  of their respective callees (`strb` at 0x3b240, `strb` at 0x3b254). A later word store writes
  `[0x119dd8]-1` at 0x119dd8 (`str` at 0x3b280). `bgt` takes its positive signed path past the
  word-clear and the call to 0x6bab8 with r0 candidate 0x0 (`bgt` at 0x3b288, `str` at 0x3b29c, `bl`
  at 0x3b2a4). The restoration semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b1d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3b1d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3b1e4 | store | 4 | `sp -0x18` | `ret(0x3dae8)` |
| `ldr` at 0x3b1e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3b1fc | store | 4 | `sp -0x24` | `ret(0x3db60)` |
| `ldr` at 0x3b204 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3b208 | load | 1 | `ret(0x3dae8) +0x10` | `-` |
| `ldr` at 0x3b228 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3b22c | load | 1 | `[sp-24] +0x10` | `-` |
| `strb` at 0x3b240 | store | 1 | `sp -0x19` | `(ret(0x1c79c) & 0xff)` |
| `ldr` at 0x3b248 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3b254 | store | 1 | `sp -0x19` | `(ret(0x3d8ac) & 0xff)` |
| `ldrb` at 0x3b258 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3b25c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3b26c | store | 4 | `sp -0x20` | `ret(0x3dea4)` |
| `ldr` at 0x3b270 | load | 4 | `abs 0x3b2b8` | `-` |
| `ldr` at 0x3b274 | load | 4 | `abs 0x3b2b8` | `-` |
| `ldr` at 0x3b278 | load | 4 | `abs 0x119dd8` | `-` |
| `str` at 0x3b280 | store | 4 | `abs 0x119dd8` | `[0x119dd8]-1` |
| `ldr` at 0x3b294 | load | 4 | `abs 0x3b2b8` | `-` |
| `str` at 0x3b29c | store | 4 | `abs 0x119dd8` | `0x0` |
| `ldr` at 0x3b2a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3b2ac | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x3b2b0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x3b2b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b2b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b2b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b1dc | `(unknown callee) 0x3dae8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3b1f4 | `(unknown callee) 0x3db60` | `ret(0x3dae8)` | `?` | `?` | `ret(0x3dae8)` |
| `bl` at 0x3b214 | `(unknown callee) 0x1c79c` | `[ret(0x3dae8)+16]:1+20` | `?` | `?` | `[ret(0x3dae8)+16]:1+20` |
| `bl` at 0x3b238 | `(unknown callee) 0x1c79c` | `[[sp-24]+16]:1+130` | `?` | `?` | `[[sp-24]+16]:1+130` |
| `bl` at 0x3b24c | `(unknown callee) 0x3d8ac` | `[sp-24]` | `?` | `?` | `ret(0x1c79c)-1` |
| `bl` at 0x3b264 | `(unknown callee) 0x3dea4` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x3b290 | `(unknown callee) 0x3e294` | `0x64` | `?` | `0x119dd8` | `[0x119dd8]-1` |
| `bl` at 0x3b2a4 | `(unknown callee) 0x6bab8` | `0x0` | `?` | `0x119dd8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3b1f0 | `ret(0x3dae8) != 0x0` | `ret(0x3dae8) == 0x0` |
| `bhi` at 0x3b224 | `ret(0x1c79c)-1 > 0x3 (unsigned, 32-bit)` | `ret(0x1c79c)-1 <= 0x3 (unsigned, 32-bit)` |
| `bgt` at 0x3b288 | `[0x119dd8]-1 > 0x0 (signed, 32-bit)` | `[0x119dd8]-1 <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b2b4 | `r0` | `?` |

## `powerMarginAll`

- `powerMarginAll`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b2bc to 0x3b31c.
  Established. The call to 0x3b31c has first candidates 0x1a, entry r0 and 0x0 (`bl` at 0x3b2dc).
  `bne` takes the nonzero-result path past the global store and second call (`bne` at 0x3b2f0, `str`
  at 0x3b2fc, `bl` at 0x3b304). Equal fall-through writes `[sp-20]` as a word at 0x1155e0, then
  calls 0x5ee50 with that expression as its first candidate (`str` at 0x3b2fc, `bl` at 0x3b304). The
  margin selector and notification meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b2cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3b2d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b2d8 | load | 4 | `abs 0x3b314` | `-` |
| `str` at 0x3b2e4 | store | 4 | `sp -0x18` | `ret(0x3b31c)` |
| `ldr` at 0x3b2e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b2f4 | load | 4 | `abs 0x3b318` | `-` |
| `ldr` at 0x3b2f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3b2fc | store | 4 | `abs 0x1155e0` | `[sp-20]` |
| `ldr` at 0x3b308 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3b310 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b310 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b310 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b2dc | `(unknown callee) 0x3b31c` | `0x1a` | `r0` | `0x0` | `r3` |
| `bl` at 0x3b304 | `(unknown callee) 0x5ee50` | `[sp-20]` | `?` | `0x1155e0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3b2f0 | `ret(0x3b31c) != 0x0` | `ret(0x3b31c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b310 | `r0` | `?` |

## `powerResetAgain`

- `powerResetAgain`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dca8 to 0x3dd10.
  Established. The word read at 0x119ddc decides the first `beq`, whose taken zero path reaches the
  zero-result save (`ldr` at 0x3dcbc, `beq` at 0x3dcc4, `str` at 0x3dcfc). With nonzero
  fall-through, a second `beq` takes the zero-word path from 0x27ee88 to the same save (`ldr` at
  0x3dccc, `beq` at 0x3dcd4, `str` at 0x3dcfc); its nonzero path reloads 0x27ee88 and reaches an
  unresolved computed transfer (`ldr` at 0x3dcdc, `mov` at 0x3dce8). The known return carries 0x0,
  and additional effects of that transfer remain unknown (`ldmdb` at 0x3dd04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3dcb8 | load | 4 | `abs 0x3dd08` | `-` |
| `ldr` at 0x3dcbc | load | 4 | `abs 0x119ddc` | `-` |
| `ldr` at 0x3dcc8 | load | 4 | `abs 0x3dd0c` | `-` |
| `ldr` at 0x3dccc | load | 4 | `abs 0x27ee88` | `-` |
| `ldr` at 0x3dcd8 | load | 4 | `abs 0x3dd0c` | `-` |
| `ldr` at 0x3dcdc | load | 4 | `abs 0x27ee88` | `-` |
| `str` at 0x3dcfc | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x3dd00 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3dd04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dd04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dd04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3dcc4 | `[0x119ddc] == 0x0` | `[0x119ddc] != 0x0` |
| `beq` at 0x3dcd4 | `[0x27ee88] == 0x0` | `[0x27ee88] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dd04 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0x3dce8).

## `powerResetStart`

- `powerResetStart`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dd10 to 0x3dd30.
  Established. The four-byte store at 0x119dd8 writes 0x0 (`str` at 0x3dd24). The return carries
  entry r0 (`ldmdb` at 0x3dd28). This clear alone does not establish a physical reset-start effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3dd1c | load | 4 | `abs 0x3dd2c` | `-` |
| `str` at 0x3dd24 | store | 4 | `abs 0x119dd8` | `0x0` |
| `ldmdb` at 0x3dd28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dd28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dd28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dd28 | `r0` | `r0` |

## `powerResetDone`

- `powerResetDone`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dd30 to 0x3dda4.
  Established. A word store clears 0x119ddc before testing the word at 0x119dd8 (`str` at 0x3dd48,
  `ldr` at 0x3dd50). `beq` takes the zero-word path directly to the return (`beq` at 0x3dd58,
  `ldmdb` at 0x3dd8c). Nonzero fall-through calls 0x1eed0, clears 0x119dd8 and calls 0x6bab8 with
  first candidate 0x0 (`bl` at 0x3dd74, `str` at 0x3dd80, `bl` at 0x3dd88). The globals'
  reset-completion roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3dd40 | load | 4 | `abs 0x3dd90` | `-` |
| `str` at 0x3dd48 | store | 4 | `abs 0x119ddc` | `0x0` |
| `ldr` at 0x3dd4c | load | 4 | `abs 0x3dd94` | `-` |
| `ldr` at 0x3dd50 | load | 4 | `abs 0x119dd8` | `-` |
| `ldr` at 0x3dd5c | load | 4 | `abs 0x3dd98` | `-` |
| `str` at 0x3dd60 | store | 4 | `sp -0x14` | `0xe1be8` |
| `ldr` at 0x3dd68 | load | 4 | `abs 0x3dd9c` | `-` |
| `ldr` at 0x3dd6c | load | 4 | `abs 0x3dda0` | `-` |
| `ldr` at 0x3dd78 | load | 4 | `abs 0x3dd94` | `-` |
| `str` at 0x3dd80 | store | 4 | `abs 0x119dd8` | `0x0` |
| `ldmdb` at 0x3dd8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dd8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dd8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3dd74 | `(unknown callee) 0x1eed0` | `0x4` | `0xe1584` | `0x9af` | `0x0` |
| `bl` at 0x3dd88 | `(unknown callee) 0x6bab8` | `0x0` | `?` | `0x119dd8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3dd58 | `[0x119dd8] == 0x0` | `[0x119dd8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dd8c | `r0` | `?` |
