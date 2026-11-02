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

## 6.5.18:l1#1 at 0xfff6e690

- `0xfff6e690`, `6.5.18/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.18:l1#1`, version `1.16.0`. Payload file offset 0xecca0.
  Load 0xffe00000. Analysed span 0xfff6e690 to 0xfff6e72e.
  Established. The call target is 0xfff53310, with candidates 9, 0xfff6eef3, 0x14d, zero, 0xfff6ef19
  and unknown (`jsr` at 0xfff6e71a). The early `beq.b` can bypass the later stores and this call
  when `(? & 0x20) == 0` (`beq.b` at 0xfff6e6d2). Before that decision, byte 0x13 goes through
  `[0x30000340] +0x1e4` and byte 0x80 through +0x1e8 (`move.b` at 0xfff6e6b6, `move.b` at
  0xfff6e6c2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff6e6ae | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xfff6e6b0 | store | 1 | `[0x30000340] +0x1e8` | `0x0` |
| `movea.l` at 0xfff6e6b4 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xfff6e6b6 | store | 1 | `[0x30000340] +0x1e4` | `0x13` |
| `movea.l` at 0xfff6e6ba | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xfff6e6bc | store | 1 | `[0x30000340] +0x1e0` | `0x0` |
| `movea.l` at 0xfff6e6c0 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xfff6e6c2 | store | 1 | `[0x30000340] +0x1e8` | `0x80` |
| `movea.l` at 0xfff6e6c6 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xfff6e6ca | load | 1 | `[0x30000340] +0x1ec` | `-` |
| `movea.l` at 0xfff6e6d4 | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xfff6e6d6 | store | 1 | `[0x30000340] +0x1e8` | `0x0` |
| `movea.l` at 0xfff6e6da | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xfff6e6dc | store | 1 | `[0x30000340] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff6e6e0 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xfff6e6e2 | load | 1 | `[0x30000340] +0x1f0` | `-` |
| `movea.l` at 0xfff6e6e6 | load | 4 | `abs 0x30000340` | `-` |
| `move.b` at 0xfff6e6ea | load | 1 | `[0x30000340] +0x1ec` | `-` |
| `movea.l` at 0xfff6e6ee | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xfff6e6f0 | store | 1 | `[0x30000340] +0x1ec` | `0x0` |
| `movea.l` at 0xfff6e6f4 | load | 4 | `abs 0x30000340` | `-` |
| `clr.b` at 0xfff6e6f6 | store | 1 | `[0x30000340] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff6e71a | `0xfff53310` | `0x9` | `0xfff6eef3` | `0x14d` | `0x0` | `0xfff6ef19` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff6e6d2 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff6e6fe | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff6e72c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.16.0-package-1/controller-write-loops-facts.txt).

## 6.5.19:l1#0 at 0xffe6bb40

- `0xffe6bb40`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.19:l1#0`, version `1.18.3`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe6bb40 to 0xffe6bbde.
  Established. The initial store sites write bytes zero, 0x13, zero and 0x80 through `[0x30000370]
  +0x1e8`, +0x1e4, +0x1e0 and +0x1e8 respectively (`clr.b` at 0xffe6bb60, `move.b` at 0xffe6bb66,
  `clr.b` at 0xffe6bb6c, `move.b` at 0xffe6bb72). The first branch takes its restore route for `(? &
  0x20) == 0` (`beq.b` at 0xffe6bb82); its nonzero-mask fall-through includes byte 0xa0 through
  `[0x30000370] +0x1e8` (`beq.b` at 0xffe6bb82, `move.b` at 0xffe6bb8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6bb5e | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe6bb60 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6bb64 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe6bb66 | store | 1 | `[0x30000370] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6bb6a | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe6bb6c | store | 1 | `[0x30000370] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6bb70 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe6bb72 | store | 1 | `[0x30000370] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6bb76 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe6bb7a | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xffe6bb84 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe6bb86 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6bb8a | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe6bb8c | store | 1 | `[0x30000370] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6bb90 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe6bb92 | load | 1 | `[0x30000370] +0x1f0` | `-` |
| `movea.l` at 0xffe6bb96 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffe6bb9a | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xffe6bb9e | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe6bba0 | store | 1 | `[0x30000370] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6bba4 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffe6bba6 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6bbca | `0xffe52560` | `0x9` | `0xffe6c3c9` | `0x162` | `0x0` | `0xffe6c3ef` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6bb82 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6bbae | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6bbdc | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.18.3-package-0/controller-write-loops-facts.txt).

## 6.5.19:l1#0 at 0xffedbb40

- `0xffedbb40`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.19:l1#0`, version `1.18.3`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffedbb40 to 0xffedbbde.
  Established. Two byte-read sites are rendered through `[0x30000370] +0x1ec`, one before `beq.b`
  and the other on its nonzero-mask fall-through (`move.b` at 0xffedbb7a, `beq.b` at 0xffedbb82,
  `move.b` at 0xffedbb9a). The later region also reads a byte through `[0x30000370] +0x1f0`
  (`move.b` at 0xffedbb92); its call to 0xffec2560 has an unknown sixth printed candidate (`jsr` at
  0xffedbbca).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedbb5e | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffedbb60 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xffedbb64 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffedbb66 | store | 1 | `[0x30000370] +0x1e4` | `0x13` |
| `movea.l` at 0xffedbb6a | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffedbb6c | store | 1 | `[0x30000370] +0x1e0` | `0x0` |
| `movea.l` at 0xffedbb70 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffedbb72 | store | 1 | `[0x30000370] +0x1e8` | `0x80` |
| `movea.l` at 0xffedbb76 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffedbb7a | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xffedbb84 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffedbb86 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |
| `movea.l` at 0xffedbb8a | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffedbb8c | store | 1 | `[0x30000370] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedbb90 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffedbb92 | load | 1 | `[0x30000370] +0x1f0` | `-` |
| `movea.l` at 0xffedbb96 | load | 4 | `abs 0x30000370` | `-` |
| `move.b` at 0xffedbb9a | load | 1 | `[0x30000370] +0x1ec` | `-` |
| `movea.l` at 0xffedbb9e | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffedbba0 | store | 1 | `[0x30000370] +0x1ec` | `0x0` |
| `movea.l` at 0xffedbba4 | load | 4 | `abs 0x30000370` | `-` |
| `clr.b` at 0xffedbba6 | store | 1 | `[0x30000370] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedbbca | `0xffec2560` | `0x9` | `0xffedc3c9` | `0x162` | `0x0` | `0xffedc3ef` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedbb82 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedbbae | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedbbdc | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.18.3-package-0/controller-write-loops-facts.txt).

## 6.5.19:l1#1 at 0xffe8513c

