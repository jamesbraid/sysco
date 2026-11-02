# L1 console and log messages

Every text the Origin 300 L1 prints to its console or writes to its log is passed to one of two routines in the ColdFire body. This note covers those call sites in L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`), first flash bank, loaded at `0xffe00000`. The table is [inventory/console-messages.tsv](../inventory/console-messages.tsv), made by [scripts/console_messages.py](../../scripts/console_messages.py) from the listing in [disasm/mcf5206e-1.44.0-fuel-pe-o300/](disasm/mcf5206e-1.44.0-fuel-pe-o300/README.md). Re-run the script after any change to the listing or the curated roles.

## The two sinks

- `log_message(severity, file, line, flags, format, ...)` at `0xffe58380`. The arguments are on the stack, first argument on top. Each call passes the source file name and line of the L1 code that logged it, so the file column names the module (`Env.c`, `fan.c`, `SMPCmd.c`). The message is formatted with the variadic arguments, then printed with a prefix chosen by severity.
- `console_printf(format, ...)` at `0xffe5aed8`. A format and its arguments, printed on the L1 console without a severity or source line.

Severity is not a fixed label. The routine ANDs the severity with the word at `0x3000039c` and dispatches on the result (0 to 9). The prefix for each result comes from a table of pointers at `0x30000368`, which the body builds at run time. The routine that fills that table was not located, so the prefix text for each class is unknown. Case 9 (read in part) compares the counter at `(a2)` with 2; on a match it prints `recursive PANIC detected.....rebooting NOW`, then calls the controller restart routine `0xffe56654` with 0x3e7. The table column gives the severity literal each call passed, not the printed prefix.

## What the table holds

583 sites in the first bank: 507 `log_message` and 76 `console_printf`. Each row gives the call address, routine, severity, flags, source file and line, format, the number of variadic arguments against the conversions in the format, the nearest guard branch that jumps over the call (with the compare it reads), and the pushed argument values. Values read as `expr`, `call`, `joined` or `unknown` were not resolved to a literal by the script.

Status is `extracted` for all rows except the 24 that have a curated role in [console-message-roles.tsv](console-message-roles.tsv). Those rows carry a grade. Established rows were read in the listing (guard, flag and argument); inferred rows were read from the table row only: the format, the arguments and the guard. No routine body behind an inferred row was read.

## Errors and faults first

Of the 583 sites, roughly 400 match an error keyword (error, fail, cannot, invalid, too many and similar). The curated rows start with the fan, power and monitor modules, which are the hardware-fault messages (monitor register reads, interrupt status, SMBus alert, fan speed, power margin). Those modules print through `log_message` with the source file named in the table.

Two cases read as hardware faults with a clear trigger: the failed monitor register read (`Error reading register %02x on monitor %s, %s`) and the failed I/O expander fan write. Both print a status text that comes from a routine the table names only by address (`0xffe5b7b0`, a value-to-text conversion whose identity is not established).

## Not covered yet

- The second flash bank (`0xffec...` onward, the same code shifted by `0x70000`). Its copies of `log_message` and `console_printf` are listed in the function table; their call sites are not.
- Replies sent back to the operator as text (`general_error_reply`, the text fields of requests). Those are protocol replies, not console or log output, and are not in the table.
- The ARM L1 programs and the L2 daemon. Neither has a sink table yet.
- The other four ColdFire packages of 1.44.0 and the other releases.

## Routes to settle the open points

- Severity prefixes: find the writes to `0x30000368` and to `0x3000039c` (initialiser, or a block copy from the flash image). A read of the reset path that sets them would settle the mask.
- `ffe5b7b0`: read the routine end to end and name what its text table holds.
- Guards marked `joined`: read the branch targets listed in the table's `pushes` column to see which path reaches the call.
