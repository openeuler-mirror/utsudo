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

#[no_mangle]
pub unsafe extern "C" fn sudo_strtomode_v1(
    mut cp: *const libc::c_char,
    mut errstr: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lval: libc::c_long = 0;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    *__errno_location() = 0 as libc::c_int;
    lval = strtol(cp, &mut ep, 8 as libc::c_int);
   debug_return_int!(0)
}

