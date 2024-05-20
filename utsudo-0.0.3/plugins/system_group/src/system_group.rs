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

/*
 * Returns true if "user" is a member of "group", else false.
 */
unsafe extern "C" fn sysgroup_query(
    mut user: *const libc::c_char,
    mut group: *const libc::c_char,
    mut pwd: *const passwd,
) -> libc::c_int {
    let mut member: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut grp: *mut group = 0 as *mut group;
    grp = sysgroup_getgrnam.expect("non-null function pointer")(group);
    if grp.is_null()
        && *group.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        && *group.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
        let mut gid: gid_t = sudo_strtoid_v2(group.offset(1 as libc::c_int as isize), &mut errstr);
        if errstr.is_null() {
            grp = sysgroup_getgrgid.expect("non-null function pointer")(gid);
        }
    }
    if !grp.is_null() {
        if !((*grp).gr_mem).is_null() {
            member = (*grp).gr_mem;
            while !(*member).is_null() {
                if strcasecmp(user, *member) == 0 as libc::c_int {
                    if sysgroup_gr_delref.is_some() {
                        sysgroup_gr_delref.expect("non-null function pointer")(grp);
                    }
                    return 1 as libc::c_int;
                }
                member = member.offset(1);
            }
        }
        if sysgroup_gr_delref.is_some() {
            sysgroup_gr_delref.expect("non-null function pointer")(grp);
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
                sysgroup_init
                    as unsafe extern "C" fn(
                        libc::c_int,
                        sudo_printf_t,
                        *const *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            cleanup: Some(sysgroup_cleanup as unsafe extern "C" fn() -> ()),
            query: Some(
                sysgroup_query
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
