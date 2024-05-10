/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_variables)]

use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

extern "C" {
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn openpty(
        __amaster: *mut libc::c_int,
        __aslave: *mut libc::c_int,
        __name: *mut libc::c_char,
        __termp: *const termios,
        __winp: *const winsize,
    ) -> libc::c_int;
    fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
}

pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;

#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}

#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

#[no_mangle]
pub unsafe extern "C" fn get_pty(
    mut master: *mut libc::c_int,
    mut slave: *mut libc::c_int,
    mut name: *mut libc::c_char,
    mut namesz: size_t,
    mut ttyuid: uid_t,
) -> bool {
    let mut gr: *mut group = 0 as *mut group;
    let mut ttygid: gid_t = -(1 as libc::c_int) as gid_t;
    let mut ret: bool = false;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    gr = getgrnam(b"tty\0" as *const u8 as *const libc::c_char);
    if !gr.is_null() {
        ttygid = (*gr).gr_gid;
    }

    if openpty(
        master,
        slave,
        name,
        0 as *const termios,
        0 as *const winsize,
    ) == 0
    {
        if chown(name, ttyuid, ttygid) == 0 {
            ret = true;
        }
    }
    debug_return_bool!(ret)
}
