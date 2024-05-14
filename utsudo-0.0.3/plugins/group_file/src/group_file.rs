/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_unsafe,
    unused_variables
)]
use crate::common::*;
extern "C" {
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn mysetgrfile(_: *const libc::c_char);
    fn mysetgrent();
    fn myendgrent();
    fn mygetgrnam(_: *const libc::c_char) -> *mut group;
}


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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(xstat_flag as libc::c_int, __path, __statbuf);
}
static mut sudo_log: sudo_printf_t = None;
unsafe extern "C" fn sample_init(
    mut version: libc::c_int,
    mut sudo_printf: sudo_printf_t,
    mut argv: *const *mut libc::c_char,
) -> libc::c_int {
    let mut sb: stat = sb_all_arch;
    sudo_log = sudo_printf;
    if SUDO_API_VERSION_GET_MAJOR!(version) != GROUP_API_VERSION_MAJOR {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG,
            b"group_file: incompatible major version %d, expected %d\n\0" as *const u8
                as *const libc::c_char,
            SUDO_API_VERSION_GET_MAJOR!(version),
            GROUP_API_VERSION_MAJOR,
        );
        return -(1 as libc::c_int);
    }
    /* Sanity check the specified group file. */
    if argv.is_null() || (*argv.offset(0 as libc::c_int as isize)).is_null() {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG,
            b"group_file: path to group file not specified\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if stat(*argv.offset(0 as libc::c_int as isize), &mut sb) != 0 as libc::c_int {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG,
            b"group_file: %s: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if sb.st_mode & (S_IWGRP!() | S_IWOTH!()) != 0 {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG,
            b"%s must be only be writable by owner\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return -(1 as libc::c_int);
    }
    mysetgrfile(*argv.offset(0 as libc::c_int as isize));
    mysetgrent();
    return 1 as libc::c_int;
}
unsafe extern "C" fn sample_cleanup() {
    myendgrent();
}
/*
 * Returns true if "user" is a member of "group", else false.
 */
unsafe extern "C" fn sample_query(
    mut user: *const libc::c_char,
    mut group: *const libc::c_char,
    mut pwd: *const passwd,
) -> libc::c_int {
    let mut grp: *mut group = 0 as *mut group;
    let mut member: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    grp = mygetgrnam(group);
    if !grp.is_null() && !((*grp).gr_mem).is_null() {
        member = (*grp).gr_mem;
        while !(*member).is_null() {
            if strcasecmp(user, *member) == 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            member = member.offset(1);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut group_plugin: sudoers_group_plugin = unsafe {
    {
        let mut init = sudoers_group_plugin {
            version: ((1 as libc::c_int) << 16 as libc::c_int | 0 as libc::c_int) as libc::c_uint,
            init: Some(
                sample_init
                    as unsafe extern "C" fn(
                        libc::c_int,
                        sudo_printf_t,
                        *const *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            cleanup: Some(sample_cleanup as unsafe extern "C" fn() -> ()),
            query: Some(
                sample_query
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        *const passwd,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
