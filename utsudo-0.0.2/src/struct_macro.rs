/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(non_camel_case_types, non_snake_case, unused_imports)]

// #define SUDO_CONV_REPL_MAX	255
pub const SUDO_CONV_REPL_MAX: libc::c_int = 255;
// #define O_WRONLY	     01
pub const O_WRONLY: libc::c_int = 0o1;
// #define TGP_ECHO	0x01		/* leave echo on when reading passwd */
pub const TGP_ECHO: libc::c_int = 0x01;
// #define TGP_MASK	0x08		/* mask user input when reading */
pub const TGP_MASK: libc::c_int = 0x08;
// #define TGP_NOECHO_TRY	0x10		/* turn off echo if possible */
pub const TGP_NOECHO_TRY: libc::c_int = 0x10;

/* Standard file descriptors.  */
//#define	STDIN_FILENO	0	/* Standard input.  */
//#define	STDOUT_FILENO	1	/* Standard output.  */
//#define	STDERR_FILENO	2	/* Standard error output.  */
pub const STDIN_FILENO: libc::c_int = 0;
pub const STDOUT_FILENO: libc::c_int = 1;
pub const STDERR_FILENO: libc::c_int = 2;

/* Indices into io_fds[] when running a command in a pty. */
// #define SFD_STDIN	0
// #define SFD_STDOUT	1
// #define SFD_STDERR	2
// #define SFD_MASTER	3
// #define SFD_SLAVE	4
// #define SFD_USERTTY	5
pub const SFD_STDIN: libc::c_int = 0;
pub const SFD_STDOUT: libc::c_int = 1;
pub const SFD_STDERR: libc::c_int = 2;
pub const SFD_MASTER: libc::c_int = 3;
pub const SFD_SLAVE: libc::c_int = 4;
pub const SFD_USERTTY: libc::c_int = 5;

//#define	__S_ISUID	04000	/* Set user ID on execution.  */
pub const S_ISUID: libc::c_int = 0o4000;

pub const SI_USER: libc::c_int = 0;
pub const SIG2STR_MAX: libc::c_int = 32;
pub const SIGCONT_FG: libc::c_int = -2;
pub const SIGCONT_BG: libc::c_int = -3;

/* Adjustments and additions to the signal number constants for
most Linux systems.  */
// #define	SIGUSR1		10
// #define	SIGUSR2		12
// #define	SIGCHLD		17
// #define	SIGCONT		18
// #define	SIGSTOP		19

pub const SIGUSR1: libc::c_int = 10;
pub const SIGUSR2: libc::c_int = 12;
pub const SIGCHLD: libc::c_int = 17;
pub const SIGCONT: libc::c_int = 18;
pub const SIGSTOP: libc::c_int = 19;

pub const __SIGRTMAX: libc::c_int = 64;
pub const NSIG: libc::c_int = __SIGRTMAX + 1;

/* Nonstandard signals found in all modern POSIX systems
(including both BSD and Linux).  */
pub const SIGWINCH: libc::c_int = 28; /* Window size change (4.3 BSD, Sun).  */

/* 0x54 is just a magic number to make these relatively unique ('T') */
pub const TIOCGWINSZ: libc::c_int = 0x5413;
pub const TIOCSWINSZ: libc::c_int = 0x5414;

/* Status passed between parent and child via socketpair */
// #define CMD_INVALID	0
// #define CMD_ERRNO	1
// #define CMD_WSTATUS	2
// #define CMD_SIGNO	3
// #define CMD_PID		4
// #define CMD_TTYWINCH	5
pub const CMD_INVALID: libc::c_int = 0;
pub const CMD_ERRNO: libc::c_int = 1;
pub const CMD_WSTATUS: libc::c_int = 2;
pub const CMD_SIGNO: libc::c_int = 3;
pub const CMD_PID: libc::c_int = 4;
pub const CMD_TTYWINCH: libc::c_int = 5;

