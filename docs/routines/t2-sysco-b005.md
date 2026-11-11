# Sysco tier-2 reading batch 5

Batch 5 covers the 25 routines at ranks 125 to 149 (zero-based) of the located-grade target order, all in `6.5.12/usr/sbin/flashsc`. All 25 rendered and passed the grader, so none were rejected. Every grade is inferred: the facts give callees, constants and branch conditions, but not the values of entry arguments or the contents of the strings and globals they refer to.

## `6.5.12/usr/sbin/flashsc` at 0x1001ea40

`l1d_read_packet` calls `IRouterReadPacket` at 0x10020de0 with the constant 0x10047520 and the address of `l1d_read_ppp`, then branches on the return value and on the global at 0x10047530. Its status and error exits use `fprintf` (format strings at 0x10045500, 0x10045538, 0x10045578 and 0x10045598), `LoggerDefaultMessage` at 0x1002f840, `IRFmtError` at 0x10022140, `IRFmtPkt` at 0x10022060 and `EventStop` at 0x10025210. The name suggests the read side of the debug packet path, but the facts do not show what the packet is, and the global at 0x10047554 is compared against 2 on two paths without a known meaning.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001eca0

`l1d_write_data` makes two `fprintf` calls, at 0x1001ecf8 and 0x1001ed74, with format strings at 0x100455b0 and 0x100455d0. On one path it reads `errno` at 0x1001ed6c. It makes an indirect call at 0x1001ed10 whose target the renderer could not name, and a direct call to an unnamed routine at 0x1001f280 with the constant 0x10047820 as its first argument. The write-data role comes only from the name, and the facts do not show what the indirect call does with its arguments.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001eda0

`l1d_write_ppp` prints through `fprintf` at 0x1001edf0 and 0x1001ee3c, then calls `PPPWrite` at 0x10030120 with the global `ppp` as first argument and the address of `l1d_write_data` as second. It passes its first three entry arguments on as the third through fifth arguments of `PPPWrite` and returns that call's result. The two branches on the global at 0x10047530 decide whether the print runs, so it reads as a logging wrapper around the PPP write, but the facts do not show what the PPP layer does with the callback.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001ee60

`l1d_write_packet` calls `IRouterWritePacket` at 0x100213a0 with the constant 0x10047520 and the address of `l1d_write_ppp`, which makes it the write-side counterpart of `l1d_read_packet`. It loads a word at offset 0x18 of its second argument and passes it to an unnamed routine at 0x1001f040, then logs the return value of `IRouterWritePacket` through `LoggerDefaultMessage` at 0x1002f840. Its three `fprintf` calls use format strings at 0x10045638, 0x10045658 and 0x10045688. The field at offset 0x18 is not named in the facts.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001ef80

`l1d_ppperr` passes its four entry arguments unchanged to `LoggerDefaultMessage` at 0x1002f840, then prints a fixed format string at 0x100456a0 through `fprintf` at 0x1001efd8, using the logger's return value as the third argument. Its only branch, at 0x1001efe0, is always taken, so the routine has one path. The name suggests a PPP error reporter, and the facts agree with that, but they show only that it logs and prints.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001f3b0

`IRouterConnectL3` calls `RouterInit` at 0x10026ed0. If that returns a negative value, it logs through `syslog` at 0x10030eb0 and calls `exit` with -1. On the success path it calls `RouterMakeConnection` at 0x1002df80 with the constants 2, 0x12c and 0x100 and the callbacks `rtr_connected` and `rtr_event`, then starts the event loop with `EventStart` at 0x100250b0 on the globals `rtr_server`, `rtr_error` and `IRouterEventLoop`. This reads as the connect step of the L3 router, with `IRouterDisconnectL3` at 0x1001f590 as its counterpart by name. The meaning of the constants 2, 0x12c, 0x100 and 0x2fb60 is not named in the facts.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001f590

`IRouterDisconnectL3` only calls `RouterClose` at 0x1002d320 with its four entry arguments unchanged, and returns that call's result. It has no branches and no memory accesses of its own, so any disconnect behaviour sits in `RouterClose`. By name it is the counterpart of `IRouterConnectL3`.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001f5d0

`IRouterRequestL3` calls `RouterFindClient` at 0x10028d80 with its first four entry arguments. The branch at 0x1001f61c sends a failed lookup to a return of -0x67 (-103). On a found client it builds a packet with `IRouterMakePacket` at 0x10020d30 and forwards it with `IRouterForwardPacket` at 0x10022dd0, and it returns -0x194 (-404) when the packet cannot be made. The halfword at offset 6 of its second argument and the store through its seventh argument at 0x1001f654 are not named in the facts.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001f6e0

