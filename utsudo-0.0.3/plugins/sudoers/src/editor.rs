/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    unused_variables,
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unreachable_code
)]
use crate::common::*;
extern "C" {
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    fn strndup(__string: *const libc::c_char, __n: size_t) -> *mut libc::c_char;
    fn find_path(
        infile: *const libc::c_char,
        outfile: *mut *mut libc::c_char,
        sbp: *mut stat,
        path: *const libc::c_char,
        ignore_dot: libc::c_int,
        whitelist: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn strlen(__s: *const libc::c_char) -> size_t;
    static mut sudo_defs_table: [sudo_defs_types; 0];
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type def_tuple = libc::c_uint;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_member {
    pub entries: TMP_struct,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TMP_struct {
    pub sle_next: *mut list_member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_members {
    pub slh_first: *mut list_member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sudo_defs_val {
    pub flag: libc::c_int,
    pub ival: libc::c_int,
    pub uival: libc::c_int,
    pub tuple: def_tuple,
    pub str_0: *mut libc::c_char,
    pub mode: mode_t,
    pub tspec: timespec,
    pub list: list_members,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_values {
    pub sval: *mut libc::c_char,
    pub nval: def_tuple,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_defs_types {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub desc: *mut libc::c_char,
    pub values: *mut def_values,
    pub callback: Option<unsafe extern "C" fn(*const sudo_defs_val) -> bool>,
    pub sd_un: sudo_defs_val,
}
#[no_mangle]
unsafe extern "C" fn resolve_editor(
    mut ed: *const libc::c_char,
    mut edlen: size_t,
    mut nfiles: libc::c_int,
    mut files: *mut *mut libc::c_char,
    mut argc_out: *mut libc::c_int,
    mut argv_out: *mut *mut *mut libc::c_char,
    mut whitelist: *const *mut libc::c_char,
) -> *mut libc::c_char {
    let mut editor_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut editor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *const libc::c_char = 0 as *mut libc::c_char;
    let mut ep: *const libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *mut libc::c_char;
    let mut edend: *const libc::c_char = 0 as *const libc::c_char;
    edend = ed.offset(edlen as isize); //isize usize ?
    let mut user_editor_sb: stat = sb_all_arch;
    let mut nargc: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    cp = sudo_strsplit_v1(
        ed,
        edend,
        b"\t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );
    if cp.is_null() {
        debug_return_str!(0 as *mut libc::c_char);
    }
    editor = strndup(cp, ep.offset_from(cp) as size_t);
    if editor.is_null() {
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_str!(0 as *mut libc::c_char);
    }
    if find_path(
        editor,
        &mut editor_path,
        &mut user_editor_sb,
        getenv(b"PATH\0" as *const u8 as *const libc::c_char),
        0,
        whitelist,
    ) != 0
    {
        free(editor as *mut libc::c_void);
        *__errno_location() = 2;
        debug_return_str!(0 as *mut libc::c_char);
    }
    nargc = 1;
    tmp = ep;
    while !(sudo_strsplit_v1(
        0 as *const libc::c_char,
        edend,
        b"\t\0" as *const u8 as *const libc::c_char,
        &mut tmp,
    ))
    .is_null()
    {
        nargc += 1;
    }
    if nfiles != 0 {
        nargc += nfiles + 1;
    }
    nargv = reallocarray(
        0 as *mut libc::c_void,
        (nargc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if nargv.is_null() {
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        free(editor as *mut libc::c_void);
        free(editor_path as *mut libc::c_void);
        debug_return_str!(0 as *mut libc::c_char);
    }
    *nargv.offset(0) = editor;
    nargc = 1;
    loop {
        cp = sudo_strsplit_v1(
            0 as *mut libc::c_char,
            edend,
            b" \t\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );
        if cp.is_null() {
            break;
        }
        *nargv.offset(nargc as isize) = strndup(cp, ep.offset_from(cp) as size_t);
        if (*nargv.offset(nargc as isize)).is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            free(editor_path as *mut libc::c_void);
            loop {
                let fresh2 = nargc;
                nargc = nargc - 1;
                if !(fresh2 != 0) {
                    break;
                }
                free(*nargv.offset(nargc as isize) as *mut libc::c_void);
            }
            free(nargv as *mut libc::c_void);
            debug_return_str!(0 as *mut libc::c_char);
        }
        nargc += 1;
    }
    if nfiles != 0 as libc::c_int {
        let fresh3 = nargc;
        nargc = nargc + 1;
        let ref mut fresh4 = *nargv.offset(fresh3 as isize);
        *fresh4 = b"--\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        loop {
            let fresh5 = nfiles;
            nfiles = nfiles - 1;
            if !(fresh5 != 0) {
                break;
            }
            let fresh6 = files;
            files = files.offset(1);
            let fresh7 = nargc;
            nargc = nargc + 1;
            let ref mut fresh8 = *nargv.offset(fresh7 as isize);
            *fresh8 = *fresh6;
        }
    }
    let ref mut fresh9 = *nargv.offset(nargc as isize);
    *fresh9 = 0 as *mut libc::c_char;
    *argc_out = nargc;
    *argv_out = nargv;
    debug_return_str!(editor_path);
}
#[no_mangle]
unsafe extern "C" fn find_editor(
    mut nfiles: libc::c_int,
    mut files: *mut *mut libc::c_char,
    mut argc_out: *mut libc::c_int,
    mut argv_out: *mut *mut *mut libc::c_char,
    mut whitelist: *const *mut libc::c_char,
    mut env_editor: *mut *const libc::c_char,
    env_error: bool,
) -> *mut libc::c_char {
    let mut ev: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut editor_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_uint = 0 as libc::c_uint;
    let SUDOERS_DEBUG_UTIL: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(17 as libc::c_int as isize)
        as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL);
    *env_editor = 0 as *const libc::c_char;
    ev[0] = b"SUDO_EDITOR\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    ev[1] = b"VISUAL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    ev[2] = b"EDITOR\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    i = 0;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        let mut editor: *mut libc::c_char = getenv(ev[i as usize]) as *mut libc::c_char;
        if !editor.is_null() && *editor as libc::c_int != '\u{0}' as i32 {
            // libc::c_int ?
            *env_editor = editor;
            editor_path = resolve_editor(
                editor,
                strlen(editor),
                nfiles,
                files,
                argc_out,
                argv_out,
                whitelist,
            );
            if !editor_path.is_null() {
                break;
            }
            if *__errno_location() != 2 {
                debug_return_str!(0 as *mut libc::c_char);
            }
        }
        i += 1;
    }
    if editor_path.is_null() {
        let mut def_editor_end: *const libc::c_char = ((*sudo_defs_table
            .as_mut_ptr()
            .offset(53 as libc::c_int as isize))
        .sd_un
        .str_0)
            .offset(strlen(
                (*sudo_defs_table
                    .as_mut_ptr()
                    .offset(53 as libc::c_int as isize))
                .sd_un
                .str_0,
            ) as isize); // ??
        let mut cp: *const libc::c_char = 0 as *const libc::c_char;
        let mut ep: *const libc::c_char = 0 as *const libc::c_char;
        if env_error as libc::c_int != 0 && !(*env_editor).is_null() {
            debug_return_str!(0 as *mut libc::c_char);
        }
        cp = sudo_strsplit_v1(
            (*sudo_defs_table.as_mut_ptr().offset(53)).sd_un.str_0,
            def_editor_end,
            b":\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );
        while !cp.is_null() {
            editor_path = resolve_editor(
                cp,
                ep.offset_from(cp) as size_t,
                nfiles,
                files,
                argc_out,
                argv_out,
                whitelist,
            );
            if !editor_path.is_null() {
                break;
            }
            if *__errno_location() != 2 {
                debug_return_str!(0 as *mut libc::c_char);
            }
        }
    }
    debug_return_str!(editor_path);
}
