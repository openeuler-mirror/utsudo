/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_variables, clashing_extern_declarations)]
use crate::struct_macro::*;
use crate::ISSET;
use crate::USER_SIGNALED;
use crate::WEXITSTATUS;
use crate::WIFSIGNALED;
use crate::WIFSTOPPED;
use crate::WSTOPSIG;
use crate::WTERMSIG;
use crate::_PATH_TTY;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
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

/* Signal callback */
#[inline]
unsafe extern "C" fn signal_cb_nopty(
    mut signo: libc::c_int,
    mut what: libc::c_int,
    mut v: *mut libc::c_void,
) {
    let mut sc: *mut sudo_ev_siginfo_container = v as *mut sudo_ev_siginfo_container;
    let mut ec: *mut exec_closure_nopty = (*sc).closure as *mut exec_closure_nopty;
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; 32];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    if (*ec).cmnd_pid == -1 {
        debug_return!();
    }
    if sudo_sig2str(signo, signame.as_mut_ptr()) == -1 {
        snprintf(
            signame.as_mut_ptr(),
            std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            signo,
        );
    }
    sudo_debug_printf!(
        SUDO_DEBUG_DIAG,
        b"%s: evbase %p, command: %d, signo %s(%d), cstat %p\0" as *const u8 as *const libc::c_char,
        stdext::function_name!().as_ptr(),
        (*ec).evbase,
        (*ec).cmnd_pid,
        signame,
        signo,
        (*ec).cstat
    );
    match signo {
        SIGCHLD => {
            handle_sigchld_nopty(ec);
            if (*ec).cmnd_pid == -1 {
                /* Command exited or was killed, exit event loop. */
                sudo_ev_loopexit_v1((*ec).evbase);
            }
            debug_return!();
        }
        SIGINT | SIGQUIT | SIGTSTP => {
            /*
             * Only forward user-generated signals not sent by a process in
             * the command's own process group.  Signals sent by the kernel
             * may include SIGTSTP when the user presses ^Z.  Curses programs
             * often trap ^Z and send SIGTSTP to their own pgrp, so we don't
             * want to send an extra SIGTSTP.
             */
            if !USER_SIGNALED!((*sc).siginfo) {
                debug_return!();
            }
            if (*(*sc).siginfo)._sifields._kill.si_pid != 0 {
                let mut si_pgrp: pid_t = getpgid((*(*sc).siginfo)._sifields._kill.si_pid);
                if si_pgrp != -1 {
                    if si_pgrp == (*ec).ppgrp || si_pgrp == (*ec).cmnd_pid {
                        debug_return!();
                    } else if (*(*sc).siginfo)._sifields._kill.si_pid == (*ec).cmnd_pid {
                        debug_return!();
                    }
                }
            }
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
                if si_pgrp != -1 {
                    if si_pgrp == (*ec).ppgrp || si_pgrp == (*ec).cmnd_pid {
                        debug_return!();
                    } else if (*(*sc).siginfo)._sifields._kill.si_pid == (*ec).cmnd_pid {
                        debug_return!();
                    }
                }
            }
        }
    }
    /* Send signal to command. */
    if signo == SIGALRM {
        terminate_command((*ec).cmnd_pid, false);
    } else if kill((*ec).cmnd_pid, signo) != 0 {
        sudo_warn!(
            b"kill(%d, SIG%s)\0" as *const u8 as *const libc::c_char,
            (*ec).cmnd_pid,
            signame
        );
    }
    debug_return!();
}

/*
 * Fill in the exec closure and setup initial exec events.
 * Allocates events for the signal pipe and error pipe.
 */
