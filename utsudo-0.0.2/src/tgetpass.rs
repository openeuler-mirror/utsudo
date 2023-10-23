/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(clashing_extern_declarations)]

use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

use crate::ISSET;
use crate::SET;
use crate::WIFSTOPPED;
use crate::_PATH_TTY;

/* Conversation callback API version major/minor */
// #define SUDO_CONV_CALLBACK_VERSION_MAJOR	1
// #define SUDO_CONV_CALLBACK_VERSION_MINOR	0
pub const SUDO_CONV_CALLBACK_VERSION_MAJOR: libc::c_int = 1;
pub const SUDO_CONV_CALLBACK_VERSION_MINOR: libc::c_int = 0;

/* Getters and setters for plugin API versions */
// #define SUDO_API_VERSION_GET_MAJOR(v) ((v) >> 16)
// #define SUDO_API_VERSION_GET_MINOR(v) ((v) & 0xffffU)
macro_rules! SUDO_API_VERSION_GET_MAJOR {
    ($v:expr) => {
        (($v) >> 16)
    };
}

// #define TGP_STDIN	0x02		/* read from stdin, not /dev/tty */
// #define TGP_ASKPASS	0x04		/* read from askpass helper program */
// #define TGP_BELL	0x20		/* bell on password prompt */
pub const TGP_STDIN: libc::c_int = 0x02;
pub const TGP_ASKPASS: libc::c_int = 0x04;
pub const TGP_BELL: libc::c_int = 0x20;

pub type tgetpass_errval = libc::c_uint;
pub const TGP_ERRVAL_READERROR: tgetpass_errval = 3;
pub const TGP_ERRVAL_NOPASSWORD: tgetpass_errval = 2;
pub const TGP_ERRVAL_TIMEOUT: tgetpass_errval = 1;
pub const TGP_ERRVAL_NOERROR: tgetpass_errval = 0;

extern "C" {
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(_set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn sudo_conf_askpass_path_v1() -> *const libc::c_char;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_fork_v1() -> pid_t;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_term_cbreak_v1(fd: libc::c_int) -> bool;
    fn sudo_term_noecho_v1(fd: libc::c_int) -> bool;
    fn sudo_term_restore_v1(fd: libc::c_int, flush: bool) -> bool;
    fn sudo_closefrom(_: libc::c_int);
    static mut user_details: user_details;
    fn getenv_unhooked(name: *const libc::c_char) -> *mut libc::c_char;
    static mut sudo_term_eof: libc::c_int;
    static mut sudo_term_erase: libc::c_int;
    static mut sudo_term_kill: libc::c_int;
}

static mut signo: [sig_atomic_t; 65] = [0; 65];

unsafe extern "C" fn suspend(
    mut signo_0: libc::c_int,
    mut callback: *mut sudo_conv_callback,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_CONV);

    if !callback.is_null()
        && SUDO_API_VERSION_GET_MAJOR!((*callback).version)
            != SUDO_CONV_CALLBACK_VERSION_MAJOR as libc::c_uint
    {
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
            b"callback major version mismatch, expected %u, got %u\0" as *const u8
                as *const libc::c_char,
            SUDO_CONV_CALLBACK_VERSION_MAJOR as libc::c_uint,
            SUDO_API_VERSION_GET_MAJOR!((*callback).version)
        );
        callback = 0 as *mut sudo_conv_callback;
    }

    if !callback.is_null() && ((*callback).on_suspend).is_some() {
        if ((*callback).on_suspend).expect("non-null function pointer")(
            signo_0,
            (*callback).closure,
        ) == -(1 as libc::c_int)
        {
            ret = -(1 as libc::c_int);
        }
    }

    kill(getpid(), signo_0);
    if !callback.is_null() && ((*callback).on_resume).is_some() {
        if ((*callback).on_resume).expect("non-null function pointer")(signo_0, (*callback).closure)
            == -(1 as libc::c_int)
        {
            ret = -(1 as libc::c_int);
        }
    }

    debug_return_int!(ret);
}

unsafe extern "C" fn tgetpass_display_error(mut errval: tgetpass_errval) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_CONV);

    match errval {
        TGP_ERRVAL_NOERROR => {}
        TGP_ERRVAL_TIMEOUT => {
            sudo_warnx!(b"timed out reading password\0" as *const u8 as *const libc::c_char,);
        }
        TGP_ERRVAL_NOPASSWORD => {
            sudo_warnx!(b"no password was provided\0" as *const u8 as *const libc::c_char,);
        }
        TGP_ERRVAL_READERROR => {
            sudo_warn!(b"unable to read password\0" as *const u8 as *const libc::c_char,);
        }
        _ => {}
    }
    debug_return!();
}

/*
 * Like getpass(3) but with timeout and echo flags.
 */
