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

use crate::S_IWUSR;

pub type size_t = usize;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type __id_t = libc::c_uint;
pub type id_t = __id_t;


/*
 * The debug priorities and subsystems are currently hard-coded.
 * In the future we might consider allowing plugins to register their
 * own subsystems and provide direct access to the debugging API.
 */

/* Note: this must match the order in sudo_debug.h */
static mut sudo_debug_priorities: [*const libc::c_char; 9] = [
    b"crit\0" as *const u8 as *const libc::c_char,
    b"err\0" as *const u8 as *const libc::c_char,
    b"warn\0" as *const u8 as *const libc::c_char,
    b"notice\0" as *const u8 as *const libc::c_char,
    b"diag\0" as *const u8 as *const libc::c_char,
    b"info\0" as *const u8 as *const libc::c_char,
    b"trace\0" as *const u8 as *const libc::c_char,
    b"debug\0" as *const u8 as *const libc::c_char,
    0 as *const u8 as *const libc::c_char,
];

static mut sudo_debug_default_subsystems: [*const libc::c_char; 15] = [
    b"args\0" as *const u8 as *const libc::c_char,
    b"conv\0" as *const u8 as *const libc::c_char,
    b"edit\0" as *const u8 as *const libc::c_char,
    b"event\0" as *const u8 as *const libc::c_char,
    b"exec\0" as *const u8 as *const libc::c_char,
    b"hooks\0" as *const u8 as *const libc::c_char,
    b"main\0" as *const u8 as *const libc::c_char,
    b"netif\0" as *const u8 as *const libc::c_char,
    b"pcomm\0" as *const u8 as *const libc::c_char,
    b"plugin\0" as *const u8 as *const libc::c_char,
    b"pty\0" as *const u8 as *const libc::c_char,
    b"selinux\0" as *const u8 as *const libc::c_char,
    b"util\0" as *const u8 as *const libc::c_char,
    b"utmp\0" as *const u8 as *const libc::c_char,
    0 as *const u8 as *const libc::c_char,
];


#[macro_export]
macro_rules! O_WRONLY {
    () => {
        01
    };
}

#[macro_export]
macro_rules! O_APPEND {
    () => {
        02000
    };
}

#[macro_export]
macro_rules! O_CREAT {
    () => {
        0100
    };
}

#[macro_export]
macro_rules! S_IRUSR {
    () => {
        0400
    };
}

#[macro_export]
macro_rules! IGNORE_RESULT {
    ($x:expr) => {
        __typeof__(x) y = (x);
       (void)y;
    };
}

#[macro_export]
macro_rules! F_SETFD {
    () => {
        2
    };
}

macro_rules! sudo_setbit {
    ($_a:expr, $_i:expr) => {{
        (*(($_a).offset((($_i) / NBBY) as isize)) |= (1 << (($_i) % NBBY)))
    }};
}


#[macro_export]
macro_rules! SUDO_DEBUG_LINENO {
    () => {
        (1 << 5)
    };
}

#[macro_export]
macro_rules! SUDO_DEBUG_ERRNO {
    () => {
        (1 << 4)
    };
}

#[macro_export]
macro_rules! SUDO_DEBUG_LINENO {
    () => {
        (1 << 5)
    };
}


/* Initializer for instance index to indicate that debugging is not setup. */
// #define SUDO_DEBUG_INSTANCE_INITIALIZER      -1
#[macro_export]
macro_rules! SUDO_DEBUG_INSTANCE_INITIALIZER {
    () => {
        -1
    };
}
