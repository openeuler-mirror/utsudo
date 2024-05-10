/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    unused_variables,
    non_camel_case_types,
    unused_variables,
    unused_assignments,
    unused_mut
)]

use crate::macro_struct::*;
use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

extern "C" {
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn setgroups(__n: size_t, __groups: *const gid_t) -> libc::c_int;
}

//func sudo_setgroups_v1
#[no_mangle]
unsafe extern "C" fn sudo_setgroups_v1(
    mut ngids: libc::c_int,
    mut gids: *const gid_t,
) -> libc::c_int {
    let mut maxgids: libc::c_int = 0;
    let mut ret: libc::c_int = 0;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    //line43
    ret = setgroups(ngids as size_t, gids as *mut gid_t);

    //line44
    if ret == -(1 as libc::c_int) && *__errno_location() == EINVAL as libc::c_int {
        //line47
        maxgids = sysconf(_SC_NGROUPS_MAX as libc::c_int) as libc::c_int;
        //line48
        if maxgids == -(1 as libc::c_int) {
            maxgids = NGROUPS_MAX as libc::c_int;
        }
        //line50
        if ngids > maxgids {
            ret = setgroups(maxgids as size_t, gids as *mut gid_t);
        }
    }
    debug_return_int!(ret)
}
