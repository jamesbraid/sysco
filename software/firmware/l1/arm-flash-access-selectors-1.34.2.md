# ARM flash access and selector paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/flash-access-selectors-facts.txt) retain all tool sections.

## `flash_init`

- `flash_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae980 to 0xaeaa0.
  Established. A zero word at absolute `0x27eb30` takes the edge into the longer path, while nonzero
  falls through to a stack store of `0x0` (`ldr` at 0xae998, `beq` at 0xae9a0, `str` at
  0xae9a8). The longer path includes a call to `0x18bb4` with candidates `0x124f0e5`, `0x1`,
  `0x3b6`, `[0x1157a0]` (`bl` at 0xae9fc). That result is stored as a word at absolute `0x1157a0`
  (`str` at 0xaea08). A signed-nonnegative load from that address takes the edge to a word store
  of `0x1` at absolute `0x27eb30`, while a negative value falls through into further calls (`bge` at
  0xaea14, `str` at 0xaea74). The numeric callees’ resource types and the merged return value remain
  unknown (`ldmdb` at 0xaea84).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae990 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xae994 | load | 4 | `abs 0xaea88` | `-` |
| `ldr` at 0xae998 | load | 4 | `abs 0x27eb30` | `-` |
| `str` at 0xae9a8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xae9b4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xae9b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xae9c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xae9d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xae9d8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xae9e0 | load | 4 | `abs 0xaea8c` | `-` |
| `ldr` at 0xae9e4 | load | 4 | `abs 0x1157a0` | `-` |
| `ldr` at 0xae9f0 | load | 4 | `abs 0xaea90` | `-` |
| `ldr` at 0xae9f8 | load | 4 | `abs 0xaea94` | `-` |
| `ldr` at 0xaea04 | load | 4 | `abs 0xaea8c` | `-` |
| `str` at 0xaea08 | store | 4 | `abs 0x1157a0` | `ret(0x18bb4)` |
| `ldr` at 0xaea0c | load | 4 | `abs 0x1157a0` | `-` |
| `ldr` at 0xaea2c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaea38 | load | 4 | `abs 0xaea98` | `-` |
| `str` at 0xaea3c | store | 4 | `sp -0x2c` | `0x108584` |
| `ldr` at 0xaea40 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xaea44 | store | 4 | `sp -0x28` | `[ret(0x188b4)]` |
| `str` at 0xaea48 | store | 4 | `sp -0x24` | `ret(0x18890)` |
| `ldr` at 0xaea50 | load | 4 | `abs 0xaea9c` | `-` |
| `str` at 0xaea64 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaea6c | load | 4 | `abs 0xaea88` | `-` |
| `str` at 0xaea74 | store | 4 | `abs 0x27eb30` | `0x1` |
| `str` at 0xaea7c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaea80 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaea84 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaea84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaea84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaea84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae9cc | `(unknown callee) 0xae4e8` | `?` | `?` | `?` | `?` |
| `bl` at 0xae9fc | `(unknown callee) 0x18bb4` | `0x124f0e5` | `0x1` | `0x3b6` | `[0x1157a0]` |
| `bl` at 0xaea18 | `(unknown callee) 0x188b4` | `ret(0x18bb4)` | `?` | `ret(0x18bb4)` | `[0x1157a0]` |
| `bl` at 0xaea24 | `(unknown callee) 0x188b4` | `ret(0x188b4)` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaea30 | `(unknown callee) 0x18890` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaea5c | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0xd9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xae9a0 | `[0x27eb30] == 0x0` | `[0x27eb30] != 0x0` |
| `ble` at 0xae9c0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xae9ec | `[0x1157a0] >= 0x0 (signed, 32-bit)` | `[0x1157a0] < 0x0 (signed, 32-bit)` |
| `bge` at 0xaea14 | `[0x1157a0] >= 0x0 (signed, 32-bit)` | `[0x1157a0] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaea84 | `r0` | `?` |

## `flash_lock`

- `flash_lock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeaa0 to 0xaeb54.
  Established. A signed-negative word at absolute `0x1157a0` takes the bypass edge, while a
  nonnegative value falls through to the call to `0x18bfc` (`ldr` at 0xaeab4, `blt` at 0xaeabc,
  `bl` at 0xaead0). That call has candidates `[0x1157a0]`, `0x115788`, `0x2`, `0x1157a0` (`bl` at
  0xaead0); its signed-nonnegative result takes the edge to a word store of `0x0` at `sp -0x18`,
  while a negative result falls through to additional calls (`bge` at 0xaeadc, `str` at 0xaeb38,
  `bl` at 0xaeae0, `bl` at 0xaeaec, `bl` at 0xaeaf8, `bl` at 0xaeb24). The lock mechanism and return
  value are not established (`ldmdb` at 0xaeb40).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaeab0 | load | 4 | `abs 0xaeb44` | `-` |
| `ldr` at 0xaeab4 | load | 4 | `abs 0x1157a0` | `-` |
| `ldr` at 0xaeac0 | load | 4 | `abs 0xaeb44` | `-` |
| `ldr` at 0xaeac4 | load | 4 | `abs 0x1157a0` | `-` |
| `ldr` at 0xaeac8 | load | 4 | `abs 0xaeb48` | `-` |
| `ldr` at 0xaeaf4 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaeb00 | load | 4 | `abs 0xaeb4c` | `-` |
| `str` at 0xaeb04 | store | 4 | `sp -0x24` | `0x1085c8` |
| `ldr` at 0xaeb08 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xaeb0c | store | 4 | `sp -0x20` | `[ret(0x188b4)]` |
| `str` at 0xaeb10 | store | 4 | `sp -0x1c` | `ret(0x18890)` |
| `ldr` at 0xaeb18 | load | 4 | `abs 0xaeb50` | `-` |
| `str` at 0xaeb2c | store | 4 | `sp -0x18` | `?` |
| `str` at 0xaeb38 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xaeb3c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaeb40 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaeb40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaeb40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaeb40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaead0 | `(unknown callee) 0x18bfc` | `[0x1157a0]` | `0x115788` | `0x2` | `0x1157a0` |
| `bl` at 0xaeae0 | `(unknown callee) 0x188b4` | `ret(0x18bfc)` | `?` | `?` | `ret(0x18bfc)` |
| `bl` at 0xaeaec | `(unknown callee) 0x188b4` | `ret(0x188b4)` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaeaf8 | `(unknown callee) 0x18890` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaeb24 | `(unknown callee) 0x1eed0` | `0x4` | `0x108464` | `0xea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xaeabc | `[0x1157a0] < 0x0 (signed, 32-bit)` | `[0x1157a0] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xaeadc | `ret(0x18bfc) >= 0x0 (signed, 32-bit)` | `ret(0x18bfc) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaeb40 | `r0` | `?` |

## `flash_unlock`

