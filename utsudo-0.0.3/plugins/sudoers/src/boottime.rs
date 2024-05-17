/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_assignments)]

use crate::common::*;

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn free(_: *mut libc::c_void);
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

pub const LLONG_MAX: i64 = 9223372036854775807;

#[no_mangle]
pub unsafe extern "C" fn get_boottime(mut ts: *mut timespec) -> bool {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linesize: size_t = 0 as size_t;
    let mut found: bool = false;
    let mut llval: libc::c_longlong = 0;
    let mut len: ssize_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    /* read btime from /proc/stat */
    fp = fopen(
        b"/proc/stat\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !fp.is_null() {
        loop {
            len = getdelim(&mut line, &mut linesize, '\n' as i32, fp);
            if !(len != -(1 as libc::c_int) as libc::c_long) {
                break;
            }
            if strncmp(
                line,
                b"btime \0" as *const u8 as *const libc::c_char,
                6 as libc::c_ulong,
            ) == 0
            {
                if *line.offset((len - 1 as libc::c_long) as isize) as libc::c_int == '\n' as i32 {
                    *line.offset((len - 1 as libc::c_long) as isize) = '\0' as i32 as libc::c_char;
                }
                llval = sudo_strtonum(
                    line.offset(6 as isize),
                    1 as libc::c_longlong,
                    LLONG_MAX as libc::c_longlong,
                    0 as *mut *const libc::c_char,
                );
                if llval > 0 as libc::c_longlong {
                    (*ts).tv_sec = llval as time_t;
                    (*ts).tv_nsec = 0 as __syscall_slong_t;
                    found = true;
                    sudo_debug_printf!(
                        SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                        b"found btime in /proc/stat: %lld\0" as *const u8 as *const libc::c_char,
                        llval
                    );
                    break;
                } else {
                    sudo_debug_printf!(
                        SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                        b"invalid btime in /proc/stat: %s\0" as *const u8 as *const libc::c_char,
                        line
                    );
                }
            }
        }
        fclose(fp);
        free(line as *mut libc::c_void);
    }
    debug_return_bool!(found);
}
