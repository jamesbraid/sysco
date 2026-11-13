<!-- generated from formats/mmsc_commands/format.toml by defs-gen - do not edit, change the source -->

# typed command table of the multi-module system

Typed command table of the multi-module system controller firmware, and the user authority
levels it checks. The command table is text-keyed: an operator types a command word, and the
dispatcher compares it with each table entry in table order and runs the first match. Each
CMD.index value is the position of that entry in the table, counted from zero with aliases
included. The dispatcher compares the typed word, so the position is not a code sent on any link.
The one-character help alias (table position 57) is not an identifier and has no enum value.
Some commands compare the session's user level with 499 or 999 before they act. The user levels
are fixed values in the firmware (AUTH.level); which command checks which level is in the command
notes, not here.

Unit `mmsc_commands`, macro prefix `MMSC`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `AUTH` | 32 | established |  |
| `CMD` | 32 | established |  |
| `GATE` | 32 | established |  |

### Word `AUTH`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 15:0 | `level` | established | basic=100, supervisor=500, service=1000 |  |  |

### Word `CMD`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:0 | `index` | established | auth=0, authority=1, bs=2, cecho=3, com=4, cons=5, console=6, debug=7, numracks=8, dest=9, direct=10, downloader=11, elsc=12, emsg=13, end=14, esc=15, exit=16, ffsc=17, flash=18, graphics_command=19, help=20, ipaddr=21, kill=22, log=23, mem=24, mmsc=25, mmsg=26, msc=27, nap_time=28, nop=29, options=30, pager=31, pas=32, password=33, printenv=34, pwr=35, rackid=36, rat=37, reboot_ffsc=38, reset_ffsc=39, reset_mmsc=40, reset_nvram=41, rmsg=42, scan=43, setenv=44, steal=45, partlst=46, partscan=47, partget=48, partnam=49, partclr=50, partset=51, swapcons=52, tty_reinit=53, unsetenv=54, unsteal=55, ver=56, arbiter=58, macaddr=59 |  |  |

### Word `GATE`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 15:0 | `level` | established | above_499=499, above_999=999 |  |  |
