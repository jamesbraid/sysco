# L1 1.44.0: does the Bedrock node link wait on CTS?

The L1 talks to its node (the Bedrock UART) on software port 6, channel 3 of the quad UART, at a nominal 57,600 baud. The question is whether the firmware ever stops sending because the far end has not asserted clear-to-send, and what level the board has to present on the CTS input for the link to carry data.

Read from L1 1.44.0: the UART open and programming routines of package 0 (`Base 1MB image`, C-brick; programming routine at `0xffe63ca8`), package 1 (Origin 300; `0xffe63960`) and package 2 (`MIPS 2MB image`; four-channel driver `0xffe8535c`, control switch `0xffe857c0`, status routines `0xffe85464` to `0xffe8559c`, board set-up `0xffe75500` to `0xffe75a00`). Body hashes as in [peer-link-slots-and-cables.md](peer-link-slots-and-cables.md). The register layout and the 16C654-class reading are in [quad-uart-and-serial-links.md](quad-uart-and-serial-links.md).

Grades follow [../../../AGENTS.md](../../../AGENTS.md): **established** (the whole routine was read and the claim follows), **inferred** (reasoning stated), **unknown**.

## 1. What the firmware programs (established)

- The board set-up opens port 6 for the node link with the settings block {rate `0xe100` (57,600 nominal), flow mode 1, loop-back 0} and client class 3. In package 2 this is the arm for letters `0 1 2 4 7 8 9 A C E Q R V` of the node-link table (`0xffe7583a`, arm `0xffe75888`); packages 0 and 1 have the same arm. The letter `C` (C-brick) and `4` (Origin 300) are in it.
- The programming routine writes line control `0xbf`, then, by flow mode, the enhanced-feature register: mode 0 writes `0x10` (enhanced mode, no flow control), **mode 1 writes `0xd0`** (enhanced mode, bit 7 and bit 6 set), mode 2 writes `0x1a` and the Xon and Xoff characters `0x11` and `0x13`. Mode 1 is the only mode the node link uses. The peer links use mode 2.
- Modem control is written `0x03` (DTR and RTS asserted by software) in normal use and `0x10` (loop-back) only when the settings block's loop-back byte is set; no block built by the board code sets it.
- FIFO control is `0x0f` or `0x8f` (FIFOs enabled and reset, receive trigger by a field of the device record); the transmitter is fed from the interrupt handler's transmit-empty case, up to 56 bytes (`0x38`) per entry when the line status register reports an empty transmitter (`btst #5` of register `+5`, `0xffe8590c`).

## 2. What the part does with it (inferred)

With enhanced mode on, bit 7 of the enhanced-feature register is the automatic CTS enable and bit 6 the automatic RTS enable on the 16C654 and 16C554 families, the set of registers the driver's probe and register use match. With bit 7 set, the transmitter holds off while the CTS input is inactive, and the transmit FIFO stays full; with bit 6 set, the RTS output is driven by the receive FIFO fill level instead of by modem control. The register identification is from the register behaviour and the published family data, not from a fitted part (inferred; no image names the part).

## 3. What the firmware does about CTS (established for the driver routines read)

- No instruction in the driver tests bit 4 of a modem-status copy (bit 4 is CTS). The modem-status register (`+6`) is read at initialisation to clear it (`0xffe8540e`), in the diagnostic dump of the control switch's last case (`0xffe85998`, printed with the line status and the interrupt identification), and in two status routines (`0xffe85464`, `0xffe854a4`) that copy it and test bits 2 and 6. Those two routines branch on the channel record's field at `+4`, which the open call fills with the client class (inferred from the open call's argument): the values 1 (peer link) and 2 (console) lead to the tests, any other value, including 3 for the node link, does nothing with the copied byte. The status-query routine `0xffe8552c` takes a request mask whose bit 4 selects "read the modem-status register into a local"; that is a request flag, not a CTS test.
- The transmit path writes whenever the transmitter reads empty; it does not look at CTS before or after. So the firmware has no stall of its own on CTS.
- Consequence: **the stall exists only in the UART part, through the automatic CTS enable.** A node link whose CTS input is inactive accepts at most 56 bytes into the transmit FIFO and then stops: the transmitter never empties, the transmit-empty interrupt never comes, and the writer task blocks once its buffer is full. This follows from the transmit routine and the enable bit (inferred for the part's behaviour, established for the routine).

## 4. Level the board has to present (inferred)

CTS on a 16C654-class part is an active-low input: the modem-status CTS bit reads 1 when the pin is low. The node link works only if the board drives the pin low (asserted) or ties it low; a floating or high pin stops transmission as in section 3. RTS needs no input: it is an output the far end may ignore. The wiring of the pins to the Bedrock end is not visible to the image (unknown). With automatic flow control and the receive trigger the driver sets, the node link behaves as a link with a deep buffer in the receive direction as long as the CTS input is asserted.

## 5. Other letters

- Letter `6` (Cx-brick) opens its node links on ports 7 to 11 of the eight-channel UART with the same settings block (flow mode 1) through function pointers (`0xffe84e14`); how that driver programs flow control was not read (unknown), see [cxbrick-mbus-and-uart-population.md](cxbrick-mbus-and-uart-population.md).
- Letters outside the node-link arm (`& N # % = ? 3 5 ...`) have no node link on port 6.

## Not settled

| Item | Why open |
|---|---|
| The fitted UART part | no image names it |
| Whether flow mode 1 of the eight-channel driver means a hardware CTS gate | the programming routine was not read |
| The CTS and RTS nets on the board | not visible to the image |
