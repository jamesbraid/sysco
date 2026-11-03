# ARM Linux 2.6 regulator status and automatic fan paths in 1.44.0

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
Computed transfers in autofanEnv and autofanEnvQuery leave unresolved edges. Their tables
cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-regulator-autofan-state-facts.txt) retain all tool sections.

## `vrmEnabled`

- `vrmEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7ebc0 to 0x7ecc0.
  Established. A word store puts 0xfffff9b9 at sp-0x20 on one path (`str` at 0x7ec18).
  Another path loads the byte at offset nine of the pointer from sp-0x1c and stores it as a word at sp-0x20 (`ldrb` at 0x7ec58, `str` at 0x7ec5c).
  The later computed byte load has an unknown effective address, and the exit value is unknown, leaving the enabled-state interpretation unestablished (`ldrb` at 0x7ec8c, `ldm` at 0x7ecb0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7ebd0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7ebd4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x7ebd8 | load | 4 | `abs 0x7ecb4` | `-` |
| `ldr` at 0x7ebdc | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7ebe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7ebfc | load | 4 | `abs 0x7ecb8` | `-` |
| `ldr` at 0x7ec00 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7ec08 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7ec14 | load | 4 | `abs 0x7ecbc` | `-` |
| `str` at 0x7ec18 | store | 4 | `sp -0x20` | `0xfffff9b9` |
| `ldr` at 0x7ec20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7ec34 | load | 4 | `abs 0x7ecb8` | `-` |
| `ldr` at 0x7ec38 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7ec40 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7ec44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7ec48 | load | 1 | `[sp-28] +0x9` | `-` |
| `ldr` at 0x7ec54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7ec58 | load | 1 | `[sp-28] +0x9` | `-` |
| `str` at 0x7ec5c | store | 4 | `sp -0x20` | `[[sp-28]+9]:1` |
| `ldr` at 0x7ec64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7ec68 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x7ec6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ec78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7ec7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ec84 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x7ec8c | load | 1 | `(unknown)` | `-` |
| `str` at 0x7ec98 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x7eca4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x7eca8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x7ecb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7ecb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7ecb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7ebe4 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7ec10 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7ec50 | `[[sp-28]+9]:1 == 0x0` | `[[sp-28]+9]:1 != 0x0` |
| `ble` at 0x7ec74 | `[[sp-28]+8]:1 <= r1 (signed, 32-bit)` | `[[sp-28]+8]:1 > r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7ecb0 | `r0` | `?` |

## `vrmPresent`

- `vrmPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7edd0 to 0x7ef28.
  Established. A byte load uses the low byte of entry r1 plus the word at offset four of the pointer from sp-0x1c (`ldrb` at 0x7ee7c).
  The call to unnamed 0x125b8c has candidate values entry r0, the byte at the pointer from sp-0x20, sp-0x16 and that same byte (`bl` at 0x7eed0);
  its zero-result branch reaches the later call to unnamed 0x7ed48, whose candidate values are the pointer from sp-0x20, the byte at sp-0x16, unknown and that same byte (`beq` at 0x7eedc, `bl` at 0x7eef8).
  The physical presence test and recorded exit value remain unknown (`ldm` at 0x7ef18).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7ede0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7ede8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7edec | load | 4 | `abs 0x7ef1c` | `-` |
| `ldr` at 0x7edf0 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7edfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7ee10 | load | 4 | `abs 0x7ef20` | `-` |
| `ldr` at 0x7ee14 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7ee1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7ee28 | load | 4 | `abs 0x7ef24` | `-` |
| `str` at 0x7ee2c | store | 4 | `sp -0x24` | `0xfffff9b9` |
| `ldr` at 0x7ee34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7ee48 | load | 4 | `abs 0x7ef20` | `-` |
| `ldr` at 0x7ee4c | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7ee54 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7ee58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7ee5c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x7ee60 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7ee6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7ee70 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7ee74 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x7ee7c | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldr` at 0x7ee88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7ee8c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7ee94 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x7ee9c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7eea0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7eea4 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0x7eeb4 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x7eebc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7eec0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x7eec8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7eee4 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x7eeec | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7eef0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7ef00 | store | 4 | `sp -0x24` | `ret(0x7ed48)` |
| `str` at 0x7ef0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7ef10 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x7ef18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7ef18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7ef18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7eed0 | `(unknown callee) 0x125b8c` | `r0` | `[[sp-32]]:1` | `sp-22` | `[[sp-32]]:1` |
| `bl` at 0x7eef8 | `(unknown callee) 0x7ed48` | `[sp-32]` | `[sp-22]:1` | `?` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7edf8 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7ee24 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x7ee68 | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |
| `beq` at 0x7ee84 | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x7eeac | `[[sp-32]+3]:1 != 0x0` | `[[sp-32]+3]:1 == 0x0` |
| `beq` at 0x7eedc | `ret(0x125b8c) == 0x0` | `ret(0x125b8c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7ef18 | `r0` | `?` |

## `vrmOK`

- `vrmOK`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7ef28 to 0x7f14c.
  Established. The call to unnamed 0x586fc has candidate values the word at sp-0x14, the byte at offset one of the pointer from sp-0x20, unknown and that same byte (`bl` at 0x7f060).
  The following coprocessor load addresses absolute 0x7f13c, with width and result unknown (`ldc` at 0x7f068).
  Later word stores can put zero, one or a result of unnamed 0x7ecc0 at sp-0x24, without establishing a hardware health criterion (`str` at 0x7f0f8, `str` at 0x7f104, `bl` at 0x7f118, `str` at 0x7f120).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7ef38 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7ef40 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7ef44 | load | 4 | `abs 0x7f140` | `-` |
| `ldr` at 0x7ef48 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7ef54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7ef68 | load | 4 | `abs 0x7f144` | `-` |
| `ldr` at 0x7ef6c | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7ef74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7ef80 | load | 4 | `abs 0x7f148` | `-` |
| `str` at 0x7ef84 | store | 4 | `sp -0x24` | `0xfffff9b9` |
| `ldr` at 0x7ef8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7efa0 | load | 4 | `abs 0x7f144` | `-` |
| `ldr` at 0x7efa4 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7efac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7efb0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7efb4 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x7efb8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7efc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7efc8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7efcc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x7efd4 | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldrb` at 0x7efe0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7efe4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7f000 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7f008 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f00c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7f014 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x7f01c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7f020 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7f024 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x7f02c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7f048 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7f050 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7f054 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0x7f058 | load | 4 | `sp -0x14` | `-` |
| `ldc` at 0x7f068 | load | unknown | `abs 0x7f13c` | `-` |
| `ldrb` at 0x7f078 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x7f080 | store | 1 | `sp -0x16` | `?` |
| `ldr` at 0x7f084 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7f088 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x7f098 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7f09c | load | 1 | `[sp-32] +0x4` | `-` |
| `ldrb` at 0x7f0a8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7f0ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7f0c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7f0cc | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0x7f0d8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7f0dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7f0f8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x7f104 | store | 4 | `sp -0x24` | `0x1` |
| `ldrb` at 0x7f10c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7f110 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7f120 | store | 4 | `sp -0x24` | `ret(0x7ecc0)` |
| `str` at 0x7f12c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7f130 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x7f138 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7f138 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7f138 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7efec | `(unknown callee) 0x7edd0` | `r0` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x7f034 | `(unknown callee) 0x125b8c` | `[sp-20]` | `[[sp-32]]:1` | `sp-22` | `[[sp-32]]:1` |
| `bl` at 0x7f060 | `(unknown callee) 0x586fc` | `[sp-20]` | `[[sp-32]+1]:1` | `?` | `[[sp-32]+1]:1` |
| `bl` at 0x7f0b4 | `(unknown callee) 0x7ecc0` | `[sp-32]` | `[sp-22]:1` | `?` | `[sp-22]:1` |
| `bl` at 0x7f0e4 | `(unknown callee) 0x7ed48` | `[sp-32]` | `[sp-22]:1` | `?` | `[sp-22]:1` |
| `bl` at 0x7f118 | `(unknown callee) 0x7ecc0` | `[sp-32]` | `[sp-22]:1` | `?` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7ef50 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7ef7c | `? != 0x0` | `? == 0x0` |
| `bls` at 0x7efc0 | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |
| `beq` at 0x7efdc | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x7eff8 | `ret(0x7edd0) != 0x0` | `ret(0x7edd0) == 0x0` |
| `beq` at 0x7f040 | `ret(0x125b8c) == 0x0` | `ret(0x125b8c) != 0x0` |
| `beq` at 0x7f070 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x7f094 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f0a4 | `[[sp-32]+4]:1 == 0x0` | `[[sp-32]+4]:1 != 0x0` |
| `bne` at 0x7f0c0 | `ret(0x7ecc0) != 0x0` | `ret(0x7ecc0) == 0x0` |
| `beq` at 0x7f0d4 | `[[sp-32]+3]:1 == 0x0` | `[[sp-32]+3]:1 != 0x0` |
| `bne` at 0x7f0f0 | `ret(0x7ed48) != 0x0` | `ret(0x7ed48) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7f138 | `r0` | `?` |

## `vrmAllPresent`

- `vrmAllPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7f14c to 0x7f278.
  Established. The word at sp-0x18 starts at one and can be cleared after the call to unnamed 0x7f764 (`str` at 0x7f164, `bl` at 0x7f238, `str` at 0x7f240).
  That call has candidate values the word at sp-0x14, the word at sp-0x24, 0x1665e8 and a result of unnamed 0x7edd0 (`bl` at 0x7f238).
  A different path stores 0xfffff9b9 at sp-0x28, while the exit value is unknown (`str` at 0x7f1a8, `ldm` at 0x7f264).
  These stack values do not prove that all physical regulators are present.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7f15c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7f164 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x7f168 | load | 4 | `abs 0x7f268` | `-` |