- `flash_unlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeb54 to 0xaec08.
  Established. The call to `0x18bfc` uses candidates `[0x1157a0]`, `0x115798`, `0x1`, `0x1157a0`
  (`bl` at 0xaeb84). The prior branch takes its bypass for `[0x1157a0] < 0x0` signed and
  otherwise falls through to that call (`blt` at 0xaeb70). The call’s signed-nonnegative result
  takes the edge to a word store of `0x0` at `sp -0x18`, while a negative result falls through
  into calls to `0x188b4`, `0x18890` and `0x1eed0` (`bge` at 0xaeb90, `str` at 0xaebec, `bl` at
  0xaeb94, `bl` at 0xaeba0, `bl` at 0xaebac, `bl` at 0xaebd8). The callee’s release operation is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaeb64 | load | 4 | `abs 0xaebf8` | `-` |
| `ldr` at 0xaeb68 | load | 4 | `abs 0x1157a0` | `-` |
| `ldr` at 0xaeb74 | load | 4 | `abs 0xaebf8` | `-` |
| `ldr` at 0xaeb78 | load | 4 | `abs 0x1157a0` | `-` |
| `ldr` at 0xaeb7c | load | 4 | `abs 0xaebfc` | `-` |
| `ldr` at 0xaeba8 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaebb4 | load | 4 | `abs 0xaec00` | `-` |
| `str` at 0xaebb8 | store | 4 | `sp -0x24` | `0x108604` |
| `ldr` at 0xaebbc | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xaebc0 | store | 4 | `sp -0x20` | `[ret(0x188b4)]` |
| `str` at 0xaebc4 | store | 4 | `sp -0x1c` | `ret(0x18890)` |
| `ldr` at 0xaebcc | load | 4 | `abs 0xaec04` | `-` |
| `str` at 0xaebe0 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xaebec | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xaebf0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaebf4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaebf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaebf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaebf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaeb84 | `(unknown callee) 0x18bfc` | `[0x1157a0]` | `0x115798` | `0x1` | `0x1157a0` |
| `bl` at 0xaeb94 | `(unknown callee) 0x188b4` | `ret(0x18bfc)` | `?` | `?` | `ret(0x18bfc)` |
| `bl` at 0xaeba0 | `(unknown callee) 0x188b4` | `ret(0x188b4)` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaebac | `(unknown callee) 0x18890` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaebd8 | `(unknown callee) 0x1eed0` | `0x4` | `0x108464` | `0xf5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xaeb70 | `[0x1157a0] < 0x0 (signed, 32-bit)` | `[0x1157a0] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xaeb90 | `ret(0x18bfc) >= 0x0 (signed, 32-bit)` | `ret(0x18bfc) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaebf4 | `r0` | `?` |

## `flash_offset`

- `flash_offset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeca0 to 0xaed2c.
  Established. The call to `0xaec08` has entry r0 through r3 as candidates (`bl` at 0xaecb8). A
  signed-nonnegative result takes the edge into a second comparison, while a negative result falls
  through to an unknown-value word store at `sp -0x20` (`bge` at 0xaeccc, `str` at 0xaecd4). The
  second branch compares unsigned operands that are both unknown, following its edge for less-than and
  falling through for greater-than-or-equal (`blo` at 0xaecf4). The final return reports `[sp-32]`
  in r0 (`ldmdb` at 0xaed20). The indexed address calculation is not resolved to a proven flash
  offset.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaecb0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaecb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaecc0 | store | 4 | `sp -0x1c` | `ret(0xaec08)` |
| `ldr` at 0xaecc4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaecd4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaecdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaece4 | load | 4 | `abs 0xaed24` | `-` |
| `ldr` at 0xaece8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaecec | load | 4 | `(unknown)` | `-` |
| `str` at 0xaecfc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaed04 | load | 4 | `abs 0xaed28` | `-` |
| `ldr` at 0xaed08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaed0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaed10 | load | 4 | `(unknown)` | `-` |
| `str` at 0xaed18 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaed1c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaed20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaed20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaed20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaecb8 | `(unknown callee) 0xaec08` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xaeccc | `ret(0xaec08) >= 0x0 (signed, 32-bit)` | `ret(0xaec08) < 0x0 (signed, 32-bit)` |
| `blo` at 0xaecf4 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaed20 | `r0` | `[sp-32]` |

## `flash_read_addr`

- `flash_read_addr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaed2c to 0xaee40.
  Established. A call to `0xaeca0` has first candidate `[sp-20]`, second and third unknown, and
  fourth `ret(0xaec08)` (`bl` at 0xaed6c). A later call to `0xd6ae0` uses first candidate unknown,
  second and third `sp-112`, and fourth `[sp-120]` (`bl` at 0xaed98); its signed-nonnegative result
  takes the bypass edge, while a negative result falls through to the calls that include
  `0x188b4` and `0x1e71c` (`bge` at 0xaeda4, `bl` at 0xaeda8, `bl` at 0xaedb4). The final return
  reports `[sp-124]` (`ldmdb` at 0xaee28). Neither the returned address mapping nor the called
  routine’s data-record type is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaed3c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaed40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaed4c | store | 4 | `sp -0x78` | `ret(0xaec08)` |
| `ldr` at 0xaed50 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xaed60 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xaed68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaed74 | store | 4 | `sp -0x74` | `ret(0xaeca0)` |
| `ldr` at 0xaed78 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xaed84 | load | 4 | `abs 0xaee2c` | `-` |
| `ldr` at 0xaed88 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xaed90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaedb0 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaedbc | load | 4 | `abs 0xaee30` | `-` |
| `str` at 0xaedc0 | store | 4 | `sp -0x88` | `0x108644` |
| `ldr` at 0xaedc4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xaedc8 | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0xaedcc | store | 4 | `sp -0x80` | `ret(0x1e71c)` |
| `ldr` at 0xaedd4 | load | 4 | `abs 0xaee34` | `-` |
| `ldr` at 0xaedd8 | load | 4 | `abs 0xaee38` | `-` |
| `str` at 0xaede8 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xaedf0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xaedf4 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xaee04 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xaee0c | load | 4 | `abs 0xaee3c` | `-` |
| `ldr` at 0xaee10 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xaee14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaee18 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xaee20 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xaee24 | load | 4 | `sp -0x7c` | `-` |
| `ldmdb` at 0xaee28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaee28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaee28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaed44 | `(unknown callee) 0xaec08` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xaed6c | `(unknown callee) 0xaeca0` | `[sp-20]` | `?` | `?` | `ret(0xaec08)` |
| `bl` at 0xaed98 | `(unknown callee) 0xd6ae0` | `?` | `sp-112` | `sp-112` | `[sp-120]` |
| `bl` at 0xaeda8 | `(unknown callee) 0x188b4` | `ret(0xd6ae0)` | `?` | `?` | `ret(0xd6ae0)` |
| `bl` at 0xaedb4 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaede0 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x12d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xaed58 | `ret(0xaec08) >= 0x0 (signed, 32-bit)` | `ret(0xaec08) < 0x0 (signed, 32-bit)` |
| `ble` at 0xaed80 | `[sp-120] <= 0x0 (signed, 32-bit)` | `[sp-120] > 0x0 (signed, 32-bit)` |
| `bge` at 0xaeda4 | `ret(0xd6ae0) >= 0x0 (signed, 32-bit)` | `ret(0xd6ae0) < 0x0 (signed, 32-bit)` |
| `bhi` at 0xaedfc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaee28 | `r0` | `[sp-124]` |

## `flash_write_addr`

- `flash_write_addr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaee40 to 0xaeedc.
  Established. The body includes calls to `0xaec08` with entry r0 through r3 as candidates, then to
  `0xaeca0` with first candidate `[sp-20]` (`bl` at 0xaee58, `bl` at 0xaee80). Another call to
  `0xaec08` has candidates `[0x284070]`, unknown, unknown, `0x284060` (`bl` at 0xaee94). A following
  inequality branch takes the edge for `[sp-28] != ret(0xaec08)`, while equality falls through
  to an unknown-value stack word store (`bne` at 0xaeea4, `str` at 0xaeeac). The return reports
  `[sp-32]` (`ldmdb` at 0xaeed0). Which mapped addresses permit writes remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaee50 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaee54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaee60 | store | 4 | `sp -0x1c` | `ret(0xaec08)` |
| `ldr` at 0xaee64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaee74 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaee7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaee88 | store | 4 | `sp -0x18` | `ret(0xaeca0)` |
| `ldr` at 0xaee8c | load | 4 | `abs 0xaeed4` | `-` |
| `ldr` at 0xaee90 | load | 4 | `abs 0x284070` | `-` |
| `ldr` at 0xaee9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaeeac | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaeeb4 | load | 4 | `abs 0xaeed8` | `-` |
| `ldr` at 0xaeeb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaeebc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaeec0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaeec8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaeecc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaeed0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaeed0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaeed0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaee58 | `(unknown callee) 0xaec08` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xaee80 | `(unknown callee) 0xaeca0` | `[sp-20]` | `?` | `?` | `ret(0xaec08)` |
| `bl` at 0xaee94 | `(unknown callee) 0xaec08` | `[0x284070]` | `?` | `?` | `0x284060` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xaee6c | `ret(0xaec08) >= 0x0 (signed, 32-bit)` | `ret(0xaec08) < 0x0 (signed, 32-bit)` |
| `bne` at 0xaeea4 | `[sp-28] != ret(0xaec08)` | `[sp-28] == ret(0xaec08)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaeed0 | `r0` | `[sp-32]` |

