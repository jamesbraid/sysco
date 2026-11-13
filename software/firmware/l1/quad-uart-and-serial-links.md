# Origin 300 L1 1.44.0: the four-channel UART and the serial link settings

Read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB image`, body sha256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`, load address `0xffe00000`, first program copy). The same switch tables and driver bytes occur in the other ColdFire packages of the composite; only package 1 was read for the UART programming. The on-chip UART 1 letter table (the last row of the table below) was also read in packages 0, 2, 3 and 5: packages 0 and 2 have the same two groups, and packages 3 and 5 open UART 1 for every letter except `O` ([power-bay-link.md](power-bay-link.md#which-bricks-start-the-power-bay-task-all-five-coldfire-packages)). This note says what the software programs into the external UART block at `CS4` (`0x50000000`) and which software port each use belongs to, so that a firmware-backed machine can be checked against its own register log and link traffic.

## The part behind the register block

- **Register layout (established, driver `0xffe636b0`-`0xffe64100`).** Four channels at byte offsets `+0x00`, `+0x08`, `+0x10`, `+0x18` from the select base, eight byte registers each. The driver treats the registers as the 16550 set: `+0` receive/transmit data and divisor low, `+1` interrupt enable and divisor high, `+2` interrupt identification on read and FIFO control on write, `+3` line control, `+4` modem control, `+5` line status, `+6` modem status, `+7` scratch.
- **Enhanced register set (established).** The initialiser (`0xffe638f4`-`0xffe63a1a`) writes line control `0xbf`, then, with that value present, uses `+2` as the enhanced feature register, and `+4`, `+6` as the Xon-1 and Xoff-1 characters (`0x11` and `0x13`), before restoring line control. Register `+2` holds three values by mode: `0x10` (enhanced mode on, no flow control), `0xd0` (enhanced on plus automatic RTS and CTS flow control) and `0x1a` (enhanced on plus software transmit and receive flow control with the characters above). This set of registers and meanings is that of the 16C654-class quad UARTs (a 64-byte FIFO part with an enhanced feature register); the software never reads a device identifier, so the part family is inferred from register use and is unknown as a marking.
- **Probe (established, `0xffe636f6`-`0xffe63754`).** For each channel: clear `+3` (line control), write `0xa0 | channel` into the scratch register `+7`, and later read all four scratch registers back; any mismatch logs an error and abandons the whole device. Then clear `+1` (interrupt enable) of every channel, create the driver objects, install the level-4 handler and unmask the interrupt. The part needs eight independent byte registers per channel, at least the scratch register being read-write.
- **FIFO and interrupt use (established).** The initialiser writes the FIFO control byte `0x0f` or `0x8f` to `+2` after the speed is set (FIFO enable, reset of both FIFOs, the DMA-mode bit, and receive trigger bits `00` or `10`; the choice depends on whether a field of the device record equals 2), and reads `+5`, `+6` once to clear pending status (`0xffe63a0e`). The handler (`0xffe63d44`) loops over the four channel records and, for each channel whose interrupt identification byte `+2` is not `0xc1` (no interrupt pending, FIFOs enabled), dispatches on it (`0xffe63ba4`): `0xc4` receive-data trigger (read 56 bytes at once when the software counter allows, else byte at a time while line status bit 0 is set), `0xc6` and `0xcc` receive error or character timeout (read while data ready), `0xc2` transmit holding register empty (write up to 56 bytes), `0xc0` modem status. The upper identification bits therefore read `11` (FIFOs on) and the part must present a nonzero identification (not `0xc1`) when it wants service. The 56-byte quantum is the code's own constant.
- **Modem control (established).** `+4` is written `0x03` (DTR and RTS) in normal use or `0x10` (loop-back) when the channel's loop-back flag is set; the flag is `0` in every settings block the board code builds. The modem status register is read to clear status; no path tests individual modem-status bits for data flow beyond the automatic flow control programmed through the enhanced register.
- **Clock and divisor (established arithmetic, inferred board).** The divisor written to the divisor latch (`+0` low, `+1` high, with line control bit 7 set by the `0xbf`/`0x80`-style access) is the rounded value of clock divided by 16 times the baud rate (`0xffe638f4`), computed from the software clock constant: 12,000,000 for `CS4` and for `CS5`, 24,000,000 for `CS4` when the brick letter is `6`. A 12 MHz input and a 16x divisor give the published figures 107,142, 57,692 and 37,500 baud (divisors 7, 13 and 20), which are the actual rates behind the nominal 107,142, 57,600 and 38,400 below. Timer 2 of the processor outputs the same 12 MHz (see [the board contract](board-boot-contract.md)); that the board uses it as the UART clock is inferred.
- **Line format (established).** Line control is `0x03` for 8 data bits and no parity, `0x1b` for even parity, `0x0b` for odd parity, chosen by one configuration field; one stop bit in all three.

## Software port to channel, and what each port is used for

The board set-up code (`0xffe54be0`-`0xffe5518e`) calls the driver open routine (`0xffe63fdc`) with a software port number (3 to 6, which is channel 0 to 3) and a settings block of baud rate, parity and flow mode. The port tables are five jump tables indexed by the brick letter, found at `0xffe54cbc`, `0xffe54da8`, `0xffe54e9e`, `0xffe55026` and `0xffe5510e`; the groups below were decoded entry by entry. For the Origin 300 (letter `4`) the result is:

| Software port | Channel | Baud (actual) | Parity | Flow | Use (established unless marked) |
|---|---|---|---|---|---|
| 3 | 0 | 107,142 (107,143 from the divisor `7`) | none | software, Xon `0x11` Xoff `0x13` | peer link, RS-422, peer slot 1 |
| 4 | 1 | 107,142 | none | software | peer link, RS-422, peer slot 9 |
| 5 | 2 | 38,400 (37,500) | none | none | the L1 console: opened with `0x9600`, then handed to the console routine (`0xffe79ae4`) |
| 6 | 3 | 57,600 (57,692) | none | automatic RTS/CTS | the node (Bedrock) link: opened with `0xe100` and flow mode 1 |

The second argument of the open call is a client class: 1 for a peer link, 2 for the console, 3 for the node link. The nominal baud argument is the figure the `uart` console command prints; the actual rate follows from the divisor. The five jump tables partition the brick letters into groups. The groups for each table, decoded entry by entry (established):

| Table | Group of letters | Effect |
|---|---|---|
| console (`0xffe54da8`) | `# % 0 1 2 4 7 8 9 = ? A C E I P Q R T V X Y` | console on port 5 at `0x9600`, no flow control |
| console | `& N` | no console on port 5 |
| console | `6` | console through the function pointer at `0x300006e0`, software port 12 |
| peer links (`0xffe54e9e`) | `# % 0 1 2 4 7 8 = ? C E I P Q T V X Y` | ports 3 and 4 at `0x1a286` (107,142), software flow control |
| peer links | `& N` | ports 3, 4, 5 and 6 as peer links at 107,142 |
| peer links | `6` | ports 3 to 6 as peer links, then software ports 13 and 14 at 3,000,000 through the pointer at `0x300006e0` |
| node link (`0xffe55026`) | `0 1 2 4 7 8 9 A C E Q R V` | port 6 at `0xe100`, automatic RTS/CTS |
| node link | `6` | software ports 7 to 11 through the pointer at `0x300006e0` |
| on-chip UART 1 (`0xffe5510e`) | `0 4 7 8 9 A E Q V` | UART 1 not opened |
| on-chip UART 1 | every other letter | UART 1 opened at 19,200 baud |

Letters outside a table's listed groups fall into the "nothing" arm of that table. For the Origin 300 (`4`) the rows read: console on port 5, peers on ports 3 and 4, node link on port 6, UART 1 closed. The routines behind the function pointers at `0x300006dc`/`0x300006e0` are empty stubs in this package (`0xffe70430`, `0xffe70438`: each is `link`, `unlk`, `rts`), so letter `6` has no working second UART path here.

- **Flow control agrees with the link framing (established).** The peer links run software flow control with `0x11` and `0x13`, and the peer framing context escapes exactly those two bytes (see [rs422-router.md](rs422-router.md)). The node link runs automatic hardware flow control, and its framing context escapes neither. If the modem-control lines on port 6 are not connected, the node link can stall once the receive FIFO fills, because the enhanced-register setting makes the part stop transmitting when CTS is deasserted; with CTS asserted the link behaves as one with ample buffering.
- **Which cable each peer port reaches (unknown).** Port 3 is peer slot 1 and port 4 is peer slot 9 in the router; the image does not say which connector each reaches. The user guide's cabling text is a hint, not a finding.
- **Which port the node link occupies on the processor side (established for the framing, unknown for the electrical path).** The framing on port 6 is the Bedrock-side UART framing described in [rs422-router.md](rs422-router.md) and the L1 datasheet's framing section; the connection from channel 3's pins to the processor's UART block is not visible to the image.

## Interrupt wiring (established)

`CS4`'s UART drives external level 4 (vector 28); a `CS5` UART, present only for letter `6`, drives level 3 (vector 27). The handler masks nothing itself and the driver clears the interrupt-enable registers (`+1`) of all four channels at probe time and enables receive and line-status interrupts per open channel. The routine `0xffe63d44` services every channel with a pending identification in one pass.

## Node-side frame and register block

The node link frames its traffic as the peer links do ([rs422-router.md](rs422-router.md), "Peer frame format"): flag `0x7e`, escape `0x7d` with exclusive-or `0x20`, and the same frame check. Only the frame's own flag and escape bytes are escaped on this port, so neither flow-control byte is. The setup context (`0xffe346d4` to `0xffe34706`, with the context set-up at `0xffe62330`) gives a receive body offset of 0, no growth and 256-byte buffers. Inferred: the receive body cap is 256 bytes, and the sender puts at most 128 payload bytes in one frame. The frame header carries the type in bits 7 to 5 and the subchannel in bits 4 to 0.

The node end of the link is a 16550-style register block in the node's Hub special space, at the node base plus `0x3000_0080` (`L1LF_NODE_UART_BLOCK_OFFSET`). Registers sit one per 8 bytes and are accessed as 64-bit loads and stores, with the low byte significant. The L1 side is channel 3 of the quad UART (port 6 above), and this reading does not show which channel pin reaches the block.

The source gives no routine address for the block offset, the 256-byte cap or the 128-byte payload limit, so those rest on the reading alone and are graded inferred in [formats/l1_link_framing/format.toml](../../../formats/l1_link_framing/format.toml). No independent review has been done.

## Not settled

| Item | Why open |
|---|---|
| Fitted part marking | no image names it; register behaviour matches the 16C654 family, which the code uses at least as far as the registers above |
| Whether the board feeds timer 2's output to the UART clock input | the writes agree; the net is not visible |
| Cable assignment of peer slots 1 and 9 | the image uses selectors only |
| What the letter-`6` function-pointer ports are | the pointer targets are empty stubs in package 1; the other packages were not read |
