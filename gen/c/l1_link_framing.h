/* generated from defs/l1_link_framing.toml by defs-gen - do not edit, change the source */
/* frame bytes and link context constants of the ColdFire L1 node and peer links */
#ifndef L1LF_GEN_H
#define L1LF_GEN_H

#include <stdint.h>


#define L1LF_FRAME_CHECK_RECEIVE                     0xf0b8u /* established */
#define L1LF_FRAME_CHECK_SEED                        0xffffu /* established */
#define L1LF_FRAME_ESCAPE                            0x7du /* established */
#define L1LF_FRAME_ESCAPE_XOR                        0x20u /* established */
#define L1LF_FRAME_FLAG                              0x7eu /* established */
#define L1LF_NODE_BODY_CAP                           0x100u /* inferred */
#define L1LF_NODE_PAYLOAD_MAX                        0x80u /* inferred */
#define L1LF_NODE_UART_BLOCK_OFFSET                  0x30000080u /* inferred */
#define L1LF_PEER_BODY_OFFSET                        0x10u /* established */
#define L1LF_PEER_ESCAPE_TX_A                        0x11u /* inferred */
#define L1LF_PEER_ESCAPE_TX_B                        0x13u /* inferred */
#define L1LF_PEER_GROWTH_LIMIT                       0x410u /* established */

#endif /* L1LF_GEN_H */
