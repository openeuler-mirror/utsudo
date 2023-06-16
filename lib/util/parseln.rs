/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */


use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_ssize_t_v1;
use crate::sudo_debug::sudo_debug_printf2_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_LINENO;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

pub struct _IO_marker {
    _unused: [u8; 0],
}
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
pub struct _IO_wide_data {
    _unused: [u8; 0],
}


pub type __SIZE_TYPE__ = libc::c_ulong;
pub type size_t = __SIZE_TYPE__;
pub type FILE = _IO_FILE;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

pub struct _IO_FILE {
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
    pub _unused2: [libc::c_char; 20],
}

pub unsafe extern "C" fn sudo_parseln_v2()


pub unsafe fn sudo_parseln_v1()
