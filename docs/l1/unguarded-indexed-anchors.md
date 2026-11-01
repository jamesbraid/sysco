# Four indexed data tables in Origin 300 L1 1.44.0

Two routines in the first bank (and their copies `0x70000` higher) end in a PC-relative indexed jump whose table sits straight after the jump. Each routine compares its index with `0xb2` first (unsigned; a negative index is also larger) and branches around the table for any larger value, so each table has 179 entries of one signed 16-bit offset each, 358 bytes. Established: the guard and the table extent, read in the listing, for the four tables below. Together they are 1,432 bytes of data inside the linear listing.

| Table | Entries | Data SHA-256 | Offset range | Destinations |
|---|---|---|---|---|
| `0xffe58782`–`0xffe588e8` | 179 | `d2e5b04ac68082409648dcad4e010c226ddb8d34740bcf773089bafe78dabb75` | 358 to 3,588 | 111 distinct targets, `0xffe588e8` to `0xffe59586` |
| `0xffe595b0`–`0xffe59716` | 179 | `3613a1701af4fd52e3a8cc773be0bea96152701d836849095f9162a53c1bafdc` | 358 to 2,264 | 111 distinct targets, `0xffe59716` to `0xffe59e88` |
| `0xffec8782`–`0xffec88e8` | 179 | same as the first | 358 to 3,588 | the first table's targets plus `0x70000` |
| `0xffec95b0`–`0xffec9716` | 179 | same as the second | 358 to 2,264 | the second table's targets plus `0x70000` |

The first two routines are the settings writer (`0xffe58758`) and reader (`0xffe59590`) used by the `debug` command ([debug-console.md](debug-console.md)): each switches on a setting index from zero to `0xb2`. The lowest target of each table is the first byte after it, and the highest is the branch-around destination (`0xffe59586` and `0xffe59e88`); 47 of the 179 entries in each table, including entry zero, point at that destination. The listing decodes these table bytes as instructions (the `ori` and `bclr` lines in `l1.lst` around `0xffe58782` and `0xffe595b0`); they are data.

Unknown: whether any other path enters the routines past the guard. No indirect entry is excluded by this reading.
