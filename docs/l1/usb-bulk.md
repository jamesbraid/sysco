# Origin 300 L1 1.44.0: USB bulk packet stream

It describes bytes passed to the L1's management USB bulk callbacks and the software paths around them. The descriptor reading identifies one declared interface with bulk OUT endpoint 1 and bulk IN endpoint 2, both with 64-byte maximum packets. The management USB reading separates this device port from the IO-8 PCI OHCI guest ports.

## Evidence and method

The traced code: the first-bank packet allocator and builder at `0xffe77e10`– `0xffe782a2`, stream reader and writer at `0xffe790a8`–`0xffe79342`, USB receive/write wrappers at `0xffe2d85c`–`0xffe2d922`, and lower device read/write entry points at `0xffe6e220`–`0xffe6e3de` and `0xffe6d7c4`–`0xffe6d8b6`. Each of these routines has a copy exactly `0x70000` bytes above its first-bank start. Twenty selected instructions covering segment allocation, offsets, marker masks, count decrement and callback arguments have the same encoding in both copies.

USB 1.0 assigns configuration-attributes bit 7 to bus power, bit 6 to self power and bit 5 to remote wakeup. The selected image declares USB 1.00 and sets bits 6 and 5 while clearing bit 7 (descriptor reading). That is a self-powered/remote-wakeup declaration under revision 1.0. It does not establish actual board power, later-revision compliance or L2 host acceptance.

## Segment layout passed to the bulk callbacks

The sender passes each linked packet segment starting at buffer offset `0x10` and uses a big-endian 16-bit length at that offset as the callback byte count (`0xffe792c8`–`0xffe792e4`). The first 16 bytes of its allocation hold internal links and reference state. They are not submitted to the USB write callback. ColdFire word and longword stores, followed by a raw buffer write without a software endian conversion, make the multi-byte fields big-endian at this callback boundary (`0xffe77f7a`–`0xffe77fba`, `0xffe2d8c0`–`0xffe2d922`).

Offsets below are relative to the callback's first byte. They are a field map from builder stores and receiver reads, not an extracted packet transcript.

| Offset | Width | Selected-software use |
|---|---:|---|
| `0x00` | 2 | segment byte count, including this field and the rest of the segment |
| `0x02` | 1 | builder sets a format discriminator to 2. The receive stream does not check it |
| `0x03` | 1 | builder-supplied flag whose complete meaning is unresolved |
| `0x04` | 1 | low byte of a router transaction number |
| `0x05` | 1 | first-segment flag in bit 7 and remaining-segment count in bits 6:0 |
| `0x06` | 1 | packet class: request 0, response 1 or event 2 |
| `0x07` | 1 | builder-supplied source-port/subchannel field. Its full external meaning is unresolved |
| `0x08` | 4 | destination router address |
| `0x0c` | 4 | source router address |
| `0x10` | 1 | argument-descriptor count, copied into continuations |
| `0x11` | 1 | high byte of the transaction number |
| `0x12` | 2 | router request or event code |
| `0x14` | variable | first-segment argument descriptors, eight bytes each, followed by inline argument data |

The builder's minimum header is 20 callback bytes. It accepts at most 64 argument descriptors and adds eight bytes per descriptor before copying indirect argument data (`0xffe77ef0`–`0xffe77f42`, `0xffe77fc2`–`0xffe77fe0`). Its segment allocator caps a segment at 1024 callback bytes and fails if 128 or more linked segments would be needed (`0xffe77f42`–`0xffe77f58`, `0xffe78244`–`0xffe78298`). The USB stream reader allocates 272 bytes, reserves the first 16 for internal state, and requests 256 bytes per receive callback (`0xffe790d0`–`0xffe79122`). A lower-layer overflow path invokes its registered reallocation callback, then resumes a read with an additional 64-byte offset. The callback's exact resulting capacity is unresolved (`0xffe6e220`–`0xffe6e3d4`). I did not establish that an L2 uses 1024-byte segments or how one such segment would be delivered to a 256-byte reader. Treat the 1024/256 difference as an unresolved interoperability condition, not a claim that a real transfer is truncated.

For a packet of `N` linked segments, where `1 <= N <= 127`, the builder puts the first flag and `N` in the first segment's count byte. Later segments carry `N-1`, `N-2`, down to `1`, without the first flag (`0xffe78238`– `0xffe78298`). Continuation segments copy the 20-byte common header from the first segment before updating their own length and count. The reader requires the first flag on its first positive callback result, then compares only the low seven count bits on continuations. It does not separately reject a first flag set on a continuation (`0xffe791b8`–`0xffe79296`). A zero count in the first segment does not complete a packet: its decrement underflows and the next count comparison fails. The reader does not visibly compare a positive callback byte count against the segment's embedded length or the minimum six bytes needed to reach the count marker before inspecting the buffer.

This is the router's linked-segment stream, not Bedrock UART framing. No PPP flag, escape or frame-check conversion occurs in this USB callback path.

