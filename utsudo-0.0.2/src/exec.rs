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

/*
 * Setup the execution environment and execute the command.
 * If SELinux is enabled, run the command via sesh, otherwise
 * execute it directly.
 * If the exec fails, cstat is filled in with the value of errno.
 */
#[no_mangle]
pub unsafe extern "C" fn exec_cmnd(mut details: *mut command_details, mut errfd: libc::c_int) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    restore_signals();
    if exec_setup(details) == true {
        /* headed for execve() */
        if (*details).closefrom >= 0 {
            let mut fd: libc::c_int = 0;
            let mut maxfd: libc::c_int = 0;
            let mut debug_fds: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            /* Preserve debug fds and error pipe as needed.*/
            maxfd = sudo_debug_get_fds_v1(&mut debug_fds);
            while fd <= maxfd {
                if sudo_isset!(debug_fds, fd) != 0 {
                    add_preserved_fd(&mut (*details).preserved_fds, fd);
                }
                fd += 1;
            }
            if errfd != -1 {
                add_preserved_fd(&mut (*details).preserved_fds, errfd);
            }
            /* Close all fds except those explicitly preserved.*/
            closefrom_except((*details).closefrom, &mut (*details).preserved_fds);
        }
        if ISSET!((*details).flags, CD_RBAC_ENABLED) != 0 {
            selinux_execve(
                (*details).execfd,
                (*details).command,
                (*details).argv as *const *mut libc::c_char,
                (*details).envp,
                ISSET!((*details).flags, CD_NOEXEC) != 0,
            );
        } else {
            sudo_execve(
                (*details).execfd,
                (*details).command,
                (*details).argv as *const *mut libc::c_char,
                (*details).envp,
                ISSET!((*details).flags, CD_NOEXEC) != 0,
            );
        }
    }
    sudo_debug_printf!(
        SUDO_DEBUG_ERROR,
        b"unable to exec %s: %s\0" as *const u8 as *const libc::c_char,
        (*details).command,
        strerror(*__errno_location())
    );
    debug_return!();
}
/*
 * Check for caught signals sent to sudo before command execution.
 * Also suspends the process if SIGTSTP was caught.
 * Returns true if we should terminate, else false.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_terminated(mut cstat: *mut command_status) -> bool {
    let mut signo: libc::c_int = 0;
    let mut sigtstp: bool = false;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    while signo < NSIG as libc::c_int {
        if signal_pending(signo) {
            match signo {
                SIGCHLD => {
                    /* Ignore. */
                    break;
                }
                SIGTSTP => {
                    /* Suspend below if not terminated.*/
                    sigtstp = true;
                    break;
                }
                _ => {
                    /* Terminal signal, do not exec command.*/
                    (*cstat).type_0 = CMD_WSTATUS;
                    (*cstat).val = signo + 128;
                    debug_return_bool!(true);
                    break;
                }
            }
        }
        signo += 1;
    }
    if sigtstp {
        let mut sa: sigaction = sigaction {
            __sigaction_handler: Signal_handler { sa_handler: None },
            sa_mask: __sigset_t {
                __val: [0; _SIGSET_NWORDS as usize],
            },
            sa_flags: 0,
            sa_restorer: None,
        };
        let mut set: sigset_t = __sigset_t {
            __val: [0; _SIGSET_NWORDS as usize],
        };
        let mut oset: sigset_t = __sigset_t {
            __val: [0; _SIGSET_NWORDS as usize],
        };
        /* Send SIGTSTP to ourselves, unblocking it if needed.*/
        memset(
            &mut sa as *mut sigaction as *mut libc::c_void,
            0,
            std::mem::size_of::<sigaction>() as libc::c_ulong,
        );
        sigemptyset(&mut sa.sa_mask);
        sa.sa_flags = SA_RESTART;
        sa.__sigaction_handler.sa_handler = None;
        if sudo_sigaction(SIGTSTP, &mut sa, 0 as *mut sigaction) != 0 {
            sudo_warn!(
                b"unable to set handler for signal %d" as *const u8 as *const libc::c_char,
                SIGTSTP
            );
        }
        sigemptyset(&mut set);
        sigaddset(&mut set, SIGTSTP);
        sigprocmask(SIG_UNBLOCK, &mut set, &mut oset);
        if kill(getpid(), SIGTSTP) != 0 {
            sudo_warn!(
                b"kill(%d, SIGTSTP)" as *const u8 as *const libc::c_char,
                getpid()
            );
        }
        sigprocmask(SIG_SETMASK, &mut oset, 0 as *mut sigset_t);
        /* No need to restore old SIGTSTP handler. */
    }
    debug_return_bool!(false)
}
unsafe extern "C" fn sudo_needs_pty(mut details: *mut command_details) -> bool {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    if ISSET!((*details).flags, CD_USE_PTY) != 0 {
        return true;
    }
    plugin = io_plugins.tqh_first;
    while !plugin.is_null() {
        if ((*(*plugin).u.io).log_ttyin).is_some()
            || ((*(*plugin).u.io).log_ttyout).is_some()
            || ((*(*plugin).u.io).log_stdin).is_some()
            || ((*(*plugin).u.io).log_stdout).is_some()
            || ((*(*plugin).u.io).log_stderr).is_some()
            || ((*(*plugin).u.io).change_winsize).is_some()
            || ((*(*plugin).u.io).log_suspend).is_some()
        {
            return true;
        }
        plugin = (*plugin).entries.tqe_next;
    }
    return false;
}

