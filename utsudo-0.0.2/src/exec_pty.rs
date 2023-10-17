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