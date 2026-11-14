# L1 1.44.0: the C-brick power-up window, I/O-brick supply enables, expander state across an L1 restart, and the expander poll

Four questions about how the ColdFire L1 treats its power and reset expanders, answered from the shipped image. They extend [power-reset-cbrick.md](power-reset-cbrick.md), [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md) and [ip45-io-expanders.md](ip45-io-expanders.md) and correct them where a re-read found more (end of the note).

Read from L1 1.44.0 of the IRIX 6.5.30 `l1.bin`, package 0 (`Base 1MB`, the C-brick, I-brick and R-brick body, load address `0xffe00000`, first program copy) through a linear listing made with the same decoder settings as [disasm/](disasm/) (capstone 5, 68040 mode; not committed, 17 MB). Package 1 was used to cross-check: the routines named below are instruction-identical with address operands masked unless a section says otherwise. Grades as in [../../AGENTS.md](../../../AGENTS.md): E established (the routine or table was read in full), I inferred, U unknown. A "tick" is the argument unit of the delay routine `0xffe40e54` and of the task timeouts. Timer 1 is programmed with reference `0x752f` and a system-clock divide of 16 ([mcf5206e-peripherals.md](mcf5206e-peripherals.md)), which is 100 Hz when the system clock is 48 MHz; the image does not state the clock, so seconds below are inferred (I).

## 1. What a C-brick reads between enabling its supplies and releasing reset

Answer: almost nothing, and nothing that it waits for. The power-up routine (`0xffe4cb4c`) never polls a power-good, VCC-OK or voltage reading for a `C` brick, has no timeout and no retry. Between the last supply enable and the reset release it makes two settings reads and one bus read per port. E.

The walk for type `C` (it takes the generic arm at `0xffe4ce8c`; only type `4`, the Origin 300, takes the polling arm at `0xffe4ccce`, see [ip45-io-expanders.md](ip45-io-expanders.md)):

1. Per record of the 20-record supply table (`0xffe457c4`), in order, skipping records whose expander byte (record `+2`) is `0xff`: write the supply's enable bit (`0xffe4c76c`; a bus write of one expander byte, no read-back), then wait 25 ticks (0.25 s I). Records that switch, with expander and mask: record 0 `FAN CONTROL` (index 11) `0x80`; record 1 `PIMM0 VCPU`, `PIMM0 POWER` (12) `0x01`; record 2 `PIMM1 VCPU`, `PIMM1 POWER` (14) `0x01`; record 3 `DIMM`, `POWER CONTROL` (9) `0x20`; record 4 `POWER CONTROL` (9) `0x01`; record 10 `PIMM0 SRAM`, expander 12 `0x08`; record 15 `PIMM1 SRAM`, expander 14 `0x08`. Records 5 to 9, 11 to 14 and 16 to 19 have no enable bit. Which records are active depends on the per-record enabled byte, which board code clears for modules that are absent (U, see the open item in [power-reset-cbrick.md](power-reset-cbrick.md)). The first supply to be switched, when none is on, is preceded by the fan start (writes only; an error aborts the walk) and a 50-tick wait.
2. A 100-tick wait (`0xffe4cf40`).
3. The board hook at power-table offset `0x20` (`0xffe4cf4c`): the C-brick registration passes zero for it (`0xffe45448`, args 0, 0, 20, table), so it is skipped. E.
4. Two persistent-setting reads, keys `0x3d` (61) and `0x0d` (13) (`0xffe4cf68`, `0xffe4cf76`). NVRAM, not a device. For `C` they only choose the generic port arm below: the per-port arm is for types `R D 6 @` with key `0x3d` equal to 0 or 2 and the system serial not starting with `M`.
5. The port bring-up for all ports (`0xffe50b74(0xff, 3)`, call at `0xffe4d01c`). The C-brick registers two port records (`0xffe459f4`, 24 bytes each; registration `0xffe4f454`). At registration each port's state byte is set to 2 if its control expander (index 8, `PORT CONTROL`) and the second expander named in the record (index 6, `BEDROCK CNTL 1`) are enabled, else 0; both are enabled for the C-brick, so both ports are processed (the skip tests at `0xffe50c10` and `0xffe50c70` pass: port 0 has flag `0x40`, port 1 has a zero field at `+0x10`). Per port the helper `0xffe509a4` takes the port bus lock and makes one real read of `PORT CONTROL` (wire address `0x46`, bus segment 3, input mask `0x55`) through `0xffe65ffc`; port 1 uses the high nibble of the byte (shifted right by 4). It passes the result to the port state routine `0xffe504dc` and releases the lock. Another helper (`0xffe500fc`) writes the port enables (`0xffe50084`, `0xffe4feec`). E for the read and the lock; the port state routine was not followed past its first branches (U for what each bit value makes it do).
6. The environment monitor is told the power-up phase has ended (`0xffe2be6c`, which stamps the time and calls `0xffe2b8d4` with 4). It sets flags and returns; the monitor workers act later (I, from [environment.md](environment.md)).
7. For type `C` a 50-tick wait, then the reset release.

