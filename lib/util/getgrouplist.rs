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