- `0xffe8513c`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.19:l1#1`, version `1.18.3`. Payload file offset 0xee8c8.
  Load 0xffe00000. Analysed span 0xffe8513c to 0xffe851da.
  Established. Four-byte reads at 0x3000048c precede the initial byte stores through `[0x3000048c]
  +0x1e8` and +0x1e4 (`movea.l` at 0xffe8515a, `clr.b` at 0xffe8515c, `movea.l` at 0xffe85160,
  `move.b` at 0xffe85162). Those two stored values are zero and 0x13 (`clr.b` at 0xffe8515c,
  `move.b` at 0xffe85162). The later direct call targets 0xffe6a884 with first candidate 9 (`jsr` at
  0xffe851c6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe8515a | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xffe8515c | store | 1 | `[0x3000048c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe85160 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xffe85162 | store | 1 | `[0x3000048c] +0x1e4` | `0x13` |
| `movea.l` at 0xffe85166 | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xffe85168 | store | 1 | `[0x3000048c] +0x1e0` | `0x0` |
| `movea.l` at 0xffe8516c | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xffe8516e | store | 1 | `[0x3000048c] +0x1e8` | `0x80` |
| `movea.l` at 0xffe85172 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xffe85176 | load | 1 | `[0x3000048c] +0x1ec` | `-` |
| `movea.l` at 0xffe85180 | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xffe85182 | store | 1 | `[0x3000048c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe85186 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xffe85188 | store | 1 | `[0x3000048c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe8518c | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xffe8518e | load | 1 | `[0x3000048c] +0x1f0` | `-` |
| `movea.l` at 0xffe85192 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xffe85196 | load | 1 | `[0x3000048c] +0x1ec` | `-` |
| `movea.l` at 0xffe8519a | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xffe8519c | store | 1 | `[0x3000048c] +0x1ec` | `0x0` |
| `movea.l` at 0xffe851a0 | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xffe851a2 | store | 1 | `[0x3000048c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe851c6 | `0xffe6a884` | `0x9` | `0xffe859c5` | `0x162` | `0x0` | `0xffe859eb` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe8517e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe851aa | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe851d8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.18.3-package-1/controller-write-loops-facts.txt).

## 6.5.19:l1#1 at 0xfff7513c

- `0xfff7513c`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.19:l1#1`, version `1.18.3`. Payload file offset 0xee8c8.
  Load 0xffe00000. Analysed span 0xfff7513c to 0xfff751da.
  Established. The last two byte stores are zero through `[0x3000048c] +0x1ec` and zero through
  +0x1e8 (`clr.b` at 0xfff7519c, `clr.b` at 0xfff751a2). Each uses its own prior four-byte load
  at 0x3000048c (`movea.l` at 0xfff7519a, `movea.l` at 0xfff751a0). `beq.b` bypasses this later
  write region for `(? & 0x20) == 0` (`beq.b` at 0xfff7517e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff7515a | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xfff7515c | store | 1 | `[0x3000048c] +0x1e8` | `0x0` |
| `movea.l` at 0xfff75160 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xfff75162 | store | 1 | `[0x3000048c] +0x1e4` | `0x13` |
| `movea.l` at 0xfff75166 | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xfff75168 | store | 1 | `[0x3000048c] +0x1e0` | `0x0` |
| `movea.l` at 0xfff7516c | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xfff7516e | store | 1 | `[0x3000048c] +0x1e8` | `0x80` |
| `movea.l` at 0xfff75172 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xfff75176 | load | 1 | `[0x3000048c] +0x1ec` | `-` |
| `movea.l` at 0xfff75180 | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xfff75182 | store | 1 | `[0x3000048c] +0x1e8` | `0x0` |
| `movea.l` at 0xfff75186 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xfff75188 | store | 1 | `[0x3000048c] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff7518c | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xfff7518e | load | 1 | `[0x3000048c] +0x1f0` | `-` |
| `movea.l` at 0xfff75192 | load | 4 | `abs 0x3000048c` | `-` |
| `move.b` at 0xfff75196 | load | 1 | `[0x3000048c] +0x1ec` | `-` |
| `movea.l` at 0xfff7519a | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xfff7519c | store | 1 | `[0x3000048c] +0x1ec` | `0x0` |
| `movea.l` at 0xfff751a0 | load | 4 | `abs 0x3000048c` | `-` |
| `clr.b` at 0xfff751a2 | store | 1 | `[0x3000048c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff751c6 | `0xfff5a884` | `0x9` | `0xfff759c5` | `0x162` | `0x0` | `0xfff759eb` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff7517e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff751aa | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff751d8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.18.3-package-1/controller-write-loops-facts.txt).

## 6.5.20:l1#0 at 0xffe6d004

- `0xffe6d004`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.20:l1#0`, version `1.20.6`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe6d004 to 0xffe6d0a2.
  Established. The two clears before the first conditional branch write byte zero through
  `[0x3000083c] +0x1e8` and +0x1e0 (`clr.b` at 0xffe6d024, `clr.b` at 0xffe6d030). The interleaved
  byte stores carry 0x13 through +0x1e4 and 0x80 through +0x1e8 (`move.b` at 0xffe6d02a, `move.b` at
  0xffe6d036). The `beq.b` takes the restore tail for `(? & 0x20) == 0x0`, with the tested value
  unknown (`beq.b` at 0xffe6d046).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6d022 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffe6d024 | store | 1 | `[0x3000083c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6d028 | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffe6d02a | store | 1 | `[0x3000083c] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6d02e | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffe6d030 | store | 1 | `[0x3000083c] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6d034 | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffe6d036 | store | 1 | `[0x3000083c] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6d03a | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffe6d03e | load | 1 | `[0x3000083c] +0x1ec` | `-` |
| `movea.l` at 0xffe6d048 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffe6d04a | store | 1 | `[0x3000083c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6d04e | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffe6d050 | store | 1 | `[0x3000083c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6d054 | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffe6d056 | load | 1 | `[0x3000083c] +0x1f0` | `-` |
| `movea.l` at 0xffe6d05a | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffe6d05e | load | 1 | `[0x3000083c] +0x1ec` | `-` |
| `movea.l` at 0xffe6d062 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffe6d064 | store | 1 | `[0x3000083c] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6d068 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffe6d06a | store | 1 | `[0x3000083c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6d08e | `0xffe60de0` | `0x9` | `0xffe6d88d` | `0x162` | `0x0` | `0xffe6d8b3` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6d046 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6d072 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6d0a0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.20.6-package-0/controller-write-loops-facts.txt).

## 6.5.20:l1#0 at 0xffedd004

- `0xffedd004`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.20:l1#0`, version `1.20.6`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffedd004 to 0xffedd0a2.
  Established. The early `beq.b` selects the restore tail for `(? & 0x20) == 0`, otherwise falling
  through to another byte clear through `[0x3000083c] +0x1e8` and a byte 0xa0 store through
  `[0x3000083c] +0x1e8` (`beq.b` at 0xffedd046, `clr.b` at 0xffedd04a, `move.b` at 0xffedd050). The
  backward `blt.b` has unresolved signed-condition operands (`blt.b` at 0xffedd072). Falling through
  `blt.b` reaches a call to 0xffed0de0 (`blt.b` at 0xffedd072, `jsr` at 0xffedd08e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedd022 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffedd024 | store | 1 | `[0x3000083c] +0x1e8` | `0x0` |
| `movea.l` at 0xffedd028 | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffedd02a | store | 1 | `[0x3000083c] +0x1e4` | `0x13` |
| `movea.l` at 0xffedd02e | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffedd030 | store | 1 | `[0x3000083c] +0x1e0` | `0x0` |
| `movea.l` at 0xffedd034 | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffedd036 | store | 1 | `[0x3000083c] +0x1e8` | `0x80` |
| `movea.l` at 0xffedd03a | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffedd03e | load | 1 | `[0x3000083c] +0x1ec` | `-` |
| `movea.l` at 0xffedd048 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffedd04a | store | 1 | `[0x3000083c] +0x1e8` | `0x0` |
| `movea.l` at 0xffedd04e | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffedd050 | store | 1 | `[0x3000083c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedd054 | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffedd056 | load | 1 | `[0x3000083c] +0x1f0` | `-` |
| `movea.l` at 0xffedd05a | load | 4 | `abs 0x3000083c` | `-` |
| `move.b` at 0xffedd05e | load | 1 | `[0x3000083c] +0x1ec` | `-` |
| `movea.l` at 0xffedd062 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffedd064 | store | 1 | `[0x3000083c] +0x1ec` | `0x0` |
| `movea.l` at 0xffedd068 | load | 4 | `abs 0x3000083c` | `-` |
| `clr.b` at 0xffedd06a | store | 1 | `[0x3000083c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedd08e | `0xffed0de0` | `0x9` | `0xffedd88d` | `0x162` | `0x0` | `0xffedd8b3` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedd046 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedd072 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedd0a0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.20.6-package-0/controller-write-loops-facts.txt).

## 6.5.20:l1#1 at 0xffe94e84

- `0xffe94e84`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.20:l1#1`, version `1.20.6`. Payload file offset 0xefe40.
  Load 0xffe00000. Analysed span 0xffe94e84 to 0xffe94f22.
  Established. A1 receives constant 0x3e8 before the write loop (`movea.w` at 0xffe94e92). The
  backward `blt.b` condition remains unresolved, so no count is assigned here (`blt.b` at
  0xffe94ef2). The call after that branch's fall-through targets 0xffe7a2ec with candidates 9,
  0xffe9570d, 0x162, zero, 0xffe95733 and unknown (`jsr` at 0xffe94f0e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe94ea2 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xffe94ea4 | store | 1 | `[0x30000974] +0x1e8` | `0x0` |
| `movea.l` at 0xffe94ea8 | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xffe94eaa | store | 1 | `[0x30000974] +0x1e4` | `0x13` |
| `movea.l` at 0xffe94eae | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xffe94eb0 | store | 1 | `[0x30000974] +0x1e0` | `0x0` |
| `movea.l` at 0xffe94eb4 | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xffe94eb6 | store | 1 | `[0x30000974] +0x1e8` | `0x80` |
| `movea.l` at 0xffe94eba | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xffe94ebe | load | 1 | `[0x30000974] +0x1ec` | `-` |
| `movea.l` at 0xffe94ec8 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xffe94eca | store | 1 | `[0x30000974] +0x1e8` | `0x0` |
| `movea.l` at 0xffe94ece | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xffe94ed0 | store | 1 | `[0x30000974] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe94ed4 | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xffe94ed6 | load | 1 | `[0x30000974] +0x1f0` | `-` |
| `movea.l` at 0xffe94eda | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xffe94ede | load | 1 | `[0x30000974] +0x1ec` | `-` |
| `movea.l` at 0xffe94ee2 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xffe94ee4 | store | 1 | `[0x30000974] +0x1ec` | `0x0` |
| `movea.l` at 0xffe94ee8 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xffe94eea | store | 1 | `[0x30000974] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe94f0e | `0xffe7a2ec` | `0x9` | `0xffe9570d` | `0x162` | `0x0` | `0xffe95733` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe94ec6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe94ef2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe94f20 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.20.6-package-1/controller-write-loops-facts.txt).

## 6.5.20:l1#1 at 0xfff84e84

- `0xfff84e84`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.20:l1#1`, version `1.20.6`. Payload file offset 0xefe40.
  Load 0xffe00000. Analysed span 0xfff84e84 to 0xfff84f22.
  Established. The byte stores before `beq.b` include 0x13 through `[0x30000974] +0x1e4` and 0x80
  through +0x1e8 (`move.b` at 0xfff84eaa, `move.b` at 0xfff84eb6, `beq.b` at 0xfff84ec6). A later
  byte-zero store through `[0x30000974] +0x1ec` and another through +0x1e8 lie on the nonzero-mask
  fall-through (`beq.b` at 0xfff84ec6, `clr.b` at 0xfff84ee4, `clr.b` at 0xfff84eea).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff84ea2 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xfff84ea4 | store | 1 | `[0x30000974] +0x1e8` | `0x0` |
| `movea.l` at 0xfff84ea8 | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xfff84eaa | store | 1 | `[0x30000974] +0x1e4` | `0x13` |
| `movea.l` at 0xfff84eae | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xfff84eb0 | store | 1 | `[0x30000974] +0x1e0` | `0x0` |
| `movea.l` at 0xfff84eb4 | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xfff84eb6 | store | 1 | `[0x30000974] +0x1e8` | `0x80` |
| `movea.l` at 0xfff84eba | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xfff84ebe | load | 1 | `[0x30000974] +0x1ec` | `-` |
| `movea.l` at 0xfff84ec8 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xfff84eca | store | 1 | `[0x30000974] +0x1e8` | `0x0` |
| `movea.l` at 0xfff84ece | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xfff84ed0 | store | 1 | `[0x30000974] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff84ed4 | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xfff84ed6 | load | 1 | `[0x30000974] +0x1f0` | `-` |
| `movea.l` at 0xfff84eda | load | 4 | `abs 0x30000974` | `-` |
| `move.b` at 0xfff84ede | load | 1 | `[0x30000974] +0x1ec` | `-` |
| `movea.l` at 0xfff84ee2 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xfff84ee4 | store | 1 | `[0x30000974] +0x1ec` | `0x0` |
| `movea.l` at 0xfff84ee8 | load | 4 | `abs 0x30000974` | `-` |
| `clr.b` at 0xfff84eea | store | 1 | `[0x30000974] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff84f0e | `0xfff6a2ec` | `0x9` | `0xfff8570d` | `0x162` | `0x0` | `0xfff85733` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff84ec6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff84ef2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff84f20 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.20.6-package-1/controller-write-loops-facts.txt).

## 6.5.21:l1#0 at 0xffe6cfc8

- `0xffe6cfc8`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.21:l1#0`, version `1.22.2`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffe6cfc8 to 0xffe6d066.
  Established. The first four-byte base read at 0x30000324 feeds the byte-zero store through
  `[0x30000324] +0x1e8` (`movea.l` at 0xffe6cfe6, `clr.b` at 0xffe6cfe8). Separate four-byte loads
  at 0x30000324 precede the byte 0x13 store through `[0x30000324] +0x1e4` and the byte-zero store
  through `[0x30000324] +0x1e0` (`movea.l` at 0xffe6cfec, `move.b` at 0xffe6cfee, `movea.l` at
  0xffe6cff2, `clr.b` at 0xffe6cff4). The early `beq.b` takes its skip for `(? & 0x20) == 0`
  (`beq.b` at 0xffe6d00a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6cfe6 | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffe6cfe8 | store | 1 | `[0x30000324] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6cfec | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffe6cfee | store | 1 | `[0x30000324] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6cff2 | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffe6cff4 | store | 1 | `[0x30000324] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6cff8 | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffe6cffa | store | 1 | `[0x30000324] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6cffe | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffe6d002 | load | 1 | `[0x30000324] +0x1ec` | `-` |
| `movea.l` at 0xffe6d00c | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffe6d00e | store | 1 | `[0x30000324] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6d012 | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffe6d014 | store | 1 | `[0x30000324] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6d018 | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffe6d01a | load | 1 | `[0x30000324] +0x1f0` | `-` |
| `movea.l` at 0xffe6d01e | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffe6d022 | load | 1 | `[0x30000324] +0x1ec` | `-` |
| `movea.l` at 0xffe6d026 | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffe6d028 | store | 1 | `[0x30000324] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6d02c | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffe6d02e | store | 1 | `[0x30000324] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6d052 | `0xffe5aed0` | `0x9` | `0xffe6d851` | `0x162` | `0x0` | `0xffe6d877` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6d00a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6d036 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6d064 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.22.2-package-0/controller-write-loops-facts.txt).

## 6.5.21:l1#0 at 0xffedcfc8

- `0xffedcfc8`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.21:l1#0`, version `1.22.2`. Payload file offset 0x40.
  Load 0xffe00000. Analysed span 0xffedcfc8 to 0xffedd066.
  Established. The direct call targets 0xffecaed0 with candidates 9, 0xffedd851, 0x162, zero,
  0xffedd877 and unknown (`jsr` at 0xffedd052). The backward `blt.b` reaches the first base-load
  site when its unresolved signed-condition flags select the taken arm (`blt.b` at 0xffedd036); its
  other arm reaches the call's setup region (`blt.b` at 0xffedd036).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedcfe6 | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffedcfe8 | store | 1 | `[0x30000324] +0x1e8` | `0x0` |
| `movea.l` at 0xffedcfec | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffedcfee | store | 1 | `[0x30000324] +0x1e4` | `0x13` |
| `movea.l` at 0xffedcff2 | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffedcff4 | store | 1 | `[0x30000324] +0x1e0` | `0x0` |
| `movea.l` at 0xffedcff8 | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffedcffa | store | 1 | `[0x30000324] +0x1e8` | `0x80` |
| `movea.l` at 0xffedcffe | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffedd002 | load | 1 | `[0x30000324] +0x1ec` | `-` |
| `movea.l` at 0xffedd00c | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffedd00e | store | 1 | `[0x30000324] +0x1e8` | `0x0` |
| `movea.l` at 0xffedd012 | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffedd014 | store | 1 | `[0x30000324] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedd018 | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffedd01a | load | 1 | `[0x30000324] +0x1f0` | `-` |
| `movea.l` at 0xffedd01e | load | 4 | `abs 0x30000324` | `-` |
| `move.b` at 0xffedd022 | load | 1 | `[0x30000324] +0x1ec` | `-` |
| `movea.l` at 0xffedd026 | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffedd028 | store | 1 | `[0x30000324] +0x1ec` | `0x0` |
| `movea.l` at 0xffedd02c | load | 4 | `abs 0x30000324` | `-` |
| `clr.b` at 0xffedd02e | store | 1 | `[0x30000324] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedd052 | `0xffecaed0` | `0x9` | `0xffedd851` | `0x162` | `0x0` | `0xffedd877` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedd00a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedd036 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedd064 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.22.2-package-0/controller-write-loops-facts.txt).

## 6.5.21:l1#1 at 0xffe98618

- `0xffe98618`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.21:l1#1`, version `1.22.2`. Payload file offset 0xef9fc.
  Load 0xffe00000. Analysed span 0xffe98618 to 0xffe986b6.
  Established. The bit-test `beq.b` takes the register-restore path for `(? & 0x20) == 0` (`beq.b`
  at 0xffe9865a). The nonzero-mask route clears a byte through `[0x300004f8] +0x1e8`, writes byte
  0xa0 through `[0x300004f8] +0x1e8`, and later clears a byte through +0x1ec (`beq.b` at 0xffe9865a,
  `clr.b` at 0xffe9865e, `move.b` at 0xffe98664, `clr.b` at 0xffe98678).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe98636 | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xffe98638 | store | 1 | `[0x300004f8] +0x1e8` | `0x0` |
| `movea.l` at 0xffe9863c | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xffe9863e | store | 1 | `[0x300004f8] +0x1e4` | `0x13` |
| `movea.l` at 0xffe98642 | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xffe98644 | store | 1 | `[0x300004f8] +0x1e0` | `0x0` |
| `movea.l` at 0xffe98648 | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xffe9864a | store | 1 | `[0x300004f8] +0x1e8` | `0x80` |
| `movea.l` at 0xffe9864e | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xffe98652 | load | 1 | `[0x300004f8] +0x1ec` | `-` |
| `movea.l` at 0xffe9865c | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xffe9865e | store | 1 | `[0x300004f8] +0x1e8` | `0x0` |
| `movea.l` at 0xffe98662 | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xffe98664 | store | 1 | `[0x300004f8] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe98668 | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xffe9866a | load | 1 | `[0x300004f8] +0x1f0` | `-` |
| `movea.l` at 0xffe9866e | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xffe98672 | load | 1 | `[0x300004f8] +0x1ec` | `-` |
| `movea.l` at 0xffe98676 | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xffe98678 | store | 1 | `[0x300004f8] +0x1ec` | `0x0` |
| `movea.l` at 0xffe9867c | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xffe9867e | store | 1 | `[0x300004f8] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe986a2 | `0xffe7a40c` | `0x9` | `0xffe98ea1` | `0x162` | `0x0` | `0xffe98ec7` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe9865a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe98686 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe986b4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.22.2-package-1/controller-write-loops-facts.txt).

## 6.5.21:l1#1 at 0xfff88618

- `0xfff88618`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.21:l1#1`, version `1.22.2`. Payload file offset 0xef9fc.
  Load 0xffe00000. Analysed span 0xfff88618 to 0xfff886b6.
  Established. Two byte reads use `[0x300004f8] +0x1ec`, with the first before `beq.b` and the
  second on its nonzero-mask fall-through (`move.b` at 0xfff88652, `beq.b` at 0xfff8865a, `move.b`
  at 0xfff88672). That later region has a byte read through `[0x300004f8] +0x1f0` between the two
  sites (`move.b` at 0xfff8866a). The sixth printed candidate at the subsequent call is unknown
  (`jsr` at 0xfff886a2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff88636 | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xfff88638 | store | 1 | `[0x300004f8] +0x1e8` | `0x0` |
| `movea.l` at 0xfff8863c | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xfff8863e | store | 1 | `[0x300004f8] +0x1e4` | `0x13` |
| `movea.l` at 0xfff88642 | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xfff88644 | store | 1 | `[0x300004f8] +0x1e0` | `0x0` |
| `movea.l` at 0xfff88648 | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xfff8864a | store | 1 | `[0x300004f8] +0x1e8` | `0x80` |
| `movea.l` at 0xfff8864e | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xfff88652 | load | 1 | `[0x300004f8] +0x1ec` | `-` |
| `movea.l` at 0xfff8865c | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xfff8865e | store | 1 | `[0x300004f8] +0x1e8` | `0x0` |
| `movea.l` at 0xfff88662 | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xfff88664 | store | 1 | `[0x300004f8] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff88668 | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xfff8866a | load | 1 | `[0x300004f8] +0x1f0` | `-` |
| `movea.l` at 0xfff8866e | load | 4 | `abs 0x300004f8` | `-` |
| `move.b` at 0xfff88672 | load | 1 | `[0x300004f8] +0x1ec` | `-` |
| `movea.l` at 0xfff88676 | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xfff88678 | store | 1 | `[0x300004f8] +0x1ec` | `0x0` |
| `movea.l` at 0xfff8867c | load | 4 | `abs 0x300004f8` | `-` |
| `clr.b` at 0xfff8867e | store | 1 | `[0x300004f8] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff886a2 | `0xfff6a40c` | `0x9` | `0xfff88ea1` | `0x162` | `0x0` | `0xfff88ec7` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff8865a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff88686 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff886b4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.22.2-package-1/controller-write-loops-facts.txt).

## 6.5.22:l1#0 at 0xffe6c250

- `0xffe6c250`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.22:l1#0`, version `1.24.8`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffe6c250 to 0xffe6c2ee.
  Established. The byte write of 0x13 goes through `[0x300008b4] +0x1e4`, followed by byte zero
  through +0x1e0 and byte 0x80 through +0x1e8 (`move.b` at 0xffe6c276, `clr.b` at 0xffe6c27c,
  `move.b` at 0xffe6c282). The call target later in the body is 0xffe5ba2c, with the third candidate
  0x182 (`jsr` at 0xffe6c2da). The `beq.b` route for `(? & 0x20) == 0x0` bypasses the later stores
  and call (`beq.b` at 0xffe6c292).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6c26e | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffe6c270 | store | 1 | `[0x300008b4] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c274 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffe6c276 | store | 1 | `[0x300008b4] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6c27a | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffe6c27c | store | 1 | `[0x300008b4] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6c280 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffe6c282 | store | 1 | `[0x300008b4] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6c286 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffe6c28a | load | 1 | `[0x300008b4] +0x1ec` | `-` |
| `movea.l` at 0xffe6c294 | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffe6c296 | store | 1 | `[0x300008b4] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c29a | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffe6c29c | store | 1 | `[0x300008b4] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6c2a0 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffe6c2a2 | load | 1 | `[0x300008b4] +0x1f0` | `-` |
| `movea.l` at 0xffe6c2a6 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffe6c2aa | load | 1 | `[0x300008b4] +0x1ec` | `-` |
| `movea.l` at 0xffe6c2ae | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffe6c2b0 | store | 1 | `[0x300008b4] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6c2b4 | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffe6c2b6 | store | 1 | `[0x300008b4] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6c2da | `0xffe5ba2c` | `0x9` | `0xffe6cacd` | `0x182` | `0x0` | `0xffe6caf3` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c292 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6c2be | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c2ec | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.24.8-package-0/controller-write-loops-facts.txt).

## 6.5.22:l1#0 at 0xffedc250

- `0xffedc250`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.22:l1#0`, version `1.24.8`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffedc250 to 0xffedc2ee.
  Established. The `beq.b` restore decision is `(? & 0x20) == 0x0`, with its tested value unknown,
  and the backward `blt.b` retains unresolved signed-condition operands (`beq.b` at 0xffedc292,
  `blt.b` at 0xffedc2be). The call after the backward branch's fall-through targets 0xffecba2c
  (`jsr` at 0xffedc2da). The recorded D0 return is unknown (`rts` at 0xffedc2ec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedc26e | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffedc270 | store | 1 | `[0x300008b4] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc274 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffedc276 | store | 1 | `[0x300008b4] +0x1e4` | `0x13` |
| `movea.l` at 0xffedc27a | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffedc27c | store | 1 | `[0x300008b4] +0x1e0` | `0x0` |
| `movea.l` at 0xffedc280 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffedc282 | store | 1 | `[0x300008b4] +0x1e8` | `0x80` |
| `movea.l` at 0xffedc286 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffedc28a | load | 1 | `[0x300008b4] +0x1ec` | `-` |
| `movea.l` at 0xffedc294 | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffedc296 | store | 1 | `[0x300008b4] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc29a | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffedc29c | store | 1 | `[0x300008b4] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedc2a0 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffedc2a2 | load | 1 | `[0x300008b4] +0x1f0` | `-` |
| `movea.l` at 0xffedc2a6 | load | 4 | `abs 0x300008b4` | `-` |
| `move.b` at 0xffedc2aa | load | 1 | `[0x300008b4] +0x1ec` | `-` |
| `movea.l` at 0xffedc2ae | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffedc2b0 | store | 1 | `[0x300008b4] +0x1ec` | `0x0` |
| `movea.l` at 0xffedc2b4 | load | 4 | `abs 0x300008b4` | `-` |
| `clr.b` at 0xffedc2b6 | store | 1 | `[0x300008b4] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedc2da | `0xffecba2c` | `0x9` | `0xffedcacd` | `0x182` | `0x0` | `0xffedcaf3` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc292 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedc2be | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc2ec | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.24.8-package-0/controller-write-loops-facts.txt).

## 6.5.22:l1#1 at 0xffe6aff0

- `0xffe6aff0`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.22:l1#1`, version `1.24.8`. Payload file offset 0xeec5c.
  Load 0xffe00000. Analysed span 0xffe6aff0 to 0xffe6b08e.
  Established. The initial byte stores carry zero through `[0x3000094c] +0x1e8`, 0x13 through
  +0x1e4, zero through +0x1e0 and 0x80 through +0x1e8 (`clr.b` at 0xffe6b010, `move.b` at
  0xffe6b016, `clr.b` at 0xffe6b01c, `move.b` at 0xffe6b022). The subsequent `beq.b` takes the
  restore path for `(? & 0x20) == 0x0`, with the tested value unknown (`beq.b` at 0xffe6b032).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6b00e | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffe6b010 | store | 1 | `[0x3000094c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b014 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffe6b016 | store | 1 | `[0x3000094c] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6b01a | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffe6b01c | store | 1 | `[0x3000094c] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6b020 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffe6b022 | store | 1 | `[0x3000094c] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6b026 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffe6b02a | load | 1 | `[0x3000094c] +0x1ec` | `-` |
| `movea.l` at 0xffe6b034 | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffe6b036 | store | 1 | `[0x3000094c] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b03a | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffe6b03c | store | 1 | `[0x3000094c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6b040 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffe6b042 | load | 1 | `[0x3000094c] +0x1f0` | `-` |
| `movea.l` at 0xffe6b046 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffe6b04a | load | 1 | `[0x3000094c] +0x1ec` | `-` |
| `movea.l` at 0xffe6b04e | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffe6b050 | store | 1 | `[0x3000094c] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6b054 | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffe6b056 | store | 1 | `[0x3000094c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6b07a | `0xffe5a5c8` | `0x9` | `0xffe6b86d` | `0x182` | `0x0` | `0xffe6b893` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6b032 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6b05e | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6b08c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.24.8-package-1/controller-write-loops-facts.txt).

## 6.5.22:l1#1 at 0xffedaff0

- `0xffedaff0`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.22:l1#1`, version `1.24.8`. Payload file offset 0xeec5c.
  Load 0xffe00000. Analysed span 0xffedaff0 to 0xffedb08e.
  Established. The direct call has target 0xffeca5c8 and candidates 9, 0xffedb86d, 0x182, zero,
  0xffedb893 and unknown (`jsr` at 0xffedb07a). The backward `blt.b` condition retains unresolved
  operands (`blt.b` at 0xffedb05e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedb00e | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffedb010 | store | 1 | `[0x3000094c] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb014 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffedb016 | store | 1 | `[0x3000094c] +0x1e4` | `0x13` |
| `movea.l` at 0xffedb01a | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffedb01c | store | 1 | `[0x3000094c] +0x1e0` | `0x0` |
| `movea.l` at 0xffedb020 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffedb022 | store | 1 | `[0x3000094c] +0x1e8` | `0x80` |
| `movea.l` at 0xffedb026 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffedb02a | load | 1 | `[0x3000094c] +0x1ec` | `-` |
| `movea.l` at 0xffedb034 | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffedb036 | store | 1 | `[0x3000094c] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb03a | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffedb03c | store | 1 | `[0x3000094c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedb040 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffedb042 | load | 1 | `[0x3000094c] +0x1f0` | `-` |
| `movea.l` at 0xffedb046 | load | 4 | `abs 0x3000094c` | `-` |
| `move.b` at 0xffedb04a | load | 1 | `[0x3000094c] +0x1ec` | `-` |
| `movea.l` at 0xffedb04e | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffedb050 | store | 1 | `[0x3000094c] +0x1ec` | `0x0` |
| `movea.l` at 0xffedb054 | load | 4 | `abs 0x3000094c` | `-` |
| `clr.b` at 0xffedb056 | store | 1 | `[0x3000094c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedb07a | `0xffeca5c8` | `0x9` | `0xffedb86d` | `0x182` | `0x0` | `0xffedb893` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedb032 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedb05e | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedb08c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.24.8-package-1/controller-write-loops-facts.txt).

## 6.5.22:l1#2 at 0xffe96dcc

- `0xffe96dcc`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.22:l1#2`, version `1.24.8`. Payload file offset 0x1dc6c0.
  Load 0xffe00000. Analysed span 0xffe96dcc to 0xffe96e6a.
  Established. The byte-read sites render as `[0x30000b08] +0x1ec` at two instructions and
  `[0x30000b08] +0x1f0` at another (`move.b` at 0xffe96e06, `move.b` at 0xffe96e26, `move.b` at
  0xffe96e1e). The later two reads occur on the `beq.b` fall-through for `(? & 0x20) != 0x0`
  (`beq.b` at 0xffe96e0e, `move.b` at 0xffe96e1e, `move.b` at 0xffe96e26).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe96dea | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xffe96dec | store | 1 | `[0x30000b08] +0x1e8` | `0x0` |
| `movea.l` at 0xffe96df0 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xffe96df2 | store | 1 | `[0x30000b08] +0x1e4` | `0x13` |
| `movea.l` at 0xffe96df6 | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xffe96df8 | store | 1 | `[0x30000b08] +0x1e0` | `0x0` |
| `movea.l` at 0xffe96dfc | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xffe96dfe | store | 1 | `[0x30000b08] +0x1e8` | `0x80` |
| `movea.l` at 0xffe96e02 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xffe96e06 | load | 1 | `[0x30000b08] +0x1ec` | `-` |
| `movea.l` at 0xffe96e10 | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xffe96e12 | store | 1 | `[0x30000b08] +0x1e8` | `0x0` |
| `movea.l` at 0xffe96e16 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xffe96e18 | store | 1 | `[0x30000b08] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe96e1c | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xffe96e1e | load | 1 | `[0x30000b08] +0x1f0` | `-` |
| `movea.l` at 0xffe96e22 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xffe96e26 | load | 1 | `[0x30000b08] +0x1ec` | `-` |
| `movea.l` at 0xffe96e2a | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xffe96e2c | store | 1 | `[0x30000b08] +0x1ec` | `0x0` |
| `movea.l` at 0xffe96e30 | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xffe96e32 | store | 1 | `[0x30000b08] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe96e56 | `0xffe7d0fc` | `0x9` | `0xffe97649` | `0x182` | `0x0` | `0xffe9766f` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe96e0e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe96e3a | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe96e68 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.24.8-package-2/controller-write-loops-facts.txt).

## 6.5.22:l1#2 at 0xfff86dcc

- `0xfff86dcc`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.22:l1#2`, version `1.24.8`. Payload file offset 0x1dc6c0.
  Load 0xffe00000. Analysed span 0xfff86dcc to 0xfff86e6a.
  Established. The last byte-zero stores render as `[0x30000b08] +0x1ec` and `[0x30000b08] +0x1e8`
  (`clr.b` at 0xfff86e2c, `clr.b` at 0xfff86e32). They are bypassed by the taken `beq.b` route for
  `(? & 0x20) == 0x0` (`beq.b` at 0xfff86e0e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff86dea | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xfff86dec | store | 1 | `[0x30000b08] +0x1e8` | `0x0` |
| `movea.l` at 0xfff86df0 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xfff86df2 | store | 1 | `[0x30000b08] +0x1e4` | `0x13` |
| `movea.l` at 0xfff86df6 | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xfff86df8 | store | 1 | `[0x30000b08] +0x1e0` | `0x0` |
| `movea.l` at 0xfff86dfc | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xfff86dfe | store | 1 | `[0x30000b08] +0x1e8` | `0x80` |
| `movea.l` at 0xfff86e02 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xfff86e06 | load | 1 | `[0x30000b08] +0x1ec` | `-` |
| `movea.l` at 0xfff86e10 | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xfff86e12 | store | 1 | `[0x30000b08] +0x1e8` | `0x0` |
| `movea.l` at 0xfff86e16 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xfff86e18 | store | 1 | `[0x30000b08] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff86e1c | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xfff86e1e | load | 1 | `[0x30000b08] +0x1f0` | `-` |
| `movea.l` at 0xfff86e22 | load | 4 | `abs 0x30000b08` | `-` |
| `move.b` at 0xfff86e26 | load | 1 | `[0x30000b08] +0x1ec` | `-` |
| `movea.l` at 0xfff86e2a | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xfff86e2c | store | 1 | `[0x30000b08] +0x1ec` | `0x0` |
| `movea.l` at 0xfff86e30 | load | 4 | `abs 0x30000b08` | `-` |
| `clr.b` at 0xfff86e32 | store | 1 | `[0x30000b08] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff86e56 | `0xfff6d0fc` | `0x9` | `0xfff87649` | `0x182` | `0x0` | `0xfff8766f` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff86e0e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff86e3a | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff86e68 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.24.8-package-2/controller-write-loops-facts.txt).

## 6.5.23:l1#0 at 0xffe6c2dc

- `0xffe6c2dc`, `6.5.23/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.23:l1#0`, version `1.26.5`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffe6c2dc to 0xffe6c37a.
  Established. A1 receives the constant 0x3e8 (`movea.w` at 0xffe6c2ea). The later `blt.b` is
  backward, but its condition operands remain unresolved (`blt.b` at 0xffe6c34a). The direct call
  after the branch's fall-through has target 0xffe5b744 (`blt.b` at 0xffe6c34a, `jsr` at
  0xffe6c366).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6c2fa | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6c2fc | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c300 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6c302 | store | 1 | `[0x30000808] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6c306 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6c308 | store | 1 | `[0x30000808] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6c30c | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6c30e | store | 1 | `[0x30000808] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6c312 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6c316 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffe6c320 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6c322 | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c326 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6c328 | store | 1 | `[0x30000808] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6c32c | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6c32e | load | 1 | `[0x30000808] +0x1f0` | `-` |
| `movea.l` at 0xffe6c332 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6c336 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffe6c33a | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6c33c | store | 1 | `[0x30000808] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6c340 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6c342 | store | 1 | `[0x30000808] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6c366 | `0xffe5b744` | `0x9` | `0xffe6cb55` | `0x182` | `0x0` | `0xffe6cb7b` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c31e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6c34a | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c378 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.26.5-package-0/controller-write-loops-facts.txt).

