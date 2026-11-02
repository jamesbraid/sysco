# Host tool command lines: `l1cmd`, `l2cmd` and `flashsc`

What the IRIX-side tools accept and what they send, read from the IRIX 6.5.30 files (`usr/sbin/l1cmd`, `usr/sbin/l2cmd`, `usr/sbin/sysco/flashsc`; N32, MIPS-III, big-endian ELF). The three programs link the same System Controller client library, which parses the shared options and does the transport. Grades follow [AGENTS.md](../AGENTS.md).

## The shared option set

The client library's usage text (printed by the library's usage routine at `0x10006490` in `l1cmd`, read in the string table) lists these options (established):

| Option | Argument | Meaning in the usage text |
|---|---|---|
| `--l2` | `<host>` | host name or IP address of the L2 to connect to |
| `--sc` | none | connect through the default system controller (lowest rack and slot) |
| `--scdev` | `<device>` | connect through the named system controller device |
| `--interface`, `--if` | `<iface>` | network interface (for example `eth0`) of the L2 network |
| `--ssn` | `<serial>[:<rack>]` | serial number of the system to connect to |
| `--sysname` | `<name>[:<rack>]` | system name of the system to connect to |
| `--pw` | `<password>` | L2 password |
| `--irtr` | none | debug: show router packets sent and received |
| `--serial` | `<host>:<port>` | Ethernet-attached serial port (a terminal server) |
| `--dev` | `<device name>` | local serial port device |

The parser's string table also holds `--rack`, `--copy`, `--quiet` and `--debug` (established that the strings exist; the argument syntax and behaviour of `--rack`, `--quiet` and `--debug` were not read). `--copy <n>` is accepted for 0 through 128 and biases every network port by `n` (the message "using copy %d (biasing all network ports by %d)"). An `--ssn` or `--sysname` value may carry a rack number after a colon, and a bad one is rejected with a "bad rack number" message. The message "--serial/--dev options (direct serial connection) not supported" is present in all three programs, and the `flashsc` usage names `--serial` and `--dev` for `-l2recover` (established from the strings; which tool reaches the rejection was not read).

How `l1cmd` finds its target when no option names one (strings, inferred for the order): it lists the local hardware graph with a shell `ls` of `/hw/module/*c*/L1/controller`; if nothing is there it prints that no local L1 controller was found, falls back to an L2 at `localhost` "as a last resort", and then to L2 discovery on the network (a request sent to each interface's broadcast address, named `l2find` in the messages). With several L2 controllers found it refuses and asks for `--l2`, `--sysname` or `--ssn`.

## `l1cmd` (6.5.30)

Its `main` (`0x10006240` to `0x10006484`) was read in full (established):

1. It saves `argv[0]` as the program name and calls the library's option parser with `argc` and `argv`; a negative result prints "error while parsing library command arguments" and exits with -1.
2. If fewer than two words remain after option parsing, it prints the usage text and exits with -1.
3. It concatenates the remaining words, each followed by one space, into a 1024-byte buffer.
4. It initialises the library with the program name; a negative result exits with -1.
5. It sets up a destination word `0x0ffff003` and cuts the buffer at each semicolon; for each piece it calls the library's text-command routine (`sn1SCTextCmd`, `0x10009650`) with the destination, the piece and a reply pointer. A non-empty reply is written to the stream at offset 16 of the C library's stream table when the call returned zero and to the one at offset 32 otherwise (the usage text goes to offset 32; reading those as standard output and standard error is inferred from the 16-byte entry size). The exit status is the last call's result.

So `l1cmd` carries no command table of its own: whatever follows the options is sent as text, and the L1's own command parser decides ([l1/debug-task.md](l1/debug-task.md) describes that parser). The text-command routine's body (request construction, the constant argument 30 at `0x100097d8`, the reply decode) was read only as far as its calls (`sn1SCReq`, `sn1SCCmd`); the wire format it builds is not described here.

## `l2cmd` (6.5.30)

`l2cmd` has the same loaded segments as `l1cmd`, byte for byte, except one instruction: the destination word is built with `lui a2,0x1fff` instead of `0x0fff` (file offset `0x6392`), so the destination is `0x1ffff003` (established, from a byte comparison of the first `0x28000` bytes and of the data segment). Five more differing bytes are in the ELF header's section-header offset, count and string-table index, one is the link timestamp in the dynamic section (the two files were linked two seconds apart), and one is in the dynamic symbol table (offset `0x2c67`, not interpreted). The 1.44.0 Origin 300 L1 image loads the same word `0x1ffff003` at `0xffe14ab4`, in the routine beside the helper the `*` broadcast command uses (`0xffe14acc`, [l1/debug-task.md](l1/debug-task.md)); that the destination word is an address of the same kind is inferred from the value alone.

The shipped `l2cmd` file is cut short: its section-header table starts at `446,168` and has 39 entries of 40 bytes, which ends past the file's 446,880 bytes, so `readelf` and `objdump` cannot read its sections. The program headers and the loaded segments are intact, which is why the comparison above holds. The extra size lies after the loaded segments; that it is debug information is inferred from the larger header count.

## `flashsc` (6.5.30)

Read from the usage and message strings (established that the strings exist; the code behind each option was not read in 6.5.30; the SG0007149 build is described in [l1/package-container.md](l1/package-container.md)):

- Usage: `flashsc [-p [<maxgrp>]] [-f] [sysco options] <bin image> <target>`, and `flashsc -version|-v|-vv <bin image>`.
- `-p [<maxgrp>]`: flash controllers in parallel instead of one at a time, optionally at most `maxgrp` at once; a bad value prints "invalid <maxgrp> argument for -p".
- `-f`: force the update, skipping the version check. The string table also has `-forceall`.
- `-l2recover`: emergency L2 flash recovery over a serial connection chosen with `--serial` or `--dev`; it needs an L2 firmware image in the binary and cannot update several bricks at once over serial.
- `<target>`: `<rack>.<slot>`, a list of those, `all` or `local` (for an L2). The string table also holds the words `router`, `compute`, `other`, `l2local` and `local`, the options `-router`, `-io`, `-other`, `-test`, `-manual`, `-new`, `-image`, `-boot`, `-bootldr` and `-compute`, and the messages that go with `-image` ("invalid image (%d) selected with -image option") and `-manual` ("MANUAL FLASH IMAGE SELECTED").
- `-n`: named in two notices that say it is forced when a system mixes legacy and newer bricks, and that a lone legacy brick that will join newer hardware needs it; it makes the legacy brick take firmware that can talk to the newer hardware.
- An L1 flash binary must hold both an "A" and a "B" image, with relocation tables, or the tool refuses it ("does not contain both the A and B images"); a multi-image binary lists its images (`Image %d:`).
- The banner reads "System Controller Flash Utility - Version 1.4.1"; `-version` prints "Supports flash binary rev. %d".

The update protocol itself (the requests `flashsc` sends to a controller, and the checks the controller makes) is not read from this build.
