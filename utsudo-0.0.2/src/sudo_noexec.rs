/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(non_camel_case_types, non_snake_case, unused_mut)]

use crate::struct_macro::*;

//WRDE_NOCMD = (1 << 2),	/* Don't do command substitution.  */
pub const WRDE_NOCMD: libc::c_uint = 1 << 2;

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn execv(
    mut _a1: *const libc::c_char,
    mut _a2: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}