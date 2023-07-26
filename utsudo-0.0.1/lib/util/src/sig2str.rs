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

use crate::EINVAL;

// #define SIGRTMIN        (__libc_current_sigrtmin ())
#[macro_export]
macro_rules! SIGRTMIN {
    () => {
        (__libc_current_sigrtmin())
    };
}

// #define SIGRTMAX        (__libc_current_sigrtmax ())
#[macro_export]
macro_rules! SIGRTMAX {
    () => {
        (__libc_current_sigrtmax())
    };
}

// # define SIG2STR_MAX 32
#[macro_export]
macro_rules! SIG2STR_MAX {
    () => {
        32
    };
}

/* Biggest signal number + 1 (including real-time signals).  */
// #define _NSIG		(__SIGRTMAX + 1)
#[macro_export]
macro_rules! NSIG {
    () => {
        (SIGRTMAX!() + 1)
    };
}

/* sys_sigabbrev is not declared by glibc */
//   #  define sudo_sys_signame	sys_sigabbrev
#[macro_export]
macro_rules! sudo_sys_signame {
    () => {
        sys_sigabbrev
    };
}

pub type size_t = libc::c_ulong;
pub const _SC_RTSIG_MAX: libc::c_int = 31;
pub type __int32_t = libc::c_int;
pub const _ISlower: libc::c_int = 512;

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
    static sys_sigabbrev: [*const libc::c_char; 65];
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}

#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}

/*
 * Translate signal number to name.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_sig2str(
    signo: libc::c_int,
    signame: *mut libc::c_char,
) -> libc::c_int {
    /* Realtime signal support. */
    if signo >= SIGRTMIN!() && signo <= SIGRTMAX!() {
        let mut rtmax: libc::c_long = sysconf(_SC_RTSIG_MAX);
        if rtmax > 0 {
            if signo == SIGRTMIN!() {
                sudo_strlcpy(
                    signame,
                    b"RTMIN\0" as *const u8 as *const libc::c_char,
                    SIG2STR_MAX!(),
                );
            } else if signo == SIGRTMAX!() {
                sudo_strlcpy(
                    signame,
                    b"RTMAX\0" as *const u8 as *const libc::c_char,
                    SIG2STR_MAX!(),
                );
            } else if signo as libc::c_long
                <= (SIGRTMIN!() as libc::c_long + (rtmax / 2) - 1) as libc::c_long
            {
                snprintf(
                    signame,
                    SIG2STR_MAX!(),
                    b"RTMIN+%d\0" as *const u8 as *const libc::c_char,
                    signo - SIGRTMIN!(),
                );
            } else {
                snprintf(
                    signame,
                    SIG2STR_MAX!(),
                    b"RTMAX-%d\0" as *const u8 as *const libc::c_char,
                    SIGRTMAX!() - signo,
                );
            }
        }
        return 0;
    }

    if signo > 0 && signo < NSIG!() && !sudo_sys_signame!()[signo as usize].is_null() {
        sudo_strlcpy(signame, sudo_sys_signame!()[signo as usize], SIG2STR_MAX!());

        /* Make sure we always return an upper case signame. */
        if *(*__ctype_b_loc()).offset(*signame.offset(0 as isize) as isize) as libc::c_int
            & _ISlower
            != 0
        {
            let mut i: libc::c_int = 0;
            while *signame.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
                *signame.offset(i as isize) =
                    toupper((*signame.offset(i as isize)).into()) as libc::c_char;
                i += 1;
            }
        }
        return 0 as libc::c_int;
    }
    *__errno_location() = EINVAL!();
    return -(1 as libc::c_int);
}
