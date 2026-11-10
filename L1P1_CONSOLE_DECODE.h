/*
 * L1P1_CONSOLE_DECODE.h - console command tables of ColdFire L1 firmware package 1
 *
 * The package registers 2 command tables. Each CMD<t>.index below is the position of an
 * entry in table t, counted from zero, and table t is the t-th table in the order of the
 * registration calls in the image. That is not necessarily the order the parser tries them in;
 * the order of the calls at run time is not read here. Entry layout and argument kinds are in
 * the shared console header. The typed pattern is repeated above each ENUM line; hidden entries are
 * marked. Labels are the first alternative of each word joined by underscores, with the
 * system-link word spelled sys_link; positions of entries that share a pattern are told apart by
 * a numeric suffix.
 */
#ifndef L1P1_CONSOLE_DECODE_H
#define L1P1_CONSOLE_DECODE_H

/* FIELD: CMD0.index 7:0 established */
#define L1P1C_CMD0_INDEX_SHIFT 0
#define L1P1C_CMD0_INDEX_MASK 0xffu

/* typed pci */
/* ENUM: CMD0.index 0 pci established */
#define L1P1C_CMD0_INDEX_PCI 0

/* typed pci u */
/* ENUM: CMD0.index 1 pci_u established */
#define L1P1C_CMD0_INDEX_PCI_U 1

/* typed pci d */
/* ENUM: CMD0.index 2 pci_d established */
#define L1P1C_CMD0_INDEX_PCI_D 2

/* typed pci rst|reset */
/* ENUM: CMD0.index 3 pci_rst established */
#define L1P1C_CMD0_INDEX_PCI_RST 3

/* typed pci <number> u */
/* ENUM: CMD0.index 4 pci_number_u established */
#define L1P1C_CMD0_INDEX_PCI_NUMBER_U 4

/* typed pci <number> d */
/* ENUM: CMD0.index 5 pci_number_d established */
#define L1P1C_CMD0_INDEX_PCI_NUMBER_D 5

/* typed pci <number> rst|reset */
/* ENUM: CMD0.index 6 pci_number_rst established */
#define L1P1C_CMD0_INDEX_PCI_NUMBER_RST 6

/* typed pci <number> <number> u */
/* ENUM: CMD0.index 7 pci_number_number_u established */
#define L1P1C_CMD0_INDEX_PCI_NUMBER_NUMBER_U 7

/* typed pci <number> <number> d */
/* ENUM: CMD0.index 8 pci_number_number_d established */
#define L1P1C_CMD0_INDEX_PCI_NUMBER_NUMBER_D 8

/* typed pci <number> <number> rst|reset */
/* ENUM: CMD0.index 9 pci_number_number_rst established */
#define L1P1C_CMD0_INDEX_PCI_NUMBER_NUMBER_RST 9

/* typed pci power|pwr check */
/* ENUM: CMD0.index 10 pci_power_check established */
#define L1P1C_CMD0_INDEX_PCI_POWER_CHECK 10

/* typed pci power|pwr check on */
/* ENUM: CMD0.index 11 pci_power_check_on established */
#define L1P1C_CMD0_INDEX_PCI_POWER_CHECK_ON 11

/* typed pci power|pwr check off */
/* ENUM: CMD0.index 12 pci_power_check_off established */
#define L1P1C_CMD0_INDEX_PCI_POWER_CHECK_OFF 12

/* FIELD: CMD1.index 8:0 established */
#define L1P1C_CMD1_INDEX_SHIFT 0
#define L1P1C_CMD1_INDEX_MASK 0x1ffu

/* typed * <command> */
/* ENUM: CMD1.index 0 all_command established */
#define L1P1C_CMD1_INDEX_ALL_COMMAND 0

/* typed <rack_slot> <command> */
/* ENUM: CMD1.index 1 rack_slot_command established */
#define L1P1C_CMD1_INDEX_RACK_SLOT_COMMAND 1

/* typed autopower|apwr */
/* ENUM: CMD1.index 2 autopower established */
#define L1P1C_CMD1_INDEX_AUTOPOWER 2

/* typed autopower|apwr on */
/* ENUM: CMD1.index 3 autopower_on established */
#define L1P1C_CMD1_INDEX_AUTOPOWER_ON 3

/* typed autopower|apwr off */
/* ENUM: CMD1.index 4 autopower_off established */
#define L1P1C_CMD1_INDEX_AUTOPOWER_OFF 4

/* typed autopower|apwr abort */
/* ENUM: CMD1.index 5 autopower_abort established */
#define L1P1C_CMD1_INDEX_AUTOPOWER_ABORT 5

/* typed syscom|junkbus|jb|bedrock */
/* ENUM: CMD1.index 6 sys_link established */
#define L1P1C_CMD1_INDEX_SYS_LINK 6

/* typed syscom|junkbus|jb|bedrock none (hidden) */
/* ENUM: CMD1.index 7 sys_link_none established */
#define L1P1C_CMD1_INDEX_SYS_LINK_NONE 7

/* typed syscom|junkbus|jb|bedrock ppp (hidden) */
/* ENUM: CMD1.index 8 sys_link_ppp established */
#define L1P1C_CMD1_INDEX_SYS_LINK_PPP 8

/* typed syscom|junkbus|jb|bedrock enable (hidden) */
/* ENUM: CMD1.index 9 sys_link_enable established */
#define L1P1C_CMD1_INDEX_SYS_LINK_ENABLE 9

/* typed syscom reset (hidden) */
/* ENUM: CMD1.index 10 sys_link_reset established */
#define L1P1C_CMD1_INDEX_SYS_LINK_RESET 10

/* typed syscom packets (hidden) */
/* ENUM: CMD1.index 11 sys_link_packets established */
#define L1P1C_CMD1_INDEX_SYS_LINK_PACKETS 11

/* typed brick */
/* ENUM: CMD1.index 12 brick established */
#define L1P1C_CMD1_INDEX_BRICK 12

/* typed brick type <word> */
/* ENUM: CMD1.index 13 brick_type_word established */
#define L1P1C_CMD1_INDEX_BRICK_TYPE_WORD 13

/* typed brick ptype <word> */
/* ENUM: CMD1.index 14 brick_ptype_word established */
#define L1P1C_CMD1_INDEX_BRICK_PTYPE_WORD 14

/* typed brick rack <decimal> */
/* ENUM: CMD1.index 15 brick_rack_decimal established */
#define L1P1C_CMD1_INDEX_BRICK_RACK_DECIMAL 15

/* typed brick slot|bay|upos <decimal> */
/* ENUM: CMD1.index 16 brick_slot_decimal established */
#define L1P1C_CMD1_INDEX_BRICK_SLOT_DECIMAL 16

/* typed brick rackslot|rs <decimal> <decimal> */
/* ENUM: CMD1.index 17 brick_rackslot_decimal_decimal established */
#define L1P1C_CMD1_INDEX_BRICK_RACKSLOT_DECIMAL_DECIMAL 17

/* typed brick rackslot|rs reset <number> <number> (hidden) */
/* ENUM: CMD1.index 18 brick_rackslot_reset_number_number established */
#define L1P1C_CMD1_INDEX_BRICK_RACKSLOT_RESET_NUMBER_NUMBER 18

