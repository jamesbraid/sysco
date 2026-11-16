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

## What the L1 reports about DIMM population

Package 1 (1.44.0, `Fuel/PE/O300 1MB`) has no record of which DIMM pair the factory fitted; it only reads what is present. Nothing in its configuration table (`config`, rack, slot, partition, type and flash size) covers memory. Population comes from the four DIMM serial-presence records read at startup (see [mbus.md](mbus.md)): a DIMM whose first eight-byte read fails is marked absent and reads back as `no hardware detected` (string at body offset `0x24950`); a present one is listed as JEDEC SPD data. Established from the strings and the probe note in mbus.md; the formatting routine was not read.

- The published `eeprom` output (007-4365-001 printed pp. 75 to 76) prints `DIMM n - no hardware detected` or `DIMM n (JEDEC)` followed by the 64-byte SPD dump, for DIMM 0 to 3, after the node and I/O-8 records.
- The package carries a DIMM listing with the header `<4-char name>EEPROM     JEDEC-SPD Info           Part Number        Rev  Speed  SGI     <text>` and a date and size column (`DATE   DIMM Size`, size printed as `%d%s(%dx%d%s)`). Its help lines read "show dimm part/serial numbers." and "show dimm part/serial numbers with extended data and EEPROM indexes." (body offsets `0x22ead`, `0x22ecc`), next to the brick part and serial lines. The exact command words were not decoded: unknown.
- The name table for the four records lists them in the order `DIMM 0`, `DIMM 2`, `DIMM 1`, `DIMM 3` (body offsets `0x4616d` to `0x46182` and `0x4a133` to `0x4a148`). Which memory bank each DIMM number is, and so whether DIMM 0 and 1 are the first pair, is unknown here; route: follow the PROM's DIMM read to the multiplexer select (see the chips repository's `platforms/ip35/memory-population.md`).
- The manual's example prints DIMM 0 and 1 absent and DIMM 2 and 3 present. That is one example from a four-processor system, not a factory census.
