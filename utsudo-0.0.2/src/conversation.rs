/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unreachable_code,
    non_snake_case
)]

use crate::struct_macro::*;

use crate::ISSET;
use crate::SET;
use crate::_PATH_TTY;

extern "C" {
    static mut tgetpass_flags: libc::c_int;
    static mut sudo_debug_instance: libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn sudo_debug_get_active_instance_v1() -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn tgetpass(
        prompt: *const libc::c_char,
        timeout: libc::c_int,
        flags: libc::c_int,
        callback: *mut sudo_conv_callback,
    ) -> *mut libc::c_char;
    fn sudo_memset_s(v: *mut libc::c_void, smax: size_t, c: libc::c_int, n: size_t) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_debug_set_active_instance_v1(inst: libc::c_int) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;

    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;

}

pub const EOF: libc::c_int = -1;

// #define SUDO_CONV_PROMPT_ECHO_OFF   0x0001  /* do not echo user input */
// #define SUDO_CONV_PROMPT_ECHO_ON    0x0002  /* echo user input */
// #define SUDO_CONV_ERROR_MSG	    0x0003  /* error message */
// #define SUDO_CONV_INFO_MSG	    0x0004  /* informational message */
// #define SUDO_CONV_PROMPT_MASK	    0x0005  /* mask user input */
// #define SUDO_CONV_PROMPT_ECHO_OK    0x1000  /* flag: allow echo if no tty */
// #define SUDO_CONV_PREFER_TTY	    0x2000  /* flag: use tty if possible */
pub const SUDO_CONV_PROMPT_ECHO_OFF: libc::c_int = 0x0001;
pub const SUDO_CONV_PROMPT_ECHO_ON: libc::c_int = 0x0002;
pub const SUDO_CONV_ERROR_MSG: libc::c_int = 0x0003;
pub const SUDO_CONV_INFO_MSG: libc::c_int = 0x0004;
pub const SUDO_CONV_PROMPT_MASK: libc::c_int = 0x0005;
pub const SUDO_CONV_PROMPT_ECHO_OK: libc::c_int = 0x1000;
pub const SUDO_CONV_PREFER_TTY: libc::c_int = 0x2000;

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

#[no_mangle]
pub unsafe extern "C" fn sudo_conversation_1_7(
    mut num_msgs: libc::c_int,
    mut msgs: *const sudo_conv_message,
    mut replies: *mut sudo_conv_reply,
) -> libc::c_int {
    return sudo_conversation(num_msgs, msgs, replies, 0 as *mut sudo_conv_callback);
}

// sudo_conversation_printf 函数涉及可变参数，暂不重构，
// 函数实现放到了 lib/util/utsudo_variadic.c 文件中