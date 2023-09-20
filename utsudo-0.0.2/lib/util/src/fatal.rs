/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_unsafe,
    non_upper_case_globals,
    dead_code
)]
use crate::__LC_MESSAGES;
use libc::FILE;

#[macro_export]
macro_rules! __LC_MESSAGES {
    () => {
        5
    };
}

extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn exit(__status: libc::c_int);
    fn strerror(__errnum: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn malloc(__size: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_getprogname() -> *const libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}

pub type ssize_t = libc::c_long;
pub type sudo_fatal_callback_t = Option<unsafe extern "C" fn()>;
static mut sudo_warn_conversation: sudo_conv_t = None;
static mut sudo_warn_setlocale: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool> = None;
static mut sudo_warn_setlocale_prev: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool> =
    None;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_fatal_callback {
    pub entries: STRUCT_unnamed,
    pub func: Option<unsafe extern "C" fn() -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct STRUCT_unnamed {
    pub sle_next: *mut sudo_fatal_callback,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_fatal_callback_list {
    pub slh_first: *mut sudo_fatal_callback,
}

static mut callbacks: sudo_fatal_callback_list = {
    let mut init = sudo_fatal_callback_list {
        slh_first: 0 as *const sudo_fatal_callback as *mut sudo_fatal_callback,
    };
    init
};

pub const SUDO_CONV_PROMPT_ECHO_OFF: libc::c_int = 1;
pub const SUDO_CONV_PROMPT_ECHO_ON: libc::c_int = 2;
pub const SUDO_CONV_ERROR_MSG: libc::c_int = 3;
pub const SUDO_CONV_INFO_MSG: libc::c_int = 4;
pub const SUDO_CONV_PROMPT_MASK: libc::c_int = 5;
pub const SUDO_CONV_PROMPT_ECHO_OK: libc::c_int = 4096;
pub const SUDO_CONV_PREFER_TTY: libc::c_int = 8192;

#[derive(Copy, Clone)]
pub struct sudo_conv_message {
    pub msg_type: libc::c_int,
    pub timeout: libc::c_int,
    pub msg: *const libc::c_char,
}

pub struct sudo_conv_reply {
    pub reply: *mut libc::c_char,
}

pub type sudo_conv_callback_fn_t =
    Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int>;

pub struct sudo_conv_callback {
    pub version: libc::c_uint,
    pub closure: *mut libc::c_void,
    pub on_suspend: sudo_conv_callback_fn_t,
    pub on_resume: sudo_conv_callback_fn_t,
}

pub type sudo_conv_t = Option<
    unsafe extern "C" fn(
        libc::c_int,
        *const sudo_conv_message,
        *mut sudo_conv_reply,
        *mut sudo_conv_callback,
    ) -> libc::c_int,
>;

#[no_mangle]
pub fn do_cleanup() {
    let mut cb: *mut sudo_fatal_callback = 0 as *mut sudo_fatal_callback;

    loop {
        cb = unsafe { callbacks.slh_first };
        if cb.is_null() {
            break;
        }
        unsafe {
            callbacks.slh_first = (*callbacks.slh_first).entries.sle_next;
            ((*cb).func).expect("the pointer is not null")();
            free(cb as *mut libc::c_void)
        };
    }
}

#[no_mangle]
unsafe fn sudo_fatal_callback_register_v1(mut func: sudo_fatal_callback_t) -> libc::c_int {
    let mut cb: *mut sudo_fatal_callback = callbacks.slh_first as *mut sudo_fatal_callback;
    cb = callbacks.slh_first;
    while !cb.is_null() {
        if func == (*cb).func {
            return -1;
        }
        cb = (*cb).entries.sle_next;
    }

    cb = malloc(std::mem::size_of::<sudo_fatal_callback>() as libc::c_ulong)
        as *mut sudo_fatal_callback;
    if cb.is_null() {
        return -1;
    }
    (*cb).func = func;
    (*cb).entries.sle_next = cb;
    callbacks.slh_first = cb;
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_fatal_callback_deregister_v1(
    mut func: sudo_fatal_callback_t,
) -> libc::c_int {
    let mut cb: *mut sudo_fatal_callback = 0 as *mut sudo_fatal_callback;
    let mut prev: *mut *mut sudo_fatal_callback = 0 as *mut *mut sudo_fatal_callback;
    prev = &mut callbacks.slh_first;
    loop {
        cb = unsafe { *prev };
        if cb.is_null() {
            break;
        }
        if (*cb).func == func {
            if cb == callbacks.slh_first {
                callbacks.slh_first = (*callbacks.slh_first).entries.sle_next;
            } else {
                let ref mut fresh8 = (**prev).entries.sle_next;
                *fresh8 = (*(**prev).entries.sle_next).entries.sle_next;
            }
            free(cb as *mut libc::c_void);
            return 0;
        }
        prev = &mut (*cb).entries.sle_next;
    }
    return -1;
}

#[no_mangle]
extern "C" fn sudo_warn_set_conversation_v1(mut conv: sudo_conv_t) {
    unsafe { sudo_warn_conversation = conv };
}

//#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub unsafe extern "C" fn sudo_warn_set_locale_func_v1(
    mut func: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool>,
) {
    sudo_warn_setlocale_prev = sudo_warn_setlocale;
    sudo_warn_setlocale = func;
}

#[no_mangle]
fn sudo_warn_gettext_v1(
    domainname: *const libc::c_char,
    msgid: *const libc::c_char,
) -> *mut libc::c_char {
    let mut cookie: libc::c_int = 0;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;

    unsafe {
        if sudo_warn_setlocale.is_some() {
            sudo_warn_setlocale.expect("is not null func pointer")(
                0 as libc::c_int != 0,
                &mut cookie,
            );
        }
    }
    msg = unsafe { dcgettext(domainname, msgid, __LC_MESSAGES!() as libc::c_int) };

    unsafe {
        if sudo_warn_setlocale.is_some() {
            sudo_warn_setlocale.expect("is not null function pointer")(
                1 as libc::c_int != 0,
                &mut cookie,
            );
        }
    }
    return msg;
}
