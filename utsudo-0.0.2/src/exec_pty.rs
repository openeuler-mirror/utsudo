/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_variables, unreachable_patterns, clashing_extern_declarations)]

use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_buffer_list {
    pub slh_first: *mut io_buffer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_buffer {
    pub entries: SLIST_ENTRY_io_buffer,
    pub ec: *mut exec_closure_pty,
    pub revent: *mut sudo_event,
    pub wevent: *mut sudo_event,
    pub action: sudo_io_action_t,
    pub len: libc::c_int,
    pub off: libc::c_int,
    pub buf: [libc::c_char; 64 * 1024],
}

pub type sudo_io_action_t =
    Option<unsafe extern "C" fn(*const libc::c_char, libc::c_uint, *mut io_buffer) -> bool>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SLIST_ENTRY_io_buffer {
    pub sle_next: *mut io_buffer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct monitor_message_list {
    pub tqh_first: *mut monitor_message,
    pub tqh_last: *mut *mut monitor_message,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monitor_message {
    pub entries: TAILQ_ENTRY_monitor_message,
    pub cstat: command_status,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_monitor_message {
    pub tqe_next: *mut monitor_message,
    pub tqe_prev: *mut *mut monitor_message,
}


#[macro_export]
macro_rules! TAILQ_EMPTY {
    ($head:expr) => {
        (($head).tqh_first).is_null()
    };
}

#[macro_export]
macro_rules! TAILQ_FIRST {
    ($head:expr) => {
        (($head).tqh_first)
    };
}

#[macro_export]
macro_rules! SLIST_FIRST {
    ($head:expr) => {
        (($head).slh_first)
    };
}

#[macro_export]
macro_rules! sudo_ev_get_fd {
    ($_ev:expr) => {
        if !($_ev).is_null() {
            (*$_ev).fd
        } else {
            -(1 as libc::c_int)
        }
    };
}

#[macro_export]
macro_rules! sudo_ev_get_base {
    ($_ev:expr) => {
        if !($_ev).is_null() {
            (*$_ev).base
        } else {
            0 as *mut sudo_event_base
        }
    };
}

#[macro_export]
macro_rules! USERTTY_EVENT {
    ($_ev:expr) => {
        (sudo_ev_get_fd!(($_ev)) == io_fds[SFD_USERTTY as usize])
    };
}

#[macro_export]
macro_rules! S_ISFIFO {
    ($mode: expr) => {
        __S_ISTYPE!(($mode), __S_IFIFO as libc::c_uint)
    };
}

#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
        #[cfg(target_arch = "x86_64")]
        return __fxstat(1, __fd, __statbuf);
        #[cfg(not(target_arch = "x86_64"))]
        return __fxstat(0, __fd, __statbuf);
}
static mut ptyname: [libc::c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
#[no_mangle]
pub static mut io_fds: [libc::c_int; 6] = [-1; 6];
static mut foreground: bool = false;
static mut pipeline: bool = false;
static mut ttymode: libc::c_int = TERM_COOKED;
static mut ttyblock: sigset_t = sigset_t { __val: [0; 16] };
static mut iobufs: io_buffer_list = io_buffer_list {
    slh_first: 0 as *const io_buffer as *mut io_buffer,
};
static mut utmp_user: *const libc::c_char = 0 as *const libc::c_char;

/*
 * Cleanup hook for sudo_fatal()/sudo_fatalx()
 */
#[no_mangle]
pub unsafe extern "C" fn pty_cleanup() {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    if io_fds[SFD_USERTTY as usize] != -(1 as libc::c_int) {
        sudo_term_restore_v1(io_fds[SFD_USERTTY as usize], false);
    }
    if !utmp_user.is_null() {
        utmp_logout(ptyname.as_mut_ptr(), 0);
    }

    debug_return!();
}