/*
 * Execute a command, potentially in a pty with I/O loggging, and
 * wait for it to finish.
 * This is a little bit tricky due to how POSIX job control works and
 * we fact that we have two different controlling terminals to deal with.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_execute(
    mut details: *mut command_details,
    mut cstat: *mut command_status,
) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* If running in background mode, fork and exit.*/
    if ISSET!((*details).flags, CD_BACKGROUND) != 0 {
        match sudo_debug_fork_v1() {
            -1 => {
                (*cstat).type_0 = CMD_ERRNO;
                (*cstat).val = *__errno_location();
                debug_return_int!(-(1 as libc::c_int));
            }
            0 => {
                /* child continues without controlling terminal */
                setpgid(0 as pid_t, 0 as pid_t);
            }
            _ => {
                /* parent exits (but does not flush buffers) */
                sudo_debug_exit_int_v1(
                    (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"sudo_execute\0"))
                        .as_ptr(),
                    b"exec.c\0" as *const u8 as *const libc::c_char,
                    line!() as libc::c_int,
                    sudo_debug_subsys,
                    0,
                );
                _exit(0);
            }
        }
    }
    /*
     * Restore resource limits before running.
     * We must do this *before* calling the PAM session module.
     */
    restore_limits();
    'done: loop {
        /*
         * Run the command in a new pty if there is an I/O plugin or the policy
         * has requested a pty.  If /dev/tty is unavailable and no I/O plugin
         * is configured, this returns false and we run the command without a pty.
         */
        if sudo_needs_pty(details) {
            if exec_pty(details, cstat) {
                break 'done;
            }
        }
        /*
         * If we are not running the command in a pty, we were not invoked
         * as sudoedit, there is no command timeout and there is no close
         * function, just exec directly.  Only returns on error.
         */
        if ISSET!((*details).flags, CD_SET_TIMEOUT | CD_SUDOEDIT) != 0
            && ((*policy_plugin.u.policy).close).is_none()
        {
            if sudo_terminated(cstat) {
                exec_cmnd(details, -1);
                (*cstat).type_0 = CMD_ERRNO;
                (*cstat).val = *__errno_location();
            }
            break 'done;
        }
        /*
         * Run the command in the existing tty (if any) and wait for it to finish.
         */
        exec_nopty(details, cstat);
        break 'done;
    }
    /* The caller will run any plugin close functions. */
    if (*cstat).type_0 == CMD_ERRNO {
        debug_return_int!(-(1 as libc::c_int))
    } else {
        debug_return_int!(0)
    }
}

/*
 * Kill command with increasing urgency.
 */
#[no_mangle]
pub unsafe extern "C" fn terminate_command(mut pid: pid_t, mut use_pgrp: bool) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* Avoid killing more than a single process or process group. */
    if pid <= 0 {
        debug_return!();
    }
    /*
     * Note that SIGCHLD will interrupt the sleep()
     */
    if use_pgrp {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"killpg %d SIGHUP\0" as *const u8 as *const libc::c_char,
            pid
        );
        killpg(pid, SIGHUP);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"killpg %d SIGTERM\0" as *const u8 as *const libc::c_char,
            pid
        );
        killpg(pid, SIGTERM);
        sleep(2 as libc::c_uint);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"killpg %d SIGKILL\0" as *const u8 as *const libc::c_char,
            pid
        );
        killpg(pid, SIGKILL);
    } else {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"kill %d SIGHUP\0" as *const u8 as *const libc::c_char,
            pid
        );
        kill(pid, SIGHUP);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"kill %d SIGTERM\0" as *const u8 as *const libc::c_char,
            pid
        );
        kill(pid, SIGTERM);
        sleep(2 as libc::c_uint);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"kill %d SIGKILL\0" as *const u8 as *const libc::c_char,
            pid
        );
        kill(pid, SIGKILL);
    }
    debug_return!();
}
