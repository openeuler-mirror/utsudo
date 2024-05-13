
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




unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut ch: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut group: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plugin: *const libc::c_char = b"group_file.so\0" as *const u8 as *const libc::c_char;
    let mut pwd: *const passwd = 0 as *const passwd;
    loop {
        ch = getopt(argc, argv, b"p:\0" as *const u8 as *const libc::c_char);
        if ch == -1 {
            break;
        }
        match ch as u8 as char {
            'p' => plugin = optarg,
            _ => {
                usage();
            }
        } //match
    } //loop


    argc -= optind;
    argv = argv.offset(optind as isize);
    if argc < 1 {
        usage();
    }
    if group_plugin_load(plugin as *mut libc::c_char) != 1 {
        fprintf(
            stderr,
            b"unable to load plugin: %s\0" as *const u8 as *const libc::c_char,
            plugin,
        );
        exit(1);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(argv.offset(i as isize)).is_null() {
        user = (*argv).offset(i as isize);
        group = strchr((*argv).offset(i as isize), ':' as i32);

        if group.is_null() {
            i += 1;
            continue;
        }
        *group = '\0' as libc::c_char;
        group = group.offset(1 as isize);
        pwd = getpwnam(user) as *mut passwd;
        found = group_plugin_query(user, group, pwd);
        printf(
            b"user %s %s in group %s\n\0" as *const u8 as *const libc::c_char,
            user,
            if found != 0 {
                b"is\0" as *const u8 as *const libc::c_char
            } else {
                b"NOT \0" as *const u8 as *const libc::c_char
            },
            group,
        );
        i += 1;
    }
    group_plugin_unload();
    exit(0)
}

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






