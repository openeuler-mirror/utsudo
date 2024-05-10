/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(non_camel_case_types, unused_mut, unused_variables, unused_assignments)]

use crate::macro_struct::*;

extern "C" {
    fn getgrouplist(
        __user: *const libc::c_char,
        __group: __gid_t,
        __groups: *mut __gid_t,
        __ngroups: *mut libc::c_int,
    ) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
}

//line_72
#[no_mangle]
pub unsafe extern "C" fn sudo_getgrouplist2_v1(
    mut name: *const libc::c_char,
    mut basegid: gid_t,
    mut groupsp: *mut *mut gid_t,
    mut ngroupsp: *mut libc::c_int,
) -> libc::c_int {
    let mut groups: *mut gid_t = *groupsp;
    let mut ngroups: libc::c_int = 0;
    let mut grpsize: libc::c_int = 0;
    let mut tries: libc::c_int = 0;

    //line82
    if !groups.is_null() {
        return getgrouplist(name, basegid, groups, ngroupsp);
    }

    //line91
    grpsize = sysconf(_SC_NGROUPS_MAX as libc::c_int) as libc::c_int;

    //line92
    if grpsize < 0 as libc::c_int {
        grpsize = NGROUPS_MAX as libc::c_int;
    }

    //line94
    grpsize += 1;

    //line99
    while tries < 10 {
        free(groups as *mut libc::c_void);
        //line101
        groups = reallocarray(
            0 as *mut libc::c_void,
            grpsize as size_t,
            ::std::mem::size_of::<gid_t>() as size_t,
        ) as *mut gid_t;
        if groups.is_null() {
            return -1 as libc::c_int;
        }
        ngroups = grpsize;
        //line105
        if getgrouplist(name, basegid, groups, &mut ngroups) != -1 {
            *groupsp = groups;
            *ngroupsp = ngroups;
            return 0 as libc::c_int;
        }
        if ngroups == grpsize {
            break;
        }
        grpsize = ngroups;
        tries += 1;
    }
    free(groups as *mut libc::c_void);
    return -1 as libc::c_int;
}