| `ldr` at 0x7f16c | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7f178 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f18c | load | 4 | `abs 0x7f26c` | `-` |
| `ldr` at 0x7f190 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7f198 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7f1a4 | load | 4 | `abs 0x7f270` | `-` |
| `str` at 0x7f1a8 | store | 4 | `sp -0x28` | `0xfffff9b9` |
| `ldr` at 0x7f1b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f1c4 | load | 4 | `abs 0x7f26c` | `-` |
| `ldr` at 0x7f1c8 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7f1d0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x7f1d8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7f1dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7f1e0 | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x7f1e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7f1f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7f1f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7f1fc | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x7f204 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7f208 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7f210 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f22c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f230 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7f234 | load | 4 | `abs 0x7f274` | `-` |
| `str` at 0x7f240 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x7f244 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7f24c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x7f254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f258 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7f25c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x7f264 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7f264 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7f264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7f218 | `(unknown callee) 0x7edd0` | `?` | `?` | `?` | `?` |
| `bl` at 0x7f238 | `(unknown callee) 0x7f764` | `[sp-20]` | `[sp-36]` | `0x1665e8` | `ret(0x7edd0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7f174 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7f1a0 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7f1ec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7f224 | `ret(0x7edd0) == 0x0` | `ret(0x7edd0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7f264 | `r0` | `?` |

## `vrmAllOK`

- `vrmAllOK`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7f278 to 0x7f3cc.
  Established. A word store initializes the word at sp-0x18 to one, and another can replace it with zero (`str` at 0x7f290, `str` at 0x7f390).
  The coprocessor load from absolute 0x7f3b8 has unknown width and result (`ldc` at 0x7f370).
  The call to unnamed 0x7f764 has candidate values the word at sp-0x14, the word at sp-0x24, 0x1665f4 and unknown, leaving the physical all-OK condition unknown (`bl` at 0x7f388).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7f288 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7f290 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x7f294 | load | 4 | `abs 0x7f3bc` | `-` |
| `ldr` at 0x7f298 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7f2a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f2b8 | load | 4 | `abs 0x7f3c0` | `-` |
| `ldr` at 0x7f2bc | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7f2c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7f2d0 | load | 4 | `abs 0x7f3c4` | `-` |
| `str` at 0x7f2d4 | store | 4 | `sp -0x28` | `0xfffff9b9` |
| `ldr` at 0x7f2dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f2f0 | load | 4 | `abs 0x7f3c0` | `-` |
| `ldr` at 0x7f2f4 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7f2fc | store | 4 | `sp -0x20` | `?` |
| `str` at 0x7f304 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7f308 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7f30c | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x7f310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7f31c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7f320 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7f328 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x7f330 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7f334 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7f33c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f358 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7f35c | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x7f360 | load | 4 | `sp -0x14` | `-` |
| `ldc` at 0x7f370 | load | unknown | `abs 0x7f3b8` | `-` |
| `ldr` at 0x7f37c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f380 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7f384 | load | 4 | `abs 0x7f3c8` | `-` |
| `str` at 0x7f390 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x7f394 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7f39c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x7f3a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f3a8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7f3ac | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x7f3b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7f3b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7f3b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7f344 | `(unknown callee) 0x7ef28` | `?` | `?` | `?` | `?` |
| `bl` at 0x7f368 | `(unknown callee) 0x586fc` | `[sp-20]` | `[[sp-36]+1]:1` | `?` | `[[sp-36]+1]:1` |
| `bl` at 0x7f388 | `(unknown callee) 0x7f764` | `[sp-20]` | `[sp-36]` | `0x1665f4` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7f2a0 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7f2cc | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7f318 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7f350 | `ret(0x7ef28) == 0x0` | `ret(0x7ef28) != 0x0` |
| `beq` at 0x7f378 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7f3b4 | `r0` | `?` |

## `vrmInt`

- `vrmInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7f3cc to 0x7f764.
  Established. The low bytes of entry r1, r2 and r3 are saved at sp-0x15, sp-0x16 and sp-0x17 (`strb` at 0x7f3e8, `strb` at 0x7f3f0, `strb` at 0x7f3f8).
  The call to unnamed 0x3ee14 is followed by a signed branch to 0x7f41c when its result exceeds zero, otherwise execution falls through to the call to unnamed 0x7e970 (`bl` at 0x7f400, `bgt` at 0x7f40c, `bl` at 0x7f414).
  A later call to unnamed 0xa9740 has candidate values zero, 0x3500, 0x1a4 and 0x1666b4 (`bl` at 0x7f694).
  Coprocessor width and result remain unknown, so these paths do not establish an interrupt's hardware cause (`ldc` at 0x7f648).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7f3dc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7f3e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7f3f0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x7f3f8 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x7f3fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f41c | load | 4 | `abs 0x7f734` | `-` |
| `ldr` at 0x7f420 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7f42c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f440 | load | 4 | `abs 0x7f738` | `-` |
| `ldr` at 0x7f444 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7f44c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7f45c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f470 | load | 4 | `abs 0x7f738` | `-` |
| `ldr` at 0x7f474 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7f47c | store | 4 | `sp -0x64` | `?` |
| `str` at 0x7f484 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x7f488 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0x7f48c | load | 1 | `[sp-100] +0x8` | `-` |
| `ldr` at 0x7f490 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x7f49c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x7f4a0 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x7f4a8 | load | 4 | `[sp-100] +0x0` | `-` |
| `str` at 0x7f4b0 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x7f4b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f4cc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x7f4d0 | load | 1 | `[sp-104] +0x0` | `-` |
| `ldrb` at 0x7f4d4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7f4e0 | load | 4 | `abs 0x7f73c` | `-` |
| `ldr` at 0x7f4e4 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldrb` at 0x7f4f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7f4fc | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x7f50c | store | 1 | `sp -0x5e` | `(ret(0x125b8c) & 0xff)` |
| `ldrb` at 0x7f510 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7f514 | load | 1 | `sp -0x5d` | `-` |
| `ldrb` at 0x7f520 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x7f524 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7f53c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7f540 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x7f544 | load | 1 | `sp -0x5d` | `-` |
| `ldr` at 0x7f548 | load | 4 | `abs 0x7f740` | `-` |
| `ldrb` at 0x7f55c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7f560 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x7f564 | load | 1 | `sp -0x5d` | `-` |
| `ldr` at 0x7f568 | load | 4 | `abs 0x7f744` | `-` |
| `ldr` at 0x7f578 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x7f57c | load | 1 | `[sp-104] +0x3` | `-` |
| `ldrb` at 0x7f580 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x7f594 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x7f598 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7f5b0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x7f5b4 | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x7f5c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f5c8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7f5cc | load | 4 | `abs 0x7f748` | `-` |
| `ldr` at 0x7f5dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f5e0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7f5e4 | load | 4 | `abs 0x7f74c` | `-` |
| `ldr` at 0x7f5f4 | load | 4 | `abs 0x7f73c` | `-` |
| `ldr` at 0x7f5f8 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0x7f608 | load | 4 | `abs 0x7f750` | `-` |
| `ldr` at 0x7f614 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x7f618 | load | 1 | `[sp-104] +0x4` | `-` |
| `ldrb` at 0x7f61c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7f630 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x7f634 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x7f638 | load | 4 | `sp -0x14` | `-` |
| `ldc` at 0x7f648 | load | unknown | `abs 0x7f730` | `-` |
| `ldrb` at 0x7f658 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x7f660 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x7f664 | load | 1 | `[sp-104] +0x4` | `-` |
| `str` at 0x7f678 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x7f680 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x7f690 | load | 4 | `abs 0x7f754` | `-` |
| `ldrb` at 0x7f69c | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x7f6a0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7f6b8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x7f6bc | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x7f6cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f6d0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7f6d4 | load | 4 | `abs 0x7f758` | `-` |
| `ldr` at 0x7f6e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7f6e8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7f6ec | load | 4 | `abs 0x7f75c` | `-` |
| `ldr` at 0x7f6fc | load | 4 | `abs 0x7f73c` | `-` |
| `ldr` at 0x7f700 | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0x7f710 | load | 4 | `abs 0x7f760` | `-` |
| `ldr` at 0x7f718 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x7f720 | store | 4 | `sp -0x5c` | `[sp-92]+1` |
| `ldm` at 0x7f72c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7f72c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7f72c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7f400 | `(unknown callee) 0x3ee14` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x7f414 | `(unknown callee) 0x7e970` | `[sp-20]` | `?` | `?` | `ret(0x3ee14)` |
| `bl` at 0x7f4b8 | `(unknown callee) 0x3ee14` | `[sp-20]` | `[sp-100]` | `?` | `?` |
| `bl` at 0x7f504 | `(unknown callee) 0x125b8c` | `[sp-20]` | `[sp-21]:1` | `sp-93` | `[sp-21]:1` |
| `bl` at 0x7f554 | `(unknown callee) 0x29908` | `0x1665fc` | `[sp-22]:1` | `[sp-23]:1` | `[sp-93]:1` |
| `bl` at 0x7f574 | `(unknown callee) 0x29908` | `0x166638` | `[sp-22]:1` | `[sp-23]:1` | `[sp-93]:1` |
| `bl` at 0x7f5a0 | `(unknown callee) 0x7ed48` | `[sp-104]` | `[sp-23]:1` | `[[sp-104]+3]:1` | `[sp-23]:1` |
| `bl` at 0x7f5d4 | `(unknown callee) 0x7f97c` | `[sp-20]` | `[sp-104]` | `0x166674` | `0x0` |
| `bl` at 0x7f5ec | `(unknown callee) 0x7f97c` | `[sp-20]` | `[sp-104]` | `0x1665e8` | `0x1` |
| `bl` at 0x7f60c | `(unknown callee) 0x29908` | `0x166684` | `?` | `?` | `?` |
| `bl` at 0x7f640 | `(unknown callee) 0x586fc` | `[sp-20]` | `[[sp-104]+1]:1` | `[[sp-104]+4]:1` | `[[sp-104]+1]:1` |
| `bl` at 0x7f694 | `(unknown callee) 0xa9740` | `0x0` | `0x3500` | `0x1a4` | `0x1666b4` |
| `bl` at 0x7f6a8 | `(unknown callee) 0x7ecc0` | `[sp-104]` | `[sp-23]:1` | `?` | `[sp-23]:1` |
| `bl` at 0x7f6dc | `(unknown callee) 0x7f97c` | `[sp-20]` | `[sp-104]` | `0x1666cc` | `0x0` |
| `bl` at 0x7f6f4 | `(unknown callee) 0x7f97c` | `[sp-20]` | `[sp-104]` | `0x1665f4` | `0x0` |
| `bl` at 0x7f714 | `(unknown callee) 0x29908` | `0x1666dc` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x7f40c | `ret(0x3ee14) > 0x0 (signed, 32-bit)` | `ret(0x3ee14) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x7f428 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7f454 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7f498 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bgt` at 0x7f4c4 | `ret(0x3ee14) > 0x0 (signed, 32-bit)` | `ret(0x3ee14) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x7f4dc | `[[sp-104]]:1 != [sp-21]:1` | `[[sp-104]]:1 == [sp-21]:1` |
| `beq` at 0x7f4f0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f51c | `[sp-22]:1 == [sp-93]:1` | `[sp-22]:1 != [sp-93]:1` |
| `bne` at 0x7f534 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7f590 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7f5ac | `ret(0x7ed48) != 0x0` | `ret(0x7ed48) == 0x0` |
| `beq` at 0x7f5c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f604 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f62c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f650 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x7f670 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7f6b4 | `ret(0x7ecc0) != 0x0` | `ret(0x7ecc0) == 0x0` |
| `beq` at 0x7f6c8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f70c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7f72c | `r0` | `?` |

## `vrm_alert`

- `vrm_alert`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7f764 to 0x7f97c.
  Established. Two paths store 0x166708 or 0x16670c at sp-0x34 (`str` at 0x7f798, `str` at 0x7f7a4).
  A byte loaded from offset two of entry r1 sends execution to 0x7f8a0 when it is zero (`ldrb` at 0x7f7b4, `beq` at 0x7f7bc).
  On the fall-through path, a separate load from that address sends execution to 0x7f8a0 when its value is 0xff (`ldrb` at 0x7f7c4, `beq` at 0x7f7cc).
  Calls to unnamed 0x22070 have candidate values one, 0x1664bc, 0x1fa or 0x1fe, and zero (`bl` at 0x7f898, `bl` at 0x7f954).
  The record does not establish runtime output or physical alert delivery.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7f774 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7f778 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x7f77c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x7f780 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f784 | load | 1 | `r1 +0x5` | `-` |
| `ldr` at 0x7f794 | load | 4 | `abs 0x7f960` | `-` |
| `str` at 0x7f798 | store | 4 | `sp -0x34` | `0x166708` |
| `ldr` at 0x7f7a0 | load | 4 | `abs 0x7f964` | `-` |
| `str` at 0x7f7a4 | store | 4 | `sp -0x34` | `0x16670c` |
| `ldr` at 0x7f7a8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7f7ac | store | 4 | `sp -0x24` | `[sp-52]` |
| `ldr` at 0x7f7b0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f7b4 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0x7f7c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f7c4 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0x7f7d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f7e4 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x7f7e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7f7f8 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x7f7fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7f824 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x7f828 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7f82c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7f830 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7f83c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f840 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x7f844 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f854 | load | 4 | `abs 0x7f968` | `-` |
| `str` at 0x7f858 | store | 4 | `(unknown)` | `0x166710` |
| `ldr` at 0x7f85c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7f860 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x7f864 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7f868 | store | 4 | `(unknown)` | `[sp-32]` |
| `str` at 0x7f86c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x7f870 | store | 4 | `(unknown)` | `ret(0x58280)` |
| `ldr` at 0x7f874 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7f878 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x7f87c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f880 | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0x7f884 | store | 4 | `(unknown)` | `[[sp-28]+2]:1` |
| `ldr` at 0x7f88c | load | 4 | `abs 0x7f96c` | `-` |
| `ldr` at 0x7f890 | load | 4 | `abs 0x7f970` | `-` |
| `ldr` at 0x7f8a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7f8b4 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x7f8b8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7f8c8 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x7f8cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7f8f4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7f8f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7f8fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7f900 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7f90c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f910 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x7f914 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7f924 | load | 4 | `abs 0x7f974` | `-` |
| `str` at 0x7f928 | store | 4 | `(unknown)` | `0x166724` |
| `ldr` at 0x7f92c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7f930 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0x7f934 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7f938 | store | 4 | `(unknown)` | `[sp-32]` |
| `str` at 0x7f93c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x7f940 | store | 4 | `(unknown)` | `ret(0x58280)` |
| `ldr` at 0x7f948 | load | 4 | `abs 0x7f96c` | `-` |
| `ldr` at 0x7f94c | load | 4 | `abs 0x7f978` | `-` |
| `ldm` at 0x7f95c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7f95c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7f95c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7f95c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7f7e0 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7f7ec | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7f834 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x7f84c | `(unknown callee) 0x58280` | `[sp-24]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0x7f898 | `(unknown callee) 0x22070` | `0x1` | `0x1664bc` | `0x1fa` | `0x0` |
| `bl` at 0x7f8b0 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7f8bc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7f904 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-48]` | `[sp-44]` | `?` |
| `bl` at 0x7f91c | `(unknown callee) 0x58280` | `[sp-24]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0x7f954 | `(unknown callee) 0x22070` | `0x1` | `0x1664bc` | `0x1fe` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7f790 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f7bc | `[r1+2]:1 == 0x0` | `[r1+2]:1 != 0x0` |
| `beq` at 0x7f7cc | `[r1+2]:1 == 0xff` | `[r1+2]:1 != 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7f95c | `r0` | `ret(0x22070)` |

## `vrm_event`

- `vrm_event`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7f97c to 0x7fbc8.
  Established. Nonzero entry r3 selects stores of 0x3401 and 0x1a2 in sp-0x6c and sp-0x70 (`beq` at 0x7f9d4, `str` at 0x7f9dc, `str` at 0x7f9e4).
  Zero selects stores of 0x3402 and 0x1a3 in those fields (`beq` at 0x7f9d4, `str` at 0x7f9f0, `str` at 0x7f9f8).
  The final call to unnamed 0xa9740 has candidate values zero, the words at sp-0x6c and sp-0x70, and sp-0x64 (`bl` at 0x7fb9c).
  The recorded exit carries that call result, without establishing an event destination or its hardware meaning (`bl` at 0x7fb9c, `ldm` at 0x7fba4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7f98c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7f990 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x7f994 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x7f998 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x7f99c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7f9a0 | load | 1 | `r1 +0x5` | `-` |
| `ldr` at 0x7f9b0 | load | 4 | `abs 0x7fba8` | `-` |
| `str` at 0x7f9b4 | store | 4 | `sp -0x80` | `0x166708` |
| `ldr` at 0x7f9bc | load | 4 | `abs 0x7fbac` | `-` |
| `str` at 0x7f9c0 | store | 4 | `sp -0x80` | `0x16670c` |
| `ldr` at 0x7f9c4 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x7f9c8 | store | 4 | `sp -0x68` | `[sp-128]` |
| `ldr` at 0x7f9cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7f9d8 | load | 4 | `abs 0x7fbb0` | `-` |
| `str` at 0x7f9dc | store | 4 | `sp -0x6c` | `0x3401` |
| `ldr` at 0x7f9e0 | load | 4 | `abs 0x7fbb4` | `-` |
| `str` at 0x7f9e4 | store | 4 | `sp -0x70` | `0x1a2` |
| `ldr` at 0x7f9ec | load | 4 | `abs 0x7fbb8` | `-` |
| `str` at 0x7f9f0 | store | 4 | `sp -0x6c` | `0x3402` |
| `ldr` at 0x7f9f4 | load | 4 | `abs 0x7fbbc` | `-` |
| `str` at 0x7f9f8 | store | 4 | `sp -0x70` | `0x1a3` |
| `ldr` at 0x7f9fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7fa00 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0x7fa0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7fa10 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0x7fa1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7fa30 | store | 4 | `sp -0x74` | `ret(0x6e1b0)` |
| `ldr` at 0x7fa34 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x7fa44 | store | 4 | `sp -0x78` | `ret(0x2031c)+1` |
| `ldr` at 0x7fa48 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x7fa70 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x7fa74 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x7fa78 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x7fa7c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x7fa88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7fa8c | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x7fa90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7faa4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x7faa8 | store | 4 | `(unknown)` | `ret(0x58280)` |
| `ldr` at 0x7faac | load | 4 | `sp -0x68` | `-` |
| `str` at 0x7fab0 | store | 4 | `(unknown)` | `[sp-104]` |
| `ldr` at 0x7fab4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7fab8 | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0x7fabc | store | 4 | `(unknown)` | `[[sp-28]+2]:1` |
| `ldr` at 0x7fac4 | load | 4 | `abs 0x7fbc0` | `-` |
| `ldr` at 0x7fac8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7facc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7fad8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7faec | store | 4 | `sp -0x7c` | `ret(0x6e1b0)` |
| `ldr` at 0x7faf0 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x7fb00 | store | 4 | `sp -0x78` | `ret(0x2031c)+1` |
| `ldr` at 0x7fb04 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x7fb2c | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x7fb30 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x7fb34 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x7fb38 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x7fb44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7fb48 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x7fb4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7fb60 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x7fb64 | store | 4 | `(unknown)` | `ret(0x58280)` |
| `ldr` at 0x7fb6c | load | 4 | `abs 0x7fbc4` | `-` |
| `ldr` at 0x7fb70 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x7fb74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7fb84 | store | 4 | `(unknown)` | `0x1` |
| `str` at 0x7fb8c | store | 4 | `(unknown)` | `0x10` |
| `ldr` at 0x7fb94 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x7fb98 | load | 4 | `sp -0x70` | `-` |
| `ldm` at 0x7fba4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7fba4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7fba4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7fba4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7fa2c | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7fa38 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7fa80 | `(unknown callee) 0x203f4` | `[sp-124]` | `[sp-116]` | `[sp-120]` | `?` |
| `bl` at 0x7fa98 | `(unknown callee) 0x58280` | `[sp-24]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0x7fad0 | `(unknown callee) 0x205bc` | `sp-100` | `0x166710` | `[sp-104]` | `[sp-32]` |
| `bl` at 0x7fae8 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7faf4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7fb3c | `(unknown callee) 0x203f4` | `[sp-116]` | `[sp-124]` | `[sp-120]` | `?` |
| `bl` at 0x7fb54 | `(unknown callee) 0x58280` | `[sp-24]` | `[[sp-28]+1]:1` | `?` | `[[sp-28]+1]:1` |
| `bl` at 0x7fb78 | `(unknown callee) 0x205bc` | `sp-100` | `0x166724` | `[sp-104]` | `[sp-32]` |
| `bl` at 0x7fb9c | `(unknown callee) 0xa9740` | `0x0` | `[sp-108]` | `[sp-112]` | `sp-100` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7f9ac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7f9d4 | `r3 == 0x0` | `r3 != 0x0` |
| `beq` at 0x7fa08 | `[r1+2]:1 == 0x0` | `[r1+2]:1 != 0x0` |
| `beq` at 0x7fa18 | `[r1+2]:1 == 0xff` | `[r1+2]:1 != 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7fba4 | `r0` | `ret(0xa9740)` |

## `autofanInit`

- `autofanInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35d98 to 0x361f0.
  Established. Word stores put 0x64 and 0x3c at absolute addresses 0x1a5a20 and 0x1a5a1c on one path (`str` at 0x360f4, `str` at 0x36100).
  The alternative store to 0x1a5a20 has an unknown value, while the following store to 0x1a5a1c receives a result of unnamed 0x14fe64 (`str` at 0x36128, `bl` at 0x36138, `str` at 0x36140).
  Two word stores with unknown effective addresses carry 0xff (`str` at 0x36094, `str` at 0x360c4).
  These values do not establish time units, fan speeds or a control law.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x35da8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x35dac | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x35db0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x35db4 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x35db8 | load | 4 | `abs 0x361c8` | `-` |
