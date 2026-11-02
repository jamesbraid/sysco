# L1 pointer-relative controller write loops

These ColdFire entries record each package and ROM location separately. They establish the
listed byte writes and call routes. The hardware roles of the loaded bases and the purposes
of the callees remain unknown here. The [M-Bus overview](mbus.md) gives context for the later
reference image, without establishing those meanings for these bodies.

`sp` is the entry stack pointer. Brackets denote loaded values. Each base load is a separate
access, with no assumption that repeated loads return a stable value. Call candidates name
assumed M68K stack locations with unknown arity. An unknown bit-test value or loop condition
is left unknown. The analysed spans include the terminating return.

The access tables include non-stack sites. A dash in Stored value marks a load.

## 6.5.14:l1#0 at 0xffe67434

- `0xffe67434`, `6.5.14/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.14:l1#0`, version `1.8.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffe67434 to 0xffe674d2.
  Established. The first four byte stores carry zero through `[0x3000074c] +0x1e8`, 0x13 through
  +0x1e4, zero through +0x1e0 and 0x80 through +0x1e8 (`clr.b` at 0xffe67454, `move.b` at
  0xffe6745a, `clr.b` at 0xffe67460, `move.b` at 0xffe67466). The subsequent `beq.b` takes the
  restore path when `(? & 0x20) == 0`, with the tested value unresolved (`beq.b` at 0xffe67476).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe67452 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffe67454 | store | 1 | `[0x3000074c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe67458 | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffe6745a | store | 1 | `[0x3000074c] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6745e | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffe67460 | store | 1 | `[0x3000074c] +0x1e0` | `0x0` |
| `movea.l` at 0xffe67464 | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffe67466 | store | 1 | `[0x3000074c] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6746a | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffe6746e | load | 1 | `[0x3000074c] +0x1ec` | `-` |
| `movea.l` at 0xffe67478 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffe6747a | store | 1 | `[0x3000074c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6747e | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffe67480 | store | 1 | `[0x3000074c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe67484 | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffe67486 | load | 1 | `[0x3000074c] +0x1f0` | `-` |
| `movea.l` at 0xffe6748a | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffe6748e | load | 1 | `[0x3000074c] +0x1ec` | `-` |
| `movea.l` at 0xffe67492 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffe67494 | store | 1 | `[0x3000074c] +0x1ec` | `0x0` |
| `movea.l` at 0xffe67498 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffe6749a | store | 1 | `[0x3000074c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe674be | `0xffe579f4` | `0x9` | `0xffe67c4f` | `0x148` | `0x0` | `0xffe67c71` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe67476 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe674a2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe674d0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.8.0-package-0/controller-write-loops-facts.txt).

## 6.5.14:l1#0 at 0xffed7434

- `0xffed7434`, `6.5.14/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.14:l1#0`, version `1.8.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffed7434 to 0xffed74d2.
  Established. Falling through `beq.b` for `(? & 0x20) != 0` reaches a byte-clear site and a byte
  0xa0 store site, each using the effective expression `[0x3000074c] +0x1e8` (`beq.b` at 0xffed7476,
  `clr.b` at 0xffed747a, `move.b` at 0xffed7480). Each store uses a separate four-byte load from
  0x3000074c (`movea.l` at 0xffed7478, `movea.l` at 0xffed747e). The later backward `blt.b` retains
  unknown flag operands (`blt.b` at 0xffed74a2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffed7452 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffed7454 | store | 1 | `[0x3000074c] +0x1e8` | `0x0` |
| `movea.l` at 0xffed7458 | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffed745a | store | 1 | `[0x3000074c] +0x1e4` | `0x13` |
| `movea.l` at 0xffed745e | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffed7460 | store | 1 | `[0x3000074c] +0x1e0` | `0x0` |
| `movea.l` at 0xffed7464 | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffed7466 | store | 1 | `[0x3000074c] +0x1e8` | `0x80` |
| `movea.l` at 0xffed746a | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffed746e | load | 1 | `[0x3000074c] +0x1ec` | `-` |
| `movea.l` at 0xffed7478 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffed747a | store | 1 | `[0x3000074c] +0x1e8` | `0x0` |
| `movea.l` at 0xffed747e | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffed7480 | store | 1 | `[0x3000074c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffed7484 | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffed7486 | load | 1 | `[0x3000074c] +0x1f0` | `-` |
| `movea.l` at 0xffed748a | load | 4 | `abs 0x3000074c` | `-` |
| `move.b` at 0xffed748e | load | 1 | `[0x3000074c] +0x1ec` | `-` |
| `movea.l` at 0xffed7492 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffed7494 | store | 1 | `[0x3000074c] +0x1ec` | `0x0` |
| `movea.l` at 0xffed7498 | load | 4 | `abs 0x3000074c` | `-` |
| `clr.b` at 0xffed749a | store | 1 | `[0x3000074c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffed74be | `0xffec79f4` | `0x9` | `0xffed7c4f` | `0x148` | `0x0` | `0xffed7c71` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffed7476 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffed74a2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffed74d0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.8.0-package-0/controller-write-loops-facts.txt).

## 6.5.15:l1#0 at 0xffe6b704

- `0xffe6b704`, `6.5.15/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.15:l1#0`, version `1.10.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffe6b704 to 0xffe6b7a2.
  Established. The byte read through `[0x300007d8] +0x1ec` precedes the bit-test branch (`move.b` at
  0xffe6b73e, `beq.b` at 0xffe6b746). On the branch's nonzero-mask fall-through, later byte reads
  use `[0x300007d8] +0x1f0` and `[0x300007d8] +0x1ec` (`beq.b` at 0xffe6b746, `move.b` at
  0xffe6b756, `move.b` at 0xffe6b75e). The sixth printed candidate at the later call is unknown (`jsr` at 0xffe6b78e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6b722 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffe6b724 | store | 1 | `[0x300007d8] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b728 | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffe6b72a | store | 1 | `[0x300007d8] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6b72e | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffe6b730 | store | 1 | `[0x300007d8] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6b734 | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffe6b736 | store | 1 | `[0x300007d8] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6b73a | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffe6b73e | load | 1 | `[0x300007d8] +0x1ec` | `-` |
| `movea.l` at 0xffe6b748 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffe6b74a | store | 1 | `[0x300007d8] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b74e | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffe6b750 | store | 1 | `[0x300007d8] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6b754 | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffe6b756 | load | 1 | `[0x300007d8] +0x1f0` | `-` |
| `movea.l` at 0xffe6b75a | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffe6b75e | load | 1 | `[0x300007d8] +0x1ec` | `-` |
| `movea.l` at 0xffe6b762 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffe6b764 | store | 1 | `[0x300007d8] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6b768 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffe6b76a | store | 1 | `[0x300007d8] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6b78e | `0xffe583ac` | `0x9` | `0xffe6bf3b` | `0x149` | `0x0` | `0xffe6bf61` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6b746 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6b772 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6b7a0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.10.0-package-0/controller-write-loops-facts.txt).

## 6.5.15:l1#0 at 0xffedb704

- `0xffedb704`, `6.5.15/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.15:l1#0`, version `1.10.0`. Payload file offset 0x2c.
  Load 0xffe00000. Analysed span 0xffedb704 to 0xffedb7a2.
  Established. The direct call targets 0xffec83ac with candidates beginning 9, 0xffedbf3b, 0x149,
  zero, 0xffedbf61 and unknown (`jsr` at 0xffedb78e). The earlier `beq.b` can skip the remaining
  writes, loop branch and call by following its `(? & 0x20) == 0` path (`beq.b` at 0xffedb746). The
  callee's role and the meanings of these candidate values remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedb722 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffedb724 | store | 1 | `[0x300007d8] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb728 | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffedb72a | store | 1 | `[0x300007d8] +0x1e4` | `0x13` |
| `movea.l` at 0xffedb72e | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffedb730 | store | 1 | `[0x300007d8] +0x1e0` | `0x0` |
| `movea.l` at 0xffedb734 | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffedb736 | store | 1 | `[0x300007d8] +0x1e8` | `0x80` |
| `movea.l` at 0xffedb73a | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffedb73e | load | 1 | `[0x300007d8] +0x1ec` | `-` |
| `movea.l` at 0xffedb748 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffedb74a | store | 1 | `[0x300007d8] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb74e | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffedb750 | store | 1 | `[0x300007d8] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedb754 | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffedb756 | load | 1 | `[0x300007d8] +0x1f0` | `-` |
| `movea.l` at 0xffedb75a | load | 4 | `abs 0x300007d8` | `-` |
| `move.b` at 0xffedb75e | load | 1 | `[0x300007d8] +0x1ec` | `-` |
| `movea.l` at 0xffedb762 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffedb764 | store | 1 | `[0x300007d8] +0x1ec` | `0x0` |
| `movea.l` at 0xffedb768 | load | 4 | `abs 0x300007d8` | `-` |
| `clr.b` at 0xffedb76a | store | 1 | `[0x300007d8] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedb78e | `0xffec83ac` | `0x9` | `0xffedbf3b` | `0x149` | `0x0` | `0xffedbf61` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedb746 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedb772 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedb7a0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.10.0-package-0/controller-write-loops-facts.txt).

## 6.5.16:l1#0 at 0xffe69dc8

- `0xffe69dc8`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#0`, version `1.12.3`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe69dc8 to 0xffe69e66.
  Established. Before the bit-test branch, byte 0x13 is stored through `[0x30000630] +0x1e4` and
  byte 0x80 through `[0x30000630] +0x1e8` (`move.b` at 0xffe69dee, `move.b` at 0xffe69dfa). The
  later byte 0xa0 store through `[0x30000630] +0x1e8` is on the nonzero-mask fall-through of `beq.b`
  (`beq.b` at 0xffe69e0a, `move.b` at 0xffe69e14). A further byte clear through `[0x30000630]
  +0x1ec` precedes the final byte clear through +0x1e8 (`clr.b` at 0xffe69e28, `clr.b` at
  0xffe69e2e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe69de6 | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffe69de8 | store | 1 | `[0x30000630] +0x1e8` | `0x0` |
| `movea.l` at 0xffe69dec | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffe69dee | store | 1 | `[0x30000630] +0x1e4` | `0x13` |
| `movea.l` at 0xffe69df2 | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffe69df4 | store | 1 | `[0x30000630] +0x1e0` | `0x0` |
| `movea.l` at 0xffe69df8 | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffe69dfa | store | 1 | `[0x30000630] +0x1e8` | `0x80` |
| `movea.l` at 0xffe69dfe | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffe69e02 | load | 1 | `[0x30000630] +0x1ec` | `-` |
| `movea.l` at 0xffe69e0c | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffe69e0e | store | 1 | `[0x30000630] +0x1e8` | `0x0` |
| `movea.l` at 0xffe69e12 | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffe69e14 | store | 1 | `[0x30000630] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe69e18 | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffe69e1a | load | 1 | `[0x30000630] +0x1f0` | `-` |
| `movea.l` at 0xffe69e1e | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffe69e22 | load | 1 | `[0x30000630] +0x1ec` | `-` |
| `movea.l` at 0xffe69e26 | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffe69e28 | store | 1 | `[0x30000630] +0x1ec` | `0x0` |
| `movea.l` at 0xffe69e2c | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffe69e2e | store | 1 | `[0x30000630] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe69e52 | `0xffe5e128` | `0x9` | `0xffe6a62b` | `0x14d` | `0x0` | `0xffe6a651` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe69e0a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe69e36 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe69e64 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-0/controller-write-loops-facts.txt).

## 6.5.16:l1#0 at 0xffed9dc8

- `0xffed9dc8`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#0`, version `1.12.3`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffed9dc8 to 0xffed9e66.
  Established. The constant assignment to A1 is 0x3e8 (`movea.w` at 0xffed9dd6). The backward
  `blt.b` condition has unresolved operands, so this entry does not establish a count from that
  constant (`blt.b` at 0xffed9e36). The subsequent call targets 0xffece128 and has first candidate 9
  (`jsr` at 0xffed9e52).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffed9de6 | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffed9de8 | store | 1 | `[0x30000630] +0x1e8` | `0x0` |
| `movea.l` at 0xffed9dec | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffed9dee | store | 1 | `[0x30000630] +0x1e4` | `0x13` |
| `movea.l` at 0xffed9df2 | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffed9df4 | store | 1 | `[0x30000630] +0x1e0` | `0x0` |
| `movea.l` at 0xffed9df8 | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffed9dfa | store | 1 | `[0x30000630] +0x1e8` | `0x80` |
| `movea.l` at 0xffed9dfe | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffed9e02 | load | 1 | `[0x30000630] +0x1ec` | `-` |
| `movea.l` at 0xffed9e0c | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffed9e0e | store | 1 | `[0x30000630] +0x1e8` | `0x0` |
| `movea.l` at 0xffed9e12 | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffed9e14 | store | 1 | `[0x30000630] +0x1e8` | `0xa0` |
| `movea.l` at 0xffed9e18 | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffed9e1a | load | 1 | `[0x30000630] +0x1f0` | `-` |
| `movea.l` at 0xffed9e1e | load | 4 | `abs 0x30000630` | `-` |
| `move.b` at 0xffed9e22 | load | 1 | `[0x30000630] +0x1ec` | `-` |
| `movea.l` at 0xffed9e26 | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffed9e28 | store | 1 | `[0x30000630] +0x1ec` | `0x0` |
| `movea.l` at 0xffed9e2c | load | 4 | `abs 0x30000630` | `-` |
| `clr.b` at 0xffed9e2e | store | 1 | `[0x30000630] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffed9e52 | `0xffece128` | `0x9` | `0xffeda62b` | `0x14d` | `0x0` | `0xffeda651` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffed9e0a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffed9e36 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffed9e64 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-0/controller-write-loops-facts.txt).

## 6.5.16:l1#1 at 0xffe73150

- `0xffe73150`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#1`, version `1.12.3`. Payload file offset 0xedf50.
  Load 0xffe00000. Analysed span 0xffe73150 to 0xffe731ee.
  Established. A four-byte read from 0x300006a0 supplies the base for the first byte-zero store
  through `[0x300006a0] +0x1e8` (`movea.l` at 0xffe7316e, `clr.b` at 0xffe73170). On the later
  nonzero-mask fall-through, the final two byte clears target `[0x300006a0] +0x1ec` and +0x1e8,
  using fresh base loads (`beq.b` at 0xffe73192, `movea.l` at 0xffe731ae, `clr.b` at 0xffe731b0,
  `movea.l` at 0xffe731b4, `clr.b` at 0xffe731b6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe7316e | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xffe73170 | store | 1 | `[0x300006a0] +0x1e8` | `0x0` |
| `movea.l` at 0xffe73174 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xffe73176 | store | 1 | `[0x300006a0] +0x1e4` | `0x13` |
| `movea.l` at 0xffe7317a | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xffe7317c | store | 1 | `[0x300006a0] +0x1e0` | `0x0` |
| `movea.l` at 0xffe73180 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xffe73182 | store | 1 | `[0x300006a0] +0x1e8` | `0x80` |
| `movea.l` at 0xffe73186 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xffe7318a | load | 1 | `[0x300006a0] +0x1ec` | `-` |
| `movea.l` at 0xffe73194 | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xffe73196 | store | 1 | `[0x300006a0] +0x1e8` | `0x0` |
| `movea.l` at 0xffe7319a | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xffe7319c | store | 1 | `[0x300006a0] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe731a0 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xffe731a2 | load | 1 | `[0x300006a0] +0x1f0` | `-` |
| `movea.l` at 0xffe731a6 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xffe731aa | load | 1 | `[0x300006a0] +0x1ec` | `-` |
| `movea.l` at 0xffe731ae | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xffe731b0 | store | 1 | `[0x300006a0] +0x1ec` | `0x0` |
| `movea.l` at 0xffe731b4 | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xffe731b6 | store | 1 | `[0x300006a0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe731da | `0xffe6625c` | `0x9` | `0xffe739b3` | `0x14d` | `0x0` | `0xffe739d9` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe73192 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe731be | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe731ec | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-1/controller-write-loops-facts.txt).

## 6.5.16:l1#1 at 0xfff63150

- `0xfff63150`, `6.5.16/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.16:l1#1`, version `1.12.3`. Payload file offset 0xedf50.
  Load 0xffe00000. Analysed span 0xfff63150 to 0xfff631ee.
  Established. The byte-zero store through `[0x300006a0] +0x1e0` and byte 0x80 store through +0x1e8
  occur before `beq.b` (`clr.b` at 0xfff6317c, `move.b` at 0xfff63182, `beq.b` at 0xfff63192).
  Taking that branch for `(? & 0x20) == 0` reaches the register-restore path (`beq.b` at
  0xfff63192); its fall-through instead reaches the later byte 0xa0 store through +0x1e8 (`beq.b` at
  0xfff63192, `move.b` at 0xfff6319c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff6316e | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xfff63170 | store | 1 | `[0x300006a0] +0x1e8` | `0x0` |
| `movea.l` at 0xfff63174 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xfff63176 | store | 1 | `[0x300006a0] +0x1e4` | `0x13` |
| `movea.l` at 0xfff6317a | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xfff6317c | store | 1 | `[0x300006a0] +0x1e0` | `0x0` |
| `movea.l` at 0xfff63180 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xfff63182 | store | 1 | `[0x300006a0] +0x1e8` | `0x80` |
| `movea.l` at 0xfff63186 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xfff6318a | load | 1 | `[0x300006a0] +0x1ec` | `-` |
| `movea.l` at 0xfff63194 | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xfff63196 | store | 1 | `[0x300006a0] +0x1e8` | `0x0` |
| `movea.l` at 0xfff6319a | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xfff6319c | store | 1 | `[0x300006a0] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff631a0 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xfff631a2 | load | 1 | `[0x300006a0] +0x1f0` | `-` |
| `movea.l` at 0xfff631a6 | load | 4 | `abs 0x300006a0` | `-` |
| `move.b` at 0xfff631aa | load | 1 | `[0x300006a0] +0x1ec` | `-` |
| `movea.l` at 0xfff631ae | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xfff631b0 | store | 1 | `[0x300006a0] +0x1ec` | `0x0` |
| `movea.l` at 0xfff631b4 | load | 4 | `abs 0x300006a0` | `-` |
| `clr.b` at 0xfff631b6 | store | 1 | `[0x300006a0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff631da | `0xfff5625c` | `0x9` | `0xfff639b3` | `0x14d` | `0x0` | `0xfff639d9` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff63192 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff631be | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff631ec | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.12.3-package-1/controller-write-loops-facts.txt).

## 6.5.17:l1#0 at 0xffe6b780

- `0xffe6b780`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#0`, version `1.14.1`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe6b780 to 0xffe6b81e.
  Established. The branch pair has different limits: `beq.b` takes the restore path for `(? & 0x20)
  == 0`, while the backward `blt.b` uses unresolved signed-condition flags (`beq.b` at 0xffe6b7c2,
  `blt.b` at 0xffe6b7ee). A direct call after the backward branch's fall-through targets 0xffe5d328
  with candidates 9, 0xffe6bfe3, 0x14d, zero, 0xffe6c009 and unknown (`jsr` at 0xffe6b80a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6b79e | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffe6b7a0 | store | 1 | `[0x300002d4] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b7a4 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffe6b7a6 | store | 1 | `[0x300002d4] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6b7aa | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffe6b7ac | store | 1 | `[0x300002d4] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6b7b0 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffe6b7b2 | store | 1 | `[0x300002d4] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6b7b6 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffe6b7ba | load | 1 | `[0x300002d4] +0x1ec` | `-` |
| `movea.l` at 0xffe6b7c4 | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffe6b7c6 | store | 1 | `[0x300002d4] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b7ca | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffe6b7cc | store | 1 | `[0x300002d4] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6b7d0 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffe6b7d2 | load | 1 | `[0x300002d4] +0x1f0` | `-` |
| `movea.l` at 0xffe6b7d6 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffe6b7da | load | 1 | `[0x300002d4] +0x1ec` | `-` |
| `movea.l` at 0xffe6b7de | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffe6b7e0 | store | 1 | `[0x300002d4] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6b7e4 | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffe6b7e6 | store | 1 | `[0x300002d4] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6b80a | `0xffe5d328` | `0x9` | `0xffe6bfe3` | `0x14d` | `0x0` | `0xffe6c009` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6b7c2 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6b7ee | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6b81c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-0/controller-write-loops-facts.txt).

## 6.5.17:l1#0 at 0xffedb780

- `0xffedb780`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#0`, version `1.14.1`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffedb780 to 0xffedb81e.
  Established. Two byte-read sites use `[0x300002d4] +0x1ec`, one before `beq.b` and the other on
  its nonzero-mask fall-through (`move.b` at 0xffedb7ba, `beq.b` at 0xffedb7c2, `move.b` at
  0xffedb7da). The latter region ends its byte writes with zero through `[0x300002d4] +0x1ec` and
  zero through +0x1e8 (`clr.b` at 0xffedb7e0, `clr.b` at 0xffedb7e6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedb79e | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffedb7a0 | store | 1 | `[0x300002d4] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb7a4 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffedb7a6 | store | 1 | `[0x300002d4] +0x1e4` | `0x13` |
| `movea.l` at 0xffedb7aa | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffedb7ac | store | 1 | `[0x300002d4] +0x1e0` | `0x0` |
| `movea.l` at 0xffedb7b0 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffedb7b2 | store | 1 | `[0x300002d4] +0x1e8` | `0x80` |
| `movea.l` at 0xffedb7b6 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffedb7ba | load | 1 | `[0x300002d4] +0x1ec` | `-` |
| `movea.l` at 0xffedb7c4 | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffedb7c6 | store | 1 | `[0x300002d4] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb7ca | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffedb7cc | store | 1 | `[0x300002d4] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedb7d0 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffedb7d2 | load | 1 | `[0x300002d4] +0x1f0` | `-` |
| `movea.l` at 0xffedb7d6 | load | 4 | `abs 0x300002d4` | `-` |
| `move.b` at 0xffedb7da | load | 1 | `[0x300002d4] +0x1ec` | `-` |
| `movea.l` at 0xffedb7de | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffedb7e0 | store | 1 | `[0x300002d4] +0x1ec` | `0x0` |
| `movea.l` at 0xffedb7e4 | load | 4 | `abs 0x300002d4` | `-` |
| `clr.b` at 0xffedb7e6 | store | 1 | `[0x300002d4] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedb80a | `0xffecd328` | `0x9` | `0xffedbfe3` | `0x14d` | `0x0` | `0xffedc009` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedb7c2 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedb7ee | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedb81c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-0/controller-write-loops-facts.txt).

## 6.5.17:l1#1 at 0xffe7d164

- `0xffe7d164`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#1`, version `1.14.1`. Payload file offset 0xee8b4.
  Load 0xffe00000. Analysed span 0xffe7d164 to 0xffe7d202.
  Established. The last two byte-clear sites target `[0x30000370] +0x1ec` and `[0x30000370] +0x1e8`
  (`clr.b` at 0xffe7d1c4, `clr.b` at 0xffe7d1ca). They lie on the nonzero-mask fall-through of
  `beq.b`, whose zero-mask path goes to the restore tail (`beq.b` at 0xffe7d1a6). Each of those
  clear sites has its own prior four-byte base load at 0x30000370 (`movea.l` at 0xffe7d1c2,
  `movea.l` at 0xffe7d1c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe7d182 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe7d184 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xffe7d188 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe7d18a | store | 1 | `[0x30000370] +0x1e4` | `0x13` |
| `movea.l` at 0xffe7d18e | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe7d190 | store | 1 | `[0x30000370] +0x1e0` | `0x0` |
| `movea.l` at 0xffe7d194 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe7d196 | store | 1 | `[0x30000370] +0x1e8` | `0x80` |
| `movea.l` at 0xffe7d19a | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe7d19e | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xffe7d1a8 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe7d1aa | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xffe7d1ae | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe7d1b0 | store | 1 | `[0x30000370] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe7d1b4 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe7d1b6 | load | 1 | `[0x30000370] +0x1f0` | `-` |
| `movea.l` at 0xffe7d1ba | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe7d1be | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xffe7d1c2 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe7d1c4 | store | 1 | `[0x30000370] +0x1ec` | `0x0` |
| `movea.l` at 0xffe7d1c8 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe7d1ca | store | 1 | `[0x30000370] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe7d1ee | `0xffe6da54` | `0x9` | `0xffe7d9c7` | `0x14d` | `0x0` | `0xffe7d9ed` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe7d1a6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe7d1d2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe7d200 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-1/controller-write-loops-facts.txt).

## 6.5.17:l1#1 at 0xfff6d164

- `0xfff6d164`, `6.5.17/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.17:l1#1`, version `1.14.1`. Payload file offset 0xee8b4.
  Load 0xffe00000. Analysed span 0xfff6d164 to 0xfff6d202.
  Established. The call to 0xfff5da54 receives candidates 9, 0xfff6d9c7, 0x14d, zero, 0xfff6d9ed and
  unknown (`jsr` at 0xfff6d1ee). Before that call, the backward `blt.b` can return to the first
  base-load site, with its flag operands unresolved (`blt.b` at 0xfff6d1d2). The call's purpose is
  not established by these candidate values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff6d182 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xfff6d184 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xfff6d188 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xfff6d18a | store | 1 | `[0x30000370] +0x1e4` | `0x13` |
| `movea.l` at 0xfff6d18e | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xfff6d190 | store | 1 | `[0x30000370] +0x1e0` | `0x0` |
| `movea.l` at 0xfff6d194 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xfff6d196 | store | 1 | `[0x30000370] +0x1e8` | `0x80` |
| `movea.l` at 0xfff6d19a | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xfff6d19e | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xfff6d1a8 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xfff6d1aa | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xfff6d1ae | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xfff6d1b0 | store | 1 | `[0x30000370] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff6d1b4 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xfff6d1b6 | load | 1 | `[0x30000370] +0x1f0` | `-` |
| `movea.l` at 0xfff6d1ba | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xfff6d1be | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xfff6d1c2 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xfff6d1c4 | store | 1 | `[0x30000370] +0x1ec` | `0x0` |
| `movea.l` at 0xfff6d1c8 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xfff6d1ca | store | 1 | `[0x30000370] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff6d1ee | `0xfff5da54` | `0x9` | `0xfff6d9c7` | `0x14d` | `0x0` | `0xfff6d9ed` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff6d1a6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff6d1d2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff6d200 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.14.1-package-1/controller-write-loops-facts.txt).

## 6.5.18:l1#0 at 0xffe6a664

- `0xffe6a664`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.18:l1#0`, version `1.16.0`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe6a664 to 0xffe6a702.
  Established. The constant loads include A1 set to 0x3e8 and D4 set to 0x13 (`movea.w` at
  0xffe6a672, `moveq` at 0xffe6a67a). A byte 0x13 is then written through `[0x3000029c] +0x1e4`
  (`move.b` at 0xffe6a68a). The backward `blt.b` retains unknown condition operands, and the
  bit-test `beq.b` takes the restore tail for `(? & 0x20) == 0` (`blt.b` at 0xffe6a6d2, `beq.b` at
  0xffe6a6a6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6a682 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffe6a684 | store | 1 | `[0x3000029c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6a688 | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffe6a68a | store | 1 | `[0x3000029c] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6a68e | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffe6a690 | store | 1 | `[0x3000029c] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6a694 | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffe6a696 | store | 1 | `[0x3000029c] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6a69a | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffe6a69e | load | 1 | `[0x3000029c] +0x1ec` | `-` |
| `movea.l` at 0xffe6a6a8 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffe6a6aa | store | 1 | `[0x3000029c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6a6ae | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffe6a6b0 | store | 1 | `[0x3000029c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6a6b4 | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffe6a6b6 | load | 1 | `[0x3000029c] +0x1f0` | `-` |
| `movea.l` at 0xffe6a6ba | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffe6a6be | load | 1 | `[0x3000029c] +0x1ec` | `-` |
| `movea.l` at 0xffe6a6c2 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffe6a6c4 | store | 1 | `[0x3000029c] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6a6c8 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffe6a6ca | store | 1 | `[0x3000029c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6a6ee | `0xffe5453c` | `0x9` | `0xffe6aec7` | `0x14d` | `0x0` | `0xffe6aeed` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6a6a6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6a6d2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6a700 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.16.0-package-0/controller-write-loops-facts.txt).

## 6.5.18:l1#0 at 0xffeda664

- `0xffeda664`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.18:l1#0`, version `1.16.0`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffeda664 to 0xffeda702.
  Established. The restore tail uses `movem.l`, followed by frame removal and an unresolved D0
  return (`movem.l` at 0xffeda6f8, `unlk` at 0xffeda6fe, `rts` at 0xffeda700). The early `beq.b`
  reaches this tail for `(? & 0x20) == 0`, while its nonzero-mask fall-through reaches the remaining
  write region (`beq.b` at 0xffeda6a6). The call in that later region has target 0xffec453c (`jsr`
  at 0xffeda6ee).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffeda682 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffeda684 | store | 1 | `[0x3000029c] +0x1e8` | `0x0` |
| `movea.l` at 0xffeda688 | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffeda68a | store | 1 | `[0x3000029c] +0x1e4` | `0x13` |
| `movea.l` at 0xffeda68e | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffeda690 | store | 1 | `[0x3000029c] +0x1e0` | `0x0` |
| `movea.l` at 0xffeda694 | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffeda696 | store | 1 | `[0x3000029c] +0x1e8` | `0x80` |
| `movea.l` at 0xffeda69a | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffeda69e | load | 1 | `[0x3000029c] +0x1ec` | `-` |
| `movea.l` at 0xffeda6a8 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffeda6aa | store | 1 | `[0x3000029c] +0x1e8` | `0x0` |
| `movea.l` at 0xffeda6ae | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffeda6b0 | store | 1 | `[0x3000029c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffeda6b4 | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffeda6b6 | load | 1 | `[0x3000029c] +0x1f0` | `-` |
| `movea.l` at 0xffeda6ba | load | 4 | `abs 0x3000029c` | `-` |
| `move.b` at 0xffeda6be | load | 1 | `[0x3000029c] +0x1ec` | `-` |
| `movea.l` at 0xffeda6c2 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffeda6c4 | store | 1 | `[0x3000029c] +0x1ec` | `0x0` |
| `movea.l` at 0xffeda6c8 | load | 4 | `abs 0x3000029c` | `-` |
| `clr.b` at 0xffeda6ca | store | 1 | `[0x3000029c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffeda6ee | `0xffec453c` | `0x9` | `0xffedaec7` | `0x14d` | `0x0` | `0xffedaeed` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffeda6a6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffeda6d2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffeda700 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.16.0-package-0/controller-write-loops-facts.txt).

## 6.5.18:l1#1 at 0xffe7e690

- `0xffe7e690`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.18:l1#1`, version `1.16.0`. Payload file offset 0xecca0.
  Load 0xffe00000. Analysed span 0xffe7e690 to 0xffe7e72e.
  Established. The initial write region uses four separate base reads at 0x30000340 for byte values
  zero, 0x13, zero and 0x80 (`movea.l` at 0xffe7e6ae, `clr.b` at 0xffe7e6b0, `movea.l` at
  0xffe7e6b4, `move.b` at 0xffe7e6b6, `movea.l` at 0xffe7e6ba, `clr.b` at 0xffe7e6bc, `movea.l` at
  0xffe7e6c0, `move.b` at 0xffe7e6c2). The later call targets 0xffe63310 with candidates 9,
  0xffe7eef3, 0x14d, zero, 0xffe7ef19 and unknown (`jsr` at 0xffe7e71a). The early `beq.b` skips the
  later write region and call for `(? & 0x20) == 0` (`beq.b` at 0xffe7e6d2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe7e6ae | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xffe7e6b0 | store | 1 | `[0x30000340] +0x1e8` | `0x0` |
| `movea.l` at 0xffe7e6b4 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xffe7e6b6 | store | 1 | `[0x30000340] +0x1e4` | `0x13` |
| `movea.l` at 0xffe7e6ba | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xffe7e6bc | store | 1 | `[0x30000340] +0x1e0` | `0x0` |
| `movea.l` at 0xffe7e6c0 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xffe7e6c2 | store | 1 | `[0x30000340] +0x1e8` | `0x80` |
| `movea.l` at 0xffe7e6c6 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xffe7e6ca | load | 1 | `[0x30000340] +0x1ec` | `-` |
| `movea.l` at 0xffe7e6d4 | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xffe7e6d6 | store | 1 | `[0x30000340] +0x1e8` | `0x0` |
| `movea.l` at 0xffe7e6da | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xffe7e6dc | store | 1 | `[0x30000340] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe7e6e0 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xffe7e6e2 | load | 1 | `[0x30000340] +0x1f0` | `-` |
| `movea.l` at 0xffe7e6e6 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xffe7e6ea | load | 1 | `[0x30000340] +0x1ec` | `-` |
| `movea.l` at 0xffe7e6ee | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xffe7e6f0 | store | 1 | `[0x30000340] +0x1ec` | `0x0` |
| `movea.l` at 0xffe7e6f4 | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xffe7e6f6 | store | 1 | `[0x30000340] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe7e71a | `0xffe63310` | `0x9` | `0xffe7eef3` | `0x14d` | `0x0` | `0xffe7ef19` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe7e6d2 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe7e6fe | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe7e72c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.16.0-package-1/controller-write-loops-facts.txt).
