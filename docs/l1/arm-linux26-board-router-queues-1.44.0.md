# ARM Linux 2.6 board links and router queues in 1.44.0

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
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers in B2BInit and B2BReqRsp leave unresolved edges. Their tables cover
known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-board-router-queues-facts.txt) retain all tool sections.

## `B2BInit`

- `B2BInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5fcc to 0xa6d48.
  Established. Known-edge facts show word stores of 0xfff, 1 and 0 to absolute 0x1a6fac (`str` at 0xa6024, `str` at 0xa6030, `str` at 0xa603c).
  Two calls to unnamed 0x20550 print candidate r0/r1/r2 values 0x31b45c/0/0x144 and 0x31b478/unknown/0x94 (`bl` at 0xa605c, `bl` at 0xa606c).
  The conditional transfer is unresolved (`ldrls` at 0xa607c), so later unreached listing effects remain unestablished.
  Board-link initialization choices and physical device roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa5fdc | load | 4 | `abs 0xa6cbc` | `-` |
| `ldr` at 0xa5fe0 | load | 4 | `abs 0x1a6fa4` | `-` |
| `ldr` at 0xa5fec | load | 4 | `abs 0xa6cc0` | `-` |
| `str` at 0xa5ff0 | store | 4 | `sp -0x64` | `0x16b5bc` |
| `ldr` at 0xa5ff8 | load | 4 | `abs 0xa6cc4` | `-` |
| `ldr` at 0xa6008 | load | 4 | `abs 0xa6cbc` | `-` |
| `ldr` at 0xa600c | load | 4 | `abs 0xa6cbc` | `-` |
| `ldr` at 0xa6010 | load | 4 | `abs 0x1a6fa4` | `-` |
| `str` at 0xa6018 | store | 4 | `abs 0x1a6fa4` | `[0x1a6fa4]+1` |
| `ldr` at 0xa601c | load | 4 | `abs 0xa6cc8` | `-` |
| `ldr` at 0xa6020 | load | 4 | `abs 0xa6ccc` | `-` |
| `str` at 0xa6024 | store | 4 | `abs 0x1a6fac` | `0xfff` |
| `ldr` at 0xa6028 | load | 4 | `abs 0xa6cc8` | `-` |
| `str` at 0xa6030 | store | 4 | `abs 0x1a6fac` | `0x1` |
| `ldr` at 0xa6034 | load | 4 | `abs 0xa6cc8` | `-` |
| `str` at 0xa603c | store | 4 | `abs 0x1a6fac` | `0x0` |
| `str` at 0xa604c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa6050 | load | 4 | `abs 0xa6cd0` | `-` |
| `ldr` at 0xa6060 | load | 4 | `abs 0xa6cd4` | `-` |
| `ldr` at 0xa6070 | load | 4 | `sp -0x34` | `-` |
| `ldrls` at 0xa607c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa6518 | load | 4 | `abs 0xa6cd8` | `-` |
| `ldr` at 0xa651c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa6520 | load | 4 | `abs 0xa6cdc` | `-` |
| `ldm` at 0xa6cb8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa6cb8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa6cb8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa6cb8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa6cb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6004 | `(unknown callee) 0x22070` | `0x9` | `0x16b5d8` | `0x84` | `0x0` |
| `bl` at 0xa6040 | `(unknown callee) 0x6dc0c` | `?` | `?` | `0x1a6fac` | `0x0` |
| `bl` at 0xa605c | `(unknown callee) 0x20550` | `0x31b45c` | `0x0` | `0x144` | `?` |
| `bl` at 0xa606c | `(unknown callee) 0x20550` | `0x31b478` | `?` | `0x94` | `?` |
| `bl` at 0xa6524 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b5f0` | `?` | `0x1a288c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa5fe8 | `[0x1a6fa4] == 0x0` | `[0x1a6fa4] != 0x0` |
| `ldrls` at 0xa607c | `[sp-52]-36 <= 0x59 (unsigned, 32-bit)` | `[sp-52]-36 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa6cb8 | `r0` | `ret(0x244d4)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa607c).

## `B2BEventMesh`

- `B2BEventMesh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa794c to 0xa83d4.
  Established. Word loads read entry r0 +0xc and +0x1c, and a word store writes [sp-20] to absolute 0x1b1020 (`ldr` at 0xa796c, `ldr` at 0xa7978, `str` at 0xa7c38).
  A call to unnamed 0xa3e30 prints candidate r0/r1/r2/r3 values sp-32/sp-48/sp-52/sp-32 (`bl` at 0xa7d7c).
  Another call to unnamed 0xfcb5c prints [sp-76]/[sp-20]/unknown/unknown (`bl` at 0xa802c).
  Mesh-event field meanings and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa795c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7964 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa7968 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa796c | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xa7970 | store | 4 | `sp -0x20` | `[r0+12]` |
