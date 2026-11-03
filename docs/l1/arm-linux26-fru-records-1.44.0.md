# ARM Linux 2.6 FRU record routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-fru-records-facts.txt) retain all tool sections.

## `L1FRU_SHUBCapture`

- `L1FRU_SHUBCapture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe74c4 to 0xe784c.
  Established. Word stores write zero and four at the address formed by [sp-40] plus 0x50 (`str` at 0xe7550, `str` at 0xe7774).
  The result of the call to unnamed 0x6e1b0 is stored at sp-0x40 (`bl` at 0xe7788, `str` at 0xe778c).
  A call to unnamed 0x2031c, an add of one and a word store produce ret(0x2031c)+1 at sp-0x44 (`bl` at 0xe7794, `add` at 0xe779c, `str` at 0xe77a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe74d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe74d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe74dc | load | 4 | `abs 0xe7844` | `-` |
| `ldr` at 0xe74e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe74f4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe74fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7518 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7520 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe7528 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe752c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7530 | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe7538 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7548 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7550 | store | 4 | `[sp-40] +0x50` | `0x0` |
| `str` at 0xe7558 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe755c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7560 | load | 4 | `[sp-40] +0x50` | `-` |
| `ldr` at 0xe756c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7570 | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe7574 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7584 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe7588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe758c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7594 | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe759c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe75a0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe75a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe75a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe75b0 | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe75b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe75bc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe75c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe75cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe75d0 | load | 4 | `[sp-40] +0x58` | `-` |
| `str` at 0xe75e0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe75e4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe75e8 | store | 4 | `(unknown)` | `[sp-48]` |
| `str` at 0xe75f0 | store | 4 | `(unknown)` | `sp-56` |
| `str` at 0xe75f8 | store | 4 | `(unknown)` | `sp-60` |
| `str` at 0xe7620 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7628 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe762c | load | 4 | `[sp-40] +0x58` | `-` |
| `ldr` at 0xe7644 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7660 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7664 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7670 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7674 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe767c | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7684 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe768c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7690 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7694 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe769c | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe76a4 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe76ac | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe76b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe76b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe76bc | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe76c4 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe76cc | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe76d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe76d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe76dc | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe76e4 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe76e8 | store | 1 | `(unknown)` | `([sp-56] & 0xff)` |
| `ldr` at 0xe76ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe76f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe76f8 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7700 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7708 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe770c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7718 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7720 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7728 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe772c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7730 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7738 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7740 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7748 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe774c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7750 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7758 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7760 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7764 | store | 1 | `(unknown)` | `([sp-60] & 0xff)` |
| `ldr` at 0xe776c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7774 | store | 4 | `[sp-40] +0x50` | `0x4` |
| `ldr` at 0xe7778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe778c | store | 4 | `sp -0x40` | `ret(0x6e1b0)` |
| `ldr` at 0xe7790 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe77a0 | store | 4 | `sp -0x44` | `ret(0x2031c)+1` |
| `ldr` at 0xe77a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe77cc | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe77d0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe77d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe77d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe77e4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe77e8 | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0xe77ec | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe77f0 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xe77f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe77f8 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xe77fc | load | 4 | `abs 0xe7848` | `-` |
| `ldr` at 0xe7804 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7808 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7810 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7818 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe7824 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7828 | load | 4 | `[sp-40] +0x50` | `-` |
| `ldm` at 0xe7840 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe7840 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe7840 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7524 | `(unknown callee) 0x134634` | `r0` | `?` | `?` | `?` |
| `bl` at 0xe7544 | `(unknown callee) 0x20550` | `[[sp-40]+100]` | `0x0` | `?` | `?` |
| `bl` at 0xe75d4 | `(unknown callee) 0x654f0` | `[[sp-40]+88]` | `[sp-40]` | `?` | `[sp-40]` |
| `bl` at 0xe760c | `(unknown callee) 0x86eac` | `ret(0x654f0)` | `?` | `?` | `?` |
| `bl` at 0xe7630 | `(unknown callee) 0x654f0` | `[[sp-40]+88]` | `[sp-40]` | `?` | `[sp-40]` |
| `bl` at 0xe764c | `(unknown callee) 0x13d1a0` | `ret(0x654f0)` | `[sp-48]` | `sp-56` | `sp-60` |
| `bl` at 0xe7788 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xe7794 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xe77dc | `(unknown callee) 0x203f4` | `[sp-72]` | `[sp-64]` | `[sp-68]` | `?` |
| `bl` at 0xe780c | `(unknown callee) 0x29908` | `0x1847cc` | `ret(0x203f4)` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xe7820 | `(unknown callee) 0x1346fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7568 | `[[sp-40]+80] != 0x0` | `[[sp-40]+80] == 0x0` |
| `ble` at 0xe757c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xe75c8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xe766c | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe7840 | `r0` | `?` |

## `L1FRU_NL4RInit`