#[inline]
unsafe extern "C" fn fill_exec_closure_nopty(
    mut ec: *mut exec_closure_nopty,
    mut cstat: *mut command_status,
    mut details: *mut command_details,
    mut errfd: libc::c_int,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /* Fill in the non-event part of the closure. */
    (*ec).ppgrp = getpgrp();
    let ref mut cstat0 = (*ec).cstat;
    *cstat0 = cstat;
    let ref mut details0 = (*ec).details;
    *details0 = details;
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
    /* Event for command status via errfd. */
    let ref mut errpipe_event0 = (*ec).errpipe_event;
    *errpipe_event0 = sudo_ev_alloc_v1(
        errfd,
        (SUDO_EV_READ | SUDO_EV_PERSIST) as libc::c_short,
        Some(errpipe_cb as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> ()),
        ec as *mut libc::c_void,
    );
    if ((*ec).errpipe_event).is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).errpipe_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"error pipe fd %d\n\0" as *const u8 as *const libc::c_char,
        errfd
    );
    /* Events for local signals. */
    let ref mut sigint_event0 = (*ec).sigint_event;
    *sigint_event0 = sudo_ev_alloc_v1(
        SIGINT,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_nopty
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
            signal_cb_nopty
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
            signal_cb_nopty
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
            signal_cb_nopty
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
            signal_cb_nopty
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
            signal_cb_nopty
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
    let ref mut sigpipe_event0 = (*ec).sigpipe_event;
    *sigpipe_event0 = sudo_ev_alloc_v1(
        SIGPIPE,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_nopty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigpipe_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigpipe_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }
    let ref mut sigusr1_event0 = (*ec).sigusr1_event;
    *sigusr1_event0 = sudo_ev_alloc_v1(
        SIGUSR1,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_nopty
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
            signal_cb_nopty
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
            signal_cb_nopty
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
    let ref mut sigcont_event0 = (*ec).sigcont_event;
    *sigcont_event0 = sudo_ev_alloc_v1(
        SIGCONT,
        SUDO_EV_SIGINFO as libc::c_short,
        Some(
            signal_cb_nopty
                as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
        ),
        ec as *mut libc::c_void,
    );
    if (*ec).sigcont_event.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    if sudo_ev_add_v2((*ec).evbase, (*ec).sigcont_event, 0 as *mut timespec, false) == -1 {
        sudo_fatal!(b"unable to add event to queue \0" as *const u8 as *const libc::c_char,);
    }
    /* Set the default event base. */
    sudo_ev_base_setdef_v1((*ec).evbase);
    debug_return!();
}

/*
 * Free the dynamically-allocated contents of the exec closure.
 */
unsafe extern "C" fn free_exec_closure_nopty(mut ec: *mut exec_closure_nopty) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    sudo_ev_base_free_v1((*ec).evbase);
    sudo_ev_free_v1((*ec).errpipe_event);
    sudo_ev_free_v1((*ec).sigint_event);
    sudo_ev_free_v1((*ec).sigquit_event);
    sudo_ev_free_v1((*ec).sigtstp_event);
    sudo_ev_free_v1((*ec).sigterm_event);
    sudo_ev_free_v1((*ec).sighup_event);
    sudo_ev_free_v1((*ec).sigalrm_event);
    sudo_ev_free_v1((*ec).sigpipe_event);
    sudo_ev_free_v1((*ec).sigusr1_event);
    sudo_ev_free_v1((*ec).sigusr2_event);
    sudo_ev_free_v1((*ec).sigchld_event);
    sudo_ev_free_v1((*ec).sigcont_event);
    sudo_ev_free_v1((*ec).siginfo_event);
    debug_return!();
}
/*
 * Execute a command and wait for it to finish.
 */