| `ldr` at 0xa7974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7978 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7984 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7988 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa7990 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa7994 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa799c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa79a0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa79b0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa79b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa79bc | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa79c4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa79c8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa79d8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa79dc | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa79e0 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0xa79e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa79e8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa79f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa79f8 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa7a00 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7a04 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa7a0c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7a10 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa7a20 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7a28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7a2c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa7a34 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7a38 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa7a48 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa7a4c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa7a50 | store | 4 | `sp -0x28` | `[sp-84]` |
| `ldr` at 0xa7a54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7a58 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7a64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7a68 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa7a70 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa7a74 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa7a7c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa7a80 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa7a90 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa7a98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7a9c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa7aa4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa7aa8 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa7ab8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa7abc | load | 4 | `sp -0x58` | `-` |
| `str` at 0xa7ac0 | store | 4 | `sp -0x2c` | `[sp-88]` |
| `ldr` at 0xa7ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7ac8 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0xa7ad8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xa7adc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7ae0 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0xa7aec | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa7af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7af4 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0xa7b00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b04 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0xa7b18 | store | 4 | `sp -0x5c` | `0xe` |
| `str` at 0xa7b24 | store | 4 | `sp -0x5c` | `0xf` |
| `ldr` at 0xa7b2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b30 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0xa7b38 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa7b3c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa7b44 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa7b48 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xa7b4c | store | 4 | `sp -0x38` | `[sp-92]` |
| `ldr` at 0xa7b50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b54 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa7b64 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xa7b68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b6c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa7b78 | store | 4 | `sp -0x40` | `?` |
| `str` at 0xa7b80 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xa7b84 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7b88 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7b98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7b9c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7ba8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7bac | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7bc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7bc8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7bd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7bdc | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7bec | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7c04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7c08 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa7c1c | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7c20 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7c24 | load | 4 | `abs 0x1a6fac` | `-` |
| `str` at 0xa7c2c | store | 4 | `abs 0x1a6fac` | `?` |
| `ldr` at 0xa7c30 | load | 4 | `abs 0xa8370` | `-` |
| `ldr` at 0xa7c34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7c38 | store | 4 | `abs 0x1b1020` | `[sp-20]` |
| `ldr` at 0xa7c3c | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7c40 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7c50 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7c54 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa7c58 | load | 4 | `abs 0xa8378` | `-` |
| `ldr` at 0xa7c5c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7c60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa7c68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7c78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7c90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7c98 | store | 4 | `[sp-20] +0x18` | `0x0` |
| `str` at 0xa7ca0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xa7ca8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xa7cac | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7cb0 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7cc0 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7cc4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa7cc8 | load | 4 | `abs 0xa837c` | `-` |
| `ldr` at 0xa7ccc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa7cd0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa7cd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7ce4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa7cf0 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xa7cf8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa7cfc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7d08 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa7d14 | load | 4 | `abs 0xa8380` | `-` |
| `ldr` at 0xa7d18 | load | 4 | `abs 0x1a6fb4` | `-` |
| `ldr` at 0xa7d1c | load | 4 | `[0x1a6fb4] +0x58` | `-` |
| `ldr` at 0xa7d28 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7d34 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7d38 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7d48 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7d4c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa7d50 | load | 4 | `abs 0xa8384` | `-` |
| `ldr` at 0xa7d58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7d80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7d8c | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7d90 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7da0 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7da4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa7da8 | load | 4 | `abs 0xa8388` | `-` |
| `ldr` at 0xa7db0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7dbc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7dcc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa7de4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa7de8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7e04 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7e08 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7e18 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7e1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa7e20 | store | 4 | `sp -0x7c` | `[sp-32]` |
| `ldr` at 0xa7e24 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa7e28 | load | 4 | `abs 0xa838c` | `-` |
| `ldr` at 0xa7e2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7e30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa7e38 | load | 4 | `abs 0xa8390` | `-` |
| `ldr` at 0xa7e3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7e40 | load | 4 | `abs 0x1a6fb8` | `-` |
| `ldr` at 0xa7e4c | load | 4 | `abs 0xa8394` | `-` |
| `ldr` at 0xa7e50 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa7e54 | load | 4 | `abs 0x1a6fbc` | `-` |
| `ldr` at 0xa7e60 | load | 4 | `abs 0xa8398` | `-` |
| `str` at 0xa7e64 | store | 4 | `sp -0x4c` | `0x31a744` |
| `ldr` at 0xa7e68 | load | 4 | `abs 0xa8380` | `-` |
| `ldr` at 0xa7e6c | load | 4 | `abs 0x1a6fb4` | `-` |
| `ldr` at 0xa7e70 | load | 4 | `[0x1a6fb4] +0x58` | `-` |
| `ldr` at 0xa7e98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7ea4 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7ea8 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7eb8 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7ebc | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa7ec0 | load | 4 | `abs 0xa839c` | `-` |
| `ldr` at 0xa7ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7ed4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa7ee4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa7efc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa7f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7f20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7f24 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa7f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7f34 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa7f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7f50 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa7f64 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7f68 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7f78 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7f7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7f80 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa7f90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7f94 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa7fa0 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa7fa4 | store | 4 | `sp -0x7c` | `[sp-72]` |
| `ldr` at 0xa7fa8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa7fac | load | 4 | `abs 0xa83a0` | `-` |
| `ldr` at 0xa7fb8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa7fc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7fd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7fdc | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa7fe0 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa7ff0 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa7ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7ff8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa8008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa800c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xa8018 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa801c | load | 4 | `abs 0xa83a4` | `-` |
| `ldr` at 0xa8024 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8028 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8034 | store | 4 | `sp -0x1c` | `ret(0xfcb5c)` |
| `ldr` at 0xa8038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8044 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8050 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa8054 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8064 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8070 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa8074 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa8078 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0xa807c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa8080 | load | 4 | `abs 0xa83a8` | `-` |
| `ldr` at 0xa8090 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa809c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa80a8 | load | 4 | `abs 0xa83ac` | `-` |
| `ldr` at 0xa80b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa80b4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa80c0 | store | 4 | `sp -0x18` | `ret(0xa3c34)` |
| `ldr` at 0xa80c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa80d4 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa80d8 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa80e8 | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa80ec | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa80f0 | load | 4 | `abs 0xa83b0` | `-` |
| `ldr` at 0xa80f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa80f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa8100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa810c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8110 | load | 4 | `ret(0xa3c34) +0x10` | `-` |
| `str` at 0xa8114 | store | 4 | `sp -0x20` | `[ret(0xa3c34)+16]` |
| `ldr` at 0xa811c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8130 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa8134 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8144 | load | 4 | `abs 0xa8374` | `-` |
| `str` at 0xa8148 | store | 4 | `sp -0x60` | `0x1a288c` |
| `ldr` at 0xa814c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8158 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8164 | load | 4 | `abs 0xa83b4` | `-` |
| `str` at 0xa8168 | store | 4 | `sp -0x64` | `0x16b9c4` |
| `ldr` at 0xa8170 | load | 4 | `abs 0xa83b8` | `-` |
| `str` at 0xa8174 | store | 4 | `sp -0x64` | `0x16b9c8` |
| `ldr` at 0xa817c | load | 4 | `abs 0xa83bc` | `-` |
| `str` at 0xa8180 | store | 4 | `sp -0x64` | `0x16b9cc` |
| `ldr` at 0xa8184 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa8188 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0xa818c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa8190 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0xa8194 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa8198 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0xa819c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa81a0 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xa81a4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa81a8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa81ac | load | 4 | `abs 0xa83c0` | `-` |
| `ldr` at 0xa81b0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa81b4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa81bc | load | 4 | `abs 0xa83c4` | `-` |
| `ldr` at 0xa81c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa81d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa81e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa81ec | store | 4 | `sp -0x1c` | `ret(0xfcb5c)` |
| `ldr` at 0xa81f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa81fc | load | 4 | `abs 0xa83c4` | `-` |
| `ldr` at 0xa8200 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8214 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa822c | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa8230 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8240 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa824c | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa8250 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa8254 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0xa8258 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa825c | load | 4 | `abs 0xa83c8` | `-` |
| `ldr` at 0xa826c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8278 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa827c | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa828c | load | 4 | `abs 0xa8374` | `-` |
| `ldr` at 0xa8290 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa8294 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0xa8298 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa829c | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0xa82a0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa82a4 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0xa82a8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa82ac | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xa82b0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa82b4 | load | 4 | `abs 0xa83cc` | `-` |
| `ldr` at 0xa82b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa82bc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa82c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa82d0 | load | 4 | `abs 0xa836c` | `-` |
| `ldr` at 0xa82d4 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa82e4 | load | 4 | `abs 0xa8374` | `-` |
| `str` at 0xa82e8 | store | 4 | `sp -0x68` | `0x1a288c` |
| `ldr` at 0xa82ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa82f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8304 | load | 4 | `abs 0xa83b4` | `-` |
| `str` at 0xa8308 | store | 4 | `sp -0x6c` | `0x16b9c4` |
| `ldr` at 0xa8310 | load | 4 | `abs 0xa83b8` | `-` |
| `str` at 0xa8314 | store | 4 | `sp -0x6c` | `0x16b9c8` |
| `ldr` at 0xa831c | load | 4 | `abs 0xa83bc` | `-` |
| `str` at 0xa8320 | store | 4 | `sp -0x6c` | `0x16b9cc` |
| `ldr` at 0xa8324 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa8328 | store | 4 | `sp -0x7c` | `[sp-48]` |
| `ldr` at 0xa832c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa8330 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0xa8334 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa8338 | store | 4 | `sp -0x74` | `[sp-60]` |
| `ldr` at 0xa833c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa8340 | store | 4 | `sp -0x70` | `[sp-64]` |
| `ldr` at 0xa8344 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xa8348 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa834c | load | 4 | `abs 0xa83d0` | `-` |
| `ldr` at 0xa8350 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0xa8354 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa835c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa8368 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8368 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8368 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7c64 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b814` | `[sp-60]` | `[sp-64]` |
| `bl` at 0xa7c7c | `(unknown callee) 0xa3fa0` | `?` | `?` | `?` | `?` |
| `bl` at 0xa7cd4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b834` | `[sp-60]` | `[sp-64]` |
| `bl` at 0xa7ce8 | `(unknown callee) 0xa1b8c` | `[sp-20]` | `?` | `?` | `[sp-36]` |
| `bl` at 0xa7d54 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b860` | `?` | `0x1a288c` |
| `bl` at 0xa7d5c | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa7d7c | `(unknown callee) 0xa3e30` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0xa7dac | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b878` | `?` | `0x1a288c` |
| `bl` at 0xa7db4 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa7e00 | `(unknown callee) 0x203f4` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0xa7e34 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b8a4` | `[sp-48]` | `[sp-52]` |
| `bl` at 0xa7e94 | `(unknown callee) 0xa3e30` | `sp-32` | `sp-48` | `sp-52` | `sp-32` |
| `bl` at 0xa7ec4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b8c0` | `?` | `0x1a288c` |
| `bl` at 0xa7ecc | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa7f18 | `(unknown callee) 0x203f4` | `[sp-20]+24` | `sp-68` | `0x4` | `[sp-20]+24` |
| `bl` at 0xa7fb4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b8f4` | `?` | `?` |
| `bl` at 0xa7fc8 | `(unknown callee) 0xa1cec` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xa7fd4 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `[sp-72]` |
| `bl` at 0xa8020 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b92c` | `?` | `?` |
| `bl` at 0xa802c | `(unknown callee) 0xfcb5c` | `[sp-76]` | `[sp-20]` | `?` | `?` |
| `bl` at 0xa8048 | `(unknown callee) 0xb6148` | `[sp-76]` | `?` | `?` | `ret(0xfcb5c)` |
| `bl` at 0xa8068 | `(unknown callee) 0x219a4` | `ret(0xfcb5c)` | `?` | `?` | `?` |
| `bl` at 0xa808c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b954` | `[[sp-20]+22]:1` | `ret(0x219a4)` |
| `bl` at 0xa8094 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa80ac | `(unknown callee) 0x9adfc` | `0x16b980` | `?` | `?` | `[sp-32]` |
| `bl` at 0xa80b8 | `(unknown callee) 0xa3c34` | `[sp-48]` | `[sp-52]` | `?` | `?` |
| `bl` at 0xa80d0 | `(unknown callee) 0x9afe8` | `ret(0xa3c34)` | `?` | `?` | `ret(0xa3c34)` |
| `bl` at 0xa80fc | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b990` | `[sp-48]` | `[sp-52]` |
| `bl` at 0xa8104 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa8118 | `(unknown callee) 0x9afe8` | `ret(0xa3c34)` | `?` | `?` | `[ret(0xa3c34)+16]` |
| `bl` at 0xa8120 | `(unknown callee) 0xa92d8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa81b8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b9d0` | `?` | `[sp-56]` |
| `bl` at 0xa81e4 | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa8224 | `(unknown callee) 0xb6148` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa8244 | `(unknown callee) 0x219a4` | `ret(0xfcb5c)` | `?` | `?` | `?` |
| `bl` at 0xa8268 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ba00` | `[[sp-20]+22]:1` | `ret(0x219a4)` |
| `bl` at 0xa8270 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa82c0 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ba2c` | `[sp-32]` | `[sp-56]` |
| `bl` at 0xa82c8 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa8358 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ba60` | `?` | `[sp-56]` |
| `bl` at 0xa8360 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa7980 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0xa79f0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0xa7a60 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bge` at 0xa7afc | `[r0+24] >= 0x0 (signed, 32-bit)` | `[r0+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xa7b10 | `? != 0x2` | `? == 0x2` |
| `beq` at 0xa7b94 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xa7ba4 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0xa7bbc | `? == 0x5` | `? != 0x5` |
| `beq` at 0xa7bd4 | `? == 0x5` | `? != 0x5` |
| `bge` at 0xa7be4 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0xa7bfc | `? == 0x4` | `? != 0x4` |
| `beq` at 0xa7c14 | `? == 0x4` | `? != 0x4` |
| `beq` at 0xa7c4c | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xa7c74 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xa7c88 | `ret(0xa3fa0) != 0x0` | `ret(0xa3fa0) == 0x0` |
| `beq` at 0xa7cbc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa7ce0 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0xa7d04 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0xa7d10 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `bne` at 0xa7d24 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa7d30 | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0xa7d44 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa7d88 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xa7d9c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa7e14 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa7e48 | `[sp-48] != [0x1a6fb8]` | `[sp-48] == [0x1a6fb8]` |
| `bne` at 0xa7e5c | `[sp-52] != [0x1a6fbc]` | `[sp-52] == [0x1a6fbc]` |
| `beq` at 0xa7e78 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xa7ea0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xa7eb4 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xa7f2c | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0xa7f44 | `? == 0xe` | `? != 0xe` |
| `beq` at 0xa7f5c | `? == 0xe` | `? != 0xe` |
| `beq` at 0xa7f74 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa7fc0 | `[sp-72] != 0x0` | `[sp-72] == 0x0` |
| `beq` at 0xa7fec | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa8040 | `ret(0xfcb5c) != 0x0` | `ret(0xfcb5c) == 0x0` |
| `beq` at 0xa8060 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa80a4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa80cc | `ret(0xa3c34) != 0x0` | `ret(0xa3c34) == 0x0` |
| `beq` at 0xa80e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa812c | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0xa8140 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa8154 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0xa8160 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xa81f8 | `ret(0xfcb5c) != 0x0` | `ret(0xfcb5c) == 0x0` |
| `beq` at 0xa823c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa8288 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa82e0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa82f4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0xa8300 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8368 | `r0` | `?` |

## `B2BEventRing`

- `B2BEventRing`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa83d4 to 0xa8c68.
  Established. Word loads read entry r0 +0xc, +0x18 and +0x1c (`ldr` at 0xa83f4, `ldr` at 0xa8400, `ldr` at 0xa842c).
  A word store has unknown effective address and value [0x1a6fa8] (`str` at 0xa850c).
  Calls to unnamed 0xa3eac print candidate r0 values 1 and 2 on one path (`bl` at 0xa8b34, `bl` at 0xa8b44).
  Ring-event field meanings and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa83e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa83ec | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa83f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa83f4 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xa83f8 | store | 4 | `sp -0x1c` | `[r0+12]` |
| `ldr` at 0xa83fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8400 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0xa8410 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa8414 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8418 | load | 4 | `r0 +0x18` | `-` |
| `str` at 0xa8424 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa8428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa842c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa843c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa8440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8444 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa8450 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xa8454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8458 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa8464 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8468 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa8470 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa8474 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa847c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa8480 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa8490 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa8498 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa849c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa84a4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa84a8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa84b8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xa84bc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa84c0 | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0xa84c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa84d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa84dc | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa84e0 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa84f0 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa84f4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa84f8 | load | 4 | `abs 0xa8c34` | `-` |
| `ldr` at 0xa8500 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8504 | load | 4 | `abs 0xa8c38` | `-` |
| `ldr` at 0xa8508 | load | 4 | `abs 0x1a6fa8` | `-` |
| `str` at 0xa850c | store | 4 | `(unknown)` | `[0x1a6fa8]` |
| `ldr` at 0xa8514 | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa8518 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8528 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa852c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8530 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa8540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8544 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa8550 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa8554 | load | 4 | `abs 0xa8c3c` | `-` |
| `ldr` at 0xa855c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8568 | load | 4 | `abs 0xa8c40` | `-` |
| `ldr` at 0xa856c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8574 | load | 4 | `abs 0xa8c44` | `-` |
| `ldr` at 0xa8578 | load | 4 | `abs 0x1a6fb4` | `-` |
| `ldr` at 0xa857c | load | 4 | `[0x1a6fb4] +0x58` | `-` |
| `ldr` at 0xa8588 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa8594 | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa8598 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa85a8 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa85ac | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa85b0 | load | 4 | `abs 0xa8c48` | `-` |
| `ldr` at 0xa85b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa85c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa85d4 | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa85d8 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa85e8 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa85ec | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa85f0 | load | 4 | `abs 0xa8c4c` | `-` |
| `ldr` at 0xa85f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa85fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa860c | store | 4 | `sp -0x44` | `0x2` |
| `str` at 0xa8618 | store | 4 | `sp -0x44` | `0x1` |
| `ldr` at 0xa861c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa8620 | store | 4 | `sp -0x1c` | `[sp-68]` |
| `ldr` at 0xa8624 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa8630 | store | 4 | `sp -0x38` | `ret(0xa3eac)` |
| `ldr` at 0xa8634 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8648 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa864c | load | 4 | `[sp-56] +0x4` | `-` |
| `ldr` at 0xa8658 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa865c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa8668 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa866c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa8678 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa867c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa8688 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa868c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa8698 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa869c | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa86a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa86ac | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa86b8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa86bc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa86c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa86cc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa86d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa86dc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa86e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa86ec | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa86f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa86fc | load | 4 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xa870c | load | 4 | `abs 0xa8c40` | `-` |
| `ldr` at 0xa8710 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8724 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa873c | store | 4 | `sp -0x20` | `ret(0xfcb5c)` |
| `ldr` at 0xa8740 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa875c | load | 4 | `abs 0xa8c50` | `-` |
| `ldr` at 0xa8760 | load | 4 | `abs 0x1a662c` | `-` |
| `ldr` at 0xa876c | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa8770 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8780 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa8784 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa8788 | load | 4 | `abs 0xa8c54` | `-` |
| `ldr` at 0xa8790 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xa87a0 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xa87a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa87b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa87c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa87d0 | load | 4 | `abs 0xa8c40` | `-` |
| `ldr` at 0xa87d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa87e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa87f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8814 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa881c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa8824 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8834 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xa8838 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8844 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8858 | load | 4 | `abs 0xa8c40` | `-` |
| `ldr` at 0xa885c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8870 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa887c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8894 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa889c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa88a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa88b4 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xa88b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa88c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa88d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa88e4 | store | 4 | `sp -0x38` | `ret(0xa3eac)` |
| `ldr` at 0xa88e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa88ec | load | 4 | `ret(0xa3eac) +0x4` | `-` |
| `ldr` at 0xa88f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa88fc | load | 4 | `ret(0xa3eac) +0x78` | `-` |
| `ldr` at 0xa8908 | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa890c | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa891c | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa8920 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa8924 | load | 4 | `abs 0xa8c58` | `-` |
| `ldr` at 0xa8928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8930 | load | 4 | `abs 0xa8c40` | `-` |
| `ldr` at 0xa8934 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8948 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8954 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8960 | store | 4 | `sp -0x20` | `ret(0xfcb5c)` |
| `ldr` at 0xa8964 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8980 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa8988 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0xa8994 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xa8998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa89a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa89b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa89c4 | store | 4 | `sp -0x38` | `ret(0xa3eac)` |
| `ldr` at 0xa89c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa89cc | load | 4 | `ret(0xa3eac) +0x4` | `-` |
| `ldr` at 0xa89d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa89dc | load | 4 | `ret(0xa3eac) +0x10` | `-` |
| `ldr` at 0xa89e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa89ec | load | 4 | `ret(0xa3eac) +0x10` | `-` |
| `ldr` at 0xa89f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa89fc | load | 4 | `ret(0xa3eac) +0x10` | `-` |
| `ldr` at 0xa8a08 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa8a0c | load | 4 | `ret(0xa3eac) +0x10` | `-` |
| `ldr` at 0xa8a18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa8a1c | load | 4 | `ret(0xa3eac) +0x10` | `-` |
| `ldr` at 0xa8a28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa8a2c | load | 4 | `ret(0xa3eac) +0x10` | `-` |
| `ldr` at 0xa8a38 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa8a3c | load | 4 | `ret(0xa3eac) +0x10` | `-` |
| `ldr` at 0xa8a4c | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa8a50 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8a60 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa8a64 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa8a68 | load | 4 | `abs 0xa8c5c` | `-` |
| `ldr` at 0xa8a6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8a74 | load | 4 | `abs 0xa8c40` | `-` |
| `ldr` at 0xa8a78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8a8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8a98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa8aa4 | store | 4 | `sp -0x20` | `ret(0xfcb5c)` |
| `ldr` at 0xa8aa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8ac4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa8acc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa8ad4 | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa8ad8 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8ae8 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa8aec | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa8af0 | load | 4 | `abs 0xa8c60` | `-` |
| `str` at 0xa8afc | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xa8b00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8b0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8b24 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa8b3c | store | 4 | `sp -0x38` | `ret(0xa3eac)` |
| `str` at 0xa8b4c | store | 4 | `sp -0x3c` | `ret(0xa3eac)` |
| `ldr` at 0xa8b50 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa8b54 | load | 4 | `ret(0xa3eac) +0x4` | `-` |
| `ldr` at 0xa8b60 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa8b64 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa8b68 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0xa8b6c | load | 4 | `ret(0xa3eac) +0x8` | `-` |
| `ldr` at 0xa8b78 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa8b7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa8b80 | load | 4 | `[sp-56] +0xc` | `-` |
| `ldr` at 0xa8b84 | load | 4 | `ret(0xa3eac) +0xc` | `-` |
| `ldr` at 0xa8b94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8ba0 | load | 4 | `abs 0xa8c40` | `-` |
| `ldr` at 0xa8ba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8bb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa8bc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8bd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8be4 | load | 4 | `abs 0xa8c2c` | `-` |
| `ldr` at 0xa8be8 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa8bf8 | load | 4 | `abs 0xa8c30` | `-` |
| `ldr` at 0xa8bfc | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa8c00 | load | 4 | `abs 0xa8c64` | `-` |
| `ldr` at 0xa8c04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8c0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa8c14 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa8c1c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa8c28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa8c28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa8c28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa84fc | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ba90` | `r2` | `0x1a288c` |
| `bl` at 0xa8558 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16baa8` | `?` | `?` |
| `bl` at 0xa8564 | `(unknown callee) 0xf9a70` | `?` | `0x1` | `?` | `?` |
| `bl` at 0xa8570 | `(unknown callee) 0xfca84` | `0x31b45c` | `[sp-20]` | `?` | `?` |
| `bl` at 0xa85b4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16b860` | `?` | `0x1a288c` |
| `bl` at 0xa85bc | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa85f8 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bacc` | `[sp-28]` | `0x1a288c` |
| `bl` at 0xa8628 | `(unknown callee) 0xa3eac` | `[sp-68]` | `?` | `?` | `[sp-68]` |
| `bl` at 0xa8638 | `(unknown callee) 0xa92d8` | `[sp-28]` | `?` | `?` | `ret(0xa3eac)` |
| `bl` at 0xa8734 | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa8754 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa878c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bae8` | `?` | `0x1a288c` |
| `bl` at 0xa87b4 | `(unknown callee) 0xa92d8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa87cc | `(unknown callee) 0xf9a70` | `[sp-20]` | `0x1` | `?` | `ret(0xa92d8)` |
| `bl` at 0xa87f8 | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa8810 | `(unknown callee) 0xf9a70` | `[sp-20]` | `?` | `?` | `ret(0xfcb5c)` |
| `bl` at 0xa8828 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa8848 | `(unknown callee) 0xa92d8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa8880 | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa88a8 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa88c8 | `(unknown callee) 0xa92d8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa88dc | `(unknown callee) 0xa3eac` | `[sp-28]` | `?` | `?` | `ret(0xa92d8)` |
| `bl` at 0xa892c | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bb14` | `[sp-28]` | `0x1a288c` |
| `bl` at 0xa8958 | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa8978 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `ret(0xfcb5c)` |
| `bl` at 0xa89a8 | `(unknown callee) 0xa92d8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa89bc | `(unknown callee) 0xa3eac` | `[sp-28]` | `?` | `?` | `ret(0xa92d8)` |
| `bl` at 0xa8a70 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bb38` | `[sp-28]` | `0x1a288c` |
| `bl` at 0xa8a9c | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa8abc | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `ret(0xfcb5c)` |
| `bl` at 0xa8af4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bb5c` | `?` | `0x1a288c` |
| `bl` at 0xa8b10 | `(unknown callee) 0xa92d8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa8b34 | `(unknown callee) 0xa3eac` | `0x1` | `?` | `?` | `[sp-28]` |
| `bl` at 0xa8b44 | `(unknown callee) 0xa3eac` | `0x2` | `?` | `?` | `ret(0xa3eac)` |
| `bl` at 0xa8b9c | `(unknown callee) 0xf9a70` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa8bc8 | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa8be0 | `(unknown callee) 0xf9a70` | `[sp-20]` | `?` | `?` | `ret(0xfcb5c)` |
| `bl` at 0xa8c08 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bb78` | `[sp-28]` | `0x1a288c` |
| `bl` at 0xa8c20 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa8460 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0xa84cc | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0xa84d8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xa84ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa8524 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa8584 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa8590 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0xa85a4 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xa85d0 | `[sp-28]-1 > 0x1 (unsigned, 32-bit)` | `[sp-28]-1 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xa85e4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa8604 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `beq` at 0xa8644 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0xa8654 | `[[sp-56]+4] == 0x0` | `[[sp-56]+4] != 0x0` |
| `beq` at 0xa8664 | `[[sp-56]+16] == 0x35` | `[[sp-56]+16] != 0x35` |
| `beq` at 0xa8674 | `[[sp-56]+16] == 0x4f` | `[[sp-56]+16] != 0x4f` |
| `beq` at 0xa8684 | `[[sp-56]+16] == 0x42` | `[[sp-56]+16] != 0x42` |
| `beq` at 0xa8694 | `[[sp-56]+16] == 0x46` | `[[sp-56]+16] != 0x46` |
| `beq` at 0xa86a4 | `[[sp-56]+16] == 0x4a` | `[[sp-56]+16] != 0x4a` |
| `beq` at 0xa86b4 | `[[sp-56]+16] == 0x4b` | `[[sp-56]+16] != 0x4b` |
| `beq` at 0xa86c4 | `[[sp-56]+16] == 0x21` | `[[sp-56]+16] != 0x21` |
| `beq` at 0xa86d4 | `[[sp-56]+16] == 0x5e` | `[[sp-56]+16] != 0x5e` |
| `beq` at 0xa86e4 | `[[sp-56]+16] == 0x2b` | `[[sp-56]+16] != 0x2b` |
| `beq` at 0xa86f4 | `[[sp-56]+16] == 0x4c` | `[[sp-56]+16] != 0x4c` |
| `beq` at 0xa8704 | `[[sp-56]+16] == 0x4d` | `[[sp-56]+16] != 0x4d` |
| `bne` at 0xa8748 | `ret(0xfcb5c) != 0x0` | `ret(0xfcb5c) == 0x0` |
| `ble` at 0xa8768 | `[0x1a662c] <= 0x0 (signed, 32-bit)` | `[0x1a662c] > 0x0 (signed, 32-bit)` |
| `beq` at 0xa877c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa8798 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `bgt` at 0xa87ac | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xa87c0 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0xa8804 | `ret(0xfcb5c) == 0x0` | `ret(0xfcb5c) != 0x0` |
| `bgt` at 0xa8840 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xa8854 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `bne` at 0xa888c | `ret(0xfcb5c) != 0x0` | `ret(0xfcb5c) == 0x0` |
| `bgt` at 0xa88c0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xa88d4 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0xa88f4 | `[ret(0xa3eac)+4] == 0x0` | `[ret(0xa3eac)+4] != 0x0` |
| `bne` at 0xa8904 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa8918 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa896c | `ret(0xfcb5c) != 0x0` | `ret(0xfcb5c) == 0x0` |
| `bgt` at 0xa89a0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xa89b4 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `beq` at 0xa89d4 | `[ret(0xa3eac)+4] == 0x0` | `[ret(0xa3eac)+4] != 0x0` |
| `beq` at 0xa89e4 | `[ret(0xa3eac)+16] == 0x52` | `[ret(0xa3eac)+16] != 0x52` |
| `beq` at 0xa89f4 | `[ret(0xa3eac)+16] == 0x36` | `[ret(0xa3eac)+16] != 0x36` |
| `beq` at 0xa8a04 | `[ret(0xa3eac)+16] == 0x44` | `[ret(0xa3eac)+16] != 0x44` |
| `beq` at 0xa8a14 | `[ret(0xa3eac)+16] == 0x40` | `[ret(0xa3eac)+16] != 0x40` |
| `beq` at 0xa8a24 | `[ret(0xa3eac)+16] == 0x7d` | `[ret(0xa3eac)+16] != 0x7d` |
| `beq` at 0xa8a34 | `[ret(0xa3eac)+16] == 0x2e` | `[ret(0xa3eac)+16] != 0x2e` |
| `beq` at 0xa8a44 | `[ret(0xa3eac)+16] == 0x2d` | `[ret(0xa3eac)+16] != 0x2d` |
| `beq` at 0xa8a5c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa8ab0 | `ret(0xfcb5c) != 0x0` | `ret(0xfcb5c) == 0x0` |
| `beq` at 0xa8ae4 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0xa8b08 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xa8b1c | `ret(0xa92d8) != 0x0` | `ret(0xa92d8) == 0x0` |
| `bne` at 0xa8b2c | `[sp-28] != 0x2` | `[sp-28] == 0x2` |
| `beq` at 0xa8b5c | `[ret(0xa3eac)+4] == 0x0` | `[ret(0xa3eac)+4] != 0x0` |
| `bne` at 0xa8b74 | `[[sp-56]+8] != [ret(0xa3eac)+8]` | `[[sp-56]+8] == [ret(0xa3eac)+8]` |
| `bne` at 0xa8b8c | `[[sp-56]+12] != [ret(0xa3eac)+12]` | `[[sp-56]+12] == [ret(0xa3eac)+12]` |
| `beq` at 0xa8bd4 | `ret(0xfcb5c) == 0x0` | `ret(0xfcb5c) != 0x0` |
| `beq` at 0xa8bf4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa8c28 | `r0` | `?` |

