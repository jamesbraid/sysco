# MMSC console modes and the system-input dispatch

Read from the MMSC images `usr/cpu/firmware/mmscfw.bin` of IRIX 6.5.30 (MMSC 2.3; addresses below) and, for the dispatch table, of every other image held: 6.4, 6.4-origin, 6.5, 6.5.2, 6.5.5, 6.5.6, 6.5.7, 6.5.9, 6.5.10 and patch SG0002378. Grades follow [../../AGENTS.md](../../AGENTS.md).

## The mode word

Each console record has a mode word at offset `+0x18`. It is 0 when the console is in no special mode (stored by `_consCreate` and by every `_clLeave*Mode` routine) and is set by the matching `_ceEnter*Mode` routine. Established, one store per routine (names are the image's own symbols):

| Mode | Enter routine | Leave routine |
|---:|---|---|
| 1 | `_ceEnterConsoleMode` | `_clLeaveConsoleMode` |
| 2 | `_ceEnterFFSCMode` | `_clLeaveFFSCMode` |
| 3 | `_ceEnterELSCMode` | `_clLeaveELSCMode` |
| 4 | `_ceEnterCopyUserMode` | `_clLeaveCopyUserMode` |
| 5 | `_ceEnterCopySysMode` | `_clLeaveCopySysMode` |
| 6 | `_ceEnterFlashMode` | `_clLeaveFlashMode` |
| 7 | `_ceEnterModemMode` | `_clLeaveModemMode` |
| 8 | `_ceEnterIdleMode` | none found |
| 9 | `_ceEnterWatchSysMode` | `_clLeaveWatchSysMode` |
| 10 | `_ceEnterRATMode` | `_clLeaveRATMode` |
| 11 | `_ceEnterPagerMode` | `_clLeavePagerMode` |

## `_csSysIn` (6.5.6 `0x11f8c8`, 6.5.30 `0x11f8d8`)

Input that arrives on a console's system-side port is handed to `_csSysIn`, whose only caller is the console task. Established, whole routine read in every image listed above: it loads the mode word, subtracts one and, if the result is above 8 (unsigned), takes the default path; otherwise it jumps through a nine-entry table of absolute addresses. The table holds the same pattern in all eleven images, and each entry either calls one of three handlers with the console record and returns its result, or is the default path:

| Mode | Handler |
|---:|---|
| 1 (console) and 9 (watch system) | `_csSysInWatchOOB` |
| 3 (ELSC) | `_csSysInELSC` |
| 4 (copy user), 5 (copy system), 7 (modem) | `_csSysInEcho` |
| 2 (FFSC), 6 (flash), 8 (idle), and 0, 10, 11 or anything else | default |

The default path logs, through `_ffscMsg`, the text "csSysIn called for %s while in mode %d" with the console's name and the mode, and returns -1. So system-side input is never expected in FFSC, flash, idle, RAT or pager mode, nor while the console is in no mode. Established for the code and the string; inferred that RAT and pager mode (10, 11) fall to the default because they take the console's input by another route, which is not read here.

The jump that earlier analysis could not resolve is therefore resolved: its destinations are the three handlers and the default path above, and the nine table entries are the only destinations the instruction can reach. `_cuUserIn`, the user-side counterpart (`0x120128` in 6.5.6), uses the same mode word and an eleven-entry table; it is not read here.
