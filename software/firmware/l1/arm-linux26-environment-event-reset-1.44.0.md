# ARM Linux 2.6 environment events and reset paths in 1.44.0

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
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers in EnvReset and EnvTestEvent leave unresolved edges. Their tables cover
known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-environment-event-reset-facts.txt) retain all tool sections.

## `EvtIRouter`

- `EvtIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x956cc to 0x957c4.
  Established. A word loaded from memory at entry r0 plus 0x1c is stored at entry sp minus 0x18 (`ldr` at 0x956e4, `str` at 0x956e8).
  The routine separately loads a byte at entry r0 plus 0x16 and a halfword at entry r0 plus 0x22 (`ldrb` at 0x95750, `ldrh` at 0x95760).
  A byte store writes 2 to memory at entry r0 plus 0x16 (`strb` at 0x95778).
  Calls to unnamed 0xa1cec and 0xbb4f8 each have printed r0-r3 candidates [sp-20], entry r1, 2 and [sp-28] (`bl` at 0x957a4, `bl` at 0x957b0).
  What the copied fields represent remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x956dc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x956e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x956e4 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x956e8 | store | 4 | `sp -0x18` | `[r0+28]` |
| `ldr` at 0x956ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x956f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x95700 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x95704 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9570c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x95714 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9571c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x95720 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x95724 | store | 4 | `sp -0x1c` | `[sp-40]` |
| `ldr` at 0x95728 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x95738 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x9573c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x95748 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9574c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x95750 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x9575c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x95760 | load | 2 | `r0 +0x22` | `-` |
| `ldr` at 0x95770 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x95778 | store | 1 | `r0 +0x16` | `0x2` |
| `ldr` at 0x9577c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x95780 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x95784 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x95790 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x957a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x957ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x957b4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x957c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x957c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x957c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x957a4 | `(unknown callee) 0xa1cec` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x957b0 | `(unknown callee) 0xbb4f8` | `[sp-20]` | `r1` | `0x2` | `[sp-28]` |
| `bl` at 0x957b8 | `(unknown callee) 0xfb794` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x956f4 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x95758 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `beq` at 0x95768 | `[r0+34]:2 == 0x0` | `[r0+34]:2 != 0x0` |
| `beq` at 0x9578c | `[sp-28] == 0x4` | `[sp-28] != 0x4` |
| `beq` at 0x95798 | `[sp-28] == 0xe` | `[sp-28] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x957c0 | `r0` | `ret(0xfb794)` |

## `EvtPrintf`

- `EvtPrintf`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x957c4 to 0x95800.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x957d4).
  The result of the call to unnamed 0x2031c is stored as a word at entry sp minus 0x18 (`bl` at 0x957dc, `str` at 0x957e4).
  A later call to unnamed 0x95800 has printed r0 and r1 candidates 0 and [sp-20] (`bl` at 0x957f4).
  Any runtime output behavior of this wrapper remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x957d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x957d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x957e4 | store | 4 | `sp -0x18` | `ret(0x2031c)` |
