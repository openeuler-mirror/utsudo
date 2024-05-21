/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    unused_variables,
    unused_assignments,
    non_camel_case_types,
    unreachable_code,
    bindings_with_variant_name,
    unreachable_patterns,
    unused_mut,
    non_upper_case_globals
)]
#[macro_use]
mod common;
use crate::common::alias as alias_struct;
use crate::common::defaults as defaults_struct;
use crate::common::defaults_list;
use crate::common::member;
use crate::common::member_list;
use crate::common::*;
mod alias;
mod audit;
mod base64;
mod cvtsudoers_json;
mod cvtsudoers_ldif;
mod cvtsudoers_pwutil;
mod defaults;
mod digestname;
mod filedigest;
mod fmtsudoers;
mod gentime;
mod gmtoff;
mod gram;
mod hexchar;
mod ldap_util;
mod linux_audit;
mod locale;
mod logging;
mod logwrap;
mod r#match;
mod match_addr;
mod match_command;
mod match_digest;
mod parse_ldif;
mod pwutil;
mod pwutil_impl;
mod rcstr;
mod redblack;
mod set_perms;
mod strlist;
mod stubs;
mod sudo_printf;
mod sudoers_debug;
mod timeout;
mod timestr;
mod toke;
mod toke_util;
#[link(name = "audit")]
#[link(name = "utsudo_util")]
#[link(name = "util_variadic")]
#[link(name = "plugins_variadic")]
extern "C" {
    static mut parsed_policy: sudoers_parse_tree;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    static mut sudo_defs_table: [sudo_defs_types; 0];
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut opterr: libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut errorfile: *mut libc::c_char;
    static mut parse_error: bool;
    static mut sudoersin: *mut FILE;
    static mut errorlineno: libc::c_int;
    fn alias_apply(
        parse_tree: *mut sudoers_parse_tree,
        func: Option<
            unsafe extern "C" fn(
                *mut sudoers_parse_tree,
                *mut alias_struct,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        cookie: *mut libc::c_void,
    );
    fn free_member(m: *mut member);
    fn alias_free(a: *mut libc::c_void);
    fn alias_remove(
        parse_tree: *mut sudoers_parse_tree,
        name: *mut libc::c_char,
        type_0: libc::c_int,
    ) -> *mut alias_struct;
    fn alias_find_used(parse_tree: *mut sudoers_parse_tree, used_aliases: *mut rbtree) -> bool;
    fn alloc_aliases() -> *mut rbtree;
    fn free_aliases(aliases: *mut rbtree);
    fn free_default(def: *mut defaults_struct, binding: *mut *mut member_list);
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn initprogname(_: *const libc::c_char);
    fn sudoers_initlocale(ulocale: *const libc::c_char, slocale: *const libc::c_char) -> bool;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_warn_set_locale_func_v1(
        func: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool>,
    );
    fn sudo_conf_read_v1(conf_file: *const libc::c_char, conf_types: libc::c_int) -> libc::c_int;
    fn sudoers_debug_register(
        plugin_path: *const libc::c_char,
        debug_files: *mut sudo_conf_debug_file_list,
    ) -> bool;
    fn sudo_getprogname() -> *const libc::c_char;
    fn sudo_conf_debug_files_v1(progname: *const libc::c_char) -> *mut sudo_conf_debug_file_list;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sudo_pwutil_set_backend(
        _: sudo_make_pwitem_t,
        _: sudo_make_gritem_t,
        _: sudo_make_gidlist_item_t,
        _: sudo_make_grlist_item_t,
    );
    fn convert_sudoers_ldif(
        parse_tree: *mut sudoers_parse_tree,
        output_file: *const libc::c_char,
        conf: *mut cvtsudoers_config,
    ) -> bool;
    fn sudoers_format_default_line(
        lbuf: *mut sudo_lbuf,
        parse_tree: *mut sudoers_parse_tree,
        d: *mut defaults_struct,
        next: *mut *mut defaults_struct,
        expand_aliases: bool,
    ) -> bool;
    fn sudo_parseln_v2(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        lineno: *mut libc::c_uint,
        fp: *mut FILE,
        flags: libc::c_int,
    ) -> ssize_t;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn rcstr_dup(src: *const libc::c_char) -> *mut libc::c_char;
    fn cvtsudoers_make_pwitem(uid: uid_t, name: *const libc::c_char) -> *mut cache_item;
    fn cvtsudoers_make_gritem(gid: gid_t, name: *const libc::c_char) -> *mut cache_item;
    fn cvtsudoers_make_gidlist_item(
        pw: *const passwd,
        unused1: *const *mut libc::c_char,
        type0: libc::c_uint,
    ) -> *mut cache_item;
    fn sudoers_warn_setlocale(restore: bool, cookie: *mut libc::c_int) -> bool;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn alias_put(a: *mut alias_struct);
    fn free_privilege(priv_0: *mut privilege);
    fn cvtsudoers_make_grlist_item(
        pw: *const passwd,
        unused1: *const *mut libc::c_char,
    ) -> *mut cache_item;
    fn get_hostname();
    fn init_parser(path: *const libc::c_char, quiet: bool) -> bool;
    fn init_defaults() -> bool;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rcstr_delref(s: *const libc::c_char);
    fn sudo_strtobool_v1(str: *const libc::c_char) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sudo_strtoid_v2(str: *const libc::c_char, errstr: *mut *const libc::c_char) -> id_t;
    fn sudoers_parse_ldif(
        parse_tree: *mut sudoers_parse_tree,
        fp: *mut FILE,
        sudoers_base: *const libc::c_char,
        store_options: bool,
    ) -> bool;
    fn convert_sudoers_json(
        parse_tree: *mut sudoers_parse_tree,
        output_file: *const libc::c_char,
        conf: *mut cvtsudoers_config,
    ) -> bool;
    fn sudo_getpwuid(_: uid_t) -> *mut passwd;
    fn sudo_getpwnam(_: *const libc::c_char) -> *mut passwd;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn sudo_lbuf_append_v1(lbuf: *mut sudo_lbuf, fmt: *const libc::c_char, _: ...) -> bool;
    fn alias_type_to_string(alias_type: libc::c_int) -> *const libc::c_char;
    fn sudoers_format_member(
        lbuf: *mut sudo_lbuf,
        parse_tree: *mut sudoers_parse_tree,
        m: *mut member,
        separator: *const libc::c_char,
        alias_type: libc::c_int,
    ) -> bool;
    fn sudoersparse() -> libc::c_int;
    fn alias_get(
        parse_tree: *mut sudoers_parse_tree,
        name: *const libc::c_char,
        type0: libc::c_int,
    ) -> *mut alias_struct;
    fn sudo_lbuf_init_v1(
        lbuf: *mut sudo_lbuf,
        output: sudo_lbuf_output_t,
        indent: libc::c_int,
        continuation: *const libc::c_char,
        cols: libc::c_int,
    );
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_lbuf_print_v1(lbuf: *mut sudo_lbuf);
    fn sudoers_format_userspecs(
        lbuf: *mut sudo_lbuf,
        parse_tree: *mut sudoers_parse_tree,
        separator: *const libc::c_char,
        expand_aliases: bool,
        flush: bool,
    ) -> bool;
    fn free_userspec(us: *mut userspec);
    fn host_matches(
        parse_tree: *mut sudoers_parse_tree,
        pw: *const passwd,
        host: *const libc::c_char,
        shost: *const libc::c_char,
        m: *const member,
    ) -> libc::c_int;
    fn sudo_lbuf_error_v1(lbuf: *mut sudo_lbuf) -> bool;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_lbuf_destroy_v1(lbuf: *mut sudo_lbuf);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn user_matches(
        parse_tree: *mut sudoers_parse_tree,
        pw: *const passwd,
        m: *const member,
    ) -> libc::c_int;
    fn sudo_pw_delref(_: *mut passwd);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

pub const no_argument: libc::c_int = 0;
pub const required_argument: libc::c_int = 1;
pub const EXIT_FAILURE: libc::c_int = 1;
pub const LC_ALL: libc::c_int = 6;
pub const SUDO_CONF_DEBUG: libc::c_int = 0x01;
pub const SUDO_CONF_PLUGINS: libc::c_int = 0x04;
pub const EXIT_SUCCESS: libc::c_int = 0;
pub const CONF_BOOL: libc::c_int = 0;
pub const CONF_UINT: libc::c_int = 1;
pub const CONF_STR: libc::c_int = 2;
/* Flags for cvtsudoers_config.defaults */
// #define CVT_DEFAULTS_GLOBAL	0x01
// #define CVT_DEFAULTS_USER	0x02
// #define CVT_DEFAULTS_RUNAS	0x04
// #define CVT_DEFAULTS_HOST	0x08
// #define CVT_DEFAULTS_CMND	0x10
// #define CVT_DEFAULTS_ALL	    0xff
pub const CVT_DEFAULTS_GLOBAL: libc::c_int = 0x01;
pub const CVT_DEFAULTS_USER: libc::c_int = 0x02;
pub const CVT_DEFAULTS_RUNAS: libc::c_int = 0x04;
pub const CVT_DEFAULTS_HOST: libc::c_int = 0x08;
pub const CVT_DEFAULTS_CMND: libc::c_int = 0x10;
pub const CVT_DEFAULTS_ALL: libc::c_int = 0xff;
/* Flags for cvtsudoers_config.suppress */
// #define SUPPRESS_DEFAULTS	0x01
// #define SUPPRESS_ALIASES	0x02
// #define SUPPRESS_PRIVS		0x04
pub const SUPPRESS_DEFAULTS: libc::c_int = 0x01;
pub const SUPPRESS_ALIASES: libc::c_int = 0x02;
pub const SUPPRESS_PRIVS: libc::c_int = 0x04;
pub const DEFAULTS: libc::c_int = 265;
pub const DEFAULTS_HOST: libc::c_int = 266;
pub const DEFAULTS_USER: libc::c_int = 267;
pub const DEFAULTS_RUNAS: libc::c_int = 268;
pub const DEFAULTS_CMND: libc::c_int = 269;
pub const ENOMEM: libc::c_int = 12;
pub const LC_MESSAGES: libc::c_int = 5;
pub const __INT_MAX__: libc::c_int = 2147483647;
pub const SUDOERS_GRAMMAR_VERSION: libc::c_int = 46;
pub type sudo_make_pwitem_t =
    Option<unsafe extern "C" fn(uid_t, *const libc::c_char) -> *mut cache_item>;
pub type sudo_make_gritem_t =
    Option<unsafe extern "C" fn(gid_t, *const libc::c_char) -> *mut cache_item>;
pub type sudo_make_gidlist_item_t = Option<
    unsafe extern "C" fn(*const passwd, *const *mut libc::c_char, libc::c_uint) -> *mut cache_item,
>;
pub type sudo_lbuf_output_t = Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>;
pub type sudo_make_grlist_item_t =
    Option<unsafe extern "C" fn(*const passwd, *const *mut libc::c_char) -> *mut cache_item>;
pub type sudoers_formats = libc::c_uint;
pub const format_sudoers: sudoers_formats = 2;
pub const format_ldif: sudoers_formats = 1;
pub const format_json: sudoers_formats = 0;
/*
#[derive(Copy, Clone)]
#[repr(C)]
pub enum sudoers_formats {
    format_json,
    format_ldif,
    format_sudoers,
}
*/
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
pub struct cvtsudoers_filter {
    pub users: sudoers_str_list,
    pub groups: sudoers_str_list,
    pub hosts: sudoers_str_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_str_list {
    pub stqh_first: *mut sudoers_string,
    pub stqh_last: *mut *mut sudoers_string,
    pub refcnt: libc::c_uint,
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
pub struct sudoers_parse_tree {
    pub userspecs: userspec_list,
    pub defaults: defaults_list,
    pub aliases: *mut rbtree,
    pub shost: *const libc::c_char,
    pub lhost: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvtsudoers_conf_table {
    pub conf_str: *const libc::c_char,
    pub type_0: libc::c_int,
    pub valp: *mut libc::c_void,
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
    pub entries: C2RustUnnamed_6,
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
    pub entries: C2RustUnnamed_3,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub stqe_next: *mut sudoers_comment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut userspec,
    pub tqe_prev: *mut *mut userspec,
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
    pub entries: C2RustUnnamed_5,
    pub ldap_role: *mut libc::c_char,
    pub hostlist: member_list,
    pub cmndlist: cmndspec_list,
    pub defaults: defaults_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut privilege,
    pub tqe_prev: *mut *mut privilege,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmndspec_list {
    pub tqh_first: *mut cmndspec,
    pub tqh_last: *mut *mut cmndspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_string {
    pub entries: C2RustUnnamed_8,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub stqe_next: *mut sudoers_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmndspec {
    pub entries: C2RustUnnamed_4,
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
pub struct C2RustUnnamed_4 {
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
macro_rules! _PATH_CVTSUDOERS_CONF {
    () => {
        (b"/etc/cvtutsudoers.conf\0" as *const u8 as *const libc::c_char)
    };
}
#[macro_export]
macro_rules! LOCALEDIR {
    () => {
        (b"/usr/share/locale\0" as *const u8 as *const libc::c_char)
    };
}
#[macro_export]
macro_rules! SET {
    ($t:expr,$f:expr) => {
        (($t) |= ($f))
    };
}
#[macro_export]
macro_rules! UINT_MAX {
    () => {
        ((__INT_MAX__ as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_longlong)
    };
}
#[macro_export]
macro_rules! isblank {
    ($c:expr) => {
        __isctype!($c, _ISblank!())
    };
}
#[macro_export]
macro_rules! _ISblank {
    () => {
        _ISbit!(8)
    };
}
#[macro_export]
macro_rules! _ISbit {
    ($bit:expr) => {
        if ($bit) < 8 {
            ((1 << ($bit)) << 8)
        } else {
            ((1 << ($bit)) >> 8)
        }
    };
}
#[macro_export]
macro_rules! PACKAGE_VERSION {
    () => {
        (b"1.8.29\0" as *const u8 as *const libc::c_char)
    };
}
#[macro_export]
macro_rules! STAILQ_EMPTY {
    ($head:expr) => {
        (($head).stqh_first.is_null())
    };
}
#[macro_export]
macro_rules! TAILQ_NEXT {
    ($elm:expr, $field:expr) => {
        ((*$elm).($field).tqh_first)
    };
}
#[macro_export]
macro_rules! ISSET {
    ($t:expr,$f:expr) => {
        (($t) & ($f))
    };
}
#[no_mangle]
pub static mut filters: *mut cvtsudoers_filter =
    0 as *const cvtsudoers_filter as *mut cvtsudoers_filter;
#[no_mangle]
pub static mut sudo_user: sudo_user = sudo_user {
    pw: 0 as *const passwd as *mut passwd,
    _runas_pw: 0 as *const passwd as *mut passwd,
    _runas_gr: 0 as *const group as *mut group,
    cmnd_stat: 0 as *const stat as *mut stat,
    name: 0 as *const libc::c_char as *mut libc::c_char,
    path: 0 as *const libc::c_char as *mut libc::c_char,
    tty: 0 as *const libc::c_char as *mut libc::c_char,
    ttypath: 0 as *const libc::c_char as *mut libc::c_char,
    host: 0 as *const libc::c_char as *mut libc::c_char,
    shost: 0 as *const libc::c_char as *mut libc::c_char,
    runhost: 0 as *const libc::c_char as *mut libc::c_char,
    srunhost: 0 as *const libc::c_char as *mut libc::c_char,
    prompt: 0 as *const libc::c_char as *mut libc::c_char,
    cmnd: 0 as *const libc::c_char as *mut libc::c_char,
    cmnd_args: 0 as *const libc::c_char as *mut libc::c_char,
    cmnd_base: 0 as *const libc::c_char as *mut libc::c_char,
    cmnd_safe: 0 as *const libc::c_char as *mut libc::c_char,
    class_name: 0 as *const libc::c_char as *mut libc::c_char,
    krb5_ccname: 0 as *const libc::c_char as *mut libc::c_char,
    gid_list: 0 as *const gid_list as *mut gid_list,
    env_vars: 0 as *const *mut libc::c_char,
    role: 0 as *const libc::c_char as *mut libc::c_char,
    type_0: 0 as *const libc::c_char as *mut libc::c_char,
    cwd: 0 as *const libc::c_char,
    iolog_file: 0 as *const libc::c_char as *mut libc::c_char,
    gids: 0 as *const gid_t as *mut gid_t,
    execfd: 0,
    ngids: 0,
    closefrom: 0,
    lines: 0,
    cols: 0,
    flags: 0,
    max_groups: 0,
    timeout: 0,
    umask: 0,
    uid: 0,
    gid: 0,
    sid: 0,
};
#[no_mangle]
pub static mut list_pw: *mut passwd = 0 as *const passwd as *mut passwd;
static mut short_opts: [libc::c_char; 28] = unsafe {
    *::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(b"b:c:d:ef:hi:I:m:Mo:O:pP:s:V\0")
};
static mut long_opts: [option; 17] = [
    {
        let mut base = option {
            name: b"base\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        base
    },
    {
        let mut config = option {
            name: b"config\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        config
    },
    {
        let mut defaults = option {
            name: b"defaults\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        defaults
    },
    {
        let mut expand_aliases = option {
            name: b"expand-aliases\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        expand_aliases
    },
    {
        let mut output_format = option {
            name: b"output-format\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        output_format
    },
    {
        let mut help = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        help
    },
    {
        let mut input_format = option {
            name: b"input-format\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        input_format
    },
    {
        let mut increment = option {
            name: b"increment\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        increment
    },
    {
        let mut match0 = option {
            name: b"match\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        match0
    },
    {
        let mut match_local = option {
            name: b"match-local\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'M' as i32,
        };
        match_local
    },
    {
        let mut prune_matches = option {
            name: b"prune-matches\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        prune_matches
    },
    {
        let mut padding = option {
            name: b"padding\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'P' as i32,
        };
        padding
    },
    {
        let mut order_start = option {
            name: b"order-start\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'O' as i32,
        };
        order_start
    },
    {
        let mut output = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        output
    },
    {
        let mut suppress = option {
            name: b"suppress\0" as *const u8 as *const libc::c_char,
            has_arg: required_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        suppress
    },
    {
        let mut version = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: no_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        version
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: no_argument,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '\0' as i32,
        };
        init
    },
];
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut exitcode: libc::c_int = EXIT_FAILURE;
    let mut output_format: sudoers_formats = format_ldif;
    let mut input_format: sudoers_formats = format_sudoers;
    let mut conf: *mut cvtsudoers_config = 0 as *mut cvtsudoers_config;
    let mut match_local: bool = false;
    let mut input_file: *const libc::c_char = b"-\0" as *const u8 as *const libc::c_char;
    let mut output_file: *const libc::c_char = b"-\0" as *const u8 as *const libc::c_char;
    let mut conf_file: *const libc::c_char = _PATH_CVTSUDOERS_CONF!();
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    debug_decl!(SUDOERS_DEBUG_MAIN!());
    initprogname(if argc > 0 as libc::c_int {
        *argv.offset(0 as isize) as *const libc::c_char
    } else {
        b"cvtutsudoers\0" as *const u8 as *const libc::c_char
    });
    if !sudoers_initlocale(
        setlocale(LC_ALL, b"\0" as *const u8 as *const libc::c_char),
        def_sudoers_locale!(),
    ) {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    sudo_warn_set_locale_func_v1(Some(
        sudoers_warn_setlocale as unsafe extern "C" fn(bool, *mut libc::c_int) -> bool,
    ));
    bindtextdomain(
        b"sudoers\0" as *const u8 as *const libc::c_char,
        LOCALEDIR!(),
    );
    textdomain(b"sudoers\0" as *const u8 as *const libc::c_char);
    /* Read debug and plugin sections of sudo.conf. */
    if sudo_conf_read_v1(
        0 as *const libc::c_char,
        SUDO_CONF_DEBUG | SUDO_CONF_PLUGINS,
    ) == -1
    {
        cvtsudoers_conf_free(conf);
        sudo_debug_exit_int_v1(
            get_function_name!(),
            get_file_name!(),
            line!() as libc::c_int,
            sudo_debug_subsys,
            exitcode,
        );
        return exitcode;
    }
    /* Initialize the debug subsystem. */
    if !sudoers_debug_register(
        sudo_getprogname(),
        sudo_conf_debug_files_v1(sudo_getprogname()),
    ) {
        cvtsudoers_conf_free(conf);
        sudo_debug_exit_int_v1(
            get_function_name!(),
            get_file_name!(),
            line!() as libc::c_int,
            sudo_debug_subsys,
            exitcode,
        );
        return exitcode;
    }
    /* Check for --config option first (no getopt warnings). */
    opterr = 0;
    loop {
        ch = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            short_opts.as_ptr(),
            long_opts.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        match ch as u8 as char {
            'c' => {
                conf_file = optarg;
            }
            _ => {}
        }
    }
    /* Read conf file. */
    conf = cvtsudoers_conf_read(conf_file);
    /*
     * Reset getopt and handle the rest of the arguments.
     */
    opterr = 1;
    optind = 1;
    loop {
        ch = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            short_opts.as_ptr(),
            long_opts.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        match ch as u8 as char {
            'b' => {
                free((*conf).sudoers_base as *mut libc::c_void);
                (*conf).sudoers_base = strdup(optarg);
                if ((*conf).sudoers_base).is_null() {
                    sudo_fatalx!(
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        get_function_name!(),
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    );
                }
            }
            'c' => { /* handled above */ }
            'd' => {
                (*conf).defstr = optarg;
            }
            'e' => {
                (*conf).expand_aliases = true;
            }
            'f' => {
                free((*conf).output_format as *mut libc::c_void);
                (*conf).output_format = strdup(optarg);
                if ((*conf).output_format).is_null() {
                    sudo_fatalx!(
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        get_function_name!(),
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    );
                }
            }
            'h' => {
                help();
            }
            'i' => {
                free((*conf).input_format as *mut libc::c_void);
                (*conf).input_format = strdup(optarg);
                if ((*conf).input_format).is_null() {
                    sudo_fatalx!(
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        get_function_name!(),
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    );
                }
            }
            'I' => {
                (*conf).order_increment =
                    sudo_strtonum(optarg, 1, UINT_MAX!(), &mut errstr) as libc::c_uint;
                if !errstr.is_null() {
                    sudo_warnx!(
                        b"order increment: %s: %s\0" as *const u8 as *const libc::c_char,
                        optarg,
                        errstr
                    );
                    usage(1);
                }
            }
            'm' => {
                (*conf).filter = optarg;
            }
            'M' => {
                match_local = true;
            }
            'o' => {
                output_file = optarg;
            }
            'O' => {
                (*conf).sudo_order =
                    sudo_strtonum(optarg, 0, UINT_MAX!(), &mut errstr) as libc::c_uint;
                if !errstr.is_null() {
                    sudo_warnx!(
                        b"starting order: %s: %s\0" as *const u8 as *const libc::c_char,
                        optarg,
                        errstr
                    );
                    usage(1);
                }
            }
            'p' => {
                (*conf).prune_matches = true;
            }
            'P' => {
                (*conf).order_padding =
                    sudo_strtonum(optarg, 1, UINT_MAX!(), &mut errstr) as libc::c_uint;
                if !errstr.is_null() {
                    sudo_warnx!(
                        b"order padding: %s: %s\0" as *const u8 as *const libc::c_char,
                        optarg,
                        errstr
                    );
                    usage(1);
                }
            }
            's' => {
                (*conf).supstr = optarg;
            }
            'V' => {
                printf(
                    b"%s version %s\n\0" as *const u8 as *const libc::c_char,
                    sudo_getprogname(),
                    PACKAGE_VERSION!(),
                );
                printf(
                    b"%s grammar version %d\n\0" as *const u8 as *const libc::c_char,
                    sudo_getprogname(),
                    SUDOERS_GRAMMAR_VERSION,
                );
                exitcode = EXIT_SUCCESS;
                cvtsudoers_conf_free(conf);
                sudo_debug_exit_int_v1(
                    get_function_name!(),
                    get_file_name!(),
                    line!() as libc::c_int,
                    sudo_debug_subsys,
                    exitcode,
                );
                return exitcode;
            }
            _ => {
                usage(1);
            }
        }
    }
    argc -= optind;
    argv = argv.offset(optind as isize);
    if !((*conf).input_format).is_null() {
        if strcasecmp(
            (*conf).input_format,
            b"ldif\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            input_format = format_ldif;
        } else if strcasecmp(
            (*conf).input_format,
            b"sudoers\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            input_format = format_sudoers;
        } else {
            sudo_warnx!(
                b"unsupported input format %s\0" as *const u8 as *const libc::c_char,
                (*conf).input_format
            );
            usage(1);
        }
    }
    if !((*conf).output_format).is_null() {
        if strcasecmp(
            (*conf).output_format,
            b"json\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            output_format = format_json;
            (*conf).store_options = true;
        } else if strcasecmp(
            (*conf).output_format,
            b"ldif\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            output_format = format_ldif;
            (*conf).store_options = true;
        } else if strcasecmp(
            (*conf).output_format,
            b"sudoers\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            output_format = format_sudoers;
            (*conf).store_options = false;
        } else {
            sudo_warnx!(
                b"unsupported output format %s\0" as *const u8 as *const libc::c_char,
                (*conf).output_format
            );
            usage(1);
        }
    }
    if !((*conf).filter).is_null() {
        /* We always expand aliases when filtering (may change in future). */
        if !cvtsudoers_parse_filter((*conf).filter) {
            usage(1);
        }
    }
    if !((*conf).defstr).is_null() {
        (*conf).defaults = cvtsudoers_parse_defaults((*conf).defstr) as libc::c_short;
        if (*conf).defaults == -1 {
            usage(1);
        }
    }
    if !((*conf).supstr).is_null() {
        (*conf).suppress = cvtsudoers_parse_suppression((*conf).supstr) as libc::c_short;
        if (*conf).suppress == -1 {
            usage(1);
        }
    }
    /* Apply padding to sudo_order if present. */
    if (*conf).sudo_order != 0 as libc::c_uint && (*conf).order_padding != 0 as libc::c_uint {
        let mut multiplier: libc::c_uint = 1 as libc::c_uint;
        loop {
            multiplier = multiplier.wrapping_mul(10 as libc::c_uint);
            (*conf).order_padding = ((*conf).order_padding).wrapping_sub(1);
            if !((*conf).order_padding != 0 as libc::c_uint) {
                break;
            }
        }
        (*conf).sudo_order = ((*conf).sudo_order).wrapping_mul(multiplier);
        (*conf).order_max =
            ((*conf).sudo_order).wrapping_add(multiplier.wrapping_sub(1 as libc::c_uint));
        (*conf).order_padding = multiplier;
    }
    /* If no base DN specified, check SUDOERS_BASE. */
    if (*conf).sudoers_base.is_null() {
        (*conf).sudoers_base = getenv(b"SUDOERS_BASE\0" as *const u8 as *const libc::c_char);
        if !((*conf).sudoers_base).is_null() && *(*conf).sudoers_base as libc::c_int != '\0' as i32
        {
            (*conf).sudoers_base = strdup((*conf).sudoers_base);
            if ((*conf).sudoers_base).is_null() {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
        }
    }
    /* Input file (defaults to stdin). */
    if argc > 0 {
        if argc > 1 {
            usage(1);
        }
        input_file = *argv.offset(0 as isize);
    }
    if strcmp(input_file, b"-\0" as *const u8 as *const libc::c_char) != 0 {
        if strcmp(input_file, output_file) == 0 {
            sudo_fatalx!(
                b"%s: input and output files must be different\0" as *const u8
                    as *const libc::c_char,
                input_file
            );
        }
    }
    /* Set pwutil backend to use the filter data. */
    if !((*conf).filter).is_null() && !match_local {
        sudo_pwutil_set_backend(
            Some(
                cvtsudoers_make_pwitem
                    as unsafe extern "C" fn(uid_t, *const libc::c_char) -> *mut cache_item,
            ),
            Some(
                cvtsudoers_make_gritem
                    as unsafe extern "C" fn(gid_t, *const libc::c_char) -> *mut cache_item,
            ),
            Some(
                cvtsudoers_make_gidlist_item
                    as unsafe extern "C" fn(
                        *const passwd,
                        *const *mut libc::c_char,
                        libc::c_uint,
                    ) -> *mut cache_item,
            ),
            Some(
                cvtsudoers_make_grlist_item
                    as unsafe extern "C" fn(
                        *const passwd,
                        *const *mut libc::c_char,
                    ) -> *mut cache_item,
            ),
        );
    }
    /* We may need the hostname to resolve %h escapes in include files. */
    get_hostname();
    /* Setup defaults data structures. */
    if !init_defaults() {
        sudo_fatalx!(
            b"unable to initialize sudoers default values\0" as *const u8 as *const libc::c_char,
        );
    }
    match input_format {
        format_ldif => {
            if !parse_ldif(&mut parsed_policy, input_file, conf) {
                cvtsudoers_conf_free(conf);
                sudo_debug_exit_int_v1(
                    get_function_name!(),
                    get_file_name!(),
                    line!() as libc::c_int,
                    sudo_debug_subsys,
                    exitcode,
                );
                return exitcode;
            }
        }
        format_sudoers => {
            if !parse_sudoers(input_file, conf) {
                cvtsudoers_conf_free(conf);
                sudo_debug_exit_int_v1(
                    get_function_name!(),
                    get_file_name!(),
                    line!() as libc::c_int,
                    sudo_debug_subsys,
                    exitcode,
                );
                return exitcode;
            }
        }
        _ => {
            sudo_fatalx!(
                b"error: unhandled input %d\0" as *const u8 as *const libc::c_char,
                input_format
            );
        }
    }
    /*
     * cvtsudoers group filtering doesn't work if def_match_group_by_gid
     * is set to true by default (at compile-time). It cannot be set to false
     * because cvtsudoers doesn't apply the parsed Defaults.
     *
     * Related: sudo-1.8.23-legacy-group-processing.patch
     */
    def_legacy_group_processing!() = false as libc::c_int;
    def_match_group_by_gid!() = def_legacy_group_processing!();
    /* Apply filters. */
    filter_userspecs(&mut parsed_policy, conf);
    filter_defaults(&mut parsed_policy, conf);
    if !filters.is_null() {
        alias_remove_unused(&mut parsed_policy);
        if (*conf).prune_matches as libc::c_int != 0 && (*conf).expand_aliases as libc::c_int != 0 {
            alias_prune(&mut parsed_policy, conf);
        }
    }
    match output_format {
        format_json => {
            exitcode = !convert_sudoers_json(&mut parsed_policy, output_file, conf) as libc::c_int;
        }
        format_ldif => {
            exitcode = !convert_sudoers_ldif(&mut parsed_policy, output_file, conf) as libc::c_int;
        }
        format_sudoers => {
            exitcode =
                !convert_sudoers_sudoers(&mut parsed_policy, output_file, conf) as libc::c_int;
        }
        _ => {
            sudo_fatalx!(
                b"error: unhandled output format %d\0" as *const u8 as *const libc::c_char,
                output_format as libc::c_uint
            );
        }
    }
    cvtsudoers_conf_free(conf);
    sudo_debug_exit_int_v1(
        get_function_name!(),
        get_file_name!(),
        line!() as libc::c_int,
        sudo_debug_subsys,
        exitcode,
    );
    return exitcode;
}
/*
 * cvtsudoers configuration data.
 */
static mut cvtsudoers_config: cvtsudoers_config = {
    let mut init = cvtsudoers_config {
        sudo_order: 1 as libc::c_int as libc::c_uint,
        order_increment: 1 as libc::c_int as libc::c_uint,
        order_padding: 0 as libc::c_int as libc::c_uint,
        order_max: 0 as libc::c_int as libc::c_uint,
        defaults: 0xff as libc::c_int as libc::c_short,
        suppress: 0 as libc::c_int as libc::c_short,
        expand_aliases: false,
        store_options: true,
        prune_matches: false,
        sudoers_base: 0 as *const libc::c_char as *mut libc::c_char,
        input_format: 0 as *const libc::c_char as *mut libc::c_char,
        output_format: 0 as *const libc::c_char as *mut libc::c_char,
        filter: 0 as *const libc::c_char as *mut libc::c_char,
        defstr: 0 as *const libc::c_char as *mut libc::c_char,
        supstr: 0 as *const libc::c_char as *mut libc::c_char,
    };
    init
};
static mut cvtsudoers_conf_vars: [cvtsudoers_conf_table; 11] = [cvtsudoers_conf_table {
    conf_str: 0 as *const libc::c_char,
    type_0: 0,
    valp: 0 as *mut libc::c_void,
}; 11];
unsafe extern "C" fn run_static_initializers() {
    cvtsudoers_conf_vars = [
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"order_start\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                valp: &mut cvtsudoers_config.sudo_order as *mut libc::c_uint as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"order_increment\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                valp: &mut cvtsudoers_config.order_increment as *mut libc::c_uint
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"order_padding\0" as *const u8 as *const libc::c_char,
                type_0: 1 as libc::c_int,
                valp: &mut cvtsudoers_config.order_padding as *mut libc::c_uint
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"sudoers_base\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                valp: &mut cvtsudoers_config.sudoers_base as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"input_format\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                valp: &mut cvtsudoers_config.input_format as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"output_format\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                valp: &mut cvtsudoers_config.output_format as *mut *mut libc::c_char
                    as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"match\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                valp: &mut cvtsudoers_config.filter as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"defaults\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                valp: &mut cvtsudoers_config.defstr as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"suppress\0" as *const u8 as *const libc::c_char,
                type_0: 2 as libc::c_int,
                valp: &mut cvtsudoers_config.supstr as *mut *mut libc::c_char as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"expand_aliases\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                valp: &mut cvtsudoers_config.expand_aliases as *mut bool as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = cvtsudoers_conf_table {
                conf_str: b"prune_matches\0" as *const u8 as *const libc::c_char,
                type_0: 0 as libc::c_int,
                valp: &mut cvtsudoers_config.prune_matches as *mut bool as *mut libc::c_void,
            };
            init
        },
    ];
}
/*
 * Look up keyword in config table.
 * Returns true if found, else false.
 */
unsafe extern "C" fn cvtsudoers_parse_keyword(
    mut conf_file: *const libc::c_char,
    mut keyword: *const libc::c_char,
    mut value: *const libc::c_char,
    mut table: *mut cvtsudoers_conf_table,
) -> bool {
    let mut cur: *mut cvtsudoers_conf_table = 0 as *mut cvtsudoers_conf_table;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    /* Look up keyword in config tables */
    cur = table;
    while !((*cur).conf_str).is_null() {
        if strcasecmp(keyword, (*cur).conf_str) == 0 {
            match (*cur).type_0 {
                CONF_BOOL => {
                    *((*cur).valp as *mut bool) = sudo_strtobool_v1(value) == true as libc::c_int;
                }
                CONF_UINT => {
                    let mut uval: libc::c_uint =
                        sudo_strtonum(value, 0 as libc::c_longlong, UINT_MAX!(), &mut errstr)
                            as libc::c_uint;
                    if !errstr.is_null() {
                        sudo_warnx!(
                            b"%s: %s: %s: %s\0" as *const u8 as *const libc::c_char,
                            conf_file,
                            keyword,
                            value
                        );
                        cur = cur.offset(1);
                        continue;
                    }
                    *((*cur).valp as *mut libc::c_uint) = uval;
                }
                CONF_STR => {
                    let mut cp: *mut libc::c_char = strdup(value);
                    if cp.is_null() {
                        sudo_fatalx!(
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                            get_function_name!(),
                            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                        );
                    }
                    free(*((*cur).valp as *mut *mut libc::c_char) as *mut libc::c_void);
                    *((*cur).valp as *mut *mut libc::c_char) = cp;
                }
                _ => {}
            }
            debug_return_bool!(true);
        }
        cur = cur.offset(1);
    }
    debug_return_bool!(false);
}
unsafe extern "C" fn cvtsudoers_conf_read(
    mut conf_file: *const libc::c_char,
) -> *mut cvtsudoers_config {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linesize: size_t = 0 as libc::c_int as size_t;
    let mut fp: *mut FILE = 0 as *mut FILE;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    fp = fopen(conf_file, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        debug_return_ptr!(&mut cvtsudoers_config as *mut cvtsudoers_config);
    }
    while sudo_parseln_v2(&mut line, &mut linesize, 0 as *mut libc::c_uint, fp, 0)
        != -(1 as libc::c_int) as libc::c_long
    {
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut keyword: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        if *line as libc::c_int == '\0' as i32 {
            continue; /* skip empty line */
        }
        /* Parse keyword = value */
        keyword = line;
        cp = strchr(line, '=' as i32);
        if cp.is_null() {
            continue;
        }
        value = cp.offset(1 as isize);
        cp = cp.offset(-1);
        /* Trim whitespace after keyword. */
        while cp != line
            && isblank!(*cp.offset(-(1 as libc::c_int) as isize) as libc::c_uchar as isize) != 0
        {
            cp = cp.offset(-1);
        }
        *cp = '\0' as i32 as libc::c_char;
        /* Trim whitespace before value. */
        while isblank!(*value as libc::c_uchar as isize) != 0 {
            value = value.offset(1);
        }
        /* Look up keyword in config tables */
        if !cvtsudoers_parse_keyword(conf_file, keyword, value, cvtsudoers_conf_vars.as_mut_ptr()) {
            sudo_warnx!(
                b"%s: unknown key word: %s\0" as *const u8 as *const libc::c_char,
                conf_file,
                keyword
            );
        }
    }
    free(line as *mut libc::c_void);
    fclose(fp);
    debug_return_ptr!(&mut cvtsudoers_config as *mut cvtsudoers_config);
}
unsafe extern "C" fn cvtsudoers_conf_free(mut conf: *mut cvtsudoers_config) {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if !conf.is_null() {
        free((*conf).sudoers_base as *mut libc::c_void);
        free((*conf).input_format as *mut libc::c_void);
        free((*conf).output_format as *mut libc::c_void);
        (*conf).sudoers_base = 0 as *mut libc::c_char;
        (*conf).input_format = 0 as *mut libc::c_char;
        (*conf).output_format = 0 as *mut libc::c_char;
    }
    debug_return!();
}
unsafe extern "C" fn cvtsudoers_parse_defaults(mut expression: *mut libc::c_char) -> libc::c_int {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = expression;
    let mut flags: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    cp = strtok_r(cp, b",\0" as *const u8 as *const libc::c_char, &mut last);
    while !cp.is_null() {
        if strcasecmp(cp, b"all\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, CVT_DEFAULTS_ALL);
        } else if strcasecmp(cp, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, CVT_DEFAULTS_GLOBAL);
        } else if strcasecmp(cp, b"user\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, CVT_DEFAULTS_USER);
        } else if strcasecmp(cp, b"runas\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, CVT_DEFAULTS_RUNAS);
        } else if strcasecmp(cp, b"host\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, CVT_DEFAULTS_HOST);
        } else if strcasecmp(cp, b"command\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, CVT_DEFAULTS_CMND);
        } else {
            sudo_warnx!(
                b"invalid defaults type: %s\0" as *const u8 as *const libc::c_char,
                cp
            );
            debug_return_int!(-1);
        }
        cp = strtok_r(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
            &mut last,
        );
    }
    debug_return_int!(flags);
}
unsafe extern "C" fn cvtsudoers_parse_suppression(
    mut expression: *mut libc::c_char,
) -> libc::c_int {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = expression;
    let mut flags: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    cp = strtok_r(cp, b",\0" as *const u8 as *const libc::c_char, &mut last);
    while !cp.is_null() {
        if strcasecmp(cp, b"defaults\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, SUPPRESS_DEFAULTS);
        } else if strcasecmp(cp, b"aliases\0" as *const u8 as *const libc::c_char) == 0 {
            SET!(flags, SUPPRESS_ALIASES);
        } else if strcasecmp(cp, b"privileges\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(cp, b"privs\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            SET!(flags, SUPPRESS_PRIVS);
        } else {
            sudo_warnx!(
                b"invalid suppression type: %s\0" as *const u8 as *const libc::c_char,
                cp
            );
            debug_return_int!(-1);
        }
        cp = strtok_r(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
            &mut last,
        );
    }
    debug_return_int!(flags);
}
unsafe extern "C" fn cvtsudoers_parse_filter(mut expression: *mut libc::c_char) -> bool {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = expression;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if filters.is_null() {
        filters = malloc(::core::mem::size_of::<cvtsudoers_filter>() as libc::c_ulong)
            as *mut cvtsudoers_filter;
        if filters.is_null() {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }
        //STAILQ_INIT
        (*filters).users.stqh_first = 0 as *mut sudoers_string;
        (*filters).users.stqh_last = &mut (*filters).users.stqh_first;
        (*filters).groups.stqh_first = 0 as *mut sudoers_string;
        (*filters).groups.stqh_last = &mut (*filters).groups.stqh_first;
        (*filters).hosts.stqh_first = 0 as *mut sudoers_string;
        (*filters).hosts.stqh_last = &mut (*filters).hosts.stqh_first;
    }
    cp = strtok_r(cp, b",\0" as *const u8 as *const libc::c_char, &mut last);
    while !cp.is_null() {
        /*
         * Filter expression:
         *	user=foo,group=bar,host=baz
         */
        let mut keyword: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *mut sudoers_string = 0 as *mut sudoers_string;
        s = malloc(::core::mem::size_of::<sudoers_string>() as libc::c_ulong)
            as *mut sudoers_string;
        if s.is_null() {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }
        /* Parse keyword = value */
        keyword = cp;
        cp = strchr(cp, '=' as i32);
        if cp.is_null() {
            sudo_warnx!(
                b"invalid filter: %s\0" as *const u8 as *const libc::c_char,
                keyword
            );
            free(s as *mut libc::c_void);
            debug_return_bool!(false);
        }
        *cp = '\0' as i32 as libc::c_char;
        cp = cp.offset(1);
        (*s).str_0 = cp;
        if strcmp(keyword, b"user\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*s).entries.stqe_next = 0 as *mut sudoers_string;
            *(*filters).users.stqh_last = s;
            (*filters).users.stqh_last = &mut (*s).entries.stqe_next;
        } else if strcmp(keyword, b"group\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*s).entries.stqe_next = 0 as *mut sudoers_string;
            *(*filters).groups.stqh_last = s;
            (*filters).groups.stqh_last = &mut (*s).entries.stqe_next;
        } else if strcmp(keyword, b"host\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*s).entries.stqe_next = 0 as *mut sudoers_string;
            *(*filters).hosts.stqh_last = s;
            (*filters).hosts.stqh_last = &mut (*s).entries.stqe_next;
        } else {
            sudo_warnx!(
                b"invalid filter: %s\0" as *const u8 as *const libc::c_char,
                keyword
            );
            free(s as *mut libc::c_void);
            debug_return_bool!(false);
        }
        cp = strtok_r(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
            &mut last,
        );
    }
    debug_return_bool!(true);
}
unsafe extern "C" fn parse_ldif(
    mut parse_tree: *mut sudoers_parse_tree,
    mut input_file: *const libc::c_char,
    mut conf: *mut cvtsudoers_config,
) -> bool {
    let mut fp: *mut FILE = stdin;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    /* Open LDIF file and parse it. */
    if strcmp(input_file, b"-\0" as *const u8 as *const libc::c_char) != 0 {
        fp = fopen(input_file, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            sudo_fatal!(
                b"unable to open %s\0" as *const u8 as *const libc::c_char,
                input_file,
            );
        }
    }
    debug_return_bool!(sudoers_parse_ldif(
        parse_tree,
        fp,
        (*conf).sudoers_base,
        (*conf).store_options
    ));
}
unsafe extern "C" fn parse_sudoers(
    mut input_file: *const libc::c_char,
    mut conf: *mut cvtsudoers_config,
) -> bool {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    /* Open sudoers file and parse it. */
    if strcmp(input_file, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        sudoersin = stdin;
        input_file = b"stdin\0" as *const u8 as *const libc::c_char;
    } else {
        sudoersin = fopen(input_file, b"r\0" as *const u8 as *const libc::c_char);
        if sudoersin.is_null() {
            sudo_fatal!(
                b"unable to open %s\0" as *const u8 as *const libc::c_char,
                input_file,
            );
        }
    }
    init_parser(input_file, false);
    if sudoersparse() != 0 && !parse_error {
        sudo_warnx!(
            b"failed to parse %s file, unknown error\0" as *const u8 as *const libc::c_char,
            input_file
        );
        parse_error = true;
        rcstr_delref(errorfile);
        errorfile = rcstr_dup(input_file);
        if errorfile.is_null() {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }
    }
    if parse_error {
        if errorlineno != -(1 as libc::c_int) {
            sudo_warnx!(
                b"parse error in %s near line %d\n\0" as *const u8 as *const libc::c_char,
                errorfile,
                errorlineno
            );
        } else if !errorfile.is_null() {
            sudo_warnx!(
                b"parse error in %s\n\0" as *const u8 as *const libc::c_char,
                errorfile
            );
        }
        debug_return_bool!(false);
    }
    debug_return_bool!(true);
}
#[no_mangle]
pub unsafe extern "C" fn open_sudoers(
    mut sudoers: *const libc::c_char,
    mut doedit: bool,
    mut keepopen: *mut bool,
) -> *mut FILE {
    return fopen(sudoers, b"r\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn userlist_matches_filter(
    mut parse_tree: *mut sudoers_parse_tree,
    mut users: *mut member_list,
    mut conf: *mut cvtsudoers_config,
) -> bool {
    let mut s: *mut sudoers_string = 0 as *mut sudoers_string;
    let mut m: *mut member = 0 as *mut member;
    let mut next: *mut member = 0 as *mut member;
    let mut ret: bool = false;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if filters.is_null() || STAILQ_EMPTY!((*filters).users) && STAILQ_EMPTY!((*filters).groups) {
        debug_return_bool!(true);
    }
    m = *(*((*users).tqh_last as *mut member_list)).tqh_last;
    while !m.is_null() && {
        next = *(*((*m).entries.tqe_prev as *mut member_list)).tqh_last;
        1 as libc::c_int != 0
    } {
        let mut matched: bool = false;
        if STAILQ_EMPTY!((*filters).users) {
            let mut pw: passwd = passwd {
                pw_name: 0 as *mut libc::c_char,
                pw_passwd: 0 as *mut libc::c_char,
                pw_uid: 0,
                pw_gid: 0,
                pw_gecos: 0 as *mut libc::c_char,
                pw_dir: 0 as *mut libc::c_char,
                pw_shell: 0 as *mut libc::c_char,
            };
            /*
             * Only groups in filter, make a dummy user so userlist_matches()
             * can do its thing.
             */
            memset(
                &mut pw as *mut passwd as *mut libc::c_void,
                0,
                ::core::mem::size_of::<passwd>() as libc::c_ulong,
            );
            pw.pw_name = b"_nobody\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            pw.pw_uid = -(1 as libc::c_int) as uid_t;
            pw.pw_gid = -(1 as libc::c_int) as gid_t;
            if user_matches(parse_tree, &mut pw, m) == true as libc::c_int {
                matched = true;
            }
        } else {
            s = (*filters).users.stqh_first;
            while !s.is_null() {
                let mut pw: *mut passwd = 0 as *mut passwd;
                /* An upper case filter entry may be a User_Alias */
                /* XXX - doesn't handle nested aliases */
                if (*m).type0 as libc::c_int == ALIAS as libc::c_int && !(*conf).expand_aliases {
                    if strcmp((*m).name, (*s).str_0) == 0 {
                        matched = true;
                        break;
                    }
                }
                if *((*s).str_0).offset(0 as isize) as libc::c_int == '#' as i32 {
                    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
                    let mut uid: uid_t =
                        sudo_strtoid_v2(((*s).str_0).offset(1 as isize), &mut errstr);
                    if errstr.is_null() {
                        pw = sudo_getpwuid(uid);
                    }
                }
                if pw.is_null() {
                    pw = sudo_getpwnam((*s).str_0);
                }
                if !pw.is_null() {
                    if user_matches(parse_tree, pw, m) == true as libc::c_int {
                        matched = true;
                    }
                    sudo_pw_delref(pw);
                    /* Only need one user in the filter to match. */
                    if matched {
                        break;
                    }
                }
                s = (*s).entries.stqe_next;
            }
        }
        if matched {
            ret = true;
        } else if (*conf).prune_matches {
            if !((*m).entries.tqe_next).is_null() {
                (*(*m).entries.tqe_next).entries.tqe_prev = (*m).entries.tqe_prev;
            } else {
                (*users).tqh_last = (*m).entries.tqe_prev;
            }
            *(*m).entries.tqe_prev = (*m).entries.tqe_next;
            free_member(m);
        }
        m = next;
    }
    debug_return_bool!(ret);
}
unsafe extern "C" fn hostlist_matches_filter(
    mut parse_tree: *mut sudoers_parse_tree,
    mut hostlist: *mut member_list,
    mut conf: *mut cvtsudoers_config,
) -> bool {
    let mut s: *mut sudoers_string = 0 as *mut sudoers_string;
    let mut m: *mut member = 0 as *mut member;
    let mut next: *mut member = 0 as *mut member;
    let mut lhost: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shost: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: bool = false;
    let mut shosts: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if filters.is_null() || STAILQ_EMPTY!((*filters).hosts) {
        debug_return_bool!(true);
    }
    /* Create an array of short host names. */
    s = (*filters).hosts.stqh_first;
    while !s.is_null() {
        n += 1;
        s = (*s).entries.stqe_next;
    }
    shosts = reallocarray(
        0 as *mut libc::c_void,
        n as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if shosts.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    n = 0;
    s = (*filters).hosts.stqh_first;
    while !s.is_null() {
        lhost = (*s).str_0;
        shost = strchr(lhost, '.' as i32);
        if !shost.is_null() {
            shost = strndup(lhost, shost.offset_from(lhost) as libc::c_long as size_t);
            if shost.is_null() {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
        } else {
            shost = lhost;
        }
        n = n + 1;
        let ref mut fresh5 = *shosts.offset(n as isize);
        *fresh5 = shost;
        s = (*s).entries.stqe_next;
    }
    m = *(*((*hostlist).tqh_last as *mut member_list)).tqh_last;
    while !m.is_null() && {
        next = *(*((*m).entries.tqe_prev as *mut member_list)).tqh_last;
        1 as libc::c_int != 0
    } {
        let mut matched: bool = false;
        n = 0;
        s = (*filters).hosts.stqh_first;
        while !s.is_null() {
            lhost = (*s).str_0;
            n = n + 1;
            shost = *shosts.offset(n as isize);
            /* An upper case filter entry may be a Host_Alias */
            /* XXX - doesn't handle nested aliases */
            if (*m).type0 as libc::c_int == ALIAS as libc::c_int && !(*conf).expand_aliases {
                if strcmp((*m).name, (*s).str_0) == 0 {
                    matched = true;
                    break;
                }
            }
            /* Only need one host in the filter to match. */
            /* XXX - can't use netgroup_tuple with NULL pw */
            if host_matches(parse_tree, 0 as *const passwd, lhost, shost, m) == true as libc::c_int
            {
                matched = true;
                break;
            } else {
                s = (*s).entries.stqe_next;
            }
        }
        if matched {
            ret = true;
        } else if (*conf).prune_matches {
            if !((*m).entries.tqe_next).is_null() {
                (*(*m).entries.tqe_next).entries.tqe_prev = (*m).entries.tqe_prev;
            } else {
                (*hostlist).tqh_last = (*m).entries.tqe_prev;
            }
            *(*m).entries.tqe_prev = (*m).entries.tqe_next;
            free_member(m);
        }
        m = next;
    }
    /* Free shosts array and its contents. */
    n = 0;
    s = (*filters).hosts.stqh_first;
    while !s.is_null() {
        lhost = (*s).str_0;
        n = n + 1;
        shost = *shosts.offset(n as isize);
        if shost != lhost {
            free(shost as *mut libc::c_void);
        }
        s = (*s).entries.stqe_next;
    }
    free(shosts as *mut libc::c_void);
    debug_return_bool!(ret == true);
}
/*
 * Display Defaults entries
 */
unsafe extern "C" fn print_defaults_sudoers(
    mut parse_tree: *mut sudoers_parse_tree,
    mut lbuf: *mut sudo_lbuf,
    mut expand_aliases: bool,
) -> bool {
    let mut def: *mut defaults_struct = 0 as *mut defaults_struct;
    let mut next: *mut defaults_struct = 0 as *mut defaults_struct;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    def = (*parse_tree).defaults.tqh_first;
    while !def.is_null() && {
        next = (*def).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        sudoers_format_default_line(lbuf, parse_tree, def, &mut next, expand_aliases);
        def = next;
    }
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
unsafe extern "C" fn print_alias_sudoers(
    mut parse_tree: *mut sudoers_parse_tree,
    mut a: *mut alias_struct,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut lbuf: *mut sudo_lbuf = v as *mut sudo_lbuf;
    let mut m: *mut member = 0 as *mut member;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    sudo_lbuf_append_v1(
        lbuf,
        b"%s %s = \0" as *const u8 as *const libc::c_char,
        alias_type_to_string((*a).type0 as libc::c_int),
        (*a).name,
    );
    m = (*a).members.tqh_first;
    while !m.is_null() {
        if m != (*a).members.tqh_first {
            sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
        }
        sudoers_format_member(lbuf, parse_tree, m, 0 as *const libc::c_char, UNSPEC);
        m = (*m).entries.tqe_next;
    }
    sudo_lbuf_append_v1(lbuf, b"\n\0" as *const u8 as *const libc::c_char);
    if sudo_lbuf_error_v1(lbuf) as libc::c_int != 0 {
        debug_return_int!(-(1 as libc::c_int))
    } else {
        debug_return_int!(0)
    };
}
/*
 * Display aliases
 */
unsafe extern "C" fn print_aliases_sudoers(
    mut parse_tree: *mut sudoers_parse_tree,
    mut lbuf: *mut sudo_lbuf,
) -> bool {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    alias_apply(
        parse_tree,
        Some(
            print_alias_sudoers
                as unsafe extern "C" fn(
                    *mut sudoers_parse_tree,
                    *mut alias_struct,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        lbuf as *mut libc::c_void,
    );
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
/* global for convert_sudoers_output */
static mut output_fp: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn convert_sudoers_output(mut buf: *const libc::c_char) -> libc::c_int {
    return fputs(buf, output_fp);
}
/*
 * Apply filters to userspecs, removing non-matching entries.
 */
unsafe extern "C" fn filter_userspecs(
    mut parse_tree: *mut sudoers_parse_tree,
    mut conf: *mut cvtsudoers_config,
) {
    let mut us: *mut userspec = 0 as *mut userspec;
    let mut next_us: *mut userspec = 0 as *mut userspec;
    let mut priv_0: *mut privilege = 0 as *mut privilege;
    let mut next_priv: *mut privilege = 0 as *mut privilege;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if filters.is_null() {
        debug_return!();
    }
    /*
     * Does not currently prune out non-matching entries in the user or
     * host lists.  It acts more like a grep than a true filter.
     * In the future, we may want to add a prune option.
     */
    us = (*parse_tree).userspecs.tqh_first;
    while !us.is_null() && {
        next_us = (*us).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        if !userlist_matches_filter(parse_tree, &mut (*us).users, conf) {
            if !((*us).entries.tqe_next).is_null() {
                (*(*us).entries.tqe_next).entries.tqe_prev = (*us).entries.tqe_prev;
            } else {
                (*parse_tree).userspecs.tqh_last = (*us).entries.tqe_prev;
            }
            *(*us).entries.tqe_prev = (*us).entries.tqe_next;
            free_userspec(us);
        } else {
            priv_0 = (*us).privileges.tqh_first;
            while !priv_0.is_null() && {
                next_priv = (*priv_0).entries.tqe_next;
                1 as libc::c_int != 0
            } {
                if !hostlist_matches_filter(parse_tree, &mut (*priv_0).hostlist, conf) {
                    if !((*priv_0).entries.tqe_next).is_null() {
                        (*(*priv_0).entries.tqe_next).entries.tqe_prev = (*priv_0).entries.tqe_prev;
                    } else {
                        (*us).privileges.tqh_last = (*priv_0).entries.tqe_prev;
                    }
                    *(*priv_0).entries.tqe_prev = (*priv_0).entries.tqe_next;
                    free_privilege(priv_0);
                }
                priv_0 = next_priv;
            }
            if ((*us).privileges.tqh_first).is_null() {
                if !((*us).entries.tqe_next).is_null() {
                    (*(*us).entries.tqe_next).entries.tqe_prev = (*us).entries.tqe_prev;
                } else {
                    (*parse_tree).userspecs.tqh_last = (*us).entries.tqe_prev;
                }
                *(*us).entries.tqe_prev = (*us).entries.tqe_next;
                free_userspec(us);
            }
        }
        us = next_us;
    }
    debug_return!();
}
/*
 * Check whether the alias described by "alias_name" is the same
 * as "name" or includes an alias called "name".
 * Returns true if matched, else false.
 */
unsafe extern "C" fn alias_matches(
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *const libc::c_char,
    mut alias_name: *const libc::c_char,
    mut alias_type: libc::c_int,
) -> bool {
    let mut a: *mut alias_struct = 0 as *mut alias_struct;
    let mut m: *mut member = 0 as *mut member;
    let mut ret: bool = false;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if strcmp(name, alias_name) == 0 {
        debug_return_bool!(true);
    }
    a = alias_get(parse_tree, alias_name, alias_type);
    if !a.is_null() {
        m = (*a).members.tqh_first;
        while !m.is_null() {
            if !((*m).type0 as libc::c_int != ALIAS) {
                if alias_matches(parse_tree, name, (*m).name, alias_type) {
                    ret = true;
                    break;
                }
            }
            m = (*m).entries.tqe_next;
        }
        alias_put(a);
    }
    debug_return_bool!(ret);
}
/*
 * Check whether userspecs uses the aliases in the specified member lists.
 * If used, they are removed (and freed) from the list.
 * This does *not* check Defaults for used aliases, only userspecs.
 */
unsafe extern "C" fn alias_used_by_userspecs(
    mut parse_tree: *mut sudoers_parse_tree,
    mut user_aliases: *mut member_list,
    mut runas_aliases: *mut member_list,
    mut host_aliases: *mut member_list,
    mut cmnd_aliases: *mut member_list,
) {
    let mut priv_0: *mut privilege = 0 as *mut privilege;
    let mut priv_next: *mut privilege = 0 as *mut privilege;
    let mut us: *mut userspec = 0 as *mut userspec;
    let mut us_next: *mut userspec = 0 as *mut userspec;
    let mut cs: *mut cmndspec = 0 as *mut cmndspec;
    let mut cs_next: *mut cmndspec = 0 as *mut cmndspec;
    let mut m: *mut member = 0 as *mut member;
    let mut m_next: *mut member = 0 as *mut member;
    let mut am: *mut member = 0 as *mut member;
    let mut am_next: *mut member = 0 as *mut member;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    /* Iterate over the policy, checking for aliases. */
    us = (*parse_tree).userspecs.tqh_first;
    while !us.is_null() && {
        us_next = (*us).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        m = (*us).users.tqh_first;
        while !m.is_null() && {
            m_next = (*m).entries.tqe_next;
            1 as libc::c_int != 0
        } {
            if (*m).type0 as libc::c_int == ALIAS {
                /* If alias is used, remove from user_aliases and free. */
                am = (*user_aliases).tqh_first;
                while !am.is_null() && {
                    am_next = (*am).entries.tqe_next;
                    1 as libc::c_int != 0
                } {
                    if alias_matches(parse_tree, (*am).name, (*m).name, USERALIAS) {
                        if !((*am).entries.tqe_next).is_null() {
                            (*(*am).entries.tqe_next).entries.tqe_prev = (*am).entries.tqe_prev;
                        } else {
                            (*user_aliases).tqh_last = (*am).entries.tqe_prev;
                        }
                        *(*am).entries.tqe_prev = (*am).entries.tqe_next;
                        free_member(am);
                    }
                    am = am_next
                }
            }
            m = m_next;
        }
        priv_0 = (*us).privileges.tqh_first;
        while !priv_0.is_null() && {
            priv_next = (*priv_0).entries.tqe_next;
            1 as libc::c_int != 0
        } {
            m = (*priv_0).hostlist.tqh_first;
            while !m.is_null() {
                if (*m).type0 as libc::c_int == ALIAS {
                    /* If alias is used, remove from host_aliases and free. */
                    am = (*host_aliases).tqh_first;
                    while !am.is_null() && {
                        am_next = (*am).entries.tqe_next;
                        1 as libc::c_int != 0
                    } {
                        if alias_matches(parse_tree, (*am).name, (*m).name, HOSTALIAS) {
                            if !((*am).entries.tqe_next).is_null() {
                                (*(*am).entries.tqe_next).entries.tqe_prev = (*am).entries.tqe_prev;
                            } else {
                                (*host_aliases).tqh_last = (*am).entries.tqe_prev;
                            }
                            *(*am).entries.tqe_prev = (*am).entries.tqe_next;
                            free_member(am);
                        }
                        am = am_next;
                    }
                }
                m = (*m).entries.tqe_next;
            }
            cs = (*priv_0).cmndlist.tqh_first;
            while !cs.is_null() && {
                cs_next = (*cs).entries.tqe_next;
                1 as libc::c_int != 0
            } {
                if !((*cs).runasuserlist).is_null() {
                    m = (*(*cs).runasuserlist).tqh_first;
                    while !m.is_null() && {
                        m_next = (*m).entries.tqe_next;
                        1 as libc::c_int != 0
                    } {
                        if (*m).type0 as libc::c_int == ALIAS {
                            /* If alias is used, remove from runas_aliases and free. */
                            am = (*runas_aliases).tqh_first;
                            while !am.is_null() && {
                                am_next = (*am).entries.tqe_next;
                                1 as libc::c_int != 0
                            } {
                                if alias_matches(parse_tree, (*am).name, (*m).name, RUNASALIAS) {
                                    if !((*am).entries.tqe_next).is_null() {
                                        (*(*am).entries.tqe_next).entries.tqe_prev =
                                            (*am).entries.tqe_prev;
                                    } else {
                                        (*runas_aliases).tqh_last = (*am).entries.tqe_prev;
                                    }
                                    *(*am).entries.tqe_prev = (*am).entries.tqe_next;
                                    free_member(am);
                                }
                                am = am_next;
                            }
                        }
                        m = m_next;
                    }
                }
                if !((*cs).runasgrouplist).is_null() {
                    m = (*(*cs).runasgrouplist).tqh_first;
                    while !m.is_null() && {
                        m_next = (*m).entries.tqe_next;
                        1 as libc::c_int != 0
                    } {
                        if (*m).type0 as libc::c_int == ALIAS {
                            /* If alias is used, remove from runas_aliases and free. */
                            am = (*runas_aliases).tqh_first;
                            while !am.is_null() && {
                                am_next = (*am).entries.tqe_next;
                                1 as libc::c_int != 0
                            } {
                                if alias_matches(parse_tree, (*am).name, (*m).name, RUNASALIAS) {
                                    if !((*am).entries.tqe_next).is_null() {
                                        (*(*am).entries.tqe_next).entries.tqe_prev =
                                            (*am).entries.tqe_prev;
                                    } else {
                                        (*runas_aliases).tqh_last = (*am).entries.tqe_prev;
                                    }
                                    *(*am).entries.tqe_prev = (*am).entries.tqe_next;
                                    free_member(am);
                                }
                                am = am_next;
                            }
                        }
                        m = m_next;
                    }
                }
                m = (*cs).cmnd;
                if (*m).type0 as libc::c_int == ALIAS {
                    /* If alias is used, remove from cmnd_aliases and free. */
                    am = (*cmnd_aliases).tqh_first;
                    while !am.is_null() && {
                        am_next = (*am).entries.tqe_next;
                        1 as libc::c_int != 0
                    } {
                        if alias_matches(parse_tree, (*am).name, (*m).name, CMNDALIAS) {
                            if !((*am).entries.tqe_next).is_null() {
                                (*(*am).entries.tqe_next).entries.tqe_prev = (*am).entries.tqe_prev;
                            } else {
                                (*cmnd_aliases).tqh_last = (*am).entries.tqe_prev;
                            }
                            *(*am).entries.tqe_prev = (*am).entries.tqe_next;
                            free_member(am);
                        }
                        am = am_next;
                    }
                }
                cs = cs_next;
            }
            priv_0 = priv_next
        }
        us = us_next;
    }
    debug_return!();
}
/*
 * Apply filters to host/user-based Defaults, removing non-matching entries.
 */
unsafe extern "C" fn filter_defaults(
    mut parse_tree: *mut sudoers_parse_tree,
    mut conf: *mut cvtsudoers_config,
) {
    let mut user_aliases: member_list = member_list {
        tqh_first: 0 as *mut member,
        tqh_last: 0 as *mut *mut member,
    };
    user_aliases = {
        let mut init = member_list {
            tqh_first: 0 as *mut member,
            tqh_last: &mut user_aliases.tqh_first,
        };
        init
    };
    let mut runas_aliases: member_list = member_list {
        tqh_first: 0 as *mut member,
        tqh_last: 0 as *mut *mut member,
    };
    runas_aliases = {
        let mut init = member_list {
            tqh_first: 0 as *mut member,
            tqh_last: &mut runas_aliases.tqh_first,
        };
        init
    };
    let mut host_aliases: member_list = member_list {
        tqh_first: 0 as *mut member,
        tqh_last: 0 as *mut *mut member,
    };
    host_aliases = {
        let mut init = member_list {
            tqh_first: 0 as *mut member,
            tqh_last: &mut host_aliases.tqh_first,
        };
        init
    };
    let mut cmnd_aliases: member_list = member_list {
        tqh_first: 0 as *mut member,
        tqh_last: 0 as *mut *mut member,
    };
    cmnd_aliases = {
        let mut init = member_list {
            tqh_first: 0 as *mut member,
            tqh_last: &mut cmnd_aliases.tqh_first,
        };
        init
    };
    let mut prev_binding: *mut member_list = 0 as *mut member_list;
    let mut def: *mut defaults_struct = 0 as *mut defaults_struct;
    let mut def_next: *mut defaults_struct = 0 as *mut defaults_struct;
    let mut m: *mut member = 0 as *mut member;
    let mut m_next: *mut member = 0 as *mut member;
    let mut a: *mut alias_struct = 0 as *mut alias_struct;
    let mut alias_type: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if filters.is_null() && (*conf).defaults as libc::c_int == CVT_DEFAULTS_ALL {
        debug_return!();
    }
    def = (*parse_tree).defaults.tqh_first;
    while !def.is_null() && {
        def_next = (*def).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        let mut keep: bool = true;
        match (*def).type_0 as libc::c_int {
            DEFAULTS => {
                if ISSET!((*conf).defaults as libc::c_int, CVT_DEFAULTS_GLOBAL) == 0 {
                    keep = false;
                }
                alias_type = UNSPEC;
            }
            DEFAULTS_USER => {
                if ISSET!((*conf).defaults as libc::c_int, CVT_DEFAULTS_USER) == 0
                    || !userlist_matches_filter(parse_tree, (*def).binding, conf)
                {
                    keep = false;
                }
                alias_type = USERALIAS;
            }
            DEFAULTS_RUNAS => {
                if ISSET!((*conf).defaults as libc::c_int, CVT_DEFAULTS_RUNAS) == 0 {
                    keep = false;
                }
                alias_type = RUNASALIAS;
            }
            DEFAULTS_HOST => {
                if ISSET!((*conf).defaults as libc::c_int, CVT_DEFAULTS_HOST) == 0
                    || !hostlist_matches_filter(parse_tree, (*def).binding, conf)
                {
                    keep = false;
                }
                alias_type = HOSTALIAS;
            }
            DEFAULTS_CMND => {
                if ISSET!((*conf).defaults as libc::c_int, CVT_DEFAULTS_CMND) == 0 {
                    keep = false;
                }
                alias_type = CMNDALIAS;
            }
            _ => {
                sudo_fatalx_nodebug_v1(
                    b"unexpected defaults type %d\0" as *const u8 as *const libc::c_char,
                    (*def).type_0 as libc::c_int,
                );
            }
        }
        if !keep {
            /* Look for aliases used by the binding. */
            /* XXX - move to function */
            if alias_type != UNSPEC && (*def).binding != prev_binding {
                m = (*(*def).binding).tqh_first;
                while !m.is_null() && {
                    m_next = (*m).entries.tqe_next;
                    1 as libc::c_int != 0
                } {
                    if (*m).type0 as libc::c_int == ALIAS {
                        if !((*m).entries.tqe_next).is_null() {
                            (*(*m).entries.tqe_next).entries.tqe_prev = (*m).entries.tqe_prev;
                        } else {
                            (*(*def).binding).tqh_last = (*m).entries.tqe_prev;
                        }
                        *(*m).entries.tqe_prev = (*m).entries.tqe_next;
                        match alias_type {
                            USERALIAS => {
                                (*m).entries.tqe_next = 0 as *mut member;
                                (*m).entries.tqe_prev = user_aliases.tqh_last;
                                *user_aliases.tqh_last = m;
                                user_aliases.tqh_last = &mut (*m).entries.tqe_next;
                            }
                            RUNASALIAS => {
                                (*m).entries.tqe_next = 0 as *mut member;
                                (*m).entries.tqe_prev = runas_aliases.tqh_last;
                                *runas_aliases.tqh_last = m;
                                runas_aliases.tqh_last = &mut (*m).entries.tqe_next;
                            }
                            HOSTALIAS => {
                                (*m).entries.tqe_next = 0 as *mut member;
                                (*m).entries.tqe_prev = host_aliases.tqh_last;
                                *host_aliases.tqh_last = m;
                                host_aliases.tqh_last = &mut (*m).entries.tqe_next;
                            }
                            CMNDALIAS => {
                                (*m).entries.tqe_next = 0 as *mut member;
                                (*m).entries.tqe_prev = cmnd_aliases.tqh_last;
                                *cmnd_aliases.tqh_last = m;
                                cmnd_aliases.tqh_last = &mut (*m).entries.tqe_next;
                            }
                            _ => {
                                sudo_fatalx_nodebug_v1(
                                    b"unexpected alias type %d\0" as *const u8
                                        as *const libc::c_char,
                                    alias_type,
                                );
                            }
                        }
                    }
                    m = m_next;
                }
            }
            if !((*def).entries.tqe_next).is_null() {
                (*(*def).entries.tqe_next).entries.tqe_prev = (*def).entries.tqe_prev;
            } else {
                (*parse_tree).defaults.tqh_last = (*def).entries.tqe_prev;
            }
            *(*def).entries.tqe_prev = (*def).entries.tqe_next;
            free_default(def, &mut prev_binding);
            if !prev_binding.is_null() {
                /* Remove and free Defaults that share the same binding. */
                while !def_next.is_null() && (*def_next).binding == prev_binding {
                    def = def_next;
                    def_next = (*def).entries.tqe_next;
                    if !((*def).entries.tqe_next).is_null() {
                        (*(*def).entries.tqe_next).entries.tqe_prev = (*def).entries.tqe_prev;
                    } else {
                        (*parse_tree).defaults.tqh_last = (*def).entries.tqe_prev;
                    }
                    *(*def).entries.tqe_prev = (*def).entries.tqe_next;
                    free_default(def, &mut prev_binding);
                }
            }
        } else {
            prev_binding = (*def).binding;
        }
        def = def_next;
    }
    /* Remove now-unreferenced aliases. */
    alias_used_by_userspecs(
        parse_tree,
        &mut user_aliases,
        &mut runas_aliases,
        &mut host_aliases,
        &mut cmnd_aliases,
    );
    m = user_aliases.tqh_first;
    while !m.is_null() && {
        m_next = (*m).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        a = alias_remove(parse_tree, (*m).name, USERALIAS);
        alias_free(a as *mut libc::c_void);
        free_member(m);
        m = m_next;
    }
    m = runas_aliases.tqh_first;
    while !m.is_null() && {
        m_next = (*m).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        a = alias_remove(parse_tree, (*m).name, RUNASALIAS);
        alias_free(a as *mut libc::c_void);
        free_member(m);
        m = m_next;
    }
    m = host_aliases.tqh_first;
    while !m.is_null() && {
        m_next = (*m).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        a = alias_remove(parse_tree, (*m).name, HOSTALIAS);
        alias_free(a as *mut libc::c_void);
        free_member(m);
        m = m_next;
    }
    m = cmnd_aliases.tqh_first;
    while !m.is_null() && {
        m_next = (*m).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        a = alias_remove(parse_tree, (*m).name, CMNDALIAS);
        alias_free(a as *mut libc::c_void);
        free_member(m);
        m = m_next;
    }
    debug_return!();
}
/*
 * Remove unreferenced aliases.
 */
unsafe extern "C" fn alias_remove_unused(mut parse_tree: *mut sudoers_parse_tree) {
    let mut used_aliases: *mut rbtree = 0 as *mut rbtree;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    used_aliases = alloc_aliases();
    if used_aliases.is_null() {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    /* Move all referenced aliases to used_aliases. */
    if !alias_find_used(parse_tree, used_aliases) {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    /* Only unreferenced aliases are left, swap and free the unused ones. */
    free_aliases((*parse_tree).aliases);
    (*parse_tree).aliases = used_aliases;
    debug_return!();
}
/*
 * Prune out non-matching entries from user and host aliases.
 */
unsafe extern "C" fn alias_prune_helper(
    mut parse_tree: *mut sudoers_parse_tree,
    mut a: *mut alias_struct,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut conf: *mut cvtsudoers_config = v as *mut cvtsudoers_config;
    /* XXX - misue of these functions */
    match (*a).type0 as libc::c_int {
        USERALIAS => {
            userlist_matches_filter(parse_tree, &mut (*a).members, conf);
        }
        HOSTALIAS => {
            hostlist_matches_filter(parse_tree, &mut (*a).members, conf);
        }
        _ => {}
    }
    return 0;
}
/*
 * Prune out non-matching entries from within aliases.
 */
unsafe extern "C" fn alias_prune(
    mut parse_tree: *mut sudoers_parse_tree,
    mut conf: *mut cvtsudoers_config,
) {
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    alias_apply(
        parse_tree,
        Some(
            alias_prune_helper
                as unsafe extern "C" fn(
                    *mut sudoers_parse_tree,
                    *mut alias_struct,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        conf as *mut libc::c_void,
    );
    debug_return!();
}
/*
 * Convert back to sudoers.
 */
unsafe extern "C" fn convert_sudoers_sudoers(
    mut parse_tree: *mut sudoers_parse_tree,
    mut output_file: *const libc::c_char,
    mut conf: *mut cvtsudoers_config,
) -> bool {
    let mut ret: bool = true;
    let mut lbuf: sudo_lbuf = sudo_lbuf {
        output: None,
        buf: 0 as *mut libc::c_char,
        continuation: 0 as *const libc::c_char,
        indent: 0,
        len: 0,
        size: 0,
        cols: 0,
        error: 0,
    };
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if strcmp(output_file, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        output_fp = stdout;
    } else {
        output_fp = fopen(output_file, b"w\0" as *const u8 as *const libc::c_char);
        if output_fp.is_null() {
            sudo_fatal!(
                b"unable to open %s\0" as *const u8 as *const libc::c_char,
                output_file,
            );
        }
    }
    /* Wrap lines at 80 columns with a 4 character indent. */
    sudo_lbuf_init_v1(
        &mut lbuf,
        Some(convert_sudoers_output as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int),
        4,
        b"\\\0" as *const u8 as *const libc::c_char,
        80,
    );
    'done: loop {
        /* Print Defaults */
        if ISSET!((*conf).suppress as libc::c_int, SUPPRESS_DEFAULTS) == 0 {
            if !print_defaults_sudoers(parse_tree, &mut lbuf, (*conf).expand_aliases) {
                break 'done;
            }
            if lbuf.len > 0 {
                sudo_lbuf_print_v1(&mut lbuf);
                sudo_lbuf_append_v1(
                    &mut lbuf as *mut sudo_lbuf,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        /* Print Aliases */
        if !(*conf).expand_aliases && ISSET!((*conf).suppress as libc::c_int, SUPPRESS_ALIASES) == 0
        {
            if !print_aliases_sudoers(parse_tree, &mut lbuf) {
                break 'done;
            }
            if lbuf.len > 1 {
                sudo_lbuf_print_v1(&mut lbuf);
                sudo_lbuf_append_v1(
                    &mut lbuf as *mut sudo_lbuf,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        /* Print User_Specs, separated by blank lines. */
        if ISSET!((*conf).suppress as libc::c_int, SUPPRESS_PRIVS) == 0 {
            if !sudoers_format_userspecs(
                &mut lbuf,
                parse_tree,
                b"\n\0" as *const u8 as *const libc::c_char,
                (*conf).expand_aliases,
                true,
            ) {
                break 'done;
            }
            if lbuf.len > 1 {
                sudo_lbuf_print_v1(&mut lbuf);
            }
        }
        break;
    }
    if sudo_lbuf_error_v1(&mut lbuf) {
        if errno!() == ENOMEM {
            sudo_fatalx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
        }
        ret = false;
    }
    sudo_lbuf_destroy_v1(&mut lbuf);
    fflush(output_fp);
    if ferror(output_fp) != 0 {
        sudo_warn!(
            b"unable to write to %s\0" as *const u8 as *const libc::c_char,
            output_file
        );
        ret = false;
    }
    if output_fp != stdout {
        fclose(output_fp);
    }
    debug_return_bool!(ret);
}
unsafe extern "C" fn usage(mut fatal: libc::c_int) {
    fprintf(
        if fatal != 0 { stderr } else { stdout },
        b"usage: %s [-ehMpV] [-b dn] [-c conf_file ] [-d deftypes] [-f output_format] [-i input_format] [-I increment] [-m filter] [-o output_file] [-O start_point] [-P padding] [-s sections] [input_file]\n\0"
            as *const u8 as *const libc::c_char,
        sudo_getprogname(),
    );
    if fatal != 0 {
        exit(1);
    }
}
unsafe extern "C" fn help() -> ! {
    printf(
        dcgettext(
            b"sudoers\0" as *const u8 as *const libc::c_char,
            b"%s - convert between utsudoers file formats\n\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        sudo_getprogname(),
    );
    usage(0 as libc::c_int);
    puts(dcgettext(
        b"sudoers\0" as *const u8 as *const libc::c_char,
        b"\nOptions: 
  -b, --base=dn              the base DN for sudo LDAP queries
  -c, --config=conf_file     the path to the configuration file
  -d, --defaults=deftypes    only convert Defaults of the specified types
  -e, --expand-aliases       expand aliases when converting
  -f, --output-format=format set output format: JSON, LDIF or sudoers
  -i, --input-format=format  set input format: LDIF or sudoers
  -I, --increment=num        amount to increase each sudoOrder by
  -h, --help                 display help message and exit
  -m, --match=filter         only convert entries that match the filter
  -M, --match-local          match filter uses passwd and group databases
  -o, --output=output_file   write converted utsudoers to output_file
  -O, --order-start=num      starting point for first sudoOrder
  -p, --prune-matches        prune non-matching users, groups and hosts
  -P, --padding=num          base padding for sudoOrder increment
  -s, --suppress=sections    suppress output of certain sections
  -V, --version              display version information and exit\0" as *const u8
            as *const libc::c_char,
        LC_MESSAGES,
    ));
    exit(0);
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

