/* generated from formats/l2_router_requests/format.toml by defs-gen - do not
 * edit, change the source */
/* SPDX-License-Identifier: BSD-3-Clause */
/* defs-gen 5 */
/* input sha256 877400033358bc5733bdc0424e204b91d1981956aabae6bedaaa0326b17f4e08 */
/* request codes of the L2 router library */
#ifndef L2R_GEN_H
#define L2R_GEN_H

#include <stdint.h>


/* word CMD: 32 bits */
#define L2R_CMD_WIDTH                                32 /* established */
#define L2R_CMD_CODE_SHIFT                           0 /* established */
#define L2R_CMD_CODE_MASK                            0xffffu
#define L2R_CMD_CODE_INPLACE (L2R_CMD_CODE_MASK << L2R_CMD_CODE_SHIFT)
#define L2R_CMD_CODE_CHUNK_FIRST_FLAGGED             12289 /* established */
#define L2R_CMD_CODE_CHUNK_FIRST                     12290 /* established */
#define L2R_CMD_CODE_CHUNK_NEXT                      12291 /* established */

#endif /* L2R_GEN_H */
