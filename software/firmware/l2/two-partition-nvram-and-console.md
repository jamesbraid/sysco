# Two partitions at once: what to preload, what each controller refuses, how the console is chosen

Written 2026-11-13. Re-reads section 13 of [l1-conversation.md](l1-conversation.md) against the shipped bytes and turns it into rules. L1 facts are from L1 1.44.0 (`l1.bin` of IRIX 6.5.30, sha256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`), package 1 (`Fuel/PE/O300 1MB`) as the reference, with packages 0 and 2 compared for the routines named (normalized instruction streams equal over `0xffe31300` to `0xffe34400` for package 0; package 2 equal at a fixed offset). L2 facts are from L2 1.44.0 (`bin/l2`, `lib/libmain`, `lib/libsys`, `lib/libpart`, `l2part`; `l2.bin` sha256 `6bcfda07a24cb4d56850922dab17a98223abcd14221d307e3a8c3778434e8761`). Addresses are first-bank addresses in L1 package 1 and virtual addresses in the named L2 file. Grades: established (whole routine read), inferred (reasoning stated), unknown (routes at the end). Each carried row says "re-read: confirmed", "re-read: corrected" or "not re-read".

## The rules, then the evidence

### What to preload, and where

| Store | Preload | Why | Grade |
|---|---|---|---|
| L1 settings record, setting 14 (byte `+0x1f`), on **every compute brick** | the partition number: 1 on the bricks of partition 1, 2 on those of partition 2. 0 means unpartitioned | the L1 copies it into its module record at start-up and reports it to the L2 as the record's partition word | established |
| the same record's mirror and checksum bytes | consistent with the primary (the setter recomputes both; a preloaded window must already carry valid values) | [../l1/settings-record.md](../l1/settings-record.md) | earlier grade |
| L1 setting 14 on a **router** (types `R`, `D`) | nothing: it is forced to 0 at every start-up | start-up clears the module word and the setting | established |
| L1 setting 14 on an **I/O brick** | leave it 0 or equal to the attached compute brick's partition; it is overwritten from the attached compute brick's record when that is applied | the I/O brick adopts its partition from the attached compute brick | established (the adoption arm); inferred (that an attached brick's record is applied before a console command is used) |
| L1 setting 18 (partition-database flag, byte `+0x14` bits 3:2) | 0 if the L1's own setting is to be the authority; 1 only if the boot firmware is to ask the L2 | it is advice to the boot firmware; no L1 routine enforces it | established |
| L1 settings 1 and 2 (rack, bay) | nonzero on every compute brick | a brick with rack 0 or bay 0 never volunteers as console owner and is skipped by the L2's selection | established |
| L2 NVRAM (flash `0xfe3f0000`) | nothing for partitions: no key names a partition or a console | keys 1 to 26 only | established for the range; the key roles not re-read |
| L2 partition database (flash `0xfe3e0000`, 64 KiB) | one layout that lists every compute brick (or its rack, with the rack-wide flag) with its partition byte, a mask with the bits of partitions 1 and 2, the mode that selects the layout; the header's mode word chooses the active mode | an unlisted brick answers partition 0 | established |

### What each controller refuses

| Controller | Refuses | Does not refuse |
|---|---|---|
| L1 console `brick partition none` and `brick partition <number>` (the two rows; there is no row for a bare `brick partition`, section "The `brick partition` console command") | on a router (`R`, `D` and the `1`/`2` pseudo-types over them) and on the I/O types `I X Y P G N & # = E V`: prints a refusal. **The setting has already been zeroed by then**, so a refused command on such a brick leaves setting 14 at 0 | any number on a compute brick: there is no compare in the arm; the value goes straight to the setter |
| L1 message `0x1006` (set partition) | fewer than one argument (error -100) | the brick type, setting 18 and the value: none is tested |
| L1 console `select` for a named rack and slot | a brick that is not of type `C` | |
| L1 console owner change from a peer notice or from the subchannel message `0x1002` | an announcer whose partition word differs from the current owner's | the local volunteer message and a local rack-and-slot select, which do not compare |
| L1 start-up | a stored partition of 255 or more: replaced by 0 and written back | 1 to 254 |
| L2 `p <list>` / `partition <list>` | a number of 64 or more, an empty item, a dangling `-`, a descending run | **0**: accepted, selects bricks whose partition word is 0; `*` and `all` |
| L2 partition database | a wrong magic, checksum, or version | a brick count above 256 or a partition above 63: not checked by the loader as far as read (not re-read in this note) |
| L2 console selection | records whose rack or slot is 0; non-compute types | partition numbers above 63 (the arrays are far larger; see below) |

### Partition 0 and partition 64 or more, controller by controller

| Value | L1 | L2 |
|---|---|---|
| 0 | the default. Stored and reported. All bricks left at 0 compare equal, so they share one console route and `p 0` selects them all | `p 0` is accepted and selects every non-router brick reporting 0 (including unlisted ones). A brick unlisted in the database is reported to the PROM as 0, never -1 |
| 1 to 63 | stored and reported | addressable by `p`; one database environment each |
| 64 to 254 | stored (the `brick partition` arm and `0x1006` do not compare), reported to the L2 | `p` refuses them; the database record has room for 64 partitions only; the console arrays are indexed by the partition word without a compare |
| 255 or more | accepted at runtime in the module word, replaced by 0 at the next start-up | not reachable except through a 255 held until the next boot |

A two-partition system therefore needs partition numbers in 1 to 63 (or 0 for the unpartitioned remainder) so that every controller can name them.

### The console rules

1. **Rack or bay 0 never volunteers.** The volunteer routine returns 0 without touching the destination when either argument is 0. Established.
2. **Retargeting is within one partition.** A peer notice moves the destination only when the announcing record and the current owner's record have equal partition words; the subchannel message does too. The local volunteer message does not compare. Established for the three compares.
3. **Equal numbers share a route.** The comparison is on the numbers only, so bricks of partition 0 match each other and a peer of another system with the same number would. Inferred from the compares.
4. **The L2 picks one console per partition from the reported records** (`L2CfgMGetConsole`): compute types only, rack and slot nonzero, valid byte set; it keeps two candidates per partition and replaces one when the new brick's rack is at most the stored rack **and** its slot is at most the stored slot (two independent compares: a lower rack with a higher slot is rejected). The preferred candidate also needs the subchannel helper to return 4. Established for the compares; the helper's meaning inferred.

So each partition needs at least one compute brick with nonzero rack and slot whose L1 reported it; the console is the lowest-rack, lowest-slot candidate that passes both compares, and a partition whose bricks sit at (low rack, high slot) and (high rack, low slot) pairs may select neither.

## The `brick partition` console command (re-read 2026-11-13, L1 1.44.0, packages 0 and 2)

Table rows (package 0 `0xffe1c5ac` and `0xffe1c5f0`; package 2 the same rows in its second table), one handler (package 0 `0xffe1445c` case at `0xffe14816`; package 2 `0xffe19c34`, instruction streams identical with address operands masked, 537 instructions). Grades: established unless marked.

- **Forms.** `brick partition none` (aliases `part`, `p`; help text "clear brick partition") and `brick partition <number>` (alias `part` only; `p` is not an alias of the numbered row; help text "set brick partition"). The number is any value the number parser takes (decimal, `0x`, octal prefixes). A bare `brick partition` matches neither row: the parser walks past both (a literal `none` or a number is missing; text left over also rejects the shorter `brick` row), nothing runs, and the console prints `ERROR: invalid arguments for 'brick' command, try "help brick"` (first matching word is echoed twice; the no-match text is `ERROR: command not found.`). So the form "with no argument" that zeroes the setting is `brick partition none`; the handler's work is the same for both rows up to the point described next. The first-word echo rule is read from the parser's tail (package 0 `0xffe6232c` to `0xffe62378`); the exact trigger for the first branch is inferred from the stores to its saved pointer.
- **Order of effects.** (1) Setting 14 is set to 0 unconditionally, before any test. (2) The reply buffer is loaded with `brick partition cleared.`. (3) A router (`R`, `D`, and the pseudo-types `1`, `2` when the real type is `R` or `D`) or an I/O brick type (`I X Y P G N & # = E V`) makes the handler copy a refusal over the reply and return; the setting stays 0, and nothing is applied. (4) Otherwise, if setting 18 is nonzero the reply is replaced by the warning text. (5) Only the numbered row then stores the typed value into setting 14 (`0xffe14a26`) and appends the confirmation. (6) The apply routine (`0xffe332c8`) runs for both rows with the value read back from the setting.
- **Chimera types `7`, `8`, `9`, `Q` (re-read 2026-11-14, E).** The type tests in the handler compare the brick type byte with `R`, `D`, then (for the pseudo-types `1`, `2`) the pseudo-type byte with `R`, `D`, then with `I X Y P G N & # = E V`, in that order (comparison constants `0x52 0x44`, `0x31 0x32`, `0x52 0x44` for the pseudo-type; `0x49 0x58 0x59 0x50 0x47 0x4e 0x26 0x23 0x3d 0x45 0x56` twice, for the type byte and for the pseudo-type byte; same sequence in package 2 `0xffe19fee` region as package 0/1 `0xffe14816`, compared by constants). `7`, `8`, `9` (`0x37 0x38 0x39`) and `Q` (`0x51`) are in none of the lists, so `brick partition` runs on them as on a `C` brick: no refusal, setting 14 stored, the apply routine run. The numbered row's `~A` guard excludes only type `A`.
- **Texts (all with a trailing newline).** `brick partition cleared.` (none row, compute type, setting 18 clear); `WARNING: brick appears to get partition from L2.` (setting 18 nonzero; this copy overwrites the `cleared` text, so with the flag set the `none` row prints only the warning, and the numbered row prints the warning followed by the confirmation); `brick partition set to %d.` (numbered row; the number printed is the setting read back, not the typed text); `INFO: can't set partition on routers.`; `INFO: can't set partition on I/O brick (it is acquired from attached C-brick)`. The refusal overwrites the buffer, so a refused `none` prints only the refusal even though the setting was zeroed.
- **The bare `none` form on a refused brick still clears.** Confirmed: the zeroing (step 1) comes before the type tests, and no later step restores the old value. An I/O brick that later receives its attached compute brick's record adopts that brick's value (above).
- **When the new value is acted on.** At once, in the L1: the setter writes the battery-backed settings record immediately; the apply routine, when the value differs from record 0's word `+0x3c`, stores it there, runs `0xffe30acc`, publishes a notice of kind 6, and for the 17 compute types queues four configuration messages (kinds 1, 3, 14, 15) to its peers. Nothing in the handler waits for a restart or reboots. The published manuals say the change takes effect after the next L1 reboot or power cycle. Both can hold: the L1 stores and propagates at once, while the consumers the L1 cannot see (the boot firmware and the L2's partition lookup) are not read here, so whether they act before a restart is unknown. A reboot also clamps a stored value of 255 or more to 0 (above).
- **Number range.** The numbered row has no range test: any 32-bit typed value goes to the setter; the setter keeps only what setting 14's byte holds (a byte at record offset `+0x1f`), so a larger value is truncated by the setter (inferred from the byte-wide record offset; the setter's entry was not decoded) while the confirmation prints the value read back.

