# The inventory record behind `syssgi` selector 102

Read from `root/6.5/usr/etc/sysctlrd` (IRIX 6.5 Foundation 1, SHA-256 `c92b4b43c67fc22e...`), the same daemon as [records.md](records.md). Addresses are virtual addresses in that daemon. Grades follow [../../AGENTS.md](../../../AGENTS.md).

## Who reads it

`getpromerrs` (`0x10006494`) calls `syssgi(0x66, buffer, 0x6e8)` on a 1,768-byte stack buffer. A non-zero result logs a failure at priority 3 and stops. Otherwise it passes the buffer to `check_inventory` (`0x1000703c`) and then checks the text at buffer offset `0x6dc` with `valid_serial`. **Established**, both routines read whole.

## Record layout, as the reporting routines use it

The buffer is sixteen slot records of `0x6c` bytes (offsets `0` to `0x6bf`), followed by 40 bytes. `check_inventory` walks the sixteen slots in order; the slot number it prints is its loop index when it reports a change of board, and the byte at slot offset `0x69` in every other message. **Established**: a loop bound of 16, stride `0x6c`, and `16 * 0x6c + 40 = 0x6e8`, the length the daemon passes.

| Offset in a slot | Use | Grade |
|---|---|---|
| `0x64` | board type found now (the codes of `board_name` in [records.md](records.md)); zero means the slot is empty | established |
| `0x66` | zero means the board is disabled (tested for every board type, after the diagnostic code) | established |
| `0x67` | board type recorded earlier; compared with `0x64` | established |
| `0x68` | diagnostic code of the board itself; non-zero is passed to `evdiag_msg` | established |
| `0x69` | slot number, as printed in messages | established |
| `0x00` to `0x63` | board-type-specific sub-records, see below | established for the offsets listed, otherwise unread |
| `0x65`, `0x6a`, `0x6b` | never read by any of the four routines | unknown |

Trailing 40 bytes: the serial-number text starts at buffer offset `0x6dc` (12 bytes to the end); `valid_serial` accepts a non-empty printable string of at most ten characters. The 28 bytes from `0x6c0` to `0x6db` are not read by the daemon. **Established** for what is read; unknown for the rest.

## What `check_inventory` reports

For each slot, in this order (**established**, `0x1000703c` to `0x10007244`):

1. Slot now empty (`0x64` zero) but `0x67` non-zero: priority 5, "Slot N previously contained an X board but is empty".
2. Both non-zero and different: priority 5, "Slot N previously contained an X board but now an Y board", then the walk goes on with the current type.
3. Diagnostic code `0x68` non-zero: priority 3, "The X board in slot S failed diagnostics" followed by priority 3 "Reason: <evdiag_msg text>".
4. Otherwise, when `0x66` is zero: priority 5, "The X board in slot S is DISABLED".
5. Otherwise, for `0x67` equal to `0x64` the board is examined by type: high nibble `0x10` (CPU) by the routine at `0x10007248`, `0x30` (memory) at `0x10007734`, `0x20` (I/O) at `0x10008288`. Each takes the slot record and the address of a local "something was reported" flag.

The flag is set by every message above and by the sub-reports; nothing reads it in `check_inventory`, so it is not a return value (**established**: the routine returns nothing).

## Sub-records

**CPU board** (`0x10007248`, four slices, 16 bytes apart from record offset 0). For slice k at `0x10 * k`: byte `+9` zero means the slice is disabled; byte `+0xb` is the slice's diagnostic code, where `0xff` means none was recorded and a code other than `0xff` or zero is reported as "Slice k on the X in slot S failed diagnostics" plus the reason; byte `+0xa`, tested only when `+0xb` is `0xff`, makes the routine report "Slice k ... isn't visible" when it is not `0xff`. On board type `0x12` (IP21 in `board_name`) the disabled test is skipped for slices 2 and 3. **Established** for the offsets and messages; what `+0xa` carries is unknown.

**Memory board** (`0x10007734`, eight banks A to H, 12 bytes apart starting at `+4`). For bank k at `4 + 0xc * k`: `+1` zero means the bank is disabled ("Bank L on the MC3 in slot S is DISABLED."), `+3` is the diagnostic code ("failed diagnostics" plus reason). Bytes `+2` and `+6` of neighbouring banks are compared with the value 7 and with each other before a bank is reported; that pairing is not decoded. **Established** for `+1` and `+3`; the pairing is unknown. Bank letters come from a local helper at `0x10008dcc`.

**I/O board** (`0x10008288`, entries 12 bytes apart starting at `+4`; the code reads seven). For adapter k at `4 + 0xc * k`: byte `+0` nonzero means enabled, `+1` the adapter type recorded earlier, `+2` the diagnostic code, `+3` the adapter type now (`0xf` means none), `+5` the subtype now. A change is declared when the two types differ, except that a recorded "none" (`0xf`) followed by a current type `0x20` or `0x11` is not a change (helper `0x10008248`): priority 5 "IOA k on the IO4 in slot S has changed" and "IOA used to be X and now is Y" with names from `ioa_name`. A non-zero diagnostic code gives priority 3 "The X at ioa k on the IO4 in slot S failed diags." plus the reason; a zero enable byte with a present adapter gives priority 5 "... is DISABLED". **Established** for the offsets and tests read in the first adapter's code at `0x10008430` to `0x10008574`; the other adapters repeat it at 12-byte steps (their loads, `+0x13` through `+0x4f`, were read; their bodies were not compared one by one, so those are inferred).

## What is not settled

- Who fills the buffer. The producer is the IRIX kernel's handler for `syssgi` selector 102, which is not in `root/`; the layout above is the consumer's view only. What would settle it: the kernel image of a release that has the selector, or a header that describes the record.
- The meaning of slot offsets `0x65`, `0x6a`, `0x6b`, of CPU byte `+0xa`, and of the memory-bank pairing. None is read by any routine in the daemon, so no further software in `root/` reads them either; they stay unknown until the producer is available.
