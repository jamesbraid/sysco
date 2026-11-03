# ARM system messages and console paths in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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
Computed transfers leave unresolved edges in the brick-type check routine, identified below. Tables cover known control-flow edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/system-message-console-facts.txt) retain all tool sections.

## `SMPLock`

- `SMPLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c230 to 0x5c2c4.
  Established. A word store records `ret(0x22184)` at `0x132658` (`str` at 0x5c290).
  A nonzero `[0x299968]` takes `0x5c288`, while zero falls through (`bne` at 0x5c278).
  The unnamed `0xbda54` call carries `[0x132658]`, `0x13268c`, `0x132658`, `0x132658` (`bl` at 0x5c2a0).
  Message-lock ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c240 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5c244 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5c250 | store | 4 | `sp -0x18` | `ret(0x22184)` |
| `ldr` at 0x5c254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5c264 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x5c26c | load | 4 | `abs 0x5c2b8` | `-` |
| `ldr` at 0x5c270 | load | 4 | `abs 0x299968` | `-` |
| `ldr` at 0x5c27c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5c280 | store | 4 | `sp -0x1c` | `ret(0x22184)` |
| `ldr` at 0x5c288 | load | 4 | `abs 0x5c2bc` | `-` |
| `ldr` at 0x5c28c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5c290 | store | 4 | `abs 0x132658` | `ret(0x22184)` |
| `ldr` at 0x5c294 | load | 4 | `abs 0x5c2bc` | `-` |
| `ldr` at 0x5c298 | load | 4 | `abs 0x132658` | `-` |
| `ldr` at 0x5c29c | load | 4 | `abs 0x5c2c0` | `-` |
| `ldr` at 0x5c2a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5c2a8 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x5c2ac | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x5c2b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5c2b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5c2b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c248 | `(unknown callee) 0x22184` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x5c2a0 | `(unknown callee) 0xbda54` | `[0x132658]` | `0x13268c` | `0x132658` | `0x132658` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5c25c | `ret(0x22184) != 0x0` | `ret(0x22184) == 0x0` |
| `bne` at 0x5c278 | `[0x299968] != 0x0` | `[0x299968] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5c2b4 | `r0` | `?` |

## `SMPUnlock`

- `SMPUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c2c4 to 0x5c310.
  Established. A nonzero `[0x132658]` takes `0x5c2e4`, while zero falls through (`bne` at 0x5c2dc).
  A word store clears `0x132658` (`str` at 0x5c2fc).
  The unnamed `0xbda94` call carries `[0x132658]`, `0x13268c`, entry `r2`, `0x132658` (`bl` at 0x5c2f0).
  Message-lock release policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5c2d0 | load | 4 | `abs 0x5c308` | `-` |
| `ldr` at 0x5c2d4 | load | 4 | `abs 0x132658` | `-` |
| `ldr` at 0x5c2e4 | load | 4 | `abs 0x5c308` | `-` |
| `ldr` at 0x5c2e8 | load | 4 | `abs 0x132658` | `-` |
| `ldr` at 0x5c2ec | load | 4 | `abs 0x5c30c` | `-` |
| `ldr` at 0x5c2f4 | load | 4 | `abs 0x5c308` | `-` |
| `str` at 0x5c2fc | store | 4 | `abs 0x132658` | `0x0` |
| `ldm` at 0x5c304 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5c304 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5c304 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c2f0 | `(unknown callee) 0xbda94` | `[0x132658]` | `0x13268c` | `r2` | `0x132658` |
| `bl` at 0x5c300 | `(unknown callee) 0x221d4` | `ret(0xbda94)` | `?` | `0x132658` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5c2dc | `[0x132658] != 0x0` | `[0x132658] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5c304 | `r0` | `?` |

## `SMPInit`

