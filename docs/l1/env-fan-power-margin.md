# L1 `env`, `fan`, `power`, `margin` and `pbay`: applicability, output, sensor tables

Read from L1 1.44.0, IRIX 6.5.30 `l1.bin` (sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), package 2 (`MIPS 2MB image`, body loaded at `0xffe00000`, first bank; the package that serves `C I P X Y N & % 6 7 8 0 V Q E 9 A R G` per [personalities.md](personalities.md)). Addresses are first-bank addresses. Package 0 (`Base 1MB image`) carries the same module literals; its tables were not compared (U). Grades follow [AGENTS.md](../../AGENTS.md). The two scripts that made the tables are [../../scripts/l1_command_table.py](../../scripts/l1_command_table.py) and [../../scripts/l1_board_tables.py](../../scripts/l1_board_tables.py). The public manual 007-3938-006 (printed pages 66 to 72 `env`, 72 `fan`, 87 to 89 `power`, 103 to 105 `margin`) gives example outputs for Altix systems; those are hints and are marked as such.

Rail, expander and tachometer labels from the image are written in lower case where they contain `aux` or `xio`; the image prints them in capitals.

## Which bricks accept which commands

A command-table entry is 17 words: ten token words, a handler, a pointer to the string of brick-type characters the command is allowed on (null: any brick), three flag words and the help text. A leading `~` in the string negates the set. The match routine is at `0xffe198ac` (E). It reads the brick-type byte through `0xffe75fc8` (a getter for the byte at `0x300008cc`), sign-extends it, and folds `&` to `N` and `Y` to `X`; `3` is not folded. The fold runs before the search, so a set's own `&` or `Y` never matches: the set has to name `N` or `X`. For types up to `Y` (`0x59`) it reads a class letter from a byte table at `0xffe267ec`, indexed by the folded type (`i` I/O and graphics, `c` compute, `r` router, `t` for `1` and `2`). The table's `&` and `Y` entries are never read, and types above `Y` have no class. The command is allowed when the type, or its class letter, is in the string. For a negated string it is allowed when either is absent, so it is refused only when both are present.

| Command | Brick types allowed | Grade |
|---|---|---|
| `env`, `env check` (and the `env on\|off`, `env temp on\|off`, `env altitude high\|low`, `env reset` family) | any: the entry has no brick string | E for `env` and `env check`; the other sub-commands' entries have a different shape and were not extracted, so their filter is U |
| `power`, `power check`, `power up`, `power up hold`, `power down` | any | E |
| `fan` (status) | all of `^ O C R I X Y P % # = N 1 2 4 5 6 7 8 9 0 Q V E A G @ B D S J K F H L` | E |
| `fan on\|off`, one fan or all | the status set without `2 4 D G J K R S` | E |
| `fan speed` status, and set to `low\|med\|high` (one fan or all) | the `fan on\|off` set plus `G J K` | E |
| `fan speed <0..31>` (one fan or all) | the `fan on\|off` set plus `J K` (no `G`) | E |
| `fan auto on\|off` (workstation automatic fan control) | `A 8 9 0 Q` | E |
| `margin` (show), `margin default\|low\|norm\|high` (all supplies) | any | E |
| `margin gbrick default\|low\|norm\|high` | `G` only | E |
| `margin <index> default\|low\|norm\|high`, `margin <index> <value>` | negated empty set: any (the `~` string is `~` alone) | E |
| `margin dimm v <volts>` | `C 1` | E |
| `margin pci norm2hi [on\|off]` | `7 8 Q E` | E |
| `pbay` and its 15 sub-commands (`version`, `dps [n]`, `dcport [n]`, `fru [n]`, `reset`, `env [on\|off]`, `init`, `serial`) | `C 3 5 6 P % # = I X Y R N & !` (every brick that can sit on a power bay; the bay is not a brick personality; the `&` and `Y` entries match only through their folded letters `N` and `X`, which are also listed) | E |
| `l1dbg env`, `l1dbg margin` | any | E |

A command refused by this test prints a short error and returns -1 (E, `0xffe19950` to `0xffe1997c`).

## What each type character is

