/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::struct_macro::*;
use crate::process_hooks_unsetenv;
pub const SUDO_HOOK_RET_STOP: libc::c_int = 1;
pub const SUDO_HOOK_RET_ERROR: libc::c_int = -1;
pub type sudo_fn_putenv_t = Option<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>;
pub type sudo_fn_setenv_t = Option<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char, libc::c_int) -> libc::c_int,
>;
pub type sudo_fn_unsetenv_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;

extern "C" {
    static mut environ: *mut *mut libc::c_char;
    fn strncmp(__s1: *const libc::c_char, __s2: *const libc::c_char, __n: size_t) -> libc::c_int;
    fn strchr(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn memcpy(
        __restrict: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(__s: *const libc::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sudo_dso_findsym_v1(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn process_hooks_setenv(
        name: *const libc::c_char,
        value: *const libc::c_char,
        overwrite: libc::c_int,
    ) -> libc::c_int;
    fn process_hooks_putenv(string: *mut libc::c_char) -> libc::c_int;
    fn process_hooks_getenv(
        name: *const libc::c_char,
        value: *mut *mut libc::c_char,
    ) -> libc::c_int;
}

static mut priv_environ: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;

#[no_mangle]
unsafe extern "C" fn rpl_putenv(mut string: *mut libc::c_char) -> libc::c_int {
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut len: size_t = 0 as size_t;
    let mut found: bool = false;
    len = (strchr(string, '=' as i32).offset_from(string) + 1) as size_t;
    ep = environ;
    while !(*ep).is_null() {
        if strncmp(string, *ep, len) == 0 {
            *ep = string;
            found = true;
        }
        ep = ep.offset(1);
    }
    if found {
        while !(*ep).is_null() {
            if strncmp(string, *ep, len) == 0 {
                let mut cur: *mut *mut libc::c_char = ep;
                loop {
                    *cur = *cur.offset(1);
                    if !(*cur).is_null() {
                        break;
                    }
                    cur = cur.offset(1);
                }
            } else {
                ep = ep.offset(1);
            }
        }
    }
    if !found {
        let mut env_len: size_t = ep.offset_from(environ) as size_t;
        let mut envp: *mut *mut libc::c_char = reallocarray(
            priv_environ as *mut libc::c_void,
            env_len + 2 as libc::c_ulong,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        if envp.is_null() {
            return -1;
        }
        if environ != priv_environ {
            memcpy(
                envp as *mut libc::c_void,
                environ as *mut libc::c_void,
                env_len * (::std::mem::size_of::<*mut libc::c_char>() as size_t),
            );
        };
        let mut tmp = env_len + 1;
        *envp.offset(tmp as isize) = string;
        *envp.offset(env_len as isize) = 0 as *mut libc::c_char;
        environ = envp;
        priv_environ = environ;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn getenv(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    match process_hooks_getenv(name, &mut val) {
        SUDO_HOOK_RET_STOP => return val,
        SUDO_HOOK_RET_ERROR => return 0 as *mut libc::c_char,
        _ => return getenv_unhooked(name),
    };
}

#[no_mangle]
unsafe extern "C" fn rpl_setenv(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut overwrite: libc::c_int,
) -> libc::c_int {
    let mut envstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut esize: size_t = 0 as size_t;
    if var.is_null() || *var == '\u{0}' as libc::c_char {
        *__errno_location() = EINVAL as libc::c_int;
        return -1;
    }
    src = var;
    while *src != '\u{0}' as libc::c_char && *src != '=' as libc::c_char {
        src = src.offset(1);
    }
    esize = (src.offset_from(var) as libc::c_long as size_t).wrapping_add(2);
    if !val.is_null() {
        esize = esize.wrapping_add(strlen(val));
    }
    envstr = malloc(esize) as *mut libc::c_char;
    if envstr.is_null() {
        return -1;
    }
    src = var;
    dst = envstr;
    while *src != '\u{0}' as libc::c_char && *src != '=' as libc::c_char {
        src = src.offset(1);
        dst = dst.offset(1);
        *dst = *src;
    }
    dst = dst.offset(1);
    *dst = '=' as libc::c_char;
    if !val.is_null() {
        src = val;
        while *src as libc::c_int != '\u{0}' as i32 {
            src = src.offset(1);
            dst = dst.offset(1);
            *dst = *src;
        }
    }
    *dst = '\u{0}' as libc::c_char;
    if overwrite == 0 && !getenv(var).is_null() {
        free(envstr as *mut libc::c_void);
        return 0;
    }
    if rpl_putenv(envstr) == -1 {
        free(envstr as *mut libc::c_void);
        return -1;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn getenv_unhooked(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namelen: size_t = 0 as size_t;
    while *name.offset(namelen as isize) != '\u{0}' as libc::c_char
        && *name.offset(namelen as isize) != '=' as libc::c_char
    {
        namelen += 1;
    }
    ep = environ;
    while !(*ep).is_null() {
        if strncmp(*ep, name, namelen) == 0
            && *(*ep).offset(namelen as isize) == '=' as libc::c_char
        {
            val = (*ep).offset(namelen as isize).offset(1 as isize);
            break;
        }
        ep = ep.offset(1);
    }
    return val;
}

#[no_mangle]
pub unsafe extern "C" fn setenv(
    mut var: *const libc::c_char,
    mut val: *const libc::c_char,
    mut overwrite: libc::c_int,
) -> libc::c_int {
    match process_hooks_setenv(var, val, overwrite) {
        SUDO_HOOK_RET_STOP => return 0,
        SUDO_HOOK_RET_ERROR => return -1,
        _ => return setenv_unhooked(var, val, overwrite),
    };
}

#[no_mangle]
unsafe extern "C" fn rpl_unsetenv(mut var: *const libc::c_char) -> libc::c_int {
    let mut ep: *mut *mut libc::c_char = environ;
    let mut len: size_t = 0 as size_t;
    if var.is_null() || *var == '\u{0}' as libc::c_char || !(strchr(var, '=' as i32)).is_null() {
        *__errno_location() = EINVAL;
        return -1;
    }
    len = strlen(var);
    while !(*ep).is_null() {
        if strncmp(var, *ep, len) == 0 as libc::c_int
            && *(*ep).offset(len as isize) as libc::c_int == '=' as i32
        {
            let mut cur: *mut *mut libc::c_char = ep;
            loop {
                *cur = *cur.offset(1);
                if (*cur).is_null() {
                    break;
                }
                cur = cur.offset(1);
            }
        } else {
            ep = ep.offset(1);
        }
    }
    return 0;
}

unsafe extern "C" fn unsetenv_unhooked(mut var: *const libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut fn_3: sudo_fn_unsetenv_t = None;
    fn_3 = ::std::mem::transmute::<*mut libc::c_void, sudo_fn_unsetenv_t>(sudo_dso_findsym_v1(
        -(1 as libc::c_int) as *mut libc::c_void,
        b"unsetenv\0" as *const u8 as *const libc::c_char,
    ));
    if fn_3.is_some() {
        ret = fn_3.expect("is null func pointer")(var as *mut libc::c_void) as libc::c_int;
    } else {
        ret = rpl_unsetenv(var);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn unsetenv(mut var: *const libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    match process_hooks_unsetenv(var) {
        SUDO_HOOK_RET_STOP => ret = 0,
        SUDO_HOOK_RET_ERROR => ret = -1,
        _ => ret = unsetenv_unhooked(var),
    };
    return ret;
}
