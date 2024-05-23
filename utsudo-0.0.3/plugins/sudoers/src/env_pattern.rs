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
    unused_variables
)]
extern "C" {
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
}
//define const
use crate::common::debug_decl;
use crate::common::debug_decl_vars;
use crate::common::debug_return_bool;
use crate::common::sudo_debug_enter_v1;
use crate::common::sudo_debug_exit_bool_v1;
use crate::common::sudo_debug_subsys;
use crate::common::*;
pub type size_t = libc::c_ulong;
//func
#[no_mangle]
unsafe extern "C" fn matches_env_pattern(
    mut pattern: *const libc::c_char,
    mut var: *const libc::c_char,
    mut full_match: *mut bool,
) -> bool {
    let mut match_0: bool = false;
    let mut iswild: bool = false;
    let mut saw_sep: bool = false;
    let mut len: size_t = 0 as size_t;
    let mut sep_pos: size_t = 0 as size_t;
    let mut cp: *const libc::c_char;
    let SUDOERS_DEBUG_UTIL: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize) as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL);
    sep_pos = strcspn(var, b"=\0" as *const u8 as *const libc::c_char);
    cp = pattern;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *cp as libc::c_int == '*' as i32 {
            iswild = true;
            break;
        }
        cp = cp.offset(1);
    }
    len = cp.offset_from(pattern) as size_t;
    if iswild {
        if strncmp(pattern, var, len) == 0 {
            //while
            while *cp as libc::c_int != '\u{0}' as i32 {
                if *cp as libc::c_int == '*' as i32 {
                    loop {
                        cp = cp.offset(1);
                        if !(*cp as libc::c_int == '*' as i32) {
                            break;
                        }
                    }
                    if *cp as libc::c_int == '\u{0}' as i32 {
                        match_0 = true;
                        break;
                    }
                    if *cp as libc::c_int == '=' as i32 {
                        saw_sep = true;
                    }
                    while (saw_sep as libc::c_int != 0 || len != sep_pos)
                        && *var.offset(len as isize) as libc::c_int != '\u{0}' as i32
                        && *var.offset(len as isize) as libc::c_int != *cp as i32
                    {
                        len = len.wrapping_add(1);
                    }
                } //end of if *cp == '*'
                if *var.offset(len as isize) != *cp {
                    break;
                }
                cp = cp.offset(1 as isize);
                len = len.wrapping_add(1 as size_t);
            }
            if *cp as libc::c_int == '\0' as i32
                && (len == sep_pos || *var.offset(len as isize) as libc::c_int == '\0' as i32)
            {
                match_0 = true;
            }
        }
    } else {
        if strncmp(pattern, var, len) == 0
            && (len == sep_pos || *var.offset(len as isize) as libc::c_int == '\0' as i32)
        {
            match_0 = true;
        }
    }
    if match_0 {
        *full_match = len > sep_pos.wrapping_add(1);
    }
    debug_return_bool!(match_0);
}
