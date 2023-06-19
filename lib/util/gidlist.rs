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

