# ARM Linux 2.6 packet, flash and router routines in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM. Its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Call lines include every reported call instruction and every printed register slot.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Role: unknown. Pointee meanings remain unknown. Selected access and branch facts below cover known control-flow edges. The
full fact sheets retain all reported accesses and implicit stack operations. Separate loads at the same printed address need not observe the same value.
Computed transfers with unresolved edges are identified under their routines. Effects on omitted paths remain unknown.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-packet-flash-router-facts.txt) retain all tool sections.

The notation `ret(ADDR)` identifies a call-result expression without distinguishing
individual call sites to that target. Each return line records machine-register
state from the analysis rather than a C return type.


Groups summarize static access records by printed shape. Instruction-address order can include different control-flow paths.

## `syscom_process_inbound_request`

- `syscom_process_inbound_request`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9050 to 0xb94e8.
  Established facts:
  Decoded words in [0xb94d8, 0xb94e8) are not reached from the entry by known edges.
  Call (`bl` at 0xb908c): (unknown callee), target 0xb5c90, r0=`r0`, r1=`r1`, r2=`0x2`, r3=`0x0`.
  Call (`bl` at 0xb90b4): (unknown callee), target 0xb5db0, r0=`[sp-28]`, r1=`[sp-32]`, r2=`0x16d578`, r3=`[sp-36]`.
  Call (`bl` at 0xb9164): (unknown callee), target 0xbb844, r0=`[sp-28]`, r1=`[sp-32]`, r2=`[sp-36]`, r3=`[sp-40]`.
  Call (`bl` at 0xb91a4): (unknown callee), target 0x958d4, r0=`sp-132`, r1=`sp-128`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb9220): (unknown callee), target 0xb80fc, r0=`[sp-32]`, r1=`[sp-148]`, r2=`sp-24`, r3=`unknown`.
  Grouped accesses: 2 reported word load records with absolute target `0x1b0cb8`, including target `abs 0x1b0cb8` (`ldr` at 0xb923c), target `abs 0x1b0cb8` (`ldr` at 0xb9254).
  Branch (`beq` at 0xb9244): taken to 0xb9270 when `[0x1b0cb8] == 0x0`, falls through to 0xb9248 when `[0x1b0cb8] != 0x0`.
  Call (`mov` at 0xb926c): (unknown callee), target (unknown target), r0=`[sp-28]`, r1=`[sp-32]`, r2=`[sp-148]`, r3=`[sp-36]`.
  Branch (`bls` at 0xb9278): taken to 0xb9290 when `[sp-40] <= 0x7 (unsigned, 32-bit)`, falls through to 0xb927c when `[sp-40] > 0x7 (unsigned, 32-bit)`.
  Call (`bl` at 0xb9340): (unknown callee), target 0x95934, r0=`sp-132`, r1=`sp-128`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb9358): (unknown callee), target 0x2031c, r0=`[sp-36]`, r1=`unknown`, r2=`[[sp-36]]:1`, r3=`[[sp-36]]:1`.
  Call (`bl` at 0xb9388): (unknown callee), target 0x95b5c, r0=`sp-132`, r1=`sp-128`, r2=`[sp-36]+1`, r3=`ret(0x2031c)+1`.
  Call (`bl` at 0xb9390): (unknown callee), target 0x2031c, r0=`[sp-36]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb93d4): (unknown callee), target 0x959dc, r0=`sp-132`, r1=`sp-128`, r2=`[sp-36]+1`, r3=`unknown`.
  Call (`bl` at 0xb9430): (unknown callee), target 0xfcf98, r0=`sp-48`, r1=`sp-140`, r2=`0x0`, r3=`[sp-28]`.
  Call (`bl` at 0xb9458): (unknown callee), target 0x6e1b0, r0=`[sp-28]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xb9464): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb94ac): (unknown callee), target 0x203f4, r0=`[sp-164]`, r1=`[sp-156]`, r2=`[sp-160]`, r3=`unknown`.
  Call (`bl` at 0xb94b8): (unknown callee), target 0x219a4, r0=`[sp-136]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb94cc): (unknown callee), target 0x29908, r0=`0x16d588`, r1=`ret(0x203f4)`, r2=`[sp-32]`, r3=`ret(0x219a4)`.
  Recorded r0 = `unknown` (`ldm` at 0xb94d4).

## `syscom_process_inbound_event`

- `syscom_process_inbound_event`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb94e8 to 0xb99fc.
  Established facts:
  Decoded words in [0xb99e0, 0xb99fc) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with absolute target `0x31841c`, including target `abs 0x31841c` (`ldr` at 0xb950c), target `abs 0x31841c` (`ldr` at 0xb9518).
  Branch (`bne` at 0xb9520): taken to 0xb952c when `[0x31841c] != 0x0`, falls through to 0xb9524 when `[0x31841c] == 0x0`.
  Call (`bl` at 0xb9590): (unknown callee), target 0x203f4, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb95f0): (unknown callee), target 0x28230, r0=`0xe`, r1=`unknown`, r2=`unknown`, r3=`[[0x1b0ce0]]`.
  Call (`bl` at 0xb9614): (unknown callee), target 0x958d4, r0=`sp-84`, r1=`sp-80`, r2=`unknown`, r3=`sp-84`.
  Call (`bl` at 0xb9644): (unknown callee), target 0x95b5c, r0=`sp-84`, r1=`sp-80`, r2=`[sp-36]+1`, r3=`[sp-40]-1`.
  Call (`bl` at 0xb9654): (unknown callee), target 0xb5c90, r0=`[sp-28]`, r1=`[sp-32]`, r2=`0x4`, r3=`unknown`.
  Call (`bl` at 0xb967c): (unknown callee), target 0xb5db0, r0=`[sp-28]`, r1=`[sp-32]`, r2=`0x16d5b8`, r3=`[sp-36]`.
  Call (`bl` at 0xb9690): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb96a4): (unknown callee), target 0x3e458, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0xb96b8): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb96cc): (unknown callee), target 0x3e498, r0=`[sp-32]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0xb96fc): (unknown callee), target 0x818a0, r0=`[sp-136]`, r1=`unknown`, r2=`[sp-100]`, r3=`unknown`.
  Call (`bl` at 0xb9708): (unknown callee), target 0xb80fc, r0=`[sp-32]`, r1=`[sp-100]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb972c): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`[sp-108]`.
  Call (`bl` at 0xb9740): (unknown callee), target 0x3e458, r0=`[sp-88]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0xb9754): (unknown callee), target 0x3e38c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb9768): (unknown callee), target 0x3e498, r0=`[sp-88]`, r1=`unknown`, r2=`unknown`, r3=`ret(0x3e38c)`.
  Call (`bl` at 0xb9794): (unknown callee), target 0x81b18, r0=`[sp-144]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb97ac): (unknown callee), target 0xb80fc, r0=`[sp-88]`, r1=`0x4`, r2=`unknown`, r3=`ret(0x81b18)`.
  Call (`bl` at 0xb97d0): (unknown callee), target 0xb7e54, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb97dc): (unknown callee), target 0xb7f6c, r0=`[sp-96]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb97f0): (unknown callee), target 0x942d4, r0=`[sp-28]`, r1=`ret(0xb7e54)`, r2=`ret(0xb7f6c)`, r3=`ret(0xb7f6c)`.
  Call (`bl` at 0xb985c): (unknown callee), target 0xb80fc, r0=`[sp-32]`, r1=`[sp-100]`, r2=`sp-24`, r3=`unknown`.
  Call (`mov` at 0xb98d8): (unknown callee), target (unknown target), r0=`[sp-156]`, r1=`[sp-36]+1`, r2=`[sp-40]-1`, r3=`[sp-40]-1`.
  Call (`bl` at 0xb98e0): (unknown callee), target 0x28230, r0=`0x81`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Word load: target `abs 0x31b5bc` (`ldr` at 0xb9904).
  Call (`bl` at 0xb991c): (unknown callee), target 0xfd284, r0=`sp-48`, r1=`[sp-28]`, r2=`0x0`, r3=`[sp-84]`.
  Call (`bl` at 0xb9944): (unknown callee), target 0x6e1b0, r0=`[sp-28]`, r1=`0x1`, r2=`0x0`, r3=`0x0`.
  Call (`bl` at 0xb9950): (unknown callee), target 0x2031c, r0=`ret(0x6e1b0)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb9998): (unknown callee), target 0x203f4, r0=`[sp-124]`, r1=`[sp-112]`, r2=`[sp-120]`, r3=`unknown`.
  Call (`bl` at 0xb99a4): (unknown callee), target 0x219a4, r0=`[sp-92]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xb99b8): (unknown callee), target 0x29908, r0=`0x16d5c8`, r1=`ret(0x203f4)`, r2=`[sp-32]`, r3=`ret(0x219a4)`.
  Call (`bl` at 0xb99d4): (unknown callee), target 0xb99fc, r0=`[0x31b61c]`, r1=`unknown`, r2=`unknown`, r3=`0x31b5c0`.
  Recorded r0 = `unknown` (`ldm` at 0xb99dc).

