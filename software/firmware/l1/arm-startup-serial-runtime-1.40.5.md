# ARM startup, serial and runtime helpers in 1.40.5

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/startup-serial-runtime-facts.txt) retain all tool sections.

## `init`

- `init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83328 to 0x83b98.
  Established. The absolute word at `0x2b24b0` receives entry `r0` (`str` at 0x83348). The absolute
  word assignment at `0x2b24b4` writes `ret(0x1b1b4)` (`str` at 0x83380). The unnamed `0x21eb8` call
  carries `0x1`, `0x112380`, `0x166`, `0x0` (`bl` at 0x834dc). Initialization policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x83338 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8333c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x83340 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83344 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x83348 | store | 4 | `abs 0x2b24b0` | `r0` |
| `ldr` at 0x8334c | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83350 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x83380 | store | 4 | `abs 0x2b24b4` | `ret(0x1b1b4)` |
| `ldr` at 0x83384 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83388 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x833ac | load | 4 | `abs 0x2b24b4` | `-` |
| `str` at 0x833c0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x833c4 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x833c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x833cc | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0x833dc | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x833e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83404 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x8340c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83420 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83428 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8342c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x83430 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x83434 | store | 4 | `(unknown)` | `[sp-88]` |
| `ldr` at 0x83438 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x83444 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x83454 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83458 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8347c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83484 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83498 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x834a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x834ac | store | 4 | `(unknown)` | `ret(0xca524)` |
| `ldr` at 0x834b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x834bc | load | 4 | `abs 0x83b64` | `-` |
| `str` at 0x834c0 | store | 4 | `sp -0x78` | `0x11272c` |
| `ldr` at 0x834c4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x834c8 | store | 4 | `sp -0x74` | `[sp-52]` |
| `ldr` at 0x834d0 | load | 4 | `abs 0x83b68` | `-` |
| `ldr` at 0x834d4 | load | 4 | `abs 0x83b6c` | `-` |
| `ldr` at 0x834e4 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x834e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8350c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83518 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x83528 | store | 1 | `sp -0x3f` | `0x1` |
| `strb` at 0x83530 | store | 1 | `sp -0x3c` | `0x1` |
| `ldr` at 0x83534 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83538 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8355c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83568 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x83574 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83578 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8359c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x835a4 | load | 4 | `abs 0x83b70` | `-` |
| `str` at 0x835a8 | store | 4 | `(unknown)` | `0x87650` |
| `ldr` at 0x835ac | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x835b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x835d4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x835dc | load | 4 | `abs 0x83b74` | `-` |
| `str` at 0x835e0 | store | 4 | `(unknown)` | `0x87754` |
| `ldr` at 0x835e8 | load | 4 | `abs 0x83b78` | `-` |
| `ldr` at 0x835ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x835f4 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x835f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8361c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83624 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83638 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83640 | load | 4 | `(unknown)` | `-` |
| `str` at 0x83644 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x83648 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x8364c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83670 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83678 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8368c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83694 | load | 4 | `(unknown)` | `-` |
| `str` at 0x83698 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8369c | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x836a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x836c4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x836cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x836e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x836e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x836ec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x836f0 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x836f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83718 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83720 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83734 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8373c | load | 4 | `(unknown)` | `-` |
| `str` at 0x83740 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8374c | load | 4 | `abs 0x83b7c` | `-` |
| `ldr` at 0x83750 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83758 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x8375c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83780 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x837ac | load | 4 | `abs 0x83b80` | `-` |
| `ldr` at 0x837b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x837b8 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x837bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x837e0 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83804 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83808 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8382c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83834 | load | 4 | `abs 0x83b84` | `-` |
| `str` at 0x83838 | store | 4 | `(unknown)` | `0x87880` |
| `ldr` at 0x8383c | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83840 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83864 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x8386c | load | 4 | `abs 0x83b88` | `-` |
| `str` at 0x83870 | store | 4 | `(unknown)` | `0x87af8` |
| `ldr` at 0x83874 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83878 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8389c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x838a4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x838a8 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0x838ac | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x838b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x838d4 | load | 4 | `abs 0x2b24b4` | `-` |
| `str` at 0x838e0 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x838e4 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x838e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8390c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83914 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x83930 | load | 4 | `(unknown)` | `-` |
| `str` at 0x83934 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x83940 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83944 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83968 | load | 4 | `abs 0x2b24b4` | `-` |
| `str` at 0x83988 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8398c | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83990 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x839b4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x839bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x839c0 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x839c4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x839d0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x839e0 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x839e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83a08 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83a14 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83a18 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83a3c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83a48 | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83a4c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83a70 | load | 4 | `abs 0x2b24b4` | `-` |
| `str` at 0x83a80 | store | 4 | `sp -0x78` | `0x100` |
| `str` at 0x83a88 | store | 4 | `sp -0x74` | `0x0` |
| `str` at 0x83a90 | store | 4 | `sp -0x70` | `0x0` |
| `str` at 0x83a98 | store | 4 | `sp -0x6c` | `0x0` |
| `str` at 0x83aa0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x83aa4 | load | 4 | `abs 0x83b8c` | `-` |
| `str` at 0x83aa8 | store | 4 | `sp -0x64` | `0x874ac` |
| `ldr` at 0x83aac | load | 4 | `sp -0x34` | `-` |
| `str` at 0x83ab0 | store | 4 | `sp -0x60` | `[sp-52]` |
| `ldr` at 0x83ad0 | load | 4 | `abs 0x83b90` | `-` |
| `ldr` at 0x83ad4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83adc | load | 4 | `abs 0x83b60` | `-` |
| `ldr` at 0x83ae0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x83b04 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x83b14 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x83b18 | store | 4 | `sp -0x78` | `[sp-52]` |
| `str` at 0x83b20 | store | 4 | `sp -0x74` | `0xb20` |
| `str` at 0x83b28 | store | 4 | `sp -0x70` | `0x19` |
| `str` at 0x83b30 | store | 4 | `sp -0x6c` | `0xa` |
| `str` at 0x83b38 | store | 4 | `sp -0x68` | `0x9` |
| `ldr` at 0x83b40 | load | 4 | `abs 0x83b94` | `-` |
| `ldr` at 0x83b4c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x83b54 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldmdb` at 0x83b5c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x83b5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x83b5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x83b5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83378 | `(unknown callee) 0x1b1b4` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0x833b8 | `(unknown callee) 0x1b4cc` | `[0x2b24b4]` | `0x0` | `?` | `?` |
| `bl` at 0x834a8 | `(unknown callee) 0xca524` | `?` | `0x2` | `?` | `?` |
| `bl` at 0x834dc | `(unknown callee) 0x21eb8` | `0x1` | `0x112380` | `0x166` | `0x0` |
| `bl` at 0x83520 | `(unknown callee) 0xcade4` | `?` | `sp-84` | `sp-84` | `?` |
| `bl` at 0x83570 | `(unknown callee) 0xcae20` | `?` | `sp-84` | `sp-84` | `?` |
| `bl` at 0x835f0 | `(unknown callee) 0x2079c` | `0x112748` | `[sp-52]` | `?` | `[sp-88]` |
| `bl` at 0x83754 | `(unknown callee) 0x1b52c` | `sp-44` | `0x112758` | `[sp-52]` | `sp-44` |
| `bl` at 0x837a0 | `(unknown callee) 0x229b0` | `?` | `sp-44` | `0x0` | `0x9` |
| `bl` at 0x837b4 | `(unknown callee) 0x1b52c` | `sp-44` | `0x112764` | `[sp-52]` | `sp-44` |
| `bl` at 0x83800 | `(unknown callee) 0x229b0` | `?` | `sp-44` | `0x0` | `0x9` |
| `bl` at 0x83974 | `(unknown callee) 0x1f26c` | `0x8` | `[sp-52]` | `?` | `[0x2b24b4]` |
| `bl` at 0x83ac4 | `(unknown callee) 0xbb094` | `?` | `?` | `0x100` | `?` |
| `bl` at 0x83ad8 | `(unknown callee) 0x1b52c` | `sp-44` | `0x112770` | `[sp-52]` | `sp-44` |
| `bl` at 0x83b48 | `(unknown callee) 0x22480` | `?` | `sp-44` | `0x84b58` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x833d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x83440 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `beq` at 0x8344c | `[sp-88] == 0x1` | `[sp-88] != 0x1` |
| `bne` at 0x834b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8393c | `? != 0x2` | `? == 0x2` |
| `beq` at 0x839cc | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0x839d8 | `[sp-92] == 0x1` | `[sp-92] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x83b5c | `r0` | `?` |

## `sernumInit`

- `sernumInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5116c to 0x516d8.
  Established. The absolute word at `0x2af388` is read (`ldr` at 0x511bc); its zero value takes
  `0x511dc`, while nonzero falls through (`beq` at 0x511c4). The indirect call at `0x511d4` has an
  unknown target and candidates `ret(0x1f26c)`, unknown, unknown, `[0x2af388]` (`mov` at 0x511d4).
  Serial initialization policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x51180 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x51190 | store | 4 | `sp -0x14` | `?` |
