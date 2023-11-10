/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_variables, unreachable_patterns, clashing_extern_declarations)]

use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

use crate::errno;
use crate::CLR;
use crate::ISSET;
use crate::SIG_IGN;
use crate::SUDO_API_MKVERSION;
use crate::USER_SIGNALED;
use crate::WEXITSTATUS;
use crate::WIFSIGNALED;
use crate::WIFSTOPPED;
use crate::WSTOPSIG;
use crate::WTERMSIG;
use crate::_PATH_TTY;
use crate::__S_ISTYPE;

//#define PATH_MAX        4096	/* # chars in a path name including nul */
pub const PATH_MAX: libc::c_int = 4096;

// #define TERM_COOKED	0
// #define TERM_RAW	1
pub const TERM_COOKED: libc::c_int = 0;
pub const TERM_RAW: libc::c_int = 1;

//#define TIOCSCTTY	0x540E
pub const TIOCSCTTY: libc::c_int = 0x540E;

//#define SUDO_EVLOOP_NONBLOCK	0x02	/* Do not block in event loop */
pub const SUDO_EVLOOP_NONBLOCK: libc::c_int = 0x02;

//#define FD_CLOEXEC	1	/* Actually anything with low bit set goes */
pub const FD_CLOEXEC: libc::c_int = 1;

// SOCK_STREAM = 1,		/* Sequenced, reliable, connection-based byte streams.  */
pub const SOCK_STREAM: libc::c_int = 1;

// #define PF_LOCAL	1	/* Local to host (pipes and file-domain).  */
// #define PF_UNIX		PF_LOCAL /* POSIX name for PF_LOCAL.  */
pub const PF_LOCAL: libc::c_int = 1;
pub const PF_UNIX: libc::c_int = PF_LOCAL;

