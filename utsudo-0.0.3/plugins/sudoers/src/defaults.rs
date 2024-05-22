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



#[no_mangle]
unsafe extern "C" fn store_int(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> bool {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if str0.is_null() {
        (*sd_un).ival = 0;
    } else {
        i = sudo_strtonum(
            str0,
            (-(217483647) - 1) as libc::c_longlong,
            217483647 as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        if !errstr.is_null() {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                str0,
                errstr
            );
            debug_return_bool!(false);
        }
        (*sd_un).ival = i;
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn store_list(
    mut str: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
    mut op: libc::c_int,
) -> bool {
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if op == 0 || op == 1 {
        list_op(0 as *const libc::c_char, 0 as size_t, sd_un, freeall);
    }
    if !str.is_null() {
        let mut cp: *const libc::c_char = 0 as *const libc::c_char;
        let mut ep: *const libc::c_char = 0 as *const libc::c_char;
        let mut end: *const libc::c_char = str.offset(strlen(str) as isize);
        let lop: list_ops = (if op == '-' as i32 { delete } else { add }) as list_ops;
        cp = sudo_strsplit_v1(
            str,
            end,
            b" \t\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );
        while !cp.is_null() {
            if !list_op(cp, ep.offset_from(cp) as size_t, sd_un, lop) {
                debug_return_bool!(false);
            }
            cp = sudo_strsplit_v1(
                0 as *const libc::c_char,
                end,
                b"\t\0" as *const u8 as *const libc::c_char,
                &mut ep,
            );
        }
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn list_op(
    mut str0: *const libc::c_char,
    mut len: size_t,
    mut sd_un: *mut sudo_defs_val,
    mut op: list_ops,
) -> bool {
    let mut cur: *mut list_member = 0 as *mut list_member;
    let mut prev: *mut list_member = 0 as *mut list_member;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if op == freeall {
        loop {
            cur = (*sd_un).list.slh_first;
            if cur.is_null() {
                break;
            }
            (*sd_un).list.slh_first = (*(*sd_un).list.slh_first).entries.sle_next;
            free((*cur).value as *mut libc::c_void);
            free(cur as *mut libc::c_void);
        }
        debug_return_bool!(true);
    }
    cur = (*sd_un).list.slh_first;
    while !cur.is_null() {
        if strncmp((*cur).value, str0, len) == 0 as libc::c_int
            && *((*cur).value).offset(len as isize) as libc::c_int == '\u{0}' as i32
        {
            if op == add {
                debug_return_bool!(true);
            }
            if prev.is_null() {
                (*sd_un).list.slh_first = (*(*sd_un).list.slh_first).entries.sle_next;
            } else {
                (*prev).entries.sle_next = (*(*prev).entries.sle_next).entries.sle_next;
            }
            free((*cur).value as *mut libc::c_void);
            free(cur as *mut libc::c_void);
            break;
        }
        prev = cur;
        cur = (*cur).entries.sle_next;
    }
    if op == add {
        cur = calloc(1, ::std::mem::size_of::<list_member>() as libc::c_ulong) as *mut list_member;
        if cur.is_null() || {
            (*cur).value = strndup(str0, len);
            (*cur).value.is_null()
        } {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            free(cur as *mut libc::c_void);
            debug_return_bool!(false);
        }
        (*cur).entries.sle_next = (*sd_un).list.slh_first;
        (*sd_un).list.slh_first = cur;
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn store_mode(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> bool {
    let mut mode: mode_t = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let SUDOERS_DEBUG_DEFAULTS: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize)
        as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if str0.is_null() {
        (*sd_un).mode = ACCESSPERMS as mode_t;
    } else {
        mode = sudo_strtomode_v1(str0, &mut errstr) as mode_t;
        if !errstr.is_null() {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"%s is %s\0" as *const u8 as *const libc::c_char,
                str0,
                errstr
            );
            debug_return_bool!(false);
        }
        (*sd_un).mode = mode;
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn store_str(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> libc::c_int {
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    free((*sd_un).str_0 as *mut libc::c_void);
    if str0.is_null() {
        (*sd_un).str_0 = 0 as *mut libc::c_char;
    } else {
        (*sd_un).str_0 = strdup(str0);
        if ((*sd_un).str_0).is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            debug_return_int!(-1);
        }
    }
    debug_return_int!(1);
}
#[no_mangle]
unsafe extern "C" fn store_syslogfac(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> bool {
    let mut fac: *mut strmap = 0 as *mut strmap;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if str0.is_null() {
        (*sd_un).ival = 0 as libc::c_int;
        debug_return_bool!(true);
    }
    fac = facilities.as_mut_ptr();
    while !((*fac).name).is_null() {
        if strcmp(str0, (*fac).name) == 0 {
            (*sd_un).ival = (*fac).num;
            debug_return_bool!(true);
        }
        fac = fac.offset(1);
    }
    debug_return_bool!(false);
}
#[no_mangle]
unsafe extern "C" fn store_syslogpri(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> bool {
    let mut pri: *mut strmap = 0 as *mut strmap;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if str0.is_null() {
        (*sd_un).ival = -(1 as libc::c_int);
        debug_return_bool!(true);
    }
    pri = priorities.as_mut_ptr();
    while !((*pri).name).is_null() {
        if strcmp(str0, (*pri).name) == 0 {
            (*sd_un).ival = (*pri).num;
            debug_return_bool!(true);
        }
        pri = pri.offset(1);
    }
    debug_return_bool!(false)
}
#[no_mangle]
unsafe extern "C" fn store_timeout(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> bool {
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if str0.is_null() {
        (*sd_un).ival = 0;
    } else {
        let mut seconds: libc::c_int = parse_timeout(str0);
        if seconds == -1 {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_ERRNO | SUDO_DEBUG_LINENO,
                b"%s\0" as *const u8 as *const libc::c_char,
                str0
            );
            debug_return_bool!(false);
        }
        (*sd_un).ival = seconds;
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn store_tuple(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
    mut tuple_vals: *mut def_values,
) -> bool {
    let mut v: *mut def_values = 0 as *mut def_values;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if str0.is_null() {
        (*sd_un).ival = 0;
    } else {
        v = tuple_vals;
        while !((*v).sval).is_null() {
            if strcmp((*v).sval, str0) == 0 {
                (*sd_un).tuple = (*v).nval;
                break;
            }
            v = v.offset(1);
        }
        if ((*v).sval).is_null() {
            debug_return_bool!(true);
        }
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn store_uint(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> bool {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut u: libc::c_uint = 0 as libc::c_uint;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if str0.is_null() {
        (*sd_un).uival = 0;
    } else {
        u = sudo_strtonum(
            str0,
            0,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_longlong,
            &mut errstr,
        ) as libc::c_uint;
        if !errstr.is_null() {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                str0,
                errstr
            );
            debug_return_bool!(false);
        }
        (*sd_un).uival = u;
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn store_timespec(
    mut str0: *const libc::c_char,
    mut sd_un: *mut sudo_defs_val,
) -> bool {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut sign: libc::c_char = '+' as i32 as libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    ts.tv_nsec = 0;
    ts.tv_sec = ts.tv_nsec;
    if !str0.is_null() {
        if *str0 as libc::c_int == '+' as i32 || *str0 as libc::c_int == '-' as i32 {
            sign = *(str0.offset(1));
        }
        while *str0 as libc::c_int != '\0' as i32 && *str0 as libc::c_int != '.' as i32 {
            if *(*__ctype_b_loc()).offset(*str0 as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                debug_return_bool!(false);
            }
            if ts.tv_sec as libc::c_longlong
                > 9223372036854775807 as libc::c_longlong / 10 as libc::c_int as libc::c_longlong
            {
                debug_return_bool!(false);
            }
            ts.tv_sec *= 10 as libc::c_int as libc::c_long;
            let fresh1 = str0;
            str0 = str0.offset(1);
            ts.tv_sec += (*fresh1 as libc::c_int - '0' as i32) as libc::c_long;
        }
        let fresh2 = str0;
        str0 = str0.offset(1);
        if *fresh2 as libc::c_int == '.' as i32 {
            i = 100000000 as libc::c_int;
            while i > 0 as libc::c_int {
                if *str0 as libc::c_int == '\u{0}' as i32 {
                    break;
                }
                if *(*__ctype_b_loc()).offset(*str0 as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
                {
                    debug_return_bool!(false);
                }
                let fresh3 = str0;
                str0 = str0.offset(1);
                ts.tv_nsec += (i * (*fresh3 as libc::c_int - '0' as i32)) as libc::c_long;
                i /= 10 as libc::c_int;
            }
        }
        if ts.tv_sec as libc::c_longlong
            > 9223372036854775807 as libc::c_longlong / 60 as libc::c_int as libc::c_longlong
        {
            debug_return_bool!(false);
        }
        ts.tv_sec *= 60 as libc::c_int as libc::c_long;
        ts.tv_nsec *= 60 as libc::c_int as libc::c_long;
        while ts.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
            ts.tv_sec += 1;
            ts.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
        }
    } //if
    if sign as libc::c_int == '-' as i32 {
        (*sd_un).tspec.tv_sec = -ts.tv_sec;
        (*sd_un).tspec.tv_nsec = -ts.tv_nsec;
    } else {
        (*sd_un).tspec.tv_sec = ts.tv_sec;
        (*sd_un).tspec.tv_nsec = ts.tv_nsec;
    }
    debug_return_bool!(true);
}
#[no_mangle]
unsafe extern "C" fn logfac2str(mut n: libc::c_int) -> *const libc::c_char {
    let mut fac: *mut strmap = 0 as *mut strmap;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    fac = facilities.as_mut_ptr();
    while !((*fac).name).is_null() && (*fac).num != n {
        fac = fac.offset(1);
    }
    debug_return_const_str!((*fac).name);
}
#[no_mangle]
unsafe extern "C" fn logpri2str(mut n: libc::c_int) -> *const libc::c_char {
    let mut pri: *mut strmap = 0 as *mut strmap;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    pri = priorities.as_mut_ptr();
    while !((*pri).name).is_null() {
        if (*pri).num == n {
            debug_return_const_str!((*pri).name);
        }
        pri = pri.offset(1);
    }
    let mut sudo_debug_ret_mid: *const libc::c_char =
        b"unknown\0" as *const u8 as *const libc::c_char;
    debug_return_const_str!(sudo_debug_ret_mid);
}
#[no_mangle]
unsafe extern "C" fn find_default(
    mut name: *const libc::c_char,
    mut file: *const libc::c_char,
    mut lineno: libc::c_int,
    mut quiet: bool,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    i = 0 as libc::c_int;
    while !(sudo_defs_table[i as usize].name).is_null() {
        if strcmp(name, sudo_defs_table[i as usize].name) == 0 as libc::c_int {
            debug_return_int!(i);
        }
        i += 1;
    }
    if !quiet && sudo_defs_table[106 as libc::c_int as usize].sd_un.flag == 0 {
        if lineno > 0 as libc::c_int {
            sudo_warnx!(
                b"%s:%d unknown defaults entry \"%s\"\0" as *const u8 as *const libc::c_char,
                file,
                lineno,
                name,
            );
        } else {
            sudo_warnx!(
                b"%s: unknown defaults entry \"%s\"\0" as *const u8 as *const libc::c_char,
                file,
                name,
            );
        }
    }
    debug_return_int!(-1);
}
pub const T_MASK: libc::c_int = 0x0FF;
pub const T_FLAG: libc::c_int = 0x004;
pub const T_TUPLE: libc::c_int = 0x009;
pub const T_LOGFAC: libc::c_int = 0x007;
pub const T_BOOL: libc::c_int = 0x100;
pub const T_LOGPRI: libc::c_int = 0x008;
pub const T_STR: libc::c_int = 0x003;
pub const T_PATH: libc::c_int = 0x200;
pub const T_INT: libc::c_int = 0x001;
pub const T_UINT: libc::c_int = 0x002;
pub const T_MODE: libc::c_int = 0x005;
pub const T_LIST: libc::c_int = 0x006;
pub const T_TIMEOUT: libc::c_int = 0x020;
pub const T_TIMESPEC: libc::c_int = 0x010;
pub const SUDO_CONV_INFO_MSG: libc::c_int = 4;
#[no_mangle]
unsafe extern "C" fn parse_default_entry(
    mut def: *mut sudo_defs_types,
    mut val: *const libc::c_char,
    mut op: libc::c_int,
    mut sd_un: *mut sudo_defs_val,
    mut file: *const libc::c_char,
    mut lineno: libc::c_int,
    mut quiet: bool,
) -> bool {
    let mut rc: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"%s: %s:%d %s=%s op=%d\0" as *const u8 as *const libc::c_char,
        get_function_name!(),
        get_file_name!(),
        line!(),
        (*def).name,
        if !val.is_null() {
            val
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        op
    );
    if val.is_null() {
        match (*def).type_0 & T_MASK {
            T_FLAG => {}
            T_TUPLE => {
                if (*def).type_0 & T_BOOL as libc::c_int == 0 {
                    if op == 1 {
                        val = b"authpriv\0" as *const u8 as *const libc::c_char;
                    }
                }
            }
            T_LOGFAC => {
                if op == 1 {
                    val = b"authpriv\0" as *const u8 as *const libc::c_char;
                }
            }
            _ => {
                if (*def).type_0 & T_BOOL as libc::c_int == 0 || op != 0 as libc::c_int {
                    if !quiet {
                        if lineno > 0 {
                            sudo_warnx!(
                                b"%s:%d no value specified for \"%s\"\0" as *const u8
                                    as *const libc::c_char,
                                file,
                                lineno,
                                (*def).name,
                            );
                        } else {
                            sudo_warnx!(
                                b"%s: no value specified for \"%s\"\0" as *const u8
                                    as *const libc::c_char,
                                file,
                                (*def).name,
                            );
                        }
                    }
                    debug_return_bool!(false);
                }
            }
        }
    }
    match (*def).type_0 & T_MASK {
        T_LOGFAC => {
            rc = store_syslogfac(val, sd_un) as libc::c_int;
        }
        T_LOGPRI => {
            rc = store_syslogpri(val, sd_un) as libc::c_int;
        }
        T_STR => {
            if (*def).type_0 & T_PATH != 0 && !val.is_null() && *val as libc::c_int != '/' as i32 {
                if !quiet {
                    if lineno > 0 {
                        sudo_warnx!(
                            b"%s:%d values for \"%s\" must start with a '/'\0" as *const u8
                                as *const libc::c_char,
                            file,
                            lineno,
                            (*def).name,
                        );
                    } else {
                        sudo_warnx!(
                            b"%s: values for \"%s\" must start with a '/'\0" as *const u8
                                as *const libc::c_char,
                            file,
                            (*def).name,
                        );
                    }
                }
                rc = -1;
            }
            rc = store_str(val, sd_un) as libc::c_int;
        }
        T_INT => {
            rc = store_int(val, sd_un) as libc::c_int;
        }
        T_UINT => {
            rc = store_uint(val, sd_un) as libc::c_int;
        }
        T_MODE => {
            rc = store_mode(val, sd_un) as libc::c_int;
        }
        T_FLAG => {
            if !val.is_null() {
                if !quiet {
                    if lineno > 0 {
                        sudo_warnx!(
                            b"%s:%d option \"%s\" does not take a value\0" as *const u8
                                as *const libc::c_char,
                            file,
                            lineno,
                            (*def).name,
                        );
                    } else {
                        sudo_warnx!(
                            b"%s: option \"%s\" does not take a value\0" as *const u8
                                as *const libc::c_char,
                            file,
                            (*def).name,
                        );
                    }
                }
                rc = -1;
            }
            (*sd_un).flag = op;
            rc = 1 as libc::c_int;
        }
        T_LIST => {
            rc = store_list(val, sd_un, op) as libc::c_int;
        }
        T_TIMEOUT => {
            rc = store_timeout(val, sd_un) as libc::c_int;
        }
        T_TUPLE => {
            rc = store_tuple(val, sd_un, (*def).values) as libc::c_int;
        }
        T_TIMESPEC => {
            rc = store_timespec(val, sd_un) as libc::c_int;
        }
        _ => {
            if !quiet {
                if lineno > 0 {
                    sudo_warnx!(
                        b"%s:%d invalid defaults type 0x%x for option \"%s\"\0" as *const u8
                            as *const libc::c_char,
                        file,
                        lineno,
                        (*def).type_0,
                        (*def).name,
                    );
                } else {
                    sudo_warnx!(
                        b"%s: invalid defaults type 0x%x for option \"%s\"\0" as *const u8
                            as *const libc::c_char,
                        file,
                        (*def).type_0,
                        (*def).name,
                    );
                }
            }
            rc = -1;
        }
    } //end of match
    if rc == 0 as libc::c_int {
        if !quiet {
            if lineno > 0 as libc::c_int {
                sudo_warnx!(
                    b"%s:%d no value \"%s\" is invalid for option \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    file,
                    lineno,
                    val,
                    (*def).name,
                );
            } else {
                sudo_warnx!(
                    b"%s: no value \"%s\" is invalid for option \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    file,
                    val,
                    (*def).name,
                );
            }
        }
    }
    debug_return_bool!(rc == true as libc::c_int);
}
#[no_mangle]
unsafe extern "C" fn is_early_default(mut name: *const libc::c_char) -> *mut early_default {
    let mut early: *mut early_default = 0 as *mut early_default;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    early = early_defaults.as_mut_ptr();
    while (*early).idx != -1 {
        if strcmp(name, sudo_defs_table[(*early).idx as usize].name) == 0 as libc::c_int {
            debug_return_ptr!(early);
        }
        early = early.offset(1);
    }
    debug_return_ptr!(0 as *mut early_default);
}
#[no_mangle]
unsafe extern "C" fn run_callback(mut def: *mut sudo_defs_types) -> bool {
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if ((*def).callback).is_none() {
        debug_return_bool!(true);
    }
    debug_return_bool!(((*def).callback).expect("non-null function pointer")(
        &mut (*def).sd_un
    ));
}
#[no_mangle]
unsafe extern "C" fn set_default(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut op: libc::c_int,
    mut file: *const libc::c_char,
    mut lineno: libc::c_int,
    mut quiet: bool,
) -> bool {
    let mut idx: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    idx = find_default(var, file, lineno, quiet);
    if idx != -1 {
        let mut def: *mut sudo_defs_types =
            &mut *sudo_defs_table.as_mut_ptr().offset(idx as isize) as *mut sudo_defs_types;
        if parse_default_entry(def, val, op, &mut (*def).sd_un, file, lineno, quiet) {
            debug_return_bool!(run_callback(def));
        }
    }
    debug_return_bool!(false);
}
#[no_mangle]
unsafe extern "C" fn set_early_default(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut op: libc::c_int,
    mut file: *const libc::c_char,
    mut lineno: libc::c_int,
    mut quiet: bool,
    mut early: *mut early_default,
) -> bool {
    let mut idx: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    idx = find_default(var, file, lineno, quiet);
    if idx != -1 {
        let mut def: *mut sudo_defs_types =
            &mut *sudo_defs_table.as_mut_ptr().offset(idx as isize) as *mut sudo_defs_types;
        if parse_default_entry(def, val, op, &mut (*def).sd_un, file, lineno, quiet) {
            (*early).run_callback = 1 as libc::c_int as libc::c_short;
            debug_return_bool!(true);
        }
    }
    debug_return_bool!(false);
}
#[no_mangle]
unsafe extern "C" fn free_defs_val(mut type_0: libc::c_int, mut sd_un: *mut sudo_defs_val) {
    match type_0 & 0xff as libc::c_int {
        T_STR => {
            free((*sd_un).str_0 as *mut libc::c_void);
        }
        T_LIST => {
            list_op(
                0 as *const libc::c_char,
                0 as libc::c_int as size_t,
                sd_un,
                freeall,
            );
        }
        _ => {}
    }
    memset(
        sd_un as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sudo_defs_val>() as libc::c_ulong,
    );
}
#[no_mangle]
unsafe extern "C" fn default_binding_matches(
    mut parse_tree: *mut sudoers_parse_tree,
    mut d: *mut defaults,
    mut what: libc::c_int,
) -> bool {
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    match (*d).type_0 as libc::c_int {
        DEFAULTS => {
            debug_return_bool!(true);
        }
        DEFAULTS_USER => {
            if userlist_matches(parse_tree, sudo_user.pw, (*d).binding) == ALLOW {
                debug_return_bool!(true);
            }
        }
        DEFAULTS_RUNAS => {
            if runaslist_matches(
                parse_tree,
                (*d).binding,
                0 as *mut member_list,
                0 as *mut *mut member,
                0 as *mut *mut member,
            ) == ALLOW
            {
                debug_return_bool!(true);
            }
        }
        DEFAULTS_HOST => {
            if hostlist_matches(parse_tree, sudo_user.pw, (*d).binding) == ALLOW {
                debug_return_bool!(true);
            }
        }
        DEFAULTS_CMND => {
            if cmndlist_matches(parse_tree, (*d).binding) == ALLOW {
                debug_return_bool!(true);
            }
        }
        _ => {}
    }
    debug_return_bool!(false);
}
#[no_mangle]
unsafe extern "C" fn default_type_matches(mut d: *mut defaults, mut what: libc::c_int) -> bool {
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    match (*d).type_0 as libc::c_int {
        DEFAULTS => {
            if what & SETDEF_GENERIC as libc::c_int != 0 {
                debug_return_bool!(true);
            }
        }
        DEFAULTS_USER => {
            if what & SETDEF_USER as libc::c_int != 0 {
                debug_return_bool!(true);
            }
        }
        DEFAULTS_RUNAS => {
            if what & DEFAULTS_RUNAS as libc::c_int != 0 {
                debug_return_bool!(true);
            }
        }
        DEFAULTS_HOST => {
            if what & SETDEF_HOST as libc::c_int != 0 {
                debug_return_bool!(true);
            }
        }
        DEFAULTS_CMND => {
            if what & SETDEF_CMND as libc::c_int != 0 {
                debug_return_bool!(true);
            }
        }
        _ => {}
    }
    debug_return_bool!(false);
}
#[no_mangle]
pub unsafe extern "C" fn run_early_defaults() -> bool {
    let mut early: *mut early_default = 0 as *mut early_default;
    let mut ret: bool = 1 as libc::c_int != 0;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    early = early_defaults.as_mut_ptr();
    while (*early).idx as libc::c_int != -(1 as libc::c_int) {
        if (*early).run_callback != 0 {
            if !run_callback(&mut *sudo_defs_table.as_mut_ptr().offset((*early).idx as isize)) {
                ret = 0 as libc::c_int != 0;
            }
            (*early).run_callback = 0 as libc::c_int as libc::c_short;
        }
        early = early.offset(1);
    }
    debug_return_bool!(ret);
}
#[no_mangle]
unsafe extern "C" fn update_defaults(
    mut parse_tree: *mut sudoers_parse_tree,
    mut defs: *mut defaults_list,
    mut what: libc::c_int,
    mut quiet: bool,
) -> bool {
    let mut d: *mut defaults = 0 as *mut defaults;
    let mut ret: bool = true as bool;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    sudo_debug_printf!(
        SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
        b"what: 0x%02x\0" as *const u8 as *const libc::c_char,
        what
    );
    if defs.is_null() {
        defs = &mut (*parse_tree).defaults;
    }
    d = (*defs).tqh_first;
    while !d.is_null() {
        let mut early: *mut early_default = is_early_default((*d).var) as *mut early_default;
        if !early.is_null() {
            if !(!default_type_matches(d, what) || !default_binding_matches(parse_tree, d, what)) {
                if !set_early_default(
                    (*d).var,
                    (*d).val,
                    (*d).op as libc::c_int,
                    (*d).file,
                    (*d).lineno,
                    quiet,
                    early,
                ) {
                    ret = 0 as libc::c_int != 0;
                }
            }
        }
        d = (*d).entries.tqe_next;
    }
    if !run_early_defaults() {
        ret = 0 as libc::c_int != 0;
    }
    d = (*defs).tqh_first;
    while !d.is_null() {
        if (is_early_default((*d).var)).is_null() {
            if !(!default_type_matches(d, what) || !default_binding_matches(parse_tree, d, what)) {
                if !set_default(
                    (*d).var,
                    (*d).val,
                    (*d).op as libc::c_int,
                    (*d).file,
                    (*d).lineno,
                    quiet,
                ) {
                    ret = 0 as libc::c_int != 0;
                }
            }
        }
        d = (*d).entries.tqe_next;
    }
    debug_return_bool!(ret);
}
#[no_mangle]
unsafe extern "C" fn dump_defaults() {
    let mut cur: *mut sudo_defs_types = 0 as *mut sudo_defs_types;
    let mut item: *mut list_member = 0 as *mut list_member;
    let mut def: *mut def_values = 0 as *mut def_values;
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    cur = sudo_defs_table.as_mut_ptr();
    while !((*cur).name).is_null() {
        if !((*cur).desc).is_null() {
            desc = dcgettext(
                b"sudoers\0" as *const u8 as *const libc::c_char,
                (*cur).desc,
                5 as libc::c_int,
            );
            match (*cur).type_0 & 0xff as libc::c_int {
                T_FLAG => {
                    if (*cur).sd_un.flag != 0 {
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            desc,
                        );
                    }
                }
                T_STR => {
                    if !((*cur).sd_un.str_0).is_null() {
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            desc,
                            (*cur).sd_un.str_0,
                        );
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                T_LOGFAC => {
                    if (*cur).sd_un.ival != 0 {
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            desc,
                            logfac2str((*cur).sd_un.ival),
                        );
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                T_LOGPRI => {
                    if (*cur).sd_un.ival != 0 {
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            desc,
                            logpri2str((*cur).sd_un.ival),
                        );
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                T_INT => {
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        desc,
                        (*cur).sd_un.ival,
                    );
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                T_UINT => {
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        desc,
                        (*cur).sd_un.uival,
                    );
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                T_TIMESPEC => {
                    let mut d: libc::c_double = (*cur).sd_un.tspec.tv_sec as libc::c_double
                        + (*cur).sd_un.tspec.tv_nsec as libc::c_double / 1000000000.0f64;
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        desc,
                        d / 60.0f64,
                    );
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                T_MODE => {
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        desc,
                        (*cur).sd_un.mode,
                    );
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                T_LIST => {
                    if !((*cur).sd_un.list.slh_first).is_null() {
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            desc,
                        );
                        item = (*cur).sd_un.list.slh_first;
                        while !item.is_null() {
                            sudo_printf.expect("non-null function pointer")(
                                SUDO_CONV_INFO_MSG as libc::c_int,
                                b"\t%s\n\0" as *const u8 as *const libc::c_char,
                                (*item).value,
                            );
                            item = (*item).entries.sle_next;
                        }
                    }
                }
                T_TIMEOUT => {
                    if (*cur).sd_un.ival != 0 {
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            desc,
                            (*cur).sd_un.ival,
                        );
                        sudo_printf.expect("non-null function pointer")(
                            SUDO_CONV_INFO_MSG as libc::c_int,
                            b"\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                T_TUPLE => {
                    def = (*cur).values;
                    while !((*def).sval).is_null() {
                        if (*cur).sd_un.tuple as libc::c_uint == (*def).nval as libc::c_uint {
                            sudo_printf.expect("non-null function pointer")(
                                SUDO_CONV_INFO_MSG as libc::c_int,
                                desc,
                                (*def).sval,
                            );
                            break;
                        } else {
                            def = def.offset(1);
                        }
                    }
                    sudo_printf.expect("non-null function pointer")(
                        SUDO_CONV_INFO_MSG as libc::c_int,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {}
            } //end of match
        } //if
        cur = cur.offset(1);
    }
    debug_return!();
}
#[no_mangle]
pub unsafe extern "C" fn init_defaults() -> bool {
    static mut firsttime: libc::c_int = 1 as libc::c_int;
    let mut def: *mut sudo_defs_types = 0 as *mut sudo_defs_types;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    if firsttime == 0 {
        def = sudo_defs_table.as_mut_ptr();
        while !((*def).name).is_null() {
            free_defs_val((*def).type_0, &mut (*def).sd_un);
            def = def.offset(1);
        }
    }
    sudo_defs_table[117 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[100 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[4 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[7 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[12 as libc::c_int as usize].sd_un.tuple = once;
    sudo_defs_table[14 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[15 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[21 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[25 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[111 as libc::c_int as usize].sd_un.tuple = tty;
    sudo_defs_table[80 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"%{seq}\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[80 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[79 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"/var/log/utsudo-io\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[79 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[69 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"C\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[69 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[61 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[30 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[58 as libc::c_int as usize].sd_un.ival = 2 as libc::c_int + 1 as libc::c_int;
    sudo_defs_table[86 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"sudo\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[86 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[87 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"sudo-i\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[87 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[89 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[92 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[96 as libc::c_int as usize].sd_un.flag = 0 as libc::c_int;
    sudo_defs_table[93 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[104 as libc::c_int as usize].sd_un.mode =
        (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t;
    sudo_defs_table[105 as libc::c_int as usize].sd_un.tuple = digest_only;
    sudo_defs_table[115 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[116 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[119 as libc::c_int as usize].sd_un.flag = 0 as libc::c_int;
    store_syslogfac(
        b"authpriv\0" as *const u8 as *const libc::c_char,
        &mut (*sudo_defs_table
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .sd_un,
    );
    store_syslogpri(
        b"notice\0" as *const u8 as *const libc::c_char,
        &mut (*sudo_defs_table
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .sd_un,
    );
    store_syslogpri(
        b"alert\0" as *const u8 as *const libc::c_char,
        &mut (*sudo_defs_table
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize))
        .sd_un,
    );
    store_tuple(
        b"any\0" as *const u8 as *const libc::c_char,
        &mut (*sudo_defs_table
            .as_mut_ptr()
            .offset(54 as libc::c_int as isize))
        .sd_un,
        sudo_defs_table[54 as libc::c_int as usize].values,
    );
    store_tuple(
        b"all\0" as *const u8 as *const libc::c_char,
        &mut (*sudo_defs_table
            .as_mut_ptr()
            .offset(55 as libc::c_int as isize))
        .sd_un,
        sudo_defs_table[55 as libc::c_int as usize].values,
    );
    sudo_defs_table[37 as libc::c_int as usize].sd_un.mode = 0o22 as libc::c_int as mode_t;
    sudo_defs_table[33 as libc::c_int as usize].sd_un.uival = 80 as libc::c_int as libc::c_uint;
    sudo_defs_table[34 as libc::c_int as usize]
        .sd_un
        .tspec
        .tv_sec = (5 as libc::c_int * 60 as libc::c_int) as __time_t;
    sudo_defs_table[35 as libc::c_int as usize]
        .sd_un
        .tspec
        .tv_sec = (5 as libc::c_int * 60 as libc::c_int) as __time_t;
    sudo_defs_table[36 as libc::c_int as usize].sd_un.uival = 3 as libc::c_int as libc::c_uint;
    sudo_defs_table[76 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[97 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[98 as libc::c_int as usize].sd_un.flag = 0 as libc::c_int;
    sudo_defs_table[99 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[41 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"root\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[41 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[43 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"*** SECURITY information for %h ***\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[43 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[44 as libc::c_int as usize].sd_un.str_0 = strdup(dcgettext(
        b"sudoers\0" as *const u8 as *const libc::c_char,
        b"Sorry, try again.\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    ));
    if (sudo_defs_table[44 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[45 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"/var/db/sudo/lectured\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[45 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[46 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"/run/sudo/ts\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[46 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[49 as libc::c_int as usize].sd_un.str_0 = strdup(dcgettext(
        b"sudoers\0" as *const u8 as *const libc::c_char,
        b"[utsudo] password for %p: \0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    ));
    if (sudo_defs_table[49 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[51 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"root\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[51 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[39 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"/usr/sbin/sendmail\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[39 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[40 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"-t\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[40 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[53 as libc::c_int as usize].sd_un.str_0 =
        strdup(b"/bin/vi\0" as *const u8 as *const libc::c_char);
    if (sudo_defs_table[53 as libc::c_int as usize].sd_un.str_0).is_null() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    sudo_defs_table[81 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[90 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[88 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[101 as libc::c_int as usize].sd_un.uival = 960 as libc::c_int as libc::c_uint;
    sudo_defs_table[113 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    sudo_defs_table[114 as libc::c_int as usize].sd_un.flag = 1 as libc::c_int;
    if firsttime == 0 {
        if !sudoers_initlocale(
            0 as *const libc::c_char,
            sudo_defs_table[69 as libc::c_int as usize].sd_un.str_0,
        ) {
            //goto oom;
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            debug_return_bool!(false);
        }
    }
    if !init_envtables() {
        //goto oom;
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_bool!(false);
    }
    firsttime = 0;
    debug_return_bool!(true);
    //goto oom;
    //sudo_warnx!(b"%s: %s\0" as *const u8 as *const libc::c_char,get_function_name!(),b"unable to allocate memory\0" as *const u8 as *const libc::c_char);
    //debug_return_bool!(false);
}
#[no_mangle]
pub unsafe extern "C" fn check_defaults(
    mut parse_tree: *mut sudoers_parse_tree,
    mut quiet: bool,
) -> bool {
    let mut d: *mut defaults = 0 as *mut defaults;
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut idx: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_DEFAULTS!());
    d = (*parse_tree).defaults.tqh_first;
    while !d.is_null() {
        idx = find_default((*d).var, (*d).file, (*d).lineno, quiet);
        if idx != -(1 as libc::c_int) {
            let mut def: *mut sudo_defs_types =
                &mut *sudo_defs_table.as_mut_ptr().offset(idx as isize) as *mut sudo_defs_types;
            let mut sd_un: sudo_defs_val = sudo_defs_val { flag: 0 };
            memset(
                &mut sd_un as *mut sudo_defs_val as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sudo_defs_val>() as libc::c_ulong,
            );
            if parse_default_entry(
                def,
                (*d).val,
                (*d).op as libc::c_int,
                &mut sd_un,
                (*d).file,
                (*d).lineno,
                quiet,
            ) {
                free_defs_val((*def).type_0, &mut sd_un);
                d = (*d).entries.tqe_next;
                continue;
            }
        }
        (*d).error = true as libc::c_int as libc::c_char;
        ret = false;
        d = (*d).entries.tqe_next;
    }
    debug_return_bool!(ret);
}






