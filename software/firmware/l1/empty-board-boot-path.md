# L1 1.44.0 ColdFire packages 0, 1 and 2: the boot path on a board with nothing behind the chip selects

This page answers one question for three packages: what does the shipped controller program do if the flash holding it and the RAM it runs in are the only devices on the external bus, and every other device (battery-backed NVRAM and clock, quad UART, USB controller, anything on the two-wire bus) is absent? "Absent" means a read returns a value the host picks and a write is lost. It lists the accesses in order, says which reads change the path and where each outcome leads, and states the idle behaviour and the reset-status truth table.

Images: IRIX 6.5.30 `l1.bin`, L1 1.44.0. Package 0 `Base 1MB image`, body sha256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`; package 1 `Fuel/PE/O300 1MB image`, `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`; package 2 `MIPS 2MB image`, `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`. Body layout in [image-layout.md](image-layout.md). Packages 0 and 2 are the Origin 3000 C-brick images; package 1 is the Origin 300 image read in earlier notes ([board-boot-contract.md](board-boot-contract.md), [../../../devices/l1_board/README.md](../../../devices/l1_board/README.md)).

## 1. How this was established

Two kinds of evidence, kept apart in the grades.

- **Read.** The loader (all of it), the board set-up routines, the NVRAM header check and repair, the clock read, the quad UART probe, the identity ladder, the watchdog and restart routines and the idle loop were disassembled and read in package 1; the same routines were located in packages 0 and 2 by the routine pairing in [../../../evidence/personality-diffs/routines.tsv](../../../evidence/personality-diffs/routines.tsv) and their bytes compared where the pairing says same bytes or same code moved.
- **Executed.** Each body was also run instruction by instruction in a ColdFire (V4e) interpreter with a host model: flash and RAM real, the on-chip registers backed by a register file plus a model of the timer tick, the two-wire module and the on-chip UART status, and every external device replaced by a policy (read returns zero, all ones, a fixed byte, or reads back what was written). The run gives the ordered access log and the messages the program wrote. It shows what the program does under that model, not what a board does; every executed statement below is graded **inferred** at most, and says which policy it was.

Executions covered: all three packages with every external device absent and the policies zero, all ones and a fixed byte `0xa5`; all three with the NVRAM backed by plain storage and the quad UART modelled as a 16C654-style register file; package 1 with several two-wire behaviours (no completion, stuck busy, completion without acknowledge) and several line-status values; the loader alone with seeded retained records and a range of reset-status bytes in all three packages.

Offsets: package 1 addresses are first-bank addresses; the second bank is `+0x70000` (packages 0 and 1) or `+0xf0000` (package 2). On a cold start the loader runs the **second** bank, so the executed addresses are second-bank ones; the table in section 3 gives first-bank addresses.

## 2. Windows the program touches (all three packages the same unless marked)

| Window | Role | Read as |
|---|---|---|
| `0xffe00000` upward | boot flash (global chip select 0 until its mask register is written, then a 2 MB window; 1 MB parts are mirrored in packages 0 and 1) | real |
| `0x30100000`-`0x30101fff` | on-chip SRAM (retained record, stack, vector copy) | real; contents at power-on arbitrary |
| `0x30000000`+ | external RAM on chip select 2, 128 KB or 512 KB | real; holds all program data |
| `0x10000000`+ | on-chip peripherals once the base register is valid | chip, not external |
| `0x20000000`+ | chip select 1: NVRAM, log ring, clock | **absent** |
| `0x40000000`, `0x60000000` | chip selects 3 and 5: USB controllers, second quad UART | **absent** |
| `0x50000000` | chip select 4: quad UART | **absent** |

The two-wire module is on chip; the devices on its bus are absent, so the module sees no acknowledge.

## 3. Ordered accesses from reset to the first console byte

Routine addresses are first-bank, per package (package 0 / package 1 / package 2). The shared loader is at the same addresses in all three.

| Routine | Pkg 0 | Pkg 1 | Pkg 2 |
|---|---|---|---|
| reset entry, loader, record check | `0xffe00400`, `0xffe00a80`, `0xffe008fc` | same | same (upper bound constants differ) |
| board set-up (early hardware init) | `0xffe54ebc` | `0xffe54b0c` | `0xffe75320` |
| interrupt and SIM registers / chip selects | `0xffe54d10` / `0xffe54d88` | `0xffe54960` / `0xffe549d8` | `0xffe75174` / `0xffe751ec` |
| reset-cause capture | `0xffe56964` | `0xffe565b4` | `0xffe76dc8` |
| log writer / formatted print | `0xffe5a92c` / `0xffe5b904` | `0xffe59f00` / `0xffe5aed8` | `0xffe7ad90` / `0xffe7bd68` |
| clock read | `0xffe5e674` | `0xffe5e32c` | not located by pairing; read in package 1 only |
| two-wire driver start / bus-free wait / ISR entry | `0xffe6cc98` / `0xffe6cdc0` / `0xffe6d318` | `0xffe6cb48` / `0xffe6cc70` / `0xffe6d1c8` | `0xffe8e9cc` / `0xffe8eaf4` / `0xffe8f04c` |
| quad UART probe | `0xffe639f8` | `0xffe636b0` | `0xffe850ac` |
| on-chip UART open | `0xffe63130` | `0xffe62de8` | `0xffe83acc` |
| watchdog arm / restart | `0xffe56684` / `0xffe56a04` | `0xffe562d4` / `0xffe56654` | `0xffe76ae8` / `0xffe76e68` |
| console task (opens the console UART) | `0xffe36f74` | `0xffe36f74` | `0xffe3cfe4` |

Packages 0 and 2 execute the same sequence of device accesses as package 1 in every run (same register values, same order of windows, same log messages); only addresses and RAM constants differ. Differences of substance are listed in section 8.

### 3.1 Up to the banner (the first log line)

1. **Flash.** Vector fetches. **On-chip SRAM:** RAMBAR write (`0x30100001`), stack and status register set.
2. **On-chip SRAM:** read the retained record marker word. **Class: value-insensitive for the path.** Marker present or absent, the loader ends in the same scan (section 5); the record is zeroed, marked and stamped (version 1, length `0x4d8`) when absent. The reset-time `a6` value is stored in the record and never used for a decision.
3. **External bus:** one byte read at `0x10000040` before the base register is written (the "reset-status byte"). **Class: value-sensitive**, see section 5.
4. **Flash:** pointer table, the two 48-byte bank records, build-stamp strings, 1 KB copy to SRAM; vector base write. Flash-only; any content the image holds. A damaged table or record sum removes that bank from the choice; both removed gives the loader's halt word.
5. **Program entry (second bank on a cold start).** RAMBAR, base register write. From here `0x10000000` is the on-chip register block.
6. **Board set-up, all on-chip registers except where marked.** Interrupt mask and the interrupt control registers, pin assignment, the byte at base+7; DRAM controller registers cleared; chip select 0 address, mask (2 MB), control; **flash probe**: 1024 words at the flash base compared with 1024 words 1 MB higher (class below); chip selects 1 and 2 written; **RAM probe**: read and clear a word 128 KB-4 into chip select 2, write all ones 512 KB-4 in, read the first word back (class below); chip selects 3 to 7 cleared; parallel-port direction cleared; **reset-cause capture** (on-chip status register read, bit 5 stored in the retained record, byte written back; record created if absent); vector copy flash to SRAM; zeroing of the RAM data area; copy of the initialised data from flash; the timer routine (copied to RAM) writes UART 2 output-port set, pin assignment, timer 1 and timer 2 modes and references, unmasks timer 1, pulses the shared reset-out/RTS2 pin (ten and 1000 pin-register reads as delays), all on chip.
7. **Cache enable, kernel start, main task.** No device access.
8. **NVRAM start-up (chip select 1).** Reads: the 8-byte header (sum and nibble tests), then, depending on what the header repair leaves visible, the record version nibble, the 112-byte settings record and its mirror, the 36-byte log-control record and its copy, the 0xb4-byte record; every repair is a block clear plus field stores plus a negated byte sum, written back. Classes in section 4.
9. **Banner.** The formatted print builds the text and calls the log writer, which reads the clock (section 4), then appends the text to the log ring through the ring pointers held in the log-control record, rewrites that record, its sum and its mirror. The banner goes to the NVRAM ring; no serial port is open.

### 3.2 Banner to the first console byte

10. **Two-wire bus start (on chip), identity.** Module reset (up to 1000 status reads, class below), divider, enable, interrupt unmask; then board record `0xff` is read: slave address byte `0xa4` written to the data register and the transaction waits. Record `0xfe` follows (`0xa8`) because the type is still `?`. In every executed run exactly these two identity reads occur. Records `0xfd` and `0xfc` are read only when the type is `%` (0x25) after the `0xfe` step (compare at `0xffe555be` in package 1); with all reads failing the type stays `?` and they are not read. This corrects the ladder in [../../../devices/l1_board/README.md](../../../devices/l1_board/README.md) section 11 step 8.
11. **Type fallback.** With the type still `?` the program reads stored setting 9 (a byte in NVRAM) and subtracts `0x23`; a value outside `0x23`-`0x59` (including 0x00 and 0xff) leaves the type `?`, a value inside selects the per-type arm of a 55-entry table. A blank NVRAM (settings repaired to zero) gives `?`.
12. **Quad UART (chip select 4).** Chip select 4 written (package 1 `0xffe54c62`...); then the probe: for each of four channels write 0 to the line-control register and `0xa0 | channel` to the scratch register (byte offsets 3 and 7 of each 8-byte channel block), then read the four scratch registers back. Any mismatch logs `UART invalid SPR register ...`, abandons the device without registering its port, and each later open of a quad channel logs `UART not probed`.
13. **On-chip UART 1** is opened (mode, status-clock, baud, vector and command registers at base+0x140...): receiver enabled; no byte is written yet.
14. Parallel-port set-up (on chip), then board registration (two-wire reads of the display expander and the board EEPROMs, each failing, each logged).
15. **Console.** The console task (software port 5, quad channel 2) opens its UART, then prints. First byte: see section 6.

## 4. Classification of every read on the path (inferred, from the reads plus executions)

| Read | Class | Outcomes |
|---|---|---|
| Retained record marker and fields (on-chip SRAM) | value-insensitive on a cold start | Any contents: the marker mismatches (probability about 2^-32) or is rewritten; with a coincidental valid marker the table in section 5 applies. |
| Flash vector and loader reads, table, records, stamps, copies | flash, not host-chosen | Fixed by the image. |
| Reset-status byte at the loader | value-sensitive (bit 5 only) | Section 5. |
| Flash size probe (1024 words at base against 1 MB higher) | value-sensitive | Equal (a mirrored 1 MB part) rewrites the chip select 0 mask to 1 MB; any difference keeps 2 MB. Package 2 is a real 2 MB image, so it keeps 2 MB. A 1 MB package with an unmirrored upper half (any host value that differs from the lower megabyte) keeps 2 MB; the only later consumer is the flash class stored for the update commands (class 1 for 1 MB, 2 for 2 MB, read back from the mask register). Never changes the boot path. |
| RAM size probe | value-sensitive, RAM must be storage | Word at `+0x1fffc` saved then cleared; all ones written at `+0x7fffc`; if the first word is still zero the size is 512 KB, otherwise the mask is cut to 128 KB and the end address stored. A 512 KB RAM gives 512 KB; a 128 KB RAM that aliases gives 128 KB. A bus that reads as a fixed value is not a valid RAM: the program's data lives there. |
| Reset-cause status read and write-back (on chip) | value-insensitive | Only bit 5 is stored; the byte is written back as read. |
| NVRAM header (8-byte sum, low nibble of byte 0, byte 1) | value-sensitive, one repair path | Header accepted only if the low byte of the 8-byte sum is zero and the nibble of byte 0 and byte 1 are nonzero. Zero bus: nibble zero, repair. All-ones bus: sum low byte `0xf8`, repair. Any fixed byte: sum zero needs the byte to be a multiple of 32, which makes the nibble zero: repair. So an unmodelled bus always repairs; the repair rewrites the header (clear 8 bytes, nibble 1, byte 1 = 1, negated sum in byte 7) and re-enters the record set-up. Because the writes are lost the program re-reads the host value again and may repair up to twice more in a row (one repair per failing test), then continues. |
| Record version nibble, record sums | value-sensitive | Nibble 0 or above 10 takes the default arm: the settings record checked by sum, repaired from its mirror, then initialised to defaults and stamped. Nibbles 1-9 take the matching upgrade arm. All arms end at the same place: the ring pointers. |
| Log ring pointers (start, end, write head, read head) in the NVRAM record | **value-sensitive and unsafe on an absent device** | The program checks the start and end against the expected NVRAM ring bounds, rewrites them if wrong, then reads the write and read heads back **from the same NVRAM** and uses them as byte pointers. With NVRAM that does not hold writes the heads are host values: zero bus gives pointer 0, all ones gives `0xffffffff`, `0xa5` gives `0xa5a5a5a5`. The ring writer stores the text through the head and scans for a newline through it. A host that maps nothing there faults or hangs. Executed: zero policy runs on (pointer 0 reads zero), all-ones and fixed-byte policies fall into a data-dependent jump to an invalid address shortly after the identity step and stop. NVRAM that reads back what was written gives a normal run. |
| Clock (read-latch bit set/cleared as read-modify-write of the control register; seven registers read, high nibble then low nibble) | value-insensitive | Straight-line BCD arithmetic into the time stamp of the log line; no loop, no validity test, no wait. Any values give a time stamp. |
| Stored settings 1 and 2 (rack, bay) | value-insensitive | Masked and stored (settings 1 with `0x3ff`, 2 with `0x3f`). |
| Identity record reads (two-wire) | value-sensitive | Failure (no acknowledge, or no completion before the timeout) leaves the type `?`. See section 7 for the bus outcomes. |
| Stored setting 9 | value-sensitive | Section 3.2 step 11. |
| Quad UART scratch read-back | value-sensitive | Must equal `0xa0 | channel` for all four channels. A bus that returns a constant cannot pass (channel 0 would need `0xa0`, channel 1 `0xa1`). Reads that return what was written pass. |
| Quad UART line status (transmit-holding-empty, bit 5) | value-sensitive | The driver sends a byte only when bit 5 is set; executed with status `0x00` or `0x40` no byte is sent and the console task stalls (no timeout seen within 4 million instructions); `0x20`, `0x60` and `0xff` send. |
| On-chip UART status | on chip | Model returned transmit-ready; not host-chosen on a real board. |

## 5. The reset-status byte read (the loader and the later reset-cause routine)

**What the byte is.** Established from the public MCF5206e User's Manual, revision 1 (page numbers printed): at reset the base register's valid bit is cleared and its remaining bits are uninitialised (8-2, "At system reset, the MBAR valid bit is cleared to prevent incorrect references to resources before the MBAR is written"); an access is claimed by an internal peripheral only while the valid bit is set (8-2); the chip select 0 address register and mask register reset to zero and its control register to one of `3C1F`...`3DDF` set by the interrupt pins at reset (9-27, table 9-4); "After system reset, CS[0] is asserted for all accesses except for CPU space accesses ... and internal peripheral accesses ... CS[0] operates in this manner until CSMR0 is written" (9-8, section 9.3.2); an external cycle with no acknowledge inserts wait states until one arrives, and ends with an internal bus error only if the bus monitor is enabled (9-10, 9-11; 6-44), and the monitor is off at reset (8-14, system protection control register reset 0). The loader never writes the chip select 0 mask and never writes the base register before its read, so the read at `0x10000040` is an external byte cycle on chip select 0, which is the boot flash. **Grade: established** for the chip behaviour. That the flash array answers at offset `0x40` (address wiring) is **inferred**: no image states the wiring. The byte the flash holds there is `0xff` in packages 0, 1 and 2 (next byte `0xe0`, also bit 5 set), established from the bodies.

**Bits used.** Only bit 5 is tested. Loader: `btst #5` after the byte read (package 1 `0xffe00acc`), one test, no other bit examined, no store of the byte. Reset-cause routine: `btst #5` (`0xffe56606`), stores "set/clear" as one word in the retained record, then writes the whole byte back to the on-chip register (write-one-to-clear) with no other bit examined. In packages 0 and 2 the same bytes at the paired routines. **Established** (read, package 1; package 0 same bytes; package 2 same code moved).