#[no_mangle]
pub unsafe extern "C" fn tgetpass(
    mut prompt: *const libc::c_char,
    mut timeout: libc::c_int,
    mut flags: libc::c_int,
    mut callback: *mut sudo_conv_callback,
) -> *mut libc::c_char {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savealrm: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut saveint: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savehup: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savequit: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut saveterm: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savetstp: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savettin: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savettou: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut askpass: *const libc::c_char = 0 as *const libc::c_char;
    static mut buf: [libc::c_char; (SUDO_CONV_REPL_MAX + 1) as usize] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut input: libc::c_int = 0;
    let mut output: libc::c_int = 0;
    let mut save_errno: libc::c_int = 0;
    let mut ttyfd: libc::c_int = 0;
    let mut need_restart: bool = false;
    let mut neednl: bool = false;
    let mut feedback: bool = ISSET!(flags, TGP_MASK) != 0;
    let mut errval: tgetpass_errval = TGP_ERRVAL_NOERROR;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_CONV);

    fflush(stdout);
    if askpass.is_null() {
        askpass = getenv_unhooked(b"SUDO_ASKPASS\0" as *const u8 as *const libc::c_char);
        if askpass.is_null() || *askpass as libc::c_int == '\0' as i32 {
            askpass = sudo_conf_askpass_path_v1();
        }
    }

    // restart:
    'restart: loop {
        /* Try to open /dev/tty if we are going to be using it for I/O. */
        ttyfd = -(1 as libc::c_int);
        if ISSET!(flags, TGP_STDIN | TGP_ASKPASS) == 0 {
            ttyfd = open(_PATH_TTY!(), O_RDWR);
            /* If no tty present and we need to disable echo, try askpass. */
            if ttyfd == -(1 as libc::c_int) && ISSET!(flags, TGP_ECHO | TGP_NOECHO_TRY) == 0 {
                if askpass.is_null()
                    || getenv_unhooked(b"DISPLAY\0" as *const u8 as *const libc::c_char).is_null()
                {
                    sudo_warnx!(b"a terminal is required to read the password; either use the -S option to read from standard input or configure an askpass helper\0" as *const u8 as *const libc::c_char, );
                    debug_return_str!(0 as *mut libc::c_char);
                }
                SET!(flags, TGP_ASKPASS);
            }
        }

        /* If using a helper program to get the password, run it instead. */
        if ISSET!(flags, TGP_ASKPASS) != 0 {
            if askpass.is_null() || *askpass as libc::c_int == '\0' as i32 {
                sudo_fatalx!(
                    b"no askpass program specified, try setting SUDO_ASKPASS\0" as *const u8
                        as *const libc::c_char,
                );
            }
            debug_return_str_masked!(sudo_askpass(askpass, prompt));
        }

        /* Reset state. */
        i = 0;
        while i < NSIG {
            ::core::ptr::write_volatile(&mut signo[i as usize] as *mut sig_atomic_t, 0);
            i += 1;
        }
        pass = 0 as *mut libc::c_char;
        save_errno = 0 as libc::c_int;
        need_restart = false;

        /* Use tty for reading/writing if available else use stdin/stderr. */
        if ttyfd == -(1 as libc::c_int) {
            input = STDIN_FILENO;
            output = STDERR_FILENO;
        } else {
            input = ttyfd;
            output = ttyfd;
        }
        /*
         * If we are using a tty but are not the foreground pgrp this will
         * return EINTR.  We send ourself SIGTTOU bracketed by callbacks.
         */
        if ISSET!(flags, TGP_ECHO) == 0 {
            loop {
                if feedback {
                    neednl = sudo_term_cbreak_v1(input);
                } else {
                    neednl = sudo_term_noecho_v1(input);
                }
                if (neednl as libc::c_int != 0) || (*__errno_location() != EINTR) {
                    break;
                }

                /* Received SIGTTOU, suspend the process. */
                if suspend(SIGTTOU, callback) == -(1 as libc::c_int) {
                    if input != STDIN_FILENO {
                        close(input);
                    }
                    debug_return_ptr!(0 as *mut libc::c_char);
                }
            } // !loop
        }

        /* Only use feedback mode when we can disable echo. */
        if !neednl {
            feedback = false;
        }

        /*
         * Catch signals that would otherwise cause the user to end
         * up with echo turned off in the shell.
         */
        memset(
            &mut sa as *mut sigaction as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<sigaction>() as libc::c_ulong,
        );
        sigemptyset(&mut sa.sa_mask);
        sa.sa_flags = 0;
        sa.__sigaction_handler.sa_handler =
            Some(tgetpass_handler as unsafe extern "C" fn(libc::c_int) -> ());
        sigaction(SIGALRM, &mut sa, &mut savealrm);
        sigaction(SIGINT, &mut sa, &mut saveint);
        sigaction(SIGHUP, &mut sa, &mut savehup);
        sigaction(SIGQUIT, &mut sa, &mut savequit);
        sigaction(SIGTERM, &mut sa, &mut saveterm);
        sigaction(SIGTSTP, &mut sa, &mut savetstp);
        sigaction(SIGTTIN, &mut sa, &mut savettin);
        sigaction(SIGTTOU, &mut sa, &mut savettou);

        'restore: loop {
            if ISSET!(flags, TGP_BELL) != 0 && output != STDERR_FILENO {
                /* Ring the bell if requested and there is a tty. */
                if write(
                    output,
                    b"\x07\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1,
                ) == -1
                {
                    break 'restore;
                }
            }
            if !prompt.is_null() {
                if write(output, prompt as *const libc::c_void, strlen(prompt)) == -1 {
                    break 'restore;
                }
            }

            if timeout > 0 {
                alarm(timeout as libc::c_uint);
            }
            pass = getln(
                input,
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                feedback,
                &mut errval,
            );
            alarm(0 as libc::c_uint);
            save_errno = *__errno_location();

            if neednl || pass.is_null() {
                if write(
                    output,
                    b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                ) == -(1 as libc::c_int) as libc::c_long
                {
                    break 'restore;
                }
            }
            tgetpass_display_error(errval);
            break 'restore;
        } // ! 'restore :loop

        // restore:
        /* Restore old signal handlers. */
        sigaction(SIGALRM, &mut savealrm, 0 as *mut sigaction);
        sigaction(SIGINT, &mut saveint, 0 as *mut sigaction);
        sigaction(SIGHUP, &mut savehup, 0 as *mut sigaction);
        sigaction(SIGQUIT, &mut savequit, 0 as *mut sigaction);
        sigaction(SIGTERM, &mut saveterm, 0 as *mut sigaction);
        sigaction(SIGTSTP, &mut savetstp, 0 as *mut sigaction);
        sigaction(SIGTTIN, &mut savettin, 0 as *mut sigaction);
        sigaction(SIGTTOU, &mut savettou, 0 as *mut sigaction);

        /* Restore old tty settings. */
        if ISSET!(flags, TGP_ECHO) == 0 {
            /* Restore old tty settings if possible. */
            sudo_term_restore_v1(input, true);
        }

        if input != STDIN_FILENO {
            close(input);
        }

        /*
         * If we were interrupted by a signal, resend it to ourselves
         * now that we have restored the signal handlers.
         */
        while i < NSIG {
            if signo[i as usize] != 0 {
                match i {
                    SIGALRM => {}
                    SIGTSTP | SIGTTIN | SIGTTOU => {
                        if suspend(i, callback) == 0 {
                            need_restart = true;
                        }
                    }
                    _ => {
                        kill(getpid(), i);
                    }
                } // !match i
            }
            i += 1;
        } // !while i < NSIG

        if !need_restart {
            break 'restart;
        }
    } // !‘restart：loop

    if save_errno != 0 {
        *__errno_location() = save_errno;
    }

    debug_return_str_masked!(pass);
}