| `ldr` at 0x35dbc | load | 4 | `abs 0x1a59f4` | `-` |
| `ldr` at 0x35dc8 | load | 4 | `abs 0x361cc` | `-` |
| `str` at 0x35dcc | store | 4 | `sp -0x44` | `0x159674` |
| `ldr` at 0x35dd0 | load | 4 | `abs 0x361c8` | `-` |
| `ldr` at 0x35dd4 | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x35dd8 | store | 4 | `sp -0x40` | `[0x1a59f4]` |
| `ldr` at 0x35ddc | load | 4 | `abs 0x361d0` | `-` |
| `ldr` at 0x35de0 | load | 4 | `abs 0x1a59f8` | `-` |
| `str` at 0x35de4 | store | 4 | `sp -0x3c` | `[0x1a59f8]` |
| `ldr` at 0x35dec | load | 4 | `abs 0x361d4` | `-` |
| `ldr` at 0x35dfc | load | 4 | `abs 0x361d0` | `-` |
| `ldr` at 0x35e00 | load | 4 | `abs 0x1a59f8` | `-` |
| `ldr` at 0x35e14 | load | 4 | `abs 0x361d0` | `-` |
| `str` at 0x35e18 | store | 4 | `abs 0x1a59f8` | `ret(0x6e140)` |
| `ldr` at 0x35e1c | load | 4 | `abs 0x361d0` | `-` |
| `ldr` at 0x35e20 | load | 4 | `abs 0x1a59f8` | `-` |
| `str` at 0x35e28 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x35e2c | load | 4 | `abs 0x361c8` | `-` |
| `ldr` at 0x35e30 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x35e40 | store | 4 | `abs 0x1a59f4` | `ret(0x204d8)` |
| `ldr` at 0x35e44 | load | 4 | `abs 0x1a59f4` | `-` |
| `ldr` at 0x35e50 | load | 4 | `abs 0x361d8` | `-` |
| `str` at 0x35e54 | store | 4 | `sp -0x44` | `0x1596c0` |
| `ldr` at 0x35e58 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x35e5c | store | 4 | `sp -0x40` | `[sp-48]` |
| `ldr` at 0x35e64 | load | 4 | `abs 0x361d4` | `-` |
| `ldr` at 0x35e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x35e7c | load | 4 | `abs 0x361c8` | `-` |
| `ldr` at 0x35e80 | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x35e88 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x35e8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35e90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x35e94 | store | 4 | `[sp-44] +0x4` | `?` |
| `ldr` at 0x35e98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35e9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x35ea0 | store | 4 | `[sp-44] +0x0` | `[sp-32]` |
| `ldr` at 0x35ea4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35ea8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35eac | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x35ebc | store | 4 | `[sp-44] +0x8` | `ret(0x204d8)` |
| `ldr` at 0x35ec0 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x35ecc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x35ee0 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x35ee4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x35ef4 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x35ef8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x35f20 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x35f24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x35f28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x35f2c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x35f38 | load | 4 | `abs 0x361dc` | `-` |
| `str` at 0x35f3c | store | 4 | `(unknown)` | `0x1596fc` |
| `str` at 0x35f40 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x35f44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f48 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x35f5c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x35f64 | load | 4 | `abs 0x361d4` | `-` |
| `ldr` at 0x35f74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x35f7c | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x35f8c | store | 4 | `[sp-44] +0xc` | `ret(0x204d8)` |
| `ldr` at 0x35f90 | load | 4 | `[sp-44] +0xc` | `-` |
| `ldr` at 0x35f9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x35fb0 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x35fb4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x35fc4 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x35fc8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x35ff0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x35ff4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x35ff8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x35ffc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36008 | load | 4 | `abs 0x361e0` | `-` |
| `str` at 0x3600c | store | 4 | `(unknown)` | `0x159734` |
| `str` at 0x36010 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x36014 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36018 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x36034 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3603c | load | 4 | `abs 0x361d4` | `-` |
| `str` at 0x36050 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x36054 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36058 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x3605c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36068 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3606c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36088 | load | 4 | `[sp-44] +0xc` | `-` |
| `str` at 0x36094 | store | 4 | `(unknown)` | `0xff` |
| `ldr` at 0x36098 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3609c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x360b8 | load | 4 | `[sp-44] +0xc` | `-` |
| `str` at 0x360c4 | store | 4 | `(unknown)` | `0xff` |
| `ldr` at 0x360c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x360d0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x360d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x360e4 | load | 4 | `abs 0x361e4` | `-` |
| `ldr` at 0x360ec | load | 4 | `abs 0x361e8` | `-` |
| `str` at 0x360f4 | store | 4 | `abs 0x1a5a20` | `0x64` |
| `ldr` at 0x360f8 | load | 4 | `abs 0x361ec` | `-` |
| `str` at 0x36100 | store | 4 | `abs 0x1a5a1c` | `0x3c` |
| `ldr` at 0x36108 | load | 4 | `abs 0x361e8` | `-` |
| `ldr` at 0x3610c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36128 | store | 4 | `abs 0x1a5a20` | `?` |
| `ldr` at 0x3612c | load | 4 | `abs 0x361ec` | `-` |
| `ldr` at 0x36134 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x36140 | store | 4 | `abs 0x1a5a1c` | `ret(0x14fe64)` |
| `str` at 0x36148 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3614c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36150 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x36154 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36160 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36164 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36178 | load | 4 | `[sp-44] +0x8` | `-` |
| `strb` at 0x36184 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x36188 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3618c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x361a0 | load | 4 | `[sp-44] +0x8` | `-` |
| `strb` at 0x361ac | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x361b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x361b8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0x361c4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x361c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x361c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x361c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x35df8 | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0x9c` | `0x0` |
| `bl` at 0x35e0c | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a59f8]` |
| `bl` at 0x35e38 | `(unknown callee) 0x204d8` | `[sp-48]` | `0x1` | `ret(0x6e140)` | `?` |
| `bl` at 0x35e70 | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0xa7` | `0x0` |
| `bl` at 0x35eb4 | `(unknown callee) 0x204d8` | `[[sp-44]]` | `0x18` | `[sp-44]` | `[sp-44]` |
| `bl` at 0x35edc | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x35ee8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x35f30 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x35f70 | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0xb1` | `0x0` |
| `bl` at 0x35f84 | `(unknown callee) 0x204d8` | `[[sp-44]]` | `0x10c` | `?` | `[sp-44]` |
| `bl` at 0x35fac | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x35fb8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x36000 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x36048 | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0xb5` | `0x0` |
| `bl` at 0x360e8 | `(unknown callee) 0x293f8` | `0x15976c` | `?` | `[[sp-44]]` | `[sp-36]` |
| `bl` at 0x36138 | `(unknown callee) 0x14fe64` | `0x3c` | `[sp-36]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x35dc4 | `[0x1a59f4] == 0x0` | `[0x1a59f4] != 0x0` |
| `bne` at 0x35e08 | `[0x1a59f8] != 0x0` | `[0x1a59f8] == 0x0` |
| `bne` at 0x35e4c | `[0x1a59f4] != 0x0` | `[0x1a59f4] == 0x0` |
| `bne` at 0x35ec8 | `[[sp-44]+8] != 0x0` | `[[sp-44]+8] == 0x0` |
| `bne` at 0x35f98 | `[[sp-44]+12] != 0x0` | `[[sp-44]+12] == 0x0` |
| `ble` at 0x36064 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bgt` at 0x360e0 | `[sp-36] > 0x0 (signed, 32-bit)` | `[sp-36] <= 0x0 (signed, 32-bit)` |
| `ble` at 0x3615c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x361c4 | `r0` | `?` |

## `autofanSubZoneInit`

- `autofanSubZoneInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x361f0 to 0x3650c.
  Established. Word stores put results of unnamed 0x6e140 and 0x204d8 at absolute addresses 0x1a5a00 and 0x1a59fc (`bl` at 0x36274, `str` at 0x36280, `bl` at 0x362a0, `str` at 0x362a8).
  Calls to unnamed 0x204d8 have candidate values headed by the word at the pointer from sp-0x28, followed by four or 0x108 respectively (`bl` at 0x3631c, `bl` at 0x363e0).
  Results of those calls are stored at offsets eight and twelve of that pointer (`bl` at 0x3631c, `str` at 0x36324, `bl` at 0x363e0, `str` at 0x363e8).
  A later byte store has unknown target, so the physical subzone mapping remains unknown (`strb` at 0x364d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36200 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x36204 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x36208 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x3620c | load | 4 | `abs 0x364f0` | `-` |
| `ldr` at 0x36210 | load | 4 | `abs 0x1a59fc` | `-` |
| `ldr` at 0x36224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36230 | load | 4 | `abs 0x364f4` | `-` |
| `str` at 0x36234 | store | 4 | `sp -0x40` | `0x15979c` |
| `ldr` at 0x36238 | load | 4 | `abs 0x364f0` | `-` |
| `ldr` at 0x3623c | load | 4 | `abs 0x1a59fc` | `-` |
| `str` at 0x36240 | store | 4 | `sp -0x3c` | `[0x1a59fc]` |
| `ldr` at 0x36244 | load | 4 | `abs 0x364f8` | `-` |
| `ldr` at 0x36248 | load | 4 | `abs 0x1a5a00` | `-` |
| `str` at 0x3624c | store | 4 | `sp -0x38` | `[0x1a5a00]` |
| `ldr` at 0x36254 | load | 4 | `abs 0x364fc` | `-` |
| `ldr` at 0x36264 | load | 4 | `abs 0x364f8` | `-` |
| `ldr` at 0x36268 | load | 4 | `abs 0x1a5a00` | `-` |
| `ldr` at 0x3627c | load | 4 | `abs 0x364f8` | `-` |
| `str` at 0x36280 | store | 4 | `abs 0x1a5a00` | `ret(0x6e140)` |
| `ldr` at 0x36284 | load | 4 | `abs 0x364f8` | `-` |
| `ldr` at 0x36288 | load | 4 | `abs 0x1a5a00` | `-` |
| `str` at 0x36290 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x36294 | load | 4 | `abs 0x364f0` | `-` |
| `ldr` at 0x36298 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x362a8 | store | 4 | `abs 0x1a59fc` | `ret(0x204d8)` |
| `ldr` at 0x362ac | load | 4 | `abs 0x1a59fc` | `-` |
| `ldr` at 0x362b8 | load | 4 | `abs 0x36500` | `-` |
| `str` at 0x362bc | store | 4 | `sp -0x40` | `0x1597d8` |
| `ldr` at 0x362c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x362c4 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x362cc | load | 4 | `abs 0x364fc` | `-` |
| `ldr` at 0x362dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x362e4 | load | 4 | `abs 0x364f0` | `-` |
| `ldr` at 0x362e8 | load | 4 | `abs 0x1a59fc` | `-` |
| `str` at 0x362f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x362f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x362f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x362fc | store | 4 | `[sp-40] +0x4` | `?` |
| `ldr` at 0x36300 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36304 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x36308 | store | 4 | `[sp-40] +0x0` | `[sp-32]` |
| `ldr` at 0x3630c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36310 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36314 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x36324 | store | 4 | `[sp-40] +0x8` | `ret(0x204d8)` |
| `ldr` at 0x36328 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x36334 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36348 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x3634c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3635c | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x36360 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x36388 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3638c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36390 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36394 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x363a0 | load | 4 | `abs 0x36504` | `-` |
| `str` at 0x363a4 | store | 4 | `(unknown)` | `0x15981c` |
| `str` at 0x363a8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x363ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x363b0 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x363b8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x363c0 | load | 4 | `abs 0x364fc` | `-` |
| `ldr` at 0x363d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x363d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x363d8 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x363e8 | store | 4 | `[sp-40] +0xc` | `ret(0x204d8)` |
| `ldr` at 0x363ec | load | 4 | `[sp-40] +0xc` | `-` |
| `ldr` at 0x363f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3640c | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x36410 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x36420 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x36424 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3644c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x36450 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x36454 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36458 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x36464 | load | 4 | `abs 0x36508` | `-` |
| `str` at 0x36468 | store | 4 | `(unknown)` | `0x15985c` |
| `str` at 0x3646c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x36470 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36474 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x36488 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x36490 | load | 4 | `abs 0x364fc` | `-` |
| `str` at 0x364a4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x364a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x364ac | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x364b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x364bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x364c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x364c8 | load | 4 | `[sp-40] +0x8` | `-` |
| `strb` at 0x364d4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x364d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x364e0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0x364ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x364ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x364ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x364ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3621c | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x1a59fc]` |
| `bl` at 0x36260 | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0xdd` | `0x0` |
| `bl` at 0x36274 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a5a00]` |
| `bl` at 0x362a0 | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `ret(0x6e140)` | `?` |
| `bl` at 0x362d8 | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0xe8` | `0x0` |
| `bl` at 0x3631c | `(unknown callee) 0x204d8` | `[[sp-40]]` | `0x4` | `[sp-40]` | `[sp-40]` |
| `bl` at 0x36344 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x36350 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x36398 | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x363cc | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0xf2` | `0x0` |
| `bl` at 0x363e0 | `(unknown callee) 0x204d8` | `[[sp-40]]` | `0x108` | `?` | `[sp-40]` |
| `bl` at 0x36408 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x36414 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x3645c | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-52]` | `[sp-48]` | `?` |
| `bl` at 0x3649c | `(unknown callee) 0x22070` | `0x9` | `0x1596a8` | `0xf5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36218 | `[0x1a59fc] == 0x0` | `[0x1a59fc] != 0x0` |
| `bne` at 0x3622c | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x36270 | `[0x1a5a00] != 0x0` | `[0x1a5a00] == 0x0` |
| `bne` at 0x362b4 | `[0x1a59fc] != 0x0` | `[0x1a59fc] == 0x0` |
| `bne` at 0x36330 | `[[sp-40]+8] != 0x0` | `[[sp-40]+8] == 0x0` |
| `bne` at 0x363f4 | `[[sp-40]+12] != 0x0` | `[[sp-40]+12] == 0x0` |
| `ble` at 0x364b8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x364ec | `r0` | `?` |

## `autofanSubZoneRemove`

- `autofanSubZoneRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x365d4 to 0x3669c.
  Established. When the word at absolute 0x1a5a00 is zero, execution branches to 0x3668c (`ldr` at 0x365ec, `beq` at 0x365f4).
  Word stores through the pointer loaded from sp-0x18 clear offsets zero, four, eight and twelve along the continuing path (`str` at 0x3664c, `str` at 0x36658, `str` at 0x36670, `str` at 0x36688).
  Two calls to unnamed 0x20514 have first candidate values the object's words at offsets eight and twelve respectively (`bl` at 0x36664, `bl` at 0x3667c).
  The software clears do not establish fan-zone removal from hardware.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x365e4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x365e8 | load | 4 | `abs 0x36694` | `-` |
