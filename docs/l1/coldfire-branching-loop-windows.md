# ColdFire branching loop windows

Each entry identifies its exact carrier, package, payload and selected byte position.
Addresses use the payload load plus byte offset. Entry shapes, contained branches,
returns and bounded contexts support inferred analysis spans. They do not establish
complete function extents, additional unique functions, physical banks or runtime
selection. Hardware roles and pointee meanings remain unknown.

Decoding uses the big-endian M68K 68040 common subset. Package metadata supplies the
ColdFire classification and load, without independently identifying fitted silicon.
Unsupported effects retain their printed unknown values or effective addresses. Static
groups describe instruction shapes, not equal values from separate loads or one
execution path across branch arms.

The six call slots are candidate locations under an assumed m68k-sysv convention. Actual
arity and custom conventions remain unknown. A bare general-register name in a value
expression denotes its value at entry.

The [complete facts](disasm/coldfire-variants/branching-loop-windows-facts.txt) retain warnings and omitted frame accesses.

- `0xffe8aec8`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe8aec8 to 0xffe8affc (exclusive), 308 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `0b5e3c1b9ff3ed82651a00c28dd64c97a64509d489580ffeabb609023162329f`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe8aed0).
  Call (`jsr` at 0xffe8aed4): (unknown callee), target 0xffe4a13e, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000048c`, including `movea.l` at 0xffe8aee0, `movea.l` at 0xffe8af06.
  Byte load: target `[0x3000048c] +0x1c9` (`move.b` at 0xffe8aee4).
  Branch (`beq.b` at 0xffe8aef2): taken to 0xffe8af0c when `unknown == unknown`, falls through to 0xffe8aef4 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe8aef6): taken to 0xffe8af00 when `unknown == 0x0`, falls through to 0xffe8aef8 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe8aefe): target 0xffe8af06.
  Byte store: target `[0x3000048c] +0x1c9`, value `unknown` (`move.b` at 0xffe8af08).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe8af10).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe8af16).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe8af1c).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe8af22).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe8af40).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe8af4a).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe8af4c).
  Branch (`beq.b` at 0xffe8af54): taken to 0xffe8af9a when `unknown == 0x0`, falls through to 0xffe8af56 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe8af72, `move.b` at 0xffe8af7c, `move.b` at 0xffe8af8e.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe8af74, `move.b` at 0xffe8afc4.
  Branch (`bge.b` at 0xffe8af92): taken to 0xffe8af56 when `ge flags condition (operands unknown)`, falls through to 0xffe8af94 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe8af94, `move.l` at 0xffe8afe6.
  Branch (`bgt.b` at 0xffe8af98): taken to 0xffe8af4a when `gt flags condition (operands unknown)`, falls through to 0xffe8af9a when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe8afa2): taken to 0xffe8afe8 when `Z == 1`, falls through to 0xffe8afa4 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe8afe4): taken to 0xffe8afa6 when `gt flags condition (operands unknown)`, falls through to 0xffe8afe6 when `not (gt flags condition (operands unknown))`.
  At the return (`rts` at 0xffe8affa), d0 is `0x1`.

- `0xffe72930`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.20:l1#0`, version `1.20.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72930 to 0xffe72a64 (exclusive), 308 bytes.
  Carrier: 2752144 bytes, SHA256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`.
  Package: carrier offset 0x0, 982548 bytes, SHA256 `422d287101c8919f17eaf903e9e69401f30ee481ce43e445290cc1007ba9b2f9`.
  Payload: carrier offset 0x40, 982484 bytes, SHA256 `4cbaeb5a69e89e79595fefde6bdf506b2a529c180deb9bfb01cad7d92a2287a3`.
  Read window: SHA256 `25f0b77517d5ed16d6a6a019c5b8137613e7c6df62b32b226640150fb66f9a4d`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe72938).
  Call (`jsr` at 0xffe7293c): (unknown callee), target 0xffe423fe, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000083c`, including `movea.l` at 0xffe72948, `movea.l` at 0xffe7296e.
  Byte load: target `[0x3000083c] +0x1c9` (`move.b` at 0xffe7294c).
  Branch (`beq.b` at 0xffe7295a): taken to 0xffe72974 when `unknown == unknown`, falls through to 0xffe7295c when `unknown != unknown`.
  Branch (`beq.b` at 0xffe7295e): taken to 0xffe72968 when `unknown == 0x0`, falls through to 0xffe72960 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe72966): target 0xffe7296e.
  Byte store: target `[0x3000083c] +0x1c9`, value `unknown` (`move.b` at 0xffe72970).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe72978).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe7297e).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe72984).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe7298a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe729a8).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe729b2).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe729b4).
  Branch (`beq.b` at 0xffe729bc): taken to 0xffe72a02 when `unknown == 0x0`, falls through to 0xffe729be when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe729da, `move.b` at 0xffe729e4, `move.b` at 0xffe729f6.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe729dc, `move.b` at 0xffe72a2c.
  Branch (`bge.b` at 0xffe729fa): taken to 0xffe729be when `ge flags condition (operands unknown)`, falls through to 0xffe729fc when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe729fc, `move.l` at 0xffe72a4e.
  Branch (`bgt.b` at 0xffe72a00): taken to 0xffe729b2 when `gt flags condition (operands unknown)`, falls through to 0xffe72a02 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe72a0a): taken to 0xffe72a50 when `Z == 1`, falls through to 0xffe72a0c when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe72a4c): taken to 0xffe72a0e when `gt flags condition (operands unknown)`, falls through to 0xffe72a4e when `not (gt flags condition (operands unknown))`.
  The exit (`rts` at 0xffe72a62) reports d0 = `0x1`.

- `0xffe9a7b0`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.20:l1#1`, version `1.20.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9a7b0 to 0xffe9a8e4 (exclusive), 308 bytes.
  Carrier: 2752144 bytes, SHA256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`.
  Package: carrier offset 0xefe14, 1769596 bytes, SHA256 `fa86b77f661d6f17243158a26aadf41a842ebd5e89248c73423e05c16e945ce9`.
  Payload: carrier offset 0xefe40, 1769552 bytes, SHA256 `ae841c9d40a0f5bfa1224e27e43c002300ed1704df032e3a97db6bd1a77f1c28`.
  Read window: SHA256 `956749945bb0b667bd021f595aa8e0e34d93865786e59038c52f44d607039fe1`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9a7b8).
  Call (`jsr` at 0xffe9a7bc): (unknown callee), target 0xffe4c40a, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000974`, including `movea.l` at 0xffe9a7c8, `movea.l` at 0xffe9a7ee.
  Byte load: target `[0x30000974] +0x1c9` (`move.b` at 0xffe9a7cc).
  Branch (`beq.b` at 0xffe9a7da): taken to 0xffe9a7f4 when `unknown == unknown`, falls through to 0xffe9a7dc when `unknown != unknown`.
  Branch (`beq.b` at 0xffe9a7de): taken to 0xffe9a7e8 when `unknown == 0x0`, falls through to 0xffe9a7e0 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe9a7e6): target 0xffe9a7ee.
  Byte store: target `[0x30000974] +0x1c9`, value `unknown` (`move.b` at 0xffe9a7f0).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe9a7f8).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe9a7fe).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe9a804).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe9a80a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe9a828).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe9a832).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe9a834).
  Branch (`beq.b` at 0xffe9a83c): taken to 0xffe9a882 when `unknown == 0x0`, falls through to 0xffe9a83e when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe9a85a, `move.b` at 0xffe9a864, `move.b` at 0xffe9a876.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe9a85c, `move.b` at 0xffe9a8ac.
  Branch (`bge.b` at 0xffe9a87a): taken to 0xffe9a83e when `ge flags condition (operands unknown)`, falls through to 0xffe9a87c when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe9a87c, `move.l` at 0xffe9a8ce.
  Branch (`bgt.b` at 0xffe9a880): taken to 0xffe9a832 when `gt flags condition (operands unknown)`, falls through to 0xffe9a882 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe9a88a): taken to 0xffe9a8d0 when `Z == 1`, falls through to 0xffe9a88c when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe9a8cc): taken to 0xffe9a88e when `gt flags condition (operands unknown)`, falls through to 0xffe9a8ce when `not (gt flags condition (operands unknown))`.
  Recorded d0 at the return is `0x1` (`rts` at 0xffe9a8e2).

- `0xffe72944`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#0`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe72944 to 0xffe72a78 (exclusive), 308 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0x0, 981456 bytes, SHA256 `9a9930f67c71e90a89751858eb80145e5c94bfcf4a25ff050943034f16d97d34`.
  Payload: carrier offset 0x40, 981392 bytes, SHA256 `2ee201adbe9fde941fa147cf8c1b5cc3d3b32026f9dfa108e1cde0f06fd88a2b`.
  Read window: SHA256 `be7077c056417a0dd7d30ba588ea29fedb44f24da49567bab13de72d3cdb39f6`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe7294c).
  Call (`jsr` at 0xffe72950): (unknown callee), target 0xffe42716, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000324`, including `movea.l` at 0xffe7295c, `movea.l` at 0xffe72982.
  Byte load: target `[0x30000324] +0x1c9` (`move.b` at 0xffe72960).
  Branch (`beq.b` at 0xffe7296e): taken to 0xffe72988 when `unknown == unknown`, falls through to 0xffe72970 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe72972): taken to 0xffe7297c when `unknown == 0x0`, falls through to 0xffe72974 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe7297a): target 0xffe72982.
  Byte store: target `[0x30000324] +0x1c9`, value `unknown` (`move.b` at 0xffe72984).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe7298c).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe72992).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe72998).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe7299e).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe729bc).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe729c6).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe729c8).
  Branch (`beq.b` at 0xffe729d0): taken to 0xffe72a16 when `unknown == 0x0`, falls through to 0xffe729d2 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe729ee, `move.b` at 0xffe729f8, `move.b` at 0xffe72a0a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe729f0, `move.b` at 0xffe72a40.
  Branch (`bge.b` at 0xffe72a0e): taken to 0xffe729d2 when `ge flags condition (operands unknown)`, falls through to 0xffe72a10 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe72a10, `move.l` at 0xffe72a62.
  Branch (`bgt.b` at 0xffe72a14): taken to 0xffe729c6 when `gt flags condition (operands unknown)`, falls through to 0xffe72a16 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe72a1e): taken to 0xffe72a64 when `Z == 1`, falls through to 0xffe72a20 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe72a60): taken to 0xffe72a22 when `gt flags condition (operands unknown)`, falls through to 0xffe72a62 when `not (gt flags condition (operands unknown))`.
  `rts` at 0xffe72a76 has recorded d0 = `0x1`.

- `0xffe9df94`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#1`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9df94 to 0xffe9e0c8 (exclusive), 308 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0xef9d0, 1784816 bytes, SHA256 `d597012cce4081272d0987ef729afeb15d40a0de6590702331370b07e76dcfae`.
  Payload: carrier offset 0xef9fc, 1784772 bytes, SHA256 `c960b88c14cba5aaca4d5efed9325e5830948f33cb623bf8dbc9887a4b1a8c26`.
  Read window: SHA256 `915a72e025c4c8115e5b18f87d2baa6f6f3b43f802b57ca96868b570e3baf1ee`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9df9c).
  Call (`jsr` at 0xffe9dfa0): (unknown callee), target 0xffe4d2ba, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300004f8`, including `movea.l` at 0xffe9dfac, `movea.l` at 0xffe9dfd2.
  Byte load: target `[0x300004f8] +0x1c9` (`move.b` at 0xffe9dfb0).
  Branch (`beq.b` at 0xffe9dfbe): taken to 0xffe9dfd8 when `unknown == unknown`, falls through to 0xffe9dfc0 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe9dfc2): taken to 0xffe9dfcc when `unknown == 0x0`, falls through to 0xffe9dfc4 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe9dfca): target 0xffe9dfd2.
  Byte store: target `[0x300004f8] +0x1c9`, value `unknown` (`move.b` at 0xffe9dfd4).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe9dfdc).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe9dfe2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe9dfe8).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe9dfee).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe9e00c).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe9e016).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe9e018).
  Branch (`beq.b` at 0xffe9e020): taken to 0xffe9e066 when `unknown == 0x0`, falls through to 0xffe9e022 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe9e03e, `move.b` at 0xffe9e048, `move.b` at 0xffe9e05a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe9e040, `move.b` at 0xffe9e090.
  Branch (`bge.b` at 0xffe9e05e): taken to 0xffe9e022 when `ge flags condition (operands unknown)`, falls through to 0xffe9e060 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe9e060, `move.l` at 0xffe9e0b2.
  Branch (`bgt.b` at 0xffe9e064): taken to 0xffe9e016 when `gt flags condition (operands unknown)`, falls through to 0xffe9e066 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe9e06e): taken to 0xffe9e0b4 when `Z == 1`, falls through to 0xffe9e070 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe9e0b0): taken to 0xffe9e072 when `gt flags condition (operands unknown)`, falls through to 0xffe9e0b2 when `not (gt flags condition (operands unknown))`.
  At the return (`rts` at 0xffe9e0c6), d0 is `0x1`.

- `0xffe71bdc`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#0`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe71bdc to 0xffe71d10 (exclusive), 308 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0x0, 977968 bytes, SHA256 `5660c129f64680c9639f50d5f3d7c61428912f2270dde66185a7acccdcf64755`.
  Payload: carrier offset 0x48, 977896 bytes, SHA256 `0fe5e128ec92fb73681f4b9c7e148b1a68de5cf5199c150251c4703beef4e13f`.
  Read window: SHA256 `219f1cebeb3302e8694b210fe81fa634d43b40bbcc8580d281fd12f0fd60ba1e`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe71be4).
  Call (`jsr` at 0xffe71be8): (unknown callee), target 0xffe4289a, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300008b4`, including `movea.l` at 0xffe71bf4, `movea.l` at 0xffe71c1a.
  Byte load: target `[0x300008b4] +0x1c9` (`move.b` at 0xffe71bf8).
  Branch (`beq.b` at 0xffe71c06): taken to 0xffe71c20 when `unknown == unknown`, falls through to 0xffe71c08 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe71c0a): taken to 0xffe71c14 when `unknown == 0x0`, falls through to 0xffe71c0c when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe71c12): target 0xffe71c1a.
  Byte store: target `[0x300008b4] +0x1c9`, value `unknown` (`move.b` at 0xffe71c1c).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe71c24).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe71c2a).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe71c30).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe71c36).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe71c54).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe71c5e).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe71c60).
  Branch (`beq.b` at 0xffe71c68): taken to 0xffe71cae when `unknown == 0x0`, falls through to 0xffe71c6a when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe71c86, `move.b` at 0xffe71c90, `move.b` at 0xffe71ca2.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe71c88, `move.b` at 0xffe71cd8.
  Branch (`bge.b` at 0xffe71ca6): taken to 0xffe71c6a when `ge flags condition (operands unknown)`, falls through to 0xffe71ca8 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe71ca8, `move.l` at 0xffe71cfa.
  Branch (`bgt.b` at 0xffe71cac): taken to 0xffe71c5e when `gt flags condition (operands unknown)`, falls through to 0xffe71cae when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe71cb6): taken to 0xffe71cfc when `Z == 1`, falls through to 0xffe71cb8 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe71cf8): taken to 0xffe71cba when `gt flags condition (operands unknown)`, falls through to 0xffe71cfa when `not (gt flags condition (operands unknown))`.
  The exit (`rts` at 0xffe71d0e) reports d0 = `0x1`.

- `0xffe7097c`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#1`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe7097c to 0xffe70ab0 (exclusive), 308 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0xeec30, 973412 bytes, SHA256 `8ce4c2cf2d93012d81b4ba6bc7a1deb66de83a32c5cb3052715b3a5d96fb7971`.
  Payload: carrier offset 0xeec5c, 973368 bytes, SHA256 `1640737f85acb71067aab42164e36704e6f1510cd8346bbdca7526e8a63c6b12`.
  Read window: SHA256 `b71f8642db9748078e54c607fbe737bc527e9c3cecaa0307555b260008ba01d7`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe70984).
  Call (`jsr` at 0xffe70988): (unknown callee), target 0xffe4289a, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000094c`, including `movea.l` at 0xffe70994, `movea.l` at 0xffe709ba.
  Byte load: target `[0x3000094c] +0x1c9` (`move.b` at 0xffe70998).
  Branch (`beq.b` at 0xffe709a6): taken to 0xffe709c0 when `unknown == unknown`, falls through to 0xffe709a8 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe709aa): taken to 0xffe709b4 when `unknown == 0x0`, falls through to 0xffe709ac when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe709b2): target 0xffe709ba.
  Byte store: target `[0x3000094c] +0x1c9`, value `unknown` (`move.b` at 0xffe709bc).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe709c4).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe709ca).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe709d0).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe709d6).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe709f4).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe709fe).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe70a00).
  Branch (`beq.b` at 0xffe70a08): taken to 0xffe70a4e when `unknown == 0x0`, falls through to 0xffe70a0a when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe70a26, `move.b` at 0xffe70a30, `move.b` at 0xffe70a42.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe70a28, `move.b` at 0xffe70a78.
  Branch (`bge.b` at 0xffe70a46): taken to 0xffe70a0a when `ge flags condition (operands unknown)`, falls through to 0xffe70a48 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe70a48, `move.l` at 0xffe70a9a.
  Branch (`bgt.b` at 0xffe70a4c): taken to 0xffe709fe when `gt flags condition (operands unknown)`, falls through to 0xffe70a4e when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe70a56): taken to 0xffe70a9c when `Z == 1`, falls through to 0xffe70a58 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe70a98): taken to 0xffe70a5a when `gt flags condition (operands unknown)`, falls through to 0xffe70a9a when `not (gt flags condition (operands unknown))`.
  Recorded d0 at the return is `0x1` (`rts` at 0xffe70aae).

- `0xffe9c758`, `6.5.22/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.22:l1#2`, version `1.24.8`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe9c758 to 0xffe9c88c (exclusive), 308 bytes.
  Carrier: 3723916 bytes, SHA256 `5b90997945dabdc099162d5237e302472f5c3c80a808a1de5abb246c290a963b`.
  Package: carrier offset 0x1dc694, 1772536 bytes, SHA256 `6d06a281a09291d82228d78cfc1c0f2419eff91a940d388bf6bc3b406fbf32ff`.
  Payload: carrier offset 0x1dc6c0, 1772492 bytes, SHA256 `953894c5c4e86f03e78ada263bebd12e0e7af316fe0733a8bc606f2d94ce66da`.
  Read window: SHA256 `25c300a36752d788f479b741569ce5257a45570d0a5b54a9c26254301e348409`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe9c760).
  Call (`jsr` at 0xffe9c764): (unknown callee), target 0xffe4ba8a, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000b08`, including `movea.l` at 0xffe9c770, `movea.l` at 0xffe9c796.
  Byte load: target `[0x30000b08] +0x1c9` (`move.b` at 0xffe9c774).
  Branch (`beq.b` at 0xffe9c782): taken to 0xffe9c79c when `unknown == unknown`, falls through to 0xffe9c784 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe9c786): taken to 0xffe9c790 when `unknown == 0x0`, falls through to 0xffe9c788 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe9c78e): target 0xffe9c796.
  Byte store: target `[0x30000b08] +0x1c9`, value `unknown` (`move.b` at 0xffe9c798).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe9c7a0).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe9c7a6).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe9c7ac).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe9c7b2).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe9c7d0).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe9c7da).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe9c7dc).
  Branch (`beq.b` at 0xffe9c7e4): taken to 0xffe9c82a when `unknown == 0x0`, falls through to 0xffe9c7e6 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe9c802, `move.b` at 0xffe9c80c, `move.b` at 0xffe9c81e.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe9c804, `move.b` at 0xffe9c854.
  Branch (`bge.b` at 0xffe9c822): taken to 0xffe9c7e6 when `ge flags condition (operands unknown)`, falls through to 0xffe9c824 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe9c824, `move.l` at 0xffe9c876.
  Branch (`bgt.b` at 0xffe9c828): taken to 0xffe9c7da when `gt flags condition (operands unknown)`, falls through to 0xffe9c82a when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe9c832): taken to 0xffe9c878 when `Z == 1`, falls through to 0xffe9c834 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe9c874): taken to 0xffe9c836 when `gt flags condition (operands unknown)`, falls through to 0xffe9c876 when `not (gt flags condition (operands unknown))`.
  `rts` at 0xffe9c88a has recorded d0 = `0x1`.

- `0xffe725a4`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#0`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe725a4 to 0xffe726d8 (exclusive), 308 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x0, 982480 bytes, SHA256 `c391b1c9925c25fa5040d54e9adc51c8cb2ed9b5626a8734938bbc19267e4f08`.
  Payload: carrier offset 0x68, 982376 bytes, SHA256 `3c8a7aa93eb38502289cf0c2921ea91e6800228005bd8abd67425ce615f7b364`.
  Read window: SHA256 `dfd1e3f4c27bb63371955f12264b8a561c20e0e4f3064f8288b979146f936030`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe725ac).
  Call (`jsr` at 0xffe725b0): (unknown callee), target 0xffe412ee, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300002bc`, including `movea.l` at 0xffe725bc, `movea.l` at 0xffe725e2.
  Byte load: target `[0x300002bc] +0x1c9` (`move.b` at 0xffe725c0).
  Branch (`beq.b` at 0xffe725ce): taken to 0xffe725e8 when `unknown == unknown`, falls through to 0xffe725d0 when `unknown != unknown`.
  Branch (`beq.b` at 0xffe725d2): taken to 0xffe725dc when `unknown == 0x0`, falls through to 0xffe725d4 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe725da): target 0xffe725e2.
  Byte store: target `[0x300002bc] +0x1c9`, value `unknown` (`move.b` at 0xffe725e4).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe725ec).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe725f2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe725f8).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe725fe).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe7261c).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe72626).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe72628).
  Branch (`beq.b` at 0xffe72630): taken to 0xffe72676 when `unknown == 0x0`, falls through to 0xffe72632 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe7264e, `move.b` at 0xffe72658, `move.b` at 0xffe7266a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe72650, `move.b` at 0xffe726a0.
  Branch (`bge.b` at 0xffe7266e): taken to 0xffe72632 when `ge flags condition (operands unknown)`, falls through to 0xffe72670 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe72670, `move.l` at 0xffe726c2.
  Branch (`bgt.b` at 0xffe72674): taken to 0xffe72626 when `gt flags condition (operands unknown)`, falls through to 0xffe72676 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe7267e): taken to 0xffe726c4 when `Z == 1`, falls through to 0xffe72680 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe726c0): taken to 0xffe72682 when `gt flags condition (operands unknown)`, falls through to 0xffe726c2 when `not (gt flags condition (operands unknown))`.
  At the return (`rts` at 0xffe726d6), d0 is `0x1`.

- `0xffe95600`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.30:l1#2`, version `1.44.0`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffe95600 to 0xffe95734 (exclusive), 308 bytes.
  Carrier: 15988636 bytes, SHA256 `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
  Package: carrier offset 0x1dfb5c, 1755564 bytes, SHA256 `22e340c4657850647ce06e36173f6e671359ed8d26a6468c3c0ef6bde644540c`.
  Payload: carrier offset 0x1dfb88, 1755520 bytes, SHA256 `ac81f501f01c6789e756ab1f5ec69fadb1e4c04225511d541340831686e04e70`.
  Read window: SHA256 `6822ec8ac2c159a315a9aa03d7b5fb81dc9fdcb0290dea99b88e97d05abbd8ee`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffe95608).
  Call (`jsr` at 0xffe9560c): (unknown callee), target 0xffe4735e, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x300008bc`, including `movea.l` at 0xffe95618, `movea.l` at 0xffe9563e.
  Byte load: target `[0x300008bc] +0x1c9` (`move.b` at 0xffe9561c).
  Branch (`beq.b` at 0xffe9562a): taken to 0xffe95644 when `unknown == unknown`, falls through to 0xffe9562c when `unknown != unknown`.
  Branch (`beq.b` at 0xffe9562e): taken to 0xffe95638 when `unknown == 0x0`, falls through to 0xffe95630 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffe95636): target 0xffe9563e.
  Byte store: target `[0x300008bc] +0x1c9`, value `unknown` (`move.b` at 0xffe95640).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffe95648).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffe9564e).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffe95654).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffe9565a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffe95678).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffe95682).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffe95684).
  Branch (`beq.b` at 0xffe9568c): taken to 0xffe956d2 when `unknown == 0x0`, falls through to 0xffe9568e when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffe956aa, `move.b` at 0xffe956b4, `move.b` at 0xffe956c6.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffe956ac, `move.b` at 0xffe956fc.
  Branch (`bge.b` at 0xffe956ca): taken to 0xffe9568e when `ge flags condition (operands unknown)`, falls through to 0xffe956cc when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffe956cc, `move.l` at 0xffe9571e.
  Branch (`bgt.b` at 0xffe956d0): taken to 0xffe95682 when `gt flags condition (operands unknown)`, falls through to 0xffe956d2 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffe956da): taken to 0xffe95720 when `Z == 1`, falls through to 0xffe956dc when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffe9571c): taken to 0xffe956de when `gt flags condition (operands unknown)`, falls through to 0xffe9571e when `not (gt flags condition (operands unknown))`.
  The exit (`rts` at 0xffe95732) reports d0 = `0x1`.

