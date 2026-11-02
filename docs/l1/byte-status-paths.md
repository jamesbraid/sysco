# L1 conditional byte-copy and clear paths

These ColdFire bodies were read separately at each package and ROM location listed below.
They contain no calls. Their controller roles and the physical devices behind the loaded bases
remain unknown here. The [M-Bus overview](mbus.md) supplies context for the later reference image;
its labels and device meanings are not assumed for these locations.

The analysed spans include the terminating `rts`. `sp` denotes the entry stack pointer,
and a bracketed address denotes a loaded value. Every recorded D0 exit is unresolved.
The tables list the non-stack access sites collected for each routine, including a read
and a write on separate rows when one instruction performs both. A dash in Stored value marks a load.

## 6.5.26:l1#0 at 0xffe6ba80

- `0xffe6ba80`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#0`, version `1.32.6`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffe6ba80 to 0xffe6baba.
  Established. Zero at the four-byte location 0x30000648 takes `beq.b` directly to the restore path
  (`tst.l` at 0xffe6ba8c, `beq.b` at 0xffe6ba90). Nonzero falls through to the byte copy from
  `[0x300003f8] +0x1ec` to 0x30000634 (`beq.b` at 0xffe6ba90, `move.b` at 0xffe6ba9a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6ba8c | load | 4 | `abs 0x30000648` | `-` |
| `movea.l` at 0xffe6ba98 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffe6ba9a | load | 1 | `[0x300003f8] +0x1ec` | `-` |
| `move.b` at 0xffe6ba9a | store | 1 | `abs 0x30000634` | `([[0x300003f8]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6ba9e | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffe6baa0 | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6baa4 | store | 4 | `abs 0x30000648` | `0x0` |
| `move.l` at 0xffe6baae | store | 4 | `abs 0x3000064c` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6ba90 | `[0x30000648] == 0x0` | `[0x30000648] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6bab8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-0/byte-status-facts.txt).

## 6.5.26:l1#0 at 0xffedba80

- `0xffedba80`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#0`, version `1.32.6`. Payload file offset 0x48.
  Load 0xffe00000. Analysed span 0xffedba80 to 0xffedbaba.
  Established. The byte copied to 0x30000634 comes from a base loaded at 0x300003f8, with access
  offset +0x1ec (`movea.l` at 0xffedba98, `move.b` at 0xffedba9a). Before the copy, `beq.b` skips
  the whole write region when `[0x30000648] == 0` (`beq.b` at 0xffedba90).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedba8c | load | 4 | `abs 0x30000648` | `-` |
| `movea.l` at 0xffedba98 | load | 4 | `abs 0x300003f8` | `-` |
| `move.b` at 0xffedba9a | load | 1 | `[0x300003f8] +0x1ec` | `-` |
| `move.b` at 0xffedba9a | store | 1 | `abs 0x30000634` | `([[0x300003f8]+492]:1 & 0xff)` |
| `movea.l` at 0xffedba9e | load | 4 | `abs 0x300003f8` | `-` |
| `clr.b` at 0xffedbaa0 | store | 1 | `[0x300003f8] +0x1e8` | `0x0` |
| `clr.l` at 0xffedbaa4 | store | 4 | `abs 0x30000648` | `0x0` |
| `move.l` at 0xffedbaae | store | 4 | `abs 0x3000064c` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedba90 | `[0x30000648] == 0x0` | `[0x30000648] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedbab8 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-0/byte-status-facts.txt).

## 6.5.26:l1#1 at 0xffe6c620

- `0xffe6c620`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#1`, version `1.32.6`. Payload file offset 0xeef98.
  Load 0xffe00000. Analysed span 0xffe6c620 to 0xffe6c65a.
  Established. When `[0x30000684]` is nonzero, execution falls through `beq.b` to the byte read through
  `[0x30000434] +0x1ec` and store at 0x30000670 (`beq.b` at 0xffe6c630, `move.b` at 0xffe6c63a). The following byte clear
  targets `[0x30000434] +0x1e8` (`clr.b` at 0xffe6c640).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6c62c | load | 4 | `abs 0x30000684` | `-` |