| `str` at 0x5119c | store | 4 | `sp -0x18` | `ret(0x1f26c)` |
| `str` at 0x511a8 | store | 4 | `sp -0x20` | `ret(0x1f26c)` |
| `str` at 0x511b4 | store | 4 | `sp -0x24` | `ret(0x1f26c)` |
| `ldr` at 0x511b8 | load | 4 | `abs 0x516bc` | `-` |
| `ldr` at 0x511bc | load | 4 | `abs 0x2af388` | `-` |
| `ldr` at 0x511c8 | load | 4 | `abs 0x516bc` | `-` |
| `ldr` at 0x511cc | load | 4 | `abs 0x2af388` | `-` |
| `str` at 0x511ec | store | 4 | `sp -0x28` | `ret(0x4f730)` |
| `ldr` at 0x511f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x511fc | load | 4 | `abs 0x516c0` | `-` |
| `str` at 0x51200 | store | 4 | `sp -0xb0` | `0x10b790` |
| `ldr` at 0x51208 | load | 4 | `abs 0x516c4` | `-` |
| `str` at 0x5121c | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x51220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5122c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5125c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51280 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5128c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x512b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x512bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x512c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x512d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x512e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x512ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x512f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5131c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5134c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5137c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51388 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x513f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51404 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x51410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5141c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x51434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5145c | load | 4 | `abs 0x516bc` | `-` |
| `ldrb` at 0x51460 | load | 1 | `abs 0x2af28c` | `-` |
| `ldr` at 0x5146c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51470 | load | 4 | `abs 0x516bc` | `-` |
| `ldrb` at 0x51474 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x51478 | load | 1 | `abs 0x2af28c` | `-` |
| `ldr` at 0x51484 | load | 4 | `abs 0x516c8` | `-` |
| `ldm` at 0x5148c | load | 4 | `abs 0x10b9ec` | `-` |
| `ldm` at 0x5148c | load | 4 | `abs 0x10b9f0` | `-` |
| `ldm` at 0x5148c | load | 4 | `abs 0x10b9f4` | `-` |
| `stm` at 0x51490 | store | 4 | `sp -0xac` | `[0x10b9ec]` |
| `stm` at 0x51490 | store | 4 | `sp -0xa8` | `[0x10b9f0]` |
| `strb` at 0x51494 | store | 1 | `sp -0xa4` | `([0x10b9f4] & 0xff)` |
| `ldr` at 0x51498 | load | 4 | `abs 0x516bc` | `-` |
| `ldrb` at 0x5149c | load | 1 | `abs 0x2af28c` | `-` |
| `strb` at 0x514a0 | store | 1 | `sp -0xac` | `([0x2af28c]:1 & 0xff)` |
| `ldr` at 0x514b4 | load | 4 | `abs 0x516cc` | `-` |
| `ldr` at 0x514bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x514c0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x514cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x514d0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x514e0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x514e8 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x514f8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51500 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x5151c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51524 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x5152c | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x51534 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51554 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5155c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldr` at 0x51564 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x5156c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x5158c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51594 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5159c | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x515a4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x515c4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x515cc | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x515d4 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x515dc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x515fc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x51604 | load | 1 | `[sp-32] +0x6` | `-` |
| `ldr` at 0x5160c | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x51614 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51634 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5163c | load | 1 | `[sp-32] +0x7` | `-` |
| `ldr` at 0x51644 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x5164c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x51660 | load | 4 | `abs 0x516d0` | `-` |
| `ldr` at 0x51668 | load | 4 | `abs 0x516cc` | `-` |
| `ldr` at 0x51670 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5167c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x51680 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x51698 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x516a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x516b0 | load | 4 | `abs 0x516d4` | `-` |
| `ldmdb` at 0x516b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x516b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x516b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x51184 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x51198 | `(unknown callee) 0x1f26c` | `0x3d` | `?` | `?` | `?` |
| `bl` at 0x511a4 | `(unknown callee) 0x1f26c` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x511b0 | `(unknown callee) 0x1f26c` | `0xf` | `?` | `?` | `?` |
| `mov` at 0x511d4 | `(unknown callee) (unknown target)` | `ret(0x1f26c)` | `?` | `?` | `[0x2af388]` |
| `bl` at 0x511e4 | `(unknown callee) 0x1d41c` | `0x54` | `0x1` | `?` | `[0x2af388]` |
| `bl` at 0x511e8 | `(unknown callee) 0x4f730` | `ret(0x1d41c)` | `?` | `?` | `?` |
| `bl` at 0x51214 | `(unknown callee) 0x21eb8` | `0x1` | `0x10b7b8` | `0x300` | `0x0` |
| `bl` at 0x512ac | `(unknown callee) 0x1d41c` | `0x50` | `0x0` | `?` | `[sp-20]` |
| `bl` at 0x513c0 | `(unknown callee) 0x1f26c` | `0x50` | `?` | `?` | `[sp-20]` |
| `bl` at 0x51448 | `(unknown callee) 0x1f26c` | `0x3e` | `?` | `?` | `[sp-20]` |
| `bl` at 0x514b0 | `(unknown callee) 0x1d41c` | `0xd` | `sp-172` | `[0x10b9f4]` | `sp-172` |
| `bl` at 0x514b8 | `(unknown callee) 0x202d0` | `0x10b9f8` | `?` | `?` | `?` |
| `bl` at 0x5150c | `(unknown callee) 0x1b55c` | `?` | `?` | `0x1` | `[[sp-32]+1]:1` |
| `bl` at 0x51544 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x5157c | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x515b4 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x515ec | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x51624 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x51664 | `(unknown callee) 0x1d41c` | `0xd` | `0x10ba14` | `?` | `?` |
| `bl` at 0x5166c | `(unknown callee) 0x202d0` | `0x10b9f8` | `?` | `?` | `?` |
| `bl` at 0x51688 | `(unknown callee) 0x1b2e0` | `[sp-36]` | `[sp-40]` | `0x6` | `[sp-40]` |
| `bl` at 0x516ac | `(unknown callee) 0x1d41c` | `0xf` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0x516b4 | `(unknown callee) 0x202d0` | `0x10ba20` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x511c4 | `[0x2af388] == 0x0` | `[0x2af388] != 0x0` |
| `bne` at 0x511f8 | `ret(0x4f730) != 0x0` | `ret(0x4f730) == 0x0` |
| `beq` at 0x51228 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x51234 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x51240 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x5124c | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x51258 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x51264 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x51270 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x5127c | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0x51288 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x51294 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0x512a0 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `beq` at 0x512b8 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x512c4 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x512d0 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x512dc | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x512e8 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x512f4 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x51300 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x5130c | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0x51318 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x51324 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `bne` at 0x51330 | `[sp-20] != 0x4d` | `[sp-20] == 0x4d` |
| `beq` at 0x5133c | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x51348 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x51354 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x51360 | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0x5136c | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x51378 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0x51384 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0x51390 | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0x5139c | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0x513a8 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0x513b4 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `bne` at 0x513cc | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `beq` at 0x513d8 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x513e4 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x513f0 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x513fc | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0x5140c | `[sp-24] == 0x1` | `[sp-24] != 0x1` |
| `beq` at 0x51418 | `[sp-20] == 0x52` | `[sp-20] != 0x52` |
| `beq` at 0x51424 | `[sp-20] == 0x44` | `[sp-20] != 0x44` |
| `beq` at 0x51430 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0x5143c | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `bne` at 0x51454 | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `beq` at 0x51468 | `[0x2af28c]:1 == 0x0` | `[0x2af28c]:1 != 0x0` |
| `beq` at 0x51480 | `[[sp-32]]:1 == [0x2af28c]:1` | `[[sp-32]]:1 != [0x2af28c]:1` |
| `bls` at 0x514c8 | `[[sp-32]]:1 <= 0x4b (unsigned, 32-bit)` | `[[sp-32]]:1 > 0x4b (unsigned, 32-bit)` |
| `bhi` at 0x514d8 | `[[sp-32]]:1 > 0x52 (unsigned, 32-bit)` | `[[sp-32]]:1 <= 0x52 (unsigned, 32-bit)` |
| `bls` at 0x514f0 | `[[sp-32]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-32]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0x51508 | `[[sp-32]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-32]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0x51540 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x51578 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x515b0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x515e8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x51620 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x51658 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x51678 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0x51694 | `ret(0x1b2e0) == 0x0` | `ret(0x1b2e0) != 0x0` |
| `bne` at 0x516a0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x516b8 | `r0` | `?` |

## `KeyLogInsert`

- `KeyLogInsert`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50da0 to 0x50f94.
  Established. The unnamed `0x1f26c` call at `0x50dbc` carries `0x51`, entry `r1`, `r2`, `sp-28`
  (`bl` at 0x50dbc). Included byte assignments through `[sp-36] +0x0` and `[sp-36] +0x6` have
  unknown values (`strb` at 0x50e38, `strb` at 0x50f28). The unnamed `0x1d41c` call at `0x50f80`
  carries `0x52`, `[sp-40]`, unknown, unknown (`bl` at 0x50f80). Serial key log format remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stm` at 0x50db4 | store | 4 | `sp -0x1c` | `r0` |
