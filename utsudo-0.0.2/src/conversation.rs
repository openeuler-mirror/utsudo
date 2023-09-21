

/*
 * Sudo conversation function.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_conversation(
    mut num_msgs: libc::c_int,
    mut msgs: *const sudo_conv_message,
    mut replies: *mut sudo_conv_reply,
    mut callback: *mut sudo_conv_callback,
) -> libc::c_int {
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let conv_debug_instance: libc::c_int = sudo_debug_get_active_instance_v1();

    sudo_debug_set_active_instance_v1(sudo_debug_instance);

    'err: loop {
        loop {
            if !(n < num_msgs) {
                break;
            }

            let mut msg: *const sudo_conv_message =
                &*msgs.offset(n as isize) as *const sudo_conv_message;
            let mut flags: libc::c_int = tgetpass_flags;
            let mut fp: *mut FILE = stdout;

            'instance: loop {
                'read_pass: loop {
                    match (*msg).msg_type & 0xff as libc::c_int {
                        SUDO_CONV_PROMPT_ECHO_ON => {
                            SET!(flags, TGP_ECHO);
                            break 'read_pass;
                        }
                        SUDO_CONV_PROMPT_MASK | SUDO_CONV_PROMPT_ECHO_OFF => {
                            if (*msg).msg_type & 0xff as libc::c_int == SUDO_CONV_PROMPT_MASK {
                                SET!(flags, TGP_MASK);
                            }
                            /* FALLTHROUGH */
                            if ISSET!((*msg).msg_type, SUDO_CONV_PROMPT_ECHO_OK) != 0 {
                                SET!(flags, TGP_NOECHO_TRY);
                            }
                            break 'read_pass;
                        }

                        SUDO_CONV_ERROR_MSG | SUDO_CONV_INFO_MSG => {
                            if (*msg).msg_type & 0xff as libc::c_int == SUDO_CONV_ERROR_MSG {
                                fp = stderr;
                            }
                            /* FALLTHROUGH */
                            if !((*msg).msg).is_null() {
                                if ISSET!((*msg).msg_type, SUDO_CONV_PREFER_TTY) != 0 {
                                    /* Try writing to /dev/tty first. */
                                    fd = open(_PATH_TTY!(), O_WRONLY);
                                    if fd != -(1 as libc::c_int) {
                                        let mut nwritten: ssize_t = write(
                                            fd,
                                            (*msg).msg as *const libc::c_void,
                                            strlen((*msg).msg),
                                        );
                                        close(fd);
                                        if nwritten != -(1 as libc::c_int) as libc::c_long {
                                            break 'instance;
                                        }
                                    }
                                }
                                if fputs((*msg).msg, fp) == EOF {
                                    break 'err;
                                }
                            }
                            break 'instance;
                        }
                        _ => {
                            break 'err;
                        }
                    }
                    break;
                }
                /* Read the password unless interrupted. */
                pass = tgetpass((*msg).msg, (*msg).timeout, flags, callback);
                if pass.is_null() {
                    break 'err;
                }
                (*replies.offset(n as isize)).reply = strdup(pass);
                if ((*replies.offset(n as isize)).reply).is_null() {
                    sudo_fatalx_nodebug_v1(
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                        ),
                        b"sudo_conversation\0" as *const u8 as *const libc::c_char,
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                }
                sudo_memset_s(
                    pass as *mut libc::c_void,
                    SUDO_CONV_REPL_MAX as size_t,
                    0,
                    strlen(pass as *const libc::c_char),
                );
                break 'instance;
            }
            n += 1;
        }
        sudo_debug_set_active_instance_v1(conv_debug_instance);
        return 0;
    }
    /* Zero and free allocated memory and return an error. */
    if !replies.is_null() {
        loop {
            let mut repl: *mut sudo_conv_reply =
                &mut *replies.offset(n as isize) as *mut sudo_conv_reply;
            if !((*repl).reply).is_null() {
                sudo_memset_s(
                    (*repl).reply as *mut libc::c_void,
                    SUDO_CONV_REPL_MAX as size_t,
                    0,
                    strlen((*repl).reply),
                );
                free((*repl).reply as *mut libc::c_void);
                let ref mut repl = (*repl).reply;
                *repl = 0 as *mut libc::c_char;
            }
            let num = n;
            n = n - 1;
            if num == 0 {
                break;
            }
        }
    }

    sudo_debug_set_active_instance_v1(conv_debug_instance);
    return -(1 as libc::c_int);
}