/*
 * L1P5_CONSOLE_DECODE.h - console command tables of ColdFire L1 firmware package 5
 *
 * The package registers 3 command tables. Each CMD<t>.index below is the position of an
 * entry in table t, counted from zero, and table t is the t-th table in the order of the
 * registration calls in the image. That is not necessarily the order the parser tries them in;
 * the order of the calls at run time is not read here. Entry layout and argument kinds are in
 * the shared console header. The typed pattern is repeated above each ENUM line; hidden entries are
 * marked. Labels are the first alternative of each word joined by underscores, with the
 * system-link word spelled sys_link; positions of entries that share a pattern are told apart by
 * a numeric suffix.
 */
#ifndef L1P5_CONSOLE_DECODE_H
#define L1P5_CONSOLE_DECODE_H

/* FIELD: CMD0.index 8:0 established */
#define L1P5C_CMD0_INDEX_SHIFT 0
#define L1P5C_CMD0_INDEX_MASK 0x1ffu

/* typed * <command> */
/* ENUM: CMD0.index 0 all_command established */
#define L1P5C_CMD0_INDEX_ALL_COMMAND 0

/* typed <rack_slot> <command> */
/* ENUM: CMD0.index 1 rack_slot_command established */
#define L1P5C_CMD0_INDEX_RACK_SLOT_COMMAND 1

/* typed autopower|apwr */
/* ENUM: CMD0.index 2 autopower established */
#define L1P5C_CMD0_INDEX_AUTOPOWER 2

/* typed autopower|apwr on */
/* ENUM: CMD0.index 3 autopower_on established */
#define L1P5C_CMD0_INDEX_AUTOPOWER_ON 3

/* typed autopower|apwr off */
/* ENUM: CMD0.index 4 autopower_off established */
#define L1P5C_CMD0_INDEX_AUTOPOWER_OFF 4

/* typed autopower|apwr abort */
/* ENUM: CMD0.index 5 autopower_abort established */
#define L1P5C_CMD0_INDEX_AUTOPOWER_ABORT 5

/* typed syscom|junkbus|jb|bedrock */
/* ENUM: CMD0.index 6 sys_link established */
#define L1P5C_CMD0_INDEX_SYS_LINK 6

/* typed syscom|junkbus|jb|bedrock none (hidden) */
/* ENUM: CMD0.index 7 sys_link_none established */
#define L1P5C_CMD0_INDEX_SYS_LINK_NONE 7

/* typed syscom|junkbus|jb|bedrock ppp (hidden) */
/* ENUM: CMD0.index 8 sys_link_ppp established */
#define L1P5C_CMD0_INDEX_SYS_LINK_PPP 8

/* typed syscom|junkbus|jb|bedrock enable (hidden) */
/* ENUM: CMD0.index 9 sys_link_enable established */
#define L1P5C_CMD0_INDEX_SYS_LINK_ENABLE 9

/* typed syscom reset (hidden) */
/* ENUM: CMD0.index 10 sys_link_reset established */
#define L1P5C_CMD0_INDEX_SYS_LINK_RESET 10

/* typed syscom packets (hidden) */
/* ENUM: CMD0.index 11 sys_link_packets established */
#define L1P5C_CMD0_INDEX_SYS_LINK_PACKETS 11

/* typed brick */
/* ENUM: CMD0.index 12 brick established */
#define L1P5C_CMD0_INDEX_BRICK 12

/* typed brick type <word> */
/* ENUM: CMD0.index 13 brick_type_word established */
#define L1P5C_CMD0_INDEX_BRICK_TYPE_WORD 13

/* typed brick rack <decimal> */
/* ENUM: CMD0.index 14 brick_rack_decimal established */
#define L1P5C_CMD0_INDEX_BRICK_RACK_DECIMAL 14

/* typed brick slot|bay|upos <decimal> */
/* ENUM: CMD0.index 15 brick_slot_decimal established */
#define L1P5C_CMD0_INDEX_BRICK_SLOT_DECIMAL 15

/* typed brick rackslot|rs <decimal> <decimal> */
/* ENUM: CMD0.index 16 brick_rackslot_decimal_decimal established */
#define L1P5C_CMD0_INDEX_BRICK_RACKSLOT_DECIMAL_DECIMAL 16

/* typed brick rackslot|rs reset <number> <number> (hidden) */
/* ENUM: CMD0.index 17 brick_rackslot_reset_number_number established */
#define L1P5C_CMD0_INDEX_BRICK_RACKSLOT_RESET_NUMBER_NUMBER 17

/* typed brick partition|part|p none */
/* ENUM: CMD0.index 18 brick_partition_none established */
#define L1P5C_CMD0_INDEX_BRICK_PARTITION_NONE 18

/* typed brick partition|part <number> */
/* ENUM: CMD0.index 19 brick_partition_number established */
#define L1P5C_CMD0_INDEX_BRICK_PARTITION_NUMBER 19

/* typed partdb */
/* ENUM: CMD0.index 20 partdb established */
#define L1P5C_CMD0_INDEX_PARTDB 20

/* typed partdb enable|e|on */
/* ENUM: CMD0.index 21 partdb_enable established */
#define L1P5C_CMD0_INDEX_PARTDB_ENABLE 21

/* typed partdb disable|d|off */
/* ENUM: CMD0.index 22 partdb_disable established */
#define L1P5C_CMD0_INDEX_PARTDB_DISABLE 22

/* typed cpu */
/* ENUM: CMD0.index 23 cpu established */
#define L1P5C_CMD0_INDEX_CPU 23

/* typed cpu e|enable|d|disable */
/* ENUM: CMD0.index 24 cpu_e established */
#define L1P5C_CMD0_INDEX_CPU_E 24

/* typed cpu <word> e|enable|d|disable */
/* ENUM: CMD0.index 25 cpu_word_e established */
#define L1P5C_CMD0_INDEX_CPU_WORD_E 25

/* typed b2b <word> <command> */
/* ENUM: CMD0.index 26 b2b_word_command established */
#define L1P5C_CMD0_INDEX_B2B_WORD_COMMAND 26

/* typed config|cfg */
/* ENUM: CMD0.index 27 config established */
#define L1P5C_CMD0_INDEX_CONFIG 27

/* typed config|cfg v|verbose */
/* ENUM: CMD0.index 28 config_v established */
#define L1P5C_CMD0_INDEX_CONFIG_V 28

/* typed config|cfg reset */
/* ENUM: CMD0.index 29 config_reset established */
#define L1P5C_CMD0_INDEX_CONFIG_RESET 29

/* typed config|cfg ring */
/* ENUM: CMD0.index 30 config_ring established */
#define L1P5C_CMD0_INDEX_CONFIG_RING 30

/* typed debug */
/* ENUM: CMD0.index 31 debug established */
#define L1P5C_CMD0_INDEX_DEBUG 31

/* typed debug <number> */
/* ENUM: CMD0.index 32 debug_number established */
#define L1P5C_CMD0_INDEX_DEBUG_NUMBER 32

/* typed display|dsp */
/* ENUM: CMD0.index 33 display established */
#define L1P5C_CMD0_INDEX_DISPLAY 33

/* typed display|dsp <number> <command> */
/* ENUM: CMD0.index 34 display_number_command established */
#define L1P5C_CMD0_INDEX_DISPLAY_NUMBER_COMMAND 34

/* typed display|dsp power|pwr on */
/* ENUM: CMD0.index 35 display_power_on established */
#define L1P5C_CMD0_INDEX_DISPLAY_POWER_ON 35

/* typed display|dsp attention|attn on */
/* ENUM: CMD0.index 36 display_attention_on established */
#define L1P5C_CMD0_INDEX_DISPLAY_ATTENTION_ON 36

/* typed display|dsp failure|fail on */
/* ENUM: CMD0.index 37 display_failure_on established */
#define L1P5C_CMD0_INDEX_DISPLAY_FAILURE_ON 37

/* typed display|dsp power|pwr off */
/* ENUM: CMD0.index 38 display_power_off established */
#define L1P5C_CMD0_INDEX_DISPLAY_POWER_OFF 38

/* typed display|dsp attention|attn off */
/* ENUM: CMD0.index 39 display_attention_off established */
#define L1P5C_CMD0_INDEX_DISPLAY_ATTENTION_OFF 39

/* typed display|dsp failure|fail off */
/* ENUM: CMD0.index 40 display_failure_off established */
#define L1P5C_CMD0_INDEX_DISPLAY_FAILURE_OFF 40

/* typed button|btn */
/* ENUM: CMD0.index 41 button established */
#define L1P5C_CMD0_INDEX_BUTTON 41

/* typed button|btn all enable|en|e */
/* ENUM: CMD0.index 42 button_all_enable established */
#define L1P5C_CMD0_INDEX_BUTTON_ALL_ENABLE 42

/* typed button|btn all disable|dis|d */
/* ENUM: CMD0.index 43 button_all_disable established */
#define L1P5C_CMD0_INDEX_BUTTON_ALL_DISABLE 43

/* typed button|btn pwr|power enable|en|e */
/* ENUM: CMD0.index 44 button_pwr_enable established */
#define L1P5C_CMD0_INDEX_BUTTON_PWR_ENABLE 44

/* typed button|btn pwr|power disable|dis|d */
/* ENUM: CMD0.index 45 button_pwr_disable established */
#define L1P5C_CMD0_INDEX_BUTTON_PWR_DISABLE 45

/* typed button|btn rst|reset enable|en|e */
/* ENUM: CMD0.index 46 button_rst_enable established */
#define L1P5C_CMD0_INDEX_BUTTON_RST_ENABLE 46

/* typed button|btn rst|reset disable|dis|d */
/* ENUM: CMD0.index 47 button_rst_disable established */
#define L1P5C_CMD0_INDEX_BUTTON_RST_DISABLE 47

/* typed button|btn nmi enable|en|e */
/* ENUM: CMD0.index 48 button_nmi_enable established */
#define L1P5C_CMD0_INDEX_BUTTON_NMI_ENABLE 48

