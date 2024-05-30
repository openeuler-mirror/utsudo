/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    clashing_extern_declarations,
    unused_variables,
    unused_imports,
    unused_attributes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::common::*;
//use crate::defaults::stat;
/*use crate::ut_common::debug_return_bool;
use crate::ut_common::sudo_debug_exit_bool_v1;
use crate::ut_common::sudo_debug_subsys;
use crate::ut_common::debug_decl;
use crate::ut_common::sudo_debug_enter_v1;
use crate::ut_common::debug_decl_vars;
*/
//use crate::defaults::timespec;

pub const EACCES: libc::c_int = 13;
pub const __S_IFREG: libc::c_int = 0o100000;
pub const __S_IFMT: libc::c_int = 0o170000;
pub const __S_IEXEC: libc::c_int = 0o100;

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn __errno_location() -> *mut libc::c_int;
    fn stat(filename: *const libc::c_char, buf: *mut stat) -> libc::c_int;
}

#[no_mangle]
unsafe extern "C" fn sudo_goodpath(mut path: *const libc::c_char, mut sbp: *mut stat) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let SUDOERS_DEBUG_UTIL: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(17 as libc::c_int as isize)
        as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL);

    if !path.is_null() {
        let mut sb: stat = sb_all_arch;

        if sbp.is_null() {
            sbp = &mut sb;
        }

        if stat(path, sbp) == 0 {
            if (*sbp).st_mode & __S_IFMT as libc::c_uint == __S_IFREG as libc::c_uint
                && (*sbp).st_mode
                    & (__S_IEXEC | __S_IEXEC >> 3 | __S_IEXEC >> 3 >> 3) as libc::c_uint
                    != 0
            {
                ret = 1 != 0;
            } else {
                *__errno_location() = EACCES as libc::c_int;
            }
        }
    }
    debug_return_bool!(ret as bool);
}
