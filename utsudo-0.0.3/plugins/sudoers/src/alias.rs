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
    unreachable_code,
    non_snake_case,
    unreachable_patterns,
    unused_variables,
    clashing_extern_declarations
)]


use crate::common::*;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn rbfind(tree: *mut rbtree, key: *mut libc::c_void) -> *mut rbnode;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn rcstr_addref(s: *const libc::c_char) -> *mut libc::c_char;
    fn rbinsert(_: *mut rbtree, _: *mut libc::c_void, _: *mut *mut rbnode) -> libc::c_int;
    fn sudo_snprintf(
        str0: *mut libc::c_char,
        n: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn rbapply_node(
        _: *mut rbtree,
        _: *mut rbnode,
        _: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
        _: rbtraversal,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn rcstr_delref(s: *const libc::c_char);
    fn free_members(members: *mut member_list);
    fn rbdestroy(_: *mut rbtree, _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>);
    fn rbdelete(_: *mut rbtree, _: *mut rbnode) -> *mut libc::c_void;
    fn rbcreate(
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    ) -> *mut rbtree;
}


pub const ENOENT: libc::c_int = 2;
pub const ELOOP: libc::c_int = 40;
pub type size_t = libc::c_ulong;
pub type time_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_parse_tree {
    pub userspecs: userspec_list,
    pub defaults: defaults_list,
    pub aliases: *mut rbtree,
    pub shost: *mut libc::c_char,
    pub lhost: *mut libc::c_char,
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
    pub entries: TAILQ_ENTRY_userspec,
    pub users: member_list,
    pub privileges: privilege_list,
    pub comments: comment_list,
    pub lineno: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_userspec {
    pub tqe_next: *mut userspec,
    pub tqe_prev: *mut *mut userspec,
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
    pub entries: TAILQ_ENTRY_sudoers_comment,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_sudoers_comment {
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
    pub entries: TAILQ_ENTRY_privilege,
    pub ldap_role: *mut libc::c_char,
    pub hostlist: member_list,
    pub cmndlist: cmndspec_list,
    pub defaults: defaults_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_privilege {
    pub tqe_next: *mut privilege,
    pub tqe_prev: *mut *mut privilege,
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
    pub entries: TAILQ_ENTRY_defaults,
    pub var: *mut libc::c_char,
    pub val: *mut libc::c_char,
    pub binding: *mut member_list,
    pub file: *mut libc::c_char,
    pub type0: libc::c_short,
    pub op: libc::c_char,
    pub error: libc::c_char,
    pub lineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_defaults {
    pub tqe_next: *mut defaults,
    pub tqe_prev: *mut *mut defaults,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub compar:
        Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    pub root: rbnode,
    pub nil: rbnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub data: *mut libc::c_void,
    pub color: rbcolor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum rbcolor {
    red,
    black,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum rbtraversal {
    preorder,
    inorder,
    postorder,
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
    pub entries: TAILQ_ENTRY_cmndspec,
    pub runasuserlist: *mut member_list,
    pub runasgrouplist: *mut member_list,
    pub cmnd: *mut member,
    pub tags: cmndtag,
    pub timeout: libc::c_int,
    pub notbefore: time_t,
    pub notafter: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_cmndspec {
    pub tqe_next: *mut cmndspec,
    pub tqe_prev: *mut *mut cmndspec,
}
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
#[macro_export]
macro_rules! errno {
    () => {
        (*__errno_location())
    };
}
#[macro_export]
macro_rules! TAILQ_FIRST {
    ($head:expr) => {
        (($head).tqh_first)
    };
}
#[macro_export]
macro_rules! rbapply {
    ($t:expr, $f:expr, $c:expr, $o:expr) => {
        (rbapply_node(($t), (*($t)).root.left, ($f), ($c), ($o)))
    };
}
#[macro_export]
macro_rules! rbisempty {
    ($t:expr) => {
        ((*$t).root.left == &mut (*$t).nil as *mut rbnode
            && (*$t).root.right == &mut (*$t).nil as *mut rbnode)
    };
}