## `flash_read`

- `flash_read`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaeedc to 0xaef6c.
  Established. A nonzero word at absolute `0x27eb30` takes the edge past the call to `0xae980`,
  while zero falls through to that call (`ldr` at 0xaeefc, `bne` at 0xaef04, `bl` at 0xaef0c).
  The joined path includes calls to `0xaed2c`, `0xaeaa0`, `0x18ae8` and `0xaeb54` (`bl` at 0xaef34,
  `bl` at 0xaef40, `bl` at 0xaef50, `bl` at 0xaef54). The call to `0x18ae8` has candidates
  `[sp-20]`, `[sp-32]`, `[sp-28]`, unknown (`bl` at 0xaef50). A word store then places `0x0` at `sp
  -0x28` (`str` at 0xaef5c). The merged return value and the physical memory operation remain
  unknown (`ldmdb` at 0xaef64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaeeec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaeef0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaeef4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xaeef8 | load | 4 | `abs 0xaef68` | `-` |
| `ldr` at 0xaeefc | load | 4 | `abs 0x27eb30` | `-` |
| `str` at 0xaef14 | store | 4 | `sp -0x24` | `ret(0xae980)` |
| `ldr` at 0xaef18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaef24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaef28 | store | 4 | `sp -0x28` | `ret(0xae980)` |
| `ldr` at 0xaef30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaef3c | store | 4 | `sp -0x20` | `ret(0xaed2c)` |
| `ldr` at 0xaef44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaef48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaef4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaef5c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xaef60 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xaef64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaef64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaef64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaef0c | `(unknown callee) 0xae980` | `0x0` | `r1` | `r2` | `[0x27eb30]` |
| `bl` at 0xaef34 | `(unknown callee) 0xaed2c` | `?` | `?` | `?` | `?` |
| `bl` at 0xaef40 | `(unknown callee) 0xaeaa0` | `ret(0xaed2c)` | `?` | `?` | `ret(0xaed2c)` |
| `bl` at 0xaef50 | `(unknown callee) 0x18ae8` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0xaef54 | `(unknown callee) 0xaeb54` | `ret(0x18ae8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaef04 | `[0x27eb30] != 0x0` | `[0x27eb30] == 0x0` |
| `bge` at 0xaef20 | `ret(0xae980) >= 0x0 (signed, 32-bit)` | `ret(0xae980) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaef64 | `r0` | `?` |

## `flash_erase`

- `flash_erase`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaef6c to 0xaef9c.
  Established. Entry r0, r1 and r2 are saved as words at `sp -0x14`, `sp -0x18` and `sp -0x1c`
  (`str` at 0xaef7c, `str` at 0xaef80, `str` at 0xaef84). Another stack word receives `0x0` at `sp
  -0x24` (`str` at 0xaef8c). The body sets r0 to `0x0` and returns that value (`mov` at 0xaef94,
  `ldmdb` at 0xaef98). No calls or conditional branches are reported. An erase operation is not
  established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaef7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaef80 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaef84 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaef8c | store | 4 | `sp -0x24` | `0x0` |
| `ldmdb` at 0xaef98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaef98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaef98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaef98 | `r0` | `0x0` |

## `flash_program`

- `flash_program`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaef9c to 0xaf3fc.
  Established. The body includes a call to `0x18bf0` with first candidate unknown, second
  `0xc00c4d04`, third `sp-92` and fourth `[sp-40]` (`bl` at 0xaf114). A zero result takes the edge
  to a byte test, while a nonzero result falls through to further calls (`beq` at 0xaf120). The
  byte test takes its edge for `[sp-75]:1 == 0xff` and falls through for inequality (`beq` at
  0xaf174). A later call to `0x187b8` has candidates unknown, `[sp-24]`, `[sp-28]`, `[sp-40]` (`bl`
  at 0xaf238); its result equal to `[sp-28]` takes the edge to a call to `0x18998`. Inequality
  falls through to other calls (`beq` at 0xaf250, `bl` at 0xaf2b0). A later word store writes
  `[sp-20]` at `[sp-32] +0x0` on the nonzero-pointer path (`beq` at 0xaf390, `str` at 0xaf39c). The
  numeric request’s device operation and final return value remain unknown (`ldmdb` at 0xaf3b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaefac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaefb0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xaefb4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xaefb8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xaefc0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xaefc4 | load | 4 | `abs 0xaf3b4` | `-` |
| `ldr` at 0xaefc8 | load | 4 | `abs 0x27eb30` | `-` |
| `str` at 0xaefe0 | store | 4 | `sp -0x40` | `ret(0xae980)` |
| `ldr` at 0xaefe4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xaeff0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xaeff4 | store | 4 | `sp -0x60` | `ret(0xae980)` |
| `ldr` at 0xaf000 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf00c | store | 4 | `sp -0x28` | `ret(0xaec08)` |
| `ldr` at 0xaf010 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf01c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf028 | store | 4 | `sp -0x30` | `ret(0xaee40)` |
| `ldr` at 0xaf02c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaf03c | load | 4 | `abs 0xaf3b8` | `-` |
| `str` at 0xaf040 | store | 4 | `sp -0x2c` | `0xfffffc7c` |
| `ldr` at 0xaf048 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf054 | store | 4 | `sp -0x34` | `ret(0xaeca0)` |
| `ldr` at 0xaf058 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf064 | load | 4 | `abs 0xaf3bc` | `-` |
| `ldr` at 0xaf068 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaf070 | load | 4 | `abs 0x284080` | `-` |
| `ldr` at 0xaf084 | load | 4 | `abs 0xaf3c0` | `-` |
| `str` at 0xaf088 | store | 4 | `sp -0x70` | `0x10867c` |
| `ldr` at 0xaf08c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf090 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xaf098 | load | 4 | `abs 0xaf3c4` | `-` |
| `ldr` at 0xaf09c | load | 4 | `abs 0xaf3c8` | `-` |
| `str` at 0xaf0ac | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xaf0b8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xaf0bc | load | 4 | `abs 0xaf3bc` | `-` |
| `ldr` at 0xaf0c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaf0c4 | load | 4 | `abs 0x284080` | `-` |
| `ldr` at 0xaf0cc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xaf0e0 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xaf0e8 | store | 4 | `sp -0x58` | `0x10` |
| `str` at 0xaf0f0 | store | 4 | `sp -0x54` | `sp-80` |
| `ldr` at 0xaf0f4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xaf0f8 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xaf0fc | load | 4 | `abs 0xaf3cc` | `-` |
| `ldr` at 0xaf100 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf108 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf10c | load | 4 | `abs 0xaf3d0` | `-` |
| `ldr` at 0xaf12c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf138 | load | 4 | `abs 0xaf3d4` | `-` |
| `str` at 0xaf13c | store | 4 | `sp -0x70` | `0x1086b8` |
| `ldr` at 0xaf140 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xaf144 | store | 4 | `sp -0x6c` | `[sp-64]` |
| `str` at 0xaf148 | store | 4 | `sp -0x68` | `ret(0x1e71c)` |
| `ldr` at 0xaf150 | load | 4 | `abs 0xaf3c4` | `-` |
| `ldr` at 0xaf154 | load | 4 | `abs 0xaf3d8` | `-` |
| `str` at 0xaf164 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xaf16c | load | 1 | `sp -0x4b` | `-` |
| `ldr` at 0xaf178 | load | 4 | `abs 0xaf3bc` | `-` |
| `ldr` at 0xaf17c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaf180 | load | 4 | `abs 0x284080` | `-` |
| `str` at 0xaf188 | store | 4 | `sp -0x34` | `([sp-52] + [0x284080])` |
| `ldr` at 0xaf18c | load | 4 | `abs 0xaf3bc` | `-` |
| `ldr` at 0xaf190 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaf194 | load | 4 | `abs 0x284080` | `-` |
| `str` at 0xaf19c | store | 4 | `sp -0x30` | `([sp-48] + [0x284080])` |
| `ldr` at 0xaf1a0 | load | 4 | `abs 0xaf3bc` | `-` |
| `ldr` at 0xaf1a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xaf1a8 | load | 4 | `abs 0x284080` | `-` |
| `str` at 0xaf1b0 | store | 4 | `sp -0x40` | `([sp-64] + [0x284080])` |
| `ldr` at 0xaf1b8 | load | 4 | `abs 0xaf3dc` | `-` |
| `ldr` at 0xaf1bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf1c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf1c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xaf1e4 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf1f0 | load | 4 | `abs 0xaf3e0` | `-` |
| `str` at 0xaf1f4 | store | 4 | `sp -0x70` | `0x1086ec` |
| `ldr` at 0xaf1f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf1fc | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xaf200 | store | 4 | `sp -0x68` | `ret(0x1e71c)` |
| `ldr` at 0xaf208 | load | 4 | `abs 0xaf3c4` | `-` |
| `ldr` at 0xaf20c | load | 4 | `abs 0xaf3e4` | `-` |
| `str` at 0xaf21c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xaf224 | load | 4 | `abs 0xaf3dc` | `-` |
| `ldr` at 0xaf228 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaf22c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xaf230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf234 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaf240 | store | 4 | `sp -0x2c` | `ret(0x187b8)` |
| `ldr` at 0xaf244 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xaf248 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf25c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf268 | load | 4 | `abs 0xaf3e8` | `-` |
| `str` at 0xaf26c | store | 4 | `sp -0x70` | `0x108718` |
| `ldr` at 0xaf270 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf274 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0xaf278 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaf27c | store | 4 | `sp -0x68` | `[sp-44]` |
| `str` at 0xaf280 | store | 4 | `sp -0x64` | `ret(0x1e71c)` |
| `ldr` at 0xaf288 | load | 4 | `abs 0xaf3c4` | `-` |
| `ldr` at 0xaf28c | load | 4 | `abs 0xaf3ec` | `-` |
| `str` at 0xaf29c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xaf2a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaf2a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf2c8 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf2d4 | load | 4 | `abs 0xaf3f0` | `-` |
| `str` at 0xaf2d8 | store | 4 | `sp -0x70` | `0x108748` |
| `ldr` at 0xaf2dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf2e0 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0xaf2e4 | store | 4 | `sp -0x68` | `ret(0x1e71c)` |
| `ldr` at 0xaf2ec | load | 4 | `abs 0xaf3c4` | `-` |
| `ldr` at 0xaf2f0 | load | 4 | `abs 0xaf3f4` | `-` |
| `str` at 0xaf300 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xaf30c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xaf310 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaf314 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf324 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaf328 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaf330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf334 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xaf33c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xaf340 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xaf34c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xaf350 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaf358 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xaf35c | load | 4 | `abs 0xaf3f8` | `-` |
| `str` at 0xaf360 | store | 4 | `sp -0x2c` | `0xfffffc7a` |
| `ldr` at 0xaf368 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaf370 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xaf380 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xaf388 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaf394 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xaf398 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf39c | store | 4 | `[sp-32] +0x0` | `[sp-20]` |
| `ldr` at 0xaf3a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaf3a8 | store | 4 | `sp -0x60` | `[sp-44]` |
| `ldr` at 0xaf3ac | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xaf3b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf3b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf3b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaefd8 | `(unknown callee) 0xae980` | `0x0` | `r1` | `r2` | `[0x27eb30]` |
| `bl` at 0xaeffc | `(unknown callee) 0xaeaa0` | `?` | `?` | `?` | `?` |
| `bl` at 0xaf004 | `(unknown callee) 0xaec08` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xaf020 | `(unknown callee) 0xaee40` | `[sp-20]` | `?` | `?` | `ret(0xaec08)` |
| `bl` at 0xaf04c | `(unknown callee) 0xaeca0` | `[sp-20]` | `?` | `?` | `ret(0xaee40)` |
| `bl` at 0xaf074 | `(unknown callee) 0xd574c` | `ret(0xaeca0)` | `[0x284080]` | `ret(0xaeca0)` | `0x284060` |
| `bl` at 0xaf0a4 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x193` | `0x0` |
| `bl` at 0xaf114 | `(unknown callee) 0x18bf0` | `?` | `0xc00c4d04` | `sp-92` | `[sp-40]` |
| `bl` at 0xaf124 | `(unknown callee) 0x188b4` | `ret(0x18bf0)` | `?` | `?` | `ret(0x18bf0)` |
| `bl` at 0xaf130 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf15c | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x19e` | `0x0` |
| `bl` at 0xaf1cc | `(unknown callee) 0x18ad0` | `?` | `?` | `0x0` | `0x1157b0` |
| `bl` at 0xaf1dc | `(unknown callee) 0x188b4` | `ret(0x18ad0)` | `?` | `?` | `ret(0x18ad0)` |
| `bl` at 0xaf1e8 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf214 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x1af` | `0x0` |
| `bl` at 0xaf238 | `(unknown callee) 0x187b8` | `?` | `[sp-24]` | `[sp-28]` | `[sp-40]` |
| `bl` at 0xaf254 | `(unknown callee) 0x188b4` | `ret(0x187b8)` | `?` | `ret(0x187b8)` | `[sp-28]` |
| `bl` at 0xaf260 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf294 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x1b5` | `0x0` |
| `bl` at 0xaf2b0 | `(unknown callee) 0x18998` | `[sp-48]` | `[sp-28]` | `0x6` | `[sp-28]` |
| `bl` at 0xaf2c0 | `(unknown callee) 0x188b4` | `ret(0x18998)` | `?` | `?` | `ret(0x18998)` |
| `bl` at 0xaf2cc | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf2f8 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x1bd` | `0x0` |
| `bl` at 0xaf378 | `(unknown callee) 0xaeb54` | `ret(0x18998)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xaf3a0 | `(unknown callee) 0xaeb54` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaefd0 | `[0x27eb30] != 0x0` | `[0x27eb30] == 0x0` |
| `bge` at 0xaefec | `ret(0xae980) >= 0x0 (signed, 32-bit)` | `ret(0xae980) < 0x0 (signed, 32-bit)` |
| `beq` at 0xaf018 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xaf034 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xaf060 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xaf080 | `ret(0xd574c) == 0x0` | `ret(0xd574c) != 0x0` |
| `blo` at 0xaf0d4 | `C == 0` | `not (C == 0)` |
| `beq` at 0xaf120 | `ret(0x18bf0) == 0x0` | `ret(0x18bf0) != 0x0` |
| `beq` at 0xaf174 | `[sp-75]:1 == 0xff` | `[sp-75]:1 != 0xff` |
| `bge` at 0xaf1d8 | `ret(0x18ad0) >= 0x0 (signed, 32-bit)` | `ret(0x18ad0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xaf250 | `ret(0x187b8) == [sp-28]` | `ret(0x187b8) != [sp-28]` |
| `bge` at 0xaf2bc | `ret(0x18998) >= 0x0 (signed, 32-bit)` | `ret(0x18998) < 0x0 (signed, 32-bit)` |
| `blo` at 0xaf31c | `C == 0` | `not (C == 0)` |
| `beq` at 0xaf348 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xaf390 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf3b0 | `r0` | `?` |

## `flash_addr_to_index`

- `flash_addr_to_index`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaec08 to 0xaeca0.
  Established. A nonzero absolute word at `0x27eb30` takes the edge that bypasses a call to
  `0xae980`, while zero falls through to that call (`ldr` at 0xaec20, `bne` at 0xaec28, `bl` at
  0xaec30). Two later signed comparisons take their edges for `[sp-24] < 0x0` and `[sp-24] > 0x2`,
  respectively (`blt` at 0xaec6c, `bgt` at 0xaec78). The other path stores `[sp-24]` as a word at
  `sp -0x20` (`str` at 0xaec90). The return reports `[sp-32]` (`ldmdb` at 0xaec98). A particular
  flash-bank interpretation of the selector is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaec18 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaec1c | load | 4 | `abs 0xaec9c` | `-` |
| `ldr` at 0xaec20 | load | 4 | `abs 0x27eb30` | `-` |
| `str` at 0xaec38 | store | 4 | `sp -0x1c` | `ret(0xae980)` |
| `ldr` at 0xaec3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaec48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaec4c | store | 4 | `sp -0x20` | `ret(0xae980)` |
| `ldr` at 0xaec54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaec60 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xaec64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaec70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaec84 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaec8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaec90 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xaec94 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaec98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaec98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaec98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaec30 | `(unknown callee) 0xae980` | `0x0` | `r1` | `r2` | `[0x27eb30]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaec28 | `[0x27eb30] != 0x0` | `[0x27eb30] == 0x0` |
| `bge` at 0xaec44 | `ret(0xae980) >= 0x0 (signed, 32-bit)` | `ret(0xae980) < 0x0 (signed, 32-bit)` |
| `blt` at 0xaec6c | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bgt` at 0xaec78 | `[sp-24] > 0x2 (signed, 32-bit)` | `[sp-24] <= 0x2 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaec98 | `r0` | `[sp-32]` |

## `flash_set_boot_image`

- `flash_set_boot_image`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf3fc to 0xaf620.
  Established. A nonzero entry r0 takes the edge to comparisons against absolute words
  `0x284068` and `0x28406c` (`bne` at 0xaf418, `bne` at 0xaf4a8, `bne` at 0xaf4c8). Falling through
  those inequality branches selects byte stores of `0x41` and `0x42`, respectively, at `sp -0x15`
  (`bne` at 0xaf4a8, `strb` at 0xaf4b0, `bne` at 0xaf4c8, `strb` at 0xaf4d0). A call to `0x18b54`
  later has candidates `0x108774`, `0x241`, entry r0 and unknown (`bl` at 0xaf510). The call to
  `0x187b8` uses candidates `ret(0x18b54)`, `sp-21`, `0x1`, `sp-21` (`bl` at 0xaf580). A result
  equal to `0x1` takes the edge to `0x18800`, while another value falls through to
  additional calls (`beq` at 0xaf58c, `bl` at 0xaf5dc). The byte selector’s boot policy is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf40c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf41c | load | 4 | `abs 0xaf5f0` | `-` |
| `ldr` at 0xaf438 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf44c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf458 | load | 4 | `abs 0xaf5f4` | `-` |
| `str` at 0xaf45c | store | 4 | `sp -0x2c` | `0x108788` |
| `ldr` at 0xaf460 | load | 4 | `abs 0xaf5f0` | `-` |
| `str` at 0xaf464 | store | 4 | `sp -0x28` | `0x108774` |
| `str` at 0xaf468 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf470 | load | 4 | `abs 0xaf5f8` | `-` |
| `str` at 0xaf484 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xaf490 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf498 | load | 4 | `abs 0xaf5fc` | `-` |
| `ldr` at 0xaf49c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf4a0 | load | 4 | `abs 0x284068` | `-` |
| `strb` at 0xaf4b0 | store | 1 | `sp -0x15` | `0x41` |
| `ldr` at 0xaf4b8 | load | 4 | `abs 0xaf5fc` | `-` |
| `ldr` at 0xaf4bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf4c0 | load | 4 | `abs 0x28406c` | `-` |
| `strb` at 0xaf4d0 | store | 1 | `sp -0x15` | `0x42` |
| `ldr` at 0xaf4d8 | load | 4 | `abs 0xaf600` | `-` |
| `str` at 0xaf4dc | store | 4 | `sp -0x2c` | `0x1087b4` |
| `ldr` at 0xaf4e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf4e4 | store | 4 | `sp -0x28` | `r0` |
| `ldr` at 0xaf4ec | load | 4 | `abs 0xaf5f8` | `-` |
| `ldr` at 0xaf4f0 | load | 4 | `abs 0xaf604` | `-` |
| `str` at 0xaf500 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaf508 | load | 4 | `abs 0xaf5f0` | `-` |
| `ldr` at 0xaf50c | load | 4 | `abs 0xaf608` | `-` |
| `str` at 0xaf518 | store | 4 | `sp -0x1c` | `ret(0x18b54)` |
| `ldr` at 0xaf51c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf530 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf53c | load | 4 | `abs 0xaf60c` | `-` |
| `str` at 0xaf540 | store | 4 | `sp -0x2c` | `0x1087dc` |
| `ldr` at 0xaf544 | load | 4 | `abs 0xaf610` | `-` |
| `str` at 0xaf548 | store | 4 | `sp -0x28` | `0x108808` |
| `str` at 0xaf54c | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf554 | load | 4 | `abs 0xaf5f8` | `-` |
| `ldr` at 0xaf558 | load | 4 | `abs 0xaf614` | `-` |
| `str` at 0xaf568 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaf574 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaf598 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf5a4 | load | 4 | `abs 0xaf618` | `-` |
| `str` at 0xaf5a8 | store | 4 | `sp -0x2c` | `0x108820` |
| `ldr` at 0xaf5ac | load | 4 | `abs 0xaf5f0` | `-` |
| `str` at 0xaf5b0 | store | 4 | `sp -0x28` | `0x108774` |
| `str` at 0xaf5b4 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf5bc | load | 4 | `abs 0xaf5f8` | `-` |
| `ldr` at 0xaf5c0 | load | 4 | `abs 0xaf61c` | `-` |
| `str` at 0xaf5d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaf5d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xaf5e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf5e8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaf5ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf5ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf5ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf420 | `(unknown callee) 0x1883c` | `0x108774` | `r1` | `r2` | `r0` |
| `bl` at 0xaf430 | `(unknown callee) 0x188b4` | `ret(0x1883c)` | `?` | `?` | `ret(0x1883c)` |
| `bl` at 0xaf444 | `(unknown callee) 0x188b4` | `ret(0x188b4)` | `?` | `?` | `[ret(0x188b4)]` |
| `bl` at 0xaf450 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf47c | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x1e8` | `0x0` |
| `bl` at 0xaf4f8 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x1f2` | `0x0` |
| `bl` at 0xaf510 | `(unknown callee) 0x18b54` | `0x108774` | `0x241` | `r0` | `?` |
| `bl` at 0xaf528 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `?` | `ret(0x18b54)` |
| `bl` at 0xaf534 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf560 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x1f7` | `0x0` |
| `bl` at 0xaf580 | `(unknown callee) 0x187b8` | `ret(0x18b54)` | `sp-21` | `0x1` | `sp-21` |
| `bl` at 0xaf590 | `(unknown callee) 0x188b4` | `ret(0x187b8)` | `?` | `?` | `ret(0x187b8)` |
| `bl` at 0xaf59c | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf5c8 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x1fd` | `0x0` |
| `bl` at 0xaf5dc | `(unknown callee) 0x18800` | `[sp-28]` | `?` | `?` | `ret(0x187b8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf418 | `r0 != 0x0` | `r0 == 0x0` |
| `bge` at 0xaf42c | `ret(0x1883c) >= 0x0 (signed, 32-bit)` | `ret(0x1883c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xaf440 | `[ret(0x188b4)] == 0x2` | `[ret(0x188b4)] != 0x2` |
| `bne` at 0xaf4a8 | `r0 != [0x284068]` | `r0 == [0x284068]` |
| `bne` at 0xaf4c8 | `r0 != [0x28406c]` | `r0 == [0x28406c]` |
| `bge` at 0xaf524 | `ret(0x18b54) >= 0x0 (signed, 32-bit)` | `ret(0x18b54) < 0x0 (signed, 32-bit)` |
| `beq` at 0xaf58c | `ret(0x187b8) == 0x1` | `ret(0x187b8) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf5ec | `r0` | `?` |

## `flash_set_default_image`

- `flash_set_default_image`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf620 to 0xaf8d0.
  Established. Two opening word stores address `[0x28407c] +0x0` and `[0x28407c] +0x4`, both with
  unknown values (`str` at 0xaf64c, `str` at 0xaf678). A nonzero `[sp-20]` takes the edge to
  comparisons against `[0x284068]` and `[0x28406c] (`bne` at 0xaf684, `bne` at 0xaf714, `bne` at
  0xaf734). Their equality fall-through paths select byte `0x41` or `0x42` at `sp -0x19` (`bne` at
  0xaf714, `strb` at 0xaf71c, `bne` at 0xaf734, `strb` at 0xaf73c). A later call to `0x187b8` has
  candidates `ret(0x18b54)`, `sp-25`, `0x1`, `sp-25` (`bl` at 0xaf7ec). The body also includes a
  word store whose effective target and value are both unknown (`str` at 0xaf884). The state
  changes’ default-image meanings are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf630 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaf634 | load | 4 | `abs 0xaf8a0` | `-` |
| `ldr` at 0xaf638 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf63c | load | 4 | `abs 0xaf8a0` | `-` |
| `ldr` at 0xaf640 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf644 | load | 4 | `[0x28407c] +0x0` | `-` |
| `str` at 0xaf64c | store | 4 | `[0x28407c] +0x0` | `?` |
| `ldr` at 0xaf650 | load | 4 | `abs 0xaf8a0` | `-` |
| `ldr` at 0xaf658 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf660 | load | 4 | `abs 0xaf8a0` | `-` |
| `ldr` at 0xaf668 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf670 | load | 4 | `[0x28407c] +0x4` | `-` |
| `str` at 0xaf678 | store | 4 | `[0x28407c] +0x4` | `?` |
| `ldr` at 0xaf67c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf688 | load | 4 | `abs 0xaf8a4` | `-` |
| `ldr` at 0xaf6a4 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf6b8 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf6c4 | load | 4 | `abs 0xaf8a8` | `-` |
| `str` at 0xaf6c8 | store | 4 | `sp -0x2c` | `0x10884c` |
| `ldr` at 0xaf6cc | load | 4 | `abs 0xaf8a4` | `-` |
| `str` at 0xaf6d0 | store | 4 | `sp -0x28` | `0x108808` |
| `str` at 0xaf6d4 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf6dc | load | 4 | `abs 0xaf8ac` | `-` |
| `ldr` at 0xaf6e0 | load | 4 | `abs 0xaf8b0` | `-` |
| `str` at 0xaf6f0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xaf6fc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf704 | load | 4 | `abs 0xaf8a0` | `-` |
| `ldr` at 0xaf708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf70c | load | 4 | `abs 0x284068` | `-` |
| `strb` at 0xaf71c | store | 1 | `sp -0x19` | `0x41` |
| `ldr` at 0xaf724 | load | 4 | `abs 0xaf8a0` | `-` |
| `ldr` at 0xaf728 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf72c | load | 4 | `abs 0x28406c` | `-` |
| `strb` at 0xaf73c | store | 1 | `sp -0x19` | `0x42` |
| `ldr` at 0xaf744 | load | 4 | `abs 0xaf8b4` | `-` |
| `str` at 0xaf748 | store | 4 | `sp -0x2c` | `0x10887c` |
| `ldr` at 0xaf74c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaf750 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xaf758 | load | 4 | `abs 0xaf8ac` | `-` |
| `ldr` at 0xaf75c | load | 4 | `abs 0xaf8b8` | `-` |
| `str` at 0xaf76c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaf774 | load | 4 | `abs 0xaf8a4` | `-` |
| `ldr` at 0xaf778 | load | 4 | `abs 0xaf8bc` | `-` |
| `str` at 0xaf784 | store | 4 | `sp -0x18` | `ret(0x18b54)` |
| `ldr` at 0xaf788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf79c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf7a8 | load | 4 | `abs 0xaf8c0` | `-` |
| `str` at 0xaf7ac | store | 4 | `sp -0x2c` | `0x1087dc` |
| `ldr` at 0xaf7b0 | load | 4 | `abs 0xaf8a4` | `-` |
| `str` at 0xaf7b4 | store | 4 | `sp -0x28` | `0x108808` |
| `str` at 0xaf7b8 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf7c0 | load | 4 | `abs 0xaf8ac` | `-` |
| `str` at 0xaf7d4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaf7e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaf804 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf810 | load | 4 | `abs 0xaf8c4` | `-` |
| `str` at 0xaf814 | store | 4 | `sp -0x2c` | `0x1088a8` |
| `ldr` at 0xaf818 | load | 4 | `abs 0xaf8a4` | `-` |
| `str` at 0xaf81c | store | 4 | `sp -0x28` | `0x108808` |
| `str` at 0xaf820 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf828 | load | 4 | `abs 0xaf8ac` | `-` |
| `ldr` at 0xaf82c | load | 4 | `abs 0xaf8c8` | `-` |
| `str` at 0xaf83c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaf844 | load | 4 | `abs 0xaf8a0` | `-` |
| `ldrb` at 0xaf848 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xaf84c | load | 4 | `abs 0xaf8cc` | `-` |
| `ldr` at 0xaf854 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf860 | load | 4 | `abs 0xaf8a0` | `-` |
| `ldrb` at 0xaf864 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xaf868 | load | 4 | `abs 0xaf8cc` | `-` |
| `ldr` at 0xaf870 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf87c | load | 4 | `(unknown)` | `-` |
| `str` at 0xaf884 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xaf888 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaf894 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf898 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xaf89c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf89c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf89c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf68c | `(unknown callee) 0x1883c` | `0x108808` | `[0x28407c]+4` | `0x4` | `[sp-20]` |
| `bl` at 0xaf69c | `(unknown callee) 0x188b4` | `ret(0x1883c)` | `?` | `?` | `ret(0x1883c)` |
| `bl` at 0xaf6b0 | `(unknown callee) 0x188b4` | `ret(0x188b4)` | `?` | `?` | `[ret(0x188b4)]` |
| `bl` at 0xaf6bc | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf6e8 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x215` | `0x0` |
| `bl` at 0xaf764 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x21f` | `0x0` |
| `bl` at 0xaf77c | `(unknown callee) 0x18b54` | `0x108808` | `0x241` | `[sp-20]` | `?` |
| `bl` at 0xaf794 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `?` | `ret(0x18b54)` |
| `bl` at 0xaf7a0 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf7cc | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x224` | `0x0` |
| `bl` at 0xaf7ec | `(unknown callee) 0x187b8` | `ret(0x18b54)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xaf7fc | `(unknown callee) 0x188b4` | `ret(0x187b8)` | `?` | `?` | `ret(0x187b8)` |
| `bl` at 0xaf808 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf834 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x22a` | `0x0` |
| `bl` at 0xaf88c | `(unknown callee) 0x18800` | `[sp-24]` | `0x284060` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaf684 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bge` at 0xaf698 | `ret(0x1883c) >= 0x0 (signed, 32-bit)` | `ret(0x1883c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xaf6ac | `[ret(0x188b4)] == 0x2` | `[ret(0x188b4)] != 0x2` |
| `bne` at 0xaf714 | `[sp-20] != [0x284068]` | `[sp-20] == [0x284068]` |
| `bne` at 0xaf734 | `[sp-20] != [0x28406c]` | `[sp-20] == [0x28406c]` |
| `bge` at 0xaf790 | `ret(0x18b54) >= 0x0 (signed, 32-bit)` | `ret(0x18b54) < 0x0 (signed, 32-bit)` |
| `beq` at 0xaf7f8 | `ret(0x187b8) == 0x1` | `ret(0x187b8) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf89c | `r0` | `?` |

## `flash_get_default_image`

- `flash_get_default_image`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf8d0 to 0xafafc.
  Established. The opening stores address `[0x28407c] +0x0` and `[0x28407c] +0x4`, both with unknown
  word values (`str` at 0xaf8f8, `str` at 0xaf924). A call to `0x18b54` uses candidates `0x108808`,
  `0x0`, `0x4`, unknown (`bl` at 0xaf930). A later call to `0x18cc8` uses `ret(0x18b54)`, `sp-25`,
  `0x1`, `sp-25` (`bl` at 0xaf9c0); its result equal to `0x1` takes the edge to byte tests, while
  other values fall through to additional calls (`beq` at 0xaf9cc). The equality fall-through paths for
  byte `0x41` or `0x42` select stores of `[0x284068]` or `[0x28406c]` at `sp -0x18`, respectively (`bne` at 0xafa20, `str` at 0xafa2c, `bne` at 0xafa3c, `str` at 0xafa48). The
  default-selection semantics and merged return value are unknown (`ldmdb` at 0xafad8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaf8e0 | load | 4 | `abs 0xafadc` | `-` |
| `ldr` at 0xaf8e4 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf8e8 | load | 4 | `abs 0xafadc` | `-` |
| `ldr` at 0xaf8ec | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf8f0 | load | 4 | `[0x28407c] +0x0` | `-` |
| `str` at 0xaf8f8 | store | 4 | `[0x28407c] +0x0` | `?` |
| `ldr` at 0xaf8fc | load | 4 | `abs 0xafadc` | `-` |
| `ldr` at 0xaf904 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf90c | load | 4 | `abs 0xafadc` | `-` |
| `ldr` at 0xaf914 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xaf91c | load | 4 | `[0x28407c] +0x4` | `-` |
| `str` at 0xaf924 | store | 4 | `[0x28407c] +0x4` | `?` |
| `ldr` at 0xaf928 | load | 4 | `abs 0xafae0` | `-` |
| `str` at 0xaf938 | store | 4 | `sp -0x14` | `ret(0x18b54)` |
| `ldr` at 0xaf93c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf950 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xaf960 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xaf970 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf97c | load | 4 | `abs 0xafae4` | `-` |
| `str` at 0xaf980 | store | 4 | `sp -0x2c` | `0x1088d4` |
| `ldr` at 0xaf984 | load | 4 | `abs 0xafae0` | `-` |
| `str` at 0xaf988 | store | 4 | `sp -0x28` | `0x108808` |
| `str` at 0xaf98c | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf994 | load | 4 | `abs 0xafae8` | `-` |
| `str` at 0xaf9a8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xaf9b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf9d8 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xaf9e4 | load | 4 | `abs 0xafaec` | `-` |
| `str` at 0xaf9e8 | store | 4 | `sp -0x2c` | `0x108904` |
| `ldr` at 0xaf9ec | load | 4 | `abs 0xafae0` | `-` |
| `str` at 0xaf9f0 | store | 4 | `sp -0x28` | `0x108808` |
| `str` at 0xaf9f4 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0xaf9fc | load | 4 | `abs 0xafae8` | `-` |
| `ldr` at 0xafa00 | load | 4 | `abs 0xafaf0` | `-` |
| `str` at 0xafa10 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xafa18 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xafa24 | load | 4 | `abs 0xafadc` | `-` |
| `ldr` at 0xafa28 | load | 4 | `abs 0x284068` | `-` |
| `str` at 0xafa2c | store | 4 | `sp -0x18` | `[0x284068]` |
| `ldrb` at 0xafa34 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xafa40 | load | 4 | `abs 0xafadc` | `-` |
| `ldr` at 0xafa44 | load | 4 | `abs 0x28406c` | `-` |
| `str` at 0xafa48 | store | 4 | `sp -0x18` | `[0x28406c]` |
| `ldr` at 0xafa50 | load | 4 | `abs 0xafaf4` | `-` |
| `str` at 0xafa54 | store | 4 | `sp -0x2c` | `0x108938` |
| `ldrb` at 0xafa58 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xafa5c | store | 4 | `sp -0x28` | `[sp-25]:1` |
| `ldr` at 0xafa64 | load | 4 | `abs 0xafae8` | `-` |
| `str` at 0xafa78 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xafa80 | load | 4 | `abs 0xafadc` | `-` |
| `ldrb` at 0xafa84 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xafa88 | load | 4 | `abs 0xafaf8` | `-` |
| `ldr` at 0xafa90 | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xafa9c | load | 4 | `abs 0xafadc` | `-` |
| `ldrb` at 0xafaa0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xafaa4 | load | 4 | `abs 0xafaf8` | `-` |
| `ldr` at 0xafaac | load | 4 | `abs 0x28407c` | `-` |
| `ldr` at 0xafab8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xafac0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xafac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafacc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xafad0 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xafad4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xafad8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xafad8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xafad8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf930 | `(unknown callee) 0x18b54` | `0x108808` | `0x0` | `0x4` | `?` |
| `bl` at 0xaf948 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `?` | `ret(0x18b54)` |
| `bl` at 0xaf968 | `(unknown callee) 0x188b4` | `ret(0x188b4)` | `?` | `?` | `[ret(0x188b4)]` |
| `bl` at 0xaf974 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xaf9a0 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x248` | `0x0` |
| `bl` at 0xaf9c0 | `(unknown callee) 0x18cc8` | `ret(0x18b54)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0xaf9d0 | `(unknown callee) 0x188b4` | `ret(0x18cc8)` | `?` | `?` | `ret(0x18cc8)` |
| `bl` at 0xaf9dc | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xafa08 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x24e` | `0x0` |
| `bl` at 0xafa70 | `(unknown callee) 0x1eed0` | `0x3` | `0x108464` | `0x258` | `0x0` |
| `bl` at 0xafac8 | `(unknown callee) 0x18800` | `[sp-20]` | `0x284060` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xaf944 | `ret(0x18b54) >= 0x0 (signed, 32-bit)` | `ret(0x18b54) < 0x0 (signed, 32-bit)` |
| `bne` at 0xaf958 | `[ret(0x188b4)] != 0x2` | `[ret(0x188b4)] == 0x2` |
| `beq` at 0xaf9cc | `ret(0x18cc8) == 0x1` | `ret(0x18cc8) != 0x1` |
| `bne` at 0xafa20 | `[sp-25]:1 != 0x41` | `[sp-25]:1 == 0x41` |
| `bne` at 0xafa3c | `[sp-25]:1 != 0x42` | `[sp-25]:1 == 0x42` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xafad8 | `r0` | `?` |

## `FlashInit`

- `FlashInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8b12c to 0x8b2bc.
  Established. A word store increments absolute `0x11b26c` (`str` at 0x8b178). A call to `0x9a7e0`
  has candidates `0x282240`, `0x10`, `0x0`, `[0x11b26c]+1` (`bl` at 0x8b188). A later call to
  `0x99520` uses candidates `0x282240`, `[0x282240]`, `0x1`, `0x28224c` (`bl` at 0x8b200). The final
  call to `0x1f498` has candidates `0x282248`, `0xfb094`, `0x8b2bc`, `0x0`, and its result reaches
  the return (`bl` at 0x8b27c, `ldmdb` at 0x8b280). The callees’ registration and task-creation
  behavior remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8b13c | load | 4 | `abs 0x8b284` | `-` |
| `ldr` at 0x8b140 | load | 4 | `abs 0x11b26c` | `-` |
| `ldr` at 0x8b14c | load | 4 | `abs 0x8b288` | `-` |
| `str` at 0x8b150 | store | 4 | `sp -0x30` | `0xfb010` |
| `ldr` at 0x8b158 | load | 4 | `abs 0x8b28c` | `-` |
| `ldr` at 0x8b15c | load | 4 | `abs 0x8b290` | `-` |
| `ldr` at 0x8b168 | load | 4 | `abs 0x8b284` | `-` |
| `ldr` at 0x8b16c | load | 4 | `abs 0x8b284` | `-` |
| `ldr` at 0x8b170 | load | 4 | `abs 0x11b26c` | `-` |
| `str` at 0x8b178 | store | 4 | `abs 0x11b26c` | `[0x11b26c]+1` |
| `ldr` at 0x8b17c | load | 4 | `abs 0x8b294` | `-` |
| `str` at 0x8b190 | store | 4 | `sp -0x18` | `0x19` |
| `str` at 0x8b198 | store | 4 | `sp -0x1c` | `0x14` |
| `ldr` at 0x8b19c | load | 4 | `abs 0x8b298` | `-` |
| `ldr` at 0x8b1a0 | load | 4 | `abs 0x8b29c` | `-` |
| `str` at 0x8b1b0 | store | 4 | `sp -0x14` | `ret(0x9a8f8)` |
| `ldr` at 0x8b1b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b1c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b1cc | load | 4 | `abs 0x8b2a0` | `-` |
| `str` at 0x8b1d0 | store | 4 | `sp -0x30` | `0xfb04c` |
| `str` at 0x8b1d4 | store | 4 | `sp -0x2c` | `ret(0x1e71c)` |
| `ldr` at 0x8b1dc | load | 4 | `abs 0x8b28c` | `-` |
| `ldr` at 0x8b1e0 | load | 4 | `abs 0x8b2a4` | `-` |
| `ldr` at 0x8b1ec | load | 4 | `abs 0x8b294` | `-` |
| `ldr` at 0x8b1f0 | load | 4 | `abs 0x8b294` | `-` |
| `ldr` at 0x8b1f4 | load | 4 | `abs 0x282240` | `-` |
| `ldr` at 0x8b1fc | load | 4 | `abs 0x8b298` | `-` |
| `str` at 0x8b208 | store | 4 | `sp -0x14` | `ret(0x99520)` |
| `ldr` at 0x8b20c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b218 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8b224 | load | 4 | `abs 0x8b2a8` | `-` |
| `str` at 0x8b228 | store | 4 | `sp -0x30` | `0xfb070` |
| `str` at 0x8b22c | store | 4 | `sp -0x2c` | `ret(0x1e71c)` |
| `ldr` at 0x8b234 | load | 4 | `abs 0x8b28c` | `-` |
| `ldr` at 0x8b238 | load | 4 | `abs 0x8b2ac` | `-` |
| `str` at 0x8b248 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x8b250 | store | 4 | `sp -0x2c` | `0x400` |
| `ldr` at 0x8b254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8b258 | store | 4 | `sp -0x28` | `[sp-24]` |
| `ldr` at 0x8b25c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8b260 | store | 4 | `sp -0x24` | `[sp-28]` |
| `str` at 0x8b268 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x8b26c | load | 4 | `abs 0x8b2b0` | `-` |
| `ldr` at 0x8b270 | load | 4 | `abs 0x8b2b4` | `-` |
| `ldr` at 0x8b274 | load | 4 | `abs 0x8b2b8` | `-` |
| `ldmdb` at 0x8b280 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8b280 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8b280 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8b164 | `(unknown callee) 0x1eed0` | `0x9` | `0xfb028` | `0xbc1` | `0x0` |
| `bl` at 0x8b188 | `(unknown callee) 0x9a7e0` | `0x282240` | `0x10` | `0x0` | `[0x11b26c]+1` |
| `bl` at 0x8b1a8 | `(unknown callee) 0x9a8f8` | `0x28224c` | `0xfb044` | `0x5` | `0x14` |
| `bl` at 0x8b1c4 | `(unknown callee) 0x1e71c` | `ret(0x9a8f8)` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0x8b1e8 | `(unknown callee) 0x1eed0` | `0x9` | `0xfb028` | `0xbd9` | `0x0` |
| `bl` at 0x8b200 | `(unknown callee) 0x99520` | `0x282240` | `[0x282240]` | `0x1` | `0x28224c` |
| `bl` at 0x8b21c | `(unknown callee) 0x1e71c` | `ret(0x99520)` | `?` | `?` | `ret(0x99520)` |
| `bl` at 0x8b240 | `(unknown callee) 0x1eed0` | `0x9` | `0xfb028` | `0xbe2` | `0x0` |
| `bl` at 0x8b27c | `(unknown callee) 0x1f498` | `0x282248` | `0xfb094` | `0x8b2bc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8b148 | `[0x11b26c] == 0x0` | `[0x11b26c] != 0x0` |
| `beq` at 0x8b1bc | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `beq` at 0x8b214 | `ret(0x99520) == 0x0` | `ret(0x99520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8b280 | `r0` | `ret(0x1f498)` |

## `FlashTask`

- `FlashTask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8b2bc to 0x8b390.
  Established. The loop begins with a call to `0x9a93c` whose candidates are `sp-32`, `0x28224c`,
  unknown and `sp-32` (`bl` at 0x8b2e4). A zero result takes the edge into further testing, while
  nonzero falls through to additional calls (`beq` at 0x8b2f8). A nonzero absolute word at
  `0x11b2d4` falls through its equality branch to a decrementing word store there, while zero
  takes the other edge (`beq` at 0x8b30c, `str` at 0x8b320). On the latter path, a zero byte at
  `[sp-32] +0x16` falls through its inequality branch to a call to `0x8b3b8` (`ldrb` at 0x8b32c,
  `bne` at 0x8b334, `bl` at 0x8b33c). The joined path calls `0x9a998` and branches back to `0x8b2d4`
  (`bl` at 0x8b374, `b` at 0x8b378). No return is reported, and the byte’s command meaning is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8b2cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8b2d0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8b2dc | load | 4 | `abs 0x8b37c` | `-` |
| `str` at 0x8b2ec | store | 4 | `sp -0x1c` | `ret(0x9a93c)` |
| `ldr` at 0x8b2f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8b300 | load | 4 | `abs 0x8b380` | `-` |
| `ldr` at 0x8b304 | load | 4 | `abs 0x11b2d4` | `-` |
| `ldr` at 0x8b310 | load | 4 | `abs 0x8b380` | `-` |
| `ldr` at 0x8b314 | load | 4 | `abs 0x8b380` | `-` |
| `ldr` at 0x8b318 | load | 4 | `abs 0x11b2d4` | `-` |
| `str` at 0x8b320 | store | 4 | `abs 0x11b2d4` | `[0x11b2d4]-1` |
| `ldr` at 0x8b328 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x8b32c | load | 1 | `[sp-32] +0x16` | `-` |
| `ldr` at 0x8b338 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8b344 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8b350 | load | 4 | `abs 0x8b384` | `-` |
| `str` at 0x8b354 | store | 4 | `sp -0x28` | `0xfb09c` |
| `str` at 0x8b358 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0x8b360 | load | 4 | `abs 0x8b388` | `-` |
| `ldr` at 0x8b364 | load | 4 | `abs 0x8b38c` | `-` |
| `ldr` at 0x8b370 | load | 4 | `sp -0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8b2e4 | `(unknown callee) 0x9a93c` | `sp-32` | `0x28224c` | `?` | `sp-32` |
| `bl` at 0x8b33c | `(unknown callee) 0x8b3b8` | `[sp-32]` | `?` | `?` | `[[sp-32]+22]:1` |
| `bl` at 0x8b348 | `(unknown callee) 0x1e71c` | `ret(0x9a93c)` | `?` | `?` | `ret(0x9a93c)` |
| `bl` at 0x8b36c | `(unknown callee) 0x1eed0` | `0x3` | `0xfb028` | `0xc0e` | `0x0` |
| `bl` at 0x8b374 | `(unknown callee) 0x9a998` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8b2f8 | `ret(0x9a93c) == 0x0` | `ret(0x9a93c) != 0x0` |
| `beq` at 0x8b30c | `[0x11b2d4] == 0x0` | `[0x11b2d4] != 0x0` |
| `bne` at 0x8b334 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
