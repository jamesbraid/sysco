# MCF5206e M-Bus: the generic module

This page is the chip's own contract for the on-chip M-Bus module of the Motorola MCF5206e
ColdFire processor. It says what the manual specifies, independent of any board. The Origin 300
L1 uses the module, and the board facts are in [l1/mbus.md](l1/mbus.md) and
[l1/ip45-mbus-public-fitment.md](l1/ip45-mbus-public-fitment.md). The register layout and
field names are in [devices/mcf5206e/registers.toml](../devices/mcf5206e/registers.toml), and the generated reference is
[devices/mcf5206e/registers.md](../devices/mcf5206e/registers.md).

Sources:

- MCF5206e User's Manual, MCF5206EUM rev. 1 (1998), chapter 13 (M-Bus) and chapter 8 (system
  integration, interrupt routing), cited below as "manual" with printed page numbers.
  [PDF](https://www.nxp.com/docs/en/data-sheet/MCF5206EUM.pdf)
- MCF5206e User's Manual errata, MCF5206EUMAD rev. 3 (2001), printed pages 17 and 18, cited as
  "errata". [PDF](https://www.nxp.com/docs/en/reference-manual/MCF5206EUMAD.pdf)

Grades here follow the rule in [AGENTS.md](../AGENTS.md): nothing in this page has been
confirmed in a shipped image, so manual-derived facts are inferred. Unknown means no source
settles the point.

## Registers

Five 8-bit registers at MBAR-relative offsets `+0x1e0` (MADR), `+0x1e4` (MFDR), `+0x1e8`
(MBCR), `+0x1ec` (MBSR) and `+0x1f0` (MBDR). The manual's diagrams mark all five readable and
writable in supervisor and user mode (manual pp. 13-6 to 13-11).

Reset values, from the manual's register diagrams (manual pp. 13-6 to 13-11):

| Register | Reset | Note |
| --- | --- | --- |
| MADR, MFDR, MBCR, MBDR | `0x00` | |
| MBSR | `0x81` | `MCF` and `RXAK` set |

Unknown:

- Whether clearing `MEN` restores the power-on byte of every register, or resets only the
  transfer logic and keeps the programmable bytes. The manual says the module is held in software
  reset while `MEN=0`, and that the registers stay accessible.
- The result of word, longword or misaligned access to these bytes. Chapter 13 does not say.
- Reserved-bit readback and writes: MADR bit 0, MBCR bits 1 and 0, MBSR bit 3, and MFDR bits 7
  and 6.
- What the last MBDR read in receive mode exposes.

Software writes to MBSR clear only `MAL` and `MIF`, by writing zero. The register diagram
marks MBSR read/write, but section 13.5.4 limits software writes to those two clears (manual
pp. 13-8 to 13-10). The other bits report hardware state.

## Enable, start and stop

- `MEN=0` holds the module in reset. `MEN=1` enables it. Other MBCR control bits have no effect
  until `MEN` is set (manual pp. 13-8 to 13-10).
- On reset the interface is a slave receiver.
- With the module enabled, a rising `MSTA` selects master mode and generates START. A falling
  `MSTA` generates STOP and returns to slave mode. Arbitration loss clears `MSTA` without a STOP
  (manual sections 13.3 to 13.4, pp. 13-2 to 13-10).
- `MBB` rises when START is seen and falls when STOP is seen. A START requested while the bus
  is already busy is an arbitration-loss case.
- A repeated START requested in slave mode, or while another master owns the bus, loses
  arbitration.

## Byte transfer

- In master mode, `MTX=1` sends the next byte written to MBDR, most significant bit first. The
  calling byte carries the seven-bit target address in bits 7 to 1 and the direction in bit 0.
  MADR is the module's own slave address, not the calling byte (manual sections 13.4.2 to 13.4.6).
- A master receiver sets `MTX=0` after sending the calling byte.
- In receive mode, `TXAK=0` drives ACK after a received byte and `TXAK=1` drives NACK. `TXAK`
  does not affect the acknowledge the module receives while it transmits.
- A master receiver sets `TXAK=1` before it reads the second-last byte, and requests STOP before
  it reads the last one.
- Writing `RSTA=1` as the current master generates a repeated START without a STOP. `RSTA` reads
  zero (manual sections 13.5.3 to 13.5.5 and 13.6.3 to 13.6.5).
- Clock stretching, arbitration on SDA and the nine-clock byte are bus behaviour in the generic
  manual (manual sections 13.4.6 to 13.4.9, pp. 13-4 and 13-5). How a fitted peer stretches the
  clock is not in the manual.

## Status and interrupt request

- `MCF` clears during a byte transfer and sets on the falling edge of the ninth clock.
- `MIF` sets when a byte completes, when the module is addressed as a slave receiver, and on
  arbitration loss.
- `MIEN=1` lets `MIF` request a ColdFire interrupt. `MIEN=0` masks the request and leaves `MIF`
  set.
- `MIF` and `MAL` are cleared independently, by writing zero to each.
- `RXAK=0` means the byte was acknowledged on the ninth clock. `RXAK=1` means NACK.
- An own-address match sets `MAAS`. An MBCR write clears it. `SRW` gives the direction of the
  completed address match only.
- Arbitration loss sets `MAL`, sets `MIF`, and clears master selection without a STOP.
- The module has one interrupt output. It requests the processor when `MIF && MIEN`.

Errata (MCF5206EUMAD rev. 3, printed p. 18, correction to section 13.6.6):

- A general call to address zero is not recognized.
- A STOP does not generate a slave interrupt. A slave must count bytes or poll for STOP.

## Interrupt routing through the SIM

Chapter 8 routes the M-Bus request to the SIM (manual pp. 8-5, 8-7, 8-10 to 8-12).

- The control register is ICR11 at `MBAR+0x1e`. It resets to `0x80`: the autovector bit is
  forced to one, the level in bits 4 to 2 and the priority in bits 1 and 0 are programmable.
- Level zero requests no processor interrupt. A nonzero level's autovector number is 24 plus
  that level.
- The mask is bit 11 of IMR at `+0x36`. It resets to masked (one). A masked bit does not
  deliver the request.
- The pending bit is bit 11 of IPR at `+0x3a`. An active internal request sets it on the
  following processor clock, whether or not the mask is set.
- The pending bit clears when the request clears, or at the end of the interrupt acknowledge.
- Acknowledge does not clear `MIF`. Software must clear `MIF` to remove the level-sensitive
  condition.
- Inferred from that level-sensitive condition: a request that continues after acknowledge may
  become pending again. The timing of that re-pend needs a test rather than an assumption.

This is the generic routing. The level, priority and vector a given board programs are firmware
facts; the L1's programmed value is in [l1-board-contract.md](l1-board-contract.md).

## Clock selector

`MBC5` to `MBC0` in MFDR index manual table 13-2. The selector is not the divider itself
(manual p. 13-7):

| Selector | Divider |
| --- | --- |
| `0x00` | 28 |
| `0x07` | 68 |
| `0x13` | 480 |
| `0x20` | 20 |
| `0x3f` | 2048 |

Unverified: the `0x07` and `0x20` rows are not yet checked against the manual's table 13-2.

Errata (MCF5206EUMAD rev. 3, printed p. 17, correction to section 13.5.2): the SCL frequency is the
CPU frequency divided by the selected divider. The selection must keep the M-Bus below 100 kHz.

The rate in hertz follows from the controller clock, which the manual does not give for a
board, so the rate is unknown. For `0x13` (divider 480) the L1 firmware's clock inference is in
[l1-board-contract.md](l1-board-contract.md).

## No DMA request

Chapter 13's programmer model and block diagram have no M-Bus DMA register or request. Software
moves each byte through MBDR (manual figure 13-1 and table 13-1, pp. 13-2 and 13-6). This says
nothing about whether the chip has a separate DMA controller.

## Unknown on the IP45 board

The generic contract does not give:

- the SCL rate the IP45 bus runs at, since the controller clock is not specified for that board;
- which slaves are fitted, their pull-up values, or the bus branches;
- how a fitted peer stretches the clock;
- the interrupt level, priority and vector the IP45 firmware delivers in practice. The firmware
  writes `0x99` to ICR11 (level 6, priority 1); see [l1-board-contract.md](l1-board-contract.md).
  Whether that is what the board sees is not captured anywhere.