//#define	__S_IFIFO	0010000	/* FIFO.  */
pub const __S_IFIFO: libc::c_int = 0o010000;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_buffer_list {
    pub slh_first: *mut io_buffer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_buffer {
    pub entries: SLIST_ENTRY_io_buffer,
    pub ec: *mut exec_closure_pty,
    pub revent: *mut sudo_event,
    pub wevent: *mut sudo_event,
    pub action: sudo_io_action_t,
    pub len: libc::c_int,
    pub off: libc::c_int,
    pub buf: [libc::c_char; 64 * 1024],
}

pub type sudo_io_action_t =
    Option<unsafe extern "C" fn(*const libc::c_char, libc::c_uint, *mut io_buffer) -> bool>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SLIST_ENTRY_io_buffer {
    pub sle_next: *mut io_buffer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct monitor_message_list {
    pub tqh_first: *mut monitor_message,
    pub tqh_last: *mut *mut monitor_message,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monitor_message {
    pub entries: TAILQ_ENTRY_monitor_message,
    pub cstat: command_status,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_monitor_message {
    pub tqe_next: *mut monitor_message,
    pub tqe_prev: *mut *mut monitor_message,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct exec_closure_pty {
    pub monitor_pid: pid_t,
    pub cmnd_pid: pid_t,
    pub ppgrp: pid_t,
    pub rows: libc::c_short,
    pub cols: libc::c_short,
    pub cstat: *mut command_status,
    pub details: *mut command_details,
    pub evbase: *mut sudo_event_base,
    pub backchannel_event: *mut sudo_event,
    pub fwdchannel_event: *mut sudo_event,
    pub sigint_event: *mut sudo_event,
    pub sigquit_event: *mut sudo_event,
    pub sigtstp_event: *mut sudo_event,
    pub sigterm_event: *mut sudo_event,
    pub sighup_event: *mut sudo_event,
    pub sigalrm_event: *mut sudo_event,
    pub sigusr1_event: *mut sudo_event,
    pub sigusr2_event: *mut sudo_event,
    pub sigchld_event: *mut sudo_event,
    pub sigwinch_event: *mut sudo_event,
    pub monitor_messages: monitor_message_list,
}


#[macro_export]
macro_rules! TAILQ_EMPTY {
    ($head:expr) => {
        (($head).tqh_first).is_null()
    };
}

#[macro_export]
macro_rules! TAILQ_FIRST {
    ($head:expr) => {
        (($head).tqh_first)
    };
}

#[macro_export]
macro_rules! SLIST_FIRST {
    ($head:expr) => {
        (($head).slh_first)
    };
}

#[macro_export]
macro_rules! sudo_ev_get_fd {
    ($_ev:expr) => {
        if !($_ev).is_null() {
            (*$_ev).fd
        } else {
            -(1 as libc::c_int)
        }
    };
}

#[macro_export]
macro_rules! sudo_ev_get_base {
    ($_ev:expr) => {
        if !($_ev).is_null() {
            (*$_ev).base
        } else {
            0 as *mut sudo_event_base
        }
    };
}

#[macro_export]
macro_rules! USERTTY_EVENT {
    ($_ev:expr) => {
        (sudo_ev_get_fd!(($_ev)) == io_fds[SFD_USERTTY as usize])
    };
}

#[macro_export]
macro_rules! S_ISFIFO {
    ($mode: expr) => {
        __S_ISTYPE!(($mode), __S_IFIFO as libc::c_uint)
    };
}

#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
        #[cfg(target_arch = "x86_64")]
        return __fxstat(1, __fd, __statbuf);
        #[cfg(not(target_arch = "x86_64"))]
        return __fxstat(0, __fd, __statbuf);
}
static mut ptyname: [libc::c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
#[no_mangle]
pub static mut io_fds: [libc::c_int; 6] = [-1; 6];
static mut foreground: bool = false;
static mut pipeline: bool = false;
static mut ttymode: libc::c_int = TERM_COOKED;
static mut ttyblock: sigset_t = sigset_t { __val: [0; 16] };
static mut iobufs: io_buffer_list = io_buffer_list {
    slh_first: 0 as *const io_buffer as *mut io_buffer,
};
static mut utmp_user: *const libc::c_char = 0 as *const libc::c_char;

/*
 * Cleanup hook for sudo_fatal()/sudo_fatalx()
 */
#[no_mangle]
pub unsafe extern "C" fn pty_cleanup() {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    if io_fds[SFD_USERTTY as usize] != -(1 as libc::c_int) {
        sudo_term_restore_v1(io_fds[SFD_USERTTY as usize], false);
    }
    if !utmp_user.is_null() {
        utmp_logout(ptyname.as_mut_ptr(), 0);
    }

    debug_return!();
}

/*
 * Allocate a pty if /dev/tty is a tty.
 * Fills in io_fds[SFD_USERTTY], io_fds[SFD_MASTER], io_fds[SFD_SLAVE]
 * and ptyname globals.
 */
unsafe extern "C" fn pty_setup(
    mut details: *mut command_details,
    mut tty: *const libc::c_char,
) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    io_fds[SFD_USERTTY as usize] = open(_PATH_TTY!(), O_RDWR);
    if io_fds[SFD_USERTTY as usize] == -(1 as libc::c_int) {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: no %s, not allocating a pty\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            _PATH_TTY!()
        );
        debug_return_bool!(false);
    }

    if !get_pty(
        &mut *io_fds.as_mut_ptr().offset(SFD_MASTER as isize),
        &mut *io_fds.as_mut_ptr().offset(SFD_SLAVE as isize),
        ptyname.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        (*details).euid,
    ) {
        sudo_fatal!(b"unable to allocate pty\0" as *const u8 as *const libc::c_char,);
    }

    /* Update tty name in command details (used by SELinux and AIX). */
    (*details).tty = ptyname.as_mut_ptr();

    /* Add entry to utmp/utmpx? */
    if ISSET!((*details).flags, CD_SET_UTMP) != 0 {
        utmp_user = if !((*details).utmp_user).is_null() {
            (*details).utmp_user
        } else {
            user_details.username
        };
        utmp_login(
            tty,
            ptyname.as_mut_ptr(),
            io_fds[SFD_SLAVE as usize],
            utmp_user,
        );
    }

    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"%s: %s fd %d, pty master fd %d, pty slave fd %d\0" as *const u8 as *const libc::c_char,
        stdext::function_name!().as_ptr(),
        _PATH_TTY!(),
        io_fds[SFD_USERTTY as usize],
        io_fds[SFD_MASTER as usize],
        io_fds[SFD_SLAVE as usize]
    );

    debug_return_bool!(true)
}

/*
 * Make the tty slave the controlling tty.
 * This is only used by the monitor but ptyname[] is static.
 */
#[no_mangle]
pub unsafe extern "C" fn pty_make_controlling() -> libc::c_int {
    if io_fds[SFD_SLAVE as usize] != -(1 as libc::c_int) {
        if ioctl(
            io_fds[SFD_SLAVE as usize],
            TIOCSCTTY as libc::c_ulong,
            0 as *mut libc::c_void,
        ) != 0
        {
            return -(1 as libc::c_int);
        }
    }
    return 0;
}

/* Call I/O plugin tty input log method. */
unsafe extern "C" fn log_ttyin(
    mut buf: *const libc::c_char,
    mut n: libc::c_uint,
    mut iob: *mut io_buffer,
) -> bool {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut ret: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    sigprocmask(SIG_BLOCK, &mut ttyblock, &mut omask);
    plugin = TAILQ_FIRST!(io_plugins);
    while !plugin.is_null() {
        if ((*(*plugin).u.io).log_ttyin).is_some() {
            let mut rc: libc::c_int = 0;

            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            rc = ((*(*plugin).u.io).log_ttyin).expect("non-null function pointer")(buf, n);
            if rc <= 0 {
                if rc < 0 {
                    /* Error: disable plugin's I/O function. */
                    (*(*plugin).u.io).log_ttyin = None;
                }
                ret = false;
                break;
            }
        }
        plugin = (*plugin).entries.tqe_next;
    }
    sudo_debug_set_active_instance_v1(sudo_debug_instance);
    sigprocmask(SIG_SETMASK, &mut omask, 0 as *mut sigset_t);

    debug_return_bool!(ret)
}

/* Call I/O plugin stdin log method. */
unsafe extern "C" fn log_stdin(
    mut buf: *const libc::c_char,
    mut n: libc::c_uint,
    mut iob: *mut io_buffer,
) -> bool {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut ret: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    sigprocmask(SIG_BLOCK, &mut ttyblock, &mut omask);
    plugin = TAILQ_FIRST!(io_plugins);
    while !plugin.is_null() {
        if ((*(*plugin).u.io).log_stdin).is_some() {
            let mut rc: libc::c_int = 0;

            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            rc = ((*(*plugin).u.io).log_stdin).expect("non-null function pointer")(buf, n);
            if rc <= 0 {
                if rc < 0 {
                    /* Error: disable plugin's I/O function. */
                    (*(*plugin).u.io).log_stdin = None;
                }
                ret = false;
                break;
            }
        }
        plugin = (*plugin).entries.tqe_next;
    }
    sudo_debug_set_active_instance_v1(sudo_debug_instance);
    sigprocmask(SIG_SETMASK, &mut omask, 0 as *mut sigset_t);

    debug_return_bool!(ret)
}

/* Call I/O plugin tty output log method. */
unsafe extern "C" fn log_ttyout(
    mut buf: *const libc::c_char,
    mut n: libc::c_uint,
    mut iob: *mut io_buffer,
) -> bool {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut ret: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    sigprocmask(SIG_BLOCK, &mut ttyblock, &mut omask);
    plugin = TAILQ_FIRST!(io_plugins);
    while !plugin.is_null() {
        if ((*(*plugin).u.io).log_ttyout).is_some() {
            let mut rc: libc::c_int = 0;

            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            rc = ((*(*plugin).u.io).log_ttyout).expect("non-null function pointer")(buf, n);
            if rc <= 0 {
                if rc < 0 {
                    /* Error: disable plugin's I/O function. */
                    (*(*plugin).u.io).log_ttyout = None;
                }
                ret = false;
                break;
            }
        }
        plugin = (*plugin).entries.tqe_next;
    }
    sudo_debug_set_active_instance_v1(sudo_debug_instance);
    if !ret {
        /*
         * I/O plugin rejected the output, delete the write event
         * (user's tty) so we do not display the rejected output.
         */
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: deleting and freeing devtty wevent %p\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*iob).wevent
        );
        sudo_ev_free_v1((*iob).wevent);
        (*iob).wevent = 0 as *mut sudo_event;
        (*iob).len = 0 as libc::c_int;
        (*iob).off = (*iob).len;
    }
    sigprocmask(SIG_SETMASK, &mut omask, 0 as *mut sigset_t);

    debug_return_bool!(ret)
}

