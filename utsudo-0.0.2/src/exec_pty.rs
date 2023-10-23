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