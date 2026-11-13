# ARM L1 fan entry paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. An unresolved computed transfer can leave additional effects and return paths unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/fan-facts.txt) retain all tool sections.

## `fanTachInit`

- `fanTachInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x419d8 to 0x41b04.
  Established. The word at 0x119e54 decides `beq`, whose taken zero path skips the first call to
  0x1eed0 (`ldr` at 0x419f4, `beq` at 0x419fc, `bl` at 0x41a18). The call to 0x18920 supplies the
  value stored as a word at 0x119e64 (`bl` at 0x41a24, `str` at 0x41a2c). Later word stores write
  `[sp-24]` at 0x119e54 and `[sp-28]` at 0x119e58 (`str` at 0x41a74, `str` at 0x41a80). The loop's
  `blt` and inner `bne` retain unknown operands and Z == 0 respectively (`blt` at 0x41a98, `bne` at
  0x41ac8). The globals' physical roles and iteration count are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x419e8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x419ec | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x419f0 | load | 4 | `abs 0x41aec` | `-` |
| `ldr` at 0x419f4 | load | 4 | `abs 0x119e54` | `-` |
| `ldr` at 0x41a00 | load | 4 | `abs 0x41af0` | `-` |
| `str` at 0x41a04 | store | 4 | `sp -0x24` | `0xe2404` |
| `ldr` at 0x41a0c | load | 4 | `abs 0x41af4` | `-` |
| `ldr` at 0x41a1c | load | 4 | `abs 0x41af8` | `-` |
| `ldr` at 0x41a20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x41a2c | store | 4 | `abs 0x119e64` | `ret(0x18920)` |
| `ldr` at 0x41a30 | load | 4 | `abs 0x119e64` | `-` |
| `ldr` at 0x41a3c | load | 4 | `abs 0x41afc` | `-` |
| `str` at 0x41a40 | store | 4 | `sp -0x24` | `0xe2434` |
| `ldr` at 0x41a48 | load | 4 | `abs 0x41af4` | `-` |
| `ldr` at 0x41a58 | load | 4 | `abs 0x41af8` | `-` |
| `ldr` at 0x41a5c | load | 4 | `abs 0x119e64` | `-` |
| `ldr` at 0x41a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41a6c | load | 4 | `abs 0x41aec` | `-` |
| `ldr` at 0x41a70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x41a74 | store | 4 | `abs 0x119e54` | `[sp-24]` |
| `ldr` at 0x41a78 | load | 4 | `abs 0x41b00` | `-` |
| `ldr` at 0x41a7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x41a80 | store | 4 | `abs 0x119e58` | `[sp-28]` |
| `str` at 0x41a88 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x41a8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41a90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41aa0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41ab4 | load | 4 | `abs 0x41aec` | `-` |
| `ldr` at 0x41ab8 | load | 4 | `abs 0x119e54` | `-` |
| `ldrb` at 0x41ac0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x41acc | load | 1 | `sp -0x20` | `-` |
| `ldr` at 0x41ad8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x41ae0 | store | 4 | `sp -0x20` | `?` |
| `ldmdb` at 0x41ae8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x41ae8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41ae8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41ae8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41a18 | `(unknown callee) 0x1eed0` | `0x9` | `0xe2420` | `0x3e` | `0x0` |
| `bl` at 0x41a24 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x41a54 | `(unknown callee) 0x1eed0` | `0x9` | `0xe2420` | `0x44` | `0x0` |
| `bl` at 0x41a68 | `(unknown callee) 0x18c20` | `[0x119e64]` | `0x1` | `[sp-28]` | `0x119e64` |
| `bl` at 0x41ad4 | `(unknown callee) 0x41b74` | `[sp-32]:1` | `?` | `?` | `[sp-32]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x419fc | `[0x119e54] == 0x0` | `[0x119e54] != 0x0` |
| `bne` at 0x41a38 | `[0x119e64] != 0x0` | `[0x119e64] == 0x0` |
| `blt` at 0x41a98 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x41ac8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41ae8 | `r0` | `?` |

## `fanCtrlInit`

- `fanCtrlInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41b04 to 0x41b74.
  Established. The word loaded at 0x119e5c decides `beq`, whose taken zero path skips the call to
  0x1eed0 (`ldr` at 0x41b20, `beq` at 0x41b28, `bl` at 0x41b44). The subsequent word stores target
  0x119e5c with an unknown value and 0x119e60 with `[sp-24]` (`str` at 0x41b50, `str` at 0x41b5c).
  This body does not establish the stored values' device meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41b14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41b18 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x41b1c | load | 4 | `abs 0x41b64` | `-` |
| `ldr` at 0x41b20 | load | 4 | `abs 0x119e5c` | `-` |
| `ldr` at 0x41b2c | load | 4 | `abs 0x41b68` | `-` |
| `str` at 0x41b30 | store | 4 | `sp -0x1c` | `0xe2460` |
| `ldr` at 0x41b38 | load | 4 | `abs 0x41b6c` | `-` |
| `ldr` at 0x41b48 | load | 4 | `abs 0x41b64` | `-` |
| `ldr` at 0x41b4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x41b50 | store | 4 | `abs 0x119e5c` | `?` |
| `ldr` at 0x41b54 | load | 4 | `abs 0x41b70` | `-` |
| `ldr` at 0x41b58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x41b5c | store | 4 | `abs 0x119e60` | `[sp-24]` |
| `ldmdb` at 0x41b60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41b60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41b60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41b44 | `(unknown callee) 0x1eed0` | `0x9` | `0xe2420` | `0x61` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x41b28 | `[0x119e5c] == 0x0` | `[0x119e5c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41b60 | `r0` | `?` |

## `fanTachDisable`

- `fanTachDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41b74 to 0x41c0c.
  Established. `bne` takes the path into the body when `(r0 & 0xff) != 0xff`. Equality falls through
  to a jump to the return (`bne` at 0x41b94, `b` at 0x41b98). The subsequent `blt` takes the path
  past 0x1eed0 when `(r0 & 0xff) < [0x119e58]` signed (`blt` at 0x41bac, `bl` at 0x41bdc). The final
  byte store writes 0x0 to an unknown effective target (`strb` at 0x41bf4). The disable role implied
  by the name remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x41b88 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x41b8c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x41b9c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x41ba0 | load | 4 | `abs 0x41bfc` | `-` |
| `ldr` at 0x41ba4 | load | 4 | `abs 0x119e58` | `-` |
| `ldr` at 0x41bb0 | load | 4 | `abs 0x41c00` | `-` |
| `str` at 0x41bb4 | store | 4 | `sp -0x20` | `0xe247c` |
| `ldrb` at 0x41bb8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x41bbc | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x41bc0 | load | 4 | `abs 0x41bfc` | `-` |
| `ldr` at 0x41bc4 | load | 4 | `abs 0x119e58` | `-` |
| `str` at 0x41bc8 | store | 4 | `sp -0x18` | `[0x119e58]` |
| `ldr` at 0x41bd0 | load | 4 | `abs 0x41c04` | `-` |
| `ldrb` at 0x41be0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x41be4 | load | 4 | `abs 0x41c08` | `-` |
| `ldr` at 0x41be8 | load | 4 | `abs 0x119e64` | `-` |
| `strb` at 0x41bf4 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x41bf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41bf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41bf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41bdc | `(unknown callee) 0x1eed0` | `0x9` | `0xe2420` | `0x74` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x41b94 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x41bac | `(r0 & 0xff) < [0x119e58] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119e58] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41bf8 | `r0` | `?` |

## `fanTachEnable`

