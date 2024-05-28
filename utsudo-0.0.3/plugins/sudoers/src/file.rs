/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    unused_unsafe,
    unused_mut,
    unused_variables,
    clashing_extern_declarations,
    unused_assignments
)]
use crate::common::*;
//use crate::defaults::sudo_defs_table;
pub const I_IGNORE_LOCAL_SUDOERS: libc::c_int = 57;
pub const SLOG_SEND_MAIL: libc::c_int = 8;
extern "C" {
    static mut sudo_defs_table: [sudo_defs_types; 122];
    fn log_warningx(flags: libc::c_int, fmt: *const libc::c_char, _: ...) -> bool;
    static mut errorfile: *mut libc::c_char;
    fn reparent_parse_tree(new_tree: *mut sudoers_parse_tree);
    static mut errorlineno: libc::c_int;
    static mut parse_error: bool;
    static mut sudoersin: *mut FILE;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn free_parse_tree(parse_tree: *mut sudoers_parse_tree);
    fn open_sudoers(_: *const libc::c_char, _: bool, _: *mut bool) -> *mut FILE;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut sudoers_file: *const libc::c_char;
    fn init_parse_tree(
        parse_tree: *mut sudoers_parse_tree,
        shost: *const libc::c_char,
        lhost: *const libc::c_char,
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn sudoersparse() -> libc::c_int;
}
use crate::alias::cmndspec_list;
use crate::alias::comment_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec_mid {
    pub tqe_next: *mut userspec,
    pub tqe_prev: *mut *mut userspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec {
    pub entries: userspec_mid,
    pub users: member_list,
    pub privileges: privilege_list,
    pub comments: comment_list,
    pub lineno: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec_list {
    pub tqh_first: *mut userspec,
    pub tqh_last: *mut *mut userspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege_list {
    pub tqh_first: *mut privilege,
    pub tqh_last: *mut *mut privilege,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct sudo_file_handle {
    pub fp: *mut FILE,
    pub parse_tree: sudoers_parse_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_parse_tree {
    pub userspecs: userspec_list,
    pub defaults: defaults_list,
    pub aliases: *mut rbtree,
    pub shost: *const libc::c_char,
    pub lhost: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege {
    pub entries: privilege_mid,
    pub ldap_role: *mut libc::c_char,
    pub hostlist: member_list,
    pub cmndlist: cmndspec_list,
    pub defaults: defaults_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_nss {
    pub entries: sudo_nss_mid,
    pub open: Option<unsafe extern "C" fn(*mut sudo_nss) -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut sudo_nss) -> libc::c_int>,
    pub parse: Option<unsafe extern "C" fn(*mut sudo_nss) -> *mut sudoers_parse_tree>,
    pub query: Option<unsafe extern "C" fn(*mut sudo_nss, *mut passwd) -> libc::c_int>,
    pub getdefs: Option<unsafe extern "C" fn(*mut sudo_nss) -> libc::c_int>,
    pub handle: *mut libc::c_void,
    pub parse_tree: *mut sudoers_parse_tree,
    pub ret_if_found: bool,
    pub ret_if_notfound: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_nss_mid {
    pub tqe_next: *mut sudo_nss,
    pub tqe_prev: *mut *mut sudo_nss,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege_mid {
    pub tqe_next: *mut privilege,
    pub tqe_prev: *mut *mut privilege,
}

#[no_mangle]
pub unsafe extern "C" fn sudo_file_close(mut nss: *mut sudo_nss) -> libc::c_int {
    debug_decl!(SUDOERS_DEBUG_NSS!());
    let mut handle: *mut sudo_file_handle = (*nss).handle as *mut sudo_file_handle;
    if !handle.is_null() {
        fclose((*handle).fp);
        sudoersin = 0 as *const FILE as *mut FILE;
        free_parse_tree(&mut (*handle).parse_tree);
        free(handle as *mut libc::c_void);
        (*nss).handle = 0 as *mut libc::c_void;
    }
    debug_return_int!(0);
}
#[no_mangle]
pub unsafe extern "C" fn sudo_file_open(mut nss: *mut sudo_nss) -> libc::c_int {
    debug_decl!(SUDOERS_DEBUG_NSS!());
    let mut handle: *mut sudo_file_handle = 0 as *mut sudo_file_handle;
    if (*sudo_defs_table
        .as_mut_ptr()
        .offset(I_IGNORE_LOCAL_SUDOERS as isize))
    .sd_un
    .flag
        != 0
    {
        debug_return_int!(-1);
    }
    if !((*nss).handle).is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR,
            b"%s: called with non-NULL handle %p\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            (*nss).handle
        );
        sudo_file_close(nss);
    }
    handle =
        malloc(::std::mem::size_of::<sudo_file_handle>() as libc::c_ulong) as *mut sudo_file_handle;
    if !handle.is_null() {
        (*handle).fp = open_sudoers(sudoers_file, false, 0 as *mut bool);
        if !((*handle).fp).is_null() {
            init_parse_tree(
                &mut (*handle).parse_tree,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            );
        } else {
            free(handle as *mut libc::c_void);
            handle = 0 as *mut sudo_file_handle;
        }
    }
    (*nss).handle = handle as *mut libc::c_void;
    debug_return_int!(if !((*nss).handle).is_null() { 0 } else { -1 });
}
#[no_mangle]
pub unsafe extern "C" fn sudo_file_parse(mut nss: *mut sudo_nss) -> *mut sudoers_parse_tree {
    debug_decl!(SUDOERS_DEBUG_NSS!());
    let mut handle: *mut sudo_file_handle = (*nss).handle as *mut sudo_file_handle;
    if handle.is_null() || ((*handle).fp).is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR,
            b"%s: called with NULL handle %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            if !handle.is_null() {
                b"file pointer\0" as *const u8 as *const libc::c_char
            } else {
                b"handle\0" as *const u8 as *const libc::c_char
            }
        );
        debug_return_ptr!(0 as *mut sudoers_parse_tree);
    }
    sudoersin = (*handle).fp as *mut FILE;
    if sudoersparse() != 0 || parse_error as libc::c_int != 0 {
        if errorlineno != -1 {
            log_warningx(
                SLOG_SEND_MAIL,
                b"parse error in %s near line %d\0" as *const u8 as *const libc::c_char,
                errorfile,
                errorlineno,
            );
        } else {
            log_warningx(
                SLOG_SEND_MAIL,
                b"parse error in %s\0" as *const u8 as *const libc::c_char,
                errorfile,
            );
        }
        debug_return_ptr!(0 as *mut sudoers_parse_tree);
    }
    reparent_parse_tree(&mut (*handle).parse_tree);
    debug_return_ptr!(&mut (*handle).parse_tree as *mut sudoers_parse_tree);
}
#[no_mangle]
pub unsafe extern "C" fn sudo_file_query(
    mut nss: *mut sudo_nss,
    mut pw: *mut passwd,
) -> libc::c_int {
    debug_decl!(SUDOERS_DEBUG_NSS!());
    debug_return_int!(0);
}
#[no_mangle]
pub unsafe extern "C" fn sudo_file_getdefs(mut nss: *mut sudo_nss) -> libc::c_int {
    debug_decl!(SUDOERS_DEBUG_NSS!());
    debug_return_int!(0);
}
#[no_mangle]
pub static mut sudo_nss_file: sudo_nss = unsafe {
    {
        let mut init = sudo_nss {
            entries: {
                let mut init = sudo_nss_mid {
                    tqe_next: 0 as *const sudo_nss as *mut sudo_nss,
                    tqe_prev: 0 as *const *mut sudo_nss as *mut *mut sudo_nss,
                };
                init
            },
            open: Some(sudo_file_open as unsafe extern "C" fn(*mut sudo_nss) -> libc::c_int),
            close: Some(sudo_file_close as unsafe extern "C" fn(*mut sudo_nss) -> libc::c_int),
            parse: Some(
                sudo_file_parse as unsafe extern "C" fn(*mut sudo_nss) -> *mut sudoers_parse_tree,
            ),
            query: Some(
                sudo_file_query as unsafe extern "C" fn(*mut sudo_nss, *mut passwd) -> libc::c_int,
            ),
            getdefs: Some(sudo_file_getdefs as unsafe extern "C" fn(*mut sudo_nss) -> libc::c_int),
            handle: 0 as *const libc::c_void as *mut libc::c_void,
            parse_tree: 0 as *const sudoers_parse_tree as *mut sudoers_parse_tree,
            ret_if_found: false,
            ret_if_notfound: false,
        };
        init
    }
};

