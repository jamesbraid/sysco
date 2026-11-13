# ARM scan command and TAP routines in 1.40.5

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
sn1sc_process_scancmd has an unresolved computed transfer; its tables and return cover known edges only. Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/scan-tap-facts.txt) retain all tool sections.

## `sn1sc_select_port`

- `sn1sc_select_port`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3c98 to 0xe3cf8.
  Established. The halfword store targets `[0x2b45c0] +0xe` with unknown value (`strh` at 0xe3ce4).
  Earlier explicit byte stores retain `(r0 & 0xff)` at `sp -0x11` and `(r1 & 0xff)` at `sp -0x12`
  (`strb` at 0xe3cb0, `strb` at 0xe3cb8). The exit carries `0x1` (`ldmdb` at 0xe3cf0).
  Port-selection encoding and hardware-window identity are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xe3cb0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xe3cb8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xe3cc0 | load | 4 | `abs 0xe3cf4` | `-` |
| `ldr` at 0xe3cc4 | load | 4 | `abs 0x2b45c0` | `-` |
| `ldrb` at 0xe3ccc | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xe3cd4 | load | 1 | `sp -0x11` | `-` |
| `strh` at 0xe3ce4 | store | 2 | `[0x2b45c0] +0xe` | `?` |
| `ldmdb` at 0xe3cf0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3cf0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3cf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3cf0 | `r0` | `0x1` |

## `sn1sc_process_scancmd`

- `sn1sc_process_scancmd`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3dcc to 0xe5738.
  Established. The prefix assigns byte `0x0` at absolute `0x2b4528` (`strb` at 0xe3e34). The branch
  takes `[0x2adfd4] != 0x0` to `0xe3e80` (`bne` at 0xe3e44). An unknown-target call has unknown,
  unknown, `[0x14385c]`, unknown (`mov` at 0xe3ec8). The computed transfer takes an unresolved edge
  for `? <= 0x14` unsigned 32-bit and falls through for `? > 0x14` (`ldrls` at 0xe3f54).
  Selector-specific command effects and packet-field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3ddc | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xe3de0 | load | 4 | `abs 0xe4d84` | `-` |