## 6.5.23:l1#0 at 0xffedc2dc

- `0xffedc2dc`, `6.5.23/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.23:l1#0`, version `1.26.5`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffedc2dc to 0xffedc37a.
  Established. The call to 0xffecb744 has first candidate 9, third candidate 0x182 and an unknown
  sixth candidate (`jsr` at 0xffedc366). The early `beq.b` skips that call and the later stores for
  `(? & 0x20) == 0x0` (`beq.b` at 0xffedc31e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedc2fa | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedc2fc | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc300 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedc302 | store | 1 | `[0x30000808] +0x1e4` | `0x13` |
| `movea.l` at 0xffedc306 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedc308 | store | 1 | `[0x30000808] +0x1e0` | `0x0` |
| `movea.l` at 0xffedc30c | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedc30e | store | 1 | `[0x30000808] +0x1e8` | `0x80` |
| `movea.l` at 0xffedc312 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedc316 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffedc320 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedc322 | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc326 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedc328 | store | 1 | `[0x30000808] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedc32c | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedc32e | load | 1 | `[0x30000808] +0x1f0` | `-` |
| `movea.l` at 0xffedc332 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedc336 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffedc33a | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedc33c | store | 1 | `[0x30000808] +0x1ec` | `0x0` |
| `movea.l` at 0xffedc340 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedc342 | store | 1 | `[0x30000808] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedc366 | `0xffecb744` | `0x9` | `0xffedcb55` | `0x182` | `0x0` | `0xffedcb7b` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc31e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedc34a | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc378 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.26.5-package-0/controller-write-loops-facts.txt).

