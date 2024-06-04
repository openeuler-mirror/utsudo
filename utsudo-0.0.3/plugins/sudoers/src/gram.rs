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

#[no_mangle]
pub static mut sudoers_warnings: bool = 1 as libc::c_int != 0;
#[no_mangle]
pub static mut parse_error: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub static mut errorlineno: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut errorfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut parsed_policy: sudoers_parse_tree = sudoers_parse_tree {
    userspecs: userspec_list {
        tqh_first: 0 as *const userspec as *mut userspec,
        tqh_last: 0 as *const *mut userspec as *mut *mut userspec,
    },
    defaults: defaults_list {
        tqh_first: 0 as *const defaults as *mut defaults,
        tqh_last: 0 as *const *mut defaults as *mut *mut defaults,
    },
    aliases: 0 as *const rbtree as *mut rbtree,
    shost: 0 as *const libc::c_char,
    lhost: 0 as *const libc::c_char,
};
#[no_mangle]
pub static mut sudoerslhs: [libc::c_short; 117] = [
    -1, 0, 0, 32, 32, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 4, 4, 3, 3, 3, 3, 3, 21, 21,
    20, 11, 11, 9, 9, 9, 9, 9, 2, 2, 1, 31, 31, 31, 31, 7, 7, 6, 6, 28, 29, 30, 24, 25, 26, 27, 18,
    18, 19, 19, 19, 19, 19, 23, 23, 23, 23, 23, 23, 23, 23, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
    22, 22, 22, 22, 22, 5, 5, 5, 35, 35, 38, 10, 10, 36, 36, 39, 8, 8, 37, 37, 40, 34, 34, 41, 14,
    14, 12, 12, 13, 13, 13, 13, 13, 17, 17, 15, 15, 16, 16, 16,
];
#[no_mangle]
pub static mut sudoerslen: [libc::c_short; 117] = [
    2, 0, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 1, 3, 1, 2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 1, 1,
    1, 1, 1, 3, 4, 3, 3, 3, 3, 1, 2, 1, 2, 3, 3, 3, 3, 3, 3, 3, 0, 3, 0, 1, 3, 2, 1, 0, 2, 2, 2, 2,
    2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 3, 3, 1, 3, 1, 3, 3, 1, 3, 1,
    3, 3, 1, 3, 3, 1, 3, 1, 2, 1, 1, 1, 1, 1, 1, 3, 1, 2, 1, 1, 1,
];
#[no_mangle]
pub static mut sudoersdefred: [libc::c_short; 186] = [
    0, 0, 105, 107, 108, 109, 0, 0, 0, 0, 0, 106, 5, 0, 0, 0, 0, 0, 0, 101, 103, 0, 0, 3, 6, 0, 0,
    17, 0, 29, 32, 31, 33, 30, 0, 27, 0, 88, 0, 0, 84, 83, 82, 0, 0, 0, 0, 0, 43, 41, 93, 0, 0, 0,
    0, 85, 0, 0, 90, 0, 0, 98, 0, 0, 95, 104, 0, 0, 24, 0, 4, 0, 0, 0, 20, 0, 28, 0, 0, 0, 0, 44,
    0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 102, 0, 0, 21, 22, 23, 18, 89, 37, 38, 39, 40,
    94, 0, 86, 0, 91, 0, 99, 0, 96, 0, 34, 0, 59, 25, 0, 0, 0, 0, 0, 114, 116, 115, 0, 110, 112, 0,
    0, 53, 35, 0, 0, 0, 0, 0, 0, 0, 0, 63, 64, 65, 66, 62, 60, 61, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 80, 81, 78, 79, 36, 111, 49, 48, 50, 51, 45, 46, 47,
];
#[no_mangle]
pub static mut sudoersdgoto: [libc::c_short; 42] = [
    18, 119, 120, 27, 28, 48, 49, 50, 51, 35, 67, 37, 19, 20, 21, 132, 133, 134, 121, 125, 68, 69,
    145, 127, 146, 147, 148, 149, 150, 151, 152, 52, 22, 23, 60, 54, 57, 63, 55, 58, 64, 61,
];
#[no_mangle]
pub static mut sudoerssindex: [libc::c_short; 186] = [
    512, -272, 0, 0, 0, 0, -23, 227, -19, -19, -5, 0, 0, -239, -236, -234, -232, -231, 0, 0, 0,
    -33, 512, 0, 0, -3, -220, 0, 3, 0, 0, 0, 0, 0, -225, 0, -28, 0, -24, -24, 0, 0, 0, -240, -15,
    -8, 2, 4, 0, 0, 0, -21, -12, -9, 6, 0, 7, 12, 0, 10, 14, 0, 13, 25, 0, 0, -19, -36, 0, 26, 0,
    -208, -202, -198, 0, -23, 0, 227, 3, 3, 3, 0, -179, -178, -174, -173, -5, 3, 0, 227, -239, -5,
    -236, -19, -234, -19, -232, 0, 52, 227, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 51, 0, 54, 0, 54,
    0, -29, 0, 55, 0, 0, 289, -7, 59, 52, -216, 0, 0, 0, -217, 0, 0, 57, 289, 0, 0, 32, 41, 42, 43,
    44, 45, 47, 450, 0, 0, 0, 0, 0, 0, 0, 0, 289, 57, -154, -153, -150, -149, -148, -147, -146, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]