| `stm` at 0x50db4 | store | 4 | `sp -0x18` | `r1` |
| `stm` at 0x50db4 | store | 4 | `sp -0x14` | `r2` |
| `strb` at 0x50dc4 | store | 1 | `sp -0x1d` | `(ret(0x1f26c) & 0xff)` |
| `str` at 0x50dd0 | store | 4 | `sp -0x28` | `ret(0x1f26c)` |
| `str` at 0x50ddc | store | 4 | `sp -0x24` | `ret(0x1f26c)` |
| `ldrb` at 0x50de0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x50df0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50df8 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x50dfc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x50e0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x50e14 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x50e1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50e20 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x50e38 | store | 1 | `[sp-36] +0x0` | `?` |
| `str` at 0x50e40 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x50e48 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50e4c | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x50e5c | store | 1 | `[sp-36] +0x1` | `?` |
| `str` at 0x50e64 | store | 4 | `sp -0x24` | `[sp-36]+2` |
| `ldr` at 0x50e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50e70 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x50e88 | store | 1 | `[sp-36] +0x2` | `?` |
| `str` at 0x50e90 | store | 4 | `sp -0x24` | `[sp-36]+3` |
| `ldr` at 0x50e98 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50e9c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x50eac | store | 1 | `[sp-36] +0x3` | `?` |
| `str` at 0x50eb4 | store | 4 | `sp -0x24` | `[sp-36]+4` |
| `ldr` at 0x50ebc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50ec0 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x50ed8 | store | 1 | `[sp-36] +0x4` | `?` |
| `str` at 0x50ee0 | store | 4 | `sp -0x24` | `[sp-36]+5` |
| `ldr` at 0x50ee8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x50eec | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x50f04 | store | 1 | `[sp-36] +0x5` | `?` |
| `str` at 0x50f0c | store | 4 | `sp -0x24` | `[sp-36]+6` |
| `ldr` at 0x50f14 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50f18 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x50f28 | store | 1 | `[sp-36] +0x6` | `?` |
| `str` at 0x50f30 | store | 4 | `sp -0x24` | `[sp-36]+7` |
| `ldrb` at 0x50f34 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x50f3c | load | 4 | `abs 0x50f90` | `-` |
| `strb` at 0x50f64 | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0x50f68 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x50f7c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x50f8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x50f8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x50f8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50dbc | `(unknown callee) 0x1f26c` | `0x51` | `r1` | `r2` | `sp-28` |
| `bl` at 0x50dcc | `(unknown callee) 0x1f26c` | `0x52` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x50dd8 | `(unknown callee) 0x1f26c` | `0x53` | `?` | `?` | `?` |
| `bl` at 0x50f74 | `(unknown callee) 0x1d41c` | `0x51` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x50f80 | `(unknown callee) 0x1d41c` | `0x52` | `[sp-40]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x50f8c | `r0` | `0x0` |

## `KeyInLog`

- `KeyInLog`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50f94 to 0x5116c.
  Established. The unnamed `0x1f26c` call at `0x50fb0` carries `0x52`, entry `r1`, entry `r2`,
  `sp-28` (`bl` at 0x50fb0). Included byte reads use `([sp-44] + [sp-36]) +0x0` and `([sp-44] + [sp-36]) +0x6` (`ldrb` at 0x50ffc, `ldrb` at 0x510fc). The condition `[sp-56] != [sp-20]` takes
  `0x5114c`, while equality falls through (`bne` at 0x51118). Logged key matching policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stm` at 0x50fa8 | store | 4 | `sp -0x1c` | `r0` |