Failure behaviour (E):

- A supply enable that fails ends the walk at once, and so does a failed fan start before the first supply: the routine runs the power-down with reason 5 and returns the error (`0xffe4ced2`). Nothing later runs, and the reset is not released.
- An error from the port bring-up is latched (the first one wins, register d3) but does not stop the sequence. The reset release at `0xffe4d15e` still runs. The latched error changes the result: the routine displays `Power Up Error` (`0xffe4d1a0`), does not set the record-0 power state to 1, and returns the error. An error from the reset release itself is latched the same way.
- No timeout exists in this window. The loop that waits on bit `0x40` of expander 4 and then on four voltage readings is the `4` arm only.

Not read by this routine at all: `PIMM0 VCC OK` and `PIMM1 VCC OK` (expanders 13 and 15, input mask `0x3c`), the status bits of `POWER CONTROL` (expander 9, mask `0xde`), the temperature and voltage monitors, the fan tachometers. Expanders 13 and 15 have a change hook that only writes a log line (`0xffe65b14`) and no interrupt-enabled bits, so a change in their bits is never reported either; the poll of section 4 does read them (index range 0 to 15). I: no consumer of the VCC OK bits was found in the power-up path; a scan of every read call site by index was not completed (U).

## 2. Supply enables of the I-brick family and the R-brick

The registration routine `0xffe46028` selects a table pair by brick type: `I` (`0xffe4606a`), `P` (`0xffe46120`), `X` (`0xffe461ee`), `Y` (`0xffe462a6`), `N` (`0xffe46370`), `&` (`0xffe4645a`); the R-brick has its own initialiser (`0xffe4a234`, tables `0xffe4a3bc` and `0xffe4a460`). The type letters map to the host-side names in [personalities.md](personalities.md): `R` router, `P` PBRICK, `I` IBRICK, `X` XBRICK, `Y` X2BRICK. E for every table.

Enable polarity (E, `0xffe4c4bc`): a bit is "on" when clear, for every supply type in these tables (only type `0x17` is active high and none of them contains it). Cold start writes `0xff` to the enable expanders (input bits ORed in), so every enable reads off. Order is table order, skipping records with no enable bit.

| Letter | Enable expander (index, wire address) | Bits cleared in order | Cold-start byte |
|---|---|---|---|
| `I` | `POWER ENABLE` (0, `0x70`) | `0x01` (48 V), `0x04` (12 V bias), `0x08`, `0x10`, `0x40` (12 V bias), `0x20`, `0x80` | `0xff`; bit `0x02` unused |
| `P` | `POWER ENABLE` (0, `0x70`) | `0x01`, `0x04`, `0x08`, `0x10`, `0x20`, `0x40`, `0x80` | `0xff` |
| `X` | `POWER ENABLE` (0, `0x70`) | `0x01`, `0x04`, `0x08`, `0x10`, `0x20`, `0x40` | `0x7f` ORed with input bit `0x80` = `0xff` |
| `Y` | `48V ENABLE` (0), then `POWER ENABLE` (2); both register-pair form on wire `0x80`, registers 0 and 2 | 48 V: `0x02`; then `0x02`, `0x04`, `0x08`, `0x10`, `0x20`, `0x40` | `0xff` each |
| `N` | `48V ENABLE` (0, register 0), `POWER ENABLE` (1, register 2), wire `0x80` | 48 V: `0x02`; then `0x02`, `0x04`, `0x08`, `0x10` | `0xff` each |
| `&` | `48V ENABLE` (0, register 0), `POWER ENABLE` (2, register 2), wire `0x80` | 48 V: `0x02`; then `0x02`, `0x04`, `0x08` | `0xff` each |
| `R` | `POWER CONTROL` (0, `0x72`) | `0x01` (48 V), `0x04` (supply type `0x0f`) | `0xff` (input bits `0xe0`); bit `0x08` is the reset line, see below |

The bits not listed belong to records with no enable (monitored only: the 3.3 V and 5 V auxiliary supplies that stay up with the L1, and on `X` one more monitored record, type `0x16`). On `X` bit `0x80` of `POWER ENABLE` is an input. The status expanders (`POWER STATUS 1` and `2`) are read-only inputs.

Sequence and delays, identical for all of these letters (E, the same routine as section 1, `0xffe4cb4c`):