pub static mut sudoersrindex: [libc::c_short; 186] = [
    118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 1, 0, 0, 145, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159, 0, 0, 193, 0, 0,
    207, 0, 0, 241, 0, 0, 0, 0, 0, 275, 0, 0, 0, 0, 0, 0, 0, 0, 309, 323, 357, 0, 0, 0, 0, 0, 0,
    371, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 404, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 49, 0, 63, 0,
    97, 0, 79, 0, 111, 0, 0, 81, 82, 0, 404, 483, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]
pub static mut sudoersgindex: [libc::c_short; 42] = [
    0, 5, 0, 53, 18, 86, 74, -79, 36, 98, -1, 56, 68, 120, -6, -18, 8, 11, 0, 0, 39, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 113, 0, 0, 0, 0, 58, 48, 46, 60,
];
#[no_mangle]
pub static mut sudoerstable: [libc::c_short; 802] = [
    34, 19, 38, 39, 17, 26, 36, 109, 77, 26, 26, 66, 26, 24, 17, 87, 77, 40, 41, 53, 66, 43, 56,
    86, 59, 98, 62, 2, 43, 123, 3, 4, 5, 29, 19, 30, 31, 66, 32, 74, 72, 128, 73, 82, 42, 19, 129,
    75, 87, 92, 83, 135, 89, 11, 78, 100, 79, 80, 71, 33, 84, 101, 85, 100, 90, 102, 177, 130, 91,
    87, 92, 93, 94, 87, 95, 138, 139, 140, 141, 142, 143, 144, 92, 96, 99, 105, 106, 114, 110, 116,
    107, 108, 118, 156, 77, 86, 100, 97, 66, 126, 136, 154, 157, 158, 159, 160, 161, 92, 162, 179,
    180, 26, 124, 181, 182, 183, 184, 185, 1, 2, 54, 100, 58, 55, 57, 56, 88, 112, 103, 81, 97,
    137, 76, 104, 97, 70, 178, 65, 122, 153, 113, 0, 117, 0, 26, 12, 155, 0, 111, 0, 0, 0, 0, 0,
    115, 97, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 10, 30, 31, 2, 32, 25, 3, 4, 5, 25, 25, 0, 25, 2, 8, 11, 3,
    4, 5, 40, 41, 0, 0, 0, 0, 33, 40, 41, 0, 11, 0, 19, 0, 19, 34, 0, 19, 19, 19, 11, 19, 19, 19,
    19, 19, 87, 42, 87, 11, 7, 87, 87, 87, 42, 87, 87, 87, 87, 87, 19, 19, 19, 19, 19, 19, 0, 0, 0,
    44, 45, 46, 47, 0, 87, 87, 87, 87, 87, 87, 92, 0, 92, 7, 15, 92, 92, 92, 0, 92, 92, 92, 92, 92,
    100, 0, 100, 131, 13, 100, 100, 100, 0, 100, 100, 100, 100, 100, 92, 92, 92, 92, 92, 92, 0, 0,
    0, 15, 0, 0, 0, 0, 100, 100, 100, 100, 100, 100, 97, 0, 97, 13, 14, 97, 97, 97, 0, 97, 97, 97,
    97, 97, 26, 0, 26, 0, 16, 26, 26, 26, 0, 26, 26, 26, 26, 26, 97, 97, 97, 97, 97, 97, 0, 0, 0,
    14, 0, 0, 0, 0, 26, 26, 26, 26, 26, 26, 12, 0, 12, 16, 0, 12, 12, 12, 0, 12, 12, 12, 12, 12, 9,
    0, 9, 0, 0, 9, 9, 9, 0, 9, 9, 9, 9, 9, 12, 12, 12, 12, 12, 12, 0, 0, 52, 0, 0, 0, 0, 0, 9, 9,
    9, 9, 9, 9, 10, 0, 10, 0, 0, 10, 10, 10, 0, 10, 10, 10, 10, 10, 8, 0, 8, 0, 0, 8, 8, 8, 0, 8,
    8, 8, 8, 8, 10, 10, 10, 10, 10, 10, 43, 0, 29, 0, 30, 31, 0, 32, 8, 8, 8, 8, 8, 8, 11, 0, 11,
    0, 0, 11, 11, 11, 0, 11, 11, 11, 11, 11, 33, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 11, 11,
    11, 11, 11, 11, 7, 0, 7, 0, 0, 7, 7, 7, 0, 7, 7, 7, 7, 7, 17, 0, 128, 0, 0, 0, 0, 129, 0, 0, 0,
    0, 0, 0, 7, 7, 7, 7, 7, 7, 15, 0, 15, 0, 0, 15, 15, 15, 130, 15, 15, 15, 15, 15, 13, 0, 13, 0,
    0, 13, 13, 13, 0, 13, 13, 13, 13, 13, 15, 15, 15, 15, 15, 15, 0, 0, 0, 0, 0, 0, 0, 0, 13, 13,
    13, 13, 13, 13, 14, 0, 14, 0, 0, 14, 14, 14, 0, 14, 14, 14, 14, 14, 16, 0, 16, 0, 0, 16, 16,
    16, 0, 16, 16, 16, 16, 16, 14, 14, 14, 14, 14, 14, 0, 0, 0, 0, 0, 0, 0, 0, 16, 16, 16, 16, 16,
    16, 52, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
    52, 52, 52, 0, 0, 0, 0, 0, 0, 52, 52, 52, 52, 52, 52, 52, 0, 52, 52, 52, 52, 40, 41, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
    42, 0, 0, 0, 0, 0, 67, 67, 0, 0, 0, 0, 0, 0, 0, 44, 45, 46, 47, 67, 67, 67, 67, 67, 67, 67, 67,
    67, 67, 67, 67, 67, 67, 67, 1, 0, 2, 0, 0, 3, 4, 5, 0, 6, 7, 8, 9, 10, 67, 67, 67, 67, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 11, 12, 13, 14, 15, 16,
];
#[no_mangle]
pub static mut sudoerscheck: [libc::c_short; 802] = [
    33, 0, 8, 9, 33, 33, 7, 86, 44, 33, 33, 44, 33, 285, 33, 0, 44, 257, 258, 258, 44, 33, 258, 44,
    258, 61, 258, 258, 33, 58, 261, 262, 263, 258, 33, 260, 261, 44, 263, 259, 43, 258, 45, 58,
    284, 44, 263, 44, 33, 0, 58, 58, 61, 284, 36, 263, 38, 39, 61, 284, 58, 263, 58, 0, 58, 263,
    145, 284, 61, 51, 58, 61, 58, 58, 61, 291, 292, 293, 294, 295, 296, 297, 33, 58, 58, 264, 264,
    93, 89, 95, 264, 264, 40, 61, 44, 44, 33, 0, 44, 44, 41, 44, 61, 61, 61, 61, 61, 58, 61, 263,
    263, 0, 118, 263, 263, 263, 263, 263, 0, 0, 41, 58, 41, 41, 41, 41, 52, 91, 75, 43, 33, 126,
    34, 77, 66, 22, 154, 17, 99, 131, 92, -1, 96, -1, 33, 0, 135, -1, 90, -1, -1, -1, -1, -1, 94,
    58, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, 58, -1, -1, -1, -1, -1, -1, -1, -1, 33,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 33, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 258, 33,
    260, 261, 258, 263, 259, 261, 262, 263, 259, 259, -1, 259, 258, 33, 0, 261, 262, 263, 257, 258,
    -1, -1, -1, -1, 284, 257, 258, -1, 284, -1, 256, -1, 258, 33, -1, 261, 262, 263, 284, 265, 266,
    267, 268, 269, 256, 284, 258, 33, 0, 261, 262, 263, 284, 265, 266, 267, 268, 269, 284, 285,
    286, 287, 288, 289, -1, -1, -1, 299, 300, 301, 302, -1, 284, 285, 286, 287, 288, 289, 256, -1,
    258, 33, 0, 261, 262, 263, -1, 265, 266, 267, 268, 269, 256, -1, 258, 33, 0, 261, 262, 263, -1,
    265, 266, 267, 268, 269, 284, 285, 286, 287, 288, 289, -1, -1, -1, 33, -1, -1, -1, -1, 284,
    285, 286, 287, 288, 289, 256, -1, 258, 33, 0, 261, 262, 263, -1, 265, 266, 267, 268, 269, 256,
    -1, 258, -1, 0, 261, 262, 263, -1, 265, 266, 267, 268, 269, 284, 285, 286, 287, 288, 289, -1,
    -1, -1, 33, -1, -1, -1, -1, 284, 285, 286, 287, 288, 289, 256, -1, 258, 33, -1, 261, 262, 263,
    -1, 265, 266, 267, 268, 269, 256, -1, 258, -1, -1, 261, 262, 263, -1, 265, 266, 267, 268, 269,
    284, 285, 286, 287, 288, 289, -1, -1, 33, -1, -1, -1, -1, -1, 284, 285, 286, 287, 288, 289,
    256, -1, 258, -1, -1, 261, 262, 263, -1, 265, 266, 267, 268, 269, 256, -1, 258, -1, -1, 261,
    262, 263, -1, 265, 266, 267, 268, 269, 284, 285, 286, 287, 288, 289, 33, -1, 258, -1, 260, 261,
    -1, 263, 284, 285, 286, 287, 288, 289, 256, -1, 258, -1, -1, 261, 262, 263, -1, 265, 266, 267,
    268, 269, 284, -1, -1, -1, -1, 33, -1, -1, -1, -1, -1, -1, -1, -1, 284, 285, 286, 287, 288,
    289, 256, -1, 258, -1, -1, 261, 262, 263, -1, 265, 266, 267, 268, 269, 33, -1, 258, -1, -1, -1,
    -1, 263, -1, -1, -1, -1, -1, -1, 284, 285, 286, 287, 288, 289, 256, -1, 258, -1, -1, 261, 262,
    263, 284, 265, 266, 267, 268, 269, 256, -1, 258, -1, -1, 261, 262, 263, -1, 265, 266, 267, 268,
    269, 284, 285, 286, 287, 288, 289, -1, -1, -1, -1, -1, -1, -1, -1, 284, 285, 286, 287, 288,
    289, 256, -1, 258, -1, -1, 261, 262, 263, -1, 265, 266, 267, 268, 269, 256, -1, 258, -1, -1,
    261, 262, 263, -1, 265, 266, 267, 268, 269, 284, 285, 286, 287, 288, 289, -1, -1, -1, -1, -1,
    -1, -1, -1, 284, 285, 286, 287, 288, 289, 257, 258, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, -1, -1, -1, -1, -1,
    -1, 291, 292, 293, 294, 295, 296, 297, -1, 299, 300, 301, 302, 257, 258, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283,
    284, -1, -1, -1, -1, -1, 257, 258, -1, -1, -1, -1, -1, -1, -1, 299, 300, 301, 302, 270, 271,
    272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 256, -1, 258, -1, -1, 261,
    262, 263, -1, 265, 266, 267, 268, 269, 299, 300, 301, 302, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 284, 285, 286, 287, 288, 289,
];
#[no_mangle]
pub static mut sudoersdebug: libc::c_int = 0;
#[no_mangle]
pub static mut sudoersnerrs: libc::c_int = 0;
#[no_mangle]
pub static mut sudoerserrflag: libc::c_int = 0;
#[no_mangle]
pub static mut sudoerschar: libc::c_int = 0;
#[no_mangle]
pub static mut sudoersssp: *mut libc::c_short = 0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub static mut sudoersvsp: *mut YYSTYPE = 0 as *const YYSTYPE as *mut YYSTYPE;
#[no_mangle]
pub static mut sudoersval: YYSTYPE = YYSTYPE {
    cmndspec: 0 as *const cmndspec as *mut cmndspec,
};
#[no_mangle]
pub static mut sudoerslval: YYSTYPE = YYSTYPE {
    cmndspec: 0 as *const cmndspec as *mut cmndspec,
};
#[no_mangle]
pub static mut sudoersss: *mut libc::c_short = 0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub static mut sudoerssslim: *mut libc::c_short = 0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub static mut sudoersvs: *mut YYSTYPE = 0 as *const YYSTYPE as *mut YYSTYPE;
#[no_mangle]
pub static mut sudoersstacksize: libc::c_uint = 0;
// #define COMMENT 285
pub const COMMENT: libc::c_int = 285;
pub const COMMAND: libc::c_int = 257;
pub const UNSPEC: libc::c_int = -1;
/* If we last saw a newline the entry is on the preceding line. */
#[macro_export]
macro_rules! this_lineno {
    () => {
        if last_token == COMMENT {
            sudolineno - 1 as libc::c_int
        } else {
            sudolineno
        }
    };
}
// LEXTRACE
#[macro_export]
macro_rules! LEXTRACE {
    ($arg:expr) => {
        if trace_print.is_some() {
            (Some(trace_print.expect("non-null function pointer")))
                .expect("non-null function pointer")($arg);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sudoerserror(mut s: *const libc::c_char) {
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    /* Save the line the first error occurred on. */
    if errorlineno == -(1 as libc::c_int) {
        errorlineno = this_lineno!();
        rcstr_delref(errorfile);
        errorfile = rcstr_addref(sudoers);
    }
    if sudoers_warnings as libc::c_int != 0 && !s.is_null() {
        LEXTRACE!(b"<*> \0" as *const u8 as *const libc::c_char);
        if trace_print.is_none()
            || trace_print
                == Some(
                    sudoers_trace_print as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
                )
        {
            let fmt: [libc::c_char; 29] = *::core::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b">>> %s: %s near line %d <<<\n\0",
            );
            let mut oldlocale: libc::c_int = 0;
            /* Warnings are displayed in the user's locale. */
            sudoers_setlocale(0 as libc::c_int, &mut oldlocale);
            sudo_printf.expect("non-null function pointer")(
                SUDO_CONV_ERROR_MSG,
                fmt.as_ptr(),
                sudoers,
                s,
                this_lineno!(),
            );
            sudoers_setlocale(oldlocale, 0 as *mut libc::c_int);
        }
    }
    parse_error = true;
    debug_return!();
}
unsafe extern "C" fn new_default(
    mut var: *mut libc::c_char,
    mut val: *mut libc::c_char,
    mut op: libc::c_short,
) -> *mut defaults {
    let mut d: *mut defaults = 0 as *mut defaults;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    d = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<defaults>() as libc::c_ulong,
    ) as *mut defaults;
    if d.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_ptr!(0 as *mut defaults);
    }
    (*d).var = var;
    (*d).val = val;
    /* d->type = 0; */
    (*d).op = op as libc::c_char;
    /* d->binding = NULL */
    (*d).lineno = this_lineno!();
    (*d).file = rcstr_addref(sudoers);
    (*d).entries.tqe_next = 0 as *mut defaults;
    (*d).entries.tqe_prev = &mut (*d).entries.tqe_next;
    debug_return_ptr!(d);
}
unsafe extern "C" fn new_member(
    mut name: *mut libc::c_char,
    mut type_0: libc::c_int,
) -> *mut member {
    let mut m: *mut member = 0 as *mut member;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    m = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<member>() as libc::c_ulong,
    ) as *mut member;
    if m.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_ptr!(0 as *mut member);
    }
    (*m).name = name;
    (*m).type_0 = type_0 as libc::c_short;
    (*m).entries.tqe_next = 0 as *mut member;
    (*m).entries.tqe_prev = &mut (*m).entries.tqe_next;
    debug_return_ptr!(m);
}
unsafe extern "C" fn new_digest(
    mut digest_type: libc::c_int,
    mut digest_str: *mut libc::c_char,
) -> *mut command_digest {
    let mut digest: *mut command_digest = 0 as *mut command_digest;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    digest =
        malloc(::core::mem::size_of::<command_digest>() as libc::c_ulong) as *mut command_digest;
    if digest.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_ptr!(0 as *mut command_digest);
    }
    (*digest).digest_type = digest_type as libc::c_uint;
    (*digest).digest_str = digest_str;
    if ((*digest).digest_str).is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        free(digest as *mut libc::c_void);
        digest = 0 as *mut command_digest;
    }
    debug_return_ptr!(digest);
}

