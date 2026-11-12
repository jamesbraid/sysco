/* generated from defs/mmsc_commands.toml by defs-gen - do not edit, change the source */
/* typed command table of the multi-module system */
#ifndef MMSC_GEN_H
#define MMSC_GEN_H

#include <stdint.h>


/* word AUTH: 32 bits */
#define MMSC_AUTH_WIDTH                              32 /* established */
#define MMSC_AUTH_LEVEL_SHIFT                        0 /* established */
#define MMSC_AUTH_LEVEL_MASK                         0xffffu
#define MMSC_AUTH_LEVEL_INPLACE                      (MMSC_AUTH_LEVEL_MASK << MMSC_AUTH_LEVEL_SHIFT)
#define MMSC_AUTH_LEVEL_BASIC                        100 /* established */
#define MMSC_AUTH_LEVEL_SUPERVISOR                   500 /* established */
#define MMSC_AUTH_LEVEL_SERVICE                      1000 /* established */

/* word CMD: 32 bits */
#define MMSC_CMD_WIDTH                               32 /* established */
#define MMSC_CMD_INDEX_SHIFT                         0 /* established */
#define MMSC_CMD_INDEX_MASK                          0xffu
#define MMSC_CMD_INDEX_INPLACE                       (MMSC_CMD_INDEX_MASK << MMSC_CMD_INDEX_SHIFT)
#define MMSC_CMD_INDEX_AUTH                          0 /* established */
#define MMSC_CMD_INDEX_AUTHORITY                     1 /* established */
#define MMSC_CMD_INDEX_BS                            2 /* established */
#define MMSC_CMD_INDEX_CECHO                         3 /* established */
#define MMSC_CMD_INDEX_COM                           4 /* established */
#define MMSC_CMD_INDEX_CONS                          5 /* established */
#define MMSC_CMD_INDEX_CONSOLE                       6 /* established */
#define MMSC_CMD_INDEX_DEBUG                         7 /* established */
#define MMSC_CMD_INDEX_NUMRACKS                      8 /* established */
#define MMSC_CMD_INDEX_DEST                          9 /* established */
#define MMSC_CMD_INDEX_DIRECT                        10 /* established */
#define MMSC_CMD_INDEX_DOWNLOADER                    11 /* established */
#define MMSC_CMD_INDEX_ELSC                          12 /* established */
#define MMSC_CMD_INDEX_EMSG                          13 /* established */
#define MMSC_CMD_INDEX_END                           14 /* established */
#define MMSC_CMD_INDEX_ESC                           15 /* established */
#define MMSC_CMD_INDEX_EXIT                          16 /* established */
#define MMSC_CMD_INDEX_FFSC                          17 /* established */
#define MMSC_CMD_INDEX_FLASH                         18 /* established */
#define MMSC_CMD_INDEX_GRAPHICS_COMMAND              19 /* established */
#define MMSC_CMD_INDEX_HELP                          20 /* established */
#define MMSC_CMD_INDEX_IPADDR                        21 /* established */
#define MMSC_CMD_INDEX_KILL                          22 /* established */
#define MMSC_CMD_INDEX_LOG                           23 /* established */
#define MMSC_CMD_INDEX_MEM                           24 /* established */
#define MMSC_CMD_INDEX_MMSC                          25 /* established */
#define MMSC_CMD_INDEX_MMSG                          26 /* established */
#define MMSC_CMD_INDEX_MSC                           27 /* established */
#define MMSC_CMD_INDEX_NAP_TIME                      28 /* established */
#define MMSC_CMD_INDEX_NOP                           29 /* established */
#define MMSC_CMD_INDEX_OPTIONS                       30 /* established */
#define MMSC_CMD_INDEX_PAGER                         31 /* established */
#define MMSC_CMD_INDEX_PAS                           32 /* established */
#define MMSC_CMD_INDEX_PASSWORD                      33 /* established */
#define MMSC_CMD_INDEX_PRINTENV                      34 /* established */
#define MMSC_CMD_INDEX_PWR                           35 /* established */
#define MMSC_CMD_INDEX_RACKID                        36 /* established */
#define MMSC_CMD_INDEX_RAT                           37 /* established */
#define MMSC_CMD_INDEX_REBOOT_FFSC                   38 /* established */
#define MMSC_CMD_INDEX_RESET_FFSC                    39 /* established */
#define MMSC_CMD_INDEX_RESET_MMSC                    40 /* established */
#define MMSC_CMD_INDEX_RESET_NVRAM                   41 /* established */
#define MMSC_CMD_INDEX_RMSG                          42 /* established */
#define MMSC_CMD_INDEX_SCAN                          43 /* established */
#define MMSC_CMD_INDEX_SETENV                        44 /* established */
#define MMSC_CMD_INDEX_STEAL                         45 /* established */
#define MMSC_CMD_INDEX_PARTLST                       46 /* established */
#define MMSC_CMD_INDEX_PARTSCAN                      47 /* established */
#define MMSC_CMD_INDEX_PARTGET                       48 /* established */
#define MMSC_CMD_INDEX_PARTNAM                       49 /* established */
#define MMSC_CMD_INDEX_PARTCLR                       50 /* established */
#define MMSC_CMD_INDEX_PARTSET                       51 /* established */
#define MMSC_CMD_INDEX_SWAPCONS                      52 /* established */
#define MMSC_CMD_INDEX_TTY_REINIT                    53 /* established */
#define MMSC_CMD_INDEX_UNSETENV                      54 /* established */
#define MMSC_CMD_INDEX_UNSTEAL                       55 /* established */
#define MMSC_CMD_INDEX_VER                           56 /* established */
#define MMSC_CMD_INDEX_ARBITER                       58 /* established */
#define MMSC_CMD_INDEX_MACADDR                       59 /* established */

/* word GATE: 32 bits */
#define MMSC_GATE_WIDTH                              32 /* established */
#define MMSC_GATE_LEVEL_SHIFT                        0 /* established */
#define MMSC_GATE_LEVEL_MASK                         0xffffu
#define MMSC_GATE_LEVEL_INPLACE                      (MMSC_GATE_LEVEL_MASK << MMSC_GATE_LEVEL_SHIFT)
#define MMSC_GATE_LEVEL_ABOVE_499                    499 /* established */
#define MMSC_GATE_LEVEL_ABOVE_999                    999 /* established */

#endif /* MMSC_GEN_H */
