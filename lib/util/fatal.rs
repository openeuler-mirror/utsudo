/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
 #![allow(
    non_camel_case_types,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_unsafe,
    non_upper_case_globals,
    dead_code
)]
use crate::__LC_MESSAGES;
use libc::FILE;

#[macro_export]
macro_rules! __LC_MESSAGES {
    () => {
        5
    };
}


pub type ssize_t = libc::c_long;
pub type sudo_fatal_callback_t = Option<unsafe extern "C" fn()>;
static mut sudo_warn_conversation: sudo_conv_t = None;
static mut sudo_warn_setlocale: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool> = None;
static mut sudo_warn_setlocale_prev: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool> =
    None;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_fatal_callback {
    pub entries: STRUCT_unnamed,
    pub func: Option<unsafe extern "C" fn() -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct STRUCT_unnamed {
    pub sle_next: *mut sudo_fatal_callback,
}