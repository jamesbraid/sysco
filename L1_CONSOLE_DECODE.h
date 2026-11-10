/*
 * L1_CONSOLE_DECODE.h - entry layout and argument kinds of the L1 console command tables
 *
 * The console command line of the ColdFire L1 controller firmware is table driven. Each package
 * registers up to ten tables, and each table is an array of 68-byte entries made of seventeen
 * big-endian 32-bit words. The parser tries the registered tables in registration order and
 * each table from its first entry, and runs the handler of the first entry whose typed pattern
 * matches the line.
 *
 * The pattern is up to ten words and ends at the first zero word. A word above 11 points at a
 * literal; its alternatives are separated by a vertical bar and a typed word matches when it
 * equals one alternative byte for byte. A word of 1 to 11 stands for an argument of the kind
 * TOKEN.kind names. The values 7, 9 and 10 are not argument kinds; no table uses them.
 *
 * Four words follow the handler and are passed to it as one block. Handlers that restrict a
 * command to some brick types read the first word as a pointer to the string of allowed
 * brick-type characters (zero allows every brick; a leading '~' lists the excluded types).
 * The other handlers read the first word, and the three after it, as small selectors.
 *
 * The last word of an entry is 1 when the entry is left out of the command listing.
 *
 * Which entry sits at which position in each package is in the per-package console headers.
 */
#ifndef L1_CONSOLE_DECODE_H
#define L1_CONSOLE_DECODE_H

#include <stdint.h>

#define L1C_ENTRY_SIZE 68 /* established */
#define L1C_ENTRY_WORDS 17 /* established */
#define L1C_PATTERN_WORDS 10 /* established */
#define L1C_TABLES_MAX 10 /* established */

/* REG: ENTRY.pattern 0x00 r32 count=10 reset=unknown established */
/* REG: ENTRY.handler 0x28 r32 reset=unknown established */
/* REG: ENTRY.arg 0x2c r32 count=4 reset=unknown established */
/* REG: ENTRY.help 0x3c r32 reset=unknown established */
/* REG: ENTRY.hidden 0x40 r32 reset=unknown established */

/* FIELD: TOKEN.kind 3:0 established */
#define L1C_TOKEN_KIND_SHIFT 0
#define L1C_TOKEN_KIND_MASK 0xfu

/* end of the pattern */
/* ENUM: TOKEN.kind 0 end established */
#define L1C_TOKEN_KIND_END 0

/* number, base taken from its prefix (decimal, 0 for octal, 0x for hexadecimal) */
/* ENUM: TOKEN.kind 1 number established */
#define L1C_TOKEN_KIND_NUMBER 1

/* number, always decimal */
/* ENUM: TOKEN.kind 2 decimal established */
#define L1C_TOKEN_KIND_DECIMAL 2

/* number, always hexadecimal */
/* ENUM: TOKEN.kind 3 hex established */
#define L1C_TOKEN_KIND_HEX 3

/* number, always octal */
/* ENUM: TOKEN.kind 4 octal established */
#define L1C_TOKEN_KIND_OCTAL 4

/* one word, handed to the command as a string */
/* ENUM: TOKEN.kind 5 word established */
#define L1C_TOKEN_KIND_WORD 5

/* the rest of the line, handed to the command as a string */
/* ENUM: TOKEN.kind 6 rest_of_line established */
#define L1C_TOKEN_KIND_REST_OF_LINE 6

/* a colon, a port number, a colon and a controller number */
/* ENUM: TOKEN.kind 8 port_l1 established */
#define L1C_TOKEN_KIND_PORT_L1 8

/* a rack number, one separator character and a slot number */
/* ENUM: TOKEN.kind 11 rack_slot established */
#define L1C_TOKEN_KIND_RACK_SLOT 11

#endif
