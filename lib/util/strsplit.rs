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
#[no_mangle]
unsafe fn sudo_strsplit_v1(
    mut str: *const libc::c_char,
    mut endstr: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut last: *mut *const libc::c_char,
) -> *const libc::c_char {
    //line 44
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;

    //line45
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    //line 48
    if str.is_null() {
        str = *last;
    }

    //line 52
    while str < endstr {
        s = sep;
        while *s as libc::c_int != '\u{0}' as i32 {
            if *str as libc::c_int == *s as libc::c_int {
                str = str.offset(1);
                break;
            } else {
                s = s.offset(1);
            }
        }
        //line 59
        if *s as libc::c_int == '\u{0}' as i32 {
            break;
        }
    }

    //line 64
    if str >= endstr {
        *last = endstr;
        debug_return_ptr!(0 as *const libc::c_char)
    }

    //line70
    cp = str;
    while cp < endstr {
        s = sep;
        while *s as libc::c_int != '\u{0}' as i32 {
            s = s.offset(1);
        }
        if *s as libc::c_int != '\u{0}' as i32 {
            break;
        }
        cp = cp.offset(1);
    }

    //line 78
    *last = cp;
    debug_return_const_ptr!(str)
}
