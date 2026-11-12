# console request codes of ColdFire L1 package 5

<!-- generated from defs/l1_pkg5_requests.toml by defs-gen - do not edit, change the source -->

Console request codes of one ColdFire L1 firmware package. Codes are unique within the package.

Unit `l1_pkg5_requests`, macro prefix `L1P5`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `CMD` | 32 | established |  |

### Word `CMD`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 15:0 | `code` | established | general_req_rack_bay=2, general_req_rack_bay_type=3, general_req_brick_serial=4, general_req_eeprom_read=6, general_req_eeprom_format=7, general_req_system_serial=8, general_req_port_speeds=10, general_req_attached_ports_1_to_4=12, general_req_attached_ports_5_and_10=13, general_req_requesting_port=14, general_req_processor_masks=16, general_req_scan_record=17, general_req_type_char=19, general_req_write_debug_switches=4097, general_req_display_line_1=4100, general_req_display_line_2=4101, general_req_partition_set=4102, general_req_event_subchannel_set=4103, general_req_two_bit_setting_set=4104, general_req_panel_control=4105, general_req_event_subchannel_clear=4106, general_req_prom_version_report=4107, general_req_per_processor_setting=4108, general_req_system_reset=8193, general_req_pci_operation_a=8194, general_req_pci_operation_b=8195, general_req_pci_operation_c=8196, general_req_power_down=8197, general_req_lifecycle_third_action=8198, general_req_set_clock=8205, general_req_get_clock=8206, general_req_hook_3001=12289, general_req_fru_3002=12290, general_req_fru_3003=12291, general_req_fru_3004=12292, general_req_inventory=16384, general_req_war_flag=16385, general_req_controller_reboot=16386, general_req_dimm_text=16388 |  |  |
