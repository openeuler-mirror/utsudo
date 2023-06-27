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
    unreachable_code
)]



/*
 * "Set variable_name value"
 */
#[no_mangle]
pub unsafe extern "C" fn parse_variable(
    mut entry: *const libc::c_char,
    mut conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
    debug_return_int!(0);
}
