# L1 pointer-relative setup and window-call paths

These ColdFire bodies perform setup stores and pass address constants to
helpers. The reference-package context is described in
[USB controller accesses](usb-controller-registers.md). The tables below
record each selected package and ROM location independently.

A bracketed address denotes a loaded value, not a known absolute base.
`sp` is the stack pointer at routine entry. Call columns are candidate
locations under the assumed M68K stack convention, with unknown arity.
Unknown selector operands, stored values and callee purposes stay unknown.
Bounds end at the next LINK prologue and are inferred.

## 6.5.10:l1#0 at 0xffe49b3c

- `0xffe49b3c`, `6.5.10/usr/cpu/firmware/l1.bin`:
  Package `6.5.10:l1#0`, version `1.1.6`. Payload file offset 0x28.
  Load 0xffe00000. Analysed span 0xffe49b3c to 0xffe49bd4.
  Established. The selected write path reaches the helper at 0xffe5b128 with candidate stack values
  0, 0x40000000 and 2 (`jsr` at 0xffe49bc0). The branch operands remain unknown at the flag branches
  (`bgt.b` at 0xffe49b66, `beq.b` at 0xffe49b68).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe49b98 | 2 | `[0x30000490] +0x88` | `0x4000` |
| `clr.l` at 0xffe49ba2 | 4 | `[0x30000490] +0x8c` | `0x0` |
| `move.w` at 0xffe49bb0 | 2 | `[0x30000490] +0x92` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe49b42 | `0xffe4e1c4` | `d7` | `a6` | `?` | `?` |
| `bsr.w` at 0xffe49b52 | `0xffe49bd4` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffe49bc0 | `0xffe5b128` | `0x0` | `0x40000000` | `0x2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt.b` at 0xffe49b66 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq.b` at 0xffe49b68 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe49b70 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe49b74 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe49b78 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe49b7e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe49b86 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe49b8a | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.1.6-package-0/window-callers-facts.txt).

## 6.5.10:l1#0 at 0xffeb9b3c

- `0xffeb9b3c`, `6.5.10/usr/cpu/firmware/l1.bin`:
  Package `6.5.10:l1#0`, version `1.1.6`. Payload file offset 0x28.
  Load 0xffe00000. Analysed span 0xffeb9b3c to 0xffeb9bd4.
  Established. A four-byte clear through `[0x30000490] +0x8c` lies between two halfword writes of
  0x4000 and 0x953 (`move.w` at 0xffeb9b98, `clr.l` at 0xffeb9ba2, `move.w` at 0xffeb9bb0). The
  later call targets 0xffecb128 (`jsr` at 0xffeb9bc0).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffeb9b98 | 2 | `[0x30000490] +0x88` | `0x4000` |
| `clr.l` at 0xffeb9ba2 | 4 | `[0x30000490] +0x8c` | `0x0` |
| `move.w` at 0xffeb9bb0 | 2 | `[0x30000490] +0x92` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffeb9b42 | `0xffebe1c4` | `d7` | `a6` | `?` | `?` |
| `bsr.w` at 0xffeb9b52 | `0xffeb9bd4` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffeb9bc0 | `0xffecb128` | `0x0` | `0x40000000` | `0x2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt.b` at 0xffeb9b66 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq.b` at 0xffeb9b68 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffeb9b70 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffeb9b74 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffeb9b78 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffeb9b7e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffeb9b86 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffeb9b8a | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.1.6-package-0/window-callers-facts.txt).

## 6.5.12:l1#0 at 0xffe54dcc

- `0xffe54dcc`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.12:l1#0`, version `1.4.1`. Payload file offset 0x28.
  Load 0xffe00000. Analysed span 0xffe54dcc to 0xffe54e78.
  Established. Before the conditional setup, three byte stores create values 0, 5 and 0 at entry
  stack -0x8, -0x7 and -0x6 (`clr.b` at 0xffe54de8, `move.b` at 0xffe54dee, `clr.b` at 0xffe54df2).
  The address-bearing call includes `sp-8` in its fourth candidate location (`jsr` at 0xffe54e66).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe54e3a | 2 | `[0x30000198] +0x88` | `0x4000` |