## 1. Where the partition number lives (re-read)

| Store | Re-read | Grade |
|---|---|---|
| L1 settings record, setting 14 = byte `+0x1f` | **confirmed.** Getter `0xffe59590`, jump table at `0xffe595b0`: entry 14 is `0xffe59740` (`moveq #0x1c, d7; bra 0xffe59820`), which reads the byte at record base + `d7` + 3 = `+0x1f`, base from the word at `0x300003a8` | established |
| module record 0, word `+0x3c` | **confirmed.** Start-up (`0xffe3144e` to `0xffe31472`): getter(14) stored into `+0x3c`; a value of `0xff` or more is replaced by 0 and written back through the setter. Router types (`R` 0x52, `D` 0x44) clear the word and call the setter with 0 (`0xffe31560` to `0xffe31590`) | established |
| module records 1 and up | **confirmed.** The partition notice handler (`0xffe33944`) finds a record by rack and bay and stores the word into `+0x3c`; it does nothing when none matches (`tst.l a5 / beq`, `0xffe339cc`); nothing is written to the settings record | established |
| L2 NVRAM keys | range **confirmed**: `nvramGet` accepts key - 1 up to `0x19`, so keys 1 to 26 (libmain `0xd070` to `0xd078`); a key that names a partition or console was not looked for again | established (range); earlier grade (roles) |
| L2 partition database | lookup **confirmed** (libpart `0x5374`); record size arithmetic `0x430 + 64 x 0x208 = 0x8630` and the room `0x30 + 256 x 4 = 0x430` check; the loader's missing bound checks not re-read | established (lookup); earlier grade (loader) |

