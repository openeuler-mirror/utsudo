/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_attributes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_unsafe
)]
#![feature(extern_types, register_tool)]
//use crate::defaults::list_member;
//use crate::defaults::list_members;
//use crate::defaults::sudo_defs_table;
use crate::gc::sudoers_gc_add;
use crate::sudoers::sudo_user;
#[warn(unused_unsafe)]
use libc::free;
use crate::common::*;
pub const GC_PTR: sudoers_gc_types = 2;
pub const I_ENV_KEEP: libc::c_int = 64;
pub const I_ENV_DELETE: libc::c_int = 63;
pub const I_ENV_CHECK: libc::c_int = 62;
pub const I_ENV_RESET: libc::c_int = 61;
pub const I_ENV_FILE: libc::c_int = 67;
pub const I_SUDOERS_LOCALE: libc::c_int = 69;
pub const I_SECURE_PATH: libc::c_int = 52;
pub const I_SET_LOGNAME: libc::c_int = 30;
pub type sudoers_gc_types = libc::c_uint;
pub const PATH_MAX: libc::c_int = 4096;
pub const KEPT_MAIL: libc::c_int = 8388608;
pub const KEPT_LOGNAME: libc::c_int = 1048576;
pub const KEPT_USER: libc::c_int = 2097152;
pub const KEPT_USER_VARIABLES: libc::c_int = KEPT_LOGNAME | KEPT_USER;
pub const SUDO_HOOK_RET_STOP: libc::c_int = 1;
pub const SUDO_HOOK_RET_NEXT: libc::c_int = 0;
pub const SUDOERS_LOCALE_SUDOERS: libc::c_int = 1;
pub const _ISprint: libc::c_int = 16384;
extern "C" {
    static mut sudo_defs_table: [sudo_defs_types; 122];
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sudoers_getlocale() -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn sudo_parseln_v2(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        lineno: *mut libc::c_uint,
        fp: *mut FILE,
        flags: libc::c_int,
    ) -> ssize_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn log_warningx(flags: libc::c_int, fmt: *const libc::c_char, _: ...) -> bool;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn user_is_exempt() -> bool;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn matches_env_pattern(
        pattern: *const libc::c_char,
        var: *const libc::c_char,
        full_match: *mut bool,
    ) -> bool;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_strlcat(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_env_file {
    pub open: Option<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void>,
    pub close: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub next:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_int) -> *mut libc::c_char>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment {
    pub envp: *mut *mut libc::c_char,
    pub old_envp: *mut *mut libc::c_char,
    pub env_size: size_t,
    pub env_len: size_t,
}


static mut env: environment = environment {
    envp: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    old_envp: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    env_size: 0,
    env_len: 0,
};
#[no_mangle]
pub unsafe extern "C" fn env_init(mut envp: *const *mut libc::c_char) -> bool {
    let mut ep: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    let mut len: size_t = 0 as size_t;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    if envp.is_null() {
        free(env.old_envp as *mut libc::c_void);
        env.old_envp = env.envp;
        env.envp = 0 as *mut *mut libc::c_char;
        env.env_size = 0 as size_t;
        env.env_len = 0 as size_t;
    } else {
        ep = envp;
        loop {
            if (*ep).is_null() {
                break;
            }
            ep = ep.offset(1);
        }
        len = ep.offset_from(envp) as size_t;
        env.env_len = len;
        env.env_size = len.wrapping_add(1 as size_t).wrapping_add(128 as size_t);
        env.envp = reallocarray(
            0 as *mut libc::c_void,
            env.env_size,
            ::std::mem::size_of::<*mut libc::c_char>() as size_t,
        ) as *mut *mut libc::c_char;
        if env.envp.is_null() {
            env.env_size = 0;
            env.env_len = 0;
            //sudo_warnx(U_("%s: %s"), __func__, U_("unable to allocate memory"));
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            debug_return_bool!(false);
        }
        memcpy(
            env.envp as *mut libc::c_void,
            envp as *const libc::c_void,
            len.wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as size_t) as size_t,
        );
        *(env.envp).offset(len as isize) = 0 as *mut libc::c_char;
        free(env.old_envp as *mut libc::c_void);
        env.old_envp = 0 as *mut *mut libc::c_char;
    } //else
    debug_return_bool!(true);
}
#[no_mangle]
pub unsafe extern "C" fn env_get(_: libc::c_void) -> *mut *mut libc::c_char {
    return env.envp;
}
#[no_mangle]
pub unsafe extern "C" fn env_swap_old(_: libc::c_void) -> bool {
    let mut old_envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if env.old_envp.is_null() {
        return false;
    }
    old_envp = env.old_envp;
    env.old_envp = env.envp;
    env.envp = old_envp;
    return true;
}
pub const SIZE_MAX: size_t = 18446744073709551615;
pub const EOVERFLOW: libc::c_int = 75;
#[no_mangle]
pub unsafe extern "C" fn sudo_putenv_nodebug(
    mut str_0: *mut libc::c_char,
    mut dupcheck: bool,
    mut overwrite: bool,
) -> libc::c_int {
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut len: size_t = 0;
    let mut found: bool = false as bool;
    if env.env_size > 2 && env.env_len > (env.env_size).wrapping_sub(2 as size_t) {
        let mut nenvp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut nsize: size_t = 0 as size_t;
        if env.env_size > SIZE_MAX.wrapping_sub(128 as size_t) {
            sudo_warnx_nodebug_v1(b"sudo_putenv_nodebug\0" as *const u8 as *const libc::c_char);
            *__errno_location() = EOVERFLOW;
            return -1;
        }
        nsize = (env.env_size).wrapping_add(128 as size_t);
        if nsize > SIZE_MAX.wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as size_t) {
            sudo_warnx_nodebug_v1(b"sudo_putenv_nodebug\0" as *const u8 as *const libc::c_char);
            *__errno_location() = EOVERFLOW;
            return -1;
        }
        nenvp = reallocarray(
            env.envp as *mut libc::c_void,
            nsize as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as size_t,
        ) as *mut *mut libc::c_char;
        if nenvp.is_null() {
            return -1;
        }
        env.envp = nenvp;
        env.env_size = nsize;
    }
    if dupcheck {
        len = (strchr(str_0, '=' as i32).offset_from(str_0)) as size_t + 1 as size_t;
        ep = env.envp;
        while !(*ep).is_null() {
            if strncmp(str_0, *ep, len) == 0 {
                if overwrite {
                    *ep = str_0;
                }
                found = true;
                break;
            }
            ep = ep.offset(1 as isize);
        }
        if found && overwrite {
            loop {
                ep = ep.offset(1);
                if (*ep).is_null() {
                    break;
                }
                if strncmp(str_0, *ep, len) == 0 {
                    let mut cur: *mut *mut libc::c_char = ep as *mut *mut libc::c_char;
                    loop {
                        *cur = (*cur).offset(1 as isize);
                        if (*cur).is_null() {
                            break;
                        }
                        cur = cur.offset(1 as isize);
                    }
                    ep = ep.offset(-1 as isize);
                }
            }
            env.env_len = ep.offset_from(env.envp) as size_t;
        }
    }
    if !found {
        ep = (env.envp).offset(env.env_len as isize);
        env.env_len = (env.env_len).wrapping_add(1 as size_t);
        *ep = str_0;
        ep = ep.offset(1);
        *ep = 0 as *mut libc::c_char;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn sudo_putenv(
    mut str_1: *mut libc::c_char,
    mut dupcheck: bool,
    mut overwrite: bool,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"sudo_putenv: %s\0" as *const u8 as *const libc::c_char,
        str_1
    );
    ret = sudo_putenv_nodebug(str_1, dupcheck, overwrite);
    if ret == -1 {}
    debug_return_int!(ret);
}
#[no_mangle]
pub unsafe extern "C" fn sudo_setenv2(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut dupcheck: bool,
    mut overwrite: bool,
) -> libc::c_int {
    let mut estring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut esize: size_t = 0 as size_t;
    let mut ret: libc::c_int = -1 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    esize = strlen(var)
        .wrapping_add(1)
        .wrapping_add(strlen(val))
        .wrapping_add(1);
    estring = malloc(esize) as *mut libc::c_char;
    if estring.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_int!(-1);
    }
    if sudo_strlcpy(estring, var, esize) >= esize
        || sudo_strlcat(estring, b"=\0" as *const u8 as *const libc::c_char, esize) >= esize
        || sudo_strlcat(estring, val, esize) >= esize
    {
        sudo_warnx!(
            b"internal error,%s overflow\0" as *const u8 as *const libc::c_char,
            get_function_name!()
        );
        *__errno_location() = EOVERFLOW;
    } else {
        ret = sudo_putenv(estring, dupcheck, overwrite);
    }
    if ret == -1 {
        free(estring as *mut libc::c_void);
    } else {
        sudoers_gc_add(GC_PTR, estring as *mut libc::c_void);
    }
    debug_return_int!(ret);
}
#[no_mangle]
pub unsafe extern "C" fn sudo_setenv(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut overwrite: libc::c_int,
) -> libc::c_int {
    return sudo_setenv2(var, val, true, overwrite != 0);
}
pub const EINVAL: libc::c_int = 22;
#[no_mangle]
unsafe extern "C" fn sudo_setenv_nodebug(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut overwrite: libc::c_int,
) -> libc::c_int {
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut estring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut esize: size_t = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if val.is_null() || *var as libc::c_int == '\u{0}' as i32 {
        *__errno_location() = EINVAL;
        if ret == -1 {
            free(estring as *mut libc::c_void);
        } else {
            sudoers_gc_add(GC_PTR, estring as *mut libc::c_void);
        }
        return ret;
    }
    cp = var;
    while *cp != 0 && *cp as libc::c_int != '=' as i32 {
        cp = cp.offset(1);
    }
    esize = cp.offset_from(var).wrapping_add(2) as size_t;
    if !val.is_null() {
        esize = esize.wrapping_add(strlen(val) as size_t) as size_t;
    }
    ep = malloc(esize) as *mut libc::c_char;
    estring = ep;
    if estring.is_null() {
        if ret == -1 {
            free(estring as *mut libc::c_void);
        } else {
            sudoers_gc_add(GC_PTR, estring as *mut libc::c_void);
        }
        return ret;
    }
    cp = var;
    while *cp != 0 && *cp as libc::c_int != '=' as i32 {
        *ep = *cp;
        ep = ep.offset(1);
        cp = cp.offset(1);
    }
    *ep = '=' as i32 as libc::c_char;
    ep = ep.offset(1);
    if !val.is_null() {
        cp = val;
        while *cp != 0 {
            *ep = *cp;
            ep = ep.offset(1);
            cp = cp.offset(1);
        }
    }
    *ep = '\u{0}' as i32 as libc::c_char;
    ret = sudo_putenv_nodebug(estring, true, overwrite != 0);
    //done;
    if ret == -1 {
        free(estring as *mut libc::c_void);
    } else {
        sudoers_gc_add(GC_PTR, estring as *mut libc::c_void);
    }
    return ret;
}


