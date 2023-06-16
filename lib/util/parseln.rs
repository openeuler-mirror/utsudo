/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

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
    pub _unused2: [libc::c_char; 20],
}

pub unsafe extern "C" fn sudo_parseln_v2()


pub unsafe fn sudo_parseln_v1()

