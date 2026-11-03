# ARM Linux 2.6 external interrupt paths in 1.44.0

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
No unresolved computed transfers were reported for these routines.
No undecodable byte ranges were reported in these spans.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-external-interrupts-facts.txt) retain all tool sections.

## `extIntInit`

- `extIntInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b350 to 0x10b4e8.
  Established. A byte store writes an unknown value to absolute 0x31473c, and the call to unnamed 0x204d8 has candidate r0/r1 values [0x31473c]:1/0x44 (`strb` at 0x10b3d0, `bl` at 0x10b3e8).
  A word store saves its result at absolute 0x314738, and a later word store writes 0 at [sp-36] +8 (`str` at 0x10b3ec, `str` at 0x10b4b4).
  A further byte store has unknown effective address and value (`strb` at 0x10b48c), so the initialized field and its hardware meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10b360 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x10b364 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x10b368 | load | 4 | `abs 0x10b4d0` | `-` |
| `ldrb` at 0x10b36c | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b378 | load | 4 | `abs 0x10b4d4` | `-` |
| `str` at 0x10b37c | store | 4 | `sp -0x2c` | `0x18ca38` |
| `ldr` at 0x10b380 | load | 4 | `abs 0x10b4d0` | `-` |
| `ldrb` at 0x10b384 | load | 1 | `abs 0x31473c` | `-` |
| `str` at 0x10b388 | store | 4 | `sp -0x28` | `[0x31473c]:1` |
| `ldr` at 0x10b390 | load | 4 | `abs 0x10b4d8` | `-` |
| `ldr` at 0x10b3a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10b3ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10b3b8 | load | 4 | `abs 0x10b4dc` | `-` |
| `str` at 0x10b3bc | store | 4 | `sp -0x18` | `0x1a04c8` |
| `str` at 0x10b3c4 | store | 4 | `sp -0x1c` | `0x6` |
| `ldr` at 0x10b3c8 | load | 4 | `abs 0x10b4d0` | `-` |
| `ldr` at 0x10b3cc | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x10b3d0 | store | 1 | `abs 0x31473c` | `?` |
| `ldr` at 0x10b3d4 | load | 4 | `abs 0x10b4e0` | `-` |
| `ldr` at 0x10b3d8 | load | 4 | `abs 0x10b4d0` | `-` |
| `ldrb` at 0x10b3dc | load | 1 | `abs 0x31473c` | `-` |
| `str` at 0x10b3ec | store | 4 | `abs 0x314738` | `ret(0x204d8)` |
| `ldr` at 0x10b3f0 | load | 4 | `abs 0x314738` | `-` |
| `ldr` at 0x10b3fc | load | 4 | `abs 0x10b4e4` | `-` |
| `str` at 0x10b400 | store | 4 | `sp -0x2c` | `0x18ca70` |
| `ldr` at 0x10b404 | load | 4 | `abs 0x10b4d0` | `-` |
| `ldrb` at 0x10b408 | load | 1 | `abs 0x31473c` | `-` |
| `str` at 0x10b41c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x10b424 | load | 4 | `abs 0x10b4d8` | `-` |
| `str` at 0x10b438 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x10b43c | load | 4 | `abs 0x10b4d0` | `-` |
| `ldrb` at 0x10b440 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b444 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10b450 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10b464 | load | 4 | `abs 0x10b4e0` | `-` |
| `ldr` at 0x10b468 | load | 4 | `abs 0x314738` | `-` |
| `str` at 0x10b470 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x10b474 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10b478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10b480 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10b488 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x10b48c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x10b490 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x10b494 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10b49c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10b4a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x10b4a8 | store | 4 | `[sp-36] +0xc` | `?` |
| `ldr` at 0x10b4ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x10b4b4 | store | 4 | `[sp-36] +0x8` | `0x0` |
| `ldr` at 0x10b4b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10b4c0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0x10b4cc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10b4cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b4cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b4cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10b39c | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x61` | `0x0` |
| `bl` at 0x10b3e8 | `(unknown callee) 0x204d8` | `[0x31473c]:1` | `0x44` | `0x31473c` | `[0x31473c]:1` |
| `bl` at 0x10b430 | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x71` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10b374 | `[0x31473c]:1 == 0x0` | `[0x31473c]:1 != 0x0` |
| `beq` at 0x10b3a8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x10b3b4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x10b3f8 | `[0x314738] != 0x0` | `[0x314738] == 0x0` |
| `ble` at 0x10b44c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b4cc | `r0` | `?` |

## `extIntRegister`

- `extIntRegister`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b4e8 to 0x10b644.
  Established. A byte load reads [sp-32] +0x38, and an unsigned comparison branches to 0x10b5ac when that loaded byte is at most 2 (`ldrb` at 0x10b578, `bls` at 0x10b580).
  A word store writes an unknown value at [sp-28], while byte stores write ([sp-22]:1 & 0xff) and ([sp-23]:1 & 0xff) at [sp-28] +8 and +9 (`str` at 0x10b5f8, `strb` at 0x10b604, `strb` at 0x10b610).
  A further byte store updates [sp-32] +0x38 to ([[sp-32]+56]:1+1 & 0xff), but the represented software fields remain Unknown (`strb` at 0x10b624).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10b4f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x10b504 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x10b50c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x10b514 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x10b518 | load | 4 | `abs 0x10b630` | `-` |
| `ldrb` at 0x10b51c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x10b520 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b52c | load | 4 | `abs 0x10b634` | `-` |
| `str` at 0x10b530 | store | 4 | `sp -0x28` | `0x18caa0` |
| `ldrb` at 0x10b534 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x10b538 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x10b540 | load | 4 | `abs 0x10b638` | `-` |
| `ldrb` at 0x10b550 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x10b564 | load | 4 | `abs 0x10b63c` | `-` |
| `ldr` at 0x10b568 | load | 4 | `abs 0x314738` | `-` |
| `str` at 0x10b570 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x10b574 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10b578 | load | 1 | `[sp-32] +0x38` | `-` |
| `ldr` at 0x10b584 | load | 4 | `abs 0x10b640` | `-` |
| `str` at 0x10b588 | store | 4 | `sp -0x28` | `0x18cae4` |
| `ldr` at 0x10b58c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10b590 | load | 1 | `[sp-32] +0x38` | `-` |
| `str` at 0x10b594 | store | 4 | `sp -0x24` | `[[sp-32]+56]:1` |
| `ldr` at 0x10b59c | load | 4 | `abs 0x10b638` | `-` |
| `ldrb` at 0x10b5ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x10b5c0 | load | 4 | `abs 0x10b63c` | `-` |
| `ldr` at 0x10b5c4 | load | 4 | `abs 0x314738` | `-` |
| `ldrb` at 0x10b5cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x10b5e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10b5ec | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x10b5f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10b5f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10b5f8 | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x10b5fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x10b600 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x10b604 | store | 1 | `[sp-28] +0x8` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x10b608 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x10b60c | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x10b610 | store | 1 | `[sp-28] +0x9` | `([sp-23]:1 & 0xff)` |
| `ldr` at 0x10b614 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10b618 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10b61c | load | 1 | `[sp-32] +0x38` | `-` |
| `strb` at 0x10b624 | store | 1 | `[sp-32] +0x38` | `([[sp-32]+56]:1+1 & 0xff)` |
| `ldm` at 0x10b62c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b62c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b62c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10b54c | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x8f` | `0x0` |
| `bl` at 0x10b5a8 | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x97` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10b528 | `(r1 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r1 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |
| `bls` at 0x10b580 | `[[sp-32]+56]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-32]+56]:1 > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b62c | `r0` | `?` |

## `extIntRegister2`

- `extIntRegister2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b644 to 0x10b7a0.
  Established. The unsigned comparison takes the branch to 0x10b6ac when (entry r1 & 0xff) is less than [0x31473c]:1 (`blo` at 0x10b684).
  A word store writes an unknown value at [sp-28] +4, followed by byte stores at [sp-28] +8 and +9 (`str` at 0x10b754, `strb` at 0x10b760, `strb` at 0x10b76c).
  The final byte store writes ([[sp-32]+56]:1+1 & 0xff) at [sp-32] +0x38 (`strb` at 0x10b780).
  The resolved relative addresses do not identify physical interrupt roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10b654 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x10b660 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x10b668 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x10b670 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x10b674 | load | 4 | `abs 0x10b78c` | `-` |
| `ldrb` at 0x10b678 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x10b67c | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b688 | load | 4 | `abs 0x10b790` | `-` |
| `str` at 0x10b68c | store | 4 | `sp -0x28` | `0x18caa0` |
| `ldrb` at 0x10b690 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x10b694 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x10b69c | load | 4 | `abs 0x10b794` | `-` |
| `ldrb` at 0x10b6ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x10b6c0 | load | 4 | `abs 0x10b798` | `-` |
| `ldr` at 0x10b6c4 | load | 4 | `abs 0x314738` | `-` |
| `str` at 0x10b6cc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x10b6d0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10b6d4 | load | 1 | `[sp-32] +0x38` | `-` |
| `ldr` at 0x10b6e0 | load | 4 | `abs 0x10b79c` | `-` |
| `str` at 0x10b6e4 | store | 4 | `sp -0x28` | `0x18cae4` |
| `ldr` at 0x10b6e8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10b6ec | load | 1 | `[sp-32] +0x38` | `-` |
| `str` at 0x10b6f0 | store | 4 | `sp -0x24` | `[[sp-32]+56]:1` |
| `ldr` at 0x10b6f8 | load | 4 | `abs 0x10b794` | `-` |
| `ldrb` at 0x10b708 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x10b71c | load | 4 | `abs 0x10b798` | `-` |
| `ldr` at 0x10b720 | load | 4 | `abs 0x314738` | `-` |
| `ldrb` at 0x10b728 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x10b73c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10b748 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x10b74c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10b750 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10b754 | store | 4 | `[sp-28] +0x4` | `?` |
| `ldr` at 0x10b758 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x10b75c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x10b760 | store | 1 | `[sp-28] +0x8` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0x10b764 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x10b768 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x10b76c | store | 1 | `[sp-28] +0x9` | `([sp-23]:1 & 0xff)` |
| `ldr` at 0x10b770 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10b774 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10b778 | load | 1 | `[sp-32] +0x38` | `-` |
| `strb` at 0x10b780 | store | 1 | `[sp-32] +0x38` | `([[sp-32]+56]:1+1 & 0xff)` |
| `ldm` at 0x10b788 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b788 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b788 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10b6a8 | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0xb5` | `0x0` |
| `bl` at 0x10b704 | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0xbd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10b684 | `(r1 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r1 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |
| `bls` at 0x10b6dc | `[[sp-32]+56]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-32]+56]:1 > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b788 | `r0` | `?` |

## `extIntRemove`

- `extIntRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b7a0 to 0x10b838.
  Established. A byte store saves (entry r0 & 0xff) at sp-0x11, and a byte load reads absolute 0x31473c (`strb` at 0x10b7b4, `ldrb` at 0x10b7c0).
  The sole call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 9/0x18ca64/0xd7/0 (`bl` at 0x10b7ec).
  A later byte store writes 0 at [sp-28] +0x38, leaving the hardware meaning of the cleared field Unknown (`strb` at 0x10b81c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10b7b4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10b7b8 | load | 4 | `abs 0x10b828` | `-` |
| `ldrb` at 0x10b7bc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10b7c0 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b7cc | load | 4 | `abs 0x10b82c` | `-` |
| `str` at 0x10b7d0 | store | 4 | `sp -0x24` | `0x18cb10` |
| `ldrb` at 0x10b7d4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10b7d8 | store | 4 | `sp -0x20` | `(r0 & 0xff)` |
| `ldr` at 0x10b7e0 | load | 4 | `abs 0x10b830` | `-` |
| `ldrb` at 0x10b7f0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10b804 | load | 4 | `abs 0x10b834` | `-` |
| `ldr` at 0x10b808 | load | 4 | `abs 0x314738` | `-` |
| `str` at 0x10b810 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x10b814 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x10b81c | store | 1 | `[sp-28] +0x38` | `0x0` |
| `ldm` at 0x10b824 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b824 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b824 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10b7ec | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0xd7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10b7c8 | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b824 | `r0` | `?` |

## `extIntSetPollTimeout`

- `extIntSetPollTimeout`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b838 to 0x10b8d4.
  Established. Word and byte stores save entry r1 and (entry r0 & 0xff) at sp-0x18 and sp-0x11 (`str` at 0x10b84c, `strb` at 0x10b850).
  An unsigned comparison branches to 0x10b88c when (entry r0 & 0xff) is less than [0x31473c]:1, falling through to a call to unnamed 0x22070 otherwise (`blo` at 0x10b864, `bl` at 0x10b888).
  The later word store writes an unknown value at [sp-32] +8, so neither a timeout value nor its physical control meaning is established (`str` at 0x10b8b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10b84c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x10b850 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10b854 | load | 4 | `abs 0x10b8c4` | `-` |
| `ldrb` at 0x10b858 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10b85c | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b868 | load | 4 | `abs 0x10b8c8` | `-` |
| `str` at 0x10b86c | store | 4 | `sp -0x28` | `0x18caa0` |
| `ldrb` at 0x10b870 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10b874 | store | 4 | `sp -0x24` | `(r0 & 0xff)` |
| `ldr` at 0x10b87c | load | 4 | `abs 0x10b8cc` | `-` |
| `ldrb` at 0x10b88c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10b8a0 | load | 4 | `abs 0x10b8d0` | `-` |
| `ldr` at 0x10b8a4 | load | 4 | `abs 0x314738` | `-` |
| `str` at 0x10b8ac | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x10b8b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10b8b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10b8b8 | store | 4 | `[sp-32] +0x8` | `?` |
| `ldm` at 0x10b8c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b8c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b8c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10b888 | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0xed` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10b864 | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b8c0 | `r0` | `?` |

## `extIntEnable`

- `extIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b8d4 to 0x10b9b0.
  Established. A byte store writes 1 at an unknown effective address (`strb` at 0x10b940).
  Word stores place 0x1000, 10, 10 and 9 at sp-0x28, sp-0x24, sp-0x20 and sp-0x1c (`str` at 0x10b95c, `str` at 0x10b964, `str` at 0x10b96c, `str` at 0x10b974).
  The sole call to unnamed 0x22540 has candidate r0/r1/r2/r3 values [sp-24]+16/[[sp-24]+12]/0x10becc/[sp-20] (`bl` at 0x10b988).
  The stored byte and constants have no established device-level interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10b8e8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x10b8ec | load | 4 | `abs 0x10b9a4` | `-` |
| `ldrb` at 0x10b8f0 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b8f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10b900 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10b914 | load | 4 | `abs 0x10b9a8` | `-` |
| `ldr` at 0x10b918 | load | 4 | `abs 0x314738` | `-` |
| `str` at 0x10b920 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x10b924 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10b928 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x10b938 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x10b940 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x10b944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10b94c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10b950 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10b954 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `str` at 0x10b95c | store | 4 | `sp -0x28` | `0x1000` |
| `str` at 0x10b964 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x10b96c | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x10b974 | store | 4 | `sp -0x1c` | `0x9` |
| `ldr` at 0x10b97c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x10b980 | load | 4 | `abs 0x10b9ac` | `-` |
| `ldr` at 0x10b984 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10b98c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10b994 | store | 4 | `sp -0x14` | `?` |
| `ldm` at 0x10b9a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b9a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b9a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10b988 | `(unknown callee) 0x22540` | `[sp-24]+16` | `[[sp-24]+12]` | `0x10becc` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x10b8fc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x10b930 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b9a0 | `r0` | `?` |

## `extIntMaxInts`

- `extIntMaxInts`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b9b0 to 0x10b9d0.
  Established. A literal load reads absolute 0x10b9cc and a byte load reads absolute 0x31473c (`ldr` at 0x10b9bc, `ldrb` at 0x10b9c0).
  No calls are emitted, and the recorded r0 return is [0x31473c]:1 (`ldm` at 0x10b9c8).
  What the loaded byte represents remains Unknown from this body alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x10b9bc | load | 4 | `abs 0x10b9cc` | `-` |
| `ldrb` at 0x10b9c0 | load | 1 | `abs 0x31473c` | `-` |
| `ldm` at 0x10b9c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b9c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b9c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b9c8 | `r0` | `[0x31473c]:1` |

## `extIntCount`

- `extIntCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10b9d0 to 0x10ba60.
  Established. A byte store saves (entry r0 & 0xff) at sp-0x11 (`strb` at 0x10b9e4).
  The unsigned comparison branches to 0x10ba20 when that masked entry value is less than [0x31473c]:1, while its fall-through reaches unnamed 0x22070 with candidate r2=0x114 (`blo` at 0x10b9f8, `bl` at 0x10ba1c).
  The later word load has unknown effective address and the recorded r0 return is unknown, leaving the source of the returned value unidentified (`ldr` at 0x10ba40, `ldm` at 0x10ba4c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10b9e4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10b9e8 | load | 4 | `abs 0x10ba50` | `-` |
| `ldrb` at 0x10b9ec | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10b9f0 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10b9fc | load | 4 | `abs 0x10ba54` | `-` |
| `str` at 0x10ba00 | store | 4 | `sp -0x1c` | `0x18cb54` |
| `ldrb` at 0x10ba04 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10ba08 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x10ba10 | load | 4 | `abs 0x10ba58` | `-` |
| `ldrb` at 0x10ba20 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10ba34 | load | 4 | `abs 0x10ba5c` | `-` |
| `ldr` at 0x10ba38 | load | 4 | `abs 0x314738` | `-` |
| `ldr` at 0x10ba40 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x10ba4c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10ba4c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10ba4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ba1c | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x114` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10b9f8 | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10ba4c | `r0` | `?` |

## `extIntName`

- `extIntName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10ba60 to 0x10baf4.
  Established. Word stores place 0x18cb90 at sp-0x1c and (entry r0 & 0xff) at sp-0x18 (`str` at 0x10ba90, `str` at 0x10ba98).
  The sole call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 9/0x18ca64/0x11f/0 (`bl` at 0x10baac).
  A subsequent word load reads absolute 0x314738, but the final computed word load has unknown effective address, so the loaded word remains Unknown (`ldr` at 0x10bac8, `ldr` at 0x10bad0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10ba74 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10ba78 | load | 4 | `abs 0x10bae0` | `-` |
| `ldrb` at 0x10ba7c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10ba80 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10ba8c | load | 4 | `abs 0x10bae4` | `-` |
| `str` at 0x10ba90 | store | 4 | `sp -0x1c` | `0x18cb90` |
| `ldrb` at 0x10ba94 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10ba98 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x10baa0 | load | 4 | `abs 0x10bae8` | `-` |
| `ldr` at 0x10baa4 | load | 4 | `abs 0x10baec` | `-` |
| `ldrb` at 0x10bab0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10bac4 | load | 4 | `abs 0x10baf0` | `-` |
| `ldr` at 0x10bac8 | load | 4 | `abs 0x314738` | `-` |
| `ldr` at 0x10bad0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x10badc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10badc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10badc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10baac | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x11f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10ba88 | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10badc | `r0` | `?` |

## `extIntIRQ`

- `extIntIRQ`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10baf4 to 0x10bb88.
  Established. Byte loads read absolute 0x31473c and an unknown effective address (`ldrb` at 0x10bb14, `ldrb` at 0x10bb64).
  The call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 9/0x18ca64/0x12a/0 (`bl` at 0x10bb40).
  The recorded r0 return is unknown, and no physical interrupt number is established by these facts (`ldm` at 0x10bb70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10bb08 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10bb0c | load | 4 | `abs 0x10bb74` | `-` |
| `ldrb` at 0x10bb10 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10bb14 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10bb20 | load | 4 | `abs 0x10bb78` | `-` |
| `str` at 0x10bb24 | store | 4 | `sp -0x1c` | `0x18cbcc` |
| `ldrb` at 0x10bb28 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10bb2c | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x10bb34 | load | 4 | `abs 0x10bb7c` | `-` |
| `ldr` at 0x10bb38 | load | 4 | `abs 0x10bb80` | `-` |
| `ldrb` at 0x10bb44 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10bb58 | load | 4 | `abs 0x10bb84` | `-` |
| `ldr` at 0x10bb5c | load | 4 | `abs 0x314738` | `-` |
| `ldrb` at 0x10bb64 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x10bb70 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10bb70 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10bb70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10bb40 | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x12a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10bb1c | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10bb70 | `r0` | `?` |

## `extIntIsPolling`

- `extIntIsPolling`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10bb88 to 0x10bc18.
  Established. The unsigned comparison takes the branch to 0x10bbd8 when (entry r0 & 0xff) is less than [0x31473c]:1 (`blo` at 0x10bbb0);
  its other path reaches a call to unnamed 0x22070 with candidate r0/r1/r2/r3 values 9/0x18ca64/0x134/0 (`bl` at 0x10bbd4).
  The later byte load has unknown effective address and the r0 return is unknown, leaving the meaning of the loaded byte Unknown (`ldrb` at 0x10bbf8, `ldm` at 0x10bc04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x10bb9c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10bba0 | load | 4 | `abs 0x10bc08` | `-` |
| `ldrb` at 0x10bba4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10bba8 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10bbb4 | load | 4 | `abs 0x10bc0c` | `-` |
| `str` at 0x10bbb8 | store | 4 | `sp -0x1c` | `0x18cc04` |
| `ldrb` at 0x10bbbc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10bbc0 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0x10bbc8 | load | 4 | `abs 0x10bc10` | `-` |
| `ldrb` at 0x10bbd8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10bbec | load | 4 | `abs 0x10bc14` | `-` |
| `ldr` at 0x10bbf0 | load | 4 | `abs 0x314738` | `-` |
| `ldrb` at 0x10bbf8 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0x10bc04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10bc04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10bc04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10bbd4 | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x134` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10bbb0 | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10bc04 | `r0` | `?` |

## `extIntPollingMask`

- `extIntPollingMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10bc18 to 0x10bcac.
  Established. Word stores initialize sp-0x14 and sp-0x18 to 0 (`str` at 0x10bc2c, `str` at 0x10bc34).
  A byte load reads absolute 0x31473c, while the later computed byte load has unknown effective address (`ldrb` at 0x10bc3c, `ldrb` at 0x10bc6c).
  A backward transfer returns to 0x10bc38, and the later word store at sp-0x14 has unknown value (`b` at 0x10bc90, `str` at 0x10bc80).
  No calls are emitted, and the recorded unknown r0 return does not establish a physical polling mask (`ldm` at 0x10bca0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10bc2c | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x10bc34 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x10bc38 | load | 4 | `abs 0x10bca4` | `-` |
| `ldrb` at 0x10bc3c | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10bc40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10bc4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10bc60 | load | 4 | `abs 0x10bca8` | `-` |
| `ldr` at 0x10bc64 | load | 4 | `abs 0x314738` | `-` |
| `ldrb` at 0x10bc6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x10bc70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10bc78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10bc80 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x10bc84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x10bc8c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x10bc94 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10bca0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10bca0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10bca0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x10bc48 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10bca0 | `r0` | `?` |

## `extIntIgnoreSpurious`

- `extIntIgnoreSpurious`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10bcac to 0x10bd70.
  Established. A word store saves entry r1 at sp-0x18, and a byte load reads absolute 0x31473c (`str` at 0x10bcc0, `ldrb` at 0x10bcd0).
  The sole call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 9/0x18ca64/0x14d/0 (`bl` at 0x10bcfc).
  The later byte load and byte store have unknown effective addresses, and the store value is unknown, so the meaning of the modified software field remains Unknown (`ldrb` at 0x10bd44, `strb` at 0x10bd50).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10bcc0 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x10bcc4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10bcc8 | load | 4 | `abs 0x10bd5c` | `-` |
| `ldrb` at 0x10bccc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10bcd0 | load | 1 | `abs 0x31473c` | `-` |
| `ldr` at 0x10bcdc | load | 4 | `abs 0x10bd60` | `-` |
| `str` at 0x10bce0 | store | 4 | `sp -0x20` | `0x18cc40` |
| `ldrb` at 0x10bce4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x10bce8 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x10bcf0 | load | 4 | `abs 0x10bd64` | `-` |
| `ldr` at 0x10bcf4 | load | 4 | `abs 0x10bd68` | `-` |
| `ldrb` at 0x10bd00 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10bd14 | load | 4 | `abs 0x10bd6c` | `-` |
| `ldr` at 0x10bd18 | load | 4 | `abs 0x314738` | `-` |
| `ldrb` at 0x10bd20 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10bd34 | load | 4 | `abs 0x10bd6c` | `-` |
| `ldr` at 0x10bd38 | load | 4 | `abs 0x314738` | `-` |
| `ldr` at 0x10bd40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10bd44 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x10bd50 | store | 1 | `(unknown)` | `?` |
| `ldm` at 0x10bd58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10bd58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10bd58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10bcfc | `(unknown callee) 0x22070` | `0x9` | `0x18ca64` | `0x14d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10bcd8 | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10bd58 | `r0` | `?` |

## `extIntQuery`

- `extIntQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10bd70 to 0x10becc.
  Established. Byte and word loads read [sp-32] +5, +0x40 and +0xc (`ldrb` at 0x10bdf4, `ldr` at 0x10bdfc, `ldr` at 0x10be0c).
  One call to unnamed 0x205bc has candidate r0/r1/r2/r3 values 0x314698/0x18ccb0/[[sp-32]+12]/[[sp-32]+5]:1 (`bl` at 0x10be14).
  Another uses candidate r0=[sp-28]+ret(0x2031c), r1=0x18ccd0, r2=[sp-40] and r3=[[sp-32]+57]:1, and the recorded r0 return is 0x314698 (`bl` at 0x10be94, `ldm` at 0x10bea8).
  The query's field meanings and hardware correspondence remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10bd84 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x10bd88 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x10bd8c | load | 4 | `abs 0x10beac` | `-` |
| `str` at 0x10bd90 | store | 4 | `sp -0x1c` | `0x314698` |
| `ldr` at 0x10bd94 | load | 4 | `abs 0x10beb0` | `-` |
| `ldrb` at 0x10bd98 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x10bd9c | load | 1 | `abs 0x31473c` | `-` |
| `ldrb` at 0x10bda8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10bdac | load | 4 | `abs 0x10beac` | `-` |
| `ldr` at 0x10bdb0 | load | 4 | `abs 0x10beb4` | `-` |
| `ldr` at 0x10bdbc | load | 4 | `abs 0x10beac` | `-` |
| `str` at 0x10bdc0 | store | 4 | `sp -0x24` | `0x314698` |
| `ldrb` at 0x10bdc8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x10bddc | load | 4 | `abs 0x10beb8` | `-` |
| `ldr` at 0x10bde0 | load | 4 | `abs 0x314738` | `-` |
| `str` at 0x10bde8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x10bdec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10bdf0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10bdf4 | load | 1 | `[sp-32] +0x5` | `-` |
| `ldr` at 0x10bdf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10bdfc | load | 4 | `[sp-32] +0x40` | `-` |
| `str` at 0x10be00 | store | 4 | `sp -0x30` | `[[sp-32]+64]` |
| `ldr` at 0x10be04 | load | 4 | `abs 0x10beac` | `-` |
| `ldr` at 0x10be08 | load | 4 | `abs 0x10bebc` | `-` |
| `ldr` at 0x10be0c | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0x10be18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10be24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10be34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10be3c | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x2031c))` |
| `ldr` at 0x10be40 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10be44 | load | 1 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x10be50 | load | 4 | `abs 0x10bec0` | `-` |
| `str` at 0x10be54 | store | 4 | `sp -0x28` | `0x18ccc8` |
| `ldr` at 0x10be5c | load | 4 | `abs 0x10bec4` | `-` |
| `str` at 0x10be60 | store | 4 | `sp -0x28` | `0x18cccc` |
| `ldr` at 0x10be64 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10be68 | load | 1 | `[sp-32] +0x39` | `-` |
| `ldr` at 0x10be6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10be70 | load | 1 | `[sp-32] +0x3a` | `-` |
| `str` at 0x10be74 | store | 4 | `sp -0x30` | `[[sp-32]+58]:1` |
| `ldr` at 0x10be78 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x10be7c | load | 1 | `[sp-32] +0x3c` | `-` |
| `str` at 0x10be80 | store | 4 | `sp -0x2c` | `[[sp-32]+60]:1` |
| `ldr` at 0x10be84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10be88 | load | 4 | `abs 0x10bec8` | `-` |
| `ldr` at 0x10be8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x10be98 | load | 4 | `abs 0x10beac` | `-` |
| `str` at 0x10be9c | store | 4 | `sp -0x24` | `0x314698` |
| `ldr` at 0x10bea0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x10bea8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10bea8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10bea8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10bdb8 | `(unknown callee) 0x205bc` | `0x314698` | `0x18cc90` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x10be14 | `(unknown callee) 0x205bc` | `0x314698` | `0x18ccb0` | `[[sp-32]+12]` | `[[sp-32]+5]:1` |
| `bl` at 0x10be28 | `(unknown callee) 0x2031c` | `[sp-28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x10be94 | `(unknown callee) 0x205bc` | `([sp-28] + ret(0x2031c))` | `0x18ccd0` | `[sp-40]` | `[[sp-32]+57]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x10bda4 | `(r0 & 0xff) < [0x31473c]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x31473c]:1 (unsigned, 32-bit)` |
| `beq` at 0x10be20 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x10be4c | `[[sp-32]+4]:1 == 0x0` | `[[sp-32]+4]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10bea8 | `r0` | `0x314698` |

## `i2cInitialize`

- `i2cInitialize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59920 to 0x59a1c.
  Established. The call to unnamed 0x8767c has candidate r0/r1 values 0/0 (`bl` at 0x5993c).
  The next call to unnamed 0x59cdc supplies the result tested by a zero-result branch to 0x599b0 (`bl` at 0x59940, `beq` at 0x5994c).
  In the second loop, equality between ret(0x6e190) and [sp-20] takes the path to 0x599f8, while the other path tests ret(0x3ee14) (`beq` at 0x599dc, `bl` at 0x599e4).
  A call to unnamed 0x598a8 has candidate r0/r1 values [sp-20]/0, and the recorded r0 return is ret(0x70560) (`bl` at 0x59a00, `ldm` at 0x59a18).
  The body does not establish I2C register effects or physical bus identities.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59954 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x59960 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59980 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59998 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x599a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x599a8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x599b4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x599c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x599d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x599e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x599f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59a04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59a0c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0x59a18 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x59a18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x59a18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x59a18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59930 | `(unknown callee) 0x123480` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x5993c | `(unknown callee) 0x8767c` | `0x0` | `0x0` | `?` | `?` |
| `bl` at 0x59940 | `(unknown callee) 0x59cdc` | `ret(0x8767c)` | `?` | `?` | `?` |
| `bl` at 0x59958 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x5996c | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `ret(0x6e140)` | `[sp-20]` |
| `bl` at 0x59984 | `(unknown callee) 0x3ee14` | `[sp-20]` | `?` | `ret(0x6e190)` | `[sp-20]` |
| `bl` at 0x5999c | `(unknown callee) 0x125150` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x599b8 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x599cc | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `ret(0x6e140)` | `[sp-20]` |
| `bl` at 0x599e4 | `(unknown callee) 0x3ee14` | `[sp-20]` | `?` | `ret(0x6e190)` | `[sp-20]` |
| `bl` at 0x59a00 | `(unknown callee) 0x598a8` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x59a14 | `(unknown callee) 0x70560` | `ret(0x6e140)` | `?` | `ret(0x6e140)` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5994c | `ret(0x59cdc) == 0x0` | `ret(0x59cdc) != 0x0` |
| `ble` at 0x59968 | `ret(0x6e140) <= [sp-20] (signed, 32-bit)` | `ret(0x6e140) > [sp-20] (signed, 32-bit)` |
| `beq` at 0x5997c | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |
| `bne` at 0x59990 | `ret(0x3ee14) != 0x0` | `ret(0x3ee14) == 0x0` |
| `ble` at 0x599c8 | `ret(0x6e140) <= [sp-20] (signed, 32-bit)` | `ret(0x6e140) > [sp-20] (signed, 32-bit)` |
| `beq` at 0x599dc | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |
| `bne` at 0x599f0 | `ret(0x3ee14) != 0x0` | `ret(0x3ee14) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x59a18 | `r0` | `ret(0x70560)` |
