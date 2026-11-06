# MMSC port and interrupt leaf windows

Each entry names one canonical entry in its own exact shipped carrier.
The analysis spans are stated separately from native nlist symbol bounds, whose ends are inferred from the next symbol.
An explicitly given combined view does not replace a native symbol size or prove complete function extent.
No internal-label window is counted as a separate routine.
Hardware roles, types, pointee meanings, actual arity, custom ABI and runtime placement remain unknown.
Local CFG statements assume ordinary call fallthrough and do not establish runtime feasibility.
I/O records retain space=io. A port coordinate is not a memory address or an identified physical device.
Static access groups require identical encoded instructions and native access fields within one view.
Their counts establish neither dynamic access counts nor equal runtime pointees.
All native accesses are retained in these assembly views. No frame bookkeeping is omitted.
Known expressions stay literal. Only the native value ? becomes unknown.
No iteration count is inferred from an entry register constant.
Unknown flags and unsupported instructions do not establish interrupt state, timing or saved-flag values.
Outside-view known targets remain stated. Their subsequent effects and return state are not filled in.
Decoded records outside known local edges have unlisted effects and are not established as globally unreachable.

The [full facts](disasm/port-interrupt-leaf-windows-facts.txt) purpose-label canonical views and separate native/internal symbol evidence.

- `_sysWait`, `6.4/usr/cpu/firmware/mmscfw.bin`, `0x1081a0`.
  Observation view: 0x1081a0 to 0x1081b0 (exclusive), 16 bytes. Explicit combined given view.
  Native nlist window: 0x1081a0 to 0x1081a2 (exclusive), 2 bytes, end inferred from the next symbol. Observation grade: established. Role: unknown.
  Carrier: 998378 bytes, SHA256 `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
  Nlist symbol index 16, body file offset 0x1c0, view SHA256 `eaa800b6482732c6af6ddf042ccf5e5345eeabd51d5c58a4a59c6350be99f15a`.
  Separate native symbol model: 2 bytes, no exit modeled. The combined view does not import an exit into that model.
  Byte load, pattern 1: space `io`, target `abs 0x64`, raw base `port`. Count 1 static access. Sites: `in` at 0x1081a7.
  Branch (`loopne` at 0x1081aa): target 0x1081a2. Taken when `? (ecx after decrement, 32-bit wrap) != 0x0 and (Z == 0)`. Falls through when `? (ecx after decrement, 32-bit wrap) == 0x0 or (not (Z == 0))`.
  At the exit (`ret` at 0x1081ac), eax=`unknown`.
  Register fact (`xor` at 0x1081a0): ecx=`0x0`.
  Register fact (`mov` at 0x1081a2): edx=`0x64`.
  Immediate (`and` at 0x1081a8): `0x2`.
  Decoded records outside known local edges: 0x1081ad to 0x1081af, ending at 0x1081b0 (exclusive). Count 3.

- `_sysWait`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x1081a0`.
  Observation view: 0x1081a0 to 0x1081b0 (exclusive), 16 bytes. Explicit combined given view.
  Native nlist window: 0x1081a0 to 0x1081a2 (exclusive), 2 bytes, end inferred from the next symbol. Observation grade: established. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 16, body file offset 0x1c0, view SHA256 `eaa800b6482732c6af6ddf042ccf5e5345eeabd51d5c58a4a59c6350be99f15a`.
  Separate native symbol model: 2 bytes, no exit modeled. The combined view does not import an exit into that model.
  Byte load, pattern 1: space `io`, target `abs 0x64`, raw base `port`. Count 1 static access. Sites: `in` at 0x1081a7.
  Branch (`loopne` at 0x1081aa): target 0x1081a2. Taken when `? (ecx after decrement, 32-bit wrap) != 0x0 and (Z == 0)`. Falls through when `? (ecx after decrement, 32-bit wrap) == 0x0 or (not (Z == 0))`.
  At the exit (`ret` at 0x1081ac), eax=`unknown`.
  Register fact (`xor` at 0x1081a0): ecx=`0x0`.
  Register fact (`mov` at 0x1081a2): edx=`0x64`.
  Immediate (`and` at 0x1081a8): `0x2`.
  Decoded records outside known local edges: 0x1081ad to 0x1081af, ending at 0x1081b0 (exclusive). Count 3.

