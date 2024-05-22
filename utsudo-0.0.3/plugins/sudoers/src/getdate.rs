/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::common::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
// 多处定义，后期统一处理
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
// 多处定义，后期统一处理
pub const _ISalnum: libc::c_uint = 8;
pub const _ISpunct: libc::c_uint = 4;
pub const _IScntrl: libc::c_uint = 2;
pub const _ISblank: libc::c_uint = 1;
pub const _ISgraph: libc::c_uint = 32768;
pub const _ISprint: libc::c_uint = 16384;
pub const _ISspace: libc::c_uint = 8192;
pub const _ISxdigit: libc::c_uint = 4096;
pub const _ISdigit: libc::c_uint = 2048;
pub const _ISalpha: libc::c_uint = 1024;
pub const _ISlower: libc::c_uint = 512;
pub const _ISupper: libc::c_uint = 256;