| `ldr` at 0x957ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x957f0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x957fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x957fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x957fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x957dc | `(unknown callee) 0x2031c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x957f4 | `(unknown callee) 0x95800` | `0x0` | `[sp-20]` | `ret(0x2031c)` | `ret(0x2031c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x957fc | `r0` | `ret(0x95800)` |

## `EnvEvent`

- `EnvEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9740 to 0xa9e44.
  Established. The low byte of the result from unnamed 0x6dc0c is stored at entry sp minus 0x55 (`bl` at 0xa9760, `strb` at 0xa9768).
  A call to unnamed 0x22ca0 has its result stored as a word at entry sp minus 0xf4 (`bl` at 0xa9774, `str` at 0xa977c).
  Two word stores to absolute address 0x1a6fe0 have unknown values in the resolved access records (`str` at 0xa9974, `str` at 0xa9a4c).
  After a call to unnamed 0xa3c7c, the word loaded from its result plus 0x5c is stored at entry sp minus 0xf8 (`bl` at 0xa9b50, `ldr` at 0xa9b58, `str` at 0xa9b5c).
  The purpose of the word at absolute address 0x1a6fe0 remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9750 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xa9754 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xa9758 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xa975c | store | 4 | `sp -0x28` | `r3` |
| `strb` at 0xa9768 | store | 1 | `sp -0x55` | `(ret(0x6dc0c) & 0xff)` |
| `ldr` at 0xa976c | load | 4 | `abs 0xa9e0c` | `-` |
| `str` at 0xa977c | store | 4 | `sp -0xf4` | `ret(0x22ca0)` |
| `ldr` at 0xa9780 | load | 4 | `sp -0xf4` | `-` |
| `ldr` at 0xa978c | load | 4 | `abs 0xa9e10` | `-` |
| `str` at 0xa9790 | store | 4 | `sp -0x104` | `0x16bd48` |
| `ldr` at 0xa9794 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0xa9798 | store | 4 | `sp -0x100` | `ret(0x22ca0)` |
| `ldr` at 0xa97a0 | load | 4 | `abs 0xa9e14` | `-` |
| `ldr` at 0xa97b0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa97bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa97cc | load | 4 | `abs 0xa9e18` | `-` |
| `ldrb` at 0xa9804 | load | 1 | `sp -0x55` | `-` |
| `ldr` at 0xa9824 | load | 4 | `abs 0xa9e1c` | `-` |
| `ldr` at 0xa9834 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9844 | load | 4 | `abs 0xa9e18` | `-` |
| `ldr` at 0xa984c | load | 4 | `abs 0xa9e20` | `-` |
| `ldr` at 0xa9850 | load | 4 | `abs 0x1a6fe4` | `-` |
| `ldr` at 0xa985c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa987c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9894 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa98c0 | store | 4 | `sp -0x104` | `0x1` |
| `str` at 0xa98fc | store | 4 | `sp -0x104` | `sp-80` |
| `str` at 0xa9904 | store | 4 | `sp -0x100` | `0x0` |
| `ldr` at 0xa9908 | load | 4 | `abs 0xa9e24` | `-` |
| `ldr` at 0xa9914 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa9920 | store | 4 | `sp -0xf4` | `ret(0xfd284)` |
| `ldr` at 0xa9924 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa9940 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9950 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa9960 | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9964 | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9968 | load | 4 | `abs 0x1a6fe0` | `-` |
| `ldr` at 0xa996c | load | 4 | `sp +0x4` | `-` |
| `str` at 0xa9974 | store | 4 | `abs 0x1a6fe0` | `?` |
| `ldr` at 0xa9978 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa9994 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa99a0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa99b0 | load | 4 | `abs 0xa9e2c` | `-` |
| `ldr` at 0xa99b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa99c0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa99d0 | load | 4 | `abs 0xa9e30` | `-` |
| `ldr` at 0xa99d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa99e0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa99f0 | load | 4 | `abs 0xa9e34` | `-` |
| `ldr` at 0xa99f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9a00 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa9a10 | load | 4 | `abs 0xa9e38` | `-` |
| `ldr` at 0xa9a14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9a20 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa9a30 | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9a34 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa9a40 | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9a44 | load | 4 | `abs 0x1a6fe0` | `-` |
| `str` at 0xa9a4c | store | 4 | `abs 0x1a6fe0` | `?` |
| `ldr` at 0xa9a54 | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9a58 | load | 4 | `abs 0x1a6fe0` | `-` |
| `ldr` at 0xa9a6c | load | 4 | `abs 0xa9e2c` | `-` |
| `ldr` at 0xa9a7c | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9a80 | load | 4 | `abs 0x1a6fe0` | `-` |
| `ldr` at 0xa9a94 | load | 4 | `abs 0xa9e30` | `-` |
| `ldr` at 0xa9aa4 | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9aa8 | load | 4 | `abs 0x1a6fe0` | `-` |
| `ldr` at 0xa9abc | load | 4 | `abs 0xa9e34` | `-` |
| `ldr` at 0xa9acc | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9ad0 | load | 4 | `abs 0x1a6fe0` | `-` |
| `ldr` at 0xa9ae4 | load | 4 | `abs 0xa9e38` | `-` |
| `ldr` at 0xa9b00 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9b0c | load | 4 | `abs 0xa9e0c` | `-` |
| `str` at 0xa9b18 | store | 4 | `sp -0xf4` | `ret(0x2302c)` |
| `ldr` at 0xa9b1c | load | 4 | `sp -0xf4` | `-` |
| `ldr` at 0xa9b28 | load | 4 | `abs 0xa9e3c` | `-` |
| `str` at 0xa9b2c | store | 4 | `sp -0x104` | `0x16bd88` |
| `ldr` at 0xa9b30 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0xa9b34 | store | 4 | `sp -0x100` | `ret(0x2302c)` |
| `ldr` at 0xa9b3c | load | 4 | `abs 0xa9e14` | `-` |
| `ldr` at 0xa9b40 | load | 4 | `abs 0xa9e40` | `-` |
| `ldr` at 0xa9b58 | load | 4 | `ret(0xa3c7c) +0x5c` | `-` |
| `str` at 0xa9b5c | store | 4 | `sp -0xf8` | `[ret(0xa3c7c)+92]` |
| `ldr` at 0xa9b60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9b68 | store | 4 | `sp -0xfc` | `?` |
| `ldr` at 0xa9b6c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9b78 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9b84 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9b90 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9b9c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9ba8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9bb4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9bc0 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9bd0 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9bdc | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9bec | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9bf8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c04 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c10 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c20 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c2c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c3c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c48 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c54 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c60 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c6c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c78 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c88 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9c94 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9ca4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9cb0 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9cbc | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9cc8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9cd8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9ce4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9cf0 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9d00 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0xa9d10 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d18 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d20 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d28 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d30 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d38 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d40 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d48 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d50 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d58 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d60 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d68 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d70 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d78 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d80 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d88 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9d90 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9d98 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9da0 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9da8 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9db0 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9db8 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9dc0 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9dc8 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9dd0 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9dd8 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9de0 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9de8 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9df0 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0xa9df8 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0xa9dfc | load | 4 | `sp -0xf8` | `-` |
| `ldm` at 0xa9e08 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa9e08 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa9e08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa9e08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa9e08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9760 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xa9774 | `(unknown callee) 0x22ca0` | `0x1a7258` | `?` | `?` | `ret(0x6dc0c)` |
| `bl` at 0xa97ac | `(unknown callee) 0x22070` | `0x9` | `0x16bd68` | `0xae` | `0x0` |
| `bl` at 0xa97c0 | `(unknown callee) 0x293f8` | `[sp-40]` | `?` | `?` | `[sp]` |
| `bl` at 0xa97d0 | `(unknown callee) 0x20634` | `sp-240` | `0x16bd7c` | `?` | `sp-240` |
| `bl` at 0xa97e0 | `(unknown callee) 0x2031c` | `sp-240` | `?` | `?` | `sp-240` |
| `bl` at 0xa97f0 | `(unknown callee) 0x28230` | `0x1` | `?` | `?` | `ret(0x2031c)` |
| `bl` at 0xa97fc | `(unknown callee) 0x28230` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xa9818 | `(unknown callee) 0x96878` | `(sp-240 + ret(0x2031c))` | `ret(0x28230)` | `[sp-85]:1` | `ret(0x28230)` |
| `bl` at 0xa9828 | `(unknown callee) 0x203c4` | `sp-240` | `0x16bd80` | `?` | `sp-240` |
| `bl` at 0xa9838 | `(unknown callee) 0x203c4` | `sp-240` | `[sp-40]` | `?` | `sp-240` |
| `bl` at 0xa9848 | `(unknown callee) 0x203c4` | `sp-240` | `0x16bd7c` | `?` | `sp-240` |
| `bl` at 0xa9880 | `(unknown callee) 0x958d4` | `sp-84` | `sp-80` | `[sp-32]` | `sp-84` |
| `bl` at 0xa9898 | `(unknown callee) 0x95934` | `sp-84` | `sp-80` | `[sp-36]` | `sp-84` |
| `bl` at 0xa98a4 | `(unknown callee) 0x2031c` | `sp-240` | `?` | `?` | `sp-240` |
| `bl` at 0xa98d0 | `(unknown callee) 0x95b5c` | `sp-84` | `sp-80` | `sp-240` | `ret(0x2031c)` |
| `bl` at 0xa98d8 | `(unknown callee) 0x28230` | `0xe` | `?` | `?` | `?` |
| `bl` at 0xa98f4 | `(unknown callee) 0x95934` | `sp-84` | `sp-80` | `ret(0x28230)` | `sp-84` |
| `bl` at 0xa9918 | `(unknown callee) 0xfd284` | `0x1a7058` | `0x0` | `0x0` | `[sp-84]` |
| `bl` at 0xa9938 | `(unknown callee) 0x70bc4` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xa9948 | `(unknown callee) 0x706a0` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0xa998c | `(unknown callee) 0x70bc4` | `0x1` | `0x1a6fe0` | `[0x1a6fe0]` | `?` |
| `bl` at 0xa999c | `(unknown callee) 0x706a0` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0xa99b8 | `(unknown callee) 0x20634` | `0x1b0b48` | `[sp-40]` | `?` | `?` |
| `bl` at 0xa99d8 | `(unknown callee) 0x20634` | `0x1b0b98` | `[sp-40]` | `?` | `?` |
| `bl` at 0xa99f8 | `(unknown callee) 0x20634` | `0x1b0be8` | `[sp-40]` | `?` | `?` |
| `bl` at 0xa9a18 | `(unknown callee) 0x20634` | `0x1b0c38` | `[sp-40]` | `?` | `?` |
| `bl` at 0xa9a50 | `(unknown callee) 0x70b90` | `?` | `0x1a6fe0` | `?` | `?` |
| `bl` at 0xa9a74 | `(unknown callee) 0x706a0` | `0x32` | `0x1b0b48` | `0x2` | `?` |
| `bl` at 0xa9a9c | `(unknown callee) 0x706a0` | `0x32` | `0x1b0b98` | `0x2` | `?` |
| `bl` at 0xa9ac4 | `(unknown callee) 0x706a0` | `0x32` | `0x1b0be8` | `0x2` | `?` |
| `bl` at 0xa9aec | `(unknown callee) 0x706a0` | `0x32` | `0x1b0c38` | `0x2` | `?` |
| `bl` at 0xa9af8 | `(unknown callee) 0x70cf0` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xa9b08 | `(unknown callee) 0x706a0` | `0x32` | `[sp-40]` | `0x1` | `?` |
| `bl` at 0xa9b10 | `(unknown callee) 0x2302c` | `0x1a7258` | `?` | `?` | `?` |
| `bl` at 0xa9b48 | `(unknown callee) 0x22070` | `0x9` | `0x16bd68` | `0x117` | `0x0` |
| `bl` at 0xa9b50 | `(unknown callee) 0xa3c7c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa9e00 | `(unknown callee) 0xa4910` | `[sp-248]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa9788 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0xa97b8 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0xa9858 | `[0x1a6fe4] == 0x0` | `[0x1a6fe4] != 0x0` |
| `bgt` at 0xa9864 | `[sp-28] > 0x1 (signed, 32-bit)` | `[sp-28] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xa9930 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa995c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9984 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa99ac | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa99cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa99ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9a0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9a2c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9a64 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9a8c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9ab4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9adc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9b24 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |
| `beq` at 0xa9b74 | `[sp-252] == 0x3300` | `[sp-252] != 0x3300` |
| `bhi` at 0xa9b80 | `[sp-252] > 0x3300 (unsigned, 32-bit)` | `[sp-252] <= 0x3300 (unsigned, 32-bit)` |
| `beq` at 0xa9b8c | `[sp-252] == 0x2000` | `[sp-252] != 0x2000` |
| `bhi` at 0xa9b98 | `[sp-252] > 0x2000 (unsigned, 32-bit)` | `[sp-252] <= 0x2000 (unsigned, 32-bit)` |
| `beq` at 0xa9ba4 | `[sp-252] == 0x1200` | `[sp-252] != 0x1200` |
| `bhi` at 0xa9bb0 | `[sp-252] > 0x1200 (unsigned, 32-bit)` | `[sp-252] <= 0x1200 (unsigned, 32-bit)` |
| `beq` at 0xa9bbc | `[sp-252] == 0x1000` | `[sp-252] != 0x1000` |
| `beq` at 0xa9bc8 | `[sp-252] == 0x1100` | `[sp-252] != 0x1100` |
| `beq` at 0xa9bd8 | `[sp-252] == 0x1300` | `[sp-252] != 0x1300` |
| `beq` at 0xa9be4 | `[sp-252] == 0x1400` | `[sp-252] != 0x1400` |
| `beq` at 0xa9bf4 | `[sp-252] == 0x3000` | `[sp-252] != 0x3000` |
| `bhi` at 0xa9c00 | `[sp-252] > 0x3000 (unsigned, 32-bit)` | `[sp-252] <= 0x3000 (unsigned, 32-bit)` |
| `beq` at 0xa9c0c | `[sp-252] == 0x2100` | `[sp-252] != 0x2100` |
| `beq` at 0xa9c18 | `[sp-252] == 0x2200` | `[sp-252] != 0x2200` |
| `beq` at 0xa9c28 | `[sp-252] == 0x3100` | `[sp-252] != 0x3100` |
| `beq` at 0xa9c34 | `[sp-252] == 0x3200` | `[sp-252] != 0x3200` |
| `beq` at 0xa9c44 | `[sp-252] == 0x6000` | `[sp-252] != 0x6000` |
| `bhi` at 0xa9c50 | `[sp-252] > 0x6000 (unsigned, 32-bit)` | `[sp-252] <= 0x6000 (unsigned, 32-bit)` |
| `beq` at 0xa9c5c | `[sp-252] == 0x4000` | `[sp-252] != 0x4000` |
| `bhi` at 0xa9c68 | `[sp-252] > 0x4000 (unsigned, 32-bit)` | `[sp-252] <= 0x4000 (unsigned, 32-bit)` |
| `beq` at 0xa9c74 | `[sp-252] == 0x3400` | `[sp-252] != 0x3400` |
| `beq` at 0xa9c80 | `[sp-252] == 0x3500` | `[sp-252] != 0x3500` |
| `beq` at 0xa9c90 | `[sp-252] == 0x4100` | `[sp-252] != 0x4100` |
| `beq` at 0xa9c9c | `[sp-252] == 0x5000` | `[sp-252] != 0x5000` |
| `beq` at 0xa9cac | `[sp-252] == 0x7200` | `[sp-252] != 0x7200` |
| `bhi` at 0xa9cb8 | `[sp-252] > 0x7200 (unsigned, 32-bit)` | `[sp-252] <= 0x7200 (unsigned, 32-bit)` |
| `beq` at 0xa9cc4 | `[sp-252] == 0x7000` | `[sp-252] != 0x7000` |
| `beq` at 0xa9cd0 | `[sp-252] == 0x7100` | `[sp-252] != 0x7100` |
| `beq` at 0xa9ce0 | `[sp-252] == 0x7400` | `[sp-252] != 0x7400` |
| `bhi` at 0xa9cec | `[sp-252] > 0x7400 (unsigned, 32-bit)` | `[sp-252] <= 0x7400 (unsigned, 32-bit)` |
| `beq` at 0xa9cf8 | `[sp-252] == 0x7300` | `[sp-252] != 0x7300` |
| `beq` at 0xa9d08 | `[sp-252] == 0x7500` | `[sp-252] != 0x7500` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa9e08 | `r0` | `ret(0xa4910)` |

## `EnvInit`

- `EnvInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9e44 to 0xaa230.
  Established. A word store writes 1 at entry sp minus 0x24 (`str` at 0xa9e58).
  After the call to unnamed 0xaa230, the branch to 0xa9e7c is taken when its result is at most 0x7ffff unsigned (`bl` at 0xa9e5c, `bls` at 0xa9e6c).
  That arm stores 0x5dc at entry sp minus 0x30, while the fallthrough stores 0x960 there (`str` at 0xa9e80, `str` at 0xa9e74).
  Word stores update absolute address 0x1a6fc0 with its loaded value plus 1 and write 0 to absolute address 0x1a6fc8 (`ldr` at 0xa9ef0, `add` at 0xa9ef4, `str` at 0xa9ef8, `str` at 0xa9f04).
  The result from unnamed 0x6e140 is stored as a word at absolute address 0x1b0b44 (`bl` at 0xaa160, `str` at 0xaa164).
  Whether 0x960 and 0x5dc have units remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9e58 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xa9e64 | load | 4 | `abs 0xaa178` | `-` |
| `str` at 0xa9e74 | store | 4 | `sp -0x30` | `0x960` |
| `ldr` at 0xa9e7c | load | 4 | `abs 0xaa17c` | `-` |
| `str` at 0xa9e80 | store | 4 | `sp -0x30` | `0x5dc` |
| `ldr` at 0xa9e84 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa9e88 | store | 4 | `sp -0x28` | `[sp-48]` |
| `ldr` at 0xa9e94 | load | 4 | `abs 0xaa178` | `-` |
| `str` at 0xa9ea4 | store | 4 | `sp -0x34` | `0x960` |
| `ldr` at 0xa9eac | load | 4 | `abs 0xaa17c` | `-` |
| `str` at 0xa9eb0 | store | 4 | `sp -0x34` | `0x5dc` |
| `ldr` at 0xa9eb4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa9eb8 | store | 4 | `sp -0x2c` | `[sp-52]` |
| `ldr` at 0xa9ebc | load | 4 | `abs 0xaa180` | `-` |
| `ldr` at 0xa9ec0 | load | 4 | `abs 0x1a6fc0` | `-` |
| `ldr` at 0xa9ecc | load | 4 | `abs 0xaa184` | `-` |
| `str` at 0xa9ed0 | store | 4 | `sp -0x48` | `0x16bdac` |
| `ldr` at 0xa9ed8 | load | 4 | `abs 0xaa188` | `-` |
| `ldr` at 0xa9edc | load | 4 | `abs 0xaa18c` | `-` |
| `ldr` at 0xa9ee8 | load | 4 | `abs 0xaa180` | `-` |
| `ldr` at 0xa9eec | load | 4 | `abs 0xaa180` | `-` |
| `ldr` at 0xa9ef0 | load | 4 | `abs 0x1a6fc0` | `-` |
| `str` at 0xa9ef8 | store | 4 | `abs 0x1a6fc0` | `[0x1a6fc0]+1` |
| `ldr` at 0xa9efc | load | 4 | `abs 0xaa190` | `-` |
| `str` at 0xa9f04 | store | 4 | `abs 0x1a6fc8` | `0x0` |
| `ldr` at 0xa9f08 | load | 4 | `abs 0xaa194` | `-` |
| `ldr` at 0xa9f18 | load | 4 | `abs 0xaa198` | `-` |
| `ldr` at 0xa9f1c | load | 4 | `abs 0xaa19c` | `-` |
| `ldr` at 0xa9f2c | load | 4 | `abs 0xaa1a0` | `-` |
| `ldr` at 0xa9f30 | load | 4 | `abs 0xaa1a4` | `-` |
| `ldr` at 0xa9f40 | load | 4 | `abs 0xaa1a8` | `-` |
| `ldr` at 0xa9f44 | load | 4 | `abs 0xaa1ac` | `-` |
| `ldr` at 0xa9f54 | load | 4 | `abs 0xaa1b0` | `-` |
| `ldr` at 0xa9f58 | load | 4 | `abs 0xaa1b4` | `-` |
| `ldr` at 0xa9f68 | load | 4 | `abs 0xaa1b8` | `-` |
| `ldr` at 0xa9f6c | load | 4 | `abs 0xaa1bc` | `-` |
| `ldr` at 0xa9f7c | load | 4 | `abs 0xaa1c0` | `-` |
| `ldr` at 0xa9f80 | load | 4 | `abs 0xaa1c4` | `-` |
| `ldr` at 0xa9f90 | load | 4 | `abs 0xaa1c8` | `-` |
| `ldr` at 0xa9f94 | load | 4 | `abs 0xaa1cc` | `-` |
| `ldr` at 0xa9fa4 | load | 4 | `abs 0xaa1d0` | `-` |
| `ldr` at 0xa9fa8 | load | 4 | `abs 0xaa1d4` | `-` |
| `str` at 0xa9fbc | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xa9fc0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa9fc4 | store | 4 | `sp -0x44` | `[sp-40]` |
| `str` at 0xa9fcc | store | 4 | `sp -0x40` | `0xa` |
| `str` at 0xa9fd4 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0xa9fdc | store | 4 | `sp -0x38` | `0x9` |
| `ldr` at 0xa9fe0 | load | 4 | `abs 0xaa1d8` | `-` |
| `ldr` at 0xa9fe4 | load | 4 | `abs 0xaa1dc` | `-` |
| `ldr` at 0xa9fe8 | load | 4 | `abs 0xaa1e0` | `-` |
| `ldr` at 0xa9fec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9ff8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xa9ffc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa000 | store | 4 | `sp -0x44` | `[sp-40]` |
| `str` at 0xaa008 | store | 4 | `sp -0x40` | `0xa` |
| `str` at 0xaa010 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0xaa018 | store | 4 | `sp -0x38` | `0x9` |
| `ldr` at 0xaa01c | load | 4 | `abs 0xaa1e4` | `-` |
| `ldr` at 0xaa020 | load | 4 | `abs 0xaa1e8` | `-` |
| `ldr` at 0xaa024 | load | 4 | `abs 0xaa1ec` | `-` |
| `ldr` at 0xaa028 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa034 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xaa038 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xaa03c | store | 4 | `sp -0x44` | `[sp-40]` |
| `str` at 0xaa044 | store | 4 | `sp -0x40` | `0xa` |
| `str` at 0xaa04c | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0xaa054 | store | 4 | `sp -0x38` | `0x9` |
| `ldr` at 0xaa058 | load | 4 | `abs 0xaa1f0` | `-` |
| `ldr` at 0xaa05c | load | 4 | `abs 0xaa1f4` | `-` |
| `ldr` at 0xaa060 | load | 4 | `abs 0xaa1f8` | `-` |
| `ldr` at 0xaa064 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa070 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xaa074 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa078 | store | 4 | `sp -0x44` | `[sp-44]` |
| `str` at 0xaa080 | store | 4 | `sp -0x40` | `0xa` |
| `str` at 0xaa088 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0xaa090 | store | 4 | `sp -0x38` | `0x9` |
| `ldr` at 0xaa094 | load | 4 | `abs 0xaa1fc` | `-` |
| `ldr` at 0xaa098 | load | 4 | `abs 0xaa200` | `-` |
| `ldr` at 0xaa09c | load | 4 | `abs 0xaa204` | `-` |
| `ldr` at 0xaa0a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa0ac | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xaa0b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa0b4 | store | 4 | `sp -0x44` | `[sp-44]` |
| `str` at 0xaa0bc | store | 4 | `sp -0x40` | `0xa` |
| `str` at 0xaa0c4 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0xaa0cc | store | 4 | `sp -0x38` | `0x9` |
| `ldr` at 0xaa0d0 | load | 4 | `abs 0xaa208` | `-` |
| `ldr` at 0xaa0d4 | load | 4 | `abs 0xaa20c` | `-` |
| `ldr` at 0xaa0d8 | load | 4 | `abs 0xaa210` | `-` |
| `ldr` at 0xaa0dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa0e8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xaa0ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa0f0 | store | 4 | `sp -0x44` | `[sp-44]` |
| `str` at 0xaa0f8 | store | 4 | `sp -0x40` | `0xa` |
| `str` at 0xaa100 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0xaa108 | store | 4 | `sp -0x38` | `0x9` |
| `ldr` at 0xaa10c | load | 4 | `abs 0xaa214` | `-` |
| `ldr` at 0xaa110 | load | 4 | `abs 0xaa218` | `-` |
| `ldr` at 0xaa114 | load | 4 | `abs 0xaa21c` | `-` |
| `ldr` at 0xaa118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa120 | load | 4 | `abs 0xaa220` | `-` |
| `ldr` at 0xaa124 | load | 4 | `abs 0x318464` | `-` |
| `ldr` at 0xaa130 | load | 4 | `abs 0xaa220` | `-` |
| `ldr` at 0xaa134 | load | 4 | `abs 0x318464` | `-` |
| `ldr` at 0xaa138 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa13c | load | 4 | `abs 0xaa224` | `-` |
| `ldr` at 0xaa148 | load | 4 | `abs 0xaa224` | `-` |
| `ldr` at 0xaa150 | load | 4 | `abs 0xaa228` | `-` |
| `ldr` at 0xaa158 | load | 4 | `abs 0xaa224` | `-` |
| `ldr` at 0xaa15c | load | 4 | `abs 0xaa22c` | `-` |
| `str` at 0xaa164 | store | 4 | `abs 0x1b0b44` | `ret(0x6e140)` |
| `ldm` at 0xaa174 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xaa174 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xaa174 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaa174 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaa174 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9e5c | `(unknown callee) 0xaa230` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0xa9e8c | `(unknown callee) 0xaa230` | `ret(0xaa230)` | `?` | `ret(0xaa230)` | `[sp-48]` |
| `bl` at 0xa9ee4 | `(unknown callee) 0x22070` | `0x9` | `0x16bd68` | `0x162` | `0x0` |
| `bl` at 0xa9f14 | `(unknown callee) 0xfb5cc` | `0x1a7058` | `0x4` | `0x0` | `0x0` |
| `bl` at 0xa9f28 | `(unknown callee) 0x22b84` | `0x1a7060` | `0x16bdc8` | `0x0` | `0x9` |
| `bl` at 0xa9f3c | `(unknown callee) 0x22b84` | `0x1a70b4` | `0x16bdd4` | `0x0` | `0x9` |
| `bl` at 0xa9f50 | `(unknown callee) 0x22b84` | `0x1a7108` | `0x16bde0` | `0x0` | `0x9` |
| `bl` at 0xa9f64 | `(unknown callee) 0x22b84` | `0x1a715c` | `0x16bdec` | `0x0` | `0x9` |
| `bl` at 0xa9f78 | `(unknown callee) 0x22b84` | `0x1a71b0` | `0x16bdf8` | `0x0` | `0x9` |
| `bl` at 0xa9f8c | `(unknown callee) 0x22b84` | `0x1a7204` | `0x16be04` | `0x0` | `0x9` |
| `bl` at 0xa9fa0 | `(unknown callee) 0x22b84` | `0x1a7258` | `0x16be10` | `0x1` | `0x9` |
| `bl` at 0xa9fb4 | `(unknown callee) 0x22b84` | `0x1a72ac` | `0x16be1c` | `0x1` | `0x9` |
| `bl` at 0xa9ff0 | `(unknown callee) 0x22540` | `0x1a7300` | `0x16be28` | `0xad574` | `[sp-36]` |
| `bl` at 0xaa02c | `(unknown callee) 0x22540` | `0x1a7304` | `0x16be34` | `0xae274` | `[sp-36]` |
| `bl` at 0xaa068 | `(unknown callee) 0x22540` | `0x1a7308` | `0x16be40` | `0xaecf8` | `[sp-36]` |
| `bl` at 0xaa0a4 | `(unknown callee) 0x22540` | `0x1a730c` | `0x16be4c` | `0xaf93c` | `[sp-36]` |
| `bl` at 0xaa0e0 | `(unknown callee) 0x22540` | `0x1a7310` | `0x16be58` | `0xafd98` | `[sp-36]` |
| `bl` at 0xaa11c | `(unknown callee) 0x22540` | `0x1a7314` | `0x16be64` | `0xb03b0` | `[sp-36]` |
| `mov` at 0xaa144 | `(unknown callee) (unknown target)` | `[sp-36]` | `0x1a7318` | `?` | `[0x318464]` |
| `bl` at 0xaa154 | `(unknown callee) 0x20550` | `0x1a7318` | `0x0` | `0x9830` | `?` |
| `bl` at 0xaa160 | `(unknown callee) 0x6e140` | `ret(0x20550)` | `?` | `?` | `?` |
| `bl` at 0xaa16c | `(unknown callee) 0xb286c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xa9e6c | `ret(0xaa230) <= 0x7ffff (unsigned, 32-bit)` | `ret(0xaa230) > 0x7ffff (unsigned, 32-bit)` |
| `bls` at 0xa9e9c | `ret(0xaa230) <= 0x7ffff (unsigned, 32-bit)` | `ret(0xaa230) > 0x7ffff (unsigned, 32-bit)` |
| `beq` at 0xa9ec8 | `[0x1a6fc0] == 0x0` | `[0x1a6fc0] != 0x0` |
| `beq` at 0xaa12c | `[0x318464] == 0x0` | `[0x318464] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaa174 | `r0` | `ret(0xb286c)` |

## `EnvPowerInt`

- `EnvPowerInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac2bc to 0xac7e4.
  Established. Byte stores preserve the low eight bits of entry r0, r1 and r2 at entry sp minus 0x15, 0x16 and 0x17 respectively (`strb` at 0xac2d0, `strb` at 0xac2d8, `strb` at 0xac2e0).
  A call to unnamed 0x6e1b0 has its result stored as a word at entry sp minus 0x28 (`bl` at 0xac430, `str` at 0xac434).
  The result from unnamed 0x2031c plus 1 is stored as a word at entry sp minus 0x2c (`bl` at 0xac43c, `add` at 0xac444, `str` at 0xac448).
  A word store consumes the result from unnamed 0x203f4, but its effective target is unknown (`bl` at 0xac484, `str` at 0xac494).
  What the three saved bytes represent remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xac2d0 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0xac2d8 | store | 1 | `sp -0x16` | `(r1 & 0xff)` |
| `strb` at 0xac2e0 | store | 1 | `sp -0x17` | `(r2 & 0xff)` |
| `str` at 0xac2e8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xac2ec | load | 4 | `abs 0xac7b8` | `-` |
| `ldrb` at 0xac2f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac328 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xac32c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac338 | load | 4 | `abs 0xac7b8` | `-` |
| `ldrb` at 0xac33c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac388 | load | 4 | `(unknown)` | `-` |
| `str` at 0xac390 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xac394 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xac398 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldrb` at 0xac39c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xac3a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xac3ac | load | 1 | `[sp-32] +0x5` | `-` |
| `ldrb` at 0xac3b0 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0xac3c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac3c8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xac3d0 | load | 4 | `abs 0xac7b8` | `-` |
| `ldrb` at 0xac3d4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac40c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xac410 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac41c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xac434 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0xac438 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xac448 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0xac44c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac474 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xac478 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xac47c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac480 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xac48c | load | 4 | `abs 0xac7bc` | `-` |
| `str` at 0xac490 | store | 4 | `(unknown)` | `0x16c078` |
| `str` at 0xac494 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0xac498 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xac49c | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xac4a0 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xac4a4 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xac4ac | load | 4 | `abs 0xac7c0` | `-` |
| `ldr` at 0xac4b0 | load | 4 | `abs 0xac7c4` | `-` |
| `str` at 0xac4c0 | store | 4 | `sp -0x34` | `0x0` |
| `ldrb` at 0xac4c8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac4f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac50c | load | 4 | `abs 0xac7c8` | `-` |
| `str` at 0xac514 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xac518 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac51c | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0xac528 | load | 4 | `abs 0xac7b8` | `-` |
| `ldrb` at 0xac52c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac564 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xac570 | load | 4 | `abs 0xac7b8` | `-` |
| `ldrb` at 0xac574 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac5ac | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xac5b8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xac5d0 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0xac5d4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xac5e4 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0xac5e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac610 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xac614 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac618 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xac61c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xac628 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac630 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xac63c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac650 | load | 4 | `abs 0xac7cc` | `-` |
| `str` at 0xac654 | store | 4 | `(unknown)` | `0x16c0b0` |
| `str` at 0xac658 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0xac65c | store | 4 | `(unknown)` | `ret(0x58280)` |
| `ldrb` at 0xac660 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xac664 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xac668 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xac66c | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xac674 | load | 4 | `abs 0xac7c0` | `-` |
| `ldr` at 0xac678 | load | 4 | `abs 0xac7d0` | `-` |
| `str` at 0xac688 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xac690 | load | 4 | `abs 0xac7d4` | `-` |
| `ldr` at 0xac694 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldrb` at 0xac6a4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xac6bc | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0xac6c0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xac6d0 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0xac6d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac6fc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xac700 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac704 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xac708 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xac714 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac71c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xac728 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac73c | load | 4 | `abs 0xac7d8` | `-` |
| `ldr` at 0xac74c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac750 | load | 1 | `[sp-36] +0x3` | `-` |
| `ldr` at 0xac75c | load | 4 | `abs 0xac7d4` | `-` |
| `ldr` at 0xac760 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0xac770 | load | 4 | `abs 0xac7dc` | `-` |
| `ldr` at 0xac778 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xac780 | store | 1 | `[sp-36] +0x3` | `0x1` |
| `ldr` at 0xac784 | load | 4 | `abs 0xac7e0` | `-` |
| `ldr` at 0xac798 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xac7a0 | store | 1 | `[sp-36] +0x3` | `0x0` |
| `str` at 0xac7a8 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xac7ac | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xac7b4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xac7b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac7b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac7b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac430 | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xac43c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xac484 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0xac4b8 | `(unknown callee) 0x22070` | `0x1` | `0x16bd68` | `0x455` | `0x0` |
| `bl` at 0xac5cc | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xac5d8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xac620 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-48]` | `[sp-44]` | `?` |
| `bl` at 0xac634 | `(unknown callee) 0x58a38` | `[sp-21]:1` | `[sp-32]` | `?` | `[sp-21]:1` |
| `bl` at 0xac648 | `(unknown callee) 0x58280` | `[sp-21]:1` | `ret(0x58a38)` | `ret(0x58a38)` | `[sp-21]:1` |
| `bl` at 0xac680 | `(unknown callee) 0x22070` | `0x1` | `0x16bd68` | `0x462` | `0x0` |
| `bl` at 0xac6b8 | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xac6c4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xac70c | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-48]` | `[sp-44]` | `?` |
| `bl` at 0xac720 | `(unknown callee) 0x58a38` | `[sp-21]:1` | `[sp-32]` | `?` | `[sp-21]:1` |
| `bl` at 0xac734 | `(unknown callee) 0x58280` | `[sp-21]:1` | `ret(0x58a38)` | `ret(0x58a38)` | `[sp-21]:1` |
| `bl` at 0xac748 | `(unknown callee) 0x29908` | `0x16c0e4` | `ret(0x203f4)` | `ret(0x58280)` | `ret(0x58280)` |
| `bl` at 0xac774 | `(unknown callee) 0x29908` | `0x16c0fc` | `?` | `?` | `?` |
| `bl` at 0xac788 | `(unknown callee) 0x2302c` | `0x1a7060` | `?` | `[sp-36]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xac334 | `? <= [sp-28] (unsigned, 32-bit)` | `? > [sp-28] (unsigned, 32-bit)` |
| `bne` at 0xac3a4 | `[[sp-32]+4]:1 != [sp-22]:1` | `[[sp-32]+4]:1 == [sp-22]:1` |
| `bne` at 0xac3b8 | `[[sp-32]+5]:1 != [sp-23]:1` | `[[sp-32]+5]:1 == [sp-23]:1` |
| `bhi` at 0xac418 | `? > [sp-28] (unsigned, 32-bit)` | `? <= [sp-28] (unsigned, 32-bit)` |
| `bne` at 0xac524 | `[[sp-36]+2]:1 != 0x0` | `[[sp-36]+2]:1 == 0x0` |
| `bne` at 0xac56c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xac5b4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xac6a0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xac758 | `[[sp-36]+3]:1 != 0x0` | `[[sp-36]+3]:1 == 0x0` |
| `beq` at 0xac76c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xac794 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac7b4 | `r0` | `?` |

## `EnvFanInt`

- `EnvFanInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac7e4 to 0xacda8.
  Established. Byte stores preserve the low eight bits of entry r0, r1 and r2 at entry sp minus 0x15, 0x16 and 0x17 respectively (`strb` at 0xac7f8, `strb` at 0xac800, `strb` at 0xac808).
  The result from unnamed 0x6e1b0 is stored as a word at entry sp minus 0x2c (`bl` at 0xac958, `str` at 0xac95c).
  A later call to unnamed 0x2031c has its result plus 1 stored as a word at entry sp minus 0x30 (`bl` at 0xac964, `add` at 0xac96c, `str` at 0xac970).
  The two byte stores have printed targets [sp-40] plus 4 and values 1 and 0 respectively (`strb` at 0xacd44, `strb` at 0xacd64).
  The meaning of the byte written at [sp-40] plus 4 remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xac7f8 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0xac800 | store | 1 | `sp -0x16` | `(r1 & 0xff)` |
