/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use libc::__errno_location;
use libc::strerror;
extern "C" {
    fn malloc(__size: size_t) -> *mut libc::c_void;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn sudo_closefrom(_: libc::c_int);
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn sudo_debug_update_fd_v1(ofd: libc::c_int, nfd: libc::c_int);
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
}
use crate::sudo_debug_printf2_v1;
use stdext::function_name;
