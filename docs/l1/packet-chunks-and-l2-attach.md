# Origin 300 L1 1.44.0: chunked requests, L2 attach and request 0x0ffff00e

Evidence for the L1 hardware contract sheet. Everything here is read from the 1.44.0
package 1 body described in [README.md](README.md) (`Fuel/PE/O300 1MB`, loaded at
`0xffe00000`, first flash bank). Addresses are first-bank; the listing is
[disasm/mcf5206e-1.44.0-fuel-pe-o300/l1.lst](disasm/mcf5206e-1.44.0-fuel-pe-o300/l1.lst),
with per-routine calls in `l1.sum` beside it. Grades follow [AGENTS.md](../../AGENTS.md).
Routine names are our own descriptions, listed in [functions.tsv](functions.tsv).

Two terms used below. A **chunk frame** is one Bedrock-link frame carrying a piece of a
larger request under the pseudo-codes `0x3001` to `0x3003`. A **subchannel entry** is one
of the 0x14-byte reassembly records of a link port (see 1.1).

## 1. Chunked requests: from frames to a router packet

**1.1 Frame and reassembly-state layout (established, `chunk_assembler` at `0xffe3519c`,
read in full).** The first argument is the port index, the second the frame pointer, the
third the payload length. The routine computes the link record as the port table pointer read
from RAM word `0x3000504c` plus port index times `0x584`; the subchannel array is at that
record's `+0x2f8`. The frame fields it reads:

- byte 0, low five bits: subchannel index into that array;
- bytes 5 and 6, big-endian 16-bit: the code (`0x3001`, `0x3002`, `0x3003`, else an error);
- byte 7: the frame count, the number of buffers the message will use;
- bytes 8 and 9, big-endian 16-bit: the total message length;
- byte 10 onward: payload.

Each subchannel entry is 0x14 bytes: `+0x0` head buffer pointer, `+0x4` write position,
`+0x8` bytes still expected, `+0xc` tail buffer while the chain is built and the buffer being
filled after that, `+0x10` frames still expected. Each buffer is 0x410 bytes, zeroed on
allocation; its first word is the link to the next buffer; the packet bytes start at buffer
`+0x10` (the write position starts there).

**1.2 Code 0x3001, first frame (established, `0xffe35258`-`0xffe352f4`).** If the entry's
head pointer is already set, the routine logs `incomplete frame`, sends an error reply
with detail 23 (reply word `0xf9a9`, minus 1623) through `response_send`, and frees the
old chain. It then allocates one 0x410-byte buffer for each unit of the frame count (loop
decrements the count in the entry and stops at zero), writes the count back to `+0x10`,
and clears `+0xc`. The call at `0xffe352fe` to `0xffe7d268` (zero and 20 as arguments) is
not established as a slot clear: its target is in the body of a copy routine, so it stays
unsettled (see Open). It then continues into the 0x3002 path (1.3).

**1.3 Code 0x3002, next buffer (established, `0xffe3537a`-`0xffe35416`).** Requires an open
message (`+0x0` nonzero; else `bad chunk`, no reply). Bytes still expected (`+0x8`) must be
zero, else reply detail 25 (`0xf9a7`, minus 1625, `frame data underflow`). Frames still
expected (`+0x10`) must be nonzero, else reply detail 24 (`0xf9a8`, minus 1624, `frame count
overflow`). Otherwise it moves to the next buffer, decrements `+0x10` by one, sets `+0x8`
to the total length from bytes 8 and 9, and falls into the payload copy (1.4). A 0x3001
frame therefore consumes one buffer of the count it announced.

**1.4 Code 0x3003, continuation (established, `0xffe3541a`-`0xffe35528`).** Does not change
`+0x10`; it appends to the current buffer. Requires an open message (else silent drop, as
1.3). If the payload is longer than `+0x8`, the routine replies detail 24 (`frame data
overflow`, reply word `0xf9a8`) and drops the message. Otherwise it copies the payload,
advances `+0x4` by the payload length and subtracts it from `+0x8`.

**1.5 Unknown code (established, `0xffe354fc`-`0xffe3551c`).** A code outside `0x3001` to
`0x3003` logs `bad chunk`. If an open message exists it replies detail 26 (`0xfffff9a6`,
minus 1626) and drops it; with no open message it returns silently.

**1.6 Completion and hand-off to the router (established, `0xffe3556c`-`0xffe35626`).** After
each accepted frame, the routine returns unless both `+0x8` and `+0x10` are zero. When both
are zero it reads the class byte at head buffer `+0x16` and writes the source word into head
buffer `+0x1c`. Class 0 and class 1 store the whole word. Class 2
stores the new high bits and keeps the low five bits already in the word. Other classes
store nothing. It then calls `packet_receive_entry` with the link record `0x30005038` as the
first argument and the head buffer as the packet, and clears the entry. The router then
routes the packet by its own destination word, not by the chunk's subchannel.

