<!-- generated from formats/l2_router_requests/format.toml by defs-gen - do not edit, change the source -->

# request codes of the L2 router library

Request codes of the L2 controller router library. Codes are unique within the library.

Unit `l2_router_requests`, macro prefix `L2R`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `CMD` | 32 | established |  |

### Word `CMD`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 15:0 | `code` | established | chunk_first_flagged=12289, chunk_first=12290, chunk_next=12291 |  |  |
