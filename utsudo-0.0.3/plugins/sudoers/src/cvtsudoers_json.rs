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
    unused_mut
)]
use crate::common::*;
extern "C" {
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sudo_debug_enter_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    static mut sudo_defs_table: [sudo_defs_types; 0];
    fn alias_get(
        parse_tree: *mut sudoers_parse_tree,
        name: *const libc::c_char,
        type_0: libc::c_int,
    ) -> *mut alias;
    fn alias_apply(
        parse_tree: *mut sudoers_parse_tree,
        func: Option<
            unsafe extern "C" fn(
                *mut sudoers_parse_tree,
                *mut alias,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        cookie: *mut libc::c_void,
    );
    fn alias_put(a: *mut alias);
    fn digest_type_to_name(digest_type: libc::c_int) -> *const libc::c_char;
    fn sudo_strtoid_v2(str: *const libc::c_char, errstr: *mut *const libc::c_char) -> id_t;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_debug_exit_int_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: libc::c_int,
    );
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    fn sudo_debug_exit_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
}
pub const DEFAULTS_CMND: libc::c_int = 269;
pub const DEFAULTS_RUNAS: libc::c_int = 268;
pub const DEFAULTS_USER: libc::c_int = 267;
pub const DEFAULTS_HOST: libc::c_int = 266;

pub const T_MASK: libc::c_int = 0x0FF;
pub const T_FLAG: libc::c_int = 0x004;
pub const T_LIST: libc::c_int = 0x006;

pub const IMPLIED: libc::c_int = 2;

pub const SUPPRESS_DEFAULTS: libc::c_int = 0x01;
pub const SUPPRESS_ALIASES: libc::c_int = 0x02;
pub const SUPPRESS_PRIVS: libc::c_int = 0x04;

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

/*
 * JSON values may be of the following types.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub enum json_value_type {
    JSON_STRING,
    JSON_ID,
    JSON_NUMBER,
    JSON_OBJECT,
    JSON_ARRAY,
    JSON_BOOL,
    JSON_NULL,
}
/*
 * JSON value suitable for printing.
 * Note: this does not support object or array values.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value {
    pub type_0: json_value_type,
    pub u: json_value_u,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union json_value_u {
    pub string: *mut libc::c_char,
    pub number: libc::c_int,
    pub id: id_t,
    pub boolean: bool,
}

/*
 * Closure used to store state when iterating over all aliases.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_alias_closure {
    pub fp: *mut FILE,
    pub title: *const libc::c_char,
    pub count: libc::c_uint,
    pub alias_type: libc::c_int,
    pub indent: libc::c_int,
    pub need_comma: bool,
}

/*
 * Type values used to disambiguate the generic WORD and ALIAS types.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub enum word_type {
    TYPE_COMMAND,
    TYPE_HOSTNAME,
    TYPE_RUNASGROUP,
    TYPE_RUNASUSER,
    TYPE_USERNAME,
}



/*
 * Print "indent" number of blank characters.
 */
unsafe extern "C" fn print_indent(mut fp: *mut FILE, mut indent: libc::c_int) {
    loop {
        let fresh0 = indent;
        indent = indent - 1;
        if !(fresh0 != 0) {
            break;
        }
        putc(' ' as i32, fp);
    }
}

/*
 * Print a JSON string, escaping special characters.
 * Does not support unicode escapes.
 */
unsafe extern "C" fn print_string_json_unquoted(mut fp: *mut FILE, mut str: *const libc::c_char) {
    let mut ch: libc::c_char = 0;

    loop {
        ch = *str;
        str = str.offset(1);
        if !(ch as libc::c_int != '\0' as i32) {
            break;
        }
        match ch as libc::c_int {
            34 | 92 => {
                putc('\\' as i32, fp);
            }
            8 => {
                ch = 'b' as i32 as libc::c_char;
                putc('\\' as i32, fp);
            }
            12 => {
                ch = 'f' as i32 as libc::c_char;
                putc('\\' as i32, fp);
            }
            10 => {
                ch = 'n' as i32 as libc::c_char;
                putc('\\' as i32, fp);
            }
            13 => {
                ch = 'r' as i32 as libc::c_char;
                putc('\\' as i32, fp);
            }
            9 => {
                ch = 't' as i32 as libc::c_char;
                putc('\\' as i32, fp);
            }
            _ => {}
        }
        putc(ch as libc::c_int, fp);
    }
}

/*
 * Print a quoted JSON string, escaping special characters.
 * Does not support unicode escapes.
 */
unsafe extern "C" fn print_string_json(mut fp: *mut FILE, mut str: *const libc::c_char) {
    putc('"' as i32, fp);
    print_string_json_unquoted(fp, str);
    putc('"' as i32, fp);
}

/*
 * Print a JSON name: value pair with proper quoting and escaping.
 */
