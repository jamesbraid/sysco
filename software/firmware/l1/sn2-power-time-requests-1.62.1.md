# What the L1 does for the node's text commands, time-of-day requests and event clear (L1 1.62.1)

The node side is in the chips repository, `software/sal/runtime-services.md` section 6 and `software/firmware/snprom/l1-requests.md` ("Requests after boot"). This page reads the ColdFire L1 `O` (package 3, `2MB image`, string `O5#=?`, SHA-256 `14b7122d...a66e`; Ghidra `68000:BE:32:Coldfire`, flat load `0xffe00000`, 44-byte header skipped) and, for the command table, the ARM Linux L1 program (SHA-256 `22cb6287...`, extracted from the CramFS of package 4, flat load `0x8000`). Grades follow [AGENTS.md](../../../AGENTS.md). No emulator ran.

## Time of day

**`0x200e`, get (established, ColdFire package 3).** A block of the general-task dispatcher `0xffe3f9d4`. It sets status 0, calls the clock reader `0xffe7ba9c` (seconds since 1970, or -1), converts it with `0xffe7fa80` (`gmtime`; the reader adds setting 72 (`0x48`, the 32-bit word at settings `+0x50`, the standard-time offset the zone string builder `0xffe7bfec` prints) to the seconds first) and appends six integers in this order: year (`tm_year + 1900`, four digits), month (`tm_mon + 1`), day, hour, minute, second. No arguments are read. Request with any argument count works the same.

**`0xffe7ba9c` source (established).** It reads a BCD timekeeper in the L1's own address space at `0x2000_07f8` to `0x2000_07ff`: seconds `+1`, minutes `+2`, hours `+3`, date `+5`, month `+6`, year `+7`, with the century digits in the top bits of `+0` (control byte; the code ORs in bit `0x40` before the digit read and clears it again; the 0x40 is ORed into the value read, the store only clears it). It returns -1 when every field is zero or the year is outside 1930 to 2492. The handler does not test for -1, so a dead or unset clock gives a reply from `gmtime(-1)` (**inferred** from the missing test; the reply would be 1969-12-31 23:59:59).

**`0x200d`, set (established, ColdFire package 3).** Needs seven words including the code (`-100` otherwise). Arguments 1 to 6 are year, month, day, hour, minute, second. The handler formats `(date -u %02d%02d%02d%02d%04d.%02d; hwclock -u --systohc) >/dev/null 2>/dev/null` (the text is shared with the ARM program) and passes it to `0xffead624`, which in this ColdFire image is `moveq #0,d0; rts` (bytes `7000 4e75`), the only caller being this handler. So on ColdFire `O` the set request changes nothing and replies status 0.

**ARM `F` (established for `0x200e`).** The fragment at `0xce3d0` calls `time(0)` and `gmtime`, then appends year (`tm_year + 1900`), month, day, hour, minute, second as integers, so the layout matches the ColdFire one. `0x200d` on ARM (`0xcdc64`) was not re-read; the shell text suggests it really sets the Linux clock.

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

**`power down` (established for the call structure, `0xffe15a48`).** Selector 1 calls the power-down hook `0x3000093c` (selector 0, power up, the hook `0x30000940`); a failure is reported as "ERROR: %s". The ARM handler `0xd8fe0` has the same shape (selector 1 calls the power-down hook) and adds a per-slot mask path (`powerUpSlot`, `powerDownSlot`).

**What the hooks drive (unknown for `O` and `F`).** The hook words are filled by the per-brick-type initialisers, which were not traced for `O`; the instructions in the package only read them (no store to `0x30000900`, `0x30000904`, `0x3000093c` or `0x30000940` exists as an absolute-address store; the three small routines at `0xffe66440` to `0xffe6647c` install defaults only when a flag routine returns 1). For the sibling letters the older pages show the pattern: [power-reset-ibrick-rbrick.md](power-reset-ibrick-rbrick.md) (`power down` cuts the switched supplies in reverse order after holding the bridge resets; `reset` holds then releases expander lines) and [power-reset-cbrick.md](power-reset-cbrick.md).

**The `*` handler (`0xffe1196c`, established for the structure).** It builds the destination list from the brick's own module records and the port table (one entry per distinct module, value `(rack/slab fields) << 12 | 3`), then sends the command to each and runs it locally; with no peers the list is the local brick. A lone Altix 350 brick therefore runs `rst` itself.

## A text command addressed to an L2 (`0x1ffff003`) with no L2

