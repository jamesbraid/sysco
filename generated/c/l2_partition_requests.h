/* generated from formats/l2_partition_requests/format.toml by defs-gen - do not edit, change the source */
/* request codes of the L2 partition task image */
#ifndef L2P_GEN_H
#define L2P_GEN_H

#include <stdint.h>


/* word CMD: 32 bits */
#define L2P_CMD_WIDTH                                32 /* established */
#define L2P_CMD_CODE_SHIFT                           0 /* established */
#define L2P_CMD_CODE_MASK                            0xffffu
#define L2P_CMD_CODE_INPLACE                         (L2P_CMD_CODE_MASK << L2P_CMD_CODE_SHIFT)
#define L2P_CMD_CODE_PARTITION_FOR_BRICK             1 /* established */
#define L2P_CMD_CODE_PARTITION_ENVIRONMENT           2 /* established */
#define L2P_CMD_CODE_DATABASE_GET                    3 /* established */
#define L2P_CMD_CODE_DATABASE_TIME                   4 /* established */
#define L2P_CMD_CODE_DATABASE_SET                    5 /* established */
#define L2P_CMD_CODE_MODE_SET                        6 /* established */
#define L2P_CMD_CODE_PARK_REQUEST                    7 /* inferred */
#define L2P_CMD_CODE_RELEASE_PARKED                  8 /* inferred */

#endif /* L2P_GEN_H */
