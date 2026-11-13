# ARM flash access and image selection in 1.40.5

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
These routines have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/flash-control-facts.txt) retain all tool sections.

## `brickGetFlashType`

- `brickGetFlashType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f568 to 0x4f588.
  Established. The byte read at absolute `0x148f7d` supplies the printed return value `[0x148f7d]:1`
  (`ldrb` at 0x4f578, `ldmdb` at 0x4f580). The byte encoding and its relationship to a device remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4f574 | load | 4 | `abs 0x4f584` | `-` |
| `ldrb` at 0x4f578 | load | 1 | `abs 0x148f7d` | `-` |
| `ldmdb` at 0x4f580 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f580 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f580 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f580 | `r0` | `[0x148f7d]:1` |

## `FlashTask`

- `FlashTask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5ad0 to 0xa5ba4.
  Established. The unnamed `0xb5f90` call has `sp-32`, `0x2b27ac`, unknown, `sp-32` (`bl` at
  0xa5af8). A later four-byte absolute assignment at `0x14a6f4` writes `[0x14a6f4]-1` (`str` at
  0xa5b34). The branch takes `[[sp-32]+22]:1 != 0x0` to `0xa5b84` (`bne` at 0xa5b48);
  its fall-through path includes the unnamed `0xa5bcc` call with `[sp-32]`, unknown, unknown,
  `[[sp-32]+22]:1` (`bl` at 0xa5b50). Task messages and the counter meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa5ae0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa5ae4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa5af0 | load | 4 | `abs 0xa5b90` | `-` |
| `str` at 0xa5b00 | store | 4 | `sp -0x1c` | `ret(0xb5f90)` |
| `ldr` at 0xa5b04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5b14 | load | 4 | `abs 0xa5b94` | `-` |
| `ldr` at 0xa5b18 | load | 4 | `abs 0x14a6f4` | `-` |
| `ldr` at 0xa5b24 | load | 4 | `abs 0xa5b94` | `-` |
| `ldr` at 0xa5b28 | load | 4 | `abs 0xa5b94` | `-` |
| `ldr` at 0xa5b2c | load | 4 | `abs 0x14a6f4` | `-` |
| `str` at 0xa5b34 | store | 4 | `abs 0x14a6f4` | `[0x14a6f4]-1` |
| `ldr` at 0xa5b3c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xa5b40 | load | 1 | `[sp-32] +0x16` | `-` |
| `ldr` at 0xa5b4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa5b58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa5b64 | load | 4 | `abs 0xa5b98` | `-` |
| `str` at 0xa5b68 | store | 4 | `sp -0x28` | `0x126dd8` |
| `str` at 0xa5b6c | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xa5b74 | load | 4 | `abs 0xa5b9c` | `-` |
| `ldr` at 0xa5b78 | load | 4 | `abs 0xa5ba0` | `-` |
| `ldr` at 0xa5b84 | load | 4 | `sp -0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5af8 | `(unknown callee) 0xb5f90` | `sp-32` | `0x2b27ac` | `?` | `sp-32` |
| `bl` at 0xa5b50 | `(unknown callee) 0xa5bcc` | `[sp-32]` | `?` | `?` | `[[sp-32]+22]:1` |
| `bl` at 0xa5b5c | `(unknown callee) 0x21704` | `ret(0xb5f90)` | `?` | `?` | `ret(0xb5f90)` |
| `bl` at 0xa5b80 | `(unknown callee) 0x21eb8` | `0x3` | `0x126d64` | `0xc0e` | `0x0` |
| `bl` at 0xa5b88 | `(unknown callee) 0xb5fec` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa5b0c | `ret(0xb5f90) == 0x0` | `ret(0xb5f90) != 0x0` |
| `beq` at 0xa5b20 | `[0x14a6f4] == 0x0` | `[0x14a6f4] != 0x0` |
| `bne` at 0xa5b48 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `flash_init`

- `flash_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb804 to 0xcb924.
  Established. The branch takes `[0x2adf50] == 0x0` to `0xcb834` (`beq` at 0xcb824). The unnamed
  `0x1b460` call carries `0x124f0e5`, `0x1`, `0x3b6`, `[0x143790]`, followed by a four-byte
  assignment of its result at absolute `0x143790` (`bl` at 0xcb880, `str` at 0xcb88c). An included
  later four-byte absolute store at `0x2adf50` writes `0x1` (`str` at 0xcb8f8). Initialization and
  descriptor meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcb814 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xcb818 | load | 4 | `abs 0xcb90c` | `-` |
| `ldr` at 0xcb81c | load | 4 | `abs 0x2adf50` | `-` |
| `str` at 0xcb82c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xcb838 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xcb83c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb84c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcb854 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcb85c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xcb864 | load | 4 | `abs 0xcb910` | `-` |
| `ldr` at 0xcb868 | load | 4 | `abs 0x143790` | `-` |
| `ldr` at 0xcb874 | load | 4 | `abs 0xcb914` | `-` |
| `ldr` at 0xcb87c | load | 4 | `abs 0xcb918` | `-` |
| `ldr` at 0xcb888 | load | 4 | `abs 0xcb910` | `-` |
| `str` at 0xcb88c | store | 4 | `abs 0x143790` | `ret(0x1b460)` |
| `ldr` at 0xcb890 | load | 4 | `abs 0x143790` | `-` |
| `ldr` at 0xcb8b0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcb8bc | load | 4 | `abs 0xcb91c` | `-` |
| `str` at 0xcb8c0 | store | 4 | `sp -0x2c` | `0x1344c0` |
| `ldr` at 0xcb8c4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xcb8c8 | store | 4 | `sp -0x28` | `[ret(0x1b148)]` |
| `str` at 0xcb8cc | store | 4 | `sp -0x24` | `ret(0x1b124)` |
| `ldr` at 0xcb8d4 | load | 4 | `abs 0xcb920` | `-` |
| `str` at 0xcb8e8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcb8f0 | load | 4 | `abs 0xcb90c` | `-` |
| `str` at 0xcb8f8 | store | 4 | `abs 0x2adf50` | `0x1` |
| `str` at 0xcb900 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcb904 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcb908 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcb908 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb908 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb908 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb850 | `(unknown callee) 0xcb36c` | `?` | `?` | `?` | `?` |
| `bl` at 0xcb880 | `(unknown callee) 0x1b460` | `0x124f0e5` | `0x1` | `0x3b6` | `[0x143790]` |
| `bl` at 0xcb89c | `(unknown callee) 0x1b148` | `ret(0x1b460)` | `?` | `ret(0x1b460)` | `[0x143790]` |
| `bl` at 0xcb8a8 | `(unknown callee) 0x1b148` | `ret(0x1b148)` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcb8b4 | `(unknown callee) 0x1b124` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcb8e0 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0xd9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcb824 | `[0x2adf50] == 0x0` | `[0x2adf50] != 0x0` |
| `ble` at 0xcb844 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xcb870 | `[0x143790] >= 0x0 (signed, 32-bit)` | `[0x143790] < 0x0 (signed, 32-bit)` |
| `bge` at 0xcb898 | `[0x143790] >= 0x0 (signed, 32-bit)` | `[0x143790] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb908 | `r0` | `?` |

## `flash_lock`

- `flash_lock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb924 to 0xcb9d8.
  Established. A signed negative `[0x143790]` takes the branch to `0xcb9b8` (`blt` at 0xcb940). The
  unnamed `0x1b4a8` call carries `[0x143790]`, `0x143778`, `0x2`, `0x143790` (`bl` at 0xcb954).
  Included four-byte stack assignments at `sp -0x24` and `sp -0x20` retain `0x134504` and
  `[ret(0x1b148)]` (`str` at 0xcb988, `str` at 0xcb990). Locking effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xcb934 | load | 4 | `abs 0xcb9c8` | `-` |
