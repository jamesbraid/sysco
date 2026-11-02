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

## 6.5.28:l1#1 at 0xffedcc14

- `0xffedcc14`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#1`, version `1.38.4`. Payload file offset 0xefdc4.
  Load 0xffe00000. Analysed span 0xffedcc14 to 0xffedcc4e.
  Established. The conditional store sequence includes byte zero through `[0x300002e8] +0x1e8` and
  four-byte zero at 0x30000690 (`clr.b` at 0xffedcc34, `clr.l` at 0xffedcc38). It is bypassed by
  `beq.b` when `[0x30000690] == 0` (`beq.b` at 0xffedcc24). A final four-byte store in the
  fall-through path writes 0xfffffe70 at 0x30000694 (`move.l` at 0xffedcc42).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedcc20 | load | 4 | `abs 0x30000690` | `-` |
| `movea.l` at 0xffedcc2c | load | 4 | `abs 0x300002e8` | `-` |
| `move.b` at 0xffedcc2e | load | 1 | `[0x300002e8] +0x1ec` | `-` |
| `move.b` at 0xffedcc2e | store | 1 | `abs 0x3000067c` | `([[0x300002e8]+492]:1 & 0xff)` |
| `movea.l` at 0xffedcc32 | load | 4 | `abs 0x300002e8` | `-` |
| `clr.b` at 0xffedcc34 | store | 1 | `[0x300002e8] +0x1e8` | `0x0` |
| `clr.l` at 0xffedcc38 | store | 4 | `abs 0x30000690` | `0x0` |
| `move.l` at 0xffedcc42 | store | 4 | `abs 0x30000694` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedcc24 | `[0x30000690] == 0x0` | `[0x30000690] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedcc4c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-1/byte-status-facts.txt).

## 6.5.28:l1#2 at 0xffe8e844

- `0xffe8e844`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#2`, version `1.38.4`. Payload file offset 0x1dfb30.
  Load 0xffe00000. Analysed span 0xffe8e844 to 0xffe8e87e.
  Established. The byte-copy instruction reads through `[0x300008bc] +0x1ec` and writes at
  0x30000c78 (`move.b` at 0xffe8e85e). Later, a separate instruction clears a byte through
  `[0x300008bc] +0x1e8` (`clr.b` at 0xffe8e864). The guard selects the restore tail for zero
  `[0x30000c8c]` (`beq.b` at 0xffe8e854).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe8e850 | load | 4 | `abs 0x30000c8c` | `-` |
