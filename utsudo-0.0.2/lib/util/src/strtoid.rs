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

type id_t = u32;
use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_id_t_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;
use crate::INT_MAX;

// #define INT_MIN (-__INT_MAX__ - 1)
#[macro_export]
macro_rules! INT_MIN {
    () => {
        (-(INT_MAX!()) - 1)
    };
}

// #define UINT_MAX (__INT_MAX__ * 2U + 1U)
#[macro_export]
macro_rules! UINT_MAX {
    () => {
        ((INT_MAX!()) * (2 as libc::c_uint) + 1 as libc::c_uint)
    };
}

// #define	EINVAL		22	/* Invalid argument */
#[macro_export]
macro_rules! EINVAL {
    () => {
        22
    };
}

extern "C" {
    fn sudo_strtonumx(
        str: *const libc::c_char,
        minval: libc::c_longlong,
        maxval: libc::c_longlong,
        endp: *mut *mut libc::c_char,
        errstrp: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn __errno_location() -> *mut libc::c_int;
}

/*
 * Make sure that the ID ends with a valid separator char.
 */
unsafe extern "C" fn valid_separator(
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
    mut sep: *const libc::c_char,
) -> bool {
    let mut valid: bool = false;
    if ep != p {
        /* check for valid separator (including '\0') */
        if sep.is_null() {
            sep = b"\0" as *const u8 as *const libc::c_char;
        }
        loop {
            if *ep == *sep {
                valid = true;
            }
            if !(*sep as libc::c_int != '\u{0}' as i32) {
                break;
            }
            sep = sep.offset(1);
        }
    } // !eq != p
    return valid;
}

/*
 * Parse a uid/gid in string form.
 * If sep is non-NULL, it contains valid separator characters (e.g. comma, space)
 * If endp is non-NULL it is set to the next char after the ID.
 * On success, returns the parsed ID and clears errstr.
 * On error, returns 0 and sets errstr.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_strtoidx_v1(
    mut p: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
    mut errstrp: *mut *const libc::c_char,
) -> id_t {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: id_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
    ret = sudo_strtonumx(
        p,
        INT_MIN!() as libc::c_longlong,
        UINT_MAX!() as libc::c_longlong,
        &mut ep,
        &mut errstr,
    ) as id_t;
    if errstr.is_null() {
        /*
         * Disallow id -1 (UINT_MAX), which means "no change"
         * and check for a valid separator (if specified).
         */
        if ret == -(1 as libc::c_int) as id_t
            || ret == UINT_MAX!() as id_t
            || !valid_separator(p, ep, sep)
        {
            errstr = b"invalid value\0" as *const u8 as *const libc::c_char;
            *__errno_location() = EINVAL!() as libc::c_int;
            ret = 0;
        }
    }
    if !errstrp.is_null() {
        *errstrp = errstr;
    }
    if !endp.is_null() {
        *endp = ep;
    }
    debug_return_id_t!(ret)
}

/* Backwards compatibility */
#[no_mangle]
pub unsafe extern "C" fn sudo_strtoid_v1(
    mut p: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
    mut errstrp: *mut *const libc::c_char,
) -> id_t {
    return sudo_strtoidx_v1(p, sep, endp, errstrp);
}

/* Simplified interface */
#[no_mangle]
pub unsafe extern "C" fn sudo_strtoid_v2(
    mut p: *const libc::c_char,
    mut errstrp: *mut *const libc::c_char,
) -> id_t {
    return sudo_strtoidx_v1(
        p,
        0 as *const libc::c_char,
        0 as *mut *mut libc::c_char,
        errstrp,
    );
}
