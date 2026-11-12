// generated from defs/l1_pkg5_console.toml by defs-gen - do not edit, change the source
// console command tables of ColdFire L1 firmware package 5
#![allow(dead_code)]

/// (established)
pub mod cmd0 {
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
        pub const BRICK_RACK_DECIMAL: u32 = 14; // established
        pub const BRICK_SLOT_DECIMAL: u32 = 15; // established
        pub const BRICK_RACKSLOT_DECIMAL_DECIMAL: u32 = 16; // established
        pub const BRICK_RACKSLOT_RESET_NUMBER_NUMBER: u32 = 17; // established
        pub const BRICK_PARTITION_NONE: u32 = 18; // established
        pub const BRICK_PARTITION_NUMBER: u32 = 19; // established
        pub const PARTDB: u32 = 20; // established
        pub const PARTDB_ENABLE: u32 = 21; // established
        pub const PARTDB_DISABLE: u32 = 22; // established
        pub const CPU: u32 = 23; // established
        pub const CPU_E: u32 = 24; // established
        pub const CPU_WORD_E: u32 = 25; // established
        pub const B2B_WORD_COMMAND: u32 = 26; // established
        pub const CONFIG: u32 = 27; // established
        pub const CONFIG_V: u32 = 28; // established
        pub const CONFIG_RESET: u32 = 29; // established
        pub const CONFIG_RING: u32 = 30; // established
        pub const DEBUG: u32 = 31; // established
        pub const DEBUG_NUMBER: u32 = 32; // established
        pub const DISPLAY: u32 = 33; // established
        pub const DISPLAY_NUMBER_COMMAND: u32 = 34; // established
        pub const DISPLAY_POWER_ON: u32 = 35; // established
        pub const DISPLAY_ATTENTION_ON: u32 = 36; // established
        pub const DISPLAY_FAILURE_ON: u32 = 37; // established
        pub const DISPLAY_POWER_OFF: u32 = 38; // established
        pub const DISPLAY_ATTENTION_OFF: u32 = 39; // established
        pub const DISPLAY_FAILURE_OFF: u32 = 40; // established
        pub const BUTTON: u32 = 41; // established
        pub const BUTTON_ALL_ENABLE: u32 = 42; // established
        pub const BUTTON_ALL_DISABLE: u32 = 43; // established
        pub const BUTTON_PWR_ENABLE: u32 = 44; // established
        pub const BUTTON_PWR_DISABLE: u32 = 45; // established
        pub const BUTTON_RST_ENABLE: u32 = 46; // established
        pub const BUTTON_RST_DISABLE: u32 = 47; // established
        pub const BUTTON_NMI_ENABLE: u32 = 48; // established
        pub const BUTTON_NMI_DISABLE: u32 = 49; // established
        pub const LET_THE_CARNAGE_BEGIN: u32 = 50; // established
        pub const SLIPPED_MY_MIND: u32 = 51; // established
        pub const SUPER_SIZED_PLEASE: u32 = 52; // established
        pub const ENV: u32 = 53; // established
        pub const ENV_CHECK: u32 = 54; // established
        pub const ENV_ON: u32 = 55; // established
        pub const ENV_OFF: u32 = 56; // established
        pub const ENV_POWER_ON: u32 = 57; // established
        pub const ENV_POWER_OFF: u32 = 58; // established
        pub const ENV_POWER_MONITOR: u32 = 59; // established
        pub const ENV_TEMP_ON: u32 = 60; // established
        pub const ENV_TEMP_OFF: u32 = 61; // established
        pub const ENV_ALTITUDE_HIGH: u32 = 62; // established
        pub const ENV_ALTITUDE_LOW: u32 = 63; // established
        pub const ENV_RESET: u32 = 64; // established
        pub const ENV_TEST_WARNING: u32 = 65; // established
        pub const ENV_TEST_FAULT: u32 = 66; // established
        pub const ENV_TEST_TMPADV: u32 = 67; // established
        pub const ENV_TEST_TMPCRIT: u32 = 68; // established
        pub const ENV_TEST_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 69; // established
        pub const ENV_TEST_PWRDN: u32 = 70; // established
        pub const ENV_MESSAGES: u32 = 71; // established
        pub const ENV_MESSAGES_FAULTSONLY: u32 = 72; // established
        pub const ENV_MESSAGES_ALL: u32 = 73; // established
        pub const FAN: u32 = 74; // established
        pub const FAN_ON_NUMBER: u32 = 75; // established
        pub const FAN_ON: u32 = 76; // established
        pub const FAN_SPEED: u32 = 77; // established
        pub const FAN_SPEED_NUMBER_NUMBER: u32 = 78; // established
        pub const FAN_SPEED_NUMBER: u32 = 79; // established
        pub const FAN_SPEED_LOW_NUMBER: u32 = 80; // established
        pub const FAN_SPEED_LOW: u32 = 81; // established
        pub const FAN_AUTO_ON: u32 = 82; // established
        pub const HELP: u32 = 83; // established
        pub const HELP_WORD: u32 = 84; // established
        pub const HELP_LIST_HIDDEN: u32 = 85; // established
        pub const HELP_WORD_HIDDEN: u32 = 86; // established
        pub const HISTORY: u32 = 87; // established
        pub const L1DBG: u32 = 88; // established
        pub const L1DBG_SYS_LINK_NUMBER: u32 = 89; // established
        pub const L1DBG_SYS_LINK_NUMBER_NUMBER: u32 = 90; // established
        pub const L1DBG_IRTR_NUMBER: u32 = 91; // established
        pub const L1DBG_ENV_NUMBER: u32 = 92; // established
        pub const L1DBG_ENV_OFF: u32 = 93; // established
        pub const L1DBG_PORT_ON: u32 = 94; // established
        pub const L1DBG_I2C_ON: u32 = 95; // established
        pub const L1DBG_MARGIN_ON: u32 = 96; // established
        pub const L1DBG_CONSOLE_ON: u32 = 97; // established
        pub const L1DBG_MEMPANIC_ON: u32 = 98; // established
        pub const L1DBG_PPPDUMP_ON: u32 = 99; // established
        pub const L1DBG_PPPDELAY_NUMBER: u32 = 100; // established
        pub const L1DBG_PROMREQ_ON: u32 = 101; // established
        pub const L1DBG_QSUSP_NUMBER_NUMBER: u32 = 102; // established
        pub const L1DBG_PRINTF_ON: u32 = 103; // established
        pub const L1DBG_FLASHIGNORE_NUMBER_NUMBER: u32 = 104; // established
        pub const L1DBG_MISC_NUMBER: u32 = 105; // established
        pub const L1DBG_GAL_NUMBER: u32 = 106; // established
        pub const LINK: u32 = 107; // established
        pub const LINK_VERBOSE: u32 = 108; // established
        pub const LINK_CHECK: u32 = 109; // established
        pub const LINK_CHECK_V: u32 = 110; // established
        pub const LINK_STABLE: u32 = 111; // established
        pub const LOG: u32 = 112; // established
        pub const LOG_CLEAR: u32 = 113; // established
        pub const LOG_INSERT_COMMAND: u32 = 114; // established
        pub const IOPORT: u32 = 115; // established
        pub const IOPORT_400: u32 = 116; // established
        pub const IOPORT_A_400: u32 = 117; // established
        pub const IOPORT_B_400: u32 = 118; // established
        pub const IOPORT_FORCE_400: u32 = 119; // established
        pub const IOPORT_FORCE_ALT1: u32 = 120; // established
        pub const IOPORT_FORCE_ALT2: u32 = 121; // established
        pub const IOPORT_CLKSRC_A: u32 = 122; // established
        pub const IOPORT_CLKSRC_B: u32 = 123; // established
        pub const ISTAT_MEMORY: u32 = 124; // established
        pub const ISTAT_QUEUES: u32 = 125; // established
        pub const ISTAT_QUEUES_RESET: u32 = 126; // established
        pub const ISTAT_TASKS: u32 = 127; // established
        pub const ISTAT_PMALLOC: u32 = 128; // established
        pub const ISTAT_DUMP_WORD: u32 = 129; // established
        pub const B2B: u32 = 130; // established
        pub const L1: u32 = 131; // established
        pub const LEDS: u32 = 132; // established
        pub const MARGIN: u32 = 133; // established
        pub const MARGIN_DEFAULT: u32 = 134; // established
        pub const MARGIN_NUMBER_DEFAULT: u32 = 135; // established
        pub const MARGIN_NUMBER_NUMBER: u32 = 136; // established
        pub const MARGIN_DIMM_V_WORD: u32 = 137; // established
        pub const NETWORK: u32 = 138; // established
        pub const NETWORK_USB: u32 = 139; // established
        pub const NETWORK_AUTODETECT_ON: u32 = 140; // established
        pub const PIMM: u32 = 141; // established
        pub const PIMM_AUTO: u32 = 142; // established
        pub const PIMM_CLKSRC_EXT: u32 = 143; // established
        pub const PIMM_CLKSRC_INT: u32 = 144; // established
        pub const PIMM_CLKMODE_ASYNC: u32 = 145; // established
        pub const PIMM_CLKMODE_SYNC: u32 = 146; // established
        pub const PORT: u32 = 147; // established
        pub const PORT_V: u32 = 148; // established
        pub const POWER: u32 = 149; // established
        pub const POWER_CHECK: u32 = 150; // established
        pub const POWER_VRM: u32 = 151; // established
        pub const POWER_UP: u32 = 152; // established
        pub const POWER_UP_HOLD: u32 = 153; // established
        pub const POWER_DOWN: u32 = 154; // established
        pub const RESET: u32 = 155; // established
        pub const RESET_HOLD: u32 = 156; // established
        pub const NMI: u32 = 157; // established
        pub const SOFTRESET: u32 = 158; // established
        pub const SOFTRESET_NUMBER: u32 = 159; // established
        pub const SELECT: u32 = 160; // established
        pub const SELECT_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 161; // established
        pub const SELECT_SUBCHANNEL_CONSOLE: u32 = 162; // established
        pub const SELECT_SUBCHANNEL_A: u32 = 163; // established
        pub const SELECT_SUBCHANNEL_NUMBER: u32 = 164; // established
        pub const SELECT_SUBCHANNEL_WORD: u32 = 165; // established
        pub const SELECT_ENABLE: u32 = 166; // established
        pub const SELECT_DISABLE: u32 = 167; // established
        pub const SELECT_FILTER_ON: u32 = 168; // established
        pub const SELECT_FILTER_OFF: u32 = 169; // established
        pub const SERIAL: u32 = 170; // established
        pub const SERIAL_VERIFY: u32 = 171; // established
        pub const SERIAL_ALL: u32 = 172; // established
        pub const SERIAL_ALL_V: u32 = 173; // established
        pub const SERIAL_B: u32 = 174; // established
        pub const SERIAL_B_V: u32 = 175; // established
        pub const SERIAL_CPU: u32 = 176; // established
        pub const SERIAL_CPU_V: u32 = 177; // established
        pub const SERIAL_DIMM: u32 = 178; // established
        pub const SERIAL_DIMM_V: u32 = 179; // established
        pub const SERIAL_DIMM_CFG: u32 = 180; // established
        pub const SERIAL_DIMM_STATUS: u32 = 181; // established
        pub const SERIAL_CLEAR: u32 = 182; // established
        pub const SERIAL_SET_WORD: u32 = 183; // established
        pub const SERIAL_WORD_WORD_WORD_WORD: u32 = 184; // established
        pub const SERIAL_SECURITY_OFF_WORD_WORD_WORD_WORD: u32 = 185; // established
        pub const SERIAL_SECURITY_ON: u32 = 186; // established
        pub const SERIAL_BRICK_DISABLE: u32 = 187; // established
        pub const SERIAL_BRICK_DISABLE_CONFIRM: u32 = 188; // established
        pub const SYSSTATE: u32 = 189; // established
        pub const EEPROM: u32 = 190; // established
        pub const EEPROM_NUMBER_NUMBER_NUMBER: u32 = 191; // established
        pub const EEPROM_FILL_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 192; // established
        pub const UART: u32 = 193; // established
        pub const UART_SYS_LINK_RESET: u32 = 194; // established
        pub const USB: u32 = 195; // established
        pub const ROUTER: u32 = 196; // established
        pub const ROUTER_META: u32 = 197; // established
        pub const ROUTER_META1: u32 = 198; // established
        pub const ROUTER_REPEATER: u32 = 199; // established
        pub const ROUTER_ORDINARY: u32 = 200; // established
        pub const ROUTER_6_PORT: u32 = 201; // established
        pub const ROUTER_6_PORT_CONFIRM: u32 = 202; // established
        pub const ROUTER_8_PORT: u32 = 203; // established
        pub const ROUTER_8_PORT_WORD_WORD_WORD_WORD: u32 = 204; // established
        pub const ROUTER_SERVICE_WORD_WORD_WORD_WORD: u32 = 205; // established
        pub const ROUTER_SERVICE_OFF: u32 = 206; // established
        pub const ROUTER_SPARE: u32 = 207; // established
        pub const SERVICE_WORD_WORD_WORD_WORD: u32 = 208; // established
        pub const SERVICE_OFF: u32 = 209; // established
        pub const DATE_TZ: u32 = 210; // established
        pub const DATE_TZ_WORD: u32 = 211; // established
        pub const DATE: u32 = 212; // established
        pub const DATE_RAW: u32 = 213; // established
        pub const DATE_WORD: u32 = 214; // established
        pub const DATE_WORD_WORD: u32 = 215; // established
        pub const NVRAM_RESET: u32 = 216; // established
        pub const NVRAM_READ_NUMBER_NUMBER: u32 = 217; // established
        pub const NVRAM_WRITE_NUMBER_NUMBER: u32 = 218; // established
        pub const NVRAM_SCRATCH: u32 = 219; // established
        pub const NVRAM_SCRATCH_NUMBER: u32 = 220; // established
        pub const L2_COMMAND: u32 = 221; // established
        pub const SECURITY_VERIFY_3400: u32 = 222; // established
        pub const SECURITY_VERIFY_3800: u32 = 223; // established
        pub const SLEEP_NUMBER_NUMBER_COMMAND: u32 = 224; // established
        pub const MTB: u32 = 225; // established
        pub const FLASH_DEFAULT_RESET: u32 = 226; // established
        pub const FLASH_DEFAULT_A: u32 = 227; // established
        pub const FLASH_DEFAULT_B: u32 = 228; // established
        pub const FLASH_DEFAULT_CURRENT: u32 = 229; // established
        pub const FLASH_DEFAULT_OLD: u32 = 230; // established
        pub const FLASH_DEFAULT_NEW: u32 = 231; // established
        pub const FLASH_STATUS: u32 = 232; // established
        pub const FLASH_STATUS_V: u32 = 233; // established
        pub const REBOOT_L1: u32 = 234; // established
        pub const REBOOT_L1_A: u32 = 235; // established
        pub const REBOOT_L1_B: u32 = 236; // established
        pub const REBOOT_L1_CURRENT: u32 = 237; // established
        pub const REBOOT_L1_OLD: u32 = 238; // established
        pub const REBOOT_L1_NEW: u32 = 239; // established
        pub const REBOOT_L1_OTHER: u32 = 240; // established
        pub const REBOOT_L1_ADDR_NUMBER: u32 = 241; // established
        pub const REBOOT_L1_RAW_NUMBER: u32 = 242; // established
        pub const VERSION: u32 = 243; // established
        pub const PBAY: u32 = 244; // established
        pub const PBAY_VERSION: u32 = 245; // established
        pub const PBAY_DPS: u32 = 246; // established
        pub const PBAY_DPS_WORD: u32 = 247; // established
        pub const PBAY_DCPORT: u32 = 248; // established
        pub const PBAY_DCPORT_WORD: u32 = 249; // established
        pub const PBAY_FRU: u32 = 250; // established
        pub const PBAY_FRU_WORD: u32 = 251; // established
        pub const PBAY_RESET: u32 = 252; // established
        pub const PBAY_ENV: u32 = 253; // established
        pub const PBAY_ENV_ON: u32 = 254; // established
        pub const PBAY_ENV_OFF: u32 = 255; // established
        pub const PBAY_INIT: u32 = 256; // established
        pub const PBAY_SERIAL: u32 = 257; // established
        pub const TEST_I2C: u32 = 258; // established
        pub const TEST_I2C_NUMBER: u32 = 259; // established
        pub const TEST_IOEXP_SET_NUMBER_NUMBER: u32 = 260; // established
        pub const TEST_IOEXP_GET_NUMBER: u32 = 261; // established
        pub const TEST_IOEXP_GET_ALL: u32 = 262; // established
        pub const TEST_INTR: u32 = 263; // established
        pub const TEST_DISPLAY: u32 = 264; // established
        pub const TEST_EXCEPTION_MEM: u32 = 265; // established
        pub const TEST_EXCEPTION_STACK: u32 = 266; // established
        pub const TEST_EXCEPTION_WDOG: u32 = 267; // established
        pub const TEST_EXCEPTION_FATAL: u32 = 268; // established
        pub const SCAN_RESET: u32 = 269; // established
        pub const SCAN_RESET_HARD: u32 = 270; // established
        pub const SCAN_RESET_SOFT: u32 = 271; // established
        pub const SCAN_RESET_BOTH: u32 = 272; // established
        pub const SCAN_SEL_NUMBER_NUMBER: u32 = 273; // established
        pub const SCAN_SEL_NUMBER_NUMBER_NUMBER: u32 = 274; // established
        pub const SCAN_SEL_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 275; // established
        pub const SCAN_IDS_NUMBER: u32 = 276; // established
        pub const SCAN_SET_TRST_0: u32 = 277; // established
        pub const SCAN_SET_TRST_1: u32 = 278; // established
        pub const SCAN_SET_TCK_0: u32 = 279; // established
        pub const SCAN_SET_TCK_1: u32 = 280; // established
        pub const SCAN_SET_TMS_0: u32 = 281; // established
        pub const SCAN_SET_TMS_1: u32 = 282; // established
        pub const SCAN_SET_TDI_0: u32 = 283; // established
        pub const SCAN_SET_TDI_1: u32 = 284; // established
        pub const SCAN_SET_PSI_0: u32 = 285; // established
        pub const SCAN_SET_PSI_1: u32 = 286; // established
        pub const SCAN_GET: u32 = 287; // established
        pub const SCAN_DEBUG_NUMBER: u32 = 288; // established
        pub const SCAN_DEBUG: u32 = 289; // established
        pub const SCAN_CFG: u32 = 290; // established
        pub const SCAN_IMP_SET_DLLBYP_NUMBER: u32 = 291; // established
        pub const SCAN_IMP_SET_NUMBER: u32 = 292; // established
        pub const SCAN_IMP_UNLOCK: u32 = 293; // established
        pub const SCAN_TEST_NUMBER: u32 = 294; // established
        pub const FRU: u32 = 295; // established
        pub const FRU_COMMAND: u32 = 296; // established
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
            CpuWordE,
            B2bWordCommand,
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
            L1dbgGalNumber,
            Link,
            LinkVerbose,
            LinkCheck,
            LinkCheckV,
            LinkStable,
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
            IstatQueuesReset,
            IstatTasks,
            IstatPmalloc,
            IstatDumpWord,
            B2b,
            L1,
            Leds,
            Margin,
            MarginDefault,
            MarginNumberDefault,
            MarginNumberNumber,
            MarginDimmVWord,
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
            PortV,
            Power,
            PowerCheck,
            PowerVrm,
            PowerUp,
            PowerUpHold,
            PowerDown,
            Reset,
            ResetHold,
            Nmi,
            Softreset,
            SoftresetNumber,
            Select,
            SelectNumberNumberNumberNumber,
            SelectSubchannelConsole,
            SelectSubchannelA,
            SelectSubchannelNumber,
            SelectSubchannelWord,
            SelectEnable,
            SelectDisable,
            SelectFilterOn,
            SelectFilterOff,
            Serial,
            SerialVerify,
            SerialAll,
            SerialAllV,
            SerialB,
            SerialBV,
            SerialCpu,
            SerialCpuV,
            SerialDimm,
            SerialDimmV,
            SerialDimmCfg,
            SerialDimmStatus,
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
            EepromFillNumberNumberNumberNumber,
            Uart,
            UartSysLinkReset,
            Usb,
            Router,
            RouterMeta,
            RouterMeta1,
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
            SleepNumberNumberCommand,
            Mtb,
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
            TestIoexpSetNumberNumber,
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
            ScanCfg,
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
                    14 => Self::BrickRackDecimal,
                    15 => Self::BrickSlotDecimal,
                    16 => Self::BrickRackslotDecimalDecimal,
                    17 => Self::BrickRackslotResetNumberNumber,
                    18 => Self::BrickPartitionNone,
                    19 => Self::BrickPartitionNumber,
                    20 => Self::Partdb,
                    21 => Self::PartdbEnable,
                    22 => Self::PartdbDisable,
                    23 => Self::Cpu,
                    24 => Self::CpuE,
                    25 => Self::CpuWordE,
                    26 => Self::B2bWordCommand,
                    27 => Self::Config,
                    28 => Self::ConfigV,
                    29 => Self::ConfigReset,
                    30 => Self::ConfigRing,
                    31 => Self::Debug,
                    32 => Self::DebugNumber,
                    33 => Self::Display,
                    34 => Self::DisplayNumberCommand,
                    35 => Self::DisplayPowerOn,
                    36 => Self::DisplayAttentionOn,
                    37 => Self::DisplayFailureOn,
                    38 => Self::DisplayPowerOff,
                    39 => Self::DisplayAttentionOff,
                    40 => Self::DisplayFailureOff,
                    41 => Self::Button,
                    42 => Self::ButtonAllEnable,
                    43 => Self::ButtonAllDisable,
                    44 => Self::ButtonPwrEnable,
                    45 => Self::ButtonPwrDisable,
                    46 => Self::ButtonRstEnable,
                    47 => Self::ButtonRstDisable,
                    48 => Self::ButtonNmiEnable,
                    49 => Self::ButtonNmiDisable,
                    50 => Self::LetTheCarnageBegin,
                    51 => Self::SlippedMyMind,
                    52 => Self::SuperSizedPlease,
                    53 => Self::Env,
                    54 => Self::EnvCheck,
                    55 => Self::EnvOn,
                    56 => Self::EnvOff,
                    57 => Self::EnvPowerOn,
                    58 => Self::EnvPowerOff,
                    59 => Self::EnvPowerMonitor,
                    60 => Self::EnvTempOn,
                    61 => Self::EnvTempOff,
                    62 => Self::EnvAltitudeHigh,
                    63 => Self::EnvAltitudeLow,
                    64 => Self::EnvReset,
                    65 => Self::EnvTestWarning,
                    66 => Self::EnvTestFault,
                    67 => Self::EnvTestTmpadv,
                    68 => Self::EnvTestTmpcrit,
                    69 => Self::EnvTestNumberNumberNumberNumberNumber,
                    70 => Self::EnvTestPwrdn,
                    71 => Self::EnvMessages,
                    72 => Self::EnvMessagesFaultsonly,
                    73 => Self::EnvMessagesAll,
                    74 => Self::Fan,
                    75 => Self::FanOnNumber,
                    76 => Self::FanOn,
                    77 => Self::FanSpeed,
                    78 => Self::FanSpeedNumberNumber,
                    79 => Self::FanSpeedNumber,
                    80 => Self::FanSpeedLowNumber,
                    81 => Self::FanSpeedLow,
                    82 => Self::FanAutoOn,
                    83 => Self::Help,
                    84 => Self::HelpWord,
                    85 => Self::HelpListHidden,
                    86 => Self::HelpWordHidden,
                    87 => Self::History,
                    88 => Self::L1dbg,
                    89 => Self::L1dbgSysLinkNumber,
                    90 => Self::L1dbgSysLinkNumberNumber,
                    91 => Self::L1dbgIrtrNumber,
                    92 => Self::L1dbgEnvNumber,
                    93 => Self::L1dbgEnvOff,
                    94 => Self::L1dbgPortOn,
                    95 => Self::L1dbgI2cOn,
                    96 => Self::L1dbgMarginOn,
                    97 => Self::L1dbgConsoleOn,
                    98 => Self::L1dbgMempanicOn,
                    99 => Self::L1dbgPppdumpOn,
                    100 => Self::L1dbgPppdelayNumber,
                    101 => Self::L1dbgPromreqOn,
                    102 => Self::L1dbgQsuspNumberNumber,
                    103 => Self::L1dbgPrintfOn,
                    104 => Self::L1dbgFlashignoreNumberNumber,
                    105 => Self::L1dbgMiscNumber,
                    106 => Self::L1dbgGalNumber,
                    107 => Self::Link,
                    108 => Self::LinkVerbose,
                    109 => Self::LinkCheck,
                    110 => Self::LinkCheckV,
                    111 => Self::LinkStable,
                    112 => Self::Log,
                    113 => Self::LogClear,
                    114 => Self::LogInsertCommand,
                    115 => Self::Ioport,
                    116 => Self::Ioport400,
                    117 => Self::IoportA400,
                    118 => Self::IoportB400,
                    119 => Self::IoportForce400,
                    120 => Self::IoportForceAlt1,
                    121 => Self::IoportForceAlt2,
                    122 => Self::IoportClksrcA,
                    123 => Self::IoportClksrcB,
                    124 => Self::IstatMemory,
                    125 => Self::IstatQueues,
                    126 => Self::IstatQueuesReset,
                    127 => Self::IstatTasks,
                    128 => Self::IstatPmalloc,
                    129 => Self::IstatDumpWord,
                    130 => Self::B2b,
                    131 => Self::L1,
                    132 => Self::Leds,
                    133 => Self::Margin,
                    134 => Self::MarginDefault,
                    135 => Self::MarginNumberDefault,
                    136 => Self::MarginNumberNumber,
                    137 => Self::MarginDimmVWord,
                    138 => Self::Network,
                    139 => Self::NetworkUsb,
                    140 => Self::NetworkAutodetectOn,
                    141 => Self::Pimm,
                    142 => Self::PimmAuto,
                    143 => Self::PimmClksrcExt,
                    144 => Self::PimmClksrcInt,
                    145 => Self::PimmClkmodeAsync,
                    146 => Self::PimmClkmodeSync,
                    147 => Self::Port,
                    148 => Self::PortV,
                    149 => Self::Power,
                    150 => Self::PowerCheck,
                    151 => Self::PowerVrm,
                    152 => Self::PowerUp,
                    153 => Self::PowerUpHold,
                    154 => Self::PowerDown,
                    155 => Self::Reset,
                    156 => Self::ResetHold,
                    157 => Self::Nmi,
                    158 => Self::Softreset,
                    159 => Self::SoftresetNumber,
                    160 => Self::Select,
                    161 => Self::SelectNumberNumberNumberNumber,
                    162 => Self::SelectSubchannelConsole,
                    163 => Self::SelectSubchannelA,
                    164 => Self::SelectSubchannelNumber,
                    165 => Self::SelectSubchannelWord,
                    166 => Self::SelectEnable,
                    167 => Self::SelectDisable,
                    168 => Self::SelectFilterOn,
                    169 => Self::SelectFilterOff,
                    170 => Self::Serial,
                    171 => Self::SerialVerify,
                    172 => Self::SerialAll,
                    173 => Self::SerialAllV,
                    174 => Self::SerialB,
                    175 => Self::SerialBV,
                    176 => Self::SerialCpu,
                    177 => Self::SerialCpuV,
                    178 => Self::SerialDimm,
                    179 => Self::SerialDimmV,
                    180 => Self::SerialDimmCfg,
                    181 => Self::SerialDimmStatus,
                    182 => Self::SerialClear,
                    183 => Self::SerialSetWord,
                    184 => Self::SerialWordWordWordWord,
                    185 => Self::SerialSecurityOffWordWordWordWord,
                    186 => Self::SerialSecurityOn,
                    187 => Self::SerialBrickDisable,
                    188 => Self::SerialBrickDisableConfirm,
                    189 => Self::Sysstate,
                    190 => Self::Eeprom,
                    191 => Self::EepromNumberNumberNumber,
                    192 => Self::EepromFillNumberNumberNumberNumber,
                    193 => Self::Uart,
                    194 => Self::UartSysLinkReset,
                    195 => Self::Usb,
                    196 => Self::Router,
                    197 => Self::RouterMeta,
                    198 => Self::RouterMeta1,
                    199 => Self::RouterRepeater,
                    200 => Self::RouterOrdinary,
                    201 => Self::Router6Port,
                    202 => Self::Router6PortConfirm,
                    203 => Self::Router8Port,
                    204 => Self::Router8PortWordWordWordWord,
                    205 => Self::RouterServiceWordWordWordWord,
                    206 => Self::RouterServiceOff,
                    207 => Self::RouterSpare,
                    208 => Self::ServiceWordWordWordWord,
                    209 => Self::ServiceOff,
                    210 => Self::DateTz,
                    211 => Self::DateTzWord,
                    212 => Self::Date,
                    213 => Self::DateRaw,
                    214 => Self::DateWord,
                    215 => Self::DateWordWord,
                    216 => Self::NvramReset,
                    217 => Self::NvramReadNumberNumber,
                    218 => Self::NvramWriteNumberNumber,
                    219 => Self::NvramScratch,
                    220 => Self::NvramScratchNumber,
                    221 => Self::L2Command,
                    222 => Self::SecurityVerify3400,
                    223 => Self::SecurityVerify3800,
                    224 => Self::SleepNumberNumberCommand,
                    225 => Self::Mtb,
                    226 => Self::FlashDefaultReset,
                    227 => Self::FlashDefaultA,
                    228 => Self::FlashDefaultB,
                    229 => Self::FlashDefaultCurrent,
                    230 => Self::FlashDefaultOld,
                    231 => Self::FlashDefaultNew,
                    232 => Self::FlashStatus,
                    233 => Self::FlashStatusV,
                    234 => Self::RebootL1,
                    235 => Self::RebootL1A,
                    236 => Self::RebootL1B,
                    237 => Self::RebootL1Current,
                    238 => Self::RebootL1Old,
                    239 => Self::RebootL1New,
                    240 => Self::RebootL1Other,
                    241 => Self::RebootL1AddrNumber,
                    242 => Self::RebootL1RawNumber,
                    243 => Self::Version,
                    244 => Self::Pbay,
                    245 => Self::PbayVersion,
                    246 => Self::PbayDps,
                    247 => Self::PbayDpsWord,
                    248 => Self::PbayDcport,
                    249 => Self::PbayDcportWord,
                    250 => Self::PbayFru,
                    251 => Self::PbayFruWord,
                    252 => Self::PbayReset,
                    253 => Self::PbayEnv,
                    254 => Self::PbayEnvOn,
                    255 => Self::PbayEnvOff,
                    256 => Self::PbayInit,
                    257 => Self::PbaySerial,
                    258 => Self::TestI2c,
                    259 => Self::TestI2cNumber,
                    260 => Self::TestIoexpSetNumberNumber,
                    261 => Self::TestIoexpGetNumber,
                    262 => Self::TestIoexpGetAll,
                    263 => Self::TestIntr,
                    264 => Self::TestDisplay,
                    265 => Self::TestExceptionMem,
                    266 => Self::TestExceptionStack,
                    267 => Self::TestExceptionWdog,
                    268 => Self::TestExceptionFatal,
                    269 => Self::ScanReset,
                    270 => Self::ScanResetHard,
                    271 => Self::ScanResetSoft,
                    272 => Self::ScanResetBoth,
                    273 => Self::ScanSelNumberNumber,
                    274 => Self::ScanSelNumberNumberNumber,
                    275 => Self::ScanSelNumberNumberNumberNumber,
                    276 => Self::ScanIdsNumber,
                    277 => Self::ScanSetTrst0,
                    278 => Self::ScanSetTrst1,
                    279 => Self::ScanSetTck0,
                    280 => Self::ScanSetTck1,
                    281 => Self::ScanSetTms0,
                    282 => Self::ScanSetTms1,
                    283 => Self::ScanSetTdi0,
                    284 => Self::ScanSetTdi1,
                    285 => Self::ScanSetPsi0,
                    286 => Self::ScanSetPsi1,
                    287 => Self::ScanGet,
                    288 => Self::ScanDebugNumber,
                    289 => Self::ScanDebug,
                    290 => Self::ScanCfg,
                    291 => Self::ScanImpSetDllbypNumber,
                    292 => Self::ScanImpSetNumber,
                    293 => Self::ScanImpUnlock,
                    294 => Self::ScanTestNumber,
                    295 => Self::Fru,
                    296 => Self::FruCommand,
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
                    Self::BrickRackDecimal => 14,
                    Self::BrickSlotDecimal => 15,
                    Self::BrickRackslotDecimalDecimal => 16,
                    Self::BrickRackslotResetNumberNumber => 17,
                    Self::BrickPartitionNone => 18,
                    Self::BrickPartitionNumber => 19,
                    Self::Partdb => 20,
                    Self::PartdbEnable => 21,
                    Self::PartdbDisable => 22,
                    Self::Cpu => 23,
                    Self::CpuE => 24,
                    Self::CpuWordE => 25,
                    Self::B2bWordCommand => 26,
                    Self::Config => 27,
                    Self::ConfigV => 28,
                    Self::ConfigReset => 29,
                    Self::ConfigRing => 30,
                    Self::Debug => 31,
                    Self::DebugNumber => 32,
                    Self::Display => 33,
                    Self::DisplayNumberCommand => 34,
                    Self::DisplayPowerOn => 35,
                    Self::DisplayAttentionOn => 36,
                    Self::DisplayFailureOn => 37,
                    Self::DisplayPowerOff => 38,
                    Self::DisplayAttentionOff => 39,
                    Self::DisplayFailureOff => 40,
                    Self::Button => 41,
                    Self::ButtonAllEnable => 42,
                    Self::ButtonAllDisable => 43,
                    Self::ButtonPwrEnable => 44,
                    Self::ButtonPwrDisable => 45,
                    Self::ButtonRstEnable => 46,
                    Self::ButtonRstDisable => 47,
                    Self::ButtonNmiEnable => 48,
                    Self::ButtonNmiDisable => 49,
                    Self::LetTheCarnageBegin => 50,
                    Self::SlippedMyMind => 51,
                    Self::SuperSizedPlease => 52,
                    Self::Env => 53,
                    Self::EnvCheck => 54,
                    Self::EnvOn => 55,
                    Self::EnvOff => 56,
                    Self::EnvPowerOn => 57,
                    Self::EnvPowerOff => 58,
                    Self::EnvPowerMonitor => 59,
                    Self::EnvTempOn => 60,
                    Self::EnvTempOff => 61,
                    Self::EnvAltitudeHigh => 62,
                    Self::EnvAltitudeLow => 63,
                    Self::EnvReset => 64,
                    Self::EnvTestWarning => 65,
                    Self::EnvTestFault => 66,
                    Self::EnvTestTmpadv => 67,
                    Self::EnvTestTmpcrit => 68,
                    Self::EnvTestNumberNumberNumberNumberNumber => 69,
                    Self::EnvTestPwrdn => 70,
                    Self::EnvMessages => 71,
                    Self::EnvMessagesFaultsonly => 72,
                    Self::EnvMessagesAll => 73,
                    Self::Fan => 74,
                    Self::FanOnNumber => 75,
                    Self::FanOn => 76,
                    Self::FanSpeed => 77,
                    Self::FanSpeedNumberNumber => 78,
                    Self::FanSpeedNumber => 79,
                    Self::FanSpeedLowNumber => 80,
                    Self::FanSpeedLow => 81,
                    Self::FanAutoOn => 82,
                    Self::Help => 83,
                    Self::HelpWord => 84,
                    Self::HelpListHidden => 85,
                    Self::HelpWordHidden => 86,
                    Self::History => 87,
                    Self::L1dbg => 88,
                    Self::L1dbgSysLinkNumber => 89,
                    Self::L1dbgSysLinkNumberNumber => 90,
                    Self::L1dbgIrtrNumber => 91,
                    Self::L1dbgEnvNumber => 92,
                    Self::L1dbgEnvOff => 93,
                    Self::L1dbgPortOn => 94,
                    Self::L1dbgI2cOn => 95,
                    Self::L1dbgMarginOn => 96,
                    Self::L1dbgConsoleOn => 97,
                    Self::L1dbgMempanicOn => 98,
                    Self::L1dbgPppdumpOn => 99,
                    Self::L1dbgPppdelayNumber => 100,
                    Self::L1dbgPromreqOn => 101,
                    Self::L1dbgQsuspNumberNumber => 102,
                    Self::L1dbgPrintfOn => 103,
                    Self::L1dbgFlashignoreNumberNumber => 104,
                    Self::L1dbgMiscNumber => 105,
                    Self::L1dbgGalNumber => 106,
                    Self::Link => 107,
                    Self::LinkVerbose => 108,
                    Self::LinkCheck => 109,
                    Self::LinkCheckV => 110,
                    Self::LinkStable => 111,
                    Self::Log => 112,
                    Self::LogClear => 113,
                    Self::LogInsertCommand => 114,
                    Self::Ioport => 115,
                    Self::Ioport400 => 116,
                    Self::IoportA400 => 117,
                    Self::IoportB400 => 118,
                    Self::IoportForce400 => 119,
                    Self::IoportForceAlt1 => 120,
                    Self::IoportForceAlt2 => 121,
                    Self::IoportClksrcA => 122,
                    Self::IoportClksrcB => 123,
                    Self::IstatMemory => 124,
                    Self::IstatQueues => 125,
                    Self::IstatQueuesReset => 126,
                    Self::IstatTasks => 127,
                    Self::IstatPmalloc => 128,
                    Self::IstatDumpWord => 129,
                    Self::B2b => 130,
                    Self::L1 => 131,
                    Self::Leds => 132,
                    Self::Margin => 133,
                    Self::MarginDefault => 134,
                    Self::MarginNumberDefault => 135,
                    Self::MarginNumberNumber => 136,
                    Self::MarginDimmVWord => 137,
                    Self::Network => 138,
                    Self::NetworkUsb => 139,
                    Self::NetworkAutodetectOn => 140,
                    Self::Pimm => 141,
                    Self::PimmAuto => 142,
                    Self::PimmClksrcExt => 143,
                    Self::PimmClksrcInt => 144,
                    Self::PimmClkmodeAsync => 145,
                    Self::PimmClkmodeSync => 146,
                    Self::Port => 147,
                    Self::PortV => 148,
                    Self::Power => 149,
                    Self::PowerCheck => 150,
                    Self::PowerVrm => 151,
                    Self::PowerUp => 152,
                    Self::PowerUpHold => 153,
                    Self::PowerDown => 154,
                    Self::Reset => 155,
                    Self::ResetHold => 156,
                    Self::Nmi => 157,
                    Self::Softreset => 158,
                    Self::SoftresetNumber => 159,
                    Self::Select => 160,
                    Self::SelectNumberNumberNumberNumber => 161,
                    Self::SelectSubchannelConsole => 162,
                    Self::SelectSubchannelA => 163,
                    Self::SelectSubchannelNumber => 164,
                    Self::SelectSubchannelWord => 165,
                    Self::SelectEnable => 166,
                    Self::SelectDisable => 167,
                    Self::SelectFilterOn => 168,
                    Self::SelectFilterOff => 169,
                    Self::Serial => 170,
                    Self::SerialVerify => 171,
                    Self::SerialAll => 172,
                    Self::SerialAllV => 173,
                    Self::SerialB => 174,
                    Self::SerialBV => 175,
                    Self::SerialCpu => 176,
                    Self::SerialCpuV => 177,
                    Self::SerialDimm => 178,
                    Self::SerialDimmV => 179,
                    Self::SerialDimmCfg => 180,
                    Self::SerialDimmStatus => 181,
                    Self::SerialClear => 182,
                    Self::SerialSetWord => 183,
                    Self::SerialWordWordWordWord => 184,
                    Self::SerialSecurityOffWordWordWordWord => 185,
                    Self::SerialSecurityOn => 186,
                    Self::SerialBrickDisable => 187,
                    Self::SerialBrickDisableConfirm => 188,
                    Self::Sysstate => 189,
                    Self::Eeprom => 190,
                    Self::EepromNumberNumberNumber => 191,
                    Self::EepromFillNumberNumberNumberNumber => 192,
                    Self::Uart => 193,
                    Self::UartSysLinkReset => 194,
                    Self::Usb => 195,
                    Self::Router => 196,
                    Self::RouterMeta => 197,
                    Self::RouterMeta1 => 198,
                    Self::RouterRepeater => 199,
                    Self::RouterOrdinary => 200,
                    Self::Router6Port => 201,
                    Self::Router6PortConfirm => 202,
                    Self::Router8Port => 203,
                    Self::Router8PortWordWordWordWord => 204,
                    Self::RouterServiceWordWordWordWord => 205,
                    Self::RouterServiceOff => 206,
                    Self::RouterSpare => 207,
                    Self::ServiceWordWordWordWord => 208,
                    Self::ServiceOff => 209,
                    Self::DateTz => 210,
                    Self::DateTzWord => 211,
                    Self::Date => 212,
                    Self::DateRaw => 213,
                    Self::DateWord => 214,
                    Self::DateWordWord => 215,
                    Self::NvramReset => 216,
                    Self::NvramReadNumberNumber => 217,
                    Self::NvramWriteNumberNumber => 218,
                    Self::NvramScratch => 219,
                    Self::NvramScratchNumber => 220,
                    Self::L2Command => 221,
                    Self::SecurityVerify3400 => 222,
                    Self::SecurityVerify3800 => 223,
                    Self::SleepNumberNumberCommand => 224,
                    Self::Mtb => 225,
                    Self::FlashDefaultReset => 226,
                    Self::FlashDefaultA => 227,
                    Self::FlashDefaultB => 228,
                    Self::FlashDefaultCurrent => 229,
                    Self::FlashDefaultOld => 230,
                    Self::FlashDefaultNew => 231,
                    Self::FlashStatus => 232,
                    Self::FlashStatusV => 233,
                    Self::RebootL1 => 234,
                    Self::RebootL1A => 235,
                    Self::RebootL1B => 236,
                    Self::RebootL1Current => 237,
                    Self::RebootL1Old => 238,
                    Self::RebootL1New => 239,
                    Self::RebootL1Other => 240,
                    Self::RebootL1AddrNumber => 241,
                    Self::RebootL1RawNumber => 242,
                    Self::Version => 243,
                    Self::Pbay => 244,
                    Self::PbayVersion => 245,
                    Self::PbayDps => 246,
                    Self::PbayDpsWord => 247,
                    Self::PbayDcport => 248,
                    Self::PbayDcportWord => 249,
                    Self::PbayFru => 250,
                    Self::PbayFruWord => 251,
                    Self::PbayReset => 252,
                    Self::PbayEnv => 253,
                    Self::PbayEnvOn => 254,
                    Self::PbayEnvOff => 255,
                    Self::PbayInit => 256,
                    Self::PbaySerial => 257,
                    Self::TestI2c => 258,
                    Self::TestI2cNumber => 259,
                    Self::TestIoexpSetNumberNumber => 260,
                    Self::TestIoexpGetNumber => 261,
                    Self::TestIoexpGetAll => 262,
                    Self::TestIntr => 263,
                    Self::TestDisplay => 264,
                    Self::TestExceptionMem => 265,
                    Self::TestExceptionStack => 266,
                    Self::TestExceptionWdog => 267,
                    Self::TestExceptionFatal => 268,
                    Self::ScanReset => 269,
                    Self::ScanResetHard => 270,
                    Self::ScanResetSoft => 271,
                    Self::ScanResetBoth => 272,
                    Self::ScanSelNumberNumber => 273,
                    Self::ScanSelNumberNumberNumber => 274,
                    Self::ScanSelNumberNumberNumberNumber => 275,
                    Self::ScanIdsNumber => 276,
                    Self::ScanSetTrst0 => 277,
                    Self::ScanSetTrst1 => 278,
                    Self::ScanSetTck0 => 279,
                    Self::ScanSetTck1 => 280,
                    Self::ScanSetTms0 => 281,
                    Self::ScanSetTms1 => 282,
                    Self::ScanSetTdi0 => 283,
                    Self::ScanSetTdi1 => 284,
                    Self::ScanSetPsi0 => 285,
                    Self::ScanSetPsi1 => 286,
                    Self::ScanGet => 287,
                    Self::ScanDebugNumber => 288,
                    Self::ScanDebug => 289,
                    Self::ScanCfg => 290,
                    Self::ScanImpSetDllbypNumber => 291,
                    Self::ScanImpSetNumber => 292,
                    Self::ScanImpUnlock => 293,
                    Self::ScanTestNumber => 294,
                    Self::Fru => 295,
                    Self::FruCommand => 296,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// (established)
pub mod cmd1 {
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
        pub const CHECK_POWER_ON: u32 = 0; // established
        pub const CHECK_MARGIN_HIGH: u32 = 1; // established
        pub const CHECK_SCRAMBLE_ENABLED: u32 = 2; // established
        pub const CHECK_LINKS: u32 = 3; // established
        pub const CHECK_DEBUG_NUMBER: u32 = 4; // established
        pub const CHECK_SECURITY_ON: u32 = 5; // established
        pub const CHECK_SERIAL_WORD: u32 = 6; // established
        pub const FRU_CAPTURE: u32 = 7; // established
        pub const FRU_CAPTURE_VERBOSE: u32 = 8; // established
        pub const FRU_DUMP: u32 = 9; // established
        pub const FRU_RESET: u32 = 10; // established
        pub const FRU_STATUS: u32 = 11; // established
        pub const FRU_STATUS_VERBOSE: u32 = 12; // established
        pub const DEADLOCK: u32 = 13; // established
        pub const DEADLOCK_VERBOSE: u32 = 14; // established
        pub const MEMCFG: u32 = 15; // established
        pub const MEMCFG_1BANK166_E: u32 = 16; // established
        pub const SHCFG: u32 = 17; // established
        pub const SHCFG_FSB_RESET: u32 = 18; // established
        pub const SHCFG_FSB: u32 = 19; // established
        pub const SHCFG_FSB_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 20; // established
        pub const SHCFG_WRT90_RESET: u32 = 21; // established
        pub const SHCFG_WRT90: u32 = 22; // established
        pub const SHCFG_WRT90_NUMBER_NUMBER_NUMBER: u32 = 23; // established
        pub const SHCFG_MCI_RESET: u32 = 24; // established
        pub const SHCFG_MCI: u32 = 25; // established
        pub const SHCFG_MCI_NUMBER_NUMBER_NUMBER: u32 = 26; // established
        pub const SHCFG_MDCLK: u32 = 27; // established
        pub const SHCFG_MDCLK_NUMBER: u32 = 28; // established
        pub const SHCFG_NICLK: u32 = 29; // established
        pub const SHCFG_NICLK_NUMBER: u32 = 30; // established
        pub const SHCFG_OSC: u32 = 31; // established
        pub const SHCFG_OSC_NUMBER: u32 = 32; // established
        pub const SHCFG_GTL_RE: u32 = 33; // established
        pub const SHCFG_GTL_RE_NUMBER: u32 = 34; // established
        pub const SHCFG_JCLKSTOP: u32 = 35; // established
        pub const SHCFG_JCLKSTOP_NUMBER: u32 = 36; // established
        pub const SHCFG_TP: u32 = 37; // established
        pub const SHCFG_TP_NUMBER: u32 = 38; // established
        pub const NICFG_NI0: u32 = 39; // established
        pub const NICFG_NUMBER_NI0: u32 = 40; // established
        pub const NICFG_NI0_NUMBER: u32 = 41; // established
        pub const NICFG_NI0_NUMBER_NUMBER: u32 = 42; // established
        pub const NICFG_NI0_NUMBER_NUMBER_NUMBER: u32 = 43; // established
        pub const NICFG_NI0_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 44; // established
        pub const NICFG_NI0_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 45; // established
        pub const NICFG_NUMBER_NI0_NUMBER: u32 = 46; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER: u32 = 47; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER: u32 = 48; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 49; // established
        pub const NICFG_NUMBER_NI0_NUMBER_NUMBER_NUMBER_NUMBER_NUMBER: u32 = 50; // established
        pub const RMMR_WORD_NUMBER: u32 = 51; // established
        pub const WMMR_WORD_NUMBER_NUMBER_NUMBER: u32 = 52; // established
        pub const PKTGEN_READ_WORD_NUMBER: u32 = 53; // established
        pub const PKTGEN_WRITE_WORD_NUMBER_NUMBER_NUMBER: u32 = 54; // established
        pub const PKTGEN_DEBUG_NUMBER: u32 = 55; // established
        pub const TUNE_GET: u32 = 56; // established
        pub const PROMICE: u32 = 57; // established
        pub const PROMICE_E: u32 = 58; // established
        pub const PROMICE_D: u32 = 59; // established
        pub const PROMICE_NUMBER_E: u32 = 60; // established
        pub const PROMICE_NUMBER_D: u32 = 61; // established
        pub const PROMVER: u32 = 62; // established
        pub const NODE: u32 = 63; // established
        pub const NODE_E: u32 = 64; // established
        pub const NODE_D: u32 = 65; // established
        pub const NODE_NUMBER_E: u32 = 66; // established
        pub const NODE_NUMBER_D: u32 = 67; // established
        pub const PIC_MODE: u32 = 68; // established
        pub const WAR: u32 = 69; // established
        pub const WAR_E: u32 = 70; // established
        pub const WAR_D: u32 = 71; // established
        pub const SHDBG: u32 = 72; // established
        pub const SHDBG_RETRYLIMIT_NUMBER: u32 = 73; // established
        pub const SHDBG_RDELAY_NUMBER: u32 = 74; // established
        pub const SHDBG_FULLRESET_NUMBER: u32 = 75; // established
        pub const SHDBG_LINKDOWN_NUMBER: u32 = 76; // established
        pub const MBRICK: u32 = 77; // established
        pub const MBRICK_MEMORY: u32 = 78; // established
        pub const MBRICK_MEMORY_CONFIRM: u32 = 79; // established
        pub const MBRICK_CPU_WORD_WORD_WORD_WORD: u32 = 80; // established
        pub const MBRICK_CPU: u32 = 81; // established
        pub const IF_I_ONLY_HAD_A_BRAIN: u32 = 82; // established
        pub const SSI: u32 = 83; // established
        pub const SSI_SERVICE_WORD_WORD_WORD_WORD: u32 = 84; // established
        pub const SSI_SERVICE_OFF: u32 = 85; // established
        pub const SSI_WORD_WORD_WORD_WORD_WORD: u32 = 86; // established
        pub const MAD_WITH_POWER_NUMBER: u32 = 87; // established
        pub const PWM_READ: u32 = 88; // established
        pub const PWM_SET_NUMBER_NUMBER: u32 = 89; // established
        pub const SYS_LINK_SIM_ON: u32 = 90; // established
        pub const SYS_LINK_SIM_ON_NUMBER_NUMBER: u32 = 91; // established
        pub const SYS_LINK_SIM_OFF: u32 = 92; // established
        pub const SYS_LINK_DUMP_INPUT_NUMBER: u32 = 93; // established
        pub const SYS_LINK_DUMP_OUTPUT_NUMBER: u32 = 94; // established
        pub const SYS_LINK_CAPTURE_RESET: u32 = 95; // established
        pub const SYS_LINK_LOG_NUMBER: u32 = 96; // established
        pub const SYS_LINK_LOG_ALL: u32 = 97; // established
        pub const SYS_LINK_LOG_RESET: u32 = 98; // established
        pub const SYS_LINK_LOG_NUMBER_INSERT_COMMAND: u32 = 99; // established
        pub const SYS_LINK_CONFIG: u32 = 100; // established
        pub const SYS_LINK_QSUSPEND: u32 = 101; // established
        pub const SYS_LINK_QSUSPEND_NUMBER: u32 = 102; // established
        pub const ERROR_NUMBER: u32 = 103; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            CheckPowerOn,
            CheckMarginHigh,
            CheckScrambleEnabled,
            CheckLinks,
            CheckDebugNumber,
            CheckSecurityOn,
            CheckSerialWord,
            FruCapture,
            FruCaptureVerbose,
            FruDump,
            FruReset,
            FruStatus,
            FruStatusVerbose,
            Deadlock,
            DeadlockVerbose,
            Memcfg,
            Memcfg1bank166E,
            Shcfg,
            ShcfgFsbReset,
            ShcfgFsb,
            ShcfgFsbNumberNumberNumberNumberNumberNumberNumber,
            ShcfgWrt90Reset,
            ShcfgWrt90,
            ShcfgWrt90NumberNumberNumber,
            ShcfgMciReset,
            ShcfgMci,
            ShcfgMciNumberNumberNumber,
            ShcfgMdclk,
            ShcfgMdclkNumber,
            ShcfgNiclk,
            ShcfgNiclkNumber,
            ShcfgOsc,
            ShcfgOscNumber,
            ShcfgGtlRe,
            ShcfgGtlReNumber,
            ShcfgJclkstop,
            ShcfgJclkstopNumber,
            ShcfgTp,
            ShcfgTpNumber,
            NicfgNi0,
            NicfgNumberNi0,
            NicfgNi0Number,
            NicfgNi0NumberNumber,
            NicfgNi0NumberNumberNumber,
            NicfgNi0NumberNumberNumberNumber,
            NicfgNi0NumberNumberNumberNumberNumber,
            NicfgNumberNi0Number,
            NicfgNumberNi0NumberNumber,
            NicfgNumberNi0NumberNumberNumber,
            NicfgNumberNi0NumberNumberNumberNumber,
            NicfgNumberNi0NumberNumberNumberNumberNumber,
            RmmrWordNumber,
            WmmrWordNumberNumberNumber,
            PktgenReadWordNumber,
            PktgenWriteWordNumberNumberNumber,
            PktgenDebugNumber,
            TuneGet,
            Promice,
            PromiceE,
            PromiceD,
            PromiceNumberE,
            PromiceNumberD,
            Promver,
            Node,
            NodeE,
            NodeD,
            NodeNumberE,
            NodeNumberD,
            PicMode,
            War,
            WarE,
            WarD,
            Shdbg,
            ShdbgRetrylimitNumber,
            ShdbgRdelayNumber,
            ShdbgFullresetNumber,
            ShdbgLinkdownNumber,
            Mbrick,
            MbrickMemory,
            MbrickMemoryConfirm,
            MbrickCpuWordWordWordWord,
            MbrickCpu,
            IfIOnlyHadABrain,
            Ssi,
            SsiServiceWordWordWordWord,
            SsiServiceOff,
            SsiWordWordWordWordWord,
            MadWithPowerNumber,
            PwmRead,
            PwmSetNumberNumber,
            SysLinkSimOn,
            SysLinkSimOnNumberNumber,
            SysLinkSimOff,
            SysLinkDumpInputNumber,
            SysLinkDumpOutputNumber,
            SysLinkCaptureReset,
            SysLinkLogNumber,
            SysLinkLogAll,
            SysLinkLogReset,
            SysLinkLogNumberInsertCommand,
            SysLinkConfig,
            SysLinkQsuspend,
            SysLinkQsuspendNumber,
            ErrorNumber,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::CheckPowerOn,
                    1 => Self::CheckMarginHigh,
                    2 => Self::CheckScrambleEnabled,
                    3 => Self::CheckLinks,
                    4 => Self::CheckDebugNumber,
                    5 => Self::CheckSecurityOn,
                    6 => Self::CheckSerialWord,
                    7 => Self::FruCapture,
                    8 => Self::FruCaptureVerbose,
                    9 => Self::FruDump,
                    10 => Self::FruReset,
                    11 => Self::FruStatus,
                    12 => Self::FruStatusVerbose,
                    13 => Self::Deadlock,
                    14 => Self::DeadlockVerbose,
                    15 => Self::Memcfg,
                    16 => Self::Memcfg1bank166E,
                    17 => Self::Shcfg,
                    18 => Self::ShcfgFsbReset,
                    19 => Self::ShcfgFsb,
                    20 => Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber,
                    21 => Self::ShcfgWrt90Reset,
                    22 => Self::ShcfgWrt90,
                    23 => Self::ShcfgWrt90NumberNumberNumber,
                    24 => Self::ShcfgMciReset,
                    25 => Self::ShcfgMci,
                    26 => Self::ShcfgMciNumberNumberNumber,
                    27 => Self::ShcfgMdclk,
                    28 => Self::ShcfgMdclkNumber,
                    29 => Self::ShcfgNiclk,
                    30 => Self::ShcfgNiclkNumber,
                    31 => Self::ShcfgOsc,
                    32 => Self::ShcfgOscNumber,
                    33 => Self::ShcfgGtlRe,
                    34 => Self::ShcfgGtlReNumber,
                    35 => Self::ShcfgJclkstop,
                    36 => Self::ShcfgJclkstopNumber,
                    37 => Self::ShcfgTp,
                    38 => Self::ShcfgTpNumber,
                    39 => Self::NicfgNi0,
                    40 => Self::NicfgNumberNi0,
                    41 => Self::NicfgNi0Number,
                    42 => Self::NicfgNi0NumberNumber,
                    43 => Self::NicfgNi0NumberNumberNumber,
                    44 => Self::NicfgNi0NumberNumberNumberNumber,
                    45 => Self::NicfgNi0NumberNumberNumberNumberNumber,
                    46 => Self::NicfgNumberNi0Number,
                    47 => Self::NicfgNumberNi0NumberNumber,
                    48 => Self::NicfgNumberNi0NumberNumberNumber,
                    49 => Self::NicfgNumberNi0NumberNumberNumberNumber,
                    50 => Self::NicfgNumberNi0NumberNumberNumberNumberNumber,
                    51 => Self::RmmrWordNumber,
                    52 => Self::WmmrWordNumberNumberNumber,
                    53 => Self::PktgenReadWordNumber,
                    54 => Self::PktgenWriteWordNumberNumberNumber,
                    55 => Self::PktgenDebugNumber,
                    56 => Self::TuneGet,
                    57 => Self::Promice,
                    58 => Self::PromiceE,
                    59 => Self::PromiceD,
                    60 => Self::PromiceNumberE,
                    61 => Self::PromiceNumberD,
                    62 => Self::Promver,
                    63 => Self::Node,
                    64 => Self::NodeE,
                    65 => Self::NodeD,
                    66 => Self::NodeNumberE,
                    67 => Self::NodeNumberD,
                    68 => Self::PicMode,
                    69 => Self::War,
                    70 => Self::WarE,
                    71 => Self::WarD,
                    72 => Self::Shdbg,
                    73 => Self::ShdbgRetrylimitNumber,
                    74 => Self::ShdbgRdelayNumber,
                    75 => Self::ShdbgFullresetNumber,
                    76 => Self::ShdbgLinkdownNumber,
                    77 => Self::Mbrick,
                    78 => Self::MbrickMemory,
                    79 => Self::MbrickMemoryConfirm,
                    80 => Self::MbrickCpuWordWordWordWord,
                    81 => Self::MbrickCpu,
                    82 => Self::IfIOnlyHadABrain,
                    83 => Self::Ssi,
                    84 => Self::SsiServiceWordWordWordWord,
                    85 => Self::SsiServiceOff,
                    86 => Self::SsiWordWordWordWordWord,
                    87 => Self::MadWithPowerNumber,
                    88 => Self::PwmRead,
                    89 => Self::PwmSetNumberNumber,
                    90 => Self::SysLinkSimOn,
                    91 => Self::SysLinkSimOnNumberNumber,
                    92 => Self::SysLinkSimOff,
                    93 => Self::SysLinkDumpInputNumber,
                    94 => Self::SysLinkDumpOutputNumber,
                    95 => Self::SysLinkCaptureReset,
                    96 => Self::SysLinkLogNumber,
                    97 => Self::SysLinkLogAll,
                    98 => Self::SysLinkLogReset,
                    99 => Self::SysLinkLogNumberInsertCommand,
                    100 => Self::SysLinkConfig,
                    101 => Self::SysLinkQsuspend,
                    102 => Self::SysLinkQsuspendNumber,
                    103 => Self::ErrorNumber,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::CheckPowerOn => 0,
                    Self::CheckMarginHigh => 1,
                    Self::CheckScrambleEnabled => 2,
                    Self::CheckLinks => 3,
                    Self::CheckDebugNumber => 4,
                    Self::CheckSecurityOn => 5,
                    Self::CheckSerialWord => 6,
                    Self::FruCapture => 7,
                    Self::FruCaptureVerbose => 8,
                    Self::FruDump => 9,
                    Self::FruReset => 10,
                    Self::FruStatus => 11,
                    Self::FruStatusVerbose => 12,
                    Self::Deadlock => 13,
                    Self::DeadlockVerbose => 14,
                    Self::Memcfg => 15,
                    Self::Memcfg1bank166E => 16,
                    Self::Shcfg => 17,
                    Self::ShcfgFsbReset => 18,
                    Self::ShcfgFsb => 19,
                    Self::ShcfgFsbNumberNumberNumberNumberNumberNumberNumber => 20,
                    Self::ShcfgWrt90Reset => 21,
                    Self::ShcfgWrt90 => 22,
                    Self::ShcfgWrt90NumberNumberNumber => 23,
                    Self::ShcfgMciReset => 24,
                    Self::ShcfgMci => 25,
                    Self::ShcfgMciNumberNumberNumber => 26,
                    Self::ShcfgMdclk => 27,
                    Self::ShcfgMdclkNumber => 28,
                    Self::ShcfgNiclk => 29,
                    Self::ShcfgNiclkNumber => 30,
                    Self::ShcfgOsc => 31,
                    Self::ShcfgOscNumber => 32,
                    Self::ShcfgGtlRe => 33,
                    Self::ShcfgGtlReNumber => 34,
                    Self::ShcfgJclkstop => 35,
                    Self::ShcfgJclkstopNumber => 36,
                    Self::ShcfgTp => 37,
                    Self::ShcfgTpNumber => 38,
                    Self::NicfgNi0 => 39,
                    Self::NicfgNumberNi0 => 40,
                    Self::NicfgNi0Number => 41,
                    Self::NicfgNi0NumberNumber => 42,
                    Self::NicfgNi0NumberNumberNumber => 43,
                    Self::NicfgNi0NumberNumberNumberNumber => 44,
                    Self::NicfgNi0NumberNumberNumberNumberNumber => 45,
                    Self::NicfgNumberNi0Number => 46,
                    Self::NicfgNumberNi0NumberNumber => 47,
                    Self::NicfgNumberNi0NumberNumberNumber => 48,
                    Self::NicfgNumberNi0NumberNumberNumberNumber => 49,
                    Self::NicfgNumberNi0NumberNumberNumberNumberNumber => 50,
                    Self::RmmrWordNumber => 51,
                    Self::WmmrWordNumberNumberNumber => 52,
                    Self::PktgenReadWordNumber => 53,
                    Self::PktgenWriteWordNumberNumberNumber => 54,
                    Self::PktgenDebugNumber => 55,
                    Self::TuneGet => 56,
                    Self::Promice => 57,
                    Self::PromiceE => 58,
                    Self::PromiceD => 59,
                    Self::PromiceNumberE => 60,
                    Self::PromiceNumberD => 61,
                    Self::Promver => 62,
                    Self::Node => 63,
                    Self::NodeE => 64,
                    Self::NodeD => 65,
                    Self::NodeNumberE => 66,
                    Self::NodeNumberD => 67,
                    Self::PicMode => 68,
                    Self::War => 69,
                    Self::WarE => 70,
                    Self::WarD => 71,
                    Self::Shdbg => 72,
                    Self::ShdbgRetrylimitNumber => 73,
                    Self::ShdbgRdelayNumber => 74,
                    Self::ShdbgFullresetNumber => 75,
                    Self::ShdbgLinkdownNumber => 76,
                    Self::Mbrick => 77,
                    Self::MbrickMemory => 78,
                    Self::MbrickMemoryConfirm => 79,
                    Self::MbrickCpuWordWordWordWord => 80,
                    Self::MbrickCpu => 81,
                    Self::IfIOnlyHadABrain => 82,
                    Self::Ssi => 83,
                    Self::SsiServiceWordWordWordWord => 84,
                    Self::SsiServiceOff => 85,
                    Self::SsiWordWordWordWordWord => 86,
                    Self::MadWithPowerNumber => 87,
                    Self::PwmRead => 88,
                    Self::PwmSetNumberNumber => 89,
                    Self::SysLinkSimOn => 90,
                    Self::SysLinkSimOnNumberNumber => 91,
                    Self::SysLinkSimOff => 92,
                    Self::SysLinkDumpInputNumber => 93,
                    Self::SysLinkDumpOutputNumber => 94,
                    Self::SysLinkCaptureReset => 95,
                    Self::SysLinkLogNumber => 96,
                    Self::SysLinkLogAll => 97,
                    Self::SysLinkLogReset => 98,
                    Self::SysLinkLogNumberInsertCommand => 99,
                    Self::SysLinkConfig => 100,
                    Self::SysLinkQsuspend => 101,
                    Self::SysLinkQsuspendNumber => 102,
                    Self::ErrorNumber => 103,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
/// (established)
pub mod cmd2 {
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
        pub const PCI_RST_HOLD: u32 = 4; // established
        pub const PCI_RST_HOLD_NUMBER: u32 = 5; // established
        pub const PCI_RST_HOLD_NUMBER_NUMBER: u32 = 6; // established
        pub const PCI_RST_RELEASE: u32 = 7; // established
        pub const PCI_RST_RELEASE_NUMBER: u32 = 8; // established
        pub const PCI_RST_RELEASE_NUMBER_NUMBER: u32 = 9; // established
        pub const PCI_NUMBER_U: u32 = 10; // established
        pub const PCI_NUMBER_D: u32 = 11; // established
        pub const PCI_NUMBER_RST: u32 = 12; // established
        pub const PCI_NUMBER_NUMBER_U: u32 = 13; // established
        pub const PCI_NUMBER_NUMBER_D: u32 = 14; // established
        pub const PCI_NUMBER_NUMBER_RST: u32 = 15; // established
        pub const PCI_FORCE_ALL_ON: u32 = 16; // established
        pub const PCI_MODE_LIMIT_NUMBER_PCI33: u32 = 17; // established
        pub const PCI_MODE_LIMIT_PCI33: u32 = 18; // established
        pub const PCI_MODE_FORCE_NUMBER_PCI33: u32 = 19; // established
        pub const PCI_MODE_FORCE_PCI33: u32 = 20; // established
        pub const PCI_MODE_DEFAULT_NUMBER: u32 = 21; // established
        pub const PCI_MODE_DEFAULT: u32 = 22; // established
        pub const PCI_MODE_WARNING_DISABLE: u32 = 23; // established
        pub const PCI_MODE_WARNING_ENABLE: u32 = 24; // established
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Value {
            Pci,
            PciU,
            PciD,
            PciRst,
            PciRstHold,
            PciRstHoldNumber,
            PciRstHoldNumberNumber,
            PciRstRelease,
            PciRstReleaseNumber,
            PciRstReleaseNumberNumber,
            PciNumberU,
            PciNumberD,
            PciNumberRst,
            PciNumberNumberU,
            PciNumberNumberD,
            PciNumberNumberRst,
            PciForceAllOn,
            PciModeLimitNumberPci33,
            PciModeLimitPci33,
            PciModeForceNumberPci33,
            PciModeForcePci33,
            PciModeDefaultNumber,
            PciModeDefault,
            PciModeWarningDisable,
            PciModeWarningEnable,
            Other(u32),
        }
        impl Value {
            pub const fn from_raw(v: u32) -> Self {
                match v {
                    0 => Self::Pci,
                    1 => Self::PciU,
                    2 => Self::PciD,
                    3 => Self::PciRst,
                    4 => Self::PciRstHold,
                    5 => Self::PciRstHoldNumber,
                    6 => Self::PciRstHoldNumberNumber,
                    7 => Self::PciRstRelease,
                    8 => Self::PciRstReleaseNumber,
                    9 => Self::PciRstReleaseNumberNumber,
                    10 => Self::PciNumberU,
                    11 => Self::PciNumberD,
                    12 => Self::PciNumberRst,
                    13 => Self::PciNumberNumberU,
                    14 => Self::PciNumberNumberD,
                    15 => Self::PciNumberNumberRst,
                    16 => Self::PciForceAllOn,
                    17 => Self::PciModeLimitNumberPci33,
                    18 => Self::PciModeLimitPci33,
                    19 => Self::PciModeForceNumberPci33,
                    20 => Self::PciModeForcePci33,
                    21 => Self::PciModeDefaultNumber,
                    22 => Self::PciModeDefault,
                    23 => Self::PciModeWarningDisable,
                    24 => Self::PciModeWarningEnable,
                    _ => Self::Other(v),
                }
            }
            pub const fn raw(self) -> u32 {
                match self {
                    Self::Pci => 0,
                    Self::PciU => 1,
                    Self::PciD => 2,
                    Self::PciRst => 3,
                    Self::PciRstHold => 4,
                    Self::PciRstHoldNumber => 5,
                    Self::PciRstHoldNumberNumber => 6,
                    Self::PciRstRelease => 7,
                    Self::PciRstReleaseNumber => 8,
                    Self::PciRstReleaseNumberNumber => 9,
                    Self::PciNumberU => 10,
                    Self::PciNumberD => 11,
                    Self::PciNumberRst => 12,
                    Self::PciNumberNumberU => 13,
                    Self::PciNumberNumberD => 14,
                    Self::PciNumberNumberRst => 15,
                    Self::PciForceAllOn => 16,
                    Self::PciModeLimitNumberPci33 => 17,
                    Self::PciModeLimitPci33 => 18,
                    Self::PciModeForceNumberPci33 => 19,
                    Self::PciModeForcePci33 => 20,
                    Self::PciModeDefaultNumber => 21,
                    Self::PciModeDefault => 22,
                    Self::PciModeWarningDisable => 23,
                    Self::PciModeWarningEnable => 24,
                    Self::Other(v) => v,
                }
            }
        }
    }
}