- `fanTachEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41c0c to 0x41ca4.
  Established. Entry r0's masked byte is saved at sp -0x11 (`strb` at 0x41c20). The first `bne`
  takes the non-0xff path into the body (`bne` at 0x41c2c). The later `blt` takes the path past the
  call to 0x1eed0 for `(r0 & 0xff) < [0x119e58]` signed (`blt` at 0x41c44, `bl` at 0x41c74). The
  byte store at the end writes 0x1 to an unknown effective target (`strb` at 0x41c8c). The target's
  hardware identity is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x41c20 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x41c24 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x41c34 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x41c38 | load | 4 | `abs 0x41c94` | `-` |
| `ldr` at 0x41c3c | load | 4 | `abs 0x119e58` | `-` |
| `ldr` at 0x41c48 | load | 4 | `abs 0x41c98` | `-` |
| `str` at 0x41c4c | store | 4 | `sp -0x20` | `0xe247c` |
| `ldrb` at 0x41c50 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x41c54 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x41c58 | load | 4 | `abs 0x41c94` | `-` |
| `ldr` at 0x41c5c | load | 4 | `abs 0x119e58` | `-` |
| `str` at 0x41c60 | store | 4 | `sp -0x18` | `[0x119e58]` |
| `ldr` at 0x41c68 | load | 4 | `abs 0x41c9c` | `-` |
| `ldrb` at 0x41c78 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x41c7c | load | 4 | `abs 0x41ca0` | `-` |
| `ldr` at 0x41c80 | load | 4 | `abs 0x119e64` | `-` |
| `strb` at 0x41c8c | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x41c90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41c90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41c90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41c74 | `(unknown callee) 0x1eed0` | `0x9` | `0xe2420` | `0x83` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x41c2c | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x41c44 | `(r0 & 0xff) < [0x119e58] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119e58] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41c90 | `r0` | `?` |

## `fanTachPresent`

- `fanTachPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41ca4 to 0x41d40.
  Established. When `(r0 & 0xff) == 0xff`, the first `bne` falls through to a jump past the
  byte-load path (`bne` at 0x41cc4, `b` at 0x41cc8). Otherwise `blt` takes the path past the call to
  0x1eed0 for `(r0 & 0xff) < [0x119e58]` signed (`blt` at 0x41cdc, `bl` at 0x41d0c). The later byte
  load has an unknown effective target, and its word save at sp -0x18 has an unknown value (`ldrb`
  at 0x41d20, `str` at 0x41d24). The return retains `[sp-24]` (`ldmdb` at 0x41d2c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x41cb8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x41cbc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x41ccc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x41cd0 | load | 4 | `abs 0x41d30` | `-` |
| `ldr` at 0x41cd4 | load | 4 | `abs 0x119e58` | `-` |
| `ldr` at 0x41ce0 | load | 4 | `abs 0x41d34` | `-` |
| `str` at 0x41ce4 | store | 4 | `sp -0x24` | `0xe247c` |
| `ldrb` at 0x41ce8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x41cec | store | 4 | `sp -0x20` | `(r0 & 0xff)` |
| `ldr` at 0x41cf0 | load | 4 | `abs 0x41d30` | `-` |
| `ldr` at 0x41cf4 | load | 4 | `abs 0x119e58` | `-` |
| `str` at 0x41cf8 | store | 4 | `sp -0x1c` | `[0x119e58]` |
| `ldr` at 0x41d00 | load | 4 | `abs 0x41d38` | `-` |
| `ldrb` at 0x41d10 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x41d14 | load | 4 | `abs 0x41d3c` | `-` |
| `ldr` at 0x41d18 | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x41d20 | load | 1 | `(unknown)` | `-` |
| `str` at 0x41d24 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x41d28 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x41d2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41d2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41d2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41d0c | `(unknown callee) 0x1eed0` | `0x9` | `0xe2420` | `0x92` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x41cc4 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x41cdc | `(r0 & 0xff) < [0x119e58] (signed, 32-bit)` | `(r0 & 0xff) >= [0x119e58] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41d2c | `r0` | `[sp-24]` |

## `fanReset`

