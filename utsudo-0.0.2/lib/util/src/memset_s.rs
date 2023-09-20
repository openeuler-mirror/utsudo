/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(dead_code, non_camel_case_types, unused_mut)]

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}

pub type size_t = libc::c_ulong;
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615;
pub const RSIZE_MAX: libc::c_ulong = SIZE_MAX >> 1;
pub const EINVAL: libc::c_int = 22;

#[no_mangle]
pub fn sudo_memset_s(
    mut v: *mut libc::c_void,
    mut smax: size_t,
    mut c: libc::c_int,
    mut n: size_t,
) -> libc::c_int {
    let mut ret = 0;
    let mut s: *mut libc::c_uchar = v as *mut libc::c_uchar;

    if s.is_null() || smax > RSIZE_MAX {
        let ref mut fresh0 = unsafe { *__errno_location() };
        *fresh0 = EINVAL;
        ret = *fresh0;
    } else {
        if n > smax {
            n = smax;
            let ref mut fresh1 = unsafe { *__errno_location() };
            *fresh1 = EINVAL;
            ret = *fresh1;
        }
        loop {
            let fresh2 = n;
            n = n.wrapping_sub(1); //n--
            if !(fresh2 != 0) {
                break;
            }
            let fresh3 = s;
            unsafe {
                s = s.offset(1); //*s++
                ::std::ptr::write_volatile(fresh3, c as libc::c_uchar);
            }
        }
    }
    return ret;
}