/*
 * Fork a child and exec sudo-askpass to get the password from the user.
 */
unsafe extern "C" fn sudo_askpass(
    mut askpass: *const libc::c_char,
    mut prompt: *const libc::c_char,
) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 256] = [0; 256];
    static mut pass: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savechld: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut errval: tgetpass_errval = TGP_ERRVAL_NOERROR;
    let mut pfd: [libc::c_int; 2] = [0; 2];
    let mut status: libc::c_int = 0;
    let mut child: pid_t = 0;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_CONV);

    /* Set SIGCHLD handler to default since we call waitpid() below. */
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    sa.__sigaction_handler.sa_handler = None;
    sigaction(SIGCHLD, &mut sa, &mut savechld);

    if pipe(pfd.as_mut_ptr()) == -(1 as libc::c_int) {
        sudo_fatal!(b"unable to create pipe\0" as *const u8 as *const libc::c_char,);
    }

    child = sudo_debug_fork_v1();
    if child == -(1 as libc::c_int) {
        sudo_fatal!(b"unable to fork\0" as *const u8 as *const libc::c_char,);
    }

    if child == 0 {
        /* child, point stdout to output side of the pipe and exec askpass */
        if dup2(pfd[1], STDOUT_FILENO) == -(1 as libc::c_int) {
            sudo_warn!(b"dup2\0" as *const u8 as *const libc::c_char,);
            _exit(255);
        }
        if setuid(ROOT_UID as libc::c_uint) == -(1 as libc::c_int) {
            sudo_warn!(
                b"setuid(%d)\0" as *const u8 as *const libc::c_char,
                ROOT_UID
            );
        }
        if setgid(user_details.gid) != 0 {
            sudo_warn!(
                b"unable to set gid to %u\0" as *const u8 as *const libc::c_char,
                user_details.gid
            );
            _exit(255);
        }
        if setuid(user_details.uid) != 0 {
            sudo_warn!(
                b"unable to set uid to %u\0" as *const u8 as *const libc::c_char,
                user_details.uid
            );
            _exit(255);
        }
        sudo_closefrom(STDERR_FILENO + 1);
        execl(askpass, askpass, prompt, 0 as *mut libc::c_char);
        sudo_warn!(
            b"unable to run %s\0" as *const u8 as *const libc::c_char,
            askpass
        );
        _exit(255);
    }

    /* Get response from child (askpass). */
    close(pfd[1]);
    pass = getln(
        pfd[0 as libc::c_int as usize],
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        0 as libc::c_int != 0,
        &mut errval,
    );
    close(pfd[0]);

    tgetpass_display_error(errval);

    /* Wait for child to exit. */
    loop {
        let mut rv: pid_t = waitpid(child, &mut status, 0);
        if rv == -1 && *__errno_location() != EINTR {
            break;
        }
        if rv != -1 && !WIFSTOPPED!(status) {
            break;
        }
    }

    if pass.is_null() {
        *__errno_location() = EINTR; /* make cancel button simulate ^C */
    }

    /* Restore saved SIGCHLD handler. */
    sigaction(SIGCHLD, &mut savechld, 0 as *mut sigaction);

    debug_return_str_masked!(pass);
}