## 6.5.23:l1#1 at 0xffe6bdf4

- `0xffe6bdf4`, `6.5.23/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.23:l1#1`, version `1.26.5`. Payload file offset 0xefd88.
  Load 0xffe00000. Analysed span 0xffe6bdf4 to 0xffe6be92.
  Established. Byte 0x13 through `[0x300008b0] +0x1e4` and byte 0x80 through +0x1e8 are written
  before the bit-test branch (`move.b` at 0xffe6be1a, `move.b` at 0xffe6be26, `beq.b` at
  0xffe6be36). The later byte 0xa0 store through `[0x300008b0] +0x1e8` lies on the branch's
  fall-through for `(? & 0x20) != 0x0` (`beq.b` at 0xffe6be36, `move.b` at 0xffe6be40).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6be12 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6be14 | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6be18 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6be1a | store | 1 | `[0x300008b0] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6be1e | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6be20 | store | 1 | `[0x300008b0] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6be24 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6be26 | store | 1 | `[0x300008b0] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6be2a | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6be2e | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffe6be38 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6be3a | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6be3e | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6be40 | store | 1 | `[0x300008b0] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6be44 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6be46 | load | 1 | `[0x300008b0] +0x1f0` | `-` |
| `movea.l` at 0xffe6be4a | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6be4e | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffe6be52 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6be54 | store | 1 | `[0x300008b0] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6be58 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6be5a | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6be7e | `0xffe5b058` | `0x9` | `0xffe6c66d` | `0x182` | `0x0` | `0xffe6c693` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6be36 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6be62 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6be90 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.26.5-package-1/controller-write-loops-facts.txt).

## 6.5.23:l1#1 at 0xffedbdf4

- `0xffedbdf4`, `6.5.23/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.23:l1#1`, version `1.26.5`. Payload file offset 0xefd88.
  Load 0xffe00000. Analysed span 0xffedbdf4 to 0xffedbe92.
  Established. The recorded D0 exit value is unresolved (`rts` at 0xffedbe90). The call in the later
  region has target 0xffecb058 and first candidate 9 (`jsr` at 0xffedbe7e); its purpose is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedbe12 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedbe14 | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffedbe18 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedbe1a | store | 1 | `[0x300008b0] +0x1e4` | `0x13` |