/* typed button|btn nmi disable|dis|d */
/* ENUM: CMD0.index 49 button_nmi_disable established */
#define L1P5C_CMD0_INDEX_BUTTON_NMI_DISABLE 49

/* typed let the carnage begin (hidden) */
/* ENUM: CMD0.index 50 let_the_carnage_begin established */
#define L1P5C_CMD0_INDEX_LET_THE_CARNAGE_BEGIN 50

/* typed slipped my mind (hidden) */
/* ENUM: CMD0.index 51 slipped_my_mind established */
#define L1P5C_CMD0_INDEX_SLIPPED_MY_MIND 51

/* typed super sized please (hidden) */
/* ENUM: CMD0.index 52 super_sized_please established */
#define L1P5C_CMD0_INDEX_SUPER_SIZED_PLEASE 52

/* typed env */
/* ENUM: CMD0.index 53 env established */
#define L1P5C_CMD0_INDEX_ENV 53

/* typed env check|s|summary */
/* ENUM: CMD0.index 54 env_check established */
#define L1P5C_CMD0_INDEX_ENV_CHECK 54

/* typed env on */
/* ENUM: CMD0.index 55 env_on established */
#define L1P5C_CMD0_INDEX_ENV_ON 55

/* typed env off */
/* ENUM: CMD0.index 56 env_off established */
#define L1P5C_CMD0_INDEX_ENV_OFF 56

/* typed env power|pwr on (hidden) */
/* ENUM: CMD0.index 57 env_power_on established */
#define L1P5C_CMD0_INDEX_ENV_POWER_ON 57

/* typed env power|pwr off (hidden) */
/* ENUM: CMD0.index 58 env_power_off established */
#define L1P5C_CMD0_INDEX_ENV_POWER_OFF 58

/* typed env power|pwr monitor (hidden) */
/* ENUM: CMD0.index 59 env_power_monitor established */
#define L1P5C_CMD0_INDEX_ENV_POWER_MONITOR 59

/* typed env temp on */
/* ENUM: CMD0.index 60 env_temp_on established */
#define L1P5C_CMD0_INDEX_ENV_TEMP_ON 60

/* typed env temp off */
/* ENUM: CMD0.index 61 env_temp_off established */
#define L1P5C_CMD0_INDEX_ENV_TEMP_OFF 61

/* typed env altitude high */
/* ENUM: CMD0.index 62 env_altitude_high established */
#define L1P5C_CMD0_INDEX_ENV_ALTITUDE_HIGH 62

/* typed env altitude low */
/* ENUM: CMD0.index 63 env_altitude_low established */
#define L1P5C_CMD0_INDEX_ENV_ALTITUDE_LOW 63

/* typed env reset|rst */
/* ENUM: CMD0.index 64 env_reset established */
#define L1P5C_CMD0_INDEX_ENV_RESET 64

/* typed env test warning */
/* ENUM: CMD0.index 65 env_test_warning established */
#define L1P5C_CMD0_INDEX_ENV_TEST_WARNING 65

/* typed env test fault */
/* ENUM: CMD0.index 66 env_test_fault established */
#define L1P5C_CMD0_INDEX_ENV_TEST_FAULT 66

/* typed env test tmpadv */
/* ENUM: CMD0.index 67 env_test_tmpadv established */
#define L1P5C_CMD0_INDEX_ENV_TEST_TMPADV 67

/* typed env test tmpcrit */
/* ENUM: CMD0.index 68 env_test_tmpcrit established */
#define L1P5C_CMD0_INDEX_ENV_TEST_TMPCRIT 68

/* typed env test <number> <number> <number> <number> <number> (hidden) */
/* ENUM: CMD0.index 69 env_test_number_number_number_number_number established */
#define L1P5C_CMD0_INDEX_ENV_TEST_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER 69

/* typed env test pwrdn (hidden) */
/* ENUM: CMD0.index 70 env_test_pwrdn established */
#define L1P5C_CMD0_INDEX_ENV_TEST_PWRDN 70

/* typed env messages (hidden) */
/* ENUM: CMD0.index 71 env_messages established */
#define L1P5C_CMD0_INDEX_ENV_MESSAGES 71

/* typed env messages faultsonly (hidden) */
/* ENUM: CMD0.index 72 env_messages_faultsonly established */
#define L1P5C_CMD0_INDEX_ENV_MESSAGES_FAULTSONLY 72

/* typed env messages all (hidden) */
/* ENUM: CMD0.index 73 env_messages_all established */
#define L1P5C_CMD0_INDEX_ENV_MESSAGES_ALL 73

/* typed fan */
/* ENUM: CMD0.index 74 fan established */
#define L1P5C_CMD0_INDEX_FAN 74

/* typed fan on|off <number> (hidden) */
/* ENUM: CMD0.index 75 fan_on_number established */
#define L1P5C_CMD0_INDEX_FAN_ON_NUMBER 75

/* typed fan on|off (hidden) */
/* ENUM: CMD0.index 76 fan_on established */
#define L1P5C_CMD0_INDEX_FAN_ON 76

/* typed fan speed (hidden) */
/* ENUM: CMD0.index 77 fan_speed established */
#define L1P5C_CMD0_INDEX_FAN_SPEED 77

/* typed fan speed <number> <number> (hidden) */
/* ENUM: CMD0.index 78 fan_speed_number_number established */
#define L1P5C_CMD0_INDEX_FAN_SPEED_NUMBER_NUMBER 78

/* typed fan speed <number> (hidden) */
/* ENUM: CMD0.index 79 fan_speed_number established */
#define L1P5C_CMD0_INDEX_FAN_SPEED_NUMBER 79

/* typed fan speed low|l|med|m|high|h <number> (hidden) */
/* ENUM: CMD0.index 80 fan_speed_low_number established */
#define L1P5C_CMD0_INDEX_FAN_SPEED_LOW_NUMBER 80

/* typed fan speed low|l|med|m|high|h (hidden) */
/* ENUM: CMD0.index 81 fan_speed_low established */
#define L1P5C_CMD0_INDEX_FAN_SPEED_LOW 81

/* typed fan auto on|off (hidden) */
/* ENUM: CMD0.index 82 fan_auto_on established */
#define L1P5C_CMD0_INDEX_FAN_AUTO_ON 82

/* typed help|hlp */
/* ENUM: CMD0.index 83 help established */
#define L1P5C_CMD0_INDEX_HELP 83

/* typed help|hlp <word> */
/* ENUM: CMD0.index 84 help_word established */
#define L1P5C_CMD0_INDEX_HELP_WORD 84

/* typed help list hidden (hidden) */
/* ENUM: CMD0.index 85 help_list_hidden established */
#define L1P5C_CMD0_INDEX_HELP_LIST_HIDDEN 85

/* typed help <word> hidden (hidden) */
/* ENUM: CMD0.index 86 help_word_hidden established */
#define L1P5C_CMD0_INDEX_HELP_WORD_HIDDEN 86

/* typed history|hist */
/* ENUM: CMD0.index 87 history established */
#define L1P5C_CMD0_INDEX_HISTORY 87

/* typed l1dbg */
/* ENUM: CMD0.index 88 l1dbg established */
#define L1P5C_CMD0_INDEX_L1DBG 88

/* typed l1dbg syscom <number> */
/* ENUM: CMD0.index 89 l1dbg_sys_link_number established */
#define L1P5C_CMD0_INDEX_L1DBG_SYS_LINK_NUMBER 89

/* typed l1dbg syscom <number> <number> */
/* ENUM: CMD0.index 90 l1dbg_sys_link_number_number established */
#define L1P5C_CMD0_INDEX_L1DBG_SYS_LINK_NUMBER_NUMBER 90

/* typed l1dbg irtr <number> */
/* ENUM: CMD0.index 91 l1dbg_irtr_number established */
#define L1P5C_CMD0_INDEX_L1DBG_IRTR_NUMBER 91

/* typed l1dbg env <number> */
/* ENUM: CMD0.index 92 l1dbg_env_number established */
#define L1P5C_CMD0_INDEX_L1DBG_ENV_NUMBER 92

/* typed l1dbg env off */
/* ENUM: CMD0.index 93 l1dbg_env_off established */
#define L1P5C_CMD0_INDEX_L1DBG_ENV_OFF 93

/* typed l1dbg port on|off */
/* ENUM: CMD0.index 94 l1dbg_port_on established */
#define L1P5C_CMD0_INDEX_L1DBG_PORT_ON 94

/* typed l1dbg i2c on|off */
/* ENUM: CMD0.index 95 l1dbg_i2c_on established */
#define L1P5C_CMD0_INDEX_L1DBG_I2C_ON 95

/* typed l1dbg margin|mgn on|off */
/* ENUM: CMD0.index 96 l1dbg_margin_on established */
#define L1P5C_CMD0_INDEX_L1DBG_MARGIN_ON 96

/* typed l1dbg console on|off */
/* ENUM: CMD0.index 97 l1dbg_console_on established */
#define L1P5C_CMD0_INDEX_L1DBG_CONSOLE_ON 97

/* typed l1dbg mempanic on|off */
/* ENUM: CMD0.index 98 l1dbg_mempanic_on established */
#define L1P5C_CMD0_INDEX_L1DBG_MEMPANIC_ON 98

/* typed l1dbg pppdump on|off */
/* ENUM: CMD0.index 99 l1dbg_pppdump_on established */
#define L1P5C_CMD0_INDEX_L1DBG_PPPDUMP_ON 99

/* typed l1dbg pppdelay <number> */
/* ENUM: CMD0.index 100 l1dbg_pppdelay_number established */
#define L1P5C_CMD0_INDEX_L1DBG_PPPDELAY_NUMBER 100

/* typed l1dbg promreq|req on|off */
/* ENUM: CMD0.index 101 l1dbg_promreq_on established */
#define L1P5C_CMD0_INDEX_L1DBG_PROMREQ_ON 101

