# ARM flash and port callers in 1.40.5

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
cmdFlashBoot, cmdFlashSetDefault, brickGetSerialSlab and brickGetSerial have unresolved computed transfers. Tables cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/flash-port-callers-facts.txt) retain all tool sections.

## `FlashValidateImage`

- `FlashValidateImage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa54b0 to 0xa5940.
  Established. A byte absolute assignment at `0x14a690` writes `0x0` (`strb` at 0xa54e8). The branch
  takes unsigned `ret(0x1aff8) < [sp-24]` to `0xa554c` (`blo` at 0xa552c). Included later four-byte
  assignments at `[sp+4] +0x0` and `[sp+8] +0x0` retain `[sp-60]` and `[sp-64]` respectively (`str`
  at 0xa56c8, `str` at 0xa589c). Image validation criteria and pointee format remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa54c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa54c4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa54c8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa54cc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xa54d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa54d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa54dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa54e0 | load | 4 | `abs 0xa590c` | `-` |
| `strb` at 0xa54e8 | store | 1 | `abs 0x14a690` | `0x0` |
| `ldr` at 0xa54ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa54f8 | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa54fc | load | 4 | `abs 0xa5910` | `-` |
| `ldr` at 0xa5500 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa550c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5510 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xa551c | store | 4 | `sp -0x2c` | `ret(0x1aff8)` |
| `ldr` at 0xa5520 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa5524 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5534 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa553c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa554c | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa5550 | load | 4 | `abs 0xa5914` | `-` |
| `ldr` at 0xa5554 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa5560 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa5574 | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa5578 | load | 4 | `abs 0xa5918` | `-` |
| `ldr` at 0xa557c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa5588 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa558c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa5594 | store | 4 | `sp -0x3c` | `([sp-44] + [sp-36])` |
| `ldr` at 0xa5598 | load | 4 | `sp -0x3c` | `-` |
| `ldrh` at 0xa559c | load | 2 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0xa55b4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa55b8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa55c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa55c8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa55d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa55d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa55e8 | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa55ec | load | 4 | `abs 0xa591c` | `-` |
| `ldr` at 0xa55f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa55fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa5600 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa561c | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa5620 | load | 4 | `abs 0xa5920` | `-` |
| `ldr` at 0xa5624 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa5634 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xa563c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa5640 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa5644 | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xa5648 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa564c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa565c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5660 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa5670 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa5678 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1aff8))` |
| `ldr` at 0xa567c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa5684 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xa5688 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa5690 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xa5698 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa56a4 | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa56a8 | load | 4 | `abs 0xa5924` | `-` |
| `ldr` at 0xa56b4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa56c0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa56c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa56c8 | store | 4 | `[sp+4] +0x0` | `[sp-60]` |
| `ldr` at 0xa56cc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa56d0 | load | 4 | `[sp-60] +0x18` | `-` |
| `str` at 0xa56dc | store | 4 | `sp -0x2c` | `ret(0x1aff8)` |
| `ldr` at 0xa56e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa56ec | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa56f0 | load | 4 | `abs 0xa5928` | `-` |
| `ldr` at 0xa56fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa5700 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa5708 | store | 4 | `sp -0x40` | `(ret(0x1aff8) + [sp-36])` |
| `ldr` at 0xa570c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa5710 | store | 4 | `sp -0x30` | `[sp-20]` |
| `str` at 0xa5718 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xa5720 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa5724 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa5728 | load | 4 | `[sp-60] +0xc` | `-` |
| `ldr` at 0xa5738 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa5748 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa574c | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xa575c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa5764 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1aff8))` |
| `ldr` at 0xa5768 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa5770 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xa5774 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa577c | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xa5784 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa5788 | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0xa5798 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa57a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa57a8 | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0xa57bc | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa57c0 | load | 4 | `abs 0xa592c` | `-` |
| `str` at 0xa57cc | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa57d4 | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa57d8 | load | 4 | `abs 0xa5930` | `-` |
| `ldr` at 0xa57e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa57f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa57f4 | load | 4 | `[sp-64] +0x4` | `-` |
| `str` at 0xa5800 | store | 4 | `sp -0x34` | `ret(0x1aff8)` |
| `str` at 0xa5808 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xa5810 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa5814 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa5818 | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xa581c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa5820 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa5830 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa5834 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa5844 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa584c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1aff8))` |
| `ldr` at 0xa5850 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa5858 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xa585c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa5864 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xa586c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa5878 | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa587c | load | 4 | `abs 0xa5934` | `-` |
| `ldr` at 0xa5888 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa5894 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xa5898 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa589c | store | 4 | `[sp+8] +0x0` | `[sp-64]` |
| `ldr` at 0xa58a0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa58ac | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa58b0 | load | 4 | `abs 0xa590c` | `-` |
| `str` at 0xa58b4 | store | 4 | `[sp] +0x0` | `0x14a690` |
| `str` at 0xa58bc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xa58c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa58d0 | load | 4 | `abs 0xa5938` | `-` |
| `ldr` at 0xa58d4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xa58d8 | load | 4 | `abs 0xa593c` | `-` |
| `ldr` at 0xa58dc | load | 4 | `abs 0xa590c` | `-` |
| `ldr` at 0xa58e4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa58f0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa58f4 | load | 4 | `abs 0xa590c` | `-` |
| `str` at 0xa58f8 | store | 4 | `[sp] +0x0` | `0x14a690` |
| `str` at 0xa5900 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa5904 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xa5908 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa5908 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa5908 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5504 | `(unknown callee) 0x1b52c` | `0x14a690` | `0x126bd0` | `[sp-20]` | `[sp-20]` |
| `bl` at 0xa5514 | `(unknown callee) 0x1aff8` | `[[sp-20]]` | `r1` | `0x14a690` | `[sp-20]` |
| `bl` at 0xa5558 | `(unknown callee) 0x1b52c` | `0x14a690` | `0x126bf0` | `ret(0x1aff8)` | `?` |
| `bl` at 0xa5564 | `(unknown callee) 0xcbbb0` | `ret(0x1aff8)` | `?` | `([sp-24] + [sp-28])` | `ret(0x1aff8)` |
| `bl` at 0xa5580 | `(unknown callee) 0x1b52c` | `0x14a690` | `0x126c10` | `[sp-44]` | `ret(0xcbbb0)` |
| `bl` at 0xa55a4 | `(unknown callee) 0x1b298` | `[([sp-44] + [sp-36])+2]:2` | `?` | `[sp-44]` | `[([sp-44] + [sp-36])+2]:2` |
| `bl` at 0xa55f4 | `(unknown callee) 0x1b52c` | `0x14a690` | `0x126c4c` | `[sp-52]` | `?` |
| `bl` at 0xa560c | `(unknown callee) 0xcbbb0` | `([sp-44] + [sp-52])` | `([sp-44] + [sp-52])` | `[sp-44]` | `([sp-44] + [sp-52])` |
| `bl` at 0xa5628 | `(unknown callee) 0x1b52c` | `0x14a690` | `0x126c70` | `[sp-52]` | `ret(0xcbbb0)` |
| `bl` at 0xa5664 | `(unknown callee) 0x1aff8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa56ac | `(unknown callee) 0x1b598` | `0x14a690` | `0x126cb8` | `?` | `?` |
| `bl` at 0xa56d4 | `(unknown callee) 0x1aff8` | `[[sp-60]+24]` | `?` | `?` | `[sp-60]` |
| `bl` at 0xa56f4 | `(unknown callee) 0x1b598` | `0x14a690` | `0x126cd4` | `?` | `ret(0x1aff8)` |
| `bl` at 0xa572c | `(unknown callee) 0x1aff8` | `[[sp-60]+12]` | `?` | `ret(0x1aff8)` | `[sp-60]` |
| `bl` at 0xa5750 | `(unknown callee) 0x1aff8` | `[[sp-48]]` | `?` | `ret(0x1aff8)` | `[sp-48]` |
| `bl` at 0xa578c | `(unknown callee) 0x1aff8` | `[[sp-60]+32]` | `?` | `ret(0x1aff8)` | `[sp-60]` |
| `bl` at 0xa57ac | `(unknown callee) 0x1aff8` | `[[sp-60]+32]` | `?` | `ret(0x1aff8)` | `[sp-60]` |
| `bl` at 0xa57c4 | `(unknown callee) 0x1b598` | `0x14a690` | `0x126cf0` | `?` | `ret(0x1aff8)` |
| `bl` at 0xa57dc | `(unknown callee) 0x1b598` | `0x14a690` | `0x126d04` | `?` | `ret(0x1aff8)` |
| `bl` at 0xa57f8 | `(unknown callee) 0x1aff8` | `[[sp-64]+4]` | `?` | `?` | `[sp-64]` |
| `bl` at 0xa5838 | `(unknown callee) 0x1aff8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa5880 | `(unknown callee) 0x1b598` | `0x14a690` | `0x126d18` | `?` | `?` |
| `bl` at 0xa58e0 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x126d38` | `0x14a690` | `0x145ae8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa54f4 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xa552c | `ret(0x1aff8) < [sp-24] (unsigned, 32-bit)` | `ret(0x1aff8) >= [sp-24] (unsigned, 32-bit)` |
| `bhi` at 0xa5544 | `ret(0x1aff8) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `ret(0x1aff8) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0xa5570 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xa55c0 | `[sp-52] <= 0x1f (unsigned, 32-bit)` | `[sp-52] > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0xa55e0 | `([sp-44] + [sp-52]) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `([sp-44] + [sp-52]) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0xa5618 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xa5654 | `C == 0` | `not (C == 0)` |
| `beq` at 0xa56a0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa56bc | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `bne` at 0xa56e8 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `blo` at 0xa5740 | `[sp-56] < ret(0x1aff8) (unsigned, 32-bit)` | `[sp-56] >= ret(0x1aff8) (unsigned, 32-bit)` |
| `beq` at 0xa57a0 | `[sp-40] == ret(0x1aff8)` | `[sp-40] != ret(0x1aff8)` |
| `bne` at 0xa57b8 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `beq` at 0xa57ec | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `blo` at 0xa5828 | `C == 0` | `not (C == 0)` |
| `beq` at 0xa5874 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa5890 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0xa58a8 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0xa58cc | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xa58ec | `[sp] == 0x0` | `[sp] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa5908 | `r0` | `[sp-68]` |

## `FlashInit`