## `B2BReqRsp`

- `B2BReqRsp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8c68 to 0xa92d8.
  Established. Known-edge facts include word loads at [sp-20] +0x18 and a byte load at [sp-20] +0x16 (`ldr` at 0xa8c88, `ldrb` at 0xa916c).
  The conditional transfer is unresolved (`ldrls` at 0xa8d64), so unreached selector paths establish no effects.
  Calls to unnamed 0xfd0dc print candidate r0/r1/r2/r3 values 0x31b45c/0xfffffda0/[sp-20]/0 and 0x31b45c/0xfffffd9e/[sp-20]/0 (`bl` at 0xa9190, `bl` at 0xa9234).
  Request-to-response selection and omitted selector effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8c78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa8c80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa8c84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8c88 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8c98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa8c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8ca0 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8cac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa8cb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8cb4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8cc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8cc4 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8ccc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xa8cd0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa8cd8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xa8ce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8ce4 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8cec | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xa8cf0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xa8cf4 | store | 4 | `sp -0x2c` | `[sp-56]` |
| `ldr` at 0xa8cf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8cfc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8d08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8d0c | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8d20 | store | 4 | `sp -0x3c` | `0xe` |
| `str` at 0xa8d2c | store | 4 | `sp -0x3c` | `0xf` |
| `ldr` at 0xa8d34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8d38 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8d40 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xa8d44 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa8d4c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xa8d50 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa8d54 | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xa8d58 | load | 4 | `sp -0x30` | `-` |
| `ldrls` at 0xa8d64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9024 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9030 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa903c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9048 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9054 | load | 4 | `abs 0xa92ac` | `-` |
| `ldr` at 0xa905c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9060 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa9078 | store | 4 | `sp -0x34` | `ret(0xa3bbc)` |
| `ldr` at 0xa907c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa9088 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa908c | load | 4 | `ret(0xa3bbc) +0x10` | `-` |
| `str` at 0xa9090 | store | 4 | `sp -0x20` | `[ret(0xa3bbc)+16]` |
| `ldr` at 0xa9094 | load | 4 | `abs 0xa92a0` | `-` |
| `ldr` at 0xa9098 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa90a8 | load | 4 | `abs 0xa92a4` | `-` |
| `ldr` at 0xa90ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa90b0 | store | 4 | `sp -0x48` | `[ret(0xa3bbc)+16]` |
| `ldr` at 0xa90b4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa90b8 | load | 4 | `abs 0xa92b0` | `-` |
| `ldr` at 0xa90bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa90c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa90cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa90d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa90e4 | store | 4 | `sp -0x20` | `ret(0xa93cc)` |
| `ldr` at 0xa90e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa90f8 | load | 4 | `abs 0xa92a0` | `-` |
| `ldr` at 0xa90fc | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa910c | load | 4 | `abs 0xa92a4` | `-` |
| `ldr` at 0xa9110 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9114 | store | 4 | `sp -0x48` | `ret(0xa93cc)` |
| `ldr` at 0xa9118 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa911c | load | 4 | `abs 0xa92b4` | `-` |
| `ldr` at 0xa9120 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9124 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa912c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9140 | load | 4 | `abs 0xa92a0` | `-` |
| `ldr` at 0xa9144 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa9154 | load | 4 | `abs 0xa92a4` | `-` |
| `ldr` at 0xa9158 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa915c | load | 4 | `abs 0xa92b8` | `-` |
| `ldr` at 0xa9160 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9168 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa916c | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0xa917c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xa9180 | load | 4 | `abs 0xa92bc` | `-` |
| `ldr` at 0xa9184 | load | 4 | `abs 0xa92c0` | `-` |
| `ldr` at 0xa9188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa91a0 | load | 4 | `abs 0xa92bc` | `-` |
| `ldr` at 0xa91a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa91b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa91c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa91d0 | store | 4 | `sp -0x18` | `ret(0xfcb5c)` |
| `ldr` at 0xa91d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa91e0 | load | 4 | `abs 0xa92a0` | `-` |
| `ldr` at 0xa91e4 | load | 4 | `abs 0x1a6fac` | `-` |
| `ldr` at 0xa91f4 | load | 4 | `abs 0xa92a4` | `-` |
| `ldr` at 0xa91f8 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa91fc | load | 4 | `abs 0xa92c4` | `-` |
| `ldr` at 0xa9200 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa920c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa9210 | load | 1 | `[sp-20] +0x16` | `-` |
| `str` at 0xa9220 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xa9224 | load | 4 | `abs 0xa92bc` | `-` |
| `ldr` at 0xa9228 | load | 4 | `abs 0xa92c8` | `-` |
| `ldr` at 0xa922c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9238 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa9240 | store | 4 | `sp -0x40` | `ret(0x219a4)` |
| `ldr` at 0xa9244 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa9248 | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0xa9254 | load | 4 | `abs 0xa92cc` | `-` |
| `str` at 0xa9258 | store | 4 | `sp -0x44` | `0x16bc78` |
| `ldr` at 0xa9260 | load | 4 | `abs 0xa92d0` | `-` |
| `str` at 0xa9264 | store | 4 | `sp -0x44` | `0x16bc7c` |
| `ldr` at 0xa9268 | load | 4 | `abs 0xa92d4` | `-` |
| `ldr` at 0xa926c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9270 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa9274 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa927c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa9288 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa9288 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa9288 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9058 | `(unknown callee) 0x9adfc` | `0x16bbc8` | `r1` | `r2` | `[sp-40]` |
| `bl` at 0xa9064 | `(unknown callee) 0xa3ac4` | `[sp-36]` | `[sp-40]` | `?` | `?` |
| `bl` at 0xa9070 | `(unknown callee) 0xa3bbc` | `ret(0xa3ac4)` | `?` | `?` | `ret(0xa3ac4)` |
| `bl` at 0xa90c4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bbd4` | `[sp-36]` | `[sp-40]` |
| `bl` at 0xa90c8 | `(unknown callee) 0x9afe8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa90dc | `(unknown callee) 0xa93cc` | `[sp-20]` | `?` | `?` | `[sp-32]` |
| `bl` at 0xa9128 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bbf8` | `[sp-36]` | `[sp-40]` |
| `bl` at 0xa9130 | `(unknown callee) 0xa92d8` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa9164 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bc34` | `[sp-32]` | `0x1a288c` |
| `bl` at 0xa9190 | `(unknown callee) 0xfd0dc` | `0x31b45c` | `0xfffffda0` | `[sp-20]` | `0x0` |
| `bl` at 0xa9198 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xa91c8 | `(unknown callee) 0xfcb5c` | `?` | `[sp-20]` | `0x31b45c` | `?` |
| `bl` at 0xa9208 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16bc50` | `[sp-32]` | `ret(0xfcb5c)` |
| `bl` at 0xa9234 | `(unknown callee) 0xfd0dc` | `0x31b45c` | `0xfffffd9e` | `[sp-20]` | `0x0` |
| `bl` at 0xa923c | `(unknown callee) 0x219a4` | `?` | `?` | `?` | `?` |
| `bl` at 0xa9278 | `(unknown callee) 0x29908` | `0x16bc80` | `[sp-32]` | `[sp-68]` | `ret(0x219a4)` |
| `bl` at 0xa9280 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa8cbc | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bge` at 0xa8d04 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xa8d18 | `? != 0x2` | `? == 0x2` |
| `ldrls` at 0xa8d64 | `[sp-60]-1 <= 0xe (unsigned, 32-bit)` | `[sp-60]-1 > 0xe (unsigned, 32-bit)` |
| `bne` at 0xa9020 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa902c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xa9038 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xa9044 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa9050 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa9084 | `ret(0xa3bbc) == 0x0` | `ret(0xa3bbc) != 0x0` |
| `beq` at 0xa90a4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa90d4 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0xa90f0 | `ret(0xa93cc) > 0x0 (signed, 32-bit)` | `ret(0xa93cc) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xa9108 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa913c | `ret(0xa92d8) != 0x0` | `ret(0xa92d8) == 0x0` |
| `beq` at 0xa9150 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa9174 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `beq` at 0xa91dc | `ret(0xfcb5c) == 0x0` | `ret(0xfcb5c) != 0x0` |
| `beq` at 0xa91f0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa9218 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |
| `bne` at 0xa9250 | `[[sp-20]+22]:1 != 0x0` | `[[sp-20]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa9288 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa8d64).

## `B2BActive`

- `B2BActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa92d8 to 0xa9384.
  Established. Word stores set sp-0x18 to 0 and, on a later path, 1 (`str` at 0xa92f0, `str` at 0xa936c).
  Three word loads have unknown effective addresses despite their printed base registers and displacements (`ldr` at 0xa930c, `ldr` at 0xa9334, `ldr` at 0xa935c).
  No calls are emitted and the recorded r0 return is unknown (`ldm` at 0xa937c).
  Active-state encoding and device membership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa92e8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa92f0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa92f4 | load | 4 | `abs 0xa9380` | `-` |