/* typed l1dbg qsusp <number> <number> */
/* ENUM: CMD0.index 102 l1dbg_qsusp_number_number established */
#define L1P5C_CMD0_INDEX_L1DBG_QSUSP_NUMBER_NUMBER 102

/* typed l1dbg printf on|off */
/* ENUM: CMD0.index 103 l1dbg_printf_on established */
#define L1P5C_CMD0_INDEX_L1DBG_PRINTF_ON 103

/* typed l1dbg flashignore <number> <number> */
/* ENUM: CMD0.index 104 l1dbg_flashignore_number_number established */
#define L1P5C_CMD0_INDEX_L1DBG_FLASHIGNORE_NUMBER_NUMBER 104

/* typed l1dbg misc <number> (hidden) */
/* ENUM: CMD0.index 105 l1dbg_misc_number established */
#define L1P5C_CMD0_INDEX_L1DBG_MISC_NUMBER 105

/* typed l1dbg gal <number> (hidden) */
/* ENUM: CMD0.index 106 l1dbg_gal_number established */
#define L1P5C_CMD0_INDEX_L1DBG_GAL_NUMBER 106

/* typed link */
/* ENUM: CMD0.index 107 link established */
#define L1P5C_CMD0_INDEX_LINK 107

/* typed link verbose|v */
/* ENUM: CMD0.index 108 link_verbose established */
#define L1P5C_CMD0_INDEX_LINK_VERBOSE 108

/* typed link check */
/* ENUM: CMD0.index 109 link_check established */
#define L1P5C_CMD0_INDEX_LINK_CHECK 109

/* typed link check v */
/* ENUM: CMD0.index 110 link_check_v established */
#define L1P5C_CMD0_INDEX_LINK_CHECK_V 110

/* typed link stable */
/* ENUM: CMD0.index 111 link_stable established */
#define L1P5C_CMD0_INDEX_LINK_STABLE 111

/* typed log */
/* ENUM: CMD0.index 112 log established */
#define L1P5C_CMD0_INDEX_LOG 112

/* typed log clear|reset */
/* ENUM: CMD0.index 113 log_clear established */
#define L1P5C_CMD0_INDEX_LOG_CLEAR 113

/* typed log insert <command> */
/* ENUM: CMD0.index 114 log_insert_command established */
#define L1P5C_CMD0_INDEX_LOG_INSERT_COMMAND 114

/* typed ioport|ioprt */
/* ENUM: CMD0.index 115 ioport established */
#define L1P5C_CMD0_INDEX_IOPORT 115

/* typed ioport|ioprt 400|600 */
/* ENUM: CMD0.index 116 ioport_400 established */
#define L1P5C_CMD0_INDEX_IOPORT_400 116

/* typed ioport|ioprt a 400|600 */
/* ENUM: CMD0.index 117 ioport_a_400 established */
#define L1P5C_CMD0_INDEX_IOPORT_A_400 117

/* typed ioport|ioprt b 400|600 */
/* ENUM: CMD0.index 118 ioport_b_400 established */
#define L1P5C_CMD0_INDEX_IOPORT_B_400 118

/* typed ioport|ioprt force 400|600 (hidden) */
/* ENUM: CMD0.index 119 ioport_force_400 established */
#define L1P5C_CMD0_INDEX_IOPORT_FORCE_400 119

/* typed ioport|ioprt force alt1 (hidden) */
/* ENUM: CMD0.index 120 ioport_force_alt1 established */
#define L1P5C_CMD0_INDEX_IOPORT_FORCE_ALT1 120

/* typed ioport|ioprt force alt2 (hidden) */
/* ENUM: CMD0.index 121 ioport_force_alt2 established */
#define L1P5C_CMD0_INDEX_IOPORT_FORCE_ALT2 121

/* typed ioport|ioprt clksrc a */
/* ENUM: CMD0.index 122 ioport_clksrc_a established */
#define L1P5C_CMD0_INDEX_IOPORT_CLKSRC_A 122

/* typed ioport|ioprt clksrc b */
/* ENUM: CMD0.index 123 ioport_clksrc_b established */
#define L1P5C_CMD0_INDEX_IOPORT_CLKSRC_B 123

/* typed istat memory|m */
/* ENUM: CMD0.index 124 istat_memory established */
#define L1P5C_CMD0_INDEX_ISTAT_MEMORY 124

/* typed istat queues|q */
/* ENUM: CMD0.index 125 istat_queues established */
#define L1P5C_CMD0_INDEX_ISTAT_QUEUES 125

/* typed istat queues|q reset|r */
/* ENUM: CMD0.index 126 istat_queues_reset established */
#define L1P5C_CMD0_INDEX_ISTAT_QUEUES_RESET 126

/* typed istat tasks|t */
/* ENUM: CMD0.index 127 istat_tasks established */
#define L1P5C_CMD0_INDEX_ISTAT_TASKS 127

/* typed istat pmalloc|p */
/* ENUM: CMD0.index 128 istat_pmalloc established */
#define L1P5C_CMD0_INDEX_ISTAT_PMALLOC 128

/* typed istat dump|d <word> */
/* ENUM: CMD0.index 129 istat_dump_word established */
#define L1P5C_CMD0_INDEX_ISTAT_DUMP_WORD 129

/* typed b2b (hidden) */
/* ENUM: CMD0.index 130 b2b established */
#define L1P5C_CMD0_INDEX_B2B 130

/* typed l1 */
/* ENUM: CMD0.index 131 l1 established */
#define L1P5C_CMD0_INDEX_L1 131

/* typed leds */
/* ENUM: CMD0.index 132 leds established */
#define L1P5C_CMD0_INDEX_LEDS 132

/* typed margin|mgn */
/* ENUM: CMD0.index 133 margin established */
#define L1P5C_CMD0_INDEX_MARGIN 133

/* typed margin|mgn default|d|low|l|norm|n|high|h */
/* ENUM: CMD0.index 134 margin_default established */
#define L1P5C_CMD0_INDEX_MARGIN_DEFAULT 134

/* typed margin|mgn <number> default|d|low|l|norm|n|high|h */
/* ENUM: CMD0.index 135 margin_number_default established */
#define L1P5C_CMD0_INDEX_MARGIN_NUMBER_DEFAULT 135

/* typed margin|mgn <number> <number> */
/* ENUM: CMD0.index 136 margin_number_number established */
#define L1P5C_CMD0_INDEX_MARGIN_NUMBER_NUMBER 136

/* typed margin|mgn dimm v|voltage <word> */
/* ENUM: CMD0.index 137 margin_dimm_v_word established */
#define L1P5C_CMD0_INDEX_MARGIN_DIMM_V_WORD 137

/* typed network */
/* ENUM: CMD0.index 138 network established */
#define L1P5C_CMD0_INDEX_NETWORK 138

/* typed network usb|422|serial */
/* ENUM: CMD0.index 139 network_usb established */
#define L1P5C_CMD0_INDEX_NETWORK_USB 139

/* typed network autodetect|auto on|off */
/* ENUM: CMD0.index 140 network_autodetect_on established */
#define L1P5C_CMD0_INDEX_NETWORK_AUTODETECT_ON 140

/* typed pimm */
/* ENUM: CMD0.index 141 pimm established */
#define L1P5C_CMD0_INDEX_PIMM 141

/* typed pimm auto (hidden) */
/* ENUM: CMD0.index 142 pimm_auto established */
#define L1P5C_CMD0_INDEX_PIMM_AUTO 142

/* typed pimm clksrc ext|external (hidden) */
/* ENUM: CMD0.index 143 pimm_clksrc_ext established */
#define L1P5C_CMD0_INDEX_PIMM_CLKSRC_EXT 143

/* typed pimm clksrc int|internal (hidden) */
/* ENUM: CMD0.index 144 pimm_clksrc_int established */
#define L1P5C_CMD0_INDEX_PIMM_CLKSRC_INT 144

/* typed pimm clkmode async (hidden) */
/* ENUM: CMD0.index 145 pimm_clkmode_async established */
#define L1P5C_CMD0_INDEX_PIMM_CLKMODE_ASYNC 145

/* typed pimm clkmode sync (hidden) */
/* ENUM: CMD0.index 146 pimm_clkmode_sync established */
#define L1P5C_CMD0_INDEX_PIMM_CLKMODE_SYNC 146

/* typed port|prt */
/* ENUM: CMD0.index 147 port established */
#define L1P5C_CMD0_INDEX_PORT 147

/* typed port|prt v */
/* ENUM: CMD0.index 148 port_v established */
#define L1P5C_CMD0_INDEX_PORT_V 148

/* typed power|pwr */
/* ENUM: CMD0.index 149 power established */
#define L1P5C_CMD0_INDEX_POWER 149

/* typed power|pwr check|s|summary */
/* ENUM: CMD0.index 150 power_check established */
#define L1P5C_CMD0_INDEX_POWER_CHECK 150

/* typed power|pwr vrm */
/* ENUM: CMD0.index 151 power_vrm established */
#define L1P5C_CMD0_INDEX_POWER_VRM 151

/* typed power|pwr up|u */
/* ENUM: CMD0.index 152 power_up established */
#define L1P5C_CMD0_INDEX_POWER_UP 152

/* typed power|pwr up|u hold */
/* ENUM: CMD0.index 153 power_up_hold established */
#define L1P5C_CMD0_INDEX_POWER_UP_HOLD 153

/* typed power|pwr down|d */
/* ENUM: CMD0.index 154 power_down established */
#define L1P5C_CMD0_INDEX_POWER_DOWN 154

/* typed reset|rst */
/* ENUM: CMD0.index 155 reset established */
#define L1P5C_CMD0_INDEX_RESET 155

/* typed reset|rst hold */
/* ENUM: CMD0.index 156 reset_hold established */
#define L1P5C_CMD0_INDEX_RESET_HOLD 156

/* typed nmi */
/* ENUM: CMD0.index 157 nmi established */
#define L1P5C_CMD0_INDEX_NMI 157

/* typed softreset|softrst */
/* ENUM: CMD0.index 158 softreset established */
#define L1P5C_CMD0_INDEX_SOFTRESET 158

