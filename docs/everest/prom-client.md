# The standalone diagnostic as a second client of the controller

The host daemon is not the only program that talks to the Challenge and Onyx system controller. The standalone diagnostic program for the IP19 CPU board (`stand/ide.IP19` on the IRIX diagnostics discs) carries its own driver for the same UART and its own serial-number access, written independently of the IRIX kernel driver. It gives a second reading of the register interface and of the get and set frames, from a client that runs before any operating system. Grades follow [../../AGENTS.md](../../AGENTS.md).

## Inputs

| Input | Disc | Bytes | Notes |
|---|---|---:|---|
| `stand/ide.IP19` | Diagnostics 5.2 for Onyx Extreme | 1,423,300 | 32-bit-addressed MIPS-III ECOFF, not stripped: the routine names below are the program's own symbols; addresses are the link addresses (`0xa0...`) |
| `stand/ide.IP19` | Diagnostics 5.3 | 1,002,096 | stripped; same routines (UART helpers at `0xa0197d20`-`0xa0197dc0`, serial routines at `0xa019a920` and `0xa019aa7c`); read by constant and shape |
| `stand/ide.IP21` | Diagnostics 6.0.1 | 1,157,808 | 64-bit ELF, stripped; has the same serial-number error texts; not read further |

The 5.2 image is the one read in full for the routines below.

## UART access

**Established** (5.2 `ide.IP19`):

- `ccuart_poll` (`0xa0196380`) loads the 64-bit word at `0xb8000400` and returns bit 1 (`andi 2`). `ccuart_getc` (`0xa0196398`) spins on the same bit and then loads the 64-bit word at `0xb8000408`. `ccuart_putc` (`0xa01963bc`) polls bit 0 of the status word up to 1,000,000 times (`lui 0xf; ori 0x4240`), then stores the byte as a 64-bit word at `0xb8000408`; if the bit never sets it returns without storing. `ccuart_flush` (`0xa01963fc`) reads and discards one byte if bit 1 is set. `ccuart_puthex` (`0xa0196424`) sends eight hex digits, high nibble first, through `ccuart_putc`.
- So the status doubleword is at offset 0 with bit 1 meaning a received byte is waiting and bit 0 meaning the transmitter can take a byte; the data doubleword is at offset 8; accesses are 64 bits wide. This is the register layout the kernel driver uses ([driver.md](driver.md)). The address `0xb8000400` is in the unmapped uncached 32-bit window, so the physical address is `0x18000400`, as already read from the driver objects ([early-builds.md](early-builds.md)).
- The program also registers a character device on this UART (`ccuart_install` at `0xa017cff4`, strategy routine `_ccuart_strat` at `0xa017ce34` with init, open, read, write, ioctl and poll entries). What that device is used for in the diagnostic was not read.

## Reading a reply

**Established** (5.2): `spin_read` (`0xa0198ec8`, 104 bytes) calls `ccuart_poll`; if a byte is waiting it returns `ccuart_getc`'s value; otherwise it calls `us_delay(200)` and tries again, in steps counted up to 1,000,000, and returns -1 if nothing arrived. With a delay argument in microseconds, as the name says, that is a one-second limit per byte. `clear_incoming` (`0xa0198e70`) drains the receive side before a request.

## Serial number: the one request pair the daemon never sends

**Established** (5.2 `sysctlr_getserial` at `0xa0198f30`, 348 bytes; `sysctlr_setserial` at `0xa019908c`):

- Get: drain the input, send `0x18`, `G`, `S`, `S`, `0x0a` (escape, get, tag `S`, item `S`, line feed) with no length digits, then read the reply byte by byte with `spin_read`. The first byte must be `R` (82), the second `S` (the tag) and the third `S` (the item); each mismatch drains the input, prints one of three texts that name the offending value ("got bad response char", "got bad sequence", "got bad response type") and returns -1. Then up to ten more bytes are read into the caller's buffer, stopping at a zero byte (returns 0 with that zero stored); ten non-zero bytes, or a timeout, clear the first byte of the buffer and return -1 (a timeout also prints "spin_read return -1"). So the reply is `R S S` then at most nine characters and a zero, which is the `R tag item data NUL` form of [daemon-wire.md](daemon-wire.md) in its plain, unframed variant.
- Set: copy at most ten characters of the new serial number into a zeroed 11-byte stack buffer (the same text is also copied, up to 11 bytes, to a fixed low address; what reads it was not followed), send `0x18`, `S`, `S`, then exactly ten bytes from the buffer (unused tail bytes are zero), then `0x0a`. It reads no reply (5.3 stripped build at `0xa019aa7c`, same shape).
- These are the controller's item `S` (serial number) in both directions, which the shipped header names (`SC_SERIAL`, can be got and set) and the daemon only reads indirectly, through the PROM inventory ([records.md](records.md)). The diagnostic compares the controller's serial number with the one in non-volatile memory and, when one is invalid, rewrites it from the other: the program's texts say that "NVRAM and system controller serial numbers" disagree, "reprogramming system controller" and "reprogramming nvram" (located by string; the decision routine `read_serial` at `0xa0191084` was not read through).
- This client never sends a length field, never sends the `C` check item and sends no other item. A controller therefore has to answer plain frames from power-up: the diagnostic runs before any daemon could have negotiated length mode. **Inferred**: plain framing is the controller's default and `set_length_mode` in the daemon switches it on.

## What it settles

- The register interface is confirmed by a second, independent client.
- Items `G S` and `S S` (serial number) are real requests with the same framing as the others; the item catalogue in [daemon-wire.md](daemon-wire.md) gains a *seen* entry for `S` (in the standalone program, not in the daemon).
- The meaning of the replies to the length probe is not settled by this program ([../open-questions.md](../open-questions.md)): it never sends the `C` item.