From the identity routine ([personalities.md](personalities.md)): `C` IP35 C-brick; `R` router; `I` I-brick; `P` P-brick; `X` X-brick; `Y` X2 (XY); `#` PX; `=` IX; `%` PE; `N` N-brick; `&` a power-board name `NIPWR`; `G` the board named `KCARL1` (G-brick); `V` CG Voyager (V-brick); `0` CGN Voyager; `A` IP34 board; `6` Twister; `7` and `9` Chimera server and workstation; `4` Origin 300 node board (IP45). The mapping `R`, `D`, `-` router class and the others is also used by the L2 (see [../l2/power-queue.md](../l2/power-queue.md)).

## Output formats

All of these are format strings in the image, E; the text below is paraphrase except short column heads.

- **`env`** (`0xffe30ca4`): optional warning banner when monitoring or temperature failsafes are off; one line `Environmental monitoring is enabled` followed by one of `and running.`, `, but not running (initialization error).`, `... (configuration error).`, `... (start up error).`, `... (unknown error - 0x%02x).`, or `, but in unknown state - state: 0x%02x error: 0x%02x.`; then three tables.
  - Supplies (`0xffe3035c`): head `Description State Warning Limits Fault Limits Current`, row `%14s %10s  ` then `%2d%% %6.2f/%6.2f  %2d%% %6.2f/%6.2f  %7.3f`; an absent supply prints `%14s <not present>`; state words `Fault`, `Warning`, `Enabled`, `Wait Pwr`, `Disabled`; when setting 125 equals 2 the line `**** MONITOR-ONLY, NO BRICK SHUTDOWN ON POWER FAULTS ****` is printed before and after.
  - Fans (`0xffe3064c`): head `Description State Warning RPM Current RPM`, row `FAN %2d %8s %10s  ` then `%11d  %11d`; states `Fault`, `Warning`, `Active`, `Enabled`, `Wait Pwr`, ` Disabled`; absent: `FAN %2d %8s <not present>`; when automatic control is off a line saying automatic fan control is disabled.
  - Temperatures (`0xffe307d8`): head `Description State Advisory Temp Critical Temp Fault Temp Current Temp` plus an optional `Bias` column; row `%2d %-14s %10s  ` then either `[Autofan Control]` with a fault and a current pair, or four `%3dC/%3dF` pairs and `%4d`; a disabled sensor prints `Disabled` in the limit columns.
- **`env check`** and the power summary line: `power appears on`, `power appears off`, `power appears mixed - some supplies on, some supplies off` (`0x2b3a3` to `0x2b3c8` in the body, E).
- **`power`** (`0xffe1dd90`): head `Supply State Voltage Margin Value` (the `margin` form adds ` Index`); rows `%14.14s  %5.5s %8.3fV  `, with `N/A` for no reading and `ERROR (%d)` for a failed read, then the margin name `%7.7s` and the value (and index); an absent supply prints `<not present>`. With no supplies: `ERROR: no power supplies available.`
- **`fan`**: first `fan(s) are on.` or `fan(s) are off.`, then one line per fan: `fan %d %-8s rpm %d`, `fan %d %-8s <not present>` or `fan %d %-8s ERROR %s.`; the speed forms print `fan %d %-8s speed: %d` or `speed: ERROR: %s` (E, `0x2a907` to `0x2a982`). The names are the tachometer table's names (below); the manual's older example (`fan 0 rpm 2465 (339)`) prints a tachometer count in brackets that this image does not.
- **`pbay`** (strings `0x43bb0` to `0x43e00`): per-supply `Dps ACOK ACRANGE, a power-fail column and FAIL` rows with `Ok`, `Low`, `110Vac`, `220Vac`, `Failing`, `Not_Present`; `DcPort State` rows with `48V Off`, `48V On, ...`, `Failed`, `**read error**`; `Total current` as `%.1f Amps (+/-11%%)`; FRU rows `Dps Mfg Product Name Part Number Rev Serial Mfg Date` or `**FRU checksum error**`; `PBay FW Version %02d.%02d, Built %02d/%02d/%04d %02d:%02d:%02d`; and the states `PBay not initialized`, `busy initializing`, `busy...try again later`, `busy flashing`, `communication failure`, `state unknown`, `already initialized`, `ready`; `Power Bay env monitoring enabled|disabled`.

