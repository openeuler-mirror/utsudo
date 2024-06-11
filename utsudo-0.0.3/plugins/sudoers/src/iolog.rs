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
    unused_variables,
    unused_unsafe
)]
use crate::common::*;

extern "C" {
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int, __stat_buf: *mut stat) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __offset: __off_t,
    ) -> ssize_t;
    fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn sudo_gettime_awake_v1(ts: *mut timespec) -> libc::c_int;
    fn sudo_lock_file_v1(fd: libc::c_int, action: libc::c_int) -> bool;
    fn sudo_strtobool_v1(str: *const libc::c_char) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn sudo_strtoid_v2(str: *const libc::c_char, errstr: *mut *const libc::c_char) -> id_t;
    fn log_warning(flags: libc::c_int, fmt: *const libc::c_char, _: ...) -> bool;
    fn log_warningx(flags: libc::c_int, fmt: *const libc::c_char, _: ...) -> bool;
    static mut sudo_defs_table: [sudo_defs_types; 0];
    fn sudo_strtomode_v1(cp: *const libc::c_char, errstr: *mut *const libc::c_char) -> libc::c_int;
    static mut sudo_printf:
        Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
    fn sudo_debug_enter_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    fn sudo_debug_exit_int_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: libc::c_int,
    );
    fn sudo_debug_exit_size_t_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: size_t,
    );
    fn sudo_debug_exit_str_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: *const libc::c_char,
    );
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_strlcat(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_sig2str(signo: libc::c_int, signame: *mut libc::c_char) -> libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_pw_delref(_: *mut passwd);
    fn sudo_freepwcache();
    fn sudo_freegrcache();
    fn sudo_getpwuid(_: uid_t) -> *mut passwd;
    fn sudo_getpwnam(_: *const libc::c_char) -> *mut passwd;
    fn sudo_fakepwnam(_: *const libc::c_char, _: gid_t) -> *mut passwd;
    fn sudo_fakegrnam(_: *const libc::c_char) -> *mut group;
    fn sudo_gr_delref(_: *mut group);
    fn sudo_getgrnam(_: *const libc::c_char) -> *mut group;
    fn sudo_getgrgid(_: gid_t) -> *mut group;
    fn restore_perms() -> bool;
    fn set_perms(_: libc::c_int) -> bool;
    static mut sudo_conv: sudo_conv_t;
    fn sudoers_debug_parse_flags(
        debug_files: *mut sudo_conf_debug_file_list,
        entry: *const libc::c_char,
    ) -> bool;
    fn sudoers_debug_register(
        plugin_path: *const libc::c_char,
        debug_files: *mut sudo_conf_debug_file_list,
    ) -> bool;
    fn sudoers_debug_deregister();
    fn sudo_mkdir_parents(
        path: *mut libc::c_char,
        uid: uid_t,
        gid: gid_t,
        mode: mode_t,
        quiet: bool,
    ) -> bool;
    fn gzdopen(fd: libc::c_int, mode: *const libc::c_char) -> gzFile;
    fn gzwrite(file: gzFile, buf: voidpc, len: libc::c_uint) -> libc::c_int;
    fn gzflush(file: gzFile, flush: libc::c_int) -> libc::c_int;
    fn gzclose(file: gzFile) -> libc::c_int;
    fn gzerror(file: gzFile, errnum: *mut libc::c_int) -> *const libc::c_char;
}


pub const SESSID_MAX: libc::c_uint = 2176782336;

pub const S_IWUSR: libc::c_int = 0o200;
pub const S_ISUID: libc::c_int = 0o4000;
pub const S_ISGID: libc::c_int = 0o2000;
pub const S_ISTXT: libc::c_int = 0o001000;

pub const S_IRGRP: libc::c_int = S_IRUSR >> 3;
pub const S_IWGRP: libc::c_int = S_IWUSR >> 3;
pub const S_IROTH: libc::c_int = S_IRGRP >> 3;
pub const S_IWOTH: libc::c_int = S_IWGRP >> 3;

pub const S_IRWXG: libc::c_int = S_IRWXU >> 3;
pub const S_IRWXO: libc::c_int = S_IRWXG >> 3;