/* typed softreset|softrst <number> */
/* ENUM: CMD0.index 159 softreset_number established */
#define L1P5C_CMD0_INDEX_SOFTRESET_NUMBER 159

/* typed select|sel */
/* ENUM: CMD0.index 160 select established */
#define L1P5C_CMD0_INDEX_SELECT 160

/* typed select|sel <number> <number> <number> <number> (hidden) */
/* ENUM: CMD0.index 161 select_number_number_number_number established */
#define L1P5C_CMD0_INDEX_SELECT_NUMBER_NUMBER_NUMBER_NUMBER 161

/* typed select|sel subchannel|sub|s console|con */
/* ENUM: CMD0.index 162 select_subchannel_console established */
#define L1P5C_CMD0_INDEX_SELECT_SUBCHANNEL_CONSOLE 162

/* typed select|sel subchannel|sub|s a|b|c|d */
/* ENUM: CMD0.index 163 select_subchannel_a established */
#define L1P5C_CMD0_INDEX_SELECT_SUBCHANNEL_A 163

/* typed select|sel subchannel|sub|s <number> */
/* ENUM: CMD0.index 164 select_subchannel_number established */
#define L1P5C_CMD0_INDEX_SELECT_SUBCHANNEL_NUMBER 164

/* typed select|sel subchannel|sub|s <word> */
/* ENUM: CMD0.index 165 select_subchannel_word established */
#define L1P5C_CMD0_INDEX_SELECT_SUBCHANNEL_WORD 165

/* typed select|sel enable */
/* ENUM: CMD0.index 166 select_enable established */
#define L1P5C_CMD0_INDEX_SELECT_ENABLE 166

/* typed select|sel disable */
/* ENUM: CMD0.index 167 select_disable established */
#define L1P5C_CMD0_INDEX_SELECT_DISABLE 167

/* typed select|sel filter on */
/* ENUM: CMD0.index 168 select_filter_on established */
#define L1P5C_CMD0_INDEX_SELECT_FILTER_ON 168

/* typed select|sel filter off */
/* ENUM: CMD0.index 169 select_filter_off established */
#define L1P5C_CMD0_INDEX_SELECT_FILTER_OFF 169

/* typed serial */
/* ENUM: CMD0.index 170 serial established */
#define L1P5C_CMD0_INDEX_SERIAL 170

/* typed serial verify */
/* ENUM: CMD0.index 171 serial_verify established */
#define L1P5C_CMD0_INDEX_SERIAL_VERIFY 171

/* typed serial all */
/* ENUM: CMD0.index 172 serial_all established */
#define L1P5C_CMD0_INDEX_SERIAL_ALL 172

/* typed serial all v|verbose */
/* ENUM: CMD0.index 173 serial_all_v established */
#define L1P5C_CMD0_INDEX_SERIAL_ALL_V 173

/* typed serial b|board */
/* ENUM: CMD0.index 174 serial_b established */
#define L1P5C_CMD0_INDEX_SERIAL_B 174

/* typed serial b|board v|verbose */
/* ENUM: CMD0.index 175 serial_b_v established */
#define L1P5C_CMD0_INDEX_SERIAL_B_V 175

/* typed serial cpu */
/* ENUM: CMD0.index 176 serial_cpu established */
#define L1P5C_CMD0_INDEX_SERIAL_CPU 176

/* typed serial cpu v|verbose */
/* ENUM: CMD0.index 177 serial_cpu_v established */
#define L1P5C_CMD0_INDEX_SERIAL_CPU_V 177

/* typed serial dimm */
/* ENUM: CMD0.index 178 serial_dimm established */
#define L1P5C_CMD0_INDEX_SERIAL_DIMM 178

/* typed serial dimm v|verbose */
/* ENUM: CMD0.index 179 serial_dimm_v established */
#define L1P5C_CMD0_INDEX_SERIAL_DIMM_V 179

/* typed serial dimm cfg|config */
/* ENUM: CMD0.index 180 serial_dimm_cfg established */
#define L1P5C_CMD0_INDEX_SERIAL_DIMM_CFG 180

/* typed serial dimm status (hidden) */
/* ENUM: CMD0.index 181 serial_dimm_status established */
#define L1P5C_CMD0_INDEX_SERIAL_DIMM_STATUS 181

/* typed serial clear */
/* ENUM: CMD0.index 182 serial_clear established */
#define L1P5C_CMD0_INDEX_SERIAL_CLEAR 182

/* typed serial set <word> (hidden) */
/* ENUM: CMD0.index 183 serial_set_word established */
#define L1P5C_CMD0_INDEX_SERIAL_SET_WORD 183

/* typed serial <word> <word> <word> <word> */
/* ENUM: CMD0.index 184 serial_word_word_word_word established */
#define L1P5C_CMD0_INDEX_SERIAL_WORD_WORD_WORD_WORD 184

/* typed serial security off <word> <word> <word> <word> (hidden) */
/* ENUM: CMD0.index 185 serial_security_off_word_word_word_word established */
#define L1P5C_CMD0_INDEX_SERIAL_SECURITY_OFF_WORD_WORD_WORD_WORD 185

/* typed serial security on */
/* ENUM: CMD0.index 186 serial_security_on established */
#define L1P5C_CMD0_INDEX_SERIAL_SECURITY_ON 186

/* typed serial brick disable (hidden) */
/* ENUM: CMD0.index 187 serial_brick_disable established */
#define L1P5C_CMD0_INDEX_SERIAL_BRICK_DISABLE 187

/* typed serial brick disable confirm (hidden) */
/* ENUM: CMD0.index 188 serial_brick_disable_confirm established */
#define L1P5C_CMD0_INDEX_SERIAL_BRICK_DISABLE_CONFIRM 188

/* typed sysstate */
/* ENUM: CMD0.index 189 sysstate established */
#define L1P5C_CMD0_INDEX_SYSSTATE 189

/* typed eeprom */
/* ENUM: CMD0.index 190 eeprom established */
#define L1P5C_CMD0_INDEX_EEPROM 190

/* typed eeprom <number> <number> <number> */
/* ENUM: CMD0.index 191 eeprom_number_number_number established */
#define L1P5C_CMD0_INDEX_EEPROM_NUMBER_NUMBER_NUMBER 191

/* typed eeprom fill <number> <number> <number> <number> (hidden) */
/* ENUM: CMD0.index 192 eeprom_fill_number_number_number_number established */
#define L1P5C_CMD0_INDEX_EEPROM_FILL_NUMBER_NUMBER_NUMBER_NUMBER 192

/* typed uart */
/* ENUM: CMD0.index 193 uart established */
#define L1P5C_CMD0_INDEX_UART 193

/* typed uart syscom|junkbus|jb|bedrock reset|r */
/* ENUM: CMD0.index 194 uart_sys_link_reset established */
#define L1P5C_CMD0_INDEX_UART_SYS_LINK_RESET 194

/* typed usb */
/* ENUM: CMD0.index 195 usb established */
#define L1P5C_CMD0_INDEX_USB 195

/* typed router|rtr */
/* ENUM: CMD0.index 196 router established */
#define L1P5C_CMD0_INDEX_ROUTER 196

/* typed router|rtr meta */
/* ENUM: CMD0.index 197 router_meta established */
#define L1P5C_CMD0_INDEX_ROUTER_META 197

/* typed router|rtr meta1|meta2|meta3 */
/* ENUM: CMD0.index 198 router_meta1 established */
#define L1P5C_CMD0_INDEX_ROUTER_META1 198

/* typed router|rtr repeater|rep */
/* ENUM: CMD0.index 199 router_repeater established */
#define L1P5C_CMD0_INDEX_ROUTER_REPEATER 199

/* typed router|rtr ordinary|ord */
/* ENUM: CMD0.index 200 router_ordinary established */
#define L1P5C_CMD0_INDEX_ROUTER_ORDINARY 200

/* typed router|rtr 6 port */
/* ENUM: CMD0.index 201 router_6_port established */
#define L1P5C_CMD0_INDEX_ROUTER_6_PORT 201

/* typed router|rtr 6 port confirm (hidden) */
/* ENUM: CMD0.index 202 router_6_port_confirm established */
#define L1P5C_CMD0_INDEX_ROUTER_6_PORT_CONFIRM 202

/* typed router|rtr 8 port */
/* ENUM: CMD0.index 203 router_8_port established */
#define L1P5C_CMD0_INDEX_ROUTER_8_PORT 203

/* typed router|rtr 8 port <word> <word> <word> <word> */
/* ENUM: CMD0.index 204 router_8_port_word_word_word_word established */
#define L1P5C_CMD0_INDEX_ROUTER_8_PORT_WORD_WORD_WORD_WORD 204

/* typed router|rtr service <word> <word> <word> <word> */
/* ENUM: CMD0.index 205 router_service_word_word_word_word established */
#define L1P5C_CMD0_INDEX_ROUTER_SERVICE_WORD_WORD_WORD_WORD 205

/* typed router|rtr service off */
/* ENUM: CMD0.index 206 router_service_off established */
#define L1P5C_CMD0_INDEX_ROUTER_SERVICE_OFF 206

/* typed router|rtr spare */
/* ENUM: CMD0.index 207 router_spare established */
#define L1P5C_CMD0_INDEX_ROUTER_SPARE 207

/* typed service <word> <word> <word> <word> */
/* ENUM: CMD0.index 208 service_word_word_word_word established */
#define L1P5C_CMD0_INDEX_SERVICE_WORD_WORD_WORD_WORD 208

/* typed service off */
/* ENUM: CMD0.index 209 service_off established */
#define L1P5C_CMD0_INDEX_SERVICE_OFF 209

/* typed date tz */
/* ENUM: CMD0.index 210 date_tz established */
#define L1P5C_CMD0_INDEX_DATE_TZ 210

/* typed date tz <word> */
/* ENUM: CMD0.index 211 date_tz_word established */
#define L1P5C_CMD0_INDEX_DATE_TZ_WORD 211