/* typed brick partition|part|p none */
/* ENUM: CMD1.index 19 brick_partition_none established */
#define L1P1C_CMD1_INDEX_BRICK_PARTITION_NONE 19

/* typed brick partition|part <number> */
/* ENUM: CMD1.index 20 brick_partition_number established */
#define L1P1C_CMD1_INDEX_BRICK_PARTITION_NUMBER 20

/* typed partdb */
/* ENUM: CMD1.index 21 partdb established */
#define L1P1C_CMD1_INDEX_PARTDB 21

/* typed partdb enable|e|on */
/* ENUM: CMD1.index 22 partdb_enable established */
#define L1P1C_CMD1_INDEX_PARTDB_ENABLE 22

/* typed partdb disable|d|off */
/* ENUM: CMD1.index 23 partdb_disable established */
#define L1P1C_CMD1_INDEX_PARTDB_DISABLE 23

/* typed cpu */
/* ENUM: CMD1.index 24 cpu established */
#define L1P1C_CMD1_INDEX_CPU 24

/* typed cpu e */
/* ENUM: CMD1.index 25 cpu_e established */
#define L1P1C_CMD1_INDEX_CPU_E 25

/* typed cpu d */
/* ENUM: CMD1.index 26 cpu_d established */
#define L1P1C_CMD1_INDEX_CPU_D 26

/* typed cpu <word> e */
/* ENUM: CMD1.index 27 cpu_word_e established */
#define L1P1C_CMD1_INDEX_CPU_WORD_E 27

/* typed cpu <word> d */
/* ENUM: CMD1.index 28 cpu_word_d established */
#define L1P1C_CMD1_INDEX_CPU_WORD_D 28

/* typed nia|ni|ctc <command> */
/* ENUM: CMD1.index 29 nia_command established */
#define L1P1C_CMD1_INDEX_NIA_COMMAND 29

/* typed nib <command> */
/* ENUM: CMD1.index 30 nib_command established */
#define L1P1C_CMD1_INDEX_NIB_COMMAND 30

/* typed iia|ii|cti <command> */
/* ENUM: CMD1.index 31 iia_command established */
#define L1P1C_CMD1_INDEX_IIA_COMMAND 31

/* typed iib <command> */
/* ENUM: CMD1.index 32 iib_command established */
#define L1P1C_CMD1_INDEX_IIB_COMMAND 32

/* typed iic <command> */
/* ENUM: CMD1.index 33 iic_command established */
#define L1P1C_CMD1_INDEX_IIC_COMMAND 33

/* typed iid <command> */
/* ENUM: CMD1.index 34 iid_command established */
#define L1P1C_CMD1_INDEX_IID_COMMAND 34

/* typed config|cfg */
/* ENUM: CMD1.index 35 config established */
#define L1P1C_CMD1_INDEX_CONFIG 35

/* typed config|cfg v|verbose */
/* ENUM: CMD1.index 36 config_v established */
#define L1P1C_CMD1_INDEX_CONFIG_V 36

/* typed config|cfg reset */
/* ENUM: CMD1.index 37 config_reset established */
#define L1P1C_CMD1_INDEX_CONFIG_RESET 37

/* typed config|cfg ring */
/* ENUM: CMD1.index 38 config_ring established */
#define L1P1C_CMD1_INDEX_CONFIG_RING 38

/* typed debug */
/* ENUM: CMD1.index 39 debug established */
#define L1P1C_CMD1_INDEX_DEBUG 39

/* typed debug <number> */
/* ENUM: CMD1.index 40 debug_number established */
#define L1P1C_CMD1_INDEX_DEBUG_NUMBER 40

/* typed display|dsp */
/* ENUM: CMD1.index 41 display established */
#define L1P1C_CMD1_INDEX_DISPLAY 41

/* typed display|dsp <number> <command> */
/* ENUM: CMD1.index 42 display_number_command established */
#define L1P1C_CMD1_INDEX_DISPLAY_NUMBER_COMMAND 42

/* typed display|dsp power|pwr on */
/* ENUM: CMD1.index 43 display_power_on established */
#define L1P1C_CMD1_INDEX_DISPLAY_POWER_ON 43

/* typed display|dsp attention|attn on */
/* ENUM: CMD1.index 44 display_attention_on established */
#define L1P1C_CMD1_INDEX_DISPLAY_ATTENTION_ON 44

/* typed display|dsp failure|fail on */
/* ENUM: CMD1.index 45 display_failure_on established */
#define L1P1C_CMD1_INDEX_DISPLAY_FAILURE_ON 45

/* typed display|dsp power|pwr off */
/* ENUM: CMD1.index 46 display_power_off established */
#define L1P1C_CMD1_INDEX_DISPLAY_POWER_OFF 46

/* typed display|dsp attention|attn off */
/* ENUM: CMD1.index 47 display_attention_off established */
#define L1P1C_CMD1_INDEX_DISPLAY_ATTENTION_OFF 47

/* typed display|dsp failure|fail off */
/* ENUM: CMD1.index 48 display_failure_off established */
#define L1P1C_CMD1_INDEX_DISPLAY_FAILURE_OFF 48

/* typed button|btn */
/* ENUM: CMD1.index 49 button established */
#define L1P1C_CMD1_INDEX_BUTTON 49

/* typed button|btn all enable|en|e */
/* ENUM: CMD1.index 50 button_all_enable established */
#define L1P1C_CMD1_INDEX_BUTTON_ALL_ENABLE 50

/* typed button|btn all disable|dis|d */
/* ENUM: CMD1.index 51 button_all_disable established */
#define L1P1C_CMD1_INDEX_BUTTON_ALL_DISABLE 51

/* typed button|btn pwr|power enable|en|e */
/* ENUM: CMD1.index 52 button_pwr_enable established */
#define L1P1C_CMD1_INDEX_BUTTON_PWR_ENABLE 52

/* typed button|btn pwr|power disable|dis|d */
/* ENUM: CMD1.index 53 button_pwr_disable established */
#define L1P1C_CMD1_INDEX_BUTTON_PWR_DISABLE 53

/* typed button|btn rst|reset enable|en|e */
/* ENUM: CMD1.index 54 button_rst_enable established */
#define L1P1C_CMD1_INDEX_BUTTON_RST_ENABLE 54

/* typed button|btn rst|reset disable|dis|d */
/* ENUM: CMD1.index 55 button_rst_disable established */
#define L1P1C_CMD1_INDEX_BUTTON_RST_DISABLE 55

/* typed button|btn nmi enable|en|e */
/* ENUM: CMD1.index 56 button_nmi_enable established */
#define L1P1C_CMD1_INDEX_BUTTON_NMI_ENABLE 56

/* typed button|btn nmi disable|dis|d */
/* ENUM: CMD1.index 57 button_nmi_disable established */
#define L1P1C_CMD1_INDEX_BUTTON_NMI_DISABLE 57

/* typed let the carnage begin (hidden) */
/* ENUM: CMD1.index 58 let_the_carnage_begin established */
#define L1P1C_CMD1_INDEX_LET_THE_CARNAGE_BEGIN 58