// /* Bits in the third argument to `waitpid'.  */
// #define	WNOHANG		1	/* Don't block waiting.  */
// #define	WUNTRACED	2	/* Report status of stopped children.  */
pub const WNOHANG: libc::c_int = 1;
pub const WUNTRACED: libc::c_int = 2;

//#define	EINTR		 4	/* Interrupted system call */
//#define	EIO		 5	/* I/O error */
//#define	ENXIO		 6	/* No such device or address */
//#define	EBADF		 9	/* Bad file number */
//#define	ECHILD		10	/* No child processes */
//#define	EAGAIN		11	/* Try again */
//#define	EISDIR		21	/* Is a directory */
//#define	EINVAL		22	/* Invalid argument */
//#define	EPIPE		32	/* Broken pipe */
pub const EINTR: libc::c_int = 4;
pub const EIO: libc::c_int = 5;
pub const ENXIO: libc::c_int = 6;
pub const EBADF: libc::c_int = 9;
pub const ECHILD: libc::c_int = 10;
pub const EAGAIN: libc::c_int = 11;
pub const EISDIR: libc::c_int = 21;
pub const EINVAL: libc::c_int = 22;
pub const EPIPE: libc::c_int = 32;

/*
 * This error code is special: arch syscall entry code will return
 * -ENOSYS if users try to call a syscall that doesn't exist.  To keep
 * failures of syscalls that really do exist distinguishable from
 * failures due to attempts to use a nonexistent syscall, syscall
 * implementations should refrain from returning -ENOSYS.
 */
//#define	ECONNRESET	104	/* Connection reset by peer */
pub const ECONNRESET: libc::c_int = 104;

// #define CD_SET_UID		0x000001
// #define CD_SET_EUID		0x000002
// #define CD_SET_GID		0x000004
// #define CD_SET_EGID		0x000008
// #define CD_PRESERVE_GROUPS	0x000010
// #define CD_NOEXEC		0x000020
// #define CD_SET_PRIORITY		0x000040
// #define CD_SET_UMASK		0x000080
// #define CD_SET_TIMEOUT		0x000100
// #define CD_SUDOEDIT		0x000200
// #define CD_BACKGROUND		0x000400
// #define CD_RBAC_ENABLED		0x000800
// #define CD_USE_PTY		0x001000
// #define CD_SET_UTMP		0x002000
// #define CD_EXEC_BG		0x004000
// #define CD_SUDOEDIT_FOLLOW	0x008000
// #define CD_SUDOEDIT_CHECKDIR	0x010000
// #define CD_SET_GROUPS		0x020000
// #define CD_LOGIN_SHELL		0x040000
// #define CD_OVERRIDE_UMASK	0x080000
pub const CD_SET_UID: libc::c_int = 0x000001;
pub const CD_SET_EUID: libc::c_int = 0x000002;
pub const CD_SET_GID: libc::c_int = 0x000004;
pub const CD_SET_EGID: libc::c_int = 0x000008;
pub const CD_PRESERVE_GROUPS: libc::c_int = 0x000010;
pub const CD_NOEXEC: libc::c_int = 0x000020;
pub const CD_SET_PRIORITY: libc::c_int = 0x000040;
pub const CD_SET_UMASK: libc::c_int = 0x000080;
pub const CD_SET_TIMEOUT: libc::c_int = 0x000100;
pub const CD_SUDOEDIT: libc::c_int = 0x000200;
pub const CD_BACKGROUND: libc::c_int = 0x000400;
pub const CD_RBAC_ENABLED: libc::c_int = 0x000800;
pub const CD_USE_PTY: libc::c_int = 0x001000;
pub const CD_SET_UTMP: libc::c_int = 0x002000;
pub const CD_EXEC_BG: libc::c_int = 0x004000;
pub const CD_SUDOEDIT_FOLLOW: libc::c_int = 0x008000;
pub const CD_SUDOEDIT_CHECKDIR: libc::c_int = 0x010000;
pub const CD_SET_GROUPS: libc::c_int = 0x020000;
pub const CD_LOGIN_SHELL: libc::c_int = 0x040000;
pub const CD_OVERRIDE_UMASK: libc::c_int = 0x080000;

