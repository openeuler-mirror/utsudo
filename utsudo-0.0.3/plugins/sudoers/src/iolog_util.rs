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
    unused_mut
)]
use crate::common::*;

extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_str2sig(signame: *const libc::c_char, signum: *mut libc::c_int) -> libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_enter_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_debug_exit_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    fn sudo_debug_exit_ptr_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: *const libc::c_void,
    );
    fn sudo_debug_exit_str_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: *const libc::c_char,
    );
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
}

pub const INT_MAX: libc::c_int = 2147483647;

pub const LLONG_MAX: i64 = 9223372036854775807;
pub const TIME_T_MAX: i64 = LLONG_MAX;

pub const LONG_MAX: libc::c_long = 9223372036854775807;
pub const __LONG_MAX__: libc::c_long = LONG_MAX;

pub const IO_EVENT_COUNT: libc::c_int = 8;
pub const IO_EVENT_SUSPEND: libc::c_int = 7;
pub const IO_EVENT_TTYOUT_1_8_7: libc::c_int = 6;
pub const IO_EVENT_WINSIZE: libc::c_int = 5;
pub const ERANGE: libc::c_int = 34;

pub type __off64_t = libc::c_long;
pub type off64_t = __off64_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union io_fd {
    pub f: *mut FILE,
    pub g: gzFile,
    pub v: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_info {
    pub cwd: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub runas_user: *mut libc::c_char,
    pub runas_group: *mut libc::c_char,
    pub tty: *mut libc::c_char,
    pub cmd: *mut libc::c_char,
    pub tstamp: time_t,
    pub rows: libc::c_int,
    pub cols: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timing_closure {
    pub decimal: *const libc::c_char,
    pub max_delay: *mut timespec,
    pub fd: io_fd,
    pub event: libc::c_int,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub winsize: C2RustUnnamed_1,
    pub nbytes: size_t,
    pub signo: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub rows: libc::c_int,
    pub cols: libc::c_int,
}

#[macro_export]
macro_rules! sudo_timespeccmp {
    ($ts1:expr, $ts2:expr, $op:tt) => {{
    (if (*$ts1).tv_sec == (*$ts2).tv_sec {
        ((*$ts1).tv_nsec $op (*$ts2).tv_nsec) as libc::c_int
    } else {
        ((*$ts1).tv_sec $op (*$ts2).tv_sec) as libc::c_int
    })
    }};
}

#[macro_export]
macro_rules! ULONG_MAX {
    () => {{
        (__LONG_MAX__ as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
    }};
}
static mut timing_event_adj: libc::c_int = 0;