- `FlashInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5940 to 0xa5ad0.
  Established. An included four-byte absolute assignment at `0x14a68c` writes `[0x14a68c]+1` (`str`
  at 0xa598c). The unnamed `0xb5f4c` call has `0x2b27ac`, `0x126d80`, `0x5`, `0x14` (`bl` at
  0xa59bc). The later unnamed `0x22480` call carries `0x2b27a8`, `0x126dd0`, `0xa5ad0`, `0x0` (`bl`
  at 0xa5a90). Flash setup fields and registration effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa5950 | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5954 | load | 4 | `abs 0x14a68c` | `-` |
| `ldr` at 0xa5960 | load | 4 | `abs 0xa5a9c` | `-` |
| `str` at 0xa5964 | store | 4 | `sp -0x30` | `0x126d4c` |
| `ldr` at 0xa596c | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa5970 | load | 4 | `abs 0xa5aa4` | `-` |
| `ldr` at 0xa597c | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5980 | load | 4 | `abs 0xa5a98` | `-` |
| `ldr` at 0xa5984 | load | 4 | `abs 0x14a68c` | `-` |
| `str` at 0xa598c | store | 4 | `abs 0x14a68c` | `[0x14a68c]+1` |
| `ldr` at 0xa5990 | load | 4 | `abs 0xa5aa8` | `-` |
| `str` at 0xa59a4 | store | 4 | `sp -0x18` | `0x19` |
| `str` at 0xa59ac | store | 4 | `sp -0x1c` | `0x14` |
| `ldr` at 0xa59b0 | load | 4 | `abs 0xa5aac` | `-` |
| `ldr` at 0xa59b4 | load | 4 | `abs 0xa5ab0` | `-` |
| `str` at 0xa59c4 | store | 4 | `sp -0x14` | `ret(0xb5f4c)` |
| `ldr` at 0xa59c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa59d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa59e0 | load | 4 | `abs 0xa5ab4` | `-` |
| `str` at 0xa59e4 | store | 4 | `sp -0x30` | `0x126d88` |
| `str` at 0xa59e8 | store | 4 | `sp -0x2c` | `ret(0x21704)` |
| `ldr` at 0xa59f0 | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa59f4 | load | 4 | `abs 0xa5ab8` | `-` |
| `ldr` at 0xa5a00 | load | 4 | `abs 0xa5aa8` | `-` |
| `ldr` at 0xa5a04 | load | 4 | `abs 0xa5aa8` | `-` |
| `ldr` at 0xa5a08 | load | 4 | `abs 0x2b27a0` | `-` |
| `ldr` at 0xa5a10 | load | 4 | `abs 0xa5aac` | `-` |
| `str` at 0xa5a1c | store | 4 | `sp -0x14` | `ret(0xb4b18)` |
| `ldr` at 0xa5a20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5a2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5a38 | load | 4 | `abs 0xa5abc` | `-` |
| `str` at 0xa5a3c | store | 4 | `sp -0x30` | `0x126dac` |
| `str` at 0xa5a40 | store | 4 | `sp -0x2c` | `ret(0x21704)` |
| `ldr` at 0xa5a48 | load | 4 | `abs 0xa5aa0` | `-` |
| `ldr` at 0xa5a4c | load | 4 | `abs 0xa5ac0` | `-` |
| `str` at 0xa5a5c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xa5a64 | store | 4 | `sp -0x2c` | `0x400` |
| `ldr` at 0xa5a68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa5a6c | store | 4 | `sp -0x28` | `[sp-24]` |
| `ldr` at 0xa5a70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa5a74 | store | 4 | `sp -0x24` | `[sp-28]` |
| `str` at 0xa5a7c | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0xa5a80 | load | 4 | `abs 0xa5ac4` | `-` |
| `ldr` at 0xa5a84 | load | 4 | `abs 0xa5ac8` | `-` |
| `ldr` at 0xa5a88 | load | 4 | `abs 0xa5acc` | `-` |
| `ldmdb` at 0xa5a94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa5a94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa5a94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5978 | `(unknown callee) 0x21eb8` | `0x9` | `0x126d64` | `0xbc1` | `0x0` |
| `bl` at 0xa599c | `(unknown callee) 0xb5e34` | `0x2b27a0` | `0x10` | `0x0` | `[0x14a68c]+1` |
| `bl` at 0xa59bc | `(unknown callee) 0xb5f4c` | `0x2b27ac` | `0x126d80` | `0x5` | `0x14` |
| `bl` at 0xa59d8 | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0xa59fc | `(unknown callee) 0x21eb8` | `0x9` | `0x126d64` | `0xbd9` | `0x0` |
| `bl` at 0xa5a14 | `(unknown callee) 0xb4b18` | `0x2b27a0` | `[0x2b27a0]` | `0x1` | `0x2b27ac` |
| `bl` at 0xa5a30 | `(unknown callee) 0x21704` | `ret(0xb4b18)` | `?` | `?` | `ret(0xb4b18)` |
| `bl` at 0xa5a54 | `(unknown callee) 0x21eb8` | `0x9` | `0x126d64` | `0xbe2` | `0x0` |
| `bl` at 0xa5a90 | `(unknown callee) 0x22480` | `0x2b27a8` | `0x126dd0` | `0xa5ad0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa595c | `[0x14a68c] == 0x0` | `[0x14a68c] != 0x0` |
| `beq` at 0xa59d0 | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `beq` at 0xa5a28 | `ret(0xb4b18) == 0x0` | `ret(0xb4b18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa5a94 | `r0` | `ret(0x22480)` |

## `FlashIRouter`

- `FlashIRouter`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5bcc to 0xa6460.
  Established. Included byte stores at `sp -0x88` and `sp -0x87` and a halfword store at `sp -0x86`
  retain `0x0` (`strb` at 0xa5d38, `strb` at 0xa5d40, `strh` at 0xa5d58). The unnamed `0xa54b0` call
  carries `ret(0xcbbb0)`, `[0x2b4428]`, `[0x2b4438]`, `0x0` (`bl` at 0xa5df8). A later unnamed
  `0xb74d4` call has `0x0`, `0x0`, `[sp-20]`, `[sp-188]` (`bl` at 0xa62f0). Flash routing and reply
  encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa5bdc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa5be4 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0xa5bf0 | store | 4 | `sp -0x1c` | `ret(0x4f4d0)` |
| `ldr` at 0xa5bf4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa5bf8 | load | 2 | `[sp-20] +0x22` | `-` |
| `str` at 0xa5bfc | store | 4 | `sp -0x20` | `[[sp-20]+34]:2` |
| `str` at 0xa5c04 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0xa5c08 | load | 4 | `abs 0xa644c` | `-` |
| `ldr` at 0xa5c0c | load | 4 | `abs 0x14a6f8` | `-` |
| `ldr` at 0xa5c1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa5c20 | store | 4 | `sp -0xcc` | `[sp-32]` |
| `ldr` at 0xa5c24 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5c30 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5c3c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5c4c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5c5c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5c60 | load | 4 | `abs 0x2b4434` | `-` |
| `ldr` at 0xa5c6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5c70 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xa5c7c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5c80 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa5c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5c94 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0xa5ca4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa5cc4 | store | 4 | `sp -0x94` | `0x5a` |
| `ldr` at 0xa5cc8 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5ccc | load | 4 | `abs 0x2b4434` | `-` |
| `ldr` at 0xa5cd8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5cdc | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xa5ce8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5cec | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa5cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5d00 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0xa5d10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa5d30 | store | 4 | `sp -0x94` | `0x5a` |
| `strb` at 0xa5d38 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0xa5d40 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0xa5d44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5d48 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa5d50 | store | 4 | `sp -0x84` | `([sp-24] + [sp-148])` |
| `strh` at 0xa5d58 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0xa5d5c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5d60 | load | 4 | `abs 0x2b4448` | `-` |
| `strb` at 0xa5d70 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa5d78 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0xa5d7c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5d80 | load | 4 | `abs 0x2b4448` | `-` |
| `ldr` at 0xa5d84 | load | 4 | `[0x2b4448] +0x0` | `-` |
| `str` at 0xa5d88 | store | 4 | `sp -0x7c` | `[[0x2b4448]]` |
| `strh` at 0xa5d90 | store | 2 | `sp -0x7e` | `0x0` |
| `strb` at 0xa5d9c | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa5da4 | store | 1 | `sp -0x7f` | `0x0` |
| `str` at 0xa5dac | store | 4 | `sp -0x7c` | `0x0` |
| `strh` at 0xa5db4 | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0xa5db8 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5dbc | load | 4 | `abs 0x2b4428` | `-` |
| `ldr` at 0xa5dc8 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5dcc | load | 4 | `abs 0xa6450` | `-` |
| `str` at 0xa5dd4 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xa5ddc | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0xa5de4 | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0xa5dec | load | 4 | `abs 0x2b4428` | `-` |
| `ldr` at 0xa5df0 | load | 4 | `abs 0x2b4438` | `-` |
| `str` at 0xa5e00 | store | 4 | `sp -0x90` | `ret(0xa54b0)` |
| `ldr` at 0xa5e04 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa5e14 | store | 1 | `sp -0x78` | `0x0` |
| `strb` at 0xa5e1c | store | 1 | `sp -0x77` | `0x0` |
| `str` at 0xa5e24 | store | 4 | `sp -0x74` | `0x0` |
| `strh` at 0xa5e2c | store | 2 | `sp -0x76` | `0x0` |
| `strb` at 0xa5e34 | store | 1 | `sp -0x70` | `0x0` |
| `strb` at 0xa5e3c | store | 1 | `sp -0x6f` | `0x0` |
| `str` at 0xa5e44 | store | 4 | `sp -0x6c` | `0x0` |
| `strh` at 0xa5e4c | store | 2 | `sp -0x6e` | `0x0` |
| `ldr` at 0xa5e54 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5e58 | load | 4 | `abs 0x2b4428` | `-` |
| `ldr` at 0xa5e64 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5e68 | load | 4 | `abs 0x2b4428` | `-` |
| `str` at 0xa5e70 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0xa5e74 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa5e78 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0xa5e88 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa5e90 | store | 4 | `sp -0xa4` | `(ret(0x1aff8) + [sp-160])` |
| `strb` at 0xa5e98 | store | 1 | `sp -0x78` | `0x1` |
| `strb` at 0xa5ea0 | store | 1 | `sp -0x77` | `0x0` |
| `ldr` at 0xa5ea4 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa5ea8 | store | 4 | `sp -0x74` | `(ret(0x1aff8) + [sp-160])` |
| `ldr` at 0xa5eac | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0xa5ebc | store | 2 | `sp -0x76` | `(ret(0x1b2bc)+1 & 0xffff)` |
| `ldr` at 0xa5ec0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa5ec4 | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0xa5ed4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa5edc | store | 4 | `sp -0xa4` | `(ret(0x1aff8) + [sp-160])` |
| `strb` at 0xa5ee4 | store | 1 | `sp -0x70` | `0x1` |
| `strb` at 0xa5eec | store | 1 | `sp -0x6f` | `0x0` |
| `ldr` at 0xa5ef0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa5ef4 | store | 4 | `sp -0x6c` | `(ret(0x1aff8) + [sp-160])` |
| `ldr` at 0xa5ef8 | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0xa5f08 | store | 2 | `sp -0x6e` | `(ret(0x1b2bc)+1 & 0xffff)` |
| `ldr` at 0xa5f0c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5f10 | load | 4 | `abs 0x2b442c` | `-` |
| `ldr` at 0xa5f1c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5f20 | load | 4 | `abs 0x2b442c` | `-` |
| `ldr` at 0xa5f2c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5f30 | load | 4 | `abs 0xa6450` | `-` |
| `str` at 0xa5f38 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xa5f40 | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0xa5f48 | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0xa5f50 | load | 4 | `abs 0x2b442c` | `-` |
| `ldr` at 0xa5f54 | load | 4 | `abs 0x2b4438` | `-` |
| `str` at 0xa5f64 | store | 4 | `sp -0x90` | `ret(0xa54b0)` |
| `str` at 0xa5f70 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xa5f74 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa5f84 | store | 1 | `sp -0x68` | `0x0` |
| `strb` at 0xa5f8c | store | 1 | `sp -0x67` | `0x0` |
| `str` at 0xa5f94 | store | 4 | `sp -0x64` | `0x0` |
| `strh` at 0xa5f9c | store | 2 | `sp -0x66` | `0x0` |
| `strb` at 0xa5fa4 | store | 1 | `sp -0x60` | `0x0` |
| `strb` at 0xa5fac | store | 1 | `sp -0x5f` | `0x0` |
| `str` at 0xa5fb4 | store | 4 | `sp -0x5c` | `0x0` |
| `strh` at 0xa5fbc | store | 2 | `sp -0x5e` | `0x0` |
| `ldr` at 0xa5fc4 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5fc8 | load | 4 | `abs 0x2b442c` | `-` |
| `ldr` at 0xa5fd4 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa5fd8 | load | 4 | `abs 0x2b442c` | `-` |
| `str` at 0xa5fe0 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xa5fe4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa5fe8 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0xa5ff8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa6000 | store | 4 | `sp -0xa0` | `(ret(0x1aff8) + [sp-164])` |
| `strb` at 0xa6008 | store | 1 | `sp -0x68` | `0x1` |
| `strb` at 0xa6010 | store | 1 | `sp -0x67` | `0x0` |
| `ldr` at 0xa6014 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa6018 | store | 4 | `sp -0x64` | `(ret(0x1aff8) + [sp-164])` |
| `ldr` at 0xa601c | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0xa602c | store | 2 | `sp -0x66` | `(ret(0x1b2bc)+1 & 0xffff)` |
| `ldr` at 0xa6030 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa6034 | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0xa6044 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa604c | store | 4 | `sp -0xa0` | `(ret(0x1aff8) + [sp-164])` |
| `strb` at 0xa6054 | store | 1 | `sp -0x60` | `0x1` |
| `strb` at 0xa605c | store | 1 | `sp -0x5f` | `0x0` |
| `ldr` at 0xa6060 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa6064 | store | 4 | `sp -0x5c` | `(ret(0x1aff8) + [sp-164])` |
| `ldr` at 0xa6068 | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0xa6078 | store | 2 | `sp -0x5e` | `(ret(0x1b2bc)+1 & 0xffff)` |
| `strb` at 0xa6080 | store | 1 | `sp -0x58` | `0x0` |
| `strb` at 0xa6088 | store | 1 | `sp -0x57` | `0x0` |
| `ldr` at 0xa608c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa6090 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa6094 | load | 4 | `abs 0x2b4430` | `-` |
| `ldr` at 0xa6098 | load | 4 | `abs 0x2b4428` | `-` |
| `str` at 0xa60a8 | store | 4 | `sp -0x54` | `?` |
| `strh` at 0xa60b0 | store | 2 | `sp -0x56` | `0x0` |
| `strb` at 0xa60b8 | store | 1 | `sp -0x50` | `0x0` |
| `strb` at 0xa60c0 | store | 1 | `sp -0x4f` | `0x0` |
| `ldr` at 0xa60c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa60c8 | store | 4 | `sp -0x4c` | `[sp-28]` |
| `strh` at 0xa60d0 | store | 2 | `sp -0x4e` | `0x0` |
| `strb` at 0xa60d8 | store | 1 | `sp -0x48` | `0x0` |
| `strb` at 0xa60e0 | store | 1 | `sp -0x47` | `0x0` |
| `ldr` at 0xa60e4 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa60e8 | load | 4 | `abs 0x2b4434` | `-` |
| `str` at 0xa60ec | store | 4 | `sp -0x44` | `[0x2b4434]` |
| `strh` at 0xa60f4 | store | 2 | `sp -0x46` | `0x0` |
| `strb` at 0xa60fc | store | 1 | `sp -0x40` | `0x0` |
| `strb` at 0xa6104 | store | 1 | `sp -0x3f` | `0x0` |
| `ldr` at 0xa6108 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa610c | load | 4 | `abs 0x2b4428` | `-` |
| `str` at 0xa6110 | store | 4 | `sp -0x3c` | `[0x2b4428]` |
| `strh` at 0xa6118 | store | 2 | `sp -0x3e` | `0x0` |
| `strb` at 0xa6120 | store | 1 | `sp -0x38` | `0x0` |
| `strb` at 0xa6128 | store | 1 | `sp -0x37` | `0x0` |
| `ldr` at 0xa612c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa6130 | load | 4 | `abs 0x2b442c` | `-` |
| `str` at 0xa6134 | store | 4 | `sp -0x34` | `[0x2b442c]` |
| `strh` at 0xa613c | store | 2 | `sp -0x36` | `0x0` |
| `str` at 0xa6144 | store | 4 | `sp -0x8c` | `0xb` |
| `ldr` at 0xa6148 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa614c | load | 4 | `abs 0x2b4444` | `-` |
| `ldr` at 0xa6158 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa615c | load | 4 | `abs 0x2b4440` | `-` |
| `strb` at 0xa616c | store | 1 | `sp -0x30` | `0x0` |
| `strb` at 0xa6174 | store | 1 | `sp -0x2f` | `0x0` |
| `ldr` at 0xa6178 | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa617c | load | 4 | `abs 0x2b4444` | `-` |
| `str` at 0xa6180 | store | 4 | `sp -0x2c` | `[0x2b4444]` |
| `strh` at 0xa6188 | store | 2 | `sp -0x2e` | `0x0` |
| `strb` at 0xa6190 | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0xa6198 | store | 1 | `sp -0x27` | `0x0` |
| `ldr` at 0xa619c | load | 4 | `abs 0xa6450` | `-` |
| `ldr` at 0xa61a0 | load | 4 | `abs 0x2b4440` | `-` |
| `str` at 0xa61a4 | store | 4 | `sp -0x24` | `[0x2b4440]` |
| `strh` at 0xa61ac | store | 2 | `sp -0x26` | `0x0` |
| `str` at 0xa61b4 | store | 4 | `sp -0x8c` | `0xd` |
| `str` at 0xa61bc | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0xa61c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa61cc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa61d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa61dc | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0xa61ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa61f0 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0xa61f4 | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0xa61fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa6208 | store | 4 | `sp -0xb4` | `ret(0xb823c)` |
| `ldr` at 0xa620c | load | 4 | `sp -0xb4` | `-` |
| `str` at 0xa6210 | store | 4 | `sp -0xa4` | `[sp-180]` |
| `ldr` at 0xa6214 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa6218 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa6228 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa622c | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0xa6230 | store | 4 | `sp -0xb8` | `[[sp-20]+48]` |
| `ldr` at 0xa6238 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa6244 | store | 4 | `sp -0xb8` | `ret(0xb823c)` |
| `ldr` at 0xa6248 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0xa624c | store | 4 | `sp -0xa0` | `[sp-184]` |
| `ldr` at 0xa6254 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xa6258 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa6268 | store | 4 | `sp -0x90` | `ret(0xcbdf0)` |
| `strb` at 0xa6270 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0xa6278 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0xa627c | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa6280 | store | 4 | `sp -0x84` | `ret(0xcbdf0)` |
| `strh` at 0xa6288 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0xa628c | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa629c | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa62a4 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0xa62a8 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xa62ac | store | 4 | `sp -0x7c` | `[sp-168]` |
| `strh` at 0xa62b4 | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0xa62b8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa62c8 | store | 4 | `sp -0xbc` | `0x2` |
| `str` at 0xa62d4 | store | 4 | `sp -0xbc` | `0x1` |
| `str` at 0xa62dc | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0xa62e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa62ec | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0xa62f8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa62fc | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0xa630c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6310 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0xa6314 | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0xa631c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa6328 | store | 4 | `sp -0xc0` | `ret(0xb823c)` |
| `ldr` at 0xa632c | load | 4 | `sp -0xc0` | `-` |
| `str` at 0xa6330 | store | 4 | `sp -0xa4` | `[sp-192]` |
| `ldr` at 0xa6334 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa6338 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa6348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa634c | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0xa6350 | store | 4 | `sp -0xc4` | `[[sp-20]+48]` |
| `ldr` at 0xa6358 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa6364 | store | 4 | `sp -0xc4` | `ret(0xb823c)` |
| `ldr` at 0xa6368 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0xa636c | store | 4 | `sp -0xb0` | `[sp-196]` |
| `ldr` at 0xa6370 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa6374 | load | 2 | `[sp-20] +0x2e` | `-` |
| `str` at 0xa6378 | store | 4 | `sp -0xa0` | `[[sp-20]+46]:2` |
| `ldr` at 0xa6380 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xa6384 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa6388 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa6394 | store | 4 | `sp -0x90` | `ret(0xcbe20)` |
| `strb` at 0xa639c | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0xa63a4 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0xa63a8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa63ac | store | 4 | `sp -0x84` | `ret(0xcbe20)` |
| `strh` at 0xa63b4 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0xa63b8 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa63c8 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa63d0 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0xa63d4 | load | 4 | `sp -0xac` | `-` |
| `str` at 0xa63d8 | store | 4 | `sp -0x7c` | `[sp-172]` |
| `strh` at 0xa63e0 | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0xa63e4 | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa63f4 | store | 4 | `sp -0xc8` | `0x2` |
| `str` at 0xa6400 | store | 4 | `sp -0xc8` | `0x1` |
| `str` at 0xa6408 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0xa6414 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6418 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0xa6424 | load | 4 | `abs 0xa6454` | `-` |
| `str` at 0xa6428 | store | 4 | `sp -0xd8` | `0x126df8` |
| `ldr` at 0xa642c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa6430 | store | 4 | `sp -0xd4` | `[sp-32]` |
| `ldr` at 0xa6438 | load | 4 | `abs 0xa6458` | `-` |
| `ldr` at 0xa643c | load | 4 | `abs 0xa645c` | `-` |
| `ldmdb` at 0xa6448 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6448 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6448 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5be8 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0xa5c18 | `(unknown callee) 0xa5ba4` | `ret(0x4f4d0)` | `?` | `?` | `[0x14a6f8]` |
| `bl` at 0xa5cac | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa5d18 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa5dc0 | `(unknown callee) 0xcbbb0` | `[0x2b4428]` | `?` | `[sp-24]` | `0x2b4420` |
| `bl` at 0xa5df8 | `(unknown callee) 0xa54b0` | `ret(0xcbbb0)` | `[0x2b4428]` | `[0x2b4438]` | `0x0` |
| `bl` at 0xa5e5c | `(unknown callee) 0xcbbb0` | `[0x2b4428]` | `?` | `?` | `0x2b4420` |
| `bl` at 0xa5e7c | `(unknown callee) 0x1aff8` | `[[sp-152]+20]` | `?` | `ret(0xcbbb0)` | `[sp-152]` |
| `bl` at 0xa5eb0 | `(unknown callee) 0x1b2bc` | `(ret(0x1aff8) + [sp-160])` | `?` | `ret(0x1aff8)` | `(ret(0x1aff8) + [sp-160])` |
| `bl` at 0xa5ec8 | `(unknown callee) 0x1aff8` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0xa5efc | `(unknown callee) 0x1b2bc` | `(ret(0x1aff8) + [sp-160])` | `?` | `ret(0x1aff8)` | `(ret(0x1aff8) + [sp-160])` |
| `bl` at 0xa5f24 | `(unknown callee) 0xcbbb0` | `[0x2b442c]` | `?` | `?` | `0x2b4420` |
| `bl` at 0xa5f5c | `(unknown callee) 0xa54b0` | `ret(0xcbbb0)` | `[0x2b442c]` | `[0x2b4438]` | `0x0` |
| `bl` at 0xa5fcc | `(unknown callee) 0xcbbb0` | `[0x2b442c]` | `?` | `?` | `0x2b4420` |
| `bl` at 0xa5fec | `(unknown callee) 0x1aff8` | `[[sp-152]+20]` | `?` | `ret(0xcbbb0)` | `[sp-152]` |
| `bl` at 0xa6020 | `(unknown callee) 0x1b2bc` | `(ret(0x1aff8) + [sp-164])` | `?` | `ret(0x1aff8)` | `(ret(0x1aff8) + [sp-164])` |
| `bl` at 0xa6038 | `(unknown callee) 0x1aff8` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0xa606c | `(unknown callee) 0x1b2bc` | `(ret(0x1aff8) + [sp-164])` | `?` | `ret(0x1aff8)` | `(ret(0x1aff8) + [sp-164])` |
| `bl` at 0xa61d0 | `(unknown callee) 0xb74d4` | `0x0` | `0x0` | `[sp-20]` | `?` |
| `bl` at 0xa6204 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0xa6240 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa6260 | `(unknown callee) 0xcbdf0` | `[sp-164]` | `[sp-184]` | `sp-168` | `sp-168` |
| `bl` at 0xa62f0 | `(unknown callee) 0xb74d4` | `0x0` | `0x0` | `[sp-20]` | `[sp-188]` |
| `bl` at 0xa6324 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0xa6360 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa638c | `(unknown callee) 0xcbe20` | `[sp-164]` | `[sp-196]` | `[[sp-20]+46]:2` | `sp-172` |
| `bl` at 0xa641c | `(unknown callee) 0xb74d4` | `0x0` | `0x0` | `[sp-20]` | `[sp-200]` |
| `bl` at 0xa6444 | `(unknown callee) 0x21eb8` | `0x4` | `0x126d64` | `0xcdc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa5c14 | `[0x14a6f8] != 0x0` | `[0x14a6f8] == 0x0` |
| `beq` at 0xa5c2c | `[sp-32] == 0x73` | `[sp-32] != 0x73` |
| `bgt` at 0xa5c38 | `[sp-32] > 0x73 (signed, 32-bit)` | `[sp-32] <= 0x73 (signed, 32-bit)` |
| `beq` at 0xa5c44 | `[sp-32] == 0x72` | `[sp-32] != 0x72` |
| `beq` at 0xa5c54 | `[sp-32] == 0x74` | `[sp-32] != 0x74` |
| `bne` at 0xa5c68 | `[0x2b4434] != 0x2` | `[0x2b4434] == 0x2` |
| `bls` at 0xa5c78 | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xa5c8c | `? != 0x0` | `? == 0x0` |
| `bls` at 0xa5c9c | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0xa5cb8 | `ret(0xb823c) <= 0x3 (unsigned, 32-bit)` | `ret(0xb823c) > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xa5cd4 | `[0x2b4434] != 0x3` | `[0x2b4434] == 0x3` |
| `bls` at 0xa5ce4 | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xa5cf8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xa5d08 | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0xa5d24 | `ret(0xb823c) <= 0x3 (unsigned, 32-bit)` | `ret(0xb823c) > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xa5d68 | `[0x2b4448] == 0x0` | `[0x2b4448] != 0x0` |
| `bge` at 0xa5e0c | `ret(0xa54b0) >= 0x0 (signed, 32-bit)` | `ret(0xa54b0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xa5f18 | `[0x2b442c] == 0x0` | `[0x2b442c] != 0x0` |
| `bge` at 0xa5f7c | `[sp-144] >= 0x0 (signed, 32-bit)` | `[sp-144] < 0x0 (signed, 32-bit)` |
| `beq` at 0xa6154 | `[0x2b4444] == 0x0` | `[0x2b4444] != 0x0` |
| `beq` at 0xa6164 | `[0x2b4440] == 0x0` | `[0x2b4440] != 0x0` |
| `bne` at 0xa61e8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa6224 | `? != 0x0` | `? == 0x0` |
| `bge` at 0xa6294 | `ret(0xcbdf0) >= 0x0 (signed, 32-bit)` | `ret(0xcbdf0) < 0x0 (signed, 32-bit)` |
| `bge` at 0xa62c0 | `ret(0xcbdf0) >= 0x0 (signed, 32-bit)` | `ret(0xcbdf0) < 0x0 (signed, 32-bit)` |
| `bne` at 0xa6308 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa6344 | `? != 0x0` | `? == 0x0` |
| `bge` at 0xa63c0 | `ret(0xcbe20) >= 0x0 (signed, 32-bit)` | `ret(0xcbe20) < 0x0 (signed, 32-bit)` |
| `bge` at 0xa63ec | `ret(0xcbe20) >= 0x0 (signed, 32-bit)` | `ret(0xcbe20) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6448 | `r0` | `?` |

