/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    unused_mut,
    unused_unsafe,
    dead_code,
    non_upper_case_globals,
    unused_variables,
    unused_assignments
)]

//call libc_func
use libc::abort;

//define
pub const RSBUFSZ: i32 = 1024;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const KEYSZ: i32 = 32;
pub const IVSZ: i32 = 8;

//aliase of type
pub type pid_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pthread_list_t = __pthread_internal_list;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sig_atomic_t = libc::c_int;
pub type u8 = libc::c_uchar;
pub type u32 = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rs {
    pub rs_have: size_t,
    pub rs_count: size_t,
}
















