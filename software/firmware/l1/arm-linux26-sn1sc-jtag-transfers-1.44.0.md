# ARM Linux 2.6 sn1sc and JTAG routines in 1.44.0

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
register locations r0 through r3 with unknown arity. Custom assembly calling conventions
remain unknown. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
An indirect transfer in sn1sc_process_scancmd remains unresolved; its tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-sn1sc-jtag-transfers-facts.txt) retain all tool sections.

## `sn1sc_process_scancmd`

- `sn1sc_process_scancmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1347dc to 0x13648c.
  Established. A byte store writes 0 to absolute address 0x31e4ec (`strb` at 0x134844).
  Word stores to memory at [sp-24] have unknown values on the resolved paths (`str` at 0x134880, `str` at 0x134974, `str` at 0x1363f0).
  The indirect transfer at 0x13499c has an unknown taken target and an unknown operand in its unsigned comparison with 0x15 (`ldrls` at 0x13499c).
  Omitted effects beyond that transfer remain unknown (`ldrls` at 0x13499c).
  A later call to unnamed 0x12c684 has printed r0-r3 candidates 0, 0x190e60, 0 and [sp-24] (`bl` at 0x136474).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1347ec | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x1347f0 | load | 4 | `abs 0x135540` | `-` |
| `ldr` at 0x1347f4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x134804 | load | 4 | `abs 0x13550c` | `-` |
| `ldr` at 0x13480c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134818 | load | 4 | `abs 0x135540` | `-` |
| `ldr` at 0x13481c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x13482c | load | 4 | `abs 0x135510` | `-` |
| `ldr` at 0x134834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13483c | load | 4 | `abs 0x135538` | `-` |
| `strb` at 0x134844 | store | 1 | `abs 0x31e4ec` | `0x0` |
| `ldr` at 0x134848 | load | 4 | `abs 0x135514` | `-` |
| `ldr` at 0x13484c | load | 4 | `abs 0x315030` | `-` |
| `str` at 0x13485c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x134860 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134864 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x134870 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134874 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x134880 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0x134888 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x134894 | load | 4 | `abs 0x135518` | `-` |
| `ldr` at 0x13489c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1348a4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x1348b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1348bc | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x1348d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1348d8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x1348e4 | load | 4 | `abs 0x1a0874` | `-` |
| `ldr` at 0x134908 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134910 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x134920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134928 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x13493c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134944 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x134950 | load | 4 | `abs 0x135538` | `-` |
| `ldr` at 0x134954 | load | 4 | `abs 0x13551c` | `-` |
| `ldr` at 0x134964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134968 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13496c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x134974 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0x13497c | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x134984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x134988 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0x13499c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1363b8 | load | 4 | `abs 0x135538` | `-` |
| `ldr` at 0x1363bc | load | 4 | `abs 0x135530` | `-` |
| `str` at 0x1363c8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x1363d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1363d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1363e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1363e4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x1363f0 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0x1363f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x136400 | load | 4 | `abs 0x135540` | `-` |
| `ldr` at 0x136404 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x136410 | load | 4 | `abs 0x135538` | `-` |
| `ldrb` at 0x136414 | load | 1 | `abs 0x31e4ec` | `-` |
| `ldr` at 0x136420 | load | 4 | `abs 0x135534` | `-` |
| `ldr` at 0x136424 | load | 4 | `abs 0x135538` | `-` |
| `ldr` at 0x13642c | load | 4 | `abs 0x135540` | `-` |
| `ldr` at 0x136430 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x136440 | load | 4 | `abs 0x13553c` | `-` |
| `ldr` at 0x136448 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x136454 | load | 4 | `abs 0x135540` | `-` |
| `ldr` at 0x136458 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x136468 | load | 4 | `abs 0x135544` | `-` |
| `ldr` at 0x136470 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x136478 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13647c | store | 4 | `sp -0x64` | `[sp-28]` |
| `ldr` at 0x136480 | load | 4 | `sp -0x64` | `-` |
| `ldm` at 0x136488 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x136488 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x136488 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x136488 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x134810 | `(unknown callee) 0x12c684` | `0x0` | `0x190d70` | `0x1` | `r0` |
| `bl` at 0x134838 | `(unknown callee) 0x12c684` | `0x0` | `0x190d94` | `0x0` | `r0` |
| `bl` at 0x134890 | `(unknown callee) 0x134634` | `?` | `?` | `0x31e4ec` | `[0x315030]` |
| `mov` at 0x1348f4 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x1a0874]` | `?` |
| `bl` at 0x13495c | `(unknown callee) 0x205bc` | `0x31e4ec` | `0x190db8` | `?` | `?` |
| `bl` at 0x134960 | `(unknown callee) 0x1346fc` | `ret(0x205bc)` | `?` | `?` | `?` |
| `bl` at 0x1363c0 | `(unknown callee) 0x20634` | `0x31e4ec` | `0x190dfc` | `?` | `?` |
| `bl` at 0x1363cc | `(unknown callee) 0x1346fc` | `ret(0x20634)` | `?` | `?` | `0x0` |
| `bl` at 0x136428 | `(unknown callee) 0x243d4` | `0x190e28` | `0x31e4ec` | `?` | `[0x31e4ec]:1` |
| `bl` at 0x13644c | `(unknown callee) 0x12c684` | `0x0` | `0x190e38` | `0x1` | `[sp-24]` |
| `bl` at 0x136474 | `(unknown callee) 0x12c684` | `0x0` | `0x190e60` | `0x0` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x1347fc | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0x134824 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0x134854 | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `beq` at 0x134900 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `ldrls` at 0x13499c | `? <= 0x15 (unsigned, 32-bit)` | `? > 0x15 (unsigned, 32-bit)` |
| `bne` at 0x1363fc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x13640c | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x13641c | `[0x31e4ec]:1 == 0x0` | `[0x31e4ec]:1 != 0x0` |
| `bls` at 0x136438 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0x136460 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x136488 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x13499c).

