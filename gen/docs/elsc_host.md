# two-wire addresses the host driver of the Origin 2000 module controller uses

<!-- generated from defs/elsc_host.toml by defs-gen - do not edit, change the source -->

Two-wire device addresses the host driver of the Origin 2000 module controller uses: what the
host-side driver for the module system controller (the MSC, also called ELSC) passes as the
device argument of its two-wire transfers, and how it builds the one that varies. Everything here
is read from the host driver object; the controller's own firmware is not held, so what sits
behind each address is a reading, not a finding. The device argument is a 7-bit address when the
transfer routine takes it as one; the routine is in the kernel, not in the object, so that form
is inferred.

Unit `elsc_host`, macro prefix `ELSC`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `MAILBOX_SLAVE` | 0x8 | established |  | established: the argument in the call; 7-bit form inferred |
| `NVRAM_PAGES` | 0x8 | established |  | established: addresses 0 to 0x7ff |
| `NVRAM_PAGE_BITS` | 0x7 | established |  | established |
| `NVRAM_PAGE_LOW_BIT` | 0x8 | established |  | established: address bits 10:8 select the page |
| `NVRAM_SLAVE_BASE` | 0x50 | established |  | established |
| `UST_SLAVE` | 0x20 | established |  | established for the call; meaning unknown |