unsafe extern "C" fn print_pair_json(
    mut fp: *mut FILE,
    mut pre: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *const json_value,
    mut post: *const libc::c_char,
    mut indent: libc::c_int,
) {
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    print_indent(fp, indent);

    /* prefix */
    if !pre.is_null() {
        fputs(pre, fp);
    }

    /* name */
    print_string_json(fp, name);
    putc(':' as i32, fp);
    putc(' ' as i32, fp);

    /* value */
    match (*value).type_0 {
        json_value_type::JSON_STRING => {
            print_string_json(fp, (*value).u.string);
        }
        json_value_type::JSON_ID => {
            fprintf(
                fp,
                b"%u\0" as *const u8 as *const libc::c_char,
                (*value).u.id,
            );
        }
        json_value_type::JSON_NUMBER => {
            fprintf(
                fp,
                b"%d\0" as *const u8 as *const libc::c_char,
                (*value).u.number,
            );
        }
        json_value_type::JSON_NULL => {
            fputs(b"null\0" as *const u8 as *const libc::c_char, fp);
        }
        json_value_type::JSON_BOOL => {
            fputs(
                if (*value).u.boolean as libc::c_int != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
                fp,
            );
        }
        json_value_type::JSON_OBJECT => {
            sudo_fatalx!(
                b"internal error: can't print JSON_OBJECT\0" as *const u8 as *const libc::c_char,
            );
        }
        json_value_type::JSON_ARRAY => {
            sudo_fatalx!(
                b"internal error: can't print JSON_ARRAY\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }

    /* postfix */
    if !post.is_null() {
        fputs(post, fp);
    }

    debug_return!();
}

/*
 * Print a JSON string with optional prefix and postfix to fp.
 * Strings are not quoted but are escaped as per the JSON spec.
 */
unsafe extern "C" fn printstr_json(
    mut fp: *mut FILE,
    mut pre: *const libc::c_char,
    mut str: *const libc::c_char,
    mut post: *const libc::c_char,
    mut indent: libc::c_int,
) {
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    print_indent(fp, indent);
    if !pre.is_null() {
        fputs(pre, fp);
    }
    if !str.is_null() {
        print_string_json_unquoted(fp, str);
    }
    if !post.is_null() {
        fputs(post, fp);
    }

    debug_return!();
}


/*
 * Print sudo command member in JSON format, with specified indentation.
 * If last_one is false, a comma will be printed before the newline
 * that closes the object.
 */
unsafe extern "C" fn print_command_json(
    mut fp: *mut FILE,
    mut name: *const libc::c_char,
    mut type_0: libc::c_int,
    mut negated: bool,
    mut indent: libc::c_int,
    mut last_one: bool,
) {
    let mut c: *mut sudo_command = name as *mut sudo_command;
    let mut value: json_value = json_value {
        type_0: json_value_type::JSON_STRING,
        u: json_value_u {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut digest_name: *const libc::c_char = 0 as *const libc::c_char;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    printstr_json(
        fp,
        b"{\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        indent,
    );
    if negated as libc::c_int != 0 || !((*c).digest).is_null() {
        putc('\n' as i32, fp);
        indent += 4 as libc::c_int;
    } else {
        putc(' ' as i32, fp);
        indent = 0 as libc::c_int;
    }

    /* Print command with optional command line args. */
    if !((*c).args).is_null() {
        printstr_json(
            fp,
            b"\"\0" as *const u8 as *const libc::c_char,
            b"command\0" as *const u8 as *const libc::c_char,
            b"\": \0" as *const u8 as *const libc::c_char,
            indent,
        );
        printstr_json(
            fp,
            b"\"\0" as *const u8 as *const libc::c_char,
            (*c).cmnd,
            b" \0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        printstr_json(
            fp,
            0 as *const libc::c_char,
            (*c).args,
            b"\"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        value.type_0 = json_value_type::JSON_STRING;
        value.u.string = (*c).cmnd;
        print_pair_json(
            fp,
            0 as *const libc::c_char,
            b"command\0" as *const u8 as *const libc::c_char,
            &mut value,
            0 as *const libc::c_char,
            indent,
        );
    }

    /* Optional digest. */
    if !((*c).digest).is_null() {
        fputs(b",\n\0" as *const u8 as *const libc::c_char, fp);
        digest_name = digest_type_to_name((*(*c).digest).digest_type as libc::c_int);
        value.type_0 = json_value_type::JSON_STRING;
        value.u.string = (*(*c).digest).digest_str;
        print_pair_json(
            fp,
            0 as *const libc::c_char,
            digest_name,
            &mut value,
            0 as *const libc::c_char,
            indent,
        );
    }

    /* Command may be negated. */
    if negated {
        fputs(b",\n\0" as *const u8 as *const libc::c_char, fp);
        value.type_0 = json_value_type::JSON_BOOL;
        value.u.boolean = true;
        print_pair_json(
            fp,
            0 as *const libc::c_char,
            b"negated\0" as *const u8 as *const libc::c_char,
            &mut value,
            0 as *const libc::c_char,
            indent,
        );
    }

    if indent != 0 as libc::c_int {
        indent -= 4 as libc::c_int;
        putc('\n' as i32, fp);
        print_indent(fp, indent);
    } else {
        putc(' ' as i32, fp);
    }
    putc('}' as i32, fp);
    if !last_one {
        putc(',' as i32, fp);
    }
    putc('\n' as i32, fp);

    debug_return!();
}

/*
 * Map an alias type to enum word_type.
 */
unsafe extern "C" fn alias_to_word_type(mut alias_type: libc::c_int) -> word_type {
    match alias_type {
        CMNDALIAS => return word_type::TYPE_COMMAND,
        HOSTALIAS => return word_type::TYPE_HOSTNAME,
        RUNASALIAS => return word_type::TYPE_RUNASUSER,
        USERALIAS => return word_type::TYPE_USERNAME,
        _ => {
            sudo_fatalx_nodebug_v1(
                b"unexpected alias type %d\0" as *const u8 as *const libc::c_char,
                alias_type,
            );
        }
    };
}

/*
 * Map a Defaults type to enum word_type.
 */
unsafe extern "C" fn defaults_to_word_type(mut defaults_type: libc::c_int) -> word_type {
    match defaults_type {
        DEFAULTS_CMND => return word_type::TYPE_COMMAND,
        DEFAULTS_HOST => return word_type::TYPE_HOSTNAME,
        DEFAULTS_RUNAS => return word_type::TYPE_RUNASUSER,
        DEFAULTS_USER => return word_type::TYPE_USERNAME,
        _ => {
            sudo_fatalx_nodebug_v1(
                b"unexpected defaults type %d\0" as *const u8 as *const libc::c_char,
                defaults_type,
            );
        }
    };
}

/*
 * Print struct member in JSON format, with specified indentation.
 * If last_one is false, a comma will be printed before the newline
 * that closes the object.
 */
unsafe extern "C" fn print_member_json_int(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *mut libc::c_char,
    mut type_0: libc::c_int,
    mut negated: bool,
    mut word_type: word_type,
    mut last_one: bool,
    mut indent: libc::c_int,
    mut expand_aliases: bool,
) {
    let mut value: json_value = json_value {
        type_0: json_value_type::JSON_STRING,
        u: json_value_u {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut typestr: *const libc::c_char = 0 as *const libc::c_char;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut alias_type: libc::c_int = UNSPEC;
    let mut id: id_t = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    /* Most of the time we print a string. */
    value.type_0 = json_value_type::JSON_STRING;
    if !name.is_null() {
        value.u.string = name;
    } else {
        match type_0 {
            ALL => {
                value.u.string = b"ALL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            MYSELF => {
                value.u.string = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            _ => {
                sudo_fatalx!(
                    b"missing member name for type %d\0" as *const u8 as *const libc::c_char,
                    type_0
                );
            }
        }
    }

    match type_0 {
        USERGROUP => {
            value.u.string = (value.u.string).offset(1); /* skip leading '%' */
            if *value.u.string as libc::c_int == ':' as i32 {
                value.u.string = (value.u.string).offset(1);
                typestr = b"nonunixgroup\0" as *const u8 as *const libc::c_char;
                if *value.u.string as libc::c_int == '#' as i32 {
                    id = sudo_strtoid_v2((value.u.string).offset(1 as isize), &mut errstr);
                    if !errstr.is_null() {
                        sudo_warnx!(
                            b"internal error: non-Unix group-ID %s: \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            errstr,
                            (value.u.string).offset(1 as isize)
                        );
                    } else {
                        value.type_0 = json_value_type::JSON_ID;
                        value.u.id = id;
                        typestr = b"nonunixgid\0" as *const u8 as *const libc::c_char;
                    }
                }
            } else {
                typestr = b"usergroup\0" as *const u8 as *const libc::c_char;
                if *value.u.string as libc::c_int == '#' as i32 {
                    id = sudo_strtoid_v2(
                        (value.u.string).offset(1 as libc::c_int as isize),
                        &mut errstr,
                    );
                    if !errstr.is_null() {
                        sudo_warnx!(
                            b"internal error: group-ID %s: \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            errstr,
                            (value.u.string).offset(1 as isize)
                        );
                    } else {
                        value.type_0 = json_value_type::JSON_ID;
                        value.u.id = id;
                        typestr = b"usergid\0" as *const u8 as *const libc::c_char;
                    }
                }
            }
        }
        NETGROUP => {
            typestr = b"netgroup\0" as *const u8 as *const libc::c_char;
            value.u.string = (value.u.string).offset(1); /* skip leading '+' */
        }
        NTWKADDR => {
            typestr = b"networkaddr\0" as *const u8 as *const libc::c_char;
        }
        COMMAND => {
            print_command_json(fp, name, type_0, negated, indent, last_one);
            debug_return!();
        }
        ALL | MYSELF | WORD => match word_type {
            word_type::TYPE_COMMAND => {
                typestr = b"command\0" as *const u8 as *const libc::c_char;
            }
            word_type::TYPE_HOSTNAME => {
                typestr = b"hostname\0" as *const u8 as *const libc::c_char;
            }
            word_type::TYPE_RUNASGROUP => {
                typestr = b"usergroup\0" as *const u8 as *const libc::c_char;
            }
            word_type::TYPE_RUNASUSER | word_type::TYPE_USERNAME => {
                typestr = b"username\0" as *const u8 as *const libc::c_char;
                if *value.u.string as libc::c_int == '#' as i32 {
                    id = sudo_strtoid_v2(
                        (value.u.string).offset(1 as libc::c_int as isize),
                        &mut errstr,
                    );
                    if !errstr.is_null() {
                        sudo_warnx!(
                            b"internal error: user-ID %s: \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            errstr,
                            name
                        );
                    } else {
                        value.type_0 = json_value_type::JSON_ID;
                        value.u.id = id;
                        typestr = b"userid\0" as *const u8 as *const libc::c_char;
                    }
                }
            }
            _ => {
                sudo_fatalx!(
                    b"unexpected word type %d\0" as *const u8 as *const libc::c_char,
                    word_type
                );
            }
        },
        ALIAS => match word_type {
            word_type::TYPE_COMMAND => {
                if expand_aliases {
                    alias_type = CMNDALIAS;
                } else {
                    typestr = b"cmndalias\0" as *const u8 as *const libc::c_char;
                }
            }
            word_type::TYPE_HOSTNAME => {
                if expand_aliases {
                    alias_type = HOSTALIAS;
                } else {
                    typestr = b"hostalias\0" as *const u8 as *const libc::c_char;
                }
            }
            word_type::TYPE_RUNASGROUP | word_type::TYPE_RUNASUSER => {
                if expand_aliases {
                    alias_type = RUNASALIAS;
                } else {
                    typestr = b"runasalias\0" as *const u8 as *const libc::c_char;
                }
            }
            word_type::TYPE_USERNAME => {
                if expand_aliases {
                    alias_type = USERALIAS;
                } else {
                    typestr = b"useralias\0" as *const u8 as *const libc::c_char;
                }
            }
            _ => {
                sudo_fatalx!(
                    b"unexpected word type %d\0" as *const u8 as *const libc::c_char,
                    word_type
                );
            }
        },
        _ => {
            sudo_fatalx!(
                b"unexpected member type %d\0" as *const u8 as *const libc::c_char,
                type_0
            );
        }
    }

    if expand_aliases as libc::c_int != 0 && type_0 == ALIAS {
        let mut a: *mut alias = 0 as *mut alias;
        let mut m: *mut member = 0 as *mut member;

        /* Print each member of the alias. */
        a = alias_get(parse_tree, value.u.string, alias_type);
        if !a.is_null() {
            m = (*a).members.tqh_first;
            while !m.is_null() {
                print_member_json_int(
                    fp,
                    parse_tree,
                    (*m).name,
                    (*m).type0 as libc::c_int,
                    if negated as libc::c_int != 0 {
                        ((*m).negated == 0) as libc::c_int
                    } else {
                        (*m).negated as libc::c_int
                    } != 0,
                    alias_to_word_type(alias_type),
                    last_one as libc::c_int != 0 && ((*m).entries.tqe_next).is_null(),
                    indent,
                    true,
                );
                m = (*m).entries.tqe_next;
            }
            alias_put(a);
        }
    } else {
        if negated {
            print_indent(fp, indent);
            fputs(b"{\n\0" as *const u8 as *const libc::c_char, fp);
            indent += 4 as libc::c_int;
            print_pair_json(
                fp,
                0 as *const libc::c_char,
                typestr,
                &mut value,
                b",\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = true;
            print_pair_json(
                fp,
                0 as *const libc::c_char,
                b"negated\0" as *const u8 as *const libc::c_char,
                &mut value,
                b"\n\0" as *const u8 as *const libc::c_char,
                indent,
            );
            indent -= 4 as libc::c_int;
            print_indent(fp, indent);
            putc('}' as i32, fp);
        } else {
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                typestr,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
        }

        if !last_one {
            putc(',' as i32, fp);
        }
        putc('\n' as i32, fp);
    }

    debug_return!();
}

unsafe extern "C" fn print_member_json(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut m: *mut member,
    mut word_type: word_type,
    mut last_one: bool,
    mut indent: libc::c_int,
    mut expand_aliases: bool,
) {
    print_member_json_int(
        fp,
        parse_tree,
        (*m).name,
        (*m).type0 as libc::c_int,
        (*m).negated != 0,
        word_type,
        last_one,
        indent,
        expand_aliases,
    );
}

/*
 * Callback for alias_apply() to print an alias entry if it matches
 * the type specified in the closure.
 */
unsafe extern "C" fn print_alias_json(
    mut parse_tree: *mut sudoers_parse_tree,
    mut a: *mut alias,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut closure: *mut json_alias_closure = v as *mut json_alias_closure;
    let mut m: *mut member = 0 as *mut member;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    if (*a).type0 as libc::c_int != (*closure).alias_type {
        debug_return_int!(0);
    }

    /* Open the aliases object or close the last entry, then open new one. */
    if (*closure).count == 0 as libc::c_int as libc::c_uint {
        (*closure).count = ((*closure).count).wrapping_add(1);
        fprintf(
            (*closure).fp,
            b"%s\n%*s\"%s\": {\n\0" as *const u8 as *const libc::c_char,
            if (*closure).need_comma as libc::c_int != 0 {
                b",\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*closure).indent,
            b"\0" as *const u8 as *const libc::c_char,
            (*closure).title,
        );
        (*closure).indent += 4 as libc::c_int;
    } else {
        fprintf(
            (*closure).fp,
            b"%*s],\n\0" as *const u8 as *const libc::c_char,
            (*closure).indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    printstr_json(
        (*closure).fp,
        b"\"\0" as *const u8 as *const libc::c_char,
        (*a).name,
        b"\": [\n\0" as *const u8 as *const libc::c_char,
        (*closure).indent,
    );

    (*closure).indent += 4 as libc::c_int;
    m = (*a).members.tqh_first;
    while !m.is_null() {
        print_member_json(
            (*closure).fp,
            parse_tree,
            m,
            alias_to_word_type((*closure).alias_type),
            ((*m).entries.tqe_next).is_null(),
            (*closure).indent,
            false,
        );
        m = (*m).entries.tqe_next;
    }
    (*closure).indent -= 4 as libc::c_int;
    debug_return_int!(0);
}

/*
 * Print the binding for a Defaults entry of the specified type.
 */
unsafe extern "C" fn print_binding_json(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut binding: *mut member_list,
    mut type_0: libc::c_int,
    mut indent: libc::c_int,
    mut expand_aliases: bool,
) {
    let mut m: *mut member = 0 as *mut member;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    if ((*binding).tqh_first).is_null() {
        debug_return!();
    }

    fprintf(
        fp,
        b"%*s\"Binding\": [\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );
    indent += 4 as libc::c_int;

    /* Print each member object in binding. */
    m = (*binding).tqh_first;
    while !m.is_null() {
        print_member_json(
            fp,
            parse_tree,
            m,
            defaults_to_word_type(type_0),
            ((*m).entries.tqe_next).is_null(),
            indent,
            expand_aliases,
        );
        m = (*m).entries.tqe_next;
    }

    indent -= 4 as libc::c_int;
    fprintf(
        fp,
        b"%*s],\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );

    debug_return!();
}

/*
 * Print a Defaults list JSON format.
 */
unsafe extern "C" fn print_defaults_list_json(
    mut fp: *mut FILE,
    mut def: *mut defaults,
    mut indent: libc::c_int,
) {
    let mut savech: libc::c_char = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = (*def).val;
    let mut value: json_value = json_value {
        type_0: json_value_type::JSON_STRING,
        u: json_value_u {
            string: 0 as *mut libc::c_char,
        },
    };
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    fprintf(
        fp,
        b"%*s{\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );
    indent += 4 as libc::c_int;
    value.type_0 = json_value_type::JSON_STRING;
    match (*def).op as libc::c_int {
        43 => {
            value.u.string = b"list_add\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        45 => {
            value.u.string =
                b"list_remove\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        1 => {
            value.u.string =
                b"list_assign\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            sudo_warnx!(
                b"internal error: unexpected list op %d\0" as *const u8 as *const libc::c_char,
                (*def).op as libc::c_int
            );
            value.u.string =
                b"unsupported\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    print_pair_json(
        fp,
        0 as *const libc::c_char,
        b"operation\0" as *const u8 as *const libc::c_char,
        &mut value,
        b",\n\0" as *const u8 as *const libc::c_char,
        indent,
    );
    printstr_json(
        fp,
        b"\"\0" as *const u8 as *const libc::c_char,
        (*def).var,
        b"\": [\n\0" as *const u8 as *const libc::c_char,
        indent,
    );
    indent += 4 as libc::c_int;
    print_indent(fp, indent);
    /* Split value into multiple space-separated words. */
    loop {
        /* Remove leading blanks, must have a non-empty string. */
        start = end;
        while isblank!(*start as libc::c_uchar as libc::c_int as isize) != 0 {
            start = start.offset(1);
        }
        if *start as libc::c_int == '\0' as i32 {
            break;
        }

        /* Find the end and print it. */
        end = start;
        while *end as libc::c_int != 0
            && isblank!(*end as libc::c_uchar as libc::c_int as isize) == 0
        {
            end = end.offset(1);
        }
        savech = *end;
        *end = '\0' as i32 as libc::c_char;
        print_string_json(fp, start);
        if savech as libc::c_int != '\0' as i32 {
            putc(',' as i32, fp);
        }
        *end = savech;
        if !(*end as libc::c_int != '\0' as i32) {
            end = end.offset(1);
            break;
        }
        end = end.offset(1);
    }
    putc('\n' as i32, fp);
    indent -= 4 as libc::c_int;
    fprintf(
        fp,
        b"%*s]\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );
    indent -= 4 as libc::c_int;
    fprintf(
        fp,
        b"%*s}\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );

    debug_return!();
}

unsafe extern "C" fn get_defaults_type(mut def: *mut defaults) -> libc::c_int {
    let mut cur: *mut sudo_defs_types = 0 as *mut sudo_defs_types;

    /* Look up def in table to find its type. */
    cur = sudo_defs_table.as_mut_ptr();
    while !((*cur).name).is_null() {
        if strcmp((*def).var, (*cur).name) == 0 as libc::c_int {
            return (*cur).type_0;
        }
        cur = cur.offset(1);
    }
    return -(1 as libc::c_int);
}

/*
 * Export all Defaults in JSON format.
 */
unsafe extern "C" fn print_defaults_json(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut indent: libc::c_int,
    mut expand_aliases: bool,
    mut need_comma: bool,
) -> bool {
    let mut value: json_value = json_value {
        type_0: json_value_type::JSON_STRING,
        u: json_value_u {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut def: *mut defaults = 0 as *mut defaults;
    let mut next: *mut defaults = 0 as *mut defaults;
    let mut type_0: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    if ((*parse_tree).defaults.tqh_first).is_null() {
        debug_return_bool!(need_comma);
    }

    fprintf(
        fp,
        b"%s\n%*s\"Defaults\": [\n\0" as *const u8 as *const libc::c_char,
        if need_comma as libc::c_int != 0 {
            b",\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );
    indent += 4 as libc::c_int;

    def = (*parse_tree).defaults.tqh_first;
    while !def.is_null() && {
        next = (*def).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        type_0 = get_defaults_type(def);
        if type_0 == -(1 as libc::c_int) {
            sudo_warnx!(
                b"unknown defaults entry \"%s\"\0" as *const u8 as *const libc::c_char,
                (*def).var as libc::c_int
            );
            /* XXX - just pass it through as a string anyway? */
        } else {
            /* Found it, print object container and binding (if any). */
            fprintf(
                fp,
                b"%*s{\n\0" as *const u8 as *const libc::c_char,
                indent,
                b"\0" as *const u8 as *const libc::c_char,
            );
            indent += 4 as libc::c_int;
            print_binding_json(
                fp,
                parse_tree,
                (*def).binding,
                (*def).type_0 as libc::c_int,
                indent,
                expand_aliases,
            );

            /* Validation checks. */
            /* XXX - validate values in addition to names? */

            /* Print options, merging ones with the same binding. */
            fprintf(
                fp,
                b"%*s\"Options\": [\n\0" as *const u8 as *const libc::c_char,
                indent,
                b"\0" as *const u8 as *const libc::c_char,
            );
            indent += 4 as libc::c_int;
            loop {
                next = (*def).entries.tqe_next;
                /* XXX - need to update cur too */
                if type_0 & T_MASK == T_FLAG || ((*def).val).is_null() {
                    value.type_0 = json_value_type::JSON_BOOL;
                    value.u.boolean = (*def).op != 0;
                    print_pair_json(
                        fp,
                        b"{ \0" as *const u8 as *const libc::c_char,
                        (*def).var,
                        &mut value,
                        b" }\0" as *const u8 as *const libc::c_char,
                        indent,
                    );
                } else if type_0 & T_MASK == T_LIST {
                    print_defaults_list_json(fp, def, indent);
                } else {
                    value.type_0 = json_value_type::JSON_STRING;
                    value.u.string = (*def).val;
                    print_pair_json(
                        fp,
                        b"{ \0" as *const u8 as *const libc::c_char,
                        (*def).var,
                        &mut value,
                        b" }\0" as *const u8 as *const libc::c_char,
                        indent,
                    );
                }
                if next.is_null() || (*def).binding != (*next).binding {
                    break;
                }
                def = next;
                type_0 = get_defaults_type(def);
                if type_0 == -(1 as libc::c_int) {
                    sudo_warnx!(
                        b"unknown defaults entry \"%s\"\0" as *const u8 as *const libc::c_char,
                        (*def).var as libc::c_int
                    );
                    /* XXX - just pass it through as a string anyway? */
                    break;
                } else {
                    fputs(b",\n\0" as *const u8 as *const libc::c_char, fp);
                }
            }
            putc('\n' as i32, fp);
            indent -= 4 as libc::c_int;
            print_indent(fp, indent);
            fputs(b"]\n\0" as *const u8 as *const libc::c_char, fp);
            indent -= 4 as libc::c_int;
            print_indent(fp, indent);
            fprintf(
                fp,
                b"}%s\n\0" as *const u8 as *const libc::c_char,
                if !next.is_null() {
                    b",\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
        def = next;
    }

    /* Close Defaults array; comma (if any) & newline will be printer later. */
    indent -= 4 as libc::c_int;
    print_indent(fp, indent);
    fputs(b"]\0" as *const u8 as *const libc::c_char, fp);

    debug_return_bool!(true);
}


/*
 * Export all aliases of the specified type in JSON format.
 * Iterates through the entire aliases tree.
 */
unsafe extern "C" fn print_aliases_by_type_json(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut alias_type: libc::c_int,
    mut title: *const libc::c_char,
    mut indent: libc::c_int,
    mut need_comma: bool,
) -> bool {
    let mut closure: json_alias_closure = json_alias_closure {
        fp: 0 as *mut FILE,
        title: 0 as *const libc::c_char,
        count: 0,
        alias_type: 0,
        indent: 0,
        need_comma: false,
    };
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    closure.fp = fp;
    closure.indent = indent;
    closure.count = 0 as libc::c_int as libc::c_uint;
    closure.alias_type = alias_type;
    closure.title = title;
    closure.need_comma = need_comma;
    alias_apply(
        parse_tree,
        Some(
            print_alias_json
                as unsafe extern "C" fn(
                    *mut sudoers_parse_tree,
                    *mut alias,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut closure as *mut json_alias_closure as *mut libc::c_void,
    );
    if closure.count != 0 as libc::c_int as libc::c_uint {
        print_indent(fp, closure.indent);
        fputs(b"]\n\0" as *const u8 as *const libc::c_char, fp);
        closure.indent -= 4 as libc::c_int;
        print_indent(fp, closure.indent);
        putc('}' as i32, fp);
        need_comma = true;
    }

    debug_return_bool!(need_comma);
}

/*
 * Export all aliases in JSON format.
 */
unsafe extern "C" fn print_aliases_json(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut indent: libc::c_int,
    mut need_comma: bool,
) -> bool {
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    need_comma = print_aliases_by_type_json(
        fp,
        parse_tree,
        USERALIAS,
        b"User_Aliases\0" as *const u8 as *const libc::c_char,
        indent,
        need_comma,
    );
    need_comma = print_aliases_by_type_json(
        fp,
        parse_tree,
        RUNASALIAS,
        b"Runas_Aliases\0" as *const u8 as *const libc::c_char,
        indent,
        need_comma,
    );
    need_comma = print_aliases_by_type_json(
        fp,
        parse_tree,
        HOSTALIAS,
        b"Host_Aliases\0" as *const u8 as *const libc::c_char,
        indent,
        need_comma,
    );
    need_comma = print_aliases_by_type_json(
        fp,
        parse_tree,
        CMNDALIAS,
        b"Command_Aliases\0" as *const u8 as *const libc::c_char,
        indent,
        need_comma,
    );

    debug_return_bool!(need_comma);
}

/*
 * Print a Cmnd_Spec in JSON format at the specified indent level.
 * A pointer to the next Cmnd_Spec is passed in to make it possible to
 * merge adjacent entries that are identical in all but the command.
 */
unsafe extern "C" fn print_cmndspec_json(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut cs: *mut cmndspec,
    mut nextp: *mut *mut cmndspec,
    mut options: *mut defaults_list,
    mut expand_aliases: bool,
    mut indent: libc::c_int,
) {
    let mut next: *mut cmndspec = *nextp;
    let mut value: json_value = json_value {
        type_0: json_value_type::JSON_STRING,
        u: json_value_u {
            string: 0 as *mut libc::c_char,
        },
    };
    let mut def: *mut defaults = 0 as *mut defaults;
    let mut m: *mut member = 0 as *mut member;
    let mut tp: *mut tm = 0 as *mut tm;
    let mut last_one: bool = false;
    let mut timebuf: [libc::c_char; 16] = [0; 16];
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    /* Open Cmnd_Spec object. */
    fprintf(
        fp,
        b"%*s{\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );
    indent += 4 as libc::c_int;

    /* Print runasuserlist */
    if !((*cs).runasuserlist).is_null() {
        fprintf(
            fp,
            b"%*s\"runasusers\": [\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;
        m = (*(*cs).runasuserlist).tqh_first;
        while !m.is_null() {
            print_member_json(
                fp,
                parse_tree,
                m,
                word_type::TYPE_RUNASUSER,
                ((*m).entries.tqe_next).is_null(),
                indent,
                expand_aliases,
            );
            m = (*m).entries.tqe_next;
        }
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s],\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }

    /* Print runasgrouplist */
    if !((*cs).runasgrouplist).is_null() {
        fprintf(
            fp,
            b"%*s\"runasgroups\": [\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;
        m = (*(*cs).runasgrouplist).tqh_first;
        while !m.is_null() {
            print_member_json(
                fp,
                parse_tree,
                m,
                word_type::TYPE_RUNASGROUP,
                ((*m).entries.tqe_next).is_null(),
                indent,
                expand_aliases,
            );
            m = (*m).entries.tqe_next;
        }
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s],\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }

    /* Print options and tags */
    if (*cs).timeout > 0 as libc::c_int
        || (*cs).notbefore != UNSPEC as libc::c_long
        || (*cs).notafter != UNSPEC as libc::c_long
        || TAGS_SET!((*cs).tags)
        || !((*options).tqh_first).is_null()
    {
        let mut tag: cmndtag = (*cs).tags;
        let mut prefix: *const libc::c_char = b"\n\0" as *const u8 as *const libc::c_char;

        fprintf(
            fp,
            b"%*s\"Options\": [\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;
        if (*cs).timeout > 0 as libc::c_int {
            value.type_0 = json_value_type::JSON_NUMBER;
            value.u.number = (*cs).timeout;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"command_timeout\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        if (*cs).notbefore != UNSPEC as libc::c_long {
            tp = gmtime(&mut (*cs).notbefore);
            if tp.is_null() {
                sudo_warn!(b"unable to get GMT time\0" as *const u8 as *const libc::c_char,);
            } else {
                if strftime(
                    timebuf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    b"%Y%m%d%H%M%SZ\0" as *const u8 as *const libc::c_char,
                    tp,
                ) == 0 as libc::c_int as libc::c_ulong
                {
                    sudo_warnx!(
                        b"unable to format timestamp\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    value.type_0 = json_value_type::JSON_STRING;
                    value.u.string = timebuf.as_mut_ptr();
                    fputs(prefix, fp);
                    print_pair_json(
                        fp,
                        b"{ \0" as *const u8 as *const libc::c_char,
                        b"notbefore\0" as *const u8 as *const libc::c_char,
                        &mut value,
                        b" }\0" as *const u8 as *const libc::c_char,
                        indent,
                    );
                    prefix = b",\n\0" as *const u8 as *const libc::c_char;
                }
            }
        }
        if (*cs).notafter != UNSPEC as libc::c_long {
            tp = gmtime(&mut (*cs).notafter);
            if tp.is_null() {
                sudo_warn!(b"unable to get GMT time\0" as *const u8 as *const libc::c_char,);
            } else {
                if strftime(
                    timebuf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    b"%Y%m%d%H%M%SZ\0" as *const u8 as *const libc::c_char,
                    tp,
                ) == 0 as libc::c_int as libc::c_ulong
                {
                    sudo_warnx!(
                        b"unable to format timestamp\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    value.type_0 = json_value_type::JSON_STRING;
                    value.u.string = timebuf.as_mut_ptr();
                    fputs(prefix, fp);
                    print_pair_json(
                        fp,
                        b"{ \0" as *const u8 as *const libc::c_char,
                        b"notafter\0" as *const u8 as *const libc::c_char,
                        &mut value,
                        b" }\0" as *const u8 as *const libc::c_char,
                        indent,
                    );
                    prefix = b",\n\0" as *const u8 as *const libc::c_char;
                }
            }
        }
        if tag.nopasswd() != UNSPEC {
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = tag.nopasswd() == 0;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"authenticate\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        if tag.noexec() != UNSPEC {
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = tag.noexec() != 0;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"noexec\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        if tag.send_mail() != UNSPEC {
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = tag.send_mail() != 0;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"send_mail\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        if tag.setenv() != UNSPEC {
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = tag.setenv() != 0;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"setenv\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        if tag.follow() != UNSPEC {
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = tag.follow() != 0;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"sudoedit_follow\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        if tag.log_input() != UNSPEC {
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = tag.log_input() != 0;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"log_input\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        if tag.log_output() != UNSPEC {
            value.type_0 = json_value_type::JSON_BOOL;
            value.u.boolean = tag.log_output() != 0;
            fputs(prefix, fp);
            print_pair_json(
                fp,
                b"{ \0" as *const u8 as *const libc::c_char,
                b"log_output\0" as *const u8 as *const libc::c_char,
                &mut value,
                b" }\0" as *const u8 as *const libc::c_char,
                indent,
            );
            prefix = b",\n\0" as *const u8 as *const libc::c_char;
        }
        def = (*options).tqh_first;
        while !def.is_null() {
            let mut type_0: libc::c_int = get_defaults_type(def);
            if type_0 == -(1 as libc::c_int) {
                sudo_warnx!(
                    b"unknown defaults entry \"%s\"\0" as *const u8 as *const libc::c_char,
                    (*def).var
                );
                /* XXX - just pass it through as a string anyway? */
            } else {
                fputs(prefix, fp);
                if type_0 & T_MASK == T_FLAG || ((*def).val).is_null() {
                    value.type_0 = json_value_type::JSON_BOOL;
                    value.u.boolean = (*def).op != 0;
                    print_pair_json(
                        fp,
                        b"{ \0" as *const u8 as *const libc::c_char,
                        (*def).var,
                        &mut value,
                        b" }\0" as *const u8 as *const libc::c_char,
                        indent,
                    );
                } else if type_0 & T_MASK == T_LIST {
                    print_defaults_list_json(fp, def, indent);
                } else {
                    value.type_0 = json_value_type::JSON_STRING;
                    value.u.string = (*def).val;
                    print_pair_json(
                        fp,
                        b"{ \0" as *const u8 as *const libc::c_char,
                        (*def).var,
                        &mut value,
                        b" }\0" as *const u8 as *const libc::c_char,
                        indent,
                    );
                }
                prefix = b",\n\0" as *const u8 as *const libc::c_char;
            }
            def = (*def).entries.tqe_next;
        }
        putc('\n' as i32, fp);
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s],\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }

    /* Print SELinux role/type */
    if !((*cs).role).is_null() && !((*cs).type_0).is_null() {
        fprintf(
            fp,
            b"%*s\"SELinux_Spec\": [\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;
        value.type_0 = json_value_type::JSON_STRING;
        value.u.string = (*cs).role;
        print_pair_json(
            fp,
            0 as *const libc::c_char,
            b"role\0" as *const u8 as *const libc::c_char,
            &mut value,
            b",\n\0" as *const u8 as *const libc::c_char,
            indent,
        );
        value.u.string = (*cs).type_0;
        print_pair_json(
            fp,
            0 as *const libc::c_char,
            b"type\0" as *const u8 as *const libc::c_char,
            &mut value,
            b"\n\0" as *const u8 as *const libc::c_char,
            indent,
        );
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s],\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }

    /*
     * Merge adjacent commands with matching tags, runas, SELinux
     * role/type and Solaris priv settings.
     */
    fprintf(
        fp,
        b"%*s\"Commands\": [\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );
    indent += 4 as libc::c_int;
    loop {
        /* Does the next entry differ only in the command itself? */
        /* XXX - move into a function that returns bool */
        last_one = next.is_null()
            || RUNAS_CHANGED!(cs, next)
            || TAGS_CHANGED!((*cs).tags, (*next).tags)
            || (*cs).role != (*next).role
            || (*cs).type_0 != (*next).type_0;

        print_member_json(
            fp,
            parse_tree,
            (*cs).cmnd,
            word_type::TYPE_COMMAND,
            last_one,
            indent,
            expand_aliases,
        );
        if last_one {
            break;
        }
        cs = next;
        next = (*cs).entries.tqe_next;
    }
    indent -= 4 as libc::c_int;
    fprintf(
        fp,
        b"%*s]\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
    );

    /* Close Cmnd_Spec object. */
    indent -= 4 as libc::c_int;
    fprintf(
        fp,
        b"%*s}%s\n\0" as *const u8 as *const libc::c_char,
        indent,
        b"\0" as *const u8 as *const libc::c_char,
        if !((*cs).entries.tqe_next).is_null() {
            b",\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    *nextp = next;

    debug_return!();
}

/*
 * Print a User_Spec in JSON format at the specified indent level.
 */
unsafe extern "C" fn print_userspec_json(
    mut fp: *mut FILE,
    mut parse_tree: *mut sudoers_parse_tree,
    mut us: *mut userspec,
    mut indent: libc::c_int,
    mut expand_aliases: bool,
) {
    let mut priv_0: *mut privilege = 0 as *mut privilege;
    let mut m: *mut member = 0 as *mut member;
    let mut cs: *mut cmndspec = 0 as *mut cmndspec;
    let mut next: *mut cmndspec = 0 as *mut cmndspec;
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    /*
     * Each userspec struct may contain multiple privileges for
     * a user.  We export each privilege as a separate User_Spec
     * object for simplicity's sake.
     */
    priv_0 = (*us).privileges.tqh_first;
    while !priv_0.is_null() {
        /* Open User_Spec object. */
        fprintf(
            fp,
            b"%*s{\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;

        /* Print users list. */
        fprintf(
            fp,
            b"%*s\"User_List\": [\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;
        m = (*us).users.tqh_first;
        while !m.is_null() {
            print_member_json(
                fp,
                parse_tree,
                m,
                word_type::TYPE_USERNAME,
                ((*m).entries.tqe_next).is_null(),
                indent,
                expand_aliases,
            );
            m = (*m).entries.tqe_next;
        }
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s],\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );

        /* Print hosts list. */
        fprintf(
            fp,
            b"%*s\"Host_List\": [\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;
        m = (*priv_0).hostlist.tqh_first;
        while !m.is_null() {
            print_member_json(
                fp,
                parse_tree,
                m,
                word_type::TYPE_HOSTNAME,
                ((*m).entries.tqe_next).is_null(),
                indent,
                expand_aliases,
            );
            m = (*m).entries.tqe_next;
        }
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s],\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );

        /* Print commands. */
        fprintf(
            fp,
            b"%*s\"Cmnd_Specs\": [\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );
        indent += 4 as libc::c_int;
        cs = (*priv_0).cmndlist.tqh_first;
        while !cs.is_null() && {
            next = (*cs).entries.tqe_next;
            1 as libc::c_int != 0
        } {
            print_cmndspec_json(
                fp,
                parse_tree,
                cs,
                &mut next,
                &mut (*priv_0).defaults,
                expand_aliases,
                indent,
            );
            cs = next;
        }
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s]\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
        );

        /* Close User_Spec object. */
        indent -= 4 as libc::c_int;
        fprintf(
            fp,
            b"%*s}%s\n\0" as *const u8 as *const libc::c_char,
            indent,
            b"\0" as *const u8 as *const libc::c_char,
            if !((*priv_0).entries.tqe_next).is_null() || !((*us).entries.tqe_next).is_null() {
                b",\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        priv_0 = (*priv_0).entries.tqe_next;
    }

    debug_return!();
}

