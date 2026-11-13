# The router packet in memory, and how the packet layer builds, writes and frees it

Read from L2 1.44.0, `/stand/sysco/lib/libIRouter.ppclinux.so` (40,084 bytes; 32-bit big-endian PowerPC, compiled without optimisation, so each routine is a straight copy of its source steps). Addresses are virtual addresses inside that library. Grades as in [daemon.md](daemon.md). The frame cut that a stream link applies afterwards is in [partition-task.md](partition-task.md); this note covers what is cut.

All four routines below were read in full: `IRouterMakePacket` (`0x67e8`, 524 bytes), the static builder it calls (`sub_5b2c`, `0x5b2c`), the static segment allocator (`sub_59dc`, `0x59dc`), `IRouterWritePacket` (`0x6ef0`, 272 bytes) and `IRouterFreePacket` (`0x657c`, 492 bytes), plus `IRouterInit` (`0x6384`, 324 bytes).

## Segment record

A packet is a singly linked list of segments. One segment is a heap block of (data size + 0x10) bytes, zeroed on allocation (`sub_59dc`: `malloc`, then `memset` of the whole block, established). The first 0x10 bytes are bookkeeping that never goes on a wire; the wire bytes start at segment offset 0x10.

| Segment offset | Content (established from the stores of `sub_5b2c` and the loads of the other routines) |
|---|---|
| +0x00 | pointer to the next segment, 0 in the last |
| +0x04 | word: zero from the allocator, then bits 1 to 15 (mask `0x7fff0000`) are a reference count, set to 1 when the packet is finished and tested and decremented by `IRouterFreePacket`; bits 14 and 15 from the low end (mask `0x0000c000`) hold the halfword at `IRByteOrder`+2 shifted left 14; the low 14 bits hold the last stack argument of `sub_5b2c` (zero from `IRouterMakePacket`) |
| +0x08 | pointer to an array of per-argument heap pointers that `IRouterFreePacket` frees, or 0 (never set by `sub_5b2c`) |
| +0x0c | word from the fifth stack argument of `sub_5b2c` (0 from `IRouterMakePacket`) |
| +0x10 | halfword: bytes on the wire for this segment, the header plus its arguments |
| +0x12 | byte 2, written by the allocator |
| +0x13 | byte from the seventh register argument of `sub_5b2c` (0 from `IRouterMakePacket`) |
| +0x14 | low byte of the sequence number (the eighth register argument), and +0x21 holds the next eight bits of it |
| +0x15 | bit 7 set on the first segment only; the low 7 bits count the segments, N on the first down to 1 on the last |
| +0x16 | the packet class byte (the second register argument of `sub_5b2c`, the value `IRouterMakePacket` receives as its fifth argument; the partition service tests it for 0, a request) |
| +0x17 | `(fourth register argument & 0x1f) << 3 | (fifth register argument & 7)`: a five-bit and a three-bit field; `IRouterMakePacket` supplies its third argument and the byte at +4 of the record behind its first argument |
| +0x18 | the four bytes behind the second pointer argument of `IRouterMakePacket` (one address word) |
| +0x1c | the four bytes behind the first pointer argument (the word the partition service classifies for the task) |
| +0x20 | byte: the number of descriptors (low byte of the first stack argument), written last; `IRouterFreePacket` reads it as the entry count of the pointer array at +8 |
| +0x22 | halfword: the request or reply code (the low half of the sixth register argument of `sub_5b2c`, `IRouterMakePacket`'s fourth argument) |
| +0x24 | the argument descriptors, 8 bytes each, copied from the caller's array |
| after the descriptors | the argument payloads |

Each descriptor in the caller's array is 8 bytes: byte 0 low 3 bits the type, bit 3 (0x08) a request that the payload pointer be freed after it is copied, a halfword at +2 the length and a word at +4 a pointer to the payload (read from the loops in `sub_5b2c`, established). A type of 0 means the descriptor carries no payload: its length is not added to the packet size. After copying, the byte at descriptor +1 gets the number of the segment (1 for the first) in which the payload begins and the word at +4 is replaced by the payload's offset from the start of that segment record (so a finished packet holds offsets, not pointers).

## Building (IRouterMakePacket, sub_5b2c, sub_59dc)

- `IRouterMakePacket` is the public entry; it forwards to `sub_5b2c` with the sequence number taken from the halfword at +6 of the record behind its first argument (incremented in place) and the byte at +4 of the same record as one of the two small fields at +0x17. That record's first word is the local address the check below tests. Before calling it decides whether the packet is "local": it sets a flag to 1 only when the second pointer argument is non-null, the fifth argument (class) is 1 (this is the "response" form by the reply path; inferred), the sixth argument is above 1 and both address words (the first word of each of the two pointer arguments) have a router-address form accepted by the check (a clear top bit with a top nibble of 1 or 2, or a set top bit with the top two bits `10` giving nibble 14 or the top two bits otherwise giving nibble 15; read from the compares at `0x6850` to `0x6920`). The flag selects the largest segment size, 0x3000 bytes instead of 0x400.
- `sub_5b2c` first returns -0x25f (-607) when the descriptor count is non-zero and the descriptor array pointer is null. It then totals 0x14 plus 8 per descriptor plus the length of each descriptor whose type is non-zero, allocates the first segment with `sub_59dc`, writes the fields in the table, and walks the descriptors copying payloads. A payload that does not fit the space left in a segment is split across segments: a new segment is allocated and given the same 0x14-byte header copy (`sub_59dc` copies 0x14 bytes from the previous segment's wire start, established), the segment counter is incremented, and the header bytes are the copy and payload placement continues after it. A payload that fits a fresh segment but not the room left in the current one starts a new segment instead of being split. At most 127 segments are allowed: a count above 0x7f frees every segment built so far and returns -0x272 (-626). An allocation failure returns -0x25c (-604). On success it stores the head pointer through its first argument, returns 0, and, when the debug flag `IRouterDebugMallocFree` is set, prints `CREATE` and the address to standard error.
- `sub_59dc(size, 0x400, limit, &chosen, head, last)` sizes and links a segment: the chosen size is the request when it is at most 0x400, otherwise the smaller of the request and the limit (0x3000 with the local flag, else 0x400), stored through the fourth argument. The block is `malloc(size + 0x10)` and zeroed, and byte +0x12 is set to 2. When a head segment is passed, its 0x14 wire header bytes are copied into the new segment, and the new segment's address is stored in the next-pointer of the last segment passed. If `malloc` fails it frees the chain from the head passed and returns 0; without a head it just returns 0.

## Writing (IRouterWritePacket)

`IRouterWritePacket(_, write_fn, context, head)` walks the list from the head. For every segment it calls `write_fn(context, segment + 0x10, halfword at segment + 0x10)` through a function pointer and requires the return value to equal the length. A short or failed write frees the whole packet (`IRouterFreePacket(head)`) and returns -0x26e (-622) when the write returned 0, and -0x269 (-617) for any other mismatch. When every segment is written it frees the packet and returns 0. So a packet is consumed by the write whether or not it succeeds. A byte-order value is computed from the word at +4 (bits 14 and 15) in the loop but never used (established; the stored local is not read again).

## Freeing (IRouterFreePacket)

Examines the reference field of the first segment. A field of 0 prints `packet free error, class: %d, src: %08x` (the class byte at +0x16 and the word at +0x1c) and returns 0 without freeing. Otherwise it decrements the field, and only when the result is zero frees the per-argument pointers (the count is the byte at +0x20, the array the pointer at +8, when it is non-null), the array itself, and then every segment along the list. It returns 0 in all cases. With the debug flag it prints `FREE` with the address and the count minus one first.

## IRouterInit

`IRouterInit(id)` sets `IRByteOrder` to 1 (the test that would choose 2 reads a local that was set to 1 on the previous instruction, so the `ntohl` branch at `0x644c` is never reached; established), clears `IRouterLocalAddress`, sets it to `0x80000000` plus the low 24 bits of `id` (top byte of the old value kept, which is zero), and calls `IRouterInitID` on `IRouterInternalID` with 1 and 3. It returns 0.

## What this settles for the partition fetch

The reply to a task-10 request is one packet built by this path: its code halfword at +0x22 and class byte at +0x16 come from the caller and the argument payloads follow the descriptors. A database of up to 64 KiB spans several segments (of at most 0x3000 bytes with the local flag, 0x400 without; both stay below the 127-segment limit). On a stream link the chunk writer sends each segment's wire bytes in 118-byte pieces; the segment counter in byte +0x15 and its bit 7 are what the `0x3001` first frame reports in its byte 6 (the low 7 bits of data byte 5, which is segment offset 0x15) and tests to choose `0x3001` over `0x3002`. This reading of data byte 5 as segment offset 0x15 is inferred from the +0x10 wire start shown here, not from a trace of a chunk write.
