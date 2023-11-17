/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(non_camel_case_types, non_snake_case, unused_mut)]

use crate::struct_macro::*;

//WRDE_NOCMD = (1 << 2),	/* Don't do command substitution.  */
pub const WRDE_NOCMD: libc::c_uint = 1 << 2;

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}

pub type sudo_fn_wordexp_t =
    Option<unsafe extern "C" fn(*const libc::c_char, *mut wordexp_t, libc::c_int) -> libc::c_int>;

#[no_mangle]
pub unsafe extern "C" fn execv(
    mut _a1: *const libc::c_char,
    mut _a2: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn execvp(
    mut _a1: *const libc::c_char,
    mut _a2: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn execve(
    mut _a1: *const libc::c_char,
    mut _a2: *const *mut libc::c_char,
    mut _a3: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn execvpe(
    mut _a1: *const libc::c_char,
    mut _a2: *const *mut libc::c_char,
    mut _a3: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn fexecve(
    mut _a1: libc::c_int,
    mut _a2: *const *mut libc::c_char,
    mut _a3: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn posix_spawn(
    mut _a1: *mut pid_t,
    mut _a2: *const libc::c_char,
    mut _a3: *const posix_spawn_file_actions_t,
    mut _a4: *const posix_spawnattr_t,
    mut _a5: *const *mut libc::c_char,
    mut _a6: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn posix_spawnp(
    mut _a1: *mut pid_t,
    mut _a2: *const libc::c_char,
    mut _a3: *const posix_spawn_file_actions_t,
    mut _a4: *const posix_spawnattr_t,
    mut _a5: *const *mut libc::c_char,
    mut _a6: *const *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn system(mut _a1: *const libc::c_char) -> libc::c_int {
    *__errno_location() = EACCES as libc::c_int;
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn popen(
    mut _c: *const libc::c_char,
    mut _t: *const libc::c_char,
) -> *mut FILE {
    *__errno_location() = EACCES as libc::c_int;
    return 0 as *mut FILE;
}

#[no_mangle]
pub unsafe extern "C" fn wordexp(
    mut words: *const libc::c_char,
    mut we: *mut wordexp_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut fn_0: *mut libc::c_void = dlsym(
        -(1 as libc::c_long) as *mut libc::c_void,
        b"wordexp\0" as *const u8 as *const libc::c_char,
    );
    if fn_0.is_null() {
        *__errno_location() = EACCES as libc::c_int;
        return -(1 as libc::c_int);
    }
    return (::core::mem::transmute::<*mut libc::c_void, sudo_fn_wordexp_t>(fn_0))
        .expect("non-null function pointer")(
        words, we, flags | WRDE_NOCMD as libc::c_int
    );
}