| `ldr` at 0xa92f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa930c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9318 | load | 4 | `abs 0xa9380` | `-` |
| `ldr` at 0xa931c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9330 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9334 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9340 | load | 4 | `abs 0xa9380` | `-` |
| `ldr` at 0xa9344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa935c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa936c | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xa9370 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa937c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa937c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa937c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa9314 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa933c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa9364 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa937c | `r0` | `?` |

## `B2BConfigured`

- `B2BConfigured`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9384 to 0xa93cc.
  Established. A word load reads the literal at absolute 0xa93c8, while the later word load has unknown effective address (`ldr` at 0xa9398, `ldr` at 0xa93b0).
  The tool invalidates unsupported cmn effects and merges both predicated moves with their unexecuted paths (`cmn` at 0xa93b4, `moveq` at 0xa93b8, `movne` at 0xa93bc).
  No calls are emitted and the recorded r0 return is unknown (`ldm` at 0xa93c4).
  Configured-state interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9394 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa9398 | load | 4 | `abs 0xa93c8` | `-` |
| `ldr` at 0xa939c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa93b0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xa93c4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa93c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa93c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa93c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa93c4 | `r0` | `?` |

## `IRouterRegisterFunction`

- `IRouterRegisterFunction`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa1b8 to 0xfa264.
  Established. The first call to unnamed 0xf9be0 prints candidate r0/r1/r2/r3 values entry r0/entry r1/1/0 (`bl` at 0xfa208).
  The second prints unknown/unknown/2/0, and a word store saves its result at sp-0x24 (`bl` at 0xfa248, `str` at 0xfa250).
  The recorded r0 return is unknown (`ldm` at 0xfa260).
  Function registration field ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa1c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfa1cc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfa1d0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfa1d4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xfa1dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xfa1e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa1f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa1f4 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xfa1f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa1fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfa210 | store | 4 | `sp -0x24` | `ret(0xf9be0)` |