## Thresholds

- **Supply limits are fixed fractions of a nominal value.** The `env` row computes the warning limits as the nominal times 1 plus or minus 0.1 and the fault limits as the nominal times 1 plus or minus 0.2, using the constants `0.1` and `0.2` (IEEE double bit patterns `0x3fb999999999999a` and `0x3fc999999999999a`) and prints `10` and `20` for the percentages (E, `0xffe30504` to `0xffe305d2`). The nominal comes from the supply's type through the table below, or, for a type with a board-supplied nominal (VCPU and the DIMM supply), from the board code (the margin-value routine `0xffe6c070` has its own cases for types `0x0f`, `0x14` and `0x15`, I).
- **Supply type table** at `0xffe6eb4c` (12-byte entries indexed by type: a single-precision nominal in volts, a flag word whose high half is a small class number, and a name pointer; E, all 30 entries read). Nominal and name are in the same entry. The high half of the flag word, which I read as a class used by the analogue conversion, is given where nonzero.

| Type | Name | Nominal V | | Type | Name | Nominal V |
|---:|---|---:|---|---:|---|---:|
| 0 | 1.2V aux | 1.2 | | 15 | 2.5V | 2.5 |
| 1 | 1.5V aux | 1.5 | | 16 | 3.3V | 3.3 |
| 2 | 1.8V aux | 1.8 | | 17 | 5V | 5.0 |
| 3 | 1.85V aux | 1.85 | | 18 | -12V (class 1) | -12.0 |
| 4 | 2.5V aux | 2.5 | | 19 | 24V (class 3) | 24.0 |
| 5 | 3.3V aux | 3.3 | | 20 | VCPU | board-supplied |
| 6 | 5V aux | 5.0 | | 21 | SRAM (class 2) | 2.5 |
| 7 | 12V aux | 12.0 | | 22 | VTERM | 1.5 |
| 8 | 48V | 48.0 | | 23 | ITANIUM | 1.5 |
| 9 | 12V | 12.0 | | 24 | 1.2V (class 4) | 1.2 |
| 10 | 1.2V | 1.2 | | 25 | 3.3V (class 5) | 3.3 |
| 11 | 1.25V | 1.25 | | 26 | 5V aux (class 6) | 5.0 |
| 12 | 1.5V | 1.5 | | 27 | 5V (class 6) | 5.0 |
| 13 | 1.8V | 1.8 | | 28 | 12V (class 7) | 12.0 |
| 14 | 1.85V | 1.85 | | 29 | DIMM | 2.5 |

  Which type a board's supply record carries is not decoded (see the record-layout note below), so the mapping from a named supply to a row here is I, by the name.
- **Fan and temperature limits** are per-entry fields in the board tables (below), not constants in the code that prints them; they were not decoded (U). `env altitude high|low` selects between two sets of temperature limits (E for the existence: help texts `set high altitude > 5000'/1500m temperature limits` and `set low altitude <= 5000'/1500m temperature limits.`; the two sets' values U). `env faultsonly`-style event filtering, `env warning`, `env fault` and the temperature test commands exist (E, help strings).
- **Event codes.** Supply, fan and temperature transitions send events with code bases `0x1000`, `0x2000` and `0x3000` (ranges and the countdown codes are in [mbus-environment-task.md](mbus-environment-task.md)); the fan log lines are `FAN %d warning limit reached @ %d RPM.`, `FAN %d fault limit reached @ %d RPM.` and `FAN %d stabilized @ %d RPM.` (E).

## Sensor tables by personality

Each board's init routine hands nine tables to nine registration routines. The dispatch (E, `0xffea38f8` and `0xffe4a404`): `C` to the IP35 routine `0xffe4b468`; `R` to the router routine `0xffe692d4`; `I P X Y N &` to the shared I/O routine `0xffe4c098`, which selects one of six blocks by the type character; `G` to `0xffe78160`; `A` to `0xffe68414`; `%` to `0xffe4ddbc`; `6` Twister to `0xffe50d18`; `7` ChiServ to `0xffe5ef60`; `9` ChiWS to `0xffe616c0`. The roles of seven of the nine routines are E from the names in their tables (expanders, record routes, monitors, supplies, fan control, tachometers) and the temperature routine is I (it takes the counts a temperature table would have). The last two tables (counts in the last two columns) are not identified. Counts are the number of table entries the call passes; the names are the strings the tables point at (a supply with no name string prints the type's name from the table above, E from `0xffe6c030`).