/*
 * Add a list of defaults structures to the defaults list.
 * The binding, if non-NULL, specifies a list of hosts, users, or
 * runas users the entries apply to (specified by the type).
 */
unsafe extern "C" fn add_defaults(
    mut type_0: libc::c_int,
    mut bmem: *mut member,
    mut defs: *mut defaults,
) -> bool {
    let mut d: *mut defaults = 0 as *mut defaults;
    let mut next: *mut defaults = 0 as *mut defaults;
    let mut binding: *mut member_list = 0 as *mut member_list;
    let mut ret: bool = true;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    if !defs.is_null() {
        /*
         * We use a single binding for each entry in defs.
         */
        binding =
            malloc(::core::mem::size_of::<member_list>() as libc::c_ulong) as *mut member_list;
        if binding.is_null() {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            sudoerserror(b"unable to allocate memory\0" as *const u8 as *const libc::c_char);
            debug_return_bool!(false);
        }
        if !bmem.is_null() {
            (*binding).tqh_first = bmem;
            (*binding).tqh_last = (*bmem).entries.tqe_prev;
            (*bmem).entries.tqe_prev = &mut (*binding).tqh_first;
        } else {
            (*binding).tqh_first = 0 as *mut member;
            (*binding).tqh_last = &mut (*binding).tqh_first;
        }
        /*
         * Set type and binding (who it applies to) for new entries.
         * Then add to the global defaults list.
         */
        d = defs;
        while !d.is_null() && {
            next = (*d).entries.tqe_next;
            1 as libc::c_int != 0
        } {
            (*d).type_0 = type_0 as libc::c_short;
            (*d).binding = binding;
            (*d).entries.tqe_next = 0 as *mut defaults;
            (*d).entries.tqe_prev = parsed_policy.defaults.tqh_last;
            *parsed_policy.defaults.tqh_last = d;
            parsed_policy.defaults.tqh_last = &mut (*d).entries.tqe_next;
            d = next;
        }
    }
    debug_return_bool!(ret);
}
/*
 * Allocate a new struct userspec, populate it, and insert it at the
 * end of the userspecs list.
 */
