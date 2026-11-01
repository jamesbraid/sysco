# How each ColdFire package turns the brick-type character into board code

Read from the IRIX 6.5.30 `l1.bin` (L1 1.44.0). The type character itself, and how it is chosen, are in [personalities.md](personalities.md). This note covers the next step: what the firmware does with the character once it is set. Package numbers and labels follow that note (0 `Base 1MB image`, 1 `Fuel/PE/O300 1MB image`, 2 `MIPS 2MB image`, 3 `Legacy 2MB image`, 5 `2MB image`). Addresses are in the first flash bank. Grades follow [AGENTS.md](../../AGENTS.md).

## Where the choice is made

Established (read in every package listed). Each package has one routine that takes a pointer to a RAM descriptor block and a type character, and calls one initialiser per supported type. Each initialiser fills the descriptor with that board's hooks. A character the routine does not list takes the same fallback path in all of them: log two messages at level 2 (the first names the type through the display-name routine, the second reads `Defaulting to 'unknown' brick type`), store `?` as the type through the type setter, then call the blank initialiser that fills the descriptor with empty hooks. The log source file literal is `brick_1m1.c` in package 1, `brick_base.c` in packages 0 and 2 and `brick_2m.c` in packages 3 and 5.

The routine that finishes the identity work stores the chosen character as persistent setting 9 and then calls the dispatcher with the character and a RAM descriptor block. Call sites (the address of the `jsr`): package 1 `0xffe557a0` with the block at `0x30001f20`; package 0 `0xffe55b50`, through a 24-byte forwarder at `0xffe79ab8`, with `0x30001e30`; package 2 `0xffe75fb4` with `0x3000281c`; package 3 `0xffe6b366` with `0x30002b90`; package 5 `0xffe7a1b8` with `0x30003404`. Each is preceded by a two-argument call whose first pushed argument is 9 and second the character (package 1 and 0 call the setting writer `setting_set`; the writers in packages 2, 3 and 5 were identified by that shape only).

| Package | Dispatcher | Type character to initialiser |
|---|---|---|
| 1 | `0xffe79968` | `%` to `0xffe44738`; `4` to `0xffe45bac`; `A` to `0xffe49820`; `?` straight to the blank initialiser `0xffe4a258`; anything else takes the fallback |
| 0 | `0xffe44394` | `C` to `0xffe45504`; `R` to `0xffe4a330`; `&`, `I`, `N`, `P`, `X`, `Y` to `0xffe4676c`; `G` to `0xffe57dd0`; `?` to the blank initialiser `0xffe49e9c`; anything else takes the fallback |
| 2 | `0xffea38f4` first, then `0xffe4a404` | `0xffea38f4`: `7` to `0xffe5f858`; `6` to `0xffe517c4`; `9` to `0xffe61bc0`; `%` to `0xffe4dfb0`; `A` to `0xffe68504`; any other type goes to `0xffe4a404`, whose table is package 0's (`C` `R` `&INPXY` `G`, same set, different targets) and whose fallback is the one above. After either, a common tail calls eight further initialisers |
| 3 | `0xffea8a2c` | `#` and `=` to `0xffe559c4`; `5` to `0xffe51ae0`; `O` to `0xffe53ec4`; `?` to the blank initialiser `0xffe6b6c4`; anything else takes the fallback. A tail calls `0xffe8a640` and `0xffe87b64` after the per-type call |
| 5 | `0xffeaa964` | the same five characters with the same structure: `#` and `=` to `0xffe5f028`; `5` to `0xffe58960`; `O` to `0xffe54a24`; blank initialiser through the `?` path |

How the table was read: package 0's dispatcher at `0xffe44394` subtracts `0x26` from the type, rejects a result above `0x33`, and jumps through a 52-entry table of 16-bit offsets at `0xffe443bc`. Entries for `&`, `I`, `N`, `P`, `X` and `Y` all land on one call of `0xffe4676c`, `C` on `0xffe45504`, `R` on `0xffe4a330`, `G` on `0xffe57dd0`, every other entry on the fallback. Package 2's table at `0xffe4a42c` has the same shape and the same four target groups. Packages 1, 3 and 5 use a compare chain of `add.l`/`beq` steps on the type instead of a table (package 1 against `0x25`, `0x34`, `0x3f`, `0x41` cumulatively; packages 3 and 5 against `0x3d`, `0x23`, `0x35`, `0x3f`, `0x4f`).

