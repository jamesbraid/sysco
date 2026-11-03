# ARM Linux 2.6 SHUB reset and control routines in 1.44.0

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
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-shub-reset-controls-facts.txt) retain all tool sections.

## `SHUB_Reset_PreRelease`

- `SHUB_Reset_PreRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a220 to 0x6a76c.
  Established. A byte store preserves the low eight bits of entry r0 at entry sp minus 0x11 (`strb` at 0x6a234).
  An initial word store writes 0 at entry sp minus 0x18 (`str` at 0x6a23c).
  A later word store there writes the result from unnamed 0x68bc8 (`bl` at 0x6a264, `str` at 0x6a26c).
  Another word store writes 0xfffff883 at entry sp minus 0x74 (`str` at 0x6a2d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6a234 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x6a23c | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x6a240 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x6a248 | load | 4 | `abs 0x6a6f8` | `-` |
| `ldr` at 0x6a24c | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x6a254 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6a258 | store | 4 | `sp -0x70` | `?` |
| `ldrb` at 0x6a25c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a26c | store | 4 | `sp -0x18` | `ret(0x68bc8)` |
| `ldr` at 0x6a270 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a27c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a280 | store | 4 | `sp -0x74` | `ret(0x68bc8)` |
| `ldrb` at 0x6a288 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a298 | store | 4 | `sp -0x18` | `ret(0x68fe4)` |
| `ldr` at 0x6a29c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a2a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a2ac | store | 4 | `sp -0x74` | `ret(0x68fe4)` |
| `ldr` at 0x6a2d4 | load | 4 | `abs 0x6a6fc` | `-` |
| `str` at 0x6a2d8 | store | 4 | `sp -0x74` | `0xfffff883` |
| `ldrb` at 0x6a2e0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a2f4 | store | 4 | `sp -0x18` | `ret(0x6814c)` |
| `ldr` at 0x6a2f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a304 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a308 | store | 4 | `sp -0x74` | `ret(0x6814c)` |
| `ldrb` at 0x6a310 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a320 | store | 4 | `sp -0x18` | `ret(0x6653c)` |
| `ldr` at 0x6a324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a330 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a334 | store | 4 | `sp -0x74` | `ret(0x6653c)` |
| `ldr` at 0x6a33c | load | 4 | `abs 0x6a700` | `-` |
| `ldr` at 0x6a340 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a34c | load | 4 | `abs 0x6a704` | `-` |
| `str` at 0x6a350 | store | 4 | `sp -0x7c` | `0x163fe0` |
| `ldr` at 0x6a358 | load | 4 | `abs 0x6a708` | `-` |
| `ldr` at 0x6a35c | load | 4 | `abs 0x6a70c` | `-` |
| `str` at 0x6a36c | store | 4 | `sp -0x1c` | `0x10` |
| `ldr` at 0x6a370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a374 | load | 4 | `abs 0x6a710` | `-` |
| `ldr` at 0x6a380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a388 | load | 4 | `abs 0x6a710` | `-` |
| `str` at 0x6a394 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a39c | load | 4 | `abs 0x6a714` | `-` |
| `ldr` at 0x6a3a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a3b0 | load | 4 | `abs 0x6a714` | `-` |
| `str` at 0x6a3bc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a3c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a3c4 | load | 4 | `abs 0x6a718` | `-` |
| `ldr` at 0x6a3d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a3d8 | load | 4 | `abs 0x6a718` | `-` |
| `str` at 0x6a3e4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a3e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a3f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6a40c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a410 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a420 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6a434 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x6a440 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x6a444 | load | 4 | `abs 0x6a71c` | `-` |
| `str` at 0x6a448 | store | 4 | `sp -0x78` | `0x163ff4` |
| `ldr` at 0x6a44c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x6a468 | load | 4 | `abs 0x6a720` | `-` |
| `str` at 0x6a46c | store | 4 | `sp -0x7c` | `0x164004` |
| `ldrb` at 0x6a470 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a474 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x6a47c | load | 4 | `abs 0x6a708` | `-` |
| `ldr` at 0x6a480 | load | 4 | `abs 0x6a724` | `-` |
| `ldr` at 0x6a48c | load | 4 | `abs 0x6a728` | `-` |
| `str` at 0x6a490 | store | 4 | `sp -0x74` | `0xfffff894` |
| `str` at 0x6a4a0 | store | 4 | `sp -0x7c` | `0xa` |
| `ldr` at 0x6a4a4 | load | 4 | `abs 0x6a72c` | `-` |
| `str` at 0x6a4a8 | store | 4 | `sp -0x78` | `0x164034` |
| `ldr` at 0x6a4ac | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x6a4c8 | load | 4 | `abs 0x6a730` | `-` |
| `str` at 0x6a4cc | store | 4 | `sp -0x7c` | `0x164044` |
| `ldrb` at 0x6a4d0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a4d4 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x6a4dc | load | 4 | `abs 0x6a708` | `-` |
| `ldr` at 0x6a4e0 | load | 4 | `abs 0x6a734` | `-` |
| `ldr` at 0x6a4ec | load | 4 | `abs 0x6a728` | `-` |
| `str` at 0x6a4f0 | store | 4 | `sp -0x74` | `0xfffff894` |
| `ldr` at 0x6a4f8 | load | 4 | `abs 0x6a700` | `-` |
| `ldr` at 0x6a4fc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a508 | load | 4 | `abs 0x6a738` | `-` |
| `str` at 0x6a50c | store | 4 | `sp -0x7c` | `0x164074` |
| `ldr` at 0x6a514 | load | 4 | `abs 0x6a708` | `-` |
| `ldr` at 0x6a518 | load | 4 | `abs 0x6a73c` | `-` |
| `ldr` at 0x6a524 | load | 4 | `abs 0x6a740` | `-` |
| `str` at 0x6a52c | store | 4 | `sp -0x7c` | `0x7e` |
| `ldr` at 0x6a530 | load | 4 | `abs 0x6a744` | `-` |
| `str` at 0x6a534 | store | 4 | `sp -0x78` | `0x164080` |
| `ldr` at 0x6a538 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x6a540 | load | 4 | `abs 0x1a5cc4` | `-` |
| `ldr` at 0x6a558 | load | 4 | `abs 0x6a748` | `-` |
| `str` at 0x6a55c | store | 4 | `sp -0x7c` | `0x164088` |
| `ldrb` at 0x6a560 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a564 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x6a56c | load | 4 | `abs 0x6a708` | `-` |
| `ldr` at 0x6a570 | load | 4 | `abs 0x6a74c` | `-` |
| `ldr` at 0x6a57c | load | 4 | `abs 0x6a750` | `-` |
| `str` at 0x6a580 | store | 4 | `sp -0x74` | `0xfffff884` |
| `ldr` at 0x6a588 | load | 4 | `abs 0x6a700` | `-` |
| `ldr` at 0x6a58c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a598 | load | 4 | `abs 0x6a754` | `-` |
| `str` at 0x6a59c | store | 4 | `sp -0x7c` | `0x1640b0` |
| `ldr` at 0x6a5a4 | load | 4 | `abs 0x6a708` | `-` |
| `ldr` at 0x6a5a8 | load | 4 | `abs 0x6a758` | `-` |
| `str` at 0x6a5b8 | store | 4 | `sp -0x1c` | `0xff` |
| `ldr` at 0x6a5bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a5c0 | load | 4 | `abs 0x6a710` | `-` |
| `ldr` at 0x6a5cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a5d4 | load | 4 | `abs 0x6a710` | `-` |
| `str` at 0x6a5e0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a5e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a5e8 | load | 4 | `abs 0x6a714` | `-` |
| `ldr` at 0x6a5f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a5fc | load | 4 | `abs 0x6a714` | `-` |
| `str` at 0x6a608 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a60c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a610 | load | 4 | `abs 0x6a718` | `-` |
| `ldr` at 0x6a61c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a624 | load | 4 | `abs 0x6a718` | `-` |
| `str` at 0x6a630 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a634 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a644 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6a658 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6a65c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a66c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6a680 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x6a68c | store | 4 | `sp -0x7c` | `0x8` |
| `ldr` at 0x6a690 | load | 4 | `abs 0x6a75c` | `-` |
| `str` at 0x6a694 | store | 4 | `sp -0x78` | `0x1640c0` |
| `ldr` at 0x6a698 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x6a6b4 | load | 4 | `abs 0x6a760` | `-` |
| `str` at 0x6a6b8 | store | 4 | `sp -0x7c` | `0x1640cc` |
| `ldrb` at 0x6a6bc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a6c0 | store | 4 | `sp -0x78` | `[sp-17]:1` |
| `ldr` at 0x6a6c8 | load | 4 | `abs 0x6a708` | `-` |
| `ldr` at 0x6a6cc | load | 4 | `abs 0x6a764` | `-` |
| `ldr` at 0x6a6d8 | load | 4 | `abs 0x6a768` | `-` |
| `str` at 0x6a6dc | store | 4 | `sp -0x74` | `0xfffff892` |
| `str` at 0x6a6e8 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x6a6ec | load | 4 | `sp -0x74` | `-` |
| `ldm` at 0x6a6f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a6f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a6f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a264 | `(unknown callee) 0x68bc8` | `(r0 & 0xff)` | `r1` | `?` | `(r0 & 0xff)` |
| `bl` at 0x6a290 | `(unknown callee) 0x68fe4` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x6a2c4 | `(unknown callee) 0x12cdbc` | `0x1` | `0x0` | `sp-108` | `sp-108` |
| `bl` at 0x6a2ec | `(unknown callee) 0x6814c` | `[sp-17]:1` | `0x1` | `?` | `[sp-17]:1` |
| `bl` at 0x6a318 | `(unknown callee) 0x6653c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x6a364 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x767` | `0x0` |
| `bl` at 0x6a458 | `(unknown callee) 0x13d9cc` | `[sp-112]` | `0x60` | `sp-28` | `0x0` |
| `bl` at 0x6a488 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x76d` | `0x0` |
| `bl` at 0x6a4b8 | `(unknown callee) 0x13d9cc` | `[sp-112]` | `0x62` | `sp-28` | `0x0` |
| `bl` at 0x6a4e8 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x772` | `0x0` |
| `bl` at 0x6a520 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x77a` | `0x0` |
| `bl` at 0x6a548 | `(unknown callee) 0x13d9cc` | `[sp-112]` | `0x6` | `[0x1a5cc4]` | `0x0` |
| `bl` at 0x6a578 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x77e` | `0x0` |
| `bl` at 0x6a5b0 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x786` | `0x0` |
| `bl` at 0x6a6a4 | `(unknown callee) 0x13d9cc` | `[sp-112]` | `0x9` | `sp-28` | `0x0` |
| `bl` at 0x6a6d4 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x78c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a278 | `ret(0x68bc8) == 0x0` | `ret(0x68bc8) != 0x0` |
| `beq` at 0x6a2a4 | `ret(0x68fe4) == 0x0` | `ret(0x68fe4) != 0x0` |
| `bne` at 0x6a2d0 | `ret(0x12cdbc) != 0x0` | `ret(0x12cdbc) == 0x0` |
| `beq` at 0x6a300 | `ret(0x6814c) == 0x0` | `ret(0x6814c) != 0x0` |
| `beq` at 0x6a32c | `ret(0x6653c) == 0x0` | `ret(0x6653c) != 0x0` |
| `beq` at 0x6a348 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6a464 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x6a4c4 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x6a504 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6a554 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x6a594 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6a6b0 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a6f4 | `r0` | `?` |

