/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_attributes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_unsafe
)]
#![feature(extern_types, register_tool)]
//use crate::defaults::list_member;
//use crate::defaults::list_members;
//use crate::defaults::sudo_defs_table;
use crate::gc::sudoers_gc_add;
use crate::sudoers::sudo_user;
#[warn(unused_unsafe)]
use libc::free;
use crate::common::*;
pub const GC_PTR: sudoers_gc_types = 2;
pub const I_ENV_KEEP: libc::c_int = 64;
pub const I_ENV_DELETE: libc::c_int = 63;
pub const I_ENV_CHECK: libc::c_int = 62;
pub const I_ENV_RESET: libc::c_int = 61;
pub const I_ENV_FILE: libc::c_int = 67;
pub const I_SUDOERS_LOCALE: libc::c_int = 69;
pub const I_SECURE_PATH: libc::c_int = 52;
pub const I_SET_LOGNAME: libc::c_int = 30;
pub type sudoers_gc_types = libc::c_uint;
pub const PATH_MAX: libc::c_int = 4096;
pub const KEPT_MAIL: libc::c_int = 8388608;
pub const KEPT_LOGNAME: libc::c_int = 1048576;
pub const KEPT_USER: libc::c_int = 2097152;
pub const KEPT_USER_VARIABLES: libc::c_int = KEPT_LOGNAME | KEPT_USER;
pub const SUDO_HOOK_RET_STOP: libc::c_int = 1;
pub const SUDO_HOOK_RET_NEXT: libc::c_int = 0;
pub const SUDOERS_LOCALE_SUDOERS: libc::c_int = 1;
pub const _ISprint: libc::c_int = 16384;
extern "C" {
    static mut sudo_defs_table: [sudo_defs_types; 122];
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sudoers_getlocale() -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn sudo_parseln_v2(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        lineno: *mut libc::c_uint,
        fp: *mut FILE,
        flags: libc::c_int,
    ) -> ssize_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn log_warningx(flags: libc::c_int, fmt: *const libc::c_char, _: ...) -> bool;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn user_is_exempt() -> bool;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn matches_env_pattern(
        pattern: *const libc::c_char,
        var: *const libc::c_char,
        full_match: *mut bool,
    ) -> bool;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_strlcat(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_env_file {
    pub open: Option<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub next:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_int) -> *mut libc::c_char>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment {
    pub envp: *mut *mut libc::c_char,
    pub old_envp: *mut *mut libc::c_char,
    pub env_size: size_t,
    pub env_len: size_t,
}
