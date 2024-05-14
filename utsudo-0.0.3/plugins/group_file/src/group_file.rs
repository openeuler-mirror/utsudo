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
