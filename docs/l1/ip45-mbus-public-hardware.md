# Origin 300 L1 M-Bus: public hardware evidence

The published Origin 300 documentation proves that the L1 reads module identity EEPROMs and DIMM serial identification data. It does **not** identify the fitted EEPROM parts, the I/O-8 controller processor, an M-Bus switch, or the route between the I/O-8 assembly, IP45 motherboard, and DIMM sockets. Those details remain unknown for an Origin 300 board revision. Neither an industry-standard EEPROM address nor a chip photographed in an Origin 3400 C-brick establishes Origin 300 fitment.

## Established for the Origin 300

SGI's *Origin 300 User's Guide*, revision `007-4365-002`, calls out a serial ID EEPROM on the IP45 motherboard, four DIMM slots, and an I/O-8 assembly comprising a main board, daughtercard, and PCI riser. The assembly provides L1 controller functions. The guide describes a motherboard-to-riser connection and two I/O-8-to-riser connections, including a 50-pin AMP connector. It does not assign EEPROM bus signals to any of them.

The earlier guide, revision `007-4365-001`, says the L1 reads and writes module identification EEPROMs and reads DIMM serial ID PROMs. Its published `eeprom` command example prints separate `NODE`, `IO8`, and `DIMM` records. The node record contains the example board string `IP45_4CPU` and part `030_1728_001`. The I/O-8 record contains `IO8` and `030_1673_003`. DIMM 0 and 1 report no hardware. DIMM 2 and 3 print `JEDEC` data. These are values in one manual example, not a census of fitted devices or proof that each record occupies a separate serial chip.

The same appendix prints `I2C_HIGH` and `I2C_LOW` task names in an `istat tasks` example. It also assigns four CPU LED groups to I/O expander addresses written as `A_0100001x` through `A_0100100x`, and describes an expander `B_0100001x` used for NMI. This is published controller behavior and address notation.

## Chip contracts and transfer limits

Motorola's *MCF5206e ColdFire Integrated Microprocessor User's Manual* describes an on-chip M-Bus module with bidirectional open-drain SCL and SDA pins. Its section 13 says the module is compatible with I²C, supports master mode, and generates START, STOP, repeated START, and acknowledge cycles. These establish capabilities **if** an Origin 300 board fits an MCF5206e and uses those pins. They do not prove that conditional premise. The manual's separate “two-wire mode” in section 6 concerns arbitration for the processor's parallel external memory bus. It is not the M-Bus serial interface.

A private collector's photographed Origin **3400 C-brick** identifies its L1 as a Motorola MCF5206E. That is direct fitment evidence for the pictured Origin 3400 hardware, whose board and system assembly differ from Origin 300 IP45/I/O-8.

The Linux EEPROM driver documentation lists 24C01A, 24C02, and 34C02 variants as typical DIMM serial EEPROMs, commonly addressed in the I²C `0x50`–`0x57` range. Microchip's 24C02C is an example 2-Kbit, two-wire serial EEPROM. These sources explain a plausible device class for DIMM SPD and module identity stores. Neither identifies a chip fitted to an Origin 300 DIMM, node, or I/O-8 board. [Linux kernel, `eeprom` driver documentation](https://origin.kernel.org/doc/html/v5.15/misc-devices/eeprom.html). [Microchip, 24C02C product documentation](https://www.microchip.com/en-us/product/24C02C).

## Unknown fitment and software routes

No public source read here identifies the I/O-8 L1 processor marking, the EEPROM manufacturer and parts, a fitted multiplexer, the bus segment count or pin-level wiring. The four DIMM slots are a population limit, not four fitted SPD devices. The manual's `NODE`, `IO8` and `DIMM` output shows readable categories, not address, device count or route. The firmware settles part of this: it reads six records at the addresses and bus selections tabulated in [mbus.md](mbus.md), through selectors 6 to 9 for the DIMMs. For the remaining items the routes are the IRIX-side L1 host objects (which name the records and their layout) and the multiplexer select writes in the IP45 select callback (`0xffe457fc`). A published schematic would be a hint, not a finding.
