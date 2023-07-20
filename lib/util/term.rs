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
// #define	EINTR		 4	/* Interrupted system call */
pub const EINTR: libc::c_int = 4;

/* tgetpass() needs to know the erase and kill chars for cbreak mode. */
// __dso_public int sudo_term_eof;
// __dso_public int sudo_term_erase;
// __dso_public int sudo_term_kill;

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
    pub __pad0: libc::c_int,
    pub _sifields: sifields_union,
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
    pub si_overrun: libc::c_int,
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
    pub _pad: [libc::c_int; 28],
    pub _kill: kill_struct,
    pub _timer: timer_struct,
    pub _rt: rt_struct,
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
static mut changed: libc::c_int = 0 as libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    c_iflag: tcflag_t,           /* input mode flags */
    c_oflag: tcflag_t,           /* output mode flags */
    c_cflag: tcflag_t,           /* control mode flags */
    c_lflag: tcflag_t,           /* local mode flags */
    c_line: cc_t,                /* line discipline */
    c_cc: [cc_t; NCCS as usize], /* control characters */
    c_ispeed: speed_t,           /* input speed */
    c_ospeed: speed_t,           /* output speed */
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
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
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

// #define ECHO	0000010
#[macro_export]
macro_rules! ECHO {
    () => {
        0000010
    };
}

// #define ECHONL	0000100
#[macro_export]
macro_rules! ECHONL {
    () => {
        0000100
    };
}

// #define ICRNL	0000400
#[macro_export]
macro_rules! ICRNL {
    () => {
        0000400
    };
}

// #define IGNCR	0000200
#[macro_export]
macro_rules! IGNCR {
    () => {
        0000200
    };
}

// #define INLCR	0000100
#[macro_export]
macro_rules! INLCR {
    () => {
        0000100
    };
}

// #define IUCLC	0001000
#[macro_export]
macro_rules! IUCLC {
    () => {
        0001000
    };
}

// #define IXON	0002000
#[macro_export]
macro_rules! IXON {
    () => {
        0002000
    };
}

// #define OPOST	0000001
#[macro_export]
macro_rules! OPOST {
    () => {
        0000001
    };
}

// #define ICANON	0000002
#[macro_export]
macro_rules! ICANON {
    () => {
        0000002
    };
}

// #define ISIG	0000001
#[macro_export]
macro_rules! ISIG {
    () => {
        0000001
    };
}

// #define IEXTEN	0100000
#[macro_export]
macro_rules! IEXTEN {
    () => {
        0100000
    };
}

// #define IGNPAR	0000004
#[macro_export]
macro_rules! IGNPAR {
    () => {
        0000004
    };
}

// #define PARMRK	0000010
#[macro_export]
macro_rules! PARMRK {
    () => {
        0000010
    };
}
// #define INPCK	0000020
#[macro_export]
macro_rules! INPCK {
    () => {
        0000020
    };
}
// #define ISTRIP	0000040
#[macro_export]
macro_rules! ISTRIP {
    () => {
        0000040
    };
}
// #define IXANY	0004000
#[macro_export]
macro_rules! IXANY {
    () => {
        0004000
    };
}
// #define IXOFF	0010000
#[macro_export]
macro_rules! IXOFF {
    () => {
        0010000
    };
}
// #define IMAXBEL	0020000
#[macro_export]
macro_rules! IMAXBEL {
    () => {
        0020000
    };
}
// #define IUTF8	0040000
#[macro_export]
macro_rules! IUTF8 {
    () => {
        0040000
    };
}

/* c_oflag bits */
// #define OLCUC	0000002
#[macro_export]
macro_rules! OLCUC {
    () => {
        0000002
    };
}
// #define ONLCR	0000004
#[macro_export]
macro_rules! ONLCR {
    () => {
        0000004
    };
}
// #define OCRNL	0000010
#[macro_export]
macro_rules! OCRNL {
    () => {
        0000010
    };
}
// #define ONOCR	0000020
#[macro_export]
macro_rules! ONOCR {
    () => {
        0000020
    };
}
// #define ONLRET	0000040
#[macro_export]
macro_rules! ONLRET {
    () => {
        0000040
    };
}