| `ldr` at 0xcb938 | load | 4 | `abs 0x143790` | `-` |
| `ldr` at 0xcb944 | load | 4 | `abs 0xcb9c8` | `-` |
| `ldr` at 0xcb948 | load | 4 | `abs 0x143790` | `-` |
| `ldr` at 0xcb94c | load | 4 | `abs 0xcb9cc` | `-` |
| `ldr` at 0xcb978 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcb984 | load | 4 | `abs 0xcb9d0` | `-` |
| `str` at 0xcb988 | store | 4 | `sp -0x24` | `0x134504` |
| `ldr` at 0xcb98c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xcb990 | store | 4 | `sp -0x20` | `[ret(0x1b148)]` |
| `str` at 0xcb994 | store | 4 | `sp -0x1c` | `ret(0x1b124)` |
| `ldr` at 0xcb99c | load | 4 | `abs 0xcb9d4` | `-` |
| `str` at 0xcb9b0 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xcb9bc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xcb9c0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xcb9c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcb9c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb9c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb9c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcb954 | `(unknown callee) 0x1b4a8` | `[0x143790]` | `0x143778` | `0x2` | `0x143790` |
| `bl` at 0xcb964 | `(unknown callee) 0x1b148` | `ret(0x1b4a8)` | `?` | `?` | `ret(0x1b4a8)` |
| `bl` at 0xcb970 | `(unknown callee) 0x1b148` | `ret(0x1b148)` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcb97c | `(unknown callee) 0x1b124` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcb9a8 | `(unknown callee) 0x21eb8` | `0x4` | `0x1343a0` | `0xea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcb940 | `[0x143790] < 0x0 (signed, 32-bit)` | `[0x143790] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xcb960 | `ret(0x1b4a8) >= 0x0 (signed, 32-bit)` | `ret(0x1b4a8) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb9c4 | `r0` | `?` |

## `flash_unlock`

- `flash_unlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcb9d8 to 0xcba8c.
  Established. The unnamed `0x1b4a8` call uses `[0x143790]`, `0x143788`, `0x1`, `0x143790` (`bl` at
  0xcba08); its nonnegative signed result takes the branch to `0xcba6c` (`bge` at 0xcba14). Included
  four-byte stores at `sp -0x24` and `sp -0x1c` retain `0x134540` and `ret(0x1b124)` (`str` at
  0xcba3c, `str` at 0xcba48). Release effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xcb9e8 | load | 4 | `abs 0xcba7c` | `-` |
| `ldr` at 0xcb9ec | load | 4 | `abs 0x143790` | `-` |
| `ldr` at 0xcb9f8 | load | 4 | `abs 0xcba7c` | `-` |
| `ldr` at 0xcb9fc | load | 4 | `abs 0x143790` | `-` |
| `ldr` at 0xcba00 | load | 4 | `abs 0xcba80` | `-` |
| `ldr` at 0xcba2c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcba38 | load | 4 | `abs 0xcba84` | `-` |
| `str` at 0xcba3c | store | 4 | `sp -0x24` | `0x134540` |
| `ldr` at 0xcba40 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xcba44 | store | 4 | `sp -0x20` | `[ret(0x1b148)]` |
| `str` at 0xcba48 | store | 4 | `sp -0x1c` | `ret(0x1b124)` |
| `ldr` at 0xcba50 | load | 4 | `abs 0xcba88` | `-` |
| `str` at 0xcba64 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xcba70 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xcba74 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xcba78 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcba78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcba78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcba78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcba08 | `(unknown callee) 0x1b4a8` | `[0x143790]` | `0x143788` | `0x1` | `0x143790` |
| `bl` at 0xcba18 | `(unknown callee) 0x1b148` | `ret(0x1b4a8)` | `?` | `?` | `ret(0x1b4a8)` |
| `bl` at 0xcba24 | `(unknown callee) 0x1b148` | `ret(0x1b148)` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcba30 | `(unknown callee) 0x1b124` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcba5c | `(unknown callee) 0x21eb8` | `0x4` | `0x1343a0` | `0xf5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcb9f4 | `[0x143790] < 0x0 (signed, 32-bit)` | `[0x143790] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xcba14 | `ret(0x1b4a8) >= 0x0 (signed, 32-bit)` | `ret(0x1b4a8) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcba78 | `r0` | `?` |

## `flash_addr_to_index`

- `flash_addr_to_index`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcba8c to 0xcbb24.
  Established. The unnamed `0xcb804` call carries `0x0`, `r1`, `r2`, `[0x2adf50]` (`bl` at 0xcbab4).
  A signed `[sp-24] > 0x2` takes the branch to `0xcbb04`, while the other arm jumps to the four-byte
  assignment of `[sp-24]` at `sp -0x20` (`bgt` at 0xcbafc, `b` at 0xcbb00, `str` at 0xcbb14). Index
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcba9c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcbaa0 | load | 4 | `abs 0xcbb20` | `-` |
| `ldr` at 0xcbaa4 | load | 4 | `abs 0x2adf50` | `-` |
| `str` at 0xcbabc | store | 4 | `sp -0x1c` | `ret(0xcb804)` |
| `ldr` at 0xcbac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcbacc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcbad0 | store | 4 | `sp -0x20` | `ret(0xcb804)` |
| `ldr` at 0xcbad8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbae4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xcbae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcbaf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcbb08 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbb10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcbb14 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xcbb18 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcbb1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbb1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbb1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbab4 | `(unknown callee) 0xcb804` | `0x0` | `r1` | `r2` | `[0x2adf50]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcbaac | `[0x2adf50] != 0x0` | `[0x2adf50] == 0x0` |
| `bge` at 0xcbac8 | `ret(0xcb804) >= 0x0 (signed, 32-bit)` | `ret(0xcb804) < 0x0 (signed, 32-bit)` |
| `blt` at 0xcbaf0 | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bgt` at 0xcbafc | `[sp-24] > 0x2 (signed, 32-bit)` | `[sp-24] <= 0x2 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbb1c | `r0` | `[sp-32]` |