**Truth table for the loader, executed with every external device absent**, packages 0, 1, 2 identical modulo bank addresses (A first bank `0xffe10000`; B second bank `0xffe80000` in packages 0, 1 and `0xfff00000` in package 2). "valid" means the record check passes (marker valid, address aligned and in range, 48-byte record sums to zero, and the address is not the one stored at `+0x10`).

| Retained record | Bit 5 set (`0xff`, `0x20`, `0xa0`; the flash byte) | Bit 5 clear (`0x00`, `0x80`, `0xdf`) |
|---|---|---|
| marker absent | zeroed and marked, scan, **B** | same, **B** |
| marker, `+0x10` empty, `+0x18` empty | scan, **B** | **B** |
| marker, `+0x10` = A | scan skips A, **B** | `+0x10` cleared, scan, **B** |
| marker, `+0x10` = B | scan skips B, **A** | `+0x10` cleared, scan, **B** |
| marker, `+0x18` = A valid (`+0x10` empty) | **A** direct | `+0x10` cleared; `+0x18` ignored; scan: **B** |
| marker, `+0x18` = B valid (`+0x10` empty) | **B** direct | scan: **B** |
| marker, `+0x18` = A, `+0x10` = A | `+0x18` rejected (equals `+0x10`), scan skips A: **B** | **B** |
| marker, `+0x18` = B, `+0x10` = A | **B** direct | **B** |
| `+0x18` unaligned, out of range, or sum bad | ignored, scan: as the rows above | scan |