unsafe extern "C" fn add_userspec(mut members: *mut member, mut privs: *mut privilege) -> bool {
    let mut u: *mut userspec = 0 as *mut userspec;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    u = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<userspec>() as libc::c_ulong,
    ) as *mut userspec;
    if u.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    (*u).lineno = this_lineno!();
    (*u).file = rcstr_addref(sudoers);
    (*u).users.tqh_first = members;
    (*u).users.tqh_last = (*members).entries.tqe_prev;
    (*members).entries.tqe_prev = &mut (*u).users.tqh_first;
    (*u).privileges.tqh_first = privs;
    (*u).privileges.tqh_last = (*privs).entries.tqe_prev;
    (*privs).entries.tqe_prev = &mut (*u).privileges.tqh_first;
    (*u).comments.stqh_first = 0 as *mut sudoers_comment;
    (*u).comments.stqh_last = &mut (*u).comments.stqh_first;
    (*u).entries.tqe_next = 0 as *mut userspec;
    (*u).entries.tqe_prev = parsed_policy.userspecs.tqh_last;
    *parsed_policy.userspecs.tqh_last = u;
    parsed_policy.userspecs.tqh_last = &mut (*u).entries.tqe_next;
    debug_return_bool!(true);
}
/*
 * Free a member struct and its contents.
 */
