<!-- generated from formats/l1_pkg2_console/format.toml by defs-gen - do not edit, change the source -->

# console command tables of ColdFire L1 firmware package 2

Position of each console command in the registered command tables of one ColdFire L1 firmware
package. The package registers 4 command tables. Each CMD<t>.index is the position of an
entry in table t, counted from zero, and table t is the t-th table in the order of the
registration calls in the image. That is not necessarily the order the parser tries them in;
the order of the calls at run time is not read here. Entry layout and argument kinds are in
the shared console unit. The typed pattern is the doc of each enum value; hidden entries are
marked. Labels are the first alternative of each word joined by underscores, with the
system-link word spelled sys_link; positions of entries that share a pattern are told apart by
a numeric suffix.

Unit `l1_pkg2_console`, macro prefix `L1P2C`.

Every item has a grade: established, inferred or unknown. A grade says how the item was derived from its source; it does not claim the item was measured on silicon.

## Words

| Word | Width | Grade | Source |
|---|---|---|---|
| `CMD0` | 32 | established |  |
| `CMD1` | 32 | established |  |
| `CMD2` | 32 | established |  |
| `CMD3` | 32 | established |  |

### Word `CMD0`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:0 | `index` | established | pci=0, pci_u=1, pci_d=2, pci_rst=3, pci_number_u=4, pci_number_d=5, pci_number_rst=6, pci_number_number_u=7, pci_number_number_d=8, pci_number_number_rst=9, pci_power_check=10, pci_power_check_on=11, pci_power_check_off=12 |  |  |

### Word `CMD1`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:0 | `index` | established | pci=0, pci_u=1, pci_d=2, pci_rst=3, pci_rst_hold=4, pci_rst_hold_number=5, pci_rst_hold_number_number=6, pci_rst_release=7, pci_rst_release_number=8, pci_rst_release_number_number=9, pci_number_u=10, pci_number_d=11, pci_number_rst=12, pci_number_number_u=13, pci_number_number_d=14, pci_number_number_rst=15, pci_force_all_on=16, pci_mode_limit_number_pci33=17, pci_mode_limit_pci33=18, pci_mode_force_number_pci33=19, pci_mode_force_pci33=20, pci_mode_default_number=21, pci_mode_default=22, pci_mode_warning_disable=23, pci_mode_warning_enable=24 |  |  |

### Word `CMD2`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 7:0 | `index` | established | check_power_on=0, check_margin_high=1, check_links=2, check_debug_number=3, check_security_on=4, check_serial_word=5, fru_capture=6, fru_capture_verbose=7, fru_examine=8, fru_dump=9, fru_reset=10, fru_deadlock=11, fru_deadlock_verbose=12, fru_deadlock_quiet=13, fru_events=14, ip59_3vrm=15, ip59_3vrm_e=16, promver=17, node=18, node_e=19, node_d=20, node_number_e=21, node_number_d=22, reset_number=23, prom_sel=24, prom_sel_number_a=25, prom_sel_number_b=26, prom_sel_a=27, prom_sel_b=28, try_power_led=29, pic_mode=30, make_rmws_number=31, pwm_read=32, pwm_set_number_number=33, sys_link_sim_on=34, sys_link_sim_on_number_number=35, sys_link_sim_off=36, sys_link_dump_input_number=37, sys_link_dump_output_number=38, sys_link_capture_reset=39, error_number=40 |  |  |

### Word `CMD3`

