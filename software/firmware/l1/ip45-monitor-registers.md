# Origin 300 L1 1.44.0: what the three monitor chips are asked

The L1 watches supplies and temperature through three monitor chips on its two-wire bus. This note lists the register accesses the firmware makes to them and how it turns a byte into a reading, as bus traffic a hosting machine has to answer. The part is not named by any image; the register numbers agree with the register map of the DS1780 monitor ([mbus-ds1780-candidate.md](mbus-ds1780-candidate.md)), which stays a candidate.

Read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`), first program copy. Addresses are first-copy addresses.

## The descriptors (established)

Three 12-byte descriptors at `0xffe45de8`, registered by `0xffe45990`:

| Index | Type (+0) | Bus byte (+1) | Wire address (+2) | Flags (+3) | Value for register `0x47` (halfword at +4) |
|---|---|---|---|---|---|
| 0 | 1 | 0 | `0x58` | 0 | `0x00a0` |
| 1 | 1 | 0 | `0x5a` | 0 | `0x00a0` |
| 2 | 1 | 0 | `0x5c` | 0 | `0x0020` |

The bus byte is 0, which makes the bus lock routine take the lock and select nothing (`0xffe6c964-0xffe6ca24`). The three chips are therefore addressed without any segment select, like the identity EEPROMs of the node and I/O boards and the expanders on segment 0 in [ip45-io-expanders.md](ip45-io-expanders.md); that they sit on the unswitched side of the bus switch is inferred from that. An earlier note placed them on segment 1; the descriptor bytes above say 0, and segment 1 in this firmware means the switch is turned off. The seven-bit forms of the addresses are `0x2c`, `0x2d`, `0x2e`.

Each access is one byte: a register write is address, register, value; a register read is address, register, repeated start, address with the read bit, one byte (`0xffe6ce28-0xffe6cefc`).

## Initialisation writes (established)

The routine at `0xffe6836c-0xffe68412` writes, in this order, to each monitor it initialises:

1. register `0x40` the value `0x80`;
2. register `0x43` the value `0xdf`;
3. register `0x44` the value `0x13`;
4. register `0x47` the low byte of the descriptor's halfword (`0xa0`, `0xa0`, `0x20`);
5. register `0x40` the value `0x03`, unless bit 0 of the descriptor's flags byte is set (none are).

It stops at the first bus error. Initialisation iterates all three descriptors (`0xffe69958-0xffe69998`).

## Voltage reads (established)

A type 1 monitor reads a voltage channel as one register byte (`0xffe68954-0xffe68a3e`, jump table at `0xffe689b6`):

| Channel | Register | Volts per count |
|---|---|---|
| 0 | `0x20` | 0.013 |
| 1 | `0x21` | 0.0141 |
| 2 | `0x22` | 0.0172 |
| 3 | `0x23` | 0.026 |
| 8 | `0x24` | 0.0625 |
| 9 | `0x25` | 0.0141 |
| 4 to 7 | error (`0xfffff0c3`) | none |

The scales are IEEE single constants in the routine at `0xffe67b54-0xffe67c4a`; a channel with no constant there (4 to 7, or above 9) gets the value -100.0. The reading is the raw byte times the scale (the integer-to-float and multiply calls at `0xffe4be16-0xffe4be3e` are identified by position only, so this is inferred). For the supplies of the Origin 300 table in [ip45-io-expanders.md](ip45-io-expanders.md) with a nominal entry of 12, 5, 3.3 and 2.5 volts, the channel scales above times 192 give 12.0, 4.99, 3.30 and 2.50: a raw byte of `0xc0` on channels 8, 3, 2 and 0 reads as nominal on those supplies (inferred from the arithmetic). Channel 1, used by the 1.5 V record, is not consistent with a count of 192 and needs about 106 counts to read 1.5 V (inferred).

## Temperature reads (established)

Two temperature records (`0xffe45f44`, 8 bytes each: monitor index, channel, offset) both name channel 4, on monitors 0 and 1, offset 0. A type 1 monitor accepts only channel 4 for this read and maps it to register `0x27` (`0xffe69844-0xffe69948`). The byte is taken as a signed 8-bit value and the descriptor's offset is subtracted (`0xffe5e1bc-0xffe5e214`). One count is one degree Celsius under that reading; an unreadable byte gives 0 for the value and the error to the caller.

## Register sweep that reads `0x3e` and `0x3f` (established, `0xffe69a84`)

One routine reads registers of a monitor without using the values. Called with the monitor's descriptor, it reads, one byte at a time through the single-register read helper (`0xffe6cefc`), every register from `0x30` to `0x4e` inclusive and then every register from `0x98` to `0x9f` inclusive (`0xffe69a94`-`0xffe69b46`). Each byte is stored in one local byte that is overwritten on the next read and never tested. A read error stops the sweep, logs the failing register and the error through the logging routine with a source line number (`0xae0` for the first range, `0xaea` for the second) and returns the error code; a clean sweep returns 0. It is reached through the last case of the jump table at `0xffe69ba0` (`0xffe69bfa`, the call at `0xffe69bfc`) of the monitor dispatcher at `0xffe69b58`; which request selects that case was not traced.

What this settles about the part: registers `0x3e` and `0x3f` (a manufacturer-identity and a stepping register in the DS1780 and LM78-family maps, hint) are read, but only as part of this sweep and with the value thrown away. No other routine in the monitor driver (`0xffe66800` to `0xffe6a000`) loads `0x3e` or `0x3f` as a register number; the `moveq #$3e` and `moveq #$3f` instructions elsewhere in the first program copy are unrelated constants (mostly arguments to the settings getter and setter; none lies in the monitor driver). So **no image checks an identity or stepping register**, and the part is not named by any image. The sweep ranges (`0x30`-`0x4e`, `0x98`-`0x9f`) are wider than the DS1780 map as the public datasheet describes it (it ends at `0x4f`; hint, not re-checked against the sheet here), and the driver also writes limit registers at `0x0b`/`0x0c`, `0x0d`/`0x0e`, `0x39`/`0x3a` and `0x4e` to `0x53` (`0xffe691a6`-`0xffe69252`) that are not limit registers in that map (hint). Inferred: the part is DS1780-compatible for the registers the Origin 300 uses (voltage `0x20`-`0x25`, temperature `0x27`, configuration `0x40`-`0x47`) and a superset elsewhere; not settled.

## Not read

- Fan tachometer conversion: five tachometer records at `0xffe45f08` name monitor index 0, 0, 1, 1, 2 and channels 6, 7, 6, 7, 6 with the labels `LEFT`, `CENTER`, `RIGHT`, `PS`, `PS'`. Which register a tachometer reads, and the divisor use of register `0x47`, were not followed.
- Limit registers and the interrupt-status read of the monitors.
- The conversion for supply kinds other than the plain voltage entries (the table has kinds 1 to 7, for example for negative and high supplies), which the Origin 300 records do not use.
- What the third monitor answers.
