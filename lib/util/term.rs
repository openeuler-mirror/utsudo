/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;

pub type __pid_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __clock_t = libc::c_long;
pub type __uint32_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub const NCCS: libc::c_int = 32;
pub type speed_t = libc::c_uint;

pub static mut sudo_term_eof: libc::c_int = 0;
pub static mut sudo_term_erase: libc::c_int = 0;
pub static mut sudo_term_kill: libc::c_int = 0;

pub union sigval {
}

pub type __sigval_t = sigval;

pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
}

pub struct kill_struct {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}

pub struct timer_struct {
    pub si_tid: libc::c_int,
    pub si_sigval: __sigval_t,
}

pub struct rt_struct {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}

pub struct sigchld_struct {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}

pub struct sigfault_struct {
    pub si_addr_lsb: libc::c_short,
    pub _bounds: bounds_struct,
}

pub struct sigpoll_struct {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}

pub struct sigsys_struct {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}

pub union sifields_union {
}

pub struct bounds_struct {
    pub _addr_bnd: addr_bnd_struct,
    pub _key: __uint32_t,
}

pub struct addr_bnd_struct {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}

pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}

pub union __sigaction_handler_union {
}

pub struct sigaction {
    pub __sigaction_handler: __sigaction_handler_union,
    pub sa_mask: sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}

pub struct termios {
    c_iflag: tcflag_t, 
    c_oflag: tcflag_t, 
    c_cflag: tcflag_t, 
    c_lflag: tcflag_t, 
    c_line: cc_t, 
    c_cc: [cc_t; NCCS as usize], 
    c_ispeed: speed_t,  
    c_ospeed: speed_t, 
}

pub struct winsize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
}


unsafe extern "C" fn sigttou()

unsafe extern "C" fn tcsetattr_nobg()

unsafe extern "C" fn sudo_term_restore_v1()

unsafe extern "C" fn sudo_term_noecho_v1()

unsafe extern "C" fn sudo_term_raw_v1()

unsafe extern "C" fn sudo_term_cbreak_v1()
    
unsafe extern "C" fn sudo_term_copy_v1()