#[no_mangle]
pub unsafe extern "C" fn free_member(mut m: *mut member) {
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    if (*m).type_0 as libc::c_int == COMMAND {
        let mut c: *mut sudo_command = (*m).name as *mut sudo_command;
        free((*c).cmnd as *mut libc::c_void);
        free((*c).args as *mut libc::c_void);
        if !((*c).digest).is_null() {
            free((*(*c).digest).digest_str as *mut libc::c_void);
            free((*c).digest as *mut libc::c_void);
        }
    }
    free((*m).name as *mut libc::c_void);
    free(m as *mut libc::c_void);
    debug_return!();
}
/*
 * Free a tailq of members but not the struct member_list container itself.
 */
#[no_mangle]
pub unsafe extern "C" fn free_members(mut members: *mut member_list) {
    let mut m: *mut member = 0 as *mut member;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    loop {
        m = (*members).tqh_first;
        if m.is_null() {
            break;
        }
        if !((*m).entries.tqe_next).is_null() {
            (*(*m).entries.tqe_next).entries.tqe_prev = (*m).entries.tqe_prev;
        } else {
            (*members).tqh_last = (*m).entries.tqe_prev;
        }
        *(*m).entries.tqe_prev = (*m).entries.tqe_next;
        free_member(m);
    }
    debug_return!();
}
#[no_mangle]
pub unsafe extern "C" fn free_defaults(mut defs: *mut defaults_list) {
    let mut prev_binding: *mut member_list = 0 as *mut member_list;
    let mut def: *mut defaults = 0 as *mut defaults;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    loop {
        def = (*defs).tqh_first;
        if def.is_null() {
            break;
        }
        if !((*def).entries.tqe_next).is_null() {
            (*(*def).entries.tqe_next).entries.tqe_prev = (*def).entries.tqe_prev;
        } else {
            (*defs).tqh_last = (*def).entries.tqe_prev;
        }
        *(*def).entries.tqe_prev = (*def).entries.tqe_next;
        free_default(def, &mut prev_binding);
    }
    debug_return!();
}
#[no_mangle]
pub unsafe extern "C" fn free_default(mut def: *mut defaults, mut binding: *mut *mut member_list) {
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    if (*def).binding != *binding {
        *binding = (*def).binding;
        if !((*def).binding).is_null() {
            free_members((*def).binding);
            free((*def).binding as *mut libc::c_void);
        }
    }
    rcstr_delref((*def).file);
    free((*def).var as *mut libc::c_void);
    free((*def).val as *mut libc::c_void);
    free(def as *mut libc::c_void);
    debug_return!();
}
#[no_mangle]
pub unsafe extern "C" fn free_privilege(mut priv_0: *mut privilege) {
    let mut runasuserlist: *mut member_list = 0 as *mut member_list;
    let mut runasgrouplist: *mut member_list = 0 as *mut member_list;
    let mut prev_binding: *mut member_list = 0 as *mut member_list;
    let mut cs: *mut cmndspec = 0 as *mut cmndspec;
    let mut def: *mut defaults = 0 as *mut defaults;
    let mut role: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    free((*priv_0).ldap_role as *mut libc::c_void);
    free_members(&mut (*priv_0).hostlist);
    loop {
        cs = (*priv_0).cmndlist.tqh_first;
        if cs.is_null() {
            break;
        }
        if !((*cs).entries.tqe_next).is_null() {
            (*(*cs).entries.tqe_next).entries.tqe_prev = (*cs).entries.tqe_prev;
        } else {
            (*priv_0).cmndlist.tqh_last = (*cs).entries.tqe_prev;
        }
        *(*cs).entries.tqe_prev = (*cs).entries.tqe_next;
        /* Only free the first instance of a role/type. */
        if (*cs).role != role {
            role = (*cs).role;
            free((*cs).role as *mut libc::c_void);
        }
        if (*cs).type_0 != type_0 {
            type_0 = (*cs).type_0;
            free((*cs).type_0 as *mut libc::c_void);
        }
        /* Only free the first instance of runas user/group lists. */
        if !((*cs).runasuserlist).is_null() && (*cs).runasuserlist != runasuserlist {
            runasuserlist = (*cs).runasuserlist;
            free_members(runasuserlist);
            free(runasuserlist as *mut libc::c_void);
        }
        if !((*cs).runasgrouplist).is_null() && (*cs).runasgrouplist != runasgrouplist {
            runasgrouplist = (*cs).runasgrouplist;
            free_members(runasgrouplist);
            free(runasgrouplist as *mut libc::c_void);
        }
        free_member((*cs).cmnd);
        free(cs as *mut libc::c_void);
    }
    loop {
        def = (*priv_0).defaults.tqh_first;
        if def.is_null() {
            break;
        }
        if !((*def).entries.tqe_next).is_null() {
            (*(*def).entries.tqe_next).entries.tqe_prev = (*def).entries.tqe_prev;
        } else {
            (*priv_0).defaults.tqh_last = (*def).entries.tqe_prev;
        }
        *(*def).entries.tqe_prev = (*def).entries.tqe_next;
        free_default(def, &mut prev_binding);
    }
    free(priv_0 as *mut libc::c_void);
    debug_return!();
}