#[no_mangle]
pub unsafe extern "C" fn exec_nopty(
    mut details: *mut command_details,
    mut cstat: *mut command_status,
) {
    let mut ec: exec_closure_nopty = {
        let mut init = exec_closure_nopty {
            cmnd_pid: 0 as libc::c_int,
            ppgrp: 0,
            cstat: 0 as *mut command_status,
            details: 0 as *mut command_details,
            evbase: 0 as *mut sudo_event_base,
            errpipe_event: 0 as *mut sudo_event,
            sigint_event: 0 as *mut sudo_event,
            sigquit_event: 0 as *mut sudo_event,
            sigtstp_event: 0 as *mut sudo_event,
            sigterm_event: 0 as *mut sudo_event,
            sighup_event: 0 as *mut sudo_event,
            sigalrm_event: 0 as *mut sudo_event,
            sigpipe_event: 0 as *mut sudo_event,
            sigusr1_event: 0 as *mut sudo_event,
            sigusr2_event: 0 as *mut sudo_event,
            sigchld_event: 0 as *mut sudo_event,
            sigcont_event: 0 as *mut sudo_event,
            siginfo_event: 0 as *mut sudo_event,
        };
        init
    };
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let mut errpipe: [libc::c_int; 2] = [0; 2];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    /*
     * The policy plugin's session init must be run before we fork
     * or certain pam modules won't be able to track their state.
     */
    if policy_init_session(details) != true as libc::c_int {
        sudo_fatalx!(
            b"policy plugin failed session initialization\0" as *const u8 as *const libc::c_char,
        );
    }
    /*
     * We use a pipe to get errno if execve(2) fails in the child.
     */
    if pipe2(errpipe.as_mut_ptr(), O_CLOEXEC as libc::c_int) != 0 {
        sudo_fatal!(b"unable to create pipe \0" as *const u8 as *const libc::c_char,);
    }
    /*
     * Block signals until we have our handlers setup in the parent so
     * we don't miss SIGCHLD if the command exits immediately.
     */
    sigfillset(&mut set);
    sigprocmask(SIG_BLOCK, &mut set, &mut oset);
    /* Check for early termination or suspend signals before we fork. */
    if sudo_terminated(cstat) {
        sigprocmask(SIG_SETMASK, &mut oset, 0 as *mut sigset_t);
        debug_return!();
    }
    if (*details).flags & 0x800 as libc::c_int != 0 {
        if selinux_setup(
            (*details).selinux_role,
            (*details).selinux_type,
            (*details).tty,
            -(1 as libc::c_int),
            1 as libc::c_int != 0,
        ) == -(1 as libc::c_int)
        {
            (*cstat).type_0 = 1 as libc::c_int;
            (*cstat).val = *__errno_location();
            sudo_debug_exit_v1(
                (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"exec_nopty\0"))
                    .as_ptr(),
                b"exec_nopty.c\0" as *const u8 as *const libc::c_char,
                387 as libc::c_int,
                sudo_debug_subsys,
            );
            return;
        }
    }
    ec.cmnd_pid = sudo_debug_fork_v1();
    match ec.cmnd_pid {
        -1 => {
            sudo_fatal!(b"unable to fork \0" as *const u8 as *const libc::c_char,);
        }
        0 => {
            /* child */
            sigprocmask(SIG_SETMAS, &mut oset, 0 as *mut sigset_t);
            close(errpipe[0 as usize]);
            exec_cmnd(details, errpipe[1 as usize]);
            while write(
                errpipe[1 as usize],
                __errno_location() as *const libc::c_void,
                std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) == -1 as ssize_t
            {
                if *__errno_location() != EINTR {
                    break;
                }
            }
            sudo_debug_exit_int_v1(
                (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"exec_nopty\0"))
                    .as_ptr(),
                b"exec_nopty.c\0" as *const u8 as *const libc::c_char,
                line!() as libc::c_int,
                sudo_debug_subsys,
                1,
            );
            _exit(1);
        }
        _ => {}
    }
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"executed %s, pid %d \0" as *const u8 as *const libc::c_char,
        (*details).command,
        ec.cmnd_pid
    );
    close(errpipe[1 as usize]);
    /* No longer need execfd. */
    if (*details).execfd != -1 {
        close((*details).execfd);
        (*details).execfd = -1
    }
    /* Set command timeout if specified. */
    if ISSET!((*details).flags, CD_SET_TIMEOUT) != 0 {
        alarm((*details).timeout as libc::c_uint);
    }
    /*
     * Fill in exec closure, allocate event base, signal events and
     * the error pipe event.
     */
    fill_exec_closure_nopty(&mut ec, cstat, details, errpipe[0 as usize]);
    /* Restore signal mask now that signal handlers are setup. */
    sigprocmask(SIG_SETMASK, &mut oset, 0 as *mut sigset_t);
    /*
     * Non-pty event loop.
     * Wait for command to exit, handles signals and the error pipe.
     */
    if sudo_ev_dispatch_v1(ec.evbase) == -1 {
        sudo_warn!(b"error in event loop \0" as *const u8 as *const libc::c_char,);
    }
    if sudo_ev_got_break_v1(ec.evbase) {
        /* error from callback */
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR,
            b"event loop exited prematurely \0" as *const u8 as *const libc::c_char
        );
        /* kill command */
        terminate_command(ec.cmnd_pid, true);
    }
    /* Free things up. */
    free_exec_closure_nopty(&mut ec);
    debug_return!();
}

/*
 * Wait for command status after receiving SIGCHLD.
 * If the command exits, fill in cstat and stop the event loop.
 * If the command stops, save the tty pgrp, suspend sudo, then restore
 * the tty pgrp when sudo resumes.
 */
