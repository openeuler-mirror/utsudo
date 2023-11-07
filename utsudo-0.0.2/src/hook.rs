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

use crate::sudo_debug_printf2_v1;
pub const SUDO_DEBUG_HOOKS: libc::c_int = 6 << 6;
use utsudo_util::debug_decl;
use utsudo_util::debug_decl_vars;
use utsudo_util::debug_return;
use utsudo_util::debug_return_int;
use utsudo_util::sudo_debug::sudo_debug_enter_v1;
use utsudo_util::sudo_debug::sudo_debug_exit_int_v1;
use utsudo_util::sudo_debug::sudo_debug_exit_v1;
use utsudo_util::sudo_debug_macro::sudo_debug_subsys;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERROR;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_LINENO;
use utsudo_util::sudo_debug_printf;

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
pub unsafe extern "C" fn process_hooks_putenv(mut string: *mut libc::c_char) -> libc::c_int {
    let mut hook: *mut sudo_hook_entry = 0 as *mut sudo_hook_entry;
    let mut rc: libc::c_int = 0 as libc::c_int;

    hook = sudo_hook_putenv_list.slh_first;
    while !hook.is_null() {
        rc = ((*hook).u.putenv_fn).expect("non the func point")(string, (*hook).closure);
        if rc == 1 || rc == -1 {
            break;
        }

        hook = (*hook).entries.sle_next;
    }
    return rc;
}

#[no_mangle]
pub unsafe extern "C" fn process_hooks_getenv(
    mut name: *const libc::c_char,
    mut value: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut hook: *mut sudo_hook_entry = 0 as *mut sudo_hook_entry;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0 as libc::c_int;

    hook = sudo_hook_getenv_list.slh_first;
    while !hook.is_null() {
        rc = ((*hook).u.getenv_fn).expect("non the func point")(name, &mut val, (*hook).closure);
        if rc == 1 || rc == -1 {
            break;
        }
        hook = (*hook).entries.sle_next;
    }
    if !val.is_null() {
        *value = val;
    }
    return rc;
}

#[no_mangle]
pub unsafe extern "C" fn process_hooks_unsetenv(mut name: *const libc::c_char) -> libc::c_int {
    let mut hook: *mut sudo_hook_entry = 0 as *mut sudo_hook_entry;
    let mut rc: libc::c_int = 0 as libc::c_int;

    hook = sudo_hook_unsetenv_list.slh_first;
    while !hook.is_null() {
        rc = ((*hook).u.unsetenv_fn).expect("non the func point")(name, (*hook).closure);
        if rc == 1 || rc == -1 {
            break;
        }

        hook = (*hook).entries.sle_next;
    }
    return rc;
}

unsafe extern "C" fn register_hook_internal(
    mut head: *mut sudo_hook_list,
    mut hook_fn: Option<unsafe extern "C" fn() -> libc::c_int>,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut hook: *mut sudo_hook_entry = 0 as *mut sudo_hook_entry;
    //define debug_decl(register_hook_internal,SUDO_DEBUG_HOOKS)
    debug_decl!(register_hook_internal, SUDO_DEBUG_HOOKS);
    //end of define;

    hook = calloc(
        1 as size_t,
        ::std::mem::size_of::<sudo_hook_entry>() as size_t,
    ) as *mut sudo_hook_entry;
    if hook.is_null() {
        //define sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO,"unable to allocate memory");
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        //end of define;

        //define debug_return_int(-1);
        debug_return_int!(-1);
        //end of define;
    }
    (*hook).u.generic_fn = hook_fn;
    (*hook).closure = closure;

    (*hook).entries.sle_next = (*head).slh_first;
    (*head).slh_first = hook;

    //define debug_return_int(0);
    debug_return_int!(0);
    //end of define;
}

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