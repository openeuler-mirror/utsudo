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

#[no_mangle]
pub unsafe extern "C" fn sudoers_format_member(
    mut lbuf: *mut sudo_lbuf,
    mut parse_tree: *mut sudoers_parse_tree,
    mut m: *mut member,
    mut separator: *const libc::c_char,
    mut alias_type: libc::c_int,
) -> bool {
    return sudoers_format_member_int(
        lbuf,
        parse_tree,
        (*m).name,
        (*m).type0 as libc::c_int,
        (*m).negated != 0,
        separator,
        alias_type,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_defaults_to_tags(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut op: libc::c_int,
    mut tags: *mut cmndtag,
) -> bool {
    let mut ret: bool = 1 as libc::c_int != 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if op == 1 as libc::c_int || op == 0 as libc::c_int {
        if strcmp(var, b"authenticate\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*tags).set_nopasswd((op == 0 as libc::c_int) as libc::c_int);
        } else if strcmp(
            var,
            b"sudoedit_follow\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*tags).set_follow((op == 1 as libc::c_int) as libc::c_int);
        } else if strcmp(var, b"log_input\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*tags).set_log_input((op == 1 as libc::c_int) as libc::c_int);
        } else if strcmp(var, b"log_output\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*tags).set_log_output((op == 1 as libc::c_int) as libc::c_int);
        } else if strcmp(var, b"noexec\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*tags).set_noexec((op == 1 as libc::c_int) as libc::c_int);
        } else if strcmp(var, b"setenv\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*tags).set_setenv((op == 1 as libc::c_int) as libc::c_int);
        } else if strcmp(var, b"mail_all_cmnds\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(var, b"mail_always\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(var, b"mail_no_perms\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            (*tags).set_send_mail((op == 1 as libc::c_int) as libc::c_int);
        } else {
            ret = 0 as libc::c_int != 0;
        }
    } else {
        ret = false;
    }
    debug_return_bool!(ret);
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_defaults_list_to_tags(
    mut defs: *mut defaults_list,
    mut tags: *mut cmndtag,
) -> bool {
    let mut ret: bool = 1 as libc::c_int != 0;
    let mut d: *mut defaults = 0 as *mut defaults;
    let sudo_debug_subsys_1: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(17 as libc::c_int as isize)
        as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    (*tags).set_follow(-(1 as libc::c_int));
    (*tags).set_log_input(-(1 as libc::c_int));
    (*tags).set_log_output(-(1 as libc::c_int));
    (*tags).set_noexec(-(1 as libc::c_int));
    (*tags).set_nopasswd(-(1 as libc::c_int));
    (*tags).set_send_mail(-(1 as libc::c_int));
    (*tags).set_setenv(-(1 as libc::c_int));
    if !defs.is_null() {
        d = (*defs).tqh_first;
        while !d.is_null() {
            if !sudoers_defaults_to_tags((*d).var, (*d).val, (*d).op as libc::c_int, tags) {
                if !((*d).val).is_null() {
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN,
                        b"unable to convert defaults to tag: %s%s%s\0" as *const u8
                            as *const libc::c_char,
                        (*d).var,
                        if (*d).op as libc::c_int == '+' as i32 {
                            b"+=\0" as *const u8 as *const libc::c_char
                        } else if (*d).op as libc::c_int == '-' as i32 {
                            b"-=\0" as *const u8 as *const libc::c_char
                        } else {
                            b"=\0" as *const u8 as *const libc::c_char
                        },
                        (*d).val
                    );
                } else {
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN,
                        b"unable to convert defaults to tag: %s%s%s\0" as *const u8
                            as *const libc::c_char,
                        if (*d).op as libc::c_int == 0 as libc::c_int {
                            b"!\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        (*d).var,
                        b"\0" as *const u8 as *const libc::c_char
                    );
                }
                ret = false;
            }
            d = (*d).entries.tqe_next;
        }
    }
    debug_return_bool!(ret);
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_format_cmndspec(
    mut lbuf: *mut sudo_lbuf,
    mut parse_tree: *mut sudoers_parse_tree,
    mut cs: *mut cmndspec,
    mut prev_cs: *mut cmndspec,
    mut tags: cmndtag,
    mut expand_aliases: bool,
) -> bool {
    //let sudo_debug_subsys: libc::c_int = *sudoers_subsystem_ids.as_mut_ptr().offset(17 as libc::c_int as isize) as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    //TAGS_MEGE
    if ((*cs).tags).follow() != -(1 as libc::c_int) {
        tags.set_follow(((*cs).tags).follow());
    }
    if ((*cs).tags).log_input() != -(1 as libc::c_int) {
        tags.set_log_input(((*cs).tags).log_input());
    }
    if ((*cs).tags).log_output() != -(1 as libc::c_int) {
        tags.set_log_output(((*cs).tags).log_output());
    }
    if ((*cs).tags).noexec() != -(1 as libc::c_int) {
        tags.set_noexec(((*cs).tags).noexec());
    }
    if ((*cs).tags).nopasswd() != -(1 as libc::c_int) {
        tags.set_nopasswd(((*cs).tags).nopasswd());
    }
    if ((*cs).tags).send_mail() != -(1 as libc::c_int) {
        tags.set_send_mail(((*cs).tags).send_mail());
    }
    if ((*cs).tags).setenv() != -(1 as libc::c_int) {
        tags.set_setenv(((*cs).tags).setenv());
    }
    if !((*cs).role).is_null() && (prev_cs.is_null() || (*cs).role != (*prev_cs).role) {
        sudo_lbuf_append_v1(
            lbuf,
            b"ROLE=%s \0" as *const u8 as *const libc::c_char,
            (*cs).role,
        );
    }
    if !((*cs).type_0).is_null() && (prev_cs.is_null() || (*cs).type_0 != (*prev_cs).type_0) {
        sudo_lbuf_append_v1(
            lbuf,
            b"TYPE=%s \0" as *const u8 as *const libc::c_char,
            (*cs).type_0,
        );
    }
    if (*cs).timeout > 0 as libc::c_int
        && (prev_cs.is_null() || (*cs).timeout != (*prev_cs).timeout)
    {
        let mut numbuf: [libc::c_char; 13] = [0; 13];
        snprintf(
            numbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*cs).timeout,
        );
        sudo_lbuf_append_v1(
            lbuf,
            b"TIMEOUT=%s \0" as *const u8 as *const libc::c_char,
            numbuf.as_mut_ptr(),
        );
    }
    if (*cs).notbefore != UNSPEC as libc::c_int as libc::c_long
        && (prev_cs.is_null() || (*cs).notbefore != (*prev_cs).notbefore)
    {
        let mut buf: [libc::c_char; 16] = [0; 16];
        let mut tm: *mut tm = gmtime(&mut (*cs).notbefore);
        if strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%Y%m%d%H%M%SZ\0" as *const u8 as *const libc::c_char,
            tm,
        ) != 0 as libc::c_int as libc::c_ulong
        {
            sudo_lbuf_append_v1(
                lbuf,
                b"NOTBEFORE=%s \0" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
            );
        }
    }
    if (*cs).notafter != UNSPEC as libc::c_int as libc::c_long
        && (prev_cs.is_null() || (*cs).notafter != (*prev_cs).notafter)
    {
        let mut buf_0: [libc::c_char; 16] = [0; 16];
        let mut tm_0: *mut tm = gmtime(&mut (*cs).notafter);
        if strftime(
            buf_0.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%Y%m%d%H%M%SZ\0" as *const u8 as *const libc::c_char,
            tm_0,
        ) != 0 as libc::c_int as libc::c_ulong
        {
            sudo_lbuf_append_v1(
                lbuf,
                b"NOTAFTER=%s \0" as *const u8 as *const libc::c_char,
                buf_0.as_mut_ptr(),
            );
        }
    }
    if tags.setenv() != -(1 as libc::c_int)
        && tags.setenv() != 2 as libc::c_int
        && (prev_cs.is_null() || ((*cs).tags).setenv() != ((*prev_cs).tags).setenv())
    {
        sudo_lbuf_append_v1(
            lbuf,
            if tags.setenv() != 0 {
                b"SETENV: \0" as *const u8 as *const libc::c_char
            } else {
                b"NOSETENV: \0" as *const u8 as *const libc::c_char
            },
        );
    }
    if tags.noexec() != -(1 as libc::c_int)
        && tags.noexec() != 2 as libc::c_int
        && (prev_cs.is_null() || ((*cs).tags).noexec() != ((*prev_cs).tags).noexec())
    {
        sudo_lbuf_append_v1(
            lbuf,
            if tags.noexec() != 0 {
                b"NOEXEC: \0" as *const u8 as *const libc::c_char
            } else {
                b"EXEC: \0" as *const u8 as *const libc::c_char
            },
        );
    }
    if tags.nopasswd() != -(1 as libc::c_int)
        && tags.nopasswd() != 2 as libc::c_int
        && (prev_cs.is_null() || ((*cs).tags).nopasswd() != ((*prev_cs).tags).nopasswd())
    {
        sudo_lbuf_append_v1(
            lbuf,
            if tags.nopasswd() != 0 {
                b"NOPASSWD: \0" as *const u8 as *const libc::c_char
            } else {
                b"PASSWD: \0" as *const u8 as *const libc::c_char
            },
        );
    }
    if tags.log_input() != -(1 as libc::c_int)
        && tags.log_input() != 2 as libc::c_int
        && (prev_cs.is_null() || ((*cs).tags).log_input() != ((*prev_cs).tags).log_input())
    {
        sudo_lbuf_append_v1(
            lbuf,
            if tags.log_input() != 0 {
                b"LOG_INPUT: \0" as *const u8 as *const libc::c_char
            } else {
                b"NOLOG_INPUT: \0" as *const u8 as *const libc::c_char
            },
        );
    }
    if tags.log_output() != -(1 as libc::c_int)
        && tags.log_output() != 2 as libc::c_int
        && (prev_cs.is_null() || ((*cs).tags).log_output() != ((*prev_cs).tags).log_output())
    {
        sudo_lbuf_append_v1(
            lbuf,
            if tags.log_output() != 0 {
                b"LOG_OUTPUT: \0" as *const u8 as *const libc::c_char
            } else {
                b"NOLOG_OUTPUT: \0" as *const u8 as *const libc::c_char
            },
        );
    }
    if tags.send_mail() != -(1 as libc::c_int)
        && tags.send_mail() != 2 as libc::c_int
        && (prev_cs.is_null() || ((*cs).tags).send_mail() != ((*prev_cs).tags).send_mail())
    {
        sudo_lbuf_append_v1(
            lbuf,
            if tags.send_mail() != 0 {
                b"MAIL: \0" as *const u8 as *const libc::c_char
            } else {
                b"NOMAIL: \0" as *const u8 as *const libc::c_char
            },
        );
    }
    if tags.follow() != -(1 as libc::c_int)
        && tags.follow() != 2 as libc::c_int
        && (prev_cs.is_null() || ((*cs).tags).follow() != ((*prev_cs).tags).follow())
    {
        sudo_lbuf_append_v1(
            lbuf,
            if tags.follow() != 0 {
                b"FOLLOW: \0" as *const u8 as *const libc::c_char
            } else {
                b"NOFOLLOW: \0" as *const u8 as *const libc::c_char
            },
        );
    }
    sudoers_format_member(
        lbuf,
        parse_tree,
        (*cs).cmnd,
        b", \0" as *const u8 as *const libc::c_char,
        if expand_aliases as libc::c_int != 0 {
            CMNDALIAS
        } else {
            UNSPEC
        },
    );
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_format_privilege(
    mut lbuf: *mut sudo_lbuf,
    mut parse_tree: *mut sudoers_parse_tree,
    mut priv_0: *mut privilege,
    mut expand_aliases: bool,
) -> bool {
    let mut cs: *mut cmndspec = 0 as *mut cmndspec;
    let mut prev_cs: *mut cmndspec = 0 as *mut cmndspec;
    //init
    let mut tags: cmndtag = cmndtag {
        nopasswd_noexec_setenv_log_input_log_output_send_mail_follow: [0; 3],
        c2rust_padding: [0; 1],
    };
    let mut m: *mut member = 0 as *mut member;
    //let sudo_debug_subsys: libc::c_int = *sudoers_subsystem_ids.as_mut_ptr().offset(17 as libc::c_int as isize) as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    sudoers_defaults_list_to_tags(&mut (*priv_0).defaults, &mut tags);
    m = (*priv_0).hostlist.tqh_first;
    while !m.is_null() {
        if m != (*priv_0).hostlist.tqh_first {
            sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
        }
        sudoers_format_member(
            lbuf,
            parse_tree,
            m,
            b", \0" as *const u8 as *const libc::c_char,
            if expand_aliases as libc::c_int != 0 {
                HOSTALIAS as libc::c_int
            } else {
                UNSPEC as libc::c_int
            },
        );
        m = (*m).entries.tqe_next;
    }
    sudo_lbuf_append_v1(lbuf, b" = \0" as *const u8 as *const libc::c_char);
    prev_cs = 0 as *mut cmndspec;
    cs = (*priv_0).cmndlist.tqh_first;
    while !cs.is_null() {
        if prev_cs.is_null()
            || ((*cs).runasuserlist != (*prev_cs).runasuserlist
                || (*cs).runasgrouplist != (*prev_cs).runasgrouplist)
        {
            if cs != (*priv_0).cmndlist.tqh_first {
                sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
            }
            if !((*cs).runasuserlist).is_null() || !((*cs).runasgrouplist).is_null() {
                sudo_lbuf_append_v1(lbuf, b"(\0" as *const u8 as *const libc::c_char);
            }
            if !((*cs).runasuserlist).is_null() {
                m = (*(*cs).runasuserlist).tqh_first;
                while !m.is_null() {
                    if m != (*(*cs).runasuserlist).tqh_first {
                        sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
                    }
                    sudoers_format_member(
                        lbuf,
                        parse_tree,
                        m,
                        b", \0" as *const u8 as *const libc::c_char,
                        if expand_aliases as libc::c_int != 0 {
                            RUNASALIAS as libc::c_int
                        } else {
                            UNSPEC as libc::c_int
                        },
                    );
                    m = (*m).entries.tqe_next;
                }
            }
            if !((*cs).runasgrouplist).is_null() {
                sudo_lbuf_append_v1(lbuf, b" : \0" as *const u8 as *const libc::c_char);
                m = (*(*cs).runasgrouplist).tqh_first;
                while !m.is_null() {
                    if m != (*(*cs).runasgrouplist).tqh_first {
                        sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
                    }
                    sudoers_format_member(
                        lbuf,
                        parse_tree,
                        m,
                        b", \0" as *const u8 as *const libc::c_char,
                        if expand_aliases as libc::c_int != 0 {
                            RUNASALIAS as libc::c_int
                        } else {
                            UNSPEC as libc::c_int
                        },
                    );
                    m = (*m).entries.tqe_next;
                }
            }
            if !((*cs).runasuserlist).is_null() || !((*cs).runasgrouplist).is_null() {
                sudo_lbuf_append_v1(lbuf, b") \0" as *const u8 as *const libc::c_char);
            }
        } else if cs != (*priv_0).cmndlist.tqh_first {
            sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
        }
        sudoers_format_cmndspec(lbuf, parse_tree, cs, prev_cs, tags, expand_aliases);
        prev_cs = cs;
        cs = (*cs).entries.tqe_next;
    } //end of start while ,same as c:for
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_format_userspec(
    mut lbuf: *mut sudo_lbuf,
    mut parse_tree: *mut sudoers_parse_tree,
    mut us: *mut userspec,
    mut expand_aliases: bool,
) -> bool {
    let mut priv_0: *mut privilege = 0 as *mut privilege;
    let mut comment: *mut sudoers_comment = 0 as *mut sudoers_comment;
    let mut m: *mut member = 0 as *mut member;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    comment = (*us).comments.stqh_first;
    while !comment.is_null() {
        sudo_lbuf_append_v1(
            lbuf,
            b"# %s\n\0" as *const u8 as *const libc::c_char,
            (*comment).str_0,
        );
        comment = (*comment).entries.stqe_next;
    }
    m = (*us).users.tqh_first;
    while !m.is_null() {
        if m != (*us).users.tqh_first {
            sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
        }
        sudoers_format_member(
            lbuf,
            parse_tree,
            m,
            b", \0" as *const u8 as *const libc::c_char,
            if expand_aliases as libc::c_int != 0 {
                USERALIAS as libc::c_int
            } else {
                UNSPEC as libc::c_int
            },
        );
        m = (*m).entries.tqe_next;
    }
    priv_0 = (*us).privileges.tqh_first;
    while !priv_0.is_null() {
        if priv_0 != (*us).privileges.tqh_first {
            sudo_lbuf_append_v1(lbuf, b" : \0" as *const u8 as *const libc::c_char);
        } else {
            sudo_lbuf_append_v1(lbuf, b" \0" as *const u8 as *const libc::c_char);
        }
        if !sudoers_format_privilege(lbuf, parse_tree, priv_0, expand_aliases) {
            break;
        }
        priv_0 = (*priv_0).entries.tqe_next;
    }
    sudo_lbuf_append_v1(lbuf, b"\n\0" as *const u8 as *const libc::c_char);
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_format_userspecs(
    mut lbuf: *mut sudo_lbuf,
    mut parse_tree: *mut sudoers_parse_tree,
    mut separator: *const libc::c_char,
    mut expand_aliases: bool,
    mut flush: bool,
) -> bool {
    let mut us: *mut userspec = 0 as *mut userspec;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    us = (*parse_tree).userspecs.tqh_first;
    while !us.is_null() {
        if !separator.is_null() && us != (*parse_tree).userspecs.tqh_first {
            sudo_lbuf_append_v1(lbuf, b"%s\0" as *const u8 as *const libc::c_char, separator);
        }
        if !sudoers_format_userspec(lbuf, parse_tree, us, expand_aliases) {
            break;
        }
        sudo_lbuf_print_v1(lbuf);
        us = (*us).entries.tqe_next;
    }
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_format_default(
    mut lbuf: *mut sudo_lbuf,
    mut d: *mut defaults,
) -> bool {
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if !((*d).val).is_null() {
        sudo_lbuf_append_v1(
            lbuf,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*d).var,
            if (*d).op as libc::c_int == '+' as i32 {
                b"+=\0" as *const u8 as *const libc::c_char
            } else if (*d).op as libc::c_int == '-' as i32 {
                b"-=\0" as *const u8 as *const libc::c_char
            } else {
                b"=\0" as *const u8 as *const libc::c_char
            },
        );
        if !(strpbrk((*d).val, b" \t\0" as *const u8 as *const libc::c_char)).is_null() {
            sudo_lbuf_append_v1(lbuf, b"\"\0" as *const u8 as *const libc::c_char);
            sudo_lbuf_append_quoted_v1(
                lbuf,
                b"\"\0" as *const u8 as *const libc::c_char,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*d).val,
            );
            sudo_lbuf_append_v1(lbuf, b"\"\0" as *const u8 as *const libc::c_char);
        } else {
            sudo_lbuf_append_quoted_v1(
                lbuf,
                b":\\,=#\"\0" as *const u8 as *const libc::c_char,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*d).val,
            );
        }
    } else {
        sudo_lbuf_append_v1(
            lbuf,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            if (*d).op as libc::c_int == 0 as libc::c_int {
                b"!\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*d).var,
        );
    }
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_format_default_line(
    mut lbuf: *mut sudo_lbuf,
    mut parse_tree: *mut sudoers_parse_tree,
    mut d: *mut defaults,
    mut next: *mut *mut defaults,
    mut expand_aliases: bool,
) -> bool {
    let mut m: *mut member = 0 as *mut member;
    let mut alias_type: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    match (*d).type_0 {
        DEFAULTS_HOST => {
            sudo_lbuf_append_v1(lbuf, b"Defaults@\0" as *const u8 as *const libc::c_char);
            alias_type = if expand_aliases as libc::c_int != 0 {
                HOSTALIAS as libc::c_int
            } else {
                UNSPEC as libc::c_int
            };
        }
        DEFAULTS_USER => {
            sudo_lbuf_append_v1(lbuf, b"Defaults:\0" as *const u8 as *const libc::c_char);
            alias_type = if expand_aliases as libc::c_int != 0 {
                USERALIAS as libc::c_int
            } else {
                UNSPEC as libc::c_int
            };
        }
        DEFAULTS_RUNAS => {
            sudo_lbuf_append_v1(lbuf, b"Defaults>\0" as *const u8 as *const libc::c_char);
            alias_type = if expand_aliases as libc::c_int != 0 {
                RUNASALIAS as libc::c_int
            } else {
                UNSPEC as libc::c_int
            };
        }
        DEFAULTS_CMND => {
            sudo_lbuf_append_v1(lbuf, b"Defaults!\0" as *const u8 as *const libc::c_char);
            alias_type = if expand_aliases as libc::c_int != 0 {
                CMNDALIAS as libc::c_int
            } else {
                UNSPEC as libc::c_int
            };
        }
        _ => {
            sudo_lbuf_append_v1(lbuf, b"Defaults\0" as *const u8 as *const libc::c_char);
            alias_type = -(1 as libc::c_int);
        }
    } //end of match
    m = (*(*d).binding).tqh_first;
    while !m.is_null() {
        if m != (*(*d).binding).tqh_first {
            sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
        }
        sudoers_format_member(
            lbuf,
            parse_tree,
            m,
            b", \0" as *const u8 as *const libc::c_char,
            alias_type,
        );
        m = (*m).entries.tqe_next;
    }
    sudo_lbuf_append_v1(lbuf, b" \0" as *const u8 as *const libc::c_char);
    sudoers_format_default(lbuf, d);
    if !next.is_null() {
        let mut n: *mut defaults = 0 as *mut defaults;
        loop {
            n = (*d).entries.tqe_next;
            if !(!n.is_null() && (*d).binding == (*n).binding) {
                break;
            }
            sudo_lbuf_append_v1(lbuf, b", \0" as *const u8 as *const libc::c_char);
            sudoers_format_default(lbuf, n);
            d = n;
        }
        *next = n;
    }
    sudo_lbuf_append_v1(lbuf, b"\n\0" as *const u8 as *const libc::c_char);
    debug_return_bool!(!sudo_lbuf_error_v1(lbuf));
}