- `fanReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41d40 to 0x4220c.
  Established. The first loop calls 0x43158 with r0 candidate `[sp-40]:1`, then saves a byte loaded
  from the call result at sp -0x54 (`bl` at 0x41d78, `ldrb` at 0x41d88, `str` at 0x41d8c). The `beq`
  sites select paths for that byte equal to 0x2, 0x1 and 0x63 (`beq` at 0x41d98, `beq` at 0x41db0,
  `beq` at 0x41dc0). One path calls 0x1c79c with first candidates 0x4 and 0x5 at separate sites
  (`bl` at 0x41e00, `bl` at 0x41e14). The later call to 0xb9e38 has candidates `[sp-64]:1`,
  `[sp-68]:1`, `[sp-72]` and `[sp-76]` (`bl` at 0x421d0). The two loop-control `blt` predicates have
  unknown operands, so no iteration count or physical reset sequence is established (`blt` at
  0x41d68, `blt` at 0x42054).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41d54 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x41d58 | load | 4 | `abs 0x421f0` | `-` |
| `ldr` at 0x41d5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41d60 | load | 4 | `abs 0x119e60` | `-` |
| `ldrb` at 0x41d70 | load | 1 | `sp -0x28` | `-` |
| `str` at 0x41d80 | store | 4 | `sp -0x18` | `ret(0x43158)` |
| `ldr` at 0x41d84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41d88 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x41d8c | store | 4 | `sp -0x54` | `[ret(0x43158)]:1` |
| `ldr` at 0x41d90 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x41d9c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x41da8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x41db8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x41dc8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41dcc | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x41ddc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41de0 | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x41e1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41e20 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x41e28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41e30 | load | 1 | `[sp-24] +0x5` | `-` |
| `strb` at 0x41e38 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0x41e3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41e40 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x41e44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41e48 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0x41e4c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x41e60 | store | 4 | `sp -0x24` | `ret(0xb79a0)` |
| `ldr` at 0x41e64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41e70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41e74 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x41e84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41e90 | load | 4 | `abs 0x421f4` | `-` |
| `str` at 0x41e94 | store | 4 | `sp -0x60` | `0xe24a4` |
| `str` at 0x41e98 | store | 4 | `sp -0x5c` | `ret(0xb8268)` |
| `str` at 0x41e9c | store | 4 | `sp -0x58` | `ret(0x1ea50)` |
| `ldr` at 0x41ea4 | load | 4 | `abs 0x421f8` | `-` |
| `ldr` at 0x41eb8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41ebc | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x41ee4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41ee8 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x41efc | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x41f04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x41f10 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41f14 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x41f28 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x41f30 | load | 1 | `(unknown)` | `-` |
| `str` at 0x41f34 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x41f38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41f3c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x41f50 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x41f58 | load | 1 | `(unknown)` | `-` |
| `str` at 0x41f5c | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x41f60 | load | 1 | `sp -0x2c` | `-` |
| `ldrb` at 0x41f64 | load | 1 | `sp -0x30` | `-` |
| `ldr` at 0x41f68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41f6c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x41f80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41f84 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x41f88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41f8c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x41f90 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41f94 | load | 1 | `[sp-24] +0x7` | `-` |
| `str` at 0x41fac | store | 4 | `sp -0x24` | `ret(0xb0ae4)` |
| `ldr` at 0x41fb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41fbc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41fc0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x41fd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41fdc | load | 4 | `abs 0x421fc` | `-` |
| `str` at 0x41fe0 | store | 4 | `sp -0x60` | `0xe24d4` |
| `str` at 0x41fe4 | store | 4 | `sp -0x5c` | `ret(0xb14ac)` |
| `str` at 0x41fe8 | store | 4 | `sp -0x58` | `ret(0x1ea50)` |
| `ldr` at 0x41ff0 | load | 4 | `abs 0x421f8` | `-` |
| `ldr` at 0x42004 | load | 4 | `abs 0x42200` | `-` |
| `str` at 0x42008 | store | 4 | `sp -0x60` | `0xe2500` |
| `ldr` at 0x4200c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42010 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x42014 | store | 4 | `sp -0x5c` | `[ret(0x43158)]:1` |
| `ldr` at 0x4201c | load | 4 | `abs 0x421f8` | `-` |
| `ldr` at 0x4202c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x42034 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x42040 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x42044 | load | 4 | `abs 0x42204` | `-` |
| `ldr` at 0x42048 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4204c | load | 4 | `abs 0x119e58` | `-` |
| `ldr` at 0x42068 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4207c | load | 4 | `abs 0x42208` | `-` |
| `ldr` at 0x42080 | load | 4 | `abs 0x119e54` | `-` |
| `ldrb` at 0x42088 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4209c | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0x420a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x420b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x420c8 | load | 4 | `abs 0x42208` | `-` |
| `ldr` at 0x420cc | load | 4 | `abs 0x119e54` | `-` |
| `ldrb` at 0x420d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x420e8 | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0x420f0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x420f4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x420f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4210c | load | 4 | `abs 0x42208` | `-` |
| `ldr` at 0x42110 | load | 4 | `abs 0x119e54` | `-` |
| `ldrb` at 0x42118 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4212c | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0x42134 | load | 1 | `(unknown)` | `-` |
| `str` at 0x42138 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x4213c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42150 | load | 4 | `abs 0x42208` | `-` |
| `ldr` at 0x42154 | load | 4 | `abs 0x119e54` | `-` |
| `ldrb` at 0x4215c | load | 1 | `(unknown)` | `-` |
| `str` at 0x42160 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x42164 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x42178 | load | 4 | `abs 0x42208` | `-` |
| `ldr` at 0x4217c | load | 4 | `abs 0x119e54` | `-` |
| `ldrb` at 0x42184 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x42198 | load | 4 | `sp -0x50` | `-` |
| `ldrh` at 0x421a0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x421a4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x421b4 | store | 4 | `sp -0x4c` | `?` |
| `ldrb` at 0x421b8 | load | 1 | `sp -0x40` | `-` |
| `ldrb` at 0x421bc | load | 1 | `sp -0x44` | `-` |
| `ldr` at 0x421c8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x421cc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x421d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x421dc | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0x421ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x421ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x421ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x421ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41d78 | `(unknown callee) 0x43158` | `[sp-40]:1` | `?` | `?` | `[sp-40]:1` |
| `bl` at 0x41de8 | `(unknown callee) 0xb78b4` | `[ret(0x43158)+3]:1` | `?` | `?` | `[ret(0x43158)+3]:1` |
| `bl` at 0x41e00 | `(unknown callee) 0x1c79c` | `0x4` | `?` | `?` | `ret(0xb78b4)` |
| `bl` at 0x41e14 | `(unknown callee) 0x1c79c` | `0x5` | `?` | `?` | `ret(0x1c79c)` |
| `bl` at 0x41e58 | `(unknown callee) 0xb79a0` | `[[sp-24]+3]:1` | `[[sp-24]+5]:1` | `[sp-29]:1` | `[[sp-24]+5]:1` |
| `bl` at 0x41e7c | `(unknown callee) 0xb8268` | `[[sp-24]+3]:1` | `?` | `?` | `[[sp-24]+3]:1` |
| `bl` at 0x41e88 | `(unknown callee) 0x1ea50` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x41eb0 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0xc6` | `0x0` |
| `bl` at 0x41ec4 | `(unknown callee) 0xaff6c` | `[ret(0x43158)+3]:1` | `?` | `?` | `[ret(0x43158)+3]:1` |
| `bl` at 0x41ee0 | `(unknown callee) 0xaff10` | `sp-60` | `?` | `?` | `sp-60` |
| `bl` at 0x41f7c | `(unknown callee) 0xb9d98` | `[sp-44]:1` | `[sp-48]:1` | `[[sp-24]+4]:1` | `0x1` |
| `bl` at 0x41fa4 | `(unknown callee) 0xb0ae4` | `[[sp-24]+3]:1` | `[[sp-24]+4]:1` | `[[sp-24]+7]:1` | `[[sp-24]+7]:1` |
| `bl` at 0x41fc8 | `(unknown callee) 0xb14ac` | `[[sp-24]+3]:1` | `?` | `?` | `[[sp-24]+3]:1` |
| `bl` at 0x41fd4 | `(unknown callee) 0x1ea50` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x41ffc | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0xdf` | `0x0` |
| `bl` at 0x42028 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0xea` | `0x0` |
| `bl` at 0x42064 | `(unknown callee) 0xaff10` | `sp-80` | `?` | `?` | `sp-80` |
| `bl` at 0x421d0 | `(unknown callee) 0xb9e38` | `[sp-64]:1` | `[sp-68]:1` | `[sp-72]` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41d68 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x41d98 | `[ret(0x43158)]:1 == 0x2` | `[ret(0x43158)]:1 != 0x2` |
| `bgt` at 0x41da4 | `[ret(0x43158)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x43158)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x41db0 | `[ret(0x43158)]:1 == 0x1` | `[ret(0x43158)]:1 != 0x1` |
| `beq` at 0x41dc0 | `[ret(0x43158)]:1 == 0x63` | `[ret(0x43158)]:1 != 0x63` |
| `bne` at 0x41dd4 | `[ret(0x43158)+3]:1 != 0xff` | `[ret(0x43158)+3]:1 == 0xff` |
| `bne` at 0x41df4 | `ret(0xb78b4) != 0x0` | `ret(0xb78b4) == 0x0` |
| `beq` at 0x41e0c | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `beq` at 0x41e6c | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bne` at 0x41ed0 | `ret(0xaff6c) != 0x0` | `ret(0xaff6c) == 0x0` |
| `bne` at 0x41f0c | `? != 0x6` | `? == 0x6` |
| `beq` at 0x41fb8 | `ret(0xb0ae4) == 0x0` | `ret(0xb0ae4) != 0x0` |
| `blt` at 0x42054 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x420ac | `? == 0x6` | `? != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x421ec | `r0` | `0x0` |

## `fanGetTachCfg`

- `fanGetTachCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4220c to 0x42254.
  Established. `beq` takes the zero-entry-r0 path past the store through that entry value (`beq` at
  0x42228, `str` at 0x42238). On fall-through, the word loaded from 0x119e54 is stored at entry r0
  +0x0 (`ldr` at 0x42234, `str` at 0x42238). A separate word load from 0x119e58 supplies the return
  expression `[0x119e58]` (`ldr` at 0x42240, `ldmdb` at 0x42248). The returned word's count meaning
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4221c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x42220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4222c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x42230 | load | 4 | `abs 0x4224c` | `-` |
| `ldr` at 0x42234 | load | 4 | `abs 0x119e54` | `-` |
| `str` at 0x42238 | store | 4 | `r0 +0x0` | `[0x119e54]` |
| `ldr` at 0x4223c | load | 4 | `abs 0x42250` | `-` |
| `ldr` at 0x42240 | load | 4 | `abs 0x119e58` | `-` |
| `ldmdb` at 0x42248 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42248 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42248 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42228 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42248 | `r0` | `[0x119e58]` |

## `fanOn`