| `stm` at 0x50fa8 | store | 4 | `sp -0x18` | `r1` |
| `stm` at 0x50fa8 | store | 4 | `sp -0x14` | `r2` |
| `str` at 0x50fb4 | store | 4 | `sp -0x20` | `ret(0x1f26c)` |
| `str` at 0x50fc0 | store | 4 | `sp -0x2c` | `ret(0x1f26c)` |
| `str` at 0x50fc8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x50fcc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x50fdc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x50fec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x50ff0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x50ff4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x50ffc | load | 1 | `([sp-44] + [sp-36]) +0x0` | `-` |
| `str` at 0x51008 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x51010 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x51014 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x51020 | load | 1 | `([sp-44] + [sp-36]) +0x1` | `-` |
| `ldr` at 0x51024 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5102c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x51030 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x51034 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x51044 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x51048 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x51054 | load | 1 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0x51060 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x51068 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5106c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x51078 | load | 1 | `([sp-44] + [sp-36]) +0x3` | `-` |
| `ldr` at 0x5107c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x51084 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x51088 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5108c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5109c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x510a0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x510ac | load | 1 | `([sp-44] + [sp-36]) +0x4` | `-` |
| `str` at 0x510b8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x510c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x510c4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x510d0 | load | 1 | `([sp-44] + [sp-36]) +0x5` | `-` |
| `ldr` at 0x510dc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x510e4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x510ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x510f0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x510fc | load | 1 | `([sp-44] + [sp-36]) +0x6` | `-` |
| `ldr` at 0x51100 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x51108 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x5110c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x51110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5111c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x51120 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x51138 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x51144 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x5114c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x51154 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x51160 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x51164 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0x51168 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x51168 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x51168 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x50fb0 | `(unknown callee) 0x1f26c` | `0x52` | `r1` | `r2` | `sp-28` |
| `bl` at 0x50fbc | `(unknown callee) 0x1f26c` | `0x53` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x50fd4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x5103c | `[sp-48] != [sp-28]` | `[sp-48] == [sp-28]` |
| `bne` at 0x51094 | `[sp-52] != [sp-24]` | `[sp-52] == [sp-24]` |
| `bne` at 0x51118 | `[sp-56] != [sp-20]` | `[sp-56] == [sp-20]` |
| `beq` at 0x51130 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x51168 | `r0` | `?` |