## `cmdFlashBoot`

- `cmdFlashBoot`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6460 to 0xa6bac.
  Established. The unnamed `0x1b1b4` call has `0x300`, `r1`, `r2`, `[[r0+112]]` (`bl` at 0xa6490). A
  byte assignment at `[[sp-28]] +0x0` writes `0x0` (`strb` at 0xa64a8). The unresolved computed
  transfer takes unsigned `[sp-36]+3 <= 0xa` and falls through for `[sp-36]+3 > 0xa` (`ldrls` at
  0xa64cc). Boot command destinations and selector effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6470 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa6474 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa6478 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa647c | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0xa6480 | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0xa6484 | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `ldr` at 0xa6488 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6498 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xa649c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa64a0 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xa64a8 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xa64ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa64c0 | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0xa64cc | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xa6b54 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa6b54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6b54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6b54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6490 | `(unknown callee) 0x1b1b4` | `0x300` | `r1` | `r2` | `[[r0+112]]` |
| `bl` at 0xa64bc | `(unknown callee) 0xa7010` | `0x64` | `?` | `[[sp-28]]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xa64b4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ldrls` at 0xa64cc | `[sp-36]+3 <= 0xa (unsigned, 32-bit)` | `[sp-36]+3 > 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6b54 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa64cc).

## `cmdFlashVersion`

- `cmdFlashVersion`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6bac to 0xa6dc8.
  Established. The branch takes `[0x2b4430] != [0x2b4428]` to `0xa6ca0` (`bne` at 0xa6c8c); its
  taken route includes a four-byte stack assignment at `sp -0x6c` retaining `[0x2b442c]` (`str` at
  0xa6ca8). The unnamed `0xc605c` call carries `[sp-48]`, `sp-96`, unknown, `sp-96` (`bl` at
  0xa6cd4). Version encoding and selection meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6bbc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa6bc0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa6bc4 | load | 4 | `abs 0xa6db4` | `-` |
| `ldrb` at 0xa6bc8 | load | 1 | `abs 0x126f2c` | `-` |
| `strb` at 0xa6bcc | store | 1 | `sp -0x2c` | `([0x126f2c]:1 & 0xff)` |
| `strb` at 0xa6bd8 | store | 1 | `sp -0x2b` | `0x0` |
| `strb` at 0xa6be4 | store | 1 | `sp -0x2a` | `0x0` |
| `strb` at 0xa6bf0 | store | 1 | `sp -0x29` | `0x0` |
| `strb` at 0xa6bfc | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0xa6c08 | store | 1 | `sp -0x27` | `0x0` |
| `strb` at 0xa6c14 | store | 1 | `sp -0x26` | `0x0` |
| `strb` at 0xa6c20 | store | 1 | `sp -0x25` | `0x0` |
| `strb` at 0xa6c2c | store | 1 | `sp -0x24` | `0x0` |
| `strb` at 0xa6c38 | store | 1 | `sp -0x23` | `0x0` |
| `strb` at 0xa6c44 | store | 1 | `sp -0x22` | `0x0` |
| `strb` at 0xa6c50 | store | 1 | `sp -0x21` | `0x0` |
| `strb` at 0xa6c5c | store | 1 | `sp -0x20` | `0x0` |
| `strb` at 0xa6c68 | store | 1 | `sp -0x1f` | `0x0` |
| `strb` at 0xa6c74 | store | 1 | `sp -0x1e` | `0x0` |
| `ldr` at 0xa6c78 | load | 4 | `abs 0xa6db8` | `-` |
| `ldr` at 0xa6c7c | load | 4 | `abs 0xa6db8` | `-` |
| `ldr` at 0xa6c80 | load | 4 | `abs 0x2b4430` | `-` |
| `ldr` at 0xa6c84 | load | 4 | `abs 0x2b4428` | `-` |
| `ldr` at 0xa6c90 | load | 4 | `abs 0xa6db8` | `-` |
| `ldr` at 0xa6c94 | load | 4 | `abs 0x2b4428` | `-` |
| `str` at 0xa6c98 | store | 4 | `sp -0x6c` | `[0x2b4428]` |
| `ldr` at 0xa6ca0 | load | 4 | `abs 0xa6db8` | `-` |
| `ldr` at 0xa6ca4 | load | 4 | `abs 0x2b442c` | `-` |
| `str` at 0xa6ca8 | store | 4 | `sp -0x6c` | `[0x2b442c]` |
| `ldr` at 0xa6cac | load | 4 | `sp -0x6c` | `-` |
| `str` at 0xa6cb0 | store | 4 | `sp -0x30` | `[sp-108]` |
| `ldr` at 0xa6cb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6cc4 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xa6ccc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa6ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6ce8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa6cec | load | 4 | `abs 0xa6dbc` | `-` |
| `ldr` at 0xa6cf0 | load | 4 | `abs 0xa6dc0` | `-` |
| `str` at 0xa6cfc | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xa6d04 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa6d08 | store | 4 | `sp -0x64` | `[sp-80]` |
| `ldr` at 0xa6d0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6d10 | store | 4 | `sp -0x70` | `[sp-28]` |
| `ldr` at 0xa6d14 | load | 4 | `abs 0xa6db8` | `-` |
| `ldr` at 0xa6d18 | load | 4 | `abs 0xa6db8` | `-` |
| `ldr` at 0xa6d1c | load | 4 | `abs 0x2b4430` | `-` |
| `ldr` at 0xa6d20 | load | 4 | `abs 0x2b4428` | `-` |
| `str` at 0xa6d30 | store | 4 | `sp -0x74` | `0x41` |
| `str` at 0xa6d3c | store | 4 | `sp -0x74` | `0x42` |
| `ldr` at 0xa6d40 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6d48 | store | 4 | `sp -0x90` | `[sp-80]+4` |
| `ldr` at 0xa6d4c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6d54 | store | 4 | `sp -0x8c` | `[sp-80]+6` |
| `ldr` at 0xa6d58 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6d5c | store | 4 | `sp -0x88` | `[sp-80]` |
| `ldr` at 0xa6d60 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6d68 | store | 4 | `sp -0x84` | `[sp-80]+8` |
| `ldr` at 0xa6d6c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6d74 | store | 4 | `sp -0x80` | `[sp-80]+10` |
| `ldr` at 0xa6d78 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa6d80 | store | 4 | `sp -0x7c` | `[sp-80]+12` |
| `ldr` at 0xa6d84 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa6d88 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0xa6d8c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa6d90 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa6d94 | load | 4 | `abs 0xa6dc4` | `-` |
| `ldr` at 0xa6d98 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa6d9c | load | 4 | `sp -0x74` | `-` |
| `str` at 0xa6da8 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xa6dac | load | 4 | `sp -0x68` | `-` |
| `ldmdb` at 0xa6db0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa6db0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6db0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6db0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6cbc | `(unknown callee) 0x1b1b4` | `0x80` | `[0x2b4430]` | `0x2b4420` | `[sp-108]` |
| `bl` at 0xa6cd4 | `(unknown callee) 0xc605c` | `[sp-48]` | `sp-96` | `?` | `sp-96` |
| `bl` at 0xa6cf4 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x12708c` | `0x1270c0` | `[sp-28]` |
| `bl` at 0xa6da0 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x1270c8` | `[sp-76]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa6c8c | `[0x2b4430] != [0x2b4428]` | `[0x2b4430] == [0x2b4428]` |
| `beq` at 0xa6ce0 | `ret(0xc605c) == 0x0` | `ret(0xc605c) != 0x0` |
| `bne` at 0xa6d28 | `[0x2b4430] != [0x2b4428]` | `[0x2b4430] == [0x2b4428]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6db0 | `r0` | `0x0` |

## `cmdFlashSetDefault`

- `cmdFlashSetDefault`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6dc8 to 0xa7010.
  Established. A byte assignment at `[[sp-28]] +0x0` writes `0x0` (`strb` at 0xa6e18). The
  unresolved computed transfer takes unsigned `[sp-36]+1 <= 0x5` and falls through for `[sp-36]+1 > 0x5` (`ldrls` at 0xa6e28). On known edges the later unnamed `0xcc4a4` call has `[sp-40]`, unknown,
  `[[sp-28]]`, `[sp-36]+1` (`bl` at 0xa6fa4). Default selector destinations and effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6dd8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa6ddc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa6de0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6de4 | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0xa6de8 | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0xa6dec | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `str` at 0xa6df4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa6df8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6e08 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xa6e0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6e10 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xa6e18 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0xa6e1c | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0xa6e28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa6fa0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa6fac | store | 4 | `sp -0x20` | `ret(0xcc4a4)` |
| `ldr` at 0xa6fb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa6fbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa6fc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6fcc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa6fd0 | load | 4 | `abs 0xa700c` | `-` |
| `str` at 0xa6fdc | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xa6fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6fe8 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0xa6ff0 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `str` at 0xa6ff8 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0xa6ffc | load | 4 | `sp -0x90` | `-` |
| `ldmdb` at 0xa7000 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa7000 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa7000 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa7000 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6e00 | `(unknown callee) 0x1b1b4` | `0x80` | `r1` | `r2` | `0x0` |
| `bl` at 0xa6fa4 | `(unknown callee) 0xcc4a4` | `[sp-40]` | `?` | `[[sp-28]]` | `[sp-36]+1` |
| `bl` at 0xa6fc0 | `(unknown callee) 0x21704` | `ret(0xcc4a4)` | `?` | `?` | `ret(0xcc4a4)` |
| `bl` at 0xa6fd4 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x127138` | `ret(0x21704)` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xa6e28 | `[sp-36]+1 <= 0x5 (unsigned, 32-bit)` | `[sp-36]+1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xa6fb8 | `ret(0xcc4a4) == 0x0` | `ret(0xcc4a4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa7000 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa6e28).

