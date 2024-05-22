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
pub type sudoers_gc_types = libc::c_uint;
pub const GC_PTR: sudoers_gc_types = 2;
pub const GC_VECTOR: sudoers_gc_types = 1;
pub const GC_UNKNOWN: sudoers_gc_types = 0;
#[no_mangle]
pub unsafe extern "C" fn sudoers_gc_add(
    mut type_0: sudoers_gc_types,
    mut v: *mut libc::c_void,
) -> bool {
    return true;
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_gc_remove(
    mut type_0: sudoers_gc_types,
    mut v: *mut libc::c_void,
) -> bool {
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn sudoers_gc_init() {}