unsafe extern "C" fn handle_sigchld_nopty(mut ec: *mut exec_closure_nopty) {
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    let mut signame: [libc::c_char; SIG2STR_MAX as usize] = [0; SIG2STR_MAX as usize];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EXEC);
    loop {
        pid = waitpid((*ec).cmnd_pid, &mut status, WUNTRACED | WNOHANG);
        if !(pid == -(1 as libc::c_int) && *__errno_location() == EINTR) {
            break;
        }
        break;
    }
    match pid {
        0 => {
            /* waitpid() will return 0 for SIGCONT, which we don't care about */
            debug_return!();
        }
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
    if WIFSTOPPED!(status) {
        /*
         * Save the controlling terminal's process group so we can restore it
         * after we resume, if needed.  Most well-behaved shells change the
         * pgrp back to its original value before suspending so we must
         * not try to restore in that case, lest we race with the command upon
         * resume, potentially stopping sudo with SIGTTOU while the command
         * continues to run.
         */
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
        let mut saved_pgrp: pid_t = -(1 as libc::c_int);
        let mut fd: libc::c_int = 0;
        let mut signo: libc::c_int = WSTOPSIG!(status);
        if sudo_sig2str(signo, signame.as_mut_ptr()) == -(1 as libc::c_int) {
            snprintf(
                signame.as_mut_ptr(),
                std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                signo,
            );
        }
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: command (%d) stopped, SIG%s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            (*ec).cmnd_pid,
            signame
        );
        fd = open(_PATH_TTY!(), O_RDWR);
        if fd != -(1 as libc::c_int) {
            saved_pgrp = tcgetpgrp(fd);
            if saved_pgrp == -(1 as libc::c_int) {
                close(fd);
                fd = -(1 as libc::c_int);
            }
        }
        if saved_pgrp != -(1 as libc::c_int) {
            /*
             * Command was stopped trying to access the controlling terminal.
             * If the command has a different pgrp and we own the controlling
             * terminal, give it to the command's pgrp and let it continue.
             */
            if signo == SIGTTOU || signo == SIGTTIN {
                if saved_pgrp == (*ec).ppgrp {
                    let mut cmnd_pgrp: pid_t = getpgid((*ec).cmnd_pid);
                    if cmnd_pgrp != (*ec).ppgrp {
                        if tcsetpgrp_nobg(fd, cmnd_pgrp) == 0 {
                            if killpg(cmnd_pgrp, SIGCONT) != 0 {
                                sudo_warn!(
                                    b"kill(%d, SIGCONT)\0" as *const u8 as *const libc::c_char,
                                    cmnd_pgrp
                                );
                            }
                            close(fd);
                            debug_return!();
                        }
                    }
                }
            }
        }
        if signo == SIGTSTP {
            memset(
                &mut sa as *mut sigaction as *mut libc::c_void,
                0,
                std::mem::size_of::<sigaction>() as libc::c_ulong,
            );
            sigemptyset(&mut sa.sa_mask);
            sa.sa_flags = SA_RESTART;
            sa.__sigaction_handler.sa_handler = None;
            if sudo_sigaction(
                SIGTSTP,
                &mut sa as *mut sigaction,
                &mut osa as *mut sigaction,
            ) != 0
            {
                sudo_warn!(
                    b"unable to set handler for signal %d\0" as *const u8 as *const libc::c_char,
                    SIGTSTP
                );
            }
        }
        if kill(getpid(), signo) != 0 {
            sudo_warn!(
                b"kill(%d, SIG%s)0" as *const u8 as *const libc::c_char,
                getpid(),
                signame
            );
        }
        if signo == SIGTSTP {
            if sudo_sigaction(SIGTSTP, &mut osa, 0 as *mut sigaction) != 0 {
                sudo_warn!(
                    b"unable to restore handler for signal %d\0" as *const u8
                        as *const libc::c_char,
                    SIGTSTP
                );
            }
        }
        if saved_pgrp != -(1 as libc::c_int) {
            /*
             * On resume, restore foreground process group, if different.
             * Otherwise, we cannot resume some shells (pdksh).
             *
             * It is possible that we are no longer the foreground process so
             * use tcsetpgrp_nobg() to prevent sudo from receiving SIGTTOU.
             */
            if saved_pgrp != (*ec).ppgrp {
                tcsetpgrp_nobg(fd, saved_pgrp);
            }
            close(fd);
        }
    } else {
        /* Command has exited or been killed, we are done. */
        if WIFSIGNALED!(status) {
            if sudo_sig2str(WTERMSIG!(status), signame.as_mut_ptr()) == -1 {
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
                (*ec).cmnd_pid,
                signame
            );
        } else {
            sudo_debug_printf!(
                SUDO_DEBUG_INFO,
                b"%s: command (%d) exited: %d\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                (*ec).cmnd_pid,
                WEXITSTATUS!(status)
            );
        }
        /* Don't overwrite execve() failure with command exit status. */
        if (*(*ec).cstat).type_0 == CMD_INVALID {
            (*(*ec).cstat).type_0 = CMD_WSTATUS;
            (*(*ec).cstat).val = status;
        } else {
            sudo_debug_printf!(
                SUDO_DEBUG_WARN,
                b"%s: not overwriting command status %d,%d with %d,%d\0" as *const u8
                    as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                (*(*ec).cstat).type_0,
                (*(*ec).cstat).val,
                CMD_WSTATUS,
                status
            );
        }
        (*ec).cmnd_pid = -(1 as libc::c_int);
    }
}
