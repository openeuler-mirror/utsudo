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
use crate::errno;
use crate::struct_macro::*;
use crate::ISSET;
use crate::SIG_IGN;
use crate::USER_SIGNALED;
use crate::WEXITSTATUS;
use crate::WIFEXITED;
use crate::WIFSIGNALED;
use crate::WIFSTOPPED;
use crate::WSTOPSIG;
use crate::WTERMSIG;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

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

unsafe extern "C" fn mon_backchannel_cb(
    mut fd: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut mc: *mut monitor_closure = v as *mut monitor_closure;
    let mut cstmp: command_status = command_status { type_0: 0, val: 0 };
    let mut n: ssize_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /*
     * Read command from backchannel, should be a signal.
     * Note that the backchannel is a *blocking* socket.
     */
    n = recv(
        fd,
        &mut cstmp as *mut command_status as *mut libc::c_void,
        std::mem::size_of::<command_status>() as libc::c_ulong,
        MSG_WAITALL as libc::c_int,
    );
    if n as libc::c_ulong != std::mem::size_of::<command_status>() as libc::c_ulong {
        if n == -(1 as libc::c_int) as libc::c_long {
            if errno!() == EINTR || errno!() == EAGAIN {
                debug_return!();
            }
            sudo_warn!(b"error reading from socketpair\0" as *const u8 as *const libc::c_char,);
        } else {
            /* short read or EOF, parent process died? */
        }
        /* XXX - need a way to distinguish non-exec error. */
        (*(*mc).cstat).type_0 = CMD_ERRNO;
        (*(*mc).cstat).val = if n != 0 { EIO } else { ECONNRESET };
        sudo_ev_loopbreak_v1((*mc).evbase);
    } else {
        match cstmp.type_0 {
            CMD_TTYWINCH => {
                handle_winch(mc, cstmp.val as libc::c_uint);
            }
            CMD_SIGNO => {
                deliver_signal(mc, cstmp.val, true);
            }
            _ => {
                sudo_warnx!(
                    b"unexpected reply type on backchannel: %d\0" as *const u8
                        as *const libc::c_char,
                    cstmp.type_0
                );
            }
        }
    }
    debug_return!();
}

unsafe extern "C" fn exec_cmnd_pty(
    mut details: *mut command_details,
    mut foreground: bool,
    mut errfd: libc::c_int,
) {
    let mut self_0: pid_t = getpid();
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* Register cleanup function */
    sudo_fatal_callback_register_v1(Some(pty_cleanup as unsafe extern "C" fn() -> ()));
    /* Set command process group here too to avoid a race.*/
    setpgid(0, self_0);
    /* Wire up standard fds, note that stdout/stderr may be pipes. */
    if io_fds[SFD_STDIN as usize] != STDIN_FILENO {
        if dup2(io_fds[SFD_STDIN as usize], STDIN_FILENO) == -(1 as libc::c_int) {
            sudo_fatal!(b"dup2 \0" as *const u8 as *const libc::c_char,);
        }
        if io_fds[SFD_STDIN as usize] != io_fds[SFD_SLAVE as usize] {
            close(io_fds[SFD_STDIN as usize]);
        }
    }
    if io_fds[SFD_STDOUT as usize] != STDOUT_FILENO {
        if dup2(io_fds[SFD_STDOUT as usize], STDOUT_FILENO) == -(1 as libc::c_int) {
            sudo_fatal!(b"dup2 \0" as *const u8 as *const libc::c_char,);
        }
        if io_fds[SFD_STDOUT as usize] != io_fds[SFD_SLAVE as usize] {
            close(io_fds[SFD_STDOUT as usize]);
        }
    }
    if io_fds[SFD_STDERR as usize] != STDERR_FILENO {
        if dup2(io_fds[SFD_STDERR as usize], STDERR_FILENO) == -(1 as libc::c_int) {
            sudo_fatal!(b"dup2 \0" as *const u8 as *const libc::c_char,);
        }
        if io_fds[SFD_STDERR as usize] != io_fds[SFD_SLAVE as usize] {
            close(io_fds[SFD_STDERR as usize]);
        }
    }
    /* Wait for parent to grant us the tty if we are foreground. */
    if foreground as libc::c_int != 0 && ISSET!((*details).flags, CD_EXEC_BG) == 0 {
        /* 1us */
        let mut ts: timespec = {
            let mut init = timespec {
                tv_sec: 0 as __time_t,
                tv_nsec: 1000 as __syscall_slong_t,
            };
            init
        };
        sudo_debug_printf!(
            SUDO_DEBUG_DEBUG,
            b"%s: waiting for controlling tty\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr()
        );
        while tcgetpgrp(io_fds[SFD_SLAVE as usize]) != self_0 {
            nanosleep(&mut ts, 0 as *mut timespec);
        }
        sudo_debug_printf!(
            SUDO_DEBUG_DEBUG,
            b"%s: got controlling tty\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr()
        );
    }
    /* Done with the pty slave, don't leak it. */
    if io_fds[SFD_SLAVE as usize] != -(1 as libc::c_int) {
        close(io_fds[SFD_SLAVE as usize]);
    }
    /* Execute command; only returns on error. */
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"executing %s in the %s\0" as *const u8 as *const libc::c_char,
        (*details).command,
        if foreground as libc::c_int != 0 {
            b"foreground\0" as *const u8 as *const libc::c_char
        } else {
            b"background\0" as *const u8 as *const libc::c_char
        }
    );
    exec_cmnd(details, errfd);
    debug_return!();
}

