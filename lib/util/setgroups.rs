/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */


extern "C" {
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn setgroups(__n: size_t, __groups: *const gid_t) -> libc::c_int;
}

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

pub type gid_t = libc::c_uint;
pub type size_t = libc::c_ulong;

pub const EINVAL: libc::c_int = 22;
pub const _SC_NGROUPS_MAX: libc::c_int = 3;
pub const NGROUPS_MAX: libc::c_int = 65536;

unsafe extern "C" fn sudo_setgroups_v1(
    mut ngids: libc::c_int,
    mut gids: *const gid_t,
) -> libc::c_int {
    let mut maxgids: libc::c_int = 0;
    let mut ret: libc::c_int = 0;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
    ret = setgroups(ngids as size_t, gids as *mut gid_t);
    
    if *__errno_location() == EINVAL as libc::c_int {
    //max
    }
}
