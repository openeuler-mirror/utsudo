/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    unused_variables,
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    clashing_extern_declarations,
    unreachable_code
)]

use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

#[link(name = "selinux")]
#[link(name = "audit")]

extern "C" {
    fn audit_open() -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, ...) -> libc::c_int;
    fn audit_log_user_message(
        audit_fd: libc::c_int,
        type_0: libc::c_int,
        message: *const libc::c_char,
        hostname: *const libc::c_char,
        addr: *const libc::c_char,
        tty: *const libc::c_char,
        result: libc::c_int,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fgetfilecon(fd: libc::c_int, con: *mut *mut libc::c_char) -> libc::c_int;
    fn freecon(con: *mut libc::c_char);
    fn strcmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn fsetfilecon(fd: libc::c_int, con: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn string_to_security_class(name: *const libc::c_char) -> security_class_t;
    fn security_compute_relabel(
        scon: *const libc::c_char,
        tcon: *const libc::c_char,
        tclass: security_class_t,
        newcon: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn context_free(_: context_t) -> libc::c_void;
    fn get_default_type(role: *const libc::c_char, types: *mut *mut libc::c_char) -> libc::c_int;
    fn context_new(_: *const libc::c_char) -> context_t;
    fn context_role_set(_: context_t, _: *const libc::c_char) -> libc::c_int;
    fn context_type_set(_: context_t, _: *const libc::c_char) -> libc::c_int;
    fn security_check_context(con: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn context_str(_: context_t) -> *mut libc::c_char;
    fn getprevcon(con: *mut *mut libc::c_char) -> libc::c_int;
    fn security_getenforce() -> libc::c_int;
    fn setexeccon(con: *const libc::c_char) -> libc::c_int;
    fn setkeycreatecon(context: *const libc::c_char) -> libc::c_int;
    fn sudo_conf_sesh_path_v1() -> *const libc::c_char;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn sudo_execve(
        fd: libc::c_int,
        path: *const libc::c_char,
        argv: *const *mut libc::c_char,
        envp: *mut *mut libc::c_char,
        noexec: bool,
    ) -> libc::c_int;
    fn memcpy(
        __restrict__: *mut libc::c_void,
        __restrict__: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
}
