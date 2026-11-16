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

**Not read.** The other frame writers of 1.40.5 were not re-enumerated. Review re-enumerated them for 1.38.4 only (section "Review"). Build dates: the 1.40.5 package is stamped 2005-12-05, before the SN2 PROM
builds that carry the wait-then-park code (4.55, 2006-06). Which L1 build sat in a machine that ran those PROMs is not established;
1.42.x and 1.46 to 1.58 are not held.

## Review (second reading of the 0x100b wait)

Independent review of sysco `1e9c304`, with chips `a1e9539` and the page [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md). Examined: the 1.38.4 `l1` carved from the 6.5.28 overlay with this repository's CramFS code (size and SHA-256 match the table) and the 1.62.1 `l1`, both in Ghidra (retools `441b26c`). 1.40.5 was not opened.

**Confirmed for 1.38.4.** The literal at `0x892e8` is `0x100b` (the `0x100a` literal at `0x892e4`, `0x1009` at `0x892d8`). The handler requires the argument count byte to be 4, otherwise `ARG__START` with `0xffffff9c`. The word is packed as `arg2 << 24 | (arg3 & 0xffff) << 8 | arg1 & 0xff`, so (1, 5, 4) gives `0x05000401`, stored with `vramSet(slot + 0x2a, word)`. The log string at file offset `0x10a18c` reads `N%d: PROM %x.%x (PR %d)`. `SysComEventDisable` runs when the word at `+0xc` of the block at `0x2b1720` is not -1 and `SysComSlab` of it equals the slot, and it stores -1 and calls `nvramSet(0x13, 0)`. `ARG__START` is called only in the count branch. `L1ReqInit` registers `L1ReqIRouter` (`0x882a0`) as a direct function for `0xffff006`; the router (`0xb5d98`) calls it and returns 0 with no reply.

**Added by the review.** The same four routines call `PPPWrite` in 1.38.4 as in 1.62.1 (the response, chunked-response, event and request writers). The review also said the event word is not initialised to -1 in 1.38.4; that was wrong, see the next section. No frame path that could end the PROM's wait was found in 1.38.4 or 1.62.1. Which L1 tasks send packets to the SysCom tasks (the console request sources `0x68308` and `0x68430`) was not traced then; the next section traces it.

**Not checked.** 1.40.5, 1.44.0 and 1.60.3; the request and response writers' gates in 1.38.4 beyond the call graph; whether anything raises an event in 1.38.4 before the node's first request; the claims in the page about the 1.40.5 addresses.

## Review (third reading: event word and the senders)

Ghidra export of the 1.38.4 `l1` (`f652c04c...`), with the routines below cross-checked in a Capstone disassembly. The full table of what addresses the SysCom writers, and the answers for 1.62.1, are in [sn2-prom-link-1.62.1.md](sn2-prom-link-1.62.1.md), "Review (third reading: who addresses the SysCom writers)". 1.38.4 specifics:

- **Event word starts at -1 (established; corrects the paragraph above).** `SysComInit` ends in a jump table whose cases Ghidra's decompile does not show. For the 13 brick-type characters with a case (`5 @ ^ + B F L Z M | J K D`) the tail stores -1 in the word at `0x2b1720 + 0xc` (`0x83600`), then, if `exceptionIsSoftReset` (`HW_LAST_RESET` equal to `soft`) is true, reloads it with `nvramGet(0x13)` and sets it to -1 again unless the signed value is above 3 (`0x83620` to `0x83638`). Every other type goes to `0x836c0` and returns: no outbound task, no SysCom writer. The word's only other writers are the `0x1007` handler (`SysComTasknum(slab, argument)`) and `SysComEventDisable`; a scan of every load of the block address for a store at offset `0xc` found no more. So a cold start cannot send an event, and the stale-word case needs a soft reset of the L1 with a previous session's subscription.
- **Outbound task (established).** `0x85860` (not entered by Ghidra) registers destinations `0xffff005` and `0xffff015`, then an all-events queue, and calls `syscom_process_outbound_packet` per packet.
- **The two console sources.** `0x68308` sends one typed character, as a request to task 5, from `SMPLibrary` in pass-through mode (mode 1, entered by `^D` from the command-line mode 3 that `SMPInit` starts in). `0x68430` sends the typed line to task 3, the L1 command interpreter, and is not a SysCom sender. Neither fires without a person (established). The destination words are `SMPInit`'s `0x254` (task 5) and `0x256` (task 3).
- **Request writer (`0x84518`).** No `?` branch: header is the bare subchannel (class 0), so for index 5 and up the node's receiver rejects it.
- **Event sources.** Only events whose source task is 4 (the environment task, through `EnvEvent`, or a request to the event task through `EvtIRouter`) pass the writer; they follow fan, power-rail, link-error, power-down, power-bay and test conditions, and the writer sends nothing while the word is -1.
- **Not read.** 1.40.5, and which L1 commands name task 5.


## Review

Independent check of the third-reading section, in Ghidra on the 1.38.4 `l1` (`f652c04c...`). `SysComInit` (`0x832e8`) does memset the block (`0x84` bytes) and, for the 13 cased brick types, falls through to `0x835d0`: store -1 at block `+0xc` (`0x83600`), `exceptionIsSoftReset` (`0x83604`), then `nvramGet(0x13)` stored at `+0xc` and set back to -1 unless the signed value is above 3 (`0x83628` bgt, `0x83638`). The table has 13 non-default entries, `+ 5 @ B D F J K L M Z ^ |`. The single outbound task is created at `0x836bc` and runs `0x85860`, which registers queues and loops on `IRouterGetPacket`. The event writer (`0x84994`) is gated by word != -1 and source task 4; the request writer (`0x84518`) has no `?` branch. All as stated above. 1.40.5 was not read. The description of the outbound task in the 1.62.1 page needed a correction (per-slab tasks in 1.62.1), which does not change any conclusion here.
