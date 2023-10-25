/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_imports)]
#![allow(clashing_extern_declarations)]
#![allow(improper_ctypes)]

use crate::struct_macro::*;

use libc::FILE;
//c中全局此处写法固定static mut,
#[no_mangle]
pub static mut tgetpass_flags: libc::c_int = 0;

static mut sudo_settings: [sudo_settings; 24] = [
    {
        let mut init = sudo_settings {
            name: b"bsdauth_type\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"login_class\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"preserve_environment\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"runas_group\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"set_home\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"run_shell\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"login_shell\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"ignore_ticket\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"selinux_role\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"selinux_type\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"runas_user\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"progname\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"implied_shell\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"preserve_groups\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"noninteractive\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"sudoedit\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"closefrom\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"network_addrs\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"max_groups\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"plugin_dir\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"remote_host\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"timeout\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: 0 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment {
    pub envp: *mut *mut libc::c_char,
    pub env_size: size_t,
    pub env_len: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}

//'a' as i32 ?
static mut long_opts: [option; 29] = [
    {
        let mut init = option {
            name: b"askpass\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"auth-type\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"background\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bell\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"close-from\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"login-class\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-env\0" as *const u8 as *const libc::c_char,
            has_arg: 2,
            flag: 0 as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"edit\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"group\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"set-home\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"host\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 256 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"login\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-timestamp\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'K' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reset-timestamp\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"list\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"non-interactive\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-groups\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"role\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdin\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"shell\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"type\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"command-timeout\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"other-user\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"validate\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: '\u{0}' as i32,
        };
        init
    },
];


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