## `flash_offset`

- `flash_offset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbb24 to 0xcbbb0.
  Established. The unnamed `0xcba8c` call receives `r0`, `r1`, `r2`, `r3` (`bl` at 0xcbb3c).
  A nonnegative signed result takes the branch to `0xcbb60` (`bge` at 0xcbb50). Two four-byte
  reads have unknown effective targets (`ldr` at 0xcbb70, `ldr` at 0xcbb94). A later four-byte stack store at
  `sp -0x20` has unknown value (`str` at 0xcbb9c). Address translation meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbb34 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcbb38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbb44 | store | 4 | `sp -0x1c` | `ret(0xcba8c)` |
| `ldr` at 0xcbb48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcbb58 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbb60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbb68 | load | 4 | `abs 0xcbba8` | `-` |
| `ldr` at 0xcbb6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcbb70 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcbb80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbb88 | load | 4 | `abs 0xcbbac` | `-` |
| `ldr` at 0xcbb8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcbb90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbb94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcbb9c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbba0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcbba4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbba4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbba4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbb3c | `(unknown callee) 0xcba8c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcbb50 | `ret(0xcba8c) >= 0x0 (signed, 32-bit)` | `ret(0xcba8c) < 0x0 (signed, 32-bit)` |
| `blo` at 0xcbb78 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbba4 | `r0` | `[sp-32]` |

## `flash_read_addr`

- `flash_read_addr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbbb0 to 0xcbcc4.
  Established. The unnamed `0xfc570` call has unknown, `sp-112`, `sp-112`, `[sp-120]` (`bl` at
  0xcbc1c); its nonnegative signed result takes the branch to `0xcbc74` (`bge` at 0xcbc28). Included
  four-byte stack stores at `sp -0x88` and `sp -0x84` retain `0x134580` and `[sp-120]` (`str` at
  0xcbc44, `str` at 0xcbc4c). Mapping limits and device effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbbc0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcbbc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbbd0 | store | 4 | `sp -0x78` | `ret(0xcba8c)` |
| `ldr` at 0xcbbd4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xcbbe4 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcbbec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbbf8 | store | 4 | `sp -0x74` | `ret(0xcbb24)` |
| `ldr` at 0xcbbfc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xcbc08 | load | 4 | `abs 0xcbcb0` | `-` |
| `ldr` at 0xcbc0c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xcbc14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcbc34 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcbc40 | load | 4 | `abs 0xcbcb4` | `-` |
| `str` at 0xcbc44 | store | 4 | `sp -0x88` | `0x134580` |
| `ldr` at 0xcbc48 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xcbc4c | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0xcbc50 | store | 4 | `sp -0x80` | `ret(0x21704)` |
| `ldr` at 0xcbc58 | load | 4 | `abs 0xcbcb8` | `-` |
| `ldr` at 0xcbc5c | load | 4 | `abs 0xcbcbc` | `-` |
| `str` at 0xcbc6c | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcbc74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xcbc78 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xcbc88 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcbc90 | load | 4 | `abs 0xcbcc0` | `-` |
| `ldr` at 0xcbc94 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xcbc98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcbc9c | load | 4 | `sp -0x74` | `-` |
| `str` at 0xcbca4 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xcbca8 | load | 4 | `sp -0x7c` | `-` |
| `ldmdb` at 0xcbcac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbcac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbcac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbbc8 | `(unknown callee) 0xcba8c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xcbbf0 | `(unknown callee) 0xcbb24` | `[sp-20]` | `?` | `?` | `ret(0xcba8c)` |
| `bl` at 0xcbc1c | `(unknown callee) 0xfc570` | `?` | `sp-112` | `sp-112` | `[sp-120]` |
| `bl` at 0xcbc2c | `(unknown callee) 0x1b148` | `ret(0xfc570)` | `?` | `?` | `ret(0xfc570)` |
| `bl` at 0xcbc38 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcbc64 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x12d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcbbdc | `ret(0xcba8c) >= 0x0 (signed, 32-bit)` | `ret(0xcba8c) < 0x0 (signed, 32-bit)` |
| `ble` at 0xcbc04 | `[sp-120] <= 0x0 (signed, 32-bit)` | `[sp-120] > 0x0 (signed, 32-bit)` |
| `bge` at 0xcbc28 | `ret(0xfc570) >= 0x0 (signed, 32-bit)` | `ret(0xfc570) < 0x0 (signed, 32-bit)` |
| `bhi` at 0xcbc80 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbcac | `r0` | `[sp-124]` |

## `flash_write_addr`

- `flash_write_addr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbcc4 to 0xcbd60.
  Established. A four-byte read at absolute `0x2b4430` precedes the unnamed `0xcba8c` call with
  `[0x2b4430]`, unknown, unknown, `0x2b4420` (`ldr` at 0xcbd14, `bl` at 0xcbd18). The branch takes
  `[sp-28] != ret(0xcba8c)` to `0xcbd38` (`bne` at 0xcbd28). A later four-byte store at `sp -0x20`
  has unknown value (`str` at 0xcbd4c). Writable-region interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbcd4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcbcd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbce4 | store | 4 | `sp -0x1c` | `ret(0xcba8c)` |
| `ldr` at 0xcbce8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcbcf8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbd00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbd0c | store | 4 | `sp -0x18` | `ret(0xcbb24)` |
| `ldr` at 0xcbd10 | load | 4 | `abs 0xcbd58` | `-` |
| `ldr` at 0xcbd14 | load | 4 | `abs 0x2b4430` | `-` |
| `ldr` at 0xcbd20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcbd30 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbd38 | load | 4 | `abs 0xcbd5c` | `-` |
| `ldr` at 0xcbd3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcbd40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcbd44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcbd4c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcbd50 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcbd54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbd54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbd54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbcdc | `(unknown callee) 0xcba8c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xcbd04 | `(unknown callee) 0xcbb24` | `[sp-20]` | `?` | `?` | `ret(0xcba8c)` |
| `bl` at 0xcbd18 | `(unknown callee) 0xcba8c` | `[0x2b4430]` | `?` | `?` | `0x2b4420` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcbcf0 | `ret(0xcba8c) >= 0x0 (signed, 32-bit)` | `ret(0xcba8c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xcbd28 | `[sp-28] != ret(0xcba8c)` | `[sp-28] == ret(0xcba8c)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbd54 | `r0` | `[sp-32]` |

