/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    unused_must_use,
    unused_variables,
    clashing_extern_declarations,
    unreachable_patterns
)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use crate::errno;
use crate::ISSET;
use crate::SIG_IGN;
use crate::USER_SIGNALED;
use crate::WEXITSTATUS;
use crate::WIFEXITED;
use crate::WIFSIGNALED;
use crate::WIFSTOPPED;
use crate::WSTOPSIG;
use crate::WTERMSIG;

pub const WCONTINUED: libc::c_int = 8;

extern "C" {
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_sig2str(signo: libc::c_int, signame: *mut libc::c_char) -> libc::c_int;
    fn snprintf(
        _s: *mut libc::c_char,
        _maxlen: size_t,
        _format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn terminate_command(pid: pid_t, use_pgrp: bool);
    fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
    fn sudo_term_copy_v1(src: libc::c_int, dst: libc::c_int) -> bool;
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn _exit(_: libc::c_int);
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
    fn sudo_ev_loopexit_v1(base: *mut sudo_event_base);
    fn sudo_ev_loopbreak_v1(base: *mut sudo_event_base);
    fn sudo_ev_base_alloc_v1() -> *mut sudo_event_base;
    fn getpgid(__pid: __pid_t) -> __pid_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn strerror(__errnum: libc::c_int) -> *mut libc::c_char;
    fn sudo_ev_del_v1(head: *mut sudo_event_base, ev: *mut sudo_event) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn getpid() -> __pid_t;
    fn pty_cleanup();
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;
    fn exec_cmnd(details: *mut command_details, errfd: libc::c_int);
    fn getpgrp() -> __pid_t;
    fn sudo_ev_alloc_v1(
        fd: libc::c_int,
        events: libc::c_short,
        callback: sudo_ev_callback_t,
        closure: *mut libc::c_void,
    ) -> *mut sudo_event;
    fn sudo_ev_add_v2(
        head: *mut sudo_event_base,
        ev: *mut sudo_event,
        timo: *mut timespec,
        tohead: bool,
    ) -> libc::c_int;
    fn sudo_ev_base_setdef_v1(base: *mut sudo_event_base);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn setsid() -> __pid_t;
    fn pty_make_controlling() -> libc::c_int;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn sudo_debug_fork_v1() -> pid_t;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn restore_signals();
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sudo_ev_dispatch_v1(head: *mut sudo_event_base) -> libc::c_int;
    static mut io_fds: [libc::c_int; 6];
    fn sudo_fatal_callback_register_v1(func: sudo_fatal_callback_t) -> libc::c_int;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_sigaction(signo: libc::c_int, sa: *mut sigaction, osa: *mut sigaction) -> libc::c_int;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
}



