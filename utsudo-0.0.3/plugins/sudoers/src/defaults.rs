/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    unused_variables,
    non_camel_case_types,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::alias::cmndtag;
use crate::common::*;
use crate::timeout::parse_timeout;
pub const SUDO_DEBUG_ERROR: libc::c_int = 2;
pub const SUDO_DEBUG_ERRNO: libc::c_int = 1 << 4;
pub const SUDO_DEBUG_LINENO: libc::c_int = 1 << 5;
pub const ALLOW: libc::c_int = 1;
pub const DEFAULTS: libc::c_int = 265;
pub const SETDEF_GENERIC: libc::c_int = 0x01;
pub const DEFAULTS_USER: libc::c_int = 267;
pub const SETDEF_USER: libc::c_int = 0x04;
pub const DEFAULTS_RUNAS: libc::c_int = 268;
pub const SETDEF_RUNAS: libc::c_int = 0x08;
pub const DEFAULTS_HOST: libc::c_int = 266;
pub const SETDEF_HOST: libc::c_int = 0x02;
pub const DEFAULTS_CMND: libc::c_int = 269;
pub const SETDEF_CMND: libc::c_int = 0x10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_defs_types {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub desc: *mut libc::c_char,
    pub values: *mut def_values,
    pub callback: Option<unsafe extern "C" fn(*const sudo_defs_val) -> bool>,
    pub sd_un: sudo_defs_val,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct def_values {
    pub sval: *mut libc::c_char,
    pub nval: def_tuple,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strmap {
    pub name: *mut libc::c_char,
    pub num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct early_default {
    pub idx: libc::c_short,
    pub run_callback: libc::c_short,
}
static mut facilities: [strmap; 13] = [
    //init struct
    {
        let mut init = strmap {
            name: b"authpriv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (10 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"auth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (4 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"daemon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (3 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (1 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (16 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (17 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (18 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (19 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (20 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (21 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (22 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"local7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: (23 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            num: -(1 as libc::c_int),
        };
        init
    },
];
static mut priorities: [strmap; 10] = [
    {
        let mut init = strmap {
            name: b"alert\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"crit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"debug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"emerg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"err\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"notice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"warning\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = strmap {
            name: b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            num: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = strmap {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            num: -(1 as libc::c_int),
        };
        init
    },
];
static mut early_defaults: [early_default; 8] = [
    {
        let mut init = early_default {
            idx: 106 as libc::c_int as libc::c_short,
            run_callback: 0,
        };
        init
    },
    {
        let mut init = early_default {
            idx: 22 as libc::c_int as libc::c_short,
            run_callback: 0,
        };
        init
    },
    {
        let mut init = early_default {
            idx: 100 as libc::c_int as libc::c_short,
            run_callback: 0,
        };
        init
    },
    {
        let mut init = early_default {
            idx: 117 as libc::c_int as libc::c_short,
            run_callback: 0,
        };
        init
    },
    {
        let mut init = early_default {
            idx: 78 as libc::c_int as libc::c_short,
            run_callback: 0,
        };
        init
    },
    {
        let mut init = early_default {
            idx: 51 as libc::c_int as libc::c_short,
            run_callback: 0,
        };
        init
    },
    {
        let mut init = early_default {
            idx: 69 as libc::c_int as libc::c_short,
            run_callback: 0,
        };
        init
    },
    {
        let mut init = early_default {
            idx: -(1 as libc::c_int) as libc::c_short,
            run_callback: 0,
        };
        init
    },
];
#[derive(Copy, Clone)]
#[repr(C)]
pub union sudo_defs_val {
    //union
    pub flag: libc::c_int,
    pub ival: libc::c_int,
    pub uival: libc::c_uint,
    pub tuple: def_tuple,
    pub str_0: *mut libc::c_char,
    pub mode: mode_t,
    pub tspec: timespec,
    pub list: list_members,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct member {
    pub entries: member_tmp,
    pub name: *mut libc::c_char,
    pub type_0: libc::c_short,
    pub negated: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct member_tmp {
    pub tqe_next: *mut member,
    pub tqe_prev: *mut *mut member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defaults_tmp {
    pub tqe_next: *mut defaults,
    pub tqe_prev: *mut *mut defaults,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_member {
    pub entries: tmp_list_member,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec_list {
    pub tqh_first: *mut userspec,
    pub tqh_last: *mut *mut userspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec_tmp {
    pub tqe_next: *mut userspec,
    pub tqe_prev: *mut *mut userspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userspec {
    pub entries: userspec_tmp,
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
    pub entries: sudoers_comment_tmp,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_comment_tmp {
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
    pub entries: privilege_tmp,
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
    pub entries: cmndspec_tmp,
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
pub struct cmndspec_tmp {
    pub tqe_next: *mut cmndspec,
    pub tqe_prev: *mut *mut cmndspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privilege_tmp {
    pub tqe_next: *mut privilege,
    pub tqe_prev: *mut *mut privilege,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_members {
    pub slh_first: *mut list_member,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tmp_list_member {
    pub sle_next: *mut list_member,
}
pub type mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type __syscall_slong_t = libc::c_long;
pub type def_tuple = libc::c_uint;
pub const kernel: def_tuple = 9;
pub const tty: def_tuple = 8;
pub const ppid: def_tuple = 7;
pub const global: def_tuple = 6;
pub const digest_only: def_tuple = 5;
pub const all: def_tuple = 4;
pub const any: def_tuple = 3;
pub const always: def_tuple = 2;
pub const once: def_tuple = 1;
pub const never: def_tuple = 0;
//pub const INT_MIN:libc::c_ulong = -0x7fffffff - 1;
pub type list_ops = libc::c_uint; //union
pub const freeall: list_ops = 2;
pub const delete: list_ops = 2;
pub const add: list_ops = 2;
pub type size_t = libc::c_ulong;
pub const ACCESSPERMS: libc::c_int = (0o400 as libc::c_int
    | 0o200 as libc::c_int
    | 0o100 as libc::c_int
    | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) >> 3 as libc::c_int
    | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3)
    as libc::c_int;
extern "C" {
    static mut sudo_printf:
        Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
    static mut sudo_user: sudo_user;
    static mut sudo_defs_table: [sudo_defs_types; 122];
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn init_envtables() -> bool;
    fn sudoers_initlocale(ulocale: *const libc::c_char, slocale: *const libc::c_char) -> bool;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn free(__ptr: *mut libc::c_void);
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    fn sudo_strtomode_v1(cp: *const libc::c_char, errstr: *mut *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn userlist_matches(
        parse_tree: *mut sudoers_parse_tree,
        pw: *const passwd,
        list: *const member_list,
    ) -> libc::c_int;
    fn runaslist_matches(
        parse_tree: *mut sudoers_parse_tree,
        user_list: *const member_list,
        group_list: *const member_list,
        matching_user: *mut *mut member,
        matching_group: *mut *mut member,
    ) -> libc::c_int;
    fn hostlist_matches(
        parse_tree: *mut sudoers_parse_tree,
        pw: *const passwd,
        list: *const member_list,
    ) -> libc::c_int;
    fn cmndlist_matches(
        parse_tree: *mut sudoers_parse_tree,
        list: *const member_list,
    ) -> libc::c_int;
}
