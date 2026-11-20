# L1 1.44.0 C-brick: the node-link receive path and why a frame is rejected

What the C-brick L1 (package 0, `Base 1MB`, 1.44.0 of the IRIX 6.5.30 `l1.bin`) accepts on its node link (software port 6, channel 3 of the quad UART, 8N1 at 57,600 baud nominal, client class 3), how it checks a frame, what each logged PPP error means and which byte errors give which length pattern. Written for a launch that logs `SCC 0 PPP error (len=9..113): PPP:bad FCS (checksum)` 25 times and `short read/frame` once. Grades: E established (routine read in full), I inferred, U unknown. The framing constants for the peer and node links are also in [rs422-router.md](rs422-router.md) and [node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md); this note adds the receive state machine, which those do not give.

## 1. The frame the L1 accepts (E)

The receiver is the read engine at `0xffe628e4` to `0xffe62d92` (check helper `0xffe6252c`, error reporter `0xffe62604`; the frame sender is `0xffe626fc`), set up for the node link at `0xffe346ce` to `0xffe34706` by `0xffe62678` with a 256-byte receive buffer, a 256-byte transmit buffer, both optional escape flags zero, no buffer offset and no growth.

- **Flag.** `0x7e` delimits frames. A frame is the bytes between two flags. Consecutive flags (empty frames) are ignored without an error. There is no address byte, no control byte and no protocol field: the first byte after the flag is the L1 link header (class in bits 7:5, index in bits 4:0, see [node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md) section 3). A real-PPP `ff 03` prefix would be passed on as data and fail later, not at the check below.
- **Escape.** `0x7d` marks the next byte, which is exclusive-ored with `0x20` (`0xffe62b9e` to `0xffe62ba8`). Any byte can follow, the receiver does not test which. Nothing else is special on the node link: `0x0d` is data because the context flag at `+0x24` is zero (with it set, `0x0d` is rejected as a control character, error -1704), and `0x11`/`0x13` are data (the transmit flag at `+0x20` is zero and the receiver has no test for them).
- **FCS.** FCS-16 as in PPP: table-driven (table at `0xffe62d94`), reflected polynomial `0x8408`, initial value `0xffff`, no byte swapping of the data. It covers **every unescaped byte between the flags, including the header byte, up to and including the two FCS bytes**. The sender transmits the complement of the running value, low byte first. The receiver runs the same routine over the body and the last two bytes and compares the result with the constant `0xf0b8` (`0xffe62ab4`); it does not recompute and compare the two bytes separately. E.
- **Length limits.** The receive buffer is 256 bytes (the size passed at set-up). A frame needs at least 3 unescaped bytes between the flags. A longer frame than the buffer plus two bytes held back for the FCS ends in an overflow error and the rest of it is discarded up to the next flag. So the largest accepted frame is 258 bytes including the FCS (256 body). E for the rule, I that the capacity is the set-up size.

## 2. What each error is (E)

The reporter passes the code, the frame buffer, the **byte count excluding the two FCS bytes** and the running totals to the node link's callback (`0xffe3606c`), which prints `SCC %d PPP error (len=%d): %s` with that count. The codes are -1700 and up, in the order of the string table at `0xffe5c600`:

| Code | Printed text | Raised when |
|---:|---|---|
| -1700 | `bad FCS (checksum)` | a closing flag ends a frame of 3 or more bytes, there is no pending escape, and the check of section 1 does not give `0xf0b8` |
| -1701 | `read buffer overflow` | more than 256 body bytes plus 2 held-back bytes arrive before a flag |
| -1702 | `short read/frame` | a closing flag ends a frame of **1 or 2 bytes** (empty frames are ignored) |
| -1703 | `escape sequence in read stream` | a closing flag arrives straight after `0x7d` |
| -1704 | `control sequence in read stream` | `0x0d` arrives with the context flag set (not the case on the node link) |
| -1705 | `read stream not in ...` (text cut short in the table) | a byte other than a flag arrives while no frame is open; logged once per run of such bytes |
| -1706 | (buffer growth failed) | not used on the node link |

After a bad frame the receiver re-arms at once: the next flag opens a new frame, so a loss of synchronisation costs one frame, not more. A repeat of the same code is not printed unless the debug switch is on (`0xffe36098` to `0xffe360b0`: the last code is remembered). E for the codes and conditions; the exact text of -1705 was not read.

The log therefore says: 25 frames closed by flags with 3 or more bytes and failed the FCS; one closed with 1 or 2 bytes; **none** ended in a bare escape, overflowed, or arrived outside a frame, which means the stream was in step with the flags throughout.

## 3. Reading the lengths (I, from the rules above)

The printed length is the frame body without its two FCS bytes. A request frame is 8 bytes of header plus its arguments (an integer 5 bytes, a string its length plus 2), so a request with no arguments prints 8 and the PROM's `0x0006` read (four integers) prints 28. A console-output frame (class 2) has a one-byte header and no destination, so it prints 1 plus its text length. The sample lengths 9 to 113 therefore fit the PROM's console-text frames (a line of text of 8 to 112 characters) at least as well as requests, since 9 is not a legal request length (8 plus an argument is at least 13).

