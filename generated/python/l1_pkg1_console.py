"""generated from formats/l1_pkg1_console/format.toml by defs-gen - do not edit, change the source
console command tables of ColdFire L1 firmware package 1"""

UNIT = 'l1_pkg1_console'
REGISTERS = {}
WORDS = {'CMD0': {'width': 32, 'grade': 'established'}, 'CMD1': {'width': 32, 'grade': 'established'}}
FIELDS = {'CMD0': {'index': (7, 0)}, 'CMD1': {'index': (8, 0)}}
ENUMS = {'CMD0.index': {0: 'pci', 1: 'pci_u', 2: 'pci_d', 3: 'pci_rst', 4: 'pci_number_u', 5: 'pci_number_d', 6: 'pci_number_rst', 7: 'pci_number_number_u', 8: 'pci_number_number_d', 9: 'pci_number_number_rst', 10: 'pci_power_check', 11: 'pci_power_check_on', 12: 'pci_power_check_off'}, 'CMD1.index': {0: 'all_command', 1: 'rack_slot_command', 2: 'autopower', 3: 'autopower_on', 4: 'autopower_off', 5: 'autopower_abort', 6: 'sys_link', 7: 'sys_link_none', 8: 'sys_link_ppp', 9: 'sys_link_enable', 10: 'sys_link_reset', 11: 'sys_link_packets', 12: 'brick', 13: 'brick_type_word', 14: 'brick_ptype_word', 15: 'brick_rack_decimal', 16: 'brick_slot_decimal', 17: 'brick_rackslot_decimal_decimal', 18: 'brick_rackslot_reset_number_number', 19: 'brick_partition_none', 20: 'brick_partition_number', 21: 'partdb', 22: 'partdb_enable', 23: 'partdb_disable', 24: 'cpu', 25: 'cpu_e', 26: 'cpu_d', 27: 'cpu_word_e', 28: 'cpu_word_d', 29: 'nia_command', 30: 'nib_command', 31: 'iia_command', 32: 'iib_command', 33: 'iic_command', 34: 'iid_command', 35: 'config', 36: 'config_v', 37: 'config_reset', 38: 'config_ring', 39: 'debug', 40: 'debug_number', 41: 'display', 42: 'display_number_command', 43: 'display_power_on', 44: 'display_attention_on', 45: 'display_failure_on', 46: 'display_power_off', 47: 'display_attention_off', 48: 'display_failure_off', 49: 'button', 50: 'button_all_enable', 51: 'button_all_disable', 52: 'button_pwr_enable', 53: 'button_pwr_disable', 54: 'button_rst_enable', 55: 'button_rst_disable', 56: 'button_nmi_enable', 57: 'button_nmi_disable', 58: 'let_the_carnage_begin', 59: 'slipped_my_mind', 60: 'super_sized_please', 61: 'env', 62: 'env_check', 63: 'env_on', 64: 'env_off', 65: 'env_power_on', 66: 'env_power_off', 67: 'env_power_monitor', 68: 'env_temp_on', 69: 'env_temp_off', 70: 'env_altitude_high', 71: 'env_altitude_low', 72: 'env_reset', 73: 'env_test_warning', 74: 'env_test_fault', 75: 'env_test_tmpadv', 76: 'env_test_tmpcrit', 77: 'env_test_number_number_number_number_number', 78: 'env_test_pwrdn', 79: 'env_messages', 80: 'env_messages_faultsonly', 81: 'env_messages_all', 82: 'fan', 83: 'fan_on_number', 84: 'fan_on', 85: 'fan_speed', 86: 'fan_speed_number_number', 87: 'fan_speed_number', 88: 'fan_speed_low_number', 89: 'fan_speed_low', 90: 'fan_auto_on', 91: 'help', 92: 'help_word', 93: 'help_list_hidden', 94: 'help_word_hidden', 95: 'history', 96: 'l1dbg', 97: 'l1dbg_sys_link_number', 98: 'l1dbg_sys_link_number_number', 99: 'l1dbg_irtr_number', 100: 'l1dbg_env_number', 101: 'l1dbg_env_off', 102: 'l1dbg_port_on', 103: 'l1dbg_i2c_on', 104: 'l1dbg_margin_on', 105: 'l1dbg_console_on', 106: 'l1dbg_mempanic_on', 107: 'l1dbg_pppdump_on', 108: 'l1dbg_pppdelay_number', 109: 'l1dbg_promreq_on', 110: 'l1dbg_qsusp_number_number', 111: 'l1dbg_printf_on', 112: 'l1dbg_flashignore_number_number', 113: 'l1dbg_misc_number', 114: 'link', 115: 'link_verbose', 116: 'log', 117: 'log_clear', 118: 'log_insert_command', 119: 'ioport', 120: 'ioport_400', 121: 'ioport_a_400', 122: 'ioport_b_400', 123: 'ioport_force_400', 124: 'ioport_force_alt1', 125: 'ioport_force_alt2', 126: 'ioport_clksrc_a', 127: 'ioport_clksrc_b', 128: 'istat_memory', 129: 'istat_queues', 130: 'istat_tasks', 131: 'istat_pmalloc', 132: 'istat_dump_word', 133: 'b2b', 134: 'l1', 135: 'leds', 136: 'margin', 137: 'margin_default', 138: 'margin_gbrick_default', 139: 'margin_number_default', 140: 'margin_number_number', 141: 'margin_dimm_v_word', 142: 'margin_pci_norm2hi', 143: 'margin_pci_norm2hi_on', 144: 'network', 145: 'network_usb', 146: 'network_autodetect_on', 147: 'pimm', 148: 'pimm_auto', 149: 'pimm_clksrc_ext', 150: 'pimm_clksrc_int', 151: 'pimm_clkmode_async', 152: 'pimm_clkmode_sync', 153: 'port', 154: 'power', 155: 'power_check', 156: 'power_vrm', 157: 'power_up', 158: 'power_up_hold', 159: 'power_down', 160: 'reset', 161: 'reset_hold', 162: 'nmi', 163: 'nmi_number', 164: 'nmi_rack_slot_word', 165: 'softreset', 166: 'softreset_number', 167: 'select', 168: 'select_number_number', 169: 'select_subchannel_console', 170: 'select_subchannel_number', 171: 'select_subchannel_a', 172: 'select_subchannel_word', 173: 'select_enable', 174: 'select_disable', 175: 'select_filter_on', 176: 'select_filter_off', 177: 'serial', 178: 'serial_verify', 179: 'serial_all', 180: 'serial_all_v', 181: 'serial_dimm', 182: 'serial_dimm_v', 183: 'serial_clear', 184: 'serial_set_word', 185: 'serial_word_word_word_word', 186: 'serial_security_off_word_word_word_word', 187: 'serial_security_on', 188: 'serial_brick_disable', 189: 'serial_brick_disable_confirm', 190: 'sysstate', 191: 'eeprom', 192: 'eeprom_number_number_number', 193: 'eeprom_fuel_write_default', 194: 'eeprom_dimm_unsign', 195: 'uart', 196: 'uart_sys_link_reset', 197: 'usb', 198: 'router', 199: 'router_meta', 200: 'router_repeater', 201: 'router_ordinary', 202: 'router_6_port', 203: 'router_6_port_confirm', 204: 'router_8_port', 205: 'router_8_port_word_word_word_word', 206: 'router_service_word_word_word_word', 207: 'router_service_off', 208: 'router_spare', 209: 'service_word_word_word_word', 210: 'service_off', 211: 'date_tz', 212: 'date_tz_word', 213: 'date', 214: 'date_raw', 215: 'date_word', 216: 'date_word_word', 217: 'nvram_reset', 218: 'nvram_read_number_number', 219: 'nvram_write_number_number', 220: 'nvram_scratch', 221: 'nvram_scratch_number', 222: 'l2_command', 223: 'security_verify_3400', 224: 'security_verify_3800', 225: 'flash_default_reset', 226: 'flash_default_a', 227: 'flash_default_b', 228: 'flash_default_current', 229: 'flash_default_old', 230: 'flash_default_new', 231: 'flash_status', 232: 'flash_status_v', 233: 'reboot_l1', 234: 'reboot_l1_a', 235: 'reboot_l1_b', 236: 'reboot_l1_current', 237: 'reboot_l1_old', 238: 'reboot_l1_new', 239: 'reboot_l1_other', 240: 'reboot_l1_addr_number', 241: 'reboot_l1_raw_number', 242: 'version', 243: 'pbay', 244: 'pbay_version', 245: 'pbay_dps', 246: 'pbay_dps_word', 247: 'pbay_dcport', 248: 'pbay_dcport_word', 249: 'pbay_fru', 250: 'pbay_fru_word', 251: 'pbay_reset', 252: 'pbay_env', 253: 'pbay_env_on', 254: 'pbay_env_off', 255: 'pbay_init', 256: 'pbay_serial', 257: 'test_i2c', 258: 'test_i2c_number', 259: 'test_ioexp_set_number_word', 260: 'test_ioexp_get_number', 261: 'test_ioexp_get_all', 262: 'test_intr', 263: 'test_display', 264: 'test_exception_mem', 265: 'test_exception_stack', 266: 'test_exception_wdog', 267: 'test_exception_fatal', 268: 'scan_reset', 269: 'scan_reset_hard', 270: 'scan_reset_soft', 271: 'scan_reset_both', 272: 'scan_sel_number_number', 273: 'scan_sel_number_number_number', 274: 'scan_sel_number_number_number_number', 275: 'scan_ids_number', 276: 'scan_set_trst_0', 277: 'scan_set_trst_1', 278: 'scan_set_tck_0', 279: 'scan_set_tck_1', 280: 'scan_set_tms_0', 281: 'scan_set_tms_1', 282: 'scan_set_tdi_0', 283: 'scan_set_tdi_1', 284: 'scan_set_psi_0', 285: 'scan_set_psi_1', 286: 'scan_get', 287: 'scan_debug_number', 288: 'scan_debug', 289: 'scan_imp_set_dllbyp_number', 290: 'scan_imp_set_number', 291: 'scan_imp_unlock', 292: 'scan_test_number', 293: 'fru', 294: 'fru_command'}}
FIELD_ENUM = {'CMD0.index': 'CMD0.index', 'CMD1.index': 'CMD1.index'}
CONSTANTS = {}
TESTS = []


def decode(name, value):
    """Field name -> value for a register or word."""
    return {f: (value >> lo) & ((1 << (hi - lo + 1)) - 1) for f, (hi, lo) in FIELDS[name].items()}


def encode(name, **fields):
    v = 0
    for f, x in fields.items():
        hi, lo = FIELDS[name][f]
        if x >> (hi - lo + 1):
            raise ValueError('%s.%s: %d does not fit' % (name, f, x))
        v |= x << lo
    return v


def describe(name, value):
    """decode, with enum labels where the field has an enum."""
    out = {}
    for f, x in decode(name, value).items():
        e = ENUMS.get(FIELD_ENUM.get('%s.%s' % (name, f)))
        out[f] = e.get(x, x) if e else x
    return out


def selftest():
    for name, value, expect in TESTS:
        got = decode(name, value)
        for f, want in expect.items():
            assert got[f] == want, (name, hex(value), f, got[f], want)
    return len(TESTS)


if __name__ == '__main__':
    print('%s: %d vectors ok' % (UNIT, selftest()))
