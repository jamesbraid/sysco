# C-brick and I-brick `env` readings: temperature, fan, rail conversions and limits

How the L1 turns monitor registers into the numbers `env` prints, for the C-brick and the I-brick. It extends [env-fan-power-margin.md](env-fan-power-margin.md) (applicability, formats, thresholds) and [cbrick-env-pimm-display-usb.md](cbrick-env-pimm-display-usb.md) (which monitor each record names). Read from L1 1.44.0 of the IRIX 6.5.30 `l1.bin` (sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), ColdFire package 0 (`Base 1MB image`, body sha256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`, loaded at `0xffe00000`), Ghidra decompile with capstone listings of the constants. Addresses are package 0. The C-brick and I-brick monitor, supply, tachometer and temperature tables are byte-equal in package 2 (compared with the name pointers masked; the C-brick bias set-up is at `0xffe4b5b8` there). Grades follow [AGENTS.md](../../../AGENTS.md).

Soft-float helpers in this image take their operands in D0:D1 and on the stack. Their roles are read from use: `0xffe7e820` multiplies, `0xffe7e39c` adds, `0xffe7e396` subtracts, `0xffe7e64c` divides (doubles), `0xffe7e934` multiplies floats, `0xffe7e160` and `0xffe7e772` convert an integer to double and float, `0xffe7e32e` converts a double to an integer (truncation assumed, inferred), `0xffe7ed98` widens a float. The `env` temperature line shows `1.8 x C + 32` through these (constants `0x3ffccccccccccccd` and `0x40400000`).

## 1. The monitor part

Every C-brick monitor and every I-brick monitor is the firmware's monitor type 1: seven-bit addresses `0x2c`, `0x2d` and `0x2e` (written `0x58`, `0x5a`, `0x5c`), registers as in [cbrick-env-pimm-display-usb.md](cbrick-env-pimm-display-usb.md) section 1 (voltages `0x20` to `0x25`, temperature `0x27`, fans `0x28` and `0x29`, limits `0x2b` to `0x36`, `0x39` to `0x3c`, status `0x41` and `0x42`, masks `0x43` and `0x44`, configuration `0x40`, divisor byte `0x47`). The register map matches the Dallas DS1780 (and the pin-compatible National LM80 and Analog Devices ADM1024 families): hint from [mbus-ds1780-candidate.md](mbus-ds1780-candidate.md); the part is not named by the image. The monitor record is 12 bytes: type, segment, wire address, a flag byte (1 means "do not start": `NODE 2` and `POWER 2`), a halfword that is the initial divisor byte for register `0x47`, and a name pointer.

| Brick | Monitor | Segment | Wire | `0x47` start value | Started |
|---|---|---:|---|---|---|
| C | `NODE 0` | 2 | `0x58` | `0xa0` | yes |
| C | `NODE 1` | 2 | `0x5a` | `0x20` | yes |
| C | `NODE 2` | 2 | `0x5c` | `0x00` | no (flag) |
| C | `PIMM0 A` / `PIMM0 B` | 4 | `0x5c` / `0x5e` | `0x00` | yes |
| C | `PIMM1 A` / `PIMM1 B` | 5 | `0x5c` / `0x5e` | `0x00` | yes |
| I | `POWER 0` | 0 | `0x58` | `0xa0` | yes |
| I | `POWER 1` | 0 | `0x5a` | `0x20` | yes |
| I | `POWER 2` | 0 | `0x5c` | `0x00` | no (flag) |

(Established: tables at `0xffe45770` and `0xffe468d0`.)

## 2. Temperature

- **Source (established, `0xffe69b8c`, `0xffe5e504`).** One byte from register `0x27` of the sensor's monitor, taken as a signed 8-bit number, one degree Celsius per count. Nothing else is read; the half-degree bit of the part's second register is ignored.
- **Offset.** The shown value is `reg - record_bias - board_bias`. The record bias is byte 2 of the sensor's 8-byte table entry (monitor, channel `4`, bias, name pointer): 0 for all four C-brick sensors (`0xffe459d4`) and both I-brick sensors (`0xffe469fc`). The board bias is a word per sensor in the set-up table the board installer fills (`0x30001e5c` and up, read by the env reset `0xffe27e34` and the `env` printer `0xffe2b000`): **the C-brick installer sets it to 4 for all four sensors** (`moveq #4` stored at offsets `0x2c`, `0x30`, `0x34` and `0x38` of the brick-operations block, `0xffe45548`); the I/O installer (`0xffe4677c`) stores nothing there, so it stays 0. Established.
- **Result.** A C-brick sensor shows `register - 4` degrees: a register of 0 shows `-4C` and, in the Fahrenheit column, `24` (`1.8 x (-4) + 32 = 24.8`, truncated). A failed read also prints `-4C` on the C-brick, because the read routine returns 0 and the printer still subtracts the bias (established, `0xffe5e504`). On the I-brick a register of 0 shows `0C`.
- **Which sensors.** C-brick: four, `PIMM0 A`, `PIMM0 B`, `PIMM1 A`, `PIMM1 B` (monitors 3, 4, 5, 6, channel 4). There is no node-board sensor. With PIMM 1 absent monitors 5 and 6 are disabled and print `<not present>`; PIMM 0 absent leaves monitors 3 and 4 enabled, so they print (`-4C` if the model returns 0). I-brick: two, `POWER 0` and `POWER 1` (monitors 0 and 1, channel 4).
- **Limits.** The C-brick limits are the monitor-type-1 row of [env-fan-power-margin.md](env-fan-power-margin.md) plus the board bias: 34, 39, 44, 49 and 31 in register terms (shown as 30, 35, 40, 45, 27). The programmed advisory and okay values go to registers `0x39` and `0x3a` (`0xffe69488`).

## 3. Fans

- **Tachometer records** (12 bytes, `0xffe459b0` C, `0xffe469d8` I; `0xffe4ac9c` reads them): `LEFT` is monitor 0 input 6, `CENTER` monitor 0 input 7, `RIGHT` monitor 1 input 6, on both bricks (`NODE 0`/`NODE 1` on the C-brick, `POWER 0`/`POWER 1` on the I-brick). Established.
- **Count registers:** input 6 reads register `0x28`, input 7 register `0x29` (`0xffe68814`).
- **Conversion (established):** `RPM = 1,350,000 / (count x divisor)`, then multiplied by a per-record factor that is 1.0 for all six records (the low nibble of record offset 8: 1 would give 0.5, 4 gives 2.0, 8 gives 4.0, other 1.0). The divisor for input 6 is `{1,2,4,8}[(reg47 >> 4) & 3]` and for input 7 `{1,2,4,8}[(reg47 >> 6) & 3]`, with `reg47` the value written to register `0x47` at monitor init (table `0xffe6a272`). The start values `0xa0` and `0x20` both select divisor 4, so the shown speed is `337,500 / count`. A count of 0 or `0xff` leaves the speed 0.
- **Warning limit programming (established, `0xffe68794`):** register `0x3b` (input 6) or `0x3c` (input 7) gets `(1,350,000 / (limit_rpm x divisor)) & 0xff`; the nominal and warning speeds are in [env-fan-power-margin.md](env-fan-power-margin.md).
- **Count:** three fans on each brick, `FAN 0` to `FAN 2` in `env`. **Speed control** is not through the monitor but one 5-bit field of an expander (10-byte fan-control record, kind 1: enable expander and mask, speed expander, shift, mask). C-brick (`0xffe459a4`): expander 11 `FAN CONTROL` (segment 3, wire `0x4c`), speed in bits 4:0, enable bit 5. I-brick (`0xffe469cc`): enable is bit 1 (`0x02`) of expander 0 `POWER ENABLE` (segment 0, wire `0x70`), speed bits 4:0 of expander 1 `FAN SPEED` (wire `0x72`). The speed field holds the bitwise complement of the 0 to 31 value (written by `0xffe4b1f4`, read back inverted by `0xffe4adc0`); the enable is active low (the enable call clears the mask, the disable call sets it). Established, polarity from the set and clear helpers.

## 4. The third node monitor, `NODE 2`

No supply, fan or temperature record names monitor 2; the configure loops never touch it, and its start flag is set so `0x40` is left at `0x80` after init (established, [cbrick-env-pimm-display-usb.md](cbrick-env-pimm-display-usb.md) section 1). It has **no measurement channel in use**. The firmware still: initialises it (`0x40`=`0x80`, `0x43`=`0xdf`, `0x44`=`0x13`, `0x47`=`0x00`); reads its status registers `0x41` and `0x42` on every poll (`0x00` is quiet); and uses its register `0x19` as the margin device of the `DIMM` supply (record 3): the DIMM margin routine writes a byte there (static default norm `0x88`, high `0xc9`, low `0x3e`; [cbrick-pimm-psc-codes.md](cbrick-pimm-psc-codes.md) section 4). Established; the register's analogue-output role is inferred from the family map. The I-brick's `POWER 2` is the same: no channel, only init and the poll.

## 5. Rail conversion

For a rail the printer (`0xffe4c160`) reads the monitor count (inputs 0 to 3 read registers `0x20` to `0x23`, input 8 register `0x24`, input 9 register `0x25`; `0xffe68c9c`) and multiplies it by the volts-per-count of the input (`0xffe67e9c`, type 1):

| Input | Register | Volts per count | At count 192 | Used for |
|---:|---:|---:|---:|---|
| 0 | `0x20` | 0.0130 | 2.496 V | 2.5 V rails |
| 1 | `0x21` | 0.0141 | 2.707 V | `PIMMn VCPU` |
| 2 | `0x22` | 0.0172 | 3.302 V | 3.3 V rails |
| 3 | `0x23` | 0.0260 | 4.992 V | 5 V rails |
| 8 | `0x24` | 0.0625 | 12.000 V | 12 V rails |
| 9 | `0x25` | 0.0141 | 2.707 V | `PIMMn 1.5V`, and `-12V` on the I-brick |

(Float constants `0x3c54fdf4`, `0x3c6703b0`, `0x3c8ce704`, `0x3cd4fdf4`, `0x3d800000`, `0x3c6703b0`; established.) Then a class from the supply-type table (`0xffe4e6e8`, 12-byte entries) is applied:

- class 0: none;
- class 1 (type `-12V`): a count of `0xff` reads 0 V; otherwise **`V = 10.090909 x v - 37.236364`**, with `v` the product above (constants `0x40242e8b9fdbd2fa` and `0xc0429e412cf0f9d3`; established). A count `c` on input 9 therefore reads `0.142282 x c - 37.2364` volts: count 177 is -12.05 V, 178 is -11.91 V, 0 is -37.24 V (not printed as 0) and `0xff` is 0;
- class 2 (SRAM): multiplied by the per-PSC ratio of [cbrick-pimm-psc-codes.md](cbrick-pimm-psc-codes.md);
- class 3 (24 V): x 9.546; class 4: x 0.43668; class 5: x 2; class 6: x 3; class 7: x 7.

## 6. I-brick monitor channels (established, tables at `0xffe468d0` to `0xffe469fc`)

Nine supply records of 24 bytes. The enable bit is a mask on expander 0 `POWER ENABLE` (segment 0, wire `0x70`; record bytes 2 and 3); clear is on. Nominal is the supply-type table's value. Warning and fault columns are the register limits programmed (counts, high register `0x2b`/`0x2d`/`0x2f`/`0x31`/`0x33`/`0x35`, low register the next one): the firmware programs the 10 percent pair first and the 20 percent pair after a warning (state flag `0x300045da`), clamping to 1 through 254; values are `nominal x (1 -/+ fraction) / volts-per-count` truncated (truncation inferred).

| Rec | Name | Type | Nominal | Monitor, input | Register | Enable bit | Warn lo / hi | Fault lo / hi |
|---:|---|---|---:|---|---:|---|---|---|
| 0 | none | 48V | 48 V | none | none | `0x01` | none | none |
| 1 | `12V BIAS` | 12V | 12 V | `POWER 1`, 8 | `0x24` | `0x04` | 172 / 211 | 153 / 230 |
| 2 | `2.5V` | 2.5V | 2.5 V | `POWER 0`, 0 | `0x20` | `0x08` | 173 / 211 | 153 / 230 |
| 3 | `3.3V` | 3.3V | 3.3 V | `POWER 0`, 2 | `0x22` | `0x10` | 172 / 211 | 153 / 230 |
| 4 | `12V` | 12V | 12 V | `POWER 0`, 8 | `0x24` | `0x40` | 172 / 211 | 153 / 230 |
| 5 | `5V` | 5V | 5 V | `POWER 0`, 3 | `0x23` | `0x20` | 173 / 211 | 153 / 230 |
| 6 | `-12V` | -12V (class 1) | -12 V | `POWER 1`, 9 | `0x25` | `0x80` | 175 / 198 | 163 / 210 |
| 7 | `3.3V AUX` | 3.3V | 3.3 V | `POWER 1`, 2 | `0x22` | none | 172 / 211 | 153 / 230 |
| 8 | `5V AUX` | 5V | 5 V | `POWER 1`, 3 | `0x23` | none | 173 / 211 | 153 / 230 |

Other I-brick channels: temperature `POWER 0` and `POWER 1` input 4 (register `0x27`, bias 0, section 2); fans section 3; inputs 1 of the monitors are not used.

**The -12 V limits are not the inverse of its reading (established).** The limit routine (`0xffe282bc`, branch for type `-12V`) uses a different linear map from the display: it computes `(N x (1 -/+ f) + 30.994898) / (7.1989796 x 0.0141)` (constants `0x403efeb1a2a4db16` and `0x401ccbc14eea48df`), where `N` is -12. At the nominal 12 V that is count 187, while the display reads -12.00 V at count 177.4. So with the display formula, count 175 (the 10 percent low limit) is -12.34 V and count 198 (the high limit) is -9.08 V; a model that makes the monitor read -12.0 V (count 177) stays inside the limits, one that reads count 187 (-10.6 V on screen) also does. Which of the two maps matches the real divider is unknown.

The other branches in that routine apply to supply types not on these two bricks: `1.2V` and `1.85V` limits use 15 and 25 percent, and `12V AUX` a 2 V window. (Established for the branch; the 10 and 20 percent figures in [env-fan-power-margin.md](env-fan-power-margin.md) are the `env` display's, not these registers.)

## C-brick supply channels (established)

Twenty records at `0xffe457c4`, monitor and input as in the monitor table above: `48V` (no monitor), `PIMM0 VCPU` (3, input 1), `PIMM1 VCPU` (5, 1), `DIMM` 2.5 V (1, 0), a second 2.5 V (0, 0), a 1.5 V (0, 1), `3.3V AUX` (0, 2), `3.3V L1` (1, 2), `5V AUX` (0, 3), `12 BIAS` (0, 8) and for each PIMM `SRAM` (input 0 of monitor 4 / 6, 2.5 V or 3.3 V by PSC), `1.5V` (input 9), `3.3V AUX` (2), `5V AUX` (3), `12V BIAS` (8) on the A monitor. VCPU limits use the PSC nominal ([cbrick-pimm-psc-codes.md](cbrick-pimm-psc-codes.md)): at the 10 percent pair, 1.5 V is count 95 to 117 on input 1.

## Unknown

| Question | Route |
|---|---|
| The monitor part's name and whether its half-degree temperature bit exists | a data sheet matched to the register scan; no image reads an identity register |
| Which -12 V map matches the hardware | the I-brick power board schematic or a capture of a real `env` and its limit registers |
| Rounding of the double-to-integer step | read `0xffe7e32e` against a test value (the printer shows it) |
