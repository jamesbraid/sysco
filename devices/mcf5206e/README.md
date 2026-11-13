# mcf5206e

On-chip registers of the ColdFire MCF5206e that the L1 firmware uses. Offsets are from the module base register and are given as constants; the registers are not modelled as a register block. Only registers the L1 firmware touches are listed. Widths marked inferred follow the chip manual's layout because the firmware access size was not separately read. The reset values are not stated by any image, so they are unknown. Established means the firmware writes or tests the register as stated; inferred means the layout is the public manual's and was not re-read in an image.

Source: IRIX 6.5.30 l1.bin (L1 1.44.0) start-up code; MCF5206e manual layouts where graded inferred.

The values are in [registers.toml](registers.toml), each graded and sourced, and the generated reference page is [registers.md](registers.md).

## Used by

- [L1 to Bedrock hand-offs: the ordered events the firmware drives](../../platforms/bricks/l1-bedrock-handoffs.md)
- [L1 notes](../../software/firmware/l1/README.md)
- [Origin 300 L1 1.44.0: what the processor and board must provide](../../software/firmware/l1/board-boot-contract.md)
- [L1 conditional byte-copy and clear paths](../../software/firmware/l1/byte-status-paths.md)
- [Chunked request frames on the processor link (L1 side)](../../software/firmware/l1/chunked-request-frames.md)
- [Console command tables of the ColdFire L1 packages](../../software/firmware/l1/console-command-tables.md)
- [L1 console and log messages](../../software/firmware/l1/console-messages.md)
- [L1 pointer-relative controller write loops](../../software/firmware/l1/controller-write-loops.md)
- [L1 CPU features beyond plain ColdFire](../../software/firmware/l1/cpu-features.md)
- [L1 1.8.0 to 1.16.0 pointer-relative setup callers](../../software/firmware/l1/early-window-callers.md)
- [L1 firmware image layout: the 1.44.0 container, its ColdFire packages, how a controller picks and runs one, and what a C-brick needs](../../software/firmware/l1/image-layout.md)
- [Origin 300 L1 1.44.0: chunked requests, L2 attach and request 0x0ffff00e](../../software/firmware/l1/packet-chunks-and-l2-attach.md)
- [L1 pointer-relative setup and window-call paths](../../software/firmware/l1/window-callers.md)
