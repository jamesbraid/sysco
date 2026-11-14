# L1 1.44.0 Cx-brick (type `6`, "Twister"): two-wire bus population, node-link UART channels, second UART window

What the Cx-brick personality (brick letter `6`, module literal `Twister.c`) expects on its two-wire bus and on the UART windows. Everything is stated as software behaviour: an address, a register, a value written, a pointer stored.

Read from L1 1.44.0, package 2 (`MIPS 2MB image`, container offset `0x1dfb5c`, 44-byte header, body sha256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`, loaded at `0xffe00000`), first program copy (the second copy is `0x100000` higher with the same bytes). Package 0 (type `C`) and package 1 (type `4`) contain no type-`6` code. The listing was regenerated with the 68040-mode decoder described in [../../../AGENTS.md](../../../AGENTS.md); descriptor arrays and jump tables were read from the raw bytes. The per-node reset, soft-reset, NMI and disable hooks are in [../../../platforms/bricks/cbrick-node-reset-and-disable.md](../../../platforms/bricks/cbrick-node-reset-and-disable.md); this note adds where those expanders sit on the bus.

Grades follow [../../../AGENTS.md](../../../AGENTS.md): **established** (the whole table or routine was read and the claim follows), **inferred** (reasoning stated), **unknown**.

## 1. How the personality is registered (established)

- Entry: the module init `0xffe50a94` (guard byte `0x300005e2`, log text `Twister_Init() called twice`, source literal `Twister.c` at `0xffe5d6f6`), and the board setup `0xffe50d18`.
- The board setup stores two bus-select hooks (`0xffe645a0` into `0x30000c88`, `0xffe64704` into `0x30000c98`), registers nine bus-switch records from the array at `0xffe518f4` (registration routine `0xffe8c014`), reads a variant code from `0xffe62e20`, and jumps through a 16-way table (`0xffe50d82`) to one of sixteen register arrays. Each case logs a string `Twister mix NNNN` (strings from `0xffe5df2b`): NNNN is four digits, each `3` or `9`, in the order 3333, 3339, 3393, 3399, 3933, ... 9999.
- Reading of the digits (inferred): digit *i* is the board type of node *i*, and `3` and `9` are the two node boards the IP35 PROM also lists (`IP53_*` and `IP59_*`). The arrays that differ from variant 3333 differ only in the records of the nodes whose digit is `9` (established for variants 0 to 13, see section 3; variants 14 and 15 were not decoded). What produces the variant code at `0xffe62e20` was not read (unknown).

## 2. Bus switches (established)

The first record of the switch array is the switch named `ROUTER`: form 0, segment 0, wire address `0xe0`. The other eight are per-node: `NODE n DIMM 0-3` at wire `0xe2` and `NODE n DIMM 4-7` at wire `0xe4`, each on bus segment 2 + *n*. (Record layout: byte 0 form, byte 1 segment, byte 2 wire, then a name pointer. Array at `0xffe518f4`, nine records of 8 bytes.)

The select hook `0xffe645a0` takes a segment byte. Its low nibble chooses the first level, its high nibble the second:

| Low nibble (segment) | Write |
|---:|---|
| 0 | none; the bus is used as it is |
| 1 | `0x00` to the switch at `0xe0` (all channels off) |
| 2 to 5 | `0x04`, `0x05`, `0x06`, `0x07` to the switch at `0xe0`, in that order; then, when the high nibble is nonzero, the second level below |

The bytes come from the table at `0xffe8c7a6` (`00 04 05 06 07`, indexed by channel 0 to 4): a four-channel switch with an enable bit `0x04` and a two-bit channel number, the same encoding the Origin 300 uses for its switch at `0xee` (inferred family: PCA9544-class; no image names it). So segments 2 to 5 are the four node boards, node *n* on segment 2 + *n*.

Second level, on the switches of the selected node (switch index 1 + 2*n* for DIMM 0-3 and 2 + 2*n* for DIMM 4-7, in the nine-record array): high nibble `0x10`, `0x20`, `0x30`, `0x40` select channel 1 to 4 of the DIMM 0-3 switch; `0x50`, `0x60`, `0x70`, `0x80` select channel 1 to 4 of the DIMM 4-7 switch; the bytes are again `0x04` to `0x07`. A switch descriptor of form 1 would use the table at `0xffe8c7ab` (`00 01 02 04 08`, one bit per channel); every Cx record has form 0, so that table is unused here (established for the nine records; the table's other user is not read).

## 3. Expander descriptors (established, variant 3333)

Array of 64 descriptors of 16 bytes at `0xffe5196c`; the array of variant *k* is at `0xffe5196c + 0x400 * k` (variants 0 to 13 were decoded, to `0xffe54d6c`). Field layout as in [ip45-io-expanders.md](ip45-io-expanders.md): form, segment, wire address, register, input bits, applied value, event bits, change hook, name. **Form 1 is a register device**: one data byte is sent to a register number, and one device answers eight registers; form 0 is the single-byte expander of the Origin 300 and the C-brick.

| Segment | Wire | Form | Register | Name | Input bits | Applied | Event bits |
|---:|---|---|---:|---|---|---|---|
| 0 | `0x84` | 1 | 0 | POWER ENABLE | `0x81` | `0xff` | `0x01` |
| 0 | `0x84` | 1 | 1 | FAN SPEED | `0xe0` | `0xff` | `0x00` |
| 0 | `0x84` | 1 | 2 to 5 | 12V STAT 0 to 3 | `0xff` | `0xff` | `0x3f` |
| 0 | `0x84` | 1 | 6 | POWER LEDS | `0xc0` | `0xff` | `0x00` |
| 0 | `0x80` | 1 | 0 | RTR POWER | `0x77` | `0xff` | `0x60` |
| 0 | `0x80` | 1 | 1 | NODE PRES | `0xff` | `0xff` | `0x00` |
| 0 | `0x80` | 1 | 2 | WIDGET PRES | `0x00` | `0x00` | `0x00` |
| 0 | `0x80` | 1 | 3 | ROUTER CONTROL | `0x00` | `0xf1` | `0x00` |
| 0 | `0x80` | 1 | 4 to 7 | N0 N1 PORTS, N2 N3 PORTS, NIA NIB PORTS, NIC NID PORTS | `0x55` | `0xdd` | `0x11` |
| 0 | `0x40` | 0 | none | DISPLAY CONTROL | `0x0f` | `0xff` | `0x07` |
| 2 + *n* | `0x82` | 1 | 0 | NODE*n* MARGIN | `0xcc` | `0xee` | `0xcc` |
| 2 + *n* | `0x82` | 1 | 1 | NODE*n* VCPU | `0x60` | `0xe8` | `0x60` |
| 2 + *n* | `0x82` | 1 | 2 | NODE*n* LED0 LOW | `0xff` | `0xff` | `0x00` |
| 2 + *n* | `0x82` | 1 | 3 | NODE*n* PSC | `0xff` | `0xff` | `0x00` |
| 2 + *n* | `0x82` | 1 | 4 | NODE*n* ASIC CTRL 1 | `0x00` | `0x72` | `0x00` |
| 2 + *n* | `0x82` | 1 | 5 | NODE*n* ASIC CTRL 2 | `0x00` | `0x8a` | `0x00` |
| 2 + *n* | `0x82` | 1 | 6 | NODE*n* CPU CTRL | `0x00` | `0xc3` | `0x00` |
| 2 + *n* | `0x82` | 1 | 7 | NODE*n* PORT (second mask `0x80`) | `0x55` | `0xdd` | `0x11` |
| 2 + *n* | `0x4a` | 0 | none | NODE*n* LED0 HIGH | `0xff` | `0xff` | `0x00` |
| 2 + *n* | `0x4c` | 0 | none | NODE*n* LED1 LOW | `0xff` | `0xff` | `0x00` |
| 2 + *n* | `0x4e` | 0 | none | NODE*n* LED1 HIGH | `0xff` | `0xff` | `0x00` |

Each node also has a placeholder record named `NODE*n* NULL` with wire address `0xff` (absent marker, form 0). The three controls the node hooks use sit in registers 4 to 6 of the node's device: `ASIC CTRL 1` (applied `0x72`, the same byte as the C-brick and the Origin 300), `ASIC CTRL 2` (`0x8a`) and `CPU CTRL` (`0xc3`; the four CPU-disable bits `0x04` to `0x20` are clear in it). So the per-node soft reset (bit `0x80` of `ASIC CTRL 1`, set then cleared after 10 ticks) and NMI (bit `0x40`, cleared then set) of the node hooks `0xffe63750` and `0xffe637d8` act on register 4 of the device at wire `0x82` on segment 2 + *n*. The hold clears the same three group bits as the C-brick, in register 4 and 5 of the same device; the strap and release order are in the node note.

The variant arrays for a node whose digit is `9` replace three records of that node and re-point a fourth (established for variants 1 to 13, each compared with variant 0 record by record): register 1 becomes `NI/FAN CTRL` (input `0x85`, applied `0x8d`, event `0x00`), register 2 becomes `II/FAN TACH` (input `0xf5`, applied `0xfd`), `LED0 LOW` moves to a form-0 expander at wire `0x48` on the node segment, and the `VCPU` descriptor is re-pointed to register 7, where the `PORT` descriptor also sits (the overlap was not resolved: unknown). `ASIC CTRL 1`, `ASIC CTRL 2` and `CPU CTRL` keep registers 4 to 6.

What part answers the register form is not named by any image (unknown). The same wire addresses `0x84`, `0x80` and `0x82` appear in the six-record monitor table at `0xffe5193c` onward (`PWR IN` segment 0 wire `0x84`; `ROUTER` segment 0 wire `0x80`; `NODE 0` to `NODE 3` segment 2 to 5 wire `0x82`; record layout segment, wire, two zero bytes, name pointer). That the monitors and the register devices are the same parts is inferred from the equal addresses; the routine that consumes the monitor table was not read.

## 4. UART windows and node-link channels (established)

The chip-select setup (`0xffe7548c` to `0xffe754b2`) programs, for every letter, `CS4` with mask 0 and control `0x095f`; for letter `6` only it also programs `CS5`: base register `0x6000` (byte base `0x60000000`), mask 0 (64 KB), control `0x095f`.

| Window | Part | Clock | Interrupt | Software ports | Source |
|---|---|---|---|---|---|
| `CS4` `0x50000000` | an eight-channel UART with a device-identification byte at offset `0x8d`: `0x24` is read as four channels, `0x28` as eight; 16-byte register block per channel (block *c* at base + 16 *c*) | 24,000,000 Hz (`0x16e3600`) | external level 4 (vector 28) | 7 to 14 are channels 0 to 7 | init `0xffe84394` (pointer `0x30000cd8`), open `0xffe84e14` (pointer `0x30000cdc`), both stored by `0xffe845b4` |
| `CS5` `0x60000000` | a four-channel 16C654-class UART, the same driver and register set as `CS4` on the other letters ([quad-uart-and-serial-links.md](quad-uart-and-serial-links.md)) | 12,000,000 Hz (`0xb71b00`) | external level 3 (vector 27) | 3 to 6 are channels 0 to 3 | init `0xffe850ac(0x60000000, 3, 0xb71b00)` at `0xffe75586`, open `0xffe859d8` |

The UART families are inferred from register behaviour only: an eight-channel part with a 16-byte channel stride and an identification byte at `+0x8d` is consistent with the Exar XR16L78x family, which was not checked against its data sheet (inferred; no image names the part). For the other letters `CS4` is the four-channel part at 12 MHz (init `0xffe850ac(0x50000000, 4, 0xb71b00)` at `0xffe7554e`).

The board setup opens these ports for letter `6` (jump tables at `0xffe755bc`, `0xffe756b2`, `0xffe7583a`; arms read in full):

| Software port | Channel (window) | Rate (nominal) | Class | Use |
|---:|---|---|---|---|
| 7 to 11 | 0 to 4 of `CS4` | 57,600 (`0xe100`), flow mode 1 | 3 | node links (five ports, see below) |
| 12 | 5 of `CS4` | 38,400 (`0x9600`) | 2 | the L1 console, handed to the console routine as software port 12 |
| 13, 14 | 6 and 7 of `CS4` | 3,000,000 (`0x2dc6c0`) | 5 | high-speed links; port 13 is router slot 1, see [peer-link-slots-and-cables.md](peer-link-slots-and-cables.md); port 14 is not assigned to a slot by the peer table (unknown use) |
| 3 to 6 | 0 to 3 of `CS5` | 107,142 (`0x1a286`) | 1 | peer links, soft flow control, router slots 9 to 12 |

(Class is the second argument of the open call: 1 peer link, 2 console, 3 node link. Within this open code only letter `6` uses class 5.)

**Four node links or five.** The open code makes five node-link ports, 7 to 11, so five channels of the eight-channel part. The node hooks and the register arrays above have four nodes. The fifth channel is not explained by anything read (unknown; the router board is a candidate because the register arrays carry a `ROUTER` block next to the four `NODE` blocks, inferred from the names only).

**What flow mode 1 means on the eight-channel part is not established.** The setting block passed to its open routine carries the same fields as the four-channel one (rate, class, parity, flow mode); the routine that programs the part (`0xffe848b0`, `0xffe85de4`) was not read. For the 16C654-class `CS5` part the mode is not used on the peer links (they use software flow).

## Not settled

| Item | Why open |
|---|---|
| Part behind the form-1 register devices and the identity of the monitors | no image names it; the routines that consume the monitor table were not read |
| Source of the variant code `0xffe62e20`; variants 14 and 15 | not traced or not decoded |
| The fifth node-link channel; port 14 | see section 4 |
| Flow control and register programming of the eight-channel UART | `0xffe848b0` and `0xffe85de4` not read |
| Second-level switch contents (the DIMM identification routes) | only the switch bytes were read |
