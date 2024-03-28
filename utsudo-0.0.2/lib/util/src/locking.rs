/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(dead_code, non_camel_case_types, unused_mut, unused_assignments)]

use crate::macro_struct::*;
use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

extern "C" {
    fn lockf(__fd: libc::c_int, __cmd: libc::c_int, __len: off_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;

}

// #define SUDO_LOCK	1		/* lock a file */
// #define SUDO_TLOCK	2		/* test & lock a file (non-blocking) */
// #define SUDO_UNLOCK	4		/* unlock a file */
pub const SUDO_LOCK: libc::c_int = 1;
pub const SUDO_TLOCK: libc::c_int = 2;
pub const SUDO_UNLOCK: libc::c_int = 4;

// # define F_ULOCK 0	/* Unlock a previously locked region.  */
// # define F_LOCK  1	/* Lock a region for exclusive use.  */
// # define F_TLOCK 2	/* Test and lock a region for exclusive use.  */
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
    let mut op: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    match type_0 {
        SUDO_LOCK => {
            op = F_LOCK;
        }
        SUDO_TLOCK => {
            op = F_TLOCK;
        }
        SUDO_UNLOCK => {
            op = F_ULOCK;
        }
        _ => {
            *__errno_location() = EINVAL;
            debug_return_bool!(false);
        }
    }
    debug_return_bool!(lockf(fd, op, len) == 0)
}
