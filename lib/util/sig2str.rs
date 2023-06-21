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
    fn sysconf(__name: libc::c_int)
    fn snprintf()
    fn sudo_strlcpy()
    fn __ctype_toupper_loc()
    fn __ctype_b_loc()
}

pub unsafe extern "C" fn toupper()

pub unsafe extern "C" fn sudo_sig2str()