| `strb` at 0xac808 | store | 1 | `sp -0x17` | `(r2 & 0xff)` |
| `str` at 0xac810 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xac814 | load | 4 | `abs 0xacd7c` | `-` |
| `ldrb` at 0xac818 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac850 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xac854 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac860 | load | 4 | `abs 0xacd7c` | `-` |
| `ldrb` at 0xac864 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac89c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac8b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xac8b8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xac8bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac8c0 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xac8c4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xac8d0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac8d4 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xac8d8 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0xac8e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac8f0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xac8f8 | load | 4 | `abs 0xacd7c` | `-` |
| `ldrb` at 0xac8fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac934 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xac938 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac944 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xac95c | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0xac960 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac970 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0xac974 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xac99c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xac9a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xac9a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xac9a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xac9b4 | load | 4 | `abs 0xacd80` | `-` |
| `str` at 0xac9b8 | store | 4 | `(unknown)` | `0x16c11c` |
| `str` at 0xac9bc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0xac9c0 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xac9c4 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xac9c8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xac9cc | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xac9d4 | load | 4 | `abs 0xacd84` | `-` |
| `ldr` at 0xac9d8 | load | 4 | `abs 0xacd88` | `-` |
| `str` at 0xac9e8 | store | 4 | `sp -0x38` | `0x0` |
| `ldrb` at 0xac9f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xaca1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaca34 | load | 4 | `abs 0xacd8c` | `-` |
| `str` at 0xaca3c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xaca40 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xaca44 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xaca50 | load | 4 | `abs 0xacd7c` | `-` |
| `ldrb` at 0xaca54 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xaca8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaca98 | load | 4 | `abs 0xacd7c` | `-` |
| `ldrb` at 0xaca9c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xacad4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xacae0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xacaf8 | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0xacafc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xacb0c | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0xacb10 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xacb38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xacb3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xacb40 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xacb44 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xacb50 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xacb54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xacb6c | load | 4 | `abs 0xacd90` | `-` |
| `str` at 0xacb70 | store | 4 | `(unknown)` | `0x16c154` |
| `str` at 0xacb74 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0xacb78 | store | 4 | `(unknown)` | `ret(0x64414)` |
| `ldrb` at 0xacb7c | load | 1 | `sp -0x16` | `-` |
| `str` at 0xacb80 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xacb84 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xacb88 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xacb90 | load | 4 | `abs 0xacd84` | `-` |
| `ldr` at 0xacb94 | load | 4 | `abs 0xacd94` | `-` |
| `str` at 0xacba4 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xacbc4 | store | 4 | `sp -0x20` | `0x1e` |
| `str` at 0xacbe4 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0xacc04 | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0xacc10 | store | 4 | `sp -0x20` | `0xf` |
| `ldr` at 0xacc14 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xacc18 | load | 1 | `[sp-40] +0x3` | `-` |
| `strb` at 0xacc20 | store | 1 | `[sp-40] +0x3` | `([[sp-40]+3]:1+1 & 0xff)` |
| `ldr` at 0xacc28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xacc38 | store | 4 | `sp -0x38` | `0x1` |
| `ldrb` at 0xacc40 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xacc44 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xacc48 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0xacc4c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xacc50 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldr` at 0xacc60 | load | 4 | `abs 0xacd98` | `-` |
| `ldr` at 0xacc64 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldrb` at 0xacc74 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xacc8c | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0xacc90 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xacca0 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0xacca4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xacccc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xaccd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaccd4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaccd8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xacce4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xacce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xacd00 | load | 4 | `abs 0xacd9c` | `-` |
| `ldr` at 0xacd10 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xacd14 | load | 1 | `[sp-40] +0x4` | `-` |
| `ldr` at 0xacd20 | load | 4 | `abs 0xacd98` | `-` |
| `ldr` at 0xacd24 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0xacd34 | load | 4 | `abs 0xacda0` | `-` |
| `ldr` at 0xacd3c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xacd44 | store | 1 | `[sp-40] +0x4` | `0x1` |
| `ldr` at 0xacd48 | load | 4 | `abs 0xacda4` | `-` |
| `ldr` at 0xacd5c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xacd64 | store | 1 | `[sp-40] +0x4` | `0x0` |
| `str` at 0xacd6c | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xacd70 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xacd78 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xacd78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xacd78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xacd78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac958 | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xac964 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xac9ac | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0xac9e0 | `(unknown callee) 0x22070` | `0x1` | `0x16bd68` | `0x497` | `0x0` |
| `bl` at 0xacaf4 | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xacb00 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xacb48 | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-52]` | `[sp-48]` | `?` |
| `bl` at 0xacb64 | `(unknown callee) 0x64414` | `[sp-21]:1` | `?` | `[sp-21]:1` | `?` |
| `bl` at 0xacb9c | `(unknown callee) 0x22070` | `0x1` | `0x16bd68` | `0x4a4` | `0x0` |
| `bl` at 0xacbac | `(unknown callee) 0x6dc0c` | `0x1a7318` | `?` | `?` | `?` |
| `bl` at 0xacbcc | `(unknown callee) 0x6dc0c` | `ret(0x6dc0c)` | `?` | `?` | `?` |
| `bl` at 0xacbec | `(unknown callee) 0x6dc0c` | `ret(0x6dc0c)` | `?` | `?` | `?` |
| `bl` at 0xacc5c | `(unknown callee) 0x1140d0` | `[sp-21]:1` | `[[sp-36]+1]:1` | `[[sp-36]+2]:1` | `[[sp-36]+2]:1` |
| `bl` at 0xacc88 | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xacc94 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xaccdc | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-52]` | `[sp-48]` | `?` |
| `bl` at 0xaccf8 | `(unknown callee) 0x64414` | `[sp-21]:1` | `?` | `[sp-21]:1` | `?` |
| `bl` at 0xacd0c | `(unknown callee) 0x29908` | `0x16c188` | `ret(0x203f4)` | `ret(0x64414)` | `ret(0x64414)` |
| `bl` at 0xacd38 | `(unknown callee) 0x29908` | `0x16c19c` | `?` | `?` | `?` |
| `bl` at 0xacd4c | `(unknown callee) 0x2302c` | `0x1a70b4` | `?` | `[sp-40]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xac85c | `? <= [sp-28] (unsigned, 32-bit)` | `? > [sp-28] (unsigned, 32-bit)` |
| `bne` at 0xac8cc | `[[sp-36]+1]:1 != [sp-22]:1` | `[[sp-36]+1]:1 == [sp-22]:1` |
| `bne` at 0xac8e0 | `[[sp-36]+2]:1 != [sp-23]:1` | `[[sp-36]+2]:1 == [sp-23]:1` |
| `bhi` at 0xac940 | `? > [sp-28] (unsigned, 32-bit)` | `? <= [sp-28] (unsigned, 32-bit)` |
| `bne` at 0xaca4c | `[[sp-40]+2]:1 != 0x0` | `[[sp-40]+2]:1 == 0x0` |
| `bne` at 0xaca94 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xacadc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xacbbc | `? != 0x47` | `? == 0x47` |
| `bne` at 0xacbdc | `? != 0x4d` | `? == 0x4d` |
| `bne` at 0xacbfc | `? != 0x7b` | `? == 0x7b` |
| `bge` at 0xacc30 | `? >= [sp-32] (signed, 32-bit)` | `? < [sp-32] (signed, 32-bit)` |
| `beq` at 0xacc70 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xacd1c | `[[sp-40]+4]:1 != 0x0` | `[[sp-40]+4]:1 == 0x0` |
| `beq` at 0xacd30 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xacd58 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xacd78 | `r0` | `[sp-56]` |

## `EnvTempInt`

- `EnvTempInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacda8 to 0xad3a8.
  Established. Byte stores preserve the low eight bits of entry r0, r1 and r2 at entry sp minus 0x15, 0x16 and 0x17 respectively (`strb` at 0xacdbc, `strb` at 0xacdc4, `strb` at 0xacdcc).
  Byte loads at two computed targets retain unknown effective addresses (`ldrb` at 0xace7c, `ldrb` at 0xacee4).
  The result from unnamed 0x6e1b0 is stored as a word at entry sp minus 0x24 (`bl` at 0xacf68, `str` at 0xacf6c).
  A call to unnamed 0x112330 has printed r0-r3 candidates [sp-21]:1, [sp-22]:1, [sp-23]:1 and entry sp minus 45 (`bl` at 0xad120).
  The device-level meanings of the three byte candidates and stack address remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xacdbc | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0xacdc4 | store | 1 | `sp -0x16` | `(r1 & 0xff)` |