/* typed slipped my mind (hidden) */
/* ENUM: CMD1.index 59 slipped_my_mind established */
#define L1P1C_CMD1_INDEX_SLIPPED_MY_MIND 59

/* typed super sized please (hidden) */
/* ENUM: CMD1.index 60 super_sized_please established */
#define L1P1C_CMD1_INDEX_SUPER_SIZED_PLEASE 60

/* typed env */
/* ENUM: CMD1.index 61 env established */
#define L1P1C_CMD1_INDEX_ENV 61

/* typed env check */
/* ENUM: CMD1.index 62 env_check established */
#define L1P1C_CMD1_INDEX_ENV_CHECK 62

/* typed env on */
/* ENUM: CMD1.index 63 env_on established */
#define L1P1C_CMD1_INDEX_ENV_ON 63

/* typed env off */
/* ENUM: CMD1.index 64 env_off established */
#define L1P1C_CMD1_INDEX_ENV_OFF 64

/* typed env power|pwr on (hidden) */
/* ENUM: CMD1.index 65 env_power_on established */
#define L1P1C_CMD1_INDEX_ENV_POWER_ON 65

/* typed env power|pwr off (hidden) */
/* ENUM: CMD1.index 66 env_power_off established */
#define L1P1C_CMD1_INDEX_ENV_POWER_OFF 66

/* typed env power|pwr monitor (hidden) */
/* ENUM: CMD1.index 67 env_power_monitor established */
#define L1P1C_CMD1_INDEX_ENV_POWER_MONITOR 67

/* typed env temp on */
/* ENUM: CMD1.index 68 env_temp_on established */
#define L1P1C_CMD1_INDEX_ENV_TEMP_ON 68

/* typed env temp off */
/* ENUM: CMD1.index 69 env_temp_off established */
#define L1P1C_CMD1_INDEX_ENV_TEMP_OFF 69

/* typed env altitude high */
/* ENUM: CMD1.index 70 env_altitude_high established */
#define L1P1C_CMD1_INDEX_ENV_ALTITUDE_HIGH 70

/* typed env altitude low */
/* ENUM: CMD1.index 71 env_altitude_low established */
#define L1P1C_CMD1_INDEX_ENV_ALTITUDE_LOW 71

/* typed env reset|rst */
/* ENUM: CMD1.index 72 env_reset established */
#define L1P1C_CMD1_INDEX_ENV_RESET 72

/* typed env test warning */
/* ENUM: CMD1.index 73 env_test_warning established */
#define L1P1C_CMD1_INDEX_ENV_TEST_WARNING 73

/* typed env test fault */
/* ENUM: CMD1.index 74 env_test_fault established */
#define L1P1C_CMD1_INDEX_ENV_TEST_FAULT 74

/* typed env test tmpadv */
/* ENUM: CMD1.index 75 env_test_tmpadv established */
#define L1P1C_CMD1_INDEX_ENV_TEST_TMPADV 75

/* typed env test tmpcrit */
/* ENUM: CMD1.index 76 env_test_tmpcrit established */
#define L1P1C_CMD1_INDEX_ENV_TEST_TMPCRIT 76

/* typed env test <number> <number> <number> <number> <number> (hidden) */
/* ENUM: CMD1.index 77 env_test_number_number_number_number_number established */
#define L1P1C_CMD1_INDEX_ENV_TEST_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER 77

/* typed env test pwrdn (hidden) */
/* ENUM: CMD1.index 78 env_test_pwrdn established */
#define L1P1C_CMD1_INDEX_ENV_TEST_PWRDN 78

/* typed env messages (hidden) */
/* ENUM: CMD1.index 79 env_messages established */
#define L1P1C_CMD1_INDEX_ENV_MESSAGES 79

/* typed env messages faultsonly (hidden) */
/* ENUM: CMD1.index 80 env_messages_faultsonly established */
#define L1P1C_CMD1_INDEX_ENV_MESSAGES_FAULTSONLY 80

/* typed env messages all (hidden) */
/* ENUM: CMD1.index 81 env_messages_all established */
#define L1P1C_CMD1_INDEX_ENV_MESSAGES_ALL 81

/* typed fan */
/* ENUM: CMD1.index 82 fan established */
#define L1P1C_CMD1_INDEX_FAN 82

/* typed fan on|off <number> (hidden) */
/* ENUM: CMD1.index 83 fan_on_number established */
#define L1P1C_CMD1_INDEX_FAN_ON_NUMBER 83

/* typed fan on|off (hidden) */
/* ENUM: CMD1.index 84 fan_on established */
#define L1P1C_CMD1_INDEX_FAN_ON 84

/* typed fan speed (hidden) */
/* ENUM: CMD1.index 85 fan_speed established */
#define L1P1C_CMD1_INDEX_FAN_SPEED 85

/* typed fan speed <number> <number> (hidden) */
/* ENUM: CMD1.index 86 fan_speed_number_number established */
#define L1P1C_CMD1_INDEX_FAN_SPEED_NUMBER_NUMBER 86

/* typed fan speed <number> (hidden) */
/* ENUM: CMD1.index 87 fan_speed_number established */
#define L1P1C_CMD1_INDEX_FAN_SPEED_NUMBER 87

/* typed fan speed low|l|med|m|high|h <number> (hidden) */
/* ENUM: CMD1.index 88 fan_speed_low_number established */
#define L1P1C_CMD1_INDEX_FAN_SPEED_LOW_NUMBER 88

/* typed fan speed low|l|med|m|high|h (hidden) */
/* ENUM: CMD1.index 89 fan_speed_low established */
#define L1P1C_CMD1_INDEX_FAN_SPEED_LOW 89

/* typed fan auto on|off (hidden) */
/* ENUM: CMD1.index 90 fan_auto_on established */
#define L1P1C_CMD1_INDEX_FAN_AUTO_ON 90

/* typed help|hlp */
/* ENUM: CMD1.index 91 help established */
#define L1P1C_CMD1_INDEX_HELP 91

/* typed help|hlp <word> */
/* ENUM: CMD1.index 92 help_word established */
#define L1P1C_CMD1_INDEX_HELP_WORD 92

/* typed help list hidden (hidden) */
/* ENUM: CMD1.index 93 help_list_hidden established */
#define L1P1C_CMD1_INDEX_HELP_LIST_HIDDEN 93

/* typed help <word> hidden (hidden) */
/* ENUM: CMD1.index 94 help_word_hidden established */
#define L1P1C_CMD1_INDEX_HELP_WORD_HIDDEN 94

/* typed history|hist */
/* ENUM: CMD1.index 95 history established */
#define L1P1C_CMD1_INDEX_HISTORY 95

/* typed l1dbg */
/* ENUM: CMD1.index 96 l1dbg established */
#define L1P1C_CMD1_INDEX_L1DBG 96

/* typed l1dbg syscom <number> */
/* ENUM: CMD1.index 97 l1dbg_sys_link_number established */
#define L1P1C_CMD1_INDEX_L1DBG_SYS_LINK_NUMBER 97