/* typed date */
/* ENUM: CMD0.index 212 date established */
#define L1P5C_CMD0_INDEX_DATE 212

/* typed date raw */
/* ENUM: CMD0.index 213 date_raw established */
#define L1P5C_CMD0_INDEX_DATE_RAW 213

/* typed date <word> */
/* ENUM: CMD0.index 214 date_word established */
#define L1P5C_CMD0_INDEX_DATE_WORD 214

/* typed date <word> <word> */
/* ENUM: CMD0.index 215 date_word_word established */
#define L1P5C_CMD0_INDEX_DATE_WORD_WORD 215

/* typed nvram reset */
/* ENUM: CMD0.index 216 nvram_reset established */
#define L1P5C_CMD0_INDEX_NVRAM_RESET 216

/* typed nvram read <number> <number> (hidden) */
/* ENUM: CMD0.index 217 nvram_read_number_number established */
#define L1P5C_CMD0_INDEX_NVRAM_READ_NUMBER_NUMBER 217

/* typed nvram write <number> <number> (hidden) */
/* ENUM: CMD0.index 218 nvram_write_number_number established */
#define L1P5C_CMD0_INDEX_NVRAM_WRITE_NUMBER_NUMBER 218

/* typed nvram scratch (hidden) */
/* ENUM: CMD0.index 219 nvram_scratch established */
#define L1P5C_CMD0_INDEX_NVRAM_SCRATCH 219

/* typed nvram scratch <number> (hidden) */
/* ENUM: CMD0.index 220 nvram_scratch_number established */
#define L1P5C_CMD0_INDEX_NVRAM_SCRATCH_NUMBER 220

/* typed l2 <command> (hidden) */
/* ENUM: CMD0.index 221 l2_command established */
#define L1P5C_CMD0_INDEX_L2_COMMAND 221

/* typed security verify 3400 */
/* ENUM: CMD0.index 222 security_verify_3400 established */
#define L1P5C_CMD0_INDEX_SECURITY_VERIFY_3400 222

/* typed security verify 3800 */
/* ENUM: CMD0.index 223 security_verify_3800 established */
#define L1P5C_CMD0_INDEX_SECURITY_VERIFY_3800 223

/* typed sleep <number> <number> <command> (hidden) */
/* ENUM: CMD0.index 224 sleep_number_number_command established */
#define L1P5C_CMD0_INDEX_SLEEP_NUMBER_NUMBER_COMMAND 224

/* typed mtb (hidden) */
/* ENUM: CMD0.index 225 mtb established */
#define L1P5C_CMD0_INDEX_MTB 225

/* typed flash default reset */
/* ENUM: CMD0.index 226 flash_default_reset established */
#define L1P5C_CMD0_INDEX_FLASH_DEFAULT_RESET 226

/* typed flash default a */
/* ENUM: CMD0.index 227 flash_default_a established */
#define L1P5C_CMD0_INDEX_FLASH_DEFAULT_A 227

/* typed flash default b */
/* ENUM: CMD0.index 228 flash_default_b established */
#define L1P5C_CMD0_INDEX_FLASH_DEFAULT_B 228

/* typed flash default current */
/* ENUM: CMD0.index 229 flash_default_current established */
#define L1P5C_CMD0_INDEX_FLASH_DEFAULT_CURRENT 229

/* typed flash default old */
/* ENUM: CMD0.index 230 flash_default_old established */
#define L1P5C_CMD0_INDEX_FLASH_DEFAULT_OLD 230

/* typed flash default new */
/* ENUM: CMD0.index 231 flash_default_new established */
#define L1P5C_CMD0_INDEX_FLASH_DEFAULT_NEW 231

/* typed flash status */
/* ENUM: CMD0.index 232 flash_status established */
#define L1P5C_CMD0_INDEX_FLASH_STATUS 232

/* typed flash status v|verbose */
/* ENUM: CMD0.index 233 flash_status_v established */
#define L1P5C_CMD0_INDEX_FLASH_STATUS_V 233

/* typed reboot_l1 */
/* ENUM: CMD0.index 234 reboot_l1 established */
#define L1P5C_CMD0_INDEX_REBOOT_L1 234

/* typed reboot_l1 a */
/* ENUM: CMD0.index 235 reboot_l1_a established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_A 235

/* typed reboot_l1 b */
/* ENUM: CMD0.index 236 reboot_l1_b established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_B 236

/* typed reboot_l1 current */
/* ENUM: CMD0.index 237 reboot_l1_current established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_CURRENT 237

/* typed reboot_l1 old */
/* ENUM: CMD0.index 238 reboot_l1_old established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_OLD 238

/* typed reboot_l1 new */
/* ENUM: CMD0.index 239 reboot_l1_new established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_NEW 239

/* typed reboot_l1 other */
/* ENUM: CMD0.index 240 reboot_l1_other established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_OTHER 240

/* typed reboot_l1 addr <number> */
/* ENUM: CMD0.index 241 reboot_l1_addr_number established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_ADDR_NUMBER 241

/* typed reboot_l1 raw <number> */
/* ENUM: CMD0.index 242 reboot_l1_raw_number established */
#define L1P5C_CMD0_INDEX_REBOOT_L1_RAW_NUMBER 242

/* typed version|ver */
/* ENUM: CMD0.index 243 version established */
#define L1P5C_CMD0_INDEX_VERSION 243

/* typed pbay */
/* ENUM: CMD0.index 244 pbay established */
#define L1P5C_CMD0_INDEX_PBAY 244

/* typed pbay version|ver */
/* ENUM: CMD0.index 245 pbay_version established */
#define L1P5C_CMD0_INDEX_PBAY_VERSION 245

/* typed pbay dps */
/* ENUM: CMD0.index 246 pbay_dps established */
#define L1P5C_CMD0_INDEX_PBAY_DPS 246

/* typed pbay dps <word> */
/* ENUM: CMD0.index 247 pbay_dps_word established */
#define L1P5C_CMD0_INDEX_PBAY_DPS_WORD 247

/* typed pbay dcport|dcprt */
/* ENUM: CMD0.index 248 pbay_dcport established */
#define L1P5C_CMD0_INDEX_PBAY_DCPORT 248

/* typed pbay dcport|dcprt <word> */
/* ENUM: CMD0.index 249 pbay_dcport_word established */
#define L1P5C_CMD0_INDEX_PBAY_DCPORT_WORD 249

/* typed pbay fru|f */
/* ENUM: CMD0.index 250 pbay_fru established */
#define L1P5C_CMD0_INDEX_PBAY_FRU 250

/* typed pbay fru|f <word> */
/* ENUM: CMD0.index 251 pbay_fru_word established */
#define L1P5C_CMD0_INDEX_PBAY_FRU_WORD 251

/* typed pbay reset|rst */
/* ENUM: CMD0.index 252 pbay_reset established */
#define L1P5C_CMD0_INDEX_PBAY_RESET 252

/* typed pbay env */
/* ENUM: CMD0.index 253 pbay_env established */
#define L1P5C_CMD0_INDEX_PBAY_ENV 253

/* typed pbay env on */
/* ENUM: CMD0.index 254 pbay_env_on established */
#define L1P5C_CMD0_INDEX_PBAY_ENV_ON 254

/* typed pbay env off */
/* ENUM: CMD0.index 255 pbay_env_off established */
#define L1P5C_CMD0_INDEX_PBAY_ENV_OFF 255

/* typed pbay init */
/* ENUM: CMD0.index 256 pbay_init established */
#define L1P5C_CMD0_INDEX_PBAY_INIT 256

/* typed pbay serial */
/* ENUM: CMD0.index 257 pbay_serial established */
#define L1P5C_CMD0_INDEX_PBAY_SERIAL 257

/* typed test|tst i2c */
/* ENUM: CMD0.index 258 test_i2c established */
#define L1P5C_CMD0_INDEX_TEST_I2C 258

/* typed test|tst i2c <number> */
/* ENUM: CMD0.index 259 test_i2c_number established */
#define L1P5C_CMD0_INDEX_TEST_I2C_NUMBER 259

/* typed test|tst ioexp set <number> <number> */
/* ENUM: CMD0.index 260 test_ioexp_set_number_number established */
#define L1P5C_CMD0_INDEX_TEST_IOEXP_SET_NUMBER_NUMBER 260

/* typed test|tst ioexp get <number> */
/* ENUM: CMD0.index 261 test_ioexp_get_number established */
#define L1P5C_CMD0_INDEX_TEST_IOEXP_GET_NUMBER 261

/* typed test|tst ioexp get all */
/* ENUM: CMD0.index 262 test_ioexp_get_all established */
#define L1P5C_CMD0_INDEX_TEST_IOEXP_GET_ALL 262

/* typed test|tst intr */
/* ENUM: CMD0.index 263 test_intr established */
#define L1P5C_CMD0_INDEX_TEST_INTR 263

/* typed test|tst display */
/* ENUM: CMD0.index 264 test_display established */
#define L1P5C_CMD0_INDEX_TEST_DISPLAY 264

/* typed test|tst exception|exc mem */
/* ENUM: CMD0.index 265 test_exception_mem established */
#define L1P5C_CMD0_INDEX_TEST_EXCEPTION_MEM 265

/* typed test|tst exception|exc stack */
/* ENUM: CMD0.index 266 test_exception_stack established */
#define L1P5C_CMD0_INDEX_TEST_EXCEPTION_STACK 266

/* typed test|tst exception|exc wdog */
/* ENUM: CMD0.index 267 test_exception_wdog established */
#define L1P5C_CMD0_INDEX_TEST_EXCEPTION_WDOG 267

/* typed test|tst exception|exc fatal */
/* ENUM: CMD0.index 268 test_exception_fatal established */
#define L1P5C_CMD0_INDEX_TEST_EXCEPTION_FATAL 268

/* typed scan reset */
/* ENUM: CMD0.index 269 scan_reset established */
#define L1P5C_CMD0_INDEX_SCAN_RESET 269

