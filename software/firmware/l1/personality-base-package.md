# Package 0 (`Base 1MB image`) against package 1 (`Fuel/PE/O300 1MB image`), routine by routine

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0). The routine pairing is [personality-diffs/routines.tsv](../../../evidence/personality-diffs/routines.tsv); the layout and the dispatch that selects each board's code are in [personalities.md](personalities.md) and [personality-board-init.md](personality-board-init.md). Addresses are first-bank body addresses; "o300" means package 1. Grades follow [AGENTS.md](../../../AGENTS.md).

## What the 41 "changed" pairs are

The pairing marks a routine pair `changed` when its bytes differ after RAM addresses are normalised. Comparing the bytes of each pair again, word by word:

Established (every differing byte was assigned to a 32-bit word that holds a flash pointer in both packages or a RAM address in both packages; no other byte differs):

- 24 pairs differ only in those words, so the code is the same instruction for instruction. Pointer deltas are the shifts of the code and string pools. Package 0 addresses: `0xffe00b28` (63,716 bytes, the vector and loader area; the one differing word is the loader table's pointer to the first-bank record, `0xffe7fd1c` in package 1 and `0xffe7fd24` in package 0), `0xffe1f310`, `0xffe2fb08`, `0xffe31f18`, `0xffe3b3fc`, `0xffe3b78e`, `0xffe49e9c`, `0xffe4c098`, `0xffe4e600`, `0xffe50b74`, `0xffe591dc`, `0xffe60cd2`, `0xffe619a0`, `0xffe61c8c`, `0xffe61f18`, `0xffe628e4`, `0xffe63174`, `0xffe63194`, `0xffe6bdfc`, `0xffe6f69c`, `0xffe6fcbc`, `0xffe71f34`, `0xffe79638`, `0xffe7cea0`.
- 6 pairs have the same length and differ in other bytes. `0xffe7a374`: four digits of the build-stamp literal (`20060717182038` in package 1, `20060717181954` in package 0). `0xffe705e8` and `0xffe71f04`: the first word of the range, with the rest pointer-only. `0xffe7f348`: a data-dominated range. `0xffe47e70` and `0xffe49e40`: the pairing matched two different routines of the same size (see below).
- 11 pairs differ in length. They are the board-specific code and are read next.

Sample of what "same" means: `0xffe2fb08` (the serial-number reassignment routine in both) calls the type accessor and the setting reader at addresses that differ only because both packages moved (`0xffe557b4` and `0xffe55b64` are the accessor in package 1 and package 0).

## Board-specific routines, read in both packages

### The two expander toggle hooks (`0xffe445e8`, `0xffe4463c` against `0xffe45084`, `0xffe450e0`)

Established. Both pairs are stored in the board descriptor at offsets `0xa4` and `0xa8` ([personality-board-init.md](personality-board-init.md)). Each pair is a fixed sequence on one I/O expander of the board:

| Routine | Package 1 (Origin 300 board) | Package 0 (the `C` board, node type with PIMM modules) | Grade |
|---|---|---|---|
| first hook | refuses with minus 2500 (`0xfffff63c`) when the byte at `0x3000009c` equals `0xff`; otherwise sets expander 2 bit mask `0x80`, waits through the task-wait helper at `0xffe40e54` with argument 10, clears the same mask; returns the first nonzero error or 0 | refuses with minus 900 (`0xfffffc7c`) when the byte at `0x30000098` is zero; otherwise the same set, wait with argument 10, clear on expander 6 | established |
| second hook | the same refusal as its first hook; clears then sets mask `0x40` on expander 2, with no wait | the same refusal as its first hook; clears then sets mask `0x40` on expander 6, with no wait | established |

The refusal test is the one real difference besides the expander index. Package 1 tests a byte for the value `0xff`; package 0 tests a flag for nonzero. What the flag byte records is not known.

### The I2C bus-select routine (`0xffe452d0` in package 0)

Established. Package 0 registers this routine, 296 bytes, as the bus-select hook (its device registration pushes its address before calling the hook setter). It takes a bus number in argument 0, finds the expander record and state record at index 0, maps the bus number through a 14-entry table (bus 0 to 13) to the byte to write, and writes it with mask `0xff` unless the state byte already equals it. Values by bus: 0 writes nothing and returns 0; 1 writes `0xff` and clears the state byte at +1 first; 2 `0x78`; 3 `0xb8`; 4 `0xd8`; 5 `0xe8`; 6 to 13 `0xf0` to `0xf7` in order. A bus above 13 logs "Illegal IP35 brick I2C bus select: %d" (source literal the controller firmware source unit 12, line `0x7f6`) and writes nothing new.

Package 1's helper at `0xffe457fc` (200 bytes) uses record index 9 and accepts five buses: 1 writes `0xff` (and clears the state byte at +1), 6 `0xf0`, 7 `0xf1`, 8 `0xf2`, 9 `0xf3`; any other bus logs "Illegal IP45 brick I2C bus select: %d" (the controller firmware source unit 14, line `0x592`). The hook package 1 registers is the six-entry routine at `0xffe458c4`, which calls that helper; its body was not read past the jump table and the DIMM-multiplexer deselect error text ("Error deselecting the DIMM I2C multiplexer, %s", the controller firmware source unit 14 line `0x5cb`).

### The device registration routine (`0xffe453f8` against `0xffe45990`)

Established for the call sequence (read in full in both). Both call the same families of registration routines with a table address as first argument and an entry count as second. Package 0 (the `C` board) passes counts as follows, package 1 in brackets: expander table `0x11` (same); EEPROM records `0xb` (6); family at `0xffe67bb8` 7 (3); family at `0xffe4b9f4` `0x14` with two null arguments (`0xa` with three pointers); family at `0xffe4a9b4` 1 (1); family at `0xffe4aa60` 3 (5); `0xffe4f454` 2 (2); `0xffe51268` `0x10` (`0x10`). Package 0 adds a call of `0xffe5e3bc` with 4 (it begins with the log text "tempInit() called twice") and `0xffe58070` with 8. Package 1 has three calls the pairing does not match to a call in package 0's routine: `0xffe6a190` with 1 (second in its sequence), `0xffe5e074` with 2 and `0xffe57644` with 6.

Near the end the sequence diverges. Package 1 calls `ip45_eeprom_read`, then the routine at `0xffe4a48c`, the bridge initialiser with 1, the routine at `0xffe46cf0` with five arguments including the constant `0x42af0000`, and `ip45_init` with `0xa`. Package 0 calls `0xffe4482c` (it logs "Error reading PIMM PSC status: %s", source literal the controller firmware source unit 12), `0xffe4a83c`, and `0xffe44ae0` (it logs "IP35_Init() called twice"). Both then call the same two final register routines (`0xffe6c15c`/`0xffe6c0d4` in package 0, `0xffe6c00c`/`0xffe6bf84` in package 1, with 15 and 6 as the last argument against 15 and 1) and the routine at `0xffe544c8` (`0xffe54118`). Inferred only: that the routines in the same position serve the same stage; their roles were not read.

### Pairs that are not the same routine

Established. `0xffe47e70` returns the byte at `0x300000a4`; package 1's `0xffe45130` calls `0xffe4709c` and returns 0. The matcher paired them by size. `0xffe49e40` and `0xffe49e5c` are the front and the remainder of package 0's blank-board initialiser. It has the same structure as package 1's `0xffe4a1fc` (register one expander table, a call with 0, register two EEPROM records, then call one routine for each of three indexes 0 to 2, then one final call); the pairing split it across two ranges, so it shows as two changed pairs.

### Pairs that differ only by where the range starts

Established (aligned byte comparison, differing bytes classified as above):

- `0xffe6ae2e` (130 bytes) is the second half of package 0's EEPROM table initialiser, whose first half is the added 78-byte routine at `0xffe6ade0`. Together the 208 bytes against package 1's `0xffe6ac90` (208 bytes) differ in 12 flash-pointer words and one RAM word (the state block is at `0x3000063c` in package 0 and `0x30000668` in package 1) and nowhere else. It checks for a second initialisation (log text "eepromInit() called twice", source literal `eeprom.c`, line `0xd4`), allocates a presence array and a second array with `0x78` as the second argument of the allocator (the "Out of memory" texts at lines `0xdb` and `0xe4`), then stores the table address at +8 and the count at +0.
- `0xffe47d1a` (194 bytes) is the tail of the loop that reads two expander status bytes per entry and logs "Error reading XBridge control IOExp status" (line `0x75`) and "Error reading Port control IOExp status" (line `0x78`, source literal the controller firmware source unit 38) in both packages. Compared against package 1 from `0xffe4624a` it differs in 10 flash-pointer words and in the first word, because the package 0 range begins in the middle of an instruction; the loop step is the same (`0x18` bytes per entry, entry count in the byte at +1 of the block).
- `0xffe7f9a4` (1,628 bytes) sits 4 bytes above package 1's `0xffe7f9a0` (1,632 bytes); both start with the same `link` instruction. Over the first 860 bytes the two differ only in 4 flash-pointer words and 3 RAM words. The end of the range reaches the first-bank record (at `0xffe7fd24` in package 0 and `0xffe7fd1c` in package 1) and the words before it, which differ by design (the record words are in [personalities.md](personalities.md)).

### Pairs not read to the end

Inferred only, from the strings and calls each range contains. `0xffe4a234` is package 0's device-registration routine for the router board (the descriptor offset `0x04` of the `R` initialiser); it makes 15 calls, including two of the router initialiser at `0xffe49f40` (log text "NL3 router code already initialized!", source literal the controller firmware source unit 24), where package 1's `A`-board registration `0xffe49730` calls routines of the controller firmware source unit 1 module. `0xffe4a060` (340 bytes) is called four times, all from the router hook at `0xffe4a1b4`; it reads and writes persistent settings and updates expander bits with waits. `0xffe7f110` and `0xffe7f348` sit in a data-dominated region (jump tables and literals); their differences are mostly in the data words.

## The node-board code package 0 adds

Established for the module names, from the literals each routine references (see the module table in [personalities.md](personalities.md)): the controller firmware source unit 12 routines are the ones that read the PIMM status and set default voltage margins (`0xffe446cc` to `0xffe45504`), the controller firmware source unit 11 routines (`0xffe45ec4`, `0xffe45fa4`, `0xffe46028`, `0xffe4676c`) are the I/O-brick bus selects and board initialiser for `&INPXY`, the controller firmware source unit 24 (`0xffe49f40`, `0xffe4a330`) is the router `R`, and the controller firmware source unit 18 (`0xffe579f4` to `0xffe57fda`) is `G`. Package 1 has instead the eight the controller firmware source unit 1 routines (`0xffe48394` to `0xffe49638`), `ip45_init` and `ip45_eeprom_read`, a `temp.c` routine, the controller firmware source unit 46 and its own dispatcher: 116 package 1 routines have no package 0 partner.
