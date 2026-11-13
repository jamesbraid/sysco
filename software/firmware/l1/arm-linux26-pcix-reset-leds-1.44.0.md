# ARM Linux 2.6 PCI-X reset and LED routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-pcix-reset-leds-facts.txt) retain all tool sections.

## `pcixDown`

- `pcixDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79670 to 0x7995c.
  Established. Byte stores save the low bytes of entry r1 and r2 at sp-0x15 and sp-0x16 (`strb` at 0x79688, `strb` at 0x79690).
  The call to unnamed 0x7a3e4 has its result stored as a word at sp-0x28 (`bl` at 0x7977c, `str` at 0x79784).
  Another call targets unnamed 0x7cd40 and stores its result at sp-0x20 (`bl` at 0x79810, `str` at 0x79818).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79680 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x79688 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x79690 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x79694 | load | 4 | `abs 0x79950` | `-` |
| `ldr` at 0x79698 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7969c | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x796a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x796bc | load | 4 | `abs 0x79954` | `-` |
| `ldr` at 0x796c0 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x796c8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x796d4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x796e4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x796ec | load | 4 | `abs 0x79958` | `-` |
| `str` at 0x796f0 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x796f4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x796f8 | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x79700 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79714 | load | 4 | `abs 0x79954` | `-` |
| `ldr` at 0x79718 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x79720 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x79724 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x79734 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x79738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7973c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x79740 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7974c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79750 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79758 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79764 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x79768 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x79770 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x79778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79784 | store | 4 | `sp -0x28` | `ret(0x7a3e4)` |
| `str` at 0x7978c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x79790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79794 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x79798 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x797a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x797a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x797b0 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x797bc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x797c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x797c8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x797d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x797d4 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x797dc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x797e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x797ec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x797f4 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79800 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x79804 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79808 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79818 | store | 4 | `sp -0x20` | `ret(0x7cd40)` |
| `ldr` at 0x7981c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79828 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7982c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79834 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79840 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x79844 | load | 4 | `abs 0x79958` | `-` |
| `str` at 0x79848 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x79850 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79854 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x79860 | store | 4 | `sp -0x28` | `ret(0x7c018)` |
| `ldr` at 0x79864 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x79868 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79870 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x7987c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x79880 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79884 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x7988c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x79890 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79898 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x798a4 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x798a8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x798ac | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x798b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x798c0 | store | 4 | `sp -0x24` | `ret(0x7ce1c)` |
| `ldr` at 0x798c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x798d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x798d4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x798dc | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x798e8 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x798ec | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x798f0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x798f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79904 | store | 4 | `sp -0x30` | `ret(0x7cffc)` |
| `ldr` at 0x7990c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79910 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7991c | store | 4 | `sp -0x28` | `ret(0x7c0f4)` |
| `ldr` at 0x79920 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x79924 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7992c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79938 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x7993c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79940 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x79944 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x7994c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7994c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7994c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7977c | `(unknown callee) 0x7a3e4` | `[sp-20]` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0x79810 | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x79858 | `(unknown callee) 0x7c018` | `[sp-20]` | `ret(0x7cd40)` | `?` | `ret(0x7cd40)` |
| `bl` at 0x798b8 | `(unknown callee) 0x7ce1c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x798fc | `(unknown callee) 0x7cffc` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x79914 | `(unknown callee) 0x7c0f4` | `[sp-20]` | `ret(0x7ce1c)` | `?` | `ret(0x7ce1c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x796a4 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x796d0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x796dc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7972c | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x79748 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x797a0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x797e4 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x79824 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `bne` at 0x798cc | `ret(0x7ce1c) != 0x0` | `ret(0x7ce1c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7994c | `r0` | `?` |

## `pcixReset`

- `pcixReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7995c to 0x799e4.
  Established. The result of the call to unnamed 0x7a50c is stored as a word at sp-0x1c (`bl` at 0x79990, `str` at 0x79998).
  The branch takes 0x799b4 when that result is zero and falls through when it is nonzero (`beq` at 0x799a4).
  A later call to unnamed 0x7a808 has its result stored at sp-0x20 (`bl` at 0x799cc, `str` at 0x799d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7996c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x79974 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7997c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x79980 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79984 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x79988 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79998 | store | 4 | `sp -0x1c` | `ret(0x7a50c)` |
| `ldr` at 0x7999c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x799a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x799ac | store | 4 | `sp -0x20` | `ret(0x7a50c)` |
| `ldrb` at 0x799bc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x799c0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x799c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x799d4 | store | 4 | `sp -0x20` | `ret(0x7a808)` |
| `ldr` at 0x799d8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x799e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x799e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x799e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79990 | `(unknown callee) 0x7a50c` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x799b8 | `(unknown callee) 0x799e4` | `0x2` | `?` | `?` | `ret(0x7a50c)` |
| `bl` at 0x799cc | `(unknown callee) 0x7a808` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x799a4 | `ret(0x7a50c) == 0x0` | `ret(0x7a50c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x799e0 | `r0` | `?` |

## `pcixLedOn`

- `pcixLedOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79a0c to 0x79ca0.
  Established. A word store initializes sp-0x28 to zero (`str` at 0x79a34).
  Printed register candidates for the call to unnamed 0x7ddec include r0=[sp-20] and r2=1 (`bl` at 0x79b10);
  its result is stored as a word at sp-0x2c (`bl` at 0x79b10, `str` at 0x79b18).
  After the call to unnamed 0x7cd40, a byte load from its result plus one is stored as a word at sp-0x30 (`bl` at 0x79b70, `ldrb` at 0x79b98, `str` at 0x79b9c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79a1c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x79a24 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x79a2c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x79a34 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x79a38 | load | 4 | `abs 0x79c94` | `-` |
| `ldr` at 0x79a3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79a40 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x79a4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79a60 | load | 4 | `abs 0x79c98` | `-` |
| `ldr` at 0x79a64 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x79a6c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x79a78 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x79a88 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x79a90 | load | 4 | `abs 0x79c9c` | `-` |
| `str` at 0x79a94 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x79a98 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x79a9c | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x79aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79ab8 | load | 4 | `abs 0x79c98` | `-` |
| `ldr` at 0x79abc | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x79ac4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x79ac8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x79ad8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x79adc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79ae0 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x79ae4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79af0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79af4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79afc | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x79b04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79b18 | store | 4 | `sp -0x2c` | `ret(0x7ddec)` |
| `ldr` at 0x79b1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79b28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79b34 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x79b38 | store | 4 | `sp -0x28` | `ret(0x7ddec)` |
| `ldr` at 0x79b3c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x79b44 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x79b4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79b50 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x79b58 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x79b64 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79b68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79b78 | store | 4 | `sp -0x20` | `ret(0x7cd40)` |
| `ldr` at 0x79b7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79b88 | load | 4 | `abs 0x79c9c` | `-` |
| `str` at 0x79b8c | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x79b94 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x79b98 | load | 1 | `ret(0x7cd40) +0x1` | `-` |
| `str` at 0x79b9c | store | 4 | `sp -0x30` | `[ret(0x7cd40)+1]:1` |
| `ldr` at 0x79ba0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x79ba4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x79ba8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x79bac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x79bb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79bc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79bc4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79bcc | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x79bd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79be8 | store | 4 | `sp -0x2c` | `ret(0x7ddec)` |
| `ldr` at 0x79bec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79bf8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79c04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x79c08 | store | 4 | `sp -0x28` | `ret(0x7ddec)` |
| `ldr` at 0x79c0c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x79c14 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x79c1c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79c20 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x79c28 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79c2c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x79c30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79c40 | store | 4 | `sp -0x24` | `ret(0x7ce1c)` |
| `ldr` at 0x79c44 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x79c50 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79c54 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x79c58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79c68 | store | 4 | `sp -0x34` | `ret(0x7cffc)` |
| `ldr` at 0x79c70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79c74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x79c84 | store | 4 | `sp -0x34` | `ret(0x7ddec)` |
| `ldr` at 0x79c88 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x79c90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x79c90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x79c90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79b10 | `(unknown callee) 0x7ddec` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x79b70 | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x79be0 | `(unknown callee) 0x7ddec` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x79c38 | `(unknown callee) 0x7ce1c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x79c60 | `(unknown callee) 0x7cffc` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x79c7c | `(unknown callee) 0x7ddec` | `[sp-20]` | `ret(0x7ce1c)` | `0x1` | `ret(0x7ce1c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x79a48 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x79a74 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x79a80 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x79ad0 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x79aec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79b24 | `ret(0x7ddec) == 0x0` | `ret(0x7ddec) != 0x0` |
| `bne` at 0x79b30 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x79b60 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x79b84 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `ble` at 0x79bbc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79bf4 | `ret(0x7ddec) == 0x0` | `ret(0x7ddec) != 0x0` |
| `bne` at 0x79c00 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x79c4c | `ret(0x7ce1c) != 0x0` | `ret(0x7ce1c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x79c90 | `r0` | `?` |

## `pcixLedOff`

- `pcixLedOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79ca0 to 0x79f34.
  Established. The printed r2 candidate is 0 for the call to unnamed 0x7ddec at 0x79da4 and 1 for the later call at 0x79e74 (`bl` at 0x79da4, `bl` at 0x79e74).
  Word stores record those call results at sp-0x2c (`bl` at 0x79da4, `str` at 0x79dac, `bl` at 0x79e74, `str` at 0x79e7c).
  A call to unnamed 0x7cd40 supplies the base of a byte load at result plus one, whose value is stored as a word at sp-0x30 (`bl` at 0x79e04, `ldrb` at 0x79e2c, `str` at 0x79e30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79cb0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x79cb8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x79cc0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x79cc8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x79ccc | load | 4 | `abs 0x79f28` | `-` |
| `ldr` at 0x79cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79cd4 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x79ce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79cf4 | load | 4 | `abs 0x79f2c` | `-` |
| `ldr` at 0x79cf8 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x79d00 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x79d0c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x79d1c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x79d24 | load | 4 | `abs 0x79f30` | `-` |
| `str` at 0x79d28 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x79d2c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x79d30 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x79d38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79d4c | load | 4 | `abs 0x79f2c` | `-` |
| `ldr` at 0x79d50 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x79d58 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x79d5c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x79d6c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x79d70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79d74 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x79d78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79d84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79d88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79d90 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x79d98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79dac | store | 4 | `sp -0x2c` | `ret(0x7ddec)` |
| `ldr` at 0x79db0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79dbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79dc8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x79dcc | store | 4 | `sp -0x28` | `ret(0x7ddec)` |
| `ldr` at 0x79dd0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x79dd8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x79de0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79de4 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x79dec | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x79df8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79dfc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79e0c | store | 4 | `sp -0x20` | `ret(0x7cd40)` |
| `ldr` at 0x79e10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x79e1c | load | 4 | `abs 0x79f30` | `-` |
| `str` at 0x79e20 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x79e28 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x79e2c | load | 1 | `ret(0x7cd40) +0x1` | `-` |
| `str` at 0x79e30 | store | 4 | `sp -0x30` | `[ret(0x7cd40)+1]:1` |
| `ldr` at 0x79e34 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x79e38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x79e3c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x79e40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x79e48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79e54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79e58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x79e60 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x79e68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79e7c | store | 4 | `sp -0x2c` | `ret(0x7ddec)` |
| `ldr` at 0x79e80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79e8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x79e98 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x79e9c | store | 4 | `sp -0x28` | `ret(0x7ddec)` |
| `ldr` at 0x79ea0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x79ea8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x79eb0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79eb4 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x79ebc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79ec0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x79ec4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79ed4 | store | 4 | `sp -0x24` | `ret(0x7ce1c)` |
| `ldr` at 0x79ed8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x79ee4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79ee8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x79eec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79efc | store | 4 | `sp -0x34` | `ret(0x7cffc)` |
| `ldr` at 0x79f04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79f08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x79f18 | store | 4 | `sp -0x34` | `ret(0x7ddec)` |
| `ldr` at 0x79f1c | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x79f24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x79f24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x79f24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79da4 | `(unknown callee) 0x7ddec` | `[sp-20]` | `?` | `0x0` | `?` |
| `bl` at 0x79e04 | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x79e74 | `(unknown callee) 0x7ddec` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x79ecc | `(unknown callee) 0x7ce1c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x79ef4 | `(unknown callee) 0x7cffc` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x79f10 | `(unknown callee) 0x7ddec` | `[sp-20]` | `ret(0x7ce1c)` | `0x0` | `ret(0x7ce1c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x79cdc | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x79d08 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x79d14 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x79d64 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x79d80 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79db8 | `ret(0x7ddec) == 0x0` | `ret(0x7ddec) != 0x0` |
| `bne` at 0x79dc4 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x79df4 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x79e18 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `ble` at 0x79e50 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x79e88 | `ret(0x7ddec) == 0x0` | `ret(0x7ddec) != 0x0` |
| `bne` at 0x79e94 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x79ee0 | `ret(0x7ce1c) != 0x0` | `ret(0x7ce1c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x79f24 | `r0` | `?` |

## `pcixStatus`

- `pcixStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79f34 to 0x7a0d8.
  Established. Entry r3 is saved as a word at sp-0x1c, followed by byte saves of the low bytes of entry r1 and r2 (`str` at 0x79f48, `strb` at 0x79f50, `strb` at 0x79f58).
  Calls to unnamed 0x7ce1c and 0x7cffc have their results stored at sp-0x24 and sp-0x30 (`bl` at 0x79fdc, `str` at 0x79fe4, `bl` at 0x7a004, `str` at 0x7a00c).
  Printed candidates for the unknown-target call include r0=[sp-20], r1=ret(0x7cd40) and r2=[sp] (`bl` at 0x7a050, `mov` at 0x7a0b0);
  its result is stored as a word at sp-0x30 (`mov` at 0x7a0b0, `str` at 0x7a0b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79f44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x79f48 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x79f50 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x79f58 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x79f5c | load | 4 | `abs 0x7a0c8` | `-` |
| `ldr` at 0x79f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79f64 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x79f70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79f84 | load | 4 | `abs 0x7a0cc` | `-` |
| `ldr` at 0x79f88 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x79f90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79f9c | load | 4 | `abs 0x7a0d0` | `-` |
| `str` at 0x79fa0 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x79fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79fbc | load | 4 | `abs 0x7a0cc` | `-` |
| `ldr` at 0x79fc0 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x79fc8 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x79fcc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79fd0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x79fd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79fe4 | store | 4 | `sp -0x24` | `ret(0x7ce1c)` |
| `ldr` at 0x79fe8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x79ff4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79ff8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x79ffc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a00c | store | 4 | `sp -0x30` | `ret(0x7cffc)` |
| `ldr` at 0x7a014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a018 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a01c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7a028 | store | 4 | `sp -0x2c` | `ret(0x7d364)` |
| `ldr` at 0x7a02c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7a038 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7a03c | store | 4 | `sp -0x30` | `ret(0x7d364)` |
| `ldrb` at 0x7a044 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7a048 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a058 | store | 4 | `sp -0x28` | `ret(0x7cd40)` |
| `ldr` at 0x7a05c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a068 | load | 4 | `abs 0x7a0d0` | `-` |
| `str` at 0x7a06c | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x7a074 | load | 4 | `abs 0x7a0d4` | `-` |
| `ldr` at 0x7a078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a07c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x7a09c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a0a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a0a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a0a8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x7a0b8 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0x7a0bc | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x7a0c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a0c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a0c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79fdc | `(unknown callee) 0x7ce1c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7a004 | `(unknown callee) 0x7cffc` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7a020 | `(unknown callee) 0x7d364` | `[sp-20]` | `ret(0x7ce1c)` | `[sp-28]` | `ret(0x7ce1c)` |
| `bl` at 0x7a050 | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `mov` at 0x7a0b0 | `(unknown callee) (unknown target)` | `[sp-20]` | `ret(0x7cd40)` | `[sp]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x79f6c | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x79f98 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x79ff0 | `ret(0x7ce1c) != 0x0` | `ret(0x7ce1c) == 0x0` |
| `beq` at 0x7a034 | `ret(0x7d364) == 0x0` | `ret(0x7d364) != 0x0` |
| `bne` at 0x7a064 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a0c4 | `r0` | `?` |

## `pcixBusInReset`

- `pcixBusInReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a0d8 to 0x7a2bc.
  Established. Calls to unnamed 0x7cd40 and 0x7d364 have their results stored as words at sp-0x20 and sp-0x30 (`bl` at 0x7a170, `str` at 0x7a178, `bl` at 0x7a1f8, `str` at 0x7a200).
  Word stores of 0xfffff703 and 0xfffff702 both target sp-0x38 (`str` at 0x7a18c, `str` at 0x7a1e0).
  A load, add and store sequence writes [sp-52]+1 to sp-0x34 (`ldr` at 0x7a288, `add` at 0x7a28c, `str` at 0x7a290).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a0e8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7a0f0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7a0f4 | load | 4 | `abs 0x7a2ac` | `-` |
| `ldr` at 0x7a0f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a0fc | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7a108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a11c | load | 4 | `abs 0x7a2b0` | `-` |
| `ldr` at 0x7a120 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7a128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7a134 | load | 4 | `abs 0x7a2b4` | `-` |
| `str` at 0x7a138 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x7a140 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a154 | load | 4 | `abs 0x7a2b0` | `-` |
| `ldr` at 0x7a158 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7a160 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x7a164 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7a168 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a178 | store | 4 | `sp -0x20` | `ret(0x7cd40)` |
| `ldr` at 0x7a17c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a188 | load | 4 | `abs 0x7a2b4` | `-` |
| `str` at 0x7a18c | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x7a198 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x7a19c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7a1a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7a1a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7a1b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a1b4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7a1b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7a1bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7a1c8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x7a1d0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7a1dc | load | 4 | `abs 0x7a2b8` | `-` |
| `str` at 0x7a1e0 | store | 4 | `sp -0x38` | `0xfffff702` |
| `ldr` at 0x7a1ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a1f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7a200 | store | 4 | `sp -0x30` | `ret(0x7d364)` |
| `ldr` at 0x7a204 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a210 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7a214 | store | 4 | `sp -0x38` | `ret(0x7d364)` |
| `ldrb` at 0x7a228 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7a248 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7a268 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7a280 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x7a288 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7a290 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `str` at 0x7a29c | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x7a2a0 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x7a2a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a2a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a2a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a170 | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x7a1f8 | `(unknown callee) 0x7d364` | `[sp-20]` | `[sp-36]` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7a104 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7a130 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7a184 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `ble` at 0x7a1ac | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x7a1d8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7a20c | `ret(0x7d364) == 0x0` | `ret(0x7d364) != 0x0` |
| `beq` at 0x7a238 | `? == 0x3` | `? != 0x3` |
| `bne` at 0x7a258 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7a278 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a2a8 | `r0` | `?` |

## `pcixPowerUp`

- `pcixPowerUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a2bc to 0x7a3e4.
  Established. A zero word initializes sp-0x1c (`str` at 0x7a2d4).
  The call to unnamed 0x7b8f0 stores its result at sp-0x20, and a later load and store copy that result to sp-0x1c (`bl` at 0x7a38c, `str` at 0x7a394, `ldr` at 0x7a3b0, `str` at 0x7a3b4).
  The sequence at the loop tail loads sp-0x24, adds one and stores [sp-36]+1 there (`ldr` at 0x7a3b8, `add` at 0x7a3bc, `str` at 0x7a3c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a2cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7a2d4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7a2d8 | load | 4 | `abs 0x7a3dc` | `-` |
| `ldr` at 0x7a2dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a2e0 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7a2ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a300 | load | 4 | `abs 0x7a3e0` | `-` |
| `ldr` at 0x7a304 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7a30c | load | 4 | `(unknown)` | `-` |
| `str` at 0x7a31c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7a324 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a338 | load | 4 | `abs 0x7a3e0` | `-` |
| `ldr` at 0x7a33c | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7a344 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x7a34c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7a350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a354 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x7a358 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a368 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a37c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x7a384 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a394 | store | 4 | `sp -0x20` | `ret(0x7b8f0)` |
| `ldr` at 0x7a398 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a3a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a3b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7a3b4 | store | 4 | `sp -0x1c` | `ret(0x7b8f0)` |
| `ldr` at 0x7a3b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7a3c0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x7a3c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7a3cc | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x7a3d0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x7a3d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a3d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a3d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a38c | `(unknown callee) 0x7b8f0` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7a2e8 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7a314 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7a360 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7a3a0 | `ret(0x7b8f0) == 0x0` | `ret(0x7b8f0) != 0x0` |
| `bne` at 0x7a3ac | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a3d8 | `r0` | `?` |

## `pcixPowerDown`

- `pcixPowerDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a3e4 to 0x7a50c.
  Established. Sp-0x1c receives an initial zero word (`str` at 0x7a3fc).
  Unnamed 0x7c018 is called, with its result stored at sp-0x20 and later copied to sp-0x1c (`bl` at 0x7a4b4, `str` at 0x7a4bc, `ldr` at 0x7a4d8, `str` at 0x7a4dc).
  A word store at sp-0x24 records [sp-36]+1 after a load and add (`ldr` at 0x7a4e0, `add` at 0x7a4e4, `str` at 0x7a4e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a3f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7a3fc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7a400 | load | 4 | `abs 0x7a504` | `-` |
| `ldr` at 0x7a404 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a408 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7a414 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a428 | load | 4 | `abs 0x7a508` | `-` |
| `ldr` at 0x7a42c | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7a434 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7a444 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7a44c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a460 | load | 4 | `abs 0x7a508` | `-` |
| `ldr` at 0x7a464 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7a46c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x7a474 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7a478 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a47c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x7a480 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a48c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7a490 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a4a4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x7a4ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a4bc | store | 4 | `sp -0x20` | `ret(0x7c018)` |
| `ldr` at 0x7a4c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a4cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a4d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7a4dc | store | 4 | `sp -0x1c` | `ret(0x7c018)` |
| `ldr` at 0x7a4e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7a4e8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x7a4f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7a4f4 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x7a4f8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x7a500 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a500 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a500 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a4b4 | `(unknown callee) 0x7c018` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7a410 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7a43c | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7a488 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7a4c8 | `ret(0x7c018) == 0x0` | `ret(0x7c018) != 0x0` |
| `bne` at 0x7a4d4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a500 | `r0` | `?` |

## `pcixResetHold`

- `pcixResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a50c to 0x7a808.
  Established. The initial store at sp-0x28 writes a zero word (`str` at 0x7a534).
  The printed r2 candidate for the call to unnamed 0x7c204 is 1 (`bl` at 0x7a610).
  That call result is stored as a word at sp-0x2c (`bl` at 0x7a610, `str` at 0x7a618).
  A later call to unnamed 0x7cd40 supplies a byte-load address at result plus one, and the byte value is stored as a word at sp-0x30 (`bl` at 0x7a6d8, `ldrb` at 0x7a700, `str` at 0x7a704).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a51c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7a524 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7a52c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7a534 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7a538 | load | 4 | `abs 0x7a7fc` | `-` |
| `ldr` at 0x7a53c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a540 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7a54c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a560 | load | 4 | `abs 0x7a800` | `-` |
| `ldr` at 0x7a564 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7a56c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x7a578 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7a588 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x7a590 | load | 4 | `abs 0x7a804` | `-` |
| `str` at 0x7a594 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x7a598 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x7a59c | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x7a5a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a5b8 | load | 4 | `abs 0x7a800` | `-` |
| `ldr` at 0x7a5bc | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7a5c4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x7a5c8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7a5d8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x7a5dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a5e0 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x7a5e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a5f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a5f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a5fc | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x7a604 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a618 | store | 4 | `sp -0x2c` | `ret(0x7c204)` |
| `ldr` at 0x7a61c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7a628 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a634 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7a638 | store | 4 | `sp -0x28` | `ret(0x7c204)` |
| `ldr` at 0x7a63c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7a644 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0x7a650 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x7a654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a658 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x7a65c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a668 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a66c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a674 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x7a67c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a680 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a694 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x7a69c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x7a6a0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x7a6a4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7a6ac | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x7a6b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7a6b8 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x7a6c0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7a6cc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7a6d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a6e0 | store | 4 | `sp -0x20` | `ret(0x7cd40)` |
| `ldr` at 0x7a6e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a6f0 | load | 4 | `abs 0x7a804` | `-` |
| `str` at 0x7a6f4 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x7a6fc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7a700 | load | 1 | `ret(0x7cd40) +0x1` | `-` |
| `str` at 0x7a704 | store | 4 | `sp -0x30` | `[ret(0x7cd40)+1]:1` |
| `ldr` at 0x7a708 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7a70c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7a710 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7a714 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7a71c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a72c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a734 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x7a73c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a750 | store | 4 | `sp -0x2c` | `ret(0x7c204)` |
| `ldr` at 0x7a754 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7a760 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a76c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7a770 | store | 4 | `sp -0x28` | `ret(0x7c204)` |
| `ldr` at 0x7a774 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7a77c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x7a784 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7a788 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x7a790 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7a794 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7a798 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a7a8 | store | 4 | `sp -0x24` | `ret(0x7ce1c)` |
| `ldr` at 0x7a7ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7a7b8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7a7bc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7a7c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a7d0 | store | 4 | `sp -0x34` | `ret(0x7cffc)` |
| `ldr` at 0x7a7d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a7dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7a7ec | store | 4 | `sp -0x34` | `ret(0x7c204)` |
| `ldr` at 0x7a7f0 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x7a7f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7a7f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7a7f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a610 | `(unknown callee) 0x7c204` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x7a6d8 | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x7a748 | `(unknown callee) 0x7c204` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x7a7a0 | `(unknown callee) 0x7ce1c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7a7c8 | `(unknown callee) 0x7cffc` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7a7e4 | `(unknown callee) 0x7c204` | `[sp-20]` | `ret(0x7ce1c)` | `0x0` | `ret(0x7ce1c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7a548 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7a574 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7a580 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7a5d0 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x7a5ec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7a624 | `ret(0x7c204) == 0x0` | `ret(0x7c204) != 0x0` |
| `bne` at 0x7a630 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `ble` at 0x7a664 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x7a6c8 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x7a6ec | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `ble` at 0x7a724 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7a75c | `ret(0x7c204) == 0x0` | `ret(0x7c204) != 0x0` |
| `bne` at 0x7a768 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x7a7b4 | `ret(0x7ce1c) != 0x0` | `ret(0x7ce1c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7a7f8 | `r0` | `?` |

## `pcixResetRelease`

- `pcixResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a808 to 0x7aa40.
  Established. Sp-0x28 starts with a zero word (`str` at 0x7a830).
  Two calls to unnamed 0x7c62c store their results at sp-0x2c (`bl` at 0x7a914, `str` at 0x7a91c, `bl` at 0x7a9a0, `str` at 0x7a9a8).
  The call to unnamed 0x7cd40 at 0x7a974 has its result stored at sp-0x24 (`bl` at 0x7a974, `str` at 0x7a97c).
  The final listed call targets unnamed 0x7c7e0 and writes its result at sp-0x34 (`bl` at 0x7aa1c, `str` at 0x7aa24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a818 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7a820 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7a828 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x7a830 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7a834 | load | 4 | `abs 0x7aa34` | `-` |
| `ldr` at 0x7a838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a83c | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7a848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a85c | load | 4 | `abs 0x7aa38` | `-` |
| `ldr` at 0x7a860 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7a868 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x7a874 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7a884 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x7a88c | load | 4 | `abs 0x7aa3c` | `-` |
| `str` at 0x7a890 | store | 4 | `sp -0x38` | `0xfffff703` |
| `ldr` at 0x7a894 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x7a898 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x7a8a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a8b4 | load | 4 | `abs 0x7aa38` | `-` |
| `ldr` at 0x7a8b8 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7a8c0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x7a8c4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7a8d4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x7a8d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a8dc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x7a8e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a8ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a8f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7a904 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x7a90c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a91c | store | 4 | `sp -0x2c` | `ret(0x7c62c)` |
| `ldr` at 0x7a920 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7a92c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a938 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7a93c | store | 4 | `sp -0x28` | `ret(0x7c62c)` |
| `ldr` at 0x7a940 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7a948 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x7a950 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7a954 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x7a95c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7a968 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7a96c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a97c | store | 4 | `sp -0x24` | `ret(0x7cd40)` |
| `ldr` at 0x7a980 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7a98c | load | 4 | `abs 0x7aa3c` | `-` |
| `str` at 0x7a990 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x7a998 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7a99c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7a9a8 | store | 4 | `sp -0x2c` | `ret(0x7c62c)` |
| `ldr` at 0x7a9ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7a9b8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7a9bc | store | 4 | `sp -0x28` | `ret(0x7c62c)` |
| `ldr` at 0x7a9c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7a9c4 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x7a9cc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7a9d0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7a9d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7a9e4 | store | 4 | `sp -0x20` | `ret(0x7ce1c)` |
| `ldr` at 0x7a9e8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7a9f4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7a9f8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7a9fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7aa0c | store | 4 | `sp -0x34` | `ret(0x7cffc)` |
| `ldr` at 0x7aa14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7aa18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7aa24 | store | 4 | `sp -0x34` | `ret(0x7c7e0)` |
| `ldr` at 0x7aa28 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x7aa30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7aa30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7aa30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a914 | `(unknown callee) 0x7c62c` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x7a974 | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x7a9a0 | `(unknown callee) 0x7c62c` | `[sp-20]` | `ret(0x7cd40)` | `?` | `ret(0x7cd40)` |
| `bl` at 0x7a9dc | `(unknown callee) 0x7ce1c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7aa04 | `(unknown callee) 0x7cffc` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7aa1c | `(unknown callee) 0x7c7e0` | `[sp-20]` | `ret(0x7ce1c)` | `?` | `ret(0x7ce1c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7a844 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7a870 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7a87c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7a8cc | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x7a8e8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7a928 | `ret(0x7c62c) == 0x0` | `ret(0x7c62c) != 0x0` |
| `bne` at 0x7a934 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x7a964 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x7a988 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `beq` at 0x7a9b4 | `ret(0x7c62c) == 0x0` | `ret(0x7c62c) != 0x0` |
| `bne` at 0x7a9f0 | `ret(0x7ce1c) != 0x0` | `ret(0x7ce1c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7aa30 | `r0` | `?` |

## `pcixInt`

- `pcixInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7aa40 to 0x7ad5c.
  Established. A call to unnamed 0x7d0e0 has its result stored at sp-0x20 (`bl` at 0x7aae4, `str` at 0x7aaec).
  Byte loads from that result and result plus one are stored as words at sp-0x3c and sp-0x38 (`bl` at 0x7aae4, `ldrb` at 0x7abc8, `str` at 0x7abcc, `ldrb` at 0x7abd4, `str` at 0x7abd8).
  The call to unnamed 0x2031c is followed by an add of one and a word store of ret(0x2031c)+1 at sp-0x2c (`bl` at 0x7ab18, `add` at 0x7ab20, `str` at 0x7ab24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7aa50 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7aa5c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7aa64 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x7aa6c | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x7aa70 | load | 4 | `abs 0x7ad30` | `-` |
| `ldr` at 0x7aa74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7aa78 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7aa84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7aa98 | load | 4 | `abs 0x7ad34` | `-` |
| `ldr` at 0x7aa9c | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7aaa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7aab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7aac8 | load | 4 | `abs 0x7ad34` | `-` |
| `ldr` at 0x7aacc | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7aad4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x7aad8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7aadc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7aaec | store | 4 | `sp -0x20` | `ret(0x7d0e0)` |
| `ldr` at 0x7aaf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7aafc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7ab10 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x7ab14 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7ab24 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x7ab28 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7ab50 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x7ab54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7ab58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ab5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7ab68 | load | 4 | `abs 0x7ad38` | `-` |
| `str` at 0x7ab6c | store | 4 | `(unknown)` | `0x1660f4` |
| `str` at 0x7ab70 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x7ab74 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7ab78 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x7ab80 | load | 4 | `abs 0x7ad3c` | `-` |
| `ldrb` at 0x7ab90 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7ab94 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7ab98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7aba8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7abac | load | 1 | `ret(0x7d0e0) +0x6` | `-` |
| `ldr` at 0x7abbc | load | 4 | `abs 0x7ad40` | `-` |
| `str` at 0x7abc0 | store | 4 | `sp -0x40` | `0x166124` |
| `ldr` at 0x7abc4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7abc8 | load | 1 | `ret(0x7d0e0) +0x0` | `-` |
| `str` at 0x7abcc | store | 4 | `sp -0x3c` | `[ret(0x7d0e0)]:1` |
| `ldr` at 0x7abd0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7abd4 | load | 1 | `ret(0x7d0e0) +0x1` | `-` |
| `str` at 0x7abd8 | store | 4 | `sp -0x38` | `[ret(0x7d0e0)+1]:1` |
| `ldr` at 0x7abe0 | load | 4 | `abs 0x7ad3c` | `-` |
| `ldr` at 0x7abe4 | load | 4 | `abs 0x7ad44` | `-` |
| `ldrb` at 0x7abf0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7abf4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7abf8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7ac08 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7ac18 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x7ac28 | load | 4 | `abs 0x7ad48` | `-` |
| `str` at 0x7ac2c | store | 4 | `sp -0x40` | `0x166150` |
| `ldr` at 0x7ac30 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7ac34 | load | 1 | `ret(0x7d0e0) +0x0` | `-` |
| `str` at 0x7ac38 | store | 4 | `sp -0x3c` | `[ret(0x7d0e0)]:1` |
| `ldr` at 0x7ac3c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7ac40 | load | 1 | `ret(0x7d0e0) +0x1` | `-` |
| `str` at 0x7ac44 | store | 4 | `sp -0x38` | `[ret(0x7d0e0)+1]:1` |
| `ldr` at 0x7ac4c | load | 4 | `abs 0x7ad3c` | `-` |
| `ldr` at 0x7ac50 | load | 4 | `abs 0x7ad4c` | `-` |
| `ldr` at 0x7ac5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7ac60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7ac6c | store | 4 | `sp -0x24` | `ret(0x7c0f4)` |
| `ldr` at 0x7ac70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7ac7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7ac88 | load | 4 | `abs 0x7ad50` | `-` |
| `str` at 0x7ac8c | store | 4 | `sp -0x40` | `0x166170` |
| `ldr` at 0x7ac90 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7ac94 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x7ac98 | store | 4 | `sp -0x3c` | `[[sp-32]]:1` |
| `ldr` at 0x7ac9c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7aca0 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x7aca4 | store | 4 | `sp -0x38` | `[[sp-32]+1]:1` |
| `str` at 0x7aca8 | store | 4 | `sp -0x34` | `ret(0x21ce0)` |
| `ldr` at 0x7acb0 | load | 4 | `abs 0x7ad3c` | `-` |
| `ldr` at 0x7acb4 | load | 4 | `abs 0x7ad54` | `-` |
| `ldr` at 0x7acc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7acc4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7acd4 | store | 4 | `sp -0x24` | `ret(0x7ddec)` |
| `ldr` at 0x7acd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7ace4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7acf0 | load | 4 | `abs 0x7ad58` | `-` |
| `str` at 0x7acf4 | store | 4 | `sp -0x40` | `0x166198` |
| `ldr` at 0x7acf8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7acfc | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x7ad00 | store | 4 | `sp -0x3c` | `[[sp-32]]:1` |
| `ldr` at 0x7ad04 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7ad08 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x7ad0c | store | 4 | `sp -0x38` | `[[sp-32]+1]:1` |
| `str` at 0x7ad10 | store | 4 | `sp -0x34` | `ret(0x21ce0)` |
| `ldr` at 0x7ad18 | load | 4 | `abs 0x7ad3c` | `-` |
| `ldm` at 0x7ad2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7ad2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7ad2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7aae4 | `(unknown callee) 0x7d0e0` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x7ab0c | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7ab18 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7ab60 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x7ab8c | `(unknown callee) 0x22070` | `0x1` | `0x166044` | `0x334` | `0x0` |
| `bl` at 0x7aba0 | `(unknown callee) 0x125cf8` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7abec | `(unknown callee) 0x22070` | `0x1` | `0x166044` | `0x33f` | `0x0` |
| `bl` at 0x7ac00 | `(unknown callee) 0x125cf8` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x7ac58 | `(unknown callee) 0x22070` | `0x1` | `0x166044` | `0x34a` | `0x0` |
| `bl` at 0x7ac64 | `(unknown callee) 0x7c0f4` | `[sp-20]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x7ac80 | `(unknown callee) 0x21ce0` | `ret(0x7c0f4)` | `?` | `?` | `ret(0x7c0f4)` |
| `bl` at 0x7acbc | `(unknown callee) 0x22070` | `0x1` | `0x166044` | `0x351` | `0x0` |
| `bl` at 0x7accc | `(unknown callee) 0x7ddec` | `[sp-20]` | `[sp-32]` | `0x1` | `?` |
| `bl` at 0x7ace8 | `(unknown callee) 0x21ce0` | `ret(0x7ddec)` | `?` | `?` | `ret(0x7ddec)` |
| `bl` at 0x7ad24 | `(unknown callee) 0x22070` | `0x1` | `0x166044` | `0x358` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7aa80 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7aaac | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7aaf8 | `ret(0x7d0e0) != 0x0` | `ret(0x7d0e0) == 0x0` |
| `beq` at 0x7abb8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ac14 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7ac24 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7ac78 | `ret(0x7c0f4) == 0x0` | `ret(0x7c0f4) != 0x0` |
| `beq` at 0x7ace0 | `ret(0x7ddec) == 0x0` | `ret(0x7ddec) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7ad2c | `r0` | `?` |

## `pcixPresentInt`

- `pcixPresentInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7ad5c to 0x7af1c.
  Established. A byte store initializes sp-0x22 to zero, and another byte store writes one there (`strb` at 0x7ad90, `strb` at 0x7adb8).
  The call to unnamed 0x7d27c yields the base for a byte load at result plus six (`bl` at 0x7adcc, `ldrb` at 0x7addc).
  One call to unnamed 0x2031c is followed by an add of one and a word store of ret(0x2031c)+1 at sp-0x2c (`bl` at 0x7ae1c, `add` at 0x7ae24, `str` at 0x7ae28).
  The call at 0x7aeb0 also has its result incremented and stored at sp-0x2c (`bl` at 0x7aeb0, `add` at 0x7aeb8, `str` at 0x7aebc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7ad6c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7ad78 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7ad80 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x7ad88 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x7ad90 | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0x7ad94 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7ad98 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x7ada0 | store | 1 | `sp -0x21` | `?` |
| `ldrb` at 0x7ada4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x7adb8 | store | 1 | `sp -0x22` | `0x1` |
| `ldrb` at 0x7adbc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7adc0 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x7adc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7add4 | store | 4 | `sp -0x1c` | `ret(0x7d27c)` |
| `ldr` at 0x7add8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7addc | load | 1 | `ret(0x7d27c) +0x6` | `-` |
| `ldrb` at 0x7adf0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7ae00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7ae14 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x7ae18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7ae28 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x7ae2c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7ae54 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x7ae58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7ae5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ae60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7ae6c | load | 4 | `abs 0x7af14` | `-` |
| `ldrb` at 0x7ae7c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7ae94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7aea8 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x7aeac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7aebc | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x7aec0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7aee8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7aeec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7aef0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7aef4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7af00 | load | 4 | `abs 0x7af18` | `-` |
| `ldm` at 0x7af10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7af10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7af10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7adcc | `(unknown callee) 0x7d27c` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x7ae10 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7ae1c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7ae64 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x7ae74 | `(unknown callee) 0x29908` | `0x1661c0` | `ret(0x203f4)` | `?` | `ret(0x203f4)` |
| `bl` at 0x7aea4 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7aeb0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7aef8 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-48]` | `[sp-44]` | `?` |
| `bl` at 0x7af08 | `(unknown callee) 0x29908` | `0x1661f8` | `ret(0x203f4)` | `?` | `ret(0x203f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7adb0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ade8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7adfc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ae90 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7af10 | `r0` | `?` |

## `pcixSetBusSpeed`

- `pcixSetBusSpeed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7af1c to 0x7b064.
  Established. The branch takes 0x7af78 when entry r0 is signed-greater-than-or-equal to the word loaded from absolute 0x1a6220, and falls through when it is less (`ldr` at 0x7af40, `bge` at 0x7af48).
  The single call targets unnamed 0x7d69c and stores its result at sp-0x20 (`bl` at 0x7b00c, `str` at 0x7b014).
  A later load and store copy that result to sp-0x1c (`bl` at 0x7b00c, `ldr` at 0x7b030, `str` at 0x7b034).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7af2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7af34 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7af38 | load | 4 | `abs 0x7b05c` | `-` |
| `ldr` at 0x7af3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7af40 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7af4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7af60 | load | 4 | `abs 0x7b060` | `-` |
| `ldr` at 0x7af64 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7af6c | load | 4 | `(unknown)` | `-` |
| `str` at 0x7af7c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x7af84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7af98 | load | 4 | `abs 0x7b060` | `-` |
| `ldr` at 0x7af9c | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7afa4 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x7afac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7afb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7afb4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x7afb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7afc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7afc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7afd0 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldrb` at 0x7afd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7afe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7afe8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7affc | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x7b004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7b014 | store | 4 | `sp -0x20` | `ret(0x7d69c)` |
| `ldr` at 0x7b018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7b024 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b030 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7b034 | store | 4 | `sp -0x1c` | `ret(0x7d69c)` |
| `ldr` at 0x7b038 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7b040 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7b048 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7b04c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x7b050 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x7b058 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b058 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b058 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7b00c | `(unknown callee) 0x7d69c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7af48 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7af74 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7afc0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x7afe0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x7b020 | `ret(0x7d69c) == 0x0` | `ret(0x7d69c) != 0x0` |
| `bne` at 0x7b02c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b058 | `r0` | `?` |

## `pcixSetM66EN`

- `pcixSetM66EN`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b064 to 0x7b21c.
  Established. Zero word stores initialize sp-0x24 and sp-0x30 (`str` at 0x7b080, `str` at 0x7b088).
  Printed candidates for the call to unnamed 0x799e4 include r0=0xa, r1=[sp-28] and r3=[sp-48] (`bl` at 0x7b1a8).
  A load and add precede a store of [sp-48]+1 at sp-0x30 (`ldr` at 0x7b1ac, `add` at 0x7b1b0, `str` at 0x7b1b4).
  A call to unnamed 0x7de7c has its result stored as a word at sp-0x28 (`bl` at 0x7b1c4, `str` at 0x7b1cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b074 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7b078 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x7b080 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x7b088 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x7b08c | load | 4 | `abs 0x7b214` | `-` |
| `ldr` at 0x7b090 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b094 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7b0a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b0b4 | load | 4 | `abs 0x7b218` | `-` |
| `ldr` at 0x7b0b8 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7b0c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7b0d0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x7b0d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b0ec | load | 4 | `abs 0x7b218` | `-` |
| `ldr` at 0x7b0f0 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b0f8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x7b100 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x7b104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b108 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x7b10c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7b118 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b11c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7b124 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x7b12c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7b130 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7b134 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b148 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7b14c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b15c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7b160 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b16c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b170 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7b178 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x7b180 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b18c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7b198 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7b1ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7b1b4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x7b1b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b1bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7b1c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7b1cc | store | 4 | `sp -0x28` | `ret(0x7de7c)` |
| `ldr` at 0x7b1d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7b1dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7b1e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7b1ec | store | 4 | `sp -0x24` | `ret(0x7de7c)` |
| `ldr` at 0x7b1f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7b1f8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x7b200 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7b204 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x7b208 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x7b210 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b210 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b210 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7b1a8 | `(unknown callee) 0x799e4` | `0xa` | `[sp-28]` | `?` | `[sp-48]` |
| `bl` at 0x7b1c4 | `(unknown callee) 0x7de7c` | `[sp-20]` | `?` | `[sp-24]` | `[sp-48]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7b09c | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7b0c8 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7b114 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7b140 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7b158 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7b168 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x7b188 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7b194 | `[sp-24] != 0x1` | `[sp-24] == 0x1` |
| `bne` at 0x7b1a0 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `beq` at 0x7b1d8 | `ret(0x7de7c) == 0x0` | `ret(0x7de7c) != 0x0` |
| `bne` at 0x7b1e4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b210 | `r0` | `?` |

## `pcixBusPopulation`

- `pcixBusPopulation`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b21c to 0x7b3a8.
  Established. A byte load from the result of unnamed 0x7cd40 plus one is stored as a word at sp-0x34 (`bl` at 0x7b2bc, `ldrb` at 0x7b2e4, `str` at 0x7b2e8).
  Printed candidates for the later call to unnamed 0x7d364 include r2=sp-40 (`bl` at 0x7b32c).
  That call result is stored as a word at sp-0x30 (`bl` at 0x7b32c, `str` at 0x7b334).
  Two load, add and store sequences write [sp-44]+1 to sp-0x2c and [sp-52]+1 to sp-0x34 (`ldr` at 0x7b370, `add` at 0x7b374, `str` at 0x7b378, `ldr` at 0x7b37c, `add` at 0x7b380, `str` at 0x7b384).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b22c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7b234 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x7b23c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x7b240 | load | 4 | `abs 0x7b3a0` | `-` |
| `ldr` at 0x7b244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b248 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x7b254 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b268 | load | 4 | `abs 0x7b3a4` | `-` |
| `ldr` at 0x7b26c | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7b274 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7b284 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x7b28c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7b2a0 | load | 4 | `abs 0x7b3a4` | `-` |
| `ldr` at 0x7b2a4 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7b2ac | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x7b2b0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7b2b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7b2c4 | store | 4 | `sp -0x20` | `ret(0x7cd40)` |
| `ldr` at 0x7b2c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7b2d8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x7b2e0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7b2e4 | load | 1 | `ret(0x7cd40) +0x1` | `-` |
| `str` at 0x7b2e8 | store | 4 | `sp -0x34` | `[ret(0x7cd40)+1]:1` |
| `ldr` at 0x7b2ec | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7b2f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b2f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7b2f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b300 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7b30c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7b310 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7b318 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x7b324 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7b334 | store | 4 | `sp -0x30` | `ret(0x7d364)` |
| `ldr` at 0x7b338 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7b344 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7b348 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldrb` at 0x7b35c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7b370 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7b378 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x7b37c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7b384 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x7b38c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7b390 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0x7b394 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x7b39c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7b39c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7b39c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7b2bc | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x7b32c | `(unknown callee) 0x7d364` | `[sp-20]` | `?` | `sp-40` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7b250 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7b27c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7b2d0 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `ble` at 0x7b308 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7b340 | `ret(0x7d364) == 0x0` | `ret(0x7d364) != 0x0` |
| `beq` at 0x7b36c | `? == 0x3` | `? != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7b39c | `r0` | `?` |
