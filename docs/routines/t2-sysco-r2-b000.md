# Routine role entries, sysco second reading list, recovered batch 0

Role statements recovered from the t2 batch branches for routines that main did not yet have,
matched by routine name, binary path and address. Each role is copied unchanged from the branch;
only the header line was rewritten into the canonical `routine` in `path` at 0xADDR form.

- `l1d_write_data` in `6.5.12/usr/sbin/flashsc` at 0x1001eca0
  `l1d_write_data` makes two `fprintf` calls, at 0x1001ecf8 and 0x1001ed74, with format strings at 0x100455b0 and 0x100455d0. On one path it reads `errno` at 0x1001ed6c. It makes an indirect call at 0x1001ed10 whose target the renderer could not name, and a direct call to an unnamed routine at 0x1001f280 with the constant 0x10047820 as its first argument. The write-data role comes only from the name, and the facts do not show what the indirect call does with its arguments.
  Grade: inferred.

- `IRouterSetQueueSuspendTime` in `6.5.12/usr/sbin/flashsc` at 0x10020cf0
  `IRouterSetQueueSuspendTime` writes two words to the absolute addresses 0x10047a08 and 0x10047a0c and returns its second argument. The facts show the two stores by address but not which argument they store, so the link to a suspend time comes from the name alone.
  Grade: inferred.

- `IRouterMakePacket` in `6.5.12/usr/sbin/flashsc` at 0x10020d30
  `IRouterMakePacket` reads a byte at offset 4 and a halfword at offset 6 of its first argument, writes the halfword back at offset 6, and calls an unnamed routine at 0x10020190. That call gets a local buffer at sp-80 and its entry arguments, and a branch at 0x10020db8 tests its result. The return value is either zero or unknown in the facts, and the fields and the callee's behaviour are not established, so the packet-building role is a guess from the name.
  Grade: inferred.
