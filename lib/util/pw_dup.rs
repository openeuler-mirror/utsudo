/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    mutable_transmutes,
    non_camel_case_types,
    unused_assignments,
    unused_mut,
    clashing_extern_declarations,
    dead_code,
    unused_variables,
    unused_macros
)]

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}

pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn sudo_pw_dup(mut pw: *const passwd) -> *mut passwd {
    let mut nsize: size_t = 0 as libc::c_int as size_t;
    let mut psize: size_t = 0 as libc::c_int as size_t;
    let mut total: size_t = 0;
    let mut newpw: *mut passwd = 0 as *mut passwd;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    total = ::core::mem::size_of::<passwd>() as libc::c_ulong;
    if !((*pw).pw_name).is_null() {
        nsize = (strlen((*pw).pw_name)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        total = (total as libc::c_ulong).wrapping_add(nsize) as size_t as size_t;
    }
    if !((*pw).pw_passwd).is_null() {
        psize = (strlen((*pw).pw_passwd))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        total = (total as libc::c_ulong).wrapping_add(psize) as size_t as size_t;
    }
    if !((*pw).pw_gecos).is_null() {
        gsize = (strlen((*pw).pw_gecos)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        total = (total as libc::c_ulong).wrapping_add(gsize) as size_t as size_t;
    }
    cp = malloc(total) as *mut libc::c_char;
    if cp.is_null() {
        return 0 as *mut passwd;
    }
    newpw = cp as *mut passwd;
    memcpy(
        newpw as *mut libc::c_void,
        pw as *const libc::c_void,
        ::core::mem::size_of::<passwd>() as libc::c_ulong,
    );
    cp = cp.offset(::core::mem::size_of::<passwd>() as libc::c_ulong as isize);
    if !((*pw).pw_name).is_null() {
        memcpy(cp as *mut libc::c_void, (*pw).pw_name as *const libc::c_void, nsize);
        (*newpw).pw_name = cp;
        cp = cp.offset(nsize as isize);
    }
    if !((*pw).pw_passwd).is_null() {
        memcpy(cp as *mut libc::c_void, (*pw).pw_passwd as *const libc::c_void, psize);
        (*newpw).pw_passwd = cp;
        cp = cp.offset(psize as isize);
    }
    if !((*pw).pw_gecos).is_null() {
        memcpy(cp as *mut libc::c_void, (*pw).pw_gecos as *const libc::c_void, gsize);
        (*newpw).pw_gecos = cp;
        cp = cp.offset(gsize as isize);
    }
    return newpw;
}