| `strb` at 0xacdcc | store | 1 | `sp -0x17` | `(r2 & 0xff)` |
| `str` at 0xacdd4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xacdd8 | load | 4 | `abs 0xad37c` | `-` |
| `ldrb` at 0xacddc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xace14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xace18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xace24 | load | 4 | `abs 0xad37c` | `-` |
| `ldrb` at 0xace28 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xace60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xace74 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xace7c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xace80 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xace8c | load | 4 | `abs 0xad37c` | `-` |
| `ldrb` at 0xace90 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xacec8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xacedc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xacee4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xacee8 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0xacef8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xacf00 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xacf08 | load | 4 | `abs 0xad37c` | `-` |
| `ldrb` at 0xacf0c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xacf44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xacf48 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xacf54 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xacf6c | store | 4 | `sp -0x24` | `ret(0x6e1b0)` |
| `ldr` at 0xacf70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xacf80 | store | 4 | `sp -0x28` | `ret(0x2031c)+1` |
| `ldr` at 0xacf84 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xacfac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xacfb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xacfb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xacfb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xacfc4 | load | 4 | `abs 0xad380` | `-` |
| `str` at 0xacfc8 | store | 4 | `(unknown)` | `0x16c1b8` |
| `str` at 0xacfcc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0xacfd0 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xacfd4 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xacfd8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xacfdc | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xacfe4 | load | 4 | `abs 0xad384` | `-` |
| `ldr` at 0xacfe8 | load | 4 | `abs 0xad388` | `-` |
| `str` at 0xacff8 | store | 4 | `sp -0x34` | `0x0` |
| `ldrb` at 0xad000 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xad02c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad038 | load | 4 | `abs 0xad38c` | `-` |
| `str` at 0xad040 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xad044 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xad048 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xad054 | load | 4 | `abs 0xad37c` | `-` |
| `ldrb` at 0xad058 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xad090 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xad09c | load | 4 | `abs 0xad37c` | `-` |
| `ldrb` at 0xad0a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xad0d8 | load | 4 | `(unknown)` | `-` |
| `strb` at 0xad0e8 | store | 1 | `sp -0x2d` | `0x0` |
| `strb` at 0xad0f0 | store | 1 | `sp -0x2e` | `0x0` |
| `strb` at 0xad0f8 | store | 1 | `sp -0x2f` | `0x0` |
| `ldrb` at 0xad0fc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xad100 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xad104 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xad110 | store | 4 | `sp -0x54` | `sp-46` |
| `ldrb` at 0xad124 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xad128 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xad12c | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0xad148 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xad160 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0xad164 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xad174 | store | 4 | `sp -0x28` | `ret(0x2031c)+1` |
| `ldr` at 0xad178 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xad1a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xad1a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad1a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xad1ac | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xad1b8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xad1bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad1d4 | load | 4 | `abs 0xad390` | `-` |
| `str` at 0xad1d8 | store | 4 | `(unknown)` | `0x16c1f0` |
| `str` at 0xad1dc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0xad1e0 | store | 4 | `(unknown)` | `ret(0x72dd8)` |
| `ldrb` at 0xad1e4 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xad1e8 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xad1ec | load | 1 | `sp -0x17` | `-` |
| `str` at 0xad1f0 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldrb` at 0xad1f4 | load | 1 | `sp -0x2f` | `-` |
| `str` at 0xad1f8 | store | 4 | `(unknown)` | `[sp-47]:1` |
| `ldrb` at 0xad1fc | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xad200 | store | 4 | `(unknown)` | `[sp-45]:1` |
| `ldrb` at 0xad204 | load | 1 | `sp -0x2e` | `-` |
| `str` at 0xad208 | store | 4 | `(unknown)` | `[sp-46]:1` |
| `ldr` at 0xad210 | load | 4 | `abs 0xad384` | `-` |
| `ldr` at 0xad214 | load | 4 | `abs 0xad394` | `-` |
| `str` at 0xad224 | store | 4 | `sp -0x34` | `0x0` |
| `ldrb` at 0xad22c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xad230 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad244 | load | 4 | `abs 0xad398` | `-` |
| `ldr` at 0xad248 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldrb` at 0xad258 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xad270 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0xad274 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xad284 | store | 4 | `sp -0x28` | `ret(0x2031c)+1` |
| `ldr` at 0xad288 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xad2b0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xad2b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xad2b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xad2bc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xad2c8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xad2cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad2e4 | load | 4 | `abs 0xad39c` | `-` |
| `ldr` at 0xad2f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xad2f8 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xad304 | load | 4 | `abs 0xad398` | `-` |
| `ldr` at 0xad308 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0xad318 | load | 4 | `abs 0xad3a0` | `-` |
| `ldr` at 0xad31c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xad324 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xad328 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xad33c | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xad344 | store | 1 | `[sp-28] +0x3` | `0x1` |
| `ldr` at 0xad348 | load | 4 | `abs 0xad3a4` | `-` |
| `ldr` at 0xad35c | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xad364 | store | 1 | `[sp-28] +0x3` | `0x0` |
| `str` at 0xad36c | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xad370 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0xad378 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xad378 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xad378 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xad378 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xacf68 | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xacf74 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xacfbc | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-36]` | `[sp-40]` | `?` |
| `bl` at 0xacff0 | `(unknown callee) 0x22070` | `0x1` | `0x16bd68` | `0x4ea` | `0x0` |
| `bl` at 0xad120 | `(unknown callee) 0x112330` | `[sp-21]:1` | `[sp-22]:1` | `[sp-23]:1` | `sp-45` |
| `bl` at 0xad144 | `(unknown callee) 0x113108` | `[sp-21]:1` | `[sp-22]:1` | `[sp-23]:1` | `sp-47` |
| `bl` at 0xad15c | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xad168 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xad1b0 | `(unknown callee) 0x203f4` | `[sp-36]` | `[sp-44]` | `[sp-40]` | `?` |
| `bl` at 0xad1cc | `(unknown callee) 0x72dd8` | `[sp-21]:1` | `?` | `[sp-21]:1` | `?` |
| `bl` at 0xad21c | `(unknown callee) 0x22070` | `0x1` | `0x16bd68` | `0x4fd` | `0x0` |
| `bl` at 0xad240 | `(unknown callee) 0x72acc` | `[sp-21]:1` | `?` | `[sp-21]:1` | `?` |
| `bl` at 0xad26c | `(unknown callee) 0x6e1b0` | `[sp-21]:1` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xad278 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xad2c0 | `(unknown callee) 0x203f4` | `[sp-36]` | `[sp-44]` | `[sp-40]` | `?` |
| `bl` at 0xad2dc | `(unknown callee) 0x72dd8` | `[sp-21]:1` | `?` | `[sp-21]:1` | `?` |
| `bl` at 0xad2f0 | `(unknown callee) 0x29908` | `0x16c238` | `ret(0x203f4)` | `ret(0x72dd8)` | `ret(0x72dd8)` |
| `bl` at 0xad320 | `(unknown callee) 0x29908` | `0x16c24c` | `[sp-32]` | `?` | `?` |
| `bl` at 0xad338 | `(unknown callee) 0x72acc` | `[sp-21]:1` | `?` | `[sp-21]:1` | `?` |
| `bl` at 0xad34c | `(unknown callee) 0x2302c` | `0x1a7108` | `?` | `[sp-28]` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xace20 | `? <= [sp-32] (unsigned, 32-bit)` | `? > [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xace88 | `? != [sp-22]:1` | `? == [sp-22]:1` |
| `bne` at 0xacef0 | `? != [sp-23]:1` | `? == [sp-23]:1` |
| `bhi` at 0xacf50 | `? > [sp-32] (unsigned, 32-bit)` | `? <= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xad050 | `[[sp-28]+2]:1 != 0x0` | `[[sp-28]+2]:1 == 0x0` |
| `bne` at 0xad098 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xad0e0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xad254 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xad300 | `[[sp-28]+3]:1 != 0x0` | `[[sp-28]+3]:1 == 0x0` |
| `beq` at 0xad314 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xad358 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xad378 | `r0` | `?` |

## `EnvQuery`

- `EnvQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb22f8 to 0xb2768.
  Established. The result from unnamed 0x201fc is stored as a word in memory at [sp-24] (`bl` at 0xb2474, `str` at 0xb247c).
  A later call to unnamed 0x20058 also has its result stored as a word in memory at [sp-24] (`bl` at 0xb249c, `str` at 0xb24ac).
  A byte store writes 0 to memory at [[sp-24]] (`strb` at 0xb24c8).
  The word loaded from absolute address 0x1a731c is stored at entry sp minus 0x34 (`ldr` at 0xb256c, `str` at 0xb2570).
  The purpose of the data reached through [sp-24] remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb2308 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb230c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb2328 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xb2344 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xb234c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xb2354 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb2358 | load | 4 | `abs 0xb2724` | `-` |
| `ldr` at 0xb235c | load | 4 | `abs 0xb2728` | `-` |
| `ldr` at 0xb2360 | load | 4 | `abs 0x1b0b44` | `-` |
| `ldr` at 0xb2364 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2370 | load | 4 | `abs 0xb2724` | `-` |
| `ldr` at 0xb2374 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb23ac | load | 4 | `abs 0xb2724` | `-` |
| `ldr` at 0xb23b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb23e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb23ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb23f4 | load | 4 | `abs 0xb2724` | `-` |
| `ldr` at 0xb23f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2430 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2438 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb2440 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb2444 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb244c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xb2454 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb2458 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb247c | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xb2480 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xb248c | load | 4 | `abs 0xb272c` | `-` |
| `ldr` at 0xb2490 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb2498 | load | 4 | `abs 0xb2730` | `-` |
| `ldr` at 0xb24a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb24ac | store | 4 | `[sp-24] +0x0` | `ret(0x20058)` |
| `ldr` at 0xb24b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb24b8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xb24bc | store | 4 | `sp -0x30` | `[[sp-24]]` |
| `ldr` at 0xb24c0 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xb24c8 | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0xb24cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb24d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb24dc | load | 4 | `abs 0xb2734` | `-` |
| `ldr` at 0xb24e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb24e8 | load | 4 | `abs 0xb2738` | `-` |
| `ldr` at 0xb24f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb24f4 | load | 4 | `abs 0xb273c` | `-` |
| `ldr` at 0xb24fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb2508 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb250c | load | 4 | `abs 0xb2740` | `-` |
| `ldr` at 0xb2514 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2518 | load | 4 | `abs 0xb2744` | `-` |
| `ldr` at 0xb2520 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2524 | load | 4 | `abs 0xb2748` | `-` |
| `ldr` at 0xb252c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2530 | load | 4 | `abs 0xb274c` | `-` |
| `ldr` at 0xb253c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2540 | load | 4 | `abs 0xb2750` | `-` |
| `ldr` at 0xb2544 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb254c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb255c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb2564 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x2031c))` |
| `ldr` at 0xb2568 | load | 4 | `abs 0xb2724` | `-` |
| `ldr` at 0xb256c | load | 4 | `abs 0x1a731c` | `-` |
| `str` at 0xb2570 | store | 4 | `sp -0x34` | `[0x1a731c]` |
| `ldr` at 0xb2574 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb2580 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb258c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb259c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb25ac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb25b0 | load | 4 | `abs 0xb2754` | `-` |
| `ldr` at 0xb25bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb25c0 | load | 4 | `abs 0xb2758` | `-` |
| `ldr` at 0xb25cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb25d0 | load | 4 | `abs 0xb275c` | `-` |
| `ldr` at 0xb25dc | load | 4 | `abs 0xb2724` | `-` |
| `ldr` at 0xb25e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb25e4 | load | 4 | `abs 0xb2760` | `-` |
| `ldr` at 0xb25e8 | load | 4 | `abs 0x1a731c` | `-` |
| `ldr` at 0xb25f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2600 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb2608 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x2031c))` |
| `ldr` at 0xb260c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb261c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2620 | load | 4 | `abs 0xb2764` | `-` |
| `ldr` at 0xb2628 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2638 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb2640 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x2031c))` |
| `ldr` at 0xb2644 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb264c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb265c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb2664 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x2031c))` |
| `ldr` at 0xb2668 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2670 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2680 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb2688 | store | 4 | `sp -0x30` | `([sp-48] + ret(0x2031c))` |
| `ldr` at 0xb268c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2694 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb26a4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb26ac | store | 4 | `sp -0x30` | `([sp-48] + ret(0x2031c))` |
| `ldr` at 0xb26b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb26bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb26c0 | load | 4 | `abs 0xb2734` | `-` |
| `ldr` at 0xb26c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb26cc | load | 4 | `abs 0xb2738` | `-` |
| `ldr` at 0xb26d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb26d8 | load | 4 | `abs 0xb273c` | `-` |
| `ldr` at 0xb26e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb26ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb26f0 | load | 4 | `abs 0xb2740` | `-` |
| `ldr` at 0xb26f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb26fc | load | 4 | `abs 0xb2744` | `-` |
| `ldr` at 0xb2704 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2708 | load | 4 | `abs 0xb2748` | `-` |
| `ldr` at 0xb2710 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb2714 | load | 4 | `abs 0xb274c` | `-` |
| `ldm` at 0xb2720 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb2720 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb2720 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb2720 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb2314 | `(unknown callee) 0x28230` | `0x6` | `r1` | `r2` | `r3` |
| `bl` at 0xb2330 | `(unknown callee) 0x28230` | `0x7` | `?` | `?` | `?` |
| `bl` at 0xb2474 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xb2494 | `(unknown callee) 0x29908` | `0x16caf0` | `[sp-36]` | `?` | `[[sp-24]]` |
| `bl` at 0xb249c | `(unknown callee) 0x20058` | `0x16cb10` | `?` | `?` | `?` |
| `bl` at 0xb24e0 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cb40` | `[[sp-24]]` | `[sp-40]` |
| `bl` at 0xb24ec | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cb74` | `?` | `?` |
| `bl` at 0xb24f8 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cba8` | `?` | `?` |
| `bl` at 0xb2510 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cbdc` | `?` | `[sp-44]` |
| `bl` at 0xb251c | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cc10` | `?` | `?` |
| `bl` at 0xb2528 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cc44` | `?` | `?` |
| `bl` at 0xb2534 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cc74` | `?` | `?` |
| `bl` at 0xb2548 | `(unknown callee) 0x205bc` | `[sp-48]` | `0x16cca8` | `[sp-36]` | `[sp-40]` |
| `bl` at 0xb2550 | `(unknown callee) 0x2031c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb25b4 | `(unknown callee) 0x20634` | `([sp-48] + ret(0x2031c))` | `0x16ccdc` | `ret(0x2031c)` | `[0x1a731c]` |
| `bl` at 0xb25c4 | `(unknown callee) 0x20634` | `([sp-48] + ret(0x2031c))` | `0x16cce0` | `ret(0x2031c)` | `[0x1a731c]` |
| `bl` at 0xb25d4 | `(unknown callee) 0x20634` | `([sp-48] + ret(0x2031c))` | `0x16cd08` | `ret(0x2031c)` | `[0x1a731c]` |
| `bl` at 0xb25ec | `(unknown callee) 0x205bc` | `([sp-48] + ret(0x2031c))` | `0x16cd2c` | `[0x1a731c]` | `0x1a7318` |
| `bl` at 0xb25f4 | `(unknown callee) 0x2031c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb2624 | `(unknown callee) 0x203c4` | `([sp-48] + ret(0x2031c))` | `0x16c7ec` | `ret(0x2031c)` | `[sp-28]` |
| `bl` at 0xb262c | `(unknown callee) 0x2031c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb2648 | `(unknown callee) 0xb11b8` | `([sp-48] + ret(0x2031c))` | `?` | `ret(0x2031c)` | `([sp-48] + ret(0x2031c))` |
| `bl` at 0xb2650 | `(unknown callee) 0x2031c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb266c | `(unknown callee) 0xb1744` | `([sp-48] + ret(0x2031c))` | `?` | `ret(0x2031c)` | `([sp-48] + ret(0x2031c))` |
| `bl` at 0xb2674 | `(unknown callee) 0x2031c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb2690 | `(unknown callee) 0xb1b3c` | `([sp-48] + ret(0x2031c))` | `?` | `ret(0x2031c)` | `([sp-48] + ret(0x2031c))` |
| `bl` at 0xb2698 | `(unknown callee) 0x2031c` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xb26c4 | `(unknown callee) 0x203c4` | `([sp-48] + ret(0x2031c))` | `0x16cb40` | `ret(0x2031c)` | `[sp-40]` |
| `bl` at 0xb26d0 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cb74` | `?` | `?` |
| `bl` at 0xb26dc | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cba8` | `?` | `?` |
| `bl` at 0xb26f4 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cbdc` | `?` | `[sp-44]` |
| `bl` at 0xb2700 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cc10` | `?` | `?` |
| `bl` at 0xb270c | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cc44` | `?` | `?` |
| `bl` at 0xb2718 | `(unknown callee) 0x203c4` | `[sp-48]` | `0x16cc74` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb236c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb2488 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0xb24d4 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xb2504 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |
| `beq` at 0xb257c | `[0x1a731c] == 0x1` | `[0x1a731c] != 0x1` |
| `bgt` at 0xb2588 | `[0x1a731c] > 0x1 (signed, 32-bit)` | `[0x1a731c] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xb2594 | `[0x1a731c] == 0x0` | `[0x1a731c] != 0x0` |
| `beq` at 0xb25a4 | `[0x1a731c] == 0x2` | `[0x1a731c] != 0x2` |
| `bne` at 0xb2614 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xb26b8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xb26e8 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb2720 | `r0` | `?` |