## 2. Who writes setting 14, and what a change does (re-read)

- **Writers.** `0x1006` (`0xffe12f20`): needs the code plus one argument (`move.b 0x20(a3)`, `subq #2`, `bge`), sets setting 14 to the value (setter `0xffe58758`), reads it back and calls the apply routine; no other test. Console `brick partition` (case at `0xffe14816`): first `setter(14, 0)`, then the router and I/O type tests with refusals, then the setting-18 warning, then the typed value if there is one (`0xffe14a26`) and the apply routine (`0xffe14a60`). Start-up clamp and router clear (above). The adoption arm (below). **Confirmed all five;** none compares the value with a limit except the start-up clamp.
- **Apply routine `0xffe332c8`.** Confirmed: differing value is stored into record 0's `+0x3c`, `0xffe30acc` is called, a notice of kind 6 (`0xffe33060`) is published, and for the 17 compute types `C 3 5 4 A 6 9 7 8 0 Q @ O B F H L` (compare chain `0xffe332fe` to `0xffe33394`) further sends follow. The kinds 1, 3, 14 and 15 of those sends and what they carry: **not re-read**. An unchanged value does nothing (`beq 0xffe333c8`).
- **Adoption.** Confirmed: in the record-apply routine (`0xffe31f18`), the arm at `0xffe32412` copies the incoming record; for a compute type it sends its own two configuration messages and returns (`0xffe3251c`); for any other type (`0xffe32552`) it copies the incoming record's `+0x3c` into its own, writes setting 14, logs the line `New C-brick partition ID detected` and calls `0xffe30acc` when the two differ (`0xffe3259c` to `0xffe325dc`). **U12 of the earlier note is resolved:** the adoption arm is reached only for apply indices 1 to 4 (entries 1 to 4 of the second jump table, `0xffe32212`, all go to `0xffe32412`; the first table, `0xffe31f48`, sends those indices through the clear arm at `0xffe321a0` into the same tail), and indices 1 to 4 come from frames on ports 9 to 12 (the receive handler's mapping, [../l1/module-table-capacity.md](../l1/module-table-capacity.md)). So an I/O brick adopts the partition of the compute brick on the port the record came from (port 9: record 1, port 10: record 2), and when two ports disagree it holds whichever was applied last (inferred from the unconditional copy).