// #define   CS7	0000040
#[macro_export]
macro_rules! CS7 {
    () => {
        0000040
    };
}
// #define   CS8	0000060
#[macro_export]
macro_rules! CS8 {
    () => {
        0000060
    };
}
// #define PARENB	0000400
#[macro_export]
macro_rules! PARENB {
    () => {
        0000400
    };
}
// #define PARODD	0001000
#[macro_export]
macro_rules! PARODD {
    () => {
        0001000
    };
}

// # define XCASE	0000004
#[macro_export]
macro_rules! XCASE {
    () => {
        0000004
    };
}

// #define ECHOE	0000020
#[macro_export]
macro_rules! ECHOE {
    () => {
        0000020
    };
}

// #define ECHOK	0000040
#[macro_export]
macro_rules! ECHOK {
    () => {
        0000040
    };
}

// #define NOFLSH	0000200
#[macro_export]
macro_rules! NOFLSH {
    () => {
        0000200
    };
}

// #define TOSTOP	0000400
#[macro_export]
macro_rules! TOSTOP {
    () => {
        0000400
    };
}

// # define ECHOCTL 0001000
#[macro_export]
macro_rules! ECHOCTL {
    () => {
        0001000
    };
}

// # define ECHOKE	 0004000
#[macro_export]
macro_rules! ECHOKE {
    () => {
        0004000
    };
}

// # define PENDIN	 0040000
#[macro_export]
macro_rules! PENDIN {
    () => {
        0040000
    };
}

// #define  B0	0000000		/* hang up */
#[macro_export]
macro_rules! B0 {
    () => {
        0000000
    };
}

// #define  B38400	0000017
#[macro_export]
macro_rules! B38400 {
    () => {
        0000017
    };
}

// #define TIOCSWINSZ	0x5414
#[macro_export]
macro_rules! TIOCSWINSZ {
    () => {
        0x5414
    };
}

/* Termios flags to copy between terminals. */
// #define INPUT_FLAGS (IGNPAR|PARMRK|INPCK|ISTRIP|INLCR|IGNCR|ICRNL|IUCLC|IXON|IXANY|IXOFF|IMAXBEL|IUTF8)
// #define OUTPUT_FLAGS (OPOST|OLCUC|ONLCR|OCRNL|ONOCR|ONLRET)
// #define CONTROL_FLAGS (CS7|CS8|PARENB|PARODD)
// #define LOCAL_FLAGS (ISIG|ICANON|XCASE|ECHO|ECHOE|ECHOK|ECHONL|NOFLSH|TOSTOP|IEXTEN|ECHOCTL|ECHOKE|PENDIN)

#[macro_export]
macro_rules! INPUT_FLAGS {
    () => {
        (IGNPAR!()
            | PARMRK!()
            | INPCK!()
            | ISTRIP!()
            | INLCR!()
            | IGNCR!()
            | ICRNL!()
            | IUCLC!()
            | IXON!()
            | IXANY!()
            | IXOFF!()
            | IMAXBEL!()
            | IUTF8!())
    };
}

#[macro_export]
macro_rules! OUTPUT_FLAGS {
    () => {
        (OPOST!() | OLCUC!() | ONLCR!() | OCRNL!() | ONOCR!() | ONLRET!())
    };
}

#[macro_export]
macro_rules! CONTROL_FLAGS {
    () => {
        (CS7!() | CS8!() | PARENB!() | PARODD!())
    };
}

#[macro_export]
macro_rules! LOCAL_FLAGS {
    () => {
        (ISIG!()
            | ICANON!()
            | XCASE!()
            | ECHO!()
            | ECHOE!()
            | ECHOK!()
            | ECHONL!()
            | NOFLSH!()
            | TOSTOP!()
            | IEXTEN!()
            | ECHOCTL!()
            | ECHOKE!()
            | PENDIN!())
    };
}

/*
 * SIGTTOU signal handler for term_restore that just sets a flag.
 */
#[no_mangle]
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

    let mut osa: sigaction = sigaction {
        __sigaction_handler: __sigaction_handler_union { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

    let mut rc: libc::c_int = 0;

    /*
     * If we receive SIGTTOU from tcsetattr() it means we are
     * not in the foreground process group.
     * This should be less racy than using tcgetpgrp().
     */
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask as *mut sigset_t);
    sa.__sigaction_handler.sa_handler = Some(sigttou as unsafe extern "C" fn(libc::c_int) -> ());
    got_sigttou = 0;
    sigaction(
        SIGTTOU,
        &mut sa as *const sigaction,
        &mut osa as *mut sigaction,
    );

    while rc != 0 && (*__errno_location()) == EINTR && got_sigttou != 0 {
        rc = tcsetattr(fd, flags, tp);
    }

    sigaction(SIGTTOU, &mut osa as *const sigaction, 0 as *mut sigaction);

    return rc;
}

