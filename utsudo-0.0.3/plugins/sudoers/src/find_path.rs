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
    clashing_extern_declarations
)]
use crate::common::*;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;
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
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_goodpath(path: *const libc::c_char, sbp: *mut stat) -> bool;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;

unsafe extern "C" fn cmnd_allowed(
    mut cmnd: *mut libc::c_char,
    mut cmnd_size: size_t,
    mut cmnd_sbp: *mut stat,
    mut whitelist: *const *mut libc::c_char,
) -> bool {
    let mut cmnd_base: *const libc::c_char = 0 as *const libc::c_char;
    let mut wl: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    let sudo_debug_subsys_tmp: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(17 as libc::c_int as isize)
        as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cmnd_allowed\0")).as_ptr(),
        b"find_path.c\0" as *const u8 as *const libc::c_char,
        57 as libc::c_int,
        sudo_debug_subsys_tmp,
    );
    if !sudo_goodpath(cmnd, cmnd_sbp) {
        let mut sudo_debug_ret: bool = 0 as libc::c_int != 0;
        sudo_debug_exit_bool_v1(
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cmnd_allowed\0")).as_ptr(),
            b"find_path.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            sudo_debug_subsys_tmp,
            sudo_debug_ret,
        );
        return sudo_debug_ret;
    }
    if whitelist.is_null() {
        let mut sudo_debug_ret_0: bool = 1 as libc::c_int != 0;
        sudo_debug_exit_bool_v1(
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cmnd_allowed\0")).as_ptr(),
            b"find_path.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            sudo_debug_subsys_tmp,
            sudo_debug_ret_0,
        );
        return sudo_debug_ret_0;
    }
    cmnd_base = strrchr(cmnd, '/' as i32);
    if cmnd_base.is_null() {
        let mut sudo_debug_ret_1: bool = 0 as libc::c_int != 0;
        sudo_debug_exit_bool_v1(
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cmnd_allowed\0")).as_ptr(),
            b"find_path.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            sudo_debug_subsys_tmp,
            sudo_debug_ret_1,
        );
        return sudo_debug_ret_1;
    }
    cmnd_base = cmnd_base.offset(1);
    wl = whitelist;
    while !(*wl).is_null() {
        let mut sb: stat = sb_all_arch;
        let mut base: *const libc::c_char = 0 as *const libc::c_char;
        base = strrchr(*wl, '/' as i32);
        if !base.is_null() {
            base = base.offset(1);
            if !(strcmp(cmnd_base, base) != 0 as libc::c_int) {
                if sudo_goodpath(*wl, &mut sb) as libc::c_int != 0
                    && sb.st_dev == (*cmnd_sbp).st_dev
                    && sb.st_ino == (*cmnd_sbp).st_ino
                {
                    if sudo_strlcpy(cmnd, *wl, cmnd_size) < cmnd_size {
                        return 1 as libc::c_int != 0;
                    }
                    let mut sudo_debug_ret_2: bool = 1 as libc::c_int != 0;
                    sudo_debug_exit_bool_v1(
                        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(
                            b"cmnd_allowed\0",
                        ))
                        .as_ptr(),
                        b"find_path.c\0" as *const u8 as *const libc::c_char,
                        86 as libc::c_int,
                        sudo_debug_subsys_tmp,
                        sudo_debug_ret_2,
                    );
                    return sudo_debug_ret_2;
                }
            }
        }
        wl = wl.offset(1);
    }
    let mut sudo_debug_ret_3: bool = 0 as libc::c_int != 0;
    sudo_debug_exit_bool_v1(
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cmnd_allowed\0")).as_ptr(),
        b"find_path.c\0" as *const u8 as *const libc::c_char,
        89 as libc::c_int,
        sudo_debug_subsys_tmp,
        sudo_debug_ret_3,
    );
    return sudo_debug_ret_3;
}

