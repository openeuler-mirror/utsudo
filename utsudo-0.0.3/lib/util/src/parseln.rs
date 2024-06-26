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
    improper_ctypes
)]

use crate::macro_struct::*;
use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

extern "C" {
    fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

/* sudo_parseln() flags */
// #define PARSELN_COMM_BOL	0x01	/* comments only at begining of line */
// #define PARSELN_CONT_IGN	0x02	/* ignore line continuation char */
pub const PARSELN_COMM_BOL: libc::c_int = 0x01;
pub const PARSELN_CONT_IGN: libc::c_int = 0x02;

pub const _ISblank: libc::c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn sudo_parseln_v2(
    mut bufp: *mut *mut libc::c_char, //**bufp -> *mut *mut
    mut bufsizep: *mut size_t,        //*bufsizep -> *mut
    mut lineno: *mut libc::c_uint,
    mut fp: *mut FILE,
    mut flags: libc::c_int,
) -> ssize_t {
    let mut linesize: size_t = 0;
    let mut total: size_t = 0;
    let mut len: ssize_t = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut continued: bool = false;
    let mut comment: bool = false;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    loop {
        comment = false;
        continued = false;
        len = getdelim(&mut line, &mut linesize, '\n' as i32, fp);
        if len == -1 {
            break;
        }
        if !lineno.is_null() {
            *lineno = (*lineno).wrapping_add(1) //(*lineno)++
        }
        while len > 0 as libc::c_long
            && (*line.offset((len - 1 as libc::c_long) as isize) as libc::c_int == '\n' as i32
                || *line.offset((len - 1 as libc::c_long) as isize) as libc::c_int == '\r' as i32)
        {
            len -= 1;
            *line.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
        }
        cp = strchr(line, '#' as i32);
        if !cp.is_null() {
            if cp == line || (flags & PARSELN_COMM_BOL) == 0 {
                *cp = '\u{0}' as i32 as libc::c_char;
                len = cp.offset_from(line) as libc::c_long; //cp-line
                comment = true;
            }
        }
        if !comment && (flags & PARSELN_CONT_IGN) == 0 {
            if len > 0 as libc::c_long
                && *line.offset((len - 1 as libc::c_long) as isize) as libc::c_int == '\\' as i32
                && (len == 1 as libc::c_long
                    || *line.offset((len - 2 as libc::c_long) as isize) as libc::c_int
                        != '\\' as i32)
            {
                *line.offset((len - 1 as libc::c_long) as isize) = '\u{0}' as i32 as libc::c_char;
                continued = true;
            }
        }

        if !continued {
            while len > 0 as libc::c_long
                && *(*__ctype_b_loc()).offset(
                    *line.offset((len - 1 as libc::c_long) as isize) as libc::c_uchar as isize
                ) as libc::c_int
                    & _ISblank as libc::c_ushort as libc::c_int
                    != 0
            {
                *line.offset((len - 1 as libc::c_long) as isize) = '\0' as i32 as libc::c_char;
            }
        }

        //for->while
        cp = line;
        while *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISblank as libc::c_ushort as libc::c_int
            != 0
        {
            len -= 1;
            cp = cp.offset(1)
        }
        if (*bufp).is_null() || total.wrapping_add(len as libc::c_ulong) >= *bufsizep {
            let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut size: size_t = total
                .wrapping_add(len as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong);
            if size < 64 as libc::c_ulong {
                size = 64 as libc::c_ulong;
            } else if size <= 0x80000000 as libc::c_ulong {
                size = size.wrapping_sub(1); //size--
                size |= size >> 1;
                size |= size >> 2;
                size |= size >> 4;
                size |= size >> 8;
                size |= size >> 16;
                size = size.wrapping_add(1); //size++
            }
            tmp = realloc(*bufp as *mut libc::c_void, size);
            if tmp.is_null() {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
                len = -1;
                total = 0;
                break;
            }
            *bufp = tmp as *mut libc::c_char;
            *bufsizep = size;
        }
        memcpy(
            (*bufp).offset(total as isize) as *mut libc::c_void,
            cp as *const libc::c_void,
            (len + 1 as libc::c_long) as libc::c_ulong,
        );
        total = (total as libc::c_ulong).wrapping_add(len as libc::c_ulong) as size_t;
        if !continued {
            break;
        }
    }
    free(line as *mut libc::c_void);
    if len == -1 as libc::c_long && total == 0 as libc::c_ulong {
        debug_return_ssize_t!(-1 as ssize_t);
    }
    debug_return_ssize_t!(total as ssize_t)
}

#[no_mangle]
pub unsafe fn sudo_parseln_v1(
    mut bufp: *mut *mut libc::c_char,
    mut bufsizep: *mut size_t,
    mut lineno: *mut libc::c_uint,
    mut fp: *mut FILE,
) -> ssize_t {
    return sudo_parseln_v2(bufp, bufsizep, lineno, fp, 0);
}