| `ldr` at 0xfa214 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa230 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa234 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xfa238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa23c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfa250 | store | 4 | `sp -0x24` | `ret(0xf9be0)` |
| `ldr` at 0xfa254 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xfa260 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfa260 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfa260 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa208 | `(unknown callee) 0xf9be0` | `r0` | `r1` | `0x1` | `0x0` |
| `bl` at 0xfa248 | `(unknown callee) 0xf9be0` | `?` | `?` | `0x2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfa1ec | `? == 0x0` | `? != 0x0` |
| `bne` at 0xfa21c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xfa22c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfa260 | `r0` | `?` |

## `IRouterRegisterQueue`

- `IRouterRegisterQueue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa264 to 0xfa310.
  Established. The first call to unnamed 0xf9be0 prints candidate r0/r1/r2/r3 values entry r0/entry r1/1/1 (`bl` at 0xfa2b4).
  The second prints unknown/unknown/2/1, and a word store saves its result at sp-0x24 (`bl` at 0xfa2f4, `str` at 0xfa2fc).
  The recorded r0 return is unknown (`ldm` at 0xfa30c).
  Queue registration field ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa274 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfa278 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfa27c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfa280 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xfa288 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xfa28c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa29c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa2a0 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0xfa2a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa2a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfa2bc | store | 4 | `sp -0x24` | `ret(0xf9be0)` |
