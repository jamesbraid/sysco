# Console command tables of the ColdFire L1 packages

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), all five ColdFire packages, with the listing of package 1 (`Fuel/PE/O300 1MB image`) in [disasm/mcf5206e-1.44.0-fuel-pe-o300/](disasm/mcf5206e-1.44.0-fuel-pe-o300/README.md) as the reference. Addresses are first-bank addresses; the second bank repeats each package at a fixed distance (see [personalities.md](personalities.md)). Grades follow [AGENTS.md](../../../AGENTS.md). The curated table is [command-table.tsv](command-table.tsv), made and checked against the image by [../../scripts/l1_command_table.py](../../../tools/l1_command_table.py); the per-package units made from it are `defs/l1_pkg<n>_console.toml` (written by [../../scripts/l1_console_defs.py](../../../tools/l1_console_defs.py)), next to the hand-kept entry layout in `formats/l1_console/format.toml`. The generated headers are `generated/c/l1_console.h` and `generated/c/l1_pkg<n>_console.h`.

Package numbers below are container indexes (0, 1, 2, 3 and 5). The script's `PACKAGE_INDEX` counts ColdFire packages only, so container 5 is index 4.

## Entry layout (established)

Package 1, parser `0xffe61bd0` and listing routines `0xffe61658` and `0xffe61944`. An entry is 68 bytes: `moveq #0x44` and a multiply by the entry number in all three, and the second-bank copy of every entry in every package differs from the first only by the bank distance (checked by the script for all 1,849 entries).

| Words | Role | Read at |
|---|---|---|
| 0 to 9 | typed pattern, ended by the first zero word; every later word of the ten is zero | parser `0xffe61cd4`-`0xffe61fd8`, renderer `0xffe617d8` |
| 10 | handler, called with the argument array, the output pointer and the caller's third argument | `0xffe61d52`-`0xffe61d60` |
| 11 to 14 | four words the handler reads through the argument array (slot 28 holds the address of word 11) | parser `0xffe61d3c`-`0xffe61d42`; handlers read `0x70(a5)` |
| 15 | help text pointer; zero lists a one-character dash | `0xffe61a30`-`0xffe61a42` |
| 16 | 1 hides the entry from the command listing | `0xffe616e0`-`0xffe616e8`, `0xffe619b0`-`0xffe619b8` |

A pattern word above 11 points at a literal whose `|`-separated alternatives are tried with a byte-exact string compare (`0xffe618d8` returns 0 on a match; it copies at most 49 characters of the literal first). A pattern word of 1 to 11 stands for an argument:

| Value | Kind | Parser | Listing text |
|---:|---|---|---|
| 1 | number, base from its prefix | `0xffe61dce` (base argument 0) | `<exp>` |
| 2, 3, 4 | number in base 10, 16, 8 | `0xffe61dce` | `<decimal exp>`, `<hex exp>`, `<octal exp>` |
| 5 | one word, kept as a string | `0xffe61e4a` | `<str>` |
| 6 | the rest of the line | `0xffe61e7a` (tokenizes with an empty delimiter) | `<command>` |
| 8 | `:<port>:<l1>`, two numbers (format `:%d:%d`) | `0xffe61eaa` | `:<port>:<l1>` |
| 11 | `<rack>.<slot>`, two numbers around one character (format `%d%c%d`) | `0xffe61f12` | `<rack>.<slot>` |
| 7, 9, 10 | not argument kinds: the parser treats them as literal pointers | `0xffe61f80` | printed as raw words |

No table in any package uses 7, 9 or 10 (the script rejects an entry that does). After the last pattern word the parser tokenizes once more with an empty delimiter and rejects the entry if any text is left, so trailing words are an error unless the pattern ends in kind 6 (`0xffe61d18`-`0xffe61d30`).

The numbers and strings the parser collects go to fixed areas of the argument array: slots 0 to 9 follow the pattern positions, numbers fill the following nine slots and strings the nine after those. This is inferred from the stores at `0xffe61e26`, `0xffe61e72` and `0xffe61f62`, and from the `help` handler reading its string argument at slot 19 (`0xffe170d2`); the other slots were not followed in handlers.

## Where the tables come from (established)

One routine (`0xffe615f4` in package 1) appends a table base and an entry count to a list of at most ten, kept at `0x300060d0` (bases), `0x300060f8` (counts) and `0x30000c30` (how many); an eleventh call prints a "command list full" message. Package 1 calls it twice: from the PCI hook installer (`0xffe115be`, base `0xffe115f0`, 13 entries) and from the command task start-up (`0xffe1b0e2`, base `0xffe1c0a0`, 295 entries, the last entry at `0xffe20eb8`). The parser (`0xffe61bd0`) walks the registered tables in order and each from its first entry; the script finds the registration calls in each package by the routine's message string and reads the base and count pushed before each call.

| Package | Label | Registration calls (table: entries) | Entries | Hidden |
|---|---|---|---:|---:|
| 0 | `Base 1MB image` | 0: 13, 1: 295 | 308 | 48 |
| 1 | `Fuel/PE/O300 1MB image` | 0: 13, 1: 295 | 308 | 48 |
| 2 | `MIPS 2MB image` | 0: 13, 1: 25, 2: 41, 3: 295 | 374 | 59 |
| 3 | `Legacy 2MB image` | 0: 294, 1: 25, 2: 114 | 433 | 64 |
| 5 | `2MB image` | 0: 297, 1: 104, 2: 25 | 426 | 66 |

Tables are numbered by the address of their registration call, not by the order the calls run (see the unknowns).

## Hidden entries (established, package 1)