| `movea.l` at 0xffe8e85c | load | 4 | `abs 0x300008bc` | `-` |
| `move.b` at 0xffe8e85e | load | 1 | `[0x300008bc] +0x1ec` | `-` |
| `move.b` at 0xffe8e85e | store | 1 | `abs 0x30000c78` | `([[0x300008bc]+492]:1 & 0xff)` |
| `movea.l` at 0xffe8e862 | load | 4 | `abs 0x300008bc` | `-` |
| `clr.b` at 0xffe8e864 | store | 1 | `[0x300008bc] +0x1e8` | `0x0` |
| `clr.l` at 0xffe8e868 | store | 4 | `abs 0x30000c8c` | `0x0` |
| `move.l` at 0xffe8e872 | store | 4 | `abs 0x30000c90` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe8e854 | `[0x30000c8c] == 0x0` | `[0x30000c8c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe8e87c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-2/byte-status-facts.txt).

## 6.5.28:l1#2 at 0xfff7e844

- `0xfff7e844`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#2`, version `1.38.4`. Payload file offset 0x1dfb30.
  Load 0xffe00000. Analysed span 0xfff7e844 to 0xfff7e87e.
  Established. A5 is restored from entry stack -0x8 before the frame is removed and the routine
  returns (`movea.l` at 0xfff7e876, `unlk` at 0xfff7e87a, `rts` at 0xfff7e87c). The earlier `beq.b`
  reaches that restore without executing the byte-copy path for zero `[0x30000c8c]` (`beq.b` at
  0xfff7e854). On fall-through, the byte at 0x30000c78 receives the read from `[0x300008bc] +0x1ec`
  (`beq.b` at 0xfff7e854, `move.b` at 0xfff7e85e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xfff7e850 | load | 4 | `abs 0x30000c8c` | `-` |
| `movea.l` at 0xfff7e85c | load | 4 | `abs 0x300008bc` | `-` |
| `move.b` at 0xfff7e85e | load | 1 | `[0x300008bc] +0x1ec` | `-` |
| `move.b` at 0xfff7e85e | store | 1 | `abs 0x30000c78` | `([[0x300008bc]+492]:1 & 0xff)` |
| `movea.l` at 0xfff7e862 | load | 4 | `abs 0x300008bc` | `-` |
| `clr.b` at 0xfff7e864 | store | 1 | `[0x300008bc] +0x1e8` | `0x0` |
| `clr.l` at 0xfff7e868 | store | 4 | `abs 0x30000c8c` | `0x0` |
| `move.l` at 0xfff7e872 | store | 4 | `abs 0x30000c90` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff7e854 | `[0x30000c8c] == 0x0` | `[0x30000c8c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff7e87c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-2/byte-status-facts.txt).

## 6.5.28:l1#3 at 0xffe93288

- `0xffe93288`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#3`, version `1.38.4`. Payload file offset 0x38c214.
  Load 0xffe00000. Analysed span 0xffe93288 to 0xffe932c2.
  Established. A four-byte read at 0x30000e04 controls the only conditional branch (`tst.l` at
  0xffe93294, `beq.b` at 0xffe93298). For a nonzero tested value, `beq.b` falls through to a byte read
  through `[0x30000ba8] +0x1ec` and a byte store at 0x30000df0 (`beq.b` at 0xffe93298, `move.b` at
  0xffe932a2). Zero takes the branch past the intervening access sites (`beq.b` at 0xffe93298).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe93294 | load | 4 | `abs 0x30000e04` | `-` |
| `movea.l` at 0xffe932a0 | load | 4 | `abs 0x30000ba8` | `-` |
| `move.b` at 0xffe932a2 | load | 1 | `[0x30000ba8] +0x1ec` | `-` |
| `move.b` at 0xffe932a2 | store | 1 | `abs 0x30000df0` | `([[0x30000ba8]+492]:1 & 0xff)` |
| `movea.l` at 0xffe932a6 | load | 4 | `abs 0x30000ba8` | `-` |
| `clr.b` at 0xffe932a8 | store | 1 | `[0x30000ba8] +0x1e8` | `0x0` |
| `clr.l` at 0xffe932ac | store | 4 | `abs 0x30000e04` | `0x0` |
| `move.l` at 0xffe932b6 | store | 4 | `abs 0x30000e08` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe93298 | `[0x30000e04] == 0x0` | `[0x30000e04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe932c0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-3/byte-status-facts.txt).

## 6.5.28:l1#3 at 0xfff83288

- `0xfff83288`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#3`, version `1.38.4`. Payload file offset 0x38c214.
  Load 0xffe00000. Analysed span 0xfff83288 to 0xfff832c2.
  Established. The two clears in this path have different widths: one byte through `[0x30000ba8]
  +0x1e8`, then four bytes at 0x30000e04 (`clr.b` at 0xfff832a8, `clr.l` at 0xfff832ac). The
  four-byte store of 0xfffffe70 at 0x30000e08 follows them (`move.l` at 0xfff832b6). The `beq.b` path for zero `[0x30000e04]` bypasses all three (`beq.b` at 0xfff83298).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xfff83294 | load | 4 | `abs 0x30000e04` | `-` |
| `movea.l` at 0xfff832a0 | load | 4 | `abs 0x30000ba8` | `-` |
| `move.b` at 0xfff832a2 | load | 1 | `[0x30000ba8] +0x1ec` | `-` |
| `move.b` at 0xfff832a2 | store | 1 | `abs 0x30000df0` | `([[0x30000ba8]+492]:1 & 0xff)` |
| `movea.l` at 0xfff832a6 | load | 4 | `abs 0x30000ba8` | `-` |
| `clr.b` at 0xfff832a8 | store | 1 | `[0x30000ba8] +0x1e8` | `0x0` |
| `clr.l` at 0xfff832ac | store | 4 | `abs 0x30000e04` | `0x0` |
| `move.l` at 0xfff832b6 | store | 4 | `abs 0x30000e08` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff83298 | `[0x30000e04] == 0x0` | `[0x30000e04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff832c0 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.38.4-package-3/byte-status-facts.txt).

## 6.5.29:l1#0 at 0xffe6cd6c

- `0xffe6cd6c`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#0`, version `1.40.5`. Payload file offset 0x58.
  Load 0xffe00000. Analysed span 0xffe6cd6c to 0xffe6cda6.
  Established. When `[0x30000664]` is nonzero, `beq.b` falls through to the sequence using the value
  loaded at 0x3000031c (`beq.b` at 0xffe6cd7c, `movea.l` at 0xffe6cd84). A byte is copied from that
  value +0x1ec to 0x30000650 (`move.b` at 0xffe6cd86). The subsequent four-byte writes clear
  0x30000664 and put 0xfffffe70 at 0x30000668 (`clr.l` at 0xffe6cd90, `move.l` at 0xffe6cd9a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6cd78 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffe6cd84 | load | 4 | `abs 0x3000031c` | `-` |
| `move.b` at 0xffe6cd86 | load | 1 | `[0x3000031c] +0x1ec` | `-` |
| `move.b` at 0xffe6cd86 | store | 1 | `abs 0x30000650` | `([[0x3000031c]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6cd8a | load | 4 | `abs 0x3000031c` | `-` |
| `clr.b` at 0xffe6cd8c | store | 1 | `[0x3000031c] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6cd90 | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffe6cd9a | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6cd7c | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6cda4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-0/byte-status-facts.txt).

## 6.5.29:l1#0 at 0xffedcd6c

- `0xffedcd6c`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#0`, version `1.40.5`. Payload file offset 0x58.
  Load 0xffe00000. Analysed span 0xffedcd6c to 0xffedcda6.
  Established. The byte clear through `[0x3000031c] +0x1e8` is preceded by a fresh four-byte pointer
  load from 0x3000031c (`movea.l` at 0xffedcd8a, `clr.b` at 0xffedcd8c). An earlier load from that
  address supplies the base for the byte copy to 0x30000650 (`movea.l` at 0xffedcd84, `move.b` at
  0xffedcd86). The entry branch skips both operations for zero `[0x30000664]` (`beq.b` at
  0xffedcd7c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedcd78 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffedcd84 | load | 4 | `abs 0x3000031c` | `-` |
| `move.b` at 0xffedcd86 | load | 1 | `[0x3000031c] +0x1ec` | `-` |
| `move.b` at 0xffedcd86 | store | 1 | `abs 0x30000650` | `([[0x3000031c]+492]:1 & 0xff)` |
| `movea.l` at 0xffedcd8a | load | 4 | `abs 0x3000031c` | `-` |
| `clr.b` at 0xffedcd8c | store | 1 | `[0x3000031c] +0x1e8` | `0x0` |
| `clr.l` at 0xffedcd90 | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffedcd9a | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedcd7c | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedcda4 | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-0/byte-status-facts.txt).

## 6.5.29:l1#1 at 0xffe6cc24

- `0xffe6cc24`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#1`, version `1.40.5`. Payload file offset 0xefdd4.
  Load 0xffe00000. Analysed span 0xffe6cc24 to 0xffe6cc5e.
  Established. Four bytes at 0x30000690 are tested before `beq.b` (`tst.l` at 0xffe6cc30, `beq.b` at
  0xffe6cc34); its nonzero fall-through reads one byte through `[0x30000348] +0x1ec` and stores it
  at 0x3000067c (`beq.b` at 0xffe6cc34, `move.b` at 0xffe6cc3e). The later store is
  four bytes wide and carries 0xfffffe70 to 0x30000694 (`move.l` at 0xffe6cc52).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6cc30 | load | 4 | `abs 0x30000690` | `-` |
| `movea.l` at 0xffe6cc3c | load | 4 | `abs 0x30000348` | `-` |
| `move.b` at 0xffe6cc3e | load | 1 | `[0x30000348] +0x1ec` | `-` |
| `move.b` at 0xffe6cc3e | store | 1 | `abs 0x3000067c` | `([[0x30000348]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6cc42 | load | 4 | `abs 0x30000348` | `-` |
| `clr.b` at 0xffe6cc44 | store | 1 | `[0x30000348] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6cc48 | store | 4 | `abs 0x30000690` | `0x0` |
| `move.l` at 0xffe6cc52 | store | 4 | `abs 0x30000694` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6cc34 | `[0x30000690] == 0x0` | `[0x30000690] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6cc5c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-1/byte-status-facts.txt).

## 6.5.29:l1#1 at 0xffedcc24

- `0xffedcc24`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#1`, version `1.40.5`. Payload file offset 0xefdd4.
  Load 0xffe00000. Analysed span 0xffedcc24 to 0xffedcc5e.
  Established. The write region ends with a four-byte clear at 0x30000690 followed by four-byte
  0xfffffe70 at 0x30000694 (`clr.l` at 0xffedcc48, `move.l` at 0xffedcc52). Before those stores, the
  copied byte goes from `[0x30000348] +0x1ec` to 0x3000067c (`move.b` at 0xffedcc3e). For zero `[0x30000690]`, `beq.b` takes the restore route (`beq.b` at 0xffedcc34).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedcc30 | load | 4 | `abs 0x30000690` | `-` |
| `movea.l` at 0xffedcc3c | load | 4 | `abs 0x30000348` | `-` |
| `move.b` at 0xffedcc3e | load | 1 | `[0x30000348] +0x1ec` | `-` |
| `move.b` at 0xffedcc3e | store | 1 | `abs 0x3000067c` | `([[0x30000348]+492]:1 & 0xff)` |
| `movea.l` at 0xffedcc42 | load | 4 | `abs 0x30000348` | `-` |
| `clr.b` at 0xffedcc44 | store | 1 | `[0x30000348] +0x1e8` | `0x0` |
| `clr.l` at 0xffedcc48 | store | 4 | `abs 0x30000690` | `0x0` |
| `move.l` at 0xffedcc52 | store | 4 | `abs 0x30000694` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedcc34 | `[0x30000690] == 0x0` | `[0x30000690] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedcc5c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-1/byte-status-facts.txt).

## 6.5.29:l1#2 at 0xffe8e854

- `0xffe8e854`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#2`, version `1.40.5`. Payload file offset 0x1dfb50.
  Load 0xffe00000. Analysed span 0xffe8e854 to 0xffe8e88e.
  Established. A zero `[0x30000c8c]` takes `beq.b` past the byte access sites (`beq.b` at
  0xffe8e864). Otherwise, the read through `[0x3000092c] +0x1ec` feeds the store at 0x30000c78
  (`move.b` at 0xffe8e86e). The following byte clear through `[0x3000092c] +0x1e8` uses a separately
  loaded base (`movea.l` at 0xffe8e872, `clr.b` at 0xffe8e874).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe8e860 | load | 4 | `abs 0x30000c8c` | `-` |
| `movea.l` at 0xffe8e86c | load | 4 | `abs 0x3000092c` | `-` |
| `move.b` at 0xffe8e86e | load | 1 | `[0x3000092c] +0x1ec` | `-` |
| `move.b` at 0xffe8e86e | store | 1 | `abs 0x30000c78` | `([[0x3000092c]+492]:1 & 0xff)` |
| `movea.l` at 0xffe8e872 | load | 4 | `abs 0x3000092c` | `-` |
| `clr.b` at 0xffe8e874 | store | 1 | `[0x3000092c] +0x1e8` | `0x0` |
| `clr.l` at 0xffe8e878 | store | 4 | `abs 0x30000c8c` | `0x0` |
| `move.l` at 0xffe8e882 | store | 4 | `abs 0x30000c90` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe8e864 | `[0x30000c8c] == 0x0` | `[0x30000c8c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe8e88c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-2/byte-status-facts.txt).

## 6.5.29:l1#2 at 0xfff7e854

- `0xfff7e854`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#2`, version `1.40.5`. Payload file offset 0x1dfb50.
  Load 0xffe00000. Analysed span 0xfff7e854 to 0xfff7e88e.
  Established. The final two stores write four-byte zero at 0x30000c8c and four-byte 0xfffffe70 at
  0x30000c90 (`clr.l` at 0xfff7e878, `move.l` at 0xfff7e882). Earlier in the same fall-through
  region, a single instruction copies the byte read through `[0x3000092c] +0x1ec` to 0x30000c78
  (`move.b` at 0xfff7e86e). `beq.b` bypasses the region when `[0x30000c8c] == 0` (`beq.b` at
  0xfff7e864).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xfff7e860 | load | 4 | `abs 0x30000c8c` | `-` |
| `movea.l` at 0xfff7e86c | load | 4 | `abs 0x3000092c` | `-` |
| `move.b` at 0xfff7e86e | load | 1 | `[0x3000092c] +0x1ec` | `-` |
| `move.b` at 0xfff7e86e | store | 1 | `abs 0x30000c78` | `([[0x3000092c]+492]:1 & 0xff)` |
| `movea.l` at 0xfff7e872 | load | 4 | `abs 0x3000092c` | `-` |
| `clr.b` at 0xfff7e874 | store | 1 | `[0x3000092c] +0x1e8` | `0x0` |
| `clr.l` at 0xfff7e878 | store | 4 | `abs 0x30000c8c` | `0x0` |
| `move.l` at 0xfff7e882 | store | 4 | `abs 0x30000c90` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff7e864 | `[0x30000c8c] == 0x0` | `[0x30000c8c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff7e88c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-2/byte-status-facts.txt).

## 6.5.29:l1#3 at 0xffe93604

- `0xffe93604`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#3`, version `1.40.5`. Payload file offset 0x38c244.
  Load 0xffe00000. Analysed span 0xffe93604 to 0xffe9363e.
  Established. The four-byte guard location and the later four-byte clear target are both 0x30000e04
  (`tst.l` at 0xffe93610, `clr.l` at 0xffe93628). A zero tested value takes `beq.b` past the clear
  and the other writes (`beq.b` at 0xffe93614). The fall-through path also stores the byte from
  `[0x30000ba8] +0x1ec` at 0x30000df0 (`move.b` at 0xffe9361e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe93610 | load | 4 | `abs 0x30000e04` | `-` |
| `movea.l` at 0xffe9361c | load | 4 | `abs 0x30000ba8` | `-` |
| `move.b` at 0xffe9361e | load | 1 | `[0x30000ba8] +0x1ec` | `-` |
| `move.b` at 0xffe9361e | store | 1 | `abs 0x30000df0` | `([[0x30000ba8]+492]:1 & 0xff)` |
| `movea.l` at 0xffe93622 | load | 4 | `abs 0x30000ba8` | `-` |
| `clr.b` at 0xffe93624 | store | 1 | `[0x30000ba8] +0x1e8` | `0x0` |
| `clr.l` at 0xffe93628 | store | 4 | `abs 0x30000e04` | `0x0` |
| `move.l` at 0xffe93632 | store | 4 | `abs 0x30000e08` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe93614 | `[0x30000e04] == 0x0` | `[0x30000e04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe9363c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-3/byte-status-facts.txt).

## 6.5.29:l1#3 at 0xfff83604

- `0xfff83604`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#3`, version `1.40.5`. Payload file offset 0x38c244.
  Load 0xffe00000. Analysed span 0xfff83604 to 0xfff8363e.
  Established. The two pointer loads read four bytes at 0x30000ba8 before the byte-copy and
  byte-clear instructions (`movea.l` at 0xfff8361c, `movea.l` at 0xfff83622). Those instructions use
  offsets +0x1ec for the read and +0x1e8 for the zero store (`move.b` at 0xfff8361e, `clr.b` at
  0xfff83624). The copied byte is written at 0x30000df0 (`move.b` at 0xfff8361e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xfff83610 | load | 4 | `abs 0x30000e04` | `-` |
| `movea.l` at 0xfff8361c | load | 4 | `abs 0x30000ba8` | `-` |
| `move.b` at 0xfff8361e | load | 1 | `[0x30000ba8] +0x1ec` | `-` |
| `move.b` at 0xfff8361e | store | 1 | `abs 0x30000df0` | `([[0x30000ba8]+492]:1 & 0xff)` |
| `movea.l` at 0xfff83622 | load | 4 | `abs 0x30000ba8` | `-` |
| `clr.b` at 0xfff83624 | store | 1 | `[0x30000ba8] +0x1e8` | `0x0` |
| `clr.l` at 0xfff83628 | store | 4 | `abs 0x30000e04` | `0x0` |
| `move.l` at 0xfff83632 | store | 4 | `abs 0x30000e08` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xfff83614 | `[0x30000e04] == 0x0` | `[0x30000e04] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xfff8363c | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.40.5-package-3/byte-status-facts.txt).

## 6.5.30:l1#0 at 0xffe6cd84

- `0xffe6cd84`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#0`, version `1.44.0`. Payload file offset 0x68.
  Load 0xffe00000. Analysed span 0xffe6cd84 to 0xffe6cdbe.
  Established. The path selected by nonzero `[0x30000664]` includes a byte read through
  `[0x300002bc] +0x1ec`, a byte store at 0x30000650, and a byte clear through `[0x300002bc] +0x1e8`
  (`beq.b` at 0xffe6cd94, `move.b` at 0xffe6cd9e, `clr.b` at 0xffe6cda4). The later four-byte stores
  carry zero at 0x30000664 and 0xfffffe70 at 0x30000668 (`clr.l` at 0xffe6cda8, `move.l` at
  0xffe6cdb2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffe6cd90 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffe6cd9c | load | 4 | `abs 0x300002bc` | `-` |
| `move.b` at 0xffe6cd9e | load | 1 | `[0x300002bc] +0x1ec` | `-` |
| `move.b` at 0xffe6cd9e | store | 1 | `abs 0x30000650` | `([[0x300002bc]+492]:1 & 0xff)` |
| `movea.l` at 0xffe6cda2 | load | 4 | `abs 0x300002bc` | `-` |
| `clr.b` at 0xffe6cda4 | store | 1 | `[0x300002bc] +0x1e8` | `0x0` |
| `clr.l` at 0xffe6cda8 | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffe6cdb2 | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffe6cd94 | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffe6cdbc | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-0/byte-status-facts.txt).

## 6.5.30:l1#0 at 0xffedcd84

- `0xffedcd84`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#0`, version `1.44.0`. Payload file offset 0x68.
  Load 0xffe00000. Analysed span 0xffedcd84 to 0xffedcdbe.
  Established. The four-byte test at 0x30000664 feeds `beq.b`, taken for zero to the
  register-restore tail (`tst.l` at 0xffedcd90, `beq.b` at 0xffedcd94). Nonzero falls through to the
  pointer-relative byte copy and byte clear (`beq.b` at 0xffedcd94, `move.b` at 0xffedcd9e, `clr.b`
  at 0xffedcda4). Each access sequence obtains its base from a separate four-byte load at 0x300002bc
  (`movea.l` at 0xffedcd9c, `movea.l` at 0xffedcda2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `tst.l` at 0xffedcd90 | load | 4 | `abs 0x30000664` | `-` |
| `movea.l` at 0xffedcd9c | load | 4 | `abs 0x300002bc` | `-` |
| `move.b` at 0xffedcd9e | load | 1 | `[0x300002bc] +0x1ec` | `-` |
| `move.b` at 0xffedcd9e | store | 1 | `abs 0x30000650` | `([[0x300002bc]+492]:1 & 0xff)` |
| `movea.l` at 0xffedcda2 | load | 4 | `abs 0x300002bc` | `-` |
| `clr.b` at 0xffedcda4 | store | 1 | `[0x300002bc] +0x1e8` | `0x0` |
| `clr.l` at 0xffedcda8 | store | 4 | `abs 0x30000664` | `0x0` |
| `move.l` at 0xffedcdb2 | store | 4 | `abs 0x30000668` | `0xfffffe70` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq.b` at 0xffedcd94 | `[0x30000664] == 0x0` | `[0x30000664] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `rts` at 0xffedcdbc | `d0` | `?` |

[Full fact sheet and listing](disasm/mcf5206e-1.44.0-package-0/byte-status-facts.txt).