| `clr.l` at 0xffe54e44 | 4 | `[0x30000198] +0x8c` | `0x0` |
| `move.w` at 0xffe54e52 | 2 | `[0x30000198] +0x92` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe54dd2 | `0xffe55878` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe54de2 | `0xffe54e78` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffe54e66 | `0xffe65a00` | `0x0` | `0x40000000` | `0x2` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt.b` at 0xffe54e04 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq.b` at 0xffe54e06 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe54e0e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe54e12 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe54e16 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe54e1a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe54e20 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe54e28 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe54e2c | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.4.1-package-0/window-callers-facts.txt).

## 6.5.12:l1#0 at 0xffec4dcc

- `0xffec4dcc`, `6.5.12/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.12:l1#0`, version `1.4.1`. Payload file offset 0x28.
  Load 0xffe00000. Analysed span 0xffec4dcc to 0xffec4e78.
  Established. The call at 0xffec4e66 carries `0x0`, `0x40000000`, `0x2` and `sp-8` as its first
  four candidate stack values (`jsr` at 0xffec4e66); its two earlier callees are separate from that
  address-bearing call (`jsr` at 0xffec4dd2, `bsr.w` at 0xffec4de2).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffec4e3a | 2 | `[0x30000198] +0x88` | `0x4000` |
| `clr.l` at 0xffec4e44 | 4 | `[0x30000198] +0x8c` | `0x0` |
| `move.w` at 0xffec4e52 | 2 | `[0x30000198] +0x92` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffec4dd2 | `0xffec5878` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffec4de2 | `0xffec4e78` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffec4e66 | `0xffed5a00` | `0x0` | `0x40000000` | `0x2` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt.b` at 0xffec4e04 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq.b` at 0xffec4e06 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffec4e0e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffec4e12 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffec4e16 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffec4e1a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffec4e20 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffec4e28 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffec4e2c | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.4.1-package-0/window-callers-facts.txt).

## 6.5.13:l1#0 at 0xffe5b600

- `0xffe5b600`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.13:l1#0`, version `1.6.0`. Payload file offset 0x28.
  Load 0xffe00000. Analysed span 0xffe5b600 to 0xffe5b6b4.
  Established. The halfword stores through `[0x300002ac] +0x88` and +0x92 carry 0x4000 and 0x953,
  while the intervening four-byte store is zero (`move.w` at 0xffe5b674, `clr.l` at 0xffe5b67e,
  `move.w` at 0xffe5b68c). The branch conditions leading into this block retain unknown selector
  operands (`bgt.b` at 0xffe5b638, `beq.b` at 0xffe5b63a).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe5b674 | 2 | `[0x300002ac] +0x88` | `0x4000` |
| `clr.l` at 0xffe5b67e | 4 | `[0x300002ac] +0x8c` | `0x0` |
| `move.w` at 0xffe5b68c | 2 | `[0x300002ac] +0x92` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe5b606 | `0xffe572b8` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe5b616 | `0xffe5b6b4` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffe5b6a0 | `0xffe67c84` | `0x0` | `0x40000000` | `0x2` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt.b` at 0xffe5b638 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq.b` at 0xffe5b63a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b63e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b642 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b646 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b64a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b656 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b65a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b662 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffe5b666 | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.6.0-package-0/window-callers-facts.txt).

## 6.5.13:l1#0 at 0xffecb600

- `0xffecb600`, `6.5.13/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.13:l1#0`, version `1.6.0`. Payload file offset 0x28.
  Load 0xffe00000. Analysed span 0xffecb600 to 0xffecb6b4.
  Established. Separate loads from absolute address 0x300002ac supply bases for the setup stores
  (`movea.l` at 0xffecb66a, `movea.l` at 0xffecb678, `movea.l` at 0xffecb682). Their helper call has
  `0x40000000` in the second candidate location and a local-frame pointer in the fourth (`jsr` at
  0xffecb6a0).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffecb674 | 2 | `[0x300002ac] +0x88` | `0x4000` |
