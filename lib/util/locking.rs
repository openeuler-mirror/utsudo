/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(dead_code, non_camel_case_types, unused_mut)]

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

extern "C" {
    fn lockf(__fd: libc::c_int, __cmd: libc::c_int, __len: off_t) -> libc::c_int;

}

pub type __off_t = libc::c_long;
pub type off_t = __off_t;

pub const SUDO_LOCK: libc::c_int = 1;
pub const SUDO_TLOCK: libc::c_int = 2;
pub const SUDO_UNLOCK: libc::c_int = 4;
pub const F_ULOCK: libc::c_int = 0;
pub const F_LOCK: libc::c_int = 1;
pub const F_TLOCK: libc::c_int = 2;

#[no_mangle]
unsafe extern "C" fn sudo_lock_file_v1(mut fd: libc::c_int, mut type_0: libc::c_int) -> bool {
    return sudo_lock_region_v1(fd, type_0, 0 as off_t);
}

#[no_mangle]
unsafe extern "C" fn sudo_lock_region_v1(
    mut fd: libc::c_int,
    mut type_0: libc::c_int,
    mut len: off_t,
) -> bool {

    match type_0 {
        SUDO_LOCK => {
        }
        SUDO_TLOCK => {
        }
        SUDO_UNLOCK => {
        }
    }
