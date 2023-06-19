/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}

pub type size_t = libc::c_ulong;
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615;
pub const RSIZE_MAX: libc::c_ulong = SIZE_MAX >> 1;

pub fn sudo_memset_s(
    mut v: *mut libc::c_void,
    mut smax: size_t,
    mut c: libc::c_int,
    mut n: size_t,
) -> libc::c_int {
    let mut ret = 0;
    let mut s: *mut libc::c_uchar = v as *mut libc::c_uchar;

    if s.is_null() {
        let ref mut fresh0 = unsafe { *__errno_location() };
    } else {
    }
    return ret;
}

