# L1 1.44.0 I/O expanders, the bus switch and the front-panel bits

The L1 reaches most board controls through byte-wide I/O expanders on its two-wire bus: power enables, resets, status inputs, LEDs, bus select, the front-panel display and its buttons. This note reads how the firmware describes those expanders and drives them, so that a machine hosting the image can answer the bus the way the firmware expects. Everything is stated as bus traffic: an address, a value written, a value tested.

Read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`, composite offset `0xefdd0`, body SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`), first program copy, for the Origin 300 (brick letter `4`). The tables of package 0 (`Base 1MB image`) are listed at the end from the same reading. Addresses are first-copy addresses; the second copy is `0x70000` higher with the same bytes. The transaction layer (start sequence, status handling, divider) is in [mbus.md](mbus.md) and [mbus-environment-task.md](mbus-environment-task.md).

## How the firmware describes an expander (established)

Each board registers an array of 16-byte descriptors at start-up; the Origin 300 one is registered by the routine at `0xffe45990` (17 descriptors at `0xffe45c90`). The registration routine allocates one 12-byte state record per descriptor and marks it enabled unless the descriptor's address byte is `0xff` (`0xffe6562c-0xffe656e6`). A descriptor reads:

| Byte | Meaning | Where it was read |
|---|---|---|
| +0 | access form: 0 is one data byte sent to the address and read from it, 1 a register pair, 2 register 5 | the switch in the write routine (`0xffe65974-0xffe65b3a`) and the read routine (`0xffe65bf4-0xffe65cb0`); every descriptor of the Origin 300 and of package 0 has form 0 |
| +1 | segment: which bus segment to select first (see below) | passed to the bus lock routine (`0xffe65b3c-0xffe65bac`) |
| +2 | wire address in the 8-bit form (the seven-bit address shifted left by one); `0xff` marks the entry absent | sent as the first byte; the two-wire layer sets the read bit for reads |
| +3 | register byte, used only by forms 1 and 2; 0 in every table here | the write routine |
| +4 | input bits: the bits the firmware only reads | below |
| +5 | second mask, used only by form 1; 0 in every table here | the write routine |
| +6 | value applied to the outputs at board start-up | `0xffe66234-0xffe66300` |
| +7 | input bits whose change is reported from start-up | `0xffe66234-0xffe66300` and `0xffe65e04-0xffe65f08` |
| +8 | hook called as (index, changed bits, new value) when the polling worker sees an input change; 0 for none | the poll loop at `0xffe66100-0xffe66232` |
| +12 | pointer to the entry's name | the log messages |

The state record holds: byte 0 enabled; byte 1 the shadow of the last value written; byte 2 the last value read; byte 3 the set of input bits whose change is currently reported.

**Writes (established).** A caller sets or clears bits by mask. The routine takes the bus lock with the descriptor's segment, replaces the masked bits of the shadow byte and sends the whole shadow byte, once, to the wire address. It never sends a partial byte. A mask that overlaps the input bits, or a value with bits outside the mask, is logged at level 9 and the write goes ahead. After the write the lock is released. A state record whose enabled byte is clear makes the call fail with minus 407 without any bus traffic.

**Reads (established).** One read of the wire address with the read bit set returns the byte; it is stored in state byte 2.

**Start-up application (established for the steps, inferred for the condition).** At registration time the routine at `0xffe4a48c` runs a power sanity check (`0xffe4a400`) and, when it returns nonzero, applies every enabled descriptor once (`0xffe66304`). For each one it forms (value at +6) OR (input bits at +4), stores that as the shadow, writes it, reads the device back into state byte 2, clears state byte 3 and enables change reporting for the bits at +7. The write is therefore a single byte whose input bits are all ones. What the sanity check's nonzero return means was not read, so whether this application is skipped on a restart of the L1 alone is not established.

**Input bits and the shadow (established).** The routine that reads every expander for the sanity check (`0xffe65d34-0xffe65e00`) stores into the shadow the byte it read ORed with the descriptor's input bits. Change reporting accepts only input bits: a request outside them is logged and cut down to them (`0xffe65e04-0xffe65f08`).

## Bus segments and the bus switch (established)

