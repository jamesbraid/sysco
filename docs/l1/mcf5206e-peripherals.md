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