/* typed l1dbg syscom <number> <number> */
/* ENUM: CMD1.index 98 l1dbg_sys_link_number_number established */
#define L1P1C_CMD1_INDEX_L1DBG_SYS_LINK_NUMBER_NUMBER 98

/* typed l1dbg irtr <number> */
/* ENUM: CMD1.index 99 l1dbg_irtr_number established */
#define L1P1C_CMD1_INDEX_L1DBG_IRTR_NUMBER 99

/* typed l1dbg env <number> */
/* ENUM: CMD1.index 100 l1dbg_env_number established */
#define L1P1C_CMD1_INDEX_L1DBG_ENV_NUMBER 100

/* typed l1dbg env off */
/* ENUM: CMD1.index 101 l1dbg_env_off established */
#define L1P1C_CMD1_INDEX_L1DBG_ENV_OFF 101

/* typed l1dbg port on|off */
/* ENUM: CMD1.index 102 l1dbg_port_on established */
#define L1P1C_CMD1_INDEX_L1DBG_PORT_ON 102

/* typed l1dbg i2c on|off */
/* ENUM: CMD1.index 103 l1dbg_i2c_on established */
#define L1P1C_CMD1_INDEX_L1DBG_I2C_ON 103

/* typed l1dbg margin|mgn on|off */
/* ENUM: CMD1.index 104 l1dbg_margin_on established */
#define L1P1C_CMD1_INDEX_L1DBG_MARGIN_ON 104

/* typed l1dbg console on|off */
/* ENUM: CMD1.index 105 l1dbg_console_on established */
#define L1P1C_CMD1_INDEX_L1DBG_CONSOLE_ON 105

/* typed l1dbg mempanic on|off */
/* ENUM: CMD1.index 106 l1dbg_mempanic_on established */
#define L1P1C_CMD1_INDEX_L1DBG_MEMPANIC_ON 106

/* typed l1dbg pppdump on|off */
/* ENUM: CMD1.index 107 l1dbg_pppdump_on established */
#define L1P1C_CMD1_INDEX_L1DBG_PPPDUMP_ON 107

/* typed l1dbg pppdelay <number> */
/* ENUM: CMD1.index 108 l1dbg_pppdelay_number established */
#define L1P1C_CMD1_INDEX_L1DBG_PPPDELAY_NUMBER 108

/* typed l1dbg promreq|req on|off */
/* ENUM: CMD1.index 109 l1dbg_promreq_on established */
#define L1P1C_CMD1_INDEX_L1DBG_PROMREQ_ON 109

/* typed l1dbg qsusp <number> <number> */
/* ENUM: CMD1.index 110 l1dbg_qsusp_number_number established */
#define L1P1C_CMD1_INDEX_L1DBG_QSUSP_NUMBER_NUMBER 110

/* typed l1dbg printf on|off */
/* ENUM: CMD1.index 111 l1dbg_printf_on established */
#define L1P1C_CMD1_INDEX_L1DBG_PRINTF_ON 111

/* typed l1dbg flashignore <number> <number> */
/* ENUM: CMD1.index 112 l1dbg_flashignore_number_number established */
#define L1P1C_CMD1_INDEX_L1DBG_FLASHIGNORE_NUMBER_NUMBER 112

/* typed l1dbg misc <number> (hidden) */
/* ENUM: CMD1.index 113 l1dbg_misc_number established */
#define L1P1C_CMD1_INDEX_L1DBG_MISC_NUMBER 113

/* typed link */
/* ENUM: CMD1.index 114 link established */
#define L1P1C_CMD1_INDEX_LINK 114

/* typed link verbose|v */
/* ENUM: CMD1.index 115 link_verbose established */
#define L1P1C_CMD1_INDEX_LINK_VERBOSE 115

/* typed log */
/* ENUM: CMD1.index 116 log established */
#define L1P1C_CMD1_INDEX_LOG 116

/* typed log clear|reset */
/* ENUM: CMD1.index 117 log_clear established */
#define L1P1C_CMD1_INDEX_LOG_CLEAR 117

/* typed log insert <command> */
/* ENUM: CMD1.index 118 log_insert_command established */
#define L1P1C_CMD1_INDEX_LOG_INSERT_COMMAND 118

/* typed ioport|ioprt */
/* ENUM: CMD1.index 119 ioport established */
#define L1P1C_CMD1_INDEX_IOPORT 119

/* typed ioport|ioprt 400|600 */
/* ENUM: CMD1.index 120 ioport_400 established */
#define L1P1C_CMD1_INDEX_IOPORT_400 120

/* typed ioport|ioprt a 400|600 */
/* ENUM: CMD1.index 121 ioport_a_400 established */
#define L1P1C_CMD1_INDEX_IOPORT_A_400 121

/* typed ioport|ioprt b 400|600 */
/* ENUM: CMD1.index 122 ioport_b_400 established */
#define L1P1C_CMD1_INDEX_IOPORT_B_400 122

/* typed ioport|ioprt force 400|600 (hidden) */
/* ENUM: CMD1.index 123 ioport_force_400 established */
#define L1P1C_CMD1_INDEX_IOPORT_FORCE_400 123

/* typed ioport|ioprt force alt1 (hidden) */
/* ENUM: CMD1.index 124 ioport_force_alt1 established */
#define L1P1C_CMD1_INDEX_IOPORT_FORCE_ALT1 124

/* typed ioport|ioprt force alt2 (hidden) */
/* ENUM: CMD1.index 125 ioport_force_alt2 established */
#define L1P1C_CMD1_INDEX_IOPORT_FORCE_ALT2 125

/* typed ioport|ioprt clksrc a */
/* ENUM: CMD1.index 126 ioport_clksrc_a established */
#define L1P1C_CMD1_INDEX_IOPORT_CLKSRC_A 126

/* typed ioport|ioprt clksrc b */
/* ENUM: CMD1.index 127 ioport_clksrc_b established */
#define L1P1C_CMD1_INDEX_IOPORT_CLKSRC_B 127

/* typed istat memory|m */
/* ENUM: CMD1.index 128 istat_memory established */
#define L1P1C_CMD1_INDEX_ISTAT_MEMORY 128

/* typed istat queues|q */
/* ENUM: CMD1.index 129 istat_queues established */
#define L1P1C_CMD1_INDEX_ISTAT_QUEUES 129

/* typed istat tasks|t */
/* ENUM: CMD1.index 130 istat_tasks established */
#define L1P1C_CMD1_INDEX_ISTAT_TASKS 130

/* typed istat pmalloc|p */
/* ENUM: CMD1.index 131 istat_pmalloc established */
#define L1P1C_CMD1_INDEX_ISTAT_PMALLOC 131

/* typed istat dump|d <word> */
/* ENUM: CMD1.index 132 istat_dump_word established */
#define L1P1C_CMD1_INDEX_ISTAT_DUMP_WORD 132

/* typed b2b (hidden) */
/* ENUM: CMD1.index 133 b2b established */
#define L1P1C_CMD1_INDEX_B2B 133

/* typed l1 */
/* ENUM: CMD1.index 134 l1 established */
#define L1P1C_CMD1_INDEX_L1 134