## `syscom_process_outbound_request`

- `syscom_process_outbound_request`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9fa8 to 0xba514.
  Established facts:
  Decoded words in [0xba4f0, 0xba504), [0xba508, 0xba514) are not reached from the entry by known edges.
  Grouped accesses: 2 reported word load records with printed base expression `r0` and nonnegative word-aligned offsets, including target `r0 +0x18` (`ldr` at 0xb9fc8), target `r0 +0x18` (`ldr` at 0xb9fd8).
  Branch (`blt` at 0xb9fd0): taken to 0xb9ff4 when `[r0+24] < 0x0 (signed, 32-bit)`, falls through to 0xb9fd4 when `[r0+24] >= 0x0 (signed, 32-bit)`.
  Call (`bl` at 0xba020): (unknown callee), target 0xb7e54, r0=`[sp-148]`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0xba030): (unknown callee), target 0xb7f6c, r0=`[sp-144]`, r1=`unknown`, r2=`unknown`, r3=`ret(0xb7e54)`.
  Call (`bl` at 0xba040): (unknown callee), target 0x558d8, r0=`[sp-136]`, r1=`unknown`, r2=`unknown`, r3=`ret(0xb7f6c)`.
  Call (`bl` at 0xba050): (unknown callee), target 0x6dc0c, r0=`ret(0x558d8)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x558d8)`.
  Call (`bl` at 0xba078): (unknown callee), target 0x958d4, r0=`sp-120`, r1=`sp-116`, r2=`unknown`, r3=`sp-120`.
  Call (`bl` at 0xba084): (unknown callee), target 0x95dd0, r0=`[[sp-20]+24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0xba098): (unknown callee), target 0x205bc, r0=`sp-100`, r1=`0x16d634`, r2=`ret(0x95dd0)`, r3=`sp-100`.
  Call (`bl` at 0xba0a4): (unknown callee), target 0x2031c, r0=`sp-100`, r1=`unknown`, r2=`unknown`, r3=`sp-100`.
  Call (`bl` at 0xba0d0): (unknown callee), target 0x95b5c, r0=`sp-120`, r1=`sp-116`, r2=`sp-100`, r3=`ret(0x2031c)`.
  Call (`bl` at 0xba0ec): (unknown callee), target 0xfd0dc, r0=`0x31b5a0`, r1=`0x0`, r2=`[sp-20]`, r3=`[sp-120]`.
  Grouped accesses: 4 reported word load records with absolute target `0x31b5b8`, including target `abs 0x31b5b8` (`ldr` at 0xba100), target `abs 0x31b5b8` (`ldr` at 0xba1d0), target `abs 0x31b5b8` (`ldr` at 0xba31c).
  Call (`bl` at 0xba16c): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xba194): (unknown callee), target 0xb5c90, r0=`[sp-136]`, r1=`[sp-140]`, r2=`0x8`, r3=`0x0`.
  Call (`bl` at 0xba1c0): (unknown callee), target 0xb5db0, r0=`[sp-136]`, r1=`[sp-140]`, r2=`0x16d658`, r3=`sp-124`.
  Call (`mov` at 0xba22c): (unknown callee), target (unknown target), r0=`unknown`, r1=`sp-124`, r2=`0x1`, r3=`unknown`.
  Call (`bl` at 0xba284): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xba298): (unknown callee), target 0x6dc0c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xba2e0): (unknown callee), target 0xb5c90, r0=`[sp-136]`, r1=`[sp-140]`, r2=`0x10`, r3=`unknown`.
  Call (`bl` at 0xba30c): (unknown callee), target 0xb5db0, r0=`[sp-136]`, r1=`[sp-140]`, r2=`0x16d668`, r3=`sp-124`.
  Call (`bl` at 0xba3bc): (unknown callee), target 0x1003b8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`sp-124`.
  Call (`bl` at 0xba3f4): (unknown callee), target 0x958d4, r0=`sp-120`, r1=`sp-116`, r2=`unknown`, r3=`sp-120`.
  Call (`bl` at 0xba40c): (unknown callee), target 0x95f98, r0=`[[sp-20]+24]`, r1=`unknown`, r2=`unknown`, r3=`[sp-20]`.
  Call (`bl` at 0xba420): (unknown callee), target 0x205bc, r0=`sp-100`, r1=`0x16d67c`, r2=`ret(0x95f98)`, r3=`sp-100`.
  Call (`bl` at 0xba440): (unknown callee), target 0x203c4, r0=`sp-100`, r1=`0x16d690`, r2=`[sp-132]`, r3=`sp-100`.
  Call (`bl` at 0xba454): (unknown callee), target 0x203c4, r0=`sp-100`, r1=`0x16d6a8`, r2=`[sp-132]`, r3=`sp-100`.
  Call (`bl` at 0xba46c): (unknown callee), target 0x219a4, r0=`[sp-132]`, r1=`unknown`, r2=`[sp-132]`, r3=`0xfffff69e`.
  Call (`bl` at 0xba480): (unknown callee), target 0x203c4, r0=`sp-100`, r1=`ret(0x219a4)`, r2=`ret(0x219a4)`, r3=`sp-100`.
  Call (`bl` at 0xba490): (unknown callee), target 0x203c4, r0=`sp-100`, r1=`0x16d6b4`, r2=`unknown`, r3=`sp-100`.
  Call (`bl` at 0xba49c): (unknown callee), target 0x2031c, r0=`sp-100`, r1=`unknown`, r2=`unknown`, r3=`sp-100`.
  Call (`bl` at 0xba4c8): (unknown callee), target 0x95b5c, r0=`sp-120`, r1=`sp-116`, r2=`sp-100`, r3=`ret(0x2031c)`.
  Call (`bl` at 0xba4e4): (unknown callee), target 0xfd0dc, r0=`0x31b5a0`, r1=`0x0`, r2=`[sp-20]`, r3=`[sp-120]`.
  Recorded r0 = `ret(0xfd0dc)` (`ldm` at 0xba4ec, `bl` at 0xba0ec, `bl` at 0xba4e4).

## `syscom_process_outbound_event`

- `syscom_process_outbound_event`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba514 to 0xbacf8.
  Established facts:
  Decoded words in [0xbacf0, 0xbacf8) are not reached from the entry by known edges.
  Grouped accesses: 2 reported byte load records with printed base expression `r0` and nonnegative byte-aligned offsets, including target `r0 +0x20` (`ldrb` at 0xba534), target `r0 +0x3c` (`ldrb` at 0xba544).
  Branch (`bne` at 0xba53c): taken to 0xba5a4 when `[r0+32]:1 != 0x4`, falls through to 0xba540 when `[r0+32]:1 == 0x4`.
  Call (`bl` at 0xba56c): (unknown callee), target 0xfde7c, r0=`r0`, r1=`0x3`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0xba58c): (unknown callee), target 0x28230, r0=`0xe`, r1=`unknown`, r2=`unknown`, r3=`[sp-188]`.
  Grouped accesses: 4 reported word load records with absolute target `0x31b5a8`, including target `abs 0x31b5a8` (`ldr` at 0xba5a8), target `abs 0x31b5a8` (`ldr` at 0xba5bc), target `abs 0x31b5a8` (`ldr` at 0xba5cc).
  Call (`bl` at 0xba5dc): (unknown callee), target 0xb7e54, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xba5fc): (unknown callee), target 0xb7f6c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xba6f8): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xba864): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`[sp-160]`, r2=`[sp-20]`, r3=`unknown`.
  Call (`bl` at 0xba9fc): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`[sp-160]`, r2=`[sp-20]`, r3=`unknown`.
  Call (`bl` at 0xbaa1c): (unknown callee), target 0x203f4, r0=`(sp-148 + [sp-152])`, r1=`unknown`, r2=`[sp-164]`, r3=`(sp-148 + [sp-152])`.
  Call (`bl` at 0xbaaf4): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`[sp-160]`, r2=`[sp-20]`, r3=`unknown`.
  Call (`bl` at 0xbab10): (unknown callee), target 0x20634, r0=`(sp-148 + [sp-152])`, r1=`unknown`, r2=`sp-148`, r3=`(sp-148 + [sp-152])`.
  Call (`bl` at 0xbab4c): (unknown callee), target 0xb5c90, r0=`[sp-176]`, r1=`[sp-180]`, r2=`0x20`, r3=`unknown`.
  Call (`bl` at 0xbab78): (unknown callee), target 0xb5db0, r0=`[sp-176]`, r1=`[sp-180]`, r2=`0x16d6b8`, r3=`sp-148`.
  Call (`mov` at 0xbac30): (unknown callee), target (unknown target), r0=`unknown`, r1=`sp-148`, r2=`[sp-152]`, r3=`unknown`.
  Call (`bl` at 0xbace4): (unknown callee), target 0x1003b8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`sp-148`.
  Recorded r0 = `unknown` (`ldm` at 0xbacec).

## `syscom_process_outbound_response`

- `syscom_process_outbound_response`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbacf8 to 0xbb4f8.
  Established facts:
  Decoded words in [0xbb4ec, 0xbb4f8) are not reached from the entry by known edges.
  Grouped accesses: 8 reported byte load records with printed base expression `[sp-20]` and nonnegative byte-aligned offsets, including target `[sp-20] +0x17` (`ldrb` at 0xbad60), target `[sp-20] +0x13` (`ldrb` at 0xbae0c), target `[sp-20] +0x24` (`ldrb` at 0xbae20).
  Call (`bl` at 0xbad78): (unknown callee), target 0xb7e54, r0=`[sp-188]`, r1=`r1`, r2=`r2`, r3=`unknown`.
  Call (`bl` at 0xbad88): (unknown callee), target 0xb7f6c, r0=`[sp-184]`, r1=`unknown`, r2=`unknown`, r3=`ret(0xb7e54)`.
  Call (`bl` at 0xbadf4): (unknown callee), target 0xbc80c, r0=`[sp-176]`, r1=`[sp-180]`, r2=`[sp-20]`, r3=`[sp-192]`.
  Branch (`beq` at 0xbae14): taken to 0xbae1c when `[[sp-20]+19]:1 == 0x0`, falls through to 0xbae18 when `[[sp-20]+19]:1 != 0x0`.
  Call (`bl` at 0xbae48): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Branch (`bls` at 0xbaf94): taken to 0xbafa4 when `[sp-152]+5 <= 0x7f (unsigned, 32-bit)`, falls through to 0xbaf98 when `[sp-152]+5 > 0x7f (unsigned, 32-bit)`.
  Call (`bl` at 0xbb024): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`[sp-160]`, r2=`[sp-20]`, r3=`unknown`.
  Call (`bl` at 0xbb1bc): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`[sp-160]`, r2=`[sp-20]`, r3=`unknown`.
  Call (`bl` at 0xbb1dc): (unknown callee), target 0x203f4, r0=`(sp-148 + [sp-152])`, r1=`unknown`, r2=`[sp-164]`, r3=`(sp-148 + [sp-152])`.
  Call (`bl` at 0xbb2b4): (unknown callee), target 0xfde7c, r0=`[sp-20]`, r1=`[sp-160]`, r2=`[sp-20]`, r3=`unknown`.
  Call (`bl` at 0xbb2d0): (unknown callee), target 0x20634, r0=`(sp-148 + [sp-152])`, r1=`unknown`, r2=`sp-148`, r3=`(sp-148 + [sp-152])`.
  Call (`bl` at 0xbb30c): (unknown callee), target 0xb5c90, r0=`[sp-176]`, r1=`[sp-180]`, r2=`0x40`, r3=`unknown`.
  Call (`bl` at 0xbb338): (unknown callee), target 0xb5db0, r0=`[sp-176]`, r1=`[sp-180]`, r2=`0x16d6c8`, r3=`sp-148`.
  Call (`mov` at 0xbb374): (unknown callee), target (unknown target), r0=`[sp-176]`, r1=`[sp-180]`, r2=`[sp-184]`, r3=`sp-148`.
  Call (`mov` at 0xbb42c): (unknown callee), target (unknown target), r0=`unknown`, r1=`sp-148`, r2=`[sp-152]`, r3=`unknown`.
  Call (`bl` at 0xbb4e0): (unknown callee), target 0x1003b8, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`sp-148`.
  Recorded r0 = `unknown` (`ldm` at 0xbb4e8).

## `SysComSimStart`

- `SysComSimStart`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xef09c to 0xef180.
  Established facts:
  Decoded words in [0xef160, 0xef180) are not reached from the entry by known edges.
  Word load: target `abs 0x1b0f60` (`ldr` at 0xef0b0).
  Branch (`bne` at 0xef0b8): taken to 0xef118 when `[0x1b0f60] != 0x0`, falls through to 0xef0bc when `[0x1b0f60] == 0x0`.
  Call (`bl` at 0xef0cc): (unknown callee), target 0x22b84, r0=`0x1b0f6c`, r1=`0x184ef8`, r2=`0x0`, r3=`0x9`.
  Call (`bl` at 0xef108): (unknown callee), target 0x22540, r0=`0x1b0f68`, r1=`0x184f04`, r2=`0xef1a0`, r3=`0x0`.
  Word store: target `abs 0x1b0f60`, value `0x1` (`str` at 0xef114).
  Call (`bl` at 0xef11c): (unknown callee), target 0x2302c, r0=`0x1b0f6c`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xef154): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x184f3c`, r2=`0xca`, r3=`0x0`.
  Recorded r0 = `unknown` (`ldm` at 0xef15c).

