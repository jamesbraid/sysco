# The partition task (task 10) and the chunked reply framing

Read from L2 1.44.0: `/stand/sysco/bin/l2part` (16,176 bytes, executable at `0x10000000`), `/stand/sysco/lib/libpart.ppclinux.so`, `/stand/sysco/lib/libIRouter.ppclinux.so` and `/stand/sysco/bin/partmgr`. Addresses are virtual addresses in the named file. Grades as in [daemon.md](daemon.md).

## What the L1 notes already say

The L1 packet assembler accepts pseudo-request codes `0x3001` to `0x3003` as pieces of one larger request (see the `chunk_assembler` row in [../l1/functions.tsv](../l1/functions.tsv)). This note reads the L2 side of the same framing and the L2 service that fetches the partition records. The firmware of the PROM that issues the fetch is not part of these images, so **nothing here is read from the PROM**: the link from "task 10" to the PROM's fetch is a consequence of the task number and the framing only, and it is unknown what the PROM sends beyond that. It is recorded in [../open-questions.md](../open-questions.md).

## Who answers task 10

`l2part` is started by `inittab` at runlevel 5 only (see [rootfs.md](rootfs.md)). Its `main` (`0x10000ce0`-`0x10000fb0`, parse of the common options) loads the partition database (`sub_10001d2c`, below), sleeps 5 seconds, and calls `sn1SCInitL2App(host, 0, 10)`, registering as task **10** with the local daemon (established: `li r5, 0xa` at `0x10000e8c`; its second connect attempt after `retrying connection in 5 seconds...`). It then loops in `sn1SCPkt`, which hands every inbound packet to the callback `sub_10001120`.

`sub_10001120` classifies the packet's address word, the 32-bit big-endian word at packet offset 0x1c (established, the compares at `0x10001138` to `0x100012a8`). Two address forms are accepted: with bit 31 clear the top nibble must be 1 (the code also tests for nibble 0xe in that branch, which cannot hold when bit 31 is clear), and the task is the low five bits; with bit 31 set the top two bits must be binary 10, and the task is bits 29 to 24. The word `partmgr` uses, `0x1ffff00a`, is the first form with task 10. Task 4 goes to `SysCfgConnProcessEvent` in `libmain` (the configuration events of the L2 network); task **10** goes to `sub_100012c4`, the request handler.

## Request codes of task 10

`sub_100012c4` reads the request code from the halfword at packet offset 0x22, then does a bounds-checked jump through the table at `0x100029e0`, code minus 1 as the index, for codes **1 to 8**. Any other code logs `l2part task received unknown request %d` and replies with result -3. Established: the dispatch compare (`addi -1; cmplwi 7`) and the table's eight entries decoded. `partmgr` sends exactly these codes with `sn1SCCmd` to the destination word `0x1ffff00a` (task 10) with `0x1e` as the last argument (a timeout in seconds, inferred); its error strings name the codes.