- `fanOn`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42254 to 0x424b8.
  Established. The first call targets 0x431f0 with candidates `(r0 & 0xff)`, sp-25, sp-26 and `(r0 &
  0xff)` (`bl` at 0x42284). `beq` takes its zero-result path into the loop, while fall-through
  copies that result to sp -0x28 and jumps to the final join (`beq` at 0x42298, `str` at 0x422a0,
  `b` at 0x422a4). Later calls include 0xb7958 with byte-load candidates from `ret(0x43158)` +1 and
  +2, and 0xb0ae4 with byte-load candidates from the same call expression +3, +4 and +7 (`bl` at
  0x42378, `bl` at 0x423f4). The loop's `bls` predicate has unknown operands (`bls` at 0x422bc). The
  fan-on role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x42268 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4226c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4228c | store | 4 | `sp -0x20` | `ret(0x431f0)` |
| `ldr` at 0x42290 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4229c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x422a0 | store | 4 | `sp -0x28` | `ret(0x431f0)` |
| `ldrb` at 0x422a8 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x422ac | store | 1 | `sp -0x1b` | `([sp-25]:1 & 0xff)` |
| `ldrb` at 0x422b0 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x422b4 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x422c4 | load | 1 | `sp -0x1b` | `-` |
| `ldr` at 0x422c8 | load | 4 | `abs 0x4249c` | `-` |
| `ldr` at 0x422cc | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x422d4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x422e4 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x422f4 | load | 1 | `ret(0x430c4) +0x0` | `-` |
| `str` at 0x42304 | store | 4 | `sp -0x18` | `ret(0x43158)` |
| `ldr` at 0x42308 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4230c | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x42310 | store | 4 | `sp -0x2c` | `[ret(0x43158)]:1` |
| `ldr` at 0x42314 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42320 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4232c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4233c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4234c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42350 | load | 1 | `ret(0x43158) +0x1` | `-` |
| `ldr` at 0x42360 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42364 | load | 1 | `ret(0x43158) +0x1` | `-` |
| `ldr` at 0x42368 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4236c | load | 1 | `ret(0x43158) +0x2` | `-` |
| `str` at 0x42380 | store | 4 | `sp -0x24` | `ret(0xb7958)` |
| `ldr` at 0x42384 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42390 | load | 4 | `abs 0x424a0` | `-` |
| `str` at 0x42394 | store | 4 | `sp -0x34` | `0xe2524` |
| `ldr` at 0x42398 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4239c | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0x423a0 | store | 4 | `sp -0x30` | `[[sp-24]+1]:1` |
| `ldr` at 0x423a8 | load | 4 | `abs 0x424a4` | `-` |
| `ldr` at 0x423ac | load | 4 | `abs 0x424a8` | `-` |
| `ldr` at 0x423b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x423c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x423c8 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x423d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x423d4 | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x423d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x423dc | load | 1 | `ret(0x43158) +0x4` | `-` |
| `ldr` at 0x423e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x423e4 | load | 1 | `ret(0x43158) +0x7` | `-` |
| `str` at 0x423fc | store | 4 | `sp -0x24` | `ret(0xb0ae4)` |
| `ldr` at 0x42400 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4240c | load | 4 | `abs 0x424a0` | `-` |
| `str` at 0x42410 | store | 4 | `sp -0x34` | `0xe2524` |
| `ldr` at 0x42414 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42418 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x4241c | store | 4 | `sp -0x30` | `[[sp-24]+4]:1` |
| `ldr` at 0x42424 | load | 4 | `abs 0x424a4` | `-` |
| `ldr` at 0x42428 | load | 4 | `abs 0x424ac` | `-` |
| `ldr` at 0x42434 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42440 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42444 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x4244c | load | 4 | `abs 0x424b0` | `-` |
| `str` at 0x42450 | store | 4 | `sp -0x34` | `0xe2548` |
| `ldr` at 0x42454 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42458 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x4245c | store | 4 | `sp -0x30` | `[ret(0x43158)]:1` |
| `ldr` at 0x42464 | load | 4 | `abs 0x424a4` | `-` |
| `ldr` at 0x42474 | load | 4 | `abs 0x424b4` | `-` |
| `str` at 0x42478 | store | 4 | `sp -0x20` | `0xfffffa86` |
| `ldrb` at 0x4247c | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x42484 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0x4248c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42490 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42494 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x42498 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42498 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42498 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42284 | `(unknown callee) 0x431f0` | `(r0 & 0xff)` | `sp-25` | `sp-26` | `(r0 & 0xff)` |
| `bl` at 0x422ec | `(unknown callee) 0x430c4` | `?` | `?` | `?` | `?` |
| `bl` at 0x422fc | `(unknown callee) 0x43158` | `[ret(0x430c4)]:1` | `?` | `?` | `[ret(0x430c4)]:1` |
| `bl` at 0x42378 | `(unknown callee) 0xb7958` | `[ret(0x43158)+1]:1` | `[ret(0x43158)+2]:1` | `[ret(0x43158)+1]:1` | `[ret(0x43158)+2]:1` |
| `bl` at 0x423b4 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0x147` | `0x0` |
| `bl` at 0x423f4 | `(unknown callee) 0xb0ae4` | `[ret(0x43158)+3]:1` | `[ret(0x43158)+4]:1` | `[ret(0x43158)+7]:1` | `[ret(0x43158)+7]:1` |
| `bl` at 0x42430 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0x153` | `0x0` |
| `bl` at 0x42470 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0x160` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42298 | `ret(0x431f0) == 0x0` | `ret(0x431f0) != 0x0` |
| `bls` at 0x422bc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x422dc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x4231c | `[ret(0x43158)]:1 == 0x2` | `[ret(0x43158)]:1 != 0x2` |
| `bgt` at 0x42328 | `[ret(0x43158)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x43158)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x42334 | `[ret(0x43158)]:1 == 0x1` | `[ret(0x43158)]:1 != 0x1` |
| `beq` at 0x42344 | `[ret(0x43158)]:1 == 0x63` | `[ret(0x43158)]:1 != 0x63` |
| `bne` at 0x42358 | `[ret(0x43158)+1]:1 != 0xff` | `[ret(0x43158)+1]:1 == 0xff` |
| `beq` at 0x4238c | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `bne` at 0x423c0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x42408 | `ret(0xb0ae4) == 0x0` | `ret(0xb0ae4) != 0x0` |
| `bne` at 0x4243c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42498 | `r0` | `?` |

## `fanOff`

- `fanOff`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x424b8 to 0x42710.
  Established. A call to 0x431f0 precedes `beq`, whose taken zero-result path enters the loop (`bl`
  at 0x424e8, `beq` at 0x424fc); its fall-through copies `ret(0x431f0)` to sp -0x28 and jumps to the
  final join (`str` at 0x42504, `b` at 0x42508). One later call targets 0xb790c with first
  candidates `[ret(0x43158)+1]:1` and `[ret(0x43158)+2]:1` (`bl` at 0x425dc). Another targets
  0xb0ae4 with r2 candidate 0x0 (`bl` at 0x42650). The stored result at sp -0x20 can become
  0xfffffa86 (`str` at 0x426d4). The hardware-off semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x424cc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x424d0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x424f0 | store | 4 | `sp -0x20` | `ret(0x431f0)` |
