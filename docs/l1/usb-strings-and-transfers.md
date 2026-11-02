# Origin 300 L1 1.44.0: USB string descriptors and the one-segment-per-transfer rule

Read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`), body loaded at `0xffe00000` (the package carries the program twice, `0x70000` apart; the string bytes below are identical in both copies), and from the L2 1.44.0 files cited in [../l2/userspace.md](../l2/userspace.md). Addresses are first-bank. Grades as in [../../AGENTS.md](../../AGENTS.md). The descriptor fields and control path are in [usb-descriptor-fields.md](usb-descriptor-fields.md), [usb-control.md](usb-control.md) and [usb-descriptors-routing.md](usb-descriptors-routing.md); the segment layout is in [usb-bulk.md](usb-bulk.md). This note supplies the string bytes that those notes withheld (short literals only) and settles the transfer rules they left open.

All of this is the revision path in which the board's USB controller probe takes the visible callbacks. The `0x81` revision path calls a different RAM pointer ([usb-lower-controller.md](usb-lower-controller.md)); its behaviour is not read.

## String descriptors

The device descriptor (`0xffe6fd20`) names three strings: manufacturer index 1, product 2, serial number 3. The configuration descriptor (`0xffe6fd32`) names string 4 (its sixth byte) and its interface descriptor string 5. The string selector (`0xffe6f158`, table `0xffe6fe94`, read in full) picks one of two six-entry pointer arrays by the setup packet's `wIndex` (the language field), and the entry by the low byte of the descriptor index:

| `wIndex` (language) | Array (ROM copy at body offset `0x7f790` / `0x7f7a8`; run-time RAM address) | Entries |
|---|---|---|
| `0x0009` | `0x300006ac` | set A below |
| `0x0000` | `0x300006c4` | set B below |
| anything else | none | no descriptor |

Index 6 or more, or any other language, returns no descriptor: the selector returns null and the GET_DESCRIPTOR handler exits with -1 without sending data (`0xffe6f8ba` to `0xffe6f914`; what the control task then does on the status stage was not read). Established for the selector and its error path. The RAM addresses are inferred: the table holds them, the ROM image holds the same two six-pointer arrays at the given offsets (`0x18` apart as the RAM addresses are), and no instruction in the body names those RAM addresses, so a start-up copy of the data section is assumed and was not read; no code writes the arrays (no absolute reference in the listing). The descriptors themselves are in ROM at `0xffe6fd52` to `0xffe6fe92`; the descriptor sent is the object the pointer names and its length is its own first byte, clipped to the host's `wLength` and sent in packets of at most 64 bytes ([usb-bulk.md](usb-bulk.md)). Established.

Each entry is a standard string descriptor: byte 0 total length, byte 1 type 3, then UTF-16 little-endian text with no terminator.

**Set A (language `0x0009`)**

| Index | `bLength` | Characters | Text (paraphrase) |
|---|---|---|---|
| 0 | 4 | | language list: one ID, `0x0009` (bytes `04 03 09 00`) |
| 1 | 46 | 22 | the manufacturer: the company name "Silicon Graphics, Inc." |
| 2 | 50 | 24 | the product: "SN1 L1 System Controller" |
| 3 | 18 | 8 | the serial number: eight ASCII zeros, the same in every image (a constant in ROM, not per board) |
| 4 | 56 | 27 | the configuration: "USB Bulk Data Configuration" |
| 5 | 40 | 19 | the interface: "Bulk Data Interface" |

Established (bytes read; the three device-level strings agree with what an L2's log printed for a real L1, [usb-l2-revision.md](usb-l2-revision.md)). The language ID is `0x0009`, the primary language English with no sublanguage, not `0x0409`.

**Set B (language `0x0000`)**

| Index | `bLength` | Characters | Text |
|---|---|---|---|
| 0 | 4 | | the same list, `04 03 09 00` |
| 1 | 20 | 9 | the placeholder `VENDOR_ID` |
| 2 | 16 | 7 | `PRODUCT` |
| 3 | 26 | 12 | `SERIALNUMBER` |
| 4 | 22 | 10 | `CONFIG1_ID` |
| 5 | 18 | 8 | `BULKDATA` |

Established (bytes). These read like a controller vendor's sample strings left in place. They are reached only by a host that asks for an index of 1 or more with `wIndex` 0. A host that follows the usual order asks for index 0 first (any `wIndex`, answered from set B with the list above), learns language `0x0009`, and then asks set A. The Linux 2.4 USB core does so, so an L2 sees set A (inferred from the host's usual behaviour; the L2 kernel's string code was not read, and the L2's L1 match routine does not check strings at all, [../l2/userspace.md](../l2/userspace.md)).

Discriminating checks for a USB device model: (1) index 0 with `wIndex` 0 and with `wIndex` `0x0009` both give `04 03 09 00`; (2) indices 1 to 3 with `wIndex` `0x0009` give lengths 46, 50, 18 and the serial text is all zeros; (3) the same indices with `wIndex` 0 give lengths 20, 16, 26; (4) index 6 or `wIndex` `0x0409` gives no data (and a model must not answer with set A).

## One bulk transfer, one segment

### Host to L1 (bulk OUT endpoint 1)

The bulk-OUT worker is the routine at `0xffe70fe8` (reached through operation 12 of the controller callback at `0xffe71454`, `0xffe71524` to `0xffe71154`), read in full. The receive helper `0xffe6e220` starts one such receive with the caller's buffer capacity and waits for its completion. Behaviour:

1. It reads one packet at a time from the endpoint FIFO (count byte at controller offset `0x06`, data at `0x05`), at most 64 bytes, into the buffer.
2. A packet longer than the bytes the buffer still expects ends the receive with an overflow error (`0xfffffa12`). So a packet may not run past the end of the segment: **the next segment cannot begin inside the same packet**. A host that packs two short segments into one packet fails the first-packet test in step 3 instead (declared length below the packet size); later packets fail this one.
3. At the first packet, the big-endian halfword at the start of the data is the segment's declared length. The first packet must hold at least two bytes, and the declared length must be at least the bytes in that first packet; either failure is error `0xfffffa11` (`0xffe710a2` to `0xffe710c6`). The bytes still expected become the declared length minus the first packet (`0xffe710ce` to `0xffe710d6`).
4. A first packet shorter than 64 bytes while bytes are still expected is the same error (`0xffe710da` to `0xffe710ea`). So a segment longer than one short packet cannot be cut by a short first packet. Later packets are only subtracted from the bytes still expected (`0xffe7110e`); a short later packet is not rejected there, the worker keeps waiting for the rest. A segment is therefore ended by its declared length arriving, **not** by a short packet, and a declared length that is an exact multiple of 64 needs no zero-length packet from the host.
5. When the declared length exceeds the buffer capacity the worker stops after the first packet with status `0xfffffa10`; the helper then calls its registered reallocation callback with the declared length plus the header offset and resumes the read at an offset of 64 bytes (`0xffe6e2e8` to `0xffe6e3a6`). The stream reader's initial capacity is 256 bytes ([usb-bulk.md](usb-bulk.md)), so a segment of up to 1,024 bytes from the L2 is accepted through this growth path. Established for the shape of the path; that the reallocation callback honours the requested size was not read (inferred).
6. The count returned to the router is the declared length (halfword at `+0x9a` of the receive record). The first two bytes are the length field the L2's kernel driver stamps on every write ([../l2/userspace.md](../l2/userspace.md) section 3), so a segment as the router writes it and as the L1 receives it have the same first two bytes.

So one bulk-OUT transfer carries exactly one segment: the host side submits one write per segment ([../l2/userspace.md](../l2/userspace.md)), and the L1 side delimits a segment by its length prefix, requires it to start at a packet boundary and to end exactly at the declared length. Established for both halves.

### L1 to host (bulk IN endpoint 2)

The bulk-IN writer (`0xffe71240`, operation 16, `0xffe71334`), read in full, sends the segment in packets of at most 64 bytes: a packet's count is the lesser of 64 and the bytes left, written to the endpoint's count register after the data. When the last packet is a full 64 bytes the loop runs once more and writes a **zero-length packet** (`0xffe712f0` to `0xffe71300`; the second pass has a zero count and ends the transfer). So every segment ends in a short or empty packet. A host that reads one bulk-in transfer to completion therefore gets exactly one segment, whatever its length. Established. Each segment is one write call of the stream writer (`0xffe792a8` to `0xffe79342`, [usb-bulk.md](usb-bulk.md)) and the writer treats a count other than the segment's declared length as a failure.

The matching L2 reads (kernel driver: one completed bulk-in transfer per `read`, buffer of at least the transfer, up to 4096 bytes, [../l2/userspace.md](../l2/userspace.md) section 3) take one segment per call; the daemon's packet reader then applies the first-segment flag and down-count rule. Established on both sides.

## The L2's transmit layout toward the L1

The L2 sends the layout of [router-packet.md](../l2/router-packet.md) segment by segment: wire bytes from segment offset `0x10`, the first two bytes the segment length (stamped by the kernel to the write length, so a length the router already put there is unchanged), one `write` per segment, at most `0x400` bytes a segment for a destination without a local router form (destination top nibble 0, as for the configuration request), `0x3000` for local forms. Established ([../l2/router-packet.md](../l2/router-packet.md), [../l2/userspace.md](../l2/userspace.md)).

The one packet the daemon sends unprompted is its recurring configuration request (period from the NVRAM setting, 5 seconds after reset; [../l2/userspace.md](../l2/userspace.md) section 8). Its wire bytes for a daemon whose serial setting `S` is non-empty and `n` characters long (so the string argument is `n + 1` bytes with its NUL):

| Wire offset | Bytes | Content |
|---|---|---|
| `0x00` | 2 | `0x002d + n` (header `0x14`, three descriptors of 8, the string) |
| `0x02` | 1 | `0x02` |
| `0x04` | 1 | sequence number, low byte |
| `0x05` | 1 | `0x81` |
| `0x06` | 1 | class 0 (request) |
| `0x08` | 4 | destination `0x0ffff00e` (`0x1000d9c8` to `0x1000d9d0`, the word passed as the second pointer argument and stored at `+0x18`, i.e. wire `0x08`) |
| `0x0c` | 4 | source: the L2's local address (`0x80000000` plus id bits) |
| `0x10` | 1 | `0x03`, the descriptor count |
| `0x12` | 2 | code `0x0000` |
| `0x14` | 8 | integer descriptor, value 6 (value word at `0x18`) |
| `0x1c` | 8 | string descriptor: type 9, length `n + 1`, payload offset |
| `0x24` | 8 | integer descriptor, value 0 |
| `0x2c` | `n + 1` | the string, NUL-terminated |

With an empty serial setting the second descriptor is an integer 0 and the segment is 44 bytes. Established for the callback's stores (`0x1000da28` to `0x1000dbb4`, read in full); the fields at `0x02` to `0x07` are the packet builder's ([router-packet.md](../l2/router-packet.md)), and the exact value of byte `0x07` and of the descriptor offsets was not computed (the payload offset field holds the string's position from the segment start: `0x2c`, by the builder's rule; inferred). This agrees with the L1's own receive side: its handler for destination `0x0ffff00e` reads the selector from the first descriptor's value (6) and the serial string from the second ([../l1/packet-chunks-and-l2-attach.md](../l1/packet-chunks-and-l2-attach.md) 3.3 and 3.4); that agreement is the comparison of two reads, inferred.

For a bulk-OUT transfer of this request: 44 + `n` bytes, so the first USB packet holds 64 bytes or all of the segment, the declared length is at most the sum, and the receive finishes when that many bytes have arrived, with no zero-length packet needed.

## Unknowns

- Whether the reallocation callback in the receive helper honours the size it is given (so that a 1,024-byte segment is received whole). Route: read the callback registered with the stream reader (`0xffe790a8`).
- What the control task does after a GET_DESCRIPTOR error exit (`0xffe6f914`): stall, silence or a status stage. Route: the control task's loop after the handler returns -1 (`0xffe6fb6c` to `0xffe6fd1e`).
- Whether the start-up code copies the ROM pointer arrays to the RAM addresses named. Route: find the data-section copy in the boot path (`0xffe00400` onward).
