
/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    unused_variables,
    unused_mut,
    dead_code,
    unused_assignments,
    non_camel_case_types,
    non_upper_case_globals
)]
use libc::dlclose;
use libc::dlopen;
use libc::exit;
use libc::fprintf;
use libc::getopt;
use libc::getpwnam;
use libc::perror;
use libc::printf;
use libc::strchr;
use libc::strpbrk;
use libc::strtok_r;
use libc::FILE;

#[macro_use]
mod common;
use crate::common::*;
#[link(name = "utsudo_util")]
#[link(name = "util_variadic")]
#[link(name = "group_file_variadic")]



pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}