## `SysComSimStop`

- `SysComSimStop`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xef180 to 0xef1a0.
  Established facts:
  Decoded words in [0xef19c, 0xef1a0) are not reached from the entry by known edges.
  Word store: target `abs 0x1b0cb4`, value `0x0` (`str` at 0xef194).
  Recorded r0 = `r0` (`ldm` at 0xef198).

## `SMPCmdIRouter`

- `SMPCmdIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd3678 to 0xd3930.
  Established facts:
  Decoded words in [0xd3908, 0xd3930) are not reached from the entry by known edges.
  Call (`bl` at 0xd36ac): (unknown callee), target 0xfb734, r0=`sp-2104`, r1=`0x31b68c`, r2=`unknown`, r3=`sp-2104`.
  Call (`bl` at 0xd36c8): (unknown callee), target 0x219a4, r0=`ret(0xfb734)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xfb734)`.
  Call (`bl` at 0xd36ec): (unknown callee), target 0x22070, r0=`0x9`, r1=`0x17943c`, r2=`0x1c53`, r3=`0x0`.
  Grouped accesses: 2 reported byte load records with printed base expression `[sp-2104]` and nonnegative byte-aligned offsets, including target `[sp-2104] +0x20` (`ldrb` at 0xd36f4), target `[sp-2104] +0x2c` (`ldrb` at 0xd3710).
  Branch (`beq` at 0xd36fc): taken to 0xd370c when `[[sp-2104]+32]:1 == 0x2`, falls through to 0xd3700 when `[[sp-2104]+32]:1 != 0x2`.
  Call (`bl` at 0xd3738): (unknown callee), target 0xfde7c, r0=`[sp-2104]`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3760): (unknown callee), target 0xd2364, r0=`unknown`, r1=`sp-2092`, r2=`[sp-2104]`, r3=`sp-2092`.
  Call (`bl` at 0xd3788): (unknown callee), target 0x958d4, r0=`sp-32`, r1=`sp-2088`, r2=`ret(0xd2364)`, r3=`sp-2088`.
  Call (`bl` at 0xd37e8): (unknown callee), target 0x2031c, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd3824): (unknown callee), target 0x95b5c, r0=`sp-32`, r1=`sp-2088`, r2=`unknown`, r3=`ret(0x2031c)`.
  Call (`bl` at 0xd383c): (unknown callee), target 0x20514, r0=`[sp-2092]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`mov` at 0xd3884): (unknown callee), target (unknown target), r0=`[sp-32]`, r1=`sp-2088`, r2=`[0x1b0ec4]`, r3=`sp-2088`.
  Word store: target `abs 0x1b0cfc`, value `0x0` (`str` at 0xd3898).
  Call (`bl` at 0xd38bc): (unknown callee), target 0xfd0dc, r0=`0x31b684`, r1=`0x0`, r2=`[sp-2104]`, r3=`[sp-32]`.
  Call (`bl` at 0xd38e0): (unknown callee), target 0x244d4, r0=`[0x1a288c]`, r1=`0x17966c`, r2=`unknown`, r3=`0x1a288c`.
  Call (`bl` at 0xd38e8): (unknown callee), target 0x219a4, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xd38f8): (unknown callee), target 0x29908, r0=`0x17968c`, r1=`ret(0x219a4)`, r2=`unknown`, r3=`ret(0x219a4)`.
  Call (`bl` at 0xd3900): (unknown callee), target 0xfb794, r0=`[sp-2104]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Jump (`b` at 0xd3904): target 0xd3694.

## `JChipUpdatePort`

- `JChipUpdatePort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13c2dc to 0x13c568.
  Established facts:
  Decoded words in [0x13c550, 0x13c554), [0x13c558, 0x13c568) are not reached from the entry by known edges.
  Word load: target `abs 0x3151c0` (`ldr` at 0x13c2f8).
  Branch (`blt` at 0x13c300): taken to 0x13c310 when `r0 < [0x3151c0] (signed, 32-bit)`, falls through to 0x13c304 when `r0 >= [0x3151c0] (signed, 32-bit)`.
  Call (`bl` at 0x13c39c): (unknown callee), target 0x13c18c, r0=`[sp-52]`, r1=`[sp-56]`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 8 reported byte load records with printed base expression `[sp-52]` and nonnegative byte-aligned offsets, including target `[sp-52] +0x20` (`ldrb` at 0x13c3c0), target `[sp-52] +0x19` (`ldrb` at 0x13c434), target `[sp-52] +0x20` (`ldrb` at 0x13c44c).
  Call (`bl` at 0x13c460): (unknown callee), target 0x13c18c, r0=`unknown`, r1=`[sp-56]`, r2=`[sp-52]`, r3=`[[sp-52]+32]:1`.
  Call (`bl` at 0x13c4b0): (unknown callee), target 0x138850, r0=`[[sp-52]+25]:1`, r1=`unknown`, r2=`[sp-44]`, r3=`[[[sp-52]]]`.
  Call (`bl` at 0x13c528): (unknown callee), target 0x243d4, r0=`0x191bf4`, r1=`[sp-24]`, r2=`[[[sp-52]]+8]`, r3=`[[sp-52]+24]:1`.
  Recorded r0 = `unknown` (`ldm` at 0x13c54c).

