/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    dead_code
)]

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;
use crate::sudo_debug::sudo_debug_exit_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_LINENO;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn strlen(__s: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: libc::c_int, __n: size_t) -> *mut libc::c_void;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}


pub const _ISblank: libc::c_uint = 1;

pub struct sudo_lbuf {
    pub output: Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>, //int (*output)(const char *);
    pub buf: *mut libc::c_char,
    pub continuation: *const libc::c_char,
    pub indent: libc::c_int,
    pub len: libc::c_int,
    pub size: libc::c_int,
    pub cols: libc::c_short,
    pub error: libc::c_short,
}

#[no_mangle]
pub unsafe extern "C" fn sudo_lbuf_init_v1(
    mut lbuf: *mut sudo_lbuf,
    mut output: sudo_lbuf_output_t,
    mut indent: libc::c_int,
    mut continuation: *const libc::c_char,
    mut cols: libc::c_int,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    (*lbuf).output = output;
    (*lbuf).continuation = continuation;
    (*lbuf).indent = indent;
    (*lbuf).cols = cols as libc::c_short;
    (*lbuf).error = 0 as libc::c_short;
    (*lbuf).len = 0;
    (*lbuf).size = 0;
    (*lbuf).buf = 0 as *mut libc::c_char;

    debug_return!()
}

#[no_mangle]
pub unsafe extern "C" fn sudo_lbuf_destroy_v1(mut lbuf: *mut sudo_lbuf) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    free((*lbuf).buf as *mut libc::c_void);
    (*lbuf).buf = 0 as *mut libc::c_char;

    debug_return!()
}

