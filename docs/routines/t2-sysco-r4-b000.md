# Tier-2 routine role statements, sysco batch 0

These statements describe what each routine does, read from its rendered instruction facts (registers,
calls, constants and branch targets). They say nothing about hardware behaviour. Where the facts do not
settle a callee or a stored value, the statement says so.

- `sub_c00c1d74` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c1d74
  **Inferred.** Small frame routine that stores a halfword to 0xfa200930 (0xc00c1d8c) and reads it back (0xc00c1d90), then walks a table with `lwzx` loads at 0xc00c1da4 and 0xc00c1db4. It makes one indirect call with `blrl` at 0xc00c1dbc, and the `beq` at 0xc00c1dac skips that call when a loaded value is zero. It reads the word at 0xfa20094c (0xc00c1ddc) and returns an unnamed register at 0xc00c1e00. The indirect callee is not resolved in the facts, so the routine's purpose is only partly settled.

- `sub_c00c4918` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4918
  **Inferred.** Selector dispatch: the `bgt` at 0xc00c4954 sends a value above 0x13 down a separate path. The main path makes six calls to 0xc0010f08 (at 0xc00c496c, 0xc00c49ac, 0xc00c49cc, 0xc00c49f0, 0xc00c4a5c and 0xc00c4a84), two of them with constant addresses 0xc00dc188 and 0xc00dc1a4 and one with 0xc00dc228. It calls 0xc000712c twice with r3 = 0 (0xc00c4a20, 0xc00c4a44) and tests each result, stores a word to 0xfa200e54 (0xc00c4b1c), and reads 0xc00f9034 near the end (0xc00c4b68). The callees are not named, so the purpose is partly open.

- `sub_c00f7210` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00f7210
  **Inferred.** Long initialisation routine with a 0x60-byte frame. It writes words and halfwords into the block at 0xc010b16c to 0xc010b2e4 (stores from 0xc00f7288 to 0xc00f7428 and from 0xc00f7394 to 0xc00f7420) and sets the halfwords at 0xfa200950, 0xfa200952 and 0xfa200954 (0xc00f7480 to 0xc00f7488). It calls 0xc000a9d0 at four sites (0xc00f72d0, 0xc00f73fc, 0xc00f755c, 0xc00f7574) and calls 0xc00c1ebc and 0xc00c1ef0 with r3 = 0x20 and 0x80 (0xc00f7664 to 0xc00f7724). It then walks a 16-bit table with `lhz` and `sth` in counted loops (`bdnz` at 0xc00f75f8, 0xc00f76ac and 0xc00f775c). The callees are unnamed, so the meaning of the table is not settled.

- `sub_c00a5394` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5394
  **Inferred.** Setup routine that reads the word at 0xfa200e40 (0xc00a53ac) and calls 0xc00a5538 at 0xc00a53c4. It calls 0xc00a4c6c four times (0xc00a53e8, 0xc00a53fc, 0xc00a540c with r4 = 0xc012ac74, and 0xc00a5434). It calls 0xc000cfc0 at 0xc00a5414 inside a loop whose `beq` at 0xc00a5420 returns to 0xc00a5414, and it calls 0xc00a4ea0 at 0xc00a5440. The callees are unnamed, so the purpose beyond this call sequence is open.

- `sub_c00a5538` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5538
  **Inferred.** Copies halfword table entries. It writes the word at 0xfa200e40 (0xc00a5560) and reads it back (0xc00a5564), then reads halfwords at 0xc015b4dc and 0xc015b4de (0xc00a55c0 and 0xc00a55c8). It loads and stores halfwords through pointers in `bdnz` loops (0xc00a56a0 and 0xc00a56d0), and the loop is bounded by 0x31 (`ble` at 0xc00a55a4). It calls 0xc0004660 (0xc00a5590) and 0xc00102c8 with r3 = 0xc0106824 (0xc00a55b8). The table contents are not shown in the facts.

- `sub_c00a575c` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a575c
  **Inferred.** Sibling of the routine at 0xc00a5538 in the same image. It reads the word at 0xfa200e40 (0xc00a5774), has a loop bounded by 0x31 (`ble` at 0xc00a57c8), and ends with a compare against 0x32 (`bne` at 0xc00a57d0). It calls 0xc0004660 (0xc00a57b4) and 0xc00102c8 with r3 = 0xc01068cc (0xc00a57dc). It has no halfword copy and no call to 0xc00a5538, and the data it reads is not named in the facts.