## `EnvTemp`

- `EnvTemp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb2768 to 0xb27a4.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0xb2778).
  A call to unnamed 0x2624c has printed r0-r3 candidates 7, unknown, entry r2 and unknown (`bl` at 0xb2794).
  The unsupported mvn effect precedes a call to unnamed 0xb286c whose four printed register candidates are unknown (`mvn` at 0xb2798, `bl` at 0xb279c).
  The meanings of candidate 7 and the unnamed calls remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb2778 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb277c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb27a0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb27a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb27a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb27a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb2794 | `(unknown callee) 0x2624c` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0xb279c | `(unknown callee) 0xb286c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb27a0 | `r0` | `ret(0xb286c)` |

## `EnvPower`

- `EnvPower`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb27a4 to 0xb27d0.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0xb27b4).
  A call to unnamed 0x2624c has printed r0-r3 candidates 0x7d, entry r0, entry r2 and entry r3 (`bl` at 0xb27c0).
  The unsupported mvn effect precedes a call to unnamed 0xb286c whose four printed register candidates are unknown (`mvn` at 0xb27c4, `bl` at 0xb27c8).
  The role of candidate 0x7d remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb27b4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb27bc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb27cc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb27cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb27cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb27cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb27c0 | `(unknown callee) 0x2624c` | `0x7d` | `r0` | `r2` | `r3` |