| `ldr` at 0xe3de4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe3df4 | load | 4 | `abs 0xe4d54` | `-` |
| `ldr` at 0xe3dfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3e08 | load | 4 | `abs 0xe4d84` | `-` |
| `ldr` at 0xe3e0c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe3e1c | load | 4 | `abs 0xe4d58` | `-` |
| `ldr` at 0xe3e24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3e2c | load | 4 | `abs 0xe4d80` | `-` |
| `strb` at 0xe3e34 | store | 1 | `abs 0x2b4528` | `0x0` |
| `ldr` at 0xe3e38 | load | 4 | `abs 0xe4d5c` | `-` |
| `ldr` at 0xe3e3c | load | 4 | `abs 0x2adfd4` | `-` |
| `str` at 0xe3e4c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe3e50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3e54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3e60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3e64 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe3e70 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0xe3e78 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe3e84 | load | 4 | `abs 0xe4d60` | `-` |
| `ldr` at 0xe3e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3e94 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe3ea4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3eac | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe3eb8 | load | 4 | `abs 0x14385c` | `-` |
| `ldr` at 0xe3edc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3ee4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe3ef4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3efc | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xe3f08 | load | 4 | `abs 0xe4d80` | `-` |
| `ldr` at 0xe3f0c | load | 4 | `abs 0xe4d64` | `-` |
| `ldr` at 0xe3f1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3f24 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe3f2c | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0xe3f34 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xe3f3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3f40 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xe3f54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5668 | load | 4 | `abs 0xe4d80` | `-` |
| `ldr` at 0xe566c | load | 4 | `abs 0xe4d78` | `-` |
| `str` at 0xe5678 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe5680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5684 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5694 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe56a0 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xe56a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe56b0 | load | 4 | `abs 0xe4d84` | `-` |
| `ldr` at 0xe56b4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe56c0 | load | 4 | `abs 0xe4d80` | `-` |
| `ldrb` at 0xe56c4 | load | 1 | `abs 0x2b4528` | `-` |
| `ldr` at 0xe56d0 | load | 4 | `abs 0xe4d7c` | `-` |
| `ldr` at 0xe56d4 | load | 4 | `abs 0xe4d80` | `-` |
| `ldr` at 0xe56dc | load | 4 | `abs 0xe4d84` | `-` |
| `ldr` at 0xe56e0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe56f0 | load | 4 | `abs 0xe4d88` | `-` |
| `ldr` at 0xe56f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5704 | load | 4 | `abs 0xe4d8c` | `-` |
| `ldr` at 0xe5708 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe5718 | load | 4 | `abs 0xe4d90` | `-` |
| `ldr` at 0xe5720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5728 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe572c | store | 4 | `sp -0x5c` | `[sp-28]` |
| `ldr` at 0xe5730 | load | 4 | `sp -0x5c` | `-` |
| `ldmdb` at 0xe5734 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe5734 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe5734 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe5734 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3e00 | `(unknown callee) 0xdcd50` | `0x0` | `0x13625c` | `0x1` | `r0` |
| `bl` at 0xe3e28 | `(unknown callee) 0xdcd50` | `0x0` | `0x136280` | `0x0` | `r0` |
| `bl` at 0xe3e80 | `(unknown callee) 0xe3cf8` | `?` | `?` | `0x2b4528` | `[0x2adfd4]` |
| `mov` at 0xe3ec8 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x14385c]` | `?` |
| `bl` at 0xe3f14 | `(unknown callee) 0x1b52c` | `0x2b4528` | `0x1362a4` | `?` | `?` |
| `bl` at 0xe3f18 | `(unknown callee) 0xe3d64` | `ret(0x1b52c)` | `?` | `?` | `?` |
| `bl` at 0xe5670 | `(unknown callee) 0x1b598` | `0x2b4528` | `0x1362e8` | `?` | `?` |
| `bl` at 0xe567c | `(unknown callee) 0xe3d64` | `ret(0x1b598)` | `?` | `?` | `0x0` |
| `bl` at 0xe56d8 | `(unknown callee) 0x23fbc` | `0x136314` | `0x2b4528` | `?` | `[0x2b4528]:1` |
| `bl` at 0xe56fc | `(unknown callee) 0xdcd50` | `0x0` | `0x136324` | `0x1` | `[sp-24]` |
| `bl` at 0xe5724 | `(unknown callee) 0xdcd50` | `0x0` | `0x13634c` | `0x0` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xe3dec | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xe3e14 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xe3e44 | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |
| `beq` at 0xe3ed4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `ldrls` at 0xe3f54 | `? <= 0x14 (unsigned, 32-bit)` | `? > 0x14 (unsigned, 32-bit)` |
| `bne` at 0xe56ac | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xe56bc | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0xe56cc | `[0x2b4528]:1 == 0x0` | `[0x2b4528]:1 != 0x0` |
| `bls` at 0xe56e8 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xe5710 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe5734 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xe3f54).

## `sn1sc_dr_load`

- `sn1sc_dr_load`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5a00 to 0xe5ccc.
  Established. The branch takes `[r0+16] == 0x0` to `0xe5ac0` (`beq` at 0xe5a30). One included call
  to unnamed `0xdd8e8` has `0x0`, unknown, `0x1`, unknown (`bl` at 0xe5a9c). Two later calls to
  that target have `0x1`, `0x1`, `[sp-28]`, unknown, and `0x1`, `0x6`, `0x1`, unknown,
  respectively (`bl` at 0xe5bc4, `bl` at 0xe5c88). Four-byte stack stores
  at `sp -0x38` retain `0x2b4528` at three sites (`str` at 0xe5a8c, `str` at 0xe5bb0, `str` at
  0xe5c74). Scan-data transfer and scratch-record semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5a10 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe5a18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5a20 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xe5a24 | store | 4 | `sp -0x20` | `[r0+16]` |
| `ldr` at 0xe5a28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5a34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5a3c | store | 4 | `sp -0x28` | `r0+24` |
| `ldr` at 0xe5a44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5a4c | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe5a5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5a64 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe5a70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe5a74 | store | 4 | `sp -0x44` | `[r0+16]` |
| `ldr` at 0xe5a78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5a7c | store | 4 | `sp -0x40` | `r0+24` |
| `ldr` at 0xe5a80 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5a84 | store | 4 | `sp -0x3c` | `r0+24` |
| `ldr` at 0xe5a88 | load | 4 | `abs 0xe5cc8` | `-` |
| `str` at 0xe5a8c | store | 4 | `sp -0x38` | `0x2b4528` |
| `str` at 0xe5aa4 | store | 4 | `sp -0x18` | `ret(0xdd8e8)` |
| `ldr` at 0xe5aa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5ab8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe5ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5acc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5ad0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5ad4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5ae4 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe5aec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5af0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5b08 | store | 4 | `sp -0x1c` | `0x6` |
| `str` at 0xe5b14 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xe5b18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5b28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5b34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5b3c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe5b40 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe5b48 | store | 4 | `sp -0x34` | `[sp-52]+24` |
| `ldr` at 0xe5b50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5b5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5b64 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe5b68 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe5b70 | store | 4 | `sp -0x34` | `[sp-52]+28` |
| `ldr` at 0xe5b74 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe5b78 | store | 4 | `sp -0x28` | `[sp-52]` |
| `ldr` at 0xe5b80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5b88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5b94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5b98 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe5b9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5ba0 | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0xe5ba4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5ba8 | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0xe5bac | load | 4 | `abs 0xe5cc8` | `-` |
| `str` at 0xe5bb0 | store | 4 | `sp -0x38` | `0x2b4528` |
| `ldr` at 0xe5bbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe5bcc | store | 4 | `sp -0x18` | `ret(0xdd8e8)` |
| `ldr` at 0xe5bd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5bd4 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xe5bec | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe5bf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5c0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5c28 | store | 4 | `sp -0x2c` | `ret(0x1b1b4)` |
| `ldr` at 0xe5c2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5c38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5c3c | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xe5c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5c4c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xe5c58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5c5c | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xe5c60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5c64 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0xe5c68 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe5c6c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe5c70 | load | 4 | `abs 0xe5cc8` | `-` |
| `str` at 0xe5c74 | store | 4 | `sp -0x38` | `0x2b4528` |
| `str` at 0xe5c90 | store | 4 | `sp -0x18` | `ret(0xdd8e8)` |
| `ldr` at 0xe5c94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5c98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5ca8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe5cb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe5cb8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5cbc | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0xe5cc0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xe5cc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe5cc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe5cc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5a9c | `(unknown callee) 0xdd8e8` | `0x0` | `?` | `0x1` | `?` |
| `bl` at 0xe5bc4 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `[sp-28]` | `?` |
| `bl` at 0xe5c20 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xe5c88 | `(unknown callee) 0xdd8e8` | `0x1` | `0x6` | `0x1` | `?` |
| `bl` at 0xe5cb4 | `(unknown callee) 0x1b490` | `[sp-44]` | `?` | `[sp-44]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5a30 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bne` at 0xe5ab0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xe5adc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe5b00 | `? != 0x1` | `? == 0x1` |
| `bne` at 0xe5b24 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xe5be4 | `? == 0x1` | `? != 0x1` |
| `bne` at 0xe5c34 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xe5ca0 | `[sp-44] != [sp-40]` | `[sp-44] == [sp-40]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe5cc4 | `r0` | `?` |

## `sn1sc_reset_brick`

- `sn1sc_reset_brick`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5ccc to 0xe5db4.
  Established. The branch takes `[0x2adfd4] != 0x0` to `0xe5d30` (`bne` at 0xe5d14). The other arm
  writes four-byte `0xfffff7cc` at an unknown effective address (`str` at 0xe5d20). The
  unknown-target call has unknown, `0x1`, `0x2b4528`, `[0x2af36c]`, and its result is assigned as
  four bytes at `[sp-24] +0x0` (`mov` at 0xe5d5c, `str` at 0xe5d68). Brick-reset effects and
  callback meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5cdc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5ce0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe5ce8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe5cec | load | 4 | `abs 0xe5d9c` | `-` |
