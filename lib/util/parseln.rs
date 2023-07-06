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

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_ssize_t_v1;
use crate::sudo_debug::sudo_debug_printf2_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_LINENO;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

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

pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type __SIZE_TYPE__ = libc::c_ulong;
pub type size_t = __SIZE_TYPE__;
pub type FILE = _IO_FILE;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

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
            && (*line.offset((len - 1 as libc::c_long) as isize) as libc::c_int == '\n' as i32)
        {
            len -= 1;
            *line.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
        }
        cp = strchr(line, '#' as i32);
        if !cp.is_null() {
            if cp == line || (flags & PARSELN_COMM_BOL) == 0 {
                *cp = '\u{0}' as i32 as libc::c_char;
                len = cp.offset_from(line) as libc::c_long; //cp-line
            }            
        }
    }
}

#[no_mangle]
pub unsafe fn sudo_parseln_v1(
    mut bufp: *mut *mut libc::c_char,
    mut bufsizep: *mut size_t,
    mut fp: *mut FILE,
) -> ssize_t {
    return sudo_parseln_v2(bufp, bufsizep, fp);
}

