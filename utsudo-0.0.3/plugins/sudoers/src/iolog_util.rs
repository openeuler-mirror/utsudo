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
    unused_mut
)]
use crate::common::*;

extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_str2sig(signame: *const libc::c_char, signum: *mut libc::c_int) -> libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_enter_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_debug_exit_v1(
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
    fn sudo_debug_exit_ptr_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: *const libc::c_void,
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
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
}

pub const INT_MAX: libc::c_int = 2147483647;

pub const LLONG_MAX: i64 = 9223372036854775807;
pub const TIME_T_MAX: i64 = LLONG_MAX;

pub const LONG_MAX: libc::c_long = 9223372036854775807;
pub const __LONG_MAX__: libc::c_long = LONG_MAX;

pub const IO_EVENT_COUNT: libc::c_int = 8;
pub const IO_EVENT_SUSPEND: libc::c_int = 7;
pub const IO_EVENT_TTYOUT_1_8_7: libc::c_int = 6;
pub const IO_EVENT_WINSIZE: libc::c_int = 5;
pub const ERANGE: libc::c_int = 34;

pub type __off64_t = libc::c_long;
pub type off64_t = __off64_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union io_fd {
    pub f: *mut FILE,
    pub g: gzFile,
    pub v: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_info {
    pub cwd: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub runas_user: *mut libc::c_char,
    pub runas_group: *mut libc::c_char,
    pub tty: *mut libc::c_char,
    pub cmd: *mut libc::c_char,
    pub tstamp: time_t,
    pub rows: libc::c_int,
    pub cols: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timing_closure {
    pub decimal: *const libc::c_char,
    pub max_delay: *mut timespec,
    pub fd: io_fd,
    pub event: libc::c_int,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub winsize: C2RustUnnamed_1,
    pub nbytes: size_t,
    pub signo: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub rows: libc::c_int,
    pub cols: libc::c_int,
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

#[macro_export]
macro_rules! ULONG_MAX {
    () => {{
        (__LONG_MAX__ as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
    }};
}
static mut timing_event_adj: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn parse_logfile(mut logfile: *const libc::c_char) -> *mut log_info {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut cwdsize: size_t = 0 as libc::c_int as size_t;
    let mut cmdsize: size_t = 0 as libc::c_int as size_t;
    let mut li: *mut log_info = 0 as *mut log_info;
    debug_decl!(SUDO_DEBUG_UTIL);

    fp = fopen(logfile, b"r\0" as *const u8 as *const libc::c_char);
    'bad: loop {
        if fp.is_null() {
            sudo_warn!(
                b"unable to open %s\0" as *const u8 as *const libc::c_char,
                logfile
            );
            break 'bad;
        }

        /*
         * ID file has three lines:
         *  1) a log info line
         *  2) cwd
         *  3) command with args
         */
        li = calloc(
            1 as libc::c_ulong,
            ::core::mem::size_of::<log_info>() as libc::c_ulong,
        ) as *mut log_info;
        if li.is_null() {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }
        if getdelim(&mut buf, &mut bufsize, '\n' as i32, fp) == -(1 as libc::c_int) as libc::c_long
            || getdelim(&mut (*li).cwd, &mut cwdsize, '\n' as i32, fp)
                == -(1 as libc::c_int) as libc::c_long
            || getdelim(&mut (*li).cmd, &mut cmdsize, '\n' as i32, fp)
                == -(1 as libc::c_int) as libc::c_long
        {
            sudo_warn!(
                b"%s: invalid log file\0" as *const u8 as *const libc::c_char,
                logfile
            );
            break 'bad;
        }

        /* Strip the newline from the cwd and command. */
        *((*li).cwd)
            .offset(strcspn((*li).cwd, b"\n\0" as *const u8 as *const libc::c_char) as isize) =
            '\0' as i32 as libc::c_char;
        *((*li).cmd)
            .offset(strcspn((*li).cmd, b"\n\0" as *const u8 as *const libc::c_char) as isize) =
            '\0' as i32 as libc::c_char;

        /*
         * Crack the log line (rows and cols not present in old versions).
         *	timestamp:user:runas_user:runas_group:tty:rows:cols
         * XXX - probably better to use strtok and switch on the state.
         */
        *buf.offset(strcspn(buf, b"\n\0" as *const u8 as *const libc::c_char) as isize) =
            '\0' as i32 as libc::c_char;
        cp = buf;

        /* timestamp */
        ep = strchr(cp, ':' as i32);
        if ep.is_null() {
            sudo_warn!(
                b"%s: time stamp field is missing\0" as *const u8 as *const libc::c_char,
                logfile
            );
            break 'bad;
        }
        *ep = '\0' as i32 as libc::c_char;
        (*li).tstamp = sudo_strtonum(
            cp,
            0 as libc::c_longlong,
            TIME_T_MAX as libc::c_longlong,
            &mut errstr,
        ) as time_t;
        if !errstr.is_null() {
            sudo_warn!(
                b"%s: time stamp %s: %s\0" as *const u8 as *const libc::c_char,
                logfile,
                cp,
                errstr
            );
            break 'bad;
        }

        /* user */
        cp = ep.offset(1 as isize);
        ep = strchr(cp, ':' as i32);
        if ep.is_null() {
            sudo_warn!(
                b"%s: user field is missing\0" as *const u8 as *const libc::c_char,
                logfile
            );
            break 'bad;
        }
        (*li).user = strndup(cp, ep.offset_from(cp) as libc::c_long as size_t);
        if ((*li).user).is_null() {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }

        /* runas user */
        cp = ep.offset(1 as isize);
        ep = strchr(cp, ':' as i32);
        if ep.is_null() {
            sudo_warn!(
                b"%s: runas user field is missing\0" as *const u8 as *const libc::c_char,
                logfile
            );
            break 'bad;
        }
        (*li).runas_user = strndup(cp, ep.offset_from(cp) as libc::c_long as size_t);
        if ((*li).runas_user).is_null() {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }

        /* runas group */
        cp = ep.offset(1 as isize);
        ep = strchr(cp, ':' as i32);
        if ep.is_null() {
            sudo_warn!(
                b"%s: runas group field is missing\0" as *const u8 as *const libc::c_char,
                logfile
            );
            break 'bad;
        }
        if cp != ep {
            (*li).runas_group = strndup(cp, ep.offset_from(cp) as libc::c_long as size_t);
            if ((*li).runas_group).is_null() {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
        }

        /* tty, followed by optional rows + columns */
        cp = ep.offset(1 as isize);
        ep = strchr(cp, ':' as i32);
        if ep.is_null() {
            /* just the tty */
            (*li).tty = strdup(cp);
            if ((*li).tty).is_null() {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
        } else {
            /* tty followed by rows + columns */
            (*li).tty = strndup(cp, ep.offset_from(cp) as libc::c_long as size_t);
            if ((*li).tty).is_null() {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
            cp = ep.offset(1 as isize);
            /* need to NULL out separator to use sudo_strtonum() */
            /* XXX - use sudo_strtonumx */
            ep = strchr(cp, ':' as i32);
            if !ep.is_null() {
                *ep = '\0' as i32 as libc::c_char;
            }
            (*li).rows = sudo_strtonum(
                cp,
                1 as libc::c_longlong,
                INT_MAX as libc::c_longlong,
                &mut errstr,
            ) as libc::c_int;
            if !errstr.is_null() {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                    b"%s: tty rows %s: %s\0" as *const u8 as *const libc::c_char,
                    logfile,
                    cp,
                    errstr
                );
            }
            if !ep.is_null() {
                cp = ep.offset(1 as isize);
                (*li).cols = sudo_strtonum(
                    cp,
                    1 as libc::c_longlong,
                    INT_MAX as libc::c_longlong,
                    &mut errstr,
                ) as libc::c_int;
                if !errstr.is_null() {
                    sudo_debug_printf!(
                        SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                        b"%s: tty cols %s: %s\0" as *const u8 as *const libc::c_char,
                        logfile,
                        cp,
                        errstr
                    );
                }
            }
        }
        fclose(fp);
        free(buf as *mut libc::c_void);
        debug_return_ptr!(li);
    }

    //bad:
    if !fp.is_null() {
        fclose(fp);
    }
    free(buf as *mut libc::c_void);
    free_log_info(li);
    debug_return_ptr!(0 as *mut log_info);
}