| `ldr` at 0x365ec | load | 4 | `abs 0x1a5a00` | `-` |
| `ldr` at 0x365f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36600 | load | 4 | `abs 0x36698` | `-` |
| `ldr` at 0x36604 | load | 4 | `abs 0x1a59fc` | `-` |
| `ldr` at 0x3660c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3661c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36624 | load | 4 | `abs 0x36698` | `-` |
| `ldr` at 0x36628 | load | 4 | `abs 0x1a59fc` | `-` |
| `str` at 0x36630 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x36634 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36638 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x36644 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3664c | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x36650 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36658 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x3665c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36660 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x36668 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36670 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x36674 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36678 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x36680 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36688 | store | 4 | `[sp-24] +0xc` | `0x0` |
| `ldm` at 0x36690 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x36690 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x36690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36664 | `(unknown callee) 0x20514` | `[[sp-24]+8]` | `r1` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x3667c | `(unknown callee) 0x20514` | `[[sp-24]+12]` | `?` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x365f4 | `[0x1a5a00] == 0x0` | `[0x1a5a00] != 0x0` |
| `bne` at 0x36614 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x36640 | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x36690 | `r0` | `?` |

## `autofanRemove`

- `autofanRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3650c to 0x365d4.
  Established. A word load reads absolute 0x1a59f8, with a zero-value branch to 0x365c4 (`ldr` at 0x36524, `beq` at 0x3652c).
  Word stores through the pointer loaded from sp-0x18 clear offsets zero and four before the first call to unnamed 0x20514 (`str` at 0x36584, `str` at 0x36590, `bl` at 0x3659c).
  That call has candidate values the object's word at offset eight, entry r1 and the object pointer twice (`bl` at 0x3659c).
  Later stores clear offsets eight and twelve, without establishing a physical fan-disable transaction (`str` at 0x365a8, `str` at 0x365c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3651c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x36520 | load | 4 | `abs 0x365cc` | `-` |
| `ldr` at 0x36524 | load | 4 | `abs 0x1a59f8` | `-` |
| `ldr` at 0x36530 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36538 | load | 4 | `abs 0x365d0` | `-` |
| `ldr` at 0x3653c | load | 4 | `abs 0x1a59f4` | `-` |
| `ldr` at 0x36544 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3655c | load | 4 | `abs 0x365d0` | `-` |
| `ldr` at 0x36560 | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x36568 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3656c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36570 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x3657c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36584 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x36588 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36590 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x36594 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x36598 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x365a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x365a8 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x365ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x365b0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x365b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x365c0 | store | 4 | `[sp-24] +0xc` | `0x0` |
| `ldm` at 0x365c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x365c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x365c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3659c | `(unknown callee) 0x20514` | `[[sp-24]+8]` | `r1` | `[sp-24]` | `[sp-24]` |
| `bl` at 0x365b4 | `(unknown callee) 0x20514` | `[[sp-24]+12]` | `?` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3652c | `[0x1a59f8] == 0x0` | `[0x1a59f8] != 0x0` |
| `bne` at 0x3654c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x36578 | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x365c8 | `r0` | `?` |

## `autofanMonitor`

- `autofanMonitor`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3669c to 0x36f7c.
  Established. A byte store writes one to absolute 0x31a4dc, and a word store decrements the value loaded from absolute 0x19e664 (`strb` at 0x36ba8, `str` at 0x36cbc).
  Word stores through pointers loaded from sp-0x18 and sp-0x1c receive values loaded from sp-0x68 and sp-0x34 respectively (`str` at 0x369dc, `str` at 0x369e8).
  Coprocessor loads from sp-0x98 and sp-0x88 have unknown widths and results (`ldc` at 0x369f0, `ldc` at 0x36e98).
  The numeric stack states and output stores do not establish temperature units or an automatic-fan control law.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x366ac | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x366b0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x366b4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x366b8 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x366c0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x366c8 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x366d0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x366d4 | load | 4 | `abs 0x36f50` | `-` |
| `ldrb` at 0x366d8 | load | 1 | `abs 0x1a5c8c` | `-` |
| `str` at 0x366e8 | store | 4 | `sp -0xb0` | `0x0` |
| `str` at 0x366f4 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x36700 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x3670c | load | 4 | `abs 0x36f54` | `-` |
| `ldr` at 0x36710 | load | 4 | `abs 0x1a59f8` | `-` |
| `ldr` at 0x3671c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36724 | load | 4 | `abs 0x36f58` | `-` |
| `ldr` at 0x36728 | load | 4 | `abs 0x1a59f4` | `-` |
| `ldr` at 0x36730 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36740 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36748 | load | 4 | `abs 0x36f58` | `-` |
| `ldr` at 0x3674c | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x36754 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0x3675c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x36760 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36764 | load | 4 | `[sp-108] +0x0` | `-` |
| `ldr` at 0x36768 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x36778 | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x3677c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36780 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3679c | load | 4 | `[sp-108] +0x4` | `-` |
| `ldr` at 0x367a4 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x367a8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x367c4 | load | 4 | `[sp-108] +0xc` | `-` |
| `str` at 0x367d4 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x367d8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x367e8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x367f0 | load | 4 | `abs 0x36f5c` | `-` |
| `ldr` at 0x367f4 | load | 4 | `abs 0x1a59fc` | `-` |
| `str` at 0x367fc | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x36800 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36808 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36820 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36824 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36840 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldrb` at 0x36848 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x3684c | store | 1 | `sp -0x52` | `?` |
| `strb` at 0x36858 | store | 1 | `sp -0x52` | `0x0` |
| `str` at 0x36860 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x36864 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36868 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36884 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldrb` at 0x3688c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x36890 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3689c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x368a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x368bc | load | 4 | `[sp-108] +0x4` | `-` |
| `ldr` at 0x368c8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x368d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x368d8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x368f0 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x368f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36910 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldr` at 0x3691c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x36928 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x36930 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x3693c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x36944 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x3694c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36950 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3696c | load | 4 | `[sp-108] +0x4` | `-` |
| `ldr` at 0x36974 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36978 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36994 | load | 4 | `[sp-108] +0xc` | `-` |
| `ldrb` at 0x3699c | load | 1 | `sp -0x52` | `-` |
| `str` at 0x369a0 | store | 4 | `(unknown)` | `?` |
| `str` at 0x369a8 | store | 4 | `(unknown)` | `sp-160` |
| `ldr` at 0x369ac | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x369b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x369bc | load | 4 | `sp -0x94` | `-` |
| `str` at 0x369c0 | store | 4 | `sp -0x50` | `[sp-148]` |
| `ldr` at 0x369c4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x369c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x369d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x369d8 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x369dc | store | 4 | `[sp-24] +0x0` | `[sp-104]` |
| `ldr` at 0x369e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x369e4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x369e8 | store | 4 | `[sp-28] +0x0` | `[sp-52]` |
| `ldr` at 0x369ec | load | 4 | `sp -0x20` | `-` |
| `ldc` at 0x369f0 | load | unknown | `sp -0x98` | `-` |
| `str` at 0x369f8 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0x369fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x36a00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36a08 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x36a10 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36a24 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x36a28 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x36a2c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x36a30 | store | 4 | `[sp] +0x0` | `[sp-160]` |
| `ldr` at 0x36a34 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x36a38 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x36a3c | store | 4 | `[sp+4] +0x0` | `[sp-156]` |
| `ldr` at 0x36a40 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x36a44 | store | 4 | `sp -0x30` | `[sp-144]` |
| `ldr` at 0x36a48 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x36a4c | store | 4 | `sp -0x4c` | `[sp-144]` |
| `ldr` at 0x36a50 | load | 4 | `abs 0x36f60` | `-` |
| `ldr` at 0x36a54 | load | 4 | `abs 0x19e664` | `-` |
| `str` at 0x36a64 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x36a68 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36a6c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36a88 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldrb` at 0x36a90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x36a94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36aa0 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36aa4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36ac0 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldr` at 0x36acc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x36ad8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x36adc | store | 1 | `sp -0x5d` | `?` |
| `ldrb` at 0x36ae0 | load | 1 | `sp -0x5d` | `-` |
| `ldr` at 0x36ae8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36af4 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36af8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36b14 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldr` at 0x36b1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36b20 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x36b2c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x36b38 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36b3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36b50 | load | 4 | `[sp-108] +0x8` | `-` |
| `ldr` at 0x36b58 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x36b60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x36b6c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36b70 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36b84 | load | 4 | `[sp-108] +0x8` | `-` |
| `ldr` at 0x36b8c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x36b98 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x36ba0 | load | 4 | `abs 0x36f64` | `-` |
| `strb` at 0x36ba8 | store | 1 | `abs 0x31a4dc` | `0x1` |
| `ldr` at 0x36bac | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36bc0 | store | 4 | `sp -0xa4` | `ret(0x6e1b0)` |
| `ldr` at 0x36bc4 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x36bd4 | store | 4 | `sp -0xa8` | `ret(0x2031c)+1` |
| `ldr` at 0x36bd8 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x36c00 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x36c04 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x36c08 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x36c0c | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x36c18 | load | 1 | `sp -0x5d` | `-` |
| `ldr` at 0x36c1c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36c2c | load | 4 | `abs 0x36f68` | `-` |
| `str` at 0x36c30 | store | 4 | `(unknown)` | `0x15989c` |
| `str` at 0x36c34 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x36c38 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x36c3c | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0x36c40 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x36c44 | store | 4 | `(unknown)` | `[sp-40]` |
| `str` at 0x36c48 | store | 4 | `(unknown)` | `ret(0x64414)` |
| `ldr` at 0x36c4c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x36c50 | store | 4 | `(unknown)` | `[sp-100]` |
| `ldr` at 0x36c58 | load | 4 | `abs 0x36f6c` | `-` |
| `ldr` at 0x36c5c | load | 4 | `abs 0x36f70` | `-` |
| `ldr` at 0x36c6c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36c70 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36c84 | load | 4 | `[sp-108] +0x8` | `-` |
| `ldr` at 0x36c8c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x36c98 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x36c9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x36ca4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x36cac | load | 4 | `abs 0x36f60` | `-` |
| `ldr` at 0x36cb0 | load | 4 | `abs 0x36f60` | `-` |
| `ldr` at 0x36cb4 | load | 4 | `abs 0x19e664` | `-` |
| `str` at 0x36cbc | store | 4 | `abs 0x19e664` | `[0x19e664]-1` |
| `ldr` at 0x36cc0 | load | 4 | `abs 0x36f64` | `-` |
| `ldrb` at 0x36cc4 | load | 1 | `abs 0x31a4dc` | `-` |
| `ldr` at 0x36cd0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x36ce0 | store | 4 | `sp -0x30` | `0x20` |
| `str` at 0x36ce8 | store | 4 | `sp -0x4c` | `0x20` |
| `ldr` at 0x36cec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36cf4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36d04 | store | 4 | `sp -0x50` | `ret(0x38aac)` |
| `ldrb` at 0x36d08 | load | 1 | `sp -0x52` | `-` |
| `ldr` at 0x36d0c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x36d18 | load | 4 | `abs 0x36f74` | `-` |
| `ldr` at 0x36d1c | load | 4 | `abs 0x1a6fc4` | `-` |
| `ldr` at 0x36d2c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36d30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36d4c | load | 4 | `[sp-108] +0x4` | `-` |
| `ldrb` at 0x36d54 | load | 1 | `sp -0x52` | `-` |
| `ldr` at 0x36d58 | load | 4 | `abs 0x36f78` | `-` |
| `ldr` at 0x36d5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x36d60 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x36d6c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x36d70 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36d74 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36d90 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldrb` at 0x36d98 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x36d9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x36da8 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36dac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36dc8 | load | 4 | `[sp-108] +0x4` | `-` |
| `ldr` at 0x36dd4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x36de0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x36de4 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x36dec | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36dfc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x36e04 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x36e0c | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36e10 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36e24 | load | 4 | `[sp-108] +0x8` | `-` |
| `ldrb` at 0x36e2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x36e38 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36e40 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x36e48 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36e64 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36e6c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x36e74 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x36e90 | load | 4 | `sp -0x8c` | `-` |
| `ldc` at 0x36e98 | load | unknown | `sp -0x88` | `-` |
| `ldr` at 0x36ea8 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36eac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36ec0 | load | 4 | `[sp-108] +0x8` | `-` |
| `strb` at 0x36ecc | store | 1 | `(unknown)` | `0x4` |
| `str` at 0x36ed4 | store | 4 | `sp -0x30` | `0x4` |
| `ldr` at 0x36edc | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x36ee0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x36ef4 | load | 4 | `[sp-108] +0x8` | `-` |
| `ldr` at 0x36efc | load | 4 | `sp -0x4c` | `-` |
| `strb` at 0x36f00 | store | 1 | `(unknown)` | `([sp-76] & 0xff)` |
| `ldr` at 0x36f04 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x36f0c | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x36f14 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x36f1c | store | 4 | `sp -0x68` | `[sp-104]+1` |
| `ldr` at 0x36f24 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x36f34 | store | 4 | `sp -0xb0` | `0x2` |
| `ldr` at 0x36f3c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x36f40 | store | 4 | `sp -0xb0` | `[sp-48]` |
| `ldr` at 0x36f44 | load | 4 | `sp -0xb0` | `-` |
| `ldm` at 0x36f4c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x36f4c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x36f4c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x36f4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x366f8 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x367e4 | `(unknown callee) 0x39124` | `[sp-104]` | `?` | `?` | `sp-140` |
| `bl` at 0x36810 | `(unknown callee) 0x38aac` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x368e0 | `(unknown callee) 0x621d4` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x36938 | `(unknown callee) 0x640a8` | `[sp-104]` | `?` | `sp-82` | `?` |
| `bl` at 0x369b8 | `(unknown callee) 0x397a4` | `[sp-104]` | `?` | `?` | `[sp-52]` |
| `bl` at 0x36a1c | `(unknown callee) 0x38d70` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x36af0 | `(unknown callee) 0x64238` | `[sp-104]` | `[sp-93]:1` | `sp-100` | `[sp-93]:1` |
| `bl` at 0x36bbc | `(unknown callee) 0x6e1b0` | `[sp-104]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x36bc8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x36c10 | `(unknown callee) 0x203f4` | `[sp-172]` | `[sp-164]` | `[sp-168]` | `?` |
| `bl` at 0x36c24 | `(unknown callee) 0x64414` | `[sp-104]` | `[sp-93]:1` | `?` | `[sp-93]:1` |
| `bl` at 0x36c64 | `(unknown callee) 0x22070` | `0x1` | `0x1596a8` | `0x186` | `0x0` |
| `bl` at 0x36cfc | `(unknown callee) 0x38aac` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x36d64 | `(unknown callee) 0x29908` | `0x1598c4` | `?` | `[sp-82]:1` | `[sp-80]` |
| `bl` at 0x36df8 | `(unknown callee) 0x639ec` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x36e54 | `(unknown callee) 0x38e80` | `[sp-104]` | `?` | `?` | `?` |
| `bl` at 0x36e80 | `(unknown callee) 0x38fd0` | `[sp-104]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x366e0 | `[0x1a5c8c]:1 == 0x0` | `[0x1a5c8c]:1 != 0x0` |
| `ble` at 0x36708 | `ret(0x6e140) <= [sp-104] (signed, 32-bit)` | `ret(0x6e140) > [sp-104] (signed, 32-bit)` |
| `beq` at 0x36718 | `[0x1a59f8] == 0x0` | `[0x1a59f8] != 0x0` |
| `bne` at 0x36738 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x36770 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x3681c | `ret(0x38aac) != 0x0` | `ret(0x38aac) == 0x0` |
| `ble` at 0x36898 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x368ec | `ret(0x621d4) == 0x0` | `ret(0x621d4) != 0x0` |
| `ble` at 0x369d0 | `[sp-144] <= [sp-48] (signed, 32-bit)` | `[sp-144] > [sp-48] (signed, 32-bit)` |
| `bgt` at 0x36a5c | `[0x19e664] > 0x0 (signed, 32-bit)` | `[0x19e664] <= 0x0 (signed, 32-bit)` |
| `ble` at 0x36a9c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0x36b28 | `? <= [sp-100] (unsigned, 32-bit)` | `? > [sp-100] (unsigned, 32-bit)` |
| `bne` at 0x36b34 | `[sp-100] != 0x0` | `[sp-100] == 0x0` |
| `bne` at 0x36b68 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x36ccc | `[0x31a4dc]:1 == 0x0` | `[0x31a4dc]:1 != 0x0` |
| `bgt` at 0x36cd8 | `[sp-48] > 0x3 (signed, 32-bit)` | `[sp-48] <= 0x3 (signed, 32-bit)` |
| `beq` at 0x36d14 | `[sp-82]:1 == [sp-80]` | `[sp-82]:1 != [sp-80]` |
| `beq` at 0x36d28 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x36da4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x36e34 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x36e60 | `ret(0x38e80) != 0x0` | `ret(0x38e80) == 0x0` |
| `bne` at 0x36e8c | `ret(0x38fd0) != 0x0` | `ret(0x38fd0) == 0x0` |
| `bgt` at 0x36ea0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x36f2c | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x36f4c | `r0` | `?` |

