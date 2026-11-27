# The L2 utilities and network helpers in the Linux L1 image (1.62.1)

Read from the `Linux L1 image` of the Altix container `l1-1.62.1.bin` (package 4, L1 1.62.1; hashes of the container and package are in [sn2-l1-1.62.1-packages.md](sn2-l1-1.62.1-packages.md)). The CramFS was unpacked with the repository's own CramFS reader (the functions behind [arm-linux-files.tsv](../../../generated/inventory/arm-linux-files.tsv)) and the programs listed here were read at the level of their imported and exported symbols, their printed texts and the entry point. Their routines were not decoded one by one, so a statement about what a routine does rests on its name and the texts around it and is graded inferred. Grades follow [AGENTS.md](../../../AGENTS.md). The `l1`, `l1cmd` and helper programs are read routine by routine in [arm-programs.md](arm-programs.md).

All programs are 32-bit little-endian ARM executables linked against glibc 2.3.5, with their symbol tables intact. The L2-side programs carry the same host-library code (`sn1SCParseCmdline`, `SysCfgConnInit`, `L2CfgMGetConsole`, `IRouterFreePacket`, `SocketMakeInetConnection`) that the IRIX host tools link, which is why the same routine names appear in the host-tool function tables.

| Program | Size | sha256 | Role |
|---|---:|---|---|
| `l2slit` | 21,544 | `81420343c428a65feb962c64f23ecf0595a64fdcec091b719a5f0441bca42a77` | scan-based link interconnect test of brick-to-brick cables, through an L2 |
| `l2tune` | 14,612 | `ae490a1766e3ccac018c44f773a26773e47db3aff6b0a711d30ec5f8147ad25a` | checks NL4 router tuning values on brick-to-brick links, through an L2 |
| `l2term` | 15,500 | `074d67cd094ccb0eecd2de4d928c21f6ad11a7675fb90b6bf0d00171292403b0` | console terminal client for an L2 |
| `l2dhcps` | 22,532 | `5700140f6493f60b8993211caa4d11a9f81ae4249bcd93a9c1cf69ac284d8cc7` | DHCP server and address assignment for the L2 network |
| `l1log` | 4,868 | `e4156aaa667c768d8ce7c138ab5a1351265776447d9c08a8fff7e039676f4dc7` | writes one line into the L1 log |
| `l2passwd` | 4,916 | `9f9c9834b035373723e6086e76958d0f002f4e87d00213aeb8321c641b3db1ab` | password-file maintenance |
| `ulisten` | 6,084 | `a57123f638387f99d05557429fd7f41c8cc4d1efaffe533b4cf8a1bbec0a8872` | prints packets arriving on one of the 16 `l1io_uart` devices |
| `netwatch` | 2,066 | `ed2e5871dbadd60d5be3487294899e015c2ee6aa112f3094d169731d3df0c677` | shell script that restarts a silent `eth0` |

## l2slit and l2tune

**Established** (the usage texts): `l2slit` takes an optional configuration file, an optional target `<rack>.<slot>[.<port>]`, `--l2 <L2>`, an optional `<sysco>` argument, `-m <mode>` to override the test mode, `-add <num>` to add to every rack number of the configuration file (only valid with a file) and `-e <max>` to limit the errors shown. Without a file it reads the expected configuration from the controllers; with one it reads it from the file (default name `default.scf`). It prints the test parameters (`slit_mode`, `max_err`, the file flag, the rack offset, the target), issues a `slit` command to the L2 and prints, per link, passes, skips and failures, then a summary of the failing links with the cable end names.

`l2tune` takes `[<sysco>] <max>`, `[<sysco>] <max> <file>` or `[<sysco>] <max> <targ>` and a trailing `-v`: it compares NL4 tuning values on each bit of the brick-to-brick connections and reports every difference greater than `<max>`. Established as the program's own description. The comparison routines are `dmcout_data_compare` and `dmcout_data_display` and the text form of a summary is `dmcdata_summary_to_text`. Inferred: the tuning values are read through the controllers, as the usage text says, using the same expected-configuration code as `l2slit`.

