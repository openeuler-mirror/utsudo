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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct monitor_closure {
    pub cmnd_pid: pid_t,
    pub cmnd_pgrp: pid_t,
    pub mon_pgrp: pid_t,
    pub backchannel: libc::c_int,
    pub cstat: *mut command_status,
    pub evbase: *mut sudo_event_base,
    pub errpipe_event: *mut sudo_event,
    pub backchannel_event: *mut sudo_event,
    pub sigint_event: *mut sudo_event,
    pub sigquit_event: *mut sudo_event,
    pub sigtstp_event: *mut sudo_event,
    pub sigterm_event: *mut sudo_event,
    pub sighup_event: *mut sudo_event,
    pub sigusr1_event: *mut sudo_event,
    pub sigusr2_event: *mut sudo_event,
    pub sigchld_event: *mut sudo_event,
}

#[macro_export]
macro_rules! WIFCONTINUED {
    ($status: expr) => {
        ($status == 0xffff)
    };
}
static mut tty_initialized: bool = false;

unsafe extern "C" fn deliver_signal(
    mut mc: *mut monitor_closure,
    mut signo: libc::c_int,
    mut from_parent: bool,
) {
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* Avoid killing more than a single process or process group. */
    if (*mc).cmnd_pid <= 0 {
        debug_return!();
    }
    if signo == SIGCONT_FG {
        sudo_strlcpy(
            signame.as_mut_ptr(),
            b"CONT_FG\0" as *const u8 as *const libc::c_char,
            std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    } else if signo == SIGCONT_BG {
        sudo_strlcpy(
            signame.as_mut_ptr(),
            b"CONT_BG\0" as *const u8 as *const libc::c_char,
            std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    } else if sudo_sig2str(signo, signame.as_mut_ptr()) == -(1 as libc::c_int) {
        snprintf(
            signame.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            signo,
        );
    }
    /* Handle signal from parent or monitor.*/
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"received SIG%s%s\0" as *const u8 as *const libc::c_char,
        signame.as_mut_ptr(),
        if from_parent as libc::c_int != 0 {
            b" from parent\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }
    );
    match signo {
        SIGALRM => {
            terminate_command((*mc).cmnd_pid, true);
        }
        SIGCONT_FG => {
            /* Continue in foreground, grant it controlling tty. */
            if tcsetpgrp(io_fds[SFD_SLAVE as usize], (*mc).cmnd_pgrp) == -(1 as libc::c_int) {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO,
                    b"%s: unable to set foreground pgrp to %d (command)\0" as *const u8
                        as *const libc::c_char,
                    stdext::function_name!().as_ptr(),
                    (*mc).cmnd_pgrp
                );
            }
            /* Lazily initialize the pty if needed. */
            if !tty_initialized {
                if sudo_term_copy_v1(io_fds[SFD_USERTTY as usize], io_fds[SFD_SLAVE as usize]) {
                    tty_initialized = true;
                }
            }
            killpg((*mc).cmnd_pid, SIGCONT);
        }
        SIGCONT_BG => {
            /* Continue in background, I take controlling tty. */
            if tcsetpgrp(io_fds[SFD_SLAVE as usize], (*mc).mon_pgrp) == -(1 as libc::c_int) {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO,
                    b"%s: unable to set foreground pgrp to %d (monitor)\0" as *const u8
                        as *const libc::c_char,
                    stdext::function_name!().as_ptr(),
                    (*mc).mon_pgrp
                );
            }
            killpg((*mc).cmnd_pid, SIGCONT);
        }
        SIGKILL => {
            _exit(1); /* XXX */
            killpg((*mc).cmnd_pid, signo);
        }
        /* NOTREACHED */
        _ => {
            /* Relay signal to command. */
            killpg((*mc).cmnd_pid, signo);
        }
    }
    debug_return!();
}