1. Serial-number check, skipped for type `A` and when setting `0x50` is non-zero; errors `-0x834` to `-0x838`.
2. Reset hold (`0xffe4f2f0`). `I`: `XBRIDGE CONTROL` (index 5, wire `0x48`) bits `0x0b` driven low. `R`: `ROUTER CONTROL` (index 2, wire `0x46`) bits `0x0d` := `0x01` and `POWER CONTROL` bit `0x08` cleared. The hold arms of `P`, `X`, `Y`, `N` and `&` were not re-read (U); `N` and `&` register per-port `XBRIDGE CNTL` expanders instead of index 5.
3. For the first enabled supply, while none is on: the fan start (writes only), then 50 ticks (0.5 s I).
4. Each enable bit cleared in the order of the table above, 25 ticks (0.25 s I) after each. Types `A` and `6` have different first delays; none of these letters is one.
5. 100 ticks (1 s I).
6. Port bring-up (`0xffe50b74`). `I`, `P`, `X`, `Y`, `N`, `&`: the generic arm for all ports. `R`: ports 0 to 5 one at a time when setting `0x3d` is 0 or 2 and the system serial does not start with `M`, else the generic arm (E for the code; which ports an R-brick registers is the `PORT HA/BC/DE/FG CONTROL` set of [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md)).
7. No 50-tick wait for any of these letters (the jump table at `0xffe4d10a` sends `I`, `P` and `N` straight on and `X`, `Y`, `&` and `R` fall outside it), then `reset` (`0xffe4f324`): hold, 10 ticks, release. Release values are in [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md) for `I` and `R` (the `X`, `Y`, `N`, `&` and `P` release arms were not re-read, U).
8. Record-0 power state := 1 and `Powered Up`.

`power down` clears nothing and sets the enable bits again in reverse table order with no delay between them, after holding reset; it does not run the fan start in reverse (`0xffe4c904`, E).

Active-low supply enables, one write per bit, and no read-back is the whole contract: a board answers by accepting the byte writes.

## 3. An L1-only restart: are the expander outputs re-applied or kept

Answer: they are kept when the restart was a watchdog reset and the brick looks powered and consistent; they are re-applied from the table otherwise. The decision is made by the sanity routine `0xffe4a7b0`, called from `0xffe4a83c` at the end of the board registration (E).

