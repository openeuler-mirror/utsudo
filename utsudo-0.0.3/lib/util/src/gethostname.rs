/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_imports, unused_assignments, non_camel_case_types)]

use crate::macro_struct::*;

use std::ffi::CStr;
use std::string::String;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
}

pub const _SC_HOST_NAME_MAX: libc::c_uint = 180;

#[no_mangle]
pub unsafe extern "C" fn sudo_gethostname_v1() -> *mut libc::c_char {
    let mut hname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_name_max: size_t = 0;

    host_name_max = sysconf(_SC_HOST_NAME_MAX as libc::c_int) as size_t;
    if host_name_max == -(1 as libc::c_int) as size_t {
        host_name_max = 255 as libc::c_int as size_t;
    }
    hname =
        malloc(host_name_max.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if !hname.is_null() {
        if gethostname(
            hname,
            host_name_max.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
            && *hname as libc::c_int != '\u{0}' as i32
        {
            *hname.offset(host_name_max as isize) = '\u{0}' as i32 as libc::c_char;
        } else {
            free(hname as *mut libc::c_void);
            hname = 0 as *mut libc::c_char;
        }
    }
    return hname;
}