## `L1FRU_2m_PortStatus`

- `L1FRU_2m_PortStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8a40 to 0xa8a60.
  Established. The unnamed `0x42e28` call carries `0x0`, `0x2b2880`, `r2`, `r3` (`bl` at 0xa8a54);
  its reported return is `ret(0x42e28)` (`ldmdb` at 0xa8a58). The port-status wrapper result and
  target effect remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa8a50 | load | 4 | `abs 0xa8a5c` | `-` |
| `ldmdb` at 0xa8a58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8a58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8a58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8a54 | `(unknown callee) 0x42e28` | `0x0` | `0x2b2880` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8a58 | `r0` | `ret(0x42e28)` |

## `PBaySdrGetDcPortStateHdr`

- `PBaySdrGetDcPortStateHdr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac414 to 0xac460.
  Established. The unnamed `0x1b1b4` call has `0x50`, `r1`, `r2`, `r3` (`bl` at 0xac428). A
  four-byte stack assignment at `sp -0x14` retains `ret(0x1b1b4)` (`str` at 0xac430). The following
  unnamed `0x1b598` call carries `ret(0x1b1b4)`, `0x127d24`, unknown, `ret(0x1b1b4)` (`bl` at
  0xac43c). Header layout and allocation size remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac430 | store | 4 | `sp -0x14` | `ret(0x1b1b4)` |