unsafe extern "C" fn handle_winch(mut mc: *mut monitor_closure, mut wsize_packed: libc::c_uint) {
    let mut wsize: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut owsize: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* Rows and colums are stored as two shorts packed into a single int. */
    wsize.ws_row = (wsize_packed & 0xffff as libc::c_uint) as libc::c_ushort;
    wsize.ws_col = (wsize_packed >> 16 & 0xffff as libc::c_uint) as libc::c_ushort;
    if ioctl(
        io_fds[SFD_SLAVE as usize],
        TIOCGWINSZ as libc::c_ulong,
        &mut owsize as *mut winsize,
    ) == 0
        && (wsize.ws_row != owsize.ws_row || wsize.ws_col != owsize.ws_col)
    {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"window size change %dx%d -> %dx%d\0" as *const u8 as *const libc::c_char,
            owsize.ws_col as libc::c_int,
            owsize.ws_row as libc::c_int,
            wsize.ws_col as libc::c_int,
            wsize.ws_row as libc::c_int
        );
        ioctl(
            io_fds[SFD_SLAVE as usize],
            TIOCSWINSZ as libc::c_ulong,
            &mut wsize as *mut winsize,
        );
        deliver_signal(mc, SIGWINCH, true);
    }
    debug_return!();
}

unsafe extern "C" fn send_status(
    mut fd: libc::c_int,
    mut cstat: *mut command_status,
) -> libc::c_int {
    let mut n: libc::c_int = -(1 as libc::c_int);
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    if (*cstat).type_0 != CMD_INVALID {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"sending status message to parent: [%d, %d]\0" as *const u8 as *const libc::c_char,
            (*cstat).type_0,
            (*cstat).val
        );
        n = send(
            fd,
            cstat as *const libc::c_void,
            std::mem::size_of::<command_status>() as libc::c_ulong,
            0,
        ) as libc::c_int;
        if n as libc::c_ulong != std::mem::size_of::<command_status>() as libc::c_ulong {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO,
                b"%s: unable to send status to parent\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr()
            );
        }
        (*cstat).type_0 = CMD_INVALID; /* prevent re-sending */
    }
    debug_return_int!(n);
}

unsafe extern "C" fn mon_handle_sigchld(mut mc: *mut monitor_closure) {
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* Read command status. */
    loop {
        pid = waitpid(
            (*mc).cmnd_pid,
            &mut status,
            WUNTRACED | WCONTINUED | WNOHANG,
        );
        if !(pid == -(1 as libc::c_int) && errno!() == EINTR) {
            break;
        }
        break;
    }
    match pid {
        0 => {
            errno!() = ECHILD;
            sudo_warn!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                b"waitpid\0" as *const u8 as *const libc::c_char
            );
            debug_return!();
        }
        /* FALLTHROUGH */
        -1 => {
            sudo_warn!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                b"waitpid\0" as *const u8 as *const libc::c_char
            );
            debug_return!();
        }
        _ => {}
    }
    if WIFCONTINUED!(status) {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: command (%d) resumed\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*mc).cmnd_pid
        );
    } else if WIFSTOPPED!(status) {
        if sudo_sig2str(WSTOPSIG!(status), signame.as_mut_ptr()) == -(1 as libc::c_int) {
            snprintf(
                signame.as_mut_ptr(),
                std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                WSTOPSIG!(status),
            );
        }
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: command (%d) stopped, SIG%s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*mc).cmnd_pid,
            signame.as_mut_ptr()
        );
    } else if WIFSIGNALED!(status) {
        if sudo_sig2str(WTERMSIG!(status), signame.as_mut_ptr()) == -(1 as libc::c_int) {
            snprintf(
                signame.as_mut_ptr(),
                std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                WTERMSIG!(status),
            );
        }
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: command (%d) killed, SIG%s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*mc).cmnd_pid,
            signame.as_mut_ptr()
        );
        (*mc).cmnd_pid == -(1 as libc::c_int);
    } else if WIFEXITED!(status) {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: command (%d) exited: %d\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*mc).cmnd_pid,
            WEXITSTATUS!(status)
        );
        (*mc).cmnd_pid == -(1 as libc::c_int);
    } else {
        sudo_debug_printf!(
            SUDO_DEBUG_WARN,
            b"%s: unexpected wait status %d for command (%d)" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            status,
            (*mc).cmnd_pid
        );
    }
    /* Don't overwrite execve() failure with child exit status. */
    if (*(*mc).cstat).type_0 == CMD_INVALID {
        /*
         * Store wait status in cstat and forward to parent if stopped.
         * Parent does not expect SIGCONT so don't bother sending it.
         */
        if !WIFCONTINUED!(status) {
            (*(*mc).cstat).type_0 = CMD_WSTATUS;
            (*(*mc).cstat).val = status;
            if WIFSTOPPED!(status) {
                /* Save the foreground pgid so we can restore it later. */
                pid = tcgetpgrp(io_fds[SFD_SLAVE as usize]);
                if pid != (*mc).mon_pgrp {
                    (*mc).cmnd_pgrp = pid;
                }
                send_status((*mc).backchannel, (*mc).cstat);
            }
        }
    } else {
        sudo_debug_printf!(
            SUDO_DEBUG_WARN,
            b"%s: not overwriting command status %d,%d with %d,%d\0" as *const u8
                as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*(*mc).cstat).type_0,
            (*(*mc).cstat).val,
            CMD_WSTATUS,
            status
        );
    }
    debug_return!();
}