| `ldr` at 0xfa2c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa2cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa2dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa2e0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xfa2e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa2e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfa2fc | store | 4 | `sp -0x24` | `ret(0xf9be0)` |
| `ldr` at 0xfa300 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xfa30c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfa30c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfa30c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa2b4 | `(unknown callee) 0xf9be0` | `r0` | `r1` | `0x1` | `0x1` |
| `bl` at 0xfa2f4 | `(unknown callee) 0xf9be0` | `?` | `?` | `0x2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfa298 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xfa2c8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xfa2d8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfa30c | `r0` | `?` |

## `IRouterEventRegisterFunction`

- `IRouterEventRegisterFunction`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa64c to 0xfa698.
  Established. A word store writes entry r3 at sp-0x24 (`str` at 0xfa670).
  The sole call to unnamed 0xfa310 prints candidate r0/r1/r2/r3 values entry r0/entry r1/entry r2/0 (`bl` at 0xfa684).
  The recorded r0 return is that call result (`ldm` at 0xfa694).
  Event callback registration semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa65c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfa660 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfa664 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfa668 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xfa66c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa670 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xfa674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa67c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xfa694 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfa694 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfa694 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa684 | `(unknown callee) 0xfa310` | `r0` | `r1` | `r2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfa694 | `r0` | `ret(0xfa310)` |

## `IRouterEventRegisterQueue`

- `IRouterEventRegisterQueue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa698 to 0xfa6e4.
  Established. A word store writes entry r3 at sp-0x24 (`str` at 0xfa6bc).
  The sole call to unnamed 0xfa310 prints candidate r0/r1/r2/r3 values entry r0/entry r1/entry r2/1 (`bl` at 0xfa6d0).
  The recorded r0 return is that call result (`ldm` at 0xfa6e0).
  Event queue registration semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa6a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfa6ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfa6b0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xfa6b4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xfa6b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa6bc | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xfa6c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa6c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa6c8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xfa6e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfa6e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfa6e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa6d0 | `(unknown callee) 0xfa310` | `r0` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfa6e0 | `r0` | `ret(0xfa310)` |

## `IRouterUnregister`

- `IRouterUnregister`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfa6e4 to 0xfaa44.
  Established. Word stores update absolute 0x31be18 from [[sp-36]+40] and absolute 0x31baa8 from [[sp-48]+24] (`str` at 0xfa89c, `str` at 0xfa974).
  Other word stores write those values at [sp-40] +0x28 and [sp-52] +0x18 (`str` at 0xfa8b0, `str` at 0xfa988).
  The recorded r0 return is 0 (`ldm` at 0xfaa18).
  Unregistration ownership and resource lifetime remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfa6f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfa6f8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfa6fc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xfa700 | load | 4 | `abs 0xfaa1c` | `-` |
| `ldr` at 0xfa704 | load | 4 | `abs 0x31be18` | `-` |
| `str` at 0xfa708 | store | 4 | `sp -0x24` | `[0x31be18]` |
| `str` at 0xfa710 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xfa718 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xfa71c | load | 4 | `abs 0xfaa20` | `-` |
| `ldr` at 0xfa720 | load | 4 | `abs 0x31baa8` | `-` |
| `str` at 0xfa724 | store | 4 | `sp -0x30` | `[0x31baa8]` |
| `str` at 0xfa72c | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xfa734 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xfa738 | load | 4 | `abs 0xfaa24` | `-` |
| `str` at 0xfa748 | store | 4 | `sp -0x3c` | `ret(0x22ca0)` |
| `ldr` at 0xfa74c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xfa758 | load | 4 | `abs 0xfaa28` | `-` |
| `str` at 0xfa75c | store | 4 | `sp -0x48` | `0x188068` |
| `ldr` at 0xfa760 | load | 4 | `abs 0xfaa24` | `-` |
| `str` at 0xfa764 | store | 4 | `sp -0x44` | `0x1b1028` |
| `ldr` at 0xfa768 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xfa76c | store | 4 | `sp -0x40` | `ret(0x22ca0)` |
| `ldr` at 0xfa774 | load | 4 | `abs 0xfaa2c` | `-` |
| `ldr` at 0xfa784 | load | 4 | `abs 0xfaa30` | `-` |
| `ldr` at 0xfa788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa78c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa798 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xfa7ac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xfa7b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa7bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa7c8 | load | 4 | `abs 0xfaa34` | `-` |
| `ldr` at 0xfa7cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa7e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa7e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfa7ec | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xfa7f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfa80c | load | 4 | `abs 0xfaa34` | `-` |
| `ldr` at 0xfa824 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfa82c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xfa834 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa840 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa848 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xfa84c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xfa858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa864 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa868 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xfa86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa87c | load | 4 | `abs 0xfaa1c` | `-` |
| `ldr` at 0xfa880 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa884 | load | 4 | `abs 0x31be18` | `-` |
| `ldr` at 0xfa890 | load | 4 | `abs 0xfaa1c` | `-` |
| `ldr` at 0xfa894 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa898 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xfa89c | store | 4 | `abs 0x31be18` | `[[sp-36]+40]` |
| `ldr` at 0xfa8a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfa8a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa8ac | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xfa8b0 | store | 4 | `[sp-40] +0x28` | `[[sp-36]+40]` |
| `ldr` at 0xfa8b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfa8b8 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xfa8bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa8c0 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xfa8c4 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xfa8c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfa8d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xfa8d8 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xfa8dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xfa8e0 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xfa8e4 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xfa8ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfa8f8 | load | 4 | `abs 0xfaa38` | `-` |
| `ldr` at 0xfa8fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa900 | load | 4 | `abs 0x31ba84` | `-` |
| `ldr` at 0xfa904 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xfa910 | load | 4 | `abs 0xfaa38` | `-` |
| `ldr` at 0xfa920 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xfa930 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfa93c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfa940 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfa944 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xfa948 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xfa954 | load | 4 | `abs 0xfaa20` | `-` |
| `ldr` at 0xfa958 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfa95c | load | 4 | `abs 0x31baa8` | `-` |
| `ldr` at 0xfa968 | load | 4 | `abs 0xfaa20` | `-` |
| `ldr` at 0xfa96c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfa970 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xfa974 | store | 4 | `abs 0x31baa8` | `[[sp-48]+24]` |
| `ldr` at 0xfa97c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xfa980 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfa984 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xfa988 | store | 4 | `[sp-52] +0x18` | `[[sp-48]+24]` |
| `ldr` at 0xfa98c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xfa990 | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0xfa994 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfa998 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xfa99c | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xfa9a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xfa9ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xfa9b0 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xfa9b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xfa9b8 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xfa9bc | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xfa9c4 | load | 4 | `abs 0xfaa24` | `-` |
| `str` at 0xfa9d0 | store | 4 | `sp -0x3c` | `ret(0x2302c)` |
| `ldr` at 0xfa9d4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xfa9e0 | load | 4 | `abs 0xfaa3c` | `-` |
| `str` at 0xfa9e4 | store | 4 | `sp -0x48` | `0x1880c0` |
| `ldr` at 0xfa9e8 | load | 4 | `abs 0xfaa24` | `-` |
| `str` at 0xfa9ec | store | 4 | `sp -0x44` | `0x1b1028` |
| `ldr` at 0xfa9f0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xfa9f4 | store | 4 | `sp -0x40` | `ret(0x2302c)` |
| `ldr` at 0xfa9fc | load | 4 | `abs 0xfaa2c` | `-` |
| `ldr` at 0xfaa00 | load | 4 | `abs 0xfaa40` | `-` |
| `ldm` at 0xfaa18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfaa18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfaa18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfa740 | `(unknown callee) 0x22ca0` | `0x1b1028` | `?` | `r2` | `0x0` |
| `bl` at 0xfa780 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x170` | `0x0` |
| `bl` at 0xfa794 | `(unknown callee) 0x29908` | `0x1880f0` | `[sp-20]` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xfa820 | `(unknown callee) 0x20550` | `?` | `0x0` | `0x24` | `?` |
| `bl` at 0xfa8cc | `(unknown callee) 0x20514` | `[sp-36]` | `[[sp-36]]` | `[[sp-36]+40]` | `[[sp-36]+40]` |
| `bl` at 0xfa91c | `(unknown callee) 0x20550` | `0x31ba84` | `0x0` | `0x24` | `[[sp-20]]` |
| `bl` at 0xfa9a4 | `(unknown callee) 0x20514` | `[sp-48]` | `[[sp-48]]` | `[[sp-48]+24]` | `[[sp-48]+24]` |
| `bl` at 0xfa9c8 | `(unknown callee) 0x2302c` | `0x1b1028` | `?` | `?` | `?` |
| `bl` at 0xfaa08 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x1af` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfa754 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `beq` at 0xfa7a4 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0xfa7b8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xfa7c4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xfa7f4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xfa83c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0xfa854 | `[[sp-36]] != [[sp-20]]` | `[[sp-36]] == [[sp-20]]` |
| `beq` at 0xfa860 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xfa874 | `[[sp-36]+4] == [sp-24]` | `[[sp-36]+4] != [sp-24]` |
| `bne` at 0xfa88c | `[sp-36] != [0x31be18]` | `[sp-36] == [0x31be18]` |
| `bne` at 0xfa8f4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xfa90c | `[0x31ba84] != [[sp-20]]` | `[0x31ba84] == [[sp-20]]` |
| `beq` at 0xfa92c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xfa938 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `bne` at 0xfa950 | `[[sp-48]] != [[sp-20]]` | `[[sp-48]] == [[sp-20]]` |
| `bne` at 0xfa964 | `[sp-48] != [0x31baa8]` | `[sp-48] == [0x31baa8]` |
| `beq` at 0xfa9dc | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfaa18 | `r0` | `0x0` |

## `IRouterPacketCount`

- `IRouterPacketCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfaa44 to 0xfab70.
  Established. Word stores add [sp-20] to the loaded values at absolute 0x1b1014 and 0x1b1018 (`str` at 0xfaab8, `str` at 0xfaad0).
  A signed comparison branches to 0xfaafc when [0x1b1018] is less than or equal to [0x1b101c], while its fall-through reaches a store of [0x1b1018] to absolute 0x1b101c (`ble` at 0xfaae8, `str` at 0xfaaf8).
  Calls to unnamed 0x22ca0 and 0x2302c both print candidate r0 value 0x1b107c (`bl` at 0xfaa60, `bl` at 0xfab00).
  Counter-field meanings and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfaa54 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xfaa58 | load | 4 | `abs 0xfab4c` | `-` |
