/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    unused_variables,
    unused_mut,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::common::*;
extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sudo_lbuf_error_v1(lbuf: *mut sudo_lbuf) -> bool;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn sudo_lbuf_print_v1(lbuf: *mut sudo_lbuf);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn sudo_lbuf_append_v1(lbuf: *mut sudo_lbuf, fmt: *const libc::c_char, _: ...) -> bool;
    fn alias_get(
        parse_tree: *mut sudoers_parse_tree,
        name: *const libc::c_char,
        type_0: libc::c_int,
    ) -> *mut alias;
    fn digest_type_to_name(digest_type: libc::c_int) -> *const libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_lbuf_append_quoted_v1(
        lbuf: *mut sudo_lbuf,
        set: *const libc::c_char,
        fmt: *const libc::c_char,
        _: ...
    ) -> bool;
    fn alias_put(a: *mut alias);
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut sudo_user: sudo_user;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub const MYSELF: libc::c_int = 298;
pub const COMMAND: libc::c_int = 257;
pub const USERGROUP: libc::c_int = 262;
pub const DEFAULTS_HOST: libc::c_short = 266;
pub const DEFAULTS_RUNAS: libc::c_short = 268;
pub const DEFAULTS_CMND: libc::c_short = 269;
pub const DEFAULTS_USER: libc::c_short = 267;


