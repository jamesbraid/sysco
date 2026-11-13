<!-- generated from formats/l2_partition_requests/format.toml by defs-gen - do not edit, change the source -->

# request codes of the L2 partition task image

Request codes of the L2 controller partition task image. Codes are unique within the image.

Unit `l2_partition_requests`, macro prefix `L2P`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `CMD` | 32 | established |  |

### Word `CMD`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 15:0 | `code` | established | partition_for_brick=1, partition_environment=2, database_get=3, database_time=4, database_set=5, mode_set=6, park_request=7, release_parked=8 |  |  |
