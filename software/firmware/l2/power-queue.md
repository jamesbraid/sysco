# L2 `power` command: the send queue, its order and its spacing

Read from the L2 daemon `bin/l2` of L2 1.44.0 (IRIX 6.5.30 `l2.bin`, carved with [../../retools/tools/sysco/l2_extract.py](../../../retools/tools/sysco/l2_extract.py), listed with [../../retools/tools/sysco/l2_list.py](../../../retools/tools/sysco/l2_list.py)). Addresses are the daemon's load addresses. Grades follow [AGENTS.md](../../../AGENTS.md). 1.44.0 is the reference. 1.48.0 was compared for the brick table builder ([below](#the-brick-table-order-the-reply-timer-and-ver-check)).

The L2 does not power bricks itself. Every `power`, `reset` and status request is a short text command (`power up`, `power up hold`, `power down`, `power check`, `reset`) sent in a router packet to the L1 of each selected brick, and the order and spacing live in the L2. The L2's own `power` command is the entry for `0x10013ba8`; its eight sub-commands are a jump table at `0x1002c350`.

## Sub-commands

| Index | Jump target | Role | Grade |
|---:|---|---|---|
| 0 | `0x10013c90` | per-brick status: send `power check` to every selected L1, all classes, then run the queue | E |
| 1 | `0x10013ce4` | power up (below) | E |
| 2 | `0x10014390` | power down (below) | E |
| 3 | `0x100149e8` | summary: counts the bricks whose cached power state (byte `0x10` of the brick record) is on, off or other, and the cached margin state (byte `0x11`, values 0 to 5) as default, low, normal, high, custom or other; prints `all %d bricks appear powered on`, `... powered off`, `power: %d on, %d off, %d other` and the margin equivalents | E |
| 4 | `0x10014d8c` | print the delay: `delay is set to %d milliseconds` | E |
| 5 | `0x10014dac` | set the delay to the parsed argument | E |
| 6 | `0x10014dc0` | print the mode: `power up mode is %s (power up routers in %s)` with `slow`/`series` when NVRAM key 21 is nonzero and `fast`/`parallel` when it is zero | E |
| 7 | `0x10014e2c` | set the mode: first letter `s` stores 1 in NVRAM key 21, anything else stores 0 | E |

The delay is held in the word at `0x10043c4c`, initial value 500 (read from the data segment; the `power` help says milliseconds). The set command stores it in that word only; there is no NVRAM write on that path, so it reverts to 500 when the daemon restarts (E). The mode persists in NVRAM key 21 (E). Key 21 is read for every power-up, so a mode change takes effect on the next `power up` (E).

## The queue

`sub_1001f5e0` appends a 20-byte node to a first-in first-out list and `sub_1001f6e4` removes the head and returns three values: a packet, a wait flag and a delay word (E, read in full at `0x1001f5e0` to `0x1001f7cc`). The runner is `sub_1001c3ac` (`0x1001c3ac` to `0x1001c658`, E). For each node it does, in this order:

1. If the packet is non-null, send it (`sub_10022e50`); if the wait flag is nonzero, record an outstanding reply for that L1 (`sub_1001f8b8`), and arm the reply timer (`sub_10020090`, below). It then goes on to the delay word of the same node.
2. If the delay word is nonzero: `-1` means wait for replies. When any reply is still outstanding (word `0x1c` of the context is nonzero) the runner returns and is called again by the reply handler once the outstanding set empties; when none is outstanding it continues at once (E, `0x1001c550` to `0x1001c5b8`). A positive value is a blocking `usleep(value * 1000)`, so the delay unit is milliseconds (E for the multiply by 1000 at `0x1001c5ec`; the daemon is single threaded, so the sleep also stalls every other service for that time, I).
3. Next node.