- `_sysWait`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1081a0`.
  Observation view: 0x1081a0 to 0x1081b0 (exclusive), 16 bytes. Explicit combined given view.
  Native nlist window: 0x1081a0 to 0x1081a2 (exclusive), 2 bytes, end inferred from the next symbol. Observation grade: established. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 16, body file offset 0x1c0, view SHA256 `eaa800b6482732c6af6ddf042ccf5e5345eeabd51d5c58a4a59c6350be99f15a`.
  Separate native symbol model: 2 bytes, no exit modeled. The combined view does not import an exit into that model.
  Byte load, pattern 1: space `io`, target `abs 0x64`, raw base `port`. Count 1 static access. Sites: `in` at 0x1081a7.
  Branch (`loopne` at 0x1081aa): target 0x1081a2. Taken when `? (ecx after decrement, 32-bit wrap) != 0x0 and (Z == 0)`. Falls through when `? (ecx after decrement, 32-bit wrap) == 0x0 or (not (Z == 0))`.
  At the exit (`ret` at 0x1081ac), eax=`unknown`.
  Register fact (`xor` at 0x1081a0): ecx=`0x0`.
  Register fact (`mov` at 0x1081a2): edx=`0x64`.
  Immediate (`and` at 0x1081a8): `0x2`.
  Decoded records outside known local edges: 0x1081ad to 0x1081af, ending at 0x1081b0 (exclusive). Count 3.

- `_sysWait`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x1081a0`.
  Observation view: 0x1081a0 to 0x1081b0 (exclusive), 16 bytes. Explicit combined given view.
  Native nlist window: 0x1081a0 to 0x1081a2 (exclusive), 2 bytes, end inferred from the next symbol. Observation grade: established. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 16, body file offset 0x1c0, view SHA256 `eaa800b6482732c6af6ddf042ccf5e5345eeabd51d5c58a4a59c6350be99f15a`.
  Separate native symbol model: 2 bytes, no exit modeled. The combined view does not import an exit into that model.
  Byte load, pattern 1: space `io`, target `abs 0x64`, raw base `port`. Count 1 static access. Sites: `in` at 0x1081a7.
  Branch (`loopne` at 0x1081aa): target 0x1081a2. Taken when `? (ecx after decrement, 32-bit wrap) != 0x0 and (Z == 0)`. Falls through when `? (ecx after decrement, 32-bit wrap) == 0x0 or (not (Z == 0))`.
  At the exit (`ret` at 0x1081ac), eax=`unknown`.
  Register fact (`xor` at 0x1081a0): ecx=`0x0`.
  Register fact (`mov` at 0x1081a2): edx=`0x64`.
  Immediate (`and` at 0x1081a8): `0x2`.
  Decoded records outside known local edges: 0x1081ad to 0x1081af, ending at 0x1081b0 (exclusive). Count 3.

- `_sysWait`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x1081a0`.
  Observation view: 0x1081a0 to 0x1081b0 (exclusive), 16 bytes. Explicit combined given view.
  Native nlist window: 0x1081a0 to 0x1081a2 (exclusive), 2 bytes, end inferred from the next symbol. Observation grade: established. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 16, body file offset 0x1c0, view SHA256 `eaa800b6482732c6af6ddf042ccf5e5345eeabd51d5c58a4a59c6350be99f15a`.
  Separate native symbol model: 2 bytes, no exit modeled. The combined view does not import an exit into that model.
  Byte load, pattern 1: space `io`, target `abs 0x64`, raw base `port`. Count 1 static access. Sites: `in` at 0x1081a7.
  Branch (`loopne` at 0x1081aa): target 0x1081a2. Taken when `? (ecx after decrement, 32-bit wrap) != 0x0 and (Z == 0)`. Falls through when `? (ecx after decrement, 32-bit wrap) == 0x0 or (not (Z == 0))`.
  At the exit (`ret` at 0x1081ac), eax=`unknown`.
  Register fact (`xor` at 0x1081a0): ecx=`0x0`.
  Register fact (`mov` at 0x1081a2): edx=`0x64`.
  Immediate (`and` at 0x1081a8): `0x2`.
  Decoded records outside known local edges: 0x1081ad to 0x1081af, ending at 0x1081b0 (exclusive). Count 3.

- `_intLock`, `6.4/usr/cpu/firmware/mmscfw.bin`, `0x170618`.
  Observation view: 0x170618 to 0x170628 (exclusive), 16 bytes. Native inferred symbol window.
  Native nlist window: 0x170618 to 0x170628 (exclusive), 16 bytes. Observation grade: established. Role: unknown.
  Carrier: 998378 bytes, SHA256 `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
  Nlist symbol index 115, body file offset 0x68638, view SHA256 `109f7709eb74ab7db90509c77d55fcdccb007a0170b9a42b5fec981dd07d9985`.
  Four-byte load, pattern 1: target `unknown`, raw base `esp`. Count 1 static access. Sites: `pop` at 0x170619.
  At the exit (`ret` at 0x170620), eax=`unknown`.
  Immediate (`and` at 0x17061a): `0x200`.
  Tool warning: `0x170618 pushfd: unsupported effects invalidated`.
  Tool warning: `0x17061f cli: unsupported effects invalidated`.
  Decoded records outside known local edges: 0x170621 to 0x170627, ending at 0x170628 (exclusive). Count 7.

