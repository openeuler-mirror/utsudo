/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(non_camel_case_types, dead_code, unused_mut, unused_variables)]
pub type size_t = libc::c_ulong;

#[no_mangle]
fn sudo_strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut dsize: size_t,
) -> size_t {
    let mut osrc: *const libc::c_char = src;
    let mut nleft: size_t = dsize;

    if nleft != 0 as libc::c_int as libc::c_ulong {
        unsafe {
            loop {
                nleft = nleft.wrapping_sub(1);
                if !(nleft != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                let fresh0 = src;
                src = src.offset(1);
                let fresh1 = dst;
                dst = dst.offset(1);
                *fresh1 = *fresh0;

                if *fresh1 as libc::c_int == '\u{0}' as i32 {
                    break;
                }
            }
        }
    }

    if nleft == 0 as libc::c_int as libc::c_ulong {
        unsafe {
            if dsize != 0 as libc::c_int as libc::c_ulong {
                *dst = '\u{0}' as libc::c_char;
            }
            loop {
                let fresh2 = src;
                src = src.offset(1);
                if !(*fresh2 != 0) {
                    break;
                }
            }
        }
    }

    unsafe {
        return (src.offset_from(osrc) as libc::c_long - 1 as libc::c_long) as size_t;
    }
}