| `clr.l` at 0xffecb67e | 4 | `[0x300002ac] +0x8c` | `0x0` |
| `move.w` at 0xffecb68c | 2 | `[0x300002ac] +0x92` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffecb606 | `0xffec72b8` | `d7` | `?` | `?` | `a6` |
| `bsr.w` at 0xffecb616 | `0xffecb6b4` | `?` | `?` | `?` | `?` |
| `jsr` at 0xffecb6a0 | `0xffed7c84` | `0x0` | `0x40000000` | `0x2` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt.b` at 0xffecb638 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq.b` at 0xffecb63a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb63e | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb642 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb646 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb64a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb656 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb65a | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb662 | `Z == 1` | `not (Z == 1)` |
| `beq.b` at 0xffecb666 | `Z == 1` | `not (Z == 1)` |

[Full fact sheet and listing](disasm/mcf5206e-1.6.0-package-0/window-callers-facts.txt).

## 6.5.30:l1#0 at 0xffe55548

- `0xffe55548`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#0`, version `1.44.0`. Payload file offset 0x68.
  Load 0xffe00000. Analysed span 0xffe55548 to 0xffe55604.
  Established. A signed test of the four-byte value at 0x30001e40 skips the first write-and-call
  block when it is at most zero (`tst.l` at 0xffe55570, `ble.b` at 0xffe55574). A positive value
  falls through (`ble.b` at 0xffe55574). The two address-bearing calls both target 0xffe6dcf8 (`jsr`
  at 0xffe555a6, `jsr` at 0xffe555f2).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe55582 | 2 | `[0x300002bc] +0x88` | `0x4000` |
| `clr.l` at 0xffe55588 | 4 | `[0x300002bc] +0x8c` | `0x0` |
| `move.w` at 0xffe55592 | 2 | `[0x300002bc] +0x92` | `0x953` |
| `move.b` at 0xffe555c2 | 1 | `[0x300002bc] +0x16` | `0x8c` |
| `move.w` at 0xffe555cc | 2 | `[0x300002bc] +0xa0` | `0x6000` |
| `clr.l` at 0xffe555d2 | 4 | `[0x300002bc] +0xa4` | `0x0` |
| `move.w` at 0xffe555dc | 2 | `[0x300002bc] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe5554e | `0xffe55b64` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe55556 | `0xffe54f90` | `ret(0xffe55b64)` | `?` | `?` | `?` |
| `jsr` at 0xffe555a6 | `0xffe6dcf8` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe555f2 | `0xffe6dcf8` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe55574 | `[0x30001e40] <= 0x0 (signed, 32-bit)` | `[0x30001e40] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe555b6 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-0/window-callers-facts.txt).

## 6.5.30:l1#0 at 0xffec5548

- `0xffec5548`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#0`, version `1.44.0`. Payload file offset 0x68.
  Load 0xffe00000. Analysed span 0xffec5548 to 0xffec5604.
  Established. The later path writes byte 0x8c through `[0x300002bc] +0x16` before calling the
  helper with second candidate value 0x60000000 (`move.b` at 0xffec55c2, `jsr` at 0xffec55f2); its
  `bne.b` skip condition is recorded only as `Z == 0` (`bne.b` at 0xffec55b6).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffec5582 | 2 | `[0x300002bc] +0x88` | `0x4000` |
| `clr.l` at 0xffec5588 | 4 | `[0x300002bc] +0x8c` | `0x0` |
| `move.w` at 0xffec5592 | 2 | `[0x300002bc] +0x92` | `0x953` |
| `move.b` at 0xffec55c2 | 1 | `[0x300002bc] +0x16` | `0x8c` |
| `move.w` at 0xffec55cc | 2 | `[0x300002bc] +0xa0` | `0x6000` |
| `clr.l` at 0xffec55d2 | 4 | `[0x300002bc] +0xa4` | `0x0` |
| `move.w` at 0xffec55dc | 2 | `[0x300002bc] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffec554e | `0xffec5b64` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffec5556 | `0xffec4f90` | `ret(0xffec5b64)` | `?` | `?` | `?` |
| `jsr` at 0xffec55a6 | `0xffeddcf8` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffec55f2 | `0xffeddcf8` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffec5574 | `[0x30001e40] <= 0x0 (signed, 32-bit)` | `[0x30001e40] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffec55b6 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-0/window-callers-facts.txt).

## 6.5.30:l1#1 at 0xffec5198

