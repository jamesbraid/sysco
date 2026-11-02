# Power-bay serial link: framing, commands and flash sequence

Reads the Origin 300 L1 1.44.0 body (`Fuel/PE/O300 1MB` package, listing in [disasm/](disasm/README.md), the code that the 6.5.30 `l1.bin` carries as package 1) against `pbay.bin`, the 5,888-byte power-bay controller image (sha256 `7428aaf3d5beffcb23fb56aa9f82c013a6f3ebbb5f41175a61781bfcf366f9c6`, identical in every IRIX set; labelled `pbay-00.18.bin` in the 1.62.1 RPM). The image was decoded with a table-driven MCS-51 decoder written for this pass (scratch tooling, not committed); the 8051 family is **inferred** (see [../controllers.md](../controllers.md)). Grades: E established (routine read in full), I inferred, U unknown. L1 addresses are first-bank (`0xffe00000` base).

## Image layout (E from the bytes, I for the load address)

- Bytes `0x00-0x1f` are a header, not code: bytes 0-1 `17 00` (the file length is `0x1700`, so a 16-bit length field), bytes 2-3 `d5 da` (E: equals the 16-bit sum of bytes `0x04` to the end, big-endian), then an ASCII author banner and, in the last three bytes, `56 00 12` (`V`, then the version bytes `00 12` in file order; the `0xc0` reply sends the same version as `12 00`). The version matches the `00.18` in the RPM label (I).
- **Load address `0x4000` (I).** 306 of 306 absolute call and jump targets that land in `0x4000-0x56ff` hit a non-`0xff` byte with that base, and the `0xc0` reply (below) reports the same version and a chunk size that fits whole-file flashing. Under this base the file is the flash content from controller code address `0x4000`; code below it (a boot loader holding the serial driver, a transmit entry at `0x1500` that the frame trailer jumps to) is **not in the image** (E that the app ends its frames with `LJMP 0x1500`).
- `0x4700` holds `LJMP 0x55a3` and `LJMP 0x557f` (E); they look like a relocated reset and INT0 pair. The app dispatcher is at `0x4706`; handlers and tables follow to `0x5690`. Timer-0-shaped routines at `0x4300` and `0x4500` have no caller in the image (I: called by the loader).
- Build time reported by command `0xc0`: epoch `0x39d4a060` = 2000-09-29 14:00:00 UTC (E for the bytes, I that the field is a Unix time).

## Transport (E, both ends read)

L1 side: transaction routine `ffe3ddac`, task init `ffe3e470`. The task opens software UART port 1 with client mode 2, read and write (`ffe648bc(1, 2)` at `ffe3e4e2`; the first argument is the port, the second the open mode, E). Port 1 is the processor's on-chip UART 1 (see [The L1 UART handler](#the-l1-uart-handler-framing-address-filter-and-direction-control), E for the registration, I for the chip-level identification). The task fills a settings block with **19200 baud** (`0x4b00`), the L1's own 16-bit address, 8 data bits without parity, no flow control and handler mode value 2, and applies it with `ffe643e8` (E, `ffe3e520-ffe3e554`). It runs a request/response transaction per command with a bounded retry (a count in the context) and a receive-attempt count. Command `0x0b` brackets the transaction with a change to settings offset `+0xc` (set to 3 at `ffe3de8e-ffe3de9c`, back to 0 at `ffe3dee2-ffe3deee`; E); what the value 3 does in the handler is below.

**Framing (ICMB-style, I that it follows the public ICMB byte stuffing).** On the wire a frame is the start byte `0xa0`, the body with its checksum, each `0xa0`, `0xa5` or `0xaa` replaced by `0xaa 0xb0`, `0xaa 0xb5` or `0xaa 0xba`, and the end byte `0xa5`. The bay application stuffs its replies and ends them with `0xa5` before jumping to the loader transmit entry (E, tests at `0x54d6-0x54ec`, repeated for the checksum bytes at `0x552f-0x556b`; the transmit routines at `0x54d2` and `0x5509`). The bay's start byte is written by the loader (U: not in this image; I that it is kept in `iram50`, see Buffer aliasing). The L1's transaction routines build and parse unframed, unstuffed bodies (`ffe3f404`, `ffe3f378`, `ffe642d0`, `ffe64bac`; no `a0`, `a5`, `aa` constants), and the L1's own start byte, stuffing, end byte, unstuffing and destination filter are in its UART interrupt handler for the port in mode 2 (E, next section). There is no separate framing board on the L1 side.

**Frame body (E).** Little-endian 16-bit fields, offsets from the first destination byte:

| Offset | Size | Meaning |
|---|---|---|
| 0 | 2 | destination address (L1 builds it at `ffe3f404`; app reads `00 44` for itself, i.e. `0x4400`) in app bytes `iram51/52` |
| 2 | 2 | source address (`iram53/54`) |
| 4 | 1 | zero (`iram55`) |
| 5 | 1 | control: bits 0-4 a tag, bit 7 set on a reply (L1 accessors `ffe3e9c4`, `ffe3ea00`; app `ORL #0x80` at `0x54be`) (`iram56`) |
| 6 | 1 | command code (`iram57`) |
| 7 | n | request: data (`iram58` on) |
| 7 | 1 | reply: **completion code** (app sends `iram41`; the L1 requires 0, `ffe3df9a`) |
| 8.. | n | reply: data |
| last 2 | 2 | checksum |