## `exceptionInit`

- `exceptionInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20ca8 to 0x20cf0.
  Established. The word at `sp -0x1c` receives `0xfca00` (`str` at 0x20cbc). The unnamed `0x1b1e4`
  call carries entry `r0`, `r1`, `r2`, `0xfca00` (`bl` at 0x20cc4). The absolute word assignment at
  `0x145b98` has an unknown value (`str` at 0x20cd8). Exception initialization policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x20cb8 | load | 4 | `abs 0x20ce8` | `-` |
| `str` at 0x20cbc | store | 4 | `sp -0x1c` | `0xfca00` |
| `ldr` at 0x20cc0 | load | 4 | `abs 0x20cec` | `-` |
| `str` at 0x20cd8 | store | 4 | `abs 0x145b98` | `?` |
| `ldmdb` at 0x20ce4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x20ce4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20ce4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20ce4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20cc4 | `(unknown callee) 0x1b1e4` | `r0` | `r1` | `r2` | `0xfca00` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20ce4 | `r0` | `0x0` |

## `exceptionIsSoftReset`

- `exceptionIsSoftReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20cf0 to 0x20d6c.
  Established. The absolute word at `0x145b98` is read (`ldr` at 0x20d04); its nonzero value takes
  `0x20d44`, while zero falls through (`bne` at 0x20d0c). The unnamed `0x1b088` call carries
  `ret(0x1b0a0)`, `0xfca20`, unknown, `ret(0x1b0a0)` (`bl` at 0x20d34). Soft-reset classification
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x20d00 | load | 4 | `abs 0x20d60` | `-` |
| `ldr` at 0x20d04 | load | 4 | `abs 0x145b98` | `-` |
| `ldr` at 0x20d10 | load | 4 | `abs 0x20d64` | `-` |
| `str` at 0x20d1c | store | 4 | `sp -0x14` | `ret(0x1b0a0)` |
| `ldr` at 0x20d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20d2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20d30 | load | 4 | `abs 0x20d68` | `-` |
| `str` at 0x20d48 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x20d54 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x20d58 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x20d5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20d5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20d5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20d14 | `(unknown callee) 0x1b0a0` | `0xfca10` | `r1` | `r2` | `[0x145b98]` |
| `bl` at 0x20d34 | `(unknown callee) 0x1b088` | `ret(0x1b0a0)` | `0xfca20` | `?` | `ret(0x1b0a0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x20d0c | `[0x145b98] != 0x0` | `[0x145b98] == 0x0` |
| `beq` at 0x20d28 | `ret(0x1b0a0) == 0x0` | `ret(0x1b0a0) != 0x0` |
| `bne` at 0x20d40 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20d5c | `r0` | `?` |

## `exceptionReset`

- `exceptionReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20d6c to 0x20e1c.
  Established. A zero absolute word `[0x145b98]` takes `0x20dac`, while nonzero falls through (`beq` at 0x20d90). The unnamed `0xcc280` call has four unknown candidates (`bl` at 0x20dd4). The unnamed
  `0x22780` call carries `0x1388` and three unknown candidates (`bl` at 0x20de4). Reset effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20d7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x20d80 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x20d84 | load | 4 | `abs 0x20e00` | `-` |
