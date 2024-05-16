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
    unreachable_code,
    non_snake_case,
    unreachable_patterns,
    unused_variables,
    clashing_extern_declarations
)]


use crate::common::*;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn rbfind(tree: *mut rbtree, key: *mut libc::c_void) -> *mut rbnode;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn rcstr_addref(s: *const libc::c_char) -> *mut libc::c_char;
    fn rbinsert(_: *mut rbtree, _: *mut libc::c_void, _: *mut *mut rbnode) -> libc::c_int;
    fn sudo_snprintf(
        str0: *mut libc::c_char,
        n: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn rbapply_node(
        _: *mut rbtree,
        _: *mut rbnode,
        _: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
        _: rbtraversal,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn rcstr_delref(s: *const libc::c_char);
    fn free_members(members: *mut member_list);
    fn rbdestroy(_: *mut rbtree, _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>);
    fn rbdelete(_: *mut rbtree, _: *mut rbnode) -> *mut libc::c_void;
    fn rbcreate(
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    ) -> *mut rbtree;
}


pub const ENOENT: libc::c_int = 2;
pub const ELOOP: libc::c_int = 40;
pub type size_t = libc::c_ulong;
pub type time_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_parse_tree {
    pub userspecs: userspec_list,
    pub defaults: defaults_list,
    pub aliases: *mut rbtree,
    pub shost: *mut libc::c_char,
    pub lhost: *mut libc::c_char,
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
    pub entries: TAILQ_ENTRY_userspec,
    pub users: member_list,
    pub privileges: privilege_list,
    pub comments: comment_list,
    pub lineno: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_userspec {
    pub tqe_next: *mut userspec,
    pub tqe_prev: *mut *mut userspec,
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
    pub entries: TAILQ_ENTRY_sudoers_comment,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_sudoers_comment {
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
    pub entries: TAILQ_ENTRY_privilege,
    pub ldap_role: *mut libc::c_char,
    pub hostlist: member_list,
    pub cmndlist: cmndspec_list,
    pub defaults: defaults_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_privilege {
    pub tqe_next: *mut privilege,
    pub tqe_prev: *mut *mut privilege,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defaults_list {
    pub tqh_first: *mut defaults,
    pub tqh_last: *mut *mut defaults,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defaults {
    pub entries: TAILQ_ENTRY_defaults,
    pub var: *mut libc::c_char,
    pub val: *mut libc::c_char,
    pub binding: *mut member_list,
    pub file: *mut libc::c_char,
    pub type0: libc::c_short,
    pub op: libc::c_char,
    pub error: libc::c_char,
    pub lineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_defaults {
    pub tqe_next: *mut defaults,
    pub tqe_prev: *mut *mut defaults,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub compar:
        Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
    pub root: rbnode,
    pub nil: rbnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub data: *mut libc::c_void,
    pub color: rbcolor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum rbcolor {
    red,
    black,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum rbtraversal {
    preorder,
    inorder,
    postorder,
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
    pub entries: TAILQ_ENTRY_cmndspec,
    pub runasuserlist: *mut member_list,
    pub runasgrouplist: *mut member_list,
    pub cmnd: *mut member,
    pub tags: cmndtag,
    pub timeout: libc::c_int,
    pub notbefore: time_t,
    pub notafter: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TAILQ_ENTRY_cmndspec {
    pub tqe_next: *mut cmndspec,
    pub tqe_prev: *mut *mut cmndspec,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct cmndtag {
    #[bitfield(name = "nopasswd", ty = "libc::c_int", bits = "0..=2")]
    #[bitfield(name = "noexec", ty = "libc::c_int", bits = "3..=5")]
    #[bitfield(name = "setenv", ty = "libc::c_int", bits = "6..=8")]
    #[bitfield(name = "log_input", ty = "libc::c_int", bits = "9..=11")]
    #[bitfield(name = "log_output", ty = "libc::c_int", bits = "12..=14")]
    #[bitfield(name = "send_mail", ty = "libc::c_int", bits = "15..=17")]
    #[bitfield(name = "follow", ty = "libc::c_int", bits = "18..=20")]
    pub nopasswd_noexec_setenv_log_input_log_output_send_mail_follow: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[macro_export]
macro_rules! errno {
    () => {
        (*__errno_location())
    };
}
#[macro_export]
macro_rules! TAILQ_FIRST {
    ($head:expr) => {
        (($head).tqh_first)
    };
}
#[macro_export]
macro_rules! rbapply {
    ($t:expr, $f:expr, $c:expr, $o:expr) => {
        (rbapply_node(($t), (*($t)).root.left, ($f), ($c), ($o)))
    };
}
#[macro_export]
macro_rules! rbisempty {
    ($t:expr) => {
        ((*$t).root.left == &mut (*$t).nil as *mut rbnode
            && (*$t).root.right == &mut (*$t).nil as *mut rbnode)
    };
}

/*
 * Comparison function for the red-black tree.
 * Aliases are sorted by name with the type used as a tie-breaker.
 */
unsafe extern "C" fn alias_compare(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> libc::c_int {
    let mut a1: *const alias = v1 as *const alias;
    let mut a2: *const alias = v2 as *const alias;
    let mut res: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if a1.is_null() {
        res = -(1 as libc::c_int);
    } else if a2.is_null() {
        res = 1;
    } else {
        res = strcmp((*a1).name, (*a2).name);
        if res == 0 {
            res = (*a1).type0 as libc::c_int - (*a1).type0 as libc::c_int;
        }
    }
    debug_return_int!(res);
}
/*
 * Search the tree for an alias with the specified name and type.
 * Returns a pointer to the alias structure or NULL if not found.
 * Caller is responsible for calling alias_put() on the returned
 * alias to mark it as unused.
 */
#[no_mangle]
pub unsafe extern "C" fn alias_get(
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *const libc::c_char,
    mut type0: libc::c_int,
) -> *mut alias {
    let mut key: alias = alias {
        name: 0 as *mut libc::c_char,
        type0: 0,
        used: 0,
        lineno: 0,
        file: 0 as *mut libc::c_char,
        members: member_list {
            tqh_first: 0 as *mut member,
            tqh_last: 0 as *mut *mut member,
        },
    };
    let mut node: *mut rbnode = 0 as *mut rbnode;
    let mut a: *mut alias = 0 as *mut alias;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if (*parse_tree).aliases.is_null() {
        debug_return_ptr!(0 as *mut alias);
    }
    key.name = name as *mut libc::c_char;
    key.type0 = type0 as libc::c_ushort;
    node = rbfind(
        (*parse_tree).aliases,
        &mut key as *mut alias as *mut libc::c_void,
    );
    if !node.is_null() {
        /*
         * Check whether this alias is already in use.
         * If so, we've detected a loop.  If not, set the flag,
         * which the caller should clear with a call to alias_put().
         */
        a = (*node).data as *mut alias;
        if (*a).used != 0 {
            errno!() = ELOOP;
            debug_return_ptr!(0 as *mut alias);
        }
        (*a).used = true as libc::c_short;
    } else {
        errno!() = ELOOP;
    }
    debug_return_ptr!(a as *mut alias);
}
/*
 * Clear the "used" flag in an alias once the caller is done with it.
 */
#[no_mangle]
pub unsafe extern "C" fn alias_put(mut a: *mut alias) {
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    (*a).used = false as libc::c_short;
    debug_return!();
}
/*
 * Add an alias to the aliases redblack tree.
 * Note that "file" must be a reference-counted string.
 * Returns NULL on success and an error string on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn alias_add(
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *mut libc::c_char,
    mut type0: libc::c_int,
    mut file: *mut libc::c_char,
    mut lineno: libc::c_int,
    mut members: *mut member,
) -> *const libc::c_char {
    static mut errbuf: [libc::c_char; 512] = [0; 512];
    let mut a: *mut alias = 0 as *mut alias;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if ((*parse_tree).aliases).is_null() {
        (*parse_tree).aliases = alloc_aliases();
        if ((*parse_tree).aliases).is_null() {
            sudo_strlcpy(
                errbuf.as_mut_ptr(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
            debug_return_str!(errbuf.as_mut_ptr());
        }
    }
    a = calloc(
        1 as libc::c_ulong,
        ::core::mem::size_of::<alias>() as libc::c_ulong,
    ) as *mut alias;
    if a.is_null() {
        sudo_strlcpy(
            errbuf.as_mut_ptr(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        debug_return_str!(errbuf.as_mut_ptr());
    }
    (*a).name = name;
    (*a).type0 = type0 as libc::c_ushort;
    /* a->used = false; */
    (*a).file = rcstr_addref(file);
    (*a).lineno = lineno;
    //HLTQ_TO_TAILQ(&a->members, members, entries);
    (*a).members.tqh_first = members;
    (*a).members.tqh_last = (*members).entries.tqe_prev;
    (*members).entries.tqe_prev = &mut (*a).members.tqh_first;
    match rbinsert(
        (*parse_tree).aliases,
        a as *mut libc::c_void,
        0 as *mut *mut rbnode,
    ) {
        1 => {
            sudo_snprintf(
                errbuf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"Alias \"%s\" already defined\0" as *const u8 as *const libc::c_char,
                name,
            );
            alias_free(a as *mut libc::c_void);
            debug_return_str!(errbuf.as_mut_ptr());
        }
        -1 => {
            sudo_strlcpy(
                errbuf.as_mut_ptr(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
            alias_free(a as *mut libc::c_void);
            debug_return_str!(errbuf.as_mut_ptr());
        }
        _ => {}
    }
    debug_return_str!(0 as *const libc::c_char);
}
/*
 * Closure to adapt 2-arg rbapply() to 3-arg alias_apply().
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias_apply_closure {
    pub parse_tree: *mut sudoers_parse_tree,
    pub func: Option<
        unsafe extern "C" fn(*mut sudoers_parse_tree, *mut alias, *mut libc::c_void) -> libc::c_int,
    >,
    pub cookie: *mut libc::c_void,
}
/* Adapt rbapply() to alias_apply() calling convention. */
unsafe extern "C" fn alias_apply_func(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut alias = v1 as *mut alias;
    let mut closure: *mut alias_apply_closure = v2 as *mut alias_apply_closure;
    return ((*closure).func).expect("non-null function pointer")(
        (*closure).parse_tree,
        a,
        (*closure).cookie,
    );
}
/*
 * Apply a function to each alias entry and pass in a cookie.
 */
#[no_mangle]
pub unsafe extern "C" fn alias_apply(
    mut parse_tree: *mut sudoers_parse_tree,
    mut func: Option<
        unsafe extern "C" fn(*mut sudoers_parse_tree, *mut alias, *mut libc::c_void) -> libc::c_int,
    >,
    mut cookie: *mut libc::c_void,
) {
    let mut closure: alias_apply_closure = alias_apply_closure {
        parse_tree: 0 as *mut sudoers_parse_tree,
        func: None,
        cookie: 0 as *mut libc::c_void,
    };
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if !((*parse_tree).aliases).is_null() {
        closure.parse_tree = parse_tree;
        closure.func = func;
        closure.cookie = cookie;
        rbapply!(
            (*parse_tree).aliases,
            Some(
                alias_apply_func
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            ),
            &mut closure as *mut alias_apply_closure as *mut libc::c_void,
            rbtraversal::inorder
        );
    }
    debug_return!();
}
/*
 * Returns true if there are no aliases in the parse_tree, else false.
 */
#[no_mangle]
pub unsafe extern "C" fn no_aliases(mut parse_tree: *mut sudoers_parse_tree) -> bool {
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    debug_return_bool!(((*parse_tree).aliases).is_null() || rbisempty!((*parse_tree).aliases));
}
/*
 * Free memory used by an alias struct and its members.
 */
#[no_mangle]
pub unsafe extern "C" fn alias_free(mut v: *mut libc::c_void) {
    let mut a: *mut alias = v as *mut alias;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if !a.is_null() {
        free((*a).name as *mut libc::c_void);
        rcstr_delref((*a).file);
        free_members(&mut (*a).members);
        free(a as *mut libc::c_void);
    }
    debug_return!();
}
/*
 * Find the named alias, remove it from the tree and return it.
 */
#[no_mangle]
pub unsafe extern "C" fn alias_remove(
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *mut libc::c_char,
    mut type0: libc::c_int,
) -> *mut alias {
    let mut node: *mut rbnode = 0 as *mut rbnode;
    let mut key: alias = alias {
        name: 0 as *mut libc::c_char,
        type0: 0,
        used: 0,
        lineno: 0,
        file: 0 as *mut libc::c_char,
        members: member_list {
            tqh_first: 0 as *mut member,
            tqh_last: 0 as *mut *mut member,
        },
    };
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if !((*parse_tree).aliases).is_null() {
        key.name = name;
        key.type0 = type0 as libc::c_ushort;
        node = rbfind(
            (*parse_tree).aliases,
            &mut key as *mut alias as *mut libc::c_void,
        );
        if !node.is_null() {
            //debug_return_ptr(rbdelete(parse_tree->aliases, node));
            debug_return_ptr!(rbdelete((*parse_tree).aliases, node) as *mut alias);
        }
    }
    errno!() = ENOENT;
    debug_return_ptr!(0 as *mut alias);
}
#[no_mangle]
pub unsafe extern "C" fn alloc_aliases() -> *mut rbtree {
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    debug_return_ptr!(rbcreate(Some(
        alias_compare
            as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    )));
}
#[no_mangle]
pub unsafe extern "C" fn free_aliases(mut aliases: *mut rbtree) {
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if !aliases.is_null() {
        rbdestroy(
            aliases,
            Some(alias_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn alias_type_to_string(mut alias_type: libc::c_int) -> *const libc::c_char {
    return if alias_type == HOSTALIAS {
        b"Host_Alias\0" as *const u8 as *const libc::c_char
    } else if alias_type == CMNDALIAS {
        b"Cmnd_Alias\0" as *const u8 as *const libc::c_char
    } else if alias_type == USERALIAS {
        b"User_Alias\0" as *const u8 as *const libc::c_char
    } else if alias_type == RUNASALIAS {
        b"Runas_Alias\0" as *const u8 as *const libc::c_char
    } else {
        b"Invalid_Alias\0" as *const u8 as *const libc::c_char
    };
}
/*
 * Remove the alias of the specified type as well as any other aliases
 * referenced by that alias.  Stores removed aliases in a freelist.
 */
unsafe extern "C" fn alias_remove_recursive(
    mut parse_tree: *mut sudoers_parse_tree,
    mut name: *mut libc::c_char,
    mut type0: libc::c_int,
    mut freelist: *mut rbtree,
) -> bool {
    let mut m: *mut member = 0 as *mut member;
    let mut a: *mut alias = 0 as *mut alias;
    let mut ret: bool = true;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    a = alias_remove(parse_tree, name, type0);
    if !a.is_null() {
        //TAILQ_FOREACH(m, &a->members, entries)
        m = (*a).members.tqh_first;
        loop {
            if m.is_null() {
                break;
            }
            if (*m).type0 as libc::c_int == ALIAS {
                if !alias_remove_recursive(parse_tree, (*m).name, type0, freelist) {
                    ret = false;
                }
            }
            m = (*m).entries.tqe_next;
        }
        if rbinsert(freelist, a as *mut libc::c_void, 0 as *mut *mut rbnode) != 0 {
            ret = false;
        }
    }
    debug_return_bool!(ret);
}
unsafe extern "C" fn alias_find_used_members(
    mut parse_tree: *mut sudoers_parse_tree,
    mut members: *mut member_list,
    mut atype: libc::c_int,
    mut used_aliases: *mut rbtree,
) -> libc::c_int {
    let mut m: *mut member = 0 as *mut member;
    let mut errors: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    if !members.is_null() {
        //TAILQ_FOREACH(m, &a->members, entries)
        m = (*members).tqh_first;
        loop {
            if m.is_null() {
                break;
            }
            if (*m).type0 as libc::c_int != ALIAS {
                if !alias_remove_recursive(parse_tree, (*m).name, atype, used_aliases) {
                    errors += 1;
                }
            }
            m = (*m).entries.tqe_next;
        }
    }
    debug_return_int!(errors);
}
/*
 * Move all aliases referenced by userspecs to used_aliases.
 */
#[no_mangle]
pub unsafe extern "C" fn alias_find_used(
    mut parse_tree: *mut sudoers_parse_tree,
    mut used_aliases: *mut rbtree,
) -> bool {
    let mut priv0: *mut privilege = 0 as *mut privilege;
    let mut us: *mut userspec = 0 as *mut userspec;
    let mut cs: *mut cmndspec = 0 as *mut cmndspec;
    let mut d: *mut defaults = 0 as *mut defaults;
    let mut m: *mut member = 0 as *mut member;
    let mut errors: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_ALIAS!());
    /* Move referenced aliases to used_aliases. */
    //TAILQ_FOREACH(us, &parse_tree->userspecs, entries)
    us = (*parse_tree).userspecs.tqh_first;
    loop {
        if us.is_null() {
            break;
        }
        errors += alias_find_used_members(parse_tree, &mut (*us).users, USERALIAS, used_aliases);
        //TAILQ_FOREACH(priv, &us->privileges, entries)
        priv0 = (*us).privileges.tqh_first;
        loop {
            if priv0.is_null() {
                break;
            }
            errors += alias_find_used_members(
                parse_tree,
                &mut (*priv0).hostlist,
                HOSTALIAS,
                used_aliases,
            );
            //TAILQ_FOREACH(cs, &priv->cmndlist, entries)
            cs = (*priv0).cmndlist.tqh_first;
            loop {
                if cs.is_null() {
                    break;
                }
                errors += alias_find_used_members(
                    parse_tree,
                    (*cs).runasuserlist,
                    RUNASALIAS,
                    used_aliases,
                );
                errors += alias_find_used_members(
                    parse_tree,
                    (*cs).runasgrouplist,
                    RUNASALIAS,
                    used_aliases,
                );
                m = (*cs).cmnd;
                if (*m).type0 as libc::c_int == ALIAS {
                    if !alias_remove_recursive(parse_tree, (*m).name, CMNDALIAS, used_aliases) {
                        errors += 1;
                    }
                }
                cs = (*cs).entries.tqe_next;
            }
            priv0 = (*priv0).entries.tqe_next;
        }
        us = (*us).entries.tqe_next;
    }
    //TAILQ_FOREACH(d, &parse_tree->defaults, entries)
    d = (*parse_tree).defaults.tqh_first;
    loop {
        if d.is_null() {
            break;
        }
        match (*d).type0 {
            DEFAULTS_HOST => {
                errors +=
                    alias_find_used_members(parse_tree, (*d).binding, HOSTALIAS, used_aliases);
            }
            DEFAULTS_USER => {
                errors +=
                    alias_find_used_members(parse_tree, (*d).binding, USERALIAS, used_aliases);
            }
            DEFAULTS_RUNAS => {
                errors +=
                    alias_find_used_members(parse_tree, (*d).binding, RUNASALIAS, used_aliases);
            }
            DEFAULTS_CMND => {
                errors +=
                    alias_find_used_members(parse_tree, (*d).binding, CMNDALIAS, used_aliases);
            }
            _ => {}
        }
        d = (*d).entries.tqe_next;
    }
    //debug_return_int(errors ? false : true);
    if errors != 0 {
        debug_return_bool!(false);
    } else {
        debug_return_bool!(true);
    }
}