| `ldr` at 0x424f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42500 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42504 | store | 4 | `sp -0x28` | `ret(0x431f0)` |
| `ldrb` at 0x4250c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x42510 | store | 1 | `sp -0x1b` | `([sp-25]:1 & 0xff)` |
| `ldrb` at 0x42514 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x42518 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x42528 | load | 1 | `sp -0x1b` | `-` |
| `ldr` at 0x4252c | load | 4 | `abs 0x426f8` | `-` |
| `ldr` at 0x42530 | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x42538 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x42548 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x42558 | load | 1 | `ret(0x430c4) +0x0` | `-` |
| `str` at 0x42568 | store | 4 | `sp -0x18` | `ret(0x43158)` |
| `ldr` at 0x4256c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42570 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x42574 | store | 4 | `sp -0x2c` | `[ret(0x43158)]:1` |
| `ldr` at 0x42578 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42584 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42590 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x425a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x425b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x425b4 | load | 1 | `ret(0x43158) +0x1` | `-` |
| `ldr` at 0x425c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x425c8 | load | 1 | `ret(0x43158) +0x1` | `-` |
| `ldr` at 0x425cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x425d0 | load | 1 | `ret(0x43158) +0x2` | `-` |
| `str` at 0x425e4 | store | 4 | `sp -0x24` | `ret(0xb790c)` |
| `ldr` at 0x425e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x425f4 | load | 4 | `abs 0x426fc` | `-` |
| `str` at 0x425f8 | store | 4 | `sp -0x34` | `0xe2568` |
| `ldr` at 0x425fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42600 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0x42604 | store | 4 | `sp -0x30` | `[[sp-24]+1]:1` |
| `ldr` at 0x4260c | load | 4 | `abs 0x42700` | `-` |
| `ldr` at 0x4261c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42628 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4262c | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x42634 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42638 | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x4263c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42640 | load | 1 | `ret(0x43158) +0x4` | `-` |
| `str` at 0x42658 | store | 4 | `sp -0x24` | `ret(0xb0ae4)` |
| `ldr` at 0x4265c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42668 | load | 4 | `abs 0x426fc` | `-` |
| `str` at 0x4266c | store | 4 | `sp -0x34` | `0xe2568` |
| `ldr` at 0x42670 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42674 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x42678 | store | 4 | `sp -0x30` | `[[sp-24]+3]:1` |
| `ldr` at 0x42680 | load | 4 | `abs 0x42700` | `-` |
| `ldr` at 0x42690 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4269c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x426a0 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x426a8 | load | 4 | `abs 0x42704` | `-` |
| `str` at 0x426ac | store | 4 | `sp -0x34` | `0xe258c` |
| `ldr` at 0x426b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x426b4 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x426b8 | store | 4 | `sp -0x30` | `[ret(0x43158)]:1` |
| `ldr` at 0x426c0 | load | 4 | `abs 0x42700` | `-` |
| `ldr` at 0x426c4 | load | 4 | `abs 0x42708` | `-` |
| `ldr` at 0x426d0 | load | 4 | `abs 0x4270c` | `-` |
| `str` at 0x426d4 | store | 4 | `sp -0x20` | `0xfffffa86` |
| `ldrb` at 0x426d8 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x426e0 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0x426e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x426ec | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x426f0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x426f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x426f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x426f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x424e8 | `(unknown callee) 0x431f0` | `(r0 & 0xff)` | `sp-25` | `sp-26` | `(r0 & 0xff)` |
| `bl` at 0x42550 | `(unknown callee) 0x430c4` | `?` | `?` | `?` | `?` |
| `bl` at 0x42560 | `(unknown callee) 0x43158` | `[ret(0x430c4)]:1` | `?` | `?` | `[ret(0x430c4)]:1` |
| `bl` at 0x425dc | `(unknown callee) 0xb790c` | `[ret(0x43158)+1]:1` | `[ret(0x43158)+2]:1` | `[ret(0x43158)+1]:1` | `[ret(0x43158)+2]:1` |
| `bl` at 0x42618 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0x188` | `0x0` |
| `bl` at 0x42650 | `(unknown callee) 0xb0ae4` | `[ret(0x43158)+3]:1` | `[ret(0x43158)+4]:1` | `0x0` | `[ret(0x43158)+4]:1` |
| `bl` at 0x4268c | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0x194` | `0x0` |
| `bl` at 0x426cc | `(unknown callee) 0x1eed0` | `0x1` | `0xe2420` | `0x1a1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x424fc | `ret(0x431f0) == 0x0` | `ret(0x431f0) != 0x0` |
| `bls` at 0x42520 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x42540 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x42580 | `[ret(0x43158)]:1 == 0x2` | `[ret(0x43158)]:1 != 0x2` |
| `bgt` at 0x4258c | `[ret(0x43158)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x43158)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x42598 | `[ret(0x43158)]:1 == 0x1` | `[ret(0x43158)]:1 != 0x1` |
| `beq` at 0x425a8 | `[ret(0x43158)]:1 == 0x63` | `[ret(0x43158)]:1 != 0x63` |
| `bne` at 0x425bc | `[ret(0x43158)+1]:1 != 0xff` | `[ret(0x43158)+1]:1 == 0xff` |
| `beq` at 0x425f0 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |
| `bne` at 0x42624 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x42664 | `ret(0xb0ae4) == 0x0` | `ret(0xb0ae4) != 0x0` |
| `bne` at 0x42698 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x426f4 | `r0` | `?` |

## `fanStatus`

- `fanStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42710 to 0x42900.
  Established. The first call targets 0x3a8c0 (`bl` at 0x42728). `bne` takes its nonzero-result path
  to 0x431f0, while zero fall-through saves 0x0 at sp -0x28 and jumps to the final join (`bne`
  at 0x42734, `str` at 0x4273c, `b` at 0x42740, `bl` at 0x4275c). Later calls include 0xb7b78 and
  0xb0d38. The latter's r2 candidate is sp-33 (`bl` at 0x4284c, `bl` at 0x42894). The following
  `beq` takes the zero `[sp-32]` path to the loop increment (`beq` at 0x428cc, `add` at 0x428e0).
  The merged return and physical status meanings remain unknown (`ldmdb` at 0x428f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x42724 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4273c | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0x42744 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x42764 | store | 4 | `sp -0x20` | `ret(0x431f0)` |
| `ldr` at 0x42768 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42774 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42778 | store | 4 | `sp -0x28` | `ret(0x431f0)` |
| `ldrb` at 0x42780 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x42784 | store | 1 | `sp -0x1b` | `([sp-25]:1 & 0xff)` |
| `ldrb` at 0x42788 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x4278c | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x4279c | load | 1 | `sp -0x1b` | `-` |
| `ldr` at 0x427a0 | load | 4 | `abs 0x428fc` | `-` |
| `ldr` at 0x427a4 | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x427ac | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x427bc | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x427cc | load | 1 | `ret(0x430c4) +0x0` | `-` |
| `str` at 0x427dc | store | 4 | `sp -0x18` | `ret(0x43158)` |
| `ldr` at 0x427e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x427e4 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x427e8 | store | 4 | `sp -0x2c` | `[ret(0x43158)]:1` |
| `ldr` at 0x427ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x427f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42804 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42814 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42824 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42828 | load | 1 | `ret(0x43158) +0x1` | `-` |
| `str` at 0x42838 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x42840 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42844 | load | 1 | `ret(0x43158) +0x1` | `-` |
| `ldr` at 0x42854 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42858 | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0x42870 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x42878 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4287c | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x42880 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42884 | load | 1 | `ret(0x43158) +0x4` | `-` |
| `ldrb` at 0x42898 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x428a8 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x428b4 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x428c0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x428c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x428d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x428d4 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldrb` at 0x428dc | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x428e4 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0x428ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x428f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x428f4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x428f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x428f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x428f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42728 | `(unknown callee) 0x3a8c0` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x4275c | `(unknown callee) 0x431f0` | `[sp-17]:1` | `sp-25` | `sp-26` | `[sp-17]:1` |
| `bl` at 0x427c4 | `(unknown callee) 0x430c4` | `?` | `?` | `?` | `?` |
| `bl` at 0x427d4 | `(unknown callee) 0x43158` | `[ret(0x430c4)]:1` | `?` | `?` | `[ret(0x430c4)]:1` |
| `bl` at 0x4284c | `(unknown callee) 0xb7b78` | `[ret(0x43158)+1]:1` | `?` | `?` | `[ret(0x43158)+1]:1` |
| `bl` at 0x42894 | `(unknown callee) 0xb0d38` | `[ret(0x43158)+3]:1` | `[ret(0x43158)+4]:1` | `sp-33` | `[ret(0x43158)+4]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42734 | `ret(0x3a8c0) != 0x0` | `ret(0x3a8c0) == 0x0` |
| `beq` at 0x42770 | `ret(0x431f0) == 0x0` | `ret(0x431f0) != 0x0` |
| `bls` at 0x42794 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x427b4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x427f4 | `[ret(0x43158)]:1 == 0x2` | `[ret(0x43158)]:1 != 0x2` |
| `bgt` at 0x42800 | `[ret(0x43158)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x43158)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x4280c | `[ret(0x43158)]:1 == 0x1` | `[ret(0x43158)]:1 != 0x1` |
| `beq` at 0x4281c | `[ret(0x43158)]:1 == 0x63` | `[ret(0x43158)]:1 != 0x63` |
| `bne` at 0x42830 | `[ret(0x43158)+1]:1 != 0xff` | `[ret(0x43158)+1]:1 == 0xff` |
| `beq` at 0x428cc | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x428f8 | `r0` | `?` |