| What happened to the bytes | Length printed | Error |
|---|---|---|
| A byte lost (UART overrun, model drops it) | sender's length minus 1 for each byte lost | bad FCS |
| A byte duplicated or inserted | plus 1 each | bad FCS |
| A byte changed (bit error, wrong value from the model) | unchanged | bad FCS |
| Closing flag lost: two frames merge | the sum of both bodies (two frames' headers and FCS inside the data) | bad FCS |
| An extra flag inserted: one frame splits | two frames, the first shorter than the original, the second the remainder; one of them lands in 1 or 2 bytes only when the split is within two bytes of an end | bad FCS, and `short read/frame` for a 1 or 2 byte piece |
| A data `0x7e` sent unstuffed | splits as above | bad FCS |
| A data `0x7d` sent unstuffed | the next byte is xor-ed, length one shorter (the `0x7d` is consumed) | bad FCS; or -1703 if a flag follows at once |
| The sender's FCS wrong (other init, not complemented, bytes swapped, FCS over a different range) | every frame is exactly its legitimate length | bad FCS on every frame, none ever accepted |
| `0x7e` flags doubled or dropped between frames | unchanged | none (empty frames are ignored); a dropped one merges two frames |

Which one the evidence shows cannot be settled from the L1 log alone. Two checks separate them: (a) if every logged length equals a legitimate frame length (8 plus arguments, or 1 plus text) and **no frame ever passes**, the bytes are intact and the sender's or the receiver's FCS differs; (b) if the lengths are scattered below the legitimate ones, bytes are being lost, which is what a byte-stream delivered faster than the L1's UART interrupt drains it does (the 16C654-class channel has a 64-byte FIFO, [quad-uart-and-serial-links.md](quad-uart-and-serial-links.md)) and which a single `short read/frame` among 25 failures also suits (a loss of a flag or a few bytes at the end of a frame). A valid frame that the L1 rejects for another reason does not appear as an FCS error: the FCS is checked before the header is looked at. The sender is the PROM's own frame writer, which uses the same algorithm (init `0xffff`, reflected `0x8408`, complemented, low byte first; [node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md) section 2), so a wrong FCS from the PROM is unlikely and corruption in the delivery to the L1's UART is the first suspect (I).

## 4. Who sends first, and what the L1 sends in the first seconds

- **The node sends first.** The L1 has no unsolicited transmit on the node link after the release in anything read: the node-link writer sends replies to requests and the console characters of the selected console subchannel. No L1-originated frame tied to the release was found. I (an absence in the routines read, not a search of every task).
- **What the node sends.** The IP35 PROM's requests (`0x0001` debug switches, `0x0003` rack, bay and type, `0x0004` brick serial, `0x0006` board EEPROM, `0x0008` system serial, `0x0009` partition, `0x000a`, `0x000e` and the display, partition and event requests) and console output as class 2 frames ([node-link-sn1-vs-sn2.md](node-link-sn1-vs-sn2.md) section 5). The order in which it sends them is a PROM property, not in the L1 image.
- **A node that gets no reply** waits: the L1 has no retry or resend, the PROM's own timeouts are not in the L1 image. A PROM that prints nothing and loops after its first requests went unanswered is consistent with waiting for the L1's reply to `0x0001` or `0x0003`, but that is a PROM-side question (I).

## 5. Does the L1 write anything to the node at the release? (E)

- **Over the link:** nothing. The release wrapper resets the L1's own UART ports (`0xffe36a04`, `0xffe52a30`) **before** the release hook, while the node is still held, so the node-link receive state is clean when the node starts. It writes no frame.
- **Through Bedrock registers:** none; the L1 has no access path to Bedrock registers ([cbrick-power-up-after-scan.md](cbrick-power-up-after-scan.md)).
- **Through expanders:** only the reset group, the `BEDROCK CNTL 2` strap bits 7:3 (bit 3 always, bit 4 from setting 41, bit 5 from setting 40, bit 7 from setting 42 or the module record, [power-reset-cbrick.md](power-reset-cbrick.md) section 6) and the processor-module bits. What the straps select at the Bedrock is unknown.
- **EEPROM and I2C:** the power-up path writes no EEPROM: the node EEPROM and the PIMM EEPROMs are only read ([cbrick-mbus-registration.md](cbrick-mbus-registration.md)).
- **NASID, partition, module number, console selection:** the L1 holds rack, bay (settings 1 and 2), brick type and partition (setting 14) and supplies them **only as the replies to the PROM's requests** (`0x0003`, `0x0009` and `0x000b`). Nothing sets a node number in hardware. So "node N and its peer both carry number 0" is not something the L1 sets: the PROM derives the number after it has read rack and bay, which an unanswered `0x0003` leaves unset (I). Console selection is the L1's own `select` state and is only used to route characters.

## What the retained sources cannot settle

- Which of the failure patterns in section 3 the real log shows: the lengths of each failed frame against the PROM's frames are needed, and a byte dump of one failed frame.
- Whether the L1's UART interrupt keeps up with a burst: the ColdFire clock and the FIFO trigger level programmed for the node channel were not read for this note.
- The exact PROM request order and the PROM's own timeouts (PROM-side readings).
- What the `BEDROCK CNTL 2` straps select.
