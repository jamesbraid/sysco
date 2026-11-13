"""generated from formats/l1_pkg1_requests/format.toml by defs-gen - do not edit, change the source
console request codes of ColdFire L1 package 1"""

UNIT = 'l1_pkg1_requests'
REGISTERS = {}
WORDS = {'CMD': {'width': 32, 'grade': 'established'}}
FIELDS = {'CMD': {'code': (15, 0)}}
ENUMS = {'CMD.code': {0: 'command_task_loop', 1: 'general_req_read_debug_switches', 2: 'general_req_rack_bay', 3: 'general_req_rack_bay_type', 4: 'general_req_brick_serial', 5: 'general_req_firmware_revision', 6: 'general_req_eeprom_read', 7: 'general_req_eeprom_format', 8: 'general_req_system_serial', 9: 'general_req_partition_get', 10: 'general_req_port_speeds', 11: 'general_req_two_bit_setting', 12: 'general_req_attached_ports_1_to_4', 13: 'general_req_attached_ports_5_and_10', 14: 'general_req_requesting_port', 15: 'general_req_last_event_reasons', 16: 'general_req_processor_masks', 17: 'general_req_scan_record', 114: 'flash_req_status', 115: 'flash_req_0x73', 116: 'flash_req_0x74', 146: 'power_bay_flash_req_0x92', 147: 'power_bay_flash_req_0x93', 148: 'power_bay_flash_req_0x94', 4097: 'general_req_write_debug_switches', 4098: 'general_req_console_subchannel', 4099: 'general_req_console_volunteer', 4100: 'general_req_display_line_1', 4101: 'general_req_display_line_2', 4102: 'general_req_partition_set', 4103: 'general_req_event_subchannel_set', 4104: 'general_req_two_bit_setting_set', 4105: 'general_req_panel_control', 4106: 'general_req_event_subchannel_clear', 4107: 'general_req_prom_version_report', 4108: 'general_req_per_processor_setting', 8193: 'general_req_system_reset', 8194: 'general_req_pci_operation_a', 8195: 'general_req_pci_operation_b', 8196: 'general_req_pci_operation_c', 8197: 'general_req_power_down', 8198: 'general_req_lifecycle_third_action', 8199: 'general_req_expander_read', 8200: 'general_req_expander_write', 16384: 'general_req_inventory', 16386: 'general_req_controller_reboot'}}
FIELD_ENUM = {'CMD.code': 'CMD.code'}
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