| `movea.l` at 0xffe6c638 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffe6c63a | load | 1 | `[0x30000434] +0x1ec` | `-` |
| `move.b` at 0xffe6c63a | store | 1 | `abs 0x30000670` | `([[0x30000434]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6c63e | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffe6c640 | store | 1 | `[0x30000434] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6c644 | store | 4 | `abs 0x30000684` | `0x0` |
| `move.l` at 0xffe6c64e | store | 4 | `abs 0x30000688` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c630 | `[0x30000684] == 0x0` | `[0x30000684] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c658 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-1/byte-status-facts.txt).

## 6.5.26:l1#1 at 0xffedc620

- `0xffedc620`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#1`, version `1.32.6`. Payload file offset 0xeef98.
  Load 0xffe00000. Analysed span 0xffedc620 to 0xffedc65a.
  Established. A four-byte zero store at 0x30000684 precedes the four-byte 0xfffffe70 store at
  0x30000688 (`clr.l` at 0xffedc644, `move.l` at 0xffedc64e). Both belong to the path bypassed when
  `beq.b` finds zero at 0x30000684 (`beq.b` at 0xffedc630).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedc62c | load | 4 | `abs 0x30000684` | `-` |
| `movea.l` at 0xffedc638 | load | 4 | `abs 0x30000434` | `-` |
| `move.b` at 0xffedc63a | load | 1 | `[0x30000434] +0x1ec` | `-` |
| `move.b` at 0xffedc63a | store | 1 | `abs 0x30000670` | `([[0x30000434]+492]:1 & 0xff)` |
| `movea.l` at 0xffedc63e | load | 4 | `abs 0x30000434` | `-` |
| `clr.b` at 0xffedc640 | store | 1 | `[0x30000434] +0x1e8` | `0x0` |
| `clr.l` at 0xffedc644 | store | 4 | `abs 0x30000684` | `0x0` |
| `move.l` at 0xffedc64e | store | 4 | `abs 0x30000688` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc630 | `[0x30000684] == 0x0` | `[0x30000684] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc658 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-1/byte-status-facts.txt).

## 6.5.26:l1#2 at 0xffeab934

- `0xffeab934`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#2`, version `1.32.6`. Payload file offset 0x1debf8.
  Load 0xffe00000. Analysed span 0xffeab934 to 0xffeab96e.
  Established. The first four-byte test reads 0x30000ce4 (`tst.l` at 0xffeab940); its `beq.b` takes
  the restore path for zero, while nonzero falls through to writes including byte zero through
  `[0x30000a7c] +0x1e8` (`beq.b` at 0xffeab944, `clr.b` at 0xffeab954).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffeab940 | load | 4 | `abs 0x30000ce4` | `-` |
| `movea.l` at 0xffeab94c | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xffeab94e | load | 1 | `[0x30000a7c] +0x1ec` | `-` |
| `move.b` at 0xffeab94e | store | 1 | `abs 0x30000cd0` | `([[0x30000a7c]+492]:1 & 0xff)` |
| `movea.l` at 0xffeab952 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xffeab954 | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |
| `clr.l` at 0xffeab958 | store | 4 | `abs 0x30000ce4` | `0x0` |
| `move.l` at 0xffeab962 | store | 4 | `abs 0x30000ce8` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffeab944 | `[0x30000ce4] == 0x0` | `[0x30000ce4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffeab96c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-2/byte-status-facts.txt).

## 6.5.26:l1#2 at 0xfff9b934

- `0xfff9b934`, `6.5.26/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.26:l1#2`, version `1.32.6`. Payload file offset 0x1debf8.
  Load 0xffe00000. Analysed span 0xfff9b934 to 0xfff9b96e.
  Established. This entry stores the byte from `[0x30000a7c] +0x1ec` at 0x30000cd0 (`move.b` at
  0xfff9b94e). Later writes clear four bytes at 0x30000ce4 and put 0xfffffe70 at 0x30000ce8 (`clr.l`
  at 0xfff9b958, `move.l` at 0xfff9b962). The guard skips these accesses when `[0x30000ce4] == 0`
  (`beq.b` at 0xfff9b944).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xfff9b940 | load | 4 | `abs 0x30000ce4` | `-` |
