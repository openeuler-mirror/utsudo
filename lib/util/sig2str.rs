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
pub type __int32_t = libc::c_int;

extern "C" {
    fn __errno_location()
    fn __libc_current_sigrtmin()
    fn __libc_current_sigrtmax()
    fn sysconf()
    fn snprintf()
    fn sudo_strlcpy()
    fn __ctype_toupper_loc()
    fn __ctype_b_loc()
}

pub unsafe extern "C" fn toupper() -> libc::c_int {
    return libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    };
}

pub unsafe extern "C" fn sudo_sig2str(
    signo: libc::c_int,
    signame: *mut libc::c_char,
) -> libc::c_int {
    return 0;
}

