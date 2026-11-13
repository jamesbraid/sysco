# L1 helper routines

Read from package 1 of IRIX 6.5.30 `usr/cpu/firmware/sysco/l1.bin` (L1 1.44.0). [controllers.md](../../../evidence/controllers.md) identifies the L1 CPU as the ColdFire MCF5206e; [personalities.md](personalities.md) records the 0xffe00000 load base and second code copy. Listings were made from this image with Capstone in big-endian 68040 mode.

- indexed_address_store_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe80ba6
  It forms an address as four times argument 0 from base zero, reads the old longword into d0, stores argument 1 there, and returns the old value in d0 (lsl.l at 0xffe80bb0, adda.l at 0xffe80bb2, move.l at 0xffe80bb4, move.l at 0xffe80bba, rts at 0xffe80bbc). The access target is not identified by this code.

- indexed_longword_read_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe80bbe
  It loads a longword from 0xffe80004 + 4*argument 0 into d0 and returns it (lea.l at 0xffe80bbe, asl.l at 0xffe80bc8, move.l at 0xffe80bca, rts at 0xffe80bce). The table’s use is Unknown.

- call_with_two_words_then_zero_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe841b4
  It reads the pointer stored at argument 0 plus 0x70 and calls 0xffe840d4 with the longword that pointer addresses and argument 1; after testing that call’s result, it sets d0 to zero before returning (move.l at 0xffe841b8, movea.l at 0xffe841bc, movea.l at 0xffe841c0, bsr.w at 0xffe841c6, tst.l at 0xffe841cc, moveq at 0xffe841d6). The routine’s broader role is Unknown.

- zero_result_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe84390
  It sets d0 to zero and returns (moveq at 0xffe84394, rts at 0xffe84398). The routine’s broader role is Unknown.

- bounded_pointer_choice_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe8439c
  It subtracts one from argument 0 and compares that value with 0x26; bcs.b at 0xffe843aa takes the branch when the unsigned value is greater, returning 0xffe93f2b, while fall-through returns zero (subq.l at 0xffe843a4, cmp.l at 0xffe843a8, bcs.b at 0xffe843aa, move.l at 0xffe843b0, moveq at 0xffe843ac). The code does not identify what the selected pointer represents.

- inverted_call_result_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe843bc
  It calls 0xffeed32c with argument 0 and pointer 0xffe93f61; a zero result becomes 1, and a nonzero result becomes 0 (pea.l at 0xffe843c4, move.l at 0xffe843ca, jsr at 0xffe843cc, tst.l at 0xffe843d4, bne.b at 0xffe843d6, moveq at 0xffe843d8, moveq at 0xffe843dc). This wrapper establishes no role for its callee.

- two_call_forwarder_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe8bc34
  It passes argument 0 to 0xffecb6b4, then passes that call’s d0 result and pointer 0xffe96f83 to 0xffecaed8 (move.l at 0xffe8bc38, jsr at 0xffe8bc3e, move.l at 0xffe8bc46, pea.l at 0xffe8bc48, jsr at 0xffe8bc4e). The routine’s broader role is Unknown.

- selector_0x0c_call_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9b810
  It calls 0xffec8758 with selector 0x0c and argument 0, then calls 0xffe9b798 (move.l at 0xffe9b814, moveq at 0xffe9b81a, jsr at 0xffe9b81e, bsr.w at 0xffe9b826). Selector 0x0c has no identified meaning in this listing.

- selector_0x7d_call_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9b888
  It calls 0xffec8758 with selector 0x7d and argument 0, then calls 0xffe9b798 (move.l at 0xffe9b88c, moveq at 0xffe9b892, jsr at 0xffe9b896, bsr.w at 0xffe9b89e). Selector 0x7d has no identified meaning in this listing.

- selector_0x07_call_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9b8a8
  It converts argument 0 to 1 when zero and to 0 otherwise, passes that value with selector 7 to 0xffec8758, then calls 0xffe9b798 (cmp.l at 0xffe9b8b0, seq.b at 0xffe9b8b6, moveq at 0xffe9b8c0, jsr at 0xffe9b8c4, bsr.w at 0xffe9b8cc). Selector 7 has no identified meaning in this listing.

- selector_two_dispatch_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9be58
  It passes selector 2 to 0xffe9b8d4 and returns that call’s result in d0 (moveq at 0xffe9be5c, bsr.w at 0xffe9be60, rts at 0xffe9be68). What selector 2 requests is Unknown.

- forward_pointer_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9be90
  It passes argument 0 to 0xffe98178 and returns that call’s result (movea.l at 0xffe9be94, move.l at 0xffe9be98, bsr.w at 0xffe9be9a, rts at 0xffe9bea2). The callee’s effect is not established here.

- selector_three_dispatch_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9bea4
  It passes selector 3 to 0xffe9b8d4 and leaves its result in d0 for return (moveq at 0xffe9bea8, bsr.w at 0xffe9beac, rts at 0xffe9beb4). Selector 3’s meaning is Unknown.

- selector_four_then_call_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9beb8
  It passes selector 4 to 0xffe9b8d4, then calls 0xffea517c with no arguments; the second call’s d0 is returned (moveq at 0xffe9bebc, bsr.w at 0xffe9bec0, jsr at 0xffe9bec6, rts at 0xffe9bece). Neither selector 4 nor the final call’s role is established.

- call_result_pair_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9bee8
  It saves argument 0, calls 0xffe99160 without arguments, then calls 0xffe98ff8 with argument 0 and the first call’s d0 result; the second result remains in d0 on return (move.l at 0xffe9beee, bsr.w at 0xffe9bef2, move.l at 0xffe9bef6, move.l at 0xffe9bef8, bsr.w at 0xffe9befa). The calls’ roles are Unknown.

- zero_selector_call_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffea1ea4
  It passes zero to 0xffea1d7c and returns its result (clr.l at 0xffea1ea8, bsr.w at 0xffea1eaa, rts at 0xffea1eb2). The selector’s role is Unknown.

- one_selector_call_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffea1eb4
  It passes 1 to 0xffea1d7c and returns its result (moveq at 0xffea1eb8, move.l at 0xffea1eba, bsr.w at 0xffea1ebc, rts at 0xffea1ec4). The selector’s role is Unknown.

- triple_zero_selector_one_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffea3220
  It calls 0xffea3060 with arguments 1, 0, 0 and returns its result (clr.l at 0xffea3224, clr.l at 0xffea3226, moveq at 0xffea3228, bsr.w at 0xffea322c, rts at 0xffea3236). The call’s role is Unknown.

- triple_zero_selector_two_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffea3238
  It calls 0xffea3060 with arguments 2, 0, 0 and returns its result (clr.l at 0xffea323c, clr.l at 0xffea323e, moveq at 0xffea3240, bsr.w at 0xffea3244, rts at 0xffea324e). The call’s role is Unknown.

- triple_zero_selector_three_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffea3250
  It calls 0xffea3060 with arguments 3, 0, 0 and returns its result (clr.l at 0xffea3254, clr.l at 0xffea3256, moveq at 0xffea3258, bsr.w at 0xffea325c, rts at 0xffea3266). The call’s role is Unknown.