Consequences, all established by the reads above:

- Package 1's dispatcher has initialisers for three board types (`%`, `4`, `A`); its supported-brick string `%4A?` adds only `?`, which takes the blank initialiser. The Origin 300 board is `4`.
- Package 0's dispatcher has an initialiser for each of the ten characters of its supported-brick string `CIPXYN&?RG`: `C`, `R`, `G`, the six-character group, and `?` through the blank initialiser.
- Package 2's two dispatchers cover `C` `R` `&INPXY` `G` as in package 0 and add `6`, `7`, `9`, `%`, `A`. The characters `0`, `8`, `V`, `Q`, `E` that its supported-brick string also lists have no initialiser in either dispatcher and take the fallback. This is a read of the two dispatchers only; whether another routine handles them is not excluded.
- Packages 3 and 5 can initialise only four boards by type: the two-node I/O family (`#`, `=`), `5`, and `O`.

## The descriptor block each initialiser fills

Established for the first straight-line part of each initialiser (up to its first `rts`). The descriptor is addressed through argument 0; the offsets below are descriptor offsets. Only the fields that differ in role are listed.

| Offset | Content (package 1 `4`, `0xffe45bac`) | Same offset in package 0 `C` (`0xffe45504`) |
|---:|---|---|
| `0x04` | board device-registration routine (`0xffe45990`) | `0xffe453f8` |
| `0x0c` byte | `0x4d` | `0x4c` |
| `0x10` | 1 | 1 |
| `0x14` | `0x0b` | 1 |
| `0x18`, `0x1c`, `0x20` | `0xff` each | `0xff` each |
| `0x24` | `0xffe39bc8` | `0xffe39bc8` |
| `0x2c`, `0x30` | 2, 2 | 4, 4 (and `0x34`, `0x38` also 4) |
| `0x54` to `0x64` | `0x28`, `0x2e`, `0x31`, `0x31`, `0x25` | not written |
| `0x74` | 4 | 4 |
| `0x7c` | `0xffe45040` | `0xffe44530` |
| `0x98`, `0x9c` | `0xffe453e4`, `0xffe4552c` | `0xffe44bd4`, `0xffe44dfc` |
| `0xa0` | `0xffe45060` | `0xffe4458c` |
| `0xa4`, `0xa8` | `0xffe45084`, `0xffe450e0` | `0xffe445e8`, `0xffe4463c` |
| `0xac`, `0xb0` | `0xffe45714`, `0xffe45764` | `0xffe45034`, `0xffe451e8` |
| `0xb4`, `0xb8` | `0xb4` cleared | `0xffe4524c`, `0xffe44684` |
| `0xc0` | `0xffe45af0` | not written |

The two routines at `0xa4` and `0xa8` are the pair compared in [personality-base-package.md](personality-base-package.md) (set-and-clear and clear-and-set of one expander bit). The type letter at `0x0c` and several of the small integers differ per board; what each integer means is not known.

Descriptor initialisers of the other types, same read: package 1 `%` (`0xffe44738`) writes `0x04` `0xffe44544`, `0x10` 2, `0x28` `0xffe44fe9`, `0x98` and `0x9c` pointers into two literals, `0xa4` `0xffe5dd44`, `0xa8` 0 and `0xd4` `0xffe46508`. Package 1 `A` (`0xffe49820`) writes `0x04` `0xffe49730`, `0x54` to `0x64` as `0x46`, `0x4b`, `0x50`, `0x50`, `0x41`, and pointers at `0x98`, `0x9c`, `0xa0`, `0xa4`, `0xa8`, `0xac`, `0xb0`, `0xb4`, `0xcc`, `0xd0`, `0xd8`, `0xdc`, `0xe0`, `0xe4`. The blank initialisers (package 1 `0xffe4a258`, package 0 `0xffe49e9c`) set the registration hook at `0x04`, the type byte at `0x0c` to `0x4e`, `0x10` to 0, `0x14` to `0x20` to `0xff`, and clear the hooks. Package 3 and 5 descriptors are laid out differently (the same hooks sit at `0x9c`, `0xa0`, `0xa8`, `0xac`, `0xc4`, `0xc8`), so the offsets above do not carry over.