## `SHUB_Reset_PostRelease`

- `SHUB_Reset_PostRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a76c to 0x6a918.
  Established. A byte store preserves the low eight bits of entry r0 at entry sp minus 0x11 (`strb` at 0x6a780).
  A word store at entry sp minus 0x18 has an unknown value after an unsupported mvn (`mvn` at 0x6a81c, `str` at 0x6a820).
  A later word store there writes the result from unnamed 0x657a0 (`bl` at 0x6a8c0, `str` at 0x6a8c8).
  The return record places [sp-24] in r0 (`ldr` at 0x6a8dc, `ldm` at 0x6a8e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6a780 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x6a788 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6a794 | load | 4 | `abs 0x6a8ec` | `-` |
| `ldrb` at 0x6a798 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x6a7b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6a7c4 | load | 4 | `abs 0x6a8f0` | `-` |
| `ldr` at 0x6a7c8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a7d4 | load | 4 | `abs 0x6a8f4` | `-` |
| `str` at 0x6a7d8 | store | 4 | `sp -0x20` | `0x1640f8` |
| `ldrb` at 0x6a7dc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a7e0 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x6a7e8 | load | 4 | `abs 0x6a8f8` | `-` |
| `ldr` at 0x6a7ec | load | 4 | `abs 0x6a8fc` | `-` |
| `ldrb` at 0x6a7f8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x6a800 | load | 4 | `abs 0x6a900` | `-` |
| `str` at 0x6a820 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x6a828 | load | 4 | `abs 0x6a8f0` | `-` |
| `ldr` at 0x6a82c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a838 | load | 4 | `abs 0x6a904` | `-` |
| `str` at 0x6a83c | store | 4 | `sp -0x20` | `0x164118` |
| `ldrb` at 0x6a840 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a844 | store | 4 | `sp -0x1c` | `[sp-17]:1` |
| `ldr` at 0x6a84c | load | 4 | `abs 0x6a8f8` | `-` |
| `ldr` at 0x6a850 | load | 4 | `abs 0x6a908` | `-` |
| `ldrb` at 0x6a85c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x6a864 | load | 4 | `abs 0x6a90c` | `-` |
| `str` at 0x6a884 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x6a88c | load | 4 | `abs 0x6a8f0` | `-` |
| `ldr` at 0x6a890 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a89c | load | 4 | `abs 0x6a910` | `-` |
| `str` at 0x6a8a0 | store | 4 | `sp -0x20` | `0x164138` |
| `ldr` at 0x6a8a8 | load | 4 | `abs 0x6a8f8` | `-` |
| `ldrb` at 0x6a8b8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a8c8 | store | 4 | `sp -0x18` | `ret(0x657a0)` |
| `ldrb` at 0x6a8cc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x6a8d0 | load | 4 | `abs 0x6a914` | `-` |
| `ldr` at 0x6a8dc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x6a8e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a8e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a8e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a790 | `(unknown callee) 0x65d64` | `0x1` | `r1` | `r2` | `0x0` |
| `bl` at 0x6a7f4 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x7b5` | `0x0` |
| `bl` at 0x6a80c | `(unknown callee) 0x64ce0` | `[sp-17]:1` | `0x500303a0` | `0x7` | `0x0` |
| `bl` at 0x6a858 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x7ba` | `0x0` |
| `bl` at 0x6a870 | `(unknown callee) 0x64ce0` | `[sp-17]:1` | `0x500305a0` | `0x7` | `0x0` |
| `bl` at 0x6a8b4 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x7c0` | `0x0` |
| `bl` at 0x6a8c0 | `(unknown callee) 0x657a0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x6a8d8 | `(unknown callee) 0x29908` | `0x164178` | `[sp-17]:1` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6a7c0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6a7d0 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bne` at 0x6a818 | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `beq` at 0x6a834 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bne` at 0x6a87c | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `beq` at 0x6a898 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a8e8 | `r0` | `[sp-24]` |

## `shub_set_link_enable`

- `shub_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a918 to 0x6a9fc.
  Established. Word stores preserve entry r0, r1 and r2 at entry sp minus 0x14, 0x18 and 0x1c respectively (`str` at 0x6a928, `str` at 0x6a92c, `str` at 0x6a930).
  The call to unnamed 0x243d4 has printed r0-r3 candidates 0x164190, entry r0, entry r1 and entry r2 (`bl` at 0x6a954).
  A later call to unnamed 0x13d2b8 has printed candidates [sp-20], 0x10060300, 0 and 1 (`bl` at 0x6a9c8).
  The return record places 0 in r0 (`ldm` at 0x6a9e0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a928 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a92c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6a930 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x6a934 | load | 4 | `abs 0x6a9e4` | `-` |
| `ldr` at 0x6a938 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a944 | load | 4 | `abs 0x6a9e8` | `-` |
| `ldr` at 0x6a948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a94c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a950 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a958 | load | 4 | `abs 0x6a9ec` | `-` |
| `ldr` at 0x6a95c | load | 4 | `abs 0x1a6460` | `-` |
| `ldr` at 0x6a968 | load | 4 | `abs 0x6a9e8` | `-` |
| `str` at 0x6a96c | store | 4 | `sp -0x30` | `0x164190` |
| `ldr` at 0x6a970 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6a974 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x6a978 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a97c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x6a980 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6a984 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x6a98c | load | 4 | `abs 0x6a9f0` | `-` |
| `ldr` at 0x6a990 | load | 4 | `abs 0x6a9f4` | `-` |
| `ldr` at 0x6a99c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6a9ac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x6a9b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a9bc | load | 4 | `abs 0x6a9f8` | `-` |
| `str` at 0x6a9d4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x6a9d8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x6a9e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a9e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a9e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a954 | `(unknown callee) 0x243d4` | `0x164190` | `r0` | `r1` | `r2` |
| `bl` at 0x6a998 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x7ef` | `0x0` |
| `bl` at 0x6a9b4 | `(unknown callee) 0x134634` | `?` | `?` | `?` | `?` |
| `bl` at 0x6a9c8 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0x10060300` | `0x0` | `0x1` |
| `bl` at 0x6a9cc | `(unknown callee) 0x1346fc` | `ret(0x13d2b8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6a940 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6a964 | `[0x1a6460] == 0x0` | `[0x1a6460] != 0x0` |
| `bne` at 0x6a9a4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a9e0 | `r0` | `0x0` |

## `SHUB_WarCheck`

- `SHUB_WarCheck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a9fc to 0x6aaa0.
  Established. A byte loaded from absolute address 0x1a5e88 is stored as a word at entry sp minus 0x20 (`ldrb` at 0x6aa5c, `str` at 0x6aa60).
  A separate byte loaded from absolute address 0x1a5e89 is stored as a word at entry sp minus 0x1c (`ldrb` at 0x6aa68, `str` at 0x6aa6c).
  The call to unnamed 0x22070 has printed r0-r3 candidates 1, 0x16320c, 0x81d and 0 (`bl` at 0x6aa80).
  The return record retains an unknown value in r0 (`ldm` at 0x6aa88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6aa0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6aa10 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6aa14 | load | 4 | `abs 0x6aa8c` | `-` |
| `ldrb` at 0x6aa18 | load | 1 | `abs 0x1a5e88` | `-` |
| `ldr` at 0x6aa1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6aa2c | load | 4 | `abs 0x6aa90` | `-` |
| `ldrb` at 0x6aa30 | load | 1 | `abs 0x1a5e89` | `-` |
| `ldr` at 0x6aa34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6aa50 | load | 4 | `abs 0x6aa94` | `-` |
| `str` at 0x6aa54 | store | 4 | `sp -0x24` | `0x1641c8` |
| `ldr` at 0x6aa58 | load | 4 | `abs 0x6aa8c` | `-` |
| `ldrb` at 0x6aa5c | load | 1 | `abs 0x1a5e88` | `-` |
| `str` at 0x6aa60 | store | 4 | `sp -0x20` | `[0x1a5e88]:1` |
| `ldr` at 0x6aa64 | load | 4 | `abs 0x6aa90` | `-` |
| `ldrb` at 0x6aa68 | load | 1 | `abs 0x1a5e89` | `-` |
| `str` at 0x6aa6c | store | 4 | `sp -0x1c` | `[0x1a5e89]:1` |
| `ldr` at 0x6aa74 | load | 4 | `abs 0x6aa98` | `-` |
| `ldr` at 0x6aa78 | load | 4 | `abs 0x6aa9c` | `-` |
| `ldm` at 0x6aa88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6aa88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6aa88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6aa80 | `(unknown callee) 0x22070` | `0x1` | `0x16320c` | `0x81d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6aa4c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6aa88 | `r0` | `?` |

## `SHUB_WarStatus`

- `SHUB_WarStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6aaa0 to 0x6aaf0.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x6aab0).
  The branch takes 0x6aacc when entry r0 is zero and falls through when it is nonzero (`beq` at 0x6aabc).
  The call to unnamed 0x205bc has printed r0-r3 candidates entry r0, 0x164210, entry r2 and entry r0 (`bl` at 0x6aac8).
  A later byte load reads absolute address 0x1a5e89 (`ldrb` at 0x6aad0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6aab0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6aab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6aac0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6aac4 | load | 4 | `abs 0x6aae8` | `-` |
| `ldr` at 0x6aacc | load | 4 | `abs 0x6aaec` | `-` |
| `ldrb` at 0x6aad0 | load | 1 | `abs 0x1a5e89` | `-` |
| `ldm` at 0x6aae4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6aae4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6aae4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6aae4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6aac8 | `(unknown callee) 0x205bc` | `r0` | `0x164210` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6aabc | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6aae4 | `r0` | `?` |

## `SHUB_WarAllow`

- `SHUB_WarAllow`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6aaf0 to 0x6ab18.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x6ab00).
  A later byte store writes the low eight bits of entry r0 to absolute address 0x1a5e88 (`ldr` at 0x6ab08, `strb` at 0x6ab0c).
  The return record preserves entry r0 (`ldm` at 0x6ab10).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ab00 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6ab04 | load | 4 | `abs 0x6ab14` | `-` |
| `ldr` at 0x6ab08 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x6ab0c | store | 1 | `abs 0x1a5e88` | `(r0 & 0xff)` |
| `ldm` at 0x6ab10 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6ab10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6ab10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6ab10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6ab10 | `r0` | `r0` |

## `SHUB_WarEnable`

- `SHUB_WarEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ab18 to 0x6ac20.
  Established. One byte store writes 1 to absolute address 0x1a5e89 (`strb` at 0x6aba8).
  Another byte store writes 0 to that same absolute address (`strb` at 0x6abe0).
  The call through an unknown target has printed r0-r3 candidates 0xff, entry r1, entry r2 and [0x1a5cc0] (`mov` at 0x6ab68).
  The return record retains an unknown value in r0 (`ldm` at 0x6abf0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ab28 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6ab2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ab38 | load | 4 | `abs 0x6abf4` | `-` |
| `ldrb` at 0x6ab3c | load | 1 | `abs 0x1a5e88` | `-` |
| `ldr` at 0x6ab48 | load | 4 | `abs 0x6abf8` | `-` |
| `ldr` at 0x6ab4c | load | 4 | `abs 0x1a5cc0` | `-` |
| `ldr` at 0x6ab58 | load | 4 | `abs 0x6abf8` | `-` |
| `ldr` at 0x6ab5c | load | 4 | `abs 0x1a5cc0` | `-` |
| `ldr` at 0x6ab78 | load | 4 | `abs 0x6abfc` | `-` |
| `str` at 0x6ab7c | store | 4 | `sp -0x18` | `0x164220` |
| `ldr` at 0x6ab84 | load | 4 | `abs 0x6ac00` | `-` |
| `ldr` at 0x6ab88 | load | 4 | `abs 0x6ac04` | `-` |
| `ldr` at 0x6ab98 | load | 4 | `abs 0x6ac08` | `-` |
| `ldr` at 0x6aba0 | load | 4 | `abs 0x6ac0c` | `-` |
| `strb` at 0x6aba8 | store | 1 | `abs 0x1a5e89` | `0x1` |
| `ldr` at 0x6abac | load | 4 | `abs 0x6ac10` | `-` |
| `ldr` at 0x6abb8 | load | 4 | `abs 0x6ac14` | `-` |
| `str` at 0x6abbc | store | 4 | `sp -0x18` | `0x16424c` |
| `ldr` at 0x6abc4 | load | 4 | `abs 0x6ac00` | `-` |
| `ldr` at 0x6abc8 | load | 4 | `abs 0x6ac18` | `-` |
| `ldr` at 0x6abd8 | load | 4 | `abs 0x6ac0c` | `-` |
| `strb` at 0x6abe0 | store | 1 | `abs 0x1a5e89` | `0x0` |
| `ldr` at 0x6abe4 | load | 4 | `abs 0x6ac1c` | `-` |
| `ldm` at 0x6abf0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6abf0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6abf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x6ab68 | `(unknown callee) (unknown target)` | `0xff` | `r1` | `r2` | `[0x1a5cc0]` |
| `bl` at 0x6ab90 | `(unknown callee) 0x22070` | `0x1` | `0x16320c` | `0x844` | `0x0` |
| `bl` at 0x6ab9c | `(unknown callee) 0x293f8` | `0x164240` | `?` | `?` | `?` |
| `bl` at 0x6abb0 | `(unknown callee) 0x2302c` | `0x1a5e34` | `?` | `0x1` | `0x1a5e89` |
| `bl` at 0x6abd0 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x84c` | `0x0` |
| `bl` at 0x6abe8 | `(unknown callee) 0x293f8` | `0x164280` | `r1` | `0x0` | `0x1a5e89` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6ab34 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x6ab44 | `[0x1a5e88]:1 == 0x0` | `[0x1a5e88]:1 != 0x0` |
| `beq` at 0x6ab54 | `[0x1a5cc0] == 0x0` | `[0x1a5cc0] != 0x0` |
| `beq` at 0x6ab74 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6abf0 | `r0` | `?` |

## `SHUB2_Cfg_Init`

- `SHUB2_Cfg_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8c248 to 0x8cfb4.
  Established. The result from unnamed 0x6e140 is stored as a word at absolute address 0x1a641c (`bl` at 0x8c274, `str` at 0x8c280).
  The result from a call to unnamed 0x204d8 is stored as a word at absolute address 0x1a6418 (`bl` at 0x8c2ac, `str` at 0x8c2b4).
  A separate call to unnamed 0x204d8 has its result stored as a word at the address formed by [sp-52] plus 4 (`bl` at 0x8c334, `str` at 0x8c33c).
  A later byte store writes 1 to the address formed by [sp-88] plus 3 (`strb` at 0x8c63c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8c258 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8c25c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x8c260 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x8c264 | load | 4 | `abs 0x8cf7c` | `-` |
| `ldr` at 0x8c268 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8c27c | load | 4 | `abs 0x8cf80` | `-` |
| `str` at 0x8c280 | store | 4 | `abs 0x1a641c` | `ret(0x6e140)` |
| `ldr` at 0x8c284 | load | 4 | `abs 0x8cf80` | `-` |
| `ldr` at 0x8c288 | load | 4 | `abs 0x1a641c` | `-` |
| `str` at 0x8c29c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x8c2a0 | load | 4 | `abs 0x8cf7c` | `-` |
| `ldr` at 0x8c2a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8c2b4 | store | 4 | `abs 0x1a6418` | `ret(0x204d8)` |
| `ldr` at 0x8c2b8 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8c2c4 | load | 4 | `abs 0x8cf84` | `-` |
| `str` at 0x8c2c8 | store | 4 | `sp -0x164` | `0x167994` |
| `ldr` at 0x8c2cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8c2d0 | store | 4 | `sp -0x160` | `[sp-44]` |
| `ldr` at 0x8c2d8 | load | 4 | `abs 0x8cf88` | `-` |
| `ldr` at 0x8c2e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c2fc | load | 4 | `abs 0x8cf7c` | `-` |
| `ldr` at 0x8c300 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8c308 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x8c30c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c310 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8c314 | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x8c318 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c31c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8c320 | store | 4 | `[sp-52] +0x8` | `[sp-32]` |
| `ldr` at 0x8c324 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c328 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c32c | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x8c33c | store | 4 | `[sp-52] +0x4` | `ret(0x204d8)` |
| `ldr` at 0x8c340 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x8c34c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c360 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x8c364 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8c374 | store | 4 | `sp -0x3c` | `ret(0x2031c)+1` |
| `ldr` at 0x8c378 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x8c3a0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x8c3a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c3a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x8c3ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8c3b8 | load | 4 | `abs 0x8cf8c` | `-` |
| `str` at 0x8c3bc | store | 4 | `(unknown)` | `0x1679e4` |
| `str` at 0x8c3c0 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x8c3c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c3c8 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x8c3cc | store | 4 | `(unknown)` | `[[sp-52]+8]` |
| `ldr` at 0x8c3d4 | load | 4 | `abs 0x8cf88` | `-` |
| `str` at 0x8c3e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x8c3ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c3f0 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x8c3f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c400 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c404 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c418 | load | 4 | `[sp-52] +0x0` | `-` |
| `str` at 0x8c420 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x8c424 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c428 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c444 | load | 4 | `[sp-52] +0x4` | `-` |
| `str` at 0x8c44c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x8c468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c474 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c478 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c48c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x8c494 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c49c | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0x8c4a0 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0x8c4a8 | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0x8c4ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c4bc | load | 4 | `sp -0x14c` | `-` |
| `str` at 0x8c4c4 | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0x8c4cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c4d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c4e4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x8c4ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c4f0 | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0x8c4f4 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0x8c4f8 | store | 4 | `sp -0x5c` | `[sp-332]` |
| `str` at 0x8c500 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x8c508 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x8c50c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c510 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c52c | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x8c534 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x8c538 | store | 1 | `(unknown)` | `([sp-24] & 0xff)` |
| `ldr` at 0x8c53c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c550 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x8c558 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8c55c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c578 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8c57c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c598 | load | 4 | `[sp-52] +0x4` | `-` |
| `strb` at 0x8c5a4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x8c5a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c5bc | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8c5c4 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8c5c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c5d4 | store | 4 | `sp -0x30` | `ret(0x13ba30)` |
| `ldr` at 0x8c5d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8c5e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8c5f0 | load | 4 | `abs 0x8cf90` | `-` |
| `str` at 0x8c5f4 | store | 4 | `(unknown)` | `0x167a20` |
| `ldr` at 0x8c5f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8c60c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8c614 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c618 | store | 4 | `(unknown)` | `?` |
| `str` at 0x8c61c | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x8c624 | load | 4 | `abs 0x8cf88` | `-` |
| `ldr` at 0x8c634 | load | 4 | `sp -0x58` | `-` |
| `strb` at 0x8c63c | store | 1 | `[sp-88] +0x3` | `0x1` |
| `str` at 0x8c644 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x8c648 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x8c64c | load | 1 | `[sp-84] +0xe` | `-` |
| `ldr` at 0x8c650 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c65c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c660 | load | 4 | `[sp-88] +0x24` | `-` |
| `ldr` at 0x8c66c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8c670 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x8c674 | load | 1 | `[sp-84] +0x12` | `-` |
| `ldr` at 0x8c684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c690 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8c694 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x8c698 | load | 1 | `[sp-84] +0x12` | `-` |
| `ldr` at 0x8c6a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c6c0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8c6c4 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x8c6c8 | load | 1 | `[sp-84] +0x12` | `-` |
| `ldr` at 0x8c6dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c6f4 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x8c700 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8c714 | store | 4 | `sp -0x140` | `ret(0x6e1b0)` |
| `ldr` at 0x8c718 | load | 4 | `sp -0x140` | `-` |
| `str` at 0x8c728 | store | 4 | `sp -0x144` | `ret(0x2031c)+1` |
| `ldr` at 0x8c72c | load | 4 | `sp -0x144` | `-` |
| `str` at 0x8c754 | store | 4 | `sp -0x148` | `?` |
| `ldr` at 0x8c758 | load | 4 | `sp -0x148` | `-` |
| `ldr` at 0x8c75c | load | 4 | `sp -0x140` | `-` |
| `ldr` at 0x8c760 | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x8c76c | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x8c770 | load | 1 | `[sp-84] +0x12` | `-` |
| `ldr` at 0x8c774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c784 | load | 4 | `abs 0x8cf94` | `-` |
| `str` at 0x8c788 | store | 4 | `(unknown)` | `0x167a3c` |
| `str` at 0x8c78c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x8c790 | store | 4 | `(unknown)` | `ret(0x1208b8)` |
| `ldr` at 0x8c798 | load | 4 | `abs 0x8cf88` | `-` |
| `ldr` at 0x8c7b8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8c7c0 | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0x8c7c4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c7c8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c7d0 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0x8c7d8 | load | 1 | `[sp-88] +0x5` | `-` |
| `strb` at 0x8c7e0 | store | 1 | `[sp-88] +0x5` | `?` |
| `ldr` at 0x8c7e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c7fc | load | 4 | `sp -0x118` | `-` |
| `ldrb` at 0x8c800 | load | 1 | `[sp-280] +0x1` | `-` |
| `str` at 0x8c804 | store | 4 | `(unknown)` | `[[sp-280]+1]:1` |
| `ldr` at 0x8c808 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c820 | load | 4 | `sp -0x114` | `-` |
| `str` at 0x8c824 | store | 4 | `(unknown)` | `[sp-276]` |
| `ldr` at 0x8c828 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c840 | load | 4 | `sp -0x118` | `-` |
| `ldrb` at 0x8c844 | load | 1 | `[sp-280] +0x3` | `-` |
| `str` at 0x8c848 | store | 4 | `(unknown)` | `[[sp-280]+3]:1` |
| `ldr` at 0x8c84c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c864 | load | 4 | `sp -0x10c` | `-` |
| `str` at 0x8c868 | store | 4 | `(unknown)` | `[sp-268]` |
| `ldr` at 0x8c86c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c884 | load | 4 | `sp -0x110` | `-` |
| `str` at 0x8c88c | store | 4 | `(unknown)` | `ret(0x8d284)` |
| `ldr` at 0x8c890 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c8a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c8c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c8c8 | store | 4 | `(unknown)` | `ret(0x8d31c)` |
| `ldr` at 0x8c8cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c8e4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c8e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8c8ec | load | 4 | `[sp-88] +0x2c` | `-` |
| `ldr` at 0x8c8f8 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c8fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c914 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c918 | store | 4 | `[sp-88] +0x2c` | `?` |
| `ldr` at 0x8c91c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c920 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c938 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c940 | store | 4 | `[sp-88] +0x30` | `ret(0x8d3cc)` |
| `ldr` at 0x8c944 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c95c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c960 | store | 4 | `sp -0x150` | `?` |
| `ldr` at 0x8c964 | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0x8c970 | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0x8c980 | load | 4 | `sp -0x58` | `-` |
| `strb` at 0x8c988 | store | 1 | `[sp-88] +0x3` | `0x0` |
| `ldr` at 0x8c98c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c990 | load | 4 | `[sp-88] +0x1c` | `-` |
| `ldr` at 0x8c99c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c9b4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c9b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8c9bc | load | 4 | `[sp-88] +0x1c` | `-` |
| `ldr` at 0x8c9cc | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8c9d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8c9e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8c9ec | store | 4 | `[sp-88] +0x1c` | `?` |
| `ldr` at 0x8c9f4 | load | 4 | `sp -0x58` | `-` |
| `strb` at 0x8c9fc | store | 1 | `[sp-88] +0x18` | `0x1` |
| `ldr` at 0x8ca00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8ca18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ca24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8ca3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ca48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8ca60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ca64 | load | 4 | `abs 0x8cf98` | `-` |
| `ldr` at 0x8ca70 | load | 4 | `sp -0x58` | `-` |
| `strb` at 0x8ca78 | store | 1 | `[sp-88] +0x19` | `0x1` |
| `ldr` at 0x8ca80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8ca94 | store | 4 | `sp -0x148` | `ret(0x6e1b0)` |
| `ldr` at 0x8ca98 | load | 4 | `sp -0x148` | `-` |
| `str` at 0x8caa8 | store | 4 | `sp -0x144` | `ret(0x2031c)+1` |
| `ldr` at 0x8caac | load | 4 | `sp -0x144` | `-` |
| `str` at 0x8cad4 | store | 4 | `sp -0x140` | `?` |
| `ldr` at 0x8cad8 | load | 4 | `sp -0x140` | `-` |
| `ldr` at 0x8cadc | load | 4 | `sp -0x148` | `-` |
| `ldr` at 0x8cae0 | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x8caec | load | 4 | `abs 0x8cf9c` | `-` |
| `str` at 0x8caf0 | store | 4 | `(unknown)` | `0x167a84` |
| `str` at 0x8caf4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x8caf8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8cafc | store | 4 | `(unknown)` | `[sp-64]` |
| `ldr` at 0x8cb00 | load | 4 | `sp -0x118` | `-` |
| `ldrb` at 0x8cb04 | load | 1 | `[sp-280] +0x2` | `-` |
| `str` at 0x8cb08 | store | 4 | `(unknown)` | `[[sp-280]+2]:1` |
| `ldr` at 0x8cb10 | load | 4 | `abs 0x8cf88` | `-` |
| `ldr` at 0x8cb20 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cb24 | load | 4 | `abs 0x8cfa0` | `-` |
| `str` at 0x8cb28 | store | 4 | `[sp-88] +0x24` | `0xfffff0bd` |
| `ldr` at 0x8cb30 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x8cb3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cb54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8cb58 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x8cb64 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cb68 | load | 4 | `abs 0x8cfa4` | `-` |
| `str` at 0x8cb6c | store | 4 | `[sp-88] +0x24` | `0xfffff0be` |
| `ldr` at 0x8cb70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8cb84 | store | 4 | `sp -0x148` | `ret(0x6e1b0)` |
| `ldr` at 0x8cb88 | load | 4 | `sp -0x148` | `-` |
| `str` at 0x8cb98 | store | 4 | `sp -0x144` | `ret(0x2031c)+1` |
| `ldr` at 0x8cb9c | load | 4 | `sp -0x144` | `-` |
| `str` at 0x8cbc4 | store | 4 | `sp -0x140` | `?` |
| `ldr` at 0x8cbc8 | load | 4 | `sp -0x140` | `-` |
| `ldr` at 0x8cbcc | load | 4 | `sp -0x148` | `-` |
| `ldr` at 0x8cbd0 | load | 4 | `sp -0x144` | `-` |
| `ldr` at 0x8cbdc | load | 4 | `abs 0x8cfa8` | `-` |
| `str` at 0x8cbe0 | store | 4 | `(unknown)` | `0x167ac4` |
| `str` at 0x8cbe4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x8cbe8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cc00 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cc04 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x8cc08 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x8cc0c | store | 4 | `(unknown)` | `[sp-116]` |
| `ldr` at 0x8cc14 | load | 4 | `abs 0x8cf88` | `-` |
| `ldr` at 0x8cc24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cc3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8cc40 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x8cc4c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cc50 | load | 4 | `abs 0x8cfac` | `-` |
| `str` at 0x8cc54 | store | 4 | `[sp-88] +0x28` | `0xfffff0bb` |
| `ldr` at 0x8cc58 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x8cc5c | load | 1 | `[sp-84] +0xd` | `-` |
| `ldr` at 0x8cc68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cc80 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x8cc84 | load | 1 | `[sp-84] +0xd` | `-` |
| `ldr` at 0x8cc8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8cc9c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cca0 | load | 4 | `abs 0x8cfb0` | `-` |
| `str` at 0x8cca4 | store | 4 | `[sp-88] +0x28` | `0xfffff0ba` |
| `ldr` at 0x8ccb4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8ccbc | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0x8ccc8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x8cccc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8ccd8 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0x8ccdc | load | 1 | `[sp-88] +0x18` | `-` |
| `ldr` at 0x8cce8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8ccf0 | store | 4 | `sp -0x148` | `?` |
| `ldr` at 0x8ccf4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ccf8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cd0c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8cd14 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8cd18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cd28 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0x8cd2c | load | 1 | `[sp-88] +0x5` | `-` |
| `ldr` at 0x8cd30 | load | 4 | `sp -0x148` | `-` |
| `ldr` at 0x8cd44 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cd48 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8cd60 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x8cd68 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x8cd74 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0x8cd78 | load | 1 | `[sp-88] +0x5` | `-` |
| `str` at 0x8cd8c | store | 4 | `sp -0x154` | `0x0` |
| `str` at 0x8cd98 | store | 4 | `sp -0x154` | `0x1` |
| `ldr` at 0x8cd9c | load | 4 | `sp -0x154` | `-` |
| `str` at 0x8cda0 | store | 4 | `sp -0x148` | `[sp-340]` |
| `ldr` at 0x8cda4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cda8 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8cdc0 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x8cdc4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cdc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cddc | load | 4 | `sp -0x148` | `-` |
| `strb` at 0x8cde0 | store | 1 | `(unknown)` | `([sp-328] & 0xff)` |
| `ldr` at 0x8cde4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8cde8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8cdfc | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8ce04 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8ce08 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ce0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8ce20 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8ce28 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8ce30 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ce34 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8ce4c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x8ce50 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ce54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8ce68 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8ce70 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8ce74 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ce78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8ce8c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8ce94 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8ce98 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x8ce9c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x8ceb4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x8ceb8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8cec0 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0x8cec8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x8ced4 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0x8ced8 | load | 1 | `[sp-88] +0x18` | `-` |
| `ldr` at 0x8cee4 | load | 4 | `sp -0x58` | `-` |
| `strb` at 0x8ceec | store | 1 | `[sp-88] +0x3` | `0x0` |
| `ldr` at 0x8cef0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x8cefc | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0x8cf00 | load | 1 | `[sp-88] +0x18` | `-` |
| `str` at 0x8cf10 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x8cf1c | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0x8cf2c | load | 4 | `sp -0x60` | `-` |
| `str` at 0x8cf30 | store | 4 | `(unknown)` | `[sp-96]` |
| `str` at 0x8cf3c | store | 4 | `(unknown)` | `0x10` |
| `ldr` at 0x8cf40 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x8cf54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cf5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8cf64 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0x8cf78 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x8cf78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8cf78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8cf78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8c274 | `(unknown callee) 0x6e140` | `r0` | `r1` | `r2` | `[0x1a6418]` |
| `bl` at 0x8c2ac | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `[0x1a641c]` | `?` |
| `bl` at 0x8c2e4 | `(unknown callee) 0x22070` | `0x9` | `0x1679cc` | `0x78` | `0x0` |
| `bl` at 0x8c334 | `(unknown callee) 0x204d8` | `[[sp-52]+8]` | `0x34` | `[sp-52]` | `[sp-52]` |
| `bl` at 0x8c35c | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x8c368 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x8c3b0 | `(unknown callee) 0x203f4` | `[sp-64]` | `[sp-56]` | `[sp-60]` | `?` |
| `bl` at 0x8c3e0 | `(unknown callee) 0x22070` | `0x9` | `0x1679cc` | `0x84` | `0x0` |
| `bl` at 0x8c450 | `(unknown callee) 0x6e140` | `?` | `[sp-52]` | `?` | `?` |
| `bl` at 0x8c460 | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0x8c564 | `(unknown callee) 0x125958` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x8c5cc | `(unknown callee) 0x13ba30` | `[sp-92]` | `?` | `?` | `?` |
| `bl` at 0x8c5e8 | `(unknown callee) 0x219a4` | `ret(0x13ba30)` | `?` | `?` | `ret(0x13ba30)` |
| `bl` at 0x8c630 | `(unknown callee) 0x22070` | `0x9` | `0x1679cc` | `0xa3` | `0x0` |
| `bl` at 0x8c68c | `(unknown callee) 0x11fca8` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x8c6b0 | `(unknown callee) 0x120398` | `[sp-24]` | `?` | `[sp-64]` | `?` |
| `bl` at 0x8c6e4 | `(unknown callee) 0x11ff70` | `[sp-24]` | `?` | `sp-316` | `?` |
| `bl` at 0x8c710 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x8c71c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x8c764 | `(unknown callee) 0x203f4` | `[sp-328]` | `[sp-320]` | `[sp-324]` | `?` |
| `bl` at 0x8c77c | `(unknown callee) 0x1208b8` | `[sp-24]` | `[[sp-84]+18]:1` | `?` | `[[sp-84]+18]:1` |
| `bl` at 0x8c7a4 | `(unknown callee) 0x22070` | `0x1` | `0x1679cc` | `0xb7` | `0x0` |
| `bl` at 0x8c7b0 | `(unknown callee) 0x120154` | `sp-316` | `?` | `?` | `sp-316` |
| `bl` at 0x8c888 | `(unknown callee) 0x8d284` | `[sp-272]` | `sp-20` | `?` | `?` |
| `bl` at 0x8c8c4 | `(unknown callee) 0x8d31c` | `?` | `sp-20` | `?` | `?` |
| `bl` at 0x8c93c | `(unknown callee) 0x8d3cc` | `?` | `sp-20` | `?` | `?` |
| `bl` at 0x8ca90 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x8ca9c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x8cae4 | `(unknown callee) 0x203f4` | `[sp-320]` | `[sp-328]` | `[sp-324]` | `?` |
| `bl` at 0x8cb1c | `(unknown callee) 0x22070` | `0x1` | `0x1679cc` | `0xec` | `0x0` |
| `bl` at 0x8cb80 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x8cb8c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x8cbd4 | `(unknown callee) 0x203f4` | `[sp-320]` | `[sp-328]` | `[sp-324]` | `?` |
| `bl` at 0x8cc20 | `(unknown callee) 0x22070` | `0x1` | `0x1679cc` | `0xf7` | `0x0` |
| `bl` at 0x8ccb0 | `(unknown callee) 0x120154` | `sp-316` | `?` | `?` | `sp-316` |
| `bl` at 0x8cf50 | `(unknown callee) 0x13f1ac` | `[sp-92]` | `0x14` | `0x0` | `sp-80` |
| `bl` at 0x8cf58 | `(unknown callee) 0x8d47c` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8c270 | `[0x1a6418] != 0x0` | `[0x1a6418] == 0x0` |
| `bne` at 0x8c2c0 | `[0x1a6418] != 0x0` | `[0x1a6418] == 0x0` |
| `bne` at 0x8c348 | `[[sp-52]+4] != 0x0` | `[[sp-52]+4] == 0x0` |
| `ble` at 0x8c3fc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x8c45c | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `beq` at 0x8c470 | `ret(0x6e190) == [sp-24]` | `ret(0x6e190) != [sp-24]` |
| `bne` at 0x8c570 | `ret(0x125958) != 0x0` | `ret(0x125958) == 0x0` |
| `beq` at 0x8c5e0 | `ret(0x13ba30) == 0x0` | `ret(0x13ba30) != 0x0` |
| `ble` at 0x8c658 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x8c668 | `[[sp-88]+36] != 0x0` | `[[sp-88]+36] == 0x0` |
| `beq` at 0x8c6bc | `ret(0x120398) == 0x0` | `ret(0x120398) != 0x0` |
| `bne` at 0x8c6f0 | `ret(0x11ff70) != 0x0` | `ret(0x11ff70) == 0x0` |
| `beq` at 0x8c6fc | `[sp-284] == 0x0` | `[sp-284] != 0x0` |
| `ble` at 0x8c8f4 | `? <= [[sp-88]+44] (signed, 32-bit)` | `? > [[sp-88]+44] (signed, 32-bit)` |
| `beq` at 0x8c96c | `[sp-336] == 0x1f` | `[sp-336] != 0x1f` |
| `beq` at 0x8c978 | `[sp-336] == 0x20` | `[sp-336] != 0x20` |
| `beq` at 0x8c998 | `[[sp-88]+28] == 0x0` | `[[sp-88]+28] != 0x0` |
| `blt` at 0x8c9c4 | `? < [[sp-88]+28] (signed, 32-bit)` | `? >= [[sp-88]+28] (signed, 32-bit)` |
| `ble` at 0x8ca20 | `? <= 0x1 (signed, 32-bit)` | `? > 0x1 (signed, 32-bit)` |
| `bgt` at 0x8ca44 | `? > 0x3 (signed, 32-bit)` | `? <= 0x3 (signed, 32-bit)` |
| `bne` at 0x8ca6c | `? != 0x578` | `? == 0x578` |
| `ble` at 0x8cb38 | `[sp-100] <= 0x1 (signed, 32-bit)` | `[sp-100] > 0x1 (signed, 32-bit)` |
| `beq` at 0x8cb60 | `? == [sp-116]` | `? != [sp-116]` |
| `beq` at 0x8cc48 | `? == [sp-180]` | `? != [sp-180]` |
| `beq` at 0x8cc64 | `[[sp-84]+13]:1 == 0x0` | `[[sp-84]+13]:1 != 0x0` |
| `beq` at 0x8cc98 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x8ccd4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x8cce4 | `[[sp-88]+24]:1 != 0x0` | `[[sp-88]+24]:1 == 0x0` |
| `bne` at 0x8cd24 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8cd40 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x8cd70 | `[sp-100] != 0x1` | `[sp-100] == 0x1` |
| `beq` at 0x8cd84 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x8ced0 | `[sp-100] <= 0x1 (signed, 32-bit)` | `[sp-100] > 0x1 (signed, 32-bit)` |
| `beq` at 0x8cee0 | `[[sp-88]+24]:1 == 0x0` | `[[sp-88]+24]:1 != 0x0` |
| `ble` at 0x8cef8 | `[sp-100] <= 0x1 (signed, 32-bit)` | `[sp-100] > 0x1 (signed, 32-bit)` |
| `beq` at 0x8cf08 | `[[sp-88]+24]:1 == 0x0` | `[[sp-88]+24]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8cf78 | `r0` | `0x0` |

## `SHUB2_Remove`

- `SHUB2_Remove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8cfb4 to 0x8d094.
  Established. A word store writes 0xfffff0c4 at entry sp minus 0x1c (`str` at 0x8d008).
  Later word stores write 0 to the address held in [sp-24] and the addresses formed by adding 8 and 4 to it (`str` at 0x8d04c, `str` at 0x8d058, `str` at 0x8d070).
  A word loaded from [sp-24] plus 4 supplies the printed r0 candidate for a call to unnamed 0x20514 (`ldr` at 0x8d060, `bl` at 0x8d064).
  The return record retains an unknown value in r0 (`ldm` at 0x8d084).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8cfc4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8cfc8 | load | 4 | `abs 0x8d088` | `-` |
| `ldr` at 0x8cfcc | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8cfd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cfec | load | 4 | `abs 0x8d08c` | `-` |
| `ldr` at 0x8cff0 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8cff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d004 | load | 4 | `abs 0x8d090` | `-` |
| `str` at 0x8d008 | store | 4 | `sp -0x1c` | `0xfffff0c4` |
| `ldr` at 0x8d010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d024 | load | 4 | `abs 0x8d08c` | `-` |
| `ldr` at 0x8d028 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8d030 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x8d034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d038 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x8d044 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d04c | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x8d050 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d058 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x8d05c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d060 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x8d068 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d070 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `str` at 0x8d078 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x8d07c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x8d084 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8d084 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8d084 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8d064 | `(unknown callee) 0x20514` | `[[sp-24]+4]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8cfd4 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8d000 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8d040 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8d084 | `r0` | `?` |

## `SHUB2_Init`

- `SHUB2_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8d094 to 0x8d284.
  Established. The result from unnamed 0x125b8c is stored as a word at entry sp minus 0x24 (`bl` at 0x8d1c8, `str` at 0x8d1d0).
  The result from a separate call to unnamed 0x219a4 is stored as a word at entry sp minus 0x3c (`bl` at 0x8d1e4, `str` at 0x8d1fc).
  A byte store writes 1 to the address formed by [sp-48] plus 4 (`strb` at 0x8d230).
  A later word store writes 0 at entry sp minus 0x38 (`str` at 0x8d25c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8d0a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8d0a8 | load | 4 | `abs 0x8d26c` | `-` |
| `ldr` at 0x8d0ac | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8d0b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d0cc | load | 4 | `abs 0x8d270` | `-` |
| `ldr` at 0x8d0d0 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8d0d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d0e4 | load | 4 | `abs 0x8d274` | `-` |
| `str` at 0x8d0e8 | store | 4 | `sp -0x38` | `0xfffff0c4` |
| `ldr` at 0x8d0f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d104 | load | 4 | `abs 0x8d270` | `-` |
| `ldr` at 0x8d108 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8d110 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x8d118 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x8d11c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8d120 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x8d124 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d130 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8d134 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d148 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x8d150 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x8d154 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8d158 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d174 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x8d17c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x8d180 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x8d184 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d1a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d1b4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x8d1b8 | load | 1 | `[sp-44] +0x10` | `-` |
| `ldr` at 0x8d1c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8d1d0 | store | 4 | `sp -0x24` | `ret(0x125b8c)` |
| `ldr` at 0x8d1d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8d1e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8d1ec | load | 4 | `abs 0x8d278` | `-` |
| `str` at 0x8d1f0 | store | 4 | `sp -0x44` | `0x167afc` |
| `ldr` at 0x8d1f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d1f8 | store | 4 | `sp -0x40` | `[sp-24]` |
| `str` at 0x8d1fc | store | 4 | `sp -0x3c` | `ret(0x219a4)` |
| `ldr` at 0x8d204 | load | 4 | `abs 0x8d27c` | `-` |
| `ldr` at 0x8d208 | load | 4 | `abs 0x8d280` | `-` |
| `ldrb` at 0x8d218 | load | 1 | `sp -0x31` | `-` |
| `ldr` at 0x8d228 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x8d230 | store | 1 | `[sp-48] +0x4` | `0x1` |
| `ldr` at 0x8d234 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x8d238 | load | 1 | `[sp-44] +0x14` | `-` |
| `ldr` at 0x8d23c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d248 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d250 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x8d25c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x8d260 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x8d268 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8d268 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8d268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8d190 | `(unknown callee) 0x125958` | `?` | `?` | `?` | `?` |
| `bl` at 0x8d1a4 | `(unknown callee) 0x558d8` | `[sp-20]` | `?` | `?` | `ret(0x125958)` |
| `bl` at 0x8d1c8 | `(unknown callee) 0x125b8c` | `[sp-20]` | `[[sp-44]+16]:1` | `sp-49` | `[[sp-44]+16]:1` |
| `bl` at 0x8d1e4 | `(unknown callee) 0x219a4` | `ret(0x125b8c)` | `?` | `?` | `ret(0x125b8c)` |
| `bl` at 0x8d210 | `(unknown callee) 0x22070` | `0x1` | `0x1679cc` | `0x189` | `0x0` |
| `bl` at 0x8d244 | `(unknown callee) 0xb8924` | `[sp-20]` | `[[sp-44]+20]:1` | `[sp-48]` | `[[sp-44]+20]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8d0b4 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8d0e0 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x8d12c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8d19c | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |
| `bne` at 0x8d1b0 | `ret(0x558d8) != 0x1` | `ret(0x558d8) == 0x1` |
| `beq` at 0x8d1dc | `ret(0x125b8c) == 0x0` | `ret(0x125b8c) != 0x0` |
| `bne` at 0x8d224 | `? != 0xc` | `? == 0xc` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8d268 | `r0` | `?` |

## `SHUB2_Init_CPUs`

- `SHUB2_Init_CPUs`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8d878 to 0x8d904.
  Established. The result from unnamed 0x6e140 is stored as a word at entry sp minus 0x34 (`bl` at 0x8d8a0, `str` at 0x8d8a8).
  A separate load, addition and word store write [sp-20] plus 1 at that destination (`ldr` at 0x8d8b8, `add` at 0x8d8bc, `str` at 0x8d8c0).
  A later load, addition and word store write [sp-44] plus 1 at entry sp minus 0x2c (`ldr` at 0x8d8e4, `add` at 0x8d8e8, `str` at 0x8d8ec).
  The return record places 0 in r0 (`ldm` at 0x8d900).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8d888 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8d88c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8d89c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x8d8a8 | store | 4 | `sp -0x34` | `ret(0x6e140)` |
| `ldr` at 0x8d8b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8d8b4 | store | 4 | `sp -0x30` | `[sp-20]` |
| `ldr` at 0x8d8b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8d8c0 | store | 4 | `sp -0x34` | `[sp-20]+1` |
| `ldr` at 0x8d8c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8d8c8 | store | 4 | `sp -0x2c` | `[sp-48]` |
| `ldr` at 0x8d8cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8d8d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8d8dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8d8e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8d8ec | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0x8d900 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8d900 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8d900 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8d8a0 | `(unknown callee) 0x6e140` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x8d8e0 | `(unknown callee) 0x8d47c` | `?` | `?` | `?` | `[sp-52]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8d894 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x8d8d8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8d900 | `r0` | `0x0` |

## `SHUB2_ITP_Present`

- `SHUB2_ITP_Present`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8d904 to 0x8da34.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x8d914, `str` at 0x8d918).
  A word store writes 0xfffff0c4 at entry sp minus 0x24 (`str` at 0x8d95c).
  A computed byte store writes 1 to an unknown effective target (`strb` at 0x8da00).
  A later word store writes 0 at entry sp minus 0x24 (`str` at 0x8da18).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8d914 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8d918 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8d91c | load | 4 | `abs 0x8da28` | `-` |
| `ldr` at 0x8d920 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8d92c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d940 | load | 4 | `abs 0x8da2c` | `-` |
| `ldr` at 0x8d944 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8d94c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d958 | load | 4 | `abs 0x8da30` | `-` |
| `str` at 0x8d95c | store | 4 | `sp -0x24` | `0xfffff0c4` |
| `ldr` at 0x8d964 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d978 | load | 4 | `abs 0x8da2c` | `-` |
| `ldr` at 0x8d97c | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8d984 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x8d98c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x8d990 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8d994 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x8d998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d9a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8d9a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d9bc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x8d9c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d9c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d9d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8d9d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d9f4 | load | 4 | `[sp-32] +0x4` | `-` |
| `strb` at 0x8da00 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x8da04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8da0c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x8da18 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x8da1c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x8da24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8da24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8da24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8d928 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8d954 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x8d9a0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x8d9d0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8da24 | `r0` | `?` |

## `SHUB2_ResetHold`

- `SHUB2_ResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8da34 to 0x8dbdc.
  Established. Word stores write 0xfffff0c4 and 0xfffff0c2 at entry sp minus 0x34 (`str` at 0x8da94, `str` at 0x8dafc).
  The result from unnamed 0x8f5e0 is stored as a word at entry sp minus 0x20 (`bl` at 0x8db7c, `str` at 0x8db84).
  A later load and word store write that result at entry sp minus 0x24 (`bl` at 0x8db7c, `ldr` at 0x8dba0, `str` at 0x8dba4).
  The return record retains an unknown value in r0 (`ldm` at 0x8dbc8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8da44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8da48 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8da50 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x8da54 | load | 4 | `abs 0x8dbcc` | `-` |
| `ldr` at 0x8da58 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8da64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8da78 | load | 4 | `abs 0x8dbd0` | `-` |
| `ldr` at 0x8da7c | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8da84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8da90 | load | 4 | `abs 0x8dbd4` | `-` |
| `str` at 0x8da94 | store | 4 | `sp -0x34` | `0xfffff0c4` |
| `ldr` at 0x8da9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8dab0 | load | 4 | `abs 0x8dbd0` | `-` |
| `ldr` at 0x8dab4 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8dabc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x8dac0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8dad0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8dad4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8dad8 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x8dadc | store | 4 | `sp -0x2c` | `[[sp-48]+8]` |
| `ldr` at 0x8dae4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8dae8 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0x8daec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8daf8 | load | 4 | `abs 0x8dbd8` | `-` |
| `str` at 0x8dafc | store | 4 | `sp -0x34` | `0xfffff0c2` |
| `ldr` at 0x8db04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8db08 | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0x8db0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8db14 | store | 4 | `sp -0x2c` | `r1+1` |
| `ldr` at 0x8db18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8db1c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8db20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8db24 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8db30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8db34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8db48 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x8db50 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8db54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8db70 | load | 4 | `[sp-48] +0x4` | `-` |
| `str` at 0x8db84 | store | 4 | `sp -0x20` | `ret(0x8f5e0)` |
| `ldr` at 0x8db88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8db94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8dba0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8dba4 | store | 4 | `sp -0x24` | `ret(0x8f5e0)` |
| `ldr` at 0x8dba8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8dbb0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x8dbb8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8dbbc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x8dbc0 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x8dbc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8dbc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8dbc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8db7c | `(unknown callee) 0x8f5e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8da60 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8da8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8dac8 | `r1 != 0xff` | `r1 == 0xff` |
| `bgt` at 0x8daf4 | `[[sp-48]+8] > r1 (signed, 32-bit)` | `[[sp-48]+8] <= r1 (signed, 32-bit)` |
| `bge` at 0x8db2c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x8db90 | `ret(0x8f5e0) == 0x0` | `ret(0x8f5e0) != 0x0` |
| `bne` at 0x8db9c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8dbc8 | `r0` | `?` |

## `SHUB2_ResetHoldMask`

- `SHUB2_ResetHoldMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8dbdc to 0x8dcdc.
  Established. An initial word store writes 0 at entry sp minus 0x20 (`str` at 0x8dbf4).
  The result from unnamed 0x3e274 is reduced by 1 and stored as a word at entry sp minus 0x18 (`bl` at 0x8dbf8, `sub` at 0x8dc00, `str` at 0x8dc04).
  The result from unnamed 0x8da34 is later stored as a word at entry sp minus 0x1c (`bl` at 0x8dc64, `str` at 0x8dc6c).
  The return record places [sp-32] in r0 (`ldr` at 0x8dccc, `ldm` at 0x8dcd8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8dbec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8dbf4 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x8dc04 | store | 4 | `sp -0x18` | `ret(0x3e274)-1` |
| `ldr` at 0x8dc08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dc14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8dc1c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x8dc30 | store | 4 | `sp -0x18` | `ret(0x3e274)-1` |
| `ldr` at 0x8dc34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dc44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dc4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8dc5c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8dc6c | store | 4 | `sp -0x1c` | `ret(0x8da34)` |
| `ldr` at 0x8dc70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8dc7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8dc88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8dc8c | store | 4 | `sp -0x20` | `ret(0x8da34)` |
| `ldr` at 0x8dc90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8dc98 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x8dcac | store | 4 | `sp -0x18` | `ret(0x3e274)-1` |
| `ldr` at 0x8dcb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dcbc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8dcc4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x8dccc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x8dcd8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8dcd8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8dcd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8dbf8 | `(unknown callee) 0x3e274` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x8dc24 | `(unknown callee) 0x3e274` | `ret(0x3e274)` | `?` | `?` | `?` |
| `bl` at 0x8dc64 | `(unknown callee) 0x8da34` | `?` | `0xff` | `?` | `?` |
| `bl` at 0x8dca0 | `(unknown callee) 0x3e274` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x8dc10 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x8dc3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8dc58 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8dc78 | `ret(0x8da34) == 0x0` | `ret(0x8da34) != 0x0` |
| `bne` at 0x8dc84 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `blt` at 0x8dcb8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8dcd8 | `r0` | `[sp-32]` |

## `SHUB2_ResetRelease`

- `SHUB2_ResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8dcdc to 0x8e120.
  Established. An initial word store writes 0 at entry sp minus 0x28 (`str` at 0x8dcf8).
  The result from unnamed 0x6e190 is stored as a word at entry sp minus 0x44 (`bl` at 0x8de3c, `str` at 0x8de40).
  The result from a separate call to unnamed 0x13ccc0 is stored as a word at entry sp minus 0x4c (`bl` at 0x8dfd0, `str` at 0x8dfe0).
  A later word store writes the result from unnamed 0x8fb4c at entry sp minus 0x24 (`bl` at 0x8e0b0, `str` at 0x8e0b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8dcec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8dcf0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x8dcf8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8dcfc | load | 4 | `abs 0x8e104` | `-` |
| `ldr` at 0x8dd00 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8dd0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dd20 | load | 4 | `abs 0x8e108` | `-` |
| `ldr` at 0x8dd24 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8dd2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8dd38 | load | 4 | `abs 0x8e10c` | `-` |
| `str` at 0x8dd3c | store | 4 | `sp -0x40` | `0xfffff0c4` |
| `ldr` at 0x8dd44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dd58 | load | 4 | `abs 0x8e108` | `-` |
| `ldr` at 0x8dd5c | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8dd64 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x8dd68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8dd78 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x8dd7c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8dd80 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x8dd84 | store | 4 | `sp -0x30` | `[[sp-52]+8]` |
| `ldr` at 0x8dd8c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8dd90 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x8dd94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8dda0 | load | 4 | `abs 0x8e110` | `-` |
| `str` at 0x8dda4 | store | 4 | `sp -0x40` | `0xfffff0c2` |
| `ldr` at 0x8ddac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8ddb0 | store | 4 | `sp -0x2c` | `r1` |
| `ldr` at 0x8ddb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8ddbc | store | 4 | `sp -0x30` | `r1+1` |
| `ldr` at 0x8ddc0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8ddc4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8ddc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8ddcc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8dddc | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x8dde4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x8dde8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8ddec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8de00 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x8de08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8de0c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8de40 | store | 4 | `sp -0x44` | `ret(0x6e190)` |
| `ldr` at 0x8de48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8de4c | store | 4 | `sp -0x44` | `[sp-24]` |
| `ldr` at 0x8de50 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8de54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8de68 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x8de70 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8de74 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x8de84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x8de9c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x8dea4 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x8dea8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8deb0 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x8deb8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8dec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8def0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8def4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8df10 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x8df18 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8df2c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8df30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8df44 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x8df4c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8df54 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x8df58 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x8df60 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x8df64 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8df68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8df84 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x8df8c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8df94 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x8df9c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x8dfa4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8dfa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8dfbc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x8dfc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8dfc8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x8dfcc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8dfd8 | load | 4 | `abs 0x8e114` | `-` |
| `str` at 0x8dfdc | store | 4 | `sp -0x50` | `0x167b34` |
| `str` at 0x8dfe0 | store | 4 | `sp -0x4c` | `ret(0x13ccc0)` |
| `ldr` at 0x8dfe8 | load | 4 | `abs 0x8e118` | `-` |
| `ldr` at 0x8dfec | load | 4 | `abs 0x8e11c` | `-` |
| `ldr` at 0x8dffc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8e004 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8e00c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8e010 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8e014 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e018 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8e024 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8e028 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e03c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldrb` at 0x8e044 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8e048 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e064 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8e068 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e07c | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x8e084 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8e088 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e0a4 | load | 4 | `[sp-52] +0x4` | `-` |
| `str` at 0x8e0b8 | store | 4 | `sp -0x24` | `ret(0x8fb4c)` |
| `ldr` at 0x8e0bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e0c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8e0d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8e0d8 | store | 4 | `sp -0x28` | `ret(0x8fb4c)` |
| `ldr` at 0x8e0e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8e0e8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x8e0f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8e0f4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x8e0f8 | load | 4 | `sp -0x40` | `-` |
| `ldm` at 0x8e100 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x8e100 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8e100 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8e100 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8de18 | `(unknown callee) 0x6e140` | `?` | `[sp-52]` | `?` | `?` |
| `bl` at 0x8de28 | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0x8de3c | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0x8de8c | `(unknown callee) 0x7fe94` | `?` | `?` | `?` | `?` |
| `bl` at 0x8ded0 | `(unknown callee) 0x81b18` | `?` | `0x23` | `?` | `?` |
| `bl` at 0x8dee0 | `(unknown callee) 0x6e140` | `ret(0x81b18)` | `?` | `?` | `ret(0x81b18)` |
| `bl` at 0x8df1c | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `[sp-52]` | `?` | `?` |
| `bl` at 0x8dfd0 | `(unknown callee) 0x13ccc0` | `[sp-72]` | `[sp-52]` | `?` | `?` |
| `bl` at 0x8dff4 | `(unknown callee) 0x22070` | `0x1` | `0x1679cc` | `0x2f1` | `0x0` |
| `bl` at 0x8e050 | `(unknown callee) 0x125958` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e060 | `(unknown callee) 0x134634` | `ret(0x125958)` | `?` | `?` | `ret(0x125958)` |
| `bl` at 0x8e0b0 | `(unknown callee) 0x8fb4c` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e0dc | `(unknown callee) 0x1346fc` | `ret(0x8fb4c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8dd08 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8dd34 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8dd70 | `r1 != 0xff` | `r1 == 0xff` |
| `bgt` at 0x8dd9c | `[[sp-52]+8] > r1 (signed, 32-bit)` | `[[sp-52]+8] <= r1 (signed, 32-bit)` |
| `bge` at 0x8ddd4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0x8de14 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x8de24 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0x8de34 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `beq` at 0x8de98 | `ret(0x7fe94) == 0x0` | `ret(0x7fe94) != 0x0` |
| `bne` at 0x8dec0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x8dedc | `ret(0x81b18) == 0x1` | `ret(0x81b18) != 0x1` |
| `ble` at 0x8deec | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `beq` at 0x8df28 | `? == ret(0x6e190)` | `? != ret(0x6e190)` |
| `bge` at 0x8e020 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x8e05c | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |
| `beq` at 0x8e0c4 | `ret(0x8fb4c) == 0x0` | `ret(0x8fb4c) != 0x0` |
| `bne` at 0x8e0d0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8e100 | `r0` | `?` |
