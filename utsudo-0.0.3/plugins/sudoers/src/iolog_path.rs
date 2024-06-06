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
use crate::runas_gr;
use crate::runas_pw;
use crate::user_name;
extern "C" {
    static mut sudo_user: sudo_user;
    static mut sudo_defs_table: [sudo_defs_types; 0];
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn io_nextid(
        iolog_dir: *mut libc::c_char,
        iolog_dir_fallback: *mut libc::c_char,
        sessid: *mut libc::c_char,
    ) -> bool;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_getgrgid(_: gid_t) -> *mut group;
    fn sudo_gr_delref(_: *mut group);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn sudoers_setlocale(newlocale: libc::c_int, prevlocale: *mut libc::c_int) -> bool;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}
pub const PATH_MAX: usize = 4096;
pub const SUDOERS_LOCALE_SUDOERS: libc::c_int = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_user {
    pub pw: *mut passwd,
    pub _runas_pw: *mut passwd,
    pub _runas_gr: *mut group,
    pub cmnd_stat: *mut stat,
    pub name: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub tty: *mut libc::c_char,
    pub ttypath: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub shost: *mut libc::c_char,
    pub runhost: *mut libc::c_char,
    pub srunhost: *mut libc::c_char,
    pub prompt: *mut libc::c_char,
    pub cmnd: *mut libc::c_char,
    pub cmnd_args: *mut libc::c_char,
    pub cmnd_base: *mut libc::c_char,
    pub cmnd_safe: *mut libc::c_char,
    pub class_name: *mut libc::c_char,
    pub krb5_ccname: *mut libc::c_char,
    pub gid_list: *mut gid_list,
    pub env_vars: *const *mut libc::c_char,
    pub role: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub iolog_file: *mut libc::c_char,
    pub gids: *mut gid_t,
    pub execfd: libc::c_int,
    pub ngids: libc::c_int,
    pub closefrom: libc::c_int,
    pub lines: libc::c_int,
    pub cols: libc::c_int,
    pub flags: libc::c_int,
    pub max_groups: libc::c_int,
    pub timeout: libc::c_int,
    pub umask: mode_t,
    pub uid: uid_t,
    pub gid: uid_t,
    pub sid: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_escape {
    pub name: *const libc::c_char,
    pub copy_fn:
        Option<unsafe extern "C" fn(*mut libc::c_char, size_t, *mut libc::c_char) -> size_t>,
}


unsafe extern "C" fn fill_seq(
    mut str: *mut libc::c_char,
    mut strsize: size_t,
    mut logdir: *mut libc::c_char,
) -> size_t {
    static mut sessid: [libc::c_char; 7] = [0; 7];
    let mut len: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if sessid[0 as usize] as libc::c_int == '\0' as i32 {
        if !io_nextid(logdir, def_iolog_dir!(), sessid.as_mut_ptr()) {
            debug_return_size_t!(-(1 as libc::c_int) as size_t);
        }
    }
    /* Path is of the form /var/log/utsudo-io/00/00/01. */
    len = snprintf(
        str,
        strsize,
        b"%c%c/%c%c/%c%c\0" as *const u8 as *const libc::c_char,
        sessid[0 as usize] as libc::c_int,
        sessid[1 as usize] as libc::c_int,
        sessid[2 as usize] as libc::c_int,
        sessid[3 as usize] as libc::c_int,
        sessid[4 as usize] as libc::c_int,
        sessid[5 as usize] as libc::c_int,
    );
    if len < 0 {
        debug_return_size_t!(strsize); /* handle non-standard snprintf() */
    }
    debug_return_size_t!(len as size_t);
}
unsafe extern "C" fn fill_user(
    mut str_0: *mut libc::c_char,
    mut strsize: size_t,
    mut unused: *mut libc::c_char,
) -> size_t {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    debug_return_size_t!(sudo_strlcpy(str_0, user_name!(), strsize));
}
unsafe extern "C" fn fill_group(
    mut str_0: *mut libc::c_char,
    mut strsize: size_t,
    mut unused: *mut libc::c_char,
) -> size_t {
    let mut grp: *mut group = 0 as *mut group;
    let mut len: size_t = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    grp = sudo_getgrgid(user_gid!());
    if !grp.is_null() {
        len = sudo_strlcpy(str_0, (*grp).gr_name, strsize);
        sudo_gr_delref(grp);
    } else {
        len = strlen(str_0);
        len = snprintf(
            str_0.offset(len as isize),
            strsize.wrapping_sub(len),
            b"#%u\0" as *const u8 as *const libc::c_char,
            user_gid!(),
        ) as size_t;
    }
    debug_return_size_t!(len);
}
unsafe extern "C" fn fill_runas_user(
    mut str_0: *mut libc::c_char,
    mut strsize: size_t,
    mut unused: *mut libc::c_char,
) -> size_t {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    debug_return_size_t!(sudo_strlcpy(str_0, (*runas_pw!()).pw_name, strsize));
}
unsafe extern "C" fn fill_runas_group(
    mut str_0: *mut libc::c_char,
    mut strsize: size_t,
    mut unused: *mut libc::c_char,
) -> size_t {
    let mut grp: *mut group = 0 as *mut group;
    let mut len: size_t = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if !runas_gr!().is_null() {
        len = sudo_strlcpy(str_0, (*runas_gr!()).gr_name, strsize);
    } else {
        grp = sudo_getgrgid((*runas_pw!()).pw_gid);
        if !grp.is_null() {
            len = sudo_strlcpy(str_0, (*grp).gr_name, strsize);
            sudo_gr_delref(grp);
        } else {
            len = strlen(str_0);
            len = snprintf(
                str_0.offset(len as isize),
                strsize.wrapping_sub(len),
                b"#%u\0" as *const u8 as *const libc::c_char,
                (*runas_pw!()).pw_gid,
            ) as size_t;
        }
    }
    debug_return_size_t!(len);
}
unsafe extern "C" fn fill_hostname(
    mut str_0: *mut libc::c_char,
    mut strsize: size_t,
    mut unused: *mut libc::c_char,
) -> size_t {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    debug_return_size_t!(sudo_strlcpy(str_0, user_shost!(), strsize));
}
unsafe extern "C" fn fill_command(
    mut str_0: *mut libc::c_char,
    mut strsize: size_t,
    mut unused: *mut libc::c_char,
) -> size_t {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    debug_return_size_t!(sudo_strlcpy(str_0, user_base!(), strsize));
}
/* Note: "seq" must be first in the list. */
static mut io_path_escapes: [path_escape; 8] = unsafe {
    [
        {
            let mut seq = path_escape {
                name: b"seq\0" as *const u8 as *const libc::c_char,
                copy_fn: Some(
                    fill_seq
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            size_t,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            };
            seq
        },
        {
            let mut user = path_escape {
                name: b"user\0" as *const u8 as *const libc::c_char,
                copy_fn: Some(
                    fill_user
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            size_t,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            };
            user
        },
        {
            let mut group = path_escape {
                name: b"group\0" as *const u8 as *const libc::c_char,
                copy_fn: Some(
                    fill_group
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            size_t,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            };
            group
        },
        {
            let mut runas_user = path_escape {
                name: b"runas_user\0" as *const u8 as *const libc::c_char,
                copy_fn: Some(
                    fill_runas_user
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            size_t,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            };
            runas_user
        },
        {
            let mut runas_group = path_escape {
                name: b"runas_group\0" as *const u8 as *const libc::c_char,
                copy_fn: Some(
                    fill_runas_group
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            size_t,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            };
            runas_group
        },
        {
            let mut hostname = path_escape {
                name: b"hostname\0" as *const u8 as *const libc::c_char,
                copy_fn: Some(
                    fill_hostname
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            size_t,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            };
            hostname
        },
        {
            let mut command = path_escape {
                name: b"command\0" as *const u8 as *const libc::c_char,
                copy_fn: Some(
                    fill_command
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            size_t,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            };
            command
        },
        {
            let mut init = path_escape {
                name: 0 as *const libc::c_char,
                copy_fn: None,
            };
            init
        },
    ]
};

