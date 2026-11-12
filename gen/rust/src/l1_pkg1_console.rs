// generated from defs/l1_pkg1_console.toml by defs-gen - do not edit, change the source
// console command tables of ColdFire L1 firmware package 1
#![allow(dead_code)]

/// (established)
pub mod cmd0 {
    pub const WIDTH: u32 = 32;
    pub mod index { // bits 7:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0xff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const PCI: u32 = 0; // established
        pub const PCI_U: u32 = 1; // established
        pub const PCI_D: u32 = 2; // established
        pub const PCI_RST: u32 = 3; // established
        pub const PCI_NUMBER_U: u32 = 4; // established
        pub const PCI_NUMBER_D: u32 = 5; // established
        pub const PCI_NUMBER_RST: u32 = 6; // established
        pub const PCI_NUMBER_NUMBER_U: u32 = 7; // established
        pub const PCI_NUMBER_NUMBER_D: u32 = 8; // established
        pub const PCI_NUMBER_NUMBER_RST: u32 = 9; // established
        pub const PCI_POWER_CHECK: u32 = 10; // established
        pub const PCI_POWER_CHECK_ON: u32 = 11; // established
        pub const PCI_POWER_CHECK_OFF: u32 = 12; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Pci,
            PciU,
            PciD,
            PciRst,
            PciNumberU,
            PciNumberD,
            PciNumberRst,
            PciNumberNumberU,
            PciNumberNumberD,
            PciNumberNumberRst,
            PciPowerCheck,
            PciPowerCheckOn,
            PciPowerCheckOff,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::Pci,
                    1 => Self::PciU,
                    2 => Self::PciD,
                    3 => Self::PciRst,
                    4 => Self::PciNumberU,
                    5 => Self::PciNumberD,
                    6 => Self::PciNumberRst,
                    7 => Self::PciNumberNumberU,
                    8 => Self::PciNumberNumberD,
                    9 => Self::PciNumberNumberRst,
                    10 => Self::PciPowerCheck,
                    11 => Self::PciPowerCheckOn,
                    12 => Self::PciPowerCheckOff,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Pci => 0,
                    Self::PciU => 1,
                    Self::PciD => 2,
                    Self::PciRst => 3,
                    Self::PciNumberU => 4,
                    Self::PciNumberD => 5,
                    Self::PciNumberRst => 6,
                    Self::PciNumberNumberU => 7,
                    Self::PciNumberNumberD => 8,
                    Self::PciNumberNumberRst => 9,
                    Self::PciPowerCheck => 10,
                    Self::PciPowerCheckOn => 11,
                    Self::PciPowerCheckOff => 12,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// (established)
pub mod cmd1 {
    pub const WIDTH: u32 = 32;
    pub mod index { // bits 8:0, established
        pub const SHIFT: u32 = 0;
        pub const MASK: u32 = 0x1ff;
        pub const fn get(v: u32) -> u32 {
            (v >> SHIFT) & MASK
        }
        pub const fn set(v: u32, x: u32) -> u32 {
            (v & !(MASK << SHIFT)) | ((x & MASK) << SHIFT)
        }
        pub const ALL_COMMAND: u32 = 0; // established
        pub const RACK_SLOT_COMMAND: u32 = 1; // established
        pub const AUTOPOWER: u32 = 2; // established
        pub const AUTOPOWER_ON: u32 = 3; // established
        pub const AUTOPOWER_OFF: u32 = 4; // established
        pub const AUTOPOWER_ABORT: u32 = 5; // established
        pub const SYS_LINK: u32 = 6; // established
        pub const SYS_LINK_NONE: u32 = 7; // established
        pub const SYS_LINK_PPP: u32 = 8; // established
        pub const SYS_LINK_ENABLE: u32 = 9; // established
        pub const SYS_LINK_RESET: u32 = 10; // established
        pub const SYS_LINK_PACKETS: u32 = 11; // established
        pub const BRICK: u32 = 12; // established
        pub const BRICK_TYPE_WORD: u32 = 13; // established
        pub const BRICK_PTYPE_WORD: u32 = 14; // established
        pub const BRICK_RACK_DECIMAL: u32 = 15; // established
        pub const BRICK_SLOT_DECIMAL: u32 = 16; // established
        pub const BRICK_RACKSLOT_DECIMAL_DECIMAL: u32 = 17; // established
        pub const BRICK_RACKSLOT_RESET_NUMBER_NUMBER: u32 = 18; // established
        pub const BRICK_PARTITION_NONE: u32 = 19; // established
        pub const BRICK_PARTITION_NUMBER: u32 = 20; // established
        pub const PARTDB: u32 = 21; // established
        pub const PARTDB_ENABLE: u32 = 22; // established
        pub const PARTDB_DISABLE: u32 = 23; // established
        pub const CPU: u32 = 24; // established
        pub const CPU_E: u32 = 25; // established
        pub const CPU_D: u32 = 26; // established
        pub const CPU_WORD_E: u32 = 27; // established
        pub const CPU_WORD_D: u32 = 28; // established
        pub const NIA_COMMAND: u32 = 29; // established
        pub const NIB_COMMAND: u32 = 30; // established
        pub const IIA_COMMAND: u32 = 31; // established
        pub const IIB_COMMAND: u32 = 32; // established
        pub const IIC_COMMAND: u32 = 33; // established
        pub const IID_COMMAND: u32 = 34; // established
        pub const CONFIG: u32 = 35; // established
        pub const CONFIG_V: u32 = 36; // established
        pub const CONFIG_RESET: u32 = 37; // established
        pub const CONFIG_RING: u32 = 38; // established
        pub const DEBUG: u32 = 39; // established
        pub const DEBUG_NUMBER: u32 = 40; // established
        pub const DISPLAY: u32 = 41; // established
        pub const DISPLAY_NUMBER_COMMAND: u32 = 42; // established
        pub const DISPLAY_POWER_ON: u32 = 43; // established
        pub const DISPLAY_ATTENTION_ON: u32 = 44; // established
        pub const DISPLAY_FAILURE_ON: u32 = 45; // established
        pub const DISPLAY_POWER_OFF: u32 = 46; // established
        pub const DISPLAY_ATTENTION_OFF: u32 = 47; // established
        pub const DISPLAY_FAILURE_OFF: u32 = 48; // established
        pub const BUTTON: u32 = 49; // established
        pub const BUTTON_ALL_ENABLE: u32 = 50; // established
        pub const BUTTON_ALL_DISABLE: u32 = 51; // established
        pub const BUTTON_PWR_ENABLE: u32 = 52; // established
        pub const BUTTON_PWR_DISABLE: u32 = 53; // established
        pub const BUTTON_RST_ENABLE: u32 = 54; // established
        pub const BUTTON_RST_DISABLE: u32 = 55; // established
        pub const BUTTON_NMI_ENABLE: u32 = 56; // established
        pub const BUTTON_NMI_DISABLE: u32 = 57; // established
        pub const LET_THE_CARNAGE_BEGIN: u32 = 58; // established
        pub const SLIPPED_MY_MIND: u32 = 59; // established
        pub const SUPER_SIZED_PLEASE: u32 = 60; // established
        pub const ENV: u32 = 61; // established
        pub const ENV_CHECK: u32 = 62; // established
        pub const ENV_ON: u32 = 63; // established
        pub const ENV_OFF: u32 = 64; // established
        pub const ENV_POWER_ON: u32 = 65; // established
        pub const ENV_POWER_OFF: u32 = 66; // established
        pub const ENV_POWER_MONITOR: u32 = 67; // established
        pub const ENV_TEMP_ON: u32 = 68; // established
        pub const ENV_TEMP_OFF: u32 = 69; // established
        pub const ENV_ALTITUDE_HIGH: u32 = 70; // established
        pub const ENV_ALTITUDE_LOW: u32 = 71; // established
        pub const ENV_RESET: u32 = 72; // established
        pub const ENV_TEST_WARNING: u32 = 73; // established
        pub const ENV_TEST_FAULT: u32 = 74; // established
        pub const ENV_TEST_TMPADV: u32 = 75; // established
        pub const ENV_TEST_TMPCRIT: u32 = 76; // established
        pub const ENV_TEST_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 77; // established
        pub const ENV_TEST_PWRDN: u32 = 78; // established
        pub const ENV_MESSAGES: u32 = 79; // established
        pub const ENV_MESSAGES_FAULTSONLY: u32 = 80; // established
        pub const ENV_MESSAGES_ALL: u32 = 81; // established
        pub const FAN: u32 = 82; // established
        pub const FAN_ON_NUMBER: u32 = 83; // established
        pub const FAN_ON: u32 = 84; // established
        pub const FAN_SPEED: u32 = 85; // established
        pub const FAN_SPEED_NUMBER_NUMBER: u32 = 86; // established
        pub const FAN_SPEED_NUMBER: u32 = 87; // established
        pub const FAN_SPEED_LOW_NUMBER: u32 = 88; // established
        pub const FAN_SPEED_LOW: u32 = 89; // established
        pub const FAN_AUTO_ON: u32 = 90; // established
        pub const HELP: u32 = 91; // established
        pub const HELP_WORD: u32 = 92; // established
        pub const HELP_LIST_HIDDEN: u32 = 93; // established
        pub const HELP_WORD_HIDDEN: u32 = 94; // established
        pub const HISTORY: u32 = 95; // established
        pub const L1DBG: u32 = 96; // established
        pub const L1DBG_SYS_LINK_NUMBER: u32 = 97; // established
        pub const L1DBG_SYS_LINK_NUMBER_NUMBER: u32 = 98; // established
        pub const L1DBG_IRTR_NUMBER: u32 = 99; // established
        pub const L1DBG_ENV_NUMBER: u32 = 100; // established
        pub const L1DBG_ENV_OFF: u32 = 101; // established
        pub const L1DBG_PORT_ON: u32 = 102; // established
        pub const L1DBG_I2C_ON: u32 = 103; // established
        pub const L1DBG_MARGIN_ON: u32 = 104; // established
        pub const L1DBG_CONSOLE_ON: u32 = 105; // established
        pub const L1DBG_MEMPANIC_ON: u32 = 106; // established
        pub const L1DBG_PPPDUMP_ON: u32 = 107; // established
        pub const L1DBG_PPPDELAY_NUMBER: u32 = 108; // established
        pub const L1DBG_PROMREQ_ON: u32 = 109; // established
        pub const L1DBG_QSUSP_NUMBER_NUMBER: u32 = 110; // established
        pub const L1DBG_PRINTF_ON: u32 = 111; // established
        pub const L1DBG_FLASHIGNORE_NUMBER_NUMBER: u32 = 112; // established
        pub const L1DBG_MISC_NUMBER: u32 = 113; // established
        pub const LINK: u32 = 114; // established
        pub const LINK_VERBOSE: u32 = 115; // established
        pub const LOG: u32 = 116; // established
        pub const LOG_CLEAR: u32 = 117; // established
        pub const LOG_INSERT_COMMAND: u32 = 118; // established
        pub const IOPORT: u32 = 119; // established
        pub const IOPORT_400: u32 = 120; // established
        pub const IOPORT_A_400: u32 = 121; // established
        pub const IOPORT_B_400: u32 = 122; // established
        pub const IOPORT_FORCE_400: u32 = 123; // established
        pub const IOPORT_FORCE_ALT1: u32 = 124; // established
        pub const IOPORT_FORCE_ALT2: u32 = 125; // established
        pub const IOPORT_CLKSRC_A: u32 = 126; // established
        pub const IOPORT_CLKSRC_B: u32 = 127; // established
        pub const ISTAT_MEMORY: u32 = 128; // established
        pub const ISTAT_QUEUES: u32 = 129; // established
        pub const ISTAT_TASKS: u32 = 130; // established
        pub const ISTAT_PMALLOC: u32 = 131; // established
        pub const ISTAT_DUMP_WORD: u32 = 132; // established
        pub const B2B: u32 = 133; // established
        pub const L1: u32 = 134; // established
        pub const LEDS: u32 = 135; // established
        pub const MARGIN: u32 = 136; // established
        pub const MARGIN_DEFAULT: u32 = 137; // established
        pub const MARGIN_GBRICK_DEFAULT: u32 = 138; // established
        pub const MARGIN_NUMBER_DEFAULT: u32 = 139; // established
        pub const MARGIN_NUMBER_NUMBER: u32 = 140; // established
        pub const MARGIN_DIMM_V_WORD: u32 = 141; // established
        pub const MARGIN_PCI_NORM2HI: u32 = 142; // established
        pub const MARGIN_PCI_NORM2HI_ON: u32 = 143; // established
        pub const NETWORK: u32 = 144; // established
        pub const NETWORK_USB: u32 = 145; // established
        pub const NETWORK_AUTODETECT_ON: u32 = 146; // established
        pub const PIMM: u32 = 147; // established
        pub const PIMM_AUTO: u32 = 148; // established
        pub const PIMM_CLKSRC_EXT: u32 = 149; // established
        pub const PIMM_CLKSRC_INT: u32 = 150; // established
        pub const PIMM_CLKMODE_ASYNC: u32 = 151; // established
        pub const PIMM_CLKMODE_SYNC: u32 = 152; // established
        pub const PORT: u32 = 153; // established
        pub const POWER: u32 = 154; // established
        pub const POWER_CHECK: u32 = 155; // established
        pub const POWER_VRM: u32 = 156; // established
        pub const POWER_UP: u32 = 157; // established
        pub const POWER_UP_HOLD: u32 = 158; // established
        pub const POWER_DOWN: u32 = 159; // established
        pub const RESET: u32 = 160; // established
        pub const RESET_HOLD: u32 = 161; // established
        pub const NMI: u32 = 162; // established
        pub const NMI_NUMBER: u32 = 163; // established
        pub const NMI_RACK_SLOT_WORD: u32 = 164; // established
        pub const SOFTRESET: u32 = 165; // established
        pub const SOFTRESET_NUMBER: u32 = 166; // established
        pub const SELECT: u32 = 167; // established
        pub const SELECT_NUMBER_NUMBER: u32 = 168; // established
        pub const SELECT_SUBCHANNEL_CONSOLE: u32 = 169; // established
        pub const SELECT_SUBCHANNEL_NUMBER: u32 = 170; // established
        pub const SELECT_SUBCHANNEL_A: u32 = 171; // established
        pub const SELECT_SUBCHANNEL_WORD: u32 = 172; // established
        pub const SELECT_ENABLE: u32 = 173; // established
        pub const SELECT_DISABLE: u32 = 174; // established
        pub const SELECT_FILTER_ON: u32 = 175; // established
        pub const SELECT_FILTER_OFF: u32 = 176; // established
        pub const SERIAL: u32 = 177; // established
        pub const SERIAL_VERIFY: u32 = 178; // established
        pub const SERIAL_ALL: u32 = 179; // established
        pub const SERIAL_ALL_V: u32 = 180; // established
        pub const SERIAL_DIMM: u32 = 181; // established
        pub const SERIAL_DIMM_V: u32 = 182; // established
        pub const SERIAL_CLEAR: u32 = 183; // established
        pub const SERIAL_SET_WORD: u32 = 184; // established
        pub const SERIAL_WORD_WORD_WORD_WORD: u32 = 185; // established
        pub const SERIAL_SECURITY_OFF_WORD_WORD_WORD_WORD: u32 = 186; // established
        pub const SERIAL_SECURITY_ON: u32 = 187; // established
        pub const SERIAL_BRICK_DISABLE: u32 = 188; // established
        pub const SERIAL_BRICK_DISABLE_CONFIRM: u32 = 189; // established
        pub const SYSSTATE: u32 = 190; // established
        pub const EEPROM: u32 = 191; // established
        pub const EEPROM_NUMBER_NUMBER_NUMBER: u32 = 192; // established
        pub const EEPROM_FUEL_WRITE_DEFAULT: u32 = 193; // established
        pub const EEPROM_DIMM_UNSIGN: u32 = 194; // established
        pub const UART: u32 = 195; // established
        pub const UART_SYS_LINK_RESET: u32 = 196; // established
        pub const USB: u32 = 197; // established
        pub const ROUTER: u32 = 198; // established
        pub const ROUTER_META: u32 = 199; // established
        pub const ROUTER_REPEATER: u32 = 200; // established
        pub const ROUTER_ORDINARY: u32 = 201; // established
        pub const ROUTER_6_PORT: u32 = 202; // established
        pub const ROUTER_6_PORT_CONFIRM: u32 = 203; // established
        pub const ROUTER_8_PORT: u32 = 204; // established
        pub const ROUTER_8_PORT_WORD_WORD_WORD_WORD: u32 = 205; // established
        pub const ROUTER_SERVICE_WORD_WORD_WORD_WORD: u32 = 206; // established
        pub const ROUTER_SERVICE_OFF: u32 = 207; // established
        pub const ROUTER_SPARE: u32 = 208; // established
        pub const SERVICE_WORD_WORD_WORD_WORD: u32 = 209; // established
        pub const SERVICE_OFF: u32 = 210; // established
        pub const DATE_TZ: u32 = 211; // established
        pub const DATE_TZ_WORD: u32 = 212; // established
        pub const DATE: u32 = 213; // established
        pub const DATE_RAW: u32 = 214; // established
        pub const DATE_WORD: u32 = 215; // established
        pub const DATE_WORD_WORD: u32 = 216; // established
        pub const NVRAM_RESET: u32 = 217; // established
        pub const NVRAM_READ_NUMBER_NUMBER: u32 = 218; // established
        pub const NVRAM_WRITE_NUMBER_NUMBER: u32 = 219; // established
        pub const NVRAM_SCRATCH: u32 = 220; // established
        pub const NVRAM_SCRATCH_NUMBER: u32 = 221; // established
        pub const L2_COMMAND: u32 = 222; // established
        pub const SECURITY_VERIFY_3400: u32 = 223; // established
        pub const SECURITY_VERIFY_3800: u32 = 224; // established
        pub const FLASH_DEFAULT_RESET: u32 = 225; // established
        pub const FLASH_DEFAULT_A: u32 = 226; // established
        pub const FLASH_DEFAULT_B: u32 = 227; // established
        pub const FLASH_DEFAULT_CURRENT: u32 = 228; // established
        pub const FLASH_DEFAULT_OLD: u32 = 229; // established
        pub const FLASH_DEFAULT_NEW: u32 = 230; // established
        pub const FLASH_STATUS: u32 = 231; // established
        pub const FLASH_STATUS_V: u32 = 232; // established
        pub const REBOOT_L1: u32 = 233; // established
        pub const REBOOT_L1_A: u32 = 234; // established
        pub const REBOOT_L1_B: u32 = 235; // established
        pub const REBOOT_L1_CURRENT: u32 = 236; // established
        pub const REBOOT_L1_OLD: u32 = 237; // established
        pub const REBOOT_L1_NEW: u32 = 238; // established
        pub const REBOOT_L1_OTHER: u32 = 239; // established
        pub const REBOOT_L1_ADDR_NUMBER: u32 = 240; // established
        pub const REBOOT_L1_RAW_NUMBER: u32 = 241; // established
        pub const VERSION: u32 = 242; // established
        pub const PBAY: u32 = 243; // established
        pub const PBAY_VERSION: u32 = 244; // established
        pub const PBAY_DPS: u32 = 245; // established
        pub const PBAY_DPS_WORD: u32 = 246; // established
        pub const PBAY_DCPORT: u32 = 247; // established
        pub const PBAY_DCPORT_WORD: u32 = 248; // established
        pub const PBAY_FRU: u32 = 249; // established
        pub const PBAY_FRU_WORD: u32 = 250; // established
        pub const PBAY_RESET: u32 = 251; // established
        pub const PBAY_ENV: u32 = 252; // established
        pub const PBAY_ENV_ON: u32 = 253; // established
        pub const PBAY_ENV_OFF: u32 = 254; // established
        pub const PBAY_INIT: u32 = 255; // established
        pub const PBAY_SERIAL: u32 = 256; // established
        pub const TEST_I2C: u32 = 257; // established
        pub const TEST_I2C_NUMBER: u32 = 258; // established
        pub const TEST_IOEXP_SET_NUMBER_WORD: u32 = 259; // established
        pub const TEST_IOEXP_GET_NUMBER: u32 = 260; // established
        pub const TEST_IOEXP_GET_ALL: u32 = 261; // established
        pub const TEST_INTR: u32 = 262; // established
        pub const TEST_DISPLAY: u32 = 263; // established
        pub const TEST_EXCEPTION_MEM: u32 = 264; // established
        pub const TEST_EXCEPTION_STACK: u32 = 265; // established
        pub const TEST_EXCEPTION_WDOG: u32 = 266; // established
        pub const TEST_EXCEPTION_FATAL: u32 = 267; // established
        pub const SCAN_RESET: u32 = 268; // established
        pub const SCAN_RESET_HARD: u32 = 269; // established
        pub const SCAN_RESET_SOFT: u32 = 270; // established
        pub const SCAN_RESET_BOTH: u32 = 271; // established
        pub const SCAN_SEL_NUMBER_NUMBER: u32 = 272; // established
        pub const SCAN_SEL_NUMBER_NUMBER_NUMBER: u32 = 273; // established
        pub const SCAN_SEL_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 274; // established
        pub const SCAN_IDS_NUMBER: u32 = 275; // established
        pub const SCAN_SET_TRST_0: u32 = 276; // established
        pub const SCAN_SET_TRST_1: u32 = 277; // established
        pub const SCAN_SET_TCK_0: u32 = 278; // established
        pub const SCAN_SET_TCK_1: u32 = 279; // established
        pub const SCAN_SET_TMS_0: u32 = 280; // established
        pub const SCAN_SET_TMS_1: u32 = 281; // established
        pub const SCAN_SET_TDI_0: u32 = 282; // established
        pub const SCAN_SET_TDI_1: u32 = 283; // established
        pub const SCAN_SET_PSI_0: u32 = 284; // established
        pub const SCAN_SET_PSI_1: u32 = 285; // established
        pub const SCAN_GET: u32 = 286; // established
        pub const SCAN_DEBUG_NUMBER: u32 = 287; // established
        pub const SCAN_DEBUG: u32 = 288; // established
        pub const SCAN_IMP_SET_DLLBYP_NUMBER: u32 = 289; // established
        pub const SCAN_IMP_SET_NUMBER: u32 = 290; // established
        pub const SCAN_IMP_UNLOCK: u32 = 291; // established
        pub const SCAN_TEST_NUMBER: u32 = 292; // established
        pub const FRU: u32 = 293; // established
        pub const FRU_COMMAND: u32 = 294; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            AllCommand,
            RackSlotCommand,
            Autopower,
            AutopowerOn,
            AutopowerOff,
            AutopowerAbort,
            SysLink,
            SysLinkNone,
            SysLinkPpp,
            SysLinkEnable,
            SysLinkReset,
            SysLinkPackets,
            Brick,
            BrickTypeWord,
            BrickPtypeWord,
            BrickRackDecimal,
            BrickSlotDecimal,
            BrickRackslotDecimalDecimal,
            BrickRackslotResetNumberNumber,
            BrickPartitionNone,
            BrickPartitionNumber,
            Partdb,
            PartdbEnable,
            PartdbDisable,
            Cpu,
            CpuE,
            CpuD,
            CpuWordE,
            CpuWordD,
            NiaCommand,
            NibCommand,
            IiaCommand,
            IibCommand,
            IicCommand,
            IidCommand,
            Config,
            ConfigV,
            ConfigReset,
            ConfigRing,
            Debug,
            DebugNumber,
            Display,
            DisplayNumberCommand,
            DisplayPowerOn,
            DisplayAttentionOn,
            DisplayFailureOn,
            DisplayPowerOff,
            DisplayAttentionOff,
            DisplayFailureOff,
            Button,
            ButtonAllEnable,
            ButtonAllDisable,
            ButtonPwrEnable,
            ButtonPwrDisable,
            ButtonRstEnable,
            ButtonRstDisable,
            ButtonNmiEnable,
            ButtonNmiDisable,
            LetTheCarnageBegin,
            SlippedMyMind,
            SuperSizedPlease,
            Env,
            EnvCheck,
            EnvOn,
            EnvOff,
            EnvPowerOn,
            EnvPowerOff,
            EnvPowerMonitor,
            EnvTempOn,
            EnvTempOff,
            EnvAltitudeHigh,
            EnvAltitudeLow,
            EnvReset,
            EnvTestWarning,
            EnvTestFault,
            EnvTestTmpadv,
            EnvTestTmpcrit,
            EnvTestNumberNumberNumberNumberNumber,
            EnvTestPwrdn,
            EnvMessages,
            EnvMessagesFaultsonly,
            EnvMessagesAll,
            Fan,
            FanOnNumber,
            FanOn,
            FanSpeed,
            FanSpeedNumberNumber,
            FanSpeedNumber,
            FanSpeedLowNumber,
            FanSpeedLow,
            FanAutoOn,
            Help,
            HelpWord,
            HelpListHidden,
            HelpWordHidden,
            History,
            L1dbg,
            L1dbgSysLinkNumber,
            L1dbgSysLinkNumberNumber,
            L1dbgIrtrNumber,
            L1dbgEnvNumber,
            L1dbgEnvOff,
            L1dbgPortOn,
            L1dbgI2cOn,
            L1dbgMarginOn,
            L1dbgConsoleOn,
            L1dbgMempanicOn,
            L1dbgPppdumpOn,
            L1dbgPppdelayNumber,
            L1dbgPromreqOn,
            L1dbgQsuspNumberNumber,
            L1dbgPrintfOn,
            L1dbgFlashignoreNumberNumber,
            L1dbgMiscNumber,
            Link,
            LinkVerbose,
            Log,
            LogClear,
            LogInsertCommand,
            Ioport,
            Ioport400,
            IoportA400,
            IoportB400,
            IoportForce400,
            IoportForceAlt1,
            IoportForceAlt2,
            IoportClksrcA,
            IoportClksrcB,
            IstatMemory,
            IstatQueues,
            IstatTasks,
            IstatPmalloc,
            IstatDumpWord,
            B2b,
            L1,
            Leds,
            Margin,
            MarginDefault,
            MarginGbrickDefault,
            MarginNumberDefault,
            MarginNumberNumber,
            MarginDimmVWord,
            MarginPciNorm2hi,
            MarginPciNorm2hiOn,
            Network,
            NetworkUsb,
            NetworkAutodetectOn,
            Pimm,
            PimmAuto,
            PimmClksrcExt,
            PimmClksrcInt,
            PimmClkmodeAsync,
            PimmClkmodeSync,
            Port,
            Power,
            PowerCheck,
            PowerVrm,
            PowerUp,
            PowerUpHold,
            PowerDown,
            Reset,
            ResetHold,
            Nmi,
            NmiNumber,
            NmiRackSlotWord,
            Softreset,
            SoftresetNumber,
            Select,
            SelectNumberNumber,
            SelectSubchannelConsole,
            SelectSubchannelNumber,
            SelectSubchannelA,
            SelectSubchannelWord,
            SelectEnable,
            SelectDisable,
            SelectFilterOn,
            SelectFilterOff,
            Serial,
            SerialVerify,
            SerialAll,
            SerialAllV,
            SerialDimm,
            SerialDimmV,
            SerialClear,
            SerialSetWord,
            SerialWordWordWordWord,
            SerialSecurityOffWordWordWordWord,
            SerialSecurityOn,
            SerialBrickDisable,
            SerialBrickDisableConfirm,
            Sysstate,
            Eeprom,
            EepromNumberNumberNumber,
            EepromFuelWriteDefault,
            EepromDimmUnsign,
            Uart,
            UartSysLinkReset,
            Usb,
            Router,
            RouterMeta,
            RouterRepeater,
            RouterOrdinary,
            Router6Port,
            Router6PortConfirm,
            Router8Port,
            Router8PortWordWordWordWord,
            RouterServiceWordWordWordWord,
            RouterServiceOff,
            RouterSpare,
            ServiceWordWordWordWord,
            ServiceOff,
            DateTz,
            DateTzWord,
            Date,
            DateRaw,
            DateWord,
            DateWordWord,
            NvramReset,
            NvramReadNumberNumber,
            NvramWriteNumberNumber,
            NvramScratch,
            NvramScratchNumber,
            L2Command,
            SecurityVerify3400,
            SecurityVerify3800,
            FlashDefaultReset,
            FlashDefaultA,
            FlashDefaultB,
            FlashDefaultCurrent,
            FlashDefaultOld,
            FlashDefaultNew,
            FlashStatus,
            FlashStatusV,
            RebootL1,
            RebootL1A,
            RebootL1B,
            RebootL1Current,
            RebootL1Old,
            RebootL1New,
            RebootL1Other,
            RebootL1AddrNumber,
            RebootL1RawNumber,
            Version,
            Pbay,
            PbayVersion,
            PbayDps,
            PbayDpsWord,
            PbayDcport,
            PbayDcportWord,
            PbayFru,
            PbayFruWord,
            PbayReset,
            PbayEnv,
            PbayEnvOn,
            PbayEnvOff,
            PbayInit,
            PbaySerial,
            TestI2c,
            TestI2cNumber,
            TestIoexpSetNumberWord,
            TestIoexpGetNumber,
            TestIoexpGetAll,
            TestIntr,
            TestDisplay,
            TestExceptionMem,
            TestExceptionStack,
            TestExceptionWdog,
            TestExceptionFatal,
            ScanReset,
            ScanResetHard,
            ScanResetSoft,
            ScanResetBoth,
            ScanSelNumberNumber,
            ScanSelNumberNumberNumber,
            ScanSelNumberNumberNumberNumber,
            ScanIdsNumber,
            ScanSetTrst0,
            ScanSetTrst1,
            ScanSetTck0,
            ScanSetTck1,
            ScanSetTms0,
            ScanSetTms1,
            ScanSetTdi0,
            ScanSetTdi1,
            ScanSetPsi0,
            ScanSetPsi1,
            ScanGet,
            ScanDebugNumber,
            ScanDebug,
            ScanImpSetDllbypNumber,
            ScanImpSetNumber,
            ScanImpUnlock,
            ScanTestNumber,
            Fru,
            FruCommand,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::AllCommand,
                    1 => Self::RackSlotCommand,
                    2 => Self::Autopower,
                    3 => Self::AutopowerOn,
                    4 => Self::AutopowerOff,
                    5 => Self::AutopowerAbort,
                    6 => Self::SysLink,
                    7 => Self::SysLinkNone,
                    8 => Self::SysLinkPpp,
                    9 => Self::SysLinkEnable,
                    10 => Self::SysLinkReset,
                    11 => Self::SysLinkPackets,
                    12 => Self::Brick,
                    13 => Self::BrickTypeWord,
                    14 => Self::BrickPtypeWord,
                    15 => Self::BrickRackDecimal,
                    16 => Self::BrickSlotDecimal,
                    17 => Self::BrickRackslotDecimalDecimal,
                    18 => Self::BrickRackslotResetNumberNumber,
                    19 => Self::BrickPartitionNone,
                    20 => Self::BrickPartitionNumber,
                    21 => Self::Partdb,
                    22 => Self::PartdbEnable,
                    23 => Self::PartdbDisable,
                    24 => Self::Cpu,
                    25 => Self::CpuE,
                    26 => Self::CpuD,
                    27 => Self::CpuWordE,
                    28 => Self::CpuWordD,
                    29 => Self::NiaCommand,
                    30 => Self::NibCommand,
                    31 => Self::IiaCommand,
                    32 => Self::IibCommand,
                    33 => Self::IicCommand,
                    34 => Self::IidCommand,
                    35 => Self::Config,
                    36 => Self::ConfigV,
                    37 => Self::ConfigReset,
                    38 => Self::ConfigRing,
                    39 => Self::Debug,
                    40 => Self::DebugNumber,
                    41 => Self::Display,
                    42 => Self::DisplayNumberCommand,
                    43 => Self::DisplayPowerOn,
                    44 => Self::DisplayAttentionOn,
                    45 => Self::DisplayFailureOn,
                    46 => Self::DisplayPowerOff,
                    47 => Self::DisplayAttentionOff,
                    48 => Self::DisplayFailureOff,
                    49 => Self::Button,
                    50 => Self::ButtonAllEnable,
                    51 => Self::ButtonAllDisable,
                    52 => Self::ButtonPwrEnable,
                    53 => Self::ButtonPwrDisable,
                    54 => Self::ButtonRstEnable,
                    55 => Self::ButtonRstDisable,
                    56 => Self::ButtonNmiEnable,
                    57 => Self::ButtonNmiDisable,
                    58 => Self::LetTheCarnageBegin,
                    59 => Self::SlippedMyMind,
                    60 => Self::SuperSizedPlease,
                    61 => Self::Env,
                    62 => Self::EnvCheck,
                    63 => Self::EnvOn,
                    64 => Self::EnvOff,
                    65 => Self::EnvPowerOn,
                    66 => Self::EnvPowerOff,
                    67 => Self::EnvPowerMonitor,
                    68 => Self::EnvTempOn,
                    69 => Self::EnvTempOff,
                    70 => Self::EnvAltitudeHigh,
                    71 => Self::EnvAltitudeLow,
                    72 => Self::EnvReset,
                    73 => Self::EnvTestWarning,
                    74 => Self::EnvTestFault,
                    75 => Self::EnvTestTmpadv,
                    76 => Self::EnvTestTmpcrit,
                    77 => Self::EnvTestNumberNumberNumberNumberNumber,
                    78 => Self::EnvTestPwrdn,
                    79 => Self::EnvMessages,
                    80 => Self::EnvMessagesFaultsonly,
                    81 => Self::EnvMessagesAll,
                    82 => Self::Fan,
                    83 => Self::FanOnNumber,
                    84 => Self::FanOn,
                    85 => Self::FanSpeed,
                    86 => Self::FanSpeedNumberNumber,
                    87 => Self::FanSpeedNumber,
                    88 => Self::FanSpeedLowNumber,
                    89 => Self::FanSpeedLow,
                    90 => Self::FanAutoOn,
                    91 => Self::Help,
                    92 => Self::HelpWord,
                    93 => Self::HelpListHidden,
                    94 => Self::HelpWordHidden,
                    95 => Self::History,
                    96 => Self::L1dbg,
                    97 => Self::L1dbgSysLinkNumber,
                    98 => Self::L1dbgSysLinkNumberNumber,
                    99 => Self::L1dbgIrtrNumber,
                    100 => Self::L1dbgEnvNumber,
                    101 => Self::L1dbgEnvOff,
                    102 => Self::L1dbgPortOn,
                    103 => Self::L1dbgI2cOn,
                    104 => Self::L1dbgMarginOn,
                    105 => Self::L1dbgConsoleOn,
                    106 => Self::L1dbgMempanicOn,
                    107 => Self::L1dbgPppdumpOn,
                    108 => Self::L1dbgPppdelayNumber,
                    109 => Self::L1dbgPromreqOn,
                    110 => Self::L1dbgQsuspNumberNumber,
                    111 => Self::L1dbgPrintfOn,
                    112 => Self::L1dbgFlashignoreNumberNumber,
                    113 => Self::L1dbgMiscNumber,
                    114 => Self::Link,
                    115 => Self::LinkVerbose,
                    116 => Self::Log,
                    117 => Self::LogClear,
                    118 => Self::LogInsertCommand,
                    119 => Self::Ioport,
                    120 => Self::Ioport400,
                    121 => Self::IoportA400,
                    122 => Self::IoportB400,
                    123 => Self::IoportForce400,
                    124 => Self::IoportForceAlt1,
                    125 => Self::IoportForceAlt2,
                    126 => Self::IoportClksrcA,
                    127 => Self::IoportClksrcB,
                    128 => Self::IstatMemory,
                    129 => Self::IstatQueues,
                    130 => Self::IstatTasks,
                    131 => Self::IstatPmalloc,
                    132 => Self::IstatDumpWord,
                    133 => Self::B2b,
                    134 => Self::L1,
                    135 => Self::Leds,
                    136 => Self::Margin,
                    137 => Self::MarginDefault,
                    138 => Self::MarginGbrickDefault,
                    139 => Self::MarginNumberDefault,
                    140 => Self::MarginNumberNumber,
                    141 => Self::MarginDimmVWord,
                    142 => Self::MarginPciNorm2hi,
                    143 => Self::MarginPciNorm2hiOn,
                    144 => Self::Network,
                    145 => Self::NetworkUsb,
                    146 => Self::NetworkAutodetectOn,
                    147 => Self::Pimm,
                    148 => Self::PimmAuto,
                    149 => Self::PimmClksrcExt,
                    150 => Self::PimmClksrcInt,
                    151 => Self::PimmClkmodeAsync,
                    152 => Self::PimmClkmodeSync,
                    153 => Self::Port,
                    154 => Self::Power,
                    155 => Self::PowerCheck,
                    156 => Self::PowerVrm,
                    157 => Self::PowerUp,
                    158 => Self::PowerUpHold,
                    159 => Self::PowerDown,
                    160 => Self::Reset,
                    161 => Self::ResetHold,
                    162 => Self::Nmi,
                    163 => Self::NmiNumber,
                    164 => Self::NmiRackSlotWord,
                    165 => Self::Softreset,
                    166 => Self::SoftresetNumber,
                    167 => Self::Select,
                    168 => Self::SelectNumberNumber,
                    169 => Self::SelectSubchannelConsole,
                    170 => Self::SelectSubchannelNumber,
                    171 => Self::SelectSubchannelA,
                    172 => Self::SelectSubchannelWord,
                    173 => Self::SelectEnable,
                    174 => Self::SelectDisable,
                    175 => Self::SelectFilterOn,
                    176 => Self::SelectFilterOff,
                    177 => Self::Serial,
                    178 => Self::SerialVerify,
                    179 => Self::SerialAll,
                    180 => Self::SerialAllV,
                    181 => Self::SerialDimm,
                    182 => Self::SerialDimmV,
                    183 => Self::SerialClear,
                    184 => Self::SerialSetWord,
                    185 => Self::SerialWordWordWordWord,
                    186 => Self::SerialSecurityOffWordWordWordWord,
                    187 => Self::SerialSecurityOn,
                    188 => Self::SerialBrickDisable,
                    189 => Self::SerialBrickDisableConfirm,
                    190 => Self::Sysstate,
                    191 => Self::Eeprom,
                    192 => Self::EepromNumberNumberNumber,
                    193 => Self::EepromFuelWriteDefault,
                    194 => Self::EepromDimmUnsign,
                    195 => Self::Uart,
                    196 => Self::UartSysLinkReset,
                    197 => Self::Usb,
                    198 => Self::Router,
                    199 => Self::RouterMeta,
                    200 => Self::RouterRepeater,
                    201 => Self::RouterOrdinary,
                    202 => Self::Router6Port,
                    203 => Self::Router6PortConfirm,
                    204 => Self::Router8Port,
                    205 => Self::Router8PortWordWordWordWord,
                    206 => Self::RouterServiceWordWordWordWord,
                    207 => Self::RouterServiceOff,
                    208 => Self::RouterSpare,
                    209 => Self::ServiceWordWordWordWord,
                    210 => Self::ServiceOff,
                    211 => Self::DateTz,
                    212 => Self::DateTzWord,
                    213 => Self::Date,
                    214 => Self::DateRaw,
                    215 => Self::DateWord,
                    216 => Self::DateWordWord,
                    217 => Self::NvramReset,
                    218 => Self::NvramReadNumberNumber,
                    219 => Self::NvramWriteNumberNumber,
                    220 => Self::NvramScratch,
                    221 => Self::NvramScratchNumber,
                    222 => Self::L2Command,
                    223 => Self::SecurityVerify3400,
                    224 => Self::SecurityVerify3800,
                    225 => Self::FlashDefaultReset,
                    226 => Self::FlashDefaultA,
                    227 => Self::FlashDefaultB,
                    228 => Self::FlashDefaultCurrent,
                    229 => Self::FlashDefaultOld,
                    230 => Self::FlashDefaultNew,
                    231 => Self::FlashStatus,
                    232 => Self::FlashStatusV,
                    233 => Self::RebootL1,
                    234 => Self::RebootL1A,
                    235 => Self::RebootL1B,
                    236 => Self::RebootL1Current,
                    237 => Self::RebootL1Old,
                    238 => Self::RebootL1New,
                    239 => Self::RebootL1Other,
                    240 => Self::RebootL1AddrNumber,
                    241 => Self::RebootL1RawNumber,
                    242 => Self::Version,
                    243 => Self::Pbay,
                    244 => Self::PbayVersion,
                    245 => Self::PbayDps,
                    246 => Self::PbayDpsWord,
                    247 => Self::PbayDcport,
                    248 => Self::PbayDcportWord,
                    249 => Self::PbayFru,
                    250 => Self::PbayFruWord,
                    251 => Self::PbayReset,
                    252 => Self::PbayEnv,
                    253 => Self::PbayEnvOn,
                    254 => Self::PbayEnvOff,
                    255 => Self::PbayInit,
                    256 => Self::PbaySerial,
                    257 => Self::TestI2c,
                    258 => Self::TestI2cNumber,
                    259 => Self::TestIoexpSetNumberWord,
                    260 => Self::TestIoexpGetNumber,
                    261 => Self::TestIoexpGetAll,
                    262 => Self::TestIntr,
                    263 => Self::TestDisplay,
                    264 => Self::TestExceptionMem,
                    265 => Self::TestExceptionStack,
                    266 => Self::TestExceptionWdog,
                    267 => Self::TestExceptionFatal,
                    268 => Self::ScanReset,
                    269 => Self::ScanResetHard,
                    270 => Self::ScanResetSoft,
                    271 => Self::ScanResetBoth,
                    272 => Self::ScanSelNumberNumber,
                    273 => Self::ScanSelNumberNumberNumber,
                    274 => Self::ScanSelNumberNumberNumberNumber,
                    275 => Self::ScanIdsNumber,
                    276 => Self::ScanSetTrst0,
                    277 => Self::ScanSetTrst1,
                    278 => Self::ScanSetTck0,
                    279 => Self::ScanSetTck1,
                    280 => Self::ScanSetTms0,
                    281 => Self::ScanSetTms1,
                    282 => Self::ScanSetTdi0,
                    283 => Self::ScanSetTdi1,
                    284 => Self::ScanSetPsi0,
                    285 => Self::ScanSetPsi1,
                    286 => Self::ScanGet,
                    287 => Self::ScanDebugNumber,
                    288 => Self::ScanDebug,
                    289 => Self::ScanImpSetDllbypNumber,
                    290 => Self::ScanImpSetNumber,
                    291 => Self::ScanImpUnlock,
                    292 => Self::ScanTestNumber,
                    293 => Self::Fru,
                    294 => Self::FruCommand,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::AllCommand => 0,
                    Self::RackSlotCommand => 1,
                    Self::Autopower => 2,
                    Self::AutopowerOn => 3,
                    Self::AutopowerOff => 4,
                    Self::AutopowerAbort => 5,
                    Self::SysLink => 6,
                    Self::SysLinkNone => 7,
                    Self::SysLinkPpp => 8,
                    Self::SysLinkEnable => 9,
                    Self::SysLinkReset => 10,
                    Self::SysLinkPackets => 11,
                    Self::Brick => 12,
                    Self::BrickTypeWord => 13,
                    Self::BrickPtypeWord => 14,
                    Self::BrickRackDecimal => 15,
                    Self::BrickSlotDecimal => 16,
                    Self::BrickRackslotDecimalDecimal => 17,
                    Self::BrickRackslotResetNumberNumber => 18,
                    Self::BrickPartitionNone => 19,
                    Self::BrickPartitionNumber => 20,
                    Self::Partdb => 21,
                    Self::PartdbEnable => 22,
                    Self::PartdbDisable => 23,
                    Self::Cpu => 24,
                    Self::CpuE => 25,
                    Self::CpuD => 26,
                    Self::CpuWordE => 27,
                    Self::CpuWordD => 28,
                    Self::NiaCommand => 29,
                    Self::NibCommand => 30,
                    Self::IiaCommand => 31,
                    Self::IibCommand => 32,
                    Self::IicCommand => 33,
                    Self::IidCommand => 34,
                    Self::Config => 35,
                    Self::ConfigV => 36,
                    Self::ConfigReset => 37,
                    Self::ConfigRing => 38,
                    Self::Debug => 39,
                    Self::DebugNumber => 40,
                    Self::Display => 41,
                    Self::DisplayNumberCommand => 42,
                    Self::DisplayPowerOn => 43,
                    Self::DisplayAttentionOn => 44,
                    Self::DisplayFailureOn => 45,
                    Self::DisplayPowerOff => 46,
                    Self::DisplayAttentionOff => 47,
                    Self::DisplayFailureOff => 48,
                    Self::Button => 49,
                    Self::ButtonAllEnable => 50,
                    Self::ButtonAllDisable => 51,
                    Self::ButtonPwrEnable => 52,
                    Self::ButtonPwrDisable => 53,
                    Self::ButtonRstEnable => 54,
                    Self::ButtonRstDisable => 55,
                    Self::ButtonNmiEnable => 56,
                    Self::ButtonNmiDisable => 57,
                    Self::LetTheCarnageBegin => 58,
                    Self::SlippedMyMind => 59,
                    Self::SuperSizedPlease => 60,
                    Self::Env => 61,
                    Self::EnvCheck => 62,
                    Self::EnvOn => 63,
                    Self::EnvOff => 64,
                    Self::EnvPowerOn => 65,
                    Self::EnvPowerOff => 66,
                    Self::EnvPowerMonitor => 67,
                    Self::EnvTempOn => 68,
                    Self::EnvTempOff => 69,
                    Self::EnvAltitudeHigh => 70,
                    Self::EnvAltitudeLow => 71,
                    Self::EnvReset => 72,
                    Self::EnvTestWarning => 73,
                    Self::EnvTestFault => 74,
                    Self::EnvTestTmpadv => 75,
                    Self::EnvTestTmpcrit => 76,
                    Self::EnvTestNumberNumberNumberNumberNumber => 77,
                    Self::EnvTestPwrdn => 78,
                    Self::EnvMessages => 79,
                    Self::EnvMessagesFaultsonly => 80,
                    Self::EnvMessagesAll => 81,
                    Self::Fan => 82,
                    Self::FanOnNumber => 83,
                    Self::FanOn => 84,
                    Self::FanSpeed => 85,
                    Self::FanSpeedNumberNumber => 86,
                    Self::FanSpeedNumber => 87,
                    Self::FanSpeedLowNumber => 88,
                    Self::FanSpeedLow => 89,
                    Self::FanAutoOn => 90,
                    Self::Help => 91,
                    Self::HelpWord => 92,
                    Self::HelpListHidden => 93,
                    Self::HelpWordHidden => 94,
                    Self::History => 95,
                    Self::L1dbg => 96,
                    Self::L1dbgSysLinkNumber => 97,
                    Self::L1dbgSysLinkNumberNumber => 98,
                    Self::L1dbgIrtrNumber => 99,
                    Self::L1dbgEnvNumber => 100,
                    Self::L1dbgEnvOff => 101,
                    Self::L1dbgPortOn => 102,
                    Self::L1dbgI2cOn => 103,
                    Self::L1dbgMarginOn => 104,
                    Self::L1dbgConsoleOn => 105,
                    Self::L1dbgMempanicOn => 106,
                    Self::L1dbgPppdumpOn => 107,
                    Self::L1dbgPppdelayNumber => 108,
                    Self::L1dbgPromreqOn => 109,
                    Self::L1dbgQsuspNumberNumber => 110,
                    Self::L1dbgPrintfOn => 111,
                    Self::L1dbgFlashignoreNumberNumber => 112,
                    Self::L1dbgMiscNumber => 113,
                    Self::Link => 114,
                    Self::LinkVerbose => 115,
                    Self::Log => 116,
                    Self::LogClear => 117,
                    Self::LogInsertCommand => 118,
                    Self::Ioport => 119,
                    Self::Ioport400 => 120,
                    Self::IoportA400 => 121,
                    Self::IoportB400 => 122,
                    Self::IoportForce400 => 123,
                    Self::IoportForceAlt1 => 124,
                    Self::IoportForceAlt2 => 125,
                    Self::IoportClksrcA => 126,
                    Self::IoportClksrcB => 127,
                    Self::IstatMemory => 128,
                    Self::IstatQueues => 129,
                    Self::IstatTasks => 130,
                    Self::IstatPmalloc => 131,
                    Self::IstatDumpWord => 132,
                    Self::B2b => 133,
                    Self::L1 => 134,
                    Self::Leds => 135,
                    Self::Margin => 136,
                    Self::MarginDefault => 137,
                    Self::MarginGbrickDefault => 138,
                    Self::MarginNumberDefault => 139,
                    Self::MarginNumberNumber => 140,
                    Self::MarginDimmVWord => 141,
                    Self::MarginPciNorm2hi => 142,
                    Self::MarginPciNorm2hiOn => 143,
                    Self::Network => 144,
                    Self::NetworkUsb => 145,
                    Self::NetworkAutodetectOn => 146,
                    Self::Pimm => 147,
                    Self::PimmAuto => 148,
                    Self::PimmClksrcExt => 149,
                    Self::PimmClksrcInt => 150,
                    Self::PimmClkmodeAsync => 151,
                    Self::PimmClkmodeSync => 152,
                    Self::Port => 153,
                    Self::Power => 154,
                    Self::PowerCheck => 155,
                    Self::PowerVrm => 156,
                    Self::PowerUp => 157,
                    Self::PowerUpHold => 158,
                    Self::PowerDown => 159,
                    Self::Reset => 160,
                    Self::ResetHold => 161,
                    Self::Nmi => 162,
                    Self::NmiNumber => 163,
                    Self::NmiRackSlotWord => 164,
                    Self::Softreset => 165,
                    Self::SoftresetNumber => 166,
                    Self::Select => 167,
                    Self::SelectNumberNumber => 168,
                    Self::SelectSubchannelConsole => 169,
                    Self::SelectSubchannelNumber => 170,
                    Self::SelectSubchannelA => 171,
                    Self::SelectSubchannelWord => 172,
                    Self::SelectEnable => 173,
                    Self::SelectDisable => 174,
                    Self::SelectFilterOn => 175,
                    Self::SelectFilterOff => 176,
                    Self::Serial => 177,
                    Self::SerialVerify => 178,
                    Self::SerialAll => 179,
                    Self::SerialAllV => 180,
                    Self::SerialDimm => 181,
                    Self::SerialDimmV => 182,
                    Self::SerialClear => 183,
                    Self::SerialSetWord => 184,
                    Self::SerialWordWordWordWord => 185,
                    Self::SerialSecurityOffWordWordWordWord => 186,
                    Self::SerialSecurityOn => 187,
                    Self::SerialBrickDisable => 188,
                    Self::SerialBrickDisableConfirm => 189,
                    Self::Sysstate => 190,
                    Self::Eeprom => 191,
                    Self::EepromNumberNumberNumber => 192,
                    Self::EepromFuelWriteDefault => 193,
                    Self::EepromDimmUnsign => 194,
                    Self::Uart => 195,
                    Self::UartSysLinkReset => 196,
                    Self::Usb => 197,
                    Self::Router => 198,
                    Self::RouterMeta => 199,
                    Self::RouterRepeater => 200,
                    Self::RouterOrdinary => 201,
                    Self::Router6Port => 202,
                    Self::Router6PortConfirm => 203,
                    Self::Router8Port => 204,
                    Self::Router8PortWordWordWordWord => 205,
                    Self::RouterServiceWordWordWordWord => 206,
                    Self::RouterServiceOff => 207,
                    Self::RouterSpare => 208,
                    Self::ServiceWordWordWordWord => 209,
                    Self::ServiceOff => 210,
                    Self::DateTz => 211,
                    Self::DateTzWord => 212,
                    Self::Date => 213,
                    Self::DateRaw => 214,
                    Self::DateWord => 215,
                    Self::DateWordWord => 216,
                    Self::NvramReset => 217,
                    Self::NvramReadNumberNumber => 218,
                    Self::NvramWriteNumberNumber => 219,
                    Self::NvramScratch => 220,
                    Self::NvramScratchNumber => 221,
                    Self::L2Command => 222,
                    Self::SecurityVerify3400 => 223,
                    Self::SecurityVerify3800 => 224,
                    Self::FlashDefaultReset => 225,
                    Self::FlashDefaultA => 226,
                    Self::FlashDefaultB => 227,
                    Self::FlashDefaultCurrent => 228,
                    Self::FlashDefaultOld => 229,
                    Self::FlashDefaultNew => 230,
                    Self::FlashStatus => 231,
                    Self::FlashStatusV => 232,
                    Self::RebootL1 => 233,
                    Self::RebootL1A => 234,
                    Self::RebootL1B => 235,
                    Self::RebootL1Current => 236,
                    Self::RebootL1Old => 237,
                    Self::RebootL1New => 238,
                    Self::RebootL1Other => 239,
                    Self::RebootL1AddrNumber => 240,
                    Self::RebootL1RawNumber => 241,
                    Self::Version => 242,
                    Self::Pbay => 243,
                    Self::PbayVersion => 244,
                    Self::PbayDps => 245,
                    Self::PbayDpsWord => 246,
                    Self::PbayDcport => 247,
                    Self::PbayDcportWord => 248,
                    Self::PbayFru => 249,
                    Self::PbayFruWord => 250,
                    Self::PbayReset => 251,
                    Self::PbayEnv => 252,
                    Self::PbayEnvOn => 253,
                    Self::PbayEnvOff => 254,
                    Self::PbayInit => 255,
                    Self::PbaySerial => 256,
                    Self::TestI2c => 257,
                    Self::TestI2cNumber => 258,
                    Self::TestIoexpSetNumberWord => 259,
                    Self::TestIoexpGetNumber => 260,
                    Self::TestIoexpGetAll => 261,
                    Self::TestIntr => 262,
                    Self::TestDisplay => 263,
                    Self::TestExceptionMem => 264,
                    Self::TestExceptionStack => 265,
                    Self::TestExceptionWdog => 266,
                    Self::TestExceptionFatal => 267,
                    Self::ScanReset => 268,
                    Self::ScanResetHard => 269,
                    Self::ScanResetSoft => 270,
                    Self::ScanResetBoth => 271,
                    Self::ScanSelNumberNumber => 272,
                    Self::ScanSelNumberNumberNumber => 273,
                    Self::ScanSelNumberNumberNumberNumber => 274,
                    Self::ScanIdsNumber => 275,
                    Self::ScanSetTrst0 => 276,
                    Self::ScanSetTrst1 => 277,
                    Self::ScanSetTck0 => 278,
                    Self::ScanSetTck1 => 279,
                    Self::ScanSetTms0 => 280,
                    Self::ScanSetTms1 => 281,
                    Self::ScanSetTdi0 => 282,
                    Self::ScanSetTdi1 => 283,
                    Self::ScanSetPsi0 => 284,
                    Self::ScanSetPsi1 => 285,
                    Self::ScanGet => 286,
                    Self::ScanDebugNumber => 287,
                    Self::ScanDebug => 288,
                    Self::ScanImpSetDllbypNumber => 289,
                    Self::ScanImpSetNumber => 290,
                    Self::ScanImpUnlock => 291,
                    Self::ScanTestNumber => 292,
                    Self::Fru => 293,
                    Self::FruCommand => 294,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