## `flash_read`

- `flash_read`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbd60 to 0xcbdf0.
  Established. The unnamed `0x1b394` call carries `[sp-20]`, `[sp-32]`, `[sp-28]`, unknown, followed
  by the unnamed `0xcb9d8` call with `ret(0x1b394)`, unknown, unknown, unknown (`bl` at 0xcbdd4,
  `bl` at 0xcbdd8). A later four-byte stack store at `sp -0x28` writes `0x0` (`str` at 0xcbde0).
  Buffer transfer and locking semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbd70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcbd74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcbd78 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcbd7c | load | 4 | `abs 0xcbdec` | `-` |
| `ldr` at 0xcbd80 | load | 4 | `abs 0x2adf50` | `-` |
| `str` at 0xcbd98 | store | 4 | `sp -0x24` | `ret(0xcb804)` |
| `ldr` at 0xcbd9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcbda8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcbdac | store | 4 | `sp -0x28` | `ret(0xcb804)` |
| `ldr` at 0xcbdb4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcbdc0 | store | 4 | `sp -0x20` | `ret(0xcbbb0)` |
| `ldr` at 0xcbdc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcbdcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcbdd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcbde0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xcbde4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xcbde8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbde8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbde8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbd90 | `(unknown callee) 0xcb804` | `0x0` | `r1` | `r2` | `[0x2adf50]` |
| `bl` at 0xcbdb8 | `(unknown callee) 0xcbbb0` | `?` | `?` | `?` | `?` |
| `bl` at 0xcbdc4 | `(unknown callee) 0xcb924` | `ret(0xcbbb0)` | `?` | `?` | `ret(0xcbbb0)` |
| `bl` at 0xcbdd4 | `(unknown callee) 0x1b394` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0xcbdd8 | `(unknown callee) 0xcb9d8` | `ret(0x1b394)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcbd88 | `[0x2adf50] != 0x0` | `[0x2adf50] == 0x0` |
| `bge` at 0xcbda4 | `ret(0xcb804) >= 0x0 (signed, 32-bit)` | `ret(0xcb804) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbde8 | `r0` | `?` |

## `flash_erase`

- `flash_erase`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbdf0 to 0xcbe20.
  Established. The three entry candidates are saved as four-byte stack values at `sp -0x14`,
  `sp -0x18` and `sp -0x1c` (`str` at 0xcbe00, `str` at 0xcbe04, `str` at 0xcbe08). An included
  four-byte stack store at `sp -0x24` writes `0x0`, and the printed return is `r0 = 0x0` (`str` at
  0xcbe10, `ldmdb` at 0xcbe1c). The hardware role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbe00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcbe04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcbe08 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcbe10 | store | 4 | `sp -0x24` | `0x0` |
| `ldmdb` at 0xcbe1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcbe1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcbe1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcbe1c | `r0` | `0x0` |

## `flash_program`

- `flash_program`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcbe20 to 0xcc280.
  Established. The unnamed `0x1b49c` call carries unknown, `0xc00c4d04`, `sp-92`, `[sp-40]`
  (`bl` at 0xcbf98); its zero result takes the branch to `0xcbff0` (`beq` at 0xcbfa4). Included
  four-byte stack assignments at `sp -0x2c` retain `0xfffffc7c` and, later, `0xfffffc7a` (`str` at 0xcbec4,
  `str` at 0xcc1e4). A four-byte assignment through `[sp-32] +0x0` writes `[sp-20]` (`str` at
  0xcc220). Program operations and error meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcbe30 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcbe34 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcbe38 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xcbe3c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xcbe44 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xcbe48 | load | 4 | `abs 0xcc238` | `-` |
