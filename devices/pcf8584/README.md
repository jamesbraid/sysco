# PCF8584 I2C controller on the module bus

Each Hub reaches the module system controller (MSC, also called ELSC) through a PCF8584
I2C-bus controller on the management bus. The bus layout is in
[origin2000-module-management.md](../../platforms/ip27/module-management.md); this page covers the chip.
Bit values and field codes are in [devices/pcf8584/registers.toml](registers.toml), with the generated
reference at [devices/pcf8584/registers.md](registers.md).

Unless a line says otherwise, the chip facts below are **inferred**: they are read from the Philips
PCF8584 product specification (21 Oct 1997, tables 2 to 5 and section 6), and no shipped image
programs the chip here.

## Register selection

The data register is not one register. Four selections share one address on the Hub, chosen by the
serial-interface enable bit (ESO) with the register-select bits ES1 and ES2.

| ESO | ES1 | ES2 | Selected register |
|---|---|---|---|
| 0 | 0 | 0 | Own address |
| 0 | 1 | 0 | Clock control |
| 0 | 0 | 1 | Interrupt vector |
| 1 | 0 | 0 | Data |
| 1 | 0 | 1 | Interrupt vector |

ES2 still selects once ESO is set, as long as ES1 is clear. A model that treats "ESO set" as "data
register" gets the interrupt-vector case wrong. With ESO and ES1 both set the chip is in its
long-distance (4-wire) mode, where only the data and control registers are addressable (table 6).
(An earlier version of this table said ES1 stops selecting once ESO is set; the specification's
table 5 lists ESO set only with ES1 clear.)

The control bits used with this table are PIN at `0x80`, ESO at `0x40` and ES1 at `0x20`
(inferred). ES2 is the next bit down, `0x10` (table 4 bit order: PIN, ESO, ES1, ES2, ENI, STA, STO, ACK).

## Own address and transaction addresses

The own-address register holds the seven-bit address. The specification (section 6.4) says the
data register S0 and the own-address register S0' are offset by one bit: programming S0' with
`0x55` makes `0xaa` the recognised slave address. Writing an unshifted value puts the node on the
bus one bit off. The Linux `i2c-algo-pcf` driver's own-address comment was not read.

## Clock control

The clock-control register packs two fields.

- Main oscillator: `0x00` for 3 MHz, `0x10` for 4.43 MHz, `0x14` for 6 MHz, `0x18` for 8 MHz and
  `0x1c` for 12 MHz.
- SCL bus speed: `0x00` for 90 kHz, `0x01` for 45 kHz, `0x02` for 11 kHz and `0x03` for 1.5 kHz.

The Origin 2000 owner's guide is said to describe the management link as 100 kbit/s, with 90 kHz the
nearest setting the chip offers. The guide's text could not be reached in this pass, so the 100 kbit/s
figure is unverified, and it is a family-level statement, not a check against the Origin 200.

## Bus liveness probe

**Established** from the disassembly of the IRIX 6.5.30 IP27 PROM payload (SHA-256
`c2c69bf33c375b43ec7ac511fab41ddf595be1b6838ae717e58a9a1273437fdd`, the bytes from offset `0x1000` to
`0xdedd0` of the `ip27prom` image; offsets below are within that payload), the early probe that asks
whether anything is on the bus (`0x5847c`: sample the data register's low byte, re-read until it differs
or the deadline passes; returns 1 on a change, 0 on timeout):

- The probe reads the data register only. It never reads control or status and never checks
  bus-not-busy.
- It samples the low byte once, then re-reads until the byte differs from that first sample.
- The deadline is 5,000,000 units of the Hub counter after scaling it by 8/10 (payload offsets
  `0x5ad4c` to `0x5ad7c`; probe at `0x5847c`, called at `0x58da8` with the deadline). What a unit
  lasts in time is not established.
- The caller discards the failure return. A controller that never answers therefore delays that
  processor for the whole deadline and nothing more.
- The address is the table value `0x9200000001a20000` (present in the payload's data at offset
  `0xdd7e8`; the probe reads it through the global pointer, and the table slot was not matched to
  that offset) plus `NASID << 32`, for the data register. The routine at `0x58520`, which
  initialises the chip, writes the control register eight bytes higher. No Hub-local alias was
  found in the payload and none is claimed.

Consequence: a data register that returns a fixed value tells the probe that nothing is on the bus.
Any implementation of the controller has to keep the data register changing when the real
controller would send bytes.

## Interrupt output

The chip has an interrupt output, enabled by the ENI control bit. The Hub's slot-identity and
micro-controller status register carries an I2C interrupt bit, named in the public Linux and
OpenBSD headers for that register (inferred; its bit position is not given here). Whether any
guest enables ENI is not known.

## Left out

Facts whose only basis is non-public source are left out, as on the module-management page. That
covers the chip driver's initialization sequence and the controller's arbitration, grant and
address-phase behaviour.

## Used by

No software page touches this unit yet.