- `0xffec5198`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#1`, version `1.44.0`. Payload file offset 0xefdfc.
  Load 0xffe00000. Analysed span 0xffec5198 to 0xffec5254.
  Established. In this ROM span, two distinct halfword setup values are 0x4000 and 0x6000, written
  through `[0x300002e8] +0x88` and +0xa0 (`move.w` at 0xffec51d2, `move.w` at 0xffec521c). The first
  address-bearing call supplies 0, while the later one supplies 1 in the first candidate location
  (`jsr` at 0xffec51f6, `jsr` at 0xffec5242).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffec51d2 | 2 | `[0x300002e8] +0x88` | `0x4000` |
| `clr.l` at 0xffec51d8 | 4 | `[0x300002e8] +0x8c` | `0x0` |
| `move.w` at 0xffec51e2 | 2 | `[0x300002e8] +0x92` | `0x953` |
| `move.b` at 0xffec5212 | 1 | `[0x300002e8] +0x16` | `0x8c` |
| `move.w` at 0xffec521c | 2 | `[0x300002e8] +0xa0` | `0x6000` |
| `clr.l` at 0xffec5222 | 4 | `[0x300002e8] +0xa4` | `0x0` |
| `move.w` at 0xffec522c | 2 | `[0x300002e8] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffec519e | `0xffec57b4` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffec51a6 | `0xffec4be0` | `ret(0xffec57b4)` | `?` | `?` | `?` |
| `jsr` at 0xffec51f6 | `0xffeddba8` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffec5242 | `0xffeddba8` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffec51c4 | `[0x30001f30] <= 0x0 (signed, 32-bit)` | `[0x30001f30] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffec5206 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-fuel-pe-o300/window-callers-facts.txt).

## 6.5.30:l1#2 at 0xffe759ac

- `0xffe759ac`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#2`, version `1.44.0`. Payload file offset 0x1dfb88.
  Load 0xffe00000. Analysed span 0xffe759ac to 0xffe75a68.
  Established. An initial call targets 0xffe75fc8, and the next call has `ret(0xffe75fc8)` in its
  first candidate location (`jsr` at 0xffe759b2, `bsr.w` at 0xffe759ba). The later conditional
  blocks use the same helper target 0xffe90d54 with different address constants (`jsr` at
  0xffe75a0a, `jsr` at 0xffe75a56).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe759e6 | 2 | `[0x300008bc] +0x88` | `0x4000` |
| `clr.l` at 0xffe759ec | 4 | `[0x300008bc] +0x8c` | `0x0` |
| `move.w` at 0xffe759f6 | 2 | `[0x300008bc] +0x92` | `0x953` |
| `move.b` at 0xffe75a26 | 1 | `[0x300008bc] +0x16` | `0x8c` |
| `move.w` at 0xffe75a30 | 2 | `[0x300008bc] +0xa0` | `0x6000` |
| `clr.l` at 0xffe75a36 | 4 | `[0x300008bc] +0xa4` | `0x0` |
| `move.w` at 0xffe75a40 | 2 | `[0x300008bc] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe759b2 | `0xffe75fc8` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe759ba | `0xffe753f4` | `ret(0xffe75fc8)` | `?` | `?` | `?` |
| `jsr` at 0xffe75a0a | `0xffe90d54` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe75a56 | `0xffe90d54` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe759d8 | `[0x3000282c] <= 0x0 (signed, 32-bit)` | `[0x3000282c] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe75a1a | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-2/window-callers-facts.txt).

## 6.5.30:l1#2 at 0xfff659ac

- `0xfff659ac`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#2`, version `1.44.0`. Payload file offset 0x1dfb88.
  Load 0xffe00000. Analysed span 0xfff659ac to 0xfff65a68.
  Established. This body loads A5 from entry stack -0x10 before unlinking its frame and returning
  (`movea.l` at 0xfff65a60, `unlk` at 0xfff65a64, `rts` at 0xfff65a66); its last address-bearing
  call has candidates `0x1`, `0x60000000`, `0x3` and `sp-8` (`jsr` at 0xfff65a56).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xfff659e6 | 2 | `[0x300008bc] +0x88` | `0x4000` |