- `SMPInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5bd54 to 0x5c230.
  Established. Included word stores increment `0x132650` with `[0x132650]+1` and write `0x3` to `0x299bec` (`str` at 0x5bda4, `str` at 0x5bdbc).
  The unnamed `0xa89d0` call carries `0x29995c`, `0x2`, `0x0`, unknown (`bl` at 0x5bf80).
  Message service startup state meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5bd64 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x5bd68 | load | 4 | `abs 0x5c198` | `-` |
| `ldr` at 0x5bd6c | load | 4 | `abs 0x132650` | `-` |
| `ldr` at 0x5bd78 | load | 4 | `abs 0x5c19c` | `-` |
| `str` at 0x5bd7c | store | 4 | `sp -0x40` | `0xfb54c` |
| `ldr` at 0x5bd84 | load | 4 | `abs 0x5c1a0` | `-` |
| `ldr` at 0x5bd94 | load | 4 | `abs 0x5c198` | `-` |
| `ldr` at 0x5bd98 | load | 4 | `abs 0x5c198` | `-` |
| `ldr` at 0x5bd9c | load | 4 | `abs 0x132650` | `-` |
| `str` at 0x5bda4 | store | 4 | `abs 0x132650` | `[0x132650]+1` |
| `ldr` at 0x5bda8 | load | 4 | `abs 0x5c1a4` | `-` |
| `str` at 0x5bdb0 | store | 4 | `abs 0x299be8` | `0x0` |
| `ldr` at 0x5bdb4 | load | 4 | `abs 0x5c1a4` | `-` |
| `str` at 0x5bdbc | store | 4 | `abs 0x299bec` | `0x3` |
| `str` at 0x5bdc4 | store | 4 | `sp -0x24` | `0x14` |
| `ldr` at 0x5bdc8 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5bdcc | load | 4 | `abs 0x5c1a8` | `-` |
| `str` at 0x5bdd0 | store | 4 | `abs 0x299c40` | `0x299bf0` |
| `ldr` at 0x5bdd4 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5bdd8 | load | 4 | `abs 0x299c40` | `-` |
| `strb` at 0x5bde0 | store | 1 | `[0x299c40] +0x0` | `0x0` |
| `ldr` at 0x5bde4 | load | 4 | `abs 0x5c1ac` | `-` |
| `str` at 0x5bdf4 | store | 4 | `abs 0x132688` | `?` |
| `ldr` at 0x5bdf8 | load | 4 | `abs 0x5c1a4` | `-` |
| `str` at 0x5be00 | store | 4 | `abs 0x29a2a4` | `0x0` |
| `ldr` at 0x5be04 | load | 4 | `abs 0x5c1a4` | `-` |
| `str` at 0x5be0c | store | 4 | `abs 0x29a2a8` | `0x0` |
| `ldr` at 0x5be10 | load | 4 | `abs 0x5c1b0` | `-` |
| `ldr` at 0x5be14 | load | 4 | `abs 0x5c1b4` | `-` |
| `str` at 0x5be24 | store | 4 | `sp -0x1c` | `ret(0xa8af0)` |
| `ldr` at 0x5be28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5be34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5be40 | load | 4 | `abs 0x5c1b8` | `-` |
| `str` at 0x5be44 | store | 4 | `sp -0x40` | `0xfb58c` |
| `str` at 0x5be48 | store | 4 | `sp -0x3c` | `ret(0x1f8e0)` |
| `ldr` at 0x5be50 | load | 4 | `abs 0x5c1a0` | `-` |
| `ldr` at 0x5be60 | load | 4 | `abs 0x5c1bc` | `-` |
| `ldr` at 0x5be64 | load | 4 | `abs 0x5c1c0` | `-` |
| `str` at 0x5be74 | store | 4 | `sp -0x1c` | `ret(0xa8af0)` |
| `ldr` at 0x5be78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5be84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5be90 | load | 4 | `abs 0x5c1c4` | `-` |
| `str` at 0x5be94 | store | 4 | `sp -0x40` | `0xfb5b8` |
| `str` at 0x5be98 | store | 4 | `sp -0x3c` | `ret(0x1f8e0)` |
| `ldr` at 0x5bea0 | load | 4 | `abs 0x5c1a0` | `-` |
| `ldr` at 0x5beb0 | load | 4 | `abs 0x5c1c8` | `-` |
| `ldr` at 0x5beb4 | load | 4 | `abs 0x5c1cc` | `-` |
| `str` at 0x5bec4 | store | 4 | `sp -0x1c` | `ret(0xa8af0)` |
| `ldr` at 0x5bec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5bed4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5bee0 | load | 4 | `abs 0x5c1d0` | `-` |
| `str` at 0x5bee4 | store | 4 | `sp -0x40` | `0xfb5e8` |
| `str` at 0x5bee8 | store | 4 | `sp -0x3c` | `ret(0x1f8e0)` |
| `ldr` at 0x5bef0 | load | 4 | `abs 0x5c1a0` | `-` |
| `ldr` at 0x5bf00 | load | 4 | `abs 0x5c1d4` | `-` |
| `ldr` at 0x5bf04 | load | 4 | `abs 0x5c1d8` | `-` |
| `ldr` at 0x5bf14 | load | 4 | `abs 0x5c1dc` | `-` |
| `ldr` at 0x5bf18 | load | 4 | `abs 0x5c1e0` | `-` |
| `str` at 0x5bf2c | store | 4 | `sp -0x40` | `0x100` |
| `str` at 0x5bf34 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x5bf3c | store | 4 | `sp -0x38` | `0x1` |
| `str` at 0x5bf44 | store | 4 | `sp -0x34` | `0x10` |
| `str` at 0x5bf4c | store | 4 | `sp -0x30` | `0x410` |
| `ldr` at 0x5bf50 | load | 4 | `abs 0x5c1e4` | `-` |
| `str` at 0x5bf54 | store | 4 | `sp -0x2c` | `0x5e2b8` |
| `str` at 0x5bf5c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5bf60 | load | 4 | `abs 0x5c1e8` | `-` |
| `ldr` at 0x5bf64 | load | 4 | `abs 0x5c1ec` | `-` |
| `ldr` at 0x5bf6c | load | 4 | `abs 0x5c1f0` | `-` |
| `ldr` at 0x5bf74 | load | 4 | `abs 0x5c1a4` | `-` |
| `str` at 0x5bf90 | store | 4 | `sp -0x20` | `ret(0x6a0f8)` |
| `ldr` at 0x5bf94 | load | 4 | `abs 0x5c1f4` | `-` |
| `ldr` at 0x5bf98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5bf9c | load | 4 | `ret(0x6a0f8) +0x8` | `-` |
| `str` at 0x5bfa0 | store | 4 | `abs 0x132714` | `[ret(0x6a0f8)+8]` |
| `ldr` at 0x5bfa4 | load | 4 | `abs 0x5c1f8` | `-` |
| `ldr` at 0x5bfa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5bfac | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0x5bfb0 | store | 4 | `abs 0x132718` | `[[sp-32]+12]` |
| `ldr` at 0x5bfb4 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5bfb8 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5bfbc | load | 4 | `abs 0x29995c` | `-` |
| `str` at 0x5bfd4 | store | 4 | `abs 0x29a2ac` | `?` |
| `ldr` at 0x5bfd8 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5bfdc | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5bfe0 | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5bfe8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5bfec | load | 4 | `[sp-32] +0x14` | `-` |
| `str` at 0x5bffc | store | 4 | `abs 0x29a2ac` | `?` |
| `ldr` at 0x5c000 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5c004 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5c008 | load | 4 | `abs 0x29995c` | `-` |
| `str` at 0x5c020 | store | 4 | `abs 0x29a2b4` | `?` |
| `ldr` at 0x5c028 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5c038 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x5c03c | load | 4 | `abs 0x5c1fc` | `-` |
| `str` at 0x5c040 | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x5c048 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x5c050 | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x5c058 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x5c05c | load | 4 | `abs 0x5c200` | `-` |
| `ldr` at 0x5c060 | load | 4 | `abs 0x5c204` | `-` |
| `ldr` at 0x5c064 | load | 4 | `abs 0x5c208` | `-` |
| `str` at 0x5c074 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x5c078 | load | 4 | `abs 0x5c1fc` | `-` |
| `str` at 0x5c07c | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x5c084 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x5c08c | store | 4 | `sp -0x34` | `0x28` |
| `str` at 0x5c094 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x5c098 | load | 4 | `abs 0x5c20c` | `-` |
| `ldr` at 0x5c09c | load | 4 | `abs 0x5c210` | `-` |
| `ldr` at 0x5c0a0 | load | 4 | `abs 0x5c214` | `-` |
| `str` at 0x5c0b0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x5c0b4 | load | 4 | `abs 0x5c1fc` | `-` |
| `str` at 0x5c0b8 | store | 4 | `sp -0x3c` | `0xc18` |
| `str` at 0x5c0c0 | store | 4 | `sp -0x38` | `0x14` |
| `str` at 0x5c0c8 | store | 4 | `sp -0x34` | `0xa` |
| `str` at 0x5c0d0 | store | 4 | `sp -0x30` | `0x9` |
| `ldr` at 0x5c0d4 | load | 4 | `abs 0x5c218` | `-` |
| `ldr` at 0x5c0d8 | load | 4 | `abs 0x5c21c` | `-` |
| `ldr` at 0x5c0dc | load | 4 | `abs 0x5c220` | `-` |
| `ldr` at 0x5c0e8 | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5c0ec | load | 4 | `abs 0x5c224` | `-` |
| `ldr` at 0x5c0f4 | load | 4 | `abs 0x5c1c8` | `-` |
| `str` at 0x5c100 | store | 4 | `sp -0x1c` | `ret(0xa76a4)` |
| `ldr` at 0x5c104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c110 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c11c | load | 4 | `abs 0x5c228` | `-` |
| `str` at 0x5c120 | store | 4 | `sp -0x40` | `0xfb648` |
| `str` at 0x5c124 | store | 4 | `sp -0x3c` | `ret(0x1f8e0)` |
| `ldr` at 0x5c12c | load | 4 | `abs 0x5c1a0` | `-` |
| `ldr` at 0x5c13c | load | 4 | `abs 0x5c1a4` | `-` |
| `ldr` at 0x5c140 | load | 4 | `abs 0x5c224` | `-` |
| `ldr` at 0x5c148 | load | 4 | `abs 0x5c1b0` | `-` |
| `str` at 0x5c154 | store | 4 | `sp -0x1c` | `ret(0xa76a4)` |
| `ldr` at 0x5c158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c170 | load | 4 | `abs 0x5c22c` | `-` |
| `str` at 0x5c174 | store | 4 | `sp -0x40` | `0xfb66c` |
| `str` at 0x5c178 | store | 4 | `sp -0x3c` | `ret(0x1f8e0)` |
| `ldr` at 0x5c180 | load | 4 | `abs 0x5c1a0` | `-` |
| `ldm` at 0x5c194 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5c194 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5c194 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5c194 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5bd90 | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x62` | `0x0` |
| `bl` at 0x5bde8 | `(unknown callee) 0x488cc` | `?` | `?` | `[0x299c40]` | `0x0` |
| `bl` at 0x5be1c | `(unknown callee) 0xa8af0` | `0x29996c` | `0xfb580` | `0x19` | `0x0` |
| `bl` at 0x5be38 | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0x5be5c | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x80` | `0x0` |
| `bl` at 0x5be6c | `(unknown callee) 0xa8af0` | `0x299a04` | `0xfb5ac` | `0x19` | `?` |
| `bl` at 0x5be88 | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0x5beac | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x83` | `0x0` |
| `bl` at 0x5bebc | `(unknown callee) 0xa8af0` | `0x299a9c` | `0xfb5dc` | `0x19` | `?` |
| `bl` at 0x5bed8 | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0x5befc | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x86` | `0x0` |
| `bl` at 0x5bf10 | `(unknown callee) 0x20bb4` | `0x299b34` | `0xfb60c` | `0x1` | `0x9` |
| `bl` at 0x5bf24 | `(unknown callee) 0x20bb4` | `0x299b88` | `0xfb618` | `0x1` | `0x9` |
| `bl` at 0x5bf70 | `(unknown callee) 0xadcc4` | `0x29a2b8` | `0x29a2f4` | `0x100` | `0x29a3f4` |
| `bl` at 0x5bf80 | `(unknown callee) 0xa89d0` | `0x29995c` | `0x2` | `0x0` | `?` |
| `bl` at 0x5bf88 | `(unknown callee) 0x6a0f8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x5c024 | `(unknown callee) 0x5e548` | `ret(0x6a0f8)` | `0x29995c` | `0x29995c` | `?` |
| `bl` at 0x5c06c | `(unknown callee) 0x2066c` | `0x299bdc` | `0xfb624` | `0x5c310` | `0x0` |
| `bl` at 0x5c0a8 | `(unknown callee) 0x2066c` | `0x299be0` | `0xfb630` | `0x5c810` | `0x0` |
| `bl` at 0x5c0e4 | `(unknown callee) 0x2066c` | `0x299be4` | `0xfb63c` | `0x5d518` | `0x0` |
| `bl` at 0x5c0f8 | `(unknown callee) 0xa76a4` | `0x29995c` | `0xffff002` | `0x1` | `0x299a9c` |
| `bl` at 0x5c114 | `(unknown callee) 0x1f8e0` | `ret(0xa76a4)` | `?` | `?` | `ret(0xa76a4)` |
| `bl` at 0x5c138 | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0xd0` | `0x0` |
| `bl` at 0x5c14c | `(unknown callee) 0xa76a4` | `0x29995c` | `0xffff002` | `0x2` | `0x29996c` |
| `bl` at 0x5c168 | `(unknown callee) 0x1f8e0` | `ret(0xa76a4)` | `?` | `?` | `ret(0xa76a4)` |
| `bl` at 0x5c18c | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0xdc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5bd74 | `[0x132650] == 0x0` | `[0x132650] != 0x0` |
| `beq` at 0x5be30 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `beq` at 0x5be80 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `beq` at 0x5bed0 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `bne` at 0x5c030 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x5c10c | `ret(0xa76a4) == 0x0` | `ret(0xa76a4) != 0x0` |
| `beq` at 0x5c160 | `ret(0xa76a4) == 0x0` | `ret(0xa76a4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5c194 | `r0` | `?` |

## `SMPLibrary`

- `SMPLibrary`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cc64 to 0x5d518.
  Established. A zero `ret(0x20cd0)` takes `0x5ccd8`, while nonzero falls through (`beq` at 0x5cca8).
  Included word stores write `0x51` to `0x29a4fc` and clear `0x29a4f8` (`str` at 0x5ccf4, `str` at 0x5cd18).
  The unnamed `0x2105c` call carries `0x299b88` and three unknown candidates (`bl` at 0x5d450).
  Message-library state meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cc74 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x5cc78 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x5cc80 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x5cc88 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5cc8c | load | 4 | `abs 0x5d4a8` | `-` |
| `str` at 0x5cc9c | store | 4 | `sp -0x28` | `ret(0x20cd0)` |
| `ldr` at 0x5cca0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ccac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ccb8 | load | 4 | `abs 0x5d4ac` | `-` |
| `str` at 0x5ccbc | store | 4 | `sp -0x90` | `0xfb7cc` |
| `str` at 0x5ccc0 | store | 4 | `sp -0x8c` | `ret(0x1f8e0)` |
| `ldr` at 0x5ccc8 | load | 4 | `abs 0x5d4b0` | `-` |
| `ldr` at 0x5cccc | load | 4 | `abs 0x5d4b4` | `-` |
| `ldr` at 0x5ccd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5cce4 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5cce8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5ccec | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5ccf4 | store | 4 | `abs 0x29a4fc` | `0x51` |
| `str` at 0x5cd00 | store | 4 | `[sp-32] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x5cd04 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x5cd08 | store | 4 | `abs 0x29a4f4` | `[[sp-32]]` |
| `ldr` at 0x5cd0c | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5cd10 | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5cd18 | store | 4 | `abs 0x29a4f8` | `0x0` |
| `ldr` at 0x5cd1c | load | 4 | `abs 0x29a4f4` | `-` |
| `str` at 0x5cd24 | store | 4 | `[0x29a4f4] +0x0` | `0x0` |
| `ldr` at 0x5cd2c | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5cd34 | store | 4 | `abs 0x29a4f4` | `0x0` |
| `ldr` at 0x5cd38 | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5cd40 | store | 4 | `abs 0x29a4f8` | `0x0` |
| `ldr` at 0x5cd44 | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5cd4c | store | 4 | `abs 0x29a4fc` | `0x0` |
| `ldr` at 0x5cd50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cd54 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5cd60 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5cd64 | load | 4 | `abs 0x299be8` | `-` |
| `ldr` at 0x5cd70 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5cd74 | load | 4 | `abs 0x5d4bc` | `-` |
| `ldr` at 0x5cd8c | load | 4 | `abs 0x5d4a8` | `-` |
| `str` at 0x5cd98 | store | 4 | `sp -0x28` | `ret(0x2105c)` |
| `ldr` at 0x5cd9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5cda8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5cdb4 | load | 4 | `abs 0x5d4c0` | `-` |
| `str` at 0x5cdb8 | store | 4 | `sp -0x90` | `0xfb7f4` |
| `str` at 0x5cdbc | store | 4 | `sp -0x8c` | `ret(0x1f8e0)` |
| `ldr` at 0x5cdc4 | load | 4 | `abs 0x5d4b0` | `-` |
| `ldr` at 0x5cdc8 | load | 4 | `abs 0x5d4c4` | `-` |
| `str` at 0x5cdd8 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x5cde0 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5cde4 | load | 4 | `abs 0x299964` | `-` |
| `ldr` at 0x5cde8 | load | 4 | `abs 0x5d4c8` | `-` |
| `ldr` at 0x5cdf0 | load | 4 | `abs 0x5d4c8` | `-` |
| `strb` at 0x5cdf8 | store | 1 | `abs 0x13270d` | `0x1` |
| `ldr` at 0x5cdfc | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5ce00 | load | 4 | `abs 0x299964` | `-` |
| `ldr` at 0x5ce04 | load | 4 | `abs 0x5d4c8` | `-` |
| `ldr` at 0x5ce0c | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5ce14 | store | 4 | `abs 0x299be8` | `0x1` |
| `ldr` at 0x5ce18 | load | 4 | `abs 0x5d4a8` | `-` |
| `str` at 0x5ce24 | store | 4 | `sp -0x28` | `ret(0x2105c)` |
| `ldr` at 0x5ce28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ce34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5ce40 | load | 4 | `abs 0x5d4c0` | `-` |
| `str` at 0x5ce44 | store | 4 | `sp -0x90` | `0xfb7f4` |
| `str` at 0x5ce48 | store | 4 | `sp -0x8c` | `ret(0x1f8e0)` |
| `ldr` at 0x5ce50 | load | 4 | `abs 0x5d4b0` | `-` |
| `ldr` at 0x5ce54 | load | 4 | `abs 0x5d4cc` | `-` |
| `str` at 0x5ce64 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x5ce6c | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5ce70 | load | 4 | `abs 0x299bec` | `-` |
| `str` at 0x5ce74 | store | 4 | `sp -0x88` | `[0x299bec]` |
| `ldr` at 0x5ce78 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5ce84 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5ce90 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5cea0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cea4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5ceb0 | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5ceb8 | store | 4 | `abs 0x299bec` | `0x2` |
| `ldr` at 0x5cebc | load | 4 | `abs 0x5d4d0` | `-` |
| `ldr` at 0x5cec4 | load | 4 | `abs 0x5d4d4` | `-` |
| `ldr` at 0x5cecc | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5ced0 | load | 4 | `abs 0x5d4d8` | `-` |
| `str` at 0x5ced4 | store | 4 | `abs 0x299c40` | `0x299bf0` |
| `ldr` at 0x5cedc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5cee8 | store | 4 | `sp -0x24` | `ret(0x5dd5c)` |
| `ldr` at 0x5cef0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cef4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5cf00 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf04 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf10 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf14 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf20 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf24 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf30 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf34 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf40 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf44 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf50 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf54 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf60 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf64 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf70 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf74 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf80 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf84 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cf90 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cf94 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cfa0 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cfa4 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cfb0 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cfb4 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cfc0 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cfc4 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cfd0 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cfd4 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cfe0 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cfe4 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5cff0 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5cff4 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5d000 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5d004 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5d010 | load | 4 | `abs 0x5d4dc` | `-` |
| `ldr` at 0x5d014 | load | 4 | `abs 0x132688` | `-` |
| `ldr` at 0x5d020 | load | 4 | `abs 0x5d4e0` | `-` |
| `ldr` at 0x5d03c | load | 4 | `abs 0x5d4e4` | `-` |
| `ldr` at 0x5d044 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d048 | load | 4 | `abs 0x299bec` | `-` |
| `ldr` at 0x5d054 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d058 | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5d06c | load | 4 | `abs 0x5d4e8` | `-` |
| `ldr` at 0x5d080 | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5d088 | store | 4 | `abs 0x299bec` | `0x1` |
| `ldr` at 0x5d090 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d094 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5d0a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d0a4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5d0b4 | load | 4 | `abs 0x5d4d8` | `-` |
| `ldr` at 0x5d0c8 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d0cc | load | 4 | `abs 0x29a2a4` | `-` |
| `ldr` at 0x5d0e0 | load | 4 | `abs 0x5d4ec` | `-` |
| `ldr` at 0x5d0ec | load | 4 | `abs 0x5d4d8` | `-` |
| `ldr` at 0x5d0f4 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d0f8 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d0fc | load | 4 | `abs 0x29a2a4` | `-` |
| `ldr` at 0x5d104 | load | 4 | `abs 0x5d4f0` | `-` |
| `str` at 0x5d12c | store | 4 | `abs 0x29a2a4` | `?` |
| `ldr` at 0x5d130 | load | 4 | `abs 0x5d4f4` | `-` |
| `ldr` at 0x5d134 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d138 | load | 4 | `abs 0x29a2a4` | `-` |
| `str` at 0x5d13c | store | 4 | `abs 0x132660` | `[0x29a2a4]` |
| `ldr` at 0x5d140 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d144 | load | 4 | `abs 0x29a2a8` | `-` |
| `ldr` at 0x5d150 | load | 4 | `abs 0x5d4f8` | `-` |
| `ldr` at 0x5d154 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d158 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d15c | load | 4 | `abs 0x29a2a8` | `-` |
| `str` at 0x5d164 | store | 4 | `abs 0x29a2a8` | `[0x29a2a8]+1` |
| `str` at 0x5d168 | store | 4 | `abs 0x132664` | `[0x29a2a8]+1` |
| `ldr` at 0x5d170 | load | 4 | `abs 0x5d4f8` | `-` |
| `str` at 0x5d178 | store | 4 | `abs 0x132664` | `0x14` |
| `ldr` at 0x5d17c | load | 4 | `abs 0x5d4fc` | `-` |
| `str` at 0x5d18c | store | 4 | `sp -0x24` | `ret(0x5de88)` |
| `ldr` at 0x5d190 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d194 | load | 4 | `abs 0x5d4d8` | `-` |
| `str` at 0x5d198 | store | 4 | `abs 0x299c40` | `0x299bf0` |
| `ldr` at 0x5d19c | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d1a0 | load | 4 | `abs 0x299c40` | `-` |
| `strb` at 0x5d1a8 | store | 1 | `[0x299c40] +0x0` | `0x0` |
| `ldr` at 0x5d1ac | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d1b0 | load | 4 | `abs 0x299bec` | `-` |
| `ldr` at 0x5d1cc | load | 4 | `abs 0x5d4e4` | `-` |
| `ldr` at 0x5d1d4 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d1d8 | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5d1ec | load | 4 | `abs 0x5d500` | `-` |
| `ldr` at 0x5d1f4 | load | 4 | `abs 0x5d4b8` | `-` |
| `str` at 0x5d1fc | store | 4 | `abs 0x299bec` | `0x1` |
| `ldr` at 0x5d210 | load | 4 | `abs 0x5d4d4` | `-` |
| `ldr` at 0x5d21c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d220 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5d22c | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d230 | load | 4 | `abs 0x299c40` | `-` |
| `ldr` at 0x5d234 | load | 4 | `abs 0x5d4d8` | `-` |
| `ldr` at 0x5d240 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d244 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d248 | load | 4 | `abs 0x299c40` | `-` |
| `str` at 0x5d254 | store | 4 | `abs 0x299c40` | `[0x299c40]-1` |
| `strb` at 0x5d25c | store | 1 | `[0x299c40] -0x1` | `0x0` |
| `ldr` at 0x5d260 | load | 4 | `abs 0x5d504` | `-` |
| `ldr` at 0x5d26c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d270 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5d27c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d280 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5d290 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d294 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x5d2a4 | store | 4 | `sp -0x7c` | `?` |
| `str` at 0x5d2ac | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x5d2b8 | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x5d2bc | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d2c0 | load | 4 | `abs 0x29a2a8` | `-` |
| `str` at 0x5d2c4 | store | 4 | `sp -0x80` | `[0x29a2a8]` |
| `ldr` at 0x5d2c8 | load | 4 | `abs 0x5d4f8` | `-` |
| `ldr` at 0x5d2cc | load | 4 | `abs 0x132664` | `-` |
| `ldr` at 0x5d2d0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x5d2e0 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d2e4 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d2e8 | load | 4 | `abs 0x299c40` | `-` |
| `str` at 0x5d2f4 | store | 4 | `abs 0x299c40` | `[0x299c40]-1` |
| `ldr` at 0x5d2f8 | load | 4 | `abs 0x5d508` | `-` |
| `ldr` at 0x5d304 | load | 4 | `abs 0x5d504` | `-` |
| `ldr` at 0x5d310 | load | 4 | `abs 0x5d4f4` | `-` |
| `ldr` at 0x5d314 | load | 4 | `abs 0x5d4f4` | `-` |
| `ldr` at 0x5d318 | load | 4 | `abs 0x132660` | `-` |
| `ldr` at 0x5d31c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x5d328 | load | 4 | `abs 0x5d4f0` | `-` |
| `str` at 0x5d350 | store | 4 | `abs 0x132660` | `?` |
| `ldr` at 0x5d354 | load | 4 | `abs 0x5d4f8` | `-` |
| `ldr` at 0x5d358 | load | 4 | `abs 0x5d4f8` | `-` |
| `ldr` at 0x5d35c | load | 4 | `abs 0x132664` | `-` |
| `ldr` at 0x5d360 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x5d368 | store | 4 | `abs 0x132664` | `([0x132664] + [sp-124])` |
| `ldr` at 0x5d36c | load | 4 | `abs 0x5d4f4` | `-` |
| `ldr` at 0x5d370 | load | 4 | `abs 0x132660` | `-` |
| `ldr` at 0x5d384 | load | 4 | `abs 0x5d4ec` | `-` |
| `ldr` at 0x5d38c | load | 4 | `abs 0x5d4d8` | `-` |
| `ldr` at 0x5d398 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d39c | load | 4 | `abs 0x5d4d8` | `-` |
| `ldr` at 0x5d3ac | load | 4 | `abs 0x5d4d8` | `-` |
| `str` at 0x5d3b4 | store | 4 | `abs 0x299c40` | `ret(0x19538)+0x299bf0` |
| `ldr` at 0x5d3b8 | load | 4 | `abs 0x5d4d8` | `-` |
| `ldr` at 0x5d3c4 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d3c8 | load | 4 | `abs 0x299c40` | `-` |
| `ldr` at 0x5d3cc | load | 4 | `abs 0x5d50c` | `-` |
| `ldr` at 0x5d3d8 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d3e0 | load | 4 | `abs 0x299c40` | `-` |
| `ldr` at 0x5d3e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d3ec | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x5d3f0 | store | 1 | `[0x299c40] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x5d3f8 | store | 4 | `abs 0x299c40` | `[0x299c40]+1` |
| `ldr` at 0x5d3fc | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d400 | load | 4 | `abs 0x299c40` | `-` |
| `strb` at 0x5d408 | store | 1 | `[0x299c40] +0x0` | `0x0` |
| `ldr` at 0x5d410 | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d418 | load | 4 | `abs 0x299c40` | `-` |
| `ldr` at 0x5d420 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d424 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x5d428 | store | 1 | `(unknown)` | `([[sp-28]]:1 & 0xff)` |
| `ldr` at 0x5d42c | load | 4 | `abs 0x5d4b8` | `-` |
| `ldr` at 0x5d430 | load | 4 | `abs 0x299c40` | `-` |
| `strb` at 0x5d438 | store | 1 | `[0x299c40] +0x0` | `0x0` |
| `ldr` at 0x5d43c | load | 4 | `abs 0x5d510` | `-` |
| `ldr` at 0x5d444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d44c | load | 4 | `abs 0x5d4a8` | `-` |
| `str` at 0x5d458 | store | 4 | `sp -0x28` | `ret(0x2105c)` |
| `ldr` at 0x5d45c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d468 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d474 | load | 4 | `abs 0x5d4c0` | `-` |
| `str` at 0x5d478 | store | 4 | `sp -0x90` | `0xfb7f4` |
| `str` at 0x5d47c | store | 4 | `sp -0x8c` | `ret(0x1f8e0)` |
| `ldr` at 0x5d484 | load | 4 | `abs 0x5d4b0` | `-` |
| `ldr` at 0x5d488 | load | 4 | `abs 0x5d514` | `-` |
| `ldr` at 0x5d494 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d498 | store | 4 | `sp -0x84` | `[sp-36]` |
| `ldr` at 0x5d49c | load | 4 | `sp -0x84` | `-` |
| `ldm` at 0x5d4a4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x5d4a4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5d4a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5d4a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5d4a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cc94 | `(unknown callee) 0x20cd0` | `0x299b88` | `?` | `r2` | `0x0` |
| `bl` at 0x5ccb0 | `(unknown callee) 0x1f8e0` | `ret(0x20cd0)` | `?` | `?` | `ret(0x20cd0)` |
| `bl` at 0x5ccd4 | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x286` | `0x0` |
| `bl` at 0x5ccfc | `(unknown callee) 0x1943c` | `0x144` | `?` | `0x29995c` | `0x51` |
| `bl` at 0x5cd7c | `(unknown callee) 0x5e0e8` | `0x29995c` | `0xfb7f0` | `0x1` | `[0x299be8]` |
| `bl` at 0x5cd90 | `(unknown callee) 0x2105c` | `0x299b88` | `?` | `?` | `ret(0x5e0e8)` |
| `bl` at 0x5cdac | `(unknown callee) 0x1f8e0` | `ret(0x2105c)` | `?` | `?` | `ret(0x2105c)` |
| `bl` at 0x5cdd0 | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x29e` | `0x0` |
| `bl` at 0x5cdec | `(unknown callee) 0xbda54` | `[0x299964]` | `0x1326f8` | `?` | `0x29995c` |
| `bl` at 0x5ce08 | `(unknown callee) 0xbda94` | `[0x299964]` | `0x1326f8` | `0x1326f8` | `0x29995c` |
| `bl` at 0x5ce1c | `(unknown callee) 0x2105c` | `0x299b88` | `?` | `0x29995c` | `0x1` |
| `bl` at 0x5ce38 | `(unknown callee) 0x1f8e0` | `ret(0x2105c)` | `?` | `?` | `ret(0x2105c)` |
| `bl` at 0x5ce5c | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x2b3` | `0x0` |
| `bl` at 0x5cec0 | `(unknown callee) 0x5e31c` | `0xfb818` | `?` | `0x29995c` | `0x2` |
| `bl` at 0x5cec8 | `(unknown callee) 0x5e31c` | `0x299c44` | `?` | `?` | `?` |
| `bl` at 0x5cee0 | `(unknown callee) 0x5dd5c` | `[sp-28]` | `?` | `[0x299bec]` | `[[sp-28]]:1` |
| `bl` at 0x5d024 | `(unknown callee) 0x5e31c` | `0xfb83c` | `?` | `[0x299bec]` | `[0x132688]` |
| `bl` at 0x5d02c | `(unknown callee) 0x375e4` | `?` | `?` | `[0x299bec]` | `[0x132688]` |
| `bl` at 0x5d040 | `(unknown callee) 0x5e31c` | `0xfb860` | `?` | `?` | `ret(0x375e4)` |
| `bl` at 0x5d05c | `(unknown callee) 0x5f254` | `[0x29a2ac]` | `?` | `?` | `0x29995c` |
| `bl` at 0x5d070 | `(unknown callee) 0x197a8` | `sp-120` | `0xfb894` | `ret(0x5f254)` | `sp-120` |
| `bl` at 0x5d07c | `(unknown callee) 0x5e31c` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x5d0b8 | `(unknown callee) 0x19538` | `0x299bf0` | `?` | `[0x299bec]` | `[[sp-28]]:1` |
| `bl` at 0x5d0f0 | `(unknown callee) 0x19808` | `?` | `0x299bf0` | `?` | `?` |
| `bl` at 0x5d180 | `(unknown callee) 0x5e31c` | `0xfb8cc` | `?` | `?` | `?` |
| `bl` at 0x5d184 | `(unknown callee) 0x5de88` | `ret(0x5e31c)` | `?` | `?` | `?` |
| `bl` at 0x5d1bc | `(unknown callee) 0x375e4` | `ret(0x5de88)` | `?` | `[0x299c40]` | `[0x299bec]` |
| `bl` at 0x5d1d0 | `(unknown callee) 0x5e31c` | `0xfb860` | `?` | `?` | `ret(0x375e4)` |
| `bl` at 0x5d1dc | `(unknown callee) 0x5f254` | `[0x29a2ac]` | `?` | `?` | `0x29995c` |
| `bl` at 0x5d1f0 | `(unknown callee) 0x197a8` | `sp-120` | `0xfb8d0` | `ret(0x5f254)` | `sp-120` |
| `bl` at 0x5d208 | `(unknown callee) 0x5e31c` | `sp-120` | `?` | `0x29995c` | `sp-120` |
| `bl` at 0x5d214 | `(unknown callee) 0x5e31c` | `0x299c44` | `?` | `[0x299c40]` | `[0x299bec]` |
| `bl` at 0x5d264 | `(unknown callee) 0x5e31c` | `0xfb90c` | `0x29995c` | `[0x299c40]-1` | `0x0` |
| `bl` at 0x5d308 | `(unknown callee) 0x5e31c` | `0xfb90c` | `0x29995c` | `[0x299c40]-1` | `0x299bef` |
| `bl` at 0x5d394 | `(unknown callee) 0x19808` | `0x299bf0` | `?` | `?` | `?` |
| `bl` at 0x5d3a0 | `(unknown callee) 0x19538` | `0x299bf0` | `?` | `?` | `?` |
| `bl` at 0x5d3bc | `(unknown callee) 0x5e31c` | `0x299bf0` | `?` | `ret(0x19538)` | `ret(0x19538)+0x299bf0` |
| `bl` at 0x5d440 | `(unknown callee) 0x5e31c` | `0xfb910` | `?` | `[0x299c40]` | `0x0` |
| `bl` at 0x5d448 | `(unknown callee) 0x5e31c` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x5d450 | `(unknown callee) 0x2105c` | `0x299b88` | `?` | `?` | `?` |
| `bl` at 0x5d46c | `(unknown callee) 0x1f8e0` | `ret(0x2105c)` | `?` | `?` | `ret(0x2105c)` |
| `bl` at 0x5d490 | `(unknown callee) 0x200a0` | `0x9` | `0xfb56c` | `0x3a9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5cca8 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0x5cce0 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x5cd5c | `[[sp-28]]:1 != 0x18` | `[[sp-28]]:1 == 0x18` |
| `bne` at 0x5cd6c | `[0x299be8] != 0x0` | `[0x299be8] == 0x0` |
| `beq` at 0x5cd88 | `ret(0x5e0e8) == 0x1` | `ret(0x5e0e8) != 0x1` |
| `beq` at 0x5cda4 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `beq` at 0x5ce30 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `beq` at 0x5ce80 | `[0x299bec] == 0x1` | `[0x299bec] != 0x1` |
| `blo` at 0x5ce8c | `[0x299bec] < 0x1 (unsigned, 32-bit)` | `[0x299bec] >= 0x1 (unsigned, 32-bit)` |
| `bhi` at 0x5ce98 | `[0x299bec] > 0x3 (unsigned, 32-bit)` | `[0x299bec] <= 0x3 (unsigned, 32-bit)` |
| `bne` at 0x5ceac | `[[sp-28]]:1 != 0x14` | `[[sp-28]]:1 == 0x14` |
| `bne` at 0x5cefc | `[[sp-28]]:1 != 0x4` | `[[sp-28]]:1 == 0x4` |
| `beq` at 0x5cf0c | `[0x132688] == 0x43` | `[0x132688] != 0x43` |
| `beq` at 0x5cf1c | `[0x132688] == 0x33` | `[0x132688] != 0x33` |
| `beq` at 0x5cf2c | `[0x132688] == 0x35` | `[0x132688] != 0x35` |
| `beq` at 0x5cf3c | `[0x132688] == 0x34` | `[0x132688] != 0x34` |
| `beq` at 0x5cf4c | `[0x132688] == 0x41` | `[0x132688] != 0x41` |
| `beq` at 0x5cf5c | `[0x132688] == 0x36` | `[0x132688] != 0x36` |
| `beq` at 0x5cf6c | `[0x132688] == 0x39` | `[0x132688] != 0x39` |
| `beq` at 0x5cf7c | `[0x132688] == 0x37` | `[0x132688] != 0x37` |
| `beq` at 0x5cf8c | `[0x132688] == 0x38` | `[0x132688] != 0x38` |
| `beq` at 0x5cf9c | `[0x132688] == 0x30` | `[0x132688] != 0x30` |
| `beq` at 0x5cfac | `[0x132688] == 0x51` | `[0x132688] != 0x51` |
| `beq` at 0x5cfbc | `[0x132688] == 0x40` | `[0x132688] != 0x40` |
| `beq` at 0x5cfcc | `[0x132688] == 0x4f` | `[0x132688] != 0x4f` |
| `beq` at 0x5cfdc | `[0x132688] == 0x42` | `[0x132688] != 0x42` |
| `beq` at 0x5cfec | `[0x132688] == 0x46` | `[0x132688] != 0x46` |
| `beq` at 0x5cffc | `[0x132688] == 0x4c` | `[0x132688] != 0x4c` |
| `beq` at 0x5d00c | `[0x132688] == 0x4d` | `[0x132688] != 0x4d` |
| `beq` at 0x5d01c | `[0x132688] == 0x7c` | `[0x132688] != 0x7c` |
| `beq` at 0x5d038 | `ret(0x375e4) == 0x1` | `ret(0x375e4) != 0x1` |
| `bne` at 0x5d050 | `[0x299bec] != 0x3` | `[0x299bec] == 0x3` |
| `beq` at 0x5d09c | `[[sp-28]]:1 == 0xd` | `[[sp-28]]:1 != 0xd` |
| `beq` at 0x5d0ac | `[[sp-28]]:1 == 0xa` | `[[sp-28]]:1 != 0xa` |
| `beq` at 0x5d0c4 | `ret(0x19538) == 0x0` | `ret(0x19538) != 0x0` |
| `bgt` at 0x5d14c | `[0x29a2a8] > 0x13 (signed, 32-bit)` | `[0x29a2a8] <= 0x13 (signed, 32-bit)` |
| `bne` at 0x5d1b8 | `[0x299bec] != 0x2` | `[0x299bec] == 0x2` |
| `beq` at 0x5d1c8 | `ret(0x375e4) == 0x1` | `ret(0x375e4) != 0x1` |
| `bne` at 0x5d228 | `[[sp-28]]:1 != 0x8` | `[[sp-28]]:1 == 0x8` |
| `bls` at 0x5d23c | `[0x299c40] <= 0x299bf0 (unsigned, 32-bit)` | `[0x299c40] > 0x299bf0 (unsigned, 32-bit)` |
| `beq` at 0x5d278 | `[[sp-28]]:1 == 0x10` | `[[sp-28]]:1 != 0x10` |
| `beq` at 0x5d288 | `[[sp-28]]:1 == 0xe` | `[[sp-28]]:1 != 0xe` |
| `bne` at 0x5d29c | `[[sp-28]]:1 != 0x10` | `[[sp-28]]:1 == 0x10` |
| `bne` at 0x5d2d8 | `[0x132664] != [sp-128]` | `[0x132664] == [sp-128]` |
| `bls` at 0x5d300 | `[0x299c40]-1 <= 0x299bef (unsigned, 32-bit)` | `[0x299c40]-1 > 0x299bef (unsigned, 32-bit)` |
| `bhs` at 0x5d3d4 | `[0x299c40] >= 0x299c3f (unsigned, 32-bit)` | `[0x299c40] < 0x299c3f (unsigned, 32-bit)` |
| `beq` at 0x5d464 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5d4a4 | `r0` | `?` |

## `SMPConsoleSubchannelDefault`

- `SMPConsoleSubchannelDefault`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d728 to 0x5d778.
  Established. A zero `[0x12d780]` takes `0x5d768`, while nonzero falls through (`beq` at 0x5d748).
  A word assignment writes `0x1` to `0x12d780` (`str` at 0x5d764).
  The unnamed `0x5d778` call carries entry `r0`, entry `r1`, entry `r2`, `[0x12d780]` (`bl` at 0x5d750).
  Console default selection meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d738 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5d73c | load | 4 | `abs 0x5d774` | `-` |
| `ldr` at 0x5d740 | load | 4 | `abs 0x12d780` | `-` |
| `ldr` at 0x5d74c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d758 | store | 4 | `sp -0x14` | `ret(0x5d778)` |
| `ldr` at 0x5d75c | load | 4 | `abs 0x5d774` | `-` |
| `str` at 0x5d764 | store | 4 | `abs 0x12d780` | `0x1` |
| `ldr` at 0x5d768 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5d770 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5d770 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5d770 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5d770 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d750 | `(unknown callee) 0x5d778` | `r0` | `r1` | `r2` | `[0x12d780]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5d748 | `[0x12d780] == 0x0` | `[0x12d780] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5d770 | `r0` | `?` |

## `SMPConsoleSubchannel`

- `SMPConsoleSubchannel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d778 to 0x5d934.
  Established. A word store clears `0x12d780` (`str` at 0x5d794).
  Another word store writes `[sp-32]` to `0x29a2ac` (`str` at 0x5d8bc).
  Equality `[0x29a2ac] == [sp-32]` takes `0x5d910`, while inequality falls through (`beq` at 0x5d8b0).
  Console subchannel route encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d788 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5d78c | load | 4 | `abs 0x5d924` | `-` |
| `str` at 0x5d794 | store | 4 | `abs 0x12d780` | `0x0` |
| `str` at 0x5d7a4 | store | 4 | `sp -0x18` | `ret(0x6a0f8)` |
| `ldr` at 0x5d7a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d7ac | load | 4 | `ret(0x6a0f8) +0x14` | `-` |
| `ldr` at 0x5d7b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d7bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d7c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d7c4 | store | 4 | `ret(0x6a0f8) +0x14` | `[sp-20]` |
| `ldr` at 0x5d7d8 | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5d7dc | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5d7ec | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5d7f0 | load | 4 | `abs 0x29a2ac` | `-` |
| `str` at 0x5d80c | store | 4 | `sp -0x1c` | `ret(0x69eec)` |
| `ldr` at 0x5d810 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d81c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d820 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x5d828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d82c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d830 | load | 4 | `[sp-24] +0x3c` | `-` |
| `ldr` at 0x5d834 | load | 4 | `ret(0x69eec) +0x3c` | `-` |
| `ldr` at 0x5d840 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d844 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x5d84c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d850 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x5d860 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d864 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x5d880 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5d884 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d88c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d89c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5d8a0 | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5d8a4 | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5d8a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d8b4 | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5d8b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d8bc | store | 4 | `abs 0x29a2ac` | `[sp-32]` |
| `ldr` at 0x5d8c0 | load | 4 | `abs 0x5d92c` | `-` |
| `ldr` at 0x5d8c4 | load | 4 | `abs 0x13263c` | `-` |
| `ldr` at 0x5d8d0 | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5d8d4 | load | 4 | `abs 0x299bec` | `-` |
| `ldr` at 0x5d8e0 | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5d8e4 | load | 4 | `abs 0x299bec` | `-` |
| `ldr` at 0x5d8f4 | load | 4 | `abs 0x5d928` | `-` |
| `ldr` at 0x5d8f8 | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5d904 | load | 4 | `abs 0x5d930` | `-` |
| `ldr` at 0x5d910 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5d914 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x5d918 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x5d920 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5d920 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5d920 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d79c | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `0x12d780` | `0x0` |
| `bl` at 0x5d7d0 | `(unknown callee) 0x55300` | `0x1` | `[sp-20]` | `[sp-20]` | `ret(0x6a0f8)` |
| `bl` at 0x5d7d4 | `(unknown callee) 0x6a714` | `ret(0x55300)` | `?` | `?` | `?` |
| `bl` at 0x5d804 | `(unknown callee) 0x69eec` | `?` | `?` | `?` | `?` |
| `bl` at 0x5d8fc | `(unknown callee) 0x5f254` | `[0x29a2ac]` | `[[sp-24]+60]` | `0x29995c` | `0x29995c` |
| `bl` at 0x5d90c | `(unknown callee) 0x221f0` | `0xfb954` | `ret(0x5f254)` | `?` | `ret(0x5f254)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5d7b8 | `[ret(0x6a0f8)+20] == [sp-20]` | `[ret(0x6a0f8)+20] != [sp-20]` |
| `bne` at 0x5d818 | `ret(0x69eec) != 0x0` | `ret(0x69eec) == 0x0` |
| `beq` at 0x5d83c | `[[sp-24]+60] == [ret(0x69eec)+60]` | `[[sp-24]+60] != [ret(0x69eec)+60]` |
| `beq` at 0x5d8b0 | `[0x29a2ac] == [sp-32]` | `[0x29a2ac] != [sp-32]` |
| `beq` at 0x5d8cc | `[0x13263c] == 0x0` | `[0x13263c] != 0x0` |
| `beq` at 0x5d8dc | `[0x299bec] == 0x1` | `[0x299bec] != 0x1` |
| `beq` at 0x5d8ec | `[0x299bec] == 0x2` | `[0x299bec] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5d920 | `r0` | `[sp-20]` |

## `SMPSystemConsole`

- `SMPSystemConsole`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d934 to 0x5da68.
  Established. A zero entry `r0` takes `0x5d968`, while nonzero falls through (`beq` at 0x5d958).
  A word store copies `[sp-32]` to `0x29a2ac` (`str` at 0x5d9f0).
  The unnamed `0x5f254` call carries `[0x29a2ac]`, unknown, `0x29995c`, `0x29995c` (`bl` at 0x5da30).
  System console destination coding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d944 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5d948 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x5d94c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x5d950 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d95c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5d96c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x5d980 | store | 4 | `sp -0x24` | `ret(0x6a0f8)` |
| `ldr` at 0x5d984 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d994 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5d9b0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5d9b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d9bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d9cc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5d9d4 | load | 4 | `abs 0x5da5c` | `-` |
| `ldr` at 0x5d9d8 | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5d9dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5d9e8 | load | 4 | `abs 0x5da5c` | `-` |
| `ldr` at 0x5d9ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5d9f0 | store | 4 | `abs 0x29a2ac` | `[sp-32]` |
| `ldr` at 0x5d9f4 | load | 4 | `abs 0x5da60` | `-` |
| `ldr` at 0x5d9f8 | load | 4 | `abs 0x13263c` | `-` |
| `ldr` at 0x5da04 | load | 4 | `abs 0x5da5c` | `-` |
| `ldr` at 0x5da08 | load | 4 | `abs 0x299bec` | `-` |
| `ldr` at 0x5da14 | load | 4 | `abs 0x5da5c` | `-` |
| `ldr` at 0x5da18 | load | 4 | `abs 0x299bec` | `-` |
| `ldr` at 0x5da28 | load | 4 | `abs 0x5da5c` | `-` |
| `ldr` at 0x5da2c | load | 4 | `abs 0x29a2ac` | `-` |
| `ldr` at 0x5da38 | load | 4 | `abs 0x5da64` | `-` |
| `ldr` at 0x5da44 | load | 4 | `abs 0x5da5c` | `-` |
| `ldr` at 0x5da48 | load | 4 | `abs 0x29a2ac` | `-` |
| `str` at 0x5da4c | store | 4 | `sp -0x28` | `[0x29a2ac]` |
| `ldr` at 0x5da50 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x5da58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5da58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5da58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d978 | `(unknown callee) 0x6a0f8` | `0x0` | `r1` | `r2` | `r1` |
| `bl` at 0x5d9d0 | `(unknown callee) 0x6a6f4` | `ret(0x6a0f8)` | `?` | `?` | `?` |
| `bl` at 0x5da30 | `(unknown callee) 0x5f254` | `[0x29a2ac]` | `?` | `0x29995c` | `0x29995c` |
| `bl` at 0x5da40 | `(unknown callee) 0x221f0` | `0xfb97c` | `ret(0x5f254)` | `?` | `ret(0x5f254)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5d958 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x5d964 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x5d9e4 | `[0x29a2ac] == [sp-32]` | `[0x29a2ac] != [sp-32]` |
| `beq` at 0x5da00 | `[0x13263c] == 0x0` | `[0x13263c] != 0x0` |
| `beq` at 0x5da10 | `[0x299bec] == 0x1` | `[0x299bec] != 0x1` |
| `beq` at 0x5da20 | `[0x299bec] == 0x2` | `[0x299bec] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5da58 | `r0` | `?` |

## `SMPCmdResponseDelay`

- `SMPCmdResponseDelay`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81884 to 0x81950.
  Established. The unnamed `0xaa090` call carries `0x29a6b4`, `0xfffffd95`, `[[sp-28]]`, `[sp-36]` (`bl` at 0x81910).
  A zero `ret(0xaa090)` takes `0x81934`, while nonzero falls through (`beq` at 0x81924).
  A word store places `sp-52` at `sp -0x3c` (`str` at 0x818fc).
  Response-delay units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x81894 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x81898 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8189c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x818b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x818b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x818c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x818e0 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x818e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x818f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x818fc | store | 4 | `sp -0x3c` | `sp-52` |
| `ldr` at 0x81900 | load | 4 | `abs 0x81948` | `-` |
| `ldr` at 0x81904 | load | 4 | `abs 0x8194c` | `-` |
| `ldr` at 0x81908 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8190c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x81918 | store | 4 | `sp -0x20` | `ret(0xaa090)` |
| `ldr` at 0x8191c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8192c | store | 4 | `sp -0x38` | `?` |
| `str` at 0x81938 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x8193c | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x81944 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x81944 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x81944 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x818b4 | `(unknown callee) 0x5eac8` | `sp-36` | `sp-52` | `r0` | `sp-36` |
| `bl` at 0x818c8 | `(unknown callee) 0x19538` | `[sp-24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x818f0 | `(unknown callee) 0x5ed50` | `sp-36` | `sp-52` | `[sp-24]` | `ret(0x19538)` |
| `bl` at 0x81910 | `(unknown callee) 0xaa090` | `0x29a6b4` | `0xfffffd95` | `[[sp-28]]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x818c0 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x81924 | `ret(0xaa090) == 0x0` | `ret(0xaa090) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x81944 | `r0` | `[sp-56]` |

## `SMPCmdCheckBrickType`

- `SMPCmdCheckBrickType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8156c to 0x81884.
  Established. A nonzero `[sp-24]` takes `0x815c4`, while zero falls through (`bne` at 0x815b4).
  An included unnamed `0x192a4` call carries `[sp-24]`, `(ret(0x488cc) & 0xff)`, unknown, `(ret(0x488cc) & 0xff)` (`bl` at 0x8179c).
  A word store writes `ret(0x1943c)` to `[sp-28] +0x0` on known edges (`str` at 0x8184c).
  Brick-type dispatch classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8157c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x81580 | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x81588 | store | 1 | `sp -0x1d` | `(ret(0x488cc) & 0xff)` |
| `strb` at 0x81590 | store | 1 | `sp -0x1e` | `0x0` |
| `str` at 0x81598 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x815a0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x815a8 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x815ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x815bc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x815c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x815c8 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x815d8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x815dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x815e4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrb` at 0x815e8 | load | 1 | `sp -0x1d` | `-` |
| `ldrls` at 0x815f4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x8178c | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x81790 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x81794 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x817b0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x817b8 | store | 4 | `sp -0x34` | `0x0` |
| `ldrb` at 0x817bc | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x817c8 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x817cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x817e8 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x817ec | load | 4 | `sp -0x34` | `-` |
| `str` at 0x817f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x817f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x817f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81804 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81808 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81814 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x81818 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x81824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x81828 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x81830 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x81838 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x8183c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8184c | store | 4 | `[sp-28] +0x0` | `ret(0x1943c)` |
| `ldr` at 0x81850 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x81854 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x81858 | load | 4 | `abs 0x81880` | `-` |
| `str` at 0x81864 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x81870 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x81874 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x8187c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x8187c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8187c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8187c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x81584 | `(unknown callee) 0x488cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x8179c | `(unknown callee) 0x192a4` | `[sp-24]` | `(ret(0x488cc) & 0xff)` | `?` | `(ret(0x488cc) & 0xff)` |
| `bl` at 0x817d4 | `(unknown callee) 0x192a4` | `[sp-24]` | `[sp-30]:1` | `?` | `[sp-30]:1` |
| `bl` at 0x8182c | `(unknown callee) 0x1970c` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x81844 | `(unknown callee) 0x1943c` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x8185c | `(unknown callee) 0x19808` | `[[sp-28]]` | `0x1072ac` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x815b4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x815d0 | `[[sp-24]]:1 != 0x7e` | `[[sp-24]]:1 == 0x7e` |
| `ldrls` at 0x815f4 | `(ret(0x488cc) & 0xff)-35 <= 0x59 (unsigned, 32-bit)` | `(ret(0x488cc) & 0xff)-35 > 0x59 (unsigned, 32-bit)` |
| `beq` at 0x817c4 | `[sp-30]:1 == 0x0` | `[sp-30]:1 != 0x0` |
| `beq` at 0x817e0 | `ret(0x192a4) == 0x0` | `ret(0x192a4) != 0x0` |
| `beq` at 0x81800 | `[sp-36] == [sp-44]` | `[sp-36] != [sp-44]` |
| `beq` at 0x81810 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x81820 | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8187c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x815f4).

## `SMPCmdPostProcess`

- `SMPCmdPostProcess`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81e68 to 0x81f2c.
  Established. Included word stores write `[0x133ac8]` to `[sp-24] +0x410` and `0x107490` to `[sp-20] +0x0` (`str` at 0x81efc, `str` at 0x81f08).
  Inequality `ret(0x196f4) != 0x2` takes `0x81f0c`, while equality falls through (`bne` at 0x81ea4).
  Command postprocessing trigger meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x81e78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x81e7c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x81e80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x81e8c | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x81e90 | load | 4 | `abs 0x81f14` | `-` |
| `ldr` at 0x81ea8 | load | 4 | `abs 0x81f18` | `-` |
| `ldr` at 0x81eac | load | 4 | `abs 0x133ab0` | `-` |
| `ldr` at 0x81ec0 | load | 4 | `abs 0x81f1c` | `-` |
| `ldr` at 0x81edc | load | 4 | `abs 0x81f20` | `-` |
| `ldr` at 0x81ef0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x81ef4 | load | 4 | `abs 0x81f24` | `-` |
| `ldr` at 0x81ef8 | load | 4 | `abs 0x133ac8` | `-` |
| `str` at 0x81efc | store | 4 | `[sp-24] +0x410` | `[0x133ac8]` |
| `ldr` at 0x81f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x81f04 | load | 4 | `abs 0x81f28` | `-` |
| `str` at 0x81f08 | store | 4 | `[sp-20] +0x0` | `0x107490` |
| `ldm` at 0x81f10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x81f10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x81f10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x81e98 | `(unknown callee) 0x196f4` | `[r0]` | `0x107478` | `sp-56` | `sp-88` |
| `bl` at 0x81ec4 | `(unknown callee) 0x19310` | `sp-56` | `0x107484` | `?` | `sp-56` |
| `bl` at 0x81ee0 | `(unknown callee) 0x19310` | `sp-88` | `0x107488` | `?` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x81ea4 | `ret(0x196f4) != 0x2` | `ret(0x196f4) == 0x2` |
| `beq` at 0x81eb4 | `[0x133ab0] == 0x0` | `[0x133ab0] != 0x0` |
| `bne` at 0x81ed0 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |
| `bne` at 0x81eec | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x81f10 | `r0` | `?` |

## `SMPCmdNoResponse`

- `SMPCmdNoResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81e18 to 0x81e68.
  Established. A nonzero `ret(0x19310)` takes `0x81e50`, while zero falls through (`bne` at 0x81e40).
  The unnamed `0x19310` call carries entry `r0`, `0x1018cc`, entry `r2`, entry `r3` (`bl` at 0x81e34).
  No-response command classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x81e28 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x81e2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x81e30 | load | 4 | `abs 0x81e64` | `-` |
| `str` at 0x81e48 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x81e54 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x81e58 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x81e60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x81e60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x81e60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x81e34 | `(unknown callee) 0x19310` | `r0` | `0x1018cc` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x81e40 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x81e60 | `r0` | `[sp-24]` |

## `SMPCmd_2m_Init`

- `SMPCmd_2m_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x96478 to 0x964c0.
  Established. Word stores write `0x9387c` to `0x133904` and `0x93e78` to `0x133908` (`str` at 0x96498, `str` at 0x964a4).
  The unnamed `0xaec8c` call carries `0x10c49c`, `0xa6`, entry `r2`, entry `r3` (`bl` at 0x9648c).
  Stored callback roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x96484 | load | 4 | `abs 0x964ac` | `-` |
| `ldr` at 0x96490 | load | 4 | `abs 0x964b0` | `-` |
| `ldr` at 0x96494 | load | 4 | `abs 0x964b4` | `-` |
| `str` at 0x96498 | store | 4 | `abs 0x133904` | `0x9387c` |
| `ldr` at 0x9649c | load | 4 | `abs 0x964b8` | `-` |
| `ldr` at 0x964a0 | load | 4 | `abs 0x964bc` | `-` |
| `str` at 0x964a4 | store | 4 | `abs 0x133908` | `0x93e78` |
| `ldm` at 0x964a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x964a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x964a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9648c | `(unknown callee) 0xaec8c` | `0x10c49c` | `0xa6` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x964a8 | `r0` | `ret(0xaec8c)` |

## `SMPCmdInit`

- `SMPCmdInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8cec4 to 0x8d160.
  Established. The unsigned comparison `ret(0x8d160) <= 0x7ffff` takes `0x8d08c`, while greater falls through (`bls` at 0x8d07c).
  A word store increments `0x133900` with `[0x133900]+1` (`str` at 0x8d0ec).
  The unnamed `0xaec8c` call carries `0x1021a8`, `0x131` and two unknown candidates (`bl` at 0x8d0d8).
  Command initialization resource sizing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8ced4 | load | 4 | `abs 0x8d0f8` | `-` |
| `ldr` at 0x8ced8 | load | 4 | `abs 0x133900` | `-` |
| `ldr` at 0x8cee4 | load | 4 | `abs 0x8d0fc` | `-` |
| `str` at 0x8cee8 | store | 4 | `sp -0x2c` | `0x10a2b4` |
| `ldr` at 0x8cef0 | load | 4 | `abs 0x8d100` | `-` |
| `ldr` at 0x8cef4 | load | 4 | `abs 0x8d104` | `-` |
| `ldr` at 0x8cf00 | load | 4 | `abs 0x8d108` | `-` |
| `ldr` at 0x8cf10 | load | 4 | `abs 0x8d10c` | `-` |
| `ldr` at 0x8cf14 | load | 4 | `abs 0x8d110` | `-` |
| `str` at 0x8cf24 | store | 4 | `sp -0x14` | `ret(0xa8af0)` |
| `ldr` at 0x8cf28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cf34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cf40 | load | 4 | `abs 0x8d114` | `-` |
| `str` at 0x8cf44 | store | 4 | `sp -0x2c` | `0x10a2fc` |
| `str` at 0x8cf48 | store | 4 | `sp -0x28` | `ret(0x1f8e0)` |
| `ldr` at 0x8cf50 | load | 4 | `abs 0x8d100` | `-` |
| `ldr` at 0x8cf54 | load | 4 | `abs 0x8d118` | `-` |
| `ldr` at 0x8cf60 | load | 4 | `abs 0x8d108` | `-` |
| `ldr` at 0x8cf64 | load | 4 | `abs 0x8d11c` | `-` |
| `ldr` at 0x8cf6c | load | 4 | `abs 0x8d10c` | `-` |
| `str` at 0x8cf78 | store | 4 | `sp -0x14` | `ret(0xa76a4)` |
| `ldr` at 0x8cf7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cf88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cf94 | load | 4 | `abs 0x8d120` | `-` |
| `str` at 0x8cf98 | store | 4 | `sp -0x2c` | `0x10a320` |
| `str` at 0x8cf9c | store | 4 | `sp -0x28` | `ret(0x1f8e0)` |
| `ldr` at 0x8cfa4 | load | 4 | `abs 0x8d100` | `-` |
| `ldr` at 0x8cfa8 | load | 4 | `abs 0x8d124` | `-` |
| `ldr` at 0x8cfb4 | load | 4 | `abs 0x8d128` | `-` |
| `ldr` at 0x8cfb8 | load | 4 | `abs 0x8d12c` | `-` |
| `str` at 0x8cfc8 | store | 4 | `sp -0x14` | `ret(0xa8af0)` |
| `ldr` at 0x8cfcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cfd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cfe4 | load | 4 | `abs 0x8d130` | `-` |
| `str` at 0x8cfe8 | store | 4 | `sp -0x2c` | `0x10a350` |
| `str` at 0x8cfec | store | 4 | `sp -0x28` | `ret(0x1f8e0)` |
| `ldr` at 0x8cff4 | load | 4 | `abs 0x8d100` | `-` |
| `ldr` at 0x8cff8 | load | 4 | `abs 0x8d134` | `-` |
| `ldr` at 0x8d004 | load | 4 | `abs 0x8d108` | `-` |
| `ldr` at 0x8d008 | load | 4 | `abs 0x8d11c` | `-` |
| `ldr` at 0x8d010 | load | 4 | `abs 0x8d128` | `-` |
| `str` at 0x8d01c | store | 4 | `sp -0x14` | `ret(0xa76a4)` |
| `ldr` at 0x8d020 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d02c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d038 | load | 4 | `abs 0x8d138` | `-` |
| `str` at 0x8d03c | store | 4 | `sp -0x2c` | `0x10a374` |
| `str` at 0x8d040 | store | 4 | `sp -0x28` | `ret(0x1f8e0)` |
| `ldr` at 0x8d048 | load | 4 | `abs 0x8d100` | `-` |
| `ldr` at 0x8d04c | load | 4 | `abs 0x8d13c` | `-` |
| `ldr` at 0x8d058 | load | 4 | `abs 0x8d140` | `-` |
| `ldr` at 0x8d05c | load | 4 | `abs 0x8d144` | `-` |
| `ldr` at 0x8d074 | load | 4 | `abs 0x8d148` | `-` |
| `str` at 0x8d084 | store | 4 | `sp -0x18` | `0x1800` |
| `str` at 0x8d090 | store | 4 | `sp -0x18` | `0x1000` |
| `str` at 0x8d098 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x8d09c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d0a0 | store | 4 | `sp -0x28` | `[sp-24]` |
| `str` at 0x8d0a8 | store | 4 | `sp -0x24` | `0x14` |
| `str` at 0x8d0b0 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x8d0b8 | store | 4 | `sp -0x1c` | `0x9` |
| `ldr` at 0x8d0bc | load | 4 | `abs 0x8d14c` | `-` |
| `ldr` at 0x8d0c0 | load | 4 | `abs 0x8d150` | `-` |
| `ldr` at 0x8d0c4 | load | 4 | `abs 0x8d154` | `-` |
| `ldr` at 0x8d0d0 | load | 4 | `abs 0x8d158` | `-` |
| `ldr` at 0x8d0d4 | load | 4 | `abs 0x8d15c` | `-` |
| `ldr` at 0x8d0dc | load | 4 | `abs 0x8d0f8` | `-` |
| `ldr` at 0x8d0e0 | load | 4 | `abs 0x8d0f8` | `-` |
| `ldr` at 0x8d0e4 | load | 4 | `abs 0x133900` | `-` |
| `str` at 0x8d0ec | store | 4 | `abs 0x133900` | `[0x133900]+1` |
| `ldm` at 0x8d0f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8d0f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8d0f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8cefc | `(unknown callee) 0x200a0` | `0x9` | `0x10a2d8` | `0x17a2` | `0x0` |
| `bl` at 0x8cf0c | `(unknown callee) 0xa89d0` | `0x29a6b4` | `0x3` | `0x0` | `?` |
| `bl` at 0x8cf1c | `(unknown callee) 0xa8af0` | `0x29a6bc` | `0x10a2f0` | `0xa` | `?` |
| `bl` at 0x8cf38 | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0x8cf5c | `(unknown callee) 0x200a0` | `0x9` | `0x10a2d8` | `0x17af` | `0x0` |
| `bl` at 0x8cf70 | `(unknown callee) 0xa76a4` | `0x29a6b4` | `0xffff003` | `0x1` | `0x29a6bc` |
| `bl` at 0x8cf8c | `(unknown callee) 0x1f8e0` | `ret(0xa76a4)` | `?` | `?` | `ret(0xa76a4)` |
| `bl` at 0x8cfb0 | `(unknown callee) 0x200a0` | `0x9` | `0x10a2d8` | `0x17b7` | `0x0` |
| `bl` at 0x8cfc0 | `(unknown callee) 0xa8af0` | `0x29a754` | `0x10a344` | `0xa` | `?` |
| `bl` at 0x8cfdc | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0x8d000 | `(unknown callee) 0x200a0` | `0x9` | `0x10a2d8` | `0x17be` | `0x0` |
| `bl` at 0x8d014 | `(unknown callee) 0xa76a4` | `0x29a6b4` | `0xffff003` | `0x2` | `0x29a754` |
| `bl` at 0x8d030 | `(unknown callee) 0x1f8e0` | `ret(0xa76a4)` | `?` | `?` | `ret(0xa76a4)` |
| `bl` at 0x8d054 | `(unknown callee) 0x200a0` | `0x9` | `0x10a2d8` | `0x17c6` | `0x0` |
| `bl` at 0x8d068 | `(unknown callee) 0x20bb4` | `0x29a7f0` | `0x10a39c` | `0x1` | `0x9` |
| `bl` at 0x8d06c | `(unknown callee) 0x8d160` | `ret(0x20bb4)` | `?` | `?` | `?` |
| `bl` at 0x8d0cc | `(unknown callee) 0x2066c` | `0x29a7ec` | `0x10a3a8` | `0x8e46c` | `0x0` |
| `bl` at 0x8d0d8 | `(unknown callee) 0xaec8c` | `0x1021a8` | `0x131` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8cee0 | `[0x133900] == 0x0` | `[0x133900] != 0x0` |
| `beq` at 0x8cf30 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `beq` at 0x8cf84 | `ret(0xa76a4) == 0x0` | `ret(0xa76a4) != 0x0` |
| `beq` at 0x8cfd4 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `beq` at 0x8d028 | `ret(0xa76a4) == 0x0` | `ret(0xa76a4) != 0x0` |
| `bls` at 0x8d07c | `ret(0x8d160) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x8d160) > 0x7ffff (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8d0f4 | `r0` | `ret(0xaec8c)` |

## `IcmbCvtToU16`

- `IcmbCvtToU16`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1aa8 to 0xa1b04.
  Established. Byte reads address entry `r0 +0x1` and entry `r0 +0x0` (`ldrb` at 0xa1ac8, `ldrb` at 0xa1ae0).
  The integer return candidate is `[sp-22]:2` (`ldm` at 0xa1b00).
  ICMB integer conversion bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1ab8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1ac0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1ac8 | load | 1 | `r0 +0x1` | `-` |
| `strh` at 0xa1acc | store | 2 | `sp -0x16` | `([r0+1]:1 & 0xffff)` |
| `ldrh` at 0xa1ad0 | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xa1ad8 | store | 2 | `sp -0x16` | `?` |
| `ldr` at 0xa1adc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1ae0 | load | 1 | `r0 +0x0` | `-` |
| `ldrh` at 0xa1ae8 | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xa1af0 | store | 2 | `sp -0x16` | `?` |
| `ldrh` at 0xa1af4 | load | 2 | `sp -0x16` | `-` |
| `ldm` at 0xa1b00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1b00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1b00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1b00 | `r0` | `[sp-22]:2` |

## `IcmbCvtToMsg`

- `IcmbCvtToMsg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1b04 to 0xa1b4c.
  Established. Byte stores write `((r1 & 0xffff) & 0xff)` to entry `r0 +0x0` and an unknown value to `[sp-20] +0x1` (`strb` at 0xa1b28, `strb` at 0xa1b40).
  The integer return candidate retains entry `r0` (`ldm` at 0xa1b48).
  Message output capacity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1b14 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0xa1b1c | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `ldr` at 0xa1b20 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa1b24 | load | 2 | `sp -0x16` | `-` |
| `strb` at 0xa1b28 | store | 1 | `r0 +0x0` | `((r1 & 0xffff) & 0xff)` |
| `ldr` at 0xa1b30 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa1b38 | load | 2 | `sp -0x16` | `-` |
| `strb` at 0xa1b40 | store | 1 | `[sp-20] +0x1` | `?` |
| `ldm` at 0xa1b48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1b48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1b48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1b48 | `r0` | `r0` |
