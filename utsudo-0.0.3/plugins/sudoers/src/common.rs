/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    non_camel_case_types,
    unused_variables,
    dead_code,
    non_upper_case_globals
)]
////////////  调用其它模块中接口
pub use libc::O_EXCL;
pub use utsudo_util::common::*;
pub use utsudo_util::*;
pub use c2rust_bitfields::BitfieldStruct;
pub use libc::c_uint;
////////////  调用第三方库函数
////////////  定义类型
pub type ldap = LDAP;
pub type ldapmsg = LDAPMessage;
pub type uint32_t = __uint32_t;
pub type GETGROUPS_T = gid_t;
pub type rbcolor = libc::c_uint;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type DIR = __dirstream;
////////////  定义静态变量值
pub const black: rbcolor = 1;
pub const red: rbcolor = 0;
pub const AF_INET6: libc::c_int = 10;
pub const AF_INET: libc::c_int = 2;
pub const SUDOERS_LOCALE_USER: libc::c_int = 0;
pub const SUDOERS_LOCALE_SUDOERS: libc::c_int = 1;
pub const ALIAS: libc::c_int = 258;
pub const RUNASALIAS: libc::c_int = 289;
pub const CMNDALIAS: libc::c_int = 287;
pub const HOSTALIAS: libc::c_int = 286;
pub const USERALIAS: libc::c_int = 288;
pub const ALL: libc::c_int = 284;
pub const MYSELF: libc::c_int = 298;
pub const COMMAND: libc::c_int = 257;
pub const _ISalnum: libc::c_uint = 8;
pub const _ISpunct: libc::c_uint = 4;
pub const _IScntrl: libc::c_uint = 2;
pub const _ISblank: libc::c_uint = 1;
pub const _ISgraph: libc::c_uint = 32768;
pub const _ISprint: libc::c_uint = 16384;
pub const _ISspace: libc::c_uint = 8192;
pub const _ISxdigit: libc::c_uint = 4096;
pub const _ISdigit: libc::c_uint = 2048;
pub const _ISalpha: libc::c_uint = 1024;
pub const _ISlower: libc::c_uint = 512;
pub const _ISupper: libc::c_uint = 256;
#[no_mangle]
pub static mut iolog_uid: uid_t = 0 as libc::c_int as uid_t;
#[no_mangle]
pub static mut iolog_gid: gid_t = 0 as libc::c_int as gid_t;
#[no_mangle]
pub static mut timestamp_uid: uid_t = 0;
#[no_mangle]
pub static mut timestamp_gid: gid_t = 0;
#[no_mangle]
pub static mut force_umask: bool = false;
#[no_mangle]
pub static mut sudo_mode: libc::c_int = 0;
pub const I_SYSLOG: isize = 0;
pub const I_SYSLOG_GOODPRI: isize = 1;
pub const I_SYSLOG_BADPRI: isize = 2;
pub const I_LONG_OTP_PROMPT: isize = 3;
pub const I_IGNORE_DOT: isize = 4;
pub const I_MAIL_ALWAYS: isize = 5;
pub const I_MAIL_BADPASS: isize = 6;
pub const I_MAIL_NO_USER: isize = 7;
pub const I_MAIL_NO_HOST: isize = 8;
pub const I_MAIL_NO_PERMS: isize = 9;
pub const I_MAIL_ALL_CMNDS: isize = 10;
pub const I_TTY_TICKETS: isize = 11;
pub const I_LECTURE: isize = 12;
pub const I_LECTURE_FILE: isize = 13;
pub const I_AUTHENTICATE: isize = 14;
pub const I_ROOT_SUDO: isize = 15;
pub const I_LOG_HOST: isize = 16;
pub const I_LOG_YEAR: isize = 17;
pub const I_SHELL_NOARGS: isize = 18;
pub const I_SET_HOME: isize = 19;
pub const I_ALWAYS_SET_HOME: isize = 20;
pub const I_PATH_INFO: isize = 21;
pub const I_FQDN: isize = 22;
pub const I_INSULTS: isize = 23;
pub const I_REQUIRETTY: isize = 24;
pub const I_ENV_EDITOR: isize = 25;
pub const I_ROOTPW: isize = 26;
pub const I_RUNASPW: isize = 27;
pub const I_TARGETPW: isize = 28;
pub const I_USE_LOGINCLASS: isize = 29;
pub const I_SET_LOGNAME: isize = 30;
pub const I_STAY_SETUID: isize = 31;
pub const I_PRESERVE_GROUPS: isize = 32;
pub const I_LOGLINELEN: isize = 33;
pub const I_TIMESTAMP_TIMEOUT: isize = 34;
pub const I_PASSWD_TIMEOUT: isize = 35;
pub const I_PASSWD_TRIES: isize = 36;
pub const I_UMASK: isize = 37;
pub const I_LOGFILE: isize = 38;
pub const I_MAILERPATH: isize = 39;
pub const I_MAILERFLAGS: isize = 40;
pub const I_MAILTO: isize = 41;
pub const I_MAILFROM: isize = 42;
pub const I_MAILSUB: isize = 43;
pub const I_BADPASS_MESSAGE: isize = 44;
pub const I_LECTURE_STATUS_DIR: isize = 45;
pub const I_TIMESTAMPDIR: isize = 46;
pub const I_TIMESTAMPOWNER: isize = 47;
pub const I_EXEMPT_GROUP: isize = 48;
pub const I_PASSPROMPT: isize = 49;
pub const I_PASSPROMPT_OVERRIDE: isize = 50;
pub const I_RUNAS_DEFAULT: isize = 51;
pub const I_SECURE_PATH: isize = 52;
pub const I_EDITOR: isize = 53;
pub const I_LISTPW: isize = 54;
pub const I_VERIFYPW: isize = 55;
pub const I_NOEXEC: isize = 56;
pub const I_IGNORE_LOCAL_SUDOERS: isize = 57;
pub const I_CLOSEFROM: isize = 58;
pub const I_CLOSEFROM_OVERRIDE: isize = 59;
pub const I_SETENV: isize = 60;
pub const I_ENV_RESET: isize = 61;
pub const I_ENV_CHECK: isize = 62;
pub const I_ENV_DELETE: isize = 63;
pub const I_ENV_KEEP: isize = 64;
pub const I_ROLE: isize = 65;
pub const I_TYPE: isize = 66;
pub const I_ENV_FILE: isize = 67;
pub const I_RESTRICTED_ENV_FILE: isize = 68;
pub const I_SUDOERS_LOCALE: isize = 69;
pub const I_VISIBLEPW: isize = 70;
pub const I_PWFEEDBACK: isize = 71;
pub const I_FAST_GLOB: isize = 72;
pub const I_UMASK_OVERRIDE: isize = 73;
pub const I_LOG_INPUT: isize = 74;
pub const I_LOG_OUTPUT: isize = 75;
pub const I_COMPRESS_IO: isize = 76;
pub const I_USE_PTY: isize = 77;
pub const I_GROUP_PLUGIN: isize = 78;
pub const I_IOLOG_DIR: isize = 79;
pub const I_IOLOG_FILE: isize = 80;
pub const I_SET_UTMP: isize = 81;
pub const I_UTMP_RUNAS: isize = 82;
pub const I_PRIVS: isize = 83;
pub const I_LIMITPRIVS: isize = 84;
pub const I_EXEC_BACKGROUND: isize = 85;
pub const I_PAM_SERVICE: isize = 86;
pub const I_PAM_LOGIN_SERVICE: isize = 87;
pub const I_PAM_SETCRED: isize = 88;
pub const I_PAM_SESSION: isize = 89;
pub const I_PAM_ACCT_MGMT: isize = 90;
pub const I_MAXSEQ: isize = 91;
pub const I_USE_NETGROUPS: isize = 92;
pub const I_SUDOEDIT_CHECKDIR: isize = 93;
pub const I_SUDOEDIT_FOLLOW: isize = 94;
pub const I_ALWAYS_QUERY_GROUP_PLUGIN: isize = 95;
pub const I_NETGROUP_TUPLE: isize = 96;
pub const I_IGNORE_AUDIT_ERRORS: isize = 97;
pub const I_IGNORE_IOLOG_ERRORS: isize = 98;
pub const I_IGNORE_LOGFILE_ERRORS: isize = 99;
pub const I_MATCH_GROUP_BY_GID: isize = 100;
pub const I_SYSLOG_MAXLEN: isize = 101;
pub const I_IOLOG_USER: isize = 102;
pub const I_IOLOG_GROUP: isize = 103;
pub const I_IOLOG_MODE: isize = 104;
pub const I_FDEXEC: isize = 105;
pub const I_IGNORE_UNKNOWN_DEFAULTS: isize = 106;
pub const I_COMMAND_TIMEOUT: isize = 107;
pub const I_USER_COMMAND_TIMEOUTS: isize = 108;
pub const I_IOLOG_FLUSH: isize = 109;
pub const I_SYSLOG_PID: isize = 110;
pub const I_TIMESTAMP_TYPE: isize = 111;
pub const I_AUTHFAIL_MESSAGE: isize = 112;
pub const I_CASE_INSENSITIVE_USER: isize = 113;
pub const I_CASE_INSENSITIVE_GROUP: isize = 114;
pub const I_LOG_ALLOWED: isize = 115;
pub const I_LOG_DENIED: isize = 116;
pub const I_LEGACY_GROUP_PROCESSING: isize = 117;
pub const I_CMND_NO_WAIT: isize = 118;
pub const I_RUNAS_ALLOW_UNKNOWN_ID: isize = 119;
pub const I_RUNAS_CHECK_SHELL: isize = 120;
pub const MODE_BACKGROUND: libc::c_int = 0x00010000;
pub const MODE_SHELL: libc::c_int = 0x00020000;
pub const MODE_LOGIN_SHELL: libc::c_int = 0x00040000;
pub const MODE_IMPLIED_SHELL: libc::c_int = 0x00080000;
pub const MODE_RESET_HOME: libc::c_int = 0x00100000;
pub const MODE_PRESERVE_GROUPS: libc::c_int = 0x00200000;
pub const MODE_PRESERVE_ENV: libc::c_int = 0x00400000;
pub const MODE_NONINTERACTIVE: libc::c_int = 0x00800000;
pub const MODE_IGNORE_TICKET: libc::c_int = 0x01000000;
pub const UNSPEC: libc::c_int = -1;
pub const O_WRONLY: libc::c_int = 0o1;
pub const O_TRUNC: libc::c_int = 0o1000;
pub const S_IRWXU: libc::c_int = 0o400 | 0o200 | 0o100;
pub const S_IRUSR: libc::c_int = 0o400;
pub const LINE_MAX: libc::c_int = 2048;
pub const GRMEM_MAX: libc::c_int = 200;
pub const EEXIST: libc::c_int = 17;
pub const SLOG_SEND_MAIL: libc::c_int = 0x08;
pub const FLAG_BAD_PASSWORD: libc::c_int = 0x200;
pub const IMPLIED: libc::c_int = 2;
pub const NTWKADDR: libc::c_int = 260;
pub const NETGROUP: libc::c_int = 261;
pub const USERGROUP: libc::c_int = 262;
pub const WORD: libc::c_int = 263;
////////////  定义宏
// #define SUDOERS_DEBUG_ALIAS	(sudoers_subsystem_ids[ 0]) /* sudoers alias functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_ALIAS {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(0 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_AUDIT	(sudoers_subsystem_ids[ 1]) /* audit */
#[macro_export]
macro_rules! SUDOERS_DEBUG_AUDIT {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(1 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_AUTH	(sudoers_subsystem_ids[ 2]) /* authentication functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_AUTH {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(2 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_DEFAULTS	(sudoers_subsystem_ids[ 3]) /* sudoers defaults settings */
#[macro_export]
macro_rules! SUDOERS_DEBUG_DEFAULTS {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(3 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_ENV	(sudoers_subsystem_ids[ 4]) /* environment handling */
#[macro_export]
macro_rules! SUDOERS_DEBUG_ENV {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(4 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_EVENT	(sudoers_subsystem_ids[ 5]) /* event handling */
#[macro_export]
macro_rules! SUDOERS_DEBUG_EVENT {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(5 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_LDAP	(sudoers_subsystem_ids[ 6]) /* sudoers LDAP */
#[macro_export]
macro_rules! SUDOERS_DEBUG_LDAP {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(6 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_LOGGING	(sudoers_subsystem_ids[ 7]) /* logging functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_LOGGING {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(7 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_MAIN	(sudoers_subsystem_ids[ 8]) /* main() */
#[macro_export]
macro_rules! SUDOERS_DEBUG_MAIN {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(8 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_MATCH	(sudoers_subsystem_ids[ 9]) /* sudoers matching */
#[macro_export]
macro_rules! SUDOERS_DEBUG_MATCH {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(9 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_NETIF	(sudoers_subsystem_ids[10]) /* network interface functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_NETIF {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(10 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_NSS	(sudoers_subsystem_ids[11]) /* network service switch */
#[macro_export]
macro_rules! SUDOERS_DEBUG_NSS {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(11 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_PARSER	(sudoers_subsystem_ids[12]) /* sudoers parser */
#[macro_export]
macro_rules! SUDOERS_DEBUG_PARSER {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(12 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_PERMS	(sudoers_subsystem_ids[13]) /* uid/gid swapping functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_PERMS {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(13 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_PLUGIN	(sudoers_subsystem_ids[14]) /* main plugin functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_PLUGIN {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(14 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_RBTREE	(sudoers_subsystem_ids[15]) /* red-black tree functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_RBTREE {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(15 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_SSSD	(sudoers_subsystem_ids[16]) /* sudoers SSSD */
#[macro_export]
macro_rules! SUDOERS_DEBUG_SSSD {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(16 as isize) as libc::c_int)
    };
}
//#define SUDOERS_DEBUG_UTIL	(sudoers_subsystem_ids[17]) /* utility functions */
#[macro_export]
macro_rules! SUDOERS_DEBUG_UTIL {
    () => {
        (*sudoers_subsystem_ids.as_mut_ptr().offset(17 as isize) as libc::c_int)
    };
}
/* The value returned by fgetc and similar functions to indicate the
end of the file.  */
//    #define EOF (-1)
#[macro_export]
macro_rules! EOF {
    () => {
        -(1 as libc::c_int)
    };
}
#[macro_export]
macro_rules! _PATH_LDAP_CONF {
    () => {
        b"/etc/utsudo-ldap.conf\0" as *const u8 as *const libc::c_char
    };
}
#[macro_export]
macro_rules! _PATH_LDAP_SECRET {
    () => {
        b"/etc/ldap.secret\0" as *const u8 as *const libc::c_char
    };
}
#[macro_export]
macro_rules! user_name {
    () => {
        (sudo_user.name)
    };
}
#[macro_export]
macro_rules! user_uid {
    () => {
        (sudo_user.uid)
    };
}
#[macro_export]
macro_rules! user_gid {
    () => {
        (sudo_user.gid)
    };
}
#[macro_export]
macro_rules! user_sid {
    () => {
        (sudo_user.sid)
    };
}
#[macro_export]
macro_rules! user_umask {
    () => {
        (sudo_user.umask)
    };
}
#[macro_export]
macro_rules! user_gids {
    () => {
        (sudo_user.gids)
    };
}
#[macro_export]
macro_rules! user_ngids {
    () => {
        (sudo_user.ngids)
    };
}
#[macro_export]
macro_rules! user_gid_list {
    () => {
        (sudo_user.gid_list)
    };
}
#[macro_export]
macro_rules! user_tty {
    () => {
        (sudo_user.tty)
    };
}
#[macro_export]
macro_rules! user_ttypath {
    () => {
        (sudo_user.ttypath)
    };
}
#[macro_export]
macro_rules! user_cwd {
    () => {
        (sudo_user.cwd)
    };
}
#[macro_export]
macro_rules! user_cmnd {
    () => {
        (sudo_user.cmnd)
    };
}
#[macro_export]
macro_rules! user_args {
    () => {
        (sudo_user.cmnd_args)
    };
}
#[macro_export]
macro_rules! user_base {
    () => {
        (sudo_user.cmnd_base)
    };
}
#[macro_export]
macro_rules! user_stat {
    () => {
        (sudo_user.stat)
    };
}
#[macro_export]
macro_rules! user_path {
    () => {
        (sudo_user.path)
    };
}
#[macro_export]
macro_rules! user_prompt {
    () => {
        (sudo_user.prompt)
    };
}
#[macro_export]
macro_rules! user_host {
    () => {
        (sudo_user.host)
    };
}
#[macro_export]
macro_rules! user_shost {
    () => {
        (sudo_user.shost)
    };
}
#[macro_export]
macro_rules! user_runhost {
    () => {
        (sudo_user.runhost)
    };
}
#[macro_export]
macro_rules! user_srunhost {
    () => {
        (sudo_user.srunhost)
    };
}
#[macro_export]
macro_rules! user_ccname {
    () => {
        (sudo_user.krb5_ccname)
    };
}
#[macro_export]
macro_rules! safe_cmnd {
    () => {
        (sudo_user.cmnd_safe)
    };
}
#[macro_export]
macro_rules! cmnd_fd {
    () => {
        (sudo_user.execfd)
    };
}
#[macro_export]
macro_rules! login_class {
    () => {
        (sudo_user.class_name)
    };
}
#[macro_export]
macro_rules! runas_pw {
    () => {
        (sudo_user._runas_pw)
    };
}
#[macro_export]
macro_rules! runas_gr {
    () => {
        (sudo_user._runas_gr)
    };
}
#[macro_export]
macro_rules! user_role {
    () => {
        (sudo_user.role)
    };
}
#[macro_export]
macro_rules! user_type {
    () => {
        (sudo_user.type_0)
    };
}
#[macro_export]
macro_rules! user_closefrom {
    () => {
        (sudo_user.closefrom)
    };
}
#[macro_export]
macro_rules! runas_privs {
    () => {
        (sudo_user.privs)
    };
}
#[macro_export]
macro_rules! runas_limitprivs {
    () => {
        (sudo_user.limitprivs)
    };
}
#[macro_export]
macro_rules! user_timeout {
    () => {
        (sudo_user.timeout)
    };
}
#[macro_export]
macro_rules! def_syslog {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SYSLOG)).sd_un.ival
    };
}
#[macro_export]
macro_rules! def_syslog_goodpri {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SYSLOG_GOODPRI))
            .sd_un
            .ival
    };
}
#[macro_export]
macro_rules! def_syslog_badpri {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SYSLOG_BADPRI))
            .sd_un
            .ival
    };
}
#[macro_export]
macro_rules! def_long_otp_prompt {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LONG_OTP_PROMPT))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_ignore_dot {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IGNORE_DOT))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_mail_always {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAIL_ALWAYS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_mail_badpass {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAIL_BADPASS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_mail_no_user {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAIL_NO_USER))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_mail_no_host {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAIL_NO_HOST))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_mail_no_perms {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAIL_NO_PERMS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_mail_all_cmnds {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAIL_ALL_CMNDS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_tty_tickets {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_TTY_TICKETS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_lecture {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LECTURE))
            .sd_un
            .tuple
    };
}
#[macro_export]
macro_rules! def_lecture_file {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LECTURE_FILE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_authenticate {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_AUTHENTICATE))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_root_sudo {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ROOT_SUDO))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_log_host {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOG_HOST))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_log_year {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOG_YEAR))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_shell_noargs {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SHELL_NOARGS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_set_home {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SET_HOME))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_always_set_home {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ALWAYS_SET_HOME))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_path_info {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PATH_INFO))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_fqdn {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_FQDN)).sd_un.flag
    };
}
#[macro_export]
macro_rules! def_insults {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_INSULTS)).sd_un.flag
    };
}
#[macro_export]
macro_rules! def_requiretty {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_REQUIRETTY))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_env_editor {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ENV_EDITOR))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_rootpw {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ROOTPW)).sd_un.flag
    };
}
#[macro_export]
macro_rules! def_runaspw {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_RUNASPW)).sd_un.flag
    };
}
#[macro_export]
macro_rules! def_targetpw {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_TARGETPW))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_use_loginclass {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_USE_LOGINCLASS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_set_logname {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SET_LOGNAME))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_stay_setuid {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_STAY_SETUID))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_preserve_groups {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PRESERVE_GROUPS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_loglinelen {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOGLINELEN))
            .sd_un
            .uival
    };
}
#[macro_export]
macro_rules! def_timestamp_timeout {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_TIMESTAMP_TIMEOUT))
            .sd_un
            .tspec
    };
}
#[macro_export]
macro_rules! def_passwd_timeout {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PASSWD_TIMEOUT))
            .sd_un
            .tspec
    };
}
#[macro_export]
macro_rules! def_passwd_tries {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PASSWD_TRIES))
            .sd_un
            .uival
    };
}
#[macro_export]
macro_rules! def_umask {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_UMASK)).sd_un.mode
    };
}
#[macro_export]
macro_rules! def_logfile {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOGFILE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_mailerpath {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAILERPATH))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_mailerflags {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAILERFLAGS))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_mailto {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAILTO)).sd_un.str_0
    };
}
#[macro_export]
macro_rules! def_mailfrom {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAILFROM))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_mailsub {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAILSUB))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_badpass_message {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_BADPASS_MESSAGE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_lecture_status_dir {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LECTURE_STATUS_DIR))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_timestampdir {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_TIMESTAMPDIR))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_timestampowner {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_TIMESTAMPOWNER))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_exempt_group {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_EXEMPT_GROUP))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_passprompt {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PASSPROMPT))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_passprompt_override {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PASSPROMPT_OVERRIDE))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_runas_default {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_RUNAS_DEFAULT))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_secure_path {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SECURE_PATH))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_editor {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_EDITOR)).sd_un.str_0
    };
}
#[macro_export]
macro_rules! def_listpw {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LISTPW)).sd_un.tuple
    };
}
#[macro_export]
macro_rules! def_verifypw {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_VERIFYPW))
            .sd_un
            .tuple
    };
}
#[macro_export]
macro_rules! def_noexec {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_NOEXEC)).sd_un.flag
    };
}
#[macro_export]
macro_rules! def_ignore_local_sudoers {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IGNORE_LOCAL_SUDOERS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_closefrom {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_CLOSEFROM))
            .sd_un
            .ival
    };
}
#[macro_export]
macro_rules! def_closefrom_override {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_CLOSEFROM_OVERRIDE))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_setenv {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SETENV)).sd_un.flag
    };
}
#[macro_export]
macro_rules! def_env_reset {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ENV_RESET))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_env_check {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ENV_CHECK))
            .sd_un
            .list
    };
}
#[macro_export]
macro_rules! def_env_delete {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ENV_DELETE))
            .sd_un
            .list
    };
}
#[macro_export]
macro_rules! def_env_keep {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ENV_KEEP))
            .sd_un
            .list
    };
}
#[macro_export]
macro_rules! def_role {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ROLE)).sd_un.str_0
    };
}
#[macro_export]
macro_rules! def_type {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_TYPE)).sd_un.str_0
    };
}
#[macro_export]
macro_rules! def_env_file {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_ENV_FILE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_restricted_env_file {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_RESTRICTED_ENV_FILE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_sudoers_locale {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SUDOERS_LOCALE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_visiblepw {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_VISIBLEPW))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_pwfeedback {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PWFEEDBACK))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_fast_glob {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_FAST_GLOB))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_umask_override {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_UMASK_OVERRIDE))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_log_input {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOG_INPUT))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_log_output {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOG_OUTPUT))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_compress_io {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_COMPRESS_IO))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_use_pty {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_USE_PTY)).sd_un.flag
    };
}
#[macro_export]
macro_rules! def_group_plugin {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_GROUP_PLUGIN))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_iolog_dir {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IOLOG_DIR))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_iolog_file {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IOLOG_FILE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_set_utmp {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SET_UTMP))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_utmp_runas {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_UTMP_RUNAS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_privs {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PRIVS)).sd_un.str_0
    };
}
#[macro_export]
macro_rules! def_limitprivs {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LIMITPRIVS))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_exec_background {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_EXEC_BACKGROUND))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_pam_service {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PAM_SERVICE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_pam_login_service {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PAM_LOGIN_SERVICE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_pam_setcred {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PAM_SETCRED))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_pam_session {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PAM_SESSION))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_pam_acct_mgmt {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_PAM_ACCT_MGMT))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_maxseq {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MAXSEQ)).sd_un.uival
    };
}
#[macro_export]
macro_rules! def_use_netgroups {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_USE_NETGROUPS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_sudoedit_checkdir {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SUDOEDIT_CHECKDIR))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_sudoedit_follow {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SUDOEDIT_FOLLOW))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_always_query_group_plugin {
    () => {
        (*sudo_defs_table
            .as_mut_ptr()
            .offset(I_ALWAYS_QUERY_GROUP_PLUGIN))
        .sd_un
        .flag
    };
}
#[macro_export]
macro_rules! def_netgroup_tuple {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_NETGROUP_TUPLE))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_ignore_audit_errors {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IGNORE_AUDIT_ERRORS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_ignore_iolog_errors {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IGNORE_IOLOG_ERRORS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_ignore_logfile_errors {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IGNORE_LOGFILE_ERRORS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_match_group_by_gid {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_MATCH_GROUP_BY_GID))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_syslog_maxlen {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SYSLOG_MAXLEN))
            .sd_un
            .uival
    };
}
#[macro_export]
macro_rules! def_iolog_user {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IOLOG_USER))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_iolog_group {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IOLOG_GROUP))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_iolog_mode {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IOLOG_MODE))
            .sd_un
            .mode
    };
}
#[macro_export]
macro_rules! def_fdexec {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_FDEXEC)).sd_un.tuple
    };
}
#[macro_export]
macro_rules! def_ignore_unknown_defaults {
    () => {
        (*sudo_defs_table
            .as_mut_ptr()
            .offset(I_IGNORE_UNKNOWN_DEFAULTS))
        .sd_un
        .flag
    };
}
#[macro_export]
macro_rules! def_command_timeout {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_COMMAND_TIMEOUT))
            .sd_un
            .ival
    };
}
#[macro_export]
macro_rules! def_user_command_timeouts {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_USER_COMMAND_TIMEOUTS))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_iolog_flush {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_IOLOG_FLUSH))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_syslog_pid {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_SYSLOG_PID))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_timestamp_type {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_TIMESTAMP_TYPE))
            .sd_un
            .tuple
    };
}
#[macro_export]
macro_rules! def_authfail_message {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_AUTHFAIL_MESSAGE))
            .sd_un
            .str_0
    };
}
#[macro_export]
macro_rules! def_case_insensitive_user {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_CASE_INSENSITIVE_USER))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_case_insensitive_group {
    () => {
        (*sudo_defs_table
            .as_mut_ptr()
            .offset(I_CASE_INSENSITIVE_GROUP))
        .sd_un
        .flag
    };
}
#[macro_export]
macro_rules! def_log_allowed {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOG_ALLOWED))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_log_denied {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_LOG_DENIED))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_legacy_group_processing {
    () => {
        (*sudo_defs_table
            .as_mut_ptr()
            .offset(I_LEGACY_GROUP_PROCESSING))
        .sd_un
        .flag
    };
}
#[macro_export]
macro_rules! def_cmnd_no_wait {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_CMND_NO_WAIT))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! def_runas_allow_unknown_id {
    () => {
        (*sudo_defs_table
            .as_mut_ptr()
            .offset(I_RUNAS_ALLOW_UNKNOWN_ID))
        .sd_un
        .flag
    };
}
#[macro_export]
macro_rules! def_runas_check_shell {
    () => {
        (*sudo_defs_table.as_mut_ptr().offset(I_RUNAS_CHECK_SHELL))
            .sd_un
            .flag
    };
}
#[macro_export]
macro_rules! S_IFDIR {
    () => {
        0o040000
    };
}
#[macro_export]
macro_rules! __S_IFMT {
    () => {{
        0o170000
    }};
}
#[macro_export]
macro_rules! S_ISDIR {
    ($m:expr) => {
        ((($m) & crate::__S_IFMT!()) == crate::S_IFDIR!())
    };
}
#[macro_export]
macro_rules! TAGS_SET {
    ($t:expr) => {
        (($t).follow() != UNSPEC
            || ($t).log_input() != UNSPEC
            || ($t).log_output() != UNSPEC
            || ($t).noexec() != UNSPEC
            || ($t).nopasswd() != UNSPEC
            || ($t).send_mail() != UNSPEC
            || ($t).setenv() != UNSPEC)
    };
}
#[macro_export]
macro_rules! RUNAS_CHANGED {
    ($cs1:expr, $cs2:expr) => {
        ((*$cs1).runasuserlist != (*$cs2).runasuserlist
            || (*$cs1).runasgrouplist != (*$cs2).runasgrouplist)
    };
}
#[macro_export]
macro_rules! TAG_SET {
    ($tt:expr) => {
        (($tt) != UNSPEC && ($tt) != IMPLIED)
    };
}
#[macro_export]
macro_rules! TAGS_CHANGED {
    ($ot:expr, $nt:expr) => {
        ((TAG_SET!(($nt).follow()) && ($nt).follow() != ($ot).follow())
            || (TAG_SET!(($nt).log_input()) && ($nt).log_input() != ($ot).log_input())
            || (TAG_SET!(($nt).log_output()) && ($nt).log_output() != ($ot).log_output())
            || (TAG_SET!(($nt).noexec()) && ($nt).noexec() != ($ot).noexec())
            || (TAG_SET!(($nt).nopasswd()) && ($nt).nopasswd() != ($ot).nopasswd())
            || (TAG_SET!(($nt).setenv()) && ($nt).setenv() != ($ot).setenv())
            || (TAG_SET!(($nt).send_mail()) && ($nt).send_mail() != ($ot).send_mail()))
    };
}
#[macro_export]
macro_rules! isdigit {
    ($c:expr) => {{
        (__isctype!($c, _ISdigit))
    }};
}
#[macro_export]
macro_rules! isspace {
    ($c:expr) => {{
        (__isctype!($c, _ISspace))
    }};
}
#[macro_export]
macro_rules! _ISalnum {
    () => {
        _ISbit!(11)
    };
}
#[macro_export]
macro_rules! isalnum {
    ($c:expr) => {
        (__isctype!(($c), _ISalnum!()))
    };
}