| Code | Handler | What it does (read from the handler) | `partmgr` name |
|---|---|---|---|
| 1 | `0x10001384` | argument: a brick number, taken from the first argument (or the halfword at +0x2a when the argument type is immediate); returns the partition number the mode database assigns to that brick (`get_partid_for_brick`; the library's -1 for an unlisted brick is clamped to 0 by the wrapper at `0x10002054`, so the reply is 0 for unlisted and for unpartitioned); reply: result 0 and one integer | not sent by `partmgr` |
| 2 | `0x10001434` | argument: a partition number; copies that partition's environment-variable block (`get_env_for_part`, into a stack buffer) and replies with an integer length and the block | `PART_GET_VARS` |
| 3 | `0x10001594` | no argument; copies the whole mode database (`sub_10002150`, from the working buffer at `0x10013100`; it first builds the default database if none is loaded) and replies with result 0 and one binary argument holding it | `PART_GET_DB` |
| 4 | `0x10001524` | no argument; replies with result 0 and one integer, the database timestamp, the word at +0xc of its header | `PART_GET_DB_TIME` |
| 5 | `0x1000160c` | one binary argument, a complete database image; `sub_100021c8` validates it (header checks below), installs it, re-unpacks it (`unpack_mode_db`, falling back to a rebuild from flash on failure) and writes it to flash; when the request came from a local client the handler then also sends the new database as an event (`sub_100026d0`, which calls `sn1SCEvent(5, 1, record)` with the database as a binary argument); a nonzero validation result is returned negated, except that an older timestamp returns 7 positive | `PART_SET_DB` |
| 6 | `0x100016f4` | an integer and a 24-byte header (argument sizes read from the descriptor halfwords); `sub_10002464` requires the integer to equal the current timestamp (else -1), the length to be exactly 0x18 (-2), and the header's magic, size, checksum and version to equal the current ones (-3 to -6); if the word at 0x10 is also equal nothing changes (result 0), otherwise that header becomes the current one, the database is re-unpacked and written to flash. So this request changes only the header word at 0x10, which is the identifier of the selected mode (established in [libpart.md](libpart.md): in the daemon the unpacker keeps the mode whose identifier byte equals it) | `PART_SET_MODE` |
| 7 | `0x100017bc` | integer slot 0 to 63 and the request packet itself: parks the packet in a 64-entry table at `0x10023130`, freeing any packet already there, and sends no reply now (a local reply-enable flag is cleared when the slot was valid; an out-of-range slot returns 1) | not sent by `partmgr` |
| 8 | `0x10001870` | integer slot and a string: finds the packet parked in that slot (`sub_10001c4c`), and if there is one sends it a reply carrying the string (length plus one) and empties the slot; otherwise handles the request as a broadcast (`sub_10001cb4`, which only prints `Broadcasting request %d`) | not sent by `partmgr` |

Grades: the handler addresses and the actions are established (the code was read). The `partmgr` names are established as strings next to the codes it sends (`li r4, N` before each `sn1SCCmd`: 4, 3, 5, 6, 2 for time, DB, set-DB, set-mode, vars). The "parked packet" reading of codes 7 and 8 as a wait-for-notification mechanism is inferred.

Common exit (`0x10001aa0`): if the incoming packet's class byte (offset 0x16) is 0 (a request) and the reply-enable flag is set, the handler sends the reply with `sn1SCSendRsp(0, request, count, reply)`. The reply record built on the stack holds: bytes at 0x18 and 0x19 (zero), a result word at 0x1c, a halfword at 0x1a (zero), then argument descriptors: a type byte at 0x20 (0 integer, 1 binary, 2 string) and a length halfword at 0x22 with the value or pointer at 0x24; a second descriptor follows at 0x28 to 0x2c for code 2. `count` is one plus the number of arguments: 2 for codes 1, 3 and 4, 3 for code 2, and 1 for codes 5 and 6 (they never overwrite the default; corrected, see [userspace.md](userspace.md#9-the-reply-to-the-partition-record-requests)). The first integer of the code-2 reply is the number of variables, and the binary argument's length halfword is the byte length of the block. After a code-6 request the handler compares the database with the one in flash (`compare_modes`) and, if different, runs `sub_10001d10`, which is an empty routine in 1.44.0 (a hook).

## The database record

`libpart` (`unpack_mode_db`, `get_cksum`, `create_default_db`, ...) and `l2part` agree on a record with a 0x18-byte header, established from `sub_100021c8` (the validation, `0x100021c8`) and `unpack_mode_db`:

| Offset | Meaning | Check made | Grade |
|---|---|---|---|
| 0x00 | magic `0x5a4c4a21` (ASCII `ZLJ!`) | must match, else result -2 | established |
| 0x04 | total size in bytes | 0x18 to 0x10000, else -3; larger than the buffer -4 | established |
| 0x08 | checksum of the bytes from 0x18 for (size minus 0x18) | must match `get_cksum`, else -5 | established |
| 0x0c | timestamp (seconds) | a new image older than the current one is refused with result 7 (`part_set_db: New DB timestamp (%d) < Current DB timestamp (%d)`); equal means nothing to do (result 0) | established |
| 0x10 | identifier of the selected mode, the word that `PART_SET_MODE` alone changes | must equal the current one only in the no-change path | established |
| 0x14 | version | must be exactly 1: 0 and values above 1 give -6; `unpack_mode_db` applies the same test (message `database version mismatch`) | established |

The checksum (`chksum` at `0x5944` in `libpart`) is the 32-bit CRC with polynomial `0x04c11db7` (table at `0x15ca4`, first words `0, 0x04c11db7, 0x09823b6e, 0x0d4326d9`), processed most significant bit first with the length folded in a byte at a time and the result inverted. That it is the POSIX `cksum` algorithm is established by reading `chksum` in full (see [libpart.md](libpart.md)). The body from 0x18 is decoded in [libpart.md](libpart.md): layouts (brick to partition tables), environment blobs and modes (16-character names, each naming a layout and choosing a blob per partition). `partmgr` lists the grammar it accepts (`MODE`, `UNPARTITIONED`, `PARTITION`, `BRICKS`, `RACKS`, `ENVIRON`, "environment too large (maximum %d bytes)").

Storage: the database lives in the flash at `0xfe3e0000` for 64 KiB, read with `flash_read` at start-up. `sub_10001d2c` reads it, builds a default if the magic is wrong, the size is outside 0x18 to 0x10000, or `unpack_mode_db` fails (the three messages `Invalid database size detected in flash--initializing`, `Invalid database detected in flash--initializing`), and `sub_10001f3c` writes it back by `flash_erase` of the 64 KiB sector run and `flash_program` (`part_db_flush: error erasing / programming part database`). A copy is cached in RAM; `partmgr` keeps `/tmp/.partdb_cache`.

## The chunked frame

Packets that travel over a byte stream, as the router's descriptor-based ("OS") connections do, carry one frame per `write` and `read`. The writer is `sub_5474` and the reader `sub_56d8` in `libIRouter` (established by reading both in full).

Frame (writer, `0x5474`-`0x5668`): a 9-byte header and up to 0x76 (118) payload bytes, so a frame is at most 127 bytes and fits the reader's 128-byte read:

| Byte | Value |
|---|---|
| 0 to 3 | zero |
| 4, 5 | the code, big-endian: `0x3001`, `0x3002` or `0x3003` |
| 6 | for a first frame sent as `0x3001`, the low 7 bits of the byte at offset 5 of the packet data; zero for `0x3002` and `0x3003` |
| 7, 8 | the total length of the whole message, big-endian, in the first frame; zero in later frames |
| 9 on | payload |

Selection of the first code: the writer examines bit 7 of the byte at offset 5 of the packet data; if set the first frame is `0x3001`, otherwise `0x3002`. Every following frame of the same message is `0x3003` with bytes 6 to 8 cleared. The packet is walked 0x76 bytes at a time; the last frame carries the remainder; the routine returns the original length.

The reader (`0x56d8`) issues one `read` of up to 0x80 bytes per frame (`sub_5668`), decodes the code at bytes 4 and 5, the length at 7 and 8, and a data length of the bytes read minus 9. Codes below `0x3001` or above `0x3003` fail with result -0x272 (`bad chunk type %d`). `0x3001` and `0x3002` start a message: if a message is still incomplete the result is -0x271 (`frame data underflow`); if the declared total exceeds the caller's buffer, -0x270 (`frame data overflow`). `0x3003` continues. Each frame's payload is copied after the previous one; a payload longer than the bytes still expected is -0x270; when the expected count reaches zero the routine returns the total length. The two start codes are treated identically by the reader; the writer's choice between them follows the flag bit above (what that bit means is not known).

So a partition-database reply (code 3, up to 65,536 bytes) travels as one router response packet, and on a stream link it is cut into `0x3001` or `0x3002` plus as many `0x3003` frames as the length needs, 118 bytes each; the 16-bit total length in the first frame cannot express 65,536, and the 16-bit length descriptor in the reply record (`lhz 0x6a`) wraps the same way, so a database of exactly 64 KiB would be mis-framed (inferred from the field widths; the default database is far smaller).

## Open

- The PROM side of the fetch (which code it sends) is not in the images read; [partition-record.md](partition-record.md) infers code 1 from the request shape and the reply checks the firmware's published reading records. Route tried: the L1 package's chunk assembler and the L2 handler above. Next: the L1 requests that forward to task 10 (task 5 forwarding in [../l1/rs422-router.md](../l1/rs422-router.md)) and any IP35 PROM image that reaches this tree.
- Whether `0x3001` versus `0x3002` carries a meaning to the L1 (the L1 assembler's treatment is in the L1 notes).