| `ldr` at 0x20d88 | load | 4 | `abs 0x145b98` | `-` |
| `ldr` at 0x20d94 | load | 4 | `abs 0x20e04` | `-` |
| `ldr` at 0x20d98 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x20d9c | load | 4 | `abs 0x20e08` | `-` |
| `ldr` at 0x20dac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20db0 | load | 4 | `abs 0x20e0c` | `-` |
| `ldr` at 0x20dc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20dd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20dd8 | load | 4 | `abs 0x20e10` | `-` |
| `ldr` at 0x20de0 | load | 4 | `abs 0x20e14` | `-` |
| `ldr` at 0x20de8 | load | 4 | `abs 0x20e04` | `-` |
| `ldr` at 0x20dec | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x20df0 | load | 4 | `abs 0x20e18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20da0 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0xfca28` | `r2` | `0x145ae8` |
| `bl` at 0x20da8 | `(unknown callee) 0x1b43c` | `?` | `?` | `?` | `?` |
| `bl` at 0x20dc0 | `(unknown callee) 0x1b43c` | `?` | `?` | `?` | `0x3e7` |
| `bl` at 0x20dd4 | `(unknown callee) 0xcc280` | `?` | `?` | `?` | `?` |
| `bl` at 0x20ddc | `(unknown callee) 0x1b16c` | `0xfca64` | `?` | `?` | `?` |
| `bl` at 0x20de4 | `(unknown callee) 0x22780` | `0x1388` | `?` | `?` | `?` |
| `bl` at 0x20df4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0xfca74` | `?` | `0x145ae8` |
| `bl` at 0x20dfc | `(unknown callee) 0x1b43c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x20d90 | `[0x145b98] == 0x0` | `[0x145b98] != 0x0` |
| `bne` at 0x20db8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x20dcc | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|

## `exceptionBootComplete`

- `exceptionBootComplete`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20e1c to 0x20e34.
  Established. The return reports `0x0` (`ldmdb` at 0x20e30). The same instruction reads words at
  `sp -0x10`, `sp -0xc` and `sp -0x8` (`ldmdb` at 0x20e30). Boot-complete effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x20e30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20e30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20e30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20e30 | `r0` | `0x0` |

## `exceptionDumpPANIC`

- `exceptionDumpPANIC`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20e44 to 0x20e54.
  Established. The return reports entry `r0` (`ldmdb` at 0x20e50); its word reads use `sp -0x10`,
  `sp -0xc` and `sp -0x8` (`ldmdb` at 0x20e50). Panic dump effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x20e50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20e50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20e50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20e50 | `r0` | `r0` |

## `taskCurrent`

- `taskCurrent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22728 to 0x22744.
  Established. The unnamed `0x1b310` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x22734);
  its result is reported by the return (`ldmdb` at 0x22740). Current task identity semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x22740 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x22740 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x22740 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22734 | `(unknown callee) 0x1b310` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x22740 | `r0` | `ret(0x1b310)` |

## `taskName`

- `taskName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22744 to 0x22780.
  Established. Entry `r0 != 0x0` takes `0x22770`, while zero falls through (`bne` at 0x22760). The
  unnamed `0x22728` call carries entry `r0`, `r1`, `r2`, `r0` (`bl` at 0x22764). The return reports
  `0xff1f8` (`ldmdb` at 0x22778). Task label interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x22754 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x22758 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2276c | store | 4 | `sp -0x14` | `ret(0x22728)` |
| `ldr` at 0x22770 | load | 4 | `abs 0x2277c` | `-` |
| `ldmdb` at 0x22778 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x22778 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x22778 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22764 | `(unknown callee) 0x22728` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x22760 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x22778 | `r0` | `0xff1f8` |

## `tfunc`

- `tfunc`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1b978 to 0x1b9a0.
  Established. The absolute word assignment at `0x34752000` writes `0x3472` (`str` at 0x1b98c). The
  return reports `0x34752000` (`ldmdb` at 0x1b994). The destination role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1b984 | load | 4 | `abs 0x1b998` | `-` |
| `ldr` at 0x1b988 | load | 4 | `abs 0x1b99c` | `-` |
| `str` at 0x1b98c | store | 4 | `abs 0x34752000` | `0x3472` |
| `ldmdb` at 0x1b994 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1b994 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1b994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1b994 | `r0` | `0x34752000` |

