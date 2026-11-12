# GIGAchannel cabinet system-controller status word, as IRIX and the BASEIO PROM read it

<!-- generated from defs/gigachannel_status.toml by defs-gen - do not edit, change the source -->

The 16-bit status word that the GIGAchannel expansion cabinet presents to the node it hangs off, read
by the IRIX kernel and by the BASEIO PROM. The word sits in the widget-8 window of
the Crossbow in that cabinet. Nothing in the held images writes it, so the bit meanings below are
the software's reading only. Every item is inferred. The reset value is unknown.

Unit `gigachannel_status`, macro prefix `GCSTATUS`.
Default source: shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `STATUS` | 16 | inferred | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation |
| `XBOW_ID` | 32 | inferred | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation |

### Word `STATUS`

Cabinet status word, read at STATUS_OFFSET within the widget-8 window of the node's Hub. Software only reads it. No software writes it in the images read, and what a write would do is unknown. Bits 7 to 15 have no meaning that software reads. After start-up the IRIX kernel examines only bits 0 to 5, and bit 6 only once, at start-up.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 6 | `redundant_supply` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | A redundant supply is fitted in the cabinet unit. The BASEIO PROM prints its redundant-supply line while it is set. The kernel records the unit's inventory only while it is set at start-up. |
| 5 | `manufacturing_mode` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Cabinet in manufacturing mode. The kernel prints its notice once per boot. |
| 4 | `redundancy_lost` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Backup power supply has taken over and redundancy is lost. The kernel prints the notice when it sees the bit newly set and never sees it clear again, so once per boot. The BASEIO PROM prints a lost-redundancy line while it is set. |
| 3 | `event_3` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | The kernel prints a level-4 notice on every poll while set and starts the orderly shutdown once per boot, the same as bit 0. Its text was not recorded. |
| 2 | `event_2` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | The kernel prints a level-4 notice on every poll while set. Its text was not recorded. |
| 1 | `event_1` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | The kernel prints a level-4 notice on every poll while set. Its text was not recorded. |
| 0 | `blower_failure` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Cabinet blower failure. The kernel prints the blower message on every poll while set and starts the orderly shutdown once per boot. |

### Word `XBOW_ID`

Identification word of the Crossbow, read through each node's Hub at the widget-0 offset below. The kernel reads it at start-up to decide whether the node has a cabinet.

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 27:12 | `part` | inferred |  | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Part field. The kernel accepts only zero; the BASEIO PROM also accepts the value in the constant below. How the PROM extracts the value from the word was not traced. |

## Constants

| Constant | Value | Grade | Source | Description |
|---|---|---|---|---|
| `STATUS_OFFSET` | 0xe00000 | inferred | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Offset of the status word within the widget-8 window of the node's Hub I/O window. |
| `WIDGET0_ID_OFFSET` | 0xe0000004 | inferred | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Offset of the Crossbow identification word within the node's Hub I/O window. |
| `XBOW_PART_KERNEL` | 0x0 | inferred | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Part field the IRIX kernel accepts as a Crossbow. |
| `XBOW_PART_PROM` | 0xd000 | inferred | shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation | Second part field the BASEIO PROM accepts as a Crossbow. |
