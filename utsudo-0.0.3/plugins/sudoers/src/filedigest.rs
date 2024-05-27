/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    unreachable_code,
    unused_variables,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sudo_digest_free_v1(dig: *mut sudo_digest);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn sudo_digest_getlen_v1(digest_type: libc::c_int) -> libc::c_int;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_digest_alloc_v1(digest_type: libc::c_int) -> *mut sudo_digest;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn sudo_digest_final_v1(dig: *mut sudo_digest, md: *mut libc::c_uchar);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn sudo_digest_update_v1(dig: *mut sudo_digest, data: *const libc::c_void, len: size_t);
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
}
use crate::common::*;
