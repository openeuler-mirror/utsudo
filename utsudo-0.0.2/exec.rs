/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    non_snake_case,
    improper_ctypes,
    clashing_extern_declarations
)]

use crate::struct_macro::*;
use crate::ISSET;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

extern "C" {
    fn set_user_groups(details: *mut command_details) -> bool;
    fn setpriority(__which: __priority_which_t, __who: id_t, __prio: libc::c_int) -> libc::c_int;
    fn umask(_mask: __mode_t) -> __mode_t;
    fn chroot(__path: *const libc::c_char) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn setresuid(__ruid: __uid_t, __euid: __uid_t, __suid: __uid_t) -> libc::c_int;
    fn strcmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn sudo_debug_get_fds_v1(fds: *mut *mut libc::c_uchar) -> libc::c_int;
    fn add_preserved_fd(pfds: *mut preserved_fd_list, fd: libc::c_int) -> libc::c_int;
    fn closefrom_except(startfd: libc::c_int, pfds: *mut preserved_fd_list);
    fn selinux_execve(
        fd: libc::c_int,
        path: *const libc::c_char,
        argv: *const *mut libc::c_char,
        envp: *mut *mut libc::c_char,
        noexec: bool,
    );
    fn sudo_execve(
        fd: libc::c_int,
        path: *const libc::c_char,
        argv: *const *mut libc::c_char,
        envp: *mut *mut libc::c_char,
        noexec: bool,
    ) -> libc::c_int;
    fn memset(__dest: *mut libc::c_void, __ch: libc::c_int, __len: size_t) -> *mut libc::c_void;
    fn sudo_sigaction(signo: libc::c_int, sa: *mut sigaction, osa: *mut sigaction) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn signal_pending(signo: libc::c_int) -> bool;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
    fn exec_pty(details: *mut command_details, cstat: *mut command_status) -> bool;
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn _exit(__status: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn sudo_debug_fork_v1() -> pid_t;
    fn unlimit_nproc();
    fn restore_nproc();
    fn restore_signals();
    fn restore_limits();
    fn exec_nopty(details: *mut command_details, cstat: *mut command_status);
    fn __errno_location() -> *mut libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut user_details: user_details;
    static mut policy_plugin: plugin_container;
    static mut io_plugins: plugin_container_list;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
}

pub const NBBY: libc::c_int = 8;
pub const _SIGSET_NWORDS: libc::c_ulong = 16;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which = libc::c_uint;
pub type __priority_which_t = __priority_which;
