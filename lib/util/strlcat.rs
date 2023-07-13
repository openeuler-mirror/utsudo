/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

pub type size_t = libc::c_ulong;

extern "C" {
    fn strlen(__s: *const libc::c_char) -> size_t;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_strlcat(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut dsize: size_t,
) -> size_t {
    let mut odst: *const libc::c_char = dst;
    let mut osrc: *const libc::c_char = src;
    let mut n: size_t = dsize;
    let mut dlen: size_t = 0;

    while n != 0 && *dst as libc::c_int != '\u{0}' as i32 {
        dst = dst.offset(1);
        n = n.wrapping_sub(1); // n--
    }

    dlen = dst.offset_from(odst) as size_t;
    n = dsize.wrapping_sub(dlen); //dsize - dlen

    n -= 1;
    if n == 0 {
        return dlen + strlen(src);
    }

    while *src as libc::c_int != '\u{0}' as i32 {
        if n != 0 {
            *dst = *src;
            dst = dst.offset(1);
            n -= 1;
        }
        src = src.offset(1);
    }
    *dst = '\u{0}' as libc::c_char;

    return dlen.wrapping_add(src.offset_from(osrc) as libc::c_ulong);
}
