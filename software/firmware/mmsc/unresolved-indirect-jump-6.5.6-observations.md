# MMSC unresolved indirect jump

This Unknown finding covers known local edges in one inferred symbol window.
Hardware role, types, pointee meanings, actual arity and runtime placement remain unknown.
The x86-32 a.out load is 0x108000. The next-symbol bound does not prove complete code-only extent.
Local reachability assumes ordinary call fallthrough. It does not prove runtime feasibility.
The unresolved transfer prevents complete access, call and return lists for all paths.
Call slots follow the assumed cdecl32 convention and preserve all six modeled candidates.
Only native resolved memory targets are named. An unknown target does not inherit its raw displacement.
Memory patterns group identical encoded instructions and native fields within this entry.
Counts are static records. They establish neither dynamic counts nor equal runtime pointees.
Only proved paired frame saves and terminal restores are omitted from the summary.
Incoming, outgoing, local and ambiguous accesses remain selected.
Known expressions remain literal. Only the value ? becomes unknown. A ? in a condition is an unknown operand.
Decoded records outside known edges remain in the full facts. Their effects are unlisted.
Those records are not established as dead, data or globally unreachable.
No destination, extra call or argument is recovered from raw words or unmodeled paths.

Resolved afterwards: the jump is a table dispatch on the console mode word, read in [console-modes.md](console-modes.md). The findings below are the earlier, table-less observations and are kept as recorded.

The [full facts](disasm/unresolved-indirect-jump-6.5.6-observations-facts.txt) preserve the full window and native warnings.

- `_csSysIn`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x11f8c8`.
  Inferred analysis window: 0x11f8c8 to 0x11f968 (exclusive), 160 bytes. Grade: unknown. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 829, body file offset 0x178e8, window SHA256 `e7b084f7fc1a5777a9c2689f93ae7cf61c8afd2149d7df995f0e42fb289176ac`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `ebp`. Count 1 static access. Sites: `mov` at 0x11f8cb.
  Four-byte load, pattern 2: target `[sp+4] +0x18`, raw base `edx`. Count 2 static accesses. Sites: `mov` at 0x11f8ce, `mov` at 0x11f948.
  Four-byte load, pattern 3: target `unknown`, raw base `unknown`. Count 1 static access. Sites: `jmp` at 0x11f8d7.
  Four-byte store, pattern 4: target `sp -0x8`, raw base `esp`, value `[[sp+4]+24]`. Count 1 static access. Sites: `push` at 0x11f94b.
  Four-byte store, pattern 5: target `sp -0xc`, raw base `esp`, value `[sp+4]`. Count 1 static access. Sites: `push` at 0x11f94c.
  Four-byte store, pattern 6: target `sp -0x10`, raw base `esp`, value `0x11f898`. Count 1 static access. Sites: `push` at 0x11f94d.
  Call (`call` at 0x11f952): _ffscMsg, target 0x12154c. Candidate slots: stack arg0=`0x11f898`, stack arg1=`[sp+4]`, stack arg2=`[[sp+4]+24]`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Branch (`ja` at 0x11f8d5): target 0x11f948. Taken when `? > 0x8 (unsigned, 32-bit)`. Falls through when `? <= 0x8 (unsigned, 32-bit)`.
  Unknown transfer (`jmp` at 0x11f8d7): target unknown. The native CFG supplies no resolved destination. Effects beyond it remain unknown.
  At the exit (`ret` at 0x11f95f), eax=`0xffffffff`.
  Register fact (`mov` at 0x11f957): eax=`0xffffffff`.
  Tool warning: `0x11f8d1 dec: unsupported effects invalidated`.
  Tool warning: `unresolved indirect transfers: access, call and return lists cover known edges only`.
  Decoded records outside known edges: 0x11f8de to 0x11f947, ending at 0x11f948 (exclusive). Count 76. Their effects are unlisted.
  Decoded records outside known edges: 0x11f960 to 0x11f967, ending at 0x11f968 (exclusive). Count 8. Their effects are unlisted.