| `movea.l` at 0xffedbe1e | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedbe20 | store | 1 | `[0x300008b0] +0x1e0` | `0x0` |
| `movea.l` at 0xffedbe24 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedbe26 | store | 1 | `[0x300008b0] +0x1e8` | `0x80` |
| `movea.l` at 0xffedbe2a | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedbe2e | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffedbe38 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedbe3a | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffedbe3e | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedbe40 | store | 1 | `[0x300008b0] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedbe44 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedbe46 | load | 1 | `[0x300008b0] +0x1f0` | `-` |
| `movea.l` at 0xffedbe4a | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedbe4e | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffedbe52 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedbe54 | store | 1 | `[0x300008b0] +0x1ec` | `0x0` |
| `movea.l` at 0xffedbe58 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedbe5a | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedbe7e | `0xffecb058` | `0x9` | `0xffedc66d` | `0x182` | `0x0` | `0xffedc693` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedbe36 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedbe62 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedbe90 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.26.5-package-1/controller-write-loops-facts.txt).

## 6.5.23:l1#2 at 0xffe9b218

- `0xffe9b218`, `6.5.23/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.23:l1#2`, version `1.26.5`. Payload file offset 0x1df6d8.
  Load 0xffe00000. Analysed span 0xffe9b218 to 0xffe9b2b6.
  Established. Falling through `beq.b` for `(? & 0x20) != 0x0` reaches a byte read rendered as
  `[0x30000afc] +0x1f0` (`beq.b` at 0xffe9b25a, `move.b` at 0xffe9b26a). The later byte clears
  render as `[0x30000afc] +0x1ec` and `[0x30000afc] +0x1e8` (`clr.b` at 0xffe9b278, `clr.b` at
  0xffe9b27e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe9b236 | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xffe9b238 | store | 1 | `[0x30000afc] +0x1e8` | `0x0` |
| `movea.l` at 0xffe9b23c | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xffe9b23e | store | 1 | `[0x30000afc] +0x1e4` | `0x13` |
| `movea.l` at 0xffe9b242 | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xffe9b244 | store | 1 | `[0x30000afc] +0x1e0` | `0x0` |
| `movea.l` at 0xffe9b248 | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xffe9b24a | store | 1 | `[0x30000afc] +0x1e8` | `0x80` |
| `movea.l` at 0xffe9b24e | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xffe9b252 | load | 1 | `[0x30000afc] +0x1ec` | `-` |
| `movea.l` at 0xffe9b25c | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xffe9b25e | store | 1 | `[0x30000afc] +0x1e8` | `0x0` |
| `movea.l` at 0xffe9b262 | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xffe9b264 | store | 1 | `[0x30000afc] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe9b268 | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xffe9b26a | load | 1 | `[0x30000afc] +0x1f0` | `-` |
| `movea.l` at 0xffe9b26e | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xffe9b272 | load | 1 | `[0x30000afc] +0x1ec` | `-` |
| `movea.l` at 0xffe9b276 | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xffe9b278 | store | 1 | `[0x30000afc] +0x1ec` | `0x0` |
| `movea.l` at 0xffe9b27c | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xffe9b27e | store | 1 | `[0x30000afc] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe9b2a2 | `0xffe7e890` | `0x9` | `0xffe9ba91` | `0x182` | `0x0` | `0xffe9bab7` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe9b25a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe9b286 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe9b2b4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.26.5-package-2/controller-write-loops-facts.txt).

## 6.5.23:l1#2 at 0xfff8b218

- `0xfff8b218`, `6.5.23/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.23:l1#2`, version `1.26.5`. Payload file offset 0x1df6d8.
  Load 0xffe00000. Analysed span 0xfff8b218 to 0xfff8b2b6.
  Established. Before `beq.b`, the byte store sites use values zero, 0x13, zero and 0x80 through
  `[0x30000afc] +0x1e8`, +0x1e4, +0x1e0 and +0x1e8 respectively (`clr.b` at 0xfff8b238, `move.b` at
  0xfff8b23e, `clr.b` at 0xfff8b244, `move.b` at 0xfff8b24a, `beq.b` at 0xfff8b25a). The branch
  takes the restore path when `(? & 0x20) == 0x0` (`beq.b` at 0xfff8b25a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff8b236 | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xfff8b238 | store | 1 | `[0x30000afc] +0x1e8` | `0x0` |
| `movea.l` at 0xfff8b23c | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xfff8b23e | store | 1 | `[0x30000afc] +0x1e4` | `0x13` |
| `movea.l` at 0xfff8b242 | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xfff8b244 | store | 1 | `[0x30000afc] +0x1e0` | `0x0` |
| `movea.l` at 0xfff8b248 | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xfff8b24a | store | 1 | `[0x30000afc] +0x1e8` | `0x80` |
| `movea.l` at 0xfff8b24e | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xfff8b252 | load | 1 | `[0x30000afc] +0x1ec` | `-` |
| `movea.l` at 0xfff8b25c | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xfff8b25e | store | 1 | `[0x30000afc] +0x1e8` | `0x0` |
| `movea.l` at 0xfff8b262 | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xfff8b264 | store | 1 | `[0x30000afc] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff8b268 | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xfff8b26a | load | 1 | `[0x30000afc] +0x1f0` | `-` |
| `movea.l` at 0xfff8b26e | load | 4 | `abs 0x30000afc` | `-` |
| `move.b` at 0xfff8b272 | load | 1 | `[0x30000afc] +0x1ec` | `-` |
| `movea.l` at 0xfff8b276 | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xfff8b278 | store | 1 | `[0x30000afc] +0x1ec` | `0x0` |
| `movea.l` at 0xfff8b27c | load | 4 | `abs 0x30000afc` | `-` |
| `clr.b` at 0xfff8b27e | store | 1 | `[0x30000afc] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff8b2a2 | `0xfff6e890` | `0x9` | `0xfff8ba91` | `0x182` | `0x0` | `0xfff8bab7` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff8b25a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff8b286 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff8b2b4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.26.5-package-2/controller-write-loops-facts.txt).

## 6.5.24:l1#0 at 0xffe6bc48