The reply timer is `EventAddTimeOut` with 15 in the seconds argument and 0 in the microseconds argument, from the word at `0x10043c70` (initial value 15, E). On expiry, if a timeout handler is set the handler runs; otherwise the queue is cleared (`0x1001ffe4` to `0x1002008c`; the clearing routine at `0x1001f5a0` was not read, I). The 15 can be changed at run time: the word is the broadcast response timeout setting, see [below](#the-brick-table-order-the-reply-timer-and-ver-check).

## The sender `sub_1001c90c`

All sends go through one routine with ten arguments, read in full (`0x1001c90c` to `0x1001d298`, E):

| Argument | Meaning |
|---|---|
| r4 | mode: 1 = parallel, 2 = serial |
| r5 | command text |
| r6 | nonzero: queue each packet with the wait flag set |
| r7 | delay word to queue after the batch (parallel) or after each packet (serial) |
| r8 | class mask in the low 16 bits, `0xff` meaning every class; flag bits in the high half: `0x10000` skip bricks whose power byte is nonzero (only act on bricks that are off), `0x20000` skip bricks whose power byte is zero |
| r9 | list of slot numbers, terminated by -1, or null for no list |
| r10 | nonzero: broadcast, start the queue now and queue no trailing delay |

**Classes.** The brick type character of each brick record selects one class bit, by the jump table at `0x1002dd00` (list case) or `0x1002de70` (no-list case); the two tables hold the same mapping (E, decoded in full):

| Class bit | Brick type characters | What the class is |
|---:|---|---|
| 1 | `-` `D` `R` | routers (R-brick, the dense router `-`, NL4 router `D`) |
| 2 | `#` `&` `=` `E` `G` `I` `N` `P` `V` `X` `Y` | I/O and graphics bricks: PX `#`, N-power `&`, IX `=`, 2UPX `E`, G, I, N, P, V, X, XY |
| 4 | `$` `,` `.` `0` `3` to `9` `@` `A` `B` `C` `F` `L` `M` `O` `Q` `{` `}` | compute bricks (C and its variants, Chimera, Twister, Opus, Gemini, Dorado and so on) |
| `0x80` | every other character (for example `%` the PE brick, `^`, `?`) | other |

The loop is outer over class bits from the low bit of the mask to the high bit (doubling each time), then over the slot list, then over the L2's brick table (`0x184 + 4*i` in the destination context, `0x1184` entries). A brick is sent to when its slot (bits 12 to 17 of its address word) equals the list entry, or for every brick when there is no list, and its class bit is in the mask (E). Within a class the order is therefore the order of the destination's brick table, which [the builder](#the-brick-table-order-the-reply-timer-and-ver-check) fills in an order that depends on the class.

**Trailing steps.** After the loops:

- Parallel (mode 1): when any packet was sent and r6 is nonzero, queue one wait-for-replies node (`-1`), then, only when r10 is zero, one delay node with the r7 value (E, `0x1001d1b0` to `0x1001d210`). So a parallel batch costs one wait for all replies, then r7 milliseconds.
- Serial (mode 2): after every packet, queue a wait-for-replies node and a delay node with the r7 value (E, `0x1001cd1c` to `0x1001cd78` and `0x1001d128` to `0x1001d160`). So each brick is sent, its reply awaited, and r7 milliseconds slept before the next brick.
- r10 nonzero: log `starting broadcast` and start the queue (E).

## Power up

Read `0x10013ce4` to `0x10014390`. A `ver check` is run first (`0x10013d04`; the call is `sub_10010aac`, which runs the L2's own `ver check` console command and whose result is ignored, see [below](#the-brick-table-order-the-reply-timer-and-ver-check)). The destination is then classified by the word at offset `0x180` of the destination context: 2 when the destination names whole racks, 1 otherwise (E for the store at `0x1001e664`; the test is whether the rack mask copy is non-empty, I). The mode is NVRAM key 21.

### Slow (key 21 nonzero) and the destination names racks

For every rack in the L2's configuration, in table order (40 rack slots, stride 0x1c, linked by a next pointer at `+0x124`; this is the global table, not only the destination), and for every brick record of the rack in index order (stride 0x30), the router bricks (type character `R`, `D` or `-`) that are present (byte 1 nonzero) and currently off (byte `0x10` zero) are each sent `power up` (no hold) as its own packet addressed to that router's L1. Each packet is followed by a wait-for-replies node and a delay node with the delay word (500 ms by default). The routers therefore come up one at a time, separated by the reply wait plus the delay (E, `0x10013d3c` to `0x10013fb8`; the address is built from bytes 3 and 4 of the brick record as rack and slot, with the low two bits `3`, I).

### Slow and any other destination

One serial batch of `power up hold` to class 1 with delay 0 (E, `0x10013fcc` to `0x10013ff0`): each router is sent in turn with a reply wait but no sleep.

### Fast (key 21 zero)

The command text is `power up hold`; when the destination names racks the text becomes `power up`, the global brick table is temporarily widened to every known L1 (`sub_1001e458` clears the destination context to all-selected and `sub_1001e554` rebuilds the brick table), and the original destination is restored afterwards (`0x100141d0`) (E for the calls; the reason for widening is not stated in the code, so the claim that routers of all racks come up first is I from the calls). Two parallel batches follow, each to class 1 with flag `0x10000` set when the destination named racks (so only routers that are off are sent to), delay 0:

- pass 0 over the slot list 1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 49;
- pass 1 over the same list plus 2: 3, 4, 7, 8, 11, 12, 15, 16, 19, 20, 23, 24, 27, 28, 31, 32, 35, 36, 39, 40, 43, 44, 47, 48, 51.

The list is the 25 bytes at `0x1002c192`; pass 1 adds 2 to every entry (E, `0x100140a0` to `0x100141c8`). Pass 1 does not start until every reply to pass 0 has arrived (the wait node). Whether the two passes spread the routers' inrush current by slot parity is not stated anywhere (U).

### Tail common to both modes

After the router steps (`0x10014224` to `0x1001433c`):

1. A parallel batch of `power up hold` to class 2 (the I/O and graphics bricks) over the slot list 1 to 40, then a delay of the delay word (500 ms by default).
2. A parallel batch of `power up hold` to classes 4 and `0x80` together (mask `0x84`) over the same list, then a delay of twice the delay word (1,000 ms by default).
3. A broadcast `reset` to every class (mask `0xff`, no list): it releases the hold, runs the queue and queues no trailing delay.

Bricks in slots above 40 are not reached by steps 1 and 2 (E from the list built at `0x10014224`); they can be reached only by the broadcast `reset` (and, for routers, by the router steps). Whether such slots exist in a real rack is not known (U).

**Ordering summary.** `fast`: routers in two slot-parity batches, then I/O, then compute and others, each with a reply barrier, then reset. `slow`: routers one by one with reply barrier and delay, then the same I/O, compute and reset tail. The delay applies after the I/O batch (1 x) and after the compute batch (2 x) in both modes, and between routers only in `slow` (serial with a rack destination). No path sleeps between bricks of the I/O or compute batches.

## Power down

Read `0x10014390` to `0x100149e4`. After the same `ver check` precursor and a destination check (`sub_1001f15c`):

- When the destination names racks: the code counts, over every rack's routers (types `R`, `D`, `-`, present), those not in the destination and, of those, the ones that are off. When the two counts are equal, which also holds when there are none, the L2 queues a `power down` packet (wait flag set, no delay) to every present router that is on (E, `0x10014720` to `0x10014970`).
- In every case a broadcast `power down` to every class (mask `0xff`) is then queued and started (E, `0x10014994` to `0x100149b8`). Within the one parallel batch the order is class 1, 2, 4, `0x80`, with a single reply barrier after it and no sleeps.

Power down therefore has no delay at all, and the routers appear twice in the queue when the first rule applied (explicit packet, then the broadcast). The reason for the explicit step is not stated in the code (U).

## The brick table order, the reply timer and `ver check`

All read in full in L2 1.44.0 `bin/l2`; the builder is also checked against 1.48.0.

**Order of the brick table (established, builder `0x1001e554` to `0x1001ed24`).** The table the sender walks is `0x184 + 4*i` of the destination context, `0x1184` entries counted at `0x1184`. The builder walks the chain of configuration nodes (next pointer at `+0x124`, starting at the local node `0x10044640`), and in each node the 40 port records (stride `0x1c`, from `+0x15c`), skipping any port whose state word is not 2. For each such port it visits the port's brick records (stride `0x30`) from the **last index down to index 0**, skipping a record whose byte 1 is zero, whose rack byte (3) or slot byte (4) is zero, or whose rack exceeds 255 or slot 63. The table entry is `((rack & 0x3ff) << 6 | (slot & 0x3f)) << 12`, the same address word the sender masks. An entry already in the table is skipped. A brick whose type character (byte 5) is one of the compute characters `C 3 5 4 A 6 9 7 8 0 Q @ O B F L M $ , } . {` is **appended** at the end of the table; every other brick is **inserted at the front** (the builder moves the existing entries up one word with `memmove` and stores at index 0). So, in the order the sender sends: the non-compute bricks (routers and I/O bricks together) come first in the reverse of the visiting order, and the compute bricks follow in the visiting order. Within one class the order is that filtered view: compute bricks in port order and, inside a port, descending brick index; router and I/O bricks in the opposite order (last visited first). The 1.48.0 builder (`0x1001f77c`) is identical instruction for instruction, apart from the address of the configuration object.

**The reply timer (established).** The word at `0x10043c70` is the **broadcast response timeout**, in seconds. It starts at 15 (data segment). At start-up `main` reads NVRAM setting 23 and, when that is nonzero, stores it in the word (`0x10003950`-`0x1000395c`). The request that sets it (the debug sub-command `l2dbg brspto`, help text "set broadcast response timeout limit") stores the new value in the word and writes it back to setting 23 (`0x100123b0`-`0x100123cc`), and the matching display prints "broadcast response timeout %d" (`0x10012870`). The runner passes the word as the seconds argument whenever it arms the reply timer (`0x1001c524`, `0x1001c8c8`, `0x1001d27c`). Only 1.44.0 and 1.48.0 carry the string `brspto`; the 1.24.1 and 1.32.4 daemons have neither it nor its sibling `trspto` (target response timeout), so their reply timers are not settable this way (the string is absent; their timer value was not read).

**`ver check` (established for the call, inferred for the check).** `sub_10010aac` at `0x10010aac` is a thin wrapper: if the command text is non-empty it clears the caller's result word and calls the command interpreter (`CmdProcess`, argument 1, the text, the result pointer, the L2's command table, length `0xbc` and the context). Called with the text `ver check` from the power-up and power-down paths, it therefore runs the L2's own `ver check` console command, whose help text is "display any L1 or L2 firmware versions mismatches". The caller does not read the return value, so `ver check` only prints a mismatch warning and never blocks or changes the power sequence (established from the code that follows the call at `0x10013d10`). What the version comparison itself prints was not read.

## What is unknown

| Question | Why | Route |
|---|---|---|
| Purpose of the slot-parity split and of the 40-slot cut-off | not stated in the code | the L2 and L1 manuals say only that power is ordered; none of the held ones gives slot numbers |
| `auto power up` timing and its use of this queue | only the strings and the `* power up` text were seen | read the power timer routine |
| The power command in L2 1.24.1 and 1.32.4 | neither image has the class-sorting builder (no comparison against the brick type character `{` appears in either daemon), so the queue is built differently; not read | disassemble the `power` command of those two daemons (the `power` help is present in both; 1.24.1 has no `mode`, 1.32.4 has it) |