/* Call I/O plugin stdout log method. */
unsafe extern "C" fn log_stdout(
    mut buf: *const libc::c_char,
    mut n: libc::c_uint,
    mut iob: *mut io_buffer,
) -> bool {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut ret: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    sigprocmask(SIG_BLOCK, &mut ttyblock, &mut omask);
    plugin = TAILQ_FIRST!(io_plugins);
    while !plugin.is_null() {
        if ((*(*plugin).u.io).log_stdout).is_some() {
            let mut rc: libc::c_int = 0;

            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            rc = ((*(*plugin).u.io).log_stdout).expect("non-null function pointer")(buf, n);
            if rc <= 0 {
                if rc < 0 {
                    /* Error: disable plugin's I/O function. */
                    let ref mut plugin0 = (*(*plugin).u.io).log_stdout;
                    *plugin0 = None;
                }
                ret = false;
                break;
            }
        }
        plugin = (*plugin).entries.tqe_next;
    }
    sudo_debug_set_active_instance_v1(sudo_debug_instance);
    if !ret {
        /*
         * I/O plugin rejected the output, delete the write event
         * (user's stdout) so we do not display the rejected output.
         */
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: deleting and freeing stdout wevent %p\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*iob).wevent
        );
        sudo_ev_free_v1((*iob).wevent);
        let ref mut wevent0 = (*iob).wevent;
        *wevent0 = 0 as *mut sudo_event;
        let ref mut len0 = (*iob).len;
        *len0 = 0 as libc::c_int;
        (*iob).off = *len0;
    }
    sigprocmask(SIG_SETMASK, &mut omask, 0 as *mut sigset_t);

    debug_return_bool!(ret)
}

/* Call I/O plugin stderr log method. */
unsafe extern "C" fn log_stderr(
    mut buf: *const libc::c_char,
    mut n: libc::c_uint,
    mut iob: *mut io_buffer,
) -> bool {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut ret: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    sigprocmask(SIG_BLOCK, &mut ttyblock, &mut omask);
    plugin = TAILQ_FIRST!(io_plugins);
    while !plugin.is_null() {
        if ((*(*plugin).u.io).log_stderr).is_some() {
            let mut rc: libc::c_int = 0;

            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            rc = ((*(*plugin).u.io).log_stderr).expect("non-null function pointer")(buf, n);
            if rc <= 0 {
                if rc < 0 {
                    /* Error: disable plugin's I/O function. */
                    let ref mut plugin0 = (*(*plugin).u.io).log_stderr;
                    *plugin0 = None;
                }
                ret = false;
                break;
            }
        }
        plugin = (*plugin).entries.tqe_next;
    }
    sudo_debug_set_active_instance_v1(sudo_debug_instance);
    if !ret {
        /*
         * I/O plugin rejected the output, delete the write event
         * (user's stdout) so we do not display the rejected output.
         */
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: deleting and freeing stderr wevent %p\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*iob).wevent
        );
        (*iob).wevent = 0 as *mut sudo_event;
        (*iob).len = 0;
        (*iob).off = (*iob).len;
    }
    sigprocmask(SIG_SETMASK, &mut omask, 0 as *mut sigset_t);

    debug_return_bool!(ret)
}

/* Call I/O plugin suspend log method. */
unsafe extern "C" fn log_suspend(mut signo: libc::c_int) {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut ret: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    sigprocmask(SIG_BLOCK, &mut ttyblock, &mut omask);
    plugin = TAILQ_FIRST!(io_plugins);
    while !plugin.is_null() {
        if (*(*plugin).u.io).version < SUDO_API_MKVERSION!(1, 13) {
            continue;
        }

        if ((*(*plugin).u.io).log_suspend).is_some() {
            let mut rc: libc::c_int = 0;

            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            rc = ((*(*plugin).u.io).log_suspend).expect("non-null function pointer")(signo);
            if rc <= 0 {
                if rc < 0 {
                    /* Error: disable plugin's I/O function. */
                    (*(*plugin).u.io).log_suspend = None;
                }
                break;
            }
        }
        plugin = (*plugin).entries.tqe_next;
    }
    sudo_debug_set_active_instance_v1(sudo_debug_instance);
    sigprocmask(SIG_SETMASK, &mut omask, 0 as *mut sigset_t);

    debug_return!();
}

/* Call I/O plugin window change log method. */
unsafe extern "C" fn log_winchange(mut rows: libc::c_uint, mut cols: libc::c_uint) {
    let mut plugin: *mut plugin_container = 0 as *mut plugin_container;
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut ret: bool = true;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    sigprocmask(SIG_BLOCK, &mut ttyblock, &mut omask);
    plugin = TAILQ_FIRST!(io_plugins);
    while !plugin.is_null() {
        if (*(*plugin).u.io).version < SUDO_API_MKVERSION!(1, 12) {
            continue;
        }
        if ((*(*plugin).u.io).change_winsize).is_some() {
            let mut rc: libc::c_int = 0;

            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            rc = ((*(*plugin).u.io).change_winsize).expect("non-null function pointer")(rows, cols);
            if rc <= 0 {
                if rc < 0 {
                    /* Error: disable plugin's I/O function. */
                    (*(*plugin).u.io).change_winsize = None;
                }
                break;
            }
        }
        plugin = (*plugin).entries.tqe_next;
    }
    sudo_debug_set_active_instance_v1(sudo_debug_instance);
    sigprocmask(SIG_SETMASK, &mut omask, 0 as *mut sigset_t);

    debug_return!();
}

/*
 * Check whether we are running in the foregroup.
 * Updates the foreground global and does lazy init of the
 * the pty slave as needed.
 */
unsafe extern "C" fn check_foreground(mut ec: *mut exec_closure_pty) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    if io_fds[SFD_USERTTY as usize] != -(1 as libc::c_int) {
        foreground = tcgetpgrp(io_fds[SFD_USERTTY as usize]) == (*ec).ppgrp;

        /* Also check for window size changes. */
        sync_ttysize(ec);
    }

    debug_return!();
}

/*
 * Suspend sudo if the underlying command is suspended.
 * Returns SIGCONT_FG if the command should be resumed in the
 * foreground or SIGCONT_BG if it is a background process.
 */
