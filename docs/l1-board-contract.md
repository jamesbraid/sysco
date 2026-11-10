# L1 board contract: C-brick, I-brick and R-brick (ColdFire L1, firmware 1.44.0)

This is the hardware contract the shipped L1 firmware assumes on the C-brick, the I-brick and the R-brick: which on-chip peripherals and external devices it touches, at which address and width, what it writes and what it expects back, in the order it does so up to the first line it logs. It is a reading of the firmware, not of a board. Where the firmware does not say what a device is, the entry says so and names the software test that would settle it.

## 1. Scope, sources, grades

**Image.** IRIX 6.5.30 `usr/cpu/firmware/sysco/l1.bin` (sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), L1 version 1.44.0, build stamp `20060717181954`. Five ColdFire packages sit in the container; this note reads package 0, `Base 1MB image` (container offset `0x68`, 982,376 bytes, body sha256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`, load address `0xffe00000`). Its supported-brick string is `CIPXYN&?RG`, so one image serves the C-brick (type character `C`), the I-brick (`I`) and the R-brick (`R`) plus the P, X, Y, N, `&` and G boards. Every address below is a first-bank address in package 0. The second bank is the same program `0x70000` higher with relocated pointers ([personalities.md](l1/personalities.md), [package-container.md](l1/package-container.md)). Package 2 (`MIPS 2MB image`) carries the same C, I and R code; its tables were not re-decoded here (see unknown U15).

**Relation to the Origin 300 notes.** The shared base of package 0 pairs with package 1 (`Fuel/PE/O300 1MB image`, the reference reading in [board-boot-contract.md](l1/board-boot-contract.md)) as same bytes, same code moved, or same code with different RAM and pointer constants (the pairing is [personality-diffs/routines.tsv](personality-diffs/routines.tsv)). Where a fact below was read in package 1 and the package 0 routine pairs with it that way, the entry says "package 0 pairs" and gives both addresses; where the package 0 bytes were read directly the entry gives only package 0 addresses. The board-specific parts (descriptor initialisers, registration routines, device tables, reset and power hooks) were read in package 0 directly.

| Routine | Package 1 | Package 0 | Pairing |
|---|---|---|---|
| interrupt and SIM set-up | `0xffe54960` | `0xffe54d10` | same bytes |
| chip-select set-up | `0xffe549d8` | `0xffe54d88` | RAM constants differ |
| M-Bus controller bring-up | `0xffe6caa8` | `0xffe6cbf8` | RAM constants differ |
| M-Bus status check | `0xffe6cfac` | `0xffe6d0fc` | RAM constants differ |
| EEPROM read, header, SPD | `0xffe6b03c`, `0xffe6ba58`, `0xffe6b1e8` | `0xffe6b18c`, `0xffe6bba8`, `0xffe6b338` | same code, moved |
| NVRAM start-up | `0xffe5a188` | `0xffe5abb4` | RAM constants differ |
| watchdog arm | `0xffe562d4` | `0xffe56684` | RAM constants differ |
| quad UART probe and init | `0xffe636b0`, `0xffe638f4` | `0xffe639f8`, `0xffe63c3c` | RAM constants differ |
| USB controller probe | `0xffe6db6c` | `0xffe6dcbc` | same bytes |
| clock read | `0xffe5e32c` | `0xffe5e674` | same bytes |
| parallel-port scan set-up | `0xffe71d90` | `0xffe71ee0` | same code, moved |
| flash toggle-bit poll | `0xffe7f810` | `0xffe7f814` | RAM constants differ |

**Grades.** *Established*: the whole routine or table was read in the image and the claim follows. *Inferred*: reasoned from partial evidence; the reasoning is given. *Unknown*: no shipped image or public source settles it; section 12 lists the exact software test for each. A grade in the first column of a table applies to the whole row.

**Sources used.** The image above; the committed listings and tables in this repository; the IRIX L1 host tool command tables ([host-tool-options.md](host-tool-options.md)); the public Linux file `arch/m68k/include/asm/m5206sim.h` (v6.1) for MCF5206e register offsets; public Linux `drivers/hwmon/lm78.c` and `drivers/gpio/gpio-pcf857x.c` as hints for device behaviour. The Motorola MCF5206e user's manual is cited by chapter as the earlier notes do; its text was not retrievable in this pass, so no register bit field below is graded above inferred when it rests on the manual alone. Decoding used capstone in 68040 mode, which covers the ColdFire ISA_A subset the compiler emitted; data embedded in code decodes as nonsense and was checked against the raw bytes.

## 2. Boot vector, loader and flash layout

**Flash content (established, package 0 bytes).**

| Range | Content |
|---|---|
| `0xffe00000` word 0, word 1 | initial stack pointer `0x301017fc`, reset entry `0xffe00400` |
| `0xffe00004`..`0xffe003ff` | 254 vector words, all pointing into a run of `bsr` stubs that join one routine ending in a `halt` word (`0xffe00b1c` for the loader failure; `0xffe0083e` for stray vectors) |
| `0xffe00400`..`0xffe00b2b` | reset entry and loader |
| `0xffe01000` | pointer table: `0xffe10000`, `0xffe80000`, then all-ones |
| `0xffe10000`..`0xffe7ffff` | bank 1 program (0x70000 bytes); bank record at `0xffe7fd24` |
| `0xffe80000`..`0xffeeffff` | bank 2 program, same build; bank record at `0xffeefd24` |
| `0xffeefd68`..`0xffefffff` | unused in the 1 MB package |

A bank record is 48 bytes, twelve big-endian words that add to zero modulo 2^32: type/length `0x00040030`, bank start, bank size, used length, pointer to the build stamp string, pointer to the version string `1.44.0`, end pointer, a balancing word, an additive sum of the bank, a pointer to the supported-brick string, the flash class (1 here) and a pointer to the label `Base 1MB image` ([personalities.md](l1/personalities.md), table of the bank record). The loader verifies only the twelve-word sum.

**Flash device (established for the command stream; part unknown).** The flash is a 16-bit port on CS0. Commands are halfword writes at byte offsets `0xaaa` and `0x554` from the flash base: unlock `0xaa`/`0x55`, reset `0xf0`, program `0xa0`, sector erase `0x80` + unlock + `0x30` (with erase-suspend `0xb0` and resume `0x30` while it polls), autoselect `0x90`. Completion is read by comparing two successive halfword reads of data bit 6 (toggle) with bit 5 as the time-out flag. The sector map is the common top-boot layout: for the 1 MB part fifteen 64 KB sectors then 32 KB, 8 KB, 8 KB, 16 KB; for 2 MB thirty-one 64 KB sectors then the same four ([flash-nvram-and-image-records.md](l1/flash-nvram-and-image-records.md); the flash routines pair with package 1 as above). The autoselect result is stored for the caller and never compared in the image, so the manufacturer and device codes are unknown (U9).

**Loader order (established, `0xffe00400`, `0xffe00a80`..`0xffe00b26`).**

1. `RAMBAR` (control register `0xc04`) := `0x30100001`: the on-chip SRAM is enabled at `0x30100000`. Stack pointer := `0x30101800`; status register := `0x2700`.
2. Call the loader with `0x10000000` (the intended module base) as its argument. `MBAR` is not yet written, so the loader's read of the reset-status byte at `0x10000040` is an ordinary bus cycle. The flash byte at offset `0x40` is `0xff` (inferred as the value returned, if CS0 decodes that address at reset, see U3).
3. A 0x4d8-byte retained record at `0x30101990` is marked by the word `0xa1b2c3d4`. If the marker is absent the loader zeroes the record, writes the marker, version 1 and length `0x4d8`. The `MBAR` argument is stored at record offset 8.
4. If reset-status bit 5 (watchdog) is clear the loader clears the record's image-pointer word (offset `0x10`); if set it keeps a recorded image whose address lies in `0xffe00000`..`0xffefffff` and whose 48-byte record passes (`0xffe008fc`).
5. With no recorded image it walks the pointer table at `0xffe01000` backwards (`0xffe00994`): each entry must be aligned and in range and its 48-byte record must sum to zero; a set low bit forces the entry; otherwise the build stamps are compared as signed bytes and the greater wins, equal stamps keep the later table entry. Both banks carry the stamp `20060717181954`, so bank 2 (`0xffe80000`) is chosen unless a recorded image names bank 1.
6. The chosen bank's first 1 KB (from its second word) is copied to `0x30100000`, `VBR` := `0x30100000`, and the entry address (`0xffe10b02` or `0xffe80b02`) is called. If every candidate fails, or the entry returns, execution reaches the `halt` word at `0xffe00b1c`.

## 3. MCF5206e: SIM, memory map and chip selects

**Module base.** `MBAR` is written `0x10000001` by the program entry (`0xffe10b22`..`0xffe10b2e`): registers live at `0x10000000`..`0x100001ff`. Offsets below are from `MBAR` and agree with the Linux register header named in section 1. One routine reaches the registers by absolute address: the timer 1 interrupt stub (`0xffe10ff4`) writes `0xff` to `0x10000111` (the timer 1 event register); everything else goes through the base pointer stored at RAM `0x300002bc`.

**SIM and interrupt set-up (established, `0xffe54d10`, same bytes as package 1).**

| Register (offset) | Value written | Reading |
|---|---|---|
| `SIMR` (`+0x03`) | `0xc0` | as written; field meaning from the manual (hint) |
| `IMR` (`+0x36`, word) | `0xfffe` | every source masked; drivers unmask their own bit |
| `ICR1`..`ICR7` (`+0x14`..`+0x1a`) | `84 88 80 90 80 98 9c` | external levels 1 to 7, autovector bit set; level field `1 2 0 4 0 6 7` |
| `ICR8` (`+0x1b`, watchdog) | `0x1d` | level 7 priority 1, not autovectored; no handler installed |
| `ICR9` (`+0x1c`, timer 1) | `0x94` | autovector, level 5 |
| `ICR10` (`+0x1d`, timer 2) | `0x80` | autovector, level 0: never interrupts |
| `ICR11` (`+0x1e`, M-Bus) | `0x99` | autovector, level 6 priority 1 |
| `ICR12` (`+0x1f`, UART 1) | `0x1a` | vectored, level 6 priority 2 (vector from `UIVR`, `0x42`) |
| `ICR13` (`+0x20`, UART 2) | `0x00` | not used |
| `ICR14`, `ICR15` (`+0x21`, `+0x22`, DMA) | `0x09`, `0x0a` | level 2; no DMA register access was found in the package 1 reading (not rechecked in package 0) |
| `PAR` (`+0xca`, word) | `0x0071` | pin assignment; bit 7 is pulsed by the timer routine (section 4) |
| byte `+0x07` | cleared | not a register in the Linux header; harmless (U2) |

The ICR bit layout (AVEC bit 7, level bits 4:2, priority bits 1:0) is the manual's, not read in this pass (inferred). Levels 1 and 7 have vector table entries but no handler install was found (U7).

**Chip selects (established, `0xffe54d88`, plus `0xffe54f90` and `0xffe55548` for the late selects).** Each select is a 16-bit address register, a 32-bit mask register and a 16-bit control register at `CSARn` `+0x64+0xc*n`, `CSMRn` `+0x68+0xc*n`, `CSCRn` `+0x6e+0xc*n`.

| Select | When written | CSAR | CSMR | CSCR | Device and port |
|---|---|---|---|---|---|
| CS0 | board set-up | `0xffe0` | `0x001f0000`, rewritten `0x000f0000` when the first 1024 words at `0xffe00000` equal the 1024 words at `0xfff00000` | `0x1183` | boot flash, 16-bit |
| CS1 | board set-up | `0x2000` | `0` | `0x1143` | NVRAM and clock, 8-bit |
| CS2 | board set-up | `0x3000` | `0x00070000`, rewritten `0x00010000` when the alias probe fails | `0x0983` | external RAM, 16-bit |
| CS3 | USB start (`0xffe55548`), count > 0 only | `0x4000` | `0` | `0x0953` | USB device controller slot 0 |
| CS4 | UART set-up (`0xffe54f90`), all letters but G | `0x5000` | `0` | `0x095f` | quad UART |
| CS5 | letter `6` (second quad UART) or USB count 2 only | `0x6000` | `0` | `0x095f` or `0x0953` | not used on C, I, R |
| CS6, CS7 and the DRAM controller | board set-up | cleared | cleared | cleared | `DMCR` (`+0xc6`) cleared too; no DRAM, no default-memory responder |

Size probes (established): flash is 1 MB when the two 1 KB-word windows are equal; RAM is 512 KB when, after saving `0x3001fffc`, clearing it and writing all ones at `0x3007fffc`, the cleared word still reads zero, otherwise the mask is cut to 128 KB; the size and end are kept at `0x300002ec`/`0x300002f0`. The control-register field decode (port size, wait states, internal acknowledge) is the earlier notes' reading and rests on the manual: inferred. USB count by brick (established, descriptor word `+0x10`): C 1, R 1, I 0; so CS3 is programmed on the C-brick and R-brick and never on the I-brick.

**Address map the firmware relies on.**

| Range | Device | Width | Notes |
|---|---|---|---|
| `0x10000000`..`0x100001ff` | MCF5206e module registers | 8/16/32 | section 3 |
| `0x20000000`..`0x200007ff` | NVRAM and clock, 2 KB in a 64 KB window | 8 | section 7 |
| `0x30000000`..`0x30080000` | external RAM | 16 | 512 KB fitted per the probe; 128 KB if it aliases |
| `0x30100000`..`0x30101fff` | on-chip SRAM | 32 | vectors `0x30100000` (1 KB), stack top `0x30101800`, retained record `0x30101990`..`0x30101e68` |
| `0x40000000`..`0x4000001f` | USB device controller (C, R) | 8 | revision register at offset `0x18` |
| `0x50000000`..`0x5000001f` | quad UART | 8 | four channels at stride 8 |
| `0xffe00000`..`0xffefffff` | boot flash | 16 | 1 MB |

The program's own address validator accepts only `0xffexxxxx`, `0x200xxxxx`, `0x300xxxxx` and `0x301xxxxx` (package 1 `0xffe56388`).

**Clock (inferred; the image never reads or states it).** Four constants agree on 48 MHz: the on-chip UART divisor constant (48,000,000), timer 1 (section 4), the M-Bus divider `0x13` (divide by 480, 100 kHz) and timer 2. The fitted oscillator is unknown (U1).

## 4. Timers, watchdog, reset status

**Timers (established, routine copied to RAM `0x30000234`, flash `0xffe7f348`, called last in board set-up).** In order: byte `MBAR+0x1b8` := 1; `PAR` bit 7 set; timer 1 mode `+0x100` := 0, reference `+0x104` := `0x752f`, mode := `0x1d`; `IMR` bit 9 cleared (timer 1 interrupt unmasked); timer 2 mode `+0x120` := 0, reference `+0x124` := 1, mode := `0x2b`; ten dummy reads of `PAR`; `PAR` bit 7 cleared; byte `MBAR+0x1bc` := 1; a 1000-read delay of `PAR`. Timer 1 mode `0x1d` reads (manual layout, hint) as prescale 1, clock divided by 16, restart on reference, interrupt on reference, enabled: a 30,000-count period, 100 Hz at the inferred 48 MHz. Timer 2 mode `0x2b` is toggle output on reference 1 of the undivided clock: 12 MHz at 48 MHz, equal to the quad UART's assumed clock (that the board wires it there is inferred, U1). The two byte writes at `+0x1b8` and `+0x1bc` hit offsets that are not in the Linux register header (U2).

**Watchdog (established, arm `0xffe56684`, called from the main task at `0xffe1076a`, after the banner).** If the RAM flag at `0x300002e0` (initial value 1) is nonzero it is cleared and `SYPCR` (`+0x41`) := `0xfe`, `SWIVR` (`+0x42`) := 0, then the service pair `0x55`, `0xaa` is written to `SWSR` (`+0x43`). The routine is the only writer of `SYPCR`, which is write-once. Servicing is the same pair, written by a loop every 20 kernel ticks (`0xffe566bc`) and by several task loops (`0xffe107ac`, `0xffe107b4`). Nothing services or arms the watchdog before the banner. The code treats timeout as a reset, not an interrupt (`ICR8` has no handler).

**Reset cause (established, `0xffe56964`, called in board set-up).** It creates the retained record if the marker is absent, copies the word at `0x30101800` into record offset `0x24`, reads `RSR` (`+0x40`), stores "bit 5 set" at record offset `0x1c`, and writes the byte back to clear it. A restart request (`reboot_l1`, request `0x4002`) leaves a reason word at `0x30101800`, arms the watchdog if needed and spins with interrupts masked until the watchdog resets the chip ([board-boot-contract.md](l1/board-boot-contract.md), watchdog section; package 0 routine `0xffe56a04`).

## 5. Parallel port: the scan-chain interface

Established (package 1 `0xffe71d90`, `0xffe72254`; package 0 pairs at `0xffe71ee0`). The data-direction byte (`+0x1c5`) is cleared at board set-up. The scan set-up (`0xffe71ee0`, run after the UART set-up) clears `PAR` bits 5:4, writes the data byte (`+0x1c9`) `0x31`, the direction byte `0xfe` (bit 0 input, bits 1 to 7 outputs), then the data byte `0x38`, `0x28`, `0x38`, `0x30`; the scan code reads and writes the data byte afterwards. Pin roles from the firmware's own pin-print routine: bit 0 TDO (input), bit 3 PSI, bit 4 TRST, bit 5 TMS, bit 6 TCK, bit 7 TDI. The board-specific reset hook wrappers can run a pin pulse sequence (`0xffe72214` in package 0) gated by a RAM word and by bit 7 of the board byte at record offset `+0x41`; what sets either on the C-, I- or R-brick is unknown (U8).

## 6. Serial ports: on-chip UARTs and the quad UART

**On-chip UARTs.** The MCF5206e has two; the Linux header calls them UART0/UART1 at `+0x140`/`+0x180` and the manual UART 1/UART 2. The driver's port 1 selects the block at `MBAR+0x140`, port 2 the block at `+0x180` (`0xffe62de8` in package 1). Only port 1 is ever opened (the open routine `0xffe63130` has two callers, `0xffe54fec` for letter G and `0xffe55534`; `+0x180` is never registered).

| Port | Register block | C | I | R | Use |
|---|---|---|---|---|---|
| on-chip UART at `+0x140` | opened at 19,200 baud (`0x4b00`), vector `0x42`, level 6 | opened | opened | opened | the power-bay link: framed 8N1 packets with start byte `0xa0`, end byte `0xa5` and `0xaa` byte stuffing, addressed by the L1's rack and bay numbers ([power-bay-link.md](l1/power-bay-link.md)). Established: the UART set-up opens software port 1 and the power-bay task uses it. Inferred: software port 1 is the on-chip block, from the register base the driver adds for port 1 |
| on-chip UART at `+0x180` | never opened | - | - | - | unused |

The table of letters that leave this port closed is `0 4 7 8 9 A E Q V` (jump table at `0xffe554be`); C, I and R are not in it. The power-bay task itself is started for C, I and R (main task jump table at `0xffe1065e`, arm B).

**Quad UART at CS4 (`0x50000000`, established for the register use, part unknown).** Four channels at byte offsets 0, 8, 0x10, 0x18, each a 16550-style register set (`+0` data/divisor low, `+1` interrupt enable/divisor high, `+2` interrupt id/FIFO control, `+3` line control, `+4` modem control, `+5` line status, `+6` modem status, `+7` scratch). The probe clears line control, writes `0xa0 | channel` to each scratch register and reads all four back; a mismatch abandons the device. The enhanced register set (line control `0xbf`, then `+2` as the enhanced feature register `0x10`/`0xd0`/`0x1a`, `+4`/`+6` as Xon/Xoff characters `0x11`/`0x13`) is the 16C654 family's; the part marking is unknown. The divisor is `clock / (16 * baud)` rounded with a 12,000,000 Hz clock (so 107,142 baud is divisor 7, 57,600 is 13, 38,400 is 20), and the interrupt line is external level 4 (vector 28, handler `0xffe63d44` in package 1 numbering) ([quad-uart-and-serial-links.md](l1/quad-uart-and-serial-links.md)).

Port assignment by brick, decoded in package 0 from five jump tables indexed by the type character (`0xffe5506c`, `0xffe55158`, `0xffe5524e`, `0xffe553d6`, `0xffe554be`; the first three are indexed from `#` (`0x23`), the last two from `0` (`0x30`)):

| Software port | Quad channel | Baud (nominal/actual) | Flow | Client | C | I | R |
|---|---|---|---|---|---|---|---|
| 3 | 0 | 107,142 / 107,143 | software, `0x11`/`0x13` | RS-422 peer link | yes | yes | no |
| 4 | 1 | 107,142 / 107,143 | software | RS-422 peer link | yes | yes | no |
| 5 | 2 | 38,400 / 37,500 | none | L1 console | yes | yes | yes |
| 6 | 3 | 57,600 / 57,692 | automatic RTS/CTS | link to the node processor | yes | no | yes |

Table membership read entry by entry (established): quad UART initialised for `# % & 0 1 2 4 7 8 9 = ? A C E I N P Q R T V X Y` (not G); console on port 5 for `# % 0 1 2 4 7 8 9 = ? A C E I P Q R T V X Y` (not `&` or N); peers on ports 3 and 4 for `# % 0 1 2 4 7 8 = ? C E I P Q T V X Y` (not R, 9, A); node link on port 6 for `0 1 2 4 7 8 9 A C E Q R V`. The peer-link routing is in [rs422-router.md](l1/rs422-router.md): a C-brick maps router slot 1 to device 3 and slot 9 to device 4, an I-brick slot 9 to device 3 and slot 10 to device 4, and an R-brick has no peer slots ([power-reset-ibrick-rbrick.md](l1/power-reset-ibrick-rbrick.md)). Which cable each port reaches is not visible to the image (U10). The console port and the peer ports are UART channels of the quad device; the node link is the fourth channel.

**USB device controller (C, R; `0x40000000`).** Probed by `0xffe6dcbc` (pairs with package 1 `0xffe6db6c`, same bytes): scratch register `+0x1b` := `0xb5`, revision `+0x18` read twice; revision `0x81` leaves the USB side inert, other values take the register-driven path ([usb-controller-registers.md](l1/usb-controller-registers.md)). The interrupt is external level 2 (`ICR2`), passed to the driver at `0xffe5559a`. This is the link to an L2 on these bricks; the I-brick has none.

## 7. NVRAM, log ring and clock (CS1)

**Layout (established, start-up `0xffe5b184`/`0xffe5b0d4`/`0xffe5abb4`, checks `0xffe5bdd0`, `0xffe5bdf8`).** 2 KB at `0x20000000`.

| Offset | Content |
|---|---|
| `0x000`..`0x007` | header: low nibble of byte 0 is the header version (1 when created), byte 1 the record-area offset in 8-byte units (1), byte 7 makes the 8-byte byte-sum zero. A header failing the sum, with version nibble 0 or offset 0, is rebuilt (`0xffe5b0d4`) |
| `0x008`..`0x077` | settings record, 0x70 bytes, last byte (`+0x6f`) the checksum; first-word bits 27:24 are the record version, stepped by a jump table from 0 to `0xa` |
| `0x078`..`0x0e7` | mirror of the settings record |
| `0x0e8`..`0x10b` | 0x24-byte log-control record: version byte (1 when created) at `+0xe0` of the record area, then four pointers at record-area offsets `+0xe4` start, `+0xe8` end, `+0xec` tail, `+0xf0` head, checksum at `+0x103` |
| `0x10c`..`0x12f` | mirror of the log-control record |
| `0x130`..`0x1e3` | 0xb4-byte record, version byte 3 when created at record-area `+0x128`, checksum at `+0x1db` |
| `0x1e4`..`0x297` | mirror of that record |
| `0x298`..`0x7f7` | log ring: start pointer `0x20000298`, end pointer `0x200007f8`, 0x560 bytes |
| `0x7f8`..`0x7ff` | clock registers |

Offsets are device offsets; the "record area" starts at `0x20000008`. Checksums: each record's last byte is the two's complement of the byte sum of the preceding bytes (so the whole record sums to zero modulo 256); the header uses the same rule over 8 bytes. Start-up rule: a record failing its sum is replaced by its mirror and the mirror copied over it; with neither valid the record is zeroed, given its current version and a fresh checksum. If the log pointers are inconsistent (outside the ring or not equal to the ring bounds) they are reset to the ring bounds and the control record's checksum and mirror are rewritten (`0xffe5b836`..`0xffe5b8b2`, `0xffe5a8c0`). Settings number to byte mapping is in [settings-record.md](l1/settings-record.md) (read in package 1; the NVRAM routines pair as above). The contents of the 0xb4-byte record were not decoded (U13).

**Log writes (established).** Every log line is written at the head pointer in the ring; after each line the control record is re-summed and mirrored (`0xffe5aae4`..`0xffe5ab60`). Each line is timestamped, which reads the clock first (below). The ring therefore holds the banner before any serial port is open.

**Clock registers (established layout; part family inferred).** `0x200007f8` control: bit 6 is a read latch (set before a read, cleared after: `0xffe5e68a`, `0xffe5e776`), bit 7 write enable, low six bits the two century digits (tens in bits 5:4, hundreds in bits 3:0); then BCD bytes at `0x7f9` seconds, `0x7fa` minutes, `0x7fb` hours, `0x7fc` day of week, `0x7fd` date, `0x7fe` month (bit 4 tens), `0x7ff` year. The read assembles `thousands*1000 + hundreds*100 + year`; the write adds 1900 to a broken-down year. The map is that of the common 2 KB battery-backed RAM with calendar at the top of its window; the image never names the part (U12).

## 8. The M-Bus and its devices

**Controller (established, `0xffe6cbf8`, `0xffe6d0fc`; package 0 pairs with package 1 `0xffe6caa8`, `0xffe6cfac`).** On-chip M-Bus module at `MADR` `+0x1e0`, `MFDR` `+0x1e4`, `MBCR` `+0x1e8`, `MBSR` `+0x1ec`, `MBDR` `+0x1f0` (offsets from the manual's chapter 13 as the earlier notes cite them). Bring-up: `MBCR` := 0, `MFDR` := `0x13`, `MADR` := 0, `MBCR` := `0x80`; if `MBSR` bit 5 (bus busy) is set it writes `MBCR` := `0xa0`, reads `MBDR`, clears `MBSR` and `MBCR`, and retries up to 1000 times, logging the last two status values if the bus never frees. A transaction writes `MBCR` := `0xf0` and the address byte, continues in the level-6 interrupt (`ICR11`), uses `0xf4` for a repeated start, `0xe0` for receive, `0xe8` before the last byte and `0x80` to stop; there is no retry above the controller ([mbus.md](l1/mbus.md)). Status accepted when `(MBSR & 0xb2) == 0xa2`; otherwise -406, -401, -402 or -403, and -404 for no acknowledge. The M-Bus is started by the board identity routine, which calls `0xffe6cc98(6)` first (after the banner, section 11).

**Expander access (established, shared code at `0xffe65974`, `0xffe65bf4`).** Every expander is a single-byte device: a write sends the 8-bit wire address then one byte (the shadow with the masked bits replaced), a read sends the wire address with bit 0 set and returns one byte. The firmware sets every input bit to one in the byte it writes, as a quasi-bidirectional port needs (the Linux `pcf857x` driver describes the same convention; the part is not named in the image, so this is inferred). Each descriptor is 16 bytes: byte 0 access form (0 here), byte 1 bus segment, byte 2 the 8-bit wire address (`0xff` marks absent), byte 4 input-bit mask, byte 6 the value applied at cold start, byte 7 input bits whose changes are reported, +8 a change hook, +12 the name. "Wire" below is the 8-bit form and its 7-bit form.

**Bus segments (established).** Segment 0 is "no select". The C-brick registers a select hook (`0xffe452d0`, registered through `0xffe6c9fc`) that writes expander 0 (`BUS SELECT`, 7-bit `0x27`) with a value per segment, skipping the write when the shadow already holds it: segment 1 `0xff` (the shadow is cleared first so the write always happens), 2 `0x78`, 3 `0xb8`, 4 `0xd8`, 5 `0xe8`, 6 to 13 `0xf0` to `0xf7`; segment above 13 logs an error. The I-brick and R-brick register no select hook and use segment 0 only. The select device behind the values is not named (U11).

### 8.1 C-brick (letter `C`; descriptor initialiser `0xffe45504`, registration `0xffe453f8`)

The registration routine installs, in order: the select hook, 17 expanders (`0xffe455dc`), 11 identity EEPROM records (`0xffe456ec`), 7 monitor chips (`0xffe45770`), 20 supply records (`0xffe457c4`), one fan-control record (`0xffe459a4`), 3 tachometers (`0xffe459b0`), 4 temperature records (`0xffe459d4`), 8 PSC status rows (`0xffe45a24`), 2 port records (`0xffe459f4`), the display on expander 16, then the board's own initialisation (`0xffe4482c`, not followed, U18), the cold-start check `0xffe4a83c`, a second board routine (`0xffe44ae0`) and the two interrupt-polling workers. Established from the table bytes; the array start addresses correct the arrays that [ip45-io-expanders.md](l1/ip45-io-expanders.md) lists for package 0, which cut some of them at the wrong entry.

Expanders:

| Idx | Seg | Wire (8-bit / 7-bit) | Input bits | Applied | Int bits | Hook | Name |
|---:|---:|---|---|---|---|---|---|
| 0 | 0 | `0x4e` / `0x27` | `0x00` | `0xff` | `0x00` | - | BUS SELECT |
| 1 | 2 | `0x42` / `0x21` | `0xff` | `0xff` | `0x00` | - | LED0 LOW |
| 2 | 2 | `0x44` / `0x22` | `0xff` | `0xff` | `0x00` | - | LED0 HIGH |
| 3 | 2 | `0x46` / `0x23` | `0xff` | `0xff` | `0x00` | - | LED1 LOW |
| 4 | 2 | `0x48` / `0x24` | `0xff` | `0xff` | `0x00` | - | LED1 HIGH |
| 5 | 2 | `0x4c` / `0x26` | `0xf1` | `0xff` | `0x00` | yes | COM CONTROL |
| 6 | 3 | `0x42` / `0x21` | `0x00` | `0x72` | `0x00` | - | BEDROCK CNTL 1 |
| 7 | 3 | `0x44` / `0x22` | `0x00` | `0x8a` | `0x00` | - | BEDROCK CNTL 2 |
| 8 | 3 | `0x46` / `0x23` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 9 | 3 | `0x48` / `0x24` | `0xde` | `0xff` | `0xde` | yes | POWER CONTROL |
| 10 | 3 | `0x4a` / `0x25` | `0xff` | `0xff` | `0x00` | - | PIMM PSC |
| 11 | 3 | `0x4c` / `0x26` | `0x40` | `0xff` | `0x40` | yes | FAN CONTROL |
| 12 | 4 | `0x44` / `0x22` | `0x36` | `0xff` | `0x36` | yes | PIMM0 POWER |
| 13 | 4 | `0x42` / `0x21` | `0x3c` | `0xff` | `0x00` | yes | PIMM0 VCC OK |
| 14 | 5 | `0x44` / `0x22` | `0x36` | `0xff` | `0x36` | yes | PIMM1 POWER |
| 15 | 5 | `0x42` / `0x21` | `0x3c` | `0xff` | `0x00` | yes | PIMM1 VCC OK |
| 16 | 0 | `0x40` / `0x20` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

The names are the firmware's own log labels. Input-bit mask bits are never driven by the firmware; the applied value is written (ORed with the input bits) once at cold start when the cold-start check passes (section 11).

Identity EEPROMs (kind 1 board record, kind 2 memory serial-presence record; capacity is the configured length, not a probed size):

| Idx | Seg | Wire (8-bit / 7-bit) | Capacity | Kind | Name |
|---:|---:|---|---|---:|---|
| 0 | 0 | `0xa4` / `0x52` | 512 | 1 | NODE |
| 1 | 4 | `0xa8` / `0x54` | 512 | 1 | PIMM 0 |
| 2 | 5 | `0xa8` / `0x54` | 512 | 1 | PIMM 1 |
| 3 | 6 | `0xa0` / `0x50` | 256 | 2 | DIMM 0 |
| 4 | 7 | `0xa0` / `0x50` | 256 | 2 | DIMM 2 |
| 5 | 8 | `0xa0` / `0x50` | 256 | 2 | DIMM 4 |
| 6 | 9 | `0xa0` / `0x50` | 256 | 2 | DIMM 6 |
| 7 | 10 | `0xa0` / `0x50` | 256 | 2 | DIMM 1 |
| 8 | 11 | `0xa0` / `0x50` | 256 | 2 | DIMM 3 |
| 9 | 12 | `0xa0` / `0x50` | 256 | 2 | DIMM 5 |
| 10 | 13 | `0xa0` / `0x50` | 256 | 2 | DIMM 7 |

DIMM numbering follows the table order, so segments 6 to 13 carry DIMM 0, 2, 4, 6, 1, 3, 5, 7. Board record validation (common header sum, area sums, SPD byte-63 sum) is in [board-records.md](l1/board-records.md) and [mbus.md](l1/mbus.md).

Monitor chips (twelve bytes each; type 1 with the register layout of the LM78 family, a hint from the registers written):

| Idx | Type | Seg | Wire (8-bit / 7-bit) | Flags | Halfword at +4 | Name |
|---:|---:|---:|---|---|---|---|
| 0 | 1 | 2 | `0x58` / `0x2c` | `0x00` | `0x00a0` | NODE 0 |
| 1 | 1 | 2 | `0x5a` / `0x2d` | `0x00` | `0x0020` | NODE 1 |
| 2 | 1 | 2 | `0x5c` / `0x2e` | `0x01` | `0x0000` | NODE 2 |
| 3 | 1 | 4 | `0x5c` / `0x2e` | `0x00` | `0x0000` | PIMM0 A |
| 4 | 1 | 4 | `0x5e` / `0x2f` | `0x00` | `0x0000` | PIMM0 B |
| 5 | 1 | 5 | `0x5c` / `0x2e` | `0x00` | `0x0000` | PIMM1 A |
| 6 | 1 | 5 | `0x5e` / `0x2f` | `0x00` | `0x0000` | PIMM1 B |

Initialisation of each monitor (established, package 1 `0xffe6836c`; the monitor code pairs): register `0x40` := `0x80`, `0x43` := `0xdf`, `0x44` := `0x13`, `0x47` := low byte of the halfword at +4 (`0xa0`, `0x20`, or 0), then `0x40` := `0x03` unless descriptor flag bit 0 is set (monitor 2 has it set). Reads: voltage channels 0 to 3, 8 and 9 map to registers `0x20`..`0x25` with fixed scale factors, temperature channel 4 to register `0x27` as a signed byte, tachometer channels 6 and 7 to fan registers (not followed). The registers agree with the LM78/DS1780 map; the part is not named (U6).

Supplies (24-byte records; "nominal entry" indexes the 32-entry name and nominal-voltage table at `0xffe4e6e8`, entries 0 to 29 supplies, 30 `ALL`, 31 `NONE`; "enable" is expander index and mask, active low except supply type `0x17`, which no C-brick record uses):

| Idx | Nominal entry | Enable (exp idx, mask) | Monitor, channel | Name |
|---:|---|---|---|---|
| 0 | 8 48V (48) | 11, `0x80` | none | - |
| 1 | 20 VCPU (0) | 12, `0x01` | 3, 1 | PIMM0 VCPU |
| 2 | 20 VCPU (0) | 14, `0x01` | 5, 1 | PIMM1 VCPU |
| 3 | 15 2.5V (2.5) | 9, `0x20` | 1, 0 | DIMM |
| 4 | 15 2.5V (2.5) | 9, `0x01` | 0, 0 | - |
| 5 | 12 1.5V (1.5) | none | 0, 1 | - |
| 6 | 16 3.3V (3.3) | none | 0, 2 | 3.3V AUX |
| 7 | 16 3.3V (3.3) | none | 1, 2 | 3.3V L1 |
| 8 | 17 5V (5) | none | 0, 3 | 5V AUX |
| 9 | 9 12V (12) | none | 0, 8 | 12 BIAS |
| 10 | 21 SRAM (2.5) | 12, `0x08` | 3, 0 | PIMM0 SRAM |
| 11 | 12 1.5V (1.5) | none | 3, 9 | PIMM0 1.5V |
| 12 | 16 3.3V (3.3) | none | 3, 2 | PIMM0 3.3V AUX |
| 13 | 17 5V (5) | none | 3, 3 | PIMM0 5V AUX |
| 14 | 9 12V (12) | none | 3, 8 | PIMM0 12V BIAS |
| 15 | 21 SRAM (2.5) | 14, `0x08` | 5, 0 | PIMM1 SRAM |
| 16 | 12 1.5V (1.5) | none | 5, 9 | PIMM1 1.5V |
| 17 | 16 3.3V (3.3) | none | 5, 2 | PIMM1 3.3V AUX |
| 18 | 17 5V (5) | none | 5, 3 | PIMM1 5V AUX |
| 19 | 9 12V (12) | none | 5, 8 | PIMM1 12V BIAS |

Tachometers and temperatures:

| Idx | Monitor, channel | Name |
|---:|---|---|
| 0 | 0, 6 | LEFT |
| 1 | 0, 7 | CENTER |
| 2 | 1, 6 | RIGHT |

| Idx | Monitor, channel, offset |
|---:|---|
| 0 | 3, 4, 0 |
| 1 | 4, 4, 0 |
| 2 | 5, 4, 0 |
| 3 | 6, 4, 0 |

The temperature records name monitors 3 to 6 (the two PIMM sensors each); the tachometers sit on monitors 0 and 1. The 10-byte fan-control record is `01 0b 20 0b 00 1f 03 09 15 1f` at `0xffe459a4`; its fields were not decoded (U14).

### 8.2 I-brick (letter `I`; shared initialiser `0xffe4676c` for `& I N P X Y`, registration `0xffe46028`, `I` branch at `0xffe4606a`)

The initialiser sets USB count 0, no soft-reset hook, no NMI hook, no power-down hook, reset hold `0xffe47ddc`, reset release `0xffe48370`. The registration installs 13 expanders (`0xffe467e8`), 2 EEPROMs (`0xffe468b8`), 3 monitors (`0xffe468d0`), 9 supplies (`0xffe468f4`), fan control (`0xffe469cc`), 3 tachometers (`0xffe469d8`), 2 temperatures (`0xffe469fc`), 7 PSC status rows (`0xffe46a7a`), 2 port records (`0xffe46a24`), the display on expander 12, and a one-entry reset table (`0xffe46a0c`, registered by `0xffe47ed8`). Everything is on segment 0.

| Idx | Seg | Wire (8-bit / 7-bit) | Input bits | Applied | Int bits | Hook | Name |
|---:|---:|---|---|---|---|---|---|
| 0 | 0 | `0x70` / `0x38` | `0x00` | `0xff` | `0x00` | - | POWER ENABLE |
| 1 | 0 | `0x72` / `0x39` | `0x20` | `0xff` | `0x00` | - | FAN SPEED |
| 2 | 0 | `0x74` / `0x3a` | `0xff` | `0xff` | `0x0f` | yes | POWER STATUS 1 |
| 3 | 0 | `0x76` / `0x3b` | `0xff` | `0xff` | `0xff` | yes | POWER STATUS 2 |
| 4 | 0 | `0x46` / `0x23` | `0x55` | `0xdd` | `0x11` | yes | PORT CONTROL |
| 5 | 0 | `0x48` / `0x24` | `0x00` | `0xd4` | `0x00` | - | XBRIDGE CONTROL |
| 6 | 0 | `0x4a` / `0x25` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT1 |
| 7 | 0 | `0x4c` / `0x26` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT2 |
| 8 | 0 | `0x4e` / `0x27` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT3 |
| 9 | 0 | `0x42` / `0x21` | `0x7f` | `0x7f` | `0x0b` | yes | PCI BUS1 SLOT4 |
| 10 | 0 | `0x7c` / `0x3e` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS2 SLOT1 |
| 11 | 0 | `0x7e` / `0x3f` | `0x0b` | `0x7f` | `0x0b` | yes | PCI BUS2 SLOT2 |
| 12 | 0 | `0x40` / `0x20` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

| Idx | Seg | Wire (8-bit / 7-bit) | Capacity | Kind | Name |
|---:|---:|---|---|---:|---|
| 0 | 0 | `0xa4` / `0x52` | 512 | 1 | POWER |
| 1 | 0 | `0xa8` / `0x54` | 512 | 1 | LOGIC |

| Idx | Type | Seg | Wire (8-bit / 7-bit) | Flags | Halfword at +4 | Name |
|---:|---:|---:|---|---|---|---|
| 0 | 1 | 0 | `0x58` / `0x2c` | `0x00` | `0x00a0` | POWER 0 |
| 1 | 1 | 0 | `0x5a` / `0x2d` | `0x00` | `0x0020` | POWER 1 |
| 2 | 1 | 0 | `0x5c` / `0x2e` | `0x01` | `0x0000` | POWER 2 |

| Idx | Nominal entry | Enable (exp idx, mask) | Monitor, channel | Name |
|---:|---|---|---|---|
| 0 | 8 48V (48) | 0, `0x01` | none | - |
| 1 | 9 12V (12) | 0, `0x04` | 1, 8 | 12V BIAS |
| 2 | 15 2.5V (2.5) | 0, `0x08` | 0, 0 | - |
| 3 | 16 3.3V (3.3) | 0, `0x10` | 0, 2 | - |
| 4 | 9 12V (12) | 0, `0x40` | 0, 8 | - |
| 5 | 17 5V (5) | 0, `0x20` | 0, 3 | - |
| 6 | 18 -12V (-12) | 0, `0x80` | 1, 9 | - |
| 7 | 16 3.3V (3.3) | none | 1, 2 | 3.3V AUX |
| 8 | 17 5V (5) | none | 1, 3 | 5V AUX |

| Idx | Monitor, channel | Name |
|---:|---|---|
| 0 | 0, 6 | LEFT |
| 1 | 0, 7 | CENTER |
| 2 | 1, 6 | RIGHT |

| Idx | Monitor, channel, offset |
|---:|---|
| 0 | 0, 4, 0 |
| 1 | 1, 4, 0 |

Expander 0 (`POWER ENABLE`, `0x70`/`0x38`) drives the seven switched supplies, active low: masks `0x01` (48 V), `0x04` (12 V bias), `0x08` (2.5 V), `0x10` (3.3 V), `0x40` (12 V), `0x20` (5 V), `0x80` (-12 V); bit `0x02` drives none. The 3.3 V and 5 V auxiliary supplies have no enable and are only monitored.

### 8.3 R-brick (letter `R`; initialiser `0xffe4a330`, registration `0xffe4a234`)

USB count 1; reset hold `0xffe49fe0`, reset release `0xffe4a1b4`; no soft-reset, no NMI, no power-up veto. The registration installs 8 expanders (`0xffe4a3bc`), 2 EEPROMs (`0xffe4a43c`), 1 monitor (`0xffe4a454`), 5 supplies (`0xffe4a460`), 2 PSC status rows (`0xffe4a4d8`), fan control (`0xffe4a4e4`), 2 tachometers (`0xffe4a4f0`), 1 temperature (`0xffe4a508`), 8 port records (`0xffe4a510`, named `A` to `H`), the display on expander 7, the sanity routine, one reset-table entry (`0xffe4a5d0`) and the two polling workers.

| Idx | Seg | Wire (8-bit / 7-bit) | Input bits | Applied | Int bits | Hook | Name |
|---:|---:|---|---|---|---|---|---|
| 0 | 0 | `0x72` / `0x39` | `0xe0` | `0xff` | `0xe0` | yes | POWER CONTROL |
| 1 | 0 | `0x44` / `0x22` | `0x00` | `0x00` | `0x00` | yes | WIDGET PRESENT |
| 2 | 0 | `0x46` / `0x23` | `0x00` | `0xf1` | `0x00` | yes | ROUTER CONTROL |
| 3 | 0 | `0x48` / `0x24` | `0x55` | `0xd5` | `0x11` | yes | PORT HA CONTROL |
| 4 | 0 | `0x4a` / `0x25` | `0x55` | `0xdd` | `0x11` | yes | PORT BC CONTROL |
| 5 | 0 | `0x4c` / `0x26` | `0x55` | `0xdd` | `0x11` | yes | PORT DE CONTROL |
| 6 | 0 | `0x4e` / `0x27` | `0x55` | `0x5d` | `0x11` | yes | PORT FG CONTROL |
| 7 | 0 | `0x40` / `0x20` | `0x0f` | `0xff` | `0x07` | yes | DISPLAY CONTROL |

| Idx | Seg | Wire (8-bit / 7-bit) | Capacity | Kind | Name |
|---:|---:|---|---|---:|---|
| 0 | 0 | `0xa4` / `0x52` | 512 | 1 | POWER |
| 1 | 0 | `0xa8` / `0x54` | 512 | 1 | LOGIC |

| Idx | Type | Seg | Wire (8-bit / 7-bit) | Flags | Halfword at +4 | Name |
|---:|---:|---:|---|---|---|---|
| 0 | 1 | 0 | `0x58` / `0x2c` | `0x00` | `0x00a0` | POWER |

| Idx | Nominal entry | Enable (exp idx, mask) | Monitor, channel | Name |
|---:|---|---|---|---|
| 0 | 8 48V (48) | 0, `0x01` | none | - |
| 1 | 9 12V (12) | none | 0, 8 | 12 BIAS |
| 2 | 15 2.5V (2.5) | 0, `0x04` | 0, 0 | - |
| 3 | 5 3.3V AUX (3.3) | none | 0, 2 | - |
| 4 | 6 5V AUX (5) | none | 0, 3 | - |

| Idx | Monitor, channel | Name |
|---:|---|---|
| 0 | 0, 6 | LEFT |
| 1 | 0, 7 | RIGHT |

| Idx | Monitor, channel, offset |
|---:|---|
| 0 | 0, 4, 0 |

Switched supplies on expander 0 (`POWER CONTROL`, `0x72`/`0x39`), active low: mask `0x01` (48 V) and `0x04` (2.5 V). Expander 2 carries the router-type nibble (section 9).

### 8.4 Records whose fields were not decoded

Established as table contents (bytes read), not as meaning. Field positions below follow the pattern across the three bricks and are inferred.

Port records (24 bytes each; byte 0 an expander index, byte 3 a one-bit mask, name pointer at +8): the expander index and mask agree with the `PORT CONTROL` expanders of each brick, so these name per-port bits on them (inferred).

| Brick | Name | Expander idx | Byte 1 | Byte 2 | Mask |
|---|---|---:|---:|---:|---|
| C | NI | 8 | 0 | 6 | `0x04` |
| C | II | 8 | 1 | 6 | `0x08` |
| I | 10 | 4 | 0 | 5 | `0x40` |
| I | 11 | 4 | 1 | 5 | `0x80` |
| R | A | 3 | 1 | 1 | `0x02` |
| R | B | 4 | 0 | 1 | `0x04` |
| R | C | 4 | 1 | 1 | `0x08` |
| R | D | 5 | 0 | 1 | `0x10` |
| R | E | 5 | 1 | 1 | `0x20` |
| R | F | 6 | 0 | 1 | `0x40` |
| R | G | 6 | 1 | 1 | `0x80` |
| R | H | 3 | 0 | 1 | `0x01` |

Raw bytes of the other undecoded tables:

| Table | Address | Rows x size | Bytes |
|---|---|---|---|
| C fan control | `0xffe459a4` | 1 x 10 | `01 0b 20 0b 00 1f 03 09 15 1f` |
| I fan control | `0xffe469cc` | 1 x 10 | `01 00 02 01 00 1f 03 09 15 1f` |
| R fan control | `0xffe4a4e4` | 1 x 10 | `01 00 02 ff 00 00 02 00 00 00` |
| C PSC status rows | `0xffe45a24` | 8 x 6 | `09 04 01 02 04 00 09 04 02 08 10 00 09 03 03 40 80 00 0c 0a 04 10 20 00 0c 01 05 02 04 00 0e 0f 06 10 20 00 0e 02 07 02 04 00 0b 00 00 00 40 00` |
| I PSC status rows | `0xffe46a7a` | 7 x 6 | `03 04 01 04 08 00 03 05 02 01 02 00 02 03 03 01 02 00 02 03 04 04 08 00 03 02 07 10 20 00 03 06 00 00 80 00 03 00 00 00 40 00` |
| R PSC status rows | `0xffe4a4d8` | 2 x 6 | `00 02 01 40 80 00 00 00 00 00 20 00` |

## 9. Lines that drive power, reset, NMI and the front panel

All are outputs of the expanders in section 8; the firmware names no downstream pin. Writes use the masked helpers `0xffe65e84(index, mask, value)`, set `0xffe65ef8(index, mask)` and clear `0xffe65f1c(index, mask)`; each sends the whole shadow byte once. Tick counts are kernel ticks (the 100 Hz timer at the inferred clock).

**C-brick (established, each hook read in package 0).**

| Line | Expander bit | Written by | Cold-start level |
|---|---|---|---|
| reset group, line a | `BEDROCK CNTL 1` (index 6) bit 0 | hold clears it last of four; release sets it third | low (`0x72` has it clear) |
| reset group, line b | `BEDROCK CNTL 1` bit 1 | hold clears it second; release sets it last | high |
| reset group, line c | `BEDROCK CNTL 2` (index 7) bit 0 | hold clears it third; release sets it second to last | low (`0x8a` has it clear) |
| soft reset | `BEDROCK CNTL 1` bit 7 | `softreset`: set, 10 ticks, clear (`0xffe445e8`); hold clears it first | low |
| NMI | `BEDROCK CNTL 1` bit 6 | `nmi`: clear then set with no delay (`0xffe4463c`) | high |
| strap bits | `BEDROCK CNTL 2` bits 3 to 7 | written once at release under mask `0xf8`; bit 6 is written 0 | `0x8a` |
| processor-module bits | `PIMM0 VCC OK`, `PIMM1 VCC OK` bits 7:6 | hold sets both, release clears both | not applied |

Reset hold (`0xffe44bd4`): compare the two processor-module type bytes (`0x30000090`, `0x30000091`) with settings `0x56`, `0x57` and reset the voltage margins on a mismatch; refuse with -900 if the flag byte at `0x30000098` is zero; clear the four lines in the order above; for each module run `0xffe44b48`. Release (`0xffe44dfc`): strap write, 10 ticks, clear strap bit 3, 10 ticks, set line a, 10 ticks, clear the module bits with 3- and 10-tick waits, set line c, set line b. Supply enables (active low): 48 V on `FAN CONTROL` (index 11) mask `0x80`; each PIMM's core supply on `PIMM0 POWER`/`PIMM1 POWER` (indexes 12, 14) mask `0x01` and its SRAM supply mask `0x08`; two 2.5 V rails on `POWER CONTROL` (index 9) masks `0x20` and `0x01`. The power-sanity and power-up sequences are in [power-reset-cbrick.md](l1/power-reset-cbrick.md) and [power-reset-ibrick-rbrick.md](l1/power-reset-ibrick-rbrick.md).

**I-brick (established).** `POWER ENABLE` as in 8.2. Reset lines: `XBRIDGE CONTROL` (index 5, 7-bit `0x24`) bits 0, 1 and 3 form the reset group, driven low by hold and raised by release (bits 1 and 3 together, then bit 0 ten ticks later); bits 2 and 4 are the enables for the two host ports (set by release from module records 1 and 2 and settings `0x2a`, `0x2b`); bit 5 selects between two clocks (`A`/`B`); bits 6 and 7 are not written. Cold-start value `0xd4`. No soft reset, no NMI line.

**R-brick (established).** `POWER CONTROL` bit 3 (`0x08`) is cleared by hold and set by release. `ROUTER CONTROL` (index 2, 7-bit `0x23`): hold writes bits `0x0d` := `0x01`; release writes bits `0xf0` from persistent setting `0x3c` (a value outside the three accepted ones is replaced by `0xf` and stored) and then, if the supply-state routine is nonzero, steps bits `0x0d` through `0x09`, `0x0d`, `0x0c` with 10 ticks between. Cold-start value `0xf1`. No NMI, no soft reset.

**Front panel (established for the bits; polarity inferred; carried from the package 1 reading, whose routines pair with package 0: the three LED routines at `0xffe51778`, `0xffe51830`, `0xffe518e4` and the change hook `0xffe51b3c`).** `DISPLAY CONTROL` (7-bit `0x20`, segment 0, input mask `0x0f`, change-reported bits `0x07`) is expander 16 on the C-brick, 12 on the I-brick, 7 on the R-brick. LED bits are the output bits `0x40`, `0x10`, `0x20`; a clear bit is the lit state (inferred from the state query). The change hook tests bits 0 and 1 of the changed value; bit 0 is the power button (inferred: it calls the power-up or power-down routine) and may be refused by setting 119. The text display driver is behind the same expander and was not decoded (U16). The C-brick also has four `LED0/LED1 LOW/HIGH` expanders on segment 2 (7-bit `0x21`..`0x24`, all bits input-masked, applied `0xff`); their use is by name only (U16).

**Power button, power state and the automatic power-up.** The task that sends `* power up` after start-up is created for `C 3 5 4 A 6 9 7 8 0 Q @ O B F H L` only; I and R come up with supplies off and bridges or router held in reset until a `power up` request arrives from their own console, an L2 over USB (R) or a C-brick over the peer link (I) ([power-reset-ibrick-rbrick.md](l1/power-reset-ibrick-rbrick.md)).

## 10. Persistent settings that the boot path reads

Settings 1 and 2 (rack, bay) from the first NVRAM record form the L1's own 16-bit address, `((setting 1 & 0x3ff) << 6) | (setting 2 & 0x3f)`, stored by `0xffe78464` at `0xffe10602`; setting 9 holds the brick-type character written by the identity routine; setting 16 the sub-type. Settings 0x56 and 0x57 (C-brick processor-module types), `0x2a`, `0x2b`, `0x28`, `0x29` (strap and port enables), `0x3c` (router type) and `0x50` (skip the serial-number check) are read by the hooks above. The full mapping is [settings-record.md](l1/settings-record.md).

## 11. Boot order and every access before the banner

"Banner" means the first log line the main task writes, `L1 booting 1.44.0` (format string `L1 booting %s\n` at `0xffe10a99`, version string at `0xffe10aa8`), formatted at `0xffe10592`. It goes to the log ring in NVRAM, not to a serial port: no serial port is open at that point. The front-panel text `L1 booting` is written later by the display driver (`0xffe5232e`). Everything listed under "before" is the complete set of device accesses on the path from reset to that call, in order; the later steps are listed so the order of the probes is on record.

**Before the banner (established).**

1. Reset: instruction fetch of the vector words at `0xffe00000`; `RAMBAR` write; loader reads (section 2): the retained record at `0x30101990` (read, possibly cleared), byte `0x10000040` (external cycle, MBAR not yet valid), the pointer table at `0xffe01000`, the 48-byte records of both banks, the bank bytes for the sums, the build-stamp strings; copy of 1 KB from flash to `0x30100000`; `VBR` write.
2. Program entry `0xffe10b02`: `RAMBAR` again; `MBAR` := `0x10000001`; call `0xffe54ebc(0x10000000)`.
3. Board set-up: the SIM, interrupt and `PAR` writes (section 3); the DRAM registers cleared; CS0, CS1, CS2 writes with the flash (1024 words at `0xffe00000` and `0xfff00000`) and RAM probes (`0x3001fffc` read, cleared, restored; `0x3007fffc` written all ones); CS3 to CS7 and `DMCR` cleared; `PADDR` (`+0x1c5`) cleared; reset-cause read and clear (`RSR`, `0x30101800`); 1 KB of vectors copied from `0xffe10004` to `0x30100000`; RAM `0x30000c10`..`0x300060e8` zeroed; `0xc10` bytes of initialised data copied from `0xffe7f114` to `0x30000000`; the timer routine run from RAM (section 4).
4. `VBR` := `0x30100000`, cache invalidate and enable (`CACR` `0x01000000`, then `0x80000000`), kernel start `0xffe4264c`. The kernel start path (`0xffe41300`..`0xffe44000`) contains no peripheral address and no `movec`; it creates the first task (`0xffe1040c`), which calls `0xffe56654` (a kernel object, no device) and the task create routine.
5. Main task `0xffe10570`: `0xffe5b184`, the NVRAM start-up: reads the 8 header bytes at `0x20000000` and checks the sum; if bad, rewrites the header (`0xffe5b0d4`); reads the record version nibble and dispatches; checks the settings record and its mirror and repairs from the mirror; checks the log-control and 0xb4-byte records the same way; validates the log pointers; every repair writes NVRAM and recomputes sums and mirrors.
6. The banner call `0xffe5b904(format, "1.44.0")` formats the line and calls the log writer `0xffe5a92c`, which first reads the clock (`0xffe5e674`: set read latch `0x200007f8` bit 6, read `0x7f8`, `0x7ff`, `0x7fe`, `0x7fd`, `0x7fb`, `0x7fa`, `0x7f9`, clear the latch) and then stores the text and a newline into the ring and rewrites the control record, its checksum and its mirror.

No serial, M-Bus, USB, parallel-port or watchdog access happens before the banner. Cold-boot detail not settled: whether the reset-status byte read in step 1 returns the flash byte or a register (U3).

**After the banner, in order (established, main task `0xffe10570`).**

7. `0xffe549b0`: clears a 32-byte descriptor scratch at `0x30001e10`, sets the retained-record pointer if unset, reads the "restarted" word at `0x301019ac` (via `0xffe566e8`). This decides later whether the expander cold-start application runs.
8. Identity `0xffe55604`: start the M-Bus (`0xffe6cc98(6)`: create the bus lock, run the controller bring-up of section 8 directly, then clear `IMR` bit 11 to unmask the M-Bus interrupt); store the flash class from a read of `CSMR0` (`MBAR+0x68`: `0x000f0000` gives 1, `0x001f0000` gives 2); read board EEPROM record `0xff`; if the type is still `?` read record `0xfe`, then `0xfd` then `0xfc`. These four reserved record numbers resolve to M-Bus slave addresses on segment 0 through a four-row default table at `0xffe6be3c` (`0xffe6b0b8`): `0xff` wire `0xa4` (7-bit `0x52`), `0xfe` `0xa8` (`0x54`), `0xfd` `0xa0` (`0x50`), `0xfc` `0xac` (`0x56`), each 256 bytes. The decoded name string is compared by prefix: on record `0xff` `IP35` gives C, `KCARL1` G, `IP34` A, `POWERON` T, `X2PWR` Y, `NIPWR` `&`, `NPWR` N, `RPWR` R, `TWPWRIN` 6, `WS_INT_53` 9, `2U_INT_53` 7; on record `0xfe` `ROUTER` R, `PBRICK` P, `IBRICK` I, `XBRICK` X, `X2BRICK` Y, `PXBRICK` `#`, `IP45` 4, `PEBRICK` `%`, `2U_INT_53` 7; on record `0xfc` the name `BIC` gives `%`. If still unknown the stored setting 9 supplies the type. Then the type's descriptor initialiser runs (section 8) through `0xffe79ab8`. A type not in the supported set logs "Defaulting to 'unknown' brick type" and uses the blank board.
9. UART and USB (`0xffe55548`, calling `0xffe54f90`): CS4 write; quad UART probe and port opens (section 6); on-chip UART 1 open; if the USB count is nonzero CS3 write and USB probe; letter `6` only: second quad UART and USB slot 1. Console output starts here.
10. Parallel-port scan set-up (`0xffe71ee0`).
11. Board registration (`0xffe453f8` C, `0xffe46028` I, `0xffe4a234` R): installs the select hook, expanders, EEPROM records, monitors, supplies, fans, temperatures and ports and starts the display. All three then call `0xffe4a83c`: a sanity check (`0xffe4a7b0`), which returns nonzero at once when the "restarted" word is clear; when it is nonzero, every enabled expander is written once with its applied value ORed with its input bits and read back (`0xffe6664c`); then the EEPROM scan of every registered record (board header, areas, SPD; `0xffe6bdfc`), the monitor initialisation (`0xffe69ca0`) and three routines not followed (`0xffe4e600`, `0xffe58340`, `0xffe4ae94`). The two interrupt-polling workers (`0xffe6c15c` for the high-level line, `0xffe6c0d4` for the low-level line) are registered last. The C-brick runs its own board initialisation (`0xffe4482c`) before `0xffe4a83c`.
12. Settings 1 and 2 form the L1 address; request router, general, command, loopback, flash, peer-link, management and node-link tasks start; the type-specific start-up arm; the watchdog is armed (`0xffe1076a`); the power-bay task starts for C, I and R; the display shows `L1 running`, or `L1 rebooted` when the restarted word is set.

## 12. Unknowns and the software test that settles each

| ID | Unknown | Why open | Test |
|---|---|---|---|
| U1 | Fitted processor clock, and whether timer 2's output is the quad UART clock | no image reads or states the clock; four derived constants agree on 48 MHz | search the L2 root file system and kernel (`l2.bin`) and the IRIX host tools for a constant or message giving the L1 link or tick rate (107,142 or 57,692 baud, a crystal frequency); decode the divisor used for the same link on the L2 side and solve for the clock |
| U2 | Meaning of byte writes to `MBAR+0x07`, `+0x1b8`, `+0x1bc` | not in the Linux register header | extract the manual's complete register map once its text is retrievable and look up the three offsets; scan the other four ColdFire packages for the same writes and compare their context |
| U3 | What the loader's read of `0x10000040` returns before `MBAR` is written | needs CS0's reset decode | read the manual's CS0 reset-state description; the image result is the same either way (cold boot takes the table scan) so only the fidelity of the read is open |
| U4 | Which Bedrock input each C-brick reset, soft-reset, NMI and strap bit reaches | the image labels only `BEDROCK CNTL 1/2` | search the IRIX kernel and PROM symbol and string tables and the L2 daemon for register or pin names that the `reset`, `softreset` and `nmi` commands correspond to; read the Bedrock (SHub-predecessor) register documentation in the SGI manuals for reset-control bits |
| U5 | Which chip pins the I-brick `XBRIDGE CONTROL` and R-brick `ROUTER CONTROL` reset bits reach | same | same search for the XBridge and router control documents; compare the release order (bits 1 and 3, then 0) with the documented reset sequence |
| U6 | Fitted monitor, expander, bus-switch, EEPROM and quad-UART parts | no image names them | decode the register traffic against the LM78/DS1780, PCF8574/PCA9554-style and 16C654 register maps (Linux drivers as references) and look for a distinguishing read the firmware makes, for example identification registers `0x3e`/`0x3f`; search IRIX and L2 binaries for part names |
| U7 | What drives external interrupt levels 1 and 7, and the level 3 and 5 pins on these bricks | no handler install found | scan package 0 for stores to RAM vector entries 25 and 31 (`0x30100064`, `0x3010007c`) and to `ICR1`, `ICR3`, `ICR5`, `ICR7`; extend the scan to packages 2 and 1 |
| U8 | What sets the parallel-port pulse gate (RAM word and board-byte bit 7) on C, I, R | not followed | find stores to the gating RAM word and to record offset `+0x41` in the board code of each brick |
| U9 | Flash manufacturer and device codes | autoselect result unused | search packages 0 to 5 and `flashsc` for comparison constants against the two autoselect halfwords |
| U10 | Which cable each peer port reaches; C slot 1/9 against I slot 9/10 | selectors only | read the router slot tables and the neighbour handshake that fills the module records for the pairing |
| U11 | The bus switch behind the C-brick `BUS SELECT` values | not named | read the user guide wiring text and look in IRIX for the same segment values |
| U12 | NVRAM and clock part, battery-low flag | image uses only the register layout | check whether any routine tests bits of `0x200007f8` other than 6 and 7 (read all 8 writers and readers of that address) |
| U13 | Contents of the 0xb4-byte NVRAM record | not decoded | trace readers and writers of record-area offsets `+0x128`..`+0x1db` |
| U14 | The fan-control record fields, the power-supply-controller row format (`0xffe45a24`), the port-record format (`0xffe459f4`) | not decoded | read the registrar routines `0xffe4a9b4`, `0xffe58070`, `0xffe4f454` and their consumers |
| U15 | Whether package 2's C, I and R tables equal package 0's | not re-decoded | run the table decoder at package 2's initialiser pointers (paired in `personality-diffs`) and diff |
| U16 | Front-panel display text driver and the C-brick `LED0/LED1` expander use | not decoded | read `0xffe5152c` and its callees, the LED expander writers (index 1 to 4 on C) |
| U17 | The time of a kernel tick in real time | inferred 100 Hz | follow from U1 |
| U18 | The C-brick board initialisation (`0xffe4482c`, `0xffe44ae0`), the I-brick limit-table registration (`0xffe487c0`, constant 87.5) and the three routines at the end of `0xffe4a83c` | not followed | disassemble each and list the expander, monitor and EEPROM helpers it calls, with the literal arguments |