## `fanSetSpeedRaw`

- `fanSetSpeedRaw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42900 to 0x42b68.
  Established. The first call targets 0x431f0 with r0 candidate `(r0 & 0xff)` (`bl` at 0x4293c).
  After `beq` takes its zero-result path, later byte tests route into paths selected by the byte
  loaded from `ret(0x43158)` +0x0 (`beq` at 0x42950, `ldrb` at 0x429d4, `beq` at 0x429e4, `beq` at
  0x429fc). The body includes four calls to 0x1abd0: first candidates 0x5 and 0x4 occur on each of
  two paths, and both 0x4 sites have r1 candidate 0x1 (`bl` at 0x42a48, `bl` at 0x42a54, `bl` at
  0x42b1c, `bl` at 0x42b28). The meaning of those numeric candidates and the raw-speed operation
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x42918 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x42920 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x42924 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x42944 | store | 4 | `sp -0x20` | `ret(0x431f0)` |
| `ldr` at 0x42948 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42954 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42958 | store | 4 | `sp -0x28` | `ret(0x431f0)` |
| `ldrb` at 0x42960 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x42964 | store | 1 | `sp -0x1b` | `([sp-25]:1 & 0xff)` |
| `ldrb` at 0x42968 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x4296c | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x4297c | load | 1 | `sp -0x1b` | `-` |
| `ldr` at 0x42980 | load | 4 | `abs 0x42b58` | `-` |
| `ldr` at 0x42984 | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x4298c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x42998 | load | 1 | `sp -0x1b` | `-` |
| `ldr` at 0x4299c | load | 4 | `abs 0x42b5c` | `-` |
| `ldr` at 0x429a0 | load | 4 | `abs 0x119e58` | `-` |
| `ldrb` at 0x429ac | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x429bc | load | 1 | `ret(0x430c4) +0x0` | `-` |
| `str` at 0x429cc | store | 4 | `sp -0x18` | `ret(0x43158)` |
| `ldr` at 0x429d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x429d4 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x429d8 | store | 4 | `sp -0x2c` | `[ret(0x43158)]:1` |
| `ldr` at 0x429dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x429e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x429f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42a04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42a14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42a18 | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x42a24 | load | 4 | `abs 0x42b60` | `-` |
| `str` at 0x42a28 | store | 4 | `sp -0x28` | `0xfffffa87` |
| `ldrb` at 0x42a30 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x42a38 | store | 1 | `sp -0x1c` | `?` |
| `ldrb` at 0x42a3c | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x42a58 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x42a5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42a60 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x42a68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42a70 | load | 1 | `[sp-24] +0x5` | `-` |
| `strb` at 0x42a78 | store | 1 | `sp -0x1c` | `?` |
| `ldr` at 0x42a7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42a80 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x42a84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42a88 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0x42a8c | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x42aa0 | store | 4 | `sp -0x24` | `ret(0xb79a0)` |
| `ldr` at 0x42aa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42ab0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42abc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42ac0 | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `ldr` at 0x42ac8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42acc | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x42ad0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42ad4 | load | 1 | `ret(0x43158) +0x4` | `-` |
| `ldrb` at 0x42ad8 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x42aec | store | 4 | `sp -0x24` | `ret(0xb0ae4)` |
| `ldr` at 0x42af0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42afc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42b08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42b0c | store | 4 | `sp -0x20` | `ret(0xb0ae4)` |
| `ldrb` at 0x42b10 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x42b30 | load | 4 | `abs 0x42b64` | `-` |
| `str` at 0x42b34 | store | 4 | `sp -0x20` | `0xfffffa86` |
| `ldrb` at 0x42b38 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x42b40 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0x42b48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42b4c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42b50 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x42b54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42b54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42b54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4293c | `(unknown callee) 0x431f0` | `(r0 & 0xff)` | `sp-25` | `sp-26` | `(r0 & 0xff)` |
| `bl` at 0x429b4 | `(unknown callee) 0x430c4` | `?` | `?` | `[0x119e58]` | `?` |
| `bl` at 0x429c4 | `(unknown callee) 0x43158` | `[ret(0x430c4)]:1` | `?` | `?` | `[ret(0x430c4)]:1` |
| `bl` at 0x42a48 | `(unknown callee) 0x1abd0` | `0x5` | `[sp-28]:1` | `?` | `[sp-28]:1` |
| `bl` at 0x42a54 | `(unknown callee) 0x1abd0` | `0x4` | `0x1` | `?` | `?` |
| `bl` at 0x42a98 | `(unknown callee) 0xb79a0` | `[[sp-24]+3]:1` | `[[sp-24]+5]:1` | `[sp-28]:1` | `[[sp-24]+5]:1` |
| `bl` at 0x42ae4 | `(unknown callee) 0xb0ae4` | `[ret(0x43158)+3]:1` | `[ret(0x43158)+4]:1` | `[sp-18]:1` | `[ret(0x43158)+4]:1` |
| `bl` at 0x42b1c | `(unknown callee) 0x1abd0` | `0x5` | `[sp-18]:1` | `?` | `[sp-18]:1` |
| `bl` at 0x42b28 | `(unknown callee) 0x1abd0` | `0x4` | `0x1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42950 | `ret(0x431f0) == 0x0` | `ret(0x431f0) != 0x0` |
| `bls` at 0x42974 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x42994 | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x429a8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x429e4 | `[ret(0x43158)]:1 == 0x2` | `[ret(0x43158)]:1 != 0x2` |
| `bgt` at 0x429f0 | `[ret(0x43158)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x43158)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x429fc | `[ret(0x43158)]:1 == 0x1` | `[ret(0x43158)]:1 != 0x1` |
| `beq` at 0x42a0c | `[ret(0x43158)]:1 == 0x63` | `[ret(0x43158)]:1 != 0x63` |
| `bne` at 0x42a20 | `[ret(0x43158)+3]:1 != 0xff` | `[ret(0x43158)+3]:1 == 0xff` |
| `beq` at 0x42aac | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bne` at 0x42ab8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x42af8 | `ret(0xb0ae4) == 0x0` | `ret(0xb0ae4) != 0x0` |
| `bne` at 0x42b04 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42b54 | `r0` | `?` |

## `fanSetSpeed`

