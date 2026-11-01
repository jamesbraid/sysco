# A public sensor candidate for the Origin 300 L1 M-Bus

That is public controller-family naming, not an installed IP45 part record or a selected 1.44.0 M-Bus response. The guide does not give a monitor address, chip marking or wiring for Origin 300.

Their seven-bit addresses are `0x2c`, `0x2d` and `0x2e`. The software also has descriptions for ten supplies, five fan tachometers and two temperatures. A software route or description does not establish which device responds on a fitted board.

The manufacturer’s [DS1780 CPU Peripheral Monitor datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/DS1780.pdf) gives its power-on seven-bit slave address as binary `01011(A1)(A0)` on page 4 of 28. Its two address pins therefore allow `0x2c` through `0x2f`; the first three values coincide with the selected software routes. The same datasheet describes six voltage inputs, two tachometer inputs, one temperature sensor, an interrupt output and a two-wire interface on pages 1–3. Three address-strapped DS1780 devices could provide enough inputs for the *counts* in the software tables, but those counts are descriptions rather than proven fitted channels. Other parts can share these addresses, and the DS1780 can change its slave address through its serial-address register. This is a candidate chip-family match, not an Origin 300/IP45 fitment finding.

The DS1780 sheet assigns read-only company-ID register `0x3e` the power-on byte `0xda` and stepping register `0x3f` the power-on byte `0x01` on page 5 of 28. A value match alone would not prove a specific board population or settle every expander, EEPROM and sensor route. The user's available Origin 350 and Octane2 do not establish IP45 fitment by component-family resemblance.

The official public PDF was read directly. Its product specification establishes DS1780 behavior; the selected firmware reading establishes only software routes. The fitted device, bus address occupancy, interrupt wiring, processor clock, actual SCL rate and sampled values remain unknown.