/* typed scan reset hard */
/* ENUM: CMD0.index 270 scan_reset_hard established */
#define L1P5C_CMD0_INDEX_SCAN_RESET_HARD 270

/* typed scan reset soft */
/* ENUM: CMD0.index 271 scan_reset_soft established */
#define L1P5C_CMD0_INDEX_SCAN_RESET_SOFT 271

/* typed scan reset both */
/* ENUM: CMD0.index 272 scan_reset_both established */
#define L1P5C_CMD0_INDEX_SCAN_RESET_BOTH 272

/* typed scan sel <number> <number> */
/* ENUM: CMD0.index 273 scan_sel_number_number established */
#define L1P5C_CMD0_INDEX_SCAN_SEL_NUMBER_NUMBER 273

/* typed scan sel <number> <number> <number> */
/* ENUM: CMD0.index 274 scan_sel_number_number_number established */
#define L1P5C_CMD0_INDEX_SCAN_SEL_NUMBER_NUMBER_NUMBER 274

/* typed scan sel <number> <number> <number> <number> */
/* ENUM: CMD0.index 275 scan_sel_number_number_number_number established */
#define L1P5C_CMD0_INDEX_SCAN_SEL_NUMBER_NUMBER_NUMBER_NUMBER 275

/* typed scan ids <number> */
/* ENUM: CMD0.index 276 scan_ids_number established */
#define L1P5C_CMD0_INDEX_SCAN_IDS_NUMBER 276

/* typed scan set trst 0 */
/* ENUM: CMD0.index 277 scan_set_trst_0 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TRST_0 277

/* typed scan set trst 1 */
/* ENUM: CMD0.index 278 scan_set_trst_1 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TRST_1 278

/* typed scan set tck 0 */
/* ENUM: CMD0.index 279 scan_set_tck_0 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TCK_0 279

/* typed scan set tck 1 */
/* ENUM: CMD0.index 280 scan_set_tck_1 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TCK_1 280

/* typed scan set tms 0 */
/* ENUM: CMD0.index 281 scan_set_tms_0 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TMS_0 281

/* typed scan set tms 1 */
/* ENUM: CMD0.index 282 scan_set_tms_1 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TMS_1 282

/* typed scan set tdi 0 */
/* ENUM: CMD0.index 283 scan_set_tdi_0 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TDI_0 283

/* typed scan set tdi 1 */
/* ENUM: CMD0.index 284 scan_set_tdi_1 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_TDI_1 284

/* typed scan set psi 0 */
/* ENUM: CMD0.index 285 scan_set_psi_0 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_PSI_0 285

/* typed scan set psi 1 */
/* ENUM: CMD0.index 286 scan_set_psi_1 established */
#define L1P5C_CMD0_INDEX_SCAN_SET_PSI_1 286

/* typed scan get */
/* ENUM: CMD0.index 287 scan_get established */
#define L1P5C_CMD0_INDEX_SCAN_GET 287

/* typed scan debug <number> */
/* ENUM: CMD0.index 288 scan_debug_number established */
#define L1P5C_CMD0_INDEX_SCAN_DEBUG_NUMBER 288

/* typed scan debug */
/* ENUM: CMD0.index 289 scan_debug established */
#define L1P5C_CMD0_INDEX_SCAN_DEBUG 289

/* typed scan cfg */
/* ENUM: CMD0.index 290 scan_cfg established */
#define L1P5C_CMD0_INDEX_SCAN_CFG 290

/* typed scan imp set dllbyp <number> */
/* ENUM: CMD0.index 291 scan_imp_set_dllbyp_number established */
#define L1P5C_CMD0_INDEX_SCAN_IMP_SET_DLLBYP_NUMBER 291

/* typed scan imp set <number> */
/* ENUM: CMD0.index 292 scan_imp_set_number established */
#define L1P5C_CMD0_INDEX_SCAN_IMP_SET_NUMBER 292

/* typed scan imp unlock */
/* ENUM: CMD0.index 293 scan_imp_unlock established */
#define L1P5C_CMD0_INDEX_SCAN_IMP_UNLOCK 293

/* typed scan test <number> */
/* ENUM: CMD0.index 294 scan_test_number established */
#define L1P5C_CMD0_INDEX_SCAN_TEST_NUMBER 294

/* typed fru|pci|node */
/* ENUM: CMD0.index 295 fru established */
#define L1P5C_CMD0_INDEX_FRU 295

/* typed fru|pci|node <command> */
/* ENUM: CMD0.index 296 fru_command established */
#define L1P5C_CMD0_INDEX_FRU_COMMAND 296

/* FIELD: CMD1.index 7:0 established */
#define L1P5C_CMD1_INDEX_SHIFT 0
#define L1P5C_CMD1_INDEX_MASK 0xffu

/* typed check power|pwr on|up|u|off|down|d */
/* ENUM: CMD1.index 0 check_power_on established */
#define L1P5C_CMD1_INDEX_CHECK_POWER_ON 0

/* typed check margin|mgn high|h|normal|norm|n|low|l|other|o */
/* ENUM: CMD1.index 1 check_margin_high established */
#define L1P5C_CMD1_INDEX_CHECK_MARGIN_HIGH 1

/* typed check scramble|scram enabled|e|disabled|d */
/* ENUM: CMD1.index 2 check_scramble_enabled established */
#define L1P5C_CMD1_INDEX_CHECK_SCRAMBLE_ENABLED 2

/* typed check links */
/* ENUM: CMD1.index 3 check_links established */
#define L1P5C_CMD1_INDEX_CHECK_LINKS 3

/* typed check debug|dbg <number> */
/* ENUM: CMD1.index 4 check_debug_number established */
#define L1P5C_CMD1_INDEX_CHECK_DEBUG_NUMBER 4

/* typed check security on|off */
/* ENUM: CMD1.index 5 check_security_on established */
#define L1P5C_CMD1_INDEX_CHECK_SECURITY_ON 5

/* typed check serial <word> */
/* ENUM: CMD1.index 6 check_serial_word established */
#define L1P5C_CMD1_INDEX_CHECK_SERIAL_WORD 6

/* typed fru capture|c */
/* ENUM: CMD1.index 7 fru_capture established */
#define L1P5C_CMD1_INDEX_FRU_CAPTURE 7

/* typed fru capture|c verbose|v */
/* ENUM: CMD1.index 8 fru_capture_verbose established */
#define L1P5C_CMD1_INDEX_FRU_CAPTURE_VERBOSE 8

/* typed fru dump|d */
/* ENUM: CMD1.index 9 fru_dump established */
#define L1P5C_CMD1_INDEX_FRU_DUMP 9

/* typed fru reset|r */
/* ENUM: CMD1.index 10 fru_reset established */
#define L1P5C_CMD1_INDEX_FRU_RESET 10

/* typed fru status|s */
/* ENUM: CMD1.index 11 fru_status established */
#define L1P5C_CMD1_INDEX_FRU_STATUS 11

/* typed fru status|s verbose|v */
/* ENUM: CMD1.index 12 fru_status_verbose established */
#define L1P5C_CMD1_INDEX_FRU_STATUS_VERBOSE 12

/* typed deadlock */
/* ENUM: CMD1.index 13 deadlock established */
#define L1P5C_CMD1_INDEX_DEADLOCK 13

/* typed deadlock verbose|v */
/* ENUM: CMD1.index 14 deadlock_verbose established */
#define L1P5C_CMD1_INDEX_DEADLOCK_VERBOSE 14

/* typed memcfg (hidden) */
/* ENUM: CMD1.index 15 memcfg established */
#define L1P5C_CMD1_INDEX_MEMCFG 15

/* typed memcfg 1bank166 e|enable|d|disable (hidden) */
/* ENUM: CMD1.index 16 memcfg_1bank166_e established */
#define L1P5C_CMD1_INDEX_MEMCFG_1BANK166_E 16

/* typed shcfg */
/* ENUM: CMD1.index 17 shcfg established */
#define L1P5C_CMD1_INDEX_SHCFG 17

/* typed shcfg fsb reset|rst */
/* ENUM: CMD1.index 18 shcfg_fsb_reset established */
#define L1P5C_CMD1_INDEX_SHCFG_FSB_RESET 18

/* typed shcfg fsb */
/* ENUM: CMD1.index 19 shcfg_fsb established */
#define L1P5C_CMD1_INDEX_SHCFG_FSB 19

/* typed shcfg fsb <number> <number> <number> <number> <number> <number> <number> */
/* ENUM: CMD1.index 20 shcfg_fsb_number_number_number_number_number_number_number established */
#define L1P5C_CMD1_INDEX_SHCFG_FSB_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER 20

/* typed shcfg wrt90 reset|rst */
/* ENUM: CMD1.index 21 shcfg_wrt90_reset established */
#define L1P5C_CMD1_INDEX_SHCFG_WRT90_RESET 21

/* typed shcfg wrt90 */
/* ENUM: CMD1.index 22 shcfg_wrt90 established */
#define L1P5C_CMD1_INDEX_SHCFG_WRT90 22

/* typed shcfg wrt90 <number> <number> <number> */
/* ENUM: CMD1.index 23 shcfg_wrt90_number_number_number established */
#define L1P5C_CMD1_INDEX_SHCFG_WRT90_NUMBER_NUMBER_NUMBER 23

/* typed shcfg mci reset|rst */
/* ENUM: CMD1.index 24 shcfg_mci_reset established */
#define L1P5C_CMD1_INDEX_SHCFG_MCI_RESET 24

/* typed shcfg mci */
/* ENUM: CMD1.index 25 shcfg_mci established */
#define L1P5C_CMD1_INDEX_SHCFG_MCI 25

/* typed shcfg mci <number> <number> <number> */
/* ENUM: CMD1.index 26 shcfg_mci_number_number_number established */
#define L1P5C_CMD1_INDEX_SHCFG_MCI_NUMBER_NUMBER_NUMBER 26

/* typed shcfg mdclk */
/* ENUM: CMD1.index 27 shcfg_mdclk established */
#define L1P5C_CMD1_INDEX_SHCFG_MDCLK 27