- `sub_c01393ac` in `6.5.13/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc01393ac
  **Inferred.** Large initialisation routine with a 0x60-byte frame, close in shape to sub_c00f7210 in 6.5.12. It writes words and halfwords into the block at 0xc0180584 to 0xc01806fc, sets the same flag halfwords at 0xfa200950, 0xfa200952 and 0xfa200954 (0xc0139600 to 0xc0139608), and reads the word at 0xc015b4e4 as a base pointer (0xc01395ec, with `lwz` at 0xc013960c through +0xfc). It calls 0xc001348c with r3 = 3 (0xc01393c8), 0xc000c860 at four sites, 0xc006a838 at two sites, 0xc000fd1c at two sites and 0xc0026eb0 with r3 = 0x90 (0xc0139780). It calls 0xc00a1a88 and 0xc00a1abc with sizes 0x20 and 0x80 (0xc0139808 to 0xc01398c8). The callee names are not in the facts, so the purpose is partly open.

- `sub_c000a5e0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc000a5e0
  **Inferred.** Small wrapper with a 0x20-byte frame. It reads the word at 0xfa200284 (0xc000a5f4), stores a word through a pointer (0xc000a600), and reads and writes a byte through a pointer (`lbz` at 0xc000a60c, `stb` at 0xc000a614). It calls 0xc0004710 at 0xc000a5ec and 0xc0010640 with r3 = 0xc00effd8 at 0xc000a61c. The value read from 0xfa200284 is not shown being used, so its role is open.

- `sub_c00a4e98` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a4e98
  **Inferred.** Table walk with a 0x30-byte frame. It loads halfwords and words through pointers in loops (`lhz` at 0xc00a4eb8, 0xc00a4ee0 and 0xc00a4ef8) and reads the word at 0xc015e178 (0xc00a4f58). It stores a word to 0xfa200e54 (0xc00a4f6c), the same store the 6.5.12 routine at 0xc00c4918 makes. It calls 0xc0010640 with r3 = 0xc0109480 (0xc00a4ecc), 0xc0004868 (0xc00a4f38), 0xc00046d4 and then 0xc0004710 on its result (0xc00a4f40, 0xc00a4f44), and 0xc00046e0 at 0xc00a4fc0. The callees are unnamed.