| `ldr` at 0xcbe4c | load | 4 | `abs 0x2adf50` | `-` |
| `str` at 0xcbe64 | store | 4 | `sp -0x40` | `ret(0xcb804)` |
| `ldr` at 0xcbe68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcbe74 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcbe78 | store | 4 | `sp -0x60` | `ret(0xcb804)` |
| `ldr` at 0xcbe84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbe90 | store | 4 | `sp -0x28` | `ret(0xcba8c)` |
| `ldr` at 0xcbe94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcbea0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbeac | store | 4 | `sp -0x30` | `ret(0xcbcc4)` |
| `ldr` at 0xcbeb0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcbec0 | load | 4 | `abs 0xcc23c` | `-` |
| `str` at 0xcbec4 | store | 4 | `sp -0x2c` | `0xfffffc7c` |
| `ldr` at 0xcbecc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbed8 | store | 4 | `sp -0x34` | `ret(0xcbb24)` |
| `ldr` at 0xcbedc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcbee8 | load | 4 | `abs 0xcc240` | `-` |
| `ldr` at 0xcbeec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbef4 | load | 4 | `abs 0x2b4440` | `-` |
| `ldr` at 0xcbf08 | load | 4 | `abs 0xcc244` | `-` |
| `str` at 0xcbf0c | store | 4 | `sp -0x70` | `0x1345b8` |
| `ldr` at 0xcbf10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcbf14 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xcbf1c | load | 4 | `abs 0xcc248` | `-` |
| `ldr` at 0xcbf20 | load | 4 | `abs 0xcc24c` | `-` |
| `str` at 0xcbf30 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xcbf3c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xcbf40 | load | 4 | `abs 0xcc240` | `-` |
| `ldr` at 0xcbf44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcbf48 | load | 4 | `abs 0x2b4440` | `-` |
| `ldr` at 0xcbf50 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcbf64 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xcbf6c | store | 4 | `sp -0x58` | `0x10` |
| `str` at 0xcbf74 | store | 4 | `sp -0x54` | `sp-80` |
| `ldr` at 0xcbf78 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcbf7c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xcbf80 | load | 4 | `abs 0xcc250` | `-` |
| `ldr` at 0xcbf84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcbf8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcbf90 | load | 4 | `abs 0xcc254` | `-` |
| `ldr` at 0xcbfb0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcbfbc | load | 4 | `abs 0xcc258` | `-` |
| `str` at 0xcbfc0 | store | 4 | `sp -0x70` | `0x1345f4` |
| `ldr` at 0xcbfc4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xcbfc8 | store | 4 | `sp -0x6c` | `[sp-64]` |
| `str` at 0xcbfcc | store | 4 | `sp -0x68` | `ret(0x21704)` |
| `ldr` at 0xcbfd4 | load | 4 | `abs 0xcc248` | `-` |
| `ldr` at 0xcbfd8 | load | 4 | `abs 0xcc25c` | `-` |
| `str` at 0xcbfe8 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xcbff0 | load | 1 | `sp -0x4b` | `-` |
| `ldr` at 0xcbffc | load | 4 | `abs 0xcc240` | `-` |
| `ldr` at 0xcc000 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcc004 | load | 4 | `abs 0x2b4440` | `-` |
| `str` at 0xcc00c | store | 4 | `sp -0x34` | `([sp-52] + [0x2b4440])` |
| `ldr` at 0xcc010 | load | 4 | `abs 0xcc240` | `-` |
| `ldr` at 0xcc014 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc018 | load | 4 | `abs 0x2b4440` | `-` |
| `str` at 0xcc020 | store | 4 | `sp -0x30` | `([sp-48] + [0x2b4440])` |
| `ldr` at 0xcc024 | load | 4 | `abs 0xcc240` | `-` |
| `ldr` at 0xcc028 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xcc02c | load | 4 | `abs 0x2b4440` | `-` |
| `str` at 0xcc034 | store | 4 | `sp -0x40` | `([sp-64] + [0x2b4440])` |
| `ldr` at 0xcc03c | load | 4 | `abs 0xcc260` | `-` |
| `ldr` at 0xcc040 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcc048 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xcc068 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc074 | load | 4 | `abs 0xcc264` | `-` |
| `str` at 0xcc078 | store | 4 | `sp -0x70` | `0x134628` |
| `ldr` at 0xcc07c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc080 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xcc084 | store | 4 | `sp -0x68` | `ret(0x21704)` |
| `ldr` at 0xcc08c | load | 4 | `abs 0xcc248` | `-` |
| `ldr` at 0xcc090 | load | 4 | `abs 0xcc268` | `-` |
| `str` at 0xcc0a0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcc0a8 | load | 4 | `abs 0xcc260` | `-` |
| `ldr` at 0xcc0ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcc0b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcc0b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc0b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcc0c4 | store | 4 | `sp -0x2c` | `ret(0x1b058)` |
| `ldr` at 0xcc0c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcc0cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc0e0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc0ec | load | 4 | `abs 0xcc26c` | `-` |
| `str` at 0xcc0f0 | store | 4 | `sp -0x70` | `0x134654` |
| `ldr` at 0xcc0f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc0f8 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xcc0fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcc100 | store | 4 | `sp -0x68` | `[sp-44]` |
| `str` at 0xcc104 | store | 4 | `sp -0x64` | `ret(0x21704)` |
| `ldr` at 0xcc10c | load | 4 | `abs 0xcc248` | `-` |
| `ldr` at 0xcc110 | load | 4 | `abs 0xcc270` | `-` |
| `str` at 0xcc120 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcc128 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc12c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc14c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc158 | load | 4 | `abs 0xcc274` | `-` |
| `str` at 0xcc15c | store | 4 | `sp -0x70` | `0x134684` |
| `ldr` at 0xcc160 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc164 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xcc168 | store | 4 | `sp -0x68` | `ret(0x21704)` |
| `ldr` at 0xcc170 | load | 4 | `abs 0xcc248` | `-` |
| `ldr` at 0xcc174 | load | 4 | `abs 0xcc278` | `-` |
| `str` at 0xcc184 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xcc190 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcc194 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc198 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc1a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc1ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcc1b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc1b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcc1c0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xcc1c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcc1d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc1d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcc1dc | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xcc1e0 | load | 4 | `abs 0xcc27c` | `-` |
| `str` at 0xcc1e4 | store | 4 | `sp -0x2c` | `0xfffffc7a` |
| `ldr` at 0xcc1ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xcc1f4 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xcc204 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xcc20c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcc218 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcc21c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc220 | store | 4 | `[sp-32] +0x0` | `[sp-20]` |
| `ldr` at 0xcc228 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcc22c | store | 4 | `sp -0x60` | `[sp-44]` |
| `ldr` at 0xcc230 | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xcc234 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc234 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc234 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcbe5c | `(unknown callee) 0xcb804` | `0x0` | `r1` | `r2` | `[0x2adf50]` |
| `bl` at 0xcbe80 | `(unknown callee) 0xcb924` | `?` | `?` | `?` | `?` |
| `bl` at 0xcbe88 | `(unknown callee) 0xcba8c` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xcbea4 | `(unknown callee) 0xcbcc4` | `[sp-20]` | `?` | `?` | `ret(0xcba8c)` |
| `bl` at 0xcbed0 | `(unknown callee) 0xcbb24` | `[sp-20]` | `?` | `?` | `ret(0xcbcc4)` |
| `bl` at 0xcbef8 | `(unknown callee) 0xfb158` | `ret(0xcbb24)` | `[0x2b4440]` | `ret(0xcbb24)` | `0x2b4420` |
| `bl` at 0xcbf28 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x193` | `0x0` |
| `bl` at 0xcbf98 | `(unknown callee) 0x1b49c` | `?` | `0xc00c4d04` | `sp-92` | `[sp-40]` |
| `bl` at 0xcbfa8 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xcbfb4 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcbfe0 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x19e` | `0x0` |
| `bl` at 0xcc050 | `(unknown callee) 0x1b37c` | `?` | `?` | `0x0` | `0x1437a0` |
| `bl` at 0xcc060 | `(unknown callee) 0x1b148` | `ret(0x1b37c)` | `?` | `?` | `ret(0x1b37c)` |
| `bl` at 0xcc06c | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc098 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x1af` | `0x0` |
| `bl` at 0xcc0bc | `(unknown callee) 0x1b058` | `?` | `[sp-24]` | `[sp-28]` | `[sp-40]` |
| `bl` at 0xcc0d8 | `(unknown callee) 0x1b148` | `ret(0x1b058)` | `?` | `ret(0x1b058)` | `[sp-28]` |
| `bl` at 0xcc0e4 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc118 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x1b5` | `0x0` |
| `bl` at 0xcc134 | `(unknown callee) 0x1b244` | `[sp-48]` | `[sp-28]` | `0x6` | `[sp-28]` |
| `bl` at 0xcc144 | `(unknown callee) 0x1b148` | `ret(0x1b244)` | `?` | `?` | `ret(0x1b244)` |
| `bl` at 0xcc150 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc17c | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x1bd` | `0x0` |
| `bl` at 0xcc1fc | `(unknown callee) 0xcb9d8` | `ret(0x1b244)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xcc224 | `(unknown callee) 0xcb9d8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcbe54 | `[0x2adf50] != 0x0` | `[0x2adf50] == 0x0` |
| `bge` at 0xcbe70 | `ret(0xcb804) >= 0x0 (signed, 32-bit)` | `ret(0xcb804) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcbe9c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xcbeb8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xcbee4 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xcbf04 | `ret(0xfb158) == 0x0` | `ret(0xfb158) != 0x0` |
| `blo` at 0xcbf58 | `C == 0` | `not (C == 0)` |
| `beq` at 0xcbfa4 | `ret(0x1b49c) == 0x0` | `ret(0x1b49c) != 0x0` |
| `beq` at 0xcbff8 | `[sp-75]:1 == 0xff` | `[sp-75]:1 != 0xff` |
| `bge` at 0xcc05c | `ret(0x1b37c) >= 0x0 (signed, 32-bit)` | `ret(0x1b37c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcc0d4 | `ret(0x1b058) == [sp-28]` | `ret(0x1b058) != [sp-28]` |
| `bge` at 0xcc140 | `ret(0x1b244) >= 0x0 (signed, 32-bit)` | `ret(0x1b244) < 0x0 (signed, 32-bit)` |
| `blo` at 0xcc1a0 | `C == 0` | `not (C == 0)` |
| `beq` at 0xcc1cc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xcc214 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc234 | `r0` | `?` |

## `flash_set_boot_image`

- `flash_set_boot_image`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc280 to 0xcc4a4.
  Established. The branch takes `r0 != [0x2b4428]` to `0xcc33c` (`bne` at 0xcc32c);
  its fall-through arm includes a byte store of `0x41` at `sp -0x15` (`strb` at 0xcc334). Another
  byte assignment at the same stack address writes `0x42` (`strb` at 0xcc354). The unnamed `0x1b058` call carries
  `ret(0x1b400)`, `sp-21`, `0x1`, `sp-21` (`bl` at 0xcc404). Boot-selection semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc290 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcc294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc2a0 | load | 4 | `abs 0xcc474` | `-` |
| `ldr` at 0xcc2bc | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc2d0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc2dc | load | 4 | `abs 0xcc478` | `-` |
| `str` at 0xcc2e0 | store | 4 | `sp -0x2c` | `0x1346c4` |
| `ldr` at 0xcc2e4 | load | 4 | `abs 0xcc474` | `-` |
| `str` at 0xcc2e8 | store | 4 | `sp -0x28` | `0x1346b0` |
| `str` at 0xcc2ec | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc2f4 | load | 4 | `abs 0xcc47c` | `-` |
| `str` at 0xcc308 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xcc314 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcc31c | load | 4 | `abs 0xcc480` | `-` |
| `ldr` at 0xcc320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc324 | load | 4 | `abs 0x2b4428` | `-` |
| `strb` at 0xcc334 | store | 1 | `sp -0x15` | `0x41` |
| `ldr` at 0xcc33c | load | 4 | `abs 0xcc480` | `-` |
| `ldr` at 0xcc340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc344 | load | 4 | `abs 0x2b442c` | `-` |
| `strb` at 0xcc354 | store | 1 | `sp -0x15` | `0x42` |
| `ldr` at 0xcc35c | load | 4 | `abs 0xcc484` | `-` |
| `str` at 0xcc360 | store | 4 | `sp -0x2c` | `0x1346f0` |
| `ldr` at 0xcc364 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc368 | store | 4 | `sp -0x28` | `r0` |
| `ldr` at 0xcc370 | load | 4 | `abs 0xcc47c` | `-` |
| `ldr` at 0xcc374 | load | 4 | `abs 0xcc488` | `-` |
| `str` at 0xcc384 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc38c | load | 4 | `abs 0xcc474` | `-` |
| `ldr` at 0xcc390 | load | 4 | `abs 0xcc48c` | `-` |
| `str` at 0xcc39c | store | 4 | `sp -0x1c` | `ret(0x1b400)` |
| `ldr` at 0xcc3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc3b4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc3c0 | load | 4 | `abs 0xcc490` | `-` |
| `str` at 0xcc3c4 | store | 4 | `sp -0x2c` | `0x134718` |
| `ldr` at 0xcc3c8 | load | 4 | `abs 0xcc494` | `-` |
| `str` at 0xcc3cc | store | 4 | `sp -0x28` | `0x134744` |
| `str` at 0xcc3d0 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc3d8 | load | 4 | `abs 0xcc47c` | `-` |
| `ldr` at 0xcc3dc | load | 4 | `abs 0xcc498` | `-` |
| `str` at 0xcc3ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc3f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc41c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc428 | load | 4 | `abs 0xcc49c` | `-` |
| `str` at 0xcc42c | store | 4 | `sp -0x2c` | `0x13475c` |
| `ldr` at 0xcc430 | load | 4 | `abs 0xcc474` | `-` |
| `str` at 0xcc434 | store | 4 | `sp -0x28` | `0x1346b0` |
| `str` at 0xcc438 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc440 | load | 4 | `abs 0xcc47c` | `-` |
| `ldr` at 0xcc444 | load | 4 | `abs 0xcc4a0` | `-` |
| `str` at 0xcc454 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc45c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcc468 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcc46c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcc470 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc470 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc470 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc2a4 | `(unknown callee) 0x1b0d0` | `0x1346b0` | `r1` | `r2` | `r0` |
| `bl` at 0xcc2b4 | `(unknown callee) 0x1b148` | `ret(0x1b0d0)` | `?` | `?` | `ret(0x1b0d0)` |
| `bl` at 0xcc2c8 | `(unknown callee) 0x1b148` | `ret(0x1b148)` | `?` | `?` | `[ret(0x1b148)]` |
| `bl` at 0xcc2d4 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc300 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x1e8` | `0x0` |
| `bl` at 0xcc37c | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x1f2` | `0x0` |
| `bl` at 0xcc394 | `(unknown callee) 0x1b400` | `0x1346b0` | `0x241` | `r0` | `?` |
| `bl` at 0xcc3ac | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0xcc3b8 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc3e4 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x1f7` | `0x0` |
| `bl` at 0xcc404 | `(unknown callee) 0x1b058` | `ret(0x1b400)` | `sp-21` | `0x1` | `sp-21` |
| `bl` at 0xcc414 | `(unknown callee) 0x1b148` | `ret(0x1b058)` | `?` | `?` | `ret(0x1b058)` |
| `bl` at 0xcc420 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc44c | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x1fd` | `0x0` |
| `bl` at 0xcc460 | `(unknown callee) 0x1b094` | `[sp-28]` | `?` | `?` | `ret(0x1b058)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcc29c | `r0 != 0x0` | `r0 == 0x0` |
| `bge` at 0xcc2b0 | `ret(0x1b0d0) >= 0x0 (signed, 32-bit)` | `ret(0x1b0d0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcc2c4 | `[ret(0x1b148)] == 0x2` | `[ret(0x1b148)] != 0x2` |
| `bne` at 0xcc32c | `r0 != [0x2b4428]` | `r0 == [0x2b4428]` |
| `bne` at 0xcc34c | `r0 != [0x2b442c]` | `r0 == [0x2b442c]` |
| `bge` at 0xcc3a8 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcc410 | `ret(0x1b058) == 0x1` | `ret(0x1b058) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc470 | `r0` | `?` |

