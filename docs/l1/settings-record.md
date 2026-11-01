# Origin 300 L1 1.44.0: where each persistent setting lives

The L1 keeps its settings in the battery-backed window at `0x20000000` ([flash-nvram-and-image-records.md](flash-nvram-and-image-records.md) gives the header, the three records and their mirrors). This note maps every setting number the getter accepts to the bytes it reads. A machine that hosts the image needs it for two reasons: to preload the window with chosen values (rack, bay, debug switches, brick type), and to know which setting a guest request changes when it shows up as a write to the window.

Read from L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`, composite offset `0xefdd0`), first program copy. The getter is the routine at `0xffe59590`; it dispatches on the setting number through a table of `0xb3` signed 16-bit offsets at `0xffe595b0` (setting numbers `0` to `0xb2`; a larger number reads as 0). The setter at `0xffe58758` has the same shape and its table is at `0xffe58782`.

## How a setting is read (established)

The getter loads the pointer stored at `0x300003a8` (it holds the address of the first record, `0x20000008` when the header's record-area byte is 1) and returns one of: a byte, a bit field cut from a byte, a 16-bit word, a 32-bit big-endian word, or two single bits taken from two different bytes. Offsets below are from the start of that record. Offsets `0x12c` and up fall in the third record (which starts at `+0x128`), offset `0xf4` in the second (`+0xe0`); every other offset is in the first, 0x70-byte record. Nothing is computed from the clock or any other device.

After any write the setter (tail at `0xffe594de-0xffe5958e`) recomputes the checksum byte of the first record (`+0x6f`), of the third (`+0x1db`) and of the second (`+0x103`, over the 0x24 bytes from `+0xe0`), and copies the first record to `+0x70` and the second to `+0x104`; a further call at `0xffe59560` (not decoded) is taken to copy the third record to its mirror at `+0x1dc`, which is inferred. A host that watches the window sees the primary, the checksum byte and the mirror change together.

Grades: every row is established, read from the dispatch table and the short routine each entry reaches, except where a row says otherwise. Unused numbers return 0 and are not listed (see the end of the note).

## Setting numbers

Setting numbers are decimal with the hexadecimal form in brackets. A bit field is written as `bits high:low` counted from the least significant bit of the byte or word after loading it.

| Setting | Location in the record |
|---|---|
| 1 (0x01) | 32-bit word at +0x00, bits 23:14 (rack number) |
| 2 (0x02) | byte +0x02 bits 5:0 (bay number) |
| 3 (0x03) | 32-bit word at +0x1c, bits 23:8 (debug switches) |
| 4 (0x04) | byte +0x21 bit 7 |
| 5 (0x05) | byte +0x20 |
| 6 (0x06) | byte +0x21 bit 3 |
| 7 (0x07) | byte +0x21 bit 0 |
| 8 (0x08) | byte +0x21 bits 2:1 |
| 9 (0x09) | byte +0x22 (brick type character) |
| 10 (0x0a) | byte +0x21 bits 6:5 |
| 11 (0x0b) | byte +0x23 |
| 12 (0x0c) | byte +0x21 bit 4 |
| 13 (0x0d) | returns the address record +0x8 (no value is read) (system serial number (address)) |
| 14 (0x0e) | byte +0x1f (partition) |
| 15 (0x0f) | returns the address record +0x68 (no value is read) |
| 16 (0x10) | byte +0x47 (brick sub-type) |
| 17 (0x11) | byte +0x11 |
| 18 (0x12) | byte +0x14 bits 3:2 (two-bit field) |
| 19 (0x13) | byte +0x2f bits 4:0 (event subchannel) |
| 20 (0x14) | byte +0x34 |
| 21 (0x15) | byte +0x35 |
| 22 (0x16) | byte +0x36 |
| 23 (0x17) | byte +0x37 |
| 24 (0x18) | byte +0x38 |
| 25 (0x19) | byte +0x39 |
| 26 (0x1a) | byte +0x3a |
| 27 (0x1b) | byte +0x3b |
| 28 (0x1c) | byte +0x3c |
| 29 (0x1d) | byte +0x24 |
| 30 (0x1e) | byte +0x25 |
| 31 (0x1f) | byte +0x26 |
| 32 (0x20) | byte +0x27 |
| 40 (0x28) | byte +0x46 bit 7 |
| 41 (0x29) | byte +0x46 bit 6 |
| 42 (0x2a) | two bits: bit 0 of byte +0x2e (high) and bit 5 of byte +0x46 (low) |
| 43 (0x2b) | two bits: bit 7 of byte +0x2f (high) and bit 4 of byte +0x46 (low) |
| 44 (0x2c) | byte +0x46 bit 3 |
| 45 (0x2d) | byte +0x46 bit 2 |
| 46 (0x2e) | two bits: bit 6 of byte +0x2f (high) and bit 1 of byte +0x46 (low) |
| 47 (0x2f) | two bits: bit 5 of byte +0x2f (high) and bit 0 of byte +0x46 (low) |
| 50 (0x32) | byte +0x04 bit 7 |
| 52 (0x34) | byte +0x04 bit 5 |
| 53 (0x35) | byte +0x04 bit 4 |
| 54 (0x36) | byte +0x16 bit 3 |
| 55 (0x37) | byte +0x16 bit 2 |
| 56 (0x38) | byte +0x16 bit 1 |
| 57 (0x39) | byte +0x16 bit 0 |
| 60 (0x3c) | byte +0x17 bits 7:4 |
| 61 (0x3d) | byte +0x17 bits 2:1 |
| 62 (0x3e) | byte +0x17 bit 0 |
| 63 (0x3f) | 32-bit word at +0x30 |
| 70 (0x46) | returns the address record +0x48 (no value is read) |
| 71 (0x47) | returns the address record +0x4c (no value is read) |
| 72 (0x48) | 32-bit word at +0x50 |
| 73 (0x49) | 32-bit word at +0x54 |
| 74 (0x4a) | 32-bit word at +0x58 |
| 75 (0x4b) | 32-bit word at +0x5c |
| 76 (0x4c) | 32-bit word at +0x60 |
| 77 (0x4d) | 32-bit word at +0x64 |
| 80 (0x50) | byte +0x17 bit 3 |
| 81 (0x51) | byte +0x12c |
| 82 (0x52) | 32-bit word at +0x12c, low 24 bits (mask 0xffffff) |
| 83 (0x53) | returns the address record +0x130 (no value is read) |
| 84 (0x54) | byte +0x12 |
| 85 (0x55) | byte +0x13 bits 7:4 |
| 86 (0x56) | byte +0x13 bits 3:0 |
| 87 (0x57) | byte +0x15 bits 3:0 |
| 88 (0x58) | byte +0x14 bit 7 |
| 89 (0x59) | byte +0x14 bit 6 |
| 90 (0x5a) | byte +0x14 bit 5 |
| 91 (0x5b) | byte +0x14 bit 4 |
| 92 (0x5c) | byte +0x05 bit 7 |
| 93 (0x5d) | byte +0x05 bit 6 |
| 94 (0x5e) | byte +0x05 bit 5 |
| 95 (0x5f) | byte +0x05 bit 4 |
| 96 (0x60) | byte +0x05 bit 3 |
| 97 (0x61) | byte +0x05 bit 2 |
| 98 (0x62) | byte +0x05 bit 1 |
| 99 (0x63) | byte +0x05 bit 0 |
| 100 (0x64) | byte +0x06 bit 7 |
| 101 (0x65) | byte +0x06 bit 6 |
| 102 (0x66) | byte +0x06 bit 5 |
| 103 (0x67) | byte +0x06 bit 4 |
| 104 (0x68) | returns the address record +0xf4 (no value is read) |
| 105 (0x69) | 16-bit word at +0x14, bits 9:4 |
| 106 (0x6a) | byte +0x2c bit 7 |
| 107 (0x6b) | byte +0x2c bits 6:5 |
| 108 (0x6c) | byte +0x16 bit 7 |
| 109 (0x6d) | byte +0x16 bit 6 |
| 110 (0x6e) | byte +0x16 bit 5 |
| 111 (0x6f) | byte +0x16 bit 4 |
| 112 (0x70) | byte +0x04 bits 3:0 |
| 113 (0x71) | byte +0x2c bit 4 |
| 114 (0x72) | byte +0x2c bit 3 |
| 115 (0x73) | byte +0x2c bit 2 |
| 116 (0x74) | byte +0x2c bit 1 |
| 117 (0x75) | byte +0x06 bits 3:0 |
| 118 (0x76) | byte +0x07 bits 7:4 |
| 119 (0x77) | byte +0x2c bit 0 |
| 120 (0x78) | byte +0x2d bit 7 |
| 121 (0x79) | byte +0x2d bit 6 |
| 122 (0x7a) | byte +0x2d bit 5 |
| 123 (0x7b) | byte +0x2e bits 2:1 |
| 124 (0x7c) | byte +0x2e bits 6:3 |
| 125 (0x7d) | byte +0x2d bits 4:3 |
| 126 (0x7e) | 16-bit word at +0x18, low 15 bits (mask 0x7fff) |
| 127 (0x7f) | byte +0x1c |
| 128 (0x80) | byte +0x1a bit 7 |
| 129 (0x81) | byte +0x2d bit 2 |
| 130 (0x82) | byte +0x3d |
| 131 (0x83) | byte +0x3e |
| 132 (0x84) | byte +0x3f |
| 133 (0x85) | byte +0x40 |
| 134 (0x86) | byte +0x41 |
| 135 (0x87) | byte +0x42 |
| 136 (0x88) | byte +0x43 |
| 137 (0x89) | byte +0x44 |
| 138 (0x8a) | byte +0x45 |
| 139 (0x8b) | byte +0x28 |
| 140 (0x8c) | byte +0x29 |
| 141 (0x8d) | byte +0x2a |
| 142 (0x8e) | byte +0x2b |
| 160 (0xa0) | byte +0x2d bit 1 |
| 170 (0xaa) | byte +0x129 bits 7:4 |
| 171 (0xab) | byte +0x129 bits 3:0 |
| 172 (0xac) | byte +0x12a bits 7:4 |
| 173 (0xad) | byte +0x12a bits 3:0 |
| 174 (0xae) | byte +0x12b bits 7:4 |
| 175 (0xaf) | byte +0x12b bits 3:0 |
| 176 (0xb0) | byte +0x2d bit 0 |
| 177 (0xb1) | byte +0x2e bit 7 |
| 178 (0xb2) | byte +0x1a bit 6 |

Roles in brackets come from the callers already documented ([debug-console.md](debug-console.md), [board-boot-contract.md](board-boot-contract.md), [personalities.md](personalities.md)); the other settings have no role in this note because their consumers were not read.

## Entries worth singling out

- **Brick type (setting 9).** One byte at `+0x22`. The identity routine uses it only when none of the board identification records names a brick ([personalities.md](personalities.md)), and ignores a stored `0x34` (`4`) there. A window whose byte at `+0x22` is 0 therefore gives an unknown brick on an Origin 300 whose node EEPROM cannot be read. Established (location), inferred (the effect, from the earlier reading of the identity routine).
- **Brick sub-type (setting 16, 0x10).** One byte at `+0x47`.
- **Writers of settings 9 and 0x10 (established, package 1).** The setter `0xffe58758` is called with key 9 at `0xffe5578c`, the single exit of the identity routine `0xffe55254`, so the type character chosen above is written back on every path, and at `0xffe146b6`; with key `0x10` only at `0xffe146ec`. The two calls in the `0xffe146xx` handler are not yet read in full, so what command reaches them and what it checks is unknown. The debug handler `0xffe15214` reaches the setter with key 3 and with a key from its parsed arguments (`0xffe154c0`).
- **Rack and bay (settings 1, 2).** Bits 23:14 of the word at `+0` and bits 5:0 of the byte at `+2`.
- **Debug switches (setting 3).** Bits 23:8 of the word at `+0x1c`; the debug command reads and writes them as a 16-bit value.
- **Addresses returned instead of values.** Settings 13 (serial number), 15, 70, 71, 83 and 104 hand back an address inside the record (`+8`, `+0x68`, `+0x48`, `+0x4c`, `+0x130`, `+0xf4`), so the caller reads a string or array there. Their lengths were not read.
- **Two-bit settings built from two bytes.** Settings 42, 43, 46 and 47 each combine a bit of byte `+0x2e` or `+0x2f` (the high bit of the result) with a bit of byte `+0x46` (the low bit).
- **Run of byte settings.** Settings 20 to 28 are the bytes `+0x34` to `+0x3c`, 29 to 32 the bytes `+0x24` to `+0x27`, 130 to 138 the bytes `+0x3d` to `+0x45` and 139 to 142 the bytes `+0x28` to `+0x2b`: the number is added to a per-range constant, so these are consecutive bytes, not independent fields.

## Unused numbers (established)

These setting numbers jump straight to the routine that returns 0: 0, 33 to 39, 48, 49, 51, 58, 59, 64 to 69, 78, 79, 143 to 159, 161 to 169, and every number above 178 (the dispatch is skipped when the number exceeds `0xb2`). The setter's table sends exactly the same numbers to its own do-nothing exit (compared entry by entry), so a write to one of them changes nothing.

## What this does not settle

- The second record (`+0xe0`, 0x24 bytes) is reached by only one setting here (number 104, which returns the address `+0xf4`); whatever else reads it does so outside the settings accessors.
- The consumers of most settings were not read, so a field's role is known only where another note names it.
- The setter's table was compared with the getter's for settings 1, 2, 3, 9 and 16 only; the offsets of the other writes follow the getter's by inference. Unverified.