`IRouterResponseL3` is the response-side sibling of `IRouterRequestL3`. It calls `RouterFindClient` at 0x10028d80 and returns -0x67 when the lookup fails and -0x194 when `IRouterMakePacket` at 0x10020d30 returns zero. It reads a pointer at offset 0x1c and a halfword at offset 0x22 of its fourth argument to build the packet, copies bytes at offsets 0x14 and 0x21 of that argument into the new packet, and forwards it with `IRouterForwardPacket` at 0x10022dd0, passing the constant 0x64 as the eighth argument. The field meanings and the role of 0x64 are not named in the facts.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001f840

`IRouterGetPacketL3` is one of the larger routines in the batch, at 1152 bytes. It reads many fields through its third argument, which holds a client pointer, and checks the client list with `RouterFindClient` at 0x10028d80, `RouterConnectionCount` at 0x100271b0 and `RouterListClients` at 0x10028e00. It sets a timeout with the constant 0x3e8 through `EventCreateTimeOut` at 0x10024100 and `EventAddTimeOut` at 0x10024000, using `pkt_timeout` as the callback. It waits on the event loop with `EventStartSubset` at 0x10025150 and `EventStart` at 0x100250b0, cancels the timer with `EventRemoveTimeOut` at 0x100242a0, and writes `rtr_pkt`; the order of these steps and what the 0x3e8 value means beyond its use as a timeout are not established.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001fd60

`rtr_connected` writes `rtr_server` and `rtr_error` at 0x1001fd98 and 0x1001fda0 and logs through `syslog` at 0x10030eb0 with line 0x172 and format string 0x100457a0. The flush and stop calls, `RouterFlushOutput` at 0x10027f50 and `EventStop` at 0x10025210, sit on the fall-through path of the branch at 0x1001fde4 on `IRouterEventLoop`, so they run only when that global is zero. The name suggests a connection-established callback, but the stored values are not in the facts.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001fe30

`rtr_connect_failure` mirrors `rtr_connected`. It writes `rtr_server` and `rtr_error` at 0x1001fea4 and 0x1001feb0, calls `LoggerDefaultMessage` at 0x1002f840 and `syslog` at 0x10030eb0 with line 0x195, then flushes output with `RouterFlushOutput` at 0x10027f50 and calls `EventStop` at 0x10025210. As in `rtr_connected`, the flush and stop sit only on the path where `IRouterEventLoop` is zero, per the branch at 0x1001febc. The name suggests a connection failure callback; the facts do not establish its arguments.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001ff00

`rtr_event` writes `rtr_server`, `rtr_error` and `rtr_pkt`, then calls `IRouterPacketFromRouter` at 0x10023250 with its third and fourth entry arguments and the constant 0x2f010. If that call returns zero, it logs through `syslog` at 0x10030eb0 with line 0x1b9 and returns. Otherwise it flushes output and stops the event loop, and per the branch at 0x1001ffb4 those two calls run only when `IRouterEventLoop` is zero. The name suggests the event callback for packets from the router, but the facts do not show the packet layout or the meaning of 0x2f010.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x1001fff0

`pkt_timeout` writes `pkt_timeout_id` at 0x10020010, `rtr_server` at 0x10020018 and `rtr_error` at 0x10020024. It then calls `EventStop` at 0x10025210 with `pkt_timeout_id` as first argument and the constant 0x2ef20 as second, and returns that call's result. The stored values are not in the facts, so the handler's effect beyond stopping the event is unclear.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10020920

`IRouterInit` stores to the globals `IRByteOrder` (at 0x10020958 and 0x1002096c) and `IRouterLocalAddress` (five stores from 0x10020978), and compares `IRByteOrder` against 1 at 0x100209ac. It calls an indirect target at 0x100209f4 that the renderer names `ntohl`, and calls `IRouterInitID` at 0x10020a70 with the global `IRouterInternalID` and the constants 1 and 3. The value written into the byte-order global and the address are not in the facts, so the byte-order handling is only partly visible.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10020a70

`IRouterInitID` fills an 8-byte record at its first argument. It stores a word loaded from the absolute address 0x10047458 and the global `IRouterLocalAddress`, then writes a byte at offset 4 and a halfword at offset 6 with values the facts do not name. It returns zero. `IRouterInit` calls it with the internal ID object.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10020b20