| `bl` at 0xb27c8 | `(unknown callee) 0xb286c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb27cc | `r0` | `ret(0xb286c)` |

## `EnvAuto`

- `EnvAuto`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb27d0 to 0xb2840.
  Established. Entry r0 is stored as a word at entry sp minus 0x14, and the branch to 0xb2828 is taken when that entry value is zero (`str` at 0xb27e0, `ldr` at 0xb27e4, `beq` at 0xb27ec).
  Distinct calls to unnamed 0x2624c have printed r0 candidates 7, 0x7d and 6 (`bl` at 0xb2808, `bl` at 0xb2824, `bl` at 0xb2830).
  The unsupported mvn effect precedes a call to unnamed 0xb286c whose four printed register candidates are unknown (`mvn` at 0xb2834, `bl` at 0xb2838).
  The relationship between candidates 7, 0x7d and 6 remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb27e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb27e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb27f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb280c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb282c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb283c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb283c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb283c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb283c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb2808 | `(unknown callee) 0x2624c` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0xb2824 | `(unknown callee) 0x2624c` | `0x7d` | `?` | `?` | `?` |
| `bl` at 0xb2830 | `(unknown callee) 0x2624c` | `0x6` | `?` | `?` | `?` |
| `bl` at 0xb2838 | `(unknown callee) 0xb286c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb27ec | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb283c | `r0` | `ret(0xb286c)` |

## `EnvAltitude`

- `EnvAltitude`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb2840 to 0xb286c.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0xb2850).
  A call to unnamed 0x2624c has printed r0-r3 candidates 0xc, entry r0, entry r2 and entry r3 (`bl` at 0xb285c).
  The unsupported mvn effect precedes a call to unnamed 0xb286c whose four printed register candidates are unknown (`mvn` at 0xb2860, `bl` at 0xb2864).
  Whether entry r0 has units remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb2850 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb2858 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb2868 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb2868 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb2868 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb2868 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb285c | `(unknown callee) 0x2624c` | `0xc` | `r0` | `r2` | `r3` |
| `bl` at 0xb2864 | `(unknown callee) 0xb286c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb2868 | `r0` | `ret(0xb286c)` |

## `EnvReset`

- `EnvReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb286c to 0xb5630.
  Established. Word stores write 0 at entry sp minus 0x20, 0x30 and 0x34 (`str` at 0xb2884, `str` at 0xb288c, `str` at 0xb2894).
  The result from unnamed 0x28230 is stored as a word at absolute address 0x1a6fc4 (`bl` at 0xb28b4, `str` at 0xb28b8).
  The indirect transfer at 0xb3acc has an unknown taken target, so omitted effects remain unknown (`ldrls` at 0xb3acc).
  Later word stores write 0 to absolute address 0x1a6fe0 and 0x3c to absolute address 0x1a6ff4 (`str` at 0xb53a4, `str` at 0xb53c4).
  Distinct results from unnamed 0xac0e4 and 0xac1d0 are stored as words at entry sp minus 0xb8 and 0xb4 respectively (`bl` at 0xb5404, `str` at 0xb540c, `bl` at 0xb5414, `str` at 0xb541c).
  The device-level purpose of these stores and the unanalysed paths remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb287c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xb2884 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xb288c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xb2894 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb2898 | load | 4 | `abs 0xb3780` | `-` |
