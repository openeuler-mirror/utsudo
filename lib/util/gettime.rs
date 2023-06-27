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

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
// use crate::sudo_debug::sudo_debug_printf2_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_ERRNO;
use crate::sudo_debug_macro::SUDO_DEBUG_LINENO;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;
use crate::sudo_debug_macro::SUDO_DEBUG_WARN;

extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}

pub type __timezone_ptr_t = *mut timezone;

pub const CLOCK_REALTIME: libc::c_int = 0;
pub const SUDO_CLOCK_BOOTTIME: libc::c_int = 7;
pub const SUDO_CLOCK_UPTIME: libc::c_int = 1;
pub const _SC_MONOTONIC_CLOCK: libc::c_int = 149;

#[no_mangle]
pub unsafe extern "C" fn sudo_gettime_real_v1(mut ts: *mut timespec) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if clock_gettime(CLOCK_REALTIME, ts) == -1 {
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_ERRNO | SUDO_DEBUG_LINENO,
            b"clock_gettime(CLOCK_REALTIME) failed, trying gettimeofday()\0" as *const u8
                as *const libc::c_char
        );

        if gettimeofday(&mut tv, 0 as *mut timezone) == -1 {
            debug_return_int!(-1);
        }
        (*ts).tv_sec = tv.tv_sec;
        (*ts).tv_nsec = tv.tv_usec * 1000;
    }
    
    debug_return_int!(0)
}

#[no_mangle]
pub unsafe extern "C" fn sudo_gettime_mono_v1(ts: *mut timespec) -> i32 {
    debug_return_int!(0)
}