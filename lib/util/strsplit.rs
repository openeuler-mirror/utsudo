/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_variables, unused_mut, unused_assignments)]

//use other file's func
use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_ptr_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

//line42
unsafe fn sudo_strsplit_v1(
    mut str: *const libc::c_char,
    mut last: *mut *const libc::c_char,
) -> *const libc::c_char {
    //line 44
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;

    //line45
    debug_decl()

    //line 48
    if str.is_null() {
        str = *last;
    }

}
