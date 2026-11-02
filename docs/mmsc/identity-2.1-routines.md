# MMSC 2.1 identity-table and NVRAM callers

These entries cover only MMSC 2.1 in `6.5.5/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
The x86 a.out image maps at `0x108000`. The [load map](load-map.md)
describes its segments. Analysed spans end at the next text symbol and
can include trailing literals. Their end boundaries are inferred.

Here `sp` is the stack pointer at entry. Brackets denote loaded values.
Call arguments below are candidate locations under the assumed cdecl32
convention, with unknown arity. Unknown values, loop effects and callee
purposes are not filled from another build.

[Full fact sheets and listings](disasm/identity-2.1-facts.txt).

## `_identInit`

- `_identInit`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125560 to 0x1256f0.
  Established. Calls `_bzero` with candidates 0x1f4b58 and 0x1c, then stores four-byte zeros at
  0x1f1cd0 and 0x1f50c4 (`call` at 0x12556a, `mov` at 0x125572, `mov` at 0x12557c). `_nvramRead`
  receives candidates 1 and 0x1e1150 (`call` at 0x1255dc); its zero result takes `je` past the
  alternative path (`je` at 0x1255e6). Within that path, `jbe` skips the four-byte store of 0xf
  at 0x1e1150 for an unknown unsigned 32-bit value at most 0xffffee (`jbe` at 0x12562d,
  `mov` at 0x12562f). A greater value falls through to that store (`jbe` at 0x12562d,
  `mov` at 0x12562f). Both routes then store 0x64 and 0xff000000 as four bytes at 0x1e1154 and
  0x1e1158, followed by `_nvramWrite` candidates 1 and 0x1e1150 (`mov` at 0x125639,
  `mov` at 0x125643, `call` at 0x125654). A later
  `_nvramRead` has candidates 2 and 0x1f4b64 (`call` at 0x12567c). `jbe` skips the four-byte store
  of 1 at that address when `[0x1f4b64] <= 0x14` unsigned (`jbe` at 0x125699). A greater value falls
  through to the store (`jbe` at 0x125699, `mov` at 0x12569b). The two earlier loops have
  unresolved indexed store destinations and unsupported increments (`mov` at 0x12559e,
  `mov` at 0x1255b8, `inc` at 0x1255a4, `inc` at 0x1255c6).

## `_identSetRackID`

- `_identSetRackID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125f30 to 0x125fa0.
  Established. Stores `[sp+4]` as four bytes at 0x1f4b64 and calls `_nvramWrite` with candidates 2
  and 0x1f4b64 (`mov` at 0x125f3b, `call` at 0x125f48). A zero write-call result takes `je` past
  `_ffscMsg` (`je` at 0x125f52). Nonzero falls through to that call (`je` at 0x125f52, `call` at
  0x125f59). Both routes reach a four-byte store of 1 at 0x1f13f8 (`mov` at 0x125f61). The recorded
  exit has EAX zero (`xor` at 0x125f8c, `ret` at 0x125f96).

## `_identSetCapabilities`

- `_identSetCapabilities`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125c80 to 0x125d60.
  Established. Copies `[sp+4]` into the four-byte value at 0x1f4b68 (`mov` at 0x125c86). `je` skips
  `_ffscMsg` when bit 0 of the byte at 0x1e0be1 is clear (`test` at 0x125c8b, `je` at 0x125c92). The
  set-bit path calls it with candidates 0x125c55 and `[sp+4]` (`je` at 0x125c92, `call` at
  0x125c9a). EAX is set to zero before the return (`xor` at 0x125c9f, `ret` at 0x125ca4).

## `_identSetModuleNums`

- `_identSetModuleNums`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125e50 to 0x125f30.
  Established. Contains a four-byte store with unknown effective address and unknown value (`mov` at
  0x125e6b). The first backward `jbe` has unresolved operands, and the increment affecting that loop
  is unsupported (`jbe` at 0x125e78, `inc` at 0x125e75). `je` skips the later calls when bit 0 of
  the byte at 0x1e0be1 is clear (`test` at 0x125e7a, `je` at 0x125e81). When reached,
  `_ffscModuleNumToString` has unknown first candidate and `sp-16` second candidate, followed by
  `_ffscMsgN` candidates 0x125e41 and `sp-16` (`call` at 0x125ea5, `call` at 0x125eb3). A second
  unsupported increment precedes the other backward `jbe` (`inc` at 0x125ebe, `jbe` at 0x125ec5).
  The exit records EAX zero (`xor` at 0x125ed1, `ret` at 0x125edc).

## `_identUpdate`

