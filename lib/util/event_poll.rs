/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    non_camel_case_types,
    dead_code,
    unused_variables,
    unused_mut,
    unused_assignments,
    non_snake_case,
    improper_ctypes,
    unused_parens
)]

use crate::event::sudo_ev_callback_t;
use crate::sudo_debug::sudo_debug_exit_v1;
// use crate::sudo_debug::sudo_debug_printf2_v1;
use crate::term::__sigset_t;

use libc::free;

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_DEBUG;
use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_EVENT;
use crate::sudo_debug_macro::SUDO_DEBUG_INFO;
use crate::sudo_debug_macro::SUDO_DEBUG_LINENO;

pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type nfds_t = libc::c_ulong;

pub const __SIGRTMIN: libc::c_uint = 64;
pub const NSIG: libc::c_uint = __SIGRTMIN + 1;
pub const SUDO_EV_READ: libc::c_short = 0x02;
pub const POLLIN: libc::c_short = 0x001;
pub const POLLHUP: libc::c_short = 0x010;
pub const POLLNVAL: libc::c_short = 0x020;
pub const POLLERR: libc::c_short = 0x008;
pub const SUDO_EV_WRITE: libc::c_short = 0x04;
pub const POLLOUT: libc::c_short = 0x004;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
}


extern "C" {
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn ppoll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: *const timespec,
        __ss: *const sigset_t,
    ) -> libc::c_int;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mid_struct_1 {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mid_struct_2 {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mid_struct_3 {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event {
    pub entries: mid_struct_1,
    pub active_entries: mid_struct_2,
    pub timeouts_entries: mid_struct_3,
    pub base: *mut sudo_event_base,
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
    pub flags: libc::c_short,
    pub pfd_idx: libc::c_short,
    pub callback: sudo_ev_callback_t,
    pub timeout: timespec,
    pub closure: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event_list {
    pub tqh_first: *mut sudo_event,
    pub tqh_last: *mut *mut sudo_event,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event_base {
    pub events: sudo_event_list,
    pub active: sudo_event_list,
    pub timeouts: sudo_event_list,
    pub signal_event: sudo_event,
    pub signals: [sudo_event_list; NSIG as usize],
    pub orig_handlers: [*mut sigaction; NSIG as usize],
    pub siginfo: [*mut siginfo_t; NSIG as usize],
    pub signal_pending: [sig_atomic_t; NSIG as usize],
    pub signal_caught: sig_atomic_t,
    pub num_handlers: libc::c_int,
    pub signal_pipe: [libc::c_int; 2],
    pub pfds: *mut pollfd,
    pub pfd_max: libc::c_int,
    pub pfd_high: libc::c_int,
    pub pfd_free: libc::c_int,
    pub flags: libc::c_uint,
}

#[no_mangle]
unsafe fn sudo_ev_base_alloc_impl(mut base: *mut sudo_event_base) -> libc::c_int {
    let mut i: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EVENT);

    (*base).pfd_high = -1;
    (*base).pfd_max = 32;
    (*base).pfds = reallocarray(
        0 as *mut libc::c_void,
        (*base).pfd_max as libc::c_ulong,
        std::mem::size_of::<pollfd>() as size_t,
    ) as *mut pollfd;
    
    while i < (*base).pfd_max {
        (*((*base).pfds).offset(i as isize)).fd = -1;
        i += 1;
    }
    debug_return_int!(0)
}

#[no_mangle]
unsafe fn sudo_ev_base_free_impl(mut base: *mut sudo_event_base) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EVENT);

    free((*base).pfds as *mut libc::c_void);
    debug_return!()
}

#[no_mangle]
unsafe fn sudo_ev_add_impl(mut base: *mut sudo_event_base, mut ev: *mut sudo_event) -> libc::c_int {
    let mut pfd: *mut pollfd = 0 as *mut pollfd;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EVENT);

    /* If out of space in pfds array, realloc. */
    if (*base).pfd_free == (*base).pfd_max {
        let mut pfds: *mut pollfd = 0 as *mut pollfd;
        let mut i: libc::c_int = 0;

        pfds = reallocarray(
            (*base).pfds as *mut libc::c_void,
            (*base).pfd_max as size_t,
            ((2 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<pollfd>() as libc::c_ulong))
                as size_t,
        ) as *mut pollfd;

        (*base).pfds = pfds;
        (*base).pfd_max *= 4;

        i = (*base).pfd_free;
        while i < (*base).pfd_max {
            (*((*base).pfds).offset(i as isize)).fd = -1;
            i += 1;
        }
    }

    /* Fill in pfd entry. */
    (*ev).pfd_idx = (*base).pfd_free as libc::c_short;
    pfd = &mut *((*base).pfds).offset((*ev).pfd_idx as isize);
    (*pfd).fd = (*ev).fd;

    /* Update pfd_high and pfd_free. */
    if (*ev).pfd_idx as libc::c_int > (*base).pfd_high {
        (*base).pfd_high = (*ev).pfd_idx as libc::c_int;
    }
    loop {
        (*base).pfd_free += 1;
        if (*base).pfd_free == (*base).pfd_max {
            break;
        }
        if (*((*base).pfds).offset((*base).pfd_free as isize)).fd == -1 {
            break;
        }
    }
    debug_return_int!(0)
}


#[no_mangle]
unsafe fn sudo_ev_del_impl(mut base: *mut sudo_event_base, mut ev: *mut sudo_event) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EVENT);

    /* Mark pfd entry unused, add to free list and adjust high slot. */
    (*((*base).pfds).offset((*ev).pfd_idx as isize)).fd = -1;
    if ((*ev).pfd_idx as libc::c_int) < (*base).pfd_free {
        (*base).pfd_free = (*ev).pfd_idx as libc::c_int;
    }

    while ((*base).pfd_high >= 0) && (*((*base).pfds).offset((*base).pfd_high as isize)).fd == -1 {
        (*base).pfd_high -= 1;
    }
    debug_return_int!(0)
}

unsafe fn sudo_ev_poll(
    mut fds: *mut pollfd,
    mut nfds: nfds_t,
    mut timo: *mut timespec,
) -> libc::c_int {
    return ppoll(fds, nfds, timo, 0 as *const sigset_t);
}

