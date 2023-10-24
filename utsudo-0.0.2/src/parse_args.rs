/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

//externc
extern "C" {
    fn gc_add(type_0: sudo_gc_types, v: *mut libc::c_void) -> bool;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sudo_conf_plugin_dir_path_v1() -> *const libc::c_char;
    static mut list_user: *const libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __functin: *const libc::c_char,
    ) -> !;
    static mut optind: libc::c_int;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_new_key_val_v1(
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_getprogname() -> *const libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(__s: *const libc::c_char) -> size_t;
    fn exit(_: libc::c_int) -> !;
    fn sudo_lbuf_init_v1(
        lbuf: *mut sudo_lbuf,
        output: sudo_lbuf_output_t,
        indent: libc::c_int,
        continuation: *const libc::c_char,
        cols: libc::c_int,
    );
    fn sudo_lbuf_destroy_v1(lbuf: *mut sudo_lbuf);
    fn sudo_lbuf_append_v1(lbuf: *mut sudo_lbuf, fmt: *const libc::c_char, _: ...) -> bool;
    fn sudo_lbuf_print_v1(lbuf: *mut sudo_lbuf);
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut user_details: user_details;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn get_net_ifs(addrinfo: *mut *mut libc::c_char) -> libc::c_int;
    fn sudo_conf_max_groups_v1() -> libc::c_int;
    //    fn sudo_fatal_nodebug_v1(fmt:*const libc::c_char,_:...) -> !;
    //    fn sudo_fatalx_nodebug_v1(fmt:*const libc::c_char,_:...) -> !;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn getopt_long(
        __argc: libc::c_int,
        __argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    static mut optarg: *mut libc::c_char;
}
