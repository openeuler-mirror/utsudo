/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clashing_extern_declarations)]
//use libc::fstat;
use libc::exit;
use utsudo_util::macro_struct::sudo_conf_debug_file_list;
//use libc::ssize_t;
//use libc::timespec;
mod copy_file;
mod exec_common;
mod struct_macro;
pub use copy_file::*;
pub use exec_common::*;
use stdext::function_name;
pub use struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

extern "C" {
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_copy_file(
        src: *const libc::c_char,
        src_fd: libc::c_int,
        src_len: off_t,
        dst: *const libc::c_char,
        dst_fd: libc::c_int,
        dst_len: off_t,
    ) -> libc::c_int;
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int, __stat_buf: *mut stat) -> libc::c_int;
    fn geteuid() -> __uid_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn sudo_check_temp_file(
        tfd: libc::c_int,
        tname: *const libc::c_char,
        uid: uid_t,
        sb: *mut stat,
    ) -> bool;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn initprogname(_: *const libc::c_char);
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_conf_read_v1(conf_file: *const libc::c_char, conf_types: libc::c_int) -> libc::c_int;
    fn sudo_debug_register_v1(
        program: *const libc::c_char,
        subsystems: *const *const libc::c_char,
        ids: *mut libc::c_uint,
        debug_files: *mut sudo_conf_debug_file_list,
    ) -> libc::c_int;
    fn sudo_getprogname() -> *const libc::c_char;
    fn sudo_conf_debug_files_v1(progname: *const libc::c_char) -> *mut sudo_conf_debug_file_list;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_execve(
        fd: libc::c_int,
        path: *const libc::c_char,
        argv: *const *mut libc::c_char,
        envp: *mut *mut libc::c_char,
        noexec: bool,
    ) -> libc::c_int;
    fn sudo_debug_exit_int_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: libc::c_int,
    );
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    let mut vars: Vec<*mut libc::c_char> = Vec::new();
    for (var_name, var_value) in ::std::env::vars() {
        let var: String = format!("{}={}", var_name, var_value);
        vars.push(
            (::std::ffi::CString::new(var))
                .expect("Failed to convert environment variable into CString.")
                .into_raw(),
        );
    }
    vars.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
            vars.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}




