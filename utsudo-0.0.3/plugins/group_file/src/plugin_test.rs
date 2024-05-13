
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











extern "C" {
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut stderr: *mut FILE;
    fn plugin_printf(msg_type: libc::c_int, fmt: *const libc::c_char, args: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
}
static mut group_handle: *mut libc::c_void = 0 as *mut libc::c_void;
static mut group_plugin: *mut sudoers_group_plugin =
    0 as *const sudoers_group_plugin as *mut sudoers_group_plugin;
pub type sudo_printf_t =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_group_plugin {
    pub version: libc::c_uint,
    pub init: Option<
        unsafe extern "C" fn(libc::c_int, sudo_printf_t, *const *mut libc::c_char) -> libc::c_int,
    >,
    pub cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub query: Option<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *const passwd,
        ) -> libc::c_int,
    >,
}
unsafe fn group_plugin_unload() {
    ((*group_plugin).cleanup).expect("non-null function pointer")();
    dlclose(group_handle);
    group_handle = 0 as *mut libc::c_void;
}
unsafe fn group_plugin_load(mut plugin_info: *mut libc::c_char) -> libc::c_int {
    let mut args: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut savedch: libc::c_char = 0 as libc::c_char;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    args = strpbrk(plugin_info, b" \t\0" as *const u8 as *const libc::c_char);
    if args.is_null() {
        savedch = *args;
        *args = '\0' as libc::c_char;
    }
    if sudo_strlcpy(
        path.as_mut_ptr(),
        plugin_info,
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ) >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
    {
        fprintf(
            stderr,
            b"path to long: %s\n\0" as *const u8 as *const libc::c_char,
            plugin_info,
        );
        return -1;
    }
    if !args.is_null() {
        *args = savedch;
        args = args.offset(1);
    }
    group_handle = dlopen(path.as_mut_ptr(), 1);
    if group_plugin.is_null() {
        fprintf(
            stderr,
            b"unable to find symbol \"group_plugin\" in %s\n\0" as *const u8 as *const libc::c_char,
            path,
        );
        return -1;
    }
    if (*group_plugin).version >> 16 as libc::c_int != 1 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"%s: incompatible group plugin major version %d, expected %d\n\0" as *const u8
                as *const libc::c_char,
            path.as_mut_ptr(),
            (*group_plugin).version >> 16 as libc::c_int,
            1 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if !args.is_null() {
        let mut ac: libc::c_int = 0 as libc::c_int;
        let mut wasblank: libc::c_int = 1 as libc::c_int;
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        cp = args;
        while *cp as libc::c_int != '\0' as i32 {
            if *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & 1 as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                wasblank = 1 as libc::c_int;
            } else if wasblank != 0 {
                wasblank = 0 as libc::c_int;
                ac += 1;
            }
            cp = cp.offset(1);
        }
        if ac != 0 {
            let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
            argv = malloc(
                (ac as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            if argv.is_null() {
                perror(0 as *const libc::c_char);
                return -1;
            }
            ac = 0;
            cp = strtok_r(
                args,
                b" \t\0" as *const u8 as *const libc::c_char,
                &mut last,
            );
            while !cp.is_null() {
                let fresh1 = ac;
                ac = ac + 1;
                let ref mut fresh2 = *argv.offset(fresh1 as isize);
                *fresh2 = cp;
                cp = strtok_r(
                    0 as *mut libc::c_char,
                    b" \t\0" as *const u8 as *const libc::c_char,
                    &mut last,
                );
            }
        }
    }
    rc = ((*group_plugin).init).expect("non-null function pointer")(
        (1 as libc::c_int) << 16 as libc::c_int | 0 as libc::c_int,
        Some(
            plugin_printf
                as unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int,
        ),
        argv as *const *mut libc::c_char,
    );
    free(argv as *mut libc::c_void);
    return rc;
}

unsafe fn usage() {
    fprintf(
        stderr,
        b"usage: plugin_test [-p \"plugin.so plugin_args ...\"] user:group ...\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(1);
}
unsafe fn group_plugin_query(
    mut user: *const libc::c_char,
    mut group: *const libc::c_char,
    mut pwd: *const passwd,
) -> libc::c_int {
    return ((*group_plugin).query).expect("non-null function pointer")(user, group, pwd);
}







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