| `ldr` at 0xb289c | load | 4 | `abs 0x1a6fc0` | `-` |
| `ldr` at 0xb28ac | load | 4 | `abs 0xb3784` | `-` |
| `str` at 0xb28b8 | store | 4 | `abs 0x1a6fc4` | `ret(0x28230)` |
| `ldr` at 0xb28bc | load | 4 | `abs 0xb3784` | `-` |
| `ldr` at 0xb28c0 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0xb28d0 | load | 4 | `abs 0xb3788` | `-` |
| `ldr` at 0xb28d8 | load | 4 | `abs 0xb378c` | `-` |
| `ldr` at 0xb28e4 | load | 4 | `abs 0xb3784` | `-` |
| `ldr` at 0xb28e8 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0xb28f8 | load | 4 | `abs 0xb3790` | `-` |
| `ldr` at 0xb2900 | load | 4 | `abs 0xb3794` | `-` |
| `ldr` at 0xb290c | load | 4 | `abs 0xb3798` | `-` |
| `ldr` at 0xb2918 | load | 4 | `abs 0xb379c` | `-` |
| `ldr` at 0xb2924 | load | 4 | `abs 0xb37a0` | `-` |
| `ldr` at 0xb2930 | load | 4 | `abs 0xb37a4` | `-` |
| `ldr` at 0xb293c | load | 4 | `abs 0xb37a8` | `-` |
| `ldr` at 0xb2948 | load | 4 | `abs 0xb37ac` | `-` |
| `ldr` at 0xb2954 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2964 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb2968 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb296c | load | 4 | `abs 0xb37b0` | `-` |
| `ldr` at 0xb2970 | load | 4 | `abs 0x1b0b44` | `-` |
| `str` at 0xb2974 | store | 4 | `sp -0x2c` | `[0x1b0b44]` |
| `ldr` at 0xb297c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2980 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0xb2984 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb298c | store | 4 | `sp -0x2c` | `[sp-28]+1` |
| `ldr` at 0xb2990 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb2994 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb2998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb299c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb29a8 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb29ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb29e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a10 | load | 4 | `abs 0xb37b4` | `-` |
| `ldr` at 0xb2a18 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2a24 | store | 4 | `(unknown)` | `ret(0x530f4)` |
| `ldr` at 0xb2a28 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2a2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2a70 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2a74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2aac | load | 4 | `abs 0xb37b8` | `-` |
| `ldr` at 0xb2ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ab4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2abc | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2ac0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2afc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb2b04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2b30 | load | 4 | `abs 0xb37bc` | `-` |
| `ldr` at 0xb2b38 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2b3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2b74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2b94 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2b98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2bd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2bfc | load | 4 | `abs 0xb37c0` | `-` |
| `ldr` at 0xb2c04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2c10 | store | 4 | `(unknown)` | `ret(0x62bfc)` |
| `ldr` at 0xb2c14 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2c18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2c50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2c5c | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2c60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2c98 | load | 4 | `abs 0xb37c4` | `-` |
| `ldr` at 0xb2c9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ca0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2ca8 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2cac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2ce8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb2cf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2d1c | load | 4 | `abs 0xb37c8` | `-` |
| `ldr` at 0xb2d24 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2d28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2d60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2d80 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2d84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2dbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2de8 | load | 4 | `abs 0xb37cc` | `-` |
| `ldr` at 0xb2df0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2dfc | store | 4 | `(unknown)` | `ret(0x7243c)` |
| `ldr` at 0xb2e00 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2e04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2e3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2e48 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2e84 | load | 4 | `abs 0xb37d0` | `-` |
| `ldr` at 0xb2e88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2e8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2e94 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2e98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2ed4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb2edc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2f08 | load | 4 | `abs 0xb37d4` | `-` |
| `ldr` at 0xb2f10 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2f14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2f4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb2f60 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2f64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2f9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2fc8 | load | 4 | `abs 0xb37d8` | `-` |
| `ldr` at 0xb2fd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb2fdc | store | 4 | `(unknown)` | `ret(0x111db8)` |
| `ldr` at 0xb2fe0 | load | 4 | `abs 0xb3784` | `-` |
| `ldr` at 0xb2fe4 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0xb2ff4 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb2ff8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3030 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3034 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb306c | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb30a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb30ac | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0xb30b0 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb30b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb30ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xb30f0 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0xb30f4 | load | 4 | `abs 0xb37dc` | `-` |
| `ldr` at 0xb30f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb30fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb3100 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb3108 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb3114 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb3118 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb311c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3154 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb3158 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3164 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3168 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb316c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb31b8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb31bc | store | 1 | `(unknown)` | `([sp-28] & 0xff)` |
| `ldr` at 0xb31c0 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb31c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb31c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3214 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb3218 | store | 1 | `(unknown)` | `([sp-36] & 0xff)` |
| `ldr` at 0xb321c | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3220 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3224 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb3274 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb3278 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb327c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3280 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb32d0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb32d4 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb32d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb32dc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb332c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb3330 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3334 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3338 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb3388 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb338c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb3394 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb339c | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb33a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb33dc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb33e0 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb33e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb33e8 | load | 4 | `abs 0xb37e0` | `-` |
| `str` at 0xb3420 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xb3428 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb342c | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3430 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3468 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb346c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3478 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb347c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3484 | load | 4 | `abs 0xb37e4` | `-` |
| `ldr` at 0xb34cc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb34d0 | store | 1 | `(unknown)` | `([sp-28] & 0xff)` |
| `ldr` at 0xb34d4 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb34d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb34dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb34e0 | load | 4 | `abs 0xb37e4` | `-` |
| `ldr` at 0xb3528 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb352c | store | 1 | `(unknown)` | `([sp-36] & 0xff)` |
| `ldr` at 0xb3530 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3534 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3538 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb353c | load | 4 | `abs 0xb37e4` | `-` |
| `strb` at 0xb3588 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb358c | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3590 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3594 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3598 | load | 4 | `abs 0xb37e4` | `-` |
| `strb` at 0xb35e4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb35e8 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb35ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb35f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb35f4 | load | 4 | `abs 0xb37e8` | `-` |
| `strb` at 0xb3640 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb3644 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3648 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb364c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3650 | load | 4 | `abs 0xb37e8` | `-` |
| `strb` at 0xb369c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb36a0 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb36a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb36a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb36ac | load | 4 | `abs 0xb37e8` | `-` |
| `strb` at 0xb36f8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb36fc | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3700 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3708 | load | 4 | `abs 0xb37e8` | `-` |
| `strb` at 0xb3754 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb3758 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3760 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3770 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb3778 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb3828 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb382c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb3830 | load | 4 | `abs 0xb37ec` | `-` |
| `str` at 0xb3868 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xb3870 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb3874 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3878 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb38b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb38b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb38c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb38c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb38dc | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xb38e0 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb38e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb38e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb38ec | load | 4 | `abs 0xb37f0` | `-` |
| `ldr` at 0xb3928 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb392c | store | 1 | `(unknown)` | `([sp-28] & 0xff)` |
| `ldr` at 0xb3930 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3934 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3938 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb393c | load | 4 | `abs 0xb37f0` | `-` |
| `ldr` at 0xb3978 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xb397c | store | 1 | `(unknown)` | `([sp-36] & 0xff)` |
| `ldr` at 0xb3980 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3984 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3988 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb398c | load | 4 | `abs 0xb37f0` | `-` |
| `strb` at 0xb39cc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb39d0 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb39d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb39d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb39dc | load | 4 | `abs 0xb37f0` | `-` |
| `strb` at 0xb3a1c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb3a20 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3a24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3a28 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb3a6c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb3a70 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb3a74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb3a78 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xb3abc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb3ac0 | load | 4 | `sp -0xb8` | `-` |
| `ldrls` at 0xb3acc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb43f0 | load | 4 | `abs 0xb37f8` | `-` |
| `ldr` at 0xb43f4 | load | 4 | `abs 0x1a6fc8` | `-` |
| `ldr` at 0xb4400 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb4404 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4408 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb440c | load | 4 | `abs 0xb3810` | `-` |
| `ldr` at 0xb4448 | load | 4 | `abs 0xb37fc` | `-` |
| `ldr` at 0xb444c | load | 4 | `abs 0x1a6fcc` | `-` |
| `str` at 0xb4450 | store | 4 | `(unknown)` | `[0x1a6fcc]` |
| `ldr` at 0xb4454 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb4458 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb445c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4460 | load | 4 | `abs 0xb3814` | `-` |
| `ldr` at 0xb449c | load | 4 | `abs 0xb3800` | `-` |
| `ldr` at 0xb44a0 | load | 4 | `abs 0x1a6fd0` | `-` |
| `str` at 0xb44a4 | store | 4 | `(unknown)` | `[0x1a6fd0]` |
| `ldr` at 0xb44a8 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb44ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb44b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb44f0 | load | 4 | `abs 0xb3804` | `-` |
| `ldr` at 0xb44f4 | load | 4 | `abs 0x1a6fd4` | `-` |
| `str` at 0xb44f8 | store | 4 | `(unknown)` | `[0x1a6fd4]` |
| `ldr` at 0xb44fc | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb4500 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4504 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4508 | load | 4 | `abs 0xb3818` | `-` |
| `ldr` at 0xb4544 | load | 4 | `abs 0xb3808` | `-` |
| `ldr` at 0xb4548 | load | 4 | `abs 0x1a6fd8` | `-` |
| `str` at 0xb454c | store | 4 | `(unknown)` | `[0x1a6fd8]` |
| `ldr` at 0xb4550 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb4554 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4558 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb455c | load | 4 | `abs 0xb3824` | `-` |
| `ldr` at 0xb4598 | load | 4 | `abs 0xb380c` | `-` |
| `ldr` at 0xb459c | load | 4 | `abs 0x1a6fdc` | `-` |
| `str` at 0xb45a0 | store | 4 | `(unknown)` | `[0x1a6fdc]` |
| `ldr` at 0xb45b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb45bc | load | 4 | `abs 0xb381c` | `-` |
| `ldr` at 0xb45c0 | load | 4 | `abs 0x3183d0` | `-` |
| `ldr` at 0xb45cc | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb45d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb45d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb45d8 | load | 4 | `abs 0xb3810` | `-` |
| `ldr` at 0xb4614 | load | 4 | `abs 0xb381c` | `-` |
| `ldr` at 0xb4618 | load | 4 | `abs 0x3183d0` | `-` |
| `str` at 0xb461c | store | 4 | `(unknown)` | `[0x3183d0]` |
| `ldr` at 0xb4620 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb4624 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4628 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb462c | load | 4 | `abs 0xb3814` | `-` |
| `ldr` at 0xb4668 | load | 4 | `abs 0xb381c` | `-` |
| `ldr` at 0xb466c | load | 4 | `abs 0x3183d4` | `-` |
| `str` at 0xb4670 | store | 4 | `(unknown)` | `[0x3183d4]` |
| `ldr` at 0xb4674 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb4678 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb467c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb46bc | load | 4 | `abs 0xb381c` | `-` |
| `ldr` at 0xb46c0 | load | 4 | `abs 0x3183d8` | `-` |
| `str` at 0xb46c4 | store | 4 | `(unknown)` | `[0x3183d8]` |
| `ldr` at 0xb46c8 | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb46cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb46d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb46d4 | load | 4 | `abs 0xb3818` | `-` |
| `ldr` at 0xb4710 | load | 4 | `abs 0xb381c` | `-` |
| `ldr` at 0xb4714 | load | 4 | `abs 0x3183dc` | `-` |
| `str` at 0xb4718 | store | 4 | `(unknown)` | `[0x3183dc]` |
| `ldr` at 0xb471c | load | 4 | `abs 0xb3820` | `-` |
| `ldr` at 0xb4720 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4724 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4728 | load | 4 | `abs 0xb3824` | `-` |
| `ldr` at 0xb4764 | load | 4 | `abs 0xb5600` | `-` |
| `ldr` at 0xb4768 | load | 4 | `abs 0x3183e0` | `-` |
| `str` at 0xb476c | store | 4 | `(unknown)` | `[0x3183e0]` |
| `ldr` at 0xb477c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4790 | load | 4 | `abs 0xb5604` | `-` |
| `ldr` at 0xb4794 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xb479c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb47a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb47ac | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb47b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb47b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb47b8 | load | 4 | `abs 0xb560c` | `-` |
| `ldr` at 0xb47f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb47fc | load | 4 | `abs 0xb5604` | `-` |
| `ldr` at 0xb4800 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xb4808 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb480c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4810 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4814 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4818 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb481c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4820 | load | 4 | `abs 0xb5610` | `-` |
| `ldr` at 0xb485c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4864 | load | 4 | `abs 0xb5604` | `-` |
| `ldr` at 0xb4868 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xb4870 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb4874 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4878 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb487c | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4884 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb48c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb48cc | load | 4 | `abs 0xb5604` | `-` |
| `ldr` at 0xb48d0 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xb48d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb48dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xb48e0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb48e4 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb48e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb48ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb48f0 | load | 4 | `abs 0xb5614` | `-` |
| `ldr` at 0xb492c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4934 | load | 4 | `abs 0xb5604` | `-` |
| `ldr` at 0xb4938 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xb4940 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb4944 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4948 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb494c | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4950 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4958 | load | 4 | `abs 0xb5618` | `-` |
| `ldr` at 0xb4994 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb499c | load | 4 | `abs 0xb5604` | `-` |
| `ldr` at 0xb49a0 | load | 4 | `abs 0x31a520` | `-` |
| `ldr` at 0xb49a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb49ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xb49b0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb49b8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb49bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb49c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb49c4 | load | 4 | `abs 0xb560c` | `-` |
| `str` at 0xb4a04 | store | 4 | `(unknown)` | `0x1e` |
| `ldr` at 0xb4a08 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4a0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4a10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4a14 | load | 4 | `abs 0xb5610` | `-` |
| `str` at 0xb4a54 | store | 4 | `(unknown)` | `0x23` |
| `ldr` at 0xb4a58 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4a5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4a60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb4aa4 | store | 4 | `(unknown)` | `0x28` |
| `ldr` at 0xb4aa8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4aac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4ab4 | load | 4 | `abs 0xb5614` | `-` |
| `str` at 0xb4af4 | store | 4 | `(unknown)` | `0x2d` |
| `ldr` at 0xb4af8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4afc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4b00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4b04 | load | 4 | `abs 0xb5618` | `-` |
| `str` at 0xb4b44 | store | 4 | `(unknown)` | `0x1b` |
| `ldr` at 0xb4b5c | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4b60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4b64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4b68 | load | 4 | `abs 0xb560c` | `-` |
| `ldr` at 0xb4ba4 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4ba8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4bac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4bb0 | load | 4 | `abs 0xb560c` | `-` |
| `ldr` at 0xb4bec | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4bf4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4bf8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4bfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4c00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4c04 | load | 4 | `abs 0xb5610` | `-` |
| `ldr` at 0xb4c40 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4c44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4c48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4c4c | load | 4 | `abs 0xb5610` | `-` |
| `ldr` at 0xb4c88 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4c90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4c94 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4c98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4c9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4cdc | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4ce0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4d24 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4d2c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4d30 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4d34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4d38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4d3c | load | 4 | `abs 0xb5614` | `-` |
| `ldr` at 0xb4d78 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4d7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4d80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4d84 | load | 4 | `abs 0xb5614` | `-` |
| `ldr` at 0xb4dc0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4dc8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4dcc | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4dd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4dd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4dd8 | load | 4 | `abs 0xb5618` | `-` |
| `ldr` at 0xb4e14 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4e18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4e1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4e20 | load | 4 | `abs 0xb5618` | `-` |
| `ldr` at 0xb4e5c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4e64 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4e68 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4e70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4e74 | load | 4 | `abs 0xb561c` | `-` |
| `ldr` at 0xb4eb0 | load | 4 | `abs 0xb5600` | `-` |
| `ldr` at 0xb4eb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4ec8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4ecc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4ed0 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4ed4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4edc | load | 4 | `abs 0xb560c` | `-` |
| `ldr` at 0xb4f18 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4f1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4f20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4f24 | load | 4 | `abs 0xb560c` | `-` |
| `ldr` at 0xb4f60 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4f64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4f68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4f6c | load | 4 | `abs 0xb561c` | `-` |
| `ldr` at 0xb4fa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb4fac | load | 4 | `(unknown)` | `-` |
| `str` at 0xb4fb4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb4fb8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb4fbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb4fc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4fc4 | load | 4 | `abs 0xb5610` | `-` |
| `ldr` at 0xb5000 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb5004 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5008 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb500c | load | 4 | `abs 0xb5610` | `-` |
| `ldr` at 0xb5048 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb504c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5050 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5054 | load | 4 | `abs 0xb561c` | `-` |
| `ldr` at 0xb5090 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5094 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb509c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb50a0 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb50a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb50a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb50e8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb50ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb50f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5130 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb5134 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5138 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb513c | load | 4 | `abs 0xb561c` | `-` |
| `ldr` at 0xb5178 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb517c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb5184 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb5188 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb518c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5190 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5194 | load | 4 | `abs 0xb5614` | `-` |
| `ldr` at 0xb51d0 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb51d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb51d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb51dc | load | 4 | `abs 0xb5614` | `-` |
| `ldr` at 0xb5218 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb521c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5224 | load | 4 | `abs 0xb561c` | `-` |
| `ldr` at 0xb5260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5264 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb526c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb5270 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb5274 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5278 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb527c | load | 4 | `abs 0xb5618` | `-` |
| `ldr` at 0xb52b8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb52bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb52c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb52c4 | load | 4 | `abs 0xb5618` | `-` |
| `ldr` at 0xb5300 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb5304 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5308 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb530c | load | 4 | `abs 0xb561c` | `-` |
| `ldr` at 0xb5348 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb534c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb5354 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb5358 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb5360 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb5368 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb5370 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xb5378 | load | 4 | `abs 0xb5620` | `-` |
| `ldr` at 0xb537c | load | 4 | `abs 0x1a0380` | `-` |
| `ldr` at 0xb539c | load | 4 | `abs 0xb5624` | `-` |
| `str` at 0xb53a4 | store | 4 | `abs 0x1a6fe0` | `0x0` |
| `ldr` at 0xb53bc | load | 4 | `abs 0xb5628` | `-` |
| `str` at 0xb53c4 | store | 4 | `abs 0x1a6ff4` | `0x3c` |
| `ldr` at 0xb53c8 | load | 4 | `abs 0xb5608` | `-` |
| `str` at 0xb53e4 | store | 4 | `abs 0x1a7318` | `?` |
| `ldr` at 0xb53e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb53ec | store | 4 | `sp -0x1c` | `[sp-40]` |
| `ldr` at 0xb53f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb53f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb5400 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb540c | store | 4 | `sp -0xb8` | `ret(0xac0e4)` |
| `ldr` at 0xb5410 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb541c | store | 4 | `sp -0xb4` | `ret(0xac1d0)` |
| `str` at 0xb5424 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb5428 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb542c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5464 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5468 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5474 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5478 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb547c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0xb5488 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb5490 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb5494 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb549c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb54a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb54b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb54b8 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xb54c0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb54c4 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb54c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5500 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5504 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5514 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5518 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0xb5524 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb552c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb5530 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb5538 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb5540 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb5548 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xb5550 | load | 4 | `abs 0xb5608` | `-` |
| `str` at 0xb5554 | store | 4 | `sp -0xbc` | `0x1a7318` |
| `ldr` at 0xb5558 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5564 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb5568 | load | 4 | `abs 0x1a731c` | `-` |
| `str` at 0xb556c | store | 4 | `sp -0xc0` | `[0x1a731c]` |
| `str` at 0xb5578 | store | 4 | `sp -0xc0` | `0x2` |
| `ldr` at 0xb557c | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0xb5580 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0xb5584 | store | 4 | `abs 0x1a731c` | `?` |
| `ldr` at 0xb5588 | load | 4 | `abs 0xb5600` | `-` |
| `ldr` at 0xb558c | load | 4 | `abs 0x3183ec` | `-` |
| `ldr` at 0xb5598 | load | 4 | `abs 0xb5600` | `-` |
| `ldr` at 0xb559c | load | 4 | `abs 0x3183ec` | `-` |
| `ldr` at 0xb55a8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb55ac | load | 4 | `abs 0x1a7318` | `-` |
| `ldr` at 0xb55b8 | load | 4 | `abs 0xb5608` | `-` |
| `ldr` at 0xb55bc | load | 4 | `abs 0x1a731c` | `-` |
| `str` at 0xb55cc | store | 4 | `sp -0xc4` | `0x4` |
| `str` at 0xb55d8 | store | 4 | `sp -0xc4` | `0x1` |
| `str` at 0xb55e4 | store | 4 | `sp -0xc4` | `0x2` |
| `ldr` at 0xb55e8 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0xb55f0 | load | 4 | `abs 0xb562c` | `-` |
| `ldm` at 0xb55fc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb55fc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb55fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb55fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb55fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb28b4 | `(unknown callee) 0x28230` | `0xb` | `r1` | `r2` | `[0x1a6fc0]` |
| `bl` at 0xb28d4 | `(unknown callee) 0x29908` | `0x16cd5c` | `?` | `?` | `?` |
| `bl` at 0xb28e0 | `(unknown callee) 0x22ca0` | `0x1a7258` | `?` | `?` | `?` |
| `bl` at 0xb28fc | `(unknown callee) 0x29908` | `0x16cd70` | `?` | `?` | `?` |
| `bl` at 0xb2908 | `(unknown callee) 0x23118` | `0x1a7060` | `0x0` | `?` | `?` |
| `bl` at 0xb2914 | `(unknown callee) 0x23118` | `0x1a715c` | `0x0` | `?` | `?` |
| `bl` at 0xb2920 | `(unknown callee) 0x23118` | `0x1a70b4` | `0x0` | `?` | `?` |
| `bl` at 0xb292c | `(unknown callee) 0x23118` | `0x1a71b0` | `0x0` | `?` | `?` |
| `bl` at 0xb2938 | `(unknown callee) 0x23118` | `0x1a7108` | `0x0` | `?` | `?` |
| `bl` at 0xb2944 | `(unknown callee) 0x23118` | `0x1a7204` | `0x0` | `?` | `?` |
| `bl` at 0xb2950 | `(unknown callee) 0x23118` | `0x1a72ac` | `0x1` | `?` | `?` |
| `bl` at 0xb2a20 | `(unknown callee) 0x530f4` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb2ab8 | `(unknown callee) 0x29908` | `0x16cd84` | `[sp-28]` | `?` | `?` |
| `bl` at 0xb2b90 | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xb2c0c | `(unknown callee) 0x62bfc` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb2ca4 | `(unknown callee) 0x29908` | `0x16cdb0` | `[sp-28]` | `?` | `?` |
| `bl` at 0xb2d7c | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xb2df8 | `(unknown callee) 0x7243c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb2e90 | `(unknown callee) 0x29908` | `0x16cdd4` | `[sp-28]` | `?` | `?` |
| `bl` at 0xb2f5c | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xb2fd8 | `(unknown callee) 0x111db8` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb3104 | `(unknown callee) 0x29908` | `0x16cdf8` | `[sp-28]` | `?` | `?` |
| `bl` at 0xb310c | `(unknown callee) 0x111a1c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb376c | `(unknown callee) 0x63d8c` | `[sp-28]` | `?` | `0x0` | `?` |
| `bl` at 0xb38d0 | `(unknown callee) 0x72e70` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb45a8 | `(unknown callee) 0x6e190` | `0x1a7318` | `[sp-28]` | `?` | `[0x1a6fc8]` |
| `bl` at 0xb4774 | `(unknown callee) 0x6e190` | `ret(0x6e190)` | `?` | `ret(0x6e190)` | `?` |
| `bl` at 0xb4b4c | `(unknown callee) 0x28230` | `0xc` | `?` | `?` | `?` |
| `bl` at 0xb5388 | `(unknown callee) 0x70b90` | `?` | `?` | `[sp-28]` | `[0x1a0380]` |
| `bl` at 0xb5390 | `(unknown callee) 0x70cf0` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xb5398 | `(unknown callee) 0x70cf0` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xb53a8 | `(unknown callee) 0x6dc0c` | `?` | `?` | `?` | `?` |
| `bl` at 0xb53d0 | `(unknown callee) 0x28230` | `0x6` | `?` | `?` | `?` |
| `bl` at 0xb5404 | `(unknown callee) 0xac0e4` | `?` | `?` | `?` | `[sp-44]` |
| `bl` at 0xb5414 | `(unknown callee) 0xac1d0` | `[sp-28]` | `?` | `?` | `ret(0xac0e4)` |
| `bl` at 0xb5480 | `(unknown callee) 0xaa4b8` | `[sp-28]` | `[sp-36]` | `[sp-184]` | `[sp-36]` |
| `bl` at 0xb54a8 | `(unknown callee) 0xaaf14` | `[sp-28]` | `[sp-28]` | `?` | `[sp-36]` |
| `bl` at 0xb551c | `(unknown callee) 0xabc84` | `[sp-28]` | `[sp-36]` | `[sp-180]` | `[sp-36]` |
| `mov` at 0xb55a4 | `(unknown callee) (unknown target)` | `?` | `?` | `0x1a7318` | `[0x3183ec]` |
| `bl` at 0xb55ec | `(unknown callee) 0xa4910` | `?` | `?` | `?` | `?` |
| `bl` at 0xb55f4 | `(unknown callee) 0x2302c` | `0x1a7258` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb28a4 | `[0x1a6fc0] != 0x0` | `[0x1a6fc0] == 0x0` |
| `beq` at 0xb28cc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb28f4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb295c | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xb29a4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xb2a6c | `? <= 0x64 (signed, 32-bit)` | `? > 0x64 (signed, 32-bit)` |
| `ble` at 0xb2c58 | `? <= 0x14 (signed, 32-bit)` | `? > 0x14 (signed, 32-bit)` |
| `ble` at 0xb2e44 | `? <= 0x2f (signed, 32-bit)` | `? > 0x2f (signed, 32-bit)` |
| `beq` at 0xb2ff0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xb3160 | `? <= [sp-36] (signed, 32-bit)` | `? > [sp-36] (signed, 32-bit)` |
| `ble` at 0xb3474 | `? <= [sp-36] (signed, 32-bit)` | `? > [sp-36] (signed, 32-bit)` |
| `ble` at 0xb38bc | `? <= [sp-36] (signed, 32-bit)` | `? > [sp-36] (signed, 32-bit)` |
| `ldrls` at 0xb3acc | `[sp-184]-101 <= 0x3 (unsigned, 32-bit)` | `[sp-184]-101 > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xb43fc | `[0x1a6fc8] == 0x0` | `[0x1a6fc8] != 0x0` |
| `bne` at 0xb45b8 | `ret(0x6e190) != [sp-28]` | `ret(0x6e190) == [sp-28]` |
| `beq` at 0xb45c8 | `[0x3183d0] == 0x0` | `[0x3183d0] != 0x0` |
| `beq` at 0xb4784 | `ret(0x6e190) == [sp-28]` | `ret(0x6e190) != [sp-28]` |
| `beq` at 0xb47a8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb4b58 | `ret(0x28230) == 0x0` | `ret(0x28230) != 0x0` |
| `beq` at 0xb5384 | `[0x1a0380] == 0x0` | `[0x1a0380] != 0x0` |
| `bne` at 0xb53b8 | `? != 0x35` | `? == 0x35` |
| `bge` at 0xb53fc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xb5470 | `? <= [sp-36] (signed, 32-bit)` | `? > [sp-36] (signed, 32-bit)` |
| `ble` at 0xb550c | `? <= [sp-36] (signed, 32-bit)` | `? > [sp-36] (signed, 32-bit)` |
| `bne` at 0xb5560 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xb5594 | `[0x3183ec] == 0x0` | `[0x3183ec] != 0x0` |
| `bne` at 0xb55b4 | `[0x1a7318] != 0x1` | `[0x1a7318] == 0x1` |
| `beq` at 0xb55c4 | `[0x1a731c] == 0x0` | `[0x1a731c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb55fc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb3acc).

