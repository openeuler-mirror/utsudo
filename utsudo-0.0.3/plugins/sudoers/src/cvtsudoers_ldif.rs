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
    clashing_extern_declarations
)]
use crate::common::*;
extern "C" {
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn alias_get(
        parse_tree: *mut sudoers_parse_tree,
        name: *const libc::c_char,
        type_0: libc::c_int,
    ) -> *mut alias;
    fn alias_put(a: *mut alias);
    fn base64_encode(
        in_0: *const libc::c_uchar,
        in_len: size_t,
        out: *mut libc::c_char,
        out_len: size_t,
    ) -> size_t;
    fn digest_type_to_name(digest_type: libc::c_int) -> *const libc::c_char;
    fn sudoers_format_default_line(
        lbuf: *mut sudo_lbuf,
        parse_tree: *mut sudoers_parse_tree,
        d: *mut defaults,
        next: *mut *mut defaults,
        expand_aliases: bool,
    ) -> bool;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn rbfind(_: *mut rbtree, _: *mut libc::c_void) -> *mut rbnode;
    fn rbinsert(_: *mut rbtree, _: *mut libc::c_void, _: *mut *mut rbnode) -> libc::c_int;
    fn rbcreate(
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    ) -> *mut rbtree;
    fn rbdestroy(_: *mut rbtree, _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>);
    fn sudo_lbuf_init_v1(
        lbuf: *mut sudo_lbuf,
        output: sudo_lbuf_output_t,
        indent: libc::c_int,
        continuation: *const libc::c_char,
        cols: libc::c_int,
    );
    fn sudo_lbuf_destroy_v1(lbuf: *mut sudo_lbuf);
    fn sudo_lbuf_append_v1(lbuf: *mut sudo_lbuf, fmt: *const libc::c_char, _: ...) -> bool;
}
pub const DEFAULTS: libc::c_int = 265;
pub const IMPLIED: libc::c_int = 2;
pub const SUPPRESS_DEFAULTS: libc::c_int = 0x01;
pub const SUPPRESS_PRIVS: libc::c_int = 0x04;
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
pub struct seen_user {
    pub name: *const libc::c_char,
    pub count: libc::c_ulong,
}
pub type sudo_lbuf_output_t = Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>;
static mut seen_users: *mut rbtree = 0 as *const rbtree as *mut rbtree;


