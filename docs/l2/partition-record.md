# The partition record: request, reply, database body, and how the boot firmware's fetch lines up with them

Read from L2 1.44.0 (`l2.bin` of the IRIX 6.5.30 diagnostics product): `/stand/sysco/bin/l2part`, `/stand/sysco/bin/partmgr`, `/stand/sysco/lib/libpart.ppclinux.so`, `/stand/sysco/lib/libIRouter.ppclinux.so`; and from L1 1.44.0 package 1 (`Fuel/PE/O300 1MB`, body loaded at `0xffe00000`). Addresses are virtual addresses in the named file. Grades as in [../../AGENTS.md](../../AGENTS.md). The request codes are in [partition-task.md](partition-task.md), the database body in [libpart.md](libpart.md), the packet layout in [router-packet.md](router-packet.md); this note joins them to the stock boot firmware's fetch and adds the producer side of the database. The firmware's side (stock IP35 boot firmware 6.210: destination word, one zero-type descriptor, reply reader and its checks) is taken from a separate published reading of that firmware and is a hint here, not a finding: no boot-firmware image is in this repository's `root/`.

## Summary

- **What the fetch is.** The boot firmware's record fetch is a router request to controller type 1, task `0xa` (destination word `0x1ffff00a`) carrying one integer argument. The only task-10 code that takes one integer and answers with an integer that can be a partition number is **code 1** (brick to partition). Inferred, strongly: nothing in the L2 images names the sender, but no other code fits the request shape and the reply checks (below).
- **What the "record" is.** It is not the stored database. It is the reply packet of code 1: a 36-byte router segment whose second descriptor holds the partition number (0 to 63). The 64 KiB database of code 3 is a different message, used by `partmgr`.
- **Reply to an unlisted brick is 0, not -1.** Established below. The boot firmware's "-1 stops the path" check therefore cannot be reached by an answer from this handler.

## The reply to code 1, byte by byte

Handler `0x10001384` (`l2part`), read in full. It takes the brick from the request's first descriptor: when the type bits (low three of the byte at record `+0x24`) are 0 it is the halfword at record `+0x2a`, that is the low half of the big-endian value word, so wire bytes `0x1a` and `0x1b`; otherwise it is fetched through `IRArg(packet, 0)`. It calls the wrapper at `0x10002054`, which calls `get_partid_for_brick` (`libpart`, `0x546c`) and **replaces a negative result with 0** (`cmpwi r0,0 / bge / li r0,0` at `0x10002090` to `0x1000209c`). So the value is:

| Situation in the working mode | Reply value | Where read |
|---|---|---|
| the layout's "has partitions" byte (mode `+0x20`) is zero | 0 | `partid_for_brick` `0x5398` to `0x53a0` |
| partitioned, brick (or its rack, when the layout flag is set) listed | the partition byte, 1 to 63 in practice | `0x5408` to `0x543c` |
| partitioned, brick not listed | 0 (the library returns -1, the wrapper clamps it) | `0x5454`; `0x10002090` |

Established, read in full in 1.44.0; other versions not read for this wrapper. The brick value is the 16-bit identifier of [libpart.md](libpart.md#brick-and-rack-identifiers): type index in bits 15 to 12, rack in 11 to 6, slot in 5 to 0. When the layout flag (bit 15 of its count halfword) is set the library masks the brick with `0x0fc0` first, so one table entry covers a rack and the slot bits of the request do not matter.

The reply packet is built by `sn1SCSendRsp` from the record the handler leaves on its stack (`0x10001aa0`, count 2). Wire offsets (segment record offset minus `0x10`):

| Wire offset | Bytes | Content for a code-1 reply |
|---|---|---|
| `0x00` | 2 | length, big-endian: `0x0024` (header `0x14` plus two descriptors of 8; integers add no payload) |
| `0x02` | 1 | `0x02` |
| `0x03` | 1 | status byte: `(-a) mod 100` for the status argument `a`, which `l2part` always passes as 0, so **0** |
| `0x04` | 1 | low byte of the request's sequence number, copied |
| `0x05` | 1 | `0x81`: first segment, one segment |
| `0x06` | 1 | class 1 (response) |
| `0x08` | 4 | destination: the request's source word (the one the L1 stamped when it delivered the request) |
| `0x0c` | 4 | source: the L2's local address (`0x80000000` plus the low 24 bits of its id, [router-packet.md](router-packet.md#irouterinit)) (inferred; the reply path's source argument was not followed) |
| `0x10` | 1 | descriptor count: 2 |
| `0x11` | 1 | high byte of the sequence number |
| `0x12` | 2 | code, copied: `0x0001` |
| `0x14` | 8 | result descriptor: type 0, length 0, value word 0 (at wire `0x18`) |
| `0x1c` | 8 | integer descriptor: type 0, length 0, value word at wire `0x20`: the partition number, big-endian 32-bit |