| `ldr` at 0xac434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac438 | load | 4 | `abs 0xac458` | `-` |
| `ldr` at 0xac440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac444 | load | 4 | `abs 0xac45c` | `-` |
| `ldr` at 0xac44c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xac454 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac454 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac428 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xac43c | `(unknown callee) 0x1b598` | `ret(0x1b1b4)` | `0x127d24` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xac448 | `(unknown callee) 0x1b364` | `[sp-20]` | `0x127d38` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac454 | `r0` | `[sp-20]` |

## `PBaySdrGetDcPort`

- `PBaySdrGetDcPort`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac460 to 0xac540.
  Established. The branch takes signed `r1 <= 0x0` to `0xac494` (`ble` at 0xac480). A later unnamed
  `0xaddec` call has `r0`, `sp-440`, `[[sp-28]]:1`, `[[sp-28]]:1` (`bl` at 0xac4e8). A byte read
  targets `ret(0xaecd4) +0x2` (`ldrb` at 0xac52c). Port selector and reply meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac470 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac474 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xac478 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac484 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac498 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xac4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac4a8 | load | 4 | `abs 0xac53c` | `-` |
| `str` at 0xac4b0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xac4b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac4b8 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xac4c8 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xac4d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac4d8 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xac4dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac4f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac500 | store | 4 | `sp -0x1bc` | `ret(0xab0d4)` |
| `ldr` at 0xac504 | load | 4 | `sp -0x1bc` | `-` |
| `str` at 0xac514 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xac51c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xac524 | store | 4 | `sp -0x20` | `ret(0xaecd4)` |
| `ldr` at 0xac528 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xac52c | load | 1 | `ret(0xaecd4) +0x2` | `-` |
| `str` at 0xac530 | store | 4 | `sp -0x1c0` | `[ret(0xaecd4)+2]:1` |
| `ldr` at 0xac534 | load | 4 | `sp -0x1c0` | `-` |
| `ldmdb` at 0xac538 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac538 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac538 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac4e8 | `(unknown callee) 0xaddec` | `r0` | `sp-440` | `[[sp-28]]:1` | `[[sp-28]]:1` |
| `bl` at 0xac4f8 | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-440` | `?` | `sp-440` |
| `bl` at 0xac520 | `(unknown callee) 0xaecd4` | `[sp-48]` | `?` | `?` | `ret(0xab0d4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xac480 | `r1 <= 0x0 (signed, 32-bit)` | `r1 > 0x0 (signed, 32-bit)` |
| `bgt` at 0xac48c | `r1 > 0x8 (signed, 32-bit)` | `r1 <= 0x8 (signed, 32-bit)` |
| `bne` at 0xac4c0 | `[[sp-28]]:1 != 0xff` | `[[sp-28]]:1 == 0xff` |
| `bne` at 0xac50c | `ret(0xab0d4) != 0x0` | `ret(0xab0d4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac538 | `r0` | `[sp-448]` |

## `PBaySdrGetDcPortState`

- `PBaySdrGetDcPortState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac540 to 0xac68c.
  Established. A four-byte assignment at `r2 +0x0` writes `0x0` (`str` at 0xac564). Included later
  stores write `ret(0x1b1b4)` as four bytes at `[sp-28] +0x0` and `([sp-24]:1+48 & 0xff)` as a byte
  at `[sp-40] +0x3` (`str` at 0xac5ac, `strb` at 0xac5d0). The unnamed `0xac460` call carries
  `[sp-20]`, `[sp-24]`, `r2`, `0x0` (`bl` at 0xac570). Port-state representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac550 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac554 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac558 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xac55c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac564 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0xac568 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac56c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac578 | store | 4 | `sp -0x24` | `ret(0xac460)` |
| `ldr` at 0xac57c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xac58c | store | 4 | `sp -0x1cc` | `0x0` |
| `str` at 0xac5a0 | store | 4 | `sp -0x28` | `ret(0x1b1b4)` |
| `ldr` at 0xac5a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac5a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xac5ac | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xac5b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac5b4 | load | 4 | `abs 0xac670` | `-` |
| `ldr` at 0xac5c0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xac5c8 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xac5d0 | store | 1 | `[sp-40] +0x3` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0xac5d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac5e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac5e4 | load | 4 | `abs 0xac674` | `-` |
| `str` at 0xac5f0 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0xac5f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac608 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac60c | load | 4 | `abs 0xac678` | `-` |
| `ldr` at 0xac618 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac61c | load | 4 | `abs 0xac67c` | `-` |
| `ldr` at 0xac624 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac634 | load | 4 | `abs 0xac680` | `-` |
| `str` at 0xac638 | store | 4 | `sp -0x1d0` | `0x127d04` |
| `ldr` at 0xac640 | load | 4 | `abs 0xac684` | `-` |
| `str` at 0xac644 | store | 4 | `sp -0x1d0` | `0x127d70` |
| `ldr` at 0xac648 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac64c | load | 4 | `sp -0x1d0` | `-` |
| `ldr` at 0xac654 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac658 | load | 4 | `abs 0xac688` | `-` |
| `str` at 0xac664 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0xac668 | load | 4 | `sp -0x1cc` | `-` |
| `ldmdb` at 0xac66c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac66c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac66c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac570 | `(unknown callee) 0xac460` | `[sp-20]` | `[sp-24]` | `r2` | `0x0` |
| `bl` at 0xac598 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `ret(0xac460)` |
| `bl` at 0xac5b8 | `(unknown callee) 0x1b598` | `[sp-40]` | `0x127d54` | `[sp-28]` | `ret(0x1b1b4)` |
| `bl` at 0xac5e8 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127cb4` | `[sp-40]+3` | `[sp-36]` |
| `bl` at 0xac610 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127d5c` | `[sp-40]+3` | `?` |
| `bl` at 0xac620 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127d64` | `[sp-40]+3` | `?` |
| `bl` at 0xac650 | `(unknown callee) 0x1b364` | `[sp-40]` | `[sp-464]` | `?` | `?` |
| `bl` at 0xac65c | `(unknown callee) 0x1b364` | `[sp-40]` | `0x127d20` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xac584 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xac5dc | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `beq` at 0xac604 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xac630 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac66c | `r0` | `?` |

## `PBaySdrGetCardSerial`

- `PBaySdrGetCardSerial`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xad394 to 0xad860.
  Established. Included byte stores with unknown effective targets write `0x28`, `0xa` and `0x0`
  (`strb` at 0xad77c, `strb` at 0xad7c4, `strb` at 0xad7ec). The unnamed `0x1b394` call carries
  `sp-728`, `ret(0xaecd4)+1`, `0x8`, `ret(0xaecd4)+1` (`bl` at 0xad50c). A later four-byte absolute
  assignment at `0x1436a8` writes `0x0` (`str` at 0xad824). Card serial encoding and reply fields
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xad3a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xad3a8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xad3ac | load | 4 | `abs 0xad838` | `-` |
| `str` at 0xad3b0 | store | 4 | `sp -0x120` | `0x14365c` |
| `ldr` at 0xad3b4 | load | 4 | `sp -0x120` | `-` |
| `ldrb` at 0xad3b8 | load | 1 | `abs 0x14365c` | `-` |
| `ldr` at 0xad3c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xad3cc | store | 4 | `r1 +0x0` | `0x0` |
| `str` at 0xad3d4 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xad3dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xad3f4 | store | 4 | `sp -0x2cc` | `ret(0x1b1b4)` |
| `ldr` at 0xad3f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xad3fc | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xad400 | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xad404 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xad414 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xad41c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xad420 | load | 4 | `abs 0xad83c` | `-` |
| `ldr` at 0xad428 | load | 4 | `abs 0xad840` | `-` |
| `ldr` at 0xad42c | load | 4 | `abs 0x1436a8` | `-` |
| `str` at 0xad430 | store | 4 | `sp -0x2dc` | `[0x1436a8]` |
| `ldr` at 0xad438 | load | 4 | `abs 0xad83c` | `-` |
| `str` at 0xad43c | store | 4 | `sp -0x2cc` | `0x14367c` |
| `ldr` at 0xad440 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xad448 | store | 1 | `abs 0x14367c` | `0x0` |
| `ldr` at 0xad44c | load | 4 | `abs 0xad844` | `-` |
| `strh` at 0xad458 | store | 2 | `sp -0x2c2` | `0x0` |
| `ldr` at 0xad45c | load | 4 | `abs 0xad848` | `-` |
| `strb` at 0xad468 | store | 1 | `sp -0x2c3` | `0x8` |
| `ldr` at 0xad470 | load | 4 | `abs 0xad844` | `-` |
| `ldrh` at 0xad478 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xad47c | load | 4 | `abs 0xad848` | `-` |
| `ldrb` at 0xad484 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xad488 | store | 4 | `sp -0x2e4` | `0x8` |
| `ldr` at 0xad48c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad494 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xad4a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xad4b4 | store | 4 | `sp -0x2bc` | `ret(0xab0d4)` |
| `ldr` at 0xad4b8 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xad4c4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xad4c8 | load | 4 | `abs 0xad84c` | `-` |
| `ldr` at 0xad4d0 | load | 4 | `abs 0xad840` | `-` |
| `str` at 0xad4d8 | store | 4 | `abs 0x1436a8` | `?` |
| `str` at 0xad4e0 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xad4e8 | load | 4 | `sp -0x130` | `-` |
| `str` at 0xad4f0 | store | 4 | `sp -0x2d0` | `ret(0xaecd4)` |
| `ldr` at 0xad4f8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0xad510 | load | 4 | `abs 0xad850` | `-` |
| `ldrb` at 0xad51c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xad528 | load | 4 | `abs 0xad850` | `-` |
| `ldrb` at 0xad534 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xad540 | load | 4 | `abs 0xad844` | `-` |
| `ldr` at 0xad544 | load | 4 | `abs 0xad850` | `-` |
| `ldrb` at 0xad550 | load | 1 | `sp -0x2d4` | `-` |
| `strh` at 0xad55c | store | 2 | `sp -0x2c2` | `?` |
| `ldr` at 0xad560 | load | 4 | `abs 0xad848` | `-` |
| `strb` at 0xad56c | store | 1 | `sp -0x2c3` | `0x3` |
| `ldr` at 0xad574 | load | 4 | `abs 0xad844` | `-` |
| `ldrh` at 0xad57c | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xad580 | load | 4 | `abs 0xad848` | `-` |
| `ldrb` at 0xad588 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xad58c | store | 4 | `sp -0x2e4` | `0x3` |
| `str` at 0xad594 | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xad598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad5a0 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xad5b0 | load | 4 | `abs 0xad854` | `-` |
| `strb` at 0xad5b8 | store | 1 | `sp -0x2c4` | `(ret(0xae028) & 0xff)` |
| `ldr` at 0xad5bc | load | 4 | `abs 0xad854` | `-` |
| `ldrb` at 0xad5c4 | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xad5d0 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xad5d4 | load | 4 | `abs 0xad84c` | `-` |
| `ldr` at 0xad5dc | load | 4 | `abs 0xad840` | `-` |
| `str` at 0xad5e4 | store | 4 | `abs 0x1436a8` | `?` |
| `str` at 0xad5ec | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xad5f4 | load | 4 | `abs 0xad848` | `-` |
| `ldr` at 0xad5f8 | load | 4 | `abs 0xad858` | `-` |
| `ldrb` at 0xad600 | load | 1 | `sp -0x117` | `-` |
| `strb` at 0xad60c | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xad610 | load | 4 | `abs 0xad848` | `-` |
| `ldr` at 0xad614 | load | 4 | `abs 0xad848` | `-` |
| `ldr` at 0xad618 | load | 4 | `abs 0xad854` | `-` |
| `ldrb` at 0xad620 | load | 1 | `sp -0x2c3` | `-` |
| `ldrb` at 0xad628 | load | 1 | `sp -0x2c4` | `-` |
| `strb` at 0xad634 | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xad638 | load | 4 | `abs 0xad844` | `-` |
| `ldr` at 0xad63c | load | 4 | `abs 0xad844` | `-` |
| `ldrh` at 0xad644 | load | 2 | `sp -0x2c2` | `-` |
| `strh` at 0xad650 | store | 2 | `sp -0x2c2` | `([sp-706]:2+3 & 0xffff)` |
| `ldr` at 0xad658 | load | 4 | `abs 0xad844` | `-` |
| `ldrh` at 0xad660 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xad664 | load | 4 | `abs 0xad848` | `-` |
| `ldrb` at 0xad66c | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xad670 | store | 4 | `sp -0x2e4` | `[sp-707]:1` |
| `str` at 0xad678 | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xad67c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xad684 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xad694 | load | 4 | `abs 0xad854` | `-` |
| `strb` at 0xad69c | store | 1 | `sp -0x2c4` | `(ret(0xae028) & 0xff)` |
| `ldr` at 0xad6a0 | load | 4 | `abs 0xad854` | `-` |
| `ldrb` at 0xad6a8 | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xad6b4 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xad6b8 | load | 4 | `abs 0xad84c` | `-` |
| `ldr` at 0xad6c0 | load | 4 | `abs 0xad840` | `-` |
| `str` at 0xad6c8 | store | 4 | `abs 0x1436a8` | `?` |
| `str` at 0xad6d0 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0xad6dc | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xad6e4 | store | 4 | `sp -0x2c0` | `0x0` |
| `ldr` at 0xad6e8 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xad6f8 | load | 4 | `abs 0xad85c` | `-` |
| `ldr` at 0xad6fc | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xad70c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xad720 | load | 4 | `abs 0xad85c` | `-` |
| `ldr` at 0xad724 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xad734 | load | 1 | `(sp-16 + [sp-284]) -0x108` | `-` |
| `strb` at 0xad740 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xad744 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xad74c | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldr` at 0xad750 | load | 4 | `sp -0x2c0` | `-` |
| `ldrb` at 0xad764 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xad77c | store | 1 | `(unknown)` | `0x28` |
| `ldr` at 0xad784 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xad794 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xad798 | load | 4 | `sp -0x2cc` | `-` |
| `ldrb` at 0xad7b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xad7b8 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xad7c4 | store | 1 | `(unknown)` | `0xa` |
| `ldrb` at 0xad7d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xad7dc | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xad7ec | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0xad7fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xad800 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xad808 | store | 4 | `sp -0x11c` | `?` |
| `ldr` at 0xad80c | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xad814 | store | 4 | `sp -0x2c0` | `[sp-704]+1` |
| `ldr` at 0xad81c | load | 4 | `abs 0xad840` | `-` |
| `str` at 0xad824 | store | 4 | `abs 0x1436a8` | `0x0` |
| `str` at 0xad82c | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xad830 | load | 4 | `sp -0x2dc` | `-` |
| `ldmdb` at 0xad834 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad834 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xad3ec | `(unknown callee) 0x1b1b4` | `0x2a` | `r1` | `r2` | `r1` |
| `bl` at 0xad424 | `(unknown callee) 0x1b598` | `[sp-716]` | `0x14367c` | `[sp-24]` | `[sp-716]` |
| `bl` at 0xad49c | `(unknown callee) 0xadf0c` | `[sp-20]` | `sp-696` | `[sp-288]` | `0x0` |
| `bl` at 0xad4ac | `(unknown callee) 0xab0d4` | `[sp-20]` | `sp-696` | `?` | `sp-696` |
| `bl` at 0xad4cc | `(unknown callee) 0x1b364` | `[sp-716]` | `0x127cb4` | `?` | `ret(0xab0d4)` |
| `bl` at 0xad4ec | `(unknown callee) 0xaecd4` | `[sp-304]` | `?` | `?` | `ret(0xab0d4)` |
| `bl` at 0xad50c | `(unknown callee) 0x1b394` | `sp-728` | `ret(0xaecd4)+1` | `0x8` | `ret(0xaecd4)+1` |
| `bl` at 0xad5a8 | `(unknown callee) 0xae028` | `[sp-20]` | `sp-696` | `[sp-288]` | `[sp-706]:2` |
| `bl` at 0xad5d8 | `(unknown callee) 0x1b364` | `[sp-716]` | `0x127cb4` | `ret(0xae028)` | `(ret(0xae028) & 0xff)` |
| `bl` at 0xad68c | `(unknown callee) 0xae028` | `[sp-20]` | `sp-696` | `[sp-288]` | `([sp-706]:2+3 & 0xffff)` |
| `bl` at 0xad6bc | `(unknown callee) 0x1b364` | `[sp-716]` | `0x127cb4` | `sp-16` | `(ret(0xae028) & 0xff)` |
| `bl` at 0xad7a4 | `(unknown callee) 0x1b394` | `[sp-716]` | `(sp-280 + [sp-284])` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xad3c0 | `[0x14365c]:1 != 0xff` | `[0x14365c]:1 == 0xff` |
| `beq` at 0xad3e4 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xad40c | `[sp-716] != 0x0` | `[sp-716] == 0x0` |
| `bne` at 0xad4c0 | `ret(0xab0d4) != 0x0` | `ret(0xab0d4) == 0x0` |
| `beq` at 0xad524 | `[sp-724]:1 == 0x0` | `[sp-724]:1 != 0x0` |
| `beq` at 0xad53c | `[sp-724]:1 == 0xff` | `[sp-724]:1 != 0xff` |
| `bne` at 0xad5cc | `(ret(0xae028) & 0xff) != 0x0` | `(ret(0xae028) & 0xff) == 0x0` |
| `bne` at 0xad6b0 | `(ret(0xae028) & 0xff) != 0x0` | `(ret(0xae028) & 0xff) == 0x0` |
| `ble` at 0xad6f0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xad714 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xad758 | `[sp-704] != 0x4` | `[sp-704] == 0x4` |
| `bls` at 0xad76c | `? <= 0x28 (unsigned, 32-bit)` | `? > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad834 | `r0` | `?` |

## `PBaySdrFlashInfo`

- `PBaySdrFlashInfo`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae7e8 to 0xae988.
  Established. The unnamed `0xab0d4` call carries `r0`, `sp-428`, `0xfffffe64`, `sp-428` (`bl` at
  0xae844). Included four-byte stack assignments at `sp -0x22c` and `sp -0x224` retain
  `[ret(0x1b46c)+16]+1` and `[ret(0x1b46c)+20]+1900` respectively (`str` at 0xae8d4, `str` at
  0xae8f4). Flash information fields and units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae7f8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae7fc | load | 4 | `abs 0xae97c` | `-` |
| `ldr` at 0xae800 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xae804 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xae80c | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xae814 | store | 4 | `sp -0x1a8` | `0xc0` |
| `str` at 0xae81c | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xae824 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xae82c | store | 4 | `sp -0x114` | `0x0` |
| `str` at 0xae834 | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xae83c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae84c | store | 4 | `sp -0x1b0` | `ret(0xab0d4)` |
| `ldr` at 0xae850 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xae85c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xae860 | store | 4 | `sp -0x214` | `[sp-36]` |
| `ldr` at 0xae864 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xae868 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0xae870 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xae874 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xae880 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xae884 | load | 1 | `[sp-36] +0x3` | `-` |
| `ldr` at 0xae890 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xae894 | load | 1 | `[sp-36] +0x2` | `-` |
| `str` at 0xae89c | store | 4 | `sp -0x1b4` | `?` |
| `str` at 0xae8b0 | store | 4 | `sp -0x210` | `ret(0x1b46c)` |
| `ldr` at 0xae8b8 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xae8bc | load | 1 | `[sp-532] +0x1` | `-` |
| `ldr` at 0xae8c0 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xae8c4 | load | 1 | `[sp-532] +0x0` | `-` |
| `ldr` at 0xae8c8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae8cc | load | 4 | `ret(0x1b46c) +0x10` | `-` |
| `str` at 0xae8d4 | store | 4 | `sp -0x22c` | `[ret(0x1b46c)+16]+1` |
| `ldr` at 0xae8d8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae8dc | load | 4 | `ret(0x1b46c) +0xc` | `-` |
| `str` at 0xae8e0 | store | 4 | `sp -0x228` | `[ret(0x1b46c)+12]` |
| `ldr` at 0xae8e4 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae8e8 | load | 4 | `ret(0x1b46c) +0x14` | `-` |
| `str` at 0xae8f4 | store | 4 | `sp -0x224` | `[ret(0x1b46c)+20]+1900` |
| `ldr` at 0xae8f8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae8fc | load | 4 | `ret(0x1b46c) +0x8` | `-` |
| `str` at 0xae900 | store | 4 | `sp -0x220` | `[ret(0x1b46c)+8]` |
| `ldr` at 0xae904 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae908 | load | 4 | `ret(0x1b46c) +0x4` | `-` |
| `str` at 0xae90c | store | 4 | `sp -0x21c` | `[ret(0x1b46c)+4]` |
| `ldr` at 0xae910 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xae914 | load | 4 | `ret(0x1b46c) +0x0` | `-` |
| `str` at 0xae918 | store | 4 | `sp -0x218` | `[ret(0x1b46c)]` |
| `ldr` at 0xae920 | load | 4 | `abs 0xae980` | `-` |
| `str` at 0xae934 | store | 4 | `sp -0x208` | `sp-516` |
| `ldr` at 0xae93c | load | 4 | `abs 0xae984` | `-` |
| `str` at 0xae940 | store | 4 | `sp -0x208` | `0x127ea4` |
| `ldr` at 0xae944 | load | 4 | `sp -0x208` | `-` |
| `str` at 0xae960 | store | 4 | `sp -0x20c` | `ret(0x1b1b4)` |
| `ldr` at 0xae964 | load | 4 | `sp -0x20c` | `-` |
| `ldr` at 0xae968 | load | 4 | `sp -0x208` | `-` |
| `ldr` at 0xae970 | load | 4 | `sp -0x20c` | `-` |
| `ldmdb` at 0xae978 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae978 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae844 | `(unknown callee) 0xab0d4` | `r0` | `sp-428` | `0xfffffe64` | `sp-428` |
| `bl` at 0xae8a8 | `(unknown callee) 0x1b46c` | `sp-436` | `?` | `?` | `sp-436` |
| `bl` at 0xae92c | `(unknown callee) 0x1b52c` | `sp-516` | `0x127e64` | `[[sp-532]+1]:1` | `[[sp-532]]:1` |
| `bl` at 0xae948 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0xae958 | `(unknown callee) 0x1b1b4` | `ret(0x1b2bc)+1` | `?` | `?` | `ret(0x1b2bc)+1` |
| `bl` at 0xae96c | `(unknown callee) 0x1b598` | `ret(0x1b1b4)` | `[sp-520]` | `?` | `ret(0x1b1b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xae858 | `ret(0xab0d4) == 0x0` | `ret(0xab0d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae978 | `r0` | `[sp-524]` |

## `brickGetSerialSlab`

- `brickGetSerialSlab`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f588 to 0x4f730.
  Established. The unnamed `0xd2790` call has `0x0`, `sp-36`, `0x148fb8`, `sp-36` (`bl` at 0x4f5c4).
  The unresolved computed transfer takes unsigned `[sp-28] <= 0x5` and falls through for `[sp-28] > 0x5` (`ldrls` at 0x4f600). A later four-byte store with unknown effective target writes
  `ret(0x1b034)` (`str` at 0x4f704). Slab serial mapping and selector effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f598 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x4f5a0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4f5a4 | load | 4 | `abs 0x4f720` | `-` |
| `ldr` at 0x4f5a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f5ac | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f5d8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4f5f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x4f600 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f668 | load | 4 | `abs 0x4f724` | `-` |
| `str` at 0x4f66c | store | 4 | `sp -0x2c` | `0x10b238` |
| `ldr` at 0x4f674 | load | 4 | `abs 0x4f728` | `-` |
| `ldr` at 0x4f678 | load | 4 | `abs 0x4f72c` | `-` |
| `ldr` at 0x4f684 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f698 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4f6a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f6ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f6c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4f6c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f6d4 | load | 4 | `abs 0x4f720` | `-` |
| `ldr` at 0x4f6d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f6dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f6f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4f6f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f704 | store | 4 | `(unknown)` | `ret(0x1b034)` |
| `ldr` at 0x4f708 | load | 4 | `abs 0x4f720` | `-` |
| `ldr` at 0x4f70c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4f710 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f714 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4f718 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4f71c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4f71c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4f71c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f71c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f71c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4f5c4 | `(unknown callee) 0xd2790` | `0x0` | `sp-36` | `0x148fb8` | `sp-36` |
| `bl` at 0x4f5e0 | `(unknown callee) 0x4f4d0` | `ret(0xd2790)` | `?` | `?` | `ret(0xd2790)` |
| `bl` at 0x4f680 | `(unknown callee) 0x21eb8` | `0x9` | `0x10b270` | `0x1af` | `0x0` |
| `bl` at 0x4f6fc | `(unknown callee) 0x1b034` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4f5b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x4f5d0 | `ret(0xd2790) != 0x0` | `ret(0xd2790) == 0x0` |
| `beq` at 0x4f5f0 | `? == 0x40` | `? != 0x40` |
| `ldrls` at 0x4f600 | `[sp-28] <= 0x5 (unsigned, 32-bit)` | `[sp-28] > 0x5 (unsigned, 32-bit)` |
| `bne` at 0x4f6a8 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x4f6d0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f71c | `r0` | `[sp-40]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4f600).

## `brickGetSerial`

- `brickGetSerial`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4f730 to 0x4f9a4.
  Established. Included four-byte absolute assignments at `0x148f80` retain `ret(0x1b034)` (`str` at
  0x4f984). The earlier unresolved computed transfer takes unsigned `[0x1434a8]:1-35 <= 0x59` and
  falls through for `[0x1434a8]:1-35 > 0x59` (`ldrls` at 0x4f790). The unnamed `0xd2790` call has
  `0x0`, `sp-32`, `r2`, `sp-32` (`bl` at 0x4f764). Serial record layout and selector meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4f744 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4f748 | load | 4 | `abs 0x4f99c` | `-` |
| `ldr` at 0x4f74c | load | 4 | `abs 0x148f80` | `-` |
| `str` at 0x4f778 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4f780 | load | 4 | `abs 0x4f9a0` | `-` |
| `ldrb` at 0x4f784 | load | 1 | `abs 0x1434a8` | `-` |
| `ldrls` at 0x4f790 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f91c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f924 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f930 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f944 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f94c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4f958 | load | 4 | `abs 0x4f99c` | `-` |
| `ldr` at 0x4f95c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4f970 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4f978 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4f984 | store | 4 | `abs 0x148f80` | `ret(0x1b034)` |
| `ldr` at 0x4f988 | load | 4 | `abs 0x4f99c` | `-` |
| `ldr` at 0x4f98c | load | 4 | `abs 0x148f80` | `-` |
| `str` at 0x4f990 | store | 4 | `sp -0x24` | `[0x148f80]` |
| `ldr` at 0x4f994 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4f998 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4f998 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4f998 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4f998 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4f764 | `(unknown callee) 0xd2790` | `0x0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x4f97c | `(unknown callee) 0x1b034` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4f754 | `[0x148f80] != 0x0` | `[0x148f80] == 0x0` |
| `bne` at 0x4f770 | `ret(0xd2790) != 0x0` | `ret(0xd2790) == 0x0` |
| `ldrls` at 0x4f790 | `[0x1434a8]:1-35 <= 0x59 (unsigned, 32-bit)` | `[0x1434a8]:1-35 > 0x59 (unsigned, 32-bit)` |
| `bne` at 0x4f92c | `? != 0x1` | `? == 0x1` |
| `beq` at 0x4f954 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4f998 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4f790).

## `portQuery`

- `portQuery`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42e28 to 0x431d8.
  Established. The unnamed `0x42224` call has `r0`, `sp-36`, `r2`, `sp-36` (`bl` at 0x42e54); its
  nonzero result takes the branch to `0x42e94` (`bne` at 0x42e68). A later unnamed `0x42820` call
  uses `[sp-24]`, `[sp-44]:1`, `sp-40`, `[sp-44]:1` (`bl` at 0x42f18). Port query result layout and
  status meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42e38 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x42e3c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x42e44 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x42e4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42e5c | store | 4 | `sp -0x30` | `ret(0x42224)` |
| `ldr` at 0x42e60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x42e6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x42e7c | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x42e80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42e84 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x42e88 | load | 4 | `abs 0x431a8` | `-` |
| `ldr` at 0x42e94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x42e98 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x42eb8 | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x42ebc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x42ec0 | store | 4 | `sp -0x34` | `[[sp-28]]` |
| `ldr` at 0x42ec4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42ec8 | load | 4 | `abs 0x431ac` | `-` |
| `ldr` at 0x42ed0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42ee0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x42ee8 | store | 4 | `sp -0x34` | `([sp-52] + ret(0x1b2bc))` |
| `str` at 0x42ef0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x42ef4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42ef8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x42f08 | load | 1 | `sp -0x2c` | `-` |
| `ldr` at 0x42f10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42f20 | store | 4 | `sp -0x20` | `ret(0x42820)` |
| `ldr` at 0x42f24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42f30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42f34 | load | 4 | `abs 0x431b0` | `-` |
| `ldr` at 0x42f40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42f54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42f5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42f60 | load | 4 | `abs 0x431b4` | `-` |
| `ldr` at 0x42f64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42f70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42f7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42f90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42f98 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x42f9c | load | 4 | `abs 0x431b8` | `-` |
| `ldr` at 0x42fa0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42fb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42fc4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x42fcc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x42fdc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42ff0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42ff8 | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0x42ffc | load | 1 | `sp -0x28` | `-` |
| `str` at 0x43000 | store | 4 | `sp -0x3c` | `[sp-40]:1` |
| `ldrb` at 0x43004 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x43018 | load | 4 | `abs 0x431bc` | `-` |
| `str` at 0x4301c | store | 4 | `sp -0x40` | `0x1098f0` |
| `ldr` at 0x43024 | load | 4 | `abs 0x431c0` | `-` |
| `str` at 0x43028 | store | 4 | `sp -0x40` | `0x1098f4` |
| `ldr` at 0x4302c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x43030 | store | 4 | `sp -0x6c` | `[sp-64]` |
| `ldr` at 0x43034 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x43038 | load | 4 | `abs 0x431c4` | `-` |
| `ldr` at 0x4303c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x43040 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0x43044 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x43050 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x43064 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4306c | store | 4 | `sp -0x44` | `?` |
| `ldrb` at 0x43070 | load | 1 | `sp -0x28` | `-` |
| `str` at 0x43074 | store | 4 | `sp -0x48` | `[sp-40]:1` |
| `ldrb` at 0x43078 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x4308c | load | 4 | `abs 0x431c8` | `-` |
| `str` at 0x43090 | store | 4 | `sp -0x4c` | `0x109928` |
| `ldr` at 0x43098 | load | 4 | `abs 0x431cc` | `-` |
| `str` at 0x4309c | store | 4 | `sp -0x4c` | `0x109930` |
| `ldr` at 0x430a0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x430a4 | store | 4 | `sp -0x6c` | `[sp-76]` |
| `ldrb` at 0x430a8 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x430bc | load | 4 | `abs 0x431c8` | `-` |
| `str` at 0x430c0 | store | 4 | `sp -0x50` | `0x109928` |
| `ldr` at 0x430c8 | load | 4 | `abs 0x431cc` | `-` |
| `str` at 0x430cc | store | 4 | `sp -0x50` | `0x109930` |
| `ldr` at 0x430d0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x430d4 | store | 4 | `sp -0x68` | `[sp-80]` |
| `ldrb` at 0x430d8 | load | 1 | `sp -0x28` | `-` |
| `ldrb` at 0x430ec | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x43100 | load | 4 | `abs 0x431d0` | `-` |
| `str` at 0x43104 | store | 4 | `sp -0x58` | `0x109938` |
| `ldr` at 0x4310c | load | 4 | `abs 0x431c0` | `-` |
| `str` at 0x43110 | store | 4 | `sp -0x58` | `0x1098f4` |
| `ldr` at 0x43114 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x43118 | store | 4 | `sp -0x54` | `[sp-88]` |
| `ldr` at 0x43120 | load | 4 | `abs 0x431bc` | `-` |
| `str` at 0x43124 | store | 4 | `sp -0x54` | `0x1098f0` |
| `ldr` at 0x43128 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x4312c | store | 4 | `sp -0x64` | `?` |
| `ldrb` at 0x43130 | load | 1 | `sp -0x28` | `-` |
| `ldr` at 0x43144 | load | 4 | `abs 0x431bc` | `-` |
| `str` at 0x43148 | store | 4 | `sp -0x5c` | `0x1098f0` |
| `ldr` at 0x43150 | load | 4 | `abs 0x431c0` | `-` |
| `str` at 0x43154 | store | 4 | `sp -0x5c` | `0x1098f4` |
| `ldr` at 0x43158 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x4315c | store | 4 | `sp -0x60` | `[sp-92]` |
| `ldr` at 0x43160 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x43164 | load | 4 | `abs 0x431d4` | `-` |
| `ldr` at 0x43168 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x4316c | load | 4 | `[sp-68] +0x8` | `-` |
| `ldr` at 0x43170 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x43178 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x43188 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x43190 | store | 4 | `sp -0x34` | `([sp-52] + ret(0x1b2bc))` |
| `ldr` at 0x43194 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x4319c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x431a4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x431a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x431a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x431a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42e54 | `(unknown callee) 0x42224` | `r0` | `sp-36` | `r2` | `sp-36` |
| `bl` at 0x42e74 | `(unknown callee) 0x1b1b4` | `0x50` | `?` | `?` | `ret(0x42224)` |
| `bl` at 0x42e8c | `(unknown callee) 0x1b598` | `[[sp-28]]` | `0x10982c` | `?` | `[sp-28]` |
| `bl` at 0x42eb0 | `(unknown callee) 0x1b1b4` | `?` | `?` | `ret(0x42224)` | `?` |
| `bl` at 0x42ecc | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x109844` | `?` | `[[sp-28]]` |
| `bl` at 0x42ed4 | `(unknown callee) 0x1b2bc` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x42f18 | `(unknown callee) 0x42820` | `[sp-24]` | `[sp-44]:1` | `sp-40` | `[sp-44]:1` |
| `bl` at 0x42f68 | `(unknown callee) 0x1b52c` | `[sp-52]` | `0x1098bc` | `?` | `?` |
| `bl` at 0x42f74 | `(unknown callee) 0x21704` | `ret(0x42820)` | `?` | `ret(0x42820)` | `0xfffffb4e` |
| `bl` at 0x42fa8 | `(unknown callee) 0x1b52c` | `[sp-52]` | `0x1098d0` | `?` | `ret(0x21704)` |
| `bl` at 0x43048 | `(unknown callee) 0x1b52c` | `[sp-52]` | `0x1098f8` | `[[sp-56]+8]` | `[sp-40]:1` |
| `bl` at 0x43174 | `(unknown callee) 0x1b52c` | `[sp-52]` | `0x10993c` | `[[sp-68]+8]` | `[sp-40]:1` |
| `bl` at 0x4317c | `(unknown callee) 0x1b2bc` | `[sp-52]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42e68 | `ret(0x42224) != 0x0` | `ret(0x42224) == 0x0` |
| `blt` at 0x42f00 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x42f2c | `ret(0x42820) == 0x0` | `ret(0x42820) != 0x0` |
| `bne` at 0x42f3c | `ret(0x42820) != 0xfffffb4e` | `ret(0x42820) == 0xfffffb4e` |
| `beq` at 0x42fd8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x43014 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x43088 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x430b8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x430e8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x430fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x43140 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x431a4 | `r0` | `?` |