A halt (the loader's end-of-line halt word) results only when no candidate passes the record check: both banks damaged, or the single undamaged bank is the one stored at `+0x10`. None of the executed rows halts. On a bit-5-clear start the byte `0x00`/`0x80`/`0xdf` makes no difference from each other (only bit 5 is read). Hard start versus watchdog start: after a watchdog reset the chip also clears the base-register valid bit, so the loader's read is again the external flash byte; the `0x80`/`0x20` of the on-chip register is never seen by the loader. A cold start therefore differs only in the retained record (marker absent or arbitrary), not in the byte. With the shipped flash byte the clear arm is never taken (this is consistent with the earlier notes). If a host answers the read with `0x00`, `0x80` or `0xdf` the clear arm is taken on every start: a recorded restart image in `+0x18` is dropped and the scan runs.

**Reset-status register values the later routine sees (on chip):** `0x80` after a hard reset, `0x20` after a watchdog reset (manual 8-13). Only bit 5 changes the stored word; the stored word only gates the diagnostic that stores a stack back-trace into the record.

## 6. First console byte

The console is the quad UART channel 2 (software port 5), opened by the console task after the board identity step. On the empty board this port exists only if the quad UART passes its probe, which requires the scratch registers to read back; if it does not pass, the console task logs `can't open SMP read uart` at severity 9, which the log call turns into a controller restart (section 7), and **no console byte is ever written**.

With a quad UART that stores its scratch registers and reports transmit-ready, and with the NVRAM backed by storage, the executed first byte is `A` (`0x41`), the start of the line `ALERT: Error reading the display I/O expander, timeout` followed by carriage return and line feed, written to channel 2. The ALERT lines come from the failed two-wire reads of steps 10 and 14; the banner text of [console-behaviour.md](console-behaviour.md) (`SGI SN1 L1 Controller`) follows them, with prompt `---?---L1>`. Same byte, same text and same count of channel-2 bytes (298 in the five-million-instruction runs) in packages 0, 1 and 2. Which bytes come first if the two-wire reads did succeed is not established here: the ALERT path is what the empty board takes. **Grade: inferred** (executed; the text is produced by the shipped code, the model supplies the peripherals).

The first byte on any other UART is the on-chip UART 1 byte `0xa0` (the start byte of the power-bay link frame), executed as the 299th UART byte, after the console text. Quad channels 0, 1 and 3 never transmit in the executed runs.

Requirements for the first console byte: external RAM storage; NVRAM that reads back what was written (otherwise the ring pointer problem in section 4); a timer 1 tick; two-wire module that terminates every transaction either with a completion flag (no acknowledge) or by never completing (timeout); quad UART with working scratch registers and line-status bit 5.

## 7. Wait loops and hangs

| Wait | Exit | Timeout | Executed outcome |
|---|---|---|---|
| Two-wire bus-free wait | status bit 5 (bus busy) clear | counts at most 1000 status reads, then returns -405 and runs the bus reset (itself at most 1000 attempts) | Bus held busy: log `I2C reset failed`, controller restart; no console output. |
| Two-wire transfer completion (caller sleeps on a semaphore until the module interrupt) | interrupt with the status test `status & 0xb2 == 0xa2` | the sleep has a time-out in kernel ticks; the log text is `timeout` | Completion without acknowledge (bit 0 set): error -404 logged as `timeout`; no interrupt at all: the same text after the time-out. Both continue the boot. Needs the timer 1 tick; **without the tick the first transfer never ends and the program idles forever** (executed). |
| Quad UART transmit wait | line status bit 5 | none found in the driver loop | Never ready: no byte, the console task stays in the wait (no restart seen within 4 million instructions). |
| Flash toggle-bit poll (erase/program only) | two equal reads of bit 6; bit 5 time-out flag | the time-out flag only | Not on the boot path. |
| Idle spin | any ready task | none | Section 8. |
| External cycle with no acknowledge before the bus monitor is armed | an acknowledge | none until the monitor is enabled | **Hang on real silicon**: the monitor is armed only by the watchdog arm in the main task (system protection control register write `0xfe`, bus monitor on), which comes after the identity and UART steps. Any access to an unmapped address before that (for example through a wild ring pointer) never completes on silicon. A host chooses the value instead. |

The restart path: a severity-9 log (for example `can't open SMP read uart`) calls the restart routine, which arms the watchdog if not armed, stores the reason, then spins with interrupts masked until the watchdog resets the chip. Executed with a watchdog model: the board then boots again; five resets in nine million instructions, starting banks B, B, A, A, A, A, each with the loader read returning the flash byte. The bank alternation was observed, not derived. With a host that never expires the watchdog, the program spins at that point for ever (and the restart routine executes its halt only when the watchdog-enable bit reads clear).

## 8. Idle behaviour and HALT, packages 0, 1 and 2

**STOP.** The two-byte word `0x4e72` occurs in no body at an even offset (the only offsets an instruction or operand word can start at). It occurs at four odd offsets in packages 0 and 1 and six in package 2 (package 0 `0xffe604fd`, `0xffe6c565` and their second-bank copies; package 1 `0xffe601b5`, `0xffe6c415` and copies; package 2 `0xffe811c1`, `0xffe8e299`, `0xffe97ecf` and copies), each the upper byte of one word and the lower byte of the next (`..60 4e | 72 3c..`, `..01 4e | 72 00..`); none is reachable as an instruction. Established for all three. No STOP is executed.

**Idle.** One loop in each package, reached from the scheduler when no task is ready: package 0 first bank `0xffe41336`, package 1 `0xffe41336`, package 2 `0xffe473a6`. It lowers the status-register interrupt mask to the value held in a RAM word (zero at boot), then spins reading two RAM words (the highest-priority ready task and the next one) until either is nonzero. No STOP, no wait for interrupt, the CPU runs flat out. Established (read in all three; executed spin seen in all three).

**What wakes it.** Any interrupt whose handler makes a task ready: timer 1 (level 5, the 100 Hz tick that expires every timed sleep), the two-wire module (level 6), the on-chip UART (level 6, vector `0x42`), the quad UART (level 4), the USB and expander lines (levels 1, 2, 3, 6 as registered). A host that raises none sees the program execute the two loads and a branch for ever: the first sleep (the first two-wire transaction on the empty board) never ends. A host that raises only the timer 1 tick gets through the boot, as executed.

**HALT words that can execute.** Loader and shared: vector stub (`0xffe0083e`, taken by any exception before the real vector table is installed and by none after), loader end-of-line (`0xffe00b1c`, no candidate or the entry returns). Restart: package 0 `0xffe56a6c`, package 1 `0xffe566bc`, package 2 `0xffe76ed0` (taken only if the watchdog-enable bit of the system protection register reads clear; the second-bank copies `+0x70000`/`+0xf0000`). Package 2 also has three `0x4ac8` words at even offsets in the relocation table (`0xffeab156`, `0xffeb42aa`, `0xffebae6e` and second-bank copies), which are data inside `0xffeaa2bc`-`0xffebc97c`. Nothing else in packages 0, 1 or 2. None executed in any run except as noted.

## 9. Differences between the packages

- Loader: byte-identical in packages 0 and 1; package 2 differs in the three upper-bound constants only; truth table identical.
- Cold start runs the second bank: `0xffe80000` (packages 0, 1) and `0xfff00000` (package 2).
- Chip selects: identical writes except package 2 keeps the 2 MB chip select 0 mask (no mirror rewrite).
- Executed access order, messages and console text identical across the three.
- All numeric RAM constants differ.

## 10. Not settled

| Item | Why |
|---|---|
| Why the all-ones and fixed-byte runs jump to an invalid address | traced to the step after the identity string compare; the data it jumps through was not decoded. |
| Behaviour with a two-wire slave that acknowledges and returns blank data | the model for it was not validated (output identical to the no-acknowledge case, which is suspicious); not graded. |
| Whether a host that gives unmapped addresses an interpretable value changes the NVRAM-absent results | only zero and constants were run. |
| Bank alternation after repeated watchdog restarts | observed in execution, not derived. |
| Clock read routine location in packages 0 and 2 | pairing did not locate it; the access sequence is the same in the executed runs. |
