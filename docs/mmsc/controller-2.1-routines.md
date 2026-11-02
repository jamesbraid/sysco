# MMSC 2.1 controller command paths

These entries cover MMSC 2.1 in `6.5.5/usr/cpu/firmware/mmscfw.bin` only.
Image SHA-256: `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
The x86 a.out image loads at `0x108000`; the [load map](load-map.md) describes its segments.
Analysed spans end at the next text symbol, an inferred boundary that can include trailing literals.

`sp` denotes the entry stack pointer. Brackets denote loaded values. Call arguments are
candidate stack locations under assumed cdecl32, with unknown arity. Entries establish the
listed local effects and call routes. Hardware roles and callee effects remain unknown here.

[Fact sheets and complete decoded listings](disasm/controller-2.1-facts.txt).

## `_elscCheckResponse`

- `_elscCheckResponse`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12304c to 0x1231ec.
  Established. Contains two byte-zero store sites through the result of `_strchr` (`mov` at
  0x12305c, `mov` at 0x12306e). The second is bypassed by `je` when that call returns zero (`je` at
  0x12306c). A later `_strchr` call has unknown first candidate and 0x14 second candidate; nonzero
  result takes the backward `jne` to the first store (`call` at 0x12309b, `jne` at 0x1230a7). The
  body includes `_elscProcessCommand` with first candidate `[sp+4]` and unknown second candidate
  (`call` at 0x12308d). `_memmove` receives `[sp+8]`, unknown and `ret(_strlen)` (`call` at
  0x1230b9). The branch that can skip the length/copy calls resolves only to `Z == 1`, with compared
  values unknown (`je` at 0x1230ab).

## `_elscDoAdminCommand`

- `_elscDoAdminCommand`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1231ec to 0x12344c.
  Established. Uses `_msgWrite` with unknown first candidate and `[0x1e0698]` second candidate
  (`call` at 0x1232be); its zero result takes `je` into the next comparison region; nonzero falls
  through to a test of `[sp+12]` (`je` at 0x1232c8, `cmp` at 0x1232ca). `_msgRead` later receives
  unknown and `[0x1e0684]` (`call` at 0x12334f). A nonzero read-call result takes `jne` to the
  all-ones EAX load; zero falls through (`jne` at 0x123359, `mov` at 0x12321e). Two four-byte stores
  have unresolved targets and values 1 and 5 (`mov` at 0x123253, `mov` at 0x12333d). A zero
  `_strpbrk` result takes `je` past a byte-zero store through that result (`je` at 0x12339f, `mov`
  at 0x1233a1). The final `je` skips `_strcpy` for zero `[sp+12]`; otherwise the call has `[sp+12]`
  and unknown as its first two candidates (`je` at 0x123401, `call` at 0x12341b).

## `_elscInit`

- `_elscInit`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12344c to 0x12373c.
  Established. Calls `_nvramRead` with candidates 5 and `sp-68` (`call` at 0x123472). Zero result
  takes `je` past the region containing `_bzero` and `_nvramWrite`; nonzero falls through to `_bzero` candidates `sp-68`
  and 0x40 (`je` at 0x12347c, `call` at 0x123481). That region includes `_nvramWrite` candidates 5
  and `sp-68`, after a backward `jbe` with unresolved unsigned value at most 1 (`call` at 0x1234d3,
  `jbe` at 0x1234cb). Three later `_msgNew` calls have respective first candidates 0xa0, 0x50 and
  0x80; each has unknown second candidate, 1 third candidate and `[sp-100]` fourth candidate (`call`
  at 0x1235ee, `call` at 0x123622, `call` at 0x123659). `_elscSetup` has an unknown first candidate
  (`call` at 0x123683); its nonzero result takes `jne` past `_elscQueryModuleNum`, whose first two
  candidates are unknown and `sp-88` (`jne` at 0x12368d, `call` at 0x123694). The final `je` skips
  `_elscUpdateModuleNums` when `[sp-92]` is zero (`je` at 0x1236d5, `call` at 0x1236d7).

## `_elscPrintMsg`

- `_elscPrintMsg`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12373c to 0x12392c.
  Established. Reads four bytes through `[sp+4] +0x2c`; zero takes `je` past
  `_logWriteLine`, whose candidates begin `[[sp+4]+44]` and `[sp+12]` (`mov` at 0x12374e, `je` at
  0x123753, `call` at 0x12375a). When `[0x1f4b64] != [[sp+4]+20]`, `jne` selects the alternate
  region (`jne` at 0x12376b). Equality falls through to a route containing `_userSendCommand`
  candidates `[0x1e0ebc]`, `sp-248` and zero (`jne` at 0x12376b, `call` at 0x12378f). A nonzero
  result falls through the following `je` to `_ffscMsg` and exits that route before the second
  field's region (`je` at 0x123799, `call` at 0x1237a1, `jmp` at 0x1237ab). The alternate region
  skips its `_routeDoAdminCommand` when `[[sp+4]+20]` equals 0xffffffff (`je` at 0x1237c2, `call` at
  0x1237e2). The later field is read through `[sp+4] +0x18`; its region includes `_userSendCommand`
  candidates `[0x1e0ec0]`, `sp-736`, zero, and an alternative `_routeDoAdminCommand` with `sp-980`
  (`mov` at 0x1237ea, `call` at 0x123817, `call` at 0x123852).

## `_elscProcessCommand`

- `_elscProcessCommand`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12392c to 0x123bac.
  Established. Passes `[sp+8]`, 0x123880 and `sp-8` to `_strtok_r` (`call` at 0x12396d). A nonzero
  result takes `jne` into the region calling `_ffscConvertToUpperCase` with that result (`jne` at
  0x123979, `call` at 0x12399d). Later `_elscSetup` receives `[sp+4]`; its nonzero result takes
  `jne` past `_elscQueryModuleNum` (`call` at 0x123a41, `jne` at 0x123a4b, `call` at 0x123a59). When
  reached, the query call's candidates are `[[sp+4]+12]` and `sp-92` (`call` at 0x123a59). The
  following nonzero-result `jne` skips a field comparison and update (`jne` at 0x123a63). The
  equality `je` then skips a four-byte store of `[sp-92]` through `[sp+4] +0x10` and
  `_elscUpdateModuleNums` when `[sp-92] == [[sp+4]+16]` (`je` at 0x123a6f, `mov` at 0x123a78, `call`
  at 0x123a7b). Another path includes `_elscPrintMsg` candidates `[sp+4]`, `sp-88` and `[sp-8]`
  (`call` at 0x123ac2).

## `_elscProcessInput`

- `_elscProcessInput`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123bac to 0x123dac.
  Established. Only equality of `[[sp+4]+4]` with 1 takes `je` to the region calling `_msgClear` and
  `_msgRead` (`je` at 0x123bbd, `call` at 0x123be0, `call` at 0x123bfc). The read-call candidates
  begin `[[sp+4]+48]` and `[0x1e0680]` (`call` at 0x123bfc). A four-byte store writes 4 through
  `[[sp+4]+48] +0x1c` before that call (`mov` at 0x123beb). The later `jne` skips the four-byte
  update through `[[sp+4]+48] +0xc` when `[[[sp+4]+48]+28] != 0xb`; equality falls through to a
  store of `[[[sp+4]+48]+12]-2` (`jne` at 0x123c42, `mov` at 0x123c4a). Another path contains a
  byte-zero store through `[[[sp+4]+48]+12] +0x0` (`mov` at 0x123c9c). The byte comparison's `je` is
  taken for value 0x14 at `[[[sp+4]+48]+8] +0x0` (`je` at 0x123ca9, `mov` at 0x123ca5). In that
  region, inequality of `[[[sp+4]+48]+28]` with 0xb skips `_elscProcessCommand`; otherwise its
  candidates begin `[sp+4]` and unknown (`jne` at 0x123cf2, `call` at 0x123cfa).

## `_elscQueryModuleNum`

- `_elscQueryModuleNum`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123dac to 0x123f3c.
  Established. Skips a four-byte store of 0xffffffff through `[sp+8] +0x0` when `[sp+8]` is zero
  (`je` at 0x123dc0, `mov` at 0x123dc2). `_elscDoAdminCommand` receives `[sp+4]`, 0x123d29 and
  `sp-136` (`call` at 0x123dd5); its zero result takes `je` to `_strncmp` candidates `sp-136`,
  0x123d58 and 2 (`je` at 0x123ddf, `call` at 0x123e24). A zero comparison result takes `je` into
  the `_strtol` region, with candidates `sp-133`, `sp-140` and 0x10 (`je` at 0x123e2e, `call` at
  0x123e6c). The byte loaded through `[sp-140] +0x0` controls `jne`: nonzero takes the diagnostic
  route (`mov` at 0x123e7c, `jne` at 0x123e80). Otherwise `jbe` admits an unsigned `_strtol` result
  at most 0xff to the later region (`jbe` at 0x123e88). The final four-byte store through `[sp+8]
  +0x0` has unknown value and is skipped by `je` for zero `[sp+8]` (`mov` at 0x123eb9, `je` at
  0x123eb7).

## `_elscSendCommand`

- `_elscSendCommand`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x123f3c to 0x1240dc.
  Established. Contains three four-byte store sites with unknown effective addresses and respective
  values 0xb, 1 and 5 (`mov` at 0x123f87, `mov` at 0x123fc3, `mov` at 0x124060). The initial `je`
  reaches the second `_msgClear` region when an unresolved value equals 1 (`je` at 0x123f6f, `call`
  at 0x123fa1). `_msgWrite` receives unknown and `[0x1e0698]` (`call` at 0x123ff2). Zero result
  takes `je` to the next comparison, while nonzero falls through to `_strcpy` with unknown first
  candidate and 0x123140 second candidate (`je` at 0x123ffc, `call` at 0x12400b). The later `je`
  reaches the store of 5 only when another unknown value equals 0xb (`je` at 0x124026, `mov` at
  0x124060). The recorded exit carries EAX zero (`ret` at 0x124099).

## `_elscSendPassword`

- `_elscSendPassword`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1240dc to 0x12419c.
  Established. A comparison against zero controls `je`, which skips the later command route when the
  compared value is zero; that value's effective address is unknown (`je` at 0x1240fb, `cmp` at
  0x1240f4). `jg` selects the second formatter for signed `[sp+8] > 0x1f3`, otherwise falling
  through to the first (`jg` at 0x124104). The first `_sprintf` candidates begin `sp-88`, 0x12409a;
  the second begins `sp-88`, 0x12409e and unknown (`call` at 0x12410f, `call` at 0x12412b).
  `_elscDoAdminCommand` receives `[sp+4]`, `sp-88` and `sp-220` (`call` at 0x12413f). Nonzero result
  takes `jne` past `_strncmp`, whose candidates are `sp-220`, 0x123d58 and 2 (`jne` at 0x124149,
  `call` at 0x124153). A nonzero comparison result takes `jne` to the path calling `_ffscMsg` with
  0x1240a5 and `sp-220` (`jne` at 0x12415d, `call` at 0x124172).

## `_elscSetPassword`

- `_elscSetPassword`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12419c to 0x12433c.
  Established. Three `je` sites can reach the copy region directly: zero `[sp+12]`, zero `[sp+8]`,
  or zero byte through `[sp+8] +0x0` (`je` at 0x1241cc, `je` at 0x1241d6, `je` at 0x1241e3, `mov` at
  0x1241df). On the other route, `_elscSendPassword` has candidates `[sp+4]` and 0x3e8 (`call` at
  0x124225); its nonzero result takes `jne` to the all-ones EAX load (`jne` at 0x12422f, `mov` at
  0x12420d). A later `_elscDoAdminCommand` has candidates `[sp+4]`, `sp-88` and zero; its nonzero
  result takes the same failure route (`call` at 0x12424d, `jne` at 0x124257). The copy region calls
  `_strncpy` with unknown destination, `[sp+8]` and 4, followed by `_elscUpdateSaveInfo` with
  unknown candidates (`call` at 0x124298, `call` at 0x1242a0). Four-byte stores of 0xb occur at two
  unresolved destinations (`mov` at 0x124206, `mov` at 0x1242c0).

## `_elscSetup`

- `_elscSetup`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12433c to 0x12442c.
  Established. Calls `_elscDoAdminCommand` with `[[sp+4]+12]`, 0x1242e7 and `sp-196` (`call` at
  0x124377). A nonzero result takes `jne` past `_strncmp`; zero falls through to that call with
  `sp-196`, 0x123d58 and 2 (`jne` at 0x124381, `call` at 0x12438b). The comparison's zero result
  takes `je` to the later field test, while nonzero falls through to `_ffscMsg` (`je` at 0x124395,
  `call` at 0x1243a4). A following `jne` skips the second admin call when `[[sp+4]+4] != 1` (`jne`
  at 0x1243c2). Equality falls through to `_elscDoAdminCommand` candidates `[[sp+4]+12]`, 0x124311
  and zero (`jne` at 0x1243c2, `call` at 0x1243cf). Two four-byte stores through `[sp+4] +0x0` have
  unknown values (`mov` at 0x1243ad, `mov` at 0x1243db).

## `_elscUpdateDisplay`

- `_elscUpdateDisplay`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12442c to 0x12449c.
  Established. Uses `_bzero` candidates `[sp+4]+28` and 0xa (`call` at 0x124441). A zero `[sp+8]`
  takes `je` past `_strncpy`, which otherwise receives `[sp+4]+28`, `[sp+8]` and 0xa (`je` at
  0x12444b, `call` at 0x124451). The later `je` skips formatting and logging for zero `[[sp+4]+44]`
  (`je` at 0x12445e). Nonzero falls through to `_sprintf` candidates beginning `sp-84`, 0x1243f5,
  `[sp+4]+28` and unknown, then `_logWriteLine` candidates `[[sp+4]+44]` and `sp-84` (`je` at
  0x12445e, `call` at 0x124471, `call` at 0x12447e). The merged exit value is unknown (`ret` at
  0x12448c).

## `_elscUpdateModuleNums`

- `_elscUpdateModuleNums`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12449c to 0x12451c.
  Established. Has unsupported effects at `cld`, `rep movsd` and the loop increment (`cld` at
  0x1244ac, `rep movsd` at 0x1244b2, `inc` at 0x1244cb). The two collected four-byte store sites
  have unknown targets and values (`rep movsd` at 0x1244b2, `mov` at 0x1244c4). The backward `jbe`
  repeats the region for an unresolved unsigned value at most 1 and falls through for a greater
  value (`jbe` at 0x1244cf). It then calls `_addrdUpdateLocalIdentity` with first candidate `sp-32`
  and unknown later candidates (`call` at 0x1244d5). The return preserves
  `ret(_addrdUpdateLocalIdentity)` (`ret` at 0x1244e2).

## `_elscUpdateSaveInfo`

- `_elscUpdateSaveInfo`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12451c to 0x1245c8.
  Established. Begins with `_bzero` candidates `sp-68` and 0x40 (`call` at 0x12452b). The loop
  includes three four-byte stores with unresolved destinations; the third writes 3 and the other
  values are unknown (`mov` at 0x12454f, `mov` at 0x12455a, `mov` at 0x12455e). `_strncpy` receives
  unknown, unknown and 4 (`call` at 0x124570). After an unsupported increment, backward `jbe`
  repeats for an unresolved unsigned value at most 1 (`inc` at 0x124582, `jbe` at 0x124586).
  `_nvramWrite` then receives 5 and `sp-68` (`call` at 0x12458e); its nonzero result takes `jne` to
  the `_ffscMsg` route and an all-ones EAX load, whereas zero falls through to a zero EAX load
  (`jne` at 0x124598, `call` at 0x1245b1, `mov` at 0x1245b6, `xor` at 0x12459a).

## `_routeDoAdminCommand`

- `_routeDoAdminCommand`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x12c4b4 to 0x12c4d4.
  Established. Loads four bytes at 0x1e0ecc (`mov` at 0x12c4ba); its `_ProcessCommand` call has
  candidates `[sp+4]` and `[0x1e0ecc]` (`call` at 0x12c4c1). The return forwards
  `ret(_ProcessCommand)` (`ret` at 0x12c4c9). Decoding stops after this reachable return; the
  trailing bytes do not establish another action.
