# ARM L1 1.38.4 and 1.40.5: the 0x100b handler sends no reply

Request `0x100b` is the node firmware's version report (three integers: 1, then the PROM major and minor). The question was whether
the older ARM L1 builds that shipped when SGI's SN2 PROM 4.55 to 4.80 were current answer it, because a real brick must end the
node's 3,000,000-tick wait somehow ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md), "What could end the wait"). They do not. The
handler is the same code as in 1.44.0 and 1.62.1.

**Images (established, hashes recomputed).** The `l1` program carved from package 4 (`Linux L1 image`) of each container with this
repository's CramFS reader:

| L1 version | Container | Package 4 build stamp | `/stand/sysco/bin/l1` | SHA-256 |
|---|---|---|---|---|
| 1.38.4 | IRIX 6.5.28 overlays, `l1.bin` | 20050630000344 | 1,474,516 bytes, ARM, text at `0x8000` | `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8` |
| 1.40.5 | IRIX 6.5.29 overlays, `l1.bin` | 20051205140327 | 1,477,740 bytes | `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a` |

Both programs export their routine names. Read in Ghidra (retools ARM support: `L1ReqIRouter` decompiled whole, 20,356 and
20,428 bytes) and cross-checked by decoding the branch and the literal pool with Capstone (the host objdump has no ARM target).

**The handler (established for both builds).**

| | 1.38.4 | 1.40.5 |
|---|---|---|
| `L1ReqIRouter` | `0x882a0` | `0x88b50` |
| compare of the code with the literal `0x100b` (literal at `0x892e8` in 1.38.4) | `0x88680`, taken branch to `0x8abb0` | `0x88f2c`, taken branch to `0x8b460` |
| the `0x100a` handler (the previous code) | `0x8aba8`: a direct call to `SysComEventDisable` | same shape |
| `SysComEventDisable` | `0x85a0c` | `0x862bc` |

Steps of the handler, in order:

1. The argument count byte (offset `0x20` of the request) must be 4. Any other count calls `ARG__START` with status `-100`
   (`0xffffff9c`), the usual error reply. The node's three integers plus the code make 4, so this branch is not taken.
2. The three integer arguments are read (inline when the argument's type bits are zero, through `IRArg` otherwise) and packed as
   argument 2 in bits 31:24, argument 3 in bits 23:8 and argument 1 in bits 7:0. For the node's (1, 5, 4) that is
   `0x05000401` (inferred from the packing: the PROM's arguments 2 and 3 are its version bytes 5 and 4).
3. `vramSet(slot + 0x2a, word)` stores it, then `nvramLogFinsert` logs `N%d: PROM %x.%x (PR %d)` (string at file offset `0x10a18c`
   in 1.38.4, `0x10acdc` in 1.40.5).
4. If the event word (offset `0xc` of the state block named by a literal) is not -1 and `SysComSlab(event word)` equals this
   slot, `SysComEventDisable` runs. It stores -1 in the event word and writes setting `0x13` = 0. That cancels a stale event
   subscription; it sends nothing.
5. The handler jumps to the common exit. `ARG__START` is called only in the count-not-4 branch, and the router tail replies only
   when the reply-count word is set, as in 1.62.1 ([sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md)).

**Result.** Neither 1.38.4 nor 1.40.5 replies to `0x100b` for the node's form, and neither sends a frame from the handler. Grade:
established for the handler body in both builds. This agrees with 1.44.0, 1.60.3 and 1.62.1, so no ARM L1 build read, from 1.38.4 to
1.62.1, answers it.

**Not read.** The other frame writers of these two builds (the request, event, response and chunked-response writers enumerated for
1.62.1) were not re-enumerated; only the handler was. Build dates: the 1.40.5 package is stamped 2005-12-05, before the SN2 PROM
builds that carry the wait-then-park code (4.55, 2006-06). Which L1 build sat in a machine that ran those PROMs is not established;
1.42.x and 1.46 to 1.58 are not held.
