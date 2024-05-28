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
