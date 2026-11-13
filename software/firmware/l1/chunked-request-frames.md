# Chunked request frames on the processor link (L1 side)

Reading notes for the request side of the chunk codes `0x3001` to `0x3003` in the Origin 300 L1, 1.44.0, package 1 (the body in [disasm/mcf5206e-1.44.0-fuel-pe-o300/](disasm/mcf5206e-1.44.0-fuel-pe-o300/)). Addresses are first-bank, as in [README.md](README.md).

Routines: `request_frame_parse` at `0xffe35634`, `chunk_assembler` at `0xffe3519c` (both read in full through the paths below), `response_frame_serialize` at `0xffe35c08` and `outbound_chunked_response` at `0xffe35a18` (read to the first-frame header only).

Grades as in [README.md](README.md): established means the whole path was read and the claim follows from it; inferred means a partial reading or a name; unknown means nothing read settles it.

## Frame layout and dispatch

- **Dispatch.** `request_frame_parse` at `0xffe35634` sends a frame to `chunk_assembler` only when the 16-bit code (frame bytes 5 and 6, big-endian) is `0x3001`, `0x3002` or `0x3003`. It passes the link index, the frame pointer and the frame length. Every other code takes the ordinary request path. Established: both compare chains (`0xffe35678` to `0xffe356d6` and `0xffe35254` to `0xffe35258`) were read. The header-class test that selects this parser sits in `frame_classify` at `0xffe35864`, which was not read here; its table row says header bits 7:5 = 0 is a request.
- **Bytes the chunk path reads.** Byte 0 low five bits: the slot index, 0 to 31. Byte 7: the number of segments in the message. Bytes 8 and 9: the data length of this frame, big-endian. Data starts at byte 10; its length is the frame length minus 10 (the `add.l #-10` on the length argument at `0xffe351f8`). Established.
- **Bytes the chunk path does not read.** Bytes 1 to 4 are loaded into a local word by `request_frame_parse` for the ordinary path only; the chunk call does not pass it. The chunk path therefore reads no sequence number and no source word from the frame. Established.

## Reassembly state

