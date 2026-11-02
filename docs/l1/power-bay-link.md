# Power-bay serial link: framing, commands and flash sequence

Reads the Origin 300 L1 1.44.0 body (`Fuel/PE/O300 1MB` package, listing in [disasm/](disasm/README.md), the code that the 6.5.30 `l1.bin` carries as package 1) against `pbay.bin`, the 5,888-byte power-bay controller image (sha256 `7428aaf3d5beffcb23fb56aa9f82c013a6f3ebbb5f41175a61781bfcf366f9c6`, identical in every IRIX set; labelled `pbay-00.18.bin` in the 1.62.1 RPM). The image was decoded with a table-driven MCS-51 decoder written for this pass (scratch tooling, not committed); the 8051 family is **inferred** (see [../controllers.md](../controllers.md)). Grades: E established (routine read in full), I inferred, U unknown. L1 addresses are first-bank (`0xffe00000` base).

## Image layout (E from the bytes, I for the load address)

- Bytes `0x00-0x1f` are a header, not code: bytes 0-1 `17 00` (the file length is `0x1700`, so a 16-bit length field), bytes 2-3 `d5 da` (E: equals the 16-bit sum of bytes `0x04` to the end, big-endian), then an ASCII author banner and, in the last three bytes, `56 00 12` (`V`, then the version `0x0012` little-endian). The version matches the `00.18` in the RPM label (I).
- **Load address `0x4000` (I).** 306 of 306 absolute call and jump targets that land in `0x4000-0x56ff` hit a non-`0xff` byte with that base, and the `0xc0` reply (below) reports the same version and a chunk size that fits whole-file flashing. Under this base the file is the flash content from controller code address `0x4000`; code below it (a boot loader holding the serial driver, a transmit entry at `0x1500` that the frame trailer jumps to) is **not in the image** (E that the app ends its frames with `LJMP 0x1500`).
- `0x4700` holds `LJMP 0x55a3` and `LJMP 0x557f` (E); they look like a relocated reset and INT0 pair. The app dispatcher is at `0x4706`; handlers and tables follow to `0x5690`. Timer-0-shaped routines at `0x4300` and `0x4500` have no caller in the image (I: called by the loader).
- Build time reported by command `0xc0`: epoch `0x39d4a060` = 2000-09-29 14:00:00 UTC (E for the bytes, I that the field is a Unix time).

## Transport (E, both ends read)

L1 side: transaction routine `ffe3ddac`, task init `ffe3e470`. The L1 opens UART device class 1 index 2 (`ffe648bc(1,2)`), sets **19200 baud** (`0x4b00` in the settings block, E), and runs a request/response transaction per command with a bounded retry (a count in the context) and a receive-attempt count. Command `0x0b` brackets the transaction with a UART mode change (`ffe643e8` value 3 before, 0 after; meaning of the mode **U**).

**Framing (ICMB-style, I that it follows the public ICMB byte stuffing).** Frame start byte and stop byte appear in the app's transmit routines at `0x54d2` and `0x5509` as the stuffed values: the app escapes `0xa0`, `0xa5` and `0xaa` as `0xaa 0xb0`, `0xaa 0xb5`, `0xaa 0xba` (E, three tests at `0x54d6-0x54ec`, repeated for the checksum bytes at `0x552f-0x556b`), and ends the frame with `0xa5` before jumping to the loader transmit entry (E). The start byte `0xa0` is written by the loader (U: not in this image).

**Frame body (E).** Little-endian 16-bit fields:

| Offset | Size | Meaning |
|---|---|---|
| 0 | 2 | destination address (L1 builds it at `ffe3f404`; app reads `00 44` for itself, i.e. `0x4400`) |
| 2 | 2 | source address |
| 4 | 1 | zero |
| 5 | 1 | control: bits 0-4 a sequence or tag, bit 7 set on a reply (L1 accessors `ffe3e9c4`, `ffe3ea00`; app `ORL #0x80` at `0x54be`) |
| 6 | 1 | command code (`iram57` in the app) |
| 7 | 1 | request: first argument byte; reply: completion code (app sends `iram41`) |
| 8.. | n | data |
| last 2 | 2 | checksum |