| Personality | Expanders | Record routes | Monitors | Supplies | Fan ctl | Tachometers | Temperatures (I) | Other tables |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `C` C-brick (IP35) | 17 | 11 | 7 | 20 | 1 | 3 | 4 | 8 and 2 |
| `I` I-brick | 13 | 2 | 3 | 9 | 1 | 3 | 2 | 7 and 2 |
| `P` P-brick | 19 | 2 | 3 | 9 | 1 | 3 | 3 | 9 and 2 |
| `X` X-brick | 8 | 2 | 3 | 9 | 1 | 3 | 2 | 6 and 2 |
| `Y` X2 (XY) | 10 | 2 | 4 | 9 | 1 | 3 | 4 | 6 and 2 |
| `N` N-brick | 12 | 5 | 2 | 7 | 1 | 2 | 2 | 4 and 8 |
| `&` | 15 | 6 | 4 | 6 | 1 | 2 | 4 | 3 and 8 |
| `R` router | 8 | 2 | 1 | 5 | 1 | 2 | 1 | 2 and 8 |
| `G` | 4 | 1 | 1 | 6 | 1 | 1 | 1 | none |
| `A` | 10 | 8 | 6 | 18 | 5 | 6 | 6 | none |
| `%` PE | 36 (as read) | 4 | 4 | 9 | 1 | 3 | 4 | 7 and 6 |

(The 36 for PE and the extra rows for the later Twister and Chimera blocks are not reliable: those blocks do not follow the same call shapes, and their counts were not read. The first-listed seven rows are E; `A` and `%` are E for the counts as printed and I for the personality label.)

Names (E):

- **C-brick**: tachometers `LEFT`, `CENTER`, `RIGHT`; monitors `NODE 0`, `NODE 1`, `NODE 2`, `PIMM0 A`, `PIMM0 B`, `PIMM1 A`, `PIMM1 B`; supplies `PIMM0 VCPU`, `PIMM1 VCPU`, `DIMM`, `3.3V aux`, `3.3V L1`, `5V aux`, `12 BIAS` and, per processor module, `SRAM`, `1.5V`, `3.3V aux`, `5V aux`, `12V BIAS`; table of two `NI`, `II`.
- **I, P, X, Y**: tachometers `LEFT`, `CENTER`, `RIGHT`; monitors `POWER 0` to `POWER 2` (`POWER 3` on Y); supplies `12V BIAS`, `3.3V aux`, `5V aux` plus the type-table names; record routes `POWER`, `LOGIC` (`XLOGIC` on Y); expanders include `POWER ENABLE`, `FAN SPEED`, `POWER STATUS 1`, `POWER STATUS 2`, `PORT CONTROL`, `XBRIDGE CONTROL`, and PCI slot expanders per bus (I: bus 1 slots 1 to 4 and bus 2 slots 1 and 2; P: buses 1 to 6 with two slots each; X: an `xio CONTROL`; Y adds `48V ENABLE`, `COM CONTROL`).
- **N, `&`**: tachometers `LEFT`, `RIGHT`; monitors `POWER 0`, `POWER 1` (`&` to `POWER 3`); expanders `48V ENABLE`, `POWER ENABLE`, `POWER STATUS` and four `A` to `D` pairs `PORT CONTROL` / `XBRIDGE CNTL`; record routes `POWER` and four `NULL n` (`&` adds `MNIPWR`); the 8-entry table names `CBrick 0` to `CBrick 3` and `GFX 0` to `GFX 3`.
- **R**: tachometers `LEFT`, `RIGHT`; monitor `POWER`; expanders `POWER CONTROL`, `widget present`, `ROUTER CONTROL`, and port controls `PORT HA`, `BC`, `DE`, `FG`; the table of eight is named `A` to `H` (the eight router ports, I).
- **G**: tachometer `BLOWER`; monitor `L1`; supplies `5V aux`, `3.3V aux`; expanders `POWER STATUS`, `RESET CONTROL`, `POWER CONTROL`.
- **A**: tachometers `EXHAUST`, `HD`, `PCI`, `xio 1`, `xio 2`, `PS`; five fan-control entries; supplies `12V IO`, `5V aux`, `3.3V aux`, `PIMM 12V BIAS`, `PIMM 1.5V`, `PIMM 3.3V aux`, `PIMM 5V aux`, `xio 12V BIAS`, `xio 5V`, `xio 2.5V`, `xio 3.3V aux`; monitors `NODE 0` to `NODE 2`, `PIMM`, `ODYSSEY`, `BEDROCK`.
- **Twister (`6`), ChiServ (`7`), ChiWS (`9`)**: supply names found in their modules include `NODE0 CPU CTRL` to `NODE3 CPU CTRL`, `12V STAT 0` to `12V STAT 3`, `N0` to `N3` `3.3V aux`, `5V aux`, `12V`, `1.5V`, `12V #2`, `12V IO`, `PCI 5V aux`, `PCI 3.3V`, `PCI 2.5V`, `PCI 5V`, `xio 12V BIAS`, `xio 5V`, `xio 2.5V`, `xio 3.3V aux`, and `IP53`/`IP59` `3.3V aux`, `5V aux`, `12V`, `1.5V`; the per-brick split and the counts were not read (U).

