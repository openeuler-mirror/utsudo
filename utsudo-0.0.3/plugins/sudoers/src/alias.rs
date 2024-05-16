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