- `sub_c00a5eb0` in `6.5.30/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00a5eb0
  **Inferred.** Initialiser with a 0x20-byte frame. It loads the word at 0xc012f278 (0xc00a5ec8) and stores words to 0xfa200e40, 0xfa200e48, 0xfa200e44 and 0xfa200e4c (0xc00a5eec to 0xc00a5f0c). It reads halfwords at 0xc012cac8 and 0xc012caca (0xc00a5f14 and 0xc00a5f1c) and then copies bytes and halfwords through pointers in a loop closed by the `ble` at 0xc00a5ffc. It calls 0xc00046a0 (0xc00a5ef0), 0xc00aa0a0 (0xc00a5fec), 0xc0004c2c with r9 from 0xc015e518 (0xc00a6094) and 0xc00a56e0 with r3 = 0x600a0000 (0xc00a60c0). The word at 0xc015e518 is tested at 0xc00a6064. The callees are unnamed, so the purpose is partly open.

- `sub_fe004ef0` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe004ef0
  **Inferred.** Start-up sequence with a 0x20-byte frame. It calls 0xfe0051e4 with r3 = 0xe01810 (0xfe004f18), then calls 0xfe004604 six times with constant addresses from 0xe013fc to 0xe01484 (0xfe004f24 to 0xfe004f80). That pattern suggests it prints fixed messages, but the facts do not show the strings. It calls 0xfe004fe8 (0xfe004f2c) and branches on its result at 0xfe004f34, calls 0xfe004f8c at 0xfe004f48, stores a word to 0xfa200004 (0xfe004f78), and ends by calling 0xfe0046bc on the value 0xfe004604 returned.

- `sub_fe005194` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe005194
  **Inferred.** Leaf routine with no calls. It stores halfwords to 0xfa200950, 0xfa200952, 0xfa200954, 0xfa200960, 0xfa200962, 0xfa200964, 0xfa200966 and 0xfa200ac2 (0xfe00519c to 0xfe0051dc), stores words to 0xfa200abc, 0xfa200ab8 and 0xfa200ac4 (0xfe0051b0 to 0xfe0051c4), and returns r3 at 0xfe0051e0. The stored values are not shown in the facts, so only the store addresses are settled.

- `sub_fe048c30` in `6.5.9/usr/cpu/firmware/l2.bin(flash)` at 0xfe048c30
  **Inferred.** Sets up a 0x20-byte frame and tests the word at 0xe05b18 (`lwz` at 0xfe048c50, `bne` at 0xfe048c68). When that word is zero it calls 0xfe048b88 with r9 = 0xe00000 (0xfe048c6c), and when it is nonzero the call is skipped. The rest reads halfwords at 0xfa203c82 and 0xfa203c80 (0xfe048c78 and 0xfe048c94) and writes bytes to 0xfa200860, 0xfa200870 and 0xfa20086c (0xfe048ccc to 0xfe048cdc), with `bdnz` loops over halfword and word tables (0xfe048d1c and 0xfe048d94). The loop data is not named in the facts.

- `sub_c00c1d48` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c1d48
  **Inferred.** Initialiser with a 0x20-byte frame. It stores halfwords to 0xfa200952, 0xfa200950 and 0xfa200954 (0xc00c1d74 to 0xc00c1d80), the same flag set the 6.5.12 routine at 0xc00f7210 writes. It walks a table through the pointer loaded from 0xc00f6280 (0xc00c1d60, with the +0xfc load at 0xc00c1d84) using `lhz` and `sth` in loops, and calls 0xc0010f08 once at 0xc00c1d68 with r3 = 0xc00d9264 and r9 = 0xc00f0000. The table contents and the callee's purpose are not shown.

- `sub_c00c222c` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c222c
  **Inferred.** Dispatch-style routine: the `bgt` at 0xc00c2268 sends a selector above 0x13 down a separate path. The main path calls 0xc0010f08 at 0xc00c2280, 0xc00c22c0, 0xc00c22e0, 0xc00c2304, 0xc00c2370 and 0xc00c2398, with constant addresses 0xc00d9344, 0xc00d9360 and 0xc00d93e4 in r3 for three of them. It calls 0xc000712c twice with r3 = 0 (0xc00c2334 and 0xc00c2358) and tests each result. It ends with clean-up calls to 0xc00027b0 (0xc00c2400), 0xc0006930 (0xc00c2408) and 0xc000697c (0xc00c248c), and tests the word at 0xc00f6034 at 0xc00c2484. The callees are not named.

- `sub_c00c3094` in `6.5.9/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c3094
  **Inferred.** Bring-up routine. It calls 0xc0006acc (0xc00c31b8) and branches on its result at 0xc00c31c0, calls 0xc0066ce0 (0xc00c31c8), and calls 0xc0006cd4 with r9 = 0x14 and r10 = 8 (0xc00c3278). It calls 0xc00c2c4c at 0xc00c3298 and 0xc00c2994 at 0xc00c32b8 with r3 = 0x607e0000 and r4 = 0xc00c2a68. The word at 0xc00f6278 is tested at 0xc00c3240. The callees are unnamed, so the purpose beyond this call sequence is open.

