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

