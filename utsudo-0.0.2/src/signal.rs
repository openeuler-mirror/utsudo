/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_imports)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use crate::sudo_debug_printf2_v1;
use crate::sudo_warn_nodebug_v1;
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