/* Event types */
// #define SUDO_EV_TIMEOUT		0x01	/* fire after timeout */
// #define SUDO_EV_READ		0x02	/* fire when readable */
// #define SUDO_EV_WRITE		0x04	/* fire when writable */
// #define SUDO_EV_PERSIST		0x08	/* persist until deleted */
// #define SUDO_EV_SIGNAL		0x10	/* fire on signal receipt */
// #define SUDO_EV_SIGINFO		0x20	/* fire on signal receipt (siginfo) */
pub const SUDO_EV_TIMEOUT: libc::c_int = 0x01;
pub const SUDO_EV_READ: libc::c_int = 0x02;
pub const SUDO_EV_WRITE: libc::c_int = 0x04;
pub const SUDO_EV_PERSIST: libc::c_int = 0x08;
pub const SUDO_EV_SIGNAL: libc::c_int = 0x10;
pub const SUDO_EV_SIGINFO: libc::c_int = 0x20;

/* ISO C99 signals.  */
// #define	SIGINT		2	/* Interactive attention signal.  */
pub const SIGINT: libc::c_int = 2;
pub const SIGTERM: libc::c_int = 15;

/* Historical signals specified by POSIX. */
// #define	SIGHUP		1	/* Hangup.  */
// #define	SIGQUIT		3	/* Quit.  */
// #define	SIGTRAP		5	/* Trace/breakpoint trap.  */
// #define	SIGKILL		9	/* Killed.  */
// #define SIGBUS		10	/* Bus error.  */
// #define	SIGSYS		12	/* Bad system call.  */
// #define	SIGPIPE		13	/* Broken pipe.  */
// #define	SIGALRM		14	/* Alarm clock.  */
pub const SIGHUP: libc::c_int = 1;
pub const SIGQUIT: libc::c_int = 3;
pub const SIGTRAP: libc::c_int = 5;
pub const SIGKILL: libc::c_int = 9;
pub const SIGBUS: libc::c_int = 10;
pub const SIGSYS: libc::c_int = 12;
pub const SIGPIPE: libc::c_int = 13;
pub const SIGALRM: libc::c_int = 14;

/* New(er) POSIX signals (1003.1-2008, 1003.1-2013).  */
//#define	SIGTSTP		18	/* Keyboard stop.  */
// #define	SIGTTIN		21	/* Background read from control terminal.  */
// #define	SIGTTOU		22	/* Background write to control terminal.  */
pub const SIGTSTP: libc::c_int = 18;
pub const SIGTTIN: libc::c_int = 21;
pub const SIGTTOU: libc::c_int = 22;

//#define SA_NOCLDSTOP	0x00000001u
pub const SA_RESTART: libc::c_int = 0x10000000;

//# define O_CLOEXEC	0x80000000
pub const O_CLOEXEC: libc::c_int = 0o2000000;

// MSG_WAITALL		= 0x100, /* Wait for a full request.  */
// #define MSG_WAITALL	MSG_WAITALL
pub const MSG_WAITALL: libc::c_int = 0x100;

//#define	SIG_SETMASK   2		 /* Set the set of blocked signals.  */
pub const SIG_SETMASK: libc::c_int = 2;

//#define	ENOENT		 2	/* No such file or directory */
//#define	EACCES		13	/* Permission denied */
pub const ENOENT: libc::c_int = 2;
pub const EACCES: libc::c_int = 13;

//#define O_RDWR  02
pub const O_RDWR: libc::c_int = 0o2;