`help` (handler `0xffe17080`) has four entries: `help`, `help <word>`, `help list hidden` and `help <word> hidden`. Word 11 picks the list (0) or the usage of one command (1) by the handler's switch at `0xffe17094`-`0xffe1709e`. Word 12 is 0 in the first two entries and 1 in the last two, and the handler passes it to the two listing routines as their "show hidden" argument (`0xffe170b4`, `0xffe170ce`). Both routines skip an entry whose last word is 1 unless that argument is nonzero. So the 48 hidden entries of package 1 appear only under `help list hidden` and `help <word> hidden` (which are themselves hidden entries). They still run when typed.

## Brick-type restriction (established for the call sites, inferred for the rest)

A command is restricted to some brick types by the string word 11 points at: zero allows every brick, and the check (`0xffe140d4` in packages 0 and 1, `0xffe198ac` in 2, `0xffe13848` in 3, `0xffe10ff4` in 5) is made by the handler, not by the parser. The check itself, its class table and the leading `~` are read in [env-fan-power-margin.md](env-fan-power-margin.md).

- Established, package 1: all 28 calls of the check are directly preceded by the push of the value at `0x70(aN)`, that is word 11 of the entry (`movea.l $70(aN),a0; move.l (a0),-(a7)`). All 28 callers are table handlers. 26 handlers have an entry whose word 11 points at a string (the empty string included) and all 26 call the check; the other two callers (`0xffe17df4`, the `l1dbg` family, and `0xffe19740`, `power up`) have only zero or selector entries, so for them the check receives a null set and passes. The remaining 36 handlers contain no call, so this test never restricts them.
- Established: the same count in the other packages gives 28 check callers in package 0, 36 in 2, 41 in 3 and 40 in 5 (first bank). In package 2 one handler with a string entry makes no direct call (the single `make rmws <number>` entry, set `8`), and in package 5 one does not either (the single `pci` entry of the 25-entry table); how those two entries are restricted is not read.
- Established: the 110 entries of package 1 whose word 11 is a nonzero small number (including four negative ones in the `flash` family) and the 66 with zero are not subject to a brick-set test through word 11 when the handler has no call. For the small numbers the word is a selector the handler switches on: the `env` handler (`0xffe15528`) range-checks it against 14 at `0xffe1553e` and jumps through a table.
- Established, package 1: the final two entries of the large table (`fru|pci|prom|node` and the same with `<command>`) point word 11 at an empty string. Their handler (`0xffe141b4`) runs the check on it, which can never pass, writes a fixed "not supported on this brick type" information line and returns 0. The same two entries, with the pattern `fru|pci|node`, end the large table of packages 3 and 5.

## Check of the earlier extraction

The earlier script scanned the body for a word that looked like a help pointer and printed what the shape selected. Against the registered tables of package 1 (measured by comparing the printed row addresses):

| Rows | Count |
|---|---:|
| entries in the registered tables | 308 |
| printed by the earlier scan and real entries | 196 |
| real entries not printed (110 with a nonzero selector in word 11, 2 with an empty-string set) | 112 |
| printed rows that are not entries (6 inside the tables at a wrong offset, 219 past the end of the large table, which ends at `0xffe20efc`) | 225 |

The rows the earlier scan printed for the brick-restricted commands are correct; what changes is that the selector entries (all of `env`, `fan`'s selector forms, `margin <index>`, `pbay`'s, `l1dbg`'s, among others) are now present and that rows past the tables, which are other string tables with similar spacing, are gone. The earlier notes quote their row addresses as the address of the help word (entry start plus `0x3c`); the script's line output keeps that convention, `--tsv` and the table give the entry start.

Settled by this reading: [env-fan-power-margin.md](env-fan-power-margin.md) listed the brick filter of the `env` sub-commands as unread because their entries "have a different shape". They have the same shape; word 11 is a selector from 0 to 14, and the `env` handler (`0xffe1ad00` in package 2, `0xffe15528` in package 1) contains no call to the check, and no routine reachable from it by direct calls calls the check either (297 routines followed in package 1).

## Unknown, with the test that settles each

| Unknown | Test |
|---|---|
| The order the registration calls run in on a package with more than one table. It decides which table the parser tries first, and so whether the empty-set entries `fru\|pci\|prom\|node` shadow a later `pci` table (they would, if the large table were registered first, because their last pattern word swallows the rest of the line). | The PCI hook installer's callers (`0xffe44544`, `0xffe45990` in package 1) are reached through stored function pointers (data references at `0xffe44748` and `0xffe45bbc`). Follow those two pointers to the table or hook list that invokes them and compare its position in the boot sequence with the call to the command task start-up at `0xffe10610`. Repeat for the three extra-table registrations of packages 2, 3 and 5. |
| What an indirect call from a handler could do: the check for `env` is shown absent along direct calls only. | List the function-pointer stores reachable from the `env` handler and read their targets for calls to the check. |
| How the two entries named above, in packages 2 and 5, that have a string in word 11 but a handler with no direct call, are restricted. | Read the two handlers for a routine that wraps the check, and for any compare against the brick-type byte. |
| Whether the argument-array layout beyond slot 28 and the string slot 19 is as inferred. | Read the stores in three handlers that take two numbers (`rackslot`) and one that takes a word and a number, and check which slots they read. |
| Meaning of the selectors (words 12 to 14) and of word 11 as selector, per handler. | One handler at a time: read the switch on word 11 and the compares on the later words, then add the meaning to this table. |
| 1.24.8 to 1.48.1 and the other releases: the same tables. | Run the script on the `l1.bin` of each release; the registration message string and the 68-byte layout are the only things it relies on. |

The published console units repeat only what is established above: positions, typed patterns, hidden marks, the entry layout and the argument kinds. Handler addresses, selectors and brick sets stay in the table.
