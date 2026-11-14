# L1 1.44.0 C-brick (type C): the "not running (configuration error)" state, PIMM presence, the display part and the USB reset sequence

Follow-up to [cbrick-mbus-registration.md](cbrick-mbus-registration.md). It answers four questions that note left open: what gates the `env` state "monitoring is enabled, but not running (configuration error)", what a fitted processor module (PIMM) is as far as the firmware can tell, what the front-panel display part is sent and expects, and the exact USB controller reset sequence.

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), ColdFire package 0 (`Base 1MB image`, body loaded at `0xffe00000`), first program copy. Addresses are package 0 addresses. Package 2 was not re-read for this note; its tables and bus-select code match package 0 (see the earlier note), its routines were not compared. Grades as in [../../../AGENTS.md](../../../AGENTS.md): **established** (the whole routine or table was read and the claim follows), **inferred** (reasoning stated), **unknown** (nothing in the image settles it).

Two-wire transfer primitive used below (`0xffe6cdf4`, mode in the first argument; wrappers `0xffe6cee8`, `0xffe6cf4c`, `0xffe6cfbc`, `0xffe6cfe4`, `0xffe6d020`, `0xffe6d04c`): mode 1 register write (address, register, data), mode 2 plain write (address, data bytes, no register byte), mode 3 register read (address, register, repeated start, address with read bit, data), mode 5 plain read (address with read bit, data). Established. Every transfer is preceded by the bus-select hook for its segment (`0xffe6cab4`), which writes no select byte for segment 0.

## 1. `env`: "enabled, but not running (configuration error)"

### What the text depends on (established)

The status printer is `0xffe2b4cc` (called from the console command dispatcher at `0xffe15578`; inferred to be the `env` status command from the command table text "get environmental monitoring status."). It reads three things: persistent setting 6 (monitoring on), persistent setting 7 (temperature monitoring explicitly off) and two RAM words of the environment record at `0x300045a8`: the run state at `+0` and the last error at `+0xc`.

| Condition | Text printed |
|---|---|
| setting 6 is zero | `ATTENTION: Environmental monitoring is disabled!` in a star banner; if setting 7 is non-zero a second banner `ALL TEMPERATURE MONITORING FAILSAFES DISABLED!!` / `(to re-enable, use "env on" or "env temp on")`. Run state and error word are not looked at. |
| setting 6 non-zero, state 1 | `Environmental monitoring is enabled and running.` |
| setting 6 non-zero, state 0, error 0 | `Environmental monitoring is enabled` and a bare newline |
| setting 6 non-zero, state 0, error 1 | `..., but not running (initialization error).` |
| setting 6 non-zero, state 0, error 2 | `..., but not running (configuration error).` |
| setting 6 non-zero, state 0, error 3 | `..., but not running (start up error).` |
| setting 6 non-zero, state 0, any other error | `..., but not running (unknown error - 0x%02x).` |
| setting 6 non-zero, any other state | `..., but in unknown state - state: 0x%02x error: 0x%02x.` |

Where the state and error come from (established): the environment restart routine `0xffe2b798` runs at boot at the end of the environment init (`0xffe2bd1e`) and again from the setting-change helpers that store settings 12, 6, 7 and 125 and then restart it (`0xffe2b810`, `0xffe2b830`, `0xffe2b888`, `0xffe2b8a8`), which the `env` subcommands presumably call (inferred). It returns without touching the record if the word at `0x300009c4` is zero (only ever read; its initial value is data, not decoded: unknown). Otherwise it stops the monitors and re-initialises them (`0xffe27e34`, which calls the monitor init `0xffe69ca0` again), sets state to 1 if setting 6 is non-zero and 0 otherwise, and calls the configure routine `0xffe291d8`. The configure routine clears the error word, then ORs together the results of three loops:

1. `0xffe282bc` for each of the 20 supply records (`0xffe457c4`): power-input monitoring.
2. `0xffe29190` (its callee `0xffe28ad4`) for each fan record (three: LEFT on monitor 0 input 6, CENTER on monitor 0 input 7, RIGHT on monitor 1 input 6).
3. `0xffe28ff8` for each of the four temperature records (`0xffe459d4`: monitor 3, 4, 5, 6, all input 4).

