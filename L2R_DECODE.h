/*
 * L2R_DECODE.h - request codes of the L2 router library
 *
 * Request codes of the L2 controller router library.
 *
 * Each ENUM line is one request code. The comment above it gives the
 * argument shape as read in the image. Grades follow the source tables.
 */
#ifndef L2R_DECODE_H
#define L2R_DECODE_H

/* FIELD: CMD.code 15:0 established */
#define L2R_CMD_CODE_SHIFT 0
#define L2R_CMD_CODE_MASK 0xffffu

/* argument shape: first frame of a chunked message when bit 7 of packet byte 5 is set; header 9 bytes, payload up to 118 */
/* ENUM: CMD.code 0x3001 chunk_first_flagged established */
#define L2R_CMD_CODE_CHUNK_FIRST_FLAGGED 0x3001

/* argument shape: first frame otherwise */
/* ENUM: CMD.code 0x3002 chunk_first established */
#define L2R_CMD_CODE_CHUNK_FIRST 0x3002

/* argument shape: every later frame; length bytes zero */
/* ENUM: CMD.code 0x3003 chunk_next established */
#define L2R_CMD_CODE_CHUNK_NEXT 0x3003

#endif
