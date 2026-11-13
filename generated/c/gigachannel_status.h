/* generated from devices/gigachannel_status/registers.toml by defs-gen - do not edit, change the source */
/* GIGAchannel cabinet system-controller status word, as IRIX and the BASEIO PROM read it */
#ifndef GCSTATUS_GEN_H
#define GCSTATUS_GEN_H

#include <stdint.h>


/*
 * Cabinet status word, read at STATUS_OFFSET within the widget-8 window of the node's Hub. Software only reads it. No software writes it in the images read, and what a write would do is unknown. Bits 7 to 15 have no meaning that software reads. After start-up the IRIX kernel examines only bits 0 to 5, and bit 6 only once, at start-up.
 */
/* word STATUS: 16 bits; shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation */
#define GCSTATUS_STATUS_WIDTH                        16 /* inferred */
#define GCSTATUS_STATUS_REDUNDANT_SUPPLY_SHIFT       6 /* inferred */
#define GCSTATUS_STATUS_REDUNDANT_SUPPLY_MASK        0x1u
#define GCSTATUS_STATUS_REDUNDANT_SUPPLY_INPLACE     (GCSTATUS_STATUS_REDUNDANT_SUPPLY_MASK << GCSTATUS_STATUS_REDUNDANT_SUPPLY_SHIFT)
#define GCSTATUS_STATUS_MANUFACTURING_MODE_SHIFT     5 /* inferred */
#define GCSTATUS_STATUS_MANUFACTURING_MODE_MASK      0x1u
#define GCSTATUS_STATUS_MANUFACTURING_MODE_INPLACE   (GCSTATUS_STATUS_MANUFACTURING_MODE_MASK << GCSTATUS_STATUS_MANUFACTURING_MODE_SHIFT)
#define GCSTATUS_STATUS_REDUNDANCY_LOST_SHIFT        4 /* inferred */
#define GCSTATUS_STATUS_REDUNDANCY_LOST_MASK         0x1u
#define GCSTATUS_STATUS_REDUNDANCY_LOST_INPLACE      (GCSTATUS_STATUS_REDUNDANCY_LOST_MASK << GCSTATUS_STATUS_REDUNDANCY_LOST_SHIFT)
#define GCSTATUS_STATUS_EVENT_3_SHIFT                3 /* inferred */
#define GCSTATUS_STATUS_EVENT_3_MASK                 0x1u
#define GCSTATUS_STATUS_EVENT_3_INPLACE              (GCSTATUS_STATUS_EVENT_3_MASK << GCSTATUS_STATUS_EVENT_3_SHIFT)
#define GCSTATUS_STATUS_EVENT_2_SHIFT                2 /* inferred */
#define GCSTATUS_STATUS_EVENT_2_MASK                 0x1u
#define GCSTATUS_STATUS_EVENT_2_INPLACE              (GCSTATUS_STATUS_EVENT_2_MASK << GCSTATUS_STATUS_EVENT_2_SHIFT)
#define GCSTATUS_STATUS_EVENT_1_SHIFT                1 /* inferred */
#define GCSTATUS_STATUS_EVENT_1_MASK                 0x1u
#define GCSTATUS_STATUS_EVENT_1_INPLACE              (GCSTATUS_STATUS_EVENT_1_MASK << GCSTATUS_STATUS_EVENT_1_SHIFT)
#define GCSTATUS_STATUS_BLOWER_FAILURE_SHIFT         0 /* inferred */
#define GCSTATUS_STATUS_BLOWER_FAILURE_MASK          0x1u
#define GCSTATUS_STATUS_BLOWER_FAILURE_INPLACE       (GCSTATUS_STATUS_BLOWER_FAILURE_MASK << GCSTATUS_STATUS_BLOWER_FAILURE_SHIFT)

/*
 * Identification word of the Crossbow, read through each node's Hub at the widget-0 offset below. The kernel reads it at start-up to decide whether the node has a cabinet.
 */
/* word XBOW_ID: 32 bits; shipped IRIX kernel and BASEIO PROM images, read by disassembly: software expectation */
#define GCSTATUS_XBOW_ID_WIDTH                       32 /* inferred */
#define GCSTATUS_XBOW_ID_PART_SHIFT                  12 /* inferred */
#define GCSTATUS_XBOW_ID_PART_MASK                   0xffffu
#define GCSTATUS_XBOW_ID_PART_INPLACE                (GCSTATUS_XBOW_ID_PART_MASK << GCSTATUS_XBOW_ID_PART_SHIFT)

#define GCSTATUS_STATUS_OFFSET                       0xe00000u /* inferred */
#define GCSTATUS_WIDGET0_ID_OFFSET                   0xe0000004u /* inferred */
#define GCSTATUS_XBOW_PART_KERNEL                    0x0u /* inferred */
#define GCSTATUS_XBOW_PART_PROM                      0xd000u /* inferred */

#endif /* GCSTATUS_GEN_H */
