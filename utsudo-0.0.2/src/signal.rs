/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_imports)]
use crate::struct_macro::*;
use crate::sudo_debug_printf2_v1;
use crate::sudo_warn_nodebug_v1;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signal_state {
    pub signo: libc::c_int,
    pub restore: libc::c_int,
    pub sa: sigaction,
}
static mut saved_signals: [signal_state; 14] = [
    {
        let mut init = signal_state {
            signo: 14 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 17 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 18 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 1 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 2 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 13 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 3 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 15 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 20 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 21 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 22 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 10 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: 12 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
    {
        let mut init = signal_state {
            signo: -1 as libc::c_int,
            restore: 0,
            sa: sigaction {
                __sigaction_handler: Signal_handler { sa_handler: None },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0 as libc::c_int,
                sa_restorer: None,
            },
        };
        init
    },
];

type sig_atomic_t = libc::c_int;
static mut pending_signals: [sig_atomic_t; 65] = [0; 65];
unsafe extern "C" fn sudo_handler(mut signo: libc::c_int) {
    pending_signals[signo as usize] = 1;
}
#[no_mangle]
pub unsafe extern "C" fn signal_pending(mut signo: libc::c_int) -> bool {
    return pending_signals[signo as usize] == 1;
}

extern "C" {
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn memset(__s: *mut libc::c_void, __c: libc::c_int, __n: size_t) -> *mut libc::c_void;
    fn sigfillset(__set: *mut __sigset_t) -> libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn save_signals() {
    let mut ss: *mut signal_state = 0 as *mut signal_state;
    //define debug_decl(save_signals,SUDO_DEBUG_MAIN);
    debug_decl!(save_signals, SUDO_DEBUG_MAIN);
    //end of define
    //as_mut_ptr()
    ss = saved_signals.as_mut_ptr();
    while (*ss).signo != -1 {
        if sigaction((*ss).signo, 0 as *const sigaction, &mut (*ss).sa) != 0 {
            //define sudo_warn(U_("unable to save handler for signal %d"),ss->signo);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to save handler for signal %d\0" as *const u8 as *const libc::c_char
                ),
                (*ss).signo
            );
            sudo_warn!(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to save handler for signal %d\0" as *const u8 as *const libc::c_char
                ),
                (*ss).signo
            );
            //end of define
        }
        ss = ss.offset(1);
    }
    //define debug_return;
    debug_return!();
    //end of define
}

#[no_mangle]
pub unsafe extern "C" fn restore_signals() {
    let mut ss: *mut signal_state = 0 as *mut signal_state;
    //define debug_decl(restore_signals,SUDO_DEBUG_MAIN);
    debug_decl!(restore_signals, SUDO_DEBUG_MAIN);
    //end of define
    ss = saved_signals.as_mut_ptr();
    while (*ss).signo != -1 {
        if (*ss).restore != 0 {
            //define sudo_debug_printf();  if,elif,else...
            sudo_debug_printf!(
                SUDO_DEBUG_INFO,
                b"signal.c\0" as *const u8 as *const libc::c_char,
                (*ss).signo,
                if (*ss).sa.__sigaction_handler.sa_handler
                    == ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t
                    )
                {
                    b"SIG_IGN\0" as *const u8 as *const libc::c_char
                } else if ((*ss).sa.__sigaction_handler.sa_handler).is_none() {
                    b"SIG_DFL\0" as *const u8 as *const libc::c_char
                } else {
                    b"???\0" as *const u8 as *const libc::c_char
                }
            );
            //end of define
            if sigaction((*ss).signo, &mut (*ss).sa, 0 as *mut sigaction) != 0 {
                //define sudo_warn(U_("unable to restore handler for signal %d"),ss->signo);
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to restore handler for signal %d\0" as *const u8
                            as *const libc::c_char
                    ),
                    (*ss).signo
                );
                sudo_warn!(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to restore handler for signal %d\0" as *const u8
                            as *const libc::c_char
                    ),
                    (*ss).signo
                );
                //end of define
            }
        }
        ss = ss.offset(1);
    }
    //define debug_return;
    debug_return!();
    //end of define;
}

