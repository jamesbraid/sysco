# Origin 300 L1 debug command and controller restart reading

This note traces the operator `debug` and `reboot_l1` commands in a selected Origin 300 L1 image.

## Checked image and context

The image is the Origin 300 L1 1.44.0 body described in [package-container.md](package-container.md) (build stamp `20060717182038`, loaded at `0xffe00000`, reset vector `0xffe00400`). Addresses are first-bank; subtract `0xffe00000` for a body offset. Locators are read from the committed listing.

The public Origin 300 guide `007-4498-002`, printed page 181, gives an operator debug example. The public controller guide `007-3938-006`, printed pages 63–64, describes debug-switch bits. These are hints for the operator forms; the image resolves the parser and reset branches below.

## `debug` text and value

The selected command table has a no-value read form at body offset `0x1cafc` and a one-value write form at `0x1cb40`. Both reach the same handler (`0xffe15214`). Both first check brick type (`0xffe140d4`, table constraint at body offset `0x21465`). The IP45 controller stores brick character `4` (see [rs422-router.md](rs422-router.md) for the brick-type value `4`). The guard's alias map at `0xffe21014` accepts `4` for the command's `c` class (inferred from the table shape; the table at `0xffe21014` was not read entry by entry). That is why the formatter and setting paths below are reached on Origin 300/IP45. A nonmatching brick type receives an unsupported-command information field and skips the formatter. Fuel and PE behaviour is unknown from this note; the route is the same table read in the other 1.44.0 packages.

On that accepted path, the read form retrieves setting index three. The write form updates it, retrieves it again, and emits the same text. The output says the debug switches are set to a `0x`-prefixed word, using lowercase hex with at least four digits and a trailing line feed. Thus accepted IP45 `debug` returns a statement about the current word, rather than a bare number. For example, `debug 0x2a` should return `debug switches set to 0x002a`. The public guide also shows the `0x` word form but does not settle every routing condition.

The command executor (`0xffe1b0f8`) first tries the unqualified local parse. For an accepted plain local command, it returns that handler's text without adding a brick identifier. A separate targeted route may prepend a brick identifier and colon on its own text line (`0xffe1b0f8`). Response serialization occurs at `0xffe1ba5c`. The selected handler itself has no prefix. How an operator console renders separate text fields is outside the image. The selected local IP45 response fields justify an unprefixed `debug` line. A target-qualified form should be checked separately.

The update path (`0xffe58758`) stores the low 16 bits of the parsed integer in the primary controller settings record and refreshes its checksum and backup. The retrieval path (`0xffe59590`) reads that 16-bit word. This explains why a value above `0xffff` can be accepted but display a wrapped low word. It also aligns with the public operator examples and the earlier firmware reading of debug-switch retention.

## One-value conversion and errors

The common command parser (`0xffe61bd0`) assigns the one-value `debug` form an auto-base numeric token and calls the decoder at `0xffe7cd0c`. An unprefixed number is decimal, a leading zero selects octal, and `0x` or `0X` selects hexadecimal. A leading plus or minus sign is accepted. The parser requires the decoder to consume the entire token before dispatch. It does not test the decoder's conversion-error flag. From these branches, `debug 42`, `debug 052` and `debug 0x2a` all set and display `0x002a` on the accepted IP45 path.

Negative values are parsed as signed 32-bit modular values before the setting path masks them to 16 bits. `debug -1` therefore reaches the handler as `0xffffffff` and displays `0xffff` on IP45. Likewise `debug 65536` or `debug 0x10000` displays `0x0000`. These are static branch conclusions.

The decoder's disassembly checks the unsigned 32-bit quotient and remainder before accumulating each digit. On overflow it records conversion error 34 and saturates the returned value to `0xffffffff`, then continues consuming numeric digits. Since the common parser checks token consumption but never checks that error, a wholly numeric overflow such as `debug 4294967296` appears to dispatch and display `0xffff` on IP45. A suffix such as `debug 12junk` or a wholly nonnumeric value such as `debug z` fails token consumption and does not invoke the setting handler. For `debug nope` and `debug 12junk` the parser returns code -1200 with the command-not-found line, because each later command candidate clears the partial-match marker (see [debug-task.md](debug-task.md)); the invalid-arguments line is produced only when the marker survives to the end of the table. For the empty hexadecimal prefix `debug 0x`, the decoder moves past the prefix, finds no digit, and leaves the end pointer at the end of the token (`0xffe7cd9e`–`0xffe7cda4`, `0xffe7ce38`–`0xffe7ce5a`). The parser's whole-token test therefore appears to accept zero and set/display `0x0000` on IP45. This is a static inference from the two decoder ranges cited. Whitespace edge cases remain unresolved.

## `reboot_l1` and settings restoration

The no-argument restart command entry at body offset `0x1fe84` selects the restart branch at `0xffe3b3fc`. That branch calls the controller restart path (`0xffe56654`) with normal reboot mode and a zero alternate-image address. It does not invoke the settings validator or backup recovery directly. Other branches of the shared handler select particular flash images or report image status. A failed image validation can return a failure line without restarting. These are distinct from the no-argument operator request.

Before the no-argument branch reaches the restart call, it attempts a 100-unit task wait (`0xffe3b3fc`, helper `0xffe40e54`). That helper waits only with a valid current task context. Otherwise it returns immediately. The units are software task-clock units. Static code does not establish elapsed wall time or when, if ever, an operator receives a reply.

The restart path first records reboot state and calls lower reset support. If device control bit 7 is clear, it then executes ColdFire `HALT` (`0xffe566bc`). Only after that conditional branch does normal reboot mode write the boot address and its complement in controller RAM (`0xffe566c4`–`0xffe566e0`). The path then stays in a loop pending reset. One later helper called by that path is a no-effect stub in this selected image. The static image therefore does not establish which hardware event actually drives reset, the timing, or whether the operator sees a successful reply before the controller disappears. Calling this a verified watchdog reboot would overstate the evidence.

On a normal transition back through the selected reset vector, startup (`0xffe10570`, reached from `0xffe00400`) invokes settings validation at `0xffe5a758`. Invalid primary settings trigger backup recovery at `0xffe5a188`. If both records are unusable, defaults are installed. There is no observed `reboot_l1`-specific skip around this startup check. This is a conditional conclusion: the command itself does not restore settings, but a normal subsequent boot through the selected image does. A different boot image or a hardware reset path that does not re-enter this startup would need separate evidence.

## Routes to settle the remaining behaviour

- Malformed and overflow `debug` forms, the task-3 status for each, and target-qualified output: read the parser (`0xffe61bd0`), the executor (`0xffe1b0f8`) and the reply builder (`0xffe1ba5c`) end to end, then repeat the read in the 1.48.1 package of the same layout.
- `reboot_l1`: the restart path (`0xffe56654` to `0xffe566e0`) ends in a wait loop after a `HALT`; the reset source is not named in the image. Read the host-side `l1cmd` and the IRIX L1 client objects for what they expect after the command, and the MCF5206E reset behaviour from the public manual and MAME's MCF5206E core.
- Settings surviving the restart: read `0xffe5a758` and `0xffe5a188` in full for the corrupted-primary, valid-backup case.