## 3. Setting 18 (re-read)

Confirmed: the getter's entry 18 reads `+0x14`, shifts right 2 and masks 3 (`0xffe59746` to `0xffe59758`); the `partdb` handler (`0xffe19588`) writes 1 for the enable sub-command and 0 for disable (`0xffe195d2`, `0xffe195d6`, setter at `0xffe195de`) and prints one of two texts after reading the setting back; `brick partition` reads it only to print a warning and then writes (`0xffe149e4` to `0xffe14a18`); `0x1006` does not read it. The third reader (`0xffe128de`, the read message `0x000b`) was **not re-read**. So the flag is advice, not a lock. Established. Whether the boot firmware reads it before asking the L2: unknown (U16).

## 4. The console in the L1 (re-read)

- **Destination word** (RAM block `0x30005140`, word `+0x9f4`): built as rack bits 27 to 18, bay bits 17 to 12, task 5 in the low bits, subchannel in bits 11 to 5 (`0xffe368a0` to `0xffe368cc`). **Confirmed.** Subchannels 0 to 3 are CPUs a to d, 4 is the console (carried; the sub-command table was not re-read).
- **Rack or bay 0 never volunteers.** Confirmed at `0xffe36948` to `0xffe36968` (`tst.l d6 / beq`, `tst.l d7 / bne`, else `moveq #0, d0` and return).
- **The subchannel message** (`0xffe3681c`): stores the subchannel into record 0's `+0x14`, finds the record of the current owner by its rack and bay (`0xffe32ca8`), and rebuilds the word from this brick's own rack and bay only when the owner exists and its `+0x3c` equals this brick's (`0xffe36890` to `0xffe36898`). **Confirmed.**
- **Peer notices** (`0xffe334a0`, `0xffe3360c`): the equality compares were re-read (`0xffe33586` to `0xffe3358a`, `0xffe336a8` to `0xffe336ac`); the rest of each routine not re-read.

