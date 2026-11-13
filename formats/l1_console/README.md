# l1_console

The console command line of the ColdFire L1 controller firmware is table driven. Each package registers up to ten tables, and each table is an array of 68-byte entries made of seventeen big-endian 32-bit words. The parser tries the registered tables in registration order and each table from its first entry, and runs the handler of the first entry whose typed pattern matches the line. The pattern is up to ten words and ends at the first zero word. A word above 11 points at a literal; its alternatives are separated by a vertical bar and a typed word matches when it equals one alternative byte for byte. A word of 1 to 11 stands for an argument of the kind TOKEN.kind names. The values 7, 9 and 10 are not argument kinds; no table uses them. Four words follow the handler and are passed to it as one block. Handlers that restrict a command to some brick types read the first word as a pointer to the string of allowed brick-type characters (zero allows every brick; a leading '~' lists the excluded types). The other handlers read the first word, and the three after it, as small selectors. The last word of an entry is 1 when the entry is left out of the command listing. Which entry sits at which position in each package is in the per-package console units.

Source: IRIX 6.5.30 l1.bin (L1 1.44.0) console command parser.

The values are in [format.toml](format.toml), each graded and sourced, and the generated reference page is [format.md](format.md).

## Used by

- [command-table.tsv](../../software/firmware/l1/command-table.tsv)
- [Console command tables of the ColdFire L1 packages](../../software/firmware/l1/console-command-tables.md)
