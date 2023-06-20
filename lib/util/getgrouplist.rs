/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

 #![allow(non_camel_case_types, unused_mut, unused_variables, unused_assignments)]

 //line72-2(arg),
pub type gid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type size_t = libc::c_ulong;

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

// define
pub const _SC_NGROUPS_MAX: libc::c_uint = 3;
pub const NGROUPS_MAX: libc::c_uint = 65536;

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

    //line94
    grpsize += 1;

}