#[no_mangle]
pub unsafe extern "C" fn free_userspecs(mut usl: *mut userspec_list) {
    let mut us: *mut userspec = 0 as *mut userspec;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    loop {
        us = (*usl).tqh_first;
        if us.is_null() {
            break;
        }
        if !((*us).entries.tqe_next).is_null() {
            (*(*us).entries.tqe_next).entries.tqe_prev = (*us).entries.tqe_prev;
        } else {
            (*usl).tqh_last = (*us).entries.tqe_prev;
        }
        *(*us).entries.tqe_prev = (*us).entries.tqe_next;
        free_userspec(us);
    }
    debug_return!();
}
#[no_mangle]
pub unsafe extern "C" fn free_userspec(mut us: *mut userspec) {
    let mut priv_0: *mut privilege = 0 as *mut privilege;
    let mut comment: *mut sudoers_comment = 0 as *mut sudoers_comment;
    debug_decl!(SUDOERS_DEBUG_PARSER!());
    free_members(&mut (*us).users);
    loop {
        priv_0 = (*us).privileges.tqh_first;
        if priv_0.is_null() {
            break;
        }
        if !((*priv_0).entries.tqe_next).is_null() {
            (*(*priv_0).entries.tqe_next).entries.tqe_prev = (*priv_0).entries.tqe_prev;
        } else {
            (*us).privileges.tqh_last = (*priv_0).entries.tqe_prev;
        }
        *(*priv_0).entries.tqe_prev = (*priv_0).entries.tqe_next;
        free_privilege(priv_0);
    }
    loop {
        comment = (*us).comments.stqh_first;
        if comment.is_null() {
            break;
        }
        (*us).comments.stqh_first = (*(*us).comments.stqh_first).entries.stqe_next;
        if ((*us).comments.stqh_first).is_null() {
            (*us).comments.stqh_last = &mut (*us).comments.stqh_first;
        }
        free((*comment).str_0 as *mut libc::c_void);
        free(comment as *mut libc::c_void);
    }
    rcstr_delref((*us).file);
    free(us as *mut libc::c_void);
    debug_return!();
}


