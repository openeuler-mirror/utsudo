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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
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

    debug_return!()
}
