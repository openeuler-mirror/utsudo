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

#[macro_export]
macro_rules! SUDO_DSO_LAZY {
    () => {
        0x1
    };
}

#[macro_export]
macro_rules! SUDO_DSO_NOW {
    () => {
        0x2
    };
}

#[macro_export]
macro_rules! SUDO_DSO_GLOBAL {
    () => {
        0x3
    };
}

#[macro_export]
macro_rules! SUDO_DSO_LOCAL {
    () => {
        0x4
    };
}

/* The MODE argument to `dlopen' contains one of the following: */
// #define RTLD_LAZY    0x00001 /* Lazy function call binding.  */
// #define RTLD_NOW     0x00002 /* Immediate function call binding.  */
#[macro_export]
macro_rules! RTLD_LAZY {
    () => {
        0x00001
    };
}

#[macro_export]
macro_rules! RTLD_NOW {
    () => {
        0x00002
    };
}

#[macro_export]
macro_rules! RTLD_GLOBAL {
    () => {
        0x00100
    };
}

#[macro_export]
macro_rules! RTLD_LOCAL {
    () => {
        0
    };
}


pub const SUDO_DSO_NEXT: *mut libc::c_void = -(1 as libc::c_int) as *mut libc::c_void;
pub const SUDO_DSO_DEFAULT: *mut libc::c_void = -(2 as libc::c_int) as *mut libc::c_void;
pub const SUDO_DSO_SELF: *mut libc::c_void = -(3 as libc::c_int) as *mut libc::c_void;

pub const RTLD_NEXT: *mut libc::c_void = -(1 as libc::c_long) as *mut libc::c_void;

pub const RTLD_DEFAULT: *mut libc::c_void = 0 as *mut libc::c_void;

pub const ENOENT: libc::c_int = 2;

extern "C" {
    fn dlerror() -> *mut libc::c_char;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_preload_symbol {
    pub name: *const libc::c_char,
    pub addr: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_preload_table {
    pub path: *const libc::c_char,
    pub handle: *mut libc::c_void,
    pub symbols: *mut sudo_preload_symbol,
}

static mut preload_table: *mut sudo_preload_table = 0 as *mut sudo_preload_table;

#[no_mangle]
pub unsafe extern "C" fn sudo_dso_preload_table_v1(mut table: *mut sudo_preload_table) {
    preload_table = table;
}

