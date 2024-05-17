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
    unused_mut,
    unreachable_code,
    non_snake_case
)]

use crate::common::*;

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
}

pub type size_t = libc::c_ulong;

/*
 * Derived from code with the following declaration:
 *	PUBLIC DOMAIN - Jon Mayo - November 13, 2003
 */
static mut base64dec_tab: [libc::c_uchar; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 62, 255, 255, 255, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 255,
    255, 255, 0, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255, 255, 255, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

/*
 * Decode a NUL-terminated string in base64 format and store the
 * result in dst.
 */
#[no_mangle]
pub unsafe extern "C" fn base64_decode(
    mut in0: *const libc::c_char,
    mut out: *mut libc::c_uchar,
    mut out_size: size_t,
) -> size_t {
    let mut out_end: *mut libc::c_uchar = out.offset(out_size as isize);
    let mut out0: *const libc::c_uchar = out;
    let mut rem: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;
    debug_decl!(SUDOERS_DEBUG_MATCH!());

    //for (v = 0, rem = 0; *in != '\0' && *in != '='; in++)
    loop {
        if !(*in0 as libc::c_int != '\0' as i32 && *in0 as libc::c_int != '=' as i32) {
            break;
        }

        let mut ch: libc::c_uchar = base64dec_tab[*in0 as libc::c_uchar as usize];
        if ch == 255 {
            debug_return_size_t!(-(1 as libc::c_int) as size_t);
        }
        v = v << 6 | ch as libc::c_uint;
        rem = rem.wrapping_add(6);
        if rem >= 8 {
            rem = rem.wrapping_sub(8);
            if out >= out_end {
                debug_return_size_t!(-(1 as libc::c_int) as size_t);
            }
            out = out.offset(1);
            *out = (v >> rem & 0xff as libc::c_uint) as libc::c_uchar;
        }
        in0 = in0.offset(1);
    }
    if rem >= 8 {
        if out >= out_end {
            debug_return_size_t!(-(1 as libc::c_int) as size_t);
        }
        out = out.offset(1);
        *out = (v >> rem & 0xff as libc::c_uint) as libc::c_uchar;
    }
    debug_return_size_t!(out.offset_from(out0) as libc::c_long as size_t);
}

static mut base64enc_tab: [libc::c_uchar; 64] = unsafe {
    *::core::mem::transmute::<&[u8; 64], &[libc::c_uchar; 64]>(
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    )
};

#[no_mangle]
pub unsafe extern "C" fn base64_encode(
    mut in0: *const libc::c_uchar,
    mut in_len: size_t,
    mut out: *mut libc::c_char,
    mut out_len: size_t,
) -> size_t {
    let mut ii: size_t = 0;
    let mut io: size_t = 0;
    let mut rem: libc::c_uint = 0;
    let mut v: libc::c_uint = 0;

    debug_decl!(SUDOERS_DEBUG_MATCH!());

    //for (io = 0, ii = 0, v = 0, rem = 0; ii < in_len; ii++)
    loop {
        if !(ii < in_len) {
            break;
        }
        let mut ch: libc::c_uchar = *in0.offset(ii as isize);
        v = v << 8 | ch as libc::c_uint;
        rem = rem.wrapping_add(8 as libc::c_uint);
        while rem >= 6 {
            rem = rem.wrapping_sub(6 as libc::c_uint);
            if io >= out_len {
                debug_return_size_t!(-(1 as libc::c_int) as size_t); /* truncation is failure */
            }
            io = io.wrapping_add(1);
            *out.offset(io as isize) =
                base64enc_tab[(v >> rem & 63 as libc::c_uint) as usize] as libc::c_char;
        }
        ii = ii.wrapping_add(1);
    }
    if rem != 0 {
        v <<= (6 as libc::c_uint).wrapping_sub(rem);
        if io >= out_len {
            debug_return_size_t!(-(1 as libc::c_int) as size_t); /* truncation is failure */
        }
        io = io.wrapping_add(1);
        *out.offset(io as isize) = base64enc_tab[(v & 63 as libc::c_uint) as usize] as libc::c_char;
    }
    while io & 3 as libc::c_ulong != 0 {
        if io >= out_len {
            debug_return_size_t!(-(1 as libc::c_int) as size_t); /* truncation is failure */
        }
        io = io.wrapping_add(1);
        *out.offset(io as isize) = '=' as i32 as libc::c_char;
    }
    if io >= out_len {
        debug_return_size_t!(-(1 as libc::c_int) as size_t); /* no room for NUL terminator */
    }
    *out.offset(io as isize) = '\0' as i32 as libc::c_char;
    debug_return_size_t!(io as size_t);
}