If the OR is non-zero it stores state 0 and **error 2** (`0xffe29264` to `0xffe29268`) and returns -1. That is the whole path to the configuration error. The start-up error store (error 3, `0xffe29288`) sits behind a second test of the same register that is always false on that path, so it cannot be reached; error 1 is stored nowhere in this image's environment code. Established for the control flow; whether the unreachable branch is a compiler artefact is unknown.

### What makes a loop return non-zero (established)

Every item does only register **writes** to its monitor (never a read), through the monitor driver's cached-write path. The first failed transfer (no acknowledge or any bus error) is returned, logged at level 1 as `Error configuring NAME power|fan|temperature monitoring: ERR`, and ends that item. The three routines skip an item, returning 0 and writing nothing, in these cases:

- supply (`0xffe282bc`): the record's monitor field is `0xff`; or the record is disabled (`0xffe4bb44`; a PIMM probe disables records); or persistent setting 125 is 1. Then it logs `config power N disabled!` only if the debug flag is set.
- fan: the fan record is disabled (`0xffe4ab8c`).
- temperature (`0xffe28ff8`): its monitor is disabled (`0xffe5e60c` reads the monitor's enabled byte).

For an item that runs, the writes to the monitor are, in order (monitor type 1, the only type on this board):

| Item | Writes (register := value) |
|---|---|
| any input | interrupt mask register for the input masked: registers `0x43` (inputs 0 to 4, 6, 7: bit = input number) or `0x44` (inputs 8, 9: bits 0, 1), written as the cached value with the input's bit set. The cache holds what monitor init wrote last (`0x43` = `0xdf`, `0x44` = `0x13`). |
| power input 0, 1, 2, 3, 8, 9 | high and low limit pair: input 0 `0x2b`/`0x2c`, 1 `0x2d`/`0x2e`, 2 `0x2f`/`0x30`, 3 `0x31`/`0x32`, 8 `0x33`/`0x34`, 9 `0x35`/`0x36` (`0xffe68238`). Limit values are computed from the supply record by floating-point code and not decoded. |
| fan input 6, 7 | limit register `0x3b` (input 6) or `0x3c` (input 7) (`0xffe68794`), value from the tachometer divisor held in cached `0x47`. |
| temperature input 4 | `0x39` := limit, then `0x3a` := hysteresis (`0xffe69488`). Values come from tables filled by the temperature set-up routines; not decoded. |
| any input, last | the same mask register again with the input's bit cleared (unmask), but only when the run state is 1 (setting 6 non-zero). |

Reading: any monitor on the C-brick that the configure loops touch and that does not acknowledge register writes produces error 2 whenever setting 6 is non-zero. The text is only printed from that error when setting 6 is non-zero; with monitoring off the same failures are logged but the banner above is printed instead. Inputs 4 to 7 of a monitor are not valid power inputs (`0xffe682e0` returns invalid-argument); input 5 is not a valid mask input at all (`0xffe6967c` table).

### Which devices the chain reaches (established)

| Device | Reached through | When it is skipped |
|---|---|---|
| NODE 0 (`0x58`, segment 2): fan 6, 7 (LEFT, CENTER) and the node supplies | fan and supply loops | never |
| NODE 1 (`0x5a`): fan 6 (RIGHT) and the node supplies | fan and supply loops | never |
| NODE 2 (`0x5c`, segment 2) | none: no supply, fan or temperature record names monitor 2 | always |
| PIMM0 A (`0x5c`, segment 4) | supplies `PIMM0 VCPU` (input 1), `PIMM0 SRAM` (0), `PIMM0 1.5V` (9, extra field `0x0d`), `3.3V AUX` (2), `5V AUX` (3), `12V BIAS` (8); temperature sensor 0 (input 4) | the six supply records are disabled when PSC code of PIMM 0 is `0xf`; the temperature record is **not** (section 2) |
| PIMM0 B (`0x5e`, segment 4) | temperature sensor 1 (input 4) only | never |
| PIMM1 A, PIMM1 B (segment 5) | same records for PIMM 1 (monitors 5, 6) | the supply records when PIMM 1 is absent; the temperature records whenever **either** PIMM is absent (monitors 5 and 6 are disabled, section 2) |

So with the three node monitors acknowledged, the configuration error is raised by exactly this: PIMM0 A or PIMM0 B (`0x5c` or `0x5e` on segment 4) does not acknowledge the register writes of the temperature records, or, with PIMM 0 fitted, of the supply records as well. This holds whether or not PIMM 0 is fitted: with the PSC at absent (`0xf`) the two PIMM0 temperature records are still configured, so each of PIMM0 A and PIMM0 B must acknowledge four register writes (`0x43` mask, `0x39` limit, `0x3a` hysteresis, `0x43` unmask).

PIMM0 A and PIMM0 B acknowledging is therefore sufficient for the state to become "enabled and running" as far as the monitors go (nothing else in the three loops touches the bus). The routine that runs after the configure step (`0xffe33454`, called with a status code) does not store to the state or the error word at the call sites read (its body was not followed: unknown).

### What the monitors must answer (established unless marked)

Monitor init (`0xffe686b4`, run at boot from the start-up block and again by every environment restart): one register write per step, address, register, value, each must be acknowledged; the first failure stops that monitor and logs `Error initializing the NAME monitor`: `0x40` := `0x80`, `0x43` := `0xdf`, `0x44` := `0x13`, `0x47` := table value (`0xa0` NODE 0, `0x20` NODE 1, `0x00` the rest), `0x40` := `0x03` (NODE 2 is left at `0x80`).

Polling (`0xffe69a80`, registered for monitors 0 to 6 at board init through `0xffe6c0d4` and called from an interrupt-driven rescan; the interrupt wiring is unknown): for each enabled monitor of type 1 (`0xffe689a4`): read register `0x41` (interrupt status 1) and read register `0x42` (interrupt status 2). Each read is address+write, register, repeated start, address+read, one byte. A failed read logs `Error reading monitor NAME interrupt status 1|2: ERR` at level 1, stops that monitor's scan and the loop carries on with the next monitor. A zero byte means no event. A non-zero bit in `0x41` (inputs 0 to 7) or `0x42` (inputs 8, 9) dispatches to the power (`0xffe292b0`), fan (`0xffe295a4`) or temperature (`0xffe29410`) interrupt handler, which then reads the value registers: voltages `0x20` to `0x25` (inputs 0 to 3, 8, 9), temperature `0x27` (input 4, one byte), fan counts `0x28`, `0x29` (inputs 6, 7). Answering `0x00` to both status registers is enough to stay quiet. The polls are made for a monitor whose module is absent as long as the monitor is enabled (section 2). Established for the register numbers; the numeric meaning of the voltage and temperature bytes is not needed to avoid errors.

SMBus alert response: the read of address `0x18` (7-bit `0x0c`) is made only for monitors of type 3 (`0xffe69258`, `0xffe69808`). No C-brick monitor has type 3, so the earlier note's statement that the firmware reads it "when handling a monitor alert" does not apply on this board. Established.

## 2. What a fitted PIMM is

### Presence (established)

The PIMM probe (`0xffe4482c`) reads PIMM PSC (expander index 10, wire `0x4a`/`0x4b`, segment 3). An error is logged and the byte taken as `0xff`. The low nibble is stored for PIMM 0, the high nibble for PIMM 1, in the bytes at `0x30000090` and `0x30000091` (`0xffe446cc`). Accepted codes: 0 to 8, 10 and 15. Codes 9 and 11 to 14 log `Unknown PSC for PIMM N: C` and the byte keeps its start-up value (`0` in RAM; nothing initialises it to `0xf`, so such a PIMM is treated as present with code 0, inferred from the absence of a store).

- **Absent** is exactly code `0xf` for that PIMM. Nothing else (no EEPROM read, no monitor read) decides presence.
- **Present with a processor module** (the module hook `0xffe44b48`, jump table at `0xffe44b66`): codes 1, 3, 4, 5, 6, 7 and 10 run the module power-step routine around reset hold and release; codes 0, 2, 8, 9 and 11 to 14 only write `0xc0` into the PIMM expander. Codes 3, 4, 5, 7 and 10 are the only ones for which "cpu disable" is allowed (`0xffe449e4`). Established.
- **Two PIMMs**: if both are present and different, each must be 1 or 6 (`0xffe44750`); any other pair logs `PIMM types are incompatible (A, B), disabling PIMM 1` and sets PIMM 1 to `0xf`. Code 8 selects clock-source and mode setting values of 0 instead of 1 (`0xffe447b8`).
- **Typical code**: the image does not say which code a retail C-brick PIMM reports. The pair rule makes 1 and 6 the only codes that may be mixed, and 1 is the lowest code that gets the full module treatment. Inferred that 1 is a valid choice; which code real hardware reports is unknown (route: a PROM or kernel reading that names the PSC encoding, or a published module guide).

### What the probe does for an absent PIMM, and why PIMM 0's monitors stay (established)

For each PIMM index `i` (0, 1) whose code is `0xf`, the loop at `0xffe448de` to `0xffe44996` disables: supply record `i + 1` (`PIMMi VCPU`) and records `5i + 10` to `5i + 14` (`SRAM`, `1.5V`, `3.3V AUX`, `5V AUX`, `12V BIAS`) through `0xffe4ba98`; processor entries `2i + 3` and `2i + 4` of the eight-entry table registered at `0xffe45a24` (`0xffe58110`); the expanders `PIMMi POWER` and `PIMMi VCC OK` (indexes `2i + 12` and `2i + 13`, `0xffe65a8c`); and the EEPROM record `i + 1` (`0xffe6aeb0`). Then it disables monitors with the **literal** numbers 5 and 6 (`moveq #5` at `0xffe4497e` and `moveq #6` at `0xffe4498a`, each followed by `jsr 0xffe67c70`), not numbers derived from `i`. So:

- an absent PIMM 0 disables PIMM1 A and PIMM1 B (monitors 5, 6) and leaves PIMM0 A and PIMM0 B (monitors 3, 4) enabled;
- an absent PIMM 1 disables monitors 5 and 6 as intended;
- no code path disables monitor 3 or 4. The monitor driver only changes the enabled byte through `0xffe67c70` (disable) and `0xffe67cc4` (enable); the other callers are the init routine that enables all seven at registration (`0xffe67bb8`) and the monitor-init type test (`0xffe69cd8`, which disables a monitor whose type byte is `0x63`; no table row has that value).

Consequence: monitors 3 and 4 are initialised and polled (section 1) on every C-brick, fitted or not. If PIMM 0 is absent and PIMM 1 present, PIMM 1's monitors are disabled as well. Reading: the literal constants look like a defect rather than intent; the intent cannot be read from the image (inferred).

### The PIMM EEPROM (wire `0xa8`, segment 4 for PIMM 0, segment 5 for PIMM 1) (established for what the firmware does with it)

The record is loaded with every other record by the start-up block (`0xffe4a83c`, loader `0xffe6bdfc`, per-record `0xffe6bd54`) unless its presence flag was cleared. PIMM records are never probed, so the flag is clear only when the PSC says absent. The loader reads 8 bytes at offset 0 and requires them to sum to zero (`0xffe5bdd0`); on failure it logs `PIMM N EEPROM header checksum error` (and for a bus error `PIMM N EEPROM read error, ERR`) at level 1 and stores a status in the record. It is not fatal. The layout it expects is the common header and area scheme of the public IPMI Platform Management FRU Information Storage Definition: byte 0 format version, bytes 1 to 4 the offsets (in 8-byte units) of the internal-use, chassis, board and product areas, byte 7 the header checksum; each declared area has its own zero-sum check. Established for the offsets and checksum reads (`0xffe6bba8`); inferred that this is the IPMI FRU format from the field layout and the 6-bit string length mask used by the string decoder (`0xffe6b458`, length = byte and `0x3f`).

The firmware compares **no name** from a PIMM record. The only product-name comparisons are the identity ones on routes `0xff` and `0xfe` (and the type-`%` reads of `0xfd`, `0xfc`), which are segment 0, not segments 4 and 5. Other readers of the PIMM records copy the parsed fields into replies to the system controller and display the record; none of the routines that read the record index 1 or 2 branches on its text (read: the loaders, the record dump loop `0xffe39674`, the request serialiser `0xffe16b3c`; not every caller of the record accessors was followed, so established only for those). An EEPROM that never acknowledges is safe: errors are logged and the boot goes on.

## 3. The display part at wire `0xf0` / `0xf1`

### Where it sits (established)

DISPLAY CONTROL (expander index 16, wire `0x40`, segment 0) is probed first (`0xffe51268`): one read, an error logs `Error reading the display I/O expander, ERR` at level 1, disables the expander (`0xffe65a8c`) and leaves the stored display index at `0xff`. On success the display record at `0x30001b84` is filled: byte 0 = the segment of the expander's descriptor (0), byte 1 = the part's wire address `0xf0` for descriptor form 0 (a form-2 descriptor gets `0x76` and another driver), width byte `+2` = 12 (`0xffe6567c`, a width above 16 logs `Display too wide`). So the part is on **segment 0**, 7-bit address `0x78`, written at `0xf0` and read at `0xf1`. Every access takes the segment-0 path of the bus-select hook, so no select byte is written; the part must answer with whatever select byte was left by the previous access to segments 2 to 5 (inferred: it sits on the board's main bus and not behind a switch enable).

### Reset (established)

Display initialisation (`0xffe52244`) is a no-op unless the probe succeeded. Then it calls the reset routine `0xffe51974` with a delay unit of 1:

1. Save bits 4 to 6 of DISPLAY CONTROL's shadow and write them clear (all three LED outputs; a clear bit is lit, inferred).
2. Reset hook (`0xffe656d0`) with delay 100 ticks: clear bit 7 of DISPLAY CONTROL (expander write), wait, set bit 7 (expander write), wait. Bit 7 is the part's reset line, active low (inferred from the order: low first, then high, the same pattern as the BEDROCK reset bits).
3. In the hook, after the reset pulse: take the bus for segment 0 and do a plain **one-byte read** at `0xf1` into record byte `+3`. If the read fails, byte `+3` becomes 2 and a capability flag byte `+4` becomes 1, and the hook still returns success. If it succeeds, a returned value of `0x10` or more installs the one-character write hook `0xffe65854`; a value below `0x20` sets bit (value and 7) of the capability byte. No particular value is required; any acknowledged read passes.
4. Restore the LED bits, then write the greeting lines through the line writer (below). A reset hook failure is logged `Display reset error: ERR` at level 3 and **nothing is disabled**: the display stays enabled and probed. The only errors the hook can return are a failed expander write to DISPLAY CONTROL (either of the two) or a failed bus lock.

### Writes (established)

The line writer `0xffe5152c` pads the text with spaces to the width (a text longer than the width is passed whole, up to 72 characters), maps unprintable characters to `?` (`0x3f`) and calls the write hook `0xffe657bc` (line `'1'` or `'2'`, length). The hook builds a buffer at `0x30003248` whose first byte is a command and sends it as one **plain write** (mode 2, no register byte) to `0xf0`:

- line 1: command `'1'` (`0x31`), then 12 text bytes, 13 bytes in all;
- line 2 up to the width: command `'2'` (`0x32`), then 12 bytes, 13 in all;
- line 2 longer than the width: command `'6'` (`0x36`), then up to 72 bytes, 73 in all.

The one-character hook (only installed when the reset read returned `0x10` or more) sends 3 bytes: command `'3'` (line 1) or `'4'` (line 2), a position byte, a character. The part must acknowledge every byte; there is no readback after a write. The command letters and the 12-character width are established from the code; what the part does with them is not (a display controller with ASCII line commands at a 7-bit address of `0x78`, which is in the range the two-wire standard reserves for 10-bit addressing: unknown part).

A write error is logged `Display write error: ERR` at level 3 by the callers (`0xffe51e4c`, `0xffe51ed6`, `0xffe51f78`, `0xffe51ffe` in the button-event hook and `0xffe5234e` after the greeting) and returned; **nothing is disabled**. The panel state hook (`0xffe51b3c`) and the refresh task keep running. Established for these callers; the display refresh task (`0xffe52098`) writes the same hook each tick and ignores its result.

Net effect of the three display error paths: only the expander read at probe time disables anything (the expander, and with it the display driver, which then returns -801 silently to every later call). A missing `0xf0` part costs level-3 log lines and nothing else.

## 4. USB controller reset sequence

### Probe, then reset (established)

After the revision probe (`0xffe70ab8`: read `0x18`; `0x10`, `0x11`, `0x13` quiet, other `0x1n` logged, any other value fails; then every even value from `0x00` to `0xfc` is written to `0x1b` and read back), the controller is registered (`0xffe70be8`) and reset through the generic USB layer (`0xffe6e818`). The reset hook is `0xffe70c30` with a flag argument. The flag is 1 for the two callers in this image: registration (`0xffe6eb9a`, reason 0) and the network connection routine (`0xffe54370`, reason 0), which resets controller 0 after switching COM CONTROL to USB mode. The flag is 0 only for the reason code `0xfffffa08` (not produced on this path).

**The reset sequence polls nothing.** There is not a single read of any controller register between the first and last write. Offset `+0x1c` is written twice (`0x00`, later `0x01`) and never read in the reset. It is read, as a single bit, only by the periodic activity check (hook `0xffe71500`): while the device has no address, a tick that finds bit 0 set counts, and on the fourth tick writes `0x00`; a tick that finds it clear writes `0x01`. So bit 0 of `+0x1c` must read back what was last written (a latched bit, reset value 0), nothing more is needed. Offsets are from the base of chip select 3, `0x40000000`.

Sequence with the flag set (every step a byte write to base + offset unless noted):

| # | Offset | Value | Note |
|---:|---|---|---|
| 1 | `0x11` | `0x00` | |
| 2 | `0x11` | `0x08` | flag only |
| 3 | wait 2 task ticks | | flag only |
| 4 | `0x11` | `0x00` | flag only |
| 5 | `0x1c` | `0x00` | flag only |
| 6 | `0x1a` | `0x01` | |
| 7 | `0x14` | `0x00` | |
| 8 | `0x15` | `0x00` | |
| 9 | `0x12` | `0x00` | |
| 10 | `0x0f` | `0x08` | |
| 11 | `0x0f` | `0x00` | |
| 12 | `0x10` | `0x00` | |
| 13 to 44 | for endpoint e = 0 to 7: `0x0a` := e; `0x0b` := `0x3f` for e = 0, `0x00` otherwise; `0x03` := `0xa4`; `0x08` := `0xa4` | | eight times, in this order |
| 45 | `0x1a` | `0x00` | |
| 46 | `0x16` | `0x03` | |
| 47 | `0x17` | `0x00` | |
| 48 | `0x11` | `0x12` | |
| 49 | wait 2 task ticks | | flag only |
| 50 | `0x1c` | `0x01` | flag only |

With the flag clear, steps 2 to 5, 49 and 50 are absent. The earlier USB note listed the sequence without values and without the last `0x1c` write.

Register names from the public Agere USS-820D USB Device Controller data sheet, revision 4, June 2001: `0x11` SCR, `0x12` SSR, `0x0f` SOFH, `0x10` FADDR, `0x14`/`0x15` SBI/SBI1, `0x16`/`0x17` SBIE/SBIE1, `0x0a` EPINDEX, `0x0b` EPCON, `0x03` TXCON, `0x08` RXCON, `0x1a` PEND, `0x1c` MCSR (bit 0 DPEN, the D+ pull-up pin enable). Inferred that the part is that family from the offsets, the revision values and the firmware's own wording. The meaning of individual bits in `0x11` (`0x08` as the software-reset step, `0x12` as the final value with the global interrupt enable, bit 1, set), `0x0f` and `0x03`/`0x08` (`0xa4`) was not read from the data sheet: unknown.

Later writes to the same offsets (address assignment at `0x10`, endpoint configuration, interrupt service) are in [usb-controller-registers.md](usb-controller-registers.md).

## Unknown

| Question | Route |
|---|---|
| Which PSC code a retail C-brick PIMM reports | a PROM or kernel table naming the PSC encoding, or a module guide |
| Whether the reads of `0x300009c4` ever see non-zero (the environment restart skips the configure step when it is zero) | read the data initialiser that fills the data segment |
| Whether error 1 or 3 can be reached from another environment path | the console `env` and the start-up callers were read; no store found |
| The display part and its command semantics | a public data sheet for a two-wire display module with ASCII line commands at address `0x78` |
| Bit meanings of the USB controller's `0x11`, `0x0f`, `0x03`, `0x08` and `0x12` writes | read the register chapter of the public data sheet |
| Package 2 equivalents of the routines above | compare the same routines in the `MIPS 2MB image` |
