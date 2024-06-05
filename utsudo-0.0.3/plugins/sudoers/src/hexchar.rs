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

use crate::common::*;

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_debug_exit_int_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: libc::c_int,
    );
    fn sudo_debug_enter_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
}

/*
 * Converts a two-byte hex string to decimal.
 * Returns the decimal value or -1 for invalid input.
 */
#[no_mangle]
pub unsafe extern "C" fn hexchar(mut s: *const libc::c_char) -> libc::c_int {
    let mut result: [libc::c_uchar; 2] = [0; 2];
    let mut i: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        match *s.offset(i as isize) as u8 as char {
            '0' => {
                result[i as usize] = 0 as libc::c_uchar;
            }
            '1' => {
                result[i as usize] = 1 as libc::c_uchar;
            }
            '2' => {
                result[i as usize] = 2 as libc::c_uchar;
            }
            '3' => {
                result[i as usize] = 3 as libc::c_uchar;
            }
            '4' => {
                result[i as usize] = 4 as libc::c_uchar;
            }
            '5' => {
                result[i as usize] = 5 as libc::c_uchar;
            }
            '6' => {
                result[i as usize] = 6 as libc::c_uchar;
            }
            '7' => {
                result[i as usize] = 7 as libc::c_uchar;
            }
            '8' => {
                result[i as usize] = 8 as libc::c_uchar;
            }
            '9' => {
                result[i as usize] = 9 as libc::c_uchar;
            }
            'a' | 'A' => {
                result[i as usize] = 10 as libc::c_uchar;
            }
            'b' | 'B' => {
                result[i as usize] = 11 as libc::c_uchar;
            }
            'c' | 'C' => {
                result[i as usize] = 12 as libc::c_uchar;
            }
            'd' | 'D' => {
                result[i as usize] = 13 as libc::c_uchar;
            }
            'e' | 'E' => {
                result[i as usize] = 14 as libc::c_uchar;
            }
            'f' | 'F' => {
                result[i as usize] = 15 as libc::c_uchar;
            }
            _ => {
                /* Invalid input. */
                debug_return_int!(-1);
            }
        }
        i += 1;
    }
    debug_return_int!(
        (result[0 as usize] as libc::c_int) << 4 as libc::c_int | result[1 as usize] as libc::c_int
    );
}