| `clr.l` at 0xfff659ec | 4 | `[0x300008bc] +0x8c` | `0x0` |
| `move.w` at 0xfff659f6 | 2 | `[0x300008bc] +0x92` | `0x953` |
| `move.b` at 0xfff65a26 | 1 | `[0x300008bc] +0x16` | `0x8c` |
| `move.w` at 0xfff65a30 | 2 | `[0x300008bc] +0xa0` | `0x6000` |
| `clr.l` at 0xfff65a36 | 4 | `[0x300008bc] +0xa4` | `0x0` |
| `move.w` at 0xfff65a40 | 2 | `[0x300008bc] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xfff659b2 | `0xfff65fc8` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xfff659ba | `0xfff653f4` | `ret(0xfff65fc8)` | `?` | `?` | `?` |
| `jsr` at 0xfff65a0a | `0xfff80d54` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xfff65a56 | `0xfff80d54` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xfff659d8 | `[0x3000282c] <= 0x0 (signed, 32-bit)` | `[0x3000282c] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xfff65a1a | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-2/window-callers-facts.txt).

## 6.5.30:l1#3 at 0xffe6b048

- `0xffe6b048`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#3`, version `1.44.0`. Payload file offset 0x38c534.
  Load 0xffe00000. Analysed span 0xffe6b048 to 0xffe6b104.
  Established. Zero is stored as four bytes through both `[0x30000a50] +0x8c` and +0xa4 on their
  respective paths (`clr.l` at 0xffe6b088, `clr.l` at 0xffe6b0d2). The first block is bypassed by
  `ble.b` when `[0x30002ba0] <= 0x0` as a signed 32-bit comparison (`ble.b` at 0xffe6b074).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe6b082 | 2 | `[0x30000a50] +0x88` | `0x4000` |
| `clr.l` at 0xffe6b088 | 4 | `[0x30000a50] +0x8c` | `0x0` |
| `move.w` at 0xffe6b092 | 2 | `[0x30000a50] +0x92` | `0x953` |
| `move.b` at 0xffe6b0c2 | 1 | `[0x30000a50] +0x16` | `0x8c` |
| `move.w` at 0xffe6b0cc | 2 | `[0x30000a50] +0xa0` | `0x6000` |
| `clr.l` at 0xffe6b0d2 | 4 | `[0x30000a50] +0xa4` | `0x0` |
| `move.w` at 0xffe6b0dc | 2 | `[0x30000a50] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe6b04e | `0xffe6b378` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe6b056 | `0xffe6ac4c` | `ret(0xffe6b378)` | `?` | `?` | `?` |
| `jsr` at 0xffe6b0a6 | `0xffe94fa4` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe6b0f2 | `0xffe94fa4` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe6b074 | `[0x30002ba0] <= 0x0 (signed, 32-bit)` | `[0x30002ba0] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe6b0b6 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-3/window-callers-facts.txt).

## 6.5.30:l1#3 at 0xfff5b048

- `0xfff5b048`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#3`, version `1.44.0`. Payload file offset 0x38c534.
  Load 0xffe00000. Analysed span 0xfff5b048 to 0xfff5b104.
  Established. Two halfword stores carry 0x953, one through `[0x30000a50] +0x92` and the other
  through +0xaa (`move.w` at 0xfff5b092, `move.w` at 0xfff5b0dc). Calls using 0x40000000 and
  0x60000000 target 0xfff84fa4 (`jsr` at 0xfff5b0a6, `jsr` at 0xfff5b0f2).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xfff5b082 | 2 | `[0x30000a50] +0x88` | `0x4000` |
| `clr.l` at 0xfff5b088 | 4 | `[0x30000a50] +0x8c` | `0x0` |
| `move.w` at 0xfff5b092 | 2 | `[0x30000a50] +0x92` | `0x953` |
| `move.b` at 0xfff5b0c2 | 1 | `[0x30000a50] +0x16` | `0x8c` |
| `move.w` at 0xfff5b0cc | 2 | `[0x30000a50] +0xa0` | `0x6000` |
| `clr.l` at 0xfff5b0d2 | 4 | `[0x30000a50] +0xa4` | `0x0` |
| `move.w` at 0xfff5b0dc | 2 | `[0x30000a50] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xfff5b04e | `0xfff5b378` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xfff5b056 | `0xfff5ac4c` | `ret(0xfff5b378)` | `?` | `?` | `?` |
| `jsr` at 0xfff5b0a6 | `0xfff84fa4` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xfff5b0f2 | `0xfff84fa4` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xfff5b074 | `[0x30002ba0] <= 0x0 (signed, 32-bit)` | `[0x30002ba0] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xfff5b0b6 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-3/window-callers-facts.txt).