## 5. The console in the L2 (re-read of the comparison)

`L2CfgMGetConsole` (libmain `0x180e4`): the pair of compares re-read at `0x18484` to `0x184a8`: the new brick's rack (r26) against the stored minimum rack with `cmplw / bgt`, its slot (r25) against the stored minimum slot with `cmplw / bgt`, and the extra test `cmpwi r23, 4` for the preferred array; on acceptance both minima and the console word are stored (rack bits 27 to 18, slot bits 17 to 12, then the subchannel from the record's word at `+8` in the low 7 bits). **Confirmed** as a non-lexicographic minimum. The arrays are three data words per partition at 0x800-byte spacing (`0x36214`, `0x36a14`, `0x37214` in libmain), which gives room for at least 512 partitions each (inferred from the spacing). The type chain (`0x1839c` to `0x1844c`), the helper at `0x18b00`, the cache and the operator strings: **not re-read**; grades as in the earlier note.

## Corrections and a note

- [l1-conversation.md](l1-conversation.md) section 13.3 says the `p` parser cannot address partition 0. **Wrong:** `p 0` is accepted and selects the bricks whose partition word is 0 (libsys `0xd0c0` to `0xd0d0`; see [routed-shutdown-and-partition-fetch.md](routed-shutdown-and-partition-fetch.md)). What stays true is that a brick unlisted in the database is indistinguishable from a brick deliberately in partition 0.
- Section 13.2's order of events in `brick partition` is confirmed; the typed value is not range-checked in that arm.
- The kernel half of a routed `shutdown -p` reads the partition from the local L1, not from the L2 ([routed-shutdown-and-partition-fetch.md](routed-shutdown-and-partition-fetch.md)), so setting 14 is the value the shutdown prefix uses.

## Unknowns

| Question | Route |
|---|---|
| What the four sends after a partition change carry | read `0xffe31b10` kinds 1, 3, 14, 15 |
| Whether a value above 255 is truncated by the setter or kept | read the setter's entry for setting 14 (table at `0xffe58782`) |
| Whether any L2 NVRAM key is read as a console selection | follow the readers of keys 11 to 14, 20, 23, 24 |
| Whether the boot firmware reads setting 18 | find the `0x000b` message in the boot payload |
