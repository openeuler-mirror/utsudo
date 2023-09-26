/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::struct_macro::*;
use crate::process_hooks_unsetenv;
pub const SUDO_HOOK_RET_STOP: libc::c_int = 1;
pub const SUDO_HOOK_RET_ERROR: libc::c_int = -1;
pub type sudo_fn_putenv_t = Option<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>;
pub type sudo_fn_setenv_t = Option<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char, libc::c_int) -> libc::c_int,
>;
pub type sudo_fn_unsetenv_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;

extern "C" {
    static mut environ: *mut *mut libc::c_char;
    fn strncmp(__s1: *const libc::c_char, __s2: *const libc::c_char, __n: size_t) -> libc::c_int;
    fn strchr(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn memcpy(
        __restrict: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(__s: *const libc::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sudo_dso_findsym_v1(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn process_hooks_setenv(
        name: *const libc::c_char,
        value: *const libc::c_char,
        overwrite: libc::c_int,
    ) -> libc::c_int;
    fn process_hooks_putenv(string: *mut libc::c_char) -> libc::c_int;
    fn process_hooks_getenv(
        name: *const libc::c_char,
        value: *mut *mut libc::c_char,
    ) -> libc::c_int;
}

static mut priv_environ: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
