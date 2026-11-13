/* generated from devices/pcf8584/registers.toml by defs-gen - do not edit, change the source */
/* control bits and clock-control fields of the PCF8584 I2C-bus controller */
#ifndef PCF8584_GEN_H
#define PCF8584_GEN_H

#include <stdint.h>


#define PCF8584_CLOCK_OSC_12MHZ                      0x1cu /* inferred */
#define PCF8584_CLOCK_OSC_3MHZ                       0x0u /* inferred */
#define PCF8584_CLOCK_OSC_4_43MHZ                    0x10u /* inferred */
#define PCF8584_CLOCK_OSC_6MHZ                       0x14u /* inferred */
#define PCF8584_CLOCK_OSC_8MHZ                       0x18u /* inferred */
#define PCF8584_CLOCK_SCL_11KHZ                      0x2u /* inferred */
#define PCF8584_CLOCK_SCL_1_5KHZ                     0x3u /* inferred */
#define PCF8584_CLOCK_SCL_45KHZ                      0x1u /* inferred */
#define PCF8584_CLOCK_SCL_90KHZ                      0x0u /* inferred */
#define PCF8584_CTRL_ES1                             0x20u /* inferred */
#define PCF8584_CTRL_ESO                             0x40u /* inferred */
#define PCF8584_CTRL_PIN                             0x80u /* inferred */
#define PCF8584_OWN_ADDRESS_MASK                     0x7fu /* inferred */

#endif /* PCF8584_GEN_H */
