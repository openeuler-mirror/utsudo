/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unreachable_code, unused_variables, unreachable_patterns)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use crate::ISSET;
use crate::SET;
use crate::S_IRGRP;
use crate::S_IROTH;
use crate::S_IRUSR;
use crate::S_ISDIR;
use crate::S_ISREG;
use crate::S_IWUSR;
use crate::WEXITSTATUS;
use crate::WIFSIGNALED;
use crate::W_EXITCODE;
//#define	W_OK	2		/* Test for write permission.  */
pub const W_OK: libc::c_int = 2;
//# define AT_EACCESS		0x200	/* Test access permitted for effective IDs, not real IDs.  */
pub const AT_EACCESS: libc::c_int = 0x200;
//#define	ENAMETOOLONG	36	/* File name too long */
pub const ENAMETOOLONG: libc::c_int = 36;
// #define ELOOP       40  /* Too many symbolic links encountered */
pub const ELOOP: libc::c_int = 40;
// #  define AT_FDCWD	-100
pub const AT_FDCWD: libc::c_int = -100;
// # define O_CREAT	   0100	/* Not fcntl.  */
pub const O_CREAT: libc::c_int = 0o100;
pub const SESH_SUCCESS: libc::c_int = 0;
pub const SESH_ERR_FAILURE: libc::c_int = 1;
pub const SESH_ERR_KILLED: libc::c_int = 2;
pub const SESH_ERR_BAD_PATHS: libc::c_int = 31;
pub const SESH_ERR_NO_FILES: libc::c_int = 32;
pub const O_PATH: libc::c_int = 0o10000000;
pub const O_DIRECTORY: libc::c_int = 0o200000;
macro_rules! DIR_OPEN_FLAGS {
    () => {
        (O_PATH | O_DIRECTORY)
    };
}

#[macro_export]
macro_rules! sudo_timespeccmp {
    ($ts1:expr, $ts2:expr, $op:tt) => {{
    (if (*$ts1).tv_sec == (*$ts2).tv_sec {
        ((*$ts1).tv_nsec $op (*$ts2).tv_nsec) as libc::c_int
    } else {
        ((*$ts1).tv_sec $op (*$ts2).tv_sec) as libc::c_int
    })
    }};
}

extern "C" {
    fn umask(__mask: __mode_t) -> __mode_t;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int, __stat_buf: *mut stat) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn mkstemps(__template: *mut libc::c_char, __suffixlen: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn seteuid(__uid: __uid_t) -> libc::c_int;
    fn setegid(__gid: __gid_t) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn sudo_gettime_real_v1(ts: *mut timespec) -> libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_conf_sesh_path_v1() -> *const libc::c_char;
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    fn sudo_debug_fork_v1() -> pid_t;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_setgroups_v1(ngids: libc::c_int, gids: *const gid_t) -> libc::c_int;
    fn run_command(details: *mut command_details) -> libc::c_int;
    static mut user_details: user_details;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn selinux_setcon() -> libc::c_int;
    fn selinux_setup(
        role: *const libc::c_char,
        type_0: *const libc::c_char,
        ttyn: *const libc::c_char,
        ttyfd: libc::c_int,
        label_tty: bool,
    ) -> libc::c_int;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_copy_file(
        src: *const libc::c_char,
        src_fd: libc::c_int,
        src_len: off_t,
        dst: *const libc::c_char,
        dst_fd: libc::c_int,
        dst_len: off_t,
    ) -> libc::c_int;
    fn sudo_check_temp_file(
        tfd: libc::c_int,
        tname: *const libc::c_char,
        uid: uid_t,
        sb: *mut stat,
    ) -> bool;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tempfile {
    pub tfile: *mut libc::c_char,
    pub ofile: *mut libc::c_char,
    pub osize: off_t,
    pub omtim: timespec,
}