#[no_mangle]
unsafe extern "C" fn sudo_unsetenv_nodebug(mut var: *const libc::c_char) -> libc::c_int {
    let mut ep: *mut *mut libc::c_char = env.envp;
    let mut len: size_t = 0;
    if ep.is_null()
        || var.is_null()
        || *var as libc::c_int == '\u{0}' as i32
        || !strchr(var, '=' as i32).is_null()
    {
        *__errno_location() = EINVAL;
        return -1;
    }
    len = strlen(var);
    while !(*ep).is_null() {
        if strncmp(var, *ep, len) == 0 && (*ep).offset(len as isize) as libc::c_int == '=' as i32 {
            let mut cur: *mut *mut libc::c_char = ep as *mut *mut libc::c_char;
            loop {
                *cur = *cur.offset(1 as libc::c_int as isize);
                if (*cur).is_null() {
                    break;
                }
                cur = cur.offset(1);
            }
            env.env_len = (env.env_len).wrapping_sub(1);
        } else {
            ep = ep.offset(1);
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn sudo_unsetenv(mut name: *const libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"sudo_unsetenv: %s\0" as *const u8 as *const libc::c_char,
        name
    );
    ret = sudo_unsetenv_nodebug(name);
    debug_return_int!(ret);
}
#[no_mangle]
pub unsafe extern "C" fn sudo_getenv_nodebug(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namelen: size_t = 0 as libc::c_int as size_t;
    if env.env_len != 0 {
        while *name.offset(namelen as isize) as libc::c_int != '\u{0}' as i32
            && *name.offset(namelen as isize) as libc::c_int != '=' as i32
        {
            namelen = namelen.wrapping_add(1);
        }
        ep = env.envp;
        while !(*ep).is_null() {
            if strncmp(*ep, name, namelen) == 0
                && (*ep).offset(namelen as isize) as libc::c_int == '=' as i32
            {
                val = (*ep).offset(namelen as isize).offset(1 as isize);
                break;
            }
            ep = ep.offset(1);
        }
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn sudo_getenv(name: *const libc::c_char) -> *mut libc::c_char {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"sudo_getenv: %s\0" as *const u8 as *const libc::c_char,
        name
    );
    val = sudo_getenv_nodebug(name);
    debug_return_str!(val);
}
#[no_mangle]
pub unsafe extern "C" fn matches_env_list(
    mut var: *const libc::c_char,
    mut list: *mut list_members,
    mut full_match: *mut bool,
) -> bool {
    let mut cur: *mut list_member = 0 as *mut list_member;
    let mut is_logname: bool = 0 as libc::c_int != 0;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    match *var as u8 as char {
        'L' => {
            if strncmp(var, b"LOGNAME=\0" as *const u8 as *const libc::c_char, 8) == 0 {
                is_logname = true;
            }
        }
        'U' => {
            if strncmp(var, b"USER=\0" as *const u8 as *const libc::c_char, 5) == 0 {
                is_logname = true;
            }
        }
        _ => {}
    }
    if is_logname {
        cur = (*list).slh_first;
        while !cur.is_null() {
            if matches_env_pattern(
                (*cur).value,
                b"LOGNAME\0" as *const u8 as *const libc::c_char,
                full_match,
            ) == true
                || matches_env_pattern(
                    (*cur).value,
                    b"USER\0" as *const u8 as *const libc::c_char,
                    full_match,
                ) == true
            {
                debug_return_bool!(true);
            }
            cur = (*cur).entries.sle_next;
        }
    } else {
        cur = (*list).slh_first;
        while !cur.is_null() {
            if matches_env_pattern((*cur).value, var, full_match) {
                debug_return_bool!(true);
            }
            cur = (*cur).entries.sle_next;
        }
    } //else
    debug_return_bool!(false);
}
#[no_mangle]
pub unsafe extern "C" fn matches_env_delete(mut var: *const libc::c_char) -> bool {
    let mut full_match: bool = false;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    debug_return_bool!(matches_env_list(
        var,
        &mut (*sudo_defs_table.as_mut_ptr().offset(I_ENV_DELETE as isize))
            .sd_un
            .list,
        &mut full_match
    ));
}
#[no_mangle]
pub unsafe extern "C" fn tz_is_sane(mut tzval: *const libc::c_char) -> bool {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut lastch: libc::c_char = 0;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    if tzval.offset(0 as isize) as libc::c_int == ':' as i32 {
        tzval = tzval.offset(1 as isize);
    }
    if tzval.offset(0 as isize) as libc::c_int == '/' as i32 {
        if (strncmp(
            tzval,
            b"/usr/share/zoneinfo\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 20]>() as size_t).wrapping_sub(1 as size_t),
        ) != 0)
            || (tzval.offset(
                (::std::mem::size_of::<[libc::c_char; 20]>() as size_t).wrapping_sub(1 as size_t)
                    as isize,
            ) as libc::c_int
                != '/' as i32)
        {
            debug_return_bool!(false);
        }
    }
    lastch = '/' as libc::c_char;
    cp = tzval;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            debug_return_bool!(false);
        }
        if lastch as libc::c_int == '/' as i32
            && cp.offset(0 as isize) as libc::c_int == '.' as i32
            && cp.offset(1 as isize) as libc::c_int == '.' as i32
            && (cp.offset(2 as isize) as libc::c_int == '/' as i32
                || cp.offset(2 as isize) as libc::c_int == '\u{0}' as i32)
        {
            debug_return_bool!(false);
        }
        lastch = *cp;
        cp = cp.offset(1 as isize);
    }
    if cp.offset_from(tzval) as size_t >= PATH_MAX as size_t {
        debug_return_bool!(false);
    }
    debug_return_bool!(true);
}
#[no_mangle]
pub unsafe extern "C" fn matches_env_check(
    mut var: *const libc::c_char,
    mut full_match: *mut bool,
) -> libc::c_int {
    let mut keepit: libc::c_int = -1 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    if matches_env_list(
        var,
        &mut (*sudo_defs_table.as_mut_ptr().offset(I_ENV_CHECK as isize))
            .sd_un
            .list,
        full_match,
    ) {
        if strncmp(var, b"TZ=\0" as *const u8 as *const libc::c_char, 3) == 0 {
            keepit = tz_is_sane(var.offset(3 as isize)) as libc::c_int;
        } else {
            let mut val: *const libc::c_char = strchr(var, '=' as i32) as *const libc::c_char;
            if !val.is_null() {
                val = val.offset(1);
                keepit = (strpbrk(val, b"/%\0" as *const u8 as *const libc::c_char)).is_null()
                    as libc::c_int;
            }
        }
    }
    debug_return_int!(keepit);
}
#[no_mangle]
pub unsafe extern "C" fn matches_env_keep(
    mut var: *const libc::c_char,
    mut full_match: *mut bool,
) -> bool {
    let mut keepit: bool = false as bool;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    if sudo_mode & MODE_SHELL as libc::c_int != 0
        && strncmp(
            var,
            b"SHELL=\0" as *const u8 as *const libc::c_char,
            6 as size_t,
        ) == 0 as libc::c_int
    {
        keepit = true;
    } else if matches_env_list(
        var,
        &mut (*sudo_defs_table.as_mut_ptr().offset(I_ENV_KEEP as isize))
            .sd_un
            .list,
        full_match,
    ) {
        keepit = true;
    }
    debug_return_bool!(keepit);
}

#[no_mangle]
unsafe extern "C" fn env_should_delete(mut var: *const libc::c_char) -> bool {
    let mut delete_it: libc::c_int = 0 as libc::c_int;
    let mut full_match: bool = false as bool;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    delete_it = matches_env_delete(var) as libc::c_int;
    if delete_it == 0 {
        delete_it = (matches_env_check(var, &mut full_match) == 0) as libc::c_int;
    }
    if delete_it != 0 {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"delete %s: %s\0" as *const u8 as *const libc::c_char,
            var,
            b"YES\0" as *const u8 as *const libc::c_char
        );
    } else {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"delete %s: %s\0" as *const u8 as *const libc::c_char,
            var,
            b"NO\0" as *const u8 as *const libc::c_char
        );
    }
    debug_return_bool!(delete_it as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn env_should_keep(mut var: *const libc::c_char) -> bool {
    let mut keepit: libc::c_int = 0;
    let mut full_match: bool = 0 as libc::c_int != 0;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    keepit = matches_env_check(var, &mut full_match);
    if keepit == -1 {
        keepit = matches_env_keep(var, &mut full_match) as libc::c_int;
    }
    if keepit != 0 && !full_match {
        cp = strchr(var, '=' as i32);
        if !cp.is_null() {
            if strncmp(cp, b"=() \0" as *const u8 as *const libc::c_char, 4) == 0 {
                keepit = false as libc::c_int;
            }
        }
    }
    if keepit == 1 {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"keep %s: %s\0" as *const u8 as *const libc::c_char,
            var,
            b"YES\0" as *const u8 as *const libc::c_char
        );
    } else {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"keep %s: %s\0" as *const u8 as *const libc::c_char,
            var,
            b"NO\0" as *const u8 as *const libc::c_char
        );
    }
    debug_return_bool!(keepit == 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn env_merge(mut envp: *const *mut libc::c_char) -> bool {
    let mut ep: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    let mut ret: bool = 1 as libc::c_int != 0;
    debug_decl!(SUDOERS_DEBUG_ENV!());
    ep = envp;
    while !(*ep).is_null() {
        let mut overwrite: bool = if (*sudo_defs_table.as_mut_ptr().offset(I_ENV_RESET as isize))
            .sd_un
            .flag
            != 0
        {
            !env_should_keep(*ep) as libc::c_int
        } else {
            env_should_delete(*ep) as libc::c_int
        } != 0;
        if sudo_putenv(*ep, true, overwrite) == -1 {
            ret = false;
            break;
        }
        ep = ep.offset(1);
    }
    debug_return_bool!(ret);
}
pub const DID_HOME: libc::c_int = 4;
pub const DID_LOGNAME: libc::c_int = 16;
pub const DID_MAIL: libc::c_int = 128;
pub const DID_PATH: libc::c_int = 2;
pub const DID_SHELL: libc::c_int = 8;
pub const DID_TERM: libc::c_int = 1;
pub const DID_USER: libc::c_int = 32;
pub const KEPT_HOME: libc::c_uint = 262144;
#[no_mangle]
unsafe extern "C" fn env_update_didvar(mut ep: *const libc::c_char, mut didvar: *mut libc::c_uint) {
    match *ep as u8 as char {
        'H' => {
            if strncmp(ep, b"HOME=" as *const u8 as *const libc::c_char, 5) == 0 {
                *didvar |= DID_HOME as libc::c_uint;
            }
        }
        'L' => {
            if strncmp(ep, b"LOGNAME=" as *const u8 as *const libc::c_char, 8) == 0 {
                *didvar |= DID_LOGNAME as libc::c_uint;
            }
        }
        'M' => {
            if strncmp(ep, b"MAIL=" as *const u8 as *const libc::c_char, 5) == 0 {
                *didvar |= DID_MAIL as libc::c_uint;
            }
        }
        'P' => {
            if strncmp(ep, b"PATH=" as *const u8 as *const libc::c_char, 5) == 0 {
                *didvar |= DID_PATH as libc::c_uint;
            }
        }
        'S' => {
            if strncmp(ep, b"SHELL=" as *const u8 as *const libc::c_char, 6) == 0 {
                *didvar |= DID_SHELL as libc::c_uint;
            }
        }
        'T' => {
            if strncmp(ep, b"TERM=" as *const u8 as *const libc::c_char, 5) == 0 {
                *didvar |= DID_TERM as libc::c_uint;
            }
        }
        'U' => {
            if strncmp(ep, b"USER=" as *const u8 as *const libc::c_char, 5) == 0 {
                *didvar |= DID_USER as libc::c_uint;
            }
        }
        _ => {}
    } //end of match
}
pub const MAX_UID_T_LEN: libc::c_int = 10;
pub const MODE_RUN: libc::c_int = 1;
pub const MODE_RESET_HOME: libc::c_int = 1048576;
pub const MODE_LOGIN_SHELL: libc::c_int = 262144;
pub const MODE_SHELL: libc::c_int = 131072;
#[no_mangle]
pub unsafe extern "C" fn rebuild_env() -> bool {
    let mut current_block: u64;
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ps1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idbuf: [libc::c_char; 11] = [0; 11];
    let mut didvar: libc::c_uint = 0;
    let mut reset_home: bool = 0 as libc::c_int != 0;
    let sudo_debug_subsys_tmp: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"rebuild_env\0")).as_ptr(),
        b"env.c\0" as *const u8 as *const libc::c_char,
        890 as libc::c_int,
        sudo_debug_subsys_tmp,
    );
    ps1 = 0 as *mut libc::c_char;
    didvar = 0 as libc::c_int as libc::c_uint;
    env.env_len = 0 as libc::c_int as size_t;
    env.env_size = 128 as libc::c_int as size_t;
    free(env.old_envp as *mut libc::c_void);
    env.old_envp = env.envp;
    env.envp = reallocarray(
        0 as *mut libc::c_void,
        env.env_size,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if (env.envp).is_null() {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"rebuild_env\0")).as_ptr(),
            b"env.c\0" as *const u8 as *const libc::c_char,
            904 as libc::c_int,
            2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys_tmp,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
        );
        env.env_size = 0 as libc::c_int as size_t;
    } else {
        let ref mut fresh5 = *(env.envp).offset(0 as libc::c_int as isize);
        *fresh5 = 0 as *mut libc::c_char;
        if sudo_mode & 0x1 as libc::c_int != 0 {
            if (*sudo_defs_table
                .as_mut_ptr()
                .offset(20 as libc::c_int as isize))
            .sd_un
            .flag
                != 0
                || sudo_mode & (0x100000 as libc::c_int | 0x40000 as libc::c_int) != 0
                || sudo_mode & 0x20000 as libc::c_int != 0
                    && (*sudo_defs_table
                        .as_mut_ptr()
                        .offset(19 as libc::c_int as isize))
                    .sd_un
                    .flag
                        != 0
            {
                reset_home = 1 as libc::c_int != 0;
            }
        }
        if (*sudo_defs_table
            .as_mut_ptr()
            .offset(61 as libc::c_int as isize))
        .sd_un
        .flag
            != 0
            || sudo_mode & 0x40000 as libc::c_int != 0
        {
            if sudo_mode & 0x40000 as libc::c_int == 0 {
                ep = env.envp;
                while !(*ep).is_null() {
                    env_update_didvar(*ep, &mut didvar);
                    ep = ep.offset(1);
                }
            }
            ep = env.old_envp;
            loop {
                if (*ep).is_null() {
                    current_block = 3437258052017859086;
                    break;
                }
                let mut keepit: bool = false;
                keepit = env_should_keep(*ep);
                if strncmp(
                    *ep,
                    b"SUDO_PS1=\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    ps1 = (*ep).offset(5 as libc::c_int as isize);
                }
                if keepit {
                    if sudo_putenv(*ep, 1 as libc::c_int != 0, 0 as libc::c_int != 0)
                        == -(1 as libc::c_int)
                    {
                        current_block = 655114396176293719;
                        break;
                    }
                    env_update_didvar(*ep, &mut didvar);
                }
                ep = ep.offset(1);
            }
            match current_block {
                655114396176293719 => {}
                _ => {
                    didvar |= didvar << 16 as libc::c_int;
                    if sudo_mode & 0x40000 as libc::c_int != 0 {
                        if sudo_setenv2(
                            b"SHELL\0" as *const u8 as *const libc::c_char,
                            (*sudo_user._runas_pw).pw_shell,
                            didvar & 0x8 as libc::c_int as libc::c_uint != 0,
                            1 as libc::c_int != 0,
                        ) == -(1 as libc::c_int)
                        {
                            current_block = 655114396176293719;
                        } else if sudo_setenv2(
                            b"LOGNAME\0" as *const u8 as *const libc::c_char,
                            (*sudo_user._runas_pw).pw_name,
                            didvar & 0x10 as libc::c_int as libc::c_uint != 0,
                            1 as libc::c_int != 0,
                        ) == -(1 as libc::c_int)
                        {
                            current_block = 655114396176293719;
                        } else if sudo_setenv2(
                            b"USER\0" as *const u8 as *const libc::c_char,
                            (*sudo_user._runas_pw).pw_name,
                            didvar & 0x20 as libc::c_int as libc::c_uint != 0,
                            1 as libc::c_int != 0,
                        ) == -(1 as libc::c_int)
                        {
                            current_block = 655114396176293719;
                        } else {
                            current_block = 10399321362245223758;
                        }
                    } else if (*sudo_defs_table
                        .as_mut_ptr()
                        .offset(30 as libc::c_int as isize))
                    .sd_un
                    .flag
                        == 0
                    {
                        if didvar & 0x10 as libc::c_int as libc::c_uint == 0 {
                            if sudo_setenv2(
                                b"LOGNAME\0" as *const u8 as *const libc::c_char,
                                sudo_user.name,
                                0 as libc::c_int != 0,
                                1 as libc::c_int != 0,
                            ) == -(1 as libc::c_int)
                            {
                                current_block = 655114396176293719;
                            } else {
                                current_block = 5330834795799507926;
                            }
                        } else {
                            current_block = 5330834795799507926;
                        }
                        match current_block {
                            655114396176293719 => {}
                            _ => {
                                if didvar & 0x20 as libc::c_int as libc::c_uint == 0 {
                                    if sudo_setenv2(
                                        b"USER\0" as *const u8 as *const libc::c_char,
                                        sudo_user.name,
                                        0 as libc::c_int != 0,
                                        1 as libc::c_int != 0,
                                    ) == -(1 as libc::c_int)
                                    {
                                        current_block = 655114396176293719;
                                    } else {
                                        current_block = 10399321362245223758;
                                    }
                                } else {
                                    current_block = 10399321362245223758;
                                }
                            }
                        }
                    } else {
                        current_block = 10399321362245223758;
                    }
                    match current_block {
                        655114396176293719 => {}
                        _ => {
                            if didvar & 0x40000 as libc::c_int as libc::c_uint == 0 {
                                reset_home = 1 as libc::c_int != 0;
                            }
                            if sudo_mode & 0x40000 as libc::c_int != 0
                                || didvar & 0x800000 as libc::c_int as libc::c_uint == 0
                            {
                                if (*::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(
                                    b"/var/mail\0",
                                ))[(::core::mem::size_of::<
                                    [libc::c_char; 10],
                                >(
                                )
                                    as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    as usize] as libc::c_int
                                    == '/' as i32
                                {
                                    if asprintf(
                                        &mut cp as *mut *mut libc::c_char,
                                        b"MAIL=%s%s\0" as *const u8 as *const libc::c_char,
                                        b"/var/mail\0" as *const u8 as *const libc::c_char,
                                        (*sudo_user._runas_pw).pw_name,
                                    ) == -(1 as libc::c_int)
                                    {
                                        current_block = 655114396176293719;
                                    } else {
                                        current_block = 2290177392965769716;
                                    }
                                } else if asprintf(
                                    &mut cp as *mut *mut libc::c_char,
                                    b"MAIL=%s/%s\0" as *const u8 as *const libc::c_char,
                                    b"/var/mail\0" as *const u8 as *const libc::c_char,
                                    (*sudo_user._runas_pw).pw_name,
                                ) == -(1 as libc::c_int)
                                {
                                    current_block = 655114396176293719;
                                } else {
                                    current_block = 2290177392965769716;
                                }
                                match current_block {
                                    655114396176293719 => {}
                                    _ => {
                                        if sudo_putenv(
                                            cp,
                                            didvar & 0x80 as libc::c_int as libc::c_uint != 0,
                                            1 as libc::c_int != 0,
                                        ) == -(1 as libc::c_int)
                                        {
                                            free(cp as *mut libc::c_void);
                                            current_block = 655114396176293719;
                                        } else {
                                            sudoers_gc_add(GC_PTR, cp as *mut libc::c_void);
                                            current_block = 14298507163138330979;
                                        }
                                    }
                                }
                            } else {
                                current_block = 14298507163138330979;
                            }
                        }
                    }
                }
            }
        } else {
            ep = env.old_envp;
            loop {
                if (*ep).is_null() {
                    current_block = 14298507163138330979;
                    break;
                }
                if !env_should_delete(*ep) {
                    if strncmp(
                        *ep,
                        b"SUDO_PS1=\0" as *const u8 as *const libc::c_char,
                        9 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        ps1 = (*ep).offset(5 as libc::c_int as isize);
                    } else if strncmp(
                        *ep,
                        b"SHELL=\0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        didvar |= 0x8 as libc::c_int as libc::c_uint;
                    } else if strncmp(
                        *ep,
                        b"PATH=\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        didvar |= 0x2 as libc::c_int as libc::c_uint;
                    } else if strncmp(
                        *ep,
                        b"TERM=\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        didvar |= 0x1 as libc::c_int as libc::c_uint;
                    }
                    if sudo_putenv(*ep, 1 as libc::c_int != 0, 0 as libc::c_int != 0)
                        == -(1 as libc::c_int)
                    {
                        current_block = 655114396176293719;
                        break;
                    }
                }
                ep = ep.offset(1);
            }
        }
        match current_block {
            655114396176293719 => {}
            _ => {
                if !((*sudo_defs_table
                    .as_mut_ptr()
                    .offset(52 as libc::c_int as isize))
                .sd_un
                .str_0)
                    .is_null()
                    && !user_is_exempt()
                {
                    if sudo_setenv2(
                        b"PATH\0" as *const u8 as *const libc::c_char,
                        (*sudo_defs_table
                            .as_mut_ptr()
                            .offset(52 as libc::c_int as isize))
                        .sd_un
                        .str_0,
                        1 as libc::c_int != 0,
                        1 as libc::c_int != 0,
                    ) == -(1 as libc::c_int)
                    {
                        current_block = 655114396176293719;
                    } else {
                        didvar |= 0x2 as libc::c_int as libc::c_uint;
                        current_block = 10435735846551762309;
                    }
                } else {
                    current_block = 10435735846551762309;
                }
                match current_block {
                    655114396176293719 => {}
                    _ => {
                        if (*sudo_defs_table
                            .as_mut_ptr()
                            .offset(30 as libc::c_int as isize))
                        .sd_un
                        .flag
                            != 0
                            && sudo_mode & 0x40000 as libc::c_int == 0
                        {
                            if didvar
                                & (0x100000 as libc::c_int | 0x200000 as libc::c_int)
                                    as libc::c_uint
                                == 0 as libc::c_int as libc::c_uint
                            {
                                if sudo_setenv2(
                                    b"LOGNAME\0" as *const u8 as *const libc::c_char,
                                    (*sudo_user._runas_pw).pw_name,
                                    1 as libc::c_int != 0,
                                    1 as libc::c_int != 0,
                                ) == -(1 as libc::c_int)
                                {
                                    current_block = 655114396176293719;
                                } else if sudo_setenv2(
                                    b"USER\0" as *const u8 as *const libc::c_char,
                                    (*sudo_user._runas_pw).pw_name,
                                    1 as libc::c_int != 0,
                                    1 as libc::c_int != 0,
                                ) == -(1 as libc::c_int)
                                {
                                    current_block = 655114396176293719;
                                } else {
                                    current_block = 6040267449472925966;
                                }
                            } else if didvar
                                & (0x100000 as libc::c_int | 0x200000 as libc::c_int)
                                    as libc::c_uint
                                != (0x100000 as libc::c_int | 0x200000 as libc::c_int)
                                    as libc::c_uint
                            {
                                if didvar & 0x100000 as libc::c_int as libc::c_uint != 0 {
                                    cp = sudo_getenv(
                                        b"LOGNAME\0" as *const u8 as *const libc::c_char,
                                    );
                                } else if didvar & 0x200000 as libc::c_int as libc::c_uint != 0 {
                                    cp = sudo_getenv(b"USER\0" as *const u8 as *const libc::c_char);
                                } else {
                                    cp = 0 as *mut libc::c_char;
                                }
                                if !cp.is_null() {
                                    if didvar & 0x100000 as libc::c_int as libc::c_uint == 0 {
                                        if sudo_setenv2(
                                            b"LOGNAME\0" as *const u8 as *const libc::c_char,
                                            cp,
                                            1 as libc::c_int != 0,
                                            1 as libc::c_int != 0,
                                        ) == -(1 as libc::c_int)
                                        {
                                            current_block = 655114396176293719;
                                        } else {
                                            current_block = 178030534879405462;
                                        }
                                    } else {
                                        current_block = 178030534879405462;
                                    }
                                    match current_block {
                                        655114396176293719 => {}
                                        _ => {
                                            if didvar & 0x200000 as libc::c_int as libc::c_uint == 0
                                            {
                                                if sudo_setenv2(
                                                    b"USER\0" as *const u8 as *const libc::c_char,
                                                    cp,
                                                    1 as libc::c_int != 0,
                                                    1 as libc::c_int != 0,
                                                ) == -(1 as libc::c_int)
                                                {
                                                    current_block = 655114396176293719;
                                                } else {
                                                    current_block = 6040267449472925966;
                                                }
                                            } else {
                                                current_block = 6040267449472925966;
                                            }
                                        }
                                    }
                                } else {
                                    current_block = 6040267449472925966;
                                }
                            } else {
                                current_block = 6040267449472925966;
                            }
                        } else {
                            current_block = 6040267449472925966;
                        }
                        match current_block {
                            655114396176293719 => {}
                            _ => {
                                if reset_home {
                                    if sudo_setenv2(
                                        b"HOME\0" as *const u8 as *const libc::c_char,
                                        (*sudo_user._runas_pw).pw_dir,
                                        1 as libc::c_int != 0,
                                        1 as libc::c_int != 0,
                                    ) == -(1 as libc::c_int)
                                    {
                                        current_block = 655114396176293719;
                                    } else {
                                        current_block = 4691324637564808323;
                                    }
                                } else {
                                    current_block = 4691324637564808323;
                                }
                                match current_block {
                                    655114396176293719 => {}
                                    _ => {
                                        if didvar & 0x8 as libc::c_int as libc::c_uint == 0 {
                                            if sudo_setenv2(
                                                b"SHELL\0" as *const u8 as *const libc::c_char,
                                                (*sudo_user._runas_pw).pw_shell,
                                                0 as libc::c_int != 0,
                                                0 as libc::c_int != 0,
                                            ) == -(1 as libc::c_int)
                                            {
                                                current_block = 655114396176293719;
                                            } else {
                                                current_block = 2904036176499606090;
                                            }
                                        } else {
                                            current_block = 2904036176499606090;
                                        }
                                        match current_block {
                                            655114396176293719 => {}
                                            _ => {
                                                if didvar & 0x1 as libc::c_int as libc::c_uint == 0
                                                {
                                                    if sudo_putenv(
                                                        b"TERM=unknown\0" as *const u8
                                                            as *const libc::c_char
                                                            as *mut libc::c_char,
                                                        0 as libc::c_int != 0,
                                                        0 as libc::c_int != 0,
                                                    ) == -(1 as libc::c_int)
                                                    {
                                                        current_block = 655114396176293719;
                                                    } else {
                                                        current_block = 2432552683059077439;
                                                    }
                                                } else {
                                                    current_block = 2432552683059077439;
                                                }
                                                match current_block {
                                                    655114396176293719 => {}
                                                    _ => {
                                                        if didvar
                                                            & 0x2 as libc::c_int as libc::c_uint
                                                            == 0
                                                        {
                                                            if sudo_setenv2(
                                                                b"PATH\0" as *const u8
                                                                    as *const libc::c_char,
                                                                b"/usr/bin:/bin:/usr/sbin:/sbin\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                0 as libc::c_int != 0,
                                                                1 as libc::c_int != 0,
                                                            ) == -(1 as libc::c_int)
                                                            {
                                                                current_block = 655114396176293719;
                                                            } else {
                                                                current_block =
                                                                    14913924298693586572;
                                                            }
                                                        } else {
                                                            current_block = 14913924298693586572;
                                                        }
                                                        match current_block {
                                                            655114396176293719 => {}
                                                            _ => {
                                                                if !ps1.is_null() {
                                                                    if sudo_putenv(
                                                                        ps1,
                                                                        1 as libc::c_int != 0,
                                                                        1 as libc::c_int != 0,
                                                                    ) == -(1 as libc::c_int)
                                                                    {
                                                                        current_block =
                                                                            655114396176293719;
                                                                    } else {
                                                                        current_block =
                                                                            14612007084265645573;
                                                                    }
                                                                } else {
                                                                    current_block =
                                                                        14612007084265645573;
                                                                }
                                                                match current_block {
                                                                    655114396176293719 => {}
                                                                    _ => {
                                                                        if !(sudo_user.cmnd_args).is_null() {
                                                                            if asprintf(
                                                                                &mut cp as *mut *mut libc::c_char,
                                                                                b"SUDO_COMMAND=%s %s\0" as *const u8 as *const libc::c_char,
                                                                                sudo_user.cmnd,
                                                                                sudo_user.cmnd_args,
                                                                            ) == -(1 as libc::c_int)
                                                                            {
                                                                                current_block = 655114396176293719;
                                                                            } else if sudo_putenv(
                                                                                cp,
                                                                                1 as libc::c_int != 0,
                                                                                1 as libc::c_int != 0,
                                                                            ) == -(1 as libc::c_int)
                                                                            {
                                                                                free(cp as *mut libc::c_void);
                                                                                current_block = 655114396176293719;
                                                                            } else {
                                                                                sudoers_gc_add(GC_PTR, cp as *mut libc::c_void);
                                                                                current_block = 6407515180622463684;
                                                                            }
                                                                        } else if sudo_setenv2(
                                                                            b"SUDO_COMMAND\0" as *const u8 as *const libc::c_char,
                                                                            sudo_user.cmnd,
                                                                            1 as libc::c_int != 0,
                                                                            1 as libc::c_int != 0,
                                                                        ) == -(1 as libc::c_int)
                                                                        {
                                                                            current_block = 655114396176293719;
                                                                        } else {
                                                                            current_block = 6407515180622463684;
                                                                        }
                                                                        match current_block {
                                                                            655114396176293719 => {}
                                                                            _ => {
                                                                                if !(sudo_setenv2(
                                                                                    b"SUDO_USER\0" as *const u8 as *const libc::c_char,
                                                                                    sudo_user.name,
                                                                                    1 as libc::c_int != 0,
                                                                                    1 as libc::c_int != 0,
                                                                                ) == -(1 as libc::c_int))
                                                                                {
                                                                                    snprintf(
                                                                                        idbuf.as_mut_ptr(),
                                                                                        ::core::mem::size_of::<[libc::c_char; 11]>()
                                                                                            as libc::c_ulong,
                                                                                        b"%u\0" as *const u8 as *const libc::c_char,
                                                                                        sudo_user.uid,
                                                                                    );
                                                                                    if !(sudo_setenv2(
                                                                                        b"SUDO_UID\0" as *const u8 as *const libc::c_char,
                                                                                        idbuf.as_mut_ptr(),
                                                                                        1 as libc::c_int != 0,
                                                                                        1 as libc::c_int != 0,
                                                                                    ) == -(1 as libc::c_int))
                                                                                    {
                                                                                        snprintf(
                                                                                            idbuf.as_mut_ptr(),
                                                                                            ::core::mem::size_of::<[libc::c_char; 11]>()
                                                                                                as libc::c_ulong,
                                                                                            b"%u\0" as *const u8 as *const libc::c_char,
                                                                                            sudo_user.gid,
                                                                                        );
                                                                                        if !(sudo_setenv2(
                                                                                            b"SUDO_GID\0" as *const u8 as *const libc::c_char,
                                                                                            idbuf.as_mut_ptr(),
                                                                                            1 as libc::c_int != 0,
                                                                                            1 as libc::c_int != 0,
                                                                                        ) == -(1 as libc::c_int))
                                                                                        {
                                                                                            let mut sudo_debug_ret: bool = 1 as libc::c_int != 0;
                                                                                            sudo_debug_exit_bool_v1(
                                                                                                (*::core::mem::transmute::<
                                                                                                    &[u8; 12],
                                                                                                    &[libc::c_char; 12],
                                                                                                >(b"rebuild_env\0"))
                                                                                                    .as_ptr(),
                                                                                                b"env.c\0" as *const u8 as *const libc::c_char,
                                                                                                1129 as libc::c_int,
                                                                                                sudo_debug_subsys_tmp,
                                                                                                sudo_debug_ret,
                                                                                            );
                                                                                            return sudo_debug_ret;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    sudo_debug_printf2_v1(
        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"rebuild_env\0")).as_ptr(),
        b"env.c\0" as *const u8 as *const libc::c_char,
        1132 as libc::c_int,
        3 as libc::c_int
            | (1 as libc::c_int) << 5 as libc::c_int
            | (1 as libc::c_int) << 4 as libc::c_int
            | sudo_debug_subsys_tmp,
        b"sudoers\0" as *const u8 as *const libc::c_char,
        b"unable to rebuild the environment\0" as *const u8 as *const libc::c_char,
    );
    sudo_warn_nodebug_v1(
        b"sudoers\0" as *const u8 as *const libc::c_char,
        b"unable to rebuild the environment\0" as *const u8 as *const libc::c_char,
    );
    let mut sudo_debug_ret_0: bool = 0 as libc::c_int != 0;
    sudo_debug_exit_bool_v1(
        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"rebuild_env\0")).as_ptr(),
        b"env.c\0" as *const u8 as *const libc::c_char,
        1133 as libc::c_int,
        sudo_debug_subsys_tmp,
        sudo_debug_ret_0,
    );
    return sudo_debug_ret_0;
}