## `autofanEnv`

- `autofanEnv`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36f7c to 0x376ec.
  Established. The call to unnamed 0x3669c has candidate values sp-0x2c, sp-0x20, sp-0x24 and sp-0x28 (`bl` at 0x36fec).
  Known-edge stores clear the byte at absolute 0x31a4dc and put five and two in the words at 0x19e664 and 0x19e668 (`strb` at 0x37428, `str` at 0x37434, `str` at 0x37440).
  The computed transfer is unresolved (`ldrls` at 0x3714c).
  Omitted effects and the physical meaning of those environmental states remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x36f8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x36f90 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x36f94 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x36f98 | store | 4 | `sp -0xe4` | `r1` |
| `str` at 0x36fd4 | store | 4 | `(unknown)` | `sp-52` |
| `str` at 0x36fdc | store | 4 | `(unknown)` | `sp-56` |
| `str` at 0x36ff4 | store | 4 | `sp -0x1c` | `ret(0x3669c)` |
| `ldr` at 0x36ff8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37000 | load | 4 | `abs 0x375e0` | `-` |
| `ldr` at 0x37004 | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x3700c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x37010 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x37018 | load | 4 | `abs 0x37598` | `-` |
| `ldr` at 0x3701c | load | 4 | `abs 0x1a59fc` | `-` |
| `str` at 0x37024 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x37028 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x37034 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x37048 | store | 4 | `sp -0xe8` | `ret(0x6e1b0)` |
| `ldr` at 0x3704c | load | 4 | `sp -0xe8` | `-` |
| `str` at 0x3705c | store | 4 | `sp -0xec` | `ret(0x2031c)+1` |
| `ldr` at 0x37060 | load | 4 | `sp -0xec` | `-` |
| `str` at 0x37088 | store | 4 | `sp -0xf0` | `?` |
| `ldr` at 0x3708c | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x37090 | load | 4 | `sp -0xe8` | `-` |
| `ldr` at 0x37094 | load | 4 | `sp -0xec` | `-` |
| `ldr` at 0x370a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x370a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x370c4 | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x370cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x370d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x370e4 | load | 4 | `[sp-64] +0x4` | `-` |
| `ldr` at 0x370ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x370f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x370f4 | load | 4 | `abs 0x3759c` | `-` |
| `ldr` at 0x370f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3710c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x37110 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3712c | load | 4 | `[sp-60] +0x4` | `-` |
| `ldr` at 0x37134 | load | 4 | `abs 0x375a0` | `-` |
| `ldr` at 0x37138 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x37140 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x3714c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x37404 | load | 4 | `abs 0x375d0` | `-` |
| `ldr` at 0x37408 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37410 | load | 4 | `abs 0x375a4` | `-` |
| `ldr` at 0x37414 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37418 | store | 4 | `abs 0x19e668` | `[sp-28]` |
| `ldr` at 0x37420 | load | 4 | `abs 0x375d4` | `-` |
| `strb` at 0x37428 | store | 1 | `abs 0x31a4dc` | `0x0` |
| `ldr` at 0x3742c | load | 4 | `abs 0x375d8` | `-` |
| `str` at 0x37434 | store | 4 | `abs 0x19e664` | `0x5` |
| `ldr` at 0x37438 | load | 4 | `abs 0x375a4` | `-` |
| `str` at 0x37440 | store | 4 | `abs 0x19e668` | `0x2` |
| `str` at 0x3745c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x37468 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37474 | load | 4 | `abs 0x375dc` | `-` |
| `ldr` at 0x37478 | load | 4 | `abs 0x1a59f8` | `-` |
| `ldr` at 0x37484 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3748c | load | 4 | `abs 0x375e0` | `-` |
| `ldr` at 0x37490 | load | 4 | `abs 0x1a59f4` | `-` |
| `ldr` at 0x37498 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x374a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x374b0 | load | 4 | `abs 0x375e0` | `-` |
| `ldr` at 0x374b4 | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x374bc | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x374c4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x374c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x374cc | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x374d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x374dc | load | 4 | `abs 0x375e4` | `-` |
| `ldr` at 0x374e0 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x374ec | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x374f0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x374f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x37508 | load | 4 | `[sp-60] +0x8` | `-` |
| `strb` at 0x37514 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x37518 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x37520 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x37528 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x37530 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x3753c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x37548 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37554 | load | 4 | `abs 0x375dc` | `-` |
| `ldr` at 0x37558 | load | 4 | `abs 0x1a59f8` | `-` |
| `ldr` at 0x37564 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3756c | load | 4 | `abs 0x375e0` | `-` |
| `ldr` at 0x37570 | load | 4 | `abs 0x1a59f4` | `-` |
| `ldr` at 0x37578 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x375ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x375f4 | load | 4 | `abs 0x375e0` | `-` |
| `ldr` at 0x375f8 | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x37600 | store | 4 | `sp -0x3c` | `?` |
| `str` at 0x37608 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3760c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x37610 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x37614 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x37620 | load | 4 | `abs 0x375e4` | `-` |
| `ldr` at 0x37624 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x37630 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x37634 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x37638 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3764c | load | 4 | `[sp-60] +0x8` | `-` |
| `strb` at 0x37658 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x3765c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x37660 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3767c | load | 4 | `[sp-60] +0xc` | `-` |
| `str` at 0x37688 | store | 4 | `(unknown)` | `0xff` |
| `ldr` at 0x3768c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x37690 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x376ac | load | 4 | `[sp-60] +0xc` | `-` |
| `str` at 0x376b8 | store | 4 | `(unknown)` | `0xff` |
| `ldr` at 0x376bc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x376c4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x376cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x376d4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x376dc | load | 4 | `abs 0x375e8` | `-` |
| `ldr` at 0x376e0 | load | 4 | `abs 0x1a5a20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36f9c | `(unknown callee) 0x5572c` | `?` | `?` | `?` | `?` |
| `bl` at 0x36fb0 | `(unknown callee) 0x28230` | `0x6` | `?` | `?` | `ret(0x5572c)` |
| `bl` at 0x36fec | `(unknown callee) 0x3669c` | `sp-44` | `sp-32` | `sp-36` | `sp-40` |
| `bl` at 0x37044 | `(unknown callee) 0x6e1b0` | `[sp-52]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x37050 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x37098 | `(unknown callee) 0x203f4` | `[sp-240]` | `[sp-232]` | `[sp-236]` | `?` |
| `bl` at 0x37100 | `(unknown callee) 0x205bc` | `sp-224` | `0x1598d4` | `?` | `ret(0x203f4)` |
| `bl` at 0x3713c | `(unknown callee) 0x205bc` | `sp-224` | `0x1598e0` | `?` | `?` |
| `bl` at 0x3740c | `(unknown callee) 0x29908` | `0x1599b8` | `[sp-28]` | `?` | `[sp-28]-2` |
| `bl` at 0x37448 | `(unknown callee) 0x28230` | `0x6` | `?` | `0x19e668` | `0x2` |
| `bl` at 0x37460 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x37540 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x376e4 | `(unknown callee) 0x376ec` | `[0x1a5a20]` | `?` | `?` | `0x1a5a20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x36fa8 | `ret(0x5572c) == 0x0` | `ret(0x5572c) != 0x0` |
| `beq` at 0x36fbc | `ret(0x28230) == 0x0` | `ret(0x28230) != 0x0` |
| `beq` at 0x37030 | `[sp-52] == 0xff` | `[sp-52] != 0xff` |
| `ldrls` at 0x3714c | `[sp-28]-2 <= 0x1e (unsigned, 32-bit)` | `[sp-28]-2 > 0x1e (unsigned, 32-bit)` |
| `bne` at 0x37454 | `ret(0x28230) != 0x0` | `ret(0x28230) == 0x0` |
| `ble` at 0x37470 | `ret(0x6e140) <= [sp-44] (signed, 32-bit)` | `ret(0x6e140) > [sp-44] (signed, 32-bit)` |
| `beq` at 0x37480 | `[0x1a59f8] == 0x0` | `[0x1a59f8] != 0x0` |
| `bne` at 0x374a0 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x374d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x37550 | `ret(0x6e140) <= [sp-44] (signed, 32-bit)` | `ret(0x6e140) > [sp-44] (signed, 32-bit)` |
| `beq` at 0x37560 | `[0x1a59f8] == 0x0` | `[0x1a59f8] != 0x0` |
| `bne` at 0x37580 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x3761c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3714c).

## `autofanEnvQuery`

- `autofanEnvQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37714 to 0x38a40.
  Established. The entry coprocessor store addresses sp-0x1c with unknown width and value (`stc` at 0x3771c).
  Known-edge byte stores can put 0x2d, 0x3d or 0x2b at sp-0x9d (`strb` at 0x38748, `strb` at 0x38784, `strb` at 0x38790).
  The final call to unnamed 0x20634 has candidate values the word at sp-0x20, 0x159ba8, a result of unnamed 0x6e140 and the word at sp-0xa4 (`bl` at 0x38a08).
  The computed transfer is unresolved (`ldrls` at 0x37a54), leaving omitted effects and the query's physical interpretation unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x3771c | store | unknown | `sp -0x1c` | `?` |