/*
 * Restore saved terminal settings if we are in the foreground process group.
 * Returns true on success or false on failure.
 */
// #[named]
#[no_mangle]
unsafe extern "C" fn sudo_term_restore_v1(fd: libc::c_int, flush: bool) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if changed != 0 {
        let mut flags: libc::c_int = {
            if flush {
                TCSASOFT | TCSAFLUSH
            } else {
                TCSASOFT | TCSADRAIN
            }
        };

        if tcsetattr_nobg(fd, flags, &mut oterm) != 0 {
            debug_return_bool!(false);
        }
        changed = 0;
    };
    debug_return_bool!(true)
}

/*
 * Disable terminal echo.
 * Returns true on success or false on failure.
 */
#[no_mangle]
unsafe extern "C" fn sudo_term_noecho_v1(fd: libc::c_int) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if changed != 0 && tcgetattr(fd, &mut oterm) != 0 {
        debug_return_bool!(false);
    }
    memcpy(
        &mut term as *mut termios as *mut libc::c_void,
        &mut oterm as *mut termios as *mut libc::c_void,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    CLR!(term.c_lflag, ECHO!() | ECHONL!());
    if tcsetattr_nobg(fd, TCSASOFT | TCSADRAIN, &mut term) == 0 {
        changed = 1;
        debug_return_bool!(true);
    }
    debug_return_bool!(false)
}

/*
 * Set terminal to raw mode.
 * Returns true on success or false on failure.
 */
#[no_mangle]
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

    if changed != 0 && tcgetattr(fd, &mut oterm) != 0 {
        debug_return_bool!(false);
    }
    memcpy(
        &mut term_t as *mut termios as *mut libc::c_void,
        &mut oterm as *mut termios as *mut libc::c_void,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );

    /* Set terminal to raw mode */
    term_t.c_cc[VMIN as usize] = 1;
    term_t.c_cc[VTIME as usize] = 0;
    CLR!(
        term.c_iflag,
        ICRNL!() | IGNCR!() | INLCR!() | IUCLC!() | IXON!()
    );
    CLR!(term.c_oflag, OPOST!());
    CLR!(term.c_lflag, ECHO!() | ICANON!() | ISIG!() | IEXTEN!());
    if isig != 0 {
        SET!(term_t.c_lflag, ISIG!());
    }
    if tcsetattr_nobg(fd, TCSASOFT | TCSADRAIN, &mut term) == 0 {
        changed = 1;
        debug_return_bool!(true);
    }
    debug_return_bool!(false)
}

/*
 * Set terminal to cbreak mode.
 * Returns true on success or false on failure.
 */
#[no_mangle]
unsafe extern "C" fn sudo_term_cbreak_v1(fd: libc::c_int) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if changed != 0 && tcgetattr(fd, &mut oterm) != 0 {
        debug_return_bool!(false);
    }

    memcpy(
        &mut term as *mut termios as *mut libc::c_void,
        &mut oterm as *mut termios as *mut libc::c_void,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    /* Set terminal to half-cooked mode */
    term.c_cc[VMIN as usize] = 1;
    term.c_cc[VTIME as usize] = 0;

    /* cppcheck-suppress redundantAssignment */
    CLR!(term.c_lflag, ECHO!() | ECHONL!() | ICANON!() | IEXTEN!());
    /* cppcheck-suppress redundantAssignment */
    SET!(term.c_lflag, ISIG!());

    if tcsetattr_nobg(fd, TCSASOFT | TCSADRAIN, &mut term) == 0 {
        sudo_term_eof = term.c_cc[VEOF as usize] as libc::c_int;
        sudo_term_erase = term.c_cc[VERASE as usize] as libc::c_int;
        sudo_term_kill = term.c_cc[VKILL as usize] as libc::c_int;
        changed = 1;
        debug_return_bool!(true);
    }
    debug_return_bool!(false)
}

/*
 * Copy terminal settings from one descriptor to another.
 * We cannot simply copy the struct termios as src and dst may be
 * different terminal types (pseudo-tty vs. console or glass tty).
 * Returns true on success or false on failure.
 */
#[no_mangle]
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