- `_intUnlock`, `6.4/usr/cpu/firmware/mmscfw.bin`, `0x170628`.
  Observation view: 0x170628 to 0x170634 (exclusive), 12 bytes. Native inferred symbol window.
  Native nlist window: 0x170628 to 0x170634 (exclusive), 12 bytes. Observation grade: unknown. Role: unknown.
  Carrier: 998378 bytes, SHA256 `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
  Nlist symbol index 118, body file offset 0x68648, view SHA256 `ebe59457ced429b0d530380db05285d03e13df6599c29b0a588a704e23dca05b`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `esp`. Count 1 static access. Sites: `mov` at 0x170628.
  Branch (`je` at 0x170631): target 0x170634 outside this view. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  No reached exit is modeled.
  Boundary (`sti` at 0x170633): instruction end 0x170634. The native CFG supplies no local successor there.
  Immediate (`and` at 0x17062c): `0x200`.
  Tool warning: `0x170633 sti: unsupported effects invalidated`.

- `_intLock`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x173a98`.
  Observation view: 0x173a98 to 0x173aa8 (exclusive), 16 bytes. Native inferred symbol window.
  Native nlist window: 0x173a98 to 0x173aa8 (exclusive), 16 bytes. Observation grade: established. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 115, body file offset 0x6bab8, view SHA256 `109f7709eb74ab7db90509c77d55fcdccb007a0170b9a42b5fec981dd07d9985`.
  Four-byte load, pattern 1: target `unknown`, raw base `esp`. Count 1 static access. Sites: `pop` at 0x173a99.
  At the exit (`ret` at 0x173aa0), eax=`unknown`.
  Immediate (`and` at 0x173a9a): `0x200`.
  Tool warning: `0x173a98 pushfd: unsupported effects invalidated`.
  Tool warning: `0x173a9f cli: unsupported effects invalidated`.
  Decoded records outside known local edges: 0x173aa1 to 0x173aa7, ending at 0x173aa8 (exclusive). Count 7.

- `_intUnlock`, `6.5/usr/cpu/firmware/mmscfw.bin`, `0x173aa8`.
  Observation view: 0x173aa8 to 0x173ab4 (exclusive), 12 bytes. Native inferred symbol window.
  Native nlist window: 0x173aa8 to 0x173ab4 (exclusive), 12 bytes. Observation grade: unknown. Role: unknown.
  Carrier: 1016714 bytes, SHA256 `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
  Nlist symbol index 118, body file offset 0x6bac8, view SHA256 `ebe59457ced429b0d530380db05285d03e13df6599c29b0a588a704e23dca05b`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `esp`. Count 1 static access. Sites: `mov` at 0x173aa8.
  Branch (`je` at 0x173ab1): target 0x173ab4 outside this view. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  No reached exit is modeled.
  Boundary (`sti` at 0x173ab3): instruction end 0x173ab4. The native CFG supplies no local successor there.
  Immediate (`and` at 0x173aac): `0x200`.
  Tool warning: `0x173ab3 sti: unsupported effects invalidated`.

- `_intLock`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1713d0`.
  Observation view: 0x1713d0 to 0x1713e0 (exclusive), 16 bytes. Native inferred symbol window.
  Native nlist window: 0x1713d0 to 0x1713e0 (exclusive), 16 bytes. Observation grade: established. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 115, body file offset 0x693f0, view SHA256 `109f7709eb74ab7db90509c77d55fcdccb007a0170b9a42b5fec981dd07d9985`.
  Four-byte load, pattern 1: target `unknown`, raw base `esp`. Count 1 static access. Sites: `pop` at 0x1713d1.
  At the exit (`ret` at 0x1713d8), eax=`unknown`.
  Immediate (`and` at 0x1713d2): `0x200`.
  Tool warning: `0x1713d0 pushfd: unsupported effects invalidated`.
  Tool warning: `0x1713d7 cli: unsupported effects invalidated`.
  Decoded records outside known local edges: 0x1713d9 to 0x1713df, ending at 0x1713e0 (exclusive). Count 7.