/* typed leds */
/* ENUM: CMD1.index 135 leds established */
#define L1P1C_CMD1_INDEX_LEDS 135

/* typed margin|mgn */
/* ENUM: CMD1.index 136 margin established */
#define L1P1C_CMD1_INDEX_MARGIN 136

/* typed margin|mgn default|d|low|l|norm|n|high|h */
/* ENUM: CMD1.index 137 margin_default established */
#define L1P1C_CMD1_INDEX_MARGIN_DEFAULT 137

/* typed margin|mgn gbrick default|d|low|l|norm|n|high|h */
/* ENUM: CMD1.index 138 margin_gbrick_default established */
#define L1P1C_CMD1_INDEX_MARGIN_GBRICK_DEFAULT 138

/* typed margin|mgn <number> default|d|low|l|norm|n|high|h */
/* ENUM: CMD1.index 139 margin_number_default established */
#define L1P1C_CMD1_INDEX_MARGIN_NUMBER_DEFAULT 139

/* typed margin|mgn <number> <number> */
/* ENUM: CMD1.index 140 margin_number_number established */
#define L1P1C_CMD1_INDEX_MARGIN_NUMBER_NUMBER 140

/* typed margin|mgn dimm v|voltage <word> */
/* ENUM: CMD1.index 141 margin_dimm_v_word established */
#define L1P1C_CMD1_INDEX_MARGIN_DIMM_V_WORD 141

/* typed margin|mgn pci norm2hi */
/* ENUM: CMD1.index 142 margin_pci_norm2hi established */
#define L1P1C_CMD1_INDEX_MARGIN_PCI_NORM2HI 142

/* typed margin|mgn pci norm2hi on|off */
/* ENUM: CMD1.index 143 margin_pci_norm2hi_on established */
#define L1P1C_CMD1_INDEX_MARGIN_PCI_NORM2HI_ON 143

/* typed network */
/* ENUM: CMD1.index 144 network established */
#define L1P1C_CMD1_INDEX_NETWORK 144

/* typed network usb|422|serial */
/* ENUM: CMD1.index 145 network_usb established */
#define L1P1C_CMD1_INDEX_NETWORK_USB 145

/* typed network autodetect|auto on|off */
/* ENUM: CMD1.index 146 network_autodetect_on established */
#define L1P1C_CMD1_INDEX_NETWORK_AUTODETECT_ON 146

/* typed pimm */
/* ENUM: CMD1.index 147 pimm established */
#define L1P1C_CMD1_INDEX_PIMM 147

/* typed pimm auto (hidden) */
/* ENUM: CMD1.index 148 pimm_auto established */
#define L1P1C_CMD1_INDEX_PIMM_AUTO 148

/* typed pimm clksrc ext|external (hidden) */
/* ENUM: CMD1.index 149 pimm_clksrc_ext established */
#define L1P1C_CMD1_INDEX_PIMM_CLKSRC_EXT 149

/* typed pimm clksrc int|internal (hidden) */
/* ENUM: CMD1.index 150 pimm_clksrc_int established */
#define L1P1C_CMD1_INDEX_PIMM_CLKSRC_INT 150

/* typed pimm clkmode async (hidden) */
/* ENUM: CMD1.index 151 pimm_clkmode_async established */
#define L1P1C_CMD1_INDEX_PIMM_CLKMODE_ASYNC 151

/* typed pimm clkmode sync (hidden) */
/* ENUM: CMD1.index 152 pimm_clkmode_sync established */
#define L1P1C_CMD1_INDEX_PIMM_CLKMODE_SYNC 152

/* typed port|prt */
/* ENUM: CMD1.index 153 port established */
#define L1P1C_CMD1_INDEX_PORT 153

/* typed power|pwr */
/* ENUM: CMD1.index 154 power established */
#define L1P1C_CMD1_INDEX_POWER 154

/* typed power|pwr check */
/* ENUM: CMD1.index 155 power_check established */
#define L1P1C_CMD1_INDEX_POWER_CHECK 155

/* typed power|pwr vrm */
/* ENUM: CMD1.index 156 power_vrm established */
#define L1P1C_CMD1_INDEX_POWER_VRM 156

/* typed power|pwr up|u */
/* ENUM: CMD1.index 157 power_up established */
#define L1P1C_CMD1_INDEX_POWER_UP 157

/* typed power|pwr up|u hold */
/* ENUM: CMD1.index 158 power_up_hold established */
#define L1P1C_CMD1_INDEX_POWER_UP_HOLD 158

/* typed power|pwr down|d */
/* ENUM: CMD1.index 159 power_down established */
#define L1P1C_CMD1_INDEX_POWER_DOWN 159

/* typed reset|rst */
/* ENUM: CMD1.index 160 reset established */
#define L1P1C_CMD1_INDEX_RESET 160

/* typed reset|rst hold */
/* ENUM: CMD1.index 161 reset_hold established */
#define L1P1C_CMD1_INDEX_RESET_HOLD 161

/* typed nmi */
/* ENUM: CMD1.index 162 nmi established */
#define L1P1C_CMD1_INDEX_NMI 162

/* typed nmi <number> */
/* ENUM: CMD1.index 163 nmi_number established */
#define L1P1C_CMD1_INDEX_NMI_NUMBER 163

/* typed nmi <rack_slot> <word> */
/* ENUM: CMD1.index 164 nmi_rack_slot_word established */
#define L1P1C_CMD1_INDEX_NMI_RACK_SLOT_WORD 164

/* typed softreset|softrst */
/* ENUM: CMD1.index 165 softreset established */
#define L1P1C_CMD1_INDEX_SOFTRESET 165

/* typed softreset|softrst <number> */
/* ENUM: CMD1.index 166 softreset_number established */
#define L1P1C_CMD1_INDEX_SOFTRESET_NUMBER 166

/* typed select|sel */
/* ENUM: CMD1.index 167 select established */
#define L1P1C_CMD1_INDEX_SELECT 167

/* typed select|sel <number> <number> */
/* ENUM: CMD1.index 168 select_number_number established */
#define L1P1C_CMD1_INDEX_SELECT_NUMBER_NUMBER 168

/* typed select|sel subchannel|sub|s console|con */
/* ENUM: CMD1.index 169 select_subchannel_console established */
#define L1P1C_CMD1_INDEX_SELECT_SUBCHANNEL_CONSOLE 169

/* typed select|sel subchannel|sub|s <number> */
/* ENUM: CMD1.index 170 select_subchannel_number established */
#define L1P1C_CMD1_INDEX_SELECT_SUBCHANNEL_NUMBER 170

/* typed select|sel subchannel|sub|s a|b|c|d */
/* ENUM: CMD1.index 171 select_subchannel_a established */
#define L1P1C_CMD1_INDEX_SELECT_SUBCHANNEL_A 171

/* typed select|sel subchannel|sub|s <word> */
/* ENUM: CMD1.index 172 select_subchannel_word established */
#define L1P1C_CMD1_INDEX_SELECT_SUBCHANNEL_WORD 172

/* typed select|sel enable */
/* ENUM: CMD1.index 173 select_enable established */
#define L1P1C_CMD1_INDEX_SELECT_ENABLE 173

