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