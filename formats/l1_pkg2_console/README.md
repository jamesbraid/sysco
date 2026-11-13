# l1_pkg2_console

Position of each console command in the registered command tables of one ColdFire L1 firmware package. The package registers 4 command tables. Each CMD<t>.index is the position of an entry in table t, counted from zero, and table t is the t-th table in the order of the registration calls in the image. That is not necessarily the order the parser tries them in; the order of the calls at run time is not read here. Entry layout and argument kinds are in the shared console unit. The typed pattern is the doc of each enum value; hidden entries are marked. Labels are the first alternative of each word joined by underscores, with the system-link word spelled sys_link; positions of entries that share a pattern are told apart by a numeric suffix.

Source: IRIX 6.5.30 l1.bin (L1 1.44.0), ColdFire package 2 image.

The values are in [format.toml](format.toml), each graded and sourced, and the generated reference page is [format.md](format.md).

## Used by

- [command-table.tsv](../../software/firmware/l1/command-table.tsv)
- [Console command tables of the ColdFire L1 packages](../../software/firmware/l1/console-command-tables.md)
