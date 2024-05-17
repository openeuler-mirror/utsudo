/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_unsafe
)]
use crate::common::*;
extern "C" {
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn setgrent();
    fn endgrent();
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn sudo_dso_findsym_v1(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn sudo_strtoid_v2(str: *const libc::c_char, errstr: *mut *const libc::c_char) -> id_t;
}

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
pub type sysgroup_getgrnam_t = Option<unsafe extern "C" fn(*const libc::c_char) -> *mut group>;
pub type sysgroup_getgrgid_t = Option<unsafe extern "C" fn(gid_t) -> *mut group>;
pub type sysgroup_gr_delref_t = Option<unsafe extern "C" fn(*mut group) -> ()>;
static mut sudo_log: sudo_printf_t = None;
static mut sysgroup_getgrnam: sysgroup_getgrnam_t = None;
static mut sysgroup_getgrgid: sysgroup_getgrgid_t = None;
static mut sysgroup_gr_delref: sysgroup_gr_delref_t = None;
static mut need_setent: bool = false;
unsafe extern "C" fn sysgroup_init(
    mut version: libc::c_int,
    mut sudo_printf: sudo_printf_t,
    mut argv: *const *mut libc::c_char,
) -> libc::c_int {
    let mut handle: *mut libc::c_void = 0 as *mut libc::c_void;
    sudo_log = sudo_printf;
    if SUDO_API_VERSION_GET_MAJOR!(version) != GROUP_API_VERSION_MAJOR {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG,
            b"sysgroup_group: incompatible major version %d, expected %d\n\0" as *const u8
                as *const libc::c_char,
            SUDO_API_VERSION_GET_MAJOR!(version),
            GROUP_API_VERSION_MAJOR,
        );

        return -(1 as libc::c_int);
    }

    /* Share group cache with sudo if possible. */
    handle = sudo_dso_findsym_v1(
        SUDO_DSO_DEFAULT,
        b"sudo_getgrnam\0" as *const u8 as *const libc::c_char,
    );
    if !handle.is_null() {
        sysgroup_getgrnam =
            ::core::mem::transmute::<*mut libc::c_void, sysgroup_getgrnam_t>(handle);
    } else {
        sysgroup_getgrnam = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const libc::c_char) -> *mut group>,
            sysgroup_getgrnam_t,
        >(Some(
            getgrnam as unsafe extern "C" fn(*const libc::c_char) -> *mut group,
        ));
        need_setent = true;
    }
    handle = sudo_dso_findsym_v1(
        SUDO_DSO_DEFAULT,
        b"sudo_getgrgid\0" as *const u8 as *const libc::c_char,
    );
    if !handle.is_null() {
        sysgroup_getgrgid =
            ::core::mem::transmute::<*mut libc::c_void, sysgroup_getgrgid_t>(handle);
    } else {
        sysgroup_getgrgid = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(__gid_t) -> *mut group>,
            sysgroup_getgrgid_t,
        >(Some(
            getgrgid as unsafe extern "C" fn(__gid_t) -> *mut group,
        ));
        need_setent = true;
    }
    handle = sudo_dso_findsym_v1(
        SUDO_DSO_DEFAULT,
        b"sudo_gr_delref\0" as *const u8 as *const libc::c_char,
    );
    if !handle.is_null() {
        sysgroup_gr_delref =
            ::core::mem::transmute::<*mut libc::c_void, sysgroup_gr_delref_t>(handle);
    }
    if need_setent {
        setgrent();
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sysgroup_cleanup() {
    if need_setent {
        endgrent();
    }
}