The app reads request field `iram51+k` for frame offset k, and assembles its reply in
the same bytes starting at `iram51` (`0x5498` loads `R0 = 0x50` and every send at `0x54d2`
pre-increments it), so a handler must copy what it needs before it writes.

**Checksum (E, both ends).** Sum all bytes before the checksum, add the byte count of that sum, add one, and send `0xffff` minus that 16-bit total, low byte first (L1 `ffe3f314` and `ffe3f404`; app `0x54d2` counting `iram43`, `0x5509` adding the count and 1 and complementing). The L1 receiver (`ffe3f378`) accepts a frame when the checksum and the received complement sum to `0xffff`. A request whose destination bytes are not `00 44` is ignored unless the command is `0x02` or `0xc0` (E, `0x4706-0x4727`); a command not in the table takes the `0x5448` path (header-only reply, completion code `0xc1`); a status `0xff` left by the loader in `iram41` takes the `0x5451` path (completion code `0xff`).

## Link commands (the app's dispatch table at `0x4758`, handlers at `0x4763`)

| Code | Handler | What it does (E unless marked) | L1 use |
|---|---|---|---|
| `0x02` | `0x4777` | reply carries its own address `00 44` and echoes `iram53/54`; answers regardless of destination | init step 2: the reply's 16-bit value becomes the bay address; zero means no bay (state 5, "communication failure") and `0x4400` is then used |
| `0x0a` | `0x47be` | fixed three-byte reply `01 08 02` (I: device or protocol revision) | sent in `ffe3d2a8` |
| `0x0b` | `0x47db` | strobes `P1 = 0x9c` with `P3.5` low then high, sets flag `iram20.2`, then replies `iram34` mapped from a one-hot code to its bit number (8 bits), after a delay loop | bracketed with the transmit-hold setting (settings offset `+0xc` = 3) |
| `0x10` | `0x4862` | no reply (returns unless `iram41 == 0xff`) | sent at init to address 0 |
| `0x11` | `0x486b` | reply with sixteen zero bytes | none found |
| `0x12` | `0x48c9` | reply with header only (I: assign address) | init step 3, argument is the bay address |
| `0x13` | `0x48d7` | reply one byte `0x22` (I: device type) | init step 4; the byte is stored and later passed back in each tunnelled request |
| `0x20` | `0x48ea` | **tunnelled request**: byte 8+1 (`iram59`) is a network-function field in IPMI form (value already shifted left two) and `iram5a` the command; replies are built per pair, below | the main data path |
| `0xc0` | `0x51a9` | reply `12 00 60 a0 d4 39 80`: version `0x0012`, build time `0x39d4a060`, **flash chunk size `0x80`** | flash task request `0x92` and the `pbay version` print |
| `0xa2` | `0x5447` | `RET` only (no reply) | none |

`0xc1` and `0xc2` are sent by the L1 (below) and are **not in this table**: the boot loader below `0x4000` answers them (I).

### Tunnelled `0x20` pairs (E from the compare chain at `0x48f2-0x493b`)

Field one is `iram59`, field two `iram5a`; the IPMI names are a hint, not read in the image (public IPMI v1.5 command numbering matches every pair).

The data of a `0x20` request is an IPMI-shaped inner message built by the L1 at `ffe3eae8` (inside the outer frame from `ffe3f404`) and the reply data is its response (parsed at `ffe3dfb6`-`ffe3dffc`):