Established for the handler, the result descriptor and the value word (stores at `0x100013d8` to `0x1000142c`, count word 2); established for the length, `0x14 + 8 * count`, from the builder ([router-packet.md](router-packet.md)); the fields at `0x02` to `0x12` are the packet builder's and are read there. The segment-number byte of an integer descriptor (wire `0x1d`) is 0 as `l2part` leaves it or 1 if the builder overwrites it ([userspace.md](userspace.md#9-the-reply-to-the-partition-record-requests)); that is inferred, not settled.

### How this meets the boot firmware's reply checks

The firmware's published reading records that its reader copies each frame's payload into a request block from block offset `0x20`, requires block byte `0x23` to be zero, and reads a 32-bit word at block offset `0x40` as the value for its set-partition request, stopping if it is `-1`. With block offset = wire offset + `0x20`:

- block `0x23` is wire byte 3, the status byte: 0 from `l2part` (established above for the L2; the block mapping is inferred).
- block `0x40` is wire offset `0x20`: the value word of the second descriptor, the partition number (established for the L2; mapping inferred).

The mapping needs the payload of the L1's reply frame to start at the first wire byte of the segment, the two-byte length. That is what the L1 sends: the first chunked reply frame's payload starts at packet `+0x10` (L1 1.44.0, `outbound_chunked_response`, `0xffe35a18`; [../l1/packet-chunks-and-l2-attach.md](../l1/packet-chunks-and-l2-attach.md) 1.8). The two views of the frame header agree once the firmware's reader is taken to drop the L1's first byte:

| L1 frame byte (L1 notes) | Boot-firmware reader byte | Content |
|---|---|---|
| 0 | not seen | `0x20` plus the subchannel index |
| 1 to 4 | 0 to 3 | zero |
| 5, 6 | 4, 5 | code `0x3001` (first frame) |
| 7 | 6 | segment count (`+0x15`, low seven bits) |
| 8, 9 | 7, 8 | length of the first segment (the halfword at packet `+0x10`) |
| 10 on | 9 on | payload, up to 113 bytes a frame |

So the "nine-byte header" in the published reading of the boot firmware is the L1's ten-byte frame header seen without its first byte. Inferred: both ends are read from images or listings, the join is the byte-for-byte agreement of code, count and length positions and of the status and value positions above. A reply to code 1 is one segment of 36 bytes, so it travels as a single `0x3001` frame with count 1, length 36 and a 36-byte payload.

**Consequence.** Code 1 is consistent with all three checks at once (status 0, value at block `0x40`, one frame). Code 4 (timestamp) and code 2 (variable count) also put an integer at wire `0x20` but do not mean a partition number. Code 3 puts a payload offset there, not a value. A code with no matching handler (including 0) gets a one-descriptor reply of 28 bytes, so block `0x40` would hold bytes the reply did not supply. Hence the inference above. What would settle it: the code halfword (wire `0x12`) of the boot firmware's request, which that reading leaves untraced.

## Test vectors for a stand-in L2 (task 10)