The segment byte is not a wire address. The board registers one bus-switch descriptor (`0xffe45c88`: form 0, address `0xee`, name `IP45`) and a select hook (`0xffe458c4`). The hook acts on the segment number as follows (`0xffe458c4-0xffe4598e`, `0xffe6a380-0xffe6a410`, table at `0xffe6a922`):

| Segment | What the L1 sends |
|---|---|
| 0 | nothing; the bus is used as it is |
| 1 | the expander at index 9 (`BUS SELECT`, wire address `0x4e`) is written `0xff` (its shadow is cleared first, so the write always happens), then one byte `0x00` goes to the switch at `0xee` |
| 2 | one byte `0x04` to `0xee` |
| 3 | one byte `0x05` to `0xee` |
| 4 | one byte `0x06` to `0xee` |
| 5 | one byte `0x07` to `0xee` |
| 6 to 9 | when the switch's remembered channel is nonzero (segments 2 to 5), one byte `0x00` to `0xee` first; then index 9 is written `0xf0`, `0xf1`, `0xf2` or `0xf3` for segments 6, 7, 8 and 9 (skipped when the shadow already holds that value) |
| other | an error is logged (`Illegal IP45 brick I2C bus select`) |

Segments 2 to 5 are therefore the four downstream channels of the switch (the usual enable bit `0x04` plus a two-bit channel), and segments 6 to 9 are the four DIMM identification EEPROM routes of [mbus.md](mbus.md), selected through the `BUS SELECT` expander. A repeated select of the same segment sends no switch byte, though segment 1 still rewrites the expander. The firmware never reads the switch back. Established for the bytes sent; inferred that the part at `0xee` is a four-channel bus switch (no image names it).

## The Origin 300 descriptors (established)

Table at `0xffe45c90`, read in full. "Applied value" is byte +6, "Interrupt-enabled bits" byte +7.

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 0 | `0x4c` | `0xf1` | `0xff` | `0x00` | yes | COM CONTROL |
| 1 | 0 | `0x4a` | `0x7f` | `0x7f` | `0x00` | - | PCI PRESENT |
| 2 | 3 | `0x42` | `0x20` | `0x72` | `0x00` | - | ASIC CTRL 1 |
| 3 | 3 | `0x44` | `0x02` | `0x8a` | `0x00` | - | ASIC CTRL 2 |
| 4 | 4 | `0x46` | `0x4f` | `0xff` | `0x43` | yes | POWER CONTROL |
| 5 | 4 | `0x44` | `0x80` | `0xe2` | `0x00` | - | POWER VCPU |
| 6 | 4 | `0x42` | `0x00` | `0xeb` | `0x00` | - | POWER 1.5V |
| 7 | 4 | `0x48` | `0xff` | `0xff` | `0xff` | yes | POWER VRM |
| 8 | 3 | `0x46` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 9 | 0 | `0x4e` | `0x00` | `0xff` | `0x00` | - | BUS SELECT |
| 10 | 5 | `0x42` | `0x00` | `0xd4` | `0x00` | - | XBRIDGE CONTROL |
| 11 | 2 | `0x42` | `0xff` | `0xff` | `0x00` | - | LED0 LOW |
| 12 | 2 | `0x44` | `0xff` | `0xff` | `0x00` | - | LED0 HIGH |
| 13 | 2 | `0x46` | `0xff` | `0xff` | `0x00` | - | LED1 LOW |
| 14 | 2 | `0x48` | `0xff` | `0xff` | `0x00` | - | LED1 HIGH |
| 15 | 3 | `0x48` | `0x0f` | `0xff` | `0x00` | - | PIMM PSC |
| 16 | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

The names are the firmware's own labels, written into its log lines. The sections below say what the code does with some of them.

## What the firmware drives and tests on the Origin 300

Each line is a write or a test the code makes with literal arguments; indexes are those of the table above. Bits are counted from 0 (mask `0x01`).