| Frame offset | Request | Reply |
|---|---|---|
| 7 | inner byte 0 (`iram58`): `0x22` (the `0x13` reply byte at init, the sensor record's owner byte later) | link completion code (`iram41`), 0 |
| 8 | inner byte 1 (`iram59`): network function << 2, unit in bits 0-1 (L1 sends unit 0) | copy of request offset 7 (the "echo", saved in `iram45`) |
| 9 | inner byte 2 (`iram5a`): command | network function byte + 4 (`0x04 0x14 0x1c 0x2c`, `iram46`) |
| 10 | first data byte (`iram5b`) | command |
| 11 | second data byte (`iram5c`) | inner completion code (`iram42`, a copy of `iram41` at entry), 0 |
| 12.. | third data byte on | data |

An earlier version of this note put the first data byte at offset 8 and called the fourth reply byte the request's first data byte; the fourth byte is the inner completion code and a link completion code precedes the four. The L1 does not read the echo or either address; it needs the link and inner completion codes 0, tag and command equal, the response bit (bit 2 of the network function byte) set, and the network function (that byte shifted right two, response bit dropped) equal to the request's (E, `ffe3e016`). On the status-`0xff` path (`0x545a`) `iram45` is not loaded, so the echo is a stale byte.

| Pair | Handler | Behaviour |
|---|---|---|
| `0x00,0x00` | `0x4941` | fixed reply `00 22 22 00 00` after the four leading bytes (whole reply `<echo> 04 00 id 00 22 22 00 00`) (I: capabilities) |
| `0x10,0x23` | `0x4977` | jumps to `0x5448`, the unknown-command reply: header only, completion code `0xc1` |
| `0x10,0x2d` | `0x497a` | **sensor reading**: `iram5b` (first data byte) selects 1-6 (select codes `0x84,0x80,0x90,0x8c,0xa4` strobed on `P1`/`P3.5` by `0x4a33`) or 7-14 (codes `0x94,0x98` by `0x4a82`), 15 triggers the `0x4afb` sequence; reply data `00 00 <iram27> 00` (`iram28` is cleared and never set); other numbers take the `0x5448` reply; see Sensor reads below |
| `0x18,0x03` | `0x4b1f` | action: sets `iram20.7` and strobes `P1 = 0x98`, `0x94`, `0x09`, `0x00`, `0x80`, `0xa4`, `0x11`, `0x0a` in a fixed sequence with delays (I: warm reset or DC-port control) |
| `0x18,0x04` | `0x4c76` | reply `0x56 0x00` (I: self test not implemented) |
| `0x28,0x10` | `0x4c9d` | reply data copies `iram5b-5e` while those bytes are being overwritten by the reply (offsets 10-11 hold the reply's command and inner completion code), so the data is `10 00 10 00` for every request |
| `0x28,0x11` | `0x4cce` | maps `0xa0,0xa2,...,0xae` (`iram5b`) to 0-7 (other values take the `0x5448` reply), strobes `P1` with `0xac`, `iram5b`, `iram5c`, the index, `iram5e` and sets flags; the reply is streamed later, see below (I: a FRU read) |
| `0x28,0x20` | `0x4dcf` | fixed reply `01 0c` then twelve zeros, after the four leading bytes |
| `0x28,0x23` | `0x4e32` | **record read**, data = reservation (2), record id (2), offset, count; keyed by `iram5d` (record id low byte) in the ranges 0-1, 2, 3, 4-9, 10, 11, 12 (other keys take the `0x5448` reply): returns up to `iram60` bytes (default `0x20`, `0x10` or `0x30`) from the fixed tables at `0x4eb2`, `0x4f1f`, `0x4f8c`, `0x5065`, `0x50bf`, `0x511c`, `0x5179` (I: sensor data records and FRU data; the L1 prints the multirecord power-supply information and DC output fields from them) |

The `P1`/`P3.5` pairs write a command byte to a latch outside the MCU: `P1` carries the byte and `P3.5` pulses low then high (E for the pattern, 45 uses). What the latch is (U); the previous reading of the vector files points at CPLDs on the bay boards but names no register.

## Exchanges, records, sensors and errors

### Worked tunnelled frames

Frame bodies with checksum (no start byte, before stuffing), L1 address
`0x0102`, tag 5 (control `0x05` in the request, `0x85` in the reply), inner
target byte `0x22`. Every reply below was produced by running the bay
application's handlers from `pbay.bin` in a small MCS-51 interpreter and its
checksum recomputed from the rule above; this is **E** for the handler logic and
**I** for the interpreter's agreement with the part.

| Exchange | Request | Reply |
|---|---|---|
| `0x00,0x00` | `00 44 02 01 00 05 20 22 00 00 66 ff` | `02 01 00 44 00 85 20 00 22 04 00 00 00 22 22 00 00 97 fe` |
| `0x18,0x04` | `00 44 02 01 00 05 20 22 18 04 4a ff` | `02 01 00 44 00 85 20 00 22 1c 04 00 56 00 6c fe` |
| `0x28,0x20` | `00 44 02 01 00 05 20 22 28 20 1e ff` | `02 01 00 44 00 85 20 00 22 2c 20 00 01 0c` + 12 x `00` + `7d fe` |
| `0x28,0x23` key 0, count 5 | `00 44 02 01 00 05 20 22 28 23 00 00 00 00 00 05 10 ff` | `02 01 00 44 00 85 20 00 22 2c 23 00 02 00 01 00 01 02 1b 6d fe` |
| `0x28,0x23` key 12, count 5 | `00 44 02 01 00 05 20 22 28 23 00 00 0c 00 00 05 04 ff` | `02 01 00 44 00 85 20 00 22 2c 23 00 ff ff 0c 00 01 01 2c 56 fc` |
| `0x10,0x2d` sensor 1, input snapshot `0x01` | `00 44 02 01 00 05 20 22 10 2d 01 27 ff` | `02 01 00 44 00 85 20 00 22 14 2d 00 00 00 0f 00 90 fe` |
| `0x28,0x23` key 4, count `0xff` | `00 44 02 01 00 05 20 22 28 23 00 00 04 00 00 ff 12 fe` | `02 01 00 44 00 85 20 00 22 2c 23 00 05 00 04 00 01 11 0b 22 a0 80 00 00 10 00 0a 01 00 00 00 fd` |

The last reply contains `a0`, so on the wire the bay writes `aa b0` for it
(`... 22 aa b0 80 ...`), with the checksum `00 fd` computed over the unstuffed
bytes, then `a5`. The request for the same exchange on the wire, with the
loader's start byte (**U** for who sends it, see Receive rules): `a0 00 44 02 01
00 05 20 22 28 23 00 00 04 00 00 ff 12 fe a5`. The bay's reply also shows that
the L1 address in the reply destination is copied from the request source, and
the reply source is the request destination (not a fixed `00 44`; only link
command `0x02` replies with a fixed `00 44`).

### Record reads

Request data (offset 10 on): reservation (2 bytes), record id (2), offset, count.
The bay uses **only** the record id low byte (offset 12) as the key and the count
(offset 15); the reservation, record id high byte and offset are ignored
(**E**, `iram5d` and `iram60`). Reply data: next record id (2 bytes, little-endian),
then the record bytes. Keys map to records by range: keys 0 and 1 give the same
record, 2 gives record 2, 3 gives 3, 4 to 9, 10, 11, 12 give 4 to 12 and any other
key (including 13 and above) gets the error reply. The record's own id is its
first two bytes. Full records (count `0xff`; bytes shown are the record, not the
next id):

| Key | Next id | Record id | Length | Record bytes after the id |
|---|---|---|---|---|
| 0, 1 | `0002` | 1 | 32 | `01 02 1b 22 00 07 15 01 00 03 09 71 00 00 00 00 03 00 c0 00 00 08 81 00 00 00 00 00 00 00` |
| 2 | `0003` | 2 | 32 | `01 02 1b 22 00 01 0a 01 00 03 08 70 00 00 00 00 1f 00 c0 00 00 06 81 00 00 00 00 00 00 00` |
| 3 | `0004` | 3 | 16 | `01 12 0b 22 01 cb 00 00 00 00 15 01 00 00` |
| 4 to 9 | key+1 | key | 16 | `01 11 0b 22 <a0 + 2(key-4)> 80 00 00 10 00 0a <key-3> 00 00` |
| 10 | `000b` | 10 | 16 | `01 11 0b 22 ac 80 00 00 10 00 15 01 00 00` |
| 11 | `000c` | 11 | 16 | `01 11 0b 22 ae 80 00 00 10 00 b0 01 00 00` |
| 12 | `ffff` | 12 | 48 | `01 01 2c 22 00 0f 14 09 00 03 03 01 00 00 00 00 00 00 00 05 00 01 01 02 00 0b 08 00 00 00 00 00 ff 00 ... 00` (the header's length byte says 49; the table holds 48) |

The records read as IPMI sensor data records (**I**: header id, version, type,
length, then owner `22`, unit 0, sensor number): record 1 is eight sensors
numbered from 7 (entity `0x15`), record 2 six sensors numbered from 1 (entity
`0x0a`, sensor type 8, discrete mask `0x001f`), record 3 a controller locator,
records 4 to 11 FRU device locators with I2C-style addresses `a0` to `ae`, and record
12 a full sensor. The count of 12 matches the repository-info reply
(`01 0c`). The L1 starts at id 0, reads five bytes to learn the record id,
reads the record with count `0xff` and a second key equal to that id, takes the next
id from the first two reply data bytes and stops when it is `ffff` (**E**,
`ffe3e054`).

Count rules (**E**, by running the handlers). The count is clamped by the table,
not the record; `0xff` means the table size:

| Keys | Count `c` (not `0xff`) returns | `0xff` returns |
|---|---|---|
| 0, 1, 2 | `c` table bytes for `1 <= c <= 0x20`, otherwise nothing | 32 bytes |
| 3, 10, 11 | `c` bytes for `1 <= c <= 0x10`, otherwise nothing | 16 bytes |
| 12 | `c` bytes for `1 <= c <= 0x30`, otherwise nothing | 48 bytes |
| 4 to 9 | the key byte, then `c-1` bytes for `1 <= c <= 0x10`; the key byte alone for `c = 0` or `c > 0x10` | 16 bytes |

"Nothing" is an **empty body**: only the two next-id bytes. The L1 sends only
counts 5 and `0xff`, so an empty body occurs only for a malformed request. The
L1 does not check the body length (**E**, `ffe3e13e`-`ffe3e14a`): given an
empty body it would take the record id from stale buffer bytes.

### Sensor reads

Request data: sensor number (offset 10). Reply data: `00 00 <mask> 00`, where
`<mask>` bits 0 to 4 are the five inputs of the group and the last byte is
always 0 (**E**: the second mask byte is cleared and never set). Each input is
read by writing a select code to `P1`, pulsing `P3.5` low then high, releasing
`P1` to inputs and waiting for the external-interrupt routine at `0x4500` to
store a snapshot of `P1` in `iram34`; the handler then ANDs the snapshot with a
mask (**E**, handlers at `0x4a33`, `0x4a82`, `0x4aa1`):

| Sensor | Select codes (one input each, bit 0 first) | Mask applied to the snapshot |
|---|---|---|
| 1 to 6 | `84`, `80`, `90`, `8c`, `a4` (bits 0 to 4) | `1 << (n-1)` for the first four; always `04` for `a4` |
| 7 to 14 | `94`, `98` (bits 0 and 1; bits 2 to 4 stay 0) | `1 << (n-7)` |
| 15 | writes `a8`, replies later (below) | none |
| 0, 16 and above | error reply | none |

A bit is 1 when the masked snapshot is nonzero. What the pins carry (**U**) is
set by the bay's boards; the L1 reads them as follows (**E** for the L1's use,
**I** that the sensor numbers are fixed by records 1 and 2 above): for sensors 1
to 6 (one per supply) bit 0 set means the supply is not inserted, and bits 1, 2
and 3 set mean fail, predictive fail and AC fail, which the polling task logs on
a 0-to-1 change; for sensors 7 to 14 (one per DC port) bit 0 set means not
present (the port is skipped) and bit 1 set means "DCport fail". A model returns
a chosen mask per sensor; bits 4 of sensors 1 to 6 follow the fixed mask `04`
and are not read by the L1.

Sensor 15 does not reply from the handler. It sets a flag and writes `a8`; the
next time the interrupt routine runs the bay sends `<echo> 14 2d 00 <snapshot> 00`
as the data (**E**, `0x55a3`). The L1 never requests it (**E**: its sensor
numbers come from the records), so it is low priority.

### The streamed read of pair 0x28,0x11

Request data: device byte (offset 10, one of `a0 a2 a4 a6 a8 aa ac ae`), two
more bytes (offsets 11 and 12, written to the latch; offset 12 is first
replaced by the device's index 0 to 7), and a count (offset 13). The handler
returns nothing at once: it writes `ac`, the device byte, offsets 11, 12 and the
count to `P1` with `P3.5` strobes and sets a pending flag. Each later interrupt
sample (`iram34`) then appends one byte: the first call sends the frame header,
the reply prefix (`<echo> 2c 11 00`), the count byte and the first sample, and
each following call appends one sample until `count` samples are in, after which
the bay appends the checksum and `a5`. Example (count 4, samples `11 22 33
44`): reply `02 01 00 44 00 85 20 00 22 2c 11 00 04 11 22 33 44 f4 fd`. The bytes
are whatever the device returns (**U**; the device is presumably a FRU EEPROM per
device byte, **I**), and a count of 0 streams 256 bytes (the counter wraps). This
is how the L1 gets the power-supply multirecord data, so a model needs its own
FRU contents.

### Error replies

Every error is a header-only reply (no inner prefix, no data), with the control
tag copied, bit 7 set and the command echoed (**E**, tested by running the
application):

| Cause | Reply completion code |
|---|---|
| link command not in the table (including `0xc1`, `0xc2` if they ever reached the application) | `0xc1` |
| tunnelled pair not in the table, `0x10,0x23`, sensor number 0 or 16 and above, record key 13 and above, `0x28,0x11` device byte not in the list | `0xc1` |
| frame status `0xff` set by the loader (see below) on a command in the table, other than `0xa2` | `0xff` (link commands answer header-only; a tunnelled request answers with the prefix `<stale byte> <netfn+4> <cmd> ff` and no data; a tunnelled request whose network-function byte is not `00 10 18 28` gets silence) |
| frame status `0xff` on a command not in the table | `0xc1` (the unknown-command path overwrites the status) |
| destination not `00 44` (except commands `0x02` and `0xc0`) | none (silence) |
| commands `0x10` and `0xa2` (with status `0xff`, `0x10` answers header-only `0xff`; `0xa2` stays silent) | none (silence) |

The application reads a status byte left by the loader (`iram41`): `0x00` in
the normal case (**E**: the L1 requires a zero link completion code), `0xff`
when the loader flags the frame; the loader's rule for `0xff` is **U** (a bad
checksum is the likely cause, **I**). The application never checks a frame's
length: a request shorter than a command needs is answered from stale buffer
bytes.

### Buffer aliasing

The application assembles its reply in place at `iram51` onward, the same bytes
that hold the request (**E**: both use `iram51` as offset 0). Handlers copy the
fields they need before writing (echo, command, count, key), except
`0x28,0x10`, which reads four request bytes while overwriting them and so
returns `10 00 10 00`. The transmit buffer begins at `iram50` (the application
writes `MOV @R0,iram50` as a no-op placeholder); **I** that the loader keeps the
received start byte `0xa0` there and sends it as the reply's start byte. The
reply buffer ends at `iram` `0xff` at the latest.

### Receive rules

None of the receive rules are in the image: the serial receive and transmit
code is the loader below `0x4000`, which the image does not hold. What is
knowable:

| Question | Answer |
|---|---|
| Bare `0xa0` inside a frame abandons it | **U** (loader) |
| Bad escape (`0xaa` then not `b0 b5 ba`) | **U** |
| Frame ending inside an escape | **U** |
| Body over 256 bytes | the application cannot hold it: the reply buffer lies in the 256-byte internal RAM from `iram51`, and request fields are read at offsets up to 15 (**I**) |
| Frame shorter than 9 bytes | the application does not check length (**E**); the loader's minimum is **U** |
| The bay sends a start byte | **I** yes (`0xa0`, see buffer aliasing) |
| Receive timeout | **I**: a timer-0 routine at `0x4300` reloads the timer with `0xfc68` (920 machine cycles per tick), counts eight ticks, then sets flag `iram21.0` and clears `iram21.5`; nothing in the image reads these, so they are loader flags (about 7 ms at 1 MHz machine cycles, the clock being **U**) |
| What the L1 transmits | **E**: the transaction routines (`ffe3f404`, `ffe642d0`) hand the UART the frame body and checksum with no start byte, stop byte or stuffing; the port's interrupt handler in mode 2 sends `0xa0` first, stuffs the body and sends `0xa5` last (`ffe631c8-ffe631de`, `ffe63492-ffe63556`; see the section below). The checksum is not in the handler. |
| What the L1 accepts as a reply | **E** (`ffe3f378`, `ffe3ddac`): checksum rule above, control bit 7 set, tag equal, command equal, link completion code 0, and for `0x20` the inner checks (inner completion code 0, response bit set, network function equal to the request's); it does not check either address, but the UART handler has already dropped a frame whose destination is neither zero nor the L1's own address (E, `ffe63374-ffe633d4`). Up to 3 attempts (`ffe3ddac` counts in the context). |

Loader commands `0xc1` and `0xc2`: the application's dispatch answers them with
completion code `0xc1`, so for flashing to work the loader must intercept them
before the application (**I**). The L1 accepts a `0xc1` reply with any addresses,
tag equal, command `0xc1`, completion code 0 (**E**) and ignores the reply to
`0xc2`. Erase behaviour, the header-sum check and the boot after `0xc2` stay
**U**.

### Output latch

Every latch write puts a byte on `P1` and strobes `P3.5` low then high
(**E**). The codes, with every bank-select code having bit 7 set (**E**):

| Code | Used by |
|---|---|
| `84 80 90 8c a4` | input banks for sensors 1 to 6 (`80` and `a4` also in `0x18,0x03`) |
| `94 98` | input banks for sensors 7 to 14 (also in `0x18,0x03`) |
| `a8` | sensor 15 |
| `9c` | link command `0x0b` and the hook at `0x4703` |
| `ac` | start of a `0x28,0x11` read |
| `09` then `00`, `11` then `0a` | only the `0x18,0x03` action: each pair is written (about 1.45 million machine cycles apart) only when a flag from the interrupt-time code is set, `09`/`00` for flag `iram21.3`, `11`/`0a` for `iram21.4`; the flags come from `P1` snapshots after the `98`/`94` and `80`/`a4` selects |

The `0x18,0x03` reply carries only the prefix `<echo> 1c 03 00`. What the
individual `P1` bits mean on the bay's boards is **U**.

## The L1 UART handler: framing, address filter and direction control

Read in full: the handler `ffe6325c-ffe635f6` (entry stubs `ffe635f8` and `ffe6360c` for ports 1 and 2), the port command function `ffe6309c`, the port programming routine `ffe62f44`, and the open, read, write and registration routines `ffe648bc`, `ffe64bac`, `ffe642d0` and `ffe645a4` (L1 1.44.0 package 1). Grades are E unless marked.

**Which port.** The power-bay task opens port 1 (`ffe3e4e2`). Port 1 is registered by `ffe62de8`, called from `ffe54c3c` for brick letter `G` (class 2, 9600 baud) and from `ffe55184` for the default arm (class 4, 19200 baud, interrupt level 6, vector `0x42`). Its port 1 case takes a register block from the pointer at `0x300002e8` plus `0x140` (`ffe62e0c-ffe62e18`); its port 2 case would take `+0x180` (`ffe62e24-ffe62e30`), but no caller registers port 2. With the board contract's `MBAR` of `0x10000000` ([board-boot-contract.md](board-boot-contract.md)) and the public MCF5206e manual's UART offsets (I: manual, not an image), port 1 is the on-chip UART 1. The port tables in [quad-uart-and-serial-links.md](quad-uart-and-serial-links.md) leave UART 1 closed for brick letters `0 4 7 8 9 A E Q V`, so on an Origin 300 (`4`) the open at `ffe3e4e2` returns zero (`ffe648de`) and the task takes its failure branch (`ffe3e55e`, I that this ends in status 5). The registration stores the port's command function at descriptor `+0x24`; the `jsr (a0)` calls in the read and write routines are calls to that function with a command number: 4 arms a read (`ffe64c1a`), 13 begins a write (`ffe64324`), 6 enables the transmit interrupt (`ffe64354`), 2 resets the transmitter, 3 and 5 stop the receive and transmit interrupts.

**Settings that select the framed handler.** The settings block at `ffe3e520-ffe3e544` sets the long at offset `0x10` to 2. The handler tests that field (`ffe63296`, `ffe6348c`): value 2 takes the framed path below; any other value moves one raw byte per interrupt (`ffe633fc`, `ffe635a0`). The same value 2 makes the programming routine release the output-port bit at start (`ffe63044-ffe6304e`). Settings offset `+4` is the 16-bit own address (setting 1 in the high byte, setting 2 in the low byte, `ffe3e4b0-ffe3e4d6`); `+8` is 0, which programs 8 data bits, no parity (`ffe62f92`, mode byte `0x13`); `+0xc` is 0, no flow control (`ffe62f9c-ffe62fc4`).

**Receive, mode 2 (`ffe63296-ffe633ea`).** A state word at `+0xac` of the port record (0 idle, 7 and 8 reading the destination, 5 data, 3 after an escape, 6 complete, 9 rejected). Each received byte goes through these steps, in this order:

| Byte | Action |
|---|---|
| `0xa0` | if idle (state 0), enter state 7 (expect the destination); in any other state the byte is dropped and nothing restarts |
| `0xb0`, `0xb5`, `0xba` | only if the previous byte was `0xaa` (state 3): the byte becomes `0xa0`, `0xa5` or `0xaa` and the saved pre-escape state is restored; otherwise an ordinary data byte |
| `0xa5` | if no byte has been stored since the read began, back to idle with the frame discarded; otherwise state 6 and the read completes. The buffer holds the unstuffed body and checksum, without `0xa0` or `0xa5` |
| `0xaa` | save the state, enter state 3 (escape); the next byte is taken as above. After `0xaa`, a byte other than `0xb0`, `0xb5`, `0xba` or `0xa5` is dropped and the handler stays in the escape state (`ffe6335a-ffe6336c`); `0xa5` still ends the frame |
| other | state 0 (before a start byte): dropped. State 5: stored. States 7 and 8: the two destination bytes, low byte first |

After both destination bytes the handler compares the 16-bit value with zero and with the own address (`ffe63398-ffe633d4`): equal to either, it stores both bytes and enters state 5; otherwise state 9, in which bytes are dropped until `0xa5`, when the read goes back to idle and keeps waiting. A read also completes when its buffer is full (`ffe633ea`, `ffe63436`) or on a line error bit (`ffe63218` maps the four error bits to completion codes `-0x450` to `-0x453`). The handler never sums or compares the checksum; it passes the two checksum bytes up as data, and the checksum rule is in the transaction routines `ffe3f404` and `ffe3f378`. A receive interrupt is served only while a read is armed (`ffe63280`); otherwise the handler disables the receive interrupt and leaves the byte in the UART, and the next write that starts with a byte waiting is abandoned (below).

**Transmit, mode 2.** Begin-write (`ffe6318a-ffe631e2`): if the receiver holds a byte, the write is abandoned with `-0x454` and the receiver is reset; otherwise the transmitter is enabled, the output-port bit 0 is cleared through the reset-bit command (`+0x3c`, `ffe631c4`) and, if the transmit holding register is free, `0xa0` is written at once (state 0); else state 10 sends it from the interrupt (`ffe634bc`). Each transmit interrupt (`ffe63492-ffe6355c`) sends the next body byte; `0xa0`, `0xa5` and `0xaa` are sent as `0xaa` followed by `0xb0`, `0xb5` or `0xba` (`ffe63514-ffe63540`, states 1 to 3). When the buffer is empty it sends `0xa5` and resets and re-enables the receiver (`ffe63542-ffe63556`), then waits for the transmitter-empty status bit (`ffe63572`), disables the transmitter and sets the output-port bit again through the set-bit command (`+0x38`, `ffe63594-ffe6359a`) before finishing. When settings offset `+0xc` is 3, which only the `0x0b` bracket sets, the handler reads the status register 1000 times (`ffe63578-ffe63590`) after the transmitter-empty bit and before it disables the transmitter and sets the output-port bit: a hold of the driver after the last byte (E for the loop, I for the purpose). Offset `+0xc` is 0 for every other power-bay frame.

**Direction control (I).** The output-port bit is cleared for the whole transmission and set outside it; with a receiver that is reset when the frame ends, this is the shape of a half-duplex driver-enable line on a shared two-wire bus. Which pin and which transceiver is **U**: the image shows only the UART's output-port set and reset commands. That the bay's cable is a multi-drop bus is also **I** (the L1 filters on a 16-bit destination and every L1 has its own address), not shown by this image.

**"power bay ICMB card".** The string `power bay ICMB card serial number` occurs once, as the help text of a console command-table entry for `pbay serial` (entry at body offset `0x204c8`, help pointer at `0x204dc`, names `pbay` and `serial`; command number 0x10). The dispatcher `ffe27464` sends command 0x10 to `ffe27424`, which calls `ffe3cf84` with the power-bay context at `0x30004150`. That routine builds a read request with `ffe3ce74`, runs it through the same transaction routine `ffe3ddac`, and walks type and length prefixed fields (length in the low six bits, a `0xc1` end marker, the fifth field copied out, `ffe3d0bc-ffe3d11c`): the layout of an IPMI FRU product area (I: public IPMI FRU specification). The request is a tunnelled `0x20` command (`ffe3ce4a`) whose inner network function is `0x0a` (`ffe3ce5e`) and whose four data bytes are a device byte, a 16-bit offset and a count (`ffe3cea6-ffe3cec2`; I that this is the `0x28,0x11` read described below, and so a Read FRU Data in the public IPMI storage network function). So the "ICMB card" is a name for the device on the far end of the same link, whose inventory data the L1 reads; no other use of the string, no second device address and no other UART or bus access exists in this path (E: the whole path read). Whether that device is the bay's controller board or a daughter card on it is **U** (the image cannot tell; I that it is the controller, since the bay's own address `00 44` answers).

## L1 side: when and what it sends (E, order from `ffe3e470` and `ffe3e294`)

1. Task start: wait on a semaphore, set status 1 ("PBay busy initializing"), build the L1's source address from two settings (high byte setting 1, low byte setting 2), open UART, 19200 baud, wait `100 * setting + 300` ms (units of the L1 delay routine).
2. Init (`ffe3e294`): `0x10` to address 0; `0x02` to address 0 (arguments three and three); store the reply's 16-bit address (`0x4400` when zero, status 5); `0x12` with that address; `0x13` (one byte, device type); tunnelled `0x20` pair for the capability reply (bytes 1-3 saved); when byte 1 is nonzero the supply count routine runs; when byte 2 is nonzero the record enumeration (`ffe3e054`) runs: Get-repository-info-shaped `0x20`/`0x22`/`0x23` loop that reads records until the reply's next-id bytes equal `0xffff`; finally the supply and DC-port polling set-up. Status 2 ("ready") is set on success.
3. Polling task (`ffe3e5c8`): read DPS presence and fail bits and DC-port state (`ffe3d980`), log `Power Bay DPS %d inserted/removed`, `AC fail, lost redundancy`, `fail, lost redundancy`, `predictive fail`, `Power Bay DCport %d fail, lost power to brick`.
4. `pbay` command text (`ffe3d874` and `ffe3da10`): version from `0xc0` (`PBay FW Version %02d.%02d, Built ...`), per-supply rows, total current and FRU fields from the tunnelled reads.

## Flash sequence (E for the L1 half, I for the loader half)

L1 task `power_bay_flash_task` (`ffe3f480`, file `PBayFlash.c`) answers three requests; requests need the bay semaphore (`ffe4368c`) and refuse during other work:

| Request | L1 action |
|---|---|
| `0x0092` | status 3, send link command `0xc0`; reply bytes 0-1 version, 2-5 build time, byte 6 chunk size; no reply gives chunk size `0x80`; chunk size is capped at `0x8c`; status 2 |
| `0x0093` | two arguments required (else error `0xa1`; argument two must be a byte buffer, else `0xa2`): argument one the start offset, argument two the data and its length; loop of link command **`0xc1`** with payload `[count][offset low][offset high][data...]`, at most the chunk size per frame, a 4-tick pause between frames, offset advanced by the count; on success or at the end the L1 re-runs the init sequence; failure of any frame ends the loop |
| `0x0094` | status 4 ("PBay busy flashing"), send link command **`0xc2`** (no data), reply ignored, then the request returns |

Status codes the `pbay` command reports (`ffe96f9c`): 0 not initialized, 1 busy initializing, 2 ready, 3 busy try again later, 4 busy flashing, 5 communication failure.

**Unknown (U):** the loader's replies to `0xc1` and `0xc2`, its erase behaviour (no erase command is sent: the L1 sends only `0xc0`, `0xc1`, `0xc2`), its checksum check against the header sum, and whether `0xc2` boots the new image. The `flashpbay` host tool carries the same L1 packet code (see [../host-tool-functions.md](../host-tool-functions.md)); its file-to-request mapping was not read in this pass.

## What the published documents say about the link (hints, not findings)

- *Origin 3000 Series System Architecture* (108-0240-002), printed page 2-36: each of the power bay's eight output connectors carries 48 V dc, 12 V standby "and signals" to a brick over the power cable; the bay powers the L1 with the standby voltage and supplies 48 V only when the L1 asks. Printed page 2-49 lists the L2 controller's single RS-485 port, labelled "ICMB", as "Not used".
- *SGI Origin 3000 Series Internal Technical Configuration Guide* (108-0266-001), printed pages 19-20: the bay "provides power control and monitoring" and has eight 48 V outputs on 21-pin connectors.
- *System Cabling* preliminary draft (108-xxxx-PR2), printed page 1-16: the L2 controller's "ICMB" connector is a Molex RS-485 port to the D-bricks ("can link to many D bricks (32 devices)"). The draft's power-bay table (printed page 1-14) lists only the AC input and the 48 V outputs.
- None of them names an ICMB card in the bay, a bridge between an L1 and the bay, or the bay's signal pinout. The *System Controllers* manual (108-0241-001) was not available.

So the published word "ICMB" belongs to a multi-drop RS-485 bus of the same family as the byte stuffing seen here, and the bay's link rides on the power-cable connectors (I). Which wires, and whether several bricks share one bay line, are **U**.

## Not read

The remaining `P1` routines at `0x51da-0x541c` (eighteen strobes of `P1 = 0` to `0x11`, no caller found; I: called by address from the loader or unused), the record tables' contents beyond layout, the sensor-number to supply mapping, and the host tool's argument handling.