## Buffering, errors and reconnect

The stream reader allocates a new segment on every iteration. An allocation failure frees the accumulated packet and returns a resource error. A receive callback returning zero is an error, not an empty router segment. A negative result is mapped to a transport or resource error. If a request-class first segment has already been assembled, that callback failure also tries to send an error response before freeing it (`0xffe790d0`–`0xffe791b4`). Missing first flag or a wrong continuation count frees the packet and returns a malformed- segment error without that response attempt (`0xffe791b8`–`0xffe7924a`). Successful completion marks the reconstructed head as one owned router packet and passes it to normal address routing (`0xffe79282`–`0xffe792a6`, `0xffe2d3b4`–`0xffe2d49a`). The router can reject an unknown class or unregistered destination. The stream parser does not prove successful task delivery.

The stream writer submits each linked segment as one lower-layer write call with the embedded byte count. It advances only when the callback reports exactly that count. A zero, short or negative result frees the whole queued packet and returns an error. A negative result for a request-class packet also tries a response error (`0xffe792a8`–`0xffe79342`). The ordinary builder never creates a zero-length segment, and this writer has no separate explicit zero-length USB packet call. Any controller-generated zero-length packet, USB short-packet termination, endpoint retry or host transfer grouping is unverified. The USB 1.0 specification describes USB-level bulk transfers.

Negative lower-layer writes increment a shared counter. Once it reaches five, the wrapper skips later lower-layer calls while returning the requested byte count unchanged (`0xffe2d8c0`–`0xffe2d922`). The stream writer consequently treats that return as complete and frees the packet. A nonnegative bulk read resets the counter (`0xffe2d85c`–`0xffe2d8be`). It is silent software discard, not evidence of a successful USB transfer. A short positive lower-layer write does not increment the counter, yet the stream writer still treats it as an incomplete segment.

The management USB write task chooses an open function from the packet's recorded arrival port or the current event destination. Without an open destination it frees the queued packet (`0xffe2d73a`–`0xffe2d844`). The receive task unregisters its catch-all conditionally, closes a failed function, waits and attempts to reopen it. A second configured function may become the event destination (corrected route reading). The unregister comparison argument is zero and can act as a wildcard, so a later route with the same key is not guaranteed to survive withdrawal. These paths do not show replay of already freed or pending packets.

## Standard control transfers

The device control task expects eight setup bytes and converts `wValue`, `wIndex` and `wLength` from USB little-endian order before dispatching (`0xffe6fb6c`–`0xffe6fd1e`). The C-like export timed out for this routine, so this step is from the instruction listing and the control-task reading. The descriptor read path bounds its response by both the descriptor's declared length and the host's `wLength`, writes data in chunks of at most 64 bytes, and uses a zero-length data write when an exactly full chunk ends a shorter-than- requested response (`0xffe6f644`–`0xffe6f6c6`, `0xffe6f820`– `0xffe6f920`). It then requests a zero-length OUT status stage (`0xffe6fa78`–`0xffe6fb6a`). A SET_ADDRESS request applies a first address step, sends a zero-length IN status stage, then applies the final address step only after that succeeds (`0xffe6f920`–`0xffe6fa76`). Its C-like export also timed out, and the control reading gives the instruction-level recipient branches.

The read dispatcher names GET_CONFIGURATION, GET_INTERFACE and SYNCH_FRAME but constructs no visible data payload for them before its status-stage call (`0xffe6fa78`–`0xffe6fb6a`). USB 1.0 Table 9-2 expects one-byte configuration and alternate-interface responses and a two-byte frame number for these requests. The write dispatcher similarly has no visible state change for SET_DESCRIPTOR or SET_INTERFACE before its zero-length status-stage call (`0xffe6f920`–`0xffe6fa76`). Short setup reads and control errors return to the setup loop rather than creating a successful reply, as the control-task reading reports.

## Open points and software routes

| Unknown | Route |
|---|---|
| Callback byte boundaries and size compatibility with the L2 side | Read the L2 image's USB packet code (the PowerPC L2 cramfs daemon and kernel driver; unread, see open questions) and compare its framing with the builder at `0xffe77e10` and the reader at `0xffe790a8`. The public host driver in [usb-public-host-drivers.md](usb-public-host-drivers.md) shows the host framing it expects. |
| Behaviour after a segment whose length is a multiple of 64 | Read the lower write routine (`0xffe6d7c4`) for a zero-length transfer after a full packet, and the USS-820-family datasheet behaviour for a short or empty packet. |
| Five-error discard and replay | Established for the L1: the wrapper at `0xffe2d8c0` skips lower calls after five negative writes and reports the requested count. Unknown: the L2 side's replay. Route: the L2 daemon's retransmit logic. |
| Control compatibility for requests the dispatcher leaves empty | Read the L2 kernel's USB enumeration (Linux 2.4.7, in the L2 image) for which of these requests it issues. |