- **Reset pulse (established).** Index 2 (`ASIC CTRL 1`) mask `0x80` is set, the L1 waits 10 task-clock units, and the bit is cleared (`0xffe45084-0xffe450dc`). The routine returns an error without touching the bus when the byte at `0x3000009c` still holds `0xff`.
- **Second reset line (established).** Index 2 mask `0x40` is cleared and then set (`0xffe450e0-0xffe4512c`), under the same guard.
- **Node enable sequence (established).** The routine at `0xffe453e4-0xffe45486` makes these writes in order and stops at the first bus error: an update of the expander whose index is the byte at `0x3000009c`, mask `0x0b`, value 0; index 2 mask `0x80` cleared; index 2 mask `0x02` cleared; index 3 mask `0x01` cleared; index 2 mask `0x01` cleared; and then a helper (`0xffe45348`) called for index 4 with the value `0x08`, which for processor-module codes 1 to 4 first runs a routine that was not followed, waits 10 units, and sets index 4 mask `0x10` and then mask `0x20`.
- **Power-up (established for the polled bit and the tail).** For brick letter `4` the power-up routine (`0xffe4c79c-0xffe4ce82`) enables the supplies whose records name an expander, waits for index 4 (`POWER CONTROL`) mask `0x40` to read nonzero (repeating a 20-unit wait, `0xffe4c984-0xffe4c996`), then reads four supply voltages and repeats the wait and the reads until all four are inside a band around their nominal value (the band is the nominal plus and minus 0.2 times the nominal, inferred from the constant 0.2 and the add and subtract calls), then enables the remaining supplies. After the last one it clears index 10 (`XBRIDGE CONTROL`) mask `0x40` and sets index 2 mask `0x08` (`0xffe4cc8c-0xffe4cca4`). A machine therefore has to return `POWER CONTROL` with bit 6 set once its supplies are enabled, and in-band supply readings, or the L1 stays in that wait.
- **Processor-module probe (established).** Index 15 (`PIMM PSC`) is read once at registration (`0xffe4520c`); the byte is decoded into a code `1` to `5` or left `0xff` and stored in a RAM byte (`0xffe4514c-0xffe451fa`); an undecodable value logs `UNKNOWN PSC CODE`. A model that returns `0xff` takes the same path as an absent module.
- **Bridge and PCI expanders (established).** Index 1 (`PCI PRESENT`) is only read. Index 10 (`XBRIDGE CONTROL`) mask `0x24` is written by the routine at `0xffe4552c`, and index 3 mask `0xf8` is written with a value built from the settings with numbers 40 and 41: bit `0x08` always, `0x20` when setting 40 is nonzero, `0x10` when setting 41 is nonzero (`0xffe45604-0xffe45644`, setting numbers as in [settings-record.md](settings-record.md)).

### Supplies (established)

The Origin 300 registers ten supply records at `0xffe45e0c`, 24 bytes each (`0xffe45990`). Byte +0 indexes a table of 30 names and nominal voltages at `0xffe4e334` (12 bytes per entry, nominal as an IEEE single in the first word), +2 and +3 name the enable expander and mask (`0xff` for none), +4 and +5 the monitor and its channel.

| Record | Nominal entry | Enable (expander index, mask) | Monitor, channel | Name in the record |
|---|---|---|---|---|
| 0 | 12 V | 4, `0x80` | none | none |
| 1 | 12 V | none | 0, 8 | `12 IO` |
| 2 | 12 V | none | 1, 8 | `12 DIG` |
| 3 | 5 V | none | 1, 3 | none |
| 4 | 3.3 V | none | 1, 2 | none |
| 5 | 5 V | none | 0, 3 | `5V AUX` |
| 6 | 3.3 V | none | 0, 2 | `3.3V AUX` |
| 7 | 2.5 V | 5, `0x40` | 0, 0 | none |
| 8 | `VCPU` (nominal 0) | 5, `0x20` | 0, 9 | none |
| 9 | 1.5 V | 6, `0x20` | 0, 1 | none |

The enable routines (`0xffe4c3bc-0xffe4c430`, `0xffe4c4e4-0xffe4c550`) clear the mask on that expander to enable a supply and set it to disable, unless the record's first byte is `0x17`; none of these ten is. The four enabling records are the ones the power-up routine switches. Which physical supply each switches is not named by the image (inferred only from the nominal entry).

## Front-panel display expander

Index 16 (`DISPLAY CONTROL`, address `0x40`, segment 0, input bits `0x0f`, interrupts enabled on bits `0x07`):