| `movea.l` at 0xfff9b94c | load | 4 | `abs 0x30000a7c` | `-` |
| `move.b` at 0xfff9b94e | load | 1 | `[0x30000a7c] +0x1ec` | `-` |
| `move.b` at 0xfff9b94e | store | 1 | `abs 0x30000cd0` | `([[0x30000a7c]+492]:1 & 0xff)` |
| `movea.l` at 0xfff9b952 | load | 4 | `abs 0x30000a7c` | `-` |
| `clr.b` at 0xfff9b954 | store | 1 | `[0x30000a7c] +0x1e8` | `0x0` |
| `clr.l` at 0xfff9b958 | store | 4 | `abs 0x30000ce4` | `0x0` |
| `move.l` at 0xfff9b962 | store | 4 | `abs 0x30000ce8` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff9b944 | `[0x30000ce4] == 0x0` | `[0x30000ce4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff9b96c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.32.6-package-2/byte-status-facts.txt).

## 6.5.27:l1#0 at 0xffe6cad8

- `0xffe6cad8`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#0`, version `1.34.2`. Payload file offset 0x50.
  Load 0xffe00000. Analysed span 0xffe6cad8 to 0xffe6cb12.
  Established. Two four-byte loads at 0x30000410 supply bases for the byte copy and the byte clear
  respectively (`movea.l` at 0xffe6caf0, `movea.l` at 0xffe6caf6). The byte read and zero store use
  effective targets `[0x30000410] +0x1ec` and `[0x30000410] +0x1e8` respectively (`move.b` at
  0xffe6caf2, `clr.b` at 0xffe6caf8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6cae4 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffe6caf0 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffe6caf2 | load | 1 | `[0x30000410] +0x1ec` | `-` |
| `move.b` at 0xffe6caf2 | store | 1 | `abs 0x30000650` | `([[0x30000410]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6caf6 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffe6caf8 | store | 1 | `[0x30000410] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6cafc | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffe6cb06 | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6cae8 | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6cb10 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-0/byte-status-facts.txt).

## 6.5.27:l1#0 at 0xffedcad8

- `0xffedcad8`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#0`, version `1.34.2`. Payload file offset 0x50.
  Load 0xffe00000. Analysed span 0xffedcad8 to 0xffedcb12.
  Established. A nonzero `[0x30000664]` falls through `beq.b` to the byte-copy and clear region
  (`beq.b` at 0xffedcae8). That region ends its writes with four-byte 0xfffffe70 at 0x30000668
  (`move.l` at 0xffedcb06). Zero instead takes the branch to the A5 restore (`beq.b` at 0xffedcae8,
  `movea.l` at 0xffedcb0a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedcae4 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffedcaf0 | load | 4 | `abs 0x30000410` | `-` |
| `move.b` at 0xffedcaf2 | load | 1 | `[0x30000410] +0x1ec` | `-` |
| `move.b` at 0xffedcaf2 | store | 1 | `abs 0x30000650` | `([[0x30000410]+492]:1 & 0xff)` |
| `movea.l` at 0xffedcaf6 | load | 4 | `abs 0x30000410` | `-` |
| `clr.b` at 0xffedcaf8 | store | 1 | `[0x30000410] +0x1e8` | `0x0` |
| `clr.l` at 0xffedcafc | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffedcb06 | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedcae8 | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedcb10 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-0/byte-status-facts.txt).

## 6.5.27:l1#1 at 0xffe6c974

- `0xffe6c974`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#1`, version `1.34.2`. Payload file offset 0xefb74.
  Load 0xffe00000. Analysed span 0xffe6c974 to 0xffe6c9ae.
  Established. The instruction at 0xffe6c98e performs both the byte read through `[0x3000043c]
  +0x1ec` and the byte store at 0x3000067c (`move.b` at 0xffe6c98e). The next write clears a byte
  through `[0x3000043c] +0x1e8`, before a separate four-byte clear at 0x30000690 (`clr.b` at
  0xffe6c994, `clr.l` at 0xffe6c998).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6c980 | load | 4 | `abs 0x30000690` | `-` |