The cause flag. The kernel entry (`0xffe10b2e`) calls `0xffe54ebc`, which calls `0xffe56964`. That routine reads the reset status register (`MBAR+0x40`), writes bit 5 (watchdog reset) to the retained block at `0x30101990` offset `0x1c` (so word `0x301019ac`), then writes the register back to clear it. It runs before any task or board code. A reset caused by the watchdog (which includes the firmware's own restart for the `reboot_l1` request, see [board-boot-contract.md](board-boot-contract.md)) leaves the flag set. A power-on or external reset leaves it clear. E. The same block carries the retained-RAM magic `0xa1b2c3d4`; the flag is rewritten on every start, so retained RAM does not matter for it.

The decision (`0xffe4a7b0`):

1. Flag clear: return -1. `0xffe4a83c` then applies every enabled descriptor (`0xffe6664c`): shadow := applied value OR input bits, one write, one read-back into the last-read byte, report mask := the descriptor's interrupt-enabled bits. (It first calls `0xffe671d8`, not read.) This is the cold start; the C-brick values include `BEDROCK CNTL 1` = `0x72`, `BEDROCK CNTL 2` = `0x8a` and every supply enable `0xff` (off). E.
2. Flag set: read every enabled expander once from the device and store `read OR input bits` as the shadow (`0xffe6607c`). A read error logs "could not read I/O exp data values from HW" and returns -1 (apply).
3. Board hook at descriptor `+0xc8` ("brick-specific soft reset check"): not installed for the `C` brick (`0xffe45504` writes up to `+0xb8`), so skipped; a non-zero result returns -1.
4. Otherwise the supply-state routine (`0xffe4c6ac`, which reads the shadow, not the bus): none on returns -1 with no message; some on logs "mixed power supply state" and returns -1; all on continues. For types outside `0 7 8 9 E O Q V` the fan check (`0xffe4b45c`) must agree ("power supplies ON, fans are OFF"); the display expander must show the POWER light on and FAULT light off.
5. Finally the report-mask step (`0xffe66488`) sets each expander's report mask from its descriptor's interrupt-enabled bits; for the register-pair and form-2 expanders it reads the device's own mask register first and an error returns -1. Every C-brick descriptor is form 0, so no bus access happens and this step cannot fail there. An enabled expander with report bits that no hook can receive gets reporting enabled through `0xffe66294`.
6. A clean pass returns 0 and `0xffe4a83c` skips the application. The shadows are therefore the live device bytes, the last-read bytes are the same bytes and the report masks are the descriptors'. Nothing is written to any expander output. E.

So an L1 watchdog restart on a running, consistent brick leaves supplies, reset lines and the other outputs exactly as they were; the same restart on a powered-down brick, or a mixed or inconsistent one, rewrites the table's cold-start bytes (all supplies off, reset bits at their applied levels). Any non-watchdog reset always rewrites them. The "L1 rebooted" log line on this path and the "L1 running" line on the other are at `0xffe10720` to `0xffe10754`.

A board that wants the cold-start path on a restart must make the reset status register report no watchdog reset; one that wants the keep path must have a watchdog-caused reset, the display lights as the sanity step expects and all supplies reading on.

## 4. The expander input poll

Answer: there is no polling task of its own. The expander inputs are scanned by the worker of the line 1 interrupt aggregator, once whenever its semaphore is signalled and in any case every 300 ticks (3 s I), or every 1500 ticks (15 s I) while the RAM word at `0x30000b00` is non-zero.

Detail (E unless marked):

- The kernel creates two aggregators (`0xffe6c2f4`, calls to `0xffe6c204(struct, line, second number, ...)`): one at `0x30003294+0x154` on external interrupt line 6 (second number 5), and one at `0x30003294` on line 1 (second number 10; its meaning is unknown, U). The expander poll callback is registered on the line 1 aggregator (`0xffe6c15c`; the C-brick registration call is at `0xffe454d8` with first index 0 and last index `0xf`). The monitor-chip poll callback is registered on the line 6 aggregator (`0xffe6c0d4`, call at `0xffe454ee`, indices 0 to 6).
- Each aggregator's worker (`0xffe6c3ac`) loops: wait on its semaphore with a timeout (`0xffe4368c`, 300 ticks, or 1500 while the RAM word at `0x30000b00` is non-zero); on either result run every registered callback and add up the number of events they report.
- The expander callback (`0xffe66798`, range 0 to 15, so `DISPLAY CONTROL` at index 16 is outside it) visits each enabled descriptor that has a change hook and runs the per-expander poll (`0xffe66688`) on it: one bus read, XOR with the previously read byte, mask to the descriptor's input bits, and on a difference store the new byte and, if the changed bits are among the interrupt-enabled bits (descriptor `+7`), call the hook with (index, changed bits, new value). It repeats up to three times while changes keep appearing; a change in bits that are not reported is counted and logged once the count passes 200.
- After each pass with the line enabled the worker re-arms the line (it clears the source's bit in the interrupt mask register at `MBAR+0x36`). A signalled wake that finds no event 100 times in a row stops the re-arming (the line stays masked); 10 timed wakes in a row that find no event turn it back on. So with no input activity the scan interval is the timeout, 3 s, whether the line is armed or not, and a masked line is restored after about 30 s of idle timeouts (I for seconds).
- The word at `0x30000b00` is written by the handler at `0xffe17df4` (one of a switch of RAM words set by key, with `0x30000b04`, `0x30000b10`, `0x30000b14`); what drives it was not followed (U). Its reset value is zero (RAM zeroed at start), so the standard interval is 300 ticks.

Which expanders are polled, for the C-brick: indices 0 to 15 (all but the display); the descriptors with hooks and non-zero interrupt-enabled bits, whose changes are reported, are `POWER CONTROL` (9, `0xde`), `FAN CONTROL` (11, `0x40`), `PIMM0 POWER` (12, `0x36`), `PIMM1 POWER` (14, `0x36`) and `PORT CONTROL` (8, `0x11`). `COM CONTROL` (5) and the two `VCC OK` expanders (13, 15) have a hook but no report bits. The display expander has its own handler (`0xffe51b3c`) outside this poll. E for the table, I for the summary.

## Corrections to the earlier notes

- [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md) calls `0xffe4b6cc(0xff)` a check on the first supply. It is the fan start: for each enabled fan record of type 1 it clears an enable bit, for type 2 it writes a fan-controller register. No read.
- The same note says the sanity routine returns non-zero "at once when the RAM word at `0x301019ac` is clear" and that "no instruction stores to its absolute address". The word is `0x30101990 + 0x1c`, written through a pointer by the reset-cause routine (section 3), and it is the reset-status watchdog bit, not a retained marker.
- [environment.md](environment.md) and [mbus-environment-task.md](mbus-environment-task.md) attribute the 1500-tick wait to a "management-link condition". The word that selects it is set by a RAM-word handler (section 4); the link condition was not found.
- [ip45-io-expanders.md](ip45-io-expanders.md) leaves open whether the application is skipped on an L1-only restart. Section 3 answers it.

## Open

- What the port state routine does with the `PORT CONTROL` bits (section 1, item 5).
- Whether the `X`, `Y`, `N`, `&` and `P` reset-release arms differ from `I` in a way that matters for the supply order.
- The system clock frequency, hence the seconds.
- What sets the RAM word at `0x30000b00`.
