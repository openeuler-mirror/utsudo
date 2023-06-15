/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_variables, unused_assignments, clashing_extern_declarations)]
use crate::arc4random::__off_t;
use crate::sudo_conf::__ino_t;
use crate::INT_MAX;
use libc::dirfd;
use libc::DIR;
pub const _SC_OPEN_MAX: i32 = 4;

extern "C" {
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
}