- `_identUpdate`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125a40 to 0x125b70.
  Established. Starts with `_identFindIdentityNodeByUniqueID` using first candidate `[sp+4]` (`call`
  at 0x125a49); its zero result takes `je` to `_malloc` with candidate 0x2c (`je` at 0x125a55,
  `call` at 0x125a82). Nonzero falls through to `_clock_gettime` candidates zero and
  `ret(_identFindIdentityNodeByUniqueID)+8` (`je` at 0x125a55, `call` at 0x125a5d). On that
  nonzero-result route, a zero clock-call result takes `je` to `_memcpy` candidates
  `ret(_identFindIdentityNodeByUniqueID)+16`, `[sp+4]` and 0x1c (`je` at 0x125a67, `call` at
  0x125a77). The malloc route reaches its second clock call for a nonzero `_malloc` result (`jne` at
  0x125a8e, `call` at 0x125ab6). A zero clock result takes `je` to another `_memcpy`, followed by
  `_lstAdd` candidates 0x1d8c78 and `ret(_malloc)` (`je` at 0x125ac0, `call` at 0x125ae7, `call` at
  0x125af5). The nonzero clock path instead includes `_free` with candidate `ret(_malloc)` (`je` at
  0x125ac0, `call` at 0x125ad0).

## `_identScanForConflicts`

- `_identScanForConflicts`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1257b0 to 0x125a40.
  Established. Calls `_lstFirst` with first candidate 0x1d8c78 before each of two outer regions
  (`call` at 0x1257d3, `call` at 0x1258bb). In the first, `_inet_lnaof` is followed by `jg`, taken
  when its signed 32-bit return exceeds 0xf (`call` at 0x1257ec, `jg` at 0x1257f7). The fall-through
  route includes an unresolved indirect call whose candidate arguments are unknown (`call` at
  0x125801). A later `_inet_lnaof` return at most 0xf takes `jle` past the following indirect
  call (`call` at 0x125834, `jle` at 0x12583f, `call` at 0x125856). The second region has two other
  indirect calls, also with unresolved targets (`call` at 0x1258e0, `call` at 0x125925). Their
  signed-flag decisions remain unknown (`jl` at 0x1258e7, `jge` at 0x12592c). No purpose or complete
  outcome for those callees is established.

## `_identFindMaster`

- `_identFindMaster`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1254d0 to 0x125560.
  Established. Invokes `_lstFirst` with first candidate 0x1d8c78 (`call` at 0x1254da). `jne` enters
  the later call region when that result is nonzero (`jne` at 0x1254e6). Zero falls through to an
  EAX clear (`jne` at 0x1254e6, `xor` at 0x1254e8). The body includes `_ffscCompareUniqueIDs` with
  candidates `ret(_lstNext)+16` and unknown, and `_lstNext` with unknown first candidate (`call` at
  0x125508, `call` at 0x125517). `jge` uses unresolved flags at the selection point (`jge` at
  0x125512). The backward `jne` repeats the region for a nonzero `_lstNext` result, while zero falls
  through (`jne` at 0x125523). The merged return value is unknown (`ret` at 0x125530).

## `_identFindConflictingRackID`

