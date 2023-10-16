/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_variables, clashing_extern_declarations)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use crate::ISSET;
use crate::USER_SIGNALED;
use crate::WEXITSTATUS;
use crate::WIFSIGNALED;
use crate::WIFSTOPPED;
use crate::WSTOPSIG;
use crate::WTERMSIG;
use crate::_PATH_TTY;
extern "C" {
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn sudo_sig2str(signo: libc::c_int, signame: *mut libc::c_char) -> libc::c_int;
    fn snprintf(
        __s: *mut libc::c_char,
        __n: size_t,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sudo_ev_loopexit_v1(base: *mut sudo_event_base);
    fn getpgid(__pid: __pid_t) -> __pid_t;
    fn terminate_command(pid: pid_t, use_pgrp: bool);
    fn getpgrp() -> __pid_t;
    fn sudo_ev_base_alloc_v1() -> *mut sudo_event_base;
    fn sudo_ev_alloc_v1(
        fd: libc::c_int,
        event: libc::c_short,
        callback: sudo_ev_callback_t,
        closure: *mut libc::c_void,
    ) -> *mut sudo_event;
    fn sudo_ev_add_v2(
        head: *mut sudo_event_base,
        ev: *mut sudo_event,
        timo: *mut timespec,
        tohead: bool,
    ) -> libc::c_int;
    fn selinux_setup(
        role: *const libc::c_char,
        type_0: *const libc::c_char,
        ttyn: *const libc::c_char,
        ttyfd: libc::c_int,
        label_tty: bool,
    ) -> libc::c_int;
    fn sudo_ev_base_setdef_v1(base: *mut sudo_event_base);
    fn sudo_ev_base_free_v1(base: *mut sudo_event_base);
    fn sudo_ev_free_v1(ev: *mut sudo_event);
    fn sudo_ev_dispatch_v1(head: *mut sudo_event_base) -> libc::c_int;
    fn sudo_ev_got_break_v1(base: *mut sudo_event_base) -> bool;
    fn sudo_ev_del_v1(head: *mut sudo_event_base, ev: *mut sudo_event) -> libc::c_int;
    fn policy_init_session(details: *mut command_details) -> libc::c_int;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sudo_terminated(cstat: *mut command_status) -> bool;
    fn sudo_debug_fork_v1() -> pid_t;
    fn exec_cmnd(details: *mut command_details, errfd: libc::c_int);
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn _exit(__status: libc::c_int) -> !;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
    fn tcsetpgrp_nobg(fd: libc::c_int, pgrp_id: pid_t) -> libc::c_int;
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sudo_ev_loopbreak_v1(base: *mut sudo_event_base);
    fn sudo_sigaction(signo: libc::c_int, sa: *mut sigaction, osa: *mut sigaction) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_debug_exit_int_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: libc::c_int,
    );
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...);
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct exec_closure_nopty {
    pub cmnd_pid: pid_t,
    pub ppgrp: pid_t,
    pub cstat: *mut command_status,
    pub details: *mut command_details,
    pub evbase: *mut sudo_event_base,
    pub errpipe_event: *mut sudo_event,
    pub sigint_event: *mut sudo_event,
    pub sigquit_event: *mut sudo_event,
    pub sigtstp_event: *mut sudo_event,
    pub sigterm_event: *mut sudo_event,
    pub sighup_event: *mut sudo_event,
    pub sigalrm_event: *mut sudo_event,
    pub sigpipe_event: *mut sudo_event,
    pub sigusr1_event: *mut sudo_event,
    pub sigusr2_event: *mut sudo_event,
    pub sigchld_event: *mut sudo_event,
    pub sigcont_event: *mut sudo_event,
    pub siginfo_event: *mut sudo_event,
}
#[inline]
/* Note: this is basically the same as mon_errpipe_cb() in exec_monitor.c */
unsafe extern "C" fn errpipe_cb(
    mut fd: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut ec: *mut exec_closure_nopty = v as *mut exec_closure_nopty;
    let mut nread: ssize_t = 0 as ssize_t;
    let mut errval: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /*
     * Read errno from child or EOF when command is executed.
     * Note that the error pipe is *blocking*.
     */
    nread = read(
        fd,
        &mut errval as *mut libc::c_int as *mut libc::c_void,
        std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    match nread {
        -1 => {
            if *__errno_location() != EAGAIN && *__errno_location() != EINTR {
                if (*(*ec).cstat).val == CMD_INVALID {
                    /* XXX - need a way to distinguish non-exec error. */
                    (*(*ec).cstat).type_0 = CMD_ERRNO;
                    (*(*ec).cstat).val = *__errno_location();
                }
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO,
                    b"%s: failed to read error pipe\0" as *const u8 as *const libc::c_char,
                    stdext::function_name!().as_ptr()
                );
                sudo_ev_loopbreak_v1((*ec).evbase);
            }
        }
        _ => {
            if nread == 0 as ssize_t {
                /* The error pipe closes when the command is executed. */
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"EOF on error pipe\0" as *const u8 as *const libc::c_char
                );
            } else {
                /* Errno value when child is unable to execute command. */
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"errno from child: %s\0" as *const u8 as *const libc::c_char
                );
                (*(*ec).cstat).type_0 = CMD_ERRNO;
                (*(*ec).cstat).val = errval;
            }
            sudo_ev_del_v1((*ec).evbase, (*ec).errpipe_event);
            close(fd);
        }
    }
    debug_return!();
}