// /* Values for the HOW argument to `sigprocmask'.  */
// #define	SIG_BLOCK     0		 /* Block signals.  */
// #define	SIG_UNBLOCK   1		 /* Unblock signals.  */
// #define	SIG_SETMASK   2		 /* Set the set of blocked signals.  */
pub const SIG_BLOCK: libc::c_int = 0;
pub const SIG_UNBLOCK: libc::c_int = 1;
pub const SIG_SETMAS: libc::c_int = 2;

/* Values for the second argument to `fcntl'.  */
// #define F_DUPFD		0	/* Duplicate file descriptor.  */
// #define F_GETFD		1	/* Get file descriptor flags.  */
// #define F_SETFD		2	/* Set file descriptor flags.  */
// #define F_GETFL		3	/* Get file status flags.  */
// #define F_SETFL		4	/* Set file status flags.  */
pub const F_DUPFD: libc::c_int = 0;
pub const F_GETFD: libc::c_int = 1;
pub const F_SETFD: libc::c_int = 2;
pub const F_GETFL: libc::c_int = 3;
pub const F_SETFL: libc::c_int = 4;

// #define	LC_ALL		  __LC_ALL
// #define __LC_ALL		 6
pub const LC_ALL: libc::c_int = 6;

// # define ROOT_UID	0
pub const ROOT_UID: libc::c_int = 0;

//#define O_NOFOLLOW	00400000	/* don't follow links */
pub const O_NOFOLLOW: libc::c_int = 0o0400000;

//#define O_RDONLY	     00
pub const O_RDONLY: libc::c_int = 0o0;

//# define O_NONBLOCK	  04000
pub const O_NONBLOCK: libc::c_int = 0o4000;

pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type sig_atomic_t = __sig_atomic_t;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type __clock_t = libc::c_long;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type uid_t = __uid_t;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type __blkcnt_t = libc::c_long;
#[cfg(target_arch = "x86_64")]
pub type __blksize_t = libc::c_long;
#[cfg(not(target_arch = "x86_64"))]
pub type __blksize_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type __ino_t = libc::c_ulong;
#[cfg(target_arch = "x86_64")]
pub type __nlink_t = libc::c_ulong;
#[cfg(not(target_arch = "x86_64"))]
pub type __nlink_t = libc::c_uint;
pub type __id_t = libc::c_uint;
pub type id_t = __id_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __spawn_action {
    pub _address: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_message {
    pub msg_type: libc::c_int,
    pub timeout: libc::c_int,
    pub msg: *const libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_reply {
    pub reply: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_callback {
    pub version: libc::c_uint,
    pub closure: *mut libc::c_void,
    pub on_suspend: sudo_conv_callback_fn_t,
    pub on_resume: sudo_conv_callback_fn_t,
}
pub type sudo_conv_callback_fn_t =
    Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_status {
    pub type_0: libc::c_int,
    pub val: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: siginfo_t_u,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union siginfo_t_u {
    pub _pad: [libc::c_int; 28],
    pub _kill: _kill_s,
    pub _timer: _timer_s,
    pub _rt: _rt_s,
    pub _sigchld: _sigchld_s,
    pub _sigfault: _sigfault_s,
    pub _sigpoll: _sigpoll_s,
    pub _sigsys: _sigsys_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _kill_s {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _timer_s {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rt_s {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sigchld_s {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sigfault_s {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: _bounds_u,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _bounds_u {
    pub _addr_bnd: _addr_bnd_s,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _addr_bnd_s {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sigpoll_s {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sigsys_s {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event_base {
    pub events: sudo_event_list,
    pub active: sudo_event_list,
    pub timeouts: sudo_event_list,
    pub signal_event: sudo_event,
    pub signals: [sudo_event_list; NSIG as usize],
    pub orig_handlers: [*mut sigaction; NSIG as usize],
    pub siginfo: [*mut siginfo_t; NSIG as usize],
    pub signal_pending: [sig_atomic_t; NSIG as usize],
    pub signal_caught: sig_atomic_t,
    pub num_handlers: libc::c_int,
    pub signal_pipe: [libc::c_int; 2],
    pub pfds: *mut pollfd,
    pub pfd_max: libc::c_int,
    pub pfd_high: libc::c_int,
    pub pfd_free: libc::c_int,
    pub flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event_list {
    pub tqh_first: *mut sudo_event,
    pub tqh_last: *mut *mut sudo_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event {
    pub entries: TAILQ_ENTRY_sudo_event,
    pub active_entries: TAILQ_ENTRY_active_entries,
    pub timeouts_entries: TAILQ_ENTRY_timeouts_entries,
    pub base: *mut sudo_event_base,
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
    pub flags: libc::c_short,
    pub pfd_idx: libc::c_short,
    pub callback: sudo_ev_callback_t,
    pub timeout: timespec,
    pub closure: *mut libc::c_void,
}
pub type sudo_ev_callback_t =
    Option<unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_sudo_event {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_active_entries {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_timeouts_entries {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: Signal_handler,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Signal_handler {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ()>,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type sudo_fatal_callback_t = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_ev_siginfo_container {
    pub closure: *mut libc::c_void,
    pub siginfo: *mut siginfo_t,
    pub si_buf: [libc::c_char; 1],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_details {
    pub uid: uid_t,
    pub euid: uid_t,
    pub gid: gid_t,
    pub egid: gid_t,
    pub umask: mode_t,
    pub priority: libc::c_int,
    pub timeout: libc::c_int,
    pub ngroups: libc::c_int,
    pub closefrom: libc::c_int,
    pub flags: libc::c_int,
    pub execfd: libc::c_int,
    pub preserved_fds: preserved_fd_list,
    pub pw: *mut passwd,
    pub groups: *mut gid_t,
    pub command: *const libc::c_char,
    pub cwd: *const libc::c_char,
    pub login_class: *const libc::c_char,
    pub chroot: *const libc::c_char,
    pub selinux_role: *const libc::c_char,
    pub selinux_type: *const libc::c_char,
    pub utmp_user: *const libc::c_char,
    pub tty: *const libc::c_char,
    pub argv: *mut *mut libc::c_char,
    pub envp: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct preserved_fd_list {
    pub tqh_first: *mut preserved_fd,
    pub tqh_last: *mut *mut preserved_fd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct preserved_fd {
    pub entries: TAILQ_ENTRY_preserved_fd,
    pub lowfd: libc::c_int,
    pub highfd: libc::c_int,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_preserved_fd {
    pub tqe_next: *mut preserved_fd,
    pub tqe_prev: *mut *mut preserved_fd,
}

// #define	_PATH_TTY	"/dev/tty"
#[macro_export]
macro_rules! _PATH_TTY {
    () => {
        b"/dev/tty\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! SET {
    ($t:expr,$f:expr) => {
        $t |= $f
    };
}

#[macro_export]
macro_rules! ISSET {
    ($t:expr,$f:expr) => {
        $t & $f
    };
}

// #define	_PATH_BSHELL	"/bin/sh"
// #  define _PATH_SUDO_BSHELL _PATH_BSHELL
#[macro_export]
macro_rules! _PATH_SUDO_BSHELL {
    () => {
        b"/bin/sh\0" as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! USER_SIGNALED {
    ($_info:expr) => {
        (!($_info).is_null() && (*($_info)).si_code == crate::struct_macro::SI_USER)
    };
}

#[macro_export]
macro_rules! WIFSTOPPED {
    ($status:expr) => {
        ($status & 0xff == 0x7f)
    };
}
#[macro_export]
macro_rules! WIFSIGNALED {
    ($status:expr) => {
        (((($status & 0x7f) + 1) >> 1) > 0)
    };
}

#[macro_export]
macro_rules! WEXITSTATUS {
    ($status:expr) => {
        ((($status) & 0xff00) >> 8)
    };
}

#[macro_export]
macro_rules! S_IRUSR {
    () => {
        0o400
    };
}

// #define  __S_IWRITE  0200    /* Write by owner.  */
// #define  S_IWUSR __S_IWRITE  /* Write by owner.  */
#[macro_export]
macro_rules! S_IWUSR {
    () => {
        0o200
    };
}

// # define S_IRGRP    (S_IRUSR >> 3)  /* Read by group.  */
#[macro_export]
macro_rules! S_IRGRP {
    () => {
        (S_IRUSR!() >> 3)
    };
}

// # define S_IROTH    (S_IRGRP >> 3)  /* Read by others.  */
#[macro_export]
macro_rules! S_IROTH {
    () => {
        (S_IRGRP!() >> 3)
    };
}

// # define W_EXITCODE(ret, sig)	__W_EXITCODE (ret, sig)
/* Macros for constructing status values.  */
// #define	__W_EXITCODE(ret, sig)	((ret) << 8 | (sig))
#[macro_export]
macro_rules! W_EXITCODE {
    ($ret:expr, $sig:expr) => {{
        (($ret) << 8 | ($sig))
    }};
}

// #define S_IFMT  00170000
#[macro_export]
macro_rules! S_IFMT {
    () => {
        0o0170000
    };
}

// #define S_IFDIR  0040000
#[macro_export]
macro_rules! S_IFDIR {
    () => {
        0o040000
    };
}

// #define __S_ISTYPE(mode, mask)  (((mode) & __S_IFMT) == (mask))
#[macro_export]
macro_rules! __S_ISTYPE {
    ($mode:expr, $mask:expr) => {
        ((($mode) & crate::S_IFMT!()) == ($mask))
    };
}

// #define S_ISDIR(mode)    __S_ISTYPE((mode), __S_IFDIR)
//# define S_ISDIR(m)		(((m) & _S_IFMT) == _S_IFDIR)
#[macro_export]
macro_rules! S_ISDIR {
    ($m:expr) => {
        ((($m) & crate::__S_IFMT!()) == crate::S_IFDIR!())
    };
}

#[macro_export]
macro_rules! INT_MAX {
    () => {
        2147483647
    };
}

#[macro_export]
macro_rules! INT_MIN {
    () => {
        (-(INT_MAX!()) - 1)
    };
}

#[macro_export]
macro_rules! WSTOPSIG {
    ($status:expr) => {
        ((($status) & 0xff00) >> 8)
    };
}

#[macro_export]
macro_rules! WTERMSIG {
    ($status:expr) => {
        ($status & 0x7f)
    };
}
#[macro_export]
macro_rules! WIFEXITED {
    ($status: expr) => {
        (WTERMSIG!($status) == 0)
    };
}

// #define	__S_IFMT	0170000	/* These bits determine file type.  */
#[macro_export]
macro_rules! __S_IFMT {
    () => {{
        0o170000
    }};
}

// #define	__S_IFREG	0100000	/* Regular file.  */
#[macro_export]
macro_rules! __S_IFREG {
    () => {{
        0o100000
    }};
}

// #define	S_ISREG(mode)	 __S_ISTYPE((mode), __S_IFREG)
// #define	__S_ISTYPE(mode, mask)	(((mode) & __S_IFMT) == (mask))
#[macro_export]
macro_rules! S_ISREG {
    ($mode:expr) => {{
        ((($mode) & crate::__S_IFMT!()) == (crate::__S_IFREG!()))
    }};
}

#[macro_export]
macro_rules! errno {
    () => {
        (*__errno_location())
    };
}

#[macro_export]
macro_rules! SIG_IGN {
    () => {
        std::mem::transmute::<libc::intptr_t, __sighandler_t>(1 as libc::intptr_t)
    };
}

#[macro_export]
macro_rules! CLR {
    ($t: expr, $f: expr) => {
        (($t) &= !($f))
    };
}

#[macro_export]
macro_rules! SUDO_API_MKVERSION {
    ($x: expr, $y: expr) => {
        ((($x) << 16) | ($y))
    };
}