**1.7 Source word and the task 21 return (established for the value, inferred for the
intent).** The base word is the first word at `0x30005038`. If bit 31 of that word is clear,
the source word is `(base & 0xffffffe0) | 0x15`; if set, `(base & 0xc0ffffff) | 0x15000000`.
Bits 11 to 5 then carry a 7-bit tag from the tag routine (`0xffe349a8`) for the frame's
subchannel. Low five bits `0x15` are task 21. The reply path (1.8) reads the same tag back
from a reply's destination word, which is how a reply finds its subchannel again. The
intent (that task 21 is the L1's own return address for chunked requests) is inferred from
the match between this word and the reply routine.

**1.8 Reply path (established for the header, the chunk size and the continuation codes,
`outbound_chunked_response` at `0xffe35a18`).** A reply packet's destination word is
read at `+0x18`. If its bit 31 is clear, bits 11 to 5 select the subchannel; if set, no
subchannel is chosen and the tag routine receives -1. The first reply frame is written as:
byte 0, subchannel index with `0x20` set; bytes 1 to 4 zero; bytes 5 and 6 `0x3001`; byte 7,
the packet's segment counter (`+0x15`, low seven bits); bytes 8 and 9, the packet's 16-bit
length word at `+0x10`. Payload is taken from packet `+0x10` onward. The frame payload cap is
0x71 (113) bytes (established: the compare at `0xffe35b22`-`0xffe35b24`). A remaining length of
0x71 or less is copied as the final frame (`0xffe35b44`-`0xffe35b4e`); otherwise 113 bytes are
sent. Byte 6 of a continuation frame is 0x03 (code `0x3003`) while data remains (`0xffe35bd6`)
and 0x02 (code `0x3002`) when the buffer is exhausted (`0xffe35bf0`). Each frame goes to the
link's write queue (established: the routine calls `ppp_frame_send`).

**1.9 Timeout (established, negative).** None of the routines in 1.1 to 1.6 calls a timer,
delay or sleep; their call lists contain only logging, buffer and router calls. A partial
message is removed only by a new `0x3001` (1.2), by a protocol error (1.3 to 1.5) or by
completion (1.6). Whether the link tasks time out a partial message elsewhere was not read.
Unknown.

## 2. What an L1 does when an L2 attaches

**2.1 Boot-time auto power-up task (established, `0xffe2ff88`, called at `0xffe1075e`).** The
routine sets up an endpoint record, creates a queue, registers the queue for its address
(`endpoint_register_queue`), and creates a task (firmware name `APWR_TSK`, entry `0xffe30130`).
The call at `0xffe1075e` is reached from two branches of the boot code (`0xffe10744` and the
branch at `0xffe1071e`), so the task is created whether or not an L2 is present.

**2.2 Abort on L2 detection (established for the log texts and call order; inferred for the
test).** The task entry logs `* power up`, then `auto power up countdown initiated`. Later it
calls `module_info_by_port` (`0xffe31f00`), compares strings with `string_compare`
(`0xffe7d32c`), and logs `auto power up aborted, L2 detected (%d)` when that scan finds an L2.
The string it tests against was not identified. The exact string it tests against was not identified. Other
abort texts in the same task: `user abort` and `nvram changed`.

**2.3 Control transfer and the unprompted request (established for the texts and calls,
inferred for the conditions).** When the scan does not abort, the task logs `auto power up
control transfered to %s` and, after setting a transfer target through `0xffe30a50`, logs
`auto power up initiated` and `powering up the system`. It then builds a request with
`packet_build_wrapper` (`0xffe79048`), injects it with `packet_receive_entry` (`0xffe78eb8`),
and calls `queue_receive` (`0xffe783f0`) with -1 loaded just before the call (the wait value
it means was not confirmed). Errors log
`auto power up packet error` or `auto power up error`. This request is the unprompted traffic
of the power-up path, sent before any L2 traffic is seen. The meaning of -1 as the wait
argument was not confirmed.

**2.4 Event-sink registration (established for the call shape).** `config_exchange_init`
(`0xffe318b2`-`0xffe3191a`) calls the wrapper `sub_ffe77bf8` with the configuration record,
mask `-1`, flag `0` and a callback. The wrapper forwards to `event_subscribe` (`0xffe77a24`).
The wrapper `sub_ffe77c1c` is called at `0xffe2d30c`, `0xffe35ffe` (inside the link write
task) and `0xffe3802e` (in the routine at `0xffe37f60`); `sub_ffe77bf8` is also called at
`0xffe2e234` and `0xffe2e2a8`. The two wrappers differ only in the fourth argument passed to
`event_subscribe` (0 and 1). What an event subscription delivers is covered by `event_send` (`0xffe78f34`) and
`event_deliver` (`0xffe78a6c`), which were not read in full.

**2.5 Event subchannel (established).** Task 6 request `0x1007` sets the event subchannel and
persists it; `0x100a` clears it (functions table, `0xffe12fd8` and `0xffe131b2`). The
persisted value is setting `0x13`. Only task 4 events go out on that subchannel
(`outbound_event_frame`, `0xffe34e84`). A routine at `0xffe1434c` prints `system event
subchannel %d`.

**2.6 Console mode and routing (established for the texts; inferred for the absence of a
routing change).** The management port prints `entering console mode %s, <CTRL_T> to escape
to L1` and `returning to console mode %s` at `0xffe37b42` and `0xffe37c36`, in the routine headed
`0xffe378ac`, which the summary lists just before `management_port_task` (`0xffe37de4`).
Nothing in the attach path changes the router table: the abort and transfer routines in 2.2
and 2.3 call endpoint registration and the packet builder only (inferred from their call
lists). Type-4 requests go to peer slot 9 by the brick-type setting, not by any attach
event ([rs422-router.md](rs422-router.md)). The router-type string `router type changed,
effective on power up or reset` (`0xffe1a0a6`) says a routing change waits for a restart.

## 3. Request 0x0ffff00e, task 14, code 0

**3.1 Registration (established).** `config_exchange_init` (`0xffe318b2`-`0xffe318de`) calls
`endpoint_init` on the record at `0x30004f88` with task 14 and then `endpoint_register_callback`
with the address `0x0ffff00e`, flags `3` and handler `0xffe33ce4`. Flag bit 0 registers the
request slot and bit 1 the response slot (`endpoint_register_callback`, `0xffe77960`).

**3.2 Dispatch (established).** `config_exchange_handler` (`0xffe33ce4`) passes requests to
`config_exchange_request` (`0xffe325ec`) at `0xffe33d36`. Responses are dispatched on the
argument value 0 to 15 through the table at `0xffe33dcc`.

**3.3 Selector (inferred from the call shape).** `config_exchange_request` reads a selector
from the request. It is the longword at packet `+0x28` when the descriptor type bits at
`+0x24` are zero, and the first argument value (argument 0) otherwise. Selectors 0, 7 and 8
(and any other entry whose word in the table at `0xffe3264a` is `0x0570`) and any selector
above 25 go to a shared path at `0xffe32bba`. That path takes a second selector from packet
`+0xc` and dispatches 0 to 12 (table `0xffe32bd0`). The meanings of the second-level cases
were not read.

**3.4 Selector 6 (established for each step, inferred for its meaning).** The first-level
table at `0xffe3264a` sends selector 6 to `0xffe3267e`. The path needs at least two arguments
(packet argument count at `+0x20`; one or less goes to the common tail at `0xffe32702`). It
takes argument 1 (the second argument) as a string, or the longword at `+0x30` when the
descriptor type bits at `+0x2c` are zero. It compares that string with the text at `+0x64`
of the first 0x70-byte entry of the table whose pointer is stored at `0x30000aa4`
(`string_compare`). On a difference it copies the new text there (`string_copy`) and sets a
flag. It then calls `0xffe2fb08` with `0x8000`. Last it counts the entries of that table, a
count word at `0x30000aa8`, whose word at `+0x60` is nonzero. The text is the system serial
number (inferred: the L2's request carries the serial as its second argument, see
[userspace.md](../l2/userspace.md)).

**3.5 Reply (established for selector 6 with three arguments).** The tail was read in
full: the reply is class 1, status 0, to the request's source word, with the integer 7, the
request's third argument, a count `N`, the record size 0x70, a byte array of `N` table
indices and `N` 0x70-byte table records (`0xffe32702` to `0xffe32c40`; the field-by-field
account and the daemon's use of it are in
[../l2/config-request-reply.md](../l2/config-request-reply.md) section 3).

## Open

- The writer of the 0x70-byte table behind the reply of 3.5 (pointer at `0x30000aa4`), and
  the consumer of the two requests the L1 sends before it builds that reply.
- The string compared in the L2 scan of 2.2, and the meaning of `-1` in 2.3.
- Whether the link tasks time out a partial message outside 1.1 to 1.6 (1.9).
- Whether the call at `0xffe352fe` to `0xffe7d268` clears the start-frame slot (1.2). It is
  not established: the target is inside a copy routine that starts at `0xffe7d214`.
