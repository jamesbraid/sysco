/*
 * MMSC_CMD_DECODE.h - typed command table of the multi-module system
 * controller firmware, and the user authority levels it checks
 *
 * The command table is text-keyed. An operator types a command word, and the
 * dispatcher compares it with each table entry in table order and runs the
 * first match. Each CMD.index value below is the position of that entry in the
 * table, counted from zero with aliases included. The dispatcher compares the
 * typed word, so the position is not a code sent on any link. Grades follow the
 * notes that list the table.
 *
 * The one-character help alias (table position 57) is not an identifier and has
 * no ENUM line here.
 *
 * Some commands compare the session's user level with 499 or 999 before they
 * act. The user levels are fixed values in the firmware (AUTH.level); which
 * command checks which level is in the command notes, not here.
 */
#ifndef MMSC_CMD_DECODE_H
#define MMSC_CMD_DECODE_H

/* FIELD: CMD.index 7:0 established */
#define MMSC_CMD_INDEX_SHIFT 0
#define MMSC_CMD_INDEX_MASK 0xffu

/* typed word AUTH */
/* ENUM: CMD.index 0 auth established */
#define MMSC_CMD_INDEX_AUTH 0

/* typed word AUTHORITY */
/* ENUM: CMD.index 1 authority established */
#define MMSC_CMD_INDEX_AUTHORITY 1

/* typed word BS */
/* ENUM: CMD.index 2 bs established */
#define MMSC_CMD_INDEX_BS 2

/* typed word CECHO */
/* ENUM: CMD.index 3 cecho established */
#define MMSC_CMD_INDEX_CECHO 3

/* typed word COM */
/* ENUM: CMD.index 4 com established */
#define MMSC_CMD_INDEX_COM 4

/* typed word CONS */
/* ENUM: CMD.index 5 cons established */
#define MMSC_CMD_INDEX_CONS 5

/* typed word CONSOLE */
/* ENUM: CMD.index 6 console established */
#define MMSC_CMD_INDEX_CONSOLE 6

/* typed word DEBUG */
/* ENUM: CMD.index 7 debug established */
#define MMSC_CMD_INDEX_DEBUG 7

/* typed word NUMRACKS */
/* ENUM: CMD.index 8 numracks established */
#define MMSC_CMD_INDEX_NUMRACKS 8

/* typed word DEST */
/* ENUM: CMD.index 9 dest established */
#define MMSC_CMD_INDEX_DEST 9

/* typed word DIRECT */
/* ENUM: CMD.index 10 direct established */
#define MMSC_CMD_INDEX_DIRECT 10

/* typed word DOWNLOADER */
/* ENUM: CMD.index 11 downloader established */
#define MMSC_CMD_INDEX_DOWNLOADER 11

/* typed word ELSC */
/* ENUM: CMD.index 12 elsc established */
#define MMSC_CMD_INDEX_ELSC 12

/* typed word EMSG */
/* ENUM: CMD.index 13 emsg established */
#define MMSC_CMD_INDEX_EMSG 13

/* typed word END */
/* ENUM: CMD.index 14 end established */
#define MMSC_CMD_INDEX_END 14

/* typed word ESC */
/* ENUM: CMD.index 15 esc established */
#define MMSC_CMD_INDEX_ESC 15

/* typed word EXIT */
/* ENUM: CMD.index 16 exit established */
#define MMSC_CMD_INDEX_EXIT 16

/* typed word FFSC */
/* ENUM: CMD.index 17 ffsc established */
#define MMSC_CMD_INDEX_FFSC 17

/* typed word FLASH */
/* ENUM: CMD.index 18 flash established */
#define MMSC_CMD_INDEX_FLASH 18

/* typed word GRAPHICS_COMMAND */
/* ENUM: CMD.index 19 graphics_command established */
#define MMSC_CMD_INDEX_GRAPHICS_COMMAND 19

/* typed word HELP */
/* ENUM: CMD.index 20 help established */
#define MMSC_CMD_INDEX_HELP 20

/* typed word IPADDR */
/* ENUM: CMD.index 21 ipaddr established */
#define MMSC_CMD_INDEX_IPADDR 21

/* typed word KILL */
/* ENUM: CMD.index 22 kill established */
#define MMSC_CMD_INDEX_KILL 22

/* typed word LOG */
/* ENUM: CMD.index 23 log established */
#define MMSC_CMD_INDEX_LOG 23

/* typed word MEM */
/* ENUM: CMD.index 24 mem established */
#define MMSC_CMD_INDEX_MEM 24

/* typed word MMSC */
/* ENUM: CMD.index 25 mmsc established */
#define MMSC_CMD_INDEX_MMSC 25

/* typed word MMSG */
/* ENUM: CMD.index 26 mmsg established */
#define MMSC_CMD_INDEX_MMSG 26

/* typed word MSC */
/* ENUM: CMD.index 27 msc established */
#define MMSC_CMD_INDEX_MSC 27

/* typed word NAP_TIME */
/* ENUM: CMD.index 28 nap_time established */
#define MMSC_CMD_INDEX_NAP_TIME 28

/* typed word NOP */
/* ENUM: CMD.index 29 nop established */
#define MMSC_CMD_INDEX_NOP 29

/* typed word OPTIONS */
/* ENUM: CMD.index 30 options established */
#define MMSC_CMD_INDEX_OPTIONS 30