unsafe extern "C" fn fill_exec_closure_monitor(
    mut mc: *mut monitor_closure,
    mut cstat: *mut command_status,
    mut errfd: libc::c_int,
    mut backchannel: libc::c_int,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* Fill in the non-event part of the closure. */
    let ref mut cstat0 = (*mc).cstat;
    *cstat0 = cstat;
    (*mc).backchannel = backchannel;
    (*mc).mon_pgrp = getpgrp();
    /* Setup event base and events. */
    let ref mut evbase0 = (*mc).evbase;
    *evbase0 = sudo_ev_base_alloc_v1();
    if ((*mc).evbase).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    /* Event for command status via errfd. */
    let ref mut errpipe_event0 = (*mc).errpipe_event;
    *errpipe_event0 = sudo_ev_alloc_v1(
        errfd,
        (SUDO_EV_READ | SUDO_EV_PERSIST) as libc::c_short,
        Some(
            mon_errpipe_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).errpipe_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).errpipe_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    /* Event for forwarded signals via backchannel. */
    let ref mut backchannel_event0 = (*mc).errpipe_event;
    *backchannel_event0 = sudo_ev_alloc_v1(
        backchannel,
        (SUDO_EV_READ | SUDO_EV_PERSIST) as libc::c_short,
        Some(
            mon_backchannel_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).backchannel_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2(
        (*mc).evbase,
        (*mc).backchannel_event,
        0 as *mut timespec,
        false,
    ) == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    /* Events for local signals. */
    let ref mut sigint_event0 = (*mc).errpipe_event;
    *sigint_event0 = sudo_ev_alloc_v1(
        SIGINT,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sigint_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sigint_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sigquit_event0 = (*mc).errpipe_event;
    *sigquit_event0 = sudo_ev_alloc_v1(
        SIGQUIT,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sigquit_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sigquit_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sigtstp_event0 = (*mc).sigtstp_event;
    *sigtstp_event0 = sudo_ev_alloc_v1(
        SIGTSTP,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sigtstp_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sigtstp_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sigterm_event0 = (*mc).sigterm_event;
    *sigterm_event0 = sudo_ev_alloc_v1(
        SIGTERM,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sigterm_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sigterm_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sighup_event0 = (*mc).sighup_event;
    *sighup_event0 = sudo_ev_alloc_v1(
        SIGHUP,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sighup_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sighup_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sigusr1_event0 = (*mc).sigusr1_event;
    *sigusr1_event0 = sudo_ev_alloc_v1(
        SIGUSR1,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sigusr1_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sigusr1_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sigusr2_event0 = (*mc).sigusr2_event;
    *sigusr2_event0 = sudo_ev_alloc_v1(
        SIGUSR2,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sigusr2_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sigusr2_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sigchld_event0 = (*mc).sigchld_event;
    *sigchld_event0 = sudo_ev_alloc_v1(
        SIGCHLD,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            mon_signal_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        mc as *mut libc::c_void,
    );
    if ((*mc).sigchld_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*mc).evbase, (*mc).sigchld_event, 0 as *mut timespec, false)
        == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }
    /* Clear the default event base. */
    sudo_ev_base_setdef_v1(0 as *mut sudo_event_base);
    debug_return!();
}

#[no_mangle]
pub unsafe extern "C" fn exec_monitor(
    mut details: *mut command_details,
    mut oset: *mut sigset_t,
    mut foreground: bool,
    mut backchannel: libc::c_int,
) -> libc::c_int {
    let mut mc: monitor_closure = {
        let mut init = monitor_closure {
            cmnd_pid: 0 as libc::c_int,
            cmnd_pgrp: 0,
            mon_pgrp: 0,
            backchannel: 0,
            cstat: 0 as *mut command_status,
            evbase: 0 as *mut sudo_event_base,
            errpipe_event: 0 as *mut sudo_event,
            backchannel_event: 0 as *mut sudo_event,
            sigint_event: 0 as *mut sudo_event,
            sigquit_event: 0 as *mut sudo_event,
            sigtstp_event: 0 as *mut sudo_event,
            sigterm_event: 0 as *mut sudo_event,
            sighup_event: 0 as *mut sudo_event,
            sigusr1_event: 0 as *mut sudo_event,
            sigusr2_event: 0 as *mut sudo_event,
            sigchld_event: 0 as *mut sudo_event,
        };
        init
    };
    let mut cstat: command_status = command_status { type_0: 0, val: 0 };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut errpipe: [libc::c_int; 2] = [0; 2];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* The pty master is not used by the monitor. */
    if io_fds[SFD_MASTER as usize] != -(1 as libc::c_int) {
        close(io_fds[SFD_MASTER as usize]);
    }
    /* Ignore any SIGTTIN or SIGTTOU we receive (shouldn't be possible). */
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0,
        std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    sa.__sigaction_handler.sa_handler = SIG_IGN!();
    if sudo_sigaction(SIGTTIN, &mut sa, 0 as *mut sigaction) != 0 {
        sudo_warn!(
            b"unable to set handler for signal %d\0" as *const u8 as *const libc::c_char,
            SIGTTIN
        );
    }
    if sudo_sigaction(SIGTTOU, &mut sa, 0 as *mut sigaction) != 0 {
        sudo_warn!(
            b"unable to set handler for signal %d\0" as *const u8 as *const libc::c_char,
            SIGTTOU
        );
    }
    /* If we are starting in the foreground, the pty was already initialized. */
    if foreground {
        tty_initialized = true;
    }
    /*
     * Start a new session with the parent as the session leader
     * and the slave pty as the controlling terminal.
     * This allows us to be notified when the command has been suspended.
     */
    'bad: loop {
        if setsid() == -(1 as libc::c_int) {
            sudo_warn!(b"setsid\0" as *const u8 as *const libc::c_char,);
            break 'bad;
        }
        if pty_make_controlling() == -(1 as libc::c_int) {
            sudo_warn!(b"unable to set controlling tty\0" as *const u8 as *const libc::c_char,);
            break 'bad;
        }
        /*
         * We use a pipe to get errno if execve(2) fails in the child.
         */
        if pipe2(errpipe.as_mut_ptr(), O_CLOEXEC as libc::c_int) != 0 {
            sudo_fatal!(b"unable to create pipe\0" as *const u8 as *const libc::c_char,);
        }
        /*
         * Before forking, wait for the main sudo process to tell us to go.
         * Avoids race conditions when the command exits quickly.
         */
        while recv(
            backchannel,
            &mut cstat as *mut command_status as *mut libc::c_void,
            std::mem::size_of::<command_status>() as libc::c_ulong,
            MSG_WAITALL as libc::c_int,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            if errno!() != EINTR && errno!() != EAGAIN {
                sudo_fatal!(
                    b"unable to receive message from parent\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        mc.cmnd_pid = sudo_debug_fork_v1();
        match mc.cmnd_pid {
            -1 => {
                sudo_warn!(b"unable to fork\0" as *const u8 as *const libc::c_char,);
                break 'bad;
            }
            0 => {
                /* child */
                sigprocmask(SIG_SETMASK, oset, 0 as *mut sigset_t);
                close(backchannel);
                close(errpipe[0 as usize]);
                if io_fds[SFD_USERTTY as usize] != -(1 as libc::c_int) {
                    close(io_fds[SFD_USERTTY as usize]);
                }
                restore_signals();
                /* setup tty and exec command */
                exec_cmnd_pty(details, foreground, errpipe[1 as usize]);
                if write(
                    errpipe[1 as usize],
                    __errno_location() as *const libc::c_void,
                    std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) == -(1 as libc::c_int) as libc::c_long
                {
                    sudo_warn!(
                        b"unable to execute %s\0" as *const u8 as *const libc::c_char,
                        (*details).command
                    );
                    _exit(1);
                }
            }
            _ => {}
        }
        close(errpipe[1 as usize]);
        /* No longer need execfd. */
        if (*details).execfd != -(1 as libc::c_int) {
            close((*details).execfd);
            (*details).execfd = -(1 as libc::c_int);
        }
        /* Send the command's pid to main sudo process. */
        cstat.type_0 = CMD_PID;
        cstat.val = mc.cmnd_pid;
        send_status(backchannel, &mut cstat);
        /*
         * Create new event base and register read events for the
         * signal pipe, error pipe, and backchannel.
         */
        fill_exec_closure_monitor(&mut mc, &mut cstat, errpipe[0 as usize], backchannel);
        /* Restore signal mask now that signal handlers are setup. */
        sigprocmask(SIG_SETMASK, oset, 0 as *mut sigset_t);
        /* If any of stdin/stdout/stderr are pipes, close them in parent. */
        if io_fds[SFD_STDIN as usize] != io_fds[SFD_SLAVE as usize] {
            close(io_fds[SFD_STDIN as usize]);
        }
        if io_fds[SFD_STDOUT as usize] != io_fds[SFD_SLAVE as usize] {
            close(io_fds[SFD_STDOUT as usize]);
        }
        if io_fds[SFD_STDERR as usize] != io_fds[SFD_SLAVE as usize] {
            close(io_fds[SFD_STDERR as usize]);
        }
        /* Put command in its own process group. */
        mc.cmnd_pgrp = mc.cmnd_pid;
        setpgid(mc.cmnd_pid, mc.cmnd_pgrp);
        /* Make the command the foreground process for the pty slave. */
        if (foreground as libc::c_int != 0)
            && ISSET!((*details).flags, CD_EXEC_BG) as libc::c_int == 0
        {
            if tcsetpgrp(io_fds[SFD_SLAVE as usize], mc.cmnd_pgrp) == -(1 as libc::c_int) {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO,
                    b"%s: unable to set foreground pgrp to %d (command)\0" as *const u8
                        as *const libc::c_char,
                    stdext::function_name!().as_ptr(),
                    mc.cmnd_pgrp
                )
            }
        }
        /*
         * Wait for errno on pipe, signal on backchannel or for SIGCHLD.
         * The event loop ends when the child is no longer running and
         * the error pipe is closed.
         */
        cstat.type_0 = CMD_INVALID;
        cstat.val = 0;
        sudo_ev_dispatch_v1(mc.evbase);
        if mc.cmnd_pid != -(1 as libc::c_int) {
            let mut pid: pid_t = 0;
            /* Command still running, did the parent die? */
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR,
                b"Command still running after event loop exit, terminating\0" as *const u8
                    as *const libc::c_char
            );
            terminate_command(mc.cmnd_pid, true);
            loop {
                pid = waitpid(mc.cmnd_pid, 0 as *mut libc::c_int, 0);
                if !(pid == -(1 as libc::c_int) && errno!() == EINTR) {
                    break;
                }
                /* XXX - update cstat with wait status? */
            }
        }
        /*
         * Take the controlling tty.  This prevents processes spawned by the
         * command from receiving SIGHUP when the session leader (us) exits.
         */
        if tcsetpgrp(io_fds[SFD_SLAVE as usize], mc.mon_pgrp) == -1 {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO,
                b"%s: unable to set foreground pgrp to %d (monitor)\0" as *const u8
                    as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                mc.mon_pgrp
            )
        }
        /* Send parent status. */
        send_status(backchannel, &mut cstat);
        sudo_debug_exit_int_v1(
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"exec_nopty\0")).as_ptr(),
            b"exec_nopty.c\0" as *const u8 as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys,
            1,
        );
        _exit(1);
        break;
    }
    debug_return_int!(-(1 as libc::c_int));
}
