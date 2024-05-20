/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    unused_imports,
    unused_attributes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(register_tool)]

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
}

use crate::common::*;
use stdext::function_name;

#[no_mangle]
unsafe extern "C" fn digest_type_to_name(mut digest_type: libc::c_int) -> *const libc::c_char {
    let mut digest_name: *const libc::c_char = 0 as *const libc::c_char;
    let SUDOERS_DEBUG_UTIL: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(17 as libc::c_int as isize)
        as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL);

    match digest_type {
        0 => {
            digest_name = b"sha224\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            digest_name = b"sha256\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            digest_name = b"sha384\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            digest_name = b"sha512\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            digest_name = b"unknown digest\0" as *const u8 as *const libc::c_char;
        }
    }

    debug_return_const_str!(digest_name);
}
