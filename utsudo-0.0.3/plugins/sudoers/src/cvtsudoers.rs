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
