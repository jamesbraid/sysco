# MMSC 2.2 controller command and save paths

Read from `6.5.6/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.6, MMSC 2.2).
Image SHA-256: `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/controller-2.2-facts.txt) include all tool sections.

## `_elscCheckResponse`

- `_elscCheckResponse`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12305c to 0x1231fc.
  Established. The two byte-zero stores render as `ret(_strchr) +0x0` (`mov` at 0x12306c, `mov` at
  0x12307e). The two `_strchr` call sites have second candidates 0xd and 0x14 respectively, with
  first candidates unresolved (`call` at 0x123072, `call` at 0x1230ab).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123062 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x12306c | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `mov` at 0x12307e | store | 1 | `ret(_strchr) +0x0` | `0x0` |
| `test` at 0x123081 | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x123099 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123072 | `_strchr 0x184070` | `?` | `0xd` | `?` | `?` | `?` | `?` |
| `call` at 0x123090 | `_ffscMsg 0x12154c` | `0x12302c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12309d | `_elscProcessCommand 0x12393c` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1230ab | `_strchr 0x184070` | `?` | `0x14` | `?` | `?` | `?` | `?` |
| `call` at 0x1230be | `_strlen 0x1784d4` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1230c9 | `_memmove 0x183ff8` | `[sp+8]` | `?` | `ret(_strlen)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12307c | `(ret(_strchr) & ret(_strchr)) == 0x0` | `(ret(_strchr) & ret(_strchr)) != 0x0` |
| `je` at 0x123088 | `(([0x1e0bf0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x1230b7 | `(ret(_strchr) & ret(_strchr)) != 0x0` | `(ret(_strchr) & ret(_strchr)) == 0x0` |
| `je` at 0x1230bb | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1230d7 | `eax` | `?` |

## `_elscDoAdminCommand`

- `_elscDoAdminCommand`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1231fc to 0x12345c.
  Established. The `_msgWrite` and `_msgRead` calls have unknown first candidates and second
  candidates `[0x1e06a8]` and `[0x1e0694]` respectively (`call` at 0x1232ce, `call` at 0x12335f).
  Separate four-byte stores write values 1 and 5 to unknown effective targets (`mov` at 0x123263,
  `mov` at 0x12334d).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123205 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123208 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x12320e | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x12321d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12323c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12324f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123253 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12325f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123263 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x12326a | load | 1 | `abs 0x1e0bf0` | `-` |
| `movsx` at 0x123273 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x123287 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1232a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1232ae | load | 4 | `abs 0x1e06a8` | `-` |
| `mov` at 0x1232b4 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1232ba | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1232c9 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1232da | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1232e9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1232fc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123300 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123308 | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x12330c | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x12331f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12332e | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12333c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123349 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12334d | store | 4 | `(unknown)` | `0x5` |
| `mov` at 0x123354 | load | 4 | `abs 0x1e0694` | `-` |
| `mov` at 0x12335a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12336f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123373 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12337b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1233a1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1233b1 | store | 1 | `ret(_strpbrk) +0x0` | `0x0` |
| `test` at 0x1233b4 | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x1233bd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1233c1 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1233c5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1233c8 | load | 1 | `[sp+4] +0x1b0f68` | `-` |
| `cmp` at 0x1233df | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1233e6 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1233ea | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123400 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x12340d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x123413 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x123419 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12341c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123423 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123427 | load | 4 | `sp +0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123229 | `_ffscMsg 0x12154c` | `0x1230d8` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x123241 | `_msgClear 0x128a30` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x123257 | `_sprintf 0x176c40` | `?` | `0x123103` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x12327f | `_ffscMsg 0x12154c` | `0x123109` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x123298 | `_sprintf 0x176c40` | `sp-84` | `0x12313a` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x1232a6 | `_logWriteLine 0x127b84` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x1232ce | `_msgWrite 0x129300` | `?` | `[0x1e06a8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1232ed | `_sprintf 0x176c40` | `[sp+12]` | `0x123150` | `?` | `?` | `?` | `?` |
| `call` at 0x123317 | `_ffscMsg 0x12154c` | `0x12315b` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123332 | `_sprintf 0x176c40` | `[sp+12]` | `0x123150` | `?` | `?` | `?` | `?` |
| `call` at 0x123341 | `_msgClear 0x128a30` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12335f | `_msgRead 0x128cf0` | `?` | `[0x1e0694]` | `?` | `?` | `?` | `?` |
| `call` at 0x123384 | `_ffscMsg 0x12154c` | `0x12318d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1233a5 | `_strpbrk 0x18440c` | `?` | `0x1231b5` | `?` | `?` | `?` | `?` |
| `call` at 0x1233d7 | `_ffscMsg 0x12154c` | `0x1231b8` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1233f7 | `_sprintf 0x176c40` | `sp-84` | `0x1231e2` | `?` | `?` | `?` | `?` |
| `call` at 0x123405 | `_logWriteLine 0x127b84` | `?` | `sp-84` | `?` | `?` | `?` | `?` |
| `call` at 0x12342b | `_strcpy 0x1841f4` | `[sp+12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x123222 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `je` at 0x123271 | `(([0x1e0bf0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x12328c | `? == 0x0` | `? != 0x0` |
| `je` at 0x1232d8 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x1232de | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `je` at 0x123306 | `? == 0xb` | `? != 0xb` |
| `je` at 0x123323 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |
| `jne` at 0x123369 | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x123379 | `? == 0xb` | `? != 0xb` |
| `je` at 0x1233af | `(ret(_strpbrk) & ret(_strpbrk)) == 0x0` | `(ret(_strpbrk) & ret(_strpbrk)) != 0x0` |
| `je` at 0x1233bb | `(([0x1e0bf0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x1233e4 | `? == 0x0` | `? != 0x0` |
| `je` at 0x123411 | `[sp+12] == 0x0` | `[sp+12] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12343b | `eax` | `?` |

## `_elscInit`

- `_elscInit`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12345c to 0x12374c.
  Established. The `_nvramRead` and `_nvramWrite` calls begin with candidates 5 and `sp-68` (`call`
  at 0x123482, `call` at 0x1234e3). Three `_msgNew` call sites have first candidates 0xa0, 0x50 and
  0x80, each with third candidate 1 (`call` at 0x1235fe, `call` at 0x123632, `call` at 0x123669).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123465 | store | 4 | `sp -0x5c` | `0x0` |
| `mov` at 0x12346c | load | 4 | `abs 0x1f2204` | `-` |
| `mov` at 0x123471 | store | 4 | `sp -0x4c` | `[0x1f2204]` |
| `mov` at 0x123474 | load | 4 | `abs 0x1f2208` | `-` |
| `mov` at 0x123479 | store | 4 | `sp -0x48` | `[0x1f2208]` |
| `mov` at 0x12349d | store | 4 | `sp -0x7c` | `sp-68` |
| `mov` at 0x1234a5 | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1234ab | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x1234ae | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x1234c9 | load | 4 | `sp -0x7c` | `-` |
| `mov` at 0x1234cc | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x1234f5 | store | 4 | `sp -0x60` | `sp-68` |
| `mov` at 0x1234fb | store | 4 | `sp -0x64` | `sp-84` |
| `mov` at 0x123500 | store | 4 | `sp -0x68` | `0x0` |
| `mov` at 0x123507 | store | 4 | `sp -0x6c` | `0x1e10d6` |
| `mov` at 0x12350e | store | 4 | `sp -0x70` | `0x0` |
| `mov` at 0x123515 | store | 4 | `sp -0x74` | `0x1e10cc` |
| `mov` at 0x12351c | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x123526 | store | 4 | `(unknown)` | `0x0` |
| `mov` at 0x12352e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123532 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x123536 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12353a | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x123542 | load | 4 | `sp -0x68` | `-` |
| `mov` at 0x123545 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x123548 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12354b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12354f | load | 1 | `(unknown)` | `-` |
| `mov` at 0x123555 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x123559 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123560 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x123566 | load | 4 | `sp -0x74` | `-` |
| `mov` at 0x123572 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x123575 | load | 4 | `sp -0x70` | `-` |
| `test` at 0x123578 | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x12357f | load | 4 | `([sp-112] + [sp-96]) +0x4` | `-` |
| `mov` at 0x123583 | store | 4 | `(unknown)` | `[([sp-112] + [sp-96])+4]` |
| `mov` at 0x12358c | store | 4 | `(unknown)` | `0xffffffff` |
| `mov` at 0x123594 | load | 4 | `sp -0x60` | `-` |
| `mov` at 0x123597 | load | 4 | `sp -0x70` | `-` |
| `test` at 0x12359a | load | 1 | `([sp-112] + [sp-96]) +0x1c` | `-` |
| `mov` at 0x1235a8 | load | 4 | `sp -0x6c` | `-` |
| `mov` at 0x1235b4 | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x1235c1 | load | 4 | `sp -0x6c` | `-` |
| `cmp` at 0x1235cd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1235d4 | store | 4 | `(unknown)` | `0x1` |
| `mov` at 0x1235e2 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1235ee | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x1235f4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123606 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x12360e | load | 1 | `(unknown)` | `-` |
| `mov` at 0x123619 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x123625 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x12362b | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12363a | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `movsx` at 0x123642 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x12364d | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x123659 | load | 4 | `sp -0x64` | `-` |
| `mov` at 0x12365f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123671 | store | 4 | `(unknown)` | `ret(_msgNew)` |
| `mov` at 0x1236b0 | load | 4 | `sp -0x58` | `-` |
| `cmp` at 0x1236b3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1236b9 | store | 4 | `(unknown)` | `[sp-88]` |
| `mov` at 0x1236bd | store | 4 | `sp -0x5c` | `0x1` |
| `add` at 0x1236c7 | load | 4 | `sp -0x68` | `-` |
| `add` at 0x1236c7 | store | 4 | `sp -0x68` | `[sp-104]+16` |
| `add` at 0x1236cb | load | 4 | `sp -0x6c` | `-` |
| `add` at 0x1236cb | store | 4 | `sp -0x6c` | `[sp-108]+60` |
| `add` at 0x1236cf | load | 4 | `sp -0x70` | `-` |
| `add` at 0x1236cf | store | 4 | `sp -0x70` | `[sp-112]+32` |
| `add` at 0x1236d3 | load | 4 | `sp -0x74` | `-` |
| `add` at 0x1236d3 | store | 4 | `sp -0x74` | `[sp-116]+60` |
| `cmp` at 0x1236e1 | load | 4 | `sp -0x5c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123482 | `_nvramRead 0x1299c0` | `0x5` | `sp-68` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x123491 | `_bzero 0x1764e0` | `sp-68` | `0x40` | `?` | `?` | `?` | `?` |
| `call` at 0x1234c1 | `_strncpy 0x1843c8` | `?` | `0x12343c` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1234e3 | `_nvramWrite 0x129b50` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x12356a | `_bzero 0x1764e0` | `[sp-116]` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x1235ac | `_strncpy 0x1843c8` | `[sp-108]` | `([sp-112] + [sp-96])+8` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1235c5 | `_strcpy 0x1841f4` | `[sp-108]` | `0x12343c` | `?` | `?` | `?` | `?` |
| `call` at 0x1235e6 | `_sprintf 0x176c40` | `[sp-100]` | `0x123441` | `?` | `?` | `?` | `?` |
| `call` at 0x1235fe | `_msgNew 0x128ae0` | `0xa0` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x12361d | `_sprintf 0x176c40` | `[sp-100]` | `0x123448` | `?` | `?` | `?` | `?` |
| `call` at 0x123632 | `_msgNew 0x128ae0` | `0x50` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x123651 | `_sprintf 0x176c40` | `[sp-100]` | `0x12344f` | `?` | `?` | `?` | `?` |
| `call` at 0x123669 | `_msgNew 0x128ae0` | `0x80` | `?` | `0x1` | `[sp-100]` | `?` | `?` |
| `call` at 0x123693 | `_elscSetup 0x12434c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1236a4 | `_elscQueryModuleNum 0x123dbc` | `?` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x1236e7 | `_elscUpdateModuleNums 0x1244ac` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12348c | `(ret(_nvramRead) & ret(_nvramRead)) == 0x0` | `(ret(_nvramRead) & ret(_nvramRead)) != 0x0` |
| `jbe` at 0x1234db | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x12357d | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x1) != 0x0` |
| `je` at 0x12359f | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) == 0x0` | `(([([sp-112] + [sp-96])+28]:1 & 0xff) & 0x2) != 0x0` |
| `jl` at 0x1235d2 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `je` at 0x12360c | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `je` at 0x123640 | `(ret(_msgNew) & ret(_msgNew)) == 0x0` | `(ret(_msgNew) & ret(_msgNew)) != 0x0` |
| `jne` at 0x123677 | `(ret(_msgNew) & ret(_msgNew)) != 0x0` | `(ret(_msgNew) & ret(_msgNew)) == 0x0` |
| `jne` at 0x12369d | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x1236ae | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x1236b7 | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x1236db | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `je` at 0x1236e5 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1236fa | `eax` | `?` |

## `_elscPrintMsg`

- `_elscPrintMsg`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12374c to 0x12393c.
  Established. The `_logWriteLine` candidates begin `[[sp+4]+44]` and `[sp+12]` (`call` at
  0x12376a). Two `_userSendCommand` sites begin with `[0x1e0ecc]` and `[0x1e0ed0]`, followed by
  `sp-248` and `sp-736` respectively and then zero (`call` at 0x12379f, `call` at 0x123827).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123758 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12375b | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x12375e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x123766 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x123772 | load | 4 | `[sp+4] +0x14` | `-` |
| `cmp` at 0x123775 | load | 4 | `abs 0x1f4b7c` | `-` |
| `mov` at 0x12377e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123799 | load | 4 | `abs 0x1e0ecc` | `-` |
| `mov` at 0x1237cc | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1237d5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1237d9 | load | 4 | `[sp+4] +0x14` | `-` |
| `mov` at 0x1237fa | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x1237fd | load | 4 | `abs 0x1f4b7c` | `-` |
| `mov` at 0x123806 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123821 | load | 4 | `abs 0x1e0ed0` | `-` |
| `mov` at 0x12383c | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x123845 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123849 | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12376a | `_logWriteLine 0x127b84` | `[[sp+4]+44]` | `[sp+12]` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x12378e | `_sprintf 0x176c40` | `sp-248` | `0x1236fb` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x12379f | `_userSendCommand 0x12f938` | `[0x1e0ecc]` | `sp-248` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1237b1 | `_ffscMsg 0x12154c` | `0x123705` | `sp-248` | `?` | `?` | `?` | `?` |
| `call` at 0x1237e9 | `_sprintf 0x176c40` | `sp-492` | `0x12370f` | `[[sp+4]+20]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x1237f2 | `_routeDoAdminCommand 0x12c4c4` | `sp-492` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123816 | `_sprintf 0x176c40` | `sp-736` | `0x1236fb` | `[sp+8]` | `[sp+12]` | `?` | `?` |
| `call` at 0x123827 | `_userSendCommand 0x12f938` | `[0x1e0ed0]` | `sp-736` | `0x0` | `?` | `?` | `?` |
| `call` at 0x123859 | `_sprintf 0x176c40` | `sp-980` | `0x123725` | `[[sp+4]+24]` | `[sp+8]` | `[sp+12]` | `?` |
| `call` at 0x123862 | `_routeDoAdminCommand 0x12c4c4` | `sp-980` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x123763 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x12377b | `[0x1f4b7c] != [[sp+4]+20]` | `[0x1f4b7c] == [[sp+4]+20]` |
| `je` at 0x1237a9 | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x1237d2 | `[[sp+4]+20] == 0xffffffff` | `[[sp+4]+20] != 0xffffffff` |
| `jne` at 0x123803 | `[0x1f4b7c] != [[sp+4]+24]` | `[0x1f4b7c] == [[sp+4]+24]` |
| `je` at 0x12382e | `(ret(_userSendCommand) & ret(_userSendCommand)) == 0x0` | `(ret(_userSendCommand) & ret(_userSendCommand)) != 0x0` |
| `je` at 0x123842 | `[[sp+4]+24] == 0xffffffff` | `[[sp+4]+24] != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123875 | `eax` | `?` |

## `_elscProcessCommand`

- `_elscProcessCommand`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12393c to 0x123bbc.
  Established. The `_strtok_r` call has first candidates `[sp+8]`, 0x123890 and `sp-8` (`call` at
  0x12397d). A later four-byte store writes `[sp-92]` through `[sp+4] +0x10` (`mov` at 0x123a88).
  The `_elscQueryModuleNum` candidates begin `[[sp+4]+12]` and `sp-92` (`call` at 0x123a69).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123948 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12394b | load | 4 | `sp +0x8` | `-` |
| `test` at 0x12394e | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x123958 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x12396c | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x12398b | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1239d9 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1239ed | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x123a24 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x123a2b | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x123a44 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x123a65 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x123a79 | load | 4 | `[sp+4] +0x10` | `-` |
| `cmp` at 0x123a7c | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x123a85 | load | 4 | `sp -0x5c` | `-` |
| `mov` at 0x123a88 | store | 4 | `[sp+4] +0x10` | `[sp-92]` |
| `mov` at 0x123aae | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x123ab5 | load | 4 | `abs 0x1f4b7c` | `-` |
| `mov` at 0x123acc | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x123add | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x123af8 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x123b00 | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123964 | `_ffscMsg 0x12154c` | `0x123876` | `?` | `[sp+8]` | `ebx` | `esi` | `edi` |
| `call` at 0x12397d | `_strtok_r 0x184570` | `[sp+8]` | `0x123890` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x123997 | `_ffscMsg 0x12154c` | `0x123893` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1239ad | `_ffscConvertToUpperCase 0x127fd0` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1239bb | `_strcmp 0x184098` | `ret(_strtok_r)` | `0x1238ad` | `?` | `?` | `?` | `?` |
| `call` at 0x1239cd | `_strcmp 0x184098` | `ret(_strtok_r)` | `0x1238b1` | `?` | `?` | `?` | `?` |
| `call` at 0x1239e5 | `_ffscMsg 0x12154c` | `0x1238b4` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1239f2 | `_elscUpdateDisplay 0x12443c` | `[sp+4]` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x123a04 | `_strncmp 0x17abd0` | `ret(_strtok_r)` | `0x1238e0` | `0x4` | `?` | `?` | `?` |
| `call` at 0x123a18 | `_strncmp 0x17abd0` | `ret(_strtok_r)` | `0x1238e5` | `0x3` | `?` | `?` | `?` |
| `call` at 0x123a3b | `_sprintf 0x176c40` | `sp-88` | `0x1238e9` | `?` | `?` | `?` | `?` |
| `call` at 0x123a48 | `_logWriteLine 0x127b84` | `[[sp+4]+44]` | `sp-88` | `?` | `?` | `?` | `?` |
| `call` at 0x123a51 | `_elscSetup 0x12434c` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123a69 | `_elscQueryModuleNum 0x123dbc` | `[[sp+4]+12]` | `sp-92` | `?` | `?` | `?` | `?` |
| `call` at 0x123a8b | `_elscUpdateModuleNums 0x1244ac` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123aa2 | `_strcmp 0x184098` | `ret(_strtok_r)` | `0x123902` | `?` | `?` | `?` | `?` |
| `call` at 0x123ac4 | `_sprintf 0x176c40` | `sp-88` | `0x123906` | `[0x1f4b7c]` | `?` | `?` | `?` |
| `call` at 0x123ad2 | `_elscPrintMsg 0x12374c` | `[sp+4]` | `sp-88` | `[sp-8]` | `?` | `?` | `?` |
| `call` at 0x123af0 | `_sprintf 0x176c40` | `sp-348` | `0x12390d` | `?` | `[sp+8]` | `?` | `?` |
| `call` at 0x123b04 | `_logWriteLine 0x127b84` | `[[sp+4]+44]` | `sp-348` | `?` | `?` | `?` | `?` |
| `call` at 0x123b0d | `_ffscMsg 0x12154c` | `sp-348` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x123955 | `(([0x1e0bf0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x10) != 0x0` |
| `jne` at 0x123989 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `je` at 0x1239c5 | `(ret(_strcmp) & ret(_strcmp)) == 0x0` | `(ret(_strcmp) & ret(_strcmp)) != 0x0` |
| `jne` at 0x1239d7 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x123a0e | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x123a22 | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `je` at 0x123a29 | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |
| `jne` at 0x123a5b | `(ret(_elscSetup) & ret(_elscSetup)) != 0x0` | `(ret(_elscSetup) & ret(_elscSetup)) == 0x0` |
| `jne` at 0x123a73 | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) != 0x0` | `(ret(_elscQueryModuleNum) & ret(_elscQueryModuleNum)) == 0x0` |
| `je` at 0x123a7f | `[sp-92] == [[sp+4]+16]` | `[sp-92] != [[sp+4]+16]` |
| `jne` at 0x123aac | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |
| `je` at 0x123afd | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123b2a | `eax` | `?` |

## `_elscProcessInput`

- `_elscProcessInput`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123bbc to 0x123dbc.
  Established. A four-byte store places 4 through `[[sp+4]+48] +0x1c` before `_msgRead` receives
  first candidates `[[sp+4]+48]` and `[0x1e0690]` (`mov` at 0x123bfb, `call` at 0x123c0c). `je`
  takes the later command-handling region for `([[[[sp+4]+48]+8]]:1 & 0xff) == 0x14`, while
  inequality falls through to the `_elscPrintMsg` route (`je` at 0x123cb9, `call` at 0x123ce8,
  `call` at 0x123d0a).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123bc4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123bc7 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x123bcf | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x123bec | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123bf8 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123bfb | store | 4 | `[[sp+4]+48] +0x1c` | `0x4` |
| `mov` at 0x123c02 | load | 4 | `abs 0x1e0690` | `-` |
| `mov` at 0x123c08 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123c2c | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123c2f | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x123c37 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x123c3b | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123c4c | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x123c54 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x123c5a | store | 4 | `[[sp+4]+48] +0xc` | `[[[sp+4]+48]+12]-2` |
| `mov` at 0x123c6c | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x123c74 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x123c77 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x123c7e | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x123c85 | load | 4 | `abs 0x1f4b7c` | `-` |
| `mov` at 0x123c9c | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x123ca0 | store | 4 | `[[sp+4]+48] +0xc` | `?` |
| `mov` at 0x123ca3 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x123ca6 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123ca9 | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x123cac | store | 1 | `[[[sp+4]+48]+12] +0x0` | `0x0` |
| `mov` at 0x123caf | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123cb2 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x123cb5 | load | 1 | `[[[sp+4]+48]+8] +0x0` | `-` |
| `mov` at 0x123cbb | load | 4 | `[[sp+4]+48] +0xc` | `-` |
| `mov` at 0x123cbe | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x123cc1 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x123cc8 | load | 4 | `abs 0x1f4b7c` | `-` |
| `mov` at 0x123cdf | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x123ce2 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x123cfc | load | 4 | `[[sp+4]+48] +0x1c` | `-` |
| `mov` at 0x123d04 | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x123d1c | load | 4 | `[[sp+4]+48] +0x8` | `-` |
| `mov` at 0x123d20 | load | 4 | `[sp+4] +0x30` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123bd8 | `_ffscMsg 0x12154c` | `0x123b2b` | `[[sp+4]+4]` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x123bf0 | `_msgClear 0x128a30` | `[[sp+4]+48]` | `ebx` | `esi` | `?` | `?` | `?` |
| `call` at 0x123c0c | `_msgRead 0x128cf0` | `[[sp+4]+48]` | `[0x1e0690]` | `?` | `?` | `?` | `?` |
| `call` at 0x123cd7 | `_sprintf 0x176c40` | `sp-16` | `0x123b88` | `[0x1f4b7c]` | `?` | `?` | `?` |
| `call` at 0x123ce8 | `_elscPrintMsg 0x12374c` | `[sp+4]` | `sp-16` | `[[[sp+4]+48]+8]` | `?` | `?` | `?` |
| `call` at 0x123d0a | `_elscProcessCommand 0x12393c` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123d29 | `_ffscMsg 0x12154c` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x123bcd | `[[sp+4]+4] == 0x1` | `[[sp+4]+4] != 0x1` |
| `je` at 0x123c16 | `(ret(_msgRead) & ret(_msgRead)) == 0x0` | `(ret(_msgRead) & ret(_msgRead)) != 0x0` |
| `jne` at 0x123c35 | `[[[sp+4]+48]+28] != 0xa` | `[[[sp+4]+48]+28] == 0xa` |
| `jne` at 0x123c52 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |
| `je` at 0x123c72 | `[[[sp+4]+48]+28] == 0x17` | `[[[sp+4]+48]+28] != 0x17` |
| `jne` at 0x123c7c | `[[[sp+4]+48]+12] != [[[sp+4]+48]+8]` | `[[[sp+4]+48]+12] == [[[sp+4]+48]+8]` |
| `je` at 0x123cb9 | `([[[[sp+4]+48]+8]]:1 & 0xff) == 0x14` | `([[[[sp+4]+48]+8]]:1 & 0xff) != 0x14` |
| `jne` at 0x123d02 | `[[[sp+4]+48]+28] != 0xb` | `[[[sp+4]+48]+28] == 0xb` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123d38 | `eax` | `?` |

## `_elscQueryModuleNum`

- `_elscQueryModuleNum`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123dbc to 0x123f4c.
  Established. An explicit four-byte store writes 0xffffffff through `[sp+8] +0x0` (`mov` at
  0x123dd2). `_strtol` later receives candidates beginning `sp-133`, `sp-140` and 0x10 (`call` at
  0x123e7c). The following `jbe` is taken for `ret(_strtol) <= 0xff (unsigned, 32-bit)` (`jbe` at
  0x123e98).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123dc8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123dcb | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123dd2 | store | 4 | `[sp+8] +0x0` | `0xffffffff` |
| `movsx` at 0x123dfb | load | 1 | `(unknown)` | `-` |
| `or` at 0x123e0d | load | 1 | `(unknown)` | `-` |
| `or` at 0x123e0d | store | 1 | `(unknown)` | `?` |
| `movsx` at 0x123e48 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x123e86 | load | 4 | `sp -0x8c` | `-` |
| `mov` at 0x123e8c | load | 1 | `[sp-140] +0x0` | `-` |
| `movsx` at 0x123ea2 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x123ec9 | store | 4 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123de5 | `_elscDoAdminCommand 0x1231fc` | `[sp+4]` | `0x123d39` | `sp-136` | `?` | `?` | `?` |
| `call` at 0x123e08 | `_ffscMsg 0x12154c` | `0x123d3d` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123e34 | `_strncmp 0x17abd0` | `sp-136` | `0x123d68` | `0x2` | `?` | `?` | `?` |
| `call` at 0x123e56 | `_ffscMsg 0x12154c` | `0x123d6b` | `?` | `sp-136` | `?` | `?` | `?` |
| `call` at 0x123e7c | `_strtol 0x183c10` | `sp-133` | `sp-140` | `0x10` | `?` | `?` | `?` |
| `call` at 0x123eb0 | `_ffscMsg 0x12154c` | `0x123d8d` | `?` | `sp-133` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x123dd0 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x123def | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` |
| `je` at 0x123e3e | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x123e90 | `(([[sp-140]]:1 & 0xff) & ([[sp-140]]:1 & 0xff)) != 0x0` | `(([[sp-140]]:1 & 0xff) & ([[sp-140]]:1 & 0xff)) == 0x0` |
| `jbe` at 0x123e98 | `ret(_strtol) <= 0xff (unsigned, 32-bit)` | `ret(_strtol) > 0xff (unsigned, 32-bit)` |
| `jne` at 0x123ebe | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x123ec7 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x123ed9 | `eax` | `?` |

## `_elscSendCommand`

- `_elscSendCommand`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123f4c to 0x1240ec.
  Established. Three distinct four-byte store sites write 0xb, 1 and 5 to unresolved effective
  targets (`mov` at 0x123f97, `mov` at 0x123fd3, `mov` at 0x124070). `_msgWrite` has an unknown
  first candidate and `[0x1e06a8]` as its second (`call` at 0x124002). The recorded exit has EAX
  zero (`ret` at 0x1240a9).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x123f55 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x123f58 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x123f5e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x123f6d | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x123f7a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123f86 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123f8a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123f93 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123f97 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x123fac | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123fbf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123fc3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123fcf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x123fd3 | store | 4 | `(unknown)` | `0x1` |
| `test` at 0x123fda | load | 1 | `abs 0x1e0bf0` | `-` |
| `movsx` at 0x123fe3 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x123ff7 | load | 4 | `abs 0x1e06a8` | `-` |
| `mov` at 0x123ffd | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124013 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124017 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12402c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124030 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124038 | load | 4 | `(unknown)` | `-` |
| `movsx` at 0x12403c | load | 1 | `(unknown)` | `-` |
| `mov` at 0x124054 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124058 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12406c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124070 | store | 4 | `(unknown)` | `0x5` |
| `cmp` at 0x124077 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124091 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x123f72 | `_msgClear 0x128a30` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x123f8e | `_sprintf 0x176c40` | `?` | `0x123eda` | `?` | `?` | `?` | `?` |
| `call` at 0x123fb1 | `_msgClear 0x128a30` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x123fc7 | `_sprintf 0x176c40` | `?` | `0x123103` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x123fef | `_ffscMsg 0x12154c` | `0x123ee6` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x124002 | `_msgWrite 0x129300` | `?` | `[0x1e06a8]` | `?` | `?` | `?` | `?` |
| `call` at 0x12401b | `_strcpy 0x1841f4` | `?` | `0x123150` | `?` | `?` | `?` | `?` |
| `call` at 0x124047 | `_ffscMsg 0x12154c` | `0x123f09` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x12405c | `_strcpy 0x1841f4` | `?` | `0x123150` | `?` | `?` | `?` | `?` |
| `call` at 0x124088 | `_sprintf 0x176c40` | `sp-124` | `0x123f35` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x124096 | `_logWriteLine 0x127b84` | `?` | `sp-124` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x123f7f | `? == 0x1` | `? != 0x1` |
| `je` at 0x123fe1 | `(([0x1e0bf0]:1 & 0xff) & 0x10) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x12400c | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x124036 | `? == 0xb` | `? != 0xb` |
| `je` at 0x12407c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1240a9 | `eax` | `0x0` |

## `_elscSendPassword`

- `_elscSendPassword`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1240ec to 0x1241ac.
  Established. The first `_elscDoAdminCommand` candidates are `[sp+4]`, `sp-88` and `sp-220` (`call`
  at 0x12414f). Earlier `jg` selects its taken route for `[sp+8] > 0x1f3 (signed, 32-bit)` and falls
  through for `[sp+8] <= 0x1f3 (signed, 32-bit)` (`jg` at 0x124114).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1240f7 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x124104 | load | 1 | `(unknown)` | `-` |
| `cmp` at 0x12410d | load | 4 | `sp +0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12411f | `_sprintf 0x176c40` | `sp-88` | `0x1240aa` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x12413b | `_sprintf 0x176c40` | `sp-88` | `0x1240ae` | `?` | `ebx` | `esi` | `?` |
| `call` at 0x12414f | `_elscDoAdminCommand 0x1231fc` | `[sp+4]` | `sp-88` | `sp-220` | `?` | `?` | `?` |
| `call` at 0x124163 | `_strncmp 0x17abd0` | `sp-220` | `0x123d68` | `0x2` | `?` | `?` | `?` |
| `call` at 0x124182 | `_ffscMsg 0x12154c` | `0x1240b5` | `sp-220` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12410b | `? == 0x0` | `? != 0x0` |
| `jg` at 0x124114 | `[sp+8] > 0x1f3 (signed, 32-bit)` | `[sp+8] <= 0x1f3 (signed, 32-bit)` |
| `jne` at 0x124159 | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x12416d | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x124197 | `eax` | `?` |

## `_elscSetPassword`

- `_elscSetPassword`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1241ac to 0x12434c.
  Established. Calls `_elscSendPassword` with first candidates `[sp+4]` and 0x3e8 (`call` at
  0x124235). The later `_elscDoAdminCommand` candidates begin `[sp+4]`, `sp-88` and zero (`call` at
  0x12425d). Two four-byte sites write 0xb to unknown destinations (`mov` at 0x124216, `mov` at
  0x1242d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1241b5 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1241b8 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x1241be | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1241cd | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1241e2 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1241ec | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1241ef | load | 1 | `[sp+8] +0x0` | `-` |
| `cmp` at 0x1241f9 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124205 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124209 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124212 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124216 | store | 4 | `(unknown)` | `0xb` |
| `mov` at 0x124231 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x124241 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x124259 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12426e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124272 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124282 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124291 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x124295 | load | 4 | `sp +0x4` | `-` |
| `sub` at 0x12429b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1242bf | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1242c3 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1242cc | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1242d0 | store | 4 | `(unknown)` | `0xb` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1241d2 | `_msgClear 0x128a30` | `?` | `ebx` | `esi` | `edi` | `?` | `?` |
| `call` at 0x12420d | `_sprintf 0x176c40` | `?` | `0x123eda` | `?` | `?` | `?` | `?` |
| `call` at 0x124235 | `_elscSendPassword 0x1240ec` | `[sp+4]` | `0x3e8` | `?` | `?` | `?` | `?` |
| `call` at 0x12424e | `_sprintf 0x176c40` | `sp-88` | `0x124198` | `[sp+8]` | `?` | `?` | `?` |
| `call` at 0x12425d | `_elscDoAdminCommand 0x1231fc` | `[sp+4]` | `sp-88` | `0x0` | `?` | `?` | `?` |
| `call` at 0x124276 | `_strcmp 0x184098` | `?` | `0x123d68` | `?` | `?` | `?` | `?` |
| `call` at 0x124287 | `_msgClear 0x128a30` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1242a8 | `_strncpy 0x1843c8` | `?` | `[sp+8]` | `0x4` | `?` | `?` | `?` |
| `call` at 0x1242b0 | `_elscUpdateSaveInfo 0x12452c` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1242c7 | `_sprintf 0x176c40` | `?` | `0x1241a3` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1241dc | `([sp+12] & [sp+12]) == 0x0` | `([sp+12] & [sp+12]) != 0x0` |
| `je` at 0x1241e6 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `je` at 0x1241f3 | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) == 0x0` | `(([[sp+8]]:1 & 0xff) & ([[sp+8]]:1 & 0xff)) != 0x0` |
| `je` at 0x1241fe | `? == 0x1` | `? != 0x1` |
| `jne` at 0x12423f | `(ret(_elscSendPassword) & ret(_elscSendPassword)) != 0x0` | `(ret(_elscSendPassword) & ret(_elscSendPassword)) == 0x0` |
| `jne` at 0x124267 | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `jne` at 0x124280 | `(ret(_strcmp) & ret(_strcmp)) != 0x0` | `(ret(_strcmp) & ret(_strcmp)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1242e2 | `eax` | `?` |

## `_elscSetup`

- `_elscSetup`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12434c to 0x12443c.
  Established. The two `_elscDoAdminCommand` calls begin with `[[sp+4]+12]`, then 0x1242f7 and
  `sp-196` at the first site or 0x124321 and zero at the second (`call` at 0x124387, `call` at
  0x1243df). Both explicit four-byte stores through `[sp+4] +0x0` have values the tool leaves
  unknown (`mov` at 0x1243bd, `mov` at 0x1243eb).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x124357 | load | 4 | `sp +0x4` | `-` |
| `test` at 0x12435a | load | 1 | `abs 0x1e0bf0` | `-` |
| `mov` at 0x124363 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x124383 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1243a8 | load | 1 | `[sp+4] +0x2b` | `-` |
| `mov` at 0x1243b9 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1243bd | store | 4 | `[sp+4] +0x0` | `?` |
| `mov` at 0x1243cc | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1243db | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1243e7 | load | 4 | `[sp+4] +0x0` | `-` |
| `mov` at 0x1243eb | store | 4 | `[sp+4] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12436f | `_ffscMsg 0x12154c` | `0x1242e3` | `?` | `ebx` | `esi` | `?` | `?` |
| `call` at 0x124387 | `_elscDoAdminCommand 0x1231fc` | `[[sp+4]+12]` | `0x1242f7` | `sp-196` | `?` | `?` | `?` |
| `call` at 0x12439b | `_strncmp 0x17abd0` | `sp-196` | `0x123d68` | `0x2` | `?` | `?` | `?` |
| `call` at 0x1243b4 | `_ffscMsg 0x12154c` | `0x1242fd` | `?` | `sp-196` | `?` | `?` | `?` |
| `call` at 0x1243df | `_elscDoAdminCommand 0x1231fc` | `[[sp+4]+12]` | `0x124321` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1243f2 | `_ffscMsg 0x12154c` | `0x124327` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x124361 | `(([0x1e0bf0]:1 & 0xff) & 0x2) == 0x0` | `(([0x1e0bf0]:1 & 0xff) & 0x2) != 0x0` |
| `jne` at 0x124391 | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) != 0x0` | `(ret(_elscDoAdminCommand) & ret(_elscDoAdminCommand)) == 0x0` |
| `je` at 0x1243a5 | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x1243d2 | `[[sp+4]+4] != 0x1` | `[[sp+4]+4] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x124404 | `eax` | `?` |

## `_elscUpdateDisplay`

- `_elscUpdateDisplay`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12443c to 0x1244ac.
  Established. The `_bzero` and `_strncpy` calls share first candidate `[sp+4]+28` and their second
  candidates are 0xa and `[sp+8]` respectively, with third `_strncpy` candidate 0xa (`call` at
  0x124451, `call` at 0x124461). A later byte read renders as `[sp+4] +0x24` (`mov` at 0x124470).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x124445 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x124448 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x124469 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x124470 | load | 1 | `[sp+4] +0x24` | `-` |
| `mov` at 0x12448a | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x124451 | `_bzero 0x1764e0` | `[sp+4]+28` | `0xa` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x124461 | `_strncpy 0x1843c8` | `[sp+4]+28` | `[sp+8]` | `0xa` | `?` | `?` | `?` |
| `call` at 0x124481 | `_sprintf 0x176c40` | `sp-84` | `0x124405` | `[sp+4]+28` | `?` | `?` | `?` |
| `call` at 0x12448e | `_logWriteLine 0x127b84` | `[[sp+4]+44]` | `sp-84` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x12445b | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |
| `je` at 0x12446e | `([[sp+4]+44] & [[sp+4]+44]) == 0x0` | `([[sp+4]+44] & [[sp+4]+44]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12449c | `eax` | `?` |

## `_elscUpdateModuleNums`

- `_elscUpdateModuleNums`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1244ac to 0x12452c.
  Established. The `rep movsd` site has a four-byte access at absolute 0x1f4b70 and a four-byte
  store with unknown target and value (`rep movsd` at 0x1244c2). The following
  `_addrdUpdateLocalIdentity` call has first candidate `sp-32`, and its result is retained in EAX at
  return (`call` at 0x1244e5, `ret` at 0x1244f2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `rep movsd` at 0x1244c2 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x1244c2 | load | 4 | `abs 0x1f4b70` | `-` |
| `mov` at 0x1244d0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1244d4 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1244e5 | `_addrdUpdateLocalIdentity 0x10ddd0` | `sp-32` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1244df | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1244f2 | `eax` | `ret(_addrdUpdateLocalIdentity)` |

## `_elscUpdateSaveInfo`

- `_elscUpdateSaveInfo`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12452c to 0x1245d8.
  Established. Calls `_nvramWrite` with candidates beginning 5 and `sp-68` (`call` at 0x12459e). A
  prior four-byte store site writes value 3 to an unknown target (`mov` at 0x12456e). The
  nonzero write result takes `jne` to `_ffscMsg`, while zero falls through to the exit (`jne` at
  0x1245a8, `call` at 0x1245c1).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x124545 | store | 4 | `sp -0x48` | `sp-68` |
| `mov` at 0x124548 | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x12454b | store | 4 | `sp -0x54` | `0x1e10d6` |
| `mov` at 0x124559 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x12455f | store | 4 | `(unknown)` | `?` |
| `mov` at 0x124561 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x124567 | load | 4 | `sp -0x48` | `-` |
| `mov` at 0x12456a | store | 4 | `(unknown)` | `?` |
| `mov` at 0x12456e | store | 4 | `(unknown)` | `0x3` |
| `mov` at 0x124578 | load | 4 | `sp -0x54` | `-` |
| `add` at 0x12458b | load | 4 | `sp -0x54` | `-` |
| `add` at 0x12458b | store | 4 | `sp -0x54` | `[sp-84]+60` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12453b | `_bzero 0x1764e0` | `sp-68` | `0x40` | `ebx` | `esi` | `edi` | `?` |
| `call` at 0x124580 | `_strncpy 0x1843c8` | `?` | `?` | `0x4` | `?` | `?` | `?` |
| `call` at 0x12459e | `_nvramWrite 0x129b50` | `0x5` | `sp-68` | `?` | `?` | `?` | `?` |
| `call` at 0x1245c1 | `_ffscMsg 0x12154c` | `0x1244f3` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x124596 | `? <= 0x1 (unsigned, 32-bit)` | `? > 0x1 (unsigned, 32-bit)` |
| `jne` at 0x1245a8 | `(ret(_nvramWrite) & ret(_nvramWrite)) != 0x0` | `(ret(_nvramWrite) & ret(_nvramWrite)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1245d4 | `eax` | `?` |

## `_routeDoAdminCommand`

- `_routeDoAdminCommand`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12c4c4 to 0x12c4e4.
  Established. Reads four bytes at 0x1e0edc before `_ProcessCommand` receives first candidates
  `[sp+4]` and `[0x1e0edc]` (`mov` at 0x12c4ca, `call` at 0x12c4d1). The return preserves
  `ret(_ProcessCommand)` in EAX (`ret` at 0x12c4d9).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x12c4c7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x12c4ca | load | 4 | `abs 0x1e0edc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x12c4d1 | `_ProcessCommand 0x12ca84` | `[sp+4]` | `[0x1e0edc]` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x12c4d9 | `eax` | `ret(_ProcessCommand)` |