## `routerExitServiceMode`

- `routerExitServiceMode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a198 to 0x10a314.
  Established facts:
  Decoded words in [0x10a308, 0x10a314) are not reached from the entry by known edges.
  Call (`bl` at 0x10a1c8): (unknown callee), target 0x2624c, r0=`0x50`, r1=`0x0`, r2=`[0x18c7b8]`, r3=`0x18c7b0`.
  Call (`bl` at 0x10a1d4): (unknown callee), target 0x2624c, r0=`0x3e`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a1e0): (unknown callee), target 0x2624c, r0=`0x3f`, r1=`0x0`, r2=`unknown`, r3=`unknown`.
  Branch (`bne` at 0x10a1ec): taken to 0x10a204 when `[sp-20] != 0x1`, falls through to 0x10a1f0 when `[sp-20] == 0x1`.
  Call (`bl` at 0x10a1fc): (unknown callee), target 0x2624c, r0=`0xd`, r1=`sp-32`, r2=`unknown`, r3=`sp-32`.
  Word store: target `abs 0x1a6110`, value `0x2` (`str` at 0x10a20c).
  Call (`bl` at 0x10a21c): (unknown callee), target 0x2624c, r0=`0xd`, r1=`sp-32`, r2=`0x1a6110`, r3=`sp-32`.
  Call (`bl` at 0x10a228): (unknown callee), target 0x2624c, r0=`0x3d`, r1=`0x2`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a234): (unknown callee), target 0x2624c, r0=`0x7b`, r1=`0x1`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a238): (unknown callee), target 0x6dc0c, r0=`ret(0x2624c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a24c): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a260): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a274): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a288): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a29c): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a2b0): (unknown callee), target 0x6dc0c, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a2c8): (unknown callee), target 0x6e190, r0=`ret(0x6dc0c)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a2dc): (unknown callee), target 0x5ba44, r0=`ret(0x6e190)`, r1=`0x6`, r2=`0x2`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x10a2e0): (unknown callee), target 0x6e190, r0=`ret(0x5ba44)`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0x10a2f4): (unknown callee), target 0x5ba44, r0=`ret(0x6e190)`, r1=`0x7`, r2=`0x2`, r3=`ret(0x6e190)`.
  Call (`bl` at 0x10a2fc): (unknown callee), target 0x293f8, r0=`0x18c7bc`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Recorded r0 = `ret(0x293f8)` (`ldm` at 0x10a304, `bl` at 0x10a2fc).

## `SerialQuery`

- `SerialQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf4d30 to 0xf54a8.
  Established facts:
  Decoded words in [0xf5460, 0xf54a8) are not reached from the entry by known edges.
  Call (`bl` at 0xf4d50): (unknown callee), target 0x6e140, r0=`r0`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0xf4da8): (unknown callee), target 0x120340, r0=`[sp-48]`, r1=`sp-56`, r2=`[sp-24]`, r3=`sp-56`.
  Call (`bl` at 0xf4df8): (unknown callee), target 0x201fc, r0=`0x50`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf4e0c): (unknown callee), target 0x20634, r0=`[[sp-36]]`, r1=`0x1872ac`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0xf4e40): (unknown callee), target 0x201fc, r0=`unknown`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`mov` at 0xf4e80): (unknown callee), target (unknown target), r0=`0x1`, r1=`unknown`, r2=`unknown`, r3=`[0x318478]`.
  Call (`bl` at 0xf4e94): (unknown callee), target 0x28230, r0=`0xd`, r1=`unknown`, r2=`unknown`, r3=`[0x318478]`.
  Call (`bl` at 0xf4ea4): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x1872c8`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf4eac): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Byte load: target `[sp-76] +0x0` (`ldrb` at 0xf4ec8).
  Branch (`beq` at 0xf4ed0): taken to 0xf4ee8 when `[[sp-76]]:1 == 0x0`, falls through to 0xf4ed4 when `[[sp-76]]:1 != 0x0`.
  Call (`bl` at 0xf4ed8): (unknown callee), target 0x2031c, r0=`[sp-76]`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`[[sp-76]]:1`.
  Call (`bl` at 0xf4ef0): (unknown callee), target 0x20634, r0=`unknown`, r1=`0x187338`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf4f04): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x187370`, r2=`[sp-76]`, r3=`ret(0x2031c)`.
  Call (`bl` at 0xf4f0c): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Grouped accesses: 4 reported word load records with absolute target `0x1b0f54`, including target `abs 0x1b0f54` (`ldr` at 0xf4f28), target `abs 0x1b0f54` (`ldr` at 0xf4f3c), target `abs 0x1b0f54` (`ldr` at 0xf4f50).
  Call (`bl` at 0xf4fbc): (unknown callee), target 0x205bc, r0=`([sp-44] + ret(0x2031c))`, r1=`0x1873f4`, r2=`unknown`, r3=`0x31b96c`.
  Call (`bl` at 0xf4fc4): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf4fdc): (unknown callee), target 0x6debc, r0=`ret(0x2031c)`, r1=`unknown`, r2=`ret(0x2031c)`, r3=`([sp-44] + ret(0x2031c))`.
  Call (`bl` at 0xf4fe8): (unknown callee), target 0x28230, r0=`0xf`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf4ffc): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x187420`, r2=`ret(0x6debc)`, r3=`ret(0x28230)`.
  Call (`bl` at 0xf5004): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5078): (unknown callee), target 0x120340, r0=`[sp-48]`, r1=`sp-56`, r2=`[sp-24]`, r3=`sp-56`.
  Call (`bl` at 0xf50b4): (unknown callee), target 0xf2d10, r0=`[sp-48]`, r1=`[sp-56]`, r2=`ret(0x120340)`, r3=`[sp-44]`.
  Call (`bl` at 0xf512c): (unknown callee), target 0x120340, r0=`[sp-48]`, r1=`sp-56`, r2=`[sp-24]`, r3=`sp-56`.
  Call (`bl` at 0xf5168): (unknown callee), target 0xf4518, r0=`[sp-48]`, r1=`[sp-56]`, r2=`ret(0x120340)`, r3=`[sp-44]`.
  Call (`bl` at 0xf5174): (unknown callee), target 0x6dc0c, r0=`ret(0xf4518)`, r1=`unknown`, r2=`unknown`, r3=`ret(0xf4518)`.
  Call (`bl` at 0xf51b8): (unknown callee), target 0x28230, r0=`0x7b`, r1=`[sp-88]`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf51d0): (unknown callee), target 0x205bc, r0=`[sp-44]`, r1=`0x187484`, r2=`unknown`, r3=`ret(0x28230)`.
  Call (`bl` at 0xf51d8): (unknown callee), target 0x2031c, r0=`[sp-44]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf525c): (unknown callee), target 0x120340, r0=`[sp-48]`, r1=`sp-56`, r2=`[sp-24]`, r3=`sp-56`.
  Call (`bl` at 0xf5298): (unknown callee), target 0xf33a0, r0=`[sp-48]`, r1=`[sp-56]`, r2=`ret(0x120340)`, r3=`[sp-44]`.
  Call (`bl` at 0xf5310): (unknown callee), target 0x120340, r0=`[sp-48]`, r1=`sp-56`, r2=`[sp-24]`, r3=`sp-56`.
  Call (`bl` at 0xf5344): (unknown callee), target 0xf3dd0, r0=`[sp-48]`, r1=`[sp-56]`, r2=`ret(0x120340)`, r3=`[sp-44]`.
  Call (`bl` at 0xf53bc): (unknown callee), target 0x120340, r0=`[sp-48]`, r1=`sp-56`, r2=`[sp-24]`, r3=`sp-56`.
  Call (`bl` at 0xf53f0): (unknown callee), target 0xf4150, r0=`[sp-48]`, r1=`[sp-56]`, r2=`ret(0x120340)`, r3=`[sp-44]`.
  Call (`bl` at 0xf5414): (unknown callee), target 0x20634, r0=`unknown`, r1=`0x187060`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xf5420): (unknown callee), target 0x2031c, r0=`[[sp-36]]`, r1=`unknown`, r2=`unknown`, r3=`[sp-36]`.
  Call (`bl` at 0xf5448): (unknown callee), target 0x29908, r0=`0x187494`, r1=`ret(0x2031c)`, r2=`[sp-68]`, r3=`[sp-68]`.
  Recorded r0 = `[sp-80]` (`ldm` at 0xf545c).

