# L1 to Bedrock hand-offs: the ordered events the firmware drives

This note lists, in the order the firmware performs them, everything the Origin 300 and C-brick L1 does that the compute node's Bedrock chip (or the node's processors) can observe: the reset sequences with their delays, the soft-reset and NMI bits, the front-panel triggers, and the power-state notice sent to the peer L1. It reuses [power-reset-cbrick.md](l1/power-reset-cbrick.md), [power-reset-ibrick-rbrick.md](l1/power-reset-ibrick-rbrick.md), [ip45-io-expanders.md](l1/ip45-io-expanders.md) and [board-boot-contract.md](l1/board-boot-contract.md) and does not repeat their tables. It corrects and extends them where a re-read found more (section 9 lists the changes).

Grades follow [../AGENTS.md](../AGENTS.md): **E** established (routine read in full), **I** inferred, **U** unknown. Each unknown names the software test that would settle it (section 8).

## 0. Inputs and conventions

- Image: IRIX 6.5.30 `l1.bin`, L1 1.44.0, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`. **Package 0** (`Base 1MB image`, body sha256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`) is the C-brick reference (board source literal `IP35.c`, brick type `C`). **Package 1** (`Fuel/PE/O300 1MB image`, body sha256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`, board literal `IP45.c`, brick type `4`) is the Origin 300. Addresses are first-bank (`0xffe00000` base). Container offset = `0x68` + (address - `0xffe00000`) for package 0, and `0xefdd0 + 0x2c` + (address - `0xffe00000`) for package 1. The package 1 listing is committed ([l1/disasm/mcf5206e-1.44.0-fuel-pe-o300/l1.lst](l1/disasm/mcf5206e-1.44.0-fuel-pe-o300/l1.lst)); the package 0 listing is regenerated with a 68040-mode decoder (capstone) and is not committed.
- **Tick.** Every delay below is a count of firmware ticks passed to the delay routine (`0xffe40e54` in both packages), which puts the calling task to sleep for that many kernel clock units. The kernel clock counter at `0x30003c10` advances by one in the timer-1 interrupt routine (`0xffe4032e`, vector 29 at `0xffe10ff4`) and is cleared at start-up (`0xffe43c38`). E. The delay argument is read in that unit: I (the sleep-queue arming routine `0xffe40090` was not read). The wall-clock length of a tick is a function of the unmeasured processor clock: [board-boot-contract.md](l1/board-boot-contract.md) infers 48 MHz and a 100 Hz tick from the timer-1 reference `0x752f` with prescale 16. So 1 tick = 10 ms is **I**, conditional on the clock (U, section 8).
- **Expander writes.** "Set(idx, m)" and "clear(idx, m)" are the masked expander writes of [ip45-io-expanders.md](l1/ip45-io-expanders.md): the routine replaces the masked bits of the expander's shadow byte and sends the whole shadow byte to the expander's wire address once, even when the byte does not change. In package 0 the set routine is `0xffe65ef8`, the clear routine `0xffe65f1c`, the general one `0xffe65e84(idx, mask, value)`; in package 1 they are `0xffe65bb0` (set), `0xffe65bd4` (clear) and `0xffe65b3c` (general). E.
- **Reason codes** passed to the wrappers (name routine `0xffe4eca0` in package 1, `0xffe4f050` in package 0): 1 console command, 2 front panel, 3 request from an L2 or host, 5 internal (power-up and power-down). The log line `reset (%s)`, `NMI (%s)` and the like is written for every reason except 5 (reset and soft reset) and always for NMI. E for the numbers seen at call sites, I for the name strings' order (the name routine's table was read only as a six-way switch at `0xffe4f050`).

## 1. The lines, and what each one is called

Both boards describe the Bedrock-facing controls as two byte-wide expanders on the board's two-wire bus, segment 3, at wire addresses `0x42` and `0x44`, with the same applied start-up values on both boards. E (descriptor tables: package 0 `0xffe455dc`, entries 6 and 7; package 1 `0xffe45c90`, entries 2 and 3).

| Expander | Wire address | Name, C-brick | Name, Origin 300 | Applied at start | Input mask |
|---|---|---|---|---|---|
| index 6 (C), 2 (O300) | `0x42` | `BEDROCK CNTL 1` | `ASIC CTRL 1` | `0x72` | none (C), `0x20` (O300) |
| index 7 (C), 3 (O300) | `0x44` | `BEDROCK CNTL 2` | `ASIC CTRL 2` | `0x8a` | none (C), `0x02` (O300) |

The bit uses are identical in both images' hooks, so the two boards drive one control block (I: the labels differ, the addresses, applied bytes, bit masks and order of use agree; no image says the O300 `ASIC` is a Bedrock). The manual calls the NMI line "the NMI signal in I/O expander B_0100001x" (SGI Origin 300 Server Owner's Guide 007-4498-002, printed page 194, `nmi`): `0100001x` is wire address `0x42`, which agrees with the NMI bit sitting in this expander (E for the address match in the image; the manual is the source of the signal's name).

| Bit | Mask | Function | Grade |
|---|---|---|---|
| `CNTL 1` bit 7 | `0x80` | soft-reset line. Idle 0, pulsed 1 by `softreset` | E for the writes, I for "pulsed high means asserted" (only that it is set then cleared) |
| `CNTL 1` bit 6 | `0x40` | NMI line. Idle 1, driven 0 then 1 by `nmi` | E for the writes, I for polarity (low is the pulse) |
| `CNTL 1` bit 0, bit 1; `CNTL 2` bit 0 | `0x01`, `0x02`; `0x01` | the reset group: cleared by every hold, set by every release in a fixed order (section 4) | E |
| `CNTL 2` bits 3 to 7 | `0xf8` | strap or configuration bits written once per release | E for writes, U for meaning |

Which Bedrock input each bit reaches, and which Bedrock state survives each line, is not in any image: U (section 8). The only independent statement is the manual's one for `reset` ("sets various control and status signals back to their default values and reboots the operating system", printed page 201) and `nmi` (printed page 194), and the IP35 PROM's own strings, which show the software split: the PROM has a separate Hub-register NMI and soft-reset path (`PI_NMI_A`, `Sending soft reset to all CPUs on NASID`, `Soft reset received`; `ip35prom.img` in the 6.5.30 firmware set, sha256 prefix `d74b4b7e`, string offsets `0x7a910` (`Sending soft reset to all CPUs`), `0x8a875` (`Soft reset received`) and `0x730f8` (`PROM NMI HANDLER`)). Public Linux IP27 code does the same through hub registers (`arch/mips/sgi-ip27/ip27-nmi.c`, `SEND_NMI` writes `PI_NMI_A` plus the slice offset; `ip27-reset.c` writes `NI_PORT_RESET`). Both are **hints for the Origin 2000 Hub, not for Bedrock**, and say nothing about what the L1 expander bits reach.

## 2. Wrappers around every hook (the part earlier notes left open)

Each hook is called through a wrapper that logs, then does bookkeeping, then calls the board hook from the descriptor (`0x30001e30` in package 0, `0x30001f20` in package 1; hook offsets `+0x98` hold, `+0x9c` release, `+0xa0` "second reset?", `+0xa4` soft reset, `+0xa8` NMI). E for every call below; effects marked I where the callee was not read through.

**Hold wrapper** (package 0 `0xffe4f29c`, package 1 `0xffe4eeec`), no delay of its own, in order:
1. `0xffe36a04`: for ports 2 to 9, call `0xffe54a84(port, 0)`, then `0xffe3681c(0)`. These are the node-link and console UART service calls of [quad-uart-and-serial-links.md](l1/quad-uart-and-serial-links.md); not read (U for the bytes they put on a link, if any).
2. `0xffe5039c(0xff, 4)` (package 1 `0xffe4ffec`): for every entry of the port table at `0x30000134` (24-byte records) in state 2 or higher and with the right mode bits, call the port-down routine `0xffe50288`. Its log text is `port_down(%s) %s (%s)` with the action `clr LPOK` when the record's flag `0x10` is clear and `force link` otherwise (strings at `0xffe51234` to `0xffe5125e`). The port-down routine drives a port-status expander bit (`0xffe50084`: clear when record flag bit 0 is set, set otherwise, expander index in record byte 2, mask in byte 3) and a hook from the descriptor (`+0xc4`). So **before any hold, the L1 drops its local-port-OK signal on every NUMAlink or XIO-facing port** (the manual's `port` command names the status bits: remote power OK bit 0, local power OK bit 1, link LED bit 2, software LED bit 3, printed page 199). E for the call, text and structure; I that "clr LPOK" is the action taken in the all-ports case; U for the per-board port records (which expander bits).
3. `0xffe4c818` (package 1 `0xffe4c474`): clears the word `0x3000010c`. E.
4. Hook `+0x98`. A nonzero hook result is returned unless step 2's code is already nonzero.

**Hold wrapper for the `reset hold` command and for power-down and power-up** (`0xffe4f2f0` in package 0, `0xffe4ef40` in package 1) additionally runs `0xffe7219c` first, after logging `reset hold (%s)` for reasons other than 5: if the word at `0x30000044` is nonzero it writes six values to the parallel-port data register (MBAR + `0x1c9`): `PPDAT |= 0x08`, then `|= 0x18` accumulated, then `0x38`, `0x28`, `0x38`, `0x30`. E for the sequence. The word at `0x30000044` is zero in the initialised data and has no writer by absolute address (the data copy source differs per package; package 1's initial value is zero, E; package 0's was not located): whether this pulse ever runs is U. What the three parallel-port pins (bits 3, 4, 5) connect to is U.

**Release wrapper** (package 0 `0xffe4f144`, package 1 `0xffe4ed94`):
1. Log `reset release (%s)` unless reason 5.
2. If MBAR + `0x41` bit 7 (the software-watchdog enable, armed by `0xffe562d4` at start-up) is set, call the parallel-port scan routine `0xffe72214` (package 1 `0xffe720c4`); if it reports failure return its error without touching the hook. Its early exit is `-2300` when the word `0x30000044` is zero, which is why that word's value matters (U). E for the control flow.
3. **10 ticks.** E (`moveq #$a` at `0xffe4f1a8`).
4. For brick types `C 3 5 4 A 6 9 7 8 0 Q @ O B F H L` (compare chain `0xffe4f1b4` to `0xffe4f224`): `0xffe34510`, then `0xffe3517c` (clears the registered event subchannel word, see the next point). E. Other types skip both.
5. `0xffe36a04` (the port-service call above), then `0xffe52a30(0xff, 0)` (package 1 `0xffe52680`): for each of the first N ports (N = word at descriptor `+0x74`), `0xffe54a84(10 + i, 0)`. U for effect.
6. Hook `+0x9c`.
7. `0xffe4c828` (clears the power block's "just powered" word at `0x30000114` and, if the low-power monitoring count at `0x300000fc+0x10` is nonzero, logs an internal error and turns that monitoring off) then `0xffe50b74(0xff, 5)` (package 1 `0xffe507c4`): for every port in state 2 or higher whose flags allow, call `0xffe500fc` and `0xffe509a4`. These are the counterpart of step 2 of the hold wrapper and raise the ports again; the text is not read, so "restores local-port-OK" is I. E for the call structure.

**Soft-reset wrapper** (package 0 `0xffe4f0ec`, package 1 `0xffe4ed3c`): log `soft reset (%s)` unless reason 5, then hook `+0xa4`. Nothing else. E.
**NMI wrapper** (package 0 `0xffe4f0a4`, package 1 `0xffe4ecf4`): log `NMI (%s)`, then hook `+0xa8`. No port or UART call. E.

## 3. C-brick (package 0) hooks, with byte-level effect

Start state (E for the applied bytes, U for whether this application runs on an L1-only restart, see [ip45-io-expanders.md](l1/ip45-io-expanders.md)): `CNTL 1` = `0x72`, `CNTL 2` = `0x8a`.

**Hold hook `0xffe44bd4`:** (1) a mismatch between the two processor-module type bytes (`0x30000090`, `0x30000091`) and persistent settings `0x56` and `0x57` logs a notice and re-runs the default voltage margins (`0xffe4e40c`); (2) the board flag byte at `0x30000098` (set once by `IP35_Init`, `0xffe44ae0`, which logs "called twice" on a repeat) must be nonzero, else the hook returns `-900` before touching any line; (3) clear `CNTL 1` `0x80`, `CNTL 1` `0x02`, `CNTL 2` `0x01`, `CNTL 1` `0x01`, in that order, no delay; (4) per processor module (type byte, expander 13 and module 1, then expander 15 and module 2), `0xffe44b48`: type `0x0f` does nothing; types 1, 3, 4, 5, 6, 7 and 10 run the margin routine `0xffe4e4f8` and wait **10 ticks**, then every type except `0x0f` sets bits `0xc0` of the module's `VCC OK` expander (jump table at `0xffe44b66`, read from the bytes: offsets 72 for 0, 2, 8, 9, 11 to 14, 36 for 1, 3 to 7, 10, 32 for 15). E.
Resulting expander bytes from the start state: `CNTL 1` `0x72`, `0x70`; `CNTL 2` `0x8a`; `CNTL 1` `0x70`. The set of processor-module types {1, 3, 4, 5, 6, 7, 10} is the set that also gets the extra reset in section 5 and the reset-time margin step: the log text "reset again MIPS" ties it to the MIPS module types (I for the name, E for the set).

**Release hook `0xffe44dfc`**, in order (E; strap rules as in [power-reset-cbrick.md](l1/power-reset-cbrick.md) section 6):
1. `CNTL 2` mask `0xf8` := `D`, where `D` = `0x08`, plus `0x80` when the strap-7 rule holds, plus `0x20` when persistent setting `0x28` is nonzero, plus `0x10` when setting `0x29` is nonzero. Bit 6 is written 0. Result: `0x02 | D` (bits 0 to 2 of `0x8a`'s `0x02` survive).
2. **10 ticks**; clear `CNTL 2` `0x08`.
3. **10 ticks**; set `CNTL 1` `0x01`.
4. **10 ticks**; per module (expander 13 / module 1, then 15 / module 2) `0xffe44d54`: type `0x0f` nothing; otherwise clear `0xc0` of its `VCC OK` expander, **3 ticks**, and for types {1, 3, 4, 5, 6, 7, 10} run `0xffe4e450` (restore the margin; also drops the low-power-monitoring count) and **10 ticks** (jump table at `0xffe44db8`, bytes 54 and 22).
5. Set `CNTL 2` `0x01`, then set `CNTL 1` `0x02`, no delay.
From a hold state of (`CNTL 1`, `CNTL 2`) = (`0x70`, `0x8a`) and `D` = `0x08`: `CNTL 2` `0x0a`, `0x02`; `CNTL 1` `0x71`; `CNTL 2` `0x03`; `CNTL 1` `0x73`. The earlier note listed the module step before the set of `CNTL 1` bit 0; the order above is the code's (`0xffe44ed4` set, `0xffe44ef0` wait, then `0xffe44efc` module step).

**Soft-reset hook `0xffe445e8`:** flag-byte guard (`-900`); set `CNTL 1` `0x80`; **10 ticks**; clear `CNTL 1` `0x80`. Bytes from `0x73`: `0xf3`, then `0x73`. E.
**NMI hook `0xffe4463c`:** guard; clear `CNTL 1` `0x40`; set `CNTL 1` `0x40`, two consecutive writes with no delay. Bytes from `0x73`: `0x33`, then `0x73`. The pulse width is therefore the gap between two two-wire transactions: U in time (section 8). The hook ignores its argument, so the "node" and "slice" the `nmi` command takes do not select a line (E: the hook reads no stack argument); the command only records a target mask in `0x30000124` (node) and `0x30000128` (processor), `0xff` meaning all (`0xffe4efd4`, E), which the command later prints as `NMI %d.%d %d%c`. Which processors the node's firmware sees as interrupted is therefore decided downstream of the L1 (I).

## 4. Origin 300 (package 1) hooks

Descriptor initialiser `0xffe45bac` stores hold `0xffe453e4`, release `0xffe4552c`, second-reset test `0xffe45060`, soft reset `0xffe45084`, NMI `0xffe450e0`. All hooks except the soft-reset and NMI ones first require the board byte `0x3000009c` (set by `IP45_Init`, `0xffe44ff4`, called with the constant `0x0a` at `0xffe45ab2`) to be other than `0xff`; the soft-reset and NMI hooks return `-2500` (`0xfffff63c`) if it is `0xff`. So that byte is the expander index 10, `XBRIDGE CONTROL` (E for the constant, I for its use as a bus-select-style guard; the board note of [ip45-io-expanders.md](l1/ip45-io-expanders.md) already tabulates index 10). The expander is named index `X` below.

**Hold `0xffe453e4`:** write `X` mask `0x0b` := 0 (selecting segment 5 of the bus switch first, which is a side effect of the write); clear `ASIC CTRL 1` `0x80`; clear `ASIC CTRL 1` `0x02`; clear `ASIC CTRL 2` `0x01`; clear `ASIC CTRL 1` `0x01`; `0xffe45348(4, 8)`: if expander 4 (`POWER CONTROL`) is present: for processor-module PSC codes 1 to 4 run `0xffe4e148(8, 14)` and **10 ticks**; then set expander 4 `0x10` and then `0x20`. No other delays. E. Order of the four `ASIC` clears equals the C-brick order (`CNTL 1` `0x80`, `0x02`, `CNTL 2` `0x01`, `CNTL 1` `0x01`).
**Release `0xffe4552c`:** (a) if `0xffe4c2fc()` is nonzero (a predicate over the supply records, not read in full: U for its meaning) the strap-7 and `X` bit-2 rule is evaluated: `d7` := `0x80` and `d6` := 0 when the global word `0x3000015c` is set and setting `0x2a` is nonzero, or when it is clear and module record 1 is valid with its `+0x30` word nonzero; else (valid record with a zero word) `d7` := 0 and `d6` := 4, and with no valid record both settings `0x2a` and `0x2b` are first cleared and `d7` := 0, `d6` := 4; then **10 ticks**, `X` mask `0x0b` := `0x0a`, **10 ticks**, `X` mask `0x0b` := `0x0b`. (b) `d7 |= 0x08`, `|= 0x20` when setting `0x28`, `|= 0x10` when setting `0x29`. (c) `ASIC CTRL 2` mask `0xf8` := `d7`; `X` mask `0x24` := `d6`; **10 ticks**; clear `ASIC CTRL 2` `0x08`; **10 ticks**; `0xffe45488(4, 8)` (clear expander 4 `0x10`, then `0x20`, **3 ticks**; for PSC codes 1 to 4 run `0xffe4e0a0(8)` and **10 ticks**); set `ASIC CTRL 1` `0x01`; **10 ticks**; set `ASIC CTRL 2` `0x01`; set `ASIC CTRL 1` `0x02`. E for the order, delays and masks.
**Soft reset `0xffe45084`:** set `ASIC CTRL 1` `0x80`; **10 ticks**; clear. E. **NMI `0xffe450e0`:** clear `ASIC CTRL 1` `0x40`; set `0x40`; no delay. E.
**Second-reset test `0xffe45060`:** returns 1 when the PSC code byte `0x3000009d` is 2, 3 or 4, else 0. E. Section 5 shows when it matters.

Differences against the C-brick: the C-brick sets `CNTL 1` bit 0 before the module step, the O300 after it; the O300 has an `X` bus mask pulse before the strap, the C-brick does not; the guard and error codes differ (`-900` and `-2500`). E.

## 5. The `reset` command and the double reset during power-up

Reset wrapper (package 0 `0xffe4f324`, package 1 `0xffe4ef74`), argument = reason, in order (E):
1. Log `reset (%s)` unless reason 5.
2. `n := 2` if `0xffe4c7e4` (package 1 `0xffe4c434`) returns nonzero, else 1. That routine returns the `+0xa0` hook's result only when the "just powered" word `0x30000114` is nonzero; otherwise 0. The hook returns 1 on the C-brick when either processor-module type byte is in {1, 3, 4, 5, 6, 7, 10} (`0xffe4458c`, table bytes at `0xffe445b4`), and on the Origin 300 when the PSC code is 2, 3 or 4.
3. For iteration i = 0 to n-1: for i = 1 only, log `reset again MIPS` and wait **400 ticks** (`0x190`, `0xffe4f37c`); then the hold wrapper (section 2, without the parallel-port pulse); **10 ticks**; the release wrapper. An error from either wrapper is returned at once.

The "just powered" word is set to 1 by the power-up routine after its supply walk (package 0 `0xffe4d0e8`, package 1 `0xffe4cd38`) and cleared by the release wrapper's `0xffe4c828`. So the second reset happens **only inside `power up`** (the call `reset(5)` at package 0 `0xffe4d15e`, package 1 `0xffe4cdae`), and only for those module types. A `reset` typed afterwards, a front-panel reset and a request-driven reset (reason 3) see the word at 0 and do one pass. E for the code; I for "this is a MIPS-module retry" (name from the log line only). The existing note described `reset` as a single hold, 10 ticks, release; that holds for every case except `power up`.

Cost of one pass, C-brick, in ticks: hold = 10 per present module with type in the set; wrapper gap = 10; release wrapper = 10 + hook; release hook = 30 + 13 per present set-type module + 3 per present other-type module. Two in-set modules: 20 + 10 + 10 + 30 + 26 = 96 ticks, 0.96 s at the inferred 100 Hz. In `power up` the second pass follows 400 ticks after the first release returns (4 s at 100 Hz), so the node sees two full hold and release cycles about 4 s apart. E for the arithmetic, I for seconds.

The `reset hold` command (`0xffe198f4`) calls `0xffe4f2f0(1)` (hold only, with the parallel-port pulse). The `reset` command is refused with "ERROR: power appears off." unless the supply-state routine reports all on (E, [power-reset-cbrick.md](l1/power-reset-cbrick.md)).

## 6. Triggers of each hand-off

| Trigger | Calls | Reason | Gating | Grade |
|---|---|---|---|---|
| `reset` / `reset hold` console commands (entries `0xffe1eb5c`, `0xffe1eba0`; handler `0xffe1988c`) | `0xffe4f324` / `0xffe4f2f0` | 1 | all supplies on | E |
| `power up` / `power up hold` / `power down` (`0xffe19740`) | `0xffe4f2f0` (hold), then `0xffe4f324(5)` for plain `up`; `0xffe4f2f0` inside `power down` | 5 | see power notes | E |
| General request `0x2001` (handler `0xffe1335a`, both packages) | `0xffe4f324(3)` | 3 | none seen; no reply on success | E (request table: `request-codes.tsv`) |
| Front-panel reset button (change hook of `DISPLAY CONTROL` input bit 2, `0xffe51ef6` onward) | `0xffe4f324(2)` | 2 | setting `0x79` nonzero refuses with "Reset Btn Disabled"; log text "Reset attempted from disabled front-panel reset button" | E; no brick-type test seen on this branch |
| Front-panel NMI button (input bit 1) | `0xffe4f0a4(2, 0xff)` | 2 | brick type in `C 3 5 4 A 6 9 7 8 0 Q @ O B F H L`; setting `0x78` nonzero refuses with "NMI Btn Disabled"; display shows "NMI...", then "NMI done" | E |
| `nmi`, `nmi <exp>` console commands (entries `0xffe1eba8`, `0xffe1ebec`, `0xffe1ec30`; handler `0xffe1bc5c`) | `0xffe4f0a4(1, node)` | 1 | all supplies on; brick sets `c12!`, `C1256@`, `C125@` (literal strings): **not the Origin 300 type `4`** | E for the sets; the first set's lower-case `c` is as the bytes read |
| `softreset`, `softreset <exp>` (entries `0xffe1ec74`, `0xffe1ecb8`; handler `0xffe1bea0`) | `0xffe4f0ec(1, node)` | 1 | all supplies on; sets `OC14567890QA!@BDSFHL` and `C56@`: includes `4` and `C` | E |
| `nmi enable button` / `disable button` (entries `0xffe1cf84`, `0xffe1cfc8`) | writes setting `0x78` | - | - | E for the setting number; the exact flag value used for "disabled" is I |

The front-panel hook's buttons are bits 0 (power), 1 (NMI) and 2 (reset) of the `DISPLAY CONTROL` input byte, each acted on a 0 to 1 change after the optional inversion by bit 1 of the byte at `0x30001b88` (package 0 `0xffe51bae` to `0xffe51c98`), with a debounce word at `+0xc` of the hook's state at `0x3000018c`. E. This settles the open item in [ip45-io-expanders.md](l1/ip45-io-expanders.md): input bit 1 is the NMI button and bit 2 the reset button. The Origin 300 therefore has an NMI button path but no console `nmi` command in this image (U for whether a different package serves a real Origin 300, the open question in [power-reset-cbrick.md](l1/power-reset-cbrick.md)).

## 7. Power-state notice to the peer

Source of truth and the event: [power-reset-cbrick.md](l1/power-reset-cbrick.md) sections 2 and 7. The ordered facts a model must reproduce, each re-checked here:

1. Trigger: the setter `0xffe33268(w)` (package 0) compares `w` with record 0's word at `+0x34`; only when different does it store `w` and send. E.
2. Sender `0xffe33060(kind = 4, 0, 0)`: builds an event with three integer arguments in order **rack** (record 0 `+8`), **bay** (`+0xc`), **power word** (`+0x34`), kind 4 being the event code, and hands it to the event sender `0xffe79084` on the queue object at `0x30004e98`. E.
3. The word: 1 after `power up` completes (step 7 of the C-brick power-up, `0xffe4cdca` area), 0 after `power down` (`0xffe4cb1a`), and a start-up value from the supply-state routine (1 all on, -1 some, 0 none). E. A `reset` does not call the setter, so **a reset sends no notice**. E.
4. Order against the other hand-offs: on `power up` the notice follows the (possibly double) reset release and precedes the display text "Powered Up"; on `power down` it follows the supplies going off, the event-subchannel clear and the environment-monitor phase 4, and precedes a 40-tick delay. E.
5. Delivery and receipt: the peer-link event handler `0xffe2efe8` copies task-14 events to peer slots 1 and 2 when open; the receiver `0xffe3380c` stores the third argument at `+0x34` of the record whose rack and bay match, and does nothing else. E for both ends as read, I for the end-to-end path (not traced as one run). The event kinds 5 and 6 use `+0x38`, `+0x3c` through the same sender. E.
6. Whether an attached L2 sees the same event over USB, and what it does: U (section 8).

The notice carries no information about NMI, soft reset or Bedrock state; the peer's only automatic reaction found is that the auto power-up countdown abandons itself when any record's power word is nonzero. E (see the C-brick note).

## 8. Open items and the exact software test for each

| Item | Why open | Test that settles it |
|---|---|---|
| Wall-clock length of a tick | no image states the processor clock; 100 Hz is inferred from timer-1 reference `0x752f`, prescale 16, at an assumed 48 MHz | read the clock from a public MCF5206e board schematic or hardware guide for the L1 board, or boot the image under a ColdFire model with a stated clock and count `0x30003c10` against model time; any one value fixes every delay in this note |
| Which Bedrock pin each `CNTL` bit reaches, and what is preserved by `reset`, `softreset` and `nmi` | image labels only; no Bedrock datasheet is public | read the IP35 PROM or the IRIX kernel's early reset-cause reporting for the reset-cause values it prints after each L1 command (the PROM carries `Soft reset received` and a reset-cause path); record which register fields it reads, then match command to field. Host-side, software only |
| NMI pulse width | two back-to-back expander writes, no delay | measure the model's two-wire transaction time with the image's own divider (`0x13`, [mbus.md](l1/mbus.md)); the gap is one stop, one start and the address byte |
| Whether the expanders are rewritten with applied values on an L1-only restart | the sanity routine's nonzero return is not decoded | read `0xffe4a7b0` (package 1) and `0xffe4a400` against the reboot-record word `0x301019ac` in both packages |
| Whether the parallel-port pulse (`0xffe7219c`) and the scan call (`0xffe72214`) ever run | gated by word `0x30000044` (zero in package 1's initial data, no absolute writer) | search the persistent-setting load routines for a store through a base register to `0x30000040 + 4`; read the C-brick initial-data image at its copy source (not located in package 0) |
| What the hold wrapper's port-down and the release wrapper's port-up do on the wire | per-board port records not tabulated | tabulate the port records of the C-brick and Origin 300 registration tables (`port.c` literals) and the `PORT CONTROL` expander bits (`0x55`/`0xdd` masks), then replay `0xffe50288`, `0xffe500fc` and `0xffe509a4` |
| What `0xffe36a04` (ports 2 to 9) and `0xffe52a30` put on a link | not followed | read `0xffe54a84`'s jump table (50 cases) and `0xffe3681c`; list UART writes |
| Meaning of the Origin 300 predicate `0xffe4c2fc` | iterates supply records and compares bits | read `0xffe4c270`, `0xffe4c10c` and the supply record fields |
| Whether an L2 sees the power-state notice | the USB-side event path was not traced | read the USB write queue consumer for task-14 events and the L2 daemon's handler for event code 4 (`sub_1000e664` neighbours in [l2/config-request-reply.md](l2/config-request-reply.md)) |

## 9. What this note changes in earlier notes

- `reset` is hold, 10 ticks, release **once**, except inside `power up`, where it runs twice with 400 ticks between for the processor-module types in section 5 (earlier note: always once).
- The hold and release wrappers also quiesce and restore every L1 port (the `port_down` family, text `clr LPOK`/`force link`), which the earlier note listed as three unfollowed calls.
- The C-brick release order is: strap, 10, clear bit 3, 10, set `CNTL 1` bit 0, 10, module step, set `CNTL 2` bit 0, set `CNTL 1` bit 1 (earlier note placed the module step before the set of `CNTL 1` bit 0).
- Origin 300 hook bodies are tabulated in section 4; `ASIC CTRL 1` and `2` carry the same bit uses as `BEDROCK CNTL 1` and `2`.
- `DISPLAY CONTROL` input bit 1 is the NMI button and bit 2 the reset button.
- The tick is established as one timer-1 interrupt; its length stays inferred.