**Checksum (E, both ends).** Sum all bytes before the checksum, add the byte count of that sum, add one, and send `0xffff` minus that 16-bit total, low byte first (L1 `ffe3f314` and `ffe3f404`; app `0x54d2` counting `iram43`, `0x5509` adding the count and 1 and complementing). The L1 receiver (`ffe3f378`) accepts a frame when the checksum and the received complement sum to `0xffff`. A request whose destination bytes are not `00 44` is ignored unless the command is `0x02` or `0xc0` (E, `0x4706-0x4727`); a command not in the table, or `0xff` in the completion field, takes the error-reply path at `0x5451`.

## Link commands (the app's dispatch table at `0x4758`, handlers at `0x4763`)

| Code | Handler | What it does (E unless marked) | L1 use |
|---|---|---|---|
| `0x02` | `0x4777` | reply carries its own address `00 44` and echoes `iram53/54`; answers regardless of destination | init step 2: the reply's 16-bit value becomes the bay address; zero means no bay (state 5, "communication failure") and `0x4400` is then used |
| `0x0a` | `0x47be` | fixed three-byte reply `01 08 02` (I: device or protocol revision) | sent in `ffe3d2a8` |
| `0x0b` | `0x47db` | strobes `P1 = 0x9c` with `P3.5` low then high, sets flag `iram20.2`, then replies `iram34` mapped from a one-hot code to its bit number (8 bits), after a delay loop | bracketed with the UART mode change |
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

| Pair | Handler | Behaviour |
|---|---|---|
| `0x00,0x00` | `0x4941` | fixed eleven-byte reply `04 00 id 00 22 22 00 00` after the header (I: capabilities) |
| `0x10,0x23` | `0x4977` | no reply |
| `0x10,0x2d` | `0x497a` | **sensor reading**: `iram5b` selects 1-6 (select codes `0x84,0x80,0x90,0x8c,0xa4` strobed on `P1`/`P3.5` by `0x4a33`) or 7-14 (codes `0x94,0x98` by `0x4a82`), 15 triggers the `0x4afb` sequence; the reply carries two bit-mask bytes (`iram27`, `iram28`) |
| `0x18,0x03` | `0x4b1f` | action: sets `iram20.7` and strobes `P1 = 0x98`, `0x94`, `0x09`, `0x00`, `0x80`, `0xa4`, `0x11`, `0x0a` in a fixed sequence with delays (I: warm reset or DC-port control) |
| `0x18,0x04` | `0x4c76` | reply `0x56 0x00` (I: self test not implemented) |
| `0x28,0x10` | `0x4c9d` | reply echoes four request bytes (`iram5b-5e`) |
| `0x28,0x11` | `0x4cce` | maps `0xa0,0xa2,...,0xae` to 0-7, strobes `P1` with the request bytes (`iram5b`..`5e`) and sets flags (I: write to the power-bay FRU or a latch) |
| `0x28,0x20` | `0x4dcf` | fixed reply `01 0c` then fifteen zeros |
| `0x28,0x23` | `0x4e32` | **record read** keyed by `iram5d` (0, 1, 2, 3, 10, 11, 12): returns up to `iram60` bytes (default `0x20`, `0x10` or `0x30`) from the fixed tables at `0x4eb2`, `0x4f1f`, `0x4f8c`, `0x5065`, `0x50bf`, `0x511c`, `0x5179` (I: sensor data records and FRU data; the L1 prints the multirecord power-supply information and DC output fields from them) |

The `P1`/`P3.5` pairs write a command byte to a latch outside the MCU: `P1` carries the byte and `P3.5` pulses low then high (E for the pattern, 45 uses). What the latch is (U); the previous reading of the vector files points at CPLDs on the bay boards but names no register.

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

## Not read

The remaining `P1` routines at `0x51da-0x541c` (eighteen strobes of `P1 = 0` to `0x11`, no caller found; I: called by address from the loader or unused), the record tables' contents beyond layout, the sensor-number to supply mapping, and the host tool's argument handling.
