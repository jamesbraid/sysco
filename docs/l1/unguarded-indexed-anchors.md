# Four indexed tables in Origin 300 L1 1.44.0

Each check permits values zero through `0xb2` and branches away for higher values. The 179 allowed values read a 358-byte signed-offset table at the jump anchor. This establishes four more data spans, totaling 1,432 bytes. It establishes no new executable bytes inside a Ghidra gap.

## Revision and address basis

Its body after the 44-byte prefix has 982,368 bytes and SHA-256 `9db44f3bb06ec90475035781766e1ca7e924f15a0ed215cf855c1f491275fdd1`. Addresses below use the body base `0xffe00000`. The second pair is offset `0x70000` from the first pair. The package section reading does not establish which copy runs on a fitted board.

## Local bounds and destinations

Immediately before each table read, the routine copies its 32-bit input index to the register used for the table, compares it with `0xb2`, and takes an unsigned-high branch around the table for any larger value. A negative 32-bit value is also higher under that unsigned comparison. No instruction changes the index between the comparison and the scaled table read. The read uses two bytes per index, sign-extends the chosen 16-bit offset, and jumps to the table anchor plus that offset. This is a local guard, so a caller-wide limit is not needed to bound the table extent. Which of the 179 allowed values occurs in a real firmware run remains unmeasured.

It does not rule out an indirect entry or prove that every possible path reaches the local comparison. The 358 bytes at each anchor are table data on the checked path. No conclusion about a longer table or alternate entry follows from this scan.

| Anchor and exact pinned gap | Entries and data SHA-256 | Signed offset range | Destinations |
|---|---|---|---|
| `0xffe58782`–`0xffe588e8` | 179 entries / 358 bytes. SHA-256 `d2e5b04ac68082409648dcad4e010c226ddb8d34740bcf773089bafe78dabb75` | 358 to 3,588 | 179 entries, 111 distinct decoded instruction starts from `0xffe588e8` to `0xffe59586` |
| `0xffe595b0`–`0xffe59716` | 179 entries / 358 bytes. SHA-256 `3613a1701af4fd52e3a8cc773be0bea96152701d836849095f9162a53c1bafdc` | 358 to 2,264 | 179 entries, 111 distinct decoded instruction starts from `0xffe59716` to `0xffe59e88` |
| `0xffec8782`–`0xffec88e8` | 179 entries / 358 bytes. Same hash as first row | 358 to 3,588 | 179 entries, 111 distinct decoded instruction starts from `0xffec88e8` to `0xffec9586` |
| `0xffec95b0`–`0xffec9716` | 179 entries / 358 bytes. Same hash as second row | 358 to 2,264 | 179 entries, 111 distinct decoded instruction starts from `0xffec9716` to `0xffec9e88` |

The end of each 358-byte table is its lowest computed target. The highest targets are the local branch-around destinations. Entry zero also targets the branch-around destination in each table. Forty-seven of 179 entries target it.

There are zero intersections. The rest retains its unknown code/data status. This reading does not classify the large early gap or any neighboring strings, tables or erased-looking spans.
