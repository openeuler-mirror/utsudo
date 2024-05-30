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

#[no_mangle]
pub unsafe extern "C" fn find_path(
    mut infile: *const libc::c_char,
    mut outfile: *mut *mut libc::c_char,
    mut sbp: *mut stat,
    mut path: *const libc::c_char,
    mut ignore_dot: libc::c_int,
    mut whitelist: *const *mut libc::c_char,
) -> libc::c_int {
    let mut command: [libc::c_char; 4096] = [0; 4096];
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut ep: *const libc::c_char = 0 as *const libc::c_char;
    let mut pathend: *const libc::c_char = 0 as *const libc::c_char;
    let mut found: bool = 0 as libc::c_int != 0;
    let mut checkdot: bool = 0 as libc::c_int != 0;
    let mut len: libc::c_int = 0;
    let sudo_debug_subsys_tmp: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(17 as libc::c_int as isize)
        as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0")).as_ptr(),
        b"find_path.c\0" as *const u8 as *const libc::c_char,
        109 as libc::c_int,
        sudo_debug_subsys_tmp,
    );
    if !(strchr(infile, '/' as i32)).is_null() {
        if sudo_strlcpy(
            command.as_mut_ptr(),
            infile,
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        ) >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        {
            *__errno_location() = 36 as libc::c_int;
            let mut sudo_debug_ret: libc::c_int = 3 as libc::c_int;
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0"))
                    .as_ptr(),
                b"find_path.c\0" as *const u8 as *const libc::c_char,
                118 as libc::c_int,
                sudo_debug_subsys_tmp,
                sudo_debug_ret,
            );
            return sudo_debug_ret;
        }
        found = cmnd_allowed(
            command.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            sbp,
            whitelist,
        );
    } else {
        if path.is_null() {
            let mut sudo_debug_ret_0: libc::c_int = 1 as libc::c_int;
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0"))
                    .as_ptr(),
                b"find_path.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int,
                sudo_debug_subsys_tmp,
                sudo_debug_ret_0,
            );
            return sudo_debug_ret_0;
        }
        pathend = path.offset(strlen(path) as isize);
        cp = sudo_strsplit_v1(
            path,
            pathend,
            b":\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );
        while !cp.is_null() {
            if cp == ep
                || *cp as libc::c_int == '.' as i32 && cp.offset(1 as libc::c_int as isize) == ep
            {
                checkdot = 1 as libc::c_int != 0;
            } else {
                len = snprintf(
                    command.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    b"%.*s/%s\0" as *const u8 as *const libc::c_char,
                    ep.offset_from(cp) as libc::c_long as libc::c_int,
                    cp,
                    infile,
                );
                if len < 0 as libc::c_int
                    || len as libc::c_long
                        >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                            as ssize_t
                {
                    *__errno_location() = 36 as libc::c_int;
                    let mut sudo_debug_ret_1: libc::c_int = 3 as libc::c_int;
                    sudo_debug_exit_int_v1(
                        (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0"))
                            .as_ptr(),
                        b"find_path.c\0" as *const u8 as *const libc::c_char,
                        147 as libc::c_int,
                        sudo_debug_subsys_tmp,
                        sudo_debug_ret_1,
                    );
                    return sudo_debug_ret_1;
                }
                found = cmnd_allowed(
                    command.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    sbp,
                    whitelist,
                );
                if found {
                    break;
                }
            }
            cp = sudo_strsplit_v1(
                0 as *const libc::c_char,
                pathend,
                b":\0" as *const u8 as *const libc::c_char,
                &mut ep,
            );
        }
        if !found && checkdot as libc::c_int != 0 {
            len = snprintf(
                command.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                b"./%s\0" as *const u8 as *const libc::c_char,
                infile,
            );
            if len < 0 as libc::c_int
                || len as libc::c_long
                    >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as ssize_t
            {
                *__errno_location() = 36 as libc::c_int;
                let mut sudo_debug_ret_2: libc::c_int = 3 as libc::c_int;
                sudo_debug_exit_int_v1(
                    (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0"))
                        .as_ptr(),
                    b"find_path.c\0" as *const u8 as *const libc::c_char,
                    161 as libc::c_int,
                    sudo_debug_subsys_tmp,
                    sudo_debug_ret_2,
                );
                return sudo_debug_ret_2;
            }
            found = cmnd_allowed(
                command.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                sbp,
                whitelist,
            );
            if found as libc::c_int != 0 && ignore_dot != 0 {
                let mut sudo_debug_ret_3: libc::c_int = 2 as libc::c_int;
                sudo_debug_exit_int_v1(
                    (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0"))
                        .as_ptr(),
                    b"find_path.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                    sudo_debug_subsys_tmp,
                    sudo_debug_ret_3,
                );
                return sudo_debug_ret_3;
            }
        }
    }
    if found {
        *outfile = strdup(command.as_mut_ptr());
        if (*outfile).is_null() {
            let mut sudo_debug_ret_4: libc::c_int = 3 as libc::c_int;
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0"))
                    .as_ptr(),
                b"find_path.c\0" as *const u8 as *const libc::c_char,
                171 as libc::c_int,
                sudo_debug_subsys_tmp,
                sudo_debug_ret_4,
            );
            return sudo_debug_ret_4;
        }
        let mut sudo_debug_ret_5: libc::c_int = 0 as libc::c_int;
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0")).as_ptr(),
            b"find_path.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            sudo_debug_subsys_tmp,
            sudo_debug_ret_5,
        );
        return sudo_debug_ret_5;
    }
    let mut sudo_debug_ret_6: libc::c_int = 1 as libc::c_int;
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"find_path\0")).as_ptr(),
        b"find_path.c\0" as *const u8 as *const libc::c_char,
        174 as libc::c_int,
        sudo_debug_subsys_tmp,
        sudo_debug_ret_6,
    );
    return sudo_debug_ret_6;
}