1. **Listed brick.** Mode with one partitioned layout, brick `0x0041` listed as partition 2. Request code 1, integer descriptor `0x0041`. Reply: 36 bytes, class 1, code 1, result 0, value 2, status byte 0.
2. **Unlisted brick, partitioned layout.** Same mode, brick `0x0042`. Reply value **0**, not -1 or 0xffffffff.
3. **Unpartitioned mode** (layout "has partitions" byte 0): any brick gives 0.
4. **Rack-wide layout** (flag bit 15 of the count halfword set, ids listed as rack fields): a request for brick `0x2041` is masked to `0x0040` (type and slot bits dropped) and matches a table entry `0x0040`. The table side is compared verbatim, so a rack-wide table must hold rack-only identifiers (inferred from the loop at `0x53d4` to `0x5418`, which masks only the request).
5. **Unknown code** (0 or 9): result descriptor holds -3, count 1, a 28-byte reply, and the log line `l2part task received unknown request %d`.
6. **No `l2part` registered** (the first seconds after the daemon starts, or any run level but 5). The request fails in the router with `-0x260` and the daemon's send routine answers the requester itself: a 20-byte (`0x14`) class-1 packet with the request's code and sequence, **status byte 8** (wire offset 3: `(-(-0x260)) mod 100`), and no descriptors; it also logs `NO RECEIVER`. Established, `sub_10022e50` in `bin/l2` read in full ([../l2-l1-conversation.md](../l2-l1-conversation.md#122-when-a-request-cannot-be-delivered-the-error-response)); the earlier statement in that note that no reply reaches the sender was wrong. For the boot firmware's reader, block byte `0x23` is then 8, not 0.
7. **Reply cannot be routed back** (the sender's rack and slot are in no port's table): the reply's own send fails with `NO RECEIVER`; a failed class-1 packet gets no further answer. Established for the rule, inferred for this case.

The probes that would confirm vectors 1 to 7 against the running daemon are P10 to P12 in [../l2-l1-conversation.md](../l2-l1-conversation.md#124-probes-for-a-scripted-peer-to-settle-what-static-reading-cannot).

## The database body: producer side

[libpart.md](libpart.md) reads the consumer, `unpack_mode_db`. The producer is in `partmgr`: the routine at `0x10005bc8` (stack frame `0xf0`, ends `0x10008170`; established by reading the stores and the sizes of every `memcpy`-shaped call, not instruction by instruction through all of its checks) writes the image in this order, each integer big-endian:

| Step | Bytes written | Notes |
|---|---|---|
| header in a global buffer | 24 | initialised at `0x10005be8` to `0x10005c4c`: magic `0x5a4c4a21`, size 0, checksum 0, timestamp from `time(0)`, the word at `0x10` zero, version 1; the 64 KiB buffer is zeroed; body starts at buffer `+0x18` |
| layouts | 1 (count `NA`) | one record per distinct layout of the modes read, built by comparing each mode's brick table with the layouts already collected (loop at `0x10005cc8` to `0x10005e00`) |
| per layout | 2, 1, 8, then 3 bytes per brick | count halfword (brick count, bit 15 set when the layout flag is), the "has partitions" byte, the 64-bit partition mask, then brick identifier (2) and partition byte (1) for each brick |
| blobs | 2 (count), then 2 + length each | environment blobs; each is the `NAME=VALUE` strings of one partition's variables, each NUL-terminated, then one more NUL |
| modes | 1 (count `NM`) | per mode: one identifier byte, 16 bytes of name (a `0x10`-byte copy at `0x100077a8`), one layout-index byte, one partition-count byte (the population count of the 64-bit mask, `__lshrdi3` loop at `0x100079e0` to `0x10007a50`), then per partition: number (1) and blob index (2) |
| closing | | size = end minus start (`0x100080a4` to `0x100080c0`), checksum over bytes `0x18` to the end (`0x100080e0`, the library's `get_cksum`), then the header copied to the start (`0x10008150` to `0x10008164`, 24 bytes) |

Established for the order and the sizes; the checks that fail the build are the strings listed in [partition-task.md](partition-task.md) (`environment too large`, `mode database is too large`, `database conversion stored bad data`, `lost bits in database conversion`) and were not each followed. The identifier byte of a mode and the header word at `0x10` that selects the active mode: `partmgr` writes the header word as zero (it is not stored by this routine), and `PART_SET_MODE` (code 6) is what changes it; where the mode identifier byte's value comes from was not read (unknown). Unverified for the mode-identifier source.

**Field meanings, summarised.** Layout: which bricks (or racks) belong to which partition numbers, plus the set of partition numbers in use. Blob: one partition's variable list, delivered by code 2 as `NAME` NUL `VALUE` NUL pairs with two final NULs. Mode: a 16-character name that ties one layout to one blob per partition. The active mode is the identifier in the header word at `0x10`.

**Test vector for the whole image** (constructed from this reading and from [libpart.md](libpart.md); the checksum value was cross-checked against the standard `cksum` utility on the same body, which agrees with the library's `chksum` as read, but the image was not run through the L2's own unpacker). One layout, two bricks (`0x0041`, `0x2041`) in partitions 1 and 2, two blobs (`PART=one`, `PART=two`), one mode named `default` with identifier 1, header mode word 1, timestamp `0x5f000000`: 94 bytes, checksum `0x61da32aa`.

```
5a4c4a21 0000005e 61da32aa 5f000000 00000001 00000001
01 0002 01 0000000000000006 0041 01 2041 02
0002 000a 504152543d6f6e65 0000 000a 504152543d74776f 0000
01 01 64656661756c7400 0000000000000000 00 02 01 0000 02 0001
```

The empty body's checksum (`create_default_db`) is `0xffffffff`; a 24-byte header with that checksum, size `0x18`, no modes is the default image.

## Messages that carry the record

| Carrier | Direction | What it carries |
|---|---|---|
| task-10 request code 1 over the router, from the boot firmware via the L1 | firmware to L2 | brick in, partition out (this note) |
| codes 2, 3, 4 | `partmgr` to L2 | environment block of a partition, whole database, timestamp |
| codes 5, 6 | `partmgr` to L2 | replace the database (header, size, checksum, timestamp and version checks, [partition-task.md](partition-task.md)); change only the active mode |
| event, `sn1SCEvent(5, 1, record)` | L2 daemon to peers | the new database as a binary argument after a local code-5 request |
| flash `0xfe3e0000`, 64 KiB | storage | the image, read at start and rewritten on a change |

The L1's own side of the arrangement is the `0x000b` answer that makes the boot firmware ask at all: see [../l1/rs422-router.md](../l1/rs422-router.md) for task forwarding. What the boot firmware does with a 0 reply (no partition) is its own sheet's question.

## Unknowns

- The code and argument the boot firmware actually sends (the inference above). Routes tried: every task-10 handler, the reply layout against the firmware's published reader checks, the L1 chunk code. Next: a boot-firmware image or a capture.
- Whether the 1.24.1 and 1.48.0 handlers for code 1 clamp negatives the same way (the dispatch range was checked in those versions, not this wrapper).
- The source of a mode's identifier byte on the producer side.