## 6.5.30:l1#5 at 0xffe79da4

- `0xffe79da4`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#5`, version `1.44.0`. Payload file offset 0x8e127c.
  Load 0xffe00000. Analysed span 0xffe79da4 to 0xffe79e60.
  Established. Both address-bearing calls use 0xffe95744, with different first three candidate
  values and the same fourth candidate `sp-8` (`jsr` at 0xffe79e02, `jsr` at 0xffe79e4e). The tested
  four-byte value is at 0x30003418 (`tst.l` at 0xffe79dcc, `ble.b` at 0xffe79dd0).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xffe79dde | 2 | `[0x30000bd4] +0x88` | `0x4000` |
| `clr.l` at 0xffe79de4 | 4 | `[0x30000bd4] +0x8c` | `0x0` |
| `move.w` at 0xffe79dee | 2 | `[0x30000bd4] +0x92` | `0x953` |
| `move.b` at 0xffe79e1e | 1 | `[0x30000bd4] +0x16` | `0x8c` |
| `move.w` at 0xffe79e28 | 2 | `[0x30000bd4] +0xa0` | `0x6000` |
| `clr.l` at 0xffe79e2e | 4 | `[0x30000bd4] +0xa4` | `0x0` |
| `move.w` at 0xffe79e38 | 2 | `[0x30000bd4] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xffe79daa | `0xffe79e60` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xffe79db2 | `0xffe799a8` | `ret(0xffe79e60)` | `?` | `?` | `?` |
| `jsr` at 0xffe79e02 | `0xffe95744` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xffe79e4e | `0xffe95744` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xffe79dd0 | `[0x30003418] <= 0x0 (signed, 32-bit)` | `[0x30003418] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xffe79e12 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-5/window-callers-facts.txt).

## 6.5.30:l1#5 at 0xfff69da4

- `0xfff69da4`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#5`, version `1.44.0`. Payload file offset 0x8e127c.
  Load 0xffe00000. Analysed span 0xfff69da4 to 0xfff69e60.
  Established. The `bne.b` before the later block takes its skip when `Z == 0` and falls through
  when that flag condition is false (`bne.b` at 0xfff69e12). That later block includes a halfword
  0x6000 store and a four-byte zero store through `[0x30000bd4] +0xa0` and +0xa4 (`move.w` at
  0xfff69e28, `clr.l` at 0xfff69e2e).

Listed pointer-relative stores:

| Instruction | Width in bytes | Effective target | Stored value |
|---|---:|---|---|
| `move.w` at 0xfff69dde | 2 | `[0x30000bd4] +0x88` | `0x4000` |
| `clr.l` at 0xfff69de4 | 4 | `[0x30000bd4] +0x8c` | `0x0` |
| `move.w` at 0xfff69dee | 2 | `[0x30000bd4] +0x92` | `0x953` |
| `move.b` at 0xfff69e1e | 1 | `[0x30000bd4] +0x16` | `0x8c` |
| `move.w` at 0xfff69e28 | 2 | `[0x30000bd4] +0xa0` | `0x6000` |
| `clr.l` at 0xfff69e2e | 4 | `[0x30000bd4] +0xa4` | `0x0` |
| `move.w` at 0xfff69e38 | 2 | `[0x30000bd4] +0xaa` | `0x953` |

| Call instruction | Target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `jsr` at 0xfff69daa | `0xfff69e60` | `a5` | `?` | `?` | `a6` |
| `bsr.w` at 0xfff69db2 | `0xfff699a8` | `ret(0xfff69e60)` | `?` | `?` | `?` |
| `jsr` at 0xfff69e02 | `0xfff85744` | `0x0` | `0x40000000` | `0x2` | `sp-8` |
| `jsr` at 0xfff69e4e | `0xfff85744` | `0x1` | `0x60000000` | `0x3` | `sp-8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble.b` at 0xfff69dd0 | `[0x30003418] <= 0x0 (signed, 32-bit)` | `[0x30003418] > 0x0 (signed, 32-bit)` |
| `bne.b` at 0xfff69e12 | `Z == 0` | `not (Z == 0)` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-5/window-callers-facts.txt).
