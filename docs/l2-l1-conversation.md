# The L2's side of the conversation with an L1

What the L2 daemon says to an L1 and what it does with the answers, in the order it happens: attach, the first and recurring configuration request, how the reply becomes a route, how a text command that arrives from a host (`p <id> pwr d`) turns into power commands for the bricks of one partition, and how a partition-record request addressed to controller type 1, task `0xa`, is answered. It joins the single-topic notes ([config-request-reply.md](l2/config-request-reply.md), [power-queue.md](l2/power-queue.md), [partition-task.md](l2/partition-task.md), [partition-record.md](l2/partition-record.md), [router-packet.md](l2/router-packet.md)) into one sequence, re-checks them against the listings, and corrects three readings in them (section 9).

## Sources and conventions

All L2 facts are read from L2 1.44.0, the `l2.bin` of IRIX 6.5.30 (2,924,652 bytes, sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761`), carved as in [l2/image-layout.md](l2/image-layout.md) and listed with `retools/tools/sysco/l2_list.py`. Addresses are virtual addresses inside the named file; executables load at `0x10000000`, libraries at 0.

| File | sha256 (first 16 digits) |
|---|---|
| `/stand/sysco/bin/l2` (248,344 bytes) | `9923b1ee573c2815` |
| `/stand/sysco/bin/l2part` (16,176 bytes) | `7fb4ccec5bf686f6` |
| `/stand/sysco/lib/libmain.ppclinux.so` (146,708 bytes) | `0050dcb2a9616f79` |
| `/stand/sysco/lib/libsys.ppclinux.so` (108,268 bytes) | `0197021c7f083a7b` |
| `/stand/sysco/lib/libIRouter.ppclinux.so` | `a0d9885b4b512d5f` |
| `/stand/sysco/lib/libpart.ppclinux.so` | `656962b8414fff65` |

One fact about the stock IP35 boot firmware is read from its payload, `ip35prom.payload.bin` (1,476,264 bytes, sha256 `554924380189a03a512156af2faa38344902f251b49fd5c80c253c23ecaff27b`, from the same IRIX 6.5.30 set). Its offsets below are file offsets of that payload; jump targets in its code show a load base of `0x0fc00000` (inferred from `jal` targets that equal file offset plus that base). The L1 half of the exchange is cited from [l1/packet-chunks-and-l2-attach.md](l1/packet-chunks-and-l2-attach.md) and [l2/config-request-reply.md](l2/config-request-reply.md); it was not re-read here, and its grades are those notes' grades. Only L2 1.44.0 was read; the other three held sets were not compared.

Grades follow [../AGENTS.md](../AGENTS.md): **established** (E, the whole routine or table was read and the claim follows), **inferred** (I, with the reasoning), **unknown** (U, with the software test that would settle it in section 10). A fact that comes from another note of this repository and was not re-derived says so.

Words used throughout. A **router word** is a 32-bit address: with bit 31 clear its top nibble is the form (0 for an L1, 1 for an L2, 2 for an L3) and the low five bits are the task; with bit 31 set the top two bits `10` give form 14 (task in bits 29 to 24) and anything else form 15. For form 0 the **brick address** is `rack << 18 | slot << 12` (rack ten bits, slot six bits), the **module field** is bits 12 to 27, and the value `0xffff` there means "the directly attached controller". A **brick record** is one 0x30-byte entry of a port's table in the L2's configuration object (layout in section 3).

## 1. The sequence in one table

| Step | What the L2 does | Grade | Section |
|---|---|---|---|
| 1 | A USB L1 device is opened; the L2 records a port in state 1 ("attached, not configured") with no brick records and arms a one-shot timer for 750,000 microseconds | E | 2 |
| 2 | The timer sends the L1 a configuration request: class 0, code 0, destination `0x0ffff00e`, three descriptors (integer 6, the system serial string or integer 0, integer 0) | E | 2 |
| 3 | The reply (destination task 4 of the L2, source form 0, status 0, selector 7) is parsed into brick records; the port goes to state 2 | E | 3 |
| 4 | The request repeats every 2 s until a reply has been parsed, then every NVRAM key 15 seconds (5 by default); a reply that changes the table forces one extra request after 1 s and a configuration check | E | 5 |
| 5 | Every packet for a destination `rack.slot` is sent to the port whose table holds a valid record with that rack and slot | E | 4 |
| 6 | A text command `p <id> pwr d` arriving for task 3 is parsed by the L2's own command interpreter; the partition `<id>` selects the non-router bricks whose table record carries that partition number; each gets the text `power down` as a task-3 request | E | 6 |
| 7 | A request for task `0xa` is handed to the local `l2part` client, which answers code 1 with the partition number of a brick | E | 7 |

## 2. Attach and the first request

**Port record.** The L2's configuration object is at `0x10044640` in `bin/l2`. It holds 40 port records of 0x1c bytes from offset `0x15c`; a record's first word is its state (0 free, 1 attached, 2 configured), the second the connection type (2 for USB), and offsets `+0x14` and `+0x18` the brick count and a pointer to the 0x30-byte brick records. Established from `L2CfgMAddPort` (libmain `0x13a8c`), `L2CfgMCfgPort` (`0x15a54`) and the readers in `bin/l2`; layout also in [l2/config-request-reply.md](l2/config-request-reply.md) section 1.

**Attach.** `l1_open_usb_device` calls `L2CfgMAddPort` at `0x10004b04`. A return of -1 prints `ERROR: device is already in use.`; otherwise the same routine arms `EventAddTimeOut(0, 750000, 0x1000d9a8, slot)` at `0x10004b34` to `0x10004b54` (the 750,000 is `lis 0xb; ori 0x71b0`, name string `l1_open_usb_device`) and registers the input handler `0x100069b0` for the descriptor. Nothing is written to the device at open. Established. The port stays at state 1 and has no brick record until a reply is parsed (section 3).

**The request (callback `0x1000d9a8`, read in full).** The callback builds one packet with `IRouterMakePacket(0x10043ce8, &dest, 0, 0, 0, 2, 3, descriptors)`:

| Field | Value | Grade |
|---|---|---|
| destination word | `0x0ffff00e` (`lis 0xfff; ori 0xf00e`, `0x1000d9c8`): form 0, module field `0xffff`, task 14 | E |
| class, code | class 0 (request), code 0 (fifth and fourth arguments) | E |
| source word | the first word of the local record at `0x10043ce8` (the local L2 router address; `0x84000001` in the stock run of the earlier note) | E for where it comes from; the stock value is that note's observation |
| descriptor 0 | integer 6 (type 0) | E |
| descriptor 1 | `nvramGet(3)` as a string (type 9: string, free after copy), length `strlen + 1`, when non-empty; else integer 0 | E |
| descriptor 2 | integer 0 | E |

The sixth argument is the constant 2 in every call read; it enters only the "local packet" size test in `IRouterMakePacket` (it must exceed 1) and is not a wire field. The packet is written through `sub_10007478(fd, packet)`, which dispatches on the connection mode. Before building, the callback returns without sending and without re-arming when the slot's descriptor is -1 (`0x1000da94` to `0x1000daa4`, jump to the epilogue at `0x1000dcf8`), and it builds nothing while the word at `0x10043c00` is zero (section 5). Established.

## 3. The reply becomes a route

**Where the reply goes.** The input handler reads one packet, stores its slot number at packet `+0x0c`, and calls the router entry (section 4). A reply addressed to task 4 with class 1 reaches `sub_1000e664` (`0x1000e664` to `0x1000e9e4`), which acts only when the status byte at packet `+0x13` is 0. For a source word with form 0 it calls `L2CfgMCfgPort(cfg, slot, packet)`. When the brick table changed it cancels the slot's timer, re-arms one callback 1 s later (debug name `extra cfg_ping_l1`, same callback `0x1000d9a8`), and calls `sub_1000f93c(1)`. After a successful configuration it also logs `INFO: USB device %s configured: %s` (string `0x100292ac`, used at `0x1000e840`; the second argument is built from the first brick record's rack and slot, `0x1000e7c4` to `0x1000e7f8`; the conditions on the log line were not followed). Established for the call.

**What `L2CfgMCfgPort` accepts (libmain `0x15a54`).** It reads descriptor 0 and compares it with 7 and 6 only. With 7: descriptor 2 is the record count `N`, descriptor 3 the record size `S`, descriptor 4 a byte array of `N` table indices, descriptor 5 a block of `N` records of `S` bytes; each record goes through the builder `sub_156f4`. With 6: four records at fixed indices 0, 1, 5 and 6. Any other first descriptor changes only the port state. The builder copies at most 0x70 source bytes into a zeroed local, builds one 0x30-byte brick record, grows the port's array with `realloc`, and at the end compares the old and new arrays; a difference is the "table changed" result used above. Established.

**Brick record (0x30 bytes) and the source word each byte comes from.** Read from the stores at libmain `0x157b0` to `0x15828` (each source word is a big-endian 32-bit value of which only the low byte is kept; the stores are the same when the value exceeds 0xff and goes through `ntohl` first):

| Brick `+` | Source word `+` | Used by the daemon for | Grade |
|---|---|---|---|
| `0x00` | table index byte (descriptor 4) | index of the record in the L1's table; the route search keeps the smallest | E |
| `0x01` | `0x00` | valid flag; every search skips a record with 0 here | E |
| `0x03` | `0x08` | rack (route key; also the destination builder) | E |
| `0x04` | `0x0c` | slot, called "bay" in the L1 notes (route key; low six bits used) | E |
| `0x05` | `0x10` | brick type character; selects the brick class for the sender and whether a brick counts as a router or a compute brick | E |
| `0x06` | `0x3c` | **partition number** of the brick; matched against the `p <id>` target and compared across bricks by the span check | E for the use; the L1 writer is U (U1) |
| `0x10` | `0x34` | **power state** of the brick: 0 off, nonzero on; read by `power down`, the status summary, and the skip flags of the sender | E |
| `0x11` | `0x38` | margin state (0 to 5) | E |
| `0x12` | `0x60` | a value of 0 is replaced by 2 when `+0x01` is nonzero | E (the store); role U |

The remaining bytes (`+0x02`, `+0x08`, `+0x0c` to `+0x0f`, `+0x14`, and the two 12-byte strings at `+0x18` and `+0x24`) are not read by anything in sections 4 to 7; they feed the `config` display. Established that the other routines here do not read them; the display routine itself was not read.

**What the L1 sends.** The L1 half is the earlier note's reading of Origin 300 L1 1.44.0 package 1: `config_exchange_request` (`0xffe325ec`) answers selector 6 with selector 7, count `N`, size 0x70, a byte array of indices and a data block of `N` records of 0x70 bytes, status 0, destination equal to the request's source, source `(identity << 12) | 0x0e`. Not re-read here; grade as in [l2/config-request-reply.md](l2/config-request-reply.md) section 3 (established for package 1). Which L1 table fields hold the partition (`+0x3c`) and the power state (`+0x34`) was not found (U1).

## 4. Routing a packet by its destination

The router entry `sub_10022e50` hands class 0 and class 1 packets to `sub_10021158` (`0x10021158` to `0x10021ea4`). The destination word is packet `+0x18`; the source word packet `+0x1c`. Every branch that hands the packet on first adds one to the 15-bit count in packet `+0x04` (mask `0x7fff0000`, the reference count of [l2/router-packet.md](l2/router-packet.md)), because each receiver frees it. Established for all rows.

| Destination | What the L2 does | Result when nothing matches |
|---|---|---|
| form 15 | looks the word up in the node list as a node id, then looks the low 24 bits up as an address key (byte-swapped unless the word at `0x10043cd8` is 1); the local node accepts class 1 only; a remote node gets the packet through its connection | `-0x260` (`NO RECEIVER`) |
| form 14 (the L2 form) | the same two lookups by the low 24 bits; a remote node gets the packet; a local match, or a miss that `L2CfgMFindNodeLocal` still finds, continues to the task dispatch below | the node at `cfg + 0x18` if set, else `-0x260` |
| any form but 14 and 15, module field not `0xffff` | **brick route**: rack = bits 18 to 27, slot = bits 12 to 17; `L2CfgMFindBrickIP(cfg, rack, slot)` over every node, every port in state 2, every brick record with the valid byte set and matching rack and slot byte (smallest index wins); a local match is written to that port's descriptor by `sub_10007478(fd, packet)` (fd at `0x100512f0 + 0x48 * slot`); a remote match is forwarded to its node (`NR remote!` and `-0x260` if that node is gone); a local match for a packet whose destination form is 1 prints `NR R.S+IR_L2!` and fails | `-0x260`, unless the local node's word `+0x10` is 1 and `+0x14` names a default node, which then takes the packet |
| form 0, 3, 4 or 5, module field `0xffff` | **attached-controller relay**: if more than one port is in use (`cfg + 0x158 > 1`) the result is `-0x260`; otherwise the packet is written to the first slot, in slot order, whose descriptor is not -1 | `-0x260` |
| form 1, module field `0xffff` | **task dispatch** on the low five bits, table at `0x1002e4e0` | see next table |
| form 14, any module field | the same task dispatch, task in bits 29 to 24 | see next table |
| forms 2 and 6 to 13, module field `0xffff` | rejected | `-0x260` |

Task dispatch (jump table `0x1002e4e0`, index = task minus 2; tasks outside 2 to 13 take the default row). "Request" means class 0:

| Task | Handler | Role (E for the dispatch; role as marked) |
|---|---|---|
| 3 | request `sub_1001af4c`, reply `sub_1001bbc4` | text command executor and its reply collector (section 6) |
| 4 | request `sub_1000dfb8`, reply `sub_1000e664` | configuration service; the reply side is section 3 |
| 2, 7 | non-request only: `sub_1000bed4` | event input (role I, from the handler name strings of neighbouring routines) |
| 5 | non-request only: `sub_1001dbc4` | role U |
| 9 | non-request only: `sub_10020f08` | role U |
| 12 | non-request only: `sub_1001bc4c` | role U |
| 13 | non-request only: `sub_10024714` | role U |
| 6, 8, 10, 11 and any other | **local client lookup**: `L2CfgMFindNode(cfg, 0, 0x1fff0000 \| 0xf000 \| task)`; if a client holds that identity the packet goes to it with `IRouterPacketToRouter`, else `-0x260` | task 10 is `l2part`, section 7 |

Before the dispatch, a packet whose task is 13 or 23 is first passed to `sub_10024b58` (`0x100213a0` to `0x100213bc`), and a packet whose *source* is task 5, with the flag `cfg + 0x10 == 1`, a destination of form 1 and module field `0xffff`, and `cfg + 0x14` set, is redirected to that default node (`0x10021538` to `0x10021664`). Both are established as code; neither role was followed (U6).

## 5. After registration: the recurring request and what the L2 does with the answers

**The request repeats unchanged.** The callback `0x1000d9a8` rebuilds the same three-descriptor request on every firing; nothing in it depends on the previous reply. It re-arms itself at `0x1000dc5c` to `0x1000dcf8` (name `cfg_ping_l1 (%s)`) whenever the slot's descriptor is positive. The delay is the word at `0x10043d54` (`L2<->L1 freq`, NVRAM key 15, default 5; the `ping` command's sub-command "sets L2 to L1 ping frequency in seconds" is the record at `0x10040984`), **unless the port record's state word is not 2, in which case it is 2 s**. So the L2 asks every 2 s until the first reply has been parsed, then every `key 15` seconds. A table-changing reply additionally cancels the pending timer and fires once after 1 s. Established; the stock-run timings (0.0, 2.0, 7.0, 12.0 s) in the earlier note agree.

**Gating.** The word at `0x10043c00` (initial 1) is the auto-scan flag. The two timer set-ups `sub_1000f710` (periodic `cfg_setup_config_check`, callback `0x1000f93c`) and `sub_1000f7bc` (`cfg_setup_l2ping`, callback `0x10010454`) each set it to 1 when given a nonzero period and to 0 when given zero (stores at `0x1000f784`, `0x1000f7a0`, `0x1000f830`, `0x1000f84c`); they are called at start-up and from the `config` and `ping` commands. A manual `config` scan with the flag set prints `WARNING: config auto-scan is enabled; skipping manua...` (`0x10011124` to `0x1001114c`). While the flag is zero the request callback builds and sends nothing, and still re-arms. Established.

**Nothing times a port out.** The callback never counts unanswered requests, and no routine found clears a USB port's state because replies stopped; the port leaves service only when the device is closed (`sub_10004bf8` calls `L2CfgMCfgPort(cfg, slot, 0)`, which frees the port, `0x10004d4c` to `0x10004d60`), which the repeated read and write error counters trigger ([l2/daemon.md](l2/daemon.md)). The record of a brick therefore keeps its last reported power, margin and partition bytes until the next reply. Established for the callback and the close path; inferred for the absence of any other clearing path (the callers of `L2CfgMDeletePort` and `L2CfgMDeleteNode` in `bin/l2` are the network close and the L2-to-L2 ping, `0x10007e54`, `0x100105a4`, `0x10010628`).

**What the exchange feeds.** Every later decision in sections 6 and 7 reads brick records filled by the reply, never anything the L2 asks a second time: route keys (rack, slot), type, partition, power and margin bytes. The `power` command's own state reads are the cached `+0x10` and `+0x11` bytes ([l2/power-queue.md](l2/power-queue.md): sub-command 3 counts them), so the L2's idea of "which bricks are on" is at most one request period old. Established for the readers; inferred for "at most one period" (it assumes each reply carries a fresh `+0x34`, U1).

**Configuration event to peers.** `sub_1000f93c` is the timer callback armed by `sub_1000f710` under the name `cfg_setup_config_check` (`0x1000f76c`); the reply handler also calls it directly with 1 when a reply changed the table. It takes two `L2CfgMQuery` strings (query 0 and query 2), compares them with the copies it kept (`strcmp` at `0x1000fa9c` and `0x1000fae4`), and builds a packet with `IRouterMakePacket(0x10043ce8, ...)` that it sends with `sub_10022e50` (failure text `ERROR: can't create packet for configuration event.`). Established for the call chain; that the packet goes to peer L2s, and the exact condition for sending, are not followed (U6).

## 6. The routed shutdown: `p <id> pwr d`

The sender of this line is not in these images. The repository's L1 notes ([l1/power-reset-cbrick.md](l1/power-reset-cbrick.md) section 1) record that IRIX sends it as a text request to `0x1ffff003` after asking for the partition id; this is a hint, not read here. The L2's handling is read below.

**Receive (E).** `0x1ffff003` has form 1, module field `0xffff`, task 3: the task dispatch of section 4 sends a class-0 packet to `sub_1001af4c` (`0x1001af4c` to `0x1001b484`).

**Command text (E).** The routine takes the text from the **second** descriptor: it tests the type bits of the descriptor at packet `+0x2c`; type 0 means the word at `+0x30` is used, otherwise `IRArg(packet, 1)` supplies the pointer. The first descriptor is not read. The L2's own sender puts an integer 0 first and the text second, so does the user-space text client in `l1cmd`/`l2cmd` ([host-tool-options.md](host-tool-options.md)). A request with one descriptor gives a null text, and the first instruction of the parser, `lbz` of the first text byte at `0x10010ad4`, would fault on it (I: from `IRArg` returning 0 for an index beyond the count). What descriptors IRIX actually sends is U2.

**Bookkeeping (E).** The routine stores the requester's source word at `0x10043d5c`, its sequence number (packet bytes `+0x14` and `+0x21`) at `0x10043d60`, packet byte `+0x12` at `0x10043d64`, and the five-bit field of byte `+0x17` at `0x10043d68`; clears the output buffers; clears the destination context with `sub_1001e458` (all racks and slots selected, no partition selected); and runs the text through `sub_10010aac`, the L2's command-line executor (`CmdProcess` over the table at `0x1003ec08`, 188 records of 0x44 bytes, with the context at `0x10043c58`). When the context is already busy (`context + 0x1c` nonzero, `0x1001afb4` to `0x1001afc4`) the routine instead runs the line with a null context and answers at once, with `ERROR: command library is busy` if that fails (I: read from the structure).

**Target and command parse (E).**

- `p <rng>` or `partition <rng>`: `CmdProcess` (libsys `0xb200`, the compares at `0xbf10` to `0xbfe4`) parses the range into the 64-byte **partition mask** at `target + 0x1b4` with the shared range parser `sub_ccd0` (libsys `0xccd0`: `*` or `all` fills every byte with 5, `**` with 6, a number sets one byte to 1, `a-b` marks a run; a number of 64 or more is a parse error), and **clears the rack mask (0x100 bytes) and the slot mask (0x40 bytes)**. `rack`/`r` and `slot`/`s` fill the other two masks the same way and clear the partition mask. So a partition of 1 to 63 is addressable by this prefix; the console command `select partition` is a separate parser whose error text names partitions 1 through 255.
- `pwr d`: the command record `power|pwr` with sub-word `down|d` (record `0x10040cf8`, sub-command 2) calls the `power` handler `0x10013ba8`; the handler first builds the brick table (`sub_1001e554`) from the target, fails with `ERROR: no L1s found to match that destination` when it is empty, and jumps through `0x1002c350` to the power-down code at `0x10014390`.

**Brick selection, `sub_1001e554(destination, target)` (E, read in full).** Walks the node list from the local node, the 40 ports of each node (only state 2), and each port's brick records **from the last to the first**. A record is skipped when its valid byte is 0, its rack is 0 or above 0xff or its slot is 0 or above 0x3f. If the target's partition mask has any nonzero byte the **partition mode** (destination word `+0x180` = 2) applies; otherwise rack/slot mode (`+0x180` = 1). In partition mode a record is taken when mask byte 0 is 5 (`p *`: any brick, routers included), or else when its type character is not `R`, `D` or `-` (**routers are never taken by a partition number**) and the mask byte at index `record[6]` is nonzero. In rack/slot mode both mask bytes (rack index, slot index) must be nonzero. A brick address already in the list is not added twice. A record whose type is one of `C 3 5 4 A 6 9 7 8 0 Q @ O B F L M $ , } . {` is appended; every other type is inserted at the front. The list therefore holds the non-compute bricks (reverse discovery order) followed by the compute bricks (discovery order). The resulting word for each entry is `rack << 18 | slot << 12`, stored in `destination + 0x184 + 4 * i`, count at `+0x1184`.

**Power down (E; `0x10014390` to `0x100149e4`).**

1. **Span check, `sub_1001f15c`.** Only when the target has rack mask byte 0 in {5, 6} and slot mask byte 0 in {5, 6} (the default destination, or `* *`), or has no mask at all, does it build the table and compare the partition byte of every brick: a difference fails the command with `current destination spans partitions, explicit target required.` (or `default destination ...`). A `p <id>` target has empty rack and slot masks, so the check returns 0 at once.
2. **Router step, only in partition mode.** Over every present non-router brick of every node (not just the destination), count those **outside** the destination list (match on rack and slot) and, of those, the ones whose power byte `+0x10` is 0. When the two counts are equal, that is, when every non-router brick outside the partition is already off (also when there are none), the L2 queues one `power down` text request, with the wait flag set and no delay, to **each present router that is on**. Otherwise routers are left alone. The partition's own shutdown therefore takes the routers down only when it leaves nothing else running.
3. **Broadcast.** In every case a `power down` text is queued for the destination's bricks through the sender `sub_1001c90c` (mode 1 parallel, wait flag set, no delay, class mask `0xff`, broadcast flag set), which starts the queue. Classes are served in the order routers (1), I/O (2), compute (4), other (`0x80`); in partition mode the router class of the list is empty, so only step 2 reaches routers.

**What goes on the wire to each L1 (E).** One router packet per brick: destination `rack << 18 | slot << 12 | 3` (the sender clears the low five bits of the list entry and sets task 3; the router step in 2 builds the same word with `ori 3`), class 0, code 0, two descriptors: integer 0, then the string `power down` (type `0x0a`: string, free after copy; length `strlen + 1`). Source word the L2's local router address. The L2 writes the text `power down`, not `pwr d`. The L1 side is [l1/power-reset-cbrick.md](l1/power-reset-cbrick.md): the handler acts on the L1's own brick, and a brick of type `9`, `A` or `Q` with an event subchannel registered defers its first `power down` and wants it repeated; the L2 sends each text once and has no repeat. `C`, `I` and the Origin 300 `4` do not defer.

**The wait flag and the reply timer (E for the queue, I for the requester's reply).** Every queued packet has the wait flag, so the runner records an outstanding reply for each and arms the 15 s reply timer (word `0x10043c70`, [l2/power-queue.md](l2/power-queue.md)). The L1's text reply is class 1 on task 3, so it is routed to `sub_1001bbc4`, which hands it to the collector `sub_1001b920`; the collector appends each L1's output to a growing buffer, frees the packet, and lets the runner continue. After parsing the original line, `sub_1001af4c` tests the outstanding-reply count at `context + 0x1c`; when it is zero the answer is built at once, otherwise the answer is sent when the last reply (or the timer) completes the run. That deferral is inferred from the structure of the three routines (the final send at `0x1001b878` to `0x1001b8e8` reads the saved source word, sequence and task bits that `0x1001b200` to `0x1001b280` stored); no run was made.

**The answer to the requester (E for its shape).** `IRouterMakePacket(local router record, &saved source, saved task bits, code, class 1, 2, count, descriptors)`: destination the requester's saved source word, class 1, sequence restored into packet bytes `+0x14` and `+0x21`. Descriptor 0 is an integer (0 when the line was accepted, -1 when `sub_10010aac` returned nonzero); the text follows as one string descriptor (type `0x0a`, length `strlen + 1`) per chunk, cut at line ends into pieces under 64 KiB (`0x1001b680` to `0x1001b83c`), so an empty output leaves a one-descriptor answer. The text of a refused command is the error line of the failing step above.

## 7. Task `0xa` and the partition-record reply

**What asks (partly E).** The boot firmware of the stock IP35 PROM sends a request to `0x1ffff00a`. Read from the payload: at file offsets `0x57450` to `0x574d0` (and the identical copy at `0x5e91c` to `0x5e9a0`) it initialises a request context with task `0xa` (`addiu a1, zero, 0xa`, `0x57450`), builds the destination `lui 0x1fff; ori 0xf00a` (`0x5748c`, `0x57494`), and calls the request builder `0x53ae0` with **request code 1** (`addiu a2, zero, 1`, `0x5747c`) and **one descriptor** (`addiu t0, zero, 1` at `0x57470`) of type 0 (`sb zero, 0xa0(sp)` at `0x57478`; the value `ld 0xf0(sp)` stored at `0xa8(sp)`). The builder writes the code halfword at block offset `0x32` (`sh s2` at `0x53be8`) and the descriptors from block offset `0x34` (loop `0x53c44` to `0x53d34`), which is wire offsets `0x12` and `0x14` of the segment layout in [l2/router-packet.md](l2/router-packet.md) once the block's 0x20-byte lead is removed. After the reply the firmware requires block byte `0x23` (wire status byte) to be 0 and block word `0x40` (the value of the second descriptor) to differ from -1 (`lbu 0x23`, `lw 0x40`, `beq t5, -1` at `0x574e4` to `0x574f8`). Established for the payload. **This settles the code the earlier note left open**: the fetch is task-10 request code 1 with one integer; no other code takes one integer. What the integer is, is section 10, U4.

**Dispatch (E).** The destination `0x1ffff00a` has form 1, module field `0xffff`, task 10, so the dispatch of section 4 takes the local-client row: `L2CfgMFindNode(cfg, 0, 0x1ffff00a, 0)`, and the packet is delivered to the process that registered with `sn1SCInitL2App(host, 0, 10)`, which is `l2part` (`li r5, 0xa` at `0x10000e8c`). `l2part` runs only at inittab run level 5 and starts after a 5 s sleep ([l2/partition-task.md](l2/partition-task.md)); when no client holds task 10 the router answers `-0x260` and no reply reaches the sender. Established.

**Handler for code 1 (E; `l2part` `0x10001384`, read in full).** The brick is the halfword at packet `+0x2a` when the first descriptor has type 0 (the low half of the big-endian value word at `+0x28`), else `IRArg(packet, 0)`; it is sign-extended from 16 bits (`extsh` at `0x100013bc`) and passed to the wrapper at `0x10002054`, which calls `get_partid_for_brick` and **replaces a negative result with 0** (`cmpwi/bge/li` at `0x10002090` to `0x1000209c`). The reply record: result descriptor type 0 value 0 (status zero), integer descriptor type 0 with the partition number in the value word, count 2 (`stw 2, 0x58(r31)` at `0x1000142c`).

**The reply on the wire (E for fields from the builder; the source word I).** One 36-byte segment: length `0x0024`, byte 2 = 2, status 0, the request's sequence copied, first-segment flag `0x81` (one segment), class 1, destination the request's source word, source the L2's local address, descriptor count 2, code `0x0001`, result descriptor (value 0), integer descriptor (value = partition number, 0 to 63, big-endian 32-bit at wire offset `0x20`). Byte-by-byte table and the five test vectors are in [l2/partition-record.md](l2/partition-record.md); re-checked here for the fields above.

| Request | Reply value | Grade |
|---|---|---|
| mode has no partitions (layout "has partitions" byte 0) | 0 | E (partition-record.md, `partid_for_brick`) |
| brick (or its rack when the layout flag is set) listed | the partition byte | E |
| brick not listed | 0, not -1 | E (clamp above) |
| request code outside 1 to 8 | result -3, one descriptor, 28 bytes, log line `l2part task received unknown request %d` | E |

**It is the attached-L1 table that routes the reply (E).** The reply's destination is the source word the L1 stamped on the forwarded request. It goes through the brick route of section 4, so it reaches the sender only if the L1's own rack and slot are in the table the L1 reported (section 3) with the valid byte set. A reply for a source whose rack and slot are not in any port's table gives `NO RECEIVER`. The earlier note's failure with a destination of `0x00044215` is this case.

## 8. Coverage and what this note did not read

Read in full for this note: `sub_10021158`, `sub_1000e664`, the request callback `0x1000d9a8`, `sub_1001af4c`, `sub_1001e554`, `sub_1001e458`, `sub_1001f15c`, the power-down arm of `0x10013ba8`, `sub_10010aac`, `sub_ccd0` and the target arm of `CmdProcess`, `l2part`'s code-1 handler and its wrapper, and the PROM request path above. Read only in part: `sub_1001b920`, `sub_1001bbc4` and `sub_1001fba8` (the reply collector and the reply matcher), `sub_1000f93c`, the version-check command `0x1001a120`. Not read: the `config` display, the L2-to-L2 paths, `sub_10024b58`, the handlers for tasks 5, 9, 12 and 13, the boot loader and the kernel's USB driver (see [open-questions.md](open-questions.md)).

## 9. Corrections to earlier notes

1. **Destination mode 2 is a partition destination, not a rack destination.** [l2/power-queue.md](l2/power-queue.md) reads the word at `destination + 0x180` as 2 when "the destination names whole racks". The word is set by `sub_1001e554` (`0x1001e648` to `0x1001e664`) from the test of the 64-byte mask copied from `target + 0x1b4`, which the `p`/`partition` prefix and `select partition` fill; the rack mask is at `+0x40`/`+0x74`. The slow and fast power-up branches and the power-down router rule that test the word are therefore partition-mode branches. The router counts in power down are of **non-router bricks outside the destination**, not of routers: the loop skips types `R`, `D`, `-` (`cmpwi 0x52/0x44/0x2d` at `0x100144c4` to `0x10014528`), so the rule powers routers down only when everything else is already off.
2. **`ver check` is the L2's own command line, not a request.** The call at `0x10013d04` and the one at `0x10014f5c` pass the text `ver check` to `sub_10010aac`, the command-line executor, with the destination saved before and restored after (`memcpy` of 0x1188 bytes at `0x10013ce4` and `0x10013d14`). The command (`version|ver` records, handler `0x1001a120`) sends the text `ver` to the L1s and L2s of the destination and prints mismatches. Power up and reset run it; **power down does not** (the code at `0x10014390` starts with the span check, and the two string uses above are the only ones). [l2/power-queue.md](l2/power-queue.md) says power down shares the precursor.
3. **Order of the brick table inside a class** (the open question in [open-questions.md](open-questions.md)): answered by the walk and insertion rule in section 6: records are visited last to first, compute types are appended, every other type is inserted at the front.

## 10. Unknowns and the software test for each

| ID | Unknown | Test that would settle it |
|---|---|---|
| U1 | Which L1 table fields carry the partition (record `+0x3c`) and the power state (`+0x34`), and whether each reply refreshes them. The writer of the table behind RAM word `0x30000aa4` was not found. | Static: in Origin 300 L1 1.44.0 package 1 (and the other packages) find every store through the table pointer with displacement `0x3c` and `0x34`, and the readers of setting `0x0e` (the partition byte at settings-record `+0x1f`, [l1/settings-record.md](l1/settings-record.md)). The claim holds if the setting's value is copied into `+0x3c` on each exchange. |
| U2 | How many descriptors IRIX puts in the `p <id> pwr d` request (the L2 reads the second). | Static: find the controller text-send routine in the IRIX 6.5.30 kernel that sends to `0x1ffff003` and read its descriptor count and types; or run the shipped `l2` under a user-mode PowerPC interpreter ([l2/user-mode.md](l2/user-mode.md)) with a scripted peer sending a one-descriptor and a two-descriptor request and compare. |
| U3 | When the requester's answer is sent relative to the L1 replies (immediately or after the last reply or the 15 s timer). | Run `l2` as in U2 with two scripted L1 peers; send `p 1 pwr d`; delay one L1's reply and record when the answer packet is written. Or read `sub_1001b920`, `sub_1001ba44`, `sub_1001fba8` fully. |
| U4 | What integer the PROM passes in its code-1 request, and whether it uses the 16-bit brick identifier of [l2/libpart.md](l2/libpart.md) (type index 15 to 12, rack 11 to 6, slot 5 to 0). The value comes from `0x53df0`, which asks an L1 (`0x4e6cc`, request value 3) and converts the answer with `0x4e2c8`; that converter tests `slot < 0x40` (`sltiu 0x40` at `0x4e2cc`), which fits but does not prove the layout. | Read `0x4e6cc` and `0x4e2c8` in `ip35prom.payload.bin` in full and compute the converter's output for the L1 reply fields; compare with the identifier layout. |
| U5 | The consumer of the L1's own requests with selectors 2 and 13 sent to `0x0ffff00e` before its first reply (the L1 note), and what the L2 does with them: by section 4 a form-0 `0xffff` packet from a port is written back to the only open port, or fails with `-0x260` when two or more ports are in use. | Run `l2` as in U2, send such a request from a scripted L1 and record where it goes; statically, read the L1 handler registered for `0x0ffff00e` for selectors 2 and 13. |
| U6 | The role of tasks 5, 9, 12, 13, of `sub_10024b58`, of the source-task-5 redirect, and of the condition for the configuration event. | Read the five handlers and `sub_1000f93c` in full; name each by the packets it builds. |
| U7 | Whether a partition number of 64 or more can reach the L2 through `p <id>` (the parser rejects it) and what IRIX sends for such a partition. | Read the IRIX 6.5.30 kernel's partition-id source and its maximum; then run the parser (U2) with `p 64 pwr d`. |
| U8 | The `config` display text for a configured USB port and the meaning of brick record bytes `+0x02`, `+0x08`, `+0x0c` to `+0x0f`, `+0x12`, `+0x14` and the two strings. | Read `L2CfgMPrint` and its variants in libmain; match each store of section 3 to a printed column. |
| U9 | Whether anything in the L2 or an L1 ever marks a brick absent when its L1 stops answering (section 5 found no path). | Search the L2 for every writer of the port state word and of brick byte `+0x01`; run `l2` with a scripted L1 that stops replying and watch the `config` output. |