unsafe extern "C" fn suspend_sudo(
    mut ec: *mut exec_closure_pty,
    mut signo: libc::c_int,
) -> libc::c_int {
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut osa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut ret: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    loop {
        match signo {
            SIGTTOU | SIGTTIN | SIGSTOP | SIGTSTP => {
                if signo == SIGTTOU || signo == SIGTTIN {
                    /*
                     * If sudo is already the foreground process, just resume the command
                     * in the foreground.  If not, we'll suspend sudo and resume later.
                     */
                    if !foreground {
                        check_foreground(ec);
                    }
                    if foreground {
                        if ttymode != TERM_RAW {
                            if sudo_term_raw_v1(io_fds[SFD_USERTTY as usize], 0) {
                                ttymode = TERM_RAW;
                            }
                        }
                        ret = SIGCONT_FG; /* resume command in foreground */
                        break;
                    }
                    /* FALLTHROUGH */
                    /* Flush any remaining output and deschedule I/O events. */
                    del_io_events(true);

                    /* Restore original tty mode before suspending. */
                    if ttymode != TERM_COOKED {
                        sudo_term_restore_v1(io_fds[SFD_USERTTY as usize], false);
                    }

                    /* Log the suspend event. */
                    log_suspend(signo);

                    if sudo_sig2str(signo, signame.as_mut_ptr()) == -(1 as libc::c_int) {
                        snprintf(
                            signame.as_mut_ptr(),
                            std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            signo,
                        );
                    }

                    /* Suspend self and continue command when we resume. */
                    if signo != SIGSTOP {
                        memset(
                            &mut sa as *mut sigaction as *mut libc::c_void,
                            0,
                            std::mem::size_of::<sigaction>() as libc::c_ulong,
                        );
                        sigemptyset(&mut sa.sa_mask);
                        sa.sa_flags = SA_RESTART;
                        sa.__sigaction_handler.sa_handler = None;
                        if sudo_sigaction(signo, &mut sa, &mut osa) != 0 {
                            sudo_warn!(
                                b"unable to set handler for signal %d\0" as *const u8
                                    as *const libc::c_char,
                                signo
                            );
                        }
                    }
                    sudo_debug_printf!(
                        SUDO_DEBUG_INFO,
                        b"kill parent SIG%s\0" as *const u8 as *const libc::c_char,
                        signame.as_mut_ptr()
                    );
                    if killpg((*ec).ppgrp, signo) != 0 {
                        sudo_warn!(
                            b"killpg(%d, SIG%s)\0" as *const u8 as *const libc::c_char,
                            (*ec).ppgrp,
                            signame.as_mut_ptr()
                        );
                    }

                    /* Log the resume event. */
                    log_suspend(SIGCONT);

                    /* Check foreground/background status on resume. */
                    check_foreground(ec);

                    /*
                     * We always resume the command in the foreground if sudo itself
                     * is the foreground process.  This helps work around poorly behaved
                     * programs that catch SIGTTOU/SIGTTIN but suspend themselves with
                     * SIGSTOP.  At worst, sudo will go into the background but upon
                     * resume the command will be runnable.  Otherwise, we can get into
                     * a situation where the command will immediately suspend itself.
                     */
                    sudo_debug_printf!(
                        SUDO_DEBUG_INFO,
                        b"parent is in %s, ttymode %d -> %d\0" as *const u8 as *const libc::c_char,
                        if foreground as libc::c_int != 0 {
                            b"foreground\0" as *const u8 as *const libc::c_char
                        } else {
                            b"background\0" as *const u8 as *const libc::c_char
                        },
                        ttymode,
                        if foreground as libc::c_int != 0 {
                            TERM_RAW
                        } else {
                            TERM_COOKED
                        }
                    );
                    if foreground {
                        /* Foreground process, set tty to raw mode. */
                        if sudo_term_raw_v1(io_fds[SFD_USERTTY as usize], 0) {
                            ttymode = TERM_RAW;
                        }
                    } else {
                        /* Background process, no access to tty. */
                        ttymode = TERM_COOKED;
                    }

                    if signo != SIGSTOP {
                        if sudo_sigaction(signo, &mut osa, 0 as *mut sigaction) != 0 {
                            sudo_warn!(
                                b"unable to restore handler for signal %d\0" as *const u8
                                    as *const libc::c_char,
                                signo
                            );
                        }
                    }

                    ret = if ttymode == TERM_RAW {
                        SIGCONT_FG
                    } else {
                        SIGCONT_BG
                    };
                }
            }
            _ => {}
        }
        break;
    }

    debug_return_int!(ret);
}

/*
 * SIGTTIN signal handler for read_callback that just sets a flag.
 */
static mut got_sigttin: sig_atomic_t = 0;

unsafe extern "C" fn sigttin(mut signo: libc::c_int) {
    std::ptr::write_volatile(&mut got_sigttin as *mut sig_atomic_t, 1);
}

/*
 * Read an iobuf that is ready.
 */
