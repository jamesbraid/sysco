/*
 * ELSC_DECODE.h - two-wire addresses the host driver of the Origin 2000 module controller uses
 *
 * What the host-side driver for the module system controller (the MSC, also called ELSC)
 * passes as the device argument of its two-wire transfers, and how it builds the one that
 * varies. Everything here is read from the host driver object; the controller's own firmware
 * is not held, so what sits behind each address is a reading, not a finding.
 *
 * Each define carries the grade of the reading. The device argument is a 7-bit address when
 * the transfer routine takes it as one; the routine is in the kernel, not in the object, so
 * that form is inferred.
 */
#ifndef ELSC_DECODE_H
#define ELSC_DECODE_H

/* Request mailbox: one combined transfer writes the request packet and reads the reply. */
#define ELSC_MAILBOX_SLAVE 8u /* established: the argument in the call; 7-bit form inferred */

/* Non-volatile store: the device argument is a base ORed with three bits of the byte address,
 * one page per value, and the low eight bits of the address go out as the in-page byte. */
#define ELSC_NVRAM_SLAVE_BASE 0x50u /* established */
#define ELSC_NVRAM_PAGE_LOW_BIT 8u /* established: address bits 10:8 select the page */
#define ELSC_NVRAM_PAGE_BITS 0x7u /* established */
#define ELSC_NVRAM_PAGES 8u /* established: addresses 0 to 0x7ff */
/* The part is probably a 2 KB serial EEPROM; that is a hint from the paging, not a finding. */

/* A single byte, written or read with no register address; what it holds is unknown. */
#define ELSC_UST_SLAVE 0x20u /* established for the call; meaning unknown */

#endif
