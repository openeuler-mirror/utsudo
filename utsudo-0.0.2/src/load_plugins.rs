/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
extern "C" {
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn register_hook(hook: *mut sudo_hook) -> libc::c_int;
    static mut sudo_debug_instance: libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn sudo_conf_plugin_dir_path_v1() -> *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn sudo_dso_strerror_v1() -> *mut libc::c_char;
    fn sudo_dso_load_v1(path: *const libc::c_char, mode: libc::c_int) -> *mut libc::c_void;
    fn sudo_dso_findsym_v1(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_conf_debug_files_v1(progname: *const libc::c_char) -> *mut sudo_conf_debug_file_list;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_dso_unload_v1(handle: *mut libc::c_void) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn sudo_conf_plugins_v1() -> *mut plugin_info_list;
    fn sudo_debug_set_active_instance_v1(inst: libc::c_int) -> libc::c_int;
}
use crate::sudo_debug_printf2_v1;
use stdext::function_name;