**Inferred** (shared symbols): both programs build the system description with `scdata_load_from_sysco`, `scdata_load_from_file` and `scdata_create_for_brick`, find bricks and ports with `scdata_find_brick` and `scdata_find_port`, and check connections with `extcon_get` and `extcon_is_valid_brick`. `l2slit` additionally has the low-level link command path (`llpcmd_init`, `llpcmd_dump`, `sys_issue_cmd`, `sys_get_prompt`, `read_from_socket_w_poll`) and prints `LLPCMD REQUEST:` and `LLPCMD RESPONSE:` in debug mode. The scan command parser is `snl3sc_process_scancmd` (2,252 bytes in both). Unknown: the numeric meaning of the test mode value and of the `file_flag` and `ver_mode` fields. Not tried: decoding `snl3sc_process_scancmd`. Unverified.

## l2term

**Established** (texts): `usage: %s [-syscon <partition>] [sysco options]`. It connects to an L2 over TCP, forwards the terminal in both directions and ends with `<ctrl-]>` then `q`. On a lost connection it prints that it will retry and gives up with an error if it cannot reconnect. `-syscon` (or `-s`) starts it in system-console mode: it asks the configuration for the console of the given partition (`L2CfgMGetConsole`), prints which L2 holds it and redirects the connection there, and prints a warning when the configuration shows no console, no console for that partition, or when the request to enter system console mode is denied. A direct serial connection is refused. It saves and restores the terminal settings and can print them (`iflag`, `oflag`, `cflag`, `lflag`, speeds). `/usr/bin/l2term` in the same image is a 249-byte wrapper. The wrapper's text was not read. Unverified for the wrapper.

## l2dhcps

**Established** (texts): at start it configures the network interface itself (hardware address and address queries, interface flags, address, netmask and broadcast, with an alias address), then chooses a role. When it finds no DHCP server it either assigns an address manually, skips the start-up when an NVRAM setting disables the server, or starts a local DHCP server "to serve L3 clients". When the primary DHCP server is an L2 it starts a backup server (again unless an NVRAM setting disables it); when the primary is not an L2 controller it starts no backup. The server daemon forks, binds, handles the `DHCPDISCOVER`, `DHCPOFFER` and `DHCPACK` messages and disables itself after repeated `select` or `recvfrom` errors; it reports when no dynamic address is left. Which setting numbers the two NVRAM tests read was not decoded. Unknown. Not tried: finding the setting reads in the program.

## Small programs

- **`l1log`:** `usage: %s <L1 log message, in quotes>`. Opens `/dev/l1log`, waits for it to be writable and writes the message; it reports that the L1 does not appear to be reading when the wait fails. Established from the texts. The L1 side is the `l1log` pipe in the `l1` program, read in [arm-programs.md](arm-programs.md).
- **`l2passwd`:** copies `/etc/passwd` to `/etc/passwd.bak` before it edits it, removes the backup afterwards, and replaces `/etc/.rhosts` with a copy of `/rhosts.allow`. Established from the command strings. How it chooses the new password was not read. Unverified.
- **`ulisten`:** `usage: %s [ -ppp ] <uart# 0-15>`. Opens `/dev/l1io_uart<N>` for reading and writing, optionally sets the PPP mode on both directions, and prints `Pkt received: %d bytes` for each read. Established from the texts; the frame decoding was not read.
- **`netwatch`:** the script text is the program. Every 11 seconds, after a 30-second start delay, it reads `ifconfig eth0`; when the interface is `RUNNING` and neither the receive nor the transmit packet count has changed since the last pass, it appends a warning and the outcome of a `tcpdump` that waits five seconds for one packet to `/work/netwatch.log` and takes `eth0` down and up again with its address and netmask. The script comments name the cause as an Ethernet driver that stops without an error and say an L2 normally sends a packet every 5 seconds. Established.

## What is not read

`webs` (131,056 bytes, `sha256 441e6c5b7b9f16d55b5927652c011ed9e2c7d95d0eaca9d09ab09548a27c2685`, started from `inittab` in runlevels 4 and 5 beside `l2`), `l2`, `l2setmac`, `l2staticip`, `uloop` and the six L1 shared libraries remain unread at the routine level. The libraries and `l1` are covered at the call-path level by the `arm-library-*` notes.