/* typed shcfg mdclk <number> */
/* ENUM: CMD1.index 28 shcfg_mdclk_number established */
#define L1P5C_CMD1_INDEX_SHCFG_MDCLK_NUMBER 28

/* typed shcfg niclk */
/* ENUM: CMD1.index 29 shcfg_niclk established */
#define L1P5C_CMD1_INDEX_SHCFG_NICLK 29

/* typed shcfg niclk <number> */
/* ENUM: CMD1.index 30 shcfg_niclk_number established */
#define L1P5C_CMD1_INDEX_SHCFG_NICLK_NUMBER 30

/* typed shcfg osc */
/* ENUM: CMD1.index 31 shcfg_osc established */
#define L1P5C_CMD1_INDEX_SHCFG_OSC 31

/* typed shcfg osc <number> */
/* ENUM: CMD1.index 32 shcfg_osc_number established */
#define L1P5C_CMD1_INDEX_SHCFG_OSC_NUMBER 32

/* typed shcfg gtl_re */
/* ENUM: CMD1.index 33 shcfg_gtl_re established */
#define L1P5C_CMD1_INDEX_SHCFG_GTL_RE 33

/* typed shcfg gtl_re <number> */
/* ENUM: CMD1.index 34 shcfg_gtl_re_number established */
#define L1P5C_CMD1_INDEX_SHCFG_GTL_RE_NUMBER 34

/* typed shcfg jclkstop */
/* ENUM: CMD1.index 35 shcfg_jclkstop established */
#define L1P5C_CMD1_INDEX_SHCFG_JCLKSTOP 35

/* typed shcfg jclkstop <number> */
/* ENUM: CMD1.index 36 shcfg_jclkstop_number established */
#define L1P5C_CMD1_INDEX_SHCFG_JCLKSTOP_NUMBER 36

/* typed shcfg tp */
/* ENUM: CMD1.index 37 shcfg_tp established */
#define L1P5C_CMD1_INDEX_SHCFG_TP 37

/* typed shcfg tp <number> */
/* ENUM: CMD1.index 38 shcfg_tp_number established */
#define L1P5C_CMD1_INDEX_SHCFG_TP_NUMBER 38

/* typed nicfg ni0|ni1 */
/* ENUM: CMD1.index 39 nicfg_ni0 established */
#define L1P5C_CMD1_INDEX_NICFG_NI0 39

/* typed nicfg <number> ni0|ni1 */
/* ENUM: CMD1.index 40 nicfg_number_ni0 established */
#define L1P5C_CMD1_INDEX_NICFG_NUMBER_NI0 40

/* typed nicfg ni0|ni1 <number> */
/* ENUM: CMD1.index 41 nicfg_ni0_number established */
#define L1P5C_CMD1_INDEX_NICFG_NI0_NUMBER 41

/* typed nicfg ni0|ni1 <number> <number> */
/* ENUM: CMD1.index 42 nicfg_ni0_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NI0_NUMBER_NUMBER 42

/* typed nicfg ni0|ni1 <number> <number> <number> */
/* ENUM: CMD1.index 43 nicfg_ni0_number_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NI0_NUMBER_NUMBER_NUMBER 43

/* typed nicfg ni0|ni1 <number> <number> <number> <number> */
/* ENUM: CMD1.index 44 nicfg_ni0_number_number_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NI0_NUMBER_NUMBER_NUMBER_NUMBER 44

/* typed nicfg ni0|ni1 <number> <number> <number> <number> <number> */
/* ENUM: CMD1.index 45 nicfg_ni0_number_number_number_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NI0_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER 45

/* typed nicfg <number> ni0|ni1 <number> */
/* ENUM: CMD1.index 46 nicfg_number_ni0_number established */
#define L1P5C_CMD1_INDEX_NICFG_NUMBER_NI0_NUMBER 46

/* typed nicfg <number> ni0|ni1 <number> <number> */
/* ENUM: CMD1.index 47 nicfg_number_ni0_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NUMBER_NI0_NUMBER_NUMBER 47

/* typed nicfg <number> ni0|ni1 <number> <number> <number> */
/* ENUM: CMD1.index 48 nicfg_number_ni0_number_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER 48

/* typed nicfg <number> ni0|ni1 <number> <number> <number> <number> */
/* ENUM: CMD1.index 49 nicfg_number_ni0_number_number_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER_NUMBER 49

/* typed nicfg <number> ni0|ni1 <number> <number> <number> <number> <number> */
/* ENUM: CMD1.index 50 nicfg_number_ni0_number_number_number_number_number established */
#define L1P5C_CMD1_INDEX_NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER 50

/* typed rmmr|mmr_rd <word> <number> */
/* ENUM: CMD1.index 51 rmmr_word_number established */
#define L1P5C_CMD1_INDEX_RMMR_WORD_NUMBER 51

/* typed wmmr|mmr_wr <word> <number> <number> <number> */
/* ENUM: CMD1.index 52 wmmr_word_number_number_number established */
#define L1P5C_CMD1_INDEX_WMMR_WORD_NUMBER_NUMBER_NUMBER 52

/* typed pktgen read|r <word> <number> */
/* ENUM: CMD1.index 53 pktgen_read_word_number established */
#define L1P5C_CMD1_INDEX_PKTGEN_READ_WORD_NUMBER 53

/* typed pktgen write|w <word> <number> <number> <number> */
/* ENUM: CMD1.index 54 pktgen_write_word_number_number_number established */
#define L1P5C_CMD1_INDEX_PKTGEN_WRITE_WORD_NUMBER_NUMBER_NUMBER 54

/* typed pktgen debug <number> */
/* ENUM: CMD1.index 55 pktgen_debug_number established */
#define L1P5C_CMD1_INDEX_PKTGEN_DEBUG_NUMBER 55

/* typed tune get */
/* ENUM: CMD1.index 56 tune_get established */
#define L1P5C_CMD1_INDEX_TUNE_GET 56

/* typed promice */
/* ENUM: CMD1.index 57 promice established */
#define L1P5C_CMD1_INDEX_PROMICE 57

/* typed promice e */
/* ENUM: CMD1.index 58 promice_e established */
#define L1P5C_CMD1_INDEX_PROMICE_E 58

/* typed promice d */
/* ENUM: CMD1.index 59 promice_d established */
#define L1P5C_CMD1_INDEX_PROMICE_D 59

/* typed promice <number> e */
/* ENUM: CMD1.index 60 promice_number_e established */
#define L1P5C_CMD1_INDEX_PROMICE_NUMBER_E 60

/* typed promice <number> d */
/* ENUM: CMD1.index 61 promice_number_d established */
#define L1P5C_CMD1_INDEX_PROMICE_NUMBER_D 61

/* typed promver|promversion */
/* ENUM: CMD1.index 62 promver established */
#define L1P5C_CMD1_INDEX_PROMVER 62

/* typed node */
/* ENUM: CMD1.index 63 node established */
#define L1P5C_CMD1_INDEX_NODE 63

/* typed node e */
/* ENUM: CMD1.index 64 node_e established */
#define L1P5C_CMD1_INDEX_NODE_E 64

/* typed node d */
/* ENUM: CMD1.index 65 node_d established */
#define L1P5C_CMD1_INDEX_NODE_D 65

/* typed node <number> e */
/* ENUM: CMD1.index 66 node_number_e established */
#define L1P5C_CMD1_INDEX_NODE_NUMBER_E 66

/* typed node <number> d */
/* ENUM: CMD1.index 67 node_number_d established */
#define L1P5C_CMD1_INDEX_NODE_NUMBER_D 67

/* typed pic mode */
/* ENUM: CMD1.index 68 pic_mode established */
#define L1P5C_CMD1_INDEX_PIC_MODE 68

/* typed war */
/* ENUM: CMD1.index 69 war established */
#define L1P5C_CMD1_INDEX_WAR 69

/* typed war e */
/* ENUM: CMD1.index 70 war_e established */
#define L1P5C_CMD1_INDEX_WAR_E 70

/* typed war d */
/* ENUM: CMD1.index 71 war_d established */
#define L1P5C_CMD1_INDEX_WAR_D 71

/* typed shdbg */
/* ENUM: CMD1.index 72 shdbg established */
#define L1P5C_CMD1_INDEX_SHDBG 72

/* typed shdbg retrylimit <number> */
/* ENUM: CMD1.index 73 shdbg_retrylimit_number established */
#define L1P5C_CMD1_INDEX_SHDBG_RETRYLIMIT_NUMBER 73

/* typed shdbg rdelay <number> */
/* ENUM: CMD1.index 74 shdbg_rdelay_number established */
#define L1P5C_CMD1_INDEX_SHDBG_RDELAY_NUMBER 74

/* typed shdbg fullreset|fr <number> */
/* ENUM: CMD1.index 75 shdbg_fullreset_number established */
#define L1P5C_CMD1_INDEX_SHDBG_FULLRESET_NUMBER 75

/* typed shdbg linkdown|ld <number> */
/* ENUM: CMD1.index 76 shdbg_linkdown_number established */
#define L1P5C_CMD1_INDEX_SHDBG_LINKDOWN_NUMBER 76

/* typed mbrick */
/* ENUM: CMD1.index 77 mbrick established */
#define L1P5C_CMD1_INDEX_MBRICK 77

/* typed mbrick memory */
/* ENUM: CMD1.index 78 mbrick_memory established */
#define L1P5C_CMD1_INDEX_MBRICK_MEMORY 78

/* typed mbrick memory confirm (hidden) */
/* ENUM: CMD1.index 79 mbrick_memory_confirm established */
#define L1P5C_CMD1_INDEX_MBRICK_MEMORY_CONFIRM 79

/* typed mbrick cpu <word> <word> <word> <word> */
/* ENUM: CMD1.index 80 mbrick_cpu_word_word_word_word established */
#define L1P5C_CMD1_INDEX_MBRICK_CPU_WORD_WORD_WORD_WORD 80