## `cmdFlashVersion`

- `cmdFlashVersion`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5298 to 0xe54b8.
  Established facts:
  Decoded words in [0xe54a4, 0xe54b8) are not reached from the entry by known edges.
  Byte load: target `abs 0x1842d0` (`ldrb` at 0xe52b4).
  Grouped accesses: 2 reported word load records with absolute target `0x31e428`, including target `abs 0x31e428` (`ldr` at 0xe536c), target `abs 0x31e428` (`ldr` at 0xe5408).
  Branch (`bne` at 0xe5378): taken to 0xe538c when `[0x31e428] != [0x31e420]`, falls through to 0xe537c when `[0x31e428] == [0x31e420]`.
  Call (`bl` at 0xe53a8): (unknown callee), target 0x201fc, r0=`0x80`, r1=`[0x31e428]`, r2=`0x31e418`, r3=`[sp-108]`.
  Call (`bl` at 0xe53c0): (unknown callee), target 0x108488, r0=`[sp-48]`, r1=`sp-96`, r2=`unknown`, r3=`sp-96`.
  Call (`bl` at 0xe53e0): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x184430`, r2=`0x184464`, r3=`[sp-28]`.
  Call (`bl` at 0xe548c): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x18446c`, r2=`[sp-76]`, r3=`[sp-116]`.
  Recorded r0 = `0x0` (`ldm` at 0xe54a0).