- **Probe (established).** The display routine (`0xffe50eb8-0xffe50fe4`) reads index 16 once. A bus error logs `Error reading the display I/O expander` and abandons the display; a successful read records the expander index in a RAM byte.
- **LED bits (established for the bits, inferred for polarity).** The three LED routines (`0xffe513c8`, `0xffe51480`, `0xffe51534`) drive three output bits of this expander: LED 0 is mask `0x40`, LED 1 `0x10` and LED 2 `0x20`. The state query returns 1 when the shadow bit is clear and 0 when it is set, so a clear bit is the lit state (inferred from the query, not from a board).
- **Input bits (established for the hook, inferred for the meaning).** The change hook (`0xffe5178c`) tests bits 0 and 1 of its changed-bits argument. A change of bit 0 to 1 (after an inversion when bit 1 of the byte at `0x30001c78` is set) runs the power-down routine when the node is on and the power-up routine when it is off; setting 119 can make it refuse and print a message on the display. Bit 1 reaches a second, longer decision that was not followed. Bit 0 is the front-panel power button, inferred from the two routines it calls.
- **Display text.** Characters go out through a separate driver (`0xffe5117c` and the routines it reaches) that was not decoded: the write sequence and the busy handling are unknown here.

## Package 0 tables (established)

The same reading of package 0 (`Base 1MB image`, brick letters `CIPXYN&?RG`) finds eight descriptor arrays, all of form 0. Which brick letter each belongs to is not stated next to the arrays; the descriptions give the names in each and the identity EEPROM records registered beside it.

Array at `0xffe455dc` (17 entries, names `BEDROCK CNTL 1`, `PIMM0 POWER`, `PIMM1 VCC OK`, `FAN CONTROL`; identity records `NODE` at `0xa4`, `PIMM 0` and `PIMM 1` at `0xa8` on segments 4 and 5):

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 0 | `0x4e` | `0x00` | `0xff` | `0x00` | - | BUS SELECT |
| 1 | 2 | `0x42` | `0xff` | `0xff` | `0x00` | - | LED0 LOW |
| 2 | 2 | `0x44` | `0xff` | `0xff` | `0x00` | - | LED0 HIGH |
| 3 | 2 | `0x46` | `0xff` | `0xff` | `0x00` | - | LED1 LOW |
| 4 | 2 | `0x48` | `0xff` | `0xff` | `0x00` | - | LED1 HIGH |
| 5 | 2 | `0x4c` | `0xf1` | `0xff` | `0x00` | yes | COM CONTROL |
| 6 | 3 | `0x42` | `0x00` | `0x72` | `0x00` | - | BEDROCK CNTL 1 |
| 7 | 3 | `0x44` | `0x00` | `0x8a` | `0x00` | - | BEDROCK CNTL 2 |
| 8 | 3 | `0x46` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 9 | 3 | `0x48` | `0xde` | `0xff` | `0xde` | yes | POWER CONTROL |
| 10 | 3 | `0x4a` | `0xff` | `0xff` | `0x00` | - | PIMM PSC |
| 11 | 3 | `0x4c` | `0x40` | `0xff` | `0x40` | yes | FAN CONTROL |
| 12 | 4 | `0x44` | `0x36` | `0xff` | `0x36` | yes | PIMM0 POWER |
| 13 | 4 | `0x42` | `0x3c` | `0xff` | `0x00` | yes | PIMM0 VCC OK |
| 14 | 5 | `0x44` | `0x36` | `0xff` | `0x36` | yes | PIMM1 POWER |
| 15 | 5 | `0x42` | `0x3c` | `0xff` | `0x00` | yes | PIMM1 VCC OK |
| 16 | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

Array at `0xffe46828` (6 entries, `PCI BUS1 SLOT1` to `SLOT4`; identity records `POWER` `0xa4`, `LOGIC` `0xa8`, both segment 0):

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 0 | `0x46` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 1 | 0 | `0x48` | `0x00` | `0xd4` | `0x00` | - | XBRIDGE CONTROL |
| 2 | 0 | `0x4a` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT1 |
| 3 | 0 | `0x4c` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT2 |
| 4 | 0 | `0x4e` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT3 |
| 5 | 0 | `0x42` | `0x7f` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT4 |