## `flash_set_default_image`

- `flash_set_default_image`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc4a4 to 0xcc754.
  Established. Two included four-byte pointee stores target `[0x2b443c] +0x0` and `[0x2b443c] +0x4`,
  each with unknown value (`str` at 0xcc4d0, `str` at 0xcc4fc). The branch takes
  `[sp-20] != [0x2b4428]` to `0xcc5a8` (`bne` at 0xcc598). Fall-through includes the byte store
  of `0x41` at `sp -0x19` (`strb` at 0xcc5a0). The unnamed `0x1b058` call has `ret(0x1b400)`,
  `sp-25`, `0x1`, `sp-25` (`bl` at 0xcc670). Default-selection encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc4b4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcc4b8 | load | 4 | `abs 0xcc724` | `-` |
| `ldr` at 0xcc4bc | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc4c0 | load | 4 | `abs 0xcc724` | `-` |
| `ldr` at 0xcc4c4 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc4c8 | load | 4 | `[0x2b443c] +0x0` | `-` |
| `str` at 0xcc4d0 | store | 4 | `[0x2b443c] +0x0` | `?` |
| `ldr` at 0xcc4d4 | load | 4 | `abs 0xcc724` | `-` |
| `ldr` at 0xcc4dc | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc4e4 | load | 4 | `abs 0xcc724` | `-` |
| `ldr` at 0xcc4ec | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc4f4 | load | 4 | `[0x2b443c] +0x4` | `-` |
| `str` at 0xcc4fc | store | 4 | `[0x2b443c] +0x4` | `?` |
| `ldr` at 0xcc500 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc50c | load | 4 | `abs 0xcc728` | `-` |
| `ldr` at 0xcc528 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc53c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc548 | load | 4 | `abs 0xcc72c` | `-` |
| `str` at 0xcc54c | store | 4 | `sp -0x2c` | `0x134788` |
| `ldr` at 0xcc550 | load | 4 | `abs 0xcc728` | `-` |
| `str` at 0xcc554 | store | 4 | `sp -0x28` | `0x134744` |
| `str` at 0xcc558 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc560 | load | 4 | `abs 0xcc730` | `-` |
| `ldr` at 0xcc564 | load | 4 | `abs 0xcc734` | `-` |
| `str` at 0xcc574 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xcc580 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcc588 | load | 4 | `abs 0xcc724` | `-` |
| `ldr` at 0xcc58c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc590 | load | 4 | `abs 0x2b4428` | `-` |
| `strb` at 0xcc5a0 | store | 1 | `sp -0x19` | `0x41` |
| `ldr` at 0xcc5a8 | load | 4 | `abs 0xcc724` | `-` |
| `ldr` at 0xcc5ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc5b0 | load | 4 | `abs 0x2b442c` | `-` |
| `strb` at 0xcc5c0 | store | 1 | `sp -0x19` | `0x42` |
| `ldr` at 0xcc5c8 | load | 4 | `abs 0xcc738` | `-` |
| `str` at 0xcc5cc | store | 4 | `sp -0x2c` | `0x1347b8` |
| `ldr` at 0xcc5d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc5d4 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xcc5dc | load | 4 | `abs 0xcc730` | `-` |
| `ldr` at 0xcc5e0 | load | 4 | `abs 0xcc73c` | `-` |
| `str` at 0xcc5f0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc5f8 | load | 4 | `abs 0xcc728` | `-` |
| `ldr` at 0xcc5fc | load | 4 | `abs 0xcc740` | `-` |
| `str` at 0xcc608 | store | 4 | `sp -0x18` | `ret(0x1b400)` |
| `ldr` at 0xcc60c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc620 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc62c | load | 4 | `abs 0xcc744` | `-` |
| `str` at 0xcc630 | store | 4 | `sp -0x2c` | `0x134718` |
| `ldr` at 0xcc634 | load | 4 | `abs 0xcc728` | `-` |
| `str` at 0xcc638 | store | 4 | `sp -0x28` | `0x134744` |
| `str` at 0xcc63c | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc644 | load | 4 | `abs 0xcc730` | `-` |
| `str` at 0xcc658 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc664 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc688 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc694 | load | 4 | `abs 0xcc748` | `-` |
| `str` at 0xcc698 | store | 4 | `sp -0x2c` | `0x1347e4` |
| `ldr` at 0xcc69c | load | 4 | `abs 0xcc728` | `-` |
| `str` at 0xcc6a0 | store | 4 | `sp -0x28` | `0x134744` |
| `str` at 0xcc6a4 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc6ac | load | 4 | `abs 0xcc730` | `-` |
| `ldr` at 0xcc6b0 | load | 4 | `abs 0xcc74c` | `-` |
| `str` at 0xcc6c0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc6c8 | load | 4 | `abs 0xcc724` | `-` |
| `ldrb` at 0xcc6cc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc6d0 | load | 4 | `abs 0xcc750` | `-` |
| `ldr` at 0xcc6d8 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc6e4 | load | 4 | `abs 0xcc724` | `-` |
| `ldrb` at 0xcc6e8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc6ec | load | 4 | `abs 0xcc750` | `-` |
| `ldr` at 0xcc6f4 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc700 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcc708 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcc70c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcc718 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcc71c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcc720 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc720 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc720 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc510 | `(unknown callee) 0x1b0d0` | `0x134744` | `[0x2b443c]+4` | `0x4` | `[sp-20]` |
| `bl` at 0xcc520 | `(unknown callee) 0x1b148` | `ret(0x1b0d0)` | `?` | `?` | `ret(0x1b0d0)` |
| `bl` at 0xcc534 | `(unknown callee) 0x1b148` | `ret(0x1b148)` | `?` | `?` | `[ret(0x1b148)]` |
| `bl` at 0xcc540 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc56c | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x215` | `0x0` |
| `bl` at 0xcc5e8 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x21f` | `0x0` |
| `bl` at 0xcc600 | `(unknown callee) 0x1b400` | `0x134744` | `0x241` | `[sp-20]` | `?` |
| `bl` at 0xcc618 | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0xcc624 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc650 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x224` | `0x0` |
| `bl` at 0xcc670 | `(unknown callee) 0x1b058` | `ret(0x1b400)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xcc680 | `(unknown callee) 0x1b148` | `ret(0x1b058)` | `?` | `?` | `ret(0x1b058)` |
| `bl` at 0xcc68c | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc6b8 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x22a` | `0x0` |
| `bl` at 0xcc710 | `(unknown callee) 0x1b094` | `[sp-24]` | `0x2b4420` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcc508 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bge` at 0xcc51c | `ret(0x1b0d0) >= 0x0 (signed, 32-bit)` | `ret(0x1b0d0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcc530 | `[ret(0x1b148)] == 0x2` | `[ret(0x1b148)] != 0x2` |
| `bne` at 0xcc598 | `[sp-20] != [0x2b4428]` | `[sp-20] == [0x2b4428]` |
| `bne` at 0xcc5b8 | `[sp-20] != [0x2b442c]` | `[sp-20] == [0x2b442c]` |
| `bge` at 0xcc614 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `beq` at 0xcc67c | `ret(0x1b058) == 0x1` | `ret(0x1b058) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc720 | `r0` | `?` |

