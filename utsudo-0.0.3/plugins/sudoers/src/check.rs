/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unreachable_code,
    non_snake_case,
    unused_variables
)]
use crate::common::*;

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    static mut sudo_conv: sudo_conv_t;
    static mut sudo_defs_table: [sudo_defs_types; 0];
    static mut sudo_user: sudo_user;
    fn timestamp_update(vcookie: *mut libc::c_void, pw: *mut passwd) -> bool;
    fn timestamp_close(vcookie: *mut libc::c_void);
    fn sudo_getpwnam(_: *const libc::c_char) -> *mut passwd;
    fn sudo_auth_cleanup(pw: *mut passwd) -> libc::c_int;
    fn sudo_pw_addref(_: *mut passwd);
    fn sudo_pw_delref(_: *mut passwd);
    fn already_lectured(status: libc::c_int) -> bool;
    fn sudo_auth_init(pw: *mut passwd) -> libc::c_int;
    fn sudo_auth_approval(pw: *mut passwd, validated: libc::c_int, exempt: bool) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn log_warningx(flags: libc::c_int, fmt: *const libc::c_char, _: ...) -> bool;
    fn user_in_group(_: *const passwd, _: *const libc::c_char) -> bool;
    fn sudo_getpwuid(_: uid_t) -> *mut passwd;
    fn timestamp_open(user: *mut libc::c_char, sid: pid_t) -> *mut libc::c_void;
    fn timestamp_lock(vcookie: *mut libc::c_void, pw: *mut passwd) -> bool;
    fn timestamp_status(vcookie: *mut libc::c_void, pw: *mut passwd) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn log_auth_failure(status: libc::c_int, tries: libc::c_uint) -> bool;
    fn expand_prompt(
        old_prompt: *const libc::c_char,
        auth_user: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn verify_user(
        pw: *mut passwd,
        prompt: *mut libc::c_char,
        validated: libc::c_int,
        callback: *mut sudo_conv_callback,
    ) -> libc::c_int;
    fn set_lectured() -> libc::c_int;
    fn free(__pt: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn endusershell();
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn getusershell() -> *mut libc::c_char;
    fn setusershell();
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

pub const VALIDATE_SUCCESS: libc::c_int = 0x002;
pub const TS_ERROR: libc::c_int = 3;
pub const BUFSIZ: libc::c_int = 8192;
pub const TS_CURRENT: libc::c_int = 0;
pub const TS_FATAL: libc::c_int = 4;
pub const FLAG_CHECK_USER: libc::c_int = 0x010;
pub const FLAG_NON_INTERACTIVE: libc::c_int = 0x100;

pub const MODE_NONINTERACTIVE: libc::c_int = 0x00800000;
pub const MODE_IGNORE_TICKET: libc::c_int = 0x01000000;

pub const SUDO_CONV_ERROR_MSG: libc::c_int = 0x0003;
pub const SUDO_CONV_PREFER_TTY: libc::c_int = 0x2000;

pub const MODE_CHECK: libc::c_int = 0x00000100;
pub const MODE_LIST: libc::c_int = 0x00000080;

pub const ROOT_UID: libc::c_int = 0;
pub const SLOG_SEND_MAIL: libc::c_int = 0x08;
pub const SLOG_RAW_MSG: libc::c_int = 0x04;

pub type sudo_conv_t = Option<
    unsafe extern "C" fn(
        libc::c_int,
        *const sudo_conv_message,
        *mut sudo_conv_reply,
        *mut sudo_conv_callback,
    ) -> libc::c_int,
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct getpass_closure {
    pub tstat: libc::c_int,
    pub cookie: *mut libc::c_void,
    pub auth_pw: *mut passwd,
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
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_callback {
    pub version: libc::c_uint,
    pub closure: *mut libc::c_void,
    pub on_suspend: sudo_conv_callback_fn_t,
    pub on_resume: sudo_conv_callback_fn_t,
}
pub type sudo_conv_callback_fn_t =
    Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_message {
    pub msg_type: libc::c_int,
    pub timeout: libc::c_int,
    pub msg: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_reply {
    pub reply: *mut libc::c_char,
}

#[macro_export]
macro_rules! ISSET {
    ($t:expr,$f:expr) => {
        (($t) & ($f))
    };
}

// #define SUDO_CONV_CALLBACK_VERSION_MAJOR	1
// #define SUDO_CONV_CALLBACK_VERSION_MINOR	0
pub const SUDO_CONV_CALLBACK_VERSION_MAJOR: libc::c_int = 1;
pub const SUDO_CONV_CALLBACK_VERSION_MINOR: libc::c_int = 0;

#[macro_export]
macro_rules! SUDO_API_MKVERSION {
    ($x: expr, $y: expr) => {
        ((($x) << 16) | ($y))
    };
}

#[macro_export]
macro_rules! SUDO_CONV_CALLBACK_VERSION {
    () => {
        SUDO_API_MKVERSION!(
            SUDO_CONV_CALLBACK_VERSION_MAJOR,
            SUDO_CONV_CALLBACK_VERSION_MINOR
        )
    };
}


