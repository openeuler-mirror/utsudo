
/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

use crate::WEXITSTATUS;
use crate::WIFEXITED;
use crate::WIFSIGNALED;
use crate::WTERMSIG;

pub const DEAD_PROCESS: libc::c_short = 8;

extern "C" {
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getpid() -> __pid_t;
    fn setutxent();
    fn endutxent();
    fn getutxline(__line: *const utmpx) -> *mut utmpx;
    fn pututxline(__utmpx: *const utmpx) -> *mut utmpx;
    static mut user_details: user_details;
}
pub type __int32_t = libc::c_int;
pub type __suseconds_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
    pub ut_type: libc::c_short,
    pub ut_pid: __pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: __int32_t,
    pub ut_tv: C2RustUnnamed,
    pub ut_addr_v6: [__int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tv_sec: __int32_t,
    pub tv_usec: __int32_t,
}

pub type sudo_utmp_t = utmpx;








#[no_mangle]
pub unsafe extern "C" fn utmp_logout(
    mut line: *const libc::c_char,
    mut status: libc::c_int,
) -> bool {
    let mut ret: bool = false;
    let mut ut: *mut sudo_utmp_t = 0 as *mut sudo_utmp_t;
    let mut utbuf: sudo_utmp_t = sudo_utmp_t {
        ut_type: 0,
        ut_pid: 0,
        ut_line: [0; 32],
        ut_id: [0; 4],
        ut_user: [0; 32],
        ut_host: [0; 256],
        ut_exit: __exit_status {
            e_termination: 0,
            e_exit: 0,
        },
        ut_session: 0,
        ut_tv: C2RustUnnamed {
            tv_sec: 0,
            tv_usec: 0,
        },
        ut_addr_v6: [0; 4],
        __glibc_reserved: [0; 20],
    };

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTMP);

    /* Strip off /dev/ prefix from line as needed. */
    if strncmp(
        line,
        b"/dev/\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1),
    ) == 0
    {
        line = line.offset(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1) as isize,
        );
    }

    memset(
        &mut utbuf as *mut sudo_utmp_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<sudo_utmp_t>() as libc::c_ulong,
    );
    strncpy(
        (utbuf.ut_line).as_mut_ptr(),
        line,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    ut = getutxline(&mut utbuf);
    if !ut.is_null() {
        memset(
            ((*ut).ut_user).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
        (*ut).ut_type = DEAD_PROCESS;

        (*ut).ut_exit.e_termination = (if WIFSIGNALED!(status) {
            WTERMSIG!(status)
        } else {
            0
        }) as libc::c_short;

        (*ut).ut_exit.e_exit = (if WIFEXITED!(status) {
            WEXITSTATUS!(status)
        } else {
            0
        }) as libc::c_short;

        utmp_settime(ut);
        if !(pututxline(ut)).is_null() {
            ret = true;
        }
    }
    debug_return_bool!(ret)
}
