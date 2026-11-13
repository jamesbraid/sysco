<!-- generated from formats/l1_console/format.toml by defs-gen - do not edit, change the source -->

# entry layout and argument kinds of the L1 console command tables

The console command line of the ColdFire L1 controller firmware is table driven. Each package
registers up to ten tables, and each table is an array of 68-byte entries made of seventeen
big-endian 32-bit words. The parser tries the registered tables in registration order and
each table from its first entry, and runs the handler of the first entry whose typed pattern
matches the line.

The pattern is up to ten words and ends at the first zero word. A word above 11 points at a
literal; its alternatives are separated by a vertical bar and a typed word matches when it
equals one alternative byte for byte. A word of 1 to 11 stands for an argument of the kind
TOKEN.kind names. The values 7, 9 and 10 are not argument kinds; no table uses them.

Four words follow the handler and are passed to it as one block. Handlers that restrict a
command to some brick types read the first word as a pointer to the string of allowed
brick-type characters (zero allows every brick; a leading '~' lists the excluded types).
The other handlers read the first word, and the three after it, as small selectors.

The last word of an entry is 1 when the entry is left out of the command listing.

Which entry sits at which position in each package is in the per-package console units.

Unit `l1_console`, macro prefix `L1C`, register block `ENTRY`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Registers

| Register | Offset | Access | Width | Count | Space | Reset | Grade | Source |
|---|---|---|---|---|---|---|---|---|
| `pattern` | 0x0 | r | 32 | 10 |  | unknown | established |  |
| `handler` | 0x28 | r | 32 |  |  | unknown | established |  |
| `arg` | 0x2c | r | 32 | 4 |  | unknown | established |  |
| `help` | 0x3c | r | 32 |  |  | unknown | established |  |
| `hidden` | 0x40 | r | 32 |  |  | unknown | established |  |

### Register `pattern`

Up to ten pattern words, the handler, four selector words, the help text and the hidden flag; 68 bytes in all.

Array of 10 registers, stride 4.

### Register `handler`

### Register `arg`

Array of 4 registers, stride 4.

### Register `help`

### Register `hidden`

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `TOKEN` | 32 | established |  |

### Word `TOKEN`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 3:0 | `kind` | established | end=0, number=1, decimal=2, hex=3, octal=4, word=5, rest_of_line=6, port_l1=8, rack_slot=11 |  |  |

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `ENTRY_SIZE` | 0x44 | established |  |  |
| `ENTRY_WORDS` | 0x11 | established |  |  |
| `PATTERN_WORDS` | 0xa | established |  |  |
| `TABLES_MAX` | 0xa | established |  |  |
