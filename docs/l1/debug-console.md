# Origin 300 L1 debug command and controller restart reading

This note traces the operator `debug` and `reboot_l1` commands in a selected Origin 300 L1 image.

## Checked image and context

The selected 982,412-byte range in the IRIX 6.5.30 composite has SHA-256 `0c969f9e98c73826495dc37bff685b3403e248993ba92194a288333d9d42b2f4`. It identifies itself as L1 1.44.0, built 2006-07-17 for the Fuel/PE/Origin 300 family. The ColdFire body has SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`. It was analysed with load address `0xffe00000` and reset vector `0xffe00400`. Subtract `0xffe00000` from an address below for its body-relative offset. Its manifest records 2,981 complete functions and six failures. The parser, setting and startup functions cited below are complete.

The checked pass did not decompile the command handler at `0xffe3b3fc`, although it contained its machine-code region. Its reader-map SHA-256 is `235ac82661ce8129c50bcfe1523370415df33a8be0322b480c27ed6d826cd6d4`. An initial command without a local uv cache failed to lock the host uv cache because its filesystem was read-only. A retry with the local uv cache could not reach Podman's local state from the filesystem sandbox. Shared analysis tools were being edited during that run. The generic guard did not identify which input changed, and it wrote no manifest or `sha256sums`. Its function table, decompile and disassembly hashes match the stable run listed below. Its log SHA-256 was `5f9071bb52f80821ac5c2210a1b17f2351751e3897aa1033839ef9daf99ca816`.

This exact command exited zero:

```sh
UV_CACHE_DIR=.cache/uv \
TMPDIR=$PWD/.cache/l1-analysis/debug-reboot-seed-20260915/tmp-stable-1135 \
uv run --script tools/l1-analysis \
  --image root/6.5.30/usr/cpu/firmware/sysco/l1.bin \
  --cpus 2 --timeout 3600