| `movea.l` at 0xffe6c98c | load | 4 | `abs 0x3000043c` | `-` |
| `move.b` at 0xffe6c98e | load | 1 | `[0x3000043c] +0x1ec` | `-` |
| `move.b` at 0xffe6c98e | store | 1 | `abs 0x3000067c` | `([[0x3000043c]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6c992 | load | 4 | `abs 0x3000043c` | `-` |
| `clr.b` at 0xffe6c994 | store | 1 | `[0x3000043c] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6c998 | store | 4 | `abs 0x30000690` | `0x0` |
| `move.l` at 0xffe6c9a2 | store | 4 | `abs 0x30000694` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6c984 | `[0x30000690] == 0x0` | `[0x30000690] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6c9ac | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-1/byte-status-facts.txt).

## 6.5.27:l1#1 at 0xffedc974

- `0xffedc974`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#1`, version `1.34.2`. Payload file offset 0xefb74.
  Load 0xffe00000. Analysed span 0xffedc974 to 0xffedc9ae.
  Established. The common tail reloads A5 from entry stack -0x8, removes the frame and returns
  (`movea.l` at 0xffedc9a6, `unlk` at 0xffedc9aa, `rts` at 0xffedc9ac). `beq.b` reaches that tail
  without the intervening writes when `[0x30000690] == 0` (`beq.b` at 0xffedc984). The fall-through
  route includes four-byte 0xfffffe70 at 0x30000694 (`move.l` at 0xffedc9a2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedc980 | load | 4 | `abs 0x30000690` | `-` |
| `movea.l` at 0xffedc98c | load | 4 | `abs 0x3000043c` | `-` |
| `move.b` at 0xffedc98e | load | 1 | `[0x3000043c] +0x1ec` | `-` |
| `move.b` at 0xffedc98e | store | 1 | `abs 0x3000067c` | `([[0x3000043c]+492]:1 & 0xff)` |
| `movea.l` at 0xffedc992 | load | 4 | `abs 0x3000043c` | `-` |
| `clr.b` at 0xffedc994 | store | 1 | `[0x3000043c] +0x1e8` | `0x0` |
| `clr.l` at 0xffedc998 | store | 4 | `abs 0x30000690` | `0x0` |
| `move.l` at 0xffedc9a2 | store | 4 | `abs 0x30000694` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedc984 | `[0x30000690] == 0x0` | `[0x30000690] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedc9ac | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-1/byte-status-facts.txt).

## 6.5.27:l1#2 at 0xffeaf638

- `0xffeaf638`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#2`, version `1.34.2`. Payload file offset 0x1df678.
  Load 0xffe00000. Analysed span 0xffeaf638 to 0xffeaf672.
  Established. The byte load at `[0x30000a8c] +0x1ec` and store at 0x30000ce4 share one instruction
  (`move.b` at 0xffeaf652). A following store of byte zero uses a fresh four-byte load from 0x30000a8c
  as its base (`movea.l` at 0xffeaf656, `clr.b` at 0xffeaf658). The entry test can bypass both
  operations for zero `[0x30000cf8]` (`beq.b` at 0xffeaf648).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffeaf644 | load | 4 | `abs 0x30000cf8` | `-` |
| `movea.l` at 0xffeaf650 | load | 4 | `abs 0x30000a8c` | `-` |
| `move.b` at 0xffeaf652 | load | 1 | `[0x30000a8c] +0x1ec` | `-` |
| `move.b` at 0xffeaf652 | store | 1 | `abs 0x30000ce4` | `([[0x30000a8c]+492]:1 & 0xff)` |
| `movea.l` at 0xffeaf656 | load | 4 | `abs 0x30000a8c` | `-` |
| `clr.b` at 0xffeaf658 | store | 1 | `[0x30000a8c] +0x1e8` | `0x0` |
| `clr.l` at 0xffeaf65c | store | 4 | `abs 0x30000cf8` | `0x0` |
| `move.l` at 0xffeaf666 | store | 4 | `abs 0x30000cfc` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffeaf648 | `[0x30000cf8] == 0x0` | `[0x30000cf8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffeaf670 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-2/byte-status-facts.txt).

## 6.5.27:l1#2 at 0xfff9f638

- `0xfff9f638`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#2`, version `1.34.2`. Payload file offset 0x1df678.
  Load 0xffe00000. Analysed span 0xfff9f638 to 0xfff9f672.
  Established. Four-byte stores at 0x30000cf8 and 0x30000cfc carry zero and 0xfffffe70 respectively
  (`clr.l` at 0xfff9f65c, `move.l` at 0xfff9f666). They follow the pointer-relative byte clear
  through `[0x30000a8c] +0x1e8` (`clr.b` at 0xfff9f658). The single conditional branch takes its
  skip for `[0x30000cf8] == 0` (`beq.b` at 0xfff9f648).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xfff9f644 | load | 4 | `abs 0x30000cf8` | `-` |
| `movea.l` at 0xfff9f650 | load | 4 | `abs 0x30000a8c` | `-` |
| `move.b` at 0xfff9f652 | load | 1 | `[0x30000a8c] +0x1ec` | `-` |
| `move.b` at 0xfff9f652 | store | 1 | `abs 0x30000ce4` | `([[0x30000a8c]+492]:1 & 0xff)` |
| `movea.l` at 0xfff9f656 | load | 4 | `abs 0x30000a8c` | `-` |
| `clr.b` at 0xfff9f658 | store | 1 | `[0x30000a8c] +0x1e8` | `0x0` |
| `clr.l` at 0xfff9f65c | store | 4 | `abs 0x30000cf8` | `0x0` |
| `move.l` at 0xfff9f666 | store | 4 | `abs 0x30000cfc` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff9f648 | `[0x30000cf8] == 0x0` | `[0x30000cf8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff9f670 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.34.2-package-2/byte-status-facts.txt).

## 6.5.28:l1#0 at 0xffe6cd5c

- `0xffe6cd5c`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#0`, version `1.38.4`. Payload file offset 0x58.
  Load 0xffe00000. Analysed span 0xffe6cd5c to 0xffe6cd96.
  Established. A byte copied through `[0x300002bc] +0x1ec` becomes the byte at 0x30000650 (`move.b`
  at 0xffe6cd76). The remaining write sites include byte zero through `[0x300002bc] +0x1e8`,
  four-byte zero at 0x30000664 and four-byte 0xfffffe70 at 0x30000668 (`clr.b` at 0xffe6cd7c,
  `clr.l` at 0xffe6cd80, `move.l` at 0xffe6cd8a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6cd68 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffe6cd74 | load | 4 | `abs 0x300002bc` | `-` |
| `move.b` at 0xffe6cd76 | load | 1 | `[0x300002bc] +0x1ec` | `-` |
| `move.b` at 0xffe6cd76 | store | 1 | `abs 0x30000650` | `([[0x300002bc]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6cd7a | load | 4 | `abs 0x300002bc` | `-` |
| `clr.b` at 0xffe6cd7c | store | 1 | `[0x300002bc] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6cd80 | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffe6cd8a | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6cd6c | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6cd94 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-0/byte-status-facts.txt).

## 6.5.28:l1#0 at 0xffedcd5c

- `0xffedcd5c`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#0`, version `1.38.4`. Payload file offset 0x58.
  Load 0xffe00000. Analysed span 0xffedcd5c to 0xffedcd96.
  Established. The guard reads four bytes at 0x30000664 and takes `beq.b` past the write sequence
  when that value is zero (`tst.l` at 0xffedcd68, `beq.b` at 0xffedcd6c); its nonzero route loads
  the base at 0x300002bc for both a byte read at +0x1ec and a byte clear at +0x1e8 (`movea.l` at
  0xffedcd74, `move.b` at 0xffedcd76, `movea.l` at 0xffedcd7a, `clr.b` at 0xffedcd7c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedcd68 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffedcd74 | load | 4 | `abs 0x300002bc` | `-` |
| `move.b` at 0xffedcd76 | load | 1 | `[0x300002bc] +0x1ec` | `-` |
| `move.b` at 0xffedcd76 | store | 1 | `abs 0x30000650` | `([[0x300002bc]+492]:1 & 0xff)` |
| `movea.l` at 0xffedcd7a | load | 4 | `abs 0x300002bc` | `-` |
| `clr.b` at 0xffedcd7c | store | 1 | `[0x300002bc] +0x1e8` | `0x0` |
| `clr.l` at 0xffedcd80 | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffedcd8a | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedcd6c | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedcd94 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-0/byte-status-facts.txt).

## 6.5.28:l1#1 at 0xffe6cc14

- `0xffe6cc14`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#1`, version `1.38.4`. Payload file offset 0xefdc4.
  Load 0xffe00000. Analysed span 0xffe6cc14 to 0xffe6cc4e.
  Established. The conditional region stores a byte at 0x3000067c, clears a byte through
  `[0x300002e8] +0x1e8`, and clears four bytes at 0x30000690 (`move.b` at 0xffe6cc2e, `clr.b` at
  0xffe6cc34, `clr.l` at 0xffe6cc38). A four-byte 0xfffffe70 store at 0x30000694 follows (`move.l`
  at 0xffe6cc42). These writes are skipped by `beq.b` for zero `[0x30000690]` (`beq.b` at
  0xffe6cc24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6cc20 | load | 4 | `abs 0x30000690` | `-` |
| `movea.l` at 0xffe6cc2c | load | 4 | `abs 0x300002e8` | `-` |
| `move.b` at 0xffe6cc2e | load | 1 | `[0x300002e8] +0x1ec` | `-` |
| `move.b` at 0xffe6cc2e | store | 1 | `abs 0x3000067c` | `([[0x300002e8]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6cc32 | load | 4 | `abs 0x300002e8` | `-` |
| `clr.b` at 0xffe6cc34 | store | 1 | `[0x300002e8] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6cc38 | store | 4 | `abs 0x30000690` | `0x0` |
| `move.l` at 0xffe6cc42 | store | 4 | `abs 0x30000694` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6cc24 | `[0x30000690] == 0x0` | `[0x30000690] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6cc4c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-1/byte-status-facts.txt).
