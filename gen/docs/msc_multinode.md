# constants of the multi-node console and master election

<!-- generated from defs/msc_multinode.toml by defs-gen - do not edit, change the source -->

Constants that the multi-node module system controller arrangement uses: the hub network-interface
scratch registers the master election reads, the CPU-letter base of the console line prefix, and
the two-wire device family codes of the module and node records on the hub bus. Each is read from
the PROM and kernel reading described in the console note; none has been checked against the
images in this repository.

Unit `msc_multinode`, macro prefix `MSCMN`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `MODULE_RECORD_FAMILY` | 0x9 | inferred |  | two-wire device family of the hub-side module manufacturing record, enumerated on the node-local hub bus |
| `NI_SCRATCH_IDENTITY` | 0x0 | inferred |  | hub network-interface scratch register 0: board identity (NIC) and partition id, written first thing at power-on |
| `NI_SCRATCH_STATUS` | 0x1 | inferred |  | hub network-interface scratch register 1: node identity and status fields; the node number is added after arbitration |
| `NODE_RECORD_FAMILY` | 0xb | inferred |  | two-wire device family of the node and PIMM record, enumerated on the same node-local hub bus as the module record |
| `SLICE_LETTER_BASE` | 0x41 | inferred |  | console line letter for CPU slice 0 within a node; slice n prints this value plus n; the letter does not identify the node or module |
