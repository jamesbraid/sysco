# elsc_nvram

Byte addresses inside block 7 (0x700 to 0x7ff) of the module system controller's non-volatile store, the controller-reserved part that the host-side request set reaches. The marker byte at 0x700 and the module number at 0x708 are already read from the host driver (formats/elsc_host/format.toml); the offsets here are the other bytes of the block as published in the public Linux SN driver history. None of them is confirmed against a shipped image yet.

Source: public Linux 2.5.0 SN controller driver (arch/ia64/sn/io/l1_command.c); IP27 PROM Technical Reference 108-0170-001, printed page 4-22; no image address recorded.

The values are in [registers.toml](registers.toml), each graded and sourced, and the generated reference page is [registers.md](registers.md).

## Used by

- [Origin 2000 module management](../../platforms/ip27/module-management.md)