////////////  定义结构体值
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut userspec,
    pub tqe_prev: *mut *mut userspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege_list {
    pub tqh_first: *mut privilege,
    pub tqh_last: *mut *mut privilege,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec_list {
    pub tqh_first: *mut userspec,
    pub tqh_last: *mut *mut userspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec {
    pub entries: C2RustUnnamed_5,
    pub users: member_list,
    pub privileges: privilege_list,
    pub comments: comment_list,
    pub lineno: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_parse_tree {
    pub userspecs: userspec_list,
    pub defaults: defaults_list,
    pub aliases: *mut rbtree,
    pub shost: *const libc::c_char,
    pub lhost: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_lbuf {
    pub output: Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub buf: *mut libc::c_char,
    pub continuation: *const libc::c_char,
    pub indent: libc::c_int,
    pub len: libc::c_int,
    pub size: libc::c_int,
    pub cols: libc::c_short,
    pub error: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comment_list {
    pub stqh_first: *mut sudoers_comment,
    pub stqh_last: *mut *mut sudoers_comment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_comment {
    pub entries: C2RustUnnamed_6,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub stqe_next: *mut sudoers_comment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDAP;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDAPMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pam_handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmndspec {
    pub entries: C2RustUnnamed_3,
    pub runasuserlist: *mut member_list,
    pub runasgrouplist: *mut member_list,
    pub cmnd: *mut member,
    pub tags: cmndtag,
    pub timeout: libc::c_int,
    pub notbefore: time_t,
    pub notafter: time_t,
    pub role: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut cmndspec,
    pub tqe_prev: *mut *mut cmndspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_digest {
    pub digest_type: libc::c_uint,
    pub digest_str: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_command {
    pub cmnd: *mut libc::c_char,
    pub args: *mut libc::c_char,
    pub digest: *mut command_digest,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct cmndtag {
    #[bitfield(name = "nopasswd", ty = "libc::c_int", bits = "0..=2")]
    #[bitfield(name = "noexec", ty = "libc::c_int", bits = "3..=5")]
    #[bitfield(name = "setenv", ty = "libc::c_int", bits = "6..=8")]
    #[bitfield(name = "log_input", ty = "libc::c_int", bits = "9..=11")]
    #[bitfield(name = "log_output", ty = "libc::c_int", bits = "12..=14")]
    #[bitfield(name = "send_mail", ty = "libc::c_int", bits = "15..=17")]
    #[bitfield(name = "follow", ty = "libc::c_int", bits = "18..=20")]
    pub nopasswd_noexec_setenv_log_input_log_output_send_mail_follow: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmndspec_list {
    pub tqh_first: *mut cmndspec,
    pub tqh_last: *mut *mut cmndspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege {
    pub entries: C2RustUnnamed_4,
    pub ldap_role: *mut libc::c_char,
    pub hostlist: member_list,
    pub cmndlist: cmndspec_list,
    pub defaults: defaults_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut privilege,
    pub tqe_prev: *mut *mut privilege,
}
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct group_list {
    pub ngroups: libc::c_int,
    pub groups: *mut *mut libc::c_char,
}
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct gid_list {
    pub ngids: libc::c_int,
    pub gids: *mut GETGROUPS_T,
}
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct __dirstream {
    _unused: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_digest {
    pub func: *mut digest_function,
    pub ctx: SHA2_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub st32: [uint32_t; 8],
    pub st64: [uint64_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct digest_function {
    pub digest_len: libc::c_uint,
    pub init: Option<unsafe extern "C" fn(*mut SHA2_CTX) -> ()>,
    pub update: Option<unsafe extern "C" fn(*mut SHA2_CTX, *const libc::c_uchar, size_t) -> ()>,
    pub final_0: Option<unsafe extern "C" fn(*mut libc::c_uchar, *mut SHA2_CTX) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_dfa_t {
    pub _address: libc::c_uchar,
}
#[derive(Clone)]
#[repr(C)]
pub struct cache_item {
    pub refcnt: libc::c_uint,
    pub type_0: libc::c_uint,
    pub registry: [libc::c_char; 16],
    pub k: cache_item_key_union,
    pub d: cache_item_datum_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cache_item_key_union {
    pub uid: uid_t,
    pub gid: gid_t,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cache_item_datum_union {
    pub pw: *mut passwd,
    pub gr: *mut group,
    pub grlist: *mut group_list,
    pub gidlist: *mut gid_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub compar:
        Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    pub root: rbnode,
    pub nil: rbnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub data: *mut libc::c_void,
    pub color: rbcolor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
    pub name: *mut libc::c_char,
    pub type0: libc::c_ushort,
    pub used: libc::c_short,
    pub lineno: libc::c_int,
    pub file: *mut libc::c_char,
    pub members: member_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct member_list {
    pub tqh_first: *mut member,
    pub tqh_last: *mut *mut member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct member {
    pub entries: TAILQ_ENTRY_member,
    pub name: *mut libc::c_char,
    pub type0: libc::c_short,
    pub negated: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_member {
    pub tqe_next: *mut member,
    pub tqe_prev: *mut *mut member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defaults_list {
    pub tqh_first: *mut defaults,
    pub tqh_last: *mut *mut defaults,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defaults {
    pub entries: C2RustUnnamed_2,
    pub var: *mut libc::c_char,
    pub val: *mut libc::c_char,
    pub binding: *mut member_list,
    pub file: *mut libc::c_char,
    pub type_0: libc::c_short,
    pub op: libc::c_char,
    pub error: libc::c_char,
    pub lineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut member,
    pub tqe_prev: *mut *mut member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut defaults,
    pub tqe_prev: *mut *mut defaults,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_values {
    pub sval: *mut libc::c_char,
    pub nval: def_tuple,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_defs_types {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub desc: *mut libc::c_char,
    pub values: *mut def_values,
    pub callback: Option<unsafe extern "C" fn(*const sudo_defs_val) -> bool>,
    pub sd_un: sudo_defs_val,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_member {
    pub entries: TAILQ_ENTRY_list_member,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_list_member {
    pub sle_next: *mut list_member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_members {
    pub slh_first: *mut list_member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sudo_defs_val {
    pub flag: libc::c_int,
    pub ival: libc::c_int,
    pub uival: libc::c_uint,
    pub tuple: def_tuple,
    pub str_0: *mut libc::c_char,
    pub mode: mode_t,
    pub tspec: timespec,
    pub list: list_members,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum def_tuple {
    never,
    once,
    always,
    any,
    all,
    digest_only,
    global,
    ppid,
    tty,
    kernel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_options {
    pub notbefore: time_t,
    pub notafter: time_t,
    pub timeout: libc::c_int,
    pub role: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_user {
    pub pw: *mut passwd,
    pub _runas_pw: *mut passwd,
    pub _runas_gr: *mut group,
    pub cmnd_stat: *mut stat,
    pub name: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub tty: *mut libc::c_char,
    pub ttypath: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub shost: *mut libc::c_char,
    pub runhost: *mut libc::c_char,
    pub srunhost: *mut libc::c_char,
    pub prompt: *mut libc::c_char,
    pub cmnd: *mut libc::c_char,
    pub cmnd_args: *mut libc::c_char,
    pub cmnd_base: *mut libc::c_char,
    pub cmnd_safe: *mut libc::c_char,
    pub class_name: *mut libc::c_char,
    pub krb5_ccname: *mut libc::c_char,
    pub gid_list: *mut gid_list,
    pub env_vars: *const *mut libc::c_char,
    pub role: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub iolog_file: *mut libc::c_char,
    pub gids: *mut gid_t,
    pub execfd: libc::c_int,
    pub ngids: libc::c_int,
    pub closefrom: libc::c_int,
    pub lines: libc::c_int,
    pub cols: libc::c_int,
    pub flags: libc::c_int,
    pub max_groups: libc::c_int,
    pub timeout: libc::c_int,
    pub umask: mode_t,
    pub uid: uid_t,
    pub gid: uid_t,
    pub sid: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}

