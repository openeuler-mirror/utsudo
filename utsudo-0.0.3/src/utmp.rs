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

/*
 * Create ut_id from the new ut_line and the old ut_id.
 */
pub unsafe extern "C" fn utmp_setid(old: *mut sudo_utmp_t, new: *mut sudo_utmp_t) {
    let mut line: *mut libc::c_char = ((*new).ut_line).as_mut_ptr();
    let mut idlen: size_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTMP);

    /* Skip over "tty" in the id if old entry did too. */
    if old.is_null() {
        /* cppcheck-suppress uninitdata */
        if strncmp(line, b"tty\0" as *const u8 as *const libc::c_char, 3) == 0 {
            idlen = std::cmp::min(
                ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
                3,
            );
            if strncmp(
                ((*old).ut_id).as_mut_ptr(),
                b"tty\0" as *const u8 as *const libc::c_char,
                idlen,
            ) != 0
            {
                line = line.offset(3 as libc::c_int as isize);
            }
        }
    }

    /* Store as much as will fit, skipping parts of the beginning as needed. */
    /* cppcheck-suppress uninitdata */
    idlen = strlen(line);
    if idlen > ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong {
        line = line.offset(
            idlen.wrapping_sub(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                as isize,
        );
        idlen = ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong;
    }
    strncpy(((*new).ut_id).as_mut_ptr(), line, idlen);

    debug_return!();
}

/*
 * Store time in utmp structure.
 */

pub unsafe extern "C" fn utmp_settime(ut: *mut sudo_utmp_t) {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTMP);

    if gettimeofday(&mut tv, 0 as *mut timezone) == 0 {
        (*ut).ut_tv.tv_sec = tv.tv_sec as __int32_t;
        (*ut).ut_tv.tv_usec = tv.tv_usec as __int32_t;
    }
    debug_return!();
}

/*
 * Fill in a utmp entry, using an old entry as a template if there is one.
 */
pub unsafe extern "C" fn utmp_fill(
    mut line: *const libc::c_char,
    mut user: *const libc::c_char,
    mut ut_old: *mut sudo_utmp_t,
    mut ut_new: *mut sudo_utmp_t,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTMP);

    if ut_old.is_null() {
        memset(
            ut_new as *mut libc::c_void,
            0,
            ::std::mem::size_of::<sudo_utmp_t>() as libc::c_ulong,
        );

        if user.is_null() {
            strncpy(
                ((*ut_new).ut_user).as_mut_ptr(),
                user_details.username,
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            );
        }
    } else if ut_old != ut_new {
        memcpy(
            ut_new as *mut libc::c_void,
            ut_old as *const libc::c_void,
            ::std::mem::size_of::<sudo_utmp_t>() as libc::c_ulong,
        );
    }
    if !user.is_null() {
        strncpy(
            ((*ut_new).ut_user).as_mut_ptr(),
            user,
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    strncpy(
        ((*ut_new).ut_line).as_mut_ptr(),
        line,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );

    utmp_setid(ut_old, ut_new);
    (*ut_new).ut_pid = getpid();
    utmp_settime(ut_new);
    (*ut_new).ut_type = 7 as libc::c_int as libc::c_short;

    debug_return!();
}

/*
 * There are two basic utmp file types:
 *
 *  POSIX:  sequential access with new entries appended to the end.
 *	    Manipulated via {get,put}[sx]?utent()
 *
 *  Legacy: sparse file indexed by ttyslot() * sizeof(struct utmp)
 */
#[no_mangle]
pub unsafe extern "C" fn utmp_login(
    mut from_line: *const libc::c_char,
    mut to_line: *const libc::c_char,
    mut _ttyfd: libc::c_int,
    mut user: *const libc::c_char,
) -> bool {
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
    let mut ut_old: *mut sudo_utmp_t = 0 as *mut sudo_utmp_t;
    let mut ret: bool = false;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTMP);

    /* Strip off /dev/ prefix from line as needed. */
    if strncmp(
        to_line,
        b"/dev/\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1),
    ) == 0
    {
        to_line = to_line.offset(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1) as isize,
        );
    }
    setutxent();

    if from_line.is_null() {
        if strncmp(
            from_line,
            b"/dev/\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1),
        ) == 0
        {
            from_line = from_line.offset(
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1)
                    as isize,
            );
        }

        /* Lookup old line. */
        memset(
            &mut utbuf as *mut sudo_utmp_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<sudo_utmp_t>() as libc::c_ulong,
        );

        strncpy(
            (utbuf.ut_line).as_mut_ptr(),
            from_line,
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );

        ut_old = getutxline(&mut utbuf);
        setutxent();
    }
    utmp_fill(to_line, user, ut_old, &mut utbuf);
    if !(pututxline(&mut utbuf)).is_null() {
        ret = true;
    }
    endutxent();

    debug_return_bool!(ret)
}

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