The field layout of a supply record (24 bytes: a name pointer, a type-like byte, bus selector bytes, an expander bit number, and for analogue channels a 32-bit calibration word) was seen but not decoded; byte 0 of the stored record is not the index of the type table for the board tables as printed (U).

## Other findings used by the manual-level commands

- `margin`: states are default, low, normal, high and custom; the L2 summary counts them from a byte per brick (see the L2 note). The `margin` index column is the supply's position within its brick, restarting at 0 for each group (manual example, hint). Margining of the 3.3 V normal value can be forced to the high value (`margin pci norm2hi on|off`, types `7 8 Q E`, E).
- `fan auto`: workstation automatic fan control exists only on `A 8 9 0 Q`.
- `power up hold` is the L1 command the L2 sends before `reset`; its help text is `power up, hold reset` (E).

## Discriminating tests for a model

| Check | Expected from the image | Grade |
|---|---|---|
| `env` on a C-brick with all monitors present | 20 supply rows, 3 fan rows (`LEFT`, `CENTER`, `RIGHT`), warning limits at 90 and 110 percent and fault limits at 80 and 120 percent of each supply's nominal | E for the percentages; I for the C-brick row count |
| `fan` on `R`, `G`, `D`, `S`, `J`, `K` | status accepted on all six; `fan on\|off` refused on all six; `fan speed` status and named forms accepted on `G J K` only; numeric `fan speed` accepted on `J K` only | E |
| `margin gbrick low` on a non-`G` brick | refused by the brick filter | E |
| `margin dimm v 1.8` on `I` | refused (`C 1` only) | E |
| `power` with one supply moved beyond 10 percent | state `Warning`; beyond 20 percent `Fault` | I (state selection read as flags 2 and 3 of the supply record only) |
| `pbay` on a brick not in `C 3 5 6 P % # = I X Y R N & !` | refused | E |

## Not read, with routes

| Unknown | Route |
|---|---|
| Fan warning RPM, temperature advisory, critical and fault values for each entry | decode the fan, tachometer and temperature record layouts from the registration routines `0xffe6ae18`, `0xffe6aec4`, `0xffe7f080` and the printing code at `0xffe3064c` and `0xffe307d8` |
| What the two unnamed tables are | read `0xffe784d4` and `0xffe6f8b8` |
| Twister, Chimera, PE and A counts | read the three blocks at `0xffe50d5c`, `0xffe5efa6`, `0xffe616f8` and `0xffe4dddc`; their call shapes differ |
| Package 0, 1, 3 and 5 | repeat with those packages; routine addresses differ, so the roles table needs a per-package address map |
| `env` sub-command brick filters | extract the entries whose handler is `0xffe1ad00` and whose token word is `env` |
| Version differences | the same tables in 1.24.8, 1.32.15 and 1.48.1 |