/* typed select|sel disable */
/* ENUM: CMD1.index 174 select_disable established */
#define L1P1C_CMD1_INDEX_SELECT_DISABLE 174

/* typed select|sel filter on */
/* ENUM: CMD1.index 175 select_filter_on established */
#define L1P1C_CMD1_INDEX_SELECT_FILTER_ON 175

/* typed select|sel filter off */
/* ENUM: CMD1.index 176 select_filter_off established */
#define L1P1C_CMD1_INDEX_SELECT_FILTER_OFF 176

/* typed serial */
/* ENUM: CMD1.index 177 serial established */
#define L1P1C_CMD1_INDEX_SERIAL 177

/* typed serial verify */
/* ENUM: CMD1.index 178 serial_verify established */
#define L1P1C_CMD1_INDEX_SERIAL_VERIFY 178

/* typed serial all */
/* ENUM: CMD1.index 179 serial_all established */
#define L1P1C_CMD1_INDEX_SERIAL_ALL 179

/* typed serial all v|verbose */
/* ENUM: CMD1.index 180 serial_all_v established */
#define L1P1C_CMD1_INDEX_SERIAL_ALL_V 180

/* typed serial dimm */
/* ENUM: CMD1.index 181 serial_dimm established */
#define L1P1C_CMD1_INDEX_SERIAL_DIMM 181

/* typed serial dimm v|verbose */
/* ENUM: CMD1.index 182 serial_dimm_v established */
#define L1P1C_CMD1_INDEX_SERIAL_DIMM_V 182

/* typed serial clear */
/* ENUM: CMD1.index 183 serial_clear established */
#define L1P1C_CMD1_INDEX_SERIAL_CLEAR 183

/* typed serial set <word> (hidden) */
/* ENUM: CMD1.index 184 serial_set_word established */
#define L1P1C_CMD1_INDEX_SERIAL_SET_WORD 184

/* typed serial <word> <word> <word> <word> */
/* ENUM: CMD1.index 185 serial_word_word_word_word established */
#define L1P1C_CMD1_INDEX_SERIAL_WORD_WORD_WORD_WORD 185

/* typed serial security off <word> <word> <word> <word> (hidden) */
/* ENUM: CMD1.index 186 serial_security_off_word_word_word_word established */
#define L1P1C_CMD1_INDEX_SERIAL_SECURITY_OFF_WORD_WORD_WORD_WORD 186

/* typed serial security on */
/* ENUM: CMD1.index 187 serial_security_on established */
#define L1P1C_CMD1_INDEX_SERIAL_SECURITY_ON 187

/* typed serial brick disable (hidden) */
/* ENUM: CMD1.index 188 serial_brick_disable established */
#define L1P1C_CMD1_INDEX_SERIAL_BRICK_DISABLE 188

/* typed serial brick disable confirm (hidden) */
/* ENUM: CMD1.index 189 serial_brick_disable_confirm established */
#define L1P1C_CMD1_INDEX_SERIAL_BRICK_DISABLE_CONFIRM 189

/* typed sysstate */
/* ENUM: CMD1.index 190 sysstate established */
#define L1P1C_CMD1_INDEX_SYSSTATE 190

/* typed eeprom */
/* ENUM: CMD1.index 191 eeprom established */
#define L1P1C_CMD1_INDEX_EEPROM 191

/* typed eeprom <number> <number> <number> */
/* ENUM: CMD1.index 192 eeprom_number_number_number established */
#define L1P1C_CMD1_INDEX_EEPROM_NUMBER_NUMBER_NUMBER 192

/* typed eeprom Fuel write default */
/* ENUM: CMD1.index 193 eeprom_fuel_write_default established */
#define L1P1C_CMD1_INDEX_EEPROM_FUEL_WRITE_DEFAULT 193

/* typed eeprom dimm unsign (hidden) */
/* ENUM: CMD1.index 194 eeprom_dimm_unsign established */
#define L1P1C_CMD1_INDEX_EEPROM_DIMM_UNSIGN 194

/* typed uart */
/* ENUM: CMD1.index 195 uart established */
#define L1P1C_CMD1_INDEX_UART 195

/* typed uart syscom|junkbus|jb|bedrock reset|r */
/* ENUM: CMD1.index 196 uart_sys_link_reset established */
#define L1P1C_CMD1_INDEX_UART_SYS_LINK_RESET 196

/* typed usb */
/* ENUM: CMD1.index 197 usb established */
#define L1P1C_CMD1_INDEX_USB 197

/* typed router|rtr */
/* ENUM: CMD1.index 198 router established */
#define L1P1C_CMD1_INDEX_ROUTER 198

/* typed router|rtr meta */
/* ENUM: CMD1.index 199 router_meta established */
#define L1P1C_CMD1_INDEX_ROUTER_META 199

/* typed router|rtr repeater|rep */
/* ENUM: CMD1.index 200 router_repeater established */
#define L1P1C_CMD1_INDEX_ROUTER_REPEATER 200

/* typed router|rtr ordinary|ord */
/* ENUM: CMD1.index 201 router_ordinary established */
#define L1P1C_CMD1_INDEX_ROUTER_ORDINARY 201

/* typed router|rtr 6 port */
/* ENUM: CMD1.index 202 router_6_port established */
#define L1P1C_CMD1_INDEX_ROUTER_6_PORT 202

/* typed router|rtr 6 port confirm (hidden) */
/* ENUM: CMD1.index 203 router_6_port_confirm established */
#define L1P1C_CMD1_INDEX_ROUTER_6_PORT_CONFIRM 203

/* typed router|rtr 8 port */
/* ENUM: CMD1.index 204 router_8_port established */
#define L1P1C_CMD1_INDEX_ROUTER_8_PORT 204

/* typed router|rtr 8 port <word> <word> <word> <word> */
/* ENUM: CMD1.index 205 router_8_port_word_word_word_word established */
#define L1P1C_CMD1_INDEX_ROUTER_8_PORT_WORD_WORD_WORD_WORD 205

/* typed router|rtr service <word> <word> <word> <word> */
/* ENUM: CMD1.index 206 router_service_word_word_word_word established */
#define L1P1C_CMD1_INDEX_ROUTER_SERVICE_WORD_WORD_WORD_WORD 206

/* typed router|rtr service off */
/* ENUM: CMD1.index 207 router_service_off established */
#define L1P1C_CMD1_INDEX_ROUTER_SERVICE_OFF 207

/* typed router|rtr spare */
/* ENUM: CMD1.index 208 router_spare established */
#define L1P1C_CMD1_INDEX_ROUTER_SPARE 208

/* typed service <word> <word> <word> <word> */
/* ENUM: CMD1.index 209 service_word_word_word_word established */
#define L1P1C_CMD1_INDEX_SERVICE_WORD_WORD_WORD_WORD 209

/* typed service off */
/* ENUM: CMD1.index 210 service_off established */
#define L1P1C_CMD1_INDEX_SERVICE_OFF 210

/* typed date tz */
/* ENUM: CMD1.index 211 date_tz established */
#define L1P1C_CMD1_INDEX_DATE_TZ 211

