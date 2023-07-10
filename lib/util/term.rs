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
    clashing_extern_declarations
)]

//ttysize.rs文件中定义
use crate::TIOCGWINSZ;

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

/* Type of a signal handler.  */
// typedef void (*__sighandler_t) (int);
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type __pid_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __clock_t = libc::c_long;
pub type __uint32_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub const NCCS: libc::c_int = 32;
pub type speed_t = libc::c_uint;

pub struct __dso_public {
    pub sudo_term_eof = libc::c_int,
    pub sudo_term_erase = libc::c_int,
    pub sudo_term_kill = libc::c_int,
}

#[no_mangle]
pub static mut sudo_term_eof: libc::c_int = 0;
#[no_mangle]
pub static mut sudo_term_erase: libc::c_int = 0;
#[no_mangle]
pub static mut sudo_term_kill: libc::c_int = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kill_struct {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timer_struct {
    pub si_tid: libc::c_int,
    pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rt_struct {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigchld_struct {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigfault_struct {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: bounds_struct,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigpoll_struct {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigsys_struct {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union sifields_union {
    pub _kill: kill_struct,
    pub _timer: timer_struct,
    pub _sigchld: sigchld_struct,
    pub _sigfault: sigfault_struct,
    pub _sigpoll: sigpoll_struct,
    pub _sigsys: sigsys_struct,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bounds_struct {
    pub _addr_bnd: addr_bnd_struct,
    pub _key: __uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr_bnd_struct {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union __sigaction_handler_union {
    /* Used if SA_SIGINFO is not set.  */
    sa_handler: __sighandler_t,

    /* Used if SA_SIGINFO is set.  */
    sa_sigaction:
        Option<unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: __sigaction_handler_union,

    /* Additional set of signals to be blocked.  */
    pub sa_mask: sigset_t,

    /* Special flags.  */
    pub sa_flags: libc::c_int,

    /* Restore handler.  */
    // void (*sa_restorer) (void);
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}

/* An integral type that can be modified atomically, without the
possibility of a signal arriving in the middle of the operation.  */
//    typedef __sig_atomic_t sig_atomic_t;

/* C99: An integer type that can be accessed as an atomic entity,
even in the presence of asynchronous interrupts.
It is not currently necessary for this to be machine-specific.  */
// typedef int __sig_atomic_t;

pub type sig_atomic_t = libc::c_int;
static mut got_sigttou: sig_atomic_t = 0 as sig_atomic_t;

static mut term: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; NCCS as usize],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut oterm: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; NCCS as usize],
    c_ispeed: 0,
    c_ospeed: 0,
};

#[derive(Copy, Clone)]
#[repr(C)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
}

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    fn cfgetispeed(__termios_p: *const termios) -> speed_t;
}

// #define	SIGTTOU		22	/* Background write to control terminal.  */
pub const SIGTTOU: libc::c_int = 22;

// # define TCSASOFT	0
pub const TCSASOFT: libc::c_int = 0 as libc::c_int;

// #define	TCSADRAIN	1
pub const TCSADRAIN: libc::c_int = 1 as libc::c_int;

// #define	TCSAFLUSH	2
pub const TCSAFLUSH: libc::c_int = 2 as libc::c_int;

// #define VERASE 2
pub const VERASE: libc::c_int = 2 as libc::c_int;

// #define VKILL 3
pub const VKILL: libc::c_int = 3 as libc::c_int;

// #define VEOF 4
pub const VEOF: libc::c_int = 4 as libc::c_int;

// #define VTIME 5
pub const VTIME: libc::c_int = 5 as libc::c_int;

// #define VMIN 6
pub const VMIN: libc::c_int = 6 as libc::c_int;

// #define CLR(t, f)	((t) &= ~(f))
#[macro_export]
macro_rules! CLR {
    ($t:expr, $f:expr) => {
        (($t) &= !($f))
    };
}

// #define SET(t, f)	((t) |= (f))
#[macro_export]
macro_rules! SET {
    ($t:expr, $f:expr) => {
        (($t) |= ($f))
    };
}

// #define TIOCSWINSZ	0x5414
#[macro_export]
macro_rules! TIOCSWINSZ {
    () => {
        0x5414
    };
}

unsafe extern "C" fn sigttou(_signo: libc::c_int) {
    got_sigttou = 1;
}

/*
 * Like tcsetattr() but restarts on EINTR _except_ for SIGTTOU.
 * Returns 0 on success or -1 on failure, setting errno.
 * Sets got_sigttou on failure if interrupted by SIGTTOU.
 */
unsafe extern "C" fn tcsetattr_nobg(
    fd: libc::c_int,
    flags: libc::c_int,
    tp: *mut termios,
) -> libc::c_int {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: __sigaction_handler_union { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

unsafe extern "C" fn sudo_term_restore_v1(fd: libc::c_int, flush: bool) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
}

unsafe extern "C" fn sudo_term_noecho_v1(fd: libc::c_int) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
}

unsafe extern "C" fn sudo_term_raw_v1(fd: libc::c_int, isig: libc::c_int) -> bool {
    let mut term_t: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; NCCS as usize],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
}

unsafe extern "C" fn sudo_term_cbreak_v1(fd: libc::c_int) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
}

unsafe extern "C" fn sudo_term_copy_v1(src: libc::c_int, dst: libc::c_int) -> bool {
    let mut tt_src: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; NCCS as usize],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut tt_dst: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; NCCS as usize],
        c_ispeed: 0,
        c_ospeed: 0,
    };
}
