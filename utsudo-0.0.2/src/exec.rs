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

#[macro_export]
macro_rules! sudo_isset {
    ($_a:expr, $_i:expr) => {{
        (*(($_a).offset((($_i) / NBBY) as isize)) & (1 << (($_i) % NBBY)))
    }};
}
/*
 * Setup the execution environment immediately prior to the call to execve().
 * Group setup is performed by policy_init_session(), called earlier.
 * Returns true on success and false on failure.
 */
#[inline]
pub unsafe extern "C" fn exec_setup(mut details: *mut command_details) -> bool {
    let mut ret: bool = false;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    if !(*details).pw.is_null() {
        //空，该分支没有走到。
    }
    'done: loop {
        if ISSET!((*details).flags, CD_SET_GROUPS) != 0 {
            /* set_user_groups() prints error message on failure.*/
            if !set_user_groups(details) {
                break 'done;
            }
        }
        if ISSET!((*details).flags, CD_SET_PRIORITY) != 0 {
            if setpriority(PRIO_PROCESS, 0 as id_t, (*details).priority) != 0 {
                sudo_warn!(
                    b"unable to set process priority %s\0" as *const u8 as *const libc::c_char,
                );
                break 'done;
            }
        }
        /* Policy may override umask in PAM or login.conf. */
        if ISSET!((*details).flags, CD_OVERRIDE_UMASK) != 0 {
            umask((*details).umask);
        }
        if !((*details).chroot).is_null() {
            if chroot((*details).chroot) != 0
                || chdir(b"/\0" as *const u8 as *const libc::c_char) != 0
            {
                sudo_warn!(b"unable to change root to %s\0" as *const u8 as *const libc::c_char,);
                break 'done;
            }
        }
        /*
         * Unlimit the number of processes since Linux's setuid() will
         * return EAGAIN if RLIMIT_NPROC would be exceeded by the uid switch.
         */
        unlimit_nproc();
        if setresuid((*details).uid, (*details).euid, (*details).euid) != 0 {
            sudo_warn!(
                b"unable to change to runas uid (%u, %u)\0" as *const u8 as *const libc::c_char,
                (*details).uid,
                (*details).euid
            );
            break 'done;
        }
        /* Restore previous value of RLIMIT_NPROC.*/
        restore_nproc();
        /*
         * Only change cwd if we have chroot()ed or the policy modules
         * specifies a different cwd.  Must be done after uid change.
         */
        if !((*details).cwd).is_null() {
            if !((*details).chroot).is_null()
                || (user_details.cwd).is_null()
                || strcmp((*details).cwd, user_details.cwd) != 0
            {
                /* Note: cwd is relative to the new root, if any. */
                if chdir((*details).cwd) != 0 {
                    sudo_warn!(
                        b"unable to change directory to %s" as *const u8 as *const libc::c_char,
                        (*details).cwd
                    );
                    break 'done;
                }
            }
        }
        ret = true;
        break;
    }
    debug_return_bool!(ret)
}





