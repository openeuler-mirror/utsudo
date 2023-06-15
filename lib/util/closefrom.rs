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

#[no_mangle]
fn closefrom_fallback(lowfd: libc::c_int) {
    let lowfd: libc::c_int = 0;
    let mut _fd: libc::c_long = 0;
    let mut maxfd: libc::c_long;

    #[macro_export]
    macro_rules! _POSIX_OPEN_MAX {
        () => {
            20
        };
    }

    maxfd = unsafe { sysconf(_SC_OPEN_MAX) };

    if maxfd < 0 {
        maxfd = _POSIX_OPEN_MAX!();
    } else {
        let mut fd = lowfd;
        while i64::from(fd) < maxfd {
            unsafe { close(fd) };
            fd += 1;
        }
    }
}

