# Origin 300 L1 1.44.0: on-chip peripherals beyond the M-Bus

Read from L1 1.44.0, package 1, first-bank copy. Offsets are from the peripheral base stored at `0x300002e8` (MBAR `0x10000001` is written by the loader candidates, see [boot-memory-base.md](boot-memory-base.md)). Register names come from the MCF5206e user manual and are hints; the offsets, widths and values are read from the image. Per-routine entries are in [functions.tsv](functions.tsv).

| Block | Offsets the code uses | Where |
| --- | --- | --- |
| Interrupt control and mask | bytes `+0x14` to `+0x22`, word `+0x36` (mask), word `+0x3a` (pending) | `0xffe54960` writes the reset values; the M-Bus, I2C and UART code set and clear single mask bits |
| Reset status and watchdog | `+0x40` read and written back, `+0x41` written `0xfe`, `+0x42` written 0, `+0x43` written `0x55` then `0xaa` | `0xffe565b4` keeps bit 5 of `+0x40`; `0xffe562d4` arms the watchdog once; the `+0x43` pair is repeated in the flash erase loop and the flash wait |
| Chip selects | `+0x64` to `+0xc6` | `0xffe549d8`: select 0 maps the flash (1 MB or 2 MB by a mirror test), select 1 `0x20000000`, select 2 the RAM at `0x30000000` (128 KB or 512 KB by an alias test), selects 3 to 7 cleared |
| DRAM controller | `+0x46`, `+0x4a`, `+0x4c`, `+0x50`, `+0x57`, `+0x58`, `+0x5c`, `+0x63` | cleared by `0xffe54b0c`; the offsets match the manual's DRAM controller registers (hint) |
| UART 1 and UART 2 | blocks at `+0x140` and `+0x180`, mask byte at block `+0x14` | `0xffe62de8` fills a state record per port (`0x30002818`, `0x30002824`); service entries `0xffe635f8` and `0xffe6360c` |
| Parallel port | `+0x1c5` written `0xfe`, `+0x1c9` read and written, word `+0xca` | the scan chain interface: `0xffe71d90` sets it up, `0xffe7204c` drives the TAP reset pattern, `0xffe722f8` and `0xffe72454` shift data |

Pin roles on the data byte `+0x1c9`, from the firmware's own pin print routine at `0xffe72254`: bit 0 TDO (input), bit 3 PSI, bit 4 TRST, bit 5 TMS, bit 6 TCK, bit 7 TDI.

Off-chip: a four-channel UART block with registers at a stride of 8 per channel is initialised by `0xffe636b0` at `0x50000000` (external interrupt line 4) and `0x60000000` (line 3), with the value `0xb71b00` as its third argument. The USB device controller windows at `0x40000000` and `0x60000000` are in [usb-controller-registers.md](usb-controller-registers.md). The flash is erased and polled by code the loader copies to RAM at `0x30000000`: `0xffe7f810` (RAM `0x3000072c`) polls the toggle bit and `0xffe7f8b0` (RAM `0x300007cc`) writes `0x30` to a sector address.

## Timer routine and UART 2 output port (established values, meanings from the manual)

The routine at flash `0xffe7f344` is the one the board set-up copies to RAM `0x30000260` and calls last (`0xffe54bce`, with the peripheral base as its only argument; [board-boot-contract.md](board-boot-contract.md)). Read in full to its return at `0xffe7f3ca`, in this order, all offsets from the base:

| Step | Access | Hint from the manual (revision 1, chapters 2, 8, 12 and 14) |
| --- | --- | --- |
| 1 | byte `+0x1b8` written 1 | UART 2 output-port bit set register (`UOP1`); the output port bit is inverted at the pin, so this asserts RTS2 |
| 2 | word `+0xca` ORed with `0x80` | pin assignment register, bit 7 set: the pin carries UART 2 request to send instead of reset out |
| 3 | `+0x100` cleared, `+0x104` = `0x752f`, `+0x100` = `0x001d` | timer 1 mode, reference, mode: prescaler 1, no capture, toggle off, reference interrupt on, restart, clock = system clock divided by 16, timer enabled |
| 4 | word `+0x36` ANDed with `0xfdff` | interrupt mask register, bit 9 cleared; the bit position matches the control register `+0x1c` that the set-up programmed for timer 1 (`0x94`, level 5), so this unmasks the timer 1 tick (inferred from the numbering) |
| 5 | `+0x120` cleared, `+0x124` = 1, `+0x120` = `0x002b` | timer 2 mode, reference, mode: prescaler 1, no capture, toggle output, no reference interrupt, restart, clock = system clock, timer enabled |
| 6 | ten reads of `+0xca` into a stack slot | a delay loop (counter to 10) |
| 7 | word `+0xca` ANDed with `0xff7f` | pin assignment bit 7 cleared again: the pin returns to reset out |
| 8 | byte `+0x1bc` written 1 | UART 2 output-port bit reset register (`UOP0`): negates RTS2 |
| 9 | 1000 reads of `+0xca` | a delay loop |

Established: the offsets, widths, values and order. Inferred: the register names and field meanings (they come from the manual's register map, in which `+0x1b8` and `+0x1bc` are the UART 2 output-port set and reset commands and `+0x100`, `+0x104`, `+0x120`, `+0x124` the timer mode and reference registers). So the image pulses the shared RTS2 and reset-out pin: asserted as RTS2 for about ten register reads and returned to its reset-out role. What the board connects to that pin is not in any image. Nothing in the routine reads the timer counters or event registers; timer 1's tick is consumed by its level-5 interrupt, timer 2 only toggles its output.

Brick-type test in the UART opener (`0xffe54be0`, established): its argument byte (the low byte of its one argument) is compared with `0x47` at `0xffe54c0e`. Only for that value does it take the on-chip UART path: it stores `0x9600` at frame offset `-0x1c`, 1 at `-0x10` and 0 at `-0xc` and calls the per-port initialiser `0xffe62de8` with `1`, `2`, `6`, `0x42` and the address of that frame area (`0xffe54c28` to `0xffe54c3c`), then returns through `0xffe5518e`. Every other value programs chip selects 4 and 5 (`0xffe54c62` to `0xffe54c9e`: `CSAR4` `0x5000`, `CSMR4` 0, `CSCR4` `0x095f`; for the value `0x36` also `CSAR5` `0x6000`, `CSMR5` 0, `CSCR5` `0x095f`) and the four-channel UART path above. Which brick types reach the `0x47` branch was not traced.

Scan limit: a linear-flow scan of the first-bank listing that follows the pointer read from `0x300002e8` through address registers finds base-relative accesses only in routines that load that word themselves. It lists offsets `+0x16`, `+0x36` (46 sites), `+0x3a`, `+0x40` to `+0x43`, `+0x68`, `+0x88`, `+0x8c`, `+0x92`, `+0x94`, `+0x98`, `+0x9e`, `+0xa0`, `+0xa4`, `+0xaa`, `+0xca`, `+0x1c5`, `+0x1c9`, `+0x1e0` to `+0x1f0` (the M-Bus registers), and the block bases `+0x140` and `+0x180` passed on to the UART code. No access to the timer registers appears in that set. Routines that receive the base as an argument (the set-up routines `0xffe54960`, `0xffe549d8`, `0xffe54b0c`, the timer routine and the UART opener) were read directly, as above.
