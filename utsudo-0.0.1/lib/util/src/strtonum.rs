/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}

pub const _ISspace: libc::c_uint = 8192;
pub const _ISdigit: libc::c_uint = 2048;
pub const EINVAL: libc::c_int = 22;
pub const ERANGE: libc::c_int = 34;

#[derive(Eq, PartialEq)]
enum strtonum_err {
    STN_INITIAL = 0,
    STN_VALID = 1,
    STN_INVALID = 2,
    STN_TOOSMALL = 3,
    STN_TOOBIG = 4,
}

use crate::macro_struct::*;

#[no_mangle]
pub unsafe extern "C" fn sudo_strtonumx(
    mut str: *const libc::c_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut endp: *mut *mut libc::c_char,
    mut errstrp: *mut *const libc::c_char,
) -> libc::c_longlong {
    let mut errval = strtonum_err::STN_INITIAL;
    let mut lastval: libc::c_longlong = 0;
    let mut result: libc::c_longlong = 0 as libc::c_longlong;
    let mut cp: *const libc::c_char = str;
    let mut ch: libc::c_uchar = 0;
    let mut remainder: libc::c_int = 0;
    let mut sign: libc::c_char = 0;

    if minval > maxval {
        errval = strtonum_err::STN_INVALID;
    } else {
        loop {
            let fresh0 = cp;
            cp = cp.offset(1);
            ch = *fresh0 as libc::c_uchar;
            if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISspace as libc::c_ushort as libc::c_int
                != 0)
            {
                break;
            }
        }

        match ch as u8 as char {
            '-' => {
                sign = '-' as i32 as libc::c_char;
                let fresh1 = cp;
                cp = cp.offset(1);
                ch = *fresh1 as libc::c_uchar;
            }
            '+' => {
                let fresh2 = cp;
                cp = cp.offset(1);
                ch = *fresh2 as libc::c_uchar;
                sign = '+' as i32 as libc::c_char;
            }
            _ => {
                sign = '+' as i32 as libc::c_char;
            }
        }

        if sign as libc::c_int == '-' as i32 {
            lastval = minval / 10 as libc::c_longlong;
            remainder = -(minval % 10 as libc::c_longlong) as libc::c_int;
            if remainder < 0 {
                lastval += 1 as libc::c_longlong;
                remainder += 10;
            }
            while !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_ushort as libc::c_int
                == 0)
            {
                ch = (ch as libc::c_int - '0' as i32) as libc::c_uchar;
                if result < lastval || result == lastval && ch as libc::c_int > remainder {
                    loop {
                        let fresh3 = cp;
                        cp = cp.offset(1);
                        ch = *fresh3 as libc::c_uchar;
                        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                            & _ISdigit as libc::c_ushort as libc::c_int
                            != 0)
                        {
                            break;
                        }
                    }
                    errval = strtonum_err::STN_TOOSMALL;
                    break;
                } else {
                    result *= 10 as libc::c_longlong;
                    result -= ch as libc::c_longlong;
                    errval = strtonum_err::STN_VALID;
                    let fresh4 = cp;
                    cp = cp.offset(1);
                    ch = *fresh4 as libc::c_uchar;
                }
            }
            if result > maxval {
                errval = strtonum_err::STN_TOOBIG;
            }
        } else {
            lastval = maxval / 10 as libc::c_longlong;
            remainder = (maxval % 10 as libc::c_longlong) as libc::c_int;
            while !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_ushort as libc::c_int
                == 0)
            {
                ch = (ch as libc::c_int - '0' as i32) as libc::c_uchar;
                if result > lastval || result == lastval && ch as libc::c_int > remainder {
                    loop {
                        let fresh5 = cp;
                        cp = cp.offset(1);
                        ch = *fresh5 as libc::c_uchar;
                        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                            & _ISdigit as libc::c_ushort as libc::c_int
                            != 0)
                        {
                            break;
                        }
                    }
                    errval = strtonum_err::STN_TOOBIG;
                    break;
                } else {
                    result *= 10 as libc::c_longlong;
                    result += ch as libc::c_longlong;
                    errval = strtonum_err::STN_VALID;
                    let fresh6 = cp;
                    cp = cp.offset(1);
                    ch = *fresh6 as libc::c_uchar;
                }
            }
            if result < minval {
                errval = strtonum_err::STN_TOOSMALL;
            }
        }
    }

    match errval {
        strtonum_err::STN_INITIAL | strtonum_err::STN_VALID => {
            if !errstrp.is_null() {
                *errstrp = 0 as *const libc::c_char;
            }
        }
        strtonum_err::STN_INVALID => {
            result = 0 as libc::c_longlong;
            *__errno_location() = EINVAL;
            if !errstrp.is_null() {
                *errstrp = b"invalid value\0" as *const u8 as *const libc::c_char;
            }
        }
        strtonum_err::STN_TOOSMALL => {
            result = 0 as libc::c_longlong;
            *__errno_location() = ERANGE;
            if !errstrp.is_null() {
                *errstrp = b"value too small\0" as *const u8 as *const libc::c_char;
            }
        }
        strtonum_err::STN_TOOBIG => {
            result = 0 as libc::c_longlong;
            *__errno_location() = ERANGE;
            if !errstrp.is_null() {
                *errstrp = b"value too large\0" as *const u8 as *const libc::c_char;
            }
        }
    }

    if !endp.is_null() {
        if errval == strtonum_err::STN_INITIAL || errval == strtonum_err::STN_INVALID {
            *endp = str as *mut libc::c_char;
        } else {
            *endp = cp.offset(-1 as isize) as *mut libc::c_char;
        }
    }
    return result;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_strtonum(
    mut str: *const libc::c_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut errstrp: *mut *const libc::c_char,
) -> libc::c_longlong {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_longlong = 0;
    ret = sudo_strtonumx(str, minval, maxval, &mut ep, &mut errstr);

    if str == ep || *ep as libc::c_int != '\u{0}' as i32 {
        *__errno_location() = EINVAL;
        errstr = b"invalid value\0" as *const u8 as *const libc::c_char;
        ret = 0 as libc::c_longlong;
    }

    if !errstrp.is_null() {
        *errstrp = errstr;
    }
    return ret;
}
