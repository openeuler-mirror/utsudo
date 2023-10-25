/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_variables)]

use crate::struct_macro::*;

extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_hook_entry {
    pub entries: MID_1,
    pub u: MID_2,
    pub closure: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MID_1 {
    pub sle_next: *mut sudo_hook_entry,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MID_2 {
    pub generic_fn: sudo_hook_fn_t,
    pub setenv_fn: sudo_hook_fn_setenv_t,
    pub unsetenv_fn: sudo_hook_fn_unsetenv_t,
    pub getenv_fn: sudo_hook_fn_getenv_t,
    pub putenv_fn: sudo_hook_fn_putenv_t,
}

pub type sudo_hook_fn_putenv_t =
    Option<unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int>;

pub type sudo_hook_fn_setenv_t = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type sudo_hook_fn_getenv_t = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        *mut *mut libc::c_char,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type sudo_hook_fn_unsetenv_t = Option<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int>;

#[no_mangle]
pub unsafe extern "C" fn register_hook(mut hook: *mut sudo_hook) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    //define debug_decl(register_hook,SUDO_DEBUG_HOOKS);
    debug_decl!(register_hook, SUDO_DEBUG_HOOKS);
    //end of define;

    if (*hook).hook_version >> 16 != 1 {
        *__errno_location() = 22;
        ret = -1;
    } else {
        match (*hook).hook_type {
            4 => {
                ret = register_hook_internal(
                    &mut sudo_hook_getenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            3 => {
                ret = register_hook_internal(
                    &mut sudo_hook_putenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            1 => {
                ret = register_hook_internal(
                    &mut sudo_hook_setenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            2 => {
                ret = register_hook_internal(
                    &mut sudo_hook_unsetenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            _ => {
                *__errno_location() = 95;
                ret = 1;
            }
        }
    }
    //define debug_return_int(ret);
    debug_return_int!(ret);
    //end of define;
}
   
unsafe extern "C" fn deregister_hook_internal(
    mut head: *mut sudo_hook_list,
    mut hook_fn: Option<unsafe extern "C" fn() -> libc::c_int>,
    mut closure: *mut libc::c_void,
) {
    let mut hook: *mut sudo_hook_entry = 0 as *mut sudo_hook_entry;
    let mut prev: *mut sudo_hook_entry = 0 as *mut sudo_hook_entry;
    //define debug_decl(deregister_hook_internal,SUDO_DEBUG_HOOKS)
    debug_decl!(deregister_hook_internal, SUDO_DEBUG_HOOKS);
    //end of define;

    hook = (*head).slh_first;
    while !hook.is_null() {
        if (*hook).u.generic_fn == hook_fn && (*hook).closure == closure {
            if prev.is_null() {
                (*head).slh_first = (*(*head).slh_first).entries.sle_next;
            } else {
                (*prev).entries.sle_next = (*(*prev).entries.sle_next).entries.sle_next;
            }
            free(hook as *mut libc::c_void);
            break;
        }
        prev = hook;
        hook = (*hook).entries.sle_next;
    }
    //define debug_return;
    debug_return!();
    //end of define;
}

#[no_mangle]
pub unsafe extern "C" fn deregister_hook(mut hook: *mut sudo_hook) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    //define debug_decl(deregister_hook,SUDO_DEBUG_HOOKS);
    debug_decl!(deregister_hook, SUDO_DEBUG_HOOKS);
    //end of define;

    if (*hook).hook_version >> 16 != 1 {
        ret = -1;
    } else {
        match (*hook).hook_type {
            4 => {
                deregister_hook_internal(
                    &mut sudo_hook_getenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            3 => {
                deregister_hook_internal(
                    &mut sudo_hook_putenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            1 => {
                deregister_hook_internal(
                    &mut sudo_hook_setenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            2 => {
                deregister_hook_internal(
                    &mut sudo_hook_unsetenv_list,
                    (*hook).hook_fn,
                    (*hook).closure,
                );
            }
            _ => {
                ret = 1;
            }
        }
    }
    //define debug_return_int(ret);
    debug_return_int!(ret);
    //end of define
}