Array at `0xffe46ae4` (4 entries; identity records `POWER` on segment 0 and `LOGIC` on segment 2):

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 2 | `0x46` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 1 | 2 | `0x48` | `0x00` | `0xd4` | `0x00` | - | XBRIDGE CONTROL |
| 2 | 2 | `0x4a` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS3 SLOT1 |
| 3 | 2 | `0x4c` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS3 SLOT2 |

Array at `0xffe46b54` (8 entries, PCI slot names for buses 1, 2, 5 and 6):

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 3 | `0x42` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS5 SLOT2 |
| 1 | 3 | `0x44` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS6 SLOT1 |
| 2 | 3 | `0x46` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS6 SLOT2 |
| 3 | 3 | `0x48` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT1 |
| 4 | 3 | `0x4a` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT2 |
| 5 | 3 | `0x4c` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS2 SLOT1 |
| 6 | 3 | `0x4e` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS2 SLOT2 |
| 7 | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

Arrays at `0xffe46e44` and `0xffe47754` (4 entries each, `XIO CONTROL`; identity records `POWER` with `LOGIC` and with `XLOGIC` respectively):

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 0 | `0x46` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 1 | 0 | `0x48` | `0x00` | `0xd4` | `0x00` | - | XBRIDGE CONTROL |
| 2 | 0 | `0x4a` | `0xf8` | `0xf8` | `0x78` | yes | XIO CONTROL |
| 3 | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 0 | `0x46` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 1 | 0 | `0x48` | `0x00` | `0xd4` | `0x00` | - | XBRIDGE CONTROL |
| 2 | 0 | `0x4a` | `0xf8` | `0xf8` | `0x78` | yes | XIO CONTROL |
| 3 | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

Array at `0xffe4a3cc` (7 entries: `WIDGET PRESENT`, `ROUTER CONTROL` and five `PORT ... CONTROL`; identity records `POWER`, `LOGIC`):

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 0 | `0x44` | `0x00` | `0x00` | `0x00` | yes | WIDGET PRESENT |
| 1 | 0 | `0x46` | `0x00` | `0xf1` | `0x00` | yes | ROUTER CONTROL |
| 2 | 0 | `0x48` | `0x55` | `0xd5` | `0x11` | yes | PORT HA CONTROL |
| 3 | 0 | `0x4a` | `0x55` | `0xdd` | `0x11` | yes | PORT BC CONTROL |
| 4 | 0 | `0x4c` | `0x55` | `0xdd` | `0x11` | yes | PORT DE CONTROL |
| 5 | 0 | `0x4e` | `0x55` | `0x5d` | `0x11` | yes | PORT FG CONTROL |
| 6 | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

Array at `0xffe57e5c` (4 entries; one identity record `ID` at `0xa4`):

| Index | Segment | Wire address | Input-bit mask | Applied value | Interrupt-enabled bits | Change hook | Name |
|---|---:|---|---|---|---|---|---|
| 0 | 0 | `0x42` | `0xff` | `0xff` | `0x0f` | yes | POWER STATUS |
| 1 | 0 | `0x44` | `0x03` | `0xff` | `0x00` | - | RESET CONTROL |
| 2 | 0 | `0x46` | `0x00` | `0xff` | `0x00` | - | POWER CONTROL |
| 3 | 0 | `0x40` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

Other identity routes of package 0, all 512-byte form-1 records: `POWER` `0xa4` with four `NULL` records at `0xac` on segments 4 to 7 (two arrays; the second has a `MNIPWR` record at `0xa8` instead of `LOGIC`). The Origin 300's own routes are in [mbus.md](mbus.md).

## What this does not settle

- The meaning of each output bit beyond the sequences above. The firmware's labels name the expander, not the pin, and no Origin 300 path tests every bit of an expander.
- The polarity of the LED and supply-enable bits on a real board.
- The expander and switch parts. Wire addresses `0x40` to `0x4e` in eight-bit form are seven-bit addresses `0x20` to `0x27`, the range of the common eight-bit expanders, but no image names one.
- The display text driver and the meaning of the second display input bit.
- Package 2 (`MIPS 2MB image`) holds the package 0 arrays at shifted addresses plus more for other bricks; packages 3 and 5 hold no array of this form. Package 2 was found by the same byte pattern and its entries were not tabulated.