```

The stable run's manifest SHA-256 is `c10e34893c411ff206a5e1ad87ec828cf8750d967639c34ca850ca3537b963c2`. Its `sha256sum -c` passed all eight named outputs. It records 2,982 complete functions of 2,988 detected, six failed, and the seeded restart handler at `0xffe3b3fc` marked complete. Its `functions.tsv`, `decompiled.c` and `disassembled.lst` SHA-256 values are respectively `6574e20e239cdf0aa3325300e19efbf20354bfec10e6ea8d7c6edfcd4997671f`, `305455906e423e0e625c0edb0d003e01089b786268ab9d111101bcce4e726b43`, and `8b265ad13b54a8cdce5131d6a6f0e0e30eb4ae65c55c55d4fe576cc92de118d5`. The decompile and disassembly hashes match the earlier Ghidra output, while the stable wrapper now binds them to an unchanged source and image.

The public Origin 300 guide `007-4498-002`, printed page 181, gives an operator debug example. The public controller guide `007-3938-006`, printed pages 63–64, describes debug-switch bits. The publications support the operator command's existence, while the selected image resolves the parser and reset branches below.

## `debug` text and value

The selected command table has a no-value read form at body offset `0x1cafc` and a one-value write form at `0x1cb40`. Both reach the same handler (`0xffe15214`). Both first check brick type (`0xffe140d4`, table constraint at body offset `0x21465`). The IP45 controller stores brick character `4`, as recorded in the controller datasheet. The selected guard's alias map at `0xffe21014` accepts `4` for the command's `c` class. That is why the formatter and setting paths below are reached on Origin 300/IP45. A nonmatching brick type receives an unsupported-command information field and skips the formatter. Fuel and PE operator behavior requires separate evidence.

On that accepted path, the read form retrieves setting index three. The write form updates it, retrieves it again, and emits the same text. The output says the debug switches are set to a `0x`-prefixed word, using lowercase hex with at least four digits and a trailing line feed. Thus accepted IP45 `debug` returns a statement about the current word, rather than a bare number. For example, `debug 0x2a` should return `debug switches set to 0x002a`. The public guide also shows the `0x` word form but does not settle every routing condition.

The command executor (`0xffe1b0f8`) first tries the unqualified local parse. For an accepted plain local command, it returns that handler's text without adding a brick identifier. A separate targeted route may prepend a brick identifier and colon on its own text line (`0xffe1b0f8`). Response serialization occurs at `0xffe1ba5c`. The selected handler itself has no prefix. The final rendering of separate text fields by a particular operator console remains unmeasured. The selected local IP45 response fields justify an unprefixed `debug` line. A target-qualified form should be checked separately.

The update path (`0xffe58758`) stores the low 16 bits of the parsed integer in the primary controller settings record and refreshes its checksum and backup. The retrieval path (`0xffe59590`) reads that 16-bit word. This explains why a value above `0xffff` can be accepted but display a wrapped low word. It also aligns with the public operator examples and the earlier firmware reading of debug-switch retention.

## One-value conversion and errors

The common command parser (`0xffe61bd0`) assigns the one-value `debug` form an auto-base numeric token and calls the decoder at `0xffe7cd0c`. An unprefixed number is decimal, a leading zero selects octal, and `0x` or `0X` selects hexadecimal. A leading plus or minus sign is accepted. The parser requires the decoder to consume the entire token before dispatch. It does not test the decoder's conversion-error flag. From these branches, `debug 42`, `debug 052` and `debug 0x2a` all set and display `0x002a` on the accepted IP45 path.

Negative values are parsed as signed 32-bit modular values before the setting path masks them to 16 bits. `debug -1` therefore reaches the handler as `0xffffffff` and displays `0xffff` on IP45. Likewise `debug 65536` or `debug 0x10000` displays `0x0000`. These are static branch conclusions.

The decoder's disassembly checks the unsigned 32-bit quotient and remainder before accumulating each digit. On overflow it records conversion error 34 and saturates the returned value to `0xffffffff`, then continues consuming numeric digits. Since the common parser checks token consumption but never checks that error, a wholly numeric overflow such as `debug 4294967296` appears to dispatch and display `0xffff` on IP45. A suffix such as `debug 12junk` or a wholly nonnumeric value such as `debug z` fails token consumption and does not invoke the setting handler. The parser reports an invalid-arguments status for a recognized command and returns a separate status for an unrecognized command. Exact transport status and console rendering of malformed requests are not settled by this note. For the empty hexadecimal prefix `debug 0x`, the decoder moves past the prefix, finds no digit, and leaves the end pointer at the end of the token (`0xffe7cd9e`–`0xffe7cda4`, `0xffe7ce38`–`0xffe7ce5a`). The parser's whole-token test therefore appears to accept zero and set/display `0x0000` on IP45. This is a static inference. An actual response exchange must settle transport status and rendering. Whitespace edge cases remain unresolved.

## `reboot_l1` and settings restoration

The no-argument restart command entry at body offset `0x1fe84` selects the restart branch at `0xffe3b3fc`. That branch calls the controller restart path (`0xffe56654`) with normal reboot mode and a zero alternate-image address. It does not invoke the settings validator or backup recovery directly. Other branches of the shared handler select particular flash images or report image status. A failed image validation can return a failure line without restarting. These are distinct from the no-argument operator request.

Before the no-argument branch reaches the restart call, it attempts a 100-unit task wait (`0xffe3b3fc`, helper `0xffe40e54`). That helper waits only with a valid current task context. Otherwise it returns immediately. The units are software task-clock units. Static code does not establish elapsed wall time or when, if ever, an operator receives a reply.

The restart path first records reboot state and calls lower reset support. If device control bit 7 is clear, it then executes ColdFire `HALT` (`0xffe566bc`). Only after that conditional branch does normal reboot mode write the boot address and its complement in controller RAM (`0xffe566c4`–`0xffe566e0`). The path then stays in a loop pending reset. One later helper called by that path is a no-effect stub in this selected image. The static image therefore does not establish which hardware event actually drives reset, the timing, or whether the operator sees a successful reply before the controller disappears. Calling this a verified watchdog reboot would overstate the evidence.

On a normal transition back through the selected reset vector, startup (`0xffe10570`, reached from `0xffe00400`) invokes settings validation at `0xffe5a758`. Invalid primary settings trigger backup recovery at `0xffe5a188`. If both records are unusable, defaults are installed. There is no observed `reboot_l1`-specific skip around this startup check. This is a conditional conclusion: the command itself does not restore settings, but a normal subsequent boot through the selected image does. A different boot image or a hardware reset path that does not re-enter this startup would need separate evidence.

## What would settle the remaining behavior

A focused command exchange should compare raw task-3 response status and text fields for plain `debug`, target-qualified `debug`, decimal/octal/hex values, `-1`, `65536`, `4294967296`, a nonnumeric token, a trailing suffix, and `0x` without digits. For restart, record whether the controller produces a reply, which image resumes, and whether a deliberately invalid primary/valid backup settings pair survives `reboot_l1`.
