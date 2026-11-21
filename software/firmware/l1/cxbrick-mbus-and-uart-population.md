# L1 1.44.0 Cx-brick (type `6`, "Twister"): two-wire bus population, node-link UART channels, second UART window

What the Cx-brick personality (brick letter `6`, module literal `Twister.c`) expects on its two-wire bus and on the UART windows. Everything is stated as software behaviour: an address, a register, a value written, a pointer stored.

Read from L1 1.44.0, package 2 (`MIPS 2MB image`, container offset `0x1dfb5c`, 44-byte header, body sha256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`, loaded at `0xffe00000`), first program copy (the second copy is `0x100000` higher with the same bytes). Package 0 (type `C`) and package 1 (type `4`) contain no type-`6` code. The listing was regenerated with the 68040-mode decoder described in [../../../AGENTS.md](../../../AGENTS.md); descriptor arrays and jump tables were read from the raw bytes. The per-node reset, soft-reset, NMI and disable hooks are in [../../../platforms/bricks/cbrick-node-reset-and-disable.md](../../../platforms/bricks/cbrick-node-reset-and-disable.md); this note adds where those expanders sit on the bus.

Grades follow [../../../AGENTS.md](../../../AGENTS.md): **established** (the whole table or routine was read and the claim follows), **inferred** (reasoning stated), **unknown**.

## 1. How the personality is registered (established)

- Entry: the module init `0xffe50a94` (guard byte `0x300005e2`, log text `Twister_Init() called twice`, source literal `Twister.c` at `0xffe5d6f6`), and the board setup `0xffe50d18`.
- The board setup stores two bus-select hooks (`0xffe645a0` into `0x30000c88`, `0xffe64704` into `0x30000c98`), registers nine bus-switch records from the array at `0xffe518f4` (registration routine `0xffe8c014`), reads a variant code from `0xffe62e20`, and jumps through a 16-way table (`0xffe50d82`) to one of sixteen register arrays. Each case logs a string `Twister mix NNNN` (strings from `0xffe5df2b`): NNNN is four digits, each `3` or `9`, in the order 3333, 3339, 3393, 3399, 3933, ... 9999.
- Reading of the digits (inferred): digit *i* is the board type of node *i*, and `3` and `9` are the two node boards the IP35 PROM also lists (`IP53_*` and `IP59_*`). The arrays that differ from variant 3333 differ only in the records of the nodes whose digit is `9` (established for variants 0 to 13, see section 3; variants 14 and 15 were not decoded). What produces the variant code at `0xffe62e20` is read in section 5.

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

**Five node-link channels (established), what the fifth one carries (unknown).** The SysCom channel table is built by `0xffe3a608` from a table chosen by the type letter (selection in `0xffe3a7e4`): letter `6` selects the five-entry table at `0xffe3c384`, whose entries are 28 bytes each and name software ports 7, 8, 9, 10 and 11 in that order, every one with channel type 2 (the same type as the single node-link entry of the other letters, port 6 at `0xffe3c368`). Each entry gets its own receive task and, through the device-open routine `0xffe85fa0`, the same 28-byte settings block, so the fifth port (SysCom channel 4, 11 in the open table) is a channel of the same kind as the other four, not a differently configured port. The node hooks, the variant arrays and the node-type discovery all have four nodes. What sits behind channel 4 (the router board is a candidate, because the register arrays carry a `ROUTER` block next to the four `NODE` blocks) was not traced: the routines that choose a SysCom channel for a node or for the router were not read (unknown; inferred only from the names).

**What flow mode 1 means on the eight-channel part (established for the register writes, inferred for the part).** The routine `0xffe85de4` is not the programmer; it is the generic "apply settings" routine that all UART families share. It changes a value of 1 in each of three bytes of the settings block (offsets `+0x16`, `+0x17`, `+0x18`) to 2, calls the driver's control hook (the function pointer at `+0x24` of the port object, here `0xffe84c10`) with operation 0 and the block, then copies the 28-byte block into the port object at `+8`. Operation 0 reaches the real programmer `0xffe845e0`, read in full (1.44.0 MIPS 2MB image, container index 2 of 6.5.30 `field_diags_sysco`, package sha256 `22e340c4`, body loaded at `0xffe00000`). Per channel (block at base + 16 x channel) it:

- sets or clears this channel's bit in the global byte at window offset `0x88` (the per-channel 8x sampling select) by comparing the rate with the clock divided by 16: a rate above 1,500,000 baud at the 24 MHz clock selects 8x and a divisor of clock / (8 x rate), otherwise 16x and clock / (16 x rate), the divisor rounded to nearest and at least 1. The 3,000,000 baud links on ports 13 and 14 therefore run 8x with divisor 1; 57,600 gives divisor 26, an actual 57,692 baud (established arithmetic);
- writes line control `0x80`, the divisor low and high bytes at `+0` and `+1`, then line control `0` (`0x03`, `0x1b` or `0x0b` for parity 0, 1 or 2 afterwards);
- writes register `+8` as `0xc0`, with the two low bits set to 1 (to 3 for a class 2 console) only when flow mode is 1;
- writes the enhanced-feature register `+9` as `0x10` for flow mode 0, **`0xd0` for flow mode 1** (bits 7 and 6 set in enhanced mode), and `0x1a` plus the Xon and Xoff characters at `+0xc` to `+0xf` for flow mode 2 (the same three values as the four-channel part, see [node-link-cts-flow-control.md](node-link-cts-flow-control.md));
- writes `+0xa` as 8 and `+0xb` as `0x20` (console) or `0x38` (other classes), FIFO control `+2` as 1 then `0x0f`, modem control `+4` as 3 (or `0x10` for loop-back), and interrupt enable `+1` as 0.

So flow mode 1 is automatic hardware RTS/CTS flow control by the part, the same as on the four-channel part (established as register writes; that the bits mean auto-CTS (bit 7) and auto-RTS (bit 6) is inferred from the part family, not checked against a data sheet). Register `+8` (`FCTR`-shaped, trigger table bits 7 and 6 plus a hysteresis field) is inferred the same way. The source file name the code logs is `uart_1678x.c`, and the generic layer logs `uart.c`.

**What `0xffe848b0` tests, and what bit 6 is (established).** The routine reads the word at `+4` of the port object, which is the client class (the second argument of the open call, stored by `0xffe85fa0`), not the flow mode as an earlier version of this note read it. Class 3 (node link) and every class other than 1, 2, 5 and 6 return 1 without touching the device, so the node links are always taken as ready. Classes 1 and 5 poll byte `+6` of the channel's register block up to 600 times and class 2 reads it once. Byte `+6` is the modem-status register in this part's 16550-style layout (`+5` is the line status the transmit and receive code tests, bit 5 transmitter empty), and bit 6 of it is the ring-indicator input; the routine reports ready (1) when that bit is clear. Class 6 is ready only when both direction-enable bytes (`+0x50` and `+0x88` of the port object, set by `0xffe85f54`) equal 1. The open routine calls `0xffe848b0` after the settings are applied and, when it returns nonzero, calls `0xffe85f54(port, 1)` to enable the port. The modem-status interrupt path (`0xffe847d4`) copies the register into the port object (`+0x38`) and calls `0xffe866d4` unless the class's condition says no: class 2 skips it while bit 6 is set, classes 1 and 5 skip it only when bit 2 (trailing-edge ring indicator) is clear and bit 6 is set, class 6 calls it only when bit 2 is set. That routine fails the pending read or write of any direction that is not enabled (error `0xfffffbb3`, -0x44d, which the read routine `0xffe865a8` also returns at once for a disabled direction) and marks that direction's state 1. So on classes 1, 2, 5 and 6 the ring-indicator input works as a link-presence line that gates the port, and the node links (class 3) are outside it. What the board wires to that pin is not in any image (unknown).

## 5. Where the variant code comes from (established)

`0xffe62e20` is the node-type discovery routine. It takes the brick letter from the byte at `0x300008cc` (the type chosen by the identity routine), and for letter `6` reads the identity record of four node boards: four iterations of the identity reader `0xffe8da88(route, buffer)` on routes 2, 11, 20 and 29 (start 2, step 9). For other letters (`0`, `7`, `8`, `9`, `Q`) it reads one record on route 4; any other letter stores `0xf` at `0x3000061a` and logs "Unknown Brick Type for Node Type Discovery". For each record it checks that the product name starts with `IP5` (`0xffe6481b`); a name that does not logs "Error: NODE %d invalid Product Name: %s" (source literal `IP5359.c`) and leaves that node's bit alone. Otherwise the fourth character decides the bit for node *n*: `3` clears bit `8 >> n` of the byte at `0x30000619` and `9` sets it. The routine returns that byte, which is the variant index of the 16-way table: node 0 is bit 3 and node 3 is bit 0, so variant 0 is 3333, variant 1 is 3339 and variant 15 is 9999, as the log strings are ordered. So the variant is the set of node boards, as their identity EEPROMs name them (`IP53` or `IP59`), not a setting. The initial value of the byte at `0x30000619`, which a node with an invalid name keeps, was not read (U). E for the routine; I that the routes name the four node boards' identity EEPROMs (the route table behind `0xffe8da88` was not read).

## 6. The supply enable bits (variant 3333, established)

The supply table the Cx board setup registers (32 records of 24 bytes at `0xffe56674` through `0xffe6be58`; the variant arrays other than 3333 were not compared) names these enables. Polarity is the L1's usual one (clear is on; the applied values `0xff`, `0xe8` and `0xc3` have the enable bits set, and no record has supply type `0x17`). Expander indexes are the positions in the array of section 3.

| Supply (type name) | Enable expander (index) | Mask |
|---|---|---|
| 48 V | `POWER ENABLE` (0) | `0x02` |
| 12 V, node 0 / 1 / 2 / 3 | `POWER ENABLE` (0) | `0x08` / `0x10` / `0x20` / `0x40` |
| VCPU, node 0 / 1 / 2 / 3 | `NODEn VCPU` (`0x10`, `0x1c`, `0x28`, `0x34`) | `0x80` |
| SRAM, node n | `NODEn CPU CTRL` (`0x14`, `0x20`, `0x2c`, `0x38`) | `0x02` |
| 1.5 V, node n | `NODEn CPU CTRL` | `0x01` |

`POWER ENABLE` bits `0x04` and `0x80` enable nothing here (`0x80` and `0x01` are input bits). The 3.3 V and 5 V auxiliary supplies, the node auxiliaries and the router's 12 V and 5 V records have no enable and are only monitored. The power-up order is the table order; the table's order and delays for the Cx-brick were not read (U). E for the table, I for the names of the enable expanders.

## Not settled

| Item | Why open |
|---|---|
| Part behind the form-1 register devices and the identity of the monitors | no image names it; the routines that consume the monitor table were not read |
| Variants 14 and 15 and the supply table of the other variants; the power-up order and delays for type `6`; the initial value of `0x30000619` | not decoded |
| What the fifth node-link channel and port 14 carry | the router or node mapping of SysCom channels was not read, see section 4 |
| Board wiring of the eight-channel part's modem-status inputs | not visible to the image |
| Second-level switch contents (the DIMM identification routes) | only the switch bytes were read |