pub const O_RDWR: libc::c_int = 0o2;
pub const O_CREAT: libc::c_int = 0o100;

pub const IOFD_STDIN: libc::c_int = 0;
pub const IOFD_STDOUT: libc::c_int = 1;
pub const IOFD_STDERR: libc::c_int = 2;
pub const IOFD_TTYIN: libc::c_int = 3;
pub const IOFD_TTYOUT: libc::c_int = 4;

pub const PERM_IOLOG: libc::c_int = 0x07;

pub const PATH_MAX: usize = 4096;
pub const ENAMETOOLONG: libc::c_int = 36;

pub const SUDO_LOCK: libc::c_int = 1;
pub const EOVERFLOW: libc::c_int = 75;

pub const INT_MAX: libc::c_int = 2147483647;

pub const Z_SYNC_FLUSH: libc::c_int = 2;
pub const Z_OK: libc::c_int = 0;

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(xstat_flag as libc::c_int, __path, __statbuf);
}

#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
    return __fxstat(xstat_flag as libc::c_int, __fd, __statbuf);
}

static mut io_log_files: [io_log_file; 7] = [
    {
        let mut init = io_log_file {
            enabled: 0 as libc::c_int != 0,
            suffix: b"/stdin\0" as *const u8 as *const libc::c_char,
            fd: io_fd {
                f: 0 as *const FILE as *mut FILE,
            },
        };
        init
    },
    {
        let mut init = io_log_file {
            enabled: 0 as libc::c_int != 0,
            suffix: b"/stdout\0" as *const u8 as *const libc::c_char,
            fd: io_fd {
                f: 0 as *const FILE as *mut FILE,
            },
        };
        init
    },
    {
        let mut init = io_log_file {
            enabled: 0 as libc::c_int != 0,
            suffix: b"/stderr\0" as *const u8 as *const libc::c_char,
            fd: io_fd {
                f: 0 as *const FILE as *mut FILE,
            },
        };
        init
    },
    {
        let mut init = io_log_file {
            enabled: 0 as libc::c_int != 0,
            suffix: b"/ttyin\0" as *const u8 as *const libc::c_char,
            fd: io_fd {
                f: 0 as *const FILE as *mut FILE,
            },
        };
        init
    },
    {
        let mut init = io_log_file {
            enabled: 0 as libc::c_int != 0,
            suffix: b"/ttyout\0" as *const u8 as *const libc::c_char,
            fd: io_fd {
                f: 0 as *const FILE as *mut FILE,
            },
        };
        init
    },
    {
        let mut init = io_log_file {
            enabled: 1 as libc::c_int != 0,
            suffix: b"/timing\0" as *const u8 as *const libc::c_char,
            fd: io_fd {
                f: 0 as *const FILE as *mut FILE,
            },
        };
        init
    },
    {
        let mut init = io_log_file {
            enabled: 0 as libc::c_int != 0,
            suffix: 0 as *const libc::c_char,
            fd: io_fd {
                f: 0 as *const FILE as *mut FILE,
            },
        };
        init
    },
];

static mut iolog_details: iolog_details = iolog_details {
    cwd: 0 as *const libc::c_char,
    tty: 0 as *const libc::c_char,
    user: 0 as *const libc::c_char,
    command: 0 as *const libc::c_char,
    iolog_path: 0 as *const libc::c_char,
    runas_pw: 0 as *const passwd as *mut passwd,
    runas_gr: 0 as *const group as *mut group,
    lines: 0,
    cols: 0,
    ignore_iolog_errors: false,
};
static mut iolog_compress: bool = false;
static mut warned: bool = false;
static mut last_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut sessid_max: libc::c_uint = SESSID_MAX;
static mut iolog_filemode: mode_t = (S_IRUSR | S_IWUSR) as mode_t;
static mut iolog_dirmode: mode_t = S_IRWXU as mode_t;
static mut iolog_gid_set: bool = false;

/* shared with set_perms.c */
pub type gid_t = libc::c_uint;
pub const ROOT_GID: uid_t = 0;