/* typed date tz <word> */
/* ENUM: CMD1.index 212 date_tz_word established */
#define L1P1C_CMD1_INDEX_DATE_TZ_WORD 212

/* typed date */
/* ENUM: CMD1.index 213 date established */
#define L1P1C_CMD1_INDEX_DATE 213

/* typed date raw */
/* ENUM: CMD1.index 214 date_raw established */
#define L1P1C_CMD1_INDEX_DATE_RAW 214

/* typed date <word> */
/* ENUM: CMD1.index 215 date_word established */
#define L1P1C_CMD1_INDEX_DATE_WORD 215

/* typed date <word> <word> */
/* ENUM: CMD1.index 216 date_word_word established */
#define L1P1C_CMD1_INDEX_DATE_WORD_WORD 216

/* typed nvram reset */
/* ENUM: CMD1.index 217 nvram_reset established */
#define L1P1C_CMD1_INDEX_NVRAM_RESET 217

/* typed nvram read <number> <number> (hidden) */
/* ENUM: CMD1.index 218 nvram_read_number_number established */
#define L1P1C_CMD1_INDEX_NVRAM_READ_NUMBER_NUMBER 218

/* typed nvram write <number> <number> (hidden) */
/* ENUM: CMD1.index 219 nvram_write_number_number established */
#define L1P1C_CMD1_INDEX_NVRAM_WRITE_NUMBER_NUMBER 219

/* typed nvram scratch (hidden) */
/* ENUM: CMD1.index 220 nvram_scratch established */
#define L1P1C_CMD1_INDEX_NVRAM_SCRATCH 220

/* typed nvram scratch <number> (hidden) */
/* ENUM: CMD1.index 221 nvram_scratch_number established */
#define L1P1C_CMD1_INDEX_NVRAM_SCRATCH_NUMBER 221

/* typed l2 <command> (hidden) */
/* ENUM: CMD1.index 222 l2_command established */
#define L1P1C_CMD1_INDEX_L2_COMMAND 222

/* typed security verify 3400 */
/* ENUM: CMD1.index 223 security_verify_3400 established */
#define L1P1C_CMD1_INDEX_SECURITY_VERIFY_3400 223

/* typed security verify 3800 */
/* ENUM: CMD1.index 224 security_verify_3800 established */
#define L1P1C_CMD1_INDEX_SECURITY_VERIFY_3800 224

/* typed flash default reset */
/* ENUM: CMD1.index 225 flash_default_reset established */
#define L1P1C_CMD1_INDEX_FLASH_DEFAULT_RESET 225

/* typed flash default a */
/* ENUM: CMD1.index 226 flash_default_a established */
#define L1P1C_CMD1_INDEX_FLASH_DEFAULT_A 226

/* typed flash default b */
/* ENUM: CMD1.index 227 flash_default_b established */
#define L1P1C_CMD1_INDEX_FLASH_DEFAULT_B 227

/* typed flash default current */
/* ENUM: CMD1.index 228 flash_default_current established */
#define L1P1C_CMD1_INDEX_FLASH_DEFAULT_CURRENT 228

/* typed flash default old */
/* ENUM: CMD1.index 229 flash_default_old established */
#define L1P1C_CMD1_INDEX_FLASH_DEFAULT_OLD 229

/* typed flash default new */
/* ENUM: CMD1.index 230 flash_default_new established */
#define L1P1C_CMD1_INDEX_FLASH_DEFAULT_NEW 230

/* typed flash status */
/* ENUM: CMD1.index 231 flash_status established */
#define L1P1C_CMD1_INDEX_FLASH_STATUS 231

/* typed flash status v|verbose */
/* ENUM: CMD1.index 232 flash_status_v established */
#define L1P1C_CMD1_INDEX_FLASH_STATUS_V 232

/* typed reboot_l1 */
/* ENUM: CMD1.index 233 reboot_l1 established */
#define L1P1C_CMD1_INDEX_REBOOT_L1 233

/* typed reboot_l1 a */
/* ENUM: CMD1.index 234 reboot_l1_a established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_A 234

/* typed reboot_l1 b */
/* ENUM: CMD1.index 235 reboot_l1_b established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_B 235

/* typed reboot_l1 current */
/* ENUM: CMD1.index 236 reboot_l1_current established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_CURRENT 236

/* typed reboot_l1 old */
/* ENUM: CMD1.index 237 reboot_l1_old established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_OLD 237

/* typed reboot_l1 new */
/* ENUM: CMD1.index 238 reboot_l1_new established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_NEW 238

/* typed reboot_l1 other */
/* ENUM: CMD1.index 239 reboot_l1_other established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_OTHER 239

/* typed reboot_l1 addr <number> */
/* ENUM: CMD1.index 240 reboot_l1_addr_number established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_ADDR_NUMBER 240

/* typed reboot_l1 raw <number> */
/* ENUM: CMD1.index 241 reboot_l1_raw_number established */
#define L1P1C_CMD1_INDEX_REBOOT_L1_RAW_NUMBER 241

/* typed version|ver */
/* ENUM: CMD1.index 242 version established */
#define L1P1C_CMD1_INDEX_VERSION 242

/* typed pbay */
/* ENUM: CMD1.index 243 pbay established */
#define L1P1C_CMD1_INDEX_PBAY 243

/* typed pbay version|ver */
/* ENUM: CMD1.index 244 pbay_version established */
#define L1P1C_CMD1_INDEX_PBAY_VERSION 244

/* typed pbay dps */
/* ENUM: CMD1.index 245 pbay_dps established */
#define L1P1C_CMD1_INDEX_PBAY_DPS 245

/* typed pbay dps <word> */
/* ENUM: CMD1.index 246 pbay_dps_word established */
#define L1P1C_CMD1_INDEX_PBAY_DPS_WORD 246

/* typed pbay dcport|dcprt */
/* ENUM: CMD1.index 247 pbay_dcport established */
#define L1P1C_CMD1_INDEX_PBAY_DCPORT 247

/* typed pbay dcport|dcprt <word> */
/* ENUM: CMD1.index 248 pbay_dcport_word established */
#define L1P1C_CMD1_INDEX_PBAY_DCPORT_WORD 248

/* typed pbay fru|f */
/* ENUM: CMD1.index 249 pbay_fru established */
#define L1P1C_CMD1_INDEX_PBAY_FRU 249

/* typed pbay fru|f <word> */
/* ENUM: CMD1.index 250 pbay_fru_word established */
#define L1P1C_CMD1_INDEX_PBAY_FRU_WORD 250

/* typed pbay reset|rst */
/* ENUM: CMD1.index 251 pbay_reset established */
#define L1P1C_CMD1_INDEX_PBAY_RESET 251

/* typed pbay env */
/* ENUM: CMD1.index 252 pbay_env established */
#define L1P1C_CMD1_INDEX_PBAY_ENV 252

/* typed pbay env on */
/* ENUM: CMD1.index 253 pbay_env_on established */
#define L1P1C_CMD1_INDEX_PBAY_ENV_ON 253

