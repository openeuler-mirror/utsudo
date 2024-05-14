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