## `EnvTestEvent`

- `EnvTestEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5630 to 0xb5764.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0xb5640).
  An unsigned entry r0 value at most 4 selects an indirect transfer whose target and load effective address remain unknown (`ldr` at 0xb5644, `ldrls` at 0xb564c).
  The known fallthrough jumps to 0xb5740 and returns entry r0 (`b` at 0xb5650, `ldm` at 0xb5744).
  Omitted effects beyond the unresolved transfer remain unknown (`ldrls` at 0xb564c).
  The meaning of entry r0 and the unresolved choices remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5640 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5644 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xb564c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xb5744 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5744 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5744 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xb564c | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5744 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb564c).

## `EnvTestTemp`

- `EnvTestTemp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5764 to 0xb57f4.
  Established. A word store writes 1 to absolute address 0x1a6fc8 (`str` at 0xb578c).
  Further word stores write [sp-20], [sp-24], [sp-28], [sp-32] and [sp] to absolute addresses 0x1a6fcc, 0x1a6fd0, 0x1a6fd4, 0x1a6fd8 and 0x1a6fdc respectively (`str` at 0xb5798, `str` at 0xb57a4, `str` at 0xb57b0, `str` at 0xb57bc, `str` at 0xb57c8).
  A call to unnamed 0xb286c has printed r0-r3 candidates unknown, entry r1, 0x1a6fdc and [sp] (`bl` at 0xb57d0).
  What the five copied words represent remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5774 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5778 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb577c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb5780 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb5784 | load | 4 | `abs 0xb57dc` | `-` |
| `str` at 0xb578c | store | 4 | `abs 0x1a6fc8` | `0x1` |
| `ldr` at 0xb5790 | load | 4 | `abs 0xb57e0` | `-` |
| `ldr` at 0xb5794 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5798 | store | 4 | `abs 0x1a6fcc` | `[sp-20]` |
| `ldr` at 0xb579c | load | 4 | `abs 0xb57e4` | `-` |
| `ldr` at 0xb57a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb57a4 | store | 4 | `abs 0x1a6fd0` | `[sp-24]` |
| `ldr` at 0xb57a8 | load | 4 | `abs 0xb57e8` | `-` |
| `ldr` at 0xb57ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb57b0 | store | 4 | `abs 0x1a6fd4` | `[sp-28]` |
| `ldr` at 0xb57b4 | load | 4 | `abs 0xb57ec` | `-` |
| `ldr` at 0xb57b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb57bc | store | 4 | `abs 0x1a6fd8` | `[sp-32]` |
| `ldr` at 0xb57c0 | load | 4 | `abs 0xb57f0` | `-` |
| `ldr` at 0xb57c4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb57c8 | store | 4 | `abs 0x1a6fdc` | `[sp]` |
| `ldm` at 0xb57d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb57d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb57d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb57d0 | `(unknown callee) 0xb286c` | `?` | `r1` | `0x1a6fdc` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb57d8 | `r0` | `ret(0xb286c)` |
