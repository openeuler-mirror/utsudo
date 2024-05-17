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
    unused_variables
)]

use crate::common::*;

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    static mut sudo_defs_table: [sudo_defs_types; 0];
    fn linux_audit_command(argv: *mut *mut libc::c_char, result: libc::c_int) -> libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn audit_success(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    debug_decl!(SUDOERS_DEBUG_AUDIT!());

    if def_log_allowed!() == 0 {
        debug_return_int!(0);
    }

    if !argv.is_null() {
        if linux_audit_command(argv, 1) == -(1 as libc::c_int) {
            rc = -(1 as libc::c_int);
        }
    }

    debug_return_int!(rc);
}