| `str` at 0x37728 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x37730 | store | 4 | `sp -0x60` | `sp-92` |
| `str` at 0x37738 | store | 4 | `sp -0x70` | `sp-108` |
| `ldr` at 0x3773c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3774c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37754 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x37758 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3775c | load | 4 | `abs 0x380b0` | `-` |
| `ldr` at 0x37764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x37774 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3777c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `str` at 0x37784 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0x37790 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x3779c | load | 4 | `abs 0x380b4` | `-` |
| `ldr` at 0x377a0 | load | 4 | `abs 0x1a59f8` | `-` |
| `ldr` at 0x377ac | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x377b4 | load | 4 | `abs 0x380b8` | `-` |
| `ldr` at 0x377b8 | load | 4 | `abs 0x1a59f4` | `-` |
| `ldr` at 0x377c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x377d0 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x377d8 | load | 4 | `abs 0x380b8` | `-` |
| `ldr` at 0x377dc | load | 4 | `abs 0x1a59f4` | `-` |
| `str` at 0x377e4 | store | 4 | `sp -0xa8` | `?` |
| `str` at 0x377ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x377f0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x377f4 | load | 4 | `[sp-168] +0x0` | `-` |
| `ldr` at 0x377f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37804 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37808 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37824 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x3782c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37830 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3784c | load | 4 | `[sp-168] +0xc` | `-` |
| `str` at 0x3785c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x37860 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x37870 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37878 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x37890 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37894 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x378b0 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x378b8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x378bc | store | 1 | `sp -0xe9` | `?` |
| `strb` at 0x378c8 | store | 1 | `sp -0xe9` | `0x0` |
| `str` at 0x378d0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x378d4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x378d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x378f4 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x378fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x37900 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3790c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37910 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3792c | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x37938 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x37944 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x37948 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x37960 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37964 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37980 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x3798c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x37998 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x379a0 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x379ac | load | 4 | `sp -0x34` | `-` |
| `str` at 0x379b4 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x379bc | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x379c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x379dc | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x379e4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x379e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37a04 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldrb` at 0x37a0c | load | 1 | `sp -0xe9` | `-` |
| `str` at 0x37a10 | store | 4 | `(unknown)` | `?` |
| `str` at 0x37a18 | store | 4 | `(unknown)` | `sp-220` |
| `ldr` at 0x37a1c | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x37a24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37a2c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37a30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37a44 | load | 4 | `[sp-168] +0x8` | `-` |
| `ldrb` at 0x37a4c | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x37a54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x37b34 | load | 4 | `abs 0x380d8` | `-` |
| `str` at 0x37b38 | store | 4 | `sp -0x50` | `0x159b18` |
| `ldr` at 0x37b3c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37b40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37b5c | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x37b64 | load | 4 | `(unknown)` | `-` |
| `str` at 0x37b78 | store | 4 | `sp -0x60` | `sp-92` |
| `ldr` at 0x37b7c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x37b80 | load | 4 | `abs 0x380dc` | `-` |
| `ldr` at 0x37b84 | load | 4 | `abs 0x380e0` | `-` |
| `ldr` at 0x37b94 | load | 4 | `abs 0x380dc` | `-` |
| `ldr` at 0x37b98 | load | 4 | `abs 0x380e0` | `-` |
| `ldr` at 0x37ba8 | load | 4 | `abs 0x380dc` | `-` |
| `ldr` at 0x37bac | load | 4 | `abs 0x380e0` | `-` |
| `str` at 0x37bbc | store | 4 | `sp -0x60` | `sp-92` |
| `ldr` at 0x37bc0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37bc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37be0 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x37be8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x37bf4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x37bf8 | load | 4 | `abs 0x380e4` | `-` |
| `str` at 0x37c08 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x37c0c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37c10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37c2c | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x37c34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x37c38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37c44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37c50 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x37c54 | load | 4 | `abs 0x38100` | `-` |
| `ldr` at 0x37c5c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x37c6c | load | 4 | `sp -0x60` | `-` |
| `str` at 0x37c74 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x2031c))` |
| `ldr` at 0x37c78 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37c7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37c98 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x37ca4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x37cb0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x37cb4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x37cb8 | load | 4 | `abs 0x38104` | `-` |
| `ldr` at 0x37cc4 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x37cd4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x37cdc | store | 4 | `sp -0x60` | `([sp-96] + ret(0x2031c))` |
| `ldr` at 0x37ce0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x37ce8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x37cf0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37cf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37d10 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x37d18 | load | 1 | `(unknown)` | `-` |
| `str` at 0x37d1c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x37d20 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37d24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37d40 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldr` at 0x37d48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x37d4c | store | 4 | `sp -0xe0` | `?` |
| `ldr` at 0x37d50 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37d54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37d70 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldr` at 0x37d78 | load | 4 | `(unknown)` | `-` |
| `str` at 0x37d7c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x37d80 | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0x37d8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37da0 | load | 4 | `sp -0x44` | `-` |
| `ldc` at 0x37da8 | load | unknown | `abs 0x380a0` | `-` |
| `ldc` at 0x37db0 | load | unknown | `abs 0x380a8` | `-` |
| `ldr` at 0x37dc0 | load | 4 | `abs 0x380e8` | `-` |
| `ldr` at 0x37dc4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x37dd0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37dd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37df0 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldc` at 0x37df8 | load | unknown | `(unknown)` | `-` |
| `ldr` at 0x37e00 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37e04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37e20 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldc` at 0x37e28 | load | unknown | `(unknown)` | `-` |
| `ldc` at 0x37e34 | load | unknown | `abs 0x380a0` | `-` |
| `ldc` at 0x37e3c | load | unknown | `abs 0x380a8` | `-` |
| `ldr` at 0x37e48 | load | 4 | `abs 0x380ec` | `-` |
| `ldr` at 0x37e58 | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0x37e60 | load | 4 | `abs 0x380f4` | `-` |
| `ldr` at 0x37e64 | load | 4 | `abs 0x1a59fc` | `-` |
| `str` at 0x37e6c | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x37e74 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x37e78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37e8c | load | 4 | `[sp-172] +0x4` | `-` |
| `ldrb` at 0x37e94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x37e98 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x37e9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37eb0 | load | 4 | `[sp-172] +0x4` | `-` |
| `ldrb` at 0x37eb8 | load | 1 | `(unknown)` | `-` |
| `ldc` at 0x37ec0 | load | unknown | `abs 0x380a0` | `-` |
| `ldc` at 0x37ec8 | load | unknown | `abs 0x380a8` | `-` |
| `ldr` at 0x37ed4 | load | 4 | `abs 0x380e8` | `-` |
| `ldr` at 0x37ee4 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x37ee8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37efc | load | 4 | `[sp-172] +0xc` | `-` |
| `ldc` at 0x37f04 | load | unknown | `(unknown)` | `-` |
| `ldr` at 0x37f0c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x37f10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x37f24 | load | 4 | `[sp-172] +0xc` | `-` |
| `ldc` at 0x37f2c | load | unknown | `(unknown)` | `-` |
| `ldc` at 0x37f38 | load | unknown | `abs 0x380a0` | `-` |
| `ldc` at 0x37f40 | load | unknown | `abs 0x380a8` | `-` |
| `ldr` at 0x37f4c | load | 4 | `abs 0x380ec` | `-` |
| `str` at 0x37f5c | store | 4 | `sp -0x70` | `sp-108` |
| `str` at 0x37f64 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x37f68 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37f6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37f88 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x37f90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x37f94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37fa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x37fac | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x37fb0 | load | 4 | `abs 0x38100` | `-` |
| `ldr` at 0x37fb8 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x37fc8 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x37fd0 | store | 4 | `sp -0x70` | `([sp-112] + ret(0x2031c))` |
| `ldr` at 0x37fd4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x37fd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x37ff4 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x38000 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3800c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x38010 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x38014 | load | 4 | `abs 0x38104` | `-` |
| `ldr` at 0x38020 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x38030 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x38038 | store | 4 | `sp -0x70` | `([sp-112] + ret(0x2031c))` |
| `ldr` at 0x3803c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x38044 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x3804c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38054 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x3806c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38070 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3808c | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x38094 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x38098 | store | 1 | `sp -0x49` | `?` |
| `strb` at 0x38118 | store | 1 | `sp -0x49` | `0x0` |
| `str` at 0x38120 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x38124 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38128 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38144 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x3814c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x38150 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3815c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38160 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3817c | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x38188 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x38194 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x38198 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x381b0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x381b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x381d0 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x381dc | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x381e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x381f0 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x381fc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x38204 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x3820c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x38214 | load | 1 | `sp -0x49` | `-` |
| `ldr` at 0x38218 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x38228 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38230 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38234 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38250 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldrb` at 0x38258 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3825c | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x38270 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38274 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38290 | load | 4 | `[sp-168] +0x4` | `-` |
| `str` at 0x3829c | store | 4 | `(unknown)` | `sp-92` |
| `str` at 0x382a4 | store | 4 | `(unknown)` | `sp-136` |
| `str` at 0x382ac | store | 4 | `(unknown)` | `sp-124` |
| `str` at 0x382b4 | store | 4 | `(unknown)` | `sp-108` |
| `stc` at 0x382b8 | store | unknown | `(unknown)` | `?` |
| `stc` at 0x382bc | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x382c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x382c4 | load | 4 | `abs 0x380f0` | `-` |
| `ldr` at 0x382c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x382cc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x382d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x382e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x382ec | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x382f0 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x382f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38310 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x38318 | load | 4 | `(unknown)` | `-` |
| `str` at 0x38328 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3832c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38330 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3834c | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x38354 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38358 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38364 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38368 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38384 | load | 4 | `[sp-168] +0x4` | `-` |
| `ldr` at 0x3838c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x383a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x383a4 | store | 4 | `sp -0xe0` | `?` |
| `ldr` at 0x383a8 | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0x383b0 | load | 4 | `abs 0x380f4` | `-` |
| `ldr` at 0x383b4 | load | 4 | `abs 0x1a59fc` | `-` |
| `str` at 0x383bc | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x383c0 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x383c4 | load | 4 | `[sp-172] +0x0` | `-` |
| `str` at 0x383c8 | store | 4 | `sp -0xe4` | `[[sp-172]]` |
| `str` at 0x383d0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x383d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x383d8 | load | 4 | `sp -0xe4` | `-` |
| `ldr` at 0x383e4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x383e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38404 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldr` at 0x3840c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38410 | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0x3841c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38420 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3843c | load | 4 | `[sp-168] +0xc` | `-` |
| `ldr` at 0x38444 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38448 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38454 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38458 | load | 4 | `abs 0x380f8` | `-` |
| `ldr` at 0x38460 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38470 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38478 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x38480 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38484 | load | 4 | `abs 0x380fc` | `-` |
| `ldr` at 0x3848c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3849c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x384a4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x384a8 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x384ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x384c0 | load | 4 | `[sp-172] +0x4` | `-` |
| `ldrb` at 0x384c8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x384cc | store | 4 | `sp -0xe8` | `?` |
| `str` at 0x384d4 | store | 4 | `sp -0x60` | `sp-92` |
| `str` at 0x384dc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x384e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x384e4 | load | 4 | `sp -0xe8` | `-` |
| `ldr` at 0x384f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x384fc | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x38500 | load | 4 | `abs 0x38100` | `-` |
| `ldr` at 0x38508 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x38518 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x38520 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x2031c))` |
| `ldr` at 0x38524 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x38528 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3853c | load | 4 | `[sp-172] +0x4` | `-` |
| `ldr` at 0x38544 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3854c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x38550 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x38554 | load | 4 | `abs 0x38104` | `-` |
| `ldr` at 0x38560 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x38570 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x38578 | store | 4 | `sp -0x60` | `([sp-96] + ret(0x2031c))` |
| `ldr` at 0x3857c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x38584 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x3858c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x38590 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x385a4 | load | 4 | `[sp-172] +0xc` | `-` |
| `ldr` at 0x385ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x385b8 | load | 4 | `sp -0xe0` | `-` |
| `str` at 0x385cc | store | 4 | `sp -0xf0` | `ret(0x6e1b0)` |
| `ldr` at 0x385d0 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x385e0 | store | 4 | `sp -0xf4` | `ret(0x2031c)+1` |
| `ldr` at 0x385e4 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x3860c | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0x38610 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x38614 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x38618 | load | 4 | `sp -0xf4` | `-` |
| `ldr` at 0x38628 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x3862c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38640 | load | 4 | `[sp-172] +0x4` | `-` |
| `ldr` at 0x3864c | load | 4 | `abs 0x38108` | `-` |
| `ldr` at 0x38654 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38668 | load | 4 | `abs 0x3810c` | `-` |
| `ldr` at 0x38678 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3867c | load | 4 | `abs 0x38110` | `-` |
| `ldr` at 0x38684 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38694 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3869c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x386a8 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x386ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x386c0 | load | 4 | `[sp-172] +0x4` | `-` |
| `ldrb` at 0x386c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x386cc | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x386d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x386e4 | load | 4 | `[sp-172] +0x4` | `-` |
| `ldrb` at 0x386ec | load | 1 | `(unknown)` | `-` |
| `ldc` at 0x386f4 | load | unknown | `abs 0x38a18` | `-` |
| `ldc` at 0x386fc | load | unknown | `abs 0x38a20` | `-` |
| `ldr` at 0x38708 | load | 4 | `abs 0x38a28` | `-` |
| `ldr` at 0x38714 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x38718 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3872c | load | 4 | `[sp-172] +0xc` | `-` |
| `ldc` at 0x38734 | load | unknown | `(unknown)` | `-` |
| `strb` at 0x38748 | store | 1 | `sp -0x9d` | `0x2d` |
| `ldr` at 0x38750 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x38754 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38768 | load | 4 | `[sp-172] +0xc` | `-` |
| `ldc` at 0x38770 | load | unknown | `(unknown)` | `-` |
| `strb` at 0x38784 | store | 1 | `sp -0x9d` | `0x3d` |
| `strb` at 0x38790 | store | 1 | `sp -0x9d` | `0x2b` |
| `ldr` at 0x38798 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x3879c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x387b0 | load | 4 | `[sp-172] +0xc` | `-` |
| `ldc` at 0x387b8 | load | unknown | `(unknown)` | `-` |
| `ldr` at 0x387c0 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x387c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x387d8 | load | 4 | `[sp-172] +0xc` | `-` |
| `ldc` at 0x387e0 | load | unknown | `(unknown)` | `-` |
| `ldc` at 0x387ec | load | unknown | `abs 0x38a18` | `-` |
| `ldc` at 0x387f4 | load | unknown | `abs 0x38a20` | `-` |
| `ldrb` at 0x38800 | load | 1 | `sp -0x9d` | `-` |
| `str` at 0x38804 | store | 4 | `(unknown)` | `[sp-157]:1` |
| `ldr` at 0x38808 | load | 4 | `abs 0x38a2c` | `-` |
| `ldr` at 0x38818 | load | 4 | `sp -0xe0` | `-` |
| `str` at 0x3882c | store | 4 | `sp -0xf8` | `ret(0x6e1b0)` |
| `ldr` at 0x38830 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0x38840 | store | 4 | `sp -0xf4` | `ret(0x2031c)+1` |
| `ldr` at 0x38844 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x3886c | store | 4 | `sp -0xf0` | `?` |
| `ldr` at 0x38870 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x38874 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x38878 | load | 4 | `sp -0xf4` | `-` |
| `ldr` at 0x38888 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x3888c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x388a0 | load | 4 | `[sp-172] +0x4` | `-` |
| `ldr` at 0x388ac | load | 4 | `abs 0x38a30` | `-` |
| `ldr` at 0x388b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x388c8 | store | 4 | `(unknown)` | `sp-136` |
| `str` at 0x388d0 | store | 4 | `(unknown)` | `sp-124` |
| `ldr` at 0x388d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x388d8 | load | 4 | `abs 0x38a34` | `-` |
| `ldr` at 0x388e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x388f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x388fc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x38900 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x38904 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38920 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldr` at 0x38928 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3892c | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0x38938 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x3893c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38958 | load | 4 | `[sp-168] +0xc` | `-` |
| `ldr` at 0x38960 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38964 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38970 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38974 | load | 4 | `abs 0x38a38` | `-` |
| `ldr` at 0x3897c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3898c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38994 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x38998 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3899c | load | 4 | `abs 0x38a3c` | `-` |
| `ldr` at 0x389a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x389b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x389bc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x389c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x389c8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x389d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x389d8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x389e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x389e8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x389f0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x389f8 | store | 4 | `sp -0xa4` | `[sp-164]+1` |
| `ldr` at 0x38a00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38a04 | load | 4 | `abs 0x38a3c` | `-` |
| `ldc` at 0x38a0c | load | unknown | `sp -0x1c` | `-` |
| `ldm` at 0x38a14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x38a14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x38a14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37740 | `(unknown callee) 0x2031c` | `r0` | `r1` | `r2` | `sp-108` |
| `bl` at 0x37760 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x1599dc` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x37768 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x37788 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x3786c | `(unknown callee) 0x39124` | `[sp-164]` | `?` | `?` | `sp-200` |
| `bl` at 0x37880 | `(unknown callee) 0x38aac` | `[sp-164]` | `?` | `?` | `?` |
| `bl` at 0x37950 | `(unknown callee) 0x621d4` | `[sp-164]` | `?` | `?` | `?` |
| `bl` at 0x379a8 | `(unknown callee) 0x640a8` | `[sp-164]` | `?` | `sp-233` | `?` |
| `bl` at 0x37a28 | `(unknown callee) 0x397a4` | `[sp-164]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x37b88 | `(unknown callee) 0x205bc` | `sp-92` | `0x159b20` | `0x159b24` | `sp-92` |
| `bl` at 0x37b9c | `(unknown callee) 0x205bc` | `sp-136` | `0x159b20` | `0x159b24` | `sp-136` |
| `bl` at 0x37bb0 | `(unknown callee) 0x205bc` | `sp-124` | `0x159b20` | `0x159b24` | `sp-124` |
| `bl` at 0x37bfc | `(unknown callee) 0x205bc` | `sp-92` | `0x159b28` | `?` | `?` |
| `bl` at 0x37c58 | `(unknown callee) 0x205bc` | `?` | `0x159b80` | `?` | `?` |
| `bl` at 0x37c60 | `(unknown callee) 0x2031c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x37cc0 | `(unknown callee) 0x205bc` | `?` | `0x159b84` | `?` | `?` |
| `bl` at 0x37cc8 | `(unknown callee) 0x2031c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x37dc8 | `(unknown callee) 0x205bc` | `sp-136` | `0x159b2c` | `[sp-68]` | `?` |
| `bl` at 0x37e50 | `(unknown callee) 0x205bc` | `sp-124` | `0x159b38` | `?` | `?` |
| `bl` at 0x37edc | `(unknown callee) 0x205bc` | `sp-136` | `0x159b2c` | `?` | `?` |
| `bl` at 0x37f54 | `(unknown callee) 0x205bc` | `sp-124` | `0x159b38` | `?` | `?` |
| `bl` at 0x37fb4 | `(unknown callee) 0x205bc` | `?` | `0x159b80` | `?` | `?` |
| `bl` at 0x37fbc | `(unknown callee) 0x2031c` | `[sp-112]` | `?` | `?` | `?` |
| `bl` at 0x3801c | `(unknown callee) 0x205bc` | `?` | `0x159b84` | `?` | `?` |
| `bl` at 0x38024 | `(unknown callee) 0x2031c` | `[sp-112]` | `?` | `?` | `?` |
| `bl` at 0x3805c | `(unknown callee) 0x38aac` | `[sp-164]` | `?` | `?` | `?` |
| `bl` at 0x381a0 | `(unknown callee) 0x621d4` | `[sp-164]` | `?` | `?` | `?` |
| `bl` at 0x381f8 | `(unknown callee) 0x640a8` | `[sp-164]` | `?` | `sp-73` | `?` |
| `bl` at 0x38220 | `(unknown callee) 0x38d70` | `[sp-164]` | `?` | `?` | `?` |
| `bl` at 0x38268 | `(unknown callee) 0x38d70` | `[sp-164]` | `?` | `?` | `?` |
| `bl` at 0x382d0 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x159b48` | `?` | `[sp-80]` |
| `bl` at 0x382d8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3845c | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x159b78` | `?` | `?` |
| `bl` at 0x38464 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x38488 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x159b7c` | `?` | `?` |
| `bl` at 0x38490 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x38504 | `(unknown callee) 0x205bc` | `?` | `0x159b80` | `?` | `?` |
| `bl` at 0x3850c | `(unknown callee) 0x2031c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x3855c | `(unknown callee) 0x205bc` | `?` | `0x159b84` | `?` | `?` |
| `bl` at 0x38564 | `(unknown callee) 0x2031c` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0x385c8 | `(unknown callee) 0x6e1b0` | `[sp-224]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x385d4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x3861c | `(unknown callee) 0x203f4` | `[sp-248]` | `[sp-240]` | `[sp-244]` | `?` |
| `bl` at 0x38658 | `(unknown callee) 0x205bc` | `sp-156` | `0x159b88` | `ret(0x203f4)` | `?` |
| `bl` at 0x38674 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x159b90` | `sp-156` | `sp-92` |
| `bl` at 0x38680 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x159ba8` | `?` | `?` |
| `bl` at 0x38688 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x38710 | `(unknown callee) 0x205bc` | `sp-136` | `0x159b2c` | `?` | `?` |
| `bl` at 0x38814 | `(unknown callee) 0x205bc` | `sp-124` | `0x159bac` | `?` | `?` |
| `bl` at 0x38828 | `(unknown callee) 0x6e1b0` | `[sp-224]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x38834 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x3887c | `(unknown callee) 0x203f4` | `[sp-240]` | `[sp-248]` | `[sp-244]` | `?` |
| `bl` at 0x388b8 | `(unknown callee) 0x205bc` | `sp-156` | `0x159b88` | `ret(0x203f4)` | `?` |
| `bl` at 0x388e0 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x159bbc` | `sp-156` | `sp-92` |
| `bl` at 0x388e8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x38978 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x159bd0` | `?` | `[sp-44]` |
| `bl` at 0x38980 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x389a0 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x159ba8` | `?` | `?` |
| `bl` at 0x389a8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x38a08 | `(unknown callee) 0x20634` | `[sp-32]` | `0x159ba8` | `ret(0x6e140)` | `[sp-164]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x37798 | `ret(0x6e140) <= [sp-164] (signed, 32-bit)` | `ret(0x6e140) > [sp-164] (signed, 32-bit)` |
| `beq` at 0x377a8 | `[0x1a59f8] == 0x0` | `[0x1a59f8] != 0x0` |
| `bne` at 0x377c8 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x37800 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x3788c | `ret(0x38aac) != 0x0` | `ret(0x38aac) == 0x0` |
| `ble` at 0x37908 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x3795c | `ret(0x621d4) == 0x0` | `ret(0x621d4) != 0x0` |
| `ldrls` at 0x37a54 | `? <= 0x20 (unsigned, 32-bit)` | `? > 0x20 (unsigned, 32-bit)` |
| `beq` at 0x37b70 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x37bf0 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x37c40 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x37c4c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x37d88 | `[sp-224] == 0xff` | `[sp-224] != 0xff` |
| `beq` at 0x37d94 | `[sp-44] == 0xff` | `[sp-44] != 0xff` |
| `ble` at 0x37f9c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x37fa8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x38068 | `ret(0x38aac) != 0x0` | `ret(0x38aac) == 0x0` |
| `ble` at 0x38158 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x381ac | `ret(0x621d4) == 0x0` | `ret(0x621d4) != 0x0` |
| `beq` at 0x38320 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x38360 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0x383e0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x38418 | `? != [sp-224]` | `? == [sp-224]` |
| `bne` at 0x38450 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x384ec | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x384f8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x385b4 | `? != 0x0` | `? == 0x0` |
| `bmi` at 0x3873c | `N == 1` | `not (N == 1)` |
| `beq` at 0x38778 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x38934 | `? != [sp-224]` | `? == [sp-224]` |
| `bne` at 0x3896c | `? != [sp-44]` | `? == [sp-44]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x38a14 | `r0` | `ret(0x20634)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x37a54).
