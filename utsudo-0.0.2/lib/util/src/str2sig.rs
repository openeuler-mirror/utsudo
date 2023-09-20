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

use crate::EINVAL;
use crate::SIGRTMAX;
use crate::SIGRTMIN;

extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn strncmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_int;
    fn __libc_current_sigrtmin() -> libc::c_int;
    fn __libc_current_sigrtmax() -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}

pub struct sigalias {
    pub name: *const libc::c_char,
    pub number: libc::c_int,
}
static mut sigaliases: [sigalias; 6] = [
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
];

pub const SIGABRT: libc::c_int = 6;
pub const SIGCLD: libc::c_int = 17;
pub const SIGIO: libc::c_int = 29;
pub const SIGIOT: libc::c_int = 6;
pub const SIGPOLL: libc::c_int = 29;
pub const _ISdigit: libc::c_uint = 2048;
pub const __SIGRTMIN: libc::c_uint = 64;
pub const NSIG: libc::c_uint = __SIGRTMIN + 1;
pub const _SC_RTSIG_MAX: libc::c_int = 31;

#[no_mangle]
pub unsafe extern "C" fn sudo_str2sig(
    mut signame: *const libc::c_char,
    mut result: *mut libc::c_int,
) -> libc::c_int {
    let mut alias: *mut sigalias = 0 as *mut sigalias;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut signo: libc::c_int = 0;

    static mut sudo_sys_signame: [*mut libc::c_char; 65] =
        [0 as *const libc::c_char as *mut libc::c_char; 65];
    let mut i: libc::c_int = 0;
    sudo_sys_signame[0 as libc::c_int as usize] =
        b"Signal 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if (sudo_sys_signame[1 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[1 as libc::c_int as usize] =
            b"HUP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[2 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[2 as libc::c_int as usize] =
            b"INT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[3 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[3 as libc::c_int as usize] =
            b"QUIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[4 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[4 as libc::c_int as usize] =
            b"ILL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[5 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[5 as libc::c_int as usize] =
            b"TRAP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[6 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[6 as libc::c_int as usize] =
            b"ABRT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[6 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[6 as libc::c_int as usize] =
            b"IOT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[8 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[8 as libc::c_int as usize] =
            b"FPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[9 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[9 as libc::c_int as usize] =
            b"KILL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[7 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[7 as libc::c_int as usize] =
            b"BUS\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[11 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[11 as libc::c_int as usize] =
            b"SEGV\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[31 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[31 as libc::c_int as usize] =
            b"SYS\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[13 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[13 as libc::c_int as usize] =
            b"PIPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[14 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[14 as libc::c_int as usize] =
            b"ALRM\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[15 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[15 as libc::c_int as usize] =
            b"TERM\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[16 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[16 as libc::c_int as usize] =
            b"STKFLT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[29 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[29 as libc::c_int as usize] =
            b"IO\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[24 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[24 as libc::c_int as usize] =
            b"XCPU\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[25 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[25 as libc::c_int as usize] =
            b"XFSZ\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[26 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[26 as libc::c_int as usize] =
            b"VTALRM\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[27 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[27 as libc::c_int as usize] =
            b"PROF\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[28 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[28 as libc::c_int as usize] =
            b"WINCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[10 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[10 as libc::c_int as usize] =
            b"USR1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[12 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[12 as libc::c_int as usize] =
            b"USR2\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[30 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[30 as libc::c_int as usize] =
            b"PWR\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[29 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[29 as libc::c_int as usize] =
            b"POLL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[19 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[19 as libc::c_int as usize] =
            b"STOP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[20 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[20 as libc::c_int as usize] =
            b"TSTP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[18 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[18 as libc::c_int as usize] =
            b"CONT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[17 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[17 as libc::c_int as usize] =
            b"CHLD\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[17 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[17 as libc::c_int as usize] =
            b"CLD\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[21 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[21 as libc::c_int as usize] =
            b"TTIN\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[22 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[22 as libc::c_int as usize] =
            b"TTOU\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (sudo_sys_signame[23 as libc::c_int as usize]).is_null() {
        sudo_sys_signame[23 as libc::c_int as usize] =
            b"URG\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }

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
        if *signame.offset(5 as isize) as libc::c_int == '\u{0}' as i32 {
            *result = SIGRTMAX!();
            return 0;
        }
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
        if strcmp(signame, (*alias).name) == 0 {
            *result = (*alias).number;
            return 0;
        }
        alias = alias.offset(1);
    }

    signo = 1;
    while signo < NSIG as libc::c_int {
        if !(sudo_sys_signame[signo as usize]).is_null() {
            if strcasecmp(signame, sudo_sys_signame[signo as usize]) == 0 {
                *result = signo;
                return 0;
            }
        }
        signo += 1;
    }
    *__errno_location() = EINVAL!();
    return -1;
}
