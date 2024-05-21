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
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn alias_get(
        parse_tree: *mut sudoers_parse_tree,
        name: *const libc::c_char,
        type_0: libc::c_int,
    ) -> *mut alias;
    fn alias_put(a: *mut alias);
    fn base64_encode(
        in_0: *const libc::c_uchar,
        in_len: size_t,
        out: *mut libc::c_char,
        out_len: size_t,
    ) -> size_t;
    fn digest_type_to_name(digest_type: libc::c_int) -> *const libc::c_char;
    fn sudoers_format_default_line(
        lbuf: *mut sudo_lbuf,
        parse_tree: *mut sudoers_parse_tree,
        d: *mut defaults,
        next: *mut *mut defaults,
        expand_aliases: bool,
    ) -> bool;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn rbfind(_: *mut rbtree, _: *mut libc::c_void) -> *mut rbnode;
    fn rbinsert(_: *mut rbtree, _: *mut libc::c_void, _: *mut *mut rbnode) -> libc::c_int;
    fn rbcreate(
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    ) -> *mut rbtree;
    fn rbdestroy(_: *mut rbtree, _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>);
    fn sudo_lbuf_init_v1(
        lbuf: *mut sudo_lbuf,
        output: sudo_lbuf_output_t,
        indent: libc::c_int,
        continuation: *const libc::c_char,
        cols: libc::c_int,
    );
    fn sudo_lbuf_destroy_v1(lbuf: *mut sudo_lbuf);
    fn sudo_lbuf_append_v1(lbuf: *mut sudo_lbuf, fmt: *const libc::c_char, _: ...) -> bool;
}
pub const DEFAULTS: libc::c_int = 265;
pub const IMPLIED: libc::c_int = 2;
pub const SUPPRESS_DEFAULTS: libc::c_int = 0x01;
pub const SUPPRESS_PRIVS: libc::c_int = 0x04;
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
pub struct sudo_lbuf {
    pub output: Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub buf: *mut libc::c_char,
    pub continuation: *const libc::c_char,
    pub indent: libc::c_int,
    pub len: libc::c_int,
    pub size: libc::c_int,
    pub cols: libc::c_short,
    pub error: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvtsudoers_config {
    pub sudo_order: libc::c_uint,
    pub order_increment: libc::c_uint,
    pub order_padding: libc::c_uint,
    pub order_max: libc::c_uint,
    pub defaults: libc::c_short,
    pub suppress: libc::c_short,
    pub expand_aliases: bool,
    pub store_options: bool,
    pub prune_matches: bool,
    pub sudoers_base: *mut libc::c_char,
    pub input_format: *mut libc::c_char,
    pub output_format: *mut libc::c_char,
    pub filter: *mut libc::c_char,
    pub defstr: *mut libc::c_char,
    pub supstr: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seen_user {
    pub name: *const libc::c_char,
    pub count: libc::c_ulong,
}
pub type sudo_lbuf_output_t = Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>;
static mut seen_users: *mut rbtree = 0 as *const rbtree as *mut rbtree;










