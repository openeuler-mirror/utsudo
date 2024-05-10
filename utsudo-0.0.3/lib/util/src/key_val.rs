/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_assignments, non_camel_case_types)]

use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;

#[no_mangle]
pub unsafe extern "C" fn sudo_new_key_val_v1(
    key: *const libc::c_char,
    val: *const libc::c_char,
) -> *mut libc::c_char {
    let key_len: size_t = strlen(key);
    let val_len: size_t = strlen(val);
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    str = malloc(
        key_len
            .wrapping_add(1 as libc::c_ulong)
            .wrapping_add(val_len)
            .wrapping_add(1 as libc::c_ulong),
    ) as *mut libc::c_char;
    cp = str;
    if !cp.is_null() {
        memcpy(cp as *mut libc::c_void, key as *const libc::c_void, key_len);
        cp = cp.offset(key_len as isize);
        let fresh0 = cp;
        cp = cp.offset(1);
        *fresh0 = '=' as i32 as libc::c_char;
        memcpy(cp as *mut libc::c_void, val as *const libc::c_void, val_len);
        cp = cp.offset(val_len as isize);
        *cp = '\u{0}' as i32 as libc::c_char;
    }
    debug_return_str!(str)
}