- `sub_ffec7534` in `cd-ist-3.24/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffec7534
  **Inferred.** Flag-byte helper on the ColdFire L1 image. It sets a bit in the byte at 0x200007f8 (`bset.b` at 0xffec7546), reads bytes at 0x200007f8 and 0x200007ff, and clears a bit with `bclr.b` at 0xffec7590. It compares a value against 2, 3 and 4 (`beq.b` at 0xffec7628 and 0xffec762e, `bne.b` at 0xffec7634) to choose a path, and returns d0 = 0 at 0xffec7666. The meaning of the flag bits is not shown in the facts.

- `sub_ffebf300` in `6.5.10/usr/cpu/firmware/l1.bin(l1#0)` at 0xffebf300
  **Inferred.** Argument range check followed by flag-byte updates. Six stack words at sp+4 through sp+24 are compared with 0x9c4, 0xc, 0x1f, 0x18, 0x3c and 0x3d (`bgt.b` at 0xffebf322 through `bge.b` at 0xffebf344), and the failing paths converge on 0xffebf346. After the checks it sets, clears and reads the bytes at 0x200007f8 through 0x200007fc, and adds a word from the table at 0x300005a0 (`add.l` at 0xffebf52c). The return at 0xffebf5dc gives d0 from an unnamed register, so the result is open.

- `sub_c00c16e4` in `6.5.10/usr/cpu/firmware/l2.bin(kernel)` at 0xc00c16e4
  **Inferred.** Small routine that reads a table entry through a pointer (`lwzx` at 0xc00c1704), stores to two indexed words (`stwx` at 0xc00c172c and 0xc00c1730), and reads the word at 0xfa200948 (0xc00c1738). It calls 0xc0010f08 at 0xc00c1724 with r3 = 0xc00db4c0, and the `beq` at 0xc00c1714 skips that call when the tested value is zero. The return at 0xc00c175c gives r3 from an unnamed register, so the table and the callee are not settled.

- `sub_ffe5a210` in `6.5.12/usr/cpu/firmware/sysco/l1.bin(l1#0)` at 0xffe5a210
  **Inferred.** Same shape as sub_ffebf300 in 6.5.10. It range-checks the stack arguments against 0x9c4, 0x78a, 0xc, 0x1f, 0x18, 0x3c and 0x3d (`bgt.b` at 0xffe5a232 through `bge.b` at 0xffe5a260, with `ble.b` at 0xffe5a236 and `blt.b` at 0xffe5a23e), and the failing paths converge on 0xffe5a262. It then sets, clears and reads the flag bytes at 0x200007f8 through 0x200007fc, and adds a word from the table at 0x30000274 (`add.l` at 0xffe5a448). The return at 0xffe5a4f8 gives d0 from an unnamed register, so the result is open.

- `sub_c00c4fec` in `6.5.12/usr/cpu/firmware/sysco/l2.bin(kernel)` at 0xc00c4fec
  **Inferred.** Calls through a function-pointer table. It loads a pointer at 0xc00f9288 and branches on it (`bne` at 0xc00c5018), tests the words at offset +4 and +8 of that pointer (`beq` at 0xc00c5034 and 0xc00c5060), and calls 0xc0010f08 with r3 = 0xc00dc2e4 (0xc00c5024). It then makes an indirect call with `blrl` at 0xc00c5040, using r3 and r9 loaded from 0xfa200e80. The callee is not resolved in the facts.

- `L1DirectResponse` in `6.5.12/usr/sbin/flashsc` at 0x1001d980
  **Inferred.** Builds a packet and sends it. It calls IRouterMakePacket at 0x1001d9f8 (address 0x10020d30) with a2 read from offset 0x22 of its argument and a3 = 1, then calls L1DirectSendPacket at 0x1001daa4 with the packet returned. The `bne` at 0x1001da0c jumps past the -0x194 return at 0x1001da24 when IRouterMakePacket returns nonzero. The return at 0x1001dabc passes back the result of L1DirectSendPacket. The `beq` at 0x1001d9b4 branches to 0x1001d9cc, ahead of the build call, when a0 is zero, and the facts do not show what the packet carries.

- `l1d_open` in `6.5.12/usr/sbin/flashsc` at 0x1001dff0
  **Inferred.** Opens the L1 link. It picks a serial or Ethernet opener by the value at 0x1004752c: `l1d_open_serial_direct` at 0x1001e050 or `l1d_open_serial_enet` at 0x1001e06c. It sets up a PPP channel with PPPInit at 0x1001e118, passes the handle at 0x10047538 to `read`, `SocketRead` and `EventAddInput`, calls SocketNodelayMode with arguments 1 and 0x18 at 0x1001e334, and registers l1d_read_packet with EventAddInput at 0x1001e3b4. Failure paths print through fprintf and return -1 (for example at 0x1001e0b4 and 0x1001e184), and the normal path returns 0 at 0x1001e3f4. The serial and Ethernet paths are in callees not shown here.

- `l1d_close` in `6.5.12/usr/sbin/flashsc` at 0x1001e400
  **Inferred.** Close counterpart of l1d_open. It removes the input handler with EventRemoveInput at 0x1001e42c, using the handle at 0x10047538, then closes the serial or Ethernet link through `l1d_close_serial_direct` at 0x1001e464 or `l1d_close_serial_enet` at 0x1001e480, chosen by the value at 0x1004752c (`beq` at 0x1001e458). An unnamed callee is passed l1d_pkt_out at 0x1001e444. It returns 0 at 0x1001e4e0 on the normal path.

- `l1d_tty_debug` in `6.5.12/usr/sbin/flashsc` at 0x1001e4f0
  **Inferred.** Debug printer for the tty path. It calls the routine at 0x1001d2c0 at 0x1001e51c with the entry arguments and a stack buffer at sp-96, and when that result is -1 it prints an error with fprintf at 0x1001e54c (the `bne` at 0x1001e528 skips this when the result is not -1). Further fprintf calls at 0x1001e580, 0x1001e5a0, 0x1001e5c0, 0x1001e5e0 and 0x1001e624 print fixed messages, and a bound of 8 on a1 is checked at 0x1001e640. The return values are the results of fprintf at 0x1001e560 and 0x1001e654. The purpose of the 0x1001d2c0 callee is not shown.
