# Origin 300 IP45 L1 M-Bus public fitment

The published MCF5206e manual establishes the M-Bus peripheral's parent interrupt path inside that chip. The examined Origin 300 publications do not establish the fitted L1 controller clock, its programmed interrupt level, or which board devices acknowledge M-Bus calls. Those values stay unknown until the routes below are followed.

## Evidence and limits

Its chapter 13, printed pp. 13-2 and 13-7, specifies the M-Bus peripheral and divider table. These are product-level sources. The chip manual's internal interrupt map applies only if the fitted controller is the documented MCF5206e revision.


## Fitment claims

- **Parent IRQ input, published generic MCF5206e:** M-Bus uses `ICR11` at
  `MBAR+0x01e`, and bit 11 in both `IMR` and `IPR`. `MIEN` and `MIF` permit
  its request. The parent mask can suppress delivery. This is an internal SIM
  input, not an external IP45 wire number. [Manual rev. 1, printed pp. 8-5,
- **Parent vector, published generic MCF5206e:** `ICR11` always selects an
  autovector. Its level and within-level priority are programmable. Reset
  `ICR11=0x80` sets no nonzero level. The IP45 programmed level, priority,
  and resulting vector number remain unresolved. [Manual rev. 1, printed
- **Fitted clock, unresolved:** MFDR selector `0x13`, if programmed, names
  divisor 480. The corrected equation is `SCL = CPU frequency / selected
  divider`. These are published chip rules, not a fitted IP45 selector or
  SCL rate. The board clock requires a matching controller specification or
  measurement. [Manual rev. 1, printed p. 13-7, and errata rev. 3, printed
- **Responding peers, unresolved:** SGI lists a serial ID EEPROM on IP45
  and says its L1 logic monitors temperature, fan speed, and LEDs. This
  published inventory does not identify M-Bus connectivity. Device
  markings, bus branches, seven-bit target addresses, ACK behavior, and
  board-revision population remain unresolved. [SGI 007-4365-002,
  printed p. 10.](https://irix7.com/techpubs/007-4365-002.pdf)

The SGI guide's board EEPROM is a hint. The firmware establishes that the L1 reads six EEPROM records over the M-Bus ([mbus.md](mbus.md)). Routes for the open items, from software: the interrupt level and priority are the `ICR11` write in the boot path (find the store at `MBAR+0x1e`, see the open question on on-chip peripherals); the clock is not stated in the image, so take it from the timer and UART setup, and from MAME's MCF5206E core for the register meanings.