/* typed pbay env off */
/* ENUM: CMD1.index 254 pbay_env_off established */
#define L1P1C_CMD1_INDEX_PBAY_ENV_OFF 254

/* typed pbay init */
/* ENUM: CMD1.index 255 pbay_init established */
#define L1P1C_CMD1_INDEX_PBAY_INIT 255

/* typed pbay serial */
/* ENUM: CMD1.index 256 pbay_serial established */
#define L1P1C_CMD1_INDEX_PBAY_SERIAL 256

/* typed test|tst i2c */
/* ENUM: CMD1.index 257 test_i2c established */
#define L1P1C_CMD1_INDEX_TEST_I2C 257

/* typed test|tst i2c <number> */
/* ENUM: CMD1.index 258 test_i2c_number established */
#define L1P1C_CMD1_INDEX_TEST_I2C_NUMBER 258

/* typed test|tst ioexp set <number> <word> */
/* ENUM: CMD1.index 259 test_ioexp_set_number_word established */
#define L1P1C_CMD1_INDEX_TEST_IOEXP_SET_NUMBER_WORD 259

/* typed test|tst ioexp get <number> */
/* ENUM: CMD1.index 260 test_ioexp_get_number established */
#define L1P1C_CMD1_INDEX_TEST_IOEXP_GET_NUMBER 260

/* typed test|tst ioexp get all */
/* ENUM: CMD1.index 261 test_ioexp_get_all established */
#define L1P1C_CMD1_INDEX_TEST_IOEXP_GET_ALL 261

/* typed test|tst intr */
/* ENUM: CMD1.index 262 test_intr established */
#define L1P1C_CMD1_INDEX_TEST_INTR 262

/* typed test|tst display */
/* ENUM: CMD1.index 263 test_display established */
#define L1P1C_CMD1_INDEX_TEST_DISPLAY 263

/* typed test|tst exception|exc mem */
/* ENUM: CMD1.index 264 test_exception_mem established */
#define L1P1C_CMD1_INDEX_TEST_EXCEPTION_MEM 264

/* typed test|tst exception|exc stack */
/* ENUM: CMD1.index 265 test_exception_stack established */
#define L1P1C_CMD1_INDEX_TEST_EXCEPTION_STACK 265

/* typed test|tst exception|exc wdog */
/* ENUM: CMD1.index 266 test_exception_wdog established */
#define L1P1C_CMD1_INDEX_TEST_EXCEPTION_WDOG 266

/* typed test|tst exception|exc fatal */
/* ENUM: CMD1.index 267 test_exception_fatal established */
#define L1P1C_CMD1_INDEX_TEST_EXCEPTION_FATAL 267

/* typed scan reset */
/* ENUM: CMD1.index 268 scan_reset established */
#define L1P1C_CMD1_INDEX_SCAN_RESET 268

/* typed scan reset hard */
/* ENUM: CMD1.index 269 scan_reset_hard established */
#define L1P1C_CMD1_INDEX_SCAN_RESET_HARD 269

/* typed scan reset soft */
/* ENUM: CMD1.index 270 scan_reset_soft established */
#define L1P1C_CMD1_INDEX_SCAN_RESET_SOFT 270

/* typed scan reset both */
/* ENUM: CMD1.index 271 scan_reset_both established */
#define L1P1C_CMD1_INDEX_SCAN_RESET_BOTH 271

/* typed scan sel <number> <number> */
/* ENUM: CMD1.index 272 scan_sel_number_number established */
#define L1P1C_CMD1_INDEX_SCAN_SEL_NUMBER_NUMBER 272

/* typed scan sel <number> <number> <number> */
/* ENUM: CMD1.index 273 scan_sel_number_number_number established */
#define L1P1C_CMD1_INDEX_SCAN_SEL_NUMBER_NUMBER_NUMBER 273

/* typed scan sel <number> <number> <number> <number> */
/* ENUM: CMD1.index 274 scan_sel_number_number_number_number established */
#define L1P1C_CMD1_INDEX_SCAN_SEL_NUMBER_NUMBER_NUMBER_NUMBER 274

/* typed scan ids <number> */
/* ENUM: CMD1.index 275 scan_ids_number established */
#define L1P1C_CMD1_INDEX_SCAN_IDS_NUMBER 275

/* typed scan set trst 0 */
/* ENUM: CMD1.index 276 scan_set_trst_0 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TRST_0 276

/* typed scan set trst 1 */
/* ENUM: CMD1.index 277 scan_set_trst_1 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TRST_1 277

/* typed scan set tck 0 */
/* ENUM: CMD1.index 278 scan_set_tck_0 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TCK_0 278

/* typed scan set tck 1 */
/* ENUM: CMD1.index 279 scan_set_tck_1 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TCK_1 279

/* typed scan set tms 0 */
/* ENUM: CMD1.index 280 scan_set_tms_0 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TMS_0 280

/* typed scan set tms 1 */
/* ENUM: CMD1.index 281 scan_set_tms_1 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TMS_1 281

/* typed scan set tdi 0 */
/* ENUM: CMD1.index 282 scan_set_tdi_0 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TDI_0 282

/* typed scan set tdi 1 */
/* ENUM: CMD1.index 283 scan_set_tdi_1 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_TDI_1 283

/* typed scan set psi 0 */
/* ENUM: CMD1.index 284 scan_set_psi_0 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_PSI_0 284

/* typed scan set psi 1 */
/* ENUM: CMD1.index 285 scan_set_psi_1 established */
#define L1P1C_CMD1_INDEX_SCAN_SET_PSI_1 285

/* typed scan get */
/* ENUM: CMD1.index 286 scan_get established */
#define L1P1C_CMD1_INDEX_SCAN_GET 286

/* typed scan debug <number> */
/* ENUM: CMD1.index 287 scan_debug_number established */
#define L1P1C_CMD1_INDEX_SCAN_DEBUG_NUMBER 287

/* typed scan debug */
/* ENUM: CMD1.index 288 scan_debug established */
#define L1P1C_CMD1_INDEX_SCAN_DEBUG 288

/* typed scan imp set dllbyp <number> */
/* ENUM: CMD1.index 289 scan_imp_set_dllbyp_number established */
#define L1P1C_CMD1_INDEX_SCAN_IMP_SET_DLLBYP_NUMBER 289

/* typed scan imp set <number> */
/* ENUM: CMD1.index 290 scan_imp_set_number established */
#define L1P1C_CMD1_INDEX_SCAN_IMP_SET_NUMBER 290

/* typed scan imp unlock */
/* ENUM: CMD1.index 291 scan_imp_unlock established */
#define L1P1C_CMD1_INDEX_SCAN_IMP_UNLOCK 291

/* typed scan test <number> */
/* ENUM: CMD1.index 292 scan_test_number established */
#define L1P1C_CMD1_INDEX_SCAN_TEST_NUMBER 292

/* typed fru|pci|prom|node */
/* ENUM: CMD1.index 293 fru established */
#define L1P1C_CMD1_INDEX_FRU 293

/* typed fru|pci|prom|node <command> */
/* ENUM: CMD1.index 294 fru_command established */
#define L1P1C_CMD1_INDEX_FRU_COMMAND 294

#endif
