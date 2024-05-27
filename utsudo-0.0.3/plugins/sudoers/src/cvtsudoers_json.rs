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