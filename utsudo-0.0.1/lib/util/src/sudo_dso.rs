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

//secure_path.rs
use crate::ISSET;

/* Values for sudo_dso_load() mode. */
// #define SUDO_DSO_LAZY         0x1
// #define SUDO_DSO_NOW  0x2
// #define SUDO_DSO_GLOBAL       0x4
// #define SUDO_DSO_LOCAL        0x8

#[macro_export]
macro_rules! SUDO_DSO_LAZY {
    () => {
        0x1
    };
}

#[macro_export]
macro_rules! SUDO_DSO_NOW {
    () => {
        0x2
    };
}

#[macro_export]
macro_rules! SUDO_DSO_GLOBAL {
    () => {
        0x3
    };
}

#[macro_export]
macro_rules! SUDO_DSO_LOCAL {
    () => {
        0x4
    };
}

/* The MODE argument to `dlopen' contains one of the following: */
// #define RTLD_LAZY    0x00001 /* Lazy function call binding.  */
// #define RTLD_NOW     0x00002 /* Immediate function call binding.  */
#[macro_export]
macro_rules! RTLD_LAZY {
    () => {
        0x00001
    };
}

#[macro_export]
macro_rules! RTLD_NOW {
    () => {
        0x00002
    };
}

// #define RTLD_GLOBAL  0x00100
#[macro_export]
macro_rules! RTLD_GLOBAL {
    () => {
        0x00100
    };
}

// #define RTLD_LOCAL   0
#[macro_export]
macro_rules! RTLD_LOCAL {
    () => {
        0
    };
}

/* Special handle arguments for sudo_dso_findsym(). */
// #define SUDO_DSO_NEXT	 ((void *)-1)	/* Search subsequent objects. */
// #define SUDO_DSO_DEFAULT ((void *)-2)	/* Use default search algorithm. */
// #define SUDO_DSO_SELF	 ((void *)-3)	/* Search the caller itself. */
pub const SUDO_DSO_NEXT: *mut libc::c_void = -(1 as libc::c_int) as *mut libc::c_void;
pub const SUDO_DSO_DEFAULT: *mut libc::c_void = -(2 as libc::c_int) as *mut libc::c_void;
pub const SUDO_DSO_SELF: *mut libc::c_void = -(3 as libc::c_int) as *mut libc::c_void;

// # define RTLD_NEXT	((void *) -1l)
pub const RTLD_NEXT: *mut libc::c_void = -(1 as libc::c_long) as *mut libc::c_void;

// # define RTLD_DEFAULT	((void *) 0)
pub const RTLD_DEFAULT: *mut libc::c_void = 0 as *mut libc::c_void;

// #define	ENOENT		 2
pub const ENOENT: libc::c_int = 2;

extern "C" {
    fn dlerror() -> *mut libc::c_char;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_preload_symbol {
    pub name: *const libc::c_char,
    pub addr: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_preload_table {
    pub path: *const libc::c_char,
    pub handle: *mut libc::c_void,
    pub symbols: *mut sudo_preload_symbol,
}

static mut preload_table: *mut sudo_preload_table = 0 as *mut sudo_preload_table;

#[no_mangle]
pub unsafe extern "C" fn sudo_dso_preload_table_v1(mut table: *mut sudo_preload_table) {
    preload_table = table;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_dso_load_v1(
    mut path: *const libc::c_char,
    mut mode: libc::c_int,
) -> *mut libc::c_void {
    let mut pt: *mut sudo_preload_table = 0 as *mut sudo_preload_table;
    let mut flags: libc::c_int = 0;

    /* Check prelinked symbols first. */
    if !preload_table.is_null() {
        pt = preload_table;
        while !((*pt).handle).is_null() {
            if !((*pt).path).is_null() && strcmp(path, (*pt).path) == 0 {
                return (*pt).handle;
            }
            pt = pt.offset(1);
        }
    }

    /* Map SUDO_DSO_* -> RTLD_* */
    if ISSET!(mode, SUDO_DSO_LAZY!()) != 0 {
        flags = flags | RTLD_LAZY!();
    }
    if ISSET!(mode, SUDO_DSO_NOW!()) != 0 {
        flags = flags | RTLD_NOW!();
    }
    if ISSET!(mode, SUDO_DSO_GLOBAL!()) != 0 {
        flags = flags | RTLD_GLOBAL!();
    }
    if ISSET!(mode, SUDO_DSO_LOCAL!()) != 0 {
        flags = flags | RTLD_LOCAL!();
    }

    return dlopen(path, flags);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_dso_unload_v1(mut handle: *mut libc::c_void) -> libc::c_int {
    let mut pt: *mut sudo_preload_table = 0 as *mut sudo_preload_table;

    /* Check prelinked symbols first. */
    if !preload_table.is_null() {
        pt = preload_table;
        while !((*pt).handle).is_null() {
            if (*pt).handle == handle {
                return 0;
            }
            pt = pt.offset(1 as isize);
        }
    }
    return dlclose(handle);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_dso_findsym_v1(
    mut handle: *mut libc::c_void,
    mut symbol: *const libc::c_char,
) -> *mut libc::c_void {
    let mut pt: *mut sudo_preload_table = 0 as *mut sudo_preload_table;
    /* Check prelinked symbols first. */
    if !preload_table.is_null() {
        pt = preload_table;
        while !((*pt).handle).is_null() {
            if (*pt).handle == handle {
                let mut sym: *mut sudo_preload_symbol = (*pt).symbols;
                while !(*sym).name.is_null() {
                    if strcmp((*sym).name, symbol) == 0 {
                        return (*sym).addr;
                    }
                    sym = sym.offset(1);
                }
                *__errno_location() = ENOENT;
                return 0 as *mut libc::c_void;
            }
            pt = pt.offset(1);
        }
    }

    /*
     * Not all implementations support the special handles.
     */
    if handle == SUDO_DSO_NEXT {
        handle = RTLD_NEXT;
    } else if handle == SUDO_DSO_DEFAULT {
        handle = RTLD_DEFAULT;
    } else if handle == SUDO_DSO_SELF {
        *__errno_location() = ENOENT;
        return 0 as *mut libc::c_void;
    }

    return dlsym(handle, symbol);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_dso_strerror_v1() -> *mut libc::c_char {
    return dlerror();
}
