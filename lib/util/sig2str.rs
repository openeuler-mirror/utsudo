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

#[macro_export]
macro_rules! SIGRTMIN {
    () => {
        (__libc_current_sigrtmin())
    };
}

#[macro_export]
macro_rules! SIGRTMAX {
    () => {
        (__libc_current_sigrtmax())
    };
}

#[macro_export]
macro_rules! SIG2STR_MAX {
    () => {
        32
    };
}

#[macro_export]
macro_rules! NSIG {
    () => {
        (SIGRTMAX!() + 1)
    };
}

pub type size_t = libc::c_ulong;
pub const _SC_RTSIG_MAX: libc::c_int = 31;
pub type __int32_t = libc::c_int;

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn __libc_current_sigrtmin() -> libc::c_int;
    fn __libc_current_sigrtmax() -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}

pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    };
}

pub unsafe extern "C" fn sudo_sig2str(
    signo: libc::c_int,
    signame: *mut libc::c_char,
) -> libc::c_int {
    if signo >= SIGRTMIN!() && signo <= SIGRTMAX!() {
        let mut rtmax: libc::c_long = sysconf(_SC_RTSIG_MAX);
    }
    return 0;
}