- `0xffee1454`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#0`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee1454 to 0xffee1588 (exclusive), 308 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0x0, 977052 bytes, SHA256 `389a486d2213168fcde72fafaa1bd9efe5bbbde06e52e1f023f5dd76900537c5`.
  Payload: carrier offset 0x40, 976988 bytes, SHA256 `9b636fd9c1b5fe130de21c2db5a40eb3fcd6fc0452887f2ddc8a015d9fad521d`.
  Read window: SHA256 `f69ffd94a7ff7457b3eb61161354e6d803695d9fa1c5162e1493a15cf677a635`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffee145c).
  Call (`jsr` at 0xffee1460): (unknown callee), target 0xffeb1ba2, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000370`, including `movea.l` at 0xffee146c, `movea.l` at 0xffee1492.
  Byte load: target `[0x30000370] +0x1c9` (`move.b` at 0xffee1470).
  Branch (`beq.b` at 0xffee147e): taken to 0xffee1498 when `unknown == unknown`, falls through to 0xffee1480 when `unknown != unknown`.
  Branch (`beq.b` at 0xffee1482): taken to 0xffee148c when `unknown == 0x0`, falls through to 0xffee1484 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffee148a): target 0xffee1492.
  Byte store: target `[0x30000370] +0x1c9`, value `unknown` (`move.b` at 0xffee1494).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffee149c).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffee14a2).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffee14a8).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffee14ae).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffee14cc).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffee14d6).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffee14d8).
  Branch (`beq.b` at 0xffee14e0): taken to 0xffee1526 when `unknown == 0x0`, falls through to 0xffee14e2 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffee14fe, `move.b` at 0xffee1508, `move.b` at 0xffee151a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffee1500, `move.b` at 0xffee1550.
  Branch (`bge.b` at 0xffee151e): taken to 0xffee14e2 when `ge flags condition (operands unknown)`, falls through to 0xffee1520 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffee1520, `move.l` at 0xffee1572.
  Branch (`bgt.b` at 0xffee1524): taken to 0xffee14d6 when `gt flags condition (operands unknown)`, falls through to 0xffee1526 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffee152e): taken to 0xffee1574 when `Z == 1`, falls through to 0xffee1530 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffee1570): taken to 0xffee1532 when `gt flags condition (operands unknown)`, falls through to 0xffee1572 when `not (gt flags condition (operands unknown))`.
  Recorded d0 at the return is `0x1` (`rts` at 0xffee1586).

- `0xfff7aec8`, `6.5.19/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.19:l1#1`, version `1.18.3`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff7aec8 to 0xfff7affc (exclusive), 308 bytes.
  Carrier: 2668016 bytes, SHA256 `9b9fbe440979d0082cc1c41dfa89fcf57136493f359b55aedcded65f2827d562`.
  Package: carrier offset 0xee89c, 1690964 bytes, SHA256 `f3bcc8c59d2bd3da42046f540b7a93125398e6bf126e69a1c670a45d87ce03f7`.
  Payload: carrier offset 0xee8c8, 1690920 bytes, SHA256 `5ffb4512edcd71b473fcfff88b4baba1656e2bb79ed0808fa9105ba4f7648f1a`.
  Read window: SHA256 `2e4f379e1d929c784d3972245561934bcb635dffb2856cf33559b85037b9388b`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff7aed0).
  Call (`jsr` at 0xfff7aed4): (unknown callee), target 0xfff3a13e, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000048c`, including `movea.l` at 0xfff7aee0, `movea.l` at 0xfff7af06.
  Byte load: target `[0x3000048c] +0x1c9` (`move.b` at 0xfff7aee4).
  Branch (`beq.b` at 0xfff7aef2): taken to 0xfff7af0c when `unknown == unknown`, falls through to 0xfff7aef4 when `unknown != unknown`.
  Branch (`beq.b` at 0xfff7aef6): taken to 0xfff7af00 when `unknown == 0x0`, falls through to 0xfff7aef8 when `unknown != 0x0`.
  Jump (`bra.b` at 0xfff7aefe): target 0xfff7af06.
  Byte store: target `[0x3000048c] +0x1c9`, value `unknown` (`move.b` at 0xfff7af08).
  Four-byte load: target `sp +0x8` (`move.l` at 0xfff7af10).
  Four-byte load: target `sp +0xc` (`move.l` at 0xfff7af16).
  Four-byte load: target `sp +0x14` (`move.l` at 0xfff7af1c).
  Four-byte load: target `sp +0x10` (`move.l` at 0xfff7af22).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xfff7af40).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xfff7af4a).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xfff7af4c).
  Branch (`beq.b` at 0xfff7af54): taken to 0xfff7af9a when `unknown == 0x0`, falls through to 0xfff7af56 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xfff7af72, `move.b` at 0xfff7af7c, `move.b` at 0xfff7af8e.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xfff7af74, `move.b` at 0xfff7afc4.
  Branch (`bge.b` at 0xfff7af92): taken to 0xfff7af56 when `ge flags condition (operands unknown)`, falls through to 0xfff7af94 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xfff7af94, `move.l` at 0xfff7afe6.
  Branch (`bgt.b` at 0xfff7af98): taken to 0xfff7af4a when `gt flags condition (operands unknown)`, falls through to 0xfff7af9a when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xfff7afa2): taken to 0xfff7afe8 when `Z == 1`, falls through to 0xfff7afa4 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xfff7afe4): taken to 0xfff7afa6 when `gt flags condition (operands unknown)`, falls through to 0xfff7afe6 when `not (gt flags condition (operands unknown))`.
  `rts` at 0xfff7affa has recorded d0 = `0x1`.

- `0xffee2930`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.20:l1#0`, version `1.20.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee2930 to 0xffee2a64 (exclusive), 308 bytes.
  Carrier: 2752144 bytes, SHA256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`.
  Package: carrier offset 0x0, 982548 bytes, SHA256 `422d287101c8919f17eaf903e9e69401f30ee481ce43e445290cc1007ba9b2f9`.
  Payload: carrier offset 0x40, 982484 bytes, SHA256 `4cbaeb5a69e89e79595fefde6bdf506b2a529c180deb9bfb01cad7d92a2287a3`.
  Read window: SHA256 `9ebcbd4daf556ec0c15308affbdd450a1a32fcb17dc5757fab4c2464a387c37c`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffee2938).
  Call (`jsr` at 0xffee293c): (unknown callee), target 0xffeb23fe, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x3000083c`, including `movea.l` at 0xffee2948, `movea.l` at 0xffee296e.
  Byte load: target `[0x3000083c] +0x1c9` (`move.b` at 0xffee294c).
  Branch (`beq.b` at 0xffee295a): taken to 0xffee2974 when `unknown == unknown`, falls through to 0xffee295c when `unknown != unknown`.
  Branch (`beq.b` at 0xffee295e): taken to 0xffee2968 when `unknown == 0x0`, falls through to 0xffee2960 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffee2966): target 0xffee296e.
  Byte store: target `[0x3000083c] +0x1c9`, value `unknown` (`move.b` at 0xffee2970).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffee2978).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffee297e).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffee2984).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffee298a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffee29a8).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffee29b2).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffee29b4).
  Branch (`beq.b` at 0xffee29bc): taken to 0xffee2a02 when `unknown == 0x0`, falls through to 0xffee29be when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffee29da, `move.b` at 0xffee29e4, `move.b` at 0xffee29f6.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffee29dc, `move.b` at 0xffee2a2c.
  Branch (`bge.b` at 0xffee29fa): taken to 0xffee29be when `ge flags condition (operands unknown)`, falls through to 0xffee29fc when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffee29fc, `move.l` at 0xffee2a4e.
  Branch (`bgt.b` at 0xffee2a00): taken to 0xffee29b2 when `gt flags condition (operands unknown)`, falls through to 0xffee2a02 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffee2a0a): taken to 0xffee2a50 when `Z == 1`, falls through to 0xffee2a0c when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffee2a4c): taken to 0xffee2a0e when `gt flags condition (operands unknown)`, falls through to 0xffee2a4e when `not (gt flags condition (operands unknown))`.
  At the return (`rts` at 0xffee2a62), d0 is `0x1`.

- `0xfff8a7b0`, `6.5.20/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.20:l1#1`, version `1.20.6`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xfff8a7b0 to 0xfff8a8e4 (exclusive), 308 bytes.
  Carrier: 2752144 bytes, SHA256 `adc39a287c525f1d6ac9bb8812671546f26c9029809d3bd487f5f8ab7318f94d`.
  Package: carrier offset 0xefe14, 1769596 bytes, SHA256 `fa86b77f661d6f17243158a26aadf41a842ebd5e89248c73423e05c16e945ce9`.
  Payload: carrier offset 0xefe40, 1769552 bytes, SHA256 `ae841c9d40a0f5bfa1224e27e43c002300ed1704df032e3a97db6bd1a77f1c28`.
  Read window: SHA256 `a5681065e067db6a8b3981c48a13a75c5e379a372a06b2529d4d160b8ac71fa9`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xfff8a7b8).
  Call (`jsr` at 0xfff8a7bc): (unknown callee), target 0xfff3c40a, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000974`, including `movea.l` at 0xfff8a7c8, `movea.l` at 0xfff8a7ee.
  Byte load: target `[0x30000974] +0x1c9` (`move.b` at 0xfff8a7cc).
  Branch (`beq.b` at 0xfff8a7da): taken to 0xfff8a7f4 when `unknown == unknown`, falls through to 0xfff8a7dc when `unknown != unknown`.
  Branch (`beq.b` at 0xfff8a7de): taken to 0xfff8a7e8 when `unknown == 0x0`, falls through to 0xfff8a7e0 when `unknown != 0x0`.
  Jump (`bra.b` at 0xfff8a7e6): target 0xfff8a7ee.
  Byte store: target `[0x30000974] +0x1c9`, value `unknown` (`move.b` at 0xfff8a7f0).
  Four-byte load: target `sp +0x8` (`move.l` at 0xfff8a7f8).
  Four-byte load: target `sp +0xc` (`move.l` at 0xfff8a7fe).
  Four-byte load: target `sp +0x14` (`move.l` at 0xfff8a804).
  Four-byte load: target `sp +0x10` (`move.l` at 0xfff8a80a).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xfff8a828).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xfff8a832).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xfff8a834).
  Branch (`beq.b` at 0xfff8a83c): taken to 0xfff8a882 when `unknown == 0x0`, falls through to 0xfff8a83e when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xfff8a85a, `move.b` at 0xfff8a864, `move.b` at 0xfff8a876.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xfff8a85c, `move.b` at 0xfff8a8ac.
  Branch (`bge.b` at 0xfff8a87a): taken to 0xfff8a83e when `ge flags condition (operands unknown)`, falls through to 0xfff8a87c when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xfff8a87c, `move.l` at 0xfff8a8ce.
  Branch (`bgt.b` at 0xfff8a880): taken to 0xfff8a832 when `gt flags condition (operands unknown)`, falls through to 0xfff8a882 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xfff8a88a): taken to 0xfff8a8d0 when `Z == 1`, falls through to 0xfff8a88c when `not (Z == 1)`.
  Branch (`bgt.b` at 0xfff8a8cc): taken to 0xfff8a88e when `gt flags condition (operands unknown)`, falls through to 0xfff8a8ce when `not (gt flags condition (operands unknown))`.
  The exit (`rts` at 0xfff8a8e2) reports d0 = `0x1`.

- `0xffee2944`, `6.5.21/usr/cpu/firmware/sysco/l1.bin`.
  Package `6.5.21:l1#0`, version `1.22.2`, load `0xffe00000`. Role: unknown.
  Inferred analysis span: 0xffee2944 to 0xffee2a78 (exclusive), 308 bytes.
  Carrier: 2766272 bytes, SHA256 `d5aaba306f71d9bfe0aa83d5029e14f2b9cf7485a4c0c689297fc4f71d136fbb`.
  Package: carrier offset 0x0, 981456 bytes, SHA256 `9a9930f67c71e90a89751858eb80145e5c94bfcf4a25ff050943034f16d97d34`.
  Payload: carrier offset 0x40, 981392 bytes, SHA256 `2ee201adbe9fde941fa147cf8c1b5cc3d3b32026f9dfa108e1cde0f06fd88a2b`.
  Read window: SHA256 `99ec26dc5ac23125b07571c6cb22dc62fe398c91fde11c1df1ccb7094510c719`.
  Four-byte load: target `sp +0x4` (`move.l` at 0xffee294c).
  Call (`jsr` at 0xffee2950): (unknown callee), target 0xffeb2716, stack arg0=`d6`, stack arg1=`d7`, stack arg2=`unknown`, stack arg3=`unknown`, stack arg4=`unknown`, stack arg5=`unknown`.
  Grouped accesses: 2 4-byte load records with target `abs 0x30000324`, including `movea.l` at 0xffee295c, `movea.l` at 0xffee2982.
  Byte load: target `[0x30000324] +0x1c9` (`move.b` at 0xffee2960).
  Branch (`beq.b` at 0xffee296e): taken to 0xffee2988 when `unknown == unknown`, falls through to 0xffee2970 when `unknown != unknown`.
  Branch (`beq.b` at 0xffee2972): taken to 0xffee297c when `unknown == 0x0`, falls through to 0xffee2974 when `unknown != 0x0`.
  Jump (`bra.b` at 0xffee297a): target 0xffee2982.
  Byte store: target `[0x30000324] +0x1c9`, value `unknown` (`move.b` at 0xffee2984).
  Four-byte load: target `sp +0x8` (`move.l` at 0xffee298c).
  Four-byte load: target `sp +0xc` (`move.l` at 0xffee2992).
  Four-byte load: target `sp +0x14` (`move.l` at 0xffee2998).
  Four-byte load: target `sp +0x10` (`move.l` at 0xffee299e).
  Four-byte load: target `abs 0x30000048` (`movea.l` at 0xffee29bc).
  Four-byte load: target `(unknown effective address)`, base register `a5` (`move.l` at 0xffee29c6).
  Four-byte load: target `(unknown effective address)`, base register `a2` (`move.l` at 0xffee29c8).
  Branch (`beq.b` at 0xffee29d0): taken to 0xffee2a16 when `unknown == 0x0`, falls through to 0xffee29d2 when `unknown != 0x0`.
  Grouped accesses: 6 1-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a0`, including `move.b` at 0xffee29ee, `move.b` at 0xffee29f8, `move.b` at 0xffee2a0a.
  Grouped accesses: 2 1-byte load records with target `(unknown effective address)`, base register `a0`, including `move.b` at 0xffee29f0, `move.b` at 0xffee2a40.
  Branch (`bge.b` at 0xffee2a0e): taken to 0xffee29d2 when `ge flags condition (operands unknown)`, falls through to 0xffee2a10 when `not (ge flags condition (operands unknown))`.
  Grouped accesses: 2 4-byte store records with target `(unknown effective address)`, reported value `unknown` in every record, base register `a1`, including `move.l` at 0xffee2a10, `move.l` at 0xffee2a62.
  Branch (`bgt.b` at 0xffee2a14): taken to 0xffee29c6 when `gt flags condition (operands unknown)`, falls through to 0xffee2a16 when `not (gt flags condition (operands unknown))`.
  Branch (`beq.b` at 0xffee2a1e): taken to 0xffee2a64 when `Z == 1`, falls through to 0xffee2a20 when `not (Z == 1)`.
  Branch (`bgt.b` at 0xffee2a60): taken to 0xffee2a22 when `gt flags condition (operands unknown)`, falls through to 0xffee2a62 when `not (gt flags condition (operands unknown))`.
  Recorded d0 at the return is `0x1` (`rts` at 0xffee2a76).
