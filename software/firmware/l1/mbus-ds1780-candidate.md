# A public sensor candidate for the Origin 300 L1 M-Bus

This is a candidate from published naming, not a part record of the IP45. The Origin 300 guide gives no monitor address, chip marking or wiring.

Their seven-bit addresses are `0x2c`, `0x2d` and `0x2e`. The software also has descriptions for ten supplies, five fan tachometers and two temperatures. A software route or description does not establish which device responds on a fitted board.

The manufacturer’s [DS1780 CPU Peripheral Monitor datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/DS1780.pdf) gives its power-on seven-bit slave address as binary `01011(A1)(A0)` on page 4 of 28. Its two address pins therefore allow `0x2c` through `0x2f`; the first three values coincide with the selected software routes. The same datasheet describes six voltage inputs, two tachometer inputs, one temperature sensor, an interrupt output and a two-wire interface on pages 1–3. Three address-strapped DS1780 devices could provide enough inputs for the *counts* in the software tables, but those counts are descriptions rather than proven fitted channels. Other parts can share these addresses, and the DS1780 can change its slave address through its serial-address register. This is a candidate chip-family match, not an Origin 300/IP45 fitment finding.

The DS1780 sheet assigns read-only company-ID register `0x3e` the power-on byte `0xda` and stepping register `0x3f` the power-on byte `0x01` on page 5 of 28. A value match alone would not prove a specific board population or settle every expander, EEPROM and sensor route. Other SGI systems do not establish IP45 fitment by component-family resemblance.

## Software route: the register offsets the firmware writes

The monitor initialization routine (`0xffe6836c`, read in full) writes, through the monitor register wrapper at `0xffe681c0`: offset `0x40` the value `0x80`, offset `0x43` the value `0xdf`, offset `0x44` the value `0x13`, offset `0x47` a byte taken from the monitor description, and finally offset `0x40` the value `0x03` unless a disable flag in the description is set. In the DS1780 register map (hint: data sheet Table 2) `0x40` is the configuration register, `0x43` and `0x44` are interrupt mask registers 1 and 2 and `0x47` is the VID and fan-divisor register. The offsets agree with that map. Inferred: the monitor type is a DS1780 or a part with the LM78 register layout; the offsets do not tell those apart, and a full table of every offset the monitor reads and writes (the readers at `0xffe69844`, `0xffe6904c` and the temperature path at `0xffe5e1bc`) is the next step. The part marking and the board wiring are not named by any image.

The DS1780 sheet's identification registers (`0x3e` company ID, `0x3f` stepping) would tell whether the part is a DS1780 if the code that reads them, if any, is found in this image or in the IRIX-side host objects.