/* typed mbrick cpu */
/* ENUM: CMD1.index 81 mbrick_cpu established */
#define L1P5C_CMD1_INDEX_MBRICK_CPU 81

/* typed if I only had a brain (hidden) */
/* ENUM: CMD1.index 82 if_i_only_had_a_brain established */
#define L1P5C_CMD1_INDEX_IF_I_ONLY_HAD_A_BRAIN 82

/* typed ssi */
/* ENUM: CMD1.index 83 ssi established */
#define L1P5C_CMD1_INDEX_SSI 83

/* typed ssi service <word> <word> <word> <word> */
/* ENUM: CMD1.index 84 ssi_service_word_word_word_word established */
#define L1P5C_CMD1_INDEX_SSI_SERVICE_WORD_WORD_WORD_WORD 84

/* typed ssi service off */
/* ENUM: CMD1.index 85 ssi_service_off established */
#define L1P5C_CMD1_INDEX_SSI_SERVICE_OFF 85

/* typed ssi <word> <word> <word> <word> <word> */
/* ENUM: CMD1.index 86 ssi_word_word_word_word_word established */
#define L1P5C_CMD1_INDEX_SSI_WORD_WORD_WORD_WORD_WORD 86

/* typed mad with power <number> (hidden) */
/* ENUM: CMD1.index 87 mad_with_power_number established */
#define L1P5C_CMD1_INDEX_MAD_WITH_POWER_NUMBER 87

/* typed pwm read */
/* ENUM: CMD1.index 88 pwm_read established */
#define L1P5C_CMD1_INDEX_PWM_READ 88

/* typed pwm set <number> <number> */
/* ENUM: CMD1.index 89 pwm_set_number_number established */
#define L1P5C_CMD1_INDEX_PWM_SET_NUMBER_NUMBER 89

/* typed syscom sim on */
/* ENUM: CMD1.index 90 sys_link_sim_on established */
#define L1P5C_CMD1_INDEX_SYS_LINK_SIM_ON 90

/* typed syscom sim on <number> <number> */
/* ENUM: CMD1.index 91 sys_link_sim_on_number_number established */
#define L1P5C_CMD1_INDEX_SYS_LINK_SIM_ON_NUMBER_NUMBER 91

/* typed syscom sim off */
/* ENUM: CMD1.index 92 sys_link_sim_off established */
#define L1P5C_CMD1_INDEX_SYS_LINK_SIM_OFF 92

/* typed syscom dump|d input|i <number> */
/* ENUM: CMD1.index 93 sys_link_dump_input_number established */
#define L1P5C_CMD1_INDEX_SYS_LINK_DUMP_INPUT_NUMBER 93

/* typed syscom dump|d output|o <number> */
/* ENUM: CMD1.index 94 sys_link_dump_output_number established */
#define L1P5C_CMD1_INDEX_SYS_LINK_DUMP_OUTPUT_NUMBER 94

/* typed syscom capture reset|rst */
/* ENUM: CMD1.index 95 sys_link_capture_reset established */
#define L1P5C_CMD1_INDEX_SYS_LINK_CAPTURE_RESET 95

/* typed syscom log <number> */
/* ENUM: CMD1.index 96 sys_link_log_number established */
#define L1P5C_CMD1_INDEX_SYS_LINK_LOG_NUMBER 96

/* typed syscom log all|* */
/* ENUM: CMD1.index 97 sys_link_log_all established */
#define L1P5C_CMD1_INDEX_SYS_LINK_LOG_ALL 97

/* typed syscom log reset|rst */
/* ENUM: CMD1.index 98 sys_link_log_reset established */
#define L1P5C_CMD1_INDEX_SYS_LINK_LOG_RESET 98

/* typed syscom log <number> insert <command> */
/* ENUM: CMD1.index 99 sys_link_log_number_insert_command established */
#define L1P5C_CMD1_INDEX_SYS_LINK_LOG_NUMBER_INSERT_COMMAND 99

/* typed syscom config|cfg */
/* ENUM: CMD1.index 100 sys_link_config established */
#define L1P5C_CMD1_INDEX_SYS_LINK_CONFIG 100

/* typed syscom qsuspend */
/* ENUM: CMD1.index 101 sys_link_qsuspend established */
#define L1P5C_CMD1_INDEX_SYS_LINK_QSUSPEND 101

/* typed syscom qsuspend <number> */
/* ENUM: CMD1.index 102 sys_link_qsuspend_number established */
#define L1P5C_CMD1_INDEX_SYS_LINK_QSUSPEND_NUMBER 102

/* typed error <number> */
/* ENUM: CMD1.index 103 error_number established */
#define L1P5C_CMD1_INDEX_ERROR_NUMBER 103

/* FIELD: CMD2.index 7:0 established */
#define L1P5C_CMD2_INDEX_SHIFT 0
#define L1P5C_CMD2_INDEX_MASK 0xffu

/* typed pci */
/* ENUM: CMD2.index 0 pci established */
#define L1P5C_CMD2_INDEX_PCI 0

/* typed pci u */
/* ENUM: CMD2.index 1 pci_u established */
#define L1P5C_CMD2_INDEX_PCI_U 1

/* typed pci d */
/* ENUM: CMD2.index 2 pci_d established */
#define L1P5C_CMD2_INDEX_PCI_D 2

/* typed pci rst|reset */
/* ENUM: CMD2.index 3 pci_rst established */
#define L1P5C_CMD2_INDEX_PCI_RST 3

/* typed pci rst|reset hold */
/* ENUM: CMD2.index 4 pci_rst_hold established */
#define L1P5C_CMD2_INDEX_PCI_RST_HOLD 4

/* typed pci rst|reset hold <number> */
/* ENUM: CMD2.index 5 pci_rst_hold_number established */
#define L1P5C_CMD2_INDEX_PCI_RST_HOLD_NUMBER 5

/* typed pci rst|reset hold <number> <number> */
/* ENUM: CMD2.index 6 pci_rst_hold_number_number established */
#define L1P5C_CMD2_INDEX_PCI_RST_HOLD_NUMBER_NUMBER 6

/* typed pci rst|reset release|rel */
/* ENUM: CMD2.index 7 pci_rst_release established */
#define L1P5C_CMD2_INDEX_PCI_RST_RELEASE 7

/* typed pci rst|reset release|rel <number> */
/* ENUM: CMD2.index 8 pci_rst_release_number established */
#define L1P5C_CMD2_INDEX_PCI_RST_RELEASE_NUMBER 8

/* typed pci rst|reset release|rel <number> <number> */
/* ENUM: CMD2.index 9 pci_rst_release_number_number established */
#define L1P5C_CMD2_INDEX_PCI_RST_RELEASE_NUMBER_NUMBER 9

/* typed pci <number> u */
/* ENUM: CMD2.index 10 pci_number_u established */
#define L1P5C_CMD2_INDEX_PCI_NUMBER_U 10

/* typed pci <number> d */
/* ENUM: CMD2.index 11 pci_number_d established */
#define L1P5C_CMD2_INDEX_PCI_NUMBER_D 11

/* typed pci <number> rst|reset */
/* ENUM: CMD2.index 12 pci_number_rst established */
#define L1P5C_CMD2_INDEX_PCI_NUMBER_RST 12

/* typed pci <number> <number> u */
/* ENUM: CMD2.index 13 pci_number_number_u established */
#define L1P5C_CMD2_INDEX_PCI_NUMBER_NUMBER_U 13

/* typed pci <number> <number> d */
/* ENUM: CMD2.index 14 pci_number_number_d established */
#define L1P5C_CMD2_INDEX_PCI_NUMBER_NUMBER_D 14

/* typed pci <number> <number> rst|reset */
/* ENUM: CMD2.index 15 pci_number_number_rst established */
#define L1P5C_CMD2_INDEX_PCI_NUMBER_NUMBER_RST 15

/* typed pci force all on */
/* ENUM: CMD2.index 16 pci_force_all_on established */
#define L1P5C_CMD2_INDEX_PCI_FORCE_ALL_ON 16

/* typed pci mode limit|l <number> pci33|pci66|pcix66|pcix100|pcix133 (hidden) */
/* ENUM: CMD2.index 17 pci_mode_limit_number_pci33 established */
#define L1P5C_CMD2_INDEX_PCI_MODE_LIMIT_NUMBER_PCI33 17

/* typed pci mode limit|l pci33|pci66|pcix66|pcix100|pcix133 (hidden) */
/* ENUM: CMD2.index 18 pci_mode_limit_pci33 established */
#define L1P5C_CMD2_INDEX_PCI_MODE_LIMIT_PCI33 18

/* typed pci mode force|f <number> pci33|pci66|pcix66|pcix100|pcix133 (hidden) */
/* ENUM: CMD2.index 19 pci_mode_force_number_pci33 established */
#define L1P5C_CMD2_INDEX_PCI_MODE_FORCE_NUMBER_PCI33 19

/* typed pci mode force|f pci33|pci66|pcix66|pcix100|pcix133 (hidden) */
/* ENUM: CMD2.index 20 pci_mode_force_pci33 established */
#define L1P5C_CMD2_INDEX_PCI_MODE_FORCE_PCI33 20

/* typed pci mode default|d <number> (hidden) */
/* ENUM: CMD2.index 21 pci_mode_default_number established */
#define L1P5C_CMD2_INDEX_PCI_MODE_DEFAULT_NUMBER 21

/* typed pci mode default|d (hidden) */
/* ENUM: CMD2.index 22 pci_mode_default established */
#define L1P5C_CMD2_INDEX_PCI_MODE_DEFAULT 22

/* typed pci mode warning disable|d (hidden) */
/* ENUM: CMD2.index 23 pci_mode_warning_disable established */
#define L1P5C_CMD2_INDEX_PCI_MODE_WARNING_DISABLE 23

/* typed pci mode warning enable|e (hidden) */
/* ENUM: CMD2.index 24 pci_mode_warning_enable established */
#define L1P5C_CMD2_INDEX_PCI_MODE_WARNING_ENABLE 24

#endif