unsafe extern "C" fn mon_signal_cb(
    mut signo: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut sc: *mut sudo_ev_siginfo_container = v as *mut sudo_ev_siginfo_container;
    let mut mc: *mut monitor_closure = (*sc).closure as *mut monitor_closure;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /*
     * Handle SIGCHLD specially and deliver other signals
     * directly to the command.
     */
    if signo == SIGCHLD {
        mon_handle_sigchld(mc);
        if (*mc).cmnd_pid == -(1 as libc::c_int) {
            /* Command exited or was killed, exit event loop. */
            sudo_ev_loopexit_v1((*mc).evbase);
        }
    } else {
        /*
         * If the signal came from the process group of the command we ran,
         * do not forward it as we don't want the child to indirectly kill
         * itself.  This can happen with, e.g., BSD-derived versions of
         * reboot that call kill(-1, SIGTERM) to kill all other processes.
         */
        if USER_SIGNALED!((*sc).siginfo) && (*(*sc).siginfo)._sifields._kill.si_pid != 0 {
            let mut si_pgrp: pid_t = getpgid((*(*sc).siginfo)._sifields._kill.si_pid);
            if si_pgrp != -(1 as libc::c_int) {
                if si_pgrp == (*mc).cmnd_pgrp {
                    debug_return!();
                }
            } else if (*(*sc).siginfo)._sifields._kill.si_pid == (*mc).cmnd_pid {
                debug_return!();
            }
        }
        deliver_signal(mc, signo, false);
    }
    debug_return!();
}
/* Note: this is basically the same as errpipe_cb() in exec_nopty.c */
unsafe extern "C" fn mon_errpipe_cb(
    mut fd: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut mc: *mut monitor_closure = v as *mut monitor_closure;
    let mut nread: ssize_t = 0;
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
            if errno!() != EAGAIN && errno!() != EINTR {
                if (*(*mc).cstat).val == CMD_INVALID {
                    /* XXX - need a way to distinguish non-exec error. */
                    (*(*mc).cstat).type_0 = CMD_ERRNO;
                    (*(*mc).cstat).val = errno!();
                }
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO,
                    b"%s: failed to read error pipe\0" as *const u8 as *const libc::c_char,
                    stdext::function_name!().as_ptr()
                );
                sudo_ev_loopbreak_v1((*mc).evbase);
            }
        }
        _ => {
            if nread == 0 as libc::c_long {
                /* The error pipe closes when the command is executed. */
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"EOF on error pipe\0" as *const u8 as *const libc::c_char
                );
            } else {
                /* Errno value when child is unable to execute command. */
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"errno from child: %s\0" as *const u8 as *const libc::c_char,
                    strerror(errval)
                );
                (*(*mc).cstat).type_0 = CMD_ERRNO;
                (*(*mc).cstat).val = errval;
            }
            sudo_ev_del_v1((*mc).evbase, (*mc).errpipe_event);
            close(fd);
        }
    }
    debug_return!();
}