unsafe extern "C" fn read_callback(
    mut fd: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut iob: *mut io_buffer = v as *mut io_buffer;
    let mut evbase: *mut sudo_event_base = sudo_ev_get_base!((*iob).revent);
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut osa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut n: ssize_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    /*
     * We ignore SIGTTIN by default but we need to handle it when reading
     * from the terminal.  A signal event won't work here because the
     * read() would be restarted, preventing the callback from running.
     */
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0,
        std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask);
    sa.__sigaction_handler.sa_handler = Some(sigttin as unsafe extern "C" fn(libc::c_int) -> ());
    std::ptr::write_volatile(&mut got_sigttin as *mut sig_atomic_t, 0);
    sigaction(SIGTTIN, &mut sa, &mut osa);
    n = read(
        fd,
        ((*iob).buf).as_mut_ptr().offset((*iob).len as isize) as *mut libc::c_void,
        (std::mem::size_of::<[libc::c_char; 65536]>() as libc::c_ulong)
            .wrapping_sub((*iob).len as libc::c_ulong),
    );
    saved_errno = errno!();
    sigaction(SIGTTIN, &mut osa, 0 as *mut sigaction);
    errno!() = saved_errno;

    loop {
        match n {
            -1 | 0 => {
                if n == -1 {
                    if got_sigttin != 0 {
                        /* Schedule SIGTTIN to be forwared to the command. */
                        schedule_signal((*iob).ec, SIGTTIN);
                    }
                    if errno!() == EAGAIN || errno!() == EINTR {
                        break;
                    }
                    /* treat read error as fatal and close the fd */
                    sudo_debug_printf!(
                        SUDO_DEBUG_ERROR,
                        b"error reading fd %d: %s\0" as *const u8 as *const libc::c_char,
                        fd,
                        strerror(errno!())
                    );
                }
                /* FALLTHROUGH */
                /* got EOF or pty has gone away */
                if n == 0 {
                    sudo_debug_printf!(
                        SUDO_DEBUG_INFO,
                        b"read EOF from fd %d\0" as *const u8 as *const libc::c_char,
                        fd
                    );
                    safe_close(fd);
                    ev_free_by_fd(evbase, fd);
                    /* If writer already consumed the buffer, close it too. */
                    if !((*iob).wevent).is_null() && (*iob).off == (*iob).len {
                        safe_close(sudo_ev_get_fd!((*iob).wevent));
                        ev_free_by_fd(evbase, sudo_ev_get_fd!((*iob).wevent));
                        (*iob).len = 0;
                        (*iob).off = (*iob).len;
                    }
                }
            }
            _ => {
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"read %zd bytes from fd %d\0" as *const u8 as *const libc::c_char,
                    n,
                    fd
                );
                if !((*iob).action).expect("non-null function pointer")(
                    ((*iob).buf).as_mut_ptr().offset((*iob).len as isize),
                    n as libc::c_uint,
                    iob,
                ) {
                    terminate_command((*(*iob).ec).cmnd_pid, true);
                    (*(*iob).ec).cmnd_pid = -(1 as libc::c_int);
                }
                let ref mut len0 = (*iob).len;
                *len0 = (*len0 as libc::c_long + n) as libc::c_int;
                /* Enable writer now that there is data in the buffer. */
                if !((*iob).wevent).is_null() {
                    if sudo_ev_add_v2(evbase, (*iob).wevent, 0 as *mut timespec, false)
                        == -(1 as libc::c_int)
                    {
                        sudo_fatal!(
                            b"unable to add event to queue\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                /* Re-enable reader if buffer is not full. */
                if (*iob).len as libc::c_ulong
                    != std::mem::size_of::<[libc::c_char; 65536]>() as libc::c_ulong
                {
                    if sudo_ev_add_v2(evbase, (*iob).revent, 0 as *mut timespec, false)
                        == -(1 as libc::c_int)
                    {
                        sudo_fatal!(
                            b"unable to add event to queue\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        }
        break;
    }
}

/*
 * SIGTTOU signal handler for write_callback that just sets a flag.
 */
static mut got_sigttou: sig_atomic_t = 0;

unsafe extern "C" fn sigttou(mut signo: libc::c_int) {
    std::ptr::write_volatile(&mut got_sigttou as *mut sig_atomic_t, 1);
}

/*
 * Write an iobuf that is ready.
 */
unsafe extern "C" fn write_callback(
    mut fd: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut iob: *mut io_buffer = v as *mut io_buffer;
    let mut evbase: *mut sudo_event_base = sudo_ev_get_base!((*iob).wevent);
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut osa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut saved_errno: libc::c_int = 0;
    let mut n: ssize_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    /*
     * We ignore SIGTTOU by default but we need to handle it when writing
     * to the terminal.  A signal event won't work here because the
     * write() would be restarted, preventing the callback from running.
     */
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0,
        std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask);
    sa.__sigaction_handler.sa_handler = Some(sigttou as unsafe extern "C" fn(libc::c_int) -> ());
    std::ptr::write_volatile(&mut got_sigttou as *mut sig_atomic_t, 0);
    sigaction(SIGTTOU, &mut sa, &mut osa);
    n = write(
        fd,
        ((*iob).buf).as_mut_ptr().offset((*iob).off as isize) as *const libc::c_void,
        ((*iob).len - (*iob).off) as size_t,
    );
    saved_errno = errno!();
    sigaction(SIGTTOU, &mut osa, 0 as *mut sigaction);
    errno!() = saved_errno;

    if n == -(1 as libc::c_int) as libc::c_long {
        match errno!() {
            EPIPE | ENXIO | EIO | EBADF => {
                /* other end of pipe closed or pty revoked */
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"unable to write %d bytes to fd %d\0" as *const u8 as *const libc::c_char,
                    (*iob).len - (*iob).off,
                    fd
                );
                /* Close reader if there is one. */
                if !((*iob).revent).is_null() {
                    safe_close(sudo_ev_get_fd!((*iob).revent));
                    ev_free_by_fd(evbase, sudo_ev_get_fd!((*iob).revent));
                }
                safe_close(fd);
                ev_free_by_fd(evbase, fd);
            }
            EINTR => {
                if got_sigttou != 0 {
                    /* Schedule SIGTTOU to be forwared to the command. */
                    schedule_signal((*iob).ec, SIGTTOU);
                }
            }
            EAGAIN => { /* not an error */ }
            _ => {
                /* XXX - need a way to distinguish non-exec error. */
                (*(*(*iob).ec).cstat).type_0 = CMD_ERRNO;
                (*(*(*iob).ec).cstat).val = errno!();
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR,
                    b"error writing fd %d: %s\0" as *const u8 as *const libc::c_char,
                    fd,
                    strerror(errno!())
                );
                sudo_ev_loopbreak_v1(evbase);
            }
        }
    } else {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR,
            b"SUDO_DEBUG_INFO\0" as *const u8 as *const libc::c_char,
            n,
            fd
        );
        let ref mut n0 = (*iob).off;
        *n0 = (*n0 as libc::c_long + n) as libc::c_int;
        /* Reset buffer if fully consumed. */
        if (*iob).off == (*iob).len {
            let ref mut len0 = (*iob).len;
            *len0 = 0;
            (*iob).off = *len0;
            /* Forward the EOF from reader to writer. */
            if ((*iob).revent).is_null() {
                safe_close(fd);
                ev_free_by_fd(evbase, fd);
            }
        }
        /* Re-enable writer if buffer is not empty. */
        if (*iob).len > (*iob).off {
            if sudo_ev_add_v2(evbase, (*iob).wevent, 0 as *mut timespec, false)
                == -(1 as libc::c_int)
            {
                sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
            }
        }
        /* Enable reader if buffer is not full.*/
        if !((*iob).revent).is_null() && (ttymode == TERM_RAW || !(USERTTY_EVENT!((*iob).revent))) {
            if (*iob).len as libc::c_ulong
                != ::std::mem::size_of::<[libc::c_char; 65536]>() as libc::c_ulong
            {
                if sudo_ev_add_v2(evbase, (*iob).revent, 0 as *mut timespec, false)
                    == -(1 as libc::c_int)
                {
                    sudo_fatal!(
                        b"unable to add event to queue\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
}

unsafe extern "C" fn io_buf_new(
    mut rfd: libc::c_int,
    mut wfd: libc::c_int,
    mut action: Option<
        unsafe extern "C" fn(*const libc::c_char, libc::c_uint, *mut io_buffer) -> bool,
    >,
    mut ec: *mut exec_closure_pty,
    mut head: *mut io_buffer_list,
) {
    let mut n: libc::c_int = 0;
    let mut iob: *mut io_buffer = 0 as *mut io_buffer;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    /* Set non-blocking mode. */
    n = fcntl(rfd, F_GETFL, 0);
    if n != -1 && ISSET!(n, O_NONBLOCK) == 0 {
        fcntl(rfd, F_SETFL, n | O_NONBLOCK);
    }
    n = fcntl(wfd, F_GETFL, 0);
    if n != -1 && ISSET!(n, O_NONBLOCK) == 0 {
        fcntl(wfd, F_SETFL, n | O_NONBLOCK);
    }

    /* Allocate and add to head of list. */
    iob = malloc(std::mem::size_of::<io_buffer>() as libc::c_ulong) as *mut io_buffer;
    if iob.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    (*iob).ec = ec;
    let ref mut revent0 = (*iob).revent;
    *revent0 = sudo_ev_alloc_v1(
        rfd,
        SUDO_EV_READ as libc::c_short,
        Some(
            read_callback
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        iob as *mut libc::c_void,
    );
    let ref mut wevent0 = (*iob).wevent;
    *wevent0 = sudo_ev_alloc_v1(
        wfd,
        SUDO_EV_WRITE as libc::c_short,
        Some(
            write_callback
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        iob as *mut libc::c_void,
    );
    (*iob).len = 0;
    (*iob).off = 0;
    (*iob).action = action;
    (*iob).buf[0 as usize] = '\u{0}' as i32 as libc::c_char;
    if ((*iob).revent).is_null() || ((*iob).wevent).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    (*iob).entries.sle_next = (*head).slh_first;
    (*head).slh_first = iob;

    debug_return!();
}

/*
 * We already closed the slave pty so reads from the master will not block.
 */
unsafe extern "C" fn pty_finish(mut cstat: *mut command_status) {
    let mut iob: *mut io_buffer = 0 as *mut io_buffer;
    let mut n: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    /* Flush any remaining output (the plugin already got it). */
    if io_fds[SFD_USERTTY as usize] != -(1 as libc::c_int) {
        n = fcntl(io_fds[SFD_USERTTY as usize], F_GETFL, 0);
        if n != -1 && ISSET!(n, O_NONBLOCK) != 0 {
            CLR!(n, O_NONBLOCK);
            fcntl(io_fds[SFD_USERTTY as usize], F_SETFL, n);
        }
    }
    del_io_events(false);

    /* Free I/O buffers. */
    loop {
        iob = SLIST_FIRST!(iobufs);
        if iob.is_null() {
            break;
        }
        iobufs.slh_first = (*iobufs.slh_first).entries.sle_next;
        if !((*iob).revent).is_null() {
            sudo_ev_free_v1((*iob).revent);
        }
        if !((*iob).wevent).is_null() {
            sudo_ev_free_v1((*iob).wevent);
        }
        free(iob as *mut libc::c_void);
        break;
    }
    if io_fds[SFD_USERTTY as usize] != -(1 as libc::c_int) {
        sudo_term_restore_v1(io_fds[SFD_USERTTY as usize], false);
    }

    /* Update utmp */
    if !utmp_user.is_null() {
        utmp_logout(
            ptyname.as_mut_ptr(),
            if (*cstat).type_0 == CMD_WSTATUS {
                (*cstat).val
            } else {
                0
            },
        );
    }
    debug_return!();
}

/*
 * Send command status to the monitor (signal or window size change).
 */
 unsafe extern "C" fn send_command_status(
    mut ec: *mut exec_closure_pty,
    mut type_0: libc::c_int,
    mut val: libc::c_int,
) {
    let mut msg: *mut monitor_message = 0 as *mut monitor_message;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    msg = calloc(
        1 as libc::c_ulong,
        std::mem::size_of::<monitor_message>() as libc::c_ulong,
    ) as *mut monitor_message;
    if msg.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    (*msg).cstat.type_0 = type_0;
    (*msg).cstat.val = val;
    (*msg).entries.tqe_next = 0 as *mut monitor_message;
    (*msg).entries.tqe_prev = (*ec).monitor_messages.tqh_last;
    *(*ec).monitor_messages.tqh_last = msg;
    (*ec).monitor_messages.tqh_last = &mut (*msg).entries.tqe_next;

    if sudo_ev_add_v2(
        (*ec).evbase,
        (*ec).fwdchannel_event,
        0 as *mut timespec,
        true,
    ) == -(1 as libc::c_int)
    {
        sudo_fatal!(b"unable to add event to queue\0" as *const u8 as *const libc::c_char,);
    }

    /* Restart event loop to send the command immediately. */
    sudo_ev_loopcontinue_v1((*ec).evbase);

    debug_return!();
}

/*
 * Schedule a signal to be forwarded.
 */
 unsafe extern "C" fn schedule_signal(mut ec: *mut exec_closure_pty, mut signo: libc::c_int) {
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
 
    if signo == -(2 as libc::c_int) {
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

    sudo_debug_printf!(
        SUDO_DEBUG_DIAG,
        b"scheduled SIG%s for command\0" as *const u8 as *const libc::c_char,
        signame.as_mut_ptr()
    );

    send_command_status(ec, CMD_SIGNO, signo);

    debug_return!();
}

unsafe extern "C" fn backchannel_cb(
    mut fd: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut ec: *mut exec_closure_pty = v as *mut exec_closure_pty;
    let mut cstat: command_status = command_status { type_0: 0, val: 0 };
    let mut nread: ssize_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    /*
     * Read command status from the monitor.
     * Note that the backchannel is a *blocking* socket.
     */
    nread = recv(
        fd,
        &mut cstat as *mut command_status as *mut libc::c_void,
        std::mem::size_of::<command_status>() as libc::c_ulong,
        MSG_WAITALL,
    );

    match nread {
        -1 => {
            match errno!() {
                EINTR | EAGAIN => { /* Nothing ready. */ }
                _ => {
                    if (*(*ec).cstat).val == CMD_INVALID {
                        (*(*ec).cstat).type_0 = CMD_ERRNO;
                        (*(*ec).cstat).val = errno!();
                        sudo_debug_printf!(
                            SUDO_DEBUG_ERROR,
                            b"%s: failed to read command status: %s\0" as *const u8
                                as *const libc::c_char,
                            stdext::function_name!().as_ptr(),
                            strerror(errno!())
                        );
                        sudo_ev_loopbreak_v1((*ec).evbase);
                    }
                }
            }
        }
        0 => {
            /* EOF, monitor exited or was killed. */
            sudo_debug_printf!(
                SUDO_DEBUG_INFO,
                b"EOF on backchannel, monitor dead?\0" as *const u8 as *const libc::c_char
            );
            if (*(*ec).cstat).type_0 == CMD_INVALID {
                /* XXX - need new CMD_ type for monitor errors. */
                (*(*ec).cstat).type_0 = CMD_ERRNO;
                (*(*ec).cstat).val = ECONNRESET;
            }
            sudo_ev_loopexit_v1((*ec).evbase);
        }
        8 => {
            /* Check command status. */
            match cstat.type_0 {
                CMD_PID => {
                    (*ec).cmnd_pid = cstat.val;
                    sudo_debug_printf!(
                        SUDO_DEBUG_INFO,
                        b"executed %s, pid %d\0" as *const u8 as *const libc::c_char,
                        (*(*ec).details).command,
                        (*ec).cmnd_pid
                    );
                }
                CMD_WSTATUS => {
                    if WIFSTOPPED!(cstat.val) {
                        let mut signo: libc::c_int = 0;

                        /* Suspend parent and tell monitor how to resume on return. */
                        sudo_debug_printf!(
                            SUDO_DEBUG_INFO,
                            b"command stopped, suspending parent\0" as *const u8
                                as *const libc::c_char
                        );
                        signo = suspend_sudo(ec, WSTOPSIG!(cstat.val));
                        schedule_signal(ec, signo);
                        /* Re-enable I/O events */
                        add_io_events((*ec).evbase);
                    } else {
                        /* Command exited or was killed, either way we are done. */
                        sudo_debug_printf!(
                            SUDO_DEBUG_INFO,
                            b"command exited or was killed\0" as *const u8 as *const libc::c_char
                        );
                        sudo_ev_loopexit_v1((*ec).evbase);
                    }
                    *(*ec).cstat = cstat;
                }
                CMD_ERRNO => {
                    /* Monitor was unable to execute command or broken pipe. */
                    sudo_debug_printf!(
                        SUDO_DEBUG_INFO,
                        b"errno from monitor: %s\0" as *const u8 as *const libc::c_char,
                        strerror(cstat.val)
                    );
                    sudo_ev_loopbreak_v1((*ec).evbase);
                    *(*ec).cstat = cstat;
                }
                _ => {}
            }
        }
        _ => {
            /* Short read, should not happen. */
            if (*(*ec).cstat).val == CMD_INVALID {
                (*(*ec).cstat).type_0 = CMD_ERRNO;
                (*(*ec).cstat).val = EIO;
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR,
                    b"%s: failed to read command status: short read\0" as *const u8
                        as *const libc::c_char,
                    stdext::function_name!().as_ptr()
                );
                sudo_ev_loopbreak_v1((*ec).evbase);
            }
        }
    }

    debug_return!();
}

/*
 * Handle changes to the monitors's status (SIGCHLD).
 */
 unsafe extern "C" fn handle_sigchld_pty(mut ec: *mut exec_closure_pty) {
    let mut n: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    /*
     * Monitor process was signaled; wait for it as needed.
     */
    loop {
        pid = waitpid((*ec).monitor_pid, &mut status, WUNTRACED | WNOHANG);
        if !(pid == -(1 as libc::c_int) && errno!() == EINTR) {
            break;
        }
        break;
    }

    match pid {
        0 | -1 => {
            if pid == 0 {
                errno!() = ECHILD;
            }
            /* FALLTHROUGH */
            sudo_warn!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                b"waitpid\0" as *const u8 as *const libc::c_char
            );
            debug_return!();
        }
        _ => {}
    }

        /*
     * If the monitor dies we get notified via backchannel_cb().
     * If it was stopped, we should stop too (the command keeps
     * running in its pty) and continue it when we come back.
     */
     if WIFSTOPPED!(status) {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"monitor stopped, suspending sudo\0" as *const u8 as *const libc::c_char
        );
        n = suspend_sudo(ec, WSTOPSIG!(status));
        kill(pid, SIGCONT);
        schedule_signal(ec, n);
        /* Re-enable I/O events */
        add_io_events((*ec).evbase);
    } else if WIFSIGNALED!(status) {
        let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
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
            b"%s: monitor (%d) killed, SIG%s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*ec).monitor_pid,
            signame.as_mut_ptr()
        );
        (*ec).monitor_pid = -(1 as libc::c_int);
    } else {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: monitor exited, status %d\0" as *const u8 as *const libc::c_char,
            WEXITSTATUS!(status)
        );
        (*ec).monitor_pid = -(1 as libc::c_int);
    }
    
    debug_return!();
}

/* Signal callback */
unsafe extern "C" fn signal_cb_pty(
    mut signo: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut sc: *mut sudo_ev_siginfo_container = v as *mut sudo_ev_siginfo_container;
    let mut ec: *mut exec_closure_pty = (*sc).closure as *mut exec_closure_pty;
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    if (*ec).monitor_pid == -(1 as libc::c_int) {
        debug_return!();
    }

    if sudo_sig2str(signo, signame.as_mut_ptr()) == -(1 as libc::c_int) {
        snprintf(
            signame.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            signo,
        );
    }
    sudo_debug_printf!(
        SUDO_DEBUG_DIAG,
        b"%s: evbase %p, monitor: %d, signo %s(%d), cstat %p\0" as *const u8 as *const libc::c_char,
        stdext::function_name!().as_ptr(),
        (*ec).evbase,
        (*ec).monitor_pid,
        signame.as_mut_ptr(),
        signo,
        (*ec).cstat
    );

    match signo {
        SIGCHLD => {
            handle_sigchld_pty(ec);
        }
        SIGWINCH => {
            sync_ttysize(ec);
        }
        _ => {
            /*
             * Do not forward signals sent by a process in the command's process
             * group, as we don't want the command to indirectly kill itself.
             * For example, this can happen with some versions of reboot that
             * call kill(-1, SIGTERM) to kill all other processes.
             */
            if USER_SIGNALED!((*sc).siginfo) && (*(*sc).siginfo)._sifields._kill.si_pid != 0 {
                let mut si_pgrp: pid_t = getpgid((*(*sc).siginfo)._sifields._kill.si_pid);
                if si_pgrp != -(1 as libc::c_int) {
                    if si_pgrp == (*ec).ppgrp || si_pgrp == (*ec).cmnd_pid {
                        debug_return!();
                    }
                } else if (*(*sc).siginfo)._sifields._kill.si_pid == (*ec).cmnd_pid {
                    debug_return!();
                }
            }
            /* Schedule signal to be forwared to the command. */
            schedule_signal(ec, signo);
        }
    }


    debug_return!();
}

/*
 * Forward signals in monitor_messages to the monitor so it can
 * deliver them to the command.
 */
unsafe extern "C" fn fwdchannel_cb(
    mut sock: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut ec: *mut exec_closure_pty = v as *mut exec_closure_pty;
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
    let mut msg: *mut monitor_message = 0 as *mut monitor_message;
    let mut nsent: ssize_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC); 

    loop {
        msg = TAILQ_FIRST!((*ec).monitor_messages);
        if msg.is_null() {
            break;
        }
        match (*msg).cstat.type_0 {
            CMD_SIGNO => {
                if (*msg).cstat.val == SIGCONT_FG {
                    sudo_strlcpy(
                        signame.as_mut_ptr(),
                        b"CONT_FG\0" as *const u8 as *const libc::c_char,
                        std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                    );
                } else if (*msg).cstat.val == SIGCONT_BG {
                    sudo_strlcpy(
                        signame.as_mut_ptr(),
                        b"CONT_BG\0" as *const u8 as *const libc::c_char,
                        std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                    );
                } else if sudo_sig2str((*msg).cstat.val, signame.as_mut_ptr())
                    == -(1 as libc::c_int)
                {
                    snprintf(
                        signame.as_mut_ptr(),
                        std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        (*msg).cstat.val,
                    );
                }
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"sending SIG%s to monitor over backchannel\0" as *const u8
                        as *const libc::c_char,
                    signame.as_mut_ptr()
                );
            }
            CMD_TTYWINCH => {
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"sending window size change to monitor over backchannelL %d x %d\0"
                        as *const u8 as *const libc::c_char,
                    (*msg).cstat.val & 0xffff,
                    (*msg).cstat.val >> 16 & 0xffff
                );
            }
            _ => {
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"sending cstat type %d, value %d to monitor over backchannel\0" as *const u8
                        as *const libc::c_char,
                    (*msg).cstat.type_0,
                    (*msg).cstat.val
                );
            }
        }

        //TAILQ_REMOVE(&ec->monitor_messages, msg, entries);
        if !((*msg).entries.tqe_next).is_null() {
            (*(*msg).entries.tqe_next).entries.tqe_prev = (*msg).entries.tqe_prev;
        } else {
            (*ec).monitor_messages.tqh_last = (*msg).entries.tqe_prev;
        }
        *(*msg).entries.tqe_prev = (*msg).entries.tqe_next;

        nsent = send(
            sock,
            &mut (*msg).cstat as *mut command_status as *const libc::c_void,
            std::mem::size_of::<command_status>() as libc::c_ulong,
            0,
        );

        if nsent as libc::c_ulong != std::mem::size_of::<command_status>() as libc::c_ulong {
            if errno!() == EPIPE {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR,
                    b"broken pipe writing to monitor over backchannel\0" as *const u8
                        as *const libc::c_char
                );
            }
            free(msg as *mut libc::c_void);
            loop {
                msg = TAILQ_FIRST!((*ec).monitor_messages);
                if msg.is_null() {
                    break;
                }
                //TAILQ_REMOVE(&ec->monitor_messages, msg, entries);
                if !((*msg).entries.tqe_next).is_null() {
                    (*(*msg).entries.tqe_next).entries.tqe_prev = (*msg).entries.tqe_prev;
                } else {
                    (*ec).monitor_messages.tqh_last = (*msg).entries.tqe_prev;
                }
                *(*msg).entries.tqe_prev = (*msg).entries.tqe_next;
                free(msg as *mut libc::c_void);
            }
            /* XXX - need new CMD_ type for monitor errors. */
            (*(*ec).cstat).type_0 = CMD_ERRNO;
            (*(*ec).cstat).val = errno!();
            sudo_ev_loopbreak_v1((*ec).evbase);
        }
        free(msg as *mut libc::c_void);
        break;
    }

}

