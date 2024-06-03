/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(dead_code, non_camel_case_types, unused_assignments, unused_mut)]
use crate::common::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn sudoers_setlocale(newlocale: libc::c_int, prevlocale: *mut libc::c_int) -> bool;
    fn free_aliases(aliases: *mut rbtree);
    fn alias_add(
        parse_tree: *mut sudoers_parse_tree,
        name: *mut libc::c_char,
        type_0: libc::c_int,
        file: *mut libc::c_char,
        lineno: libc::c_int,
        members: *mut member,
    ) -> *const libc::c_char;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    static mut sudoers: *mut libc::c_char;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn init_lexer();
    fn sudo_debug_exit_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn parse_timeout(timestr: *const libc::c_char) -> libc::c_int;
    fn parse_gentime(expstr: *const libc::c_char) -> time_t;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    static mut sudo_printf:
        Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
    fn sudo_debug_exit_ptr_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: *const libc::c_void,
    );
    static mut sudolineno: libc::c_int;
    static mut last_token: libc::c_int;
    fn sudoerslex() -> libc::c_int;
    fn rcstr_dup(src: *const libc::c_char) -> *mut libc::c_char;
    fn rcstr_addref(s: *const libc::c_char) -> *mut libc::c_char;
    fn rcstr_delref(s: *const libc::c_char);
    fn sudoers_trace_print(msg: *const libc::c_char) -> libc::c_int;
    static mut trace_print: Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defaults_list {
    pub tqh_first: *mut defaults,
    pub tqh_last: *mut *mut defaults,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defaults {
    pub entries: C2RustUnnamed_0,
    pub var: *mut libc::c_char,
    pub val: *mut libc::c_char,
    pub binding: *mut member_list,
    pub file: *mut libc::c_char,
    pub type_0: libc::c_short,
    pub op: libc::c_char,
    pub error: libc::c_char,
    pub lineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct member_list {
    pub tqh_first: *mut member,
    pub tqh_last: *mut *mut member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct member {
    pub entries: C2RustUnnamed,
    pub name: *mut libc::c_char,
    pub type_0: libc::c_short,
    pub negated: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tqe_next: *mut member,
    pub tqe_prev: *mut *mut member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut defaults,
    pub tqe_prev: *mut *mut defaults,
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
pub struct userspec_list {
    pub tqh_first: *mut userspec,
    pub tqh_last: *mut *mut userspec,
}
//多处定义，后期统一处理
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec {
    pub entries: C2RustUnnamed_4,
    pub users: member_list,
    pub privileges: privilege_list,
    pub comments: comment_list,
    pub lineno: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comment_list {
    pub stqh_first: *mut sudoers_comment,
    pub stqh_last: *mut *mut sudoers_comment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_comment {
    pub entries: C2RustUnnamed_1,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub stqe_next: *mut sudoers_comment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege_list {
    pub tqh_first: *mut privilege,
    pub tqh_last: *mut *mut privilege,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege {
    pub entries: C2RustUnnamed_3,
    pub ldap_role: *mut libc::c_char,
    pub hostlist: member_list,
    pub cmndlist: cmndspec_list,
    pub defaults: defaults_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmndspec_list {
    pub tqh_first: *mut cmndspec,
    pub tqh_last: *mut *mut cmndspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmndspec {
    pub entries: C2RustUnnamed_2,
    pub runasuserlist: *mut member_list,
    pub runasgrouplist: *mut member_list,
    pub cmnd: *mut member,
    pub tags: cmndtag,
    pub timeout: libc::c_int,
    pub notbefore: time_t,
    pub notafter: time_t,
    pub role: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
}
//多处定义，后期统一处理
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct cmndtag {
    #[bitfield(name = "nopasswd", ty = "libc::c_int", bits = "0..=2")]
    #[bitfield(name = "noexec", ty = "libc::c_int", bits = "3..=5")]
    #[bitfield(name = "setenv", ty = "libc::c_int", bits = "6..=8")]
    #[bitfield(name = "log_input", ty = "libc::c_int", bits = "9..=11")]
    #[bitfield(name = "log_output", ty = "libc::c_int", bits = "12..=14")]
    #[bitfield(name = "send_mail", ty = "libc::c_int", bits = "15..=17")]
    #[bitfield(name = "follow", ty = "libc::c_int", bits = "18..=20")]
    pub nopasswd_noexec_setenv_log_input_log_output_send_mail_follow: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut cmndspec,
    pub tqe_prev: *mut *mut cmndspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut privilege,
    pub tqe_prev: *mut *mut privilege,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tqe_next: *mut userspec,
    pub tqe_prev: *mut *mut userspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_digest {
    pub digest_type: libc::c_uint,
    pub digest_str: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_command {
    pub cmnd: *mut libc::c_char,
    pub args: *mut libc::c_char,
    pub digest: *mut command_digest,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct runascontainer {
    pub runasusers: *mut member,
    pub runasgroups: *mut member,
}
pub type sudo_printf_t =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub cmndspec: *mut cmndspec,
    pub defaults: *mut defaults,
    pub member: *mut member,
    pub runas: *mut runascontainer,
    pub privilege: *mut privilege,
    pub digest: *mut command_digest,
    pub command: sudo_command,
    pub options: command_options,
    pub tag: cmndtag,
    pub string: *mut libc::c_char,
    pub tok: libc::c_int,
}