- **Slot table.** The link table pointer is the longword at `0x3000504c` (the same word the outbound routine reads as the identity record's `+0x14`). Each link record is `0x584` bytes. Its slot array starts at `+0x2f8`, with 20 bytes per slot. A slot holds: `+0x0` the first segment (zero means no open message), `+0x4` the write position in the current segment, `+0x8` the bytes still expected in the current frame, `+0xc` the current segment, and `+0x10` the segments still expected. Established for the offsets and stride. The field meanings are inferred from how the code uses them.
- **Segment blocks are allocated at the first frame.** A `0x3001` frame allocates one block of `0x410` bytes for each segment the byte-7 count names, chains them from the slot, and sets the count back to the byte-7 value. Established (`0xffe352f4` to `0xffe35376`). `0x3002` and `0x3003` allocate nothing.
- **Start frame (`0x3001`).** If the slot is already open, the L1 logs `incomplete frame`, sends the open message back with code `-1623` (see the error list below) and frees it (`0xffe35282` to `0xffe352ea`). Then it calls `0xffe7d268` with the slot address, zero and 20 as arguments (`0xffe352fe`). That address is in the body of a copy routine that starts at `0xffe7d214`: the body copies longwords and then bytes from its source register, so the call is not established as a slot clear. Otherwise it allocates the blocks above and falls into the segment check that `0x3002` also uses. The reply, the free and the allocation are established; the call at `0xffe7d268` is unknown.
- **Segment header (`0x3002`, and `0x3001` after allocation).** Checks, in code order: the slot must be open, or the frame is dropped with `bad chunk` and no reply (`0xffe353a6` to `0xffe353a8`); the previous segment must be complete, else `-1625`; the segment count must be nonzero, else `-1624`. Then the current segment moves to the first block (first segment) or to the next block, the write position is set to that block's data area, the bytes expected become this frame's length (bytes 8 and 9), and the segment count drops by one. The frame's own data is then copied. Established.
- **Continuation (`0x3003`).** Skips the checks above and copies its data at the write position. A continuation with no open slot is dropped with `bad chunk` and no reply. A continuation with more data than the segment still expects is answered with `-1624`. Established (`0xffe35424` to `0xffe354f8`).
- **Completion.** After each copy the bytes expected are reduced by the copied length. The message is delivered when the bytes expected are zero and the segment count is zero (`0xffe3556c` to `0xffe35574`). Otherwise the call returns and waits for more frames. Established.
- **Delivery.** The first segment's class byte (`+0x16`) decides the word at the packet's `+0x1c`. Class 0 and class 1 store the source word described below. Class 2 stores the source word with its top nibble replaced by 7, keeping the low five bits of the old word. Other classes leave `+0x1c` alone. The packet is then passed to `packet_receive_entry` with the identity word's address as the second argument, and the slot is zeroed (`0xffe355b8` to `0xffe3561c`). Established.
- **Packets written by this path.** `chunk_assembler` writes the segment links (`+0x0`, and the slot's tail pointer), the halfword `+0x4` (bit 0 set, bit 15 kept) and `+0x6` (set to 1), and `+0x1c` at delivery or error. It writes no sequence field. Established from the stores in the listing. The segment offsets match the layout the L2 router notes give for their segment record ([../l2/router-packet.md](../l2/router-packet.md)); that match is inferred, not tested against the same code.

## Source word written at delivery

- The source word is the identity longword at `0x30005038`, changed in two ways. If its bit 31 is clear, its low five bits become `0x15`, that is task 21. If bit 31 is set, bits 24 to 29 become `0x15`, which is the task field of the bit-31-set address form. Established (`0xffe35204` to `0xffe3521c`). The identity value itself is set at boot by `router_init` (`0xffe78314`) and was not read here, so the task field's final value is unknown.
- Bits 5 to 11 then take a 7-bit tag from `tag_make`, which is called with the link index and the slot index. Established for the call and the field placement (`0xffe35222` to `0xffe3523c`). The tag table behind `tag_make` was not read.
- What the router does with this word is not established here. `packet_receive_entry` at `0xffe78eb8` and `packet_deliver` at `0xffe78718` were not read. A reassembled request therefore reaches local task 21 only if `packet_deliver` routes that address to the local slot; this is inferred from the table row for `packet_deliver`.

## Errors

- `-1623` (`0xfffff9a9`): a start frame arrives while the slot is open. The open message is sent back and freed. The reply is sent through the response routine with the identity as its first argument. Established (`0xffe35288` to `0xffe352d8`).
- `-1624` (`0xfffff9a8`): the segment count is zero (`frame count overflow`), or a frame carries more data than the segment still expects (`frame data overflow`). The open message is sent back and freed. Established (`0xffe353dc`, `0xffe3546e`, `0xffe35474` to `0xffe35490`).
- `-1625` (`0xfffff9a7`): a new segment starts while the previous one still expects bytes (`frame data underflow`). The open message is sent back and freed. Established (`0xffe353be` to `0xffe353d2`, `0xffe35522`).
- `-1626` (`0xfffff9a6`): reached only from the branch for codes other than `0x3001` to `0x3003` (`0xffe354fc`). `request_frame_parse` never sends such a code, so the chunk path never returns `-1626`. Established.
- `bad chunk` with no open slot: logged, no reply, nothing freed (`0xffe3544e` to `0xffe3545c`). Established.
- The functions table gives the error details as 23 to 26 for these four codes, which matches `-1600` minus the detail (for example `-1623` is detail 23). That mapping is inferred from the table row, not from a read of the error routine.

## Replies on task 21

- **Which replies are chunked.** `response_frame_serialize` at `0xffe35c08` reads the destination word at packet `+0x18`. With bit 31 clear, the task is its low five bits; with bit 31 set, the task is bits 24 to 29. A task of 21 goes to `outbound_chunked_response` (`0xffe35c60` to `0xffe35c6a`). Established.
- **First frame.** `outbound_chunked_response` writes byte 0 as `0x20` plus the subchannel from `tag_subchannel` (the tag is bits 5 to 11 of the destination word when bit 31 is clear, or `0xff` when it is set). Bytes 1 to 4 are zero. Byte 5 is `0x30` and byte 6 is `0x01`, so the first frame's code is `0x3001`. Byte 7 is the packet's `+0x15` low seven bits (the segment count). Bytes 8 and 9 are the halfword at packet `+0x10` (the first segment's wire length). The data pointer is the packet's `+0x10`. Established for the header bytes (`0xffe35a54` to `0xffe35ac8`).
- **Later frames.** The continuation loop of `outbound_chunked_response` was not read beyond its setup (`0xffe35aea` to `0xffe35af0`). Which code the continuation frames carry, and their size, are unknown from this reading.
- **Reply path to the requester.** Whether a chunked request's reply goes back to the requester depends on the reply's destination word, which the request does not set in any way this reading established (see the source word section). Unknown.

## Timing and open slots

- **No timer in the path.** The call lists of `request_frame_parse`, `chunk_assembler`, `response_frame_serialize` and `outbound_chunked_response` contain no timer, sleep or tick routine. Established for these four routines.
- **A partial message stays open.** A slot is cleared only by delivery, by an error reply (`-1623` to `-1625`), or by a new start frame on the same slot (which replies `-1623`). Nothing in these four routines ages an open slot out. Whether another routine clears slots is unknown. The routes left are the other readers of the link record's `+0x2f8` slot array in the same image.

## Not covered

- The L1's answer to a chunked request once the L2 has it, and what the L2 does with the source word, are not read here. The L2 side of the chunk framing is in [../l2/partition-task.md](../l2/partition-task.md). Its frame carries the code at bytes 4 and 5 and the total length at 7 and 8, a different layout from the L1 frame above (code at 5 and 6, segment count at 7, data length at 8 and 9). The two framings sit on different links, and this note does not treat one as the other.
