# L1 console and log messages

Every text the Origin 300 L1 prints to its console or writes to its log is passed to one of two routines in the ColdFire body. This note covers those call sites in L1 1.44.0, package 1 (`Fuel/PE/O300 1MB`), first flash bank, loaded at `0xffe00000`. The table is [inventory/console-messages.tsv](../inventory/console-messages.tsv), made by [scripts/console_messages.py](../../scripts/console_messages.py) from the listing in [disasm/mcf5206e-1.44.0-fuel-pe-o300/](disasm/mcf5206e-1.44.0-fuel-pe-o300/README.md). Re-run the script after any change to the listing or the curated roles.

## The two sinks

- `log_message(severity, file, line, flags, format, ...)` at `0xffe58380`. The arguments are on the stack, first argument on top. Each call passes the source file name and line of the L1 code that logged it, so the file column names the module (`Env.c`, `fan.c`, `SMPCmd.c`). The message is formatted with the variadic arguments, then printed with a prefix chosen by severity.
- `console_printf(format, ...)` at `0xffe5aed8`. A format and its arguments, printed on the L1 console without a severity or source line.

Severity is not a fixed label. The routine ANDs the severity with the word at `0x3000039c` and dispatches on the result (0 to 9). The prefix for each result comes from a table of pointers at `0x30000368`, which the body builds at run time. The routine that fills that table was not located, so the prefix text for each class is unknown. When the masked severity is 9 the routine first calls `0xffe56718` and adds one to the counter at `0x30000390`. The case that tests that counter (read in part) prints `recursive PANIC detected.....rebooting NOW` when it equals 2, and calls the controller restart routine `0xffe56654` with 0x3e7 when it is above 2 or after that print. The table column gives the severity literal each call passed, not the printed prefix.

## What the table holds

583 sites in the first bank: 507 `log_message` and 76 `console_printf`. Each row gives the call address, routine, severity, flags, source file and line, format, the number of variadic arguments against the conversions in the format, the nearest guard branch that jumps over the call (with the compare it reads), and the pushed argument values. The argument count is the word count the call's own stack cleanup removes, so arguments that are themselves calls (a name routine, a status text) are included. Six messages begin with the name of the routine that logs them. The format column shows `<routine>` for that name, and the full text is at the call address in the image. Values read as `expr`, `call`, `joined` or `unknown` were not resolved to a literal by the script. A guard that ends with `other paths reach the call` has a branch target between the guard and the call, so the guard is not the only way in; read the listing for the real condition.

Status is `extracted` for all rows except the 24 that have a curated role in [console-message-roles.tsv](console-message-roles.tsv). Those rows carry a grade. Established rows were read in the listing (guard, flag and argument); inferred rows were read from the table row only: the format, the arguments and the guard. No routine body behind an inferred row was read.

## Errors and faults first

Of the 583 sites, 376 have a format that matches error, fail, cannot, can't, invalid, too many, unable, bad, illegal, overflow or out of (any case). The curated rows start with the fan, power and monitor modules, which are the hardware-fault messages (monitor register reads, interrupt status, SMBus alert, fan speed, power margin). Those modules print through `log_message` with the source file named in the table.

Two cases read as hardware faults with a clear trigger: the failed monitor register read (`Error reading register %02x on monitor %s, %s`) and the failed I/O expander fan write. Both print a status text from `0xffe5b7b0`, a status-to-text routine (read in part): it range-checks a negative status, picks an entry of 12 bytes from a table at `0xffe5be20` by a value derived from the status (count at +4, pointer to a list of text pointers at +8), and returns the text `Unknown Module` (`0xffe5d964`) outside the table's range or `Unknown Message` (`0xffe5d973`) past the entry's count. A very similar routine, `error_name` at `0xffe5b6b4`, is in the function table.

## Where the argument count and the format disagree

Seven rows have a format that is not a literal, so the count is `unknown`. The rest that differ from the format are `0xffe690e2` and `0xffe6962e` (`Invalid LM75 offset: %d`, which push no argument for the `%d`), `0xffe31d5c` (the one argument `%x` reads is the first word of a 72-byte structure copied to the stack by value before the format) and `0xffe67a56` (a second argument, the status text, that the format has no conversion for).

## Not covered yet

- The second flash bank (`0xffec...` onward, the same code shifted by `0x70000`). Its copies of `log_message` and `console_printf` are listed in the function table; their call sites are not.
- Replies sent back to the operator as text (`general_error_reply`, the text fields of requests). Those are protocol replies, not console or log output, and are not in the table.
- The ARM L1 programs and the L2 daemon. Neither has a sink table yet.
- The other four ColdFire packages of 1.44.0 and the other releases.

## Routes to settle the open points

- Severity prefixes: find the writes to `0x30000368` and to `0x3000039c` (initialiser, or a block copy from the flash image). A read of the reset path that sets them would settle the mask.
- `ffe5b7b0`: read the routine end to end and name what its text table holds.
- Guards marked `joined`: read the branch targets listed in the table's `pushes` column to see which path reaches the call.