## `cmdFlashBoot`

- `cmdFlashBoot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4b24 to 0xe5270.
  Established facts:
  Decoded words in [0xe4b98, 0xe520c), [0xe521c, 0xe5270) are not reached from the entry by known edges.
  Word load: target `r0 +0x70` (`ldr` at 0xe4b40).
  Call (`bl` at 0xe4b54): (unknown callee), target 0x201fc, r0=`0x300`, r1=`r1`, r2=`r2`, r3=`[[r0+112]]`.
  Byte store: target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0xe4b6c).
  Call (`bl` at 0xe4b80): (unknown callee), target 0xe5270, r0=`0x64`, r1=`unknown`, r2=`[[sp-28]]`, r3=`[sp-36]`.
  Branch (`ldrls` at 0xe4b90): taken to (unknown target) when `[sp-36]+3 <= 0xa (unsigned, 32-bit)`, falls through to 0xe4b94 when `[sp-36]+3 > 0xa (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0xe4b90).
  Known-edge recorded r0 = `0x0` (`ldm` at 0xe5218).

## `cmdFlashSetDefault`

- `cmdFlashSetDefault`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe54b8 to 0xe56fc.
  Established facts:
  Decoded words in [0xe551c, 0xe5688), [0xe56f0, 0xe56fc) are not reached from the entry by known edges.
  Word load: target `r0 +0x70` (`ldr` at 0xe54d4).
  Call (`bl` at 0xe54f0): (unknown callee), target 0x201fc, r0=`0x80`, r1=`r1`, r2=`r2`, r3=`0x0`.
  Grouped accesses: 2 reported byte store records with printed base expression `[[sp-28]]` and nonnegative byte-aligned offsets, including target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0xe5508), target `[[sp-28]] +0x0`, value `0x0` (`strb` at 0xe56d8).
  Branch (`ldrls` at 0xe5514): taken to (unknown target) when `[sp-36] <= 0x4 (unsigned, 32-bit)`, falls through to 0xe5518 when `[sp-36] > 0x4 (unsigned, 32-bit)`.
  Omitted-path effects remain unknown at the unresolved transfer (`ldrls` at 0xe5514).
  Call (`bl` at 0xe568c): (unknown callee), target 0x110e38, r0=`[sp-40]`, r1=`unknown`, r2=`[[sp-28]]`, r3=`[sp-36]`.
  Call (`bl` at 0xe56a8): (unknown callee), target 0x219a4, r0=`ret(0x110e38)`, r1=`unknown`, r2=`unknown`, r3=`ret(0x110e38)`.
  Call (`bl` at 0xe56bc): (unknown callee), target 0x205bc, r0=`[[sp-28]]`, r1=`0x1844dc`, r2=`ret(0x219a4)`, r3=`[sp-28]`.
  Known-edge recorded r0 = `unknown` (`ldm` at 0xe56ec).