unsafe extern "C" fn seen_user_compare(
    mut aa: *const libc::c_void,
    mut bb: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const seen_user = aa as *const seen_user;
    let mut b: *const seen_user = bb as *const seen_user;
    return strcasecmp((*a).name, (*b).name);
}
unsafe extern "C" fn seen_user_free(mut v: *mut libc::c_void) {
    let mut su: *mut seen_user = v as *mut seen_user;
    free((*su).name as *mut libc::c_void);
    free(su as *mut libc::c_void);
}
unsafe extern "C" fn safe_string(mut str: *const libc::c_char) -> bool {
    str = str.offset(1);
    let mut ch: libc::c_uint = *str as libc::c_uint;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    /* Initial char must be <= 127 and not LF, CR, SPACE, ':', '<' */
    match ch as u8 as char {
        '\0' => {
            debug_return_bool!(true);
        }
        '\n' | '\r' | ' ' | ':' | '<' => {
            debug_return_bool!(false);
        }
        _ => {
            if ch > 127 {
                debug_return_bool!(false);
            }
        }
    }
    /* Any value <= 127 decimal except NUL, LF, and CR is safe */
    loop {
        str = str.offset(1);
        ch = *str as libc::c_uint;
        if !(ch != '\0' as i32 as libc::c_uint) {
            break;
        }
        if ch > 127 as libc::c_uint
            || ch == '\n' as i32 as libc::c_uint
            || ch == '\r' as i32 as libc::c_uint
        {
            debug_return_bool!(false);
        }
    }
    debug_return_bool!(true);
}
unsafe extern "C" fn print_attribute_ldif(
    mut fp: *mut FILE,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> bool {
    let mut uvalue: *const libc::c_uchar = value as *mut libc::c_uchar;
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut esize: size_t = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    if !safe_string(value) {
        let vlen: size_t = strlen(value);
        esize = vlen
            .wrapping_add(2 as libc::c_ulong)
            .wrapping_div(3 as libc::c_ulong)
            .wrapping_mul(4 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong);
        encoded = malloc(esize) as *mut libc::c_char;
        if encoded.is_null() {
            debug_return_bool!(false);
        }
        if base64_encode(uvalue, vlen, encoded, esize) == -(1 as libc::c_int) as size_t {
            free(encoded as *mut libc::c_void);
            debug_return_bool!(false);
        }
        fprintf(
            fp,
            b"%s:: %s\n\0" as *const u8 as *const libc::c_char,
            name,
            encoded,
        );
        free(encoded as *mut libc::c_void);
    } else if *value as libc::c_int != '\0' as i32 {
        fprintf(
            fp,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            name,
            value,
        );
    } else {
        fprintf(fp, b"%s:\n\0" as *const u8 as *const libc::c_char, name);
    }
    debug_return_bool!(true);
}
/*
 * Print sudoOptions from a defaults_list.
 */
unsafe extern "C" fn print_options_ldif(
    mut fp: *mut FILE,
    mut options: *mut defaults_list,
) -> bool {
    let mut opt: *mut defaults = 0 as *mut defaults;
    let mut attr_val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    opt = (*options).tqh_first;
    while !opt.is_null() {
        if !((*opt).type_0 as libc::c_int != DEFAULTS) {
            if !((*opt).val).is_null() {
                /* There is no need to double quote values here. */
                len = asprintf(
                    &mut attr_val as *mut *mut libc::c_char,
                    b"%s%s%s\0" as *const u8 as *const libc::c_char,
                    (*opt).var,
                    if (*opt).op as libc::c_int == '+' as i32 {
                        b"+=\0" as *const u8 as *const libc::c_char
                    } else if (*opt).op as libc::c_int == '-' as i32 {
                        b"-=\0" as *const u8 as *const libc::c_char
                    } else {
                        b"=\0" as *const u8 as *const libc::c_char
                    },
                    (*opt).val,
                );
            } else {
                /* Boolean flag. */
                len = asprintf(
                    &mut attr_val as *mut *mut libc::c_char,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    if (*opt).op as libc::c_int == false as libc::c_int {
                        b"!\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (*opt).var,
                );
            }
            if len == -(1 as libc::c_int) {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
            print_attribute_ldif(
                fp,
                b"sudoOption\0" as *const u8 as *const libc::c_char,
                attr_val,
            );
            free(attr_val as *mut libc::c_void);
        }
        opt = (*opt).entries.tqe_next;
    }
    debug_return_bool!(ferror(fp) == 0);
}
/*
 * Print global Defaults in a single sudoRole object.
 */
unsafe extern "C" fn print_global_defaults_ldif(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut base: *const libc::c_char,
) -> bool {
    let mut count: libc::c_uint = 0 as libc::c_uint;
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
    let mut opt: *mut defaults = 0 as *mut defaults;
    let mut dn: *mut libc::c_char = 0 as *mut libc::c_char;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    sudo_lbuf_init_v1(
        &mut lbuf,
        None,
        0 as libc::c_int,
        0 as *const libc::c_char,
        80 as libc::c_int,
    );
    opt = (*parse_tree).defaults.tqh_first;
    while !opt.is_null() {
        /* Skip bound Defaults (unsupported). */
        if (*opt).type_0 as libc::c_int == DEFAULTS {
            count = count.wrapping_add(1);
        } else {
            lbuf.len = 0 as libc::c_int;
            sudo_lbuf_append_v1(
                &mut lbuf as *mut sudo_lbuf,
                b"# \0" as *const u8 as *const libc::c_char,
            );
            sudoers_format_default_line(
                &mut lbuf,
                parse_tree,
                opt,
                false as libc::c_int as *mut *mut defaults,
                true,
            );
            fprintf(
                fp,
                b"# Unable to translate %s:%d\n%s\n\0" as *const u8 as *const libc::c_char,
                (*opt).file,
                (*opt).lineno,
                lbuf.buf,
            );
        }
        opt = (*opt).entries.tqe_next;
    }
    sudo_lbuf_destroy_v1(&mut lbuf);
    if count == 0 as libc::c_uint {
        debug_return_bool!(true);
    }
    if asprintf(
        &mut dn as *mut *mut libc::c_char,
        b"cn=defaults,%s\0" as *const u8 as *const libc::c_char,
        base,
    ) == -(1 as libc::c_int)
    {
        sudo_fatalx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            get_function_name!(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
    }
    print_attribute_ldif(fp, b"dn\0" as *const u8 as *const libc::c_char, dn);
    free(dn as *mut libc::c_void);
    print_attribute_ldif(
        fp,
        b"objectClass\0" as *const u8 as *const libc::c_char,
        b"top\0" as *const u8 as *const libc::c_char,
    );
    print_attribute_ldif(
        fp,
        b"objectClass\0" as *const u8 as *const libc::c_char,
        b"sudoRole\0" as *const u8 as *const libc::c_char,
    );
    print_attribute_ldif(
        fp,
        b"cn\0" as *const u8 as *const libc::c_char,
        b"defaults\0" as *const u8 as *const libc::c_char,
    );
    print_attribute_ldif(
        fp,
        b"description\0" as *const u8 as *const libc::c_char,
        b"Default sudoOption's go here\0" as *const u8 as *const libc::c_char,
    );
    print_options_ldif(fp, &mut (*parse_tree).defaults);
    putc('\n' as i32, fp);
    debug_return_bool!(ferror(fp) == 0);
}
/*
 * Print struct member in LDIF format as the specified attribute.
 * See print_member_int() in parse.c.
 */
unsafe extern "C" fn print_member_ldif(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *mut libc::c_char,
    mut type_0: libc::c_int,
    mut negated: bool,
    mut alias_type: libc::c_int,
    mut attr_name: *const libc::c_char,
) {
    let mut a: *mut alias = 0 as *mut alias;
    let mut m: *mut member = 0 as *mut member;
    let mut c: *mut sudo_command = 0 as *mut sudo_command;
    let mut attr_val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    match type_0 {
        ALL => {
            print_attribute_ldif(
                fp,
                attr_name,
                if negated as libc::c_int != 0 {
                    b"!ALL\0" as *const u8 as *const libc::c_char
                } else {
                    b"ALL\0" as *const u8 as *const libc::c_char
                },
            );
        }
        MYSELF => {
            /* Only valid for sudoRunasUser */
            print_attribute_ldif(fp, attr_name, b"\0" as *const u8 as *const libc::c_char);
        }
        COMMAND => {
            c = name as *mut sudo_command;
            len = asprintf(
                &mut attr_val as *mut *mut libc::c_char,
                b"%s%s%s%s%s%s%s%s\0" as *const u8 as *const libc::c_char,
                if !((*c).digest).is_null() {
                    digest_type_to_name((*(*c).digest).digest_type as libc::c_int)
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if !((*c).digest).is_null() {
                    b":\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if !((*c).digest).is_null() {
                    (*(*c).digest).digest_str as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if !((*c).digest).is_null() {
                    b" \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if negated as libc::c_int != 0 {
                    b"!\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*c).cmnd,
                if !((*c).args).is_null() {
                    b" \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if !((*c).args).is_null() {
                    (*c).args as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if len == -(1 as libc::c_int) {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
                print_attribute_ldif(fp, attr_name, attr_val);
                free(attr_val as *mut libc::c_void);
            }
        }
        ALIAS | _ => {
            if type_0 == ALIAS {
                a = alias_get(parse_tree, name, alias_type);
                if !a.is_null() {
                    m = (*a).members.tqh_first;
                    while !m.is_null() {
                        print_member_ldif(
                            fp,
                            parse_tree,
                            (*m).name,
                            (*m).type0 as libc::c_int,
                            if negated as libc::c_int != 0 {
                                ((*m).negated == 0) as libc::c_int
                            } else {
                                (*m).negated as libc::c_int
                            } != 0,
                            alias_type,
                            attr_name,
                        );
                        m = (*m).entries.tqe_next;
                    }
                    alias_put(a);
                }
            }
            /* FALLTHROUGH */
            len = asprintf(
                &mut attr_val as *mut *mut libc::c_char,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                if negated as libc::c_int != 0 {
                    b"!\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                name,
            );
            if len == -(1 as libc::c_int) {
                sudo_fatalx!(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    get_function_name!(),
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                );
            }
            print_attribute_ldif(fp, attr_name, attr_val);
            free(attr_val as *mut libc::c_void);
        }
    }
    debug_return!();
}






