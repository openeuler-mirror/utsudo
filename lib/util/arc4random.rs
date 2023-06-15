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

//line 78
static mut rs: *mut _rs = 0 as *const _rs as *mut _rs;

//line 81
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rsx {
    pub rs_chacha: chacha_ctx,
    pub rs_buf: [libc::c_uchar; 1024],
}

//line 82 included line81
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub input: [u32; 16],
}

//line 84
static mut rsx: *mut _rsx = 0 as *const _rsx as *mut _rsx;

//called by struct __pthread_mutex_s -> __pthread_list_t
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}

//called by union pthread_mutex_t
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}

//called by _ARC4_LOCK
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; __SIZEOF_PTHREAD_MUTEX_T],
    pub __align: libc::c_long,
}

//arc4random.h_line34,init arc4random_mtx,called by 208
static mut arc4random_mtx: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_short,
            __elision: 0 as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};










