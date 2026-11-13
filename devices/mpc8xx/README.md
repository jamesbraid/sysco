# mpc8xx

Start-up values of the PowerPC communications controller on the L2 board: the values the L2 reset code and its second stage write to the machine state register, the clock and reset registers, the serial-management channel parameter page and the PCI bridge. Register offsets are from the internal memory map base. The register and field names are the public Linux headers'; the firmware's own reading of each field was not decoded, so the layouts of the clock registers and the bridge are not given. A value is established when the image writes it as stated.

Source: IRIX 6.5.30 l2.bin (L2 1.44.0) reset code; register names from the public Linux 8xx_immap.h.

The values are in [registers.toml](registers.toml), each graded and sourced, and the generated reference page is [registers.md](registers.md).

## Used by

- [L2 board contract: the core, the memory map and the devices the shipped firmware expects](../../platforms/l2/board-contract.md)
- [L2 image: container, kernel and root file system](../../software/firmware/l2/image-layout.md)
- [L2: what the user-space programs need from the kernel and the board](../../software/firmware/l2/kernel-interface.md)