/*
 * Fill in the exec closure and setup initial exec events.
 * Allocates events for the signal pipe and backchannel.
 * Forwarded signals on the backchannel are enabled on demand.
 */
unsafe extern "C" fn fill_exec_closure_pty(
    mut ec: *mut exec_closure_pty,
    mut cstat: *mut command_status,
    mut details: *mut command_details,
    mut ppgrp: pid_t,
    mut backchannel: libc::c_int,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);

    /* Fill in the non-event part of the closure. */
    (*ec).cmnd_pid = -(1 as libc::c_int);
    (*ec).ppgrp = ppgrp;
    (*ec).cstat = cstat;
    (*ec).details = details;
    (*ec).rows = user_details.ts_rows as libc::c_short;
    (*ec).cols = user_details.ts_cols as libc::c_short;
    //TAILQ_INIT!((*ec).monitor_messages);
    (*ec).monitor_messages.tqh_first = 0 as *mut monitor_message;
    (*ec).monitor_messages.tqh_last = &mut (*ec).monitor_messages.tqh_first;

    /* Setup event base and events. */
    let ref mut evbase0 = (*ec).evbase;
    *evbase0 = sudo_ev_base_alloc_v1();
    if (*ec).evbase.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }

    /* Event for command status via backchannel. */
    let ref mut backchannel_event0 = (*ec).backchannel_event;
    *backchannel_event0 = sudo_ev_alloc_v1(
        backchannel,
        (SUDO_EV_READ | SUDO_EV_PERSIST) as libc::c_short,
        Some(
            backchannel_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if ((*ec).backchannel_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2(
        (*ec).evbase,
        (*ec).backchannel_event,
        0 as *mut timespec,
        false,
    ) == -1
    {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"backchannel fd %d\n\0" as *const u8 as *const libc::c_char,
        backchannel
    );

    /* Events for local signals. */
    let ref mut sigint_event0 = (*ec).sigint_event;
    *sigint_event0 = sudo_ev_alloc_v1(
        SIGINT,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigint_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigint_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sigquit_event0 = (*ec).sigquit_event;
    *sigquit_event0 = sudo_ev_alloc_v1(
        SIGQUIT,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigquit_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigquit_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }
    
    let ref mut sigtstp_event0 = (*ec).sigtstp_event;
    *sigtstp_event0 = sudo_ev_alloc_v1(
        SIGTSTP,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigtstp_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigtstp_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sigterm_event0 = (*ec).sigterm_event;
    *sigterm_event0 = sudo_ev_alloc_v1(
        SIGTERM,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigterm_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigterm_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sighup_event0 = (*ec).sighup_event;
    *sighup_event0 = sudo_ev_alloc_v1(
        SIGHUP,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sighup_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sighup_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sigalrm_event0 = (*ec).sigalrm_event;
    *sigalrm_event0 = sudo_ev_alloc_v1(
        SIGALRM,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigalrm_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigalrm_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sigusr1_event0 = (*ec).sigusr1_event;
    *sigusr1_event0 = sudo_ev_alloc_v1(
        SIGUSR1,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigusr1_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigusr1_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sigusr2_event0 = (*ec).sigusr2_event;
    *sigusr2_event0 = sudo_ev_alloc_v1(
        SIGUSR2,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigusr2_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigusr2_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sigchld_event0 = (*ec).sigchld_event;
    *sigchld_event0 = sudo_ev_alloc_v1(
        SIGCHLD,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigchld_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigchld_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    let ref mut sigwinch_event0 = (*ec).sigwinch_event;
    *sigwinch_event0 = sudo_ev_alloc_v1(
        SIGWINCH,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_pty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if ((*ec).sigwinch_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2(
        (*ec).evbase,
        (*ec).sigwinch_event,
        0 as *mut timespec,
        false,
    ) == -1
    {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }

    /* The signal forwarding event gets added on demand. */
    let ref mut fwdchannel_event0 = (*ec).fwdchannel_event;
    *fwdchannel_event0 = sudo_ev_alloc_v1(
        backchannel,
        SUDO_EV_WRITE as libc::c_short,
        Some(
            fwdchannel_cb
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if ((*ec).fwdchannel_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }

    /* Set the default event base. */
    sudo_ev_base_setdef_v1((*ec).evbase);

    debug_return!();
}