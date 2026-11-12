/* generated from defs/l1_console.toml by defs-gen - do not edit, change the source */
/* entry layout and argument kinds of the L1 console command tables */
#ifndef L1C_GEN_H
#define L1C_GEN_H

#include <stdint.h>


/*
 * Up to ten pattern words, the handler, four selector words, the help text and the hidden flag; 68 bytes in all.
 */
/* pattern: r32, reset unknown */
#define ENTRY_PATTERN_OFFSET                         0x0u /* established */
#define ENTRY_PATTERN_COUNT                          10
#define ENTRY_PATTERN_STRIDE                         4

/* handler: r32, reset unknown */
#define ENTRY_HANDLER_OFFSET                         0x28u /* established */

/* arg: r32, reset unknown */
#define ENTRY_ARG_OFFSET                             0x2cu /* established */
#define ENTRY_ARG_COUNT                              4
#define ENTRY_ARG_STRIDE                             4

/* help: r32, reset unknown */
#define ENTRY_HELP_OFFSET                            0x3cu /* established */

/* hidden: r32, reset unknown */
#define ENTRY_HIDDEN_OFFSET                          0x40u /* established */

/* word TOKEN: 32 bits */
#define L1C_TOKEN_WIDTH                              32 /* established */
#define L1C_TOKEN_KIND_SHIFT                         0 /* established */
#define L1C_TOKEN_KIND_MASK                          0xfu
#define L1C_TOKEN_KIND_INPLACE                       (L1C_TOKEN_KIND_MASK << L1C_TOKEN_KIND_SHIFT)
#define L1C_TOKEN_KIND_END                           0 /* established */
#define L1C_TOKEN_KIND_NUMBER                        1 /* established */
#define L1C_TOKEN_KIND_DECIMAL                       2 /* established */
#define L1C_TOKEN_KIND_HEX                           3 /* established */
#define L1C_TOKEN_KIND_OCTAL                         4 /* established */
#define L1C_TOKEN_KIND_WORD                          5 /* established */
#define L1C_TOKEN_KIND_REST_OF_LINE                  6 /* established */
#define L1C_TOKEN_KIND_PORT_L1                       8 /* established */
#define L1C_TOKEN_KIND_RACK_SLOT                     11 /* established */

#define L1C_ENTRY_SIZE                               0x44u /* established */
#define L1C_ENTRY_WORDS                              0x11u /* established */
#define L1C_PATTERN_WORDS                            0xau /* established */
#define L1C_TABLES_MAX                               0xau /* established */

#endif /* L1C_GEN_H */
