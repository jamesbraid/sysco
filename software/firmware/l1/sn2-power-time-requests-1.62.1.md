# What the L1 does for the node's text commands, time-of-day requests and event clear (L1 1.62.1)

The node side is in the chips repository, `software/sal/runtime-services.md` section 6 and `software/firmware/snprom/l1-requests.md` ("Requests after boot"). This page reads the ColdFire L1 `O` (package 3, `2MB image`, string `O5#=?`, SHA-256 `14b7122d...a66e`; Ghidra `68000:BE:32:Coldfire`, flat load `0xffe00000`, 44-byte header skipped) and, for the command table, the ARM Linux L1 program (SHA-256 `22cb6287...`, extracted from the CramFS of package 4, flat load `0x8000`). Grades follow [AGENTS.md](../../../AGENTS.md). No emulator ran.

## Time of day

**`0x200e`, get (established, ColdFire package 3).** A block of the general-task dispatcher `0xffe3f9d4`. It sets status 0, calls the clock reader `0xffe7ba9c` (seconds since 1970, or -1), converts it with `0xffe7fa80` (UTC; no zone is applied) and appends six integers in this order: year (`tm_year + 1900`, four digits), month (`tm_mon + 1`), day, hour, minute, second. No arguments are read. Request with any argument count works the same.

**`0xffe7ba9c` source (established).** It reads a BCD timekeeper in the L1's own address space at `0x2000_07f8` to `0x2000_07ff`: seconds `+1`, minutes `+2`, hours `+3`, date `+5`, month `+6`, year `+7`, with the century digits in the top bits of `+0` (control byte; the code ORs in bit `0x40` before the digit read and clears it again). It returns -1 when every field is zero or the year is outside 1930 to 2492. The handler does not test for -1, so a dead or unset clock gives a reply from `gmtime(-1)` (**inferred** from the missing test; the reply would be 1969-12-31 23:59:59).

**`0x200d`, set (established, ColdFire package 3).** Needs seven words including the code (`-100` otherwise). Arguments 1 to 6 are year, month, day, hour, minute, second. The handler formats `(date -u %02d%02d%02d%02d%04d.%02d; hwclock -u --systohc) >/dev/null 2>/dev/null` with month mod 13, day and `0x1f`, hour mod 24, minute mod 60, year mod 10000, second mod 60, runs it through the shell helper `0xffead624` and replies status 0 whatever the command returns (an invalid command logs "ERROR: Invalid date command"). So the set request moves the L1's own clock and the timekeeper chip.

**ARM `F` (inferred).** The code table of `L1ReqIRouter` puts `0x200d` and `0x200e` at `0xcdc64` and `0xce3d0`, and the program carries the same `date -u ...; hwclock -u` text; those addresses are fragments of the router function and were not decompiled again here. The layout "year to second as integers" stays inferred for ARM.

## Text commands (request code `0x0000`, task 3)

**Table entries, ColdFire package 3 (established, `tools/l1_command_table.py` on the carved package).** The commands the Altix 350's PROM sends:

| Text | Entry handler | Brick filter | Selector words |
|---|---|---|---|
| `* <command>` | `0xffe1196c` | `CT124567890VQAO^+@BDJKFHLM` (includes `O`) | 0 |
| `power`/`pwr down`/`d` | `0xffe15a48` | none | `0, 1, 0`: word 12 = 1 selects power-down |
| `reset`/`rst` | `0xffe15c08` | none | `0, 0, 0` |
| `reset`/`rst hold` | `0xffe15c08` | none | `0, 1, 0` |
| `version`/`ver` | `0xffe4c01c` | none | `0, 0, 0`; the handler body was not read |

ARM table (established from the entry layout, nine pattern words then the handler): `power|pwr down|d` handler `0xd8fe0`, selector 1; `reset|rst` handler `0xd9810` (inside the routine that starts at `0xd97c0`), selector 0; `reset|rst hold` selector 1.

**`reset` (established for the call structure, ColdFire package 3 `0xffe15c08`).** It asks the supply-state routine `0xffe6baa0` and answers "ERROR: power appears off." and fails unless it returns 1. Otherwise selector 0 calls the board reset hook (RAM word `0x30000900`) between two clock reads and logs "reset took %d seconds"; selector 1 calls the reset-hold hook (`0x30000904`). Failure texts are formatted with "ERROR: %s". The ARM routine has the same shape (hooks `DAT_000d9c6c` and `DAT_000d9c74` in its literal pool, same log line). So a `rst` **resets the brick** and does not power it off.

**`power down` (established for the call structure, `0xffe15a48`).** Selector 1 calls the power-down hook `0x3000093c` (selector 0, power up, the hook `0x30000940`); a failure is reported as "ERROR: %s". The ARM handler `0xd8fe0` was not decompiled.

**What the hooks drive (unknown for `O` and `F`).** The hook words are filled by the per-brick-type initialisers, which were not traced for `O`; the instructions in the package only read them (no store to `0x30000900`, `0x30000904`, `0x3000093c` or `0x30000940` exists as an absolute-address store; the three small routines at `0xffe66440` to `0xffe6647c` install defaults only when a flag routine returns 1). For the sibling letters the older pages show the pattern: [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md) (`power down` cuts the switched supplies in reverse order after holding the bridge resets; `reset` holds then releases expander lines) and [power-reset-cbrick.md](power-reset-cbrick.md).

**The `*` handler (`0xffe1196c`, established for the structure).** It builds the destination list from the brick's own module records and the port table (one entry per distinct module, value `(rack/slab fields) << 12 | 3`), then sends the command to each and runs it locally; with no peers the list is the local brick. A lone Altix 350 brick therefore runs `rst` itself.

## Event clear `0x100a`

Established (`0xffe3df54`, same as the sn2-prom-link page): no reply; the event word becomes -1 and setting `0x13` is cleared. The node sends it only on the type `l` power-down path, which no Altix 350 letter reaches.

## What an emulated L1 needs

- `0x200e`: status 0, six integers, four-digit year first, host UTC.
- `0x200d`: status 0.
- `0x0000` text `version`: status 0 (the PROM reads only the status). Text `* rst` or `p <n> rst`: any reply frame, then reset the machine. Text `* pwr d` or `p <n> pwr d` is never sent for the letters `f` and `o`.
- `0x100a`: no reply.

## Not read

The body of the board hooks for `O` and `F`; the ARM `0xd8fe0` and `0xce3d0` handlers as functions; whether the 1.62.1 `version` handler differs from the 1.44.0 one; the real timekeeper part number of the ColdFire L1 (the register layout is a timekeeper SRAM but no datasheet was matched).