## `sn1sc_reset_brick`

- `sn1sc_reset_brick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x136a38 to 0x136b30.
  Established. A word store writes 0xfffff7cc to an unknown effective target (`str` at 0x136a8c).
  The result of the call through an unknown target is stored as a word at entry sp minus 0x24 (`mov` at 0x136ad0, `str` at 0x136ad8).
  Another word store writes [sp-36] to memory at [sp-24] (`str` at 0x136af0).
  The final computed word stored at entry sp minus 0x28 has an unknown value after predicated effects merge (`movne` at 0x136b00, `moveq` at 0x136b04, `str` at 0x136b08).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x136a48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x136a4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x136a54 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x136a58 | load | 4 | `abs 0x136b18` | `-` |
| `ldr` at 0x136a5c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x136a68 | load | 4 | `abs 0x136b1c` | `-` |
| `ldr` at 0x136a6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136a74 | load | 4 | `abs 0x136b20` | `-` |
| `ldr` at 0x136a78 | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x136a84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x136a88 | load | 4 | `abs 0x136b24` | `-` |
| `str` at 0x136a8c | store | 4 | `(unknown)` | `0xfffff7cc` |
| `str` at 0x136a94 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x136aa8 | load | 4 | `abs 0x136b28` | `-` |
| `ldr` at 0x136aac | load | 4 | `abs 0x318454` | `-` |
| `ldr` at 0x136ab8 | load | 4 | `abs 0x136b28` | `-` |
| `ldr` at 0x136abc | load | 4 | `abs 0x318454` | `-` |
| `ldr` at 0x136ac8 | load | 4 | `abs 0x136b2c` | `-` |
| `str` at 0x136ad8 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x136adc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x136ae8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x136aec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x136af0 | store | 4 | `[sp-24] +0x0` | `[sp-36]` |
| `ldr` at 0x136af4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x136af8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x136b08 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x136b0c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x136b14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x136b14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x136b14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x136a70 | `(unknown callee) 0x243d4` | `0x190f70` | `r0` | `r2` | `[0x315034]` |
| `bl` at 0x136a9c | `(unknown callee) 0x136c74` | `?` | `?` | `?` | `[0x315030]` |
| `bl` at 0x136aa4 | `(unknown callee) 0x13831c` | `?` | `?` | `?` | `?` |
| `mov` at 0x136ad0 | `(unknown callee) (unknown target)` | `?` | `0x1` | `0x31e4ec` | `[0x318454]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x136a64 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bne` at 0x136a80 | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `beq` at 0x136ab4 | `[0x318454] == 0x0` | `[0x318454] != 0x0` |
| `beq` at 0x136ae4 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x136b14 | `r0` | `?` |

## `sn1sc_reset_slot`

- `sn1sc_reset_slot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x136b30 to 0x136c74.
  Established. A word store writes 0xfffff7cc at entry sp minus 0x1c (`str` at 0x136b7c).
  Four computed word loads retain unknown effective addresses (`ldr` at 0x136bfc, `ldr` at 0x136c00, `ldr` at 0x136c20, `ldr` at 0x136c24).
  The result of the call through an unknown target is stored as a word at entry sp minus 0x18 (`mov` at 0x136c34, `str` at 0x136c3c).
  A later word store writes [sp-24] at entry sp minus 0x1c (`str` at 0x136c48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x136b40 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x136b48 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x136b4c | load | 4 | `abs 0x136c58` | `-` |
| `ldr` at 0x136b50 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x136b5c | load | 4 | `abs 0x136c5c` | `-` |
| `ldr` at 0x136b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136b68 | load | 4 | `abs 0x136c60` | `-` |
| `ldr` at 0x136b6c | load | 4 | `abs 0x315030` | `-` |
| `ldr` at 0x136b78 | load | 4 | `abs 0x136c64` | `-` |
| `str` at 0x136b7c | store | 4 | `sp -0x1c` | `0xfffff7cc` |
| `ldr` at 0x136b8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x136b9c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x136ba4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x136bbc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x136bc8 | load | 4 | `abs 0x136c68` | `-` |
| `ldr` at 0x136bcc | load | 4 | `abs 0x1a0874` | `-` |
| `ldr` at 0x136bd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136be0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136bf0 | load | 4 | `abs 0x136c6c` | `-` |
| `ldr` at 0x136bf4 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0x136bfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x136c00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x136c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136c14 | load | 4 | `abs 0x136c6c` | `-` |
| `ldr` at 0x136c18 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0x136c20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x136c24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x136c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136c2c | load | 4 | `abs 0x136c70` | `-` |
| `str` at 0x136c3c | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0x136c44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x136c48 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x136c4c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x136c54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x136c54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x136c54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x136b64 | `(unknown callee) 0x243d4` | `0x190f90` | `r0` | `r2` | `[0x315034]` |
| `bl` at 0x136b84 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `[0x315030]` |
| `bl` at 0x136ba8 | `(unknown callee) 0x3ec6c` | `[sp-20]` | `?` | `ret(0x6e190)` | `[sp-20]` |
| `bl` at 0x136bc4 | `(unknown callee) 0x134634` | `ret(0x3ec6c)` | `?` | `?` | `ret(0x3ec6c)` |
| `mov` at 0x136bdc | `(unknown callee) (unknown target)` | `0x0` | `[sp-20]` | `?` | `[0x1a0874]` |
| `bl` at 0x136be4 | `(unknown callee) 0x13831c` | `[sp-20]` | `?` | `?` | `?` |
| `mov` at 0x136c34 | `(unknown callee) (unknown target)` | `[sp-20]` | `0x31e4ec` | `?` | `?` |
| `bl` at 0x136c40 | `(unknown callee) 0x1346fc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x136b58 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bne` at 0x136b74 | `[0x315030] != 0x0` | `[0x315030] == 0x0` |
| `bne` at 0x136b94 | `ret(0x6e190) != [sp-20]` | `ret(0x6e190) == [sp-20]` |
| `bne` at 0x136bb4 | `ret(0x3ec6c) != 0x0` | `ret(0x3ec6c) == 0x0` |
| `beq` at 0x136c08 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x136c54 | `r0` | `?` |

## `sn1sc_reset_sic`

- `sn1sc_reset_sic`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x136c74 to 0x137100.
  Established. Six separate halfword loads read memory at [0x31e4dc] (`ldrh` at 0x136cc0, `ldrh` at 0x136ce4, `ldrh` at 0x136d10, `ldrh` at 0x136d3c, `ldrh` at 0x136d68, `ldrh` at 0x136d94).
  Word stores at entry sp minus 0x14 successively write constants 0x38, 0x28, 0x38 and 0x30 (`str` at 0x136ed0, `str` at 0x136f54, `str` at 0x136fd8, `str` at 0x13705c).
  The corresponding halfword stores to memory at [0x31e4dc] have unknown values (`strh` at 0x136f4c, `strh` at 0x136fd0, `strh` at 0x137054, `strh` at 0x1370d8).
  The later word store writes 1 at entry sp minus 0x18 (`str` at 0x1370e0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x136c84 | load | 4 | `abs 0x1370f0` | `-` |
| `ldr` at 0x136c88 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x136c94 | load | 4 | `abs 0x1370f4` | `-` |
| `ldr` at 0x136c9c | load | 4 | `abs 0x1370f8` | `-` |
| `ldr` at 0x136ca0 | load | 4 | `abs 0x315030` | `-` |
| `str` at 0x136cb0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x136cb8 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136cbc | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x136cc0 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x136cdc | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136ce0 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x136ce4 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x136d08 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136d0c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x136d10 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x136d34 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136d38 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x136d3c | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x136d60 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136d64 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x136d68 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x136d8c | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136d90 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x136d94 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `str` at 0x136db8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x136dbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x136dc4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x136dc8 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136dcc | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x136dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136df0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136e04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136e18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136e2c | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x136e40 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x136e44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x136e4c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x136e50 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136e54 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x136e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136e78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136ea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136eb4 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x136ec8 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x136ed0 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0x136ed4 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136ed8 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x136edc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136ee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136efc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136f10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136f38 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x136f4c | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x136f54 | store | 4 | `sp -0x14` | `0x28` |
| `ldr` at 0x136f58 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136f5c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x136f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136f6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136f80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136f94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136fbc | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x136fd0 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x136fd8 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0x136fdc | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x136fe0 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x136fe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136ff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137018 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13702c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137040 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x137054 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x13705c | store | 4 | `sp -0x14` | `0x30` |
| `ldr` at 0x137060 | load | 4 | `abs 0x1370fc` | `-` |
| `ldr` at 0x137064 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x137068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13709c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1370b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1370c4 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0x1370d8 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x1370e0 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x1370e4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1370ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1370ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1370ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x136c98 | `(unknown callee) 0x243d4` | `0x190fac` | `r1` | `r2` | `[0x315034]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x136c90 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bne` at 0x136ca8 | `[0x315030] != 0x0` | `[0x315030] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1370ec | `r0` | `?` |

## `sn1sc_dr_load`

- `sn1sc_dr_load`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x136758 to 0x136a38.
  Established. The word loaded from memory at entry r0 plus 0x10 is stored at entry sp minus 0x20 (`ldr` at 0x136778, `str` at 0x13677c).
  A word store writes entry r0 plus 24 as its value at entry sp minus 0x28 (`str` at 0x136794).
  The result from unnamed 0x12d2b8 is stored as a word at entry sp minus 0x18 (`bl` at 0x1367f4, `str` at 0x1367fc).
  A distinct result from unnamed 0x201fc is stored as a word at entry sp minus 0x2c (`bl` at 0x136988, `str` at 0x136990).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x136768 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x136770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136778 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0x13677c | store | 4 | `sp -0x20` | `[r0+16]` |
| `ldr` at 0x136780 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13678c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x136794 | store | 4 | `sp -0x28` | `r0+24` |
| `ldr` at 0x13679c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1367a4 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0x1367b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1367bc | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0x1367c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1367cc | store | 4 | `sp -0x44` | `[r0+16]` |
| `ldr` at 0x1367d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1367d4 | store | 4 | `sp -0x40` | `r0+24` |
| `ldr` at 0x1367d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1367dc | store | 4 | `sp -0x3c` | `r0+24` |
| `ldr` at 0x1367e0 | load | 4 | `abs 0x136a34` | `-` |
| `str` at 0x1367e4 | store | 4 | `sp -0x38` | `0x31e4ec` |
| `str` at 0x1367fc | store | 4 | `sp -0x18` | `ret(0x12d2b8)` |
| `ldr` at 0x136800 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x136810 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x13681c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136824 | load | 4 | `(unknown)` | `-` |
| `str` at 0x136828 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x13682c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13683c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x136844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x136848 | load | 4 | `(unknown)` | `-` |
| `str` at 0x136860 | store | 4 | `sp -0x1c` | `0x6` |
| `str` at 0x13686c | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x136870 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x136880 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x136888 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x13688c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x136894 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x13689c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1368a0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1368a8 | store | 4 | `sp -0x34` | `[sp-52]+24` |
| `ldr` at 0x1368b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1368b8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1368bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1368c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1368cc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1368d0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1368d8 | store | 4 | `sp -0x34` | `[sp-52]+28` |
| `ldr` at 0x1368dc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1368e0 | store | 4 | `sp -0x28` | `[sp-52]` |
| `ldr` at 0x1368e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1368f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1368fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x136900 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x136904 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x136908 | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0x13690c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x136910 | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0x136914 | load | 4 | `abs 0x136a34` | `-` |
| `str` at 0x136918 | store | 4 | `sp -0x38` | `0x31e4ec` |
| `ldr` at 0x136924 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x136934 | store | 4 | `sp -0x18` | `ret(0x12d2b8)` |
| `ldr` at 0x136938 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13693c | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x136954 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x13695c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x136974 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x136990 | store | 4 | `sp -0x2c` | `ret(0x201fc)` |
| `ldr` at 0x136994 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1369a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1369a4 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x1369ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1369b4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x1369c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1369c4 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0x1369c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1369cc | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0x1369d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1369d4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x1369d8 | load | 4 | `abs 0x136a34` | `-` |
| `str` at 0x1369dc | store | 4 | `sp -0x38` | `0x31e4ec` |
| `str` at 0x1369f8 | store | 4 | `sp -0x18` | `ret(0x12d2b8)` |
| `ldr` at 0x1369fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x136a00 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x136a10 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x136a18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x136a20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x136a24 | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0x136a28 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x136a30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x136a30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x136a30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1367f4 | `(unknown callee) 0x12d2b8` | `0x0` | `?` | `0x1` | `?` |
| `bl` at 0x13692c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `[sp-28]` | `?` |
| `bl` at 0x136988 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x1369f0 | `(unknown callee) 0x12d2b8` | `0x1` | `0x6` | `0x1` | `?` |
| `bl` at 0x136a1c | `(unknown callee) 0x20514` | `[sp-44]` | `?` | `[sp-44]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x136788 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bne` at 0x136808 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x136834 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x136858 | `? != 0x1` | `? == 0x1` |
| `bne` at 0x13687c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x13694c | `? == 0x1` | `? != 0x1` |
| `bne` at 0x13699c | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0x136a08 | `[sp-44] != [sp-40]` | `[sp-44] == [sp-40]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x136a30 | `r0` | `?` |

## `sn1sc_get_tap_text`

- `sn1sc_get_tap_text`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x137224 to 0x137458.
  Established. Two separate halfword loads read memory at [0x31e4dc] (`ldrh` at 0x137240, `ldrh` at 0x137264).
  The computed word stored at entry sp minus 0x18 has an unknown value (`str` at 0x137338).
  Separate paths write word constants 0x31 and 0x30 at entry sp minus 0x1c (`str` at 0x137350, `str` at 0x13735c).
  A call to unnamed 0x205bc has printed r0-r3 candidates entry r0, 0x190fc0, [sp-28] and [sp-32] (`bl` at 0x137444).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x137234 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x137238 | load | 4 | `abs 0x137450` | `-` |
| `ldr` at 0x13723c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137240 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x13725c | load | 4 | `abs 0x137450` | `-` |
| `ldr` at 0x137260 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137264 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x137288 | load | 4 | `abs 0x137450` | `-` |
| `ldr` at 0x13728c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137290 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1372b4 | load | 4 | `abs 0x137450` | `-` |
| `ldr` at 0x1372b8 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1372bc | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1372e0 | load | 4 | `abs 0x137450` | `-` |
| `ldr` at 0x1372e4 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1372e8 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x13730c | load | 4 | `abs 0x137450` | `-` |
| `ldr` at 0x137310 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137314 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `str` at 0x137338 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x13733c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137350 | store | 4 | `sp -0x1c` | `0x31` |
| `str` at 0x13735c | store | 4 | `sp -0x1c` | `0x30` |
| `ldr` at 0x137360 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137374 | store | 4 | `sp -0x20` | `0x31` |
| `str` at 0x137380 | store | 4 | `sp -0x20` | `0x30` |
| `ldr` at 0x137384 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137398 | store | 4 | `sp -0x24` | `0x31` |
| `str` at 0x1373a4 | store | 4 | `sp -0x24` | `0x30` |
| `ldr` at 0x1373a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1373ac | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0x1373b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1373c4 | store | 4 | `sp -0x28` | `0x31` |
| `str` at 0x1373d0 | store | 4 | `sp -0x28` | `0x30` |
| `ldr` at 0x1373d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1373d8 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldr` at 0x1373dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1373f0 | store | 4 | `sp -0x2c` | `0x31` |
| `str` at 0x1373fc | store | 4 | `sp -0x2c` | `0x30` |
| `ldr` at 0x137400 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x137404 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0x137408 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x13741c | store | 4 | `sp -0x30` | `0x31` |
| `str` at 0x137428 | store | 4 | `sp -0x30` | `0x30` |
| `ldr` at 0x13742c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x137430 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0x137434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137438 | load | 4 | `abs 0x137454` | `-` |
| `ldr` at 0x13743c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x137440 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x13744c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13744c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13744c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x137444 | `(unknown callee) 0x205bc` | `r0` | `0x190fc0` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x137348 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x13736c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x137390 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1373bc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x1373e8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x137414 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13744c | `r0` | `ret(0x205bc)` |

## `sn1sc_drive_tap_fixed_tms`

- `sn1sc_drive_tap_fixed_tms`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1374f0 to 0x137758.
  Established. The halfword store to memory at [0x31e4dc] has an unknown value (`strh` at 0x1376e8).
  Calls to unnamed 0x14b574 and 0x14b720 each have printed r0-r3 candidates entry sp minus 52, unknown, unknown and entry sp minus 52 (`bl` at 0x13772c, `bl` at 0x13773c).
  A later word store writes 1 at entry sp minus 0x3c (`str` at 0x137744).
  The recorded return value in r0 is 1 (`ldm` at 0x137750).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x137500 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x137504 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x137508 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x13750c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x137510 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137520 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x137528 | load | 4 | `abs 0x137754` | `-` |
| `ldr` at 0x13752c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137530 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x13754c | load | 4 | `abs 0x137754` | `-` |
| `ldr` at 0x137550 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137554 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x137578 | load | 4 | `abs 0x137754` | `-` |
| `ldr` at 0x13757c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137580 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1375a4 | load | 4 | `abs 0x137754` | `-` |
| `ldr` at 0x1375a8 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1375ac | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1375d0 | load | 4 | `abs 0x137754` | `-` |
| `ldr` at 0x1375d4 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1375d8 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1375fc | load | 4 | `abs 0x137754` | `-` |
| `ldr` at 0x137600 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137604 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `str` at 0x137628 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x13762c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x137634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137644 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137654 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x13765c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x137664 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x13766c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x137670 | load | 4 | `abs 0x137754` | `-` |
| `ldr` at 0x137674 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x137678 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x137684 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x137698 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1376ac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1376c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1376d4 | load | 4 | `sp -0x38` | `-` |
| `strh` at 0x1376e8 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x1376ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1376f0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1376f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1376f8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x1376fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x137708 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13770c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x137710 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x137714 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x137718 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x137744 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x137748 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x137750 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x137750 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x137750 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13772c | `(unknown callee) 0x14b574` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0x13773c | `(unknown callee) 0x14b720` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x137518 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x137640 | `? == ?` | `? != ?` |
| `beq` at 0x137650 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x137704 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x137720 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x137750 | `r0` | `0x1` |

## `sn1sc_drive_tap`

- `sn1sc_drive_tap`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x137758 to 0x137984.
  Established. A halfword store to memory at [0x31e4dc] has an unknown value (`strh` at 0x137938).
  The word loaded from entry sp is stored at entry sp minus 0x28 (`ldr` at 0x137954, `str` at 0x137958).
  The call to unnamed 0x14b7a0 has printed r0-r3 candidates entry sp minus 52, unknown, unknown and entry sp minus 52 (`bl` at 0x13796c).
  The return record places 1 in r0 (`ldm` at 0x13797c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x137768 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x13776c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x137770 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x137774 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x137778 | load | 4 | `abs 0x137980` | `-` |
| `ldr` at 0x13777c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137780 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x13779c | load | 4 | `abs 0x137980` | `-` |
| `ldr` at 0x1377a0 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1377a4 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1377c8 | load | 4 | `abs 0x137980` | `-` |
| `ldr` at 0x1377cc | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1377d0 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x1377f4 | load | 4 | `abs 0x137980` | `-` |
| `ldr` at 0x1377f8 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x1377fc | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x137820 | load | 4 | `abs 0x137980` | `-` |
| `ldr` at 0x137824 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137828 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `ldr` at 0x13784c | load | 4 | `abs 0x137980` | `-` |
| `ldr` at 0x137850 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldrh` at 0x137854 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `str` at 0x137878 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x13787c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x137884 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1378a4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1378ac | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1378b4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1378bc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1378c0 | load | 4 | `abs 0x137980` | `-` |
| `ldr` at 0x1378c4 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x1378c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1378d4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1378e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1378fc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x137910 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x137924 | load | 4 | `sp -0x38` | `-` |
| `strh` at 0x137938 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x13793c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x137940 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x137944 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137948 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x13794c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x137950 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x137954 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x137958 | store | 4 | `sp -0x28` | `[sp]` |
| `ldr` at 0x13795c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x137960 | store | 4 | `sp -0x24` | `?` |
| `ldm` at 0x13797c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13797c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13797c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13796c | `(unknown callee) 0x14b7a0` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x137890 | `? == ?` | `? != ?` |
| `beq` at 0x1378a0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13797c | `r0` | `0x1` |

## `sn1sc_asm_drive_tap_fixed_tms`

- `sn1sc_asm_drive_tap_fixed_tms`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14b574 to 0x14b720.
  Established. The word loaded from memory at entry r0 plus 4 is stored at entry sp minus 0x28 (`ldr` at 0x14b588, `str` at 0x14b58c).
  A computed word load retains an unknown effective address (`ldr` at 0x14b5fc).
  Halfword stores to memory at [0x31e4dc] have unknown values (`strh` at 0x14b624, `strh` at 0x14b640, `strh` at 0x14b658).
  A computed word store later retains both an unknown effective address and an unknown value (`str` at 0x14b710).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x14b580 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x14b588 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0x14b58c | store | 4 | `sp -0x28` | `[r0+4]` |
| `ldr` at 0x14b590 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0x14b594 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0x14b5ec | load | 4 | `abs 0x14b71c` | `-` |
| `ldr` at 0x14b5f0 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x14b5fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x14b600 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x14b618 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b620 | store | 4 | `sp -0x24` | `?` |
| `strh` at 0x14b624 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b62c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b634 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0x14b638 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `strh` at 0x14b640 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b64c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b654 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0x14b658 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x14b668 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x14b670 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x14b67c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x14b688 | load | 4 | `abs 0x14b71c` | `-` |
| `ldr` at 0x14b68c | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x14b690 | load | 4 | `(unknown)` | `-` |
| `str` at 0x14b6a0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x14b6c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b6c8 | store | 4 | `sp -0x24` | `?` |
| `strh` at 0x14b6cc | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b6d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b6dc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0x14b6e0 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `strh` at 0x14b6e8 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b6f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b6fc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0x14b700 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x14b710 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x14b5e0 | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0x14b664 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0x14b678 | `C == 0` | `not (C == 0)` |
| `beq` at 0x14b684 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x14b6b0 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0x14b70c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0x14b718 | `r0` | `?` |

## `sn1sc_asm_drive_tck`

- `sn1sc_asm_drive_tck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14b720 to 0x14b7a0.
  Established. A word store saves entry lr at entry sp minus 4 (`str` at 0x14b720).
  The ldm reads words from memory at entry r0 and entry r0 plus 4 (`ldm` at 0x14b724).
  Two halfword stores to memory at [0x31e4dc] have unknown values (`strh` at 0x14b77c, `strh` at 0x14b788).
  The conditional return records r0 as 0 when unsigned 0 is at least [r0+4], while the later return records an unknown value (`pophs` at 0x14b770, `pop` at 0x14b798).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x14b720 | store | 4 | `sp -0x4` | `lr` |
| `ldm` at 0x14b724 | load | 4 | `r0 +0x0` | `-` |
| `ldm` at 0x14b724 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x14b774 | load | 4 | `abs 0x14b79c` | `-` |
| `ldr` at 0x14b778 | load | 4 | `abs 0x31e4dc` | `-` |
| `strh` at 0x14b77c | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b784 | load | 4 | `abs 0x31e4dc` | `-` |
| `strh` at 0x14b788 | store | 2 | `[0x31e4dc] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `pophs` at 0x14b770 | `0x0 >= [r0+4] (unsigned, 32-bit)` | `0x0 < [r0+4] (unsigned, 32-bit)` |
| `blo` at 0x14b794 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `pophs` at 0x14b770 | `r0` | `0x0` |
| `pop` at 0x14b798 | `r0` | `?` |

## `sn1sc_asm_drive_tap`

- `sn1sc_asm_drive_tap`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x14b7a0 to 0x14b960.
  Established. The word loaded from memory at entry r0 plus 4 is stored at entry sp minus 0x2c (`ldr` at 0x14b7b4, `str` at 0x14b7b8).
  One computed word store targets memory at [sp-48] with an unknown value (`str` at 0x14b8a4).
  Later halfword stores to memory at [0x31e4dc] also have unknown values (`strh` at 0x14b90c, `strh` at 0x14b928, `strh` at 0x14b940).
  The final computed word store retains both an unknown effective address and an unknown value (`str` at 0x14b950).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x14b7ac | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x14b7b4 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0x14b7b8 | store | 4 | `sp -0x2c` | `[r0+4]` |
| `ldr` at 0x14b7bc | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0x14b7c0 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0x14b7c4 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0x14b808 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x14b820 | load | 4 | `abs 0x14b95c` | `-` |
| `ldr` at 0x14b824 | load | 4 | `abs 0x31e4dc` | `-` |
| `str` at 0x14b828 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x14b830 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x14b834 | load | 4 | `(unknown)` | `-` |
| `str` at 0x14b838 | store | 4 | `sp -0x24` | `0x0` |
| `strh` at 0x14b85c | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b864 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b86c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0x14b870 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `strh` at 0x14b878 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b884 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b88c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0x14b890 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b8a0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x14b8a4 | store | 4 | `[sp-48] +0x0` | `?` |
| `ldr` at 0x14b8ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x14b8b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x14b8c4 | load | 4 | `abs 0x14b95c` | `-` |
| `ldr` at 0x14b8c8 | load | 4 | `abs 0x31e4dc` | `-` |
| `ldr` at 0x14b8cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x14b8d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x14b8d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x14b8e0 | store | 4 | `sp -0x24` | `0x0` |
| `strh` at 0x14b90c | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b914 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b91c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0x14b920 | load | 2 | `[0x31e4dc] +0x0` | `-` |
| `strh` at 0x14b928 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `ldr` at 0x14b934 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14b93c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0x14b940 | store | 2 | `[0x31e4dc] +0x0` | `?` |
| `str` at 0x14b950 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x14b814 | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0x14b89c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0x14b8b4 | `C == 0` | `not (C == 0)` |
| `beq` at 0x14b8c0 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x14b8f0 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0x14b94c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0x14b958 | `r0` | `?` |

## `JTAG_Comm_read`

- `JTAG_Comm_read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139398 to 0x139500.
  Established. The result from unnamed 0x13d9cc is stored as a word at entry sp minus 0x20 (`bl` at 0x1393fc, `str` at 0x139404).
  One byte store has an unknown effective address and value (`strb` at 0x13946c).
  Another byte store targets memory at [sp-24] plus [sp-28], with an unknown value (`strb` at 0x1394b0).
  A later word store writes 8 at entry sp minus 0x30 (`str` at 0x1394ec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1393a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1393ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x1393b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1393c8 | store | 4 | `sp -0x2c` | `0xc` |
| `str` at 0x1393d4 | store | 4 | `sp -0x2c` | `0x11` |
| `str` at 0x1393e4 | store | 4 | `sp -0x38` | `0x40` |
| `ldr` at 0x1393e8 | load | 4 | `abs 0x1394fc` | `-` |
| `str` at 0x1393ec | store | 4 | `sp -0x34` | `0x1913a8` |
| `ldr` at 0x1393f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1393f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x139404 | store | 4 | `sp -0x20` | `ret(0x13d9cc)` |
| `ldr` at 0x139408 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139414 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x139418 | store | 4 | `sp -0x30` | `ret(0x13d9cc)` |
| `str` at 0x139438 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x13943c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13944c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139450 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139468 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x13946c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x139488 | load | 4 | `(unknown)` | `-` |
| `str` at 0x139490 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139494 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139498 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1394ac | load | 4 | `(unknown)` | `-` |
| `strb` at 0x1394b0 | store | 1 | `([sp-24] + [sp-28]) +0x0` | `?` |
| `ldr` at 0x1394cc | load | 4 | `(unknown)` | `-` |
| `str` at 0x1394d4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1394d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1394e0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x1394ec | store | 4 | `sp -0x30` | `0x8` |
| `ldr` at 0x1394f0 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x1394f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1394f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1394f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1393fc | `(unknown callee) 0x13d9cc` | `r0` | `[sp-44]` | `sp-40` | `sp-40` |
| `bl` at 0x139430 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-40` | `sp-40` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1393c0 | `? != 0x6` | `? == 0x6` |
| `beq` at 0x139410 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `bgt` at 0x139444 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1394f8 | `r0` | `?` |

## `JTAG_Comm_write`

- `JTAG_Comm_write`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139500 to 0x139664.
  Established. The stm writes two zero words to unknown effective targets (`stm` at 0x13952c).
  Separate byte loads read memory at [sp-24] plus [sp-28] plus 4 and at [sp-24] plus [sp-28] (`ldrb` at 0x1395a0, `ldrb` at 0x1395d8).
  The result from unnamed 0x13d9cc is stored as a word at entry sp minus 0x20 (`bl` at 0x139628, `str` at 0x139630).
  A later word store writes 8 at entry sp minus 0x30 (`str` at 0x139650).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139510 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x139514 | store | 4 | `sp -0x18` | `r1` |
| `stm` at 0x13952c | store | 4 | `(unknown)` | `0x0` |
| `stm` at 0x13952c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x139530 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x139548 | store | 4 | `sp -0x2c` | `0xd` |
| `str` at 0x139554 | store | 4 | `sp -0x2c` | `0x12` |
| `str` at 0x13955c | store | 4 | `sp -0x1c` | `0x3` |
| `ldr` at 0x139560 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139584 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139590 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139594 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1395a0 | load | 1 | `([sp-24] + [sp-28]) +0x4` | `-` |
| `str` at 0x1395a8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1395c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1395cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1395d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1395d8 | load | 1 | `([sp-24] + [sp-28]) +0x0` | `-` |
| `str` at 0x1395e0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1395e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1395ec | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `str` at 0x139610 | store | 4 | `sp -0x38` | `0x40` |
| `ldr` at 0x139614 | load | 4 | `abs 0x139660` | `-` |
| `str` at 0x139618 | store | 4 | `sp -0x34` | `0x1913b8` |
| `ldr` at 0x13961c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x139620 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x139630 | store | 4 | `sp -0x20` | `ret(0x13d9cc)` |
| `ldr` at 0x139634 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139640 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x139644 | store | 4 | `sp -0x30` | `ret(0x13d9cc)` |
| `str` at 0x139650 | store | 4 | `sp -0x30` | `0x8` |
| `ldr` at 0x139654 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x13965c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13965c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13965c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x139604 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-40` | `sp-40` |
| `bl` at 0x139628 | `(unknown callee) 0x13d9cc` | `[sp-20]` | `[sp-44]` | `sp-40` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x139540 | `? != 0x6` | `? == 0x6` |
| `blt` at 0x139568 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x13963c | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13965c | `r0` | `[sp-48]` |

## `JTAG_UART_read`

- `JTAG_UART_read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139664 to 0x139954.
  Established. A word store writes 0x48 at entry sp minus 0x38 (`str` at 0x139688).
  The result from unnamed 0x13d9cc is stored as a word at entry sp minus 0x20 (`bl` at 0x1396a0, `str` at 0x1396a8).
  Computed word stores retain unknown effective targets and values (`str` at 0x139744, `str` at 0x1397b4, `str` at 0x139824, `str` at 0x139894, `str` at 0x139904).
  The call to unnamed 0x203f4 has printed r0-r3 candidates [sp-24], entry sp minus 44, 9 and entry sp minus 44 (`bl` at 0x139928).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139674 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x139678 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x139688 | store | 4 | `sp -0x38` | `0x48` |
| `ldr` at 0x13968c | load | 4 | `abs 0x139944` | `-` |
| `str` at 0x139690 | store | 4 | `sp -0x34` | `0x1913c8` |
| `ldr` at 0x139694 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1396a8 | store | 4 | `sp -0x20` | `ret(0x13d9cc)` |
| `ldr` at 0x1396ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1396b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1396bc | store | 4 | `sp -0x30` | `ret(0x13d9cc)` |
| `str` at 0x1396c8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x1396cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1396d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1396f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139708 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x13970c | load | 4 | `abs 0x139948` | `-` |
| `ldr` at 0x139718 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139730 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139738 | load | 4 | `abs 0x139948` | `-` |
| `str` at 0x139744 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139748 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139760 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139778 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x13977c | load | 4 | `abs 0x13994c` | `-` |
| `ldr` at 0x139788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1397a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1397a8 | load | 4 | `abs 0x13994c` | `-` |
| `str` at 0x1397b4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1397b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1397d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1397e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1397ec | load | 4 | `abs 0x139950` | `-` |
| `ldr` at 0x1397f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139810 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139818 | load | 4 | `abs 0x139950` | `-` |
| `str` at 0x139824 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139858 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139868 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x139880 | load | 4 | `(unknown)` | `-` |
| `str` at 0x139894 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139898 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1398b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1398c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1398d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1398f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x139904 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139908 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x139910 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x13991c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13992c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x139934 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x139938 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x139940 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x139940 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x139940 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1396a0 | `(unknown callee) 0x13d9cc` | `r0` | `0x1e` | `sp-44` | `sp-44` |
| `bl` at 0x139928 | `(unknown callee) 0x203f4` | `[sp-24]` | `sp-44` | `0x9` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1396b4 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `bgt` at 0x1396d4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x139940 | `r0` | `?` |

## `JTAG_UART_write`

- `JTAG_UART_write`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139954 to 0x139d14.
  Established. Four word stores write 0 at entry sp minus 0x34, 0x30, 0x2c and 0x28 (`str` at 0x139978, `str` at 0x139984, `str` at 0x139990, `str` at 0x13999c).
  One load from entry sp minus 0x1c selects the branch to 0x1399bc when entry r2 is at most zero signed (`ldr` at 0x1399a0, `ble` at 0x1399a8).
  A separate load selects that destination when entry r2 is greater than 0x10 signed (`ldr` at 0x1399ac, `bgt` at 0x1399b4).
  A later word store writes 0 to an unknown effective target (`str` at 0x139a28).
  The result from unnamed 0x13d9cc is stored as a word at entry sp minus 0x24 (`bl` at 0x139cc4, `str` at 0x139ccc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139964 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x139968 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x13996c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x139978 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x139984 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x139990 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x13999c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x1399a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1399ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1399bc | load | 4 | `abs 0x139cfc` | `-` |
| `str` at 0x1399c0 | store | 4 | `sp -0x44` | `0x1913d4` |
| `ldr` at 0x1399c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1399c8 | store | 4 | `sp -0x40` | `r2` |
| `ldr` at 0x1399d0 | load | 4 | `abs 0x139d00` | `-` |
| `str` at 0x1399e4 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x1399ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1399f4 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x1399fc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x139a00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139a0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x139a28 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x139a2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x139a34 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x139a44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x139a48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x139a54 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x139a58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139a64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139a7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139a94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139a98 | load | 4 | `abs 0x139d04` | `-` |
| `ldr` at 0x139aa4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139abc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139ac4 | load | 4 | `abs 0x139d04` | `-` |
| `str` at 0x139ad0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139ad4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139aec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139b04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139b08 | load | 4 | `abs 0x139d08` | `-` |
| `ldr` at 0x139b14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139b2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139b34 | load | 4 | `abs 0x139d08` | `-` |
| `str` at 0x139b40 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139b44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139b5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139b74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139b78 | load | 4 | `abs 0x139d0c` | `-` |
| `ldr` at 0x139b84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139b9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139ba4 | load | 4 | `abs 0x139d0c` | `-` |
| `str` at 0x139bb0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139bb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139bcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139be4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139bf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139c0c | load | 4 | `(unknown)` | `-` |
| `str` at 0x139c20 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139c24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139c3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139c54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x139c64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x139c7c | load | 4 | `(unknown)` | `-` |
| `str` at 0x139c90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x139c94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x139c9c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x139ca8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x139cac | store | 4 | `sp -0x44` | `[sp-56]` |
| `ldr` at 0x139cb0 | load | 4 | `abs 0x139d10` | `-` |
| `str` at 0x139cb4 | store | 4 | `sp -0x40` | `0x191414` |
| `ldr` at 0x139cb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x139ccc | store | 4 | `sp -0x24` | `ret(0x13d9cc)` |
| `ldr` at 0x139cd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x139cdc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x139ce0 | store | 4 | `sp -0x3c` | `ret(0x13d9cc)` |
| `ldr` at 0x139ce8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x139cec | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0x139cf0 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x139cf8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x139cf8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x139cf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1399dc | `(unknown callee) 0x22070` | `0x1` | `0x1913f8` | `0xca` | `0x0` |
| `bl` at 0x139a4c | `(unknown callee) 0x203f4` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0x139cc4 | `(unknown callee) 0x13d9cc` | `[sp-20]` | `0x1d` | `sp-52` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x1399a8 | `r2 <= 0x0 (signed, 32-bit)` | `r2 > 0x0 (signed, 32-bit)` |
| `bgt` at 0x1399b4 | `r2 > 0x10 (signed, 32-bit)` | `r2 <= 0x10 (signed, 32-bit)` |
| `bgt` at 0x139a08 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0x139a60 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x139cd8 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x139cf8 | `r0` | `?` |