- `_intUnlock`, `6.5.2/usr/cpu/firmware/mmscfw.bin`, `0x1713e0`.
  Observation view: 0x1713e0 to 0x1713ec (exclusive), 12 bytes. Native inferred symbol window.
  Native nlist window: 0x1713e0 to 0x1713ec (exclusive), 12 bytes. Observation grade: unknown. Role: unknown.
  Carrier: 1005559 bytes, SHA256 `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
  Nlist symbol index 118, body file offset 0x69400, view SHA256 `ebe59457ced429b0d530380db05285d03e13df6599c29b0a588a704e23dca05b`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `esp`. Count 1 static access. Sites: `mov` at 0x1713e0.
  Branch (`je` at 0x1713e9): target 0x1713ec outside this view. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  No reached exit is modeled.
  Boundary (`sti` at 0x1713eb): instruction end 0x1713ec. The native CFG supplies no local successor there.
  Immediate (`and` at 0x1713e4): `0x200`.
  Tool warning: `0x1713eb sti: unsupported effects invalidated`.

- `_intLock`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x175128`.
  Observation view: 0x175128 to 0x175138 (exclusive), 16 bytes. Native inferred symbol window.
  Native nlist window: 0x175128 to 0x175138 (exclusive), 16 bytes. Observation grade: established. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 115, body file offset 0x6d148, view SHA256 `109f7709eb74ab7db90509c77d55fcdccb007a0170b9a42b5fec981dd07d9985`.
  Four-byte load, pattern 1: target `unknown`, raw base `esp`. Count 1 static access. Sites: `pop` at 0x175129.
  At the exit (`ret` at 0x175130), eax=`unknown`.
  Immediate (`and` at 0x17512a): `0x200`.
  Tool warning: `0x175128 pushfd: unsupported effects invalidated`.
  Tool warning: `0x17512f cli: unsupported effects invalidated`.
  Decoded records outside known local edges: 0x175131 to 0x175137, ending at 0x175138 (exclusive). Count 7.

- `_intUnlock`, `6.5.5/usr/cpu/firmware/mmscfw.bin`, `0x175138`.
  Observation view: 0x175138 to 0x175144 (exclusive), 12 bytes. Native inferred symbol window.
  Native nlist window: 0x175138 to 0x175144 (exclusive), 12 bytes. Observation grade: unknown. Role: unknown.
  Carrier: 1023103 bytes, SHA256 `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
  Nlist symbol index 118, body file offset 0x6d158, view SHA256 `ebe59457ced429b0d530380db05285d03e13df6599c29b0a588a704e23dca05b`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `esp`. Count 1 static access. Sites: `mov` at 0x175138.
  Branch (`je` at 0x175141): target 0x175144 outside this view. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  No reached exit is modeled.
  Boundary (`sti` at 0x175143): instruction end 0x175144. The native CFG supplies no local successor there.
  Immediate (`and` at 0x17513c): `0x200`.
  Tool warning: `0x175143 sti: unsupported effects invalidated`.

- `_intLock`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x175138`.
  Observation view: 0x175138 to 0x175148 (exclusive), 16 bytes. Native inferred symbol window.
  Native nlist window: 0x175138 to 0x175148 (exclusive), 16 bytes. Observation grade: established. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 115, body file offset 0x6d158, view SHA256 `109f7709eb74ab7db90509c77d55fcdccb007a0170b9a42b5fec981dd07d9985`.
  Four-byte load, pattern 1: target `unknown`, raw base `esp`. Count 1 static access. Sites: `pop` at 0x175139.
  At the exit (`ret` at 0x175140), eax=`unknown`.
  Immediate (`and` at 0x17513a): `0x200`.
  Tool warning: `0x175138 pushfd: unsupported effects invalidated`.
  Tool warning: `0x17513f cli: unsupported effects invalidated`.
  Decoded records outside known local edges: 0x175141 to 0x175147, ending at 0x175148 (exclusive). Count 7.

- `_intUnlock`, `6.5.6/usr/cpu/firmware/mmscfw.bin`, `0x175148`.
  Observation view: 0x175148 to 0x175154 (exclusive), 12 bytes. Native inferred symbol window.
  Native nlist window: 0x175148 to 0x175154 (exclusive), 12 bytes. Observation grade: unknown. Role: unknown.
  Carrier: 1023060 bytes, SHA256 `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
  Nlist symbol index 118, body file offset 0x6d168, view SHA256 `ebe59457ced429b0d530380db05285d03e13df6599c29b0a588a704e23dca05b`.
  Four-byte load, pattern 1: target `sp +0x4`, raw base `esp`. Count 1 static access. Sites: `mov` at 0x175148.
  Branch (`je` at 0x175151): target 0x175154 outside this view. Taken when `Z == 1`. Falls through when `not (Z == 1)`.
  No reached exit is modeled.
  Boundary (`sti` at 0x175153): instruction end 0x175154. The native CFG supplies no local successor there.
  Immediate (`and` at 0x17514c): `0x200`.
  Tool warning: `0x175153 sti: unsupported effects invalidated`.
