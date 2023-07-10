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
}        

pub const __SIGRTMIN: libc::c_uint = 64;
pub const NSIG: libc::c_uint = __SIGRTMIN + 1;

;#[no_mangle]
pub unsafe extern "C" fn sudo_str2sig(
    mut signame: *const libc::c_char,
    mut result: *mut libc::c_int,
) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut signo: libc::c_int = 0;

    if (*(*__ctype_b_loc()).offset(*signame.offset(0 as isize) as libc::c_uchar as isize)
        as libc::c_int)
        != 0
    {
        signo = sudo_strtonum(
            signame,
            0 as libc::c_longlong,
            (NSIG - 1) as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        
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
                )
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
    return 0;
}