unsafe extern "C" fn getln(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
    mut feedback: bool,
    mut errval: *mut tgetpass_errval,
) -> *mut libc::c_char {
    let mut left: size_t = bufsiz;
    let mut nr: ssize_t = -(1 as libc::c_int) as ssize_t;
    let mut cp: *mut libc::c_char = buf;
    let mut c: libc::c_char = '\0' as i32 as libc::c_char;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_CONV);

    *errval = TGP_ERRVAL_NOERROR;

    if left == 0 {
        *errval = TGP_ERRVAL_READERROR;
        *__errno_location() = EINVAL;
        debug_return_str!(0 as *mut libc::c_char); /* sanity */
    }

    loop {
        left = left.wrapping_sub(1);
        if !(left != 0) {
            break;
        }
        nr = read(fd, &mut c as *mut libc::c_char as *mut libc::c_void, 1);
        if nr != 1 || c as libc::c_int == '\n' as i32 || c as libc::c_int == '\r' as i32 {
            break;
        }

        if feedback {
            if c as libc::c_int == sudo_term_eof {
                nr = 0 as libc::c_int as ssize_t;
                break;
            } else if c as libc::c_int == sudo_term_kill {
                while cp > buf {
                    if write(
                        fd,
                        b"\x08 \x08\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                        3 as libc::c_int as size_t,
                    ) == -(1 as libc::c_int) as libc::c_long
                    {
                        break;
                    }
                    cp = cp.offset(-1);
                }
                cp = buf;
                left = bufsiz;
                continue;
            } else if c as libc::c_int == sudo_term_erase {
                if cp > buf {
                    let mut _y: ssize_t = write(
                        fd,
                        b"\x08 \x08\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                        3 as libc::c_int as size_t,
                    );
                    cp = cp.offset(-1);
                    left = left.wrapping_add(1);
                }
                continue;
            } else {
                let mut _y_0: ssize_t = write(
                    fd,
                    b"*\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            }
        } // ! feedback
        let fresh0 = cp;
        cp = cp.offset(1);
        *fresh0 = c;
    } // ! loop

    *cp = '\0' as i32 as libc::c_char;

    if feedback {
        /* erase stars */
        while cp > buf {
            if write(
                fd,
                b"\x08 \x08\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as size_t,
            ) == -(1 as libc::c_int) as libc::c_long
            {
                break;
            }
            cp = cp.offset(-1);
        }
    }

    match nr {
        -1 => {
            /* Read error */
            if *__errno_location() == EINTR {
                if signo[SIGALRM as libc::c_int as usize] == 1 {
                    *errval = TGP_ERRVAL_TIMEOUT;
                }
            } else {
                *errval = TGP_ERRVAL_READERROR;
            }
            debug_return_str!(0 as *mut libc::c_char)
        }
        0 => {
            /* EOF is only an error if no bytes were read. */
            if left == bufsiz.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                *errval = TGP_ERRVAL_NOPASSWORD;
                debug_return_str!(0 as *mut libc::c_char)
            }
        }
        /* FALLTHROUGH */
        _ => {}
    }
    debug_return_str_masked!(buf);
}

unsafe extern "C" fn tgetpass_handler(mut s: libc::c_int) {
    ::core::ptr::write_volatile(&mut signo[s as usize] as *mut sig_atomic_t, 1);
}