## `main`

- `main`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1b7bc to 0x1b978.
  Established. The absolute word at `0x145afc` receives `[r1]` (`str` at 0x1b7e8). The word at `sp -0xa4` receives `0x1b6a4` (`str` at 0x1b800). The unnamed `0x65c18` call carries `[sp-168]`,
  `0x0`, unknown, unknown (`bl` at 0x1b940). Program startup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1b7cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1b7d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1b7d8 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0x1b7dc | load | 4 | `abs 0x1b950` | `-` |
| `ldr` at 0x1b7e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1b7e4 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0x1b7e8 | store | 4 | `abs 0x145afc` | `[r1]` |
| `ldr` at 0x1b7fc | load | 4 | `abs 0x1b954` | `-` |
| `str` at 0x1b800 | store | 4 | `sp -0xa4` | `0x1b6a4` |
| `str` at 0x1b808 | store | 4 | `sp -0x20` | `0x4` |
| `ldr` at 0x1b854 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x1b85c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1b86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1b874 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x1b878 | load | 4 | `abs 0x1b958` | `-` |
| `ldr` at 0x1b89c | load | 4 | `abs 0x1b95c` | `-` |
| `ldr` at 0x1b8b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1b8c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1b8c8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x1b8cc | load | 4 | `abs 0x1b960` | `-` |
| `str` at 0x1b8e4 | store | 4 | `sp -0xa8` | `0x1` |
| `ldr` at 0x1b8e8 | load | 4 | `abs 0x1b964` | `-` |
| `ldr` at 0x1b8ec | load | 4 | `abs 0x1b968` | `-` |
| `str` at 0x1b8fc | store | 4 | `sp -0xac` | `ret(0x20ca8)` |
| `ldr` at 0x1b900 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x1b90c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x1b918 | load | 4 | `abs 0x1b96c` | `-` |
| `str` at 0x1b91c | store | 4 | `sp -0xb8` | `0xfc650` |
| `str` at 0x1b920 | store | 4 | `sp -0xb4` | `ret(0x21704)` |
| `ldr` at 0x1b928 | load | 4 | `abs 0x1b970` | `-` |
| `ldr` at 0x1b938 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x1b944 | load | 4 | `abs 0x1b974` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1b7f8 | `(unknown callee) 0x1b424` | `sp-160` | `r1` | `0x145afc` | `sp-160` |
| `bl` at 0x1b81c | `(unknown callee) 0x1b004` | `0xb` | `sp-164` | `0x0` | `sp-164` |
| `bl` at 0x1b830 | `(unknown callee) 0x1b004` | `0x7` | `sp-164` | `0x0` | `sp-164` |
| `bl` at 0x1b844 | `(unknown callee) 0x1b004` | `0x6` | `sp-164` | `0x0` | `sp-164` |
| `bl` at 0x1b850 | `(unknown callee) 0x1b274` | `sp-176` | `?` | `?` | `sp-176` |
| `bl` at 0x1b858 | `(unknown callee) 0x1b3e8` | `[sp-176]` | `?` | `?` | `?` |
| `bl` at 0x1b87c | `(unknown callee) 0x1b088` | `[[sp-24]+4]` | `0xfc618` | `0x4` | `[sp-24]+4` |
| `bl` at 0x1b88c | `(unknown callee) 0x1bcf4` | `ret(0x1b088)` | `?` | `?` | `ret(0x1b088)` |
| `bl` at 0x1b894 | `(unknown callee) 0x1f26c` | `0x46` | `?` | `?` | `?` |
| `bl` at 0x1b8a4 | `(unknown callee) 0x23fbc` | `0xfc620` | `ret(0x1f26c)` | `?` | `ret(0x1f26c)` |
| `bl` at 0x1b8ac | `(unknown callee) 0x1b43c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x1b8d0 | `(unknown callee) 0x1b088` | `[[sp-24]+4]` | `0xfc628` | `0x4` | `[sp-24]+4` |
| `bl` at 0x1b8f0 | `(unknown callee) 0x23fbc` | `0xfc62c` | `0xfc608` | `?` | `?` |
| `bl` at 0x1b8f4 | `(unknown callee) 0x20ca8` | `ret(0x23fbc)` | `?` | `?` | `?` |
| `bl` at 0x1b910 | `(unknown callee) 0x21704` | `ret(0x20ca8)` | `?` | `?` | `ret(0x20ca8)` |
| `bl` at 0x1b934 | `(unknown callee) 0x21eb8` | `0x9` | `0xfc610` | `0xa7` | `0x0` |
| `bl` at 0x1b940 | `(unknown callee) 0x65c18` | `[sp-168]` | `0x0` | `?` | `?` |
| `bl` at 0x1b948 | `(unknown callee) 0x1afe0` | `0x3d090` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x1b864 | `[sp-20] <= 0x1 (signed, 32-bit)` | `[sp-20] > 0x1 (signed, 32-bit)` |
| `bne` at 0x1b888 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `ble` at 0x1b8b8 | `[sp-20] <= 0x1 (signed, 32-bit)` | `[sp-20] > 0x1 (signed, 32-bit)` |
| `bne` at 0x1b8dc | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `beq` at 0x1b908 | `ret(0x20ca8) == 0x0` | `ret(0x20ca8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `oops_handler`

- `oops_handler`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1b6a4 to 0x1b7bc.
  Established. The unnamed `0x20e54` call carries `sp-552`, `0x0`, `0x0`, `0x0` (`bl` at 0x1b6dc).
  The absolute word at `0x2b4430` is read (`ldr` at 0x1b704). The unnamed `0x21004` call carries
  `sp-552`, `0x0`, `(sp-1352 + ret(0x1b2bc))`, `sp-552` (`bl` at 0x1b780). Failure handling policy
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1b6b4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x1b6b8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x1b6bc | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x1b6c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1b6c4 | store | 4 | `sp -0x24` | `r2` |
| `ldr` at 0x1b6e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1b6e4 | load | 4 | `[sp-36] +0x5c` | `-` |
| `str` at 0x1b6e8 | store | 4 | `sp -0x218` | `[[sp-36]+92]` |
| `str` at 0x1b6f0 | store | 4 | `sp -0x54c` | `sp-1348` |
| `ldr` at 0x1b6f4 | load | 4 | `sp -0x54c` | `-` |
| `str` at 0x1b6fc | store | 4 | `sp -0x54c` | `sp-1352` |
| `ldr` at 0x1b700 | load | 4 | `abs 0x1b7ac` | `-` |
| `ldr` at 0x1b704 | load | 4 | `abs 0x2b4430` | `-` |
| `ldr` at 0x1b710 | load | 4 | `abs 0x1b7ac` | `-` |
| `ldr` at 0x1b714 | load | 4 | `abs 0x2b4430` | `-` |
| `str` at 0x1b71c | store | 4 | `sp -0x550` | `?` |
| `ldr` at 0x1b720 | load | 4 | `sp -0x550` | `-` |
| `str` at 0x1b728 | store | 4 | `sp -0x550` | `[sp-1360]+64` |
| `str` at 0x1b734 | store | 4 | `sp -0x550` | `0x3f` |
| `ldr` at 0x1b738 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1b73c | store | 4 | `sp -0x554` | `[sp-24]` |
| `ldr` at 0x1b740 | load | 4 | `sp -0x54c` | `-` |
| `ldr` at 0x1b744 | load | 4 | `abs 0x1b7b0` | `-` |
| `ldr` at 0x1b748 | load | 4 | `sp -0x550` | `-` |
| `ldr` at 0x1b74c | load | 4 | `abs 0x1b7b4` | `-` |
| `str` at 0x1b78c | store | 4 | `sp -0x554` | `sp-1352` |
| `ldr` at 0x1b794 | load | 4 | `abs 0x1b7b8` | `-` |
| `ldrdeq` at 0x1b7b0 | load | 8 | `abs 0x1b7b8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1b6dc | `(unknown callee) 0x20e54` | `sp-552` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x1b750 | `(unknown callee) 0x1b52c` | `sp-1352` | `0xfc5dc` | `?` | `0xfc608` |
| `bl` at 0x1b768 | `(unknown callee) 0x1b2bc` | `sp-1352` | `?` | `?` | `sp-1352` |
| `bl` at 0x1b780 | `(unknown callee) 0x21004` | `sp-552` | `0x0` | `(sp-1352 + ret(0x1b2bc))` | `sp-552` |
| `bl` at 0x1b7a0 | `(unknown callee) 0x21eb8` | `0x9` | `0xfc610` | `0x49` | `0x0` |
| `bl` at 0x1b7a8 | `(unknown callee) 0x1b43c` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1b70c | `[0x2b4430] == 0x0` | `[0x2b4430] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `vector_change_bits`

- `vector_change_bits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3780 to 0xe3870.
  Established. Unsigned entry `r1 <= r0` takes `0xe37f4`, while larger values fall through (`bls` at 0xe37ac). The word through `[sp] +0x0` receives `([[sp]] + [sp-36])` on an included path (`str` at 0xe37d4). The unnamed `0xe2b10` call carries `0x0`, unknown, unknown, `[sp+4]` (`bl` at 0xe3868).
  Vector resize semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3790 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3794 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe3798 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe379c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe37a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe37a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe37b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe37b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe37bc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe37c0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe37c4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe37c8 | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xe37cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe37d4 | store | 4 | `[sp] +0x0` | `([[sp]] + [sp-36])` |
| `ldr` at 0xe37d8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe37dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe37e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe37e4 | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xe37e8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe37f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe37f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3808 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3810 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3814 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3820 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3824 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe382c | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xe3830 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe3838 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe383c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3840 | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xe3844 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe384c | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xe3850 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3854 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe385c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3860 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3864 | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0xe386c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe386c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe386c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe37ec | `(unknown callee) 0xe32b4` | `[sp-36]` | `[sp-32]` | `[[sp]]` | `[sp+4]` |
| `bl` at 0xe3834 | `(unknown callee) 0xe30a4` | `[sp-36]` | `([sp-32] + [sp-20])` | `[[sp]]` | `[sp+4]` |
| `bl` at 0xe3868 | `(unknown callee) 0xe2b10` | `0x0` | `?` | `?` | `[sp+4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe37ac | `r1 <= r0 (unsigned, 32-bit)` | `r1 > r0 (unsigned, 32-bit)` |
| `bhs` at 0xe3800 | `r1 >= r0 (unsigned, 32-bit)` | `r1 < r0 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe386c | `r0` | `ret(0xe2b10)` |