- `L1FRU_NL4RInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe784c to 0xe7ca4.
  Established. The result of the call to unnamed 0x203f4 is stored as a word at sp-0x3c (`bl` at 0xe7940, `str` at 0xe7944).
  Word stores with unknown effective addresses write 0x189930 and 0xe7ca4 (`str` at 0xe7b00, `str` at 0xe7b4c).
  A load, add and store sequence writes [sp-40]+1 at sp-0x28 (`ldr` at 0xe7c5c, `add` at 0xe7c60, `str` at 0xe7c64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe785c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe7860 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe7864 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7868 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe787c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7884 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7888 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe788c | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe78a4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe78ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xe78bc | store | 4 | `sp -0x38` | `0x1` |
| `str` at 0xe78c8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe78cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe78d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe78dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe78f0 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0xe78f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe7904 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0xe7908 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7930 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe7934 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe7938 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe793c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7944 | store | 4 | `sp -0x3c` | `ret(0x203f4)` |
| `ldr` at 0xe7958 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe7960 | store | 4 | `sp -0x40` | `ret(0x3de44)` |
| `ldr` at 0xe7968 | load | 4 | `abs 0xe7c90` | `-` |
| `str` at 0xe796c | store | 4 | `sp -0x40` | `0x1847f8` |
| `ldr` at 0xe7970 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7988 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7990 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe79ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe79b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe79b8 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xe79c0 | load | 4 | `abs 0xe7c94` | `-` |
| `ldr` at 0xe79c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe79c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe79d0 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe79d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe79e8 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe79f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7a0c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7a18 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0xe7a1c | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7a20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7a34 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7a3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7a58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7a60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7a64 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xe7a68 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7a6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7a80 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7a88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7aa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7aac | load | 4 | `abs 0xe7c98` | `-` |
| `ldr` at 0xe7ab0 | load | 4 | `abs 0x1a04b0` | `-` |
| `str` at 0xe7ab4 | store | 4 | `(unknown)` | `[0x1a04b0]` |
| `ldr` at 0xe7ab8 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7abc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7ad0 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7ad8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7af4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7afc | load | 4 | `abs 0xe7c9c` | `-` |
| `str` at 0xe7b00 | store | 4 | `(unknown)` | `0x189930` |
| `ldr` at 0xe7b04 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7b08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7b1c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7b24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7b40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7b48 | load | 4 | `abs 0xe7ca0` | `-` |
| `str` at 0xe7b4c | store | 4 | `(unknown)` | `0xe7ca4` |
| `ldr` at 0xe7b50 | load | 4 | `abs 0xe7c98` | `-` |
| `ldr` at 0xe7b54 | load | 4 | `abs 0x1a04b0` | `-` |
| `str` at 0xe7b5c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe7b60 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7b64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7b78 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7b80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7b9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7ba4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe7bb0 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe7bb4 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7bb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7bcc | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7bd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7bf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe7bf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7c08 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xe7c10 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c28 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7c30 | load | 4 | `abs 0xe7c8c` | `-` |
| `ldr` at 0xe7c34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7c48 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7c50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7c58 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe7c5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7c64 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe7c68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe7c70 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe7c7c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xe7c80 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xe7c88 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe7c88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe7c88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe7c88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe78ec | `(unknown callee) 0x6e1b0` | `?` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xe78f8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xe7940 | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0xe7948 | `(unknown callee) 0x6e140` | `ret(0x203f4)` | `?` | `?` | `?` |
| `bl` at 0xe795c | `(unknown callee) 0x3de44` | `[sp-24]` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xe79cc | `(unknown callee) 0x205bc` | `?` | `0x1847fc` | `[sp-60]` | `[sp-64]` |
| `bl` at 0xe7ba8 | `(unknown callee) 0x201fc` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe78b4 | `? != 0x0` | `? == 0x0` |
| `bge` at 0xe78d8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xe7954 | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `bne` at 0xe7c00 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe7c88 | `r0` | `?` |

## `L1FRU_NL4RCapture`

- `L1FRU_NL4RCapture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7ca4 to 0xe7fc8.
  Established. Word stores of zero and four target the address formed by [sp-40] plus 0x50 (`str` at 0xe7d30, `str` at 0xe7ef0).
  Printed candidates for the call to unnamed 0x13d1a0 include r0=ret(0x2d8ec), r2=sp-56 and r3=sp-60 (`bl` at 0xe7dac, `bl` at 0xe7dc8).
  A call to unnamed 0x2031c is followed by an add of one and a word store of ret(0x2031c)+1 at sp-0x44 (`bl` at 0xe7f10, `add` at 0xe7f18, `str` at 0xe7f1c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7cb4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe7cb8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe7cbc | load | 4 | `abs 0xe7fc0` | `-` |
| `ldr` at 0xe7cc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7cd4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe7cdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7cf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7d00 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe7d08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7d0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7d10 | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe7d18 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7d28 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7d30 | store | 4 | `[sp-40] +0x50` | `0x0` |
| `str` at 0xe7d38 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe7d3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7d40 | load | 4 | `[sp-40] +0x50` | `-` |
| `ldr` at 0xe7d4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7d50 | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe7d54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7d64 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe7d68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7d6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7d74 | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe7d7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7d80 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe7d84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7d88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7d90 | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe7d98 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe7d9c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe7da0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7da4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe7da8 | load | 4 | `[sp-40] +0x58` | `-` |
| `ldr` at 0xe7dc0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7ddc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe7de0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe7dec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7df0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7df8 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7e00 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe7e08 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7e0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7e10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e18 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7e20 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe7e28 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7e2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7e30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e38 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7e40 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe7e48 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7e4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7e50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e58 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7e60 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe7e64 | store | 1 | `(unknown)` | `([sp-56] & 0xff)` |
| `ldr` at 0xe7e68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7e6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e74 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7e7c | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7e84 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7e88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7e8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7e94 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7e9c | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7ea4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7ea8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7eac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7eb4 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7ebc | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7ec4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe7ec8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7ecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7ed4 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe7edc | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe7ee0 | store | 1 | `(unknown)` | `([sp-60] & 0xff)` |
| `ldr` at 0xe7ee8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe7ef0 | store | 4 | `[sp-40] +0x50` | `0x4` |
| `ldr` at 0xe7ef4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe7f08 | store | 4 | `sp -0x40` | `ret(0x6e1b0)` |
| `ldr` at 0xe7f0c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe7f1c | store | 4 | `sp -0x44` | `ret(0x2031c)+1` |
| `ldr` at 0xe7f20 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe7f48 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe7f4c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe7f50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe7f54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe7f60 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe7f64 | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0xe7f68 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe7f6c | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xe7f70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe7f74 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xe7f78 | load | 4 | `abs 0xe7fc4` | `-` |
| `ldr` at 0xe7f80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7f84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe7f8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe7f94 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe7fa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe7fa4 | load | 4 | `[sp-40] +0x50` | `-` |
| `ldm` at 0xe7fbc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe7fbc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe7fbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe7d04 | `(unknown callee) 0x134634` | `r0` | `?` | `?` | `?` |
| `bl` at 0xe7d24 | `(unknown callee) 0x20550` | `[[sp-40]+100]` | `0x0` | `?` | `?` |
| `bl` at 0xe7dac | `(unknown callee) 0x2d8ec` | `[sp-20]` | `[[sp-40]+88]` | `?` | `[sp-40]` |
| `bl` at 0xe7dc8 | `(unknown callee) 0x13d1a0` | `ret(0x2d8ec)` | `[sp-48]` | `sp-56` | `sp-60` |
| `bl` at 0xe7f04 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xe7f10 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xe7f58 | `(unknown callee) 0x203f4` | `[sp-72]` | `[sp-64]` | `[sp-68]` | `?` |
| `bl` at 0xe7f88 | `(unknown callee) 0x29908` | `0x18480c` | `ret(0x203f4)` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xe7f9c | `(unknown callee) 0x1346fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe7d48 | `[[sp-40]+80] != 0x0` | `[[sp-40]+80] == 0x0` |
| `ble` at 0xe7d5c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xe7de8 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe7fbc | `r0` | `?` |

## `L1FRU_SHUB2Init`

- `L1FRU_SHUB2Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe7fc8 to 0xe8420.
  Established. Word stores of 0x18b380 and 0xe8420 have unknown effective addresses (`str` at 0xe827c, `str` at 0xe82c8).
  The call to unnamed 0x3de44 records its result at sp-0x40 (`bl` at 0xe80d8, `str` at 0xe80dc).
  The increment at sp-0x24 loads its word, adds one and stores [sp-36]+1 (`ldr` at 0xe83e4, `add` at 0xe83e8, `str` at 0xe83ec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe7fd8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe7fdc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe7fe0 | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe7fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe7ff8 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8000 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8004 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8008 | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe800c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8020 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8028 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8038 | store | 4 | `sp -0x38` | `0x1` |
| `str` at 0xe8044 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe8048 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe804c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8058 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe806c | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0xe8070 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe8080 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0xe8084 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe80ac | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe80b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe80b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe80b8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe80c0 | store | 4 | `sp -0x3c` | `ret(0x203f4)` |
| `ldr` at 0xe80d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe80dc | store | 4 | `sp -0x40` | `ret(0x3de44)` |
| `ldr` at 0xe80e4 | load | 4 | `abs 0xe840c` | `-` |
| `str` at 0xe80e8 | store | 4 | `sp -0x40` | `0x184838` |
| `ldr` at 0xe80ec | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe80f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8104 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe810c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8130 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8134 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xe813c | load | 4 | `abs 0xe8410` | `-` |
| `ldr` at 0xe8140 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe8144 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe814c | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe8150 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8164 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe816c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8188 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8194 | store | 4 | `(unknown)` | `0x4` |
| `ldr` at 0xe8198 | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe819c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe81b0 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe81b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe81d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe81dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe81e0 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xe81e4 | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe81e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe81fc | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8204 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8220 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8228 | load | 4 | `abs 0xe8414` | `-` |
| `ldr` at 0xe822c | load | 4 | `abs 0x1a04b8` | `-` |
| `str` at 0xe8230 | store | 4 | `(unknown)` | `[0x1a04b8]` |
| `ldr` at 0xe8234 | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe8238 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe824c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8254 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8270 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8278 | load | 4 | `abs 0xe8418` | `-` |
| `str` at 0xe827c | store | 4 | `(unknown)` | `0x18b380` |
| `ldr` at 0xe8280 | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe8284 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8298 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe82a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe82bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe82c4 | load | 4 | `abs 0xe841c` | `-` |
| `str` at 0xe82c8 | store | 4 | `(unknown)` | `0xe8420` |
| `ldr` at 0xe82cc | load | 4 | `abs 0xe8414` | `-` |
| `ldr` at 0xe82d0 | load | 4 | `abs 0x1a04b8` | `-` |
| `str` at 0xe82d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe82dc | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe82e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe82f4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe82fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8318 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8320 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe832c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe8330 | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe8334 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8348 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8350 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe836c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8374 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8384 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xe838c | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe8390 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe83a4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe83ac | load | 4 | `abs 0xe8408` | `-` |
| `ldr` at 0xe83b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe83c4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe83cc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe83d4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe83d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe83e0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe83e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe83ec | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe83f8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xe83fc | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xe8404 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe8404 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe8404 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe8404 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8068 | `(unknown callee) 0x6e1b0` | `?` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xe8074 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xe80bc | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0xe80c4 | `(unknown callee) 0x6e140` | `ret(0x203f4)` | `?` | `?` | `?` |
| `bl` at 0xe80d8 | `(unknown callee) 0x3de44` | `[sp-24]` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xe8148 | `(unknown callee) 0x205bc` | `?` | `0x18483c` | `[sp-60]` | `[sp-64]` |
| `bl` at 0xe8324 | `(unknown callee) 0x201fc` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8030 | `? != 0x0` | `? == 0x0` |
| `bge` at 0xe8054 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xe80d0 | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `bne` at 0xe837c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe8404 | `r0` | `?` |

## `L1FRU_SHUB2Capture`

- `L1FRU_SHUB2Capture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8420 to 0xe8744.
  Established. Byte stores with unknown effective addresses include values ([sp-56] & 0xff) and ([sp-60] & 0xff) (`strb` at 0xe85e0, `strb` at 0xe865c).
  A word store writes three at the address formed by [sp-40] plus 0x50 (`str` at 0xe866c).
  The call to unnamed 0x6e1b0 has its result stored at sp-0x40 (`bl` at 0xe8680, `str` at 0xe8684).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8430 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe8434 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe8438 | load | 4 | `abs 0xe873c` | `-` |
| `ldr` at 0xe843c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8450 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8458 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8474 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe847c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8484 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8488 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe848c | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe8494 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe84a4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe84ac | store | 4 | `[sp-40] +0x50` | `0x0` |
| `str` at 0xe84b4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe84b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe84bc | load | 4 | `[sp-40] +0x50` | `-` |
| `ldr` at 0xe84c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe84cc | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe84d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe84e0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe84e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe84e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe84f0 | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe84f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe84fc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe8500 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe850c | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe8514 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8518 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe851c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8520 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8524 | load | 4 | `[sp-40] +0x58` | `-` |
| `ldr` at 0xe853c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8558 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe855c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8568 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe856c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8574 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe857c | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe8584 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe858c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8594 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe859c | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe85a4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe85a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe85ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe85b4 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe85bc | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe85c4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe85c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe85cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe85d4 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe85dc | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe85e0 | store | 1 | `(unknown)` | `([sp-56] & 0xff)` |
| `ldr` at 0xe85e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe85e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe85f0 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe85f8 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe8600 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8604 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8610 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8618 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe8620 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8624 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8628 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8630 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8638 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe8640 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8644 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8648 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8650 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8658 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe865c | store | 1 | `(unknown)` | `([sp-60] & 0xff)` |
| `ldr` at 0xe8664 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe866c | store | 4 | `[sp-40] +0x50` | `0x3` |
| `ldr` at 0xe8670 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe8684 | store | 4 | `sp -0x40` | `ret(0x6e1b0)` |
| `ldr` at 0xe8688 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe8698 | store | 4 | `sp -0x44` | `ret(0x2031c)+1` |
| `ldr` at 0xe869c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe86c4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe86c8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe86cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe86d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe86dc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe86e0 | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0xe86e4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe86e8 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xe86ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe86f0 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xe86f4 | load | 4 | `abs 0xe8740` | `-` |
| `ldr` at 0xe86fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8700 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8708 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8710 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xe871c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8720 | load | 4 | `[sp-40] +0x50` | `-` |
| `ldm` at 0xe8738 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe8738 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe8738 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8480 | `(unknown callee) 0x134634` | `r0` | `?` | `?` | `?` |
| `bl` at 0xe84a0 | `(unknown callee) 0x20550` | `[[sp-40]+100]` | `0x0` | `?` | `?` |
| `bl` at 0xe8528 | `(unknown callee) 0x8ef38` | `[sp-20]` | `[[sp-40]+88]` | `?` | `[sp-40]` |
| `bl` at 0xe8544 | `(unknown callee) 0x13d1a0` | `ret(0x8ef38)` | `[sp-48]` | `sp-56` | `sp-60` |
| `bl` at 0xe8680 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xe868c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xe86d4 | `(unknown callee) 0x203f4` | `[sp-72]` | `[sp-64]` | `[sp-68]` | `?` |
| `bl` at 0xe8704 | `(unknown callee) 0x29908` | `0x18484c` | `ret(0x203f4)` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xe8718 | `(unknown callee) 0x1346fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe84c4 | `[[sp-40]+80] != 0x0` | `[[sp-40]+80] == 0x0` |
| `ble` at 0xe84d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xe8564 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe8738 | `r0` | `?` |

## `L1FRU_TIOInit`

- `L1FRU_TIOInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8744 to 0xe8b9c.
  Established. The call to unnamed 0x203f4 has its result stored as a word at sp-0x3c (`bl` at 0xe8838, `str` at 0xe883c).
  Two word stores with unknown effective addresses write 0x18a4e0 and 0xe8b9c (`str` at 0xe89f8, `str` at 0xe8a44).
  Load, add and store sequences write [sp-40]+1 at sp-0x28 and [sp-36]+1 at sp-0x24 (`ldr` at 0xe8b54, `add` at 0xe8b58, `str` at 0xe8b5c, `ldr` at 0xe8b60, `add` at 0xe8b64, `str` at 0xe8b68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8754 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe8758 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe875c | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8760 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8774 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe877c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8780 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8784 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe879c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe87a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe87b4 | store | 4 | `sp -0x38` | `0x1` |
| `str` at 0xe87c0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe87c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe87c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe87e8 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0xe87ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe87fc | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0xe8800 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8828 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe882c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8830 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8834 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe883c | store | 4 | `sp -0x3c` | `ret(0x203f4)` |
| `ldr` at 0xe8850 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe8858 | store | 4 | `sp -0x40` | `ret(0x3de44)` |
| `ldr` at 0xe8860 | load | 4 | `abs 0xe8b88` | `-` |
| `str` at 0xe8864 | store | 4 | `sp -0x40` | `0x184878` |
| `ldr` at 0xe8868 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe886c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8880 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8888 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe88a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe88ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe88b0 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xe88b8 | load | 4 | `abs 0xe8b8c` | `-` |
| `ldr` at 0xe88bc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe88c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe88c8 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe88cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe88e0 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe88e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8904 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8910 | store | 4 | `(unknown)` | `0x3` |
| `ldr` at 0xe8914 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8918 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe892c | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8934 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8950 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8958 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe895c | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xe8960 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8978 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8980 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe899c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe89a4 | load | 4 | `abs 0xe8b90` | `-` |
| `ldr` at 0xe89a8 | load | 4 | `abs 0x1a04b4` | `-` |
| `str` at 0xe89ac | store | 4 | `(unknown)` | `[0x1a04b4]` |
| `ldr` at 0xe89b0 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe89b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe89c8 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe89d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe89ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe89f4 | load | 4 | `abs 0xe8b94` | `-` |
| `str` at 0xe89f8 | store | 4 | `(unknown)` | `0x18a4e0` |
| `ldr` at 0xe89fc | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8a00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8a14 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8a1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8a38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8a40 | load | 4 | `abs 0xe8b98` | `-` |
| `str` at 0xe8a44 | store | 4 | `(unknown)` | `0xe8b9c` |
| `ldr` at 0xe8a48 | load | 4 | `abs 0xe8b90` | `-` |
| `ldr` at 0xe8a4c | load | 4 | `abs 0x1a04b4` | `-` |
| `str` at 0xe8a54 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe8a58 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8a5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8a70 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8a78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8a94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8a9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe8aa8 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xe8aac | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8ab0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8ac4 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8acc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8ae8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8af0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8b00 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xe8b08 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8b0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8b20 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8b28 | load | 4 | `abs 0xe8b84` | `-` |
| `ldr` at 0xe8b2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8b40 | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8b48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8b50 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8b54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe8b5c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe8b60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8b68 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe8b74 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xe8b78 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xe8b80 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe8b80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe8b80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe8b80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe87e4 | `(unknown callee) 0x6e1b0` | `?` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xe87f0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xe8838 | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0xe8840 | `(unknown callee) 0x6e140` | `ret(0x203f4)` | `?` | `?` | `?` |
| `bl` at 0xe8854 | `(unknown callee) 0x3de44` | `[sp-24]` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xe88c4 | `(unknown callee) 0x205bc` | `?` | `0x18487c` | `[sp-60]` | `[sp-64]` |
| `bl` at 0xe8aa0 | `(unknown callee) 0x201fc` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe87ac | `? != 0x0` | `? == 0x0` |
| `bge` at 0xe87d0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xe884c | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `bne` at 0xe8af8 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe8b80 | `r0` | `?` |

## `L1FRU_TIOCapture`

- `L1FRU_TIOCapture`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8b9c to 0xe8f50.
  Established. Word stores write zero and three at the address formed by [sp-40] plus 0x50 (`str` at 0xe8c28, `str` at 0xe8e78).
  The tool reports r0=ret(0x740c4) as a candidate for the call to unnamed 0x86eac (`bl` at 0xe8d18, `bl` at 0xe8d50).
  A call to unnamed 0x2031c, an add of one and a word store produce ret(0x2031c)+1 at sp-0x44 (`bl` at 0xe8e98, `add` at 0xe8ea0, `str` at 0xe8ea4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8bac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe8bb0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe8bb4 | load | 4 | `abs 0xe8f48` | `-` |
| `ldr` at 0xe8bb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8bcc | load | 4 | `abs 0x31b950` | `-` |
| `ldr` at 0xe8bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8bf0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8bf8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8c00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8c04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8c08 | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe8c10 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8c20 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe8c28 | store | 4 | `[sp-40] +0x50` | `0x0` |
| `str` at 0xe8c30 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe8c34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8c38 | load | 4 | `[sp-40] +0x50` | `-` |
| `ldr` at 0xe8c44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8c48 | load | 4 | `[sp-40] +0x5c` | `-` |
| `ldr` at 0xe8c4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8c5c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe8c60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8c64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8c6c | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe8c74 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8c78 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe8c7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8c80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8c88 | load | 4 | `[sp-40] +0x60` | `-` |
| `ldr` at 0xe8c90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8c94 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8c98 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe8c9c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe8ca0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe8cac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe8cb8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xe8cc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8cd0 | load | 4 | `[sp-40] +0x58` | `-` |
| `ldr` at 0xe8ce8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8d04 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe8d0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8d10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8d14 | load | 4 | `[sp-40] +0x58` | `-` |
| `str` at 0xe8d24 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe8d28 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8d2c | store | 4 | `(unknown)` | `[sp-48]` |
| `str` at 0xe8d34 | store | 4 | `(unknown)` | `sp-56` |
| `str` at 0xe8d3c | store | 4 | `(unknown)` | `sp-60` |
| `str` at 0xe8d64 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe8d68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8d74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8d78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8d80 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8d88 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe8d90 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8d94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8d98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8da0 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8da8 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe8db0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8db4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8db8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8dc0 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8dc8 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe8dd0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8dd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8dd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8de0 | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8de8 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xe8dec | store | 1 | `(unknown)` | `([sp-56] & 0xff)` |
| `ldr` at 0xe8df0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8df4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8dfc | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8e04 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe8e0c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8e10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8e14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8e1c | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8e24 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe8e2c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8e30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8e34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8e3c | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8e44 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe8e4c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8e50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8e54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8e5c | load | 4 | `[sp-40] +0x64` | `-` |
| `ldr` at 0xe8e64 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xe8e68 | store | 1 | `(unknown)` | `([sp-60] & 0xff)` |
| `ldr` at 0xe8e70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe8e78 | store | 4 | `[sp-40] +0x50` | `0x3` |
| `ldr` at 0xe8e7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe8e90 | store | 4 | `sp -0x40` | `ret(0x6e1b0)` |
| `ldr` at 0xe8e94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe8ea4 | store | 4 | `sp -0x44` | `ret(0x2031c)+1` |
| `ldr` at 0xe8ea8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe8ed0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe8ed4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe8ed8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe8edc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe8ee8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8eec | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0xe8ef0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe8ef4 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xe8ef8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe8efc | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xe8f00 | load | 4 | `abs 0xe8f4c` | `-` |
| `ldr` at 0xe8f08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8f0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8f14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8f1c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe8f28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8f2c | load | 4 | `[sp-40] +0x50` | `-` |
| `ldm` at 0xe8f44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe8f44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe8f44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8bfc | `(unknown callee) 0x134634` | `r0` | `?` | `?` | `?` |
| `bl` at 0xe8c1c | `(unknown callee) 0x20550` | `[[sp-40]+100]` | `0x0` | `?` | `?` |
| `bl` at 0xe8cd4 | `(unknown callee) 0x740c4` | `[sp-20]` | `[[sp-40]+88]` | `?` | `[sp-40]` |
| `bl` at 0xe8cf0 | `(unknown callee) 0x13d1a0` | `ret(0x740c4)` | `[sp-48]` | `sp-56` | `sp-60` |
| `bl` at 0xe8d18 | `(unknown callee) 0x740c4` | `[sp-20]` | `[[sp-40]+88]` | `?` | `[sp-40]` |
| `bl` at 0xe8d50 | `(unknown callee) 0x86eac` | `ret(0x740c4)` | `?` | `?` | `?` |
| `bl` at 0xe8e8c | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xe8e98 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xe8ee0 | `(unknown callee) 0x203f4` | `[sp-72]` | `[sp-64]` | `[sp-68]` | `?` |
| `bl` at 0xe8f10 | `(unknown callee) 0x29908` | `0x184888` | `ret(0x203f4)` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xe8f24 | `(unknown callee) 0x1346fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8c40 | `[[sp-40]+80] != 0x0` | `[[sp-40]+80] == 0x0` |
| `ble` at 0xe8c54 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xe8ca8 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xe8cb4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0xe8cc0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xe8d70 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe8f44 | `r0` | `?` |

## `PBaySdrGetFruHdr`

- `PBaySdrGetFruHdr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb7f4 to 0xeb840.
  Established. The call to unnamed 0x201fc has its result stored as a word at sp-0x14 (`bl` at 0xeb808, `str` at 0xeb810).
  Printed candidates for the call to unnamed 0x20634 include r0=ret(0x201fc) and r1=0x184c0c (`bl` at 0xeb808, `bl` at 0xeb81c).
  The return record is [sp-20], loaded before the return (`ldr` at 0xeb82c, `ldm` at 0xeb834).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb810 | store | 4 | `sp -0x14` | `ret(0x201fc)` |
| `ldr` at 0xeb814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb818 | load | 4 | `abs 0xeb838` | `-` |
| `ldr` at 0xeb820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb824 | load | 4 | `abs 0xeb83c` | `-` |
| `ldr` at 0xeb82c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeb834 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeb834 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeb834 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeb834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb808 | `(unknown callee) 0x201fc` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xeb81c | `(unknown callee) 0x20634` | `ret(0x201fc)` | `0x184c0c` | `?` | `ret(0x201fc)` |
| `bl` at 0xeb828 | `(unknown callee) 0x203c4` | `[sp-20]` | `0x184c50` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeb834 | `r0` | `[sp-20]` |

## `PBaySdrGetFru`

- `PBaySdrGetFru`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb840 to 0xec2cc.
  Established. The call to unnamed 0x201fc has its result stored as a word at sp-0x28 and at the address held in [sp-32] (`bl` at 0xeb9b8, `str` at 0xeb9c0, `str` at 0xeb9cc).
  A halfword load, add of three and halfword store write ([sp-722]:2+3 & 0xffff) at sp-0x2d2 (`ldrh` at 0xebc0c, `add` at 0xebc10, `strh` at 0xebc18).
  Byte stores write 0x20 at [sp-36] and zero at [sp-36]+1 (`strb` at 0xebe0c, `strb` at 0xebe20).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb850 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb854 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb858 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeb85c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeb860 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeb868 | store | 4 | `r3 +0x0` | `0x0` |
| `ldr` at 0xeb86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb878 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeb88c | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xeb894 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb8a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb8a8 | load | 4 | `abs 0xec284` | `-` |
| `str` at 0xeb8b0 | store | 4 | `sp -0x130` | `?` |
| `ldr` at 0xeb8b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb8c4 | load | 4 | `abs 0xec288` | `-` |
| `str` at 0xeb8c8 | store | 4 | `sp -0x130` | `0x1a0464` |
| `ldr` at 0xeb8d0 | load | 4 | `abs 0xec284` | `-` |
| `str` at 0xeb8d4 | store | 4 | `sp -0x130` | `0x1a0448` |
| `ldr` at 0xeb8d8 | load | 4 | `sp -0x130` | `-` |
| `ldrb` at 0xeb8dc | load | 1 | `[sp-304] +0x0` | `-` |
| `str` at 0xeb8ec | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xeb8f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb900 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb908 | load | 4 | `abs 0xec28c` | `-` |
| `str` at 0xeb910 | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0xeb914 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0xeb918 | load | 1 | `[sp-740] +0x0` | `-` |
| `str` at 0xeb928 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xeb934 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0xeb938 | load | 1 | `[sp-740] +0x0` | `-` |
| `ldr` at 0xeb93c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb950 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb960 | store | 4 | `sp -0x2cc` | `ret(0xea010)` |
| `ldr` at 0xeb964 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xeb970 | load | 4 | `sp -0x140` | `-` |
| `str` at 0xeb978 | store | 4 | `sp -0x2e8` | `ret(0xedbdc)` |
| `ldr` at 0xeb980 | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0xeb984 | load | 1 | `[sp-744] +0x2` | `-` |
| `strb` at 0xeb98c | store | 1 | `(unknown)` | `([[sp-744]+2]:1 & 0xff)` |
| `ldrb` at 0xeb998 | load | 1 | `(unknown)` | `-` |
| `str` at 0xeb9ac | store | 4 | `sp -0x300` | `0x0` |
| `str` at 0xeb9c0 | store | 4 | `sp -0x28` | `ret(0x201fc)` |
| `ldr` at 0xeb9c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb9c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeb9cc | store | 4 | `[sp-32] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xeb9d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb9dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb9e0 | load | 4 | `abs 0xec290` | `-` |
| `ldr` at 0xeb9ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeb9f4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xeb9fc | store | 1 | `[sp-40] +0x1` | `([sp-24]+48 & 0xff)` |
| `ldr` at 0xeba04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeba10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeba14 | load | 4 | `abs 0xec294` | `-` |
| `ldr` at 0xeba20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeba24 | load | 4 | `abs 0xec298` | `-` |
| `ldr` at 0xeba2c | load | 4 | `abs 0xec29c` | `-` |
| `strh` at 0xeba38 | store | 2 | `sp -0x2d2` | `0x0` |
| `ldr` at 0xeba3c | load | 4 | `abs 0xec2a0` | `-` |
| `strb` at 0xeba48 | store | 1 | `sp -0x2d3` | `0x8` |
| `ldr` at 0xeba50 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xeba58 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xeba5c | load | 4 | `abs 0xec2a0` | `-` |
| `ldrb` at 0xeba64 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xeba68 | store | 4 | `sp -0x308` | `0x8` |
| `ldr` at 0xeba6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeba74 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xeba84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeba94 | store | 4 | `sp -0x2cc` | `ret(0xea010)` |
| `ldr` at 0xeba98 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xebaa4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebaa8 | load | 4 | `abs 0xec2a4` | `-` |
| `str` at 0xebab4 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xebabc | load | 4 | `sp -0x140` | `-` |
| `str` at 0xebac4 | store | 4 | `sp -0x2ec` | `ret(0xedbdc)` |
| `ldr` at 0xebacc | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xebae4 | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebaf0 | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0xebafc | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebb08 | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0xebb14 | load | 4 | `abs 0xec29c` | `-` |
| `ldr` at 0xebb18 | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebb24 | load | 1 | `sp -0x2f0` | `-` |
| `strh` at 0xebb30 | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0xebb34 | load | 4 | `abs 0xec2a0` | `-` |
| `strb` at 0xebb40 | store | 1 | `sp -0x2d3` | `0x3` |
| `ldr` at 0xebb48 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xebb50 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xebb54 | load | 4 | `abs 0xec2a0` | `-` |
| `ldrb` at 0xebb5c | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xebb60 | store | 4 | `sp -0x308` | `0x3` |
| `str` at 0xebb68 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xebb6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebb74 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xebb84 | load | 4 | `abs 0xec2ac` | `-` |
| `strb` at 0xebb8c | store | 1 | `sp -0x2d4` | `(ret(0xecf80) & 0xff)` |
| `ldr` at 0xebb90 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xebb98 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xebba4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebba8 | load | 4 | `abs 0xec2a4` | `-` |
| `str` at 0xebbb4 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xebbbc | load | 4 | `abs 0xec2a0` | `-` |
| `ldr` at 0xebbc0 | load | 4 | `abs 0xec2b0` | `-` |
| `ldrb` at 0xebbc8 | load | 1 | `sp -0x127` | `-` |
| `strb` at 0xebbd4 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0xebbd8 | load | 4 | `abs 0xec2a0` | `-` |
| `ldr` at 0xebbdc | load | 4 | `abs 0xec2a0` | `-` |
| `ldr` at 0xebbe0 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xebbe8 | load | 1 | `sp -0x2d3` | `-` |
| `ldrb` at 0xebbf0 | load | 1 | `sp -0x2d4` | `-` |
| `strb` at 0xebbfc | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0xebc00 | load | 4 | `abs 0xec29c` | `-` |
| `ldr` at 0xebc04 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xebc0c | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xebc18 | store | 2 | `sp -0x2d2` | `([sp-722]:2+3 & 0xffff)` |
| `ldr` at 0xebc20 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xebc28 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xebc2c | load | 4 | `abs 0xec2a0` | `-` |
| `ldrb` at 0xebc34 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xebc38 | store | 4 | `sp -0x308` | `[sp-723]:1` |
| `str` at 0xebc40 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xebc44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebc4c | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xebc5c | load | 4 | `abs 0xec2ac` | `-` |
| `strb` at 0xebc64 | store | 1 | `sp -0x2d4` | `(ret(0xecf80) & 0xff)` |
| `ldr` at 0xebc68 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xebc70 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xebc7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebc80 | load | 4 | `abs 0xec2a4` | `-` |
| `str` at 0xebc8c | store | 4 | `sp -0x300` | `?` |
| `str` at 0xebc98 | store | 4 | `sp -0x12c` | `0x0` |
| `str` at 0xebca0 | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0xebca4 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0xebcb0 | load | 4 | `abs 0xec2b4` | `-` |
| `ldr` at 0xebcb4 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xebcc4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebcd8 | load | 4 | `abs 0xec2b4` | `-` |
| `ldr` at 0xebcdc | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xebcec | load | 1 | `(sp-16 + [sp-300]) -0x118` | `-` |
| `strb` at 0xebcf8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xebcfc | load | 4 | `sp -0x12c` | `-` |
| `str` at 0xebd04 | store | 4 | `sp -0x12c` | `[sp-300]+1` |
| `ldr` at 0xebd08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebd18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xebd20 | store | 4 | `sp -0x24` | `(ret(0x2031c) + [sp-40])` |
| `ldr` at 0xebd28 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xebd38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebd3c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xebd54 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebd58 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0xebd60 | store | 4 | `sp -0x12c` | `?` |
| `ldrb` at 0xebd6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebd70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xebd78 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xebd7c | load | 4 | `abs 0xec2b8` | `-` |
| `ldr` at 0xebd80 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xebd8c | load | 1 | `[sp-720] +0x1a0498` | `-` |
| `ldrb` at 0xebd94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebda0 | load | 4 | `abs 0xec2b8` | `-` |
| `ldr` at 0xebda4 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xebdac | load | 1 | `[sp-720] +0x1a0498` | `-` |
| `ldrb` at 0xebdb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebdc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebdd0 | load | 4 | `abs 0xec2b8` | `-` |
| `ldr` at 0xebdd4 | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xebddc | load | 1 | `[sp-720] +0x1a0498` | `-` |
| `ldrb` at 0xebde8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebdf0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xebdf8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xebe00 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xebe0c | store | 1 | `[sp-36] +0x0` | `0x20` |
| `str` at 0xebe14 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xebe18 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xebe20 | store | 1 | `[sp-36] +0x1` | `0x0` |
| `ldr` at 0xebe24 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0xebe2c | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0xebe34 | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebe40 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0xebe4c | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebe58 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0xebe64 | load | 4 | `abs 0xec29c` | `-` |
| `ldr` at 0xebe68 | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebe74 | load | 1 | `sp -0x2f1` | `-` |
| `strh` at 0xebe80 | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0xebe84 | load | 4 | `abs 0xec2a0` | `-` |
| `strb` at 0xebe90 | store | 1 | `sp -0x2d3` | `0x6` |
| `ldr` at 0xebe98 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xebea0 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xebea4 | load | 4 | `abs 0xec2a0` | `-` |
| `ldrb` at 0xebeac | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xebeb0 | store | 4 | `sp -0x308` | `0x6` |
| `str` at 0xebeb8 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xebebc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebec4 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xebed4 | load | 4 | `abs 0xec2ac` | `-` |
| `strb` at 0xebedc | store | 1 | `sp -0x2d4` | `(ret(0xecf80) & 0xff)` |
| `ldr` at 0xebee0 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xebee8 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xebef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebef8 | load | 4 | `abs 0xec2a4` | `-` |
| `str` at 0xebf04 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xebf0c | load | 4 | `abs 0xec2bc` | `-` |
| `ldrb` at 0xebf14 | load | 1 | `sp -0x123` | `-` |
| `str` at 0xebf18 | store | 4 | `sp -0x2dc` | `[sp-291]:1` |
| `ldr` at 0xebf1c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xebf24 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xebf28 | load | 4 | `abs 0xec2c0` | `-` |
| `ldrb` at 0xebf30 | load | 1 | `sp -0x124` | `-` |
| `ldr` at 0xebf34 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xebf3c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xebf40 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xebf48 | store | 4 | `sp -0x2dc` | `?` |
| `ldrb` at 0xebf54 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebf58 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xebf60 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xebf64 | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xebf70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebf7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebf88 | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebf94 | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0xebfa0 | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebfac | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0xebfb8 | load | 4 | `abs 0xec29c` | `-` |
| `ldr` at 0xebfbc | load | 4 | `abs 0xec2a8` | `-` |
| `ldrb` at 0xebfc8 | load | 1 | `sp -0x2ef` | `-` |
| `strh` at 0xebfd4 | store | 2 | `sp -0x2d2` | `?` |
| `str` at 0xebfdc | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0xebfe4 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xebfec | load | 2 | `sp -0x2d2` | `-` |
| `str` at 0xebff4 | store | 4 | `sp -0x308` | `0x8` |
| `str` at 0xebffc | store | 4 | `sp -0x304` | `sp-764` |
| `ldr` at 0xec000 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec008 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xec018 | load | 4 | `abs 0xec2ac` | `-` |
| `strb` at 0xec020 | store | 1 | `sp -0x2d4` | `(ret(0xecf80) & 0xff)` |
| `ldr` at 0xec024 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xec02c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec038 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec03c | load | 4 | `abs 0xec2a4` | `-` |
| `str` at 0xec048 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xec070 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec074 | load | 4 | `abs 0xec2c4` | `-` |
| `str` at 0xec080 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xec088 | load | 4 | `abs 0xec29c` | `-` |
| `ldr` at 0xec08c | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xec094 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec09c | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xec0a4 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xec0b0 | store | 2 | `sp -0x2d2` | `(([sp-724]:1 + [sp-722]:2) & 0xffff)` |
| `ldr` at 0xec0b8 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xec0c0 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xec0c4 | load | 4 | `abs 0xec2c8` | `-` |
| `ldrb` at 0xec0d0 | load | 1 | `sp -0x2fa` | `-` |
| `str` at 0xec0d4 | store | 4 | `sp -0x308` | `[sp-762]:1` |
| `str` at 0xec0dc | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xec0e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec0e8 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xec0f8 | load | 4 | `abs 0xec2ac` | `-` |
| `strb` at 0xec100 | store | 1 | `sp -0x2d4` | `(ret(0xecf80) & 0xff)` |
| `ldr` at 0xec104 | load | 4 | `abs 0xec2c8` | `-` |
| `ldr` at 0xec110 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xec114 | load | 1 | `sp -0x2fa` | `-` |
| `ldrb` at 0xec11c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec128 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec12c | load | 4 | `abs 0xec2a4` | `-` |
| `str` at 0xec138 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xec140 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xec148 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec150 | load | 4 | `abs 0xec2b4` | `-` |
| `ldr` at 0xec160 | load | 4 | `abs 0xec2c8` | `-` |
| `ldrb` at 0xec16c | load | 1 | `sp -0x2f9` | `-` |
| `strb` at 0xec170 | store | 1 | `(unknown)` | `([sp-761]:1 & 0xff)` |
| `ldr` at 0xec178 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xec180 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec1a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec1a8 | load | 4 | `abs 0xec2c4` | `-` |
| `str` at 0xec1b4 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xec1bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec1cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xec1d4 | store | 4 | `sp -0x24` | `(ret(0x2031c) + [sp-40])` |
| `ldr` at 0xec1d8 | load | 4 | `abs 0xec2c8` | `-` |
| `ldrb` at 0xec1e4 | load | 1 | `sp -0x2fc` | `-` |
| `ldr` at 0xec1e8 | load | 4 | `abs 0xec2c8` | `-` |
| `ldrb` at 0xec1f4 | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0xec204 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec20c | load | 4 | `abs 0xec29c` | `-` |
| `ldr` at 0xec210 | load | 4 | `abs 0xec2ac` | `-` |
| `ldrb` at 0xec218 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec220 | load | 4 | `abs 0xec29c` | `-` |
| `ldrh` at 0xec228 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xec234 | store | 2 | `sp -0x2d2` | `(([sp-724]:1 + [sp-722]:2) & 0xffff)` |
| `ldr` at 0xec238 | load | 4 | `abs 0xec2c8` | `-` |
| `ldrsb` at 0xec244 | load | 1 | `sp -0x2fb` | `-` |
| `ldr` at 0xec254 | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0xec260 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `str` at 0xec274 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xec278 | load | 4 | `sp -0x300` | `-` |
| `ldm` at 0xec280 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xec280 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xec280 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb948 | `(unknown callee) 0xecd34` | `[sp-20]` | `sp-712` | `[[sp-740]]:1` | `[[sp-740]]:1` |
| `bl` at 0xeb958 | `(unknown callee) 0xea010` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0xeb974 | `(unknown callee) 0xedbdc` | `[sp-320]` | `?` | `?` | `ret(0xea010)` |
| `bl` at 0xeb9b8 | `(unknown callee) 0x201fc` | `0x400` | `?` | `?` | `?` |
| `bl` at 0xeb9e4 | `(unknown callee) 0x20634` | `[sp-40]` | `0x184c94` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xeba18 | `(unknown callee) 0x20634` | `[sp-40]` | `0x184c9c` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xeba28 | `(unknown callee) 0x20634` | `[sp-40]` | `0x184ca4` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xeba7c | `(unknown callee) 0xece5c` | `[sp-20]` | `sp-712` | `[sp-304]` | `0x0` |
| `bl` at 0xeba8c | `(unknown callee) 0xea010` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0xebaac | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `?` | `ret(0xea010)` |
| `bl` at 0xebac0 | `(unknown callee) 0xedbdc` | `[sp-320]` | `?` | `?` | `ret(0xea010)` |
| `bl` at 0xebae0 | `(unknown callee) 0x203f4` | `sp-756` | `ret(0xedbdc)+1` | `0x8` | `ret(0xedbdc)+1` |
| `bl` at 0xebb7c | `(unknown callee) 0xecf80` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xebbac | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `ret(0xecf80)` | `(ret(0xecf80) & 0xff)` |
| `bl` at 0xebc54 | `(unknown callee) 0xecf80` | `[sp-20]` | `sp-712` | `[sp-304]` | `([sp-722]:2+3 & 0xffff)` |
| `bl` at 0xebc84 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `sp-16` | `(ret(0xecf80) & 0xff)` |
| `bl` at 0xebd0c | `(unknown callee) 0x2031c` | `[sp-40]` | `?` | `sp-16` | `[sp-300]+1` |
| `bl` at 0xebd48 | `(unknown callee) 0x203f4` | `[sp-36]` | `(sp-296 + [sp-300])` | `?` | `?` |
| `bl` at 0xebdcc | `(unknown callee) 0x20550` | `[sp-36]` | `0x20` | `?` | `?` |
| `bl` at 0xebecc | `(unknown callee) 0xecf80` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xebefc | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `ret(0xecf80)` | `(ret(0xecf80) & 0xff)` |
| `bl` at 0xebf68 | `(unknown callee) 0xed4c0` | `[sp-732]` | `?` | `?` | `?` |
| `bl` at 0xebf78 | `(unknown callee) 0x203c4` | `[sp-40]` | `ret(0xed4c0)` | `?` | `ret(0xed4c0)` |
| `bl` at 0xec010 | `(unknown callee) 0xecf80` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xec040 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `ret(0xecf80)` | `(ret(0xecf80) & 0xff)` |
| `bl` at 0xec05c | `(unknown callee) 0xed51c` | `sp-764` | `0x8` | `ret(0xecf80)` | `sp-764` |
| `bl` at 0xec078 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184cac` | `?` | `?` |
| `bl` at 0xec0f0 | `(unknown callee) 0xecf80` | `[sp-20]` | `sp-712` | `[sp-304]` | `(([sp-724]:1 + [sp-722]:2) & 0xffff)` |
| `bl` at 0xec130 | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184b38` | `0xfffffd3c` | `(ret(0xecf80) & 0xff)` |
| `bl` at 0xec190 | `(unknown callee) 0xed51c` | `sp-296` | `[sp-724]:1+1` | `sp-296` | `[sp-724]:1+1` |
| `bl` at 0xec1ac | `(unknown callee) 0x203c4` | `[sp-40]` | `0x184cac` | `?` | `?` |
| `bl` at 0xec1c0 | `(unknown callee) 0x2031c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xec208 | `(unknown callee) 0xf92a0` | `[sp-764]:1` | `[sp-762]:1` | `sp-296` | `(ret(0x2031c) + [sp-40])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xeb874 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0xeb880 | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `ble` at 0xeb89c | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0xeb8c0 | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xeb8e4 | `[[sp-304]]:1 != 0xff` | `[[sp-304]]:1 == 0xff` |
| `ble` at 0xeb8fc | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bne` at 0xeb920 | `[[sp-740]]:1 != 0xff` | `[[sp-740]]:1 == 0xff` |
| `beq` at 0xeb96c | `ret(0xea010) == 0x0` | `ret(0xea010) != 0x0` |
| `beq` at 0xeb9a4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xeb9d8 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0xeba0c | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xebaa0 | `ret(0xea010) != 0x0` | `ret(0xea010) == 0x0` |
| `beq` at 0xebaf8 | `[sp-752]:1 == 0x0` | `[sp-752]:1 != 0x0` |
| `beq` at 0xebb10 | `[sp-752]:1 == 0xff` | `[sp-752]:1 != 0xff` |
| `bne` at 0xebba0 | `(ret(0xecf80) & 0xff) != 0x0` | `(ret(0xecf80) & 0xff) == 0x0` |
| `bne` at 0xebc78 | `(ret(0xecf80) & 0xff) != 0x0` | `(ret(0xecf80) & 0xff) == 0x0` |
| `bgt` at 0xebcac | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xebccc | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xebd9c | `[[sp-720]+1705112]:1 <= ? (unsigned, 32-bit)` | `[[sp-720]+1705112]:1 > ? (unsigned, 32-bit)` |
| `beq` at 0xebe48 | `[sp-753]:1 == 0x0` | `[sp-753]:1 != 0x0` |
| `beq` at 0xebe60 | `[sp-753]:1 == 0xff` | `[sp-753]:1 != 0xff` |
| `bne` at 0xebef0 | `(ret(0xecf80) & 0xff) != 0x0` | `(ret(0xecf80) & 0xff) == 0x0` |
| `beq` at 0xebf84 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0xebf9c | `[sp-751]:1 == 0x0` | `[sp-751]:1 != 0x0` |
| `beq` at 0xebfb4 | `[sp-751]:1 == 0xff` | `[sp-751]:1 != 0xff` |
| `beq` at 0xec034 | `(ret(0xecf80) & 0xff) == 0x8` | `(ret(0xecf80) & 0xff) != 0x8` |
| `beq` at 0xec06c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xec124 | `[sp-762]:1 == (ret(0xecf80) & 0xff)` | `[sp-762]:1 != (ret(0xecf80) & 0xff)` |
| `beq` at 0xec1a0 | `? == 0x0` | `? != 0x0` |
| `blt` at 0xec24c | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `bgt` at 0xec268 | `[sp-720] > 0x4 (signed, 32-bit)` | `[sp-720] <= 0x4 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xec280 | `r0` | `?` |

## `SdrSetChassisFru`

- `SdrSetChassisFru`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed590 to 0xed5e4.
  Established. The low byte of entry r0 is saved at sp-0x11 (`strb` at 0xed5a4).
  The branch takes 0xed5dc when the byte at absolute 0x1a0448 is not equal to 0xff, and falls through when it equals 0xff (`ldrb` at 0xed5ac, `bne` at 0xed5b4).
  Byte stores write ((r0 & 0xff) & 0xff) at absolute 0x1a0448 and zero at absolute 0x1a0449 and 0x1a044a (`strb` at 0xed5c0, `strb` at 0xed5cc, `strb` at 0xed5d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xed5a4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xed5a8 | load | 4 | `abs 0xed5e0` | `-` |
| `ldrb` at 0xed5ac | load | 1 | `abs 0x1a0448` | `-` |
| `ldr` at 0xed5b8 | load | 4 | `abs 0xed5e0` | `-` |
| `ldrb` at 0xed5bc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xed5c0 | store | 1 | `abs 0x1a0448` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xed5c4 | load | 4 | `abs 0xed5e0` | `-` |
| `strb` at 0xed5cc | store | 1 | `abs 0x1a0449` | `0x0` |
| `ldr` at 0xed5d0 | load | 4 | `abs 0xed5e0` | `-` |
| `strb` at 0xed5d8 | store | 1 | `abs 0x1a044a` | `0x0` |
| `ldm` at 0xed5dc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xed5dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xed5dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xed5dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xed5b4 | `[0x1a0448]:1 != 0xff` | `[0x1a0448]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xed5dc | `r0` | `r0` |

## `FRUInit`

- `FRUInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10325c to 0x10326c.
  Established. The span begins with mov ip, sp and ends with ldm (`mov` at 0x10325c, `ldm` at 0x103268);
  its intervening instructions are push and sub fp, ip, #4 (`push` at 0x103260, `sub` at 0x103264).
  The return record retains entry r0 (`ldm` at 0x103268).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x103268 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x103268 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x103268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x103268 | `r0` | `r0` |

## `FRUPackData`

- `FRUPackData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1032a0 to 0x104a4c.
  Established. A word store writes one at the address held in [sp+44] (`str` at 0x1032c8).
  The result from unnamed 0x201fc is stored as a word at the address held in [sp+40] (`bl` at 0x103808, `str` at 0x103810).
  A word load supplies [sp-44] for a byte store of nine (`ldr` at 0x103834, `strb` at 0x103840).
  An add and word store write [sp-44]+1 at sp-0x2c (`ldr` at 0x103834, `add` at 0x103844, `str` at 0x103848).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1032b0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x1032b4 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x1032b8 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x1032bc | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x1032c0 | load | 4 | `sp +0x2c` | `-` |
| `str` at 0x1032c8 | store | 4 | `[sp+44] +0x0` | `0x1` |
| `ldr` at 0x1032cc | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1032d0 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1032d4 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1032dc | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x1032e0 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1032e4 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1032e8 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1032f0 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x1032f4 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1032f8 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1032fc | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103304 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103308 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10330c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103310 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103318 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x10331c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103320 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103324 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x10332c | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103330 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103334 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103338 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103340 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103344 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103348 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10334c | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103354 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `str` at 0x10335c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x103360 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103364 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x103370 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103374 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103378 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103380 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+16` |
| `ldr` at 0x103384 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103388 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10338c | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103394 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `str` at 0x10339c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x1033a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1033b4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1033bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1033c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1033cc | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1033d0 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1033d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1033e8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1033f0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x10340c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103424 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103430 | store | 4 | `[sp+44] +0x0` | `(ret(0x2031c) + [[sp+44]])+1` |
| `ldr` at 0x103434 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103438 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10343c | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103444 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103448 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10344c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103450 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103458 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x10345c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103460 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103464 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x10346c | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103470 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103474 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103478 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103480 | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103484 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103488 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10348c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1034a0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1034a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1034c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1034cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1034d4 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1034dc | store | 4 | `[sp+44] +0x0` | `?` |
| `ldr` at 0x1034e0 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1034e4 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1034e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1034fc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103504 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103520 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103528 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103530 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103538 | store | 4 | `[sp+44] +0x0` | `?` |
| `ldr` at 0x10353c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x103544 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x10354c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x103554 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x10355c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103560 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103564 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x10356c | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103570 | load | 4 | `sp +0x2c` | `-` |
| `str` at 0x103574 | store | 4 | `sp -0x48` | `[sp+44]` |
| `ldr` at 0x103578 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10357c | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103580 | store | 4 | `sp -0x4c` | `[[sp+44]]` |
| `ldr` at 0x103584 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x103590 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x10359c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x1035a4 | store | 4 | `sp -0x50` | `([sp-76] + ret(0x2031c))` |
| `ldr` at 0x1035a8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x1035b0 | store | 4 | `sp -0x50` | `([sp-76] + ret(0x2031c))+1` |
| `ldr` at 0x1035b8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x1035bc | store | 4 | `sp -0x50` | `[[sp+44]]` |
| `ldr` at 0x1035c0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x1035c4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x1035c8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1035cc | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1035d0 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1035d4 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1035dc | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x1035e0 | load | 4 | `sp +0x2c` | `-` |
| `str` at 0x1035e4 | store | 4 | `sp -0x54` | `[sp+44]` |
| `ldr` at 0x1035e8 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1035ec | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1035f0 | store | 4 | `sp -0x58` | `[[sp+44]]` |
| `ldr` at 0x1035f4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x103600 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x10360c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x103614 | store | 4 | `sp -0x5c` | `([sp-88] + ret(0x2031c))` |
| `ldr` at 0x103618 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x103620 | store | 4 | `sp -0x5c` | `([sp-88] + ret(0x2031c))+1` |
| `ldr` at 0x103628 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x10362c | store | 4 | `sp -0x5c` | `[[sp+44]]` |
| `ldr` at 0x103630 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x103634 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x103638 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x10363c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103640 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103644 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x10364c | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103650 | load | 4 | `sp +0x2c` | `-` |
| `str` at 0x103654 | store | 4 | `sp -0x60` | `[sp+44]` |
| `ldr` at 0x103658 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10365c | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103660 | store | 4 | `sp -0x64` | `[[sp+44]]` |
| `ldr` at 0x103664 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0x103670 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0x10367c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x103684 | store | 4 | `sp -0x68` | `([sp-100] + ret(0x2031c))` |
| `ldr` at 0x103688 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x103690 | store | 4 | `sp -0x68` | `([sp-100] + ret(0x2031c))+1` |
| `ldr` at 0x103698 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x10369c | store | 4 | `sp -0x68` | `[[sp+44]]` |
| `ldr` at 0x1036a0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x1036a4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x1036a8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1036ac | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1036b0 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1036b4 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1036bc | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x1036c0 | load | 4 | `sp +0x2c` | `-` |
| `str` at 0x1036c4 | store | 4 | `sp -0x6c` | `[sp+44]` |
| `ldr` at 0x1036c8 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1036cc | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1036d0 | store | 4 | `sp -0x70` | `[[sp+44]]` |
| `ldr` at 0x1036d4 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0x1036e0 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0x1036ec | load | 4 | `sp -0x70` | `-` |
| `str` at 0x1036f4 | store | 4 | `sp -0x74` | `([sp-112] + ret(0x2031c))` |
| `ldr` at 0x1036f8 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x103700 | store | 4 | `sp -0x74` | `([sp-112] + ret(0x2031c))+1` |
| `ldr` at 0x103708 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x10370c | store | 4 | `sp -0x74` | `[[sp+44]]` |
| `ldr` at 0x103710 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x103714 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x103718 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x10371c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103720 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103724 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x10372c | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x103730 | load | 4 | `sp +0x2c` | `-` |
| `str` at 0x103734 | store | 4 | `sp -0x78` | `[sp+44]` |
| `ldr` at 0x103738 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x10373c | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103740 | store | 4 | `sp -0x7c` | `[[sp+44]]` |
| `ldr` at 0x103744 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0x103750 | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0x10375c | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x103764 | store | 4 | `sp -0x80` | `([sp-124] + ret(0x2031c))` |
| `ldr` at 0x103768 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x103770 | store | 4 | `sp -0x80` | `([sp-124] + ret(0x2031c))+1` |
| `ldr` at 0x103778 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x10377c | store | 4 | `sp -0x80` | `[[sp+44]]` |
| `ldr` at 0x103780 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x103784 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x103788 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x10378c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103790 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103794 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x10379c | store | 4 | `[sp+44] +0x0` | `[[sp+44]]+4` |
| `ldr` at 0x1037a0 | load | 4 | `sp +0x2c` | `-` |
| `str` at 0x1037a4 | store | 4 | `sp -0x84` | `[sp+44]` |
| `ldr` at 0x1037a8 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x1037ac | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x1037b0 | store | 4 | `sp -0x88` | `[[sp+44]]` |
| `ldr` at 0x1037b4 | load | 4 | `sp +0x24` | `-` |
| `ldr` at 0x1037c0 | load | 4 | `sp +0x24` | `-` |
| `ldr` at 0x1037cc | load | 4 | `sp -0x88` | `-` |
| `str` at 0x1037d4 | store | 4 | `sp -0x8c` | `([sp-136] + ret(0x2031c))` |
| `ldr` at 0x1037d8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x1037e0 | store | 4 | `sp -0x8c` | `([sp-136] + ret(0x2031c))+1` |
| `ldr` at 0x1037e8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x1037ec | store | 4 | `sp -0x8c` | `[[sp+44]]` |
| `ldr` at 0x1037f0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x1037f4 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x1037f8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1037fc | load | 4 | `sp +0x28` | `-` |
| `ldr` at 0x103800 | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103804 | load | 4 | `[sp+44] +0x0` | `-` |
| `str` at 0x103810 | store | 4 | `[sp+40] +0x0` | `ret(0x201fc)` |
| `ldr` at 0x103814 | load | 4 | `[sp+40] +0x0` | `-` |
| `str` at 0x103818 | store | 4 | `sp -0x2c` | `[[sp+40]]` |
| `ldr` at 0x10381c | load | 4 | `sp +0x2c` | `-` |
| `ldr` at 0x103820 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103828 | load | 4 | `[sp+44] +0x0` | `-` |
| `ldr` at 0x103834 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x103840 | store | 1 | `[sp-44] +0x0` | `0x9` |
| `str` at 0x103848 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x10384c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103850 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x103858 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103860 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103868 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x103870 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103878 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103880 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x103888 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103890 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103898 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x10389c | store | 1 | `[sp-44] +0x3` | `([sp] & 0xff)` |
| `ldr` at 0x1038a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1038a8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x1038ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1038b0 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x1038b8 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x1038c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1038c8 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x1038d0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x1038d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1038e0 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x1038e8 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x1038f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1038f8 | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x1038fc | store | 1 | `[sp-44] +0x3` | `([sp+4] & 0xff)` |
| `ldr` at 0x103900 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103908 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x10390c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103910 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x103918 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x103920 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103928 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x103930 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103938 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103940 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x103948 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103950 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103958 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x10395c | store | 1 | `[sp-44] +0x3` | `([sp-28] & 0xff)` |
| `ldr` at 0x103960 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103968 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x10396c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103970 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x103978 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x103980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103988 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x103990 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103998 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1039a0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x1039a8 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x1039b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1039b8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x1039bc | store | 1 | `[sp-44] +0x3` | `([sp-32] & 0xff)` |
| `ldr` at 0x1039c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1039c8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x1039cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1039d0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x1039d8 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x1039e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1039e8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x1039f0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x1039f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103a00 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x103a08 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103a10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103a18 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0x103a1c | store | 1 | `[sp-44] +0x3` | `([sp-36] & 0xff)` |
| `ldr` at 0x103a20 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103a28 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x103a2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103a30 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x103a38 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x103a40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103a48 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x103a50 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103a58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103a60 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x103a68 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103a70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103a78 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x103a7c | store | 1 | `[sp-44] +0x3` | `([sp-40] & 0xff)` |
| `ldr` at 0x103a80 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103a88 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x103a8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103a90 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x103a98 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x103aa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103aa8 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x103ab0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103ab8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103ac0 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x103ac8 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103ad0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103ad8 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x103adc | store | 1 | `[sp-44] +0x3` | `([sp+8] & 0xff)` |
| `ldr` at 0x103ae0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103ae8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `str` at 0x103af0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x103af4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103af8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x103b04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103b08 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x103b0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103b20 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103b28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103b38 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103b40 | store | 4 | `sp -0x2c` | `[sp-44]+16` |
| `ldr` at 0x103b44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103b48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103b5c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103b64 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103b6c | store | 1 | `[sp-44] +0x10` | `?` |
| `ldr` at 0x103b74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103b7c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103b90 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103b98 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103ba0 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103ba8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103bb0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103bc4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103bcc | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103bd4 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103bdc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103be4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103bf8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103c00 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103c04 | store | 1 | `[sp-44] +0x3` | `?` |
| `ldr` at 0x103c08 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103c10 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `str` at 0x103c18 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x103c1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103c30 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103c38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103c3c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103c48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103c5c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103c64 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103c80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103c98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103cac | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103cb4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103cd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103cd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103ce8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103cfc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103d04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103d20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103d38 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103d44 | store | 4 | `sp -0x2c` | `(ret(0x2031c) + [sp-44])+1` |
| `ldr` at 0x103d48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103d4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103d60 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103d68 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103d84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103d8c | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103d94 | store | 1 | `(ret(0x2031c) + [sp-44]) +0x1` | `?` |
| `ldr` at 0x103d9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103da4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103db8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103dc0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103ddc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103de4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103dec | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103df4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103dfc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103e10 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103e18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103e34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103e3c | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103e44 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103e4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103e54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103e68 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103e70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103e8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103e94 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103e98 | store | 1 | `[sp-44] +0x3` | `?` |
| `ldr` at 0x103e9c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x103ea4 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x103ea8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103eac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103ec0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103ec8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103ee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103eec | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103ef4 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x103efc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103f04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103f18 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103f20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103f3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103f44 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103f4c | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x103f54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103f5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103f70 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103f78 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103f94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103f9c | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103fa4 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x103fac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x103fb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x103fc8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x103fd0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x103fec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x103ff4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x103ff8 | store | 1 | `[sp-44] +0x3` | `?` |
| `ldr` at 0x103ffc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x104004 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x104008 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10400c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104020 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104028 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x104044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10404c | load | 4 | `(unknown)` | `-` |
| `strb` at 0x104054 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x10405c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104064 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104078 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104080 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x10409c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1040a4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x1040ac | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x1040b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1040bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1040d0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1040d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1040f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1040fc | load | 4 | `(unknown)` | `-` |
| `strb` at 0x104104 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x10410c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104114 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104128 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104130 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x10414c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104154 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x104158 | store | 1 | `[sp-44] +0x3` | `?` |
| `ldr` at 0x10415c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x104164 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x104168 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10416c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104180 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104188 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1041a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1041ac | load | 4 | `(unknown)` | `-` |
| `strb` at 0x1041b4 | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x1041bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1041c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1041d8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1041e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1041fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104204 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x10420c | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x104214 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10421c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104230 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104238 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x104254 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10425c | load | 4 | `(unknown)` | `-` |
| `strb` at 0x104264 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x10426c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104274 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104288 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104290 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1042ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1042b4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x1042b8 | store | 1 | `[sp-44] +0x3` | `?` |
| `ldr` at 0x1042bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1042c4 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `str` at 0x1042cc | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x1042d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1042e4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1042ec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x104308 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104310 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104314 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x104320 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104324 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104338 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104340 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x10435c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104364 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x10436c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104374 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x10437c | store | 1 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x104384 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10438c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1043a0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1043a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1043c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1043cc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1043d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1043dc | load | 4 | `(unknown)` | `-` |
| `strb` at 0x1043e4 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x1043ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1043f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104408 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104410 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x10442c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104434 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x10443c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104444 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x10444c | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x104454 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10445c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104470 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104478 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x104494 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10449c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1044a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1044ac | load | 4 | `(unknown)` | `-` |
| `strb` at 0x1044b0 | store | 1 | `[sp-44] +0x3` | `?` |
| `ldr` at 0x1044b4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1044bc | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x1044c0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x1044c8 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0x1044d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1044e4 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x1044ec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x104508 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104510 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x104524 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x10452c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x104548 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104550 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104558 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10455c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104568 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x10457c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x104584 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1045a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1045a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1045b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1045b8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1045bc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1045c4 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x1045cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1045d4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x1045dc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x1045e8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1045f4 | store | 4 | `sp -0x90` | `ret(0x2031c)` |
| `ldr` at 0x1045f8 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x104600 | store | 4 | `sp -0x90` | `ret(0x2031c)+1` |
| `str` at 0x10460c | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x104610 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x104614 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x104618 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10461c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104624 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x10462c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104634 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x10463c | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x104644 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10464c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104654 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x10465c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104664 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104668 | store | 1 | `[sp-44] +0x3` | `([sp-64] & 0xff)` |
| `ldr` at 0x10466c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x104674 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x104678 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10467c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x104680 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x104688 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10468c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x104694 | store | 4 | `sp -0x2c` | `([sp-44] + [sp-64])` |
| `ldr` at 0x104698 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x1046a4 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x1046b0 | store | 4 | `sp -0x94` | `ret(0x2031c)` |
| `ldr` at 0x1046b4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x1046bc | store | 4 | `sp -0x94` | `ret(0x2031c)+1` |
| `str` at 0x1046c8 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0x1046cc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x1046d0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x1046d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1046d8 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x1046e0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x1046e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1046f0 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x1046f8 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x104700 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104708 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104710 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x104718 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104720 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104724 | store | 1 | `[sp-44] +0x3` | `([sp-64] & 0xff)` |
| `ldr` at 0x104728 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x104730 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x104734 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104738 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x10473c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x104744 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104748 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x104750 | store | 4 | `sp -0x2c` | `([sp-44] + [sp-64])` |
| `ldr` at 0x104754 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0x104760 | load | 4 | `sp +0x18` | `-` |
| `str` at 0x10476c | store | 4 | `sp -0x98` | `ret(0x2031c)` |
| `ldr` at 0x104770 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x104778 | store | 4 | `sp -0x98` | `ret(0x2031c)+1` |
| `str` at 0x104784 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x104788 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x10478c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x104790 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104794 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x10479c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x1047a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1047ac | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x1047b4 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x1047bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1047c4 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x1047cc | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x1047d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1047dc | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x1047e0 | store | 1 | `[sp-44] +0x3` | `([sp-64] & 0xff)` |
| `ldr` at 0x1047e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1047ec | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x1047f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1047f4 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0x1047f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x104800 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104804 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x10480c | store | 4 | `sp -0x2c` | `([sp-44] + [sp-64])` |
| `ldr` at 0x104810 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0x10481c | load | 4 | `sp +0x1c` | `-` |
| `str` at 0x104828 | store | 4 | `sp -0x9c` | `ret(0x2031c)` |
| `ldr` at 0x10482c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x104834 | store | 4 | `sp -0x9c` | `ret(0x2031c)+1` |
| `str` at 0x104840 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x104844 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x104848 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x10484c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104850 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104858 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x104860 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104868 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104870 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x104878 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104880 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104888 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x104890 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104898 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x10489c | store | 1 | `[sp-44] +0x3` | `([sp-64] & 0xff)` |
| `ldr` at 0x1048a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1048a8 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x1048ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1048b0 | load | 4 | `sp +0x1c` | `-` |
| `ldr` at 0x1048b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1048bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1048c0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x1048c8 | store | 4 | `sp -0x2c` | `([sp-44] + [sp-64])` |
| `ldr` at 0x1048cc | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0x1048d8 | load | 4 | `sp +0x20` | `-` |
| `str` at 0x1048e4 | store | 4 | `sp -0xa0` | `ret(0x2031c)` |
| `ldr` at 0x1048e8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x1048f0 | store | 4 | `sp -0xa0` | `ret(0x2031c)+1` |
| `str` at 0x1048fc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x104900 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x104904 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x104908 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10490c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104914 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x10491c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104924 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x10492c | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x104934 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10493c | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104944 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x10494c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104954 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104958 | store | 1 | `[sp-44] +0x3` | `([sp-64] & 0xff)` |
| `ldr` at 0x10495c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x104964 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x104968 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10496c | load | 4 | `sp +0x20` | `-` |
| `ldr` at 0x104970 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x104978 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10497c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x104984 | store | 4 | `sp -0x2c` | `([sp-44] + [sp-64])` |
| `ldr` at 0x104988 | load | 4 | `sp +0x24` | `-` |
| `ldr` at 0x104994 | load | 4 | `sp +0x24` | `-` |
| `str` at 0x1049a0 | store | 4 | `sp -0xa4` | `ret(0x2031c)` |
| `ldr` at 0x1049a4 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x1049ac | store | 4 | `sp -0xa4` | `ret(0x2031c)+1` |
| `str` at 0x1049b8 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x1049bc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x1049c0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x1049c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1049c8 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x1049d0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x1049d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1049e0 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x1049e8 | store | 1 | `[sp-44] +0x1` | `?` |
| `ldr` at 0x1049f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1049f8 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104a00 | store | 1 | `[sp-44] +0x2` | `?` |
| `ldr` at 0x104a08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104a10 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0x104a14 | store | 1 | `[sp-44] +0x3` | `([sp-64] & 0xff)` |
| `ldr` at 0x104a18 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x104a20 | store | 4 | `sp -0x2c` | `[sp-44]+4` |
| `ldr` at 0x104a24 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104a28 | load | 4 | `sp +0x24` | `-` |
| `ldr` at 0x104a2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x104a34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x104a38 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x104a40 | store | 4 | `sp -0x2c` | `([sp-44] + [sp-64])` |
| `ldm` at 0x104a48 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x104a48 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x104a48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x104a48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x104a48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x103418 | `(unknown callee) 0x2031c` | `?` | `?` | `?` | `?` |
| `bl` at 0x103594 | `(unknown callee) 0x2031c` | `[sp+16]` | `?` | `[sp+44]` | `[sp+16]` |
| `bl` at 0x103604 | `(unknown callee) 0x2031c` | `[sp+20]` | `?` | `[sp+44]` | `[sp+20]` |
| `bl` at 0x103674 | `(unknown callee) 0x2031c` | `[sp+24]` | `?` | `[sp+44]` | `[sp+24]` |
| `bl` at 0x1036e4 | `(unknown callee) 0x2031c` | `[sp+28]` | `?` | `[sp+44]` | `[sp+28]` |
| `bl` at 0x103754 | `(unknown callee) 0x2031c` | `[sp+32]` | `?` | `[sp+44]` | `[sp+32]` |
| `bl` at 0x1037c4 | `(unknown callee) 0x2031c` | `[sp+36]` | `?` | `[sp+44]` | `[sp+36]` |
| `bl` at 0x103808 | `(unknown callee) 0x201fc` | `[[sp+44]]` | `?` | `?` | `[sp+44]` |
| `bl` at 0x10382c | `(unknown callee) 0x20550` | `[[sp+40]]` | `0x0` | `[[sp+44]]` | `[sp+44]` |
| `bl` at 0x103b34 | `(unknown callee) 0x203f4` | `?` | `?` | `0x10` | `?` |
| `bl` at 0x103c8c | `(unknown callee) 0x2031c` | `?` | `?` | `?` | `?` |
| `bl` at 0x103ce4 | `(unknown callee) 0x203f4` | `[sp-44]` | `?` | `ret(0x2031c)+1` | `?` |
| `bl` at 0x103d2c | `(unknown callee) 0x2031c` | `?` | `?` | `?` | `?` |
| `bl` at 0x104564 | `(unknown callee) 0x203f4` | `[sp-44]+4` | `?` | `?` | `?` |
| `bl` at 0x1045ec | `(unknown callee) 0x2031c` | `[sp+16]` | `?` | `?` | `[sp+16]` |
| `bl` at 0x104684 | `(unknown callee) 0x203f4` | `[sp-44]+4` | `[sp+16]` | `[sp-64]` | `[sp-44]+4` |
| `bl` at 0x1046a8 | `(unknown callee) 0x2031c` | `[sp+20]` | `?` | `[sp-44]` | `[sp+20]` |
| `bl` at 0x104740 | `(unknown callee) 0x203f4` | `[sp-44]+4` | `[sp+20]` | `[sp-64]` | `[sp-44]+4` |
| `bl` at 0x104764 | `(unknown callee) 0x2031c` | `[sp+24]` | `?` | `[sp-44]` | `[sp+24]` |
| `bl` at 0x1047fc | `(unknown callee) 0x203f4` | `[sp-44]+4` | `[sp+24]` | `[sp-64]` | `[sp-44]+4` |
| `bl` at 0x104820 | `(unknown callee) 0x2031c` | `[sp+28]` | `?` | `[sp-44]` | `[sp+28]` |
| `bl` at 0x1048b8 | `(unknown callee) 0x203f4` | `[sp-44]+4` | `[sp+28]` | `[sp-64]` | `[sp-44]+4` |
| `bl` at 0x1048dc | `(unknown callee) 0x2031c` | `[sp+32]` | `?` | `[sp-44]` | `[sp+32]` |
| `bl` at 0x104974 | `(unknown callee) 0x203f4` | `[sp-44]+4` | `[sp+32]` | `[sp-64]` | `[sp-44]+4` |
| `bl` at 0x104998 | `(unknown callee) 0x2031c` | `[sp+36]` | `?` | `[sp-44]` | `[sp+36]` |
| `bl` at 0x104a30 | `(unknown callee) 0x203f4` | `[sp-44]+4` | `[sp+36]` | `[sp-64]` | `[sp-44]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10336c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0x1033c8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x10358c | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0x1035fc | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0x10366c | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `beq` at 0x1036dc | `[sp+28] == 0x0` | `[sp+28] != 0x0` |
| `beq` at 0x10374c | `[sp+32] == 0x0` | `[sp+32] != 0x0` |
| `beq` at 0x1037bc | `[sp+36] == 0x0` | `[sp+36] != 0x0` |
| `bge` at 0x103b00 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0x103c44 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x10431c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x1045e4 | `[sp+16] == 0x0` | `[sp+16] != 0x0` |
| `beq` at 0x1046a0 | `[sp+20] == 0x0` | `[sp+20] != 0x0` |
| `beq` at 0x10475c | `[sp+24] == 0x0` | `[sp+24] != 0x0` |
| `beq` at 0x104818 | `[sp+28] == 0x0` | `[sp+28] != 0x0` |
| `beq` at 0x1048d4 | `[sp+32] == 0x0` | `[sp+32] != 0x0` |
| `beq` at 0x104990 | `[sp+36] == 0x0` | `[sp+36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x104a48 | `r0` | `ret(0x203f4)` |

## `FRUUnpackData`

- `FRUUnpackData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x104a4c to 0x105ca8.
  Established. A byte loaded from the address held in [sp-24] is stored as a word at the address held in [sp-28] (`ldrb` at 0x104a84, `str` at 0x104a88).
  The result from unnamed 0x201fc is stored at the address formed by [sp-28] plus 0x20 (`bl` at 0x104d8c, `str` at 0x104d94).
  A word store writes zero at sp-0x34 (`str` at 0x105c98).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x104a5c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x104a60 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x104a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104a74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104a7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104a84 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x104a88 | store | 4 | `[sp-28] +0x0` | `[[sp-24]]:1` |
| `str` at 0x104a90 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x104a94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104a98 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x104aa8 | store | 4 | `sp -0x34` | `0x5` |
| `ldr` at 0x104ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104ab4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104ab8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x104ac4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104acc | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x104adc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104ae4 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x104af4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104afc | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x104b04 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0x104b08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104b10 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104b14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104b18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104b1c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x104b28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104b30 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x104b40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104b48 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x104b58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104b60 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x104b68 | store | 4 | `[sp-28] +0x8` | `?` |
| `ldr` at 0x104b6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104b74 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104b78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104b7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104b80 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x104b8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104b94 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x104ba4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104bac | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x104bbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104bc4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x104bcc | store | 4 | `[sp-28] +0x14` | `?` |
| `ldr` at 0x104bd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104bd8 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104be0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104be4 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x104bf0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104bf8 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x104c08 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104c10 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x104c20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104c28 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x104c30 | store | 4 | `[sp-28] +0xc` | `?` |
| `ldr` at 0x104c34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104c3c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104c40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104c44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104c48 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x104c54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104c5c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x104c6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104c74 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x104c84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104c8c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x104c94 | store | 4 | `[sp-28] +0x10` | `?` |
| `ldr` at 0x104c98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104ca0 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104ca8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104cac | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x104cb8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104cc0 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x104cd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104cd8 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x104ce8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104cf0 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x104cf8 | store | 4 | `[sp-28] +0x18` | `?` |
| `ldr` at 0x104cfc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104d04 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104d0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104d10 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x104d1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104d24 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x104d34 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104d3c | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x104d4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104d54 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x104d5c | store | 4 | `[sp-28] +0x1c` | `?` |
| `ldr` at 0x104d60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104d68 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104d6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104d70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104d74 | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0x104d94 | store | 4 | `[sp-28] +0x20` | `ret(0x201fc)` |
| `ldr` at 0x104d98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104d9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104da0 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x104db4 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104dc8 | load | 4 | `[sp-28] +0x20` | `-` |
| `str` at 0x104dd8 | store | 4 | `sp -0x34` | `0x7` |
| `str` at 0x104de4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x104de8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104dec | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x104df0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104dfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104e00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104e14 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104e20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x104e2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104e34 | store | 4 | `sp -0x18` | `[sp-24]+16` |
| `ldr` at 0x104e38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104e3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104e50 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104e58 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104e5c | load | 1 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x104e68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104e70 | load | 1 | `[sp-24] +0x11` | `-` |
| `ldr` at 0x104e80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104e88 | load | 1 | `[sp-24] +0x12` | `-` |
| `ldr` at 0x104e98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x104ea0 | load | 1 | `[sp-24] +0x13` | `-` |
| `str` at 0x104ea8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x104eac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x104eb4 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x104eb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104ebc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104ed0 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104ed8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104ee8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104eec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104f00 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104f08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104f0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104f20 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104f28 | load | 4 | `(unknown)` | `-` |
| `str` at 0x104f50 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0x104f54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104f58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104f6c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104f74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104f78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104f8c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104f94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104fb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x104fc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x104fc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x104fd8 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x104fe0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x104ff0 | store | 4 | `sp -0x34` | `0x7` |
| `str` at 0x104ffc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x105000 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105004 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105018 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105020 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105024 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105030 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105044 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105058 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105060 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10507c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105088 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1050a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1050b0 | store | 4 | `sp -0x18` | `(ret(0x2031c) + [sp-24])+1` |
| `ldr` at 0x1050b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1050b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1050cc | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x1050d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1050f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1050f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1050fc | load | 1 | `(ret(0x2031c) + [sp-24]) +0x1` | `-` |
| `ldr` at 0x105108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105110 | load | 1 | `(ret(0x2031c) + [sp-24]) +0x2` | `-` |
| `ldr` at 0x105120 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105128 | load | 1 | `(ret(0x2031c) + [sp-24]) +0x3` | `-` |
| `ldr` at 0x105138 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105140 | load | 1 | `(ret(0x2031c) + [sp-24]) +0x4` | `-` |
| `str` at 0x105148 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x10514c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105154 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x105158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10515c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105170 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105178 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105194 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10519c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1051a0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x1051ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1051b4 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x1051c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1051cc | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x1051dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1051e4 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x1051ec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1051f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1051f8 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x1051fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105200 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105214 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x10521c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105238 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105240 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105244 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x105250 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105258 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x105268 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105270 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x105280 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105288 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x105290 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x105294 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10529c | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x1052a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1052a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1052b8 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x1052c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1052dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1052e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1052e8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x1052f4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1052fc | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x10530c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105314 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x105324 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10532c | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x105334 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x105338 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105340 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x105344 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105348 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10535c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105364 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105380 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105388 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10538c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1053a0 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x1053a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1053c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1053cc | load | 4 | `(unknown)` | `-` |
| `str` at 0x1053e0 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0x1053e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1053e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1053fc | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105404 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105420 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105428 | load | 4 | `(unknown)` | `-` |
| `str` at 0x105438 | store | 4 | `sp -0x34` | `0x7` |
| `ldr` at 0x105440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105444 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105458 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10547c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10549c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x1054a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1054c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1054c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1054d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1054e4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x1054e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1054ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105500 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105508 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105524 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10552c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105530 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x10553c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105540 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x10554c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105554 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x105564 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10556c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x10557c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105584 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x10558c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x105590 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105598 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x10559c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1055a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1055b4 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x1055bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1055d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1055e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1055e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1055f0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1055f4 | store | 4 | `(unknown)` | `[sp-48]` |
| `ldr` at 0x1055f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x105600 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x105608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10560c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105620 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105628 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105644 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10564c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105650 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105664 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x10566c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105688 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105690 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1056a4 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0x1056a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1056ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1056c0 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x1056c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1056e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1056ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x1056fc | store | 4 | `sp -0x34` | `0x7` |
| `ldr` at 0x105704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105708 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10571c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105724 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105740 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105748 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10574c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105760 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105768 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105784 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10578c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105794 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1057a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1057a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1057bc | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x1057c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1057e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1057e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1057ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x105800 | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105808 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105824 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x10582c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105834 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105848 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10585c | load | 4 | `[sp-28] +0x20` | `-` |
| `ldr` at 0x105864 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105880 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105888 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x105890 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105898 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x10589c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1058a4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x1058ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1058b4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x1058bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1058c0 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x1058cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1058d4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x1058e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1058ec | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x1058fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105904 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x10590c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x105910 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105918 | store | 4 | `sp -0x18` | `[sp-24]+4` |
| `ldr` at 0x10591c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10592c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105938 | store | 4 | `[sp-28] +0x24` | `ret(0x201fc)` |
| `ldr` at 0x10593c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105940 | load | 4 | `[sp-28] +0x24` | `-` |
| `ldr` at 0x105944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105948 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105954 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x10595c | store | 4 | `sp -0x18` | `([sp-24] + [sp-44])` |
| `ldr` at 0x105960 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105964 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105970 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105978 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105988 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105990 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x1059a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1059a8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x1059b0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1059b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1059bc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x1059c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1059cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1059d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1059dc | store | 4 | `[sp-28] +0x28` | `ret(0x201fc)` |
| `ldr` at 0x1059e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1059e4 | load | 4 | `[sp-28] +0x28` | `-` |
| `ldr` at 0x1059e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1059ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1059f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1059f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105a00 | store | 4 | `sp -0x18` | `([sp-24] + [sp-44])` |
| `ldr` at 0x105a04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105a08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105a14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105a1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105a2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105a34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105a44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105a4c | load | 1 | `(unknown)` | `-` |
| `str` at 0x105a54 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x105a58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105a60 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x105a64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105a70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105a74 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105a80 | store | 4 | `[sp-28] +0x2c` | `ret(0x201fc)` |
| `ldr` at 0x105a84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105a88 | load | 4 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0x105a8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105a90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105a98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105a9c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105aa4 | store | 4 | `sp -0x18` | `([sp-24] + [sp-44])` |
| `ldr` at 0x105aa8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105aac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105ab8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105ac0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105ad0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105ad8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105ae8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105af0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x105af8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x105afc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105b04 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x105b08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105b14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105b18 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105b24 | store | 4 | `[sp-28] +0x30` | `ret(0x201fc)` |
| `ldr` at 0x105b28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105b2c | load | 4 | `[sp-28] +0x30` | `-` |
| `ldr` at 0x105b30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105b34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105b3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105b40 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105b48 | store | 4 | `sp -0x18` | `([sp-24] + [sp-44])` |
| `ldr` at 0x105b4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105b50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105b5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105b64 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105b74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105b7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105b8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105b94 | load | 1 | `(unknown)` | `-` |
| `str` at 0x105b9c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x105ba0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105ba8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x105bac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105bb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105bbc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105bc8 | store | 4 | `[sp-28] +0x34` | `ret(0x201fc)` |
| `ldr` at 0x105bcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105bd0 | load | 4 | `[sp-28] +0x34` | `-` |
| `ldr` at 0x105bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105bd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105be0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105be4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105bec | store | 4 | `sp -0x18` | `([sp-24] + [sp-44])` |
| `ldr` at 0x105bf0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105bf4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105c00 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105c08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105c18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105c20 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105c30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x105c38 | load | 1 | `(unknown)` | `-` |
| `str` at 0x105c40 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x105c44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105c4c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x105c50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105c5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105c60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105c6c | store | 4 | `[sp-28] +0x38` | `ret(0x201fc)` |
| `ldr` at 0x105c70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105c74 | load | 4 | `[sp-28] +0x38` | `-` |
| `ldr` at 0x105c78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105c7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105c84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105c88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x105c90 | store | 4 | `sp -0x18` | `([sp-24] + [sp-44])` |
| `str` at 0x105c98 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x105c9c | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x105ca4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x105ca4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x105ca4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x105ca4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x104a70 | `(unknown callee) 0x20550` | `r1` | `0x0` | `0x3c` | `r3` |
| `bl` at 0x104d8c | `(unknown callee) 0x201fc` | `?` | `?` | `[[sp-28]+28]` | `?` |
| `bl` at 0x104dc0 | `(unknown callee) 0x20550` | `[[sp-28]+32]` | `0x0` | `?` | `?` |
| `bl` at 0x104e28 | `(unknown callee) 0x203f4` | `?` | `[sp-24]` | `0x10` | `?` |
| `bl` at 0x104f48 | `(unknown callee) 0x201fc` | `?` | `[sp-28]` | `?` | `?` |
| `bl` at 0x104fbc | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x105034 | `(unknown callee) 0x2031c` | `[sp-24]` | `[sp-28]` | `?` | `?` |
| `bl` at 0x105090 | `(unknown callee) 0x203f4` | `?` | `[sp-24]` | `ret(0x2031c)+1` | `?` |
| `bl` at 0x105098 | `(unknown callee) 0x2031c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x1053d8 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x1054dc | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x10569c | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x1057a0 | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x105840 | `(unknown callee) 0x203f4` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0x105930 | `(unknown callee) 0x201fc` | `[sp-44]` | `?` | `0x3` | `[sp-44]` |
| `bl` at 0x10594c | `(unknown callee) 0x203f4` | `[[sp-28]+36]` | `[sp-24]` | `[sp-44]` | `[sp-28]` |
| `bl` at 0x1059d4 | `(unknown callee) 0x201fc` | `[sp-44]` | `?` | `0x3` | `[sp-44]` |
| `bl` at 0x1059f0 | `(unknown callee) 0x203f4` | `[[sp-28]+40]` | `[sp-24]` | `[sp-44]` | `[sp-28]` |
| `bl` at 0x105a78 | `(unknown callee) 0x201fc` | `[sp-44]` | `?` | `0x3` | `[sp-44]` |
| `bl` at 0x105a94 | `(unknown callee) 0x203f4` | `[[sp-28]+44]` | `[sp-24]` | `[sp-44]` | `[sp-28]` |
| `bl` at 0x105b1c | `(unknown callee) 0x201fc` | `[sp-44]` | `?` | `0x3` | `[sp-44]` |
| `bl` at 0x105b38 | `(unknown callee) 0x203f4` | `[[sp-28]+48]` | `[sp-24]` | `[sp-44]` | `[sp-28]` |
| `bl` at 0x105bc0 | `(unknown callee) 0x201fc` | `[sp-44]` | `?` | `0x3` | `[sp-44]` |
| `bl` at 0x105bdc | `(unknown callee) 0x203f4` | `[[sp-28]+52]` | `[sp-24]` | `[sp-44]` | `[sp-28]` |
| `bl` at 0x105c64 | `(unknown callee) 0x201fc` | `[sp-44]` | `?` | `0x3` | `[sp-44]` |
| `bl` at 0x105c80 | `(unknown callee) 0x203f4` | `[[sp-28]+56]` | `[sp-24]` | `[sp-44]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x104aa0 | `[[sp-28]] == 0x9` | `[[sp-28]] != 0x9` |
| `bne` at 0x104dd0 | `[[sp-28]+32] != 0x0` | `[[sp-28]+32] == 0x0` |
| `ble` at 0x104df8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x104ee0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x104fe8 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x10502c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x105430 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x105538 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x1056f4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x105924 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x1059c8 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x105a6c | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x105b10 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x105bb4 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `beq` at 0x105c58 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x105ca4 | `r0` | `?` |

## `FRUAttach`

- `FRUAttach`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x105ca8 to 0x105d8c.
  Established. Byte stores at unknown effective addresses write three and zero (`strb` at 0x105cf8, `strb` at 0x105d10).
  A word store with an unknown effective address writes ([sp-28] + [sp-36]) (`ldr` at 0x105d24, `ldr` at 0x105d28, `add` at 0x105d2c, `str` at 0x105d30).
  A load, add and store sequence writes [sp-20]+1 at sp-0x14 (`ldr` at 0x105d54, `add` at 0x105d58, `str` at 0x105d5c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x105cb8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x105cbc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x105cc0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x105cc4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x105ccc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x105cd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x105cd4 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x105cd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x105ce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105cec | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x105cf8 | store | 1 | `(unknown)` | `0x3` |
| `ldr` at 0x105cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105d04 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x105d10 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x105d14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105d1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105d24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105d28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x105d30 | store | 4 | `(unknown)` | `([sp-28] + [sp-36])` |
| `ldr` at 0x105d34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105d3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x105d44 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x105d50 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0x105d54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x105d5c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x105d60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x105d68 | store | 4 | `sp -0x28` | `[sp-40]-61440` |
| `ldr` at 0x105d6c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x105d74 | store | 4 | `sp -0x24` | `[sp-36]+61440` |
| `ldr` at 0x105d7c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x105d88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x105d88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x105d88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x105ce0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x105d88 | `r0` | `?` |

## `FRUDetach`

- `FRUDetach`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x105d8c to 0x105fb0.
  Established. The call to unnamed 0x201fc has its result stored as a word at sp-0x18 (`bl` at 0x105e28, `str` at 0x105e30).
  A load and store copy that result to sp-0x1c (`bl` at 0x105e28, `ldr` at 0x105e34, `str` at 0x105e38).
  The call to unnamed 0xfde7c records its result at sp-0x2c (`bl` at 0x105ef0, `str` at 0x105ef4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x105d9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x105da4 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x105dac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x105db0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x105db4 | load | 1 | `r0 +0x20` | `-` |
| `ldr` at 0x105db8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105dc8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x105ddc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105dec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105df0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x105e04 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x105e08 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x105e10 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x105e14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x105e1c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x105e24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x105e30 | store | 4 | `sp -0x18` | `ret(0x201fc)` |
| `ldr` at 0x105e34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105e38 | store | 4 | `sp -0x1c` | `ret(0x201fc)` |
| `ldr` at 0x105e3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105e4c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x105e58 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x105e5c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x105e60 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x105e64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105e70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105e74 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x105e88 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105ea0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x105eb4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x105ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105ec8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105edc | load | 4 | `(unknown)` | `-` |
| `str` at 0x105ee0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x105ee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105eec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x105ef4 | store | 4 | `sp -0x2c` | `ret(0xfde7c)` |
| `ldr` at 0x105ef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105efc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x105f10 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x105f14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x105f18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x105f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105f28 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x105f3c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x105f40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x105f48 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x105f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105f50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x105f64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x105f68 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x105f7c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x105f88 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x105f8c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x105f94 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x105f9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x105fa0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x105fa4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x105fac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x105fac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x105fac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x105e28 | `(unknown callee) 0x201fc` | `?` | `?` | `[r0+32]:1` | `?` |
| `bl` at 0x105ef0 | `(unknown callee) 0xfde7c` | `[sp-20]` | `?` | `[sp-20]` | `?` |
| `bl` at 0x105f20 | `(unknown callee) 0x203f4` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x105dc0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x105de8 | `? != 0x3` | `? == 0x3` |
| `bne` at 0x105e44 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `ble` at 0x105e6c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x105e94 | `? == 0x3` | `? != 0x3` |
| `bne` at 0x105ec0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x105fac | `r0` | `?` |