| Bits | Field | Grade | Values | Source | Description |
|---|---|---|---|---|---|
| 8:0 | `index` | established | all_command=0, rack_slot_command=1, autopower=2, autopower_on=3, autopower_off=4, autopower_abort=5, sys_link=6, sys_link_none=7, sys_link_ppp=8, sys_link_enable=9, sys_link_reset=10, sys_link_packets=11, brick=12, brick_type_word=13, brick_ptype_word=14, brick_rack_decimal=15, brick_slot_decimal=16, brick_rackslot_decimal_decimal=17, brick_rackslot_reset_number_number=18, brick_partition_none=19, brick_partition_number=20, partdb=21, partdb_enable=22, partdb_disable=23, cpu=24, cpu_e=25, cpu_d=26, cpu_word_e=27, cpu_word_d=28, nia_command=29, nib_command=30, iia_command=31, iib_command=32, iic_command=33, iid_command=34, config=35, config_v=36, config_reset=37, config_ring=38, debug=39, debug_number=40, display=41, display_number_command=42, display_power_on=43, display_attention_on=44, display_failure_on=45, display_power_off=46, display_attention_off=47, display_failure_off=48, button=49, button_all_enable=50, button_all_disable=51, button_pwr_enable=52, button_pwr_disable=53, button_rst_enable=54, button_rst_disable=55, button_nmi_enable=56, button_nmi_disable=57, let_the_carnage_begin=58, slipped_my_mind=59, super_sized_please=60, env=61, env_check=62, env_on=63, env_off=64, env_power_on=65, env_power_off=66, env_power_monitor=67, env_temp_on=68, env_temp_off=69, env_altitude_high=70, env_altitude_low=71, env_reset=72, env_test_warning=73, env_test_fault=74, env_test_tmpadv=75, env_test_tmpcrit=76, env_test_number_number_number_number_number=77, env_test_pwrdn=78, env_messages=79, env_messages_faultsonly=80, env_messages_all=81, fan=82, fan_on_number=83, fan_on=84, fan_speed=85, fan_speed_number_number=86, fan_speed_number=87, fan_speed_low_number=88, fan_speed_low=89, fan_auto_on=90, help=91, help_word=92, help_list_hidden=93, help_word_hidden=94, history=95, l1dbg=96, l1dbg_sys_link_number=97, l1dbg_sys_link_number_number=98, l1dbg_irtr_number=99, l1dbg_env_number=100, l1dbg_env_off=101, l1dbg_port_on=102, l1dbg_i2c_on=103, l1dbg_margin_on=104, l1dbg_console_on=105, l1dbg_mempanic_on=106, l1dbg_pppdump_on=107, l1dbg_pppdelay_number=108, l1dbg_promreq_on=109, l1dbg_qsusp_number_number=110, l1dbg_printf_on=111, l1dbg_flashignore_number_number=112, l1dbg_misc_number=113, link=114, link_verbose=115, log=116, log_clear=117, log_insert_command=118, ioport=119, ioport_400=120, ioport_a_400=121, ioport_b_400=122, ioport_force_400=123, ioport_force_alt1=124, ioport_force_alt2=125, ioport_clksrc_a=126, ioport_clksrc_b=127, istat_memory=128, istat_queues=129, istat_tasks=130, istat_pmalloc=131, istat_dump_word=132, b2b=133, l1=134, leds=135, margin=136, margin_default=137, margin_gbrick_default=138, margin_number_default=139, margin_number_number=140, margin_dimm_v_word=141, margin_pci_norm2hi=142, margin_pci_norm2hi_on=143, network=144, network_usb=145, network_autodetect_on=146, pimm=147, pimm_auto=148, pimm_clksrc_ext=149, pimm_clksrc_int=150, pimm_clkmode_async=151, pimm_clkmode_sync=152, port=153, power=154, power_check=155, power_vrm=156, power_up=157, power_up_hold=158, power_down=159, reset=160, reset_hold=161, nmi=162, nmi_number=163, nmi_rack_slot_word=164, softreset=165, softreset_number=166, select=167, select_number_number=168, select_subchannel_console=169, select_subchannel_number=170, select_subchannel_a=171, select_subchannel_word=172, select_enable=173, select_disable=174, select_filter_on=175, select_filter_off=176, serial=177, serial_verify=178, serial_all=179, serial_all_v=180, serial_dimm=181, serial_dimm_v=182, serial_clear=183, serial_set_word=184, serial_word_word_word_word=185, serial_security_off_word_word_word_word=186, serial_security_on=187, serial_brick_disable=188, serial_brick_disable_confirm=189, sysstate=190, eeprom=191, eeprom_number_number_number=192, eeprom_fuel_write_default=193, eeprom_dimm_unsign=194, uart=195, uart_sys_link_reset=196, usb=197, router=198, router_meta=199, router_repeater=200, router_ordinary=201, router_6_port=202, router_6_port_confirm=203, router_8_port=204, router_8_port_word_word_word_word=205, router_service_word_word_word_word=206, router_service_off=207, router_spare=208, service_word_word_word_word=209, service_off=210, date_tz=211, date_tz_word=212, date=213, date_raw=214, date_word=215, date_word_word=216, nvram_reset=217, nvram_read_number_number=218, nvram_write_number_number=219, nvram_scratch=220, nvram_scratch_number=221, l2_command=222, security_verify_3400=223, security_verify_3800=224, flash_default_reset=225, flash_default_a=226, flash_default_b=227, flash_default_current=228, flash_default_old=229, flash_default_new=230, flash_status=231, flash_status_v=232, reboot_l1=233, reboot_l1_a=234, reboot_l1_b=235, reboot_l1_current=236, reboot_l1_old=237, reboot_l1_new=238, reboot_l1_other=239, reboot_l1_addr_number=240, reboot_l1_raw_number=241, version=242, pbay=243, pbay_version=244, pbay_dps=245, pbay_dps_word=246, pbay_dcport=247, pbay_dcport_word=248, pbay_fru=249, pbay_fru_word=250, pbay_reset=251, pbay_env=252, pbay_env_on=253, pbay_env_off=254, pbay_init=255, pbay_serial=256, test_i2c=257, test_i2c_number=258, test_ioexp_set_number_word=259, test_ioexp_get_number=260, test_ioexp_get_all=261, test_intr=262, test_display=263, test_exception_mem=264, test_exception_stack=265, test_exception_wdog=266, test_exception_fatal=267, scan_reset=268, scan_reset_hard=269, scan_reset_soft=270, scan_reset_both=271, scan_sel_number_number=272, scan_sel_number_number_number=273, scan_sel_number_number_number_number=274, scan_ids_number=275, scan_set_trst_0=276, scan_set_trst_1=277, scan_set_tck_0=278, scan_set_tck_1=279, scan_set_tms_0=280, scan_set_tms_1=281, scan_set_tdi_0=282, scan_set_tdi_1=283, scan_set_psi_0=284, scan_set_psi_1=285, scan_get=286, scan_debug_number=287, scan_debug=288, scan_imp_set_dllbyp_number=289, scan_imp_set_number=290, scan_imp_unlock=291, scan_test_number=292, fru=293, fru_command=294 |  |  |