| `ldr` at 0xe5cf0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe5cfc | load | 4 | `abs 0xe5da0` | `-` |
| `ldr` at 0xe5d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5d08 | load | 4 | `abs 0xe5da4` | `-` |
| `ldr` at 0xe5d0c | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe5d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5d1c | load | 4 | `abs 0xe5da8` | `-` |
| `str` at 0xe5d20 | store | 4 | `(unknown)` | `0xfffff7cc` |
| `str` at 0xe5d28 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe5d34 | load | 4 | `abs 0xe5dac` | `-` |
| `ldr` at 0xe5d38 | load | 4 | `abs 0x2af36c` | `-` |
| `ldr` at 0xe5d44 | load | 4 | `abs 0xe5dac` | `-` |
| `ldr` at 0xe5d48 | load | 4 | `abs 0x2af36c` | `-` |
| `ldr` at 0xe5d54 | load | 4 | `abs 0xe5db0` | `-` |
| `ldr` at 0xe5d64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5d68 | store | 4 | `[sp-24] +0x0` | `ret(unknown)` |
| `ldr` at 0xe5d6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5d70 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe5d80 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xe5d90 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xe5d94 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xe5d98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe5d98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe5d98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5d04 | `(unknown callee) 0x23fbc` | `0x13645c` | `r0` | `r2` | `[0x2adfd8]` |
| `bl` at 0xe5d30 | `(unknown callee) 0xe5db4` | `?` | `?` | `?` | `[0x2adfd4]` |
| `mov` at 0xe5d5c | `(unknown callee) (unknown target)` | `?` | `0x1` | `0x2b4528` | `[0x2af36c]` |
| `bl` at 0xe5d88 | `(unknown callee) 0xe5db4` | `ret(0xe5db4)` | `?` | `?` | `[0x2af36c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5cf8 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bne` at 0xe5d14 | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |
| `beq` at 0xe5d40 | `[0x2af36c] == 0x0` | `[0x2af36c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe5d98 | `r0` | `?` |

## `sn1sc_reset_sic`

- `sn1sc_reset_sic`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5db4 to 0xe623c.
  Established. The branch takes `[0x2adfd4] != 0x0` to `0xe5df8` (`bne` at 0xe5de8). Six halfword
  stores on that path target `[0x2b4520] +0x0` with unknown values (`strh` at 0xe5f80, `strh` at
  0xe6008, `strh` at 0xe608c, `strh` at 0xe6110, `strh` at 0xe6194, `strh` at 0xe6218). Included
  four-byte stack assignments at `sp -0x14` retain `0x38`, `0x28`, `0x38` and `0x30` at distinct
  sites (`str` at 0xe6010, `str` at 0xe6094, `str` at 0xe6118, `str` at 0xe619c). Controller-reset
  signaling and output encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe5dc4 | load | 4 | `abs 0xe622c` | `-` |
| `ldr` at 0xe5dc8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe5dd4 | load | 4 | `abs 0xe6230` | `-` |
| `ldr` at 0xe5ddc | load | 4 | `abs 0xe6234` | `-` |
| `ldr` at 0xe5de0 | load | 4 | `abs 0x2adfd4` | `-` |
| `str` at 0xe5df0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe5df8 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5dfc | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5e00 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe5e1c | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5e20 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5e24 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe5e48 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5e4c | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5e50 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe5e74 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5e78 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5e7c | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe5ea0 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5ea4 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5ea8 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe5ecc | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5ed0 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5ed4 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `str` at 0xe5ef8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe5efc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5f04 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe5f08 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5f0c | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5f10 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5f1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5f44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5f58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5f6c | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe5f80 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xe5f84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5f8c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe5f90 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe5f94 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe5f98 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5fa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5fb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5fcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5fe0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5ff4 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe6008 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xe6010 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xe6014 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe6018 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe601c | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe6028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe603c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6050 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6078 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe608c | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xe6094 | store | 4 | `sp -0x14` | `0x28` |
| `ldr` at 0xe6098 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe609c | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe60a0 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe60ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe60c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe60d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe60e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe60fc | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe6110 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xe6118 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xe611c | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe6120 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6124 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe6130 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6144 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe616c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6180 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe6194 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xe619c | store | 4 | `sp -0x14` | `0x30` |
| `ldr` at 0xe61a0 | load | 4 | `abs 0xe6238` | `-` |
| `ldr` at 0xe61a4 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe61a8 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe61b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe61c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe61dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe61f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6204 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe6218 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xe6220 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xe6224 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xe6228 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6228 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6228 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5dd8 | `(unknown callee) 0x23fbc` | `0x13647c` | `r1` | `r2` | `[0x2adfd8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5dd0 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bne` at 0xe5de8 | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6228 | `r0` | `?` |

## `sn1sc_get_tap_val`

- `sn1sc_get_tap_val`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe623c to 0xe6360.
  Established. Six halfword reads target `[0x2b4520] +0x0` at distinct instructions (`ldrh` at
  0xe6254, `ldrh` at 0xe6278, `ldrh` at 0xe62a4, `ldrh` at 0xe62d0, `ldrh` at 0xe62fc, `ldrh` at
  0xe6328). The explicit four-byte stack store at `sp -0x14` has unknown value (`str` at 0xe634c).
  The exit carries `[sp-20]` (`ldmdb` at 0xe6358). TAP-value encoding and whether successive reads
  agree remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe624c | load | 4 | `abs 0xe635c` | `-` |
| `ldr` at 0xe6250 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6254 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6270 | load | 4 | `abs 0xe635c` | `-` |
| `ldr` at 0xe6274 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6278 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe629c | load | 4 | `abs 0xe635c` | `-` |
| `ldr` at 0xe62a0 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe62a4 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe62c8 | load | 4 | `abs 0xe635c` | `-` |
| `ldr` at 0xe62cc | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe62d0 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe62f4 | load | 4 | `abs 0xe635c` | `-` |
| `ldr` at 0xe62f8 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe62fc | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6320 | load | 4 | `abs 0xe635c` | `-` |
| `ldr` at 0xe6324 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6328 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `str` at 0xe634c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe6350 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe6358 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6358 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6358 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6358 | `r0` | `[sp-20]` |

## `sn1sc_get_tap_text`

- `sn1sc_get_tap_text`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6360 to 0xe6590.
  Established. Included halfword reads use `[0x2b4520] +0x0` (`ldrh` at 0xe637c, `ldrh` at 0xe63a0,
  `ldrh` at 0xe63cc, `ldrh` at 0xe63f8, `ldrh` at 0xe6424, `ldrh` at 0xe6450). Four-byte stack
  assignments at `sp -0x1c` retain `0x31` and `0x30` on separate paths (`str` at 0xe648c, `str` at
  0xe6498). The unnamed `0x1b52c` call has `r0`, `0x136490`, `[sp-28]`, `[sp-32]` (`bl` at 0xe6580).
  Signal-to-text interpretation is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6370 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe6374 | load | 4 | `abs 0xe6588` | `-` |
| `ldr` at 0xe6378 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe637c | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6398 | load | 4 | `abs 0xe6588` | `-` |
| `ldr` at 0xe639c | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe63a0 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe63c4 | load | 4 | `abs 0xe6588` | `-` |
| `ldr` at 0xe63c8 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe63cc | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe63f0 | load | 4 | `abs 0xe6588` | `-` |
| `ldr` at 0xe63f4 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe63f8 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe641c | load | 4 | `abs 0xe6588` | `-` |
| `ldr` at 0xe6420 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6424 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6448 | load | 4 | `abs 0xe6588` | `-` |
| `ldr` at 0xe644c | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6450 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `str` at 0xe6474 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe6478 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe648c | store | 4 | `sp -0x1c` | `0x31` |
| `str` at 0xe6498 | store | 4 | `sp -0x1c` | `0x30` |
| `ldr` at 0xe649c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe64b0 | store | 4 | `sp -0x20` | `0x31` |
| `str` at 0xe64bc | store | 4 | `sp -0x20` | `0x30` |
| `ldr` at 0xe64c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe64d4 | store | 4 | `sp -0x24` | `0x31` |
| `str` at 0xe64e0 | store | 4 | `sp -0x24` | `0x30` |
| `ldr` at 0xe64e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe64e8 | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0xe64ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6500 | store | 4 | `sp -0x28` | `0x31` |
| `str` at 0xe650c | store | 4 | `sp -0x28` | `0x30` |
| `ldr` at 0xe6510 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe6514 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldr` at 0xe6518 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe652c | store | 4 | `sp -0x2c` | `0x31` |
| `str` at 0xe6538 | store | 4 | `sp -0x2c` | `0x30` |
| `ldr` at 0xe653c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe6540 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0xe6544 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6558 | store | 4 | `sp -0x30` | `0x31` |
| `str` at 0xe6564 | store | 4 | `sp -0x30` | `0x30` |
| `ldr` at 0xe6568 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe656c | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xe6570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6574 | load | 4 | `abs 0xe658c` | `-` |
| `ldr` at 0xe6578 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe657c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xe6584 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6584 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6584 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6580 | `(unknown callee) 0x1b52c` | `r0` | `0x136490` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe6484 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe64a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe64cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe64f8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe6524 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe6550 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6584 | `r0` | `ret(0x1b52c)` |

## `sn1sc_set_tap_val`

- `sn1sc_set_tap_val`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6590 to 0xe6628.
  Established. A halfword assignment at `[0x2b4520] +0x0` has unknown value (`strh` at 0xe661c). The
  prefix saves four-byte entry `r0` at `sp -0x14`, and the exit preserves entry `r0` (`str` at
  0xe65a0, `ldmdb` at 0xe6620). This span has no calls or conditional branches. TAP-output bit
  encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe65a0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe65a4 | load | 4 | `abs 0xe6624` | `-` |
| `ldr` at 0xe65a8 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe65ac | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe65b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe65cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe65e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe65f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe6608 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe661c | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldmdb` at 0xe6620 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6620 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6620 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6620 | `r0` | `r0` |

## `sn1sc_drive_tap_fixed_tms`

- `sn1sc_drive_tap_fixed_tms`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe6628 to 0xe688c.
  Established. The first branch takes entry `r1 != 0x0` to `0xe6660` (`bne` at 0xe6650). A later
  halfword store targets `[0x2b4520] +0x0` with unknown value (`strh` at 0xe6820). Separate unnamed
  calls to `0xf7cc4` and `0xf7e64` each have `sp-52`, unknown, unknown, `sp-52` (`bl` at 0xe6864,
  `bl` at 0xe6874). The exit carries `0x1` (`ldmdb` at 0xe6884). Fixed-state TAP signaling remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe6638 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe663c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe6640 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe6644 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe6648 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6658 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe6660 | load | 4 | `abs 0xe6888` | `-` |
| `ldr` at 0xe6664 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6668 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6684 | load | 4 | `abs 0xe6888` | `-` |
| `ldr` at 0xe6688 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe668c | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe66b0 | load | 4 | `abs 0xe6888` | `-` |
| `ldr` at 0xe66b4 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe66b8 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe66dc | load | 4 | `abs 0xe6888` | `-` |
| `ldr` at 0xe66e0 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe66e4 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6708 | load | 4 | `abs 0xe6888` | `-` |
| `ldr` at 0xe670c | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6710 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6734 | load | 4 | `abs 0xe6888` | `-` |
| `ldr` at 0xe6738 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe673c | load | 2 | `[0x2b4520] +0x0` | `-` |
| `str` at 0xe6760 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe6764 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe676c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe677c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe678c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe6794 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe679c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe67a4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe67a8 | load | 4 | `abs 0xe6888` | `-` |
| `ldr` at 0xe67ac | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe67b0 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xe67bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe67d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe67e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe67f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe680c | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xe6820 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xe6824 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6828 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe682c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6830 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe6834 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe6840 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6844 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6848 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe684c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe6850 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe687c | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe6880 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xe6884 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6884 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6884 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6864 | `(unknown callee) 0xf7cc4` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xe6874 | `(unknown callee) 0xf7e64` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe6650 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0xe6778 | `? == ?` | `? != ?` |
| `beq` at 0xe6788 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe683c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe6858 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6884 | `r0` | `0x1` |

## `sn1sc_drive_tap`

- `sn1sc_drive_tap`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe688c to 0xe6ab4.
  Established. Six halfword reads target `[0x2b4520] +0x0` at distinct sites (`ldrh` at 0xe68b4,
  `ldrh` at 0xe68d8, `ldrh` at 0xe6904, `ldrh` at 0xe6930, `ldrh` at 0xe695c, `ldrh` at 0xe6988). A
  halfword store at that target has unknown value (`strh` at 0xe6a6c). The unnamed `0xf7ee4` call
  carries `sp-52`, unknown, unknown, `sp-52`, followed by the `0x1` exit (`bl` at 0xe6aa0, `ldmdb`
  at 0xe6aac). TAP transitions and record-field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe689c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe68a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe68a4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe68a8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe68ac | load | 4 | `abs 0xe6ab0` | `-` |
| `ldr` at 0xe68b0 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe68b4 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe68d0 | load | 4 | `abs 0xe6ab0` | `-` |
| `ldr` at 0xe68d4 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe68d8 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe68fc | load | 4 | `abs 0xe6ab0` | `-` |
| `ldr` at 0xe6900 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6904 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6928 | load | 4 | `abs 0xe6ab0` | `-` |
| `ldr` at 0xe692c | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6930 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6954 | load | 4 | `abs 0xe6ab0` | `-` |
| `ldr` at 0xe6958 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe695c | load | 2 | `[0x2b4520] +0x0` | `-` |
| `ldr` at 0xe6980 | load | 4 | `abs 0xe6ab0` | `-` |
| `ldr` at 0xe6984 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe6988 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `str` at 0xe69ac | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe69b0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe69b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe69c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe69d8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe69e0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe69e8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe69f0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe69f4 | load | 4 | `abs 0xe6ab0` | `-` |
| `ldr` at 0xe69f8 | load | 4 | `abs 0x2b4520` | `-` |
| `ldrh` at 0xe69fc | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xe6a08 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6a1c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6a30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6a44 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6a58 | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xe6a6c | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xe6a70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe6a74 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe6a78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe6a7c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe6a80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe6a84 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe6a88 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe6a8c | store | 4 | `sp -0x28` | `[sp]` |
| `ldr` at 0xe6a90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe6a94 | store | 4 | `sp -0x24` | `?` |
| `ldmdb` at 0xe6aac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6aac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6aac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6aa0 | `(unknown callee) 0xf7ee4` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe69c4 | `? == ?` | `? != ?` |
| `beq` at 0xe69d4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6aac | `r0` | `0x1` |

## `sn1sc_asm_drive_tap_fixed_tms`

- `sn1sc_asm_drive_tap_fixed_tms`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7cc4 to 0xf7e64.
  Established. Six included halfword stores target `[0x2b4520] +0x0` with unknown values (`strh` at
  0xf7d70, `strh` at 0xf7d8c, `strh` at 0xf7da4, `strh` at 0xf7e10, `strh` at 0xf7e2c, `strh` at
  0xf7e44). Halfword reads use the same effective target at two other instructions (`ldrh` at
  0xf7d84, `ldrh` at 0xf7e24). The signed-less branch takes `0x1f < ?` to `0xf7e54`, where the
  comparison operand remains unknown (`blt` at 0xf7df4). TAP signaling and sampled-bit
  interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf7cd0 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xf7cd4 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xf7cdc | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf7ce0 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf7d38 | load | 4 | `abs 0xf7e60` | `-` |
| `ldr` at 0xf7d3c | load | 4 | `abs 0x2b4520` | `-` |
| `ldr` at 0xf7d48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7d4c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf7d64 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7d6c | store | 4 | `sp -0x28` | `?` |
| `strh` at 0xf7d70 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7d78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7d80 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf7d84 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `strh` at 0xf7d8c | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7d98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7da0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf7da4 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xf7db4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf7dcc | load | 4 | `abs 0xf7e60` | `-` |
| `ldr` at 0xf7dd0 | load | 4 | `abs 0x2b4520` | `-` |
| `ldr` at 0xf7dd4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7de4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf7e04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7e0c | store | 4 | `sp -0x28` | `?` |
| `strh` at 0xf7e10 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7e18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7e20 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf7e24 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `strh` at 0xf7e2c | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7e38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7e40 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf7e44 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xf7e54 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xf7d2c | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0xf7db0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xf7dc0 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf7dc8 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xf7df4 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0xf7e50 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0xf7e5c | `r0` | `?` |

## `sn1sc_asm_drive_tck`

- `sn1sc_asm_drive_tck`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7e64 to 0xf7ee4.
  Established. Two halfword stores write unknown values at `[0x2b4520] +0x0` (`strh` at 0xf7ec0,
  `strh` at 0xf7ecc). The first conditional return takes `0x0 >= [r0+4]`, unsigned 32-bit, and
  carries `0x0` (`pophs` at 0xf7eb4). The later conditional return uses `C == 1` with unknown return
  value (`pophs` at 0xf7ed8). Clock signaling and iteration count remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf7e64 | store | 4 | `sp -0x4` | `lr` |
| `ldm` at 0xf7e68 | load | 4 | `r0 +0x0` | `-` |
| `ldm` at 0xf7e68 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xf7eb8 | load | 4 | `abs 0xf7ee0` | `-` |
| `ldr` at 0xf7ebc | load | 4 | `abs 0x2b4520` | `-` |
| `strh` at 0xf7ec0 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7ec8 | load | 4 | `abs 0x2b4520` | `-` |
| `strh` at 0xf7ecc | store | 2 | `[0x2b4520] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `pophs` at 0xf7eb4 | `0x0 >= [r0+4] (unsigned, 32-bit)` | `0x0 < [r0+4] (unsigned, 32-bit)` |
| `pophs` at 0xf7ed8 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `pophs` at 0xf7eb4 | `r0` | `0x0` |
| `pophs` at 0xf7ed8 | `r0` | `?` |

## `sn1sc_asm_drive_tap`

- `sn1sc_asm_drive_tap`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7ee4 to 0xf80a0.
  Established. Included halfword assignments at `[0x2b4520] +0x0` have unknown values at six
  instructions (`strh` at 0xf7f9c, `strh` at 0xf7fb8, `strh` at 0xf7fd0, `strh` at 0xf804c, `strh`
  at 0xf8068, `strh` at 0xf8080). Halfword reads through that target occur at two other sites
  (`ldrh` at 0xf7fb0, `ldrh` at 0xf8060). A four-byte stack store at `sp -0x2c` retains `[r0+4]`
  (`str` at 0xf7ef8). TAP state and sampled-data meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf7ef0 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xf7ef4 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xf7ef8 | store | 4 | `sp -0x2c` | `[r0+4]` |
| `ldr` at 0xf7f00 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf7f04 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf7f08 | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xf7f60 | load | 4 | `abs 0xf809c` | `-` |
| `ldr` at 0xf7f64 | load | 4 | `abs 0x2b4520` | `-` |
| `str` at 0xf7f68 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xf7f70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf7f74 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf7f78 | store | 4 | `sp -0x28` | `0x0` |
| `strh` at 0xf7f9c | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7fa4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7fac | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf7fb0 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `strh` at 0xf7fb8 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7fc4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf7fcc | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf7fd0 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf7fe0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf7fe4 | store | 4 | `[sp-48] +0x0` | `?` |
| `ldr` at 0xf7fec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf7ff8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf8004 | load | 4 | `abs 0xf809c` | `-` |
| `ldr` at 0xf8008 | load | 4 | `abs 0x2b4520` | `-` |
| `ldr` at 0xf800c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf8010 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf8020 | store | 4 | `sp -0x28` | `0x0` |
| `strh` at 0xf804c | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf8054 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf805c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldrh` at 0xf8060 | load | 2 | `[0x2b4520] +0x0` | `-` |
| `strh` at 0xf8068 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `ldr` at 0xf8074 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf807c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `strh` at 0xf8080 | store | 2 | `[0x2b4520] +0x0` | `?` |
| `str` at 0xf8090 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xf7f54 | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0xf7fdc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xf7ff4 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf8000 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xf8030 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0xf808c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0xf8098 | `r0` | `?` |

## `setup_tdrdata_shub`

- `setup_tdrdata_shub`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf22b8 to 0xf28c0.
  Established. The unnamed `0x1b1b4` call carries `0x3c` followed by three unknown candidates, with
  its result assigned as four bytes at `[sp-24] +0x28` (`bl` at 0xf22e4, `str` at 0xf22ec). Included
  unnamed `0xf1164` calls use `0x1382d8`, `0x7c`, `0x145584`, unknown, and `0x138384`, `0x63`,
  `0x145748`, unknown (`bl` at 0xf2358, `bl` at 0xf27f0). A later four-byte store at
  `[sp-24] +0x24` retains `[sp-32]+1` (`str` at 0xf2840). Scan-record layout and ownership are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf22c8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf22d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf22d4 | load | 4 | `abs 0xf2854` | `-` |
| `ldr` at 0xf22dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf22ec | store | 4 | `[sp-24] +0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xf22f0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2300 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xf230c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf2310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf232c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2338 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf233c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf2344 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf234c | load | 4 | `abs 0xf2858` | `-` |
| `ldr` at 0xf2354 | load | 4 | `abs 0xf285c` | `-` |
| `ldr` at 0xf2360 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2368 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2374 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf237c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2384 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2388 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2398 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf23a0 | load | 4 | `abs 0xf2860` | `-` |
| `ldr` at 0xf23a8 | load | 4 | `abs 0xf2864` | `-` |
| `ldr` at 0xf23b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf23bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf23c8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf23d0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf23d8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf23dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf23ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf23f4 | load | 4 | `abs 0xf2868` | `-` |
| `ldr` at 0xf23fc | load | 4 | `abs 0xf286c` | `-` |
| `ldr` at 0xf2408 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2410 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf241c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2424 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf242c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2430 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2440 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2448 | load | 4 | `abs 0xf2870` | `-` |
| `ldr` at 0xf2450 | load | 4 | `abs 0xf2874` | `-` |
| `ldr` at 0xf245c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2464 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2470 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2478 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2480 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2484 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2494 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf249c | load | 4 | `abs 0xf2878` | `-` |
| `ldr` at 0xf24a4 | load | 4 | `abs 0xf287c` | `-` |
| `ldr` at 0xf24b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf24b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf24c4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf24cc | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf24d4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf24d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf24e8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf24f0 | load | 4 | `abs 0xf2880` | `-` |
| `ldr` at 0xf24f8 | load | 4 | `abs 0xf2884` | `-` |
| `ldr` at 0xf2504 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf250c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2518 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2520 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2528 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf252c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf253c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2544 | load | 4 | `abs 0xf2888` | `-` |
| `ldr` at 0xf254c | load | 4 | `abs 0xf288c` | `-` |
| `ldr` at 0xf2558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2560 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf256c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2574 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf257c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2580 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2590 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2598 | load | 4 | `abs 0xf2890` | `-` |
| `ldr` at 0xf25a0 | load | 4 | `abs 0xf2894` | `-` |
| `ldr` at 0xf25ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf25b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf25c0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf25c8 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf25d0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf25d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf25e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf25ec | load | 4 | `abs 0xf2898` | `-` |
| `ldr` at 0xf25f4 | load | 4 | `abs 0xf2894` | `-` |
| `ldr` at 0xf2600 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2608 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2614 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf261c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2624 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2628 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2638 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2640 | load | 4 | `abs 0xf289c` | `-` |
| `ldr` at 0xf2648 | load | 4 | `abs 0xf28a0` | `-` |
| `ldr` at 0xf2654 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf265c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2668 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2670 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2678 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf267c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf268c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2694 | load | 4 | `abs 0xf28a4` | `-` |
| `ldr` at 0xf269c | load | 4 | `abs 0xf28a0` | `-` |
| `ldr` at 0xf26a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf26b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf26bc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf26c4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf26cc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf26d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf26e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf26e8 | load | 4 | `abs 0xf28a8` | `-` |
| `ldr` at 0xf26f0 | load | 4 | `abs 0xf28ac` | `-` |
| `ldr` at 0xf26fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2704 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2710 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2718 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2720 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2724 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2734 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf273c | load | 4 | `abs 0xf28b0` | `-` |
| `ldr` at 0xf2744 | load | 4 | `abs 0xf28ac` | `-` |
| `ldr` at 0xf2750 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2758 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2764 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf276c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf2774 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2778 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2788 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2790 | load | 4 | `abs 0xf28b4` | `-` |
| `ldr` at 0xf2798 | load | 4 | `abs 0xf28b8` | `-` |
| `ldr` at 0xf27a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf27ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf27b8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf27c0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf27c8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf27cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf27dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf27e4 | load | 4 | `abs 0xf28bc` | `-` |
| `ldr` at 0xf27ec | load | 4 | `abs 0xf28b8` | `-` |
| `ldr` at 0xf27f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2800 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf280c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf2814 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf281c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf2820 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf2830 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf283c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf2840 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf2848 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf284c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf2850 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf2850 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf2850 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf2850 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf22d8 | `(unknown callee) 0x23fbc` | `0x1382c0` | `r1` | `r2` | `0x0` |
| `bl` at 0xf22e4 | `(unknown callee) 0x1b1b4` | `0x3c` | `?` | `?` | `?` |
| `bl` at 0xf2358 | `(unknown callee) 0xf1164` | `0x1382d8` | `0x7c` | `0x145584` | `?` |
| `bl` at 0xf23ac | `(unknown callee) 0xf1164` | `0x1382e4` | `0x7d` | `0x14559c` | `?` |
| `bl` at 0xf2400 | `(unknown callee) 0xf1164` | `0x1382f0` | `0x7f` | `0x1455b4` | `?` |
| `bl` at 0xf2454 | `(unknown callee) 0xf1164` | `0x1382f8` | `0x7e` | `0x143944` | `?` |
| `bl` at 0xf24a8 | `(unknown callee) 0xf1164` | `0x138300` | `0x7` | `0x1439d4` | `?` |
| `bl` at 0xf24fc | `(unknown callee) 0xf1164` | `0x138308` | `0x5` | `0x143a10` | `?` |
| `bl` at 0xf2550 | `(unknown callee) 0xf1164` | `0x138314` | `0x6` | `0x143bcc` | `?` |
| `bl` at 0xf25a4 | `(unknown callee) 0xf1164` | `0x13831c` | `0xa` | `0x143c74` | `?` |
| `bl` at 0xf25f8 | `(unknown callee) 0xf1164` | `0x138328` | `0xb` | `0x143c74` | `?` |
| `bl` at 0xf264c | `(unknown callee) 0xf1164` | `0x138334` | `0x1a` | `0x1456d0` | `?` |
| `bl` at 0xf26a0 | `(unknown callee) 0xf1164` | `0x138344` | `0x1b` | `0x1456d0` | `?` |
| `bl` at 0xf26f4 | `(unknown callee) 0xf1164` | `0x138354` | `0x60` | `0x143cec` | `?` |
| `bl` at 0xf2748 | `(unknown callee) 0xf1164` | `0x138364` | `0x62` | `0x143cec` | `?` |
| `bl` at 0xf279c | `(unknown callee) 0xf1164` | `0x138374` | `0x61` | `0x145748` | `?` |
| `bl` at 0xf27f0 | `(unknown callee) 0xf1164` | `0x138384` | `0x63` | `0x145748` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf22f8 | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bls` at 0xf2318 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf2390 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf23e4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2438 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf248c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf24e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2534 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2588 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf25dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2630 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2684 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf26d8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf272c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2780 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf27d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf2828 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf2850 | `r0` | `[sp-36]` |

## `setup_tdrdata_shub2`

- `setup_tdrdata_shub2`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf36e8 to 0xf407c.
  Established. The unnamed `0x1b1b4` call has `0x64`, `r1`, `r2`, `0x0`, and its result is stored as
  four bytes at `r0 +0x28` (`bl` at 0xf370c, `str` at 0xf3714). Nonzero `[r0+40]` takes the branch
  to `0xf3730` (`bne` at 0xf3720). Included unnamed `0xf1164` calls carry `0x13a18c`, `0x12`,
  `0x1455cc`, unknown, and `0x13a280`, `0x3`, `0x1457b8`, unknown (`bl` at 0xf39cc, `bl` at
  0xf3f60). Scan-data records and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf36f8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf3700 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf3704 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf3714 | store | 4 | `r0 +0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xf3718 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xf3728 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xf3734 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf3738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3748 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf374c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf3754 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3760 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf3764 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf376c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf3774 | load | 4 | `abs 0xf3fc4` | `-` |
| `ldr` at 0xf377c | load | 4 | `abs 0xf3fc8` | `-` |
| `ldr` at 0xf3788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3790 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf379c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf37a4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf37ac | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf37b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf37c0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf37c8 | load | 4 | `abs 0xf3fcc` | `-` |
| `ldr` at 0xf37d0 | load | 4 | `abs 0xf3fd0` | `-` |
| `ldr` at 0xf37dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf37e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf37f0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf37f8 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3800 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3804 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3814 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf381c | load | 4 | `abs 0xf3fd4` | `-` |
| `ldr` at 0xf3824 | load | 4 | `abs 0xf3fd8` | `-` |
| `ldr` at 0xf3830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3838 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3844 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf384c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3854 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3858 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3868 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3870 | load | 4 | `abs 0xf3fdc` | `-` |
| `ldr` at 0xf3878 | load | 4 | `abs 0xf3fe0` | `-` |
| `ldr` at 0xf3884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf388c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3898 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf38a0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf38a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf38ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xf38bc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf38c4 | load | 4 | `abs 0xf3fe4` | `-` |
| `ldr` at 0xf38cc | load | 4 | `abs 0xf3fe8` | `-` |
| `ldr` at 0xf38d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf38e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf38ec | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf38f4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf38fc | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3900 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3910 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3918 | load | 4 | `abs 0xf3fec` | `-` |
| `ldr` at 0xf3920 | load | 4 | `abs 0xf3ff0` | `-` |
| `ldr` at 0xf392c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3934 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3940 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3948 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3950 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3954 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3964 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf396c | load | 4 | `abs 0xf3ff4` | `-` |
| `ldr` at 0xf3974 | load | 4 | `abs 0xf3ff8` | `-` |
| `ldr` at 0xf3980 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3988 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3994 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf399c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf39a4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf39a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf39b8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf39c0 | load | 4 | `abs 0xf3ffc` | `-` |
| `ldr` at 0xf39c8 | load | 4 | `abs 0xf4000` | `-` |
| `ldr` at 0xf39d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf39dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf39e8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf39f0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf39f8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf39fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3a0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3a14 | load | 4 | `abs 0xf4004` | `-` |
| `ldr` at 0xf3a1c | load | 4 | `abs 0xf4008` | `-` |
| `ldr` at 0xf3a28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3a30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3a3c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3a44 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3a4c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3a50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3a60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3a68 | load | 4 | `abs 0xf400c` | `-` |
| `ldr` at 0xf3a70 | load | 4 | `abs 0xf4010` | `-` |
| `ldr` at 0xf3a7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3a84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3a90 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3a98 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3aa0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3aa4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3ab4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3abc | load | 4 | `abs 0xf4014` | `-` |
| `ldr` at 0xf3ac4 | load | 4 | `abs 0xf4010` | `-` |
| `ldr` at 0xf3ad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3ad8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3ae4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3aec | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3af4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3af8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3b08 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3b10 | load | 4 | `abs 0xf4018` | `-` |
| `ldr` at 0xf3b18 | load | 4 | `abs 0xf401c` | `-` |
| `ldr` at 0xf3b24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3b2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3b38 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3b40 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3b48 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3b4c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3b5c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3b64 | load | 4 | `abs 0xf4020` | `-` |
| `ldr` at 0xf3b6c | load | 4 | `abs 0xf401c` | `-` |
| `ldr` at 0xf3b78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3b80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3b8c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3b94 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3b9c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3ba0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3bb0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3bb8 | load | 4 | `abs 0xf4024` | `-` |
| `ldr` at 0xf3bc0 | load | 4 | `abs 0xf4028` | `-` |
| `ldr` at 0xf3bcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3bd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3be0 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3be8 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3bf0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3bf4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3c04 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3c0c | load | 4 | `abs 0xf402c` | `-` |
| `ldr` at 0xf3c14 | load | 4 | `abs 0xf4028` | `-` |
| `ldr` at 0xf3c20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3c28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3c34 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3c3c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3c44 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3c48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3c58 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3c60 | load | 4 | `abs 0xf4030` | `-` |
| `ldr` at 0xf3c68 | load | 4 | `abs 0xf4034` | `-` |
| `ldr` at 0xf3c74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3c7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3c88 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3c90 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3c98 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3c9c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3cac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3cb4 | load | 4 | `abs 0xf4038` | `-` |
| `ldr` at 0xf3cbc | load | 4 | `abs 0xf4034` | `-` |
| `ldr` at 0xf3cc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3cd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3cdc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3ce4 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3cec | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3cf0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3d00 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3d08 | load | 4 | `abs 0xf403c` | `-` |
| `ldr` at 0xf3d10 | load | 4 | `abs 0xf4040` | `-` |
| `ldr` at 0xf3d1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3d24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3d30 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3d38 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3d40 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3d44 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3d54 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3d5c | load | 4 | `abs 0xf4044` | `-` |
| `ldr` at 0xf3d64 | load | 4 | `abs 0xf4048` | `-` |
| `ldr` at 0xf3d70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3d78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3d84 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3d8c | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3d94 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3d98 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3da8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3db0 | load | 4 | `abs 0xf404c` | `-` |
| `ldr` at 0xf3db8 | load | 4 | `abs 0xf4050` | `-` |
| `ldr` at 0xf3dc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3dcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3dd8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3de0 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3de8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3dec | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3dfc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3e04 | load | 4 | `abs 0xf4054` | `-` |
| `ldr` at 0xf3e0c | load | 4 | `abs 0xf4058` | `-` |
| `ldr` at 0xf3e18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3e20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3e2c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3e34 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3e3c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3e40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3e50 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3e58 | load | 4 | `abs 0xf405c` | `-` |
| `ldr` at 0xf3e60 | load | 4 | `abs 0xf4060` | `-` |
| `ldr` at 0xf3e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3e74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3e80 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3e88 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3e90 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3e94 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3ea4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3eac | load | 4 | `abs 0xf4064` | `-` |
| `ldr` at 0xf3eb4 | load | 4 | `abs 0xf4068` | `-` |
| `ldr` at 0xf3ec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3ec8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3ed4 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3edc | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3ee4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3ee8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3ef8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3f00 | load | 4 | `abs 0xf406c` | `-` |
| `ldr` at 0xf3f08 | load | 4 | `abs 0xf4070` | `-` |
| `ldr` at 0xf3f14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3f1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3f28 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3f30 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3f38 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3f3c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3f4c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3f54 | load | 4 | `abs 0xf4074` | `-` |
| `ldr` at 0xf3f5c | load | 4 | `abs 0xf4078` | `-` |
| `ldr` at 0xf3f68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3f70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf3f7c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xf3f84 | store | 4 | `(unknown)` | `ret(0xf1164)` |
| `str` at 0xf3f8c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf3f90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf3fa0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf3fa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf3fac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf3fb0 | store | 4 | `[sp-24] +0x24` | `[sp-32]+1` |
| `str` at 0xf3fb8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf3fbc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf3fc0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf3fc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf3fc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf3fc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf370c | `(unknown callee) 0x1b1b4` | `0x64` | `r1` | `r2` | `0x0` |
| `bl` at 0xf3780 | `(unknown callee) 0xf1164` | `0x13a148` | `0x7c` | `0x145584` | `?` |
| `bl` at 0xf37d4 | `(unknown callee) 0xf1164` | `0x13a154` | `0x7d` | `0x14559c` | `?` |
| `bl` at 0xf3828 | `(unknown callee) 0xf1164` | `0x13a160` | `0x7f` | `0x1455b4` | `?` |
| `bl` at 0xf387c | `(unknown callee) 0xf1164` | `0x13a168` | `0x7e` | `0x144bf0` | `?` |
| `bl` at 0xf38d0 | `(unknown callee) 0xf1164` | `0x13a170` | `0x7` | `0x144c8c` | `?` |
| `bl` at 0xf3924 | `(unknown callee) 0xf1164` | `0x13a178` | `0x5` | `0x144d9c` | `?` |
| `bl` at 0xf3978 | `(unknown callee) 0xf1164` | `0x13a184` | `0x6` | `0x144fe8` | `?` |
| `bl` at 0xf39cc | `(unknown callee) 0xf1164` | `0x13a18c` | `0x12` | `0x1455cc` | `?` |
| `bl` at 0xf3a20 | `(unknown callee) 0xf1164` | `0x13a19c` | `0x11` | `0x1455e4` | `?` |
| `bl` at 0xf3a74 | `(unknown callee) 0xf1164` | `0x13a1ac` | `0xa` | `0x145238` | `?` |
| `bl` at 0xf3ac8 | `(unknown callee) 0xf1164` | `0x13a1b8` | `0xb` | `0x145238` | `?` |
| `bl` at 0xf3b1c | `(unknown callee) 0xf1164` | `0x13a1c4` | `0x1a` | `0x1456d0` | `?` |
| `bl` at 0xf3b70 | `(unknown callee) 0xf1164` | `0x13a1d4` | `0x1b` | `0x1456d0` | `?` |
| `bl` at 0xf3bc4 | `(unknown callee) 0xf1164` | `0x13a1e4` | `0x60` | `0x145314` | `?` |
| `bl` at 0xf3c18 | `(unknown callee) 0xf1164` | `0x13a1f4` | `0x62` | `0x145314` | `?` |
| `bl` at 0xf3c6c | `(unknown callee) 0xf1164` | `0x13a204` | `0x61` | `0x145748` | `?` |
| `bl` at 0xf3cc0 | `(unknown callee) 0xf1164` | `0x13a214` | `0x63` | `0x145748` | `?` |
| `bl` at 0xf3d14 | `(unknown callee) 0xf1164` | `0x13a224` | `0x14` | `0x145764` | `?` |
| `bl` at 0xf3d68 | `(unknown callee) 0xf1164` | `0x13a230` | `0x9` | `0x145028` | `?` |
| `bl` at 0xf3dbc | `(unknown callee) 0xf1164` | `0x13a23c` | `0xc` | `0x144ca8` | `?` |
| `bl` at 0xf3e10 | `(unknown callee) 0xf1164` | `0x13a24c` | `0xe` | `0x144cc0` | `?` |
| `bl` at 0xf3e64 | `(unknown callee) 0xf1164` | `0x13a25c` | `0xd` | `0x144d10` | `?` |
| `bl` at 0xf3eb8 | `(unknown callee) 0xf1164` | `0x13a268` | `0x15` | `0x144d6c` | `?` |
| `bl` at 0xf3f0c | `(unknown callee) 0xf1164` | `0x13a274` | `0x13` | `0x14577c` | `?` |
| `bl` at 0xf3f60 | `(unknown callee) 0xf1164` | `0x13a280` | `0x3` | `0x1457b8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf3720 | `[r0+40] != 0x0` | `[r0+40] == 0x0` |
| `bls` at 0xf3740 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xf37b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf380c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3860 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf38b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3908 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf395c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf39b0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3a04 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3a58 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3aac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3b00 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3b54 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3ba8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3bfc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3c50 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3ca4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3cf8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3d4c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3da0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3df4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3e48 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3e9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3ef0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3f44 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf3f98 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf3fc0 | `r0` | `[sp-36]` |