- `fanSetSpeed`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42b68 to 0x42d44.
  Established. The saved byte `[sp-18]:1` decides the `beq` sites for 0x1, 0x0 and 0x2 (`beq` at
  0x42c74, `beq` at 0x42c8c, `beq` at 0x42c9c). The 0x0, 0x1 and 0x2 paths respectively select
  masked byte-load expressions from `ret(0x43158)` +7, +8 and +9 for the byte save at sp -0x19
  (`strb` at 0x42cac, `strb` at 0x42cbc, `strb` at 0x42ccc). The default path saves `([sp-18]:1 & 0xff)` there (`strb` at 0x42cd8). The
  later call to 0x42900 has first candidate `[sp-28]:1` and an unknown second candidate (`bl` at
  0x42cec). The preset and physical-speed meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x42b80 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x42b88 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x42b8c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x42bac | store | 4 | `sp -0x20` | `ret(0x431f0)` |
| `ldr` at 0x42bb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42bbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42bc0 | store | 4 | `sp -0x28` | `ret(0x431f0)` |
| `ldrb` at 0x42bc8 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x42bcc | store | 1 | `sp -0x1c` | `([sp-26]:1 & 0xff)` |
| `ldrb` at 0x42bd0 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x42bd4 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x42be4 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x42be8 | load | 4 | `abs 0x42d38` | `-` |
| `ldr` at 0x42bec | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x42bf4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x42c00 | load | 1 | `sp -0x1c` | `-` |
| `ldr` at 0x42c04 | load | 4 | `abs 0x42d3c` | `-` |
| `ldr` at 0x42c08 | load | 4 | `abs 0x119e58` | `-` |
| `ldrb` at 0x42c14 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x42c24 | load | 1 | `ret(0x430c4) +0x0` | `-` |
| `str` at 0x42c34 | store | 4 | `sp -0x18` | `ret(0x43158)` |
| `ldr` at 0x42c38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42c3c | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldrb` at 0x42c48 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x42c58 | load | 4 | `abs 0x42d40` | `-` |
| `str` at 0x42c5c | store | 4 | `sp -0x28` | `0xfffffa87` |
| `ldrb` at 0x42c64 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x42c68 | store | 4 | `sp -0x2c` | `[sp-18]:1` |
| `ldr` at 0x42c6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42c78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42c84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42c94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42ca4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42ca8 | load | 1 | `ret(0x43158) +0x7` | `-` |
| `strb` at 0x42cac | store | 1 | `sp -0x19` | `([ret(0x43158)+7]:1 & 0xff)` |
| `ldr` at 0x42cb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42cb8 | load | 1 | `ret(0x43158) +0x8` | `-` |
| `strb` at 0x42cbc | store | 1 | `sp -0x19` | `([ret(0x43158)+8]:1 & 0xff)` |
| `ldr` at 0x42cc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42cc8 | load | 1 | `ret(0x43158) +0x9` | `-` |
| `strb` at 0x42ccc | store | 1 | `sp -0x19` | `([ret(0x43158)+9]:1 & 0xff)` |
| `ldrb` at 0x42cd4 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x42cd8 | store | 1 | `sp -0x19` | `([sp-18]:1 & 0xff)` |
| `ldrb` at 0x42cdc | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0x42ce0 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x42cf4 | store | 4 | `sp -0x24` | `ret(0x42900)` |
| `ldr` at 0x42cf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x42d04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x42d10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42d14 | store | 4 | `sp -0x20` | `ret(0x42900)` |
| `ldrb` at 0x42d18 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x42d20 | store | 1 | `sp -0x1c` | `?` |
| `ldr` at 0x42d28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x42d2c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x42d30 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x42d34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42d34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42d34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42ba4 | `(unknown callee) 0x431f0` | `(r0 & 0xff)` | `sp-26` | `sp-27` | `(r0 & 0xff)` |
| `bl` at 0x42c1c | `(unknown callee) 0x430c4` | `?` | `?` | `[0x119e58]` | `?` |
| `bl` at 0x42c2c | `(unknown callee) 0x43158` | `[ret(0x430c4)]:1` | `?` | `?` | `[ret(0x430c4)]:1` |
| `bl` at 0x42cec | `(unknown callee) 0x42900` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x42bb8 | `ret(0x431f0) == 0x0` | `ret(0x431f0) != 0x0` |
| `bls` at 0x42bdc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x42bfc | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x42c10 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x42c44 | `[ret(0x43158)+3]:1 != 0xff` | `[ret(0x43158)+3]:1 == 0xff` |
| `bne` at 0x42c50 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `beq` at 0x42c74 | `[sp-18]:1 == 0x1` | `[sp-18]:1 != 0x1` |
| `bgt` at 0x42c80 | `[sp-18]:1 > 0x1 (signed, 32-bit)` | `[sp-18]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x42c8c | `[sp-18]:1 == 0x0` | `[sp-18]:1 != 0x0` |
| `beq` at 0x42c9c | `[sp-18]:1 == 0x2` | `[sp-18]:1 != 0x2` |
| `beq` at 0x42d00 | `ret(0x42900) == 0x0` | `ret(0x42900) != 0x0` |
| `bne` at 0x42d0c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42d34 | `r0` | `?` |

## `fanGetSpeed`

- `fanGetSpeed`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42d44 to 0x42ec0.
  Established. After the call to 0x430c4, `bne` takes the nonzero indexed-byte path to 0x43158
  (`bl` at 0x42d68, `bne` at 0x42d8c, `bl` at 0x42da8); its zero fall-through stores 0xfffffa85 at
  sp -0x28 (`str` at 0x42d94). The byte-save at sp -0x21 contains `(ret(0xb7b78) & 0xff)` after the
  call to 0xb7b78 (`bl` at 0x42e20, `strb` at 0x42e28). Later byte stores at `[sp-24] +0x0` include
  an unknown value and 0x0 on separate paths (`strb` at 0x42e50, `strb` at 0x42e8c). The speed
  conversion is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42d58 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x42d5c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x42d60 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x42d70 | store | 4 | `sp -0x1c` | `ret(0x430c4)` |
| `ldrb` at 0x42d74 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x42d78 | load | 4 | `abs 0x42eb0` | `-` |
| `ldr` at 0x42d7c | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x42d84 | load | 1 | `([sp-17]:1 + [0x119e64]) +0x0` | `-` |
| `ldr` at 0x42d90 | load | 4 | `abs 0x42eb4` | `-` |
| `str` at 0x42d94 | store | 4 | `sp -0x28` | `0xfffffa85` |
| `ldr` at 0x42d9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x42da0 | load | 1 | `ret(0x430c4) +0x0` | `-` |
| `str` at 0x42db0 | store | 4 | `sp -0x20` | `ret(0x43158)` |
| `ldr` at 0x42db4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42db8 | load | 1 | `ret(0x43158) +0x0` | `-` |
| `str` at 0x42dbc | store | 4 | `sp -0x2c` | `[ret(0x43158)]:1` |
| `ldr` at 0x42dc0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42dcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42dd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42de8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x42df8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42dfc | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x42e08 | load | 4 | `abs 0x42eb8` | `-` |
| `str` at 0x42e0c | store | 4 | `sp -0x28` | `0xfffffa87` |
| `ldr` at 0x42e14 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42e18 | load | 1 | `ret(0x43158) +0x3` | `-` |
| `strb` at 0x42e28 | store | 1 | `sp -0x21` | `(ret(0xb7b78) & 0xff)` |
| `ldr` at 0x42e2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x42e30 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x42e38 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42e3c | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x42e44 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42e48 | load | 1 | `[sp-32] +0x4` | `-` |
| `strb` at 0x42e50 | store | 1 | `[sp-24] +0x0` | `?` |
| `ldr` at 0x42e58 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42e5c | load | 1 | `ret(0x43158) +0x3` | `-` |
| `ldr` at 0x42e60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x42e64 | load | 1 | `ret(0x43158) +0x4` | `-` |
| `ldr` at 0x42e70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42e7c | store | 4 | `sp -0x28` | `ret(0xb0d38)` |
| `ldr` at 0x42e84 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x42e8c | store | 1 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x42e94 | load | 4 | `abs 0x42ebc` | `-` |
| `str` at 0x42e98 | store | 4 | `sp -0x28` | `0xfffffa86` |
| `str` at 0x42ea4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x42ea8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x42eac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x42eac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x42eac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42d68 | `(unknown callee) 0x430c4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x42da8 | `(unknown callee) 0x43158` | `[ret(0x430c4)]:1` | `?` | `[sp-17]:1` | `[ret(0x430c4)]:1` |
| `bl` at 0x42e20 | `(unknown callee) 0xb7b78` | `[ret(0x43158)+3]:1` | `?` | `?` | `[ret(0x43158)+3]:1` |
| `bl` at 0x42e74 | `(unknown callee) 0xb0d38` | `[ret(0x43158)+3]:1` | `[ret(0x43158)+4]:1` | `[sp-24]` | `[ret(0x43158)+4]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42d8c | `[([sp-17]:1 + [0x119e64])]:1 != 0x0` | `[([sp-17]:1 + [0x119e64])]:1 == 0x0` |
| `beq` at 0x42dc8 | `[ret(0x43158)]:1 == 0x2` | `[ret(0x43158)]:1 != 0x2` |
| `bgt` at 0x42dd4 | `[ret(0x43158)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x43158)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x42de0 | `[ret(0x43158)]:1 == 0x1` | `[ret(0x43158)]:1 != 0x1` |
| `beq` at 0x42df0 | `[ret(0x43158)]:1 == 0x63` | `[ret(0x43158)]:1 != 0x63` |
| `bne` at 0x42e04 | `[ret(0x43158)+3]:1 != 0xff` | `[ret(0x43158)+3]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x42eac | `r0` | `?` |

