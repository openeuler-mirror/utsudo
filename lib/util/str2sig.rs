/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static sys_sigabbrev: [*const libc::c_char; 65];
    fn __errno_location() -> *mut libc::c_int;
}        

pub struct sigalias {
    pub name: *const libc::c_char,
    pub number: libc::c_int,
}

static mut sigaliases: [sigalias; 2] = [
    {
        let mut sigabrt = sigalias {
            name: b"ABRT\0" as *const u8 as *const libc::c_char,
            number: SIGABRT,
        };
        sigabrt
    },
    {
        let mut sigcld = sigalias {
            name: b"CLD\0" as *const u8 as *const libc::c_char,
            number: SIGCLD,
        };
        sigcld
    },
    {
        let mut sigio = sigalias {
            name: b"IO\0" as *const u8 as *const libc::c_char,
            number: SIGIO,
        };
        sigio
    },
    {
        let mut sigiot = sigalias {
            name: b"IOT\0" as *const u8 as *const libc::c_char,
            number: SIGIOT,
        };
        sigiot
    },
    {
        let mut sigpoll = sigalias {
            name: b"POLL\0" as *const u8 as *const libc::c_char,
            number: SIGPOLL,
        };
        sigpoll
    },
    {
        let mut init = sigalias {
            name: 0 as *const libc::c_char,
            number: -1,
        };
        init
    },
]

pub const SIGABRT: libc::c_int = 6;
pub const SIGCLD: libc::c_int = 17;
pub const SIGIO: libc::c_int = 29;
pub const SIGIOT: libc::c_int = 6;
pub const SIGPOLL: libc::c_int = 29;
pub const _ISdigit: libc::c_uint = 2048;
pub const __SIGRTMIN: libc::c_uint = 64;
pub const NSIG: libc::c_uint = __SIGRTMIN + 1;
pub const _SC_RTSIG_MAX: libc::c_int = 31;

;#[no_mangle]
pub unsafe extern "C" fn sudo_str2sig(
    mut signame: *const libc::c_char,
    mut result: *mut libc::c_int,
) -> libc::c_int {
    let mut alias: *mut sigalias = 0 as *mut sigalias;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut signo: libc::c_int = 0;

    if (*(*__ctype_b_loc()).offset(*signame.offset(0 as isize) as libc::c_uchar as isize)
        as libc::c_int
        & _ISdigit as libc::c_ushort as libc::c_int)
        != 0
    {
        signo = sudo_strtonum(
            signame,
            0 as libc::c_longlong,
            (NSIG - 1) as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        
       if !errstr.is_null() {
            return -1;
        }
        *result = signo;
        return 0;
    }
    if strncmp(
        signame,
        b"RTMIN\0" as *const u8 as *const libc::c_char,
        5 as libc::c_ulong,
    ) == 0
    {
        if *signame.offset(5 as isize) as libc::c_int == '\u{0}' as i32 {
            *result = SIGRTMIN!();
            return 0;
        }
        if *signame.offset(5 as isize) as libc::c_int == '+' as i32 {
            if (*(*__ctype_b_loc()).offset(*signame.offset(6 as isize) as libc::c_uchar as isize)
                as libc::c_int
                & _ISdigit as libc::c_ushort as libc::c_int)
                != 0
            {
                let rtmax: libc::c_long = sysconf(_SC_RTSIG_MAX);
                let off: libc::c_int = *signame.offset(6 as isize) as libc::c_int - '0' as i32;

                if rtmax > 0 as libc::c_long && (off as libc::c_long) < (rtmax / 2 as libc::c_long)
                {
                    *result = SIGRTMIN!() + off;
                    return 0;
                }
            }
        }
    }
    if strncmp(
        signame,
        b"RTMAX\0" as *const u8 as *const libc::c_char,
        5 as libc::c_ulong,
    ) == 0
    {
        if *signame.offset(5 as isize) as libc::c_int == '-' as i32 {
            if (*(*__ctype_b_loc()).offset(*signame.offset(6 as isize) as libc::c_uchar as isize)
                as libc::c_int
                & _ISdigit as libc::c_ushort as libc::c_int)
                != 0
            {
                let rtmax: libc::c_long = sysconf(_SC_RTSIG_MAX);
                let off: libc::c_int = *signame.offset(6 as isize) as libc::c_int - '0' as i32;
                if rtmax > 0 as libc::c_long && (off as libc::c_long) < (rtmax / 2 as libc::c_long)
                {
                    *result = SIGRTMAX!() - off;
                    return 0;
                }
            }
        }
    }

    alias = sigaliases.as_mut_ptr();
    while !((*alias).name).is_null() {
        alias = alias.offset(1);
    }

    signo = 1;
    while signo < NSIG as libc::c_int {
        if !(sys_sigabbrev[signo as usize]).is_null() {
            if strcasecmp(signame, sys_sigabbrev[signo as usize]) == 0 {
                *result = signo;
                return 0;
            }
        }
        signo += 1;
    }
    *__errno_location() = EINVAL!();
    return -1;
}
