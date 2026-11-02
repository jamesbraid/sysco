# L2 `power` command: the send queue, its order and its spacing

Read from the L2 daemon `bin/l2` of L2 1.44.0 (IRIX 6.5.30 `l2.bin`, carved with [../../scripts/l2_extract.py](../../scripts/l2_extract.py), listed with [../../scripts/l2_list.py](../../scripts/l2_list.py)). Addresses are the daemon's load addresses. Grades follow [AGENTS.md](../../AGENTS.md). Only 1.44.0 was read; the other three held L2 sets were not compared for this routine.

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

The reply timer is `EventAddTimeOut` with 15 in the seconds argument and 0 in the microseconds argument, from the word at `0x10043c70` (initial value 15, E). On expiry, if a timeout handler is set the handler runs; otherwise the queue is cleared (`0x1001ffe4` to `0x1002008c`; the clearing routine at `0x1001f5a0` was not read, I). Whether the 15 can be changed at run time is not known (U): the one store found is the initial value.

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

The loop is outer over class bits from the low bit of the mask to the high bit (doubling each time), then over the slot list, then over the L2's brick table (`0x184 + 4*i` in the destination context, `0x1184` entries). A brick is sent to when its slot (bits 12 to 17 of its address word) equals the list entry, or for every brick when there is no list, and its class bit is in the mask (E). Within a class the order is therefore the order of the destination's brick table; how that table is ordered was read only at its builder's start (`0x1001e554`), where it follows the rack table and each rack's brick array in index order (I).

**Trailing steps.** After the loops:

- Parallel (mode 1): when any packet was sent and r6 is nonzero, queue one wait-for-replies node (`-1`), then, only when r10 is zero, one delay node with the r7 value (E, `0x1001d1b0` to `0x1001d210`). So a parallel batch costs one wait for all replies, then r7 milliseconds.
- Serial (mode 2): after every packet, queue a wait-for-replies node and a delay node with the r7 value (E, `0x1001cd1c` to `0x1001cd78` and `0x1001d128` to `0x1001d160`). So each brick is sent, its reply awaited, and r7 milliseconds slept before the next brick.
- r10 nonzero: log `starting broadcast` and start the queue (E).

## Power up

Read `0x10013ce4` to `0x10014390`. A `ver check` request is sent first to every selected L1 (`0x10013d04`; the check itself is `sub_10010aac`, not read, U). The destination is then classified by the word at offset `0x180` of the destination context: 2 when the destination names whole racks, 1 otherwise (E for the store at `0x1001e664`; the test is whether the rack mask copy is non-empty, I). The mode is NVRAM key 21.

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

## What is unknown

| Question | Why | Route |
|---|---|---|
| Purpose of the slot-parity split and of the 40-slot cut-off | not stated in the code | the L2 and L1 manuals say only that power is ordered; none of the held ones gives slot numbers |
| Order of the brick table inside a class | builder read only at its start | read `sub_1001e554` (`0x1001e554` to `0x1001ecfc`) in full |
| Whether the reply timer value 15 can change | one store found | search the daemon for stores to `0x10043c70` |
| Behaviour of `ver check` | `sub_10010aac` not read | read it |
| `auto power up` timing and its use of this queue | only the strings and the `* power up` text were seen | read the power timer routine |
| The same routine in L2 1.24.1, 1.32.4 and 1.48.0 | not compared | carve those sets and compare the entry `0x10013ba8` |
