# L1 helper routines

Read from package 1 of IRIX 6.5.30 `usr/cpu/firmware/sysco/l1.bin` (L1 1.44.0). [controllers.md](../controllers.md) identifies the L1 CPU as the ColdFire MCF5206e; [personalities.md](personalities.md) records the 0xffe00000 load base and second code copy. Listings were made from this image with Capstone in big-endian 68040 mode.

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

- write_selector_0x0c_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9b810
  It calls 0xffec8758 with selector 0x0c and argument 0, then calls 0xffe9b798 (move.l at 0xffe9b814, moveq at 0xffe9b81a, jsr at 0xffe9b81e, bsr.w at 0xffe9b826). Selector 0x0c has no identified meaning in this listing.

- write_selector_0x7d_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9b888
  It calls 0xffec8758 with selector 0x7d and argument 0, then calls 0xffe9b798 (move.l at 0xffe9b88c, moveq at 0xffe9b892, jsr at 0xffe9b896, bsr.w at 0xffe9b89e). Selector 0x7d has no identified meaning in this listing.

- write_selector_0x07_bank2 — 6.5.30/usr/cpu/firmware/sysco/l1.bin, 0xffe9b8a8
  It converts argument 0 to 1 when zero and to 0 otherwise, passes that value with selector 7 to 0xffec8758, then calls 0xffe9b798 (cmp.l at 0xffe9b8b0, seq.b at 0xffe9b8b6, moveq at 0xffe9b8c0, jsr at 0xffe9b8c4, bsr.w at 0xffe9b8cc). Selector 7 has no identified meaning in this listing.
