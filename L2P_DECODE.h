/*
 * L2P_DECODE.h - request codes of the L2 partition task image
 *
 * Request codes of the L2 controller partition task image.
 *
 * Each ENUM line is one request code. The comment above it gives the
 * argument shape as read in the image. Grades follow the source tables.
 */
#ifndef L2P_DECODE_H
#define L2P_DECODE_H

/* FIELD: CMD.code 15:0 established */
#define L2P_CMD_CODE_SHIFT 0
#define L2P_CMD_CODE_MASK 0xffffu

/* argument shape: brick number in; partition number out (0 if unassigned); reply result 0 and one integer */
/* ENUM: CMD.code 0x0001 partition_for_brick established */
#define L2P_CMD_CODE_PARTITION_FOR_BRICK 0x0001

/* argument shape: partition number in; integer and environment block out; sent by partmgr as the variables request */
/* ENUM: CMD.code 0x0002 partition_environment established */
#define L2P_CMD_CODE_PARTITION_ENVIRONMENT 0x0002

/* argument shape: no argument; whole mode database out in one binary argument; builds the default first if none is loaded */
/* ENUM: CMD.code 0x0003 database_get established */
#define L2P_CMD_CODE_DATABASE_GET 0x0003

/* argument shape: no argument; the header timestamp out as one integer */
/* ENUM: CMD.code 0x0004 database_time established */
#define L2P_CMD_CODE_DATABASE_TIME 0x0004

/* argument shape: complete database image in; header checks give results -1 to -6, older timestamp 7; installs, unpacks and writes flash; broadcasts the new image to task 5 */
/* ENUM: CMD.code 0x0005 database_set established */
#define L2P_CMD_CODE_DATABASE_SET 0x0005

/* argument shape: integer equal to the current timestamp plus a 24-byte header equal to the current one except the word at a fixed location; installs that word; the meaning of the word is inferred as the current mode */
/* ENUM: CMD.code 0x0006 mode_set established */
#define L2P_CMD_CODE_MODE_SET 0x0006

/* argument shape: slot 0 to 63 and the request itself kept in a table, no immediate reply; used for deferred replies */
/* ENUM: CMD.code 0x0007 park_request inferred */
#define L2P_CMD_CODE_PARK_REQUEST 0x0007

/* argument shape: slot and string; replies to the parked request in that slot with the string and clears the slot */
/* ENUM: CMD.code 0x0008 release_parked inferred */
#define L2P_CMD_CODE_RELEASE_PARKED 0x0008

#endif