| `str` at 0xfaa68 | store | 4 | `sp -0x18` | `ret(0x22ca0)` |
| `ldr` at 0xfaa6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfaa78 | load | 4 | `abs 0xfab50` | `-` |
| `str` at 0xfaa7c | store | 4 | `sp -0x24` | `0x188068` |
| `ldr` at 0xfaa80 | load | 4 | `abs 0xfab4c` | `-` |
| `str` at 0xfaa84 | store | 4 | `sp -0x20` | `0x1b107c` |
| `ldr` at 0xfaa88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfaa8c | store | 4 | `sp -0x1c` | `ret(0x22ca0)` |
| `ldr` at 0xfaa94 | load | 4 | `abs 0xfab54` | `-` |
| `ldr` at 0xfaa98 | load | 4 | `abs 0xfab58` | `-` |
| `ldr` at 0xfaaa4 | load | 4 | `abs 0xfab5c` | `-` |
| `ldr` at 0xfaaa8 | load | 4 | `abs 0xfab5c` | `-` |
| `ldr` at 0xfaaac | load | 4 | `abs 0x1b1014` | `-` |
| `ldr` at 0xfaab0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfaab8 | store | 4 | `abs 0x1b1014` | `([0x1b1014] + [sp-20])` |
| `ldr` at 0xfaabc | load | 4 | `abs 0xfab60` | `-` |
| `ldr` at 0xfaac0 | load | 4 | `abs 0xfab60` | `-` |
| `ldr` at 0xfaac4 | load | 4 | `abs 0x1b1018` | `-` |
| `ldr` at 0xfaac8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfaad0 | store | 4 | `abs 0x1b1018` | `([0x1b1018] + [sp-20])` |
| `ldr` at 0xfaad4 | load | 4 | `abs 0xfab60` | `-` |
| `ldr` at 0xfaad8 | load | 4 | `abs 0xfab64` | `-` |
| `ldr` at 0xfaadc | load | 4 | `abs 0x1b1018` | `-` |
| `ldr` at 0xfaae0 | load | 4 | `abs 0x1b101c` | `-` |
| `ldr` at 0xfaaec | load | 4 | `abs 0xfab64` | `-` |
| `ldr` at 0xfaaf0 | load | 4 | `abs 0xfab60` | `-` |
| `ldr` at 0xfaaf4 | load | 4 | `abs 0x1b1018` | `-` |
| `str` at 0xfaaf8 | store | 4 | `abs 0x1b101c` | `[0x1b1018]` |
| `ldr` at 0xfaafc | load | 4 | `abs 0xfab4c` | `-` |
| `str` at 0xfab08 | store | 4 | `sp -0x18` | `ret(0x2302c)` |
| `ldr` at 0xfab0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfab18 | load | 4 | `abs 0xfab68` | `-` |
| `str` at 0xfab1c | store | 4 | `sp -0x24` | `0x1880c0` |
| `ldr` at 0xfab20 | load | 4 | `abs 0xfab4c` | `-` |
| `str` at 0xfab24 | store | 4 | `sp -0x20` | `0x1b107c` |
| `ldr` at 0xfab28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfab2c | store | 4 | `sp -0x1c` | `ret(0x2302c)` |
| `ldr` at 0xfab34 | load | 4 | `abs 0xfab54` | `-` |
| `ldr` at 0xfab38 | load | 4 | `abs 0xfab6c` | `-` |
| `ldm` at 0xfab48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfab48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfab48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfaa60 | `(unknown callee) 0x22ca0` | `0x1b107c` | `?` | `r2` | `r3` |
| `bl` at 0xfaaa0 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x1be` | `0x0` |
| `bl` at 0xfab00 | `(unknown callee) 0x2302c` | `0x1b107c` | `[0x1b1018]` | `?` | `?` |
| `bl` at 0xfab40 | `(unknown callee) 0x22070` | `0x9` | `0x188094` | `0x1c6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xfaa74 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `ble` at 0xfaae8 | `[0x1b1018] <= [0x1b101c] (signed, 32-bit)` | `[0x1b1018] > [0x1b101c] (signed, 32-bit)` |
| `beq` at 0xfab14 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfab48 | `r0` | `?` |

## `IRouterQPacket`

- `IRouterQPacket`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfcb5c to 0xfce00.
  Established. Byte and halfword loads read [sp-24] +0x16 and +0x22 (`ldrb` at 0xfcba8, `ldrh` at 0xfcc0c).
  Calls to unnamed 0x23f34 and 0x239f4 print candidate r0/r1 values [sp-20]/sp-24, with the latter also printing r2=[0x1a04a4] and r3=0xfbd10 (`bl` at 0xfcc7c, `bl` at 0xfccbc).
  Word stores write 0xfffffd9d and 0xfffffd9e to sp-0x28 on separate paths (`str` at 0xfcd18, `str` at 0xfcd98).
  Queue dispatch fields and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfcb6c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfcb70 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfcb78 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xfcb8c | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xfcba0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xfcba4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xfcba8 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xfcbb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xfcbb8 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xfcbc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfcbc8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0xfcbd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfcbd8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0xfcbf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfcbf4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `ldr` at 0xfcc08 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xfcc0c | load | 2 | `[sp-24] +0x22` | `-` |
