/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(dead_code, non_upper_case_globals, unused_mut)]

extern "C" {
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut __progname: *const libc::c_char;
}
static mut progname: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char; //*progname = ""
#[no_mangle]
pub unsafe extern "C" fn initprogname(mut name: *const libc::c_char) {
    if !__progname.is_null() && *__progname as libc::c_int != '\u{0}' as i32 {
        progname = __progname;
    } else {
        progname = strrchr(name, '/' as i32);
        if !progname.is_null() {
            progname = progname.offset(1); //progname++
        } else {
            progname = name;
        }
    }

        if *progname.offset(0 as isize) as libc::c_int == 'l' as i32
        && *progname.offset(1 as isize) as libc::c_int == 't' as i32
        && *progname.offset(2 as isize) as libc::c_int == '-' as i32
        && *progname.offset(3 as isize) as libc::c_int == '\u{0}' as i32
    {
        progname = progname.offset(3 as isize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sudo_getprogname() -> *const libc::c_char {
    return progname;
}