## `flash_get_default_image`

- `flash_get_default_image`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc754 to 0xcc980.
  Established. Included four-byte stores through `[0x2b443c] +0x0` and `[0x2b443c] +0x4` have
  unknown values (`str` at 0xcc77c, `str` at 0xcc7a8). The unnamed `0x1b58c` call carries
  `ret(0x1b400)`, `sp-25`, `0x1`, `sp-25` (`bl` at 0xcc844). The branch takes `[sp-25]:1 != 0x41` to
  `0xcc8b8` (`bne` at 0xcc8a4). Fall-through includes a four-byte assignment at `sp -0x18`
  with `[0x2b4428]` (`str` at 0xcc8b0). Stored selection and hardware roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xcc764 | load | 4 | `abs 0xcc960` | `-` |
| `ldr` at 0xcc768 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc76c | load | 4 | `abs 0xcc960` | `-` |
| `ldr` at 0xcc770 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc774 | load | 4 | `[0x2b443c] +0x0` | `-` |
| `str` at 0xcc77c | store | 4 | `[0x2b443c] +0x0` | `?` |
| `ldr` at 0xcc780 | load | 4 | `abs 0xcc960` | `-` |
| `ldr` at 0xcc788 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc790 | load | 4 | `abs 0xcc960` | `-` |
| `ldr` at 0xcc798 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc7a0 | load | 4 | `[0x2b443c] +0x4` | `-` |
| `str` at 0xcc7a8 | store | 4 | `[0x2b443c] +0x4` | `?` |
| `ldr` at 0xcc7ac | load | 4 | `abs 0xcc964` | `-` |
| `str` at 0xcc7bc | store | 4 | `sp -0x14` | `ret(0x1b400)` |
| `ldr` at 0xcc7c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc7d4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xcc7e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xcc7f4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc800 | load | 4 | `abs 0xcc968` | `-` |
| `str` at 0xcc804 | store | 4 | `sp -0x2c` | `0x134810` |
| `ldr` at 0xcc808 | load | 4 | `abs 0xcc964` | `-` |
| `str` at 0xcc80c | store | 4 | `sp -0x28` | `0x134744` |
| `str` at 0xcc810 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc818 | load | 4 | `abs 0xcc96c` | `-` |
| `str` at 0xcc82c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc85c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcc868 | load | 4 | `abs 0xcc970` | `-` |
| `str` at 0xcc86c | store | 4 | `sp -0x2c` | `0x134840` |
| `ldr` at 0xcc870 | load | 4 | `abs 0xcc964` | `-` |
| `str` at 0xcc874 | store | 4 | `sp -0x28` | `0x134744` |
| `str` at 0xcc878 | store | 4 | `sp -0x24` | `ret(0x21704)` |
| `ldr` at 0xcc880 | load | 4 | `abs 0xcc96c` | `-` |
| `ldr` at 0xcc884 | load | 4 | `abs 0xcc974` | `-` |
| `str` at 0xcc894 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xcc89c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc8a8 | load | 4 | `abs 0xcc960` | `-` |
| `ldr` at 0xcc8ac | load | 4 | `abs 0x2b4428` | `-` |
| `str` at 0xcc8b0 | store | 4 | `sp -0x18` | `[0x2b4428]` |
| `ldrb` at 0xcc8b8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc8c4 | load | 4 | `abs 0xcc960` | `-` |
| `ldr` at 0xcc8c8 | load | 4 | `abs 0x2b442c` | `-` |
| `str` at 0xcc8cc | store | 4 | `sp -0x18` | `[0x2b442c]` |
| `ldr` at 0xcc8d4 | load | 4 | `abs 0xcc978` | `-` |
| `str` at 0xcc8d8 | store | 4 | `sp -0x2c` | `0x134874` |
| `ldrb` at 0xcc8dc | load | 1 | `sp -0x19` | `-` |
| `str` at 0xcc8e0 | store | 4 | `sp -0x28` | `[sp-25]:1` |
| `ldr` at 0xcc8e8 | load | 4 | `abs 0xcc96c` | `-` |
| `str` at 0xcc8fc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcc904 | load | 4 | `abs 0xcc960` | `-` |
| `ldrb` at 0xcc908 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc90c | load | 4 | `abs 0xcc97c` | `-` |
| `ldr` at 0xcc914 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc920 | load | 4 | `abs 0xcc960` | `-` |
| `ldrb` at 0xcc924 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xcc928 | load | 4 | `abs 0xcc97c` | `-` |
| `ldr` at 0xcc930 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xcc93c | load | 4 | `(unknown)` | `-` |
| `str` at 0xcc944 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcc948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcc950 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcc954 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xcc958 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcc95c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc95c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc95c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc7b4 | `(unknown callee) 0x1b400` | `0x134744` | `0x0` | `0x4` | `?` |
| `bl` at 0xcc7cc | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0xcc7ec | `(unknown callee) 0x1b148` | `ret(0x1b148)` | `?` | `?` | `[ret(0x1b148)]` |
| `bl` at 0xcc7f8 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc824 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x248` | `0x0` |
| `bl` at 0xcc844 | `(unknown callee) 0x1b58c` | `ret(0x1b400)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xcc854 | `(unknown callee) 0x1b148` | `ret(0x1b58c)` | `?` | `?` | `ret(0x1b58c)` |
| `bl` at 0xcc860 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcc88c | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x24e` | `0x0` |
| `bl` at 0xcc8f4 | `(unknown callee) 0x21eb8` | `0x3` | `0x1343a0` | `0x258` | `0x0` |
| `bl` at 0xcc94c | `(unknown callee) 0x1b094` | `[sp-20]` | `0x2b4420` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcc7c8 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `bne` at 0xcc7dc | `[ret(0x1b148)] != 0x2` | `[ret(0x1b148)] == 0x2` |
| `beq` at 0xcc850 | `ret(0x1b58c) == 0x1` | `ret(0x1b58c) != 0x1` |
| `bne` at 0xcc8a4 | `[sp-25]:1 != 0x41` | `[sp-25]:1 == 0x41` |
| `bne` at 0xcc8c0 | `[sp-25]:1 != 0x42` | `[sp-25]:1 == 0x42` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc95c | `r0` | `?` |