| `ldr` at 0xfcc18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfcc28 | store | 4 | `sp -0x54` | `sp-24` |
| `ldr` at 0xfcc2c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xfcc30 | load | 2 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xfcc48 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xfcc4c | load | 2 | `[sp-24] +0x6` | `-` |
| `str` at 0xfcc5c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xfcc64 | load | 4 | `abs 0xfcdd8` | `-` |
| `ldr` at 0xfcc68 | load | 4 | `abs 0x1a04a8` | `-` |
| `str` at 0xfcc6c | store | 4 | `sp -0x58` | `[0x1a04a8]` |
| `ldr` at 0xfcc70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcc74 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xfcc78 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xfcc84 | store | 4 | `sp -0x28` | `ret(0x23f34)` |
| `ldr` at 0xfcc90 | load | 4 | `abs 0xfcddc` | `-` |
| `ldr` at 0xfcc94 | load | 4 | `abs 0xfcde0` | `-` |
| `str` at 0xfcc98 | store | 4 | `sp -0x74` | `0xfbe84` |
| `str` at 0xfcca0 | store | 4 | `sp -0x70` | `sp-32` |
| `str` at 0xfcca8 | store | 4 | `sp -0x6c` | `0x1` |
| `ldr` at 0xfccac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfccb4 | load | 4 | `abs 0x1a04a4` | `-` |
| `ldr` at 0xfccb8 | load | 4 | `abs 0xfcde4` | `-` |
| `str` at 0xfccc4 | store | 4 | `sp -0x28` | `ret(0x239f4)` |
| `ldr` at 0xfccc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xfccd4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xfccd8 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xfccdc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xfcce8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xfccf4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xfcd04 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xfcd14 | load | 4 | `abs 0xfcde8` | `-` |
| `str` at 0xfcd18 | store | 4 | `sp -0x28` | `0xfffffd9d` |
| `str` at 0xfcd20 | store | 4 | `sp -0x5c` | `sp-80` |
| `ldr` at 0xfcd24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfcd2c | store | 4 | `sp -0x60` | `[sp-20]+120` |
| `ldr` at 0xfcd30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xfcd34 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xfcd40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xfcd44 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xfcd50 | load | 4 | `abs 0xfcdec` | `-` |
| `str` at 0xfcd54 | store | 4 | `sp -0x64` | `0x18819c` |
| `ldr` at 0xfcd5c | load | 4 | `abs 0xfcdf0` | `-` |
| `str` at 0xfcd60 | store | 4 | `sp -0x64` | `0x188244` |
| `ldr` at 0xfcd68 | load | 4 | `abs 0xfcdf4` | `-` |
| `str` at 0xfcd6c | store | 4 | `sp -0x64` | `0x188198` |
| `ldr` at 0xfcd70 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xfcd74 | load | 4 | `abs 0xfcdf8` | `-` |
| `ldr` at 0xfcd78 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xfcd7c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xfcd94 | load | 4 | `abs 0xfcdfc` | `-` |
| `str` at 0xfcd98 | store | 4 | `sp -0x28` | `0xfffffd9e` |
| `ldr` at 0xfcda8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfcdc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xfcdc8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xfcdd4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfcdd4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfcdd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfcc20 | `(unknown callee) 0xfcea0` | `[sp-20]` | `0x0` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0xfcc7c | `(unknown callee) 0x23f34` | `[sp-20]` | `sp-24` | `?` | `?` |
| `bl` at 0xfccbc | `(unknown callee) 0x239f4` | `[sp-20]` | `sp-24` | `[0x1a04a4]` | `0xfbd10` |
| `bl` at 0xfcd80 | `(unknown callee) 0x205bc` | `sp-80` | `0x188248` | `[sp-20]+120` | `?` |
| `bl` at 0xfcd8c | `(unknown callee) 0x293f8` | `sp-80` | `?` | `?` | `sp-80` |
| `bl` at 0xfcdc4 | `(unknown callee) 0xfb794` | `?` | `sp-16` | `sp-16` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xfcbb0 | `[[sp-24]+22]:1 != 0x2` | `[[sp-24]+22]:1 == 0x2` |
| `bne` at 0xfcbc0 | `[[sp-24]+22]:1 != 0x2` | `[[sp-24]+22]:1 == 0x2` |
| `bge` at 0xfcbd0 | `[[sp-24]+28] >= 0x0 (signed, 32-bit)` | `[[sp-24]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0xfcbe8 | `? == 0x5` | `? != 0x5` |
| `beq` at 0xfcc00 | `? == 0x5` | `? != 0x5` |
| `bne` at 0xfcc14 | `[[sp-24]+34]:2 != 0x1` | `[[sp-24]+34]:2 == 0x1` |
| `beq` at 0xfcc44 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xfccd0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfcce4 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0xfccf0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xfccfc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xfcd0c | `[sp-104] == 0x6e` | `[sp-104] != 0x6e` |
| `beq` at 0xfcd3c | `[[sp-24]+22]:1 == 0x0` | `[[sp-24]+22]:1 != 0x0` |
| `bne` at 0xfcd4c | `[[sp-24]+22]:1 != 0x1` | `[[sp-24]+22]:1 == 0x1` |
| `beq` at 0xfcdb0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfcdd4 | `r0` | `[sp-40]` |

## `IRouterQPurge`

- `IRouterQPurge`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfcea0 to 0xfcf98.
  Established. A halfword load reads [sp-20] +0x86 and word loads read [sp-20] +0x94, +0x8c and +0x90 (`ldrh` at 0xfcecc, `ldr` at 0xfcee8, `ldr` at 0xfcf08, `ldr` at 0xfcf1c).
  The unknown-target call prints candidate r0/r1/r2/r3 values unknown/unknown/[[sp-20]+140]/[sp-24] (`mov` at 0xfcf50).
  Zero `ret(unknown)` takes `0xfcf6c`, while nonzero falls through (`beq` at 0xfcf5c).
  The fall-through path includes an unnamed `0xfbbe0` call carrying candidate r0/r1 values `[sp-20]`/`[sp-32]` (`bl` at 0xfcf68).
  Purge callback effects and queue ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfceb0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfceb4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xfcebc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xfcec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcec8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xfcecc | load | 2 | `[sp-20] +0x86` | `-` |
| `ldr` at 0xfced8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcee8 | load | 4 | `[sp-20] +0x94` | `-` |
| `str` at 0xfceec | store | 4 | `sp -0x20` | `[[sp-20]+148]` |
| `ldr` at 0xfcef0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfcefc | load | 4 | `abs 0xfcf94` | `-` |
| `str` at 0xfcf00 | store | 4 | `sp -0x18` | `0xfce00` |
| `ldr` at 0xfcf04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcf08 | load | 4 | `[sp-20] +0x8c` | `-` |
| `ldr` at 0xfcf0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfcf18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcf1c | load | 4 | `[sp-20] +0x90` | `-` |
| `str` at 0xfcf24 | store | 4 | `sp -0x20` | `[[sp-20]+144]-4` |
| `ldr` at 0xfcf2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xfcf34 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xfcf38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfcf3c | load | 4 | `(unknown)` | `-` |
| `str` at 0xfcf40 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xfcf44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xfcf48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xfcf60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcf64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfcf6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xfcf70 | load | 4 | `[sp-20] +0x98` | `-` |
| `ldr` at 0xfcf74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xfcf84 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xfcf90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfcf90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfcf90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xfcec4 | `(unknown callee) 0x234cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xfcedc | `(unknown callee) 0x234f4` | `[sp-20]` | `?` | `?` | `[[sp-20]+134]:2` |
| `mov` at 0xfcf50 | `(unknown callee) (unknown target)` | `?` | `?` | `[[sp-20]+140]` | `[sp-24]` |
| `bl` at 0xfcf68 | `(unknown callee) 0xfbbe0` | `[sp-20]` | `[sp-32]` | `?` | `ret(unknown)` |
| `bl` at 0xfcf88 | `(unknown callee) 0x234f4` | `[sp-20]` | `?` | `[[sp-20]+152]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xfced4 | `[[sp-20]+134]:2 != 0x0` | `[[sp-20]+134]:2 == 0x0` |
| `bne` at 0xfcef8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xfcf14 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xfcf5c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0xfcf7c | `[[sp-20]+152] == [sp-32]` | `[[sp-20]+152] != [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfcf90 | `r0` | `ret(0x234f4)` |

## `IRouterSetQueueSuspendTime`

- `IRouterSetQueueSuspendTime`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xfbb58 to 0xfbb98.
  Established. A word store writes entry r0 to absolute 0x1a04a4 (`str` at 0xfbb78).
  Another writes [sp-24] to absolute 0x1a04a8 after entry r1 was saved at sp-0x18 (`str` at 0xfbb6c, `str` at 0xfbb84).
  No calls are emitted and the recorded r0 return remains entry r0 (`ldm` at 0xfbb8c).
  Suspend-time units and physical transport remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xfbb68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xfbb6c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xfbb70 | load | 4 | `abs 0xfbb90` | `-` |
| `ldr` at 0xfbb74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xfbb78 | store | 4 | `abs 0x1a04a4` | `r0` |
| `ldr` at 0xfbb7c | load | 4 | `abs 0xfbb94` | `-` |
| `ldr` at 0xfbb80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xfbb84 | store | 4 | `abs 0x1a04a8` | `[sp-24]` |
| `ldm` at 0xfbb8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xfbb8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xfbb8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xfbb8c | `r0` | `r0` |