`IRouterFreePacket` releases a packet object and its per-entry pointers. It frees the pointer loaded from offset 8 and two other pointers, at 0x10020c80, 0x10020c48 and 0x10020cb8, and compares an index against the byte at offset 0x20 to walk an array. When the global `IRouterDebugMallocFree` is set it also prints a trace with `fprintf` at 0x10020b70, `fflush` at 0x10020b84 and `printf` at 0x10020bb4. The halfword at offset 4 and the exact loop bounds are read from the branch structure only, and the facts do not name them.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10020cf0

`IRouterSetQueueSuspendTime` writes two words to the absolute addresses 0x10047a08 and 0x10047a0c and returns its second argument. The facts show the two stores by address but not which argument they store, so the link to a suspend time comes from the name alone.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10020d30

`IRouterMakePacket` reads a byte at offset 4 and a halfword at offset 6 of its first argument, writes the halfword back at offset 6, and calls an unnamed routine at 0x10020190. That call gets a local buffer at sp-80 and its entry arguments, and a branch at 0x10020db8 tests its result. The return value is either zero or unknown in the facts, and the fields and the callee's behaviour are not established, so the packet-building role is a guess from the name.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10020de0

`IRouterReadPacket` is one of the largest routines in the batch, at 1468 bytes. It clears a 128-byte local set and calls `select` at 0x10020f18 on it, with a pointer to a local timeout at sp-232 as the fifth argument. It allocates a buffer with `malloc` at 0x10021014, clears 44 bytes of it, and sets the byte order through `IRouterSetPacketByteOrder` at 0x10021204 using `IRByteOrder`, freeing temporary buffers on several paths. Error returns are -0x199, -0x19d, -0x19e and -0x194, and the success path returns zero; most reads go through pointers the facts do not resolve, so the packet layout and the meaning of those codes are not established.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x100213a0

`IRouterWritePacket` frees its packet argument with `IRouterFreePacket` at 0x100213dc, 0x10021474 and 0x100214ec, on paths that look like error exits. It calls `IRouterSetPacketByteOrder` at 0x10021434 with the constant 1 and again at 0x100214c4 with a value the facts leave unknown. It also makes an indirect call at 0x10021450 with an unnamed target, and reads two halfwords and a word from pointers the facts do not resolve. The returns -0x199, -0x1a6, -0x1a1 and zero suggest error codes on the early exits, but the facts do not establish which path yields which code.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10021510

`IRouterSetPacketByteOrder` converts the multi-byte fields of a packet header at its first argument between host and network order. It returns its second argument early when the halfword at offset 6 equals it, and otherwise touches the fields at offsets 0x10, 0x18, 0x1c, 0x20 and 0x22. The direction follows the second argument: the branch at 0x10021560 selects the `ntohs` and `ntohl` calls when that argument is 2, and the `htons` and `htonl` calls otherwise. The renderer names those calls from indirect jumps, so the names are its guess, and the facts do not show the field layout or what the other values mean.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10021810

`IRArg` reads a one-byte count at offset 0x20 of its first argument and compares its second argument against that count, returning zero when the index is at or past the count. Inside the range it indexes an array of 8-byte records starting at offset 0x24, and it allocates a table of four-byte slots sized by the count with `malloc` at 0x100219b0, which it stores at offset 8 and clears with `memset` at 0x10021a08. It copies a block with `memcpy` at 0x10021b30, sized by a halfword in the record. The record fields and the meaning of the 0x3ed bound are not named in the facts.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10021c00

`IRFmtClas` maps a small code to one of four static addresses. Its branches test the first argument against zero, one and two, and the four returns are the constants 0x10047850, 0x10047858, 0x10047860 and 0x10047868. The strings behind those addresses are not in the facts, so the class names are unknown. Part of the routine is not reached from the entry, and the facts say so.

Grade: inferred.

## `6.5.12/usr/sbin/flashsc` at 0x10021cb0

`IRFmtAddr` formats an address into the buffer its second argument points to. It tests the first argument against the mask 0xffffffff80000000 at 0x10021ce4 and returns that same constant on one early path. Two `sprintf` calls, at 0x10021fc8 and 0x10022018, write the output, with `strlen` between them to advance the position, and the success path returns the buffer pointer. The comparison against 0x10 and the check on the third argument are not named in the facts, so the output format remains unknown.

Grade: inferred.