The node's `version` probe uses the destination word `0x1ffff003`. **Established (ColdFire `O`, package 3, Ghidra):** the router `0xffea7850` takes the local task table only when the destination shifted right 12 equals the brick's own id or `0xffff`; `0x1ffff` is neither, no remote entry matches, so it goes to the catch-all registered for `O` (`0xffe37110`; the registration routine `0xffe35f44` registers `0xffe373e4` for the I/O-style letters `I X Y P G N & # = E V`, and `O` (`0x4f`) is not among them). Type nibble 1 with module `0xffff` goes to the route chooser `0xffe36e78`: route word at brick record `+0x58` of 0 gives the log line "B2B: no receiver (REQ)" and a routing-error response (`0xffea7fc8`, code -1408, detail 8); -1 (a USB host "registered as remote", set by the task `0xffe35564`) queues it to the USB writer. Either way the request is **not** run by task 3, so the PROM never sees status 0. Wire form of that error response was not decoded for ColdFire; on the ARM program an error packet is sent as status `0xffffffff` plus a 4-byte code. Whether `O` runs the USB task (the USB count word `0x30003438` is only read in this image, never stored by an absolute store) was not decided. The ARM router `0x109af8` has the same non-local test, so `0x1ffff003` is not delivered locally there either; its catch-all was not traced.

## Event clear `0x100a`

Established (`0xffe3df54`, same as the sn2-prom-link page): no reply; the event word becomes -1 and setting `0x13` is cleared. The node sends it only on the type `l` power-down path, which no Altix 350 letter reaches.

## What an emulated L1 needs

- `0x200e`: status 0, six integers, four-digit year first, host UTC (setting 72 is the zone offset; zero unless set; its default was not read).
- `0x200d`: status 0 and no other effect (ColdFire `O`).
- `0x0000` text `version` to `0x0ffff003`: the handler `0xffe4c01c` was not read. To `0x1ffff003` (what the PROM sends): an error status, never 0. Text `* rst`: any reply frame, then reset the machine; `p <n> rst` is only sent when the probe saw status 0. Text `* pwr d` or `p <n> pwr d` is never sent for the letters `f` and `o`.
- `0x100a`: no reply.

## Not read

The body of the board hooks for `O` and `F`; the ARM `0xd8fe0` and `0xce3d0` handlers as functions; whether the 1.62.1 `version` handler differs from the 1.44.0 one; the real timekeeper part number of the ColdFire L1 (the register layout is a timekeeper SRAM but no datasheet was matched).

## Review

Independent re-read on 2026-11-16: ColdFire `O` (package 3, SHA-256 `14b7122d...a66e`, flat `0xffe00000`, 44-byte header skipped) in Ghidra with a full analysis; ARM `l1` (SHA-256 `22cb6287...bfc5`, carved from the package 4 CramFS) in Ghidra at `0x109af8`, `0xce3d0`, `0xd8fe0`, `0x6cda4`. No emulator ran; the objdump cross-check was done on the PROM side only.

**Confirmed.** `reset`/`rst` at `0xffe15c08`: "ERROR: power appears off." unless `0xffe6baa0` returns 1, selector 0 calls RAM hook `0x30000900` between two clock reads and logs "reset took %d seconds", selector 1 calls `0x30000904`, failures print "ERROR: %s". `power` at `0xffe15a48` selector 1 calls `0x3000093c`, 0 calls `0x30000940`. Hook words: only readers and the small installers `0xffe66440`..`0xffe6647c` (they load the hook address into a register), no absolute store. `0x200e` in `0xffe3f9d4`: status 0, six integers, no argument read; `0xffe7ba9c` reads the BCD bytes at `0x2000_07f8`..`07ff` with fields as listed, returns -1 when all fields are zero or the year is outside 1930..2492; the handler does not test -1. `0x200d` needs seven words (`-100`). `0x100a` calls `0xffe3df54`. ARM `SHUB_ScanSetRackSlot` at `0x6cda4` and ColdFire `0xffe5b3dc` build `rack << 26 | bay << 20 | shub << 16 | partition << 8 | 0x6c` with `datahi = (rack >> 6) & 0xf`, 50 retries, register `0x101d0100`; ARM `0xce3d0` appends six integers from `gmtime`.

**Changed.** `0x200d` on ColdFire is a no-op (the shell helper is a stub), not a clock set. `0x200e` adds the zone setting before `gmtime`. The "UTC; no zone is applied" claim, the `0x200d` "needs the timekeeper" sentence, the text-command table entry for `version` (the probe never reaches it), and a new section on the `0x1ffff003` destination. Rack, bay in the brick record (`+8`, `+0xc`) are now **established**: the router compares them with the destination's bits 27:18 and 17:12 (`0xffe35f44` stores them in `0x30001290`/`0x30001294`); the partition at `+0x3c` stays inferred. ARM `0xd8fe0` read.

**Not checked.** Handler `0xffe4c01c` (`version`); the board hooks for `O`; the wire form of the ColdFire routing-error reply; whether `O` runs the USB task; ARM `0xcdc64` and `0xd9810` (not read in detail); the ARM catch-all.
