<!-- generated from formats/l1_link_framing/format.toml by defs-gen - do not edit, change the source -->

# frame bytes and link context constants of the ColdFire L1 node and peer links

Constants of the PPP-style frame that the ColdFire L1 uses on its node link (the Bedrock UART) and on
its RS-422 peer links, and the Bedrock-side register block the node link reaches. The frame bytes and
the receive check value are shared by both links. The context values (body offset, growth limit, escape
list) belong to the peer link and the node port's setup. Offsets of the node-side register block are
from the node base in the Hub special space.

Grades: the byte values and context numbers are read from the L1 image's frame and link set-up code.
The meaning given to each one is an inference unless the item says otherwise.

Unit `l1_link_framing`, macro prefix `L1LF`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `FRAME_CHECK_RECEIVE` | 0xf0b8 | established |  | check value a receiver tests for a good frame |
| `FRAME_CHECK_SEED` | 0xffff | established |  | start value of the 16-bit frame check; sent low byte first, complemented |
| `FRAME_ESCAPE` | 0x7d | established |  | escape byte; the byte it stands for is exclusive-or'd with FRAME_ESCAPE_XOR |
| `FRAME_ESCAPE_XOR` | 0x20 | established |  | value XOR'd into an escaped byte |
| `FRAME_FLAG` | 0x7e | established |  | frame delimiter on both links |
| `NODE_BODY_CAP` | 0x100 | inferred |  | receive body cap of the node port, in bytes, with no growth |
| `NODE_PAYLOAD_MAX` | 0x80 | inferred |  | largest payload the node port sends in one frame |
| `NODE_UART_BLOCK_OFFSET` | 0x30000080 | inferred |  | offset of the node-side 16550-style register block from the node base in the Hub special space; one register per 8 bytes, accessed as 64-bit loads and stores with the low byte significant |
| `PEER_BODY_OFFSET` | 0x10 | established |  | receive body offset in bytes; read as the segment's reserved header, which is inferred |
| `PEER_ESCAPE_TX_A` | 0x11 | inferred |  | on the peer link the context escapes this byte on transmit (constants established, meaning inferred) |
| `PEER_ESCAPE_TX_B` | 0x13 | inferred |  | on the peer link the context escapes this byte on transmit (constants established, meaning inferred) |
| `PEER_GROWTH_LIMIT` | 0x410 | established |  | receive growth limit in bytes (1,040); with the 16-byte offset a frame carries up to 1,024 data bytes |