## `JChip_llp_port_by_name`

- `JChip_llp_port_by_name`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe0424 to 0xe05b4.
  Established facts:
  Decoded words in [0xe05b0, 0xe05b4) are not reached from the entry by known edges.
  Call (`bl` at 0xe0444): (unknown callee), target 0x2031c, r0=`r1`, r1=`r1`, r2=`r2`, r3=`r3`.
  Call (`bl` at 0xe046c): (unknown callee), target 0x2031c, r0=`[sp-28]`, r1=`unknown`, r2=`unknown`, r3=`unknown`.
  Call (`bl` at 0xe04a4): (unknown callee), target 0x203ac, r0=`[([sp-28] + [sp-40])]:1`, r1=`unknown`, r2=`[sp-28]`, r3=`[([sp-28] + [sp-40])]:1`.
  Grouped accesses: 3 reported byte load records with printed base expression `[sp-32]` and nonnegative byte-aligned offsets, including target `[sp-32] +0x0` (`ldrb` at 0xe04ec), target `[sp-32] +0x0` (`ldrb` at 0xe054c), target `[sp-32] +0x0` (`ldrb` at 0xe0568).
  Branch (`ble` at 0xe0500): taken to 0xe0510 when `[[sp-32]]:1-65 <= 0x7 (signed, 32-bit)`, falls through to 0xe0504 when `[[sp-32]]:1-65 > 0x7 (signed, 32-bit)`.
  Byte store: target `[sp-32] +0x1`, value `0x0` (`strb` at 0xe0520).
  Call (`bl` at 0xe0538): (unknown callee), target 0x204fc, r0=`[sp-32]`, r1=`0x180bf8`, r2=`sp-36`, r3=`sp-36`.
  Call (`bl` at 0xe0598): (unknown callee), target 0x205bc, r0=`[sp-32]`, r1=`0x180bf8`, r2=`[sp-36]`, r3=`unknown`.
  Recorded r0 = `[sp-44]` (`ldm` at 0xe05ac).