- `0xffe6bc48`, `6.5.24/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.24:l1#0`, version `1.28.3`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffe6bc48 to 0xffe6bce6.
  Established. The call candidates at target 0xffe5b1a0 begin 9, 0xffe6c476, 0x180, zero and
  0xffe6c49c, followed by unknown (`jsr` at 0xffe6bcd2). The backward `blt.b` has unresolved
  condition operands (`blt.b` at 0xffe6bcb6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6bc66 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6bc68 | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6bc6c | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6bc6e | store | 1 | `[0x30000808] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6bc72 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6bc74 | store | 1 | `[0x30000808] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6bc78 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6bc7a | store | 1 | `[0x30000808] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6bc7e | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6bc82 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffe6bc8c | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6bc8e | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6bc92 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6bc94 | store | 1 | `[0x30000808] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6bc98 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6bc9a | load | 1 | `[0x30000808] +0x1f0` | `-` |
| `movea.l` at 0xffe6bc9e | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffe6bca2 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffe6bca6 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6bca8 | store | 1 | `[0x30000808] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6bcac | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffe6bcae | store | 1 | `[0x30000808] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6bcd2 | `0xffe5b1a0` | `0x9` | `0xffe6c476` | `0x180` | `0x0` | `0xffe6c49c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6bc8a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6bcb6 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6bce4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.28.3-package-0/controller-write-loops-facts.txt).

## 6.5.24:l1#0 at 0xffedbc48

- `0xffedbc48`, `6.5.24/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.24:l1#0`, version `1.28.3`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffedbc48 to 0xffedbce6.
  Established. The byte write of 0x13 renders as `[0x30000808] +0x1e4`, and the following byte-zero
  store as `[0x30000808] +0x1e0` (`move.b` at 0xffedbc6e, `clr.b` at 0xffedbc74). A1 is separately
  assigned the constant 0x3e8, without establishing a loop count here (`movea.w` at 0xffedbc56).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedbc66 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedbc68 | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffedbc6c | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedbc6e | store | 1 | `[0x30000808] +0x1e4` | `0x13` |
| `movea.l` at 0xffedbc72 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedbc74 | store | 1 | `[0x30000808] +0x1e0` | `0x0` |
| `movea.l` at 0xffedbc78 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedbc7a | store | 1 | `[0x30000808] +0x1e8` | `0x80` |
| `movea.l` at 0xffedbc7e | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedbc82 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffedbc8c | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedbc8e | store | 1 | `[0x30000808] +0x1e8` | `0x0` |
| `movea.l` at 0xffedbc92 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedbc94 | store | 1 | `[0x30000808] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedbc98 | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedbc9a | load | 1 | `[0x30000808] +0x1f0` | `-` |
| `movea.l` at 0xffedbc9e | load | 4 | `abs 0x30000808` | `-` |
| `move.b` at 0xffedbca2 | load | 1 | `[0x30000808] +0x1ec` | `-` |
| `movea.l` at 0xffedbca6 | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedbca8 | store | 1 | `[0x30000808] +0x1ec` | `0x0` |
| `movea.l` at 0xffedbcac | load | 4 | `abs 0x30000808` | `-` |
| `clr.b` at 0xffedbcae | store | 1 | `[0x30000808] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedbcd2 | `0xffecb1a0` | `0x9` | `0xffedc476` | `0x180` | `0x0` | `0xffedc49c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedbc8a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedbcb6 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedbce4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.28.3-package-0/controller-write-loops-facts.txt).

## 6.5.24:l1#1 at 0xffe6b764

- `0xffe6b764`, `6.5.24/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.24:l1#1`, version `1.28.3`. Payload file offset 0xef4d0.
  Load 0xffe00000. Analysed span 0xffe6b764 to 0xffe6b802.
  Established. The `beq.b` takes the restore route for `(? & 0x20) == 0x0` (`beq.b` at 0xffe6b7a6);
  its fall-through for `(? & 0x20) != 0x0` includes byte zero and byte 0xa0 at sites both rendered
  as `[0x300008b0] +0x1e8` (`beq.b` at 0xffe6b7a6, `clr.b` at 0xffe6b7aa, `move.b` at 0xffe6b7b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6b782 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6b784 | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b788 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6b78a | store | 1 | `[0x300008b0] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6b78e | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6b790 | store | 1 | `[0x300008b0] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6b794 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6b796 | store | 1 | `[0x300008b0] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6b79a | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6b79e | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffe6b7a8 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6b7aa | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b7ae | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6b7b0 | store | 1 | `[0x300008b0] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6b7b4 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6b7b6 | load | 1 | `[0x300008b0] +0x1f0` | `-` |
| `movea.l` at 0xffe6b7ba | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffe6b7be | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffe6b7c2 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6b7c4 | store | 1 | `[0x300008b0] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6b7c8 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffe6b7ca | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6b7ee | `0xffe5aab8` | `0x9` | `0xffe6bf92` | `0x180` | `0x0` | `0xffe6bfb8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6b7a6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6b7d2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6b800 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.28.3-package-1/controller-write-loops-facts.txt).

## 6.5.24:l1#1 at 0xffedb764

- `0xffedb764`, `6.5.24/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.24:l1#1`, version `1.28.3`. Payload file offset 0xef4d0.
  Load 0xffe00000. Analysed span 0xffedb764 to 0xffedb802.
  Established. The backward `blt.b` transfers to the first base-load site when its unresolved LT
  condition is taken (`blt.b` at 0xffedb7d2); its fall-through reaches the setup for the call to
  0xffecaab8, whose third candidate is 0x180 (`blt.b` at 0xffedb7d2, `jsr` at 0xffedb7ee).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedb782 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedb784 | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb788 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedb78a | store | 1 | `[0x300008b0] +0x1e4` | `0x13` |
| `movea.l` at 0xffedb78e | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedb790 | store | 1 | `[0x300008b0] +0x1e0` | `0x0` |
| `movea.l` at 0xffedb794 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedb796 | store | 1 | `[0x300008b0] +0x1e8` | `0x80` |
| `movea.l` at 0xffedb79a | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedb79e | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffedb7a8 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedb7aa | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb7ae | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedb7b0 | store | 1 | `[0x300008b0] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedb7b4 | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedb7b6 | load | 1 | `[0x300008b0] +0x1f0` | `-` |
| `movea.l` at 0xffedb7ba | load | 4 | `abs 0x300008b0` | `-` |
| `move.b` at 0xffedb7be | load | 1 | `[0x300008b0] +0x1ec` | `-` |
| `movea.l` at 0xffedb7c2 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedb7c4 | store | 1 | `[0x300008b0] +0x1ec` | `0x0` |
| `movea.l` at 0xffedb7c8 | load | 4 | `abs 0x300008b0` | `-` |
| `clr.b` at 0xffedb7ca | store | 1 | `[0x300008b0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedb7ee | `0xffecaab8` | `0x9` | `0xffedbf92` | `0x180` | `0x0` | `0xffedbfb8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedb7a6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedb7d2 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedb800 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.28.3-package-1/controller-write-loops-facts.txt).

## 6.5.24:l1#2 at 0xffe9b638

- `0xffe9b638`, `6.5.24/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.24:l1#2`, version `1.28.3`. Payload file offset 0x1de568.
  Load 0xffe00000. Analysed span 0xffe9b638 to 0xffe9b6d6.
  Established. The call target is 0xffe7eed0, with first candidates 9, 0xffe9be66 and 0x180 (`jsr`
  at 0xffe9b6c2). The earlier write region includes byte zero through `[0x30000b18] +0x1ec` and byte
  zero through `[0x30000b18] +0x1e8` at its final two non-stack store sites (`clr.b` at 0xffe9b698, `clr.b` at
  0xffe9b69e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe9b656 | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xffe9b658 | store | 1 | `[0x30000b18] +0x1e8` | `0x0` |
| `movea.l` at 0xffe9b65c | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xffe9b65e | store | 1 | `[0x30000b18] +0x1e4` | `0x13` |
| `movea.l` at 0xffe9b662 | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xffe9b664 | store | 1 | `[0x30000b18] +0x1e0` | `0x0` |
| `movea.l` at 0xffe9b668 | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xffe9b66a | store | 1 | `[0x30000b18] +0x1e8` | `0x80` |
| `movea.l` at 0xffe9b66e | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xffe9b672 | load | 1 | `[0x30000b18] +0x1ec` | `-` |
| `movea.l` at 0xffe9b67c | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xffe9b67e | store | 1 | `[0x30000b18] +0x1e8` | `0x0` |
| `movea.l` at 0xffe9b682 | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xffe9b684 | store | 1 | `[0x30000b18] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe9b688 | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xffe9b68a | load | 1 | `[0x30000b18] +0x1f0` | `-` |
| `movea.l` at 0xffe9b68e | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xffe9b692 | load | 1 | `[0x30000b18] +0x1ec` | `-` |
| `movea.l` at 0xffe9b696 | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xffe9b698 | store | 1 | `[0x30000b18] +0x1ec` | `0x0` |
| `movea.l` at 0xffe9b69c | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xffe9b69e | store | 1 | `[0x30000b18] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe9b6c2 | `0xffe7eed0` | `0x9` | `0xffe9be66` | `0x180` | `0x0` | `0xffe9be8c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe9b67a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe9b6a6 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe9b6d4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.28.3-package-2/controller-write-loops-facts.txt).

## 6.5.24:l1#2 at 0xfff8b638

- `0xfff8b638`, `6.5.24/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.24:l1#2`, version `1.28.3`. Payload file offset 0x1de568.
  Load 0xffe00000. Analysed span 0xfff8b638 to 0xfff8b6d6.
  Established. The D0 value at the recorded return is unknown (`rts` at 0xfff8b6d4). The direct call
  has target 0xfff6eed0 with first candidates 9, 0xfff8be66 and 0x180 (`jsr` at 0xfff8b6c2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff8b656 | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xfff8b658 | store | 1 | `[0x30000b18] +0x1e8` | `0x0` |
| `movea.l` at 0xfff8b65c | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xfff8b65e | store | 1 | `[0x30000b18] +0x1e4` | `0x13` |
| `movea.l` at 0xfff8b662 | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xfff8b664 | store | 1 | `[0x30000b18] +0x1e0` | `0x0` |
| `movea.l` at 0xfff8b668 | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xfff8b66a | store | 1 | `[0x30000b18] +0x1e8` | `0x80` |
| `movea.l` at 0xfff8b66e | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xfff8b672 | load | 1 | `[0x30000b18] +0x1ec` | `-` |
| `movea.l` at 0xfff8b67c | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xfff8b67e | store | 1 | `[0x30000b18] +0x1e8` | `0x0` |
| `movea.l` at 0xfff8b682 | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xfff8b684 | store | 1 | `[0x30000b18] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff8b688 | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xfff8b68a | load | 1 | `[0x30000b18] +0x1f0` | `-` |
| `movea.l` at 0xfff8b68e | load | 4 | `abs 0x30000b18` | `-` |
| `move.b` at 0xfff8b692 | load | 1 | `[0x30000b18] +0x1ec` | `-` |
| `movea.l` at 0xfff8b696 | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xfff8b698 | store | 1 | `[0x30000b18] +0x1ec` | `0x0` |
| `movea.l` at 0xfff8b69c | load | 4 | `abs 0x30000b18` | `-` |
| `clr.b` at 0xfff8b69e | store | 1 | `[0x30000b18] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff8b6c2 | `0xfff6eed0` | `0x9` | `0xfff8be66` | `0x180` | `0x0` | `0xfff8be8c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff8b67a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff8b6a6 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff8b6d4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.28.3-package-2/controller-write-loops-facts.txt).

## 6.5.25:l1#0 at 0xffe6c138

- `0xffe6c138`, `6.5.25/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.25:l1#0`, version `1.30.6`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffe6c138 to 0xffe6c1d6.
  Established. Before the bit-test branch, byte stores carry zero through `[0x30000320] +0x1e8`,
  0x13 through +0x1e4, zero through +0x1e0 and 0x80 through +0x1e8 (`clr.b` at 0xffe6c158, `move.b`
  at 0xffe6c15e, `clr.b` at 0xffe6c164, `move.b` at 0xffe6c16a, `beq.b` at 0xffe6c17a). The `beq.b`
  takes the restore tail for `(? & 0x20) == 0x0` (`beq.b` at 0xffe6c17a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6c156 | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffe6c158 | store | 1 | `[0x30000320] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c15c | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffe6c15e | store | 1 | `[0x30000320] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6c162 | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffe6c164 | store | 1 | `[0x30000320] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6c168 | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffe6c16a | store | 1 | `[0x30000320] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6c16e | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffe6c172 | load | 1 | `[0x30000320] +0x1ec` | `-` |
| `movea.l` at 0xffe6c17c | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffe6c17e | store | 1 | `[0x30000320] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c182 | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffe6c184 | store | 1 | `[0x30000320] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6c188 | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffe6c18a | load | 1 | `[0x30000320] +0x1f0` | `-` |
| `movea.l` at 0xffe6c18e | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffe6c192 | load | 1 | `[0x30000320] +0x1ec` | `-` |
| `movea.l` at 0xffe6c196 | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffe6c198 | store | 1 | `[0x30000320] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6c19c | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffe6c19e | store | 1 | `[0x30000320] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6c1c2 | `0xffe58598` | `0x9` | `0xffe6c966` | `0x180` | `0x0` | `0xffe6c98c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c17a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6c1a6 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c1d4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.30.6-package-0/controller-write-loops-facts.txt).

## 6.5.25:l1#0 at 0xffedc138

- `0xffedc138`, `6.5.25/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.25:l1#0`, version `1.30.6`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffedc138 to 0xffedc1d6.
  Established. The `beq.b` fall-through for `(? & 0x20) != 0x0` reaches byte zero and byte 0xa0 at
  sites rendered as `[0x30000320] +0x1e8` (`beq.b` at 0xffedc17a, `clr.b` at 0xffedc17e, `move.b` at
  0xffedc184). Later byte-zero sites render as `[0x30000320] +0x1ec` and `[0x30000320] +0x1e8`
  (`clr.b` at 0xffedc198, `clr.b` at 0xffedc19e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedc156 | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffedc158 | store | 1 | `[0x30000320] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc15c | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffedc15e | store | 1 | `[0x30000320] +0x1e4` | `0x13` |
| `movea.l` at 0xffedc162 | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffedc164 | store | 1 | `[0x30000320] +0x1e0` | `0x0` |
| `movea.l` at 0xffedc168 | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffedc16a | store | 1 | `[0x30000320] +0x1e8` | `0x80` |
| `movea.l` at 0xffedc16e | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffedc172 | load | 1 | `[0x30000320] +0x1ec` | `-` |
| `movea.l` at 0xffedc17c | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffedc17e | store | 1 | `[0x30000320] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc182 | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffedc184 | store | 1 | `[0x30000320] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedc188 | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffedc18a | load | 1 | `[0x30000320] +0x1f0` | `-` |
| `movea.l` at 0xffedc18e | load | 4 | `abs 0x30000320` | `-` |
| `move.b` at 0xffedc192 | load | 1 | `[0x30000320] +0x1ec` | `-` |
| `movea.l` at 0xffedc196 | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffedc198 | store | 1 | `[0x30000320] +0x1ec` | `0x0` |
| `movea.l` at 0xffedc19c | load | 4 | `abs 0x30000320` | `-` |
| `clr.b` at 0xffedc19e | store | 1 | `[0x30000320] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedc1c2 | `0xffec8598` | `0x9` | `0xffedc966` | `0x180` | `0x0` | `0xffedc98c` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc17a | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedc1a6 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc1d4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.30.6-package-0/controller-write-loops-facts.txt).

## 6.5.25:l1#1 at 0xffe6c150

- `0xffe6c150`, `6.5.25/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.25:l1#1`, version `1.30.6`. Payload file offset 0xef3e8.
  Load 0xffe00000. Analysed span 0xffe6c150 to 0xffe6c1ee.
  Established. The call to 0xffe583ac has third candidate 0x180, fourth candidate zero and an
  unknown sixth candidate (`jsr` at 0xffe6c1da). The backward `blt.b` before that call has
  unresolved condition operands (`blt.b` at 0xffe6c1be).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6c16e | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffe6c170 | store | 1 | `[0x30000354] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c174 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffe6c176 | store | 1 | `[0x30000354] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6c17a | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffe6c17c | store | 1 | `[0x30000354] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6c180 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffe6c182 | store | 1 | `[0x30000354] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6c186 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffe6c18a | load | 1 | `[0x30000354] +0x1ec` | `-` |
| `movea.l` at 0xffe6c194 | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffe6c196 | store | 1 | `[0x30000354] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c19a | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffe6c19c | store | 1 | `[0x30000354] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6c1a0 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffe6c1a2 | load | 1 | `[0x30000354] +0x1f0` | `-` |
| `movea.l` at 0xffe6c1a6 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffe6c1aa | load | 1 | `[0x30000354] +0x1ec` | `-` |
| `movea.l` at 0xffe6c1ae | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffe6c1b0 | store | 1 | `[0x30000354] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6c1b4 | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffe6c1b6 | store | 1 | `[0x30000354] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6c1da | `0xffe583ac` | `0x9` | `0xffe6c97e` | `0x180` | `0x0` | `0xffe6c9a4` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c192 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6c1be | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c1ec | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.30.6-package-1/controller-write-loops-facts.txt).

## 6.5.25:l1#1 at 0xffedc150

- `0xffedc150`, `6.5.25/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.25:l1#1`, version `1.30.6`. Payload file offset 0xef3e8.
  Load 0xffe00000. Analysed span 0xffedc150 to 0xffedc1ee.
  Established. Two byte-read sites are both rendered as `[0x30000354] +0x1ec`, with a separate base
  load at each site (`move.b` at 0xffedc18a, `move.b` at 0xffedc1aa, `movea.l` at 0xffedc186,
  `movea.l` at 0xffedc1a6). The second site occurs on `beq.b` fall-through for `(? & 0x20) != 0x0`
  (`beq.b` at 0xffedc192, `move.b` at 0xffedc1aa).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedc16e | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffedc170 | store | 1 | `[0x30000354] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc174 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffedc176 | store | 1 | `[0x30000354] +0x1e4` | `0x13` |
| `movea.l` at 0xffedc17a | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffedc17c | store | 1 | `[0x30000354] +0x1e0` | `0x0` |
| `movea.l` at 0xffedc180 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffedc182 | store | 1 | `[0x30000354] +0x1e8` | `0x80` |
| `movea.l` at 0xffedc186 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffedc18a | load | 1 | `[0x30000354] +0x1ec` | `-` |
| `movea.l` at 0xffedc194 | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffedc196 | store | 1 | `[0x30000354] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc19a | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffedc19c | store | 1 | `[0x30000354] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedc1a0 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffedc1a2 | load | 1 | `[0x30000354] +0x1f0` | `-` |
| `movea.l` at 0xffedc1a6 | load | 4 | `abs 0x30000354` | `-` |
| `move.b` at 0xffedc1aa | load | 1 | `[0x30000354] +0x1ec` | `-` |
| `movea.l` at 0xffedc1ae | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffedc1b0 | store | 1 | `[0x30000354] +0x1ec` | `0x0` |
| `movea.l` at 0xffedc1b4 | load | 4 | `abs 0x30000354` | `-` |
| `clr.b` at 0xffedc1b6 | store | 1 | `[0x30000354] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedc1da | `0xffec83ac` | `0x9` | `0xffedc97e` | `0x180` | `0x0` | `0xffedc9a4` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc192 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedc1be | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc1ec | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.30.6-package-1/controller-write-loops-facts.txt).

## 6.5.25:l1#2 at 0xffea8b4c

- `0xffea8b4c`, `6.5.25/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.25:l1#2`, version `1.30.6`. Payload file offset 0x1de7f8.
  Load 0xffe00000. Analysed span 0xffea8b4c to 0xffea8bea.
  Established. A1 is assigned 0x3e8 (`movea.w` at 0xffea8b5a). The later backward `blt.b` condition
  remains unresolved, so no loop count is established here (`blt.b` at 0xffea8bba). The following
  call has target 0xffe882d8 (`jsr` at 0xffea8bd6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffea8b6a | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xffea8b6c | store | 1 | `[0x300004b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffea8b70 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xffea8b72 | store | 1 | `[0x300004b0] +0x1e4` | `0x13` |
| `movea.l` at 0xffea8b76 | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xffea8b78 | store | 1 | `[0x300004b0] +0x1e0` | `0x0` |
| `movea.l` at 0xffea8b7c | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xffea8b7e | store | 1 | `[0x300004b0] +0x1e8` | `0x80` |
| `movea.l` at 0xffea8b82 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xffea8b86 | load | 1 | `[0x300004b0] +0x1ec` | `-` |
| `movea.l` at 0xffea8b90 | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xffea8b92 | store | 1 | `[0x300004b0] +0x1e8` | `0x0` |
| `movea.l` at 0xffea8b96 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xffea8b98 | store | 1 | `[0x300004b0] +0x1e8` | `0xa0` |
| `movea.l` at 0xffea8b9c | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xffea8b9e | load | 1 | `[0x300004b0] +0x1f0` | `-` |
| `movea.l` at 0xffea8ba2 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xffea8ba6 | load | 1 | `[0x300004b0] +0x1ec` | `-` |
| `movea.l` at 0xffea8baa | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xffea8bac | store | 1 | `[0x300004b0] +0x1ec` | `0x0` |
| `movea.l` at 0xffea8bb0 | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xffea8bb2 | store | 1 | `[0x300004b0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffea8bd6 | `0xffe882d8` | `0x9` | `0xffea937a` | `0x180` | `0x0` | `0xffea93a0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffea8b8e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffea8bba | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffea8be8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.30.6-package-2/controller-write-loops-facts.txt).

## 6.5.25:l1#2 at 0xfff98b4c

- `0xfff98b4c`, `6.5.25/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.25:l1#2`, version `1.30.6`. Payload file offset 0x1de7f8.
  Load 0xffe00000. Analysed span 0xfff98b4c to 0xfff98bea.
  Established. The byte 0xa0 store rendered as `[0x300004b0] +0x1e8` is on the `beq.b` fall-through
  for `(? & 0x20) != 0x0` (`beq.b` at 0xfff98b8e, `move.b` at 0xfff98b98). The final non-stack store
  sites write byte zero through `[0x300004b0] +0x1ec` and `[0x300004b0] +0x1e8` (`clr.b` at
  0xfff98bac, `clr.b` at 0xfff98bb2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff98b6a | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xfff98b6c | store | 1 | `[0x300004b0] +0x1e8` | `0x0` |
| `movea.l` at 0xfff98b70 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xfff98b72 | store | 1 | `[0x300004b0] +0x1e4` | `0x13` |
| `movea.l` at 0xfff98b76 | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xfff98b78 | store | 1 | `[0x300004b0] +0x1e0` | `0x0` |
| `movea.l` at 0xfff98b7c | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xfff98b7e | store | 1 | `[0x300004b0] +0x1e8` | `0x80` |
| `movea.l` at 0xfff98b82 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xfff98b86 | load | 1 | `[0x300004b0] +0x1ec` | `-` |
| `movea.l` at 0xfff98b90 | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xfff98b92 | store | 1 | `[0x300004b0] +0x1e8` | `0x0` |
| `movea.l` at 0xfff98b96 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xfff98b98 | store | 1 | `[0x300004b0] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff98b9c | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xfff98b9e | load | 1 | `[0x300004b0] +0x1f0` | `-` |
| `movea.l` at 0xfff98ba2 | load | 4 | `abs 0x300004b0` | `-` |
| `move.b` at 0xfff98ba6 | load | 1 | `[0x300004b0] +0x1ec` | `-` |
| `movea.l` at 0xfff98baa | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xfff98bac | store | 1 | `[0x300004b0] +0x1ec` | `0x0` |
| `movea.l` at 0xfff98bb0 | load | 4 | `abs 0x300004b0` | `-` |
| `clr.b` at 0xfff98bb2 | store | 1 | `[0x300004b0] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff98bd6 | `0xfff782d8` | `0x9` | `0xfff9937a` | `0x180` | `0x0` | `0xfff993a0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff98b8e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff98bba | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff98be8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.30.6-package-2/controller-write-loops-facts.txt).

## 6.5.26:l1#0 at 0xffe6b8f4

- `0xffe6b8f4`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#0`, version `1.32.6`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffe6b8f4 to 0xffe6b992.
  Established. The call target 0xffe55b84 has candidates beginning 9, 0xffe6c122, 0x182, zero and
  0xffe6c148, with the sixth unknown (`jsr` at 0xffe6b97e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6b912 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffe6b914 | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b918 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffe6b91a | store | 1 | `[0x300003f8] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6b91e | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffe6b920 | store | 1 | `[0x300003f8] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6b924 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffe6b926 | store | 1 | `[0x300003f8] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6b92a | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffe6b92e | load | 1 | `[0x300003f8] +0x1ec` | `-` |
| `movea.l` at 0xffe6b938 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffe6b93a | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6b93e | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffe6b940 | store | 1 | `[0x300003f8] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6b944 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffe6b946 | load | 1 | `[0x300003f8] +0x1f0` | `-` |
| `movea.l` at 0xffe6b94a | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffe6b94e | load | 1 | `[0x300003f8] +0x1ec` | `-` |
| `movea.l` at 0xffe6b952 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffe6b954 | store | 1 | `[0x300003f8] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6b958 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffe6b95a | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6b97e | `0xffe55b84` | `0x9` | `0xffe6c122` | `0x182` | `0x0` | `0xffe6c148` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6b936 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6b962 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6b990 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-0/controller-write-loops-facts.txt).

## 6.5.26:l1#0 at 0xffedb8f4

- `0xffedb8f4`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#0`, version `1.32.6`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffedb8f4 to 0xffedb992.
  Established. The early `beq.b` selects the register-restore tail for `(? & 0x20) == 0x0`, with the
  tested value unresolved (`beq.b` at 0xffedb936). The recorded return has unknown D0 (`rts` at
  0xffedb990).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedb912 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffedb914 | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb918 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffedb91a | store | 1 | `[0x300003f8] +0x1e4` | `0x13` |
| `movea.l` at 0xffedb91e | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffedb920 | store | 1 | `[0x300003f8] +0x1e0` | `0x0` |
| `movea.l` at 0xffedb924 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffedb926 | store | 1 | `[0x300003f8] +0x1e8` | `0x80` |
| `movea.l` at 0xffedb92a | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffedb92e | load | 1 | `[0x300003f8] +0x1ec` | `-` |
| `movea.l` at 0xffedb938 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffedb93a | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |
| `movea.l` at 0xffedb93e | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffedb940 | store | 1 | `[0x300003f8] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedb944 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffedb946 | load | 1 | `[0x300003f8] +0x1f0` | `-` |
| `movea.l` at 0xffedb94a | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffedb94e | load | 1 | `[0x300003f8] +0x1ec` | `-` |
| `movea.l` at 0xffedb952 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffedb954 | store | 1 | `[0x300003f8] +0x1ec` | `0x0` |
| `movea.l` at 0xffedb958 | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffedb95a | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedb97e | `0xffec5b84` | `0x9` | `0xffedc122` | `0x182` | `0x0` | `0xffedc148` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedb936 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedb962 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedb990 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-0/controller-write-loops-facts.txt).

## 6.5.26:l1#1 at 0xffe6c494

- `0xffe6c494`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#1`, version `1.32.6`. Payload file offset 0xeef98.
  Load 0xffe00000. Analysed span 0xffe6c494 to 0xffe6c532.
  Established. The first byte-clear sites render as `[0x30000434] +0x1e8` and `[0x30000434] +0x1e0`
  (`clr.b` at 0xffe6c4b4, `clr.b` at 0xffe6c4c0). Interleaved stores carry byte 0x13 through +0x1e4
  and byte 0x80 through +0x1e8 (`move.b` at 0xffe6c4ba, `move.b` at 0xffe6c4c6). These sites precede
  `beq.b` (`clr.b` at 0xffe6c4b4, `move.b` at 0xffe6c4ba, `clr.b` at 0xffe6c4c0, `move.b` at
  0xffe6c4c6, `beq.b` at 0xffe6c4d6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6c4b2 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffe6c4b4 | store | 1 | `[0x30000434] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c4b8 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffe6c4ba | store | 1 | `[0x30000434] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6c4be | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffe6c4c0 | store | 1 | `[0x30000434] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6c4c4 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffe6c4c6 | store | 1 | `[0x30000434] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6c4ca | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffe6c4ce | load | 1 | `[0x30000434] +0x1ec` | `-` |
| `movea.l` at 0xffe6c4d8 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffe6c4da | store | 1 | `[0x30000434] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c4de | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffe6c4e0 | store | 1 | `[0x30000434] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6c4e4 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffe6c4e6 | load | 1 | `[0x30000434] +0x1f0` | `-` |
| `movea.l` at 0xffe6c4ea | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffe6c4ee | load | 1 | `[0x30000434] +0x1ec` | `-` |
| `movea.l` at 0xffe6c4f2 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffe6c4f4 | store | 1 | `[0x30000434] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6c4f8 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffe6c4fa | store | 1 | `[0x30000434] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6c51e | `0xffe5652c` | `0x9` | `0xffe6ccc2` | `0x182` | `0x0` | `0xffe6cce8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c4d6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6c502 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c530 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-1/controller-write-loops-facts.txt).

## 6.5.26:l1#1 at 0xffedc494

- `0xffedc494`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#1`, version `1.32.6`. Payload file offset 0xeef98.
  Load 0xffe00000. Analysed span 0xffedc494 to 0xffedc532.
  Established. The backward `blt.b` has unresolved condition operands and a taken target at the
  first base-load site (`blt.b` at 0xffedc502); its fall-through reaches the call setup for target
  0xffec652c, whose first candidate is 9 (`blt.b` at 0xffedc502, `jsr` at 0xffedc51e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedc4b2 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffedc4b4 | store | 1 | `[0x30000434] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc4b8 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffedc4ba | store | 1 | `[0x30000434] +0x1e4` | `0x13` |
| `movea.l` at 0xffedc4be | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffedc4c0 | store | 1 | `[0x30000434] +0x1e0` | `0x0` |
| `movea.l` at 0xffedc4c4 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffedc4c6 | store | 1 | `[0x30000434] +0x1e8` | `0x80` |
| `movea.l` at 0xffedc4ca | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffedc4ce | load | 1 | `[0x30000434] +0x1ec` | `-` |
| `movea.l` at 0xffedc4d8 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffedc4da | store | 1 | `[0x30000434] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc4de | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffedc4e0 | store | 1 | `[0x30000434] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedc4e4 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffedc4e6 | load | 1 | `[0x30000434] +0x1f0` | `-` |
| `movea.l` at 0xffedc4ea | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffedc4ee | load | 1 | `[0x30000434] +0x1ec` | `-` |
| `movea.l` at 0xffedc4f2 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffedc4f4 | store | 1 | `[0x30000434] +0x1ec` | `0x0` |
| `movea.l` at 0xffedc4f8 | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffedc4fa | store | 1 | `[0x30000434] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedc51e | `0xffec652c` | `0x9` | `0xffedccc2` | `0x182` | `0x0` | `0xffedcce8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc4d6 | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedc502 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc530 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-1/controller-write-loops-facts.txt).

## 6.5.26:l1#2 at 0xffeab7a8

- `0xffeab7a8`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#2`, version `1.32.6`. Payload file offset 0x1debf8.
  Load 0xffe00000. Analysed span 0xffeab7a8 to 0xffeab846.
  Established. On the `beq.b` fall-through for `(? & 0x20) != 0x0`, a byte read is rendered as
  `[0x30000a7c] +0x1f0` (`beq.b` at 0xffeab7ea, `move.b` at 0xffeab7fa). The byte 0xa0 store earlier
  in that fall-through region is rendered as `[0x30000a7c] +0x1e8` (`beq.b` at 0xffeab7ea, `move.b`
  at 0xffeab7f4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffeab7c6 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xffeab7c8 | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |
| `movea.l` at 0xffeab7cc | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xffeab7ce | store | 1 | `[0x30000a7c] +0x1e4` | `0x13` |
| `movea.l` at 0xffeab7d2 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xffeab7d4 | store | 1 | `[0x30000a7c] +0x1e0` | `0x0` |
| `movea.l` at 0xffeab7d8 | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xffeab7da | store | 1 | `[0x30000a7c] +0x1e8` | `0x80` |
| `movea.l` at 0xffeab7de | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xffeab7e2 | load | 1 | `[0x30000a7c] +0x1ec` | `-` |
| `movea.l` at 0xffeab7ec | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xffeab7ee | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |
| `movea.l` at 0xffeab7f2 | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xffeab7f4 | store | 1 | `[0x30000a7c] +0x1e8` | `0xa0` |
| `movea.l` at 0xffeab7f8 | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xffeab7fa | load | 1 | `[0x30000a7c] +0x1f0` | `-` |
| `movea.l` at 0xffeab7fe | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xffeab802 | load | 1 | `[0x30000a7c] +0x1ec` | `-` |
| `movea.l` at 0xffeab806 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xffeab808 | store | 1 | `[0x30000a7c] +0x1ec` | `0x0` |
| `movea.l` at 0xffeab80c | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xffeab80e | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffeab832 | `0xffe888f8` | `0x9` | `0xffeabfd6` | `0x182` | `0x0` | `0xffeabffc` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffeab7ea | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffeab816 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffeab844 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-2/controller-write-loops-facts.txt).

## 6.5.26:l1#2 at 0xfff9b7a8

- `0xfff9b7a8`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#2`, version `1.32.6`. Payload file offset 0x1debf8.
  Load 0xffe00000. Analysed span 0xfff9b7a8 to 0xfff9b846.
  Established. The direct call has target 0xfff788f8 with third candidate 0x182 and unknown sixth
  candidate (`jsr` at 0xfff9b832). The backward `blt.b` carries an unresolved condition, and the D0
  exit is also unresolved (`blt.b` at 0xfff9b816, `rts` at 0xfff9b844).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xfff9b7c6 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xfff9b7c8 | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |
| `movea.l` at 0xfff9b7cc | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xfff9b7ce | store | 1 | `[0x30000a7c] +0x1e4` | `0x13` |
| `movea.l` at 0xfff9b7d2 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xfff9b7d4 | store | 1 | `[0x30000a7c] +0x1e0` | `0x0` |
| `movea.l` at 0xfff9b7d8 | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xfff9b7da | store | 1 | `[0x30000a7c] +0x1e8` | `0x80` |
| `movea.l` at 0xfff9b7de | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xfff9b7e2 | load | 1 | `[0x30000a7c] +0x1ec` | `-` |
| `movea.l` at 0xfff9b7ec | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xfff9b7ee | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |
| `movea.l` at 0xfff9b7f2 | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xfff9b7f4 | store | 1 | `[0x30000a7c] +0x1e8` | `0xa0` |
| `movea.l` at 0xfff9b7f8 | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xfff9b7fa | load | 1 | `[0x30000a7c] +0x1f0` | `-` |
| `movea.l` at 0xfff9b7fe | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xfff9b802 | load | 1 | `[0x30000a7c] +0x1ec` | `-` |
| `movea.l` at 0xfff9b806 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xfff9b808 | store | 1 | `[0x30000a7c] +0x1ec` | `0x0` |
| `movea.l` at 0xfff9b80c | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xfff9b80e | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xfff9b832 | `0xfff788f8` | `0x9` | `0xfff9bfd6` | `0x182` | `0x0` | `0xfff9bffc` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff9b7ea | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xfff9b816 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff9b844 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-2/controller-write-loops-facts.txt).

## 6.5.27:l1#0 at 0xffe6c94c

- `0xffe6c94c`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#0`, version `1.34.2`. Payload file offset 0x50.
  Load 0xffe00000. Analysed span 0xffe6c94c to 0xffe6c9ea.
  Established. The initial store sites carry byte zero through `[0x30000410] +0x1e8`, byte 0x13
  through +0x1e4, byte zero through +0x1e0 and byte 0x80 through +0x1e8 (`clr.b` at 0xffe6c96c,
  `move.b` at 0xffe6c972, `clr.b` at 0xffe6c978, `move.b` at 0xffe6c97e). The `beq.b` restore route
  is taken for `(? & 0x20) == 0x0` (`beq.b` at 0xffe6c98e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffe6c96a | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffe6c96c | store | 1 | `[0x30000410] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c970 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffe6c972 | store | 1 | `[0x30000410] +0x1e4` | `0x13` |
| `movea.l` at 0xffe6c976 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffe6c978 | store | 1 | `[0x30000410] +0x1e0` | `0x0` |
| `movea.l` at 0xffe6c97c | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffe6c97e | store | 1 | `[0x30000410] +0x1e8` | `0x80` |
| `movea.l` at 0xffe6c982 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffe6c986 | load | 1 | `[0x30000410] +0x1ec` | `-` |
| `movea.l` at 0xffe6c990 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffe6c992 | store | 1 | `[0x30000410] +0x1e8` | `0x0` |
| `movea.l` at 0xffe6c996 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffe6c998 | store | 1 | `[0x30000410] +0x1e8` | `0xa0` |
| `movea.l` at 0xffe6c99c | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffe6c99e | load | 1 | `[0x30000410] +0x1f0` | `-` |
| `movea.l` at 0xffe6c9a2 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffe6c9a6 | load | 1 | `[0x30000410] +0x1ec` | `-` |
| `movea.l` at 0xffe6c9aa | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffe6c9ac | store | 1 | `[0x30000410] +0x1ec` | `0x0` |
| `movea.l` at 0xffe6c9b0 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffe6c9b2 | store | 1 | `[0x30000410] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffe6c9d6 | `0xffe567e4` | `0x9` | `0xffe6d17a` | `0x182` | `0x0` | `0xffe6d1a0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c98e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffe6c9ba | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c9e8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-0/controller-write-loops-facts.txt).

## 6.5.27:l1#0 at 0xffedc94c

- `0xffedc94c`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#0`, version `1.34.2`. Payload file offset 0x50.
  Load 0xffe00000. Analysed span 0xffedc94c to 0xffedc9ea.
  Established. The last non-stack write sites clear bytes through `[0x30000410] +0x1ec` and
  `[0x30000410] +0x1e8` (`clr.b` at 0xffedc9ac, `clr.b` at 0xffedc9b2). The taken `beq.b` path
  bypasses these sites for `(? & 0x20) == 0x0` (`beq.b` at 0xffedc98e). The return value in D0
  remains unknown (`rts` at 0xffedc9e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movea.l` at 0xffedc96a | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffedc96c | store | 1 | `[0x30000410] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc970 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffedc972 | store | 1 | `[0x30000410] +0x1e4` | `0x13` |
| `movea.l` at 0xffedc976 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffedc978 | store | 1 | `[0x30000410] +0x1e0` | `0x0` |
| `movea.l` at 0xffedc97c | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffedc97e | store | 1 | `[0x30000410] +0x1e8` | `0x80` |
| `movea.l` at 0xffedc982 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffedc986 | load | 1 | `[0x30000410] +0x1ec` | `-` |
| `movea.l` at 0xffedc990 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffedc992 | store | 1 | `[0x30000410] +0x1e8` | `0x0` |
| `movea.l` at 0xffedc996 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffedc998 | store | 1 | `[0x30000410] +0x1e8` | `0xa0` |
| `movea.l` at 0xffedc99c | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffedc99e | load | 1 | `[0x30000410] +0x1f0` | `-` |
| `movea.l` at 0xffedc9a2 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffedc9a6 | load | 1 | `[0x30000410] +0x1ec` | `-` |
| `movea.l` at 0xffedc9aa | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffedc9ac | store | 1 | `[0x30000410] +0x1ec` | `0x0` |
| `movea.l` at 0xffedc9b0 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffedc9b2 | store | 1 | `[0x30000410] +0x1e8` | `0x0` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `jsr` at 0xffedc9d6 | `0xffec67e4` | `0x9` | `0xffedd17a` | `0x182` | `0x0` | `0xffedd1a0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc98e | `(? & 0x20) == 0x0` | `(? & 0x20) != 0x0` |
| `blt.b` at 0xffedc9ba | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc9e8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-0/controller-write-loops-facts.txt).
