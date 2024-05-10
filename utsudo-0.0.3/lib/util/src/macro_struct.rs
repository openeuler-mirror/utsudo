/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//#define	EINVAL		22	/* Invalid argument */
pub const EINVAL: libc::c_int = 22;

//#define	SHA512_BLOCK_LENGTH		128
pub const SHA512_BLOCK_LENGTH: usize = 128;

pub const __SIGRTMIN: libc::c_uint = 64;
pub const NSIG: libc::c_uint = __SIGRTMIN + 1;

pub const _SC_NGROUPS_MAX: libc::c_uint = 3;

//#define NGROUPS_MAX    65536	/* supplemental group IDs are available */
pub const NGROUPS_MAX: libc::c_uint = 65536;

// #define SUDO_PATH_SECURE		0
// #define SUDO_PATH_MISSING		-1
// #define SUDO_PATH_BAD_TYPE		-2
// #define SUDO_PATH_WRONG_OWNER		-3
// #define SUDO_PATH_WORLD_WRITABLE	-4
// #define SUDO_PATH_GROUP_WRITABLE	-5
pub const SUDO_PATH_SECURE: libc::c_int = 0;
pub const SUDO_PATH_MISSING: libc::c_int = -1;
pub const SUDO_PATH_BAD_TYPE: libc::c_int = -2;
pub const SUDO_PATH_WRONG_OWNER: libc::c_int = -3;
pub const SUDO_PATH_WORLD_WRITABLE: libc::c_int = -4;
pub const SUDO_PATH_GROUP_WRITABLE: libc::c_int = -5;

// #define	ENOENT		 2	/* No such file or directory */
pub const ENOENT: libc::c_int = 2;

pub const _SC_RTSIG_MAX: libc::c_int = 31;
pub const _ISdigit: libc::c_uint = 2048;

//#define	ERANGE		34	/* Math result not representable */
pub const ERANGE: libc::c_int = 34;

pub type pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __ino_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint64_t;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
#[cfg(target_arch = "x86_64")]
pub type __nlink_t = libc::c_ulong;
#[cfg(not(target_arch = "x86_64"))]
pub type __nlink_t = libc::c_uint;
pub type __off64_t = libc::c_long;
#[cfg(target_arch = "x86_64")]
pub type __blksize_t = libc::c_long;
#[cfg(not(target_arch = "x86_64"))]
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type sig_atomic_t = __sig_atomic_t;
pub type time_t = __time_t;
pub type __id_t = libc::c_uint;
pub type id_t = __id_t;
pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
pub type off_t = __off_t;
pub type __int32_t = libc::c_int;
pub type FILE = _IO_FILE;

//宏
// #define __INT_MAX__ 0x7fffffff
// #define INT_MAX __INT_MAX__
#[macro_export]
macro_rules! INT_MAX {
    () => {
        0x7fffffff
    };
}

// #define	__S_IFMT	0170000	/* These bits determine file type.  */
#[macro_export]
macro_rules! _S_IFMT {
    () => {
        0o170000
    };
}

// #define	__S_IWRITE	0200	/* Write by owner.  */
// #define	S_IWUSR	__S_IWRITE	/* Write by owner.  */
#[macro_export]
macro_rules! S_IWUSR {
    () => {
        0o200
    };
}
// #define	S_IWGRP	(S_IWUSR >> 3)	/* Write by group.  */
#[macro_export]
macro_rules! S_IWGRP {
    () => {
        S_IWUSR!() >> 3
    };
}

// #define	S_IWOTH	(S_IWGRP >> 3)	/* Write by others.  */
#[macro_export]
macro_rules! S_IWOTH {
    () => {
        S_IWGRP!() >> 3
    };
}

// #define ISSET(t, f)     ((t) & (f))
#[macro_export]
macro_rules! ISSET {
    ($_t:expr, $_f:expr) => {
        (($_t) & ($_f))
    };
}

/* Extract subsystem number and convert to an index. */
// #define SUDO_DEBUG_SUBSYS(n) (((n) >> 6) - 1)
#[macro_export]
macro_rules! SUDO_DEBUG_SUBSYS {
    ($_n:expr) => {
        ((($_n) >> 6) - 1)
    };
}

/* Extract priority number and convert to an index. */
// #define SUDO_DEBUG_PRI(n) (((n) & 0x0f) - 1)
#[macro_export]
macro_rules! SUDO_DEBUG_PRI {
    ($_n:expr) => {
        ((($_n) & 0x0f) - 1)
    };
}

// #define TIOCGWINSZ	0x5413
#[macro_export]
macro_rules! TIOCGWINSZ {
    () => {
        0x5413
    };
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA2_CTX {
    pub state: state,
    pub count: [uint64_t; 2],
    pub buffer: [uint8_t; SHA512_BLOCK_LENGTH],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union state {
    pub st32: [uint32_t; 8],
    pub st64: [uint64_t; 8],
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
pub struct sigaction {
    pub __sigaction_handler: Signal_handler,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Signal_handler {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ()>,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    #[cfg(target_arch = "x86_64")]
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    #[cfg(not(target_arch = "x86_64"))]
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    #[cfg(target_arch = "x86_64")]
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    #[cfg(not(target_arch = "x86_64"))]
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    #[cfg(not(target_arch = "x86_64"))]
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    #[cfg(target_arch = "x86_64")]
    pub __glibc_reserved: [__syscall_slong_t; 3],
    #[cfg(not(target_arch = "x86_64"))]
    pub __glibc_reserved: [libc::c_int; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_file {
    pub entries: sudo_debug_file_list,
    pub debug_file: *mut libc::c_char,
    pub debug_flags: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_file_list {
    pub tqe_next: *mut sudo_debug_file,
    pub tqe_prev: *mut *mut sudo_debug_file,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_debug_file_list {
    pub tqh_first: *mut sudo_debug_file,
    pub tqh_last: *mut *mut sudo_debug_file,
}

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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