#[no_mangle]
unsafe extern "C" fn sudoers_format_member_int(
    mut lbuf: *mut sudo_lbuf,
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *mut libc::c_char,
    mut type_0: libc::c_int,
    mut negated: bool,
    mut separator: *const libc::c_char,
    mut alias_type: libc::c_int,
) -> bool {
    let mut a: *mut alias = 0 as *mut alias;
    let mut m: *mut member = 0 as *mut member;
    let mut c: *mut sudo_command = 0 as *mut sudo_command;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    match type_0 {
        ALL => {
            sudo_lbuf_append_v1(
                lbuf,
                b"%sALL\0" as *const u8 as *const libc::c_char,
                if negated as libc::c_int != 0 {
                    b"!\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        MYSELF => {
            sudo_lbuf_append_v1(
                lbuf,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                if negated as libc::c_int != 0 {
                    b"!\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if !(sudo_user.name).is_null() {
                    sudo_user.name
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        COMMAND => {
            c = name as *mut sudo_command;
            if !((*c).digest).is_null() {
                sudo_lbuf_append_v1(
                    lbuf,
                    b"%s:%s \0" as *const u8 as *const libc::c_char,
                    digest_type_to_name((*(*c).digest).digest_type as libc::c_int),
                    (*(*c).digest).digest_str,
                );
            }
            if negated {
                sudo_lbuf_append_v1(lbuf, b"!\0" as *const u8 as *const libc::c_char);
            }
            sudo_lbuf_append_quoted_v1(
                lbuf,
                b":\\,=#\" \t\0" as *const u8 as *const libc::c_char,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*c).cmnd,
            );
            if !((*c).args).is_null() {
                sudo_lbuf_append_v1(lbuf, b" \0" as *const u8 as *const libc::c_char);
                sudo_lbuf_append_quoted_v1(
                    lbuf,
                    b":\\,=#\"\0" as *const u8 as *const libc::c_char,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*c).args,
                );
            }
        }
        USERGROUP => {
            if (strpbrk(name, b" \t\0" as *const u8 as *const libc::c_char)).is_null() {
                name = name.offset(1);
                if *name as libc::c_int == ':' as i32 {
                    name = name.offset(1);
                    sudo_lbuf_append_v1(
                        lbuf,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        b"%:\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    sudo_lbuf_append_v1(
                        lbuf,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        b"%\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            //goto
            if *name.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
                && *name.offset(
                    (strspn(
                        name.offset(1 as libc::c_int as isize),
                        b"0123456789\0" as *const u8 as *const libc::c_char,
                    ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int
                    == '\u{0}' as i32
            {
                sudo_lbuf_append_v1(
                    lbuf,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    if negated as libc::c_int != 0 {
                        b"!\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    name,
                );
            } else {
                if !(strpbrk(name, b" \t\0" as *const u8 as *const libc::c_char)).is_null() {
                    sudo_lbuf_append_v1(
                        lbuf,
                        b"%s\"\0" as *const u8 as *const libc::c_char,
                        if negated as libc::c_int != 0 {
                            b"!\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                    sudo_lbuf_append_quoted_v1(
                        lbuf,
                        b"\"\0" as *const u8 as *const libc::c_char,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                    sudo_lbuf_append_v1(lbuf, b"\"\0" as *const u8 as *const libc::c_char);
                } else {
                    sudo_lbuf_append_quoted_v1(
                        lbuf,
                        b":\\,=#\"\0" as *const u8 as *const libc::c_char,
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        if negated as libc::c_int != 0 {
                            b"!\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        name,
                    );
                }
            }
            //end of goto
        }
        ALIAS => {
            a = alias_get(parse_tree, name, alias_type);
            if alias_type != UNSPEC as libc::c_int && !a.is_null() {
                m = (*a).members.tqh_first;
                while !m.is_null() {
                    if m != (*a).members.tqh_first {
                        sudo_lbuf_append_v1(
                            lbuf,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            separator,
                        );
                    }
                    sudoers_format_member_int(
                        lbuf,
                        parse_tree,
                        (*m).name,
                        (*m).type0 as libc::c_int,
                        if negated as libc::c_int != 0 {
                            ((*m).negated == 0) as libc::c_int
                        } else {
                            (*m).negated as libc::c_int
                        } != 0,
                        separator,
                        alias_type,
                    );
                    m = (*m).entries.tqe_next;
                }
                alias_put(a);
            } else {
                //goto
                if *name.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
                    && *name.offset(
                        (strspn(
                            name.offset(1 as libc::c_int as isize),
                            b"0123456789\0" as *const u8 as *const libc::c_char,
                        ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int
                        == '\u{0}' as i32
                {
                    sudo_lbuf_append_v1(
                        lbuf,
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        if negated as libc::c_int != 0 {
                            b"!\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        name,
                    );
                } else {
                    if !(strpbrk(name, b" \t\0" as *const u8 as *const libc::c_char)).is_null() {
                        sudo_lbuf_append_v1(
                            lbuf,
                            b"%s\"\0" as *const u8 as *const libc::c_char,
                            if negated as libc::c_int != 0 {
                                b"!\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                        );
                        sudo_lbuf_append_quoted_v1(
                            lbuf,
                            b"\"\0" as *const u8 as *const libc::c_char,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                        sudo_lbuf_append_v1(lbuf, b"\"\0" as *const u8 as *const libc::c_char);
                    } else {
                        sudo_lbuf_append_quoted_v1(
                            lbuf,
                            b":\\,=#\"\0" as *const u8 as *const libc::c_char,
                            b"%s%s\0" as *const u8 as *const libc::c_char,
                            if negated as libc::c_int != 0 {
                                b"!\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            name,
                        );
                    }
                }
                //end of goto
            }
        }
        _ => {
            if *name.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
                && *name.offset(
                    (strspn(
                        name.offset(1 as libc::c_int as isize),
                        b"0123456789\0" as *const u8 as *const libc::c_char,
                    ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int
                    == '\u{0}' as i32
            {
                sudo_lbuf_append_v1(
                    lbuf,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    if negated as libc::c_int != 0 {
                        b"!\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    name,
                );
            } else {
                if !(strpbrk(name, b" \t\0" as *const u8 as *const libc::c_char)).is_null() {
                    sudo_lbuf_append_v1(
                        lbuf,
                        b"%s\"\0" as *const u8 as *const libc::c_char,
                        if negated as libc::c_int != 0 {
                            b"!\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                    sudo_lbuf_append_quoted_v1(
                        lbuf,
                        b"\"\0" as *const u8 as *const libc::c_char,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                    sudo_lbuf_append_v1(lbuf, b"\"\0" as *const u8 as *const libc::c_char);
                } else {
                    sudo_lbuf_append_quoted_v1(
                        lbuf,
                        b":\\,=#\"\0" as *const u8 as *const libc::c_char,
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        if negated as libc::c_int != 0 {
                            b"!\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        name,
                    );
                }
            }
        } //end of default
    }
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