#[no_mangle]
pub unsafe extern "C" fn init_signals() {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut ss: *mut signal_state = 0 as *mut signal_state;
    //define debug_decl(init_signals,SUDO_DEBUG_MAIN);
    debug_decl!(init_signals, SUDO_DEBUG_MAIN);
    //end of define
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0,
        ::std::mem::size_of::<sigaction>() as size_t,
    );
    sigfillset(&mut sa.sa_mask);
    sa.sa_flags = 0x10000000;
    //Some
    sa.__sigaction_handler.sa_handler =
        Some(sudo_handler as unsafe extern "C" fn(libc::c_int) -> ());
    ss = saved_signals.as_mut_ptr();
    while (*ss).signo > 0 {
        match (*ss).signo {
            13 | 18 | 21 | 22 => {}
            17 => {
                if (*ss).sa.__sigaction_handler.sa_handler
                    == ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t,
                    )
                {
                    //define sudo_debug_printf(SUDO_DEBUG_INFO,"will restore signal %d on exec",SIGCHLD);
                    sudo_debug_printf!(
                        SUDO_DEBUG_INFO,
                        b"will restore signal %d on exec\0" as *const u8 as *const libc::c_char,
                        17
                    );
                    //end of define
                    (*ss).restore = 1;
                }
                if sigaction(17, &mut sa, 0 as *mut sigaction) != 0 {
                    //define sudo_warn(U_("unable to set handler for signal %d"),SIGCHLD);
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"unable to set handler for signal %d\0" as *const u8
                                as *const libc::c_char
                        ),
                        17
                    );
                    sudo_warn!(
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"unable to set handler for signal %d\0" as *const u8
                                as *const libc::c_char
                        ),
                        17
                    );
                    //end of define
                }
            }
            _ => {
                if (*ss).sa.__sigaction_handler.sa_handler
                    != ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t,
                    )
                {
                    if sigaction((*ss).signo, &mut sa, 0 as *mut sigaction) != 0 {
                        //define sudo_warn(U_("unable to set handler for signal %d"),ss->signo);
                        sudo_debug_printf!(
                            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                            sudo_warn_gettext_v1(
                                0 as *const libc::c_char,
                                b"unable to set handler for signal %d\0" as *const u8
                                    as *const libc::c_char
                            ),
                            (*ss).signo
                        );
                        sudo_warn!(
                            sudo_warn_gettext_v1(
                                0 as *const libc::c_char,
                                b"unable to set handler for signal %d\0" as *const u8
                                    as *const libc::c_char
                            ),
                            (*ss).signo
                        );
                        //end of define
                    }
                }
            }
        } //end of match
        ss = ss.offset(1);
    } //end of while
    if saved_signals[5].sa.__sigaction_handler.sa_handler
        != ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
            1 as libc::c_int as libc::intptr_t,
        )
    {
        //define sudo_debug_printf(SUDO_DEBUG_INFO,"will restore signal %d on exec",13);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"will restore signal %d on exec\0" as *const u8 as *const libc::c_char,
            13
        );
        //end of define
        //line 173 174
        saved_signals[5].restore = 1;
        sa.__sigaction_handler.sa_handler = ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
            1 as libc::c_int as libc::intptr_t,
        );
        if sigaction(13, &mut sa, 0 as *mut sigaction) != 0 {
            //sudo_warn(U_("unable to set handler for signal %d"),SIGPIPE); //SIGPIPE = 13
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to set handler for signal %d\0" as *const u8 as *const libc::c_char
                ),
                13
            );
            sudo_warn!(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to set handler for signal %d\0" as *const u8 as *const libc::c_char
                ),
                13
            );
            //end of define
        }
    }
    //define debug_return;
    debug_return!();
    //end of define;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_sigaction(
    signo: libc::c_int,
    sa: *mut sigaction,
    osa: *mut sigaction,
) -> libc::c_int {
    let mut ss: *mut signal_state = 0 as *mut signal_state;
    let mut ret: libc::c_int = 0 as libc::c_int;
    //define debug_decl(sudo_sigaction,SUDO_DEBUG_MAIN);
    debug_decl!(sudo_sigaction, SUDO_DEBUG_MAIN);
    //end of define
    ss = saved_signals.as_mut_ptr();
    while (*ss).signo > 0 {
        if (*ss).signo == signo {
            if (*ss).sa.__sigaction_handler.sa_handler
                == ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
                    1 as libc::c_int as libc::intptr_t,
                )
                || (*sa).__sigaction_handler.sa_handler
                    == ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t,
                    )
            {
                //define sudo_debug_printf(SUDO_DEBUG_INFO,"will restore signal %d on exec",signo);
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"will restore signal %d on exec\0" as *const u8 as *const libc::c_char,
                    signo
                );
                //end of define
                (*ss).restore = 1;
            }
            break;
        }
        ss = ss.offset(1);
    }
    ret = sigaction(signo, sa, osa);
    //define debug_return_int(ret);
    debug_return_int!(ret);
    //end of define
}
