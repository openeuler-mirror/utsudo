/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;

pub union sigval {
}

pub type __sigval_t = sigval;

pub struct siginfo_t {
}

pub struct kill_struct {
}

pub struct timer_struct {
}

pub struct rt_struct {
}

pub struct sigchld_struct {
}

pub struct sigfault_struct {
}

pub struct sigpoll_struct {
}

pub struct sigsys_struct {
}

pub union sifields_union {
}

pub struct bounds_struct {
}

pub struct addr_bnd_struct {
}

pub struct __sigset_t {
}

pub union __sigaction_handler_union {
}

pub struct sigaction {
}

pub struct termios {
}

pub struct winsize {
}


unsafe extern "C" fn sigttou()

unsafe extern "C" fn tcsetattr_nobg()

unsafe extern "C" fn sudo_term_restore_v1()

unsafe extern "C" fn sudo_term_noecho_v1()

unsafe extern "C" fn sudo_term_raw_v1()

unsafe extern "C" fn sudo_term_cbreak_v1()
    
unsafe extern "C" fn sudo_term_copy_v1()