/* typed word PAGER */
/* ENUM: CMD.index 31 pager established */
#define MMSC_CMD_INDEX_PAGER 31

/* typed word PAS */
/* ENUM: CMD.index 32 pas established */
#define MMSC_CMD_INDEX_PAS 32

/* typed word PASSWORD */
/* ENUM: CMD.index 33 password established */
#define MMSC_CMD_INDEX_PASSWORD 33

/* typed word PRINTENV */
/* ENUM: CMD.index 34 printenv established */
#define MMSC_CMD_INDEX_PRINTENV 34

/* typed word PWR */
/* ENUM: CMD.index 35 pwr established */
#define MMSC_CMD_INDEX_PWR 35

/* typed word RACKID */
/* ENUM: CMD.index 36 rackid established */
#define MMSC_CMD_INDEX_RACKID 36

/* typed word RAT */
/* ENUM: CMD.index 37 rat established */
#define MMSC_CMD_INDEX_RAT 37

/* typed word REBOOT_FFSC */
/* ENUM: CMD.index 38 reboot_ffsc established */
#define MMSC_CMD_INDEX_REBOOT_FFSC 38

/* typed word RESET_FFSC */
/* ENUM: CMD.index 39 reset_ffsc established */
#define MMSC_CMD_INDEX_RESET_FFSC 39

/* typed word RESET_MMSC */
/* ENUM: CMD.index 40 reset_mmsc established */
#define MMSC_CMD_INDEX_RESET_MMSC 40

/* typed word RESET_NVRAM */
/* ENUM: CMD.index 41 reset_nvram established */
#define MMSC_CMD_INDEX_RESET_NVRAM 41

/* typed word RMSG */
/* ENUM: CMD.index 42 rmsg established */
#define MMSC_CMD_INDEX_RMSG 42

/* typed word SCAN */
/* ENUM: CMD.index 43 scan established */
#define MMSC_CMD_INDEX_SCAN 43

/* typed word SETENV */
/* ENUM: CMD.index 44 setenv established */
#define MMSC_CMD_INDEX_SETENV 44

/* typed word STEAL */
/* ENUM: CMD.index 45 steal established */
#define MMSC_CMD_INDEX_STEAL 45

/* typed word PARTLST */
/* ENUM: CMD.index 46 partlst established */
#define MMSC_CMD_INDEX_PARTLST 46

/* typed word PARTSCAN */
/* ENUM: CMD.index 47 partscan established */
#define MMSC_CMD_INDEX_PARTSCAN 47

/* typed word PARTGET */
/* ENUM: CMD.index 48 partget established */
#define MMSC_CMD_INDEX_PARTGET 48

/* typed word PARTNAM */
/* ENUM: CMD.index 49 partnam established */
#define MMSC_CMD_INDEX_PARTNAM 49

/* typed word PARTCLR */
/* ENUM: CMD.index 50 partclr established */
#define MMSC_CMD_INDEX_PARTCLR 50

/* typed word PARTSET */
/* ENUM: CMD.index 51 partset established */
#define MMSC_CMD_INDEX_PARTSET 51

/* typed word SWAPCONS */
/* ENUM: CMD.index 52 swapcons established */
#define MMSC_CMD_INDEX_SWAPCONS 52

/* typed word TTY_REINIT */
/* ENUM: CMD.index 53 tty_reinit established */
#define MMSC_CMD_INDEX_TTY_REINIT 53

/* typed word UNSETENV */
/* ENUM: CMD.index 54 unsetenv established */
#define MMSC_CMD_INDEX_UNSETENV 54

/* typed word UNSTEAL */
/* ENUM: CMD.index 55 unsteal established */
#define MMSC_CMD_INDEX_UNSTEAL 55

/* typed word VER */
/* ENUM: CMD.index 56 ver established */
#define MMSC_CMD_INDEX_VER 56

/* typed word ARBITER */
/* ENUM: CMD.index 58 arbiter established */
#define MMSC_CMD_INDEX_ARBITER 58

/* typed word MACADDR */
/* ENUM: CMD.index 59 macaddr established */
#define MMSC_CMD_INDEX_MACADDR 59

/* FIELD: AUTH.level 15:0 established */
#define MMSC_AUTH_LEVEL_SHIFT 0
#define MMSC_AUTH_LEVEL_MASK 0xffffu

/* user level BASIC */
/* ENUM: AUTH.level 100 basic established */
#define MMSC_AUTH_LEVEL_BASIC 100

/* user level SUPERVISOR */
/* ENUM: AUTH.level 500 supervisor established */
#define MMSC_AUTH_LEVEL_SUPERVISOR 500

/* user level SERVICE */
/* ENUM: AUTH.level 1000 service established */
#define MMSC_AUTH_LEVEL_SERVICE 1000

/* FIELD: GATE.level 15:0 established */
#define MMSC_GATE_LEVEL_SHIFT 0
#define MMSC_GATE_LEVEL_MASK 0xffffu

/* the command needs a user level above 499 */
/* ENUM: GATE.level 499 above_499 established */
#define MMSC_GATE_LEVEL_ABOVE_499 499

/* the command needs a user level above 999 */
/* ENUM: GATE.level 999 above_999 established */
#define MMSC_GATE_LEVEL_ABOVE_999 999

#endif