## `fanGetReading`

- `fanGetReading`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x42ec0 to 0x43064.
  Established. The first `bne` takes the nonzero indexed-byte path to the conditional computed
  transfer (`bne` at 0x42f08, `ldrls` at 0x42f2c). Zero fall-through saves 0xfffffa85 at sp -0x28
  (`str` at 0x42f10). The `ldrls` taken condition is `? <= 0x7 (unsigned, 32-bit)`, and its taken
  edge is unresolved (`ldrls` at 0x42f2c). The known fall-through later stores 0x3f800000 at sp
  -0x20 (`str` at 0x42f64). Among the later known paths, `blt` takes the negative `[[sp-24]]` path,
  and the final word store writes `ret(0xd69bc)` at `[sp-44] +0x0` (`blt` at 0x42fc4, `str` at
  0x43038). Additional effects of the unresolved edge and the reading's units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x42ed4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x42ed8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x42edc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x42eec | store | 4 | `sp -0x24` | `ret(0x430c4)` |
| `ldrb` at 0x42ef0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x42ef4 | load | 4 | `abs 0x4304c` | `-` |
| `ldr` at 0x42ef8 | load | 4 | `abs 0x119e64` | `-` |
| `ldrb` at 0x42f00 | load | 1 | `([sp-17]:1 + [0x119e64]) +0x0` | `-` |
| `ldr` at 0x42f0c | load | 4 | `abs 0x43050` | `-` |
| `str` at 0x42f10 | store | 4 | `sp -0x28` | `0xfffffa85` |
| `ldr` at 0x42f18 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42f1c | load | 1 | `ret(0x430c4) +0x8` | `-` |
| `ldrls` at 0x42f2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x42f60 | load | 4 | `abs 0x43058` | `-` |
| `str` at 0x42f64 | store | 4 | `sp -0x20` | `0x3f800000` |
| `ldr` at 0x42f80 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42f84 | load | 1 | `ret(0x430c4) +0x1` | `-` |
| `ldr` at 0x42f88 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x42f8c | load | 1 | `ret(0x430c4) +0x2` | `-` |
| `ldr` at 0x42f98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42fa4 | store | 4 | `sp -0x1c` | `ret(0xb09b4)` |
| `ldr` at 0x42fa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42fac | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0x42fb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x42fb4 | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0x42fb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x42fbc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x42fc8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x42fcc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x42fd4 | store | 4 | `sp -0x34` | `ret(0xd65a4)` |
| `ldr` at 0x42fdc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x42fe0 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x42fe8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x42fec | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x43000 | store | 4 | `sp -0x34` | `ret(0xd65a4)` |
| `ldr` at 0x43004 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x43008 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x43014 | store | 4 | `sp -0x34` | `ret(0xd63dc)` |
| `ldr` at 0x43018 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x4301c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43034 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x43038 | store | 4 | `[sp-44] +0x0` | `ret(0xd69bc)` |
| `ldr` at 0x4303c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x43040 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x43044 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x43048 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x43048 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x43048 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x42ee4 | `(unknown callee) 0x430c4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x42f9c | `(unknown callee) 0xb09b4` | `[ret(0x430c4)+1]:1` | `[ret(0x430c4)+2]:1` | `[sp-24]` | `[ret(0x430c4)+2]:1` |
| `bl` at 0x42fd0 | `(unknown callee) 0xd65a4` | `[[sp-24]]` | `[sp-24]` | `[sp-24]` | `[[sp-24]]` |
| `bl` at 0x42ffc | `(unknown callee) 0xd65a4` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0x4300c | `(unknown callee) 0xd63dc` | `ret(0xd65a4)` | `ret(0xd65a4)` | `?` | `?` |
| `bl` at 0x43020 | `(unknown callee) 0xd6780` | `?` | `[sp-32]` | `?` | `?` |
| `bl` at 0x4302c | `(unknown callee) 0xd69bc` | `ret(0xd6780)` | `?` | `?` | `ret(0xd6780)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x42f08 | `[([sp-17]:1 + [0x119e64])]:1 != 0x0` | `[([sp-17]:1 + [0x119e64])]:1 == 0x0` |
| `ldrls` at 0x42f2c | `? <= 0x7 (unsigned, 32-bit)` | `? > 0x7 (unsigned, 32-bit)` |
| `blt` at 0x42fc4 | `[[sp-24]] < 0x0 (signed, 32-bit)` | `[[sp-24]] >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x43048 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x42f2c).

## `fanLocation`

- `fanLocation`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x43064 to 0x430c4.
  Established. The call to 0x430c4 has first candidate `(r0 & 0xff)` (`bl` at 0x43084). A word load
  from that result +0x4 is copied to sp -0x1c (`ldr` at 0x43094, `str` at 0x43098). A second load
  from the same effective location decides `bne`, whose taken nonzero path skips the fallback store
  of 0xe25ac (`ldr` at 0x430a0, `bne` at 0x430a8, `str` at 0x430b0). The two loads need not agree,
  and the pointed value's location-string role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x43078 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4307c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4308c | store | 4 | `sp -0x18` | `ret(0x430c4)` |
| `ldr` at 0x43090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43094 | load | 4 | `ret(0x430c4) +0x4` | `-` |
| `str` at 0x43098 | store | 4 | `sp -0x1c` | `[ret(0x430c4)+4]` |
| `ldr` at 0x4309c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x430a0 | load | 4 | `ret(0x430c4) +0x4` | `-` |
| `ldr` at 0x430ac | load | 4 | `abs 0x430c0` | `-` |
| `str` at 0x430b0 | store | 4 | `sp -0x1c` | `0xe25ac` |
| `ldr` at 0x430b4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x430bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x430bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x430bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x43084 | `(unknown callee) 0x430c4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x430a8 | `[ret(0x430c4)+4] != 0x0` | `[ret(0x430c4)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x430bc | `r0` | `?` |
