# mmsc_commands

Typed command table of the multi-module system controller firmware, and the user authority levels it checks. The command table is text-keyed: an operator types a command word, and the dispatcher compares it with each table entry in table order and runs the first match. Each CMD.index value is the position of that entry in the table, counted from zero with aliases included. The dispatcher compares the typed word, so the position is not a code sent on any link. The one-character help alias (table position 57) is not an identifier and has no enum value. Some commands compare the session's user level with 499 or 999 before they act. The user levels are fixed values in the firmware (AUTH.level); which command checks which level is in the command notes, not here.

Source: IRIX 6.5.30 mmscfw.bin (MMSC 2.3), command table.

The values are in [format.toml](format.toml), each graded and sourced, and the generated reference page is [format.md](format.md).

## Used by

- [MMSC command catalogue](../../software/firmware/mmsc/commands.md)
- [commands.tsv](../../software/firmware/mmsc/commands.tsv)
- [MMSC load map and symbols](../../software/firmware/mmsc/load-map.md)
