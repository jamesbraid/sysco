# l1_link_framing

Constants of the PPP-style frame that the ColdFire L1 uses on its node link (the Bedrock UART) and on its RS-422 peer links, and the Bedrock-side register block the node link reaches. The frame bytes and the receive check value are shared by both links. The context values (body offset, growth limit, escape list) belong to the peer link and the node port's setup. Offsets of the node-side register block are from the node base in the Hub special space. Grades: the byte values and context numbers are read from the L1 image's frame and link set-up code. The meaning given to each one is an inference unless the item says otherwise.

Source: IRIX 6.5.30 l1.bin (L1 1.44.0, Origin 300 range) frame sender, peer-link set-up and node-port set-up.

The values are in [format.toml](format.toml), each graded and sourced, and the generated reference page is [format.md](format.md).

## Used by

- [Chunked request frames on the processor link (L1 side)](../../software/firmware/l1/chunked-request-frames.md)
- [Origin 300 L1 1.44.0: the four-channel UART and the serial link settings](../../software/firmware/l1/quad-uart-and-serial-links.md)
- [Origin 300 L1 router and serial-link reading](../../software/firmware/l1/rs422-router.md)
