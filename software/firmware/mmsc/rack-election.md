# Rack identity election and the inter-MMSC link

What the MMSC (the multi-module system controller of an Origin 2000 or Onyx2 rack, also called the full-feature system controller) does to find the other MMSCs on its Ethernet, to agree on one master, to give every rack an ID and an IP address that no other rack uses, and to carry a command to a peer rack. Lane 6 of [controllers.md](../../../evidence/controllers.md).

All of it is read from `mmscfw.bin` in IRIX 6.5.30 (MMSC 2.3), sha256 `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`, the a.out image described in [controllers.md](../../../evidence/controllers.md). The same bytes ship in 6.5.7 to 6.5.30; the other builds are compared with this one in [the same election in the other builds](#the-same-election-in-the-other-builds).

## Reading the image

- The a.out header gives text 687,064 bytes, data 166,912, bss 123,288, entry `0x108000`. Text and data are one contiguous run from file offset 32: an address `A` below `0x1d8758` sits at file offset `A - 0x108000 + 32`. The copyright string at `0x1d6a58` and the nlist address of the symbol that names it agree, which checks the mapping. Everything from `0x1d8758` up is bss: the identity table, the election list, `ffscLocal`, `ffscTune` and the other globals named below are zero in the file and are filled by code at start-up.
- The 7,907 nlist entries (12 bytes each, string table after them) name every routine. The symbol table lists an object's external references with the addresses of their definitions, so the "71 routines" under the `addrd` object are 29 routines defined in `addrd.o` plus 42 references. The routines read here are the ones whose address lies inside the object's own text: 29 in `addrd`, 21 in `identity`, 9 in `remote`, 5 in `oobmsg`, 6 in the MMSC `route`; and, because a second object also called `route.o` lies inside the TCP/IP stack, its 6 routines.
- Code was disassembled with capstone in 32-bit x86 mode and every routine read through its last `ret`; string literals sit between routines, and sizes below count code only.
- Strings, the tune table and the oob response records were read from the same file. Timings are given as the code states them (microseconds, or clock ticks from `sysClkRateGet`; a tick is 1/60 second, see [the clock tick](#the-clock-tick)).

## Rack election in one page

Each MMSC runs one task, `tAddrD` (priority 25, 20,000-byte stack, started by `addrdInit`), that owns an in-memory table of every MMSC it has heard from (the identity table, [identity-table.md](identity-table.md)). Peers talk to it with raw Ethernet broadcast frames, and the MMSC's own code talks to it through the same task queue, a VxWorks pipe, so there is one writer to the table.

**Frame.** A 62-byte Ethernet frame sent by `etherOutput` on the MMSC's own interface: destination `ff:ff:ff:ff:ff:ff` (the six bytes at `etherbroadcastaddr`), source the MMSC's own MAC address, a 16-bit type field loaded with `0x0108` that `ether_output` byte-swaps, so the type bytes on the wire are `01 08`, and a 48-byte payload. The receiving hook sees the raw frame and tests the type as the little-endian word `0x0801`, which is the same two bytes. The hook is registered with `etherInputHookAdd`, writes each matching frame in full to the pipe `/pipe/addrd` (23 messages of 62 bytes), and reports the frame consumed whether or not the write worked. A frame of this type that is not exactly 62 bytes is logged and dropped by the task.

**Payload (48 bytes, little-endian 32-bit fields, no byte swapping).**

| Offset | Field | Notes |
|---|---|---|
| +0 | version | 3 when sent |
| +4 | type | 1 to 7, below |
| +8, +0xc | module numbers 0 and 1 (types 1, 2, 3, 6) | the module numbers of the MSCs on the MMSC's two bays; `0xffffffff` is unassigned |
| +0x10 to +0x1b | unused by types 1, 2, 3 and 6 | the sender does not write them |
| +0x1c | capability flags | local value is 1 |
| +0x20 | unique ID, 8 bytes | two zero bytes then the 6-byte MAC address |
| +0x28 | IP address, 4 bytes | as stored in `in_addr` |
| +0x2c | rack ID | `0xffffffff` is unassigned |

The task upgrades older payloads on receipt: version 0 or 1 gets module numbers `0xffffffff, 0xffffffff`, and version 0 to 2 gets capability flags 0; the version then reads 3. A payload with any version above 3 is taken as it is.

**Types**, in jump-table order (`type - 1` indexes seven handlers), with the debug text each carries where it has one:

| Type | Name | Sent by | What the receiver does |
|---|---|---|---|
| 1 | IDENTITY | an MMSC announcing or changing its own identity, or answering a QUERY | records the sender in the table; during an election, restarts the 2-second timer; on the master, resolves conflicts |
| 2 | QUERY | the MMSC that starts an election | as IDENTITY, then waits 3 clock ticks, broadcasts its own IDENTITY, restarts the timer and marks an election in progress |
| 3 | ASSIGN | the master giving another MMSC a new rack ID, IP address or module numbers | acted on only if the unique ID in the payload is its own |
| 4 | election start | the local code, never broadcast | broadcasts a QUERY and starts the timer |
| 5 | election end | the timer, never broadcast | prunes the table, picks the master, wakes waiters |
| 6 | REQUEST_RACKID | an MMSC whose operator set a rack ID | records the sender; on the master, reassigns any other MMSC that holds that rack ID |
| 7 | partition information | `broadcastPartitionInfo` | merges one module-to-partition fact into the local partition table |

Types 4 and 5 travel as a 62-byte message with an all-zero header except the type word, written to the pipe by `addrdLocalMsg`.

## The election

1. **Start.** At boot `ffscInit` runs `identInit`, brings up TCP/IP, then `addrdInit` and then `addrdElectMaster`, which blocks until the election finishes. `addrdElectMaster` puts a wait record (a binary semaphore and the current time) on a list, sends the local type 4, and sleeps on its semaphore without a timeout.
2. **Query.** The task handles type 4: it stamps `addrdLastQuery` with the current time, broadcasts a QUERY carrying the local identity, starts a 2-second timer (twice `sysClkRateGet` ticks) and sets the election flag. Any MMSC that receives the QUERY stamps its own `addrdLastQuery`, records the sender, waits 3 ticks, broadcasts its own IDENTITY, and starts or restarts its own 2-second timer.
3. **Extension.** Each IDENTITY, QUERY or REQUEST_RACKID received while the flag is set re-arms the same timer, so the window ends 2 seconds after the last message of any of these types.
4. **End.** The timer sends the local type 5. The handler deletes every table entry whose last-heard time is earlier than `addrdLastQuery` and whose unique ID is not the local one (clearing the master if it was the master), then chooses the master: the entry whose unique ID compares lowest, as a byte string from the first byte. The local MMSC is master if that entry is its own. It then takes the election-list guard (one second timeout), releases and removes every wait record stamped no later than `addrdLastQuery`, clears the flag and, if it is master, runs conflict resolution.
5. **Conflict resolution (master only).** `identScanForConflicts` repeats until a whole pass changes nothing, first for IP addresses and then for rack IDs. An entry whose IP address has a host part of 15 or less, or whose rack ID is unassigned, is its own conflict. Two entries with equal IP addresses (host part above 15), or equal rack IDs, conflict with each other. In every pairing the entry with the higher unique ID loses. The loser gets the lowest unused IP address in the range with host part `0x10` to `0xfffffe` on the local network number, or the lowest unused rack ID from 1 to 21, and an ASSIGN is broadcast to it. If no address is free the master logs it and carries on with the next entry; if no rack ID is free it logs and stops the whole scan, reporting that identifier conflict resolution failed.
6. **Rack ID request.** When an operator sets a rack ID, the MMSC stores it, updates its own table entry, and broadcasts a REQUEST_RACKID built from its own identity (the master also sends the same message to its own task). The master then looks for another table entry holding that rack ID and, if there is one, gives that other MMSC a new lowest-free rack ID. The newcomer's request wins; at election end the higher unique ID wins instead.

## The clock tick

Established, read in all ten MMSC images held (6.4, 6.4-origin, 6.5, 6.5.2, 6.5.5, 6.5.6, 6.5.7, 6.5.9, 6.5.10, 6.5.30) and in the patch SG0002378 image: the system clock runs at 60 ticks per second, so one tick is about 16.7 ms.

- `sysClkRateGet` returns the word `sysClkTicksPerSecond` (`0x1afe64` in 6.5.30) and nothing else. The word's initial value in the data segment is 60 in every image, and so is that of the auxiliary clock's word, 64.
- `usrRoot` (`0x10c888`) calls `sysClkConnect` with the `usrClock` routine, then `sysClkRateSet(60)` (`push 0x3c` at `0x10c8c0`, call at `0x10c8c2`), then `sysClkEnable`. That is the only call of `sysClkRateSet` in each image. The call has the same shape at `0x10c846` (6.4), `0x10c8b0` (6.5, 6.5.2, 6.5.5) and `0x10c8c0` (6.5.6 to 6.5.30). `sysClkRateSet` accepts 19 to 4,660 and stores the value, so the 60 is not changed later.
- `sysClkEnable` (`0x108548`) divides the constant `0x1234dc` (1,193,180) by the rate and writes the quotient, low byte then high byte, to port `0x40` after writing `0x36` to port `0x43`, then unmasks interrupt line 0 of the first interrupt controller. The constant, the port pair and the control byte (counter 0, two-byte load, square-wave mode) are those of an 8253/8254-style timer on the PC I/O map. The tick interrupt handler `sysClkInt` calls the connected routine, `usrClock`, which calls `tickAnnounce` (`0x193f94`).

What follows for the timings in this note, all established from the code that uses them:

| Timing | Ticks | Time |
|---|---|---|
| reply delay after a QUERY | 3 | 50 ms |
| election window | twice `sysClkRateGet` = 120 | 2 s |
| per-rack pause before the router reboots the MMSC | 60 times the rack ID | the rack ID in seconds |

Inferred, not read in an image: that the timer's input clock is the PC-standard 1.19318 MHz. The firmware assumes it when it computes the divisor, and the divisor is exact only if it holds. A published description of the MMSC's timer part or a PC-compatible chipset manual would settle it; the tick rate the firmware programs does not depend on it.

## The same election in the other builds

Established by routine-by-routine comparison of the address daemon, identity and remote-command code of every distinct MMSC image against the 6.5.30 image, with in-image addresses masked and routines paired by symbol name (images: 6.4 and 6.4 Origin, the patch SG0002378, 6.5.2, 6.5, 6.5.5, 6.5.6, and the 6.5.30 image that 6.5.7 to 6.5.10 also carry). The `_addrd*` and `_ident*` routines are instruction-for-instruction the same except where listed.

- **Same in all builds.** The frame handler `_addrd` has the same six handlers for types 1 to 6 in every image, with the same code in each; so the type table, the QUERY and IDENTITY exchange, the 3-tick reply delay, the 2-second window (twice `sysClkRateGet`) and the end-of-election pruning and master choice are unchanged from the 6.4 builds to 6.5.30. The clock rate is 60 ticks per second in all of them ([the clock tick](#the-clock-tick)), so the two timings are the same wall-clock values. The master is still the lowest unique ID. `_addrdAssignNewIdentity` is the same in every build.
- **Type 7 (partition information) arrives in 6.5.** The jump table of `_addrd` has six entries up to 6.5.2 and the patch, and seven from 6.5 (IRIX 6.5 base). The 6.5 handler for type 7 additionally logs a line; from 6.5.5 it matches the 6.5.30 handler. `_addrdUpdatePartInfo`, `_addrdSendPartitionInfo` and `_broadcastPartitionInfo` do not exist before 6.5.
- **A failed rack-ID search is handled differently before 6.5.5 (established from the code shape).** `_addrdResolveRackIDConflict` is the same modulo addresses in 6.5.5 and later. In 6.4, the patch, 6.5.2 and 6.5 it calls `_addrdFindUnusedRackID`, and when the result is -1 it logs the loser's unique ID and then still calls `_addrdAssignNewIdentity`. From 6.5.5 the -1 case logs and returns -1 without assigning. What `_addrdAssignNewIdentity` does with an unassigned rack ID in the older builds was not read here.
- **The rack-ID range grows by build.** The largest rack ID the search tries (`_addrdFindUnusedRackID`, loop bound), the limit `_ffscParseRackID` compares an operator's value against, the loop bound in `_destNumRacks`, the size of the rack array in `_MapAllRacks` and the limit in `_identInit` are the same number within a build and differ between builds:

| Build | Largest rack ID tried by the search | Parser compare | `_MapAllRacks` array |
|---|---:|---:|---:|
| 6.4, 6.4 Origin | 7 | 8 | 9 words |
| SG0002378, 6.5.2, 6.5 | 9 | 10 | 11 words |
| 6.5.5 | 19 | 20 (and a zero test) | 21 words |
| 6.5.6 and 6.5.30 | 21 | 22 (and a zero test) | 23 words |

  The `_addrdInit` queue-depth argument also grows (5 in 6.4 and the patch, 11 in 6.5.2 and 6.5, 21 in 6.5.5, 23 from 6.5.6): it is the message count of the address pipe (the same call that creates `/pipe/addrd`, with 62-byte messages), which grows with the rack range. A 6.4 or 6.5.2 MMSC therefore cannot hand out a rack ID above 7 or 9, and the rack-ID range an MMSC accepts from an operator also differs; the rack numbers named earlier in this note (1 to 21) are those of 6.5.6 and later. `_identInit` has no range check before 6.5.5.
- **Still not compared.** The payload layout (version 3 and the field offsets) was compared only through the handlers, which agree; the sender routines `_addrdLocalMsg` (differs from 6.5.5 only in string literals) and the broadcast helpers were not read field by field in 6.4.

## What a multi-rack Origin 2000 needs from each MMSC

- Every MMSC on one Ethernet segment, each with a MAC address that is unique: the election key is the lowest unique ID, which is the MAC byte string.
- A rack ID from 1 to 21 that no other rack uses. A blank NVRAM record gives rack ID 1, so a rack that was never configured collides with rack 1 and is renumbered by the master at the first election after another MMSC joins; rack IDs 0 and 22 can be stored but are never handed out, and the command router refuses a remote request that names a rack above 23.
- An IP address no other MMSC uses, `100.a.b.c` by default where the controller firmware source unit 81 are the low three bytes of the MAC address (a host part below `0x10` or above `0xfffffe` is replaced with `0xf`, which the master then reassigns).
- The module numbers of the MSCs on its two bays, refreshed whenever the module scan changes them, so every peer can route a command to a module by number.
- A TCP listener on port 5678 (the default tune value) that accepts connections only from IP addresses already in the identity table ([remote-link.md](remote-link.md)).
- An election at every boot, 2 seconds after the last message, after which exactly one MMSC reports itself master.

## Notes in this directory

| Note | Subject |
|---|---|
| [addrd-routines.md](addrd-routines.md) | the 29 routines of the address daemon, one entry each |
| [identity-table.md](identity-table.md) | the 21 routines of the identity table and the compare and format helpers they use |
| [remote-link.md](remote-link.md) | the TCP link between MMSCs, the 9 routines of `remote` and the tune values it uses |
| [oob-and-router.md](oob-and-router.md) | the out-of-band message of `oobmsg`, the command router of `route`, and the stack's routing routines |
