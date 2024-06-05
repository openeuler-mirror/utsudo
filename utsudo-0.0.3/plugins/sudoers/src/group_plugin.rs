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
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
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
    fn sudo_debug_exit_int_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: libc::c_int,
    );
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    static mut sudo_printf:
        Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
    fn sudo_dso_strerror_v1() -> *mut libc::c_char;
    fn sudo_dso_unload_v1(handle: *mut libc::c_void) -> libc::c_int;
    fn sudo_dso_findsym_v1(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn sudo_dso_load_v1(path: *const libc::c_char, mode: libc::c_int) -> *mut libc::c_void;
}



#[macro_export]
macro_rules! _PATH_SUDO_PLUGIN_DIR {
    () => {
        b"/usr/libexec/utsudo/\0" as *const u8 as *const libc::c_char
    };
}

pub type sudo_printf_t =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_group_plugin {
    pub version: libc::c_uint,
    pub init: Option<
        unsafe extern "C" fn(libc::c_int, sudo_printf_t, *const *mut libc::c_char) -> libc::c_int,
    >,
    pub cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub query: Option<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *const passwd,
        ) -> libc::c_int,
    >,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(xstat_flag as libc::c_int, __path, __statbuf);
}
static mut group_handle: *mut libc::c_void = 0 as *mut libc::c_void;
static mut group_plugin: *mut sudoers_group_plugin = 0 as *mut sudoers_group_plugin;

#[no_mangle]
pub static mut path_plugin_dir: *const libc::c_char = _PATH_SUDO_PLUGIN_DIR!();