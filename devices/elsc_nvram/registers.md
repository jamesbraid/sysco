<!-- generated from devices/elsc_nvram/registers.toml by defs-gen - do not edit, change the source -->

# configuration block of the module controller's non-volatile store

Byte addresses inside block 7 (0x700 to 0x7ff) of the module system controller's non-volatile
store, the controller-reserved part that the host-side request set reaches. The marker byte at
0x700 and the module number at 0x708 are already read from the host driver (formats/elsc_host/format.toml);
the offsets here are the other bytes of the block as published in the public Linux SN driver
history. None of them is confirmed against a shipped image yet.

Unit `elsc_nvram`, macro prefix `ELSCNV`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `CFG_CELL` | 0x70d | inferred |  | cell number |
| `CFG_CLUSTER` | 0x70c | inferred |  | cluster number |
| `CFG_CONTROLLER` | 0x707 | inferred |  | controller configuration byte; bit 0 holds the fans high, bit 1 enables automatic power-up |
| `CFG_CONTROLLER_AUTO_POWER_BIT` | 0x1 | inferred |  | bit index of the automatic power-up flag in CFG_CONTROLLER |
| `CFG_CONTROLLER_FANS_HIGH_BIT` | 0x0 | inferred |  | bit index of the fans-held-high flag in CFG_CONTROLLER |
| `CFG_DOMAIN` | 0x70b | inferred |  | domain number; a clock-select default may be written here by the controller's own initialization, and the meaning of that value is unsettled |
| `CFG_PARTITION` | 0x70a | inferred |  | partition number |
| `CFG_PASSWORD` | 0x701 | inferred |  | first of four bytes holding the controller password; default text none |
| `CFG_PASSWORD_LEN` | 0x4 | inferred |  | password length in bytes |
| `CFG_RESERVED_FIRST` | 0x70e | inferred |  | first controller-reserved byte with no published use; ends at 0x7ff |
| `CFG_SELFTEST` | 0x709 | inferred |  | self-test flags, two bits per node board |
| `CFG_SWITCH_PHYSICAL_XOR` | 0x706 | inferred |  | physical debug-switch XOR override |
| `CFG_SWITCH_VIRTUAL_XOR` | 0x705 | inferred |  | virtual debug-switch XOR override |