- `_identFindConflictingRackID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125450 to 0x1254d0.
  Established. Takes `je` to the EAX-clearing route for a zero `[sp+4]` (`je` at 0x12545b, `xor` at
  0x1254ba). For nonzero input, `jne` reaches `_lstFirst` only when `[[sp+4]+12] != 0xffffffff`
  (`mov` at 0x12545d, `jne` at 0x125463, `call` at 0x125485). Equality falls through to the zero
  route (`jne` at 0x125463, `xor` at 0x1254ba). The comparison call has unknown first candidate and
  `[sp+4]` second candidate (`call` at 0x125495); its zero result takes `je` past the following
  field comparison (`je` at 0x12549f). The later `je` and backward `jne` retain only `Z == 1` and `Z
  == 0` conditions, respectively (`je` at 0x1254a9, `jne` at 0x1254b8).

## `_identFindConflictingIPAddr`

- `_identFindConflictingIPAddr`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1253e0 to 0x125450.
  Established. A nonzero `[sp+4]` takes `jne` to the region that calls `_lstFirst` with 0x1d8c78
  (`jne` at 0x1253eb, `call` at 0x125405). Zero falls through to the EAX-clearing route (`jne` at
  0x1253eb, `xor` at 0x12543a). `_ffscCompareUniqueIDs` receives an unknown first candidate and
  `[sp+4]` second candidate (`call` at 0x125415); its zero result takes `je` past the two following
  loads and comparison (`je` at 0x12541f). The field read through `[sp+4] +0x8` is four bytes (`mov`
  at 0x125424). A later `je` uses `Z == 1`, while the loop's `jne` uses `Z == 0` (`je` at 0x125429,
  `jne` at 0x125438). Their compared operand values remain unresolved.

## `_identEnumerateRacks`

- `_identEnumerateRacks`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125200 to 0x1252b0.
  Established. Calls `_lstFirst` and `_lstNext`, with 0x1d8c78 as the former's first candidate and
  an unknown first candidate for the latter (`call` at 0x125213, `call` at 0x125237). Three
  four-byte indexed store sites have unresolved targets and values (`mov` at 0x125232, `mov` at 0x125283,
  `mov` at 0x125286). `jae` skips the latter two stores for `C == 0`, and falls through when that
  condition is false (`jae` at 0x12527b, `mov` at 0x125283, `mov` at 0x125286). The following `jl`
  returns to the load region when `[sp-16] < [sp-12]` signed, and falls through for the opposite
  relation (`jl` at 0x125294). Unsupported increments prevent a concrete index or count result
  (`inc` at 0x125235, `inc` at 0x12528d, `inc` at 0x125296).

## `_identEnumerateModules`

- `_identEnumerateModules`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125140 to 0x125200.
  Established. Begins its traversal region with `_lstFirst` candidate 0x1d8c78 (`call` at 0x125153).
  The indexed four-byte store has unknown target and value (`mov` at 0x12517c). A loop `jle` is
  taken for an unknown signed 32-bit value at most 1 and falls through for a greater value (`jle` at
  0x125184). Later `jae` bypasses two four-byte stores for `C == 0` (`jae` at 0x1251cb). Their
  effective addresses and stored values are unresolved (`mov` at 0x1251d3, `mov` at 0x1251d6). The
  following `jl` repeats the load region for `[sp-16] < [sp-12]` signed (`jl` at 0x1251e4). Several
  increment effects and the merged EAX result remain unknown (`inc` at 0x12517f, `inc` at 0x125180,
  `inc` at 0x1251dd, `inc` at 0x1251e6, `ret` at 0x1251f7).

## `_identFindByRackID`

- `_identFindByRackID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125370 to 0x1253c0.
  Established. Tests `[sp+4]` against 0xffffffff: inequality takes `jne` into the path calling
  `_lstFirst`, while equality falls through to the EAX-clearing route (`cmp` at 0x125377, `jne` at
  0x12537a, `call` at 0x125395, `xor` at 0x1253b6). `_lstFirst` has first candidate 0x1d8c78 (`call`
  at 0x125395). The later `_lstNext` first candidate is unknown (`call` at 0x1253a8). The equality
  branch into the alternate return path uses `Z == 1`, and the backward continuation uses `Z == 0`
  (`je` at 0x1253a5, `jne` at 0x1253b4). The merged EAX value is unresolved (`ret` at 0x1253be).

## `_identFindByIPAddr`

- `_identFindByIPAddr`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1252b0 to 0x125300.
  Established. Uses `_lstFirst` with candidate 0x1d8c78 and `_lstNext` with unknown first candidate
  (`call` at 0x1252bc, `call` at 0x1252e1). The four-byte input-side read is through `[sp+4] +0x0`
  (`mov` at 0x1252d3). `jne` at the comparison site takes its branch for `Z == 0` (`jne` at
  0x1252d7). Otherwise control falls through to the alternate exit path (`jne` at 0x1252d7). The
  later backward `jne` also depends on `Z == 0` (`jne` at 0x1252ed); its exit value is unknown
  (`ret` at 0x1252f7).

## `_identFindByUniqueID`

- `_identFindByUniqueID`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1253c0 to 0x1253e0.
  Established. Calls `_identFindIdentityNodeByUniqueID` with first candidate `[sp+4]` (`call` at
  0x1253c7). A zero result takes `je` past the address-forming instruction, while nonzero falls
  through to that instruction (`je` at 0x1253d0, `lea` at 0x1253d2). The final EAX value is
  unresolved after the paths merge (`ret` at 0x1253da). No offset or object type for the formed
  result is established here.

## `_identCleanOldEntries`

- `_identCleanOldEntries`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x125050 to 0x1250f0.
  Established. Calls `_lstFirst` with candidate 0x1d8c78, and skips the later region through `je`
  for a zero result (`call` at 0x12505b, `je` at 0x125067). `_ffscCompareTimestamps` has unknown
  first candidate and `[sp+4]` second candidate, followed by unresolved `jge` flags (`call` at
  0x12507f, `jge` at 0x125089). The later `_ffscCompareUniqueIDs` call has unknown first candidate
  and `[0x1f1cd0]` second candidate (`call` at 0x1250a7); its nonzero result takes `jne` past
  four-byte clears at 0x1f1cd0 and 0x1f50c4 (`jne` at 0x1250b1, `mov` at 0x1250b3, `mov` at
  0x1250bd). Zero falls through to those stores (`jne` at 0x1250b1, `mov` at 0x1250b3, `mov` at
  0x1250bd). The following calls include `_lstDelete` candidates 0x1d8c78 and unknown, then `_free`
  with unknown first candidate (`call` at 0x1250cd, `call` at 0x1250d6). The recorded return has EAX
  zero (`xor` at 0x1250e4, `ret` at 0x1250ef).
