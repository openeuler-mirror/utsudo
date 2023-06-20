/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    mutable_transmutes,
    non_camel_case_types,
    unused_assignments,
    unused_mut
)]

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

extern "C" {
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
}

pub type __gid_t = libc::c_uint; //typedef __gid_t gid_t;
pub type gid_t = __gid_t;
pub type GETGROUPS_T = gid_t; //#define GETGROUPS_T gid_t
pub type size_t = libc::c_ulong;
pub type id_t = libc::c_uint;

#[no_mangle]
pub unsafe extern "C" fn sudo_parse_gids_v1(
    mut gidstr: *const libc::c_char,
    mut basegid: *const gid_t,
    mut gidsp: *mut *mut GETGROUPS_T,
) -> libc::c_int {
    let mut ngids: libc::c_int = 0;
    let mut gids: *mut GETGROUPS_T = 0 as *mut GETGROUPS_T;
    let mut cp: *const libc::c_char = gidstr;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if *cp as libc::c_int != '\0' as i32 {
        ngids += 1; //ngids++
        loop {
            if *cp as libc::c_int == ',' as i32 {
                ngids += 1;
            }
            cp = cp.offset(1);

            if *cp as libc::c_int != '\0' as i32 {
                break;
            }
        }
    }
    if !basegid.is_null() {
        ngids += 1;
    }

    if ngids != 0 {
        gids = reallocarray(
            0 as *mut libc::c_void,
            ngids as size_t,
            ::std::mem::size_of::<GETGROUPS_T>() as libc::c_ulong, //sizeof(GETGROUPS_T)
        ) as *mut GETGROUPS_T;
        if gids.is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                b"unable to allocate memor0" as *const u8 as *const libc::c_char
            );
            debug_return_int!(-1);
        }
        ngids = 0;
        if !basegid.is_null() {
            *gids.offset(ngids as isize) = *basegid;
            ngids += 1;
        }

        cp = gidstr;

}

